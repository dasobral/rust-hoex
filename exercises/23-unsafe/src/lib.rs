//! Minimal `unsafe` Rust — raw pointers, SAFETY comments, safe wrappers.
//!
//! Callers of the public API never write `unsafe`. Concentrate unsafety in
//! tiny helpers and prove invariants at the boundary.

/// Read the byte at `idx` from `slice` via `ptr::read`.
#[must_use]
pub const fn read_at(slice: &[u8], idx: usize) -> Option<u8> {
    if idx >= slice.len() {
        return None;
    }
    // SAFETY: `idx < slice.len()` so `add(idx)` stays in-bounds for this allocation.
    let ptr = slice.as_ptr();
    Some(unsafe { ptr.add(idx).read() })
}

/// Write `val` at `idx` in `slice` via `ptr::write`.
pub const fn write_at(slice: &mut [u8], idx: usize, val: u8) -> Option<()> {
    if idx >= slice.len() {
        return None;
    }
    // SAFETY: `idx < slice.len()` so the target address is initialized and writable.
    let ptr = slice.as_mut_ptr();
    unsafe {
        ptr.add(idx).write(val);
    }
    Some(())
}

/// Find the maximum `i32` in `slice` by walking a raw pointer.
#[must_use]
pub fn max_i32(slice: &[i32]) -> Option<i32> {
    if slice.is_empty() {
        return None;
    }

    let mut best = slice[0];
    let mut remaining = slice.len().saturating_sub(1);
    let mut ptr = slice.as_ptr();

    // SAFETY: skip the first element we already loaded via safe indexing.
    if remaining > 0 {
        ptr = unsafe { ptr.add(1) };
    }

    while remaining > 0 {
        // SAFETY: `remaining` counts initialized elements ahead of `ptr` within `slice`.
        let value = unsafe { ptr.read() };
        if value > best {
            best = value;
        }
        // SAFETY: advance to the next element; may point one-past-the-end when done.
        ptr = unsafe { ptr.add(1) };
        remaining -= 1;
    }

    Some(best)
}

/// Demo helper comparing safe indexing vs raw read and reporting max.
#[must_use]
pub fn demo_buffer(data: &[u8]) -> (Option<u8>, Option<u8>, Option<i32>) {
    let via_index = data.first().copied();
    let via_raw = read_at(data, 0);
    let ints: Vec<i32> = data.iter().map(|&b| i32::from(b)).collect();
    let peak = max_i32(&ints);
    (via_index, via_raw, peak)
}

/// Run the unsafe demo on sample buffers.
pub fn run_demo(verbose: bool) -> anyhow::Result<()> {
    let mut buf = *b"RUST";
    if verbose {
        println!("initial buffer: {}", String::from_utf8_lossy(&buf));
    }

    let _ = write_at(&mut buf, 0, b'r');
    say_read(&buf, 0);
    say_read(&buf, 99);

    let ints = [4_i32, 8, 15, 16, 23, 42];
    if let Some(max) = max_i32(&ints) {
        println!("max_i32: {max}");
    }

    let (a, b, peak) = demo_buffer(&buf);
    println!("first via index: {a:?}, via raw: {b:?}, peak as i32: {peak:?}");
    Ok(())
}

fn say_read(buf: &[u8], idx: usize) {
    match read_at(buf, idx) {
        Some(byte) => println!("read_at({idx}) = {byte} ('{}')", byte as char),
        None => println!("read_at({idx}) = None (out of bounds)"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_at_in_bounds() {
        let data = [10_u8, 20, 30];
        assert_eq!(read_at(&data, 1), Some(20));
    }

    #[test]
    fn read_at_out_of_bounds() {
        assert_eq!(read_at(&[1_u8], 5), None);
    }

    #[test]
    fn write_at_updates() {
        let mut data = [1_u8, 2, 3];
        assert_eq!(write_at(&mut data, 1, 9), Some(()));
        assert_eq!(data, [1, 9, 3]);
        assert_eq!(write_at(&mut data, 9, 0), None);
    }

    #[test]
    fn max_i32_finds_peak() {
        assert_eq!(max_i32(&[1_i32, 5, 3, 9, 2]), Some(9));
        assert_eq!(max_i32(&[]), None);
        assert_eq!(max_i32(&[7_i32]), Some(7));
    }

    #[test]
    fn demo_buffer_agrees() {
        let data = [7_u8, 8, 9];
        let (a, b, peak) = demo_buffer(&data);
        assert_eq!(a, b);
        assert_eq!(peak, Some(9));
    }
}
