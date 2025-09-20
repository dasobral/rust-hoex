//! This module provides shared utilities and constants across all the exercises in this topic:
//! //! - Physical and mathematical constants
//! - Unit conversion functions  
//! - Display formatting helpers
//!
//! # Organization
//! 
//! - [`constants`] - Physical constants and mathematical values
//! - [`conversions`] - Unit conversion functions (temperature, energy, etc.)
//! - [`display`] - Formatting helpers for scientific notation and units

// Declaration of submodules
pub mod constants;
pub mod conversions;
pub mod display;

// Re-export commonly used items
pub use constants::*;
pub use conversions::*;
pub use display::*;