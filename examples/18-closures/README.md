# 18-closures

Anonymous functions (closures) for sorting, filtering, and labeling threat
events — including environment capture and `Fn` / `FnMut` / `FnOnce`.

## What this teaches

- Closure syntax: `|args| expr` and `|args| { ... }`
- Capturing the environment (including `move`)
- Passing closures to `sort_by`, `filter`, `map`
- `Fn` vs `FnMut` vs `FnOnce` at a glance
- When a closure beats a named `fn` (and when it does not)

## How to run

```bash
cd examples/18-closures
cargo run
cargo test
cargo clippy --all-targets -- -D warnings
```

```bash
cargo run -p example_closures
cargo test -p example_closures
```

## Key concepts

### Capture

```rust
.filter(|event| event.severity >= min)  // borrows `min`
```

```rust
move |list| { format!("{tag} => ...") } // owns `tag`
```

### Trait hierarchy

Every `Fn` is also `FnMut` and `FnOnce`. If the body moves a captured value
out, the closure is only `FnOnce`.

### Closures vs named functions

Use closures for local, one-off logic next to the call site. Use named `fn`
for reusable, documented, capture-free helpers.

## Exercises

1. Sort by `source` ascending with `sort_by_key` and a closure.
2. Write `partition_critical(events) -> (Vec, Vec)` using `into_iter().partition`.
3. Pass a capturing closure into `count_matching` that closes over a `HashSet` of IPs.

## Further reading

- [The Rust Book — Closures](https://doc.rust-lang.org/book/ch13-01-closures.html)
- [Rust by Example — Closures](https://doc.rust-lang.org/rust-by-example/fn/closures.html)
