---
title: Source Notes
tags: [sources, reading-notes, references]
---

# Source Notes

This page records external readings that shaped the vault.

## Rust for HFT

Source:
https://lucasbardella.com/coding/2025/rust-for-hft

Notes captured:

- Rust is attractive for HFT because it combines low-level performance with memory safety and no garbage collector.
- Memory operations and I/O should be minimized on the critical path.
- Multi-threading is common, but thread communication is not free.
- SPSC queues are a practical pattern for thread-to-thread handoff.
- Ring buffers are a common implementation strategy for those queues.
- CPU pinning can reduce jitter by preserving cache affinity.

## Understanding Perpetual Futures

Source:
https://www.investopedia.com/what-are-perpetual-futures-7494870

Article date:
Updated September 20, 2025

Notes captured:

- Perpetual futures have no expiry and are held indefinitely.
- Funding keeps perpetual prices near spot.
- Positive funding typically means longs pay shorts.
- Negative funding typically means shorts pay longs.
- Many exchanges apply funding every eight hours, but formulas vary by venue.
- Main use cases include leverage, hedging, and arbitrage.

## Rust DSA Ring Buffer

Source:
https://metame.substack.com/p/rust-dsa-ring-buffer

Status:
The page was not directly retrievable in the browser tool during this update.

Conservative note added:

- The vault was updated only with ring-buffer concepts consistent with the page title and with the accessible HFT source above.
- If you want, I can revisit this note later and expand it once we have the full article text.

Related:

- [[02 - Rust for HFT]]
- [[03 - Perpetuals Trading]]
- [[24 - Queues, Ring Buffers, and Backpressure]]

## Inter Core Communication Pt 1: Seqlock

Source:
https://louisponet.github.io/blog/posts/icc-1-seqlock/

Notes captured:

- Seqlocks favor producers over consumers and let readers retry instead of taking locks.
- Memory barriers are necessary for correctness because compiler and CPU reordering can break the read/write sequence.
- The pattern is useful when readers want a consistent latest snapshot, not a full event history.
- Cache-line alignment and careful measurement matter for low-jitter behavior.

## Inter Core Communication Pt 2: Queues and SeqLock Vectors

Source:
https://louisponet.github.io/blog/posts/icc-2-queues-vectors/

Notes captured:

- Low-latency queues can be built from seqlocked slots arranged as a ring buffer.
- Producer isolation is a major design goal: consumers should not stall or materially affect producers.
- Broadcast-style consumers are easy to attach in this model, but slow consumers can be overtaken and lose messages.
- Shared-memory-backed vectors and queues are useful for modular multi-process or multi-core designs.

## Automatic Message Tracking and Timing

Source:
https://louisponet.github.io/blog/posts/message-tracking/

Notes captured:

- Low-contention queues make it practical to attach telemetry without materially impacting the main system.
- Wrapping messages with timing metadata enables propagation-latency, processing-time, and lineage tracking.
- `rdtscp`-style local timestamps are useful within one machine; cross-machine tracking needs stronger identifiers and timestamps.
- Central message spines and adapters can preserve clean business logic while automatically attaching timing data.

## Fast Logging for HFT In Rust

Source:
https://markrbest.github.io/fast-logging-in-rust/

Notes captured:

- Synchronous logging is too expensive on the hot path because of blocking I/O.
- Even asynchronous logging can remain expensive if the strategy thread still formats strings and allocates.
- A better design is to hand off lightweight work to another thread and keep the strategy thread focused on trading logic.

Related:

- [[23 - Seqlocks Deep Dive]]
- [[25 - Logging and Telemetry Deep Dive]]

## Developing High-Frequency Trading Systems

Source:
Sebastien Donadio, Sourav Ghosh, and Romain Rossier, *Developing High-Frequency Trading Systems*, Packt, 2022.

Book structure noted:

- HFT strategy overview and exchange basics
- trading-system architecture and OMS/gateway structure
- exchange matching and order-book dynamics
- hardware, OS, memory, and networking foundations
- optimization topics such as context switches, lock-free structures, pre-allocation, kernel bypass, logging, and measurement
- implementation perspectives in C++, Java, Python, FPGA, and crypto contexts

Notes captured:

- The book reinforces that HFT is a whole-system problem, not just a model or strategy problem.
- It provides a useful critical-path decomposition: gateways, book builder, strategy, order manager/execution, and risk.
- It emphasizes that NUMA layout, NIC locality, memory hierarchy, and OS scheduling all affect latency in ways application developers need to understand.
- It treats networking and time synchronization as first-class design concerns rather than invisible infrastructure.
- It frames logging and live statistics as necessary operational machinery that must be designed to avoid harming the hot path.
- It recommends measurement-led optimization and attention to tail latency, not just average speed.
- It treats Python as primarily a research and analytics tool, with lower-latency production work delegated to compiled components.
- It presents FPGA and specialized transport choices as advanced optimizations, not replacements for basic architectural discipline.

Related:

- [[14 - Low-Latency Systems Foundations]]
- [[26 - Building a Low-Latency Trading Engine]]
- [[27 - Exchange Protocols and Connectivity]]
- [[92 - Developing High-Frequency Trading Systems (Full Research)]]

## Kernel Bypass Technologies

Source:
https://www.damonyuan.com/tech/260203-kernel-bypass-technologies

Primary references checked:

- DPDK Programmer's Guide: https://doc.dpdk.org/guides/prog_guide/
- DPDK Poll Mode Driver guide: https://doc.dpdk.org/guides-23.11/prog_guide/poll_mode_drv.html
- OpenOnload repository: https://github.com/Xilinx-CNS/onload
- NVIDIA VMA repository: https://github.com/Mellanox/libvma
- NVIDIA XLIO repository: https://github.com/Mellanox/libxlio
- Linux AF_XDP documentation: https://docs.kernel.org/networking/af_xdp.html
- PF_RING ZC documentation: https://www.ntop.org/guides/pf_ring/zc.html
- netmap paper: https://www.usenix.org/system/files/conference/atc12/atc12-final186.pdf
- mTCP paper: https://www.usenix.org/system/files/conference/nsdi14/nsdi14-paper-jeong.pdf
- F-Stack: https://www.f-stack.org/

Notes captured:

- Kernel bypass reduces system calls, context switches, interrupts, kernel-buffer copies, and generic-stack work by moving packet processing closer to user space and NIC queues.
- DPDK is a full user-space data-plane toolkit built around EAL setup, poll mode drivers, huge-page-backed packet buffers, mempools, rings, and burst-oriented packet I/O.
- DPDK alone does not provide a normal TCP/IP stack; TCP-oriented applications need a user-space stack, a different API, or a socket-bypass product.
- OpenOnload, VMA, and XLIO preserve a socket-style programming model for supported flows, commonly making them more practical for existing trading gateways than a full DPDK rewrite.
- AF_XDP is a Linux-integrated high-performance packet path and should be understood separately from full DPDK-style NIC takeover.
- Kernel bypass is valuable only when measurement shows the kernel or packet path is the real bottleneck; otherwise protocol correctness, queueing, parsing, risk checks, and observability may dominate.

Related:

- [[14 - Low-Latency Systems Foundations]]
- [[27 - Exchange Protocols and Connectivity]]
- [[31 - Market Data Ingestion Deep Dive]]
- [[37 - Kernel Bypass Technologies Deep Dive]]

## Liquidity Programs and CLMM Incentives

Primary references checked:

- Uniswap concentrated liquidity docs: https://developers.uniswap.org/docs/get-started/concepts/liquidity-providers/concentrated-liquidity
- Uniswap v3 whitepaper: https://app.uniswap.org/whitepaper-v3.pdf
- Uniswap v3 liquidity mining docs: https://developers.uniswap.org/docs/protocols/v3/concepts/liquidity-mining
- Uniswap v3 staker repository: https://github.com/Uniswap/v3-staker
- Raydium CLMM docs: https://docs.raydium.io/products/clmm
- Orca Whirlpools repository: https://github.com/orca-so/whirlpools
- Curve gauges and incentives docs: https://docs.curve.finance/protocol/gauge/overview
- Curve gauge weights docs: https://docs.curve.finance/user/dao/gauge-weights
- Balancer gauges docs: https://docs-v2.balancer.fi/reference/vebal-and-gauges/gauges.html
- Aerodrome docs: https://aerodrome.finance/docs
- Merkl concentrated liquidity campaigns: https://docs.merkl.xyz/merkl-mechanisms/campaign-types/concentrated-liquidity-mechanisms
- Coinbase Exchange liquidity program: https://www.coinbase.com/exchange/liquidity-program
- Coinbase International Exchange liquidity program: https://help.coinbase.com/en/international-exchange/trading-deposits-withdrawals/international-exchange-liquidity-program
- Binance market maker fee pages: https://www.binance.com/en/fee/spotMaker
- dYdX liquidity provider rewards overview: https://www.dydx.foundation/blog/liquidity-provider-rewards
- Deribit Market Maker Protection docs: https://support.deribit.com/hc/en-us/articles/25944738804509-Deribit-MMP

Notes captured:

- Liquidity programs should be evaluated by market quality, not only headline TVL, headline APR, or total volume.
- CLOB programs commonly use maker rebates, fee tiers, designated market-maker obligations, spread/depth/uptime scoring, and infrastructure benefits.
- AMM programs commonly use swap fees, emissions, gauges, vote incentives, points, and treasury-owned liquidity.
- CLMM positions are individualized by range, making incentives harder than fungible LP-token rewards because active in-range liquidity matters more than raw deposits.
- CLMM rewards can target active liquidity, oracle-centered ranges, wider depth bands, time-weighted participation, or fee-generating liquidity, but every formula creates a gameable strategy surface.
- Gauge and vote-incentive systems decentralize emission allocation but introduce governance-market dynamics where protocols may compete for votes rather than directly paying LPs.

Related:

- [[04 - Market Microstructure]]
- [[17 - Crypto Exchange Reality]]
- [[28 - Market Making Deep Dive]]
- [[38 - Liquidity Programs and CLMM Incentives]]

## Matching Engine Deployment, FPGA, DPDK, and Cloud Colocation

Primary references checked:

- AWS and One Trading cloud-native colocation blog: https://aws.amazon.com/blogs/industries/one-trading-exchange-and-aws-cloud-native-colocation-for-crypto-trading/
- AWS EC2 placement groups: https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/placement-groups.html
- AWS EC2 placement strategies: https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/placement-strategies.html
- AWS enhanced networking: https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/enhanced-networking.html
- AWS ENA Express: https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ena-express.html
- AWS Elastic Fabric Adapter: https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/efa.html
- Amazon ENA Linux driver best practices: https://github.com/amzn/amzn-drivers/blob/master/kernel/linux/ena/ENA_Linux_Best_Practices.rst
- DPDK Programmer's Guide: https://doc.dpdk.org/guides/prog_guide/
- AMD Alveo UL3422 accelerator: https://www.amd.com/en/products/accelerators/alveo/ul3422.html
- AMD Alveo UL3524 product brief: https://www.xilinx.com/content/dam/xilinx/publications/product-briefs/2233051_Product_Brief_UL3524_Alveo_Accelerator_Card.pdf
- Cisco Nexus SmartNIC: https://www.cisco.com/c/en/us/products/interfaces-modules/nexus-smartnic/index.html

Notes captured:

- The AWS/One Trading article frames cloud-native colocation as a topology problem: shared EC2 cluster placement groups plus VPC peering offered the lowest-latency tested access tier, followed by plain VPC peering, PrivateLink, and public internet/CloudFront.
- The AWS test focused on a single Availability Zone for latency measurement; resilient multi-AZ designs require separate replication and failover tradeoffs.
- The HFT client test flow measured order send, acknowledgement, cancel send, and cancel acknowledgement, with low-rate and high-rate message scenarios.
- The AWS baseline used application optimizations such as core pinning, thread segregation, composite buffers, io_uring, and JVM tuning, while explicitly leaving further optimizations such as IRQ handling, kernel bypass, RSS, scheduler policy, and ENA tuning out of scope.
- For matching-engine design, DPDK belongs at the gateway and market-data publisher boundary before it belongs inside matching semantics.
- FPGA and SmartNIC acceleration are best treated as narrow offloads first: timestamping, feed decode, packet filtering, simple risk checks, fixed-format order encoding, or market-data fanout.
- Rust remains a strong fit for the deterministic matching partition because it keeps ownership, allocation, and concurrency decisions explicit while preserving replayable software semantics.

Related:

- [[14 - Low-Latency Systems Foundations]]
- [[37 - Kernel Bypass Technologies Deep Dive]]
- [[39 - Matching Engine Deployment, FPGA, DPDK, and Cloud Colocation]]
- [[65 - HFT Rust System Design Master Note]]
