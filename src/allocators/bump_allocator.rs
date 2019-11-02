use crate::allocators::allocator::Allocator;
use crate::allocators::global::local_allocator::LocalAllocator;
use crate::allocators::global::memory_range::MemoryRange;
use crate::allocators::global::prelude::*;
use crate::allocators::prelude::*;
use crate::extensions::non_zero_usize::non_zero_usize;
use crate::extensions::prelude::*;
use crate::memory_address::MemoryAddress;
use crate::memory_sources::memory_source::MemorySource;
use std::alloc::AllocErr;
use std::cell::Cell;
use std::fmt::Debug;
use std::intrinsics::transmute;
use std::num::NonZeroUsize;

/// This is a very simple bump allocator of minimal utility.
///
/// It:-
///
/// * Can efficiently shrink and grow (reallocate) for the most recent allocation made (useful when pushing to a RawVec, say).
/// * Has no wrapping around at the end (but this could be achieved using a mirror ring buffer).
/// * Has no ability to resize in place if dead space occurs before next allocation because of alignment.
///
/// Is suitable for use with short-lived coroutines, such as those used to make a DNS query.
///
/// This allocator NEVER grows or shrinks its memory region.
///
/// This allocator is not thread-safe.
#[derive(Debug)]
pub struct BumpAllocator<MS: MemorySource> {
    most_recent_allocation_pointer: Cell<MemoryAddress>,
    next_allocation_at_pointer: Cell<MemoryAddress>,
    ends_at_pointer: MemoryAddress,

    memory_source: MS,
    memory_source_size: NonZeroUsize,
}

impl<MS: MemorySource> Drop for BumpAllocator<MS> {
    #[inline(always)]
    fn drop(&mut self) {
        self.memory_source
            .release(self.memory_source_size, self.allocations_start_from())
    }
}

macro_rules! allocation_ends_at_pointer
{
	($self: ident, $non_zero_size: ident, $allocation_from: ident) =>
	{
		{
			// NOTE: This evil code is used so that we can use an if hint of `unlikely!` rather than an unhinted `match` for `result`.
			let allocation_ends_at_pointer: MemoryAddress =
			{
				let size = $non_zero_size.get();
				let pointer: *mut u8 = unsafe { transmute($allocation_from.checked_add(size)) };
				if unlikely!(pointer.is_null())
				{
					return Err(AllocErr)
				}
				unsafe { transmute(pointer) }
			};

			if unlikely!(allocation_ends_at_pointer > $self.ends_at_pointer)
			{
				return Err(AllocErr)
			}

			allocation_ends_at_pointer
		}
	}
}

impl<MS: MemorySource> Allocator for BumpAllocator<MS> {
    #[inline(always)]
    fn allocate(
        &self,
        non_zero_size: NonZeroUsize,
        non_zero_power_of_two_alignment: NonZeroUsize,
    ) -> Result<MemoryAddress, AllocErr> {
        debug_assert!(
            non_zero_power_of_two_alignment <= Self::MAXIMUM_POWER_OF_TWO_ALIGNMENT,
            "non_zero_power_of_two_alignment `{}` exceeds `{}`",
            non_zero_power_of_two_alignment,
            Self::MAXIMUM_POWER_OF_TWO_ALIGNMENT
        );

        let next_allocation_at_rounded_up_pointer = self
            .next_allocation_at_pointer
            .get()
            .round_up_to_power_of_two(non_zero_power_of_two_alignment);

        self.most_recent_allocation_pointer
            .set(next_allocation_at_rounded_up_pointer);
        self.next_allocation_at_pointer
            .set(allocation_ends_at_pointer!(
                self,
                non_zero_size,
                next_allocation_at_rounded_up_pointer
            ));

        Ok(next_allocation_at_rounded_up_pointer)
    }

    #[inline(always)]
    fn deallocate(
        &self,
        _non_zero_size: NonZeroUsize,
        _non_zero_power_of_two_alignment: NonZeroUsize,
        current_memory: MemoryAddress,
    ) {
        if unlikely!(current_memory == self.most_recent_allocation_pointer.get()) {
            self.next_allocation_at_pointer
                .set(self.most_recent_allocation_pointer.get())
        }
    }

    #[inline(always)]
    fn shrinking_reallocate(
        &self,
        non_zero_new_size: NonZeroUsize,
        _non_zero_power_of_two_alignment: NonZeroUsize,
        _non_zero_current_size: NonZeroUsize,
        current_memory: MemoryAddress,
    ) -> Result<MemoryAddress, AllocErr> {
        if unlikely!(current_memory == self.most_recent_allocation_pointer.get()) {
            let size = non_zero_new_size.get();
            self.next_allocation_at_pointer
                .set(current_memory.add(size))
        }

        Ok(current_memory)
    }

    #[inline(always)]
    fn growing_reallocate(
        &self,
        non_zero_new_size: NonZeroUsize,
        non_zero_power_of_two_alignment: NonZeroUsize,
        non_zero_current_size: NonZeroUsize,
        current_memory: MemoryAddress,
    ) -> Result<MemoryAddress, AllocErr> {
        if unlikely!(current_memory == self.most_recent_allocation_pointer.get()) {
            self.next_allocation_at_pointer
                .set(allocation_ends_at_pointer!(
                    self,
                    non_zero_new_size,
                    current_memory
                ));
            Ok(current_memory)
        } else {
            let result = self.allocate(non_zero_new_size, non_zero_power_of_two_alignment);
            let pointer: *mut u8 = unsafe { transmute(result) };
            if unlikely!(pointer.is_null()) {
                Err(AllocErr)
            } else {
                let current_size = non_zero_current_size.get();
                unsafe { pointer.copy_from(current_memory.as_ptr(), current_size) };
                Ok(unsafe { transmute(pointer) })
            }
        }
    }
}

impl<MS: MemorySource> LocalAllocator for BumpAllocator<MS> {
    #[inline(always)]
    fn memory_range(&self) -> MemoryRange {
        MemoryRange::new(self.allocations_start_from(), self.ends_at_pointer)
    }
}

impl<MS: MemorySource> BumpAllocator<MS> {
    const MAXIMUM_POWER_OF_TWO_ALIGNMENT: NonZeroUsize = non_zero_usize(4096);

    /// New instance wrapping a block of memory.
    #[inline(always)]
    pub fn new(memory_source: MS, memory_source_size: NonZeroUsize) -> Result<Self, AllocErr> {
        let allocations_start_from = memory_source.obtain(memory_source_size)?;

        Ok(Self {
            most_recent_allocation_pointer: Cell::new(allocations_start_from),
            next_allocation_at_pointer: Cell::new(allocations_start_from),
            ends_at_pointer: allocations_start_from.add_non_zero(memory_source_size),

            memory_source,
            memory_source_size,
        })
    }

    #[inline(always)]
    fn allocations_start_from(&self) -> MemoryAddress {
        self.ends_at_pointer
            .subtract_non_zero(self.memory_source_size)
    }
}
