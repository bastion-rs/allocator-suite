use crate::adaptors::allocator_adaptor::AllocatorAdaptor;

use crate::extensions::non_null_pointer::non_null_pointer;
use crate::extensions::prelude::*;
use crate::extensions::usize_ext::UsizeExt;
use crate::memory_address::MemoryAddress;
use std::alloc::{AllocError, Layout};
use std::fmt::Debug;
use std::intrinsics::transmute;
use std::num::NonZeroUsize;
use std::ptr::{null_mut, NonNull};

/// A helper trait that brings together the core, common functionality required to implement the traits `GlobalAlloc` and `Alloc`.
pub trait Allocator: Debug + Sized {
    /// The sentinel value used for a zero-sized allocation.
    const ZERO_SIZED_ALLOCATION: MemoryAddress = non_null_pointer(::std::usize::MAX as *mut u8);

    /// Allocate memory.
    fn allocate(
        &self,
        non_zero_size: NonZeroUsize,
        non_zero_power_of_two_alignment: NonZeroUsize,
    ) -> Result<MemoryAddress, AllocError>;

    /// Deallocate (free) memory.
    ///
    /// The parameter `memory` will never be the value `Self::ZERO_SIZED_ALLOCATION` and will always have been allocated by this `Allocator`.
    fn deallocate(
        &self,
        non_zero_size: NonZeroUsize,
        non_zero_power_of_two_alignment: NonZeroUsize,
        current_memory: MemoryAddress,
    );

    /// Reallocate memory by growing it.
    ///
    /// `non_zero_new_size` will always be greater than `non_zero_current_size`.
    /// `non_zero_power_of_two_alignment` will be the same value as passed to `allocate()`.
    fn growing_reallocate(
        &self,
        non_zero_new_size: NonZeroUsize,
        non_zero_power_of_two_alignment: NonZeroUsize,
        non_zero_current_size: NonZeroUsize,
        current_memory: MemoryAddress,
    ) -> Result<MemoryAddress, AllocError>;

    /// Reallocate memory by shrinking it.
    ///
    /// `non_zero_new_size` will always be less than `non_zero_current_size`.
    /// `non_zero_power_of_two_alignment` will be the same value as passed to `allocate()`.
    fn shrinking_reallocate(
        &self,
        non_zero_new_size: NonZeroUsize,
        non_zero_power_of_two_alignment: NonZeroUsize,
        non_zero_current_size: NonZeroUsize,
        current_memory: MemoryAddress,
    ) -> Result<MemoryAddress, AllocError>;

    /// Adapts to a `GlobalAlloc` and `Alloc`.
    #[inline(always)]
    fn adapt<'a>(&'a self) -> AllocatorAdaptor<'a, Self> {
        AllocatorAdaptor(self)
    }

    /// Adapts a reference to a `GlobalAlloc` and `Alloc` reference.
    #[inline(always)]
    fn adapt_reference<'a>(&'a self) -> &'a AllocatorAdaptor<'a, Self> {
        unsafe { transmute(self) }
    }

    #[doc(hidden)]
    #[inline(always)]
    fn allocate_zeroed(&self, layout: Layout) -> Result<MemoryAddress, AllocError> {
        let maybe_zero_size = layout.size();

        if unlikely!(maybe_zero_size == 0) {
            return Ok(Self::ZERO_SIZED_ALLOCATION);
        }

        let non_zero_size = maybe_zero_size.non_zero();
        let non_zero_align = layout.align().non_zero();
        let result = self.allocate(non_zero_size, non_zero_align);

        // NOTE: AllocError does not implement `Copy`, but is zero-sized - seems like a Rust API oversight.
        // Hence the logic transmuting it to a pointer (for an efficient null check), then back to a result.
        let pointer = unsafe { transmute::<_, *mut u8>(result) };

        if likely!(!pointer.is_null()) {
            unsafe { pointer.write_bytes(0x00, maybe_zero_size) };
        }

        unsafe { transmute(pointer) }
    }

    #[doc(hidden)]
    #[inline(always)]
    fn reallocate(
        &self,
        current_memory: MemoryAddress,
        layout: Layout,
        new_size: usize,
    ) -> Result<MemoryAddress, AllocError> {
        let current_size = layout.size();

        if unlikely!(current_size == new_size) {
            return Ok(current_memory);
        }

        let non_zero_power_of_two_alignment = layout.align().non_zero();

        if likely!(new_size > current_size) {
            let non_zero_new_size = new_size.non_zero();

            if unlikely!(current_size == 0) {
                return self.allocate(non_zero_new_size, non_zero_power_of_two_alignment);
            }

            let non_zero_current_size = current_size.non_zero();
            self.growing_reallocate(
                non_zero_new_size,
                non_zero_power_of_two_alignment,
                non_zero_current_size,
                current_memory,
            )
        } else {
            let non_zero_current_size = current_size.non_zero();

            if unlikely!(new_size == 0) {
                self.deallocate(
                    non_zero_current_size,
                    non_zero_power_of_two_alignment,
                    current_memory,
                );
                return Ok(Self::ZERO_SIZED_ALLOCATION);
            }

            let non_zero_new_size = new_size.non_zero();
            self.shrinking_reallocate(
                non_zero_new_size,
                non_zero_power_of_two_alignment,
                non_zero_current_size,
                current_memory,
            )
        }
    }

    #[doc(hidden)]
    #[inline(always)]
    unsafe fn global_alloc_alloc(&self, layout: Layout) -> *mut u8 {
        let maybe_zero_size = layout.size();

        if unlikely!(maybe_zero_size == 0) {
            return Self::ZERO_SIZED_ALLOCATION.as_ptr();
        }

        let non_zero_usize = NonZeroUsize::new_unchecked(maybe_zero_size);
        let non_zero_align = layout.align().non_zero();
        transmute(self.allocate(non_zero_usize, non_zero_align))
    }

    #[doc(hidden)]
    #[inline(always)]
    unsafe fn global_alloc_alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        transmute(self.allocate_zeroed(layout))
    }

    #[doc(hidden)]
    #[inline(always)]
    unsafe fn global_alloc_dealloc(&self, ptr: *mut u8, layout: Layout) {
        debug_assert_ne!(ptr, null_mut(), "ptr should never be null");

        if unlikely!(ptr == Self::ZERO_SIZED_ALLOCATION.as_ptr()) {
            return;
        }

        let maybe_zero_size = layout.size();
        debug_assert_ne!(maybe_zero_size, 0, "It should not be possible for a `layout.size(` to be zero if the `ptr` was the sentinel `Allocator::ZERO_SIZED_ALLOCATION`");
        let non_zero_usize = NonZeroUsize::new_unchecked(maybe_zero_size);

        let current_memory = NonNull::new_unchecked(ptr);

        self.deallocate(non_zero_usize, layout.align().non_zero(), current_memory)
    }

    #[doc(hidden)]
    #[inline(always)]
    unsafe fn global_alloc_realloc(
        &self,
        ptr: *mut u8,
        layout: Layout,
        new_size: usize,
    ) -> *mut u8 {
        debug_assert_ne!(ptr, null_mut(), "ptr should never be null");

        transmute(self.reallocate(NonNull::new_unchecked(ptr), layout, new_size))
    }

    #[doc(hidden)]
    #[inline(always)]
    unsafe fn alloc_alloc(&self, layout: Layout) -> Result<MemoryAddress, AllocError> {
        if unlikely!(layout.size() == 0) {
            return Ok(Self::ZERO_SIZED_ALLOCATION);
        }
        let non_zero_usize = NonZeroUsize::new_unchecked(layout.size());
        let non_zero_align = NonZeroUsize::new_unchecked(layout.align());
        self.allocate(non_zero_usize, non_zero_align)
    }

    #[doc(hidden)]
    #[inline(always)]
    unsafe fn alloc_alloc_zeroed(&self, layout: Layout) -> Result<MemoryAddress, AllocError> {
        self.allocate_zeroed(layout)
    }

    #[doc(hidden)]
    #[inline(always)]
    unsafe fn alloc_dealloc(&self, ptr: MemoryAddress, layout: Layout) {
        if unlikely!(ptr == Self::ZERO_SIZED_ALLOCATION) {
            return;
        }

        debug_assert_ne!(layout.size(), 0, "It should not be possible for a `layout.size()` to be zero if the `ptr` was the sentinel `Allocator::ZERO_SIZED_ALLOCATION`");
        debug_assert_ne!(layout.align(), 0, "It should not be possible for a `layout.align()` to be zero if the `ptr` was the sentinel `Allocator::ZERO_SIZED_ALLOCATION`");

        let non_zero_usize = NonZeroUsize::new_unchecked(layout.size());
        let non_zero_align = NonZeroUsize::new_unchecked(layout.align());
        self.deallocate(non_zero_usize, non_zero_align, ptr)
    }

    #[doc(hidden)]
    #[inline(always)]
    unsafe fn alloc_realloc(
        &self,
        ptr: MemoryAddress,
        layout: Layout,
        new_size: usize,
    ) -> Result<MemoryAddress, AllocError> {
        self.reallocate(ptr, layout, new_size)
    }
}
