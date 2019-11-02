use std::num::NonZeroUsize;

/// Non-zero wrapper function.
#[inline(always)]
pub const fn non_zero_usize(value: usize) -> NonZeroUsize {
    unsafe { NonZeroUsize::new_unchecked(value) }
}
