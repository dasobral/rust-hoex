//! Integration tests for `exercise_async`.

use async_exercises::{FetchError, fetch_parallel, fetch_sequential, fetch_threat_with_timeout};

#[tokio::test]
async fn sequential_and_parallel_agree() {
    let ids = [2_u32, 3, 4];
    let seq = fetch_sequential(&ids, 1).await;
    let par = fetch_parallel(&ids, 1).await;
    assert_eq!(seq, par);
}

#[tokio::test]
async fn timeout_integration() {
    let fast = fetch_threat_with_timeout(1, 5, 50).await;
    assert_eq!(fast, Ok("threat-1".into()));

    let slow = fetch_threat_with_timeout(1, 80, 10).await;
    assert_eq!(slow, Err(FetchError::TimedOut));
}
