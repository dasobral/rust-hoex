//! Integration tests for `example_ownership`.

use example_ownership::{
    clone_password, consume_password, double_score, owned_password, password_byte_len,
    take_ownership,
};

#[test]
fn move_invalidates_original_binding_conceptually() {
    let pw = owned_password("move-me");
    let taken = take_ownership(pw);
    assert_eq!(taken, "move-me");
    // Compiling `assert_eq!(pw, ...)` here would fail — ownership moved.
}

#[test]
fn clone_keeps_both_owners() {
    let a = owned_password("twin");
    let b = clone_password(&a);
    assert_eq!(a, "twin");
    assert_eq!(b, "twin");
}

#[test]
fn consume_returns_len_and_drops() {
    assert_eq!(consume_password(owned_password("abc123")), 6);
}

#[test]
fn copy_types_remain_usable() {
    let n = 10;
    assert_eq!(double_score(n), 20);
    assert_eq!(n, 10);
}

#[test]
fn byte_len_works_for_str_and_string() {
    assert_eq!(password_byte_len("hi"), 2);
    let s = owned_password("hi");
    assert_eq!(password_byte_len(&s), 2);
}
