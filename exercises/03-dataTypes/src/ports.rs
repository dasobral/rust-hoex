//! Port scalar exercise — TCP endpoint fields and flow direction.

use crate::packet::{TcpPorts, format_port_pair, is_well_known, swap_ports};

/// Run the ports exercise with demo output.
pub fn run(verbose: bool) {
    println!("🔌 TCP Ports — Scalar `u16` Endpoint Fields");
    println!();

    let inbound = TcpPorts {
        source_port: 52341,
        dest_port: 443,
    };
    println!("1. Inbound HTTPS flow: {}", format_port_pair(inbound));
    println!(
        "   dest_port {} is well-known: {}",
        inbound.dest_port,
        is_well_known(inbound.dest_port)
    );
    println!(
        "   source_port {} is well-known: {}",
        inbound.source_port,
        is_well_known(inbound.source_port)
    );

    let outbound = swap_ports(inbound);
    println!(
        "2. Reversed (outbound view): {}",
        format_port_pair(outbound)
    );

    let dns = TcpPorts {
        source_port: 53,
        dest_port: 53,
    };
    println!("3. DNS resolver binding: {}", format_port_pair(dns));

    if verbose {
        println!();
        println!("   Why it matters:");
        println!("   - `u16` holds 0..=65535 — enough for every TCP/UDP port");
        println!("   - Structs group related scalars without losing type safety");
        println!("   - `const fn` helpers compile away in release builds");
    }
}
