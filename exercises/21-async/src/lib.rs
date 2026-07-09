//! Async/await with Tokio — offline simulated threat-intel fetches.
//!
//! Concepts: `async fn`, `.await`, `tokio::time::timeout`, `join!`, and
//! parallel `join_all`-style collection.

use std::time::Duration;

use tokio::time::{sleep, timeout};

/// Errors from async fetch helpers (library code never panics).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FetchError {
    /// Request exceeded the configured deadline.
    TimedOut,
    /// Simulated upstream failure for id 0.
    NotFound,
}

/// Simulate fetching threat intel for `id` after `delay_ms` of async sleep.
///
/// Id `0` is treated as not found; all other ids return a deterministic label.
pub async fn fetch_threat(id: u32, delay_ms: u64) -> Result<String, FetchError> {
    sleep(Duration::from_millis(delay_ms)).await;
    if id == 0 {
        return Err(FetchError::NotFound);
    }
    Ok(format!("threat-{id}"))
}

/// Wrap `fetch_threat` with a wall-clock timeout.
pub async fn fetch_threat_with_timeout(
    id: u32,
    delay_ms: u64,
    timeout_ms: u64,
) -> Result<String, FetchError> {
    match timeout(
        Duration::from_millis(timeout_ms),
        fetch_threat(id, delay_ms),
    )
    .await
    {
        Ok(result) => result,
        Err(_elapsed) => Err(FetchError::TimedOut),
    }
}

/// Fetch ids one after another (latency sums).
pub async fn fetch_sequential(ids: &[u32], delay_ms: u64) -> Vec<Result<String, FetchError>> {
    let mut out = Vec::with_capacity(ids.len());
    for &id in ids {
        out.push(fetch_threat(id, delay_ms).await);
    }
    out
}

/// Fetch ids concurrently with `tokio::join!` when there are 2–3 ids; otherwise
/// spawn one task per id and collect (join-all style).
pub async fn fetch_parallel(ids: &[u32], delay_ms: u64) -> Vec<Result<String, FetchError>> {
    match ids {
        [] => Vec::new(),
        [a] => vec![fetch_threat(*a, delay_ms).await],
        [a, b] => {
            let (ra, rb) = tokio::join!(fetch_threat(*a, delay_ms), fetch_threat(*b, delay_ms));
            vec![ra, rb]
        }
        [a, b, c] => {
            let (ra, rb, rc) = tokio::join!(
                fetch_threat(*a, delay_ms),
                fetch_threat(*b, delay_ms),
                fetch_threat(*c, delay_ms),
            );
            vec![ra, rb, rc]
        }
        rest => {
            let mut handles = Vec::with_capacity(rest.len());
            for &id in rest {
                handles.push(tokio::spawn(
                    async move { fetch_threat(id, delay_ms).await },
                ));
            }
            let mut out = Vec::with_capacity(handles.len());
            for handle in handles {
                match handle.await {
                    Ok(result) => out.push(result),
                    Err(_) => out.push(Err(FetchError::NotFound)),
                }
            }
            out
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        FetchError, fetch_parallel, fetch_sequential, fetch_threat, fetch_threat_with_timeout,
    };

    #[tokio::test]
    async fn fetch_threat_returns_label() {
        let got = fetch_threat(42, 1).await;
        assert_eq!(got, Ok("threat-42".into()));
    }

    #[tokio::test]
    async fn fetch_threat_rejects_zero_id() {
        assert_eq!(fetch_threat(0, 1).await, Err(FetchError::NotFound));
    }

    #[tokio::test]
    async fn timeout_returns_err_when_too_short() {
        let got = fetch_threat_with_timeout(5, 50, 5).await;
        assert_eq!(got, Err(FetchError::TimedOut));
    }

    #[tokio::test]
    async fn timeout_succeeds_when_generous() {
        let got = fetch_threat_with_timeout(7, 5, 100).await;
        assert_eq!(got, Ok("threat-7".into()));
    }

    #[tokio::test]
    async fn parallel_faster_than_sequential() {
        let ids = [1_u32, 2, 3];
        let start = std::time::Instant::now();
        let _ = fetch_sequential(&ids, 30).await;
        let seq_elapsed = start.elapsed();

        let start = std::time::Instant::now();
        let parallel = fetch_parallel(&ids, 30).await;
        let par_elapsed = start.elapsed();

        assert_eq!(parallel.len(), 3);
        assert!(parallel.iter().all(Result::is_ok));
        assert!(
            par_elapsed < seq_elapsed,
            "parallel {par_elapsed:?} should beat sequential {seq_elapsed:?}"
        );
    }

    #[tokio::test]
    async fn join_all_style_for_many_ids() {
        let ids: Vec<u32> = (1..=5).collect();
        let results = fetch_parallel(&ids, 1).await;
        assert_eq!(results.len(), 5);
        assert!(results.iter().all(Result::is_ok));
    }
}
