//! Seal exercise — length-prefixed packets with embedded checksums.

use crate::checksum::{checksum_hex, payload_len, seal_packet, verify_sealed};

/// Run the seal/verify exercise with demo output.
pub fn run(verbose: bool) {
    println!("📨 Sealed Packets — Length Prefix and Verification");
    println!();

    let payload = b"IDS_EVENT: port_scan";
    if let Some((packet, csum)) = seal_packet(payload) {
        println!("1. Sealed {}-byte payload", payload.len());
        println!("2. Checksum field: {}", checksum_hex(csum));
        if let Some(len) = payload_len(&packet) {
            println!("3. Length prefix: {len} bytes");
        }
        println!(
            "4. verify_sealed: {}",
            if verify_sealed(&packet) {
                "VALID"
            } else {
                "INVALID"
            }
        );
    }

    let tampered = b"corrupted";
    if let Some((mut bad, _)) = seal_packet(tampered) {
        if bad.len() > 5 {
            bad[5] ^= 0xFF;
        }
        println!(
            "5. Tampered packet verify_sealed: {}",
            if verify_sealed(&bad) {
                "VALID"
            } else {
                "INVALID"
            }
        );
    }

    if verbose {
        println!();
        println!("   Why it matters:");
        println!("   - Length prefixes frame variable payloads on the wire");
        println!("   - Embedded checksums enable integrity checks without side channels");
        println!("   - `Option` signals oversize payloads instead of panicking");
    }
}
