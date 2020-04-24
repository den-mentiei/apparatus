use log::{debug};

use crate::Result;
use crate::error::Error;
use crate::buf::Reading;

pub fn dump_guids(data: &[u8]) -> Result<()> {
	if data.len() & 15 !=0 {
		Err("Invalid #GUID heap size.")?;
	}

	// TODO(dmi): @shortcut It should use debug! and only
	// format guids if debug log levle is enabled.
	
	debug!("Available guids:");
	let mut offset = &mut 0usize;
	for g in data.chunks(16) {
		let data1: u32 = g.read(offset)?;
		let data2: u16 = g.read(offset)?;
		let data3: u16 = g.read(offset)?;

		print!("  {{{:08X}-{:04X}-{:04X}-", data1, data2, data3);
		for x in &g[8..] {
			print!("{:02X}", x)
		}
		*offset += 8;
		println!("}}");
	}
	debug!("Total: {} guid(s).", data.len() >> 4);

	Ok(())
}
