//! Password policy orchestration and reporting.

use crate::rules::{has_digit, has_min_length, has_punctuation, has_uppercase};

/// Outcome of checking a candidate password against the policy.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PolicyReport {
    /// Whether every hard rule passed.
    pub ok: bool,
    /// Human-readable reasons for failures (empty when `ok`).
    pub violations: Vec<&'static str>,
}

impl PolicyReport {
    /// Build a passing report.
    #[must_use]
    pub const fn pass() -> Self {
        Self {
            ok: true,
            violations: Vec::new(),
        }
    }
}

/// Check a password against length, character-class, and punctuation rules.
#[must_use]
pub fn check_policy(password: &str) -> PolicyReport {
    let mut violations = Vec::new();

    if !has_min_length(password) {
        violations.push("too short");
    }
    if !has_digit(password) {
        violations.push("missing digit");
    }
    if !has_uppercase(password) {
        violations.push("missing uppercase");
    }
    if !has_punctuation(password) {
        violations.push("missing punctuation");
    }

    PolicyReport {
        ok: violations.is_empty(),
        violations,
    }
}

/// Assert that a password is acceptable; returns `Err` with joined reasons.
///
/// # Errors
///
/// Returns a semicolon-joined list of violation messages when the policy fails.
pub fn check_or_err(password: &str) -> Result<(), String> {
    let report = check_policy(password);
    if report.ok {
        Ok(())
    } else {
        Err(report.violations.join("; "))
    }
}

/// Panic if the password is empty — used only to demonstrate `#[should_panic]` in tests.
pub fn assert_nonempty(password: &str) {
    assert!(!password.is_empty(), "password must not be empty");
}

/// Brute-force style scan over candidate strings — intentionally slow for `#[ignore]` demos.
#[must_use]
pub fn scan_candidates(
    candidates: &[&'static str],
    predicate: fn(&str) -> bool,
) -> Vec<&'static str> {
    let mut matches = Vec::new();
    for &candidate in candidates {
        if predicate(candidate) {
            matches.push(candidate);
        }
    }
    matches
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules::{MIN_LENGTH, has_min_length};

    #[test]
    fn accepts_strong_password() {
        let report = check_policy("Vault!2026");
        assert!(report.ok);
        assert!(report.violations.is_empty());
    }

    #[test]
    fn rejects_multiple_violations() {
        let report = check_policy("short");
        assert!(!report.ok);
        assert!(report.violations.contains(&"too short"));
        assert!(report.violations.contains(&"missing digit"));
    }

    #[test]
    fn check_or_err_ok_on_strong() -> Result<(), String> {
        check_or_err("Correct!1")?;
        Ok(())
    }

    #[test]
    #[should_panic(expected = "password must not be empty")]
    fn assert_nonempty_panics_on_empty() {
        assert_nonempty("");
    }

    #[test]
    fn min_length_constant_matches_rule() {
        let borderline = "a".repeat(MIN_LENGTH - 1);
        assert!(!has_min_length(&borderline));
    }
}
