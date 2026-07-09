//! Scoring exercise — keyword heuristics and severity mapping.

use crate::classifier::{cap_score, classify_score, score_log_line, severity_label};

/// Run the scoring exercise with demo output.
pub fn run(verbose: bool) {
    println!("📊 Threat Scoring — Keywords and Severity Mapping");
    println!();

    let samples = [
        "info: agent heartbeat ok",
        "warning: disk threshold exceeded",
        "error: authentication fail from 10.0.0.5",
        "CRITICAL: malware exploit attempt denied",
    ];

    for (i, line) in samples.iter().enumerate() {
        let raw = score_log_line(line);
        let capped = cap_score(raw);
        let level = classify_score(capped);
        println!(
            "{}. [{}] raw={raw} capped={capped} — {line}",
            i + 1,
            severity_label(level)
        );
    }

    if verbose {
        println!();
        println!("   Why it matters:");
        println!("   - SIEM rules often start as keyword + weight heuristics");
        println!("   - `match` on ranges maps continuous scores to discrete tiers");
        println!("   - `if` expressions cap scores without extra bindings");
    }
}
