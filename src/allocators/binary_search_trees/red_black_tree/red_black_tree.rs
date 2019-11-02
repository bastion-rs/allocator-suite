use crate::allocators::binary_search_trees::red_black_tree::node_pointer::NodePointer;
use crate::allocators::binary_search_trees::red_black_tree::red_black_tree_double_ended_iterator::RedBlackTreeDoubleEndedIterator;
use crate::memory_address::MemoryAddress;
use std::collections::Bound;
use crate::allocators::binary_search_trees::red_black_tree::color::Color;
use std::ops::Bound::*;
use crate::extensions::non_null_u8_node_pointer::NonNullU8NodePointer;
use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct RedBlackTree
{
	root: NodePointer,
}

impl Default for RedBlackTree
{
	#[inline(always)]
	fn default() -> Self
	{
		Self::new()
	}
}

impl RedBlackTree
{
	/// Creates an empty `RedBlackTree`.
	#[inline(always)]
	pub(crate) const fn new() -> Self
	{
		Self
		{
			root: NodePointer::null(),
		}
	}

	/// Returns `true` if the tree is empty.
	#[inline(always)]
	pub(crate) fn is_empty(&self) -> bool
	{
		self.root.is_null()
	}

	#[inline(always)]
	pub(crate) fn has_blocks(&self) -> bool
	{
		self.root.is_not_null()
	}

	#[inline(always)]
	pub(crate) fn first_child(&self) -> NodePointer
	{
		self.root.first_child()
	}

	#[inline(always)]
	pub(crate) fn remove_node_pointer(&mut self, node_pointer: NodePointer)
	{
		node_pointer.remove(&mut self.root)
	}

	#[inline(always)]
	pub(crate) fn insert_memory_address(&mut self, value: MemoryAddress) -> NodePointer
	{
		let new = self.reset_node(value);

		if unlikely!(self.is_empty())
		{
			self.insert_root(new);
		}
		else
		{
			let key = value;
			let mut tree = self.root;
			loop
			{
				if key < tree.key()
				{
					let left = tree.left();
					if unlikely!(left.is_null())
					{
						tree.insert_left(new, &mut self.root);
						break
					}
					else
					{
						tree = left
					}
				}
				else
				{
					let right = tree.right();
					if unlikely!(right.is_null())
					{
						tree.insert_right(new, &mut self.root);
						break
					}
					else
					{
						tree = right
					}
				}
			}
		}
		new
	}

	/// Gets an iterator over the objects in the `RedBlackTree`, in ascending key order.
	///
	/// Creating the iterator itself is not efficient.
	#[allow(dead_code)]
	#[inline(always)]
	pub(crate) fn double_ended_iterate<'a>(&'a self) -> RedBlackTreeDoubleEndedIterator<'a>
	{
		if self.is_empty()
		{
			RedBlackTreeDoubleEndedIterator
			{
				head: NodePointer::default(),
				tail: NodePointer::default(),
				tree: self,
			}
		}
		else
		{
			RedBlackTreeDoubleEndedIterator
			{
				head: self.root.first_child(),
				tail: self.root.last_child(),
				tree: self,
			}
		}
	}

	/// Constructs a double-ended iterator over a sub-range of elements in the tree, starting at `minimum`, and ending at `maximum`.
	///
	/// If `minimum` is `Unbounded`, then it will be treated as "negative infinity", and if `maximum` is `Unbounded`, then it will be treated as "positive infinity".
	/// Thus `range(Unbounded, Unbounded)` will yield the whole collection, and so is a more expensive choise than using `double_ended_iterate()`.
	///
	/// If `maximum` is less than `minimum` then an empty iterator is returned.
	/// If `maximum` or `minimum` is not found then a then an empty iterator is returned.
	///
	/// Creating the iterator itself is not efficient.
	#[allow(dead_code)]
	#[inline(always)]
	pub(crate) fn double_ended_range_iterate<'a>(&'a self, minimum: Bound<MemoryAddress>, maximum: Bound<MemoryAddress>) -> RedBlackTreeDoubleEndedIterator<'a>
	{
		let lower = self.lower_bound(minimum);
		let upper = self.upper_bound(maximum);
		if likely!(lower.is_not_null() && upper.is_not_null())
		{
			let lower_key = lower.key();
			let upper_key = upper.key();

			if upper_key >= lower_key
			{
				RedBlackTreeDoubleEndedIterator
				{
					head: lower,
					tail: upper,
					tree: self,
				}
			}
			else
			{
				self.empty_iterator()
			}
		}
		else
		{
			self.empty_iterator()
		}
	}

	/// Returns a `NodePointer` pointing to an element with the given key.
	///
	/// If no such element is found then a null `NodePointer` is returned.
	#[inline(always)]
	pub(crate) fn find(&self, key: MemoryAddress) -> NodePointer
	{
		let mut tree = self.root;
		while tree.is_not_null()
		{
			match key.cmp(&tree.key())
			{
				Ordering::Less => tree = tree.left(),
                Ordering::Equal => return tree,
                Ordering::Greater => tree = tree.right(),
			}
		}

		NodePointer::default()
	}

	/// Returns a `NodePointer` pointing to the first element whose key is above the given bound.
	///
	/// If no such element is found then a null `NodePointer` is returned.
	#[allow(dead_code)]
	#[inline(always)]
	pub(crate) fn lower_bound(&self, bound: Bound<MemoryAddress>) -> NodePointer
	{
		let mut tree = self.root;
		let mut result = NodePointer::default();
		while tree.is_not_null()
		{
			let cond = match bound
			{
				Unbounded => true,

				Included(key) => key <= tree.key(),

				Excluded(key) => key < tree.key(),
			};

			if cond
			{
				result = tree;
				tree = tree.left();
			}
			else
			{
				tree = tree.right();
			}
		}
		result
	}

	/// Returns a `NodePointer` pointing to the last element whose key is below the given bound.
	///
	/// If no such element is found then a null `NodePointer` is returned.
	#[allow(dead_code)]
	#[inline(always)]
	pub(crate) fn upper_bound(&self, bound: Bound<MemoryAddress>) -> NodePointer
	{
		let mut tree = self.root;
		let mut result = NodePointer::default();
		while tree.is_not_null()
		{
			let cond = match bound
			{
				Unbounded => false,

				Included(key) => key < tree.key(),

				Excluded(key) => key <= tree.key(),
			};

			if cond
			{
				tree = tree.left();
			}
			else
			{
				result = tree;
				tree = tree.right();
			}
		}
		result
	}

	#[inline(always)]
	fn empty_iterator<'a>(&'a self) -> RedBlackTreeDoubleEndedIterator<'a>
	{
		RedBlackTreeDoubleEndedIterator
		{
			head: NodePointer::default(),
			tail: NodePointer::default(),
			tree: self,
		}
	}

	#[inline(always)]
	fn reset_node(&self, value: MemoryAddress) -> NodePointer
	{
		let node_pointer = value.node_pointer();
		node_pointer.reset();
		node_pointer
	}

	#[inline(always)]
	fn insert_root(&mut self, node: NodePointer)
	{
		node.set_parent_and_color(NodePointer::default(), Color::Black);
		node.set_left(NodePointer::default());
		node.set_right(NodePointer::default());
		self.root = node;
	}
}
