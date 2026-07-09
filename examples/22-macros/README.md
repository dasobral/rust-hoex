# 22-macros

Declarative (`macro_rules!`) macros only — `say!`, `testvec!`, `maplit!`, and
`count_exprs!`. No procedural-macro crates.

## What this teaches

- `macro_rules!` matcher arms and expansion
- Designators: `$expr`, and how patterns bind fragments
- Repetition: `$()*`, `$()+`, optional trailing commas
- Hygiene: names introduced by the macro do not clash with caller locals
- When a macro helps vs when a plain function is clearer

## How to run

```bash
cd examples/22-macros
cargo run
cargo test
cargo clippy --all-targets -- -D warnings
```

```bash
cargo run -p example_macros
cargo test -p example_macros
```

## Key concepts

### Designators and repetition

```rust
macro_rules! maplit {
    ( $( $key:expr => $value:expr ),* $(,)? ) => {{
        let mut map = HashMap::new();
        $( map.insert($key, $value); )*
        map
    }};
}
```

### Hygiene (brief)

A `let mut count = 0` inside `count_exprs!` is not the same binding as a
`count` variable in the caller. Macros are hygienic by default in Rust.

### Prefer functions when possible

Macros are powerful but harder to read, IDE-navigate, and type-check in
isolation. Reach for them when you need new syntax or to avoid boilerplate
that functions cannot express.

## Exercises

1. Add a `say_err!` arm that prints to stderr.
2. Extend `testvec!` to support a single-element form without trailing comma issues.
3. Write `max_of!` that expands to nested `.max` calls for 2+ expressions.

## Further reading

- [The Rust Book — Macros](https://doc.rust-lang.org/book/ch20-05-macros.html)
- [Rust by Example — macro_rules!](https://doc.rust-lang.org/rust-by-example/macros.html)
- [The Little Book of Rust Macros](https://veykril.github.io/tlborm/)
