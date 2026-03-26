---
title: Service Boundaries and Process Topology
tags: [backend, processes, topology, architecture]
---

# Service Boundaries and Process Topology

Architecture discussions often go wrong because they begin with fashionable forms instead of responsibilities. People ask whether a system should be a monolith, a microservice mesh, a set of actors, or a collection of pinned threads. Those questions are not useless, but they are usually downstream of a more important one: who owns what, and what kind of isolation does that ownership require?

That ownership-first perspective matters because a trading platform contains several different kinds of pressure at once. Some components need latency isolation. Some need failure isolation. Some need independent deployability. Some simply need conceptual clarity so that the team can reason about them without confusion. If you draw boundaries only around technologies, the diagram may look modern while the system remains semantically tangled.

This is why good boundaries often follow ownership of state and responsibility. Market-data ingestion owns intake and normalization. The order-book engine owns local market representation. Strategy owns interpretation. Execution owns outbound order lifecycle. Risk owns permission. Telemetry and analytics own explanation rather than decision. Once you see those responsibilities clearly, the topology question becomes easier. You can ask whether any of those boundaries should live in separate threads, processes, or services based on actual requirements rather than taste.

Hot-path boundaries are usually the most sensitive because they combine speed and correctness pressure. Ingestion, book maintenance, strategy, execution, and risk frequently deserve stronger isolation or at least clearer ownership because confusion there directly affects live behavior. Soft-path components such as telemetry, persistence, analytics, alerting, and dashboards are different. They still matter, but they should usually be arranged so that they do not interfere materially with the trading loop. This is why many good designs privilege producer isolation and treat observability as a side channel rather than an in-band obligation.

There is also a tendency to over-split systems too early in the name of elegance. That can create deployment complexity, synchronization burden, and failure surfaces that are larger than the value they buy. The opposite mistake is to keep everything together merely because it is initially convenient. That can make ownership muddy and failure domains harder to reason about. The right topology therefore comes from asking a sequence of practical questions. Does this component need a different latency profile? Does it need a different failure blast radius? Is its ownership naturally separate? Will it evolve on a different deployment cadence? If the answer to several of those questions is yes, the case for a real boundary becomes much stronger.

The most helpful compact summary is this: process topology is not a style choice. It is the physical expression of ownership, latency discipline, and failure discipline. A good system boundary is one that makes the architecture easier to trust, not merely easier to draw.

Related:

- [[26 - Building a Low-Latency Trading Engine]]
- [[36 - Reliability, Failure Modes, and Recovery]]
- [[51 - Operator UI and Control Plane]]
