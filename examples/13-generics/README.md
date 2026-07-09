# 13-generics

Generic functions and structs for security-flavored data: `SecureContainer<T>`,
`Pair<A, B>`, and `find_max`.

## What this teaches

- Type parameters on structs (`SecureContainer<T>`) and functions (`find_max<T>`)
- Trait bounds: `Display`, `PartialOrd`, `Clone`, and closure bounds
- Separate `impl` blocks with extra bounds (`impl<T: Display> …`)
- **Monomorphization**: the compiler generates a concrete copy per type used
- Zero-cost abstraction — generics are resolved at compile time

## How to run

```bash
cd examples/13-generics
cargo run
cargo test
cargo clippy --all-targets -- -D warnings
cargo fmt
```

From the workspace root:

```bash
cargo run -p example_generics
cargo test -p example_generics
```

## Key concepts

### Type parameters

```rust
pub struct SecureContainer<T> {
    label: String,
    value: T,
}

pub fn find_max<T: PartialOrd>(items: &[T]) -> Option<&T> { /* ... */ }
```

`T` is a placeholder. Call sites pick the concrete type (`u32`, `String`, …).

### Trait bounds

A bound says “`T` must implement this trait”:

| Bound | Enables |
|-------|---------|
| `T: Display` | `audit_line`, `format_reading` |
| `T: PartialOrd` | comparisons in `find_max` |
| `T: Clone` | `clone_inner` |
| `F: FnMut(&T) -> bool` | predicate in `filter_owned` |

### Monomorphization

When you use `SecureContainer<String>` and `SecureContainer<u32>`, rustc emits
two specialized structs/functions. You write the code once; the machine code is
as fast as if you had handwritten each version (no runtime “generic” dispatch).

### Multiple type parameters

`Pair<A, B>` can hold two different types. `swap` returns `Pair<B, A>`.

## Exercises

1. Add `impl<T: Default> SecureContainer<T> { fn clear(&mut self) { … } }`.
2. Write `fn find_min<T: PartialOrd>(items: &[T]) -> Option<&T>`.
3. Constrain `Pair<A, B>` with `A: Eq, B: Eq` and add `fn eq_parts(&self, …)`.
4. Create `SecureContainer::map<U, F>(self, f: F) -> SecureContainer<U>` where
   `F: FnOnce(T) -> U`.

## Further reading

- [The Rust Book — Generic Types](https://doc.rust-lang.org/book/ch10-01-syntax.html)
- [The Rust Book — Traits](https://doc.rust-lang.org/book/ch10-02-traits.html) (bounds)
- [Rust by Example — Generics](https://doc.rust-lang.org/rust-by-example/generics.html)
