# Atomics Beginner Guide (Rust)

If atomics feel hard, that is normal.  
Use this guide as a practical shortcut.

---

## 1) First Principle: Prefer `Mutex` Unless You Need Atomics

If you are unsure, use `Arc<Mutex<T>>`.

Use atomics only when:
- the shared data is small (`bool`, `usize`, small state flags)
- you need very frequent updates
- lock overhead matters

If your shared state is complex (maps, vectors, structs with many fields), mutex/RwLock is usually safer.

---

## 2) What Atomics Actually Give You

Atomics give you:
- **atomicity**: one operation is indivisible
- **(optional) ordering guarantees**: depends on memory ordering used

Common misunderstanding:
- `Relaxed` gives atomicity, but **does not** order other memory.

---

## 3) Memory Ordering in Plain English

### `Relaxed`
Use when:
- you only care about the atomic variable itself
- example: a statistics counter

### `Release` (store) + `Acquire` (load)
Use when:
- one thread **publishes** data
- another thread waits for a ready flag and then reads data

Think:
- writer: write data -> `ready.store(true, Release)`
- reader: wait until `ready.load(Acquire)` -> then read data

### `AcqRel`
Use for read-modify-write operations (CAS/update) that both:
- consume prior published data
- publish new state

### `SeqCst`
Use when:
- you want simplest global reasoning
- performance is secondary

Safe beginner strategy:
1. Start with `SeqCst`.
2. Make it correct.
3. Relax to `Acquire/Release` only if needed and understood.

---

## 4) Decision Tree (Beginner)

1. Is shared state complex?  
   -> Use `Mutex`/`RwLock`.

2. Is it a simple counter only?  
   -> `AtomicUsize` + `Relaxed`.

3. Is it a ready flag for published data?  
   -> writer `Release`, reader `Acquire`.

4. Is it CAS loop/state transition?  
   -> success `AcqRel`, failure `Acquire` (or `Relaxed` if proven safe).

5. Not sure?  
   -> use `SeqCst` first.

---

## 5) “Good Defaults” You Can Copy

### Counter

```rust
counter.fetch_add(1, Ordering::Relaxed);
```

### Ready flag publication

```rust
// writer
data.store(value, Ordering::Relaxed);
ready.store(true, Ordering::Release);

// reader
while !ready.load(Ordering::Acquire) {}
let v = data.load(Ordering::Relaxed);
```

### CAS loop

```rust
loop {
    let cur = state.load(Ordering::Acquire);
    let next = f(cur);
    if state
        .compare_exchange_weak(cur, next, Ordering::AcqRel, Ordering::Acquire)
        .is_ok()
    {
        break;
    }
}
```

---

## 6) Map This Guide to Your Project

Use these runnable examples:

- Basics:
  - `cargo run -p concurrency-practices --bin atomics_lab`
- Source to read:
  - `src/atomics.rs`
  - `src/atomics_deep_dive.rs`

Suggested order:

1. `relaxed_counter`
2. `release_acquire_publication`
3. `cas_increment`
4. `SpinLock` (educational; usually prefer `Mutex`)
5. `OnceValue`

---

## 7) Common Mistakes

1. Using atomics for complex shared data structures.
2. Using `Relaxed` for publication/visibility problems.
3. Mixing multiple atomics without a clear happens-before model.
4. Building custom lock-free structures too early.
5. Optimizing ordering before correctness is proven.

---

## 8) Practical Learning Loop

For each example:

1. Predict output.
2. Run it.
3. Change ordering (e.g. to `SeqCst`) and rerun.
4. Explain in one sentence why it still works (or not).

If you can explain the “why,” you understand it.

