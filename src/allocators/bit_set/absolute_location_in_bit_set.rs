use crate::allocators::bit_set::bit_set_word_pointer::BitSetWordPointer;
use crate::allocators::bit_set::number_of_bits::NumberOfBits;

/// This is a mixed-radix representation.
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct AbsoluteLocationInBitSet {
    pub(crate) major: BitSetWordPointer,
    pub(crate) minor: NumberOfBits,
}

impl AbsoluteLocationInBitSet {
    #[inline(always)]
    pub(crate) fn align_upwards_to_next_bit_set_word_pointer<R>(
        self,
        value_to_return_if_aligned: R,
        action_if_unaligned: impl FnOnce(&Self) -> R,
    ) -> (BitSetWordPointer, R) {
        if unlikely!(self.minor.is_zero()) {
            (self.major, value_to_return_if_aligned)
        } else {
            let value_to_return = action_if_unaligned(&self);
            (self.major.increment(), value_to_return)
        }
    }
}
