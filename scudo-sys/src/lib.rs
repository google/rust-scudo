use libc::{c_int, c_ulong, c_void, size_t, uintptr_t};

type ChunkCallback = extern "C" fn(base: c_ulong, size: size_t, arg: *mut c_void);
extern "C" {
    pub fn scudo_free(ptr: *mut c_void);
    // Preferred over `malloc` because, in Rust, size and alignment are always known at
    // allocation sites.
    pub fn scudo_posix_memalign(memptr: *mut *mut c_void, alignment: size_t, size: size_t)
        -> c_int;
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

