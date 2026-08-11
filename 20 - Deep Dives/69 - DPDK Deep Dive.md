---
title: DPDK Deep Dive
tags: [dpdk, kernel-bypass, packet-processing, networking, low-latency, deep-dive]
---

# DPDK Deep Dive

## What DPDK is

DPDK is a toolkit for user-space packet processing. It provides an environment abstraction layer, memory and buffer libraries, rings, poll-mode drivers, and device-specific capabilities. It is not a TCP stack, exchange protocol, or matching engine.

The normal Linux socket path optimizes for sharing, portability, safety, and features. DPDK optimizes for explicit ownership and high packet rate. The application polls NIC descriptors, obtains preallocated packet buffers, processes them, and returns or transmits them.

## Main components

The **Environment Abstraction Layer (EAL)** is the startup and resource boundary. It discovers or is given the CPU cores, PCI devices, memory channels, hugepage-backed regions, and process identity that the data plane will use. In production, EAL configuration is part of the host image: changing lcore masks or memory sockets can change performance even when the application binary is identical.

A **Poll Mode Driver (PMD)** is the device-specific implementation that configures queues and accesses RX/TX descriptors from user space. Polling removes interrupt wakeups from the steady-state path, but it also means a dedicated core is doing work continuously. Queue ownership therefore becomes an architectural decision: one hot thread should normally own a queue, and handoff should happen only when it has a measured benefit.

An **mbuf** carries packet bytes plus metadata such as lengths, offload flags, packet type, and sometimes timestamp information. A **mempool** owns the preallocated objects from which mbufs are obtained and returned. This makes allocation predictable, but it also makes exhaustion explicit: a burst can fail because the pool is empty even when the host still has plenty of general memory.

A **ring** passes references between cores or stages. It is cheaper than copying full packets, but it still creates producer/consumer coordination and cache traffic. The queue's producer/consumer shape and overflow policy must match the meaning of the traffic. A telemetry ring may drop; an order or recovery event generally may not.

The **burst API** receives or transmits several packets per call. Batching amortizes function-call and descriptor-management overhead, but it adds a latency/throughput tradeoff: waiting for a larger burst can improve throughput while making the first packet wait longer. Benchmark burst sizes against realistic feed distributions rather than assuming the largest burst is best.

The **flow API** programs hardware steering and filtering. This is important in HFT because a packet should arrive on the queue and core that own its symbol, venue, or session. Hardware steering can reduce software classification, but incorrect rules can silently starve a queue or direct traffic to the wrong state owner, so rules belong in versioned configuration and must be tested with packet captures.

## Polling loop

```c
for (;;) {
    count = rte_eth_rx_burst(port, queue, packets, BURST);
    for (i = 0; i < count; i++) {
        event = decode(packets[i]);
        route(event);
        rte_pktmbuf_free(packets[i]);
    }
}
```

The real design must handle malformed frames, bursts, buffer exhaustion, queue ownership, statistics, timestamps, and shutdown. A busy-polling core consumes CPU while idle, but avoids scheduler wakeups and interrupt jitter.

## Memory and device binding

DPDK commonly uses hugepages to reduce translation overhead and make DMA mappings manageable. The NIC may be bound to `vfio-pci`, which uses IOMMU protection, or use a bifurcated driver where the kernel retains control while the DPDK PMD handles selected data paths. Device binding is a security and operational boundary, not just a startup command.

For production, document:

- exact PCI addresses and NIC ports;
- firmware and driver versions;
- IOMMU/VFIO configuration;
- hugepage size, count, and NUMA node;
- lcore-to-queue mapping;
- RX/TX descriptor counts;
- RSS and hardware flow rules;
- ownership of kernel-visible versus DPDK-owned traffic.

## Run-to-completion versus pipeline

In run-to-completion, one core receives a packet, parses it, applies the required logic, and transmits or publishes the result. This minimizes handoffs and is attractive for a narrow gateway.

In a pipeline, separate cores handle receive, decode, state update, risk, and transmit. This can increase throughput and isolate functions, but each ring adds queueing and cache movement. Pipeline only when one stage cannot keep up or when ownership boundaries justify the handoff.

## DPDK in an HFT system

The natural placement is at the gateway:

```text
wire -> DPDK RX -> protocol decode -> normalized command
                                  -> bounded command queue
                                  -> Rust matching/strategy core
```

For UDP multicast market data, DPDK can be appropriate for receive, filtering, timestamp capture, and packet fanout. For TCP exchange connectivity, DPDK alone is insufficient; use a socket accelerator or user-space TCP stack if the measurement shows the kernel TCP path is the bottleneck.

Do not put raw packet parsing inside the matching engine. Keep network mechanics and business semantics separate so replay and correctness tests can operate on normalized events.

The normalized-event boundary is the most important design boundary in this architecture. The DPDK side should answer, "What packet arrived, when did it arrive, and is it structurally valid?" The Rust event core should answer, "What does this event mean for market state, risk, and execution?" Keeping those questions separate allows the packet path to change from kernel sockets to DPDK or FPGA without changing matching semantics, and it allows replay to run without a NIC.

## Measurement plan

Benchmark four stages separately:

1. NIC wire timestamp to DPDK receive.
2. Receive to normalized event.
3. Normalized event through the strategy or matching partition.
4. Decision to transmit and wire timestamp.

Measure burst size, packet loss, queue depth, CPU cycles, cache misses, and p50/p99/p99.9 latency. A lower median with worse p99 under realistic bursts is not a production win.

## Failure and operations

- Mempool exhaustion must be observable and bounded.
- A full internal ring needs an explicit policy: backpressure, reject, drop telemetry, or fail closed.
- Poll loops need watchdogs and core-health checks.
- DPDK-owned ports need a recovery procedure to restore kernel control.
- Packet capture and counters must remain available when the fast path is active.
- Privileges and device access must be isolated; avoid unsafe no-IOMMU mode in production.

## References

- [DPDK overview](https://doc.dpdk.org/guides-26.07/prog_guide/overview.html)
- [DPDK EAL](https://doc.dpdk.org/guides/prog_guide/env_abstraction_layer.html)
- [DPDK Poll Mode Driver](https://doc.dpdk.org/guides-17.11/prog_guide/poll_mode_drv.html)
- [DPDK Linux drivers and VFIO](https://doc.dpdk.org/guides-25.07/linux_gsg/linux_drivers.html)
- [DPDK Linux Getting Started Guide](https://doc.dpdk.org/guides/linux_gsg/)

Related:

- [[37 - Kernel Bypass Technologies Deep Dive]]
- [[39 - Matching Engine Deployment, FPGA, DPDK, and Cloud Colocation]]
- [[68 - InfiniBand and RDMA Deep Dive]]
