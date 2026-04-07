---
title: Roadmap
tags: [roadmap, learning]
---

# Roadmap

This vault is designed as a progression from intuition to systems thinking to implementation. The goal is not simply to collect facts about HFT, Rust, and perpetuals. The goal is to build understanding in the right order, so that later technical details attach themselves to a stable mental model instead of becoming isolated fragments.

The first stage is about building the basic shape of the domain. Before worrying about concurrency models, queue design, or exchange protocol quirks, it helps to understand what makes HFT different from slower forms of trading and what makes perpetual futures different from both spot markets and dated futures. This is where you should develop a feel for the market itself: how prices are formed at short horizons, why order flow matters, why leverage changes the problem, and why exchange mechanics are part of the market rather than background implementation detail.

That is why the first cluster to read is:

- [[01 - HFT Map]]
- [[03 - Perpetuals Trading]]
- [[04 - Market Microstructure]]
- [[99 - Glossary]]

At this stage, the most important questions are simple but deep. What exactly makes HFT different from discretionary trading? What data is the system actually reacting to? How do perpetuals create opportunity and danger in the same move? These are not beginner questions to rush through. They are the conceptual foundation that keeps later engineering work grounded.

The second stage is about learning the engineering constraints that turn those market ideas into systems problems. Once you understand the market side more clearly, the next question becomes: what kind of software can actually participate in that environment? This is where latency, ownership, local state, exchange architecture, synchronization, and risk control begin to matter in a more concrete way.

The key notes for that stage are:

- [[02 - Rust for HFT]]
- [[05 - Exchange Architecture]]
- [[14 - Low-Latency Systems Foundations]]
- [[15 - Benchmarking and Tick-to-Trade Measurement]]
- [[16 - Language Roles in an HFT Stack]]
- [[18 - Time and Timestamp Semantics]]
- [[19 - Matching Engines, Queue Priority, and Order Amend Semantics]]
- [[07 - Risk Management]]
- [[20 - Detailed Guides]]
- [[13 - System Design Map]]

At this point the right questions shift. Where does latency really come from? Which parts of the system need explicit memory and concurrency discipline? How does a platform remain safe when it is both fast and uncertain? This stage is where you stop thinking of HFT as "a strategy plus code" and start thinking of it as a machine that has to preserve meaning under pressure.

The third stage is about building. This matters because many concepts only become fully clear once you try to implement them. Order books seem intuitive until you have to reconstruct one correctly from snapshots and deltas. Strategy research sounds straightforward until realistic execution modeling kills half your ideas. Observability feels secondary until you need to explain why a live paper engine behaved strangely for twenty minutes.

The main notes and hubs for this building phase are:

- [[08 - Build Projects]]
- [[06 - Strategy Research]]
- [[30 - Backend Systems Hub]]
- [[40 - Data Systems Hub]]
- [[50 - Frontend and Operator Systems Hub]]

The project sequence is intentionally conservative: replay first, then collection, then realistic backtesting, then paper trading, then deeper research discipline. That order may feel slower than jumping directly into "alpha," but it is the order that teaches you which assumptions are actually holding the system together.

The final stage is not a separate destination so much as a continuing habit. Once the basic graph is in place, the work becomes one of deepening. That means adding formulas, exchange-specific notes, code experiments, benchmarks, postmortems, and design revisions. The vault should gradually stop being only a learning resource and start becoming your research memory.

If you use the roadmap well, it does one thing above all: it helps you ask the right next question rather than merely absorbing more disconnected material.

Related:

- [[01 - HFT Map]]
- [[20 - Detailed Guides]]
- [[08 - Build Projects]]
