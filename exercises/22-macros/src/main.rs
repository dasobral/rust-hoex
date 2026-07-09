//! CLI for the 22-macros declarative macro exercises.
//!
//! ```bash
//! cargo run -p exercise_macros
//! cargo run -p exercise_macros -- --verbose
//! ```

use anyhow::Result;
use clap::Parser;
use macros_exercises::run_macro_demo;

/// Declarative macro exercises — `say!`, `testvec!`, `max_of!`
#[derive(Parser, Debug)]
#[command(
    name = "exercise_macros",
    about = "Demonstrate macro_rules! helpers for logging and fixtures",
    version
)]
struct Cli {
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    println!("🦀 rust-hoex · 22-macros exercises\n");
    run_macro_demo(cli.verbose)?;
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
