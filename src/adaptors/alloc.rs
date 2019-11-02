#[doc(hidden)]
#[macro_export]
macro_rules! alloc
{
	() =>
	{
		#[inline(always)]
		unsafe fn alloc(&mut self, layout: Layout) -> Result<MemoryAddress, AllocErr>
		{
			self.Alloc_alloc(layout)
		}

		#[inline(always)]
		unsafe fn alloc_zeroed(&mut self, layout: Layout) -> Result<MemoryAddress, AllocErr>
		{
			self.Alloc_alloc_zeroed(layout)
		}

		#[inline(always)]
		unsafe fn dealloc(&mut self, ptr: MemoryAddress, layout: Layout)
		{
			self.Alloc_dealloc(ptr, layout)
		}

		#[inline(always)]
		unsafe fn realloc(&mut self, ptr: MemoryAddress, layout: Layout, new_size: usize) -> Result<MemoryAddress, AllocErr>
		{
			self.Alloc_realloc(ptr, layout, new_size)
		}

		#[inline(always)]
		unsafe fn alloc_excess(&mut self, layout: Layout) -> Result<Excess, AllocErr>
		{
			self.Alloc_alloc_excess(layout)
		}

		#[inline(always)]
		unsafe fn realloc_excess(&mut self, ptr: MemoryAddress, layout: Layout, new_size: usize) -> Result<Excess, AllocErr>
		{
			self.Alloc_realloc_excess(ptr, layout, new_size)
		}

		#[inline(always)]
		unsafe fn grow_in_place(&mut self, ptr: MemoryAddress, layout: Layout, new_size: usize) -> Result<(), CannotReallocInPlace>
		{
			self.Alloc_grow_in_place(ptr, layout, new_size)
		}

		#[inline(always)]
		unsafe fn shrink_in_place(&mut self, ptr: MemoryAddress, layout: Layout, new_size: usize) -> Result<(), CannotReallocInPlace>
		{
			self.Alloc_shrink_in_place(ptr, layout, new_size)
		}
	}
}
