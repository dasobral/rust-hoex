//! Integration tests for `exercise_collections`.

use collections_exercises::intrusion::{IntrusionLog, LogEvent, sample_log};
use collections_exercises::{get_exercise_list, run_all, run_exercise};

#[test]
fn exercise_list_has_three_entries() {
    let list = get_exercise_list();
    assert_eq!(list.len(), 3);
    let names: Vec<&str> = list.iter().map(|e| e.name).collect();
    assert!(names.contains(&"aggregator"));
    assert!(names.contains(&"analysis"));
    assert!(names.contains(&"investigation"));
}

#[test]
fn run_each_exercise() {
    assert!(run_exercise("aggregator", false).is_ok());
    assert!(run_exercise("analysis", false).is_ok());
    assert!(run_exercise("investigation", false).is_ok());
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
fn intrusion_log_end_to_end() {
    let log = sample_log();
    assert_eq!(log.len(), 5);
    assert_eq!(log.ip_counts().get("203.0.113.10"), Some(&3));
    assert_eq!(log.hot_ips(), log.ip_counts());
    assert_eq!(log.unique_users().len(), 3);
    assert_eq!(log.filter_by_action("login_fail").len(), 3);
    assert_eq!(log.top_user().as_deref(), Some("alice"));
}

#[test]
fn empty_log_metrics() {
    let log = IntrusionLog::new();
    assert!(log.is_empty());
    assert!(log.top_user().is_none());
    assert!(log.actions().is_empty());
}

#[test]
fn custom_log_push() {
    let mut log = IntrusionLog::new();
    log.push(LogEvent::new("10.0.0.1", "root", "sudo_fail"));
    assert_eq!(log.len(), 1);
    assert!(log.actions().contains("sudo_fail"));
}
