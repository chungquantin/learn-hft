---
title: Ultra-Low-Latency Matching Engine Design
tags: [matching-engine, hft, latency, rust, system-design, interview]
---

# Ultra-Low-Latency Matching Engine Design

This note is a focused design blueprint for a matching engine in an HFT context.

Important framing:

- "sub-second" is acceptable for many trading systems
- for true HFT competitiveness, target is usually microseconds to low milliseconds
- design for microsecond-class internal latency, then enforce SLOs with p99/p99.9 metrics

## Core Goals

Primary goals:

- deterministic matching behavior
- strict price-time priority
- predictable tail latency under burst load
- operational recoverability without financial inconsistency

Non-goals:

- rich business workflow in hot path
- synchronous database calls in matching loop
- cross-partition global ordering

## Latency Budget (Per Order Command)

Reference budget for one partition:

- ingress decode + validate: `5-20us`
- book/match loop: `3-25us`
- risk gate (local fast checks): `2-15us`
- event publish enqueue: `2-10us`
- total internal p99 target: `<= 100us`

External venue/network adds additional latency outside engine core.

## Architecture Pattern

Use single-writer partitioned cores:

1. partition by `(venue, instrument_id)`
2. one pinned thread owns one partition state
3. commands arrive through lock-free bounded queue
4. matching thread mutates state without locks
5. outputs are immutable events to downstream services

Why:

- avoids lock contention in hot path
- deterministic event order per partition
- easy replay and recovery model

## Data Structures

Order indexing:

- `OrderId -> OrderRecord` in slab/arena

Price levels:

- bids and asks as tick-indexed structures
- per-level intrusive FIFO list for time priority

Command/event records:

- fixed-size structs where possible
- avoid dynamic allocation in hot path

## Matching Loop Contract

For each `OrderCommand`:

1. validate (instrument, side, qty, tick, flags)
2. dedupe/idempotency check by stable command key
3. if marketable, cross against opposite side FIFO
4. emit trade events for each fill fragment
5. if residual quantity remains and allowed, rest order
6. emit final order state event

Invariants:

- total fill qty cannot exceed original qty
- FIFO preserved within same price level
- crossing only when price constraints satisfied
- idempotent replay emits identical event stream

## Concurrency Model

Inside partition:

- no shared mutable state across threads
- single writer + multiple reader snapshots

Across partitions:

- independent execution and scaling
- no synchronous cross-partition dependencies

Control plane:

- async commands (halt, drain, resume, snapshot trigger)
- never block matching loop on control plane I/O

## Failure and Recovery

Failure classes:

- process crash
- queue overload
- malformed/duplicate command stream
- downstream consumer lag

Recovery design:

- write-ahead event log per partition
- periodic snapshots of book + live orders
- restart by loading snapshot + replaying log suffix
- idempotent command handling for duplicate upstream delivery

## Backpressure and Overload

Rules:

- bounded ingress queue per partition
- explicit rejection/defer policy when queue near full
- priority classes (cancel/risk commands can preempt new orders if policy requires)

Metrics:

- queue depth
- enqueue fail rate
- command age at dequeue
- p50/p99/p99.9 latency

## Risk Integration Pattern

Fast checks in hot path:

- max order size
- price band sanity
- account hard-stop flag

Complex checks out of hot path:

- portfolio-wide analytics
- cross-venue exposure optimization

Fail-safe:

- if risk state uncertain, fail closed for new risk-increasing commands

## Bench and Verification Plan

Microbenchmarks:

- matching loop per command type
- partial fill storm
- cancel storm against deep queue

Determinism tests:

- same seed input -> byte-identical output events

Soak tests:

- sustained burst traffic with periodic snapshots
- restart and replay validation

Chaos tests:

- drop/reorder duplicate command injections
- downstream lag and temporary sink outage

## Interview Delivery Script (5-7 Minutes)

1. Define objectives: deterministic, low-tail latency, recoverable
2. Explain partition + single-writer model
3. Explain data structures for FIFO and cache locality
4. Explain event sourcing + snapshot replay recovery
5. Explain overload policy and risk fail-closed behavior
6. Show latency budget and benchmark evidence

## Related Build Notes

- [[57 - Matching Engine Component Plan]]
- [[59 - Distributed Topology and Reliability Component Plan]]
- [[60 - TigerBeetle Integration Component Plan]]
- [[61 - HFT Interview Drills and Portfolio Packaging]]
