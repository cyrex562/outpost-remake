//
// Created by cyrex on 2/3/23.
//

#ifndef OUTPOST_1_SRC_WIN_TYPES_H_
#define OUTPOST_1_SRC_WIN_TYPES_H_

#include "op_int.h"
#include "structs/structs_2.h"
typedef struct tagBITMAPINFOHEADER tagBITMAPINFOHEADER ;typedef struct tagBITMAPINFOHEADER  *PtagBITMAPINFOHEADER;typedef struct tagBITMAPINFOHEADER{
        u32      biSize;
        long       biWidth;
        long       biHeight;
        u16       biPlanes;
        u16       biBitCount;
        u32      biCompression;
        u32      biSizeImage;
        long       biXPelsPerMeter;
        long       biYPelsPerMeter;
        u32      biClrUsed;
        u32      biClrImportant;
} BITMAPINFOHEADER, FARtypedef struct _CONTEXT _CONTEXT ;typedef struct _CONTEXT  *P_CONTEXT;typedef struct _CONTEXT CONTEXT;typedef struct _FLOATING_SAVE_AREA _FLOATING_SAVE_AREA ;typedef struct _FLOATING_SAVE_AREA  *P_FLOATING_SAVE_AREA;typedef struct _FLOATING_SAVE_AREA FLOATING_SAVE_AREA;struct _devicemodeA
{
    u8             dmDeviceName[32];
    u16             dmSpecVersion;
    u16             dmDriverVersion;
    u16             dmSize;
    u16             dmDriverExtra;
    u32            dmFields;
    union Union655 field_0x2c;
    short            dmColor;
    short            dmDuplex;
    short            dmYResolution;
    short            dmTTOption;
    short            dmCollate;
    u8             dmFormName[32];
    u16             dmLogPixels;
    u32            dmBitsPerPel;
    u32            dmPelsWidth;
    u32            dmPelsHeight;
    union Union658 field_0x74;
    u32            dmDisplayFrequency;
    u32            dmICMMethod;
    u32            dmICMi16ent;
    u32            dmMediaType;
    u32            dmDitherType;
    u32            dmReserved1;
    u32            dmReserved2;
    u32            dmPanningWidth;
    u32            dmPanningHeight;
};

struct tagBITMAPINFOHEADER
{
    u32 biSize;
    long  biWidth;
    long  biHeight;
    u16  biPlanes;
    u16  biBitCount;
    u32 biCompression;
    u32 biSizeImage;
    long  biXPelsPerMeter;
    long  biYPelsPerMeter;
    u32 biClrUsed;
    u32 biClrImportant;
};

struct _FLOATING_SAVE_AREA
{
    u32 ControlWord;
    u32 StatusWord;
    u32 TagWord;
    u32 ErrorOffset;
    u32 ErrorSelector;
    u32 DataOffset;
    u32 DataSelector;
    u8  RegisterArea[80];
    u32 Cr0NpxState;
};

struct _CONTEXT
{
    u32              ContextFlags;
    u32              Dr0;
    u32              Dr1;
    u32              Dr2;
    u32              Dr3;
    u32              Dr6;
    u32              Dr7;
    FLOATING_SAVE_AREA FloatSave;
    u32              SegGs;
    u32              SegFs;
    u32              SegEs;
    u32              SegDs;
    u32              Edi;
    u32              Esi;
    u32              Ebx;
    u32              Edx;
    u32              Ecx;
    u32              Eax;
    u32              Ebp;
    u32              Eip;
    u32              SegCs;
    u32              EFlags;
    u32              Esp;
    u32              SegSs;
    u8               ExtendedRegisters[512];
};

typedef union Union655 Union655;

union Union655
{
    Struct656 field0;
    Struct657 field1;
};

typedef union Union658 Union658;

union Union658
{
    DWORD dmDisplayFlags;
    DWORD dmNup;
};

#endif // OUTPOST_1_SRC_WIN_TYPES_H_
