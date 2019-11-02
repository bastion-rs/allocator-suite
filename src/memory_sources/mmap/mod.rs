

pub mod huge_page_size;
pub mod memory_map_source;

/// NUMA memory mapping.
pub mod numa;

pub mod prelude {
    pub use super::huge_page_size::*;
    pub use super::memory_map_source::*;
    pub use super::numa::prelude::*;
}
