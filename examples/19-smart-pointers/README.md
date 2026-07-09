# 19-smart-pointers

`Box`, `Rc`, and `RefCell` via a recursive security-rule tree and shared config
with an access counter.

## What this teaches

- `Box<T>` for heap allocation and **recursive types**
- `Rc<T>` for **shared ownership** on a single thread
- `RefCell<T>` for **interior mutability** (runtime borrow checking)
- When to reach for `Arc` (thread-safe `Rc`) instead
- Combining `Rc<RefCell<T>>` (or `Rc` + nested `RefCell` fields) safely

## How to run

```bash
cd examples/19-smart-pointers
cargo run
cargo test
cargo clippy --all-targets -- -D warnings
```

```bash
cargo run -p example_smartpointers
cargo test -p example_smartpointers
```

## Key concepts

### `Box` — indirection for recursive enums

```rust
enum Rule {
    Allow(String),
    All(Vec<Box<Rule>>),  // without Box, infinite size
    Not(Box<Rule>),
}
```

### `Rc` — clone is a refcount bump

`Rc::clone(&cfg)` does **not** deep-copy the config; it shares it. Drop the
last owner and the value is freed.

### `RefCell` — borrow rules at runtime

`borrow_mut()` panics if a borrow is already active. Keep scopes short. Across
threads, use `Mutex` / `RwLock` with `Arc` instead.

## Exercises

1. Add a `Rule::AndThen(Box<Rule>, Box<Rule>)` variant and evaluate it.
2. Replace the `hits: RefCell<u64>` field with `Rc<RefCell<u64>>` shared separately.
3. Sketch (on paper) how you would port `SharedConfig` to `Arc<Mutex<_>>`.

## Further reading

- [The Rust Book — Smart Pointers](https://doc.rust-lang.org/book/ch15-00-smart-pointers.html)
- [RefCell\<T\> and Interior Mutability](https://doc.rust-lang.org/book/ch15-05-interior-mutability.html)
