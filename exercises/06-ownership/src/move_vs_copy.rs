//! Move vs Copy exercise — credential handoff and threat scores.

use anyhow::Result;

use crate::secrets::{clone_secret, consume_secret, copy_threat_score, take_then_return};

/// Run the move-vs-copy exercise with demo output.
pub fn run(verbose: bool) -> Result<()> {
    println!("🔐 Move vs Copy — Credential Ownership");
    println!();

    let api_key = String::from("sk-live-7f3a9b2c");
    println!("1. Created owned API key: \"{api_key}\"");

    let handed_off = take_then_return(api_key);
    println!("2. Moved into new owner: \"{handed_off}\"");

    let backup = clone_secret(&handed_off);
    println!("3. Cloned for backup storage: \"{backup}\"");
    println!("   Original still valid after clone: \"{handed_off}\"");

    let consumed_len = consume_secret(handed_off);
    println!("4. Consumed primary key ({consumed_len} bytes) — binding gone");
    println!("   Backup clone still available: \"{backup}\"");

    drop(backup);
    println!("5. Backup dropped — heap buffer released");

    let threat: i32 = 15;
    let escalated = copy_threat_score(threat);
    println!();
    println!("6. Copy type demo (threat score i32):");
    println!("   original score:  {threat}");
    println!("   escalated score: {escalated}");

    if verbose {
        println!();
        println!("   Why it matters:");
        println!("   - `String` moves: one owner prevents double-free");
        println!("   - `Clone` is explicit when you need a second owned copy");
        println!("   - `i32` is `Copy`: pass-by-value duplicates, no move");
    }

    Ok(())
}
