# 23-unsafe

Minimal, well-commented `unsafe` Rust: raw pointer reads wrapped in a safe
API. **Do not use `unsafe` until you must.**

## Warning

The workspace sets `unsafe_code = "forbid"`. Cargo does **not** allow
overriding a single lint while `lints.workspace = true`, so this package
mirrors the workspace clippy/rust lints and sets:

```toml
[lints.rust]
unsafe_code = "allow"  # This example intentionally teaches unsafe
```

Never copy that allow into ordinary application crates without review.

## What this teaches

- The `unsafe` keyword and what it permits (here: deref / read raw pointers)
- Writing `// SAFETY:` comments that state invariants
- Keeping unsafe blocks **tiny**
- Exposing a **safe wrapper** (`read_at`) so callers stay in safe Rust
- Why safe abstractions exist (concentrate and audit risk)

## How to run

```bash
cd examples/23-unsafe
cargo run
cargo test
cargo clippy --all-targets -- -D warnings
```

```bash
cargo run -p example_unsafe
cargo test -p example_unsafe
```

## Key concepts

### Unsafe does not turn off the type system

It only unlocks a few extra operations. You become responsible for memory
safety invariants the compiler no longer checks.

### Document every unsafe block

```rust
// SAFETY: index was bounds-checked by the caller of this safe wrapper.
Some(unsafe { read_at_unchecked(slice, index) })
```

### Prefer safe APIs

Library users should call `read_at` / `sum_i32` — never the unchecked helper.

## Exercises

1. Add `write_at` with a safe `Option` wrapper around `ptr::write`.
2. Implement `max_i32(slice) -> Option<i32>` using a pointer walk.
3. List three standard-library APIs that contain `unsafe` internally.

## Further reading

- [The Rustonomicon](https://doc.rust-lang.org/nomicon/)
- [The Rust Book — Unsafe Rust](https://doc.rust-lang.org/book/ch20-01-unsafe-rust.html)
- [Safe Transmute / safety comments guidance](https://doc.rust-lang.org/nomicon/working-with-unsafe.html)
