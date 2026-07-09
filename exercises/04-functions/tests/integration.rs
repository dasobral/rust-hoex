//! Integration tests for the `exercise_functions` crate.

use functions_exercises::{
    checksum_hex, get_exercise_list, internet_checksum, nibble_mix, run_all, run_exercise,
    seal_packet, verify_sealed,
};

#[test]
fn test_exercise_list_contains_both() {
    let list = get_exercise_list();
    assert_eq!(list.len(), 2);

    let names: Vec<&str> = list.iter().map(|e| e.name).collect();
    assert!(names.contains(&"checksum"));
    assert!(names.contains(&"seal"));

    for exercise in &list {
        assert!(!exercise.description.is_empty());
        assert!(!exercise.concepts.is_empty());
    }
}

#[test]
fn test_run_each_exercise() {
    assert!(run_exercise("checksum", false).is_ok());
    assert!(run_exercise("seal", false).is_ok());
}

#[test]
fn test_run_unknown_exercise_errors() {
    let err = run_exercise("hash", false);
    assert!(err.is_err());
    if let Err(e) = err {
        let message = format!("{e}");
        assert!(message.contains("Unknown exercise"));
    }
}

#[test]
fn test_run_all() {
    assert!(run_all(false).is_ok());
}

#[test]
fn test_checksum_and_seal_api() {
    assert_eq!(checksum_hex(0x1234), "0x1234");
    assert_eq!(nibble_mix(0x01, 0x02), 0x12);

    let words = [0xFFFF_u16, 0x0001];
    let _ = internet_checksum(&words);

    let payload = b"test-payload";
    if let Some((packet, _)) = seal_packet(payload) {
        assert!(verify_sealed(&packet));
    }
}
