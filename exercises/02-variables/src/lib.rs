/! Variables Exercises Library
//!
//! This is a Rust library that provides a collection of exercises and utilities for learning Rust.
//! In Rust, a library is a package of Rust code that can be shared and reused across multiple projects. 
//! This is essential for code organization and modularity.
//!
//! This library demonstrates advanced Rust variable concepts through physics problems:
//! - Signed integers and their applications
//! - Overflow behavior and safety
//! - Type conversions and precision
//! - Numeric literal suffixes
//! - Range considerations for different domains
//!
//! # Modules
//!
//! - [`quantum`] - Quantum mechanics problems demonstrating signed integers
//! - [`electromagnetic`] - EM field problems demonstrating vector quantities
//! - [`temperature`] - Temperature conversion problems demonstrating type conversions
//! - [`utils`] - Shared utilities and constants


// Public module declarations
pub mod quantum;
pub mod electromagnetic;
pub mod temperature;
pub mod utils;

// Re-exporting commonly used items for convenience
pub use utils::{constants, conversions, display};

// Error type for the library
pub type Result<T> = anyhow::Result<T>; // This is a type alias for the result type used in the library, similar to a template function in C++

// Orchestration
// This section coordinates the various modules and their interactions

/// Exercise metadata for the CLI
/// We define here a derive macro. In Rust, a derive macro is a way to automatically implement certain traits for a struct or enum.
/// Traits are a way to define shared behavior in Rust.
/// Debug is a trait that enables formatting a type using the {:?} formatter.
/// Clone is a trait that allows for the creation of a copy of a value.
#[derive(Debug, Clone)]
pub struct ExerciseInfo {
    pub name: &'static str,                 // Static string slice for exercise name
    pub description: &'static str,          // Static string slice for exercise description
    pub concepts: Vec<&'static str>,        // List of concepts covered in the exercise
}

/// Get information about all available exercises
/// We now fill the ExerciseInfo struct for the different cases
pub fn get_exercise_list() -> Vec<ExerciseInfo> {
    vec![
        ExerciseInfo {
            name: "quantum",
            description: "Quantum energy levels and atomic physics",
            concepts: vec![
                "Signed integers (i8, i16, i32)",
                "Negative values in physics",
                "Type selection for quantum numbers",
                "Bounds checking for physical validity"
            ],
        },
        ExerciseInfo {
            name: "electromagnetic",
            description: "Electromagnetic fields and charged particles",
            concepts: vec![
                "Vector quantities with signed components",
                "Mixed signed/unsigned arithmetic",
                "Overflow behavior in field calculations",
                "Type safety in physics simulations"
            ],
        },
        ExerciseInfo {
            name: "temperature",
            description: "Temperature conversions and thermodynamics",
            concepts: vec![
                "Type conversions (i16 ↔ u16 ↔ f32)",
                "Precision handling in scientific calculations",
                "Bounds checking for physical ranges",
                "Numeric literal suffixes"
            ],
        },
    ]
} 

/// Run a specific exercise by name 
pub fn run_exercise(name: &str, verbose: bool) -> Result<()> {
    match name {
        "quantum" => quantum::run(verbose),
        "electromagnetic" => electromagnetic::run(verbose),
        "temperature" => temperature::run(verbose),
        _ => {
            anyhow::bail!("Unknown exercise: {}. Available exercises: quantum, electromagnetic, temperature", name)
        },
    }
}

/// Run all exercises in sequence
pub fn run_all(verbose: bool) -> Result<()> {
    let exercises = ["quantum", "electromagnetic", "temperature"];

    for (i, exercise_name) in exercises.iter().enumerate() {
        println!("🔬 Exercise {} of {}: {}", i + 1, exercises.len(), exercise_name);
        println!("{}", "=".repeat(50));

        run_exercise(exercise_name, verbose)?;
        // Here we use the question mark operator (try operator)
        // If run_exercise returns an error, it will be propagated up the call stack
        // Otherwise, the value is unwrapped and execution continues
        if i < exercises.len() - 1 {
            println!("\n{}\n", "─".repeat(50));
        }
    }

    Ok(())
}