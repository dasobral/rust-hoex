# 15-lifetimes Exercises

Lifetime annotations for borrowed log excerpts and owned summaries.

## Overview

SOC analysts often highlight tokens inside log lines. This crate shows how to return borrowed slices safely and when to return owned `String` values instead.

## Learning objectives

- [x] `shortest<'a>` with shared input/output lifetime
- [x] `ImportantExcerpt<'a>` with `new`, `level`, and `last_word`
- [x] `owned_summary` returns owned data to avoid dangling references
- [x] Document why `fn broken() -> &str { let s = String::from("x"); &s }` fails

## Running

```bash
cargo run -p exercise_lifetimes -- excerpt --verbose
cargo test -p exercise_lifetimes
cargo clippy -p exercise_lifetimes --all-targets --all-features -- -D warnings
```

## Why not return `&str` to a local?

```rust
// Does NOT compile — `s` is dropped before the caller can use the reference.
fn broken() -> &str {
    let s = String::from("alert");
    &s
}
```

Use `owned_summary` (or any `String` return) when assembling new text inside the function.

## Related material

- Example: `examples/15-lifetimes`
- [The Rust Book — Lifetimes](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html)
