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

use std::thread;
use std::time::Duration;

/// Spawns `n` worker threads, each returning a simple computed value.
///
/// Why this design:
/// - returning values through `join` is the simplest way to gather results
/// - avoids shared mutable state in your first thread examples
pub fn spawn_and_join_workers(n: usize) -> Vec<usize> {
    let mut handles = Vec::with_capacity(n);
    for i in 0..n {
        handles.push(thread::spawn(move || {
            // Simulate tiny work so scheduling is visible in logs/demos.
            thread::sleep(Duration::from_millis(2));
            i * i
        }));
    }

    let mut out = Vec::with_capacity(n);
    for handle in handles {
        out.push(handle.join().expect("worker panicked"));
    }
    out
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
