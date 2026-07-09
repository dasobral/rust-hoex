//! example: 03-dataTypes
//!
//! Network packet header analyzer — scalar and compound types.
//!
//! Run from this directory:
//! ```bash
//! cargo run
//! ```
//!
//! Key concepts:
//! - Integer sizes: i8/u8, u16, u32, u64 (and when to pick each)
//! - Floats (`f64`), bool, and char
//! - Tuples and arrays as compound types
//! - Explicit casting with `as`
//! - Numeric literal forms: decimal, hex, binary, underscores

use example_datatypes::{
    HEADER_LEN, endpoint_summary, format_ipv4, is_private_src, parse_header, protocol_label,
    protocol_name, sample_header_bytes, ttl_budget_ratio,
};

fn main() {
    println!("\nNetwork Packet Header Analyzer");
    println!("================================\n");

    // === Arrays: fixed-size, stack-allocated, homogeneous ===
    // `[u8; 20]` is an array of exactly 20 bytes — ideal for wire formats.
    let raw: [u8; HEADER_LEN] = sample_header_bytes();
    println!("Raw header ({HEADER_LEN} bytes): {raw:02X?}");

    // === Parsing into typed scalar fields ===
    let Some(header) = parse_header(&raw) else {
        println!("Failed to parse header");
        return;
    };

    // Integers of different widths match the on-wire layout.
    let version: u8 = header.version; // 0..=255
    let total_length: u16 = header.total_length; // 0..=65535
    let identification: u16 = header.identification;
    let ttl: u8 = header.ttl;
    let protocol: u8 = header.protocol;

    // Wider counters for analytics (teaching u32 / u64).
    let packets_seen: u32 = 1;
    let bytes_on_wire: u64 = u64::from(total_length);

    // Floats for ratios / estimates (not for exact wire values).
    let budget: f64 = ttl_budget_ratio(ttl, 64);

    // Bool for yes/no security heuristics.
    let from_lan: bool = is_private_src(header.src);

    // Char for a compact protocol glyph (Unicode scalar, not a C byte).
    let label: char = protocol_label(protocol);

    println!("Parsed fields:");
    println!("  version          = {version} (u8)");
    println!("  IHL              = {} words", header.ihl);
    println!("  total_length     = {total_length} (u16)");
    println!("  identification   = 0x{identification:04X} (u16)");
    println!("  TTL              = {ttl} (u8)");
    println!(
        "  protocol         = {protocol} -> {} [{label}]",
        protocol_name(protocol)
    );
    println!("  checksum         = 0x{:04X}", header.checksum);
    println!("  src              = {}", format_ipv4(header.src));
    println!("  dst              = {}", format_ipv4(header.dst));
    println!("  packets_seen     = {packets_seen} (u32)");
    println!("  bytes_on_wire    = {bytes_on_wire} (u64)");
    println!("  ttl_budget_ratio = {budget:.2} (f64)");
    println!("  from_private_lan = {from_lan} (bool)");

    // === Tuples: heterogeneous, fixed-length compound values ===
    let (src, dst, proto_char) = endpoint_summary(&header);
    println!("\nEndpoint tuple (src, dst, label):");
    println!(
        "  ({}, {}, '{proto_char}')",
        format_ipv4(src),
        format_ipv4(dst)
    );

    // === Casting with `as` (explicit, truncating / converting) ===
    // Prefer `From`/`Into` when lossless; `as` when you intentionally narrow.
    let length_as_u32: u32 = u32::from(total_length); // lossless widen
    #[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
    let ttl_as_i8: i8 = ttl as i8; // may truncate if ttl > 127 — demo only
    #[allow(clippy::cast_possible_truncation)]
    let checksum_hi: u8 = (header.checksum >> 8) as u8;

    println!("\nCasting demos:");
    println!("  total_length as u32 = {length_as_u32}");
    println!("  ttl as i8           = {ttl_as_i8}");
    println!("  checksum high byte  = 0x{checksum_hi:02X}");

    // === Numeric literal styles ===
    let syn_flag_mask: u8 = 0b0000_0010;
    let well_known_https: u16 = 443;
    let epoch_demo: u64 = 1_700_000_000;
    println!("\nLiteral styles:");
    println!("  binary mask  = 0b{syn_flag_mask:08b}");
    println!("  decimal port = {well_known_https}");
    println!("  underscored  = {epoch_demo}");

    println!("\nDone. See README.md for exercises and Rust Book links.");
}
