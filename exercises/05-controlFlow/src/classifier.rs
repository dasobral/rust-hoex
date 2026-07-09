//! Threat score and log classification helpers.

/// Severity levels for classified security events.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity {
    Low,
    Medium,
    High,
    Critical,
    Unknown,
}

/// Map a numeric threat score to a [`Severity`] using `match` ranges.
#[must_use]
pub const fn classify_score(score: u32) -> Severity {
    match score {
        0..=24 => Severity::Low,
        25..=49 => Severity::Medium,
        50..=74 => Severity::High,
        75..=100 => Severity::Critical,
        _ => Severity::Unknown,
    }
}

/// Convert severity to a short uppercase label.
#[must_use]
pub const fn severity_label(level: Severity) -> &'static str {
    match level {
        Severity::Low => "LOW",
        Severity::Medium => "MEDIUM",
        Severity::High => "HIGH",
        Severity::Critical => "CRITICAL",
        Severity::Unknown => "UNKNOWN",
    }
}

/// Whether the severity requires immediate analyst action.
#[must_use]
pub const fn requires_escalation(level: Severity) -> bool {
    matches!(level, Severity::High | Severity::Critical)
}

/// Score a single log line with keyword heuristics (uncapped raw score).
#[must_use]
pub fn score_log_line(line: &str) -> u32 {
    let lower = line.to_ascii_lowercase();
    let mut score: u32 = 0;

    for (keyword, points) in [
        ("critical", 40_u32),
        ("malware", 35),
        ("exploit", 30),
        ("denied", 25),
        ("fail", 20),
        ("error", 15),
        ("warning", 10),
        ("unauthorized", 20),
    ] {
        if lower.contains(keyword) {
            score = score.saturating_add(points);
        }
    }

    score
}

/// Cap a raw score to the 0..=100 range used by classifiers.
#[must_use]
pub const fn cap_score(raw: u32) -> u32 {
    if raw > 100 { 100 } else { raw }
}

/// Classify non-empty log lines, stopping after `max_events` counted lines.
///
/// Blank lines are skipped and do not count toward the event budget.
#[must_use]
pub fn classify_batch(lines: &[&str], max_events: usize) -> Vec<(String, Severity)> {
    let mut results = Vec::new();
    let mut events = 0_usize;

    for line in lines {
        if line.trim().is_empty() {
            continue;
        }
        if events >= max_events {
            break;
        }

        let raw = score_log_line(line);
        let capped = cap_score(raw);
        let severity = classify_score(capped);
        results.push((line.to_string(), severity));
        events += 1;
    }

    results
}

/// Collect non-empty lines using a `while let` iterator walk.
#[allow(clippy::while_let_on_iterator)] // intentional teaching demo of while-let
#[must_use]
pub fn walk_nonempty_lines<'a>(lines: &'a [&'a str]) -> Vec<&'a str> {
    let mut collected = Vec::new();
    let mut iter = lines.iter();
    while let Some(line) = iter.next() {
        if !line.trim().is_empty() {
            collected.push(*line);
        }
    }
    collected
}

/// Sum capped scores until a budget is exhausted (while-loop demo).
#[must_use]
pub fn accumulate_until_budget(lines: &[&str], budget: u32) -> u32 {
    let mut used = 0_u32;
    let mut idx = 0_usize;

    while idx < lines.len() {
        let line = lines[idx];
        idx += 1;

        if line.trim().is_empty() {
            continue;
        }

        let capped = cap_score(score_log_line(line));
        if used.saturating_add(capped) > budget {
            break;
        }
        used = used.saturating_add(capped);
    }

    used
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classify_score_ranges() {
        assert_eq!(classify_score(10), Severity::Low);
        assert_eq!(classify_score(30), Severity::Medium);
        assert_eq!(classify_score(60), Severity::High);
        assert_eq!(classify_score(90), Severity::Critical);
        assert_eq!(classify_score(200), Severity::Unknown);
    }

    #[test]
    fn score_log_line_keywords() {
        let quiet = score_log_line("routine heartbeat ok");
        let loud = score_log_line("CRITICAL malware exploit denied");
        assert!(quiet < 30);
        assert!(loud >= 75);
    }

    #[test]
    fn classify_batch_skips_blanks_and_limits() {
        let lines = ["", "warning disk", "", "error auth fail", "info ok"];
        let batch = classify_batch(&lines, 2);
        assert_eq!(batch.len(), 2);
        assert_eq!(batch[0].1, Severity::Low);
    }

    #[test]
    fn walk_nonempty_lines_while_let() {
        let lines = ["", "alert", "  ", "ok"];
        let nonempty = walk_nonempty_lines(&lines);
        assert_eq!(nonempty, vec!["alert", "ok"]);
    }

    #[test]
    fn accumulate_until_budget_stops() {
        let lines = ["critical breach", "malware detected", "info ok"];
        let used = accumulate_until_budget(&lines, 50);
        assert!(used <= 50);
        assert!(used > 0);
    }

    #[test]
    fn severity_labels_and_escalation() {
        assert_eq!(severity_label(Severity::High), "HIGH");
        assert!(requires_escalation(Severity::Critical));
        assert!(!requires_escalation(Severity::Low));
    }
}
