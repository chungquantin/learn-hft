---
title: Hardware Timestamping and PTP Deep Dive
tags: [ptp, ieee-1588, timestamping, clocks, hardware, hft, production]
---

# Hardware Timestamping and PTP Deep Dive

## Four different times

An HFT system should not collapse these into one timestamp:

1. **Exchange event time**: when the venue says the event happened.
2. **Wire time**: when the packet crossed a NIC or switch timestamp point.
3. **Monotonic process time**: local elapsed-time measurement for latency.
4. **Wall/PTP time**: a cross-host time scale for correlation and audit.

They answer different questions. Exchange time helps reconstruct market state. Wire time measures transport. Monotonic time measures local duration. PTP time correlates events between machines.

## PTP architecture

```text
GNSS / grandmaster
        |
  boundary or transparent clocks
        |
switch / NIC PHC
        |
ptp4l -> PHC
        |
phc2sys -> CLOCK_REALTIME / application clock
```

The NIC's PTP hardware clock, or PHC, is a hardware clock associated with the interface. `ptp4l` synchronizes PTP ports; `phc2sys` synchronizes one clock to another, commonly the system clock to the PHC.

## Hardware timestamp path

On receive, the NIC timestamps near the MAC/PHY boundary and attaches the timestamp to packet metadata. On transmit, the timestamp may become available only after the packet leaves the MAC, so the driver must correlate it with the original packet, often through an error queue or hardware FIFO.

Linux exposes hardware timestamping through `SO_TIMESTAMPING`, PHC APIs, and driver-specific support. DPDK exposes NIC timesync APIs when the PMD supports IEEE 1588/802.1AS.

## One-step and two-step PTP

- **One-step** inserts or records timing information during transmission when hardware can do so.
- **Two-step** sends a follow-up message carrying the measured timestamp.

Two-step designs require correct correlation of sequence ID, message type, domain, port, and timestamp FIFO entries. A timestamp can be numerically precise and still be associated with the wrong packet if correlation fails.

## What PTP improves in HFT

- feed-handler ingress attribution;
- venue-to-venue latency comparison;
- order decision and wire-egress measurement;
- switch-path and cross-connect diagnostics;
- distributed event correlation;
- compliance and audit evidence;
- replay alignment across hosts.

PTP does not establish business ordering. Use feed sequence numbers and deterministic event sequencing for that. PTP is a clock discipline and measurement system.

## Clock failure policy

Define behavior for:

- grandmaster loss;
- PHC becoming unsynchronized;
- offset threshold breach;
- frequency correction instability;
- leap-second and UTC/PTP scale mismatch;
- NIC reset;
- timestamp FIFO overflow;
- one interface synchronized and another unsynchronized.

Continue monotonic local measurements when PTP is degraded, but mark cross-host and regulatory timestamps invalid or degraded. Do not silently present stale synchronized time as trustworthy.

## Validation

Validate the full path, not only daemon status:

1. Inspect PHC association and port state.
2. Measure offset and frequency statistics.
3. Compare NIC RX/TX timestamps with a known reference.
4. Inject packets through the exact switch and cross-connect path.
5. Compare two NICs and two hosts.
6. Pull the grandmaster or disable the link and observe failover.
7. Correlate timestamp IDs through the application pipeline.

Use separate fields for raw hardware timestamp, converted PTP timestamp, exchange timestamp, and monotonic capture time.

## HFT latency schema

```text
wire_rx_ns
nic_rx_phc_ns
decoder_done_mono_ns
book_update_mono_ns
decision_mono_ns
risk_done_mono_ns
nic_tx_phc_ns
wire_tx_ns
exchange_event_ns
ptp_health
```

From this, compute local stage durations using monotonic time and cross-host attribution using PTP time. Do not subtract clocks from different domains without recording the conversion and health state.

## References

- [IEEE 1588 Working Group](https://sagroups.ieee.org/1588/news/ieee-1588-2019-evolves-to-better-serve-its-wide-variety-of-applications/)
- [Linux PTP hardware clock infrastructure](https://docs.kernel.org/next/driver-api/ptp.html)
- [Linux kernel timestamping API](https://docs.kernel.org/6.6/networking/timestamping.html)
- [linuxptp ptp4l](https://www.linuxptp.org/documentation/ptp4l/)
- [linuxptp phc2sys](https://www.linuxptp.org/documentation/phc2sys/)
- [DPDK timesync features](https://doc.dpdk.org/guides/nics/features.html)

Related:

- [[18 - Time and Timestamp Semantics]]
- [[15 - Benchmarking and Tick-to-Trade Measurement]]
- [[25 - Logging and Telemetry Deep Dive]]
- [[72 - Production Low-Latency Trading System Construction]]
