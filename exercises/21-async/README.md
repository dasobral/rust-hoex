# 21-async Exercises

Offline simulated threat-intel fetches with Tokio.

## Overview

Practice `async fn`, `.await`, `tokio::time::timeout`, sequential vs parallel fetching with `join!` and spawn/join-all style collection — no network required.

## Project layout

```bash
exercises/21-async/
├── Cargo.toml
├── README.md
├── src/
│   ├── lib.rs      # fetch_threat, timeout wrapper, sequential/parallel
│   └── main.rs     # CLI with #[tokio::main]
└── tests/integration.rs
```

## Learning objectives

- [x] Write `async fn` and `.await` simulated I/O with `tokio::time::sleep`
- [x] Wrap futures with `tokio::time::timeout`
- [x] Compare sequential latency vs `tokio::join!` / spawned tasks
- [x] Test with `#[tokio::test]`

## Running

```bash
cargo run -p exercise_async
cargo run -p exercise_async -- --ids 1,2,3,4,5 --delay 30 --timeout 150 --verbose
cargo test -p exercise_async
cargo clippy -p exercise_async --all-targets --all-features -- -D warnings
```

## Key functions

```rust
let label = fetch_threat(42, 25).await?;
let bounded = fetch_threat_with_timeout(42, 25, 100).await?;
let seq = fetch_sequential(&[1, 2, 3], 30).await;
let par = fetch_parallel(&[1, 2, 3], 30).await;
```

## Async vs threads

| Model | Best for |
|-------|----------|
| Threads (`20-concurrency`) | CPU-bound or blocking work |
| Async tasks (this crate) | Many idle waits multiplexed on few threads |

## Related example

See `examples/21-async` for the companion walkthrough.
