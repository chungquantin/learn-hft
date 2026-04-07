use criterion::{criterion_group, criterion_main, Criterion};
use hft_matching_engine_rs::{
    IdempotencyKey, MatchingEngine, OrderCommand, OrderId, OrderType, Side, TimeInForce,
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
}

criterion_group!(benches, bench_limit_orders);
criterion_main!(benches);
