//! Integration tests for `example_variables`.
//! Helpers are reimplemented locally because this is a binary-only crate.

use std::collections::HashMap;

#[test]
fn test_entropy_calculation_weak_password() {
    let password = "password123";
    let character_counts = count_characters(password);
    let shannon_entropy = calculate_shannon_entropy(&character_counts, password.len());

    assert!(
        shannon_entropy < 4.0,
        "Expected low entropy for weak password, got {shannon_entropy}"
    );
}

#[test]
fn test_entropy_calculation_strong_password() {
    let password = "Tr0ub4dor&3";
    let character_counts = count_characters(password);
    let shannon_entropy = calculate_shannon_entropy(&character_counts, password.len());

    assert!(
        shannon_entropy > 3.0,
        "Expected higher entropy for strong password, got {shannon_entropy}"
    );
}

#[test]
fn test_alphabet_size_calculation() {
    let (alphabet_size, _) = analyze_character_types("abcdef");
    assert_eq!(
        alphabet_size, 26,
        "Lowercase only should give alphabet size 26"
    );

    let (alphabet_size, _) = analyze_character_types("AbCdEf");
    assert_eq!(alphabet_size, 52, "Mixed case should give alphabet size 52");

    let (alphabet_size, _) = analyze_character_types("Ab3!ef");
    assert_eq!(alphabet_size, 94, "All types should give alphabet size 94");
}

#[test]
fn test_password_space_entropy() {
    let password = "Test123K";
    let alphabet_size = 94_u32;
    #[allow(clippy::cast_precision_loss)]
    let expected_entropy = (password.len() as f64) * f64::from(alphabet_size).log2();

    assert!(
        (expected_entropy - 52.4).abs() < 0.1,
        "Expected ~52.4 bits, got {expected_entropy}"
    );
}

#[test]
fn test_brute_force_timing() {
    let alphabet_size = 94_u128;
    let password_length = 8_u32;
    let attempts_per_second = 1_000_000_000_u128;

    let total_combinations = alphabet_size.pow(password_length);
    let seconds_to_crack = total_combinations / (attempts_per_second * 2);

    assert!(
        seconds_to_crack > 1_000_000,
        "Brute force should take significant time"
    );
}

#[test]
fn test_character_type_detection() {
    let password = "MyP@ssw0rd!";
    let (alphabet_size, complexity) = analyze_character_types(password);

    assert_eq!(alphabet_size, 94, "Should detect all character types");
    assert_eq!(complexity, 5, "Should have maximum complexity score");
}

#[test]
fn test_numeric_type_ranges() {
    let hash_size: u8 = 32;
    assert_eq!(hash_size, 32);

    let large_count: u64 = 1_000_000_000_000;
    assert!(large_count > 0);

    let huge_number: u128 = u128::MAX;
    assert!(huge_number > 0);

    let entropy: f64 = 123.456_789;
    assert!((entropy - 123.456_789).abs() < f64::EPSILON);
}

#[test]
fn test_constant_values() {
    // Bind locals so clippy does not flag assertions_on_constants.
    let min_secure_entropy: f64 = 50.0;
    let max_password_length: usize = 256;

    assert!(min_secure_entropy > 0.0 && min_secure_entropy < 1000.0);
    assert!(max_password_length > 0 && max_password_length <= 1024);
}

#[test]
fn test_variable_shadowing_behavior() {
    let entropy_threshold: f64 = 50.0;
    let entropy_threshold = entropy_threshold * 1.2;

    assert!(
        (entropy_threshold - 60.0).abs() < f64::EPSILON,
        "Shadowing should update value"
    );
}

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
    #[allow(clippy::cast_precision_loss)]
    let total_chars = total_length as f64;

    for &count in character_counts.values() {
        #[allow(clippy::cast_precision_loss)]
        let probability = count as f64 / total_chars;
        if probability > 0.0 {
            entropy = probability.mul_add(-probability.log2(), entropy);
        }
    }
    entropy
}

fn analyze_character_types(password: &str) -> (u32, u32) {
    let mut alphabet_size = 0_u32;
    let mut complexity_score = 0_u32;

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
