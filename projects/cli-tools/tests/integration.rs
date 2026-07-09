//! Integration tests for `project_cli_tools` (public library API only).

use project_cli_tools::{Strength, analyze_password, estimate_entropy};

#[test]
fn entropy_positive_for_mixed_password() {
    let est = estimate_entropy("Aa1!");
    assert!(est.bits > 0.0);
    assert_eq!(est.length, 4);
    assert_eq!(est.classes.len(), 4);
}

#[test]
fn analyze_rejects_common_password() {
    let report = analyze_password("password1");
    assert_eq!(report.strength, Strength::VeryWeak);
    assert!(
        report
            .findings
            .iter()
            .any(|f| f.contains("denylist") || f.contains("common"))
    );
}

#[test]
fn analyze_rates_diverse_long_password_highly() {
    let report = analyze_password("Xk9$mQ2!pL7#vN4@");
    assert!(report.entropy.bits >= 80.0);
    assert!(report.strength >= Strength::Strong);
    assert!(report.findings.is_empty());
}
