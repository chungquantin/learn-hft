---
title: Solarflare Onload and ef_vi Deep Dive
tags: [solarflare, onload, ef-vi, kernel-bypass, networking, hft, deep-dive]
---

# Solarflare Onload and ef_vi Deep Dive

## Two APIs, two tradeoffs

Solarflare Onload is an accelerated user-level TCP/UDP stack that preserves a POSIX socket programming model for supported traffic. `ef_vi` is a lower-level layer-2 API that gives an application direct access to the Solarflare adapter datapath and raw Ethernet frames.

```text
POSIX sockets -> Onload user-level TCP/UDP stack -> NIC
raw Ethernet  -> ef_vi queues and buffers      -> NIC
```

Onload is usually the migration-friendly option. ef_vi provides more control and lower per-message overhead, but the application must implement higher-layer protocols itself. AMD documentation explicitly describes a mixed deployment: ef_vi for UDP market data and Onload sockets for TCP order connections.

## Onload path

An application can start with ordinary socket code and run it with Onload enabled. Supported traffic is handled by the user-level stack and hardware queues; unsupported operations can fall back to the kernel path. The exact behavior depends on adapter, firmware, driver, protocol, VLAN, multicast, and configuration.

Never assume that `LD_PRELOAD` means every packet is accelerated. Verify the path using Onload diagnostics, packet counters, stack state, and hardware timestamps.

## ef_vi path

ef_vi exposes a layer-2 data path with direct access to hardware buffers. The typical responsibilities are:

- allocate and register packet buffers;
- post receive buffers;
- configure filters and queues;
- poll event queues;
- inspect RX/TX events;
- parse Ethernet/IP/UDP or a venue-specific protocol;
- recycle buffers after processing;
- handle link, error, and resource events.

The application owns more of the protocol and memory lifecycle than it does with Onload. That is why ef_vi belongs in a narrowly scoped feed handler or gateway, not casually throughout the system.

## HFT deployment pattern

```text
Solarflare NIC
  |-- ef_vi RX queue -> UDP multicast feed handler -> book builder
  |-- Onload TCP socket -> order gateway -> exchange session
  |-- kernel path -> SSH, monitoring, control plane
```

Separate control-plane and data-plane traffic. Keep management access on a kernel-managed interface or a carefully isolated queue. Avoid mixing accelerated and non-accelerated interfaces in the same broadcast domain without understanding the documented ARP and reset hazards.

## Buffer sizing

Packet buffers are not sized only by payload bytes. A 200-byte UDP datagram may consume a larger fixed buffer, and receive queues, socket queues, multicast fanout, and multiple Onload stacks all compete for the pool. Size from a traffic model:

```text
required buffers >= burst rate
                    * worst-case processing stall
                    * fanout / queueing multiplier
                    + safety margin
```

Then validate with `onload_stackdump`, NIC counters, and deliberate burst tests. Buffer exhaustion is a correctness event for market data: it can become packet loss, sequence gaps, and an invalid book.

## Onload versus DPDK

| Dimension | Onload | ef_vi | DPDK |
| --- | --- | --- | --- |
| Programming model | POSIX sockets | Raw layer-2 API | User-space packet toolkit |
| Protocol ownership | Onload stack | Application | Application or added stack |
| Migration effort | Lower | High | High |
| Control | Medium | High | High |
| Best fit | TCP/UDP applications | Specialist feed/gateway | General packet data plane |
| Hardware coupling | Solarflare/AMD NICs | Solarflare/AMD NICs | PMD/NIC dependent |

## Production checks

- Confirm which sockets and flows are accelerated.
- Confirm fallback behavior for unsupported options.
- Test multicast joins, loss, reordering, and recovery.
- Test TCP reconnects, half-close, retransmission, and exchange rejects.
- Test VLANs and multiple interfaces separately.
- Record firmware, driver, Onload release, kernel, BIOS, and NIC settings.
- Capture hardware timestamps before and after enabling Onload.
- Keep an ordinary-kernel fallback build or host available for incident response.

## References

- [AMD/Xilinx: Onload User Guide](https://www.xilinx.com/content/dam/xilinx/publications/solarflare/onload/enterprise-onload/SF-104474-CD-34_Onload_User_Guide.pdf)
- [AMD: Solarflare Onload](https://www.amd.com/en/products/ethernet-adapters/onload.html)
- [AMD: ef_vi User Guide](https://docs.amd.com/r/en-US/ug1586-onload-user/ef_vi?contentId=BOFxy7Gif8zBb26BhDBz_g)
- [AMD: packet buffer requirements](https://docs.amd.com/r/en-US/ug1586-onload-user/Identifying-Packet-Buffer-Requirements)
- [AMD: hardware multicast loopback](https://docs.amd.com/r/en-US/ug1586-onload-user/Hardware-Multicast-Loopback)
- [AMD: mixed adapter broadcast-domain considerations](https://docs.amd.com/r/en-US/ug1586-onload-user/Mixed-Adapters-Sharing-a-Broadcast-Domain)

Related:

- [[37 - Kernel Bypass Technologies Deep Dive]]
- [[72 - Production Low-Latency Trading System Construction]]
