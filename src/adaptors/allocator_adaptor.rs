use std::ops::Deref;
use std::alloc::{GlobalAlloc, Alloc, AllocErr, Layout, Excess, CannotReallocInPlace};
use crate::memory_address::MemoryAddress;

use crate::allocators::allocator::Allocator;

use std::num::NonZeroUsize;


/// Adapts an `Allocator` to the `GlobalAlloc` and `Alloc` traits.
#[repr(transparent)]
#[derive(Debug)]
pub struct AllocatorAdaptor<'a, A: 'a + Allocator>(pub(crate) &'a A);

impl<'a, A: 'a + Allocator> Deref for AllocatorAdaptor<'a, A>
{
	type Target = A;

	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		self.0
	}
}

unsafe impl<'a, A: 'a + Allocator> GlobalAlloc for AllocatorAdaptor<'a, A>
{
	#[inline(always)]
	unsafe fn alloc(&self, layout: Layout) -> *mut u8
	{
		self.GlobalAlloc_alloc(layout)
	}

	#[inline(always)]
	unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8
	{
		self.GlobalAlloc_alloc_zeroed(layout)
	}

	#[inline(always)]
	unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout)
	{
		self.GlobalAlloc_dealloc(ptr, layout)
	}

	#[inline(always)]
	unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8
	{
		self.GlobalAlloc_realloc(ptr, layout, new_size)
	}
}

unsafe impl<'a, A: 'a + Allocator> Alloc for AllocatorAdaptor<'a, A>
{
	#[inline(always)]
	unsafe fn alloc(&mut self, layout: Layout) -> Result<MemoryAddress, AllocErr>
	{
		self.Alloc_alloc(layout)
	}

	#[inline(always)]
	unsafe fn alloc_zeroed(&mut self, layout: Layout) -> Result<MemoryAddress, AllocErr>
	{
		self.Alloc_alloc_zeroed(layout)
	}

	#[inline(always)]
	unsafe fn dealloc(&mut self, ptr: MemoryAddress, layout: Layout)
	{
		self.Alloc_dealloc(ptr, layout)
	}

	#[inline(always)]
	unsafe fn realloc(&mut self, ptr: MemoryAddress, layout: Layout, new_size: usize) -> Result<MemoryAddress, AllocErr>
	{
		self.Alloc_realloc(ptr, layout, new_size)
	}

	#[inline(always)]
	unsafe fn alloc_excess(&mut self, layout: Layout) -> Result<Excess, AllocErr>
	{
		self.Alloc_alloc_excess(layout)
	}

	#[inline(always)]
	unsafe fn realloc_excess(&mut self, ptr: MemoryAddress, layout: Layout, new_size: usize) -> Result<Excess, AllocErr>
	{
		self.Alloc_realloc_excess(ptr, layout, new_size)
	}

	#[inline(always)]
	unsafe fn grow_in_place(&mut self, ptr: MemoryAddress, layout: Layout, new_size: usize) -> Result<(), CannotReallocInPlace>
	{
		self.Alloc_grow_in_place(ptr, layout, new_size)
	}

	#[inline(always)]
	unsafe fn shrink_in_place(&mut self, ptr: MemoryAddress, layout: Layout, new_size: usize) -> Result<(), CannotReallocInPlace>
	{
		self.Alloc_shrink_in_place(ptr, layout, new_size)
	}
}

impl<'a, A: 'a + Allocator> Allocator for AllocatorAdaptor<'a, A>
{
	#[inline(always)]
	fn allocate(&self, non_zero_size: NonZeroUsize, non_zero_power_of_two_alignment: NonZeroUsize) -> Result<MemoryAddress, AllocErr>
	{
		self.0.allocate(non_zero_size, non_zero_power_of_two_alignment)
	}

	#[inline(always)]
	fn deallocate(&self, non_zero_size: NonZeroUsize, non_zero_power_of_two_alignment: NonZeroUsize, current_memory: MemoryAddress)
	{
		self.0.deallocate(non_zero_size, non_zero_power_of_two_alignment, current_memory)
	}

	#[inline(always)]
	fn growing_reallocate(&self, non_zero_new_size: NonZeroUsize, non_zero_power_of_two_alignment: NonZeroUsize, non_zero_current_size: NonZeroUsize, current_memory: MemoryAddress) -> Result<MemoryAddress, AllocErr>
	{
		self.0.growing_reallocate(non_zero_new_size, non_zero_power_of_two_alignment, non_zero_current_size, current_memory)
	}

	#[inline(always)]
	fn shrinking_reallocate(&self, non_zero_new_size: NonZeroUsize, non_zero_power_of_two_alignment: NonZeroUsize, non_zero_current_size: NonZeroUsize, current_memory: MemoryAddress) -> Result<MemoryAddress, AllocErr>
	{
		self.0.shrinking_reallocate(non_zero_new_size, non_zero_power_of_two_alignment, non_zero_current_size, current_memory)
	}
}
