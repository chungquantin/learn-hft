---
title: HFT Interview Drills and Portfolio Packaging
tags: [interview, drills, portfolio, hft, rust]
---

# HFT Interview Drills and Portfolio Packaging

This note turns your implementation work into interview-ready artifacts and rehearsal loops.

## Portfolio Artifact Set

Prepare these five artifacts:

1. Architecture diagram: services, topics, state ownership
2. Matching + order book design note: invariants and edge cases
3. TigerBeetle ledger note: account model and idempotency strategy
4. Benchmark report: latency and throughput with reproducible setup
5. Incident postmortem pack: at least two simulated failures and recovery timeline

## 4-Round Mock Interview Loop

Round A: low-level Rust coding (45 min)

- implement core matching or order book delta logic
- discuss allocation strategy and borrow checker decisions

Round B: system design (60 min)

- design distributed exchange/matching/ledger architecture on whiteboard
- define consistency model and recovery behavior

Round C: debugging and operations (45 min)

- analyze synthetic incident logs
- identify root cause and propose mitigation

Round D: deep dive Q&A (30 min)

- defend tradeoffs, rejected alternatives, and scaling roadmap

## Question Bank

Core implementation:

- "Show how your cancel/replace avoids race-induced invalid state."
- "How do you preserve deterministic replay across restarts?"
- "Which data structure did you choose for the order book and why?"

Distributed systems:

- "What ordering guarantees do you need from your message bus?"
- "How do you handle risk service timeout during volatility spikes?"
- "How does sharding impact fairness and determinism?"

TigerBeetle/accounting:

- "Where do you place the accounting boundary and why?"
- "How do you reconcile partial fills and fees exactly?"
- "What policy applies when ledger lag grows but trading continues?"

## Scoring Rubric (Self-Eval)

Score each answer from `0` to `5`:

- correctness of invariants
- clarity of failure handling
- evidence from benchmarks/tests
- explicit tradeoff reasoning
- production realism

Target:

- average `>= 4.0` on three consecutive mock sessions

## Final Interview Week Checklist

- rehearse 10-minute architecture talk daily
- solve one Rust low-latency coding drill daily
- review one failure scenario and mitigation daily
- refresh TigerBeetle posting + reconciliation flows daily
- keep one-page cheat sheet of invariants and latency budget

Related:

- [[55 - Rust HFT Interview Implementation Plan]]
- [[56 - Rust HFT 16-Week Implementation Path]]
- [[15 - Benchmarking and Tick-to-Trade Measurement]]
