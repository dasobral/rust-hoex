//! example: 16-testing
//!
//! Demonstrates a testable password-policy library and how to run filtered
//! `cargo test` invocations.
//!
//! ```bash
//! cargo run
//! cargo test
//! cargo test policy
//! cargo test --test integration
//! ```

use example_testing::{check_or_err, check_policy, estimate_entropy_bits};

fn main() {
    println!("=== 16-testing: password policy (built to be tested) ===\n");

    let samples = ["", "password", "short1A", "CorrectHorseBattery1"];

    for pw in samples {
        let label = if pw.is_empty() { "(empty)" } else { pw };
        let report = check_policy(pw);
        let bits = estimate_entropy_bits(pw);
        println!("candidate: {label}");
        println!("  ok={}  entropy≈{bits:.1} bits", report.ok);
        if report.ok {
            println!("  violations: (none)");
        } else {
            println!("  violations: {}", report.violations.join(", "));
        }
        match check_or_err(pw) {
            Ok(()) => println!("  check_or_err: Ok"),
            Err(e) => println!("  check_or_err: Err({e})"),
        }
        println!();
    }

    println!("Tip: cargo test policy   # run only tests whose names match");
}
