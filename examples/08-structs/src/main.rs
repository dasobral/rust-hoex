//! example: 08-structs
//!
//! Custom data types — user accounts and security credentials.
//!
//! Run from this directory:
//! ```bash
//! cargo run
//! ```
//!
//! Key concepts:
//! - Struct definition and field access
//! - `impl` blocks and methods (`&self`, `&mut self`, `self`)
//! - Associated functions (`new`)
//! - `#[derive(Debug)]` for printable representations
//! - Struct update syntax (`..`)

use example_structs::{Role, UserAccount, rename_account};

fn main() {
    println!("\nStructs: User Accounts & Credentials");
    println!("====================================\n");

    // === Associated function: UserAccount::new ===
    let mut alice = UserAccount::new("alice", "hunter2-hash", Role::Operator);
    println!("Created: {alice:?}"); // Debug derive
    println!("Display name (&self): {}", alice.display_name());
    println!("Can authenticate: {}", alice.can_authenticate());

    // === Field access ===
    println!("\nField access:");
    println!("- username: {}", alice.username);
    println!("- role:     {:?}", alice.role);
    println!("- fails:    {}", alice.failed_logins);
    // `credential` is private — only accessible through methods:
    println!("- credential len: {}", alice.credential_len());

    // === &mut self methods ===
    println!("\nMutable methods:");
    alice.record_failed_login();
    alice.record_failed_login();
    println!(
        "After 2 failures: fails={}, locked={}",
        alice.failed_logins, alice.locked
    );
    alice.record_failed_login();
    println!(
        "After 3 failures: fails={}, locked={}",
        alice.failed_logins, alice.locked
    );
    println!(
        "verify_credential now: {}",
        alice.verify_credential("hunter2-hash")
    );

    alice.unlock();
    alice.set_role(Role::Admin);
    println!(
        "Unlocked & promoted: role={:?}, locked={}",
        alice.role, alice.locked
    );

    // === Struct update syntax ===
    let bob = rename_account(&alice, "bob");
    println!("\nStruct update syntax → renamed clone:");
    println!("- alice: {}", alice.username);
    println!("- bob:   {} (role={:?})", bob.username, bob.role);

    // === self by value (consuming method) ===
    let audit = bob.into_audit_line();
    println!("\nConsuming method into_audit_line:");
    println!("  {audit}");
    // `bob` was moved; alice remains.
    println!("Alice still available: {}\n", alice.display_name());
}
