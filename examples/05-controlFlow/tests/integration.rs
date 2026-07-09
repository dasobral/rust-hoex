//! Integration tests for `example_controlflow`.

use example_controlflow::{
    AccessDecision, Severity, access_decision, accumulate_until_budget, apply_analyst_pin,
    classify_batch, decision_label, first_critical_index, score_log_line, severity_from_score,
    severity_label,
};

#[test]
fn end_to_end_line_pipeline() {
    let line = "CRITICAL exploit on host";
    let score = score_log_line(line);
    let level = severity_from_score(score);
    assert!(matches!(level, Severity::High | Severity::Critical));
    assert_eq!(access_decision(Severity::Critical), AccessDecision::Deny);
    assert_eq!(decision_label(AccessDecision::Deny), "DENY");
}

#[test]
fn batch_and_budget() {
    let lines = ["info ok", "warning x", "CRITICAL malware"];
    let (n, weight, _) = classify_batch(&lines, 10);
    assert_eq!(n, 3);
    // info=1, warning(~10)=>Info weight 1, critical+malware(~75)=>High weight 8
    assert!(weight >= 1 + 1 + 8);

    let scores = [score_log_line(lines[0]), score_log_line(lines[2])];
    assert!(accumulate_until_budget(&scores, 100) >= 1);
}

#[test]
fn optional_pin_and_critical_search() {
    assert_eq!(
        apply_analyst_pin(AccessDecision::Allow, Some("REVIEW")),
        AccessDecision::Challenge
    );
    assert_eq!(first_critical_index(&[5, 95, 10]), Some(1));
    assert_eq!(severity_label(Severity::Low), "LOW");
}
