use super::*;

pub mod alloc;
pub mod global_alloc;

#[macro_use]
pub mod allocator_adaptor;
pub mod alloc_to_allocator_adaptor;
pub mod global_alloc_to_allocator_adaptor;

pub mod prelude {
    pub use super::alloc::*;
    pub use super::global_alloc::*;

    pub use super::alloc_to_allocator_adaptor::*;
    pub use super::allocator_adaptor::*;
    pub use super::global_alloc_to_allocator_adaptor::*;
}
