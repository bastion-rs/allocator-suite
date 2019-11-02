use crate::memory_sources::mmap::numa::prelude::*;
use crate::memory_sources::memory_source::MemorySource;
use crate::memory_address::MemoryAddress;
use std::alloc::AllocErr;
use crate::memory_sources::mmap::prelude::*;
use std::ptr::null_mut;
#[cfg(unix)] use ::libc::*;
use crate::extensions::prelude::*;
use std::num::NonZeroUsize;


/// This NUMA-aware memory source allocates memory-mapped data, optionally using NUMA policy to allocate on a memory node closest to the current thread.
///
/// It is slow and uses system calls.
///
/// On Android, DragonFlyBSD, FreeBSD, Linux and OpenBSD mappings are omitted from core dumps for data privacy.
///
/// When dropped, any memory obtained with this allocator is ***NOT*** freed.
///
/// However, it is appropriate as a 'backing store' for other memory sources.
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct MemoryMapSource
{
    map_flags: i32,

    #[cfg(not(any(target_os = "android", target_os = "netbsd", target_os = "linux")))] lock: bool,

    #[cfg(any(target_os = "android", target_os = "linux"))] madvise_flags: i32,

    #[cfg(any(target_os = "android", target_os = "linux"))] numa_settings: Option<NumaSettings>,
}

impl Default for MemoryMapSource
{
    #[inline(always)]
    fn default() -> Self
    {
        Self::new(true, true, true, false, HugePageSize::default(), None)
    }
}

impl MemorySource for MemoryMapSource
{
    #[inline(always)]
    fn obtain(&self, non_zero_size: NonZeroUsize) -> Result<MemoryAddress, AllocErr>
    {
        self.mmap_memory(non_zero_size.get())
    }

    #[inline(always)]
    fn release(&self, non_zero_size: NonZeroUsize, current_memory: MemoryAddress)
    {
        Self::munmap_memory(current_memory, non_zero_size.get())
    }
}

impl MemoryMapSource
{
    /// Create a new instance.
    ///
    /// * `lock`: Should allocated memory be locked (through a process equivalent to `mlock()`), thereby making out-of-memory fail fast. This setting will cause failures if `rlimit()` has not been increased.
    /// * `prefault`: Should allocated memory be pre-faulted, ie all pages loaded and made resident in RAM when allocation occurs? This slows down allocation but make subsequent accesses faster. Only on Android, FreeBSD and Linux.
    /// * `do_not_reserve_swap_space`: Do not reserve swap space for the mapping. Only on Android, Linux and NetBSD.
    /// * `allocate_within_first_32Gb`: Useful for stacks and creating executable code. Only on Android, FreeBSD and Linux on 64-bit CPUs.
    /// * `huge_page_size`: Huge page size to use with Transparent Huge Pages (THP). On operating systems other than Android and Linux, specifying a huge page size has no effect.
    /// * `numa_settings`: NUMA policy settings for optimizing memory allocations to the nearest node. On operating systems other than Android and Linux, specifying a value has no effect.
    #[allow(unused_variables)]
    #[inline(always)]
    pub fn new(lock: bool, prefault: bool, do_not_reserve_swap_space: bool, allocate_within_first_32Gb: bool, huge_page_size: HugePageSize, numa_settings: Option<NumaSettings>) -> Self
    {
        Self
            {
                map_flags: Self::map_flags(lock, prefault, do_not_reserve_swap_space, allocate_within_first_32Gb, huge_page_size),
                #[cfg(not(any(target_os = "android", target_os = "netbsd", target_os = "linux")))] lock,
                #[cfg(any(target_os = "android", target_os = "linux"))] madvise_flags: Self::madvise_flags(huge_page_size),
                #[cfg(any(target_os = "android", target_os = "linux"))] numa_settings,
            }
    }

    /// `size` is rounded up to system page size.
    #[inline(always)]
    pub(crate) fn mmap_memory(&self, size: usize) -> Result<MemoryAddress, AllocErr>
    {
        const UnusedFileDescriptor: i32 = -1;
        const NoOffset: i64 = 0;

        let result = unsafe { mmap(null_mut(), size, PROT_READ | PROT_WRITE, self.map_flags, UnusedFileDescriptor, NoOffset) };
        if unlikely!(result == MAP_FAILED)
        {
            Err(AllocErr)
        }
        else
        {
            #[cfg(any(target_os = "android", target_os = "linux"))] self.madvise_memory(result, size)?;

            #[cfg(any(target_os = "android", target_os = "linux"))] self.numa_memory(result, size)?;

            #[cfg(not(any(target_os = "android", target_os = "netbsd", target_os = "linux")))] self.mlock_memory(result, size)?;

            Ok(Self::cast_address(result))
        }
    }

    #[cfg(any(target_os = "android", target_os = "linux"))]
    #[inline(always)]
    fn madvise_memory(&self, address: *mut c_void, size: usize) -> Result<(), AllocErr>
    {
        let result = unsafe { madvise(address, size, self.madvise_flags) };
        if likely!(result == 0)
        {
        }
        else if likely!(result == -1)
        {
            Self::munmap_memory(Self::cast_address(address), size);
            return Err(AllocErr)
        }
        else
        {
            unreachable!()
        }
        Ok(())
    }

    #[cfg(any(target_os = "android", target_os = "linux"))]
    #[inline(always)]
    fn numa_memory(&self, address: *mut c_void, size: usize) -> Result<(), AllocErr>
    {
        match self.numa_settings
            {
                None => Ok(()),

                Some(ref numa_settings) =>
                    {
                        let outcome = numa_settings.post_allocate(address, size);
                        if unlikely!(outcome.is_err())
                        {
                            Self::munmap_memory(Self::cast_address(address), size);
                            return Err(AllocErr)
                        }
                        Ok(())
                    }
            }
    }

    #[cfg(not(any(target_os = "android", target_os = "netbsd", target_os = "linux")))]
    #[inline(always)]
    fn mlock_memory(&self, address: *mut c_void, size: usize) -> Result<(), AllocErr>
    {
        if self.lock
        {
            let result = unsafe { mlock(address, size) };
            if likely!(result == 0)
            {
            }
            else if likely!(result == -1)
            {
                Self::munmap_memory(Self::cast_address(address), size);
                return Err(AllocErr)
            }
            else
            {
                unreachable!()
            }
        }
        Ok(())
    }

    /// `size` is rounded up to system page size.
    #[cfg(any(target_os = "android", target_os = "linux", target_os = "netbsd"))]
    #[inline(always)]
    pub(crate) fn mremap_memory(&self, memory_address: MemoryAddress, old_size: usize, new_size: usize) -> Result<MemoryAddress, AllocErr>
    {
        #[cfg(target_os = "netbsd")] const MREMAP_MAYMOVE: i32 = 0;

        let result = unsafe { mremap(memory_address.as_ptr() as *mut _, old_size, new_size, MREMAP_MAYMOVE) };
        if unlikely!(result == MAP_FAILED)
        {
            Err(AllocErr)
        }
        else
        {
            Ok(Self::cast_address(result))
        }
    }

    #[cfg(not(any(target_os = "android", target_os = "linux", target_os = "netbsd")))]
    #[inline(always)]
    pub(crate) fn mremap_memory(&self, memory_address: MemoryAddress, old_size: usize, new_size: usize) -> Result<MemoryAddress, AllocErr>
    {
        let new_memory_address = self.mmap_memory(new_size)?;
        unsafe { new_memory_address.as_ptr().copy_from_nonoverlapping(memory_address.as_ptr() as *const _, old_size) };
        Self::munmap_memory(memory_address, old_size);
        Ok(new_memory_address)
    }

    /// `size` is rounded up to system page size.
    #[inline(always)]
    pub(crate) fn munmap_memory(memory_address: MemoryAddress, size: usize)
    {
        unsafe { munmap(memory_address.as_ptr() as *mut _, size) };
    }

    #[inline(always)]
    fn cast_address(address: *mut c_void) -> MemoryAddress
    {
        address.non_null().cast::<u8>()
    }

    #[allow(unused_variables)]
    #[inline(always)]
    fn map_flags(lock: bool, prefault: bool, do_not_reserve_swap_space: bool, allocate_within_first_32Gb: bool, huge_page_size: HugePageSize) -> i32
    {
        #[cfg(target_os = "netbsd")] const MAP_ANONYMOUS: i32 = MAP_ANON;

        #[cfg(all(target_os = "dragonfly", target_os = "freebsd", target_os = "openbsd"))] const OmitFromCoreDumps: i32 = MAP_NOCORE;
        #[cfg(not(any(target_os = "dragonfly", target_os = "freebsd", target_os = "openbsd")))] const OmitFromCoreDumps: i32 = 0;

        #[cfg(any(target_os = "android", target_os = "linux"))] const Locked: i32 = MAP_LOCKED;
        #[cfg(target_os = "netbsd")] const Locked: i32 = MAP_WIRED;
        #[cfg(not(any(target_os = "android", target_os = "netbsd", target_os = "linux")))] const Locked: i32 = 0;

        #[cfg(any(target_os = "android", target_os = "linux"))] const Prefault: i32 = MAP_POPULATE;
        #[cfg(target_os = "freebsd")] const Prefault: i32 = MAP_PREFAULT_READ;
        #[cfg(not(any(target_os = "android", target_os = "freebsd", target_os = "linux")))] const Prefault: i32 = 0;

        #[cfg(any(target_os = "android", target_os = "linux", target_os = "netbsd"))] const DoNotReserveSwapSpace: i32 = MAP_NORESERVE;
        #[cfg(not(any(target_os = "android", target_os = "linux", target_os = "netbsd")))] const DoNotReserveSwapSpace: i32 = 0;

        #[cfg(all(target_pointer_width = "64", any(target_os = "android", target_os = "freebsd", target_os = "linux")))] const AllocateWithnFirst32Gb: i32 = MAP_32BIT;
        #[cfg(not(all(target_pointer_width = "64", any(target_os = "android", target_os = "freebsd", target_os = "linux"))))] const AllocateWithnFirst32Gb: i32 = 0;

        let map_flags: i32 = MAP_PRIVATE | MAP_ANONYMOUS | OmitFromCoreDumps;

        let map_flags = if lock
        {
            map_flags | Locked
        }
        else
        {
            map_flags
        };

        let map_flags = if prefault
        {
            map_flags | Prefault
        }
        else
        {
            map_flags
        };

        let map_flags = if do_not_reserve_swap_space
        {
            map_flags | DoNotReserveSwapSpace
        }
        else
        {
            map_flags
        };

        let map_flags = if allocate_within_first_32Gb
        {
            map_flags | AllocateWithnFirst32Gb
        }
        else
        {
            map_flags
        };

        if cfg!(any(target_os = "android", target_os = "linux"))
        {
            map_flags | (huge_page_size as i32)
        }
        else
        {
            map_flags
        }
    }

    #[cfg(any(target_os = "android", target_os = "linux"))]
    #[inline(always)]
    fn madvise_flags(huge_page_size: HugePageSize) -> i32
    {
        const madvise_flags: i32 = MADV_DONTDUMP;

        if huge_page_size != HugePageSize::None
        {
            madvise_flags | MADV_HUGEPAGE
        }
        else
        {
            madvise_flags
        }
    }
}

