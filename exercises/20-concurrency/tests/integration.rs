//! Integration tests for `exercise_concurrency`.

#![allow(clippy::unwrap_used)]

use concurrency_exercises::{collect_with_limit, count_by_level, process_logs};

#[test]
fn end_to_end_error_filter() {
    let lines = vec![
        "INFO boot".into(),
        "ERROR auth".into(),
        "WARN slow".into(),
        "ERROR disk".into(),
    ];
    let hits = process_logs(lines, "ERROR", 2).unwrap();
    assert_eq!(hits.len(), 2);
    assert_eq!(count_by_level(&hits), vec![("ERROR".into(), 2)]);
}

#[test]
fn early_disconnect_limits_collection() {
    let lines = vec!["ERROR a".into(), "ERROR b".into(), "ERROR c".into()];
    let hits = collect_with_limit(lines, "ERROR", 2, 1);
    assert_eq!(hits.len(), 1);
}
