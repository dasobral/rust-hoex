//! Shared threat-scoring behavior via traits (static dispatch).

use std::fmt::Display;

/// Coarse risk band derived from a numeric score.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RiskLevel {
    Low,
    Medium,
    High,
    Critical,
}

impl Display for RiskLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Low => write!(f, "low"),
            Self::Medium => write!(f, "medium"),
            Self::High => write!(f, "high"),
            Self::Critical => write!(f, "critical"),
        }
    }
}

/// Something that can be scored as a potential security threat.
pub trait ThreatScorer {
    /// Numeric risk score in `0..=100`.
    fn score(&self) -> u32;

    /// Short category label (e.g. `"auth"`, `"network"`).
    fn category(&self) -> &'static str;

    /// Map score bands to a coarse risk level (overridable per type).
    fn risk_level(&self) -> RiskLevel {
        match self.score() {
            0..=24 => RiskLevel::Low,
            25..=59 => RiskLevel::Medium,
            60..=84 => RiskLevel::High,
            _ => RiskLevel::Critical,
        }
    }

    /// Human-readable one-liner (overridable per type).
    fn summary(&self) -> String {
        format!(
            "[{}] score={} ({})",
            self.category(),
            self.score(),
            self.risk_level()
        )
    }

    /// Whether the event demands immediate escalation.
    ///
    /// Default: score ≥ 85 or [`RiskLevel::Critical`].
    fn is_critical(&self) -> bool {
        self.score() >= 85 || self.risk_level() == RiskLevel::Critical
    }
}

/// Failed authentication attempt.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthFailure {
    pub user: String,
    pub attempts: u32,
    pub from_known_host: bool,
}

impl ThreatScorer for AuthFailure {
    fn score(&self) -> u32 {
        let mut s = self.attempts.saturating_mul(15);
        if !self.from_known_host {
            s = s.saturating_add(25);
        }
        s.min(100)
    }

    fn category(&self) -> &'static str {
        "auth"
    }
}

/// Suspicious network scan / connection burst.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NetworkScan {
    pub source_ip: String,
    pub ports_hit: u32,
    pub payload_bytes: u32,
}

impl ThreatScorer for NetworkScan {
    fn score(&self) -> u32 {
        let port_part = self.ports_hit.saturating_mul(5).min(60);
        let byte_part = (self.payload_bytes / 1024).min(40);
        port_part.saturating_add(byte_part).min(100)
    }

    fn category(&self) -> &'static str {
        "network"
    }

    fn summary(&self) -> String {
        format!(
            "[network] {} hit {} ports ({} bytes) → {} ({})",
            self.source_ip,
            self.ports_hit,
            self.payload_bytes,
            self.score(),
            self.risk_level()
        )
    }
}

/// Malware / hash alert from an endpoint sensor.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MalwareAlert {
    pub hash_prefix: String,
    pub confidence: u32,
}

impl ThreatScorer for MalwareAlert {
    fn score(&self) -> u32 {
        self.confidence.min(100)
    }

    fn category(&self) -> &'static str {
        "malware"
    }

    /// Confidence floor: alerts at ≥ 50 % confidence are at least **High**.
    ///
    /// If the numeric score already warrants **Critical** (≥ 85), that band
    /// is preserved — high confidence never downgrades a critical score.
    fn risk_level(&self) -> RiskLevel {
        let score = self.score();
        if score >= 85 {
            RiskLevel::Critical
        } else if self.confidence >= 50 {
            RiskLevel::High
        } else {
            match score {
                0..=24 => RiskLevel::Low,
                25..=59 => RiskLevel::Medium,
                60..=84 => RiskLevel::High,
                _ => RiskLevel::Critical,
            }
        }
    }
}

/// File integrity monitoring event.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileIntegrityEvent {
    pub path: String,
    pub severity: u32,
}

impl ThreatScorer for FileIntegrityEvent {
    fn score(&self) -> u32 {
        self.severity.min(100)
    }

    fn category(&self) -> &'static str {
        "integrity"
    }

    fn summary(&self) -> String {
        format!(
            "[integrity] {} severity={} ({})",
            self.path,
            self.score(),
            self.risk_level()
        )
    }
}

/// Highest score in a homogeneous slice (static dispatch).
#[must_use]
pub fn max_score<T: ThreatScorer>(events: &[T]) -> u32 {
    events.iter().map(ThreatScorer::score).max().unwrap_or(0)
}

/// Format a single event via trait bounds.
#[must_use]
pub fn analyze_event<T: ThreatScorer>(event: &T) -> String {
    event.summary()
}

/// Whether the event is at least High risk.
#[must_use]
pub fn is_actionable<T: ThreatScorer>(event: &T) -> bool {
    matches!(event.risk_level(), RiskLevel::High | RiskLevel::Critical)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn auth_failure_scoring() {
        let mild = AuthFailure {
            user: "alice".into(),
            attempts: 1,
            from_known_host: true,
        };
        assert_eq!(mild.score(), 15);
        assert_eq!(mild.risk_level(), RiskLevel::Low);
        assert!(!mild.is_critical());

        let bad = AuthFailure {
            user: "bob".into(),
            attempts: 5,
            from_known_host: false,
        };
        assert_eq!(bad.score(), 100);
        assert!(bad.is_critical());
    }

    #[test]
    fn malware_risk_level_floor() {
        let medium_conf = MalwareAlert {
            hash_prefix: "abc".into(),
            confidence: 55,
        };
        assert_eq!(medium_conf.risk_level(), RiskLevel::High);

        let critical = MalwareAlert {
            hash_prefix: "dead".into(),
            confidence: 90,
        };
        assert_eq!(critical.risk_level(), RiskLevel::Critical);
        assert!(critical.is_critical());
    }

    #[test]
    fn file_integrity_scores() {
        let evt = FileIntegrityEvent {
            path: "/etc/shadow".into(),
            severity: 72,
        };
        assert_eq!(evt.score(), 72);
        assert!(evt.summary().contains("/etc/shadow"));
    }

    #[test]
    fn max_score_picks_highest() {
        let events = [
            AuthFailure {
                user: "u".into(),
                attempts: 1,
                from_known_host: true,
            },
            AuthFailure {
                user: "v".into(),
                attempts: 4,
                from_known_host: false,
            },
        ];
        assert_eq!(max_score(&events), 85);
    }

    #[test]
    fn risk_level_display() {
        assert_eq!(RiskLevel::Medium.to_string(), "medium");
    }
}
