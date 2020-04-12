#![allow(dead_code)]
#![allow(unused_variables)]

mod utils;
mod buf;

use std::path::Path;

use aps::Result;
use utils::{read_whole_file, dump, align_up, os_is_64};
use buf::Buf;

const SUBJECT: &str = "subject\\bin\\Debug\\netcoreapp3.1\\subject.dll";

// Dos header magic: MZ (little-endian).
const DOS_MAGIC: u16 = 0x5a4d;
const PE_OFFSET: usize = 0x3c;

// PE header magic: PE (little-endian).
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

// Taken from ECMA 24.2.1

// Magic signature for physical metadata: BSJB (little-endian).
const METADATA_MAGIC: u32 = 0x424A5342;

const METADATA_STREAM_NAME_MAX_LEN: usize = 32;

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

		if cli_header_rva >= rva && cli_header_rva < rva + vsize {
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
	
	// TODO(dmi): @cleanup This copy-pasta should be factored out.
	let mut metadata_offset = None;
	let mut s: usize = 0;
	for i in 0..n_sections {
		let section = &section_table[s..];
		let vsize = section[SECTION_VIRTUAL_SIZE_OFFSET..].read_u32()? as usize;
		let rva   = section[SECTION_RVA_OFFSET..].read_u32()? as usize;
		let rsize = section[SECTION_RAW_DATA_SIZE_OFFSET..].read_u32()? as usize;
		let raw   = section[SECTION_RAW_DATA_PTR_OFFSET..].read_u32()? as usize;

		if metadata_rva >= rva && metadata_rva < rva + vsize {
			metadata_offset = Some(metadata_rva - rva + raw);
		}

		s += SECTION_SIZE;
	}

	let metadata_offset = metadata_offset.ok_or("Failed to find CLI metadata.")?;
	let metadata = &data[metadata_offset..];

	let magic = metadata.read_u32()?;
	if magic != METADATA_MAGIC {
		Err("Metadata signature is wrong.")?;
	}

	let len_version = metadata[12..].read_u32()? as usize;
	if len_version > 255 {
		Err("Metadata version length is incorrect.")?;
	}
	
	let version = std::str::from_utf8(&metadata[16..(16 + len_version)])?;
	println!("Version: {}", version);

	let offset = 16 + align_up(len_version, 4);

	let n_streams = metadata[(offset + 2)..].read_u16()? as usize;
	println!("Metadata streams: {}", n_streams);

	let streams = &metadata[(offset + 4)..];

	let mut s: usize = 0;
	for i in 0..n_streams {
		let header = &streams[s..];
		
		let offset = header[0..].read_u32()? as usize;
		let size   = header[4..].read_u32()? as usize;

		let name = &header[8..];
		let mut len: usize = 0;
		for j in 0..METADATA_STREAM_NAME_MAX_LEN {
			len += 1;
			if name[j] == 0 {
				break;
			}
		}
		if len > METADATA_STREAM_NAME_MAX_LEN {
			Err("Metadata stream name lenght is invalid.")?;
		}

		let name = std::str::from_utf8(&name[..len - 1])?;
		println!("Stream #{}: `{}`, at {:#0x}, {:#0x} bytes.", i, name, offset, size);

		let data = &metadata[offset..offset + size];
		match name {
			"#~" => read_logical_tables(data)?,
			"#Strings" => read_strings(data)?,
			"#US" => read_user_strings(data)?,
			"#Blob" => read_blobs(data)?,
			"#GUID" => read_guids(data)?,
			_ => println!("^ unknown table!"),
		}
		
		s += 8 + align_up(len, 4);
	}
	
	Ok(())
}

fn read_logical_tables(data: &[u8]) -> Result<()> {
	// dump(data, 24);
	Ok(())
}

fn read_strings(data: &[u8]) -> Result<()> {
	debug_assert!(data.len() >= 1);
	debug_assert!(data[0] == 0);

	let mut n: usize = 0;
	for s in data[1..].split(|c| *c == 0) {
		n += 1;

		let s = std::str::from_utf8(s)?;
		if s.len() > 0 {
			println!("  `{}`", s);
		}
	}
	println!("  {} string(s).", n);

	Ok(())
}

fn read_user_strings(data: &[u8]) -> Result<()> {
	// Strings in the #US (user string) heap are encoded using 16-bit Unicode
	// encodings. The count on each string is the number of bytes (not
	// characters) in the string. Furthermore, there is an additional
	// terminal byte (so all byte counts are odd, not even). This final byte
	// holds the value 1 if and only if any UTF16 character within the string
	// has any bit set in its top byte, or its low byte is any of the
	// following: 0x01-0x08, 0x0E-0x1F, 0x27, 0x2D, 0x7F.  Otherwise,
	// it holds 0. The 1 signifies Unicode characters that require handling
	// beyond that normally provided for 8-bit encoding sets.

	let mut n: usize = 0;
	let mut i: usize = 0;
	while i < data.len() - 1 {
		let (blob, len) = read_blob_len(&data[i..])?;
		if blob.len() > 0 {
			let len = blob.len() - 1;
			let wide: &[u16] = unsafe {
				std::slice::from_raw_parts(
					blob.as_ptr() as *const u16,
					len >> 1)
			};
			let s = String::from_utf16(wide)?;
			println!("  `{}` (fits ascii: {})", s, blob[len] == 0);

			n += 1;
		}
		i += len;
	}

	println!("  {} string(s).", n);

	Ok(())
}

fn read_blobs(data: &[u8]) -> Result<()> {
	dump(data, data.len());
	Ok(())
}

// TODO(dmi): @check Add few large strings to subject.
fn read_blob_len(data: &[u8]) -> Result<(&[u8], usize)> {
	let b0 = data[0];
	if b0 & 0b1000_0000 == 0 {
		let n = (b0 & 0b0111_1111) as usize;
		return Ok((&data[1..n + 1], n + 1));
	}

	if b0 & 0b1100_0000 == 0b1000_0000 {
		let x = data[1] as usize;
		let n = ((b0 & 0b0011_1111) as usize) << 8 + x;
		return Ok((&data[1..n + 1], n + 2));
	}

	if b0 & 0b1110_0000 == 0b1100_0000 {
		let x = data[1] as usize;
		let y = data[2] as usize;
		let z = data[3] as usize;
		let n = ((b0 & 0b0001_1111) as usize) << 24 + (x << 16) + (y << 8) + z;
		return Ok((&data[1..n + 1], n + 4));
	}

	Err("Incorrect blob length.")?
}

fn read_guids(data: &[u8]) -> Result<()> {
	if data.len() & 15 !=0 {
		Err("Invalid #GUID heap size.")?;
	}

	for g in data.chunks(16) {
		let data1 = g[0..].read_u32()?;
		let data2 = g[4..].read_u16()?;
		let data3 = g[6..].read_u16()?;

		print!("  {:08X}-{:04X}-{:04X}-", data1, data2, data3);
		for x in &g[8..] {
			print!("{:02X}", x)
		}
		println!();
	}
	
	println!("  {} guid(s).", data.len() >> 4);

	Ok(())
}
