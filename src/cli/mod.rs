mod header;
pub use self::header::*;

mod logical_tables;
pub use self::logical_tables::*;

mod guids;
pub use self::guids::*;

mod strings;
pub use self::strings::*;

mod user_strings;
pub use self::user_strings::*;

// 	// METADATA_STANDALONESIG
// 	if (valid_mask >> METADATA_STANDALONESIG) & 1 == 1 {
// 		let len  = row_lens[t * 4];
// 		println!("StandaloneSig table with {} item(s).", len);

// 		for i in 0..len {
// 			println!("* StandaloneSig #{}", i);
// 		}

// 		t += 1;
// 	}

// 	// METADATA_EVENTMAP
// 	if (valid_mask >> METADATA_EVENTMAP) & 1 == 1 {
// 		let len  = row_lens[t * 4];
// 		println!("EventMap table with {} item(s).", len);

// 		for i in 0..len {
// 			println!("* EventMap #{}", i);
// 		}

// 		t += 1;
// 	}

// 	// METADATA_EVENT
// 	if (valid_mask >> METADATA_EVENT) & 1 == 1 {
// 		let len  = row_lens[t * 4];
// 		println!("Event table with {} item(s).", len);

// 		for i in 0..len {
// 			println!("* Event #{}", i);
// 		}

// 		t += 1;
// 	}

// 	// METADATA_PROPERTYMAP
// 	if (valid_mask >> METADATA_PROPERTYMAP) & 1 == 1 {
// 		let len  = row_lens[t * 4];
// 		println!("PropertyMap table with {} item(s).", len);

// 		for i in 0..len {
// 			println!("* PropertyMap #{}", i);
// 		}

// 		t += 1;
// 	}

// 	// METADATA_PROPERTY
// 	if (valid_mask >> METADATA_PROPERTY) & 1 == 1 {
// 		let len  = row_lens[t * 4];
// 		println!("Property table with {} item(s).", len);

// 		for i in 0..len {
// 			println!("* Property #{}", i);
// 		}

// 		t += 1;
// 	}

// 	// METADATA_METHODSEMANTICS
// 	if (valid_mask >> METADATA_METHODSEMANTICS) & 1 == 1 {
// 		let len  = row_lens[t * 4];
// 		println!("MethodSemantics table with {} item(s).", len);

// 		for i in 0..len {
// 			println!("* MethodSemantics #{}", i);
// 		}

// 		t += 1;
// 	}

// 	// METADATA_METHODIMPL
// 	if (valid_mask >> METADATA_METHODIMPL) & 1 == 1 {
// 		let len  = row_lens[t * 4];
// 		println!("MethodImpl table with {} item(s).", len);

// 		for i in 0..len {
// 			println!("* MethodImpl #{}", i);
// 		}

// 		t += 1;
// 	}

// 	// METADATA_MODULEREF
// 	if (valid_mask >> METADATA_MODULEREF) & 1 == 1 {
// 		let len  = row_lens[t * 4];
// 		println!("ModuleRef table with {} item(s).", len);

// 		for i in 0..len {
// 			println!("* ModuleRef #{}", i);
// 		}

// 		t += 1;
// 	}

// 	// METADATA_TYPESPEC
// 	if (valid_mask >> METADATA_TYPESPEC) & 1 == 1 {
// 		let len  = row_lens[t * 4];
// 		println!("TypeSpec table with {} item(s).", len);

// 		for i in 0..len {
// 			println!("* TypeSpec #{}", i);
// 		}

// 		t += 1;
// 	}

// 	// METADATA_IMPLMAP
// 	if (valid_mask >> METADATA_IMPLMAP) & 1 == 1 {
// 		let len  = row_lens[t * 4];
// 		println!("ImplMap table with {} item(s).", len);

// 		for i in 0..len {
// 			println!("* ImplMap #{}", i);
// 		}

// 		t += 1;
// 	}

// 	// METADATA_FIELDRVA
// 	if (valid_mask >> METADATA_FIELDRVA) & 1 == 1 {
// 		let len  = row_lens[t * 4];
// 		println!("FieldRVA table with {} item(s).", len);

// 		for i in 0..len {
// 			println!("* FieldRVA #{}", i);
// 		}

// 		t += 1;
// 	}

// 	// METADATA_ASSEMBLY II 22.2
// 	if (valid_mask >> METADATA_ASSEMBLY) & 1 == 1 {
// 		let len  = row_lens[t * 4];
// 		println!("Assembly table with {} item(s).", len);

// 		for i in 0..len {
// 			println!("* Assembly #{}", i);

// 			// II.23.1.1
// 			const MD5:  u32 = 0x8003;
// 			const SHA1: u32 = 0x8004;

// 			let hash_algo = data[offset..].read_u32()?;
// 			offset += 4;

// 			print!("  hash algo: ");
// 			match hash_algo {
// 				MD5  => println!("MD5"),
// 				SHA1 => println!("SHA1"),
// 				_ => Err("Unknown assembly hash algo id.")?,
// 			}

// 			let major_version    = data[offset..].read_u16()?;
// 			offset += 2;
// 			let minor_version    = data[offset..].read_u16()?;
// 			offset += 2;
// 			let build_number     = data[offset..].read_u16()?;
// 			offset += 2;
// 			let revision_number  = data[offset..].read_u16()?;
// 			offset += 2;

// 			println!("  version: {}.{}.{}.{}", major_version, minor_version, build_number, revision_number);

// 			// II.23.1.2
// 			// The assembly reference holds the full (unhashed) public key.
// 			const PUBLIC_KEY: u32 = 0x0001;
// 			// The implementation of this assembly used at runtime is
// 			// not expected to match the version seen at compile time.
// 			const RETARGETABLE: u32 = 0x0100;
// 			// Reserved (a conforming implementation of the CLI can ignore this
// 			// setting on read; some implementations might use this bit to
// 			// indicate that a CIL-to-native-code compiler should generate
// 			// CIL-to-native code map)
// 			const DISABLE_JIT_COMPILE_OPTIMIZER: u32 = 0x4000;
// 			// Reserved (a conforming implementation of the CLI can ignore this
// 			// setting on read; some implementations might use this bit to
// 			// indicate that a CIL-to-native-code compiler should generate
// 			// CIL-to-native code map)
// 			const ENABLE_JIT_COMPILE_TRACKING: u32 = 0x8000;
			
// 			let flags = data[offset..].read_u32()?;
// 			offset += 4;

// 			print!("  flags: {:#0x} -> ", flags);
// 			if flags & PUBLIC_KEY != 0 {
// 				print!("public-key ");
// 			}
// 			if flags & RETARGETABLE != 0 {
// 				print!("retargetable ");
// 			}
// 			if flags & DISABLE_JIT_COMPILE_OPTIMIZER != 0 {
// 				print!("disable-jit-compile-optimizer ");
// 			}
// 			if flags & ENABLE_JIT_COMPILE_TRACKING != 0 {
// 				print!("enable-jit-compile-tracking");
// 			}
// 			println!();

// 			let key_bi = read_idx(data, offset, bi_size)?;
// 			offset += bi_size;
// 			println!("  public key: {:#0x}", key_bi);

// 			let name_si = read_idx(data, offset, si_size)?;
// 			offset += si_size;
// 			println!("  name index: {:#0x}", name_si);

// 			let culture_si = read_idx(data, offset, si_size)?;
// 			offset += si_size;
// 			println!("  culture index: {:#0x}", culture_si);
// 		}

// 		t += 1;
// 	}

// 	// METADATA_ASSEMBLYPROCESSOR
// 	if (valid_mask >> METADATA_ASSEMBLYPROCESSOR) & 1 == 1 {
// 		let len  = row_lens[t * 4];
// 		println!("AssemblyProcessor table with {} item(s).", len);

// 		for i in 0..len {
// 			println!("* AssemblyProcessor #{}", i);
// 		}

// 		t += 1;
// 	}

// 	// METADATA_ASSEMBLYOS
// 	if (valid_mask >> METADATA_ASSEMBLYOS) & 1 == 1 {
// 		let len  = row_lens[t * 4];
// 		println!("AssemblyOS table with {} item(s).", len);

// 		for i in 0..len {
// 			println!("* AssemblyOS #{}", i);
// 		}

// 		t += 1;
// 	}

// 	// METADATA_ASSEMBLYREF II.22.5
// 	if (valid_mask >> METADATA_ASSEMBLYREF) & 1 == 1 {
// 		let len  = row_lens[t * 4];
// 		println!("AssemblyRef table with {} item(s).", len);

// 		for i in 0..len {
// 			println!("* AssemblyRef #{}", i);

// 			let major_version    = data[offset..].read_u16()?;
// 			offset += 2;
// 			let minor_version    = data[offset..].read_u16()?;
// 			offset += 2;
// 			let build_number     = data[offset..].read_u16()?;
// 			offset += 2;
// 			let revision_number  = data[offset..].read_u16()?;
// 			offset += 2;

// 			println!("  version: {}.{}.{}.{}", major_version, minor_version, build_number, revision_number);

// 			// II.23.1.2
// 			// The assembly reference holds the full (unhashed) public key.
// 			const PUBLIC_KEY: u32 = 0x0001;
// 			// The implementation of this assembly used at runtime is
// 			// not expected to match the version seen at compile time.
// 			const RETARGETABLE: u32 = 0x0100;
// 			// Reserved (a conforming implementation of the CLI can ignore this
// 			// setting on read; some implementations might use this bit to
// 			// indicate that a CIL-to-native-code compiler should generate
// 			// CIL-to-native code map)
// 			const DISABLE_JIT_COMPILE_OPTIMIZER: u32 = 0x4000;
// 			// Reserved (a conforming implementation of the CLI can ignore this
// 			// setting on read; some implementations might use this bit to
// 			// indicate that a CIL-to-native-code compiler should generate
// 			// CIL-to-native code map)
// 			const ENABLE_JIT_COMPILE_TRACKING: u32 = 0x8000;
			
// 			let flags = data[offset..].read_u32()?;
// 			offset += 4;

// 			print!("  flags: {:#0x} -> ", flags);
// 			if flags & PUBLIC_KEY != 0 {
// 				print!("public-key ");
// 			}
// 			if flags & RETARGETABLE != 0 {
// 				print!("retargetable ");
// 			}
// 			if flags & DISABLE_JIT_COMPILE_OPTIMIZER != 0 {
// 				print!("disable-jit-compile-optimizer ");
// 			}
// 			if flags & ENABLE_JIT_COMPILE_TRACKING != 0 {
// 				print!("enable-jit-compile-tracking");
// 			}
// 			println!();

// 			let key_bi = read_idx(data, offset, bi_size)?;
// 			offset += bi_size;
// 			println!("  public key: {:#0x}", key_bi);

// 			let name_si = read_idx(data, offset, si_size)?;
// 			offset += si_size;
// 			println!("  name index: {:#0x}", name_si);

// 			let culture_si = read_idx(data, offset, si_size)?;
// 			offset += si_size;
// 			println!("  culture index: {:#0x}", culture_si);

// 			let hash_bi = read_idx(data, offset, bi_size)?;
// 			offset += bi_size;
// 			println!("  hash: {:#0x}", hash_bi);
// 		}

// 		t += 1;
// 	}

// 	// METADATA_ASSEMBLYREFPROCESSOR
// 	if (valid_mask >> METADATA_ASSEMBLYREFPROCESSOR) & 1 == 1 {
// 		let len  = row_lens[t * 4];
// 		println!("AssemblyRefProcessor table with {} item(s).", len);

// 		for i in 0..len {
// 			println!("* AssemblyRefProcessor #{}", i);
// 		}

// 		t += 1;
// 	}

// 	// METADATA_ASSEMBLYREFOS
// 	if (valid_mask >> METADATA_ASSEMBLYREFOS) & 1 == 1 {
// 		let len  = row_lens[t * 4];
// 		println!("AssemblyRefOS table with {} item(s).", len);

// 		for i in 0..len {
// 			println!("* AssemblyRefOS #{}", i);
// 		}

// 		t += 1;
// 	}

// 	// METADATA_FILE
// 	if (valid_mask >> METADATA_FILE) & 1 == 1 {
// 		let len  = row_lens[t * 4];
// 		println!("File table with {} item(s).", len);

// 		for i in 0..len {
// 			println!("* File #{}", i);
// 		}

// 		t += 1;
// 	}

// 	// METADATA_EXPORTEDTYPE
// 	if (valid_mask >> METADATA_EXPORTEDTYPE) & 1 == 1 {
// 		let len  = row_lens[t * 4];
// 		println!("ExportedType table with {} item(s).", len);

// 		for i in 0..len {
// 			println!("* ExportedType #{}", i);
// 		}

// 		t += 1;
// 	}

// 	// METADATA_MANIFESTRESOURCE
// 	if (valid_mask >> METADATA_MANIFESTRESOURCE) & 1 == 1 {
// 		let len  = row_lens[t * 4];
// 		println!("ManifestResource table with {} item(s).", len);

// 		for i in 0..len {
// 			println!("* ManifestResource #{}", i);
// 		}

// 		t += 1;
// 	}

// 	// METADATA_NESTEDCLASS
// 	if (valid_mask >> METADATA_NESTEDCLASS) & 1 == 1 {
// 		let len  = row_lens[t * 4];
// 		println!("NestedClass table with {} item(s).", len);

// 		for i in 0..len {
// 			println!("* NestedClass #{}", i);
// 		}

// 		t += 1;
// 	}

// 	// METADATA_GENERICPARAM
// 	if (valid_mask >> METADATA_GENERICPARAM) & 1 == 1 {
// 		let len  = row_lens[t * 4];
// 		println!("GenericParam table with {} item(s).", len);

// 		for i in 0..len {
// 			println!("* GenericParam #{}", i);
// 		}

// 		t += 1;
// 	}

// 	// METADATA_METHODSPEC
// 	if (valid_mask >> METADATA_METHODSPEC) & 1 == 1 {
// 		let len  = row_lens[t * 4];
// 		println!("MethodSpec table with {} item(s).", len);

// 		for i in 0..len {
// 			println!("* MethodSpec #{}", i);
// 		}

// 		t += 1;
// 	}

// 	// METADATA_GENERICPARAMCONSTRAINT
// 	if (valid_mask >> METADATA_GENERICPARAMCONSTRAINT) & 1 == 1 {
// 		let len  = row_lens[t * 4];
// 		println!("GenericParamConstraint table with {} item(s).", len);

// 		for i in 0..len {
// 			println!("* GenericParamConstraint #{}", i);
// 		}

// 		t += 1;
// 	}

// 	return Ok(());

// 	fn read_idx(data: &[u8], offset: usize, index_size: usize) -> Result<usize> {
// 		if index_size == 2 {
// 			Ok(data[offset..].read_u16()? as usize)
// 		} else {
// 			Ok(data[offset..].read_u32()? as usize)
// 		}
// 	}

// 	fn size_for_big_index(n: usize) -> usize { 1 << (16 - log2(n)) }
// 	fn log2(x: usize) -> usize { if x == 0 { 0 } else { 64usize - x.leading_zeros() as usize } }
// }
