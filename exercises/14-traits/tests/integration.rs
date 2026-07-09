//! Integration tests for `exercise_traits`.

use traits_exercises::{
    AuthFailure, FileIntegrityEvent, MalwareAlert, NetworkScan, RiskLevel, ThreatScorer,
    analyze_event, get_exercise_list, is_actionable, max_score, run_all, run_exercise,
};

#[test]
fn exercise_list_has_three_entries() {
    assert_eq!(get_exercise_list().len(), 3);
}

#[test]
fn run_each_exercise() {
    assert!(run_exercise("auth", false).is_ok());
    assert!(run_exercise("network", false).is_ok());
    assert!(run_exercise("malware", false).is_ok());
}

#[test]
fn run_all_succeeds() {
    assert!(run_all(false).is_ok());
}

#[test]
fn auth_failure_trait_impl() {
    let event = AuthFailure {
        user: "root".into(),
        attempts: 3,
        from_known_host: false,
    };
    assert_eq!(event.category(), "auth");
    assert!(event.score() >= 60);
    assert!(is_actionable(&event));
}

#[test]
fn network_scan_summary_override() {
    let scan = NetworkScan {
        source_ip: "10.0.0.1".into(),
        ports_hit: 10,
        payload_bytes: 1024,
    };
    let summary = analyze_event(&scan);
    assert!(summary.contains("ports"));
}

#[test]
fn malware_risk_level_floor_and_critical() {
    let high_floor = MalwareAlert {
        hash_prefix: "aa".into(),
        confidence: 50,
    };
    assert_eq!(high_floor.risk_level(), RiskLevel::High);

    let critical = MalwareAlert {
        hash_prefix: "bb".into(),
        confidence: 90,
    };
    assert_eq!(critical.risk_level(), RiskLevel::Critical);
    assert!(critical.is_critical());
}

#[test]
fn file_integrity_and_max_score() {
    let events = [
        FileIntegrityEvent {
            path: "/bin/ls".into(),
            severity: 30,
        },
        FileIntegrityEvent {
            path: "/etc/shadow".into(),
            severity: 90,
        },
    ];
    assert_eq!(max_score(&events), 90);
    assert_eq!(events[1].category(), "integrity");
}
