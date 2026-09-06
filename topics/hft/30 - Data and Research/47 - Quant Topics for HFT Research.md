---
title: Quant Topics for HFT Research
tags: [quant, research, hft, microstructure, reading-map]
---

# Quant Topics for HFT Research

Quant research for HFT is not only about finding predictive models. It is about modeling a decision problem where timing, execution, fees, inventory, and market mechanics change the value of a signal. A feature that predicts price direction but cannot be traded after latency and fill costs is not yet an edge.

This note is a reading map for the quant side of the vault.

## Probability and statistics

Foundations:

- probability distributions
- conditional expectation
- covariance and correlation
- hypothesis testing
- confidence intervals
- Bayesian updating
- multiple-testing risk
- bootstrapping and resampling

Why it matters:

Most HFT research is an argument under uncertainty. You are trying to decide whether a pattern is stable enough to trade after costs. Without statistical discipline, backtests become storytelling.

## Time series

Important topics:

- stationarity and regime shifts
- autocorrelation and partial autocorrelation
- volatility clustering
- realized volatility
- cointegration
- lead-lag relationships
- state-space models
- online estimation

Why it matters:

Market data is sequential and nonstationary. A signal can work in one regime and disappear in another. Short-horizon strategies need features that update online and degrade visibly when the market changes.

## Market microstructure

Important topics:

- bid/ask spread
- effective spread
- realized spread
- order-book depth
- imbalance
- queue position
- order-flow toxicity
- adverse selection
- price impact
- trade sign classification

Why it matters:

Microstructure turns price prediction into execution-aware research. It asks not only "where will price move?" but "can I interact with the book profitably before the opportunity disappears?"

## Point processes and event models

Important topics:

- Poisson processes
- Hawkes processes
- self-exciting order flow
- cancellation intensity
- fill intensity
- event-time sampling
- survival models for order lifetime

Why it matters:

HFT data is event-driven. Trades, cancels, quote changes, and liquidations arrive irregularly. Modeling event intensity can be more useful than forcing everything into fixed time bars.

## Optimal execution

Important topics:

- implementation shortfall
- temporary versus permanent impact
- Almgren-Chriss-style execution
- VWAP/TWAP participation
- smart order routing
- queue-aware execution
- cancel/replace policy
- taker versus maker choice

Why it matters:

Even a good signal can lose money through bad execution. Optimal execution teaches how to trade size while controlling impact, timing risk, and fill uncertainty.

## Market making

Important topics:

- spread capture
- inventory control
- reservation price
- skewed quoting
- Avellaneda-Stoikov-style models
- fill probability estimation
- adverse selection measurement
- rebate economics
- quote lifetime and refresh policy

Why it matters:

Market making is the cleanest bridge between quant modeling and system design. The model must understand inventory, fees, latency, and queue position.

## Cross-venue and arbitrage

Important topics:

- basis
- funding
- spot-perp relationships
- lead-lag signals
- latency-adjusted fair value
- transfer and inventory constraints
- stale quote detection
- exchange-specific fees and limits

Why it matters:

Crypto markets are fragmented. A price difference is not automatically arbitrage. It becomes interesting only after latency, fees, inventory location, settlement, and exchange risk.

## Perpetual futures

Important topics:

- funding-rate prediction
- mark/index divergence
- liquidation cascades
- open interest
- leverage regimes
- basis convergence
- insurance fund and ADL mechanics

Why it matters:

Perpetuals are not just spot with leverage. Funding, liquidation, and mark-price mechanics create event flow that can dominate short-horizon behavior.

## DeFi and AMM math

Important topics:

- constant-product AMMs
- stable-swap curves
- concentrated liquidity
- tick-level liquidity
- impermanent loss
- LP fee versus inventory risk
- routing and path optimization
- MEV and sandwich risk
- oracle manipulation

Why it matters:

DEX liquidity is a different market structure. In CLMMs, LP positions behave like range-bound market-making strategies. Quant research has to model pool math, chain latency, routing, and adversarial execution.

## Machine learning

Important topics:

- regularization
- cross-validation under time ordering
- leakage prevention
- online learning
- calibration
- feature importance
- model decay
- regime classification
- reinforcement learning limits

Why it matters:

ML can help, but it is easy to overfit. In HFT, the label must correspond to a tradable decision after latency, fees, and fill mechanics. Otherwise the model is solving the wrong problem beautifully.

## Risk and portfolio control

Important topics:

- drawdown control
- inventory limits
- exposure netting
- tail risk
- stress scenarios
- kill switches
- correlation breakdown
- liquidity-adjusted risk

Why it matters:

Short-horizon systems can accumulate risk quickly. Quant research should define not only when to trade, but when the strategy should stop believing its own signals.

## Backtesting realism

Important topics:

- event-driven replay
- latency modeling
- queue-position modeling
- fee and rebate modeling
- slippage modeling
- partial fills
- stale data
- survivorship bias
- parameter sweep discipline

Why it matters:

Backtesting is useful only if it preserves the constraints of the live decision problem. A backtest without execution realism is often an optimism generator.

## Suggested reading path

1. Read [[04 - Market Microstructure]] and [[28 - Market Making Deep Dive]].
2. Read [[42 - Research and Backtesting Systems]] before building strategy experiments.
3. Study event-driven features through [[46 - Order Flow and Event-Driven Trading]].
4. Study cross-venue behavior through [[29 - Arbitrage and Lead-Lag Deep Dive]].
5. Study DeFi liquidity through [[38 - Liquidity Programs and CLMM Incentives]].
6. Connect research assumptions back to deployment through [[39 - Matching Engine Deployment, FPGA, DPDK, and Cloud Colocation]].

Related:

- [[06 - Strategy Research]]
- [[42 - Research and Backtesting Systems]]
- [[43 - Feature Engineering and Labeling]]
- [[45 - Analytics and Post-Trade Review]]
- [[46 - Order Flow and Event-Driven Trading]]
