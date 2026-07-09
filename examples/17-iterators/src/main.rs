//! example: 17-iterators
//!
//! Process firewall-style log lines with iterator adapter chains.
//!
//! ```bash
//! cargo run
//! ```

use example_iterators::{
    consume_into_report, count_allows_with_next, denied_ips, first_n_sources, numbered_summaries,
    parse_logs, total_bytes, uppercase_actions,
};

fn main() {
    println!("=== 17-iterators: log lines through adapter chains ===\n");

    let raw = [
        "src=10.0.0.5 action=ALLOW bytes=512",
        "src=10.0.0.9 action=DENY bytes=0",
        "src=10.0.0.5 action=DENY bytes=128",
        "",
        "src=10.0.0.7 action=ALLOW bytes=64",
    ];

    let mut logs = parse_logs(&raw);
    println!("parsed {} lines", logs.len());
    println!("denied IPs: {:?}", denied_ips(&logs));
    println!("total bytes: {}", total_bytes(&logs));
    println!("first 2 sources: {:?}", first_n_sources(&logs, 2));
    println!("allows (manual next): {}", count_allows_with_next(&logs));

    for (i, summary) in numbered_summaries(&logs) {
        println!("  [{i}] {summary}");
    }

    uppercase_actions(&mut logs);
    let report = consume_into_report(logs);
    println!("\nconsumed report: {report}");
}
