//! SigmaOS-early-userspace — thin CLI entry point.
//!
//! The library lives in `src/lib.rs`; this binary only wires argv/stdout.

use sigmaos_early_userspace_lib::Component;

fn main() {
    let mut component = Component::new();
    if let Err(e) = component.init().and_then(|_| component.run()) {
        eprintln!("SigmaOS-early-userspace: {e}");
        std::process::exit(1);
    }
    println!("SigmaOS-early-userspace: ok");
}
