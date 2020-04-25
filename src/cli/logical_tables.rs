use log::{debug};

use crate::Result;
use crate::error::Error;
use crate::buf::Reading;

// II.24.2.6: The physical representation of a row cell e at a
// column with type C is defined as follows: 
// - If e is a constant, it is stored using the number of bytes as
// specified for its column type C (i.e., a 2-bit mask of type
// PropertyAttributes).
// - If e is an index into the GUID heap, 'blob', or String heap,
// it is stored using the number of bytes as defined in the
// HeapSizes field.
//- If e is a simple index into a table with index i, it is stored
// using 2 bytes if table i has less than 2^16 rows, otherwise it
// is stored using 4 bytes.
// - If e is a coded index that points into table ti out of n
// possible tables t0, ...tn-1, then it is stored as
// e << (log n) | tag{ t0, ...tn-1}[ti] using 2 bytes if
// the maximum number of rows of tables t0, ...tn-1, is
// less than 2^(16 - (log n)), and using 4 bytes otherwise.
// The family of finite maps tag {t0, ...tn-1} is defined below.
// Note that decoding a physical row requires the inverse of this
// mapping. [For example, the Parent column of the Constant table
// indexes a row in the Field, Param, or Property tables.  The
// actual table is encoded into the low 2 bits of the number,
// using the values: 0 => Field, 1 => Param, 2 => Property.The
// remaining bits hold the actual row number being indexed.  For
// example, a value of 0x321, indexes row number 0xC8 in the Param
// table.]

// Taken from ECMA II.22
const METADATA_MODULE:                 usize = 0x00;
const METADATA_TYPEREF:                usize = 0x01;
const METADATA_TYPEDEF:                usize = 0x02;
const METADATA_FIELD:                  usize = 0x04;
const METADATA_METHODDEF:              usize = 0x06;
const METADATA_PARAM:                  usize = 0x08;
const METADATA_INTERFACEIMPL:          usize = 0x09;
const METADATA_MEMBERREF:              usize = 0x0A;
const METADATA_CONSTANT:               usize = 0x0B;
const METADATA_CUSTOMATTRIBUTE:        usize = 0x0C;
const METADATA_FIELDMARSHAL:           usize = 0x0D;
const METADATA_DECLSECURITY:           usize = 0x0E;
const METADATA_CLASSLAYOUT:            usize = 0x0F;
const METADATA_FIELDLAYOUT:            usize = 0x10;
const METADATA_STANDALONESIG:          usize = 0x11;
const METADATA_EVENTMAP:               usize = 0x12;
const METADATA_EVENT:                  usize = 0x14;
const METADATA_PROPERTYMAP:            usize = 0x15;
const METADATA_PROPERTY:               usize = 0x17;
const METADATA_METHODSEMANTICS:        usize = 0x18;
const METADATA_METHODIMPL:             usize = 0x19;
const METADATA_MODULEREF:              usize = 0x1A;
const METADATA_TYPESPEC:               usize = 0x1B;
const METADATA_IMPLMAP:                usize = 0x1C;
const METADATA_FIELDRVA:               usize = 0x1D;
const METADATA_ASSEMBLY:               usize = 0x20;
const METADATA_ASSEMBLYPROCESSOR:      usize = 0x21;
const METADATA_ASSEMBLYOS:             usize = 0x22;
const METADATA_ASSEMBLYREF:            usize = 0x23;
const METADATA_ASSEMBLYREFPROCESSOR:   usize = 0x24;
const METADATA_ASSEMBLYREFOS:          usize = 0x25;
const METADATA_FILE:                   usize = 0x26;
const METADATA_EXPORTEDTYPE:           usize = 0x27;
const METADATA_MANIFESTRESOURCE:       usize = 0x28;
const METADATA_NESTEDCLASS:            usize = 0x29;
const METADATA_GENERICPARAM:           usize = 0x2A;
const METADATA_METHODSPEC:             usize = 0x2B;
const METADATA_GENERICPARAMCONSTRAINT: usize = 0x2C;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum IndexSize {
	/// 2-byte index.
	U16,
	/// 4-byte index.
	U32,
}

#[derive(Copy, Clone)]
pub struct Tables {
	pub string_index_size: IndexSize,
	pub guid_index_size:   IndexSize,
	pub blob_index_size:   IndexSize,
	pub lens: [u32; 64],
	pub size: usize,
}

impl Tables {
	pub fn parse(data: &[u8]) -> Result<Tables> {
		let mut offset = &mut 0usize;

		// Reserverd1, major version, minor version.
		*offset += 6;
		
		// The HeapSizes field is a bitvector that encodes the width of
		// indexes into the various heaps. If bit 0 is set, indexes into
		// the #String heap are 4 bytes wide; if bit 1 is set, indexes
		// into the #GUID heap are 4 bytes wide; if bit 2 is set, indexes
		// into the #Blob heap are 4 bytes wide. Conversely, if the
		// HeapSize bit for a particular heap is not set, indexes into
		// that heap are 2 bytes wide.
		let heap_sizes: u8 = data.read(offset)?;
		debug!("Heap sizes: {:#010b}", heap_sizes);

		let string_index_size = if heap_sizes & 0x01 == 0 { IndexSize::U16 } else { IndexSize::U32 };
		let guid_index_size   = if heap_sizes & 0x02 == 0 { IndexSize::U16 } else { IndexSize::U32 };
		let blob_index_size   = if heap_sizes & 0x04 == 0 { IndexSize::U16 } else { IndexSize::U32 };

		// Reserved2
		*offset += 1;
		
		// The Valid field is a 64-bit bitvector that has a specific bit
		// set for each table that is stored in the stream; the mapping of
		// tables to indexes is given at the start of II.22.
		let valid_mask: u64 = data.read(offset)?;
		let n = valid_mask.count_ones() as usize;
		debug!("Valid mask: {:#066b} -> {} table(s).", valid_mask, n);

		// Sorted.
		*offset += 8;
		
		let mut lens = [0u32; 64];
		for i in 0..lens.len() {
			if (valid_mask >> i) & 1 == 1 {
				lens[i] = data.read(offset)?;
				debug!("Table #{} has {:#0x} item(s).", i, lens[i]);
			}
		}

		let size = *offset;
		
		Ok(Tables {
			string_index_size,
			guid_index_size,
			blob_index_size,
			lens,
			size,
		})
	}
}

#[derive(Debug, PartialEq, Clone, Default)]
pub struct StringIndex(u32);

#[derive(Debug, PartialEq, Clone, Default)]
pub struct GuidIndex(u32);

impl StringIndex {
	fn read(header: &Tables, data: &[u8], offset: &mut usize) -> Result<Self> {
		let i = match header.string_index_size {
			IndexSize::U16 => StringIndex(data.read::<u16>(offset)? as u32),
			IndexSize::U32 => StringIndex(data.read::<u32>(offset)? as u32),
		};
		Ok(i)
	}
}

impl GuidIndex {
	fn read(header: &Tables, data: &[u8], offset: &mut usize) -> Result<Self> {
		let i = match header.guid_index_size {
			IndexSize::U16 => GuidIndex(data.read::<u16>(offset)? as u32),
			IndexSize::U32 => GuidIndex(data.read::<u32>(offset)? as u32),
		};
		Ok(i)
	}
}

#[derive(Debug, PartialEq, Clone, Default)]
pub struct TableRows {
	modules: Box<[Module]>,
}

impl TableRows {
	pub fn parse(header: &Tables, data: &[u8]) -> Result<Self> {
		let mut offset = &mut 0;
		
		let modules = Module::parse_many(header, data, offset)?;
		
		Ok(TableRows {
			modules,
		})
	}
}

/// II.22.30
#[derive(Debug, PartialEq, Clone, Default)]
pub struct Module {
	/// Module name.
	pub name: StringIndex,
	/// Simply a Guid used to distinguish between two
	/// versions of the same module.
	pub mvid: GuidIndex,
}

impl Module {
	fn parse_many(header: &Tables, data: &[u8], offset: &mut usize) -> Result<Box<[Module]>> {
		let n = header.lens[METADATA_MODULE] as usize;
		let mut result = Vec::with_capacity(n);

		for i in 0..n {
			let generation: u16 = data.read(offset)?;
			if generation != 0 {
				Err("Module has invalid generation.")?;
			}

			let name = StringIndex::read(header, data, offset)?;
			let mvid = GuidIndex::read(header, data, offset)?;

			let enc_id: u16 = data.read(offset)?;
			if enc_id != 0 {
				Err("Module.EncId is not zero.")?;
			}
			let enc_base_id: u16 = data.read(offset)?;
			if enc_base_id != 0 {
				Err("Module.EncBaseId is not zero.")?;
			}

			result.push(Module { name, mvid })
		}

		Ok(result.into_boxed_slice())
	}
}
