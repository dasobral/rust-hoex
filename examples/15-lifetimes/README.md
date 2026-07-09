# 15-lifetimes

Lifetime annotations: connect returned references to the inputs they borrow
from, and store references inside structs safely.

## What this teaches

- Why lifetimes exist (prevent dangling references)
- Explicit `'a` on functions with multiple input references
- Lifetime elision rules (documented in `lib.rs`)
- Structs that hold references: `ImportantExcerpt<'a>`
- Cases where elision is not enough

## How to run

```bash
cd examples/15-lifetimes
cargo run
cargo test
cargo clippy --all-targets -- -D warnings
cargo fmt
```

From the workspace root:

```bash
cargo run -p example_lifetimes
cargo test -p example_lifetimes
```

## Key concepts

### Why annotate?

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str
```

The output may come from `x` **or** `y`. Naming one shared lifetime `'a`
tells the compiler both inputs must outlive the result.

### Elision

With a single input (`fn first_word(s: &str) -> &str`), Rust elides lifetimes
for you. With two inputs and one output reference, you must be explicit.

### Structs with references

```rust
struct ImportantExcerpt<'a> {
    part: &'a str,
}
```

The lifetime parameter is part of the type: an `ImportantExcerpt` cannot
outlive the string it points at.

## Exercises

1. Write `shortest<'a>(x: &'a str, y: &'a str) -> &'a str`.
2. Add `ImportantExcerpt::last_word(text: &'a str) -> Self`.
3. Try returning a reference to a local `String` from a function — confirm
   the compile error, then fix it by returning an owned `String` instead.

## Further reading

- [The Rust Book — Validating References with Lifetimes](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html)
- [Rust by Example — Lifetimes](https://doc.rust-lang.org/rust-by-example/scope/lifetime.html)
