---
title: Backend Systems Hub
tags: [backend, systems, hub]
---

# Backend Systems Hub

The backend is the live nervous system of an HFT platform. It is the part of the stack that receives market events, builds local state, decides whether something should happen, and then sends instructions into an external exchange environment that does not care about your architecture, your abstractions, or your intentions. Because of that, backend design is fundamentally about preserving meaning while the system is under speed, concurrency, and failure pressure.

Many people first approach backend design as a collection of technical modules: market data, order book, strategy, risk, execution. That modular decomposition is useful, but it is not the deepest way to think about it. The more important perspective is that each module owns a different kind of truth. Market-data ingestion owns what the venue said. The order-book engine owns what the system believes the current market state is. The strategy owns how the system interprets that state. The risk engine owns whether action is allowed. The execution layer owns what has actually been attempted, acknowledged, rejected, or filled. If these truths are mixed together carelessly, the engine stops being explainable.

This branch focuses on the questions that determine whether a live trading engine is coherent:

- how market data should enter and be trusted
- how local state should be represented and invalidated
- how order intent differs from exchange truth
- how risk should constrain opportunity
- how boundaries between threads, processes, and services should be drawn
- how the system should fail when reality becomes disorderly

That is what the rest of this branch explores:

- [[31 - Market Data Ingestion Deep Dive]]
- [[32 - Order Book Engine Deep Dive]]
- [[33 - Execution Management Deep Dive]]
- [[34 - Risk Engine Deep Dive]]
- [[35 - Service Boundaries and Process Topology]]
- [[36 - Reliability, Failure Modes, and Recovery]]

These topics connect directly to the more general architecture and Rust notes:

- [[02 - Rust for HFT]]
- [[05 - Exchange Architecture]]
- [[21 - Rust for HFT Deep Dive]]
- [[26 - Building a Low-Latency Trading Engine]]
- [[27 - Exchange Protocols and Connectivity]]

One good way to read this branch is to follow the lifecycle of a single market event. A message arrives from the exchange. The ingestion layer must decide whether it is valid. The order-book engine must decide whether it changes trusted state. The strategy must decide whether that change matters. The risk engine must decide whether any resulting action is allowed. The execution layer must decide how to express intent to the venue and how to reconcile the response. Reliability logic must decide what to do if any step becomes ambiguous. Seen this way, backend design is not a pile of services. It is a chain of decisions about trust.

Related:

- [[13 - System Design Map]]
- [[40 - Data Systems Hub]]
- [[50 - Frontend and Operator Systems Hub]]
