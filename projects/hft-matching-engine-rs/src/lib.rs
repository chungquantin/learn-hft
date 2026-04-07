//! High-performance HFT matching engine primitives in Rust.
//!
//! The crate is intentionally small and focused on interview-quality, production-style
//! building blocks:
//! - price-time-priority matching
//! - heap-backed order book levels
//! - idempotent command handling
//! - bounded thread-safe ring buffer for low-latency ingress
//!
//! # Architecture Summary
//! - `command`: inbound command models (`New` / `Cancel` / `Replace`)
//! - `orderbook`: heap + per-price-level queue data structure
//! - `matcher`: pure matching transition logic
//! - `engine`: sequencing, dedupe, and command orchestration
//! - `partition`: horizontal scaling by deterministic routing
//! - `ring_buffer`: bounded SPSC ingress primitive
//! - `replay`: deterministic rebuild from append-only command log
//!
//! # Typical flow
//! 1. Produce `EngineCommand`
//! 2. Route/enqueue to partition ingress
//! 3. Drain ingress in partition
//! 4. Match/cancel/replace in engine
//! 5. Emit deterministic `ExecutionEvent` stream

pub mod command;
pub mod engine;
pub mod event;
pub mod matcher;
pub mod orderbook;
pub mod partition;
pub mod replay;
pub mod ring_buffer;
pub mod simulation;
pub mod types;

pub use command::{EngineCommand, IdempotencyKey, OrderCommand};
pub use engine::{ConcurrentMatchingEngine, MatchingEngine};
pub use event::{ExecutionEvent, ExecutionEventKind};
pub use partition::PartitionRuntime;
pub use replay::InMemoryReplayLog;
pub use ring_buffer::SpscRingBuffer;
pub use simulation::{run_partitioned_simulation, SimulationConfig, SimulationReport};
pub use types::{OrderId, OrderType, Side, TimeInForce};
