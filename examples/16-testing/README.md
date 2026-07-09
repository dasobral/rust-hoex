# 16-testing

Unit and integration tests for a small password-policy library. The library
exists so you have something meaningful to test — the lesson is the harness.

## What this teaches

- `#[cfg(test)]` modules and `#[test]` functions
- `assert!` / `assert_eq!`
- Nested test modules for organization
- `#[should_panic]` (OK in tests; panic is denied in non-test code)
- `Result`-returning tests with `?`
- Integration tests under `tests/`
- Filtering with `cargo test <filter>`

## How to run

```bash
cd examples/16-testing
cargo run
cargo test
cargo test policy              # name filter
cargo test --lib               # unit tests only
cargo test --test integration  # integration target
cargo test -- --nocapture
cargo clippy --all-targets -- -D warnings
```

From the workspace root:

```bash
cargo test -p example_testing
cargo test -p example_testing policy
```

## Key concepts

### Unit vs integration

| Kind        | Location              | Sees private items? |
|-------------|-----------------------|---------------------|
| Unit        | `src/**` + `#[cfg(test)]` | Yes              |
| Integration | `tests/*.rs`          | Public API only     |

### `should_panic`

```rust
#[test]
#[should_panic(expected = "password must not be empty")]
fn assert_nonempty_panics_on_empty() {
    assert_nonempty("");
}
```

Use sparingly. Prefer APIs that return `Result` / `Option`.

### Result tests

```rust
#[test]
fn check_or_err_ok_on_strong() -> Result<(), String> {
    check_or_err("Str0ngPass")?;
    Ok(())
}
```

## Exercises

1. Add a rule requiring one punctuation character; cover it with unit + integration tests.
2. Mark a slow test with `#[ignore]` and run it via `cargo test -- --ignored`.
3. Add a test that fails on purpose, watch the output, then fix it.

## Further reading

- [The Rust Book — How to Write Tests](https://doc.rust-lang.org/book/ch11-00-testing.html)
- [Cargo Book — `cargo test`](https://doc.rust-lang.org/cargo/commands/cargo-test.html)
