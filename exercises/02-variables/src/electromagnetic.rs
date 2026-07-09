//! Electromagnetic fields exercise — vectors, mixed arithmetic, overflow
//!
//! # Learning Focus
//!
//! Electric and magnetic field components are **directional** (signed).
//! Charges are signed. Multiplying large field values can overflow —
//! Rust gives you `checked_*`, `saturating_*`, and `wrapping_*` to handle that.
//!
//! Concepts covered:
//! - Vector quantities with signed directional components (`i16`)
//! - Mixed signed/unsigned arithmetic
//! - Overflow detection in field-strength calculations
//! - Type conversions for multi-scale physics problems

use anyhow::{Result, bail};

use crate::utils::constants::{EARTH_MAGNETIC_FIELD, ELEMENTARY_CHARGE, MAX_ELECTRIC_FIELD_AIR};
use crate::utils::conversions::{
    checked_multiplication_demo, electric_field_v_per_m_to_v_per_cm, electric_force,
    saturating_addition_demo, wrapping_subtraction_demo,
};
use crate::utils::display::{
    display_i16_range, format_charge, format_electric_field, format_em_field, format_scientific,
    format_vector_3d,
};

/// A 3D field vector with signed `i16` components (V/m or μT).
///
/// Using `i16` keeps memory small while covering typical lab-scale fields
/// (−32 768 … 32 767). Extreme fields need `i32` or `f64`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FieldVector {
    pub x: i16,
    pub y: i16,
    pub z: i16,
}

impl FieldVector {
    /// Create a new field vector.
    pub const fn new(x: i16, y: i16, z: i16) -> Self {
        Self { x, y, z }
    }

    /// Magnitude via `√(x² + y² + z²)` as `f64`.
    ///
    /// Squares are computed in `i32` to reduce intermediate overflow risk,
    /// then converted to `f64` for the square root.
    pub fn magnitude(self) -> f64 {
        let x = i32::from(self.x);
        let y = i32::from(self.y);
        let z = i32::from(self.z);
        f64::from(x * x + y * y + z * z).sqrt()
    }
}

/// Calculate the Euclidean magnitude of a 3D integer vector.
pub fn calculate_vector_magnitude(x: i16, y: i16, z: i16) -> Result<f64> {
    Ok(FieldVector::new(x, y, z).magnitude())
}

/// Force on a point charge: F = qE (one-dimensional demo).
///
/// `charge_elementary` is signed (`i8`): electrons are −1, protons +1.
/// Returns force in Newtons as `f64`.
pub fn force_on_charge(charge_elementary: i8, field_v_per_m: i32) -> Result<f64> {
    electric_force(charge_elementary, field_v_per_m)
}

/// Demonstrate overflow strategies when scaling field components.
///
/// Returns a short human-readable report of checked / saturating / wrapping results.
pub fn demonstrate_overflow(a: i16, b: i16) -> String {
    let checked = checked_multiplication_demo(a, b).map_or_else(
        || format!("checked_mul({a}, {b}) = None  ← overflow detected!\n"),
        |product| format!("checked_mul({a}, {b}) = Some({product})\n"),
    );

    // Saturating add on i8 — clamps instead of wrapping
    let sat = saturating_addition_demo(i8::MAX, 1);
    // Wrapping sub on u8 — wraps around (often surprising!)
    let wrap = wrapping_subtraction_demo(0u8, 1u8);

    format!(
        "{checked}\
         saturating_add(i8::MAX, 1) = {sat} (clamped)\n\
         wrapping_sub(0u8, 1) = {wrap} (wrapped)\n"
    )
}

/// Validate that an electric field is below air breakdown strength.
pub fn validate_field_strength(field_v_per_m: i32) -> Result<()> {
    if field_v_per_m.unsigned_abs() > MAX_ELECTRIC_FIELD_AIR.unsigned_abs() {
        bail!(
            "Field {} exceeds air breakdown (~{} V/m)",
            format_electric_field(field_v_per_m),
            MAX_ELECTRIC_FIELD_AIR
        );
    }
    Ok(())
}

/// Run the electromagnetic fields exercise.
pub fn run(verbose: bool) -> Result<()> {
    println!("⚡ Electromagnetic Fields — Vectors & Overflow Safety");
    println!();

    // --- Signed vector components ---
    println!("1. Field vectors use signed components (direction matters):");
    println!("   i16 range: {}", display_i16_range());

    let e_field = FieldVector::new(-1500, 2000, 0); // V/m
    let b_field = FieldVector::new(0, 0, EARTH_MAGNETIC_FIELD); // μT

    println!(
        "   E = {}",
        format_vector_3d(e_field.x, e_field.y, e_field.z)
    );
    println!(
        "   |E| ≈ {:.1} V/m",
        calculate_vector_magnitude(e_field.x, e_field.y, e_field.z)?
    );
    println!(
        "   {}",
        format_em_field(
            (e_field.x, e_field.y, e_field.z),
            (b_field.x, b_field.y, b_field.z)
        )
    );

    if verbose {
        println!();
        println!("   Negative x means 'westward'; positive y means 'northward'.");
        println!("   Unsigned types cannot represent direction — use signed!");
    }

    // --- Charges and force ---
    println!();
    println!("2. Charges are signed; force = q × E:");
    let electron: i8 = -1;
    let proton: i8 = 1;
    let field_strength: i32 = 1000; // V/m

    println!("   Electron: {}", format_charge(electron));
    println!("   Proton:   {}", format_charge(proton));

    let f_e = force_on_charge(electron, field_strength)?;
    let f_p = force_on_charge(proton, field_strength)?;
    println!("   F_electron = {} N", format_scientific(f_e, 3));
    println!(
        "   F_proton   = {} N  (opposite direction)",
        format_scientific(f_p, 3)
    );

    if verbose {
        println!();
        println!(
            "   Elementary charge e = {} C",
            format_scientific(ELEMENTARY_CHARGE, 4)
        );
        let field_cm = electric_field_v_per_m_to_v_per_cm(field_strength);
        println!("   {field_strength} V/m = {field_cm} V/cm (unit conversion)");
    }

    // --- Overflow demos ---
    println!();
    println!("3. Overflow protection in field scaling:");
    print!("{}", demonstrate_overflow(100, 100));
    print!("{}", demonstrate_overflow(i16::MAX, 2));

    if verbose {
        println!();
        println!("   Strategies:");
        println!("   - checked_*   → Option (None on overflow) — prefer for safety-critical code");
        println!("   - saturating_* → clamps to type limits");
        println!("   - wrapping_*  → wraps around (rarely what you want in physics)");
        match validate_field_strength(MAX_ELECTRIC_FIELD_AIR + 1) {
            Ok(()) => println!("   Unexpected: over-limit field should fail"),
            Err(e) => println!("   ✓ Bounds check: {e}"),
        }
    }

    println!();
    println!("✅ Electromagnetic exercise complete.");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector_magnitude() {
        let mag = calculate_vector_magnitude(3, 4, 0);
        assert!(mag.is_ok());
        if let Ok(m) = mag {
            assert!((m - 5.0).abs() < 1e-10);
        }

        let mag3 = calculate_vector_magnitude(100, -50, 0);
        assert!(mag3.is_ok_and(|m| m > 100.0));
    }

    #[test]
    fn test_force_signs() {
        let f_e = force_on_charge(-1, 1000);
        let f_p = force_on_charge(1, 1000);
        assert!(f_e.is_ok_and(|f| f < 0.0));
        assert!(f_p.is_ok_and(|f| f > 0.0));
        if let (Ok(e), Ok(p)) = (force_on_charge(-1, 1000), force_on_charge(1, 1000)) {
            assert!((e + p).abs() < 1e-30);
        }
    }

    #[test]
    fn test_overflow_demo() {
        let report = demonstrate_overflow(i16::MAX, 2);
        assert!(report.contains("None"));
        assert!(report.contains("overflow"));
    }

    #[test]
    fn test_field_validation() {
        assert!(validate_field_strength(1000).is_ok());
        assert!(validate_field_strength(MAX_ELECTRIC_FIELD_AIR + 1).is_err());
    }

    #[test]
    fn test_run() {
        assert!(run(false).is_ok());
        assert!(run(true).is_ok());
    }
}
