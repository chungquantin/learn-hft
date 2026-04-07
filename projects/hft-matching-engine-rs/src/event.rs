//! Immutable execution event types emitted by the engine.
//!
//! # Design
//! - Events are append-only facts, not mutable state.
//! - `seq` provides partition-local deterministic ordering.
//! - Events are compact and copyable for fast downstream fan-out.
//!
//! # How it works
//! - Matching/cancel/replace flows emit `ExecutionEvent`.
//! - Replay and telemetry consume the same event shape.
//! - Financial/accounting services can map event facts to postings.
//!
//! # Example
//! ```text
//! ExecutionEvent { seq: 1201, kind: Trade, order_id: OrderId(9), price_ticks: 101, quantity: 2 }
//! ```

use crate::types::OrderId;

/// Emitted event type from matching and order-state transitions.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExecutionEventKind {
    /// Command was accepted for processing.
    Accepted,
    /// Command was rejected.
    Rejected,
    /// Trade occurred against resting liquidity.
    Trade,
    /// Residual quantity was placed as resting liquidity.
    Rested,
    /// Order was canceled.
    Canceled,
}

/// Immutable execution event for downstream consumers and replay.
///
/// Why immutable event structs:
/// - easy to append to logs/replay streams without accidental mutation
/// - good fit for event-sourced debugging and reconciliation
/// - stable facts are easier to reason about than mutable snapshots
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ExecutionEvent {
    /// Monotonic sequence number scoped to the partition/engine instance.
    ///
    /// Sequence allows deterministic ordering, replay verification, and
    /// easier incident reconstruction when multiple events share similar
    /// timestamps.
    pub seq: u64,
    /// Correlated order identifier.
    pub order_id: OrderId,
    /// Event classification.
    pub kind: ExecutionEventKind,
    /// Price of the event in ticks.
    ///
    /// Keeping price on every event avoids hidden joins later in telemetry
    /// pipelines and supports standalone event analysis.
    pub price_ticks: u64,
    /// Quantity attached to the event.
    pub quantity: u64,
}
