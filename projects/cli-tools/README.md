# seccheck — CLI tools capstone

A small but real password-auditing CLI that consolidates concepts from the
rust-hoex examples into one package (`project_cli_tools`).

## What this consolidates

| Concept | Where it shows up |
| --- | --- |
| **clap** derive + subcommands | `src/main.rs` (`entropy`, `analyze`, `batch`) |
| **Result / anyhow** | fallible I/O, empty-password errors, `?` throughout the binary |
| **structs & enums** | `EntropyEstimate`, `AnalysisReport`, `CharClass`, `Strength` |
| **collections** | `HashSet` for character classes, `HashMap` for class counts, `Vec` for findings |
| **iterators** | class detection, batch stdin lines, formatting, denylist checks |

## Layout

```
projects/cli-tools/
├── Cargo.toml
├── README.md
├── src/
│   ├── main.rs      # clap CLI binary
│   ├── lib.rs       # crate root / re-exports
│   ├── entropy.rs   # Shannon-style entropy
│   └── analyze.rs   # strength rating + heuristics
└── tests/
    └── integration.rs
```

## Run

From the repository root:

```bash
# Entropy only
cargo run -p project_cli_tools -- entropy 's3cret!'

# Full analysis
cargo run -p project_cli_tools -- analyze 'Tr0ub4dor&3xY!'

# Batch: one password per stdin line
printf 'password\nGoodPass1!\nXk9$mQ2!pL7#vN4@\n' \
  | cargo run -p project_cli_tools -- batch

# Skip blank lines in batch mode
printf 'a\n\nb\n' | cargo run -p project_cli_tools -- batch --skip-empty
```

## Test & lint

```bash
cargo test -p project_cli_tools
cargo clippy -p project_cli_tools --all-targets -- -D warnings
```

## Notes

- Entropy is a teaching approximation (`len * log2(alphabet)`), not a
  cryptographic guarantee.
- Batch mode redacts password previews in output (`a****z`) so logs are safer
  to share.
- Non-test code avoids `unwrap` / `expect` / `panic!` / `todo!`.
