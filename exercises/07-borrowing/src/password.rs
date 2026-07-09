//! Password analysis and policy helpers.

/// Return the first character without taking ownership.
#[must_use]
pub fn first_char(s: &str) -> Option<char> {
    s.chars().next()
}

/// Count ASCII digits in a password.
#[must_use]
pub fn count_digits(s: &str) -> usize {
    s.chars().filter(char::is_ascii_digit).count()
}

/// Return whether a password meets a minimum length policy.
#[must_use]
pub const fn meets_policy(password: &str, min_len: usize) -> bool {
    password.len() >= min_len
}

/// Update a threat score based on password characteristics.
///
/// Awards points for length > 12, uppercase, digit, and symbol characters.
pub fn update_strength(score: &mut i32, password: &str) {
    *score = 0;
    if password.len() > 12 {
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

/// Average threat scores from a shared slice borrow.
#[must_use]
pub fn average_scores(scores: &[i32]) -> Option<i32> {
    if scores.is_empty() {
        return None;
    }
    let sum: i32 = scores.iter().sum();
    let len = i32::try_from(scores.len()).ok()?;
    Some(sum / len)
}

/// Mask all but the last `keep` characters for safe display.
#[must_use]
pub fn mask_keep_last(password: &str, keep: usize) -> String {
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
    fn first_char_returns_first() {
        assert_eq!(first_char("abc"), Some('a'));
        assert_eq!(first_char(""), None);
    }

    #[test]
    fn count_digits_works() {
        assert_eq!(count_digits("Ab12!"), 2);
    }

    #[test]
    fn meets_policy_checks_length() {
        assert!(meets_policy("abcdefgh", 8));
        assert!(!meets_policy("short", 8));
    }

    #[test]
    fn update_strength_scores_features() {
        let mut score = 0;
        update_strength(&mut score, "Tr0ub4dor&3Extra");
        assert_eq!(score, 30);
    }

    #[test]
    fn average_scores_on_slice() {
        assert_eq!(average_scores(&[10, 20, 30]), Some(20));
        assert_eq!(average_scores(&[]), None);
    }

    #[test]
    fn mask_keep_last_hides_prefix() {
        assert_eq!(mask_keep_last("hunter2", 2), "*****r2");
    }
}
