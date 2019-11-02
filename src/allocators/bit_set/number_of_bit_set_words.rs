use crate::allocators::bit_set::bit_set_word::BitSetWord;
use crate::allocators::bit_set::number_of_bits::NumberOfBits;
use crate::allocators::bit_set::number_of_bytes::NumberOfBytes;
use std::ops::Sub;

#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub(crate) struct NumberOfBitSetWords(pub usize);

impl NumberOfBitSetWords {
    pub(crate) const One: Self = Self(1);

    #[inline(always)]
    pub(crate) fn to_number_of_bytes(self) -> NumberOfBytes {
        NumberOfBytes(self.0 * BitSetWord::SizeInBytes)
    }

    #[inline(always)]
    pub(crate) fn to_number_of_bits(self) -> NumberOfBits {
        NumberOfBits(self.0 * BitSetWord::SizeInBits)
    }
}

impl Sub for NumberOfBitSetWords {
    type Output = Self;

    #[inline(always)]
    fn sub(self, other: Self) -> Self::Output {
        debug_assert!(
            self >= other,
            "self `{:?}` is less than other `{:?}`",
            self,
            other
        );

        Self(self.0 - other.0)
    }
}
