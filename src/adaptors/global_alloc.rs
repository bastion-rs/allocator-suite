#[doc(hidden)]
#[macro_export]
macro_rules! global_alloc {
    () => {
        #[inline(always)]
        unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
            self.global_alloc_alloc(layout)
        }

        #[inline(always)]
        unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
            self.global_alloc_dealloc(ptr, layout)
        }
    };
}
