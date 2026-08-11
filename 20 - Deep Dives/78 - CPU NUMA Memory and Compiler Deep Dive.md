---
title: CPU NUMA Memory and Compiler Deep Dive
tags: [cpu, numa, cache, memory, compiler, performance, hft]
---

# CPU NUMA Memory and Compiler Deep Dive

## Why this matters

After networking is optimized, the next latency boundary is often the CPU-memory system. A feed handler can receive a packet quickly and still lose the benefit through cache misses, remote NUMA access, branch misprediction, allocator work, or a thread migration.

The goal is not to memorize cache sizes. It is to understand where hot state lives, which core owns it, how data moves, and which instructions the compiler actually emits.

## Topics to master

- cache lines, associativity, prefetching, and false sharing;
- L1/L2/L3 and shared versus private cache topology;
- NUMA nodes, memory controllers, and PCIe locality;
- TLBs, hugepages, page faults, and memory registration;
- branch prediction and data-dependent control flow;
- SIMD/vectorization and alignment;
- atomics, fences, acquire/release, and cache coherence;
- compiler inlining, alias analysis, LTO, PGO, and assembly inspection;
- CPU affinity, SMT, C-states, frequency scaling, and thermal behavior.

## HFT layout rule

```text
NIC queue -> pinned core -> local parser state -> local book partition
```

Keep a hot data structure on the NUMA node that owns the NIC queue. If another thread needs the data, publish a compact snapshot or event rather than sharing a large mutable structure.

## Data layout

Prefer compact structures with explicit alignment and ownership. Separate hot fields from cold fields. Avoid putting a frequently updated sequence counter on the same cache line as unrelated read-mostly state.

```text
hot order: price_tick, quantity, side, next_index
cold order: client metadata, audit text, debug fields
```

Use indexes into arrays or slabs when pointer chasing is expensive. Benchmark array-of-structures against structure-of-arrays for the actual access pattern.

## Compiler discipline

The source code is not the performance model. Inspect assembly and measure:

- whether a function was inlined;
- whether bounds checks or overflow checks remain;
- whether a loop vectorized;
- whether branches were eliminated or reordered;
- whether atomics became the intended instructions;
- whether the compiler introduced unexpected loads and stores.

Unsafe optimization is justified only behind a tested invariant. A faster order book that violates memory safety or replay equivalence is not an HFT improvement.

## Experiments

1. Pin the same benchmark to local and remote NUMA memory.
2. Add deliberate false sharing and measure the tail.
3. Compare pointer-heavy and index-based book layouts.
4. Compare branchy and branchless best-price updates.
5. Inspect Rust release assembly and perf counters.
6. Measure page faults and TLB behavior with normal pages and hugepages.

## References

- [DPDK EAL and hugepage-backed memory](https://doc.dpdk.org/guides/prog_guide/env_abstraction_layer.html)
- [Linux perf security and performance counters](https://docs.kernel.org/6.2/admin-guide/perf-security.html)
- [LMAX Disruptor technical paper](https://lmax-exchange.github.io/disruptor/disruptor.html)

Related:

- [[14 - Low-Latency Systems Foundations]]
- [[23 - Seqlocks Deep Dive]]
- [[77 - LMAX Disruptor and Exchange Architecture]]
