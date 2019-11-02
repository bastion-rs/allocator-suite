/// Request that an allocation uses huge pages.
///
/// The allocation being requested has to be aligned to the size requested, viz it is best to only make allocations that use the configured huge page size.
///
/// On x86-64, this should be `2Mb` or `1Gb`.
///
/// Currently such requests assume that transparent huge pages are in effect.
///
/// On operating systems other than Android and Linux, huge page size has no effect.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[repr(i32)]
pub enum HugePageSize
{
    /// Regular, non-huge-page.
    None = 0,

    /// Equivalent to `MAP_HUGETLB`.
    #[cfg(any(target_os = "android", target_os = "linux"))] Default = MAP_HUGETLB | 0 << Self::MAP_HUGE_SHIFT,

    /// Equivalent to `MAP_HUGETLB | MAP_HUGE_64KB`.
    ///
    /// Suitable for ?
    #[cfg(any(target_os = "android", target_os = "linux"))] _64Kb = MAP_HUGETLB | 16 << Self::MAP_HUGE_SHIFT,

    /// Equivalent to `MAP_HUGETLB | MAP_HUGE_512KB`.
    ///
    /// Suitable for ?
    #[cfg(any(target_os = "android", target_os = "linux"))] _512Kb = MAP_HUGETLB | 19 << Self::MAP_HUGE_SHIFT,

    /// Equivalent to `MAP_HUGETLB | MAP_HUGE_1Mb`.
    ///
    /// Suitable for ?
    #[cfg(any(target_os = "android", target_os = "linux"))] _1Mb = MAP_HUGETLB | 20 << Self::MAP_HUGE_SHIFT,

    /// Equivalent to `MAP_HUGETLB | MAP_HUGE_2MB`.
    ///
    /// Suitable for x86-64.
    #[cfg(any(target_os = "android", target_os = "linux"))] _2Mb = MAP_HUGETLB | 21 << Self::MAP_HUGE_SHIFT,

    /// Equivalent to `MAP_HUGETLB | MAP_HUGE_8MB`.
    ///
    /// Suitable for ?
    #[cfg(any(target_os = "android", target_os = "linux"))] _8Mb = MAP_HUGETLB | 23 << Self::MAP_HUGE_SHIFT,

    /// Equivalent to `MAP_HUGETLB | MAP_HUGE_16MB`.
    ///
    /// Suitable for ?
    #[cfg(any(target_os = "android", target_os = "linux"))] _16Mb = MAP_HUGETLB | 24 << Self::MAP_HUGE_SHIFT,

    /// Equivalent to `MAP_HUGETLB | MAP_HUGE_32MB`.
    ///
    /// Suitable for ?
    #[cfg(any(target_os = "android", target_os = "linux"))] _32Mb = MAP_HUGETLB | 25 << Self::MAP_HUGE_SHIFT,

    /// Equivalent to `MAP_HUGETLB | MAP_HUGE_256MB`.
    ///
    /// Suitable for ?
    #[cfg(any(target_os = "android", target_os = "linux"))] _256Mb = MAP_HUGETLB | 28 << Self::MAP_HUGE_SHIFT,

    /// Equivalent to `MAP_HUGETLB | MAP_HUGE_512MB`.
    ///
    /// Suitable for ?
    #[cfg(any(target_os = "android", target_os = "linux"))] _512Mb = MAP_HUGETLB | 29 << Self::MAP_HUGE_SHIFT,

    /// Equivalent to `MAP_HUGETLB | MAP_HUGE_1GB`.
    ///
    /// Suitable for x86-64.
    #[cfg(any(target_os = "android", target_os = "linux"))] _1Gb = MAP_HUGETLB | 30 << Self::MAP_HUGE_SHIFT,

    /// Equivalent to `MAP_HUGETLB | MAP_HUGE_2GB`.
    ///
    /// Suitable for ?
    #[cfg(any(target_os = "android", target_os = "linux"))] _2Gb = MAP_HUGETLB | 31 << Self::MAP_HUGE_SHIFT,

    /// Equivalent to `MAP_HUGETLB | MAP_HUGE_16GB`.
    ///
    /// Suitable for PowerPC.
    #[cfg(any(target_os = "android", target_os = "linux"))] _16Gb = MAP_HUGETLB | 34 << Self::MAP_HUGE_SHIFT,
}

impl Default for HugePageSize
{
    #[inline(always)]
    fn default() -> Self
    {
        HugePageSize::None
    }
}

impl HugePageSize
{
    #[cfg(any(target_os = "android", target_os = "linux"))] const MAP_HUGE_SHIFT: i32 = 26;
}
