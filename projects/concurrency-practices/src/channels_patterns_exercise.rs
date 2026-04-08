//! Exercise: `channels_patterns`
//!
//! Goal:
//! - Practice fan-in, fan-out, and backpressure design.
//!
//! Instructions:
//! 1. Build a fan-in pipeline where multiple producers feed one consumer.
//! 2. Add bounded channel capacity and observe backpressure.
//! 3. Implement graceful close semantics when producers finish.
//! 4. Add message IDs and verify in-order handling per producer.
//! 5. Add one slow consumer scenario and record queue lag metrics.
//! 6. Write tests for no message loss/duplication under load.
//!
//! Method Hints:
//! - `std::sync::mpsc` or `crossbeam-channel`
//! - Bounded channels for explicit backpressure
//! - Sender drop semantics to signal completion
//! - Sequence IDs per producer for ordering checks

use std::collections::HashMap;
use std::time::{Duration, Instant};
use std::{sync::mpsc, thread};

fn solution_1() {
    let (sender, receiver) = mpsc::channel();

    for i in 0..10 {
        let s = sender.clone();
        thread::spawn(move || {
            s.send(10 * i).unwrap();
        });
    }

    while let Ok(msg) = receiver.recv() {
        println!("message: {:?}", msg);
    }
}

fn solution_2() {
    let (tx, rv) = mpsc::sync_channel(5);

    for i in 0..10 {
        let s = tx.clone();
        thread::spawn(move || match s.send(10 * i) {
            Ok(_) => {
                println!("Send successfully");
            }
            Err(err) => {
                println!("Error sending message: {:?}", err)
            }
        });
    }

    while let Ok(msg) = rv.recv() {
        println!("message: {:?}", msg);
    }
}

fn solution_3() {
    let (tx, rv) = mpsc::sync_channel(5);

    for i in 0..10 {
        let s = tx.clone();
        thread::spawn(move || match s.send(10 * i) {
            Ok(_) => {
                println!("Send successfully");
            }
            Err(err) => {
                println!("Error sending message: {:?}", err)
            }
        });
    }

    drop(tx);

    loop {
        match rv.recv() {
            Ok(msg) => println!("message: {:?}", msg),
            Err(err) => {
                println!("channel error: {:?}", err);
                break;
            }
        }
    }
}

#[derive(Debug)]
struct Message {
    producer_id: usize,
    sequence_id: usize,
    sent_at: Instant,
}

pub fn run_complex_pattern() {
    // 2. Bounded channel (Backpressure)
    let (tx, rx) = mpsc::sync_channel(5);
    let mut handles = Vec::new();

    // 1. Fan-out Producers
    for p_id in 0..3 {
        let tx_clone = tx.clone();
        handles.push(thread::spawn(move || {
            for seq_id in 0..10 {
                let msg = Message {
                    producer_id: p_id,
                    sequence_id: seq_id,
                    sent_at: Instant::now(),
                };
                tx_clone.send(msg).unwrap();
            }
        }));
    }

    // 3. Graceful Close: Drop the extra sender
    drop(tx);

    // 4 & 5. Consumer with Lag Tracking and Ordering Check
    let mut last_seen_ids = HashMap::new();
    let mut total_lag = Duration::ZERO;
    let mut count = 0;

    for msg in rx {
        count += 1;
        // Verify Order
        let last_id = last_seen_ids.entry(msg.producer_id).or_insert(0);
        if msg.sequence_id < *last_id && count > 1 {
            panic!(
                "OUT OF ORDER: Producer {} sent {} after {}",
                msg.producer_id, msg.sequence_id, last_id
            );
        }
        *last_id = msg.sequence_id;

        // Record Lag
        let lag = msg.sent_at.elapsed();
        total_lag += lag;

        // Simulate Slow Consumer (Goal 5)
        thread::sleep(Duration::from_millis(50));

        println!(
            "[Consumer] Recv P{}:ID{} | Current Lag: {:?}",
            msg.producer_id, msg.sequence_id, lag
        );
    }

    println!("Average Queue Lag: {:?}", total_lag / count);
}

#[cfg(test)]
mod tests {
    use crate::channels_patterns_exercise::{
        run_complex_pattern, solution_1, solution_2, solution_3,
    };

    #[test]
    fn test_1() {
        solution_1();
    }

    #[test]
    fn test_2() {
        solution_2();
    }

    #[test]
    fn test_3() {
        solution_3();
    }

    #[test]
    fn test_complex_channel_case() {
        run_complex_pattern();
    }
}
