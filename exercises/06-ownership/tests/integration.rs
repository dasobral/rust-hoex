//! Integration tests for the `exercise_ownership` crate.

use ownership_exercises::{
    clone_secret, consume_secret, copy_threat_score, get_exercise_list, run_all, run_exercise,
    take_then_return, zeroize_and_consume,
};

#[test]
fn test_exercise_list_contains_both() {
    let list = get_exercise_list();
    assert_eq!(list.len(), 2);

    let names: Vec<&str> = list.iter().map(|e| e.name).collect();
    assert!(names.contains(&"move_vs_copy"));
    assert!(names.contains(&"zeroize"));

    for exercise in &list {
        assert!(!exercise.description.is_empty());
        assert!(!exercise.concepts.is_empty());
    }
}

#[test]
fn test_run_each_exercise() {
    assert!(run_exercise("move_vs_copy", false).is_ok());
    assert!(run_exercise("zeroize", false).is_ok());
}

#[test]
fn test_run_unknown_exercise_errors() {
    let err = run_exercise("vault", false);
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
fn test_consume_secret_api() {
    let secret = String::from("api-key-123");
    assert_eq!(consume_secret(secret), 11);
}

#[test]
fn test_zeroize_and_consume_api() {
    let secret = String::from("refresh-token");
    assert_eq!(zeroize_and_consume(secret), 13);
}

#[test]
fn test_clone_and_move_api() {
    let original = String::from("primary");
    let cloned = clone_secret(&original);
    assert_eq!(original, "primary");
    assert_eq!(cloned, "primary");

    let moved = take_then_return(original);
    assert_eq!(moved, "primary");
}

#[test]
fn test_copy_threat_score_api() {
    let score = 17;
    assert_eq!(copy_threat_score(score), 34);
    assert_eq!(score, 17);
}
