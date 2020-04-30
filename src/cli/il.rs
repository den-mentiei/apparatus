// III.1.2

macro_rules! for_opcodes1 {
	($x:ident) => {
		$x!((NOP,            0x00, "nop")
			(BREAK,          0x01, "break")
			(LDARG_0,        0x02, "ldarg.0")
			(LDARG_1,        0x03, "ldarg.1")
			(LDARG_2,        0x04, "ldarg.2")
			(LDARG_3,        0x05, "ldarg.3")
			(LDLOC_0,        0x06, "ldloc.0")
			(LDLOC_1,        0x07, "ldloc.1")
			(LDLOC_2,        0x08, "ldloc.2")
			(LDLOC_3,        0x09, "ldloc.3")
			(STLOC_0,        0x0A, "stloc.0")
			(STLOC_1,        0x0B, "stloc.1")
			(STLOC_2,        0x0C, "stloc.2")
			(STLOC_3,        0x0D, "stloc.3")
			(LDARG_S,        0x0E, "ldarg.s")
			(LDARGA_S,       0x0F, "ldarga.s")
			(STARG_S,        0x10, "starg.s")
			(LDLOC_S,        0x11, "ldloc.s")
			(LDLOCA_S,       0x12, "ldloca.s")
			(STLOC_S,        0x13, "stloc.s")
			(LDNULL,         0x14, "ldnull")
			(LDC_I4_M1,      0x15, "ldc.i4.m1")
			(LDC_I4_0,       0x16, "ldc.i4.0")
			(LDC_I4_1,       0x17, "ldc.i4.1")
			(LDC_I4_2,       0x18, "ldc.i4.2")
			(LDC_I4_3,       0x19, "ldc.i4.3")
			(LDC_I4_4,       0x1A, "ldc.i4.4")
			(LDC_I4_5,       0x1B, "ldc.i4.5")
			(LDC_I4_6,       0x1C, "ldc.i4.6")
			(LDC_I4_7,       0x1D, "ldc.i4.7")
			(LDC_I4_8,       0x1E, "ldc.i4.8")
			(LDC_I4_S,       0x1F, "ldc.i4.s")
			(LDC_I4,         0x20, "ldc.i4")
			(LDC_I8,         0x21, "ldc.i8")
			(LDC_R4,         0x22, "ldc.r4")
			(LDC_R8,         0x23, "ldc.r8")
			(DUP,            0x25, "dup")
			(POP,            0x26, "pop")
			(JMP,            0x27, "jmp")
			(CALL,           0x28, "call")
			(CALLI,          0x29, "calli")
			(RET,            0x2A, "ret")
			(BR_S,           0x2B, "br.s")
			(BRFALSE_S,      0x2C, "brfalse.s")
			(BRTRUE_S,       0x2D, "brtrue.s")
			(BEQ_S,          0x2E, "beq.s")
			(BGE_S,          0x2F, "bge.s")
			(BGT_S,          0x30, "bgt.s")
			(BLE_S,          0x31, "ble.s")
			(BLT_S,          0x32, "blt.s")
			(BNE_UN_S,       0x33, "bne.un.s")
			(BGE_UN_S,       0x34, "bge.un.s")
			(BGT_UN_S,       0x35, "bgt.un.s")
			(BLE_UN_S,       0x36, "ble.un.s")
			(BLT_UN_S,       0x37, "blt.un.s")
			(BR,             0x38, "br")
			(BRFALSE,        0x39, "brfalse")
			(BRTRUE,         0x3A, "brtrue")
			(BEQ,            0x3B, "beq")
			(BGE,            0x3C, "bge")
			(BGT,            0x3D, "bgt")
			(BLE,            0x3E, "ble")
			(BLT,            0x3F, "blt")
			(BNE_UN,         0x40, "bne.un")
			(BGE_UN,         0x41, "bge.un")
			(BGT_UN,         0x42, "bgt.un")
			(BLE_UN,         0x43, "ble.un")
			(BLT_UN,         0x44, "blt.un")
			(SWITCH,         0x45, "switch")
			(LDIND_I1,       0x46, "ldind.i1")
			(LDIND_U1,       0x47, "ldind.u1")
			(LDIND_I2,       0x48, "ldind.i2")
			(LDIND_U2,       0x49, "ldind.u2")
			(LDIND_I4,       0x4A, "ldind.i4")
			(LDIND_U4,       0x4B, "ldind.u4")
			(LDIND_I8,       0x4C, "ldind.i8")
			(LDIND_I,        0x4D, "ldind.i")
			(LDIND_R4,       0x4E, "ldind.r4")
			(LDIND_R8,       0x4F, "ldind.r8")
			(LDIND_REF,      0x50, "ldind.ref")
			(STIND_REF,      0x51, "stind.ref")
			(STIND_I1,       0x52, "stind.i1")
			(STIND_I2,       0x53, "stind.i2")
			(STIND_I4,       0x54, "stind.i4")
			(STIND_I8,       0x55, "stind.i8")
			(STIND_R4,       0x56, "stind.r4")
			(STIND_R8,       0x57, "stind.r8")
			(ADD,            0x58, "add")
			(SUB,            0x59, "sub")
			(MUL,            0x5A, "mul")
			(DIV,            0x5B, "div")
			(DIV_UN,         0x5C, "div.un")
			(REM,            0x5D, "rem")
			(REM_UN,         0x5E, "rem.un")
			(AND,            0x5F, "and")
			(OR,             0x60, "or")
			(XOR,            0x61, "xor")
			(SHL,            0x62, "shl")
			(SHR,            0x63, "shr")
			(SHR_UN,         0x64, "shr.un")
			(NEG,            0x65, "neg")
			(NOT,            0x66, "not")
			(CONV_I1,        0x67, "conv.i1")
			(CONV_I2,        0x68, "conv.i2")
			(CONV_I4,        0x69, "conv.i4")
			(CONV_I8,        0x6A, "conv.i8")
			(CONV_R4,        0x6B, "conv.r4")
			(CONV_R8,        0x6C, "conv.r8")
			(CONV_U4,        0x6D, "conv.u4")
			(CONV_U8,        0x6E, "conv.u8")
			(CALLVIRT,       0x6F, "callvirt")
			(CPOBJ,          0x70, "cpobj")
			(LDOBJ,          0x71, "ldobj")
			(LDSTR,          0x72, "ldstr")
			(NEWOBJ,         0x73, "newobj")
			(CASTCLASS,      0x74, "castclass")
			(ISINST,         0x75, "isinst")
			(CONV_R_UN,      0x76, "conv.r.un")
			(UNBOX,          0x79, "unbox")
			(THROW,          0x7A, "throw")
			(LDFLD,          0x7B, "ldfld")
			(LDFLDA,         0x7C, "ldflda")
			(STFLD,          0x7D, "stfld")
			(LDSFLD,         0x7E, "ldsfld")
			(LDSFLDA,        0x7F, "ldsflda")
			(STSFLD,         0x80, "stsfld")
			(STOBJ,          0x81, "stobj")
			(CONV_OVF_I1_UN, 0x82, "conv.ovf.i1.un")
			(CONV_OVF_I2_UN, 0x83, "conv.ovf.i2.un")
			(CONV_OVF_I4_UN, 0x84, "conv.ovf.i4.un")
			(CONV_OVF_I8_UN, 0x85, "conv.ovf.i8.un")
			(CONV_OVF_U1_UN, 0x86, "conv.ovf.u1.un")
			(CONV_OVF_U2_UN, 0x87, "conv.ovf.u2.un")
			(CONV_OVF_U4_UN, 0x88, "conv.ovf.u4.un")
			(CONV_OVF_U8_UN, 0x89, "conv.ovf.u8.un")
			(CONV_OVF_I_UN,  0x8A, "conv.ovf.i.un")
			(CONV_OVF_U_UN,  0x8B, "conv.ovf.u.un")
			(BOX,            0x8C, "box")
			(NEWARR,         0x8D, "newarr")
			(LDLEN,          0x8E, "ldlen")
			(LDELEMA,        0x8F, "ldelema")
			(LDELEM_I1,      0x90, "ldelem.i1")
			(LDELEM_U1,      0x91, "ldelem.u1")
			(LDELEM_I2,      0x92, "ldelem.i2")
			(LDELEM_U2,      0x93, "ldelem.u2")
			(LDELEM_I4,      0x94, "ldelem.i4")
			(LDELEM_U4,      0x95, "ldelem.u4")
			(LDELEM_I8,      0x96, "ldelem.i8")
			(LDELEM_I,       0x97, "ldelem.i")
			(LDELEM_R4,      0x98, "ldelem.r4")
			(LDELEM_R8,      0x99, "ldelem.r8")
			(LDELEM_REF,     0x9A, "ldelem.ref")
			(STELEM_I,       0x9B, "stelem.i")
			(STELEM_I1,      0x9C, "stelem.i1")
			(STELEM_I2,      0x9D, "stelem.i2")
			(STELEM_I4,      0x9E, "stelem.i4")
			(STELEM_I8,      0x9F, "stelem.i8")
			(STELEM_R4,      0xA0, "stelem.r4")
			(STELEM_R8,      0xA1, "stelem.r8")
			(STELEM_REF,     0xA2, "stelem.ref")
			(LDELEM,         0xA3, "ldelem")
			(STELEM,         0xA4, "stelem")
			(UNBOX_ANY,      0xA5, "unbox.any")
			(CONV_OVF_I1,    0xB3, "conv.ovf.i1")
			(CONV_OVF_U1,    0xB4, "conv.ovf.u1")
			(CONV_OVF_I2,    0xB5, "conv.ovf.i2")
			(CONV_OVF_U2,    0xB6, "conv.ovf.u2")
			(CONV_OVF_I4,    0xB7, "conv.ovf.i4")
			(CONV_OVF_U4,    0xB8, "conv.ovf.u4")
			(CONV_OVF_I8,    0xB9, "conv.ovf.i8")
			(CONV_OVF_U8,    0xBA, "conv.ovf.u8")
			(REFANYVAL,      0xC2, "refanyval")
			(CKFINITE,       0xC3, "ckfinite")
			(MKREFANY,       0xC6, "mkrefany")
			(LDTOKEN,        0xD0, "ldtoken")
			(CONV_U2,        0xD1, "conv.u2")
			(CONV_U1,        0xD2, "conv.u1")
			(CONV_I,         0xD3, "conv.i")
			(CONV_OVF_I,     0xD4, "conv.ovf.i")
			(CONV_OVF_U,     0xD5, "conv.ovf.u")
			(ADD_OVF,        0xD6, "add.ovf")
			(ADD_OVF_UN,     0xD7, "add.ovf.un")
			(MUL_OVF,        0xD8, "mul.ovf")
			(MUL_OVF_UN,     0xD9, "mul.ovf.un")
			(SUB_OVF,        0xDA, "sub.ovf")
			(SUB_OVF_UN,     0xDB, "sub.ovf.un")
			(ENDFINALLY,     0xDC, "endfinally")
			(LEAVE,          0xDD, "leave")
			(LEAVE_S,        0xDE, "leave.s")
			(STIND_I,        0xDF, "stind.i")
			(CONV_U,         0xE0, "conv.u"));
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
	($(($name:ident, $op:literal, $str:literal))+) => {
		$( const $name: u8 = $op; )+
	};
}

for_opcodes1!(gen_constants);

pub fn dump_opcode(x: u8) -> &'static str {
	macro_rules! gen_match {
		($(($name:ident, $op:literal, $str:literal))+) => {
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
