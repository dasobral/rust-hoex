//! Individual password policy rules — each has focused unit tests.

/// Minimum length required by the default policy.
pub const MIN_LENGTH: usize = 8;

/// Return `true` when `password` meets the minimum length rule.
#[must_use]
pub const fn has_min_length(password: &str) -> bool {
    password.len() >= MIN_LENGTH
}

/// Return `true` when `password` contains an ASCII digit.
#[must_use]
pub fn has_digit(password: &str) -> bool {
    password.chars().any(|c| c.is_ascii_digit())
}

/// Return `true` when `password` contains an ASCII uppercase letter.
#[must_use]
pub fn has_uppercase(password: &str) -> bool {
    password.chars().any(|c| c.is_ascii_uppercase())
}

/// Return `true` when `password` contains ASCII punctuation.
#[must_use]
pub fn has_punctuation(password: &str) -> bool {
    password.chars().any(|c| c.is_ascii_punctuation())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn min_length_boundary() {
        assert!(!has_min_length("Ab1!"));
        assert!(has_min_length("Ab1!Ab1!"));
    }

    #[test]
    fn digit_rule() {
        assert!(has_digit("Vault1"));
        assert!(!has_digit("Vault"));
    }

    #[test]
    fn uppercase_rule() {
        assert!(has_uppercase("Vault1"));
        assert!(!has_uppercase("vault1"));
    }

    #[test]
    fn punctuation_rule() {
        assert!(has_punctuation("Vault!"));
        assert!(!has_punctuation("Vault1"));
    }
}
