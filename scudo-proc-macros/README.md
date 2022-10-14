This folder contains a Proc-Macro crate for configuring the Scudo allocator with various options.

The exported `set_scudo_options` attribute macro allows to set Scudo options with an annotation on
the main method:

```rust
use scudo_proc_macros::set_scudo_options;

#[set_scudo_options(delete_size_mismatch = false, release_to_os_interval_ms = 1)]
fn main() {
    // Use Scudo with the provided options.
}
```

For more on Scudo options, visit the official documentation [here](https://llvm.org/docs/ScudoHardenedAllocator.html#options).

Please note: the proc macro exported by this crate works both with the [scudo-sys](https://crates.io/crates/scudo-sys) crate as well as with the idiomatic Rust binding crate, [scudo](https://crates.io/crates/scudo).
