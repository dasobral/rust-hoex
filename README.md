# Rust Hands-On EXamples (rust-hoex)

A personal, hands-on Rust tutorial. Progressive examples use cybersecurity-flavored scenarios (passwords, logs, threat scores, packet headers) so concepts stick while you learn the language.

Requires **Rust 1.85+** (edition 2024).

## First steps

```bash
# Install rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# Verify
rustc --version
cargo --version

# Useful components (usually installed already)
rustup component add clippy rustfmt rust-src
```

Optional: clone this repo, then run `./setup.sh` and add `source /path/to/rust-hoex/utils/aliases.sh` to your shell rc.

## Repository structure

```text
rust-hoex/
├── Cargo.toml              # Workspace root
├── docs/                   # Learning guides
│   ├── 00-basics.md
│   ├── 01-fundamentals.md
│   ├── 02-intermediate.md
│   ├── 03-advanced.md
│   └── LEARNING_PATH.md
├── examples/               # Progressive tutorial (01–23)
├── exercises/              # One practice crate per example topic (01–23)
├── projects/               # Capstone work
│   └── cli-tools/          # seccheck password-auditing CLI
└── utils/
    ├── aliases.sh
    ├── check.sh
    └── creator.sh
```

## Learning path

Follow [docs/LEARNING_PATH.md](docs/LEARNING_PATH.md). Suggested loop for each example: read its README → `cargo run` → `cargo test` → try the README exercises → next.

### Examples (01–23)

| # | Directory | Package | One-liner |
|---|-----------|---------|-----------|
| 01 | `01-helloWorld` | `example_helloworld` | First program: `println!`, `format!`, lib vs bin |
| 02 | `02-variables` | `example_variables` | Variables, mutability, shadowing (crypto entropy theme) |
| 03 | `03-dataTypes` | `example_datatypes` | Scalar & compound types (packet headers) |
| 04 | `04-functions` | `example_functions` | Function syntax (network checksums) |
| 05 | `05-controlFlow` | `example_controlflow` | if/else, loops, match (threat-score classifier) |
| 06 | `06-ownership` | `example_ownership` | Moves, Clone, Copy, Drop (secure passwords) |
| 07 | `07-borrowing` | `example_borrowing` | References & borrowing without taking ownership |
| 08 | `08-structs` | `example_structs` | Structs & methods (user/credential models) |
| 09 | `09-enums` | `example_enums` | Enums & pattern matching (auth/network events) |
| 10 | `10-modules` | `example_modules` | Modules, visibility, re-exports |
| 11 | `11-collections` | `example_collections` | Vec, HashMap, HashSet (intrusion logs) |
| 12 | `12-error-handling` | `example_errorhandling` | Option, Result, `?`, custom errors |
| 13 | `13-generics` | `example_generics` | Generics & trait bounds (`SecureContainer`) |
| 14 | `14-traits` | `example_traits` | Traits & static dispatch (`ThreatScorer`) |
| 15 | `15-lifetimes` | `example_lifetimes` | Lifetime annotations on fns and structs |
| 16 | `16-testing` | `example_testing` | Unit & integration tests (password policy) |
| 17 | `17-iterators` | `example_iterators` | Iterator adapters (log lines & IPs) |
| 18 | `18-closures` | `example_closures` | Closures for sorting/filtering threats |
| 19 | `19-smart-pointers` | `example_smartpointers` | Box, Rc, RefCell (rule trees & shared config) |
| 20 | `20-concurrency` | `example_concurrency` | Threads & mpsc (parallel log processing) |
| 21 | `21-async` | `example_async` | Async/await with Tokio (offline simulated fetches) |
| 22 | `22-macros` | `example_macros` | Declarative macros (`say!`, `testvec!`, …) |
| 23 | `23-unsafe` | `example_unsafe` | Minimal unsafe: raw pointers + safe wrappers |

### Exercises (01–23, one per example topic)

Each example has a matching exercise under the same directory name. Package names use the `exercise_*` prefix (e.g. `exercises/03-dataTypes` → `exercise_datatypes`).

| # | Directory | Package | Focus |
|---|-----------|---------|--------|
| 01 | `01-helloWorld` | `exercise_helloworld` | `println!` / `format!`, banners, masking |
| 02 | `02-variables` | `exercise_variables` | Signed ints, overflow, conversions (physics) |
| 03 | `03-dataTypes` | `exercise_datatypes` | Packet header scalars & compounds |
| 04 | `04-functions` | `exercise_functions` | Checksums, seal/verify helpers |
| 05 | `05-controlFlow` | `exercise_controlflow` | Threat-score / log classifier |
| 06 | `06-ownership` | `exercise_ownership` | Moves, Clone, Copy, zeroize-before-drop |
| 07 | `07-borrowing` | `exercise_borrowing` | `&` / `&mut`, slices, policy scoring |
| 08 | `08-structs` | `exercise_structs` | UserAccount, Session, lockout |
| 09 | `09-enums` | `exercise_enums` | AuthStatus, NetworkEvent, HttpStatus |
| 10 | `10-modules` | `exercise_modules` | Multi-file analyzer, visibility |
| 11 | `11-collections` | `exercise_collections` | IntrusionLog: Vec / HashMap / HashSet |
| 12 | `12-error-handling` | `exercise_errorhandling` | ConfigError, `?`, allow-listed paths |
| 13 | `13-generics` | `exercise_generics` | `SecureContainer`, `find_min`, `Pair` |
| 14 | `14-traits` | `exercise_traits` | ThreatScorer, default methods, bounds |
| 15 | `15-lifetimes` | `exercise_lifetimes` | `shortest`, borrowed excerpts |
| 16 | `16-testing` | `exercise_testing` | Password policy + unit/integration/`#[ignore]` |
| 17 | `17-iterators` | `exercise_iterators` | Log adapters: filter / map / sum |
| 18 | `18-closures` | `exercise_closures` | Sort, partition, capturing predicates |
| 19 | `19-smart-pointers` | `exercise_smartpointers` | `Box` rules, `Rc`/`RefCell` sharing |
| 20 | `20-concurrency` | `exercise_concurrency` | Threads + mpsc log workers |
| 21 | `21-async` | `exercise_async` | Tokio fetches, timeout, join |
| 22 | `22-macros` | `exercise_macros` | `say!`, `testvec!`, `max_of!` |
| 23 | `23-unsafe` | `exercise_unsafe` | Safe wrappers over raw pointers |

- **`projects/cli-tools`** (`project_cli_tools`) — **seccheck**, a password-auditing CLI capstone

Guides: [docs/00-basics.md](docs/00-basics.md) → [01-fundamentals](docs/01-fundamentals.md) → [02-intermediate](docs/02-intermediate.md) → [03-advanced](docs/03-advanced.md)

## How to run

From an example directory:

```bash
cd examples/01-helloWorld
cargo run
cargo test
```

Or from the workspace root by package name:

```bash
cargo run -p example_helloworld
cargo test -p example_helloworld
```

Same pattern for any exercise (`exercise_helloworld` … `exercise_unsafe`) and the project (`project_cli_tools`).

## Workspace commands

```bash
cargo test --workspace          # all tests
cargo build --workspace         # build everything
cargo check --workspace         # fast compile check
./utils/check.sh                # fmt + clippy (-D warnings) + tests + more
```

## Creating new modules

```bash
./utils/creator.sh example 24-foo "Short description"
./utils/creator.sh exercise bar "Practice problem"
./utils/creator.sh project baz "Larger project"
```

Or after `source utils/aliases.sh`:

```bash
rust-example 24-foo "Short description"
rust-exercise bar "Practice problem"
rust-project baz "Larger project"
rust-check    # ./utils/check.sh
rust-test     # cargo test --workspace
rust-build
rust-fmt
rust-clippy
```

New scaffolds include a `todo!()` placeholder — replace it as you implement.

## Resources

- [The Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Rustlings](https://github.com/rust-lang/rustlings)
- [Rust Cookbook](https://rust-lang-nursery.github.io/rust-cookbook/)
- [Cargo Book](https://doc.rust-lang.org/cargo/)

## License

MIT — see [LICENSE](LICENSE).
