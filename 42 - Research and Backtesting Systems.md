---
title: Research and Backtesting Systems
tags: [data, research, backtest, replay]
---

# Research and Backtesting Systems

The purpose of research infrastructure is not to produce polished charts or convincing stories. Its purpose is to reduce the distance between a hypothesis and the live conditions under which that hypothesis would actually have to survive. This is why backtesting is valuable only when it respects the structure of the real system.

Many weak research environments answer the wrong question. They seem to ask whether a strategy works, but what they are really asking is whether an idea would have worked inside a simplified universe where fees were small, fills were easy, latency was irrelevant, market states were always valid, and exchange behavior was kinder than reality. That is still a question, but it is not the question you eventually care about.

Replay is often more trustworthy than imaginative simulation because it forces the system to confront the actual shape of event flow. A replay engine that feeds recorded market events through the same logic used in production encourages good architecture. It rewards deterministic components, explicit state transitions, and shared event models between live and research contexts. It shrinks the cultural gap between "quant code" and "production code" because both begin to speak through the same event semantics.

Backtesting should therefore be thought of as a realism discipline. It should preserve fees, slippage, queue-position assumptions, latency constraints, and invalid-state behavior wherever possible. It should make it painful to cheat unintentionally. A backtest that looks modest but respects reality is far more valuable than one that looks brilliant while silently assuming impossible execution.

Strong research infrastructure also supports repeated comparison. It should let you rerun the same experiment deterministically, sweep parameters, slice regimes, extract features, and attribute performance. That last point is especially important. If a strategy makes money, you want to know whether the source was genuine signal, favorable regime luck, unrealistic execution assumptions, hidden risk concentration, or something else entirely. Without attribution, performance numbers remain numerically real but intellectually weak.

If there is a single theme to remember here, it is that research systems are not there to flatter hypotheses. They are there to expose them to enough reality that only robust ideas survive.

Related:

- [[31 - Market Data Ingestion Deep Dive]]
- [[32 - Order Book Engine Deep Dive]]
- [[43 - Feature Engineering and Labeling]]
