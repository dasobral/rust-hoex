//! CLI for the 21-async threat-fetch exercises.
//!
//! ```bash
//! cargo run -p exercise_async
//! cargo run -p exercise_async -- --ids 1,2,3 --delay 40 --timeout 100 --verbose
//! ```

use std::time::Instant;

use anyhow::Result;
use async_exercises::{FetchError, fetch_parallel, fetch_sequential, fetch_threat_with_timeout};
use clap::Parser;

/// Async threat intel fetches with Tokio (offline simulated latency)
#[derive(Parser, Debug)]
#[command(
    name = "exercise_async",
    about = "Simulated async fetches with timeout and parallel join",
    version
)]
struct Cli {
    /// Comma-separated threat ids to fetch
    #[arg(short, long, default_value = "1,2,3")]
    ids: String,

    /// Simulated latency per fetch (milliseconds)
    #[arg(short, long, default_value_t = 40)]
    delay: u64,

    /// Timeout per fetch (milliseconds)
    #[arg(short, long, default_value_t = 200)]
    timeout: u64,

    #[arg(short, long)]
    verbose: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let ids = parse_ids(&cli.ids)?;

    println!("🦀 rust-hoex · 21-async exercises\n");

    if cli.verbose {
        println!(
            "ids={ids:?}, delay={}ms, timeout={}ms\n",
            cli.delay, cli.timeout
        );
    }

    println!("--- timeout demo (id=9) ---");
    match fetch_threat_with_timeout(9, cli.delay, cli.timeout).await {
        Ok(label) => println!("  ok: {label}"),
        Err(FetchError::TimedOut) => println!("  timed out"),
        Err(FetchError::NotFound) => println!("  not found"),
    }

    let short_timeout = cli.delay.saturating_mul(2).min(10);
    match fetch_threat_with_timeout(9, cli.delay, short_timeout).await {
        Ok(label) => println!("  short timeout ok: {label}"),
        Err(FetchError::TimedOut) => {
            println!("  short timeout: timed out (expected when delay > timeout)");
        }
        Err(FetchError::NotFound) => println!("  short timeout: not found"),
    }

    println!("\n--- sequential ---");
    let start = Instant::now();
    let sequential = fetch_sequential(&ids, cli.delay).await;
    println!("  elapsed: {:?}", start.elapsed());
    print_results(&sequential);

    println!("\n--- parallel (join / join_all) ---");
    let start = Instant::now();
    let parallel = fetch_parallel(&ids, cli.delay).await;
    println!("  elapsed: {:?}", start.elapsed());
    print_results(&parallel);

    println!("\n🎉 Demo finished.");
    Ok(())
}

fn parse_ids(raw: &str) -> Result<Vec<u32>> {
    let mut ids = Vec::new();
    for part in raw.split(',') {
        let trimmed = part.trim();
        if trimmed.is_empty() {
            continue;
        }
        let id: u32 = trimmed
            .parse()
            .map_err(|_| anyhow::anyhow!("invalid id: {trimmed}"))?;
        ids.push(id);
    }
    if ids.is_empty() {
        anyhow::bail!("at least one id is required");
    }
    Ok(ids)
}

fn print_results(results: &[Result<String, FetchError>]) {
    for (i, result) in results.iter().enumerate() {
        match result {
            Ok(label) => println!("  [{i}] ok: {label}"),
            Err(err) => println!("  [{i}] err: {err:?}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;

    #[test]
    fn cli_parses() {
        Cli::command().debug_assert();
    }

    #[test]
    fn parse_ids_splits_commas() {
        let ids = parse_ids("1, 2,3");
        assert!(ids.is_ok());
        assert_eq!(ids.ok(), Some(vec![1, 2, 3]));
    }
}
