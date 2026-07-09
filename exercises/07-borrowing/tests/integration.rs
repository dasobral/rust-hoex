//! Integration tests for the `exercise_borrowing` crate.

use borrowing_exercises::{
    average_scores, count_digits, first_char, get_exercise_list, mask_keep_last, meets_policy,
    run_all, run_exercise, update_strength,
};

#[test]
fn test_exercise_list_contains_both() {
    let list = get_exercise_list();
    assert_eq!(list.len(), 2);

    let names: Vec<&str> = list.iter().map(|e| e.name).collect();
    assert!(names.contains(&"inspect"));
    assert!(names.contains(&"policy"));

    for exercise in &list {
        assert!(!exercise.description.is_empty());
        assert!(!exercise.concepts.is_empty());
    }
}

#[test]
fn test_run_each_exercise() {
    assert!(run_exercise("inspect", false).is_ok());
    assert!(run_exercise("policy", false).is_ok());
}

#[test]
fn test_run_unknown_exercise_errors() {
    let err = run_exercise("audit", false);
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
fn test_first_char_and_digits() {
    assert_eq!(first_char("Xyl0phone!"), Some('X'));
    assert_eq!(count_digits("Xyl0phone!"), 1);
}

#[test]
fn test_meets_policy_and_strength() {
    assert!(meets_policy("long-enough", 8));
    assert!(!meets_policy("tiny", 8));

    let mut score = 0;
    update_strength(&mut score, "Tr0ub4dor&3Extra");
    assert_eq!(score, 30);
}

#[test]
fn test_average_scores_on_history() {
    assert_eq!(average_scores(&[10, 20, 30]), Some(20));
    assert_eq!(average_scores(&[]), None);
}

#[test]
fn test_mask_keep_last_for_logs() {
    assert_eq!(mask_keep_last("super-secret", 3), "*********ret");
}
