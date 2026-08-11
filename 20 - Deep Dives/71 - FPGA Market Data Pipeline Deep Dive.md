---
title: FPGA Market Data Pipeline Deep Dive
tags: [fpga, market-data, rtl, hls, hardware-acceleration, hft, deep-dive]
---

# FPGA Market Data Pipeline Deep Dive

## Why this workload fits

Market-data feeds are structured streams. The work often has a stable sequence: receive frame, validate, parse fields, decode a message, update a compact state machine, and emit an event. FPGAs can pipeline these stages so that a new message enters every cycle even while earlier messages are still moving through later stages.

This is different from saying that an FPGA is always faster. The advantage is deterministic dataflow, parallelism, and low jitter for a fixed function. GPUs are usually better for large dense numerical workloads and flexible batch computation.

## Pipeline shape

```text
Ethernet MAC
  -> frame validation
  -> IP/UDP or TCP framing
  -> feed sequence check
  -> FAST / binary protocol decode
  -> symbol/instrument lookup
  -> order-book delta update
  -> derived event / trigger
  -> timestamped output
```

Each stage needs a contract: input width, output width, latency, throughput, reset behavior, malformed-message behavior, and whether it can backpressure the previous stage.

## State and memory

An FPGA pipeline is easiest when hot state is bounded and local. An order book with arbitrary dynamic allocation is harder than a fixed-depth book or a price-level table with explicit resource limits. The design must make overflow visible instead of silently dropping or corrupting state.

Common techniques include:

- BRAM/URAM for compact local state;
- on-chip FIFOs between stages;
- fixed-width integers and fixed-point arithmetic;
- parallel lookup banks;
- deterministic arbitration for simultaneous events;
- host memory only for slower or larger state;
- explicit sequence and gap tracking.

## Verification ladder

1. Define a software reference decoder/book builder.
2. Generate vectors from real and adversarial feed captures.
3. Compare every emitted event and state transition against the reference.
4. Run simulation with malformed frames, sequence gaps, duplicates, resets, and bursts.
5. Run synthesis and inspect timing closure and resource use.
6. Test hardware-in-the-loop with timestamped packets.
7. Shadow the FPGA beside the CPU path before making it authoritative.

The shadow phase is valuable because hardware can be fast and wrong. Compare not only final book state but every intermediate normalized message, sequence decision, and timestamp.

## FPGA, SmartNIC, and CPU boundary

```text
FPGA / SmartNIC:
  framing, parsing, timestamping, filtering, fixed risk checks

CPU / Rust:
  product rules, complex order semantics, replay, persistence, recovery
```

Move a function to hardware when its interface is stable, its latency budget matters, and its correctness can be specified. Keep it in software when the rule changes frequently or has difficult cross-instrument state.

## Production failure modes

- feed sequence gap or recovery ambiguity;
- stale configuration loaded into the bitstream;
- timing closure regression after a small logic change;
- reset leaves state partially initialized;
- host/FPGA version mismatch;
- output timestamp sampled at the wrong pipeline stage;
- packet loss in the preceding NIC or switch path;
- hardware path and software shadow disagree under a rare message combination.

The operational package must include the bitstream hash, source revision, register map, configuration snapshot, test-vector version, and rollback image.

## References

- [An Accelerator for Decoding Market Data Based on FPGA](https://doi.org/10.1142/S0218126619500506)
- [High Frequency Trading Acceleration Using FPGAs](https://doi.org/10.1109/FPL.2011.64)
- [Acceleration of Trading System Back End with FPGAs Using HLS](https://www.mdpi.com/2079-9292/12/3/520)
- [AMD/Xilinx Alveo documentation](https://www.amd.com/en/products/accelerators/alveo.html)

Related:

- [[39 - Matching Engine Deployment, FPGA, DPDK, and Cloud Colocation]]
- [[72 - Production Low-Latency Trading System Construction]]
