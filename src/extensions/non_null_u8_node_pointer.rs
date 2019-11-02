use crate::allocators::binary_search_trees::red_black_tree::node_pointer::NodePointer;
use crate::extensions::non_null_u8_ext::NonNullU8Ext;
use std::ptr::NonNull;

pub(crate) trait NonNullU8NodePointer: NonNullU8Ext {
    #[inline(always)]
    fn node_pointer(self) -> NodePointer {
        NodePointer::from_memory_address(self.to_non_null_u8())
    }
}

impl NonNullU8NodePointer for NonNull<u8> {}
