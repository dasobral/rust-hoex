# 09-enums

Enums and pattern matching through auth results, network events, and HTTP-like
status codes.

## What this teaches

- Enum variants: **unit**, **tuple**, and **struct-like**
- Exhaustive `match` (the compiler forces every case)
- Methods on enums via `impl`
- `if let` and `while let` for single-pattern binding
- A custom `Option`-like enum (`MaybeToken`)

## How to run

```bash
cd examples/09-enums
cargo run
cargo test
cargo clippy --all-targets -- -D warnings
cargo fmt
```

From the workspace root:

```bash
cargo run -p example_enums
cargo test -p example_enums
```

## Key concepts

### Variant shapes

```rust
enum AuthStatus {
    Success { user: String },   // struct-like
    Failure { reason: String }, // struct-like
    Pending,                    // unit
}

enum NetworkEvent {
    Idle,                                    // unit
    PacketReceived(String, u32),             // tuple
    ConnectionClosed { peer: String, reason: Option<String> }, // struct-like
}
```

### Exhaustive `match`

When you `match` on an enum, Rust requires a branch for every variant. Leave one
out and you get a compile error — that is a feature, not a nuisance.

### `if let` / `while let`

Use these when you only care about one pattern:

- `if let AuthStatus::Success { user } = status { ... }`
- `while let Some(NetworkEvent::PacketReceived(_, n)) = queue.pop() { ... }`

### Methods on enums

`impl AuthStatus { ... }` attaches behavior (`is_authenticated`, `summary`) the
same way you would for a struct.

### Custom `Option`

`MaybeToken::{Some, None}` mirrors `Option<String>` so you can see that
`Option` is “just” an enum in the standard library.

## Exercises

1. Add an `AuthStatus::Locked { until: String }` variant and update every
   `match` (the compiler will guide you).
2. Implement `HttpStatus::from_code(u16) -> Option<HttpStatus>` for the codes
   used in this example.
3. Change `sum_packet_bytes` to also count bytes from a new
   `NetworkEvent::PacketSent(String, u32)` variant.
4. Add `MaybeToken::unwrap_or(self, default: String) -> String`.

## Further reading

- [The Rust Book — Enums](https://doc.rust-lang.org/book/ch06-00-enums.html)
- [The Rust Book — Pattern Matching](https://doc.rust-lang.org/book/ch18-00-patterns.html)
- [Rust by Example — Enums](https://doc.rust-lang.org/rust-by-example/custom_types/enum.html)
