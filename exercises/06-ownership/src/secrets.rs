//! Core ownership helpers for secure credential handling.

/// Take ownership of a secret, return its byte length, then drop it.
///
/// After this call the caller's `String` is gone — it was moved in.
#[must_use]
pub fn consume_secret(secret: String) -> usize {
    let len = secret.len();
    drop(secret);
    len
}

/// Overwrite secret bytes with zeros, then drop the buffer.
///
/// Safe alternative to heap scrubbing without `unsafe`; converts to `Vec<u8>`,
/// zeroes each byte, then drops.
#[must_use]
pub fn zeroize_and_consume(secret: String) -> usize {
    let len = secret.len();
    let mut bytes = secret.into_bytes();
    bytes.fill(0);
    drop(bytes);
    len
}

/// Explicitly clone a secret when a second owned copy is required.
#[must_use]
pub fn clone_secret(secret: &str) -> String {
    secret.to_owned()
}

/// Move a secret into a new binding (identity move for teaching).
#[must_use]
pub const fn take_then_return(secret: String) -> String {
    secret
}

/// Copy types (`i32`) duplicate on pass-by-value — the original stays usable.
#[must_use]
pub const fn copy_threat_score(score: i32) -> i32 {
    score.saturating_mul(2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn consume_secret_returns_length_and_drops() {
        let secret = String::from("hunter2");
        assert_eq!(consume_secret(secret), 7);
    }

    #[test]
    fn zeroize_and_consume_scrubs_then_drops() {
        let secret = String::from("scrub-me");
        assert_eq!(zeroize_and_consume(secret), 8);
    }

    #[test]
    fn clone_secret_creates_independent_copy() {
        let original = String::from("vault-key");
        let cloned = clone_secret(&original);
        assert_eq!(original, cloned);
        assert_eq!(cloned, "vault-key");
    }

    #[test]
    fn take_then_return_moves_identity() {
        let secret = String::from("handoff");
        let returned = take_then_return(secret);
        assert_eq!(returned, "handoff");
    }

    #[test]
    fn copy_threat_score_leaves_original_usable() {
        let score = 21;
        let doubled = copy_threat_score(score);
        assert_eq!(doubled, 42);
        assert_eq!(score, 21);
    }
}
