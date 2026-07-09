//! Integration tests for `example_modules`.
//!
//! These only see the **public** API re-exported from `lib.rs`. Internal
//! helpers like `analyzer::score::compute_score` stay private to the crate.

use example_modules::{RiskLevel, analyze};

#[test]
fn public_analyze_empty_secret() {
    let result = analyze("");
    assert_eq!(result.score, 0);
    assert_eq!(result.risk, RiskLevel::Critical);
    assert!(!result.mixed_classes);
    assert!(result.report().contains("critical"));
}

#[test]
fn public_analyze_strong_secret() {
    let result = analyze("Tr0ub4dor&3-extra-length");
    assert!(result.score >= 60);
    assert!(result.mixed_classes);
    assert!(result.length > 10);
    let report = result.report();
    assert!(report.contains("strength score"));
    assert!(report.contains(result.risk.as_str()));
}

#[test]
fn risk_level_labels_are_stable() {
    assert_eq!(RiskLevel::High.as_str(), "high");
    assert_eq!(RiskLevel::Low.as_str(), "low");
}
