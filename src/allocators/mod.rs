pub mod binary_search_trees;

/// A bit set based allocator; allows reallocations, but requires a linear scan to find free blocks.
pub mod bit_set;

/// Global, switchable allocator.
#[macro_use]
pub mod global;

pub mod allocator;
pub mod bump_allocator;
pub mod context_allocator;
pub mod memory_map_allocator;
pub mod multiple_binary_search_tree_allocator;

pub mod prelude {
    pub use super::binary_search_trees::*;
    pub use super::bit_set::*;
    pub use super::global::*;

    pub use super::allocator::*;
    pub use super::bump_allocator::*;
    pub use super::context_allocator::*;
    pub use super::memory_map_allocator::*;
    pub use super::multiple_binary_search_tree_allocator::*;
}
