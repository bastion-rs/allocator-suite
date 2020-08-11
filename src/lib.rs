#![feature(allocator_api)]
#![feature(extern_types)]
#![feature(core_intrinsics)]
#![feature(libstd_sys_internals)]
#![feature(thread_local)]
#![feature(const_fn)]
#![feature(llvm_asm)]
#![feature(nonnull_slice_from_raw_parts)]

/// Path prediction macros for likely/unlikely intrinsics
#[macro_use]
pub mod likeliness;

/// Adapt various allocator traits to one another.
#[macro_use]
pub mod adaptors;

/// Allocators.
#[macro_use]
pub mod allocators;

/// Extensions useful for working with memory; not a stable part of the API of this crate.
pub mod extensions;

/// Memory sources.
pub mod memory_sources;

/// Type alias of memory address
pub mod memory_address;

pub mod prelude {
    pub use crate::adaptors::prelude::*;
    pub use crate::adaptors::*;

    pub use crate::allocators::prelude::*;
    pub use crate::allocators::*;

    pub use crate::extensions::*;

    pub use crate::likeliness::prelude::*;
    pub use crate::likeliness::*;

    pub use crate::memory_address::*;

    pub use crate::memory_sources::prelude::*;
    pub use crate::memory_sources::*;

    pub use crate::allocators::binary_search_trees::prelude::*;
    /// Expose tree structures
    pub use crate::allocators::binary_search_trees::*;

    // Expose macros
    pub use crate::alloc_ref;
    pub use crate::choose_allocator;
    pub use crate::global_alloc;
    pub use crate::likely;
    pub use crate::switchable_allocator;
    pub use crate::unlikely;
}
