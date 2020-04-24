use log::{debug};

use crate::Result;
use crate::error::Error;
use crate::buf::Reading;

// TODO(dmi): @incomplete It should return a vector of strings,
// so we can reference it while parsing other tables.

pub fn debug_strings(data: &[u8]) -> Result<()> {
	debug_assert!(data.len() >= 1);
	debug_assert!(data[0] == 0);

	debug!("Available strings:");
	
	let mut n: usize = 0;
	for s in data[1..].split(|c| *c == 0) {
		n += 1;

		let s = std::str::from_utf8(s)
			.map_err(|_| Error::General("Found a string that is not a valid utf-8 string."))?;
		if s.len() > 0 {
			debug!("  `{}`", s);
		}
	}

	debug!("Total: {} string(s).", n);

	Ok(())
}
