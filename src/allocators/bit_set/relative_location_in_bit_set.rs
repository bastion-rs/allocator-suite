use crate::allocators::bit_set::absolute_location_in_bit_set::AbsoluteLocationInBitSet;
use crate::allocators::bit_set::bit_set_word_pointer::BitSetWordPointer;
use crate::allocators::bit_set::number_of_bits::NumberOfBits;
use crate::allocators::bit_set::number_of_bit_set_words::NumberOfBitSetWords;

/// This is a mixed-radix representation.
#[derive(Default, Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub(crate) struct RelativeLocationInBitSet
{
	pub(crate) major: NumberOfBitSetWords,
	pub(crate) minor: NumberOfBits,
}

impl RelativeLocationInBitSet
{
	#[inline(always)]
	pub(crate) fn to_absolute_location_in_bit_set(self, inclusive_start_of_bitset: BitSetWordPointer) -> AbsoluteLocationInBitSet
	{
		AbsoluteLocationInBitSet
		{
			major: inclusive_start_of_bitset.increment_in_bit_set_words(self.major),
			minor: self.minor,
		}
	}
}
