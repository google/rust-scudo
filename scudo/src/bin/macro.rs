// Copyright 2022 Google LLC
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
// Helper binary that intentionally sets an invalid Scudo flag.
// Usage: cargo run --bin macro

use scudo_proc_macros::set_scudo_options;
use scudo_sys::{scudo_allocate, scudo_deallocate};

/// This sets an invalid flag for Scudo. This doesn't result in an error,
/// but in a warning like "Scudo WARNING: found 1 unrecognized flag(s)".
#[set_scudo_options(invalid = true)]
fn main() {
    unsafe {
        let p = scudo_allocate(128, 16);
        scudo_deallocate(p, 128, 16);
    }
}
