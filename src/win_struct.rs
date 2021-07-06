pub type HANDLE16 = u16;
// typedef HANDLE16 HBRUSH16;
pub type HBRUSH16 = HANDLE16;
// typedef HANDLE16 HMENU16;
pub type HMENU16 = HANDLE16;
// typedef ushort WPARAM16;
pub type WPARAM16 = u16;
// typedef HANDLE16 HWND16;
pub type HWND16 = HANDLE16;
// typedef HANDLE16 HMODULE16;
pub type HMODULE16 = HANDLE16;
// typedef HANDLE16 HACCEL16;
pub type HACCEL16 = HANDLE16;
// typedef HANDLE16 HTASK16;
pub type HTASK16 = HANDLE16;
// typedef uint32_t COLORREF;
pub type COLORREF = u32;
// typedef HANDLE16 HDC16;
pub type HDC16 = HANDLE16;
// typedef HANDLE16 HFILE16;
pub type HFILE16 = HANDLE16;
// typedef HANDLE16 HGLOBAL16;
pub type HGLOBAL16 = HANDLE16;
// typedef char i8;
// typedef HANDLE16 HPEN16;
pub type HPEN16 = HANDLE16;
// typedef HANDLE16 HINSTANCE16;
pub type HINSTANCE16 = HANDLE16;

pub type HICON16 = HANDLE16;

// typedef long LONG_PTR;
pub type LONG_PTR = libc::c_long;
// typedef LONG_PTR LPARAM;
pub type LPARAM = LONG_PTR;

pub type HGDIOBJ16 = HANDLE16;

pub type HPALETTE16 = HANDLE16;

// typedef LONG_PTR LRESULT;
pub type LRESULT = LONG_PTR;

pub struct FLOATING_SAVE_AREA {
    // DWORD ControlWord;
    control_word: u32,
    // DWORD StatusWord;
    status_word: u32,
    // DWORD TagWord;
    tag_word: u32,
    // DWORD ErrorOffset;
    error_offset: u32,
    // DWORD ErrorSelector;
    error_selector: u32,
    // DWORD DataOffset;
    data_offset: u32,
    // DWORD DataSelector;
    data_selector: u32,
    // BYTE RegisterArea[80];
    register_area: [u8;80],
    // DWORD Cr0NpxState;
    cr0_npx_state: u32
}

pub struct CONTEXT {
    // DWORD ContextFlags;
    context_flags: u32,
    // DWORD Dr0;
    dr0: u32,
    // DWORD Dr1;
    dr1: u32,
    // DWORD Dr2;
    dr2: u32,
    // DWORD Dr3;
    dr3: u32,
    // DWORD Dr6;
    dr6: u32,
    // DWORD Dr7;
    dr7: u32,
    // FLOATING_SAVE_AREA FloatSave;
    float_save: FLOATING_SAVE_AREA,
    // DWORD SegGs;
    seg_gs: u32,
    // DWORD SegFs;
    seg_fs: u32,
    // DWORD SegEs;
    seg_es: u32,
    // DWORD SegDs;
    seg_ds: u32
    // DWORD Edi;,
    edi: u32,
    // DWORD Esi;
    esi: u32,
    // DWORD Ebx;
    ebx: u32,
    // DWORD Edx;
    edx: u32,
    // DWORD Ecx;
    ecx: u32,
    // DWORD Eax;
    eax: u32,
    // DWORD Ebp;
    ebp: u32,
    // DWORD Eip;
    eip: u32,
    // DWORD SegCs;
    seg_cs: u32,
    // DWORD EFlags;
    e_flags: u32,
    // DWORD Esp;
    esp: u32,
    // DWORD SegSs;
    seg_ss: u32,
    // BYTE ExtendedRegisters[512];
    extended_registers: [u8;512]
}

impl CONTEXT {
    pub fn new() -> CONTEXT {
        CONTEXT {
            context_flags: 0,
            dr0: 0,
            dr1: 0,
            dr2: 0,
            dr3: 0,
            dr6: 0,
            dr7: 0,
            float_save: FLOATING_SAVE_AREA {},
            seg_gs: 0,
            seg_fs: 0,
            seg_es: 0,
            seg_ds: 0,
            edi: 0,
            esi: 0,
            ebx: 0,
            edx: 0,
            ecx: 0,
            eax: 0,
            ebp: 0,
            eip: 0,
            seg_cs: 0,
            e_flags: 0,
            esp: 0,
            seg_ss: 0,
            extended_registers: []
        }
    }
}



struct RECT16 {
    INT16 x;
    INT16 y;
};

struct POINT16 {
    INT16 x;
    INT16 y;
};

struct WINDOWPLACEMENT16 {
    UINT16 length;
    UINT16 flags;
    UINT16 show_cmd;
    struct POINT16 pt_min_position;
    struct POINT16 pt_max_position;
    struct RECT16 rc_normal_position;
};

typedef struct WNDCLASS16 WNDCLASS16, *PWNDCLASS16;

typedef void * LPVOID;

typedef HANDLE16 HICON16;

typedef uint uint32_t;

typedef uint32_t SEGPTR;

struct WNDCLASS16 {
    UINT16 style;
    LPVOID lpfn_wnd_proc;
    INT16 cb_cls_extra;
    INT16 cb_wnd_extra;
    HANDLE16 h_instance;
    HICON16 h_icon;
    HCURSOR16 h_cursor;
    HBRUSH16 hbr_background;
    SEGPTR lpsz_menu_name;
    SEGPTR lpsz_class_name;
};

typedef long i32;

typedef struct TwoWords TwoWords, *PTwoWords;

struct TwoWords {
    UINT16 a_0x0;
    UINT16 b_0x2;
};



typedef ushort BOOL16;

typedef HANDLE16 HGDIOBJ16;

typedef struct LOGPALETTE LOGPALETTE, *PLOGPALETTE;

typedef ushort WORD;

struct LOGPALETTE {
    WORD pal_version;
    WORD pal_num_entries;
};

// typedef struct MSG16 MSG16, *PMSG16;



// typedef ulong DWORD;

struct MSG16 {
    HWND16 hwnd;
    UINT16 message;
    WPARAM16 wparam;
    LPARAM lparam;
    DWORD time;
    struct POINT16 pt;
};

typedef struct BITMAPINFOHEADER BITMAPINFOHEADER, *PBITMAPINFOHEADER;

typedef long LONG;

struct BITMAPINFOHEADER {
    DWORD bi_size;
    LONG bi_width;
    LONG bi_height;
    WORD bi_planes;
    WORD bi_bit_count;
    DWORD bi_compression;
    DWORD bi_size_image;
    LONG bi_x_pels_per_meter;
    LONG bi_y_pels_per_meter;
    DWORD bi_clr_used;
    DWORD bi_clr_important;
};



// typedef struct PAINTSTRUCT16 PAINTSTRUCT16, *PPAINTSTRUCT16;

struct PAINTSTRUCT16 {
    HDC16 hdc;
    BOOL16 f_erase;
    struct RECT16 rc_paint;
    BOOL16 f_restore;
    BOOL16 f_inc_update;
    BYTE rgb_reserved[16];
};

typedef struct BITMAPINFO BITMAPINFO, *PBITMAPINFO;


// WARNING! conflicting data type names: /wingdi.h/BITMAPINFOHEADER - /BITMAPINFOHEADER

typedef struct tagRGBQUAD tagRGBQUAD, *PtagRGBQUAD;

typedef struct tagRGBQUAD RGBQUAD;

struct tagRGBQUAD {
    BYTE rgbBlue;
    BYTE rgbGreen;
    BYTE rgbRed;
    BYTE rgbReserved;
};

struct BITMAPINFO {
    BITMAPINFOHEADER bim_header;
    RGBQUAD bmi_colors;
};

typedef ushort u16;



typedef HANDLE16 HPALETTE16;

typedef HANDLE16 HRSRC16;

typedef ulong u32;


// WARNING! conflicting data type names: /RGBQUAD - /wingdi.h/RGBQUAD

typedef struct tagMSG tagMSG, *PtagMSG;

typedef struct tagMSG MSG;

typedef struct HWND__ HWND__, *PHWND__;

typedef struct HWND__ * HWND;

typedef uint UINT;

typedef uint UINT_PTR;

typedef UINT_PTR WPARAM;

typedef struct tagPOINT tagPOINT, *PtagPOINT;

typedef struct tagPOINT POINT;

struct tagPOINT {
    LONG x;
    LONG y;
};

struct tagMSG {
    HWND hwnd;
    UINT message;
    WPARAM wParam;
    LPARAM lParam;
    DWORD time;
    POINT pt;
};

struct HWND__ {
    int unused;
};

typedef struct _devicemodeA _devicemodeA, *P_devicemodeA;

typedef struct _devicemodeA DEVMODEA;

typedef union _union_655 _union_655, *P_union_655;

typedef union _union_658 _union_658, *P_union_658;

typedef struct _struct_656 _struct_656, *P_struct_656;

typedef struct _struct_657 _struct_657, *P_struct_657;

typedef struct _POINTL _POINTL, *P_POINTL;

typedef struct _POINTL POINTL;

struct _POINTL {
    LONG x;
    LONG y;
};

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

struct _devicemodeA {
    BYTE dmDeviceName[32];
    WORD dmSpecVersion;
    WORD dmDriverVersion;
    WORD dmSize;
    WORD dmDriverExtra;
    DWORD dmFields;
    union _union_655 field_0x2c;
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
    union _union_658 field_0x74;
    DWORD dmDisplayFrequency;
    DWORD dmICMMethod;
    DWORD dmICMIntent;
    DWORD dmMediaType;
    DWORD dmDitherType;
    DWORD dmReserved1;
    DWORD dmReserved2;
    DWORD dmPanningWidth;
    DWORD dmPanningHeight;
};

typedef struct tagBITMAPINFOHEADER tagBITMAPINFOHEADER, *PtagBITMAPINFOHEADER;

struct tagBITMAPINFOHEADER {
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
};

// typedef struct _CONTEXT _CONTEXT, *P_CONTEXT;
//
// typedef struct _CONTEXT CONTEXT;
//
// typedef struct _FLOATING_SAVE_AREA _FLOATING_SAVE_AREA, *P_FLOATING_SAVE_AREA;
//
// typedef struct _FLOATING_SAVE_AREA FLOATING_SAVE_AREA;





typedef char CHAR;

typedef CHAR * LPCSTR;

typedef CHAR * LPSTR;

typedef CHAR * PCHAR;

typedef void * PVOID;

typedef uint UINT32;

typedef uchar UINT8;

typedef ulong ULONG_PTR;

typedef short int16_t;

typedef ulong ULONG;

typedef uchar UCHAR;

typedef WORD ATOM;



typedef void * LPCVOID;
