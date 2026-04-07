---
title: Seqlocks
tags: [rust, lock-free, synchronization, low-latency]
---

# Seqlocks

A seqlock is a synchronization primitive for publishing shared data with one writer and retrying readers.

## Core idea

- the writer increments a version counter before and after writing
- an even version means the data is stable
- a reader retries if the version changed during its read

## Why they fit low-latency systems

- writers are not blocked by readers
- readers do not block each other
- useful for publishing the latest snapshot rather than a full history

## Correctness requirement

Seqlocks depend on ordering. The version loads/stores and the data read/write cannot be freely reordered, so memory barriers or fences are part of the design, not an optimization detail.

## Practical use cases

- latest market snapshot
- shared configuration or risk state
- queue internals built from seqlocked slots

## Limitation

Seqlocks are for "latest value" style sharing. They are not a substitute for ordered message queues when every event matters.

Related:

- [[02 - Rust for HFT]]
- [[05 - Exchange Architecture]]
- [[10 - Ring Buffers in Rust]]
- [[90 - Source Notes]]
