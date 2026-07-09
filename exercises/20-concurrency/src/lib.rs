//! Parallel log processing with threads and `mpsc` channels.
//!
//! Workers parse log lines, filter by severity, and send hits through a channel.
//! The consumer collects results; workers stop early when the receiver drops.

use std::sync::mpsc::{self, RecvError, SendError};
use std::thread;

/// A parsed log line that matched the configured filter.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct LogHit {
    /// Worker index that produced this hit.
    pub worker: usize,
    /// Severity keyword (`INFO`, `WARN`, `ERROR`, …).
    pub level: String,
    /// Original line (trimmed).
    pub line: String,
}

/// Errors from the parallel pipeline (no panics in library code).
#[derive(Debug, PartialEq, Eq)]
pub enum PipelineError {
    /// `send` failed because the receiver was dropped.
    Send(String),
    /// `recv` failed because all senders were dropped before a message.
    Recv,
    /// A worker thread panicked.
    WorkerPanic(String),
}

impl From<RecvError> for PipelineError {
    fn from(_: RecvError) -> Self {
        Self::Recv
    }
}

impl<T> From<SendError<T>> for PipelineError {
    fn from(err: SendError<T>) -> Self {
        Self::Send(err.to_string())
    }
}

/// Extract the severity keyword from a log line, if present.
#[must_use]
pub fn parse_level(line: &str) -> Option<&str> {
    let trimmed = line.trim();
    if trimmed.contains("ERROR") {
        Some("ERROR")
    } else if trimmed.contains("WARN") {
        Some("WARN")
    } else if trimmed.contains("INFO") {
        Some("INFO")
    } else {
        None
    }
}

/// Returns `true` when `line` matches `filter_level`.
#[must_use]
pub fn matches_filter(line: &str, filter_level: &str) -> bool {
    parse_level(line).is_some_and(|level| level == filter_level)
}

/// Process `lines` with `worker_count` threads; only lines matching `filter_level`
/// are sent through the channel.
///
/// Each worker owns a cloned `Sender`. Workers stop sending when `send` fails
/// (receiver dropped). Join handles are matched — a panicked worker becomes
/// `PipelineError`.
#[allow(clippy::needless_pass_by_value)] // owned chunks are moved into threads
pub fn process_logs(
    lines: Vec<String>,
    filter_level: &str,
    worker_count: usize,
) -> Result<Vec<LogHit>, PipelineError> {
    let workers = worker_count.max(1);
    let (tx, rx) = mpsc::channel::<LogHit>();

    let chunk_size = lines.len().div_ceil(workers).max(1);
    let chunks: Vec<Vec<String>> = lines.chunks(chunk_size).map(<[String]>::to_vec).collect();

    let mut handles = Vec::with_capacity(chunks.len());

    for (worker_id, chunk) in chunks.into_iter().enumerate() {
        let tx = tx.clone();
        let filter = filter_level.to_owned();
        let handle = thread::spawn(move || -> Result<usize, PipelineError> {
            let mut produced = 0_usize;
            for line in chunk {
                if !matches_filter(&line, &filter) {
                    continue;
                }
                let level = parse_level(&line).map_or_else(|| filter.clone(), str::to_owned);
                let hit = LogHit {
                    worker: worker_id,
                    level,
                    line: line.trim().to_owned(),
                };
                // Stop early when the consumer disconnects.
                if tx.send(hit).is_err() {
                    break;
                }
                produced = produced.saturating_add(1);
            }
            Ok(produced)
        });
        handles.push(handle);
    }

    drop(tx);

    let mut hits = Vec::new();
    while let Ok(hit) = rx.recv() {
        hits.push(hit);
    }

    for handle in handles {
        match handle.join() {
            Ok(Ok(_)) => {}
            Ok(Err(err)) => return Err(err),
            Err(payload) => {
                let msg = payload
                    .downcast_ref::<&str>()
                    .map(|s| (*s).to_owned())
                    .or_else(|| payload.downcast_ref::<String>().cloned())
                    .unwrap_or_else(|| "worker panicked".to_owned());
                return Err(PipelineError::WorkerPanic(msg));
            }
        }
    }

    hits.sort_by(|a, b| a.line.cmp(&b.line));
    Ok(hits)
}

/// Collect hits until `limit` messages are received, then drop the receiver so
/// workers observe a disconnected channel and stop via `send().is_err()`.
#[allow(clippy::needless_pass_by_value)] // owned chunks are moved into threads
#[must_use]
pub fn collect_with_limit(
    lines: Vec<String>,
    filter_level: &str,
    worker_count: usize,
    limit: usize,
) -> Vec<LogHit> {
    let workers = worker_count.max(1);
    let (tx, rx) = mpsc::channel::<LogHit>();

    let chunk_size = lines.len().div_ceil(workers).max(1);
    let chunks: Vec<Vec<String>> = lines.chunks(chunk_size).map(<[String]>::to_vec).collect();

    let mut handles = Vec::with_capacity(chunks.len());

    for (worker_id, chunk) in chunks.into_iter().enumerate() {
        let tx = tx.clone();
        let filter = filter_level.to_owned();
        let handle = thread::spawn(move || {
            for line in chunk {
                if !matches_filter(&line, &filter) {
                    continue;
                }
                let level = parse_level(&line).map_or_else(|| filter.clone(), str::to_owned);
                let hit = LogHit {
                    worker: worker_id,
                    level,
                    line: line.trim().to_owned(),
                };
                if tx.send(hit).is_err() {
                    break;
                }
            }
        });
        handles.push(handle);
    }

    drop(tx);

    let mut hits = Vec::new();
    for hit in rx.iter().take(limit) {
        hits.push(hit);
    }
    drop(rx);

    for handle in handles {
        let _ = handle.join();
    }

    hits.sort_by(|a, b| a.line.cmp(&b.line));
    hits
}

/// Count hits grouped by severity level.
#[must_use]
pub fn count_by_level(hits: &[LogHit]) -> Vec<(String, usize)> {
    let mut levels: Vec<(String, usize)> = Vec::new();
    for hit in hits {
        match levels.iter_mut().find(|(name, _)| name == &hit.level) {
            Some((_, n)) => *n = n.saturating_add(1),
            None => levels.push((hit.level.clone(), 1)),
        }
    }
    levels.sort_by(|a, b| a.0.cmp(&b.0));
    levels
}

/// Run the log-processing demo with sample lines.
pub fn run_demo(filter_level: &str, worker_count: usize, verbose: bool) -> anyhow::Result<()> {
    let lines = vec![
        "INFO  service started".into(),
        "WARN  high latency".into(),
        "ERROR auth failed".into(),
        "INFO  request ok".into(),
        "WARN  retrying".into(),
        "ERROR disk full".into(),
        "INFO  shutdown".into(),
        "ERROR connection reset".into(),
    ];

    if verbose {
        println!(
            "filter={filter_level}, workers={worker_count}, lines={}",
            lines.len()
        );
    }

    let hits = process_logs(lines, filter_level, worker_count)
        .map_err(|err| anyhow::anyhow!("pipeline failed: {err:?}"))?;
    println!("matched {} hit(s):", hits.len());
    for hit in &hits {
        println!("  [worker {}] {:>5}  {}", hit.worker, hit.level, hit.line);
    }
    for (level, n) in count_by_level(&hits) {
        println!("  {level}: {n}");
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]

    use super::*;

    #[test]
    fn parse_level_detects_keywords() {
        assert_eq!(parse_level("ERROR disk full"), Some("ERROR"));
        assert_eq!(parse_level("WARN slow"), Some("WARN"));
        assert_eq!(parse_level("hello"), None);
    }

    #[test]
    fn matches_filter_only_target_level() {
        assert!(matches_filter("ERROR x", "ERROR"));
        assert!(!matches_filter("WARN x", "ERROR"));
    }

    #[test]
    fn process_logs_filters_errors_only() {
        let lines = vec![
            "INFO boot".into(),
            "WARN temp".into(),
            "ERROR fail".into(),
            "ERROR disk".into(),
            "INFO ok".into(),
        ];
        let hits = process_logs(lines, "ERROR", 2).unwrap();
        assert_eq!(hits.len(), 2);
        assert!(hits.iter().all(|h| h.level == "ERROR"));
        assert_eq!(
            hits.iter().map(|h| h.line.as_str()).collect::<Vec<_>>(),
            vec!["ERROR disk", "ERROR fail"]
        );
    }

    #[test]
    fn process_logs_configurable_warn_filter() {
        let lines = vec!["WARN a".into(), "ERROR b".into(), "WARN c".into()];
        let hits = process_logs(lines, "WARN", 1).unwrap();
        assert_eq!(hits.len(), 2);
        assert!(hits.iter().all(|h| h.level == "WARN"));
    }

    #[test]
    fn collect_with_limit_stops_workers_early() {
        let lines = vec![
            "ERROR one".into(),
            "ERROR two".into(),
            "ERROR three".into(),
            "ERROR four".into(),
        ];
        let hits = collect_with_limit(lines, "ERROR", 2, 2);
        assert_eq!(hits.len(), 2);
    }
}
