# 23-unsafe Exercises

Minimal `unsafe` Rust with safe public wrappers.

## Overview

Practice raw pointer read/write and pointer walking behind bounds-checked APIs. Every `unsafe` block includes a `// SAFETY:` comment explaining the invariant.

> **Note:** The Rust standard library uses `unsafe` internally (e.g. `Vec`, `String`, syscalls). Application code should stay safe unless a profiler or FFI boundary requires otherwise.

## Project layout

```bash
exercises/23-unsafe/
├── Cargo.toml          # local `unsafe_code = "allow"`
├── README.md
├── src/
│   ├── lib.rs          # read_at, write_at, max_i32
│   └── main.rs         # CLI demo
└── tests/integration.rs
```

## Learning objectives

- [x] Document invariants in `// SAFETY:` comments
- [x] Wrap `ptr::read` / `ptr::write` in safe functions returning `Option`
- [x] Walk slices with raw pointers (`max_i32`)
- [x] Keep `unsafe` localized; callers use safe APIs only

## Running

```bash
cargo run -p exercise_unsafe
cargo run -p exercise_unsafe -- --verbose
cargo test -p exercise_unsafe
cargo clippy -p exercise_unsafe --all-targets --all-features -- -D warnings
```

## Key functions

```rust
let byte = read_at(&data, 2);
write_at(&mut data, 0, b'x')?;
let peak = max_i32(&[4, 8, 15, 16, 23, 42]);
```

## Lint configuration

This crate mirrors workspace lints but sets `unsafe_code = "allow"` locally (see `Cargo.toml`), matching `examples/23-unsafe`.

## Related example

See `examples/23-unsafe` for `sum_i32` and additional pointer patterns.
