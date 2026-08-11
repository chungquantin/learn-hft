---
title: HFT Technology Stack and Priority Map
tags: [hft, technology-stack, architecture, networking, hardware, systems]
---

# HFT Technology Stack and Priority Map

## The complete stack

HFT performance is not one technology. It is a chain where every layer can create latency, loss, or ambiguity.

```text
venue protocol and session
  -> cross-connect and switch fabric
  -> NIC / FPGA / hardware timestamp
  -> kernel, Onload, ef_vi, DPDK, or RDMA
  -> parser and feed handler
  -> sequence and book state
  -> strategy and feature computation
  -> risk and order management
  -> encoder and gateway
  -> replay, telemetry, and recovery
```

## Technologies worth learning

### Connectivity and protocols

- exchange binary protocols;
- FIX, FAST, ITCH/OUCH-style feeds;
- TCP session management;
- UDP multicast, gap detection, and recovery;
- sequence numbers, heartbeats, rejects, and cancel/replace semantics;
- cross-connects, VLANs, BGP, and private routing.

### NIC and packet path

- RSS and hardware flow steering;
- RX/TX descriptor rings;
- checksum, segmentation, and timestamp offloads;
- Solarflare Onload and ef_vi;
- DPDK PMDs, mbufs, mempools, and rings;
- AF_XDP and kernel-integrated fast paths;
- NIC firmware, queue ownership, and interrupt/polling policy.

### Distributed transport

- InfiniBand and RDMA verbs;
- RoCEv2, PFC, ECN, and congestion control;
- shared memory and lock-free IPC;
- multicast fanout;
- topology-aware message routing;
- GPU/accelerator peer DMA.

### Compute and memory

- CPU cache hierarchy and false sharing;
- NUMA and PCIe locality;
- core isolation, CPU pinning, and SMT policy;
- hugepages, TLB behavior, and memory registration;
- branch prediction, SIMD, and compiler output;
- allocators, arenas, slabs, and zero-allocation paths;
- lock-free, wait-free, seqlock, and RCU tradeoffs.

### Hardware acceleration

- FPGA feed handlers;
- Alveo and SmartNIC platforms;
- custom RTL and HLS;
- inline filtering and risk;
- GPU inference and batch-size tradeoffs;
- ASIC or switch-assisted aggregation;
- hardware verification and shadow mode.

### Time and measurement

- IEEE 1588 PTP;
- PHC and `ptp4l`/`phc2sys`;
- NIC RX/TX hardware timestamps;
- monotonic versus wall-clock time;
- per-stage latency histograms;
- packet capture, sequence gaps, and drop counters;
- tail-latency attribution.

### State, research, and recovery

- deterministic single-writer books;
- LMAX Disruptor-style sequencing and dependency graphs;
- event sourcing and replay;
- snapshots and checkpoint fencing;
- kdb+/q, ClickHouse, and columnar analytics;
- feature computation without look-ahead;
- order and risk state reconciliation;
- disaster recovery and warm standby.

## Priority order for a real build

1. Correct venue protocol and order semantics.
2. Deterministic state machine and replay.
3. Hardware timestamps and stage-level measurement.
4. CPU/NUMA/PCIe topology and allocation discipline.
5. Feed-handler and gateway optimization.
6. Kernel bypass or socket acceleration where measurements justify it.
7. FPGA or inline acceleration for stable functions.
8. RDMA/RoCE for controlled cross-host communication.
9. Advanced telemetry, redundancy, and incident automation.

This order prevents the common mistake of buying the most specialized hardware before proving where time and correctness are actually being lost.

## Decision table

| Problem | First technology to test | Escalate to |
| --- | --- | --- |
| Kernel socket overhead | Onload or tuned sockets | ef_vi or DPDK |
| Raw packet rate | DPDK or ef_vi | FPGA/SmartNIC |
| Multicast burst loss | NIC queues, buffers, flow steering | FPGA fanout or capture appliance |
| Cross-host copies | shared memory or compact messages | RDMA/RoCE |
| Feed decoding cost | optimized parser | FPGA pipeline |
| Unexplained latency | PTP and hardware timestamps | switch/NIC telemetry |
| Slow historical scans | columnar/vectorized engine | distributed analytical storage |
| Non-deterministic state | single writer and replay | hardware only after semantics stabilize |
| Tail stalls | queue/cpu/NUMA instrumentation | topology redesign |

## Anti-patterns

- treating RDMA as a business protocol;
- using DPDK without a buffer and recovery model;
- enabling Onload without proving which flows accelerate;
- putting complex mutable semantics in FPGA before a reference model exists;
- using PTP timestamps as event ordering;
- optimizing averages while ignoring p99.9;
- sharing a NIC queue across unrelated hot paths;
- allowing synchronous logging or persistence in the order path;
- assuming a cloud instance has deterministic colocation because it is in the same region;
- adding hardware before fixing allocations, queueing, and state ownership.

## Capstone build

Use [[72 - Production Low-Latency Trading System Construction]] as the implementation guide. Study the stack in this order:

1. [[31 - Market Data Ingestion Deep Dive]]
2. [[32 - Order Book Engine Deep Dive]]
3. [[15 - Benchmarking and Tick-to-Trade Measurement]]
4. [[75 - Hardware Timestamping and PTP Deep Dive]]
5. [[69 - DPDK Deep Dive]] or [[70 - Solarflare Onload and ef_vi Deep Dive]]
6. [[74 - FPGA Feed Handlers and Inline Accelerators]]
7. [[73 - RDMA and RoCE Production Deep Dive]]
8. [[72 - Production Low-Latency Trading System Construction]]
9. [[77 - LMAX Disruptor and Exchange Architecture]]

Related:

- [[48 - HFT to AI Infrastructure Technology Transfer]]
- [[37 - Kernel Bypass Technologies Deep Dive]]
- [[39 - Matching Engine Deployment, FPGA, DPDK, and Cloud Colocation]]
