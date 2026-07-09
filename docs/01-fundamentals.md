# Fundamentals

Core language building blocks: variables, types, functions, control flow, ownership, and borrowing. Work through **examples 02–07** alongside this guide.

## Suggested order

| Step | Example | Focus |
|------|---------|--------|
| 1 | [`02-variables`](../examples/02-variables/) | `let`, `mut`, shadowing, constants |
| 2 | [`03-dataTypes`](../examples/03-dataTypes/) | Scalars, tuples, arrays |
| 3 | [`04-functions`](../examples/04-functions/) | Parameters, returns, expressions |
| 4 | [`05-controlFlow`](../examples/05-controlFlow/) | `if`, loops, `match` |
| 5 | [`06-ownership`](../examples/06-ownership/) | Moves, `Clone`, `Copy`, `Drop` |
| 6 | [`07-borrowing`](../examples/07-borrowing/) | `&T`, `&mut T`, borrow rules |

Also skim [`01-helloWorld`](../examples/01-helloWorld/) if you have not run a Rust program yet. Structs, enums, and modules (08–10) sit between fundamentals and intermediate — do them before collections.

## Practical tips

- Prefer immutability: start with `let`, add `mut` only when needed.
- Shadowing (`let x = …; let x = …;`) is intentional — useful for transforming values without `mut`.
- Function bodies: the last expression without `;` is the return value.
- `match` must be exhaustive; use `_` for a catch-all.
- Ownership: assigning a non-`Copy` value moves it. Use `.clone()` only when you truly need two owners.
- Borrowing: many shared refs (`&T`) **or** one mutable ref (`&mut T`) — not both at once.

## Common mistakes

| Mistake | Fix |
|---------|-----|
| Using a value after move | Borrow (`&`), clone, or redesign so ownership stays clear |
| `cannot borrow as mutable` | End the shared borrow first, or restructure scopes |
| Integer overflow in debug | Use wrapping/saturating methods or checked arithmetic when intentional |
| Expecting C-style fallthrough in `match` | Each arm is independent; no fallthrough |
| Putting `;` on a return expression | Drop the semicolon if you mean to return that value |

## How to practice

For each example:

1. Read the example README
2. `cargo run` (or `cargo run -p example_<name>`)
3. `cargo test`
4. Try the exercises listed in that README
5. Move on

After 02–07, optionally tackle [`exercises/02-variables`](../exercises/02-variables/) for deeper practice with integers and overflow.

## The Rust Book

- [Ch 3 — Common Programming Concepts](https://doc.rust-lang.org/book/ch03-00-common-programming-concepts.html) (variables, types, functions, control flow)
- [Ch 4 — Understanding Ownership](https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html)
- [Ch 5 — Structs](https://doc.rust-lang.org/book/ch05-00-structs.html) (pairs with example 08)
- [Ch 6 — Enums and Pattern Matching](https://doc.rust-lang.org/book/ch06-00-enums.html) (pairs with example 09)
- [Ch 7 — Packages, Crates, Modules](https://doc.rust-lang.org/book/ch07-00-managing-growing-projects.html) (pairs with example 10)

Next: [02-intermediate.md](02-intermediate.md)
