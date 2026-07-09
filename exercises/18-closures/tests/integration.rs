//! Integration tests for `exercise_closures`.

use std::collections::HashSet;

use closures_exercises::{
    ThreatEvent, count_matching, count_watchlisted, get_exercise_list, partition_critical, run_all,
    run_exercise, sample_events, sort_by_source,
};

#[test]
fn exercise_list_has_three_entries() {
    let list = get_exercise_list();
    assert_eq!(list.len(), 3);
    let names: Vec<&str> = list.iter().map(|e| e.name).collect();
    assert!(names.contains(&"sorting"));
    assert!(names.contains(&"partition"));
    assert!(names.contains(&"watchlist"));
}

#[test]
fn run_each_exercise() {
    assert!(run_exercise("sorting", false).is_ok());
    assert!(run_exercise("partition", false).is_ok());
    assert!(run_exercise("watchlist", false).is_ok());
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
fn threat_event_end_to_end() {
    let mut events = sample_events();
    sort_by_source(&mut events);
    assert_eq!(events[0].source, "10.0.0.5");

    let events = sample_events();
    let (critical, normal) = partition_critical(events);
    assert_eq!(critical.len() + normal.len(), 5);

    let events = sample_events();
    assert_eq!(count_matching(&events, |e| e.critical), 2);

    let mut watchlist = HashSet::new();
    watchlist.insert("203.0.113.10".to_owned());
    assert_eq!(count_watchlisted(&events, &watchlist), 2);
}

#[test]
fn custom_threat_event() {
    let event = ThreatEvent::new("192.0.2.1", 99, true);
    assert!(event.critical);
    assert_eq!(count_matching(&[event], |e| e.score > 50), 1);
}
