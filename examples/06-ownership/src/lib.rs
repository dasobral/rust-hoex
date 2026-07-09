//! Ownership helpers for secure string / sensitive-data handling.
//!
//! These functions demonstrate Rust's ownership rules in a security context:
//! passwords are moved (not copied), cloned only when intentional, and
//! consumed so the original binding can no longer be used.

/// Report how many bytes a password occupies without taking ownership.
///
/// Accepts `&str` so both `String` and string literals work. This is a
/// brief preview of borrowing — the next example covers it in depth.
#[must_use]
pub const fn password_byte_len(password: &str) -> usize {
    password.len()
}

/// Take ownership of a password and "consume" it (conceptually zeroize by drop).
///
/// After this call, the caller's `String` is gone — it was moved in.
/// Returning the length proves we used the value before it was dropped.
#[must_use]
pub fn consume_password(password: String) -> usize {
    let len = password.len();
    // `password` goes out of scope here and is dropped. In production you
    // would overwrite the heap buffer first (e.g. with the `zeroize` crate).
    drop(password);
    len
}

/// Move a password into a new owned `String` (identity move for teaching).
///
/// Useful in demos: after calling this, the original binding is invalid.
#[must_use]
pub const fn take_ownership(password: String) -> String {
    password
}

/// Explicitly clone a password when a second owned copy is required.
#[must_use]
pub fn clone_password(password: &str) -> String {
    password.to_owned()
}

/// Copy types (`i32`) are duplicated on assignment — no move occurs.
#[must_use]
pub const fn double_score(score: i32) -> i32 {
    score * 2
}

/// Build an owned password from a string slice (heap allocation).
#[must_use]
pub fn owned_password(plain: &str) -> String {
    String::from(plain)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn password_byte_len_counts_bytes() {
        assert_eq!(password_byte_len("secret"), 6);
        assert_eq!(password_byte_len(""), 0);
    }

    #[test]
    fn consume_password_returns_length() {
        let pw = String::from("hunter2");
        assert_eq!(consume_password(pw), 7);
        // `pw` cannot be used here — it was moved into `consume_password`.
    }

    #[test]
    fn take_ownership_moves_string() {
        let original = String::from("moved!");
        let taken = take_ownership(original);
        assert_eq!(taken, "moved!");
    }

    #[test]
    fn clone_password_creates_independent_copy() {
        let a = String::from("clone-me");
        let b = clone_password(&a);
        assert_eq!(a, b);
        assert_eq!(a, "clone-me");
    }

    #[test]
    fn double_score_copies_i32() {
        let score = 21;
        let doubled = double_score(score);
        assert_eq!(doubled, 42);
        assert_eq!(score, 21); // still usable — Copy
    }

    #[test]
    fn owned_password_allocates_string() {
        let s = owned_password("abc");
        assert_eq!(s, "abc");
    }
}
