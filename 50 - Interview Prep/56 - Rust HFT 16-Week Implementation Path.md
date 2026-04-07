---
title: Rust HFT 16-Week Implementation Path
tags: [hft, rust, implementation, study-plan, interview]
---

# Rust HFT 16-Week Implementation Path

This plan is grouped into six phases. Each phase has build goals, learning goals, and interview goals.

## Phase 0 (Week 1): Environment and Baseline

Build goals:

- create Rust workspace with crates: `common`, `md_ingest`, `orderbook`, `matching`, `risk`, `exec`, `ledger`, `ops`
- define canonical event schema (`MarketEvent`, `OrderIntent`, `ExecutionReport`, `LedgerEvent`)
- set up deterministic replay harness and benchmark harness (`criterion` + custom latency histograms)

Learning goals:

- ownership and borrowing under hot-path constraints
- lock contention vs message passing tradeoffs
- monotonic clocks and timestamp semantics

Interview goals:

- explain crate boundaries and why they map to ownership boundaries
- explain deterministic replay as both test and debug infrastructure

## Phase 1 (Weeks 2-3): Order Book Core

Build goals:

- implement price-level order book with deterministic update semantics
- support snapshot bootstrapping and delta application
- implement gap detection, sequence checks, and invalidation + rebuild path
- expose derived signals: best bid/ask, spread, mid, imbalance

Learning goals:

- data structure tradeoffs (`BTreeMap`, indexed array ladder, custom slab)
- top-of-book optimization and cache locality
- correctness-first state transitions

Interview goals:

- explain why state trust is explicit (`Healthy`, `Degraded`, `Rebuilding`)
- code and explain snapshot+delta recovery flow

## Phase 2 (Weeks 4-6): Matching and Execution Lifecycle

Build goals:

- implement matching core: limit, market, IOC, post-only, cancel/replace
- preserve price-time priority and deterministic fill ordering
- implement order state machine and idempotent command handling
- add self-trade prevention and reject reasons

Learning goals:

- invariants-driven design
- event-sourced state transitions
- tail-latency aware coding patterns

Interview goals:

- write matching pseudocode on a whiteboard quickly
- explain edge cases: crossing updates, partial fill, cancel race, amend semantics

## Phase 3 (Weeks 7-9): Distributed Topology and Risk Boundaries

Build goals:

- split into process topology: ingestion, book/matching, risk, execution gateway, ledger writer
- define message bus contracts and backpressure semantics
- implement kill-switch, position limits, notional limits, and max-loss guardrails
- implement dead-letter and retry policies for non-hot-path events

Learning goals:

- when to isolate by thread, process, or host
- bounded queues and overload behavior
- fault containment and graceful degradation

Interview goals:

- justify boundaries with latency and failure isolation reasoning
- explain exactly what happens when one service becomes unhealthy

## Phase 4 (Weeks 10-12): TigerBeetle Ledger and Settlement

Build goals:

- design account model (cash, reserved cash, position, fees, pnl buckets)
- map execution/fill events to balanced TigerBeetle transfers
- add idempotency keys, dedupe store, and replay-safe posting
- build reconciliation jobs: execution stream vs TigerBeetle balances

Learning goals:

- double-entry accounting for trading systems
- eventual consistency boundaries between hot trading loop and settlement truth
- operational recovery from partial posting failures

Interview goals:

- explain why ledgering is off hot path but still critical
- explain how TigerBeetle gives correctness under retries and restarts

## Phase 5 (Weeks 13-16): Production Hardening and Interview Packaging

Build goals:

- latency budget dashboard by stage (`ingest`, `book`, `risk`, `exec`, `ledger enqueue`)
- chaos tests: dropped deltas, duplicate messages, out-of-order fills, ledger outage
- create runbooks and incident timelines
- finalize architecture diagram + benchmark report + demo script

Learning goals:

- operational readiness and observability in low-latency systems
- communicating tradeoffs with evidence

Interview goals:

- present 10-minute architecture walkthrough
- handle adversarial follow-ups on consistency and failure modes

## Weekly Cadence

For each week:

1. Build one concrete deliverable
2. Write one short design note: assumptions, invariants, tradeoffs
3. Add one benchmark before/after result
4. Practice one 20-minute mock interview focusing on the current phase

Related:

- [[15 - Benchmarking and Tick-to-Trade Measurement]]
- [[26 - Building a Low-Latency Trading Engine]]
- [[61 - HFT Interview Drills and Portfolio Packaging]]
