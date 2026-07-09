//! example: 13-generics
//!
//! Generic containers, pairs, and helpers for security-flavored data.
//!
//! Run from this directory:
//! ```bash
//! cargo run
//! ```
//!
//! Key ideas:
//! - type parameters on structs and functions
//! - trait bounds (`Display`, `PartialOrd`, `Clone`)
//! - monomorphization: one concrete copy per type used
//! - generic `SecureContainer<T>` and `Pair<A, B>`

use example_generics::{Pair, SecureContainer, filter_owned, find_max, format_reading, same_label};

fn main() {
    println!("=== 13-generics: SecureContainer, Pair, find_max ===\n");

    demo_container();
    demo_pair();
    demo_find_max();
    demo_filter();
}

fn demo_container() {
    println!("-- SecureContainer<T> --");
    // Same struct, different T — compiler monomorphizes two versions.
    let key = SecureContainer::new("api-key", String::from("sk_live_demo"));
    let score = SecureContainer::new("threat-score", 91u32);

    println!("  {}", key.audit_line());
    println!("  {}", score.audit_line());
    println!(
        "  same label? {}",
        same_label(&key, &SecureContainer::new("api-key", 0u8))
    );
    println!();
}

fn demo_pair() {
    println!("-- Pair<A, B> --");
    let hit = Pair::new("203.0.113.50", 4u32);
    println!("  {}", hit.describe());
    let flipped = hit.swap();
    println!("  swapped: left={} right={}", flipped.left, flipped.right);
    println!();
}

fn demo_find_max() {
    println!("-- find_max (T: PartialOrd) --");
    let latency_ms = [12u32, 40, 9, 85, 33];
    match find_max(&latency_ms) {
        Some(max) => println!("  max latency: {max} ms"),
        None => println!("  empty sample"),
    }
    println!("  {}", format_reading("sensor", "ids-1"));
    println!();
}

fn demo_filter() {
    println!("-- filter_owned --");
    let events = vec![
        String::from("login_fail"),
        String::from("login_ok"),
        String::from("port_scan"),
        String::from("login_fail"),
    ];
    let fails = filter_owned(events, |e| e.contains("fail") || e.contains("scan"));
    println!("  flagged: {fails:?}");
}
