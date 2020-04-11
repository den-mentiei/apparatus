#![allow(dead_code)]
#![allow(unused_variables)]

use std::fs::File;
use std::io::Read;
use std::path::Path;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Result<T, E = Error> = std::result::Result<T, E>;

const SUBJECT: &str = "subject\\bin\\Debug\\netcoreapp3.1\\subject.exe";

fn main() -> Result<()> {	
	println!("Hello, sailor!");
	let path = std::env::current_dir()?;
	println!("The current directory is `{}`.", path.display());
	println!("Subject: `{}`.", SUBJECT);

	let data = read_whole_file(Path::new(SUBJECT))?;
	println!("Subject size: {} bytes.", data.len());

	Ok(())
}

fn read_whole_file(path: &Path) -> Result<Box<[u8]>> {
	let mut f   = File::open(path)?;
	let mut buf = Vec::new();
	f.read_to_end(&mut buf)?;
	Ok(buf.into_boxed_slice())
}
