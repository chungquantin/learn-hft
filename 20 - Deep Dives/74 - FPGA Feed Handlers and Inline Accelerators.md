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

The first FPGA was developed by Altera in 1984 (EP300), an EPROM-based device erased by UV lamp. The first commercial FPGA with programmable gates and interconnects was Xilinx's XC2064 in 1985. By the 2010s, Altera and Xilinx together held over 70% market share. Intel acquired Altera in 2015; AMD acquired Xilinx in 2022.

### FPGA internal structure

```text
┌─────────────────────────────────────────────┐
│  I/O Blocks (IOBs) — boundary to pins       │
│  ┌─────────────────────────────────────────┐ │
│  │  Programmable Interconnect Fabric       │ │
│  │  ┌──────┐  ┌──────┐  ┌──────┐          │ │
│  │  │ CLB  │  │ CLB  │  │ CLB  │  ...      │ │
│  │  │(LUTs,│  │      │  │      │          │ │
│  │  │ FFs, │  │      │  │      │          │ │
│  │  │carry)│  │      │  │      │          │ │
│  │  └──────┘  └──────┘  └──────┘          │ │
│  │  ┌──────┐  ┌──────┐  ┌──────┐          │ │
│  │  │ BRAM │  │ DSP  │  │ CLB  │  ...      │ │
│  │  │block │  │slice │  │      │          │ │
│  │  └──────┘  └──────┘  └──────┘          │ │
│  │  ┌──────┐  ┌──────┐                    │ │
│  │  │ PLL  │  │Clock │                    │ │
│  │  │      │  │ Mgmt │                    │ │
│  │  └──────┘  └──────┘                    │ │
│  └─────────────────────────────────────────┘ │
│  Dedicated transceivers (GTP/GTX/GTH/GTY)    │
└─────────────────────────────────────────────┘
```

Key building blocks:

- **LUTs (Lookup Tables)**: implement arbitrary boolean functions. A 6-input LUT can implement any function of 6 variables. Multiple LUTs combine for wider logic.
- **Flip-flops**: store one bit of state. Clocked by the global or regional clock network. Edge-triggered D-type flip-flops are the fundamental sequential element.
- **Carry chains**: fast dedicated routing for arithmetic (addition, subtraction, comparison). Critical for counter and comparator performance.
- **DSP slices**: hardened multiply-accumulate units (e.g., 18×18 or 25×18 multipliers). Used for signal processing, fixed-point arithmetic, and CRC computation.
- **Block RAM (BRAM)**: dual-port memory blocks, typically 18 Kb or 36 Kb. Used for FIFOs, lookup tables, and small buffers. Can be configured as ROM, single-port, or dual-port RAM.
- **UltraRAM (URAM)**: larger on-chip memory blocks (288 Kb each) in Xilinx UltraScale+. Used for order-book state, symbol tables, or deeper FIFOs.
- **PLLs / MMCMs**: clock management. Generate derived clocks, multiply/divide frequencies, phase-shift. A 12 MHz oscillator can be PLLed to 100 MHz, 200 MHz, or higher.
- **I/O blocks**: interface to physical pins. Support various voltage standards (LVCMOS, LVDS, HSTL). Differential pairs enable high-speed serial I/O.
- **Transceivers**: hardened multi-gigabit serial I/O (GTH/GTY at 10–28 Gbps). Used for 10/25/100GbE, PCIe, and other high-speed protocols.

### FPGA vs ASIC vs CPLD

| Property | FPGA | ASIC | CPLD |
|----------|------|------|------|
| Reconfigurable | Yes (SRAM/flash) | No | Limited |
| Unit cost at volume | Higher | Lower | Lower |
| NRE cost | Low | Very high ($M+) | Low |
| Speed | Fast | Fastest | Moderate |
| Power | Moderate | Lowest | Low |
| Complexity | High | Highest | Low-moderate |
| Time to market | Weeks-months | Months-years | Days-weeks |
| Use case | Prototyping, low-mid volume, reconfigurable acceleration | High-volume production | Simple glue logic, interfacing |

CPLDs (Complex Programmable Logic Devices) are simpler than FPGAs: fewer logic elements, non-volatile configuration, deterministic timing. Used for front-end glue logic, interfacing, and state machines. Not suitable for complex pipelines.

FPGAs occupy the middle ground: reconfigurable like software, parallel like hardware. For HFT, they bridge the gap between CPU flexibility and ASIC performance without the multi-million-dollar NRE cost and year-plus lead time of a custom chip.

### FPGA types

- **SRAM-based**: configuration stored in volatile SRAM cells. Must reload bitstream on every power-up. Standard for development and most production accelerator cards (Alveo, SmartNICs). Allows unlimited reprogramming. Configuration time ranges from milliseconds to seconds depending on bitstream size.
- **Flash-based**: configuration stored in non-volatile flash memory cells. Retains configuration across power cycles — instant-on within milliseconds. Intel MAX10 is flash-based (dual configuration images, internal flash). Relevant for embedded or always-on deployments where boot time matters.
- **Anti-fuse**: one-time programmable. Cannot be reconfigured. Used in defense, aerospace, and high-reliability applications where immutability and radiation hardness are requirements. Actel (now Microchip) was the primary vendor.

### Key FPGA vendors and families

| Vendor | Entry-Level | Mid-Range | High-End | HFT-Relevant |
|--------|-------------|-----------|----------|--------------|
| AMD/Xilinx | Artix-7 | Kintex-7, Kintex UltraScale | Virtex UltraScale+, Versal | Alveo U50/U250/U280 |
| Intel/Altera | MAX10, Cyclone 10 LP | Cyclone V, Arria 10 | Stratix 10, Agilex 7/9 | Stratix 10 NX, Agilex |
| Lattice | iCE40 | ECP5 | Nexus (Certus) | — |
| Microchip | — | PolarFire | PolarFire SoC | — |
| GOWIN | GW1N | GW2A | — | — |

The FPGA market is expected to exceed $20B by 2030. For HFT, AMD/Xilinx Alveo and Intel Stratix/Agilex are the primary production platforms. Entry-level boards (MAX1000 at ~$30, CYC1000 at ~$30) are useful for learning FPGA fundamentals before committing to production hardware.

### HDL and development languages

FPGAs are programmed using hardware description languages. The critical difference from software: HDL describes parallel hardware that exists simultaneously, not sequential instructions.

**VHDL**

Strongly typed, verbose, explicit. Dominant in European firms and defense. Design can be at behavioral level (abstract algorithm) or structural level (gate-level instantiation). Signal assignments use `<=` and take effect at the next clock edge (delta cycle semantics). Every process runs concurrently.

```vhdl
-- VHDL: simple counter
process(clk)
begin
  if rising_edge(clk) then
    if count < max_count then
      count <= count + 1;
    else
      count <= (others => '0');
    end if;
  end if;
end process;
```

**Verilog/SystemVerilog**

C-like syntax, less ceremony. Dominant in US trading firms and ASIC-adjacent work. SystemVerilog adds assertions, constrained-random verification, covergroups, and interfaces. Blocking (`=`) vs non-blocking (`<=`) assignment distinction is a common source of bugs.

```verilog
// Verilog: simple counter
always @(posedge clk) begin
  if (count < max_count)
    count <= count + 1;
  else
    count <= 0;
end
```

**VHDPlus**

Simplified VHDL superset created by VHDPlus/Trenz Electronic. Key language features:

- `Main{}` block: top-level entity, receives FPGA I/O pins
- `Process{}` block: clocked logic, executes every clock cycle
- `Thread{}` block: sequential multi-cycle operations using `Step{}`, `While{}`, and `Wait{}`
- Signals assigned with `<=` (update next cycle); variables assigned with `:=` (update instantly)
- Outputs cannot be read directly; use `BUFFER` if read-back needed
- Each `Main`, `Component`, and `Class` requires a separate `.vhdp` file
- Division consumes significant logic resources; use bit-shifting or counters instead
- Transpiles to standard VHDL; any VHDL code can be mixed in

```
-- VHDPlus: LED blink
Main
(
    LED : OUT STD_LOGIC;
)
{
    Process()
    {
        Thread
        {
            LED <= '1';
            Wait(12000000);  -- 1 second at 12 MHz
            LED <= '0';
            Wait(12000000);
        }
    }
}
```

VHDPlus IDE (Windows, Linux; macOS limited — Quartus unsupported on macOS) integrates:
- Code editor with syntax highlighting and auto-completion
- Graphical pin assignment tool (eliminates manual constraint files)
- Package manager for libraries and components
- GHDL-based simulation with waveform viewing
- One-click download to FPGA via integrated USB programmer
- NIOS II soft-processor project generation

The IDE uses Quartus Prime Lite (free, version 18.1) as its backend compiler. The workflow is: write VHDPlus → transpile to VHDL → Quartus synthesizes → generates bitstream → downloads to FPGA.

**HLS (C/C++)**

Vivado HLS or Vitis compiles C/C++ to RTL. Faster iteration for pipeline stages that map cleanly to loops and arrays. The compiler infers pipeline stages, loop unrolling, and memory partitioning from pragmas. Less control over cycle-level timing and resource placement than hand-written RTL. Useful for algorithm exploration; production-critical paths often migrate to hand-written RTL after HLS proves the concept.

**Python (Amaranth/PYNQ)**

High-level synthesis or SoC interaction. Amaranth (formerly nMigen) generates Verilog from Python descriptions. PYNQ uses Jupyter notebooks to interact with Zynq SoC overlays. Research and prototyping tool, not production pipeline RTL.

### FPGA development workflow

```text
1. Describe function in HDL (VHDL, Verilog, VHDPlus, or HLS)
2. Simulate: functional simulation against reference model
   - GHDL (open source, VHDL), Icarus Verilog (open source, Verilog),
     ModelSim/Questa (commercial), Vivado Simulator
3. Synthesize: HDL -> netlist (logic optimization, technology mapping)
4. Place and route: map netlist to physical FPGA resources
5. Timing analysis: static timing analysis (STA) verifies all paths
   meet clock period constraints
6. Generate bitstream: binary configuration file for the FPGA
7. Load bitstream onto FPGA (JTAG, USB programmer, or PCIe)
8. Hardware-in-the-loop test with real or replayed data
9. Shadow beside CPU path before production trust
```

Timing closure is a critical gate: a design that simulates correctly may fail synthesis if logic paths exceed the clock period. Small changes can cause timing regressions, requiring re-placement or pipeline restructuring. Resource utilization reports show LUT, FF, BRAM, DSP, and I/O usage — exceeding ~80% utilization often makes timing closure harder.

### Soft processors

Intel FPGAs support NIOS II, a 32-bit soft processor instantiated in FPGA fabric. It consumes logic resources (LUTs, FFs, BRAM) but provides a general-purpose CPU inside the FPGA for C/C++ code. The MAX1000 can run NIOS II as an Arduino-compatible processor, enabling hybrid designs where hard real-time pipeline logic in RTL runs alongside general-purpose control for configuration, symbol table management, statistics reporting, and health monitoring.

AMD/Xilinx offers MicroBlaze (soft) and the Zynq/Versal platform (hard ARM Cortex-A9/A53/R5 cores alongside FPGA fabric). Hard cores eliminate the logic-resource cost of soft processors and provide full Linux capability for management planes.

For HFT feed handlers, the typical split is: RTL pipeline handles wire-speed parsing and filtering; soft or hard processor handles configuration loading, instrument table updates, statistics, and PCIe host communication setup.

### Development board landscape

For learning and prototyping before production Alveo/SmartNIC work:

**Intel/Altera entry-level boards (detailed specs)**

| Board | FPGA Chip | Logic Elements | RAM | Flash | PLLs | Multipliers | ADC | Clock | Price |
|-------|-----------|---------------|-----|-------|------|------------|-----|-------|-------|
| MAX1000 | 10M08SAU169C8G | 8,000 | 378 Kb | 64 Mb (W74M64FVSSIQ) | 2 (>300 MHz) | 24 (18×18) | 12-bit, 1 Msps, 8ch | 12 MHz (DSC6011ME2A) | ~$30 |
| Core MAX10 | 10M08SAU169C8G | 8,000 | 378 Kb | 64 Mb | 2 (>300 MHz) | 24 (18×18) | 12-bit, 1 Msps, 8ch | 12 MHz | ~$40 |
| CYC1000 | 10CL025YU256C8G | 25,000 | 594 Kb | 16 Mb (EPCQ16A) | 4 (>300 MHz) | 66 (18×18) | — | 12 MHz | ~$30 |

All three boards share: 64 Mb SDRAM (W9864G6JT-6), 3.3V I/O voltage (level shifter needed for 5V), integrated USB programmer, 3-axis accelerometer, and VHDPlus IDE integration with pre-defined pin configurations.

The MAX1000 includes 8 configurable LEDs, 2 user buttons, Arduino MKR headers, Pmod connector, and UART. The Core MAX10 adds CRUVI high-speed and low-speed connectors for camera/display extensions. The CYC1000 offers 3× the logic capacity (25K vs 8K), enabling more complex pipeline prototypes and multi-stage feed parsers.

**AMD/Xilinx boards**

| Board | FPGA | Logic Cells | DSP Slices | Memory | Price | Key Features |
|-------|------|------------|------------|--------|-------|--------------|
| Basys 3 | Artix-7 | — | — | 1,800 Kb block RAM | ~$150 | 16 switches, 16 LEDs, 4-digit 7-seg, VGA, 4 Pmod, USB-JTAG |
| Arty A7-100T | Artix-7 | 101,440 | 240 | 4,860 Kb | ~$299 | 10/100 Ethernet, USB-UART, 4 Pmod, Arduino headers |
| USB104 A7 | Artix-7 | 101,440 | 240 | 4,860 Kb | ~$349 | SYZYGY expansion, USB-JTAG, 3 Pmod |
| Artix-7 AC701 | Artix-7 | 215,360 | 740 | 13,140 cells | ~$1,500 | GbE, HDMI, LCD, SD, JTAG, Pmod, DIP switches |
| Spartan-7 SP701 | Spartan-7 | 102,400 | 160 | 4,320 Kb | ~$800 | HDMI, 2× Ethernet, MIPI camera/display, JTAG |
| PYNQ-Z2 | Zynq (XC7Z020) | 1.3M gates | — | 512 MB DDR3 | ~$130 | ARM Cortex-A9 + FPGA, USB/Ethernet/audio/video, Python/Jupyter |

**Other notable boards**

| Board | FPGA | Logic Elements | Price | Notes |
|-------|------|---------------|-------|-------|
| iCE40-HX8K | Lattice iCE40 | 7,680 cells | ~$30 | Open-source toolchain (Yosys/nextpnr), 128 Kb RAM, 2 PLLs |
| BeMicro MAX10 | Intel MAX10 | 8,000 | — | ADXL362 accelerometer, AD5681 DAC, thermal/photo resistors |

### VHDPlus extension ecosystem

The VHDPlus ecosystem provides modular extensions using the CRUVI (Common Reconfigurable Unit Versatile Interface) connector standard:

| Extension | Key Component | Specs | Use Case |
|-----------|--------------|-------|----------|
| Shield MAX10 | Dual DC/DC converters | 8.5–28V→5V (5A), 5V→3.3V (2A) | Power supply for 3 CRUVI + 6 Pmod extensions simultaneously |
| Camera | CSI + HDMI connectors | Raspberry Pi Camera compatible, HD video, CRUVI HS differential pairs | Object detection, video processing pipeline prototyping |
| Motor | 2× DRV8871DDAR drivers | 6.5–45V, 3A per channel, encoder input, PWM speed control | Robotics, encoder-based position tracking |
| Audio | MAX9867 codec | 48 kHz, 18-bit, stereo ADC+DAC, headset jack | Audio processing, synthesizer, voice recognition |
| WiFi | ESP-01 (ESP8266) | 802.11 b/g/n, UART interface | IoT connectivity, remote monitoring |
| Level Shifter | — | 3.3V ↔ 5V bidirectional | Interface with 5V peripherals (Arduino shields, sensors) |

The Shield MAX10 is the power backbone: its separated 3.3V converter (2A max) powers extensions independently from the FPGA core, preventing motor current spikes from affecting FPGA operation.

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
