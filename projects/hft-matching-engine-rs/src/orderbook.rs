use std::collections::{BTreeMap, VecDeque};

use crate::types::{OrderId, Side};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RestingOrder {
    pub order_id: OrderId,
    pub price_ticks: u64,
    pub quantity: u64,
}

#[derive(Default)]
pub struct OrderBook {
    pub bids: BTreeMap<u64, VecDeque<RestingOrder>>,
    pub asks: BTreeMap<u64, VecDeque<RestingOrder>>,
}

impl OrderBook {
    pub fn best_bid(&self) -> Option<u64> {
        self.bids.last_key_value().map(|(p, _)| *p)
    }

    pub fn best_ask(&self) -> Option<u64> {
        self.asks.first_key_value().map(|(p, _)| *p)
    }

    pub fn add_resting(&mut self, side: Side, order: RestingOrder) {
        let levels = match side {
            Side::Buy => &mut self.bids,
            Side::Sell => &mut self.asks,
        };
        levels.entry(order.price_ticks).or_default().push_back(order);
    }
}
