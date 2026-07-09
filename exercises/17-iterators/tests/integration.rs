//! Integration tests for `exercise_iterators`.

use iterators_exercises::{
    LogLine, allowed_bytes, count_allows, denied_ips, get_exercise_list, inspect_action_counts,
    run_all, run_exercise, sample_logs,
};

#[test]
fn exercise_list_has_three_entries() {
    let list = get_exercise_list();
    assert_eq!(list.len(), 3);
    let names: Vec<&str> = list.iter().map(|e| e.name).collect();
    assert!(names.contains(&"adapters"));
    assert!(names.contains(&"filters"));
    assert!(names.contains(&"pipeline"));
}

#[test]
fn run_each_exercise() {
    assert!(run_exercise("adapters", false).is_ok());
    assert!(run_exercise("filters", false).is_ok());
    assert!(run_exercise("pipeline", false).is_ok());
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
fn log_line_processing_end_to_end() {
    let logs = sample_logs();
    assert_eq!(allowed_bytes(&logs), 896);
    assert_eq!(count_allows(&logs), 3);
    assert_eq!(denied_ips(&logs).len(), 2);
    assert_eq!(inspect_action_counts(&logs), (3, 2));
}

#[test]
fn custom_log_line() {
    let line = LogLine::new("ALLOW", "192.0.2.1", 64);
    assert!(line.is_allow());
    assert_eq!(allowed_bytes(&[line]), 64);
}
