// Copyright 2021 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
use libc::{c_int, c_ulong, c_void, size_t, uintptr_t};

type ChunkCallback = extern "C" fn(base: c_ulong, size: size_t, arg: *mut c_void);
extern "C" {
    /// Bindings to Scudo's implementation of `free`.
    pub fn scudo_free(ptr: *mut c_void);
    // Preferred over `malloc` because, in Rust, size and alignment are always known at
    // allocation sites.
    /// Bindings to Scudo's implementation of `posix_memalign`.
    pub fn scudo_posix_memalign(memptr: *mut *mut c_void, alignment: size_t, size: size_t)
        -> c_int;
    /// Bindings to Scudo's implementation of `malloc`.
    pub fn scudo_malloc(size: size_t) -> *mut c_void;
    /// Call `callback` on all allocations between addresses `[base, base + size)`.
    pub fn scudo_malloc_iterate(
        base: uintptr_t,
        size: size_t,
        callback: ChunkCallback,
        arg: *mut c_void,
    );

    /// Globally locks the scudo allocator.
    pub fn scudo_malloc_enable();
    /// Globally unlocks the scudo allocator.
    pub fn scudo_malloc_disable();

    pub fn __scudo_print_stats();
}
