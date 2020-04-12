pub fn align_up(x: usize, n: usize) -> usize {
	debug_assert!((n & (n - 1)) == 0);
	(x + (n - 1)) & !(n - 1)
}

pub fn dump(data: &[u8], n: usize) {
	assert!(n < data.len());

	const COLUMNS: usize = 16;
	const OFFSET_WIDTH: usize = std::mem::size_of::<usize>() << 1;
	
	let mut p: usize = 0;

	print!("{:1$} | ", "offset", OFFSET_WIDTH);
	for i in 0..COLUMNS {
		print!("{:02x} ", i);
	}
	println!("| data");
	for i in 0..(OFFSET_WIDTH + 3 + COLUMNS * 3 + COLUMNS + 2) {
		print!("-");
	}
	println!();

	for row in data[..n].chunks(COLUMNS) {
		print!("{1:#00$x} | ", OFFSET_WIDTH, p);
		for x in row {
			print!("{:02x} ", x);
		}
		for i in row.len()..COLUMNS {
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
}
