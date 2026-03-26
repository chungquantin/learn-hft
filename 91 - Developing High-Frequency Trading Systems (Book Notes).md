---
title: Developing High-Frequency Trading Systems (Book Notes)
tags: [sources, book-notes, hft, systems]
---

# Developing High-Frequency Trading Systems (Book Notes)

Source:
Sebastien Donadio, Sourav Ghosh, and Romain Rossier, *Developing High-Frequency Trading Systems*, Packt, 2022.

This note captures what was worth importing into the vault from the book without duplicating the whole text.

## Main value of the book

The book is strongest as a systems-oriented survey. It does not give a sharp modern edge in one narrow HFT niche, but it does connect trading-system architecture, exchange mechanics, hardware, networking, optimization, observability, and implementation tradeoffs into one continuous picture.

That makes it useful as a "full stack of low latency" source.

## Key ideas worth keeping

- HFT should be understood as a complete trading system, not merely a strategy.
- The critical path usually runs through gateways, book building, strategy, OMS/execution, and risk.
- Exchange behavior has to be learned as both protocol and system behavior, not just as API syntax.
- Hardware and OS details matter because scheduler behavior, memory hierarchy, NUMA, interrupts, and system calls all leak into latency.
- Networking is part of system design, not just transport plumbing; NIC choice, switch path, timestamping, and time distribution matter.
- Lock avoidance, pre-allocation, and cache-friendly data structures matter mainly because they reduce jitter and tail latency.
- Logging and live statistics are essential, but they must be engineered so they do not damage the hot path.
- Performance work should be measurement-led and focused on hot paths.
- Different languages play different roles: Python is valuable for research, while compiled languages usually own the latency-sensitive path.
- FPGA and physical-link optimization belong late in the optimization ladder, after architecture and measurement are already disciplined.

## What it changed in this vault

- It strengthened the vault's emphasis on system foundations as part of trading design, not as optional background reading.
- It reinforced the split between critical-path components and support-path components.
- It added more explicit attention to NIC locality, NUMA, context switching, kernel crossings, and time synchronization.
- It sharpened the idea that observability and performance measurement are first-class design concerns in HFT.

## Limits of the book

- It is broad and introductory relative to specialist production material.
- It is centered more on C++/Java framing than on Rust-native design.
- Some strategy sections are descriptive rather than deeply operational.
- The crypto material is useful as orientation, but venue-specific field notes are still more important than generic summaries.

## Best use

Use this book to reinforce architectural intuition and systems vocabulary.

Do not use it as a substitute for:

- exchange-specific protocol study
- replay-based validation
- empirical latency measurement
- modern venue observation notes

Related:

- [[14 - Low-Latency Systems Foundations]]
- [[26 - Building a Low-Latency Trading Engine]]
- [[27 - Exchange Protocols and Connectivity]]
- [[90 - Source Notes]]
