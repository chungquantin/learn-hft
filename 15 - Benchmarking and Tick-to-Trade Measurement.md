---
title: Benchmarking and Tick-to-Trade Measurement
tags: [benchmarking, latency, measurement, performance]
---

# Benchmarking and Tick-to-Trade Measurement

Performance work in HFT goes wrong when people optimize what is easy to time rather than what actually determines trading behavior. That is why benchmarking should not be treated as a one-off cleanup step. It is a design discipline that tells you where the latency really lives, how much variance exists, and whether an optimization changed anything important.

The central measurement question is not simply "how fast is this code?" It is "how long does it take a market event to become a trading action, and where was that time spent?" That is the tick-to-trade question.

## Benchmarking ladder

Different benchmark types answer different questions:

1. Correctness and determinism checks
2. Microbenchmarks for parsers, queues, and data structures
3. Component benchmarks with realistic message mixes
4. Replay-driven system benchmarks
5. End-to-end tick-to-trade measurement on the real topology

The mistake is to treat one layer as a substitute for another. A queue can benchmark beautifully in isolation while the live system still performs badly because the real cost sits in scheduling, handoff, invalid-state recovery, or exchange interaction.

## Tick-to-trade decomposition

Useful timing layers often include:

- exchange event time
- local receive time
- post-parse or normalized-event time
- strategy decision time
- pre-trade risk completion time
- order-submit time
- venue acknowledgement time
- fill time

These timestamps are not redundant. They answer different questions:

- Did the venue publish late?
- Did our ingestion path add delay?
- Did strategy or risk compute too slowly?
- Was the order slow to leave?
- Was the venue slow to acknowledge?

If these layers are collapsed into one generic timestamp, diagnosis becomes guesswork.

## Metrics that matter

In this domain, averages are rarely enough. You usually care about:

- median latency
- tail latency such as p95, p99, and worse
- jitter or variance
- drop counts
- queue wait time
- stale-state windows
- latency under burst conditions

The reason is simple: a strategy can survive a moderate average cost more easily than rare but severe spikes that break assumptions at exactly the wrong moment.

## Benchmark conditions

A useful benchmark should declare:

- warm versus cold conditions
- data shape and event mix
- burst profile
- hardware and topology
- thread placement and affinity assumptions
- whether telemetry is enabled
- whether the benchmark measures only software or includes network path

Without that context, the number is easy to repeat and hard to trust.

## Common failure modes

- benchmarking toy inputs instead of realistic event flow
- optimizing average latency while ignoring tails
- ignoring allocator, cache, and scheduler effects
- assuming microbenchmarks predict system behavior
- changing many variables at once and calling it optimization
- forgetting that observability itself can change timings

## Practical workflow

1. Make the path measurable before trying to optimize it.
2. Confirm correctness and deterministic behavior first.
3. Identify the actual hot path and the actual worst tail.
4. Change one thing.
5. Measure again under the same conditions.
6. Keep only optimizations that survive realistic replay or end-to-end timing.

This workflow sounds conservative, but it creates speed faster than folklore does because it stops the team from polishing the wrong component.

## What good looks like

A strong HFT benchmark culture makes it easy to answer:

- what got faster
- what got more stable
- what still dominates latency
- what assumptions the measurement depends on

That is the standard worth aiming for. Benchmarking should reduce ambiguity, not produce prettier numbers.

Related:

- [[14 - Low-Latency Systems Foundations]]
- [[18 - Time and Timestamp Semantics]]
- [[21 - Rust for HFT Deep Dive]]
- [[25 - Logging and Telemetry Deep Dive]]
- [[42 - Research and Backtesting Systems]]
- [[92 - Developing High-Frequency Trading Systems (Full Research)]]
