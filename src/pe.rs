use log::{trace};

use crate::Result;
use crate::buf::Reading;

/// Dos header magic: MZ (little-endian).
const DOS_MAGIC: u16 = 0x5a4d;
const PE_OFFSET: usize = 0x3c;

/// PE header magic: PE (little-endian).
const PE_MAGIC: u32 = 0x0000_4550;

const IMAGE_FILE_MACHINE_I386: u16 = 0x14c;

// Shall be zero.
const IMAGE_FILE_RELOCS_STRIPPED:  u16 = 0x0001;
// Shall be one.
const IMAGE_FILE_EXECUTABLE_IMAGE: u16 = 0x0002;
// Shall be one if and only if COMIMAGE_FLAGS_32BITREQUIRED is one.
const IMAGE_FILE_32BIT_MACHINE:    u16 = 0x0100;
// A CIL-only DLL sets flag to one, while a CIL-only .exe has flag set to zero.
const IMAGE_FILE_DLL:              u16 = 0x2000;

/// Optional header magic.
const OPT_MAGIC_PE32: u16 = 0x10b;

const DATA_DIRS_COUNT: usize = 16;
const DATA_DIR_INDEX_CLI_HEADER: usize = 14;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct Header {
	pub cli_rva:  u32,
	pub cli_size: u32,
	pub sections: Vec<Section>,
}

#[derive(Debug, PartialEq, Copy, Clone, Default)]
pub struct Section {
	pub virtual_size:    u32,
	pub virtual_address: u32,
	pub raw_address:     u32,
}

impl Header {
	pub fn rva2offset(self: &Self, rva: usize) -> Option<usize> {
		for s in &self.sections {
			// TODO(dmi): @incomplete That should handle virtual vs raw size
			// and alignments, etc.
			let s_rva  = s.virtual_address as usize;
			let s_size = s.virtual_size    as usize;
			let s_raw  = s.raw_address     as usize;

			if s_rva <= rva && rva < s_rva + s_size {
				return Some(rva - s_rva + s_raw);
			}
		}

		None
	}
	
	pub fn parse(data: &[u8]) -> Result<Self> {
		let magic: u16 = data.read_at(0)?;
		if magic != DOS_MAGIC {
			Err("Signature is wrong!")?;
		}

		let mut offset = data.read_at::<u32>(PE_OFFSET)? as usize;
		
		let magic: u32 = data.read(&mut offset)?;
		if magic != PE_MAGIC {
			Err("PE signature is wrong!")?;
		}

		let machine: u16 = data.read(&mut offset)?;
		if machine != IMAGE_FILE_MACHINE_I386 {
			Err("Unexpected target machine specified.")?;
		}

		let n_sections: u16 = data.read(&mut offset)?;
		trace!("Number of sections: {}", n_sections);

		offset += 12;
		
		let opt_header_size: u16 = data.read(&mut offset)?;
		trace!("Size of optional header: {:#0x}", opt_header_size);

		let characteristics: u16 = data.read(&mut offset)?;
		if characteristics & IMAGE_FILE_RELOCS_STRIPPED != 0 {
			Err("Relocations are not stripped.")?;
		}
		if characteristics & IMAGE_FILE_EXECUTABLE_IMAGE == 0 {
			Err("File is not marked as an executable image.")?;
		}
		if characteristics & IMAGE_FILE_DLL != 0 {
			Err("File is not a CIL executable, but a class library.")?;
		}

		let magic: u16 = data.read(&mut offset)?;
		if magic != OPT_MAGIC_PE32 {
			Err("Optional header magic is not PE32.")?;
		}

		offset += 90;

		let n_data_dirs: u32 = data.read(&mut offset)?;
		if n_data_dirs as usize != DATA_DIRS_COUNT {
			Err("Number of data directories is invalid.")?;
		}

		offset += DATA_DIR_INDEX_CLI_HEADER * 8;
		let cli_rva:  u32 = data.read(&mut offset)?;
		let cli_size: u32 = data.read(&mut offset)?;

		let n_sections = n_sections as usize;
		let mut sections = Vec::with_capacity(n_sections);

		const SECTION_SIZE: usize = 40;
		for i in 0..n_sections {
			let virtual_size:    u32 = data.read_at(offset + 16)?;
			let virtual_address: u32 = data.read_at(offset + 20)?;
			let raw_address:     u32 = data.read_at(offset + 28)?;
			offset += SECTION_SIZE;

			sections.push(Section {
				virtual_size,
				virtual_address,
				raw_address,
			});
		}
		
		Ok(Header {
			cli_rva,
			cli_size,
			sections,
		})
	}
}
