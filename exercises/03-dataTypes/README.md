# 03-dataTypes Exercises

Packet header **scalars and compound types** for cybersecurity inspection: TCP ports,
TOS bit fields, protocol labels, and fixed IPv4 header arrays.

## Overview

These exercises build on [`examples/03-dataTypes`](../../examples/03-dataTypes).
You work with the integer sizes and compound types that appear on the wire when
parsing network packets — without unsafe code or external parsing libraries.

## Learning Objectives

After completing these exercises, you should understand:

- [x] **`u8` / `u16` scalars** — port numbers, lengths, checksums
- [x] **Bit masks** — extract DSCP and ECN from a TOS byte
- [x] **`char` labels** — compact protocol identification
- [x] **Structs** — group related fields (`TcpPorts`)
- [x] **Tuples and arrays** — `(src, dst, label)` and `[u8; 20]` headers
- [x] **`Option`** — safe overflow checks (`header_words`)

## Project Layout

```text
exercises/03-dataTypes/
├── Cargo.toml
├── README.md
├── src/
│   ├── lib.rs           # Public API and orchestration
│   ├── main.rs          # clap CLI (list / all / subcommands)
│   ├── packet.rs        # Core header helpers
│   ├── ports.rs         # Port scalar exercise
│   └── tos_protocol.rs  # TOS & protocol exercise
└── tests/
    └── integration.rs
```

## Running the Exercises

From the workspace root:

```bash
cargo run -p exercise_datatypes
cargo run -p exercise_datatypes -- list
cargo run -p exercise_datatypes -- ports --verbose
cargo run -p exercise_datatypes -- tos-protocol
cargo run -p exercise_datatypes -- all
```

Quality checks:

```bash
cargo test -p exercise_datatypes
cargo clippy -p exercise_datatypes --all-targets --all-features -- -D warnings
cargo fmt -p exercise_datatypes
```

## Exercises

### 1. `ports` — TCP endpoint scalars

Demonstrates `u16` source/dest ports, well-known port checks, and reversing flow
direction with `swap_ports`.

**Types:** `TcpPorts { source_port, dest_port }`

### 2. `tos_protocol` — Bit masks and header compounds

Parses a fake IPv4 header array, splits TOS into `(dscp, ecn)`, maps protocol
numbers to labels, and prints endpoint tuples.

**Functions:** `parse_tos`, `protocol_label`, `header_words`, `parse_header`

## Public API

| Function / Type | Signature | Concept |
|-----------------|-----------|---------|
| `TcpPorts` | `{ source_port, dest_port }` | Compound endpoint |
| `swap_ports` | `(TcpPorts) -> TcpPorts` | Flow reversal |
| `protocol_label` | `(u8) -> char` | Protocol shorthand |
| `parse_tos` | `(u8) -> (u8, u8)` | Bit mask extraction |
| `header_words` | `(u8) -> Option<u16>` | Overflow-safe multiply |
| `parse_header` | `(&[u8; 20]) -> Option<PacketHeader>` | Fixed-array parse |

## Key Takeaways

1. **Pick the smallest type that fits** — ports are `u16`, TTL is `u8`.
2. **Mask before shift** — TOS sub-fields live in specific bit ranges.
3. **Compound types document intent** — a tuple of four `u8`s is an IPv4 address.
4. **Check arithmetic** — `header_words` returns `None` instead of wrapping.

## Related Material

- [`examples/03-dataTypes`](../../examples/03-dataTypes) — introductory walkthrough
- [`examples/04-functions`](../../examples/04-functions) — next topic: functions
