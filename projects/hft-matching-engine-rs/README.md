# hft-matching-engine-rs

Low-latency matching engine primitives for HFT-style system design in Rust.

## What is implemented

- Heap-backed order book:
  - max-heap bids
  - min-heap asks
  - FIFO queue per price level for strict price-time priority
- Order index map for targeted cancel by `OrderId`
- `EngineCommand` API with `New`, `Cancel`, and `Replace`
- Deterministic matching loop
- Idempotent command suppression
- Bounded thread-safe SPSC ring buffer for ingress handoff
- Partition runtime for horizontal scaling by order-id routing
- In-memory replay log for deterministic rebuild/testing
- Simulation module for synthetic HFT-style stress runs
- Benchmark scaffold with direct and ring-buffer ingress paths

## Concurrency model

- Matching state is designed for single-writer mutation.
- For multi-threaded ingestion, producers enqueue commands into `SpscRingBuffer`.
- Matching thread drains ingress and processes in deterministic order.
- `ConcurrentMatchingEngine` provides a mutex-guarded wrapper for shared runtime integration.
- `PartitionRuntime` provides N shard-local engines and per-shard ingress queues.
- `simulation` generates synthetic command flow and reports throughput/drop metrics.

## Quick start

```rust
use hft_matching_engine_rs::{
    IdempotencyKey, MatchingEngine, OrderCommand, OrderId, OrderType, Side, TimeInForce,
};

let mut engine = MatchingEngine::default();
let events = engine.on_command(OrderCommand {
    idempotency_key: IdempotencyKey(1),
    order_id: OrderId(42),
    side: Side::Buy,
    order_type: OrderType::Limit,
    tif: TimeInForce::Gtc,
    price_ticks: 10_000,
    quantity: 5,
});

assert!(!events.is_empty());
```

## Run checks

```bash
cargo check --lib
cargo test
cargo bench
```

## Simulation quick start

```rust
use hft_matching_engine_rs::{run_partitioned_simulation, SimulationConfig};

let report = run_partitioned_simulation(SimulationConfig::default());
println!("{report:?}");
```

## Notes

- The ring buffer is SPSC only; do not use multiple producers or multiple consumers on one queue.
- The current engine focuses on correctness and deterministic behavior first.
- Next evolution typically includes partition runtime, order cancel/replace flows, and persistent replay logs.
