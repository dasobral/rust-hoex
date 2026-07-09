//! Password strength analysis built on entropy and policy heuristics.

use std::collections::HashMap;

use crate::entropy::{CharClass, EntropyEstimate, estimate_entropy};

/// Qualitative strength buckets derived from entropy bits and policy checks.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Strength {
    /// Empty or trivially guessable.
    VeryWeak,
    /// Short / single-class / common patterns.
    Weak,
    /// Meets basic length and class diversity.
    Fair,
    /// Solid entropy and composition.
    Strong,
    /// High entropy with broad character classes.
    VeryStrong,
}

impl Strength {
    /// Human-readable label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::VeryWeak => "very weak",
            Self::Weak => "weak",
            Self::Fair => "fair",
            Self::Strong => "strong",
            Self::VeryStrong => "very strong",
        }
    }
}

/// Full analysis report for a single password.
#[derive(Debug, Clone, PartialEq)]
pub struct AnalysisReport {
    /// Entropy estimate.
    pub entropy: EntropyEstimate,
    /// Qualitative strength.
    pub strength: Strength,
    /// Policy / heuristic findings (empty when clean).
    pub findings: Vec<&'static str>,
    /// Count of each character class present (for display).
    pub class_counts: HashMap<&'static str, usize>,
}

/// Exact-match denylist (case-sensitive for teaching simplicity).
const COMMON: &[&str] = &[
    "password",
    "password1",
    "12345678",
    "qwertyui",
    "letmein",
    "admin",
    "welcome",
];

const MIN_LENGTH: usize = 8;

/// Analyze a password: entropy, strength rating, and heuristic findings.
#[must_use]
pub fn analyze_password(password: &str) -> AnalysisReport {
    let entropy = estimate_entropy(password);
    let findings = collect_findings(password, &entropy);
    let strength = rate_strength(&entropy, &findings);
    let class_counts = count_classes(password);

    AnalysisReport {
        entropy,
        strength,
        findings,
        class_counts,
    }
}

fn collect_findings(password: &str, entropy: &EntropyEstimate) -> Vec<&'static str> {
    let mut findings = Vec::new();

    if password.is_empty() {
        findings.push("empty password");
        return findings;
    }

    if entropy.length < MIN_LENGTH {
        findings.push("shorter than 8 characters");
    }
    if !entropy.classes.contains(&CharClass::Upper) {
        findings.push("no uppercase letter");
    }
    if !entropy.classes.contains(&CharClass::Lower) {
        findings.push("no lowercase letter");
    }
    if !entropy.classes.contains(&CharClass::Digit) {
        findings.push("no digit");
    }
    if !entropy.classes.contains(&CharClass::Symbol) {
        findings.push("no symbol");
    }
    if COMMON.contains(&password) {
        findings.push("matches common password denylist");
    }
    if has_repeated_run(password) {
        findings.push("contains a long repeated character run");
    }

    findings
}

/// True when any character repeats 3+ times in a row (`aaa`, `1111`, …).
fn has_repeated_run(password: &str) -> bool {
    let mut prev: Option<char> = None;
    let mut run = 0_usize;

    for c in password.chars() {
        if Some(c) == prev {
            run += 1;
            if run >= 3 {
                return true;
            }
        } else {
            prev = Some(c);
            run = 1;
        }
    }
    false
}

fn rate_strength(entropy: &EntropyEstimate, findings: &[&'static str]) -> Strength {
    if entropy.length == 0 {
        return Strength::VeryWeak;
    }

    if findings.contains(&"matches common password denylist") {
        return Strength::VeryWeak;
    }

    let bits = entropy.bits;
    let class_n = entropy.classes.len();

    if bits < 28.0 || class_n < 2 {
        Strength::VeryWeak
    } else if bits < 36.0 || findings.len() >= 3 {
        Strength::Weak
    } else if bits < 60.0 || findings.len() >= 2 {
        Strength::Fair
    } else if bits < 80.0 {
        Strength::Strong
    } else {
        Strength::VeryStrong
    }
}

fn count_classes(password: &str) -> HashMap<&'static str, usize> {
    let mut counts = HashMap::new();
    for c in password.chars() {
        let key: &'static str = if c.is_ascii_lowercase() {
            "lower"
        } else if c.is_ascii_uppercase() {
            "upper"
        } else if c.is_ascii_digit() {
            "digit"
        } else if c.is_ascii() && !c.is_ascii_alphanumeric() {
            "symbol"
        } else {
            "other"
        };
        *counts.entry(key).or_insert(0) += 1;
    }
    counts
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_is_very_weak() {
        let report = analyze_password("");
        assert_eq!(report.strength, Strength::VeryWeak);
        assert!(report.findings.contains(&"empty password"));
    }

    #[test]
    fn common_password_is_very_weak() {
        let report = analyze_password("password");
        assert_eq!(report.strength, Strength::VeryWeak);
        assert!(
            report
                .findings
                .contains(&"matches common password denylist")
        );
    }

    #[test]
    fn strong_mixed_password() {
        let report = analyze_password("Tr0ub4dor&3xY!");
        assert!(report.entropy.bits >= 60.0);
        assert!(report.strength >= Strength::Strong);
        assert!(report.findings.is_empty());
    }

    #[test]
    fn detects_repeated_run() {
        let report = analyze_password("aaaB1!");
        assert!(
            report
                .findings
                .contains(&"contains a long repeated character run")
        );
    }

    #[test]
    fn class_counts_sum_to_length() {
        let pw = "Ab1!";
        let report = analyze_password(pw);
        let total: usize = report.class_counts.values().copied().sum();
        assert_eq!(total, pw.chars().count());
    }

    #[test]
    fn strength_as_str_labels() {
        assert_eq!(Strength::Fair.as_str(), "fair");
        assert_eq!(Strength::VeryStrong.as_str(), "very strong");
    }
}
