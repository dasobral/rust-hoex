# Fundamentals

Core language building blocks: variables, types, functions, control flow, ownership, and borrowing. Work through **examples 01–10** alongside this guide; each has a paired exercise under `exercises/`.

## Suggested order

| Step | Example | Exercise | Focus |
|------|---------|----------|--------|
| 1 | [`01-helloWorld`](../examples/01-helloWorld/) | [`exercises/01-helloWorld`](../exercises/01-helloWorld/) | First program, `format!` |
| 2 | [`02-variables`](../examples/02-variables/) | [`exercises/02-variables`](../exercises/02-variables/) | `let`, `mut`, shadowing, overflow |
| 3 | [`03-dataTypes`](../examples/03-dataTypes/) | [`exercises/03-dataTypes`](../exercises/03-dataTypes/) | Scalars, tuples, arrays |
| 4 | [`04-functions`](../examples/04-functions/) | [`exercises/04-functions`](../exercises/04-functions/) | Parameters, returns, expressions |
| 5 | [`05-controlFlow`](../examples/05-controlFlow/) | [`exercises/05-controlFlow`](../exercises/05-controlFlow/) | `if`, loops, `match` |
| 6 | [`06-ownership`](../examples/06-ownership/) | [`exercises/06-ownership`](../exercises/06-ownership/) | Moves, `Clone`, `Copy`, `Drop` |
| 7 | [`07-borrowing`](../examples/07-borrowing/) | [`exercises/07-borrowing`](../exercises/07-borrowing/) | `&T`, `&mut T`, borrow rules |
| 8 | [`08-structs`](../examples/08-structs/) | [`exercises/08-structs`](../exercises/08-structs/) | Custom types & methods |
| 9 | [`09-enums`](../examples/09-enums/) | [`exercises/09-enums`](../exercises/09-enums/) | Enums & pattern matching |
| 10 | [`10-modules`](../examples/10-modules/) | [`exercises/10-modules`](../exercises/10-modules/) | Modules & visibility |

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

For each topic:

1. Read the example README
2. `cargo run` / `cargo test` on the example
3. Run the paired exercise (`cargo run -p exercise_<name> -- list`)
4. Move on

## The Rust Book

- [Ch 3 — Common Programming Concepts](https://doc.rust-lang.org/book/ch03-00-common-programming-concepts.html) (variables, types, functions, control flow)
- [Ch 4 — Understanding Ownership](https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html)
- [Ch 5 — Structs](https://doc.rust-lang.org/book/ch05-00-structs.html)
- [Ch 6 — Enums and Pattern Matching](https://doc.rust-lang.org/book/ch06-00-enums.html)
- [Ch 7 — Packages, Crates, Modules](https://doc.rust-lang.org/book/ch07-00-managing-growing-projects.html)

Next: [02-intermediate.md](02-intermediate.md)
