# 05-controlFlow

`if` / `else`, loops, and `match` through a **threat-score log classifier**.

## Overview

This example scores fake security log lines, maps scores to a `Severity` enum,
derives an `AccessDecision`, and aggregates batches with `loop` / `while` /
`for`. It emphasizes `match`, `if` expressions, `break`/`continue`, and `if let`.

## Learning Objectives

After completing this example, you should understand:

- [x] `if` / `else if` / `else` as expressions that yield values
- [x] `loop`, `while`, and `for` iteration
- [x] `break` and `continue`, including `break value`
- [x] `match` on integer ranges and enums
- [x] `if let` for ergonomic `Option` handling
- [x] A small domain enum (`Severity`, `AccessDecision`)

## How to run

```bash
cd examples/05-controlFlow
cargo run
cargo test
cargo clippy --all-targets -- -D warnings
cargo fmt
```

From the workspace root:

```bash
cargo run -p example_controlflow
cargo test -p example_controlflow
cargo clippy -p example_controlflow --all-targets -- -D warnings
```

## Key Concepts

### `if` as an expression

```rust
let banner = if score >= 90 {
    "SEVERE EVENT IN STREAM"
} else {
    "stream nominal"
};
```

### `match` on ranges and enums

```rust
match score {
    0..=14 => Severity::Info,
    15..=39 => Severity::Low,
    // ...
    _ => Severity::Critical,
}
```

### Loops

```rust
for (idx, line) in logs.iter().enumerate() { /* ... */ }

while idx < scores.len() { /* ... */ }

let summary = loop {
    if done {
        break (counted, weight, early);
    }
};
```

### `if let`

```rust
if let Some(idx) = first_critical_index(&scores) {
    println!("critical at {idx}");
}
```

## Exercises

1. Add a `Severity::Unknown` variant and update every `match` (the compiler will guide you).
2. Change `classify_batch` so `max_events` counts only non-empty lines, not raw indices.
3. Use `while let Some(line) = iter.next()` to walk a log iterator.
4. Replace the keyword table in `score_log_line` with a `match` on the first character of each token.

## Further Reading

- [The Rust Book — Control Flow](https://doc.rust-lang.org/book/ch03-05-control-flow.html)
- [The Rust Book — Enums and Pattern Matching](https://doc.rust-lang.org/book/ch06-00-enums.html)
- [Rust by Example — Flow Control](https://doc.rust-lang.org/rust-by-example/flow_control.html)
- [Rust by Example — match](https://doc.rust-lang.org/rust-by-example/flow_control/match.html)

## Related Examples

- `03-dataTypes`: scalar/compound types used in scores and buffers
- `04-functions`: factor classifiers into reusable functions
- `02-variables`: bindings updated inside loops
