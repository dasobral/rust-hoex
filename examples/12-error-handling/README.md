# 12-error-handling

Config path validation and credential parsing with `Option`, `Result`, the `?`
operator, and a custom error enum (`thiserror`).

## What this teaches

- `Option<T>` for absence (`Some` / `None`) — e.g. optional API tokens
- `Result<T, E>` for fallible work (`Ok` / `Err`) — e.g. path validation
- Propagating errors with `?` instead of nested `match`
- Custom error types with `thiserror` (`#[derive(Error)]` + `#[error("...")]`)
- Transforming values with `map` and chaining with `and_then`-style logic

## How to run

```bash
cd examples/12-error-handling
cargo run
cargo test
cargo clippy --all-targets -- -D warnings
cargo fmt
```

From the workspace root:

```bash
cargo run -p example_errorhandling
cargo test -p example_errorhandling
```

## Key concepts

### `Option` vs `Result`

| Type | Meaning | Typical use |
|------|---------|-------------|
| `Option<T>` | Value may be missing | Optional token, lookup miss |
| `Result<T, E>` | Operation may fail | Parse, validate, I/O |

Absence is not always an error — `parse_optional_token("")` returns `None`.

### `?` operator

```rust
pub fn load_settings(path: &str, token_raw: &str) -> Result<(ConfigPath, Option<String>), ConfigError> {
    let config = validate_config_path(path)?; // returns early on Err
    let token = parse_optional_token(token_raw).map(|t| mask_token(&t));
    Ok((config, token))
}
```

`?` desugars to: on `Err`, return it from the enclosing function; on `Ok`, unwrap.

### Custom errors with `thiserror`

```rust
#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("config path is empty")]
    EmptyPath,
    #[error("config path must be absolute: {0}")]
    NotAbsolute(String),
    // ...
}
```

`Display` (and `std::error::Error`) come for free — great for teaching and for
real libraries.

### `map` / `and_then`

- `option.map(f)` — transform `Some`, leave `None`
- `result.map(f)` — transform `Ok`, leave `Err`
- `optional_credential` shows turning `Option<&str>` into `Result<Option<String>, _>`

## Exercises

1. Add `ConfigError::MissingFile` and a `fn ensure_exists(path: &ConfigPath) -> Result<(), ConfigError>`
   that checks a hardcoded allow-list of paths (no real filesystem needed).
2. Rewrite `optional_credential` using `transpose` on `Option<Result<_,_>>`.
3. Add `fn parse_port(s: &str) -> Result<u16, ConfigError>` with a new error variant.
4. In `main`, replace `match` with `if let Err(e) = ...` for one of the demos.

## Further reading

- [The Rust Book — Error Handling](https://doc.rust-lang.org/book/ch09-00-error-handling.html)
- [thiserror](https://docs.rs/thiserror)
- [Rust by Example — Result](https://doc.rust-lang.org/rust-by-example/error/result.html)
