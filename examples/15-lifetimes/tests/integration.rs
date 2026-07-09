//! Integration tests for `example_lifetimes` (public API only).

use example_lifetimes::{ImportantExcerpt, find_in_haystack, longer_excerpt, longest};

#[test]
fn public_longest_across_owned_and_literal() {
    let owned = String::from("abcdefgh");
    assert_eq!(longest(owned.as_str(), "xyz"), owned.as_str());
}

#[test]
fn public_find_in_haystack() {
    let line = "token=abc123 expires=never";
    assert_eq!(find_in_haystack(line, "abc123"), Some("abc123"));
    assert_eq!(find_in_haystack(line, "missing"), None);
}

#[test]
fn public_excerpt_round_trip() {
    let text = "WARN buffer overflow near edge";
    let a = ImportantExcerpt::from_first_word(text);
    let b = ImportantExcerpt { part: "overflow" };
    let pick = longer_excerpt(a, b);
    assert_eq!(pick.part, "overflow");
}
