---
title: Perpetuals Deep Dive
tags: [perpetuals, derivatives, crypto, deep-dive]
---

# Perpetuals Deep Dive

Perpetual futures are easiest to misunderstand when they are described too casually. People often summarize them as futures contracts with no expiry, which is true but incomplete. From the point of view of system design and market behavior, the real significance of perpetuals is that they are a derivative market that must constantly maintain its relationship to spot without ever having the hard reset of settlement. That missing reset changes how price, funding, leverage, and liquidation interact.

The first conceptual correction a builder needs to make is to stop thinking of "the price" as a single thing. In perpetual markets, spot price, index price, mark price, and last traded price all play different roles. Spot describes the underlying market. Index price is usually an exchange-defined reference constructed from multiple spot venues. Mark price is the fair-value-like quantity often used for margin and liquidation decisions. Last traded price is simply the most recent execution on the perp venue. These prices can be close, but they are not interchangeable, and the differences matter exactly when the market is stressed enough for them to matter most.

That is why funding exists. Traditional dated futures do not need a perpetual anchoring mechanism because settlement forces convergence. Perpetuals, lacking expiry, require another mechanism to push the contract toward spot over time. Funding is that mechanism. If the perpetual trades rich to spot, longs are usually made to pay shorts. If it trades cheap, shorts are usually made to pay longs. The details vary by venue, and those details should never be treated as trivia. Timing, formula design, and price references all affect real trading behavior.

The most useful way to think about funding is not as a fee, but as market state information. Strong positive funding can suggest crowded long demand, persistent richness of the contract, and a possible opportunity for basis-aware positioning. But it can also coexist with genuine bullish structure, meaning that simply fading funding can be dangerous. Funding is therefore not a standalone signal. It is a pressure indicator inside a larger market regime.

Leverage is the next place where perpetuals become a system problem rather than merely a financial instrument. Because positions can be levered heavily, small errors become expensive quickly. Slippage hurts more. Mark-price movement matters more. Exchange outages become more consequential. Liquidation distance shrinks. This means that a perpetual trading engine cannot treat risk as a static wrapper around strategy. It has to continuously understand how much room remains before the venue itself begins to take control of the position.

Liquidation is part of the market structure for exactly that reason. In crypto perpetuals, forced position closure is not just a private account event. It can become visible market flow. Overleveraged positioning builds up, sharp moves start to trigger forced unwinds, those unwinds consume liquidity and move price further, and that movement can trigger more liquidations. For an HFT system, this means liquidation activity is often both a risk phenomenon and a microstructure phenomenon. It can change spread behavior, depth behavior, and the meaning of price movement itself.

Basis is equally central. The difference between perpetual price and spot price is not just a theoretical quantity for textbooks. It is a live expression of leverage demand, funding expectations, liquidity differences, and exchange-specific behavior. Once you start seeing perpetuals as their own market rather than as a derivative mirror of spot, basis becomes easier to understand. The market is not merely reflecting the underlying. It is expressing its own order-flow pressures.

This is why perpetuals are so attractive for HFT. They combine continuous trading, high event frequency, meaningful leverage, retail-driven order flow, rich cross-venue interactions, and very reactive order books. Those conditions create opportunities for spread capture, queue-position edge, order-flow prediction, cross-exchange arbitrage, basis trading, liquidation-aware strategies, and funding-aware inventory management. But the very same features also create operational danger. A system can appear to understand perpetuals while actually understanding only their calm-state behavior.

One of the most expensive beginner mistakes is to assume that exchange rules are implementation details. In perpetual markets, venue-specific funding formulas, liquidation engines, margin modes, ADL rules, reduce-only semantics, price bands, API sequencing, and websocket consistency all change what safe and effective trading looks like. Much of the pain in production perpetual trading comes not from abstract market theory but from the behavior of specific exchanges under stress.

This venue specificity is one reason protocol notes matter so much in perpetual trading. A venue may expose mark price on one channel, funding previews on another, position events on a private stream, and order acknowledgements through a different sequencing model altogether. If those streams are not reconciled carefully, the system can end up with a coherent local story that does not match the venue's actual state. In leveraged products, that mismatch is much more dangerous than in a lightly margined market.

Crypto venues also make internal versus external time especially important. Funding windows, mark updates, liquidation pressure, and bursty retail flow can make a few hundred milliseconds of stale interpretation economically meaningful. This does not mean every crypto system must chase nanoseconds. It means the system should know which timing assumptions its edge relies on and which staleness windows are no longer safe.

That is why a serious perpetuals system needs to know, continuously and explicitly, its current position, effective entry, collateral condition, liquidation distance, funding exposure, mark/index/last relationship, outstanding order state, and venue configuration. If any of those are ambiguous, the system is already too weak for leveraged live deployment.

The simplest useful conclusion is that perpetuals are powerful because they combine leverage, continuous trading, and liquid order books. They are dangerous for the exact same reasons. To trade them well, you have to understand contract mechanics as part of the market itself, not as exchange fine print.

Related:

- [[03 - Perpetuals Trading]]
- [[04 - Market Microstructure]]
- [[07 - Risk Management]]
- [[26 - Building a Low-Latency Trading Engine]]
