The `rust-scudo` contains the Rust bindings for the Scudo hardened allocator.

This workspace contains three crates: [scudo](https://crates.io/crates/scudo),
[scudo-sys](https://crates.io/crates/scudo-sys) and
[scudo-proc-macros](https://crates.io/crates/scudo-proc-macros). The scudo
crate contains an idiomatic Rust interface for Scudo. The latter scudo-sys
crate contains raw C to Rust FFI bindings and the scudo-proc-macro crate
contains a procedural macro to allow the configuration of the Scudo allocator.

Scudo is a dynamic user-mode memory allocator, or heap allocator, designed to be
resilient against heap-related vulnerabilities (such as heap-based buffer
overflow, use after free, and double free) while maintaining performance.


- [Main Scudo Project](https://llvm.org/docs/ScudoHardenedAllocator.html).
