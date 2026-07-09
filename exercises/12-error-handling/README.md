# 12-error-handling Exercises

Config path and credential validation with `Option`, `Result`, the `?` operator, and custom errors via `thiserror`.

## Overview

Before a security sensor starts, it must validate configuration paths, confirm files exist on an approved deployment manifest, parse service ports, and verify credentials. This crate walks through those checks without touching the real filesystem.

## Project layout

```bash
exercises/12-error-handling/
├── Cargo.toml
├── README.md
├── src/
│   ├── lib.rs
│   ├── main.rs
│   ├── config.rs           # ConfigError, ConfigPath, validate, ensure_exists, parse_port
│   ├── credentials.rs      # token/credential helpers
│   ├── bootstrap.rs        # secure_bootstrap, load_settings, bind_service
│   ├── paths.rs            # Exercise 1
│   └── credentials_exercise.rs  # Exercise 2
└── tests/integration.rs
```

## Learning objectives

- [x] Distinguish `Option` (absence) from `Result` (failure)
- [x] Propagate errors with `?`
- [x] Define `ConfigError` with `thiserror`
- [x] Use the **transpose** pattern in `optional_credential`
- [x] Chain validation in `secure_bootstrap`

## Running

```bash
cargo run -p exercise_errorhandling
cargo run -p exercise_errorhandling -- bootstrap --verbose
cargo test -p exercise_errorhandling
cargo clippy -p exercise_errorhandling --all-targets --all-features -- -D warnings
cargo fmt -p exercise_errorhandling
```

## ConfigError variants

| Variant | When |
|---------|------|
| `InvalidPath(String)` | Empty, relative, traversal, or bad extension |
| `EmptyCredential` | Missing/short credential or whitespace |
| `MissingFile(String)` | Path not on hardcoded allow-list |
| `InvalidPort { raw }` | Non-numeric or zero port |

## Key functions

```rust
let path = validate_config_path("/etc/soc/config.toml")?;
ensure_exists(&path)?;
let port = parse_port("8443")?;
let cred = require_credential("hunter2-secret")?;
let (cfg, secret) = secure_bootstrap("/etc/soc/config.toml", "hunter2-secret")?;
```

### `optional_credential` — transpose pattern

```rust
pub fn optional_credential(token: Option<&str>) -> Result<Option<String>, ConfigError> {
    token.map(require_credential).transpose()
}
```

`Option<Result<T, E>>` becomes `Result<Option<T>, E>`: absent token → `Ok(None)`, bad token → `Err`, good token → `Ok(Some(...))`.

### Allow-list (no real FS)

`ensure_exists` checks against a static list:

- `/etc/soc/config.toml`
- `/opt/sensor/config.yaml`
- `/var/secure/app.yml`
- `/etc/firewall/rules.toml`

## Exercises

1. **paths** — `validate_config_path`, `ensure_exists`, `parse_port`
2. **credentials** — `parse_optional_token`, `require_credential`, `optional_credential`
3. **bootstrap** — `secure_bootstrap`, `load_settings`, `bind_service`

## Option vs Result

| Type | Meaning | Example |
|------|---------|---------|
| `Option<T>` | Value may be absent | Optional API token |
| `Result<T, E>` | Operation may fail | Path validation |

## Testing

Unit tests live beside each module; integration tests in `tests/integration.rs` run the full pipeline.

## Related material

- Example: `examples/12-error-handling`
- [The Rust Book — Error Handling](https://doc.rust-lang.org/book/ch09-00-error-handling.html)
- [thiserror docs](https://docs.rs/thiserror)

## Stretch goals

1. Add `ConfigError::InvalidHost` and validate bind addresses
2. Implement `Display`-based logging middleware that never prints raw secrets
3. Compose `secure_bootstrap` with `parse_port` into a single `launch_config` function
