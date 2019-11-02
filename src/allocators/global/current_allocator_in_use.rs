/// Records which allocator is currently in use for `Global` allocations.
///
/// This does not affect reallocations or deallocations in any way.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum CurrentAllocatorInUse
{
	/// A coroutine local allocator.
	CoroutineLocal,

	/// A thread local allocator.
	ThreadLocal,

	/// A global allocator.
	Global,
}
