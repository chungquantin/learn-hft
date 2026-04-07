---
title: Study Questions
tags: [questions, learning]
---

# Study Questions

This page is less a checklist than a set of prompts for deepening the vault over time. The goal of these questions is not to produce one correct answer quickly. It is to push your understanding past surface familiarity and into the kind of reasoning that informs design choices, benchmarks, and research experiments.

On the HFT side, the important questions are often about distinguishing genuine edge from technical theater. When does latency matter enough to create real economic value, and when does it merely create a feeling of sophistication? Which strategies degrade first when message delay increases? How does queue position alter expected value in ways that ordinary price-series thinking misses? These questions are useful because they force you to connect system behavior to market behavior rather than studying them separately.

On the perpetuals side, the key questions usually revolve around contract mechanics and venue specificity. How exactly is funding computed on the exchanges you care about? What triggers liquidation, and which parts of that logic are exchange-specific rather than universal? Which symbols or venues show repeatable lead-lag structure, and under what market regimes? These questions matter because perpetual trading becomes dangerous when traders assume the instrument is simpler than it is.

On the Rust and systems side, the most useful prompts are about cost and architecture. Which hot paths truly need to be allocation-free? Where is a dedicated thread more appropriate than an async task? Which metrics would reveal a performance regression before you felt it in PnL? These questions matter because system quality is often determined by the costs you fail to notice until they become part of live behavior.

On the research side, the deepest questions are usually about honesty. Which assumptions in the backtest are least realistic? Which regime changes would most quickly break the strategy? What would falsify the edge faster than you currently expect? These are the kinds of questions that keep research from becoming narrative reinforcement.

The best way to use this page is to revisit it after each project, each new reading, or each unexpected result. A good question asked repeatedly at a higher level of understanding becomes more valuable than a large pile of answers collected once.

Related:

- [[00 - Roadmap]]
- [[06 - Strategy Research]]
