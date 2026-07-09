//! CLI for the 23-unsafe pointer exercises.
//!
//! ```bash
//! cargo run -p exercise_unsafe
//! cargo run -p exercise_unsafe -- --verbose
//! ```

use anyhow::Result;
use clap::Parser;
use unsafe_exercises::run_demo;

/// Minimal unsafe Rust with safe wrappers
#[derive(Parser, Debug)]
#[command(
    name = "exercise_unsafe",
    about = "Raw pointer reads/writes behind safe APIs",
    version
)]
struct Cli {
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    println!("🦀 rust-hoex · 23-unsafe exercises\n");
    println!("WARNING: unsafe is advanced — prefer safe Rust until you must.\n");
    run_demo(cli.verbose)?;
    println!("\n🎉 Demo finished.");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;

    #[test]
    fn cli_parses() {
        Cli::command().debug_assert();
    }
}
