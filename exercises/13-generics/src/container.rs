//! Generic labeled container for secrets and sensor readings.

/// A labeled wrapper around any payload — e.g. API key, threat score, nonce.
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

    /// Transform the payload, keeping the label.
    #[must_use]
    pub fn map<U, F>(self, f: F) -> SecureContainer<U>
    where
        F: FnOnce(T) -> U,
    {
        SecureContainer {
            label: self.label,
            value: f(self.value),
        }
    }
}

impl<T: Default> SecureContainer<T> {
    /// Reset the payload to `T::default()` (e.g. zeroize a buffer in demos).
    pub fn clear(&mut self) {
        self.value = T::default();
    }
}

/// Run the container demo.
pub fn run(verbose: bool) -> crate::Result<()> {
    use crate::SecureContainer;

    println!("SecureContainer<T> demo\n");

    let key = SecureContainer::new("api-key", String::from("sk_live_demo"));
    println!("  label: {}", key.label());
    println!("  value: {}", key.get());

    let mapped = key.map(|s| s.len());
    println!("  mapped length: {}", mapped.into_inner());

    let mut buffer = SecureContainer::new("scratch", vec![1_u8, 2, 3]);
    if verbose {
        println!("  before clear: {:?}", buffer.get());
    }
    buffer.clear();
    println!("  after clear: {:?}", buffer.get());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn container_basic_ops() {
        let mut c = SecureContainer::new("token", String::from("abc"));
        assert_eq!(c.label(), "token");
        assert_eq!(c.get(), "abc");
        c.get_mut().push('!');
        assert_eq!(c.into_inner(), "abc!");
    }

    #[test]
    fn map_transforms_type() {
        let c = SecureContainer::new("score", 42_u32);
        let mapped = c.map(|n| n * 2);
        assert_eq!(mapped.into_inner(), 84);
    }

    #[test]
    fn clear_resets_default() {
        let mut c = SecureContainer::new("buf", vec![9_u8]);
        c.clear();
        assert!(c.get().is_empty());
    }

    #[test]
    fn demo_runs() {
        assert!(run(false).is_ok());
    }
}
