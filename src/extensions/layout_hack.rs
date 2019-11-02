use std::alloc::Layout;
use std::intrinsics::transmute;
use std::num::NonZeroUsize;
use std::num::NonZeroU32;
use std::fmt::Debug;

/// Deliberately structured like Layout to provide access to fields.
pub(crate) struct LayoutHack
{
	pub(crate) size_: usize,
	pub(crate) align_: NonZeroUsize,
}

impl LayoutHack
{
	#[inline(always)]
	pub(crate) fn access_private_fields(layout: Layout) -> Self
	{
		unsafe { transmute(layout) }
	}
}
