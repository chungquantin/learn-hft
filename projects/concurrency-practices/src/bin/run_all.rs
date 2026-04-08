//! Runs all concurrency practice demos from basic to advanced.

use concurrency_practices::atomics::{atomic_counter, claim_once};
use concurrency_practices::atomics_deep_dive::{
    OnceValue, cas_increment, relaxed_counter, release_acquire_publication,
};
use concurrency_practices::basic_threads::spawn_and_join_workers;
use concurrency_practices::channels_patterns::{
    bounded_backpressure_demo, fan_in_sum, mpmc_worker_pool_demo,
};
use concurrency_practices::deadlock_patterns::concurrent_transfer_demo;
use concurrency_practices::green_threads_async::{bounded_concurrency_sum, run_async_workers};
use concurrency_practices::send_sync::{move_vec_across_thread, share_arc_across_threads};
use concurrency_practices::shared_state::{mutex_counter, rwlock_read_heavy_demo};
use concurrency_practices::thread_pool::thread_pool_sum_of_squares;
use concurrency_practices::thread_lifecycle::{
    clean_join_example, cooperative_shutdown, intentionally_forget_join_handle,
};
use std::sync::atomic::AtomicUsize;

#[tokio::main(flavor = "multi_thread", worker_threads = 2)]
async fn main() {
    println!("== basic_threads ==");
    println!("squares: {:?}", spawn_and_join_workers(6));

    println!("\n== shared_state ==");
    println!("mutex counter: {}", mutex_counter(4, 1_000));
    println!("rwlock read-heavy total: {}", rwlock_read_heavy_demo(4));

    println!("\n== send_sync ==");
    println!("move vec across thread sum: {}", move_vec_across_thread());
    println!("share Arc across threads total: {}", share_arc_across_threads(3));

    println!("\n== atomics ==");
    println!("atomic counter: {}", atomic_counter(4, 1_000));
    let once_flag = AtomicUsize::new(0);
    println!("claim_once #1: {}", claim_once(&once_flag));
    println!("claim_once #2: {}", claim_once(&once_flag));

    println!("\n== atomics_deep_dive ==");
    println!("relaxed_counter: {}", relaxed_counter(4, 1_000));
    println!(
        "release_acquire_publication(123): {}",
        release_acquire_publication(123)
    );
    let cas_counter = AtomicUsize::new(0);
    for _ in 0..10 {
        cas_increment(&cas_counter);
    }
    println!(
        "cas_increment after 10 calls: {}",
        cas_counter.load(std::sync::atomic::Ordering::Relaxed)
    );
    let once = OnceValue::new();
    println!("once get_or_init first: {}", once.get_or_init(|| 555));
    println!("once get_or_init second: {}", once.get_or_init(|| 999));

    println!("\n== thread_lifecycle ==");
    println!("clean join result: {}", clean_join_example());
    println!(
        "cooperative shutdown loops observed: {}",
        cooperative_shutdown(8)
    );
    intentionally_forget_join_handle();
    println!("intentionally forgot one handle (demo-only)");

    println!("\n== green_threads_async ==");
    let mut workers = run_async_workers(8).await;
    workers.sort_unstable();
    println!("async worker outputs: {:?}", workers);
    println!(
        "bounded concurrency sum (0..20): {}",
        bounded_concurrency_sum(20, 4).await
    );

    println!("\n== channels_patterns ==");
    println!("fan-in sum (3x4): {}", fan_in_sum(3, 4));
    println!(
        "bounded backpressure consumed: {}",
        bounded_backpressure_demo(2, 20)
    );
    println!("mpmc square-sum (jobs=10): {}", mpmc_worker_pool_demo(3, 10));

    println!("\n== deadlock_patterns ==");
    println!(
        "concurrent transfer total balance (should remain 2000): {}",
        concurrent_transfer_demo()
    );

    println!("\n== thread_pool ==");
    println!(
        "thread-pool sum of squares (0..10): {}",
        thread_pool_sum_of_squares(4, 10)
    );
}
