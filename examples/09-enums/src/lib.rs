//! Enums and pattern matching for auth, network events, and HTTP-like status.
//!
//! This library shows:
//! - unit, tuple, and struct-like enum variants
//! - exhaustive `match`
//! - a custom `Option`-like enum (`MaybeToken`)
//! - methods on enums via `impl`
//! - `if let` / `while let` for focused binding

/// Outcome of an authentication attempt.
///
/// Demonstrates **struct-like** variants (`Success`, `Failure`) and a **unit**
/// variant (`Pending`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AuthStatus {
    /// Credentials accepted; carry the authenticated username.
    Success { user: String },
    /// Credentials rejected; carry a human-readable reason.
    Failure { reason: String },
    /// Still waiting on a second factor or remote `IdP`.
    Pending,
}

impl AuthStatus {
    /// Create a successful auth result.
    pub fn success(user: impl Into<String>) -> Self {
        Self::Success { user: user.into() }
    }

    /// Create a failed auth result.
    pub fn failure(reason: impl Into<String>) -> Self {
        Self::Failure {
            reason: reason.into(),
        }
    }

    /// Whether this status grants access.
    pub const fn is_authenticated(&self) -> bool {
        matches!(self, Self::Success { .. })
    }

    /// Short summary suitable for logs or UI.
    pub fn summary(&self) -> String {
        match self {
            Self::Success { user } => format!("authenticated as {user}"),
            Self::Failure { reason } => format!("denied: {reason}"),
            Self::Pending => "awaiting second factor".to_owned(),
        }
    }
}

/// A network-facing event observed by a simple sensor.
///
/// Mixes **unit**, **tuple**, and **struct-like** variants in one enum.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NetworkEvent {
    /// Link is quiet.
    Idle,
    /// Bytes arrived from a peer: `(source_ip, byte_count)`.
    PacketReceived(String, u32),
    /// Connection closed with an optional reason string.
    ConnectionClosed {
        peer: String,
        reason: Option<String>,
    },
}

impl NetworkEvent {
    /// Human-readable description of the event.
    pub fn describe(&self) -> String {
        match self {
            Self::Idle => "network idle".to_owned(),
            Self::PacketReceived(ip, bytes) => {
                format!("received {bytes} bytes from {ip}")
            }
            Self::ConnectionClosed { peer, reason } => {
                // `if let` shines when you only care about the `Some` arm.
                let mut msg = format!("closed connection to {peer}");
                if let Some(why) = reason {
                    msg.push_str(": ");
                    msg.push_str(why);
                }
                msg
            }
        }
    }

    /// Bytes transferred in this event, if any.
    pub const fn bytes_transferred(&self) -> u32 {
        match self {
            Self::PacketReceived(_, bytes) => *bytes,
            Self::Idle | Self::ConnectionClosed { .. } => 0,
        }
    }
}

/// Simplified HTTP-like response status (unit + tuple variants).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HttpStatus {
    Ok,
    Created,
    BadRequest,
    Unauthorized,
    NotFound,
    /// Server error with an internal code, e.g. `InternalError(503)`.
    InternalError(u16),
}

impl HttpStatus {
    /// Numeric status code, as a web server would emit.
    pub const fn code(self) -> u16 {
        match self {
            Self::Ok => 200,
            Self::Created => 201,
            Self::BadRequest => 400,
            Self::Unauthorized => 401,
            Self::NotFound => 404,
            Self::InternalError(code) => code,
        }
    }

    /// Whether the status is in the 2xx success class.
    pub const fn is_success(self) -> bool {
        matches!(self, Self::Ok | Self::Created)
    }
}

/// A teaching stand-in for `Option<T>` — same shape, custom name.
///
/// Use this to see how `Some`/`None` are just enum variants you can rebuild.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MaybeToken {
    Some(String),
    None,
}

impl MaybeToken {
    /// Wrap a token string, or return `None` if empty.
    pub fn from_raw(raw: &str) -> Self {
        if raw.is_empty() {
            Self::None
        } else {
            Self::Some(raw.to_owned())
        }
    }

    /// Borrow the inner token when present.
    pub const fn as_str(&self) -> Option<&str> {
        match self {
            Self::Some(token) => Some(token.as_str()),
            Self::None => None,
        }
    }

    /// Map a present token through `f`, leave `None` alone.
    pub fn map_token<F>(self, f: F) -> Self
    where
        F: FnOnce(String) -> String,
    {
        match self {
            Self::Some(token) => Self::Some(f(token)),
            Self::None => Self::None,
        }
    }
}

/// Drain trailing packet events from a queue using `while let`, summing bytes.
///
/// Peeks at the end of the queue with a slice pattern, then `pop`s only when the
/// pattern matches. That way a non-packet event is left in place (a bare
/// `while let Some(PacketReceived(..)) = events.pop()` would drop it).
pub fn sum_packet_bytes(events: &mut Vec<NetworkEvent>) -> u32 {
    let mut total: u32 = 0;
    while let [.., NetworkEvent::PacketReceived(_, bytes)] = events.as_slice() {
        total = total.saturating_add(*bytes);
        events.pop();
    }
    total
}

/// Prefer an authenticated user via `if let`; otherwise return a guest label.
pub fn display_name(status: &AuthStatus) -> String {
    if let AuthStatus::Success { user } = status {
        user.clone()
    } else {
        "guest".to_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn auth_success_is_authenticated() {
        let status = AuthStatus::success("alice");
        assert!(status.is_authenticated());
        assert_eq!(status.summary(), "authenticated as alice");
    }

    #[test]
    fn auth_failure_and_pending() {
        let fail = AuthStatus::failure("bad password");
        assert!(!fail.is_authenticated());
        assert!(fail.summary().contains("denied"));

        let pending = AuthStatus::Pending;
        assert!(!pending.is_authenticated());
        assert_eq!(pending.summary(), "awaiting second factor");
    }

    #[test]
    fn network_event_describe_and_bytes() {
        let idle = NetworkEvent::Idle;
        assert_eq!(idle.bytes_transferred(), 0);

        let pkt = NetworkEvent::PacketReceived("10.0.0.2".into(), 64);
        assert_eq!(pkt.bytes_transferred(), 64);
        assert!(pkt.describe().contains("64"));
    }

    #[test]
    fn http_status_codes() {
        assert_eq!(HttpStatus::Ok.code(), 200);
        assert!(HttpStatus::Created.is_success());
        assert!(!HttpStatus::NotFound.is_success());
        assert_eq!(HttpStatus::InternalError(503).code(), 503);
    }

    #[test]
    fn maybe_token_round_trip() {
        assert_eq!(MaybeToken::from_raw(""), MaybeToken::None);
        let token = MaybeToken::from_raw("abc123").map_token(|t| t.to_uppercase());
        assert_eq!(token.as_str(), Some("ABC123"));
    }

    #[test]
    fn while_let_sums_packets_until_other_event() {
        // Drain from the end; non-packet events stay in the queue.
        let mut events = vec![
            NetworkEvent::Idle,
            NetworkEvent::PacketReceived("a".into(), 10),
            NetworkEvent::PacketReceived("b".into(), 5),
        ];
        assert_eq!(sum_packet_bytes(&mut events), 15);
        assert_eq!(events.len(), 1);
        assert_eq!(events[0], NetworkEvent::Idle);
    }

    #[test]
    fn if_let_display_name() {
        assert_eq!(display_name(&AuthStatus::success("bob")), "bob");
        assert_eq!(display_name(&AuthStatus::Pending), "guest");
    }
}
