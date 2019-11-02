use std::ops::{Add, Sub, SubAssign, Shr};
use crate::allocators::bit_set::bit_set_word_pointer::BitSetWordPointer;
use crate::allocators::bit_set::absolute_location_in_bit_set::AbsoluteLocationInBitSet;
use crate::allocators::bit_set::number_of_bit_set_words::NumberOfBitSetWords;
use crate::allocators::bit_set::number_of_bytes::NumberOfBytes;
use crate::allocators::bit_set::relative_location_in_bit_set::RelativeLocationInBitSet;
use crate::allocators::bit_set::block_size::BlockSize;
use crate::allocators::bit_set::bit_set_word::BitSetWord;

#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct NumberOfBits(pub usize);

impl Add for NumberOfBits
{
	type Output = Self;

	#[inline(always)]
	fn add(self, other: Self) -> Self::Output
	{
		Self(self.0 + other.0)
	}
}

impl Add<usize> for NumberOfBits
{
	type Output = Self;

	#[inline(always)]
	fn add(self, other: usize) -> Self::Output
	{
		Self(self.0 + other)
	}
}

impl Sub for NumberOfBits
{
	type Output = Self;

	#[inline(always)]
	fn sub(self, other: Self) -> Self::Output
	{
		debug_assert!(self >= other, "self `{:?}` is less than other `{:?}`", self, other);

		Self(self.0 - other.0)
	}
}

impl SubAssign for NumberOfBits
{
	#[inline(always)]
	fn sub_assign(&mut self, other: Self)
	{
		debug_assert!(self.0 >= other.0, "self `{:?}` is less than other `{:?}`", self, other);

		self.0 -= other.0
	}
}

impl Shr<usize> for NumberOfBits
{
	type Output = Self;

	#[inline(always)]
	fn shr(self, rhs: usize) -> Self::Output
	{
		Self(self.0 >> rhs)
	}
}

impl NumberOfBits
{
	pub(crate) const Zero: Self = Self(0);

	pub(crate) const InBitSetWord: Self = Self(BitSetWord::SizeInBits);

	#[inline(always)]
	pub(crate) fn is_zero(self) -> bool
	{
		self == Self::Zero
	}

	#[inline(always)]
	pub(crate) fn is_not_zero(self) -> bool
	{
		self != Self::Zero
	}

	#[inline(always)]
	pub(crate) fn to_usize(self) -> usize
	{
		self.0 as usize
	}

	#[inline(always)]
	pub(crate) fn to_u64(self) -> u64
	{
		self.0 as u64
	}

	#[inline(always)]
	pub(crate) fn remainder_of_bits_that_do_not_fit_in_a_bit_set_word(self) -> Self
	{
		Self(self.0 % BitSetWord::SizeInBits)
	}

	#[inline(always)]
	pub(crate) fn round_up_to_number_of_bit_set_words(self) -> NumberOfBitSetWords
	{
		NumberOfBitSetWords((self.0 + BitSetWord::SizeInBits - 1) / BitSetWord::SizeInBits)
	}

	#[inline(always)]
	pub(crate) fn scale_to_memory_offset_in_bytes(self, block_size: &BlockSize) -> NumberOfBytes
	{
		block_size.scale_to_memory_offset_in_bytes(self.0)
	}

	#[inline(always)]
	pub(crate) fn to_absolute_location_in_bit_set(self, inclusive_start_of_bitset: BitSetWordPointer) -> AbsoluteLocationInBitSet
	{
		self.to_relative_location_in_bit_set().to_absolute_location_in_bit_set(inclusive_start_of_bitset)
	}

	#[inline(always)]
	pub(crate) fn to_relative_location_in_bit_set(self) -> RelativeLocationInBitSet
	{
		let major = self.number_of_bit_set_words_rounded_down();
		let minor = self - major.to_number_of_bits();
		RelativeLocationInBitSet
		{
			major,
			minor
		}
	}

	#[inline(always)]
	pub(crate) fn is_one_bit_set_word(self) -> bool
	{
		self.0 == BitSetWord::SizeInBits
	}

	#[inline(always)]
	pub(crate) fn less_than_a_bit_set_word_required(self) -> bool
	{
		self.0 < BitSetWord::SizeInBits
	}

	#[inline(always)]
	pub(crate) fn number_of_bit_set_words_rounded_down(self) -> NumberOfBitSetWords
	{
		NumberOfBitSetWords(self.0 / BitSetWord::SizeInBits)
	}
}
