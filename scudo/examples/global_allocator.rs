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

// To use the Scudo allocator in your program, simply add the next 4 lines.
extern crate scudo;
use scudo::GlobalScudoAllocator;
use scudo_proc_macros::set_scudo_options;

#[global_allocator]
static SCUDO_ALLOCATOR: GlobalScudoAllocator = GlobalScudoAllocator;

#[set_scudo_options(delete_size_mismatch = false, release_to_os_interval_ms = 1)]
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
