//! Benchmark scenarios for matching-engine throughput/latency profiles.
//!
//! Includes:
//! - direct command submission path
//! - ring-buffer ingress drain path
//!
//! Goal:
//! - compare overhead characteristics of direct API calls vs queue handoff model.

use criterion::{criterion_group, criterion_main, Criterion};
use hft_matching_engine_rs::{
    IdempotencyKey, MatchingEngine, OrderCommand, OrderId, OrderType, Side, SpscRingBuffer,
    TimeInForce,
};

fn bench_limit_orders(c: &mut Criterion) {
    c.bench_function("match_limit_flow", |b| {
        b.iter(|| {
            let mut engine = MatchingEngine::default();
            for i in 0..5_000_u64 {
                let cmd = OrderCommand {
                    idempotency_key: IdempotencyKey(i as u128 + 1),
                    order_id: OrderId(i + 1),
                    side: if i % 2 == 0 { Side::Buy } else { Side::Sell },
                    order_type: OrderType::Limit,
                    tif: TimeInForce::Gtc,
                    price_ticks: 10_000 + (i % 32),
                    quantity: 10 + (i % 8),
                };
                let _ = engine.on_command(cmd);
            }
        });
    });

    c.bench_function("match_limit_flow_ring_buffer_ingress", |b| {
        b.iter(|| {
            let mut engine = MatchingEngine::default();
            let ingress = SpscRingBuffer::with_capacity(8_192);

            for i in 0..5_000_u64 {
                let cmd = OrderCommand {
                    idempotency_key: IdempotencyKey(i as u128 + 1),
                    order_id: OrderId(i + 1),
                    side: if i % 2 == 0 { Side::Buy } else { Side::Sell },
                    order_type: OrderType::Limit,
                    tif: TimeInForce::Gtc,
                    price_ticks: 10_000 + (i % 32),
                    quantity: 10 + (i % 8),
                };
                ingress.push(cmd).expect("ring buffer capacity");
            }

            let _events = engine.drain_ingress(&ingress);
        });
    });
}

criterion_group!(benches, bench_limit_orders);
criterion_main!(benches);
