mod ffi;
mod global_allocator;

pub use ffi::{map_chunks, print_stats};
pub use global_allocator::GlobalScudoAllocator;
