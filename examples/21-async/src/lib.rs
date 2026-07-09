//! Async/await with Tokio — simulated concurrent HTTP-like fetches.
//!
//! # Async ≠ threads
//!
//! - **Threads** (example 20): the OS schedules preemptive tasks; each has its
//!   own stack. Great for CPU-bound or blocking work.
//! - **Async tasks**: cooperatively scheduled on a runtime (here, Tokio). A
//!   task yields at `.await` so **one** OS thread can multiplex many waits
//!   (I/O, timers). Ideal for lots of idle-bound concurrency.
//!
//! This example stays **offline**: we fake network latency with
//! `tokio::time::sleep` — no real sockets required.
//!
//! Concepts: `async fn`, `.await`, `tokio::spawn`, `tokio::join!`.

use std::time::Duration;

use tokio::time::sleep;

/// Outcome of one simulated fetch.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FetchResult {
    /// Logical URL / resource name.
    pub url: String,
    /// Fake HTTP status.
    pub status: u16,
    /// Body bytes (simulated).
    pub body: String,
    /// How long we pretended to wait.
    pub latency_ms: u64,
}

/// Errors from the async helpers (library code never panics).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FetchError {
    /// Empty URL rejected.
    EmptyUrl,
    /// Simulated 5xx from the "server".
    Server(u16),
}

/// Simulate fetching `url` after `latency_ms` of async sleep.
///
/// Returning a `Future` (via `async fn`) does **no** work until polled /
/// `.await`ed. Sleeping here does not block the OS thread — it yields.
pub async fn fetch_simulated(url: &str, latency_ms: u64) -> Result<FetchResult, FetchError> {
    if url.is_empty() {
        return Err(FetchError::EmptyUrl);
    }

    sleep(Duration::from_millis(latency_ms)).await;

    // Toy routing: paths containing "fail" look like a 503.
    if url.contains("fail") {
        return Err(FetchError::Server(503));
    }

    let status = if url.contains("missing") { 404 } else { 200 };
    let body = format!("payload for {url}");
    Ok(FetchResult {
        url: url.to_owned(),
        status,
        body,
        latency_ms,
    })
}

/// Run three fetches **concurrently** with `tokio::join!`.
///
/// All three futures are polled on the same task; wall time ≈ max(latencies),
/// not the sum (contrast sequential `.await`s).
pub async fn fetch_three_concurrent(
    a: (&str, u64),
    b: (&str, u64),
    c: (&str, u64),
) -> (
    Result<FetchResult, FetchError>,
    Result<FetchResult, FetchError>,
    Result<FetchResult, FetchError>,
) {
    tokio::join!(
        fetch_simulated(a.0, a.1),
        fetch_simulated(b.0, b.1),
        fetch_simulated(c.0, c.1),
    )
}

/// Spawn independent tasks and await their `JoinHandle`s.
///
/// `tokio::spawn` requires `'static` futures — owned `String`s, not `&str`
/// borrows from the caller stack.
pub async fn fetch_many_spawned(jobs: Vec<(String, u64)>) -> Vec<Result<FetchResult, FetchError>> {
    let mut handles = Vec::with_capacity(jobs.len());
    for (url, latency) in jobs {
        handles.push(tokio::spawn(
            async move { fetch_simulated(&url, latency).await },
        ));
    }

    let mut out = Vec::with_capacity(handles.len());
    for handle in handles {
        match handle.await {
            Ok(result) => out.push(result),
            // Task panicked — surface as a synthetic server error, no unwrap.
            Err(_) => out.push(Err(FetchError::Server(500))),
        }
    }
    out
}

/// Sequential baseline: sum of latencies (for comparison demos).
pub async fn fetch_two_sequential(
    first: (&str, u64),
    second: (&str, u64),
) -> (
    Result<FetchResult, FetchError>,
    Result<FetchResult, FetchError>,
) {
    let a = fetch_simulated(first.0, first.1).await;
    let b = fetch_simulated(second.0, second.1).await;
    (a, b)
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]

    use super::*;

    #[tokio::test]
    async fn fetch_simulated_ok() {
        let got = fetch_simulated("/health", 1).await;
        assert!(got.is_ok());
        let r = got.unwrap();
        assert_eq!(r.status, 200);
        assert!(r.body.contains("/health"));
    }

    #[tokio::test]
    async fn fetch_simulated_rejects_empty() {
        let got = fetch_simulated("", 1).await;
        assert_eq!(got, Err(FetchError::EmptyUrl));
    }

    #[tokio::test]
    async fn join_runs_concurrently() {
        let start = std::time::Instant::now();
        let (a, b, c) = fetch_three_concurrent(("/a", 30), ("/b", 30), ("/c", 30)).await;
        let elapsed = start.elapsed();
        assert!(a.is_ok() && b.is_ok() && c.is_ok());
        // Concurrent: should be well under 3 * 30ms (allow generous CI slack).
        assert!(
            elapsed.as_millis() < 80,
            "expected concurrent ~30ms, got {elapsed:?}"
        );
    }

    #[tokio::test]
    async fn spawn_collects_results() {
        let jobs = vec![("/x".into(), 1_u64), ("/fail".into(), 1_u64)];
        let results = fetch_many_spawned(jobs).await;
        assert_eq!(results.len(), 2);
        assert!(results[0].is_ok());
        assert_eq!(results[1], Err(FetchError::Server(503)));
    }
}
