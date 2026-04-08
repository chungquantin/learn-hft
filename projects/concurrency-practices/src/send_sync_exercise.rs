//! Exercise: `send_sync`
//!
//! Goal:
//! - Understand `Send` and `Sync` constraints in API design.
//!
//! Instructions:
//! 1. Create one type that is `Send + Sync` and one type that is not.
//! 2. Attempt to move both across thread boundaries.
//! 3. Observe compiler errors for non-thread-safe types.
//! 4. Refactor non-thread-safe ownership to become thread-safe (if appropriate).
//! 5. Write helper generic bounds requiring `T: Send` and `T: Sync`.
//! 6. Add compile-time assertions/tests for expected trait behavior.
//!
//! Method Hints:
//! - Generic bounds: `T: Send`, `T: Sync`, `T: Send + 'static`
//! - `std::rc::Rc` vs `std::sync::Arc`
//! - `std::cell::RefCell` vs `std::sync::Mutex`
//! - Compile-time helper patterns with trait bounds
