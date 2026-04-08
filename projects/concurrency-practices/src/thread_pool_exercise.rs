//! Exercise: `thread_pool`
//!
//! Goal:
//! - Build a fixed-size worker pool with clean shutdown.
//!
//! Instructions:
//! 1. Implement a task queue accepted by N worker threads.
//! 2. Add a submission API returning task completion handles.
//! 3. Ensure tasks execute exactly once.
//! 4. Implement graceful shutdown and reject new tasks after shutdown starts.
//! 5. Add panic isolation so one bad task does not kill the entire pool.
//! 6. Add tests for throughput, correctness, and shutdown behavior.
//!
//! Method Hints:
//! - `std::sync::mpsc` for work queue
//! - Worker loop with `recv`/`recv_timeout`
//! - `Arc<Mutex<Receiver<_>>>` or MPMC queue design
//! - `Drop` implementation for shutdown/join discipline
