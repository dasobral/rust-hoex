# 10-modules

Code organization in Rust: split a small analyzer into modules, control
visibility, and re-export a clean public API from the crate root.

## What this teaches

- Declaring modules with `mod` and files / directories
- The module tree (`lib.rs` → `analyzer` → `score` / `report`)
- Visibility: `pub`, `pub(crate)`, and private (default)
- `use` paths and crate-root re-exports
- Keeping `main.rs` thin while logic lives in the library

## Layout

```text
src/
  lib.rs              # crate root: `pub mod analyzer;` + `pub use ...`
  main.rs             # binary that calls the public API
  analyzer/
    mod.rs            # `pub mod score/report` + Analysis / RiskLevel / analyze
    score.rs          # pub(crate) scoring helpers
    report.rs         # pub(crate) report formatting
tests/
  integration.rs      # only the re-exported public API
```

## How to run

```bash
cd examples/10-modules
cargo run
cargo test
cargo clippy --all-targets -- -D warnings
cargo fmt
```

From the workspace root:

```bash
cargo run -p example_modules
cargo test -p example_modules
```

## Key concepts

### `mod` and the file tree

`mod analyzer;` in `lib.rs` loads `src/analyzer.rs` **or** `src/analyzer/mod.rs`.
Inside `analyzer/mod.rs`, `mod score;` loads `analyzer/score.rs`.

### Visibility

| Keyword       | Who can use it                                      |
|---------------|-----------------------------------------------------|
| (none)        | Only the current module (and its children)          |
| `pub(crate)`  | Anywhere inside this crate, not outside             |
| `pub`         | Outside the crate (if ancestors are also public)    |

`compute_score` is `pub(crate)` so `analyze` can call it, but integration tests
cannot. That keeps the public surface small.

### Re-exports

```rust
pub use analyzer::{analyze, Analysis, RiskLevel};
```

Callers write `use example_modules::analyze` instead of
`use example_modules::analyzer::analyze`. You can rearrange internals later
without breaking users.

### Paths

- `crate::analyzer::score` — absolute from the crate root
- `super::Analysis` — parent module
- `self::report` — current module

## Exercises

1. Add `analyzer/validate.rs` with `pub(crate) fn is_too_common(secret: &str) -> bool`
   and call it from `analyze` to cap the score for `"password"` / `"123456"`.
2. Re-export `RiskLevel::as_str` usage via a new `pub fn risk_label(a: &Analysis)`.
3. Move `RiskLevel` into its own `analyzer/risk.rs` file and update `mod.rs`.
4. Try calling `example_modules::analyzer::score::compute_score` from the
   integration test — the module path exists (`pub mod`), but `pub(crate)`
   still blocks outsiders. Confirm the compile error.

## Further reading

- [The Rust Book — Managing Growing Projects with Packages, Crates, and Modules](https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html)
- [Rust by Example — Modules](https://doc.rust-lang.org/rust-by-example/mod.html)
- [The Rust Reference — Visibility and Privacy](https://doc.rust-lang.org/reference/visibility-and-privacy.html)
