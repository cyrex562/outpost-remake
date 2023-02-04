#pragma once

typedef unsigned char      u8;
typedef char               i8;
typedef unsigned short     u16;
typedef short              i16;
typedef unsigned int       u32;
typedef int                i32;
typedef unsigned long long u64;
typedef long long          i64;

typedef char *cstring;

typedef u8 BYTE;

typedef u16 HANDLE16;

typedef HANDLE16 HCURSOR16;

typedef struct PALETTEENTRY PALETTEENTRY, *PPALETTEENTRY;

struct PALETTEENTRY
{
    BYTE pe_red;
    BYTE pe_green;
    BYTE pe_blue;
    BYTE pe_flags;
};

typedef HANDLE16 HBRUSH16;

typedef HANDLE16 HMENU16;

typedef u16 WPARAM16;

typedef HANDLE16 HWND16;

typedef HANDLE16 HMODULE16;

typedef HANDLE16 HACCEL16;


typedef struct WINDOWPLACEMENT16 WINDOWPLACEMENT16, *PWINDOWPLACEMENT16;

typedef struct POINT POINT, *PPOINT;

typedef struct RECT16 RECT16, *PRECT16;

struct RECT16
{
    i16 x;
    i16 y;
};

struct POINT
{
    i16 x;
    i16 y;
};

struct WINDOWPLACEMENT16
{
    u16           length;
    u16           flags;
    u16           show_cmd;
    struct POINT  pt_min_position;
    struct POINT  pt_max_position;
    struct RECT16 rc_normal_position;
};

typedef struct WNDCLASS16 WNDCLASS16, *PWNDCLASS16;

typedef void *LPVOID;

typedef HANDLE16 HICON16;

typedef u32 SEGPTR;

struct WNDCLASS16
{
    u16       style;
    LPVOID    lpfn_wnd_proc;
    i16       cb_cls_extra;
    i16       cb_wnd_extra;
    HANDLE16  h_instance;
    HICON16   h_icon;
    HCURSOR16 h_cursor;
    HBRUSH16  hbr_background;
    SEGPTR    lpsz_menu_name;
    SEGPTR    lpsz_class_name;
};

typedef struct TwoWords TwoWords, *PTwoWords;

struct TwoWords
{
    u16 a_0x0;
    u16 b_0x2;
};

typedef u32 COLORREF;

typedef HANDLE16 HDC16;

typedef HANDLE16 HFILE16;

typedef HANDLE16 HGLOBAL16;

typedef HANDLE16 HPEN16;

typedef u16 BOOL16;

#define false 0
#define true 1

typedef HANDLE16 HGDIOBJ16;

typedef struct LOGPALETTE LOGPALETTE, *PLOGPALETTE;

typedef u16 WORD;

struct LOGPALETTE
{
    WORD pal_version;
    WORD pal_num_entries;
};

typedef struct MSG16 MSG16, *PMSG16;

typedef long LONG_PTR;

typedef LONG_PTR LPARAM;

typedef u32 DWORD;

struct MSG16
{
    HWND16       hwnd;
    u16          message;
    WPARAM16     wparam;
    LPARAM       lparam;
    DWORD        time;
    struct POINT pt;
};

typedef struct BITMAPINFOHEADER BITMAPINFOHEADER, *PBITMAPINFOHEADER;

typedef long LONG;

struct BITMAPINFOHEADER
{
    DWORD bi_size;
    LONG  bi_width;
    LONG  bi_height;
    WORD  bi_planes;
    WORD  bi_bit_count;
    DWORD bi_compression;
    DWORD bi_size_image;
    LONG  bi_x_pels_per_meter;
    LONG  bi_y_pels_per_meter;
    DWORD bi_clr_used;
    DWORD bi_clr_important;
};

typedef HANDLE16 HINSTANCE16;

typedef struct PAi16STRUCT16 PAi16STRUCT16, *PPAi16STRUCT16;

struct PAi16STRUCT16
{
    HDC16         hdc;
    BOOL16        f_erase;
    struct RECT16 rc_pai16;
    BOOL16        f_restore;
    BOOL16        f_inc_update;
    BYTE          rgb_reserved[16];
};

typedef struct BITMAPINFO BITMAPINFO, *PBITMAPINFO;

typedef struct tagRGBQUAD tagRGBQUAD, *PtagRGBQUAD;

typedef struct tagRGBQUAD RGBQUAD;

struct tagRGBQUAD
{
    BYTE rgbBlue;
    BYTE rgbGreen;
    BYTE rgbRed;
    BYTE rgbReserved;
};

struct BITMAPINFO
{
    BITMAPINFOHEADER bim_header;
    RGBQUAD          bmi_colors;
};


typedef HANDLE16 HTASK16;

typedef HANDLE16 HPALETTE16;

typedef HANDLE16 HRSRC16;


// WARNING! conflicting data type names: /RGBQUAD - /wingdi.h/RGBQUAD

typedef struct tagMSG tagMSG, *PtagMSG;

typedef struct tagMSG MSG;

typedef struct HWND__ HWND__, *PHWND__;

typedef struct HWND__ *HWND;

typedef u16 u16_PTR;

typedef u16_PTR WPARAM;

typedef struct tagPOINT tagPOINT, *PtagPOINT;

// typedef struct tagPOINT POINT;

struct tagPOINT
{
    LONG x;
    LONG y;
};

struct tagMSG
{
    HWND   hwnd;
    u16    message;
    WPARAM wParam;
    LPARAM lParam;
    DWORD  time;
    POINT  pt;
};

struct HWND__
{
    i16 unused;
};

typedef struct _devicemodeA _devicemodeA, *P_devicemodeA;

typedef struct _devicemodeA DEVMODEA;


struct _POINTL
{
    LONG x;
    LONG y;
};

struct CONTEXT
{
};
