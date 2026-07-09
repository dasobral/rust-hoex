//! A small password-policy library designed **to be tested**.
//!
//! This crate's teaching goal is the test harness itself: unit modules under
//! `#[cfg(test)]`, assertions, `#[should_panic]`, `Result`-returning tests,
//! and integration tests in `tests/`.
//!
//! # Running tests (filters)
//!
//! ```bash
//! cargo test                         # all tests in this package
//! cargo test policy                  # name filter: any test with "policy"
//! cargo test --lib                   # unit tests only (src/)
//! cargo test --test integration      # one integration target
//! cargo test -- --nocapture          # show println! from tests
//! cargo test -- --ignored            # run #[ignore] tests
//! ```

/// Outcome of checking a candidate password against the policy.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PolicyReport {
    /// Whether every hard rule passed.
    pub ok: bool,
    /// Human-readable reasons for failures (empty when `ok`).
    pub violations: Vec<&'static str>,
}

impl PolicyReport {
    /// Convenience: build a passing report.
    #[must_use]
    pub const fn pass() -> Self {
        Self {
            ok: true,
            violations: Vec::new(),
        }
    }
}

/// Minimum length required by the default policy.
pub const MIN_LENGTH: usize = 8;

/// Rejected exact matches (case-sensitive for teaching simplicity).
const COMMON: &[&str] = &["password", "12345678", "qwertyui"];

/// Check a password against length, character-class, and denylist rules.
#[must_use]
pub fn check_policy(password: &str) -> PolicyReport {
    let mut violations = Vec::new();

    if password.len() < MIN_LENGTH {
        violations.push("too short");
    }
    if !password.chars().any(|c| c.is_ascii_uppercase()) {
        violations.push("missing uppercase");
    }
    if !password.chars().any(|c| c.is_ascii_digit()) {
        violations.push("missing digit");
    }
    if COMMON.contains(&password) {
        violations.push("common password");
    }

    PolicyReport {
        ok: violations.is_empty(),
        violations,
    }
}

/// Rough Shannon-style entropy estimate in bits (ASCII alphabet assumption).
///
/// Not cryptographically rigorous — useful for tests and demos only.
#[must_use]
pub fn estimate_entropy_bits(password: &str) -> f64 {
    if password.is_empty() {
        return 0.0;
    }
    let mut classes = 0_u32;
    if password.chars().any(|c| c.is_ascii_lowercase()) {
        classes += 26;
    }
    if password.chars().any(|c| c.is_ascii_uppercase()) {
        classes += 26;
    }
    if password.chars().any(|c| c.is_ascii_digit()) {
        classes += 10;
    }
    if password.chars().any(|c| !c.is_ascii_alphanumeric()) {
        classes += 32;
    }
    if classes == 0 {
        return 0.0;
    }
    let Ok(len_u32) = u32::try_from(password.len()) else {
        return f64::INFINITY;
    };
    f64::from(len_u32) * f64::from(classes).log2()
}

/// Assert that a password is acceptable; returns `Err` with joined reasons.
///
/// Handy for `Result`-based tests: `check_or_err(pw)?; Ok(())`.
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

/// Panic if the password is empty — used only to demonstrate `#[should_panic]`.
///
/// Prefer `Result` / `Option` in real APIs; this exists for the testing lesson.
pub fn assert_nonempty(password: &str) {
    assert!(!password.is_empty(), "password must not be empty");
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- organizing unit tests: group by concern inside this module ---

    mod policy {
        use super::*;

        #[test]
        fn accepts_strong_password() {
            let report = check_policy("Correct1Horse");
            assert!(report.ok);
            assert!(report.violations.is_empty());
        }

        #[test]
        fn rejects_short_password() {
            let report = check_policy("Ab1");
            assert!(!report.ok);
            assert!(report.violations.contains(&"too short"));
        }

        #[test]
        fn rejects_missing_uppercase() {
            let report = check_policy("correct1horse");
            assert!(report.violations.contains(&"missing uppercase"));
        }

        #[test]
        fn rejects_missing_digit() {
            let report = check_policy("CorrectHorse");
            assert!(report.violations.contains(&"missing digit"));
        }

        #[test]
        fn rejects_common_password() {
            let report = check_policy("password");
            // "password" fails length? len=8 so length OK, but common + missing digit/upper
            assert!(!report.ok);
            assert!(report.violations.contains(&"common password"));
        }

        #[test]
        fn empty_password_has_multiple_violations() {
            let report = check_policy("");
            assert!(!report.ok);
            assert!(report.violations.len() >= 2);
        }
    }

    mod entropy {
        use super::*;

        #[test]
        fn empty_has_zero_entropy() {
            assert!((estimate_entropy_bits("") - 0.0).abs() < f64::EPSILON);
        }

        #[test]
        fn longer_mixed_has_more_bits_than_short() {
            let short = estimate_entropy_bits("Ab1!");
            let long = estimate_entropy_bits("Ab1!Ab1!Ab1!");
            assert!(long > short);
        }
    }

    mod result_style {
        use super::*;

        /// `Result`-returning tests: `?` propagates `Err` as a test failure.
        #[test]
        fn check_or_err_ok_on_strong() -> Result<(), String> {
            check_or_err("Str0ngPass")?;
            Ok(())
        }

        #[test]
        fn check_or_err_err_on_weak() {
            let result = check_or_err("weak");
            assert!(result.is_err());
            let err = result.err().map_or(String::new(), |e| e);
            assert!(err.contains("too short") || err.contains("missing"));
        }
    }

    mod panic_demo {
        use super::*;

        /// `#[should_panic]` is allowed in tests even though `panic = "deny"`
        /// applies to non-test code. Prefer Result APIs in libraries.
        #[test]
        #[should_panic(expected = "password must not be empty")]
        fn assert_nonempty_panics_on_empty() {
            assert_nonempty("");
        }

        #[test]
        fn assert_nonempty_accepts_text() {
            assert_nonempty("x");
        }
    }
}
