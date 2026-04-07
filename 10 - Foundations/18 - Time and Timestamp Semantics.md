---
title: Time and Timestamp Semantics
tags: [time, timestamps, latency, measurement]
---

# Time and Timestamp Semantics

Time handling in trading systems becomes dangerous when every subsystem uses the word `timestamp` as if it meant the same thing. It usually does not. Different timestamps answer different questions, and flattening them into one field makes replay, benchmarking, monitoring, and postmortems much harder than they need to be.

The practical goal is not to collect more times. It is to preserve the right times with the right meanings.

## Canonical timing layers

Useful timing fields often include:

- exchange event time
- local receive time
- normalized or publish time
- strategy decision time
- risk-complete time
- order-submit time
- venue acknowledgement time
- fill time

These fields should usually remain distinct, even if some systems do not always populate every one of them.

## What each timestamp answers

Exchange event time:

- when the venue says the event happened
- useful for market chronology and cross-venue reasoning
- not enough on its own to explain local latency

Local receive time:

- when your system first observed the event
- useful for transport and ingestion-path analysis
- does not tell you when the venue originally produced it

Normalized or publish time:

- when the event became trusted internal data
- useful for measuring ingest and handoff cost
- helps separate raw intake delay from downstream processing delay

Strategy decision time:

- when signal logic concluded that action mattered
- useful for measuring decision latency and stale-state windows

Risk-complete time:

- when pre-trade permission was established
- useful for separating signal cost from safety-gating cost

Order-submit time:

- when the system actually attempted to express intent to the venue
- useful for outbound-path measurement

Venue acknowledgement time:

- when the venue confirmed or rejected receipt
- useful for diagnosing venue or transport delay

Fill time:

- when economic exposure actually changed
- useful for execution-quality analysis and post-trade attribution

## Source quality matters

Not all timestamps are equally trustworthy for all questions.

Software timestamps are often enough for:

- internal component timing
- queue wait measurement
- replay sequencing inside one host

Hardware-adjacent timestamps may be needed for:

- packet-path diagnosis
- very tight latency attribution
- distinguishing application delay from network-interface delay

The important principle is to match the timestamp source to the question. A software timestamp may be perfectly useful for strategy-latency analysis and still be too weak for wire-level diagnosis.

## Flow through the system

A good timing model should survive the whole pipeline:

1. ingestion records receive and source timing
2. normalization preserves timing meaning
3. storage writes those fields without flattening them
4. replay reuses the same semantics
5. benchmarking and dashboards interpret the same vocabulary

If one layer renames, drops, or merges timestamps casually, every later layer becomes less trustworthy.

## Common mistakes

- using one generic `timestamp` field for several meanings
- overwriting exchange time with local receive time
- measuring only tick-to-trade totals and losing component breakdown
- assuming software timestamps are enough for every diagnosis
- failing to preserve timing fields into storage and replay

## Why this matters

Strong timing semantics make it easier to answer:

- was the feed late or were we late?
- was the decision slow or the order path slow?
- did the venue acknowledge slowly or did we submit late?
- was the strategy acting on stale state?

Without clear timestamp semantics, those questions turn into guesswork.

Related:

- [[15 - Benchmarking and Tick-to-Trade Measurement]]
- [[25 - Logging and Telemetry Deep Dive]]
- [[31 - Market Data Ingestion Deep Dive]]
- [[33 - Execution Management Deep Dive]]
- [[41 - Data Collection and Storage]]
- [[42 - Research and Backtesting Systems]]
- [[52 - Monitoring, Alerting, and Incident Response]]
- [[92 - Developing High-Frequency Trading Systems (Full Research)]]
