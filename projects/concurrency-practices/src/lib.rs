//! `concurrency-practices` is a progressive Rust concurrency playground.
//!
//! The crate is organized from basic to advanced topics so you can learn in order:
//! 1. `basic_threads` - spawning and joining OS threads
//! 2. `shared_state` - `Arc`, `Mutex`, and `RwLock` patterns
//! 3. `send_sync` - understanding thread-safety marker traits
//! 4. `atomics` - lock-free counters and synchronization basics
//! 5. `atomics_deep_dive` - memory ordering and lock-free patterns in depth
//! 6. `thread_lifecycle` - join discipline, leaked/zombie-like thread pitfalls
//! 7. `green_threads_async` - async tasks (green-thread style scheduling)
//! 8. `channels_patterns` - fan-in, backpressure, and MPMC pipelines
//! 9. `deadlock_patterns` - lock ordering and deadlock avoidance
//! 10. `thread_pool` - fixed worker pool implementation from scratch
//! 11. `concurrent_dll` - concurrent doubly linked list to handle recursive and thread-safe types
//!
//! A runnable aggregator is available at:
//! - `cargo run -p concurrency-practices --bin run_all`

pub mod atomics;
pub mod atomics_deep_dive;
pub mod basic_threads;
pub mod channels_patterns;
pub mod concurrent_dll;
pub mod deadlock_patterns;
pub mod green_threads_async;
pub mod send_sync;
pub mod shared_state;
pub mod thread_lifecycle;
pub mod thread_pool;
