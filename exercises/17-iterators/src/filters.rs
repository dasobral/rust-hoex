//! Exercise 2 — `filter`, `map`, `collect`, and `count` on deny events.

use anyhow::Result;

use crate::log_line::{count_allows, denied_ips, sample_logs};

/// Demonstrate collecting denied IPs and counting allows.
pub fn run(verbose: bool) -> Result<()> {
    println!("🚫 Filters — denied IPs and allow counts");
    println!();

    let logs = sample_logs();
    let blocked = denied_ips(&logs);
    let allows = count_allows(&logs);

    println!("  denied source IPs: {blocked:?}");
    println!("  allow count (filter().count()): {allows}");

    if verbose {
        println!();
        println!("  DENY lines:");
        for ip in &blocked {
            println!("    blocked: {ip}");
        }
    }

    Ok(())
}
