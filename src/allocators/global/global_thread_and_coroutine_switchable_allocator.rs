use crate::allocators::global::current_allocator_in_use::CurrentAllocatorInUse;
use crate::allocators::global::local_allocator::LocalAllocator;
use crate::allocators::allocator::Allocator;
use std::alloc::{Alloc, GlobalAlloc};

/// A trait that all such allocators implement.
///
/// Create a new instance using the macro `global_thread_and_coroutine_switchable_allocator`.
pub trait SwitchableAllocator: Sync + GlobalAlloc + Alloc + Allocator {
    /// Type of the coroutine local allocator.
    type CoroutineLocalAllocator: LocalAllocator;

    /// Type of the thread local allocator.
    type ThreadLocalAllocator: LocalAllocator;

    /// Type of the global allocator.
    type GlobalAllocator: Allocator;

    /// Swaps the coroutine local allocator.
    ///
    /// Used before calling a coroutine.
    ///
    /// Used after calling a coroutine.
    fn replace_coroutine_local_allocator(
        &self,
        replacement: Option<Self::CoroutineLocalAllocator>,
    ) -> Option<Self::CoroutineLocalAllocator>;

    /// Initializes the thread local allocator.
    fn initialize_thread_local_allocator(&self, thread_local_allocator: Self::ThreadLocalAllocator);

    /// Drops the thread local allocator.
    ///
    /// Panics in debug if no thread local allocator has been initialized with `initialize_thread_local_allocator()`.
    ///
    /// Could be made hidden by using a destructor with `libc::pthread_key_create()` for an otherwise unused key.
    fn drop_thread_local_allocator(&self);

    /// Save the current allocator in use.
    fn save_current_allocator_in_use(&self) -> CurrentAllocatorInUse;

    /// Restore the current allocator in use.
    fn restore_current_allocator_in_use(&self, restore_to: CurrentAllocatorInUse);

    /// Replace the current allocator in use.
    #[inline(always)]
    fn replace_current_allocator_in_use(
        &self,
        replacement: CurrentAllocatorInUse,
    ) -> CurrentAllocatorInUse {
        let was = self.save_current_allocator_in_use();
        self.restore_current_allocator_in_use(replacement);
        was
    }

    /// Switch the current allocator in use to coroutine local and execute the callback; restore it after calling the callback unless a panic occurs.
    #[inline(always)]
    fn callback_with_coroutine_local_allocator<R>(&self, callback: impl FnOnce() -> R) -> R {
        self.callback_with_different_current_allocator(
            CurrentAllocatorInUse::CoroutineLocal,
            callback,
        )
    }

    /// Switch the current allocator in use to thread local and execute the callback; restore it after calling the callback unless a panic occurs.
    #[inline(always)]
    fn callback_with_thread_local_allocator<R>(&self, callback: impl FnOnce() -> R) -> R {
        self.callback_with_different_current_allocator(CurrentAllocatorInUse::ThreadLocal, callback)
    }

    /// Switch the current allocator in use to global and execute the callback; restore it after calling the callback unless a panic occurs.
    #[inline(always)]
    fn callback_with_global_allocator<R>(&self, callback: impl FnOnce() -> R) -> R {
        self.callback_with_different_current_allocator(CurrentAllocatorInUse::Global, callback)
    }

    /// Switch the current allocator in use and execute the callback; restore it after calling the callback unless a panic occurs.
    #[inline(always)]
    fn callback_with_different_current_allocator<R>(
        &self,
        different: CurrentAllocatorInUse,
        callback: impl FnOnce() -> R,
    ) -> R {
        let restore_to = self.save_current_allocator_in_use();
        self.restore_current_allocator_in_use(different);
        let result = callback();
        self.restore_current_allocator_in_use(restore_to);
        result
    }

    /// Obtain the current coroutine local allocator, if any.
    fn coroutine_local_allocator(&self) -> Option<&Self::CoroutineLocalAllocator>;

    /// Obtain the coroutine local allocator.
    ///
    /// Panics if no coroutine local allocator has been assigned with `replace_coroutine_local_allocator()`.
    #[inline(always)]
    fn coroutine_local_allocator_unchecked(&self) -> &Self::CoroutineLocalAllocator {
        self.coroutine_local_allocator().expect("Assign the coroutine local allocator first using `replace_coroutine_local_allocator()`")
    }

    /// Obtain the thread local allocator.
    ///
    /// None if no thread local allocator has been initialized with `initialize_thread_local_allocator()`.
    fn thread_local_allocator(&self) -> Option<&Self::ThreadLocalAllocator>;

    /// Obtain the thread local allocator.
    ///
    /// Panics if no thread local allocator has been initialized with `initialize_thread_local_allocator()`.
    #[inline(always)]
    fn thread_local_allocator_unchecked(&self) -> &Self::ThreadLocalAllocator {
        self.thread_local_allocator().expect("Initialize the thread local allocator first using `initialize_thread_local_allocator()`")
    }

    /// Obtain the global allocator.
    fn global_allocator(&self) -> &Self::GlobalAllocator;
}
