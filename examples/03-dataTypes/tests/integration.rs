//! Integration tests for `example_datatypes`.
//!
//! These link against the library crate so we exercise the public API end-to-end.

#![allow(clippy::expect_used)]

use example_datatypes::{
    HEADER_LEN, PROTO_TCP, PROTO_UDP, endpoint_summary, format_ipv4, is_private_src, parse_header,
    protocol_label, protocol_name, read_u16_be, sample_header_bytes, ttl_budget_ratio,
};

#[test]
fn sample_round_trip_fields() {
    let bytes = sample_header_bytes();
    assert_eq!(bytes.len(), HEADER_LEN);

    let header = parse_header(&bytes).expect("valid sample");
    assert_eq!(header.protocol, PROTO_TCP);
    assert_eq!(protocol_name(header.protocol), "TCP");
    assert_eq!(format_ipv4(header.dst), "8.8.8.8");
}

#[test]
fn endpoint_summary_matches_header() {
    let header = parse_header(&sample_header_bytes()).expect("parse");
    let (src, dst, label) = endpoint_summary(&header);
    assert_eq!(src, header.src);
    assert_eq!(dst, header.dst);
    assert_eq!(label, protocol_label(header.protocol));
}

#[test]
fn udp_label_differs_from_tcp() {
    assert_eq!(protocol_label(PROTO_UDP), 'U');
    assert_ne!(protocol_label(PROTO_UDP), protocol_label(PROTO_TCP));
}

#[test]
fn private_address_heuristic() {
    assert!(is_private_src([10, 0, 0, 1]));
    assert!(is_private_src([172, 16, 5, 5]));
    assert!(is_private_src([192, 168, 100, 2]));
    assert!(!is_private_src([1, 1, 1, 1]));
}

#[test]
fn ttl_ratio_bounds() {
    let ratio = ttl_budget_ratio(16, 64);
    assert!(ratio > 0.0 && ratio < 1.0);
    assert!((ttl_budget_ratio(0, 64) - 0.0).abs() < f64::EPSILON);
}

#[test]
fn read_u16_be_integration() {
    assert_eq!(read_u16_be(0xAB, 0xCD), 0xABCD);
}
