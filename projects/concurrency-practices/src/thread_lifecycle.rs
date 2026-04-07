//! Thread lifecycle pitfalls and safe patterns.
//!
//! Topics:
//! - why `join()` is important
//! - what "zombie-like" thread behavior means in practice
//! - how to stop worker threads cleanly
//!
//! In Rust there are no POSIX "zombie thread objects" in user code the same way
//! as process zombies, but you can still create zombie-like behavior:
//! - detached workers that outlive main flow
//! - forgotten join handles
//! - threads blocked forever with no shutdown signal

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::{Duration, Instant};

/// Spawns one worker and joins it properly.
pub fn clean_join_example() -> usize {
    let handle = thread::spawn(|| 42usize);
    handle.join().expect("worker panicked")
}

/// Demonstrates cooperative shutdown via shared atomic stop flag.
///
/// Returns number of loop iterations observed before shutdown.
pub fn cooperative_shutdown(timeout_ms: u64) -> usize {
    let stop = Arc::new(AtomicBool::new(false));
    let local_stop = Arc::clone(&stop);
    let handle = thread::spawn(move || {
        let mut iterations = 0usize;
        while !local_stop.load(Ordering::Acquire) {
            iterations += 1;
            thread::sleep(Duration::from_millis(1));
        }
        iterations
    });

    let started = Instant::now();
    while started.elapsed().as_millis() < timeout_ms as u128 {
        thread::sleep(Duration::from_millis(1));
    }

    stop.store(true, Ordering::Release);
    handle.join().expect("worker panicked")
}

/// Intentionally leaks a thread handle to illustrate bad lifecycle hygiene.
///
/// This is a demonstration function only.
#[allow(clippy::let_underscore_must_use)]
pub fn intentionally_forget_join_handle() {
    let _ = thread::spawn(|| {
        // Worker runs briefly; if not joined, caller has no completion guarantee.
        thread::sleep(Duration::from_millis(5));
    });
}

#[cfg(test)]
mod tests {
    use super::{clean_join_example, cooperative_shutdown};

    #[test]
    fn clean_join_returns_worker_result() {
        assert_eq!(clean_join_example(), 42);
    }

    #[test]
    fn cooperative_shutdown_finishes_thread() {
        let loops = cooperative_shutdown(5);
        assert!(loops > 0);
    }
}
