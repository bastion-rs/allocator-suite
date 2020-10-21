use crate::allocators::allocator::Allocator;
use crate::allocators::bit_set::absolute_location_in_bit_set::AbsoluteLocationInBitSet;
use crate::allocators::bit_set::bit_set_word::BitSetWord;
use crate::allocators::bit_set::bit_set_word_pointer::BitSetWordPointer;
use crate::allocators::bit_set::block_size::BlockSize;
use crate::allocators::bit_set::number_of_bits::NumberOfBits;
use crate::allocators::bit_set::number_of_bytes::NumberOfBytes;
use crate::allocators::global::local_allocator::LocalAllocator;
use crate::allocators::global::memory_range::MemoryRange;
use crate::extensions::prelude::*;
use crate::memory_address::MemoryAddress;
use crate::memory_sources::memory_source::MemorySource;
use either::*;
use std::alloc::AllocError;
use std::cell::Cell;
use std::num::NonZeroUsize;

/// Bit set based allocator.
#[derive(Debug)]
pub struct BitSetAllocator<MS: MemorySource> {
    inclusive_start_of_bit_set: BitSetWordPointer,
    exclusive_end_of_bit_set: BitSetWordPointer,
    start_search_for_next_allocation_at: Cell<BitSetWordPointer>,

    allocations_start_from: MemoryAddress,
    allocations_end_at: MemoryAddress,

    block_size: BlockSize,

    memory_source: MS,
    memory_source_size: NonZeroUsize,
}

impl<MS: MemorySource> Drop for BitSetAllocator<MS> {
    #[inline(always)]
    fn drop(&mut self) {
        self.memory_source
            .release(self.memory_source_size, self.allocations_start_from)
    }
}

impl<MS: MemorySource> Allocator for BitSetAllocator<MS> {
    #[inline(always)]
    fn allocate(
        &self,
        non_zero_size: NonZeroUsize,
        non_zero_power_of_two_alignment: NonZeroUsize,
    ) -> Result<MemoryAddress, AllocError> {
        let number_of_bits_required = self.number_of_bits_required(non_zero_size);

        let power_of_two_exponent = if self
            .block_size
            .alignment_is_minimum(non_zero_power_of_two_alignment)
        {
            0
        } else {
            let power_of_two_exponent = non_zero_power_of_two_alignment.logarithm_base2()
                - self.block_size.block_size_power_of_two_exponent;

            let alignment_exceeds_that_which_can_be_accommodated_in_one_bit_set_word =
                power_of_two_exponent > BitSetWord::SIZE_IN_BITS;
            if unlikely!(alignment_exceeds_that_which_can_be_accommodated_in_one_bit_set_word) {
                return Err(AllocError);
            }

            power_of_two_exponent
        };

        self.try_to_set_number_of_bits(number_of_bits_required, power_of_two_exponent)
    }

    #[inline(always)]
    fn deallocate(
        &self,
        non_zero_size: NonZeroUsize,
        _non_zero_power_of_two_alignment: NonZeroUsize,
        current_memory: MemoryAddress,
    ) {
        #[inline(always)]
        fn unset_unaligned_trailing_bits_at_front(
            location: AbsoluteLocationInBitSet,
            number_of_bits_required: NumberOfBits,
        ) -> (BitSetWordPointer, NumberOfBits) {
            let (location_major, bits_unset_to_reach_alignment) = location
                .align_upwards_to_next_bit_set_word_pointer(NumberOfBits::ZERO, |location| {
                    let number_of_lower_bits = NumberOfBits::IN_BIT_SET_WORD - location.minor;

                    if likely!(number_of_bits_required >= number_of_lower_bits) {
                        location.major.unset_bottom_bits(number_of_lower_bits);
                        number_of_lower_bits
                    } else {
                        location
                            .major
                            .unset_middle_bits(number_of_bits_required, number_of_lower_bits);
                        number_of_bits_required
                    }
                });

            let remaining_bits_to_unset_in_middle_and_at_end =
                number_of_bits_required - bits_unset_to_reach_alignment;
            (location_major, remaining_bits_to_unset_in_middle_and_at_end)
        }

        #[inline(always)]
        fn unset_aligned_bits_in_middle(
            mut location_major: BitSetWordPointer,
            mut remaining_bits_to_unset_in_middle_and_at_end: NumberOfBits,
        ) -> (BitSetWordPointer, NumberOfBits) {
            while remaining_bits_to_unset_in_middle_and_at_end >= NumberOfBits::IN_BIT_SET_WORD {
                location_major.unset_all_bits_and_increment_assign();
                remaining_bits_to_unset_in_middle_and_at_end -= NumberOfBits::IN_BIT_SET_WORD;
            }

            (location_major, remaining_bits_to_unset_in_middle_and_at_end)
        }

        #[inline(always)]
        fn unset_unaligned_leading_bits_at_end(
            location_major: BitSetWordPointer,
            remaining_bits_to_unset_at_end: NumberOfBits,
        ) {
            if likely!(remaining_bits_to_unset_at_end.is_not_zero()) {
                location_major.unset_top_bits(remaining_bits_to_unset_at_end);
            }
        }

        let location = self.absolute_location_in_bit_set(current_memory);
        let number_of_bits_required = self.number_of_bits_required(non_zero_size);

        let (location_major, remaining_bits_to_unset_in_middle_and_at_end) =
            unset_unaligned_trailing_bits_at_front(location, number_of_bits_required);
        let (location_major, remaining_bits_to_unset_at_end) = unset_aligned_bits_in_middle(
            location_major,
            remaining_bits_to_unset_in_middle_and_at_end,
        );
        unset_unaligned_leading_bits_at_end(location_major, remaining_bits_to_unset_at_end);
    }

    #[inline(always)]
    fn growing_reallocate(
        &self,
        non_zero_new_size: NonZeroUsize,
        non_zero_power_of_two_alignment: NonZeroUsize,
        non_zero_current_size: NonZeroUsize,
        current_memory: MemoryAddress,
    ) -> Result<MemoryAddress, AllocError> {
        let current_number_of_bits_required = self.number_of_bits_required(non_zero_current_size);
        let new_number_of_bits_required = self.number_of_bits_required(non_zero_new_size);

        let current_memory_offset_in_bytes =
            current_number_of_bits_required.scale_to_memory_offset_in_bytes(&self.block_size);
        let new_memory_offset_in_bytes =
            new_number_of_bits_required.scale_to_memory_offset_in_bytes(&self.block_size);

        let reallocate_size = new_memory_offset_in_bytes - current_memory_offset_in_bytes;
        if unlikely!(reallocate_size.is_zero()) {
            return Ok(current_memory);
        }

        self.deallocate(
            non_zero_current_size,
            non_zero_power_of_two_alignment,
            current_memory,
        );
        self.start_search_for_next_allocation_at.set({
            let location = self.absolute_location_in_bit_set(current_memory);
            location.major
        });
        let allocated = self.allocate(non_zero_new_size, non_zero_power_of_two_alignment)?;

        if likely!(allocated != current_memory) {
            #[inline(always)]
            fn memmove(
                from: MemoryAddress,
                to: MemoryAddress,
                non_zero_current_size: NonZeroUsize,
            ) {
                unsafe {
                    to.as_ptr()
                        .copy_from(from.as_ptr() as *const _, non_zero_current_size.get())
                };
            }
            memmove(current_memory, allocated, non_zero_current_size)
        }
        Ok(allocated)
    }

    #[inline(always)]
    fn shrinking_reallocate(
        &self,
        non_zero_new_size: NonZeroUsize,
        non_zero_power_of_two_alignment: NonZeroUsize,
        non_zero_current_size: NonZeroUsize,
        current_memory: MemoryAddress,
    ) -> Result<MemoryAddress, AllocError> {
        let current_number_of_bits_required = self.number_of_bits_required(non_zero_current_size);
        let new_number_of_bits_required = self.number_of_bits_required(non_zero_new_size);

        let current_memory_offset_in_bytes =
            current_number_of_bits_required.scale_to_memory_offset_in_bytes(&self.block_size);
        let new_memory_offset_in_bytes =
            new_number_of_bits_required.scale_to_memory_offset_in_bytes(&self.block_size);

        let deallocate_size = current_memory_offset_in_bytes - new_memory_offset_in_bytes;
        if likely!(deallocate_size.is_not_zero()) {
            let end_of_new_memory = current_memory.add(new_memory_offset_in_bytes.to_usize());
            self.deallocate(
                deallocate_size.to_non_zero(),
                non_zero_power_of_two_alignment,
                end_of_new_memory,
            )
        }
        Ok(current_memory)
    }
}

impl<MS: MemorySource> LocalAllocator for BitSetAllocator<MS> {
    #[inline(always)]
    fn memory_range(&self) -> MemoryRange {
        MemoryRange::new(self.allocations_start_from, self.allocations_end_at)
    }
}

impl<MS: MemorySource> BitSetAllocator<MS> {
    /// New instance wrapping a block of memory for an 8 byte block size.
    #[inline(always)]
    pub fn new_by_amount_8(
        memory_source: MS,
        memory_source_size: NonZeroUsize,
    ) -> Result<Self, AllocError> {
        Self::new_by_amount(memory_source, 8usize.non_zero(), memory_source_size)
    }

    /// New instance wrapping a block of memory for a 16 byte block size.
    #[inline(always)]
    pub fn new_by_amount_16(
        memory_source: MS,
        memory_source_size: NonZeroUsize,
    ) -> Result<Self, AllocError> {
        Self::new_by_amount(memory_source, 16usize.non_zero(), memory_source_size)
    }

    /// New instance wrapping a block of memory for a 32 byte block size.
    #[inline(always)]
    pub fn new_by_amount_32(
        memory_source: MS,
        memory_source_size: NonZeroUsize,
    ) -> Result<Self, AllocError> {
        Self::new_by_amount(memory_source, 32usize.non_zero(), memory_source_size)
    }

    /// Create a new instance by memory size and block size.
    #[inline(always)]
    pub fn new_by_amount(
        memory_source: MS,
        block_size: NonZeroUsize,
        memory_source_size: NonZeroUsize,
    ) -> Result<Self, AllocError> {
        let number_of_blocks =
            ((memory_source_size.get() + (block_size.get() - 1)) / block_size.get()).non_zero();

        Self::new(memory_source, block_size, number_of_blocks)
    }

    /// Create a new instance.
    #[inline(always)]
    pub fn new(
        memory_source: MS,
        block_size: NonZeroUsize,
        number_of_blocks: NonZeroUsize,
    ) -> Result<Self, AllocError> {
        debug_assert!(
            block_size.is_power_of_two(),
            "block_size `{:?}` must be a power of 2",
            block_size
        );
        debug_assert!(block_size.get() >= BitSetWord::SIZE_IN_BYTES, "block_size `{:?}` must at least `{:?}` so that the bit set metadata holding free blocks can be allocated contiguous with the memory used for blocks", block_size, BitSetWord::SIZE_IN_BYTES);

        let size_in_bytes = number_of_blocks.get() << block_size.logarithm_base2();
        let bit_set_size_in_bytes =
            number_of_blocks.get() / NumberOfBits::IN_BIT_SET_WORD.to_usize();
        let memory_source_size = (size_in_bytes + bit_set_size_in_bytes).non_zero();
        let allocations_start_from = memory_source.obtain(memory_source_size)?;

        let allocations_end_at = allocations_start_from.add(size_in_bytes);
        let (inclusive_start_of_bit_set, exclusive_end_of_bit_set) =
            Self::initialize_bit_set_so_all_memory_is_unallocated(
                allocations_end_at,
                bit_set_size_in_bytes,
            );

        Ok(Self {
            inclusive_start_of_bit_set,
            exclusive_end_of_bit_set,
            start_search_for_next_allocation_at: Cell::new(inclusive_start_of_bit_set),

            allocations_start_from,
            allocations_end_at,

            block_size: BlockSize::new(block_size),

            memory_source_size,
            memory_source,
        })
    }

    #[inline(always)]
    fn initialize_bit_set_so_all_memory_is_unallocated(
        allocations_end_at: MemoryAddress,
        bit_set_size_in_bytes: usize,
    ) -> (BitSetWordPointer, BitSetWordPointer) {
        unsafe {
            allocations_end_at
                .as_ptr()
                .write_bytes(0x00, bit_set_size_in_bytes)
        };
        let inclusive_start_of_bit_set = BitSetWordPointer::wrap(allocations_end_at);
        (
            inclusive_start_of_bit_set,
            inclusive_start_of_bit_set.increment_in_bytes(NumberOfBytes(bit_set_size_in_bytes)),
        )
    }

    #[inline(always)]
    fn absolute_location_in_bit_set(
        &self,
        start_of_allocated_memory: MemoryAddress,
    ) -> AbsoluteLocationInBitSet {
        let blocks_offset = self
            .block_size
            .blocks_offset(self.allocations_start_from, start_of_allocated_memory);
        blocks_offset.to_absolute_location_in_bit_set(self.inclusive_start_of_bit_set)
    }

    #[inline(always)]
    fn number_of_bits_required(&self, non_zero_size: NonZeroUsize) -> NumberOfBits {
        self.block_size.number_of_blocks_required(non_zero_size)
    }

    #[inline(always)]
    fn try_to_set_number_of_bits(
        &self,
        number_of_bits_required: NumberOfBits,
        power_of_two_exponent: usize,
    ) -> Result<MemoryAddress, AllocError> {
        debug_assert!(number_of_bits_required.is_not_zero());

        macro_rules! scan
		{
			($self: ident, $number_of_bits_required: ident, $power_of_two_exponent: ident, $end_bit_set_word_pointer: ident, $callback: ident) =>
			{
				{
					let mut contiguous_unset_bits_count = NumberOfBits::ZERO;
					let mut bit_set_word_pointer = $self.start_search_for_next_allocation_at.get();
					while bit_set_word_pointer != $end_bit_set_word_pointer
					{
						let current = bit_set_word_pointer.bit_set_word();

						let current_leading_unset_bits = current.leading_unset_bits();
						let contiguous_unset_bits_now_available = contiguous_unset_bits_count + current_leading_unset_bits;

						// This statement requires no additional corrections as long as alignment can not exceed 64-bits (eg for an 8 byte block, that is an alignment of 512 bytes).
						if contiguous_unset_bits_now_available >= $number_of_bits_required
						{
							return Ok($self.allocate_in_contiguous_unset_bits(contiguous_unset_bits_count, bit_set_word_pointer, $number_of_bits_required))
						}

						contiguous_unset_bits_count = match $callback($self, $number_of_bits_required, bit_set_word_pointer, current, current_leading_unset_bits, contiguous_unset_bits_now_available, $power_of_two_exponent)
						{
							Left(successful_allocation) => return Ok(successful_allocation),

							Right(contiguous_unset_bits_count) => contiguous_unset_bits_count,
						};

						bit_set_word_pointer.increment_assign();
					}
				}
			}
		}

        let callback = if number_of_bits_required.less_than_a_bit_set_word_required() {
            Self::number_of_blocks_is_less_than_64
        } else {
            Self::number_of_blocks_is_64_or_more
        };

        let end_bit_set_word_pointer = self.exclusive_end_of_bit_set;
        scan!(
            self,
            number_of_bits_required,
            power_of_two_exponent,
            end_bit_set_word_pointer,
            callback
        );

        let end_bit_set_word_pointer = self
            .start_search_for_next_allocation_at
            .replace(self.inclusive_start_of_bit_set);
        scan!(
            self,
            number_of_bits_required,
            power_of_two_exponent,
            end_bit_set_word_pointer,
            callback
        );

        Err(AllocError)
    }

    #[inline(always)]
    fn allocate_in_contiguous_unset_bits(
        &self,
        bits_to_set_at_front_and_in_middle: NumberOfBits,
        bit_set_word_pointer: BitSetWordPointer,
        number_of_bits_required: NumberOfBits,
    ) -> MemoryAddress {
        #[inline(always)]
        fn set_unaligned_trailing_bits_in_front(
            bits_to_set_at_front_and_in_middle: NumberOfBits,
            bit_set_word_pointer: BitSetWordPointer,
            inclusive_start_of_bit_set: BitSetWordPointer,
        ) -> (BitSetWordPointer, NumberOfBits, NumberOfBits) {
            let unaligned_trailing_bits_in_front = bits_to_set_at_front_and_in_middle
                .remainder_of_bits_that_do_not_fit_in_a_bit_set_word();

            let starts_from = bit_set_word_pointer.decrement_in_bit_set_words(
                bits_to_set_at_front_and_in_middle.round_up_to_number_of_bit_set_words(),
            );

            let rounded_down_number_of_bits =
                starts_from.difference_in_number_of_bits(inclusive_start_of_bit_set);

            if likely!(unaligned_trailing_bits_in_front.is_not_zero()) {
                starts_from.set_bottom_bits(unaligned_trailing_bits_in_front);
                let offset_into_bit_set = rounded_down_number_of_bits
                    + (NumberOfBits::IN_BIT_SET_WORD - unaligned_trailing_bits_in_front);
                (
                    starts_from.increment(),
                    bits_to_set_at_front_and_in_middle - unaligned_trailing_bits_in_front,
                    offset_into_bit_set,
                )
            } else {
                (
                    starts_from,
                    bits_to_set_at_front_and_in_middle,
                    rounded_down_number_of_bits,
                )
            }
        }

        #[inline(always)]
        fn set_aligned_bits_in_middle(
            mut location_major: BitSetWordPointer,
            mut remaining_bits_to_set_in_middle: NumberOfBits,
        ) -> BitSetWordPointer {
            while remaining_bits_to_set_in_middle.is_not_zero() {
                debug_assert!(remaining_bits_to_set_in_middle >= NumberOfBits::IN_BIT_SET_WORD);

                location_major.set_all_bits_and_increment_assign();
                remaining_bits_to_set_in_middle -= NumberOfBits::IN_BIT_SET_WORD;
            }

            location_major
        }

        #[inline(always)]
        fn set_unaligned_leading_bits_in_end(
            location_major: BitSetWordPointer,
            bits_to_set_at_end: NumberOfBits,
        ) {
            if likely!(bits_to_set_at_end.is_not_zero()) {
                location_major.set_top_bits(bits_to_set_at_end)
            }
        }

        let (location_major, bits_to_set_in_middle, offset_into_bit_set) =
            set_unaligned_trailing_bits_in_front(
                bits_to_set_at_front_and_in_middle,
                bit_set_word_pointer,
                self.inclusive_start_of_bit_set,
            );
        let location_major = set_aligned_bits_in_middle(location_major, bits_to_set_in_middle);
        debug_assert_eq!(location_major, bit_set_word_pointer);
        let bits_to_set_at_end = number_of_bits_required - bits_to_set_at_front_and_in_middle;
        set_unaligned_leading_bits_in_end(location_major, bits_to_set_at_end);

        self.successful_allocation(bit_set_word_pointer, offset_into_bit_set)
    }

    #[inline(always)]
    fn number_of_blocks_is_less_than_64(
        &self,
        number_of_bits_required: NumberOfBits,
        bit_set_word_pointer: BitSetWordPointer,
        current: BitSetWord,
        current_leading_unset_bits: NumberOfBits,
        _contiguous_unset_bits_now_available: NumberOfBits,
        power_of_two_exponent: usize,
    ) -> Either<MemoryAddress, NumberOfBits> {
        debug_assert!(current_leading_unset_bits < NumberOfBits::IN_BIT_SET_WORD, "If there are 64 leading unset bits, and this allocation is for less than 64 blocks, then it should have been allocated successfully prior to this method");
        debug_assert!(number_of_bits_required > current_leading_unset_bits);

        let quick_check_to_eliminate_most_cases_that_are_likely_to_be_unsuccessful =
            current.all_unset_but_not_necessarily_contiguous_bits() - current_leading_unset_bits
                < number_of_bits_required;
        if unlikely!(quick_check_to_eliminate_most_cases_that_are_likely_to_be_unsuccessful) {
            return Right(Self::aligned_trailing_unset_bits(
                current,
                power_of_two_exponent,
            ));
        }

        let (aligned_shift, shift_decrement) = {
            let unaligned_shift = {
                let lowest_top_bit_count = {
                    let irrelevant_top_bits_count = current_leading_unset_bits + 1;
                    let lowest_top_bit_count = irrelevant_top_bits_count + number_of_bits_required;
                    if unlikely!(lowest_top_bit_count > NumberOfBits::IN_BIT_SET_WORD) {
                        return Right(Self::aligned_trailing_unset_bits(
                            current,
                            power_of_two_exponent,
                        ));
                    }
                    lowest_top_bit_count
                };

                (NumberOfBits::IN_BIT_SET_WORD - lowest_top_bit_count).to_u64()
            };

            let shift_decrement = 1 << power_of_two_exponent;

            let too_few_bits_available_for_alignment =
                unaligned_shift != 0 && unaligned_shift < shift_decrement as u64;
            if unlikely!(too_few_bits_available_for_alignment) {
                return Right(Self::aligned_trailing_unset_bits(
                    current,
                    power_of_two_exponent,
                ));
            }
            let aligned_shift =
                unaligned_shift.round_down_to_power_of_two_exponent_usize(power_of_two_exponent);

            (aligned_shift, shift_decrement)
        };

        let unshifted_bits_to_set = (1 << number_of_bits_required.to_u64()) - 1;
        let mut shift = aligned_shift;
        loop {
            let bits_to_set = unshifted_bits_to_set << shift;
            let all_bits_to_set_are_currently_unset = current.to_u64() & bits_to_set == 0;
            if all_bits_to_set_are_currently_unset {
                return {
                    bit_set_word_pointer.set_some_bits(current, bits_to_set);

                    let offset_into_bit_set = {
                        let major_location = bit_set_word_pointer
                            .difference_in_number_of_bits(self.inclusive_start_of_bit_set);
                        let minor_location = NumberOfBits::IN_BIT_SET_WORD
                            - (number_of_bits_required + NumberOfBits(shift as usize));
                        major_location + minor_location
                    };

                    Left(self.successful_allocation(bit_set_word_pointer, offset_into_bit_set))
                };
            }

            if unlikely!(shift == 0) {
                return Right(Self::aligned_trailing_unset_bits(
                    current,
                    power_of_two_exponent,
                ));
            }
            shift -= shift_decrement as u64;
        }
    }

    #[inline(always)]
    fn number_of_blocks_is_64_or_more(
        &self,
        _number_of_bits_required: NumberOfBits,
        _bit_set_word_pointer: BitSetWordPointer,
        current: BitSetWord,
        current_leading_unset_bits: NumberOfBits,
        contiguous_unset_bits_now_available: NumberOfBits,
        power_of_two_exponent: usize,
    ) -> Either<MemoryAddress, NumberOfBits> {
        if likely!(current_leading_unset_bits.is_one_bit_set_word()) {
            Right(contiguous_unset_bits_now_available)
        } else {
            Right(Self::aligned_trailing_unset_bits(
                current,
                power_of_two_exponent,
            ))
        }
    }

    #[inline(always)]
    fn successful_allocation(
        &self,
        bit_set_word_pointer: BitSetWordPointer,
        offset_into_bit_set: NumberOfBits,
    ) -> MemoryAddress {
        self.start_search_for_next_allocation_at
            .set(bit_set_word_pointer);
        self.allocations_start_from.add(
            offset_into_bit_set
                .scale_to_memory_offset_in_bytes(&self.block_size)
                .to_usize(),
        )
    }

    #[inline(always)]
    fn aligned_trailing_unset_bits(
        current: BitSetWord,
        power_of_two_exponent: usize,
    ) -> NumberOfBits {
        let unaligned_trailing_unset_bits = current.trailing_unset_bits();
        unaligned_trailing_unset_bits >> power_of_two_exponent
    }
}
