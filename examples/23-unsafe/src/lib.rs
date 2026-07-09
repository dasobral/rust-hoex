//! Minimal `unsafe` Rust — raw pointers, SAFETY comments, safe wrappers.
//!
//! # Read this first
//!
//! **Do not use `unsafe` until you must.** The workspace forbids it by
//! default (`unsafe_code = "forbid"`). This package alone allows it so we can
//! teach the keyword in a tiny, reviewed surface area.
//!
//! Goals:
//!
//! 1. Show what `unsafe` unlocks (dereferencing raw pointers).
//! 2. Document **invariants** in `// SAFETY:` comments.
//! 3. Wrap the dangerous bits in a **safe API** so callers never write `unsafe`.
//!
//! Everything outside the marked blocks is ordinary safe Rust.

/// Read the element at `index` from `slice` via a raw pointer.
///
/// # Safety
///
/// Caller must ensure `index < slice.len()`. The pointer is derived from a
/// live shared reference, so it is valid for reads for the duration of this
/// call, and we only read `T` (no write, no aliasing violation).
const unsafe fn read_at_unchecked<T: Copy>(slice: &[T], index: usize) -> T {
    // SAFETY: caller promised `index < slice.len()`, so `add` stays in-bounds
    // for this allocation, and `read` loads an initialized `T`.
    let ptr = slice.as_ptr();
    unsafe { ptr.add(index).read() }
}

/// Safe wrapper: bounds-check, then call the unsafe helper.
///
/// Callers of *this* function never need `unsafe`. That is the usual pattern:
/// concentrate unsafety, prove invariants at the boundary, expose safe API.
#[must_use]
pub const fn read_at<T: Copy>(slice: &[T], index: usize) -> Option<T> {
    if index < slice.len() {
        // SAFETY: we just verified `index < slice.len()`.
        Some(unsafe { read_at_unchecked(slice, index) })
    } else {
        None
    }
}

/// Sum all elements using pointer walking — still behind a safe function.
///
/// Demonstrates a tiny unsafe loop with an explicit remaining-length
/// invariant instead of iterator sugar.
#[must_use]
pub const fn sum_i32(slice: &[i32]) -> i32 {
    let mut total = 0_i32;
    let mut remaining = slice.len();
    let mut ptr = slice.as_ptr();

    while remaining > 0 {
        // SAFETY: `remaining > 0` and `ptr` points at the first of `remaining`
        // initialized `i32`s within `slice`. We read one, then advance.
        let value = unsafe { ptr.read() };
        total = total.saturating_add(value);
        // SAFETY: after reading, `remaining - 1` elements remain; `add(1)` is
        // either in-bounds or one-past-the-end (allowed for pointers, not deref).
        ptr = unsafe { ptr.add(1) };
        remaining -= 1;
    }
    total
}

/// Compare safe indexing vs the raw-pointer path (same results when in-bounds).
#[must_use]
pub const fn demo_values(data: &[i32]) -> (Option<i32>, Option<i32>, i32) {
    let via_safe = data.first().copied();
    let via_raw = read_at(data, 0);
    let total = sum_i32(data);
    (via_safe, via_raw, total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_at_in_bounds() {
        let data = [10_i32, 20, 30];
        assert_eq!(read_at(&data, 1), Some(20));
    }

    #[test]
    fn read_at_out_of_bounds() {
        let data = [1_i32, 2];
        assert_eq!(read_at(&data, 99), None);
    }

    #[test]
    fn sum_i32_matches_iterator() {
        let data = [1_i32, 2, 3, 4];
        assert_eq!(sum_i32(&data), data.iter().sum());
    }

    #[test]
    fn demo_values_agree() {
        let data = [7_i32, 8, 9];
        let (a, b, total) = demo_values(&data);
        assert_eq!(a, b);
        assert_eq!(total, 24);
    }

    #[test]
    fn unchecked_matches_index_when_proven() {
        let data = [5_i32, 6];
        // Test-only: we uphold the SAFETY contract ourselves.
        let got = unsafe { read_at_unchecked(&data, 0) };
        assert_eq!(got, data[0]);
    }
}
