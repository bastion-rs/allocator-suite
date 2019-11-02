use crate::allocators::bit_set::bit_set_word::BitSetWord;
use crate::allocators::bit_set::number_of_bit_set_words::NumberOfBitSetWords;
use crate::allocators::bit_set::number_of_bits::NumberOfBits;
use crate::allocators::bit_set::number_of_bytes::NumberOfBytes;
use crate::extensions::non_null_u8_ext::NonNullU8Ext;
use crate::memory_address::MemoryAddress;
use std::ptr::NonNull;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct BitSetWordPointer(pub NonNull<BitSetWord>);

impl BitSetWordPointer {
    #[inline(always)]
    pub(crate) fn wrap(memory_address: MemoryAddress) -> Self {
        debug_assert_eq!(
            memory_address.to_usize() % BitSetWord::SIZE_IN_BYTES,
            0,
            "memory_address `{:?}` must be a multiple of 8",
            memory_address
        );

        Self(memory_address.cast::<BitSetWord>())
    }

    #[inline(always)]
    pub(crate) fn difference_in_number_of_bits(self, lower: Self) -> NumberOfBits {
        self.difference_in_number_of_bytes(lower)
            .to_number_of_bits()
    }

    #[inline(always)]
    pub(crate) fn difference_in_number_of_bytes(self, lower: Self) -> NumberOfBytes {
        NumberOfBytes(self.memory_address().difference(lower.memory_address()))
    }

    #[inline(always)]
    pub(crate) fn set_bottom_bits(self, number_of_lower_bits_to_set: NumberOfBits) {
        self.memory_address()
            .set_bottom_bits_of_u64(number_of_lower_bits_to_set.0)
    }

    #[inline(always)]
    pub(crate) fn set_some_bits(self, current: BitSetWord, bits_to_set: u64) {
        self.memory_address().write(current.to_u64() | bits_to_set)
    }

    #[inline(always)]
    pub(crate) fn set_top_bits(self, number_of_upper_bits_to_set: NumberOfBits) {
        self.memory_address()
            .set_top_bits_of_u64(number_of_upper_bits_to_set.0)
    }

    #[inline(always)]
    pub(crate) fn set_all_bits_and_increment_assign(&mut self) {
        self.set_all_bits_to(0xFFFF_FFFF_FFFF_FFFF)
    }

    #[inline(always)]
    pub(crate) fn unset_bottom_bits(self, number_of_lower_bits_to_unset: NumberOfBits) {
        self.memory_address()
            .unset_bottom_bits_of_u64(number_of_lower_bits_to_unset.0)
    }

    #[inline(always)]
    pub(crate) fn unset_middle_bits(
        self,
        number_of_bits_to_unset: NumberOfBits,
        number_of_lower_bits: NumberOfBits,
    ) {
        self.memory_address()
            .unset_middle_bits_of_u64(number_of_bits_to_unset.0, number_of_lower_bits.0)
    }

    #[inline(always)]
    pub(crate) fn unset_top_bits(self, number_of_upper_bits_to_unset: NumberOfBits) {
        self.memory_address()
            .unset_top_bits_of_u64(number_of_upper_bits_to_unset.0)
    }

    #[inline(always)]
    pub(crate) fn unset_all_bits_and_increment_assign(&mut self) {
        self.set_all_bits_to(0x0000_0000_0000_0000)
    }

    #[doc(hidden)]
    #[inline(always)]
    pub(crate) fn set_all_bits_to(&mut self, value: u64) {
        let mut memory_address = self.memory_address();
        memory_address.write_and_advance(value);
        self.0 = memory_address.cast::<BitSetWord>();
    }

    #[inline(always)]
    pub(crate) fn increment_assign(&mut self) {
        *self = (*self).increment()
    }

    #[inline(always)]
    pub(crate) fn increment(self) -> Self {
        self.increment_in_bit_set_words(NumberOfBitSetWords::ONE)
    }

    #[inline(always)]
    pub(crate) fn increment_in_bit_set_words(
        self,
        number_of_bit_set_words: NumberOfBitSetWords,
    ) -> Self {
        self.increment_in_bytes(number_of_bit_set_words.to_number_of_bytes())
    }

    #[inline(always)]
    pub(crate) fn bit_set_word(self) -> BitSetWord {
        BitSetWord(self.memory_address().read_u64())
    }

    #[inline(always)]
    pub(crate) fn decrement_in_bit_set_words(
        self,
        number_of_bit_set_words: NumberOfBitSetWords,
    ) -> Self {
        self.decrement_in_bytes(number_of_bit_set_words.to_number_of_bytes())
    }

    #[inline(always)]
    pub(crate) fn increment_in_bytes(self, number_of_bytes: NumberOfBytes) -> Self {
        let number_of_bytes = number_of_bytes.0;

        debug_assert_eq!(
            number_of_bytes % BitSetWord::SIZE_IN_BYTES,
            0,
            "number_of_bytes `{:?}` is not a multiple of the size of an u64",
            number_of_bytes
        );

        Self(
            self.memory_address()
                .add(number_of_bytes)
                .cast::<BitSetWord>(),
        )
    }

    #[inline(always)]
    pub(crate) fn decrement_in_bytes(self, number_of_bytes: NumberOfBytes) -> Self {
        let number_of_bytes = number_of_bytes.0;

        debug_assert_eq!(
            number_of_bytes % BitSetWord::SIZE_IN_BYTES,
            0,
            "number_of_bytes `{:?}` is not a multiple of the size of an u64",
            number_of_bytes
        );

        Self(
            self.memory_address()
                .subtract(number_of_bytes)
                .cast::<BitSetWord>(),
        )
    }

    #[doc(hidden)]
    #[inline(always)]
    pub(crate) fn memory_address(self) -> MemoryAddress {
        self.0.cast::<u8>()
    }
}
