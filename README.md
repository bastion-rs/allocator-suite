# Allocator Suite

This crate is mostly rewritten version of `context-allocator` crate.
It contains better NUMA-aware global allocator with hygienic macros. 
Contains better likelihood paths and faster execution paths.

## Usage
```rust
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
``` 
