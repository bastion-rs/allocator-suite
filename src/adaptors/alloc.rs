#[doc(hidden)]
#[macro_export]
macro_rules! alloc_ref {
    () => {
        #[inline(always)]
        unsafe fn alloc(&mut self, layout: Layout) -> Result<MemoryAddress, AllocErr> {
            self.alloc_alloc(layout)
        }

        #[inline(always)]
        unsafe fn alloc_zeroed(&mut self, layout: Layout) -> Result<MemoryAddress, AllocErr> {
            self.alloc_alloc_zeroed(layout)
        }

        #[inline(always)]
        unsafe fn dealloc(&mut self, ptr: MemoryAddress, layout: Layout) {
            self.alloc_dealloc(ptr, layout)
        }

        #[inline(always)]
        unsafe fn realloc(
            &mut self,
            ptr: MemoryAddress,
            layout: Layout,
            new_size: usize,
        ) -> Result<MemoryAddress, AllocErr> {
            self.alloc_realloc(ptr, layout, new_size)
        }
    };
}
