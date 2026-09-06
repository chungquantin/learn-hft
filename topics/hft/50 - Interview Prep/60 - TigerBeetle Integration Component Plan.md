---
title: TigerBeetle Integration Component Plan
tags: [tigerbeetle, ledger, settlement, accounting, interview]
---

# TigerBeetle Integration Component Plan

This note explains how to integrate TigerBeetle as the accounting source of truth in an HFT stack.

## Integration Goal

Use TigerBeetle to guarantee financial correctness through double-entry transfers while keeping matching and execution latency-critical paths independent.

Hot path principle:

- matching and risk decide quickly
- ledgering is asynchronous but strongly consistent at posting boundary

## Account Model

Define a minimal chart of accounts per trading account:

- `cash_available`
- `cash_reserved_for_orders`
- `position_inventory` (per instrument if needed)
- `fees_accrued`
- `realized_pnl`
- `unrealized_pnl` (optional, often computed not posted)

Also define platform/venue side accounts for balancing transfers.

## Event To Ledger Mapping

Map `ExecutionEvent` to `LedgerInstruction`:

- order accepted with reservation -> move available cash to reserved cash
- partial/full fill -> reduce reserved, move to executed cash/position accounts
- cancellation -> release reserved cash back to available
- fee charge -> transfer to fee account

Each instruction includes:

- deterministic `ledger_event_id`
- causality links (`order_id`, `fill_id`, `exec_seq`)
- replay metadata (`source_partition`, `source_offset`)

## Idempotency and Replay Safety

Rules:

- every posting request must carry stable unique transfer id(s)
- maintain dedupe table keyed by event id before posting side effects
- retries must be safe and non-amplifying
- reconciliation job replays missing events only

## Consistency Boundaries

What is immediate:

- risk and order-state updates in trading engine memory

What is eventual:

- persisted accounting truth in TigerBeetle

Interview framing:

- "trading loop state is operational truth; TigerBeetle is financial truth"
- "reconciliation continuously closes the gap"

## Failure Handling

Failure cases:

- TigerBeetle temporarily unavailable
- timeout after unknown commit status
- duplicate delivery from message bus

Handling:

- park unposted events in durable queue
- query/dedupe before retrying unknown outcomes
- alert when posting lag breaches threshold
- block withdrawals/transfers if reconciliation gap exceeds risk policy

## Testing Plan

Integration tests:

- fill -> ledger postings balance exactly
- duplicate ledger event does not create duplicate transfer
- recovery after ledger outage results in exact-once financial outcome

Audit tests:

- sum of all debits equals sum of all credits
- account constraints never violated after replay

## Interview Questions To Drill

- "Why TigerBeetle instead of a general SQL database?"
- "How do you guarantee no double-posting under retries?"
- "What happens when execution succeeded but ledger posting is delayed?"

Related:

- [[33 - Execution Management Deep Dive]]
- [[34 - Risk Engine Deep Dive]]
- [[59 - Distributed Topology and Reliability Component Plan]]
