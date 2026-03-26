---
title: Market Making Deep Dive
tags: [strategy, market-making, deep-dive]
---

# Market Making Deep Dive

Market making is often described too simply as "buy on the bid, sell on the ask, and capture the spread." That description is not wrong, but it hides the true difficulty of the problem. A market maker is not merely harvesting spread. It is continuously deciding when to provide liquidity, how much inventory to tolerate, how much adverse selection risk to accept, and how to behave when the market starts moving for reasons that are not yet fully visible.

The spread is therefore only the visible source of gross opportunity. Net opportunity depends on whether orders fill when you want them to, whether they fill before the market moves away, how often passive fills are followed by adverse price movement, what fees look like, and how inventory evolves as fills arrive asymmetrically. Many naïve market-making ideas die as soon as they are exposed to the fact that not all fills are good fills.

Queue position is central for this reason. In a passive strategy, you do not merely choose a price. You choose a place in line. That means expected value depends not only on the quoted spread, but on how likely your order is to execute before the market reprices, how often competing liquidity joins ahead of you, and how often the fill you do get is a sign that better-informed traders wanted out. Queue position turns a visible book into a probabilistic one.

Venue rules make this even sharper. On many venues, modifying a quote can reset or worsen queue priority. A price change almost always does. A size change may as well. This means a maker is constantly balancing quote freshness against queue ownership. If you refresh too eagerly, you may preserve theoretical quote quality while repeatedly throwing away the line position that made passive execution attractive in the first place.

This is why adverse selection sits near the heart of market making. A maker wants fills, but not every fill is beneficial. Some fills happen because uninformed flow happens to meet your quote. Others happen because someone faster or better informed is happy to trade against you just before price moves against your position. The art of market making is largely the art of distinguishing between these two environments quickly enough that your quoting behavior adapts.

Inventory control is the next major force. A maker that always quotes symmetrically regardless of inventory is usually pretending that exposure is someone else's problem. In reality, inventory changes what kind of fills you want, how aggressively you should quote, and whether spread capture is still worth pursuing. Market-making systems often earn money on some trades and give it back because inventory was allowed to drift into the wrong market regime.

Fees and rebates belong in the same decision loop. A quote is not attractive simply because it sits at the touch. The realized economics depend on maker fees or rebates, the chance of actually getting filled, the probability that the fill is toxic, and the cost of cleaning up the resulting inventory. Venue-specific fee schedules and market-maker programs therefore shape strategy quality more than beginners often expect.

This makes market making a strategy about balance. You are balancing fill probability against quote quality, inventory against opportunity, spread capture against adverse selection, and persistence against caution. In perpetual markets, funding and liquidation risk add another layer. A strategy that looks acceptable in spot terms may become much less attractive if inventory is expensive to carry or more dangerous to finance.

One reason market making is such a good learning strategy is that it forces you to care about nearly every important system component at once. You need a trustworthy order book. You need execution-state clarity. You need latency awareness. You need queue reasoning. You need risk controls that are alive to inventory drift. You need analytics good enough to separate spread capture from selection cost. It is hard to bluff your way through market making because the weaknesses show up quickly.

The most useful way to think about the strategy is not as "quoting both sides," but as continuously pricing liquidity under uncertainty. That framing makes the problem richer and more realistic. You are not posting orders into a neutral book. You are deciding what kind of risk you are willing to warehouse, under what conditions, and at what price.

Related:

- [[04 - Market Microstructure]]
- [[06 - Strategy Research]]
- [[19 - Matching Engines, Queue Priority, and Order Amend Semantics]]
- [[33 - Execution Management Deep Dive]]
- [[45 - Analytics and Post-Trade Review]]
