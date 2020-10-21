use crate::allocators::global::global_switchable_allocator::GlobalSwitchableAllocator;
use crate::memory_address::MemoryAddress;
use crate::memory_sources::memory_source::MemorySource;
use std::alloc::AllocError;
use std::num::NonZeroUsize;
use std::ops::Deref;
use std::rc::Rc;

/// Represents a Reference-counted (RC) memory source.
///
/// Useful when passing in a memory source which does not implement `Clone` to an allocator.
#[derive(Debug)]
pub struct RcMemorySource<MS: MemorySource>(Rc<MS>);

impl<MS: MemorySource> Clone for RcMemorySource<MS> {
    #[inline(always)]
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<MS: MemorySource> Deref for RcMemorySource<MS> {
    type Target = MS;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<MS: MemorySource> MemorySource for RcMemorySource<MS> {
    #[inline(always)]
    fn obtain(&self, non_zero_size: NonZeroUsize) -> Result<MemoryAddress, AllocError> {
        self.0.obtain(non_zero_size)
    }

    #[inline(always)]
    fn release(&self, non_zero_size: NonZeroUsize, current_memory: MemoryAddress) {
        self.0.release(non_zero_size, current_memory)
    }
}

impl<MS: MemorySource> RcMemorySource<MS> {
    /// Creates a new thread-local instance.
    #[inline(always)]
    pub fn new_thread_local<GTACSA: GlobalSwitchableAllocator>(
        global_allocator: &GTACSA,
        underlying_memory_source: MS,
    ) -> Self {
        Self(
            global_allocator
                .callback_with_thread_local_allocator(|| Rc::new(underlying_memory_source)),
        )
    }

    /// Creates a new coroutine-local instance.
    #[inline(always)]
    pub fn new_coroutine_local<GTACSA: GlobalSwitchableAllocator>(
        global_allocator: &GTACSA,
        underlying_memory_source: MS,
    ) -> Self {
        Self(
            global_allocator
                .callback_with_coroutine_local_allocator(|| Rc::new(underlying_memory_source)),
        )
    }
}
