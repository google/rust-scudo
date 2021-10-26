// Helper binary that intentionally does unsafe memory to test Scudo protections.
// Usage: cargo run --bin crash -- ${action_name}
use scudo_sys::{scudo_malloc, scudo_free};

fn double_free() {
    unsafe {
        let p = scudo_malloc(128);
        scudo_free(p);
        scudo_free(p);
    }
}
fn misaligned_ptr() {
    unsafe {
        let mut p = scudo_malloc(128);
        p = p.add(1);
        scudo_free(p);
    }
}
fn corrupted_chunk_header() {
    unsafe {
        let mut p = scudo_malloc(16);
        p = p.add(16);
        scudo_free(p);
    }
}


fn main() {
    let action_name = std::env::args().skip(1).next().expect("Expected action name");
    match action_name.as_str() {
        "double_free" => double_free(),
        "misaligned_ptr" => misaligned_ptr(),
        "corrupted_chunk_header" => corrupted_chunk_header(),
        _ => println!("Could not find an action named `{:?}`", action_name),
    }
}
