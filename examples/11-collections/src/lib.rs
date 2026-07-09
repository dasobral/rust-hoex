//! Intrusion log aggregator built on `Vec`, `HashMap`, and `HashSet`.
//!
//! # What you will see
//!
//! - `Vec` for an ordered event stream (`push`, iterate, filter)
//! - `HashMap` for IP → hit counts (`entry` API, lookup)
//! - `HashSet` for unique usernames
//! - Ownership: collections own their elements; iterators borrow them

use std::collections::{HashMap, HashSet};

/// A single intrusion / auth-related log event.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LogEvent {
    pub ip: String,
    pub user: String,
    pub action: String,
}

impl LogEvent {
    /// Build an event from owned or borrowed string-like values.
    pub fn new(ip: impl Into<String>, user: impl Into<String>, action: impl Into<String>) -> Self {
        Self {
            ip: ip.into(),
            user: user.into(),
            action: action.into(),
        }
    }
}

/// Aggregates intrusion events and derives IP counts + unique users.
///
/// The `events` field is a `Vec` — ordered, growable, owns each `LogEvent`.
#[derive(Debug, Default, Clone)]
pub struct IntrusionLog {
    events: Vec<LogEvent>,
}

impl IntrusionLog {
    /// Empty aggregator.
    #[must_use]
    pub const fn new() -> Self {
        Self { events: Vec::new() }
    }

    /// Append an event (takes ownership of `event`).
    pub fn push(&mut self, event: LogEvent) {
        self.events.push(event);
    }

    /// Number of stored events.
    #[must_use]
    pub const fn len(&self) -> usize {
        self.events.len()
    }

    /// Whether the log is empty.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.events.is_empty()
    }

    /// Borrow the underlying event slice.
    #[must_use]
    pub fn events(&self) -> &[LogEvent] {
        &self.events
    }

    /// Count how many times each source IP appears.
    ///
    /// Uses the `HashMap` **entry API**: `or_insert(0)` inserts a zero on first
    /// sight, then we increment in place.
    #[must_use]
    pub fn ip_counts(&self) -> HashMap<String, u32> {
        let mut counts: HashMap<String, u32> = HashMap::new();
        for event in &self.events {
            // `entry` returns Occupied or Vacant; `or_insert` yields `&mut u32`.
            let counter = counts.entry(event.ip.clone()).or_insert(0);
            *counter = counter.saturating_add(1);
        }
        counts
    }

    /// Unique usernames observed in the log (`HashSet` deduplicates).
    #[must_use]
    pub fn unique_users(&self) -> HashSet<String> {
        let mut users = HashSet::new();
        for event in &self.events {
            // `insert` returns `false` if the value was already present.
            users.insert(event.user.clone());
        }
        users
    }

    /// Events whose action equals `action` (case-sensitive).
    ///
    /// Returns owned clones so the caller can keep them after the log moves.
    #[must_use]
    pub fn filter_by_action(&self, action: &str) -> Vec<LogEvent> {
        self.events
            .iter()
            .filter(|e| e.action == action)
            .cloned()
            .collect()
    }

    /// IPs that appear at least `threshold` times (suspicious repeat offenders).
    #[must_use]
    pub fn hot_ips(&self, threshold: u32) -> Vec<String> {
        let mut hot: Vec<String> = self
            .ip_counts()
            .into_iter()
            .filter(|(_, count)| *count >= threshold)
            .map(|(ip, _)| ip)
            .collect();
        hot.sort();
        hot
    }

    /// Short human-readable summary line.
    #[must_use]
    pub fn summary(&self) -> String {
        format!(
            "{} events, {} unique users, {} distinct IPs",
            self.len(),
            self.unique_users().len(),
            self.ip_counts().len()
        )
    }
}

/// Merge two logs: consume both and return a combined `IntrusionLog`.
///
/// Demonstrates moving ownership of `Vec` contents with `extend`.
#[must_use]
pub fn merge_logs(mut left: IntrusionLog, right: IntrusionLog) -> IntrusionLog {
    left.events.extend(right.events);
    left
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_log() -> IntrusionLog {
        let mut log = IntrusionLog::new();
        log.push(LogEvent::new("10.0.0.1", "alice", "login_fail"));
        log.push(LogEvent::new("10.0.0.1", "alice", "login_fail"));
        log.push(LogEvent::new("10.0.0.2", "bob", "login_ok"));
        log.push(LogEvent::new("10.0.0.1", "carol", "login_fail"));
        log
    }

    #[test]
    fn push_and_len() {
        let log = sample_log();
        assert_eq!(log.len(), 4);
        assert!(!log.is_empty());
    }

    #[test]
    fn ip_counts_use_entry_api() {
        let counts = sample_log().ip_counts();
        assert_eq!(counts.get("10.0.0.1"), Some(&3));
        assert_eq!(counts.get("10.0.0.2"), Some(&1));
        assert_eq!(counts.get("9.9.9.9"), None);
    }

    #[test]
    fn unique_users_dedup() {
        let users = sample_log().unique_users();
        assert_eq!(users.len(), 3);
        assert!(users.contains("alice"));
        assert!(users.contains("bob"));
        assert!(users.contains("carol"));
    }

    #[test]
    fn filter_by_action() {
        let fails = sample_log().filter_by_action("login_fail");
        assert_eq!(fails.len(), 3);
        assert!(fails.iter().all(|e| e.action == "login_fail"));
    }

    #[test]
    fn hot_ips_threshold() {
        let hot = sample_log().hot_ips(3);
        assert_eq!(hot, vec!["10.0.0.1".to_owned()]);
    }

    #[test]
    fn merge_logs_combines_events() {
        let a = sample_log();
        let mut b = IntrusionLog::new();
        b.push(LogEvent::new("10.0.0.9", "dave", "scan"));
        let merged = merge_logs(a, b);
        assert_eq!(merged.len(), 5);
        assert!(merged.unique_users().contains("dave"));
    }

    #[test]
    fn summary_mentions_counts() {
        let s = sample_log().summary();
        assert!(s.contains("4 events"));
        assert!(s.contains("3 unique users"));
    }
}
