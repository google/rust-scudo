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

// This is similar with the C wrapper except we define a prefix.
// Without it, it may be harder to know if `malloc` is Scudo's or not.
#define SCUDO_PREFIX(name) CONCATENATE(scudo_, name)
#define SCUDO_ALLOCATOR Allocator

extern "C" void SCUDO_PREFIX(malloc_postinit)();

SCUDO_REQUIRE_CONSTANT_INITIALIZATION
scudo::Allocator<scudo::Config, SCUDO_PREFIX(malloc_postinit)> SCUDO_ALLOCATOR;


#include "wrappers_c.inc"

#undef SCUDO_ALLOCATOR
#undef SCUDO_PREFIX

extern "C" INTERFACE void __scudo_print_stats(void) { Allocator.printStats(); }
