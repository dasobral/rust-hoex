//! Integration tests for the `exercise_datatypes` crate.

use datatypes_exercises::{
    TcpPorts, format_ipv4, get_exercise_list, header_words, parse_header, parse_tos,
    protocol_label, run_all, run_exercise, sample_header_bytes, swap_ports,
};

#[test]
fn test_exercise_list_contains_both() {
    let list = get_exercise_list();
    assert_eq!(list.len(), 2);

    let names: Vec<&str> = list.iter().map(|e| e.name).collect();
    assert!(names.contains(&"ports"));
    assert!(names.contains(&"tos_protocol"));

    for exercise in &list {
        assert!(!exercise.description.is_empty());
        assert!(!exercise.concepts.is_empty());
    }
}

#[test]
fn test_run_each_exercise() {
    assert!(run_exercise("ports", false).is_ok());
    assert!(run_exercise("tos_protocol", false).is_ok());
}

#[test]
fn test_run_unknown_exercise_errors() {
    let err = run_exercise("ethernet", false);
    assert!(err.is_err());
    if let Err(e) = err {
        let message = format!("{e}");
        assert!(message.contains("Unknown exercise"));
    }
}

#[test]
fn test_run_all() {
    assert!(run_all(false).is_ok());
}

#[test]
fn test_port_and_tos_api() {
    let flow = TcpPorts {
        source_port: 1234,
        dest_port: 443,
    };
    let reversed = swap_ports(flow);
    assert_eq!(reversed.dest_port, 1234);

    assert_eq!(protocol_label(6), 'T');
    assert_eq!(protocol_label(17), 'U');

    let (dscp, ecn) = parse_tos(0b1111_1100);
    assert_eq!(dscp, 63);
    assert_eq!(ecn, 0);

    assert_eq!(header_words(5), Some(20));

    let bytes = sample_header_bytes();
    if let Some(header) = parse_header(&bytes) {
        assert_eq!(format_ipv4(header.src), "10.0.0.5");
    }
}
