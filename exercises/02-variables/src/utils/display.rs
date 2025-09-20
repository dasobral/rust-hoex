//! Display and formatting utilities for scientific data
//!
//! This module demonstrates string formatting, scientific notation,
//! and proper display of different numeric types with appropriate precision.

use std::fmt;
use crate::utils::conversions::*;

// === Scientific Notation Formatting ===

// Format floating-point number in scientific notation
// Demonstrates f64 formatting with precision control
pub fn format_scientific(value: f64, precision: usize) -> String {
    format!("{:.precision$e}", value, precision = precision)
}

// Format energy values with appropriate units and precision
// Conditional formatting based on magnitude
pub fn format_energy(energy_joules: f64) -> String {
    let abs_energy = energy_joules.abs();

    if abs_energy >= 1.0 {
        format!("{:.3} J", energy_joules)
    } else if abs_energy >= 1e-3 {
        format!("{:.3} mJ", energy_joules * 1e3)
    } else if abs_energy >= 1e-6 {
        format!("{:.3} µJ", energy_joules * 1e6 )
    } else if abs_energy >= 1e-9 {
        format!("{:.3} nJ", energy_joules * 1e9)
    } else {
        format_scientific(energy_joules, 3)
    }
}

/// Format electric field values with appropriate units
/// Demonstrates signed integer formatting with units
pub fn format_electric_field(field_v_per_m: i32) -> String {
    let abs_field = field_v_per_m.abs();
    
    if abs_field >= 1_000_000 {
        format!("{:.2} MV/m", field_v_per_m as f64 / 1e6)
    } else if abs_field >= 1_000 {
        format!("{:.2} kV/m", field_v_per_m as f64 / 1e3)
    } else {
        format!("{} V/m", field_v_per_m)
    }
}

// === Temperature Formatting ===

/// Format temperature with appropriate scale indication
/// Demonstrates i16 formatting with sign handling
pub fn format_temperature_celsius(temp_c: i16) -> String {
    if temp_c >= 0 {
        format!("{}°C", temp_c)
    } else {
        format!("{}°C", temp_c) // Negative sign included automatically
    }
}

/// Format temperature in multiple scales
/// Demonstrates multiple type conversions for display
pub fn format_temperature_all_scales(temp_c: i16) -> String {
    // Use our methods in crate::utils::conversions
    let temp_f = celsius_to_fahrenheit(temp_c);
    let temp_k = celsius_to_kelvin(temp_c)
        .unwrap_or(0); // Default to 0 K if conversion fails
    
    format!("{}°C / {:.1}°F / {} K", temp_c, temp_f, temp_k)
}

// === Charge and Quantum Number Formatting ===

/// Format electrical charge in elementary units
/// Demonstrates i8 formatting with sign and units
pub fn format_charge(charge_elementary: i8) -> String {
    match charge_elementary {
        0 => "neutral".to_string(),
        1 => "+1e".to_string(),
        -1 => "-1e".to_string(),
        n if n > 0 => format!("+{}e", n),
        n => format!("{}e", n), // Negative sign included automatically
    }
}

/// Format quantum numbers with proper notation
/// Demonstrates u8 formatting for quantum states
pub fn format_quantum_state(n: u8, l: u8, m: i8) -> String {
    format!("n={}, l={}, m={}", n, l, m)
}

// === Range and Bounds Display ===

/// Display the valid range for a numeric type
/// Demonstrates generic formatting for different integer types
pub fn display_integer_range<T>() -> String 
where 
    T: fmt::Display + Copy,
    T: num_traits::Bounded,
{
    format!("[{} to {}]", T::min_value(), T::max_value())
}

// Note: num_traits is not available in our workspace, so let's implement specific versions

/// Display range for i8 (commonly used for quantum numbers)
pub fn display_i8_range() -> String {
    format!("[{} to {}]", i8::MIN, i8::MAX)
}

/// Display range for i16 (commonly used for temperatures)
pub fn display_i16_range() -> String {
    format!("[{} to {}]", i16::MIN, i16::MAX)
}

/// Display range for i32 (commonly used for energy in eV)
pub fn display_i32_range() -> String {
    format!("[{} to {}]", i32::MIN, i32::MAX)
}

/// Display range for u8 (commonly used for atomic numbers)
pub fn display_u8_range() -> String {
    format!("[{} to {}]", u8::MIN, u8::MAX)
}

// === Numeric Literal Examples ===

/// Demonstrate different ways to write numeric literals
/// Shows literal suffixes and formatting options
pub fn demonstrate_numeric_literals() -> String {
    let mut output = String::new();
    
    // Integer literals with suffixes
    let byte_value: u8 = 255u8;                           // Why to write u8 suffix? To indicate the type explicitly 
    let short_value: i16 = -32_768i16;
    let int_value: i32 = 2_147_483_647i32;
    let long_value: i64 = 9_223_372_036_854_775_807i64;
    
    // The push_str method is used to append formatted strings to the output (which is already a String)
    output.push_str("Integer literals with suffixes:\n");
    output.push_str(&format!("  u8: {}\n", byte_value));
    output.push_str(&format!("  i16: {}\n", short_value));
    output.push_str(&format!("  i32: {}\n", int_value));
    output.push_str(&format!("  i64: {}\n", long_value));
    
    // Floating-point literals with suffixes
    let float_value: f32 = 3.14159f32;
    let double_value: f64 = 2.718281828459045f64;
    
    output.push_str("\nFloating-point literals with suffixes:\n");
    output.push_str(&format!("  f32: {}\n", float_value));
    output.push_str(&format!("  f64: {}\n", double_value));
    
    // Scientific notation literals
    let planck: f64 = 6.626e-34f64;
    let avogadro: f64 = 6.022e23f64;
    
    output.push_str("\nScientific notation literals:\n");
    output.push_str(&format!("  Planck constant: {}\n", format_scientific(planck, 3)));
    output.push_str(&format!("  Avogadro number: {}\n", format_scientific(avogadro, 3)));
    
    // Hexadecimal, octal, and binary literals  
    let hex_value: u32 = 0xFF_FF_FF_FF;
    let octal_value: u32 = 0o777_777_777;
    let binary_value: u16 = 0b1111_1111_1111_1111;
    
    output.push_str("\nOther number bases:\n");
    output.push_str(&format!("  Hexadecimal: 0x{:X} = {}\n", hex_value, hex_value));
    output.push_str(&format!("  Octal: 0o{:o} = {}\n", octal_value, octal_value));
    output.push_str(&format!("  Binary: 0b{:b} = {}\n", binary_value, binary_value));
    
    output
}

// === Vector and Tuple Formatting ===

/// Format a 3D vector (for electromagnetic fields)
/// Demonstrates tuple formatting and vector notation
pub fn format_vector_3d(x: i16, y: i16, z: i16) -> String {
    format!("({}, {}, {}) V/m", x, y, z)
}

/// Format electromagnetic field components
/// Demonstrates structured data display
pub fn format_em_field(electric: (i16, i16, i16), magnetic: (i16, i16, i16)) -> String {
    format!(
        "E = {} V/m\nB = {} μT",
        format_vector_3d(electric.0, electric.1, electric.2),
        format_vector_3d(magnetic.0, magnetic.1, magnetic.2)
    )
}

// === Comparison and Analysis Display ===

/// Compare two values and show the difference
/// Demonstrates comparison formatting and percentage calculations
/// What are these T's? They are type parameters placed on the function for generic programming (like placeholders in templates)
/// The where clause specifies the constraints on the type parameters.
pub fn format_comparison<T>(name1: &str, value1: T, name2: &str, value2: T) -> String
where
    T: fmt::Display + Copy + PartialOrd + std::ops::Sub<Output = T>,       // This is a way to specify the behavior of the first T
    T: Into<f64>,                                                          // This is a way to specify the behavior of the second T
{
    let diff = if value1 > value2 { value1 - value2 } else { value2 - value1 };
    let percent_diff = (diff.into() / value1.into()) * 100.0;
    
    format!(
        "{}: {}\n{}: {}\nDifference: {} ({:.1}%)",
        name1, value1, name2, value2, diff, percent_diff
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scientific_formatting() {
        let planck = 6.626e-34;
        let formatted = format_scientific(planck, 3);
        assert!(formatted.contains("6.626e-34"));
    }

    #[test]
    fn test_energy_formatting() {
        // Test different energy scales
        assert!(format_energy(1.5).contains("1.500 J"));
        assert!(format_energy(0.001).contains("1.000 mJ"));
        assert!(format_energy(1e-9).contains("1.000 nJ"));
    }

    #[test]
    fn test_temperature_formatting() {
        assert_eq!(format_temperature_celsius(25), "25°C");
        assert_eq!(format_temperature_celsius(-10), "-10°C");
        assert_eq!(format_temperature_celsius(0), "0°C");
    }

    #[test]
    fn test_charge_formatting() {
        assert_eq!(format_charge(0), "neutral");
        assert_eq!(format_charge(1), "+1e");
        assert_eq!(format_charge(-1), "-1e");
        assert_eq!(format_charge(2), "+2e");
        assert_eq!(format_charge(-3), "-3e");
    }

    #[test]
    fn test_range_display() {
        let i8_range = display_i8_range();
        assert!(i8_range.contains("-128"));
        assert!(i8_range.contains("127"));
    }

    #[test]
    fn test_vector_formatting() {
        let vector = format_vector_3d(100, -50, 0);
        assert_eq!(vector, "(100, -50, 0) V/m");
    }
}