//! Config / credential path validation with `Option`, `Result`, and `?`.
//!
//! # What you will see
//!
//! - `Option` for values that may be absent (`Some` / `None`)
//! - `Result` for operations that can fail (`Ok` / `Err`)
//! - the `?` operator to propagate errors early
//! - a small custom error enum via `thiserror`
//! - `map` / `and_then` for transforming success values

use std::fmt;
use std::path::{Component, Path};

use thiserror::Error;

/// Errors that can occur while validating config paths or credentials.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum ConfigError {
    /// Path string was empty.
    #[error("config path is empty")]
    EmptyPath,

    /// Path must be absolute (start with `/` on Unix-style paths).
    #[error("config path must be absolute: {0}")]
    NotAbsolute(String),

    /// Path contains `..` — rejected to avoid traversal.
    #[error("config path must not contain '..': {0}")]
    PathTraversal(String),

    /// Extension is missing or not in the allow-list.
    #[error("unsupported config extension (want .toml/.yaml/.yml): {0}")]
    BadExtension(String),

    /// Credential string failed a basic format check.
    #[error("invalid credential: {0}")]
    InvalidCredential(String),
}

/// A validated absolute config file path.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConfigPath {
    path: String,
}

impl ConfigPath {
    /// Borrow the validated path string.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.path
    }
}

impl fmt::Display for ConfigPath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.path)
    }
}

/// Validate a config file path.
///
/// Returns `Err` on empty, relative, traversal, or bad extension.
pub fn validate_config_path(raw: &str) -> Result<ConfigPath, ConfigError> {
    if raw.is_empty() {
        return Err(ConfigError::EmptyPath);
    }

    let path = Path::new(raw);
    if !path.is_absolute() {
        return Err(ConfigError::NotAbsolute(raw.to_owned()));
    }

    if path.components().any(|c| matches!(c, Component::ParentDir)) {
        return Err(ConfigError::PathTraversal(raw.to_owned()));
    }

    let ext_ok = path
        .extension()
        .and_then(|e| e.to_str())
        .is_some_and(|e| matches!(e, "toml" | "yaml" | "yml"));

    if !ext_ok {
        return Err(ConfigError::BadExtension(raw.to_owned()));
    }

    Ok(ConfigPath {
        path: raw.to_owned(),
    })
}

/// Optional API token from an environment-like string.
///
/// Returns `None` when the input is empty or whitespace-only — absence is not
/// an error here, just “no token configured”.
#[must_use]
pub fn parse_optional_token(raw: &str) -> Option<String> {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_owned())
    }
}

/// Require a non-empty credential with a minimum length.
pub fn require_credential(raw: &str) -> Result<String, ConfigError> {
    let trimmed = raw.trim();
    if trimmed.len() < 8 {
        return Err(ConfigError::InvalidCredential(
            "must be at least 8 characters".to_owned(),
        ));
    }
    if trimmed.contains(char::is_whitespace) {
        return Err(ConfigError::InvalidCredential(
            "must not contain whitespace".to_owned(),
        ));
    }
    Ok(trimmed.to_owned())
}

/// Load settings: validate path, then optionally attach a token.
///
/// Demonstrates `?` for early return and `map` on `Option`.
pub fn load_settings(
    path: &str,
    token_raw: &str,
) -> Result<(ConfigPath, Option<String>), ConfigError> {
    let config = validate_config_path(path)?;
    // `map` transforms `Some(t)` → `Some(masked)` and leaves `None` alone.
    let token = parse_optional_token(token_raw).map(|t| mask_token(&t));
    Ok((config, token))
}

/// Mask all but the last four characters of a token for safe logging.
#[must_use]
pub fn mask_token(token: &str) -> String {
    if token.len() <= 4 {
        "****".to_owned()
    } else {
        let visible = &token[token.len() - 4..];
        format!("****{visible}")
    }
}

/// Parse a credential only when a token is present.
///
/// - `None` token → `Ok(None)` (optional auth disabled)
/// - `Some(bad)` → `Err(...)`
/// - `Some(good)` → `Ok(Some(cred))`
///
/// Uses `Option::map_or` to turn absence into `Ok(None)` and presence into a
/// nested `Result` (similar spirit to `and_then` chaining).
pub fn optional_credential(token: Option<&str>) -> Result<Option<String>, ConfigError> {
    // Absent → Ok(None); present → validate then wrap in Some.
    token.map_or(Ok(None), |raw| require_credential(raw).map(Some))
}

/// Convenience: validate path then require a credential in one pipeline.
pub fn secure_bootstrap(path: &str, secret: &str) -> Result<(ConfigPath, String), ConfigError> {
    let config = validate_config_path(path)?;
    let cred = require_credential(secret)?;
    Ok((config, cred))
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]

    use super::*;

    #[test]
    fn validate_accepts_absolute_toml() {
        let p = validate_config_path("/etc/app/config.toml").unwrap();
        assert_eq!(p.as_str(), "/etc/app/config.toml");
    }

    #[test]
    fn validate_rejects_empty_relative_traversal_bad_ext() {
        assert_eq!(validate_config_path(""), Err(ConfigError::EmptyPath));
        assert!(matches!(
            validate_config_path("relative.toml"),
            Err(ConfigError::NotAbsolute(_))
        ));
        assert!(matches!(
            validate_config_path("/etc/../secret.yaml"),
            Err(ConfigError::PathTraversal(_))
        ));
        assert!(matches!(
            validate_config_path("/etc/app/config.json"),
            Err(ConfigError::BadExtension(_))
        ));
    }

    #[test]
    fn optional_token_none_on_blank() {
        assert_eq!(parse_optional_token(""), None);
        assert_eq!(parse_optional_token("   "), None);
        assert_eq!(parse_optional_token(" abc "), Some("abc".to_owned()));
    }

    #[test]
    fn require_credential_checks_length() {
        assert!(require_credential("short").is_err());
        assert!(require_credential("good-secret").is_ok());
        assert!(require_credential("has space!").is_err());
    }

    #[test]
    fn load_settings_propagates_with_question_mark() {
        let (path, token) = load_settings("/cfg/a.yaml", "super-secret-token").unwrap();
        assert_eq!(path.as_str(), "/cfg/a.yaml");
        assert_eq!(token.as_deref(), Some("****oken"));

        assert!(load_settings("bad", "x").is_err());
    }

    #[test]
    fn optional_credential_and_then_style() {
        assert_eq!(optional_credential(None).unwrap(), None);
        assert!(optional_credential(Some("tiny")).is_err());
        assert_eq!(
            optional_credential(Some("longenough")).unwrap().as_deref(),
            Some("longenough")
        );
    }

    #[test]
    fn mask_token_hides_prefix() {
        assert_eq!(mask_token("ab"), "****");
        assert_eq!(mask_token("abcdefgh"), "****efgh");
    }

    #[test]
    fn secure_bootstrap_ok() {
        let (p, c) = secure_bootstrap("/opt/x.toml", "password1").unwrap();
        assert_eq!(p.as_str(), "/opt/x.toml");
        assert_eq!(c, "password1");
    }
}
