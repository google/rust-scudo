use libc::{c_int, c_ulong, c_void, size_t, uintptr_t};

type ChunkCallback = extern "C" fn(base: c_ulong, size: size_t, arg: *mut c_void);
extern "C" {
    // TODO: Set Scudo_prefix to be __scudo so we cannot be confused with system malloc.
    pub fn scudo_free(ptr: *mut c_void);
    // Preferred over `malloc` because, in Rust, size and alignment are always known at
    // allocation sites.
    pub fn scudo_posix_memalign(memptr: *mut *mut c_void, alignment: size_t, size: size_t)
        -> c_int;
    /// Call `callback` on all allocations between addresses `[base, base + size)`.
    fn scudo_malloc_iterate(
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

extern "C" fn callback(base: c_ulong, size: size_t, arg: *mut c_void) {
    // Safe only if `arg` is produced by map_chunks below.
    let f: &mut &mut dyn FnMut(c_ulong, size_t) = unsafe { core::mem::transmute(arg) };
    f(base, size)
}

/// Maps `f` over all chunks in the address range `[base_address, base_address + size)`.
/// This function locks the allocator. `f` must not allocate or a deadlock will occur.
pub fn map_chunks(mut f: &mut dyn FnMut(c_ulong, size_t), base_address: usize, size: usize) {
    // We are passing `f` to be called from `callback` in C++ via a void*.
    // `impl FnMut` is a closure with an anonymous type so `callback` cannot name it.
    // `&mut dyn FnMut` is a nameable type but is 2 pointers which does not fit in void*.
    // `&mut &mut dyn FnMut` is actually 1 pointer and can be cast to void* and passed to C.
    let ptr_to_dynamic_fn = &mut f as *mut _ as *mut c_void;
    unsafe {
        scudo_malloc_disable();
        scudo_malloc_iterate(base_address, size, callback, ptr_to_dynamic_fn);
        scudo_malloc_enable();
    };
}

// Prints the static Scudo Allocator's internal statistics.
pub fn print_stats() {
    unsafe { __scudo_print_stats() }
}

#[test]
fn test_map_allocator() {
    // We send Scudo a large allocation which will be handled by the MapAllocator. This means
    // it will have its own chunk which we can see with map_chunks.
    let size = 1337_8200;
    let mut ptr: *mut c_void = std::ptr::null_mut();
    let scudo_has_a_chunk_with_size_1337_8200 = || {
        let mut seen_size = false;
        map_chunks(
            &mut |_chunk_address, chunk_size| {
                if chunk_size == size {
                    seen_size = true;
                }
            },
            0,
            usize::MAX,
        );
        seen_size
    };

    assert!(!scudo_has_a_chunk_with_size_1337_8200());

    unsafe { scudo_posix_memalign(&mut ptr as *mut *mut c_void, 16, size) };
    assert!(!ptr.is_null());
    assert!(scudo_has_a_chunk_with_size_1337_8200());

    unsafe { scudo_free(ptr) };
    assert!(!scudo_has_a_chunk_with_size_1337_8200());
}
