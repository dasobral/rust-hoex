//! Quantum mechanics exercise — signed integers and energy levels
//!
//! # Learning Focus
//!
//! Bound-state energies are **negative** (you must add energy to free an electron).
//! Quantum numbers have small, signed ranges — perfect for teaching `i8` vs `i32`.
//!
//! Concepts covered:
//! - Signed integers (`i8`, `i16`, `i32`) for physically signed quantities
//! - Why energy levels are negative in quantum mechanics
//! - Type selection: `i8` for quantum numbers, `i32`/`f64` for energies
//! - Bounds checking for physical validity (selection rules)

use anyhow::{Result, bail};

use crate::utils::constants::{
    ELECTRON_SPIN_DOWN, ELECTRON_SPIN_UP, HYDROGEN_GROUND_STATE_ENERGY, RYDBERG_CONSTANT,
};
use crate::utils::conversions::{ev_to_joules, hydrogen_energy_level};
use crate::utils::display::{
    display_i8_range, display_i32_range, format_energy, format_quantum_state, format_scientific,
};

/// Validate quantum numbers `(n, l, m)` against selection rules.
///
/// - Principal quantum number `n` ≥ 1 (`u8`, always positive)
/// - Orbital quantum number `l` satisfies `0 ≤ l < n`
/// - Magnetic quantum number `m` is signed (`i8`) and satisfies `-l ≤ m ≤ +l`
pub fn validate_quantum_numbers(n: u8, l: u8, m: i8) -> Result<()> {
    if n == 0 {
        bail!("Principal quantum number n must be ≥ 1 (got {n})");
    }
    if l >= n {
        bail!("Orbital quantum number l must be < n (got l={l}, n={n})");
    }
    // `l` fits in i8 for any realistic atom; cast is safe after the checks above
    let l_signed = i8::try_from(l).map_err(|_| anyhow::anyhow!("l={l} exceeds i8 range"))?;
    if m < -l_signed || m > l_signed {
        bail!("Magnetic quantum number m must satisfy -l ≤ m ≤ +l (got m={m}, l={l})");
    }
    Ok(())
}

/// Approximate hydrogen energy level in whole eV using signed `i32`.
///
/// Uses `E_n ≈ -13 / n²` eV so students see negative bound-state energies
/// without floating-point noise. For precise values, prefer
/// [`hydrogen_energy_level`].
pub fn approximate_energy_ev(n: u8) -> Result<i32> {
    if n == 0 {
        bail!("Principal quantum number n cannot be zero");
    }
    // Integer division: ground state n=1 → -13 eV; n=2 → -3 eV
    Ok(HYDROGEN_GROUND_STATE_ENERGY / i32::from(n).pow(2))
}

/// Energy difference between two levels (photon energy for a transition).
///
/// Demonstrates signed arithmetic: `ΔE = E_final - E_initial`.
/// Absorption (`n_low` → `n_high`) yields positive ΔE; emission yields negative.
pub fn transition_energy_ev(n_initial: u8, n_final: u8) -> Result<i32> {
    let e_i = approximate_energy_ev(n_initial)?;
    let e_f = approximate_energy_ev(n_final)?;
    Ok(e_f - e_i)
}

/// Run the quantum mechanics exercise, printing educational output.
pub fn run(verbose: bool) -> Result<()> {
    println!("🔬 Quantum Energy Levels — Signed Integers in Physics");
    println!();

    // --- Signed energy levels ---
    println!("1. Bound-state energies are NEGATIVE (signed i32):");
    let ground: i32 = HYDROGEN_GROUND_STATE_ENERGY; // -13 eV
    let first_excited = approximate_energy_ev(2)?; // ≈ -3 eV
    println!("   Ground state (n=1):  {ground} eV");
    println!("   First excited (n=2): {first_excited} eV");
    println!("   i32 range: {}", display_i32_range());

    if verbose {
        println!();
        println!("   Why negative? Zero is the ionization threshold.");
        println!("   Bound electrons have less energy than free ones, so E < 0.");
        let precise = hydrogen_energy_level(1, 1)?;
        println!(
            "   Precise Rydberg ground state: {} eV",
            format_scientific(precise, 4)
        );
        println!("   In joules: {}", format_energy(ev_to_joules(ground)));
    }

    // --- Quantum numbers with i8 ---
    println!();
    println!("2. Quantum numbers use small signed types (i8):");
    println!("   i8 range: {}", display_i8_range());

    let n: u8 = 2; // always positive → unsigned
    let l: u8 = 1; // 0..n-1
    let m: i8 = -1; // -l..=+l  ← can be negative!
    let spin: i8 = ELECTRON_SPIN_DOWN; // ±1

    validate_quantum_numbers(n, l, m)?;
    println!("   State: {}", format_quantum_state(n, l, m));
    println!("   Spin:  {spin} (up={ELECTRON_SPIN_UP}, down={ELECTRON_SPIN_DOWN})");

    if verbose {
        println!();
        println!("   Type selection tip:");
        println!("   - u8 for n, l, Z  (always ≥ 0, small)");
        println!("   - i8 for m, spin  (can be negative, small)");
        println!("   - i32 for energies in whole eV");
        println!("   - f64 for precise spectroscopic work ({RYDBERG_CONSTANT} eV)");
    }

    // --- Transition energies ---
    println!();
    println!("3. Transition energies (signed arithmetic):");
    let absorption = transition_energy_ev(1, 2)?;
    let emission = transition_energy_ev(2, 1)?;
    println!("   Absorption  n=1→2: ΔE = {absorption} eV  (energy gained)");
    println!("   Emission    n=2→1: ΔE = {emission} eV  (energy released)");

    if verbose {
        println!();
        println!("   Invalid quantum numbers are rejected:");
        match validate_quantum_numbers(2, 1, 2) {
            Ok(()) => println!("   Unexpected: m=2 with l=1 should fail"),
            Err(e) => println!("   ✓ Caught: {e}"),
        }
    }

    println!();
    println!("✅ Quantum exercise complete.");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quantum_number_validation() {
        assert!(validate_quantum_numbers(2, 1, 0).is_ok());
        assert!(validate_quantum_numbers(2, 1, -1).is_ok());
        assert!(validate_quantum_numbers(2, 1, 1).is_ok());
        assert!(validate_quantum_numbers(2, 1, 2).is_err()); // m > l
        assert!(validate_quantum_numbers(0, 0, 0).is_err()); // n == 0
        assert!(validate_quantum_numbers(1, 1, 0).is_err()); // l >= n
    }

    #[test]
    fn test_approximate_energy() {
        assert!(approximate_energy_ev(1).is_ok_and(|e| e == -13));
        assert!(approximate_energy_ev(2).is_ok_and(|e| e == -3));
        assert!(approximate_energy_ev(0).is_err());
    }

    #[test]
    fn test_transition_energy() {
        // Absorption: -3 - (-13) = +10
        assert!(transition_energy_ev(1, 2).is_ok_and(|e| e == 10));
        // Emission: -13 - (-3) = -10
        assert!(transition_energy_ev(2, 1).is_ok_and(|e| e == -10));
    }

    #[test]
    fn test_run() {
        assert!(run(false).is_ok());
        assert!(run(true).is_ok());
    }
}
