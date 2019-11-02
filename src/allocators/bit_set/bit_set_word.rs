use crate::allocators::bit_set::bits_in_a_byte::BITS_IN_A_BYTE;
use crate::allocators::bit_set::number_of_bits::NumberOfBits;
use std::mem::size_of;

#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct BitSetWord(pub u64);

impl BitSetWord {
    pub(crate) const SIZE_IN_BYTES: usize = size_of::<u64>();

    pub(crate) const SIZE_IN_BITS: usize = Self::SIZE_IN_BYTES * BITS_IN_A_BYTE;

    #[inline(always)]
    pub(crate) fn leading_unset_bits(self) -> NumberOfBits {
        NumberOfBits(self.0.leading_zeros() as usize)
    }

    #[inline(always)]
    pub(crate) fn trailing_unset_bits(self) -> NumberOfBits {
        NumberOfBits(self.0.trailing_zeros() as usize)
    }

    #[inline(always)]
    pub(crate) fn all_unset_but_not_necessarily_contiguous_bits(self) -> NumberOfBits {
        NumberOfBits(self.0.count_zeros() as usize)
    }

    #[inline(always)]
    pub(crate) fn to_u64(self) -> u64 {
        self.0
    }
}
