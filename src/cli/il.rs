use crate::Result;
use crate::error::Error;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum OperandType {
	None,
	U8,
	U16,
	I32,
	I64,
	F32,
	F64,
	/// III.1.7.2 Contains u8 offset from the next instruction offset.
	ShortBranchTarget,
	/// III.1.7.2 Contains u32 offset from the next instruction offset.
	BranchTarget,
	/// III.1.9 Contains MetadataToken as u32.
	Token,
}

impl OperandType {
	fn measure(&self) -> usize {
		match self {
			OperandType::None => 0,
			OperandType::U8 => 1,
			OperandType::U16 => 2,
			OperandType::I32 => 4,
			OperandType::I64 => 8,
			OperandType::F32 => 4,
			OperandType::F64 => 8,
			OperandType::ShortBranchTarget => 1,
			OperandType::BranchTarget => 4,
			OperandType::Token => 4,
		}
	}
}

// III.1.2

macro_rules! for_opcodes1 {
	($x:ident) => {
		$x!{(NOP,            0x00, "nop",            OperandType::None)
			(BREAK,          0x01, "break",          OperandType::None)
			(LDARG_0,        0x02, "ldarg.0",        OperandType::None)
			(LDARG_1,        0x03, "ldarg.1",        OperandType::None)
			(LDARG_2,        0x04, "ldarg.2",        OperandType::None)
			(LDARG_3,        0x05, "ldarg.3",        OperandType::None)
			(LDLOC_0,        0x06, "ldloc.0",        OperandType::None)
			(LDLOC_1,        0x07, "ldloc.1",        OperandType::None)
			(LDLOC_2,        0x08, "ldloc.2",        OperandType::None)
			(LDLOC_3,        0x09, "ldloc.3",        OperandType::None)
			(STLOC_0,        0x0A, "stloc.0",        OperandType::None)
			(STLOC_1,        0x0B, "stloc.1",        OperandType::None)
			(STLOC_2,        0x0C, "stloc.2",        OperandType::None)
			(STLOC_3,        0x0D, "stloc.3",        OperandType::None)
			(LDARG_S,        0x0E, "ldarg.s",        OperandType::U8)
			(LDARGA_S,       0x0F, "ldarga.s",       OperandType::U8)
			(STARG_S,        0x10, "starg.s",        OperandType::U8)
			(LDLOC_S,        0x11, "ldloc.s",        OperandType::U8)
			(LDLOCA_S,       0x12, "ldloca.s",       OperandType::U8)
			(STLOC_S,        0x13, "stloc.s",        OperandType::U8)
			(LDNULL,         0x14, "ldnull",         OperandType::None)
			(LDC_I4_M1,      0x15, "ldc.i4.m1",      OperandType::None)
			(LDC_I4_0,       0x16, "ldc.i4.0",       OperandType::None)
			(LDC_I4_1,       0x17, "ldc.i4.1",       OperandType::None)
			(LDC_I4_2,       0x18, "ldc.i4.2",       OperandType::None)
			(LDC_I4_3,       0x19, "ldc.i4.3",       OperandType::None)
			(LDC_I4_4,       0x1A, "ldc.i4.4",       OperandType::None)
			(LDC_I4_5,       0x1B, "ldc.i4.5",       OperandType::None)
			(LDC_I4_6,       0x1C, "ldc.i4.6",       OperandType::None)
			(LDC_I4_7,       0x1D, "ldc.i4.7",       OperandType::None)
			(LDC_I4_8,       0x1E, "ldc.i4.8",       OperandType::None)
			(LDC_I4_S,       0x1F, "ldc.i4.s",       OperandType::I32)
			(LDC_I4,         0x20, "ldc.i4",         OperandType::I32)
			(LDC_I8,         0x21, "ldc.i8",         OperandType::I64)
			(LDC_R4,         0x22, "ldc.r4",         OperandType::F32)
			(LDC_R8,         0x23, "ldc.r8",         OperandType::F64)
			(DUP,            0x25, "dup",            OperandType::None)
			(POP,            0x26, "pop",            OperandType::None)
			(JMP,            0x27, "jmp",            OperandType::Token)
			(CALL,           0x28, "call",           OperandType::Token)
			(CALLI,          0x29, "calli",          OperandType::Token)
			(RET,            0x2A, "ret",            OperandType::None)
			(BR_S,           0x2B, "br.s",           OperandType::ShortBranchTarget)
			(BRFALSE_S,      0x2C, "brfalse.s",      OperandType::ShortBranchTarget)
			(BRTRUE_S,       0x2D, "brtrue.s",       OperandType::ShortBranchTarget)
			(BEQ_S,          0x2E, "beq.s",          OperandType::ShortBranchTarget)
			(BGE_S,          0x2F, "bge.s",          OperandType::ShortBranchTarget)
			(BGT_S,          0x30, "bgt.s",          OperandType::ShortBranchTarget)
			(BLE_S,          0x31, "ble.s",          OperandType::ShortBranchTarget)
			(BLT_S,          0x32, "blt.s",          OperandType::ShortBranchTarget)
			(BNE_UN_S,       0x33, "bne.un.s",       OperandType::ShortBranchTarget)
			(BGE_UN_S,       0x34, "bge.un.s",       OperandType::ShortBranchTarget)
			(BGT_UN_S,       0x35, "bgt.un.s",       OperandType::ShortBranchTarget)
			(BLE_UN_S,       0x36, "ble.un.s",       OperandType::ShortBranchTarget)
			(BLT_UN_S,       0x37, "blt.un.s",       OperandType::ShortBranchTarget)
			(BR,             0x38, "br",             OperandType::BranchTarget)
			(BRFALSE,        0x39, "brfalse",        OperandType::BranchTarget)
			(BRTRUE,         0x3A, "brtrue",         OperandType::BranchTarget)
			(BEQ,            0x3B, "beq",            OperandType::BranchTarget)
			(BGE,            0x3C, "bge",            OperandType::BranchTarget)
			(BGT,            0x3D, "bgt",            OperandType::BranchTarget)
			(BLE,            0x3E, "ble",            OperandType::BranchTarget)
			(BLT,            0x3F, "blt",            OperandType::BranchTarget)
			(BNE_UN,         0x40, "bne.un",         OperandType::BranchTarget)
			(BGE_UN,         0x41, "bge.un",         OperandType::BranchTarget)
			(BGT_UN,         0x42, "bgt.un",         OperandType::BranchTarget)
			(BLE_UN,         0x43, "ble.un",         OperandType::BranchTarget)
			(BLT_UN,         0x44, "blt.un",         OperandType::BranchTarget)
			(SWITCH,         0x45, "switch",         OperandType::None)
			(LDIND_I1,       0x46, "ldind.i1",       OperandType::None)
			(LDIND_U1,       0x47, "ldind.u1",       OperandType::None)
			(LDIND_I2,       0x48, "ldind.i2",       OperandType::None)
			(LDIND_U2,       0x49, "ldind.u2",       OperandType::None)
			(LDIND_I4,       0x4A, "ldind.i4",       OperandType::None)
			(LDIND_U4,       0x4B, "ldind.u4",       OperandType::None)
			(LDIND_I8,       0x4C, "ldind.i8",       OperandType::None)
			(LDIND_I,        0x4D, "ldind.i",        OperandType::None)
			(LDIND_R4,       0x4E, "ldind.r4",       OperandType::None)
			(LDIND_R8,       0x4F, "ldind.r8",       OperandType::None)
			(LDIND_REF,      0x50, "ldind.ref",      OperandType::None)
			(STIND_REF,      0x51, "stind.ref",      OperandType::None)
			(STIND_I1,       0x52, "stind.i1",       OperandType::None)
			(STIND_I2,       0x53, "stind.i2",       OperandType::None)
			(STIND_I4,       0x54, "stind.i4",       OperandType::None)
			(STIND_I8,       0x55, "stind.i8",       OperandType::None)
			(STIND_R4,       0x56, "stind.r4",       OperandType::None)
			(STIND_R8,       0x57, "stind.r8",       OperandType::None)
			(ADD,            0x58, "add",            OperandType::None)
			(SUB,            0x59, "sub",            OperandType::None)
			(MUL,            0x5A, "mul",            OperandType::None)
			(DIV,            0x5B, "div",            OperandType::None)
			(DIV_UN,         0x5C, "div.un",         OperandType::None)
			(REM,            0x5D, "rem",            OperandType::None)
			(REM_UN,         0x5E, "rem.un",         OperandType::None)
			(AND,            0x5F, "and",            OperandType::None)
			(OR,             0x60, "or",             OperandType::None)
			(XOR,            0x61, "xor",            OperandType::None)
			(SHL,            0x62, "shl",            OperandType::None)
			(SHR,            0x63, "shr",            OperandType::None)
			(SHR_UN,         0x64, "shr.un",         OperandType::None)
			(NEG,            0x65, "neg",            OperandType::None)
			(NOT,            0x66, "not",            OperandType::None)
			(CONV_I1,        0x67, "conv.i1",        OperandType::None)
			(CONV_I2,        0x68, "conv.i2",        OperandType::None)
			(CONV_I4,        0x69, "conv.i4",        OperandType::None)
			(CONV_I8,        0x6A, "conv.i8",        OperandType::None)
			(CONV_R4,        0x6B, "conv.r4",        OperandType::None)
			(CONV_R8,        0x6C, "conv.r8",        OperandType::None)
			(CONV_U4,        0x6D, "conv.u4",        OperandType::None)
			(CONV_U8,        0x6E, "conv.u8",        OperandType::None)
			(CALLVIRT,       0x6F, "callvirt",       OperandType::Token)
			(CPOBJ,          0x70, "cpobj",          OperandType::Token)
			(LDOBJ,          0x71, "ldobj",          OperandType::Token)
			(LDSTR,          0x72, "ldstr",          OperandType::Token)
			(NEWOBJ,         0x73, "newobj",         OperandType::Token)
			(CASTCLASS,      0x74, "castclass",      OperandType::Token)
			(ISINST,         0x75, "isinst",         OperandType::Token)
			(CONV_R_UN,      0x76, "conv.r.un",      OperandType::None)
			(UNBOX,          0x79, "unbox",          OperandType::Token)
			(THROW,          0x7A, "throw",          OperandType::None)
			(LDFLD,          0x7B, "ldfld",          OperandType::Token)
			(LDFLDA,         0x7C, "ldflda",         OperandType::Token)
			(STFLD,          0x7D, "stfld",          OperandType::Token)
			(LDSFLD,         0x7E, "ldsfld",         OperandType::Token)
			(LDSFLDA,        0x7F, "ldsflda",        OperandType::Token)
			(STSFLD,         0x80, "stsfld",         OperandType::Token)
			(STOBJ,          0x81, "stobj",          OperandType::Token)
			(CONV_OVF_I1_UN, 0x82, "conv.ovf.i1.un", OperandType::None)
			(CONV_OVF_I2_UN, 0x83, "conv.ovf.i2.un", OperandType::None)
			(CONV_OVF_I4_UN, 0x84, "conv.ovf.i4.un", OperandType::None)
			(CONV_OVF_I8_UN, 0x85, "conv.ovf.i8.un", OperandType::None)
			(CONV_OVF_U1_UN, 0x86, "conv.ovf.u1.un", OperandType::None)
			(CONV_OVF_U2_UN, 0x87, "conv.ovf.u2.un", OperandType::None)
			(CONV_OVF_U4_UN, 0x88, "conv.ovf.u4.un", OperandType::None)
			(CONV_OVF_U8_UN, 0x89, "conv.ovf.u8.un", OperandType::None)
			(CONV_OVF_I_UN,  0x8A, "conv.ovf.i.un",  OperandType::None)
			(CONV_OVF_U_UN,  0x8B, "conv.ovf.u.un",  OperandType::None)
			(BOX,            0x8C, "box",            OperandType::Token)
			(NEWARR,         0x8D, "newarr",         OperandType::Token)
			(LDLEN,          0x8E, "ldlen",          OperandType::None)
			(LDELEMA,        0x8F, "ldelema",        OperandType::Token)
			(LDELEM_I1,      0x90, "ldelem.i1",      OperandType::None)
			(LDELEM_U1,      0x91, "ldelem.u1",      OperandType::None)
			(LDELEM_I2,      0x92, "ldelem.i2",      OperandType::None)
			(LDELEM_U2,      0x93, "ldelem.u2",      OperandType::None)
			(LDELEM_I4,      0x94, "ldelem.i4",      OperandType::None)
			(LDELEM_U4,      0x95, "ldelem.u4",      OperandType::None)
			(LDELEM_I8,      0x96, "ldelem.i8",      OperandType::None)
			(LDELEM_I,       0x97, "ldelem.i",       OperandType::None)
			(LDELEM_R4,      0x98, "ldelem.r4",      OperandType::None)
			(LDELEM_R8,      0x99, "ldelem.r8",      OperandType::None)
			(LDELEM_REF,     0x9A, "ldelem.ref",     OperandType::None)
			(STELEM_I,       0x9B, "stelem.i",       OperandType::None)
			(STELEM_I1,      0x9C, "stelem.i1",      OperandType::None)
			(STELEM_I2,      0x9D, "stelem.i2",      OperandType::None)
			(STELEM_I4,      0x9E, "stelem.i4",      OperandType::None)
			(STELEM_I8,      0x9F, "stelem.i8",      OperandType::None)
			(STELEM_R4,      0xA0, "stelem.r4",      OperandType::None)
			(STELEM_R8,      0xA1, "stelem.r8",      OperandType::None)
			(STELEM_REF,     0xA2, "stelem.ref",     OperandType::None)
			(LDELEM,         0xA3, "ldelem",         OperandType::Token)
			(STELEM,         0xA4, "stelem",         OperandType::Token)
			(UNBOX_ANY,      0xA5, "unbox.any",      OperandType::Token)
			(CONV_OVF_I1,    0xB3, "conv.ovf.i1",    OperandType::None)
			(CONV_OVF_U1,    0xB4, "conv.ovf.u1",    OperandType::None)
			(CONV_OVF_I2,    0xB5, "conv.ovf.i2",    OperandType::None)
			(CONV_OVF_U2,    0xB6, "conv.ovf.u2",    OperandType::None)
			(CONV_OVF_I4,    0xB7, "conv.ovf.i4",    OperandType::None)
			(CONV_OVF_U4,    0xB8, "conv.ovf.u4",    OperandType::None)
			(CONV_OVF_I8,    0xB9, "conv.ovf.i8",    OperandType::None)
			(CONV_OVF_U8,    0xBA, "conv.ovf.u8",    OperandType::None)
			(REFANYVAL,      0xC2, "refanyval",      OperandType::Token)
			(CKFINITE,       0xC3, "ckfinite",       OperandType::None)
			(MKREFANY,       0xC6, "mkrefany",       OperandType::Token)
			(LDTOKEN,        0xD0, "ldtoken",        OperandType::Token)
			(CONV_U2,        0xD1, "conv.u2",        OperandType::None)
			(CONV_U1,        0xD2, "conv.u1",        OperandType::None)
			(CONV_I,         0xD3, "conv.i",         OperandType::None)
			(CONV_OVF_I,     0xD4, "conv.ovf.i",     OperandType::None)
			(CONV_OVF_U,     0xD5, "conv.ovf.u",     OperandType::None)
			(ADD_OVF,        0xD6, "add.ovf",        OperandType::None)
			(ADD_OVF_UN,     0xD7, "add.ovf.un",     OperandType::None)
			(MUL_OVF,        0xD8, "mul.ovf",        OperandType::None)
			(MUL_OVF_UN,     0xD9, "mul.ovf.un",     OperandType::None)
			(SUB_OVF,        0xDA, "sub.ovf",        OperandType::None)
			(SUB_OVF_UN,     0xDB, "sub.ovf.un",     OperandType::None)
			(ENDFINALLY,     0xDC, "endfinally",     OperandType::None)
			(LEAVE,          0xDD, "leave",          OperandType::I32)
			(LEAVE_S,        0xDE, "leave.s",        OperandType::U8)
			(STIND_I,        0xDF, "stind.i",        OperandType::None)
			(CONV_U,         0xE0, "conv.u",         OperandType::None)}
	};
}

// One-byte opcodes
macro_rules! opcode2 {
	($name:ident, $op:literal, $str:literal) => {
		const $name: u8 = $op;
	};
}

const TWO_BYTE_OPCODE_PREFIX: u8 = 0xFE;

// Two-byte opcodes.
opcode2!(ARGLIST,     0x00, "arglist");
opcode2!(CEQ,         0x01, "ceq");
opcode2!(CGT,         0x02, "cgt");
opcode2!(CGT_UN,      0x03, "cgt.un");
opcode2!(CLT,         0x04, "clt");
opcode2!(CLT_UN,      0x05, "clt.un");
opcode2!(LDFTN,       0x06, "ldftn");
opcode2!(LDVIRTFTN,   0x07, "ldvirtftn");
opcode2!(LDARG,       0x09, "ldarg");
opcode2!(LDARGA,      0x0A, "ldarga");
opcode2!(STARG,       0x0B, "starg");
opcode2!(LDLOC,       0x0C, "ldloc");
opcode2!(LDLOCA,      0x0D, "ldloca");
opcode2!(STLOC,       0x0E, "stloc");
opcode2!(LOCALLOC,    0x0F, "localloc");
opcode2!(ENDFILTER,   0x11, "endfilter");
opcode2!(UNALIGNED,   0x12, "unaligned.");
opcode2!(VOLATILE,    0x13, "volatile.");
opcode2!(TAIL,        0x14, "tail.");
opcode2!(INITOBJ,     0x15, "Initobj");
opcode2!(CONSTRAINED, 0x16, "constrained.");
opcode2!(CPBLK,       0x17, "cpblk");
opcode2!(INITBLK,     0x18, "initblk");
opcode2!(NO,          0x19, "no.");
opcode2!(RETHROW,     0x1A, "rethrow");
opcode2!(SIZEOF,      0x1C, "sizeof");
opcode2!(REFANYTYPE,  0x1D, "Refanytype");
opcode2!(READONLY,    0x1E, "readonly.");

// Generating stuff:

macro_rules! gen_constants {
	($(($name:ident, $op:literal, $str:literal, $operand:path))+) => {
		$( const $name: u8 = $op; )+
	};
}

for_opcodes1!(gen_constants);

pub fn dump_opcode(x: u8) -> &'static str {
	macro_rules! gen_match {
		($(($name:ident, $op:literal, $str:literal, $operand:path))+) => {
			match x {
				$(
					$op => $str,
				)+
				_ => "Unknown opcode."
			}
		};
	}

	for_opcodes1!(gen_match)
}

// TODO(dmi): @incomplete It should handle two-byte opcodes.
pub fn ins_size(op: u8) -> Result<usize> {
	Ok(1 + operand_type(op)?.measure())
}

fn operand_type(op: u8) -> Result<OperandType> {
	macro_rules! gen_match {
		($(($name:ident, $op:literal, $str:literal, $operand:path))+) => {
			match op {
				$(
					$op => Ok($operand),
				)+
				_ => Err(Error::General("Unknown opcode.")),
			}
		};
	}

	for_opcodes1!(gen_match)
}
