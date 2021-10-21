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

#[test]
fn test_box_and_vec_use_scudo() {
    let x = vec![42u8; 123456789];
    let _y = Box::new([0u64; 5]);
    let _z = Box::new([0.0f32; 3]);

    let mut seen_x = false;
    let mut seen_y = false;
    let mut seen_z = false;
    GlobalScudoAllocator::map_chunks(
        &mut |_base_address, chunk_size| {
            seen_x |= chunk_size == x.len();
            seen_y |= chunk_size == 8 * 5;
            seen_z |= chunk_size == 4 * 3;
        },
        0,
        usize::MAX,
    );
    assert!(seen_x);
    assert!(seen_y);
    assert!(seen_z);
}
