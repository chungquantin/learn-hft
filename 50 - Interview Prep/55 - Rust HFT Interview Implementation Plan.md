---
title: Rust HFT Interview Implementation Plan
tags: [hft, rust, interview, implementation, roadmap]
---

# Rust HFT Interview Implementation Plan

This branch is an implementation-first study plan for building and explaining a scalable HFT stack in Rust, with a distributed architecture and TigerBeetle-backed ledgering.

Use this as your operating note for the next 12 to 16 weeks.

## Outcomes

By the end of this plan, you should be able to:

- implement a deterministic order book and matching core in Rust
- design process and service boundaries for low-latency and fault-tolerant operation
- integrate TigerBeetle for double-entry balances, settlement, and reconciliation
- explain latency, correctness, and failure tradeoffs clearly in technical interviews
- present a production-style portfolio project with benchmarks and incident playbooks

## How To Use This Branch

1. Follow the phase plan in [[56 - Rust HFT 16-Week Implementation Path]]
2. Build engine internals using [[57 - Matching Engine Component Plan]]
3. Build state and data pipelines using [[58 - Order Book and Market Data Component Plan]]
4. Split into distributed services with [[59 - Distributed Topology and Reliability Component Plan]]
5. Add ledger and settlement correctness via [[60 - TigerBeetle Integration Component Plan]]
6. Practice interview delivery using [[61 - HFT Interview Drills and Portfolio Packaging]]

## Interview Narrative Template

Use this structure to answer most system design and implementation questions:

1. Problem statement: throughput, latency target, correctness target
2. Core invariants: price-time priority, idempotency, reconciliation, risk limits
3. Data model: event schema, order states, ledger posting types
4. Hot path: market data -> strategy -> risk -> execution -> ack/fill handling
5. Failure handling: gaps, duplicates, reordering, partial service outage
6. Measurement: p50/p99/p99.9 latency, drop rates, replay determinism, recovery RTO
7. Tradeoffs: why this design, what you rejected, and how you would scale next

## Exit Criteria

You are "interview ready" when you can do all of the following without notes:

- whiteboard end-to-end architecture in under 10 minutes
- write core matching or book update code in 30 to 45 minutes
- explain TigerBeetle consistency model and why it is used for post-trade truth
- discuss two real incidents you simulated and how your design recovers
- show benchmark evidence and explain where latency budget is spent

Related:

- [[30 - Backend Systems Hub]]
- [[32 - Order Book Engine Deep Dive]]
- [[35 - Service Boundaries and Process Topology]]
- [[36 - Reliability, Failure Modes, and Recovery]]
