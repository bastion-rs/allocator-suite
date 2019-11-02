#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct SlotIndex(pub usize);

impl SlotIndex
{
    pub const IsFullyAllocatedNextAvailableSlotIndexSentinel: Self = Self(::std::usize::MAX);

    #[inline(always)]
    pub fn is_fully_allocated(self) -> bool
    {
        self == Self::IsFullyAllocatedNextAvailableSlotIndexSentinel
    }

    #[inline(always)]
    pub(crate) fn increment(&mut self)
    {
        self.0 += 1
    }
}
