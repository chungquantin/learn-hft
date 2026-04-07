//! Async task examples (green-thread style scheduling).
//!
//! Why "green threads":
//! - async tasks are cooperatively scheduled units managed by a runtime
//! - thousands of tasks can multiplex onto a smaller OS-thread pool
//! - great for high-concurrency I/O workflows
//!
//! Here we use Tokio as the runtime.

use tokio::task::JoinSet;
use tokio::time::{Duration, sleep};

/// Runs many async tasks and gathers their outputs.
///
/// This demonstrates structured concurrency with `JoinSet`.
pub async fn run_async_workers(worker_count: usize) -> Vec<usize> {
    let mut set = JoinSet::new();
    for i in 0..worker_count {
        set.spawn(async move {
            sleep(Duration::from_millis(2)).await;
            i + 1
        });
    }

    let mut out = Vec::with_capacity(worker_count);
    while let Some(res) = set.join_next().await {
        out.push(res.expect("task panicked"));
    }
    out
}

/// Demonstrates bounded concurrency with a semaphore.
pub async fn bounded_concurrency_sum(tasks: usize, max_in_flight: usize) -> usize {
    let semaphore = std::sync::Arc::new(tokio::sync::Semaphore::new(max_in_flight));
    let mut set = JoinSet::new();

    for i in 0..tasks {
        let sem = std::sync::Arc::clone(&semaphore);
        set.spawn(async move {
            let _permit = sem.acquire_owned().await.expect("semaphore closed");
            sleep(Duration::from_millis(1)).await;
            i
        });
    }

    let mut total = 0usize;
    while let Some(res) = set.join_next().await {
        total += res.expect("task panicked");
    }
    total
}

#[cfg(test)]
mod tests {
    use super::{bounded_concurrency_sum, run_async_workers};

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn async_workers_return_all_outputs() {
        let mut out = run_async_workers(5).await;
        out.sort_unstable();
        assert_eq!(out, vec![1, 2, 3, 4, 5]);
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn bounded_concurrency_computes_expected_sum() {
        // Sum 0..10 = 45
        assert_eq!(bounded_concurrency_sum(10, 3).await, 45);
    }
}
