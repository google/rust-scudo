Rust bindings for the
[Scudo Hardened Allocator](https://llvm.org/docs/ScudoHardenedAllocator.html).

Scudo is a dynamic user-mode memory allocator, or heap allocator, designed to be
resilient against heap-related vulnerabilities (such as heap-based buffer
overflow, use after free, and double free) while maintaining performance.

This crate implements safe and standard wrappers around the FFI defined in
`scudo-sys`, including
[GlobalAlloc](https://doc.rust-lang.org/stable/std/alloc/trait.GlobalAlloc.html).
