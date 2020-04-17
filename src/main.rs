#![allow(dead_code)]
#![allow(unused_variables)]

#![allow(unused_imports)]
#![allow(unused_assignments)]
#![allow(unused_mut)]

extern crate log;

mod buf;
mod cli_header;
mod logging;
mod pe;
mod utils;

use std::path::Path;

use log::{info, trace};

use aps::Result;
use buf::Reading;
use pe::Header;
use utils::{read_whole_file, dump, align_up, os_is_64};

const SUBJECT: &str = "subject\\bin\\Debug\\netcoreapp3.1\\subject.dll";

fn main() -> Result<()> {	
	logging::init();
	
	info!("Hello, sailor!");
	let path = std::env::current_dir()?;
	info!("The current directory is `{}`.", path.display());
	info!("Subject: `{}`.", SUBJECT);

	let data = &*read_whole_file(Path::new(SUBJECT))?;
	info!("Subject size: {} bytes.", data.len());

	let header = Header::parse(data)?;

	let cli_offset = header.rva2offset(header.cli_rva as usize).ok_or("Failed to convert CLI header RVA.")?;
	if cli_offset >= data.len() {
		Err("CLI header RVA is wrong.")?;
	}

	let cli = &data[cli_offset..cli_offset + header.cli_size as usize];
	dump(cli, 64);
	
	Ok(())
}
