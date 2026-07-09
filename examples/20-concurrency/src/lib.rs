//! Threads and channels: parallel log-line processing.
//!
//! # What you will see
//!
//! - `thread::spawn` + `move` closures to own captured data
//! - `join` — always handle `Result` (a panicked worker yields `Err`)
//! - `mpsc::channel` — multiple producers, one consumer
//! - why **`Send`** / **`Sync`** matter (briefly below)
//!
//! # `Send` and `Sync` (one paragraph)
//!
//! - **`Send`**: ownership of the value may move to another thread.
//! - **`Sync`**: shared references (`&T`) may be used from another thread
//!   (`T: Sync` iff `&T: Send`).
//!
//! `mpsc::Sender` is `Send + Clone`, so workers can each own a sender.
//! `Rc` / `RefCell` are **not** `Send`/`Sync` — use `Arc` / `Mutex` instead
//! when sharing across threads (see example 19).

use std::sync::mpsc::{self, RecvError, SendError};
use std::thread;
use std::time::Duration;

/// A parsed summary of one log line.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LogHit {
    /// Worker that produced this hit.
    pub worker: usize,
    /// Severity keyword found in the line (`INFO` / `WARN` / `ERROR`).
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

/// Classify a log line into a level keyword.
#[must_use]
pub fn classify_level(line: &str) -> &str {
    if line.contains("ERROR") {
        "ERROR"
    } else if line.contains("WARN") {
        "WARN"
    } else if line.contains("INFO") {
        "INFO"
    } else {
        "OTHER"
    }
}

/// Process `lines` with `worker_count` threads; collect hits via `mpsc`.
///
/// Each worker owns a cloned `Sender`. The main thread is the sole receiver.
/// Join handles are matched — a panicked worker becomes `PipelineError`.
///
/// Takes ownership of `lines` so chunks can be `move`d into worker threads.
#[allow(clippy::needless_pass_by_value)] // owned chunks are moved into threads
pub fn process_logs(lines: Vec<String>, worker_count: usize) -> Result<Vec<LogHit>, PipelineError> {
    let workers = worker_count.max(1);
    let (tx, rx) = mpsc::channel::<LogHit>();

    // Split lines into roughly equal chunks (last chunk may be larger).
    let chunk_size = lines.len().div_ceil(workers).max(1);
    let chunks: Vec<Vec<String>> = lines.chunks(chunk_size).map(<[String]>::to_vec).collect();

    let mut handles = Vec::with_capacity(chunks.len());

    for (worker_id, chunk) in chunks.into_iter().enumerate() {
        let tx = tx.clone();
        // `move` transfers ownership of `chunk` and `tx` into the thread.
        let handle = thread::spawn(move || -> Result<usize, PipelineError> {
            let mut produced = 0_usize;
            for line in chunk {
                let level = classify_level(&line).to_owned();
                let hit = LogHit {
                    worker: worker_id,
                    level,
                    line,
                };
                tx.send(hit)?;
                produced = produced.saturating_add(1);
                // Tiny yield so the demo interleaves workers visibly.
                thread::sleep(Duration::from_millis(1));
            }
            Ok(produced)
        });
        handles.push(handle);
    }

    // Drop the original sender so `rx` can finish when workers are done.
    drop(tx);

    let mut hits = Vec::new();
    // Drain until all senders are gone (`RecvError`).
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

    Ok(hits)
}

/// Count hits per level keyword.
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

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]

    use super::*;

    #[test]
    fn classify_level_detects_keywords() {
        assert_eq!(classify_level("ERROR disk full"), "ERROR");
        assert_eq!(classify_level("WARN slow"), "WARN");
        assert_eq!(classify_level("hello"), "OTHER");
    }

    #[test]
    fn process_logs_uses_multiple_workers() {
        let lines = vec![
            "INFO boot".into(),
            "WARN temp".into(),
            "ERROR fail".into(),
            "INFO ok".into(),
        ];
        let hits = process_logs(lines, 2).unwrap();
        assert_eq!(hits.len(), 4);
        let workers: std::collections::BTreeSet<_> = hits.iter().map(|h| h.worker).collect();
        assert!(workers.len() >= 2);
    }

    #[test]
    fn count_by_level_groups() {
        let hits = vec![
            LogHit {
                worker: 0,
                level: "INFO".into(),
                line: "a".into(),
            },
            LogHit {
                worker: 1,
                level: "INFO".into(),
                line: "b".into(),
            },
            LogHit {
                worker: 0,
                level: "ERROR".into(),
                line: "c".into(),
            },
        ];
        let counts = count_by_level(&hits);
        assert_eq!(counts, vec![("ERROR".into(), 1), ("INFO".into(), 2)]);
    }
}
