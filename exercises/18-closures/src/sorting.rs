//! Exercise 1 — `sort_by_key` for deterministic source ordering.

use anyhow::Result;

use crate::threat::{sample_events, sort_by_source};

/// Demonstrate sorting threat events by source IP.
pub fn run(verbose: bool) -> Result<()> {
    println!("📋 Sorting — sort_by_key on source");
    println!();

    let mut events = sample_events();
    sort_by_source(&mut events);

    println!("  events sorted by source (ascending):");
    for event in &events {
        println!(
            "    {} score={} critical={}",
            event.source, event.score, event.critical
        );
    }

    if verbose {
        println!();
        println!("  First source: {}", events[0].source);
        let last = events.last();
        if let Some(event) = last {
            println!("  Last source: {}", event.source);
        }
    }

    Ok(())
}
