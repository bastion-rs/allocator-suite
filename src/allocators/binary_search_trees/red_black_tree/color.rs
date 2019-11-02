#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(usize)]
pub(crate) enum Color
{
	Red = 0,

	Black = 1,
}

impl Color
{
	#[inline(always)]
	pub(crate) fn color_bit(self) -> usize
	{
		self as usize
	}

	#[inline(always)]
	pub(crate) fn is_red(self) -> bool
	{
		self == Color::Red
	}

	#[inline(always)]
	pub(crate) fn is_black(self) -> bool
	{
		self == Color::Black
	}
}
