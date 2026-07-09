//! example: 04-functions
//!
//! Network checksum utilities — function syntax in Rust.
//!
//! Run from this directory:
//! ```bash
//! cargo run
//! ```
//!
//! Key concepts:
//! - `fn` syntax, parameters, and return types
//! - Expressions vs statements (`;` matters)
//! - Implicit return (last expression) vs `return`
//! - Multiple return values via tuples
//! - Early return and the unit type `()`

use example_functions::{
    checksum_strength_hint, checksum_valid, checksums_equal, fold_checksum, internet_checksum,
    nibble_mix, payload_len, seal_packet, sum_words,
};

fn main() {
    println!("\nNetwork Checksum Utilities");
    println!("===========================\n");

    // === Calling functions with arguments ===
    let sample = [0x45_u8, 0x00, 0x00, 0x3C, 0x1A, 0x2B];
    let word_sum = sum_words(&sample);
    let csum = internet_checksum(&sample);

    println!("Sample bytes: {sample:02X?}");
    println!("sum_words(...)        = 0x{word_sum:08X}");
    println!("internet_checksum(...) = 0x{csum:04X}");
    println!("strength hint         = {}", checksum_strength_hint(csum));

    // === Expressions vs statements ===
    // A statement performs an action and ends with `;`.
    let folded = fold_checksum(word_sum);
    // An expression produces a value — function calls are expressions.
    println!("fold_checksum(sum)    = 0x{folded:04X}");

    // === Multiple return values (tuple) ===
    let payload = b"HELLO-NET";
    let (packet, sealed_csum) = seal_packet(payload);
    println!(
        "\nSealed packet ({} bytes), checksum 0x{sealed_csum:04X}",
        packet.len()
    );
    println!("Packet: {packet:02X?}");

    // === Early-return style helpers ===
    match payload_len(&packet) {
        Some(len) => println!("Declared payload length: {len}"),
        None => println!("Packet too short to read length"),
    }

    let valid = checksum_valid(&packet);
    println!("checksum_valid(packet) = {valid}");

    // Recompute and compare — sealed buffers should checksum to 0.
    let recomputed = internet_checksum(&packet);
    let same = checksums_equal(recomputed, 0);
    println!("recomputed == 0       = {same}");

    // === Unit type `()` ===
    // Functions that only cause side effects return `()`.
    // `println!` expands to something that evaluates to `()`.
    greet_operator(); // return type is `()` — the empty tuple / unit type

    // === Block expressions inside helpers ===
    let mixed = nibble_mix(0xF0);
    println!("nibble_mix(0xF0)      = {mixed}");

    println!("\nDone. See README.md for exercises and Rust Book links.");
}

/// Side-effect-only helper: returns the unit type `()`.
fn greet_operator() {
    println!("\n[ops] checksum demo complete");
    // No return expression — equivalent to returning `()`.
}
