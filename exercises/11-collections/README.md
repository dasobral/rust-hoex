# 11-collections Exercises

Intrusion log aggregation with Rust collections — `Vec`, `HashMap`, and `HashSet` — in a SOC (Security Operations Center) scenario.

## Overview

Analysts ingest authentication and network events from sensors. This exercise crate teaches how to store, count, deduplicate, and filter those events using Rust's core collections.

### Why intrusion logs?

- **Vec** — ordered event stream (timeline reconstruction)
- **HashMap** — IP and user hit counts (repeat offenders)
- **HashSet** — unique actions and usernames (deduplication)

## Project layout

```bash
exercises/11-collections/
├── Cargo.toml
├── README.md
├── src/
│   ├── lib.rs              # Orchestration + ExerciseInfo
│   ├── main.rs             # clap CLI (list / all / subcommands)
│   ├── intrusion.rs        # LogEvent, IntrusionLog, aggregation API
│   ├── aggregator.rs       # Exercise 1
│   ├── analysis.rs         # Exercise 2
│   └── investigation.rs    # Exercise 3
└── tests/integration.rs
```

## Learning objectives

- [x] Build a growable event list with `Vec::push`
- [x] Count per-IP hits with `HashMap` and the **entry API**
- [x] Deduplicate users and actions with `HashSet`
- [x] Filter into a new `Vec` with iterator adapters
- [x] Implement `top_user` with deterministic tie-breaking

## Running

```bash
cargo run -p exercise_collections
cargo run -p exercise_collections -- list
cargo run -p exercise_collections -- analysis --verbose
cargo test -p exercise_collections
cargo clippy -p exercise_collections --all-targets --all-features -- -D warnings
cargo fmt -p exercise_collections
```

## Key API

### `LogEvent` and `IntrusionLog`

```rust
let mut log = IntrusionLog::new();
log.push(LogEvent::new("203.0.113.10", "alice", "login_fail"));
assert_eq!(log.len(), 1);
```

### Aggregation

| Method | Collection | Purpose |
|--------|------------|---------|
| `ip_counts()` | `HashMap<String, u32>` | IP → hit count |
| `hot_ips()` | `HashMap<String, u32>` | Alias of `ip_counts` (SOC dashboard naming) |
| `unique_users()` | `HashSet<String>` | Distinct usernames |
| `actions()` | `HashSet<String>` | Distinct action strings |
| `filter_by_action` | `Vec<LogEvent>` | Events matching one action |
| `top_user()` | `Option<String>` | User with most events |

### Tie-breaking in `top_user`

When two users share the highest count, the **lexicographically smallest** username wins. This keeps results deterministic across runs.

## Exercises

1. **aggregator** — push events, inspect `len`, iterate with `events()`
2. **analysis** — `ip_counts`, `hot_ips`, `unique_users`, `actions`
3. **investigation** — `filter_by_action`, `top_user`

## Concepts demonstrated

### HashMap entry API

```rust
let counter = counts.entry(event.ip.clone()).or_insert(0);
*counter = counter.saturating_add(1);
```

### Ownership

`LogEvent` values are **owned** by the `Vec`. Borrowing via `events()` returns `&[LogEvent]` for read-only iteration.

## Testing

- **Unit tests** in `src/intrusion.rs` cover aggregation logic
- **Integration tests** in `tests/integration.rs` exercise the public API and CLI orchestration

## Related material

- Example: `examples/11-collections`
- [The Rust Book — Common Collections](https://doc.rust-lang.org/book/ch08-00-common-collections.html)

## Stretch goals

1. Return `HashMap<&str, u32>` from `ip_counts` with a lifetime tied to `&self`
2. Add a `merge_logs` function that consumes two logs via `Vec::extend`
3. Track the most common **action** alongside `top_user`
