//! Data type exercises — packet header scalars and compound types.
//!
//! Demonstrates `u8`/`u16` wire fields, bit masks, tuples, and fixed arrays
//! in a cybersecurity packet-inspection context.

pub mod packet;
pub mod ports;
pub mod tos_protocol;

pub use packet::{
    PacketHeader, TcpPorts, format_ipv4, format_port_pair, header_words, parse_header, parse_tos,
    protocol_label, read_u16_be, sample_header_bytes, swap_ports,
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
            name: "ports",
            description: "TCP source/dest port scalars and flow reversal",
            concepts: vec![
                "`u16` port fields",
                "Struct compounds for endpoints",
                "`const fn` predicates",
                "Well-known vs ephemeral ports",
            ],
        },
        ExerciseInfo {
            name: "tos_protocol",
            description: "TOS bit masks, protocol labels, and IPv4 header arrays",
            concepts: vec![
                "Bit shifts and masks (`dscp`, `ecn`)",
                "`char` protocol labels",
                "Fixed `[u8; N]` header buffers",
                "Tuple return types",
            ],
        },
    ]
}

/// Run a single exercise by name.
pub fn run_exercise(name: &str, verbose: bool) -> Result<()> {
    match name {
        "ports" => {
            ports::run(verbose);
            Ok(())
        }
        "tos_protocol" => {
            tos_protocol::run(verbose);
            Ok(())
        }
        _ => anyhow::bail!("Unknown exercise: {name}. Available: ports, tos_protocol"),
    }
}

/// Run every exercise in sequence.
pub fn run_all(verbose: bool) -> Result<()> {
    let exercises = ["ports", "tos_protocol"];

    for (i, exercise_name) in exercises.iter().enumerate() {
        println!(
            "📡 Exercise {} of {}: {}",
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
