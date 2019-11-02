use super::memory_sources::memory_source::MemorySource;
use super::allocators::prelude::*;
use super::memory_address::MemoryAddress;
use std::alloc::AllocErr;
use super::allocators::global::local_allocator::LocalAllocator;
use super::allocators::global::memory_range::MemoryRange;
use super::allocators::prelude::bit_set_allocator::BitSetAllocator;
use std::num::NonZeroUsize;

use std::fmt::Debug;

/// An allocator designed for contexts with different lifetimes.
///
/// This allocator NEVER grows or shrinks its memory region.
///
/// This allocator is not thread-safe.
#[derive(Debug)]
pub enum ContextAllocator<MS: MemorySource>
{
	/// Use this variant for contexts with short-lived lifetimes.
	///
	/// Very fast allocation and almost costless deallocation, at the expense of the strong likelihood of running out of memory.
	///
	/// Reallocation is very expensive when growing unless reallocating the most recently made allocation.
	ShortLived(BumpAllocator<MS>),

	/// Use this variant for contexts with slightly longer than short-lived lifetimes.
	///
	/// Slower allocation and deallocation but reallocation is less expensive than for `ShortLived`.
	MediumLived(BitSetAllocator<MS>),

	/// Use this variant for contexts with long-lived lifetimes.
	LongLived(MultipleBinarySearchTreeAllocator<MS>),
}

impl<MS: MemorySource> Allocator for ContextAllocator<MS>
{
	#[inline(always)]
	fn allocate(&self, non_zero_size: NonZeroUsize, non_zero_power_of_two_alignment: NonZeroUsize) -> Result<MemoryAddress, AllocErr>
	{
		use self::ContextAllocator::*;

		match *self
		{
			ShortLived(ref allocator) => allocator.allocate(non_zero_size, non_zero_power_of_two_alignment),

			MediumLived(ref allocator) => allocator.allocate(non_zero_size, non_zero_power_of_two_alignment),

			LongLived(ref allocator) => allocator.allocate(non_zero_size, non_zero_power_of_two_alignment),
		}
	}

	#[inline(always)]
	fn deallocate(&self, non_zero_size: NonZeroUsize, non_zero_power_of_two_alignment: NonZeroUsize, current_memory: MemoryAddress)
	{
		use self::ContextAllocator::*;

		match *self
		{
			ShortLived(ref allocator) => allocator.deallocate(non_zero_size, non_zero_power_of_two_alignment, current_memory),

			MediumLived(ref allocator) => allocator.deallocate(non_zero_size, non_zero_power_of_two_alignment, current_memory),

			LongLived(ref allocator) => allocator.deallocate(non_zero_size, non_zero_power_of_two_alignment, current_memory),
		}
	}

	#[inline(always)]
	fn growing_reallocate(&self, non_zero_new_size: NonZeroUsize, non_zero_power_of_two_alignment: NonZeroUsize, non_zero_current_size: NonZeroUsize, current_memory: MemoryAddress) -> Result<MemoryAddress, AllocErr>
	{
		use self::ContextAllocator::*;

		match *self
		{
			ShortLived(ref allocator) => allocator.growing_reallocate(non_zero_new_size, non_zero_power_of_two_alignment, non_zero_current_size, current_memory),

			MediumLived(ref allocator) => allocator.growing_reallocate(non_zero_new_size, non_zero_power_of_two_alignment, non_zero_current_size, current_memory),

			LongLived(ref allocator) => allocator.growing_reallocate(non_zero_new_size, non_zero_power_of_two_alignment, non_zero_current_size, current_memory),
		}
	}

	#[inline(always)]
	fn shrinking_reallocate(&self, non_zero_new_size: NonZeroUsize, non_zero_power_of_two_alignment: NonZeroUsize, non_zero_current_size: NonZeroUsize, current_memory: MemoryAddress) -> Result<MemoryAddress, AllocErr>
	{
		use self::ContextAllocator::*;

		match *self
		{
			ShortLived(ref allocator) => allocator.shrinking_reallocate(non_zero_new_size, non_zero_power_of_two_alignment, non_zero_current_size, current_memory),

			MediumLived(ref allocator) => allocator.shrinking_reallocate(non_zero_new_size, non_zero_power_of_two_alignment, non_zero_current_size, current_memory),

			LongLived(ref allocator) => allocator.shrinking_reallocate(non_zero_new_size, non_zero_power_of_two_alignment, non_zero_current_size, current_memory),
		}
	}
}

impl<MS: MemorySource> LocalAllocator for ContextAllocator<MS>
{
	#[inline(always)]
	fn memory_range(&self) -> MemoryRange
	{
		use self::ContextAllocator::*;

		match *self
		{
			ShortLived(ref allocator) => allocator.memory_range(),

			MediumLived(ref allocator) => allocator.memory_range(),

			LongLived(ref allocator) => allocator.memory_range(),
		}
	}
}
