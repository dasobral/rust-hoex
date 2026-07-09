//! Exercise 1 — building and inspecting an intrusion log with `Vec`.

use anyhow::Result;

use crate::intrusion::{IntrusionLog, LogEvent, sample_log};

/// Run the aggregator exercise: push events and read basic metrics.
pub fn run(verbose: bool) -> Result<()> {
    println!("📋 Aggregator — Vec-backed intrusion log");
    println!();

    let mut log = IntrusionLog::new();
    log.push(LogEvent::new("192.0.2.55", "eve", "port_scan"));
    log.push(LogEvent::new("192.0.2.55", "eve", "port_scan"));
    log.push(LogEvent::new("198.51.100.1", "mallory", "login_fail"));

    println!("  events stored: {}", log.len());
    println!("  empty? {}", log.is_empty());
    println!("  summary: {}", log.summary());

    if verbose {
        println!();
        println!("  Event stream (Vec iteration):");
        for (i, event) in log.events().iter().enumerate() {
            println!("    [{i}] {} @ {} → {}", event.user, event.ip, event.action);
        }
    }

    println!();
    println!("  Sample SOC log: {}", sample_log().summary());
    Ok(())
}
