//! Exercise 1 — `filter`, `map`, and `sum` for allowed traffic volume.

use anyhow::Result;

use crate::log_line::{allowed_bytes, sample_logs};

/// Demonstrate summing bytes on allowed log lines.
pub fn run(verbose: bool) -> Result<()> {
    println!("📊 Adapters — allowed byte volume");
    println!();

    let logs = sample_logs();
    let total = allowed_bytes(&logs);

    println!("  log lines: {}", logs.len());
    println!("  allowed bytes (filter → map → sum): {total}");

    if verbose {
        println!();
        println!("  ALLOW lines:");
        for line in logs.iter().filter(|l| l.is_allow()) {
            println!("    {} → {} bytes", line.ip, line.bytes);
        }
    }

    Ok(())
}
