//! Exercise: `green_threads_async`
//!
//! Goal:
//! - Practice async task scheduling and cooperative concurrency.
//!
//! Instructions:
//! 1. Create an async workload with multiple independent tasks.
//! 2. Add bounded concurrency (limit max in-flight tasks).
//! 3. Propagate cancellation from parent to child tasks.
//! 4. Add timeout/retry logic for one task category.
//! 5. Collect task outcomes and summarize success/error counts.
//! 6. Add tests for cancellation, timeout, and bounded parallelism behavior.
//!
//! Method Hints:
//! - `tokio::spawn` / `join!` / `join_all`
//! - `tokio::sync::Semaphore` for bounded concurrency
//! - `tokio::time::timeout`
//! - `tokio::select!` for cancellation-aware flows

use std::sync::Arc;
use tokio::sync::{Semaphore, mpsc};
use tokio::time::{Duration, timeout};
use tokio_util::sync::CancellationToken;

#[derive(Debug)]
enum TaskResult {
    Success(usize),
    Error(String),
}

async fn worker_task(
    id: usize,
    sem: Arc<Semaphore>,
    cancel_token: CancellationToken,
) -> TaskResult {
    // 2. Bounded Concurrency: Acquire permit
    let _permit = sem.acquire().await.expect("Semaphore closed");

    // 3. Cancellation Aware Flow
    tokio::select! {
        _ = cancel_token.cancelled() => {
            TaskResult::Error(format!("Task {} cancelled", id))
        }
        res = async {
            // Simulate work
            tokio::time::sleep(Duration::from_millis(100)).await;
            if id % 5 == 0 { return Err("Hardware fault".to_string()); }
            Ok(id * 10)
        } => {
            match res {
                Ok(val) => TaskResult::Success(val),
                Err(e) => TaskResult::Error(e),
            }
        }
    }
}

pub async fn run_async_demo() {
    let num_tasks = 20;
    let max_concurrency = 4;
    let semaphore = Arc::new(Semaphore::new(max_concurrency));
    let cancel_token = CancellationToken::new();

    let mut handles = Vec::new();

    for i in 0..num_tasks {
        let sem = Arc::clone(&semaphore);
        let token = cancel_token.clone();

        // 1. tokio::spawn for independent tasks
        handles.push(tokio::spawn(worker_task(i, sem, token)));
    }

    // 5. Collect Outcomes
    let mut successes = 0;
    let mut errors = 0;

    for handle in handles {
        match handle.await.unwrap() {
            TaskResult::Success(_) => successes += 1,
            TaskResult::Error(_) => errors += 1,
        }
    }

    println!("Summary: {} successes, {} errors", successes, errors);
}
