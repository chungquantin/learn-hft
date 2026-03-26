---
title: Perpetuals Trading
tags: [perpetuals, derivatives, crypto]
---

# Perpetuals Trading

Perpetual futures are one of the most important instruments to understand if your goal is to learn modern crypto market structure and build short-horizon trading systems. They are often introduced as futures contracts with no expiry, which is accurate but not yet useful enough. The more important insight is that perpetuals are not just leveraged spot. They are exchange-defined markets with their own pricing mechanics, margin logic, liquidation behavior, funding flows, and operational quirks. Once you understand that, the rest of the contract starts to make more sense.

Several concepts sit at the center of the instrument. Mark price, index price, funding rate, open interest, leverage, maintenance margin, and liquidation are not just glossary terms. They determine how the contract behaves under stress and what the trading engine must monitor if it wants to survive. One of the most important early distinctions is that last traded price is not the whole truth. Mark price often matters more for risk and liquidation. Index price often matters more for fairness and anchoring logic. A system that sees only trades and ignores these other reference prices is already missing part of the market.

Perpetuals are especially interesting for HFT because they combine deep liquidity, continuous trading, rich event flow around funding and liquidation, and heavy retail participation on many venues. Those features create short-horizon opportunity, but they also create more ways to misread what is happening. A move may reflect organic demand, forced liquidation flow, funding crowding, or venue-specific mechanical pressure. This is why perpetuals reward systems that can combine microstructure awareness with contract mechanics.

The most useful trading lenses in these markets tend to capture different layers of state. Spread and depth tell you about immediate liquidity. Queue position tells you about execution quality. Basis and funding tell you about relative-value and positioning pressure. Liquidation clusters tell you about where forced flow may emerge. Seeing all of these together is what turns price action into something interpretable rather than merely observable.

Funding is central because perpetuals need a way to stay near spot without settlement. When the contract trades rich to spot, funding is usually positive and longs pay shorts. When it trades cheap, funding is usually negative and shorts pay longs. But funding should not be understood as just a fee. It is part of the regime. Persistent positive funding can indicate crowded long demand, structural richness, or the possibility of carry trades. At the same time, it can coexist with a strong bullish trend, which is why funding on its own is not a complete strategy.

The reasons traders use perpetuals translate directly into engineering requirements. Leverage requires tighter risk control. Hedging requires better cross-market state tracking. Arbitrage requires tighter synchronization and execution certainty. Continuous exposure without expiry requires ongoing awareness of funding and liquidation risk rather than a simple buy-and-hold mentality.

The risks also have a particular shape. Exchange outages, auto-deleveraging, sudden liquidity gaps, mark/index divergence, and poor sizing under leverage can all damage the system quickly. What matters here is that many dangerous events in perpetual trading are exchange-mechanical rather than purely statistical. Forced deleveraging, stress-time API behavior, and venue-specific order handling are all part of the real market environment. So learning perpetuals means learning both the contract and the exchange that implements it.

This note is the overview. The deeper version is in [[22 - Perpetuals Deep Dive]], but the essential takeaway is already clear here: perpetuals are powerful because they combine leverage, continuous trading, and rich microstructure. They are dangerous for exactly the same reasons.

Practical subtopics:

- [[04 - Market Microstructure]]
- [[05 - Exchange Architecture]]
- [[07 - Risk Management]]
- [[06 - Strategy Research]]
- [[17 - Crypto Exchange Reality]]
- [[90 - Source Notes]]
