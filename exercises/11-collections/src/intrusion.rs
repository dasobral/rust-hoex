//! Core intrusion log types and collection-based aggregation.

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

/// Aggregates intrusion events and derives counts, filters, and summaries.
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
    #[must_use]
    pub fn ip_counts(&self) -> HashMap<String, u32> {
        let mut counts: HashMap<String, u32> = HashMap::new();
        for event in &self.events {
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
            users.insert(event.user.clone());
        }
        users
    }

    /// Every distinct action string in the log.
    #[must_use]
    pub fn actions(&self) -> HashSet<String> {
        let mut actions = HashSet::new();
        for event in &self.events {
            actions.insert(event.action.clone());
        }
        actions
    }

    /// IP → hit count map.
    ///
    /// Semantically identical to [`Self::ip_counts`]; provided as a SOC analyst
    /// alias for “hot” repeat-offender tracking dashboards.
    #[must_use]
    pub fn hot_ips(&self) -> HashMap<String, u32> {
        self.ip_counts()
    }

    /// Events whose action equals `action` (case-sensitive).
    #[must_use]
    pub fn filter_by_action(&self, action: &str) -> Vec<LogEvent> {
        self.events
            .iter()
            .filter(|e| e.action == action)
            .cloned()
            .collect()
    }

    /// Username with the highest event count.
    ///
    /// On ties, returns the **lexicographically smallest** username so the
    /// result is deterministic across runs and platforms.
    #[must_use]
    pub fn top_user(&self) -> Option<String> {
        let mut counts: HashMap<String, u32> = HashMap::new();
        for event in &self.events {
            let counter = counts.entry(event.user.clone()).or_insert(0);
            *counter = counter.saturating_add(1);
        }

        counts
            .into_iter()
            .min_by(|(user_a, count_a), (user_b, count_b)| {
                count_b.cmp(count_a).then_with(|| user_a.cmp(user_b))
            })
            .map(|(user, _)| user)
    }

    /// Short human-readable summary line.
    #[must_use]
    pub fn summary(&self) -> String {
        format!(
            "{} events, {} unique users, {} distinct IPs, {} actions",
            self.len(),
            self.unique_users().len(),
            self.ip_counts().len(),
            self.actions().len()
        )
    }
}

/// Build a sample log for demos and tests.
#[must_use]
pub fn sample_log() -> IntrusionLog {
    let mut log = IntrusionLog::new();
    log.push(LogEvent::new("203.0.113.10", "alice", "login_fail"));
    log.push(LogEvent::new("203.0.113.10", "alice", "login_fail"));
    log.push(LogEvent::new("198.51.100.7", "bob", "login_ok"));
    log.push(LogEvent::new("203.0.113.10", "carol", "login_fail"));
    log.push(LogEvent::new("198.51.100.7", "bob", "file_access"));
    log
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn push_and_len() {
        let log = sample_log();
        assert_eq!(log.len(), 5);
        assert!(!log.is_empty());
    }

    #[test]
    fn ip_counts_and_hot_ips_match() {
        let log = sample_log();
        assert_eq!(log.ip_counts(), log.hot_ips());
        assert_eq!(log.ip_counts().get("203.0.113.10"), Some(&3));
    }

    #[test]
    fn actions_collects_distinct() {
        let actions = sample_log().actions();
        assert_eq!(actions.len(), 3);
        assert!(actions.contains("login_fail"));
        assert!(actions.contains("login_ok"));
        assert!(actions.contains("file_access"));
    }

    #[test]
    fn top_user_picks_max_lexicographic_tiebreak() {
        let log = sample_log();
        assert_eq!(log.top_user().as_deref(), Some("alice"));

        let mut tie = IntrusionLog::new();
        tie.push(LogEvent::new("10.0.0.1", "zara", "scan"));
        tie.push(LogEvent::new("10.0.0.2", "amy", "scan"));
        assert_eq!(tie.top_user().as_deref(), Some("amy"));
    }

    #[test]
    fn filter_by_action() {
        let fails = sample_log().filter_by_action("login_fail");
        assert_eq!(fails.len(), 3);
        assert!(fails.iter().all(|e| e.action == "login_fail"));
    }
}
