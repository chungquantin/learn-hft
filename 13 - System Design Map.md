---
title: System Design Map
tags: [systems, architecture, map]
---

# System Design Map

Designing an HFT system is easier to understand once you stop thinking of it as a single machine. It is really a set of tightly connected loops that operate at different speeds and answer different questions. One loop trades. Another loop learns. A third loop keeps the first two safe and understandable. If you ignore any of these loops, the whole system becomes weaker than it first appears.

The trading loop is the one people usually imagine first. Market data arrives, the system updates local state, a strategy forms an opinion, risk checks decide whether action is allowed, and the execution layer translates intent into exchange-side order state. This loop is where latency, jitter, and correctness matter most visibly. It is the loop that directly determines whether the system can participate in a short-horizon opportunity.

The research loop is slower, but just as important. It collects raw market data and internal system events, normalizes them, stores them, replays them, and turns them into experiments. Without this loop, a trading system becomes a collection of hunches and anecdotes. With it, the system becomes something that can learn from itself. This is where ideas are pressure-tested, where false confidence is exposed, and where small edges are separated from attractive stories.

The operations loop sits around both of the others. It is the part of the system that helps humans observe, understand, and intervene. In a live trading environment, operators need to know whether the engine is healthy, whether market state is trustworthy, whether risk is growing, and whether action is needed. A backend can be technically sophisticated and still be dangerous if the operator surface turns degraded conditions into ambiguity.

That is why this knowledge graph is divided into three main branches:

- [[30 - Backend Systems Hub]]
- [[40 - Data Systems Hub]]
- [[50 - Frontend and Operator Systems Hub]]

These branches are not isolated silos. They are connected by cross-cutting concerns that shape the whole system:

- [[05 - Exchange Architecture]]
- [[07 - Risk Management]]
- [[12 - Low-Latency Logging and Telemetry]]
- [[26 - Building a Low-Latency Trading Engine]]

The backend branch teaches how the live engine maintains trustworthy state and acts safely under latency pressure. The data branch teaches how the system remembers, explains, and improves itself. The frontend and operator branch teaches how humans stay in control of a machine that can fail faster than they can think.

One useful way to read the graph is to ask a different question in each branch. In backend, ask: how do we preserve meaning under speed? In data, ask: how do we preserve truth under iteration? In frontend and operations, ask: how do we preserve clarity under stress?

If you keep those three questions in mind, the whole graph becomes easier to navigate. The point is not merely to accumulate notes about trading infrastructure. The point is to learn how to design a system where market structure, code structure, data structure, and human structure all reinforce one another instead of drifting apart.

Related:

- [[00 - Roadmap]]
- [[20 - Detailed Guides]]
