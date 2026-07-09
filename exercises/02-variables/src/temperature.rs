//! Temperature conversions exercise — type conversions & literal suffixes
//!
//! # Learning Focus
//!
//! Different temperature scales and ranges call for different Rust types:
//! - `i8` for room-temperature sensors (small signed range)
//! - `i16` for outdoor / industrial Celsius
//! - `u16` for Kelvin (always ≥ 0)
//! - `f32`/`f64` when fractional degrees matter
//!
//! Concepts covered:
//! - Type conversions (`i16` ↔ `u16` ↔ `f32`)
//! - Numeric literal suffixes (`20i8`, `212.0f32`, `0u16`)
//! - Bounds checking for physical validity (absolute zero)
//! - Precision handling in scientific calculations

use anyhow::Result;

use crate::utils::constants::{
    ABSOLUTE_ZERO_CELSIUS, ROOM_TEMPERATURE_CELSIUS, WATER_BOILING_CELSIUS, WATER_FREEZING_CELSIUS,
};
use crate::utils::conversions::{
    celsius_to_fahrenheit, celsius_to_kelvin, fahrenheit_to_celsius, kelvin_to_celsius,
    safe_i32_to_i16, safe_i32_to_u32,
};
use crate::utils::display::{
    demonstrate_numeric_literals, display_i8_range, display_i16_range, display_u8_range,
    format_comparison, format_temperature_all_scales, format_temperature_celsius,
};

/// Convert Celsius to all common scales, returning `(fahrenheit, kelvin)`.
pub fn convert_celsius_all(celsius: i16) -> Result<(f32, u16)> {
    let f = celsius_to_fahrenheit(celsius);
    let k = celsius_to_kelvin(celsius)?;
    Ok((f, k))
}

/// Demonstrate type selection for different temperature domains.
///
/// Returns a multi-line explanation of which type fits which use case.
pub fn type_selection_examples() -> String {
    // Explicit literal suffixes pin the type at the call site
    let room: i8 = 20i8;
    let arctic: i16 = -45i16;
    let kelvin_room: u16 = 293u16;
    let body_f: f32 = 98.6f32;
    let absolute_zero_k: u16 = 0u16;

    format!(
        "Type selection by physical range:\n\
         - room temp {room}°C          → i8  (range {})\n\
         - arctic    {arctic}°C         → i16 (range {})\n\
         - Kelvin    {kelvin_room} K       → u16 (always ≥ 0, range {})\n\
         - body temp {body_f}°F      → f32 (fractional degrees)\n\
         - abs. zero {absolute_zero_k} K         → u16 (by definition)\n",
        display_i8_range(),
        display_i16_range(),
        display_u8_range(), // illustrative small unsigned range
    )
}

/// Round-trip Celsius → Kelvin → Celsius to show conversion fidelity.
pub fn round_trip_celsius(celsius: i16) -> Result<i16> {
    let kelvin = celsius_to_kelvin(celsius)?;
    kelvin_to_celsius(kelvin)
}

/// Run the temperature conversions exercise.
pub fn run(verbose: bool) -> Result<()> {
    println!("🌡️  Temperature Conversions — Types, Suffixes & Bounds");
    println!();

    // --- Type selection ---
    println!("1. Choose the smallest type that fits the physical range:");
    print!("{}", type_selection_examples());

    if verbose {
        println!("   Constants from the shared utils module:");
        println!(
            "   - Absolute zero: {}",
            format_temperature_celsius(ABSOLUTE_ZERO_CELSIUS)
        );
        println!(
            "   - Freezing:      {}",
            format_temperature_celsius(WATER_FREEZING_CELSIUS)
        );
        println!(
            "   - Boiling:       {}",
            format_temperature_celsius(WATER_BOILING_CELSIUS)
        );
        println!(
            "   - Room:          {}",
            format_temperature_celsius(i16::from(ROOM_TEMPERATURE_CELSIUS))
        );
    }

    // --- Conversions ---
    println!();
    println!("2. Safe conversions with bounds checking:");
    let freezing = WATER_FREEZING_CELSIUS;
    let (f_freezing, k_freezing) = convert_celsius_all(freezing)?;
    println!("   Freezing: {}", format_temperature_all_scales(freezing));
    println!("   → {f_freezing:.1}°F, {k_freezing} K");

    let boiling = WATER_BOILING_CELSIUS;
    let (f_boiling, k_boiling) = convert_celsius_all(boiling)?;
    println!("   Boiling:  {}", format_temperature_all_scales(boiling));
    println!("   → {f_boiling:.1}°F, {k_boiling} K");

    // Fahrenheit → Celsius round trip
    let back = fahrenheit_to_celsius(212.0)?;
    println!("   212°F → {back}°C (round-trip check)");

    if verbose {
        println!();
        println!(
            "   {}",
            format_comparison(
                "boiling °C",
                f64::from(boiling),
                "freezing °C",
                f64::from(freezing)
            )
        );
    }

    // --- Literal suffixes ---
    println!();
    println!("3. Numeric literal suffixes pin types explicitly:");
    if verbose {
        println!("{}", demonstrate_numeric_literals());
    } else {
        let freezing_i16: i16 = 0i16;
        let boiling_f: f32 = 212.0f32;
        let abs_zero_u16: u16 = 0u16;
        println!("   let freezing: i16 = {freezing_i16}i16;");
        println!("   let boiling_f: f32 = {boiling_f}f32;");
        println!("   let absolute_zero: u16 = {abs_zero_u16}u16;");
        println!("   (pass --verbose to see the full literal demo)");
    }

    // --- Bounds & safe casts ---
    println!();
    println!("4. Bounds checking prevents invalid physics:");
    match celsius_to_kelvin(-300) {
        Ok(k) => println!("   Unexpected success: {k} K"),
        Err(e) => println!("   ✓ Rejected −300°C: {e}"),
    }

    if verbose {
        match safe_i32_to_i16(100_000) {
            Ok(v) => println!("   Unexpected: {v}"),
            Err(e) => println!("   ✓ safe_i32_to_i16(100000): {e}"),
        }
        match safe_i32_to_u32(-1) {
            Ok(v) => println!("   Unexpected: {v}"),
            Err(e) => println!("   ✓ safe_i32_to_u32(-1): {e}"),
        }
        let rt = round_trip_celsius(25)?;
        println!("   Round-trip 25°C → K → °C = {rt}°C");
    }

    println!();
    println!("✅ Temperature exercise complete.");
    Ok(())
}

#[cfg(test)]
#[allow(clippy::float_cmp)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_celsius_all() {
        let zero = convert_celsius_all(0);
        assert!(zero.is_ok());
        if let Ok((f, k)) = zero {
            assert!((f - 32.0).abs() < f32::EPSILON);
            assert_eq!(k, 273);
        }

        let hundred = convert_celsius_all(100);
        assert!(hundred.is_ok());
        if let Ok((f100, k100)) = hundred {
            assert!((f100 - 212.0).abs() < f32::EPSILON);
            assert_eq!(k100, 373);
        }
    }

    #[test]
    fn test_below_absolute_zero() {
        assert!(convert_celsius_all(-300).is_err());
        assert!(celsius_to_kelvin(ABSOLUTE_ZERO_CELSIUS - 1).is_err());
    }

    #[test]
    fn test_round_trip() {
        assert!(round_trip_celsius(0).is_ok_and(|t| t == 0));
        assert!(round_trip_celsius(100).is_ok_and(|t| t == 100));
        assert!(round_trip_celsius(-40).is_ok_and(|t| t == -40));
    }

    #[test]
    fn test_type_selection_examples() {
        let text = type_selection_examples();
        assert!(text.contains("i8"));
        assert!(text.contains("i16"));
        assert!(text.contains("u16"));
    }

    #[test]
    fn test_run() {
        assert!(run(false).is_ok());
        assert!(run(true).is_ok());
    }
}
