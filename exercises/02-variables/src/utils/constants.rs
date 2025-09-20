//! Physical and mathematical constants for physics calculations
//!
//! This module demonstrates proper constant declaration with appropriate numeric types
//! for different physical domains and precision requirements.
//! 
//! - Constants are defined with types that match their expected range and precision
//! - Use f64 for high precision, f32 for moderate precision, and integer types for whole numbers
//! - The pub keyword is used to make constants accessible from other modules
//! - The const keyword is used to define constants that are inlined at compile time, they cannot be changed at runtime.

// === Mathematical Constants ===

/// Pi with high precision for scientific calculations
pub const PI: f64 = std::f64::consts::PI;

/// Euler's number (base of natural logarithm)
pub const E: f64 = std::f64::consts::E;

// === Fundamental Physical Constants ===

/// Elementary charge (exact value as of 2019 SI redefinition)
/// Using f64 for high-precision electromagnetic calculations
pub const ELEMENTARY_CHARGE: f64 = 1.602176634e-19; // Coulombs

/// Planck constant (exact value as of 2019 SI redefinition)
/// Critical for quantum mechanics calculations
pub const PLANCK_CONSTANT: f64 = 6.62607015e-34; // J⋅s

/// Speed of light in vacuum (exact value by definition)
/// Using f64 for relativistic calculations
pub const SPEED_OF_LIGHT: f64 = 299792458.0; // m/s

/// Boltzmann constant (exact value as of 2019 SI redefinition)
/// Used in thermodynamics and statistical mechanics
pub const BOLTZMANN_CONSTANT: f64 = 1.380649e-23; // J/K

// === Atomic and Quantum Constants ===

/// Hydrogen ground state binding energy (approximate)
/// Using i32 as it's commonly expressed in whole eV
pub const HYDROGEN_GROUND_STATE_ENERGY: i32 = -13; // eV (negative = bound state)

/// Ionization energy reference (by definition)
/// Zero energy reference point for atomic calculations
pub const IONIZATION_ENERGY: i32 = 0; // eV

/// Rydberg constant for hydrogen-like atoms
/// Using f64 for precise spectroscopic calculations  
pub const RYDBERG_CONSTANT: f64 = 13.6057039763; // eV

// === Temperature Constants ===

/// Absolute zero in Celsius (exact by definition)
/// Using i16 as temperatures rarely exceed this range in common applications
pub const ABSOLUTE_ZERO_CELSIUS: i16 = -273; // °C (approximately)

/// Water freezing point in Celsius (by definition)
pub const WATER_FREEZING_CELSIUS: i16 = 0; // °C

/// Water boiling point in Celsius (at standard pressure)
pub const WATER_BOILING_CELSIUS: i16 = 100; // °C

/// Room temperature (approximate)
/// Using i8 as room temperatures fit comfortably in this range
pub const ROOM_TEMPERATURE_CELSIUS: i8 = 20; // °C

// === Electromagnetic Field Constants ===

/// Vacuum permittivity (electric constant)
/// Using f64 for precise electromagnetic calculations
pub const VACUUM_PERMITTIVITY: f64 = 8.8541878128e-12; // F/m

/// Vacuum permeability (magnetic constant)  
/// Using f64 for precise electromagnetic calculations
pub const VACUUM_PERMEABILITY: f64 = 1.25663706212e-6; // H/m

// === Typical Field Strength Ranges ===

/// Maximum safe electric field in air (approximate)
/// Using i32 as field strengths are typically expressed as whole numbers
pub const MAX_ELECTRIC_FIELD_AIR: i32 = 3_000_000; // V/m (before breakdown)

/// Earth's magnetic field strength (approximate)
/// Using i16 as it's a small value in Tesla units  
pub const EARTH_MAGNETIC_FIELD: i16 = 50; // μT (microtesla)

// === Conversion Factors ===

/// Electron volt to Joule conversion
/// Using f64 for high precision energy conversions
pub const EV_TO_JOULE: f64 = ELEMENTARY_CHARGE; // 1 eV = 1.602...e-19 J

/// Celsius to Kelvin conversion offset
/// Using u16 as Kelvin temperatures are always positive
pub const CELSIUS_TO_KELVIN_OFFSET: u16 = 273; // K

/// Fahrenheit conversion factors
/// Using f32 as sufficient precision for temperature conversions
pub const FAHRENHEIT_SCALE_FACTOR: f32 = 9.0 / 5.0;
pub const FAHRENHEIT_OFFSET: f32 = 32.0;

// === Type Selection Examples ===

/// Atomic number (always positive, small range)
/// Using u8 as no element has atomic number > 255
pub const HYDROGEN_ATOMIC_NUMBER: u8 = 1;
pub const CARBON_ATOMIC_NUMBER: u8 = 6;
pub const URANIUM_ATOMIC_NUMBER: u8 = 92;

/// Particle counts (can be very large)
/// Using u64 for Avogadro-scale quantities
pub const AVOGADRO_NUMBER: u64 = 602214076000000000000000; // Approximate

/// Quantum numbers (small signed integers)
/// Using i8 as quantum numbers are typically -10 to +10 range
pub const ELECTRON_SPIN_UP: i8 = 1;
pub const ELECTRON_SPIN_DOWN: i8 = -1;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_physical_constants_ranges() {
        // Test that constants are within expected physical ranges
        assert!(ELEMENTARY_CHARGE > 0.0);
        assert!(PLANCK_CONSTANT > 0.0);
        assert!(SPEED_OF_LIGHT > 0.0);
        
        // Test signed values
        assert!(HYDROGEN_GROUND_STATE_ENERGY < 0);
        assert_eq!(IONIZATION_ENERGY, 0);
        
        // Test temperature ranges
        assert!(ABSOLUTE_ZERO_CELSIUS < 0);
        assert_eq!(WATER_FREEZING_CELSIUS, 0);
        assert!(ROOM_TEMPERATURE_CELSIUS > 0);
    }

    #[test]
    fn test_type_sizes() {
        // Verify our type choices can handle expected ranges
        assert!(u8::MAX >= URANIUM_ATOMIC_NUMBER);
        assert!(i8::MIN <= ELECTRON_SPIN_DOWN);
        assert!(i8::MAX >= ELECTRON_SPIN_UP);
        assert!(i16::MIN <= ABSOLUTE_ZERO_CELSIUS);
    }
}