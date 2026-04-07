---
title: Feature Engineering and Labeling
tags: [data, features, labeling, modeling]
---

# Feature Engineering and Labeling

Feature engineering and labeling sit at the seam between raw market data and strategy logic. This is one of the easiest places in the research stack to create false confidence, because a pipeline can be statistically elegant while still encoding assumptions that could never survive live trading. For that reason, this layer deserves to be treated as a place where realism is either preserved or quietly lost.

Raw market data is often too granular, too noisy, or too mechanically shaped to use directly as a live decision input. Features exist to summarize recent state, compress microstructure into variables that matter for action, and expose relationships across venues, contracts, or horizons that the strategy can actually reason about. Spread statistics, imbalance measures, trade-flow summaries, funding regime indicators, local volatility proxies, and venue divergence features all fit into this general idea. They are attempts to transform raw event streams into structured views of market condition.

But feature engineering is only half the problem. Labels are where many research efforts begin to drift away from reality. A label may look mathematically clean while still smuggling in impossible assumptions. It might rely on market states that were not yet stable, assume fills that would not have been achievable, or ignore latency and execution constraints that would completely change the meaning of the prediction target. In short-horizon research, those mistakes are common because the time boundaries are so tight that any hidden simplification can materially change the result.

This is why a good label is not merely one that is predictive. It is one that corresponds to a plausible decision problem. If a model predicts a future quantity that the system could never exploit given its latency, fill mechanics, and risk constraints, then the label may be analytically interesting but strategically weak. The research stack should therefore keep asking not only whether the label is statistically sharp, but whether it maps onto something the live system could actually trade.

Another important principle is computational honesty. Features that a live strategy depends on should be cheap enough, stable enough, and timely enough to compute in the live environment. This does not mean every feature must be trivial. It means the research environment should not reward representations that collapse under real-time cost or implementation constraints. Otherwise the system learns to admire signals that production cannot use.

The simplest way to remember this note is that features and labels are not just mathematical objects. They are contracts between research and deployment. If that contract is weak, every impressive result built on top of it becomes harder to trust.

Related:

- [[04 - Market Microstructure]]
- [[06 - Strategy Research]]
- [[42 - Research and Backtesting Systems]]
