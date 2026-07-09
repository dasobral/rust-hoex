//! Policy exercise — mutable borrows and slice analysis.

use anyhow::Result;

use crate::password::{average_scores, mask_keep_last, meets_policy, update_strength};

/// Run the policy exercise with demo output.
pub fn run(verbose: bool) -> Result<()> {
    let candidates = ["short1!", "GoodPass1!", "Tr0ub4dor&3ExtraLong"];
    println!("📋 Policy — Mutable Borrows and Slice Analysis");
    println!();

    for candidate in candidates {
        let ok = meets_policy(candidate, 8);
        let mut score = 0;
        update_strength(&mut score, candidate);
        println!(
            "- \"{}\" → policy ok: {ok}, strength: {score}",
            mask_keep_last(candidate, 2)
        );
    }

    let history = [10_i32, 25, 30, 35];
    match average_scores(&history) {
        Some(avg) => println!("\nAverage threat history {:?}: {avg}", &history[..]),
        None => println!("\nThreat history was empty"),
    }

    if verbose {
        println!();
        println!("   `&mut i32` grants exclusive write access to the score.");
        println!("   `&[i32]` borrows a slice without copying the array.");
    }

    Ok(())
}
