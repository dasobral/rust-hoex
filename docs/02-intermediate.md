# Intermediate

Collections, errors, generics, traits, lifetimes, testing, iterators, and closures. Work through **examples 11–18**. Finish 08–10 (structs, enums, modules) first if you skipped them. Each topic has a paired exercise.

## Suggested order

| Step | Example | Exercise | Focus |
|------|---------|----------|--------|
| 1 | [`11-collections`](../examples/11-collections/) | [`exercises/11-collections`](../exercises/11-collections/) | `Vec`, `HashMap`, `HashSet` |
| 2 | [`12-error-handling`](../examples/12-error-handling/) | [`exercises/12-error-handling`](../exercises/12-error-handling/) | `Option`, `Result`, `?`, custom errors |
| 3 | [`13-generics`](../examples/13-generics/) | [`exercises/13-generics`](../exercises/13-generics/) | Type parameters & bounds |
| 4 | [`14-traits`](../examples/14-traits/) | [`exercises/14-traits`](../exercises/14-traits/) | Shared behavior, static dispatch |
| 5 | [`15-lifetimes`](../examples/15-lifetimes/) | [`exercises/15-lifetimes`](../exercises/15-lifetimes/) | Lifetime annotations |
| 6 | [`16-testing`](../examples/16-testing/) | [`exercises/16-testing`](../exercises/16-testing/) | Unit & integration tests |
| 7 | [`17-iterators`](../examples/17-iterators/) | [`exercises/17-iterators`](../exercises/17-iterators/) | Adapters: `map`, `filter`, `collect`, … |
| 8 | [`18-closures`](../examples/18-closures/) | [`exercises/18-closures`](../exercises/18-closures/) | `|x| …`, capture modes |

## Practical tips

- Prefer `Vec` / `HashMap` APIs over manual index fiddling; iterate with `for` or iterator adapters.
- Propagate errors with `?`; reserve `.unwrap()` / `.expect()` for tests or truly infallible cases (this workspace denies `unwrap_used` in clippy).
- Start generics with concrete code, then extract the shared shape.
- Implement traits for *your* types; use trait bounds (`T: Display`) rather than guessing concrete types everywhere.
- Lifetimes name relationships the borrow checker already enforces — if the compiler is happy without annotations, you often do not need them.
- Put unit tests next to the code (`#[cfg(test)]`); put cross-module API tests under `tests/`.
- Closures: default to `||`, add `move` only when the closure must own captured data (e.g. threads).

## Common mistakes

| Mistake | Fix |
|---------|-----|
| Indexing a `Vec` without bounds checks | Prefer `.get()`, iterators, or pattern matching |
| Ignoring `Result` / `Option` | Handle with `match`, `if let`, or `?` |
| Over-using `.clone()` to silence the borrow checker | Restructure borrows or ownership instead |
| Fighting lifetime errors by sprinkling `'static` | Fix the actual data flow; `'static` is rarely the right escape hatch |
| Writing only happy-path tests | Add edge cases and error paths (see example 16) |
| Consuming an iterator twice | Collect once, or use `.by_ref()` / clone the source |

## How to practice

Same loop as fundamentals: example README → `cargo run` → `cargo test` → paired exercise → next topic.

## The Rust Book

- [Ch 8 — Common Collections](https://doc.rust-lang.org/book/ch08-00-common-collections.html)
- [Ch 9 — Error Handling](https://doc.rust-lang.org/book/ch09-00-error-handling.html)
- [Ch 10 — Generic Types, Traits, and Lifetimes](https://doc.rust-lang.org/book/ch10-00-generics.html)
- [Ch 11 — Writing Automated Tests](https://doc.rust-lang.org/book/ch11-00-testing.html)
- [Ch 13 — Functional Language Features: Iterators and Closures](https://doc.rust-lang.org/book/ch13-00-functional-features.html)

Next: [03-advanced.md](03-advanced.md)
