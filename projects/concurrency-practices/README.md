# concurrency-practices

Progressive Rust concurrency practice project from basic to advanced.

## Topics covered

1. Basic threads (`std::thread`)
2. Shared state (`Arc`, `Mutex`, `RwLock`)
3. `Send` / `Sync` fundamentals
4. Atomics (`AtomicUsize`, `compare_exchange`)
5. Atomics deep dive (memory ordering, spin lock, once init)
6. Thread lifecycle pitfalls (join discipline, zombie-like leaks)
7. Async task scheduling (green-thread style with Tokio)
8. Channel patterns (`mpsc`, `sync_channel`, `crossbeam-channel`)
9. Deadlock-safe lock ordering patterns
10. Fixed-size thread pool implementation

## Run all demos

```bash
cargo run -p concurrency-practices --bin run_all
cargo run -p concurrency-practices --bin atomics_lab
```

## Beginner-friendly atomics guide

- Read first: [docs/atomics_beginner_guide.md](docs/atomics_beginner_guide.md)
- Then run: `cargo run -p concurrency-practices --bin atomics_lab`

## Run tests

```bash
cargo test -p concurrency-practices
```

## Suggested learning order

- Read each module top-doc first in `src/*.rs`
- Run `run_all` and inspect outputs
- Then open module tests to see expected behavior

## Deeper study path

1. Start with `basic_threads.rs` and `shared_state.rs`
2. Read `send_sync.rs` until trait boundaries feel intuitive
3. Move to `atomics.rs` and reason about memory ordering assumptions
4. Practice `channels_patterns.rs` to shift from shared-state to message-passing
5. Study `deadlock_patterns.rs` and enforce lock-order rules in your own code
6. Read `thread_pool.rs` end-to-end and modify it (timeouts, queue bounds, cancellation)
7. Finish with `green_threads_async.rs` and compare async tasks vs OS threads

## Atomics-focused study plan

1. Read `src/atomics.rs` first (simple `Relaxed` + basic CAS)
2. Then read `src/atomics_deep_dive.rs` in order:
   - `relaxed_counter`
   - `release_acquire_publication`
   - `cas_increment`
   - `SpinLock`
   - `OnceValue`
3. Run:
   - `cargo run -p concurrency-practices --bin atomics_lab`
4. Modify ordering choices and re-run tests to see what breaks.
