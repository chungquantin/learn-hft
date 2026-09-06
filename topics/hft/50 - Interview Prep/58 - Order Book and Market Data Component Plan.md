---
title: Order Book and Market Data Component Plan
tags: [orderbook, market-data, rust, low-latency, interview]
---

# Order Book and Market Data Component Plan

This note covers the ingestion-to-book pipeline, where correctness and trust state are established.

## Responsibilities

Market data ingestion owns:

- feed connectivity and decoding
- sequencing and continuity checks
- normalized event emission

Order book engine owns:

- snapshot + delta reconstruction
- trusted local state publication
- derived feature generation for strategy/risk

## Trust-State Design

Book trust state:

- `Healthy`: fully synchronized and sequence-continuous
- `Degraded`: temporary inconsistency detected, trading restrictions active
- `Rebuilding`: snapshot reload and delta catch-up in progress
- `Stale`: no valid updates inside configured timeout window

Interview point:

- make trust state explicit and feed it into strategy/risk gating

## Data Structures

Recommended progression:

1. start with `BTreeMap<Tick, Level>`
2. benchmark and evolve to contiguous ladder around active window
3. add optional deep book side-channel for research only

Level model:

- `price_tick`
- `aggregate_qty`
- optional `order_count`
- optional queue metadata (if venue feed supports it)

## Continuity and Recovery

Required controls:

- strict sequence monotonic checks
- duplicate sequence suppression
- gap detection with bounded wait window
- snapshot reload with atomic state swap

Recovery pipeline:

1. detect gap
2. freeze signal/risk actions requiring trusted book
3. request/reload snapshot
4. reapply buffered deltas
5. verify sequence continuity and re-enable

## Derived Signal Contract

Expose a compact, immutable snapshot to downstream consumers:

- top-of-book
- spread and mid
- imbalance at configurable depth bands
- microprice
- rolling short-window volatility proxy

This reduces coupling and prevents downstream code from mutating book internals.

## Testing and Benchmarking

Correctness tests:

- snapshot then ordered deltas
- duplicate delta resilience
- out-of-order rejection
- recovery after synthetic gap

Performance tests:

- update throughput under bursty feed
- read latency under concurrent snapshot consumers
- cache miss and branch miss profiling

## Interview Questions To Drill

- "How do you prove your local book is trustworthy?"
- "What happens during a market data gap?"
- "How do you avoid strategy trading on stale state?"

Related:

- [[31 - Market Data Ingestion Deep Dive]]
- [[32 - Order Book Engine Deep Dive]]
- [[18 - Time and Timestamp Semantics]]
