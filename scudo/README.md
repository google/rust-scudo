Idiomatic Rust bindings for the Scudo Hardened Allocator.

Scudo is a user space heap allocator designed to be reistent to heap
exploitation. **It is useful to you if your program allocates memory and you
depend on unsafe code or you want defense-in-depth against heap exploitation.**
In addition to security, it achieves competitive performance against jemalloc,
tcmalloc and others.

- [Performance Comparison](http://expertmiami.blogspot.com/2019/05/what-is-scudo-hardened-allocator_10.html)
- [Main Project](https://llvm.org/docs/ScudoHardenedAllocator.html)

To use Scudo in your crate:
```sh
$ cargo add scudo
```

```rust
use scudo::GlobalScudoAllocator;
#[global_allocator]
static SCUDO_ALLOCATOR: GlobalScudoAllocator = GlobalScudoAllocator;
```

If you want to use the unstable `std::alloc::Allocator` trait, use the
`allocator_api` feature.
