use crate::allocators::global::current_allocator_in_use::CurrentAllocatorInUse;
use crate::allocators::global::local_allocator::LocalAllocator;

#[doc(hidden)]
#[allow(dead_code)]
pub struct PerThreadState<
    CoroutineLocalAllocator: LocalAllocator,
    ThreadLocalAllocator: LocalAllocator,
> {
    pub current_allocator_in_use: CurrentAllocatorInUse,
    pub coroutine_local_allocator: Option<CoroutineLocalAllocator>,
    pub thread_local_allocator: Option<ThreadLocalAllocator>,
}

impl<CoroutineLocalAllocator: LocalAllocator, ThreadLocalAllocator: LocalAllocator>
    PerThreadState<CoroutineLocalAllocator, ThreadLocalAllocator>
{
    #[doc(hidden)]
    #[inline(always)]
    pub const fn empty() -> Self {
        Self {
            current_allocator_in_use: CurrentAllocatorInUse::Global,
            coroutine_local_allocator: None,
            thread_local_allocator: None,
        }
    }
}
