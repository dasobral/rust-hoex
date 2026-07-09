# 04-functions

Function syntax through **network checksum utilities** (Internet checksum style).

## Overview

This example extracts reusable helpers — word summing, folding, sealing a tiny
packet, validation — so `main` only orchestrates demos. Each helper is covered
by unit tests; integration tests exercise the public library API.

## Learning Objectives

After completing this example, you should understand:

- [x] Declaring functions with `fn`, parameters, and return types
- [x] Statements (end with `;`) vs expressions (produce values)
- [x] Implicit return via the last expression vs explicit `return`
- [x] Returning multiple values with tuples
- [x] Early `return` for guard clauses
- [x] The unit type `()` for side-effect-only functions

## How to run

```bash
cd examples/04-functions
cargo run
cargo test
cargo clippy --all-targets -- -D warnings
cargo fmt
```

From the workspace root:

```bash
cargo run -p example_functions
cargo test -p example_functions
cargo clippy -p example_functions --all-targets -- -D warnings
```

## Key Concepts

### Function anatomy

```rust
fn fold_checksum(mut sum: u32) -> u16 {
    while sum > 0xFFFF {
        sum = (sum & 0xFFFF) + (sum >> 16);
    }
    !(sum as u16)  // last expression = return value
}
```

### Statements vs expressions

```rust
let sum = sum_words(data);   // statement binds a value
folded == 0xFFFF             // expression: function result when no `;`
```

### Multiple returns and early return

```rust
fn seal_packet(payload: &[u8]) -> (Vec<u8>, u16) {
    if payload.len() > MAX_PAYLOAD {
        return (Vec::new(), 0); // early return
    }
    // ...
    (buf, csum)
}
```

### Unit type

```rust
fn greet_operator() {  // returns ()
    println!("done");
}
```

## Exercises

1. Add `fn checksum_hex(csum: u16) -> String` that returns `"0xABCD"` style text; test it.
2. Change `seal_packet` to return `Option<(Vec<u8>, u16)>` instead of an empty vec on overflow.
3. Write `fn verify_sealed(packet: &[u8]) -> bool` that checks length field + checksum together.
4. Refactor `nibble_mix` to take two `u8` arguments and return their mixed value.

## Further Reading

- [The Rust Book — Functions](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html)
- [The Rust Book — Statements and Expressions](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html#statements-and-expressions)
- [Rust by Example — Functions](https://doc.rust-lang.org/rust-by-example/fn.html)
- [RFC 1071 — Computing the Internet Checksum](https://datatracker.ietf.org/doc/html/rfc1071)

## Related Examples

- `03-dataTypes`: types used in packet buffers
- `05-controlFlow`: classify results with `match` / loops
- `02-variables`: bindings and mutability inside function bodies
