#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_assignments)]
#![allow(unused_mut)]

extern crate log;

mod buf;
mod cli;
mod error;
mod logging;
mod pe;
mod utils;

use std::path::Path;
use std::convert::TryFrom;

use log::{trace, debug, info};

use buf::Reading;
use error::Result;
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

	let pe_header = Header::parse(data)?;
	// debug!("{:#?}", pe_header);

	let cli_offset = pe_header.rva2offset(pe_header.cli_rva as usize).ok_or("Failed to convert CLI header RVA.")?;
	if cli_offset >= data.len() {
		Err("CLI header RVA is wrong.")?;
	}

	let cli = &data[cli_offset..cli_offset + pe_header.cli_size as usize];
	let cli_header = cli::Header::parse(cli, &pe_header)?;
	// debug!("{:#?}", cli_header);

	let metadata_offset = pe_header.rva2offset(cli_header.metadata_rva as usize).ok_or("Failed to convert CLI metadata RVA.")?;
	if metadata_offset >= data.len() {
		Err("CLI metadata RVA is wrong.")?;
	}

	let metadata = &data[metadata_offset..metadata_offset + cli_header.metadata_size as usize];
	let cli_metadata = cli::Metadata::parse(metadata)?;
	// debug!("{:#?}", cli_metadata);

	if let Some(guids) = cli_metadata.guids {
		// cli::dump_guids(guids)?;
	}
	if let Some(strings) = cli_metadata.strings {
		// cli::debug_strings(strings)?;
	}
	if let Some(user_strings) = cli_metadata.user_strings {
		// cli::debug_user_strings(user_strings)?;
	}

	if let Some(logical_tables) = cli_metadata.logical_tables {
		trace!("Parsing logical tables...");
		let header = cli::Tables::parse(logical_tables)?;
		let rows = &logical_tables[header.size..];
		let rows = cli::TableRows::parse(&header, rows)?;
		// debug!("{:#?}", rows);

		let ep = cli::MetadataToken::try_from(cli_header.ep_token)?;
		debug!("Entry point: {:?}:{:?}", ep.table_index(), ep.row_index());
		if ep.table_index() != cli::METADATA_METHOD_DEF {
			Err("Unsupported entry-point type (non-method).")?;
		}

		let main = &rows.method_defs[ep.row_index()];
		debug!("main: {:?}", main);
		let main_offset = pe_header.rva2offset(main.rva as usize).ok_or("Failed to convert main RVA.")?;

		let method_data = &data[main_offset..];

		let mut offset = &mut 0usize;
		let b: u8 = method_data.read(offset)?;
		match b & 0b11 {
			0x2 => {
				let byte_size = (b >> 2) as usize;
				debug!("Method is CorILMethod_TinyFormat: {} byte(s).", byte_size);
				let il = &method_data[1..1 + byte_size];
				dump(il, il.len());
			},
			0x3 => {
				debug!("Method is CorILMethod_FatFormat.");
				unimplemented!();
			},
			_ => Err("Invalid method header.")?,
		}
	}

	Ok(())
}
