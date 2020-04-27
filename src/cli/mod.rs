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
