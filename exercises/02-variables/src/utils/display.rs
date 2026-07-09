//! Display and formatting utilities for scientific data
//!
//! This module demonstrates string formatting, scientific notation,
//! and proper display of different numeric types with appropriate precision.

use crate::utils::conversions::{celsius_to_fahrenheit, celsius_to_kelvin};

// === Scientific Notation Formatting ===

/// Format floating-point number in scientific notation.
/// Demonstrates `f64` formatting with precision control.
pub fn format_scientific(value: f64, precision: usize) -> String {
    format!("{value:.precision$e}")
}

/// Format energy values with appropriate units and precision.
/// Conditional formatting based on magnitude.
pub fn format_energy(energy_joules: f64) -> String {
    let abs_energy = energy_joules.abs();

    if abs_energy >= 1.0 {
        format!("{energy_joules:.3} J")
    } else if abs_energy >= 1e-3 {
        format!("{:.3} mJ", energy_joules * 1e3)
    } else if abs_energy >= 1e-6 {
        format!("{:.3} µJ", energy_joules * 1e6)
    } else if abs_energy >= 1e-9 {
        format!("{:.3} nJ", energy_joules * 1e9)
    } else {
        format_scientific(energy_joules, 3)
    }
}

/// Format electric field values with appropriate units.
/// Demonstrates signed integer formatting with units.
pub fn format_electric_field(field_v_per_m: i32) -> String {
    let abs_field = field_v_per_m.unsigned_abs();

    if abs_field >= 1_000_000 {
        format!("{:.2} MV/m", f64::from(field_v_per_m) / 1e6)
    } else if abs_field >= 1_000 {
        format!("{:.2} kV/m", f64::from(field_v_per_m) / 1e3)
    } else {
        format!("{field_v_per_m} V/m")
    }
}

// === Temperature Formatting ===

/// Format temperature with appropriate scale indication.
/// Demonstrates `i16` formatting with sign handling.
pub fn format_temperature_celsius(temp_c: i16) -> String {
    // Negative sign is included automatically by Display
    format!("{temp_c}°C")
}

/// Format temperature in multiple scales.
/// Demonstrates multiple type conversions for display.
pub fn format_temperature_all_scales(temp_c: i16) -> String {
    let temp_f = celsius_to_fahrenheit(temp_c);
    // Default to 0 K if conversion fails (e.g. below absolute zero)
    let temp_k = celsius_to_kelvin(temp_c).unwrap_or(0);

    format!("{temp_c}°C / {temp_f:.1}°F / {temp_k} K")
}

// === Charge and Quantum Number Formatting ===

/// Format electrical charge in elementary units.
/// Demonstrates `i8` formatting with sign and units.
pub fn format_charge(charge_elementary: i8) -> String {
    match charge_elementary {
        0 => "neutral".to_string(),
        1 => "+1e".to_string(),
        -1 => "-1e".to_string(),
        n if n > 0 => format!("+{n}e"),
        n => format!("{n}e"), // Negative sign included automatically
    }
}

/// Format quantum numbers with proper notation.
/// Demonstrates mixed unsigned/signed formatting for quantum states.
pub fn format_quantum_state(n: u8, l: u8, m: i8) -> String {
    format!("n={n}, l={l}, m={m}")
}

// === Range and Bounds Display ===
// Specific versions for common types (avoids needing the `num_traits` crate)

/// Display range for `i8` (commonly used for quantum numbers)
pub fn display_i8_range() -> String {
    format!("[{} to {}]", i8::MIN, i8::MAX)
}

/// Display range for `i16` (commonly used for temperatures)
pub fn display_i16_range() -> String {
    format!("[{} to {}]", i16::MIN, i16::MAX)
}

/// Display range for `i32` (commonly used for energy in eV)
pub fn display_i32_range() -> String {
    format!("[{} to {}]", i32::MIN, i32::MAX)
}

/// Display range for `u8` (commonly used for atomic numbers)
pub fn display_u8_range() -> String {
    format!("[{} to {}]", u8::MIN, u8::MAX)
}

// === Numeric Literal Examples ===

/// Demonstrate different ways to write numeric literals.
/// Shows literal suffixes and formatting options.
pub fn demonstrate_numeric_literals() -> String {
    // Integer literals with suffixes — the suffix makes the type explicit
    let byte_value: u8 = 255u8;
    let short_value: i16 = -32_768i16;
    let int_value: i32 = 2_147_483_647i32;
    let long_value: i64 = 9_223_372_036_854_775_807i64;

    // Floating-point literals with suffixes (use std consts to avoid approx_const lint)
    let float_value: f32 = std::f32::consts::PI;
    let double_value: f64 = std::f64::consts::E;

    // Scientific notation literals
    let planck: f64 = 6.626e-34_f64;
    let avogadro: f64 = 6.022e23_f64;

    // Hexadecimal, octal, and binary literals
    let hex_value: u32 = 0xFFFF_FFFF;
    let octal_value: u32 = 0o777_777_777;
    let binary_value: u16 = 0b1111_1111_1111_1111;

    // Build with `format!` once — avoids `format_push_string` lint
    format!(
        "Integer literals with suffixes:\n\
         \u{20} u8: {byte_value}\n\
         \u{20} i16: {short_value}\n\
         \u{20} i32: {int_value}\n\
         \u{20} i64: {long_value}\n\
         \n\
         Floating-point literals with suffixes:\n\
         \u{20} f32: {float_value}\n\
         \u{20} f64: {double_value}\n\
         \n\
         Scientific notation literals:\n\
         \u{20} Planck constant: {}\n\
         \u{20} Avogadro number: {}\n\
         \n\
         Other number bases:\n\
         \u{20} Hexadecimal: 0x{hex_value:X} = {hex_value}\n\
         \u{20} Octal: 0o{octal_value:o} = {octal_value}\n\
         \u{20} Binary: 0b{binary_value:b} = {binary_value}\n",
        format_scientific(planck, 3),
        format_scientific(avogadro, 3),
    )
}

// === Vector and Tuple Formatting ===

/// Format a 3D vector (for electromagnetic fields).
/// Demonstrates tuple formatting and vector notation.
pub fn format_vector_3d(x: i16, y: i16, z: i16) -> String {
    format!("({x}, {y}, {z}) V/m")
}

/// Format electromagnetic field components.
/// Demonstrates structured data display.
pub fn format_em_field(electric: (i16, i16, i16), magnetic: (i16, i16, i16)) -> String {
    format!(
        "E = {} V/m\nB = {} μT",
        format_vector_3d(electric.0, electric.1, electric.2),
        format_vector_3d(magnetic.0, magnetic.1, magnetic.2)
    )
}

// === Comparison and Analysis Display ===

/// Compare two `f64` values and show the absolute and relative difference.
///
/// Simplified to `f64` so callers convert once (via `f64::from` / `as f64`)
/// instead of wrestling with generic `Into<f64>` + `Sub` bounds.
pub fn format_comparison(name1: &str, value1: f64, name2: &str, value2: f64) -> String {
    let diff = (value1 - value2).abs();
    let percent_diff = if value1.abs() > f64::EPSILON {
        (diff / value1.abs()) * 100.0
    } else {
        0.0
    };

    format!("{name1}: {value1}\n{name2}: {value2}\nDifference: {diff} ({percent_diff:.1}%)")
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

    #[test]
    fn test_format_comparison() {
        let result = format_comparison("a", 100.0, "b", 80.0);
        assert!(result.contains("Difference"));
        assert!(result.contains('2'));
    }
}
