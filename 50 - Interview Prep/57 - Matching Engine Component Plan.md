---
title: Matching Engine Component Plan
tags: [matching, orderbook, execution, rust, interview]
---

# Matching Engine Component Plan

This note defines how to implement and explain a production-style matching engine in Rust.

## Responsibilities

The matching engine owns:

- order acceptance and validation
- price-time priority matching
- fill generation and trade event emission
- deterministic order state transitions
- audit-friendly event output for replay and reconciliation

It does not own:

- market data ingestion normalization
- strategy signal generation
- long-term ledger settlement

## Core Data Model

Minimum entities:

- `OrderId`, `ClientOrderId`, `InstrumentId`
- `Side`, `OrderType`, `TimeInForce`
- `OrderState` (`New`, `Working`, `PartiallyFilled`, `Filled`, `Canceled`, `Rejected`)
- `ExecutionEvent` (`Accepted`, `Trade`, `Canceled`, `Rejected`, `Expired`)

Implementation target:

- intrusive per-price FIFO queue for resting orders
- price ladder keyed by tick integer, not floating price
- stable sequence numbers for all state-changing events

## Invariants (Must Never Break)

- no trade prints outside crossing conditions
- total traded quantity on an order never exceeds original quantity
- cancel/replace can only act on live orders
- event ordering is deterministic for equal input stream
- each command is idempotent by `(venue, account, client_order_id, command_seq)`

## Command Flow

1. Receive `OrderCommand`
2. Validate instrument, side, quantity, tick size, risk pre-check token
3. If accepted, apply matching loop
4. Emit `ExecutionEvent` stream
5. Publish updates to:
   - execution gateway
   - risk position tracker
   - ledger enqueue topic
   - telemetry

## Performance Plan

Latency budget target example (single instrument shard):

- `p50 < 8us`
- `p99 < 40us`
- `p99.9 < 120us`

Optimization checklist:

- pre-allocate order objects from slab/pool
- avoid heap allocation in hot matching loop
- avoid string handling in hot path
- batch outbound event publication where safe
- pin matching thread core and isolate noisy neighbors

## Testing Plan

Unit tests:

- simple crossing and non-crossing
- partial fill chains
- cancel before/after fill race
- post-only reject behavior

Property tests:

- generated command stream preserves invariants
- deterministic replay equality for same seed

Scenario tests:

- burst load with cancel storm
- duplicate command replay with idempotency
- out-of-order command input rejection path

## Interview Questions To Drill

- "How do you enforce price-time priority under concurrency?"
- "Why not use one global lock?"
- "How do you recover from process crash without duplicating fills?"
- "What’s your strategy for sharding by symbol and preserving ordering?"

Expected response pattern:

1. define ordering contract
2. show single-writer or partitioned-writer model
3. explain event sourcing and replay
4. explain idempotency and dedupe boundaries

Related:

- [[19 - Matching Engines, Queue Priority, and Order Amend Semantics]]
- [[32 - Order Book Engine Deep Dive]]
- [[33 - Execution Management Deep Dive]]
