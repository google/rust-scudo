extern crate cc;

use std::fs::read_dir;
use std::path::Path;

fn main() {
    println!("cargo:rerun-if-changed=src/scudo_rust_wrapper.c");
    let scudo_dir = Path::new("llvm-project/compiler-rt/lib/scudo/standalone");

    // Get all .cpp files besides the wrappers.
    let scudo_cpp_files = read_dir(scudo_dir).unwrap().filter_map(|e| {
        let entry = e.unwrap();
        let path = entry.path();
        let filename = path.file_name().unwrap().to_str().unwrap();
        if filename.ends_with("cpp") && !filename.starts_with("wrapper") {
            Some(path)
        } else {
            None
        }
    });

    cc::Build::new()
        .files(scudo_cpp_files)
        .file("src/scudo_rust_wrapper.cpp")
        .include(scudo_dir)
        .include(scudo_dir.join("include"))
        .cpp(true)
        .pic(true) // Position Independent Code.
        .shared_flag(true)
        .compile("scudo");

    // Opt level is inferred from Cargo and environment variables.

    // TODO(cneo): -pthread -msse -std=c++17? Those flags are present at:
    //             https://llvm.org/docs/ScudoHardenedAllocator.html#library
}
