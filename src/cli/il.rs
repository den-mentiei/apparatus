/// III.1.2

// One-byte opcodes.
const NOP:            u8 = 0x00;
const BREAK:          u8 = 0x01;
const LDARG_0:        u8 = 0x02;
const LDARG_1:        u8 = 0x03;
const LDARG_2:        u8 = 0x04;
const LDARG_3:        u8 = 0x05;
const LDLOC_0:        u8 = 0x06;
const LDLOC_1:        u8 = 0x07;
const LDLOC_2:        u8 = 0x08;
const LDLOC_3:        u8 = 0x09;
const STLOC_0:        u8 = 0x0A;
const STLOC_1:        u8 = 0x0B;
const STLOC_2:        u8 = 0x0C;
const STLOC_3:        u8 = 0x0D;
const LDARG_S:        u8 = 0x0E;
const LDARGA_S:       u8 = 0x0F;
const STARG_S:        u8 = 0x10;
const LDLOC_S:        u8 = 0x11;
const LDLOCA_S:       u8 = 0x12;
const STLOC_S:        u8 = 0x13;
const LDNULL:         u8 = 0x14;
const LDC_I4_M1:      u8 = 0x15;
const LDC_I4_0:       u8 = 0x16;
const LDC_I4_1:       u8 = 0x17;
const LDC_I4_2:       u8 = 0x18;
const LDC_I4_3:       u8 = 0x19;
const LDC_I4_4:       u8 = 0x1A;
const LDC_I4_5:       u8 = 0x1B;
const LDC_I4_6:       u8 = 0x1C;
const LDC_I4_7:       u8 = 0x1D;
const LDC_I4_8:       u8 = 0x1E;
const LDC_I4_S:       u8 = 0x1F;
const LDC_I4:         u8 = 0x20;
const LDC_I8:         u8 = 0x21;
const LDC_R4:         u8 = 0x22;
const LDC_R8:         u8 = 0x23;
const DUP:            u8 = 0x25;
const POP:            u8 = 0x26;
const JMP:            u8 = 0x27;
const CALL:           u8 = 0x28;
const CALLI:          u8 = 0x29;
const RET:            u8 = 0x2A;
const BR_S:           u8 = 0x2B;
const BRFALSE_S:      u8 = 0x2C;
const BRTRUE_S:       u8 = 0x2D;
const BEQ_S:          u8 = 0x2E;
const BGE_S:          u8 = 0x2F;
const BGT_S:          u8 = 0x30;
const BLE_S:          u8 = 0x31;
const BLT_S:          u8 = 0x32;
const BNE_UN_S:       u8 = 0x33;
const BGE_UN_S:       u8 = 0x34;
const BGT_UN_S:       u8 = 0x35;
const BLE_UN_S:       u8 = 0x36;
const BLT_UN_S:       u8 = 0x37;
const BR:             u8 = 0x38;
const BRFALSE:        u8 = 0x39;
const BRTRUE:         u8 = 0x3A;
const BEQ:            u8 = 0x3B;
const BGE:            u8 = 0x3C;
const BGT:            u8 = 0x3D;
const BLE:            u8 = 0x3E;
const BLT:            u8 = 0x3F;
const BNE_UN:         u8 = 0x40;
const BGE_UN:         u8 = 0x41;
const BGT_UN:         u8 = 0x42;
const BLE_UN:         u8 = 0x43;
const BLT_UN:         u8 = 0x44;
const SWITCH:         u8 = 0x45;
const LDIND_I1:       u8 = 0x46;
const LDIND_U1:       u8 = 0x47;
const LDIND_I2:       u8 = 0x48;
const LDIND_U2:       u8 = 0x49;
const LDIND_I4:       u8 = 0x4A;
const LDIND_U4:       u8 = 0x4B;
const LDIND_I8:       u8 = 0x4C;
const LDIND_I:        u8 = 0x4D;
const LDIND_R4:       u8 = 0x4E;
const LDIND_R8:       u8 = 0x4F;
const LDIND_REF:      u8 = 0x50;
const STIND_REF:      u8 = 0x51;
const STIND_I1:       u8 = 0x52;
const STIND_I2:       u8 = 0x53;
const STIND_I4:       u8 = 0x54;
const STIND_I8:       u8 = 0x55;
const STIND_R4:       u8 = 0x56;
const STIND_R8:       u8 = 0x57;
const ADD:            u8 = 0x58;
const SUB:            u8 = 0x59;
const MUL:            u8 = 0x5A;
const DIV:            u8 = 0x5B;
const DIV_UN:         u8 = 0x5C;
const REM:            u8 = 0x5D;
const REM_UN:         u8 = 0x5E;
const AND:            u8 = 0x5F;
const OR:             u8 = 0x60;
const XOR:            u8 = 0x61;
const SHL:            u8 = 0x62;
const SHR:            u8 = 0x63;
const SHR_UN:         u8 = 0x64;
const NEG:            u8 = 0x65;
const NOT:            u8 = 0x66;
const CONV_I1:        u8 = 0x67;
const CONV_I2:        u8 = 0x68;
const CONV_I4:        u8 = 0x69;
const CONV_I8:        u8 = 0x6A;
const CONV_R4:        u8 = 0x6B;
const CONV_R8:        u8 = 0x6C;
const CONV_U4:        u8 = 0x6D;
const CONV_U8:        u8 = 0x6E;
const CALLVIRT:       u8 = 0x6F;
const CPOBJ:          u8 = 0x70;
const LDOBJ:          u8 = 0x71;
const LDSTR:          u8 = 0x72;
const NEWOBJ:         u8 = 0x73;
const CASTCLASS:      u8 = 0x74;
const ISINST:         u8 = 0x75;
const CONV_R_UN:      u8 = 0x76;
const UNBOX:          u8 = 0x79;
const THROW:          u8 = 0x7A;
const LDFLD:          u8 = 0x7B;
const LDFLDA:         u8 = 0x7C;
const STFLD:          u8 = 0x7D;
const LDSFLD:         u8 = 0x7E;
const LDSFLDA:        u8 = 0x7F;
const STSFLD:         u8 = 0x80;
const STOBJ:          u8 = 0x81;
const CONV_OVF_I1_UN: u8 = 0x82;
const CONV_OVF_I2_UN: u8 = 0x83;
const CONV_OVF_I4_UN: u8 = 0x84;
const CONV_OVF_I8_UN: u8 = 0x85;
const CONV_OVF_U1_UN: u8 = 0x86;
const CONV_OVF_U2_UN: u8 = 0x87;
const CONV_OVF_U4_UN: u8 = 0x88;
const CONV_OVF_U8_UN: u8 = 0x89;
const CONV_OVF_I_UN:  u8 = 0x8A;
const CONV_OVF_U_UN:  u8 = 0x8B;
const BOX:            u8 = 0x8C;
const NEWARR:         u8 = 0x8D;
const LDLEN:          u8 = 0x8E;
const LDELEMA:        u8 = 0x8F;
const LDELEM_I1:      u8 = 0x90;
const LDELEM_U1:      u8 = 0x91;
const LDELEM_I2:      u8 = 0x92;
const LDELEM_U2:      u8 = 0x93;
const LDELEM_I4:      u8 = 0x94;
const LDELEM_U4:      u8 = 0x95;
const LDELEM_I8:      u8 = 0x96;
const LDELEM_I:       u8 = 0x97;
const LDELEM_R4:      u8 = 0x98;
const LDELEM_R8:      u8 = 0x99;
const LDELEM_REF:     u8 = 0x9A;
const STELEM_I:       u8 = 0x9B;
const STELEM_I1:      u8 = 0x9C;
const STELEM_I2:      u8 = 0x9D;
const STELEM_I4:      u8 = 0x9E;
const STELEM_I8:      u8 = 0x9F;
const STELEM_R4:      u8 = 0xA0;
const STELEM_R8:      u8 = 0xA1;
const STELEM_REF:     u8 = 0xA2;
const LDELEM:         u8 = 0xA3;
const STELEM:         u8 = 0xA4;
const UNBOX_ANY:      u8 = 0xA5;
const CONV_OVF_I1:    u8 = 0xB3;
const CONV_OVF_U1:    u8 = 0xB4;
const CONV_OVF_I2:    u8 = 0xB5;
const CONV_OVF_U2:    u8 = 0xB6;
const CONV_OVF_I4:    u8 = 0xB7;
const CONV_OVF_U4:    u8 = 0xB8;
const CONV_OVF_I8:    u8 = 0xB9;
const CONV_OVF_U8:    u8 = 0xBA;
const REFANYVAL:      u8 = 0xC2;
const CKFINITE:       u8 = 0xC3;
const MKREFANY:       u8 = 0xC6;
const LDTOKEN:        u8 = 0xD0;
const CONV_U2:        u8 = 0xD1;
const CONV_U1:        u8 = 0xD2;
const CONV_I:         u8 = 0xD3;
const CONV_OVF_I:     u8 = 0xD4;
const CONV_OVF_U:     u8 = 0xD5;
const ADD_OVF:        u8 = 0xD6;
const ADD_OVF_UN:     u8 = 0xD7;
const MUL_OVF:        u8 = 0xD8;
const MUL_OVF_UN:     u8 = 0xD9;
const SUB_OVF:        u8 = 0xDA;
const SUB_OVF_UN:     u8 = 0xDB;
const ENDFINALLY:     u8 = 0xDC;
const LEAVE:          u8 = 0xDD;
const LEAVE_S:        u8 = 0xDE;
const STIND_I:        u8 = 0xDF;
const CONV_U:         u8 = 0xE0;

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
