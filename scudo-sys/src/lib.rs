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
#![no_std]
use libc::{c_ulong, c_void, size_t};

type ChunkCallback = extern "C" fn(base: c_ulong, size: size_t, arg: *mut c_void);

extern "C" {
    pub static SCUDO_MIN_ALIGN: size_t;
    /// Allocate memory with the global Scudo allocator.
    pub fn scudo_allocate(size: size_t, align: size_t) -> *mut c_void;

    /// Deallocate memory allocated by the global Scudo allocator.
    pub fn scudo_deallocate(ptr: *mut c_void, size: size_t, align: size_t);

    /// Iterate over all chunks iterated by the global Scudo allocator.
    pub fn scudo_iterate(callback: ChunkCallback, arg: *mut c_void);

    /// Locks the global Scudo allocator.
    pub fn scudo_enable();

    /// Unlocks the global Scudo allocator.
    pub fn scudo_disable();

    /// Prints allocation statistics from the global Scudo allocator.
    pub fn scudo_print_stats();
}
