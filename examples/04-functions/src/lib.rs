//! Shared helpers for the functions example.
//!
//! Modular network checksum utilities: Internet checksum (RFC 1071 style),
//! folding, validation, and simple packet-length helpers.

/// Maximum payload size we accept in this educational demo (bytes).
pub const MAX_PAYLOAD: usize = 1500;

/// Fold a 32-bit sum into a 16-bit one's-complement value.
///
/// Demonstrates: parameters, return type annotation, expression body (no `;`).
#[must_use]
pub const fn fold_checksum(mut sum: u32) -> u16 {
    // While there are carry bits above 16, add them back in (one's complement).
    while sum > 0xFFFF {
        sum = (sum & 0xFFFF) + (sum >> 16);
    }
    // Last expression is the return value — no `return` keyword needed.
    // After folding, `sum` fits in 16 bits; cast is intentional.
    #[allow(clippy::cast_possible_truncation)]
    {
        !(sum as u16)
    }
}

/// Sum 16-bit big-endian words from a byte slice into a running `u32`.
///
/// Odd final byte is padded with a zero low byte (RFC 1071).
#[must_use]
pub fn sum_words(data: &[u8]) -> u32 {
    let mut sum: u32 = 0;
    let mut chunks = data.chunks_exact(2);
    for pair in chunks.by_ref() {
        let word = u32::from(u16::from(pair[0]) << 8 | u16::from(pair[1]));
        sum = sum.wrapping_add(word);
    }
    if let Some(&last) = chunks.remainder().first() {
        sum = sum.wrapping_add(u32::from(u16::from(last) << 8));
    }
    sum
}

/// Compute the Internet checksum over `data`.
///
/// Teaching points:
/// - Call other functions (composition)
/// - Explicit return type
#[must_use]
pub fn internet_checksum(data: &[u8]) -> u16 {
    fold_checksum(sum_words(data))
}

/// Verify a buffer that **includes** its checksum field.
///
/// Classic property: after embedding the checksum, the one's-complement sum of
/// all words is `0xFFFF`, so [`internet_checksum`] returns `0`. Empty input is
/// treated as vacuously valid for the demo.
#[must_use]
pub fn checksum_valid(data: &[u8]) -> bool {
    // Statements end with `;` and do not yield a value to the caller.
    let sum = sum_words(data);
    let folded = fold_checksum(sum);
    // Expression without `;` is the function result.
    // Embedded checksum => folded value 0; empty => allow.
    folded == 0 || data.is_empty()
}

/// Build a tiny pseudo-header buffer: `[len_hi, len_lo, ...payload]` with a
/// placeholder checksum of `0x0000` at bytes 2..=3, then fill it in.
///
/// Returns `(buffer, checksum)` as a **tuple** — multiple return values.
#[must_use]
pub fn seal_packet(payload: &[u8]) -> (Vec<u8>, u16) {
    // Early return on oversized input (unit-like guard; returns empty + 0).
    if payload.len() > MAX_PAYLOAD {
        return (Vec::new(), 0);
    }

    let total_len = payload.len() + 4;
    let mut buf = Vec::with_capacity(total_len);
    let len_u16 = u16::try_from(payload.len()).unwrap_or(u16::MAX);
    #[allow(clippy::cast_possible_truncation)]
    {
        buf.push((len_u16 >> 8) as u8);
        buf.push((len_u16 & 0xFF) as u8);
    }
    buf.push(0); // checksum high placeholder
    buf.push(0); // checksum low placeholder
    buf.extend_from_slice(payload);

    let csum = internet_checksum(&buf);
    #[allow(clippy::cast_possible_truncation)]
    {
        buf[2] = (csum >> 8) as u8;
        buf[3] = (csum & 0xFF) as u8;
    }
    (buf, csum)
}

/// Extract payload length from a sealed packet; `None` if too short.
#[must_use]
pub fn payload_len(packet: &[u8]) -> Option<u16> {
    if packet.len() < 4 {
        // Early return with `return` keyword — useful in longer functions.
        return None;
    }
    Some(u16::from(packet[0]) << 8 | u16::from(packet[1]))
}

/// Compare two checksums for equality (side-effect free).
#[must_use]
pub const fn checksums_equal(a: u16, b: u16) -> bool {
    a == b
}

/// Describe checksum strength as a short static string (expression body).
#[must_use]
pub const fn checksum_strength_hint(csum: u16) -> &'static str {
    // `if` is an expression — both branches must produce the same type.
    if csum == 0 {
        "weak/empty"
    } else if csum == 0xFFFF {
        "all-ones"
    } else {
        "typical"
    }
}

/// Running nibble mix for teaching statement vs expression blocks.
#[must_use]
pub const fn nibble_mix(byte: u8) -> u8 {
    let high = byte >> 4;
    let low = byte & 0x0F;
    // Block expression: last line without `;` is the value of the block.
    {
        let mixed = high ^ low;
        mixed.wrapping_mul(3)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fold_checksum_known_pattern() {
        // Sum that already fits in 16 bits: fold should one's-complement it.
        let folded = fold_checksum(0x0001);
        assert_eq!(folded, !0x0001_u16);
    }

    #[test]
    fn sum_words_even_and_odd() {
        assert_eq!(sum_words(&[0x01, 0x02]), 0x0102);
        // Odd length: last byte 0x03 becomes 0x0300
        assert_eq!(sum_words(&[0x01, 0x02, 0x03]), 0x0102 + 0x0300);
    }

    #[test]
    fn internet_checksum_empty_is_all_ones() {
        // Empty sum folds to 0, one's complement is 0xFFFF.
        assert_eq!(internet_checksum(&[]), 0xFFFF);
    }

    #[test]
    fn seal_and_validate_round_trip() {
        let payload = b"PING";
        let (packet, csum) = seal_packet(payload);
        assert!(!packet.is_empty());
        assert_ne!(csum, 0);
        assert_eq!(payload_len(&packet), Some(4));
        // With checksum embedded, recomputing yields 0 (valid).
        assert_eq!(internet_checksum(&packet), 0);
        assert!(checksum_valid(&packet));
    }

    #[test]
    fn seal_rejects_oversized() {
        let big = vec![0_u8; MAX_PAYLOAD + 1];
        let (packet, csum) = seal_packet(&big);
        assert!(packet.is_empty());
        assert_eq!(csum, 0);
    }

    #[test]
    fn payload_len_too_short() {
        assert_eq!(payload_len(&[0x00, 0x01]), None);
    }

    #[test]
    fn checksums_equal_and_hints() {
        assert!(checksums_equal(0xABCD, 0xABCD));
        assert!(!checksums_equal(1, 2));
        assert_eq!(checksum_strength_hint(0), "weak/empty");
        assert_eq!(checksum_strength_hint(0xFFFF), "all-ones");
        assert_eq!(checksum_strength_hint(0x1234), "typical");
    }

    #[test]
    fn nibble_mix_values() {
        // 0xAB -> high=0xA low=0xB -> xor=1 -> *3 = 3
        assert_eq!(nibble_mix(0xAB), 3);
    }
}
