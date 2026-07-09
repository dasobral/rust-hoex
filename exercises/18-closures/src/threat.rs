//! Threat events and closure-based triage helpers.

use std::collections::HashSet;

/// A scored security event from a monitored source.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ThreatEvent {
    /// Observed source (IP or hostname).
    pub source: String,
    /// Numeric risk score (higher = more severe).
    pub score: u32,
    /// Whether the event requires immediate escalation.
    pub critical: bool,
}

impl ThreatEvent {
    /// Build an event from owned or borrowed string-like values.
    pub fn new(source: impl Into<String>, score: u32, critical: bool) -> Self {
        Self {
            source: source.into(),
            score,
            critical,
        }
    }
}

/// Sort events by `source` ascending (`sort_by_key`).
pub fn sort_by_source(events: &mut [ThreatEvent]) {
    events.sort_by_key(|event| event.source.clone());
}

/// Split owned events into `(critical, non_critical)` via `into_iter().partition`.
#[must_use]
pub fn partition_critical(events: Vec<ThreatEvent>) -> (Vec<ThreatEvent>, Vec<ThreatEvent>) {
    events.into_iter().partition(|event| event.critical)
}

/// Count events matching a caller-supplied predicate.
#[must_use]
pub fn count_matching(events: &[ThreatEvent], pred: impl Fn(&ThreatEvent) -> bool) -> usize {
    events.iter().filter(|event| pred(event)).count()
}

/// Count events whose source appears in `watchlist` (closure captures `HashSet`).
#[must_use]
pub fn count_watchlisted<S: std::hash::BuildHasher>(
    events: &[ThreatEvent],
    watchlist: &HashSet<String, S>,
) -> usize {
    let is_listed = |event: &ThreatEvent| watchlist.contains(&event.source);
    count_matching(events, is_listed)
}

/// Sample threat feed for demos and tests.
#[must_use]
pub fn sample_events() -> Vec<ThreatEvent> {
    vec![
        ThreatEvent::new("203.0.113.10", 85, true),
        ThreatEvent::new("10.0.0.5", 20, false),
        ThreatEvent::new("198.51.100.1", 55, false),
        ThreatEvent::new("203.0.113.10", 90, true),
        ThreatEvent::new("10.0.0.9", 40, false),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sort_by_source_orders_lexicographically() {
        let mut events = sample_events();
        sort_by_source(&mut events);
        assert_eq!(events[0].source, "10.0.0.5");
        assert_eq!(
            events.last().map(|e| e.source.as_str()),
            Some("203.0.113.10")
        );
    }

    #[test]
    fn partition_critical_splits_owned_vec() {
        let events = sample_events();
        let (critical, normal) = partition_critical(events);
        assert_eq!(critical.len(), 2);
        assert_eq!(normal.len(), 3);
        assert!(critical.iter().all(|e| e.critical));
        assert!(normal.iter().all(|e| !e.critical));
    }

    #[test]
    fn count_matching_accepts_predicate() {
        let events = sample_events();
        let high = count_matching(&events, |e| e.score >= 80);
        assert_eq!(high, 2);
    }

    #[test]
    fn count_watchlisted_captures_hashset() {
        let events = sample_events();
        let mut watchlist = HashSet::new();
        watchlist.insert("203.0.113.10".to_owned());
        watchlist.insert("10.0.0.5".to_owned());
        assert_eq!(count_watchlisted(&events, &watchlist), 3);
    }
}
