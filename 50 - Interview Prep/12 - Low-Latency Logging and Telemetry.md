---
title: Low-Latency Logging and Telemetry
tags: [logging, telemetry, observability, low-latency]
---

# Low-Latency Logging and Telemetry

Observability is necessary in trading systems, but it has to be designed so it does not dominate the latency budget.

## Logging rule

Do not perform blocking I/O on the strategy thread.

## Better pattern

- send log work to another thread using a low-contention queue
- move formatting work off the hot path where possible
- pin or isolate the logging thread if it helps reduce interference

## Message-level telemetry

Useful metrics include:

- propagation latency
- processing time
- queue wait time
- downstream message lineage

## Design pattern

Wrap business messages with lightweight timing metadata so the system can track origin time and publish time without forcing each subsystem to manually repack data.

## Practical warning

Telemetry should help explain system behavior, but if it creates shared contention or excessive allocation it stops being telemetry and starts being the problem.

Related:

- [[02 - Rust for HFT]]
- [[05 - Exchange Architecture]]
- [[11 - Seqlocks]]
- [[90 - Source Notes]]
