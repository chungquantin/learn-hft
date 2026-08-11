---
title: InfiniBand and RDMA Deep Dive
tags: [rdma, infiniband, roce, networking, low-latency, ai-infrastructure, deep-dive]
---

# InfiniBand and RDMA Deep Dive

## Mental model

RDMA is a family of data-movement and messaging semantics. InfiniBand is a fabric and protocol architecture that implements them. RoCE carries RDMA over Ethernet. They are related, but not synonyms.

The key idea is that an application can prepare memory and work requests in user space, submit them to a NIC, and receive completion records without copying every message through the remote CPU's normal socket path.

```text
application buffer
  -> registered memory region
  -> work request on a queue pair
  -> NIC DMA / fabric
  -> remote registered memory or receive queue
  -> completion queue entry
```

RDMA does not mean that all CPU work disappears. Connection setup, memory registration, queue management, flow control, error handling, and application protocol logic still exist. The optimization is that the steady-state data path is explicit and hardware-assisted.

## Core objects

### Protection domain

A protection domain groups resources that are allowed to interact. It is part of the isolation model around memory regions, queue pairs, and completion queues.

### Memory region

Before a NIC can access application memory, the region is registered. Registration pins or otherwise prepares pages and produces local and remote keys used by the hardware. Registration is expensive enough that production systems normally cache or pre-register long-lived buffers rather than register on every message.

### Queue pair

A queue pair contains a send queue and a receive queue. Applications post work requests to it. A send may transfer data to a remote receive queue; an RDMA write may place data directly into a remote registered region; an RDMA read fetches data from a remote region.

### Completion queue

A completion queue collects completion records for posted operations. The application can poll the CQ for predictable low latency or use an event channel when CPU efficiency matters more than the shortest path.

### Work request and scatter/gather entries

A work request describes an operation and points to one or more buffers. Scatter/gather entries let a message be assembled from multiple regions without first copying it into one contiguous application buffer.

## Reliable connection flow

A simplified reliable-connected setup is:

1. Allocate a protection domain and completion queues.
2. Register send and receive buffers.
3. Create a queue pair.
4. Exchange addressing and queue metadata through a control channel, often RDMA CM or ordinary TCP.
5. Move both queue pairs through the required state transitions.
6. Post receive work requests before the peer sends.
7. Post send, write, or read work requests.
8. Poll completions and recycle buffers.

The receive-posting rule is operationally important. A fast sender can exhaust the receiver's posted buffers. A production design needs explicit sizing, backpressure, and behavior for receive-queue starvation.

## InfiniBand versus RoCE

InfiniBand provides a purpose-built switched fabric with its own link-layer behavior and management model. RoCE preserves RDMA-style verbs while using Ethernet. RoCE therefore integrates with Ethernet infrastructure, but congestion and loss behavior must be engineered carefully. Priority flow control, traffic classes, ECN, queue sizing, and switch configuration become part of the application performance envelope.

RoCE v2 is routable at the IP layer, which makes it more flexible than a single L2 domain, but this does not remove the need for congestion design. An incorrectly tuned RoCE fabric can turn a low-copy transport into a source of queue buildup and synchronized stalls.

## GPUDirect RDMA

GPUDirect RDMA extends peer-to-peer DMA to GPU memory. A NIC can exchange data with GPU memory when PCIe topology, driver support, memory mapping, and platform constraints permit it. In distributed AI, this reduces host-memory staging and helps collective libraries keep GPUs fed.

The analogy to HFT is useful but incomplete. An HFT system cares about a small message reaching a decision path with minimal jitter. AI training cares about the progress of a collective across many devices. A single slow or congested rank can hold up the group, so bandwidth, topology, and tail behavior matter as much as per-message latency.

## When RDMA is a good fit

- GPU-to-GPU or accelerator-to-accelerator communication
- distributed storage and NVMe over Fabrics
- high-throughput internal messaging with stable peers
- data-plane services where CPU copies dominate
- tightly controlled HPC or colocation fabrics

It is a poor first choice for a small system whose real bottleneck is parsing, queueing, serialization, exchange-side latency, or unclear ownership. RDMA moves complexity downward; it does not remove complexity.

## HFT application pattern

```text
NIC / feed handler -> normalized event
                    -> single-writer strategy partition
                    -> risk gate
                    -> order gateway
```

RDMA can be useful for internal distribution, but raw RDMA should not be inserted merely because it is fast. For one host, shared memory and cache-local queues may be faster and simpler. For a multi-host plant, compare RDMA, kernel-bypass UDP, and ordinary sockets using the same workload and hardware timestamps.

## Failure modes

- memory registration failure or locked-memory limits;
- receiver ran out of posted buffers;
- completion queue overflow;
- stale or invalid remote keys;
- QP transition or connection-management failure;
- RoCE congestion and pause storms;
- PCIe root-complex or NUMA misplacement;
- silent mismatch between transport ordering and business ordering.

RDMA gives transport-level completion, not business-level correctness. The application still needs sequence numbers, idempotency, replay, fencing, and recovery.

## References

- [InfiniBand Trade Association: About InfiniBand](https://www.infinibandta.org/about-infiniband/)
- [Linux RDMA core and libibverbs](https://github.com/linux-rdma/rdma-core/blob/master/Documentation/libibverbs.md)
- [NVIDIA: GPUDirect RDMA](https://docs.nvidia.com/cuda/gpudirect-rdma/)
- [NVIDIA: PeerDirect](https://docs.nvidia.com/doca/sdk/NVIDIA-PeerDirect/index.html)
- [NVIDIA: RoCE documentation](https://docs.nvidia.com/networking-ethernet-software/cumulus-linux/Layer-1-and-Switch-Ports/Quality-of-Service/RDMA-over-Converged-Ethernet-RoCE/)
- [NVIDIA: RDMA Verbs](https://docs.nvidia.com/doca/sdk/doca-rdma-verbs/)

Related:

- [[48 - HFT to AI Infrastructure Technology Transfer]]
- [[37 - Kernel Bypass Technologies Deep Dive]]
- [[18 - Time and Timestamp Semantics]]
