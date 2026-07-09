//! Risk classification for password strength scores.

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

    /// Short label for display and re-export helpers.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Critical => "critical",
            Self::High => "high",
            Self::Medium => "medium",
            Self::Low => "low",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn risk_boundaries() {
        assert_eq!(RiskLevel::from_score(0), RiskLevel::Critical);
        assert_eq!(RiskLevel::from_score(24), RiskLevel::Critical);
        assert_eq!(RiskLevel::from_score(25), RiskLevel::High);
        assert_eq!(RiskLevel::from_score(50), RiskLevel::Medium);
        assert_eq!(RiskLevel::from_score(75), RiskLevel::Low);
    }
}
