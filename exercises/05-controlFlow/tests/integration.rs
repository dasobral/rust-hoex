//! Integration tests for the `exercise_controlflow` crate.

use controlflow_exercises::{
    Severity, classify_batch, classify_score, get_exercise_list, run_all, run_exercise,
    score_log_line, severity_label, walk_nonempty_lines,
};

#[test]
fn test_exercise_list_contains_both() {
    let list = get_exercise_list();
    assert_eq!(list.len(), 2);

    let names: Vec<&str> = list.iter().map(|e| e.name).collect();
    assert!(names.contains(&"scoring"));
    assert!(names.contains(&"batch"));

    for exercise in &list {
        assert!(!exercise.description.is_empty());
        assert!(!exercise.concepts.is_empty());
    }
}

#[test]
fn test_run_each_exercise() {
    assert!(run_exercise("scoring", false).is_ok());
    assert!(run_exercise("batch", false).is_ok());
}

#[test]
fn test_run_unknown_exercise_errors() {
    let err = run_exercise("firewall", false);
    assert!(err.is_err());
    if let Err(e) = err {
        let message = format!("{e}");
        assert!(message.contains("Unknown exercise"));
    }
}

#[test]
fn test_run_all() {
    assert!(run_all(false).is_ok());
}

#[test]
fn test_classifier_api() {
    assert_eq!(classify_score(80), Severity::Critical);
    assert_eq!(severity_label(Severity::Medium), "MEDIUM");

    let score = score_log_line("error: connection fail");
    assert!(score >= 15);

    let lines = ["", "warning event", "critical malware"];
    let batch = classify_batch(&lines, 10);
    assert_eq!(batch.len(), 2);

    let nonempty = walk_nonempty_lines(&lines);
    assert_eq!(nonempty.len(), 2);
}
