//! Integration tests for `example_structs`.

use example_structs::{Role, UserAccount, rename_account};

#[test]
fn constructor_and_debug() {
    let acct = UserAccount::new("integration", "tok", Role::Viewer);
    let debug = format!("{acct:?}");
    assert!(debug.contains("integration"));
    assert!(debug.contains("Viewer"));
}

#[test]
fn authentication_flow() {
    let mut acct = UserAccount::new("user", "pass", Role::Operator);
    assert!(acct.verify_credential("pass"));
    acct.record_failed_login();
    acct.record_failed_login();
    acct.record_failed_login();
    assert!(acct.locked);
    assert!(!acct.verify_credential("pass"));
    acct.unlock();
    assert!(acct.verify_credential("pass"));
}

#[test]
fn update_syntax_preserves_credential() {
    let a = UserAccount::new("a", "shared-cred", Role::Admin);
    let b = rename_account(&a, "b");
    assert_eq!(b.username, "b");
    assert!(b.verify_credential("shared-cred"));
    assert_eq!(a.role, b.role);
}

#[test]
fn consuming_audit_line() {
    let acct = UserAccount::new("zoe", "c", Role::Admin);
    let line = acct.into_audit_line();
    assert!(line.contains("zoe"));
    assert!(line.contains("Admin"));
}
