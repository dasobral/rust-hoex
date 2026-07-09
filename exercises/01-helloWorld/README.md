# 01-helloWorld Exercises

Cybersecurity-flavored **greetings**, **operator banners**, and **string formatting**
basics — your first hands-on step beyond the hello-world example.

## Overview

These exercises extend [`examples/01-helloWorld`](../../examples/01-helloWorld) with
security-themed text helpers: greet an operator, print a tool banner, join status
lines, and redact secrets before logging.

## Learning Objectives

After completing these exercises, you should understand:

- [x] **`format!`** — build owned `String` values from templates
- [x] **String slices (`&str`)** — borrow text without copying
- [x] **Multi-line output** — join slices with `\n` for status blocks
- [x] **Redaction** — mask sensitive values while preserving length
- [x] **CLI wiring** — `clap` subcommands that call library functions

## Project Layout

```text
exercises/01-helloWorld/
├── Cargo.toml
├── README.md
├── src/
│   ├── lib.rs      # Public API and exercise runners
│   └── main.rs     # clap CLI (list / all / greet / banner)
└── tests/
    └── integration.rs
```

## Running the Exercises

```bash
cargo run -p exercise_helloworld
cargo run -p exercise_helloworld -- list
cargo run -p exercise_helloworld -- greet --name Analyst
cargo run -p exercise_helloworld -- banner --tool nmap
cargo run -p exercise_helloworld -- all --verbose
```

Quality checks:

```bash
cargo test -p exercise_helloworld
cargo clippy -p exercise_helloworld --all-targets --all-features -- -D warnings
cargo fmt -p exercise_helloworld
```

## Public API

| Function | Signature | Purpose |
|----------|-----------|---------|
| `greet` | `(&str) -> String` | `"Hello, {name}!"` greeting |
| `security_banner` | `(&str) -> String` | ASCII security console banner |
| `join_lines` | `(&[&str]) -> String` | Join lines with newlines |
| `mask_secret` | `(&str) -> String` | Replace chars with `*` |

## Exercises

### 1. `greet` — Operator greetings and redaction

Prints a personalized greeting, a multi-line session status block, and a masked
API key suitable for operator logs.

### 2. `banner` — Security tool branding

Prints a fixed-width ASCII banner identifying the active security tool — the kind
of header you see when launching scanners or SIEM CLIs.

## Key Takeaways

1. **Keep logic in `lib.rs`** — binaries and integration tests share the same API.
2. **`format!` returns `String`** — owned, growable text for return values and logs.
3. **Redact before logging** — never print raw credentials; mask or hash instead.
4. **Thin `main`** — parse CLI args, delegate to library functions, propagate errors with `?`.

## Related Material

- [`examples/01-helloWorld`](../../examples/01-helloWorld) — introductory walkthrough
- [`examples/02-variables`](../../examples/02-variables) — next topic: variables and mutability
