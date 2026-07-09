# Learning path

Ordered checklist for rust-hoex. Work top to bottom.

## How to study each example

1. Read the example’s `README.md`
2. Run it: `cd examples/<dir> && cargo run` (or `cargo run -p <package>`)
3. Run tests: `cargo test`
4. Try the exercises / challenges in that README
5. Move to the next item

Do not skip ownership/borrowing (06–07); everything later depends on them.

---

## 0. Orientation

- [ ] Install Rust 1.85+ via [rustup](https://rustup.rs/) (see root [README](../README.md))
- [ ] Optional: `./setup.sh` and `source utils/aliases.sh`
- [ ] Read [00-basics.md](00-basics.md) — repo layout, `main`, tests, `todo!` in scaffolds

## 1. Fundamentals

Guide: [01-fundamentals.md](01-fundamentals.md)

- [ ] `01-helloWorld` — first program
- [ ] `02-variables` — `let` / `mut` / shadowing
- [ ] `03-dataTypes` — scalars & compounds
- [ ] `04-functions` — parameters & returns
- [ ] `05-controlFlow` — if, loops, match
- [ ] `06-ownership` — moves, Clone, Copy, Drop
- [ ] `07-borrowing` — references & borrow rules
- [ ] `08-structs` — custom types & methods
- [ ] `09-enums` — enums & pattern matching
- [ ] `10-modules` — modules & visibility

**Practice (after the matching example):**

- [ ] `exercises/02-variables` — physics-themed advanced variables
- [ ] `exercises/06-ownership` — moves, Clone, Copy, zeroize
- [ ] `exercises/07-borrowing` — references, slices, strength scoring
- [ ] `exercises/08-structs` — UserAccount / Session models
- [ ] `exercises/09-enums` — AuthStatus, NetworkEvent, HttpStatus

## 2. Intermediate

Guide: [02-intermediate.md](02-intermediate.md)

- [ ] `11-collections`
- [ ] `12-error-handling`
- [ ] `13-generics`
- [ ] `14-traits`
- [ ] `15-lifetimes`
- [ ] `16-testing`
- [ ] `17-iterators`
- [ ] `18-closures`

**Practice:**

- [ ] `exercises/11-collections` — intrusion log aggregator
- [ ] `exercises/12-error-handling` — config/credential Result flows
- [ ] `exercises/14-traits` — ThreatScorer implementations

## 3. Advanced

Guide: [03-advanced.md](03-advanced.md)

- [ ] `19-smart-pointers`
- [ ] `20-concurrency`
- [ ] `21-async`
- [ ] `22-macros`
- [ ] `23-unsafe` — read the unsafe warnings in the guide and example README

## 4. Capstone project

- [ ] `projects/cli-tools` (**seccheck**) — password-auditing CLI consolidating earlier concepts

```bash
cd projects/cli-tools && cargo run -- --help
# or: cargo run -p project_cli_tools -- --help
```

## 5. Quality check (optional)

From the repo root:

```bash
cargo test --workspace
./utils/check.sh
```

---

## Quick package names

| Path | `-p` name |
|------|-----------|
| `examples/01-helloWorld` … `23-unsafe` | `example_helloworld` … `example_unsafe` (see root README table) |
| `exercises/02-variables` | `exercise_variables` |
| `exercises/06-ownership` | `exercise_ownership` |
| `exercises/07-borrowing` | `exercise_borrowing` |
| `exercises/08-structs` | `exercise_structs` |
| `exercises/09-enums` | `exercise_enums` |
| `exercises/11-collections` | `exercise_collections` |
| `exercises/12-error-handling` | `exercise_errorhandling` |
| `exercises/14-traits` | `exercise_traits` |
| `projects/cli-tools` | `project_cli_tools` |
