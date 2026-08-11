---
title: Performance Engineering and eBPF Deep Dive
tags: [performance, profiling, perf, ebpf, observability, latency, hft]
---

# Performance Engineering and eBPF Deep Dive

## The investigation loop

Expert performance work is a loop, not a collection of tuning flags:

```text
hypothesis -> controlled benchmark -> profile -> change
           -> repeatable comparison -> production validation
```

Start with a stage-level latency distribution. Then identify whether the cost is CPU, memory, synchronization, I/O, scheduling, queueing, or external.

## Tools

- `perf stat`: cycles, instructions, branches, cache misses, migrations, page faults;
- `perf record` and `perf report`: sampled call stacks and hot functions;
- flame graphs: visual aggregation of sampled stacks;
- `ftrace` and tracepoints: kernel scheduling, networking, and IRQ behavior;
- eBPF/libbpf: programmable low-overhead observation and event export;
- BPF ring buffer: move structured events to user space while preserving useful ordering;
- NIC and switch counters: drops, pauses, ECN, queue depth, and errors;
- application histograms: p50, p99, p99.9, max, and coordinated omission checks.

## What to measure

```text
wire_rx -> NIC_RX -> parser -> book -> strategy -> risk -> NIC_TX -> wire_tx
```

Capture timestamps and counters at each boundary. Add queue depth, buffer occupancy, CPU migration, context switches, cache misses, and packet loss. A single end-to-end average cannot tell whether the problem is parsing or queueing.

## eBPF's role

eBPF is primarily an observability and policy mechanism around the critical path. It can observe scheduler events, socket behavior, packet paths, syscalls, and resource failures without rewriting the application. It is not a substitute for DPDK or FPGA in a microsecond packet path.

Use eBPF for:

- detecting unexpected scheduler migrations;
- observing kernel fallback from an accelerated socket path;
- tracing retransmissions and socket errors;
- correlating IRQ, softirq, and application behavior;
- measuring control-plane impact on isolated cores;
- exporting structured events to a ring buffer.

## Profiling pitfalls

- Sampling can perturb very short critical sections.
- Averages hide queueing and coordinated stalls.
- Hardware counters are CPU-specific.
- A benchmark that omits bursts can hide buffer exhaustion.
- Flame graphs show where samples landed, not necessarily causal latency.
- Instrumentation timestamps may be on different clock domains.
- Production profiling needs access control because perf data can expose sensitive memory and execution details.

## Expert workflow

1. Establish a baseline with a pinned, warmed-up workload.
2. Record hardware, firmware, kernel, compiler, and configuration hashes.
3. Measure the same packet/message distribution after every change.
4. Profile both fast and slow samples.
5. Check for tail correlation with queue depth, migrations, and clock state.
6. Validate under real burst and failure scenarios.

## References

- [Linux perf events and security](https://docs.kernel.org/6.2/admin-guide/perf-security.html)
- [Linux eBPF documentation](https://www.kernel.org/doc/html/v6.4/bpf/index.html)
- [Linux BPF ring buffer](https://www.kernel.org/doc/html/latest/bpf/ringbuf.html)
- [Linux timestamping API](https://docs.kernel.org/6.6/networking/timestamping.html)

Related:

- [[15 - Benchmarking and Tick-to-Trade Measurement]]
- [[25 - Logging and Telemetry Deep Dive]]
- [[75 - Hardware Timestamping and PTP Deep Dive]]
