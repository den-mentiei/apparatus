use std::fs::File;
use std::io::Read;
use std::path::Path;

use crate::Result;

pub fn align_up(x: usize, n: usize) -> usize {
	debug_assert!((n & (n - 1)) == 0);
	(x + (n - 1)) & !(n - 1)
}

pub fn read_whole_file(path: &Path) -> Result<Box<[u8]>> {
	let mut f   = File::open(path)?;
	let mut buf = Vec::new();
	f.read_to_end(&mut buf)?;
	Ok(buf.into_boxed_slice())
}

pub fn dump(data: &[u8], n: usize) {
	assert!(n <= data.len());

	const COLUMNS: usize = 16;
	const OFFSET_WIDTH: usize = std::mem::size_of::<usize>() << 1;
	
	let mut p: usize = 0;

	header();
	dash();

	for row in data[..n].chunks(COLUMNS) {
		print!("{1:#00$x} | ", OFFSET_WIDTH, p);
		for x in row {
			print!("{:02x} ", x);
		}
		for _ in row.len()..COLUMNS {
			print!("__ ");
		}
		print!("| ");
		for x in row {
			if x.is_ascii_alphanumeric() || x.is_ascii_punctuation() || x.is_ascii_graphic() {
				print!("{}", *x as char);
			} else {
				print!(".");
			}
		}
		println!();

		p += COLUMNS;
	}

	dash();

	fn header() {
		print!("{:1$} | ", "offset", OFFSET_WIDTH);
		for i in 0..COLUMNS {
			print!("{:02x} ", i);
		}
		println!("| data");
	}
	
	fn dash() {
		for _ in 0..(OFFSET_WIDTH + 3 + COLUMNS * 3 + COLUMNS + 2) {
			print!("-");
		}
		println!();
	}
}

#[cfg(target_pointer_width = "32")]
pub fn os_is_64() -> bool { false }

#[cfg(target_pointer_width = "64")]
pub fn os_is_64() -> bool { true }
