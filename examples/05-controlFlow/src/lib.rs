//! Shared helpers for the control-flow example.
//!
//! A tiny threat-score / log classifier: severity enums, scoring with `if`
//! expressions, `match`, loops with `break`/`continue`, and `if let`.

/// Severity levels for a classified security event.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity {
    Info,
    Low,
    Medium,
    High,
    Critical,
}

/// Access-control decision produced by the policy engine.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccessDecision {
    Allow,
    Challenge,
    Deny,
}

/// Map a numeric threat score (0..=100) to a [`Severity`].
///
/// `match` on integer ranges is exhaustive when a final `_` arm is present.
#[must_use]
pub const fn severity_from_score(score: u8) -> Severity {
    match score {
        0..=14 => Severity::Info,
        15..=39 => Severity::Low,
        40..=69 => Severity::Medium,
        70..=89 => Severity::High,
        // 90..=255 (u8 max) maps to Critical — includes out-of-range clamps.
        90.. => Severity::Critical,
    }
}

/// Convert severity to a short label (another `match` on an enum).
#[must_use]
pub const fn severity_label(level: Severity) -> &'static str {
    match level {
        Severity::Info => "INFO",
        Severity::Low => "LOW",
        Severity::Medium => "MEDIUM",
        Severity::High => "HIGH",
        Severity::Critical => "CRITICAL",
    }
}

/// Numeric weight used when aggregating events.
#[must_use]
pub const fn severity_weight(level: Severity) -> u32 {
    match level {
        Severity::Info => 1,
        Severity::Low => 2,
        Severity::Medium => 4,
        Severity::High => 8,
        Severity::Critical => 16,
    }
}

/// Decide access from severity — `if` is an **expression** here.
#[must_use]
pub const fn access_decision(level: Severity) -> AccessDecision {
    // Both branches (and else-if chain) must produce the same type.
    if matches!(level, Severity::Critical) {
        AccessDecision::Deny
    } else if matches!(level, Severity::High) {
        AccessDecision::Challenge
    } else {
        AccessDecision::Allow
    }
}

/// Human-readable decision text.
#[must_use]
pub const fn decision_label(decision: AccessDecision) -> &'static str {
    match decision {
        AccessDecision::Allow => "ALLOW",
        AccessDecision::Challenge => "CHALLENGE",
        AccessDecision::Deny => "DENY",
    }
}

/// Score a single log line with simple keyword heuristics (0..=100).
#[must_use]
pub fn score_log_line(line: &str) -> u8 {
    let lower = line.to_ascii_lowercase();
    let mut score: u32 = 0;

    // `for` iterates anything that implements `IntoIterator`.
    for (keyword, points) in [
        ("critical", 40_u32),
        ("malware", 35),
        ("exploit", 30),
        ("unauthorized", 25),
        ("failed login", 20),
        ("warning", 10),
        ("info", 2),
    ] {
        if lower.contains(keyword) {
            score = score.saturating_add(points);
        }
    }

    // Cap at 100 using an `if` expression assigned to a binding.
    let capped: u32 = if score > 100 { 100 } else { score };
    u8::try_from(capped).unwrap_or(100)
}

/// Classify many lines; skip blanks with `continue`; stop early with `break`.
///
/// Returns `(events_counted, total_weight, stopped_early)`.
#[must_use]
pub fn classify_batch(lines: &[&str], max_events: usize) -> (usize, u32, bool) {
    let mut counted = 0_usize;
    let mut total_weight = 0_u32;
    let mut stopped_early = false;

    // Infinite `loop` with a valued `break` — teaching break-with-value.
    loop {
        if counted >= lines.len() {
            break (counted, total_weight, stopped_early);
        }

        let line = lines[counted];
        counted += 1;

        // `continue` skips the rest of this iteration.
        if line.trim().is_empty() {
            continue;
        }

        let score = score_log_line(line);
        let level = severity_from_score(score);
        total_weight = total_weight.saturating_add(severity_weight(level));

        // Stop once we have processed `max_events` non-empty lines.
        let non_empty_processed = counted; // simplified budget for the demo
        if non_empty_processed >= max_events {
            stopped_early = counted < lines.len();
            break (counted, total_weight, stopped_early);
        }
    }
}

/// Sum weights with a `while` loop until the score budget is exhausted.
#[must_use]
pub fn accumulate_until_budget(scores: &[u8], budget: u32) -> u32 {
    let mut idx = 0_usize;
    let mut used = 0_u32;

    while idx < scores.len() {
        let level = severity_from_score(scores[idx]);
        let weight = severity_weight(level);
        if used.saturating_add(weight) > budget {
            break;
        }
        used = used.saturating_add(weight);
        idx += 1;
    }
    used
}

/// Optional override: if an analyst pin is present, force Challenge.
///
/// Demonstrates `if let` on `Option`.
#[must_use]
pub fn apply_analyst_pin(decision: AccessDecision, pin: Option<&str>) -> AccessDecision {
    // `if let` with a guard keeps the Option demo while staying clippy-clean.
    if let Some(code) = pin
        && code == "REVIEW"
    {
        return AccessDecision::Challenge;
    }
    decision
}

/// Find the first Critical score index, or `None`.
#[must_use]
pub fn first_critical_index(scores: &[u8]) -> Option<usize> {
    for (idx, &score) in scores.iter().enumerate() {
        if matches!(severity_from_score(score), Severity::Critical) {
            return Some(idx);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn severity_ranges() {
        assert_eq!(severity_from_score(0), Severity::Info);
        assert_eq!(severity_from_score(20), Severity::Low);
        assert_eq!(severity_from_score(50), Severity::Medium);
        assert_eq!(severity_from_score(80), Severity::High);
        assert_eq!(severity_from_score(95), Severity::Critical);
        assert_eq!(severity_from_score(200), Severity::Critical);
    }

    #[test]
    fn access_decision_ladder() {
        assert_eq!(access_decision(Severity::Info), AccessDecision::Allow);
        assert_eq!(access_decision(Severity::High), AccessDecision::Challenge);
        assert_eq!(access_decision(Severity::Critical), AccessDecision::Deny);
    }

    #[test]
    fn score_log_line_keywords() {
        let quiet = score_log_line("routine info heartbeat");
        let loud = score_log_line("CRITICAL malware exploit detected");
        assert!(quiet < 30);
        assert!(loud >= 90);
    }

    #[test]
    fn classify_batch_skips_blanks() {
        let lines = ["", "warning disk", "", "info ok"];
        let (counted, weight, _) = classify_batch(&lines, 10);
        assert_eq!(counted, 4);
        assert!(weight > 0);
    }

    #[test]
    fn accumulate_until_budget_stops() {
        let scores = [95_u8, 95, 95];
        let used = accumulate_until_budget(&scores, 20);
        // Critical weight is 16; second would exceed 20.
        assert_eq!(used, 16);
    }

    #[test]
    fn analyst_pin_and_first_critical() {
        let decision = apply_analyst_pin(AccessDecision::Allow, Some("REVIEW"));
        assert_eq!(decision, AccessDecision::Challenge);
        assert_eq!(
            apply_analyst_pin(AccessDecision::Deny, None),
            AccessDecision::Deny
        );
        assert_eq!(first_critical_index(&[10, 20, 95]), Some(2));
        assert_eq!(first_critical_index(&[10, 20]), None);
    }

    #[test]
    fn labels() {
        assert_eq!(severity_label(Severity::Medium), "MEDIUM");
        assert_eq!(decision_label(AccessDecision::Deny), "DENY");
    }
}
