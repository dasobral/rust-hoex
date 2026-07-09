//! Compare two string slices with a shared output lifetime.

/// Return the shorter of two string slices.
///
/// Both inputs and the output share `'a`: the result may borrow from either
/// argument, so both must live at least as long as the returned `&str`.
#[must_use]
pub const fn shortest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() <= y.len() { x } else { y }
}

/// Run the shortest demo.
pub fn run(verbose: bool) -> crate::Result<()> {
    println!("shortest<'a> demo\n");

    let alert = String::from("CRITICAL: disk pressure on /var");
    let notice = "ok";
    let winner = shortest(alert.as_str(), notice);
    println!("  alert: {alert}");
    println!("  notice: {notice}");
    println!("  shortest: {winner}");

    if verbose {
        println!("  both borrows must outlive `winner`");
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shortest_picks_shorter_slice() {
        assert_eq!(shortest("long message", "hi"), "hi");
        assert_eq!(shortest("equal1", "equal2"), "equal1");
    }

    #[test]
    fn demo_runs() {
        assert!(run(false).is_ok());
    }
}
