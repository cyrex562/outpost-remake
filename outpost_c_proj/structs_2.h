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
} Struct7;

typedef struct _astruct_99 {
    u16 field0_0x0;
    u16 field1_0x2;
    u32 field2_0x4;
    u32 field3_0x8;
    u16 field4_0xc;
    u16 field5_0xe;
} Struct99;

typedef struct _astruct_57 {
    u16 field0_0x0;
    u16 field1_0x2;
    u16 field2_0x4;
    u16 field3_0x6;
    u16 field4_0x8;
    u16 field5_0xa;
    u16 field6_0xc;
    u16 field7_0xe;
    u16 field8_0x10;
    u16 field9_0x12;
    u32 field10_0x14;
    u32 field11_0x18;
    u8 field12_0x1c;
    u8 field13_0x1d;
    u8 field14_0x1e;
    u8 field15_0x1f;
    u8 field16_0x20;
    u8 field17_0x21;
    u16 field18_0x22;
    u8 field19_0x24;
    u8 field20_0x25;
    u16 field21_0x26;
    u8 field22_0x28;
    u8 field23_0x29;
    u8 field24_0x2a;
    u8 field25_0x2b;
    u8 field26_0x2c;
    u8 field27_0x2d;
    u8 field28_0x2e;
    u8 field29_0x2f;
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
    u16 field78_0x60;
    INT16 field79_0x62;
    INT16 field80_0x64;
    INT16 field81_0x66;
    u16 field82_0x68;
    u32 field83_0x6a;
    u16 field84_0x6e;
    u32 field85_0x70;
    u16 field86_0x74;
    u16 field87_0x76;
    u16 field88_0x78;
    u8 field89_0x7a;
    u8 field90_0x7b;
    u8 field91_0x7c;
    u8 field92_0x7d;
    u8 field93_0x7e;
    u8 field94_0x7f;
    u8 field95_0x80;
    u8 field96_0x81;
    u8 field97_0x82;
    u8 field98_0x83;
    u8 field99_0x84;
    u8 field100_0x85;
    u8 field101_0x86;
    u8 field102_0x87;
    u8 field103_0x88;
    u8 field104_0x89;
    u16 field105_0x8a;
    u16 field106_0x8c;
} Struct57;




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
} Struct611;


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
} Struct172;

typedef struct _astruct_822 {
    char field0_0x0;
} Struct822;

typedef struct _astruct_825 {
    u8 field0_0x0;
    u16 field_0x1;
} Struct825;

typedef struct _struct_1000_0b20 {
    u16 field_0x0;
    code10 field_0x32;
    u16 field_0x1a;
} Struct_1000_0b20;

typedef struct _struct_1000_1a54 {
    i16 field_0x14;
    u16 field_0x18;
    u16 field_0x1a;
    u16 field_0x1c;

} Struct_1000_1a54;

typedef struct _astruct_1000_0782 {
    u16 field_0xe;
    u16 field_0x10;
    u16 field_0x8;
    u16 field_0x14;
} Struct_1000_0782;

typedef struct _astruct_1000_1902 {

} Struct_1000_1902;

typedef struct _astruct_27 {
    u8 field0_0x0;
    u8 field1_0x1;
    u8 field2_0x2;
    u8 field3_0x3;
    u8 field4_0x4;
    u8 field5_0x5;
    u8 field6_0x6;
    u8 field7_0x7;
    u8 field8_0x8;
    u8 field9_0x9;
    u8 field10_0xa;
    u8 field11_0xb;
    u8 field12_0xc;
    u8 field13_0xd;
    u8 field14_0xe;
    u8 field15_0xf;
    u8 field16_0x10;
    u8 field17_0x11;
    u16 field18_0x12;
    u16 field19_0x14;
    i16 field20_0x16;
    u8 field21_0x18;
    u8 field22_0x19;
    u16 field23_0x1a;
    u8 field24_0x1c;
    u8 field25_0x1d;
    u8 field26_0x1e;
    u8 field27_0x1f;
    u8 field28_0x20;
    u8 field29_0x21;
    u32 field30_0x22;
    u8 field31_0x26;
    u8 field32_0x27;
    u16 field33_0x28;
    u16 field34_0x2a;
    u16 field35_0x2c;
    u32 field36_0x2e;
    u8 field37_0x32;
    u8 field38_0x33;
    u8 field39_0x34;
    u8 field40_0x35;
    u8 field41_0x36;
    u8 field42_0x37;
    u8 field43_0x38;
    u8 field44_0x39;
    u8 field45_0x3a;
    u8 field46_0x3b;
    u8 field47_0x3c;
    u8 field48_0x3d;
    u8 field49_0x3e;
    u8 field50_0x3f;
    u8 field51_0x40;
    u8 field52_0x41;
    u8 field53_0x42;
    u8 field54_0x43;
    u8 field55_0x44;
    u8 field56_0x45;
    u8 field57_0x46;
    u8 field58_0x47;
    u8 field59_0x48;
    u8 field60_0x49;
    u8 field61_0x4a;
    u8 field62_0x4b;
    u8 field63_0x4c;
    u8 field64_0x4d;
    u8 field65_0x4e;
    u8 field66_0x4f;
    u8 field67_0x50;
    u8 field68_0x51;
    i32 field69_0x52;
    u16 field_0xa;
    u16 field_0x10;
} Struct27;


typedef struct _astruct_638 {
    u8 field0_0x0;
    u8 field1_0x1;
    u16 field2_0x2;
    u8 field3_0x4;
    u8 field4_0x5;
    u8 field5_0x6;
    u8 field6_0x7;
    u8 field7_0x8;
    u8 field8_0x9;
    u32 field9_0xa;
    u32 field10_0xe;
    u32 field11_0x12;
    u32 field12_0x16;
    u32 field13_0x1a;
    u32 field14_0x1e;
    u8 field15_0x22;
    u8 field16_0x23;
    u8 field17_0x24;
    u8 field18_0x25;
    u8 field19_0x26;
    u8 field20_0x27;
    u8 field21_0x28;
    u8 field22_0x29;
    u8 field23_0x2a;
    u8 field24_0x2b;
    u8 field25_0x2c;
    u8 field26_0x2d;
    u8 field27_0x2e;
    u8 field28_0x2f;
    u8 field29_0x30;
    u8 field30_0x31;
    u8 field31_0x32;
    u8 field32_0x33;
    u8 field33_0x34;
    u8 field34_0x35;
    u8 field35_0x36;
    u8 field36_0x37;
    u8 field37_0x38;
    u8 field38_0x39;
    u8 field39_0x3a;
    u8 field40_0x3b;
    u8 field41_0x3c;
    u8 field42_0x3d;
    u8 field43_0x3e;
    u8 field44_0x3f;
    u8 field45_0x40;
    u8 field46_0x41;
    u8 field47_0x42;
    u8 field48_0x43;
    u8 field49_0x44;
    u8 field50_0x45;
    u8 field51_0x46;
    u8 field52_0x47;
    u8 field53_0x48;
    u8 field54_0x49;
    u8 field55_0x4a;
    u8 field56_0x4b;
    u8 field57_0x4c;
    u8 field58_0x4d;
    u8 field59_0x4e;
    u8 field60_0x4f;
    u8 field61_0x50;
    u8 field62_0x51;
    u8 field63_0x52;
    u8 field64_0x53;
    u8 field65_0x54;
    u8 field66_0x55;
    u8 field67_0x56;
    u8 field68_0x57;
    u8 field69_0x58;
    u8 field70_0x59;
    u8 field71_0x5a;
    u8 field72_0x5b;
    u8 field73_0x5c;
    u8 field74_0x5d;
    u8 field75_0x5e;
    u8 field76_0x5f;
    u8 field77_0x60;
    u8 field78_0x61;
    u8 field79_0x62;
    u8 field80_0x63;
    u8 field81_0x64;
    u8 field82_0x65;
    u8 field83_0x66;
    u8 field84_0x67;
    u8 field85_0x68;
    u8 field86_0x69;
    u8 field87_0x6a;
    u8 field88_0x6b;
    u8 field89_0x6c;
    u8 field90_0x6d;
    u8 field91_0x6e;
    u8 field92_0x6f;
    u8 field93_0x70;
    u8 field94_0x71;
    u8 field95_0x72;
    u8 field96_0x73;
    u8 field97_0x74;
    u8 field98_0x75;
    u8 field99_0x76;
    u8 field100_0x77;
    u8 field101_0x78;
    u8 field102_0x79;
    u8 field103_0x7a;
    u8 field104_0x7b;
    u8 field105_0x7c;
    u8 field106_0x7d;
    u8 field107_0x7e;
    u8 field108_0x7f;
    u8 field109_0x80;
    u8 field110_0x81;
    u8 field111_0x82;
    u8 field112_0x83;
    u8 field113_0x84;
    u8 field114_0x85;
    u8 field115_0x86;
    u8 field116_0x87;
    u8 field117_0x88;
    u8 field118_0x89;
    u8 field119_0x8a;
    u8 field120_0x8b;
    u8 field121_0x8c;
    u8 field122_0x8d;
    u8 field123_0x8e;
    u8 field124_0x8f;
    u8 field125_0x90;
    u8 field126_0x91;
    u8 field127_0x92;
    u8 field128_0x93;
    u8 field129_0x94;
    u8 field130_0x95;
    u8 field131_0x96;
    u8 field132_0x97;
    u8 field133_0x98;
    u8 field134_0x99;
    u8 field135_0x9a;
    u8 field136_0x9b;
    u8 field137_0x9c;
    u8 field138_0x9d;
    u8 field139_0x9e;
    u8 field140_0x9f;
    u8 field141_0xa0;
    u8 field142_0xa1;
    u8 field143_0xa2;
    u8 field144_0xa3;
    u8 field145_0xa4;
    u8 field146_0xa5;
    u8 field147_0xa6;
    u8 field148_0xa7;
    u8 field149_0xa8;
    u8 field150_0xa9;
    u8 field151_0xaa;
    u8 field152_0xab;
    u8 field153_0xac;
    u8 field154_0xad;
    u8 field155_0xae;
    u8 field156_0xaf;
    u8 field157_0xb0;
    u8 field158_0xb1;
    u8 field159_0xb2;
    u8 field160_0xb3;
    u8 field161_0xb4;
    u8 field162_0xb5;
    u8 field163_0xb6;
    u8 field164_0xb7;
    u8 field165_0xb8;
    u8 field166_0xb9;
    u8 field167_0xba;
    u8 field168_0xbb;
    u8 field169_0xbc;
    u8 field170_0xbd;
    u8 field171_0xbe;
    u8 field172_0xbf;
    u8 field173_0xc0;
    u8 field174_0xc1;
    u8 field175_0xc2;
    u8 field176_0xc3;
    u8 field177_0xc4;
    u8 field178_0xc5;
    u8 field179_0xc6;
    u8 field180_0xc7;
    u8 field181_0xc8;
    u8 field182_0xc9;
    u8 field183_0xca;
    u8 field184_0xcb;
    u8 field185_0xcc;
    u8 field186_0xcd;
    u8 field187_0xce;
    u8 field188_0xcf;
    u8 field189_0xd0;
    u8 field190_0xd1;
    u8 field191_0xd2;
    u8 field192_0xd3;
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
    u8 field217_0xec;
    u8 field218_0xed;
    u8 field219_0xee;
    u8 field220_0xef;
    u8 field221_0xf0;
    u8 field222_0xf1;
    u8 field223_0xf2;
    u8 field224_0xf3;
    u8 field225_0xf4;
    u8 field226_0xf5;
    u8 field227_0xf6;
    u8 field228_0xf7;
    u8 field229_0xf8;
    u8 field230_0xf9;
    u8 field231_0xfa;
    u8 field232_0xfb;
    u8 field233_0xfc;
    u8 field234_0xfd;
    u8 field235_0xfe;
    u8 field236_0xff;
    u8 field237_0x100;
    u8 field238_0x101;
    u8 field239_0x102;
    u8 field240_0x103;
    u8 field241_0x104;
    u8 field242_0x105;
    u8 field243_0x106;
    u8 field244_0x107;
    u8 field245_0x108;
    u8 field246_0x109;
    u8 field247_0x10a;
    u8 field248_0x10b;
    u8 field249_0x10c;
    u8 field250_0x10d;
    u8 field251_0x10e;
    u8 field252_0x10f;
    u8 field253_0x110;
    u8 field254_0x111;
    u8 field255_0x112;
    u8 field256_0x113;
    u8 field257_0x114;
    u8 field258_0x115;
    u8 field259_0x116;
    u8 field260_0x117;
    u8 field261_0x118;
    u8 field262_0x119;
    u8 field263_0x11a;
    u8 field264_0x11b;
    u8 field265_0x11c;
    u8 field266_0x11d;
    u8 field267_0x11e;
    u8 field268_0x11f;
    u8 field269_0x120;
    u8 field270_0x121;
    u16 field271_0x122;
    i16 field272_0x124;
    u32 field273_0x126;
    u32 field274_0x12a;
    u16 field275_0x12e;
    u16 field276_0x130;
    u32 field277_0x132;
    u32 field278_0x136;
    u16 field279_0x13a;
    u16 field280_0x13c;
    u32 field281_0x13e;
    u32 field282_0x142;
} Struct638;


typedef struct _astruct_19 {
    u16 offset_0x0;
    u16 segment_0x2;
    u32 field2_0x4;
    u16 field3_0x8;
    INT16 horiz_res_0xa;
    u16 ver_res_0xc;
    u8 field6_0xe;
    u8 field7_0xf;
    u16 field8_0x10;
    u16 field9_0x12;
    INT16 field10_0x14;
    INT16 field11_0x16;
    u16 field12_0x18;
    u8 field13_0x1a;
    u8 field14_0x1b;
    u16 field15_0x1c;
    u16 field16_0x1e;
    u16 field17_0x20;
    u16 field18_0x22;
    u16 field19_0x24;
    struct StructD * field20_0x26;
    u16 field21_0x2a;
    u16 field22_0x2c;
    u16 field23_0x2e;
    u16 field24_0x30;
    u32 field25_0x32;
    u16 field26_0x36;
    u16 field27_0x38;
    u8 field28_0x3a;
    u8 field29_0x3b;
    u8 field30_0x3c;
    u8 field31_0x3d;
    u8 field32_0x3e;
    u8 field33_0x3f;
    u32 field34_0x40;
    u16 field35_0x44;
    u32 field36_0x46;
    u16 field37_0x4a;
    u8 field38_0x4c;
    u8 field39_0x4d;
    u8 field40_0x4e;
    u8 field41_0x4f;
    u8 field42_0x50;
    u8 field43_0x51;
    u16 field44_0x52;
    u8 field45_0x54;
    u8 field46_0x55;
    u16 field47_0x56;
    u8 field48_0x58;
    u8 field49_0x59;
    u8 field50_0x5a;
    u8 field51_0x5b;
    u16 field52_0x5c;
    u16 field53_0x5e;
} Struct19;

#endif //OUTPOST_C_PROJ__STRUCTS_2_H_
