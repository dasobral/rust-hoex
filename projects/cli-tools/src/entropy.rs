//! Shannon-style password entropy estimation.
//!
//! Not cryptographically rigorous — educational and useful for demos.

use std::collections::HashSet;

/// Character classes that contribute to the assumed alphabet size.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CharClass {
    /// `a`–`z` (26)
    Lower,
    /// `A`–`Z` (26)
    Upper,
    /// `0`–`9` (10)
    Digit,
    /// Non-alphanumeric printable ASCII (≈32)
    Symbol,
}

impl CharClass {
    /// Alphabet size contributed by this class.
    #[must_use]
    pub const fn alphabet_size(self) -> u32 {
        match self {
            Self::Lower | Self::Upper => 26,
            Self::Digit => 10,
            Self::Symbol => 32,
        }
    }

    /// Detect which classes appear in `password`.
    #[must_use]
    pub fn detect(password: &str) -> HashSet<Self> {
        password
            .chars()
            .filter_map(|c| {
                if c.is_ascii_lowercase() {
                    Some(Self::Lower)
                } else if c.is_ascii_uppercase() {
                    Some(Self::Upper)
                } else if c.is_ascii_digit() {
                    Some(Self::Digit)
                } else if !c.is_ascii_alphanumeric() && c.is_ascii() {
                    Some(Self::Symbol)
                } else {
                    None
                }
            })
            .collect()
    }
}

/// Result of an entropy estimate.
#[derive(Debug, Clone, PartialEq)]
pub struct EntropyEstimate {
    /// Approximate entropy in bits.
    pub bits: f64,
    /// Character classes found in the password.
    pub classes: HashSet<CharClass>,
    /// Password length in Unicode scalar values.
    pub length: usize,
    /// Assumed alphabet size (sum of class sizes).
    pub alphabet_size: u32,
}

/// Rough Shannon-style entropy: `len * log2(alphabet_size)`.
///
/// Empty passwords yield zero bits. Non-ASCII-only passwords with no detected
/// ASCII classes also yield zero (alphabet unknown under this model).
#[must_use]
pub fn estimate_entropy(password: &str) -> EntropyEstimate {
    let length = password.chars().count();
    let classes = CharClass::detect(password);
    let alphabet_size = classes.iter().map(|c| c.alphabet_size()).sum();

    let bits = if length == 0 || alphabet_size == 0 {
        0.0
    } else if let Ok(len_u32) = u32::try_from(length) {
        f64::from(len_u32) * f64::from(alphabet_size).log2()
    } else {
        f64::INFINITY
    };

    EntropyEstimate {
        bits,
        classes,
        length,
        alphabet_size,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_has_zero_entropy() {
        let est = estimate_entropy("");
        assert!((est.bits - 0.0).abs() < f64::EPSILON);
        assert_eq!(est.length, 0);
        assert!(est.classes.is_empty());
    }

    #[test]
    fn longer_password_has_more_entropy() {
        let short = estimate_entropy("Ab1!");
        let long = estimate_entropy("Ab1!Ab1!Ab1!");
        assert!(long.bits > short.bits);
    }

    #[test]
    fn detects_all_four_classes() {
        let classes = CharClass::detect("Aa1!");
        assert_eq!(classes.len(), 4);
        assert!(classes.contains(&CharClass::Lower));
        assert!(classes.contains(&CharClass::Upper));
        assert!(classes.contains(&CharClass::Digit));
        assert!(classes.contains(&CharClass::Symbol));
    }

    #[test]
    fn lowercase_only_alphabet_is_26() {
        let est = estimate_entropy("hello");
        assert_eq!(est.alphabet_size, 26);
        assert_eq!(est.classes.len(), 1);
    }
}
