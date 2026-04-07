//! Simple simulation runner binary.
//!
//! Usage:
//! - `cargo run --bin sim`
//! - `RUST_LOG=hft_matching_engine_rs=trace cargo run --bin sim`

use hft_matching_engine_rs::{run_partitioned_simulation, SimulationConfig};

fn main() {
    let cfg = SimulationConfig::default();
    let report = run_partitioned_simulation(cfg);
    println!("Simulation report: {report:#?}");
}
