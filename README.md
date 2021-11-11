The `rust-scudo` contains the Rust bindings for the Scudo hardened allocator.

This workspace contains two crates: [scudo](https://crates.io/crates/scudo)
and [scudo-sys](https://crates.io/crates/scudo-sys). The former is the
idiomatic Rust interface for Scudo. The latter contains raw C to Rust FFI.

Scudo is a dynamic user-mode memory allocator, or heap allocator, designed to be
resilient against heap-related vulnerabilities (such as heap-based buffer
overflow, use after free, and double free) while maintaining performance.


- [Main Scudo Project](https://llvm.org/docs/ScudoHardenedAllocator.html).
