//! Network event and HTTP status exercise.

use anyhow::Result;

use crate::auth_status::AuthStatus;
use crate::http_status::HttpStatus;
use crate::maybe_token::MaybeToken;
use crate::network::{NetworkEvent, sum_packet_bytes};

/// Run the network-incident exercise.
pub fn run(verbose: bool) -> Result<()> {
    println!("📡 Network & HTTP — Incident Event Modeling");
    println!();

    let events = [
        NetworkEvent::ConnectionClosed {
            peer: "10.0.0.9".to_owned(),
            reason: Some("idle timeout".to_owned()),
        },
        NetworkEvent::PacketReceived("10.0.0.2".to_owned(), 128),
        NetworkEvent::PacketSent("10.0.0.3".to_owned(), 256),
        NetworkEvent::Idle,
    ];

    println!("Event log:");
    for event in &events {
        println!("  • {}", event.describe());
    }

    let total = sum_packet_bytes(&events);
    println!();
    println!("Total packet bytes (rx+tx): {total}");

    println!();
    println!("HTTP gateway responses:");
    for code in [200u16, 401, 403, 404, 500] {
        if let Some(status) = HttpStatus::from_code(code) {
            let kind = if status.is_success() {
                "success"
            } else {
                "error"
            };
            println!("  HTTP {} ({kind})", status.code());
        }
    }

    let token = MaybeToken::None;
    let bearer = token.unwrap_or("anonymous".to_owned());
    println!();
    println!("Bearer header fallback: {bearer}");

    if verbose {
        let auth = AuthStatus::Failure {
            reason: "expired session".to_owned(),
        };
        println!("Related auth outcome: {}", auth.summary());
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn network_demo_runs() {
        assert!(run(false).is_ok());
    }
}
