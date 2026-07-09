# 01-helloWorld

Your first Rust program: print to the console, use macros, and see how a library
crate works alongside a binary.

## What this teaches

- The `main` function as the program entry point
- Macros: `println!`, `format!`, and `dbg!` (the `!` marks a macro)
- String formatting with `{name}` placeholders
- Splitting code into `lib.rs` (reusable) and `main.rs` (the binary)

## How to run

```bash
cd examples/01-helloWorld
cargo run          # print greetings
cargo test         # unit + integration tests
cargo clippy       # lint
cargo fmt          # format
```

From the workspace root (after package rename):

```bash
cargo run -p example_helloworld
cargo test -p example_helloworld
```

## Key concepts

### `fn main()`

Every binary crate needs a `main` function. Cargo compiles `src/main.rs` into
an executable named after the package (`example_helloworld`).

### Macros vs functions

`println!` looks like a function call but is a **macro**. Macros can take a
variable number of arguments and expand into more code at compile time. That is
why formatting placeholders work without a fixed function signature.

### `lib` vs `bin`

- `src/lib.rs` defines the library API (`pub fn greet`)
- `src/main.rs` is the binary that calls into that library
- Integration tests under `tests/` can `use example_helloworld::greet`

This split is common in real crates: keep logic testable in the library, keep
`main` thin.

### Cargo

`cargo run` builds (if needed) and runs the binary. `cargo test` runs unit tests
in `lib.rs`/`main.rs` and integration tests in `tests/`.

## Exercises

1. Change `greet` so it returns `"Hi, {name}!"` instead of `"Hello, {name}!"`,
   then update the tests.
2. In `main`, greet two different names using `greet` and print both lines.
3. Use `format!` to build a sentence that includes a number (e.g. your age or
   the year) and print it with `println!`.
4. Replace one `println!` with `dbg!` and observe the extra file/line output.

## Further reading

- [The Rust Book — Chapter 1: Getting Started](https://doc.rust-lang.org/book/ch01-00-getting-started.html)
- [The Rust Book — Hello, World!](https://doc.rust-lang.org/book/ch01-02-hello-world.html)
- [The Rust Book — Hello, Cargo!](https://doc.rust-lang.org/book/ch01-03-hello-cargo.html)
- [Rust by Example — Hello World](https://doc.rust-lang.org/rust-by-example/hello.html)
