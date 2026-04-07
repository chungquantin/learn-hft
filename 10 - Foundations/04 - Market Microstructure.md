---
title: Market Microstructure
tags: [microstructure, orderbook]
---

# Market Microstructure

Market microstructure is the study of how orders interact to form prices, spreads, and fills. In an HFT context, this subject matters because short-horizon trading is usually driven less by broad narrative and more by the immediate state of liquidity, order flow, and queue dynamics. At these timescales, the market is not a smooth price series. It is a living process of orders arriving, resting, getting canceled, and consuming one another.

That is why concepts like limit order book, spread, depth, imbalance, trades versus quotes, maker versus taker, queue priority, and adverse selection are not just vocabulary items. Each one changes expected PnL. Spread affects whether there is gross opportunity to capture at all. Queue priority affects whether a passive order is likely to fill before the market moves away. Maker versus taker choice changes both fees and exposure to adverse selection. Imbalance can hint at short-horizon pressure, but usually only in context rather than as a universal signal.

At short timescales, price movement is often shaped by local conditions: who is demanding liquidity, who is withdrawing it, which side of the book looks fragile, and whether a move is being driven by ordinary participation or stress-like flow. This is why a microstructure-aware system is always asking a deeper question than "did price go up or down?" It is asking why the move is happening and what kind of path it is likely to take from here.

Several mental models help. One is that the order book is a state machine rather than a static table. Another is that fills are path-dependent, not just price-dependent. Two strategies can enter and exit at similar quoted prices and still have very different real outcomes because one filled earlier in queue, one crossed more spread, or one traded during thinner liquidity. The path matters because the cost and probability of interaction with the book are part of the trade, not a layer outside of it.

Measurement therefore matters a great deal. Microprice, spread distribution, imbalance, cancel-to-trade ratio, realized slippage, and fill probability by queue depth are all useful, but they become more useful when measured conditionally. Time of day, venue, volatility regime, funding regime, and strategy state often change what those metrics mean. Many microstructure edges are conditional rather than universal.

The main point of studying microstructure is not to become fascinated by order-book details for their own sake. It is to become capable of seeing what kind of market event you are actually dealing with at the moment a strategy must act. That ability is what allows short-horizon systems to distinguish real edge from expensive noise.

Related:

- [[05 - Exchange Architecture]]
- [[06 - Strategy Research]]
- [[07 - Risk Management]]
