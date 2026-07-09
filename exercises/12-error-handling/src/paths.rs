//! Exercise — validating config paths and ports.

use anyhow::Result;

use crate::config::{ensure_exists, parse_port, validate_config_path};

/// Run the paths exercise.
pub fn run(verbose: bool) -> Result<()> {
    println!("📁 Paths — validate config locations");
    println!();

    let samples = [
        "/etc/soc/config.toml",
        "",
        "relative.yaml",
        "/etc/../passwd.toml",
        "/etc/app/config.json",
    ];

    for raw in samples {
        match validate_config_path(raw) {
            Ok(path) => match ensure_exists(&path) {
                Ok(()) => println!("  OK   {path} (on allow-list)"),
                Err(e) => println!("  MISS {path}: {e}"),
            },
            Err(e) => println!("  ERR  {raw}: {e}"),
        }
    }

    if verbose {
        println!();
        println!("  ensure_exists checks a hardcoded allow-list — no real FS I/O.");
    }

    println!();
    println!("  Port parsing:");
    for sample in ["443", "0", "70000", "abc"] {
        match parse_port(sample) {
            Ok(port) => println!("    {sample} → {port}"),
            Err(e) => println!("    {sample} → {e}"),
        }
    }

    Ok(())
}
