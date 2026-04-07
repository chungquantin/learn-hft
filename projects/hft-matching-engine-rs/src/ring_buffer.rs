//! Bounded single-producer/single-consumer ring buffer.
//!
//! This queue is intended for low-latency ingress handoff:
//! - producer thread enqueues commands
//! - matching thread dequeues and processes in-order
//!
//! The implementation uses atomics and a fixed-size power-of-two buffer to keep
//! indexing cheap and predictable.
//!
//! # Design rationale
//! - Bounded capacity gives explicit backpressure (no unbounded memory growth).
//! - SPSC contract avoids mutex overhead in hot handoff paths.
//! - Power-of-two capacity enables fast index masking (`idx = cursor & mask`).
//!
//! # Memory ordering model
//! - Producer writes slot, then `Release` stores tail.
//! - Consumer `Acquire` loads tail before reading slot.
//! - Consumer reads slot, then `Release` stores head.
//!
//! # Example (conceptual)
//! ```text
//! producer: push(cmd_1), push(cmd_2)
//! consumer: pop() => cmd_1, pop() => cmd_2
//! ```

use std::cell::UnsafeCell;
use std::mem::MaybeUninit;
use std::sync::atomic::{AtomicUsize, Ordering};

/// Lock-free style bounded SPSC ring buffer.
///
/// Thread-safety contract:
/// - exactly one producer thread calls `push`
/// - exactly one consumer thread calls `pop`
/// - additional producers/consumers are not supported
pub struct SpscRingBuffer<T> {
    // Bitmask for modulo indexing. Requires power-of-two capacity.
    mask: usize,
    // Consumer cursor (next readable sequence).
    head: AtomicUsize,
    // Producer cursor (next writable sequence).
    tail: AtomicUsize,
    // Raw storage slots; UnsafeCell enables interior mutation with atomics
    // coordinating visibility/ownership between threads.
    slots: Box<[UnsafeCell<MaybeUninit<T>>]>,
}

impl<T> SpscRingBuffer<T> {
    /// Creates a new ring buffer with a fixed power-of-two capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        assert!(capacity.is_power_of_two(), "capacity must be power of two");
        assert!(capacity >= 2, "capacity must be at least 2");

        let mut slots = Vec::with_capacity(capacity);
        for _ in 0..capacity {
            slots.push(UnsafeCell::new(MaybeUninit::uninit()));
        }

        Self {
            mask: capacity - 1,
            head: AtomicUsize::new(0),
            tail: AtomicUsize::new(0),
            slots: slots.into_boxed_slice(),
        }
    }

    /// Returns current queue length estimate.
    pub fn len(&self) -> usize {
        // Acquire loads ensure we observe a consistent head/tail relationship.
        let head = self.head.load(Ordering::Acquire);
        let tail = self.tail.load(Ordering::Acquire);
        tail.wrapping_sub(head)
    }

    /// Returns `true` when queue has no items.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns total ring capacity.
    pub fn capacity(&self) -> usize {
        self.mask + 1
    }

    /// Attempts to enqueue one item.
    ///
    /// Returns the original item when queue is full.
    pub fn push(&self, value: T) -> Result<(), T> {
        // Producer can use Relaxed for own cursor read; ordering against consumer
        // is established by Acquire on head and Release on tail publish.
        let tail = self.tail.load(Ordering::Relaxed);
        let head = self.head.load(Ordering::Acquire);

        if tail.wrapping_sub(head) == self.capacity() {
            return Err(value);
        }

        let idx = tail & self.mask;
        // SAFETY: SPSC contract ensures producer has exclusive write access for this slot
        // before tail is published.
        unsafe {
            (*self.slots[idx].get()).write(value);
        }
        // Publish new tail with Release so consumer sees initialized slot contents.
        self.tail.store(tail.wrapping_add(1), Ordering::Release);
        Ok(())
    }

    /// Attempts to dequeue one item.
    pub fn pop(&self) -> Option<T> {
        // Consumer reads own cursor with Relaxed; synchronizes with producer via
        // Acquire on tail and Release on head store.
        let head = self.head.load(Ordering::Relaxed);
        let tail = self.tail.load(Ordering::Acquire);
        if head == tail {
            return None;
        }

        let idx = head & self.mask;
        // SAFETY: SPSC contract and head/tail ordering ensure this slot is initialized
        // and exclusively owned by the consumer at this point.
        let value = unsafe { (*self.slots[idx].get()).assume_init_read() };
        // Release publishes slot ownership back to producer side.
        self.head.store(head.wrapping_add(1), Ordering::Release);
        Some(value)
    }
}

impl<T> Drop for SpscRingBuffer<T> {
    fn drop(&mut self) {
        let mut head = self.head.load(Ordering::Relaxed);
        let tail = self.tail.load(Ordering::Relaxed);
        while head != tail {
            let idx = head & self.mask;
            // SAFETY: elements in [head, tail) are initialized by producer and
            // not yet consumed by consumer.
            unsafe {
                (*self.slots[idx].get()).assume_init_drop();
            }
            head = head.wrapping_add(1);
        }
    }
}

// SAFETY: SpscRingBuffer enforces internal synchronization with atomics and the
// SPSC usage contract. T must be Send to cross thread boundary.
unsafe impl<T: Send> Send for SpscRingBuffer<T> {}
// SAFETY: Shared references are safe under SPSC contract and atomic coordination.
unsafe impl<T: Send> Sync for SpscRingBuffer<T> {}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use std::thread;

    use super::SpscRingBuffer;

    #[test]
    fn push_pop_roundtrip() {
        let q = SpscRingBuffer::with_capacity(8);
        assert!(q.is_empty());
        q.push(10_u64).expect("push 10");
        q.push(11_u64).expect("push 11");
        assert_eq!(q.pop(), Some(10));
        assert_eq!(q.pop(), Some(11));
        assert_eq!(q.pop(), None);
    }

    #[test]
    fn concurrent_spsc_flow() {
        let q = Arc::new(SpscRingBuffer::with_capacity(1024));
        let producer_q = Arc::clone(&q);
        let consumer_q = Arc::clone(&q);

        let n = 50_000_u64;
        let producer = thread::spawn(move || {
            for i in 0..n {
                loop {
                    if producer_q.push(i).is_ok() {
                        break;
                    }
                    std::hint::spin_loop();
                }
            }
        });

        let consumer = thread::spawn(move || {
            let mut expected = 0_u64;
            while expected < n {
                match consumer_q.pop() {
                    Some(v) => {
                        assert_eq!(v, expected);
                        expected += 1;
                    }
                    None => std::hint::spin_loop(),
                }
            }
        });

        producer.join().expect("producer join");
        consumer.join().expect("consumer join");
    }
}
