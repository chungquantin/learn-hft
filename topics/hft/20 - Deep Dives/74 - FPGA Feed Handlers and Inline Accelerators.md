---
title: FPGA Feed Handlers and Inline Accelerators
tags: [fpga, feed-handler, inline-accelerator, alveo, rtl, hft, production]
---

# FPGA Feed Handlers and Inline Accelerators

## Where the FPGA belongs

An FPGA is most valuable beside the wire, where the work is fixed-format, streaming, parallel, and latency-sensitive. It should not be used as a vague “faster CPU.” The engineering question is which function has a stable interface and enough value to justify a hardware lifecycle.

Good candidates:

- Ethernet/IP/UDP framing;
- exchange feed decoding;
- sequence and gap detection;
- timestamp capture;
- symbol filtering and fanout;
- simple price-band or size-limit checks;
- order-entry encoding;
- packet capture and mirroring;
- deterministic trigger generation.

Poor first candidates:

- rapidly changing product logic;
- complex cross-instrument risk;
- full exchange semantics with frequent rule changes;
- persistence and recovery orchestration;
- anything without a software oracle.

## Feed-handler pipeline

```text
MAC -> Ethernet -> IP/UDP -> venue header -> message decoder
    -> sequence validator -> normalized event -> CPU/strategy
```

Each pipeline stage should have an explicit latency and throughput contract. The design should say what happens when a message is malformed, a sequence number skips, a FIFO fills, or a reset occurs.

## FPGA memory choices

- **Registers**: tiny, very fast control/state fields.
- **BRAM/URAM**: local tables, FIFOs, book levels, and lookup state.
- **DDR/HBM**: larger state, usually with more variable access cost and arbitration.
- **Host memory over PCIe**: flexible but adds protocol and transfer latency.

For a market-data parser, keep the critical metadata and lookup path on chip where possible. Avoid turning every message into a host round trip.

## Inline versus look-aside

An inline accelerator sits directly in the packet path:

```text
wire -> FPGA parse/filter/risk -> host NIC or order port -> wire
```

A look-aside accelerator receives work from the CPU and returns a result:

```text
CPU -> PCIe request -> FPGA -> PCIe response -> CPU
```

Inline avoids PCIe round trips but can make failure and bypass behavior harder. Look-aside is easier to introduce and shadow, but the request/response path must be faster than the work it replaces.

## Alveo and custom RTL

Alveo-class cards provide a general development platform for programmable acceleration. Custom RTL can achieve tighter control over framing, pipeline depth, and timing, while HLS can improve iteration speed for suitable kernels. The choice is a lifecycle decision:

```text
HLS / reference implementation -> RTL refinement -> timing closure
                                -> hardware test -> production bitstream
```

Use custom RTL when cycle-level behavior and resource control justify the additional verification burden. Use HLS when the algorithm maps cleanly to a pipeline and iteration speed is more valuable than maximum hand-tuned control.

## Verification strategy

The hardware implementation needs a byte-for-byte or event-for-event reference model. Test with:

- real feed captures;
- maximum and minimum message sizes;
- burst traffic;
- duplicates and out-of-order packets;
- sequence gaps and recovery;
- malformed length/checksum fields;
- symbol configuration changes;
- reset in every pipeline stage;
- timestamp wrap and clock adjustment;
- full and empty FIFO boundaries.

Run the FPGA in shadow mode beside the CPU feed handler. Compare normalized events, sequence decisions, book deltas, and timestamps before allowing hardware output to drive a strategy.

## Inline risk

Inline FPGA risk should be limited to rules with clear local inputs and bounded state:

- maximum order size;
- price collars;
- static instrument enablement;
- session-level rate limit;
- kill-switch state;
- duplicate client sequence check.

Complex risk depends on account, inventory, collateral, and cross-venue state. That state belongs in a carefully designed CPU service or colocated risk partition unless the hardware contract is exceptionally stable.

## Deployment artifact

Version and deploy together:

- bitstream hash;
- RTL/HLS source revision;
- register map;
- FPGA shell/platform version;
- host driver and firmware;
- symbol and venue configuration;
- test-vector corpus;
- software reference version;
- rollback image.

An FPGA release without its matching host and configuration is not a complete release.

## References

- [An Accelerator for Decoding Market Data Based on FPGA](https://doi.org/10.1142/S0218126619500506)
- [High Frequency Trading Acceleration Using FPGAs](https://doi.org/10.1109/FPL.2011.64)
- [Acceleration of Trading System Back End with FPGAs Using HLS](https://www.mdpi.com/2079-9292/12/3/520)
- [AMD Alveo SN1000 SmartNIC data sheet](https://docs.amd.com/api/khub/documents/rllO00cN~P_4HGlYrQWB6w/content)

Related:

- [[71 - FPGA Market Data Pipeline Deep Dive]]
- [[70 - Solarflare Onload and ef_vi Deep Dive]]
- [[72 - Production Low-Latency Trading System Construction]]
