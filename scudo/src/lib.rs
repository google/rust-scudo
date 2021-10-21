use libc::{c_ulong, c_void, size_t};
use scudo_sys::{scudo_free, scudo_posix_memalign};
use std::alloc::{GlobalAlloc, Layout};

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

impl GlobalScudoAllocator {
    /// Prints the global Scudo allocator's internal statistics.
    pub fn print_stats() {
        unsafe { scudo_sys::__scudo_print_stats() }
    }
    /// Maps `f` over all chunks in the address range
    /// `[base_ddress, base_adress + size)`, locking the allocator.
    /// `f` must not allocate or deallocate or it will deadlock.
    /// `f` take two arguments, allocation address and size.
    pub fn map_chunks(f: &mut dyn FnMut(c_ulong, size_t), base_address: usize, size: usize) {
        scudo_sys::map_chunks(f, base_address, size);
    }
}
