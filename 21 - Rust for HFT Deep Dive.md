---
title: Rust for HFT Deep Dive
tags: [rust, hft, systems, deep-dive]
---

# Rust for HFT Deep Dive

Rust does not make a strategy profitable. That is the first sentence worth keeping in mind because it prevents a common kind of confusion. People are often drawn to Rust in low-latency finance because it has the aura of seriousness: systems programming, performance, safety, modern tooling. All of that is true, but none of it automatically creates edge. What Rust can do is help you build a platform whose costs, invariants, and failure modes are easier to reason about. In HFT, that is extremely valuable because a small edge is easy to destroy with unclear system behavior.

The core optimization target in HFT is also often misunderstood. People say the field is about speed, but the more useful way to think about it is that it is about predictability under pressure. Low latency matters, of course, but so do low jitter, high throughput, deterministic state transitions, and operational safety. A system that is fast on average but occasionally pauses, allocates heavily in bursts, or becomes semantically unclear during load spikes is not really behaving well enough for short-horizon trading. Tail behavior matters more than people expect.

This is one reason garbage-collected languages are treated cautiously in these environments. They may deliver excellent average throughput while still introducing pauses or scheduling variability that become unacceptable once reaction timing is part of the edge. Rust, by avoiding a garbage collector and exposing more of the memory model directly, gives you a better chance of seeing where cost and unpredictability come from.

That phrase "seeing where cost comes from" is central. In low-latency Rust, the real cost model matters more than surface syntax. Some operations are naturally cheap: integer arithmetic, fixed-size stack operations, branch-light access to warm contiguous memory. Others are expensive or unpredictably expensive: heap allocation, deallocation, string formatting, locks, syscalls, cache misses, blocking I/O, page faults. Rust is useful partly because it makes many of these choices harder to hide from yourself. A `String` is not just text. It is probably allocation. A `Vec::push` is not just append. It may be reallocation if you lost track of capacity. A `clone()` is not merely convenience. It may be semantic duplication plus real cost.

That is why ownership in Rust should be understood as a design tool, not as a syntax hurdle. Ownership forces the system designer to answer questions that are already present in HFT, whether the language exposes them clearly or not. Who owns this state? Who is allowed to mutate it? Is it better represented as a local object, a message in motion, or a shared snapshot? Should the data be copied, borrowed, or moved? These are not language trivia. They are architecture questions wearing syntax.

One of the strongest rules of thumb that falls out of this is to prefer message passing over shared mutable state unless there is a clear reason not to. If the market-data thread owns parsing and normalization, if the strategy thread owns signal logic, and if the logging thread owns rendering and I/O, the system usually becomes both simpler and faster than if each of those concerns shares mutable structures with the others. Clear ownership has performance consequences because it reduces synchronization burden and conceptual consequences because it makes bugs easier to localize.

Data layout is another place where Rust becomes more than "a fast language." CPUs are extremely fast at arithmetic and surprisingly slow at waiting for memory. So low-latency performance depends heavily on whether structs are compact, whether storage is contiguous, whether access patterns are sequential, and whether pointer indirection is multiplying cache misses. This is why arrays, ring buffers, fixed-capacity structures, and flat message representations appear so often in HFT code. The algorithm is only part of the story. The memory geometry is part of the algorithm too.

Concurrency is often where beginners take the wrong lesson next. More threads do not automatically imply more performance. Every additional thread introduces scheduling risk, synchronization cost, cache-coherency traffic, and debugging complexity. The better mental model is not "parallelize everything" but "use isolation where it actually preserves the hot path." Splitting market data, strategy, execution, telemetry, and persistence may make sense when their latency profiles and ownership boundaries differ. Splitting because concurrency feels modern is a weaker reason.

This is also where the async versus dedicated-thread question becomes practical. Async Rust is excellent for many I/O-heavy situations, especially when you need to multiplex many connections. But the hottest event loop in an HFT path may still benefit from a dedicated thread with stable CPU and cache locality, less runtime indirection, and more explicit control over the shape of work. The right lesson is not to avoid async entirely. It is to avoid forcing the most timing-sensitive part of the engine into an abstraction that makes its behavior harder to reason about.

All of this converges into a simple principle: the hot path should do less. It should parse the event, update local state, compute the decision that matters, and hand off what comes next. It should not casually format strings, allocate arbitrarily, rebuild large structures, or drag observability and persistence into the same timing budget. Telemetry matters. Logging matters. Audit matters. But those belong to side paths designed not to distort the thing they are observing.

The language-boundary question matters here too. In many real stacks, Rust is not the only language. Python may own research notebooks, feature analysis, orchestration, and offline modeling. A compiled language owns the hot path. The important point is not ideological purity. It is choosing which layer needs ergonomics and which layer needs tight control over latency, memory, and concurrency. Rust fits especially well when you want one compiled language that can stay honest about ownership and data movement without inheriting garbage-collector risk.

Rust's `unsafe` facilities fit into the same philosophy. Unsafe code is not forbidden, but it is a debt instrument. It may be the right choice for some lock-free structures, FFI boundaries, or extremely tight data-path optimizations. But the correct order is safe Rust first, benchmarking second, and unsafe only when the bottleneck is measured and the invariants are explicit. Otherwise the codebase gradually stops being "safe Rust with a few sharp tools" and starts becoming "C++ with better branding."

Benchmarking itself deserves skepticism. Microbenchmarks are useful, but only if they reflect realistic data, realistic branch behavior, realistic memory warmth, and the right latency metric. Measuring only average throughput is an easy way to miss the problem you actually care about. Good benchmarking in this domain eventually expands from function-level timing to component benches, replay-driven system tests, and end-to-end latency profiling under load.

A useful benchmarking ladder in a Rust HFT project looks something like this:

1. Verify correctness and determinism first.
2. Run microbenchmarks on parsers, queues, and data structures.
3. Run component benchmarks with realistic message mixes and warm/cold conditions.
4. Run replay-driven benchmarks that include sequencing, state transitions, and telemetry handoff.
5. Run end-to-end latency measurements on the actual topology you intend to use.

Each level answers a different question. Microbenchmarks tell you whether a small implementation choice is expensive. Replay and end-to-end runs tell you whether the system shape is expensive. Confusing those levels is one reason teams optimize code that was never on the real critical path.

One final practical lesson is to distrust "optimization by folklore." Kernel bypass, lock-free structures, thread pinning, custom allocators, and unsafe data-path tricks are all real tools. But none of them should be adopted because they sound like HFT. They should be adopted because a measured bottleneck, on your workload and topology, says they buy something worth their complexity.

If there is one deeper lesson to internalize, it is this: Rust matters in HFT not because it is simply fast, but because it forces explicit choices about ownership, mutability, allocation, and concurrency. Those choices are already the real design problems. Rust just makes them harder to ignore.

Related:

- [[02 - Rust for HFT]]
- [[11 - Seqlocks]]
- [[24 - Queues, Ring Buffers, and Backpressure]]
- [[26 - Building a Low-Latency Trading Engine]]
