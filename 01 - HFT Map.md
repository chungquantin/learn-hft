---
title: HFT Map
tags: [hft, overview]
---

# HFT Map

High-frequency trading is the design of trading systems that react to market data and submit orders under tight latency, throughput, and risk constraints.

That sentence is accurate, but still too abstract. In practice, HFT is the intersection of:

- market structure
- systems engineering
- statistical reasoning
- operational discipline

If any one of those is weak, the whole system becomes fragile.

## Core pillars

- [[04 - Market Microstructure]]
- [[05 - Exchange Architecture]]
- [[06 - Strategy Research]]
- [[07 - Risk Management]]
- [[02 - Rust for HFT]]

## HFT in practice

Typical loop:

1. Receive market data
2. Update local state
3. Compute signal
4. Apply risk checks
5. Send, cancel, or modify orders
6. Track fills and inventory

That loop is the visible part of the system. Underneath it sit the hidden requirements:

- message ordering must be trusted
- local state must be coherent
- risk checks must be fast and decisive
- execution state must be reconciled continuously
- telemetry must explain what happened without distorting the hot path

The important point is that HFT is not "an algorithm plus an API key". It is a continuous control system with very little room for ambiguity.

## Important distinctions

- HFT is not just "fast trading"; it is systems engineering plus market structure awareness.
- Low latency matters only if the strategy edge depends on timing.
- Fast systems without tight risk controls fail quickly.

There are several bad simplifications to avoid:

- "low latency" is not the same as "good trading"
- "more data" is not the same as "better signal"
- "lock-free" is not the same as "correct"
- "profitable backtest" is not the same as "deployable strategy"

The right mental model is:

HFT is the attempt to preserve a small edge through a very unforgiving system.

That is why seemingly minor issues matter:

- one missed order-book update can corrupt state
- one hidden allocation can widen tail latency
- one wrong liquidation assumption can break leveraged trading
- one unclear operator dashboard can delay intervention during an incident

## Main strategy families

- Market making
- Cross-exchange arbitrage
- Basis trading
- Liquidation-aware trading
- Order flow prediction
- Short-horizon mean reversion

Each family depends on different bottlenecks.

Market making depends heavily on:

- queue position
- adverse selection
- cancel efficiency
- inventory control

Cross-exchange arbitrage depends heavily on:

- synchronized data views
- transfer and collateral constraints
- execution certainty
- venue-specific latency and fees

Liquidation-aware and order-flow strategies depend heavily on:

- microstructure interpretation
- event timing
- sudden liquidity changes
- correct classification of normal versus stressed regimes

## What beginners should actually learn

A good ramp-up sequence is:

1. Learn how order books work.
2. Learn how perpetuals differ from spot and dated futures.
3. Learn how a live trading engine is structured.
4. Learn how to replay data deterministically.
5. Learn how risk and execution state differ from signal logic.

If you skip directly to alpha ideas before understanding the system, you will misread almost every result.

For perpetual-focused work, start with [[03 - Perpetuals Trading]].

Related:

- [[13 - System Design Map]]
- [[20 - Detailed Guides]]
- [[22 - Perpetuals Deep Dive]]
