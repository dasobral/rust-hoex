//! Borrowing helpers: analyze and update data without taking ownership.
//!
//! Continues the security theme — inspect passwords via `&str`, update a
//! strength score via `&mut i32`, and work with slices (`&[T]`, `&str`).

/// Count ASCII letters in a password without taking ownership.
#[must_use]
pub fn count_ascii_letters(password: &str) -> usize {
    password.chars().filter(char::is_ascii_alphabetic).count()
}

/// Return whether the password meets a minimum length (immutable borrow).
#[must_use]
pub const fn meets_min_length(password: &str, min: usize) -> bool {
    password.len() >= min
}

/// Summarize password traits using multiple immutable borrows of the same data.
#[must_use]
pub fn password_summary(password: &str) -> (usize, usize, bool) {
    let len = password.len();
    let letters = count_ascii_letters(password);
    let ok = meets_min_length(password, 8);
    (len, letters, ok)
}

/// Increase a mutable strength score based on password characteristics.
///
/// Demonstrates `&mut T`: exclusive mutable access while the borrow is live.
pub fn update_strength_score(score: &mut i32, password: &str) {
    *score = 0;
    if meets_min_length(password, 8) {
        *score += 10;
    }
    if password.chars().any(|c| c.is_ascii_uppercase()) {
        *score += 5;
    }
    if password.chars().any(|c| c.is_ascii_digit()) {
        *score += 5;
    }
    if password.chars().any(|c| !c.is_ascii_alphanumeric()) {
        *score += 10;
    }
}

/// First `n` bytes of a password as a string slice (may be mid-char for non-ASCII).
///
/// Returns `None` if `n` is past the end. For teaching we keep it byte-oriented.
#[must_use]
pub fn password_prefix(password: &str, n: usize) -> Option<&str> {
    password.get(..n)
}

/// Average of integer samples via a shared slice borrow.
#[must_use]
pub fn average_scores(scores: &[i32]) -> Option<i32> {
    if scores.is_empty() {
        return None;
    }
    let sum: i32 = scores.iter().sum();
    let len = i32::try_from(scores.len()).ok()?;
    Some(sum / len)
}

/// Mask all but the last `keep` characters, returning an owned display string.
///
/// Borrows the input; allocates only the masked result.
#[must_use]
pub fn mask_password(password: &str, keep: usize) -> String {
    let chars: Vec<char> = password.chars().collect();
    if chars.len() <= keep {
        return "*".repeat(chars.len());
    }
    let hide = chars.len() - keep;
    let mut out = String::with_capacity(password.len());
    out.extend(std::iter::repeat_n('*', hide));
    out.extend(chars[hide..].iter().copied());
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_ascii_letters_works() {
        assert_eq!(count_ascii_letters("Ab12!"), 2);
        assert_eq!(count_ascii_letters(""), 0);
    }

    #[test]
    fn meets_min_length_checks() {
        assert!(meets_min_length("abcdefgh", 8));
        assert!(!meets_min_length("short", 8));
    }

    #[test]
    fn password_summary_combines_borrows() {
        let (len, letters, ok) = password_summary("Abcdef12");
        assert_eq!(len, 8);
        assert_eq!(letters, 6);
        assert!(ok);
    }

    #[test]
    fn update_strength_score_mutates() {
        let mut score = 0;
        update_strength_score(&mut score, "GoodPass1!");
        assert_eq!(score, 30); // 10 + 5 + 5 + 10
    }

    #[test]
    fn password_prefix_returns_slice() {
        assert_eq!(password_prefix("secret", 3), Some("sec"));
        assert_eq!(password_prefix("ab", 5), None);
    }

    #[test]
    fn average_scores_on_slice() {
        assert_eq!(average_scores(&[10, 20, 30]), Some(20));
        assert_eq!(average_scores(&[]), None);
    }

    #[test]
    fn mask_password_hides_prefix() {
        assert_eq!(mask_password("hunter2", 2), "*****r2");
        assert_eq!(mask_password("ab", 5), "**");
    }
}
