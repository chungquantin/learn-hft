---
title: Changelog
tags: [changelog, maintenance]
---

# Changelog

This page records meaningful changes to the Learn HFT vault and companion code.

Update this page every time new changes are made. Keep entries short, dated, and useful for future traversal:

- Added: new notes, projects, examples, or major sections.
- Changed: reorganized pages, rewritten explanations, or renamed paths.
- Fixed: corrected technical explanations, broken links, build issues, or project bugs.

## 2026-08-11

### Added

- Added this changelog page so future vault and project updates have a single visible history.
- Linked the changelog from the Start pages so it is part of the normal navigation path.

- Added the [HFT to AI Infrastructure Technology Transfer](30%20-%20Data%20and%20Research/48%20-%20HFT%20to%20AI%20Infrastructure%20Technology%20Transfer.md) research note.
- Added the [Expert HFT Technology Curriculum](20%20-%20Deep%20Dives/81%20-%20Expert%20HFT%20Technology%20Curriculum.md).

### Changed

- Consolidated duplicate low-latency and matching-engine notes into canonical deep dives: [Queues, Ring Buffers, and Backpressure](20%20-%20Deep%20Dives/24%20-%20Queues,%20Ring%20Buffers,%20and%20Backpressure.md), [Seqlocks Deep Dive](20%20-%20Deep%20Dives/23%20-%20Seqlocks%20Deep%20Dive.md), [Logging and Telemetry Deep Dive](20%20-%20Deep%20Dives/25%20-%20Logging%20and%20Telemetry%20Deep%20Dive.md), and [HFT Rust System Design Master Note](50%20-%20Interview%20Prep/65%20-%20HFT%20Rust%20System%20Design%20Master%20Note.md).
- Expanded the technology curriculum with [InfiniBand and RDMA](20%20-%20Deep%20Dives/68%20-%20InfiniBand%20and%20RDMA%20Deep%20Dive.md), [DPDK](20%20-%20Deep%20Dives/69%20-%20DPDK%20Deep%20Dive.md), [Solarflare Onload and ef_vi](20%20-%20Deep%20Dives/70%20-%20Solarflare%20Onload%20and%20ef_vi%20Deep%20Dive.md), [FPGA Feed Handlers](20%20-%20Deep%20Dives/74%20-%20FPGA%20Feed%20Handlers%20and%20Inline%20Accelerators.md), [Hardware Timestamping and PTP](20%20-%20Deep%20Dives/75%20-%20Hardware%20Timestamping%20and%20PTP%20Deep%20Dive.md), [LMAX Disruptor](20%20-%20Deep%20Dives/77%20-%20LMAX%20Disruptor%20and%20Exchange%20Architecture.md), [CPU/NUMA/Memory](20%20-%20Deep%20Dives/78%20-%20CPU%20NUMA%20Memory%20and%20Compiler%20Deep%20Dive.md), [Performance Engineering and eBPF](20%20-%20Deep%20Dives/79%20-%20Performance%20Engineering%20and%20eBPF%20Deep%20Dive.md), and [SmartNIC/DPU Offload](20%20-%20Deep%20Dives/80%20-%20SmartNIC%20DPU%20and%20Network%20Offload%20Deep%20Dive.md).
- Added the integrated [Production Low-Latency Trading System Construction](20%20-%20Deep%20Dives/72%20-%20Production%20Low-Latency%20Trading%20System%20Construction.md) guide and [HFT Technology Stack and Priority Map](20%20-%20Deep%20Dives/76%20-%20HFT%20Technology%20Stack%20and%20Priority%20Map.md).

### Verified

- Quartz build passed with 88 Markdown inputs and 595 generated files.
