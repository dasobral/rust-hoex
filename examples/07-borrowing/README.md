# 07-borrowing

References and borrowing — continue the security theme by **analyzing a password
without taking ownership**, and updating a strength score via a mutable borrow.

## Overview

Ownership moves values; borrowing lets you *look at* or *temporarily mutate*
them without becoming the owner. The borrow checker enforces: many shared
references (`&T`) **or** one mutable reference (`&mut T`), never both at once.

## Learning Objectives

After completing this example, you should understand:

- [x] **Shared references** (`&T`) — read-only access without move
- [x] **Mutable references** (`&mut T`) — exclusive write access
- [x] **Borrow checker rules** — many `&` XOR one `&mut`
- [x] **Why dangling refs are impossible** — lifetimes tied to owners
- [x] **Slices** — `&str` and `&[T]` as views into contiguous data

## Running the Code

```bash
cd examples/07-borrowing
cargo run
cargo test
cargo clippy --all-targets -- -D warnings
cargo fmt
```

From the workspace root:

```bash
cargo run -p example_borrowing
cargo test -p example_borrowing
cargo clippy -p example_borrowing --all-targets -- -D warnings
```

## Key Concepts Demonstrated

### 1. Immutable borrows (many at once)

```rust
let password = String::from("Tr0ub4dor&3");
let letters = count_ascii_letters(&password);
let ok = meets_min_length(&password, 8);
// both borrows are live; password still owned
```

### 2. Mutable borrow exclusivity

```rust
let mut score = 0;
update_strength_score(&mut score, &password);
// while &mut score is active, you cannot also have &score
```

### 3. Slices

```rust
let prefix: Option<&str> = password_prefix(&password, 4);
let avg = average_scores(&[10, 20, 30]); // &[i32]
```

## Exercises

1. Uncomment the conflicting `&` / `&mut` example in `main.rs` and explain the error.
2. Write a function `fn first_char(s: &str) -> Option<char>` and test it.
3. Change `update_strength_score` to also add points for password length over 12.
4. Pass a subslice of an array (`&history[1..3]`) into `average_scores`.

## Further Reading

- [The Rust Book — References and Borrowing](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)
- [The Rust Book — The Slice Type](https://doc.rust-lang.org/book/ch04-03-slices.html)
- [Rust by Example — Borrowing](https://doc.rust-lang.org/rust-by-example/scope/borrow.html)

## Related Examples

- `06-ownership`: moves, Clone, Copy, Drop
- `08-structs`: methods that take `&self` / `&mut self`

---

**Note**: Educational only. Do not log real passwords; masking here is for demos.
