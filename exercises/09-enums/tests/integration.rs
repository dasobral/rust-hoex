//! Integration tests for `exercise_enums`.

use enums_exercises::{
    AuthStatus, HttpStatus, MaybeToken, NetworkEvent, get_exercise_list, run_all, run_exercise,
    sum_packet_bytes,
};

#[test]
fn exercise_list_complete() {
    let list = get_exercise_list();
    assert_eq!(list.len(), 2);
    let names: Vec<&str> = list.iter().map(|e| e.name).collect();
    assert!(names.contains(&"auth-status"));
    assert!(names.contains(&"network"));
}

#[test]
fn run_each_exercise() {
    assert!(run_exercise("auth-status", false).is_ok());
    assert!(run_exercise("network", false).is_ok());
}

#[test]
fn run_all_exercises() {
    assert!(run_all(false).is_ok());
}

#[test]
fn auth_status_locked_variant() {
    let status = AuthStatus::Locked {
        until: "2026-07-10".to_owned(),
    };
    assert!(!status.is_authenticated());
    assert!(status.summary().contains("locked"));
}

#[test]
fn sum_packet_bytes_rx_and_tx() {
    let events = [
        NetworkEvent::PacketReceived("1.1.1.1".to_owned(), 64),
        NetworkEvent::PacketSent("8.8.8.8".to_owned(), 36),
        NetworkEvent::Idle,
    ];
    assert_eq!(sum_packet_bytes(&events), 100);
}

#[test]
fn http_from_code_all_required() {
    for code in [200u16, 201, 400, 401, 403, 404, 500] {
        assert!(HttpStatus::from_code(code).is_some());
    }
}

#[test]
fn maybe_token_unwrap_or() {
    let present = MaybeToken::Some("tok".to_owned());
    assert_eq!(present.unwrap_or("default".to_owned()), "tok");

    let absent = MaybeToken::None;
    assert_eq!(absent.unwrap_or("default".to_owned()), "default");
}
