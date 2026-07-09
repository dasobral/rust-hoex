# 21-async

Async/await with Tokio: simulated concurrent HTTP-like fetches using
`sleep` so the demo runs fully offline.

## What this teaches

- `async fn` and `.await`
- `tokio::join!` for concurrent futures on one task
- `tokio::spawn` for independent tasks + awaiting `JoinHandle`
- Why async ≠ OS threads (cooperative scheduling vs preemption)
- Keeping demos offline (timers instead of real network I/O)

## How to run

```bash
cd examples/21-async
cargo run
cargo test
cargo clippy --all-targets -- -D warnings
```

```bash
cargo run -p example_async
cargo test -p example_async
```

## Key concepts

### Futures are lazy

Calling `fetch_simulated(...)` builds a future; work starts when it is
`.await`ed (or spawned onto the runtime).

### `join!` vs sequential awaits

```rust
// ~max(latencies)
tokio::join!(fetch_a(), fetch_b(), fetch_c());

// ~sum(latencies)
let a = fetch_a().await;
let b = fetch_b().await;
```

### Spawn needs `'static`

Spawned tasks cannot borrow from the caller's stack — move owned data in
(`String`, not `&str` from a local).

## Exercises

1. Add a timeout with `tokio::time::timeout` around a slow fetch.
2. Use `futures::future::join_all` (add dep) or a loop of spawns for N jobs.
3. Measure sequential vs `join!` with larger latencies.

## Further reading

- [Tokio tutorial](https://tokio.rs/tokio/tutorial)
- [The Rust Book — Async](https://doc.rust-lang.org/book/ch17-00-async-await.html)
- [Asynchronous Programming in Rust](https://rust-lang.github.io/async-book/)
