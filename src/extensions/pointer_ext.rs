use crate::extensions::prelude::*;
use core::ptr::NonNull;
use std::num::NonZeroU32;

/// Useful extensions.
pub(crate) trait PointerExt<T>: Sized {
    /// Non null.
    fn non_null(self) -> NonNull<T>;

    /// Add.
    fn add_bytes(self, offset: usize) -> Self;

    /// Add.
    #[inline(always)]
    fn add_bytes_u32(self, offset: u32) -> Self {
        self.add_bytes(offset as usize)
    }

    /// Add.
    #[inline(always)]
    fn add_bytes_non_zero_u32(self, offset: NonZeroU32) -> Self {
        self.add_bytes_u32(offset.get())
    }

    /// To usize.
    fn to_usize(self) -> usize;

    /// Is not null.
    fn is_not_null(self) -> bool;

    /// Reference.
    fn reference<'a>(self) -> &'a T;
}

impl<T> PointerExt<T> for *const T {
    #[inline(always)]
    fn non_null(self) -> NonNull<T> {
        non_null_pointer(self as *mut T)
    }

    #[inline(always)]
    fn add_bytes(self, offset: usize) -> Self {
        ((self as usize) + offset) as *const T
    }

    #[inline(always)]
    fn to_usize(self) -> usize {
        self as usize
    }

    #[inline(always)]
    fn is_not_null(self) -> bool {
        !self.is_null()
    }

    #[inline(always)]
    fn reference<'a>(self) -> &'a T {
        debug_assert!(self.is_not_null(), "null pointers can not be derefenced");

        unsafe { &*self }
    }
}

impl<T> PointerExt<T> for *mut T {
    #[inline(always)]
    fn non_null(self) -> NonNull<T> {
        non_null_pointer(self)
    }

    #[inline(always)]
    fn add_bytes(self, offset: usize) -> Self {
        ((self as usize) + offset) as *mut T
    }

    #[inline(always)]
    fn to_usize(self) -> usize {
        self as usize
    }

    #[inline(always)]
    fn is_not_null(self) -> bool {
        !self.is_null()
    }

    #[inline(always)]
    fn reference<'a>(self) -> &'a T {
        debug_assert!(self.is_not_null(), "null pointers can not be derefenced");

        unsafe { &*self }
    }
}
