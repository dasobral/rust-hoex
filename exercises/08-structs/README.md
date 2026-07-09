# 08-structs Exercises

Struct exercises for modeling **user accounts**, **credentials**, and **session tokens** in a cybersecurity context.

## Overview

Practice Rust structs through access-control scenarios: roles, lockout policies, private credential fields, and session token validation.

## Learning Objectives

- **Struct definition** with public and private fields
- **`impl` blocks** — `new`, `is_admin`, `verify_credential`, `into_audit_line`
- **Method receivers** — `&self`, `&mut self`, `self`
- **Associated constants** — `UserAccount::LOCK_THRESHOLD`
- **Session modeling** with `PartialEq` and token matching

## Running

```bash
cargo run -p exercise_structs
cargo run -p exercise_structs -- list
cargo run -p exercise_structs -- auth-flow --verbose
cargo run -p exercise_structs -- lockout
cargo test -p exercise_structs
cargo clippy -p exercise_structs --all-targets -- -D warnings
cargo fmt -p exercise_structs -- --check
```

## Exercises

### auth-flow

Create accounts, authenticate credentials, mint sessions, and observe lockout after failed attempts.

### lockout

Simulate credential-stuffing probes against a viewer account until the lockout threshold triggers.

## Key Types

```rust
pub enum Role { Viewer, Operator, Admin }

pub struct UserAccount {
    pub username: String,
    pub email: String,
    pub role: Role,
    pub failed_logins: u32,
    pub locked: bool,
    // credential is private
}

pub struct Session {
    pub user: String,
    pub token: String,
}
```

## Related

- `examples/08-structs` — introductory struct walkthrough
- `exercises/09-enums` — enum-based auth and network events
