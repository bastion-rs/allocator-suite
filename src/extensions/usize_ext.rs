use std::num::NonZeroUsize;
use std::fmt::Debug;
use crate::extensions::non_zero_usize_ext::NonZeroUsizeExt;

/// Useful extensions.
pub trait UsizeExt: Sized + Copy + Ord + Debug
{
	/// Is odd.
	#[inline(always)]
	fn is_odd(self) -> bool
	{
		self.to_usize() & 0b1 == 0b1
	}

	/// Round up.
	#[inline(always)]
	fn round_up_to_power_of_two(self, non_zero_power_of_two_alignment: NonZeroUsize) -> usize
	{
		let power_of_two = non_zero_power_of_two_alignment.get();
		let power_of_two_less_one = power_of_two - 1;

		let value = self.to_usize();

		debug_assert!(value.checked_add(power_of_two_less_one).is_some(), "non_zero_power_of_two_alignment is far too close to the maximum value of a pointer");

		(value + power_of_two_less_one) & !power_of_two_less_one
	}

	/// Round down.
	#[inline(always)]
	fn round_down_to_power_of_two(self, power_of_two: NonZeroUsize) -> usize
	{
		let power_of_two_exponent = power_of_two.logarithm_base2();
		self.round_down_to_power_of_two_exponent(power_of_two_exponent)
	}

	/// Round down to power of two exponent.
	#[inline(always)]
	fn round_down_to_power_of_two_exponent(self, power_of_two_exponent: usize) -> usize
	{
		let value = self.to_usize();

		value & !((1 << power_of_two_exponent) - 1)
	}

	/// Non zero.
	#[inline(always)]
	fn non_zero(self) -> NonZeroUsize
	{
		NonZeroUsize::non_zero(self.to_usize())
	}

	#[doc(hidden)]
	fn to_usize(self) -> usize;
}

impl UsizeExt for usize
{
	#[inline(always)]
	fn to_usize(self) -> usize
	{
		self
	}
}
