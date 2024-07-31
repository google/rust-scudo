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
    let min_align = unsafe { SCUDO_MIN_ALIGN };
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

    /// Test-only type that holds information about an allocation.
    #[repr(C)]
    struct TestAllocation {
        address: c_ulong,
        size: size_t,
        is_valid: bool,
    }

    extern "C" fn contains(address: c_ulong, size: size_t, expected_allocation: *mut c_void) {
        let expected_allocation = unsafe { &mut *(expected_allocation as *mut TestAllocation) };

        if expected_allocation.address == address && expected_allocation.size == size {
            expected_allocation.is_valid = true
        }
    }

    /// Test-only function that returns whether there is an existing allocation at
    /// `address` with the specified `size`.
    fn check_alloc_with_address_and_size<T>(address: *const T, size: usize) -> bool {
        let mut expected_allocation = TestAllocation {
            address: address as c_ulong,
            size,
            is_valid: false,
        };

        unsafe {
            scudo_disable();
            scudo_iterate(
                contains,
                &mut expected_allocation as *mut TestAllocation as *mut c_void,
            );
            scudo_enable();
        }

        expected_allocation.is_valid
    }

    #[test]
    fn test_alloc_and_dealloc_use_scudo() {
        let a = GlobalScudoAllocator;
        let layout = Layout::from_size_align(4242, 16).unwrap();

        let p = unsafe { a.alloc(layout) };
        assert!(check_alloc_with_address_and_size(p, 4242));

        unsafe { a.dealloc(p, layout) };
        assert!(!check_alloc_with_address_and_size(p, 4242));
    }

    #[global_allocator]
    static A: GlobalScudoAllocator = GlobalScudoAllocator;

    #[test]
    fn test_vec_uses_scudo() {
        let mut v = vec![8u8; 8200_1337];
        let ptr = v.as_ptr();

        assert!(check_alloc_with_address_and_size(ptr, 8200_1337));

        v.clear();
        v.shrink_to_fit();
        assert!(!check_alloc_with_address_and_size(ptr, 8200_1337));
    }

    #[cfg(feature = "allocator_api")]
    #[test]
    fn test_vec_with_custom_allocator_uses_scudo() {
        let mut v = Vec::<u8, GlobalScudoAllocator>::with_capacity_in(8200_4242, A);
        let ptr = v.as_ptr();

        assert!(check_alloc_with_address_and_size(ptr, 8200_4242));

        v.shrink_to_fit();
        assert!(!check_alloc_with_address_and_size(ptr, 8200_4242));
    }

    #[test]
    fn test_box_uses_scudo() {
        let b = Box::new([3.0f32; 5]);
        let ptr = b.as_ptr();

        assert!(check_alloc_with_address_and_size(ptr, 20));

        // Move b
        (move || b)();
        assert!(!check_alloc_with_address_and_size(ptr, 20));
    }

    #[cfg(feature = "allocator_api")]
    #[test]
    fn test_box_with_custom_allocator_uses_scudo() {
        let b = Box::new_in([3.0f32; 7], A);
        let ptr = b.as_ptr();

        assert!(check_alloc_with_address_and_size(ptr, 28));

        // Move b
        (move || b)();
        assert!(!check_alloc_with_address_and_size(ptr, 28));
    }

    #[test]
    fn test_1byte_box_uses_scudo() {
        let b = Box::new(1i8);
        let ptr = &*b as *const _;

        assert!(check_alloc_with_address_and_size(
            ptr,
            std::mem::size_of::<i8>()
        ));

        // Move b
        (move || b)();
        assert!(!check_alloc_with_address_and_size(
            ptr,
            std::mem::size_of::<i8>()
        ));
    }
}
