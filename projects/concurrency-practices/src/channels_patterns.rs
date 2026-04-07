//! Channel patterns for thread communication.
//!
//! This module goes beyond mutexes by showing message-passing designs:
//! - fan-in with `std::sync::mpsc`
//! - bounded backpressure with `sync_channel`
//! - MPMC workers with `crossbeam-channel`
//!
//! Why channels:
//! - reduce shared-state locking complexity
//! - make ownership transfer explicit
//! - model pipelines and actor-like systems naturally

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

/// Fan-in pattern: multiple producers send into one receiver.
pub fn fan_in_sum(producers: usize, values_per_producer: usize) -> usize {
    let (tx, rx) = mpsc::channel::<usize>();
    let mut handles = Vec::with_capacity(producers);

    for p in 0..producers {
        let tx_local = tx.clone();
        handles.push(thread::spawn(move || {
            for i in 0..values_per_producer {
                tx_local
                    .send(p * values_per_producer + i)
                    .expect("send failed");
            }
        }));
    }
    drop(tx);

    for handle in handles {
        handle.join().expect("producer panicked");
    }

    rx.into_iter().sum::<usize>()
}

/// Bounded channel demo: producer blocks when queue is full.
///
/// Returns number of values consumed.
pub fn bounded_backpressure_demo(capacity: usize, values: usize) -> usize {
    let (tx, rx) = mpsc::sync_channel::<usize>(capacity);

    let producer = thread::spawn(move || {
        for i in 0..values {
            // Blocks when queue is at capacity until consumer receives.
            tx.send(i).expect("send failed");
        }
    });

    let consumer = thread::spawn(move || {
        let mut count = 0usize;
        for _ in 0..values {
            let _ = rx.recv().expect("recv failed");
            count += 1;
            thread::sleep(Duration::from_millis(1));
        }
        count
    });

    producer.join().expect("producer panicked");
    consumer.join().expect("consumer panicked")
}

/// MPMC worker demo using crossbeam channels.
///
/// Sends jobs through one channel and gathers results from another.
pub fn mpmc_worker_pool_demo(workers: usize, jobs: usize) -> usize {
    let (job_tx, job_rx) = crossbeam_channel::bounded::<usize>(jobs.max(1));
    let (result_tx, result_rx) = crossbeam_channel::bounded::<usize>(jobs.max(1));

    let mut handles = Vec::with_capacity(workers);
    for _ in 0..workers {
        let rx = job_rx.clone();
        let tx = result_tx.clone();
        handles.push(thread::spawn(move || {
            while let Ok(job) = rx.recv() {
                // Work function = square for deterministic checks.
                tx.send(job * job).expect("result send failed");
            }
        }));
    }
    drop(result_tx);

    for j in 0..jobs {
        job_tx.send(j).expect("job send failed");
    }
    drop(job_tx);

    let total = (0..jobs)
        .map(|_| result_rx.recv().expect("result recv failed"))
        .sum::<usize>();

    for handle in handles {
        handle.join().expect("worker panicked");
    }

    total
}

#[cfg(test)]
mod tests {
    use super::{bounded_backpressure_demo, fan_in_sum, mpmc_worker_pool_demo};

    #[test]
    fn fan_in_collects_all_values() {
        // values are 0..12
        assert_eq!(fan_in_sum(3, 4), (0..12).sum::<usize>());
    }

    #[test]
    fn bounded_channel_consumes_everything() {
        assert_eq!(bounded_backpressure_demo(2, 20), 20);
    }

    #[test]
    fn mpmc_worker_pool_returns_expected_square_sum() {
        // squares 0..9
        assert_eq!(mpmc_worker_pool_demo(3, 10), (0..10).map(|x| x * x).sum());
    }
}
