//! Exercise: `concurrent_dll`
//!
//! Goal:
//! - Practice designing a thread-safe doubly linked list API.
//!
//! Instructions:
//! 1. Define node ownership and mutation model clearly.
//! 2. Implement push/pop at both ends with synchronization.
//! 3. Preserve structural invariants for `prev`/`next` pointers.
//! 4. Add iterator or snapshot traversal strategy under concurrency.
//! 5. Decide and justify lock granularity (global lock vs finer-grained).
//! 6. Add stress tests for concurrent inserts/removes and invariant checks.
//!
//! Method Hints:
//! - `Arc<Mutex<Node>>` or `Arc<RwLock<Node>>` patterns
//! - Interior mutability via `RefCell` only in single-threaded segments
//! - Snapshot traversal to avoid long-held locks
//! - Invariant checks after each mutation in tests
