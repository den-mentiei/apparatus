// III.1.2

macro_rules! opcode {
	($name:ident, $op:literal, $str:literal) => {
		const $name: u8 = $op;
	};
}

// One-byte opcodes.
opcode!(NOP,            0x00, "nop");
opcode!(BREAK,          0x01, "break");
opcode!(LDARG_0,        0x02, "ldarg.0");
opcode!(LDARG_1,        0x03, "ldarg.1");
opcode!(LDARG_2,        0x04, "ldarg.2");
opcode!(LDARG_3,        0x05, "ldarg.3");
opcode!(LDLOC_0,        0x06, "ldloc.0");
opcode!(LDLOC_1,        0x07, "ldloc.1");
opcode!(LDLOC_2,        0x08, "ldloc.2");
opcode!(LDLOC_3,        0x09, "ldloc.3");
opcode!(STLOC_0,        0x0A, "stloc.0");
opcode!(STLOC_1,        0x0B, "stloc.1");
opcode!(STLOC_2,        0x0C, "stloc.2");
opcode!(STLOC_3,        0x0D, "stloc.3");
opcode!(LDARG_S,        0x0E, "ldarg.s");
opcode!(LDARGA_S,       0x0F, "ldarga.s");
opcode!(STARG_S,        0x10, "starg.s");
opcode!(LDLOC_S,        0x11, "ldloc.s");
opcode!(LDLOCA_S,       0x12, "ldloca.s");
opcode!(STLOC_S,        0x13, "stloc.s");
opcode!(LDNULL,         0x14, "ldnull");
opcode!(LDC_I4_M1,      0x15, "ldc.i4.m1");
opcode!(LDC_I4_0,       0x16, "ldc.i4.0");
opcode!(LDC_I4_1,       0x17, "ldc.i4.1");
opcode!(LDC_I4_2,       0x18, "ldc.i4.2");
opcode!(LDC_I4_3,       0x19, "ldc.i4.3");
opcode!(LDC_I4_4,       0x1A, "ldc.i4.4");
opcode!(LDC_I4_5,       0x1B, "ldc.i4.5");
opcode!(LDC_I4_6,       0x1C, "ldc.i4.6");
opcode!(LDC_I4_7,       0x1D, "ldc.i4.7");
opcode!(LDC_I4_8,       0x1E, "ldc.i4.8");
opcode!(LDC_I4_S,       0x1F, "ldc.i4.s");
opcode!(LDC_I4,         0x20, "ldc.i4");
opcode!(LDC_I8,         0x21, "ldc.i8");
opcode!(LDC_R4,         0x22, "ldc.r4");
opcode!(LDC_R8,         0x23, "ldc.r8");
opcode!(DUP,            0x25, "dup");
opcode!(POP,            0x26, "pop");
opcode!(JMP,            0x27, "jmp");
opcode!(CALL,           0x28, "call");
opcode!(CALLI,          0x29, "calli");
opcode!(RET,            0x2A, "ret");
opcode!(BR_S,           0x2B, "br.s");
opcode!(BRFALSE_S,      0x2C, "brfalse.s");
opcode!(BRTRUE_S,       0x2D, "brtrue.s");
opcode!(BEQ_S,          0x2E, "beq.s");
opcode!(BGE_S,          0x2F, "bge.s");
opcode!(BGT_S,          0x30, "bgt.s");
opcode!(BLE_S,          0x31, "ble.s");
opcode!(BLT_S,          0x32, "blt.s");
opcode!(BNE_UN_S,       0x33, "bne.un.s");
opcode!(BGE_UN_S,       0x34, "bge.un.s");
opcode!(BGT_UN_S,       0x35, "bgt.un.s");
opcode!(BLE_UN_S,       0x36, "ble.un.s");
opcode!(BLT_UN_S,       0x37, "blt.un.s");
opcode!(BR,             0x38, "br");
opcode!(BRFALSE,        0x39, "brfalse");
opcode!(BRTRUE,         0x3A, "brtrue");
opcode!(BEQ,            0x3B, "beq");
opcode!(BGE,            0x3C, "bge");
opcode!(BGT,            0x3D, "bgt");
opcode!(BLE,            0x3E, "ble");
opcode!(BLT,            0x3F, "blt");
opcode!(BNE_UN,         0x40, "bne.un");
opcode!(BGE_UN,         0x41, "bge.un");
opcode!(BGT_UN,         0x42, "bgt.un");
opcode!(BLE_UN,         0x43, "ble.un");
opcode!(BLT_UN,         0x44, "blt.un");
opcode!(SWITCH,         0x45, "switch");
opcode!(LDIND_I1,       0x46, "ldind.i1");
opcode!(LDIND_U1,       0x47, "ldind.u1");
opcode!(LDIND_I2,       0x48, "ldind.i2");
opcode!(LDIND_U2,       0x49, "ldind.u2");
opcode!(LDIND_I4,       0x4A, "ldind.i4");
opcode!(LDIND_U4,       0x4B, "ldind.u4");
opcode!(LDIND_I8,       0x4C, "ldind.i8");
opcode!(LDIND_I,        0x4D, "ldind.i");
opcode!(LDIND_R4,       0x4E, "ldind.r4");
opcode!(LDIND_R8,       0x4F, "ldind.r8");
opcode!(LDIND_REF,      0x50, "ldind.ref");
opcode!(STIND_REF,      0x51, "stind.ref");
opcode!(STIND_I1,       0x52, "stind.i1");
opcode!(STIND_I2,       0x53, "stind.i2");
opcode!(STIND_I4,       0x54, "stind.i4");
opcode!(STIND_I8,       0x55, "stind.i8");
opcode!(STIND_R4,       0x56, "stind.r4");
opcode!(STIND_R8,       0x57, "stind.r8");
opcode!(ADD,            0x58, "add");
opcode!(SUB,            0x59, "sub");
opcode!(MUL,            0x5A, "mul");
opcode!(DIV,            0x5B, "div");
opcode!(DIV_UN,         0x5C, "div.un");
opcode!(REM,            0x5D, "rem");
opcode!(REM_UN,         0x5E, "rem.un");
opcode!(AND,            0x5F, "and");
opcode!(OR,             0x60, "or");
opcode!(XOR,            0x61, "xor");
opcode!(SHL,            0x62, "shl");
opcode!(SHR,            0x63, "shr");
opcode!(SHR_UN,         0x64, "shr.un");
opcode!(NEG,            0x65, "neg");
opcode!(NOT,            0x66, "not");
opcode!(CONV_I1,        0x67, "conv.i1");
opcode!(CONV_I2,        0x68, "conv.i2");
opcode!(CONV_I4,        0x69, "conv.i4");
opcode!(CONV_I8,        0x6A, "conv.i8");
opcode!(CONV_R4,        0x6B, "conv.r4");
opcode!(CONV_R8,        0x6C, "conv.r8");
opcode!(CONV_U4,        0x6D, "conv.u4");
opcode!(CONV_U8,        0x6E, "conv.u8");
opcode!(CALLVIRT,       0x6F, "callvirt");
opcode!(CPOBJ,          0x70, "cpobj");
opcode!(LDOBJ,          0x71, "ldobj");
opcode!(LDSTR,          0x72, "ldstr");
opcode!(NEWOBJ,         0x73, "newobj");
opcode!(CASTCLASS,      0x74, "castclass");
opcode!(ISINST,         0x75, "isinst");
opcode!(CONV_R_UN,      0x76, "conv.r.un");
opcode!(UNBOX,          0x79, "unbox");
opcode!(THROW,          0x7A, "throw");
opcode!(LDFLD,          0x7B, "ldfld");
opcode!(LDFLDA,         0x7C, "ldflda");
opcode!(STFLD,          0x7D, "stfld");
opcode!(LDSFLD,         0x7E, "ldsfld");
opcode!(LDSFLDA,        0x7F, "ldsflda");
opcode!(STSFLD,         0x80, "stsfld");
opcode!(STOBJ,          0x81, "stobj");
opcode!(CONV_OVF_I1_UN, 0x82, "conv.ovf.i1.un");
opcode!(CONV_OVF_I2_UN, 0x83, "conv.ovf.i2.un");
opcode!(CONV_OVF_I4_UN, 0x84, "conv.ovf.i4.un");
opcode!(CONV_OVF_I8_UN, 0x85, "conv.ovf.i8.un");
opcode!(CONV_OVF_U1_UN, 0x86, "conv.ovf.u1.un");
opcode!(CONV_OVF_U2_UN, 0x87, "conv.ovf.u2.un");
opcode!(CONV_OVF_U4_UN, 0x88, "conv.ovf.u4.un");
opcode!(CONV_OVF_U8_UN, 0x89, "conv.ovf.u8.un");
opcode!(CONV_OVF_I_UN,  0x8A, "conv.ovf.i.un");
opcode!(CONV_OVF_U_UN,  0x8B, "conv.ovf.u.un");
opcode!(BOX,            0x8C, "box");
opcode!(NEWARR,         0x8D, "newarr");
opcode!(LDLEN,          0x8E, "ldlen");
opcode!(LDELEMA,        0x8F, "ldelema");
opcode!(LDELEM_I1,      0x90, "ldelem.i1");
opcode!(LDELEM_U1,      0x91, "ldelem.u1");
opcode!(LDELEM_I2,      0x92, "ldelem.i2");
opcode!(LDELEM_U2,      0x93, "ldelem.u2");
opcode!(LDELEM_I4,      0x94, "ldelem.i4");
opcode!(LDELEM_U4,      0x95, "ldelem.u4");
opcode!(LDELEM_I8,      0x96, "ldelem.i8");
opcode!(LDELEM_I,       0x97, "ldelem.i");
opcode!(LDELEM_R4,      0x98, "ldelem.r4");
opcode!(LDELEM_R8,      0x99, "ldelem.r8");
opcode!(LDELEM_REF,     0x9A, "ldelem.ref");
opcode!(STELEM_I,       0x9B, "stelem.i");
opcode!(STELEM_I1,      0x9C, "stelem.i1");
opcode!(STELEM_I2,      0x9D, "stelem.i2");
opcode!(STELEM_I4,      0x9E, "stelem.i4");
opcode!(STELEM_I8,      0x9F, "stelem.i8");
opcode!(STELEM_R4,      0xA0, "stelem.r4");
opcode!(STELEM_R8,      0xA1, "stelem.r8");
opcode!(STELEM_REF,     0xA2, "stelem.ref");
opcode!(LDELEM,         0xA3, "ldelem");
opcode!(STELEM,         0xA4, "stelem");
opcode!(UNBOX_ANY,      0xA5, "unbox.any");
opcode!(CONV_OVF_I1,    0xB3, "conv.ovf.i1");
opcode!(CONV_OVF_U1,    0xB4, "conv.ovf.u1");
opcode!(CONV_OVF_I2,    0xB5, "conv.ovf.i2");
opcode!(CONV_OVF_U2,    0xB6, "conv.ovf.u2");
opcode!(CONV_OVF_I4,    0xB7, "conv.ovf.i4");
opcode!(CONV_OVF_U4,    0xB8, "conv.ovf.u4");
opcode!(CONV_OVF_I8,    0xB9, "conv.ovf.i8");
opcode!(CONV_OVF_U8,    0xBA, "conv.ovf.u8");
opcode!(REFANYVAL,      0xC2, "refanyval");
opcode!(CKFINITE,       0xC3, "ckfinite");
opcode!(MKREFANY,       0xC6, "mkrefany");
opcode!(LDTOKEN,        0xD0, "ldtoken");
opcode!(CONV_U2,        0xD1, "conv.u2");
opcode!(CONV_U1,        0xD2, "conv.u1");
opcode!(CONV_I,         0xD3, "conv.i");
opcode!(CONV_OVF_I,     0xD4, "conv.ovf.i");
opcode!(CONV_OVF_U,     0xD5, "conv.ovf.u");
opcode!(ADD_OVF,        0xD6, "add.ovf");
opcode!(ADD_OVF_UN,     0xD7, "add.ovf.un");
opcode!(MUL_OVF,        0xD8, "mul.ovf");
opcode!(MUL_OVF_UN,     0xD9, "mul.ovf.un");
opcode!(SUB_OVF,        0xDA, "sub.ovf");
opcode!(SUB_OVF_UN,     0xDB, "sub.ovf.un");
opcode!(ENDFINALLY,     0xDC, "endfinally");
opcode!(LEAVE,          0xDD, "leave");
opcode!(LEAVE_S,        0xDE, "leave.s");
opcode!(STIND_I,        0xDF, "stind.i");
opcode!(CONV_U,         0xE0, "conv.u");

// Two-byte opcodes.
// 0xFE 0x00 arglist
// 0xFE 0x01 ceq
// 0xFE 0x02 cgt
// 0xFE 0x03 cgt.un
// 0xFE 0x04 clt
// 0xFE 0x05 clt.un
// 0xFE 0x06 ldftn
// 0xFE 0x07 ldvirtftn
// 0xFE 0x09 ldarg
// 0xFE 0x0A ldarga
// 0xFE 0x0B starg
// 0xFE 0x0C ldloc
// 0xFE 0x0D ldloca
// 0xFE 0x0E stloc
// 0xFE 0x0F localloc
// 0xFE 0x11 endfilter
// 0xFE 0x12 unaligned.
// 0xFE 0x13 volatile.
// 0xFE 0x14 tail.
// 0xFE 0x15 Initobj
// 0xFE 0x16 constrained.
// 0xFE 0x17 cpblk
// 0xFE 0x18 initblk
// 0xFE 0x19 no.
// 0xFE 0x1A rethrow
// 0xFE 0x1C sizeof
// 0xFE 0x1D Refanytype
// 0xFE 0x1E readonly.
