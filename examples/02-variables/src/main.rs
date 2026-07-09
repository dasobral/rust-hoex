//! example: 02-variables
//!
//! Cryptographic entropy and password strength analysis.
//!
//! Run from this directory:
//! ```bash
//! cargo run
//! ```
//!
//! Key concepts:
//! - Variable declaration and mutability
//! - Type inference and explicit annotations
//! - Constants vs variables
//! - Shadowing and scope
//! - Numeric types (integers, floats), strings, and booleans

use std::collections::HashMap;

/// Minimum secure password-space entropy in bits (aligned with common guidance).
const MIN_SECURE_ENTROPY: f64 = 50.0;
/// Maximum password length we will analyze (characters).
const MAX_PASSWORD_LENGTH: usize = 256;

fn main() {
    println!("\nCryptographic Entropy Calculator");
    println!("=====================================\n");

    // === Variable Declaration and Type Inference ===

    // Immutable by default — cannot change after assignment
    let test_password = "!Pep3ThyFr0g7"; // inferred as &str
    let password_length = test_password.len(); // inferred as usize

    // Explicit type annotations when clarity helps
    let entropy_threshold: f64 = MIN_SECURE_ENTROPY;
    let is_secure: bool = false; // placeholder; shadowed after analysis

    println!("Analyzing password: \"{test_password}\"");
    println!("Length: {password_length} characters");
    println!("Minimum secure entropy threshold: {MIN_SECURE_ENTROPY:.2} bits");
    println!("Current entropy threshold: {entropy_threshold:.2} bits");
    println!("Maximum password length: {MAX_PASSWORD_LENGTH} characters");
    println!("Initial security flag (before analysis): {is_secure}");

    // === Mutability ===
    let mut char_counts: HashMap<char, u32> = HashMap::new();
    let mut complexity_score: u32 = 0;

    // === Character Set Analysis ===
    let mut has_lowercase = false;
    let mut has_uppercase = false;
    let mut has_digits = false;
    let mut has_symbols = false;

    for ch in test_password.chars() {
        // Update character frequency map
        let count = char_counts.entry(ch).or_insert(0);
        *count += 1;

        if ch.is_ascii_lowercase() {
            has_lowercase = true;
        } else if ch.is_ascii_uppercase() {
            has_uppercase = true;
        } else if ch.is_ascii_digit() {
            has_digits = true;
        } else if ch.is_ascii_punctuation() {
            has_symbols = true;
        }
    }

    // Assigned once after the loop (avoids unused_assignments on an initial 0)
    let unique_chars = char_counts.len();

    // === Alphabet Size Calculation ===
    let mut alphabet_size: u32 = 0;
    if has_lowercase {
        alphabet_size += 26; // a-z
        complexity_score += 1;
    }
    if has_uppercase {
        alphabet_size += 26; // A-Z
        complexity_score += 1;
    }
    if has_digits {
        alphabet_size += 10; // 0-9
        complexity_score += 1;
    }
    if has_symbols {
        alphabet_size += 32; // common symbols
        complexity_score += 2;
    }

    println!("\nCharacter Set Analysis:");
    println!("- Lowercase letters: {has_lowercase}");
    println!("- Uppercase letters: {has_uppercase}");
    println!("- Digits: {has_digits}");
    println!("- Symbols: {has_symbols}");
    println!("- Unique characters: {unique_chars}");
    println!("- Alphabet size: {alphabet_size}");
    println!("- Complexity score: {complexity_score}");

    // === Entropy Calculation (Shannon) ===
    let mut shannon_entropy = 0.0;
    // Password lengths in this example are tiny; cast is intentional for teaching.
    #[allow(clippy::cast_precision_loss)]
    let total_chars = password_length as f64;

    // `&` borrows map values without taking ownership
    for &count in char_counts.values() {
        let probability = f64::from(count) / total_chars;
        if probability > 0.0 {
            shannon_entropy = probability.mul_add(-probability.log2(), shannon_entropy);
        }
    }

    let pass_space_entropy = total_chars * f64::from(alphabet_size).log2();

    // === Variable Shadowing ===
    // Reuse the name with a new immutable binding after computation
    let entropy_threshold = if complexity_score >= 3 {
        MIN_SECURE_ENTROPY * 0.8
    } else {
        MIN_SECURE_ENTROPY * 1.2
    };

    let is_secure = pass_space_entropy >= entropy_threshold;

    println!("\nEntropy Analysis:");
    println!("- Shannon entropy: {shannon_entropy:.2} bits");
    println!("- Password space entropy: {pass_space_entropy:.2} bits");
    println!("- Security threshold: {entropy_threshold:.2} bits");
    println!("- Complexity score: {complexity_score}");

    if is_secure {
        println!("\nPassword meets security requirements!");
    } else {
        println!("\nPassword is NOT secure enough!");

        let recommended_f64 = (entropy_threshold / f64::from(alphabet_size).log2()).ceil();
        // ceil() yields a non-negative finite value here; truncate to usize for display.
        #[allow(
            clippy::cast_possible_truncation,
            clippy::cast_sign_loss,
            clippy::cast_precision_loss
        )]
        let recommended_length = recommended_f64 as usize;
        println!("Recommended minimum length: {recommended_length} characters");
    }

    // === Common pattern check ===
    let password_lower = test_password.to_lowercase();
    let has_common_patterns = password_lower.contains("123")
        || password_lower.contains("abc")
        || password_lower.contains("password");

    if has_common_patterns {
        println!("Warning: Contains common patterns");
    }

    // === Different numeric types ===
    let brute_force_attempts_per_second: u64 = 1_000_000_000;
    let total_combinations: u128 =
        u128::from(alphabet_size).pow(u32::try_from(password_length).unwrap_or(u32::MAX));

    let seconds_to_crack = total_combinations / (u128::from(brute_force_attempts_per_second) * 2);
    // Approximate years for display; u128→f64 loses precision for huge values by design.
    #[allow(clippy::cast_precision_loss)]
    let years_to_crack = seconds_to_crack as f64 / (365.25 * 24.0 * 3600.0);

    println!("\nBrute Force Analysis:");
    println!("- Total possible combinations: {total_combinations}");
    println!("- Estimated time to crack: {years_to_crack:.2e} years");

    // === Memory sizes (small fixed widths) ===
    let password_bytes: usize = test_password.len();
    let hash_size_sha256: u8 = 32;
    let key_size_aes256: u8 = 32;

    println!("\nMemory Usage:");
    println!("- Password storage: {password_bytes} bytes");
    println!("- SHA-256 hash: {hash_size_sha256} bytes");
    println!("- AES-256 key: {key_size_aes256} bytes");
}

#[cfg(test)]
mod tests {
    use super::MAX_PASSWORD_LENGTH;
    use super::MIN_SECURE_ENTROPY;

    #[test]
    fn test_constants_are_positive() {
        // Bind to locals so clippy does not flag assertions_on_constants.
        let min_entropy = MIN_SECURE_ENTROPY;
        let max_len = MAX_PASSWORD_LENGTH;
        assert!(min_entropy > 0.0);
        assert!(max_len > 0);
        assert!(min_entropy < 1000.0);
        assert!((min_entropy - 50.0).abs() < f64::EPSILON);
        assert_eq!(max_len, 256);
    }

    #[test]
    fn test_password_analysis() {
        let test_pw = "Abc123!";
        assert_eq!(test_pw.len(), 7);

        let has_lower = test_pw.chars().any(|c| c.is_ascii_lowercase());
        let has_upper = test_pw.chars().any(|c| c.is_ascii_uppercase());
        let has_digit = test_pw.chars().any(|c| c.is_ascii_digit());
        let has_symbol = test_pw.chars().any(|c| c.is_ascii_punctuation());

        assert!(has_lower);
        assert!(has_upper);
        assert!(has_digit);
        assert!(has_symbol);
    }

    #[test]
    fn test_alphabet_size_calculation() {
        let has_all_types = true;
        let size = if has_all_types {
            26 + 26 + 10 + 32 // lower + upper + digits + symbols
        } else {
            0
        };

        assert_eq!(size, 94);
    }

    #[test]
    fn test_numeric_types() {
        let large_number: u128 = u128::MAX;
        let small_number: u8 = 255;
        let float_precision: f64 = 1.234_567_890_123_45;

        assert!(large_number > 0);
        assert_eq!(small_number, 255);
        assert!((float_precision - 1.234_567_890_123_45).abs() < f64::EPSILON);
    }
}
