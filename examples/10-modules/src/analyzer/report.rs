//! Turn an [`Analysis`](super::Analysis) into a readable multi-line report.
//!
//! Sibling modules reach this via paths relative to `analyzer` — for example
//! `report::format_report` from `mod.rs`. The function is `pub(crate)` so it
//! stays internal to this crate.

use super::Analysis;

/// Format a full report string for `analysis`.
pub(crate) fn format_report(analysis: &Analysis) -> String {
    let mixed = if analysis.mixed_classes { "yes" } else { "no" };
    format!(
        "strength score : {}\nrisk level     : {}\ninput length   : {}\nmixed classes  : {}\nadvice         : {}",
        analysis.score,
        analysis.risk.as_str(),
        analysis.length,
        mixed,
        advice(analysis),
    )
}

const fn advice(analysis: &Analysis) -> &'static str {
    match analysis.risk {
        super::RiskLevel::Critical => "reject — too weak for any sensitive use",
        super::RiskLevel::High => "require a longer passphrase or more character classes",
        super::RiskLevel::Medium => "acceptable for low-risk contexts; prefer longer still",
        super::RiskLevel::Low => "looks solid for typical account passwords",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::analyzer::{RiskLevel, analyze};

    #[test]
    fn report_contains_risk_label() {
        let analysis = analyze("short");
        let text = format_report(&analysis);
        assert!(text.contains(analysis.risk.as_str()));
        assert!(text.contains("strength score"));
    }

    #[test]
    fn critical_advice_mentions_reject() {
        let analysis = Analysis {
            score: 0,
            risk: RiskLevel::Critical,
            length: 0,
            mixed_classes: false,
        };
        assert!(format_report(&analysis).contains("reject"));
    }
}
