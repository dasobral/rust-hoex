//! Iterator adapters: process log lines and IP addresses with lazy chains.
//!
//! # Consuming vs adapting
//!
//! - **Adapters** (`map`, `filter`, `enumerate`, `take`, …) return a new
//!   iterator; they are lazy — nothing runs until you consume.
//! - **Consumers** (`collect`, `fold`, `count`, `for`, …) drive the iterator
//!   to produce a final value.
//!
//! # `into_iter` vs `iter` vs `iter_mut`
//!
//! | Method       | Yields     | Ownership                          |
//! |--------------|------------|------------------------------------|
//! | `into_iter()`| `T`        | Consumes the collection            |
//! | `iter()`     | `&T`       | Shared borrows                     |
//! | `iter_mut()` | `&mut T`   | Exclusive mutable borrows          |

/// A single parsed firewall / access log line.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LogLine {
    /// Source IP as text (e.g. `"10.0.0.5"`).
    pub src: String,
    /// Action token: `ALLOW` or `DENY`.
    pub action: String,
    /// Bytes transferred (0 if absent).
    pub bytes: u64,
}

impl LogLine {
    /// Parse `src=… action=… bytes=…` style lines. Unknown keys are ignored.
    #[must_use]
    pub fn parse(line: &str) -> Option<Self> {
        let mut src = None;
        let mut action = None;
        let mut bytes = 0_u64;

        for part in line.split_whitespace() {
            if let Some(v) = part.strip_prefix("src=") {
                src = Some(v.to_owned());
            } else if let Some(v) = part.strip_prefix("action=") {
                action = Some(v.to_owned());
            } else if let Some(v) = part.strip_prefix("bytes=") {
                bytes = v.parse().unwrap_or(0);
            }
        }

        Some(Self {
            src: src?,
            action: action?,
            bytes,
        })
    }
}

/// Parse many raw lines, skipping blanks and malformed entries.
#[must_use]
pub fn parse_logs(lines: &[&str]) -> Vec<LogLine> {
    lines
        .iter()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .filter_map(LogLine::parse)
        .collect()
}

/// Source IPs for `DENY` actions only (adapter chain + `collect`).
#[must_use]
pub fn denied_ips(logs: &[LogLine]) -> Vec<&str> {
    logs.iter()
        .filter(|log| log.action == "DENY")
        .map(|log| log.src.as_str())
        .collect()
}

/// Sum of `bytes` across all lines (`fold` as an explicit consumer).
#[must_use]
pub fn total_bytes(logs: &[LogLine]) -> u64 {
    logs.iter().fold(0_u64, |acc, log| acc + log.bytes)
}

/// First `n` unique source IPs in encounter order.
#[must_use]
pub fn first_n_sources(logs: &[LogLine], n: usize) -> Vec<&str> {
    let mut seen = Vec::new();
    logs.iter()
        .map(|log| log.src.as_str())
        .filter(|ip| {
            if seen.contains(ip) {
                false
            } else {
                seen.push(*ip);
                true
            }
        })
        .take(n)
        .collect()
}

/// Numbered summaries: `(index, "src action")` via `enumerate`.
#[must_use]
pub fn numbered_summaries(logs: &[LogLine]) -> Vec<(usize, String)> {
    logs.iter()
        .enumerate()
        .map(|(i, log)| (i, format!("{} {}", log.src, log.action)))
        .collect()
}

/// Demonstrate `iter_mut`: uppercase every action in place.
pub fn uppercase_actions(logs: &mut [LogLine]) {
    for log in logs.iter_mut() {
        log.action = log.action.to_ascii_uppercase();
    }
}

/// Consume owned logs into a single owned report string (`into_iter`).
#[must_use]
pub fn consume_into_report(logs: Vec<LogLine>) -> String {
    logs.into_iter()
        .map(|log| format!("{}:{}:{}", log.src, log.action, log.bytes))
        .collect::<Vec<_>>()
        .join(" | ")
}

/// Manual `next` loop — what `for` desugars toward.
#[must_use]
pub fn count_allows_with_next(logs: &[LogLine]) -> usize {
    let mut iter = logs.iter().filter(|log| log.action == "ALLOW");
    let mut count = 0;
    while iter.next().is_some() {
        count += 1;
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> Vec<LogLine> {
        parse_logs(&[
            "src=10.0.0.1 action=ALLOW bytes=100",
            "src=10.0.0.2 action=DENY bytes=0",
            "src=10.0.0.1 action=DENY bytes=50",
            "",
            "not-a-log",
            "src=10.0.0.3 action=ALLOW bytes=20",
        ])
    }

    #[test]
    fn parse_logs_skips_junk() {
        let logs = sample();
        assert_eq!(logs.len(), 4);
    }

    #[test]
    fn denied_ips_filters() {
        let logs = sample();
        assert_eq!(denied_ips(&logs), vec!["10.0.0.2", "10.0.0.1"]);
    }

    #[test]
    fn total_bytes_folds() {
        assert_eq!(total_bytes(&sample()), 170);
    }

    #[test]
    fn first_n_sources_unique() {
        assert_eq!(first_n_sources(&sample(), 2), vec!["10.0.0.1", "10.0.0.2"]);
    }

    #[test]
    fn numbered_summaries_enumerate() {
        let s = numbered_summaries(&sample());
        assert_eq!(s[0], (0, "10.0.0.1 ALLOW".to_owned()));
    }

    #[test]
    fn uppercase_actions_uses_iter_mut() {
        let mut logs = sample();
        for log in &mut logs {
            log.action = log.action.to_ascii_lowercase();
        }
        uppercase_actions(&mut logs);
        assert!(
            logs.iter()
                .all(|l| l.action == "ALLOW" || l.action == "DENY")
        );
    }

    #[test]
    fn consume_into_report_takes_ownership() {
        let logs = sample();
        let report = consume_into_report(logs);
        assert!(report.contains("10.0.0.1:ALLOW:100"));
    }

    #[test]
    fn count_allows_manual_next_loop() {
        assert_eq!(count_allows_with_next(&sample()), 2);
    }
}
