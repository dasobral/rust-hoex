//! Packet header scalars and compound types for cybersecurity demos.

/// IANA protocol numbers used in exercises.
pub const PROTO_TCP: u8 = 6;
pub const PROTO_UDP: u8 = 17;

/// Fixed size of our educational IPv4-like header (bytes).
pub const HEADER_LEN: usize = 20;

/// TCP endpoint pair — compound struct of two `u16` port scalars.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TcpPorts {
    /// Source port on the initiating host.
    pub source_port: u16,
    /// Destination port on the target service.
    pub dest_port: u16,
}

/// Swap source and destination ports (useful when reversing flow direction).
#[must_use]
pub const fn swap_ports(ports: TcpPorts) -> TcpPorts {
    TcpPorts {
        source_port: ports.dest_port,
        dest_port: ports.source_port,
    }
}

/// Whether a port is in the well-known range 0..=1023.
#[must_use]
pub const fn is_well_known(port: u16) -> bool {
    port <= 1023
}

/// Map a protocol number to a single-character label.
#[must_use]
pub const fn protocol_label(proto: u8) -> char {
    match proto {
        PROTO_TCP => 'T',
        PROTO_UDP => 'U',
        _ => '?',
    }
}

/// Convert IHL (32-bit words) to header length in bytes; `None` on overflow.
#[must_use]
pub const fn header_words(ihl: u8) -> Option<u16> {
    match ihl.checked_mul(4) {
        Some(n) => Some(n as u16),
        None => None,
    }
}

/// Split Type-of-Service byte into `(dscp, ecn)` via bit masks.
#[must_use]
pub const fn parse_tos(tos: u8) -> (u8, u8) {
    let dscp = tos >> 2;
    let ecn = tos & 0b11;
    (dscp, ecn)
}

/// Read a big-endian `u16` from two consecutive bytes.
#[must_use]
pub const fn read_u16_be(hi: u8, lo: u8) -> u16 {
    (hi as u16) << 8 | (lo as u16)
}

/// Parsed view of a simplified IPv4-like header.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PacketHeader {
    /// IP version (typically 4).
    pub version: u8,
    /// Internet Header Length in 32-bit words.
    pub ihl: u8,
    /// Type of Service / Differentiated Services field.
    pub tos: u8,
    /// Total length of the packet in bytes.
    pub total_length: u16,
    /// Time To Live.
    pub ttl: u8,
    /// Protocol number (6 = TCP, 17 = UDP).
    pub protocol: u8,
    /// Header checksum.
    pub checksum: u16,
    /// Source IPv4 address as four octets.
    pub src: [u8; 4],
    /// Destination IPv4 address as four octets.
    pub dst: [u8; 4],
}

/// Build demo IPv4-like header bytes for tuple/array exercises.
#[must_use]
pub fn sample_header_bytes() -> [u8; HEADER_LEN] {
    let version_ihl: u8 = 0x45;
    let tos: u8 = 0b1011_0100; // dscp=45, ecn=0
    let total_length: u16 = 60;
    let ttl: u8 = 64;
    let protocol: u8 = PROTO_TCP;
    let checksum: u16 = 0xBEEF;
    let src: [u8; 4] = [10, 0, 0, 5];
    let dst: [u8; 4] = [192, 168, 1, 1];

    let mut bytes = [0_u8; HEADER_LEN];
    bytes[0] = version_ihl;
    bytes[1] = tos;
    #[allow(clippy::cast_possible_truncation)]
    {
        bytes[2] = (total_length >> 8) as u8;
        bytes[3] = (total_length & 0xFF) as u8;
        bytes[6] = 0;
        bytes[7] = 0;
        bytes[8] = ttl;
        bytes[9] = protocol;
        bytes[10] = (checksum >> 8) as u8;
        bytes[11] = (checksum & 0xFF) as u8;
    }
    bytes[12..16].copy_from_slice(&src);
    bytes[16..20].copy_from_slice(&dst);
    bytes
}

/// Parse a [`PacketHeader`] from a 20-byte buffer.
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
        ttl: bytes[8],
        protocol: bytes[9],
        checksum: read_u16_be(bytes[10], bytes[11]),
        src: [bytes[12], bytes[13], bytes[14], bytes[15]],
        dst: [bytes[16], bytes[17], bytes[18], bytes[19]],
    })
}

/// Return `(src, dst, protocol_label)` as a tuple demo.
#[must_use]
pub const fn endpoint_summary(header: &PacketHeader) -> ([u8; 4], [u8; 4], char) {
    (header.src, header.dst, protocol_label(header.protocol))
}

/// Format an IPv4 address array as dotted decimal.
#[must_use]
pub fn format_ipv4(octets: [u8; 4]) -> String {
    format!("{}.{}.{}.{}", octets[0], octets[1], octets[2], octets[3])
}

/// Human-readable port pair for logs.
#[must_use]
pub fn format_port_pair(ports: TcpPorts) -> String {
    format!("{} -> {}", ports.source_port, ports.dest_port)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn swap_and_well_known_ports() {
        let flow = TcpPorts {
            source_port: 54321,
            dest_port: 443,
        };
        let reversed = swap_ports(flow);
        assert_eq!(reversed.source_port, 443);
        assert_eq!(reversed.dest_port, 54321);
        assert!(is_well_known(443));
        assert!(!is_well_known(8080));
    }

    #[test]
    fn protocol_label_maps_tcp_udp() {
        assert_eq!(protocol_label(PROTO_TCP), 'T');
        assert_eq!(protocol_label(PROTO_UDP), 'U');
        assert_eq!(protocol_label(99), '?');
    }

    #[test]
    fn header_words_no_overflow() {
        assert_eq!(header_words(5), Some(20));
        assert_eq!(header_words(15), Some(60));
        assert!(header_words(64).is_none());
    }

    #[test]
    fn parse_tos_splits_dscp_ecn() {
        let (dscp, ecn) = parse_tos(0b1011_0100);
        assert_eq!(dscp, 45);
        assert_eq!(ecn, 0);
        let (dscp2, ecn2) = parse_tos(0b0000_0011);
        assert_eq!(dscp2, 0);
        assert_eq!(ecn2, 3);
    }

    #[test]
    fn sample_header_parses() {
        let bytes = sample_header_bytes();
        let header = parse_header(&bytes);
        assert!(header.is_some());
        if let Some(h) = header {
            assert_eq!(h.version, 4);
            assert_eq!(h.ihl, 5);
            assert_eq!(h.protocol, PROTO_TCP);
            assert_eq!(h.src, [10, 0, 0, 5]);
        }
    }

    #[test]
    fn endpoint_summary_tuple() {
        let bytes = sample_header_bytes();
        if let Some(header) = parse_header(&bytes) {
            let (src, dst, label) = endpoint_summary(&header);
            assert_eq!(src[0], 10);
            assert_eq!(dst, [192, 168, 1, 1]);
            assert_eq!(label, 'T');
        }
    }

    #[test]
    fn format_helpers() {
        assert_eq!(format_ipv4([127, 0, 0, 1]), "127.0.0.1");
        let ports = TcpPorts {
            source_port: 49152,
            dest_port: 53,
        };
        assert_eq!(format_port_pair(ports), "49152 -> 53");
    }
}
