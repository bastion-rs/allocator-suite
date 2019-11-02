use std::fmt::Debug;

/// Useful extensions.
pub(crate) trait U64Ext: Sized + Copy + Ord + Debug
{
	/// Round down to power of two exponent (usize).
	#[inline(always)]
	fn round_down_to_power_of_two_exponent_usize(self, power_of_two_exponent: usize) -> u64
	{
		self.round_down_to_power_of_two_exponent(power_of_two_exponent as u64)
	}

	/// Round down to power of two exponent (u64).
	#[inline(always)]
	fn round_down_to_power_of_two_exponent(self, power_of_two_exponent: u64) -> u64
	{
		let value = self.to_u64();

		value & !((1 << power_of_two_exponent) - 1)
	}

	#[doc(hidden)]
	fn to_u64(self) -> u64;
}

impl U64Ext for u64
{
	#[inline(always)]
	fn to_u64(self) -> u64
	{
		self
	}
}
