# 18-closures Exercises

Threat event triage with Rust closures — sorting, partitioning, and capturing a watchlist.

## Overview

Security events carry a source IP, risk score, and critical flag. Closures express one-off predicates and comparators without boilerplate named functions.

## Project layout

```bash
exercises/18-closures/
├── Cargo.toml
├── README.md
├── src/
│   ├── lib.rs           # Orchestration + ExerciseInfo
│   ├── main.rs          # clap CLI
│   ├── threat.rs        # ThreatEvent + closure helpers
│   ├── sorting.rs       # Exercise 1
│   ├── partition.rs     # Exercise 2
│   └── watchlist.rs     # Exercise 3
└── tests/integration.rs
```

## Learning objectives

- [x] Sort with `sort_by_key` on a field
- [x] Split a `Vec` with `into_iter().partition`
- [x] Pass `impl Fn` predicates to `count_matching`
- [x] Capture a `HashSet` watchlist inside a closure

## Running

```bash
cargo run -p exercise_closures
cargo run -p exercise_closures -- list
cargo run -p exercise_closures -- watchlist --verbose
cargo test -p exercise_closures
cargo clippy -p exercise_closures --all-targets --all-features -- -D warnings
```

## Key API

| Function | Closure role | Purpose |
|----------|--------------|---------|
| `sort_by_source` | key extractor | Order events by source IP |
| `partition_critical` | partition predicate | Split critical vs normal |
| `count_matching` | generic `Fn` predicate | Count custom matches |
| `count_watchlisted` | captures `HashSet` | Count events from listed IPs |

## Exercises

1. **sorting** — `sort_by_key` on source
2. **partition** — `into_iter().partition` for escalation queues
3. **watchlist** — closure capturing suspicious IP set

## Related material

- Example: `examples/18-closures`
- [The Rust Book — Closures](https://doc.rust-lang.org/book/ch13-01-closures.html)
