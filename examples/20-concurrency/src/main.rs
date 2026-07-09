//! example: 20-concurrency
//!
//! Parallel log processing with threads and `mpsc` channels.
//!
//! ```bash
//! cargo run
//! ```

use example_concurrency::{count_by_level, process_logs};

fn main() {
    println!("=== 20-concurrency: threads + mpsc ===\n");

    let lines = vec![
        "INFO  service started".into(),
        "WARN  high latency".into(),
        "ERROR auth failed".into(),
        "INFO  request ok".into(),
        "WARN  retrying".into(),
        "ERROR disk full".into(),
        "INFO  shutdown".into(),
        "DEBUG noise".into(),
    ];

    match process_logs(lines, 3) {
        Ok(hits) => {
            println!("received {} hits:", hits.len());
            for hit in &hits {
                println!("  [worker {}] {:>5}  {}", hit.worker, hit.level, hit.line);
            }
            println!("\nby level:");
            for (level, n) in count_by_level(&hits) {
                println!("  {level}: {n}");
            }
        }
        Err(err) => {
            eprintln!("pipeline failed: {err:?}");
        }
    }
}
