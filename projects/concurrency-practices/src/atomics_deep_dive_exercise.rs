//! Exercise: `atomics_deep_dive`
//!
//! Goal:
//! - Deepen understanding of memory ordering and lock-free patterns.
//!
//! Instructions:
//! 1. Rebuild a small producer/consumer handshake using atomics only.
//! 2. Implement one version with overly-strong ordering (`SeqCst`).
//! 3. Implement a second version with minimized orderings (`Acquire/Release`).
//! 4. Explain why weaker orderings are still correct.
//! 5. Add stress tests with high iteration counts to look for rare races.
//! 6. Document the linearization point for each operation.
//!
//! Method Hints:
//! - `compare_exchange` / `compare_exchange_weak`
//! - `fetch_update`
//! - `spin_loop` and bounded retry loops
//! - Pairing `Release` writes with `Acquire` reads
