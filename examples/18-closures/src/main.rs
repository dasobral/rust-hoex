//! example: 18-closures
//!
//! Custom sort and filter of threat events using closures.
//!
//! ```bash
//! cargo run
//! ```

use example_closures::{
    Severity, ThreatEvent, consume_with_tag, count_matching, filter_by_severity, label_events,
    sort_threats, summarize_with_counter, with_event_once,
};

fn main() {
    println!("=== 18-closures: triage threat events ===\n");

    let mut events = vec![
        ThreatEvent::new("port-scan", "192.168.1.10", Severity::Low, 12),
        ThreatEvent::new("ransomware", "192.168.1.20", Severity::Critical, 99),
        ThreatEvent::new("credential-stuffing", "192.168.1.30", Severity::High, 70),
        ThreatEvent::new("dns-tunnel", "192.168.1.20", Severity::Medium, 40),
    ];

    sort_threats(&mut events);
    println!("sorted (severity, then score):");
    for event in &events {
        println!("  {:?} {:>5}  {}", event.severity, event.score, event.kind);
    }

    let criticalish = filter_by_severity(&events, Severity::High);
    println!("\nhigh+: {}", criticalish.len());

    for label in label_events(&events, "EVT") {
        println!("  {label}");
    }

    let from_20 = count_matching(&events, |e| e.source.ends_with(".20"));
    println!("\nfrom *.20: {from_20}");

    let (n, summary) = summarize_with_counter(&events);
    println!("FnMut counter saw {n}: {summary}");

    let tagged = consume_with_tag(events, "batch-7".to_owned());
    println!("move capture: {tagged}");

    let one = ThreatEvent::new("beacon", "10.0.0.8", Severity::Low, 5);
    let msg = with_event_once(one, |e| format!("handled {}", e.kind));
    println!("{msg}");
}
