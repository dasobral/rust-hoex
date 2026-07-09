//! CLI for the 20-concurrency log-processing exercises.
//!
//! ```bash
//! cargo run -p exercise_concurrency
//! cargo run -p exercise_concurrency -- --filter WARN --workers 4 --verbose
//! ```

use anyhow::Result;
use clap::Parser;
use concurrency_exercises::run_demo;

/// Parallel log processing with threads and mpsc channels
#[derive(Parser, Debug)]
#[command(
    name = "exercise_concurrency",
    about = "Filter log lines in parallel using threads and mpsc",
    version
)]
struct Cli {
    /// Severity level to keep (`ERROR`, `WARN`, `INFO`)
    #[arg(short, long, default_value = "ERROR")]
    filter: String,

    /// Number of worker threads
    #[arg(short, long, default_value_t = 3)]
    workers: usize,

    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    println!("🦀 rust-hoex · 20-concurrency exercises\n");
    run_demo(&cli.filter, cli.workers, cli.verbose)?;
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
