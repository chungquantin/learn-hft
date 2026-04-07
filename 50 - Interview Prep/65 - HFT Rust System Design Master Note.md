---
title: HFT Rust System Design Master Note
tags: [hft, rust, system-design, algorithms, interview, master-note]
---

# HFT Rust System Design Master Note

This is a single-note reference for designing and implementing an HFT system in Rust, with emphasis on matching engine design, low-latency operation, distributed reliability, and interview readiness.

## 1. Problem Framing

An HFT system is a real-time decision and execution machine under strict constraints:

- low latency and predictable tail behavior
- high throughput during bursts
- deterministic and auditable state transitions
- strong risk control under uncertainty
- operational resilience during data and infrastructure failures

Success is not just speed. Success is preserving correctness and explainability while operating near latency limits.

## 2. End-to-End System Design

### 2.1 Logical Components

Hot path:

- market data gateway
- local order book engine
- signal/strategy logic
- risk gate
- matching/execution core
- execution gateway (venue connectivity)

Warm/cold path:

- ledger/settlement writer (e.g., TigerBeetle integration)
- telemetry, metrics, tracing, logging
- reconciliation and backfill jobs
- operator control plane and incident tooling

### 2.2 Dataflow

Core loop:

1. receive market events
2. apply sequencing and trust checks
3. update local order book state
4. compute strategy intent
5. run risk checks
6. submit/modify/cancel orders
7. process acknowledgements/fills/rejects
8. emit deterministic events for persistence, ledgering, and observability

### 2.3 Service Boundaries

Use ownership-driven boundaries:

- ingestion owns what venue said
- order book owns trusted market state
- strategy owns interpretation
- risk owns permission
- execution owns venue interaction truth
- ledger owns financial truth

Boundary principles:

- single-writer ownership for mutable hot state
- no synchronous dependency from matching loop to slow services
- isolate failure domains by partition and responsibility

### 2.4 Partitioning and Scaling

Common partition key:

- `(venue, instrument_id)` or `instrument_id`

Rules:

- one writer thread/process per partition
- preserve strict ordering per partition
- avoid cross-partition synchronous operations in hot path
- aggregate cross-partition views asynchronously

## 3. Key Design Considerations

### 3.1 Latency Discipline

Important distinction:

- "sub-second" latency is not HFT-grade on its own
- for HFT internals, target microsecond to low-millisecond ranges

Track:

- `p50`, `p99`, `p99.9`, and max spikes
- per-stage budget (ingest, book, risk, match, emit)
- queue wait time and command age

### 3.2 Correctness and Determinism

Must-have properties:

- deterministic output for deterministic input
- explicit event sequencing
- idempotent command processing
- replayable state reconstruction
- invariant checks on every state mutation

### 3.3 Risk and Safety

Risk controls split:

- fast synchronous checks in hot path
- deeper portfolio analytics off hot path

Fail-safe behavior:

- fail closed when risk state is uncertain
- kill switch with immediate effect
- bounded exposure by symbol/account/venue

### 3.4 Reliability and Recovery

Recovery primitives:

- write-ahead event log
- periodic snapshots
- replay-on-restart
- dedupe on retried delivery

Failure containment:

- bounded queues
- overload shedding policy
- degraded mode and clear operator signaling

### 3.5 Financial Consistency

Execution vs accounting:

- execution state is operational truth for immediate control
- ledger (e.g., TigerBeetle) is financial truth for balances/settlement

Requirement:

- exact double-entry outcomes after retries and restarts
- continuous reconciliation between execution and ledger streams

## 4. Core Algorithms and Data Structures

### 4.1 Order Book Reconstruction

Algorithm:

1. bootstrap from snapshot
2. apply ordered deltas
3. detect duplicates/gaps/out-of-order events
4. invalidate and rebuild on continuity breach

Data structures:

- price ladder keyed by integer ticks
- per-level aggregate size and FIFO queue metadata
- top-of-book optimized fast path

### 4.2 Matching Engine (Price-Time Priority)

Basic algorithm for limit order:

1. validate order
2. while marketable and quantity remains:
   - take best opposite level
   - consume FIFO resting orders
   - emit trade events
3. if residual and allowed, rest on own side
4. emit final state event

Invariants:

- no overfill
- strict FIFO at same price
- no trade at invalid price
- deterministic fill sequence

### 4.3 Idempotency and Exactly-Once Effects (Practically)

Approach:

- assign stable idempotency key per command/event
- keep dedupe record on consumer side
- make downstream side effects retry-safe

Outcome:

- at-least-once transport with exactly-once business effect

### 4.4 Replay and Recovery

Algorithm:

1. load latest snapshot
2. replay log suffix in sequence order
3. rebuild in-memory indexes
4. verify invariants and sequence continuity
5. resume processing

### 4.5 Risk Algorithms (Minimal Set)

- static limits: max order size/notional
- dynamic limits: net position and short-window loss
- price guards: stale price and band checks
- kill-switch evaluation

### 4.6 Queue and Backpressure Control

Strategies:

- bounded lock-free queues
- priority classes (`cancel`/`risk` over new orders when stressed)
- drop/defer policies with explicit telemetry

## 5. Technical Problems in Real HFT Systems

### 5.1 Market Data Pathologies

- dropped messages
- out-of-order events
- sequence gaps
- stale snapshots

Mitigation:

- sequence validation, gap detection, rebuild flow

### 5.2 Latency Jitter and Tail Spikes

Causes:

- allocator pressure
- lock contention
- cache misses
- noisy neighbors and CPU scheduling
- GC pauses in non-Rust components

Mitigation:

- pre-allocation
- single-writer model
- core pinning and CPU isolation
- profiling by tail percentile

### 5.3 Concurrency Bugs

- race conditions around cancel/fill
- deadlocks in shared-state architectures
- inconsistent view across services

Mitigation:

- ownership isolation
- immutable event handoff
- deterministic replay tests

### 5.4 Distributed System Issues

- duplicate delivery
- partial outage
- split-brain decisions if control signals lag

Mitigation:

- idempotent consumers
- fail-closed policies for risk uncertainty
- clear service health gating in hot path

### 5.5 Financial Reconciliation Gaps

- execution happened but posting delayed
- retry ambiguity after timeout
- accidental double posting

Mitigation:

- stable transfer IDs
- dedupe before re-post
- reconciliation jobs with strict alert thresholds

## 6. Rust Knowledge Required

### 6.1 Language Fundamentals for HFT

- ownership and borrowing
- lifetimes for zero-copy-ish designs
- enums and pattern matching for state machines
- error handling with explicit failure paths
- trait-based abstraction without hot-path over-abstraction

### 6.2 Performance-Critical Rust

- stack vs heap awareness
- `Vec`, `VecDeque`, `BTreeMap`, `HashMap` tradeoffs
- avoiding allocations in hot loops
- minimizing clones and temporary objects
- branch prediction and cache locality awareness

### 6.3 Concurrency in Rust

- thread ownership models
- channels and queue patterns
- atomics and memory ordering basics
- when to avoid locks entirely via single writer

### 6.4 Unsafe Rust (Optional, Advanced)

Only when profiling justifies:

- custom memory pools/slabs
- intrusive structures
- FFI with specialized low-latency libs

Rule:

- isolate unsafe code behind small audited APIs

### 6.5 Tooling and Workflow

- `cargo check/test/bench`
- criterion benchmarks
- `perf` and flamegraphs
- tracing/metrics instrumentation
- property testing for invariants

## 7. Interview-Focused Answer Templates

### 7.1 System Design Prompt

Answer order:

1. clarify objectives and SLOs
2. define service boundaries by ownership
3. explain partitioning and ordering guarantees
4. describe matching/risk invariants
5. describe failure and replay recovery
6. present measurement and tradeoffs

### 7.2 Matching Engine Prompt

Highlight:

- single-writer partition
- price-time FIFO algorithm
- idempotent command processing
- deterministic event stream and replay

### 7.3 Reliability Prompt

Highlight:

- degrade vs halt policies
- bounded queues and overload strategy
- ledger reconciliation and exact financial outcomes

## 8. Practical Build Roadmap (Condensed)

1. build deterministic order book core
2. build matching engine with invariants and tests
3. add partitioned single-writer runtime
4. integrate risk gating and kill switch
5. add execution gateway adapters
6. add ledger writer and reconciliation
7. add benchmark + chaos + replay test suites
8. package architecture, benchmark evidence, and incident stories for interview

## 9. Final Checklist Before Interview

- can explain architecture in under 10 minutes
- can implement core matching logic on whiteboard/editor
- can discuss two failure scenarios and recovery path
- can justify major tradeoffs with measured evidence
- can explain Rust-specific design decisions under latency pressure

## Related Notes

- [[55 - Rust HFT Interview Implementation Plan]]
- [[56 - Rust HFT 16-Week Implementation Path]]
- [[57 - Matching Engine Component Plan]]
- [[60 - TigerBeetle Integration Component Plan]]
- [[62 - Ultra-Low-Latency Matching Engine Design]]
- [[63 - Rust Matching Engine Implementation Blueprint]]
- [[64 - Matching Engine Benchmark and Profiling Plan]]
