//! Shared helpers for the hello-world example.
//!
//! In Rust, a crate can expose a **library** (`lib.rs`) that other code — including
//! the binary in `main.rs` and integration tests — can import and reuse.

/// Build a friendly greeting for `name`.
///
/// # Examples
///
/// ```
/// use example_helloworld::greet;
/// assert_eq!(greet("Rust"), "Hello, Rust!");
/// ```
pub fn greet(name: &str) -> String {
    format!("Hello, {name}!")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greet_includes_name() {
        assert_eq!(greet("World"), "Hello, World!");
    }

    #[test]
    fn greet_handles_empty_name() {
        assert_eq!(greet(""), "Hello, !");
    }
}
