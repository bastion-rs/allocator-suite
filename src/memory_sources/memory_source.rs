use crate::memory_address::MemoryAddress;
use std::alloc::AllocErr;
use std::fmt::Debug;
use std::num::NonZeroUsize;

/// A memory source is a sort-of crude allocator that can obtain and release memory, from, say, the operating system, an arena or some fixed range.
///
/// It is thread-aware but not necessarily thread-safe.
pub trait MemorySource: Debug {
    /// Obtain memory from the operating system, say.
    ///
    /// Alignment will be whatever is appropriate, but is likely to be quite large.
    fn obtain(&self, non_zero_size: NonZeroUsize) -> Result<MemoryAddress, AllocErr>;

    /// Release memory to the operating system, say.
    ///
    /// Alignment will be whatever is appropriate, but is likely to be quite large.
    fn release(&self, non_zero_size: NonZeroUsize, current_memory: MemoryAddress);
}
