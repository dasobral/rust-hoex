//! Integration tests for `exercise_structs`.

use structs_exercises::account::{Role, UserAccount};
use structs_exercises::auth_flow::authenticate;
use structs_exercises::session::Session;
use structs_exercises::{get_exercise_list, run_all, run_exercise};

#[test]
fn exercise_list_complete() {
    let list = get_exercise_list();
    assert_eq!(list.len(), 2);
    let names: Vec<&str> = list.iter().map(|e| e.name).collect();
    assert!(names.contains(&"auth-flow"));
    assert!(names.contains(&"lockout"));
}

#[test]
fn run_each_exercise() {
    assert!(run_exercise("auth-flow", false).is_ok());
    assert!(run_exercise("lockout", false).is_ok());
}

#[test]
fn run_all_exercises() {
    assert!(run_all(false).is_ok());
}

#[test]
fn account_session_integration() {
    let acct = UserAccount::new("ops", "ops@secops.local", "key-42", Role::Operator);
    let session = authenticate(&acct, "key-42");
    assert!(session.is_ok());
    if let Ok(s) = session {
        assert_eq!(s.user, "ops");
        assert!(s.matches_token(&s.token));
    }
}

#[test]
fn session_partial_eq() {
    let s1 = Session::new("alice", "tok");
    let s2 = Session::new("alice", "tok");
    let s3 = Session::new("alice", "other");
    assert_eq!(s1, s2);
    assert_ne!(s1, s3);
}

#[test]
fn admin_audit_consumes_account() {
    let acct = UserAccount::new("admin", "admin@secops.local", "x", Role::Admin);
    let line = acct.into_audit_line();
    assert!(line.contains("admin@secops.local"));
    assert!(line.contains("Admin"));
}
