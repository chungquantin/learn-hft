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

Physical placement also belongs in topology design. In low-latency systems, a boundary is not only a code boundary or a deploy boundary. It may also imply a queue hop, a cache transition, a NUMA crossing, a kernel transition, or a network hop. This is why process diagrams should eventually be read alongside hardware diagrams. If the market-data process sits far from the NIC it depends on, or if execution and risk chatter constantly across sockets, the topology may be logically clean while still being physically expensive.

That does not mean every hot component must be fused together. It means boundary decisions should be explicit about their physical cost. A process or thread boundary is strongest when it buys something concrete such as clearer ownership, lower blast radius, or stronger operational control that justifies the extra movement of data and coordination.

A useful set of topology questions therefore becomes:

- should this boundary be a function boundary, a thread boundary, a process boundary, or a host boundary?
- does this component need independent failure handling or merely clean ownership?
- does this handoff preserve ordered events, publish snapshots, or carry control commands?
- what is the latency and contention cost of this boundary on real hardware?
- can this component live near the NIC, the hot caches, or the state it depends on?

One of the book's useful reminders is that internal networks matter too. A trading server talking to a risk service, telemetry collector, or internal market-data fanout is still paying network and switching costs even when the outside exchange path is healthy. That is why "internal" should never be treated as synonymous with "free." In larger systems, minimizing unnecessary internal hops is as much a topology question as it is a networking question.

The most helpful compact summary is this: process topology is not a style choice. It is the physical expression of ownership, latency discipline, and failure discipline. A good system boundary is one that makes the architecture easier to trust, not merely easier to draw.

Related:

- [[26 - Building a Low-Latency Trading Engine]]
- [[36 - Reliability, Failure Modes, and Recovery]]
- [[14 - Low-Latency Systems Foundations]]
- [[51 - Operator UI and Control Plane]]
