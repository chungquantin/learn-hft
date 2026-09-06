---
title: Matching Engine Benchmark and Profiling Plan
tags: [benchmark, profiling, latency, matching-engine, rust]
---

# Matching Engine Benchmark and Profiling Plan

This note defines how to measure and improve matching engine latency for HFT-style workloads.

## Measurement Targets

Measure:

- per-command latency (`p50`, `p99`, `p99.9`)
- commands/sec throughput
- queue depth under burst
- reject/defer rate under overload
- topology latency from client to gateway, gateway to sequencer, and sequencer to matching partition

Track separately by command type:

- new limit
- market
- cancel
- replace

## Benchmark Scenarios

1. Steady-state mixed flow
2. Cancel storm
3. Deep crossing sweep
4. Post-only reject heavy flow
5. Replay determinism validation
6. Gateway-to-matcher handoff with DPDK or socket-accelerated ingress
7. Same-host, same-AZ, shared placement group, peering, PrivateLink, and public path comparisons where applicable

## Harness Rules

- pin benchmark process to a core when possible
- warm up before capture
- use fixed RNG seeds for reproducibility
- export raw run metadata with commit hash and build mode

## Profiling Stack

Runtime profiling:

- `perf` / flamegraph for CPU hotspots
- allocator stats for unintended allocations
- cache miss and branch miss counters where available

Code-level checks:

- avoid string formatting in hot path
- avoid heap allocations in tight loops
- avoid hash map churn in matching loop

## Regression Gate

Fail CI/local gate if:

- `p99` regresses by more than configured threshold
- throughput drops below baseline threshold
- determinism test produces divergent event stream

## Interview Angle

Emphasize:

- you measure tail latency, not just average
- you benchmark by workload class, not one synthetic test
- you treat determinism as a first-class performance/correctness contract

Related:

- [[15 - Benchmarking and Tick-to-Trade Measurement]]
- [[65 - HFT Rust System Design Master Note]]
- [[63 - Rust Matching Engine Implementation Blueprint]]
- [[39 - Matching Engine Deployment, FPGA, DPDK, and Cloud Colocation]]
