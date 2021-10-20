use std::alloc::{GlobalAlloc, Layout};

use crate::ffi::{scudo_free, scudo_posix_memalign};
use libc::c_void;

/// Zero sized type representing the global static scudo allocator declared in C.
pub struct GlobalScudoAllocator;

unsafe impl GlobalAlloc for GlobalScudoAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        // Use `posix_memalign` over `malloc` because Rust allocators always give
        // type and alignment, and may be overaligned. `memalign`, `valloc` and
        // `pvalloc` are obsolete according to linux's man page.
        // NOTE: Scudo enforces a minimum alignment of sizeof(void*).
        let mut ptr = std::ptr::null_mut();
        let min_size = std::mem::size_of::<*const c_void>();
        let alignment = std::cmp::max(min_size, layout.align());
        //TODO(cneo): std::intrinsics::unlikely.
        if scudo_posix_memalign(&mut ptr as _, alignment, layout.size()) == 0 {
            ptr as *mut u8
        } else {
            std::ptr::null_mut()
        }
    }
    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        scudo_free(ptr as *mut c_void)
    }
}
