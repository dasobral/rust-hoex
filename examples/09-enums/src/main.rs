//! example: 09-enums
//!
//! Enums and pattern matching with a cybersecurity flavor.
//!
//! Run from this directory:
//! ```bash
//! cargo run
//! ```
//!
//! Key ideas:
//! - enum variants: unit, tuple, and struct-like
//! - exhaustive `match` (the compiler checks every case)
//! - methods on enums via `impl`
//! - `if let` / `while let` for one-pattern binding
//! - a custom `Option`-like enum (`MaybeToken`)

use example_enums::{
    AuthStatus, HttpStatus, MaybeToken, NetworkEvent, display_name, sum_packet_bytes,
};

fn main() {
    println!("=== 09-enums: auth, network events, HTTP status ===\n");

    demo_auth();
    demo_network();
    demo_http();
    demo_maybe_token();
}

fn demo_auth() {
    println!("-- AuthStatus --");
    let outcomes = [
        AuthStatus::success("alice"),
        AuthStatus::failure("invalid MFA code"),
        AuthStatus::Pending,
    ];

    for status in &outcomes {
        // Exhaustive match: omit a variant and the compiler errors.
        let icon = match status {
            AuthStatus::Success { .. } => "[ok]",
            AuthStatus::Failure { .. } => "[no]",
            AuthStatus::Pending => "[..]",
        };
        println!(
            "{icon} {} (user label: {})",
            status.summary(),
            display_name(status)
        );
    }
    println!();
}

fn demo_network() {
    println!("-- NetworkEvent --");
    let mut queue = vec![
        NetworkEvent::ConnectionClosed {
            peer: "10.0.0.9".into(),
            reason: Some("idle timeout".into()),
        },
        NetworkEvent::PacketReceived("10.0.0.2".into(), 128),
        NetworkEvent::PacketReceived("10.0.0.3".into(), 256),
    ];

    // `while let` keeps going while the pattern matches.
    let bytes = sum_packet_bytes(&mut queue);
    println!("drained {bytes} bytes from packet events");
    if let Some(remaining) = queue.first() {
        println!("next event: {}", remaining.describe());
    }
    println!();
}

fn demo_http() {
    println!("-- HttpStatus --");
    let statuses = [
        HttpStatus::Ok,
        HttpStatus::Unauthorized,
        HttpStatus::InternalError(503),
    ];
    for status in statuses {
        let kind = if status.is_success() {
            "success"
        } else {
            "error"
        };
        println!("HTTP {} ({kind})", status.code());
    }
    println!();
}

fn demo_maybe_token() {
    println!("-- MaybeToken (Option-like) --");
    let present = MaybeToken::from_raw("sess-9f3a");
    let missing = MaybeToken::from_raw("");

    // `if let` is ideal when you only care about one variant.
    if let MaybeToken::Some(token) = present {
        println!("session token: {token}");
    }
    match missing {
        MaybeToken::None => println!("no session token provided"),
        MaybeToken::Some(token) => println!("unexpected token: {token}"),
    }
}
