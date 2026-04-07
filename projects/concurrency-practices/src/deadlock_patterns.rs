//! Deadlock patterns and avoidance techniques.
//!
//! Instead of intentionally hanging tests, this module demonstrates safe
//! lock-ordering patterns that prevent deadlocks in real systems.
//!
//! Core principle:
//! - if multiple locks are required, acquire them in a globally consistent order.

use std::sync::{Arc, Mutex};
use std::thread;

/// Shared account state for transfer demos.
#[derive(Debug)]
pub struct Account {
    pub id: usize,
    pub balance: i64,
}

/// Safe transfer by locking accounts in sorted-id order.
pub fn transfer_with_lock_ordering(
    a: &Arc<Mutex<Account>>,
    b: &Arc<Mutex<Account>>,
    amount: i64,
) -> bool {
    // First, inspect ids without keeping both locks.
    let a_id = a.lock().expect("lock poisoned").id;
    let b_id = b.lock().expect("lock poisoned").id;

    // Acquire in deterministic order by account id.
    if a_id <= b_id {
        let mut from = a.lock().expect("lock poisoned");
        let mut to = b.lock().expect("lock poisoned");
        if from.balance < amount {
            return false;
        }
        from.balance -= amount;
        to.balance += amount;
    } else {
        let mut to = b.lock().expect("lock poisoned");
        let mut from = a.lock().expect("lock poisoned");
        if from.balance < amount {
            return false;
        }
        from.balance -= amount;
        to.balance += amount;
    }
    true
}

/// Runs many concurrent transfers safely and returns final total balance.
pub fn concurrent_transfer_demo() -> i64 {
    let a = Arc::new(Mutex::new(Account { id: 1, balance: 1_000 }));
    let b = Arc::new(Mutex::new(Account { id: 2, balance: 1_000 }));

    let mut handles = Vec::new();
    for _ in 0..100 {
        let a1 = Arc::clone(&a);
        let b1 = Arc::clone(&b);
        handles.push(thread::spawn(move || {
            let _ = transfer_with_lock_ordering(&a1, &b1, 1);
        }));

        let a2 = Arc::clone(&a);
        let b2 = Arc::clone(&b);
        handles.push(thread::spawn(move || {
            let _ = transfer_with_lock_ordering(&b2, &a2, 1);
        }));
    }

    for h in handles {
        h.join().expect("transfer thread panicked");
    }

    let a_final = a.lock().expect("lock poisoned").balance;
    let b_final = b.lock().expect("lock poisoned").balance;
    a_final + b_final
}

#[cfg(test)]
mod tests {
    use super::{concurrent_transfer_demo, Account, transfer_with_lock_ordering};
    use std::sync::{Arc, Mutex};

    #[test]
    fn lock_ordering_transfer_preserves_money() {
        let a = Arc::new(Mutex::new(Account { id: 1, balance: 10 }));
        let b = Arc::new(Mutex::new(Account { id: 2, balance: 10 }));
        assert!(transfer_with_lock_ordering(&a, &b, 3));
        assert_eq!(a.lock().expect("lock poisoned").balance, 7);
        assert_eq!(b.lock().expect("lock poisoned").balance, 13);
    }

    #[test]
    fn concurrent_transfers_keep_total_balance_constant() {
        assert_eq!(concurrent_transfer_demo(), 2_000);
    }
}
