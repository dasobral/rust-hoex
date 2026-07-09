//! Option-like token wrapper for session handling exercises.

/// Teaching stand-in for `Option<String>` when modeling bearer tokens.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MaybeToken {
    Some(String),
    None,
}

impl MaybeToken {
    /// Return the inner token or a caller-supplied default.
    #[must_use]
    pub fn unwrap_or(self, default: String) -> String {
        match self {
            Self::Some(token) => token,
            Self::None => default,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unwrap_or_returns_inner() {
        let token = MaybeToken::Some("abc".to_owned());
        assert_eq!(token.unwrap_or("fallback".to_owned()), "abc");
    }

    #[test]
    fn unwrap_or_uses_default() {
        let token = MaybeToken::None;
        assert_eq!(token.unwrap_or("guest".to_owned()), "guest");
    }
}
