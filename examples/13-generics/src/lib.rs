//! Generic helpers for security tooling: containers, pairs, and `find_max`.
//!
//! # What you will see
//!
//! - Type parameters on functions and structs (`<T>`, `<T, U>`)
//! - Trait bounds: `Display`, `PartialOrd`, `Clone`, `Eq`
//! - A generic `SecureContainer<T>` that wraps a value with a label
//! - A note on **monomorphization**: the compiler generates a concrete copy
//!   of each generic for every type you use (zero-cost abstraction)

use std::fmt::Display;

/// A labeled wrapper around any payload — e.g. a secret, a score, a key id.
///
/// `T` is a **type parameter**. At compile time the compiler monomorphizes
/// this struct into separate concrete types like `SecureContainer<String>`
/// and `SecureContainer<u32>`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SecureContainer<T> {
    label: String,
    value: T,
}

impl<T> SecureContainer<T> {
    /// Wrap `value` under `label`.
    pub fn new(label: impl Into<String>, value: T) -> Self {
        Self {
            label: label.into(),
            value,
        }
    }

    /// Borrow the label.
    #[must_use]
    pub fn label(&self) -> &str {
        &self.label
    }

    /// Borrow the inner value.
    #[must_use]
    pub const fn get(&self) -> &T {
        &self.value
    }

    /// Mutable access to the payload.
    pub const fn get_mut(&mut self) -> &mut T {
        &mut self.value
    }

    /// Consume the container and return the payload.
    #[must_use]
    pub fn into_inner(self) -> T {
        self.value
    }

    /// Replace the payload, returning the old one.
    pub const fn replace(&mut self, new_value: T) -> T {
        std::mem::replace(&mut self.value, new_value)
    }
}

impl<T: Display> SecureContainer<T> {
    /// Format for logs — only available when `T: Display`.
    ///
    /// This `impl` block has an **extra trait bound**. Methods here exist only
    /// for containers whose `T` implements `Display`.
    #[must_use]
    pub fn audit_line(&self) -> String {
        format!("[{}] {}", self.label, self.value)
    }
}

impl<T: Clone> SecureContainer<T> {
    /// Clone the inner value without consuming the container.
    #[must_use]
    pub fn clone_inner(&self) -> T {
        self.value.clone()
    }
}

/// A generic pair of related values (e.g. IP + score, user + role).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Pair<A, B> {
    pub left: A,
    pub right: B,
}

impl<A, B> Pair<A, B> {
    /// Construct a pair.
    pub const fn new(left: A, right: B) -> Self {
        Self { left, right }
    }

    /// Swap the two sides — types flip: `Pair<A, B>` → `Pair<B, A>`.
    #[must_use]
    pub fn swap(self) -> Pair<B, A> {
        Pair {
            left: self.right,
            right: self.left,
        }
    }
}

impl<A: Display, B: Display> Pair<A, B> {
    /// Human-readable `"left=… right=…"`.
    #[must_use]
    pub fn describe(&self) -> String {
        format!("left={} right={}", self.left, self.right)
    }
}

/// Return a reference to the maximum element, or `None` if the slice is empty.
///
/// Bound `T: PartialOrd` lets us compare elements with `>`.
#[must_use]
pub fn find_max<T: PartialOrd>(items: &[T]) -> Option<&T> {
    let mut best: Option<&T> = None;
    for item in items {
        best = match best {
            Some(current) if current >= item => Some(current),
            _ => Some(item),
        };
    }
    best
}

/// Format any displayable value as a tagged sensor reading.
pub fn format_reading<T: Display>(sensor: &str, value: T) -> String {
    format!("{sensor}={value}")
}

/// Keep items that match `predicate` — generic over element type and closure.
pub fn filter_owned<T, F>(items: Vec<T>, mut predicate: F) -> Vec<T>
where
    F: FnMut(&T) -> bool,
{
    items.into_iter().filter(|item| predicate(item)).collect()
}

/// Compare two containers by label (ignores payload). Useful for sorting keys.
#[must_use]
pub fn same_label<T, U>(a: &SecureContainer<T>, b: &SecureContainer<U>) -> bool {
    a.label == b.label
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn container_wraps_and_replaces() {
        let mut c = SecureContainer::new("api-key", String::from("secret"));
        assert_eq!(c.label(), "api-key");
        assert_eq!(c.get(), "secret");
        let old = c.replace(String::from("rotated"));
        assert_eq!(old, "secret");
        assert_eq!(c.into_inner(), "rotated");
    }

    #[test]
    fn display_bound_enables_audit_line() {
        let c = SecureContainer::new("threat-score", 87u32);
        assert_eq!(c.audit_line(), "[threat-score] 87");
    }

    #[test]
    fn clone_inner_requires_clone() {
        let c = SecureContainer::new("nonce", 42u64);
        assert_eq!(c.clone_inner(), 42);
        assert_eq!(*c.get(), 42);
    }

    #[test]
    fn pair_swap_and_describe() {
        let p = Pair::new("10.0.0.1", 9u8);
        assert_eq!(p.describe(), "left=10.0.0.1 right=9");
        let s = p.swap();
        assert_eq!(s.left, 9);
        assert_eq!(s.right, "10.0.0.1");
    }

    #[test]
    fn find_max_on_scores() {
        let scores = [10u32, 55, 42, 90, 3];
        assert_eq!(find_max(&scores), Some(&90));
        let empty: [u32; 0] = [];
        assert_eq!(find_max(&empty), None);
    }

    #[test]
    fn format_reading_is_generic() {
        assert_eq!(format_reading("cpu", 72), "cpu=72");
        assert_eq!(format_reading("iface", "eth0"), "iface=eth0");
    }

    #[test]
    fn filter_owned_keeps_matches() {
        let ips = vec!["10.0.0.1", "192.168.0.1", "10.0.0.2"];
        let private = filter_owned(ips, |ip| ip.starts_with("10."));
        assert_eq!(private, vec!["10.0.0.1", "10.0.0.2"]);
    }

    #[test]
    fn same_label_across_payload_types() {
        let a = SecureContainer::new("session", 1u32);
        let b = SecureContainer::new("session", "abc");
        let c = SecureContainer::new("other", 1u32);
        assert!(same_label(&a, &b));
        assert!(!same_label(&a, &c));
    }
}
