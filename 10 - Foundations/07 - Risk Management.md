---
title: Risk Management
tags: [risk, controls, trading]
---

# Risk Management

Risk management is part of the trading system, not a wrapper around it.

That means risk is not there to decorate the strategy. It is there to decide whether the strategy is allowed to act.

## Layers of risk

- market risk
- inventory risk
- leverage risk
- liquidation risk
- exchange risk
- operational risk
- software risk

These layers interact.

Example:

- a software bug can create unintended inventory
- unintended inventory creates market risk
- leverage converts that market risk into liquidation risk
- an exchange outage can then prevent correction

This is why good systems do not treat risk as only a "position sizing" problem.

## Hard controls

- max position size
- max notional by symbol
- max order size
- max loss per day
- stale market-data cutoff
- kill switch
- reject on inconsistent internal state

Hard controls should be:

- explicit
- testable
- explainable
- fast to evaluate

If a control is only "known by the team" but not encoded clearly, it is not a reliable control.

## For perpetuals specifically

- margin mode awareness
- mark-price driven liquidation distance
- funding exposure
- cross-venue collateral fragmentation

Perpetuals amplify risk because leverage makes time and uncertainty more expensive.

You need to know:

- how close the position is to forced action
- which price drives that forced action
- whether collateral assumptions are still valid
- whether exchange-side behavior changes under stress

## Principle

If a system cannot explain why it has risk, it should not be allowed to add more.

Operational version of the principle:

When state is ambiguous, reduce risk before adding complexity.

Related:

- [[03 - Perpetuals Trading]]
- [[05 - Exchange Architecture]]
