---
title: Rust Matching Engine Implementation Blueprint
tags: [rust, matching-engine, implementation, blueprint, interview]
---

# Rust Matching Engine Implementation Blueprint

This note maps the low-latency design into concrete Rust modules and interfaces.

## Project Layout

```text
projects/hft-matching-engine-rs/
  Cargo.toml
  src/
    lib.rs
    types.rs
    command.rs
    event.rs
    orderbook.rs
    matcher.rs
    engine.rs
  benches/
    matching_latency.rs
```

## Module Responsibilities

- `types.rs`: core IDs and enums (`OrderId`, `Side`, `OrderType`, `TimeInForce`)
- `command.rs`: inbound command model and idempotency key
- `event.rs`: outbound deterministic event model
- `orderbook.rs`: price levels, resting order queues, mutation primitives
- `matcher.rs`: matching loop logic and invariants
- `engine.rs`: single-writer orchestration, command dispatch, event emission

## Core Traits

```rust
pub trait EventSink {
    fn on_event(&mut self, event: ExecutionEvent);
}

pub trait Clock {
    fn now_nanos(&self) -> u64;
}
```

Why:

- `EventSink` decouples hot path from transport choices
- `Clock` enables deterministic tests by swapping time source

## State Model

Engine state:

- `OrderBook` with bid/ask ladders
- `HashMap<OrderId, LiveOrderMeta>`
- `HashSet<IdempotencyKey>` for duplicate command suppression
- `u64 next_event_seq` monotonic event sequence

Single-writer rule:

- only one thread mutates this state for a partition

## Command Handling Pseudocode

```text
on_command(cmd):
  if dedupe_seen(cmd.idempotency_key): return
  validate(cmd)
  emit(Accepted or Rejected)
  if accepted:
    match_against_book(cmd)
    if residual and should_rest:
      rest_on_book(residual)
    emit(final_order_state)
```

## Invariant Checklist

- no negative quantities
- no over-fill
- FIFO preserved within level
- deterministic output sequence for deterministic input
- cancel/replace only for working orders

## Interview Talking Points

- why single-writer partitioning beats shared-lock models for p99 latency
- how idempotency avoids duplicate side effects during retries/replay
- how event sequence IDs support replay and reconciliation

Related:

- [[62 - Ultra-Low-Latency Matching Engine Design]]
- [[57 - Matching Engine Component Plan]]
