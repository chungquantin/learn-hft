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

## FPGA resource mapping for pipeline stages

Each pipeline stage consumes specific FPGA resources. Understanding the mapping helps estimate whether a design fits a target device:

| Pipeline Stage | Primary Resources | Typical Depth | Notes |
|---------------|-------------------|---------------|-------|
| Ethernet MAC | I/O blocks, transceivers | Fixed (vendor IP) | 10/25/100GbE MAC typically provided as hard IP or vendor-licensed soft IP |
| Frame validation | LUTs, FFs | 1–2 cycles | CRC check uses DSP slices or LUT-based XOR trees |
| IP/UDP framing | LUTs, FFs | 2–4 cycles | Header field extraction, checksum validation |
| Sequence check | LUTs, FFs, BRAM | 1–2 cycles | Per-feed expected sequence number stored in BRAM |
| Protocol decode | LUTs, FFs | 3–10 cycles | FAST template decode is wider than fixed-binary; field-length parsing adds cycles |
| Symbol lookup | BRAM, CAM (LUT-based) | 1–3 cycles | Hash-based or direct-indexed. CAM for small symbol sets (<256) |
| Book delta update | BRAM/URAM, LUTs | 2–5 cycles | Fixed-depth price-level array. Read-modify-write with forwarding |
| Timestamp capture | FFs, dedicated I/O | 1 cycle | Sample free-running counter at pipeline entry, not exit |
| Output formatting | LUTs, FFs | 1–3 cycles | Pack normalized event into host-bound descriptor |

A simple UDP feed parser (MAC → IP/UDP → decode → filter → timestamp → output) on a Cyclone 10 LP (25K logic elements) or Artix-7 can comfortably fit a single-feed pipeline. Multi-feed designs or full book state require larger devices (Kintex, Stratix, or Alveo-class).

### Clock domains and crossing

Market-data pipelines typically involve multiple clock domains:

- **MAC receive clock**: recovered from the incoming Ethernet signal (125 MHz for 1GbE, 156.25 MHz for 10GbE)
- **Pipeline clock**: internal processing clock, often 200–300 MHz for adequate throughput
- **PCIe clock**: 250 MHz (Gen3) or 500 MHz (Gen4) for host communication
- **Host interface clock**: AXI4-Lite register access, typically 100–250 MHz

Clock domain crossings (CDCs) require explicit synchronization: dual-clock FIFOs, gray-code pointers, or handshake protocols. Every CDC is a potential source of metastability bugs. Formal verification tools (CDC checkers in Vivado, Quartus) should flag all crossings.

### Latency accounting

Measure pipeline latency from MAC start-of-frame to normalized-event valid signal:

```text
MAC receive:           ~8 ns (64-bit bus at 125 MHz)
Frame validation:      ~4–8 ns (1–2 pipeline stages)
IP/UDP parse:          ~8–16 ns
Feed header + decode:  ~12–40 ns (protocol-dependent)
Symbol lookup:         ~4–12 ns
Timestamp:             ~4 ns
Output format:         ~4–12 ns
─────────────────────────────────────
Total wire-to-event:   ~44–96 ns typical for simple binary feed
```

Compare to CPU path: kernel NIC driver + interrupt + kernel stack + user-space copy + software parser often adds 5–50 µs. Kernel-bypass (DPDK/ef_vi) reduces this to 1–5 µs. FPGA inline processing targets sub-100 ns.

## FPGA development boards and tooling

Production FPGA market-data work typically targets Alveo-class or SmartNIC cards, but learning and prototyping often starts on smaller hardware.

### Learning path: entry-level to production

```text
Stage 1: Learn HDL fundamentals (MAX1000 / CYC1000, ~$30)
  - LED blink, counters, shift registers, UART
  - Understand signals vs variables, clock edges, simulation
  - VHDPlus or Quartus Prime Lite

Stage 2: Build simple pipelines (CYC1000 / Arty A7, $30–$299)
  - Multi-stage pipeline with FIFOs
  - SPI/I2C/UART communication
  - ADC sampling and processing
  - Timing closure practice

Stage 3: Protocol parsing (Arty A7 / PYNQ-Z2, $130–$299)
  - Ethernet frame reception (if board has Ethernet)
  - UDP packet parsing
  - Binary message decode
  - BRAM-based lookup tables

Stage 4: Production acceleration (Alveo U50/U250, $2K–$5K)
  - 10/25GbE MAC integration
  - PCIe DMA to host
  - Full feed parser pipeline
  - Shadow mode beside CPU path
```

### Entry-level board details

**MAX1000** (~$30, Intel MAX10 10M08SAU169C8G)

8,000 logic elements, 378 Kb RAM, 2 PLLs (>300 MHz), 24× 18×18 multipliers, 12-bit ADC (1 Msps, 8 channels). 64 Mb SDRAM, 64 Mb flash. 12 MHz oscillator (PLL to 100+ MHz). 8 LEDs, 2 buttons, Arduino MKR headers, Pmod connector, UART, integrated USB programmer. Flash-based — retains configuration across power cycles (instant-on).

**CYC1000** (~$30, Intel Cyclone 10 LP 10CL025YU256C8G)

25,000 logic elements (3× MAX1000), 594 Kb RAM, 4 PLLs (>300 MHz), 66× 18×18 multipliers. 64 Mb SDRAM, 16 Mb flash. SRAM-based — must reload configuration on power-up. Same oscillator, programmer, and accelerometer as MAX1000. The 3× logic capacity and 2.75× multiplier count make it suitable for multi-stage pipeline prototypes.

**Core MAX10** (~$40, same FPGA as MAX1000)

Same 10M08SAU169C8G chip. Adds CRUVI high-speed connector (differential pairs for camera/display extensions) and CRUVI low-speed connector. Part of the VHDPlus modular ecosystem with Shield, Camera, Motor, Audio, and WiFi extensions.

### HDL choices for pipeline development

- **VHDL**: strongly typed, verbose. Natural for pipeline stage contracts where signal widths and types are enforced at compile time. Dominant in European firms and defense.
- **Verilog/SystemVerilog**: C-like syntax, less ceremony. SystemVerilog adds assertions and constrained-random verification. Dominant in US trading firms and ASIC work.
- **HLS (C/C++ via Vivado/Vitis)**: describes algorithms in C/C++, compiler generates RTL. Faster iteration for pipeline stages that map cleanly to loops and arrays. Less control over cycle-level timing and resource placement than hand-written RTL.
- **VHDPlus**: simplified VHDL superset. `Main{}` defines I/O, `Process{}` runs every clock cycle, `Thread{}` enables multi-cycle sequences with `Step{}` and `Wait{}`. Signals (`<=`) update next cycle; variables (`:=`) update instantly. Transpiles to VHDL. IDE integrates simulation (GHDL), pin assignment, package manager, and one-click download to FPGA.
- **Python (Amaranth/PYNQ)**: high-level synthesis or SoC control. Research and prototyping, not production pipeline RTL.

For a market-data pipeline, the practical path is often: prototype stage logic in HLS or VHDPlus, refine critical stages to hand-written RTL for timing closure, then integrate into the production shell (Alveo platform, PCIe DMA, host driver).

### Simulation and verification tools

| Tool | Language | License | Notes |
|------|----------|---------|-------|
| GHDL | VHDL | Open source | Integrated into VHDPlus IDE, fast compile |
| Icarus Verilog | Verilog | Open source | Lightweight, good for unit-level simulation |
| Verilator | Verilog/SV | Open source | Cycle-accurate, compiles to C++, fast for large designs |
| ModelSim/Questa | VHDL/Verilog/SV | Commercial (free Lite edition) | Industry standard, waveform viewer, coverage |
| Vivado Simulator | VHDL/Verilog/SV | Free with Vivado | Xilinx-specific, integrated timing simulation |
| Quartus Prime Lite | VHDL/Verilog | Free | Intel/Altera-specific, includes TimeQuest STA |

Simulation workflow: write testbench → generate stimulus (real feed captures or synthetic vectors) → run simulation → compare output against software reference model → inspect waveforms for timing and protocol correctness.

### Soft processors in hybrid designs

Intel NIOS II is a 32-bit soft processor consuming FPGA logic resources (LUTs, FFs, BRAM). On MAX1000, it can run as an Arduino-compatible processor for C/C++ code alongside RTL pipeline logic. AMD MicroBlaze serves the same role on Xilinx devices.

Soft processors handle slow-path tasks: configuration loading, symbol table management, statistics reporting, health monitoring, and PCIe setup. The hard pipeline handles wire-speed parsing and filtering; the soft processor handles management without consuming host CPU cycles.

Zynq (hard ARM Cortex-A9) and Versal (ARM Cortex-A72 + Cortex-R5) integrate hard processor cores alongside FPGA fabric. This eliminates the logic-resource cost of soft processors and provides full Linux capability for management, logging, and host communication.

### FPGA interface protocols for market data

Pipeline stages communicate with the host and network through standard interfaces:

- **AXI4-Stream**: point-to-point streaming interface. Natural fit for pipeline stages. Each stage produces/consumes a stream with TVALID/TREADY handshaking. Backpressure propagates upstream when a consumer deasserts TREADY. TLAST marks end of packet. TKEEP/TSTRB mark valid bytes in the final beat.
- **AXI4-Lite**: register-mapped control interface. 32-bit address, 32-bit data. Used for configuration registers, status readback, and slow-path host communication. Simple read/write protocol with AWVALID/AWREADY/WVALID/WREADY handshaking.
- **AXI4-Full (AXI4-MM)**: burst-capable memory-mapped interface. Used for DMA engine access to DDR/HBM or host memory. Supports burst lengths up to 256 beats.
- **PCIe DMA**: bulk data transfer between FPGA and host memory. Alveo cards use XDMA or QDMA IP. QDMA supports multiple queues with independent descriptors — useful for per-feed or per-symbol event delivery. Adds 1–3 µs transfer latency depending on payload size and PCIe generation.
- **10/25/100GbE MAC**: network-facing interface. Receives raw Ethernet frames from the exchange feed. The pipeline begins at MAC output. 10GbE uses 64-bit data bus at 156.25 MHz; 25GbE and 100GbE use wider buses or higher clocks.

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
