//
// Created by cyrex562 on 2/12/23.
//

#ifndef OUTPOST_1_SRC_STRUCTS_STRUCTS_14_H_STRUCTS_1X_H_
#define OUTPOST_1_SRC_STRUCTS_STRUCTS_14_H_STRUCTS_1X_H_

#include "../../op_int.h"
#include "../structs_0xx/struct_21.h"
#include "../structs_1xx/structs_11x.h"
#include "../structs_1xx/structs_15x.h"
#include "../structs_1xx/structs_16x.h"
#include "../structs_1xx/structs_17x.h"
#include "../structs_1xx/structs_18x.h"
#include "../structs_3xx/structs_30x.h"
#include "../structs_3xx/structs_38x.h"
#include "../structs_4xx/structs_43x.h"
#include "structs_2x.h"
#include "structs_3x.h"
typedef struct Struct16 Struct16;
typedef struct Struct13 Struct13;
typedef struct Struct11 Struct11;
typedef struct Struct19 Struct19;
typedef struct Struct14 Struct14;
typedef struct Struct17 Struct17;
typedef struct Struct15 Struct15;
struct Struct16
{
    void      *field_0x0;
    u8         field_0x1;
    u16        field_0x2;
    u8         field_0x4;
    u8         field_0x5;
    u8         field_0x6;
    u8         field_0x7;
    u8         field_0x8;
    u8         field_0x9;
    u8         field_0xa;
    u8         field_0xb;
    u8         field_0xc;
    u8         field_0xd;
    u8         field_0xe;
    u8         field_0xf;
    u8         field_0x10;
    u8         field_0x11;
    u8         field_0x12;
    u8         field_0x13;
    u32 field_0x14;
    HGDIOBJ16  field_0x18;
    HGDIOBJ16  field_0x1a;
    BOOL16     field_0x1c;
    i16    field_0x1e;
    BOOL16     field_0x1c_addr_base;
    HGDIOBJ16  field_0x1a_addr_offset;
};
struct Struct13
{
    u8   field_0x0;
    u8   field_0x1;
    u8   field_0x2;
    u8   field_0x3;
    u8  *field_0x4;
    u8   field_0x8;
    u8   field_0x9;
    u8   field_0xa;
    u8   field_0xb;
    short           field_0xc;
    u16 *field_0xe;
};
struct Struct11
{
    u8   field_0x0;
    u8   field_0x1;
    u16         field_0x2;
    u8   field_0x4;
    u8   field_0x5;
    u8   field_0x6;
    u8   field_0x7;
    u8   field_0x8;
    u8   field_0x9;
    u32 *field_0xa;
    u16         field_0xc;
    u32 *field_0xe;
    u16         field_0x10;
    i16         field_0x12;
    u8   field_0x14;
    u8   field_0x15;
    u8   field_0x16;
    u8   field_0x17;
    u8   field_0x18;
    u8   field_0x19;
    BOOL16      field_0x1a;
};
struct Struct19
{
    u16       field_0x0;
    u16       field_0x2;
    u8 field_0x4;
    u8 field_0x5;
    u8 field_0x6;
    u8 field_0x7;
    u8 field_0x8;
    u8 field_0x9;
    u16     field_0xa;
    u16     field_0xc;
    u8 field_0xe;
    u8 field_0xf;
    u8 field_0x10;
    u8 field_0x11;
    u8 field_0x12;
    u8 field_0x13;
    u16     field_0x14;
    u16     field_0x16;
    u16      *field_0x18;
};
struct Struct14
{
    u8 field_0x0;
    u8 field_0x1;
    u8 field_0x2;
    u8 field_0x3;
    u8 field_0x4;
    u8 field_0x5;
    u16       field_0x6;
    u8 field_0x8;
    u8 field_0x9;
    u8 field_0xa;
    u8 field_0xb;
    u8 field_0xc;
    u8 field_0xd;
    u8 field_0xe;
    u8 field_0xf;
    char      field_0x10;
    u8 field_0x11;
    u8 field_0x12;
    u8 field_0x13;
    u8 field_0x14;
    u8 field_0x15;
    u8 field_0x16;
    u8 field_0x17;
    u8 field_0x18;
    u8 field_0x19;
    u8 field_0x1a;
    u8 field_0x1b;
    u8 field_0x1c;
    u8 field_0x1d;
    u8 field_0x1e;
    u8 field_0x1f;
    u8 field_0x20;
    u8 field_0x21;
    u8 field_0x22;
    u8 field_0x23;
    u8 field_0x24;
    u8 field_0x25;
    u8 field_0x26;
    u8 field_0x27;
    u8 field_0x28;
    u8 field_0x29;
    u8 field_0x2a;
    u8 field_0x2b;
    u8 field_0x2c;
    u8 field_0x2d;
    u8 field_0x2e;
    u8 field_0x2f;
    u8 field_0x30;
    u8 field_0x31;
    u8 field_0x32;
    u8 field_0x33;
    u8 field_0x34;
    u8 field_0x35;
    u8 field_0x36;
    u8 field_0x37;
    u8 field_0x38;
    u8 field_0x39;
    u8 field_0x3a;
    u8 field_0x3b;
    u8 field_0x3c;
    u8 field_0x3d;
    u8 field_0x3e;
    u8 field_0x3f;
    u8 field_0x40;
    u8 field_0x41;
    u8 field_0x42;
    u8 field_0x43;
    u8 field_0x44;
    u8 field_0x45;
    u8 field_0x46;
    u8 field_0x47;
    u8 field_0x48;
    u8 field_0x49;
    u8 field_0x4a;
    u8 field_0x4b;
    u8 field_0x4c;
    u8 field_0x4d;
    u8 field_0x4e;
    u8 field_0x4f;
    u8 field_0x50;
    u8 field_0x51;
    u8 field_0x52;
    u8 field_0x53;
    u8 field_0x54;
    u8 field_0x55;
    u8 field_0x56;
    u8 field_0x57;
    u8 field_0x58;
    u8 field_0x59;
    u8 field_0x5a;
    u8 field_0x5b;
    u8 field_0x5c;
    u8 field_0x5d;
    u8 field_0x5e;
    u8 field_0x5f;
    i16       field_0x60;
    i16       field_0x62;
    u16       field_0x64;
    i16       field_0x66;
    u8 field_0x68;
    u8 field_0x69;
    u8 field_0x6a;
    u8 field_0x6b;
    u8 field_0x6c;
    u8 field_0x6d;
    u16       field_0x6e;
    u8 field_0x70;
    u8 field_0x71;
    u8 field_0x72;
    u8 field_0x73;
    u8 field_0x74;
    u8 field_0x75;
    i16       field_0x76;
    u8 field_0x78;
    u8 field_0x79;
    u16       field_0x7a;
    i16       field_0x7c;
    i16       field_0x7e;
    i16       field_0x80;
};
struct Struct17
{
    u8   field_0x0;
    u8   field_0x1;
    u8   field_0x2;
    u8   field_0x3;
    u8   field_0x4;
    u8   field_0x5;
    u8   field_0x6;
    u8   field_0x7;
    u32 *field_0x8;
    u32 *field_0xc;
    u32 *field_0x10;
    u8   field_0x14;
    u8   field_0x15;
    u8   field_0x16;
    u8   field_0x17;
    u8   field_0x18;
    u8   field_0x19;
    u8   field_0x1a;
    u8   field_0x1b;
    u8   field_0x1c;
    u8   field_0x1d;
    u8   field_0x1e;
    u8   field_0x1f;
    u8   field_0x20;
    u8   field_0x21;
    u8   field_0x22;
    u8   field_0x23;
    u8   field_0x24;
    u8   field_0x25;
    u16         field_0x26;
    u16         field_0x28;
};
struct Struct15
{
    u8 field_0x0;
    u8 field_0x1;
    u8 field_0x2;
    u8 field_0x3;
    u8 field_0x4;
    u8 field_0x5;
    u8 field_0x6;
    u8 field_0x7;
    u8 field_0x8;
    u8 field_0x9;
    u8 field_0xa;
    u8 field_0xb;
    u8 field_0xc;
    u8 field_0xd;
    u8 field_0xe;
    u8 field_0xf;
    u8 field_0x10;
    u8 field_0x11;
    u8 field_0x12;
    u8 field_0x13;
    u8 field_0x14;
    u8 field_0x15;
    u8 field_0x16;
    u8 field_0x17;
    u8 field_0x18;
    u8 field_0x19;
    u8 field_0x1a;
    u8 field_0x1b;
    u8 field_0x1c;
    u8 field_0x1d;
    u8 field_0x1e;
    u8 field_0x1f;
    u8 field_0x20;
    u8 field_0x21;
    u8 field_0x22;
    u8 field_0x23;
    u8 field_0x24;
    u8 field_0x25;
    u8 field_0x26;
    u8 field_0x27;
    u8 field_0x28;
    u8 field_0x29;
    u8 field_0x2a;
    u8 field_0x2b;
    u8 field_0x2c;
    u8 field_0x2d;
    u8 field_0x2e;
    u8 field_0x2f;
    u8 field_0x30;
    u8 field_0x31;
    u8 field_0x32;
    u8 field_0x33;
    u8 field_0x34;
    u8 field_0x35;
    u8 field_0x36;
    u8 field_0x37;
    u8 field_0x38;
    u8 field_0x39;
    u8 field_0x3a;
    u8 field_0x3b;
    u8 field_0x3c;
    u8 field_0x3d;
    u8 field_0x3e;
    u8 field_0x3f;
    u8 field_0x40;
    u8 field_0x41;
    u8 field_0x42;
    u8 field_0x43;
    u8 field_0x44;
    u8 field_0x45;
    u8 field_0x46;
    u8 field_0x47;
    u8 field_0x48;
    u8 field_0x49;
    u8 field_0x4a;
    u8 field_0x4b;
    u8 field_0x4c;
    u8 field_0x4d;
    u8 field_0x4e;
    u8 field_0x4f;
    u8 field_0x50;
    u8 field_0x51;
    u8 field_0x52;
    u8 field_0x53;
    u8 field_0x54;
    u8 field_0x55;
    u8 field_0x56;
    u8 field_0x57;
    u8 field_0x58;
    u8 field_0x59;
    u8 field_0x5a;
    u8 field_0x5b;
    u8 field_0x5c;
    u8 field_0x5d;
    u8 field_0x5e;
    u8 field_0x5f;
    i16       field_0x60;
    u8 field_0x62;
    u8 field_0x63;
    i16       field_0x64;
    i16       field_0x66;
    u8 field_0x68;
    u8 field_0x69;
    u8 field_0x6a;
    u8 field_0x6b;
    u8 field_0x6c;
    u8 field_0x6d;
    u8 field_0x6e;
    u8 field_0x6f;
    u8 field_0x70;
    u8 field_0x71;
    u8 field_0x72;
    u8 field_0x73;
    u8 field_0x74;
    u8 field_0x75;
    u8 field_0x76;
    u8 field_0x77;
    u8 field_0x78;
    u8 field_0x79;
    i16       field_0x7a;
    i16       field_0x7c;
    u8 field_0x7e;
    u8 field_0x7f;
    i16       field_0x80;
    u16       field_0x82;
    i16       field_0x84;
    u16       field_0x86;
    i16       field_0x88;
};
#endif // OUTPOST_1_SRC_STRUCTS_STRUCTS_14_H_STRUCTS_1X_H_
