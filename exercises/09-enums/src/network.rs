//! Network and incident event enums.

/// Observed network activity on a monitored segment.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NetworkEvent {
    Idle,
    PacketReceived(String, u32),
    PacketSent(String, u32),
    ConnectionClosed {
        peer: String,
        reason: Option<String>,
    },
}

impl NetworkEvent {
    /// Human-readable description for dashboards.
    #[must_use]
    pub fn describe(&self) -> String {
        match self {
            Self::Idle => "network idle".to_owned(),
            Self::PacketReceived(ip, bytes) => format!("received {bytes} bytes from {ip}"),
            Self::PacketSent(ip, bytes) => format!("sent {bytes} bytes to {ip}"),
            Self::ConnectionClosed { peer, reason } => {
                let mut msg = format!("closed connection to {peer}");
                if let Some(why) = reason {
                    msg.push_str(": ");
                    msg.push_str(why);
                }
                msg
            }
        }
    }

    /// Bytes transferred in this event (0 for idle/close).
    #[must_use]
    pub const fn bytes_transferred(&self) -> u32 {
        match self {
            Self::PacketReceived(_, bytes) | Self::PacketSent(_, bytes) => *bytes,
            Self::Idle | Self::ConnectionClosed { .. } => 0,
        }
    }
}

/// Sum bytes from received and sent packet events in a slice.
#[must_use]
pub fn sum_packet_bytes(events: &[NetworkEvent]) -> u32 {
    events
        .iter()
        .map(NetworkEvent::bytes_transferred)
        .fold(0u32, u32::saturating_add)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn describe_variants() {
        let recv = NetworkEvent::PacketReceived("10.0.0.1".to_owned(), 64);
        assert!(recv.describe().contains("64"));

        let sent = NetworkEvent::PacketSent("10.0.0.2".to_owned(), 32);
        assert!(sent.describe().contains("sent"));
    }

    #[test]
    fn sum_counts_received_and_sent() {
        let events = [
            NetworkEvent::Idle,
            NetworkEvent::PacketReceived("a".to_owned(), 100),
            NetworkEvent::PacketSent("b".to_owned(), 50),
            NetworkEvent::ConnectionClosed {
                peer: "c".to_owned(),
                reason: None,
            },
        ];
        assert_eq!(sum_packet_bytes(&events), 150);
    }
}
