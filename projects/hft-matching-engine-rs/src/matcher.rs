use std::collections::VecDeque;

use crate::command::OrderCommand;
use crate::event::{ExecutionEvent, ExecutionEventKind};
use crate::orderbook::{OrderBook, RestingOrder};
use crate::types::Side;

fn pop_empty_level(
    levels: &mut std::collections::BTreeMap<u64, VecDeque<RestingOrder>>,
    price: u64,
) {
    if levels.get(&price).is_some_and(|q| q.is_empty()) {
        let _ = levels.remove(&price);
    }
}

pub fn match_command(
    seq_start: u64,
    book: &mut OrderBook,
    cmd: OrderCommand,
) -> (u64, Vec<ExecutionEvent>) {
    let mut seq = seq_start;
    let mut remaining = cmd.quantity;
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

        let (cross_price, crosses) = match cmd.side {
            Side::Buy => {
                if let Some(best_ask) = book.best_ask() {
                    (best_ask, best_ask <= cmd.price_ticks)
                } else {
                    (0, false)
                }
            }
            Side::Sell => {
                if let Some(best_bid) = book.best_bid() {
                    (best_bid, best_bid >= cmd.price_ticks)
                } else {
                    (0, false)
                }
            }
        };

        if !crosses {
            break;
        }

        let levels = match cmd.side {
            Side::Buy => &mut book.asks,
            Side::Sell => &mut book.bids,
        };

        let Some(level_queue) = levels.get_mut(&cross_price) else {
            continue;
        };

        let Some(mut top) = level_queue.pop_front() else {
            pop_empty_level(levels, cross_price);
            continue;
        };

        let traded = remaining.min(top.quantity);
        remaining -= traded;
        top.quantity -= traded;

        events.push(ExecutionEvent {
            seq,
            order_id: cmd.order_id,
            kind: ExecutionEventKind::Trade,
            price_ticks: cross_price,
            quantity: traded,
        });
        seq += 1;

        if top.quantity > 0 {
            level_queue.push_front(top);
        }
        pop_empty_level(levels, cross_price);
    }

    if remaining > 0 {
        book.add_resting(
            cmd.side,
            RestingOrder {
                order_id: cmd.order_id,
                price_ticks: cmd.price_ticks,
                quantity: remaining,
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
