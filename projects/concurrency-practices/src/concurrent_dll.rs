//! Playground module: minimal concurrent doubly linked list.
//!
//! Design:
//! - node links (`prev`/`next`) are individually protected by `Mutex`
//! - node payload is protected by `RwLock`
//! - list structural mutation is serialized by a list-level `Mutex`
//!
//! This keeps operations simple and avoids deadlocks from multi-node lock ordering.

use std::sync::{Arc, Mutex, RwLock, RwLockReadGuard, RwLockWriteGuard, Weak};

pub struct ConcurrentDllNode<T: Send + Sync> {
    prev: Mutex<Option<Weak<ConcurrentDllNode<T>>>>,
    next: Mutex<Option<Arc<ConcurrentDllNode<T>>>>,
    value: RwLock<T>,
}

impl<T: Send + Sync> ConcurrentDllNode<T> {
    pub fn new(value: T) -> Arc<Self> {
        Arc::new(Self {
            prev: Mutex::new(None),
            next: Mutex::new(None),
            value: RwLock::new(value),
        })
    }

    pub fn read_value(&self) -> RwLockReadGuard<'_, T> {
        self.value.read().expect("node value lock poisoned")
    }

    pub fn write_value(&self) -> RwLockWriteGuard<'_, T> {
        self.value.write().expect("node value lock poisoned")
    }

    pub fn next(&self) -> Option<Arc<Self>> {
        self.next.lock().expect("node next lock poisoned").clone()
    }

    pub fn prev(&self) -> Option<Arc<Self>> {
        self.prev
            .lock()
            .expect("node prev lock poisoned")
            .as_ref()
            .and_then(Weak::upgrade)
    }
}

struct ConcurrentDllState<T: Send + Sync> {
    head: Option<Arc<ConcurrentDllNode<T>>>,
    tail: Option<Arc<ConcurrentDllNode<T>>>,
    len: usize,
}

pub struct ConcurrentDll<T: Send + Sync> {
    state: Mutex<ConcurrentDllState<T>>,
}

impl<T: Send + Sync> Default for ConcurrentDllState<T> {
    fn default() -> Self {
        Self {
            head: None,
            tail: None,
            len: 0,
        }
    }
}

impl<T: Send + Sync> Default for ConcurrentDll<T> {
    fn default() -> Self {
        Self {
            state: Mutex::new(ConcurrentDllState::default()),
        }
    }
}

impl<T: Send + Sync + Default> ConcurrentDll<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn len(&self) -> usize {
        self.state.lock().expect("list lock poisoned").len
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn head(&self) -> Option<Arc<ConcurrentDllNode<T>>> {
        self.state
            .lock()
            .expect("list lock poisoned")
            .head
            .as_ref()
            .cloned()
    }

    pub fn tail(&self) -> Option<Arc<ConcurrentDllNode<T>>> {
        self.state
            .lock()
            .expect("list lock poisoned")
            .tail
            .as_ref()
            .cloned()
    }

    pub fn push_front(&self, value: T) -> Arc<ConcurrentDllNode<T>> {
        let new_node = ConcurrentDllNode::new(value);
        let mut state = self.state.lock().expect("list lock poisoned");

        match state.head.take() {
            Some(old_head) => {
                *old_head.prev.lock().expect("node prev lock poisoned") =
                    Some(Arc::downgrade(&new_node));
                *new_node.next.lock().expect("node next lock poisoned") = Some(old_head);
                state.head = Some(Arc::clone(&new_node));
            }
            None => {
                state.head = Some(Arc::clone(&new_node));
                state.tail = Some(Arc::clone(&new_node));
            }
        }

        state.len += 1;
        new_node
    }

    pub fn push_back(&self, value: T) -> Arc<ConcurrentDllNode<T>> {
        let new_node = ConcurrentDllNode::new(value);
        let mut state = self.state.lock().expect("list lock poisoned");

        match state.tail.take() {
            Some(old_tail) => {
                *old_tail.next.lock().expect("node next lock poisoned") =
                    Some(Arc::clone(&new_node));
                *new_node.prev.lock().expect("node prev lock poisoned") =
                    Some(Arc::downgrade(&old_tail));
                state.tail = Some(Arc::clone(&new_node));
                if state.head.is_none() {
                    state.head = Some(old_tail);
                }
            }
            None => {
                state.head = Some(Arc::clone(&new_node));
                state.tail = Some(Arc::clone(&new_node));
            }
        }

        state.len += 1;
        new_node
    }

    pub fn pop_front(&self) -> Option<Arc<ConcurrentDllNode<T>>> {
        let mut state = self.state.lock().expect("list lock poisoned");
        let old_head = state.head.take()?;

        let next = old_head
            .next
            .lock()
            .expect("node next lock poisoned")
            .take();
        match next {
            Some(next_node) => {
                *next_node.prev.lock().expect("node prev lock poisoned") = None;
                state.head = Some(next_node);
            }
            None => {
                state.tail = None;
            }
        }

        *old_head.prev.lock().expect("node prev lock poisoned") = None;
        state.len -= 1;
        Some(old_head)
    }

    pub fn pop_back(&self) -> Option<Arc<ConcurrentDllNode<T>>> {
        let mut state = self.state.lock().expect("list lock poisoned");
        let old_tail = state.tail.take()?;

        let prev = old_tail
            .prev
            .lock()
            .expect("node prev lock poisoned")
            .take()
            .and_then(|w| Weak::upgrade(&w));

        match prev {
            Some(prev_node) => {
                *prev_node.next.lock().expect("node next lock poisoned") = None;
                state.tail = Some(prev_node);
            }
            None => {
                state.head = None;
            }
        }

        *old_tail.next.lock().expect("node next lock poisoned") = None;
        state.len -= 1;
        Some(old_tail)
    }
}

#[cfg(test)]
mod tests {
    use super::ConcurrentDll;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn push_and_pop_preserve_links() {
        let list = ConcurrentDll::new();
        let n1 = list.push_back(1);
        let n2 = list.push_back(2);
        let n3 = list.push_back(3);

        assert_eq!(*n1.read_value(), 1);
        assert_eq!(*n2.read_value(), 2);
        assert_eq!(*n3.read_value(), 3);

        assert!(n1.prev().is_none());
        assert_eq!(*n1.next().expect("n1.next missing").read_value(), 2);
        assert_eq!(*n2.prev().expect("n2.prev missing").read_value(), 1);
        assert_eq!(*n2.next().expect("n2.next missing").read_value(), 3);
        assert_eq!(*n3.prev().expect("n3.prev missing").read_value(), 2);
        assert!(n3.next().is_none());

        let popped = list.pop_front().expect("pop_front should return node");
        assert_eq!(*popped.read_value(), 1);
        assert_eq!(list.len(), 2);
        assert_eq!(*list.head().expect("head should exist").read_value(), 2);
    }

    #[test]
    fn concurrent_push_back_keeps_consistent_length() {
        let list = Arc::new(ConcurrentDll::new());
        let mut handles = Vec::new();

        for i in 0..8 {
            let list_clone = Arc::clone(&list);
            handles.push(thread::spawn(move || {
                for j in 0..250 {
                    list_clone.push_back(i * 1_000 + j);
                }
            }));
        }

        for handle in handles {
            handle.join().expect("thread panicked");
        }

        assert_eq!(list.len(), 2_000);
        assert!(list.head().is_some());
        assert!(list.tail().is_some());
    }
}
