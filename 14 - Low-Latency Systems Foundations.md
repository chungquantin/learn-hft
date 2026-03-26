---
title: Low-Latency Systems Foundations
tags: [systems, low-latency, hardware, networking]
---

# Low-Latency Systems Foundations

Low latency is not a property of a single optimization. It is the result of many layers cooperating: hardware layout, operating-system behavior, network path, memory access, thread topology, protocol design, and measurement discipline.

That matters because many early HFT learners focus too quickly on strategy code or language choice. The harder truth is that a trading engine usually becomes slow, jittery, or unsafe because its system foundations are vague.

## Hardware first, but not hardware worship

Most HFT systems do not begin with exotic hardware. They usually begin with ordinary but carefully configured servers.

What matters early:

- CPU topology
- cache behavior
- NUMA boundaries
- NIC locality
- memory hierarchy

The key lesson is that physical layout leaks into software behavior. If your market-data thread runs far from the NIC it depends on, or if cross-socket traffic is constant, latency and jitter can grow even when the code looks clean.

## Operating system costs are real costs

The operating system is not just background machinery. It is part of the latency path.

Important sources of cost include:

- context switches
- system calls
- page faults
- interrupt placement
- thread migration

These costs matter because they often increase tail latency rather than only average latency. In trading systems, tail latency is usually what breaks assumptions.

A practical heuristic is:

- keep the hot path small
- keep ownership clear
- avoid unnecessary blocking
- pre-allocate where practical
- avoid designs that force constant kernel crossings

## Network path is part of the strategy path

A trading system does not merely "use the network." The network is the path through which market reality arrives and orders leave.

That means you should care about:

- exchange-side transport and protocol semantics
- internal network hops between components
- NIC behavior and packet path
- packet capture and timestamping
- clock synchronization and time distribution

The useful distinction is between external connectivity and internal distribution. External links connect you to the venue. Internal links connect your own market data, risk, execution, and monitoring components. Both need deliberate design.

## Memory and synchronization discipline

Many low-latency improvements are really about avoiding expensive or unpredictable memory behavior.

Common themes:

- pre-allocate hot-path structures
- minimize dynamic allocation during trading
- prefer cache-friendly layouts
- use queues or snapshots based on semantics, not fashion
- treat lock avoidance as a means, not a goal

The important question is not "can I make this lock-free?"

It is:

"What communication pattern preserves meaning with the least interference?"

## Measurement before mythology

Performance work without measurement turns into superstition quickly.

Good measurement in this domain means:

- focus on hot paths
- separate average latency from tail latency
- use low-overhead instrumentation
- measure regularly, not only after incidents
- validate whether an optimization changed the path that actually matters

The general 90/10 idea is useful here: a small portion of the code usually dominates runtime and latency exposure. Optimization effort should follow that reality.

## Optimization ladder

The sensible order is usually:

1. Make architecture and ownership clear.
2. Remove obvious blocking, excess copying, and allocation churn.
3. Improve thread placement, queues, and memory layout.
4. Measure again.
5. Only then consider advanced techniques such as kernel bypass, special transport choices, or FPGA acceleration.

This order matters because advanced optimization cannot rescue a confused architecture.

## Language role separation

A useful practical split is:

- Python for research, analytics, and orchestration
- compiled languages for latency-sensitive components

The exact compiled language can vary. The deeper principle is that research ergonomics and production latency often pull in different directions, so forcing one language to do everything is not always the strongest design.

## Crypto-specific reminder

Crypto does not remove these systems lessons. It usually intensifies them.

You still need:

- explicit protocol handling
- replayable event flow
- strong monitoring
- venue-specific risk awareness

What changes is that exchange heterogeneity, operational instability, and cloud-heavy deployments often become more common.

Related:

- [[05 - Exchange Architecture]]
- [[15 - Benchmarking and Tick-to-Trade Measurement]]
- [[12 - Low-Latency Logging and Telemetry]]
- [[26 - Building a Low-Latency Trading Engine]]
- [[27 - Exchange Protocols and Connectivity]]
- [[30 - Backend Systems Hub]]
- [[90 - Source Notes]]
