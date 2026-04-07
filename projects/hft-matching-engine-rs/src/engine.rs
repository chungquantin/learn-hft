//! Matching engine state and thread-safe orchestration helpers.
//!
//! # Design
//! - `MatchingEngine` is single-writer by default for deterministic ordering.
//! - `ConcurrentMatchingEngine` wraps it with `Arc<Mutex<...>>` for integration.
//! - Idempotency dedupe protects against duplicate command delivery.
//!
//! # How it works
//! - `process(EngineCommand)` handles `New`, `Cancel`, `Replace`.
//! - Event sequence is advanced in one place (`emit_event`) to prevent drift.
//! - Drain helpers batch ingress consumption for better throughput.
//!
//! # Example (conceptual)
//! ```text
//! process(New) -> Accepted + (Trade|Rested...)
//! process(Cancel) -> Canceled or Rejected
//! process(Replace) -> Canceled/Rejected + New-order event sequence
//! ```

use std::collections::HashSet;
use std::sync::{Arc, Mutex};

use crate::command::{EngineCommand, IdempotencyKey, OrderCommand};
use crate::event::{ExecutionEvent, ExecutionEventKind};
use crate::matcher::match_command;
use crate::orderbook::OrderBook;
use crate::ring_buffer::SpscRingBuffer;

/// Single-partition matching engine.
///
/// This type is intentionally single-writer for deterministic ordering and lower jitter.
pub struct MatchingEngine {
    /// Heap-backed order book state.
    pub book: OrderBook,
    // Dedupe set for idempotency keys.
    //
    // Why HashSet:
    // - expected O(1) membership checks
    // - simple correctness mechanism for duplicate delivery suppression
    seen: HashSet<IdempotencyKey>,
    // Partition-local monotonic event sequence.
    next_seq: u64,
}

impl Default for MatchingEngine {
    fn default() -> Self {
        Self {
            book: OrderBook::default(),
            seen: HashSet::new(),
            next_seq: 1,
        }
    }
}

impl MatchingEngine {
    /// Builds one event and increments sequence counter.
    ///
    /// Keeping this helper central avoids sequence drift bugs when adding
    /// new command variants.
    fn emit_event(
        &mut self,
        order_id: crate::types::OrderId,
        kind: ExecutionEventKind,
        price_ticks: u64,
        quantity: u64,
    ) -> ExecutionEvent {
        // Capture current sequence for event, then advance for next emission.
        let seq = self.next_seq;
        self.next_seq = self.next_seq.wrapping_add(1);

        ExecutionEvent {
            seq,
            order_id,
            kind,
            price_ticks,
            quantity,
        }
    }

    /// Handles new order path without dedupe checks.
    ///
    /// This is used internally by both direct `on_command` and by `Replace`.
    fn process_new_without_dedupe(&mut self, cmd: OrderCommand) -> Vec<ExecutionEvent> {
        // `match_command` returns updated sequence and generated events.
        let (next_seq, events) = match_command(self.next_seq, &mut self.book, cmd);
        // Persist new sequence so future emissions remain monotonic.
        self.next_seq = next_seq;
        events
    }

    /// Processes any engine command type (`New`, `Cancel`, `Replace`).
    pub fn process(&mut self, cmd: EngineCommand) -> Vec<ExecutionEvent> {
        // Global dedupe: same idempotency key always maps to same side effect (no-op).
        if self.seen.contains(&cmd.idempotency_key()) {
            return Vec::new();
        }
        self.seen.insert(cmd.idempotency_key());

        match cmd {
            EngineCommand::New(order_cmd) => self.process_new_without_dedupe(order_cmd),
            EngineCommand::Cancel { order_id, .. } => {
                // Cancel only succeeds for resting orders currently present in book index.
                if let Some(removed) = self.book.cancel_order(order_id) {
                    vec![self.emit_event(
                        removed.order_id,
                        ExecutionEventKind::Canceled,
                        removed.price_ticks,
                        removed.quantity,
                    )]
                } else {
                    // Emit reject-like signal for observability when cancel target is absent.
                    // We use quantity/price as zero placeholders for non-fill rejection.
                    vec![self.emit_event(order_id, ExecutionEventKind::Rejected, 0, 0)]
                }
            }
            EngineCommand::Replace {
                cancel_order_id,
                new_order,
                ..
            } => {
                // Replace semantics:
                // 1) attempt cancel old resting order
                // 2) submit new order as fresh time-priority entry
                let mut events = Vec::with_capacity(8);
                if let Some(removed) = self.book.cancel_order(cancel_order_id) {
                    events.push(self.emit_event(
                        removed.order_id,
                        ExecutionEventKind::Canceled,
                        removed.price_ticks,
                        removed.quantity,
                    ));
                } else {
                    events.push(self.emit_event(
                        cancel_order_id,
                        ExecutionEventKind::Rejected,
                        0,
                        0,
                    ));
                }
                // New order in replace gets fresh queue time semantics by design.
                events.extend(self.process_new_without_dedupe(new_order));
                events
            }
        }
    }

    /// Processes one order command and returns emitted execution events.
    pub fn on_command(&mut self, cmd: OrderCommand) -> Vec<ExecutionEvent> {
        // Backward-compatible wrapper for callers that only submit new orders.
        self.process(EngineCommand::New(cmd))
    }

    /// Drains all available commands from an SPSC ring buffer and matches them in-order.
    pub fn drain_ingress(&mut self, ingress: &SpscRingBuffer<OrderCommand>) -> Vec<ExecutionEvent> {
        // Batch draining amortizes lock/dispatch overhead in runtime loops.
        let mut out = Vec::with_capacity(256);
        while let Some(cmd) = ingress.pop() {
            out.extend(self.on_command(cmd));
        }
        out
    }

    /// Drains all available engine commands from an SPSC ring buffer.
    pub fn drain_command_ingress(
        &mut self,
        ingress: &SpscRingBuffer<EngineCommand>,
    ) -> Vec<ExecutionEvent> {
        // Single consumer loop preserves ingress order exactly.
        let mut out = Vec::with_capacity(256);
        while let Some(cmd) = ingress.pop() {
            out.extend(self.process(cmd));
        }
        out
    }
}

/// Thread-safe wrapper for shared runtime integration.
///
/// Internal matching still follows single-writer semantics by locking around command handling.
#[derive(Clone, Default)]
pub struct ConcurrentMatchingEngine {
    inner: Arc<Mutex<MatchingEngine>>,
}

impl ConcurrentMatchingEngine {
    /// Creates a new thread-safe engine wrapper.
    pub fn new() -> Self {
        Self::default()
    }

    /// Processes a command with mutual exclusion, preserving deterministic sequence semantics.
    pub fn on_command(&self, cmd: OrderCommand) -> Vec<ExecutionEvent> {
        let mut guard = self.inner.lock().expect("matching engine lock poisoned");
        guard.on_command(cmd)
    }

    /// Processes a general engine command under mutual exclusion.
    pub fn process(&self, cmd: EngineCommand) -> Vec<ExecutionEvent> {
        let mut guard = self.inner.lock().expect("matching engine lock poisoned");
        guard.process(cmd)
    }

    /// Drains all available ingress commands from a shared ring buffer.
    pub fn drain_ingress(&self, ingress: &SpscRingBuffer<OrderCommand>) -> Vec<ExecutionEvent> {
        let mut guard = self.inner.lock().expect("matching engine lock poisoned");
        guard.drain_ingress(ingress)
    }

    /// Drains command ingress ring under mutual exclusion.
    pub fn drain_command_ingress(
        &self,
        ingress: &SpscRingBuffer<EngineCommand>,
    ) -> Vec<ExecutionEvent> {
        let mut guard = self.inner.lock().expect("matching engine lock poisoned");
        guard.drain_command_ingress(ingress)
    }
}

#[cfg(test)]
mod tests {
    use crate::types::{OrderId, OrderType, Side, TimeInForce};

    use super::*;

    fn mk_new(idem: u128, order_id: u64, side: Side, price: u64, qty: u64) -> EngineCommand {
        EngineCommand::New(OrderCommand {
            idempotency_key: IdempotencyKey(idem),
            order_id: OrderId(order_id),
            side,
            order_type: OrderType::Limit,
            tif: TimeInForce::Gtc,
            price_ticks: price,
            quantity: qty,
        })
    }

    #[test]
    fn cancel_removes_resting_order() {
        let mut eng = MatchingEngine::default();
        let _ = eng.process(mk_new(1, 10, Side::Buy, 100, 5));
        let events = eng.process(EngineCommand::Cancel {
            idempotency_key: IdempotencyKey(2),
            order_id: OrderId(10),
        });
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].kind, ExecutionEventKind::Canceled);
    }

    #[test]
    fn replace_cancels_then_submits_new() {
        let mut eng = MatchingEngine::default();
        let _ = eng.process(mk_new(1, 10, Side::Buy, 100, 5));
        let events = eng.process(EngineCommand::Replace {
            idempotency_key: IdempotencyKey(3),
            cancel_order_id: OrderId(10),
            new_order: OrderCommand {
                idempotency_key: IdempotencyKey(999),
                order_id: OrderId(11),
                side: Side::Buy,
                order_type: OrderType::Limit,
                tif: TimeInForce::Gtc,
                price_ticks: 101,
                quantity: 5,
            },
        });
        assert!(events.iter().any(|e| e.kind == ExecutionEventKind::Canceled));
        assert!(events.iter().any(|e| e.order_id == OrderId(11)));
    }
}
