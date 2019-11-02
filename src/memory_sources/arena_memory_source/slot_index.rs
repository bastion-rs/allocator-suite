#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct SlotIndex(pub usize);

impl SlotIndex {
    pub const IS_FULLY_ALLOCATED_NEXT_AVAILABLE_SLOT_INDEX_SENTINEL: Self = Self(::std::usize::MAX);

    #[inline(always)]
    pub fn is_fully_allocated(self) -> bool {
        self == Self::IS_FULLY_ALLOCATED_NEXT_AVAILABLE_SLOT_INDEX_SENTINEL
    }

    #[inline(always)]
    pub(crate) fn increment(&mut self) {
        self.0 += 1
    }
}
