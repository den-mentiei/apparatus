use std::str;

use log::{debug};

use crate::Result;
use crate::error::Error;
use crate::buf::Reading;
use crate::utils::{align_up, dump, os_is_64};

// Taken from ECMA II.25.3.3.1

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

// Taken from ECMA II.24.2.1

// Magic signature for physical metadata: BSJB (little-endian).
const METADATA_MAGIC: u32 = 0x424A5342;

const METADATA_STREAM_NAME_MAX_LEN: usize = 32;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct Header {
	pub ep_token: u32,
	pub metadata_rva:  u32,
	pub metadata_size: u32,
}

impl Header {
	pub fn parse(data: &[u8], pe: &crate::pe::Header) -> Result<Header> {
		let mut offset = &mut 0usize;
		
		let size: u32 = data.read(offset)?;
		if size != pe.cli_size {
			Err("CLI header specifies wrong size.")?;
		}

		// Offsets are defined in ECMA II.25.3.3.
		let rt_major: u16 = data.read(offset)?;
		let rt_minor: u16 = data.read(offset)?;
		debug!("CLI runtime: {}.{}", rt_major, rt_minor);

		let metadata_rva  = data.read(offset)?;
		let metadata_size = data.read(offset)?;
		debug!("CLI physical metadata: {:#0x}, {:#0x} bytes.", metadata_rva, metadata_size);

		Header::check_flags(data, offset)?;

		let ep_token: u32 = data.read(offset)?;

		Header::check_fields(data, offset)?;

		Ok(Header { ep_token, metadata_rva, metadata_size })
	}

	fn check_flags(data: &[u8], offset: &mut usize) -> Result<()> {
		let flags: u32 = data.read(offset)?;
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
		Ok(())
	}

	fn check_fields(data: &[u8], offset: &mut usize) -> Result<()> {
		let cm_table: u64 = data.read(offset)?;
		if cm_table != 0 {
			Err("Assembly has code manager table.")?;
		}

		let vtable_fixups: u64 = data.read(offset)?;
		if vtable_fixups != 0 {
			Err("Assembly has VTable fixups.")?;
		}

		let eat_jumps: u64 = data.read(offset)?;
		if eat_jumps != 0 {
			Err("Assembly has export address table jumps.")?;
		}

		let managed_native_header: u64 = data.read(offset)?;
		if managed_native_header != 0 {
			Err("Assembly has managed native header.")?;
		}

		Ok(())
	}
}

#[derive(Debug, PartialEq, Clone, Default)]
pub struct Metadata<'a> {
	pub logical_tables: Option<&'a [u8]>,
	pub strings:        Option<&'a [u8]>,
	pub user_strings:   Option<&'a [u8]>,
	pub blobs:          Option<&'a [u8]>,
	pub guids:          Option<&'a [u8]>,
}

impl<'a> Metadata<'a> {
	pub fn parse(data: &'a [u8]) -> Result<Metadata<'a>> {
		let mut offset = &mut 0usize;

		let magic: u32 = data.read(offset)?;
		if magic != METADATA_MAGIC {
			Err("Metadata signature is wrong.")?;
		}

		*offset += 8;

		let len_version: u32 = data.read(offset)?;
		if len_version > 255 {
			Err("Metadata version length is incorrect.")?;
		}
		
		let version = str::from_utf8(&data[16..(16 + len_version as usize)])
			.map_err(|_| Error::General("Version string is not a valid utf-8 string."))?;
		debug!("Version: {}", version);

		*offset += align_up(len_version as usize, 4) + 2;

		let n_streams: u16 = data.read(offset)?;
		debug!("Metadata streams: {}", n_streams);

		let mut logical_tables = None;
		let mut strings =        None;
		let mut user_strings =   None;
		let mut blobs =          None;
		let mut guids =          None;

		for i in 0..n_streams {
			let s_offset = data.read::<u32>(offset)? as usize;
			let s_size   = data.read::<u32>(offset)? as usize;

			let name = &data[*offset..];
			let mut len = 0;
			
			for j in 0..METADATA_STREAM_NAME_MAX_LEN {
				len += 1;
				if name[j] == 0 {
					break;
				}
			}
			if len > METADATA_STREAM_NAME_MAX_LEN {
				Err("Metadata stream name length is invalid.")?;
			}

			let name = str::from_utf8(&name[..len - 1])
				.map_err(|_| Error::General("Metadata stream name is not a valid utf-8 string."))?;

			debug!("Found stream: `{}` at {:#0x}, {:#0x} byte(s).", name, s_offset, s_size);

			let stream_data = &data[s_offset..s_offset + s_size];
			
			match name {
				"#~"       => logical_tables = Some(stream_data),
				"#Strings" => strings = Some(stream_data),
				"#US"      => user_strings = Some(stream_data),
				"#GUID"    => guids = Some(stream_data),
				"#Blob"    => blobs = Some(stream_data),
				_ => Err("Unknown section name.")?,
			};
			
			*offset += align_up(len, 4);
		}
		
		Ok(Metadata {
			logical_tables,
			strings,
			user_strings,
			blobs,
			guids,
		})
	}
}
