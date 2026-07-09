//! example: 10-modules
//!
//! Code organization: a small multi-file crate with a clear public API.
//!
//! Run from this directory:
//! ```bash
//! cargo run
//! ```
//!
//! Key ideas:
//! - `mod` declares a module; files / directories hold the code
//! - `pub` vs `pub(crate)` control who can see an item
//! - the crate root (`lib.rs`) re-exports a tidy public surface
//! - `use` / paths navigate the module tree

use example_modules::{RiskLevel, analyze};

fn main() {
    println!("=== 10-modules: secret strength analyzer ===\n");

    let samples = [
        "",
        "password",
        "S3cret!",
        "correct-horse-battery-staple-99!",
    ];

    for secret in samples {
        let label = if secret.is_empty() { "(empty)" } else { secret };
        println!("input: {label}");
        let analysis = analyze(secret);
        print_summary(&analysis);
        println!("{}\n", analysis.report());
    }
}

fn print_summary(analysis: &example_modules::Analysis) {
    let badge = match analysis.risk {
        RiskLevel::Critical => "CRIT",
        RiskLevel::High => "HIGH",
        RiskLevel::Medium => "MED ",
        RiskLevel::Low => "LOW ",
    };
    println!("  [{badge}] score={}", analysis.score);
}
