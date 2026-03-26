---
title: Queues, Ring Buffers, and Backpressure
tags: [queues, ring-buffer, backpressure, deep-dive]
---

# Queues, Ring Buffers, and Backpressure

Queues matter in trading systems not merely because data has to move, but because event movement defines system semantics. A queue is never just a piece of plumbing. It decides whether order matters, whether data may be dropped, whether producers may be slowed down by consumers, and what happens when the system is under stress. That is why a fast queue is not automatically a good queue. The right queue is the one whose failure and overload behavior match the meaning of the messages it carries.

Trading systems are full of pipelines: parser to normalizer to order book, market data to strategy to risk to execution, strategy to telemetry to persistence. Those pipelines are usually drawn as boxes and arrows, but the arrows themselves carry architectural truth. If every message matters, the handoff semantics must preserve that. If some messages are optional, the handoff semantics can be more forgiving. If the message is not really a message at all but just the newest coherent state, then perhaps a queue is the wrong abstraction from the start.

Ring buffers appear so often in these systems because they align with several things low-latency code cares about: fixed capacity, contiguous memory, cheap indexing, and no further allocation once initialized. Those properties are powerful because they make costs more visible and reduce one major source of jitter. But fixed capacity always hides a policy decision. Eventually a producer catches a consumer. When that happens, what does the system do? Block the producer? Drop the newest item? Drop the oldest unread item? Overwrite silently? Signal backpressure and degrade? This policy question is at least as important as the queue's raw speed.

The same is true for queue shape. SPSC, SPMC, and MPMC are not just abbreviations to memorize. They describe fundamentally different coordination requirements. The more general the queue, the more coordination and atomic traffic it usually needs. That is why the right engineering question is not "what is the most flexible queue we could use?" but "what is the simplest shape that matches the actual ownership graph of this subsystem?" Generality that the system does not need often becomes measurable cost.

One of the strongest ideas in low-latency queue design is producer isolation. If the producer is on the hot path, then slow consumers should affect it as little as possible. That is especially important for side channels such as logging, persistence, or optional analysis subscribers. A design where consumers maintain their own read position and may fall behind can be excellent for those contexts. The exact same design can be completely wrong for execution-critical messages where losing or overwriting events would violate system meaning.

This is why backpressure cannot be treated as an afterthought. Any queue that can fill already has a backpressure problem, whether the designer admits it or not. The question is simply which consequence the system chooses. Blocking market-data ingestion may be catastrophic. Dropping fills or cancels may be catastrophic. Dropping debug telemetry may be acceptable. Once you see it this way, queue policy becomes inseparable from message importance.

Another subtle distinction is the difference between broadcast and work distribution. Sometimes multiple consumers should all observe the same event. Sometimes only one consumer should own it. These are different patterns, and confusing them leads to queue designs that either oversynchronize or accidentally change semantics. A market-data recorder and a strategy engine may both need to see the same normalized event. An execution worker pool might instead need ownership transfer of a task. The queue is part of that meaning, not just a generic transport mechanism.

The most practical way to evaluate a queue design is to imagine the ugly cases. What happens if the consumer becomes ten times slower than usual? What happens during burst traffic? Which messages can be lost, and what does that loss mean? Can an optional consumer be detached with almost no effect on the producer? If those questions have vague answers, the queue design is still a concept rather than a trustworthy system component.

The cleanest summary is that queues and ring buffers are valuable not because they are fashionable low-latency structures, but because they force the system to be explicit about overload, ordering, ownership, and loss.

Related:

- [[10 - Ring Buffers in Rust]]
- [[11 - Seqlocks]]
- [[12 - Low-Latency Logging and Telemetry]]
- [[26 - Building a Low-Latency Trading Engine]]
