---
title: Research and Backtesting Systems
tags: [data, research, backtest, replay]
---

# Research and Backtesting Systems

The purpose of research infrastructure is not to produce polished charts or convincing stories. Its purpose is to reduce the distance between a hypothesis and the live conditions under which that hypothesis would actually have to survive. This is why backtesting is valuable only when it respects the structure of the real system.

Many weak research environments answer the wrong question. They seem to ask whether a strategy works, but what they are really asking is whether an idea would have worked inside a simplified universe where fees were small, fills were easy, latency was irrelevant, market states were always valid, and exchange behavior was kinder than reality. That is still a question, but it is not the question you eventually care about.

Replay is often more trustworthy than imaginative simulation because it forces the system to confront the actual shape of event flow. A replay engine that feeds recorded market events through the same logic used in production encourages good architecture. It rewards deterministic components, explicit state transitions, and shared event models between live and research contexts. It shrinks the cultural gap between "quant code" and "production code" because both begin to speak through the same event semantics.

Backtesting should therefore be thought of as a realism discipline. It should preserve fees, slippage, queue-position assumptions, latency constraints, and invalid-state behavior wherever possible. It should make it painful to cheat unintentionally. A backtest that looks modest but respects reality is far more valuable than one that looks brilliant while silently assuming impossible execution.

This is also where language roles become easier to reason about. Research systems often benefit from Python because iteration speed, data tooling, and analytical ergonomics matter. Production systems often benefit from a compiled language because timing behavior, memory layout, and concurrency control matter. The mistake is not using multiple languages. The mistake is allowing the seam between them to become semantically inconsistent. The safest pattern is to keep event models, timestamps, identifiers, and execution assumptions aligned across both worlds even if the implementation languages differ.

Benchmarking belongs in research infrastructure too. It is not only a production concern. If your replay engine cannot tell you how latency assumptions, queueing assumptions, or execution delays change outcomes, then your research environment is still pretending that system cost is irrelevant. A mature research stack therefore measures not only PnL but also sensitivity to timing, stale-state windows, and execution friction.

Strong research infrastructure also supports repeated comparison. It should let you rerun the same experiment deterministically, sweep parameters, slice regimes, extract features, and attribute performance. That last point is especially important. If a strategy makes money, you want to know whether the source was genuine signal, favorable regime luck, unrealistic execution assumptions, hidden risk concentration, or something else entirely. Without attribution, performance numbers remain numerically real but intellectually weak.

One helpful way to structure research validation is to ask progressively harsher questions:

1. Did the idea survive with clean replay and correct accounting?
2. Did it survive realistic fees and spread costs?
3. Did it survive latency and queue-position assumptions?
4. Did it survive bad states such as feed gaps, stale marks, or delayed acknowledgements?
5. Did it still make sense after attribution by regime, venue condition, and execution quality?

That sequence is useful because it turns the backtest from a persuasion tool into a filtering tool.

If there is a single theme to remember here, it is that research systems are not there to flatter hypotheses. They are there to expose them to enough reality that only robust ideas survive.

Related:

- [[18 - Time and Timestamp Semantics]]
- [[31 - Market Data Ingestion Deep Dive]]
- [[32 - Order Book Engine Deep Dive]]
- [[41 - Data Collection and Storage]]
- [[43 - Feature Engineering and Labeling]]
