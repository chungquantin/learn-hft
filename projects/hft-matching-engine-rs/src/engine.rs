use std::collections::HashSet;

use crate::command::{IdempotencyKey, OrderCommand};
use crate::event::ExecutionEvent;
use crate::matcher::match_command;
use crate::orderbook::OrderBook;

pub struct MatchingEngine {
    pub book: OrderBook,
    seen: HashSet<IdempotencyKey>,
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
    pub fn on_command(&mut self, cmd: OrderCommand) -> Vec<ExecutionEvent> {
        if self.seen.contains(&cmd.idempotency_key) {
            return Vec::new();
        }
        self.seen.insert(cmd.idempotency_key);

        let (next_seq, events) = match_command(self.next_seq, &mut self.book, cmd);
        self.next_seq = next_seq;
        events
    }
}
