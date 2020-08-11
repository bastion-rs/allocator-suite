#![feature(test)]
#![feature(allocator_api)]
#![feature(extern_types)]
#![feature(core_intrinsics)]
#![feature(libstd_sys_internals)]
#![feature(thread_local)]
#![feature(const_fn)]
#![feature(nonnull_slice_from_raw_parts)]

extern crate test;

#[cfg(test)]
mod switchable_perf {
    // Import bencher
    use test::Bencher;

    #[bench]
    pub fn bench_switchable_allocator(b: &mut Bencher) {
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

        b.iter(|| {
            let mut vec = Vec::<usize>::with_capacity(10_000_000);
            (0..1_000_000).for_each(|_| {
                vec.push(100);
            });
        })
    }
}
