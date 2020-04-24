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

// macro_rules! max {
// 	($x:expr) => ($x);
// 	($x:expr, $($xs:expr),+) => {
// 		{
// 			use std::cmp::max;
// 			max($x, max!($($xs),+))
// 		}
// 	};
// }

// II.24.2.6
// fn read_logical_tables(data: &[u8]) -> Result<()> {
// 	let tables   = &data[24 + n * 4..];

// 	// II.24.2.6: The physical representation of a row cell e at a
// 	// column with type C is defined as follows: 
// 	// - If e is a constant, it is stored using the number of bytes as
// 	// specified for its column type C (i.e., a 2-bit mask of type
// 	// PropertyAttributes).
// 	// - If e is an index into the GUID heap, 'blob', or String heap,
// 	// it is stored using the number of bytes as defined in the
// 	// HeapSizes field.
// 	//- If e is a simple index into a table with index i, it is stored
// 	// using 2 bytes if table i has less than 2^16 rows, otherwise it
// 	// is stored using 4 bytes.
// 	// - If e is a coded index that points into table ti out of n
// 	// possible tables t0, ...tn-1, then it is stored as
// 	// e << (log n) | tag{ t0, ...tn-1}[ti] using 2 bytes if
// 	// the maximum number of rows of tables t0, ...tn-1, is
// 	// less than 2^(16 - (log n)), and using 4 bytes otherwise.
// 	// The family of finite maps tag {t0, ...tn-1} is defined below.
// 	// Note that decoding a physical row requires the inverse of this
// 	// mapping. [For example, the Parent column of the Constant table
// 	// indexes a row in the Field, Param, or Property tables.  The
// 	// actual table is encoded into the low 2 bits of the number,
// 	// using the values: 0 => Field, 1 => Param, 2 => Property.The
// 	// remaining bits hold the actual row number being indexed.  For
// 	// example, a value of 0x321, indexes row number 0xC8 in the Param
// 	// table.]
	
// 	// METADATA_MODULE II.22.30
// 	if (valid_mask >> METADATA_MODULE) & 1 == 1 {
// 		let len  = row_lens[t * 4];
// 		println!("Module table with {} item(s).", len);

// 		for i in 0..len {
// 			println!("* Module #{}", i);
			
// 			let generation = data[offset..].read_u16()?;
// 			if generation != 0 {
// 				Err("Module has invalid generation.")?;
// 			}
// 			offset += 2;

// 			let name_si = read_idx(data, offset, si_size)?;
// 			offset += si_size;
// 			let mvid_gi = read_idx(data, offset, gi_size)?;
// 			offset += gi_size;

// 			// EncId & EncBaseId.
// 			offset += gi_size * 2;

// 			println!("  name index: {:#0x}", name_si);
// 			println!("  mvid index: {:#0x}", mvid_gi);
// 		}
		
// 		t += 1;
// 	}

// 	// METADATA_TYPEREF II.22.30
// 	if (valid_mask >> METADATA_TYPEREF) & 1 == 1 {
// 		let len  = row_lens[t * 4];
// 		println!("TypeRef table with {} item(s).", len);

// 		for i in 0..len {
// 			println!("* TypeRef #{}", i);

// 			// II.24.2.6:
// 			const TAG_MASK: usize = 0b11;
// 			const RESOLUTION_SCOPE_MODULE:       usize = 0;
// 			const RESOLUTION_SCOPE_MODULE_REF:   usize = 1;
// 			const RESOLUTION_SCOPE_ASSEMBLY_REF: usize = 2;
// 			const RESOLUTION_SCOPE_TYPE_REF:     usize = 3;
			
// 			let max_len = max!(
// 				table_lens[METADATA_MODULE],
// 				table_lens[METADATA_MODULEREF],
// 				table_lens[METADATA_ASSEMBLYREF],
// 				table_lens[METADATA_TYPEREF]) as usize;
// 			let size  = if max_len < size_for_big_index(4) { 2 } else { 4 };
// 			let shift = log2(TAG_MASK);

// 			let scope = read_idx(data, offset, size)?;
// 			offset += size;
// 			let type_name_si = read_idx(data, offset, si_size)?;
// 			offset += si_size;
// 			let type_namespace_si = read_idx(data, offset, si_size)?;
// 			offset += si_size;

// 			print!("-> ");
// 			match scope & TAG_MASK {
// 				RESOLUTION_SCOPE_MODULE => print!("Module"),
// 				RESOLUTION_SCOPE_MODULE_REF => print!("ModuleRef"),
// 				RESOLUTION_SCOPE_ASSEMBLY_REF => print!("AssemblyRef"),
// 				RESOLUTION_SCOPE_TYPE_REF => print!("TypeRef"),
// 				_ => Err("Invalid ResolutionScope tag.")?,
// 			};
// 			print!(" {:#0x}, ", scope >> shift);
// 			println!("  name index: {:#0x}, namespace index: {:#0x}", type_name_si, type_namespace_si);
// 		}

// 		t += 1;
// 	}

// 	// METADATA_TYPEDEF II.22.37
// 	if (valid_mask >> METADATA_TYPEDEF) & 1 == 1 {
// 		let len  = row_lens[t * 4];
// 		println!("TypeDef table with {} item(s).", len);

// 		for i in 0..len {
// 			println!("* TypeDef #{}", i);

// 			// TODO(dmi): @incomplete Parse flags.
// 			let flags = data[offset..].read_u32()?;
// 			offset += 4;
// 			println!("  flags: {:#0x}", flags);

// 			let type_name_si = read_idx(data, offset, si_size)?;
// 			offset += si_size;
// 			let type_namespace_si = read_idx(data, offset, si_size)?;
// 			offset += si_size;

// 			println!("  type name index: {:#0x}", type_name_si);
// 			println!("  type namespace index: {:#0x}", type_namespace_si);

// 			// II.24.2.6:
// 			const TAG_MASK: usize = 0b11;
// 			const TYPEDEF:  usize = 0;
// 			const TYPEREF:  usize = 1;
// 			const TYPESPEC: usize = 2;

// 			let max_len = max!(
// 				table_lens[METADATA_TYPEDEF],
// 				table_lens[METADATA_TYPEREF],
// 				table_lens[METADATA_TYPESPEC]) as usize;
// 			let size  = if max_len < size_for_big_index(3) { 2 } else { 4 };
// 			let shift = log2(TAG_MASK);

// 			let extends = read_idx(data, offset, size)?;
// 			offset += size;

// 			print!("  extends ");
// 			match extends & TAG_MASK {
// 				TYPEDEF => print!("TypeDef"),
// 				TYPEREF => print!("TypeRef"),
// 				TYPESPEC => print!("TypeSpec"),
// 				_ => Err("Invalid TypeDefOrRef tag.")?,
// 			};
// 			println!(" {:#0x}", extends >> shift);

// 			let fi_size = if table_lens[METADATA_FIELD] <= 0xFFFF { 2 } else { 4 };
// 			let first_field_idx = read_idx(data, offset, fi_size)?;
// 			offset += fi_size;
// 			println!("  first field index: {:#0x}", first_field_idx);

// 			let mi_size = if table_lens[METADATA_METHODDEF] <= 0xFFFF { 2 } else { 4 };
// 			let first_method_idx = read_idx(data, offset, mi_size)?;
// 			offset += mi_size;
// 			println!("  first method index: {:#0x}", first_method_idx);
// 		}

// 		t += 1;
// 	}

// 	// METADATA_FIELD II.22.15
// 	if (valid_mask >> METADATA_FIELD) & 1 == 1 {
// 		let len  = row_lens[t * 4];
// 		println!("Field table with {} item(s).", len);

// 		for i in 0..len {
// 			println!("* Field #{}", i);

// 			let flags = data[offset..].read_u16()?;
// 			println!("  flags: {:#0x}", flags);
// 			offset += 2;
			
// 			let name_si = read_idx(data, offset, si_size)?;
// 			offset += si_size;
// 			println!("  name index: {:#0x}", name_si);

// 			let signature_bi = read_idx(data, offset, bi_size)?;
// 			offset += bi_size;
// 			println!("  signature index: {:#0x}", signature_bi);
// 		}

// 		t += 1;
// 	}
	
// 	// METADATA_METHODDEF II.22.26
// 	if (valid_mask >> METADATA_METHODDEF) & 1 == 1 {
// 		let len  = row_lens[t * 4];
// 		println!("MethodDef table with {} item(s).", len);

// 		for i in 0..len {
// 			println!("* MethodDef #{}", i);

// 			// TODO(dmi): @next Finally! Can find entry point method now
// 			// and rush to get its IL-code.
// 			let rva = data[offset..].read_u32()?;
// 			offset += 4;
// 			println!("  rva: {:#0x}", rva);
			
// 			let impl_flags = data[offset..].read_u16()?;
// 			offset += 2;
// 			println!("  impl flags: {:#0x}", impl_flags);

// 			let flags = data[offset..].read_u16()?;
// 			offset += 2;
// 			println!("  flags: {:#0x}", flags);

// 			let name_si = read_idx(data, offset, si_size)?;
// 			offset += si_size;
// 			println!("  name index: {:#0x}", name_si);

// 			let signature_bi = read_idx(data, offset, bi_size)?;
// 			offset += bi_size;
// 			println!("  signature index: {:#0x}", signature_bi);

// 			let pi_size = if table_lens[METADATA_PARAM] <= 0xFFFF { 2 } else { 4 };
// 			let first_param_idx = read_idx(data, offset, pi_size)?;
// 			offset += pi_size;
// 			println!("  first param index: {:#0x}", first_param_idx);
// 		}

// 		t += 1;
// 	}
	
// 	// METADATA_PARAM II.22.33
// 	if (valid_mask >> METADATA_PARAM) & 1 == 1 {
// 		let len  = row_lens[t * 4];
// 		println!("Param table with {} item(s).", len);

// 		for i in 0..len {
// 			println!("* Param #{}", i);

//             // II.23.1.13
// 			const IN:                u16 = 0x0001;
// 			const OUT:               u16 = 0x0002;
// 			const OPTIONAL:          u16 = 0x0010;
// 			const HAS_DEFAULT:       u16 = 0x1000;
// 			const HAS_FIELD_MARSHAL: u16 = 0x2000;
// 			const UNUSED:            u16 = 0xcfe0;
// 			let flags = data[offset..].read_u16()?;
// 			offset += 2;

// 			print!("  flags: {:#0x} -> ", flags);
// 			if flags & IN != 0 { print!("In "); }
// 			if flags & OUT != 0 { print!("Out "); }
// 			if flags & OPTIONAL != 0 { print!("Optional "); }
// 			if flags & HAS_DEFAULT != 0 { print!("HasDefault "); }
// 			if flags & HAS_FIELD_MARSHAL != 0 { print!("HasFieldMarshal "); }
// 			if flags & UNUSED != 0 { print!("Unused "); }
// 			println!("");

// 			let seq = data[offset..].read_u16()?;
// 			offset += 2;
// 			println!("  sequence: {:#0x}", seq);

// 			let name_si = read_idx(data, offset, si_size)?;
// 			offset += si_size;
// 			println!("  name index: {:#0x}", name_si);
// 		}

// 		t += 1;
// 	}

// 	// METADATA_INTERFACEIMPL II.22.23
// 	// The InterfaceImpl table records the interfaces a type
// 	// implements explicitly.  Conceptually, each row in the
// 	// InterfaceImpl table indicates that Class implements Interface.
// 	if (valid_mask >> METADATA_INTERFACEIMPL) & 1 == 1 {
// 		let len  = row_lens[t * 4];
// 		println!("InterfaceImpl table with {} item(s).", len);

// 		for i in 0..len {
// 			println!("* InterfaceImpl #{}", i);

// 			let ti_size   = if table_lens[METADATA_TYPEDEF] <= 0xFFFF { 2 } else { 4 };
// 			let class_idx = read_idx(data, offset, ti_size)?;
// 			offset += ti_size;
// 			println!("  class: {:#0x}", class_idx);

// 			// II.24.2.6:
// 			const TAG_MASK: usize = 0b11;
// 			const TYPEDEF:  usize = 0;
// 			const TYPEREF:  usize = 1;
// 			const TYPESPEC: usize = 2;

// 			let max_len = max!(
// 				table_lens[METADATA_TYPEDEF],
// 				table_lens[METADATA_TYPEREF],
// 				table_lens[METADATA_TYPESPEC]) as usize;
// 			let size  = if max_len < size_for_big_index(3) { 2 } else { 4 };
// 			let shift = log2(TAG_MASK);

// 			let interface = read_idx(data, offset, size)?;
// 			offset += size;

// 			print!("  interface ");
// 			match interface & TAG_MASK {
// 				TYPEDEF => print!("TypeDef"),
// 				TYPEREF => print!("TypeRef"),
// 				TYPESPEC => print!("TypeSpec"),
// 				_ => Err("Invalid TypeDefOrRef tag.")?,
// 			};
// 			println!(" {:#0x}", interface >> shift);
// 		}

// 		t += 1;
// 	}
	
// 	// METADATA_MEMBERREF II 22.25
// 	// An entry is made into the MemberRef table whenever a reference
// 	// is made in the CIL code to a method or field which is defined
// 	// in another module or assembly.
// 	// (Also, an entry is made for a call to a method with a VARARG
// 	// signature, even when it is defined in the same module as the
// 	// call site.)
// 	if (valid_mask >> METADATA_MEMBERREF) & 1 == 1 {
// 		let len  = row_lens[t * 4];
// 		println!("MemberRef table with {} item(s).", len);

// 		for i in 0..len {
// 			println!("* MemberRef #{}", i);

// 			// II.24.2.6
// 			const TAG_MASK:  usize = 0b111;
// 			const TYPEDEF:   usize = 0;
// 			const TYPEREF:   usize = 1;
// 			const MODULEREF: usize = 2;
// 			const METHODDEF: usize = 3;
// 			const TYPESPEC:  usize = 4;

// 			let max_len = max!(
// 				table_lens[METADATA_TYPEDEF],
// 				table_lens[METADATA_TYPEREF],
// 				table_lens[METADATA_MODULEREF],
// 				table_lens[METADATA_METHODDEF],
// 				table_lens[METADATA_TYPESPEC]) as usize;
// 			let size  = if max_len < size_for_big_index(5) { 2 } else { 4 };
// 			let shift = log2(TAG_MASK);

// 			let class = read_idx(data, offset, size)?;
// 			offset += size;

// 			print!("  class ");
// 			match class & TAG_MASK {
// 				TYPEDEF   => print!("TypeDef"),
// 				TYPEREF   => print!("TypeRef"),
// 				MODULEREF => print!("ModuleRef"),
// 				METHODDEF => print!("MethodDef"),
// 				TYPESPEC  => print!("TypeSpec"),
// 				_ => Err("Invalid MemberRefParent tag.")?,
// 			};
// 			println!(" {:#0x}", class >> shift);

// 			let name_si = read_idx(data, offset, si_size)?;
// 			offset += si_size;
// 			println!("  name index: {:#0x}", name_si);

// 			let signature_bi = read_idx(data, offset, bi_size)?;
// 			offset += bi_size;
// 			println!("  signature index: {:#0x}", signature_bi);
// 		}

// 		t += 1;
// 	}
	
// 	// METADATA_CONSTANT II.22.9
// 	if (valid_mask >> METADATA_CONSTANT) & 1 == 1 {
// 		let len  = row_lens[t * 4];
// 		println!("Constant table with {} item(s).", len);

// 		for i in 0..len {
// 			println!("* Constant #{}", i);

// 			// A 1-byte constant, followed by a 1-byte padding zero.
// 			let ty = data[offset..].read_u8()?;
// 			offset += 2;

// 			// TODO(dmi): @incomplete As far as I understand, it can be
// 			// followed by other bytes depending on type.

// 			// II.24.2.6
// 			const TAG_MASK: usize = 0b11;
// 			const FIELD:    usize = 0;
// 			const PARAM:    usize = 1;
// 			const PROPERTY: usize = 2;

// 			let max_len = max!(
// 				table_lens[METADATA_PARAM],
// 				table_lens[METADATA_FIELD],
// 				table_lens[METADATA_PROPERTY]) as usize;
// 			let size  = if max_len < size_for_big_index(3) { 2 } else { 4 };
// 			let shift = log2(TAG_MASK);

// 			let parent = read_idx(data, offset, size)?;
// 			offset += size;

// 			print!("  parent ");
// 			match parent & TAG_MASK {
// 				FIELD    => print!("Field"),
// 				PARAM    => print!("Param"),
// 				PROPERTY => print!("Property"),
// 				_ => Err("Invalid HasConstant tag.")?,
// 			};
// 			println!(" {:#0x}", parent >> shift);
			
// 			let value_bi = read_idx(data, offset, bi_size)?;
// 			offset += bi_size;
// 			println!("  value: {:#0x}", value_bi);
// 		}

// 		t += 1;
// 	}
	
// 	// METADATA_CUSTOMATTRIBUTE II.22.10
// 	if (valid_mask >> METADATA_CUSTOMATTRIBUTE) & 1 == 1 {
// 		let len  = row_lens[t * 4];
// 		println!("CustomAttribute table with {} item(s).", len);

// 		for i in 0..len {
// 			println!("* CustomAttribute #{}", i);

// 			// II.24.2.6
// 			{
// 				const TAG_MASK: usize = 0b11111;
// 				const METHOD_DEF:               usize = 0;
// 				const FIELD:                    usize = 1;
// 				const TYPE_REF:                 usize = 2;
// 				const TYPE_DEF:                 usize = 3;
// 				const PARAM:                    usize = 4;
// 				const INTERFACE_IMPL:           usize = 5;
// 				const MEMBER_REF:               usize = 6;
// 				const MODULE:                   usize = 7;
// 				const PERMISSION:               usize = 8;
// 				const PROPERTY:                 usize = 9;
// 				const EVENT:                    usize = 10;
// 				const STANDALONE_SIG:           usize = 11;
// 				const MODULE_REF:               usize = 12;
// 				const TYPE_SPEC:                usize = 13;
// 				const ASSEMBLY:                 usize = 14;
// 				const ASSEMBLY_REF:             usize = 15;
// 				const FILE:                     usize = 16;
// 				const EXPORTED_TYPE:            usize = 17;
// 				const MANIFEST_RESOURCE:        usize = 18;
// 				const GENERIC_PARAM:            usize = 19;
// 				const GENERIC_PARAM_CONSTRAINT: usize = 20;
// 				const METHOD_SPEC:              usize = 21;

// 				let max_len = max!(
// 					table_lens[METADATA_METHODDEF],
// 					table_lens[METADATA_FIELD],
// 					table_lens[METADATA_TYPEREF],
// 					table_lens[METADATA_TYPEDEF],
// 					table_lens[METADATA_PARAM],
// 					table_lens[METADATA_INTERFACEIMPL],
// 					table_lens[METADATA_MEMBERREF],
// 					table_lens[METADATA_MODULE],
// 					table_lens[METADATA_PROPERTY],
// 					table_lens[METADATA_EVENT],
// 					table_lens[METADATA_STANDALONESIG],
// 					table_lens[METADATA_MODULEREF],
// 					table_lens[METADATA_TYPESPEC],
// 					table_lens[METADATA_ASSEMBLY],
// 					table_lens[METADATA_ASSEMBLYREF],
// 					table_lens[METADATA_FILE],
// 					table_lens[METADATA_EXPORTEDTYPE],
// 					table_lens[METADATA_MANIFESTRESOURCE],
// 					table_lens[METADATA_GENERICPARAM],
// 					table_lens[METADATA_GENERICPARAMCONSTRAINT],
// 					table_lens[METADATA_METHODSPEC]) as usize;
// 				let size  = if max_len < size_for_big_index(21) { 2 } else { 4 };
// 				let shift = log2(TAG_MASK);

// 				let parent = read_idx(data, offset, size)?;
// 				offset += size;

// 				print!("  parent ");
// 				match parent & TAG_MASK {
// 					METHOD_DEF               => print!("MethodDef"),
// 					FIELD                    => print!("Field"),
// 					TYPE_REF                 => print!("TypeRef"),
// 					TYPE_DEF                 => print!("TypeDef"),
// 					PARAM                    => print!("Param"),
// 					INTERFACE_IMPL           => print!("InterfaceImpl"),
// 					MEMBER_REF               => print!("MemberRef"),
// 					MODULE                   => print!("Module"),
// 					PERMISSION               => print!("Permission"),
// 					PROPERTY                 => print!("Property"),
// 					EVENT                    => print!("Event"),
// 					STANDALONE_SIG           => print!("StandAloneSig"),
// 					MODULE_REF               => print!("ModuleRef"),
// 					TYPE_SPEC                => print!("TypeSpec"),
// 					ASSEMBLY                 => print!("Assembly"),
// 					ASSEMBLY_REF             => print!("AssemblyRef"),
// 					FILE                     => print!("File"),
// 					EXPORTED_TYPE            => print!("ExportedType"),
// 					MANIFEST_RESOURCE        => print!("ManifestResource"),
// 					GENERIC_PARAM            => print!("GenericParam"),
// 					GENERIC_PARAM_CONSTRAINT => print!("GenericParamConstraint"),
// 					METHOD_SPEC              => print!("MethodSpec"),
// 					_ => Err("Invalid HasCustomAttribute tag.")?,
// 				};
// 				println!(" {:#0x}", parent >> shift);
// 			}
// 			{
// 				// II.24.2.6
// 				const TAG_MASK:   usize = 0b111;
// 				const METHOD_DEF: usize = 2;
// 				const MEMBER_REF: usize = 3;

// 				let max_len = max!(
// 					table_lens[METADATA_METHODDEF],
// 					table_lens[METADATA_MEMBERREF]) as usize;
// 				let size  = if max_len < size_for_big_index(2) { 2 } else { 4 };
// 				let shift = log2(TAG_MASK);

// 				let ty = read_idx(data, offset, size)?;
// 				offset += size;

// 				print!("  type ");
// 				match ty & TAG_MASK {
// 					METHOD_DEF => print!("MethodDef"),
// 					MEMBER_REF => print!("MemberRef"),
// 					_ => Err("Invalid CustomAttributeType tag.")?,
// 				};
// 				println!(" {:#0x}", ty >> shift);
// 			}

// 			let value_bi = read_idx(data, offset, bi_size)?;
// 			offset += bi_size;
// 			println!("  value: {:#0x}", value_bi);
// 		}

// 		t += 1;
// 	}

// 	// METADATA_FIELDMARSHAL II.22.17
// 	// The FieldMarshal table has two columns.  It "links" an existing
// 	// row in the Field or Param table, to information in the Blob
// 	// heap that defines how that field or parameter (which, as usual,
// 	// covers the method return, as parameter number 0) shall be
// 	// marshalled when calling to or from unmanaged code via PInvoke
// 	// dispatch.
// 	if (valid_mask >> METADATA_FIELDMARSHAL) & 1 == 1 {
// 		let len  = row_lens[t * 4];
// 		println!("FieldMarshal table with {} item(s).", len);

// 		for i in 0..len {
// 			println!("* FieldMarshal #{}", i);

// 			// II.24.2.6
// 			const TAG_MASK: usize = 0b1;
// 			const FIELD: usize = 0;
// 			const PARAM: usize = 1;

// 			let max_len = max!(
// 				table_lens[METADATA_FIELD],
// 				table_lens[METADATA_PARAM]) as usize;
// 			let size  = if max_len < size_for_big_index(2) { 2 } else { 4 };
// 			let shift = log2(TAG_MASK);

// 			let parent = read_idx(data, offset, size)?;
// 			offset += size;

// 			print!("  parent ");
// 			match parent & TAG_MASK {
// 				FIELD => print!("Field"),
// 				PARAM => print!("Param"),
// 				_ => Err("Invalid MEMBERREF tag.")?,
// 			};
// 			println!(" {:#0x}", parent >> shift);

// 			let type_bi = read_idx(data, offset, bi_size)?;
// 			offset += bi_size;
// 			println!("  native type: {:#0x}", type_bi);
// 		}

// 		t += 1;
// 	}

// 	// METADATA_DECLSECURITY
// 	if (valid_mask >> METADATA_DECLSECURITY) & 1 == 1 {
// 		let len  = row_lens[t * 4];
// 		println!("DeclSecurity table with {} item(s).", len);

// 		for i in 0..len {
// 			println!("* DeclSecurity #{}", i);
// 		}

// 		t += 1;
// 	}

// 	// METADATA_CLASSLAYOUT
// 	if (valid_mask >> METADATA_CLASSLAYOUT) & 1 == 1 {
// 		let len  = row_lens[t * 4];
// 		println!("ClassLayout table with {} item(s).", len);

// 		for i in 0..len {
// 			println!("* ClassLayout #{}", i);
// 		}

// 		t += 1;
// 	}

// 	// METADATA_FIELDLAYOUT
// 	if (valid_mask >> METADATA_FIELDLAYOUT) & 1 == 1 {
// 		let len  = row_lens[t * 4];
// 		println!("FieldLayout table with {} item(s).", len);

// 		for i in 0..len {
// 			println!("* FieldLayout #{}", i);
// 		}

// 		t += 1;
// 	}

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
