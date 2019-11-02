/// A memory source which uses an arena.
pub mod arena_memory_source;

/// A memory map (mmap) based allocator with support for NUMA.
#[cfg(unix)]
pub mod mmap;

pub mod memory_source;
pub mod rc_memory_source;

pub mod prelude {
    /// A memory source which uses an arena.
    pub use super::arena_memory_source::*;

    /// A memory map (mmap) based allocator with support for NUMA.
    #[cfg(unix)]
    pub use super::mmap::*;

    pub use super::memory_source::*;
    pub use super::rc_memory_source::*;
}
