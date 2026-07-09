//! example: 05-controlFlow
//!
//! Threat-score log classifier — if/else, loops, and match.
//!
//! Run from this directory:
//! ```bash
//! cargo run
//! ```
//!
//! Key concepts:
//! - `if` / `else` as expressions
//! - `loop`, `while`, and `for`
//! - `break` / `continue` (including break with a value)
//! - `match` on integers and enums
//! - `if let` for concise `Option` handling

use example_controlflow::{
    AccessDecision, accumulate_until_budget, apply_analyst_pin, classify_batch, decision_label,
    first_critical_index, score_log_line, severity_from_score, severity_label, severity_weight,
};

fn main() {
    println!("\nThreat-Score Log Classifier");
    println!("============================\n");

    let logs = [
        "info: scheduler tick",
        "",
        "warning: failed login for alice",
        "ALERT: unauthorized access attempt",
        "CRITICAL malware signature matched",
        "info: connection closed",
    ];

    // === if as an expression ===
    let headline_score = score_log_line(logs[4]);
    let headline = severity_from_score(headline_score);
    let banner = if headline_score >= 90 {
        "SEVERE EVENT IN STREAM"
    } else {
        "stream nominal"
    };
    println!("Banner: {banner}");
    println!(
        "Headline line score={headline_score} => {} (weight {})",
        severity_label(headline),
        severity_weight(headline)
    );

    // === for loop over an array ===
    println!("\nPer-line classification:");
    for (idx, line) in logs.iter().enumerate() {
        if line.is_empty() {
            // continue skips blank lines in the display loop
            continue;
        }
        let score = score_log_line(line);
        let level = severity_from_score(score);
        let decision = access_with_optional_pin(level, score);
        println!(
            "  [{idx}] score={score:3} {:>8} -> {:<9} | {line}",
            severity_label(level),
            decision_label(decision)
        );
    }

    // === loop / while helpers ===
    let (scanned, weight, early) = classify_batch(&logs, 4);
    println!("\nBatch summary: scanned={scanned}, weight={weight}, early_stop={early}");

    let scores: Vec<u8> = logs
        .iter()
        .filter(|l| !l.is_empty())
        .map(|l| score_log_line(l))
        .collect();
    let used = accumulate_until_budget(&scores, 30);
    println!("Budget 30 consumed: {used}");

    // === match + if let ===
    // `if let` unwraps Option without a full match.
    if let Some(idx) = first_critical_index(&scores) {
        println!(
            "First critical score at index {idx} (value {})",
            scores[idx]
        );
    } else {
        println!("No critical scores in this batch");
    }

    // match on the decision enum for a final policy note
    let final_decision = apply_analyst_pin(AccessDecision::Allow, Some("REVIEW"));
    let note = match final_decision {
        AccessDecision::Allow => "open the gate",
        AccessDecision::Challenge => "step-up authentication required",
        AccessDecision::Deny => "block and page on-call",
    };
    println!(
        "Analyst pin applied => {}: {note}",
        decision_label(final_decision)
    );

    println!("\nDone. See README.md for exercises and Rust Book links.");
}

fn access_with_optional_pin(level: example_controlflow::Severity, score: u8) -> AccessDecision {
    use example_controlflow::access_decision;

    let base = access_decision(level);
    // Demonstrate if-expression selecting an optional pin.
    let pin = if score >= 70 { Some("REVIEW") } else { None };
    apply_analyst_pin(base, pin)
}
