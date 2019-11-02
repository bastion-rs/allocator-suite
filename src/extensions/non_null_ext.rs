use std::ptr::NonNull;

/// Extensions to make working with NonNull easier.
pub(crate) trait NonNullExt<T>
{
	/// To a reference.
	fn reference<'any>(self) -> &'any T;

	/// To a mutable reference.
	fn mutable_reference<'any>(self) -> &'any mut T;
}

impl<T> NonNullExt<T> for NonNull<T>
{
	#[inline(always)]
	fn reference<'any>(self) -> &'any T
	{
		unsafe { & * self.as_ptr() }
	}

	#[inline(always)]
	fn mutable_reference<'any>(self) -> &'any mut T
	{
		unsafe { &mut * self.as_ptr() }
	}
}
