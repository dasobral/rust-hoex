# 08-structs

Custom data types — model a **user account / security credential** with structs,
`impl` methods, and update syntax.

## Overview

Structs let you group related data. Methods attach behavior: `&self` to read,
`&mut self` to update, and `self` to consume. Associated functions like `new`
act as constructors.

## Learning Objectives

After completing this example, you should understand:

- [x] **Struct definition** and public vs private fields
- [x] **`impl` blocks** for inherent methods
- [x] **Method receivers** — `&self`, `&mut self`, `self`
- [x] **Associated functions** — `UserAccount::new`
- [x] **`#[derive(Debug)]`** for printable representations
- [x] **Field access** and **struct update syntax** (`..`)

## Running the Code

```bash
cd examples/08-structs
cargo run
cargo test
cargo clippy --all-targets -- -D warnings
cargo fmt
```

From the workspace root:

```bash
cargo run -p example_structs
cargo test -p example_structs
cargo clippy -p example_structs --all-targets -- -D warnings
```

## Key Concepts Demonstrated

### 1. Definition and constructor

```rust
let alice = UserAccount::new("alice", "hunter2-hash", Role::Operator);
println!("{alice:?}"); // Debug
```

### 2. Method receivers

```rust
alice.display_name();          // &self
alice.record_failed_login();   // &mut self
let line = alice.into_audit_line(); // self (consumes)
```

### 3. Struct update syntax

```rust
let bob = UserAccount {
    username: String::from("bob"),
    ..alice.clone()
};
```

## Exercises

1. Add a `email: String` field and update `new` / tests.
2. Implement `fn is_admin(&self) -> bool` and unit-test it.
3. Add a max-login constant and make lockout threshold configurable.
4. Derive `PartialEq` on a new `Session` struct and compare two instances in a test.

## Further Reading

- [The Rust Book — Defining and Instantiating Structs](https://doc.rust-lang.org/book/ch05-01-defining-structs.html)
- [The Rust Book — Method Syntax](https://doc.rust-lang.org/book/ch05-03-method-syntax.html)
- [Rust by Example — Structures](https://doc.rust-lang.org/rust-by-example/custom_types/structs.html)

## Related Examples

- `06-ownership`: how owned fields (`String`) move
- `07-borrowing`: why methods take `&self` / `&mut self`

---

**Note**: Credentials here are plaintext for teaching. Production systems store salted hashes only.
