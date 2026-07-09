//! Integration tests for `example_collections`.

use example_collections::{IntrusionLog, LogEvent, merge_logs};

#[test]
fn end_to_end_aggregation() {
    let mut log = IntrusionLog::new();
    log.push(LogEvent::new("1.1.1.1", "u1", "login_fail"));
    log.push(LogEvent::new("1.1.1.1", "u2", "login_fail"));
    log.push(LogEvent::new("2.2.2.2", "u1", "login_ok"));

    assert_eq!(log.len(), 3);
    assert_eq!(log.ip_counts().get("1.1.1.1"), Some(&2));
    assert_eq!(log.unique_users().len(), 2);
    assert_eq!(log.filter_by_action("login_fail").len(), 2);
    assert_eq!(log.hot_ips(2), vec!["1.1.1.1".to_owned()]);
}

#[test]
fn merge_preserves_all_events() {
    let mut a = IntrusionLog::new();
    a.push(LogEvent::new("10.0.0.1", "a", "x"));
    let mut b = IntrusionLog::new();
    b.push(LogEvent::new("10.0.0.2", "b", "y"));
    let m = merge_logs(a, b);
    assert_eq!(m.len(), 2);
    assert!(m.summary().contains("2 events"));
}
