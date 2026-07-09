//! The `analyzer` module: shared types plus child modules `score` and `report`.
//!
//! Declaring `pub mod score;` and `pub mod report;` here tells Rust to look for
//! `score.rs` / `report.rs` next to this `mod.rs`. The modules are public so
//! paths like `analyzer::score` exist, but the helpers inside stay `pub(crate)`.

pub mod report;
pub mod score;

/// Coarse risk bucket derived from a numeric score.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RiskLevel {
    Critical,
    High,
    Medium,
    Low,
}

impl RiskLevel {
    /// Map a 0–100 score into a risk level.
    pub(crate) const fn from_score(score: u8) -> Self {
        match score {
            0..=24 => Self::Critical,
            25..=49 => Self::High,
            50..=74 => Self::Medium,
            _ => Self::Low,
        }
    }

    /// Short label for display.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Critical => "critical",
            Self::High => "high",
            Self::Medium => "medium",
            Self::Low => "low",
        }
    }
}

/// Result of analyzing a secret (password, API token, etc.).
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
}

impl Analysis {
    /// Render a multi-line human-readable report.
    pub fn report(&self) -> String {
        report::format_report(self)
    }
}

/// Analyze `secret` and return an [`Analysis`].
///
/// This is the main entry point re-exported from the crate root.
pub fn analyze(secret: &str) -> Analysis {
    let length = secret.len();
    let mixed_classes = score::has_mixed_classes(secret);
    let score = score::compute_score(secret);
    let risk = RiskLevel::from_score(score);
    Analysis {
        score,
        risk,
        length,
        mixed_classes,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn risk_from_score_boundaries() {
        assert_eq!(RiskLevel::from_score(0), RiskLevel::Critical);
        assert_eq!(RiskLevel::from_score(24), RiskLevel::Critical);
        assert_eq!(RiskLevel::from_score(25), RiskLevel::High);
        assert_eq!(RiskLevel::from_score(50), RiskLevel::Medium);
        assert_eq!(RiskLevel::from_score(75), RiskLevel::Low);
        assert_eq!(RiskLevel::from_score(100), RiskLevel::Low);
    }

    #[test]
    fn analyze_sets_length_and_mixed_flag() {
        let result = analyze("Ab1!");
        assert_eq!(result.length, 4);
        assert!(result.mixed_classes);
    }
}
