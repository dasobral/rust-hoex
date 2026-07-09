# 20-concurrency Exercises

Parallel log processing with `std::thread` and `mpsc` channels.

## Overview

Parse simple log lines, filter by severity (default `ERROR`, configurable), and collect matching hits from a worker pool through a channel. Workers stop early when the consumer disconnects (`send().is_err()`).

## Project layout

```bash
exercises/20-concurrency/
├── Cargo.toml
├── README.md
├── src/
│   ├── lib.rs      # parse_level, process_logs, collect_with_limit
│   └── main.rs     # CLI demo
└── tests/integration.rs
```

## Learning objectives

- [x] Spawn worker threads with `move` closures
- [x] Use `mpsc::channel` for many-producer / one-consumer messaging
- [x] Filter log lines before sending to the channel
- [x] Detect a dropped receiver via `send().is_err()` and stop workers
- [x] Join handles and surface panics as `PipelineError`

## Running

```bash
cargo run -p exercise_concurrency
cargo run -p exercise_concurrency -- --filter WARN --workers 4 --verbose
cargo test -p exercise_concurrency
cargo clippy -p exercise_concurrency --all-targets --all-features -- -D warnings
```

## Key functions

```rust
let hits = process_logs(lines, "ERROR", 3)?;
let limited = collect_with_limit(lines, "ERROR", 2, 1);
let counts = count_by_level(&hits);
```

## Concepts

| Topic | Detail |
|-------|--------|
| `Send` | Values that may move to another thread |
| `Sync` | Shared references safe across threads |
| Early stop | Drop receiver or stop recv loop; workers see failed `send` |

## Related example

See `examples/20-concurrency` for the companion walkthrough.
