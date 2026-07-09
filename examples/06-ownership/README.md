# 06-ownership

Ownership rules through **secure password / sensitive-data handling**: passwords
are moved, not copied by accident; cloned only when you mean it; and consumed
so they cannot be reused after use.

## Overview

Rust's ownership system is the foundation of memory safety without a garbage
collector. This example uses a security-flavored theme — handling passwords —
to show why moves matter: two owners of the same heap buffer would be a
use-after-free waiting to happen.

## Learning Objectives

After completing this example, you should understand:

- [x] **Ownership** — each value has exactly one owner at a time
- [x] **Move semantics** — assigning a `String` transfers ownership
- [x] **`Clone`** — explicit deep copy when a second owned value is needed
- [x] **`Copy` types** — `i32` (and other `Copy` types) duplicate on assign
- [x] **Scope and `Drop`** — values are freed when their owner goes out of scope
- [x] **`String` vs `&str`** — owned heap string vs borrowed string slice

## Running the Code

```bash
cd examples/06-ownership
cargo run
cargo test
cargo clippy --all-targets -- -D warnings
cargo fmt
```

From the workspace root:

```bash
cargo run -p example_ownership
cargo test -p example_ownership
cargo clippy -p example_ownership --all-targets -- -D warnings
```

## Key Concepts Demonstrated

### 1. Move semantics

```rust
let secret = String::from("N0tARealP@ss!");
let moved = take_ownership(secret);
// secret is invalid here — ownership moved into `moved`
```

### 2. Clone when needed

```rust
let backup = clone_password(&moved); // second owned String
// both `moved` and `backup` are valid
```

### 3. Consume and drop

```rust
let len = consume_password(moved); // takes ownership, then drops
// `moved` cannot be used afterward
```

### 4. Copy types

```rust
let attempts: i32 = 3;
let doubled = double_score(attempts);
// `attempts` is still usable — i32 implements Copy
```

## Exercises

1. Try printing `secret` after `take_ownership(secret)` and read the compiler error.
2. Change `consume_password` to overwrite each byte with `0` before `drop` (manual zeroize).
3. Pass an `i32` into a function by value and confirm the caller still has its copy.
4. Create two clones of a password and consume only one — verify the other remains.

## Further Reading

- [The Rust Book — What is Ownership?](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html)
- [The Rust Book — References and Borrowing](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html) (next example)
- [Rust by Example — Ownership](https://doc.rust-lang.org/rust-by-example/scope/move.html)

## Related Examples

- `02-variables`: immutability and types
- `07-borrowing`: references without taking ownership
- `08-structs`: bundling owned fields into custom types

---

**Note**: Educational only. For production secrets, use established crates (e.g. `zeroize`, `secrecy`) and never log passwords.
