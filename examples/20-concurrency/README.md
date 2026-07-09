# 20-concurrency

Parallel log-line processing with OS threads and `std::sync::mpsc` channels.

## What this teaches

- `thread::spawn` and `move` closures
- Joining threads and handling `JoinError` **without** `unwrap`
- `mpsc::channel`: cloneable `Sender`, single `Receiver`
- Why `Send` / `Sync` gate what you can share across threads
- Contrast with async (example 21): threads are preemptive OS tasks

## How to run

```bash
cd examples/20-concurrency
cargo run
cargo test
cargo clippy --all-targets -- -D warnings
```

```bash
cargo run -p example_concurrency
cargo test -p example_concurrency
```

## Key concepts

### Spawn + move

```rust
let tx = tx.clone();
thread::spawn(move || {
    tx.send(hit)?;
    Ok(())
});
```

The closure must own everything it uses after the parent continues.

### Join without panic

```rust
match handle.join() {
    Ok(Ok(_)) => {}
    Ok(Err(err)) => return Err(err),
    Err(payload) => /* worker panicked */,
}
```

### `Send` / `Sync`

Channel messages must be `Send`. Types like `Rc` are not — use `Arc` when
sharing across threads.

## Exercises

1. Filter so only `ERROR` hits are sent on the channel.
2. Use `tx.send(...).is_err()` to stop early if the consumer disconnects.
3. Compare wall-clock time for 1 vs N workers on a larger input.

## Further reading

- [The Rust Book — Fearless Concurrency](https://doc.rust-lang.org/book/ch16-00-concurrency.html)
- [std::sync::mpsc](https://doc.rust-lang.org/std/sync/mpsc/index.html)
