//
// Created by cyrex on 2022-05-23.
//

#ifndef OUTPOST_C_PROJ__STRUCTS_2_H_
#define OUTPOST_C_PROJ__STRUCTS_2_H_

#include "types.h"
#include "func_ptrs.h"

typedef struct _astruct_7 {
    int field_0x40;
    int field_0x3e;
    u16 field_0x36;
    u8* field_0xa;
    u16 field_0x16;
    u16 field_0x2;
    u16 field_0x4;
    u16 field_0xc;
    int field_0x14;
    u16 field_0x1e;
    u16 field_0x1a;
    u16 field_0x18;
} astruct_7;

typedef struct _astruct_99 {
    u16 field0_0x0;
    u16 field1_0x2;
    u32 field2_0x4;
    u32 field3_0x8;
    u16 field4_0xc;
    u16 field5_0xe;
} astruct_99;


typedef struct _astruct_611 {
    u8 field0_0x0;
    u8 field1_0x1;
    u16 field2_0x2;
    u8 field3_0x4;
    u8 field4_0x5;
    u8 field5_0x6;
    u8 field6_0x7;
    u8 field7_0x8;
    u8 field8_0x9;
    u8 field9_0xa;
    u8 field10_0xb;
    u8 field11_0xc;
    u8 field12_0xd;
    u8 field13_0xe;
    u8 field14_0xf;
    u8 field15_0x10;
    u8 field16_0x11;
    u8 field17_0x12;
    u8 field18_0x13;
    u8 field19_0x14;
    u8 field20_0x15;
    u8 field21_0x16;
    u8 field22_0x17;
    u8 field23_0x18;
    u8 field24_0x19;
    u8 field25_0x1a;
    u8 field26_0x1b;
    u8 field27_0x1c;
    u8 field28_0x1d;
    u32 * field29_0x1e;
    u16 field30_0x20;
    u8 field31_0x22;
    u8 field32_0x23;
    u16 field33_0x24;
    u16 field34_0x26;
    u16 field35_0x28;
    u8 field36_0x2a;
    u8 field37_0x2b;
    u8 field38_0x2c;
    u8 field39_0x2d;
    u8 field40_0x2e;
    u8 field41_0x2f;
    u8 field42_0x30;
    u8 field43_0x31;
    u8 field44_0x32;
    u8 field45_0x33;
    u8 field46_0x34;
    u8 field47_0x35;
    u32 * field48_0x36;
    u16 field49_0x38;
    u32 * field50_0x3a;
    u16 field51_0x3c;
    u32 * field52_0x3e;
    u16 field53_0x40;
} astruct_611;


typedef struct _StructD {
    u16 address_offset_field_0x0;
    u16 address_offset_field_0x2;
    HFILE16 hfile_0x4;
    u8 field3_0x6;
    u8 field4_0x7;
    u16 field5_0x8;
    u16 field6_0xa;
    struct astruct_589 * field7_0xc;
    u16 field8_0xe;
    u8 field9_0x10;
    u8 field10_0x11;
    HDC16 field11_0x12;
    i32 field12_0x14;
    HWND16 field13_0x18;
    HPALETTE16 field14_0x1a;
    u8 field15_0x1c;
    u8 field16_0x1d;
    u8 field17_0x1e;
    u8 field18_0x1f;
    u16 field19_0x20;
    u16 field20_0x22;
    u8 field21_0x24;
    u8 field22_0x25;
    u8 field23_0x26;
    u8 field24_0x27;
    u8 field25_0x28;
    u8 field26_0x29;
    u8 field27_0x2a;
    u8 field28_0x2b;
    char * field29_0x2c;
    u8 field30_0x30;
    u8 field31_0x31;
    u8 field32_0x32;
    u8 field33_0x33;
    u8 field34_0x34;
    u8 field35_0x35;
    u8 field36_0x36;
    u8 field37_0x37;
    u8 field38_0x38;
    u8 field39_0x39;
    u8 field40_0x3a;
    u8 field41_0x3b;
    u8 field42_0x3c;
    u8 field43_0x3d;
    u8 field44_0x3e;
    u8 field45_0x3f;
    u8 field46_0x40;
    u8 field47_0x41;
    u8 field48_0x42;
    u8 field49_0x43;
    u8 field50_0x44;
    u8 field51_0x45;
    u8 field52_0x46;
    u8 field53_0x47;
    u8 field54_0x48;
    u8 field55_0x49;
    u8 field56_0x4a;
    u8 field57_0x4b;
    u8 field58_0x4c;
    u8 field59_0x4d;
    u8 field60_0x4e;
    u8 field61_0x4f;
    u8 field62_0x50;
    u8 field63_0x51;
    u8 field64_0x52;
    u8 field65_0x53;
    u8 field66_0x54;
    u8 field67_0x55;
    u8 field68_0x56;
    u8 field69_0x57;
    u8 field70_0x58;
    u8 field71_0x59;
    u8 field72_0x5a;
    u8 field73_0x5b;
    u8 field74_0x5c;
    u8 field75_0x5d;
    u8 field76_0x5e;
    u8 field77_0x5f;
    u8 field78_0x60;
    u8 field79_0x61;
    u8 field80_0x62;
    u8 field81_0x63;
    u8 field82_0x64;
    u8 field83_0x65;
    u8 field84_0x66;
    u8 field85_0x67;
    u8 field86_0x68;
    u8 field87_0x69;
    u8 field88_0x6a;
    u8 field89_0x6b;
    u8 field90_0x6c;
    u8 field91_0x6d;
    u8 field92_0x6e;
    u8 field93_0x6f;
    u8 field94_0x70;
    u8 field95_0x71;
    u8 field96_0x72;
    u8 field97_0x73;
    u8 field98_0x74;
    u8 field99_0x75;
    u8 field100_0x76;
    u8 field101_0x77;
    u8 field102_0x78;
    u8 field103_0x79;
    u8 field104_0x7a;
    u8 field105_0x7b;
    u8 field106_0x7c;
    u8 field107_0x7d;
    u8 field108_0x7e;
    u8 field109_0x7f;
    u8 field110_0x80;
    u8 field111_0x81;
    u8 field112_0x82;
    u8 field113_0x83;
    u8 field114_0x84;
    u8 field115_0x85;
    u8 field116_0x86;
    u8 field117_0x87;
    u8 field118_0x88;
    u8 field119_0x89;
    u8 field120_0x8a;
    u8 field121_0x8b;
    u8 field122_0x8c;
    u8 field123_0x8d;
    u8 field124_0x8e;
    u8 field125_0x8f;
    u8 field126_0x90;
    u8 field127_0x91;
    u8 field128_0x92;
    u8 field129_0x93;
    u8 field130_0x94;
    u8 field131_0x95;
    u8 field132_0x96;
    u8 field133_0x97;
    u8 field134_0x98;
    u8 field135_0x99;
    u8 field136_0x9a;
    u8 field137_0x9b;
    u8 field138_0x9c;
    u8 field139_0x9d;
    u8 field140_0x9e;
    u8 field141_0x9f;
    u8 field142_0xa0;
    u8 field143_0xa1;
    u8 field144_0xa2;
    u8 field145_0xa3;
    u8 field146_0xa4;
    u8 field147_0xa5;
    u8 field148_0xa6;
    u8 field149_0xa7;
    u8 field150_0xa8;
    u8 field151_0xa9;
    u8 field152_0xaa;
    u8 field153_0xab;
    u8 field154_0xac;
    u8 field155_0xad;
    u8 field156_0xae;
    u8 field157_0xaf;
    u8 field158_0xb0;
    u8 field159_0xb1;
    u8 field160_0xb2;
    u8 field161_0xb3;
    u8 field162_0xb4;
    u8 field163_0xb5;
    u8 field164_0xb6;
    u8 field165_0xb7;
    u8 field166_0xb8;
    u8 field167_0xb9;
    u8 field168_0xba;
    u8 field169_0xbb;
    u8 field170_0xbc;
    u8 field171_0xbd;
    u8 field172_0xbe;
    u8 field173_0xbf;
    u8 field174_0xc0;
    u8 field175_0xc1;
    u8 field176_0xc2;
    u8 field177_0xc3;
    u8 field178_0xc4;
    u8 field179_0xc5;
    u8 field180_0xc6;
    u8 field181_0xc7;
    u8 field182_0xc8;
    u8 field183_0xc9;
    u8 field184_0xca;
    u8 field185_0xcb;
    u8 field186_0xcc;
    u8 field187_0xcd;
    u8 field188_0xce;
    u8 field189_0xcf;
    u8 field190_0xd0;
    u8 field191_0xd1;
    struct StructD * field192_0xd2;
    u8 field193_0xd4;
    u8 field194_0xd5;
    u8 field195_0xd6;
    u8 field196_0xd7;
    u8 field197_0xd8;
    u8 field198_0xd9;
    u8 field199_0xda;
    u8 field200_0xdb;
    u8 field201_0xdc;
    u8 field202_0xdd;
    u8 field203_0xde;
    u8 field204_0xdf;
    u8 field205_0xe0;
    u8 field206_0xe1;
    u8 field207_0xe2;
    u8 field208_0xe3;
    u8 field209_0xe4;
    u8 field210_0xe5;
    u8 field211_0xe6;
    u8 field212_0xe7;
    u8 field213_0xe8;
    u8 field214_0xe9;
    u8 field215_0xea;
    u8 field216_0xeb;
    HMENU16 hmenu_0xec;
} StructD;

typedef struct _astruct_172 {
    u16 field0_0x0;
    u16 field1_0x2;
    u16 field2_0x4;
    i16 field3_0x6;
    u16 field4_0x8;
} astruct_172;

typedef struct _astruct_822 {
    char field0_0x0;
} astruct_822;

typedef struct _astruct_825 {
    u8 field0_0x0;
    u16 field_0x1;
} astruct_825;

typedef struct _struct_1000_0b20 {
    u16 field_0x0;
    code10 field_0x32;
    u16 field_0x1a;
} struct_1000_0b20;

typedef struct _struct_1000_1a54 {
    i16 field_0x14;
    u16 field_0x18;
    u16 field_0x1a;
    u16 field_0x1c;

} struct_1000_1a54;

typedef struct _astruct_1000_0782 {
    u16 field_0xe;
    u16 field_0x10;
    u16 field_0x8;
    u16 field_0x14;
} astruct_1000_0782;

typedef struct _astruct_1000_1902 {

} astruct_1000_1902;

#endif //OUTPOST_C_PROJ__STRUCTS_2_H_
