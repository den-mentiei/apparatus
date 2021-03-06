use log::{debug};

use crate::Result;
use crate::error::Error;
use crate::buf::Reading;

pub fn parse_blobs(data: &[u8]) -> Result<Box<[&[u8]]>> {
	debug!("Parsing blobs...");

	let mut blobs = Vec::new();

	let mut i: usize = 0;
	while i < data.len() - 1 {
		let (blob, len) = parse_blob(&data[i..])?;
		blobs.push(blob);
		i += len;
	}

	debug!("Found {} blob(s).", blobs.len());
	Ok(blobs.into_boxed_slice())
}

pub fn parse_user_strings(data: &[u8]) -> Result<Box<[String]>> {
	// Strings in the #US (user string) heap are encoded using 16-bit Unicode
	// encodings. The count on each string is the number of bytes (not
	// characters) in the string. Furthermore, there is an additional
	// terminal byte (so all byte counts are odd, not even). This final byte
	// holds the value 1 if and only if any UTF16 character within the string
	// has any bit set in its top byte, or its low byte is any of the
	// following: 0x01-0x08, 0x0E-0x1F, 0x27, 0x2D, 0x7F.  Otherwise,
	// it holds 0. The 1 signifies Unicode characters that require handling
	// beyond that normally provided for 8-bit encoding sets.

	debug!("Parsing user strings:");

	let mut strings = Vec::with_capacity(1);
	strings.push(String::new());

	let mut i: usize = 0;
	while i < data.len() - 1 {
		let (blob, len) = parse_blob(&data[i..])?;
		if blob.len() > 0 {
			let len = blob.len() - 1;
			let wide: &[u16] = unsafe {
				std::slice::from_raw_parts(
					blob.as_ptr() as *const u16,
					len >> 1)
			};
			let s = String::from_utf16(wide)
				.map_err(|_| Error::General("User string is not a valid utf-16 string."))?;
			debug!("  `{}` (fits ascii: {})", s, blob[len] == 0);

			strings.push(s);
		}
		i += len;
	}

	debug!("Found {} user string(s).", strings.len());
	Ok(strings.into_boxed_slice())
}

// TODO(dmi): @check Add few large strings to subject.
fn parse_blob(data: &[u8]) -> Result<(&[u8], usize)> {
	let b0 = data[0];
	if b0 & 0b1000_0000 == 0 {
		let n = (b0 & 0b0111_1111) as usize;
		return Ok((&data[1..n + 1], n + 1));
	}

	if b0 & 0b1100_0000 == 0b1000_0000 {
		let x = data[1] as usize;
		let n = ((b0 & 0b0011_1111) as usize) << 8 + x;
		return Ok((&data[2..n + 2], n + 2));
	}

	if b0 & 0b1110_0000 == 0b1100_0000 {
		let x = data[1] as usize;
		let y = data[2] as usize;
		let z = data[3] as usize;
		let n = ((b0 & 0b0001_1111) as usize) << 24 + (x << 16) + (y << 8) + z;
		return Ok((&data[4..n + 4], n + 4));
	}

	Err("Incorrect blob length.")?
}
