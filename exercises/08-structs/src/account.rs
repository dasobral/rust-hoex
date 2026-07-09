//! User account and role modeling for access-control exercises.

/// Privilege tier for a security principal.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Role {
    Viewer,
    Operator,
    Admin,
}

/// A user account with credential material and lockout tracking.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserAccount {
    pub username: String,
    pub email: String,
    pub role: Role,
    pub failed_logins: u32,
    pub locked: bool,
    credential: String,
}

impl UserAccount {
    /// Maximum failed attempts before the account is locked.
    pub const LOCK_THRESHOLD: u32 = 3;

    /// Create a new unlocked account with zero failed login attempts.
    #[must_use]
    pub fn new(
        username: impl Into<String>,
        email: impl Into<String>,
        credential: impl Into<String>,
        role: Role,
    ) -> Self {
        Self {
            username: username.into(),
            email: email.into(),
            role,
            credential: credential.into(),
            failed_logins: 0,
            locked: false,
        }
    }

    /// Whether this account holds administrative privileges.
    #[must_use]
    pub const fn is_admin(&self) -> bool {
        matches!(self.role, Role::Admin)
    }

    /// Whether authentication attempts are permitted.
    #[must_use]
    pub const fn can_authenticate(&self) -> bool {
        !self.locked
    }

    /// Check a candidate credential without exposing stored material.
    #[must_use]
    pub fn verify_credential(&self, candidate: &str) -> bool {
        !self.locked && self.credential == candidate
    }

    /// Record a failed login; lock when the threshold is reached.
    pub const fn record_failed_login(&mut self) {
        self.failed_logins = self.failed_logins.saturating_add(1);
        if self.failed_logins >= Self::LOCK_THRESHOLD {
            self.locked = true;
        }
    }

    /// Clear lockout state after an administrator reset.
    pub const fn unlock(&mut self) {
        self.failed_logins = 0;
        self.locked = false;
    }

    /// Produce a single-line audit record and consume the account.
    #[must_use]
    pub fn into_audit_line(self) -> String {
        format!(
            "user={} email={} role={:?} locked={} fails={}",
            self.username, self.email, self.role, self.locked, self.failed_logins
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_account_is_unlocked() {
        let acct = UserAccount::new("alice", "alice@secops.local", "hunter2", Role::Operator);
        assert!(acct.can_authenticate());
        assert!(!acct.is_admin());
        assert_eq!(acct.failed_logins, 0);
    }

    #[test]
    fn admin_role_detected() {
        let acct = UserAccount::new("root", "root@secops.local", "x", Role::Admin);
        assert!(acct.is_admin());
    }

    #[test]
    fn verify_credential_respects_lock() {
        let mut acct = UserAccount::new("bob", "bob@secops.local", "token", Role::Viewer);
        for _ in 0..UserAccount::LOCK_THRESHOLD {
            acct.record_failed_login();
        }
        assert!(acct.locked);
        assert!(!acct.verify_credential("token"));
    }

    #[test]
    fn unlock_restores_access() {
        let mut acct = UserAccount::new("carol", "carol@secops.local", "y", Role::Viewer);
        for _ in 0..UserAccount::LOCK_THRESHOLD {
            acct.record_failed_login();
        }
        acct.unlock();
        assert!(acct.can_authenticate());
        assert!(acct.verify_credential("y"));
    }

    #[test]
    fn into_audit_line_includes_email() {
        let acct = UserAccount::new("dave", "dave@secops.local", "z", Role::Admin);
        let line = acct.into_audit_line();
        assert!(line.contains("dave@secops.local"));
        assert!(line.contains("Admin"));
    }
}
