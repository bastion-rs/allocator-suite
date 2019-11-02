#[inline(always)]
pub(crate) const fn logarithm_base2_as_usize(value: usize) -> usize {
    value.trailing_zeros() as usize
}
