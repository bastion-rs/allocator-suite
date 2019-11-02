use crate::allocators::binary_search_trees::red_black_tree::prelude::*;
use crate::memory_address::MemoryAddress;
use crate::allocators::binary_search_trees::red_black_tree::red_black_tree::RedBlackTree;
use crate::allocators::binary_search_trees::red_black_tree::node_pointer::NodePointer;

/// An iterator over references to the items of a `RedBlackTree`.
///
/// Expensive to construct.
pub struct RedBlackTreeDoubleEndedIterator<'a> {
    pub(crate) head: NodePointer,
    pub(crate) tail: NodePointer,
    pub(crate) tree: &'a RedBlackTree,
}

impl<'a> Iterator for RedBlackTreeDoubleEndedIterator<'a> {
    type Item = MemoryAddress;

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        let head = self.head;

        if unlikely!(head.is_null()) {
            return None;
        }

        self.head = if head == self.tail {
            self.tail = NodePointer::default();

            NodePointer::default()
        } else {
            head.next()
        };

        Some(head.value())
    }
}

impl<'a> DoubleEndedIterator for RedBlackTreeDoubleEndedIterator<'a> {
    #[inline(always)]
    fn next_back(&mut self) -> Option<Self::Item> {
        let tail = self.tail;

        if unlikely!(tail.is_null()) {
            return None;
        }

        self.tail = if tail == self.head {
            self.head = NodePointer::default();

            NodePointer::default()
        } else {
            tail.previous()
        };

        Some(tail.value())
    }
}

impl<'a> Clone for RedBlackTreeDoubleEndedIterator<'a> {
    #[inline(always)]
    fn clone(&self) -> RedBlackTreeDoubleEndedIterator<'a> {
        Self {
            head: self.head,
            tail: self.tail,
            tree: self.tree,
        }
    }
}
