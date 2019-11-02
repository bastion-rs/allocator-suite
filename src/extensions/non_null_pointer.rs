use std::ptr::NonNull;

#[inline(always)]
pub(crate) const fn non_null_pointer<T>(value: *mut T) -> NonNull<T> {
    unsafe { NonNull::new_unchecked(value as *mut T) }
}
