use crate::types::OrderId;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExecutionEventKind {
    Accepted,
    Rejected,
    Trade,
    Rested,
    Canceled,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ExecutionEvent {
    pub seq: u64,
    pub order_id: OrderId,
    pub kind: ExecutionEventKind,
    pub price_ticks: u64,
    pub quantity: u64,
}
