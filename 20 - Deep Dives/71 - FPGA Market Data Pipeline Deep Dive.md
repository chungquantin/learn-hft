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

## FPGA development boards and tooling

Production FPGA market-data work typically targets Alveo-class or SmartNIC cards, but learning and prototyping often starts on smaller hardware. Relevant entry points:

- **MAX1000** (~$30, Intel MAX10, 2K–16K logic elements, 8MB SDRAM, 12 MHz + PLL to 100 MHz, ADC, UART, SPI, I2C). Cheapest path to hands-on FPGA experience.
- **CYC1000** (~$30, Intel Cyclone 10 LP, 25K logic elements, 8MB SDRAM, 8MB flash). More capacity for larger pipeline prototypes.
- **Arty A7-100T** (~$299, AMD Artix-7, 101K logic cells, 240 DSP slices, Ethernet). Closer to production-class work.
- **Basys 3** (~$150, AMD Artix-7, 16 user switches, 7-segment display). Common in university FPGA courses.
- **PYNQ-Z2** (~$130, Zynq SoC, ARM Cortex-A9 + FPGA fabric). Useful for hybrid CPU/FPGA pipeline prototyping.

### HDL choices for pipeline development

- **VHDL**: strongly typed, verbose. Natural for pipeline stage contracts where signal widths and types are enforced at compile time. Dominant in European firms and defense.
- **Verilog/SystemVerilog**: C-like syntax, less ceremony. SystemVerilog adds assertions and constrained-random verification. Dominant in US trading firms and ASIC work.
- **HLS (C/C++ via Vivado/Vitis)**: describes algorithms in C/C++, compiler generates RTL. Faster iteration for pipeline stages that map cleanly to loops and arrays. Less control over cycle-level timing and resource placement than hand-written RTL.
- **VHDPlus**: simplified VHDL superset with IDE, simulator, package manager, and NIOS II soft-processor support. Good for rapid prototyping before moving to Quartus Prime, Vivado, or Vitis for production work.
- **Python (Amaranth/PYNQ)**: high-level synthesis or SoC control. Research and prototyping, not production pipeline RTL.

For a market-data pipeline, the practical path is often: prototype stage logic in HLS or VHDPlus, refine critical stages to hand-written RTL for timing closure, then integrate into the production shell (Alveo platform, PCIe DMA, host driver).

### Soft processors in hybrid designs

Intel NIOS II and AMD MicroBlaze are soft processors instantiated in FPGA fabric. They handle slow-path tasks: configuration loading, symbol table management, statistics reporting, and health monitoring. The hard pipeline handles wire-speed parsing and filtering; the soft processor handles management without consuming host CPU cycles. Zynq and Versal devices integrate hard ARM cores alongside FPGA fabric, eliminating the soft-processor resource overhead for control-plane tasks.

### FPGA interface protocols for market data

Pipeline stages communicate with the host and network through standard interfaces:

- **AXI4-Stream**: point-to-point streaming interface. Natural fit for pipeline stages. Each stage produces/consumes a stream with valid/ready handshaking. Backpressure propagates upstream.
- **AXI4-Lite**: register-mapped control interface. Used for configuration, status registers, and slow-path communication between host driver and FPGA logic.
- **PCIe DMA**: bulk data transfer between FPGA and host memory. Used for normalized event delivery to CPU strategies. Adds transfer latency compared to inline processing.
- **10/25/100GbE MAC**: network-facing interface. Receives raw Ethernet frames from the exchange feed. The pipeline begins at MAC output.

Three FPGA configuration types matter for deployment:

- **SRAM-based**: volatile, must reload on power-up, standard for prototyping and most production cards.
- **Flash-based**: non-volatile, instant-on at power-up, relevant for embedded or always-on deployments.
- **Anti-fuse**: one-time programmable, used in defense/aerospace where reconfiguration is a liability.

## References

- [An Accelerator for Decoding Market Data Based on FPGA](https://doi.org/10.1142/S0218126619500506)
- [High Frequency Trading Acceleration Using FPGAs](https://doi.org/10.1109/FPL.2011.64)
- [Acceleration of Trading System Back End with FPGAs Using HLS](https://www.mdpi.com/2079-9292/12/3/520)
- [AMD/Xilinx Alveo documentation](https://www.amd.com/en/products/accelerators/alveo.html)
- [VHDPlus FPGA components and IDE](https://vhdplus.com/docs/components/overview/)
- Dogan Ibrahim, *FPGA Programming and Hardware Essentials*, Elektor, 2024 (ISBN 978-3-89576-644-2)

Related:

- [[39 - Matching Engine Deployment, FPGA, DPDK, and Cloud Colocation]]
- [[72 - Production Low-Latency Trading System Construction]]
- [[74 - FPGA Feed Handlers and Inline Accelerators]]
