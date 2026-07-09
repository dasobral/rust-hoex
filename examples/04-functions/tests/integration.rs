//! Integration tests for `example_functions`.

use example_functions::{
    MAX_PAYLOAD, checksum_strength_hint, checksum_valid, fold_checksum, internet_checksum,
    nibble_mix, payload_len, seal_packet, sum_words,
};

#[test]
fn seal_packet_embeds_length_and_checksum() {
    let (packet, csum) = seal_packet(b"ABC");
    assert_eq!(packet.len(), 7); // 4 header + 3 payload
    assert_eq!(payload_len(&packet), Some(3));
    assert_eq!(internet_checksum(&packet), 0);
    assert_ne!(csum, 0);
    assert!(checksum_valid(&packet));
}

#[test]
fn empty_payload_still_seals() {
    let (packet, _) = seal_packet(b"");
    assert_eq!(packet.len(), 4);
    assert_eq!(payload_len(&packet), Some(0));
    assert_eq!(internet_checksum(&packet), 0);
}

#[test]
fn oversized_payload_early_returns() {
    let oversized = vec![1_u8; MAX_PAYLOAD + 10];
    let (packet, csum) = seal_packet(&oversized);
    assert!(packet.is_empty());
    assert_eq!(csum, 0);
}

#[test]
fn sum_and_fold_pipeline() {
    let data = [0xFF, 0xFF, 0x00, 0x01];
    let sum = sum_words(&data);
    let folded = fold_checksum(sum);
    assert_eq!(folded, internet_checksum(&data));
}

#[test]
fn hints_and_nibble_mix_smoke() {
    assert_eq!(checksum_strength_hint(0), "weak/empty");
    assert_eq!(nibble_mix(0x11), 0); // 1^1=0
}
