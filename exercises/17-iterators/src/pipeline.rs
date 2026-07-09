//! Exercise 3 — `inspect` for side effects inside lazy iterator chains.

use anyhow::Result;

use crate::log_line::{allowed_bytes, inspect_action_counts, sample_logs};

/// Walk a pipeline with `inspect` before consuming with `for_each`.
pub fn run(verbose: bool) -> Result<()> {
    println!("🔍 Pipeline — inspect-friendly action tally");
    println!();

    let logs = sample_logs();
    let (allows, denies) = inspect_action_counts(&logs);
    let bytes = allowed_bytes(&logs);

    println!("  inspect tallies: {allows} ALLOW / {denies} DENY");
    println!("  allowed bytes after same pass data: {bytes}");

    if verbose {
        println!();
        println!("  Chained view (enumerate + inspect):");
        let traced = logs
            .iter()
            .enumerate()
            .inspect(|(i, line)| {
                println!("    [{i}] {} {}", line.action, line.ip);
            })
            .count();
        println!("  traced {traced} lines");
    }

    Ok(())
}
