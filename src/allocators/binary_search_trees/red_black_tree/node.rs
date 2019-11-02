use std::cell::Cell;
use super::allocators::binary_search_trees::red_black_tree::parent_and_color::ParentAndColor;
use super::allocators::binary_search_trees::red_black_tree::color::Color;
use crate::allocators::binary_search_trees::red_black_tree::node_pointer::NodePointer;

// TODO: Save memory be using compressed (32-bit) pointers.
#[repr(align(32))]
#[derive(Debug)]
pub(crate) struct Node
{
	left: Cell<NodePointer>,
	right: Cell<NodePointer>,
	parent_and_color: Cell<ParentAndColor>,
}

impl Node
{
	#[inline(always)]
	pub(crate) fn reset(&mut self)
	{
		self.left = Cell::default();
		self.right = Cell::default();
		self.parent_and_color = Cell::default();
	}

	#[inline(always)]
	pub(crate) fn parent(&self) -> NodePointer
	{
		self.parent_and_color().parent()
	}

	#[inline(always)]
	pub(crate) fn set_parent(&self, parent: NodePointer)
	{
		self.set_parent_and_color(parent, self.color())
	}

	#[inline(always)]
	pub(crate) fn color(&self) -> Color
	{
		self.parent_and_color().color()
	}

	#[inline(always)]
	pub(crate) fn set_color(&self, color: Color)
	{
		self.set_parent_and_color(self.parent(), color)
	}

	#[inline(always)]
	pub(crate) fn parent_and_color(&self) -> ParentAndColor
	{
		self.parent_and_color.get()
	}

	#[inline(always)]
	pub(crate) fn set_parent_and_color(&self, parent: NodePointer, color: Color)
	{
		self.parent_and_color.set(ParentAndColor::new(parent, color))
	}

	#[inline(always)]
	pub(crate) fn left(&self) -> NodePointer
	{
		self.left.get()
	}

	#[inline(always)]
	pub(crate) fn set_left(&self, left: NodePointer)
	{
		self.left.set(left);
	}

	#[inline(always)]
	pub(crate) fn right(&self) -> NodePointer
	{
		self.right.get()
	}

	#[inline(always)]
	pub(crate) fn set_right(&self, right: NodePointer)
	{
		self.right.set(right);
	}
}
