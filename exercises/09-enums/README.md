# 09-enums Exercises

Enum exercises for **authentication outcomes**, **network incidents**, **HTTP status codes**, and **token fallbacks**.

## Overview

Practice Rust enums through cybersecurity scenarios: auth results, packet events, API gateway responses, and optional bearer tokens.

## Learning Objectives

- **Enum variants** — unit, tuple, and struct-like
- **Exhaustive `match`** on `AuthStatus` and `NetworkEvent`
- **`HttpStatus::from_code`** for common HTTP codes
- **`MaybeToken::unwrap_or`** as an `Option`-like pattern
- **`sum_packet_bytes`** aggregating received and sent traffic

## Running

```bash
cargo run -p exercise_enums
cargo run -p exercise_enums -- list
cargo run -p exercise_enums -- auth-status --verbose
cargo run -p exercise_enums -- network
cargo test -p exercise_enums
cargo clippy -p exercise_enums --all-targets -- -D warnings
cargo fmt -p exercise_enums -- --check
```

## Exercises

### auth-status

Walk through `Success`, `Failure`, `Pending`, and `Locked{until}` variants with `is_authenticated` and `summary`.

### network

Log network events, sum packet bytes, map HTTP codes, and demonstrate token fallbacks.

## Key Types

```rust
pub enum AuthStatus {
    Success { user: String },
    Failure { reason: String },
    Pending,
    Locked { until: String },
}

pub enum NetworkEvent {
    Idle,
    PacketReceived(String, u32),
    PacketSent(String, u32),
    ConnectionClosed { peer: String, reason: Option<String> },
}

pub fn sum_packet_bytes(events: &[NetworkEvent]) -> u32;
```

## Related

- `examples/09-enums` — introductory enum walkthrough
- `exercises/08-structs` — struct-based account and session modeling
