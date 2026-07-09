//! Function exercises — network checksum helpers and sealed packets.
//!
//! Demonstrates function composition, return types, and `Option` guards in a
//! packet integrity context.

pub mod checksum;
mod checksum_exercise;
mod seal_exercise;

pub use checksum::{
    bytes_to_words, checksum_hex, fold_checksum, internet_checksum, nibble_mix, payload_len,
    seal_packet, verify_sealed,
};

pub type Result<T> = anyhow::Result<T>;

/// Exercise metadata for the CLI.
#[derive(Debug, Clone)]
pub struct ExerciseInfo {
    /// Static exercise identifier.
    pub name: &'static str,
    /// Short description shown in `list`.
    pub description: &'static str,
    /// Concepts covered by the exercise.
    pub concepts: Vec<&'static str>,
}

/// Return metadata for every exercise in this crate.
#[must_use]
pub fn get_exercise_list() -> Vec<ExerciseInfo> {
    vec![
        ExerciseInfo {
            name: "checksum",
            description: "Hex formatting, word sums, and carry folding",
            concepts: vec![
                "Function parameters and return types",
                "Composition (`internet_checksum` calls `fold_checksum`)",
                "Const vs runtime functions",
                "Nibble masking helpers",
            ],
        },
        ExerciseInfo {
            name: "seal",
            description: "Length-prefixed packets with embedded checksums",
            concepts: vec![
                "`Option` for oversize payloads",
                "Tuple returns `(Vec<u8>, u16)`",
                "Odd-byte padding to 16-bit words",
                "Verify-by-recompute pattern",
            ],
        },
    ]
}

/// Run a single exercise by name.
pub fn run_exercise(name: &str, verbose: bool) -> Result<()> {
    match name {
        "checksum" => {
            checksum_exercise::run(verbose);
            Ok(())
        }
        "seal" => {
            seal_exercise::run(verbose);
            Ok(())
        }
        _ => anyhow::bail!("Unknown exercise: {name}. Available: checksum, seal"),
    }
}

/// Run every exercise in sequence.
pub fn run_all(verbose: bool) -> Result<()> {
    let exercises = ["checksum", "seal"];

    for (i, exercise_name) in exercises.iter().enumerate() {
        println!(
            "🧮 Exercise {} of {}: {}",
            i + 1,
            exercises.len(),
            exercise_name
        );
        println!("{}", "=".repeat(50));

        run_exercise(exercise_name, verbose)?;

        if i < exercises.len() - 1 {
            println!("\n{}\n", "─".repeat(50));
        }
    }

    Ok(())
}
