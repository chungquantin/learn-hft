use crate::types::{OrderId, OrderType, Side, TimeInForce};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct IdempotencyKey(pub u128);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct OrderCommand {
    pub idempotency_key: IdempotencyKey,
    pub order_id: OrderId,
    pub side: Side,
    pub order_type: OrderType,
    pub tif: TimeInForce,
    pub price_ticks: u64,
    pub quantity: u64,
}
