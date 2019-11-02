use crate::allocators::binary_search_trees::red_black_tree::prelude::*;
use crate::extensions::non_null_u8_ext::NonNullU8Ext;
use crate::extensions::non_null_u8_node_pointer::*;
use crate::extensions::non_zero_usize_ext::NonZeroUsizeExt;
use crate::extensions::usize_ext::UsizeExt;
use crate::memory_address::MemoryAddress;
use std::num::NonZeroUsize;
use crate::allocators::binary_search_trees::red_black_tree::node_pointer::NodePointer;
use crate::allocators::binary_search_trees::red_black_tree::red_black_tree::RedBlackTree;
use crate::allocators::binary_search_trees::red_black_tree::red_black_tree_double_ended_iterator::RedBlackTreeDoubleEndedIterator;

#[derive(Debug)]
pub struct BinarySearchTreeWithCachedKnowledgeOfFirstChild {
    tree: RedBlackTree,
    cached_first_child: NodePointer,
}

impl Default for BinarySearchTreeWithCachedKnowledgeOfFirstChild {
    fn default() -> Self {
        let tree = RedBlackTree::default();
        Self {
            cached_first_child: tree.first_child(),
            tree,
        }
    }
}

impl BinarySearchTreeWithCachedKnowledgeOfFirstChild {
    #[inline(always)]
    pub(crate) fn has_blocks(&self) -> bool {
        self.tree.has_blocks()
    }

    #[inline(always)]
    pub(crate) fn find(&self, key: MemoryAddress) -> NodePointer {
        self.tree.find(key)
    }

    #[inline(always)]
    pub(crate) fn blocks_to_coalesce(
        &mut self,
        inserted_node_pointer: NodePointer,
        difference: NonZeroUsize,
        block_size: NonZeroUsize,
        furthest_back_contiguous_with_inserted_node_pointer_memory_address: MemoryAddress,
        furthest_forward_contiguous_with_inserted_node_pointer_memory_address: MemoryAddress,
    ) -> (MemoryAddress, MemoryAddress) {
        let number_of_contiguous_blocks_excluding_inserted_node =
            difference.divide_power_of_two_by_power_of_two(block_size);

        let even_sic_total_number_of_contiguous_blocks_to_coalesce =
            number_of_contiguous_blocks_excluding_inserted_node.is_odd();

        if even_sic_total_number_of_contiguous_blocks_to_coalesce {
            (
                furthest_back_contiguous_with_inserted_node_pointer_memory_address,
                furthest_forward_contiguous_with_inserted_node_pointer_memory_address,
            )
        } else {
            let insert_node_pointer_memory_address = inserted_node_pointer.value();
            if unlikely!(
                furthest_forward_contiguous_with_inserted_node_pointer_memory_address
                    == insert_node_pointer_memory_address
            ) {
                (
                    furthest_back_contiguous_with_inserted_node_pointer_memory_address,
                    furthest_forward_contiguous_with_inserted_node_pointer_memory_address
                        .node_pointer()
                        .previous()
                        .value(),
                )
            } else if unlikely!(
                furthest_back_contiguous_with_inserted_node_pointer_memory_address
                    == insert_node_pointer_memory_address
            ) {
                let furthest_back_node_pointer =
                    furthest_back_contiguous_with_inserted_node_pointer_memory_address
                        .node_pointer();

                (
                    furthest_back_node_pointer.next().value(),
                    furthest_forward_contiguous_with_inserted_node_pointer_memory_address,
                )
            } else {
                let furthest_back_node_pointer = self.insert_memory_address(
                    furthest_back_contiguous_with_inserted_node_pointer_memory_address,
                );

                (
                    furthest_back_node_pointer.next().value(),
                    furthest_forward_contiguous_with_inserted_node_pointer_memory_address,
                )
            }
        }
    }

    #[inline(always)]
    pub(crate) fn remove_contiguous_blocks(
        &mut self,
        first_block_memory_address: MemoryAddress,
        last_block_memory_address: MemoryAddress,
        block_size: NonZeroUsize,
    ) {
        let mut to_remove_memory_address = first_block_memory_address;
        while {
            let to_remove_node_pointer = to_remove_memory_address.node_pointer();
            let is_cached_first_child = to_remove_node_pointer == self.cached_first_child();
            self.remove(to_remove_node_pointer, is_cached_first_child);

            to_remove_memory_address.add_assign_non_zero(block_size);
            likely!(to_remove_memory_address <= last_block_memory_address)
        } {}
    }

    #[inline(always)]
    pub(crate) fn remove(&mut self, node_pointer: NodePointer, is_cached_first_child: bool) {
        if unlikely!(is_cached_first_child) {
            self.update_cached_first_child(node_pointer.next());
        }

        self.tree.remove_node_pointer(node_pointer);
        self.debug_assert_cached_first_child_is_valid();
    }

    #[inline(always)]
    pub(crate) fn insert_memory_address(&mut self, memory_address: MemoryAddress) -> NodePointer {
        let cached_first_child = self.cached_first_child();

        if unlikely!(cached_first_child.is_null() || memory_address < cached_first_child.value()) {
            self.update_cached_first_child(memory_address.node_pointer())
        }

        self.tree.insert_memory_address(memory_address);
        self.debug_assert_cached_first_child_is_valid();
        memory_address.node_pointer()
    }

    #[inline(always)]
    pub(crate) fn double_ended_iterate<'a>(&'a self) -> RedBlackTreeDoubleEndedIterator<'a> {
        self.tree.double_ended_iterate()
    }

    #[inline(always)]
    pub(crate) fn cached_first_child(&self) -> NodePointer {
        self.cached_first_child
    }

    #[inline(always)]
    fn update_cached_first_child(&mut self, new_first_child_to_cache: NodePointer) {
        self.cached_first_child = new_first_child_to_cache
    }

    #[inline(always)]
    fn debug_assert_cached_first_child_is_valid(&self) {
        debug_assert_eq!(
            self.cached_first_child,
            self.tree.first_child(),
            "First child is not valid"
        );
    }
}
