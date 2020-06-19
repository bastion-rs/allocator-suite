#[doc(hidden)]
#[macro_export]
macro_rules! alloc_ref {
    () => {
        #[inline(always)]
        fn alloc(&mut self, layout: Layout, init: AllocInit) -> Result<MemoryBlock, AllocErr> {
            let size = layout.size();
            let ptr = match init {
                AllocInit::Uninitialized => unsafe { self.alloc_alloc(layout) },
                AllocInit::Zeroed => unsafe { self.alloc_alloc_zeroed(layout) },
            }?;
            Ok(MemoryBlock { ptr, size })
        }

        #[inline(always)]
        unsafe fn dealloc(&mut self, ptr: MemoryAddress, layout: Layout) {
            self.alloc_dealloc(ptr, layout)
        }
    };
}
