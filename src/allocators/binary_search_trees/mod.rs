//! There are various kinds of binary search tree available suitable for storing a free list of free blocks.
//!
//! Normally, these use nodes allocated from a heap, with fields for pointers to a key and a value.
//! As we know unused blocks of memory are free, we can re-use these as nodes.
//! We can then dispense with the value - it is the pointer to the node itself, that being the free block - and with the key (which is the same as the pointer to the node itself, too).
//! All binary search tree nodes need pointers to a lefthand (or lesser) node and righthand (or greater) node.
//! We can compress these pointers by using a `u32` relative offset which is scaled down by the minimum size of a free block (eg if a node had to be 8 bytes, the relative offset would be scaled by 8, giving a maximum relative offset of 4Gb x 8 => 32Gb).
//! The minimum size of a free block is dictated by the size of the fields required to represent a binary search tree node.
//! For effectiveness, a free block size must be a power of two.
//!
//! Of the types of tree we know of, the following are probably most suitable for allocating and deallocating free blocks:-
//!
//! * A red-black tree;
//! * A left-leaning red-black tree (Sedgwick);
//! * An AA (Arne Andersson) tree;
//! * An AVL (Adelson-Velsky and Landis) tree.
//! * Scapegoat tree.
//!
//! There are trade-offs in choosing one to use:-
//!
//! * Whilst AA tree ands AVL trees perform better generally for look-ups than Red-Black tree, they usually are worse for deletions and insertions;
//! * Deletions and insertions are a major part of the operations of free list (indeed, if splitting free blocks into smaller ones is at all common, they are the dominant operation);
//! * An AA tree requires an additional 4 - 8 bytes to hold an integer 'level`;
//! * A Red-Black tree requires an additional bit to hold a color combined with a `parent` pointer.


use super::*;


pub(crate) mod red_black_tree;


pub mod binary_search_trees_with_cached_knowledge_of_first_child;
pub mod binary_search_tree_with_cached_knowledge_of_first_child;
