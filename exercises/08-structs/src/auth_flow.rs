//! End-to-end authentication flow combining accounts and sessions.

use anyhow::{Result, bail};

use crate::account::{Role, UserAccount};
use crate::session::Session;

/// Attempt login: verify credentials and mint a session on success.
pub fn authenticate(account: &UserAccount, candidate: &str) -> Result<Session> {
    if !account.can_authenticate() {
        bail!("account {} is locked", account.username);
    }
    if account.verify_credential(candidate) {
        let token = format!("sess-{}-{}", account.username, account.email.len());
        Ok(Session::new(account.username.clone(), token))
    } else {
        bail!("invalid credential for {}", account.username);
    }
}

/// Run the authentication-flow exercise.
pub fn run(verbose: bool) -> Result<()> {
    println!("🔐 Authentication Flow — Accounts & Sessions");
    println!();

    let mut operator = UserAccount::new(
        "alice",
        "alice@secops.local",
        "hunter2-hash",
        Role::Operator,
    );
    println!("1. Created operator account: {}", operator.username);
    println!("   email: {}", operator.email);
    println!("   can authenticate: {}", operator.can_authenticate());

    match authenticate(&operator, "hunter2-hash") {
        Ok(session) => {
            println!("2. Login succeeded → session user={}", session.user);
            println!("   token match: {}", session.matches_token(&session.token));
        }
        Err(e) => println!("2. Login failed: {e}"),
    }

    println!();
    println!("3. Simulating brute-force lockout:");
    for attempt in 1..=UserAccount::LOCK_THRESHOLD {
        operator.record_failed_login();
        println!(
            "   attempt {attempt}: fails={}, locked={}",
            operator.failed_logins, operator.locked
        );
    }

    let locked_result = authenticate(&operator, "hunter2-hash");
    if locked_result.is_err() {
        println!("4. Locked account correctly rejects login");
    }

    operator.unlock();
    println!(
        "5. After admin unlock: can_authenticate={}",
        operator.can_authenticate()
    );

    if verbose {
        println!();
        println!("   Admin audit line:");
        let admin = UserAccount::new("root", "root@secops.local", "admin-key", Role::Admin);
        println!("   {}", admin.into_audit_line());
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn authenticate_success() {
        let acct = UserAccount::new("alice", "a@x.local", "secret", Role::Viewer);
        let session = authenticate(&acct, "secret");
        assert!(session.is_ok());
        if let Ok(s) = session {
            assert_eq!(s.user, "alice");
            assert!(s.matches_token(&s.token));
        }
    }

    #[test]
    fn authenticate_locked_fails() {
        let mut acct = UserAccount::new("bob", "b@x.local", "x", Role::Viewer);
        for _ in 0..UserAccount::LOCK_THRESHOLD {
            acct.record_failed_login();
        }
        assert!(authenticate(&acct, "x").is_err());
    }
}
