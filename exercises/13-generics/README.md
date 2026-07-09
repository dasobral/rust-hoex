# 13-generics Exercises

Generic containers, pairs, and slice search for security-flavored data.

## Overview

Type parameters let one implementation work for API keys, threat scores, and sensor readings. This crate exercises `SecureContainer<T>`, `Pair<A, B>`, and `find_min`.

## Learning objectives

- [x] Generic struct with methods: `new`, `get`, `get_mut`, `into_inner`, `map`
- [x] Trait-bound impl: `clear` when `T: Default`
- [x] `Pair::eq_parts` with `A: Eq, B: Eq`
- [x] `find_min<T: PartialOrd>` over slices

## Running

```bash
cargo run -p exercise_generics -- list
cargo run -p exercise_generics -- container --verbose
cargo test -p exercise_generics
cargo clippy -p exercise_generics --all-targets --all-features -- -D warnings
```

## Key API

```rust
let mut c = SecureContainer::new("api-key", String::from("secret"));
let len = c.map(|s| s.len()).into_inner();

let p = Pair::new("203.0.113.1", 4_u32);
assert!(p.eq_parts(&Pair::new("203.0.113.1", 4)));

let min = find_min(&[85_u32, 12, 40]);
```

## Related material

- Example: `examples/13-generics`
- [The Rust Book — Generics](https://doc.rust-lang.org/book/ch10-00-generics.html)
