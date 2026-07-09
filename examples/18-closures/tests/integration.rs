//! Integration tests for `example_closures`.

use example_closures::{Severity, ThreatEvent, filter_by_severity, sort_threats};

#[test]
fn public_sort_and_filter() {
    let mut events = vec![
        ThreatEvent::new("a", "1.1.1.1", Severity::Medium, 1),
        ThreatEvent::new("b", "2.2.2.2", Severity::Critical, 1),
    ];
    sort_threats(&mut events);
    assert_eq!(events[0].kind, "b");
    let high = filter_by_severity(&events, Severity::High);
    assert_eq!(high.len(), 1);
}
