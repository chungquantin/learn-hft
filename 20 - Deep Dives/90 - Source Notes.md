---
title: Source Notes
tags: [sources, reading-notes, references]
---

# Source Notes

This page records external readings that shaped the vault.

## Rust for HFT

Source:
https://lucasbardella.com/coding/2025/rust-for-hft

Notes captured:

- Rust is attractive for HFT because it combines low-level performance with memory safety and no garbage collector.
- Memory operations and I/O should be minimized on the critical path.
- Multi-threading is common, but thread communication is not free.
- SPSC queues are a practical pattern for thread-to-thread handoff.
- Ring buffers are a common implementation strategy for those queues.
- CPU pinning can reduce jitter by preserving cache affinity.

## Understanding Perpetual Futures

Source:
https://www.investopedia.com/what-are-perpetual-futures-7494870

Article date:
Updated September 20, 2025

Notes captured:

- Perpetual futures have no expiry and are held indefinitely.
- Funding keeps perpetual prices near spot.
- Positive funding typically means longs pay shorts.
- Negative funding typically means shorts pay longs.
- Many exchanges apply funding every eight hours, but formulas vary by venue.
- Main use cases include leverage, hedging, and arbitrage.

## Rust DSA Ring Buffer

Source:
https://metame.substack.com/p/rust-dsa-ring-buffer

Status:
The page was not directly retrievable in the browser tool during this update.

Conservative note added:

- The vault was updated only with ring-buffer concepts consistent with the page title and with the accessible HFT source above.
- If you want, I can revisit this note later and expand it once we have the full article text.

Related:

- [[02 - Rust for HFT]]
- [[03 - Perpetuals Trading]]
- [[10 - Ring Buffers in Rust]]

## Inter Core Communication Pt 1: Seqlock

Source:
https://louisponet.github.io/blog/posts/icc-1-seqlock/

Notes captured:

- Seqlocks favor producers over consumers and let readers retry instead of taking locks.
- Memory barriers are necessary for correctness because compiler and CPU reordering can break the read/write sequence.
- The pattern is useful when readers want a consistent latest snapshot, not a full event history.
- Cache-line alignment and careful measurement matter for low-jitter behavior.

## Inter Core Communication Pt 2: Queues and SeqLock Vectors

Source:
https://louisponet.github.io/blog/posts/icc-2-queues-vectors/

Notes captured:

- Low-latency queues can be built from seqlocked slots arranged as a ring buffer.
- Producer isolation is a major design goal: consumers should not stall or materially affect producers.
- Broadcast-style consumers are easy to attach in this model, but slow consumers can be overtaken and lose messages.
- Shared-memory-backed vectors and queues are useful for modular multi-process or multi-core designs.

## Automatic Message Tracking and Timing

Source:
https://louisponet.github.io/blog/posts/message-tracking/

Notes captured:

- Low-contention queues make it practical to attach telemetry without materially impacting the main system.
- Wrapping messages with timing metadata enables propagation-latency, processing-time, and lineage tracking.
- `rdtscp`-style local timestamps are useful within one machine; cross-machine tracking needs stronger identifiers and timestamps.
- Central message spines and adapters can preserve clean business logic while automatically attaching timing data.

## Fast Logging for HFT In Rust

Source:
https://markrbest.github.io/fast-logging-in-rust/

Notes captured:

- Synchronous logging is too expensive on the hot path because of blocking I/O.
- Even asynchronous logging can remain expensive if the strategy thread still formats strings and allocates.
- A better design is to hand off lightweight work to another thread and keep the strategy thread focused on trading logic.

Related:

- [[11 - Seqlocks]]
- [[12 - Low-Latency Logging and Telemetry]]

## Developing High-Frequency Trading Systems

Source:
Sebastien Donadio, Sourav Ghosh, and Romain Rossier, *Developing High-Frequency Trading Systems*, Packt, 2022.

Book structure noted:

- HFT strategy overview and exchange basics
- trading-system architecture and OMS/gateway structure
- exchange matching and order-book dynamics
- hardware, OS, memory, and networking foundations
- optimization topics such as context switches, lock-free structures, pre-allocation, kernel bypass, logging, and measurement
- implementation perspectives in C++, Java, Python, FPGA, and crypto contexts

Notes captured:

- The book reinforces that HFT is a whole-system problem, not just a model or strategy problem.
- It provides a useful critical-path decomposition: gateways, book builder, strategy, order manager/execution, and risk.
- It emphasizes that NUMA layout, NIC locality, memory hierarchy, and OS scheduling all affect latency in ways application developers need to understand.
- It treats networking and time synchronization as first-class design concerns rather than invisible infrastructure.
- It frames logging and live statistics as necessary operational machinery that must be designed to avoid harming the hot path.
- It recommends measurement-led optimization and attention to tail latency, not just average speed.
- It treats Python as primarily a research and analytics tool, with lower-latency production work delegated to compiled components.
- It presents FPGA and specialized transport choices as advanced optimizations, not replacements for basic architectural discipline.

Related:

- [[14 - Low-Latency Systems Foundations]]
- [[26 - Building a Low-Latency Trading Engine]]
- [[27 - Exchange Protocols and Connectivity]]
- [[91 - Developing High-Frequency Trading Systems (Book Notes)]]
- [[92 - Developing High-Frequency Trading Systems (Full Research)]]
