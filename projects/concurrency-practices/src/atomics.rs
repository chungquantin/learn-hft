//! Atomic types and lock-free coordination basics.
//!
//! This module demonstrates:
//! - `AtomicUsize` counters
//! - compare-and-swap style updates (`compare_exchange`)
//! - why memory ordering matters
//!
//! Note:
//! - Atomics are powerful but easy to misuse.
//! - Prefer `Mutex` unless profiling or design constraints justify lock-free logic.

use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;

/// Increments an atomic counter from many threads.
///
/// Uses `Ordering::Relaxed` because:
/// - we only need atomicity of the counter value itself
/// - we do not use this counter to synchronize other memory
pub fn atomic_counter(thread_count: usize, increments_per_thread: usize) -> usize {
    let counter = Arc::new(AtomicUsize::new(0));
    let mut handles = Vec::with_capacity(thread_count);

    for _ in 0..thread_count {
        let local = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            for _ in 0..increments_per_thread {
                local.fetch_add(1, Ordering::Relaxed);
            }
        }));
    }

    for handle in handles {
        handle.join().expect("thread panicked");
    }
    counter.load(Ordering::Relaxed)
}

/// Sets an atomic "once" flag, returning `true` only for the first caller.
pub fn claim_once(flag: &AtomicUsize) -> bool {
    flag.compare_exchange(
        0, // expected old value
        1, // new value
        Ordering::AcqRel,
        Ordering::Acquire,
    )
    .is_ok()
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use std::sync::atomic::AtomicUsize;
    use std::thread;

    use super::{atomic_counter, claim_once};

    #[test]
    fn atomic_counter_matches_expected() {
        assert_eq!(atomic_counter(4, 1000), 4000);
    }

    #[test]
    fn only_one_thread_can_claim_once() {
        let flag = Arc::new(AtomicUsize::new(0));
        let mut handles = Vec::new();
        for _ in 0..16 {
            let local = Arc::clone(&flag);
            handles.push(thread::spawn(move || claim_once(&local)));
        }
        let winners = handles
            .into_iter()
            .map(|h| h.join().expect("thread panicked"))
            .filter(|&v| v)
            .count();
        assert_eq!(winners, 1);
    }
}
