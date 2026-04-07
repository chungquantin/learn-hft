//! Minimal fixed-size thread pool implementation.
//!
//! Learning goals:
//! - worker threads waiting on a job queue
//! - graceful shutdown semantics
//! - collecting outputs via channels
//!
//! This is intentionally small and educational, not production-grade.

use std::sync::{Arc, Mutex, mpsc};
use std::thread::{self, JoinHandle};

type Job = Box<dyn FnOnce() + Send + 'static>;

/// Fixed worker pool that executes jobs submitted via `execute`.
pub struct ThreadPool {
    workers: Vec<JoinHandle<()>>,
    sender: Option<mpsc::Sender<Job>>,
}

impl ThreadPool {
    /// Creates a pool with `size` worker threads.
    pub fn new(size: usize) -> Self {
        assert!(size > 0, "thread pool size must be > 0");
        let (tx, rx) = mpsc::channel::<Job>();
        let shared_rx = Arc::new(Mutex::new(rx));
        let mut workers = Vec::with_capacity(size);

        for _ in 0..size {
            let local_rx = Arc::clone(&shared_rx);
            workers.push(thread::spawn(move || {
                loop {
                    let msg = local_rx
                        .lock()
                        .expect("receiver mutex poisoned")
                        .recv();
                    match msg {
                        Ok(job) => job(),
                        Err(_) => break, // Sender dropped => shutdown.
                    }
                }
            }));
        }

        Self {
            workers,
            sender: Some(tx),
        }
    }

    /// Submits one task for execution.
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        self.sender
            .as_ref()
            .expect("pool already shut down")
            .send(Box::new(f))
            .expect("failed to submit task");
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        // Close job channel first so workers break out of recv loop.
        self.sender.take();
        // Join workers to prevent zombie-like leaked worker threads.
        for worker in self.workers.drain(..) {
            worker.join().expect("worker panicked");
        }
    }
}

/// Demonstrates pool usage by computing sum of squares in parallel.
pub fn thread_pool_sum_of_squares(workers: usize, n: usize) -> usize {
    let pool = ThreadPool::new(workers);
    let (result_tx, result_rx) = mpsc::channel::<usize>();

    for i in 0..n {
        let tx = result_tx.clone();
        pool.execute(move || {
            tx.send(i * i).expect("failed to send result");
        });
    }
    drop(result_tx);

    // Pool drops here => joins all workers before function exits.
    drop(pool);

    result_rx.into_iter().sum::<usize>()
}

#[cfg(test)]
mod tests {
    use super::thread_pool_sum_of_squares;

    #[test]
    fn thread_pool_executes_all_jobs() {
        assert_eq!(thread_pool_sum_of_squares(4, 10), (0..10).map(|x| x * x).sum());
    }
}
