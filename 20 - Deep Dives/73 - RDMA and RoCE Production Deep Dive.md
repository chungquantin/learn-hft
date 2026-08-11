---
title: RDMA and RoCE Production Deep Dive
tags: [rdma, roce, infiniband, verbs, networking, production, hft]
---

# RDMA and RoCE Production Deep Dive

## The production question

RDMA is valuable when the application is moving data between known peers and the cost of CPU copies, kernel traversal, or synchronization dominates. It is not automatically the best transport for an HFT system. A single host may get lower latency from cache-local shared memory; a venue-facing TCP session may be constrained by the exchange; and an incorrectly tuned RoCE fabric can introduce congestion and tail stalls.

The decision must therefore be based on topology and measured stages:

```text
wire / NIC -> decode -> state -> strategy -> risk -> encode -> NIC / wire
                 \------ internal RDMA candidate ------/
```

Use RDMA most naturally for the internal multi-host plant, GPU/accelerator communication, storage fabrics, or controlled market-data distribution. Keep venue protocol and business semantics above the transport.

## InfiniBand, RoCE, and verbs

- **InfiniBand** is a complete switched fabric architecture with RDMA semantics.
- **RoCE** carries RDMA over Ethernet; RoCEv2 uses IP routing.
- **libibverbs** exposes user-space access to RDMA hardware.
- **rdma_cm** handles connection management and addressing.
- **QP** is the queue pair for send/receive or RDMA operations.
- **CQ** records completed work requests.

The verbs API is intentionally low-level. The application owns registered memory, queue depth, buffer lifecycle, posting receives, completion polling, and error recovery.

## Memory registration and buffer ownership

Memory registration is a performance boundary. Register long-lived pools during startup and reuse them. Registering and deregistering for every message creates latency and resource pressure.

Define ownership explicitly:

```text
FREE -> POSTED_RX -> APPLICATION_READ -> RECYCLE
FREE -> LOCAL_WRITE -> POSTED_SEND -> TX_COMPLETE -> RECYCLE
```

Never recycle a buffer until the corresponding completion proves that hardware no longer owns it. For one-sided operations, the remote application must also define when the written region is safe to consume. RDMA transport completion is not the same as a business event.

## Queue-pair design

Choose QP count, CQ count, queue depth, and polling ownership from the topology. A common low-latency pattern is one QP/CQ pair per producer-consumer lane or per NUMA-local worker. Sharing everything through one QP may simplify setup but create contention and head-of-line effects.

Polling CQs gives the tightest control but consumes a core. Event-driven completion reduces idle CPU use but adds wakeup variability. Hybrid polling can poll aggressively during bursts and back off during quiet periods, but the transition itself must be measured.

## RoCE fabric design

RoCE needs network engineering, not only NIC configuration. Document:

- MTU and frame-size policy;
- priority and traffic-class mapping;
- PFC policy and pause scope;
- ECN marking and congestion control;
- switch buffer allocation;
- DSCP/priority mapping end to end;
- multicast and routing behavior;
- telemetry for drops, pause frames, ECN, and queue depth.

The danger is treating “lossless Ethernet” as a universal property. PFC can prevent drops for a class while spreading backpressure to unrelated traffic. Congestion control must be tested under incast and burst traffic, not only a steady point-to-point benchmark.

## HFT use cases

### Internal event distribution

RDMA can distribute normalized market-data events between feed handlers and strategy hosts. Consider whether a replicated shared-memory ring, kernel-bypass UDP, or RDMA gives the best combination of ordering, recovery, and operational simplicity.

### Cross-host risk and execution

Cross-host risk checks are dangerous if they insert an unbounded network dependency into the order path. Prefer colocated risk state or a local read-only snapshot with explicit staleness bounds. Use RDMA only when the ownership and failure model are stronger than the alternative.

### GPU/accelerator communication

GPUDirect RDMA can remove host staging between NICs and GPU memory. NCCL or another collective library still defines the communication algorithm; RDMA is one transport layer below it.

## Failure model

Implement and test:

- QP error and reconnection;
- remote process death;
- stale remote keys;
- CQ overflow;
- receive-queue starvation;
- memory-registration exhaustion;
- RoCE pause and congestion storms;
- fabric partition;
- sequence gap and replay recovery;
- duplicate or late event after reconnect.

The recovery protocol should fence the old producer, establish a new epoch, replay from a known sequence, and publish a state-valid marker before the strategy resumes.

## Benchmark matrix

Compare RDMA against shared memory, DPDK UDP, Onload TCP/UDP, and ordinary sockets using:

- one-way latency;
- p99 and p99.9 latency;
- burst absorption;
- CPU cycles per message;
- memory footprint;
- loss and recovery behavior;
- failure detection time;
- operational complexity.

The winner is the system that preserves correctness and tail behavior under the expected burst, not the lowest isolated ping.

## References

- [Linux RDMA core and libibverbs](https://github.com/linux-rdma/rdma-core/blob/master/Documentation/libibverbs.md)
- [NVIDIA RDMA Verbs](https://docs.nvidia.com/doca/sdk/doca-rdma-verbs/)
- [NVIDIA RoCE documentation](https://docs.nvidia.com/networking-ethernet-software/cumulus-linux/Layer-1-and-Switch-Ports/Quality-of-Service/RDMA-over-Converged-Ethernet-RoCE/)
- [NVIDIA GPUDirect RDMA](https://docs.nvidia.com/cuda/gpudirect-rdma/)
- [Meta: RDMA over Ethernet for Distributed AI Training](https://engineering.fb.com/wp-content/uploads/2024/08/sigcomm24-final246.pdf)

Related:

- [[68 - InfiniBand and RDMA Deep Dive]]
- [[69 - DPDK Deep Dive]]
- [[72 - Production Low-Latency Trading System Construction]]
