use crate::Result;
use crate::error::Error;
use crate::buf::Reading;

pub fn parse_strings(data: & [u8]) -> Result<Box<[&str]>> {
	if data.len() < 1 || data[0] != 0 {
		Err("Strings heap is invalid.")?;
	}

	let mut strings = Vec::new();
	
	for s in data[1..].split(|c| *c == 0) {
		strings.push(
			std::str::from_utf8(s)
			.map_err(|_| Error::General("Found a string that is not a valid utf-8 string."))?);
	}

	Ok(strings.into_boxed_slice())
}
