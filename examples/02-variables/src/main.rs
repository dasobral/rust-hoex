// example: 02-variables
// Cryptographic entropy and password strength analysis
//
// To run this program:
// 1. Navigate to this directory: cd examples/02-variables
// 2. Run the program: cargo run
//
// Key concepts demonstrated:
// - Variable declaration and mutability
// - Type inference and explicit annotations
// - Constants vs variables
// - Shadowing and scope
// - Different data types (integers, floats, strings, booleans)

use std::collections::HashMap;

// Constants - compile time values that never change
const MIN_SECURE_ENTROPY: f64 = 32.0; // Minimum secure entropy in bits
const MAX_PASSWORD_LENGTH: usize = 512; // Maximum password length in bits

fn main() {
    println!("\n🔐 Cryptographic Entropy Calculator");
    println!("=====================================\n");

    // === Variable Declaration and Type Inference ===

    // Immutable variables (default in Rust) cannot change after initial assignment
    let test_password = "!Pep3ThyFr0g7"; // Type inferred as &str
    let password_length = test_password.len(); // Type inferred as usize

    // Explicit type annotations
    let entropy_threshold: f64 = MIN_SECURE_ENTROPY; // Explicitly annotated as f64
    let is_secure: bool = false; // Explicitly annotated as bool, to be updated later

    println!("Analyzing password: \"{test_password}\"");
    println!("Length: {password_length} characters");
    println!(
        "Minimum secure entropy threshold: {MIN_SECURE_ENTROPY:.2} bits"
    );
    println!("Current entropy threshold: {entropy_threshold:.2} bits");
    println!("Maximum password length: {MAX_PASSWORD_LENGTH} bits");

    // === Mutability ===
    // Mutable variables allow for modification after initial assignment
    let mut char_counts = HashMap::new(); // Mutable HashMap to store character counts  
    let mut unique_chars = 0;
    let mut complexity_score = 0;

    // === Character Set Analysis ===
    // Boolean flags for character types. We define them as mutable for convenience
    let mut has_lowercase = false;
    let mut has_uppercase = false;
    let mut has_digits = false;
    let mut has_symbols = false;

    // Analyze each character
    for char in test_password.chars() {
        // Update character fequency map
        let count = char_counts.entry(char).or_insert(0);
        *count += 1;

        // Check character types and update flags
        if char.is_ascii_lowercase() {
            has_lowercase = true;
        } else if char.is_ascii_uppercase() {
            has_uppercase = true;
        } else if char.is_ascii_digit() {
            has_digits = true;
        } else if char.is_ascii_punctuation() {
            has_symbols = true;
        }
    }

    // Calculate unique character count
    unique_chars = char_counts.len();

    // === Alphabet Size Calculation ===

    // Calculate effective alphabet size based on character types used
    let mut alphabet_size = 0;
    if has_lowercase {
        alphabet_size += 26; // a-z
        complexity_score += 1; // Lowercase adds some complexity
    }
    if has_uppercase {
        alphabet_size += 26; // A-Z
        complexity_score += 1; // Uppercase adds some complexity
    }
    if has_digits {
        alphabet_size += 10; // 0-9
        complexity_score += 1; // Digits add some complexity
    }
    if has_symbols {
        alphabet_size += 32; // Common symbols
        complexity_score += 2; // Symbols add some complexity
    }

    println!("\nCharacter Set Analysis:");
    println!("- Lowercase letters: {has_lowercase}");
    println!("- Uppercase letters: {has_uppercase}");
    println!("- Digits: {has_digits}");
    println!("- Symbols: {has_symbols}");
    println!("- Unique characters: {unique_chars}");
    println!("- Alphabet size: {alphabet_size}");
    println!("- Complexity score: {complexity_score}");

    // === Entropy Calculation ===
    // Let's use Shannon entropy from Information Theory
    let mut shannon_entropy = 0.0;
    let total_chars = password_length as f64; // Recast password_length to f64 from usize

    // We use the following operators
    // - The & (Reference) operator allows us to borrow values without taking ownership (reference to a value)
    // - The .log2() method computes the base-2 logarithm of a number
    for &count in char_counts.values() {
        let probability = count as f64 / total_chars;
        if probability > 0.0 {
            shannon_entropy -= probability * probability.log2();
        }
    }

    // Calculate password space entropy (cryptographic strength)
    let pass_space_entropy = total_chars * (alphabet_size as f64).log2();

    // === Variable Shadowing ===
    // Shadowing allows us to reuse variable names, with the new variable "hiding" the old one
    // Shadow the original entropy_threshold with a calculated value
    let entropy_threshold = if complexity_score >= 3 {
        MIN_SECURE_ENTROPY * 0.8 // Lower threshold for higher password complexity
    } else {
        MIN_SECURE_ENTROPY * 1.2 // Higher threshold for simpler passwords
    };

    // Update security assessment
    let is_secure = pass_space_entropy >= entropy_threshold;

    // === Results display ===
    println!("\nEntropy Analysis:");
    println!("- Shannon entropy: {shannon_entropy:.2} bits");
    println!("- Password space entropy: {pass_space_entropy:.2} bits");
    println!("- Security threshold: {entropy_threshold:.2} bits");
    println!("- Complexity score: {complexity_score}");

    if is_secure {
        println!("\n✅ Password meets security requirements!");
    } else {
        println!("\n❌ Password is NOT secure enough!");

        // Calculate recommended minimum length
        let recommended_length =
            (entropy_threshold / (alphabet_size as f64).log2()).ceil() as usize;
        println!(
            "💡 Recommended minimum length: {recommended_length} characters"
        );
    }

    // === Additional Security Analysis ===

    // Check for common patterns (demonstrating string operations)
    let password_lower = test_password.to_lowercase();
    let has_common_patterns = password_lower.contains("123")
        || password_lower.contains("abc")
        || password_lower.contains("password");

    if has_common_patterns {
        println!("⚠️  Warning: Contains common patterns");
    }

    // === Demonstration of Different Numeric Types ===

    // Different integer types for different use cases
    let brute_force_attempts_per_second: u64 = 1_000_000_000; // 1 billion attempts/sec
    let total_combinations: u128 = (alphabet_size as u128).pow(password_length as u32);

    // Calculate time to crack (worst case)
    let seconds_to_crack = total_combinations / (brute_force_attempts_per_second as u128 * 2); // Average case
    let years_to_crack = seconds_to_crack as f64 / (365.25 * 24.0 * 3600.0);

    println!("\nBrute Force Analysis:");
    println!("- Total possible combinations: {total_combinations}");
    println!("- Estimated time to crack: {years_to_crack:.2e} years");

    // === Memory Usage Analysis (demonstrating different numeric types) ===

    let password_bytes: usize = test_password.len();
    let hash_size_sha256: u8 = 32; // SHA-256 produces 32 bytes
    let key_size_aes256: u8 = 32; // AES-256 uses 32-byte keys

    println!("\nMemory Usage:");
    println!("- Password storage: {password_bytes} bytes");
    println!("- SHA-256 hash: {hash_size_sha256} bytes");
    println!("- AES-256 key: {key_size_aes256} bytes");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constants() {
        // Test that our constants are reasonable
        assert!(MIN_SECURE_ENTROPY > 0.0);
        assert!(MAX_PASSWORD_LENGTH > 0);
        assert!(MIN_SECURE_ENTROPY < 1000.0); // Sanity check
    }

    #[test]
    fn test_password_analysis() {
        // Test password with known characteristics
        let test_pw = "Abc123!";
        assert_eq!(test_pw.len(), 7);

        // Verify it contains different character types
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
        // Test alphabet size calculation logic
        let mut size = 0;

        // Simulate password with all character types
        let has_all_types = true;
        if has_all_types {
            size += 26 + 26 + 10 + 32; // lower + upper + digits + symbols
        }

        assert_eq!(size, 94); // Standard printable ASCII characters
    }

    #[test]
    fn test_numeric_types() {
        // Test that our numeric types can handle expected ranges
        let large_number: u128 = u128::MAX;
        let small_number: u8 = 255;
        let float_precision: f64 = 1.23456789012345;

        assert!(large_number > 0);
        assert_eq!(small_number, 255);
        assert!((float_precision - 1.23456789012345).abs() < f64::EPSILON);
    }
}
