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
#include "platform.h"
#include "allocator_config.h"
#include "wrappers_c.h"
#include "wrappers_c_checks.h"

#include <stdint.h>
#include <stdio.h>

// SCUDO_ALLOCATOR below is the global scudo allocator to be used in Rust.
extern "C" void scudo_postinit();
SCUDO_REQUIRE_CONSTANT_INITIALIZATION
scudo::Allocator<scudo::Config, scudo_postinit> SCUDO_ALLOCATOR;

// Define the Rust-C FFI.
// We could depend on wrapper_c.inc and share an interface with posix/C but this
// way we can have a more rust-y interface with Scudo, e.g. knowing size and
// alignment on both allocate and deallocate.
extern "C" {

size_t SCUDO_MIN_ALIGN = 1 << SCUDO_MIN_ALIGNMENT_LOG;

void* scudo_allocate(size_t size, size_t alignment) {
  return SCUDO_ALLOCATOR.allocate(size, scudo::Chunk::Origin::Malloc,
                                  alignment);
}

void scudo_deallocate(void* ptr, size_t size, size_t alignment) {
  SCUDO_ALLOCATOR.deallocate(ptr, scudo::Chunk::Origin::Malloc, size,
                             alignment);
}

void scudo_iterate(void(*callback)(uintptr_t base, size_t size, void* arg),
                   void* arg) {
  // Set base=0 size=MAX to iterate over all chunks in memory.
  SCUDO_ALLOCATOR.iterateOverChunks(/*base=*/0, /*size=*/(size_t)-1, callback,
                                    arg);
}

void scudo_enable() {
  SCUDO_ALLOCATOR.enable();
}

void scudo_disable() {
  SCUDO_ALLOCATOR.disable();
}

void scudo_postinit() {
  SCUDO_ALLOCATOR.initGwpAsan();
  pthread_atfork(scudo_disable, scudo_enable, scudo_enable);
}

void scudo_print_stats() {
  SCUDO_ALLOCATOR.printStats();
}

}  // extern "C"

#undef SCUDO_ALLOCATOR
#undef SCUDO_PREFIX

