# 02-variables

Demonstrating how variables work in Rust - Cryptographic entropy and password strength analysis.

## Overview

This example demonstrates Rust's variable system through a **real-world cryptographic entropy calculator**. Instead of trivial examples, we implement a practical tool that cybersecurity professionals could use to assess password strength and calculate entropy - key concepts in information security.

The program analyzes passwords using:

- **Shannon entropy** (information theory measure)
- **Password space entropy** (cryptographic strength)
- **Character set analysis** (complexity assessment)
- **Brute force timing estimates** (practical security implications)

## Learning Objectives

After completing this example, you should understand:

- [x] **Variable declaration** with `let` keyword
- [x] **Immutability by default** and explicit mutability with `mut`
- [x] **Type inference** vs **explicit type annotations**
- [x] **Constants** vs variables and when to use each
- [x] **Variable shadowing** and its practical applications
- [x] **Scope** and variable lifetime
- [x] **Rust's numeric types** and their appropriate use cases
- [x] **Boolean variables** and conditional logic

## Running the Code

```bash
# Run the entropy calculator
cargo run

# Run tests to verify functionality
cargo test

# Check code with clippy for best practices
cargo clippy

# Format code according to Rust conventions
cargo fmt
```

## Key Concepts Demonstrated

### 1. Variable Declaration and Immutability

```rust
// Immutable by default - core Rust safety principle
let test_password = "MyP@ssw0rd123!";      // Cannot be changed
let password_length = test_password.len(); // Computed once, immutable

// Explicit mutability when needed
let mut character_counts = HashMap::new(); // Can be modified
let mut complexity_score = 0;              // Will be updated
```

**Why this matters in cybersecurity:**

- Immutable data prevents accidental modification of critical security parameters
- Mutable variables are explicitly marked, making data flow clear
- Reduces bugs that could lead to security vulnerabilities

### 2. Type Inference vs Explicit Annotations

```rust
// Type inference - Rust figures out the type
let password_length = test_password.len(); // Inferred as usize

// Explicit type annotations - for clarity or when needed
let entropy_threshold: f64 = MIN_SECURE_ENTROPY;
let is_secure: bool = false;
let brute_force_attempts_per_second: u64 = 1_000_000_000;
```

**Best practices:**

- Use inference when type is obvious from context
- Use explicit annotations for clarity in complex calculations
- Always annotate when the type isn't immediately clear

### 3. Constants vs Variables

```rust
// Constants - known at compile time, never change
const MIN_SECURE_ENTROPY: f64 = 50.0;     // Security standard
const MAX_PASSWORD_LENGTH: usize = 256;    // System limit

// Variables - computed at runtime, may change
let entropy_threshold = MIN_SECURE_ENTROPY; // Runtime calculation
let mut complexity_score = 0;               // Will be updated
```

**When to use constants:**

- Security thresholds and limits
- Configuration values
- Mathematical constants
- System constraints

### 4. Variable Shadowing

```rust
// Original value
let entropy_threshold: f64 = MIN_SECURE_ENTROPY;

// Shadow with computed value (different value, same name)
let entropy_threshold = if complexity_score >= 3 {
    MIN_SECURE_ENTROPY * 0.8  // Adjusted for complex passwords
} else {
    MIN_SECURE_ENTROPY * 1.2  // Higher bar for simple passwords
};
```

**Shadowing benefits:**

- Allows value transformation while keeping meaningful names
- Creates immutable final values after computation
- Prevents accidental use of intermediate values

### 5. Rust's Numeric Type System

```rust
// Unsigned integers (no negative values)
let hash_size_sha256: u8 = 32;                         // Small values (0-255)
let brute_force_attempts: u64 = 1_000_000_000;         // Large values
let total_combinations: u128 = (alphabet_size as u128).pow(password_length as u32);

// Floating point for calculations
let shannon_entropy = 0.0;                             // f64 default
let probability = count as f64 / total_chars;          // Precision matters

// Sizes and lengths
let password_bytes: usize = test_password.len();       // Memory/array indexing
```

**Type selection strategy:**

- `u8`: Small values (0-255) - hash sizes, key lengths
- `u16`, `u32` : for larger values.
- `u64`: Large countable values - operation counts, timestamps  
- `u128`: Cryptographic calculations requiring extreme precision
- `usize`: Array indexing and memory sizes
- `f32` : floats with reduced precision
- `f64`: Mathematical calculations requiring decimal precision

> NOTE: while not applicable in this particular example, there signed integers (allow for negative values) also exist in Rust. You define them similarly as the unsigned integers with `i8, i16, i32, i64, i128` and can define a pointer size with `isize`.

### 6. Boolean Logic and Flags

```rust
// Boolean flags for tracking state
let mut has_lowercase = false;
let mut has_uppercase = false;
let mut has_digits = false;
let mut has_symbols = false;

// Update flags based on analysis
for character in test_password.chars() {
    if character.is_ascii_lowercase() {
        has_lowercase = true;
    }
    // ... more conditions
}

// Combine flags for final assessment
let is_secure = password_space_entropy >= entropy_threshold;
```

## Real-World Application: Password Security

This example implements industry-standard password analysis:

### Shannon Entropy Calculation

Measures the randomness in the password based on character frequency:

```rust
let mut shannon_entropy = 0.0;
for &count in character_counts.values() {
    let probability = count as f64 / total_chars;
    if probability > 0.0 {
        shannon_entropy -= probability * probability.log2();
    }
}
```

### Password Space Entropy

Calculates the theoretical strength against brute force:

```rust
let password_space_entropy = (password_length as f64) * (alphabet_size as f64).log2();
```

### Security Thresholds

- **50+ bits**: Generally considered secure for most applications
- **80+ bits**: High security applications
- **128+ bits**: Cryptographic applications

## Exercises

1. **Modify Security Thresholds**:
   - Change `MIN_SECURE_ENTROPY` to different values
   - Observe how it affects password assessments
   - Test with government security standards (NIST guidelines)

2. **Add New Character Categories**:
   - Extend analysis to include Unicode characters
   - Add detection for extended ASCII symbols
   - Calculate alphabet sizes for international character sets

3. **Implement Time-based Security**:
   - Add variables for different attack scenarios (online vs offline)
   - Calculate entropy requirements based on desired security lifetime
   - Consider quantum computing threat models

4. **Memory Security Analysis**:
   - Add variables tracking sensitive data lifetime
   - Implement secure string handling (zero on drop)
   - Calculate memory usage for different password storage schemes

## Security Considerations Demonstrated

1. **Explicit State Management**: Mutable variables clearly show where security-critical state changes
2. **Type Safety**: Proper numeric types prevent overflow in security calculations
3. **Immutable Security Parameters**: Constants ensure security thresholds can't be accidentally modified
4. **Clear Data Flow**: Variable scoping shows exactly where sensitive data is processed

## Further Reading

- [The Rust Book - Chapter 3: Variables and Mutability](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html)
- [The Rust Book - Chapter 3: Data Types](https://doc.rust-lang.org/book/ch03-02-data-types.html)
- [NIST SP 800-63B: Authentication Guidelines](https://pages.nist.gov/800-63-3/sp800-63b.html)
- [Shannon's Information Theory](https://en.wikipedia.org/wiki/Entropy_(information_theory))
- [OWASP Password Storage Cheat Sheet](https://cheatsheetseries.owasp.org/cheatsheets/Password_Storage_Cheat_Sheet.html)

## Related Examples

- `03-dataTypes`: Deep dive into Rust's type system
- `04-functions`: Functions for modular security calculations
- `08-structs`: Organizing security data with custom types
- `12-error-handling`: Robust error handling for security applications

---

**Note**: This example demonstrates variable concepts through practical cryptography. The password analysis is educational - for production systems, use established libraries like `zxcvbn` and follow current security best practices.
