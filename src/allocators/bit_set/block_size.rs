use crate::allocators::bit_set::number_of_bits::NumberOfBits;
use crate::allocators::bit_set::number_of_bytes::NumberOfBytes;
use crate::extensions::non_null_u8_ext::NonNullU8Ext;
use crate::extensions::non_zero_usize_ext::NonZeroUsizeExt;
use crate::memory_address::MemoryAddress;
use std::num::NonZeroUsize;

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub(crate) struct BlockSize {
    block_size: NonZeroUsize,
    block_size_less_one: usize,
    pub(crate) block_size_power_of_two_exponent: usize,
}

impl BlockSize {
    #[inline(always)]
    pub(crate) fn new(block_size: NonZeroUsize) -> Self {
        debug_assert!(
            block_size.is_power_of_two(),
            "block_size `{:?}` is not a power of two",
            block_size
        );

        Self {
            block_size,
            block_size_less_one: block_size.decrement(),
            block_size_power_of_two_exponent: block_size.logarithm_base2(),
        }
    }

    #[inline(always)]
    pub(crate) fn alignment_is_minimum(
        &self,
        non_zero_power_of_two_alignment: NonZeroUsize,
    ) -> bool {
        non_zero_power_of_two_alignment <= self.block_size
    }

    #[inline(always)]
    pub(crate) fn number_of_blocks_required(&self, non_zero_size: NonZeroUsize) -> NumberOfBits {
        NumberOfBits(
            (non_zero_size.get() + self.block_size_less_one)
                >> self.block_size_power_of_two_exponent,
        )
    }

    #[inline(always)]
    pub(crate) fn blocks_offset(
        &self,
        allocations_start_from: MemoryAddress,
        start_of_allocated_memory: MemoryAddress,
    ) -> NumberOfBits {
        debug_assert!(
            start_of_allocated_memory >= allocations_start_from,
            "start_of_allocated_memory must be >= allocations_start_from"
        );

        NumberOfBits(
            start_of_allocated_memory.difference(allocations_start_from)
                >> self.block_size_power_of_two_exponent,
        )
    }

    #[inline(always)]
    pub(crate) fn scale_to_memory_offset_in_bytes(&self, number_of_bits: usize) -> NumberOfBytes {
        NumberOfBytes(number_of_bits << self.block_size_power_of_two_exponent)
    }
}
