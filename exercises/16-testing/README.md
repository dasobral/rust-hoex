# 16-testing Exercises

Password policy library focused on teaching Rust's test harness.

## Overview

Each policy rule lives in `rules.rs` with its own unit tests. `policy.rs` combines rules into `check_policy`, and integration tests in `tests/` exercise the public API.

## Rules

| Rule | Function | Violation message |
|------|----------|-------------------|
| Min length (8) | `has_min_length` | `too short` |
| Digit | `has_digit` | `missing digit` |
| Uppercase | `has_uppercase` | `missing uppercase` |
| Punctuation | `has_punctuation` | `missing punctuation` |

## Running

```bash
cargo run -p exercise_testing -- policy --verbose
cargo test -p exercise_testing
cargo test -p exercise_testing -- --ignored   # slow tests
cargo clippy -p exercise_testing --all-targets --all-features -- -D warnings
```

## Test types demonstrated

- **Unit tests** — one module per rule in `src/rules.rs`
- **Integration tests** — `tests/integration.rs` uses public API only
- **`#[ignore]`** — slow candidate scan in lib and integration tests
- **`#[should_panic]`** — only inside `#[cfg(test)]` modules (workspace denies panic in lib code)

## Related material

- Example: `examples/16-testing`
- [The Rust Book — Testing](https://doc.rust-lang.org/book/ch11-00-testing.html)
