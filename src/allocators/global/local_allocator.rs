use crate::allocators::global::memory_range::MemoryRange;
use crate::memory_address::MemoryAddress;
use crate::allocators::allocator::Allocator;

/// A local allocator is an allocator with a known range of memory addresses it uses for allocated memory.
///
/// This allows logic to determine which allocator should be used to free (deallocate) which memory pointers.
pub trait LocalAllocator: Allocator
{
	/// The range of memory addresses that can be used to allocate memory by this allocator.
	///
	/// This function is called repeatedly, so ideally should be inline and fast.
	fn memory_range(&self) -> MemoryRange;

	/// Returns `true` if this allocator is responsible for an allocation starting with the given `from_memory_address`.
	///
	/// This function is called repeatedly, so ideally should be inline and fast.
	#[inline(always)]
	fn contains(&self, from_memory_address: MemoryAddress) -> bool
	{
		self.memory_range().contains(from_memory_address)
	}
}
