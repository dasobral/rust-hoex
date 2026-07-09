//! Integration tests for `example_traits`.

use example_traits::{
    AuthFailure, MalwareAlert, NetworkScan, RiskLevel, ThreatScorer, analyze_event, is_actionable,
};

#[test]
fn three_event_types_share_trait() {
    let auth = AuthFailure {
        user: "x".into(),
        attempts: 2,
        from_known_host: true,
    };
    let scan = NetworkScan {
        source_ip: "1.2.3.4".into(),
        ports_hit: 3,
        payload_bytes: 0,
    };
    let mal = MalwareAlert {
        hash_prefix: "ab".into(),
        confidence: 10,
    };

    assert_eq!(auth.category(), "auth");
    assert_eq!(scan.category(), "network");
    assert_eq!(mal.category(), "malware");
    assert!(analyze_event(&auth).contains("auth"));
    assert!(!is_actionable(&mal));
    assert_eq!(mal.risk_level(), RiskLevel::Low);
}

#[test]
fn default_risk_bands() {
    let low = MalwareAlert {
        hash_prefix: "a".into(),
        confidence: 20,
    };
    let mid = MalwareAlert {
        hash_prefix: "b".into(),
        confidence: 50,
    };
    let high = MalwareAlert {
        hash_prefix: "c".into(),
        confidence: 70,
    };
    let crit = MalwareAlert {
        hash_prefix: "d".into(),
        confidence: 95,
    };
    assert_eq!(low.risk_level(), RiskLevel::Low);
    assert_eq!(mid.risk_level(), RiskLevel::Medium);
    assert_eq!(high.risk_level(), RiskLevel::High);
    assert_eq!(crit.risk_level(), RiskLevel::Critical);
}
