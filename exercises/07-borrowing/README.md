# 07-borrowing Exercises

References and borrowing — **inspect and score passwords without taking ownership**,
update threat metrics via mutable borrows, and aggregate history from slice views.

## Overview

These exercises continue the security theme from [`examples/07-borrowing`](../../examples/07-borrowing).
You implement helpers that read password traits through `&str`, mutate a score through
`&mut i32`, and summarize arrays via `&[i32]` — all while the original owner keeps
the data.

## Learning Objectives

After completing these exercises, you should understand:

- [x] **Shared references** (`&T`) — read-only access without move
- [x] **Mutable references** (`&mut T`) — exclusive write access
- [x] **Borrow checker rules** — many `&` XOR one `&mut`
- [x] **Slices** — `&str` and `&[T]` as views into contiguous data
- [x] **Safe display** — mask secrets for logs without consuming them

## Project Layout

```text
exercises/07-borrowing/
├── Cargo.toml
├── README.md
├── src/
│   ├── lib.rs       # Public API and orchestration
│   ├── main.rs      # clap CLI (list / all / subcommands)
│   ├── password.rs  # Core borrowing helpers
│   ├── inspect.rs   # Immutable-borrow exercise
│   └── policy.rs    # Mutable-borrow exercise
└── tests/
    └── integration.rs
```

## Running the Exercises

From the workspace root:

```bash
cargo run -p exercise_borrowing
cargo run -p exercise_borrowing -- list
cargo run -p exercise_borrowing -- inspect --verbose
cargo run -p exercise_borrowing -- policy
cargo run -p exercise_borrowing -- all
```

Quality checks:

```bash
cargo test -p exercise_borrowing
cargo clippy -p exercise_borrowing --all-targets --all-features -- -D warnings
cargo fmt -p exercise_borrowing
```

## Exercises

### 1. `inspect` — Read passwords via immutable borrows

Analyze a password's first character, digit count, policy compliance, and masked
display — multiple `&str` borrows at once.

**Functions:** `first_char`, `count_digits`, `meets_policy`, `mask_keep_last`

### 2. `policy` — Score passwords and aggregate threat history

Update a mutable threat score from password features and compute the average of
a score history slice.

**Functions:** `meets_policy`, `update_strength`, `average_scores`

## Public API

| Function | Signature | Concept |
|----------|-----------|---------|
| `first_char` | `(&str) -> Option<char>` | Immutable borrow |
| `count_digits` | `(&str) -> usize` | Iterate without move |
| `meets_policy` | `(&str, usize) -> bool` | Policy check |
| `update_strength` | `(&mut i32, &str)` | Mutable borrow |
| `average_scores` | `(&[i32]) -> Option<i32>` | Slice borrow |
| `mask_keep_last` | `(&str, usize) -> String` | Borrow in, owned out |

## Scoring Rules (`update_strength`)

| Condition | Points |
|-----------|--------|
| Length > 12 | +10 |
| Uppercase letter | +5 |
| ASCII digit | +5 |
| Symbol (non-alphanumeric) | +10 |

## Key Takeaways

1. **Borrowing avoids moves** — inspect credentials without consuming them.
2. **One `&mut` at a time** — exclusive mutation prevents data races at compile time.
3. **Slices are views** — `&[i32]` works for arrays and vectors alike.
4. **Mask for display** — never log raw secrets; borrow and redact instead.

## Related Material

- [`examples/07-borrowing`](../../examples/07-borrowing) — introductory walkthrough
- [`exercises/06-ownership`](../06-ownership) — prerequisite: ownership rules
