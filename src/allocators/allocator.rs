use super::memory_address::MemoryAddress;
use super::adaptors::prelude::*;
use std::intrinsics::transmute;
use std::alloc::{Layout, AllocErr, Excess, CannotReallocInPlace};
use super::extensions::prelude::*;
use std::ptr::{null_mut, NonNull};
use std::num::NonZeroUsize;
use std::num::NonZeroU32;
use std::fmt::Debug;


/// A helper trait that brings together the core, common functionality required to implement the traits `GlobalAlloc` and `Alloc`.
pub trait Allocator: Debug + Sized
{
	/// The sentinel value used for a zero-sized allocation.
	const ZeroSizedAllocation: MemoryAddress = non_null_pointer(::std::usize::MAX as *mut u8);

	/// Allocate memory.
	fn allocate(&self, non_zero_size: NonZeroUsize, non_zero_power_of_two_alignment: NonZeroUsize) -> Result<MemoryAddress, AllocErr>;

	/// Deallocate (free) memory.
	///
	/// The parameter `memory` will never be the value `Self::ZeroSizedAllocation` and will always have been allocated by this `Allocator`.
	fn deallocate(&self, non_zero_size: NonZeroUsize, non_zero_power_of_two_alignment: NonZeroUsize, current_memory: MemoryAddress);

	/// Reallocate memory by growing it.
	///
	/// `non_zero_new_size` will always be greater than `non_zero_current_size`.
	/// `non_zero_power_of_two_alignment` will be the same value as passed to `allocate()`.
	fn growing_reallocate(&self, non_zero_new_size: NonZeroUsize, non_zero_power_of_two_alignment: NonZeroUsize, non_zero_current_size: NonZeroUsize, current_memory: MemoryAddress) -> Result<MemoryAddress, AllocErr>;

	/// Reallocate memory by shrinking it.
	///
	/// `non_zero_new_size` will always be less than `non_zero_current_size`.
	/// `non_zero_power_of_two_alignment` will be the same value as passed to `allocate()`.
	fn shrinking_reallocate(&self, non_zero_new_size: NonZeroUsize, non_zero_power_of_two_alignment: NonZeroUsize, non_zero_current_size: NonZeroUsize, current_memory: MemoryAddress) -> Result<MemoryAddress, AllocErr>;

	/// Adapts to a `GlobalAlloc` and `Alloc`.
	#[inline(always)]
	fn adapt<'a>(&'a self) -> AllocatorAdaptor<'a, Self>
	{
		AllocatorAdaptor(self)
	}

	/// Adapts a reference to a `GlobalAlloc` and `Alloc` reference.
	#[inline(always)]
	fn adapt_reference<'a>(&'a self) -> &'a AllocatorAdaptor<'a, Self>
	{
		unsafe { transmute(self) }
	}

	#[doc(hidden)]
	#[inline(always)]
	fn allocate_zeroed(&self, layout: Layout) -> Result<MemoryAddress, AllocErr>
	{
		let layout = LayoutHack::access_private_fields(layout);

		let zero_size = layout.size_;

		if unlikely!(zero_size == 0)
		{
			return Ok(Self::ZeroSizedAllocation)
		}

		let non_zero_size = layout.size_.non_zero();
		let result = self.allocate(non_zero_size, layout.align_);

		// NOTE: AllocErr does not implement `Copy`, but is zero-sized - seems like a Rust API oversight.
		// Hence the logic transmuting it to a pointer (for an efficient null check), then back to a result.
		let pointer = unsafe { transmute::<_, *mut u8>(result) };

		if likely!(!pointer.is_null())
		{
			unsafe { pointer.write_bytes(0x00, zero_size) };
		}

		unsafe { transmute(pointer) }
	}

	#[doc(hidden)]
	#[inline(always)]
	fn reallocate(&self, current_memory: MemoryAddress, layout: Layout, new_size: usize) -> Result<MemoryAddress, AllocErr>
	{
		let layout = LayoutHack::access_private_fields(layout);

		let current_size = layout.size_;

		if unlikely!(current_size == new_size)
		{
			return Ok(current_memory)
		}

		let non_zero_power_of_two_alignment = layout.align_;

		if likely!(new_size > current_size)
		{
			let non_zero_new_size = new_size.non_zero();

			if unlikely!(current_size == 0)
			{
				return self.allocate(non_zero_new_size, non_zero_power_of_two_alignment)
			}

			let non_zero_current_size = current_size.non_zero();
			self.growing_reallocate(non_zero_new_size, non_zero_power_of_two_alignment, non_zero_current_size, current_memory)
		}
		else
		{
			let non_zero_current_size = current_size.non_zero();

			if unlikely!(new_size == 0)
			{
				self.deallocate(non_zero_current_size, non_zero_power_of_two_alignment, current_memory);
				return Ok(Self::ZeroSizedAllocation)
			}

			let non_zero_new_size = new_size.non_zero();
			self.shrinking_reallocate(non_zero_new_size, non_zero_power_of_two_alignment, non_zero_current_size, current_memory)
		}
	}

	#[doc(hidden)]
	#[inline(always)]
	unsafe fn GlobalAlloc_alloc(&self, layout: Layout) -> *mut u8
	{
		let layout = LayoutHack::access_private_fields(layout);

		let zero_size = layout.size_;

		if unlikely!(zero_size == 0)
		{
			return Self::ZeroSizedAllocation.as_ptr()
		}

		let non_zero_size = NonZeroUsize::new_unchecked(zero_size);
		transmute(self.allocate(non_zero_size, layout.align_))
	}

	#[doc(hidden)]
	#[inline(always)]
	unsafe fn GlobalAlloc_alloc_zeroed(&self, layout: Layout) -> *mut u8
	{
		transmute(self.allocate_zeroed(layout))
	}

	#[doc(hidden)]
	#[inline(always)]
	unsafe fn GlobalAlloc_dealloc(&self, ptr: *mut u8, layout: Layout)
	{
		debug_assert_ne!(ptr, null_mut(), "ptr should never be null");

		if unlikely!(ptr == Self::ZeroSizedAllocation.as_ptr())
		{
			return
		}

		let layout = LayoutHack::access_private_fields(layout);

		let zero_size = layout.size_;
		debug_assert_ne!(zero_size, 0, "It should not be possible for a `layout.size_` to be zero if the `ptr` was the sentinel `Allocator::ZeroSizedAllocation`");
		let non_zero_size = NonZeroUsize::new_unchecked(zero_size);

		let current_memory = NonNull::new_unchecked(ptr);

		self.deallocate(non_zero_size,layout.align_, current_memory)
	}

	#[doc(hidden)]
	#[inline(always)]
	unsafe fn GlobalAlloc_realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8
	{
		debug_assert_ne!(ptr, null_mut(), "ptr should never be null");

		transmute(self.reallocate(NonNull::new_unchecked(ptr), layout, new_size))
	}

	#[doc(hidden)]
	#[inline(always)]
	unsafe fn Alloc_alloc(&self, layout: Layout) -> Result<MemoryAddress, AllocErr>
	{
		let layout = LayoutHack::access_private_fields(layout);
		if unlikely!(layout.size_ == 0)
		{
			return Ok(Self::ZeroSizedAllocation)
		}
		let non_zero_size = NonZeroUsize::new_unchecked(layout.size_);
		self.allocate(non_zero_size, layout.align_)
	}

	#[doc(hidden)]
	#[inline(always)]
	unsafe fn Alloc_alloc_zeroed(&self, layout: Layout) -> Result<MemoryAddress, AllocErr>
	{
		self.allocate_zeroed(layout)
	}

	#[doc(hidden)]
	#[inline(always)]
	unsafe fn Alloc_dealloc(&self, ptr: MemoryAddress, layout: Layout)
	{
		if unlikely!(ptr == Self::ZeroSizedAllocation)
		{
			return
		}

		let layout = LayoutHack::access_private_fields(layout);
		debug_assert_ne!(layout.size_, 0, "It should not be possible for a `layout.size_` to be zero if the `ptr` was the sentinel `Allocator::ZeroSizedAllocation`");

		let non_zero_size = NonZeroUsize::new_unchecked(layout.size_);
		self.deallocate(non_zero_size, layout.align_, ptr)
	}

	#[doc(hidden)]
	#[inline(always)]
	unsafe fn Alloc_realloc(&self, ptr: MemoryAddress, layout: Layout, new_size: usize) -> Result<MemoryAddress, AllocErr>
	{
		self.reallocate(ptr, layout, new_size)
	}

	#[doc(hidden)]
	#[inline(always)]
	unsafe fn Alloc_alloc_excess(&self, layout: Layout) -> Result<Excess, AllocErr>
	{
		let layout = LayoutHack::access_private_fields(layout);
		if unlikely!(layout.size_ == 0)
		{
			return Ok(Excess(Self::ZeroSizedAllocation, 0))
		}
		let size = layout.size_;
		let non_zero_size = NonZeroUsize::new_unchecked(size);

		let result = self.allocate(non_zero_size, layout.align_);

		// NOTE: AllocErr does not implement `Copy`, but is zero-sized - seems like a Rust API oversight.
		// Hence the logic transmuting it to a pointer (for an efficient null check), then back to a result.
		let pointer: *mut u8 = transmute(result);
		if unlikely!(pointer.is_null())
		{
			Err(AllocErr)
		}
		else
		{
			Ok(Excess(NonNull::new_unchecked(pointer), size))
		}
	}

	#[doc(hidden)]
	#[inline(always)]
	unsafe fn Alloc_realloc_excess(&self, ptr: MemoryAddress, layout: Layout, new_size: usize) -> Result<Excess, AllocErr>
	{
		let result = self.reallocate(ptr, layout, new_size);

		// NOTE: AllocErr does not implement `Copy`, but is zero-sized - seems like a Rust API oversight.
		// Hence the logic transmuting it to a pointer (for an efficient null check), then back to a result.
		let pointer: *mut u8 = transmute(result);
		if unlikely!(pointer.is_null())
		{
			Err(AllocErr)
		}
		else
		{
			Ok(Excess(NonNull::new_unchecked(pointer), new_size))
		}
	}

	#[doc(hidden)]
	#[inline(always)]
	unsafe fn Alloc_grow_in_place(&self, _ptr: MemoryAddress, layout: Layout, new_size: usize) -> Result<(), CannotReallocInPlace>
	{
		let layout = LayoutHack::access_private_fields(layout);
		let size_ = layout.size_;
		debug_assert!(new_size >= size_, "new_size `{}` is less than layout.size_ `{}`", new_size, size_);
		Err(CannotReallocInPlace)
	}

	#[doc(hidden)]
	#[inline(always)]
	unsafe fn Alloc_shrink_in_place(&self, _ptr: MemoryAddress, layout: Layout, new_size: usize) -> Result<(), CannotReallocInPlace>
	{
		let layout = LayoutHack::access_private_fields(layout);
		let size_ = layout.size_;
		debug_assert!(new_size <= size_, "layout.size_ `{}` is less than new_size `{}`", size_, new_size);
		Err(CannotReallocInPlace)
	}
}
