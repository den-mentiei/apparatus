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

pub trait Buf {
	fn read_u8(&self) -> Result<u8>;
	fn read_i8(&self) -> Result<i8>;

	fn read_u16(&self) -> Result<u16>;
	fn read_i16(&self) -> Result<i16>;

	fn read_u32(&self) -> Result<u32>;
	fn read_i32(&self) -> Result<i32>;

	fn read_u64(&self) -> Result<u64>;
	fn read_i64(&self) -> Result<i64>;
}

macro_rules! read_impl {
	($self:ident, $ty: ty) => {
		{
			const SIZE: usize = std::mem::size_of::<$ty>();
			debug_assert!($self.len() >= SIZE);
			unsafe {
				Ok(std::ptr::read($self.as_ptr() as *const $ty))
			}
		}
	}
}

impl Buf for [u8] {
	#[inline]
	fn read_u8(&self) -> Result<u8> { read_impl!(self, u8) }
	#[inline]
	fn read_i8(&self) -> Result<i8> { read_impl!(self, i8) }

	#[inline]
	fn read_u16(&self) -> Result<u16> { read_impl!(self, u16) }
	#[inline]
	fn read_i16(&self) -> Result<i16> { read_impl!(self, i16) }

	#[inline]
	fn read_u32(&self) -> Result<u32> { read_impl!(self, u32) }
	#[inline]
	fn read_i32(&self) -> Result<i32> { read_impl!(self, i32) }

	#[inline]
	fn read_u64(&self) -> Result<u64> { read_impl!(self, u64) }
	#[inline]
	fn read_i64(&self) -> Result<i64> { read_impl!(self, i64) }
}

macro_rules! delegate {
	($name: ident, $ty: ty) => {
		#[inline]
		fn $name(&self) -> Result<$ty> { (**self).$name() }
	}
}

impl<B: Buf> Buf for Box<B> {
	delegate!(read_u8, u8);
	delegate!(read_i8, i8);

	delegate!(read_u16, u16);
	delegate!(read_i16, i16);

	delegate!(read_u32, u32);
	delegate!(read_i32, i32);

	delegate!(read_u64, u64);
	delegate!(read_i64, i64);
}
