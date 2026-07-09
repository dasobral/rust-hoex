//! Integration tests for `example_enums`.
//!
//! These link against the library crate and exercise the public API end-to-end.

use example_enums::{
    AuthStatus, HttpStatus, MaybeToken, NetworkEvent, display_name, sum_packet_bytes,
};

#[test]
fn auth_pipeline_labels_users() {
    let ok = AuthStatus::success("carol");
    let bad = AuthStatus::failure("locked account");
    assert_eq!(display_name(&ok), "carol");
    assert_eq!(display_name(&bad), "guest");
    assert!(ok.is_authenticated());
    assert!(!bad.is_authenticated());
}

#[test]
fn network_queue_drains_packets_only() {
    let mut events = vec![
        NetworkEvent::Idle,
        NetworkEvent::PacketReceived("192.168.1.1".into(), 40),
        NetworkEvent::PacketReceived("192.168.1.2".into(), 60),
    ];
    assert_eq!(sum_packet_bytes(&mut events), 100);
    assert_eq!(events, vec![NetworkEvent::Idle]);
}

#[test]
fn http_and_token_public_api() {
    assert_eq!(HttpStatus::Created.code(), 201);
    assert!(HttpStatus::Ok.is_success());

    let token = MaybeToken::from_raw("tok").map_token(|t| format!("Bearer {t}"));
    assert_eq!(token.as_str(), Some("Bearer tok"));
    assert_eq!(MaybeToken::from_raw("").as_str(), None);
}
