use crate::memory_address::MemoryAddress;

/// Represents a memory range for which an allocator can allocate.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct MemoryRange {
    /// From (inclusive).
    pub from: MemoryAddress,

    /// To (exclusive).
    pub to: MemoryAddress,
}

impl MemoryRange {
    /// Create a new instance.
    #[inline(always)]
    pub const fn new(from: MemoryAddress, to: MemoryAddress) -> Self {
        Self { from, to }
    }

    #[inline(always)]
    pub(crate) fn contains(&self, from_memory_address: MemoryAddress) -> bool {
        from_memory_address >= self.from && from_memory_address < self.to
    }
}
