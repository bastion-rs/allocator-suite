use std::cell::Cell;
use crate::memory_address::MemoryAddress;
use crate::extensions::prelude::*;
use std::num::NonZeroUsize;
use crate::memory_sources::arena_memory_source::slot_index::SlotIndex;
use crate::memory_sources::arena_memory_source::unsized_block::Unsized;

#[repr(C)]
pub struct UnallocatedBlock
{
    next_available_slot_index: Cell<SlotIndex>,
    _remainder: Unsized,
}

impl UnallocatedBlock
{
    #[inline(always)]
    pub(crate) fn initialize(&self, block_size: NonZeroUsize, block_initializer: &impl Fn(MemoryAddress, NonZeroUsize), slot_index: SlotIndex)
    {
        block_initializer((self as *const Self as *const u8).non_null(), block_size);
        self.set_unoccupied_next_available_slot_index(slot_index)
    }

    #[inline(always)]
    pub(crate) fn next_available_slot_index(&self) -> SlotIndex
    {
        self.next_available_slot_index.get()
    }

    #[inline(always)]
    pub(crate) fn set_unoccupied_next_available_slot_index(&self, slot_index: SlotIndex)
    {
        self.next_available_slot_index.set(slot_index)
    }

    #[inline(always)]
    pub(crate) fn from_memory_address<'a>(memory_address: MemoryAddress) -> &'a Self
    {
        unsafe { & * (memory_address.as_ptr() as *const Self) }
    }

    #[inline(always)]
    pub(crate) fn to_memory_address(&self) -> MemoryAddress
    {
        (self as *const Self as *const u8 as *mut u8).non_null()
    }
}
