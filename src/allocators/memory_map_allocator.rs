use super::allocators::allocator::Allocator;
use super::memory_address::MemoryAddress;
use super::memory_sources::mmap::memory_map_source::MemoryMapSource;
use std::alloc::AllocErr;
use std::num::NonZeroUsize;

use std::fmt::Debug;

/// This NUMA-aware allocator allocates memory-mapped data, optionally using NUMA policy to allocate on a memory node closest to the current thread.
///
/// It is slow and uses system calls.
///
/// On non-Linux systems except for NetBSD, this allocator is extremely inefficient when reallocating.
///
/// On Android, DragonFlyBSD, FreeBSD, Linux and OpenBSD mappings are omitted from core dumps for data privacy.
///
/// When dropped, any memory allocated with this allocator is ***NOT*** freed.
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct MemoryMapAllocator(MemoryMapSource);

impl Allocator for MemoryMapAllocator {
    #[inline(always)]
    fn allocate(
        &self,
        non_zero_size: NonZeroUsize,
        non_zero_power_of_two_alignment: NonZeroUsize,
    ) -> Result<MemoryAddress, AllocErr> {
        const AssumedPageSize: usize = 4096;

        if unlikely!(non_zero_power_of_two_alignment.get() > AssumedPageSize) {
            return Err(AllocErr);
        }

        self.0.mmap_memory(non_zero_size.get())
    }

    #[inline(always)]
    fn deallocate(
        &self,
        non_zero_size: NonZeroUsize,
        _non_zero_power_of_two_alignment: NonZeroUsize,
        current_memory: MemoryAddress,
    ) {
        MemoryMapSource::munmap_memory(current_memory, non_zero_size.get())
    }

    #[inline(always)]
    fn growing_reallocate(
        &self,
        non_zero_new_size: NonZeroUsize,
        _non_zero_power_of_two_alignment: NonZeroUsize,
        non_zero_current_size: NonZeroUsize,
        current_memory: MemoryAddress,
    ) -> Result<MemoryAddress, AllocErr> {
        self.0.mremap_memory(
            current_memory,
            non_zero_current_size.get(),
            non_zero_new_size.get(),
        )
    }

    #[inline(always)]
    fn shrinking_reallocate(
        &self,
        non_zero_new_size: NonZeroUsize,
        _non_zero_power_of_two_alignment: NonZeroUsize,
        non_zero_current_size: NonZeroUsize,
        current_memory: MemoryAddress,
    ) -> Result<MemoryAddress, AllocErr> {
        self.0.mremap_memory(
            current_memory,
            non_zero_current_size.get(),
            non_zero_new_size.get(),
        )
    }
}
