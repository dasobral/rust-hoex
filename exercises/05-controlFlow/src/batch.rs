//! Batch exercise — classify log batches with loops and limits.

use crate::classifier::{
    accumulate_until_budget, classify_batch, severity_label, walk_nonempty_lines,
};

/// Run the batch classification exercise with demo output.
pub fn run(verbose: bool) {
    println!("📋 Log Batch Classifier — Loops, Limits, and while-let");
    println!();

    let lines = [
        "",
        "info: routine scan complete",
        "error: login fail count exceeded",
        "malware signature match on host-7",
        "",
        "warning: outbound connection spike",
    ];

    let nonempty = walk_nonempty_lines(&lines);
    println!("1. Non-empty lines (while-let walk): {}", nonempty.len());

    let batch = classify_batch(&lines, 3);
    println!("2. Classified (max 3 events):");
    for (line, severity) in &batch {
        println!("   [{}] {line}", severity_label(*severity));
    }

    let used = accumulate_until_budget(&lines, 60);
    println!("3. Score budget consumed: {used}/60");

    if verbose {
        println!();
        println!("   Why it matters:");
        println!("   - Blank lines must not inflate event counters");
        println!("   - `break` stops processing when quotas are hit");
        println!("   - `while let` walks iterators without indexing panics");
    }
}
