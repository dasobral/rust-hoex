# 03-dataTypes

Scalar and compound types through a **network packet header analyzer**.

## Overview

This example parses a simplified IPv4-like header from a fixed `[u8; 20]` array
and maps each wire field onto an appropriate Rust type: small integers for
octets and TTL, `u16` for length/ID/checksum, arrays for addresses, tuples for
multi-value returns, `bool` for heuristics, `char` for protocol labels, and
`f64` for ratio estimates.

## Learning Objectives

After completing this example, you should understand:

- [x] Integer types (`i8`/`u8`, `u16`, `u32`, `u64`) and choosing widths
- [x] Floating-point (`f64`) vs integers for exact wire data
- [x] `bool` and `char`
- [x] Arrays (`[T; N]`) and tuples `(A, B, C)`
- [x] Explicit casting with `as` vs lossless `From`/`Into`
- [x] Numeric literals: decimal, hex (`0x`), binary (`0b`), underscores

## How to run

```bash
cd examples/03-dataTypes
cargo run
cargo test
cargo clippy --all-targets -- -D warnings
cargo fmt
```

From the workspace root:

```bash
cargo run -p example_datatypes
cargo test -p example_datatypes
cargo clippy -p example_datatypes --all-targets -- -D warnings
```

## Key Concepts

### Scalar integers match the wire

```rust
let version: u8 = header.version;       // nibble-sized field
let total_length: u16 = header.total_length;
let bytes_on_wire: u64 = u64::from(total_length);
```

### Arrays for fixed buffers

```rust
let raw: [u8; 20] = sample_header_bytes();
let src: [u8; 4] = [192, 168, 1, 10];
```

### Tuples for multiple return values

```rust
let (src, dst, label) = endpoint_summary(&header);
```

### Casting

```rust
let hi: u8 = (checksum >> 8) as u8;     // intentional narrow
let wide: u32 = u32::from(total_length); // preferred lossless widen
```

## Exercises

1. Add a `u16` source/destination port pair to a fake TCP pseudo-header and print them.
2. Change the sample protocol to UDP (`17`) and update tests for the new label `'U'`.
3. Implement `fn header_words(ihl: u8) -> u16` that returns `ihl * 4` without overflowing.
4. Parse TOS into a tuple `(dscp: u8, ecn: u8)` using bit masks.

## Further Reading

- [The Rust Book — Data Types](https://doc.rust-lang.org/book/ch03-02-data-types.html)
- [Rust by Example — Primitives](https://doc.rust-lang.org/rust-by-example/primitives.html)
- [Rust by Example — Tuples](https://doc.rust-lang.org/rust-by-example/primitives/tuples.html)
- [Rust by Example — Arrays](https://doc.rust-lang.org/rust-by-example/primitives/array.html)

## Related Examples

- `02-variables`: mutability, shadowing, constants
- `04-functions`: modular helpers built on these types
- `05-controlFlow`: classify packets with `if` / `match` / loops
