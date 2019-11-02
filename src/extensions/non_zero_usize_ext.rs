use crate::extensions::prelude::*;
use std::fmt::Debug;
use std::num::NonZeroU32;
use std::num::NonZeroUsize;

/// Useful extensions.
pub trait NonZeroUsizeExt: Sized + Copy + Ord + Debug {
    /// Next power of two.
    #[inline(always)]
    fn next_power_of_two(self) -> Self {
        Self::non_zero_unchecked(self.to_usize().next_power_of_two())
    }

    /// Round up.
    #[inline(always)]
    fn round_up_to_power_of_two(self, non_zero_power_of_two_alignment: NonZeroUsize) -> Self {
        let power_of_two = non_zero_power_of_two_alignment.get();
        let power_of_two_less_one = power_of_two - 1;

        debug_assert!(
            self.checked_add(power_of_two_less_one).is_some(),
            "non_zero_power_of_two_alignment is far too close to the maximum value of a pointer"
        );

        Self::non_zero(self.add(power_of_two_less_one).to_usize() & !power_of_two_less_one)
    }

    /// Round down.
    #[inline(always)]
    fn round_down_to_power_of_two(self, power_of_two: NonZeroUsize) -> usize {
        let value = self.to_usize();
        let power_of_two_exponent = power_of_two.logarithm_base2();

        value & !((1 << power_of_two_exponent) - 1)
    }

    /// Divide.
    #[inline(always)]
    fn divide_power_of_two_by_power_of_two(self, divisor: NonZeroUsize) -> usize {
        debug_assert!(
            self.is_power_of_two(),
            "self `{:?}` is not a power of two",
            self
        );
        debug_assert!(
            divisor.is_power_of_two(),
            "divisor `{:?}` is not a power of two",
            divisor
        );

        self.to_usize() >> divisor.logarithm_base2()
    }

    /// Is power of two.
    #[inline(always)]
    fn is_power_of_two(self) -> bool {
        self.to_usize().is_power_of_two()
    }

    /// Logarithm base two.
    #[inline(always)]
    fn logarithm_base2(self) -> usize {
        self.to_usize().trailing_zeros() as usize
    }

    /// Decrement.
    #[inline(always)]
    fn decrement(self) -> usize {
        self.to_usize() - 1
    }

    /// Add.
    #[inline(always)]
    fn add(self, increment: usize) -> Self {
        Self::non_zero(self.to_usize() + increment)
    }

    /// Add.
    #[inline(always)]
    fn add_non_zero(self, increment: NonZeroUsize) -> Self {
        Self::non_zero(self.to_usize() + increment.get())
    }

    /// Add.
    #[inline(always)]
    fn checked_add(self, increment: usize) -> Option<Self> {
        self.to_usize().checked_add(increment).map(Self::non_zero)
    }

    /// Add.
    #[inline(always)]
    fn add_assign(&mut self, increment: usize) {
        *self = (*self).add(increment)
    }

    /// Double.
    #[inline(always)]
    fn doubled(self) -> NonZeroUsize {
        (self.to_usize() << 1).non_zero()
    }

    /// Difference.
    #[inline(always)]
    fn difference(self, other: Self) -> usize {
        debug_assert!(
            self >= other,
            "other `{:?}` is less than self `{:?}`",
            other,
            self
        );

        self.to_usize() - other.to_usize()
    }

    /// Difference.
    #[inline(always)]
    fn difference_non_zero(self, other: Self) -> NonZeroUsize {
        self.difference(other).non_zero()
    }

    /// Multiply.
    #[inline(always)]
    fn multiply(self, other: Self) -> NonZeroUsize {
        (self.to_usize() * other.to_usize()).non_zero()
    }

    /// Is odd.
    #[inline(always)]
    fn is_odd(self) -> bool {
        self.to_usize().is_odd()
    }

    /// Non zero.
    #[inline(always)]
    fn to_non_zero_u32(self) -> NonZeroU32 {
        let usize = self.to_usize();
        debug_assert!(
            usize <= ::std::u32::MAX as usize,
            "exceeds `{}` ::std::u32::MAX `{}`",
            usize,
            ::std::u32::MAX
        );
        NonZeroU32::non_zero_unchecked(usize as u32)
    }

    /// Non zero.
    #[inline(always)]
    fn non_zero(value: usize) -> Self {
        debug_assert_ne!(value, 0, "value is zero");

        Self::non_zero_unchecked(value)
    }

    /// Non zero.
    fn non_zero_unchecked(value: usize) -> Self;

    #[doc(hidden)]
    fn to_usize(self) -> usize;
}

impl NonZeroUsizeExt for NonZeroUsize {
    #[inline(always)]
    fn difference(self, other: Self) -> usize {
        debug_assert!(
            self >= other,
            "other `{:?}` is less than self `{:?}`",
            other,
            self
        );

        self.get() - other.get()
    }

    #[inline(always)]
    fn non_zero_unchecked(value: usize) -> Self {
        non_zero_usize(value)
    }

    #[inline(always)]
    fn to_usize(self) -> usize {
        self.get()
    }
}
