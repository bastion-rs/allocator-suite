use crate::allocators::binary_search_trees::red_black_tree::color::Color;
use crate::allocators::binary_search_trees::red_black_tree::node::Node;
use crate::allocators::binary_search_trees::red_black_tree::node_pointer::NodePointer;
use std::intrinsics::transmute;
use std::mem::align_of;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct ParentAndColor(usize);

impl Default for ParentAndColor {
    #[inline(always)]
    fn default() -> Self {
        Self::new(NodePointer::default(), Color::Red)
    }
}

impl ParentAndColor {
    const COLOR_BITMASK: usize = 0b1;

    const PARENT_BITMASK: usize = !Self::COLOR_BITMASK;

    #[inline(always)]
    pub(crate) fn new(parent: NodePointer, color: Color) -> Self {
        debug_assert!(align_of::<Node>() >= 2, "Node needs to be aligned to 2 bytes or more otherwise we can not set the color_bit using unused bits in the parent pointer");

        Self((parent.0 as usize & Self::PARENT_BITMASK) | color.color_bit())
    }

    #[inline(always)]
    pub(crate) fn parent(self) -> NodePointer {
        NodePointer((self.0 & Self::PARENT_BITMASK) as *const Node)
    }

    #[inline(always)]
    pub(crate) fn color(self) -> Color {
        unsafe { transmute(self.0 & Self::COLOR_BITMASK) }
    }
}
