//! Deep-dive atomics curriculum: basics -> advanced.
//!
//! This module is intentionally verbose and heavily commented for learning.
//! It covers:
//! 1. `Relaxed` counters (atomicity without synchronization)
//! 2. Release/Acquire message passing
//! 3. Compare-and-swap (CAS) loops
//! 4. A tiny spin lock built with atomics
//! 5. One-time initialization state machine with atomics
//!
//! If this still feels hard, start with:
//! - `docs/atomics_beginner_guide.md`
//! - `cargo run -p concurrency-practices --bin atomics_lab`
//!
//! ---------------------------------------------------------------------------
//! Memory ordering mental model (practical):
//!
//! - `Relaxed`: atomic read/write only, no ordering of other memory.
//! - `Release`: publish prior writes before this store.
//! - `Acquire`: after this load, see writes published by matching release.
//! - `AcqRel`: both acquire and release in one operation.
//! - `SeqCst`: strongest global ordering; easiest to reason about, often slower.
//!
//! If unsure, start with `SeqCst`, prove correctness, then relax carefully.
//! ---------------------------------------------------------------------------

use std::cell::UnsafeCell;
use std::hint::spin_loop;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicU8, AtomicUsize, Ordering};
use std::thread;
use std::time::Duration;

/// Stage 1: `Relaxed` counter demo.
///
/// Why this is valid:
/// - We only care about final numeric count.
/// - We do NOT use this counter to synchronize access to other shared memory.
pub fn relaxed_counter(threads: usize, increments: usize) -> usize {
    let counter = Arc::new(AtomicUsize::new(0));
    let mut handles = Vec::with_capacity(threads);

    for _ in 0..threads {
        let c = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            for _ in 0..increments {
                c.fetch_add(1, Ordering::Relaxed);
            }
        }));
    }

    for h in handles {
        h.join().expect("relaxed counter worker panicked");
    }
    counter.load(Ordering::Relaxed)
}

/// Stage 2: Release/Acquire publication demo.
///
/// Pattern:
/// - writer stores `data` (Relaxed), then sets `ready=true` with Release.
/// - reader spins until `ready` with Acquire, then reads `data`.
///
/// Why it works:
/// - Acquire load of `ready` synchronizes with Release store.
/// - all writes before release become visible after acquire.
pub fn release_acquire_publication(value: usize) -> usize {
    let data = Arc::new(AtomicUsize::new(0));
    let ready = Arc::new(AtomicBool::new(false));

    let w_data = Arc::clone(&data);
    let w_ready = Arc::clone(&ready);
    let writer = thread::spawn(move || {
        w_data.store(value, Ordering::Relaxed);
        w_ready.store(true, Ordering::Release);
    });

    let r_data = Arc::clone(&data);
    let r_ready = Arc::clone(&ready);
    let reader = thread::spawn(move || {
        while !r_ready.load(Ordering::Acquire) {
            spin_loop();
        }
        r_data.load(Ordering::Relaxed)
    });

    writer.join().expect("writer panicked");
    reader.join().expect("reader panicked")
}

/// Stage 3: CAS-loop increment.
///
/// Why CAS loops are common:
/// - implement lock-free updates when `fetch_*` primitives are not enough
/// - update derived state based on current value atomically
pub fn cas_increment(counter: &AtomicUsize) {
    loop {
        let current = counter.load(Ordering::Acquire);
        let next = current.wrapping_add(1);
        match counter.compare_exchange_weak(current, next, Ordering::AcqRel, Ordering::Acquire) {
            Ok(_) => break,
            Err(_) => {
                // Another thread won the race; retry with latest value.
                spin_loop();
            }
        }
    }
}

/// Stage 4: tiny spin lock for educational purposes.
///
/// NOTE:
/// - This is not fair and can waste CPU under contention.
/// - Prefer `Mutex` in production unless you have a strong reason.
pub struct SpinLock<T> {
    flag: AtomicBool,
    value: UnsafeCell<T>,
}

impl<T> SpinLock<T> {
    /// Creates a new spin lock.
    pub fn new(value: T) -> Self {
        Self {
            flag: AtomicBool::new(false),
            value: UnsafeCell::new(value),
        }
    }

    /// Acquires lock and returns guard.
    pub fn lock(&self) -> SpinLockGuard<'_, T> {
        // Lock acquisition:
        // false -> unlocked, true -> locked.
        while self
            .flag
            .compare_exchange_weak(false, true, Ordering::Acquire, Ordering::Relaxed)
            .is_err()
        {
            spin_loop();
        }

        SpinLockGuard { lock: self }
    }
}

/// Guard that unlocks on drop.
pub struct SpinLockGuard<'a, T> {
    lock: &'a SpinLock<T>,
}

impl<T> std::ops::Deref for SpinLockGuard<'_, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        // SAFETY:
        // Guard implies this thread holds lock exclusively for mutation.
        // Shared deref is always safe while lock is held.
        unsafe { &*self.lock.value.get() }
    }
}

impl<T> std::ops::DerefMut for SpinLockGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // SAFETY:
        // Spin lock guarantees exclusive mutable access while guard lives.
        unsafe { &mut *self.lock.value.get() }
    }
}

impl<T> Drop for SpinLockGuard<'_, T> {
    fn drop(&mut self) {
        // Release unlock publishes all writes in critical section.
        self.lock.flag.store(false, Ordering::Release);
    }
}

// SAFETY: lock enforces synchronized access; T must be Send to move across threads.
unsafe impl<T: Send> Send for SpinLock<T> {}
// SAFETY: shared references are safe because mutation is guarded by atomic lock.
unsafe impl<T: Send> Sync for SpinLock<T> {}

/// Stage 5: one-time initialization state machine.
///
/// States:
/// - 0 = uninitialized
/// - 1 = initializing (one winner thread)
/// - 2 = initialized
pub struct OnceValue {
    state: AtomicU8,
    value: AtomicUsize,
}

impl OnceValue {
    /// Create empty once value.
    pub fn new() -> Self {
        Self {
            state: AtomicU8::new(0),
            value: AtomicUsize::new(0),
        }
    }

    /// Returns initialized value; first caller computes it.
    pub fn get_or_init<F>(&self, init: F) -> usize
    where
        F: FnOnce() -> usize,
    {
        loop {
            match self.state.load(Ordering::Acquire) {
                2 => {
                    // Already initialized.
                    return self.value.load(Ordering::Acquire);
                }
                0 => {
                    // Try to become initializer.
                    if self
                        .state
                        .compare_exchange(0, 1, Ordering::AcqRel, Ordering::Acquire)
                        .is_ok()
                    {
                        let v = init();
                        self.value.store(v, Ordering::Relaxed);
                        // Publish completion.
                        self.state.store(2, Ordering::Release);
                        return v;
                    }
                }
                1 => {
                    // Another thread is initializing; wait briefly.
                    spin_loop();
                    thread::sleep(Duration::from_micros(10));
                }
                _ => unreachable!("invalid once state"),
            }
        }
    }
}

impl Default for OnceValue {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::{OnceValue, SpinLock, cas_increment, relaxed_counter, release_acquire_publication};
    use std::sync::Arc;
    use std::sync::atomic::AtomicUsize;
    use std::thread;

    #[test]
    fn relaxed_counter_matches_expected() {
        assert_eq!(relaxed_counter(4, 1_000), 4_000);
    }

    #[test]
    fn release_acquire_publication_sees_written_value() {
        assert_eq!(release_acquire_publication(42), 42);
    }

    #[test]
    fn cas_loop_increments_correctly_under_contention() {
        let counter = Arc::new(AtomicUsize::new(0));
        let mut handles = Vec::new();
        for _ in 0..8 {
            let c = Arc::clone(&counter);
            handles.push(thread::spawn(move || {
                for _ in 0..500 {
                    cas_increment(&c);
                }
            }));
        }
        for h in handles {
            h.join().expect("cas worker panicked");
        }
        assert_eq!(counter.load(std::sync::atomic::Ordering::Relaxed), 4_000);
    }

    #[test]
    fn spin_lock_guards_shared_mutation() {
        let lock = Arc::new(SpinLock::new(0usize));
        let mut handles = Vec::new();
        for _ in 0..4 {
            let l = Arc::clone(&lock);
            handles.push(thread::spawn(move || {
                for _ in 0..1_000 {
                    let mut guard = l.lock();
                    *guard += 1;
                }
            }));
        }
        for h in handles {
            h.join().expect("spin lock worker panicked");
        }
        assert_eq!(*lock.lock(), 4_000);
    }

    #[test]
    fn once_value_initializes_exactly_once() {
        let once = Arc::new(OnceValue::new());
        let init_calls = Arc::new(AtomicUsize::new(0));
        let mut handles = Vec::new();
        for _ in 0..16 {
            let o = Arc::clone(&once);
            let c = Arc::clone(&init_calls);
            handles.push(thread::spawn(move || {
                o.get_or_init(|| {
                    c.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                    777
                })
            }));
        }

        let mut vals = Vec::new();
        for h in handles {
            vals.push(h.join().expect("once thread panicked"));
        }
        assert!(vals.iter().all(|&v| v == 777));
        assert_eq!(init_calls.load(std::sync::atomic::Ordering::Relaxed), 1);
    }
}
