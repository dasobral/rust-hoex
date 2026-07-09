//! Exercise 2 — `HashMap` / `HashSet` aggregation over intrusion events.

use anyhow::Result;

use crate::intrusion::sample_log;

/// Run the analysis exercise: IP counts, unique users, actions, hot IPs.
pub fn run(verbose: bool) -> Result<()> {
    println!("📊 Analysis — HashMap & HashSet aggregation");
    println!();

    let log = sample_log();
    let ip_counts = log.ip_counts();
    let hot = log.hot_ips();

    println!("  ip_counts == hot_ips: {}", ip_counts == hot);
    println!("  unique users: {}", log.unique_users().len());

    let mut sorted_ips: Vec<_> = ip_counts.into_iter().collect();
    sorted_ips.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));

    println!("  IP hit counts:");
    for (ip, count) in &sorted_ips {
        println!("    {ip:>15}  {count}");
    }

    let mut actions: Vec<_> = log.actions().into_iter().collect();
    actions.sort();
    println!("  distinct actions: {}", actions.join(", "));

    if verbose {
        println!();
        println!("  hot_ips is an alias for ip_counts — same IP→count map,");
        println!("  named for SOC dashboards tracking repeat offenders.");
    }

    Ok(())
}
