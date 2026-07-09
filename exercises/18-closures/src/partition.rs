//! Exercise 2 — `into_iter().partition` for critical vs normal queues.

use anyhow::Result;

use crate::threat::{partition_critical, sample_events};

/// Demonstrate splitting events into critical and non-critical buckets.
pub fn run(verbose: bool) -> Result<()> {
    println!("⚡ Partition — into_iter().partition");
    println!();

    let events = sample_events();
    let total = events.len();
    let (critical, normal) = partition_critical(events);

    println!("  total events consumed: {total}");
    println!("  critical queue: {}", critical.len());
    println!("  normal queue: {}", normal.len());

    if verbose {
        println!();
        println!("  Critical sources:");
        for event in &critical {
            println!("    {} (score {})", event.source, event.score);
        }
        println!("  Normal sources:");
        for event in &normal {
            println!("    {} (score {})", event.source, event.score);
        }
    }

    Ok(())
}
