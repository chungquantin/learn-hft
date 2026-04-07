//! Deterministic replay log helpers.
//!
//! This module keeps an in-memory append-only command log and provides
//! rebuild helpers for testing and incident reconstruction.
//!
//! # Why replay matters
//! - Reproduces exact behavior after incidents.
//! - Validates that refactors preserve engine semantics.
//! - Supports deterministic regression tests.
//!
//! # How to use
//! 1. Append live or synthetic commands into the log.
//! 2. Call `rebuild()` to create fresh engine and replay all commands.
//! 3. Compare resulting events/state across builds/versions.
//!
//! # Example (conceptual)
//! ```text
//! log.append(New(...))
//! log.append(Cancel(...))
//! let (_engine, events) = log.rebuild()
//! ```

use crate::command::EngineCommand;
use crate::engine::MatchingEngine;
use crate::event::ExecutionEvent;

/// In-memory append-only command log.
#[derive(Default, Clone)]
pub struct InMemoryReplayLog {
    // Append-only command history.
    //
    // Vec is chosen for:
    // - compact contiguous storage
    // - fast iteration during replay
    // - deterministic ordering by append index
    commands: Vec<EngineCommand>,
}

impl InMemoryReplayLog {
    /// Appends one command to the log.
    pub fn append(&mut self, cmd: EngineCommand) {
        // Append-only writes preserve a single source of truth for command order.
        self.commands.push(cmd);
    }

    /// Returns immutable command slice for external inspection.
    pub fn commands(&self) -> &[EngineCommand] {
        &self.commands
    }

    /// Replays commands into a provided engine in the original order.
    pub fn replay_into(&self, engine: &mut MatchingEngine) -> Vec<ExecutionEvent> {
        // We keep replay order exactly identical to append order,
        // which is required for deterministic state reconstruction.
        let mut out = Vec::with_capacity(self.commands.len() * 2);
        for &cmd in &self.commands {
            // We call same production engine method (`process`) so replay and
            // live behavior share exactly the same transition logic.
            out.extend(engine.process(cmd));
        }
        out
    }

    /// Rebuilds a fresh engine from scratch and returns resulting events.
    pub fn rebuild(&self) -> (MatchingEngine, Vec<ExecutionEvent>) {
        let mut engine = MatchingEngine::default();
        let events = self.replay_into(&mut engine);
        (engine, events)
    }
}

#[cfg(test)]
mod tests {
    use crate::command::{EngineCommand, IdempotencyKey, OrderCommand};
    use crate::types::{OrderId, OrderType, Side, TimeInForce};

    use super::InMemoryReplayLog;

    #[test]
    fn replay_is_deterministic_for_same_log() {
        let mut log = InMemoryReplayLog::default();
        log.append(EngineCommand::New(OrderCommand {
            idempotency_key: IdempotencyKey(1),
            order_id: OrderId(1),
            side: Side::Buy,
            order_type: OrderType::Limit,
            tif: TimeInForce::Gtc,
            price_ticks: 100,
            quantity: 5,
        }));
        log.append(EngineCommand::Cancel {
            idempotency_key: IdempotencyKey(2),
            order_id: OrderId(1),
        });

        let (_a_engine, a_events) = log.rebuild();
        let (_b_engine, b_events) = log.rebuild();
        assert_eq!(a_events, b_events);
    }
}
