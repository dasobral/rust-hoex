//! Checksum exercise — hex formatting and word folding.

use crate::checksum::{checksum_hex, fold_checksum, internet_checksum, nibble_mix};

/// Run the checksum exercise with demo output.
pub fn run(verbose: bool) {
    println!("🔢 Checksum Helpers — Hex Format and Word Folding");
    println!();

    let words = [0x4500_u16, 0x003C, 0x1C46, 0x4000];
    let csum = internet_checksum(&words);
    println!("1. Pseudo-header words: {words:?}");
    println!("2. Internet checksum: {}", checksum_hex(csum));

    let folded = fold_checksum(0x0001_0002);
    println!("3. Folded carry demo: {}", checksum_hex(folded));

    let mixed = nibble_mix(0x0F, 0x0A);
    println!("4. Nibble mix 0x0F + 0x0A -> 0x{mixed:02X}");

    if verbose {
        println!();
        println!("   Why it matters:");
        println!("   - Checksums detect bit flips in transit");
        println!("   - Carry folding keeps sums in 16-bit range");
        println!("   - Functions compose: fold after sum, format for logs");
    }
}
