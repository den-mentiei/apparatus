use log::{debug};

use std::fmt;

use crate::Result;
use crate::error::Error;
use crate::buf::Reading;

#[derive(Debug, PartialEq, Eq, Copy, Clone, Default)]
pub struct Guid {
	data0: u32,
	data1: u16,
	data2: u16,
	data3: [u8; 8],
}

impl Guid {
	fn parse(data: &[u8], offset: &mut usize) -> Result<Guid> {
		let data0: u32 = data.read(offset)?;
		let data1: u16 = data.read(offset)?;
		let data2: u16 = data.read(offset)?;

		let mut data3 = [0u8; 8];
		for i in 0..8 {
			data3[i] = data.read(offset)?;
		}

		Ok(Guid { data0, data1, data2, data3 })
	}
}

impl fmt::Display for Guid {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{{{:08X}-{:04X}-{:04X}-", self.data0, self.data1, self.data2)?;
		for x in &self.data3 {
			write!(f, "{:02X}", x)?;
		}
		write!(f, "}}")
	}
}

pub fn parse_guids(data: &[u8]) -> Result<Box<[Guid]>> {
	debug!("Parsing guids:");

	if data.len() & 15 !=0 {
		Err("Invalid #GUID heap size.")?;
	}

	let mut guids = Vec::with_capacity(data.len() >> 4);

	let mut offset = &mut 0usize;
	for g in data.chunks(16) {
		let guid = Guid::parse(g, offset)?;
		guids.push(guid);
		debug!("  {}", guid);
	}
	debug!("Found {} guid(s).", guids.len());

	Ok(guids.into_boxed_slice())
}
