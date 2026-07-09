//! example: 11-collections
//!
//! Intrusion log aggregation with `Vec`, `HashMap`, and `HashSet`.
//!
//! Run from this directory:
//! ```bash
//! cargo run
//! ```
//!
//! Key ideas:
//! - `Vec` stores an ordered event stream
//! - `HashMap` counts hits per IP via the `entry` API
//! - `HashSet` tracks unique usernames
//! - collections own their elements; iterators usually borrow

use example_collections::{IntrusionLog, LogEvent, merge_logs};

fn main() {
    println!("=== 11-collections: intrusion log aggregator ===\n");

    let mut log = IntrusionLog::new();
    log.push(LogEvent::new("203.0.113.10", "alice", "login_fail"));
    log.push(LogEvent::new("203.0.113.10", "alice", "login_fail"));
    log.push(LogEvent::new("198.51.100.7", "bob", "login_ok"));
    log.push(LogEvent::new("203.0.113.10", "carol", "login_fail"));
    log.push(LogEvent::new("198.51.100.7", "bob", "file_access"));

    println!("summary: {}", log.summary());
    println!();

    println!("-- IP hit counts (HashMap) --");
    let mut counts: Vec<_> = log.ip_counts().into_iter().collect();
    counts.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
    for (ip, n) in counts {
        println!("  {ip:>15}  {n}");
    }
    println!();

    println!("-- Unique users (HashSet) --");
    let mut users: Vec<_> = log.unique_users().into_iter().collect();
    users.sort();
    for user in &users {
        println!("  {user}");
    }
    println!();

    println!("-- Failed logins only (Vec filter) --");
    for event in log.filter_by_action("login_fail") {
        println!("  {} @ {} → {}", event.user, event.ip, event.action);
    }
    println!();

    println!("-- Hot IPs (threshold ≥ 3) --");
    for ip in log.hot_ips(3) {
        println!("  suspect: {ip}");
    }
    println!();

    // Ownership demo: merge consumes both logs into one.
    let mut extra = IntrusionLog::new();
    extra.push(LogEvent::new("192.0.2.55", "eve", "port_scan"));
    let combined = merge_logs(log, extra);
    println!("after merge: {}", combined.summary());
}
