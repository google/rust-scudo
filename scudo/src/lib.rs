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
#![cfg_attr(feature = "allocator_api", feature(allocator_api))]
#![cfg_attr(feature = "allocator_api", feature(nonnull_slice_from_raw_parts))]
#[cfg(test)]
#[macro_use]
extern crate std;

use scudo_sys::{scudo_allocate, scudo_deallocate, scudo_print_stats, SCUDO_MIN_ALIGN};

use core::alloc::{GlobalAlloc, Layout};
use core::cmp::max;

/// Zero sized type representing the global static scudo allocator declared in C.
#[derive(Clone, Copy)]
pub struct GlobalScudoAllocator;

/// Returns `layout` or the minimum size/align layout for scudo if its too small.
fn fit_layout(layout: Layout) -> Layout {
    // SAFETY: SCUDO_MIN_ALIGN is constant and known to be powers of 2.
    let min_align = unsafe { SCUDO_MIN_ALIGN } as usize;
    let align = max(min_align, layout.align());
    // SAFETY: Size and align are good by construction.
    unsafe { Layout::from_size_align_unchecked(layout.size(), align) }
}

unsafe impl GlobalAlloc for GlobalScudoAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let layout = fit_layout(layout);
        scudo_allocate(layout.size(), layout.align()) as _
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        let layout = fit_layout(layout);
        scudo_deallocate(ptr as _, layout.size(), layout.align());
    }
}

impl GlobalScudoAllocator {
    /// Prints the global Scudo allocator's internal statistics.
    pub fn print_stats() {
        unsafe { scudo_print_stats() }
    }
}

#[cfg(feature = "allocator_api")]
use core::alloc::AllocError;
#[cfg(feature = "allocator_api")]
use core::ptr::NonNull;
#[cfg(feature = "allocator_api")]
unsafe impl core::alloc::Allocator for GlobalScudoAllocator {
    fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        let layout = fit_layout(layout);
        // TODO(cneo): Scudo buckets and therefore overallocates. Use SizeClassMap to
        // return the correct length for the slice?
        let ptr = unsafe { scudo_allocate(layout.size(), layout.align()) } as _;
        let n = NonNull::new(ptr).ok_or(AllocError)?;
        Ok(NonNull::slice_from_raw_parts(n, layout.size()))
    }
    unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
        let layout = fit_layout(layout);
        scudo_deallocate(ptr.as_ptr() as _, layout.size(), layout.align());
    }
}

#[cfg(test)]
pub mod test {
    use super::*;
    use std::prelude::v1::*;

    use core::alloc::Layout;
    use libc::{c_ulong, c_void, size_t};
    use scudo_sys::{scudo_disable, scudo_enable, scudo_iterate};

    extern "C" fn contains(_address: c_ulong, size: size_t, pair: *mut c_void) {
        let (target_size, count) = unsafe { &mut *(pair as *mut (usize, usize)) };
        if size == *target_size {
            *count += 1;
        }
    }

    /// Test-only function that returns the number of allocations of a given size.
    fn count_allocations_by_size(size: usize) -> usize {
        let mut size_and_count = (size, 0usize);
        unsafe {
            scudo_disable();
            scudo_iterate(
                contains,
                &mut size_and_count as *mut (usize, usize) as *mut c_void,
            );
            scudo_enable();
        }
        size_and_count.1
    }
    #[test]
    fn test_alloc_and_dealloc_use_scudo() {
        let a = GlobalScudoAllocator;
        let layout = Layout::from_size_align(4242, 16).unwrap();
        assert_eq!(count_allocations_by_size(4242), 0);
        let p = unsafe { a.alloc(layout) };
        assert_eq!(count_allocations_by_size(4242), 1);
        unsafe { a.dealloc(p, layout) };
        assert_eq!(count_allocations_by_size(4242), 0);
    }

    #[global_allocator]
    static A: GlobalScudoAllocator = GlobalScudoAllocator;

    #[test]
    fn test_vec_uses_scudo() {
        assert_eq!(count_allocations_by_size(8200_1337), 0);
        let mut v = vec![8u8; 8200_1337];
        assert_eq!(count_allocations_by_size(8200_1337), 1);
        v.clear();
        v.shrink_to_fit();
        assert_eq!(count_allocations_by_size(8200_1337), 0);
    }

    #[cfg(feature = "allocator_api")]
    #[test]
    fn test_vec_with_custom_allocator_uses_scudo() {
        assert_eq!(count_allocations_by_size(8200_4242), 0);
        let mut v = Vec::<u8, GlobalScudoAllocator>::with_capacity_in(8200_4242, A);
        assert_eq!(count_allocations_by_size(8200_4242), 1);
        v.shrink_to_fit();
        assert_eq!(count_allocations_by_size(8200_4242), 0);
    }

    #[test]
    fn test_box_uses_scudo() {
        assert_eq!(count_allocations_by_size(20), 0);
        let b = Box::new([3.0f32; 5]);
        assert_eq!(count_allocations_by_size(20), 1);
        // Move b
        (move || b)();
        assert_eq!(count_allocations_by_size(20), 0);
    }

    #[cfg(feature = "allocator_api")]
    #[test]
    fn test_box_with_custom_allocator_uses_scudo() {
        assert_eq!(count_allocations_by_size(28), 0);
        let b = Box::new_in([3.0f32; 7], A);
        assert_eq!(count_allocations_by_size(28), 1);
        // Move b
        (move || b)();
        assert_eq!(count_allocations_by_size(28), 0);
    }


    #[test]
    fn test_1byte_box_uses_scudo() {
        // Unlike the other arbitrary size allocations, it seems
        // Rust's test harness does have some 1 byte allocations so we cannot
        // assert there are 0, then 1, then 0.
        let before = count_allocations_by_size(1);
        let b = Box::new(1i8);
        assert_eq!(count_allocations_by_size(1), before + 1);
        // Move b
        (move || b)();
        assert_eq!(count_allocations_by_size(1), before);
    }
}
