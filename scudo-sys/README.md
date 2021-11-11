C-Rust FFI bindings for the Scudo Hardened Allocator.

Most users should prefer the idiomatic Rust binding crate,
[scudo](https://crates.io/crates/scudo).

Scudo is a user space heap allocator designed to be reistent to heap
exploitation. **It is useful to you if your program allocates memory and you
depend on unsafe code or you want defense-in-depth against heap exploitation.**
In addition to security, it achieves competitive performance against jemalloc,
tcmalloc and others.

- [Performance Comparison](http://expertmiami.blogspot.com/2019/05/what-is-scudo-hardened-allocator_10.html)
- [Main Project](https://llvm.org/docs/ScudoHardenedAllocator.html)

