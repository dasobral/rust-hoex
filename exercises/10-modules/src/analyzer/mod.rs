//! Password analyzer module tree — shared types plus `score`, `risk`, and `validate`.

pub mod risk;
pub mod score;
pub mod validate;

pub use risk::RiskLevel;

/// Maximum score allowed when a secret is on the common-password denylist.
const COMMON_SCORE_CAP: u8 = 10;

/// Result of analyzing a password or API token.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Analysis {
    /// Numeric strength from 0 (worst) to 100 (best).
    pub score: u8,
    /// Coarse risk classification.
    pub risk: RiskLevel,
    /// Length of the input in bytes (UTF-8).
    pub length: usize,
    /// Whether the input mixes character classes.
    pub mixed_classes: bool,
    /// Whether the secret matched a denylisted common password.
    pub too_common: bool,
}

impl Analysis {
    /// One-line summary for CLI output.
    #[must_use]
    pub fn summary(&self) -> String {
        format!(
            "score={} risk={} len={} mixed={} common={}",
            self.score,
            self.risk.as_str(),
            self.length,
            self.mixed_classes,
            self.too_common
        )
    }
}

/// Analyze `secret` and return an [`Analysis`], capping score when too common.
#[must_use]
pub fn analyze(secret: &str) -> Analysis {
    let length = secret.len();
    let mixed_classes = score::has_mixed_classes(secret);
    let too_common = validate::is_too_common(secret);
    let mut score = score::compute_score(secret);

    if too_common {
        score = score.min(COMMON_SCORE_CAP);
    }

    let risk = RiskLevel::from_score(score);

    Analysis {
        score,
        risk,
        length,
        mixed_classes,
        too_common,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn analyze_caps_common_password() {
        let result = analyze("password");
        assert!(result.too_common);
        assert!(result.score <= COMMON_SCORE_CAP);
        assert_eq!(result.risk, RiskLevel::Critical);
    }

    #[test]
    fn analyze_strong_secret_not_capped() {
        let result = analyze("Correct-Horse-99!");
        assert!(!result.too_common);
        assert!(result.score > COMMON_SCORE_CAP);
    }
}
