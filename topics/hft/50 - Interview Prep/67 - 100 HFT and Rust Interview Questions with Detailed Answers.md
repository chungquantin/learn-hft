---
title: 100 HFT and Rust Interview Questions with Detailed Answers
tags: [hft, rust, interview, questions, answers]
---

# 100 HFT and Rust Interview Questions with Detailed Answers

## Market Microstructure and Trading Fundamentals

### 1) What makes HFT different from regular algorithmic trading?
**Answer:** HFT is defined by extreme sensitivity to latency, queue position, and microstructure details. Regular algo trading may optimize execution over seconds or minutes; HFT often competes at microsecond-to-millisecond horizons. In HFT, engineering quality (determinism, jitter control, feed integrity, and risk gating) is as important as strategy logic. Small implementation issues can erase edge.

### 2) Why is queue position so important in market making?
**Answer:** At the same price level, exchanges usually fill by time priority. Being earlier in queue directly increases fill probability and spread capture. Poor queue position means adverse selection risk rises because you get filled later, often when information has moved against you. That is why low cancel/replace latency and accurate local book state are central.

### 3) Explain adverse selection in one practical scenario.
**Answer:** Suppose you quote both sides and get lifted on your ask just before the market moves higher. If informed flow is trading against stale quotes, your fills are systematically biased. In practice, this appears as good fill rates but poor post-fill PnL. Mitigations include tighter risk filters, microprice-based skewing, and faster quote updates.

### 4) What is the difference between maker and taker behavior?
**Answer:** Maker behavior posts passive liquidity and tries to earn spread or rebates; taker behavior crosses the spread for immediacy. Makers face queue and adverse selection risk; takers face explicit spread/fee costs and potential slippage. Many HFT systems combine both: passive by default, aggressive when signal confidence or risk pressure crosses thresholds.

### 5) What is market impact, and does it matter for HFT?
**Answer:** Market impact is the price movement caused by your own trading. Even in HFT, impact matters, especially in thinner books or bursty regimes. Large or repeated child orders can move top-of-book and worsen future fills. Good engines incorporate participation limits, quote sizing logic, and cooldown rules to avoid self-induced deterioration.

### 6) Why can a profitable backtest fail in live HFT?
**Answer:** Backtests often underestimate queue effects, message latency, rejects, data gaps, and exchange-specific behavior. They may assume perfect fill models or ignore cancel delays. Live systems face jitter, packet loss, and changing market regimes. Robust validation requires replay with realistic execution simulation, slippage modeling, and stress scenarios.

### 7) How do perpetual futures differ from spot for system design?
**Answer:** Perpetuals add leverage, liquidation dynamics, funding mechanics, and position/margin constraints. Your risk and accounting pipelines must handle collateral reservation, mark/index price logic, and liquidation events. Systemically, this means tighter state coupling between execution, risk, and ledger reconciliation than simple spot-only designs.

### 8) What is microprice and why use it?
**Answer:** Microprice is a weighted midpoint using best bid/ask sizes to estimate short-horizon pressure. If bid size dominates ask size, microprice shifts upward from midpoint, suggesting upward pressure. It is useful for quote skewing and short-term signal generation, but must be combined with regime filters to avoid noise overfitting.

### 9) Why is spread alone a weak signal?
**Answer:** Spread is necessary context but insufficient. A narrow spread can hide toxic flow or weak queue position. A wider spread can still be attractive if fill toxicity is low and inventory/risk state favors quoting. Better decisions combine spread with imbalance, trade flow, volatility, and queue dynamics.

### 10) What does “preserve meaning under pressure” mean in HFT systems?
**Answer:** It means system outputs must stay semantically correct under burst load, partial outages, and ambiguous inputs. Events must remain ordered, state transitions must obey invariants, and risk decisions must remain safe. Fast-but-ambiguous behavior is unacceptable because it creates hidden financial and operational risk.

## Matching Engine and Order Book Design

### 11) What invariants must a matching engine never violate?
**Answer:** Core invariants include: no overfill, strict price-time priority, no crossing outside valid price conditions, and consistent order-state transitions (e.g., canceled orders cannot fill afterward). Event sequencing must be deterministic for the same input stream. Violating any of these breaks trust and makes reconciliation difficult.

### 12) Why use integer ticks instead of floating prices?
**Answer:** Floating-point arithmetic can introduce rounding ambiguity and non-deterministic comparisons. Integer ticks give exact ordering and consistent matching behavior across platforms and replays. They also simplify validation against tick-size constraints and make event logs cleaner for audit and debugging.

### 13) Describe a deterministic matching loop.
**Answer:** Validate command, check idempotency, then repeatedly match against best opposite price while marketable and quantity remains. Consume resting orders FIFO at each level, emit trade events, and update remaining quantity. Rest residual quantity if allowed by order type/time-in-force. Emit final state event with monotonic sequence IDs.

### 14) How do you handle cancel/replace safely?
**Answer:** Treat cancel/replace as explicit state transitions with versioning or sequence checks. Ensure only live orders can be modified. If concurrent fill activity exists, process by strict partition order so outcomes are deterministic. Emit clear reject reasons when stale versions or unknown order IDs are seen.

### 15) Why is single-writer partitioning common in low-latency engines?
**Answer:** A single writer per symbol partition avoids shared mutable-state contention and lock overhead. It produces deterministic ordering, simpler invariants, and lower tail jitter. Horizontal scale comes from many independent partitions rather than multithreaded mutation of the same book state.

### 16) What data structure is best for an order book?
**Answer:** There is no universal best. Common choices: tree maps for flexibility, array ladders for cache locality near top-of-book, and slab-backed FIFO queues for resting orders. The right choice depends on instrument characteristics, depth requirements, and update/read mix. Benchmark-driven selection is required.

### 17) How do you detect stale or invalid local order book state?
**Answer:** Use exchange sequence numbers, continuity checks, heartbeat/timeouts, and explicit trust states (`Healthy`, `Degraded`, `Rebuilding`, `Stale`). When a gap or invalid sequence appears, downgrade trust and gate strategy/risk actions until recovery completes.

### 18) Snapshot + delta: what can go wrong?
**Answer:** Deltas can arrive before snapshot application, sequence ranges may not align, or gaps/duplicates can appear during handoff. Without careful buffering and sequence validation, local state diverges. Correct handling includes bounded delta buffers, exact range checks, and full rebuild on continuity breach.

### 19) How do you test matching correctness beyond unit tests?
**Answer:** Add property-based tests for invariants under randomized command streams, deterministic replay tests for identical outputs, and scenario tests (cancel storms, partial fills, out-of-order commands). Include long-run fuzzing with invariant assertions enabled to catch rare transition bugs.

### 20) What does “idempotent command handling” mean in practice?
**Answer:** If the same logical command is delivered twice, the second delivery should not create new side effects. Use stable idempotency keys and dedupe windows. Return prior outcome or no-op deterministically. This is essential in at-least-once messaging environments and during replay/recovery.

## Latency Engineering and Performance

### 21) Why focus on p99/p99.9 instead of average latency?
**Answer:** Average hides tail spikes that cause real financial damage: missed queue position, stale actions, and risk lag. HFT outcomes are often dominated by outliers. p99/p99.9 captures worst operational behavior and forces engineering toward predictable low-jitter systems.

### 22) What causes latency jitter in Rust systems?
**Answer:** Common causes are allocator pressure, cache misses, branch misprediction, lock contention, NUMA effects, and noisy CPU scheduling. Even logging or string formatting in hot paths can introduce jitter. Profiling must be done under realistic load, not only microbenchmarks.

### 23) How do you reduce allocations in a hot path?
**Answer:** Pre-allocate buffers, reuse structs via pools/slabs, avoid temporary string creation, use fixed-size representations where possible, and design APIs that operate on borrowed data. Measure allocation counts directly to verify assumptions.

### 24) When should you use lock-free queues?
**Answer:** Use them at critical handoff points where low contention and bounded behavior are required. They reduce lock overhead, but complexity increases. They are not automatically faster under all patterns; benchmark under your exact producer/consumer profile and watch backpressure behavior.

### 25) What is a realistic latency budget decomposition?
**Answer:** Break down end-to-end command handling into decode/validate, book update/match, risk gate, event publish, and downstream handoff. Assign stage budgets and measure each separately. This identifies where tail growth occurs and prevents optimization in the wrong layer.

### 26) Why can "fast code" still lose money?
**Answer:** If state is wrong, risk checks are weak, or fills are toxic, speed amplifies bad decisions. Profitability depends on strategy edge plus execution quality plus risk discipline. Speed without correctness just increases the rate of loss.

### 27) How do CPU pinning and isolation help?
**Answer:** Pinning critical threads to dedicated cores reduces scheduler jitter and cache thrash. Isolation limits noisy neighbors from stealing cycles. Combined with controlled IRQ and topology-aware deployment, this improves tail consistency.

### 28) What is false sharing and why care?
**Answer:** False sharing occurs when independent data used by different threads resides on the same cache line, causing unnecessary coherence traffic. It increases latency and jitter. Use padding/alignment and careful data layout for heavily updated counters/state.

### 29) Why measure queue age, not only queue depth?
**Answer:** Depth alone misses urgency. A shallow queue with old commands can be worse than a deeper queue with fresh items. Queue age directly reflects time-in-system and helps trigger overload controls before latency SLOs are breached.

### 30) What’s a good strategy for performance regression control?
**Answer:** Keep reproducible benchmarks, store baselines, and fail CI when key percentiles regress beyond thresholds. Include workload metadata and seed control. Track both microbench and scenario-level benchmarks to avoid over-optimizing synthetic paths.

## Distributed Systems and Reliability in HFT

### 31) Should HFT systems be monoliths or microservices?
**Answer:** It depends on ownership and latency boundaries. Hot paths often stay tightly coupled (or co-located) to reduce hop cost, while warm paths split for resilience and team ownership. The right design is responsibility-driven, not style-driven.

### 32) How do you preserve order in distributed pipelines?
**Answer:** Partition by key (e.g., instrument), enforce single writer per partition, and use ordered transport guarantees per partition. Avoid cross-partition synchronous dependencies for hot decisions. Sequence IDs and replay logs provide additional consistency checks.

### 33) How do you design for at-least-once delivery?
**Answer:** Assume duplicates happen. Add stable event IDs, idempotent consumers, and dedupe stores. Side effects (ledger posts, notifications, state writes) must be safe under retries. Exactly-once transport is rare; exactly-once business effect is engineered.

### 34) What is a fail-closed policy in trading?
**Answer:** When system confidence in risk-critical state is degraded, reject or halt risk-increasing actions. This prevents uncertain conditions from creating unbounded exposure. Fail-open may be acceptable for non-critical telemetry, not for execution permissions.

### 35) How do you handle a market data gap during live trading?
**Answer:** Immediately downgrade trust state, gate strategies needing trusted book, attempt gap recovery or snapshot rebuild, and only re-enable once continuity is verified. Emit operator alerts and record incident context for postmortem.

### 36) What does graceful degradation look like?
**Answer:** The system keeps essential safety and core functionality while disabling non-essential features. For example, keep cancel support and risk controls active while pausing new quoting in degraded market-data state.

### 37) How do you reduce blast radius in outages?
**Answer:** Isolate by partition and service boundary, avoid shared global mutable dependencies, and implement circuit breakers/timeouts. Keep failure domains small so one bad partition or gateway does not stall the full platform.

### 38) Why is replay infrastructure a production feature, not just research tooling?
**Answer:** Replay enables deterministic debugging, incident reconstruction, state recovery verification, and regression testing. In live operations, the ability to reproduce exact behavior is crucial for trust and fast remediation.

### 39) What SLOs matter most for HFT backend reliability?
**Answer:** Key SLOs include hot-path latency percentiles, queue age bounds, feed continuity/trust uptime, risk gate availability, and reconciliation lag. Uptime alone is insufficient if system quality degrades silently.

### 40) How do you design operator controls safely?
**Answer:** Provide explicit, audited controls: halt/resume, kill switch, mode toggles, and partition-specific draining. Actions should be idempotent, logged, and bounded by authorization. Control plane must not block hot path.

## Risk Management and Controls

### 41) What is the minimal real-time risk layer for HFT?
**Answer:** At minimum: max order size, price-band sanity checks, account-level kill switch, max position/notional limits, and stale-state guards. These checks must be fast, deterministic, and always-on.

### 42) Why should risk checks be separated into fast and deep layers?
**Answer:** Fast checks protect every command without adding heavy latency. Deep checks (portfolio optimization, cross-venue aggregation) are computationally heavier and can run asynchronously. This split keeps hot path safe and fast.

### 43) How do you prevent runaway quoting loops?
**Answer:** Use rate limits, inventory caps, cooldown timers, and feedback controls tied to fill quality and PnL drift. Add hard stop conditions triggered by abnormal reject ratios or stale-state indicators.

### 44) What is inventory skewing?
**Answer:** It adjusts bid/ask aggressiveness based on current inventory to reduce directional exposure. For long inventory, quotes may be made less aggressive on bids and more aggressive on asks. It is a practical control in market making.

### 45) How do liquidation events affect risk logic in perp markets?
**Answer:** Liquidation cascades can rapidly change depth and volatility. Risk logic should tighten limits, widen guards, and possibly reduce strategy aggressiveness during stress regimes. Mark/index integrity and latency to liquidation signals matter.

### 46) Why are reject reason codes important?
**Answer:** They enable precise diagnosis, rule tuning, and postmortem analysis. Without explicit reasons, risk behavior appears random and hard to trust. Codes should be stable and structured for analytics.

### 47) What is “toxic flow” and how do you detect it?
**Answer:** Toxic flow is order flow that predicts adverse short-term moves against your quotes. Detection often uses post-fill short-horizon alpha decay, trade imbalance, and quote-to-fill performance metrics. Mitigation includes dynamic spread/skew adjustments.

### 48) How do you test risk guardrails effectively?
**Answer:** Use deterministic scenario tests: stale price, fast move, limit breach, gateway lag, and duplicate events. Confirm intended action (reject, cancel, halt) occurs within latency budget and leaves state consistent.

### 49) What’s the relationship between risk and latency?
**Answer:** Risk must be fast enough not to bottleneck execution, but strict enough to prevent catastrophic behavior. The design goal is low-latency deterministic checks with bounded complexity and clear fallback behavior.

### 50) What should happen if risk service times out?
**Answer:** For new risk-increasing commands, default to fail-closed. Allow safe operations like reducing exposure or canceling orders if policy permits. Emit alerts, mark degraded mode, and require recovery criteria before re-enabling normal flow.

## Execution, Connectivity, and Exchange Behavior

### 51) Why is exchange adapter design critical?
**Answer:** Exchange APIs differ in semantics, errors, and sequencing. A clean adapter layer normalizes these differences while preserving raw truth. Poor adapter design leaks venue quirks into core logic and increases failure risk.

### 52) How do you handle out-of-order execution reports?
**Answer:** Maintain per-order state machines with sequence/version checks and pending transitions. Buffer or reconcile when possible, and mark ambiguity when impossible. Deterministic handling rules are essential for reproducibility.

### 53) What is self-trade prevention (STP)?
**Answer:** STP prevents your own orders from matching against each other under configurable behavior (cancel newer, cancel older, decrement both, etc.). It avoids unnecessary fees and misleading volume while simplifying risk/accounting.

### 54) Why can cancel latency dominate strategy quality?
**Answer:** In fast markets, stale quotes become toxic quickly. If cancel round-trip is slow, you remain exposed at bad prices. Strong alpha with weak cancel performance still loses due to adverse fills.

### 55) What should execution state machine track?
**Answer:** Track order lifecycle (`New`, `Acked`, `PartiallyFilled`, `Filled`, `Canceled`, `Rejected`, `Expired`) plus venue IDs, timestamps, and cumulative fill quantity. Transitions must be validated and auditable.

### 56) How do you measure execution quality?
**Answer:** Use fill ratio, slippage vs decision price, post-fill markout, reject rates, cancel success latency, and queue-to-fill conversion. Segment by regime and symbol; averages across all conditions can hide failures.

### 57) How do you reconcile local intent vs venue truth?
**Answer:** Treat venue acknowledgements/fills as authoritative execution truth, while local intent is pending until confirmed. Continuously reconcile differences and time out ambiguous states with explicit recovery procedures.

### 58) What is a practical retry strategy for order submission?
**Answer:** Use idempotent client order IDs, bounded retries, and explicit retry reasons (timeout vs reject vs transport error). Never blind-retry without state checks; this can duplicate exposure.

### 59) Why do you need exchange-specific simulation in testing?
**Answer:** Generic simulators miss venue quirks like partial ack patterns, throttle behavior, and amend semantics. Exchange-specific behavior strongly affects fill and risk outcomes, so realistic simulation improves transfer to production.

### 60) How do you design for multiple venues?
**Answer:** Keep core abstractions stable (commands/events/state) and isolate venue-specific logic in adapters. Normalize where safe, preserve raw fields for debugging, and avoid forcing lowest-common-denominator semantics into critical logic.

## Data Integrity, Time, and Observability

### 61) Why is timestamp semantics a core design topic?
**Answer:** Multiple clocks exist: exchange event time, gateway receive time, engine process time, and operator wall time. Confusing them breaks latency analysis and incident debugging. Define clear timestamp taxonomy and usage contracts.

### 62) Event time vs processing time: why distinguish?
**Answer:** Event time reflects when the market event occurred; processing time reflects when your system handled it. Strategies and analytics relying on causality need this distinction to avoid false conclusions about latency and edge.

### 63) What should every critical event log include?
**Answer:** Include stable IDs, partition key, sequence number, event type, core quantities/prices, and timestamps from relevant stages. Logs must support deterministic replay and postmortem traceability.

### 64) How do you avoid telemetry hurting hot-path latency?
**Answer:** Use non-blocking async sinks, bounded buffers, sampling for high-volume traces, and minimal structured logging in critical loops. Prioritize essential counters/events and avoid heavy formatting on hot threads.

### 65) What metrics should be first on an HFT dashboard?
**Answer:** p99/p99.9 stage latencies, queue age/depth, feed trust state, reject/cancel rates, fill quality, and reconciliation lag. These directly indicate whether the system is fast, safe, and financially coherent.

### 66) What is data lineage in trading systems?
**Answer:** Data lineage tracks where each derived decision input originated and how it was transformed. It supports debugging, model accountability, and compliance by enabling exact reconstruction of decision context.

### 67) How do you detect silent data corruption?
**Answer:** Use checksums, schema/version validation, sequence consistency checks, invariants, and cross-source reconciliation. Silent corruption is dangerous because systems keep running while decisions degrade.

### 68) Why keep immutable event logs?
**Answer:** Immutable logs provide auditable history and replay capability. Mutable state can be derived from them, but immutable facts should remain intact. This improves trust, incident analysis, and recovery workflows.

### 69) What is an SLI/SLO pair you would define for market data?
**Answer:** Example SLI: percentage of time partitions are in `Healthy` trust state with no unresolved sequence gaps. SLO: `>= 99.9%` during trading hours. This links operational quality directly to decision reliability.

### 70) How do you build useful incident postmortems?
**Answer:** Include timeline, impact, root cause, contributing factors, detection gaps, and concrete remediations with owners. Attach reproducible replay evidence. Focus on system learning, not blame.

## Rust Language and Systems Programming Knowledge

### 71) How does ownership help in HFT code quality?
**Answer:** Ownership enforces clear state responsibility and prevents data races at compile time. In HFT, this reduces concurrency bugs and ambiguity in mutable access. It encourages architecture aligned with single-writer state models.

### 72) Explain borrowing choices in hot code paths.
**Answer:** Prefer borrowing over cloning to avoid allocations and copies, but keep lifetimes manageable for maintainability. Overly complex lifetime patterns can hurt readability; use owned boundaries where it simplifies correctness without violating performance budgets.

### 73) When would you use `Arc<Mutex<T>>` in HFT Rust?
**Answer:** Use it sparingly, typically in non-hot control paths or shared config/state where contention is low and clarity matters. In hot matching loops, prefer ownership partitioning and message passing to avoid mutex jitter.

### 74) What are common Rust collections in trading engines?
**Answer:** `Vec`/`VecDeque` for contiguous queues, `HashMap` for ID lookup, `BTreeMap` for ordered price levels, and slab-like structures for stable storage and indices. Choice must follow access pattern and latency profile.

### 75) Why can trait abstraction hurt performance?
**Answer:** Dynamic dispatch and abstraction boundaries can inhibit inlining and branch predictability in hot loops. This does not mean avoid traits entirely; isolate high-frequency paths and profile to ensure abstractions do not regress tails.

### 76) How do you approach unsafe Rust in HFT?
**Answer:** Use only when profiling proves necessity (e.g., custom allocators/intrusive structures). Keep unsafe blocks small, documented with invariants, and covered by heavy tests/fuzzing. Safe wrappers should expose the API.

### 77) What is interior mutability and when to avoid it?
**Answer:** Interior mutability (`RefCell`, `Cell`, `Mutex`) allows mutation behind shared references. It is useful in controlled cases, but excessive use can hide ownership design issues and introduce runtime overhead/risks in critical paths.

### 78) How would you model order state transitions idiomatically?
**Answer:** Use enums for state representation plus validated transition functions. Invalid transitions return explicit errors. This creates readable, compile-time guided logic and supports deterministic testing.

### 79) What is the role of `Result` and error typing in HFT services?
**Answer:** Rich typed errors improve observability and control flow, enabling precise reject reasons and retry policies. Avoid opaque string errors in core systems; they reduce debuggability and policy precision.

### 80) How do you manage config safely in Rust HFT apps?
**Answer:** Parse strongly typed config at startup, validate constraints early, and treat runtime mutability carefully. Critical controls should be explicit and auditable, not silently changed through ad hoc environment overrides.

## Testing, Verification, and Benchmarking

### 81) What is deterministic replay testing?
**Answer:** Feed recorded event/command streams into the engine and verify output events and resulting state are identical across runs and builds. It validates correctness, regression safety, and recoverability assumptions.

### 82) Why use property-based tests in matching systems?
**Answer:** They generate broad input spaces and uncover edge cases humans miss. For matching, properties like conservation of quantity, FIFO order, and no invalid transitions are ideal property candidates.

### 83) How would you fuzz an exchange adapter?
**Answer:** Fuzz parser/decoder inputs, malformed packets, partial messages, and sequence anomalies. Validate that adapter never panics, emits safe errors, and preserves ordering/metadata contracts.

### 84) What is a good benchmark matrix for HFT engines?
**Answer:** Include normal flow, burst flow, cancel storms, mixed order types, deep sweep scenarios, and degraded-mode behavior. Benchmark by symbol profile and depth regime, not one generic case.

### 85) How do you validate no hidden allocations in hot paths?
**Answer:** Use profiling/instrumentation tools and targeted tests under load. Review code for formatting/cloning patterns and container growth behavior. Pre-size and reuse structures where possible.

### 86) Why should CI include latency regression checks?
**Answer:** Functional correctness alone does not protect trading quality. Small code changes can significantly worsen tail latency. Automated thresholds prevent silent performance decay.

### 87) What is the value of long-running soak tests?
**Answer:** Soak tests reveal memory growth, rare timing bugs, queue drift, and stability issues not visible in short runs. HFT systems must remain predictable over full trading sessions, not just short benchmarks.

### 88) How do you test recovery from crash mid-session?
**Answer:** Simulate crash at controlled offsets, restart from snapshot + log replay, and verify state/event equivalence against uninterrupted baseline. Confirm idempotent handling of potentially redelivered commands.

### 89) What does “benchmark representativeness” mean?
**Answer:** It means test loads match real production patterns: message mix, burstiness, symbol concentration, and exchange behavior. Unrepresentative benchmarks can optimize the wrong bottlenecks.

### 90) What should be in a benchmark report for interviews?
**Answer:** Include hardware/setup, workloads, percentile latencies, throughput, regression comparisons, and known limits. Show concrete optimization decisions with before/after evidence and tradeoff discussion.

## Accounting, TigerBeetle Integration, and Practical Interview Delivery

### 91) Why integrate TigerBeetle in an HFT stack?
**Answer:** TigerBeetle provides high-performance double-entry accounting with strong correctness properties for financial posting. It is ideal for keeping settlement/balance truth robust under retries and failures, while execution remains low-latency and operationally decoupled.

### 92) How do you map execution events to ledger transfers?
**Answer:** Define deterministic mapping rules: reserve funds on order placement, move reserved-to-executed on fill, release reserve on cancel, and post fees separately. Every transfer gets stable IDs linking back to order/fill identifiers.

### 93) How do you prevent double posting during retries?
**Answer:** Use stable transfer IDs and dedupe checks before posting. On timeout ambiguity, query status or apply idempotent re-submit patterns. Reconciliation jobs verify eventual consistency between execution and ledger state.

### 94) What’s the difference between operational and financial truth?
**Answer:** Operational truth is immediate in-memory state used for risk/execution decisions. Financial truth is persisted double-entry state used for balances, reporting, and settlement guarantees. Systems must reconcile the two continuously.

### 95) What if matching succeeds but ledger is temporarily unavailable?
**Answer:** Persist unposted ledger events durably, retry with idempotency, and monitor posting lag. Apply policy limits (e.g., restrict withdrawals or tighten risk) if reconciliation lag exceeds thresholds. Never lose execution truth.

### 96) How would you explain your architecture in 5 minutes?
**Answer:** Start with objectives/SLOs, then show partitioned single-writer hot path, risk gates, execution adapters, and asynchronous ledger/reconciliation. Finish with failure handling, replay model, and measured latency evidence. Keep it ownership-focused and concrete.

### 97) What common interview mistake should you avoid?
**Answer:** Avoid giving only high-level diagrams without invariants, failure behavior, or measurable targets. Interviewers look for engineering depth: exact ordering guarantees, backpressure policy, replay strategy, and tradeoff justification.

### 98) How do you answer “what tradeoffs did you make?”
**Answer:** State choice, rejected alternative, reason, and impact. Example: single-writer partitions chosen over shared-lock model for deterministic tail latency; tradeoff is more complex partition management and cross-partition aggregation complexity.

### 99) How do you show production readiness in an interview?
**Answer:** Present concrete artifacts: architecture docs, invariants, benchmark reports, chaos/recovery tests, and postmortems. Demonstrate not only algorithm correctness but operational discipline and evidence-driven improvements.

### 100) What final checklist proves HFT + Rust interview readiness?
**Answer:** You can: implement/whiteboard matching logic quickly, explain partitioning and ordering guarantees, discuss risk fail-closed behavior, detail replay and idempotency, defend Rust design choices for latency, and present benchmark + incident evidence clearly.

## Related Notes

- [[65 - HFT Rust System Design Master Note]]
- [[66 - HFT Rust 30-Minute Interview Cheat Sheet]]
