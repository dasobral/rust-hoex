//! Integration tests for `example_borrowing`.

use example_borrowing::{
    average_scores, count_ascii_letters, mask_password, meets_min_length, password_prefix,
    password_summary, update_strength_score,
};

#[test]
fn multiple_immutable_borrows_of_same_password() {
    let pw = String::from("Secure1!");
    let a = count_ascii_letters(&pw);
    let b = meets_min_length(&pw, 8);
    let (len, letters, ok) = password_summary(&pw);
    assert_eq!(a, 6);
    assert!(b);
    assert_eq!(len, 8);
    assert_eq!(letters, 6);
    assert!(ok);
    assert_eq!(pw, "Secure1!"); // still owned
}

#[test]
fn mutable_borrow_updates_score_exclusively() {
    let mut score = 0;
    update_strength_score(&mut score, "Aa1!");
    // length < 8 → no +10; has upper, digit, symbol → 5+5+10
    assert_eq!(score, 20);
}

#[test]
fn slices_str_and_array() {
    assert_eq!(password_prefix("abcdef", 3), Some("abc"));
    assert_eq!(average_scores(&[5, 15]), Some(10));
}

#[test]
fn mask_keeps_suffix() {
    assert_eq!(mask_password("password", 4), "****word");
}
