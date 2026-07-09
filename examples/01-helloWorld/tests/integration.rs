//! Integration tests for `example_helloworld`.
//!
//! These tests link against the library crate (not the binary), which is why
//! we put `greet` in `lib.rs`.

use example_helloworld::greet;

#[test]
fn greet_returns_expected_message() {
    assert_eq!(greet("World"), "Hello, World!");
}

#[test]
fn greet_preserves_unicode_names() {
    assert_eq!(greet("世界"), "Hello, 世界!");
}

#[test]
fn greet_is_not_empty_for_typical_input() {
    let message = greet("Rust");
    assert!(!message.is_empty());
    assert!(message.contains("Rust"));
}
