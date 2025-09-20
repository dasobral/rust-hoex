//! Unit conversion functions demonstrating type conversions and numeric precision
//!
//! This module shows how to safely convert between different numeric types
//! while maintaining precision and handling edge cases in scientific calculations.

use crate::utils::constants::*;    // crate here is the current library, allowing access to constants. 
use anyhow::{Result, bail};        // Result is a module, bail is a macro for error handling

// === TEMPERATURE CONVERSIONS ===

// Celsius to Kelvin. Demonstrates i16 --> u16 conversion with bounds checking
pub fn celsius_to_kelvin(celsius: i16) -> Result<u16> {
    if celsius < ABSOLUTE_ZERO_CELSIUS {
        bail!("Temperature {} °C is below absolute zero", celsius);
    }

    // Safe conversion: add offset and cast to unsigned
    let kelvin = celsius + CELSIUS_TO_KELVIN_OFFSET as i16;         // The conversion is safe because we checked the lower bound
    Ok(kelvin as u16)                                               // The Ok variant wraps the result so it can be used in a Result context. 
                                                                    // This is like the return statement in other languages.

    // Result context is automatic at the call site 
}

// Kelvin to celsius. Demonstrates u16 --> i16 conversion with bounds checking
pub fn kelvin_to_celsius(kelvin: u16) -> Result<i16> {
    if kelvin < CELSIUS_TO_KELVIN_OFFSET {
        bail!("Temperature {} K is below absolute zero", kelvin);
    }

    // Safe conversion: subtract offset and cast to signed
    let celsius = kelvin as i32 - CELSIUS_TO_KELVIN_OFFSET as i32;
    

    // Check if result fits in i16 and transform
    if celsius < i16::MIN as i32 || celsius > i16::MAX as i32 {
        bail!("Temperature {} K is out of bounds for i16", kelvin);
    }
    Ok(celsius as i16)
}

// Fahrenheit to Celsius. Demonstrates f32 -> i16 conversion with rounding and bounds checking
pub fn fahrenheit_to_celsius(fahrenheit: f32) -> Result<i16> {
    // Calculate Celsius as floating point
    let celsius_f32 = (fahrenheit - FAHRENHEIT_OFFSET) / FAHRENHEIT_SCALE_FACTOR;
    
    // Round to nearest integer
    let celsius_rounded = celsius_f32.round();
    
    // Check bounds before casting
    if celsius_rounded > i16::MAX as f32 || celsius_rounded < i16::MIN as f32 {
        bail!("Temperature {} °F converts to {} °C, outside i16 range", 
              fahrenheit, celsius_rounded);
    }
    
    Ok(celsius_rounded as i16)
}

// === Energy Conversions ===

// electron volts (eV) to joules. Demonstrates i32 -> f64 conversion for high-precision physics
pub fn ev_to_joules(electron_volts: i32) -> f64 {
    // Convert to f64 for high precision, multiply by conversion factor
    electron_volts as f64 * EV_TO_JOULE
}

/// Joules to electron volts (approximate). Demonstrates f64 -> i32 conversion with precision loss warning
pub fn joules_to_ev(joules: f64) -> Result<i32> {
    let ev_f64 = joules / EV_TO_JOULE;
    
    // Check if the value fits in i32 range
    if ev_f64 > i32::MAX as f64 || ev_f64 < i32::MIN as f64 {
        bail!("Energy {} J converts to {} eV, outside i32 range", joules, ev_f64);
    }
    
    // Round to nearest integer (precision loss is expected)
    Ok(ev_f64.round() as i32)
}

// Calculate quantum energy level for hydrogen-like atoms. Demonstrates mixed type arithmetic and precision handling
pub fn hydrogen_energy_level(n: u8, z: u8) -> Result<f64> {
    // Validate quantum numbers
    if n == 0 {
        bail!("Principal quantum number n cannot be zero");
    }
    if z == 0 {
        bail!("Atomic number Z cannot be zero");  
    }
    
    // Calculate energy: E_n = -13.6 * Z^2 / n^2 eV
    // Demonstrates u8 -> f64 conversion for calculations
    let n_f64 = n as f64;
    let z_f64 = z as f64;
    
    let energy_ev = -RYDBERG_CONSTANT * z_f64.powi(2) / n_f64.powi(2);
    Ok(energy_ev)
}

// === Electric Field Conversions ===

/// Electric field from V/m to V/cm. Demonstrates i32 -> f32 conversion with unit scaling
pub fn electric_field_v_per_m_to_v_per_cm(field_v_per_m: i32) -> f32 {
    // 1 V/m = 0.01 V/cm
    field_v_per_m as f32 / 100.0
}

// Calculate force on charge in electric field. Demonstrates mixed signed integer arithmetic
pub fn electric_force(charge_elementary: i8, field_v_per_m: i32) -> Result<f64> {
    // F = qE, where q is in elementary charges, E in V/m
    // Result in Newtons
    
    // Convert to f64 for precision
    let charge_coulombs = charge_elementary as f64 * ELEMENTARY_CHARGE;
    let field_f64 = field_v_per_m as f64;
    
    let force_newtons = charge_coulombs * field_f64;
    Ok(force_newtons)
}

// === Demonstration of Overflow Behavior ===
// We use methods that belong to the Rust standard library

/// Demonstrate checked arithmetic (returns None on overflow)
pub fn checked_multiplication_demo(a: i16, b: i16) -> Option<i16> {
    // The checked_mul method returns None if overflow occurs
    a.checked_mul(b)
}

/// Demonstrate saturating arithmetic (clamps to type limits)  
pub fn saturating_addition_demo(a: i8, b: i8) -> i8 {
    // The saturating_add method clamps to i8::MAX or i8::MIN
    a.saturating_add(b)
}

/// Demonstrate wrapping arithmetic (wraps around on overflow)
pub fn wrapping_subtraction_demo(a: u8, b: u8) -> u8 {
    // The wrapping_sub method wraps around on overflow
    a.wrapping_sub(b)
}

// === Type Conversion Utilities ===

/// Safe conversion from larger to smaller signed integers
pub fn safe_i32_to_i16(value: i32) -> Result<i16> {
    if value > i16::MAX as i32 || value < i16::MIN as i32 {
        bail!("Value {} outside i16 range [{}, {}]", value, i16::MIN, i16::MAX);
    }
    Ok(value as i16)
}

/// Safe conversion from signed to unsigned integers
pub fn safe_i32_to_u32(value: i32) -> Result<u32> {
    if value < 0 {
        bail!("Cannot convert negative value {} to unsigned type", value);
    }
    Ok(value as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_temperature_conversions() {
        // Test basic conversions
        assert_eq!(celsius_to_fahrenheit(0), 32.0);
        assert_eq!(celsius_to_fahrenheit(100), 212.0);
        assert_eq!(fahrenheit_to_celsius(32.0).unwrap(), 0);
        assert_eq!(fahrenheit_to_celsius(212.0).unwrap(), 100);
        
        // Test Kelvin conversions
        assert_eq!(celsius_to_kelvin(0).unwrap(), 273);
        assert_eq!(kelvin_to_celsius(273).unwrap(), 0);
    }

    #[test]
    fn test_energy_conversions() {
        let energy_ev = -13;
        let energy_joules = ev_to_joules(energy_ev);
        let back_to_ev = joules_to_ev(energy_joules).unwrap();
        
        // Should be approximately equal (allowing for floating point precision)
        assert_eq!(back_to_ev, energy_ev);
    }

    #[test]
    fn test_quantum_calculations() {
        // Test hydrogen ground state (n=1, Z=1)
        let ground_state = hydrogen_energy_level(1, 1).unwrap();
        assert!((ground_state - RYDBERG_CONSTANT).abs() < 0.001);
        
        // Test first excited state (n=2, Z=1)  
        let first_excited = hydrogen_energy_level(2, 1).unwrap();
        assert!((first_excited - (-RYDBERG_CONSTANT / 4.0)).abs() < 0.001);
    }

    #[test]
    fn test_overflow_behavior() {
        // Test checked arithmetic
        assert_eq!(checked_multiplication_demo(100, 100), Some(10000));
        assert_eq!(checked_multiplication_demo(i16::MAX, 2), None);
        
        // Test saturating arithmetic
        assert_eq!(saturating_addition_demo(127, 1), 127); // i8::MAX
        assert_eq!(saturating_addition_demo(-128, -1), -128); // i8::MIN
        
        // Test wrapping arithmetic
        assert_eq!(wrapping_subtraction_demo(0, 1), 255); // Wraps around
    }

    #[test]
    fn test_bounds_checking() {
        // Test temperature bounds
        assert!(celsius_to_kelvin(-300).is_err()); // Below absolute zero
        
        // Test energy conversion bounds
        assert!(joules_to_ev(f64::INFINITY).is_err());
        
        // Test safe conversions
        assert!(safe_i32_to_i16(100000).is_err()); // Too large for i16
        assert!(safe_i32_to_u32(-1).is_err()); // Negative value
    }
}