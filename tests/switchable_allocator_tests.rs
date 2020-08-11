#![feature(allocator_api)]
#![feature(extern_types)]
#![feature(core_intrinsics)]
#![feature(libstd_sys_internals)]
#![feature(thread_local)]
#![feature(const_fn)]
#![feature(nonnull_slice_from_raw_parts)]

#[cfg(test)]
mod switchable_allocator_tests {
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

    #[test]
    pub fn switchable_generation() {
        let _vec = Vec::<usize>::with_capacity(1234);
    }
}
