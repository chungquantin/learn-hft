---
title: Exchange Architecture
tags: [exchange, systems, connectivity]
---

# Exchange Architecture

A trading system usually splits into market data ingestion, state management, decision logic, execution, and supervision.

This split exists because each part has different latency, correctness, and ownership requirements.

If all of them are mixed together, the system becomes difficult to reason about:

- market-data bugs leak into signal logic
- risk checks become implicit
- execution state gets confused with strategy intent
- replay becomes difficult or impossible

## Canonical pipeline

1. Connect to websocket / multicast / REST endpoints
2. Parse and normalize exchange messages
3. Maintain local order book and positions
4. Run signal logic
5. Run pre-trade risk checks
6. Send orders
7. Reconcile acknowledgements, fills, cancels, rejects

Notice that "send orders" is not the end. In a real engine, reconciliation is what turns intent into trusted state.

## Components

- market data gateway
- local book builder
- strategy engine
- execution gateway
- risk engine
- persistence and replay
- monitoring and alerting

It helps to think of these as two classes:

Hot path:

- market data gateway
- local book builder
- strategy engine
- execution gateway
- risk engine

Support path:

- persistence and replay
- monitoring and alerting

The hot path protects speed and correctness.

The support path protects explainability and long-term improvement.

## Internal handoff patterns

When you split the system across threads, a common pattern is:

- hot producer thread for market data or strategy output
- SPSC queue between adjacent stages
- fixed-capacity ring buffer semantics to avoid unbounded growth and lock contention

This is especially useful for side pipelines such as logging, persistence, and replay ingestion, where you want to decouple work without adding blocking behavior to the hot path.

The main design question is not "which queue is fastest?".

It is:

"What semantics does this handoff need?"

Examples:

- every order event matters
- some telemetry may be droppable
- some state is better published as the latest snapshot

That choice determines whether you want a queue, a seqlock, or strict local ownership.

## Shared-state patterns

- Use queues when you want ordered message passing between stages.
- Use seqlocks when you want a writer to publish the latest snapshot while readers retry on contention.
- Favor designs where slow consumers do not stall producers.

This distinction is a major architectural decision, not a minor implementation detail.

## Critical design constraints

- packet loss and reconnects
- sequencing and gap recovery
- clock synchronization
- deterministic replay
- rate limits
- idempotent order handling

These constraints exist because exchanges are external systems with their own failure modes.

A good architecture assumes:

- messages may be lost
- connections may flap
- acknowledgements may arrive late
- internal state may temporarily become invalid

The system should know when it is trustworthy and when it is not.

## For perpetual exchanges

Expect venue-specific differences in:

- funding calculation
- position modes
- liquidation rules
- order types
- websocket schemas
- checksum / book sync behavior

Treat these differences as first-class design inputs.

A system that pretends exchanges are interchangeable usually hides brittle assumptions in the wrong places.

Related:

- [[02 - Rust for HFT]]
- [[03 - Perpetuals Trading]]
- [[08 - Build Projects]]
- [[19 - Matching Engines, Queue Priority, and Order Amend Semantics]]
- [[10 - Ring Buffers in Rust]]
- [[11 - Seqlocks]]
- [[12 - Low-Latency Logging and Telemetry]]
