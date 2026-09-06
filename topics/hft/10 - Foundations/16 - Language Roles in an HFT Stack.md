---
title: Language Roles in an HFT Stack
tags: [languages, architecture, research, production]
---

# Language Roles in an HFT Stack

One of the easiest ways to become confused about trading-system design is to ask which single language should do everything. Real systems often become stronger when different layers optimize for different needs. The goal is not language purity. The goal is to keep research fast, production honest, and the seam between them semantically clean.

## Useful split

A practical architecture often looks like this:

- Python for research, analysis, feature work, notebooks, experiments, and orchestration
- a compiled language for ingestion, order-book maintenance, execution, and hot risk paths
- specialized hardware or ultra-low-level code only after the simpler bottlenecks are measured

The exact compiled language can vary. The deeper lesson is that research ergonomics and latency discipline are different optimization targets.

## Why Python still matters

Python remains useful because:

- the data ecosystem is strong
- iteration is fast
- modeling and analytics are convenient
- glue code and orchestration are easy to write

What Python is usually weaker at is the hottest live path, where interpreter overhead, garbage-collection behavior in dependencies, the GIL, and looser control over memory layout can all become part of the edge budget.

## Why compiled languages own the hot path

Compiled languages tend to fit better when you need:

- predictable latency
- explicit memory and allocation control
- stronger concurrency control
- tighter control over data layout
- lower-overhead handoff between hot components

The key phrase is predictable latency, not just raw speed.

## The seam matters more than the slogan

The risky part of a mixed-language stack is rarely that there are two languages. The risky part is when the research world and the production world stop meaning the same thing.

The seam should preserve:

- canonical event schemas
- timestamp semantics
- order and fill identifiers
- fee and funding assumptions
- execution-state definitions
- invalid-state behavior

If research and production disagree on those meanings, the stack becomes intellectually split even if the code compiles cleanly.

## Healthy patterns

- keep one canonical event model and reuse it across research and production
- make replay the bridge between languages whenever possible
- push expensive data processing and exploration into research layers
- keep the live trading path small and explicit
- treat FFI or bindings as a semantic bridge, not just a performance trick

## Unhealthy patterns

- prototyping with unrealistic assumptions and calling it portable to production
- letting research code invent event semantics that the live engine does not use
- using Python in the hot path because it is convenient, then compensating with panic optimizations
- overcomplicating the production stack just to mirror notebook ergonomics

## What to optimize for

Research should optimize for:

- iteration speed
- analytical reach
- hypothesis generation
- reproducibility

Production should optimize for:

- trustworthiness of state
- predictable timing
- clear ownership
- controlled failure behavior

Those are different jobs. Good architecture respects that.

Related:

- [[02 - Rust for HFT]]
- [[21 - Rust for HFT Deep Dive]]
- [[42 - Research and Backtesting Systems]]
- [[08 - Build Projects]]
- [[92 - Developing High-Frequency Trading Systems (Full Research)]]
