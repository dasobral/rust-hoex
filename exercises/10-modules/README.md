# 10-modules Exercises

Multi-file module layout for password strength analysis — `pub`, `pub(crate)`, and re-exports.

## Overview

Security teams often split password tooling into scoring, risk classification, and denylist validation. This exercise crate mirrors that layout with a small module tree under `analyzer/`.

## Project layout

```bash
exercises/10-modules/
├── Cargo.toml
├── README.md
├── src/
│   ├── lib.rs              # Crate root + risk_label re-export
│   ├── main.rs             # clap CLI (list / all / score / validate)
│   ├── score.rs            # Exercise 1 demo
│   ├── validate.rs         # Exercise 2 demo
│   └── analyzer/
│       ├── mod.rs          # Analysis, analyze
│       ├── score.rs        # pub(crate) compute_score
│       ├── risk.rs         # RiskLevel
│       └── validate.rs     # pub(crate) is_too_common
└── tests/integration.rs
```

## Learning objectives

- [x] Organize code across `mod.rs` and sibling files
- [x] Use `pub(crate)` for crate-internal helpers
- [x] Re-export a tidy public API from `lib.rs`
- [x] Cap scores when `is_too_common` matches `"password"` or `"123456"` (case-insensitive)

## Running

```bash
cargo run -p exercise_modules
cargo run -p exercise_modules -- list
cargo run -p exercise_modules -- score --verbose
cargo test -p exercise_modules
cargo clippy -p exercise_modules --all-targets --all-features -- -D warnings
```

## Key API

```rust
let analysis = analyze("S3cret!");
assert_eq!(risk_label(&analysis), analysis.risk.as_str());
```

| Function | Visibility | Purpose |
|----------|------------|---------|
| `analyze` | public | Full analysis with common-password cap |
| `risk_label` | public | Re-export style risk string |
| `compute_score` | `pub(crate)` | Raw score before cap |
| `is_too_common` | `pub(crate)` | Denylist check |

## Related material

- Example: `examples/10-modules`
- [The Rust Book — Modules](https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html)
