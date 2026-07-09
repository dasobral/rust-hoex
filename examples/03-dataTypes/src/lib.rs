//! Shared helpers for the data-types example.
//!
//! Parses a simplified IPv4-like header from a fixed byte array and exposes
//! typed fields for demos, unit tests, and integration tests.

/// Fixed size of our educational packet header (bytes).
pub const HEADER_LEN: usize = 20;

/// Protocol numbers used in the demo header (subset of IANA IP protocol numbers).
pub const PROTO_ICMP: u8 = 1;
pub const PROTO_TCP: u8 = 6;
pub const PROTO_UDP: u8 = 17;

/// Parsed view of a simplified IPv4-like header.
///
/// Field widths mirror common wire formats so learners see why Rust offers
/// many integer sizes (`u8`, `u16`, `u32`, …).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PacketHeader {
    /// IP version (typically 4).
    pub version: u8,
    /// Internet Header Length in 32-bit words (typically 5 => 20 bytes).
    pub ihl: u8,
    /// Differentiated Services / Type of Service.
    pub tos: u8,
    /// Total length of the packet in bytes.
    pub total_length: u16,
    /// Identification for fragmentation reassembly.
    pub identification: u16,
    /// Time To Live — hop count before discard.
    pub ttl: u8,
    /// Protocol number (e.g. 6 = TCP, 17 = UDP).
    pub protocol: u8,
    /// Header checksum (stored as-is; not validated here).
    pub checksum: u16,
    /// Source IPv4 address as four octets.
    pub src: [u8; 4],
    /// Destination IPv4 address as four octets.
    pub dst: [u8; 4],
}

/// Build a demo IPv4-like header as a fixed-size byte array.
///
/// Layout (big-endian multi-byte fields):
/// ```text
///  0: version(4)|ihl(4)   1: TOS
///  2-3: total length      4-5: identification
///  6-7: flags/frag (0)    8: TTL   9: protocol
/// 10-11: checksum        12-15: src   16-19: dst
/// ```
#[must_use]
pub fn sample_header_bytes() -> [u8; HEADER_LEN] {
    // Numeric literals: hex (`0x`), binary (`0b`), and decimal with `_` separators.
    let version_ihl: u8 = 0x45; // version=4, ihl=5
    let tos: u8 = 0b0000_0000;
    let total_length: u16 = 60;
    let identification: u16 = 0x1A2B;
    let ttl: u8 = 64;
    let protocol: u8 = PROTO_TCP;
    let checksum: u16 = 0xBEEF;
    let src: [u8; 4] = [192, 168, 1, 10];
    let dst: [u8; 4] = [8, 8, 8, 8];

    let mut bytes = [0_u8; HEADER_LEN];
    bytes[0] = version_ihl;
    bytes[1] = tos;
    // Split u16 into high/low bytes with `as` casts (truncating to u8).
    #[allow(clippy::cast_possible_truncation)]
    {
        bytes[2] = (total_length >> 8) as u8;
        bytes[3] = (total_length & 0xFF) as u8;
        bytes[4] = (identification >> 8) as u8;
        bytes[5] = (identification & 0xFF) as u8;
        // bytes[6..=7] left as zero (flags / fragment offset)
        bytes[8] = ttl;
        bytes[9] = protocol;
        bytes[10] = (checksum >> 8) as u8;
        bytes[11] = (checksum & 0xFF) as u8;
    }
    bytes[12..16].copy_from_slice(&src);
    bytes[16..20].copy_from_slice(&dst);
    bytes
}

/// Read a big-endian `u16` from two consecutive bytes.
#[must_use]
pub const fn read_u16_be(hi: u8, lo: u8) -> u16 {
    // Widen each byte to u16, then combine. `as` is an explicit cast.
    (hi as u16) << 8 | (lo as u16)
}

/// Parse a [`PacketHeader`] from a 20-byte buffer.
///
/// Returns `None` if the version/IHL nibble encoding is invalid for this demo
/// (version must be 4, IHL must be at least 5).
#[must_use]
pub const fn parse_header(bytes: &[u8; HEADER_LEN]) -> Option<PacketHeader> {
    let version = bytes[0] >> 4;
    let ihl = bytes[0] & 0x0F;
    if version != 4 || ihl < 5 {
        return None;
    }

    Some(PacketHeader {
        version,
        ihl,
        tos: bytes[1],
        total_length: read_u16_be(bytes[2], bytes[3]),
        identification: read_u16_be(bytes[4], bytes[5]),
        ttl: bytes[8],
        protocol: bytes[9],
        checksum: read_u16_be(bytes[10], bytes[11]),
        src: [bytes[12], bytes[13], bytes[14], bytes[15]],
        dst: [bytes[16], bytes[17], bytes[18], bytes[19]],
    })
}

/// Map a protocol number to a single-character label for display.
///
/// `char` is a Unicode scalar value (4 bytes in Rust), not a C `char`.
#[must_use]
pub const fn protocol_label(protocol: u8) -> char {
    match protocol {
        PROTO_ICMP => 'I',
        PROTO_TCP => 'T',
        PROTO_UDP => 'U',
        _ => '?',
    }
}

/// Human-readable protocol name (for demos).
#[must_use]
pub const fn protocol_name(protocol: u8) -> &'static str {
    match protocol {
        PROTO_ICMP => "ICMP",
        PROTO_TCP => "TCP",
        PROTO_UDP => "UDP",
        _ => "UNKNOWN",
    }
}

/// Format an IPv4 address array as dotted decimal.
#[must_use]
pub fn format_ipv4(octets: [u8; 4]) -> String {
    format!("{}.{}.{}.{}", octets[0], octets[1], octets[2], octets[3])
}

/// Return `(src, dst, protocol_label)` as a tuple — compound type demo.
#[must_use]
pub const fn endpoint_summary(header: &PacketHeader) -> ([u8; 4], [u8; 4], char) {
    (header.src, header.dst, protocol_label(header.protocol))
}

/// Estimate remaining path budget from TTL using a float hop-cost model.
///
/// Teaching point: `f32` / `f64` for approximate math; integers for wire data.
#[must_use]
pub const fn ttl_budget_ratio(ttl: u8, initial_ttl: u8) -> f64 {
    if initial_ttl == 0 {
        return 0.0;
    }
    (ttl as f64) / (initial_ttl as f64)
}

/// Whether the packet looks like internal LAN traffic (simple heuristic).
#[must_use]
pub const fn is_private_src(src: [u8; 4]) -> bool {
    // 10.0.0.0/8, 172.16.0.0/12, 192.168.0.0/16
    src[0] == 10
        || (src[0] == 172 && src[1] >= 16 && src[1] <= 31)
        || (src[0] == 192 && src[1] == 168)
}

#[cfg(test)]
#[allow(clippy::expect_used)]
mod tests {
    use super::*;

    #[test]
    fn sample_header_parses() {
        let bytes = sample_header_bytes();
        let header = parse_header(&bytes).expect("sample header should parse");
        assert_eq!(header.version, 4);
        assert_eq!(header.ihl, 5);
        assert_eq!(header.total_length, 60);
        assert_eq!(header.identification, 0x1A2B);
        assert_eq!(header.ttl, 64);
        assert_eq!(header.protocol, PROTO_TCP);
        assert_eq!(header.checksum, 0xBEEF);
        assert_eq!(header.src, [192, 168, 1, 10]);
        assert_eq!(header.dst, [8, 8, 8, 8]);
    }

    #[test]
    fn read_u16_be_combines_bytes() {
        assert_eq!(read_u16_be(0x12, 0x34), 0x1234);
        assert_eq!(read_u16_be(0x00, 0xFF), 255);
    }

    #[test]
    fn invalid_version_rejected() {
        let mut bytes = sample_header_bytes();
        bytes[0] = 0x65; // version 6
        assert!(parse_header(&bytes).is_none());
    }

    #[test]
    fn protocol_label_and_name() {
        assert_eq!(protocol_label(PROTO_TCP), 'T');
        assert_eq!(protocol_name(PROTO_UDP), "UDP");
        assert_eq!(protocol_label(99), '?');
    }

    #[test]
    fn endpoint_summary_tuple() {
        let header = parse_header(&sample_header_bytes()).expect("parse");
        let (src, dst, label) = endpoint_summary(&header);
        assert_eq!(src[0], 192);
        assert_eq!(dst, [8, 8, 8, 8]);
        assert_eq!(label, 'T');
    }

    #[test]
    fn ttl_budget_and_private_check() {
        assert!((ttl_budget_ratio(32, 64) - 0.5).abs() < f64::EPSILON);
        assert!(is_private_src([192, 168, 0, 1]));
        assert!(!is_private_src([8, 8, 8, 8]));
    }

    #[test]
    fn format_ipv4_dotted() {
        assert_eq!(format_ipv4([127, 0, 0, 1]), "127.0.0.1");
    }

    #[test]
    fn numeric_literal_styles() {
        let hex: u32 = 0xDEAD_BEEF;
        let bin: u8 = 0b1010_1010;
        let dec: u64 = 1_000_000;
        assert_eq!(hex, 3_735_928_559);
        assert_eq!(bin, 170);
        assert_eq!(dec, 1_000_000);
    }
}
