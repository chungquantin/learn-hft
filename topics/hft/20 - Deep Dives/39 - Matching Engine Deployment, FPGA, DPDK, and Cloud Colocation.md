---
title: Matching Engine Deployment, FPGA, DPDK, and Cloud Colocation
tags: [matching-engine, deployment, fpga, dpdk, aws, colocation, rust, deep-dive]
---

# Matching Engine Deployment, FPGA, DPDK, and Cloud Colocation

A matching engine is usually discussed as a pure software problem: data structures, price-time priority, command validation, and event emission. That is the right starting point, but it is not the full design. In a real venue or serious trading stack, matching-engine behavior is shaped by the physical and network environment around it: NIC queues, CPU locality, kernel path, user-space packet handling, FPGA offload, and where clients sit relative to the exchange gateway.

This note connects the matching engine to four deployment questions:

- what should be written in Rust
- when DPDK belongs in the design
- where FPGA acceleration helps
- what cloud-native colocation on AWS actually means

The most important principle is sequencing. Do not jump to FPGA or DPDK before the engine has deterministic semantics, replayable events, measured latency, and clear ownership. Specialized infrastructure amplifies a good design. It does not rescue a vague one.

## Matching engine boundaries

A production venue is not only a matching loop. A useful deployment decomposition is:

```text
client / market maker
  -> external access tier
  -> order gateway
  -> risk and session checks
  -> sequencer
  -> matching partition
  -> event publisher
  -> market data publisher
  -> persistence, surveillance, clearing, analytics
```

The matching partition should be the smallest deterministic core that can:

- accept a totally ordered command stream
- validate local instrument/order constraints
- apply matching rules
- emit fills, cancels, rejects, and state events
- preserve replay equivalence

Everything else should justify why it belongs near the hot path. Session login, TLS termination, external protocol decoding, surveillance, historical persistence, dashboards, and analytics usually do not belong inside the matching core.

## Rust development role

Rust fits the matching-engine core because it makes ownership, mutation, allocation, and concurrency explicit without requiring a garbage collector.

Good Rust boundaries:

- `types`: IDs, sides, ticks, quantities, time-in-force, order states
- `command`: validated inbound command model and idempotency keys
- `orderbook`: price levels, FIFO queues, order indexes
- `matcher`: deterministic crossing and residual resting logic
- `engine`: single-writer command dispatch and event sequencing
- `event`: append-only output facts for replay and reconciliation
- `replay`: snapshot plus event-log restoration
- `bench`: workload-specific latency and throughput measurement

The strongest Rust design is usually a single-writer partition model. One pinned thread owns one symbol or shard. It mutates its local book without locks. Input arrives through bounded queues. Output leaves as immutable events. This gives three benefits at once: predictable latency, deterministic ordering, and a recovery model that can be tested by replay.

Avoid these traps:

- using async runtimes inside the matching loop
- hiding allocations behind convenient containers
- logging strings from the hot path
- sharing mutable order-book state across threads
- allowing wall-clock time to affect deterministic matching decisions
- making persistence synchronous with order matching

Rust should own correctness before it owns cleverness. A matching engine with boring data structures and strong invariants is better than one with exotic structures and unclear replay behavior.

## Concrete Rust hot-path shape

The hot path should look closer to this:

```text
while running:
  cmd = ingress.pop()
  seq = next_sequence()
  result = partition.apply(seq, cmd)
  event_sink.publish(result.events)
```

Inside `apply`:

```text
validate command
dedupe idempotency key
emit accepted or rejected
while marketable and quantity remains:
  read best opposite price level
  consume FIFO resting order
  emit trade event
  update or remove resting order
if residual can rest:
  insert into own-side price level FIFO
  emit resting event
emit terminal command result
```

Hot-path allocation should be designed out. Use integer ticks, fixed-width IDs, pre-allocated arenas or slabs for live orders, and compact event structs. If a structure can allocate during a cancel storm or partial-fill sweep, it needs measurement and likely redesign.

## DPDK in the architecture

DPDK belongs below the matching engine, not inside the matching logic. It is a user-space packet I/O toolkit for building low-latency data planes. In an exchange architecture, DPDK is most likely to appear in:

- order gateways
- market-data publishers
- packet capture
- internal multicast or custom distribution
- risk gateways that must process very high message rates

The matching engine should receive already normalized commands, not raw Ethernet frames. A DPDK gateway can own NIC queues, parse packets, validate sessions, decode protocol frames, and enqueue clean commands into the sequencer or matching partition.

Useful DPDK design choices:

- one poll-mode thread per hot NIC queue
- core pinning and NUMA-local memory
- huge-page-backed mempools
- pre-allocated packet buffers
- burst receive and transmit
- explicit drop/backpressure policy
- separate telemetry path from packet path

DPDK is less natural when the exchange API is ordinary TCP unless paired with a user-space TCP/IP stack or a carefully designed socket acceleration layer. Raw DPDK gives packet control, not a complete venue protocol by itself.

The clean boundary is:

```text
DPDK / kernel-bypass gateway
  -> normalized command queue
  -> deterministic sequencer
  -> Rust matching partition
```

This keeps packet mechanics from infecting matching semantics.

## FPGA in the architecture

FPGA acceleration is for extremely stable, latency-critical, pipeline-friendly work. It is not a general replacement for a matching engine unless the venue has the engineering budget and operational maturity to make hardware logic part of the product lifecycle.

FPGA can help with:

- Ethernet, IP, UDP, TCP, or feed-protocol parsing
- market-data normalization
- timestamping
- pre-trade risk checks
- simple strategy triggers
- order-entry message generation
- market-data fanout
- packet filtering and capture
- line-rate checksumming and framing

FPGA is harder for:

- complex matching rules that change often
- multi-symbol global policies
- rich order types with many edge cases
- surveillance and compliance workflows
- business logic that product teams frequently revise
- debugging ambiguous state after an incident

The most practical split is often hybrid:

```text
FPGA / SmartNIC:
  parse, timestamp, filter, simple risk, fast response

Rust / CPU:
  matching semantics, replay, recovery, product rules, orchestration
```

For an exchange-operated matching engine, FPGA may be used around the matching engine before the matching core itself is moved into hardware. The reason is change velocity. Hardware pipelines can be extraordinarily fast, but they are more expensive to develop, verify, deploy, and debug than Rust services. Matching semantics are legally and economically sensitive, so correctness and auditability dominate until the rules are frozen and the latency budget demands hardware.

## FPGA development model

If FPGA becomes real, treat it as a separate product with its own verification ladder:

1. Define a narrow hardware function.
2. Build a bit-accurate software reference model.
3. Generate shared test vectors from production-like traffic.
4. Verify RTL or HLS output against the reference model.
5. Run hardware-in-the-loop tests.
6. Compare timestamped end-to-end behavior against the CPU path.
7. Deploy behind a feature flag or shadow path before trusting production flow.

Do not begin with "put the matching engine on FPGA." Begin with one function whose inputs, outputs, and failure behavior are small enough to prove.

Good first FPGA candidates:

- feed packet timestamping
- UDP multicast decode
- fixed-format order-entry encode
- simple max-order-size or price-band risk gate
- deterministic packet fanout

Poor first candidates:

- full matching engine with many order types
- cross-instrument risk
- dynamic fee logic
- anything requiring frequent product changes

## AWS cloud-native colocation

The AWS and One Trading reference is important because it shows a crypto-specific variant of colocation. Traditional colocation means placing participant servers in, or extremely near, an exchange data center. Cloud-native colocation means using cloud placement and private networking controls to reduce distance and hops between exchange and participant workloads.

The One Trading/AWS testing compared several access topologies:

- VPC peering with shared EC2 cluster placement groups
- VPC peering without shared cluster placement groups
- AWS PrivateLink
- internet access through CloudFront

The lowest-latency tier was VPC peering plus shared EC2 cluster placement groups. The reason is physical locality inside one Availability Zone plus private logical connectivity between the exchange VPC and market-maker VPC. Plain VPC peering keeps private connectivity but loses the same placement-locality guarantee. PrivateLink adds service endpoints and load-balancer hops. Internet/CloudFront is broadest and most accessible, but highest-latency.

The important design lesson is that cloud latency is topology-sensitive. Region, Availability Zone, placement group, instance type, VPC path, load balancers, NAT, and public routing can dominate the careful work done inside the matching engine.

## AWS topology ladder

A crypto exchange on AWS can expose multiple connectivity tiers:

```text
Tier 1:
  same Region + same AZ + shared cluster placement group + VPC peering
  -> lowest cloud-native access latency

Tier 2:
  same Region + same AZ + VPC peering
  -> private, low latency, less physical locality control

Tier 3:
  PrivateLink
  -> scalable private service access, additional network hops

Tier 4:
  public internet / CloudFront / public load balancer
  -> broad access, highest latency, useful for non-colocated clients
```

For market makers, this creates a venue-access market structure. Participants in Tier 1 have a different latency profile from participants over public internet. The exchange must think about fairness, disclosure, eligibility, capacity, and whether the market design assumes symmetric access.

## AWS instance and OS considerations

The AWS test used bare-metal EC2 instances and application-level optimizations such as core pinning, thread segregation, buffer management, io_uring, and JVM tuning. The blog explicitly notes that further low-latency tuning such as IRQ handling, P-state/C-state controls, kernel bypass, RSS, transmit packet steering, Linux scheduler policy, and ENA tuning was not applied in the baseline.

For a Rust matching-engine deployment, the equivalent checklist is:

- use bare-metal or latency-suitable instance types where possible
- keep matching partitions on isolated pinned cores
- avoid cross-AZ traffic in the hot path
- use cluster placement groups for tightly coupled low-latency components
- use enhanced networking and verify ENA driver behavior
- evaluate ENA Express or EFA only where their communication model matches the workload
- avoid NAT gateways, public load balancers, and unnecessary service hops in the market-maker path
- separate hot path from persistence, analytics, and dashboards
- measure p99 and p99.9 by topology, not just inside one process

Cloud-native colocation is not the same as classical exchange colocation. It can be very powerful for crypto because many venues and participants are already cloud-native. But it remains a network design problem, not a checkbox.

## Matching engine plus AWS placement

A reasonable AWS-native exchange topology:

```text
Market maker VPC
  trade engine EC2 in shared CPG
  private subnet
  pinned client threads
       |
       | VPC peering
       v
Exchange VPC
  order gateways in shared CPG
  sequencer near gateways
  matching partitions on pinned bare-metal cores
  market data publishers near matching partitions
       |
       +-> async persistence
       +-> async surveillance
       +-> async analytics
       +-> cross-AZ replicas outside hot path
```

Resilience still matters. A single-AZ hot path can reduce latency, but production venues usually need multi-AZ or multi-region recovery. The important distinction is hot path versus recovery path. Synchronous cross-AZ coordination in the matching loop can destroy the latency profile. Asynchronous replication may preserve latency but creates recovery-point and failover questions. That tradeoff has to be explicit.

## Sequencing and fairness

Once clients have different connectivity tiers, sequencing becomes a market-design question. The matching engine must define the authoritative ordering point. Is order priority determined when the gateway receives the packet, when the sequencer stamps the command, or when the matching partition dequeues it?

For a serious venue, the answer must be:

- deterministic
- documented
- monitored
- replayable
- hard to manipulate

Cloud placement affects arrival time, but the engine still needs one authoritative event order. The sequencer is usually the clean boundary: gateways normalize and authenticate, then the sequencer assigns a monotonically increasing order per partition or product group. Matching partitions consume that order deterministically.

## Where quant topics connect

Deployment design changes quant research. A strategy that looks profitable at `50us` may fail at `500us`. A market maker with Tier 1 access may quote tighter than one over the public path. A CLMM arbitrage strategy may be dominated by chain confirmation and MEV rather than local compute. A backtest that ignores connectivity tier is not modeling the actual game.

So quant research should model:

- latency distributions by topology
- queue position decay by delay
- adverse selection after passive fills
- cancel success probability under bursts
- fill probability by queue depth
- market impact under available depth
- venue-specific fee and rebate schedules
- recovery behavior during feed gaps

In other words, infrastructure is a feature of the market.

## Design ladder

The practical ladder is:

1. Implement deterministic matching in Rust.
2. Add property tests, replay, event sourcing, and snapshot recovery.
3. Benchmark realistic command mixes and tail latency.
4. Pin threads and remove hot-path allocations.
5. Separate order gateway, sequencer, matching partition, and event publisher.
6. Measure topology: same host, same AZ, shared CPG, peering, PrivateLink, internet.
7. Add DPDK or socket acceleration to gateways if packet path is the bottleneck.
8. Add FPGA only for narrow proven functions.
9. Re-evaluate market fairness and operational recovery after every latency-tier change.

The ladder matters because each rung changes the system's cost model. Skipping rungs creates impressive-looking infrastructure with unclear economics.

Related:

- [[14 - Low-Latency Systems Foundations]]
- [[15 - Benchmarking and Tick-to-Trade Measurement]]
- [[18 - Time and Timestamp Semantics]]
- [[19 - Matching Engines, Queue Priority, and Order Amend Semantics]]
- [[37 - Kernel Bypass Technologies Deep Dive]]
- [[57 - Matching Engine Component Plan]]
- [[65 - HFT Rust System Design Master Note]]
- [[63 - Rust Matching Engine Implementation Blueprint]]
- [[64 - Matching Engine Benchmark and Profiling Plan]]
