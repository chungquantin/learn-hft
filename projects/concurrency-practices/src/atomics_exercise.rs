//! Exercise: `atomics`
//!
//! Goal:
//! - Practice lock-free counters and atomic coordination basics.
//!
//! Instructions:
//! 1. Implement a shared atomic counter incremented by many threads.
//! 2. Choose and justify memory ordering for increment/load operations.
//! 3. Add a stop flag with `AtomicBool` to terminate worker loops.
//! 4. Measure total operations executed before stop.
//! 5. Add tests that validate monotonic growth and correct final counts.
//! 6. Add notes explaining when atomics are better/worse than a mutex.
//!
//! Method Hints:
//! - `std::sync::atomic::{AtomicU64, AtomicBool, Ordering}`
//! - `fetch_add`, `load`, `store`
//! - `Ordering::Relaxed`, `Acquire`, `Release`, `SeqCst`
//! - `std::sync::Arc` for sharing atomics
