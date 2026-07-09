# Learning path

Ordered checklist for rust-hoex. Work top to bottom.

## How to study each topic

1. Read the example’s `README.md`
2. Run it: `cd examples/<dir> && cargo run` (or `cargo run -p <package>`)
3. Run tests: `cargo test`
4. Do the paired exercise under `exercises/<same-dir>/`
5. Move to the next item

Do not skip ownership/borrowing (06–07); everything later depends on them.

---

## 0. Orientation

- [ ] Install Rust 1.85+ via [rustup](https://rustup.rs/) (see root [README](../README.md))
- [ ] Optional: `./setup.sh` and `source utils/aliases.sh`
- [ ] Read [00-basics.md](00-basics.md) — repo layout, `main`, tests, `todo!` in scaffolds

## 1. Fundamentals

Guide: [01-fundamentals.md](01-fundamentals.md)

| Example | Exercise |
|---------|----------|
| [ ] `01-helloWorld` | [ ] `exercises/01-helloWorld` |
| [ ] `02-variables` | [ ] `exercises/02-variables` |
| [ ] `03-dataTypes` | [ ] `exercises/03-dataTypes` |
| [ ] `04-functions` | [ ] `exercises/04-functions` |
| [ ] `05-controlFlow` | [ ] `exercises/05-controlFlow` |
| [ ] `06-ownership` | [ ] `exercises/06-ownership` |
| [ ] `07-borrowing` | [ ] `exercises/07-borrowing` |
| [ ] `08-structs` | [ ] `exercises/08-structs` |
| [ ] `09-enums` | [ ] `exercises/09-enums` |
| [ ] `10-modules` | [ ] `exercises/10-modules` |

## 2. Intermediate

Guide: [02-intermediate.md](02-intermediate.md)

| Example | Exercise |
|---------|----------|
| [ ] `11-collections` | [ ] `exercises/11-collections` |
| [ ] `12-error-handling` | [ ] `exercises/12-error-handling` |
| [ ] `13-generics` | [ ] `exercises/13-generics` |
| [ ] `14-traits` | [ ] `exercises/14-traits` |
| [ ] `15-lifetimes` | [ ] `exercises/15-lifetimes` |
| [ ] `16-testing` | [ ] `exercises/16-testing` |
| [ ] `17-iterators` | [ ] `exercises/17-iterators` |
| [ ] `18-closures` | [ ] `exercises/18-closures` |

## 3. Advanced

Guide: [03-advanced.md](03-advanced.md)

| Example | Exercise |
|---------|----------|
| [ ] `19-smart-pointers` | [ ] `exercises/19-smart-pointers` |
| [ ] `20-concurrency` | [ ] `exercises/20-concurrency` |
| [ ] `21-async` | [ ] `exercises/21-async` |
| [ ] `22-macros` | [ ] `exercises/22-macros` |
| [ ] `23-unsafe` | [ ] `exercises/23-unsafe` — read the unsafe warnings first |

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
| `examples/01-helloWorld` … `23-unsafe` | `example_helloworld` … `example_unsafe` |
| `exercises/01-helloWorld` … `23-unsafe` | `exercise_helloworld` … `exercise_unsafe` |
| `projects/cli-tools` | `project_cli_tools` |

See the root [README](../README.md) for the full directory ↔ package table.
