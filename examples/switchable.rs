#![feature(allocator_api)]
#![feature(extern_types)]
#![feature(core_intrinsics)]
#![feature(libstd_sys_internals)]
#![feature(thread_local)]
#![feature(const_fn)]

// Allocator generator macro
use allocator_suite::switchable_allocator;

// General imports
use allocator_suite::adaptors::prelude::*;
use std::alloc::System;

switchable_allocator!(
    application_allocator,
    BumpAllocator<ArenaMemorySource<MemoryMapSource>>,
    MultipleBinarySearchTreeAllocator<MemoryMapSource>,
    GlobalAllocToAllocatorAdaptor<System>,
    GlobalAllocToAllocatorAdaptor(System)
);

fn main() {
    // Allocated by Global switchable allocator's global allocator.
    let _vec = Vec::<usize>::with_capacity(1234);
}
