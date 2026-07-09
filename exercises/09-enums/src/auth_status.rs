//! Authentication outcome modeling with enum variants.

/// Result of an authentication attempt against an identity provider.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AuthStatus {
    Success { user: String },
    Failure { reason: String },
    Pending,
    Locked { until: String },
}

impl AuthStatus {
    /// Whether this status grants authenticated access.
    #[must_use]
    pub const fn is_authenticated(&self) -> bool {
        matches!(self, Self::Success { .. })
    }

    /// Short summary suitable for SIEM logs or UI banners.
    #[must_use]
    pub fn summary(&self) -> String {
        match self {
            Self::Success { user } => format!("authenticated as {user}"),
            Self::Failure { reason } => format!("denied: {reason}"),
            Self::Pending => "awaiting second factor".to_owned(),
            Self::Locked { until } => format!("account locked until {until}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn success_is_authenticated() {
        let status = AuthStatus::Success {
            user: "alice".to_owned(),
        };
        assert!(status.is_authenticated());
        assert!(status.summary().contains("alice"));
    }

    #[test]
    fn locked_not_authenticated() {
        let status = AuthStatus::Locked {
            until: "2026-07-10T00:00:00Z".to_owned(),
        };
        assert!(!status.is_authenticated());
        assert!(status.summary().contains("locked"));
    }
}
