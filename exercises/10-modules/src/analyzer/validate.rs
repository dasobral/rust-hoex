//! Validation helpers for denylisted secrets.

const COMMON_SECRETS: &[&str] = &["password", "123456"];

/// Return `true` when `secret` matches a well-known weak password (case-insensitive).
pub(crate) fn is_too_common(secret: &str) -> bool {
    let normalized = secret.to_ascii_lowercase();
    COMMON_SECRETS.iter().any(|&blocked| normalized == blocked)
}

/// Human-readable validation message for CLI demos.
pub(crate) fn validation_message(secret: &str) -> &'static str {
    if is_too_common(secret) {
        "deny: secret appears on the common-password list"
    } else if secret.is_empty() {
        "deny: empty secret"
    } else {
        "allow: not on denylist"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_common_passwords_case_insensitive() {
        assert!(is_too_common("password"));
        assert!(is_too_common("PASSWORD"));
        assert!(is_too_common("123456"));
        assert!(is_too_common("123456"));
    }

    #[test]
    fn allows_unique_secrets() {
        assert!(!is_too_common("S3cret!"));
        assert!(!is_too_common("correct-horse-battery"));
    }
}
