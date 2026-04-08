//! Exercise: `deadlock_patterns`
//!
//! Goal:
//! - Learn to identify and prevent deadlocks.
//!
//! Instructions:
//! 1. Define two resources protected by separate locks.
//! 2. Create a deadlock-prone access pattern (documented, do not run indefinitely).
//! 3. Refactor to a strict lock ordering policy.
//! 4. Add a non-blocking fallback using `try_lock` where appropriate.
//! 5. Add instrumentation to detect prolonged lock wait times.
//! 6. Write tests proving deadlock-free behavior under contention.
//!
//! Method Hints:
//! - Global lock ordering by resource ID
//! - `Mutex::try_lock` fallback path
//! - Time-bounded retries with `std::time::Instant`
//! - Keep critical sections minimal
