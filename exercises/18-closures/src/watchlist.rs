//! Exercise 3 — closures capturing a `HashSet` watchlist.

use std::collections::HashSet;

use anyhow::Result;

use crate::threat::{count_matching, count_watchlisted, sample_events};

/// Demonstrate a closure that captures a watchlist of suspicious IPs.
pub fn run(verbose: bool) -> Result<()> {
    println!("👁️  Watchlist — closure over HashSet");
    println!();

    let events = sample_events();
    let mut watchlist = HashSet::new();
    watchlist.insert("203.0.113.10".to_owned());
    watchlist.insert("198.51.100.1".to_owned());

    let listed = count_watchlisted(&events, &watchlist);
    let high_score = count_matching(&events, |e| e.score >= 50);

    println!("  watchlist size: {}", watchlist.len());
    println!("  events from watchlisted IPs: {listed}");
    println!("  events with score >= 50: {high_score}");

    if verbose {
        println!();
        println!("  Watchlist entries:");
        for ip in &watchlist {
            println!("    {ip}");
        }
        let flagged = events
            .iter()
            .filter(|e| watchlist.contains(&e.source))
            .collect::<Vec<_>>();
        for event in flagged {
            println!(
                "    flagged: {} score={} critical={}",
                event.source, event.score, event.critical
            );
        }
    }

    Ok(())
}
