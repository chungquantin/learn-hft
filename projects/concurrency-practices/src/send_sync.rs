//! `Send` and `Sync` understanding by example.
//!
//! Definitions:
//! - `Send`: a value can be moved to another thread safely
//! - `Sync`: `&T` can be shared across threads safely
//!
//! Practical interpretation:
//! - Most plain data (`usize`, `Vec<T>`, etc.) is `Send + Sync` when `T` is.
//! - `Rc<T>` is not `Send`/`Sync` (single-thread refcount).
//! - `Arc<T>` is thread-safe and usually preferred for shared ownership.

use std::sync::Arc;
use std::thread;

/// Marker function: compiles only if `T: Send`.
pub fn assert_send<T: Send>() {}

/// Marker function: compiles only if `T: Sync`.
pub fn assert_sync<T: Sync>() {}

/// Demonstrates moving a `Send` type across thread boundaries.
pub fn move_vec_across_thread() -> usize {
    let payload = vec![10usize, 20, 30];
    let handle = thread::spawn(move || payload.into_iter().sum::<usize>());
    handle.join().expect("thread panicked")
}

/// Demonstrates sharing a `Sync` type via `Arc`.
pub fn share_arc_across_threads(workers: usize) -> usize {
    let shared = Arc::new(vec![1usize, 2, 3, 4]);
    let mut handles = Vec::with_capacity(workers);
    for _ in 0..workers {
        let local = Arc::clone(&shared);
        handles.push(thread::spawn(move || local.iter().sum::<usize>()));
    }
    handles
        .into_iter()
        .map(|h| h.join().expect("thread panicked"))
        .sum::<usize>()
}

#[cfg(test)]
mod tests {
    use super::{assert_send, assert_sync, move_vec_across_thread, share_arc_across_threads};

    #[test]
    fn send_sync_constraints_compile_for_expected_types() {
        assert_send::<Vec<u8>>();
        assert_sync::<Vec<u8>>();
    }

    #[test]
    fn can_move_send_values_between_threads() {
        assert_eq!(move_vec_across_thread(), 60);
    }

    #[test]
    fn can_share_sync_values_with_arc() {
        // Each worker sees sum=10
        assert_eq!(share_arc_across_threads(3), 30);
    }
}
