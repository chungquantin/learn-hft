//! Shared-state concurrency primitives (`Arc`, `Mutex`, `RwLock`).
//!
//! Why these types exist:
//! - `Arc<T>`: atomically reference-counted shared ownership across threads
//! - `Mutex<T>`: mutual exclusion for one mutable owner at a time
//! - `RwLock<T>`: many readers or one writer
//!
//! Rules of thumb:
//! - use `Mutex` when writes are common or simplicity matters
//! - use `RwLock` when reads dominate and write contention is low

use std::sync::{Arc, Mutex, RwLock};
use std::thread;

/// Increments a shared counter from many threads with `Arc<Mutex<_>>`.
pub fn mutex_counter(thread_count: usize, increments_per_thread: usize) -> usize {
    let counter = Arc::new(Mutex::new(0usize));
    let mut handles = Vec::with_capacity(thread_count);

    for _ in 0..thread_count {
        let shared = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            for _ in 0..increments_per_thread {
                let mut guard = shared.lock().expect("mutex poisoned");
                *guard += 1;
            }
        }));
    }

    for handle in handles {
        handle.join().expect("worker panicked");
    }

    *counter.lock().expect("mutex poisoned")
}

/// Demonstrates read-heavy access with `Arc<RwLock<Vec<_>>>`.
pub fn rwlock_read_heavy_demo(readers: usize) -> usize {
    let data = Arc::new(RwLock::new(vec![1usize, 2, 3, 4, 5]));
    let mut handles = Vec::with_capacity(readers);

    // Single write phase.
    {
        let mut writer = data.write().expect("rwlock poisoned");
        writer.push(6);
    }

    // Many readers can proceed concurrently.
    for _ in 0..readers {
        let shared = Arc::clone(&data);
        handles.push(thread::spawn(move || {
            let guard = shared.read().expect("rwlock poisoned");
            guard.iter().sum::<usize>()
        }));
    }

    let mut totals = 0usize;
    for handle in handles {
        totals += handle.join().expect("reader panicked");
    }
    totals
}

#[cfg(test)]
mod tests {
    use super::{mutex_counter, rwlock_read_heavy_demo};

    #[test]
    fn mutex_counter_matches_expected_total() {
        assert_eq!(mutex_counter(8, 1000), 8000);
    }

    #[test]
    fn rwlock_readers_can_observe_written_state() {
        // Each reader sees sum(1..=6)=21.
        assert_eq!(rwlock_read_heavy_demo(4), 84);
    }
}
