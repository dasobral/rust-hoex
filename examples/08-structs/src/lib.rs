//! User account and security credential structs.
//!
//! Demonstrates struct definition, `impl` blocks, method receivers
//! (`&self`, `&mut self`, `self`), associated functions (`new`),
//! `Debug`, field access, and struct update syntax.

/// Access level for an account.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Role {
    Viewer,
    Operator,
    Admin,
}

/// A user account with a security credential (password hash placeholder).
///
/// In real systems the credential would be a salted hash, not a plaintext
/// string. We keep a short owned token here for teaching field ownership.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserAccount {
    pub username: String,
    pub role: Role,
    /// Opaque credential material (educational stand-in for a password hash).
    credential: String,
    pub failed_logins: u32,
    pub locked: bool,
}

impl UserAccount {
    /// Associated function (constructor) — call as `UserAccount::new(...)`.
    #[must_use]
    pub fn new(username: impl Into<String>, credential: impl Into<String>, role: Role) -> Self {
        Self {
            username: username.into(),
            role,
            credential: credential.into(),
            failed_logins: 0,
            locked: false,
        }
    }

    /// Shared method — reads through `&self` without taking ownership.
    #[must_use]
    pub fn display_name(&self) -> &str {
        &self.username
    }

    /// Whether the account may attempt authentication.
    #[must_use]
    pub const fn can_authenticate(&self) -> bool {
        !self.locked
    }

    /// Credential length (never expose the raw credential in APIs).
    #[must_use]
    pub const fn credential_len(&self) -> usize {
        self.credential.len()
    }

    /// Verify a candidate against the stored credential (toy equality check).
    #[must_use]
    pub fn verify_credential(&self, candidate: &str) -> bool {
        !self.locked && self.credential == candidate
    }

    /// Mutable method — update state through `&mut self`.
    pub const fn record_failed_login(&mut self) {
        self.failed_logins = self.failed_logins.saturating_add(1);
        if self.failed_logins >= 3 {
            self.locked = true;
        }
    }

    /// Reset lockout counters.
    pub const fn unlock(&mut self) {
        self.failed_logins = 0;
        self.locked = false;
    }

    /// Promote role (mutable field update).
    pub const fn set_role(&mut self, role: Role) {
        self.role = role;
    }

    /// Consuming method — takes `self` by value and returns a summary string.
    ///
    /// After calling this, the `UserAccount` is gone (moved into the method).
    #[must_use]
    pub fn into_audit_line(self) -> String {
        format!(
            "user={} role={:?} locked={} fails={}",
            self.username, self.role, self.locked, self.failed_logins
        )
    }
}

/// Build a sibling account with the same role/lock state but a new username
/// via **struct update syntax**.
#[must_use]
pub fn rename_account(base: &UserAccount, new_username: impl Into<String>) -> UserAccount {
    UserAccount {
        username: new_username.into(),
        // remaining fields copied/cloned from `base` via update syntax:
        // `credential` and `username` are String (Clone); role/flags are Copy.
        ..base.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_creates_unlocked_account() {
        let acct = UserAccount::new("alice", "s3cret", Role::Operator);
        assert_eq!(acct.display_name(), "alice");
        assert_eq!(acct.role, Role::Operator);
        assert!(acct.can_authenticate());
        assert_eq!(acct.failed_logins, 0);
        assert_eq!(acct.credential_len(), 6);
    }

    #[test]
    fn verify_credential_checks_value() {
        let acct = UserAccount::new("bob", "token", Role::Viewer);
        assert!(acct.verify_credential("token"));
        assert!(!acct.verify_credential("wrong"));
    }

    #[test]
    fn record_failed_login_locks_after_three() {
        let mut acct = UserAccount::new("carol", "x", Role::Viewer);
        acct.record_failed_login();
        acct.record_failed_login();
        assert!(!acct.locked);
        acct.record_failed_login();
        assert!(acct.locked);
        assert!(!acct.can_authenticate());
        assert!(!acct.verify_credential("x"));
    }

    #[test]
    fn unlock_clears_lockout() {
        let mut acct = UserAccount::new("dave", "y", Role::Admin);
        for _ in 0..3 {
            acct.record_failed_login();
        }
        acct.unlock();
        assert!(acct.can_authenticate());
        assert_eq!(acct.failed_logins, 0);
    }

    #[test]
    fn set_role_updates_field() {
        let mut acct = UserAccount::new("erin", "z", Role::Viewer);
        acct.set_role(Role::Admin);
        assert_eq!(acct.role, Role::Admin);
    }

    #[test]
    fn into_audit_line_consumes_account() {
        let acct = UserAccount::new("frank", "c", Role::Operator);
        let line = acct.into_audit_line();
        assert!(line.contains("frank"));
        assert!(line.contains("Operator"));
    }

    #[test]
    fn rename_account_uses_update_syntax() {
        let base = UserAccount::new("old", "cred", Role::Admin);
        let renamed = rename_account(&base, "new");
        assert_eq!(renamed.username, "new");
        assert_eq!(renamed.role, Role::Admin);
        assert!(renamed.verify_credential("cred"));
        assert_eq!(base.username, "old");
    }
}
