//! example: 14-traits
//!
//! Shared `ThreatScorer` behavior across auth, network, and malware events.
//!
//! Run from this directory:
//! ```bash
//! cargo run
//! ```
//!
//! Key ideas:
//! - trait definition + `impl Trait for Type`
//! - default methods (`risk_level`, `summary`)
//! - trait bounds on functions (static dispatch)
//! - derive for `Debug` / `Clone`; manual impl for domain logic

use example_traits::{
    AuthFailure, MalwareAlert, NetworkScan, ThreatScorer, analyze_event, higher_risk,
    is_actionable, summarize_all,
};

fn main() {
    println!("=== 14-traits: ThreatScorer (static dispatch) ===\n");

    let auth = AuthFailure {
        user: String::from("alice"),
        attempts: 3,
        from_known_host: false,
    };
    let scan = NetworkScan {
        source_ip: String::from("198.51.100.20"),
        ports_hit: 12,
        payload_bytes: 2048,
    };
    let malware = MalwareAlert {
        hash_prefix: String::from("cafebabe"),
        confidence: 88,
    };

    println!("-- analyze_event (trait bound) --");
    println!("  {}", analyze_event(&auth));
    println!("  {}", analyze_event(&scan));
    println!("  {}", analyze_event(&malware));
    println!();

    println!("-- actionable? --");
    for (name, flag) in [
        ("auth", is_actionable(&auth)),
        ("scan", is_actionable(&scan)),
        ("malware", is_actionable(&malware)),
    ] {
        println!("  {name}: {flag}");
    }
    println!();

    println!("-- higher_risk (same concrete type) --");
    let other = MalwareAlert {
        hash_prefix: String::from("00ff00ff"),
        confidence: 40,
    };
    let top = higher_risk(&malware, &other);
    println!("  winner: {} (score={})", top.hash_prefix, top.score());
    println!();

    println!("-- summarize_all (homogeneous slice) --");
    let batch = [
        AuthFailure {
            user: String::from("bob"),
            attempts: 1,
            from_known_host: true,
        },
        AuthFailure {
            user: String::from("carol"),
            attempts: 6,
            from_known_host: false,
        },
    ];
    for line in summarize_all(&batch) {
        println!("  {line}");
    }
}
