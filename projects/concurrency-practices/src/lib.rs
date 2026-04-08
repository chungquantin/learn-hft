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
//!
//! Exercise-to-solution mapping:
//! - `*_exercise` modules are instruction-only stubs for you to implement.
//! - `*_solution` aliases below point to the completed reference implementations.

pub mod atomics;
pub mod atomics_deep_dive;
pub mod atomics_deep_dive_exercise;
pub mod atomics_exercise;
pub mod basic_threads;
pub mod basic_threads_exercise;
pub mod channels_patterns;
pub mod channels_patterns_exercise;
pub mod concurrent_dll;
pub mod concurrent_dll_exercise;
pub mod deadlock_patterns;
pub mod deadlock_patterns_exercise;
pub mod green_threads_async;
pub mod green_threads_async_exercise;
pub mod send_sync;
pub mod send_sync_exercise;
pub mod shared_state;
pub mod shared_state_exercise;
pub mod thread_lifecycle;
pub mod thread_lifecycle_exercise;
pub mod thread_pool;
pub mod thread_pool_exercise;

pub use atomics as atomics_solution;
pub use atomics_deep_dive as atomics_deep_dive_solution;
pub use basic_threads as basic_threads_solution;
pub use channels_patterns as channels_patterns_solution;
pub use concurrent_dll as concurrent_dll_solution;
pub use deadlock_patterns as deadlock_patterns_solution;
pub use green_threads_async as green_threads_async_solution;
pub use send_sync as send_sync_solution;
pub use shared_state as shared_state_solution;
pub use thread_lifecycle as thread_lifecycle_solution;
pub use thread_pool as thread_pool_solution;
