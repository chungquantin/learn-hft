---
title: Rust for HFT
tags: [rust, systems, low-latency]
---

# Rust for HFT

Rust is useful in HFT because it offers predictable performance, strong memory safety, and good control over data layout and concurrency.

The important word here is predictable.

In HFT, the problem is often not that code is slow on average. The problem is that some operations are unpredictably expensive under stress. Rust helps because it forces more of those costs into the open.

## Why Rust fits

- Low-level performance without a garbage collector
- Safer concurrency than C++
- Strong type system for protocol and risk invariants
- Good ecosystem for async networking, parsing, and benchmarking

Rust is especially attractive when you care about:

- explicit ownership of state
- minimizing hidden allocations
- isolating unsafe code
- building systems that stay understandable as performance tuning increases

## What to learn first

- ownership and borrowing
- stack vs heap allocation
- `Vec`, slices, and memory layout
- enums for protocol modeling
- lock-free vs channel-based concurrency
- async I/O tradeoffs

What matters is not just knowing these features syntactically. You need to understand how they change design.

Examples:

- ownership changes how you split system boundaries
- enums make protocol and state-machine modeling cleaner
- slices and layout knowledge help avoid unnecessary copying
- concurrency primitives determine whether you are publishing snapshots or passing ordered events

## HFT-specific Rust topics

- zero-copy parsing
- cache-friendly structs
- minimizing allocations on hot paths
- SPSC queues and ring buffers
- seqlocks for read-mostly shared state
- pinning threads and CPU affinity
- telemetry without blocking
- benchmarking with realistic message rates

These topics matter because low-latency systems are mostly about disciplined handling of:

- memory
- synchronization
- I/O
- observability

If you keep those four under control, many higher-level design choices stay tractable.

## Design heuristics

- Prefer fewer communicating threads when the strategy path is latency-sensitive.
- Treat memory operations as expensive and avoid heap churn in hot paths.
- Keep disk and network I/O out of the critical decision loop whenever possible.

Add one more rule:

Prefer explicit costs over clever abstractions.

Abstractions are good when they make invariants clearer. They are bad when they hide allocations, synchronization, or ownership.

## Concurrency notes

- Single-threaded designs are simpler, but multi-threaded systems are common once you split market data, strategy, logging, and execution.
- Inter-thread communication has real cost, so the thread model should be intentionally small.
- SPSC queues are a practical default when one producer hands work to one consumer.
- Seqlocks are useful when a writer should never block readers and readers can tolerate retries.

The semantic distinction matters:

- queues preserve event flow
- seqlocks publish the latest coherent snapshot

If every event matters, use a queue.

If only the latest state matters, a shared-snapshot design may be better.

## Observability notes

- Logging should be moved off the strategy thread.
- Building formatted strings on the hot path is often too expensive.
- Attach telemetry in a way that preserves business-logic ergonomics and minimizes added contention.

One of the most common mistakes is treating observability as something to bolt on later. In HFT, observability should be designed early, because otherwise the system becomes impossible to explain once timing-sensitive bugs appear.

## CPU pinning

Pinning a hot thread to a core can reduce jitter by preserving cache locality and avoiding unnecessary scheduler movement. This matters more once you have already removed larger bottlenecks such as allocations and blocking I/O.

CPU pinning is not a magic first optimization. It matters after the architecture is already shaped sensibly. The wrong thread topology pinned perfectly is still the wrong topology.

## Suggested code modules

- `market_data`
- `order_book`
- `signals`
- `execution`
- `risk`
- `replay`

You can treat these modules as ownership boundaries:

- `market_data` owns decoding and normalization
- `order_book` owns local market-state reconstruction
- `signals` owns alpha logic
- `execution` owns order lifecycle
- `risk` owns permission to act
- `replay` owns reproducibility and learning

## Anti-patterns

- allocating on every tick
- hiding latency inside logging
- overusing async where a dedicated hot thread is simpler
- mixing strategy logic with exchange protocol details

Also avoid:

- premature unsafe code without a measured bottleneck
- shared mutable state when ownership transfer would be clearer
- benchmark conclusions drawn from unrealistic toy data

Related:

- [[05 - Exchange Architecture]]
- [[10 - Ring Buffers in Rust]]
- [[11 - Seqlocks]]
- [[12 - Low-Latency Logging and Telemetry]]
- [[08 - Build Projects]]
- [[90 - Source Notes]]
