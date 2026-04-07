//! Core domain types used by the matching engine.
//!
//! # Design
//! - Types are small, copyable, and integer-based for deterministic behavior.
//! - Enums model finite state/behavior sets with compile-time exhaustiveness.
//! - No floating-point values are used in core matching decisions.
//!
//! # How this file is used
//! - `OrderId` identifies orders across command, book, and event flows.
//! - `Side`, `OrderType`, and `TimeInForce` drive matching and lifecycle logic.
//!
//! # Example
//! ```text
//! OrderId(42), Side::Buy, OrderType::Limit, TimeInForce::Gtc
//! ```

/// Stable order identifier used across engine events.
///
/// Why `u64`:
/// - compact (8 bytes), cache-friendly, and cheap to copy
/// - enough headroom for very large event volumes
/// - deterministic and easy to serialize
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct OrderId(pub u64);

/// Side of a market/order-book action.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Side {
    /// Bid side (buy liquidity or taking ask liquidity).
    Buy,
    /// Ask side (sell liquidity or taking bid liquidity).
    Sell,
}

/// Supported order type.
///
/// We keep this enum intentionally small at first. In production engines,
/// more variants (stop, peg, iceberg, etc.) are often added later after
/// core matching invariants are stable and well-tested.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OrderType {
    /// Order with an explicit price.
    Limit,
    /// Order intended to cross immediately against opposite liquidity.
    Market,
}

/// Time-in-force behavior for resting/matching semantics.
///
/// Why enum:
/// - compile-time exhaustiveness in `match` statements
/// - explicit behavior selection without string parsing overhead
/// - safer evolution than loosely typed config flags
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TimeInForce {
    /// Good-till-cancel.
    Gtc,
    /// Immediate-or-cancel.
    Ioc,
    /// Fill-or-kill.
    Fok,
}
