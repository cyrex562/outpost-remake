//
// Created by cyrex on 2022-05-22.
//

#ifndef OUTPOST_C_PROJ__TYPES_H_
#define OUTPOST_C_PROJ__TYPES_H_

#include <stdbool.h>

typedef char i8;
typedef unsigned char u8;
typedef unsigned short u16;
typedef short i16;
typedef unsigned long u32;
typedef long i32;
typedef unsigned long long u64;
typedef long long i64;

typedef let mut UINT16: u16;
typedef u32 SEGPTR;

typedef let mut HANDLE16: u16;
typedef HANDLE16 HCURSOR16;
typedef HANDLE16 HBRUSH16;
typedef HANDLE16 HMENU16;
typedef let mut WPARAM16: u16;
typedef HANDLE16 HWND16;
typedef HANDLE16 HMODULE16;
typedef HANDLE16 HACCEL16;
typedef HANDLE16 HDC16;
typedef HANDLE16 HFILE16;
typedef HANDLE16 HGLOBAL16;
typedef HANDLE16 HPEN16;
typedef HANDLE16 HGDIOBJ16;
typedef i32 LONG_PTR;
typedef LONG_PTR LPARAM;

typedef let mut BOOL16: u16;


typedef char * LPCSTR;
typedef u8 BYTE;
typedef i16 WORD;
typedef i16 INT16;
typedef let mut u16: u16;
typedef HANDLE16 HICON16;

typedef struct LOGPALETTE {
    WORD pal_version;
    WORD pal_num_entries;
} LOGPALETTE;

typedef struct PALETTEENTRY {
    BYTE pe_red;
    BYTE pe_green;
    BYTE pe_blue;
    BYTE pe_flags;
} PALETTEENTRY;

typedef struct RECT16 {
    INT16 x;
    INT16 y;
} RECT16;

typedef struct POINT16 {
    INT16 x;
    INT16 y;
} POINT16;

typedef struct WINDOWPLACEMENT16 {
    let mut length: u16;
    let mut flags: u16;
    let mut show_cmd: u16;
    struct POINT16 pt_min_position;
    struct POINT16 pt_max_position;
    struct RECT16 rc_normal_position;
} WINDOWPLACEMENT16;

typedef struct WNDCLASS16 {
    let mut style: u16;
pub fn * lpfn_wnd_proc;
    INT16 cb_cls_extra;
    INT16 cb_wnd_extra;
    HANDLE16 h_instance;
    HICON16 h_icon;
    HCURSOR16 h_cursor;
    HBRUSH16 hbr_background;
    let mut lpsz_menu_name: u32;
    let mut lpsz_class_name: u32;
} WNDCLASS16;

struct tagRGBQUAD {
    BYTE rgbBlue;
    BYTE rgbGreen;
    BYTE rgbRed;
    BYTE rgbReserved;
};

typedef i32 LONG;
typedef u32 DWORD;
typedef HANDLE16 HINSTANCE16;


typedef struct tagBITMAPINFOHEADER {
    DWORD biSize;
    LONG biWidth;
    LONG biHeight;
    WORD biPlanes;
    WORD biBitCount;
    DWORD biCompression;
    DWORD biSizeImage;
    LONG biXPelsPerMeter;
    LONG biYPelsPerMeter;
    DWORD biClrUsed;
    DWORD biClrImportant;
} BITMAPINFO;

typedef struct MSG16 {
    HWND16 hwnd;
    let mut message: u16;
    WPARAM16 wparam;
    LPARAM lparam;
    DWORD time;
    struct POINT16 pt;
} MSG16;

typedef void * LPVOID;

typedef struct PAINTSTRUCT16 {
    HDC16 hdc;
    BOOL16 f_erase;
    struct RECT16 rc_paint;
    BOOL16 f_restore;
    BOOL16 f_inc_update;
    BYTE rgb_reserved[16];
} PAINTSTRUCT16;



typedef HANDLE16 HTASK16;

typedef HANDLE16 HPALETTE16;

typedef HANDLE16 HRSRC16;

struct tagPOINT {
    LONG x;
    LONG y;
};

struct tagMSG {
    HWND16 hwnd;
    let mut message: u16;
    WPARAM16 wParam;
    LPARAM lParam;
    DWORD time;
    struct POINT16 pt;
};

typedef struct _POINTL {
    LONG x;
    LONG y;
} POINTL;



struct _struct_657 {
    POINTL dmPosition;
    DWORD dmDisplayOrientation;
    DWORD dmDisplayFixedOutput;
};

struct _struct_656 {
    short dmOrientation;
    short dmPaperSize;
    short dmPaperLength;
    short dmPaperWidth;
    short dmScale;
    short dmCopies;
    short dmDefaultSource;
    short dmPrintQuality;
};


union _union_655 {
    struct _struct_656 field0;
    struct _struct_657 field1;
};


union _union_658 {
    DWORD dmDisplayFlags;
    DWORD dmNup;
};

typedef struct _devicemodeA {
    BYTE dmDeviceName[32];
    WORD dmSpecVersion;
    WORD dmDriverVersion;
    WORD dmSize;
    WORD dmDriverExtra;
    DWORD dmFields;
    union _union_655 field6_0x2c;
    short dmColor;
    short dmDuplex;
    short dmYResolution;
    short dmTTOption;
    short dmCollate;
    BYTE dmFormName[32];
    WORD dmLogPixels;
    DWORD dmBitsPerPel;
    DWORD dmPelsWidth;
    DWORD dmPelsHeight;
    union _union_658 field17_0x74;
    DWORD dmDisplayFrequency;
    DWORD dmICMMethod;
    DWORD dmICMIntent;
    DWORD dmMediaType;
    DWORD dmDitherType;
    DWORD dmReserved1;
    DWORD dmReserved2;
    DWORD dmPanningWidth;
    DWORD dmPanningHeight;
} DEVMODEA;


typedef struct _FLOATING_SAVE_AREA {
    DWORD ControlWord;
    DWORD StatusWord;
    DWORD TagWord;
    DWORD ErrorOffset;
    DWORD ErrorSelector;
    DWORD DataOffset;
    DWORD DataSelector;
    BYTE RegisterArea[80];
    DWORD Cr0NpxState;
} FLOATING_SAVE_AREA;

typedef struct _CONTEXT {
    DWORD ContextFlags;
    DWORD Dr0;
    DWORD Dr1;
    DWORD Dr2;
    DWORD Dr3;
    DWORD Dr6;
    DWORD Dr7;
    FLOATING_SAVE_AREA FloatSave;
    DWORD SegGs;
    DWORD SegFs;
    DWORD SegEs;
    DWORD SegDs;
    DWORD Edi;
    DWORD Esi;
    DWORD Ebx;
    DWORD Edx;
    DWORD Ecx;
    DWORD Eax;
    DWORD Ebp;
    DWORD Eip;
    DWORD SegCs;
    DWORD EFlags;
    DWORD Esp;
    DWORD SegSs;
    BYTE ExtendedRegisters[512];
} CONTEXT;


typedef struct tagRECT {
    LONG left;
    LONG top;
    LONG right;
    LONG bottom;
} RECT;

typedef WORD ATOM;

typedef void * PVOID;

typedef LONG_PTR LRESULT;

typedef DWORD COLORREF;

#define NULL 0

#endif //OUTPOST_C_PROJ__TYPES_H_
