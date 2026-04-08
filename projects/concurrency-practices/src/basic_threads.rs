//! Basic thread primitives in Rust.
//!
//! This module teaches:
//! - `std::thread::spawn`
//! - `JoinHandle::join`
//! - why joining matters for deterministic program completion
//!
//! Core idea:
//! - OS threads are preemptively scheduled and expensive compared to async tasks.
//! - They are still the right tool for CPU-parallel work.

use std::{sync::Mutex, thread, time::Duration};

/// Spawns `n` worker threads, each returning a simple computed value.
///
/// Why this design:
/// - returning values through `join` is the simplest way to gather results
/// - avoids shared mutable state in your first thread examples
pub fn spawn_and_join_workers(n: usize) -> Vec<usize> {
    let out = Mutex::new(Vec::with_capacity(n));

    thread::scope(|s| {
        for i in 0..n {
            let out_ref = &out; // Create a reference to the Mutex
            s.spawn(move || {
                thread::sleep(Duration::from_millis(2));
                // Move the REFERENCE and 'i' into the thread
                out_ref.lock().unwrap().push(i * i);
            });
        }
    });

    // Since the threads are gone, we can safely take the Vec out of the Mutex.
    out.into_inner().expect("Mutex was poisoned")
}

#[cfg(test)]
mod tests {
    use super::spawn_and_join_workers;

    #[test]
    fn collects_all_worker_outputs() {
        let mut got = spawn_and_join_workers(4);
        got.sort_unstable();
        assert_eq!(got, vec![0, 1, 4, 9]);
    }
}
