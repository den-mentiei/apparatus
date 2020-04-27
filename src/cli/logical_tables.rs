use log::{debug};

use crate::Result;
use crate::error::Error;
use crate::buf::Reading;

// II.24.2.6: The physical representation of a row cell e at a
// column with type C is defined as follows: 
// - If e is a constant, it is stored using the number of bytes as
// specified for its column type C (i.e., a 2-bit mask of type
// PropertyAttributes).
// - If e is an index into the GUID heap, 'blob', or String heap,
// it is stored using the number of bytes as defined in the
// HeapSizes field.
//- If e is a simple index into a table with index i, it is stored
// using 2 bytes if table i has less than 2^16 rows, otherwise it
// is stored using 4 bytes.
// - If e is a coded index that points into table ti out of n
// possible tables t0, ...tn-1, then it is stored as
// e << (log n) | tag{ t0, ...tn-1}[ti] using 2 bytes if
// the maximum number of rows of tables t0, ...tn-1, is
// less than 2^(16 - (log n)), and using 4 bytes otherwise.
// The family of finite maps tag {t0, ...tn-1} is defined below.
// Note that decoding a physical row requires the inverse of this
// mapping. [For example, the Parent column of the Constant table
// indexes a row in the Field, Param, or Property tables.  The
// actual table is encoded into the low 2 bits of the number,
// using the values: 0 => Field, 1 => Param, 2 => Property.The
// remaining bits hold the actual row number being indexed.  For
// example, a value of 0x321, indexes row number 0xC8 in the Param
// table.]

// Taken from ECMA II.22
const METADATA_MODULE:                   usize = 0x00;
const METADATA_TYPE_REF:                 usize = 0x01;
const METADATA_TYPE_DEF:                 usize = 0x02;
const METADATA_FIELD:                    usize = 0x04;
const METADATA_METHOD_DEF:               usize = 0x06;
const METADATA_PARAM:                    usize = 0x08;
const METADATA_INTERFACE_IMPL:           usize = 0x09;
const METADATA_MEMBER_REF:               usize = 0x0A;
const METADATA_CONSTANT:                 usize = 0x0B;
const METADATA_CUSTOM_ATTRIBUTE:         usize = 0x0C;
const METADATA_FIELD_MARSHAL:            usize = 0x0D;
const METADATA_DECL_SECURITY:            usize = 0x0E;
const METADATA_CLASS_LAYOUT:             usize = 0x0F;
const METADATA_FIELD_LAYOUT:             usize = 0x10;
const METADATA_STANDALONE_SIG:           usize = 0x11;
const METADATA_EVENT_MAP:                usize = 0x12;
const METADATA_EVENT:                    usize = 0x14;
const METADATA_PROPERTY_MAP:             usize = 0x15;
const METADATA_PROPERTY:                 usize = 0x17;
const METADATA_METHOD_SEMANTICS:         usize = 0x18;
const METADATA_METHOD_IMPL:              usize = 0x19;
const METADATA_MODULE_REF:               usize = 0x1A;
const METADATA_TYPE_SPEC:                usize = 0x1B;
const METADATA_IMPL_MAP:                 usize = 0x1C;
const METADATA_FIELD_RVA:                usize = 0x1D;
const METADATA_ASSEMBLY:                 usize = 0x20;
const METADATA_ASSEMBLY_PROCESSOR:       usize = 0x21;
const METADATA_ASSEMBLY_OS:              usize = 0x22;
const METADATA_ASSEMBLY_REF:             usize = 0x23;
const METADATA_ASSEMBLY_REF_PROCESSOR:   usize = 0x24;
const METADATA_ASSEMBLY_REF_OS:          usize = 0x25;
const METADATA_FILE:                     usize = 0x26;
const METADATA_EXPORTED_TYPE:            usize = 0x27;
const METADATA_MANIFEST_RESOURCE:        usize = 0x28;
const METADATA_NESTED_CLASS:             usize = 0x29;
const METADATA_GENERIC_PARAM:            usize = 0x2A;
const METADATA_METHOD_SPEC:              usize = 0x2B;
const METADATA_GENERIC_PARAM_CONSTRAINT: usize = 0x2C;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum IndexSize {
	/// 2-byte index.
	U16,
	/// 4-byte index.
	U32,
}

#[derive(Copy, Clone)]
pub struct Tables {
	pub string_index_size: IndexSize,
	pub guid_index_size:   IndexSize,
	pub blob_index_size:   IndexSize,
	pub lens: [u32; 64],
	pub size: usize,
	valid_mask: u64,
}

impl Tables {
	pub fn parse(data: &[u8]) -> Result<Tables> {
		let mut offset = &mut 0usize;

		// Reserverd1, major version, minor version.
		*offset += 6;
		
		// The HeapSizes field is a bitvector that encodes the width of
		// indexes into the various heaps. If bit 0 is set, indexes into
		// the #String heap are 4 bytes wide; if bit 1 is set, indexes
		// into the #GUID heap are 4 bytes wide; if bit 2 is set, indexes
		// into the #Blob heap are 4 bytes wide. Conversely, if the
		// HeapSize bit for a particular heap is not set, indexes into
		// that heap are 2 bytes wide.
		let heap_sizes: u8 = data.read(offset)?;
		debug!("Heap sizes: {:#010b}", heap_sizes);

		let string_index_size = if heap_sizes & 0x01 == 0 { IndexSize::U16 } else { IndexSize::U32 };
		let guid_index_size   = if heap_sizes & 0x02 == 0 { IndexSize::U16 } else { IndexSize::U32 };
		let blob_index_size   = if heap_sizes & 0x04 == 0 { IndexSize::U16 } else { IndexSize::U32 };

		// Reserved2
		*offset += 1;
		
		// The Valid field is a 64-bit bitvector that has a specific bit
		// set for each table that is stored in the stream; the mapping of
		// tables to indexes is given at the start of II.22.
		let valid_mask: u64 = data.read(offset)?;
		let n = valid_mask.count_ones() as usize;
		debug!("Valid mask: {:#066b} -> {} table(s).", valid_mask, n);

		// Sorted.
		*offset += 8;
		
		let mut lens = [0u32; 64];
		for i in 0..lens.len() {
			if (valid_mask >> i) & 1 == 1 {
				lens[i] = data.read(offset)?;
				debug!("Table #{} has {:#0x} item(s).", i, lens[i]);
			}
		}

		let size = *offset;
		
		Ok(Tables {
			string_index_size,
			guid_index_size,
			blob_index_size,
			lens,
			size,
			valid_mask,
		})
	}

	fn has_table(&self, id: usize) -> bool {
		(self.valid_mask >> id) & 1 == 1
	}
}

#[derive(Debug, PartialEq, Clone, Default)]
pub struct TableRows {
	pub modules: Box<[Module]>,
	pub type_refs: Box<[TypeRef]>,
	/// The first row represents the pseudo class that acts
	/// as a parent for functions and variables define at
	/// module scope.
	pub type_defs: Box<[TypeDef]>,
	/// Conceptually, each row is owned by one, and only one,
	/// row in the type_defs table. However, the owner of any
	/// row is not stored anywhere in the Field itself. There
	/// is merely a "forward-pointer" from each row in the
	/// type_defs table.
	pub fields: Box<[Field]>,
	/// Conceptually, every row is owned by one, and only one,
	/// row in type_defs.
	pub method_defs: Box<[MethodDef]>,
	/// Conceptually, every row is owned by one, and only one,
	/// row in method_defs.
	pub params: Box<[Param]>,
	/// Records the interfaces a type implements explicitly.
	pub interface_impls: Box<[InterfaceImpl]>,
	/// An entry is made into the MemberRef table whenever a reference is
	/// made in the CIL code to a method or field which is defined in another
	/// module or assembly. (Also, an entry is made for a call to a method
	/// with a VARARG signature, even when it is defined in the same module as
	/// the call site.)
	pub member_refs: Box<[MemberRef]>,
	/// Note that Constant information does not directly influence runtime
	/// behavior, although it is visible via Reflection (and hence can be used
	/// to implement functionality such as that provided by
	/// System.Enum.ToString). Compilers inspect this information, at compile
	/// time, when importing metadata, but the value of the constant itself,
	/// if used, becomes embedded into the CIL stream the compiler emits.
	pub constants: Box<[Constant]>,
	/// The column called ty is slightly misleading - it actually indexes a
	/// constructor method - the owner of that constructor method is the Type
	/// of the Custom Attribute.
	pub custom_attributes: Box<[CustomAttribute]>,
	/// The FieldMarshal table has two columns. It "links" an existing
	/// row in the Field or Param table, to information in the Blob heap that
	/// defines how that field or parameter (which, as usual, covers the
	/// method return, as parameter number 0) shall be marshalled when calling
	/// to or from unmanaged code via PInvoke dispatch.
	pub field_marshals: Box<[FieldMarshal]>,
	/// Security attributes, which derive from
	/// System.Security.Permissions.SecurityAttribute (see Partition IV), can
	/// be attached to a TypeDef, a Method, or an Assembly.
	pub security_attributes: Box<[DeclSecutity]>,
	/// The ClassLayout table is used to define how the fields of a class
	/// or value type shall be laid out by the CLI. (Normally, the CLI is free
	/// to reorder and/or insert gaps between the fields defined for a class
	/// or value type.)
	pub class_layouts: Box<[ClassLayout]>,
	pub field_layouts: Box<[FieldLayout]>,
	/// Signatures are stored in the metadata Blob heap. In most cases, they
	/// are indexed by a column in some table - Field.Signature,
	/// Method.Signature, MemberRef.Signature, etc. However, there are two
	/// cases that require a metadata token for a signature that is not
	/// indexed by any metadata table. The StandAloneSig table fulfils this
	/// need.
	/// The signature shall describe either:
	/// - a method - code generators create a row in the StandAloneSig
	/// table for each occurrence of a calli CIL instruction. That row indexes
	/// the call-site signature for the function pointer operand of the calli
	/// instruction
	/// - local variables - code generators create one row in the
	/// standalone_signatures for each method, to describe all of its local
	/// variables.
	pub standalone_signatures: Box<[StandAloneSig]>,
	/// EventMap info does not directly influence runtime behavior;
	/// what counts is the information stored for each method that the
	/// event comprises.
	pub event_maps: Box<[EventMap]>,
	/// Events are treated within metadata much like Properties; that
	/// is, as a way to associate a collection of methods defined on a given
	/// class.
	pub events: Box<[Event]>,
	pub property_maps: Box<[PropertyMap]>,
    /// Properties within metadata are best viewed as a means to
	/// gather together collections of methods defined on a class, give them a
	/// name, and not much else.
	pub properties: Box<[Property]>,
	pub method_semantics: Box<[MethodSemantics]>,
	/// MethodImpl tables let a compiler override the default inheritance
	/// rules provided by the CLI. Their original use was to allow a class C,
	/// that inherited method M from both interfaces I and J, to provide
	/// implementations for both methods (rather than have only one slot for M
	/// in its vtable).
	pub method_impls: Box<[MethodImpl]>,
	pub module_refs: Box<[ModuleRef]>,
	/// TypeSpec tokens can be used with any of the CIL instructions
	/// that take a TypeDef or TypeRef token; specifically, castclass, cpobj,
	/// initobj, isinst, ldelema, ldobj, mkrefany, newarr, refanyval, sizeof,
	/// stobj, box, and unbox.
	pub type_specs: Box<[TypeSpec]>,
	/// The ImplMap table holds information about unmanaged methods that can
	/// be reached from managed code, using PInvoke dispatch. Each row of the
	/// ImplMap table associates a row in the MethodDef table (MemberForwarded)
	/// with the name of a routine (ImportName) in some unmanaged DLL (ImportScope).
	pub impl_maps: Box<[ImplMap]>,
	/// Conceptually, each row in the FieldRVA table is an extension to
	/// exactly one row in the Field table, and records the RVA (Relative
	/// Virtual Address) within the image file at which this field's initial
	/// value is stored.  A row in the FieldRVA table is created for each
	/// static parent field that has specified the optional data label
	/// II.16). The RVA column is the relative virtual address of the data in
	/// the PE file (II.16.3).
	pub field_rvas: Box<[FieldRVA]>,
	pub assemblies: Box<[Assembly]>,
	pub assembly_refs: Box<[AssemblyRef]>,
	pub files: Box<[File]>,
}

impl TableRows {
	pub fn parse(header: &Tables, data: &[u8]) -> Result<Self> {
		let mut offset = &mut 0;

		macro_rules! table {
			($table:ident, $id:ident, $type:ty) => {
				let $table = if header.has_table($id) {
					let n = header.lens[$id] as usize;
					let mut result = Vec::with_capacity(n);

					for i in 0..n {
						result.push(<$type>::parse(header, data, offset)?);
					}

					result.into_boxed_slice()
				} else {
					empty()
				};
			};
		}

		table!(modules,               METADATA_MODULE,           Module);
		table!(type_refs,             METADATA_TYPE_REF,         TypeRef);
		table!(type_defs,             METADATA_TYPE_DEF,         TypeDef);
		table!(fields,                METADATA_FIELD,            Field);
		table!(method_defs,           METADATA_METHOD_DEF,       MethodDef);
		table!(params,                METADATA_PARAM,            Param);
		table!(interface_impls,       METADATA_INTERFACE_IMPL,   InterfaceImpl);
		table!(member_refs,           METADATA_MEMBER_REF,       MemberRef);
		table!(constants,             METADATA_CONSTANT,         Constant);
		table!(custom_attributes,     METADATA_CUSTOM_ATTRIBUTE, CustomAttribute);
		table!(field_marshals,        METADATA_FIELD_MARSHAL,    FieldMarshal);
		table!(security_attributes,   METADATA_FIELD_MARSHAL,    DeclSecutity);
		table!(class_layouts,         METADATA_CLASS_LAYOUT,     ClassLayout);
		table!(field_layouts,         METADATA_FIELD_LAYOUT,     FieldLayout);
		table!(standalone_signatures, METADATA_STANDALONE_SIG,   StandAloneSig);
		table!(event_maps,            METADATA_EVENT_MAP,        EventMap);
		table!(events,                METADATA_EVENT,            Event);
		table!(property_maps,         METADATA_PROPERTY_MAP,     PropertyMap);
		table!(properties,            METADATA_PROPERTY,         Property);
		table!(method_semantics,      METADATA_METHOD_SEMANTICS, MethodSemantics);
		table!(method_impls,          METADATA_METHOD_IMPL,      MethodImpl);
		table!(module_refs,           METADATA_MODULE_REF,       ModuleRef);
		table!(type_specs,            METADATA_TYPE_SPEC,        TypeSpec);
		table!(impl_maps,             METADATA_IMPL_MAP,         ImplMap);
		table!(field_rvas,            METADATA_FIELD_RVA,        FieldRVA);
		table!(assemblies,            METADATA_ASSEMBLY,         Assembly);
		table!(assembly_refs,         METADATA_ASSEMBLY_REF,     AssemblyRef);
		table!(files,                 METADATA_FILE,             File);

		// II.22.4
		if header.has_table(METADATA_ASSEMBLY_PROCESSOR) {
			Err("AssemblyProcessor should not be emitted into any PE file.")?;
		}
		// II.22.3
		if header.has_table(METADATA_ASSEMBLY_OS) {
			Err("AssemblyOS should not be emitted into any PE file.")?;
		}
		// II.22.7
		if header.has_table(METADATA_ASSEMBLY_REF_PROCESSOR) {
			Err("AssemblyRefProcessor should not be emitted into any PE file.")?;
		}
		// II.22.6
		if header.has_table(METADATA_ASSEMBLY_REF_OS) {
			Err("AssemblyRefOS should not be emitted into any PE file.")?;
		}
		
		Ok(TableRows {
			modules,
			type_refs,
			type_defs,
			fields,
			method_defs,
			params,
			interface_impls,
			member_refs,
			constants,
			custom_attributes,
			field_marshals,
			security_attributes,
			class_layouts,
			field_layouts,
			standalone_signatures,
			event_maps,
			events,
			property_maps,
			properties,
			method_semantics,
			method_impls,
			module_refs,
			type_specs,
			impl_maps,
			field_rvas,
			assemblies,
			assembly_refs,
			files,
		})
	}
}

#[derive(Debug, PartialEq, Clone, Default)]
pub struct StringIndex(u32);

impl StringIndex {
	fn parse(header: &Tables, data: &[u8], offset: &mut usize) -> Result<Self> {
		let i = match header.string_index_size {
			IndexSize::U16 => StringIndex(data.read::<u16>(offset)? as u32),
			IndexSize::U32 => StringIndex(data.read::<u32>(offset)? as u32),
		};
		Ok(i)
	}
}

#[derive(Debug, PartialEq, Clone, Default)]
pub struct GuidIndex(u32);

impl GuidIndex {
	fn parse(header: &Tables, data: &[u8], offset: &mut usize) -> Result<Self> {
		let i = match header.guid_index_size {
			IndexSize::U16 => GuidIndex(data.read::<u16>(offset)? as u32),
			IndexSize::U32 => GuidIndex(data.read::<u32>(offset)? as u32),
		};
		Ok(i)
	}
}

#[derive(Debug, PartialEq, Clone, Default)]
pub struct BlobIndex(u32);

// TODO(dmi): @incomplete Not sure if blob index can have different size.
impl BlobIndex {
	fn parse(header: &Tables, data: &[u8], offset: &mut usize) -> Result<Self> {
		Ok(BlobIndex(data.read::<u16>(offset)? as u32))
	}
}

macro_rules! simple_index {
	($name:ident, $id:ident) => {
		#[derive(Debug, PartialEq, Clone, Default)]
		pub struct $name(u32);

		impl $name {
			fn parse(header: &Tables, data: &[u8], offset: &mut usize) -> Result<Self> {
				let i = if header.lens[$id] <= 0xFFFF {
					$name(data.read::<u16>(offset)? as u32)
				} else {
					$name(data.read::<u32>(offset)? as u32)
				};
				Ok(i)
			}
		}
	};
}

simple_index!(FieldIndex,     METADATA_FIELD);
simple_index!(MethodDefIndex, METADATA_METHOD_DEF);
simple_index!(ParamIndex,     METADATA_PARAM);
simple_index!(TypeDefIndex,   METADATA_TYPE_DEF);
simple_index!(EventIndex,     METADATA_EVENT);
simple_index!(PropertyIndex,  METADATA_PROPERTY);
simple_index!(ModuleRefIndex, METADATA_MODULE_REF);

macro_rules! max {
	($x:expr) => ($x);
	($x:expr, $($xs:expr),+) => {
		{
			use std::cmp::max;
			max($x, max!($($xs),+))
		}
	};
}

macro_rules! count_tts {
    () => {0usize};
    ($_head:tt $($tail:tt)*) => {1usize + count_tts!($($tail)*)};
}

const fn size_for_big_index(n: usize) -> usize { 1 << (16 - log2(n)) }
const fn log2(x: usize) -> usize { 64usize - x.leading_zeros() as usize }

macro_rules! coded_index {
	($name:ident, $bits:expr, $(($v:ident $t:expr, $id:ident))+) => {
		#[derive(Debug, PartialEq, Copy, Clone)]
		pub enum $name {
			$($v(u32),)+
		}

		impl $name {
			fn parse(header: &Tables, data: &[u8], offset: &mut usize) -> Result<$name> {
				const COUNT: usize = count_tts!($($v),+);
				let max_len = max!($(header.lens[$id]),+) as usize;
				let value = if max_len < size_for_big_index(COUNT) {
					data.read::<u16>(offset)? as u32
				} else {
					data.read::<u32>(offset)?
				};

				let tag = value & (1 << $bits) - 1;
				let idx = value >> $bits;
				
				let r = match tag {
					$(
						$t => $name::$v(idx),
					)+
					_ => Err("Unknown coded index tag.")?,
				};

				Ok(r)
			}
		}
	};
}

// II 24.2.6

coded_index!(TypeDefOrRef, 2,
	(TypeDef  0, METADATA_TYPE_DEF)
	(TypeRef  1, METADATA_TYPE_REF)
	(TypeSpec 2, METADATA_TYPE_SPEC));

coded_index!(HasConstant, 2,
	(Field    0, METADATA_FIELD)
	(Param    1, METADATA_PARAM)
	(Property 2, METADATA_PROPERTY));

coded_index!(HasCustomAttribute, 5,
	(MethodDef               0, METADATA_METHOD_DEF)
	(Field                   1, METADATA_FIELD)
	(TypeRef                 2, METADATA_TYPE_REF)
	(TypeDef                 3, METADATA_TYPE_DEF)
	(Param                   4, METADATA_PARAM)
	(InterfaceImpl           5, METADATA_INTERFACE_IMPL)
	(MemberRef               6, METADATA_MEMBER_REF)
	(Module                  7, METADATA_MODULE)
	(Permission              8, METADATA_DECL_SECURITY)
	(Property                9, METADATA_PROPERTY)
	(Event                  10, METADATA_EVENT)
	(StandAloneSig          11, METADATA_STANDALONE_SIG)
	(ModuleRef              12, METADATA_MODULE_REF)
	(TypeSpec               13, METADATA_TYPE_SPEC)
	(Assembly               14, METADATA_ASSEMBLY)
	(AssemblyRef            15, METADATA_ASSEMBLY_REF)
	(File                   16, METADATA_FILE)
	(ExportedType           17, METADATA_EXPORTED_TYPE)
	(ManifestResource       18, METADATA_MANIFEST_RESOURCE)
	(GenericParam           19, METADATA_GENERIC_PARAM)
	(GenericParamConstraint 20, METADATA_GENERIC_PARAM_CONSTRAINT)
	(MethodSpec             21, METADATA_METHOD_SPEC));

coded_index!(HasFieldMarshall, 1,
	(Field 0, METADATA_FIELD)
	(Param 1, METADATA_PARAM));

coded_index!(HasDeclSecurity, 2,
	(TypeDef   0, METADATA_TYPE_DEF)
	(MethodDef 1, METADATA_METHOD_DEF)
	(Assembly  2, METADATA_ASSEMBLY));

coded_index!(MemberRefParent, 3,
	(TypeDef   0, METADATA_TYPE_DEF)
	(TypeRef   1, METADATA_TYPE_REF)
	(ModuleRef 2, METADATA_MODULE_REF)
	(MethodDef 3, METADATA_METHOD_DEF)
	(TypeSpec  4, METADATA_TYPE_SPEC));

coded_index!(HasSemantics, 1,
	(Event    0, METADATA_EVENT)
	(Property 1, METADATA_PROPERTY));

coded_index!(MethodDefOrRef, 1,
	(MethodDef 0, METADATA_METHOD_DEF)
	(MemberRef 1, METADATA_MEMBER_REF));

coded_index!(MemberForwarded, 1,
	(Field     0, METADATA_FIELD)
	(MethodDef 1, METADATA_METHOD_DEF));

coded_index!(Implementation, 2,
	(File         0, METADATA_FILE)
	(AssemblyRef  1, METADATA_ASSEMBLY_REF)
	(ExportedType 2, METADATA_EXPORTED_TYPE));

coded_index!(CustomAttributeType, 3,
	(MethodDef 2, METADATA_METHOD_DEF)
	(MemberRef 3, METADATA_MEMBER_REF));

coded_index!(ResolutionScope, 2,
	(Module      0, METADATA_MODULE)
	(ModuleRef   1, METADATA_MODULE_REF)
	(AssemblyRef 2, METADATA_ASSEMBLY_REF)
	(TypeRef     3, METADATA_TYPE_REF));

/// II.22.30
#[derive(Debug, PartialEq, Clone)]
pub struct Module {
	/// Module name.
	pub name: StringIndex,
	/// Simply a Guid used to distinguish between two
	/// versions of the same module.
	pub mvid: GuidIndex,
}

impl Module {
	fn parse(header: &Tables, data: &[u8], offset: &mut usize) -> Result<Module> {
		let generation: u16 = data.read(offset)?;
		if generation != 0 {
			Err("Module has invalid generation.")?;
		}

		let name = StringIndex::parse(header, data, offset)?;
		let mvid = GuidIndex::parse(header, data, offset)?;

		let enc_id: u16 = data.read(offset)?;
		if enc_id != 0 {
			Err("Module.EncId is not zero.")?;
		}
		let enc_base_id: u16 = data.read(offset)?;
		if enc_base_id != 0 {
			Err("Module.EncBaseId is not zero.")?;
		}

		Ok(Module { name, mvid })
	}
}

/// II.24.2.6
#[derive(Debug, PartialEq, Clone)]
pub struct TypeRef {
	pub scope: ResolutionScope,
	pub name: StringIndex,
	pub namespace: StringIndex,
}

impl TypeRef {
	fn parse(header: &Tables, data: &[u8], offset: &mut usize) -> Result<TypeRef> {
		let scope = ResolutionScope::parse(header, data, offset)?;
		let name = StringIndex::parse(header, data, offset)?;
		let namespace = StringIndex::parse(header, data, offset)?;
		Ok(TypeRef { scope, name, namespace })
	}
}

/// II.22.37
#[derive(Debug, PartialEq, Clone)]
pub struct TypeDef {
	// TODO(dmi): @incomplete See TypeAttributes II.23.1.15
	flags: u32,
	pub name: StringIndex,
	pub namespace: StringIndex,
	pub extends: TypeDefOrRef,
	// It marks the first of a contiguous run of
    // Fields owned by this Type. The run continues to the smaller of:
	// - the last row of the Field table
	// - the next run of Fields, found by inspecting the field_list of
	// the next row in TypeDef table.
	field_list: FieldIndex,
	// It marks the first of a continguous run of
	// Methods owned bu this Type. The run continues to the smaller of:
	// - the last row of the MethodDef table
	// - the next run of Methods, found by inspecting the method_list of
	// the next row in TypeDef table.
	method_list: MethodDefIndex,
}

impl TypeDef {
	fn parse(header: &Tables, data: &[u8], offset: &mut usize) -> Result<TypeDef> {
		let flags: u32 = data.read(offset)?;
		let name = StringIndex::parse(header, data, offset)?;
		let namespace = StringIndex::parse(header, data, offset)?;
		let extends = TypeDefOrRef::parse(header, data, offset)?;
		let field_list = FieldIndex::parse(header, data, offset)?;
		let method_list = MethodDefIndex::parse(header, data, offset)?;
		
		Ok(TypeDef {
			flags,
			name,
			namespace,
			extends,
			field_list,
			method_list,
		})
	}
}

/// II.22.15
#[derive(Debug, PartialEq, Clone)]
pub struct Field {
	// TODO(dmi): @incomplete See FieldAttributes II.23.1.5
	flags: u16,
	pub name: StringIndex,
	pub sig: BlobIndex,
}

impl Field {
	fn parse(header: &Tables, data: &[u8], offset: &mut usize) -> Result<Self> {
		let flags: u16 = data.read(offset)?;
		let name = StringIndex::parse(header, data, offset)?;
		let sig = BlobIndex::parse(header, data, offset)?;
		Ok(Field { flags, name, sig })
	}
}

/// II.22.26
#[derive(Debug, PartialEq, Clone)]
pub struct MethodDef {
	pub rva: u32,
	// TODO(dmi): @incomplete See MethodImplAttributes II.23.1.10
	impl_flags: u16,
	// TODO(dmi): @incomplete See MethodAttributes II.23.1.10
	flags: u16,
	pub name: StringIndex,
	pub sig: BlobIndex,
	// TODO(dmi): @incomplete It marks the first of a contiguous run of
	// parameters owned by this method. The run continues
	// to the smaller of:
	// - the last row of the params table
	// - the next run of params, found by inspecting the
	// param_list of the next row.
	pub param_list: ParamIndex,
}

impl MethodDef {
	fn parse(header: &Tables, data: &[u8], offset: &mut usize) -> Result<Self> {
		// TODO(dmi): @next Finally! Can find entry point method now
		// and rush to get its IL-code.
		let rva: u32 = data.read(offset)?;
		let impl_flags: u16 = data.read(offset)?;
		let flags: u16 = data.read(offset)?;
		let name = StringIndex::parse(header, data, offset)?;
		let sig = BlobIndex::parse(header, data, offset)?;
		let param_list = ParamIndex::parse(header, data, offset)?;

		Ok(MethodDef {
			rva,
			impl_flags,
			flags,
			name,
			sig,
			param_list,
		})
	}
}

/// II.22.33
#[derive(Debug, PartialEq, Clone)]
pub struct Param {
	// TODO(dmi): @incomplete See ParamAttributes II.23.1.13
	flags: u16,
	pub seq: u16,
	pub name: StringIndex,
}

impl Param {
	fn parse(header: &Tables, data: &[u8], offset: &mut usize) -> Result<Self> {
		let flags: u16 = data.read(offset)?;
		let seq: u16 = data.read(offset)?;
		let name = StringIndex::parse(header, data, offset)?;
		Ok(Param { flags, seq, name })
	}
}

/// II.22.23
#[derive(Debug, PartialEq, Clone)]
pub struct InterfaceImpl {
	pub class: TypeDefIndex,
	pub iface: TypeDefOrRef,
}

impl InterfaceImpl {
	fn parse(header: &Tables, data: &[u8], offset: &mut usize) -> Result<Self> {
		let class = TypeDefIndex::parse(header, data, offset)?;
		let iface = TypeDefOrRef::parse(header, data, offset)?;
		Ok(InterfaceImpl { class, iface })
	}
}

/// II.22.25
#[derive(Debug, PartialEq, Clone)]
pub struct MemberRef {
	pub class: MemberRefParent,
	pub name: StringIndex,
	pub sig: BlobIndex,
}

impl MemberRef {
	fn parse(header: &Tables, data: &[u8], offset: &mut usize) -> Result<Self> {
		let class = MemberRefParent::parse(header, data, offset)?;
		let name = StringIndex::parse(header, data, offset)?;
		let sig = BlobIndex::parse(header, data, offset)?;
		Ok(MemberRef { class, name, sig })
	}
}

/// II.22.9
#[derive(Debug, PartialEq, Clone)]
pub struct Constant {
	// TODO(dmi) @incomplete See II.23.1.6
	ty: u8,
	pub parent: HasConstant,
	pub value: BlobIndex,
}

impl Constant {
	fn parse(header: &Tables, data: &[u8], offset: &mut usize) -> Result<Self> {
		let ty: u8 = data.read(offset)?;
		// Padding.
		*offset += 1;
		let parent = HasConstant::parse(header, data, offset)?;
		let value = BlobIndex::parse(header, data, offset)?;
		Ok(Constant { ty, parent, value })
	}
}

/// II.22.10
#[derive(Debug, PartialEq, Clone)]
pub struct CustomAttribute {
	pub parent: HasCustomAttribute,
	pub ty: CustomAttributeType,
	pub value: BlobIndex,
}

impl CustomAttribute {
	fn parse(header: &Tables, data: &[u8], offset: &mut usize) -> Result<Self> {
		let parent = HasCustomAttribute::parse(header, data, offset)?;
		let ty = CustomAttributeType::parse(header, data, offset)?;
		let value = BlobIndex::parse(header, data, offset)?;
		Ok(CustomAttribute { parent, ty, value })
	}
}

/// II.22.17
#[derive(Debug, PartialEq, Clone)]
pub struct FieldMarshal {
	pub parent: HasFieldMarshall,
	pub native_ty: BlobIndex,
}

impl FieldMarshal {
	fn parse(header: &Tables, data: &[u8], offset: &mut usize) -> Result<Self> {
		let parent = HasFieldMarshall::parse(header, data, offset)?;
		let native_ty = BlobIndex::parse(header, data, offset)?;
		Ok(FieldMarshal { parent, native_ty })
	}
}

/// II.22.11
#[derive(Debug, PartialEq, Clone)]
pub struct DeclSecutity {
	// TODO(dmi): @incomplete See table in II.22.11
	action: u16,
	pub parent: HasDeclSecurity,
	pub permission_set: BlobIndex,
}

impl DeclSecutity {
	fn parse(header: &Tables, data: &[u8], offset: &mut usize) -> Result<Self> {
		let action: u16 = data.read(offset)?;
		let parent = HasDeclSecurity::parse(header, data, offset)?;
		let permission_set = BlobIndex::parse(header, data, offset)?;
		Ok(DeclSecutity { action, parent, permission_set })
	}
}

/// II.22.8
#[derive(Debug, PartialEq, Clone)]
pub struct ClassLayout {
	pub packing_size: u16,
	pub class_size: u32,
	pub parent: TypeDefIndex,
}

impl ClassLayout {
	fn parse(header: &Tables, data: &[u8], offset: &mut usize) -> Result<Self> {
		let packing_size: u16 = data.read(offset)?;
		let class_size: u32 = data.read(offset)?;
		let parent = TypeDefIndex::parse(header, data, offset)?;
		Ok(ClassLayout { packing_size, class_size, parent })
	}
}

/// II.22.16
#[derive(Debug, PartialEq, Clone)]
pub struct FieldLayout {
	pub offset: u32,
	pub field: FieldIndex,
}

impl FieldLayout {
	fn parse(header: &Tables, data: &[u8], offset: &mut usize) -> Result<Self> {
		let f_offset: u32 = data.read(offset)?;
		let field = FieldIndex::parse(header, data, offset)?;
		Ok(FieldLayout { offset: f_offset, field })
	}
}

/// II.22.36
#[derive(Debug, PartialEq, Clone)]
pub struct StandAloneSig {
	pub sig: BlobIndex,
}

impl StandAloneSig {
	fn parse(header: &Tables, data: &[u8], offset: &mut usize) -> Result<Self> {
		let sig = BlobIndex::parse(header, data, offset)?;
		Ok(StandAloneSig { sig })
	}
}

/// II.22.12
#[derive(Debug, PartialEq, Clone)]
pub struct EventMap {
	pub parent: TypeDefIndex,
	/// It marks the first of a contiguous run of Events owned by this
	/// type. That run continues to the smaller of:
	/// - the last row othe events
	/// - the next run of Events, found by inspecting the event_list of
	/// the next row in event_maps
	pub event_list: EventIndex,
}

impl EventMap {
	fn parse(header: &Tables, data: &[u8], offset: &mut usize) -> Result<Self> {
		let parent = TypeDefIndex::parse(header, data, offset)?;
		let event_list = EventIndex::parse(header, data, offset)?;
		Ok(EventMap { parent, event_list })
	}
}

/// II.22.13
#[derive(Debug, PartialEq, Clone)]
pub struct Event {
	// TODO(dmi): @incomplete See EventAttributes II.23.1.4
	flags: u16,
	pub name: StringIndex,
	pub ty: TypeDefOrRef,
}

impl Event {
	fn parse(header: &Tables, data: &[u8], offset: &mut usize) -> Result<Self> {
		let flags: u16 = data.read(offset)?;
		let name = StringIndex::parse(header, data, offset)?;
		let ty = TypeDefOrRef::parse(header, data, offset)?;
		Ok(Event { flags, name, ty })
	}
}

/// II.22.35
#[derive(Debug, PartialEq, Clone)]
pub struct PropertyMap {
	pub parent: TypeDefIndex,
	/// It marks the first of a contiguous run of Properties owned by
	/// Parent. The run continues to the smaller of:
	/// - the last row of the Property table
	/// - the next run of Properties, found by inspecting the
	/// property_list of the next row in property_maps
	pub property_list: PropertyIndex,
}

impl PropertyMap {
	fn parse(header: &Tables, data: &[u8], offset: &mut usize) -> Result<Self> {
		let parent = TypeDefIndex::parse(header, data, offset)?;
		let property_list = PropertyIndex::parse(header, data, offset)?;
		Ok(PropertyMap { parent, property_list })
	}
}

/// II.22.34
#[derive(Debug, PartialEq, Clone)]
pub struct Property {
	// TODO(dmi): @incomplete See PropertyAttributes II.23.1.14
	flags: u16,
	pub name: StringIndex,
	/// The name of this column is misleading. It does not index a TypeDef or
	/// TypeRef table - instead it indexes the signature in the Blob heap of
	/// the Property
	pub ty: BlobIndex,
}

impl Property {
	fn parse(header: &Tables, data: &[u8], offset: &mut usize) -> Result<Self> {
		let flags: u16 = data.read(offset)?;
		let name = StringIndex::parse(header, data, offset)?;
		let ty = BlobIndex::parse(header, data, offset)?;
		Ok(Property { flags, name, ty })
	}
}

/// II.22.28
#[derive(Debug, PartialEq, Clone)]
pub struct MethodSemantics {
	// TODO(dmi): @incomplete See MethodSemanticsAttributes II.23.1.12
	semantics: u16,
	pub method: MethodDefIndex,
	pub assoc: HasSemantics,
}

impl MethodSemantics {
	fn parse(header: &Tables, data: &[u8], offset: &mut usize) -> Result<Self> {
		let semantics: u16 = data.read(offset)?;
		let method = MethodDefIndex::parse(header, data, offset)?;
		let assoc = HasSemantics::parse(header, data, offset)?;
		Ok(MethodSemantics { semantics, method, assoc })
	}
}

/// II.22.27
#[derive(Debug, PartialEq, Clone)]
pub struct MethodImpl {
	pub class: TypeDefIndex,
	pub body: MethodDefOrRef,
	pub decl: MethodDefOrRef,
}

impl MethodImpl {
	fn parse(header: &Tables, data: &[u8], offset: &mut usize) -> Result<Self> {
		let class = TypeDefIndex::parse(header, data, offset)?;
		let body = MethodDefOrRef::parse(header, data, offset)?;
		let decl = MethodDefOrRef::parse(header, data, offset)?;
		Ok(MethodImpl { class, body, decl })
	}
}

/// II.22.31
#[derive(Debug, PartialEq, Clone)]
pub struct ModuleRef {
	pub name: StringIndex,
}

impl ModuleRef {
	fn parse(header: &Tables, data: &[u8], offset: &mut usize) -> Result<Self> {
		let name = StringIndex::parse(header, data, offset)?;
		Ok(ModuleRef { name })
	}
}

/// II.22.39
/// The TypeSpec table has just one column, which indexes the
/// specification of a Type, stored in the Blob heap. This provides a
/// metadata token for that Type (rather than simply an index into the
/// Blob heap).  This is required, typically, for array operations, such
/// as creating, or calling methods on the array class.
#[derive(Debug, PartialEq, Clone)]
pub struct TypeSpec {
	pub sig: BlobIndex,
}

impl TypeSpec {
	fn parse(header: &Tables, data: &[u8], offset: &mut usize) -> Result<Self> {
		let sig = BlobIndex::parse(header, data, offset)?;
		Ok(TypeSpec { sig })
	}
}

/// II.22.22
#[derive(Debug, PartialEq, Clone)]
pub struct ImplMap {
	// TODO(dmi): @incomplete See PInvoke.Attributes II.23.18
	flags: u16,
	pub member_fwd: MemberForwarded,
	pub name: StringIndex,
	pub scope: ModuleRefIndex,
}

impl ImplMap {
	fn parse(header: &Tables, data: &[u8], offset: &mut usize) -> Result<Self> {
		let flags: u16 = data.read(offset)?;
		let member_fwd = MemberForwarded::parse(header, data, offset)?;
		let name = StringIndex::parse(header, data, offset)?;
		let scope = ModuleRefIndex::parse(header, data, offset)?;
		Ok(ImplMap { flags, member_fwd, name, scope })
	}
}

/// II.22.18
#[derive(Debug, PartialEq, Clone)]
pub struct FieldRVA {
	pub rva: u32,
	pub field: FieldIndex,
}

impl FieldRVA {
	fn parse(header: &Tables, data: &[u8], offset: &mut usize) -> Result<Self> {
		let rva: u32 = data.read(offset)?;
		let field = FieldIndex::parse(header, data, offset)?;
		Ok(FieldRVA { rva, field })
	}
}

/// II.22.2

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum HashAlgo {
	MD5,
	SHA1,
}

impl HashAlgo {
	fn parse(data: &[u8], offset: &mut usize) -> Result<Self> {
		let algo: u32 = data.read(offset)?;
		match algo {
			0x8003 => Ok(HashAlgo::MD5),
			0x8004 => Ok(HashAlgo::SHA1),
			_ => Err("Unknown hash algorithm.")?,
		}
	}
}

#[derive(Debug, PartialEq, Clone)]
pub struct Assembly {
	pub hash_algo: HashAlgo,
	pub major_version: u16,
	pub minor_version: u16,
	pub build_number: u16,
	pub revision_number: u16,
	// TODO(dmi): @incomplete See AssemblyFlags II.23.1.2
	flags: u32,
	pub pub_key: BlobIndex,
	pub name: StringIndex,
	pub culture: StringIndex,
}

impl Assembly {
	fn parse(header: &Tables, data: &[u8], offset: &mut usize) -> Result<Self> {
		let hash_algo = HashAlgo::parse(data, offset)?;
		let major_version: u16 = data.read(offset)?;
		let minor_version: u16 = data.read(offset)?;
		let build_number: u16 = data.read(offset)?;
		let revision_number: u16 = data.read(offset)?;
		let flags: u32 = data.read(offset)?;
		let pub_key = BlobIndex::parse(header, data, offset)?;
		let name = StringIndex::parse(header, data, offset)?;
		let culture = StringIndex::parse(header, data, offset)?;
		
		Ok(Assembly {
			hash_algo,
			major_version,
			minor_version,
			build_number,
			revision_number,
			flags,
			pub_key,
			name,
			culture,
		})
	}
}

/// II.22.5
#[derive(Debug, PartialEq, Clone)]
pub struct AssemblyRef {
	pub major_version: u16,
	pub minor_version: u16,
	pub build_number: u16,
	pub revision_number: u16,
	// TODO(dmi): @incomplete See AssemblyFlags II.23.1.2
	flags: u32,
	/// Indicating the public key or token that identifies the author
	/// of this Assembly.
	pub pub_key_or_token: BlobIndex,
	pub name: StringIndex,
	pub culture: StringIndex,
	pub hash: BlobIndex,
}

impl AssemblyRef {
	fn parse(header: &Tables, data: &[u8], offset: &mut usize) -> Result<Self> {
		let major_version: u16 = data.read(offset)?;
		let minor_version: u16 = data.read(offset)?;
		let build_number: u16 = data.read(offset)?;
		let revision_number: u16 = data.read(offset)?;
		let flags: u32 = data.read(offset)?;
		let pub_key_or_token = BlobIndex::parse(header, data, offset)?;
		let name = StringIndex::parse(header, data, offset)?;
		let culture = StringIndex::parse(header, data, offset)?;
		let hash = BlobIndex::parse(header, data, offset)?;

		Ok(AssemblyRef {
			major_version,
			minor_version,
			build_number,
			revision_number,
			flags,
			pub_key_or_token,
			name,
			culture,
			hash,
		})
	}
}

/// II.22.19
#[derive(Debug, PartialEq, Clone)]
pub struct File {
	// TODO(dmi): @incomplete See FileAttributes II.23.1.6
	flags: u32,
	pub name: StringIndex,
	pub hash: BlobIndex,
}

impl File {
	fn parse(header: &Tables, data: &[u8], offset: &mut usize) -> Result<Self> {
		let flags: u32 = data.read(offset)?;
		let name = StringIndex::parse(header, data, offset)?;
		let hash = BlobIndex::parse(header, data, offset)?;
		Ok(File { flags, name, hash  })
	}
}

// =======================================================================================

/// II.24.2.6
#[derive(Debug, PartialEq, Clone)]
pub struct D {
}

impl D {
	fn parse(header: &Tables, data: &[u8], offset: &mut usize) -> Result<Self> {
		Ok(D {  })
	}
}

fn empty<T>() -> Box<[T]> {
	Box::new([])
}
