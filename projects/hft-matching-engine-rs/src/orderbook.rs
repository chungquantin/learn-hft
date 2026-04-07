//! Heap-backed order book with strict price-time priority.
//!
//! Price priority:
//! - bids use a max-heap (highest price first)
//! - asks use a min-heap (lowest price first)
//!
//! Time priority:
//! - each price level stores resting orders in FIFO order via `VecDeque`.
//!
//! # Design rationale
//! - Heaps give fast best-price discovery.
//! - Per-level queues preserve intra-price order fairness.
//! - Order index enables targeted cancellation without full-book scan.
//! - Time-priority key makes ordering explicit and replay-friendly.
//!
//! # How matching uses this
//! - Incoming buy pops best ask level.
//! - Incoming sell pops best bid level.
//! - Partial fills are requeued with original `time_priority`.
//!
//! # Example (conceptual)
//! ```text
//! Asks:
//!   101 -> [order A (t=10), order B (t=11)]
//!   102 -> [order C (t=12)]
//! Buy limit @101 fills A first, then B, never C.
//! ```

use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, VecDeque};

use crate::types::{OrderId, Side};

/// Resting liquidity entry inside one price level FIFO queue.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RestingOrder {
    /// Order identifier.
    pub order_id: OrderId,
    /// Price in integer ticks.
    pub price_ticks: u64,
    /// Remaining quantity.
    ///
    /// Remaining (not original) quantity is stored so partial fills can
    /// update in-place without extra lookup into a separate structure.
    pub quantity: u64,
    /// Monotonic time-priority key.
    ///
    /// Lower value means older order and therefore higher fill priority
    /// when price is equal.
    pub time_priority: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct BidPrice(u64);

impl Ord for BidPrice {
    fn cmp(&self, other: &Self) -> Ordering {
        // For bids we want highest price first, and BinaryHeap is max-heap
        // by default, so natural ordering gives desired behavior.
        self.0.cmp(&other.0)
    }
}

impl PartialOrd for BidPrice {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct AskPrice(u64);

impl Ord for AskPrice {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse ordering so BinaryHeap pops lowest ask first.
        //
        // We wrap ask price in its own type instead of storing negative values
        // because explicit types reduce accidental side confusion.
        other.0.cmp(&self.0)
    }
}

impl PartialOrd for AskPrice {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Default)]
struct SideBook<P>
where
    P: Ord + Copy,
{
    // Heap answers "which price level has best priority right now?" quickly.
    heap: BinaryHeap<P>,
    // HashMap stores actual queues for each price level.
    //
    // Why not heap-only:
    // - heap stores one key per level, not all orders
    // - we still need FIFO queues for orders at equal price
    levels: HashMap<u64, VecDeque<RestingOrder>>,
}

impl<P> SideBook<P>
where
    P: Ord + Copy,
{
    fn remove_stale_best<F>(&mut self, price_of: F) -> Option<u64>
    where
        F: Fn(P) -> u64,
    {
        // We lazily discard stale heap keys because removing arbitrary heap items
        // on every level-empty event would be expensive.
        while let Some(top) = self.heap.peek().copied() {
            let price = price_of(top);
            match self.levels.get(&price) {
                Some(queue) if !queue.is_empty() => return Some(price),
                _ => {
                    let _ = self.heap.pop();
                }
            }
        }
        None
    }

    fn add_level_if_absent<F>(&mut self, price: u64, to_heap_price: F)
    where
        F: Fn(u64) -> P,
    {
        // We push price to heap exactly once per active level.
        // Multiple orders at same price reuse same queue.
        if !self.levels.contains_key(&price) {
            self.heap.push(to_heap_price(price));
            self.levels.insert(price, VecDeque::new());
        }
    }
}

/// Order book state for one instrument partition.
#[derive(Default)]
pub struct OrderBook {
    bids: SideBook<BidPrice>,
    asks: SideBook<AskPrice>,
    // Fast lookup from order id to (side, price) so cancel does not need
    // a full-book scan. Queue removal inside one level is still linear in
    // that level depth, but this keeps lookup bounded and practical.
    order_index: HashMap<OrderId, (Side, u64)>,
    // Monotonic counter used to stamp resting orders with deterministic time priority.
    next_time_priority: u64,
}

/// Default implementation for BidPrice.
impl Default for BidPrice {
    fn default() -> Self {
        Self(0)
    }
}

/// Default implementation for AskPrice.
impl Default for AskPrice {
    fn default() -> Self {
        Self(0)
    }
}

impl OrderBook {
    /// Returns next monotonic time-priority value.
    fn alloc_time_priority(&mut self) -> u64 {
        // Monotonic counter is deterministic and replay-friendly.
        // Wrapping add is acceptable at very high horizon; if this becomes a
        // concern in production, use wider counter or epoch-based reset policy.
        let v = self.next_time_priority;
        self.next_time_priority = self.next_time_priority.wrapping_add(1);
        v
    }

    /// Inserts order into one price-level queue by ascending `time_priority`.
    ///
    /// We still use `VecDeque` for contiguous queue behavior, but insertion is
    /// explicit by logical time key instead of assuming call-site order.
    fn insert_by_time(queue: &mut VecDeque<RestingOrder>, order: RestingOrder) {
        // Fast path for common case: newer order comes after existing tail.
        if queue
            .back()
            .is_none_or(|last| last.time_priority <= order.time_priority)
        {
            queue.push_back(order);
            return;
        }

        // General path: find first element with strictly greater time key and insert before it.
        // This maintains stable ascending order for time_priority.
        let mut insert_idx = queue.len();
        for i in 0..queue.len() {
            if queue
                .get(i)
                .is_some_and(|existing| existing.time_priority > order.time_priority)
            {
                insert_idx = i;
                break;
            }
        }
        queue.insert(insert_idx, order);
    }

    /// Returns best bid price, cleaning stale heap entries lazily.
    pub fn best_bid(&mut self) -> Option<u64> {
        self.bids.remove_stale_best(|p| p.0)
    }

    /// Returns best ask price, cleaning stale heap entries lazily.
    pub fn best_ask(&mut self) -> Option<u64> {
        self.asks.remove_stale_best(|p| p.0)
    }

    /// Adds resting liquidity to the correct side while preserving FIFO at that price.
    pub fn add_resting(&mut self, side: Side, mut order: RestingOrder) {
        // Assign monotonic time priority when caller does not provide one.
        // `0` means "assign automatically".
        if order.time_priority == 0 {
            order.time_priority = self.alloc_time_priority();
        }

        // Index the order first so external cancel calls can discover its level quickly.
        //
        // Why only (side, price) in index:
        // - smaller index payload
        // - keeps source of truth for queue order inside level queue itself
        self.order_index
            .insert(order.order_id, (side, order.price_ticks));
        match side {
            Side::Buy => {
                self.bids.add_level_if_absent(order.price_ticks, BidPrice);
                let queue = self.bids.levels.entry(order.price_ticks).or_default();
                // Time-priority insertion ensures deterministic behavior even if
                // commands arrive with externally assigned timestamps.
                Self::insert_by_time(queue, order);
            }
            Side::Sell => {
                self.asks.add_level_if_absent(order.price_ticks, AskPrice);
                let queue = self.asks.levels.entry(order.price_ticks).or_default();
                Self::insert_by_time(queue, order);
            }
        }
    }

    /// Pops the oldest resting order at the current best ask level.
    pub fn pop_best_ask_order(&mut self) -> Option<(u64, RestingOrder)> {
        let price = self.best_ask()?;
        let queue = self.asks.levels.get_mut(&price)?;
        let order = queue.pop_front()?;
        // Once removed from resting queue, this order is no longer cancelable as resting.
        // It is either matching now or already matched.
        let _ = self.order_index.remove(&order.order_id);
        if queue.is_empty() {
            let _ = self.asks.levels.remove(&price);
        }
        Some((price, order))
    }

    /// Pops the oldest resting order at the current best bid level.
    pub fn pop_best_bid_order(&mut self) -> Option<(u64, RestingOrder)> {
        let price = self.best_bid()?;
        let queue = self.bids.levels.get_mut(&price)?;
        let order = queue.pop_front()?;
        // Once removed from resting queue, this order is no longer cancelable as resting.
        let _ = self.order_index.remove(&order.order_id);
        if queue.is_empty() {
            let _ = self.bids.levels.remove(&price);
        }
        Some((price, order))
    }

    /// Re-inserts partially filled liquidity to the front to preserve time priority.
    pub fn requeue_front(&mut self, side: Side, price: u64, order: RestingOrder) {
        // Re-introduce the order into cancel index after partial fill.
        self.order_index.insert(order.order_id, (side, price));
        match side {
            Side::Buy => {
                self.bids.add_level_if_absent(price, BidPrice);
                let queue = self.bids.levels.entry(price).or_default();
                // For partial fills, `time_priority` stays unchanged, so this
                // order remains ahead of newer orders at same price.
                Self::insert_by_time(queue, order);
            }
            Side::Sell => {
                self.asks.add_level_if_absent(price, AskPrice);
                let queue = self.asks.levels.entry(price).or_default();
                Self::insert_by_time(queue, order);
            }
        }
    }

    /// Cancels one resting order by id.
    ///
    /// Returns removed order when found; `None` when order is already filled/absent.
    pub fn cancel_order(&mut self, order_id: OrderId) -> Option<RestingOrder> {
        // First, locate the side+price in O(1)-ish expected time via hash map.
        let (side, price) = self.order_index.remove(&order_id)?;

        // Then, remove from that single level FIFO queue.
        let levels = match side {
            Side::Buy => &mut self.bids.levels,
            Side::Sell => &mut self.asks.levels,
        };
        let queue = levels.get_mut(&price)?;

        // VecDeque::remove keeps remaining order sequence stable, preserving FIFO/time order.
        let mut removed: Option<RestingOrder> = None;
        for idx in 0..queue.len() {
            if queue.get(idx).is_some_and(|o| o.order_id == order_id) {
                removed = queue.remove(idx);
                break;
            }
        }

        // If queue became empty after cancellation, drop the level map entry.
        if queue.is_empty() {
            let _ = levels.remove(&price);
        }

        removed
    }
}

#[cfg(test)]
mod tests {
    use crate::types::OrderId;

    use super::{OrderBook, RestingOrder};

    #[test]
    fn best_prices_follow_heap_priority() {
        let mut book = OrderBook::default();
        book.add_resting(
            crate::types::Side::Buy,
            RestingOrder {
                order_id: OrderId(1),
                price_ticks: 100,
                quantity: 1,
                time_priority: 0,
            },
        );
        book.add_resting(
            crate::types::Side::Buy,
            RestingOrder {
                order_id: OrderId(2),
                price_ticks: 105,
                quantity: 1,
                time_priority: 0,
            },
        );
        book.add_resting(
            crate::types::Side::Sell,
            RestingOrder {
                order_id: OrderId(3),
                price_ticks: 110,
                quantity: 1,
                time_priority: 0,
            },
        );
        book.add_resting(
            crate::types::Side::Sell,
            RestingOrder {
                order_id: OrderId(4),
                price_ticks: 108,
                quantity: 1,
                time_priority: 0,
            },
        );

        assert_eq!(book.best_bid(), Some(105));
        assert_eq!(book.best_ask(), Some(108));
    }

    #[test]
    fn fifo_is_preserved_within_same_price_level() {
        let mut book = OrderBook::default();
        book.add_resting(
            crate::types::Side::Sell,
            RestingOrder {
                order_id: OrderId(10),
                price_ticks: 101,
                quantity: 5,
                time_priority: 0,
            },
        );
        book.add_resting(
            crate::types::Side::Sell,
            RestingOrder {
                order_id: OrderId(11),
                price_ticks: 101,
                quantity: 5,
                time_priority: 0,
            },
        );

        let (_, first) = book.pop_best_ask_order().expect("first order");
        let (_, second) = book.pop_best_ask_order().expect("second order");
        assert_eq!(first.order_id, OrderId(10));
        assert_eq!(second.order_id, OrderId(11));
    }

    #[test]
    fn cancel_uses_index_and_removes_target() {
        let mut book = OrderBook::default();
        book.add_resting(
            crate::types::Side::Buy,
            RestingOrder {
                order_id: OrderId(20),
                price_ticks: 99,
                quantity: 3,
                time_priority: 0,
            },
        );
        book.add_resting(
            crate::types::Side::Buy,
            RestingOrder {
                order_id: OrderId(21),
                price_ticks: 99,
                quantity: 4,
                time_priority: 0,
            },
        );

        let canceled = book.cancel_order(OrderId(20)).expect("order 20 canceled");
        assert_eq!(canceled.order_id, OrderId(20));
        let (_, first_after_cancel) = book.pop_best_bid_order().expect("remaining order");
        assert_eq!(first_after_cancel.order_id, OrderId(21));
    }

    #[test]
    fn lower_time_priority_fills_first_at_same_price() {
        let mut book = OrderBook::default();
        // Explicitly set out-of-order insertion by time key to verify queue sorting.
        book.add_resting(
            crate::types::Side::Sell,
            RestingOrder {
                order_id: OrderId(30),
                price_ticks: 100,
                quantity: 1,
                time_priority: 20,
            },
        );
        book.add_resting(
            crate::types::Side::Sell,
            RestingOrder {
                order_id: OrderId(31),
                price_ticks: 100,
                quantity: 1,
                time_priority: 10,
            },
        );
        let (_, first) = book.pop_best_ask_order().expect("first order");
        assert_eq!(first.order_id, OrderId(31));
    }
}
