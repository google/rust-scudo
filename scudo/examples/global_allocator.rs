// To use the Scudo allocator in your program, simply add the next 4 lines.
extern crate scudo;
use scudo::GlobalScudoAllocator;
#[global_allocator]
static SCUDO_ALLOCATOR: GlobalScudoAllocator = GlobalScudoAllocator;


fn main() {
    let mut x = vec![[42u8; 1024]; 12345];
    let mut y = vec![Box::new(42u32); 5678];

    println!(
        "Showing Scudo statistics, look for a MapAllocator allocation \
              slightly larger than {} kBit and look for at least {} \
              SizeClassAllocator allocations of 32 bytes.\n\n",
        x.len(),
        y.len()
    );
    GlobalScudoAllocator::print_stats();

    x.clear();
    y.clear();
}
