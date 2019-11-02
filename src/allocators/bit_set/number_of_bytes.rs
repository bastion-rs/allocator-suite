use super::extensions::prelude::*;
use crate::allocators::bit_set::bits_in_a_byte::BITS_IN_A_BYTE;
use crate::allocators::bit_set::number_of_bits::NumberOfBits;
use std::num::NonZeroUsize;
use std::ops::Sub;

use std::fmt::Debug;

#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub(crate) struct NumberOfBytes(pub usize);

impl Sub for NumberOfBytes {
    type Output = Self;

    #[inline(always)]
    fn sub(self, other: Self) -> Self::Output {
        debug_assert!(self.0 >= other.0);

        Self(self.0 - other.0)
    }
}

impl NumberOfBytes {
    #[inline(always)]
    pub(crate) fn is_zero(self) -> bool {
        self.0 == 0
    }

    #[inline(always)]
    pub(crate) fn is_not_zero(self) -> bool {
        self.0 != 0
    }

    #[inline(always)]
    pub(crate) fn to_usize(self) -> usize {
        self.0
    }

    #[inline(always)]
    pub(crate) fn to_non_zero(self) -> NonZeroUsize {
        self.0.non_zero()
    }

    #[inline(always)]
    pub(crate) fn to_number_of_bits(self) -> NumberOfBits {
        NumberOfBits(self.0 * BITS_IN_A_BYTE)
    }
}
