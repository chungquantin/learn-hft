---
title: Data Collection and Storage
tags: [data, storage, ingestion, research]
---

# Data Collection and Storage

Data collection is the act of deciding what future questions your system will be able to answer. That is a more useful way to think about it than simply calling it storage. Every retained stream expands the platform's future ability to replay events, explain decisions, test hypotheses, and investigate failures. Every missing stream narrows that ability.

A strong data layer collects both external and internal events. External market data shows what the world did: trades, quotes, depth changes, funding rates, open interest, and other venue-side state. Internal data shows how the engine interpreted that world: local timestamps, derived states, strategy decisions, risk outcomes, order lifecycle events, and telemetry. The distinction matters because postmortems are shallow when they can only answer what the market did and not what the system believed when it acted.

This is why internal events are not optional luxuries. They are the evidence required to explain latency spikes, missed opportunities, poor fills, incorrect risk gating, and strange decision behavior. In many real failures, the market data alone is not enough to reveal the problem. You also need to know what the engine knew, when it knew it, and what path that information followed through the system.

Good storage design also respects layers. Raw captured data should be preserved when possible because it is the closest thing to the original external record. Normalized canonical data should be versioned because the act of cleaning and reshaping data changes its meaning. Derived features and research artifacts should be reproducible because otherwise analysis becomes dependent on invisible transformations. If these layers blur together, later debugging turns into archaeology without strata.

Another important design habit is to treat timestamps and identifiers explicitly. One hidden danger in data systems is semantic flattening: different times get merged into one generic timestamp, different identifiers get overloaded, and data that once reflected a clear event lifecycle becomes hard to reason about. Storage then remains technically intact while intellectually degraded.

One practical way to avoid that degradation is to preserve distinct storage layers:

- raw venue capture when feasible
- normalized canonical events
- internal decision and lifecycle events
- derived research tables and features

Each layer answers different questions. Raw capture helps when you need to challenge your parser, timestamp model, or assumptions about what the venue actually sent. Canonical events support replay and cross-venue logic. Internal events explain what the engine believed and did. Derived layers support analytics and modeling. Trying to collapse all of these into one "market data table" usually makes every later investigation worse.

Time modeling matters here as much as schema design. Exchange time, local receive time, local publish time, order-submit time, acknowledgement time, and fill time should usually remain distinguishable in storage. Those timestamps are not redundant. They are the basis for reconstructing causality, market chronology, system latency, and execution quality. A serious research memory should keep these distinctions alive.

This is also where the book's infrastructure emphasis is useful: packet capture, hardware-adjacent timing, and network-path observation are not only operations concerns. They can become part of the data system too, especially when the platform needs to explain rare latency spikes or disputed exchange interactions. Not every learner needs this on day one, but the vault should remember that the full evidence chain can extend below application logs.

The cleanest summary of this note is that storage is not just for preservation. It is what makes trust, replay, attribution, and disciplined improvement possible.

Related:

- [[18 - Time and Timestamp Semantics]]
- [[42 - Research and Backtesting Systems]]
- [[44 - Data Quality and Lineage]]
- [[45 - Analytics and Post-Trade Review]]
