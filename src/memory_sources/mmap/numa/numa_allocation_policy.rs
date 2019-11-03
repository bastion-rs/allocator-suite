use crate::memory_sources::mmap::numa::numa_node_bit_set::NumaNodeBitSet;

/// Defaults to `Default`.
///
/// Ignored on operating systems other than Android and Linux.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum NumaAllocationPolicy {
    /// `MPOL_DEFAULT`.
    ///
    /// This mode requests that any non-default policy be removed, restoring default behavior.
    ///
    /// When applied to a range of memory this means to use the thread memory policy, which may have been set with the syscall `set_mempolicy`.
    ///
    /// If the mode of the thread memory policy is also `NumaNodePolicy::Default`, then the system-wide default policy will be used.
    /// The system-wide default policy allocates pages on the node of the CPU that triggers the allocation.
    Default,

    /// `MPOL_PREFERRED`.
    ///
    /// This mode sets the preferred node for allocation.
    ///
    /// The kernel will try to allocate pages from this node first and fall back to other nodes if the preferred node is low on free memory.
    ///
    /// The first node in the bit set will be selected as the preferred node.
    /// If the NUMA node bit set is empty, then the memory is allocated on the node of the CPU that triggered the allocation.
    Preferred(NumaNodeBitSet),

    /// `MPOL_BIND`.
    ///
    /// This mode specifies a strict policy that restricts memory allocation to the nodes specified in nodemask.
    ///
    /// Page allocations will come from the node in the NUMA node node bit set with sufficient free memory that is closest to the node where the allocation takes place.
    /// Pages will not be allocated from any node not specified in the NUMA node node bit set.
    Bind(NumaNodeBitSet),

    /// `MPOL_INTERLEAVE`.
    ///
    /// This mode specifies that page allocations be interleaved across the set of nodes specified in the NUMA node bit set.
    ///
    /// This optimizes for bandwidth instead of latency by spreading out pages and memory accesses to those pages across multiple nodes.
    /// To be effective the memory area should be fairly large, at least 1 MB or bigger with a fairly uniform access pattern.
    /// Accesses to a single page of the area will still be limited to the memory bandwidth of a single node.
    Interleave(NumaNodeBitSet),

    /// `MPOL_LOCAL`.
    ///
    /// This mode specifies "local allocation"; the memory is allocated on the node of the CPU that triggered the allocation (the "local node").
    ///
    /// If the "local node" is low on free memory, the kernel will try to allocate memory from other nodes.
    /// The kernel will allocate memory from the "local node"  whenever memory for this node is available.
    /// If the "local node" is not allowed by the thread's current cpuset context, the kernel will try to allocate memory from other nodes.
    /// The kernel will allocate memory from the "local node" whenever it becomes allowed by the thread's current cpuset context.
    ///
    /// Since Linux 3.8.
    Local,
}

impl Default for NumaAllocationPolicy {
    #[inline(always)]
    fn default() -> Self {
        NumaAllocationPolicy::Default
    }
}

impl NumaAllocationPolicy {
    #[cfg(any(target_os = "android", target_os = "linux"))]
    #[inline(always)]
    pub(crate) fn values(&self) -> (i32, (i32, Option<usize>, usize)) {
        use self::NumaAllocationPolicy::*;

        match *self {
            Default => (0, NumaNodeBitSet::NO_MODE_FLAGS_NODEMASK_MAXNODE),

            Preferred(ref numa_node_bit_set) => (1, numa_node_bit_set.mask_and_size()),

            Bind(ref numa_node_bit_set) => (2, numa_node_bit_set.mask_and_size()),

            Interleave(ref numa_node_bit_set) => (3, numa_node_bit_set.mask_and_size()),

            Local => (4, NumaNodeBitSet::NO_MODE_FLAGS_NODEMASK_MAXNODE),
        }
    }
}
