use std::num::NonZeroU32;

/// Useful extensions.
pub(crate) trait NonZeroU32Ext: Sized + Copy
{
	/// Add.
	#[inline(always)]
	fn checked_add(self, increment: Self) -> Option<Self>
	{
		self.to_u32().checked_add(increment.to_u32()).map(Self::non_zero_unchecked)
	}

	/// Add.
	#[inline(always)]
	fn add_assign(&mut self, increment: Self)
	{
		*self = Self::non_zero_unchecked(self.to_u32() + increment.to_u32())
	}

	/// Next power of two.
	#[inline(always)]
	fn next_power_of_two(self) -> Self
	{
		Self::non_zero_unchecked(self.to_u32().next_power_of_two())
	}

	/// Non zero.
	#[inline(always)]
	fn non_zero(value: u32) -> Self
	{
		debug_assert_ne!(value, 0, "value is zero");

		Self::non_zero_unchecked(value)
	}

	/// Non zero.
	fn non_zero_unchecked(value: u32) -> Self;

	#[doc(hidden)]
	fn to_u32(self) -> u32;
}

impl NonZeroU32Ext for NonZeroU32
{
	#[inline(always)]
	fn to_u32(self) -> u32
	{
		self.get()
	}

	#[inline(always)]
	fn non_zero_unchecked(value: u32) -> Self
	{
		unsafe { NonZeroU32::new_unchecked(value) }
	}
}
