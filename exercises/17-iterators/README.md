# 17-iterators Exercises

Firewall log line processing with Rust iterator adapters — `filter`, `map`, `sum`, `collect`, `count`, and `inspect`.

## Overview

SOC analysts stream access-control logs where each line records an action (`ALLOW` / `DENY`), a source IP, and bytes transferred. This crate teaches lazy iterator chains on borrowed log slices.

## Project layout

```bash
exercises/17-iterators/
├── Cargo.toml
├── README.md
├── src/
│   ├── lib.rs           # Orchestration + ExerciseInfo
│   ├── main.rs          # clap CLI
│   ├── log_line.rs      # LogLine + core iterator helpers
│   ├── adapters.rs      # Exercise 1
│   ├── filters.rs       # Exercise 2
│   └── pipeline.rs      # Exercise 3
└── tests/integration.rs
```

## Learning objectives

- [x] Sum allowed bytes with `filter` → `map` → `sum`
- [x] Collect denied IPs with `filter` → `map` → `collect`
- [x] Count allows with `filter().count()`
- [x] Use `inspect` for side effects inside a pipeline

## Running

```bash
cargo run -p exercise_iterators
cargo run -p exercise_iterators -- list
cargo run -p exercise_iterators -- filters --verbose
cargo test -p exercise_iterators
cargo clippy -p exercise_iterators --all-targets --all-features -- -D warnings
```

## Key API

| Function | Iterator chain | Purpose |
|----------|----------------|---------|
| `allowed_bytes` | `filter` → `map` → `sum` | Total bytes on ALLOW lines |
| `denied_ips` | `filter` → `map` → `collect` | Source IPs from DENY lines |
| `count_allows` | `filter` → `count` | Number of ALLOW lines |
| `inspect_action_counts` | `inspect` → `for_each` | Tally without transforming items |

## Exercises

1. **adapters** — sum allowed traffic volume
2. **filters** — collect denied IPs and count allows
3. **pipeline** — `inspect` for debug-friendly tallies

## Related material

- Example: `examples/17-iterators`
- [The Rust Book — Iterators](https://doc.rust-lang.org/book/ch13-02-iterators.html)
