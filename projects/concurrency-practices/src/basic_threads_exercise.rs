//! Exercise: `basic_threads`
//!
//! Goal:
//! - Practice spawning and joining threads safely.
//!
//! Instructions:
//! 1. Create a function that spawns N threads.
//! 2. Give each thread a unique ID and print start/finish logs.
//! 3. Sleep each thread for a different duration to observe interleaving.
//! 4. Join all handles and propagate panic information if any thread fails.
//! 5. Return a deterministic summary (e.g., number of successful joins).
//! 6. Add a test that verifies all threads joined and no handles were leaked.
//!
//! Method Hints:
//! - `std::thread::spawn`
//! - `std::thread::JoinHandle::join`
//! - `std::time::Duration` + `std::thread::sleep`
//! - `Vec<JoinHandle<_>>` for handle management

use std::{
    sync::{
        Arc, Mutex,
        atomic::{AtomicUsize, Ordering},
    },
    thread,
    time::Duration,
};

// 1. Create a function that spawns N threads.
fn spawn_join_threads(num_of_threads: usize) -> anyhow::Result<usize> {
    let counter = Arc::new(AtomicUsize::new(0));
    let mut handles = Vec::with_capacity(num_of_threads);
    for _ in 0..num_of_threads {
        let local = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            local.fetch_add(1, Ordering::Relaxed);
        }));
    }

    for handle in handles {
        handle.join().expect("join thread increase counter");
    }

    Ok(counter.load(Ordering::Relaxed))
}

// 2. Give each thread a unique ID and print start/finish logs.
fn spawn_scoped_threads_and_its_id(num_of_threads: usize) -> anyhow::Result<usize> {
    let counter = Mutex::new(0);

    thread::scope(|s| {
        for _ in 0..num_of_threads {
            // Shadow the reference so we can 'move' it into the closure
            let counter_ref = &counter;

            s.spawn(move || {
                println!("[Thread {:?}] Starting...", thread::current().id());

                // Simulate work
                thread::sleep(Duration::from_millis(50));

                let mut lock = counter_ref.lock().unwrap();
                *lock += 1;

                println!(
                    "[Thread {:?}] Finished. Counter is now: {}",
                    thread::current().id(),
                    *lock
                );
            });
        }
        // Scope automatically joins all threads here
    });

    // Extract the final value from the Mutex
    let final_val = counter
        .into_inner()
        .map_err(|_| anyhow::anyhow!("Mutex poisoned"))?;
    Ok(final_val)
}

// 3. Sleep each thread for a different duration to observe interleaving.

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exercise_1() {
        let counter = self::spawn_join_threads(5).unwrap();
        assert_eq!(counter, 5);
    }

    #[test]
    fn test_exercise_2() {
        let counter = spawn_scoped_threads_and_its_id(5).unwrap();
        assert_eq!(counter, 5);
    }
}
