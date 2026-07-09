//! Scoring helpers — `pub(crate)` so only sibling modules call them directly.

/// Bit flags for ASCII character classes.
#[derive(Debug, Default, Clone, Copy)]
struct ClassFlags(u8);

impl ClassFlags {
    const LOWER: u8 = 0b0001;
    const UPPER: u8 = 0b0010;
    const DIGIT: u8 = 0b0100;
    const SYMBOL: u8 = 0b1000;

    const fn observe(mut self, ch: char) -> Self {
        if ch.is_ascii_lowercase() {
            self.0 |= Self::LOWER;
        } else if ch.is_ascii_uppercase() {
            self.0 |= Self::UPPER;
        } else if ch.is_ascii_digit() {
            self.0 |= Self::DIGIT;
        } else {
            self.0 |= Self::SYMBOL;
        }
        self
    }

    fn count(self) -> u8 {
        u8::from(self.0 & Self::LOWER != 0)
            + u8::from(self.0 & Self::UPPER != 0)
            + u8::from(self.0 & Self::DIGIT != 0)
            + u8::from(self.0 & Self::SYMBOL != 0)
    }
}

/// Scan `secret` and return whether at least two character classes appear.
pub(crate) fn has_mixed_classes(secret: &str) -> bool {
    let flags = secret
        .chars()
        .fold(ClassFlags::default(), ClassFlags::observe);
    flags.count() >= 2
}

/// Compute a 0–100 strength score before common-password capping.
pub(crate) fn compute_score(secret: &str) -> u8 {
    if secret.is_empty() {
        return 0;
    }

    let len = secret.len();
    let flags = secret
        .chars()
        .fold(ClassFlags::default(), ClassFlags::observe);
    let class_bonus = u16::from(flags.count()) * 12;
    let capped_len = u16::try_from(len.min(20)).unwrap_or(20);
    let length_bonus = capped_len * 3;
    let variety = if has_mixed_classes(secret) { 16 } else { 0 };

    let raw = class_bonus + length_bonus + variety;
    u8::try_from(raw.min(100)).unwrap_or(100)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_scores_zero() {
        assert_eq!(compute_score(""), 0);
        assert!(!has_mixed_classes(""));
    }

    #[test]
    fn mixed_classes_detected() {
        assert!(has_mixed_classes("aA"));
        assert!(!has_mixed_classes("aaaa"));
    }

    #[test]
    fn longer_mixed_secret_scores_higher() {
        let weak = compute_score("aaaa");
        let stronger = compute_score("Aa1!xxxx");
        assert!(stronger > weak);
    }
}
