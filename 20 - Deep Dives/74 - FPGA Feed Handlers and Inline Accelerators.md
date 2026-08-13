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

## FPGA architecture fundamentals

An FPGA is a semiconductor device built around a matrix of configurable logic blocks (CLBs) connected by user-programmable interconnects. CLBs are the fundamental logic module; other modules include I/O blocks, DSP blocks, and embedded memory (BRAM/URAM). The logic blocks can implement both combinational and sequential logic functions through programmable lookup tables (LUTs), flip-flops, and carry chains.

### FPGA types

- **SRAM-based**: configuration stored in volatile SRAM cells. Must reload bitstream on every power-up. Standard for development and most production accelerator cards (Alveo, SmartNICs). Allows unlimited reprogramming.
- **Flash-based**: configuration stored in non-volatile flash. Retains configuration across power cycles. Relevant for instant-on embedded or always-on deployments where boot time matters.
- **Anti-fuse**: one-time programmable. Cannot be reconfigured. Used in defense, aerospace, and high-reliability applications where immutability and radiation hardness are requirements.

### Key FPGA vendors and families

| Vendor | Families | Notes |
|--------|----------|-------|
| AMD/Xilinx | Artix-7, Kintex, Virtex, Versal, Alveo | Alveo cards dominate HFT acceleration |
| Intel/Altera | MAX10, Cyclone, Arria, Stratix, Agilex | Stratix/Agilex for high-end, MAX10/Cyclone for learning |
| Lattice | iCE40, ECP5, Nexus | Low-power, smaller designs |
| Microchip | PolarFire, SmartFusion | Flash-based, lower power |
| Others | QuickLogic, Renesas, Flex Logix, GOWIN, Efinix | Niche or emerging players |

The FPGA market is expected to exceed $20B by 2030. For HFT, AMD/Xilinx Alveo and Intel Stratix/Agilex are the primary production platforms. Entry-level boards (MAX1000 at ~$30, CYC1000 at ~$30) are useful for learning FPGA fundamentals before committing to production hardware.

### HDL and development languages

FPGAs are programmed using hardware description languages:

- **VHDL**: verbose, strongly typed, dominant in European and defense contexts. Design at behavioral or structural level.
- **Verilog/SystemVerilog**: C-like syntax, dominant in US and ASIC-adjacent work. SystemVerilog adds verification constructs.
- **VHDPlus**: simplified VHDL superset with IDE, simulator, and package manager. Useful for rapid prototyping. Transpiles to standard VHDL.
- **HLS (C/C++)**: Vivado HLS or Vitis compiles C/C++ to RTL. Faster iteration but less cycle-level control than hand-written RTL.
- **Python**: Amaranth (formerly nMigen), MyHDL, or PYNQ for high-level synthesis and SoC interaction. Research and prototyping tool, not production RTL.

### FPGA development workflow

```text
1. Describe function in HDL (VHDL, Verilog, or HLS)
2. Simulate and verify against reference model
3. Synthesize: HDL -> netlist -> place-and-route -> timing closure
4. Generate bitstream
5. Load bitstream onto FPGA
6. Test in hardware (hardware-in-the-loop)
7. Shadow beside CPU path before production trust
```

Timing closure is a critical gate: a design that simulates correctly may fail synthesis if logic paths exceed the clock period. Small changes can cause timing regressions, requiring re-placement or pipeline restructuring.

### Soft processors

Intel FPGAs support NIOS II, a soft processor instantiated in FPGA fabric. AMD/Xilinx offers MicroBlaze. These enable hybrid designs: hard real-time pipeline logic in RTL alongside general-purpose C/C++ control for configuration, monitoring, or slow-path handling. Zynq and Versal devices integrate hard ARM cores alongside FPGA fabric, eliminating the soft-processor resource cost.

### Development board landscape

For learning and prototyping before production Alveo/SmartNIC work:

| Board | FPGA | Logic Elements | Price | Key Features |
|-------|------|---------------|-------|--------------|
| MAX1000 | Intel MAX10 | 2K–16K | ~$30 | 8MB SDRAM, ADC, accelerometer, UART, Arduino header |
| CYC1000 | Intel Cyclone 10 LP | 25K | ~$30 | 8MB SDRAM, 8MB flash, accelerometer |
| Arty A7-100T | AMD Artix-7 | 101K cells | ~$299 | 240 DSP, Ethernet, Pmod/Arduino connectors |
| Basys 3 | AMD Artix-7 | — | ~$150 | 16 switches, 16 LEDs, 7-segment, VGA, good for coursework |
| PYNQ-Z2 | Xilinx Zynq | 1.3M gates | ~$130 | ARM Cortex-A9 + FPGA, Python/Jupyter workflow |
| USB104 A7 | AMD Artix-7 | 101K cells | ~$349 | SYZYGY expansion, USB-JTAG |
| BeMicro MAX10 | Intel MAX10 | 8K | — | ADC, sensors, Arrow dev ecosystem |

## References

- [An Accelerator for Decoding Market Data Based on FPGA](https://doi.org/10.1142/S0218126619500506)
- [High Frequency Trading Acceleration Using FPGAs](https://doi.org/10.1109/FPL.2011.64)
- [Acceleration of Trading System Back End with FPGAs Using HLS](https://www.mdpi.com/2079-9292/12/3/520)
- [AMD Alveo SN1000 SmartNIC data sheet](https://docs.amd.com/api/khub/documents/rllO00cN~P_4HGlYrQWB6w/content)
- [VHDPlus FPGA components and IDE](https://vhdplus.com/docs/components/overview/)
- Dogan Ibrahim, *FPGA Programming and Hardware Essentials*, Elektor, 2024 (ISBN 978-3-89576-644-2)

Related:

- [[71 - FPGA Market Data Pipeline Deep Dive]]
- [[70 - Solarflare Onload and ef_vi Deep Dive]]
- [[72 - Production Low-Latency Trading System Construction]]
