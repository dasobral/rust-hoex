//! Integration tests for `example_async`.

#![allow(clippy::unwrap_used)]

use example_async::{fetch_simulated, fetch_three_concurrent};

#[tokio::test]
async fn public_fetch_and_join() {
    let r = fetch_simulated("/ping", 1).await.unwrap();
    assert_eq!(r.status, 200);
    let (a, b, c) = fetch_three_concurrent(("/1", 1), ("/2", 1), ("/3", 1)).await;
    assert!(a.is_ok() && b.is_ok() && c.is_ok());
}
