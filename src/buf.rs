use std::ops::{Index, RangeFrom};

#[derive(Debug)]
pub enum Error {
	// TODO(dmi): @robust Add `available` and `required` sizes.
	NotEnoughData
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
	fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
		match *self {
			Error::NotEnoughData => { write!(fmt, "not enough input data!") },
		}
	}
}

pub type Result<T> = std::result::Result<T, Error>;

// Must not fail.
pub trait Part<Src: ?Sized = [u8]> {
	fn read(src: &Src) -> Self;
}

pub trait TryPart<'a, Src: ?Sized = [u8]> where Self: 'a + Sized {
	fn try_read(src: &'a Src) -> Result<Self>;
}

pub trait PartSize {
	fn size() -> usize;
}

// TODO(dmi): @idea impl<'a, Src: ?Sized, T> TryBuf<'a, Src> for T where T: 'a { ... }

macro_rules! read_impl {
	($ty:tt, $size:expr) => {
		impl PartSize for $ty {
			#[inline(always)]
			fn size() -> usize {
				std::mem::size_of::<$ty>()
			}
		}
		
		impl<'a> Part for $ty {
			#[inline]
			fn read(src: &[u8]) -> Self {
				debug_assert!(src.len() >= $size);
				unsafe { std::ptr::read(src.as_ptr() as *const $ty) }
			}
		}

		impl<'a> TryPart<'a> for $ty where $ty: Part {
			#[inline]
			fn try_read(src: &'a [u8]) -> Result<Self> {
				if $size > src.len () {
					Err($crate::buf::Error::NotEnoughData)
				} else {
					Ok(Part::read(src))
				}
			}
		}

		impl<'a, Src> Part<Src> for $ty where Src: AsRef<[u8]> {
			#[inline]
			fn read(src: &Src) -> Self {
				let src = src.as_ref();
				Self::read(src)
			}
		}

		impl<'a, Src> TryPart<'a, Src> for $ty where $ty: Part<Src>, Src: AsRef<[u8]> {
			#[inline]
			fn try_read(src: &'a Src) -> Result<Self> {
				let src = src.as_ref();
				Self::try_read(src)
			}
		}
	}
}

read_impl!(u8,   1);
read_impl!(i8,   1);
read_impl!(u16,  2);
read_impl!(i16,  2);
read_impl!(u32,  4);
read_impl!(i32,  4);
read_impl!(u64,  8);
read_impl!(i64,  8);
read_impl!(u128, 16);
read_impl!(i128, 16);

pub trait Reading : Index<usize> + Index<RangeFrom<usize>> {
	fn read_at<'a, T>(self: &'a Self, offset: usize) -> Result<T>
	where
		<Self as Index<RangeFrom<usize>>>::Output: 'a,
		T: TryPart<'a, <Self as Index<RangeFrom<usize>>>::Output>
	{
		// TODO(dmi): @robustness Any offset checks?
		T::try_read(&self[offset..])
	}

	fn read<'a, T>(self: &'a Self, offset: &mut usize) -> Result<T>
	where
		<Self as Index<RangeFrom<usize>>>::Output: 'a,
		T: TryPart<'a, <Self as Index<RangeFrom<usize>>>::Output> + PartSize
	{
		// TODO(dmi): @robustness Any offset checks?
		let o = *offset;
		T::try_read(&self[o..]).and_then(|x| {
			*offset += T::size();
			Ok(x)
		})
	}
}

impl<T> Reading for T where T: ?Sized + Index<usize> + Index<RangeFrom<usize>> {}
