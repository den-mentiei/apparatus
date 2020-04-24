use log::{trace};

use crate::Result;
use crate::error::Error;
use crate::buf::Reading;

// II.24.2.6

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

#[derive(Copy, Clone)]
pub struct Tables {
	pub string_index_bytes: usize,
	pub guid_index_bytes:   usize,
	pub blob_index_bytes:   usize,
	pub lens: [u32; 64],
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
		trace!("Heap sizes: {:#010b}", heap_sizes);

		let string_index_bytes: usize = if heap_sizes & 0x01 == 0 { 2 } else { 4 };
		let guid_index_bytes:   usize = if heap_sizes & 0x02 == 0 { 2 } else { 4 };
		let blob_index_bytes:   usize = if heap_sizes & 0x04 == 0 { 2 } else { 4 };

		// Reserved2
		*offset += 1;
		
		// The Valid field is a 64-bit bitvector that has a specific bit
		// set for each table that is stored in the stream; the mapping of
		// tables to indexes is given at the start of II.22.
		let valid_mask: u64 = data.read(offset)?;
		let n = valid_mask.count_ones() as usize;
		trace!("Valid mask: {:#066b} -> {} table(s).", valid_mask, n);

		// Sorted.
		*offset += 8;
		
		let mut lens = [0u32; 64];
		for i in 0..lens.len() {
			if (valid_mask >> i) & 1 == 1 {
				lens[i] = data.read(offset)?;
				trace!("Table #{} has {:#0x} item(s).", i, lens[i]);
			}
		}
		
		Ok(Tables {
			string_index_bytes,
			guid_index_bytes,
			blob_index_bytes,
			lens,
		})
	}
}
