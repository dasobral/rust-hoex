//! Integration tests for `example_testing`.
//!
//! These compile as a **separate crate** that depends on the library's public
//! API only — they cannot see private helpers or `#[cfg(test)]` modules.

use example_testing::{MIN_LENGTH, check_or_err, check_policy, estimate_entropy_bits};

#[test]
fn public_min_length_constant() {
    assert_eq!(MIN_LENGTH, 8);
}

#[test]
fn public_check_policy_edge_exactly_min_length() {
    // Exactly 8 chars, has upper + digit, not on denylist.
    let report = check_policy("Abcdefg1");
    assert!(report.ok, "violations: {:?}", report.violations);
}

#[test]
fn public_check_or_err_result() -> Result<(), String> {
    check_or_err("GoodPass1")?;
    Ok(())
}

#[test]
fn public_entropy_positive_for_mixed() {
    let bits = estimate_entropy_bits("Aa1!");
    assert!(bits > 0.0);
}
