//! Generic pair for related security values (IP + score, user + role).

/// A generic pair of related values.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Pair<A, B> {
    /// Left component (e.g. source IP).
    pub left: A,
    /// Right component (e.g. hit count).
    pub right: B,
}

impl<A, B> Pair<A, B> {
    /// Construct a pair.
    pub const fn new(left: A, right: B) -> Self {
        Self { left, right }
    }
}

impl<A: Eq, B: Eq> Pair<A, B> {
    /// Return `true` when both sides equal another pair's sides.
    #[must_use]
    pub fn eq_parts(&self, other: &Self) -> bool {
        self.left == other.left && self.right == other.right
    }
}

/// Run the pair demo.
pub fn run(verbose: bool) -> crate::Result<()> {
    println!("Pair<A, B> demo\n");

    let alert = Pair::new("203.0.113.50", 4_u32);
    let same = Pair::new("203.0.113.50", 4_u32);
    let different = Pair::new("10.0.0.1", 4_u32);

    println!("  alert: {} hits={}", alert.left, alert.right);
    println!("  eq_parts same? {}", alert.eq_parts(&same));
    println!("  eq_parts different IP? {}", alert.eq_parts(&different));

    if verbose {
        println!("  eq requires A: Eq and B: Eq bounds");
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eq_parts_compares_both_sides() {
        let a = Pair::new("alice", 3_u32);
        let b = Pair::new("alice", 3_u32);
        let c = Pair::new("bob", 3_u32);
        assert!(a.eq_parts(&b));
        assert!(!a.eq_parts(&c));
    }

    #[test]
    fn demo_runs() {
        assert!(run(false).is_ok());
    }
}
