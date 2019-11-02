use std::fmt::Formatter;
use std::fmt;
use super::extensions::prelude::*;
use std::mem::size_of;
use std::cmp::max;
use super::allocators::binary_search_trees::binary_search_tree_with_cached_knowledge_of_first_child::BinarySearchTreeWithCachedKnowledgeOfFirstChild;
use super::allocators::binary_search_trees::red_black_tree::prelude::*;
use std::cell::UnsafeCell;
use std::num::NonZeroUsize;
use std::fmt::Debug;

pub(crate) struct BinarySearchTreesWithCachedKnowledgeOfFirstChild {
    binary_search_trees_of_free_blocks_sorted_by_ascending_memory_address_and_indexed_by_power_of_two_exponent_less_smallest_power_of_two:
        [UnsafeCell<BinarySearchTreeWithCachedKnowledgeOfFirstChild>;
            Self::NumberOfBinarySearchTrees],
}

impl Debug for BinarySearchTreesWithCachedKnowledgeOfFirstChild {
    #[inline(always)]
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        writeln!(f)?;
        writeln!(f, "\tBlockSize => Count  Cached first child is null?")?;
        for binary_search_tree_index in 0..Self::NumberOfBinarySearchTrees {
            let block_size = Self::binary_search_tree_index_to_block_size(binary_search_tree_index);
            let binary_search_tree = self.binary_search_trees_of_free_blocks_sorted_by_ascending_memory_address_and_indexed_by_power_of_two_exponent_less_smallest_power_of_two[binary_search_tree_index].get().mutable_reference();

            let has_blocks = binary_search_tree.has_blocks();
            if has_blocks {
                let mut count = 0;
                for _ in binary_search_tree.double_ended_iterate() {
                    count += 1;
                }

                writeln!(
                    f,
                    "\t{:?} => {:?}  {:?}",
                    block_size,
                    count,
                    binary_search_tree.cached_first_child().is_null()
                )?;
            }
        }
        Ok(())
    }
}

impl Default for BinarySearchTreesWithCachedKnowledgeOfFirstChild {
    #[inline(always)]
    fn default() -> Self {
        Self
		{
			binary_search_trees_of_free_blocks_sorted_by_ascending_memory_address_and_indexed_by_power_of_two_exponent_less_smallest_power_of_two: Default::default(),
		}
    }
}

impl BinarySearchTreesWithCachedKnowledgeOfFirstChild {
    const SmallestInclusivePowerOfTwoExponent: NonZeroUsize =
        Self::logarithm_base2(size_of::<Node>());

    pub(crate) const NumberOfBinarySearchTrees: usize = 16;

    const LargestInclusiveBinarySearchTreeIndex: usize = Self::NumberOfBinarySearchTrees - 1;

    const LargestInclusivePowerOfTwoExponent: NonZeroUsize =
        non_zero_usize(Self::binary_search_tree_index_to_power_of_two_exponent(
            Self::LargestInclusiveBinarySearchTreeIndex,
        ));

    pub(crate) const MinimumAllocationSize: NonZeroUsize =
        non_zero_usize(1 << Self::SmallestInclusivePowerOfTwoExponent.get());

    pub(crate) const MaximumAllocationSize: NonZeroUsize =
        non_zero_usize(1 << Self::LargestInclusivePowerOfTwoExponent.get());

    pub(crate) const MinimumAlignment: NonZeroUsize = Self::MinimumAllocationSize;

    const MaximumAlignment: NonZeroUsize = Self::MaximumAllocationSize;

    #[inline(always)]
    const fn logarithm_base2(value: usize) -> NonZeroUsize {
        non_zero_usize(logarithm_base2_as_usize(value))
    }

    #[inline(always)]
    pub(crate) fn binary_search_tree_index(block_size: NonZeroUsize) -> usize {
        debug_assert_eq!(
            block_size.next_power_of_two(),
            block_size,
            "A block_size was not passed"
        );
        debug_assert!(
            block_size >= Self::MinimumAllocationSize,
            "Block size was too small"
        );
        debug_assert!(
            block_size <= Self::MaximumAllocationSize,
            "Block size was too large"
        );

        let power_of_two_exponent = logarithm_base2_as_usize(block_size.get());

        power_of_two_exponent - Self::SmallestInclusivePowerOfTwoExponent.get()
    }

    #[inline(always)]
    const fn binary_search_tree_index_to_power_of_two_exponent(
        binary_search_tree_index: usize,
    ) -> usize {
        Self::SmallestInclusivePowerOfTwoExponent.get() + binary_search_tree_index
    }

    #[inline(always)]
    pub(crate) fn binary_search_tree_index_to_block_size(binary_search_tree_index: usize) -> usize {
        1 << Self::binary_search_tree_index_to_power_of_two_exponent(binary_search_tree_index)
    }

    #[inline(always)]
    pub(crate) fn size_is_less_than_minimum_allocation_size(size: usize) -> bool {
        size < Self::MinimumAllocationSize.get()
    }

    #[inline(always)]
    pub(crate) fn size_is_greater_than_minimum_allocation_size(size: usize) -> bool {
        size >= Self::MinimumAllocationSize.get()
    }

    #[inline(always)]
    pub(crate) fn size_exceeds_maximum_allocation_size(non_zero_size: NonZeroUsize) -> bool {
        non_zero_size > Self::MaximumAllocationSize
    }

    #[inline(always)]
    pub(crate) fn alignment_exceeds_maximum_alignment(
        non_zero_power_of_two_alignment: NonZeroUsize,
    ) -> bool {
        non_zero_power_of_two_alignment > Self::MaximumAlignment
    }

    #[inline(always)]
    pub(crate) fn floor_size_to_minimum(unfloored_non_zero_size: NonZeroUsize) -> NonZeroUsize {
        max(unfloored_non_zero_size, Self::MinimumAllocationSize)
    }

    #[inline(always)]
    pub(crate) fn floor_alignment_to_minimum(
        unfloored_non_zero_power_of_two_alignment: NonZeroUsize,
    ) -> NonZeroUsize {
        max(
            unfloored_non_zero_power_of_two_alignment,
            Self::MinimumAlignment,
        )
    }

    #[inline(always)]
    pub(crate) fn binary_search_tree_for(
        &self,
        binary_search_tree_index: usize,
    ) -> &mut BinarySearchTreeWithCachedKnowledgeOfFirstChild {
        debug_assert!(
            binary_search_tree_index < Self::NumberOfBinarySearchTrees,
            "binary_search_tree_index `{}` is too large",
            binary_search_tree_index
        );

        unsafe { self.binary_search_trees_of_free_blocks_sorted_by_ascending_memory_address_and_indexed_by_power_of_two_exponent_less_smallest_power_of_two.get_unchecked(binary_search_tree_index) }.get().mutable_reference()
    }

    #[inline(always)]
    pub(crate) fn smallest_power_of_two_difference(difference: usize) -> NonZeroUsize {
        debug_assert!(
            Self::size_is_greater_than_minimum_allocation_size(difference),
            "difference `{}` is too small to be a block"
        );

        (1 << difference.trailing_zeros()).non_zero()
    }

    #[allow(dead_code)]
    #[inline(always)]
    pub(crate) fn largest_power_of_two_difference(difference: usize) -> NonZeroUsize {
        debug_assert!(
            Self::size_is_greater_than_minimum_allocation_size(difference),
            "difference `{}` is too small to be a block"
        );

        const BitsInAByte: usize = 8;
        const BitsInAnUsize: usize = size_of::<usize>() * BitsInAByte;
        const ZeroBased: usize = BitsInAnUsize - 1;

        let shift = ZeroBased - difference.leading_zeros() as usize;

        (1 << shift).non_zero()
    }
}
