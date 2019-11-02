use crate::extensions::prelude::*;
use std::mem::size_of;
use std::ptr::NonNull;
use crate::memory_address::MemoryAddress;
use std::num::NonZeroUsize;
use std::num::NonZeroU32;
use std::fmt::Debug;

/// Extensions to make working with NonNull<u8> easier.
pub trait NonNullU8Ext: Sized + Copy + Ord + Debug
{
	/// Round up to power of two.
	#[inline(always)]
	fn round_up_to_power_of_two(self, non_zero_power_of_two_alignment: NonZeroUsize) -> Self
	{
		Self::from_usize(self.to_usize().non_zero().round_up_to_power_of_two(non_zero_power_of_two_alignment).to_usize())
	}

	/// Add.
	#[inline(always)]
	fn add(self, increment: usize) -> Self
	{
		Self::from_usize(self.to_usize() + increment)
	}

	/// Add.
	#[inline(always)]
	fn add_non_zero(self, increment: NonZeroUsize) -> Self
	{
		Self::from_usize(self.to_usize() + increment.get())
	}

	/// Add.
	#[inline(always)]
	fn checked_add(self, increment: usize) -> Option<Self>
	{
		self.to_usize().checked_add(increment).map(Self::from_usize)
	}

	/// Add.
	#[inline(always)]
	fn add_assign(&mut self, increment: usize)
	{
		*self = (*self).add(increment)
	}

	/// Add.
	#[inline(always)]
	fn add_assign_non_zero(&mut self, increment: NonZeroUsize)
	{
		self.add_assign(increment.get())
	}

	/// Subtract.
	#[inline(always)]
	fn subtract(self, decrement: usize) -> Self
	{
		let usize = self.to_usize();
		debug_assert!(usize >= decrement, "decrement is too large");

		Self::from_usize(usize - decrement)
	}

	/// Subtract.
	#[inline(always)]
	fn subtract_non_zero(self, decrement: NonZeroUsize) -> Self
	{
		self.subtract(decrement.get())
	}

	/// Difference.
	#[inline(always)]
	fn difference(self, other: Self) -> usize
	{
		debug_assert!(self >= other, "other `{:?}` is less than self `{:?}`", other, self);

		self.to_usize() - other.to_usize()
	}

	/// Difference.
	#[inline(always)]
	fn difference_u32(self, other: Self) -> u32
	{
		let difference_usize = self.difference(other);
		debug_assert!(difference_usize <= ::std::u32::MAX as usize, "difference `{}` exceeds ::std::u32::MAX `{}`", difference_usize, ::std::u32::MAX);
		difference_usize as u32
	}

	/// Difference.
	#[inline(always)]
	fn difference_u32_non_zero(self, other: Self) -> NonZeroU32
	{
		NonZeroU32::non_zero(self.difference_u32(other))
	}

	/// Read.
	#[inline(always)]
	fn read<V: Copy>(self) -> V
	{
		unsafe { (self.to_pointer() as *const V).read() }
	}

	/// Read.
	#[inline(always)]
	fn read_u64(self) -> u64
	{
		self.read::<u64>()
	}

	/// Write.
	#[inline(always)]
	fn write<V: Copy>(self, value: V)
	{
		unsafe { (self.to_pointer() as *mut V).write(value) }
	}

	/// Write and advance.
	#[inline(always)]
	fn write_and_advance<V: Copy>(&mut self, value: V)
	{
		self.write(value);
		self.add_assign(size_of::<V>())
	}

	#[doc(hidden)]
	#[inline(always)]
	fn or_u64(self, bits_to_set: u64)
	{
		let current_value = self.read_u64();
		self.write::<u64>(current_value | bits_to_set)
	}

	#[doc(hidden)]
	#[inline(always)]
	fn and_u64(self, bits_to_preserve: u64)
	{
		let current_value = self.read_u64();
		self.write::<u64>(current_value & bits_to_preserve)
	}

	#[doc(hidden)]
	const BitsInAByte: usize = 8;

	#[doc(hidden)]
	const BitsInAnU64: usize = size_of::<u64>() * Self::BitsInAByte;

	#[doc(hidden)]
	#[inline(always)]
	fn set_bottom_bits_of_u64(self, number_of_bits_to_set: usize)
	{
		self.set_middle_bits_of_u64(number_of_bits_to_set, number_of_bits_to_set)
	}

	#[doc(hidden)]
	#[inline(always)]
	fn set_middle_bits_of_u64(self, number_of_bits_to_set: usize, number_of_lower_bits: usize)
	{
		debug_assert!(number_of_bits_to_set <= Self::BitsInAnU64);
		debug_assert!(number_of_lower_bits <= Self::BitsInAnU64);
		debug_assert!(number_of_bits_to_set <= number_of_lower_bits, "number_of_lower_bits `{}` is greater than number_of_bits_to_set `{}`", number_of_lower_bits, number_of_bits_to_set);

		let number_of_bits_to_set = number_of_bits_to_set as u64;
		let number_of_lower_bits = number_of_lower_bits as u64;

		self.or_u64(((1 << number_of_bits_to_set) - 1) << (number_of_lower_bits - number_of_bits_to_set));
	}

	#[doc(hidden)]
	#[inline(always)]
	fn set_top_bits_of_u64(self, number_of_bits_to_set: usize)
	{
		self.set_middle_bits_of_u64(number_of_bits_to_set, Self::BitsInAnU64 as usize)
	}

	#[doc(hidden)]
	#[inline(always)]
	fn unset_bottom_bits_of_u64(self, number_of_bits_to_unset: usize)
	{
		self.unset_middle_bits_of_u64(number_of_bits_to_unset, number_of_bits_to_unset)
	}

	#[doc(hidden)]
	#[inline(always)]
	fn unset_middle_bits_of_u64(self, number_of_bits_to_unset: usize, number_of_lower_bits: usize)
	{
		debug_assert!(number_of_bits_to_unset <= Self::BitsInAnU64);
		debug_assert!(number_of_lower_bits <= Self::BitsInAnU64);
		debug_assert!(number_of_bits_to_unset <= number_of_lower_bits, "number_of_lower_bits `{}` is greater than number_of_bits_to_unset `{}`", number_of_lower_bits, number_of_bits_to_unset);

		let number_of_bits_to_unset = number_of_bits_to_unset as u64;

		let number_of_lower_bits = number_of_lower_bits as u64;

		let bits_to_preserve = !((1 << number_of_bits_to_unset - 1) << (number_of_lower_bits - number_of_bits_to_unset));
		self.and_u64(bits_to_preserve);
	}

	#[doc(hidden)]
	#[inline(always)]
	fn unset_top_bits_of_u64(self, number_of_bits_to_unset: usize)
	{
		self.unset_middle_bits_of_u64(number_of_bits_to_unset, Self::BitsInAnU64 as usize)
	}

	/// Is aligned to.
	#[inline(always)]
	fn is_aligned_to(self, non_zero_power_of_two_alignment: NonZeroUsize) -> bool
	{
		let value = self.to_usize();
		let bitmask = non_zero_power_of_two_alignment.get() - 1;

		value & bitmask == 0
	}

	#[doc(hidden)]
	#[inline(always)]
	fn to_pointer(self) -> *mut u8
	{
		self.to_non_null_u8().as_ptr()
	}

	#[doc(hidden)]
	fn to_non_null_u8(self) -> NonNull<u8>;

	#[doc(hidden)]
	fn to_usize(self) -> usize;

	#[doc(hidden)]
	fn from_usize(value: usize) -> Self;
}

impl NonNullU8Ext for MemoryAddress
{
	#[inline(always)]
	fn to_non_null_u8(self) -> NonNull<u8>
	{
		self
	}

	#[inline(always)]
	fn to_usize(self) -> usize
	{
		self.as_ptr() as usize
	}

	#[inline(always)]
	fn from_usize(value: usize) -> Self
	{
		(value as *mut u8).non_null()
	}
}
