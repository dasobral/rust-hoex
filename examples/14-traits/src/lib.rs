//! Shared threat-scoring behavior via traits (static dispatch).
//!
//! # What you will see
//!
//! - Defining a trait (`ThreatScorer`) with required + default methods
//! - Implementing the trait for different event types
//! - Trait bounds on functions (`T: ThreatScorer`)
//! - `derive` vs manual impl (e.g. `Debug` derived, scoring logic manual)
//! - Prefer **static dispatch** (`impl ThreatScorer` / generics) over `dyn`

use std::fmt::Display;

/// Something that can be scored as a potential threat.
///
/// Types that implement this trait share a common API even if their fields
/// differ — that is the point of traits.
pub trait ThreatScorer {
    /// Numeric risk score in `0..=100`.
    fn score(&self) -> u32;

    /// Short category label (e.g. `"auth"`, `"network"`).
    fn category(&self) -> &'static str;

    /// Default method — works for every implementor unless overridden.
    ///
    /// Default methods let you ship shared behavior in the trait itself.
    fn risk_level(&self) -> RiskLevel {
        match self.score() {
            0..=24 => RiskLevel::Low,
            25..=59 => RiskLevel::Medium,
            60..=84 => RiskLevel::High,
            _ => RiskLevel::Critical,
        }
    }

    /// Another default: human-readable one-liner.
    fn summary(&self) -> String {
        format!(
            "[{}] score={} ({:?})",
            self.category(),
            self.score(),
            self.risk_level()
        )
    }
}

/// Coarse risk band derived from a score.
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

/// Failed authentication attempt.
///
/// `Debug` / `Clone` / `PartialEq` come from **derive**. Scoring is a **manual**
/// `impl ThreatScorer` — derive cannot invent domain logic.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthFailure {
    pub user: String,
    pub attempts: u32,
    pub from_known_host: bool,
}

impl ThreatScorer for AuthFailure {
    fn score(&self) -> u32 {
        let mut s: u32 = self.attempts.saturating_mul(15);
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

    // Override the default summary for a richer network-specific line.
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
}

/// Score any `ThreatScorer` — **static dispatch** via a trait bound.
///
/// The compiler monomorphizes this for each concrete `T`. No vtable needed.
pub fn analyze_event<T: ThreatScorer>(event: &T) -> String {
    event.summary()
}

/// Return `true` when the event is at least `High` risk.
pub fn is_actionable<T: ThreatScorer>(event: &T) -> bool {
    matches!(event.risk_level(), RiskLevel::High | RiskLevel::Critical)
}

/// Pick the higher-scoring of two events (same concrete type).
pub fn higher_risk<'a, T: ThreatScorer>(a: &'a T, b: &'a T) -> &'a T {
    if a.score() >= b.score() { a } else { b }
}

/// Collect summaries from a slice of the **same** concrete type.
///
/// Homogeneous lists stay with static dispatch. (Heterogeneous lists would
/// need `dyn ThreatScorer` — useful later, but skip for now.)
pub fn summarize_all<T: ThreatScorer>(events: &[T]) -> Vec<String> {
    events.iter().map(ThreatScorer::summary).collect()
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
        assert_eq!(mild.category(), "auth");

        let bad = AuthFailure {
            user: "bob".into(),
            attempts: 5,
            from_known_host: false,
        };
        assert_eq!(bad.score(), 100);
        assert_eq!(bad.risk_level(), RiskLevel::Critical);
        assert!(is_actionable(&bad));
    }

    #[test]
    fn network_scan_overrides_summary() {
        let scan = NetworkScan {
            source_ip: "203.0.113.9".into(),
            ports_hit: 20,
            payload_bytes: 4096,
        };
        // ports: min(20*5, 60)=60; bytes: min(4096/1024, 40)=4 → 64
        assert_eq!(scan.score(), 64);
        assert!(scan.summary().contains("203.0.113.9"));
        assert!(scan.summary().contains("ports"));
    }

    #[test]
    fn malware_alert_uses_confidence() {
        let alert = MalwareAlert {
            hash_prefix: "deadbeef".into(),
            confidence: 70,
        };
        assert_eq!(alert.score(), 70);
        assert_eq!(alert.risk_level(), RiskLevel::High);
        assert!(analyze_event(&alert).contains("malware"));
    }

    #[test]
    fn higher_risk_picks_max() {
        let a = MalwareAlert {
            hash_prefix: "aaa".into(),
            confidence: 40,
        };
        let b = MalwareAlert {
            hash_prefix: "bbb".into(),
            confidence: 80,
        };
        assert_eq!(higher_risk(&a, &b).hash_prefix, "bbb");
    }

    #[test]
    fn summarize_all_static_dispatch() {
        let events = [
            AuthFailure {
                user: "u".into(),
                attempts: 2,
                from_known_host: true,
            },
            AuthFailure {
                user: "v".into(),
                attempts: 4,
                from_known_host: false,
            },
        ];
        let lines = summarize_all(&events);
        assert_eq!(lines.len(), 2);
        assert!(lines[0].contains("auth"));
    }

    #[test]
    fn risk_level_display() {
        assert_eq!(RiskLevel::Medium.to_string(), "medium");
    }
}
