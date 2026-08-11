---
title: Expert HFT Technology Curriculum
tags: [hft, curriculum, expert, systems, networking, trading-technology]
---

# Expert HFT Technology Curriculum

## What expertise means

Expertise is not knowing the names of every low-latency product. It means being able to explain and measure the full path, select the simplest technology that meets the requirement, and recover safely when the fast path fails.

You should eventually be able to answer:

- Where was the packet at each timestamp?
- Which core and NUMA node owned it?
- Which queue or buffer introduced delay?
- What event sequence defines truth?
- What happens when a packet, clock, NIC, FPGA, or primary process fails?
- Can the system replay the decision exactly?
- Which optimization improves p99.9 rather than only the median?

## Level 1: market and protocol foundations

Study:

- market microstructure and queue priority;
- exchange order types and cancel/replace semantics;
- FIX and binary protocols;
- UDP multicast, sequence gaps, recovery, and TCP sessions;
- fees, rebates, adverse selection, and execution quality.

Build:

- a deterministic feed decoder;
- a replayable order book;
- a session state machine with rejects and reconnects.

Use [[04 - Market Microstructure]], [[27 - Exchange Protocols and Connectivity]], [[31 - Market Data Ingestion Deep Dive]], and [[19 - Matching Engines, Queue Priority, and Order Amend Semantics]].

## Level 2: deterministic event core

Study:

- single-writer partitioning;
- LMAX Disruptor and dependency graphs;
- bounded rings and backpressure;
- seqlocks, atomics, memory ordering, and ownership;
- event sourcing, journals, snapshots, and replay.

Build:

- a Rust single-writer matching engine;
- preallocated command and event rings;
- journal/replay equivalence tests;
- warm standby state verification.

Use [[77 - LMAX Disruptor and Exchange Architecture]], [[24 - Queues, Ring Buffers, and Backpressure]], [[23 - Seqlocks Deep Dive]], and [[36 - Reliability, Failure Modes, and Recovery]].

## Level 3: host performance

Study:

- CPU cache and coherence;
- NUMA and PCIe locality;
- hugepages and TLB behavior;
- branch prediction, SIMD, compiler output;
- CPU pinning, SMT, C-states, and allocator behavior.

Build:

- local-versus-remote NUMA benchmarks;
- cache-line false-sharing benchmark;
- assembly-reviewed order-book hot loops;
- allocation and page-fault regression tests.

Use [[78 - CPU NUMA Memory and Compiler Deep Dive]] and [[14 - Low-Latency Systems Foundations]].

## Level 4: packet and transport engineering

Study:

- NIC descriptors, RSS, flow steering, and offloads;
- Onload, ef_vi, DPDK, AF_XDP;
- InfiniBand verbs and RDMA memory registration;
- RoCE PFC, ECN, QoS, and congestion;
- multicast fanout and switch buffering.

Build:

- a DPDK or ef_vi feed handler;
- an Onload order gateway;
- a shared-memory versus RDMA benchmark;
- burst-loss and recovery tests.

Use [[69 - DPDK Deep Dive]], [[70 - Solarflare Onload and ef_vi Deep Dive]], [[73 - RDMA and RoCE Production Deep Dive]], and [[80 - SmartNIC DPU and Network Offload Deep Dive]].

## Level 5: hardware acceleration and time

Study:

- FPGA feed pipelines, RTL, HLS, and timing closure;
- SmartNIC/DPU/P4 boundaries;
- hardware RX/TX timestamps;
- PHC, PTP, `ptp4l`, `phc2sys`, and clock failure;
- FPGA shadow mode and bitstream release discipline.

Build:

- an FPGA or software reference feed decoder;
- a PTP-aware wire-to-wire latency harness;
- a hardware shadow pipeline with event equivalence checks.

Use [[74 - FPGA Feed Handlers and Inline Accelerators]], [[75 - Hardware Timestamping and PTP Deep Dive]], and [[80 - SmartNIC DPU and Network Offload Deep Dive]].

## Level 6: measurement and production

Study:

- `perf`, eBPF, tracepoints, flame graphs, and PMU counters;
- packet capture and sequence diagnostics;
- p50/p99/p99.9 and coordinated omission;
- risk controls, kill switches, fencing, and failover;
- colocation, cross-connects, switch configuration, and runbooks.

Build:

- a complete tick-to-trade histogram pipeline;
- an eBPF scheduler/network diagnostic;
- failure drills for feed gaps, clock loss, NIC reset, and primary failover;
- a reproducible host-image and hardware configuration manifest.

Use [[79 - Performance Engineering and eBPF Deep Dive]], [[15 - Benchmarking and Tick-to-Trade Measurement]], [[72 - Production Low-Latency Trading System Construction]], and [[25 - Logging and Telemetry Deep Dive]].

## Expert capstone

Build one system in stages:

```text
ordinary sockets
  -> tuned sockets and timestamps
  -> Onload gateway
  -> DPDK/ef_vi feed handler
  -> FPGA shadow decoder
  -> production-style single-writer event core
  -> PTP and perf/eBPF observability
  -> replay, standby, and failure drills
```

At every stage, keep the same normalized event schema and test corpus. The goal is to compare technologies without changing the business semantics.

## Technologies to add later

- kernel scheduling and IRQ/softirq internals;
- switch ASICs, P4, and programmable telemetry;
- NVMe, io_uring, and sequential journal design;
- Aeron, SBE, Chronicle Queue, and other low-latency messaging systems;
- exchange gateways, FIX engines, and binary codecs;
- formal verification and model checking for matching/risk state;
- cryptographic signing, audit, and regulatory timestamping;
- GPU inference for latency-sensitive but compute-heavy signals;
- cloud placement, EFA, ENA Express, and bare-metal benchmarking;
- market-data capture formats and columnar replay storage.

The rule is to add each technology because a measured bottleneck or correctness requirement demands it.

## References

- [LMAX Disruptor technical paper](https://lmax-exchange.github.io/disruptor/disruptor.html)
- [DPDK documentation](https://doc.dpdk.org/guides/linux_gsg/)
- [Linux RDMA core](https://github.com/linux-rdma/rdma-core/blob/master/Documentation/libibverbs.md)
- [Linux PTP](https://www.linuxptp.org/documentation/ptp4l/)
- [Linux perf and eBPF documentation](https://www.kernel.org/doc/html/v6.4/bpf/index.html)
- [NVIDIA DOCA](https://docs.nvidia.com/doca/sdk/doca-programming-guide/)

Related:

- [[76 - HFT Technology Stack and Priority Map]]
- [[72 - Production Low-Latency Trading System Construction]]
- [[02 - Rust for HFT]]
