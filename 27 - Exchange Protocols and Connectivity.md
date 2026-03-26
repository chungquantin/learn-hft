---
title: Exchange Protocols and Connectivity
tags: [exchange, protocols, connectivity, deep-dive]
---

# Exchange Protocols and Connectivity

Exchange connectivity is where abstract system design becomes concrete. Up to this point it is easy to talk about ingestion, execution, and reconciliation as if they were generic building blocks. Real exchanges make those abstractions uneven. They expose different websocket semantics, different order APIs, different sequencing guarantees, different failure modes, and different operational limits. That is why protocol work should not be treated as a thin adapter layer. It is part of the core system design.

The first useful mental model is that an exchange protocol is not only a transport format. It is a behavioral contract. A websocket feed tells you not just what fields appear in a message, but how continuity works, what a reconnect means, what counts as a snapshot, whether checksums exist, how sequence numbers behave, and what forms of ambiguity you must expect. Similarly, an order API tells you not just how to submit an order, but how the venue thinks about acknowledgement, rejection, idempotency, cancellation, rate limiting, and state recovery after disconnection.

This matters because many trading-system bugs begin as protocol misunderstandings. A developer thinks they have a clean order-book feed when they actually have a stream that requires explicit gap recovery. They think an order is safely accepted when they only know it was transmitted. They treat a client order ID as globally meaningful when it is only session-local. They assume testnet behavior is representative when the production exchange behaves very differently under stress. In each case, the software may appear correct while its assumptions are wrong.

Connectivity design therefore has to respect two truths at once. First, market data and order state are external facts that arrive on exchange-defined terms. Second, the internal system needs a much cleaner and more stable representation than any one venue will provide directly. That tension is why robust protocol layers normalize aggressively while preserving enough raw context to debug. They turn venue-shaped events into internal event types, but they do not discard the details needed to reconstruct what really happened when something becomes ambiguous.

Another useful perspective is that protocol work is part latency engineering and part failure engineering. On the latency side, you care about parsing cost, message size, batching behavior, transport jitter, and how quickly the engine can convert raw messages into trusted internal events. On the failure side, you care about reconnect semantics, stale subscriptions, partial acknowledgements, duplicate deliveries, order-status drift, and what the venue does when under stress. A system that optimizes only for fast-path parsing but does not understand disconnect and resync behavior is not actually strong.

This is especially important in perpetual venues because exchange mechanics are often tightly coupled to risk. Funding intervals, mark-price semantics, reduce-only rules, liquidation engines, and price-band constraints all influence what the protocol layer should expose upstream. If the execution and risk layers are unaware of these exchange-specific behaviors, they may appear consistent internally while still disagreeing with the venue in exactly the moments when the disagreement becomes expensive.

One of the best habits here is to treat protocol documentation as necessary but insufficient. Read it carefully, but then validate behavior empirically through replay, sandbox testing, and production-observation notes. Exchanges do not always behave as cleanly as their documentation suggests, especially during load or volatility. Over time, your system should accumulate not only protocol code, but protocol knowledge.

The simplest summary is that exchange connectivity is not just an integration problem. It is the part of the trading system where external reality first touches internal truth.

Related:

- [[05 - Exchange Architecture]]
- [[31 - Market Data Ingestion Deep Dive]]
- [[33 - Execution Management Deep Dive]]
- [[22 - Perpetuals Deep Dive]]
