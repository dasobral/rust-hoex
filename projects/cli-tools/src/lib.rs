//! Library API for **seccheck**: password entropy estimation and strength analysis.
//!
//! Capstone for rust-hoex — ties together clap (in the binary), `Result`/`anyhow`,
//! structs/enums, collections, and iterators into a small real CLI.

pub mod analyze;
pub mod entropy;

pub use analyze::{AnalysisReport, Strength, analyze_password};
pub use entropy::{CharClass, EntropyEstimate, estimate_entropy};
