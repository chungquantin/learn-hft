//! Command models consumed by the matching runtime.
//!
//! # Design
//! - `OrderCommand` holds new-order intent.
//! - `EngineCommand` is the runtime envelope (`New`, `Cancel`, `Replace`).
//! - Every command variant carries an idempotency key for duplicate suppression.
//!
//! # How it works
//! - Producer side builds `EngineCommand`.
//! - Engine dedupes by `idempotency_key`.
//! - Engine routes by `primary_order_id` for partitioned execution.
//!
//! # Example
//! ```text
//! EngineCommand::Replace {
//!   idempotency_key: ...,
//!   cancel_order_id: OrderId(100),
//!   new_order: OrderCommand { ... }
//! }
//! ```

use crate::types::{OrderId, OrderType, Side, TimeInForce};

/// Stable deduplication key for at-least-once delivery environments.
///
/// Why `u128`:
/// - allows large keyspace for globally unique IDs (UUID-like encoding)
/// - still Copy and cheap for HashSet/HashMap keys
/// - avoids string IDs in hot paths
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct IdempotencyKey(pub u128);

/// Inbound order command consumed by the matching engine.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct OrderCommand {
    /// Unique key used to ignore duplicated command deliveries.
    pub idempotency_key: IdempotencyKey,
    /// Engine-side order identifier.
    ///
    /// Why separate from `idempotency_key`:
    /// - `order_id` identifies trading intent/state object
    /// - `idempotency_key` identifies transport command instance for dedupe
    pub order_id: OrderId,
    /// Buy or sell side.
    pub side: Side,
    /// Limit or market.
    pub order_type: OrderType,
    /// Order lifetime behavior.
    pub tif: TimeInForce,
    /// Price expressed in integer ticks.
    ///
    /// Why integer ticks:
    /// - floating point can introduce comparison/rounding ambiguity
    /// - exact equality/ordering is critical for matching correctness
    pub price_ticks: u64,
    /// Total requested quantity in lot units.
    ///
    /// Why integer quantity:
    /// - deterministic arithmetic in fills/remainders
    /// - avoids precision drift over many partial executions
    pub quantity: u64,
}

/// Unified command type processed by the engine runtime.
///
/// We keep all variants carrying an `idempotency_key` so duplicate delivery
/// can be safely ignored regardless of command class.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EngineCommand {
    /// New order submission command.
    New(OrderCommand),
    /// Cancel request for a resting order.
    Cancel {
        /// Unique dedupe key for cancel action.
        idempotency_key: IdempotencyKey,
        /// Target order to cancel.
        ///
        /// Cancel targets `OrderId` because cancel is a lifecycle action on an
        /// existing order object, not a new matching intent.
        order_id: OrderId,
    },
    /// Atomic cancel + new order submission request.
    ///
    /// This keeps queue semantics explicit: old order is removed first,
    /// then new order is processed as a fresh order with current time priority.
    Replace {
        /// Unique dedupe key for the replace action.
        idempotency_key: IdempotencyKey,
        /// Existing order to remove.
        ///
        /// This explicit field clarifies replacement semantics: old order loses
        /// its queue priority because it is canceled first.
        cancel_order_id: OrderId,
        /// New order payload to insert/match.
        ///
        /// Replace creates a fresh order-time priority, which mirrors common
        /// exchange semantics for amend/replace behavior.
        new_order: OrderCommand,
    },
}

impl EngineCommand {
    /// Returns command-level dedupe key.
    pub fn idempotency_key(&self) -> IdempotencyKey {
        match *self {
            EngineCommand::New(cmd) => cmd.idempotency_key,
            EngineCommand::Cancel {
                idempotency_key, ..
            } => idempotency_key,
            EngineCommand::Replace {
                idempotency_key, ..
            } => idempotency_key,
        }
    }

    /// Returns primary order id used for partition routing.
    pub fn primary_order_id(&self) -> OrderId {
        match *self {
            EngineCommand::New(cmd) => cmd.order_id,
            EngineCommand::Cancel { order_id, .. } => order_id,
            EngineCommand::Replace {
                cancel_order_id, ..
            } => cancel_order_id,
        }
    }
}
