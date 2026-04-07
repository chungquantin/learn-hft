---
title: Detailed Guides
tags: [guides, deep-dive, index]
---

# Detailed Guides

This section is meant to be read more like a sequence of short chapters than a collection of lookup notes. The higher-level map pages in the vault tell you what the territory looks like. These guides are where the territory starts to feel solid. They are the notes you read slowly, annotate, challenge, and return to after building something.

The purpose of this layer is not only to tell you what concepts exist. It is to explain why they matter, where they break, and how they connect to system design. If the map pages give you navigation, these guides should give you mental models.

The core guides are:

- [[21 - Rust for HFT Deep Dive]]
- [[22 - Perpetuals Deep Dive]]
- [[23 - Seqlocks Deep Dive]]
- [[24 - Queues, Ring Buffers, and Backpressure]]
- [[25 - Logging and Telemetry Deep Dive]]
- [[26 - Building a Low-Latency Trading Engine]]
- [[27 - Exchange Protocols and Connectivity]]
- [[28 - Market Making Deep Dive]]
- [[29 - Arbitrage and Lead-Lag Deep Dive]]
- [[46 - Order Flow and Event-Driven Trading]]

One good way to use this section is to read a guide, then immediately turn part of it into something concrete. That might be a benchmark, a small Rust experiment, a data collector, a replay tool, or simply a written attempt to explain the idea in your own words. The point is to make the concept resist passive agreement. If an idea matters for HFT, it should eventually survive contact with code, data, or system behavior.

Another good habit is to keep track of what still feels slippery. In topics like queues, seqlocks, risk, and microstructure, the first danger is not ignorance but premature confidence. It is easy to feel like you understand something because the definition is clear. It is much harder to know whether you would design correctly under pressure. These guides are meant to narrow that gap.

Related:

- [[00 - Roadmap]]
- [[08 - Build Projects]]
- [[90 - Source Notes]]
