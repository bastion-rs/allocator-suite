#[doc(hidden)]
#[macro_export]
macro_rules! global_alloc
{
	() =>
	{
		#[inline(always)]
		unsafe fn alloc(&self, layout: Layout) -> *mut u8
		{
			self.global_alloc_alloc(layout)
		}

		#[inline(always)]
		unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8
		{
			self.global_alloc_alloc_zeroed(layout)
		}

		#[inline(always)]
		unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout)
		{
			self.global_alloc_dealloc(ptr, layout)
		}

		#[inline(always)]
		unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8
		{
			self.global_alloc_realloc(ptr, layout, new_size)
		}
	}
}
