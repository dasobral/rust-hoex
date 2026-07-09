# 19-smart-pointers Exercises

Recursive security policy rules with `Box`, shared config with `Rc`, and hit counting with `RefCell`.

## Overview

Access policies form recursive trees (allow read **and** deny delete). Multiple policy engines share one configuration and a consultation counter — patterns that map directly to `Box`, `Rc`, and `RefCell`.

## Project layout

```bash
exercises/19-smart-pointers/
├── Cargo.toml
├── README.md
├── src/
│   ├── lib.rs           # Orchestration + ExerciseInfo
│   ├── main.rs          # clap CLI
│   ├── rule.rs          # Rule enum (Box recursion)
│   ├── config.rs        # SharedConfig (Rc + RefCell)
│   ├── engine.rs        # PolicyEngine
│   ├── rules_demo.rs    # Exercise 1
│   ├── sharing.rs       # Exercise 2
│   └── clones.rs        # Exercise 3
└── tests/integration.rs
```

## Learning objectives

- [x] Model recursive rules with `Box<Rule>`
- [x] Compose `Allow`, `Deny`, `AndThen`, and `Or`
- [x] Share policy metadata via `Rc<SharedConfig>`
- [x] Track hits with `Rc<RefCell<u64>>` interior mutability

## Running

```bash
cargo run -p exercise_smartpointers
cargo run -p exercise_smartpointers -- list
cargo run -p exercise_smartpointers -- sharing --verbose
cargo test -p exercise_smartpointers
cargo clippy -p exercise_smartpointers --all-targets --all-features -- -D warnings
```

## Key API

### `Rule`

| Variant | Meaning |
|---------|---------|
| `Allow(action)` | Permit when subject matches |
| `Deny(action)` | Permit when subject differs |
| `AndThen(a, b)` | Both children must allow |
| `Or(a, b)` | Either child may allow |

### `SharedConfig`

- `name()` — shared policy label via `Rc<String>`
- `record_hit()` / `hits()` — `RefCell` counter through `&self`
- `hit_counter()` — `Rc<RefCell<u64>>` handle for shared mutation

## Exercises

1. **rules** — evaluate recursive policy trees
2. **sharing** — `Rc` config owners and `RefCell` hits
3. **clones** — cheap `PolicyEngine` clones sharing one counter

## Next step: threads

This crate stays single-threaded. For concurrent SOC workers:

- Replace `Rc` with **`Arc`** (atomic reference counting)
- Replace `RefCell` with **`Mutex`** or **`RwLock`** for shared mutable state

See `examples/20-concurrency` for the threaded evolution.

## Related material

- Example: `examples/19-smart-pointers`
- [The Rust Book — Smart Pointers](https://doc.rust-lang.org/book/ch15-00-smart-pointers.html)
