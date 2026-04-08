//! Exercise: `thread_lifecycle`
//!
//! Goal:
//! - Practice robust thread startup/shutdown discipline.
//!
//! Instructions:
//! 1. Create a worker thread that loops until a shutdown signal is received.
//! 2. Design a clean shutdown path that guarantees join.
//! 3. Demonstrate a bad lifecycle pattern (detached thread) in comments only.
//! 4. Add timeout handling for slow shutdown.
//! 5. Track thread state transitions (starting, running, stopping, stopped).
//! 6. Add tests that ensure no thread leaks on normal and error paths.
//!
//! Method Hints:
//! - `std::thread::spawn` + `JoinHandle::join`
//! - `AtomicBool` / channel-based shutdown signals
//! - `std::sync::mpsc` for control messages
//! - Optional state enum for lifecycle transitions
