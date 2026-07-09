//! Generic search helpers over slices of sensor readings.

/// Return a reference to the minimum element, or `None` if the slice is empty.
#[must_use]
pub fn find_min<T: PartialOrd>(items: &[T]) -> Option<&T> {
    let mut best: Option<&T> = None;
    for item in items {
        best = match best {
            Some(current) if current <= item => Some(current),
            _ => Some(item),
        };
    }
    best
}

/// Run the search demo.
pub fn run(verbose: bool) -> crate::Result<()> {
    println!("find_min<T: PartialOrd> demo\n");

    let latency_ms = [85_u32, 12, 40, 9, 33];
    match find_min(&latency_ms) {
        Some(min) => println!("  min latency: {min} ms"),
        None => println!("  empty sample"),
    }

    let scores = [91_i32, 55, 42, 90, 3];
    if let Some(min) = find_min(&scores) {
        println!("  min threat score: {min}");
    }

    if verbose {
        println!("  compiler monomorphizes one find_min per concrete T");
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_min_on_scores() {
        let scores = [10_u32, 55, 42, 90, 3];
        assert_eq!(find_min(&scores), Some(&3));
        let empty: [u32; 0] = [];
        assert_eq!(find_min(&empty), None);
    }

    #[test]
    fn demo_runs() {
        assert!(run(false).is_ok());
    }
}
