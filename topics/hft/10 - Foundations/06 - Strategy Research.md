---
title: Strategy Research
tags: [strategy, research, alpha]
---

# Strategy Research

The purpose of strategy research is not to discover patterns that look impressive in historical data. It is to identify repeatable edges that survive fees, slippage, latency, inventory risk, and all the practical friction imposed by the live system. That difference matters because many ideas look persuasive as statistical stories and become uninteresting the moment execution enters the picture.

A useful way to think about research is that it turns an intuition into a deployable claim. A claim strong enough to matter usually sounds something like this: under a defined set of conditions, a certain signal predicts an outcome strongly enough that the live system can exploit it after costs, constraints, and risk controls are applied. Anything weaker than that may still be a good hypothesis, but it is not yet a trading result.

This is why good beginner strategy buckets are valuable. Market making teaches fill quality, inventory control, and adverse selection. Short-horizon imbalance signals teach microstructure interpretation. Spread mean reversion teaches the difference between statistical signal and realistic execution. Cross-exchange basis trades teach synchronization and market-state comparison. Funding-aware carry teaches how contract mechanics change the economics of holding exposure. These strategies are useful not only because they may produce edge, but because each one teaches a different part of the market-system interface.

The research workflow is conceptually straightforward: define a hypothesis, determine the needed data, build features, simulate execution realistically, measure net outcomes after costs, and stress test across regimes. But in practice, the crucial stages are often the ones that feel least glamorous. Realistic execution modeling and regime stress testing are where many attractive ideas stop looking attractive. Fees, queue position, partial fills, stale signals, and market-state instability are not polish to add at the end. They are part of the hypothesis test itself.

The most common failure modes in research follow a pattern. The system ignores fees, ignores queue position, trains on cleaned data that would never exist live, optimizes for gross edge rather than deployable edge, or leaks future information indirectly through the label design. Other failures are subtler: fitting too tightly to one venue or one regime, measuring average outcomes while hiding dangerous tails, or forgetting that live risk limits change strategy behavior. These problems do not merely weaken the result. They change the question being answered.

Perpetual-specific research questions are often especially useful because they connect contract mechanics to tradable market state. Does funding predict near-term positioning pressure? Do liquidation cascades create transient overshoot? Does one venue reliably lead another for a given pair? Questions like these are powerful because they are not generic forecasting exercises. They are attempts to understand how the actual structure of the instrument creates temporary edge.

The best research culture is therefore skeptical but constructive. It wants ideas, but it wants them to survive reality quickly. It does not confuse a clean backtest with a valid strategy. It treats failure as information. And it uses replay, attribution, and good system instrumentation to narrow the gap between what looked true in analysis and what can survive in deployment.

Related:

- [[04 - Market Microstructure]]
- [[07 - Risk Management]]
- [[08 - Build Projects]]
- [[28 - Market Making Deep Dive]]
- [[29 - Arbitrage and Lead-Lag Deep Dive]]
- [[46 - Order Flow and Event-Driven Trading]]
