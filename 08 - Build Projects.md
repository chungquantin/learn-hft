---
title: Build Projects
tags: [projects, roadmap, practice]
---

# Build Projects

Use projects to convert abstract concepts into durable intuition.

The real purpose of these projects is not to build a production trading firm immediately. It is to force the right questions:

- what data shape am I actually receiving?
- what invariants does this subsystem rely on?
- where does latency come from?
- where does state become ambiguous?
- what would make this system unsafe live?

## Project 1: Order book replay

Goal:
Build a Rust tool that replays historical level-2 updates and reconstructs a local book.

Learn:

- parsing
- sequencing
- snapshot + delta logic
- deterministic testing

Why this project matters:

This is the foundation for almost everything else. If you cannot reconstruct and trust local market state, strategy work stays speculative.

## Project 2: Perpetuals market data collector

Goal:
Collect trades, book updates, funding data, and open interest into a local dataset.

Learn:

- websocket ingestion
- schema normalization
- durable storage
- reconnect handling

Why this project matters:

It teaches that research quality depends on data engineering quality. It also forces you to confront exchange-specific quirks early.

## Project 3: Backtester with execution model

Goal:
Simulate fills with fees, slippage, and latency assumptions.

Learn:

- event-driven design
- realistic PnL accounting
- inventory tracking

Why this project matters:

This is where many optimistic ideas die, which is healthy. If an idea cannot survive realistic execution modeling, it is better to learn that in a backtester than in a live account.

## Project 4: Paper trading engine

Goal:
Run signals against live market data with simulated or exchange testnet execution.

Learn:

- live supervision
- operational safety
- observability

Why this project matters:

Paper trading reveals system weaknesses that backtests hide:

- stale state
- reconnect problems
- ambiguous execution state
- operator visibility gaps

## Project 5: Strategy lab notebook

Goal:
Turn this vault into a research log with:

- experiment notes
- hypotheses
- failures
- benchmarks
- code links

Why this project matters:

The notebook turns the vault from static notes into a research instrument. Over time, this becomes more valuable than a pile of disconnected code experiments.

## Suggested order of seriousness

You can think of the projects as maturity levels:

1. replay
2. collection
3. realistic backtest
4. paper trading
5. research discipline

Do not rush past the early projects. Most of the important engineering lessons show up before live deployment.

Related:

- [[00 - Roadmap]]
- [[02 - Rust for HFT]]
- [[06 - Strategy Research]]
