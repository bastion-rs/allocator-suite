use super::extensions::prelude::*;
use crate::allocators::allocator::Allocator;
use crate::memory_address::MemoryAddress;
use std::alloc::{Alloc, AllocErr, Layout};
use std::cell::UnsafeCell;
use std::fmt;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::num::NonZeroUsize;
use std::ops::Deref;

/// Adapts implementations of `Alloc` to `Allocator`.
pub struct AllocToAllocatorAdaptor<A: Alloc>(UnsafeCell<A>);

impl<A: Alloc> Debug for AllocToAllocatorAdaptor<A> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "AllocToAllocatorAdaptor")
    }
}

impl<A: Alloc> Deref for AllocToAllocatorAdaptor<A> {
    type Target = A;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        self.0.get().reference()
    }
}

impl<A: Alloc> Allocator for AllocToAllocatorAdaptor<A> {
    #[inline(always)]
    fn allocate(
        &self,
        non_zero_size: NonZeroUsize,
        non_zero_power_of_two_alignment: NonZeroUsize,
    ) -> Result<MemoryAddress, AllocErr> {
        unsafe {
            self.mutable_reference()
                .alloc(Self::layout(non_zero_size, non_zero_power_of_two_alignment))
        }
    }

    #[inline(always)]
    fn deallocate(
        &self,
        non_zero_size: NonZeroUsize,
        non_zero_power_of_two_alignment: NonZeroUsize,
        current_memory: MemoryAddress,
    ) {
        unsafe {
            self.mutable_reference().dealloc(
                current_memory,
                Self::layout(non_zero_size, non_zero_power_of_two_alignment),
            )
        }
    }

    #[inline(always)]
    fn growing_reallocate(
        &self,
        non_zero_new_size: NonZeroUsize,
        non_zero_power_of_two_alignment: NonZeroUsize,
        non_zero_current_size: NonZeroUsize,
        current_memory: MemoryAddress,
    ) -> Result<MemoryAddress, AllocErr> {
        unsafe {
            self.mutable_reference().realloc(
                current_memory,
                Self::layout(non_zero_current_size, non_zero_power_of_two_alignment),
                non_zero_new_size.get(),
            )
        }
    }

    #[inline(always)]
    fn shrinking_reallocate(
        &self,
        non_zero_new_size: NonZeroUsize,
        non_zero_power_of_two_alignment: NonZeroUsize,
        non_zero_current_size: NonZeroUsize,
        current_memory: MemoryAddress,
    ) -> Result<MemoryAddress, AllocErr> {
        unsafe {
            self.mutable_reference().realloc(
                current_memory,
                Self::layout(non_zero_current_size, non_zero_power_of_two_alignment),
                non_zero_new_size.get(),
            )
        }
    }
}

impl<A: Alloc> AllocToAllocatorAdaptor<A> {
    #[inline(always)]
    fn mutable_reference(&self) -> &mut A {
        self.0.get().mutable_reference()
    }

    #[inline(always)]
    fn layout(
        non_zero_size: NonZeroUsize,
        non_zero_power_of_two_alignment: NonZeroUsize,
    ) -> Layout {
        unsafe {
            Layout::from_size_align_unchecked(
                non_zero_size.get(),
                non_zero_power_of_two_alignment.get(),
            )
        }
    }
}
