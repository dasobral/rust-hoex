//! Exercise 2 — `NetworkScan` with overridden summary.

use anyhow::Result;

use crate::scoring::{NetworkScan, analyze_event, is_actionable, max_score};

/// Run the network exercise.
pub fn run(verbose: bool) -> Result<()> {
    println!("🌐 Network — port scans and trait overrides");
    println!();

    let batch = [
        NetworkScan {
            source_ip: String::from("198.51.100.20"),
            ports_hit: 12,
            payload_bytes: 2048,
        },
        NetworkScan {
            source_ip: String::from("203.0.113.9"),
            ports_hit: 20,
            payload_bytes: 4096,
        },
    ];

    for scan in &batch {
        println!("  {}", analyze_event(scan));
        println!("    actionable={}", is_actionable(scan));
    }

    println!();
    println!("  max_score in batch: {}", max_score(&batch));

    if verbose {
        println!();
        println!("  NetworkScan overrides summary() for richer SOC output.");
    }

    Ok(())
}
