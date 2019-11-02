use crate::extensions::prelude::*;

/// Useful extensions.
pub(crate) trait PointerMutExt<T>: PointerExt<T>
{
	/// Mutable reference.
	fn mutable_reference<'a>(self) -> &'a mut T;
}

impl<T> PointerMutExt<T> for *mut T
{
	#[inline(always)]
	fn mutable_reference<'a>(self) -> &'a mut T
	{
		debug_assert!(self.is_not_null(), "null pointers can not be derefenced");

		unsafe { &mut * self }
	}
}
