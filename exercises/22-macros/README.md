# 22-macros Exercises

Declarative macros with `macro_rules!`.

## Overview

Build compile-time helpers for stdout/stderr logging, test fixture vectors, and nested maximum selection — no procedural macro crates required.

## Project layout

```bash
exercises/22-macros/
├── Cargo.toml
├── README.md
├── src/
│   ├── lib.rs      # say!, say_err!, testvec!, max_of!
│   └── main.rs     # CLI demo
└── tests/integration.rs
```

## Learning objectives

- [x] Multiple matcher arms (`say!` with/without values)
- [x] Repetition `$( ... ),+` in `testvec!` and `max_of!`
- [x] Empty, single, and multi-element `testvec!` expansions
- [x] Recursive macro expansion in `max_of!`
- [x] Export with `#[macro_export]` and test via `$crate` / crate root

## Running

```bash
cargo run -p exercise_macros
cargo run -p exercise_macros -- --verbose
cargo test -p exercise_macros
cargo clippy -p exercise_macros --all-targets --all-features -- -D warnings
```

## Macros

| Macro | Purpose |
|-------|---------|
| `say!` | Tagged stdout logging |
| `say_err!` | Tagged stderr logging |
| `testvec!` | Build `Vec` (empty / one / many) |
| `max_of!` | Nested `.max()` over 2+ expressions |

## Related example

See `examples/22-macros` for `maplit!` and additional macro patterns.
