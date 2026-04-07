//! Partitioned runtime for horizontal concurrency.
//!
//! Design:
//! - Commands are routed by primary order id to one partition.
//! - Each partition has:
//!   - one ingress SPSC ring buffer
//!   - one matching engine instance
//! - Determinism is preserved per partition by single-consumer drain order.
//!
//! # Why this model
//! - Avoids shared mutable-state contention across instruments/orders.
//! - Scales via many independent shards instead of one globally locked engine.
//! - Keeps ordering guarantees explicit: deterministic within partition.
//!
//! # Example (conceptual)
//! ```text
//! partitions = 4
//! order_id 9  -> shard 1
//! order_id 22 -> shard 2
//! each shard drains independently
//! ```

use std::sync::Arc;

use crate::command::EngineCommand;
use crate::engine::ConcurrentMatchingEngine;
use crate::event::ExecutionEvent;
use crate::ring_buffer::SpscRingBuffer;
use tracing::{debug, trace};

/// One partition shard: ingress queue + engine.
#[derive(Clone)]
pub struct PartitionShard {
    /// Bounded ingress queue for this shard.
    pub ingress: Arc<SpscRingBuffer<EngineCommand>>,
    /// Thread-safe wrapper around shard-local engine state.
    pub engine: ConcurrentMatchingEngine,
}

/// Multi-partition matching runtime.
pub struct PartitionRuntime {
    // Fixed shard vector keeps routing deterministic and allocation stable.
    shards: Vec<PartitionShard>,
}

impl PartitionRuntime {
    /// Creates `num_partitions` independent engine shards.
    pub fn new(num_partitions: usize, ingress_capacity_per_partition: usize) -> Self {
        assert!(num_partitions > 0, "num_partitions must be > 0");
        let mut shards = Vec::with_capacity(num_partitions);
        for _ in 0..num_partitions {
            shards.push(PartitionShard {
                ingress: Arc::new(SpscRingBuffer::with_capacity(
                    ingress_capacity_per_partition,
                )),
                engine: ConcurrentMatchingEngine::new(),
            });
        }
        Self { shards }
    }

    /// Returns number of configured partitions.
    pub fn partition_count(&self) -> usize {
        self.shards.len()
    }

    /// Returns deterministic partition index for command routing.
    pub fn route_partition(&self, cmd: &EngineCommand) -> usize {
        // Modulo partitioning by order id gives stable routing and preserves
        // per-order in-partition sequencing.
        //
        // This is intentionally simple and fast; advanced runtimes may use
        // consistent hashing or symbol maps for better load distribution.
        (cmd.primary_order_id().0 as usize) % self.shards.len()
    }

    /// Enqueues command into its target partition.
    ///
    /// Returns the original command if queue is full.
    pub fn enqueue(&self, cmd: EngineCommand) -> Result<(), EngineCommand> {
        let idx = self.route_partition(&cmd);
        trace!(partition = idx, ?cmd, "enqueue command to partition ingress");
        self.shards[idx].ingress.push(cmd)
    }

    /// Drains one partition and returns produced events.
    pub fn drain_partition(&self, idx: usize) -> Vec<ExecutionEvent> {
        let events = self.shards[idx]
            .engine
            .drain_command_ingress(&self.shards[idx].ingress);
        debug!(
            partition = idx,
            emitted_events = events.len(),
            "drained partition ingress"
        );
        events
    }

    /// Drains all partitions and returns combined event stream.
    ///
    /// Note: cross-partition order is not globally deterministic by design.
    pub fn drain_all(&self) -> Vec<ExecutionEvent> {
        let mut out = Vec::with_capacity(1024);
        for idx in 0..self.shards.len() {
            // Draining in fixed shard order keeps output order deterministic
            // for a given runtime schedule, though no global cross-partition
            // sequencing guarantee is implied.
            out.extend(self.drain_partition(idx));
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use crate::command::{EngineCommand, IdempotencyKey, OrderCommand};
    use crate::types::{OrderId, OrderType, Side, TimeInForce};

    use super::PartitionRuntime;

    #[test]
    fn route_is_stable_for_same_order() {
        let rt = PartitionRuntime::new(4, 64);
        let cmd = EngineCommand::New(OrderCommand {
            idempotency_key: IdempotencyKey(1),
            order_id: OrderId(123),
            side: Side::Buy,
            order_type: OrderType::Limit,
            tif: TimeInForce::Gtc,
            price_ticks: 100,
            quantity: 1,
        });
        let p1 = rt.route_partition(&cmd);
        let p2 = rt.route_partition(&cmd);
        assert_eq!(p1, p2);
    }
}
