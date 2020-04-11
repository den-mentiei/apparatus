#![allow(dead_code)]
#![allow(unused_variables)]

use std::fs::File;
use std::io::Read;
use std::path::Path;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Result<T, E = Error> = std::result::Result<T, E>;

const SUBJECT: &str = "subject\\bin\\Debug\\netcoreapp3.1\\subject.exe";

const DOS_MAGIC: u16 = 0x5a4d;

fn main() -> Result<()> {	
	println!("Hello, sailor!");
	let path = std::env::current_dir()?;
	println!("The current directory is `{}`.", path.display());
	println!("Subject: `{}`.", SUBJECT);

	let data = read_whole_file(Path::new(SUBJECT))?;
	println!("Subject size: {} bytes.", data.len());

	let magic = data.read_u16()?;
	if magic != DOS_MAGIC {
		Err("Signature is wrong!")?;
	}
	
	Ok(())
}

fn read_whole_file(path: &Path) -> Result<Box<[u8]>> {
	let mut f   = File::open(path)?;
	let mut buf = Vec::new();
	f.read_to_end(&mut buf)?;
	Ok(buf.into_boxed_slice())
}

// Reading
// -------

#[derive(Debug)]
enum BufError {
	// TODO(dmi): @robust Add `available` and `required` sizes.
	NotEnoughData
}

impl std::error::Error for BufError {}

impl std::fmt::Display for BufError {
	fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
		match *self {
			BufError::NotEnoughData => { write!(fmt, "not enough input data!") },
		}
	}
}

type BufResult<T> = std::result::Result<T, BufError>;

trait Buf {
	fn read_u8(&self) -> BufResult<u8>;
	fn read_i8(&self) -> BufResult<i8>;

	fn read_u16(&self) -> BufResult<u16>;
	fn read_i16(&self) -> BufResult<i16>;

	fn read_u32(&self) -> BufResult<u32>;
	fn read_i32(&self) -> BufResult<i32>;

	fn read_u64(&self) -> BufResult<u64>;
	fn read_i64(&self) -> BufResult<i64>;
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
	fn read_u8(&self) -> BufResult<u8> { read_impl!(self, u8) }
	#[inline]
	fn read_i8(&self) -> BufResult<i8> { read_impl!(self, i8) }

	#[inline]
	fn read_u16(&self) -> BufResult<u16> { read_impl!(self, u16) }
	#[inline]
	fn read_i16(&self) -> BufResult<i16> { read_impl!(self, i16) }

	#[inline]
	fn read_u32(&self) -> BufResult<u32> { read_impl!(self, u32) }
	#[inline]
	fn read_i32(&self) -> BufResult<i32> { read_impl!(self, i32) }

	#[inline]
	fn read_u64(&self) -> BufResult<u64> { read_impl!(self, u64) }
	#[inline]
	fn read_i64(&self) -> BufResult<i64> { read_impl!(self, i64) }
}

macro_rules! delegate {
	($name: ident, $ty: ty) => {
		#[inline]
		fn $name(&self) -> BufResult<$ty> { (**self).$name() }
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
