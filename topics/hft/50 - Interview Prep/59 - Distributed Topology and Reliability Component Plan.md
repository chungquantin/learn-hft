---
title: Distributed Topology and Reliability Component Plan
tags: [distributed-systems, reliability, topology, hft, rust]
---

# Distributed Topology and Reliability Component Plan

This note defines a practical distributed architecture for scaling while protecting latency-sensitive paths.

## Service Topology

Hot path services:

- `md-gateway`: feed handlers and normalization
- `book-engine`: local market state per instrument partition
- `matching-core`: order matching and execution events
- `risk-core`: synchronous permission checks and kill switch
- `exec-gateway`: venue adapters and order lifecycle reconciliation

Warm path services:

- `ledger-writer`: posts balanced entries to TigerBeetle
- `telemetry-pipeline`: metrics, traces, structured events
- `reconciler`: compares execution truth vs ledger/account truth
- `ops-api`: operator controls and state introspection

## Partitioning Strategy

Primary partition key:

- `instrument_id` or `(venue, instrument_id)`

Rules:

- single writer per partition for deterministic ordering
- no cross-partition synchronous calls in hot path
- cross-partition aggregation happens in warm path

## Messaging and Contracts

Event backbone requirements:

- ordered delivery per partition
- at-least-once delivery with idempotent consumers
- bounded queues and backpressure metrics
- explicit schema versioning

Core topics:

- `market_events`
- `order_commands`
- `execution_events`
- `risk_events`
- `ledger_events`
- `ops_events`

## Failure Model and Recovery

Failure scenarios to simulate:

- feed disconnect and replay gap
- matching-core process restart
- risk-core timeout
- exec-gateway reject storm
- ledger writer outage

Recovery principles:

- fail closed for uncertain risk state
- fail open only for non-critical telemetry paths
- isolate blast radius by partition and service boundary
- replay to rebuild state deterministically

## Reliability SLO Set

Example SLOs:

- no unbounded queue growth in hot path
- `> 99.95%` successful risk checks under nominal load
- recovery to trusted state within `RTO <= 60s` after single service crash
- zero unreconciled ledger delta after replay completion

## Interview Questions To Drill

- "Why this boundary split instead of one process?"
- "How do you prevent duplicate side effects in distributed retries?"
- "How do you keep deterministic behavior when scaling horizontally?"

Related:

- [[35 - Service Boundaries and Process Topology]]
- [[36 - Reliability, Failure Modes, and Recovery]]
- [[52 - Monitoring, Alerting, and Incident Response]]
