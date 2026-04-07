//! Matching logic that enforces price-time priority.
//!
//! Price priority is delegated to the heap-backed order book.
//! Time priority is preserved by FIFO queues per price level.
//!
//! # Design rationale
//! - Keep this module free from networking/persistence for predictable hot-path latency.
//! - Emit immutable events so downstream systems share one factual stream.
//! - Return next sequence cursor to avoid hidden global state.
//!
//! # Example (conceptual)
//! ```text
//! Incoming Buy 5 @ 101
//! Book asks: 101:[2,3], 102:[4]
//! Result: trades at 101 for 2 then 3, residual 0, no resting insert.
//! ```

use crate::command::OrderCommand;
use crate::event::{ExecutionEvent, ExecutionEventKind};
use crate::orderbook::{OrderBook, RestingOrder};
use crate::types::Side;

/// Matches one command against the order book and emits deterministic events.
///
/// Design intent:
/// - this function is pure matching+state transition logic
/// - transport, persistence, and network I/O are intentionally excluded
///   to keep hot-path latency predictable
pub fn match_command(
    seq_start: u64,
    book: &mut OrderBook,
    cmd: OrderCommand,
) -> (u64, Vec<ExecutionEvent>) {
    // Local sequence cursor. We advance this as events are emitted and return it
    // so caller can persist monotonic ordering across commands.
    let mut seq = seq_start;
    // Remaining quantity tracks partial fills in a single command lifecycle.
    let mut remaining = cmd.quantity;
    // Small pre-allocation avoids early reallocation for common event counts.
    let mut events = Vec::with_capacity(8);

    events.push(ExecutionEvent {
        seq,
        order_id: cmd.order_id,
        kind: ExecutionEventKind::Accepted,
        price_ticks: cmd.price_ticks,
        quantity: cmd.quantity,
    });
    seq += 1;

    loop {
        if remaining == 0 {
            break;
        }

        let crosses = match cmd.side {
            Side::Buy => {
                if let Some(best_ask) = book.best_ask() {
                    best_ask <= cmd.price_ticks
                } else {
                    false
                }
            }
            Side::Sell => {
                if let Some(best_bid) = book.best_bid() {
                    best_bid >= cmd.price_ticks
                } else {
                    false
                }
            }
        };

        if !crosses {
            break;
        }

        let level_pop = match cmd.side {
            // Buy order consumes asks.
            Side::Buy => book.pop_best_ask_order(),
            // Sell order consumes bids.
            Side::Sell => book.pop_best_bid_order(),
        };

        let Some((book_price, mut top)) = level_pop else {
            continue;
        };

        let traded = remaining.min(top.quantity);
        remaining -= traded;
        top.quantity -= traded;

        events.push(ExecutionEvent {
            seq,
            order_id: cmd.order_id,
            kind: ExecutionEventKind::Trade,
            price_ticks: book_price,
            quantity: traded,
        });
        seq += 1;

        if top.quantity > 0 {
            let resting_side = match cmd.side {
                Side::Buy => Side::Sell,
                Side::Sell => Side::Buy,
            };
            // Requeue with same time priority to preserve original queue precedence.
            book.requeue_front(resting_side, book_price, top);
        }
    }

    if remaining > 0 {
        // Residual liquidity becomes resting order on incoming side.
        // Time priority will be assigned by order book on insertion.
        book.add_resting(
            cmd.side,
            RestingOrder {
                order_id: cmd.order_id,
                price_ticks: cmd.price_ticks,
                quantity: remaining,
                // `0` means order book assigns monotonic time key at insert time.
                time_priority: 0,
            },
        );
        events.push(ExecutionEvent {
            seq,
            order_id: cmd.order_id,
            kind: ExecutionEventKind::Rested,
            price_ticks: cmd.price_ticks,
            quantity: remaining,
        });
        seq += 1;
    }

    (seq, events)
}
