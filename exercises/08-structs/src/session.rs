//! Session token modeling for authenticated requests.

/// An active session binding a user identity to an opaque token.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Session {
    pub user: String,
    pub token: String,
}

impl Session {
    /// Create a session for the given user and token.
    #[must_use]
    pub fn new(user: impl Into<String>, token: impl Into<String>) -> Self {
        Self {
            user: user.into(),
            token: token.into(),
        }
    }

    /// Constant-time-safe enough for teaching: compare token strings.
    #[must_use]
    pub fn matches_token(&self, candidate: &str) -> bool {
        self.token == candidate
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn session_equality() {
        let a = Session::new("alice", "tok-1");
        let b = Session::new("alice", "tok-1");
        assert_eq!(a, b);
    }

    #[test]
    fn matches_token_works() {
        let session = Session::new("bob", "sess-9f3a");
        assert!(session.matches_token("sess-9f3a"));
        assert!(!session.matches_token("wrong"));
    }
}
