---
title: HFT Rust 30-Minute Interview Cheat Sheet
tags: [hft, rust, interview, cheat-sheet, revision]
---

# HFT Rust 30-Minute Interview Cheat Sheet

Use this note right before interviews for fast recall.

## 1. 60-Second System Pitch

"I design HFT systems around deterministic single-writer partitions per instrument, with strict price-time matching, fast inline risk checks, and asynchronous financial settlement. Hot path is optimized for microsecond-level internal latency and bounded tail behavior. Reliability comes from event logs, snapshots, replay, and idempotent processing."

## 2. Core Architecture (Memorize)

Hot path:

- market data gateway
- order book
- strategy
- risk gate
- matching/execution core
- venue gateway

Warm path:

- ledger writer (TigerBeetle)
- telemetry
- reconciliation
- operator control plane

## 3. Must-Say Design Principles

- single writer per partition
- strict ordering per partition
- deterministic event stream
- idempotent command handling
- fail closed on uncertain risk state
- no blocking DB/network in matching loop

## 4. Matching Algorithm (Short Form)

1. validate command
2. dedupe by idempotency key
3. cross while marketable against best opposite level FIFO
4. emit trade events
5. rest residual quantity if allowed
6. emit final order state

Invariants:

- no overfill
- FIFO at same price
- no invalid-price trade
- deterministic output for same input

## 5. Latency Talking Points

- measure `p50`, `p99`, `p99.9` (not average only)
- track per-stage latency budget
- watch queue delay and command age
- optimize for tail stability under bursts, not just peak throughput

## 6. Common HFT Failure Modes

- feed gap/out-of-order updates
- cancel/fill race
- queue overload and latency spikes
- duplicate delivery in distributed pipeline
- execution and ledger divergence

Mitigations:

- sequence checks + rebuild
- deterministic state machine
- bounded queues + overload policy
- idempotent consumers
- reconciliation jobs + alert thresholds

## 7. TigerBeetle One-Liner

"Execution is operational truth for immediate control; TigerBeetle is financial truth for double-entry correctness. We use stable transfer IDs, dedupe, and reconciliation so retries/restarts do not create financial drift."

## 8. Rust Knowledge Checklist

- ownership/borrowing for safe hot-state mutation
- enums/state machines for order lifecycle
- allocation control in hot loops
- concurrency model (single writer, bounded queues, atomics basics)
- benchmarking/profiling workflow (`cargo`, criterion, perf/flamegraph)

## 9. Interview Answer Structure (Use Every Time)

1. objective + SLO
2. invariants
3. dataflow and ownership boundaries
4. failure handling and recovery
5. tradeoffs and rejected alternatives
6. measurement evidence

## 10. Fast Self-Test (5 Questions)

- can I explain partitioning and ordering in 90 seconds?
- can I write price-time matching pseudocode quickly?
- can I explain idempotency and replay clearly?
- can I describe fail-closed risk behavior?
- can I defend ledger consistency under retries/outages?

Related:

- [[65 - HFT Rust System Design Master Note]]
- [[62 - Ultra-Low-Latency Matching Engine Design]]
