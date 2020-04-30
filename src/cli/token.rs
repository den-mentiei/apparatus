use std::convert::TryFrom;

use crate::Result;
use crate::error::Error;
use crate::cli::constants::*;

// II.22
#[derive(Debug, PartialEq, Eq, Copy, Clone, Default)]
pub struct MetadataToken(u32);

impl TryFrom<u32> for MetadataToken {
	type Error = crate::error::Error;

	fn try_from(x: u32) -> Result<Self> {
		let idx = table_index(x);
		if idx >= METADATA_MODULE && idx <= METADATA_GENERIC_PARAM_CONSTRAINT {
			Ok(MetadataToken(x))
		} else {
			Err(Error::General("Unknown metadata table in possible token."))
		}
	}
}

impl MetadataToken {
	pub fn table_index(&self) -> usize {
		table_index(self.0)
	}

	pub fn row_index(&self) -> usize {
		((self.0 & 0xFFFFFF) - 1) as usize
	}
}

fn table_index(x: u32) -> usize {
	(x >> 24) as usize
}
