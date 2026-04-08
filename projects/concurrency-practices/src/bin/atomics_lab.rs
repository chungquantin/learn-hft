//! Focused runner for atomics deep-dive examples.
//!
//! Run:
//! - `cargo run -p concurrency-practices --bin atomics_lab`

use concurrency_practices::atomics::atomic_counter;
use concurrency_practices::atomics_deep_dive::{
    OnceValue, SpinLock, cas_increment, relaxed_counter, release_acquire_publication,
};
use std::sync::atomic::AtomicUsize;
use std::sync::Arc;
use std::thread;

fn main() {
    println!("== atomics basics ==");
    println!("atomic_counter(4,1000): {}", atomic_counter(4, 1_000));
    println!("relaxed_counter(4,1000): {}", relaxed_counter(4, 1_000));
    println!(
        "release_acquire_publication(42): {}",
        release_acquire_publication(42)
    );

    println!("\n== CAS loop increment ==");
    let c = Arc::new(AtomicUsize::new(0));
    let mut handles = Vec::new();
    for _ in 0..4 {
        let local = Arc::clone(&c);
        handles.push(thread::spawn(move || {
            for _ in 0..1_000 {
                cas_increment(&local);
            }
        }));
    }
    for h in handles {
        h.join().expect("cas worker panicked");
    }
    println!("cas counter after 4000 increments: {}", c.load(std::sync::atomic::Ordering::Relaxed));

    println!("\n== spin lock demo ==");
    let lock = Arc::new(SpinLock::new(0usize));
    let mut spin_workers = Vec::new();
    for _ in 0..4 {
        let l = Arc::clone(&lock);
        spin_workers.push(thread::spawn(move || {
            for _ in 0..500 {
                let mut guard = l.lock();
                *guard += 1;
            }
        }));
    }
    for h in spin_workers {
        h.join().expect("spin worker panicked");
    }
    println!("spin lock protected value: {}", *lock.lock());

    println!("\n== once init demo ==");
    let once = Arc::new(OnceValue::new());
    let mut init_workers = Vec::new();
    for _ in 0..8 {
        let o = Arc::clone(&once);
        init_workers.push(thread::spawn(move || o.get_or_init(|| 777)));
    }
    let results: Vec<_> = init_workers
        .into_iter()
        .map(|h| h.join().expect("once worker panicked"))
        .collect();
    println!("once values: {:?}", results);
}
