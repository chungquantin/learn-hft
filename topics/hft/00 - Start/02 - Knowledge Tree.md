---
title: Knowledge Tree
tags: [index, learning-path, knowledge-tree]
---

# Knowledge Tree

This page is the traversal map for the HFT topic. Use it when the flat index feels too noisy.

The best path is not strictly linear. The right mental model is a tree with a few trunks: market structure, quant research, system design, implementation, latency infrastructure, and operations.

For vault history and update notes, use [[CHANGELOG]].

## 1. Market intuition

Start here to understand what the machine is trying to trade.

- [[01 - HFT Map]]
- [[03 - Perpetuals Trading]]
- [[04 - Market Microstructure]]
- [[17 - Crypto Exchange Reality]]
- [[38 - Liquidity Programs and CLMM Incentives]]
- [[99 - Glossary]]

Core question:

What kind of market event is the system reacting to?

## 2. Quant and research

Read this branch when you want stronger signal, model, and validation discipline.

- [[06 - Strategy Research]]
- [[47 - Quant Topics for HFT Research]]
- [[42 - Research and Backtesting Systems]]
- [[43 - Feature Engineering and Labeling]]
- [[44 - Data Quality and Lineage]]
- [[45 - Analytics and Post-Trade Review]]
- [[46 - Order Flow and Event-Driven Trading]]
- [[48 - HFT to AI Infrastructure Technology Transfer]]
- [[29 - Arbitrage and Lead-Lag Deep Dive]]

Core question:

Can this idea survive realistic data, fees, latency, fills, and regime changes?

## 3. Trading-system architecture

Read this branch to understand how a live engine preserves state and meaning.

- [[05 - Exchange Architecture]]
- [[13 - System Design Map]]
- [[26 - Building a Low-Latency Trading Engine]]
- [[30 - Backend Systems Hub]]
- [[31 - Market Data Ingestion Deep Dive]]
- [[32 - Order Book Engine Deep Dive]]
- [[33 - Execution Management Deep Dive]]
- [[34 - Risk Engine Deep Dive]]
- [[35 - Service Boundaries and Process Topology]]
- [[36 - Reliability, Failure Modes, and Recovery]]

Core question:

Which subsystem owns each truth, and how does that truth move safely?

## 4. Matching engine implementation

Read this branch when building the Rust matching-engine project or preparing for system-design interviews.

- [[19 - Matching Engines, Queue Priority, and Order Amend Semantics]]
- [[57 - Matching Engine Component Plan]]
- [[65 - HFT Rust System Design Master Note]]
- [[63 - Rust Matching Engine Implementation Blueprint]]
- [[64 - Matching Engine Benchmark and Profiling Plan]]
- [[39 - Matching Engine Deployment, FPGA, DPDK, and Cloud Colocation]]
- [[65 - HFT Rust System Design Master Note]]
- [[66 - HFT Rust 30-Minute Interview Cheat Sheet]]
- [[67 - 100 HFT and Rust Interview Questions with Detailed Answers]]

Core question:

How do we produce deterministic fills, low tail latency, and recoverable state under burst load?

## 5. Low-latency infrastructure

Read this branch after the software ownership model is clear.

- [[14 - Low-Latency Systems Foundations]]
- [[15 - Benchmarking and Tick-to-Trade Measurement]]
- [[18 - Time and Timestamp Semantics]]
- [[23 - Seqlocks Deep Dive]]
- [[24 - Queues, Ring Buffers, and Backpressure]]
- [[25 - Logging and Telemetry Deep Dive]]
- [[37 - Kernel Bypass Technologies Deep Dive]]
- [[39 - Matching Engine Deployment, FPGA, DPDK, and Cloud Colocation]]
- [[48 - HFT to AI Infrastructure Technology Transfer]]
- [[68 - InfiniBand and RDMA Deep Dive]]
- [[69 - DPDK Deep Dive]]
- [[70 - Solarflare Onload and ef_vi Deep Dive]]
- [[71 - FPGA Market Data Pipeline Deep Dive]]
- [[72 - Production Low-Latency Trading System Construction]]
- [[73 - RDMA and RoCE Production Deep Dive]]
- [[74 - FPGA Feed Handlers and Inline Accelerators]]
- [[75 - Hardware Timestamping and PTP Deep Dive]]
- [[76 - HFT Technology Stack and Priority Map]]
- [[77 - LMAX Disruptor and Exchange Architecture]]
- [[78 - CPU NUMA Memory and Compiler Deep Dive]]
- [[79 - Performance Engineering and eBPF Deep Dive]]
- [[80 - SmartNIC DPU and Network Offload Deep Dive]]
- [[81 - Expert HFT Technology Curriculum]]

Core question:

Where is time actually being spent, and which layer can reduce it without damaging correctness?

## 6. Rust build path

Use this branch when turning the notes into code.

- [[02 - Rust for HFT]]
- [[21 - Rust for HFT Deep Dive]]
- [[08 - Build Projects]]
- [[55 - Rust HFT Interview Implementation Plan]]
- [[56 - Rust HFT 16-Week Implementation Path]]
- [[24 - Queues, Ring Buffers, and Backpressure]]
- [[23 - Seqlocks Deep Dive]]
- [[25 - Logging and Telemetry Deep Dive]]

Core question:

Which costs are explicit in the Rust design, and which ones are still hiding?

## 7. Operations and control

Read this branch when the engine needs to be operated safely rather than merely built.

- [[07 - Risk Management]]
- [[40 - Data Systems Hub]]
- [[50 - Frontend and Operator Systems Hub]]
- [[51 - Operator UI and Control Plane]]
- [[52 - Monitoring, Alerting, and Incident Response]]
- [[53 - Research Dashboards and Visualization]]
- [[54 - UX for Trading and Operations]]

Core question:

Can humans understand and intervene before ambiguity becomes damage?

## Practical traversal

If you are building now:

1. Read [[01 - HFT Map]], [[04 - Market Microstructure]], and [[17 - Crypto Exchange Reality]].
2. Read [[02 - Rust for HFT]], [[14 - Low-Latency Systems Foundations]], and [[15 - Benchmarking and Tick-to-Trade Measurement]].
3. Build through [[57 - Matching Engine Component Plan]], [[65 - HFT Rust System Design Master Note]], and [[63 - Rust Matching Engine Implementation Blueprint]].
4. Add deployment realism with [[37 - Kernel Bypass Technologies Deep Dive]] and [[39 - Matching Engine Deployment, FPGA, DPDK, and Cloud Colocation]].
5. Study the infrastructure layers with [[68 - InfiniBand and RDMA Deep Dive]], [[69 - DPDK Deep Dive]], [[70 - Solarflare Onload and ef_vi Deep Dive]], and [[71 - FPGA Market Data Pipeline Deep Dive]].
6. Go deeper into production concerns with [[73 - RDMA and RoCE Production Deep Dive]], [[74 - FPGA Feed Handlers and Inline Accelerators]], and [[75 - Hardware Timestamping and PTP Deep Dive]].
7. Use [[76 - HFT Technology Stack and Priority Map]] to choose the next technology based on the measured bottleneck.
8. Build the integrated production plan in [[72 - Production Low-Latency Trading System Construction]].
9. Study the event-core pattern with [[77 - LMAX Disruptor and Exchange Architecture]].
10. Study CPU topology with [[78 - CPU NUMA Memory and Compiler Deep Dive]].
11. Learn observability with [[79 - Performance Engineering and eBPF Deep Dive]].
12. Add SmartNIC/DPU and network offload knowledge with [[80 - SmartNIC DPU and Network Offload Deep Dive]].
13. Follow [[81 - Expert HFT Technology Curriculum]] as the complete progression.
14. Add research realism with [[47 - Quant Topics for HFT Research]], [[42 - Research and Backtesting Systems]], and [[45 - Analytics and Post-Trade Review]].
Related:

- [[00 - Roadmap]]
- [[13 - System Design Map]]
- [[20 - Detailed Guides]]
- [[CHANGELOG]]
