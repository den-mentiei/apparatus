#![allow(dead_code)]
#![allow(unused_variables)]

use std::fs::File;
use std::io::Read;
use std::path::Path;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Result<T, E = Error> = std::result::Result<T, E>;

const SUBJECT: &str = "subject\\bin\\Debug\\netcoreapp3.1\\subject.dll";

// Dos header magic: MZ little-endian.
const DOS_MAGIC: u16 = 0x5a4d;
const PE_OFFSET: usize = 0x3c;

// PE header magic: PE little-endian.
const PE_MAGIC: u32 = 0x0000_4550;
const IMAGE_FILE_MACHINE_I386: u16 = 0x14c;

// The following were taken from ECMA 25.2.2.1

// Shall be zero.
const IMAGE_FILE_RELOCS_STRIPPED: u16  = 0x0001;
// Shall be one.
const IMAGE_FILE_EXECUTABLE_IMAGE: u16 = 0x0002;
// Shall be one if and only if COMIMAGE_FLAGS_32BITREQUIRED is one.
const IMAGE_FILE_32BIT_MACHINE: u16    = 0x0100;
// A CIL-only DLL sets flag to one, while a CIL-only .exe has flag set to zero.
const IMAGE_FILE_DLL: u16              = 0x2000;

// Optional header magic.
const OPT_MAGIC_PE32: u16 = 0x10b;
const STANDARD_FIELDS_32_SIZE: usize = 28;
const WINDOWS_FIELDS_32_SIZE: usize = 68;

const DATA_DIRS_OFFSET: usize = STANDARD_FIELDS_32_SIZE + WINDOWS_FIELDS_32_SIZE;
const DATA_DIRS_COUNT: usize = 16;
const DATA_DIR_INDEX_CLI_HEADER: usize = 14;

const SECTION_SIZE: usize = 40;
const SECTION_VIRTUAL_SIZE_OFFSET: usize  = 8;
const SECTION_RVA_OFFSET: usize           = 12;
const SECTION_RAW_DATA_SIZE_OFFSET: usize = 16;
const SECTION_RAW_DATA_PTR_OFFSET: usize  = 20;

// Taken from ECMA 25.3.3.1

// Shall be one.
const COMIMAGE_FLAGS_ILONLY: u32            = 0x00000001;
// Set if image can only be loaded into a 32-bit process,
// for instance if there are 32-bit vtable fixups, or casts
// from native integers into int32. CLI implementation that
// have 64-bit native integers shall refuce loading binaries
// with this flag set.
const COMIMAGE_FLAGS_32BITREQUIRED: u32     = 0x00000002;
// Image has a strong name signature.
const COMIMAGE_FLAGS_STRONGNAMESIGNED: u32  = 0x00000008;
// Shall be zero.
const COMIMAGE_FLAGS_NATIVE_ENTRYPOINT: u32 = 0x00000010;
// Should be zero.
const COMIMAGE_FLAGS_TRACKDEBUGDATA: u32    = 0x00010000;

fn main() -> Result<()> {	
	println!("Hello, sailor!");
	let path = std::env::current_dir()?;
	println!("The current directory is `{}`.", path.display());
	println!("Subject: `{}`.", SUBJECT);

	let data = &*read_whole_file(Path::new(SUBJECT))?;
	println!("Subject size: {} bytes.", data.len());

	let magic = data.read_u16()?;
	if magic != DOS_MAGIC {
		Err("Signature is wrong!")?;
	}

	let pe_offset = data[PE_OFFSET..].read_u32()? as usize;
	
	let pe = &data[pe_offset..];
	let magic = pe.read_u32()?;
	if magic != PE_MAGIC {
		Err("PE signature is wrong!")?;
	}

	let coff = &pe[4..];
	let machine = coff.read_u16()?;

	if machine != IMAGE_FILE_MACHINE_I386 {
		Err("Unexpected target machine specified.")?;
	}

	let n_sections = coff[2..].read_u16()? as usize;
	println!("Number of sections: {}", n_sections);

	let opt_header_size = coff[16..].read_u16()? as usize;
	println!("Size of optional header: {:#0x}", opt_header_size);

	let characteristics = coff[18..].read_u16()?;

	if characteristics & IMAGE_FILE_RELOCS_STRIPPED != 0 {
		Err("Relocations are not stripped.")?;
	}
	if characteristics & IMAGE_FILE_EXECUTABLE_IMAGE == 0 {
		Err("File is not marked as an executable image.")?;
	}
	if characteristics & IMAGE_FILE_DLL != 0 {
		Err("File is not a CIL executable, but a class library.")?;
	}

	let opt_header = &coff[20..];
	let magic = opt_header.read_u16()?;
	if magic != OPT_MAGIC_PE32 {
		Err("Optional header magic is not PE32.")?;
	}

	let n_data_dirs = opt_header[92..].read_u32()? as usize;
	if n_data_dirs != DATA_DIRS_COUNT {
		Err("Number of data directories is invalid.")?;
	}

	let cli_header_rva  = opt_header[(DATA_DIRS_OFFSET + DATA_DIR_INDEX_CLI_HEADER * 8)..].read_u32()? as usize;
	let cli_header_size = opt_header[(DATA_DIRS_OFFSET + DATA_DIR_INDEX_CLI_HEADER * 8 + 4)..].read_u32()? as usize;
	println!("CLI header RVA: {:#0x}", cli_header_rva);
	println!("CLI header size: {:#0x}", cli_header_size);

	let section_table = &opt_header[opt_header_size..];

	let mut cli_header_offset = None;
	
	let mut s: usize = 0;
	for i in 0..n_sections {
		let section = &section_table[s..];
		let vsize = section[SECTION_VIRTUAL_SIZE_OFFSET..].read_u32()? as usize;
		let rva   = section[SECTION_RVA_OFFSET..].read_u32()? as usize;
		let rsize = section[SECTION_RAW_DATA_SIZE_OFFSET..].read_u32()? as usize;
		let raw   = section[SECTION_RAW_DATA_PTR_OFFSET..].read_u32()? as usize;

		println!("Section #{}:", i);
		println!("  rva: {:#0x}", rva);
		println!("  virtual size: {:#0x}", vsize);
		println!("  raw: {:#0x}", raw);
		println!("  raw size: {:#0x}", rsize);

		if cli_header_rva >= rva && cli_header_rva < rva + vsize {
			println!("  * contains CLI header!");
			cli_header_offset = Some(cli_header_rva - rva + raw);
		}

		s += SECTION_SIZE;
	}

	let cli_header_offset = cli_header_offset.ok_or("Failed to find CLI header.")?;
	let cli_header = &data[cli_header_offset..];

	let size = cli_header.read_u32()? as usize;
	if cli_header_size != size {
		Err("CLI header specifies wrong size.")?;
	}

	// Offsets are defined in ECMA 25.3.3.
	let rt_major = cli_header[4..].read_u16()?;
	let rt_minor = cli_header[6..].read_u16()?;
	println!("CLI runtime: {}.{}", rt_major, rt_minor);

	let metadata_rva  = cli_header[8..].read_u32()? as usize;
	let metadata_size = cli_header[12..].read_u32()? as usize;
	println!("CLI physical metadata: {:#0x}, {:#0x} bytes.", metadata_rva, metadata_size);

	let flags = cli_header[16..].read_u32()?;
	if flags & COMIMAGE_FLAGS_ILONLY == 0 {
		Err("Assembly contains not only IL.")?;
	}
	if (flags & COMIMAGE_FLAGS_32BITREQUIRED != 0) && os_is_64() {
		Err("Assembly can be loaded only in 32-bit process.")?;
	}
	if flags & COMIMAGE_FLAGS_STRONGNAMESIGNED != 0 {
		println!("Assembly has a strong name signature.");
	}
	if flags & COMIMAGE_FLAGS_NATIVE_ENTRYPOINT != 0 {
		Err("Assembly has native entry-point.")?;
	}
	if flags & COMIMAGE_FLAGS_TRACKDEBUGDATA != 0 {
		Err("Assembly requires debug data tracking.")?;
	}

	let ep_token = cli_header[20..].read_u32()?;

	let cm_table = cli_header[40..].read_u64()?;
	if cm_table != 0 {
		Err("Assembly has code manager table.")?;
	}

	let vtable_fixups = cli_header[48..].read_u64()?;
	if vtable_fixups != 0 {
		Err("Assembly has VTable fixups.")?;
	}

	let eat_jumps = cli_header[56..].read_u64()?;
	if eat_jumps != 0 {
		Err("Assembly has export address table jumps.")?;
	}
	
	let managed_native_header = cli_header[64..].read_u64()?;
	if managed_native_header != 0 {
		Err("Assembly has managed native header.")?;
	}
	
	// dump(cli_header, 128);

	Ok(())
}

fn read_whole_file(path: &Path) -> Result<Box<[u8]>> {
	let mut f   = File::open(path)?;
	let mut buf = Vec::new();
	f.read_to_end(&mut buf)?;
	Ok(buf.into_boxed_slice())
}

fn dump(data: &[u8], n: usize) {
	assert!(n < data.len());

	const COLUMNS: usize = 16;
	const OFFSET_WIDTH: usize = std::mem::size_of::<usize>() << 1;
	
	let mut p: usize = 0;

	for row in data[..n].chunks(COLUMNS) {
		print!("{1:#00$x} | ", OFFSET_WIDTH, p);
		for x in row {
			print!("{:02x} ", x);
		}
		print!("| ");
		for x in row {
			if x.is_ascii_alphanumeric() || x.is_ascii_punctuation() || x.is_ascii_graphic() {
				print!("{}", *x as char);
			} else {
				print!(".");
			}
		}
		println!("");

		p += COLUMNS;
	}
}

#[cfg(target_pointer_width = "32")]
fn os_is_64() -> bool { false }

#[cfg(target_pointer_width = "64")]
fn os_is_64() -> bool { true }

// Buffer
// ------

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
