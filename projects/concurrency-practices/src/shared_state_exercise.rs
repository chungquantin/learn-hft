//! Exercise: `shared_state`
//!
//! Goal:
//! - Practice `Arc`, `Mutex`, and `RwLock` patterns.
//!
//! Instructions:
//! 1. Build a shared counter using `Arc<Mutex<_>>`.
//! 2. Spawn multiple worker threads that increment it.
//! 3. Ensure final value equals `workers * increments_per_worker`.
//! 4. Add a read-heavy structure using `Arc<RwLock<_>>`.
//! 5. Create a workload with many readers and occasional writers.
//! 6. Compare behavior/tradeoffs between `Mutex` and `RwLock`.
//! 7. Add tests for correctness and lock poisoning recovery strategy.
//!
//! Method Hints:
//! - `std::sync::Arc`
//! - `std::sync::Mutex`, `std::sync::RwLock`
//! - `lock()`, `read()`, `write()`
//! - `PoisonError` handling via `into_inner()`
