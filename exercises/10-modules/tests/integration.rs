//! Integration tests for `exercise_modules`.
//!
//! Uses the **public API only** — no direct access to `pub(crate)` helpers.

use modules_exercises::{RiskLevel, analyze, get_exercise_list, risk_label, run_all, run_exercise};

#[test]
fn exercise_list_has_two_entries() {
    let list = get_exercise_list();
    assert_eq!(list.len(), 2);
    let names: Vec<&str> = list.iter().map(|e| e.name).collect();
    assert!(names.contains(&"score"));
    assert!(names.contains(&"validate"));
}

#[test]
fn run_each_exercise() {
    assert!(run_exercise("score", false).is_ok());
    assert!(run_exercise("validate", false).is_ok());
}

#[test]
fn run_unknown_exercise_errors() {
    let err = run_exercise("firewall", false);
    assert!(err.is_err());
    if let Err(e) = err {
        assert!(format!("{e}").contains("Unknown exercise"));
    }
}

#[test]
fn run_all_succeeds() {
    assert!(run_all(false).is_ok());
}

#[test]
fn analyze_common_password_is_critical() {
    let result = analyze("PASSWORD");
    assert!(result.too_common);
    assert!(result.score <= 10);
    assert_eq!(result.risk, RiskLevel::Critical);
    assert_eq!(risk_label(&result), "critical");
}

#[test]
fn analyze_strong_password_scores_higher() {
    let weak = analyze("aaaa");
    let strong = analyze("VaultKey-2026!");
    assert!(strong.score > weak.score);
    assert!(!strong.too_common);
}

#[test]
fn risk_label_tracks_risk_level() {
    let low = analyze("Z9x!Correct-Horse-Battery");
    assert_eq!(risk_label(&low), low.risk.as_str());
}
