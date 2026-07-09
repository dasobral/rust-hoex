//! Unit conversion functions demonstrating type conversions and numeric precision
//!
//! This module shows how to safely convert between different numeric types
//! while maintaining precision and handling edge cases in scientific calculations.

use anyhow::{Result, bail};

use crate::utils::constants::{
    ABSOLUTE_ZERO_CELSIUS, CELSIUS_TO_KELVIN_OFFSET, ELEMENTARY_CHARGE, EV_TO_JOULE,
    FAHRENHEIT_OFFSET, FAHRENHEIT_SCALE_FACTOR, RYDBERG_CONSTANT,
};

// === TEMPERATURE CONVERSIONS ===

/// Celsius to Kelvin. Demonstrates `i16` → `u16` conversion with bounds checking.
pub fn celsius_to_kelvin(celsius: i16) -> Result<u16> {
    if celsius < ABSOLUTE_ZERO_CELSIUS {
        bail!("Temperature {celsius} °C is below absolute zero");
    }

    // After the absolute-zero check, `celsius + 273` is in `0..=i16::MAX+273`,
    // which always fits in `u16`. Use `checked_add` + `try_from` instead of casts.
    let offset = i16::try_from(CELSIUS_TO_KELVIN_OFFSET)
        .map_err(|_| anyhow::anyhow!("CELSIUS_TO_KELVIN_OFFSET does not fit in i16"))?;
    let kelvin_i16 = celsius
        .checked_add(offset)
        .ok_or_else(|| anyhow::anyhow!("Temperature {celsius} °C overflows i16 when converting"))?;
    u16::try_from(kelvin_i16)
        .map_err(|_| anyhow::anyhow!("Temperature {celsius} °C converts to a negative Kelvin"))
}

/// Kelvin to Celsius. Demonstrates `u16` → `i16` conversion.
///
/// Kelvin is always ≥ 0 (`u16`), so absolute-zero is already enforced by the type.
/// Values below 273 K are valid (negative Celsius). We only check `i16` fit.
pub fn kelvin_to_celsius(kelvin: u16) -> Result<i16> {
    // Widen to i32 before subtracting so we never underflow unsigned math
    let celsius = i32::from(kelvin) - i32::from(CELSIUS_TO_KELVIN_OFFSET);
    i16::try_from(celsius).map_err(|_| {
        anyhow::anyhow!("Temperature {kelvin} K converts to {celsius} °C, outside i16 range")
    })
}

/// Celsius to Fahrenheit. Demonstrates `i16` → `f32` conversion with scale factors.
///
/// Formula: `F = C × 9/5 + 32`
/// Returns `f32` because Fahrenheit values are often fractional in practice.
pub fn celsius_to_fahrenheit(celsius: i16) -> f32 {
    // `mul_add` is more accurate than `a * b + c` for floating-point
    f32::from(celsius).mul_add(FAHRENHEIT_SCALE_FACTOR, FAHRENHEIT_OFFSET)
}

/// Fahrenheit to Celsius. Demonstrates `f32` → `i16` conversion with rounding and bounds checking.
pub fn fahrenheit_to_celsius(fahrenheit: f32) -> Result<i16> {
    let celsius_f32 = (fahrenheit - FAHRENHEIT_OFFSET) / FAHRENHEIT_SCALE_FACTOR;
    let celsius_rounded = celsius_f32.round();

    if celsius_rounded > f32::from(i16::MAX) || celsius_rounded < f32::from(i16::MIN) {
        bail!("Temperature {fahrenheit} °F converts to {celsius_rounded} °C, outside i16 range");
    }

    // Bound-checked above; `as i16` after round is intentional truncation of the fractional part
    #[allow(clippy::cast_possible_truncation)]
    Ok(celsius_rounded as i16)
}

// === Energy Conversions ===

/// Electron volts (eV) to joules. Demonstrates `i32` → `f64` conversion for high-precision physics.
pub fn ev_to_joules(electron_volts: i32) -> f64 {
    f64::from(electron_volts) * EV_TO_JOULE
}

/// Joules to electron volts (approximate). Demonstrates `f64` → `i32` conversion with precision loss.
pub fn joules_to_ev(joules: f64) -> Result<i32> {
    let ev_f64 = joules / EV_TO_JOULE;

    if !ev_f64.is_finite() || ev_f64 > f64::from(i32::MAX) || ev_f64 < f64::from(i32::MIN) {
        bail!("Energy {joules} J converts to {ev_f64} eV, outside i32 range");
    }

    // Round then truncate — precision loss is expected and documented
    #[allow(clippy::cast_possible_truncation)]
    Ok(ev_f64.round() as i32)
}

/// Calculate quantum energy level for hydrogen-like atoms.
/// Demonstrates mixed type arithmetic and precision handling.
///
/// Formula: `E_n = -R_∞ × Z² / n²` eV
pub fn hydrogen_energy_level(n: u8, z: u8) -> Result<f64> {
    if n == 0 {
        bail!("Principal quantum number n cannot be zero");
    }
    if z == 0 {
        bail!("Atomic number Z cannot be zero");
    }

    let n_f64 = f64::from(n);
    let z_f64 = f64::from(z);
    let energy_ev = -RYDBERG_CONSTANT * z_f64.powi(2) / n_f64.powi(2);
    Ok(energy_ev)
}

// === Electric Field Conversions ===

/// Electric field from V/m to V/cm. Demonstrates `i32` → `f32` conversion with unit scaling.
pub fn electric_field_v_per_m_to_v_per_cm(field_v_per_m: i32) -> f32 {
    // 1 V/m = 0.01 V/cm. Field magnitudes of interest fit comfortably in f32.
    #[allow(clippy::cast_precision_loss)]
    {
        field_v_per_m as f32 / 100.0
    }
}

/// Calculate force on charge in electric field. Demonstrates mixed signed integer arithmetic.
///
/// `F = qE`, where `q` is in elementary charges and `E` is in V/m. Result in Newtons.
pub fn electric_force(charge_elementary: i8, field_v_per_m: i32) -> Result<f64> {
    let charge_coulombs = f64::from(charge_elementary) * ELEMENTARY_CHARGE;
    let field_f64 = f64::from(field_v_per_m);
    Ok(charge_coulombs * field_f64)
}

// === Demonstration of Overflow Behavior ===

/// Demonstrate checked arithmetic (returns `None` on overflow)
pub const fn checked_multiplication_demo(a: i16, b: i16) -> Option<i16> {
    a.checked_mul(b)
}

/// Demonstrate saturating arithmetic (clamps to type limits)
pub const fn saturating_addition_demo(a: i8, b: i8) -> i8 {
    a.saturating_add(b)
}

/// Demonstrate wrapping arithmetic (wraps around on overflow)
pub const fn wrapping_subtraction_demo(a: u8, b: u8) -> u8 {
    a.wrapping_sub(b)
}

// === Type Conversion Utilities ===

/// Safe conversion from larger to smaller signed integers
pub fn safe_i32_to_i16(value: i32) -> Result<i16> {
    i16::try_from(value).map_err(|_| {
        anyhow::anyhow!(
            "Value {value} outside i16 range [{}, {}]",
            i16::MIN,
            i16::MAX
        )
    })
}

/// Safe conversion from signed to unsigned integers
pub fn safe_i32_to_u32(value: i32) -> Result<u32> {
    u32::try_from(value)
        .map_err(|_| anyhow::anyhow!("Cannot convert negative value {value} to unsigned type"))
}

#[cfg(test)]
#[allow(clippy::float_cmp)] // Exact expected values for integer-ish conversions
mod tests {
    use super::*;

    #[test]
    fn test_temperature_conversions() {
        assert!((celsius_to_fahrenheit(0) - 32.0).abs() < f32::EPSILON);
        assert!((celsius_to_fahrenheit(100) - 212.0).abs() < f32::EPSILON);

        assert!(fahrenheit_to_celsius(32.0).is_ok_and(|t| t == 0));
        assert!(fahrenheit_to_celsius(212.0).is_ok_and(|t| t == 100));

        assert!(celsius_to_kelvin(0).is_ok_and(|k| k == 273));
        assert!(kelvin_to_celsius(273).is_ok_and(|c| c == 0));
    }

    #[test]
    fn test_energy_conversions() {
        let energy_ev = -13;
        let energy_joules = ev_to_joules(energy_ev);
        assert!(joules_to_ev(energy_joules).is_ok_and(|e| e == energy_ev));
    }

    #[test]
    fn test_quantum_calculations() {
        let ground_state = hydrogen_energy_level(1, 1);
        assert!(ground_state.is_ok());
        if let Ok(e) = ground_state {
            assert!((e + RYDBERG_CONSTANT).abs() < 0.001);
        }

        let first_excited = hydrogen_energy_level(2, 1);
        assert!(first_excited.is_ok());
        if let Ok(e) = first_excited {
            assert!((e - (-RYDBERG_CONSTANT / 4.0)).abs() < 0.001);
        }
    }

    #[test]
    fn test_overflow_behavior() {
        assert_eq!(checked_multiplication_demo(100, 100), Some(10_000));
        assert_eq!(checked_multiplication_demo(i16::MAX, 2), None);

        assert_eq!(saturating_addition_demo(127, 1), 127); // i8::MAX
        assert_eq!(saturating_addition_demo(-128, -1), -128); // i8::MIN

        assert_eq!(wrapping_subtraction_demo(0, 1), 255); // Wraps around
    }

    #[test]
    fn test_bounds_checking() {
        assert!(celsius_to_kelvin(-300).is_err());
        assert!(joules_to_ev(f64::INFINITY).is_err());
        assert!(safe_i32_to_i16(100_000).is_err());
        assert!(safe_i32_to_u32(-1).is_err());
    }
}
