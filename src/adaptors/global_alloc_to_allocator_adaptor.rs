use crate::allocators::allocator::Allocator;
use crate::memory_address::MemoryAddress;
use std::alloc::{AllocErr, GlobalAlloc, Layout};
use std::fmt;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::intrinsics::transmute;
use std::num::NonZeroUsize;
use std::ops::Deref;

/// Adapts implementations of `GlobalAlloc` to `Allocator`.
pub struct GlobalAllocToAllocatorAdaptor<GA: GlobalAlloc>(pub GA);

impl<GA: GlobalAlloc> Debug for GlobalAllocToAllocatorAdaptor<GA> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "GlobalAllocToAllocatorAdaptor")
    }
}

impl<GA: GlobalAlloc> Deref for GlobalAllocToAllocatorAdaptor<GA> {
    type Target = GA;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<GA: GlobalAlloc> Allocator for GlobalAllocToAllocatorAdaptor<GA> {
    #[inline(always)]
    fn allocate(
        &self,
        non_zero_size: NonZeroUsize,
        non_zero_power_of_two_alignment: NonZeroUsize,
    ) -> Result<MemoryAddress, AllocErr> {
        unsafe {
            transmute(
                self.0
                    .alloc(Self::layout(non_zero_size, non_zero_power_of_two_alignment)),
            )
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
            self.0.dealloc(
                current_memory.as_ptr(),
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
            transmute(self.0.realloc(
                current_memory.as_ptr(),
                Self::layout(non_zero_current_size, non_zero_power_of_two_alignment),
                non_zero_new_size.get(),
            ))
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
            transmute(self.0.realloc(
                current_memory.as_ptr(),
                Self::layout(non_zero_current_size, non_zero_power_of_two_alignment),
                non_zero_new_size.get(),
            ))
        }
    }
}

impl<GA: GlobalAlloc> GlobalAllocToAllocatorAdaptor<GA> {
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
