//! Integration tests for `example_concurrency`.

#![allow(clippy::unwrap_used)]

use example_concurrency::{classify_level, process_logs};

#[test]
fn end_to_end_pipeline() {
    let lines = vec!["INFO a".into(), "ERROR b".into(), "WARN c".into()];
    let hits = process_logs(lines, 2).unwrap();
    assert_eq!(hits.len(), 3);
    assert_eq!(classify_level("ERROR x"), "ERROR");
}
