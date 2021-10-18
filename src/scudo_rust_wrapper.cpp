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
