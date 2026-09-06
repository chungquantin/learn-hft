---
title: LMAX Disruptor and Exchange Architecture
tags: [lmax, disruptor, concurrency, ring-buffer, matching-engine, event-sourcing, hft]
---

# LMAX Disruptor and Exchange Architecture

## Why LMAX matters

LMAX is valuable to study because it treated latency as an architectural problem rather than a collection of micro-optimizations. Its publicly documented work combines an exchange core, a single-threaded business-logic model, event sourcing, in-memory state, and the Disruptor for low-latency inter-thread messaging.

The most important lesson is not “use the Disruptor everywhere.” It is:

> Make the ownership, ordering, and persistence model explicit, then remove concurrency and queueing where the business semantics do not require them.

## The Disruptor in one sentence

The Disruptor is a preallocated, bounded ring buffer plus sequencing, consumer dependency barriers, and wait strategies for passing events between threads with low contention and predictable behavior.

It is more than a queue. The design separates:

- storage of reusable event slots;
- producer sequence allocation;
- publication of completed events;
- consumer progress tracking;
- dependency ordering;
- waiting strategy.

## Why an ordinary queue can hurt

An ordinary blocking queue often combines data storage, producer coordination, consumer coordination, locking, allocation, and notification. Under contention, each event can cause cache-line movement, lock arbitration, memory allocation, and scheduler activity.

The Disruptor attacks these costs through:

- preallocated event entries;
- bounded capacity;
- power-of-two ring sizing;
- sequence counters instead of item removal;
- cache-line padding to reduce false sharing;
- single-producer specialization;
- consumer dependency graphs;
- configurable wait strategies;
- batch visibility and processing.

## Core objects

### Ring buffer

The ring stores reusable event slots. Producers claim a sequence, populate the slot, and publish it. Consumers read by sequence and advance their own progress. The buffer is bounded, so capacity pressure is visible rather than hidden in unbounded heap growth.

### Sequencer

The sequencer coordinates producer claims. A single-producer sequencer can avoid unnecessary compare-and-swap contention. A multi-producer sequencer handles concurrent claims with more coordination.

### Sequence barrier

A consumer waits on a barrier that knows which producer or upstream consumers must have published before an event is safe to process. This models a dependency graph rather than forcing every stage through a serial queue.

### Wait strategy

The wait strategy determines how a consumer waits:

- busy spin for lowest latency and dedicated cores;
- yielding or sleeping for lower CPU consumption;
- blocking for more conventional resource use.

The choice is a production tradeoff. Busy spin is appropriate only when CPU ownership and power/thermal behavior are intentional.

## LMAX-style exchange shape

```text
network gateway
      |
      v
input sequencer / ring
      |
      v
single business-logic processor
      |
      +--> journal / replication
      +--> market-data publication
      +--> risk and downstream consumers
```

The business-logic processor is a natural single writer for state where event order determines the result: order matching, account transitions, balances, and other exchange semantics. Parallelize around it, not through it, when the state is inherently sequential.

## Single writer and deterministic order

The single-writer model removes locks from the state mutation path. It also gives a clean definition of order:

```text
input sequence -> validate -> mutate state -> emit facts
```

This is powerful for HFT because the same event stream can drive matching, risk, journaling, replication, and replay. It does not mean the entire system is single-threaded. Gateways, persistence, market-data encoding, and analytics can run concurrently as downstream consumers.

## Event sourcing and journal

Event sourcing stores the ordered facts needed to reconstruct state. A journal provides a durable or replicated sequence of commands/events; a snapshot shortens recovery time but does not replace the journal.

For a trading system, define:

- event sequence and epoch;
- command identity and idempotency key;
- input timestamp and receive timestamp;
- normalized event schema;
- state version;
- journal commit point;
- snapshot compatibility;
- replay and divergence checks.

The performance lesson is sequential I/O and in-memory decision state. The correctness lesson is that persistence and matching must have a clear relationship. “The message was sent” is not necessarily “the business event was committed.”

## Replication and high availability

An LMAX-style design can replicate the ordered event stream to a standby processor. The standby should consume the same deterministic sequence and verify state hashes or checkpoints. On failover, an epoch/fence prevents the old primary from continuing to publish orders.

The system needs to distinguish:

- input received;
- input accepted;
- state mutation applied;
- event journaled;
- output published;
- external order acknowledged.

These are different facts and should not be collapsed into one status flag.

## Applying Disruptor ideas in Rust

Rust does not require using the Java Disruptor library. The transferable design is:

- a bounded preallocated ring;
- explicit sequence numbers;
- single-producer specialization where possible;
- cache-padded producer/consumer cursors;
- acquire/release ordering at publication boundaries;
- fixed-size or arena-backed events;
- a clear policy for full capacity;
- deterministic consumer dependencies.

For a Rust matching engine, the likely shape is:

```text
ingress thread -> command ring -> single matching partition
                                  |
                                  +-> event ring -> journal
                                  +-> event ring -> market data
                                  +-> event ring -> risk/telemetry
```

Use the pattern only where the ownership model fits. A ring buffer does not make an algorithm safe by itself. Memory ordering, wraparound, reclamation, backpressure, and shutdown still require proof and tests.

## What not to copy blindly

- A ring buffer does not remove exchange-side latency.
- Lock-free does not mean wait-free or allocation-free.
- Busy spinning can damage tail latency elsewhere through thermal or core pressure.
- Multicast consumers must not observe partially published events.
- A single writer can become a bottleneck if the partition is too broad.
- Event sourcing does not make a bad event schema replayable.
- Java-specific GC techniques do not map directly to Rust.

## When to use it

Use an LMAX-style design when:

- event ordering is central to correctness;
- state mutation is naturally sequential per partition;
- multiple consumers need the same ordered events;
- allocation and queueing jitter matter;
- replay and recovery are first-class requirements.

Use a simpler channel when the workload is low-rate, the state is naturally concurrent, or the main bottleneck is network or exchange latency. The architecture should earn its complexity.

## References

- [LMAX Disruptor technical paper](https://lmax-exchange.github.io/disruptor/disruptor.html)
- [LMAX Disruptor User Guide](https://lmax-exchange.github.io/disruptor/user-guide/)
- [LMAX Disruptor project](https://lmax-exchange.github.io/disruptor/)
- [LMAX RingBuffer API](https://lmax-exchange.github.io/disruptor/javadoc/com.lmax.disruptor/com/lmax/disruptor/RingBuffer.html)
- [The LMAX Architecture by Martin Fowler](https://martinfowler.com/articles/lmax.html)

Related:

- [[24 - Queues, Ring Buffers, and Backpressure]]
- [[23 - Seqlocks Deep Dive]]
- [[65 - HFT Rust System Design Master Note]]
- [[63 - Rust Matching Engine Implementation Blueprint]]
- [[72 - Production Low-Latency Trading System Construction]]
