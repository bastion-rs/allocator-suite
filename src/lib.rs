#![feature(allocator_api)]
#![feature(extern_types)]
#![feature(core_intrinsics)]
#![feature(libstd_sys_internals)]
#![feature(thread_local)]
#![feature(const_fn)]

/// Path prediction macros for likely/unlikely intrinsics
#[macro_use]
pub mod likeliness;

/// Adapt various allocator traits to one another.
#[macro_use]
pub mod adaptors;

/// Allocators.
pub mod allocators;

/// Extensions useful for working with memory; not a stable part of the API of this crate.
pub mod extensions;

/// Memory sources.
pub mod memory_sources;

/// Type alias of memory address
pub mod memory_address;

pub mod prelude {
    pub use crate::adaptors::*;
    pub use crate::allocators::*;
    pub use crate::extensions::*;
    pub use crate::likeliness::*;
    pub use crate::memory_address::*;
    pub use crate::memory_sources::*;

    /// Expose tree structures
    pub use crate::allocators::binary_search_trees::*;
}
