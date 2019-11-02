pub mod current_allocator_in_use;
pub mod global_switchable_allocator;
pub mod local_allocator;
pub mod memory_range;
pub mod per_thread_state;
#[macro_use]
pub mod switchable_allocator;

#[macro_use]
pub mod prelude {
    pub use super::current_allocator_in_use::*;
    pub use super::global_switchable_allocator::*;
    pub use super::local_allocator::*;
    pub use super::memory_range::*;
    pub use super::per_thread_state::*;
    #[macro_use]
    pub use super::switchable_allocator::*;
}
