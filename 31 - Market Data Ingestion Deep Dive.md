---
title: Market Data Ingestion Deep Dive
tags: [backend, market-data, ingestion, deep-dive]
---

# Market Data Ingestion Deep Dive

Market-data ingestion is the first place where the live engine meets the outside world. That is why it deserves to be thought of as more than a networking component. It is a trust boundary. Every other subsystem depends on the assumption that the events it receives have already been checked, interpreted correctly, and marked with the right validity status. If that assumption fails, the strategy may appear unstable even though the real problem is simply that the engine is no longer looking at a trustworthy picture of the market.

At a high level, ingestion is responsible for receiving exchange messages, decoding them, validating their sequencing, translating them into internal event types, and publishing them downstream. That description sounds simple until you remember that exchanges are not designed around your internal abstractions. Every venue has its own websocket schemas, subscription models, snapshot procedures, gap semantics, and reconnection behavior. Raw exchange data is therefore not your internal market model. It is merely source material from which a market model may or may not be built correctly.

This is why normalization is so important. A serious trading system does not allow strategy logic to depend directly on venue payload shapes. It inserts a normalization layer that turns exchange-specific messages into canonical internal events. That separation protects the system in several ways at once. It keeps strategy code from becoming coupled to wire-format accidents. It makes replay possible because live and historical inputs can share the same internal representation. It makes multi-exchange support easier because new venues can be adapted into the same event vocabulary instead of infecting the rest of the codebase with new schema rules.

The core ingestion loop is usually straightforward in outline: receive a frame, decode it, validate its place in the event stream, convert it into internal form, and publish it. The difficulty lies in what happens when continuity breaks. A missed delta, a stale subscription, an ignored checksum mismatch, or a reconnect that silently changes the semantic position of the stream can all produce a market view that still looks plausible while being wrong. That is one of the most dangerous states in an HFT system. A system that stops loudly when continuity is broken is often safer than one that keeps trading on a state that only looks continuous.

This leads to one of the most important principles in ingestion design: never pretend continuity when continuity is broken. If there is a sequence gap, the system should explicitly mark the downstream state as invalid, rebuild from a fresh snapshot when possible, and resume only after consistency has been re-established. A broken stream is not merely a minor data-quality issue. It is a change in what the rest of the system is allowed to trust.

Timestamping is another place where weak designs create long-term confusion. Ingestion should not collapse exchange event time, local receive time, and internal publish time into a single field called `timestamp`. Those fields answer different questions. Exchange time helps reconstruct market chronology. Local receive time helps reason about transport and intake latency. Internal publish time helps understand how the event moved through the engine. If these distinctions are blurred early, later debugging becomes much harder because replay, latency analysis, cross-venue alignment, and postmortems all start from a damaged time model.

Good ingestion systems therefore behave conservatively. They are suspicious rather than optimistic. They encode explicit invariants such as sequence monotonicity, symbol mapping consistency, checksum validity when available, and known subscription state after reconnect. They alarm not because every anomaly is fatal, but because the system must know the moment data stops being trustworthy. Ingestion is not rewarded for being clever. It is rewarded for being boring, strict, and honest.

If you want a compact way to remember the role of ingestion, use this: its job is not to get data in quickly at any cost. Its job is to make sure that every downstream decision is built on an event stream that deserves to be believed.

Related:

- [[32 - Order Book Engine Deep Dive]]
- [[36 - Reliability, Failure Modes, and Recovery]]
- [[41 - Data Collection and Storage]]
