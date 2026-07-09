//! Integration tests for `exercise_lifetimes`.

use lifetimes_exercises::{
    ImportantExcerpt, get_exercise_list, owned_summary, run_all, run_exercise, shortest,
};

#[test]
fn exercise_list_has_three_entries() {
    assert_eq!(get_exercise_list().len(), 3);
}

#[test]
fn run_each_exercise() {
    assert!(run_exercise("shortest", false).is_ok());
    assert!(run_exercise("excerpt", false).is_ok());
    assert!(run_exercise("summary", false).is_ok());
}

#[test]
fn run_all_succeeds() {
    assert!(run_all(false).is_ok());
}

#[test]
fn shortest_with_local_string() {
    let long = String::from("intrusion detected on segment 7");
    let short = "ok";
    assert_eq!(shortest(long.as_str(), short), "ok");
}

#[test]
fn excerpt_borrows_from_source() {
    let text = "WARN port_scan detected from 203.0.113.9";
    let excerpt = ImportantExcerpt::last_word(text);
    assert_eq!(excerpt.part, "203.0.113.9");
    assert!(excerpt.level().contains("len=11"));
}

#[test]
fn owned_summary_outlives_parts() {
    let a = String::from("CRITICAL");
    let b = String::from("disk");
    let parts = [a.as_str(), b.as_str()];
    let summary = owned_summary(&parts);
    assert_eq!(summary, "CRITICAL / disk");
}
