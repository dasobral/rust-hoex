//! Integration tests for `exercise_testing`.

use testing_exercises::{
    PolicyReport, SAMPLE_PASSWORDS, check_or_err, check_policy, get_exercise_list, has_digit,
    has_min_length, has_punctuation, has_uppercase, run_all, run_exercise,
};

#[test]
fn exercise_list_has_policy_entry() {
    let list = get_exercise_list();
    assert_eq!(list.len(), 1);
    assert_eq!(list[0].name, "policy");
}

#[test]
fn run_policy_exercise() {
    assert!(run_exercise("policy", false).is_ok());
}

#[test]
fn run_all_succeeds() {
    assert!(run_all(false).is_ok());
}

#[test]
fn each_rule_tested_via_public_api() {
    assert!(!has_min_length("Ab1!"));
    assert!(!has_digit("NoDigit!"));
    assert!(!has_uppercase("lower1!"));
    assert!(!has_punctuation("NoPunct1"));
}

#[test]
fn strong_password_passes_policy() {
    let report = check_policy("Vault!2026");
    assert_eq!(report, PolicyReport::pass());
}

#[test]
fn sample_passwords_include_expected_cases() {
    assert!(SAMPLE_PASSWORDS.contains(&"Vault!2026"));
    assert!(SAMPLE_PASSWORDS.contains(&""));
}

#[test]
fn check_or_err_reports_violations() {
    let result = check_or_err("weak");
    assert!(result.is_err());
    if let Err(msg) = result {
        assert!(msg.contains("too short"));
    }
}

#[test]
#[ignore = "slow brute-force style candidate scan"]
fn slow_integration_scan() {
    let candidates = ["password", "Vault!2026", "x", "NoPunct1"];
    let mut failures = 0_u32;
    for pw in candidates {
        if !check_policy(pw).ok {
            failures = failures.saturating_add(1);
        }
    }
    assert!(failures >= 2);
}
