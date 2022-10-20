#!/bin/bash
#
# Copyright 2021 Google LLC
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     https://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.
#
#
# This script tests Scudo error types [1] that lead to process abort. These
# cannot be tested within the standard test harness which does not support
# aborts. This script is expected to be run in the root of the rust-scudo
# workspace directory.
#
# [1] https://llvm.org/docs/ScudoHardenedAllocator.html#error-types

cargo build --release --bins || exit 1

# Run the `crash` binary with the first arg. This binary will crash.
# If the second arg is present in the crash message, the test passes.
function run_test {
  tmp=$(mktemp)
  target/release/$1 $2 2>$tmp
  if grep -q "$3" $tmp
  then
    echo "Test '$3' pass"
  else
    echo "Test '$3' failed"
    exit 1
  fi
}

function run_crash_test {
  run_test scudo_test_crash $1 "$2"
}

function run_macro_test {
  run_test scudo_test_macro "" "$1"
}

run_crash_test double_free "invalid chunk state"
run_crash_test misaligned_ptr "misaligned pointer"
run_crash_test corrupted_chunk_header "corrupted chunk header"
run_crash_test delete_size_mismatch "invalid sized delete"
run_macro_test "Scudo WARNING: found 1 unrecognized flag(s)"

echo "All tests pass"
