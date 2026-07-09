//! Firewall log lines and iterator-based processing.

/// A parsed access-control log entry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LogLine {
    /// `ALLOW` or `DENY`.
    pub action: String,
    /// Source IP address.
    pub ip: String,
    /// Bytes transferred for allowed traffic.
    pub bytes: u64,
}

impl LogLine {
    /// Build a log line from owned or borrowed string-like values.
    pub fn new(action: impl Into<String>, ip: impl Into<String>, bytes: u64) -> Self {
        Self {
            action: action.into(),
            ip: ip.into(),
            bytes,
        }
    }

    /// Whether this line represents permitted traffic.
    #[must_use]
    pub fn is_allow(&self) -> bool {
        self.action == "ALLOW"
    }

    /// Whether this line represents blocked traffic.
    #[must_use]
    pub fn is_deny(&self) -> bool {
        self.action == "DENY"
    }
}

/// Sum bytes for all `ALLOW` entries (`filter` → `map` → `sum`).
#[must_use]
pub fn allowed_bytes(logs: &[LogLine]) -> u64 {
    logs.iter()
        .filter(|line| line.is_allow())
        .map(|line| line.bytes)
        .sum()
}

/// Collect source IPs from `DENY` entries.
#[must_use]
pub fn denied_ips(logs: &[LogLine]) -> Vec<String> {
    logs.iter()
        .filter(|line| line.is_deny())
        .map(|line| line.ip.clone())
        .collect()
}

/// Count `ALLOW` lines with `filter().count()`.
#[must_use]
pub fn count_allows(logs: &[LogLine]) -> usize {
    logs.iter().filter(|line| line.is_allow()).count()
}

/// Side-effect hook for debugging iterator pipelines via `inspect`.
///
/// Returns `(allow_count, deny_count)` while leaving the original slice untouched.
#[must_use]
pub fn inspect_action_counts(logs: &[LogLine]) -> (usize, usize) {
    let mut allows = 0_usize;
    let mut denies = 0_usize;

    logs.iter()
        .inspect(|line| {
            if line.is_allow() {
                allows += 1;
            } else if line.is_deny() {
                denies += 1;
            }
        })
        .count();

    (allows, denies)
}

/// Sample firewall log for demos and tests.
#[must_use]
pub fn sample_logs() -> Vec<LogLine> {
    vec![
        LogLine::new("ALLOW", "10.0.0.1", 512),
        LogLine::new("DENY", "203.0.113.10", 0),
        LogLine::new("ALLOW", "10.0.0.2", 128),
        LogLine::new("DENY", "198.51.100.5", 0),
        LogLine::new("ALLOW", "10.0.0.1", 256),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn allowed_bytes_sums_allow_only() {
        let logs = sample_logs();
        assert_eq!(allowed_bytes(&logs), 512 + 128 + 256);
    }

    #[test]
    fn denied_ips_collects_deny_sources() {
        let logs = sample_logs();
        assert_eq!(
            denied_ips(&logs),
            vec!["203.0.113.10".to_owned(), "198.51.100.5".to_owned()]
        );
    }

    #[test]
    fn count_allows_uses_filter_count() {
        assert_eq!(count_allows(&sample_logs()), 3);
    }

    #[test]
    fn inspect_action_counts_tracks_both() {
        let (allows, denies) = inspect_action_counts(&sample_logs());
        assert_eq!(allows, 3);
        assert_eq!(denies, 2);
    }

    #[test]
    fn empty_log_yields_zeros() {
        let logs: Vec<LogLine> = vec![];
        assert_eq!(allowed_bytes(&logs), 0);
        assert!(denied_ips(&logs).is_empty());
        assert_eq!(count_allows(&logs), 0);
    }
}
