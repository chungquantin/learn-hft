---
title: Crypto Exchange Reality
tags: [crypto, exchanges, market-structure, perpetuals]
---

# Crypto Exchange Reality

Crypto trading is easiest to misread when it is treated as a faster version of traditional market structure. In practice, much of the challenge is exchange reality: 24/7 operation, uneven liquidity, venue-specific mechanics, inconsistent historical data quality, and infrastructure choices that are often more cloud-shaped than classical colocation-shaped.

That means many crypto HFT problems are not primarily about abstract strategy. They are about understanding the actual exchanges well enough that your system knows what kind of reality it is trading inside.

## Centralized exchanges first

For most practical HFT-style crypto work, centralized exchanges are still the main venue type to understand.

Why:

- order books are explicit
- API and websocket behavior directly shape the trading system
- maker/taker economics matter
- perpetual mechanics are venue-defined

That is where the majority of exchange-mechanics learning pays off.

## DEXs are not a small variation

Decentralized exchanges are not just centralized exchanges with different endpoints. Their latency, settlement, routing, transaction ordering, and execution guarantees are different enough that they should usually be treated as a different system class.

This matters because a strategy and architecture that make sense for a centralized limit-order-book venue may be poorly matched to an on-chain execution environment.

## What makes crypto venue reality hard

- liquidity is uneven across venues and regimes
- fee schedules and market-maker programs can shape competition materially
- historical data may be incomplete, interpolated, or operationally messy
- mark price, index price, and liquidation rules are exchange-defined
- public and private stream quality varies under stress
- testnets often fail to represent production behavior

These are not side notes. They are part of the market.

## Data quality matters more than people expect

In crypto, first-party capture is often much more important than beginners expect. Historical datasets may be missing fields, normalized loosely, or reconstructed from imperfect sources. If your research depends on queue behavior, mark/index relationships, funding state, or microstructural timing, directly captured venue data is usually safer than third-party summaries.

## Cloud reality

Crypto venues are often more cloud-adjacent than traditional colocated exchanges, but that does not make cloud deployment automatically good.

Cloud can be acceptable when:

- the venue itself is cloud-adjacent
- the strategy tolerates the latency budget
- region placement is carefully chosen

Cloud can still be dangerous when:

- virtualization noise dominates the edge
- region mismatch adds avoidable delay
- internal topology becomes more complex than the strategy justifies

The practical question is not "cloud or no cloud?" It is "what latency and reliability budget does this strategy actually require?"

## What to remember

The most useful compact summary is:

Crypto HFT is mostly exchange-mechanics work before it becomes alpha work.

If the system does not understand venue rules, data quality, funding mechanics, liquidity asymmetry, and deployment reality, it is likely to misread both research and production behavior.

Related:

- [[03 - Perpetuals Trading]]
- [[22 - Perpetuals Deep Dive]]
- [[27 - Exchange Protocols and Connectivity]]
- [[41 - Data Collection and Storage]]
- [[92 - Developing High-Frequency Trading Systems (Full Research)]]
