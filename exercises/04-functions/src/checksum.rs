//! Network checksum helpers — educational Internet checksum folding.

/// Maximum payload size accepted by the seal helpers (bytes).
pub const MAX_PAYLOAD: usize = 1500;

/// Format a checksum as uppercase hex, e.g. `0xABCD`.
#[must_use]
pub fn checksum_hex(csum: u16) -> String {
    format!("0x{csum:04X}")
}

/// Fold a 32-bit accumulator into a 16-bit one's-complement value.
///
/// This is the carry-fold step from RFC 1071. Repeatedly add the high 16 bits
/// back into the low 16 bits until no carry remains, then bitwise-NOT.
#[must_use]
pub const fn fold_checksum(mut sum: u32) -> u16 {
    while sum > 0xFFFF {
        sum = (sum & 0xFFFF) + (sum >> 16);
    }
    #[allow(clippy::cast_possible_truncation)]
    {
        !(sum as u16)
    }
}

/// Compute an Internet-style checksum over 16-bit words.
///
/// Educational simplification: sums each word into a `u32` accumulator with
/// wrapping addition, folds carries, then returns the one's complement.
/// Real implementations often operate on byte buffers with odd-byte padding.
#[must_use]
pub fn internet_checksum(words: &[u16]) -> u16 {
    let mut sum: u32 = 0;
    for &word in words {
        sum = sum.wrapping_add(u32::from(word));
    }
    fold_checksum(sum)
}

/// Combine high and low nibbles into one byte, masking each to four bits.
#[must_use]
pub const fn nibble_mix(hi: u8, lo: u8) -> u8 {
    (hi & 0x0F) << 4 | (lo & 0x0F)
}

/// Convert a byte slice to big-endian `u16` words, padding an odd final byte.
#[must_use]
pub fn bytes_to_words(data: &[u8]) -> Vec<u16> {
    let mut words = Vec::new();
    let mut chunks = data.chunks_exact(2);
    for pair in chunks.by_ref() {
        words.push(u16::from(pair[0]) << 8 | u16::from(pair[1]));
    }
    if let Some(&last) = chunks.remainder().first() {
        words.push(u16::from(last) << 8);
    }
    words
}

/// Seal a payload with a 4-byte header: `[len_hi, len_lo, csum_hi, csum_lo, ...payload]`.
///
/// Returns `None` when the payload exceeds [`MAX_PAYLOAD`] or length does not fit `u16`.
#[must_use]
pub fn seal_packet(payload: &[u8]) -> Option<(Vec<u8>, u16)> {
    if payload.len() > MAX_PAYLOAD {
        return None;
    }
    let len_u16 = u16::try_from(payload.len()).ok()?;

    let mut buf = Vec::with_capacity(4 + payload.len());
    #[allow(clippy::cast_possible_truncation)]
    {
        buf.push((len_u16 >> 8) as u8);
        buf.push((len_u16 & 0xFF) as u8);
    }
    buf.push(0);
    buf.push(0);
    buf.extend_from_slice(payload);

    let words = bytes_to_words(&buf);
    let csum = internet_checksum(&words);
    #[allow(clippy::cast_possible_truncation)]
    {
        buf[2] = (csum >> 8) as u8;
        buf[3] = (csum & 0xFF) as u8;
    }
    Some((buf, csum))
}

/// Verify a sealed packet: checksum over all bytes (including embedded checksum) is zero.
#[must_use]
pub fn verify_sealed(packet: &[u8]) -> bool {
    if packet.len() < 4 {
        return false;
    }
    let words = bytes_to_words(packet);
    let folded = internet_checksum(&words);
    folded == 0
}

/// Read the payload length prefix from a sealed packet.
#[must_use]
pub fn payload_len(packet: &[u8]) -> Option<u16> {
    if packet.len() < 4 {
        return None;
    }
    Some(u16::from(packet[0]) << 8 | u16::from(packet[1]))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn checksum_hex_uppercase() {
        assert_eq!(checksum_hex(0xabcd), "0xABCD");
        assert_eq!(checksum_hex(0x0001), "0x0001");
    }

    #[test]
    fn internet_checksum_folds_carries() {
        let words = [0x0001_u16, 0x0002];
        let csum = internet_checksum(&words);
        assert_ne!(csum, 0);
    }

    #[test]
    fn nibble_mix_masks_inputs() {
        assert_eq!(nibble_mix(0x0A, 0x0B), 0xAB);
        assert_eq!(nibble_mix(0xFA, 0xCB), 0xAB);
    }

    #[test]
    fn seal_and_verify_round_trip() {
        let payload = b"ALERT";
        let sealed = seal_packet(payload);
        assert!(sealed.is_some());
        if let Some((packet, csum)) = sealed {
            assert_eq!(payload_len(&packet), Some(5));
            assert_eq!(checksum_hex(csum).len(), 6);
            assert!(verify_sealed(&packet));
        }
    }

    #[test]
    fn seal_rejects_oversized() {
        let big = vec![0_u8; MAX_PAYLOAD + 1];
        assert!(seal_packet(&big).is_none());
    }

    #[test]
    fn verify_rejects_short_buffer() {
        assert!(!verify_sealed(&[0x00, 0x01]));
    }
}
