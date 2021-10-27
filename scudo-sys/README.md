Rust/C FFI bindings for the [Scudo Hardened Allocator](https://llvm.org/docs/ScudoHardenedAllocator.html).

Scudo is a dynamic user-mode memory allocator, or heap allocator, designed to be
resilient against heap-related vulnerabilities (such as heap-based buffer
overflow, use after free, and double free) while maintaining performance.

This crate builds scudo and defines the unsafe Rust bindings.
