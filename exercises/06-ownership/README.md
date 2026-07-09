# 06-ownership Exercises

Ownership rules through **secure credential handling**: API keys and session tokens
are moved (not copied by accident), cloned only when intentional, zeroized before
drop, and consumed so bindings cannot be reused.

## Overview

These exercises build on [`examples/06-ownership`](../../examples/06-ownership).
You implement and run functions that demonstrate why Rust's ownership model matters
for secrets: two owners of the same heap buffer would be a use-after-free waiting
to happen.

## Learning Objectives

After completing these exercises, you should understand:

- [x] **Move semantics** — assigning a `String` transfers ownership
- [x] **`Clone`** — explicit deep copy when a second owned value is needed
- [x] **`Copy` types** — `i32` duplicates on pass-by-value; `String` does not
- [x] **Consume-on-use** — hand secrets to functions that drop them
- [x] **Zeroization** — scrub heap bytes before drop (safe `into_bytes` pattern)
- [x] **Scope and `Drop`** — values freed when their owner goes out of scope

## Project Layout

```text
exercises/06-ownership/
├── Cargo.toml
├── README.md
├── src/
│   ├── lib.rs           # Public API and orchestration
│   ├── main.rs          # clap CLI (list / all / subcommands)
│   ├── secrets.rs       # Core ownership helpers
│   ├── move_vs_copy.rs  # Move vs Copy exercise
│   └── zeroize.rs       # Zeroize exercise
└── tests/
    └── integration.rs
```

## Running the Exercises

From the workspace root:

```bash
cargo run -p exercise_ownership
cargo run -p exercise_ownership -- list
cargo run -p exercise_ownership -- move-vs-copy --verbose
cargo run -p exercise_ownership -- zeroize
cargo run -p exercise_ownership -- all
```

Quality checks:

```bash
cargo test -p exercise_ownership
cargo clippy -p exercise_ownership --all-targets --all-features -- -D warnings
cargo fmt -p exercise_ownership
```

## Exercises

### 1. `move_vs_copy` — Credential handoff and threat scores

Demonstrates moving an API key between owners, cloning a backup copy, consuming
the primary secret, and doubling a threat score via a `Copy` type.

**Functions:** `take_then_return`, `clone_secret`, `consume_secret`, `copy_threat_score`

### 2. `zeroize` — Scrub secrets before drop

Shows plain consume vs zeroize-then-consume for session tokens, including scoped
ephemeral credentials.

**Functions:** `consume_secret`, `zeroize_and_consume`

## Public API

| Function | Signature | Concept |
|----------|-----------|---------|
| `consume_secret` | `(String) -> usize` | Move + drop |
| `zeroize_and_consume` | `(String) -> usize` | Scrub then drop |
| `clone_secret` | `(&str) -> String` | Explicit clone |
| `take_then_return` | `(String) -> String` | Identity move |
| `copy_threat_score` | `(i32) -> i32` | Copy semantics |

## Key Takeaways

1. **Moves prevent double-free** — only one owner frees heap memory.
2. **Clone is explicit** — copying secrets should be a deliberate act.
3. **Zeroize before drop** — overwrite sensitive bytes when possible.
4. **`Copy` vs move** — small stack types behave differently from heap strings.

## Related Material

- [`examples/06-ownership`](../../examples/06-ownership) — introductory walkthrough
- [`examples/07-borrowing`](../../examples/07-borrowing) — next topic: references
