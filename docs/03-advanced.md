# Advanced

Smart pointers, concurrency, async, macros, and unsafe. Work through **examples 19–23**. These build on ownership, traits, and iterators — finish the intermediate track first.

## Suggested order

| Step | Example | Focus |
|------|---------|--------|
| 1 | [`19-smart-pointers`](../examples/19-smart-pointers/) | `Box`, `Rc`, `RefCell` |
| 2 | [`20-concurrency`](../examples/20-concurrency/) | Threads, `mpsc` |
| 3 | [`21-async`](../examples/21-async/) | `async`/`await`, Tokio |
| 4 | [`22-macros`](../examples/22-macros/) | Declarative `macro_rules!` |
| 5 | [`23-unsafe`](../examples/23-unsafe/) | Raw pointers & safe wrappers |

Capstone after this track: [`projects/cli-tools`](../projects/cli-tools/) (**seccheck**).

## Practical tips

- `Box<T>`: heap allocation / recursive types. `Rc<T>`: shared ownership (single-threaded). `RefCell<T>`: interior mutability with runtime borrow checks.
- Prefer message passing (`mpsc`) or clear ownership of shared state (`Arc` + `Mutex` when you need it) over ad-hoc shared mutability.
- Async: `.await` only inside `async` contexts; pick one runtime (here, Tokio) and stay consistent.
- Macros: start with small `macro_rules!` helpers; keep them readable — metaprogramming is easy to overuse.
- Prefer safe abstractions. Reach for `unsafe` only when you can state why the compiler cannot verify safety and you can uphold the invariants yourself.

## Unsafe — read this carefully

**`unsafe` does not turn off the borrow checker.** It only unlocks a few extra powers (dereferencing raw pointers, calling unsafe functions, etc.). You become responsible for memory safety in those blocks.

Rules of thumb for this tutorial (and beyond):

1. **Minimize** `unsafe` surface area — ideally a thin block behind a safe API.
2. Document **why** it is sound (`// SAFETY: …`) every time.
3. Never copy “unsafe snippets” from the internet without understanding aliasing, lifetimes, and validity.
4. Example 23 is intentionally tiny. It is not a license to sprinkle `unsafe` through learning code.
5. Workspace lints forbid `unsafe_code` by default in most crates; the unsafe example opts in deliberately.

If you are unsure whether you need `unsafe`, you almost certainly do not.

## Common mistakes

| Mistake | Fix |
|---------|-----|
| `Rc` + mutation without `RefCell` | Use `Rc<RefCell<T>>` (single-threaded) or rethink ownership |
| Sharing `RefCell` across threads | Use `Arc<Mutex<T>>` (or similar) instead |
| Blocking inside async tasks | Use async-aware APIs; do not `std::thread::sleep` on the runtime |
| Giant macros that obscure logic | Prefer functions; macros only when syntax sugar is worth it |
| `unsafe` without a safety comment or invariant | Write `SAFETY` docs and keep the unsafe region small |

## How to practice

README → `cargo run` → `cargo test` → README exercises → next. For 23, read every `SAFETY` comment before changing anything.

## The Rust Book

- [Ch 15 — Smart Pointers](https://doc.rust-lang.org/book/ch15-00-smart-pointers.html)
- [Ch 16 — Fearless Concurrency](https://doc.rust-lang.org/book/ch16-00-concurrency.html)
- [Ch 17 — Async and Await](https://doc.rust-lang.org/book/ch17-00-async-await.html)
- [Ch 20 — Unsafe Rust](https://doc.rust-lang.org/book/ch20-01-unsafe-rust.html)
- [Ch 20 — Macros](https://doc.rust-lang.org/book/ch20-05-macros.html)

Also useful: [Tokio tutorial](https://tokio.rs/tokio/tutorial), [The Rustonomicon](https://doc.rust-lang.org/nomicon/) (advanced / unsafe deep dive).

Back to the full checklist: [LEARNING_PATH.md](LEARNING_PATH.md)
