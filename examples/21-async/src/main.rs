//! example: 21-async
//!
//! Concurrent simulated fetches with Tokio (`async` / `.await` / `join!` / `spawn`).
//!
//! ```bash
//! cargo run
//! ```

use std::time::Instant;

use example_async::{fetch_many_spawned, fetch_three_concurrent, fetch_two_sequential};

#[tokio::main]
async fn main() {
    println!("=== 21-async: simulated concurrent fetches ===\n");

    let start = Instant::now();
    let (first, second) = fetch_two_sequential(("/seq-a", 40), ("/seq-b", 40)).await;
    println!("sequential (~80ms): {:?}", start.elapsed());
    print_one("seq-a", &first);
    print_one("seq-b", &second);

    let start = Instant::now();
    let (alpha, beta, gamma) =
        fetch_three_concurrent(("/api/a", 40), ("/api/b", 40), ("/api/c", 40)).await;
    println!("\njoin! concurrent (~40ms): {:?}", start.elapsed());
    print_one("a", &alpha);
    print_one("b", &beta);
    print_one("c", &gamma);

    let jobs = vec![
        ("/users".into(), 25_u64),
        ("/orders".into(), 25),
        ("/fail".into(), 10),
    ];
    let spawned = fetch_many_spawned(jobs).await;
    println!("\nspawned tasks:");
    for (i, result) in spawned.iter().enumerate() {
        print_one(&format!("job-{i}"), result);
    }

    println!("\n(Async multiplexes waits; threads preempt. Pick based on workload.)");
}

fn print_one(label: &str, result: &Result<example_async::FetchResult, example_async::FetchError>) {
    match result {
        Ok(r) => println!("  {label}: {} {} ({}ms)", r.status, r.url, r.latency_ms),
        Err(e) => println!("  {label}: err {e:?}"),
    }
}
