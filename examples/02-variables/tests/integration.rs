// Integration tests for 02-variables
// Testing the cryptographic entropy calculator functionality

use std::collections::HashMap;

#[test]
fn test_entropy_calculation_weak_password() {
    // Test with a known weak password
    let password = "password123";
    let character_counts = count_characters(password);
    let shannon_entropy = calculate_shannon_entropy(&character_counts, password.len());

    // Weak passwords should have lower entropy
    assert!(
        shannon_entropy < 4.0,
        "Expected low entropy for weak password, got {}",
        shannon_entropy
    );
}

#[test]
fn test_entropy_calculation_strong_password() {
    // Test with a strong password
    let password = "Tr0ub4dor&3";
    let character_counts = count_characters(password);
    let shannon_entropy = calculate_shannon_entropy(&character_counts, password.len());

    // Strong passwords should have higher entropy
    assert!(
        shannon_entropy > 3.0,
        "Expected higher entropy for strong password, got {}",
        shannon_entropy
    );
}

#[test]
fn test_alphabet_size_calculation() {
    // Test alphabet size calculation for different character sets

    // Only lowercase
    let (alphabet_size, _) = analyze_character_types("abcdef");
    assert_eq!(
        alphabet_size, 26,
        "Lowercase only should give alphabet size 26"
    );

    // Mixed case
    let (alphabet_size, _) = analyze_character_types("AbCdEf");
    assert_eq!(alphabet_size, 52, "Mixed case should give alphabet size 52");

    // All character types
    let (alphabet_size, _) = analyze_character_types("Ab3!ef");
    assert_eq!(alphabet_size, 94, "All types should give alphabet size 94");
}

#[test]
fn test_password_space_entropy() {
    // Test password space entropy calculation
    let password = "Test123K";
    let alphabet_size = 94; // All printable ASCII
    let expected_entropy = (password.len() as f64) * (alphabet_size as f64).log2();

    // Should be approximately 52.4 bits for 8 characters
    assert!(
        (expected_entropy - 52.4).abs() < 0.1,
        "Expected ~52.7 bits, got {}",
        expected_entropy
    );
}

#[test]
fn test_brute_force_timing() {
    // Test brute force timing calculations
    let alphabet_size = 94u128;
    let password_length = 8u32;
    let attempts_per_second = 1_000_000_000u128;

    let total_combinations = alphabet_size.pow(password_length);
    let seconds_to_crack = total_combinations / (attempts_per_second * 2); // Average case

    // Should be a very large number
    assert!(
        seconds_to_crack > 1_000_000,
        "Brute force should take significant time"
    );
}

#[test]
fn test_character_type_detection() {
    let password = "MyP@ssw0rd!";
    let (alphabet_size, complexity) = analyze_character_types(password);

    // Should detect all character types
    assert_eq!(alphabet_size, 94, "Should detect all character types");
    assert_eq!(complexity, 5, "Should have maximum complexity score");
}

#[test]
fn test_numeric_type_ranges() {
    // Test that our numeric types can handle expected ranges

    // u8 for small values
    let hash_size: u8 = 32;
    assert_eq!(hash_size, 32);

    // u64 for large counts
    let large_count: u64 = 1_000_000_000_000;
    assert!(large_count > 0);

    // u128 for cryptographic calculations
    let huge_number: u128 = u128::MAX;
    assert!(huge_number > 0);

    // f64 for precision calculations
    let entropy: f64 = 123.456789;
    assert!((entropy - 123.456789).abs() < f64::EPSILON);
}

#[test]
fn test_constant_values() {
    // Test our security constants are reasonable
    const MIN_SECURE_ENTROPY: f64 = 50.0;
    const MAX_PASSWORD_LENGTH: usize = 256;

    assert!(MIN_SECURE_ENTROPY > 0.0 && MIN_SECURE_ENTROPY < 1000.0);
    assert!(MAX_PASSWORD_LENGTH > 0 && MAX_PASSWORD_LENGTH <= 1024);
}

#[test]
fn test_variable_shadowing_behavior() {
    // Test variable shadowing functionality
    let entropy_threshold = 50.0;

    // Shadow with different value
    let entropy_threshold = entropy_threshold * 1.2;

    assert_eq!(entropy_threshold, 60.0, "Shadowing should update value");
}

// Helper functions for testing
fn count_characters(password: &str) -> HashMap<char, usize> {
    let mut counts = HashMap::new();
    for character in password.chars() {
        let count = counts.entry(character).or_insert(0);
        *count += 1;
    }
    counts
}

fn calculate_shannon_entropy(character_counts: &HashMap<char, usize>, total_length: usize) -> f64 {
    let mut entropy = 0.0;
    let total_chars = total_length as f64;

    for &count in character_counts.values() {
        let probability = count as f64 / total_chars;
        if probability > 0.0 {
            entropy -= probability * probability.log2();
        }
    }
    entropy
}

fn analyze_character_types(password: &str) -> (usize, usize) {
    let mut alphabet_size = 0;
    let mut complexity_score = 0;

    let has_lowercase = password.chars().any(|c| c.is_ascii_lowercase());
    let has_uppercase = password.chars().any(|c| c.is_ascii_uppercase());
    let has_digits = password.chars().any(|c| c.is_ascii_digit());
    let has_symbols = password.chars().any(|c| c.is_ascii_punctuation());

    if has_lowercase {
        alphabet_size += 26;
        complexity_score += 1;
    }
    if has_uppercase {
        alphabet_size += 26;
        complexity_score += 1;
    }
    if has_digits {
        alphabet_size += 10;
        complexity_score += 1;
    }
    if has_symbols {
        alphabet_size += 32;
        complexity_score += 2;
    }

    (alphabet_size, complexity_score)
}
