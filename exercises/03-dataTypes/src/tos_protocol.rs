//! TOS and protocol exercise — bit masks, tuples, and header arrays.

use crate::packet::{
    HEADER_LEN, endpoint_summary, format_ipv4, header_words, parse_header, parse_tos,
    protocol_label, sample_header_bytes,
};

/// Run the TOS/protocol exercise with IPv4 header tuple/array demos.
pub fn run(verbose: bool) {
    println!("📦 TOS & Protocol — Bit Masks and Header Compounds");
    println!();

    let bytes: [u8; HEADER_LEN] = sample_header_bytes();
    println!("1. Raw header array (first 4 bytes): {:02X?}", &bytes[..4]);

    if let Some(header) = parse_header(&bytes) {
        let (dscp, ecn) = parse_tos(header.tos);
        println!(
            "2. Parsed TOS 0x{:02X} -> dscp={dscp}, ecn={ecn}",
            header.tos
        );

        let label = protocol_label(header.protocol);
        println!("3. Protocol {} -> label '{label}'", header.protocol);

        if let Some(len) = header_words(header.ihl) {
            println!("4. IHL {} -> header length {len} bytes", header.ihl);
        }

        let (src, dst, proto_char) = endpoint_summary(&header);
        println!(
            "5. Endpoint tuple: {} -> {} [{proto_char}]",
            format_ipv4(src),
            format_ipv4(dst)
        );
    }

    if verbose {
        println!();
        println!("   Why it matters:");
        println!("   - Wire formats mix scalars (`u8`, `u16`) and fixed arrays");
        println!("   - Bit masks extract sub-fields without parsing libraries");
        println!("   - Tuples return multiple values without heap allocation");
    }
}
