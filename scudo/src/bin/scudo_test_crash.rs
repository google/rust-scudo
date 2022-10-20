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
//
// Helper binary that intentionally does unsafe memory to test Scudo protections.
// Usage: cargo run --bin crash -- ${action_name}
use scudo_sys::{scudo_allocate, scudo_deallocate};

fn double_free() {
    unsafe {
        let p = scudo_allocate(128, 16);
        scudo_deallocate(p, 128, 16);
        scudo_deallocate(p, 128, 16);
    }
}
fn misaligned_ptr() {
    unsafe {
        let mut p = scudo_allocate(128, 16);
        p = p.add(1);
        scudo_deallocate(p, 128, 16);
    }
}
fn corrupted_chunk_header() {
    unsafe {
        let mut p = scudo_allocate(16, 16);
        p = p.add(16);
        scudo_deallocate(p, 16, 16);
    }
}
fn delete_size_mismatch() {
    unsafe {
        let p = scudo_allocate(128, 16);
        scudo_deallocate(p, 64, 16);
    }
}

fn main() {
    let action_name = std::env::args().nth(1).expect("Expected action name");
    match action_name.as_str() {
        "double_free" => double_free(),
        "misaligned_ptr" => misaligned_ptr(),
        "corrupted_chunk_header" => corrupted_chunk_header(),
        "delete_size_mismatch" => delete_size_mismatch(),
        _ => println!("Could not find an action named `{:?}`", action_name),
    }
}
