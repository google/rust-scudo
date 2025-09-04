The `rust-scudo` crate contains Rust bindings for the Scudo hardened
allocator.

This repository contains three crates: [scudo](https://crates.io/crates/scudo),
[scudo-sys](https://crates.io/crates/scudo-sys) and
[scudo-proc-macros](https://crates.io/crates/scudo-proc-macros). The scudo
crate contains an idiomatic Rust interface for Scudo. The latter scudo-sys
crate contains raw C to Rust FFI bindings and the scudo-proc-macro crate
contains a procedural macro to allow the configuration of the Scudo allocator.

Scudo is a dynamic user-mode memory allocator, or heap allocator, designed to be
resilient against heap-related vulnerabilities (such as heap-based buffer
overflow, use after free, and double free) while maintaining performance.

For more information on the allocator, take a look at the following resources:

- [Main Scudo Project](https://llvm.org/docs/ScudoHardenedAllocator.html).
- [Blog post about Scudo internals](https://www.l3harris.com/newsroom/editorial/2023/10/scudo-hardened-allocator-unofficial-internals-documentation)
authored by [bsdaemon@](https://github.com/rrbranco)
