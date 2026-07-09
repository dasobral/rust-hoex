//! Credential parsing with `Option` and `Result`.

use crate::config::ConfigError;

/// Minimum length for a non-empty credential.
const MIN_CREDENTIAL_LEN: usize = 8;

/// Optional API token from an environment-like string.
///
/// Returns `None` when the input is empty or whitespace-only.
#[must_use]
pub fn parse_optional_token(raw: &str) -> Option<String> {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_owned())
    }
}

/// Require a non-empty credential with a minimum length and no whitespace.
pub fn require_credential(raw: &str) -> Result<String, ConfigError> {
    let trimmed = raw.trim();
    if trimmed.is_empty() || trimmed.len() < MIN_CREDENTIAL_LEN {
        return Err(ConfigError::EmptyCredential);
    }
    if trimmed.contains(char::is_whitespace) {
        return Err(ConfigError::EmptyCredential);
    }
    Ok(trimmed.to_owned())
}

/// Validate a credential only when a token is present.
///
/// Uses the **transpose** pattern: `Option<Result<T, E>>` → `Result<Option<T>, E>`.
pub fn optional_credential(token: Option<&str>) -> Result<Option<String>, ConfigError> {
    token.map(require_credential).transpose()
}

/// Mask all but the last four characters for safe logging.
#[must_use]
pub fn mask_token(token: &str) -> String {
    if token.len() <= 4 {
        "****".to_owned()
    } else {
        let visible = &token[token.len() - 4..];
        format!("****{visible}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn optional_token_blank_is_none() {
        assert_eq!(parse_optional_token(""), None);
        assert_eq!(parse_optional_token("  "), None);
        assert_eq!(parse_optional_token(" tok "), Some("tok".to_owned()));
    }

    #[test]
    fn require_credential_checks() {
        assert!(require_credential("").is_err());
        assert!(require_credential("short").is_err());
        assert!(require_credential("good-secret").is_ok());
    }

    #[test]
    fn optional_credential_uses_transpose() {
        assert_eq!(optional_credential(None).ok(), Some(None));
        assert!(optional_credential(Some("tiny")).is_err());
        let cred = optional_credential(Some("longenough"));
        assert!(cred.is_ok());
        if let Ok(Some(value)) = cred {
            assert_eq!(value, "longenough");
        }
    }

    #[test]
    fn mask_token_hides_prefix() {
        assert_eq!(mask_token("ab"), "****");
        assert_eq!(mask_token("abcdefgh"), "****efgh");
    }
}
