//! Closures: anonymous functions that can capture their environment.
//!
//! # When closures shine
//!
//! - One-off logic passed to `sort_by`, `filter`, `map`, …
//! - Capturing local thresholds / config without a new named `fn`
//! - Short adapters where a full function would be noise
//!
//! Prefer a named `fn` when the logic is reused, needs a clear name, or
//! should not capture (easier to test in isolation).
//!
//! # `Fn` / `FnMut` / `FnOnce` (briefly)
//!
//! | Trait    | Can call…        | Captures…                          |
//! |----------|------------------|------------------------------------|
//! | `Fn`     | many times       | by shared reference (`&T`)         |
//! | `FnMut`  | many times       | by mutable reference (`&mut T`)    |
//! | `FnOnce` | at most once     | by value (may move out)            |
//!
//! Closures implement the most permissive trait(s) their body allows.
//! `move` forces captures by value (useful for threads / `'static` bounds).

/// Severity of a detected threat event.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Severity {
    Low = 1,
    Medium = 2,
    High = 3,
    Critical = 4,
}

/// A security / threat event waiting to be triaged.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ThreatEvent {
    /// Short label (e.g. `"brute-force"`).
    pub kind: String,
    /// Where it was observed.
    pub source: String,
    pub severity: Severity,
    /// Arbitrary score used for custom ordering demos.
    pub score: u32,
}

impl ThreatEvent {
    /// Construct an event from borrowed pieces.
    #[must_use]
    pub fn new(kind: &str, source: &str, severity: Severity, score: u32) -> Self {
        Self {
            kind: kind.to_owned(),
            source: source.to_owned(),
            severity,
            score,
        }
    }
}

/// Sort events: higher severity first, then higher score (custom `sort_by`).
pub fn sort_threats(events: &mut [ThreatEvent]) {
    events.sort_by(|a, b| {
        // Closure captures nothing — still clearer inline than a free fn here.
        b.severity
            .cmp(&a.severity)
            .then_with(|| b.score.cmp(&a.score))
    });
}

/// Keep events at or above `min` severity (`filter` + closure capturing `min`).
#[must_use]
pub fn filter_by_severity(events: &[ThreatEvent], min: Severity) -> Vec<&ThreatEvent> {
    events
        .iter()
        .filter(|event| event.severity >= min)
        .collect()
}

/// Map events to display labels with a captured prefix string.
#[must_use]
pub fn label_events(events: &[ThreatEvent], prefix: &str) -> Vec<String> {
    // `prefix` is borrowed by the closure (`Fn`).
    events
        .iter()
        .map(|event| format!("{prefix}:{}[{}]", event.kind, event.score))
        .collect()
}

/// Count how many events match a caller-supplied predicate (`Fn`).
pub fn count_matching<F>(events: &[ThreatEvent], mut predicate: F) -> usize
where
    F: FnMut(&ThreatEvent) -> bool,
{
    events.iter().filter(|e| predicate(e)).count()
}

/// Drain events into a summary, capturing a mutable counter (`FnMut`).
pub fn summarize_with_counter(events: &[ThreatEvent]) -> (usize, String) {
    let mut seen = 0_usize;
    let parts: Vec<String> = events
        .iter()
        .map(|event| {
            seen += 1;
            format!("{seen}.{}", event.kind)
        })
        .collect();
    (seen, parts.join(", "))
}

/// Consume an owned event list with a `move` closure (`FnOnce`-style capture).
///
/// The closure takes ownership of `tag` so it can be returned inside the
/// string without borrowing the caller's stack frame.
#[must_use]
pub fn consume_with_tag(events: Vec<ThreatEvent>, tag: String) -> String {
    let finalize = move |list: Vec<ThreatEvent>| {
        let kinds: Vec<_> = list.into_iter().map(|e| e.kind).collect();
        format!("{tag} => {}", kinds.join("+"))
    };
    finalize(events)
}

/// Apply a one-shot closure to a single event (`FnOnce`).
pub fn with_event_once<F, R>(event: ThreatEvent, f: F) -> R
where
    F: FnOnce(ThreatEvent) -> R,
{
    f(event)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> Vec<ThreatEvent> {
        vec![
            ThreatEvent::new("scan", "10.0.0.1", Severity::Low, 10),
            ThreatEvent::new("exfil", "10.0.0.2", Severity::Critical, 90),
            ThreatEvent::new("brute-force", "10.0.0.3", Severity::High, 50),
            ThreatEvent::new("malware", "10.0.0.2", Severity::High, 80),
        ]
    }

    #[test]
    fn sort_threats_orders_by_severity_then_score() {
        let mut events = sample();
        sort_threats(&mut events);
        assert_eq!(events[0].kind, "exfil");
        assert_eq!(events[1].kind, "malware");
        assert_eq!(events[2].kind, "brute-force");
    }

    #[test]
    fn filter_by_severity_captures_threshold() {
        let events = sample();
        let high = filter_by_severity(&events, Severity::High);
        assert_eq!(high.len(), 3);
        assert!(high.iter().all(|e| e.severity >= Severity::High));
    }

    #[test]
    fn label_events_captures_prefix() {
        let labels = label_events(&sample(), "ALERT");
        assert!(labels[0].starts_with("ALERT:"));
    }

    #[test]
    fn count_matching_accepts_closure() {
        let n = count_matching(&sample(), |e| e.source == "10.0.0.2");
        assert_eq!(n, 2);
    }

    #[test]
    fn summarize_with_counter_is_fn_mut() {
        let (n, text) = summarize_with_counter(&sample());
        assert_eq!(n, 4);
        assert!(text.starts_with("1.scan"));
    }

    #[test]
    fn consume_with_tag_moves_capture() {
        let out = consume_with_tag(sample(), "triage".to_owned());
        assert!(out.starts_with("triage => "));
        assert!(out.contains("exfil"));
    }

    #[test]
    fn with_event_once_fn_once() {
        let event = ThreatEvent::new("phish", "8.8.8.8", Severity::Medium, 40);
        let kind = with_event_once(event, |e| e.kind);
        assert_eq!(kind, "phish");
    }
}
