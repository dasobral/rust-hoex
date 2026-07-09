# 04-functions Exercises

Network **checksum helpers** and **sealed packet** functions: hex formatting,
16-bit word folding, length-prefixed buffers, and integrity verification.

## Overview

These exercises build on [`examples/04-functions`](../../examples/04-functions).
You implement composable functions that mirror real packet integrity checks while
staying educational and safe (no `unwrap`, no `unsafe`).

## Learning Objectives

After completing these exercises, you should understand:

- [x] **Function signatures** — parameters, return types, expression bodies
- [x] **Composition** — `internet_checksum` built from `fold_checksum`
- [x] **Tuple returns** — `(Vec<u8>, u16)` from `seal_packet`
- [x] **`Option` guards** — `None` for oversize payloads
- [x] **Const functions** — `fold_checksum`, `nibble_mix`
- [x] **Verify-by-recompute** — embedded checksum yields zero sum

## Project Layout

```text
exercises/04-functions/
├── Cargo.toml
├── README.md
├── src/
│   ├── lib.rs                # Public API and orchestration
│   ├── main.rs               # clap CLI
│   ├── checksum.rs           # Core checksum helpers
│   ├── checksum_exercise.rs  # Checksum demo
│   └── seal_exercise.rs      # Seal/verify demo
└── tests/
    └── integration.rs
```

## Running the Exercises

```bash
cargo run -p exercise_functions
cargo run -p exercise_functions -- list
cargo run -p exercise_functions -- checksum --verbose
cargo run -p exercise_functions -- seal
cargo run -p exercise_functions -- all
```

Quality checks:

```bash
cargo test -p exercise_functions
cargo clippy -p exercise_functions --all-targets --all-features -- -D warnings
cargo fmt -p exercise_functions
```

## Exercises

### 1. `checksum` — Hex format and word folding

Formats checksums as `0xABCD`, sums `u16` words with carry folding, and mixes
nibbles for header field assembly.

**Functions:** `checksum_hex`, `internet_checksum`, `fold_checksum`, `nibble_mix`

### 2. `seal` — Length-prefixed sealed packets

Builds `[len_hi, len_lo, csum_hi, csum_lo, ...payload]`, embeds a checksum,
and verifies integrity with `verify_sealed`.

**Functions:** `seal_packet`, `verify_sealed`, `payload_len`

## Public API

| Function | Signature | Concept |
|----------|-----------|---------|
| `checksum_hex` | `(u16) -> String` | Uppercase hex display |
| `internet_checksum` | `(&[u16]) -> u16` | Word-sum + fold |
| `nibble_mix` | `(u8, u8) -> u8` | Safe nibble combine |
| `seal_packet` | `(&[u8]) -> Option<(Vec<u8>, u16)>` | Length prefix + checksum |
| `verify_sealed` | `(&[u8]) -> bool` | Integrity check |

## Key Takeaways

1. **Small functions compose** — fold is reusable after any accumulator.
2. **Return types document failure** — `Option` beats sentinel values.
3. **Pad odd bytes** — wire formats align to 16-bit word boundaries.
4. **Document simplifications** — educational checksums differ from kernel offload.

## Related Material

- [`examples/04-functions`](../../examples/04-functions) — introductory walkthrough
- [`examples/05-controlFlow`](../../examples/05-controlFlow) — next topic: control flow
