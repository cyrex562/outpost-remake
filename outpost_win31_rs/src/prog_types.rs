//
// Created by cyrex on 2022-05-22.
//

use std::ffi::c_void;
use std::os::raw::c_char;

pub type HANDLE16 = u16;
pub type HGLOBAL16 = HANDLE16;
pub type HTASK16 = HANDLE16;
pub type HINSTANCE16 = HANDLE16;
pub type HWND16 = HANDLE16;
pub type HMENU16 = HANDLE16;

// typedef u32 SEGPTR;
pub type SEGPTR = u32;

// typedef let mut HANDLE16: u16;
// typedef HANDLE16 HCURSOR16;
pub type HCURSOR16 = HANDLE16;
// typedef HANDLE16 HBRUSH16;
pub type HBRUSH16 = HANDLE16;
// typedef HANDLE16 HMENU16;
// typedef let mut WPARAM16: u16;
pub type WPARAM16 = HANDLE16;
// typedef HANDLE16 HWND16;
// typedef HANDLE16 HMODULE16;
pub type HMODULE16 = HANDLE16;
// typedef HANDLE16 HACCEL16;
pub type HACCEL16 = HANDLE16;
// typedef HANDLE16 HDC16;
pub type HDC16 = HANDLE16;
// typedef HANDLE16 HFILE16;
pub type HFILE16 = HANDLE16;
// typedef HANDLE16 HGLOBAL16;
// typedef HANDLE16 HPEN16;
pub type HPEN16 = HANDLE16;
// typedef HANDLE16 HGDIOBJ16;
pub type HGDIOBJ16 = HANDLE16;
// typedef let mut LONG_PTR: i32;
pub type LONG_PTR = i32;
// typedef LONG_PTR LPARAM;
pub type LPARAM = LONG_PTR;
pub type OpLparam = *mut c_void;

// typedef let mut BOOL16: u16;


// typedef char * LPCSTR;
// typedef let mut BYTE: u8;
// typedef i16 WORD;
// typedef i16 INT16;
// typedef let mut u16: u16;
// typedef HANDLE16 HICON16;
pub type HICON16 = HANDLE16;

pub struct LOGPALETTE {
    // WORD pal_version;
    pub pal_version: u16,
    // WORD pal_num_entries;
    pub pal_num_entries: u16,
} //LOGPALETTE;

pub struct PALETTEENTRY {
    // BYTE pe_red;
    pub pe_red: u8,
    // BYTE pe_green;
    pub pre_green: u8,
    // BYTE pe_blue;
    pub pe_blue: u8,
    // BYTE pe_flags;
    pub pe_flags: u8,
 } // PALETTEENTRY;

pub struct RECT16 {
    // INT16 x;
    pub x: i16,
    // INT16 y;
    pub y: i16,
} //RECT16;

pub struct POINT16 {
    // INT16 x;
    pub x: i16,
    // INT16 y;
    pub y: i16,
 } // POINT16;

pub struct WINDOWPLACEMENT16 {
    // let mut length: u16;
    pub length: u16,
    // let mut flags: u16;
    pub flags: u16,
    // let mut show_cmd: u16;
    pub show_cmd: u16,
    // struct POINT16 pt_min_position;
    pub pt_min_position: POINT16,
    // struct POINT16 pt_max_position;
    pub pt_max_position: POINT16,
    // struct let mut rc_normal_position: RECT16;
    pub rc_normal_position: RECT16,
 } //WINDOWPLACEMENT16;

pub struct WNDCLASS16 {
    // let mut style: u16;
    pub sytle: u16,
    // pub fn * lpfn_wnd_proc;
    pub lpfn_wnd_proc: fn(u16, u16, LPARAM, WPARAM16, u16, HWND16) -> LRESULT,
    // INT16 cb_cls_extra;
    pub cb_cls_extra: i16,
    // INT16 cb_wnd_extra;
    pub cb_wnd_extra: i16,
    // HANDLE16 h_instance;
    pub h_instance: HANDLE16,
    // HICON16 h_icon;
    pub h_icon: HICON16,
    // HCURSOR16 h_cursor;
    pub h_cursor: HCURSOR16,
    // HBRUSH16 hbr_background;
    pub hbr_background: HBRUSH16,
    // let mut lpsz_menu_name: u32;
    pub lpsz_menu_name: *mut c_char,
    // let mut lpsz_class_name: u32;
    pub lpsz_class_name: *mut c_char,
} // WNDCLASS16;

#[allow(non_snake_case)]
pub struct tagRGBQUAD {
    // BYTE rgbBlue;
    pub rgbBlue: u8,
    // BYTE rgbGreen;
    pub rgbGreen: u8,
    // BYTE rgbRed;
    pub rgbRed: u8,
    // BYTE rgbReserved;
    pub rgbReserved: u8,
}

// typedef let mut LONG: i32;
// typedef u32 DWORD;
// typedef HANDLE16 HINSTANCE16;


#[allow(non_snake_case)]
pub struct tagBITMAPINFOHEADER {
    // let mut biSize: u32;
    pub biSize: u32,
    // LONG biWidth;
    pub biWidth: i32,
    // LONG biHeight;
    pub biHeight: i32,
    // WORD biPlanes;
    pub biPlanes: u8,
    // WORD biBitCount;
    pub biBitCount: u8,
    // let mut biCompression: u32;
    pub biCompression: u32,
    // let mut biSizeImage: u32;
    pub biSizeImage: u32,
    // LONG biXPelsPerMeter;
    pub biXPelsPerMeter: i32,
    // LONG biYPelsPerMeter;
    pub biYPelsPerMeter: i32,
    // let mut biClrUsed: u32;
    pub biClrUsed: u32,
    // let mut biClrImportant: u32;
    pub biClrImportant: u32,
} //BITMAPINFO;

pub struct MSG16 {
    // HWND16 hwnd;
    pub hwnd: HWND16,
    // let mut message: u16;
    pub message: u16,
    // let mut wparam: WPARAM16;
    pub wparam: WPARAM16,
    // LPARAM lparam;
    pub lparam: LPARAM,
    // let mut time: u32;
    pub time: u32,
    // struct POINT16 pt;
    pub pt: POINT16,
} //MSG16;

// typedef void * LPVOID;

pub struct PAINTSTRUCT16 {
    // HDC16 hdc;
    pub hdc: HDC16,
    // let mut f_erase: bool;
    pub bool: f_erase,
    // struct let mut rc_paint: RECT16;
    pub rc_paint: RECT16,
    // let mut f_restore: bool;
    pub f_restore: bool,
    // let mut f_inc_update: bool;
    pub f_inc_update: bool,
    // BYTE rgb_reserved[16];
    pub rgb_reserved: [u8;16],
 } //PAINTSTRUCT16;



// typedef HANDLE16 HTASK16;

// typedef HANDLE16 HPALETTE16;

// typedef HANDLE16 HRSRC16;

pub struct tagPOINT {
    // LONG x;
    pub x: i32,
    // LONG y;
    pub y: i32,
}

#[allow(non_snake_case)]
pub struct tagMSG {
    // HWND16 hwnd;
    pub hwnd: HWND16,
    // let mut message: u16;
    pub message: u16,
    // let mut wParam: WPARAM16;
    pub wParam: WPARAM16,
    // LPARAM lParam;
    pub lParam: LPARAM,
    // let mut time: u32;
    pub time: u32,
    // struct POINT16 pt;
    pub pt: POINT16
}

pub struct POINTL {
    // LONG x;
    pub x: i32,
    // LONG y;
    pub y: i32,
} // POINTL;



#[allow(non_snake_case)]
pub struct _struct_657 {
    // POINTL dmPosition;
    pub dmPosition: POINTL,
    // let mut dmDisplayOrientation: u32;
    pub dmDisplayOrientation: u32,
    // let mut dmDisplayFixedOutput: u32;
    pub dmDisplayFixedOutput: u32,
}

#[allow(non_snake_case)]
pub struct _struct_656 {
    // short dmOrientation;
    pub dmOrientation: i16,
    // short dmPaperSize;
    pub dmPaperSize: i16,
    // short dmPaperLength;
    pub dmPaperLength: i16,
    // short dmPaperWidth;
    pub dmPaperWidth: i16,
    // short dmScale;
    pub dmScale: i16,
    // short dmCopies;
    pub dmCopies: i16,
    // short dmDefaultSource;
    pub dmDefaultSource: i16,
    // short dmPrintQuality;
    pub dmPrintQuality: i16,
}


pub union _union_655 {
    // struct _struct_656 field0;
    pub field0: _struct_656,
    // struct _struct_657 field1;
    pub field1: _struct_657,
}


#[allow(non_snake_case)]
pub union _union_658 {
    dmDisplayFlags: u32,
    dmNup: u32,
}

#[allow(non_snake_case)]
pub struct DEVMODEA {
    // BYTE dmDeviceName[32];
    pub dmDeviceName: [u8;32],
    // WORD dmSpecVersion;
    pub dmSpecVersion: u16,
    // WORD dmDriverVersion;
    pub dmDriverVersion: u16,
    // WORD dmSize;
    pub dmSize: u16,
    // WORD dmDriverExtra;
    pub dmDriverExtra: u16,
    // let mut dmFields: u32;
    pub dmFields: u32,
    // union _union_655 field6_0x2c;
    pub field6_0x2c: _union_655,
    // short dmColor;
    pub dmColor: i16,
    // short dmDuplex;
    pub dmDuplex: i16,
    // short dmYResolution;
    pub dmYResolution: i16,
    // short dmTTOption;
    pub dmTTOption: i16,
    // short dmCollate;
    pub dmCollate: i16,
    // BYTE dmFormName[32];
    pub dmFormName: [u8;32],
    // WORD dmLogPixels;
    pub dmLogPixels: i16,
    // let mut dmBitsPerPel: u32;
    pub dmBitsPerPel: u32,
    // let mut dmPelsWidth: u32;
    pub dmPelsWidth: u32,
    // let mut dmPelsHeight: us32;
    pub dmPelsHeight: u32,
    // union _union_658 field17_0x74;
    pub field17_0x74: _union_658,
    // let mut dmDisplayFrequency: u32;
    pub dmDipslayFrequency: u32,
    // let mut dmICMMethod: u32;
    pub dmICMMethod: u32,
    // let mut dmICMIntent: u32;
    pub dmICMIntent: u32,
    // let mut dmMediaType: u32;
    pub dmMediaType: u32,
    // let mut dmDitherType: u32;
    pub dmDitherType: u32,
    // let mut dmReserved1: u32;
    pub dmReserved1: u32,
    // let mut dmReserved2: u32;
    pub dmReserved2: u32,
    // let mut dmPanningWidth: u32;
    pub dmPanningWidth: u32,
    // let mut dmPanningHeight: u32;
    pub dmPanningHeight: u32,
} //DEVMODEA;


#[allow(non_snake_case)]
pub struct FLOATING_SAVE_AREA {
    // let mut ControlWord: u32;
    pub ControlWord: u32,
    // let mut StatusWord: u32;
    pub StatusWord: u32,
    // let mut TagWord: u32;
    pub TagWord: u32,
    // let mut ErrorOffset: u32;
    pub ErrorOffset: u32,
    // let mut ErrorSelector: u32;
    pub ErrorSelector: u32,
    // let mut DataOffset: u32;
    pub DataOffset: u32,
    // let mut DataSelector: u32;
    pub DataSelector: u32,
    // BYTE RegisterArea[80];
    pub RegisterArea: [u8;80],
    // let mut Cr0NpxState: u32;
    pub Cr0NpxState: u32,
 } // FLOATING_SAVE_AREA;

// typedef struct _CONTEXT {
//     let mut ContextFlags: u32;
//     let mut Dr0: u32;
//     let mut Dr1: u32;
//     let mut Dr2: u32;
//     let mut Dr3: u32;
//     let mut Dr6: u32;
//     let mut Dr7: u32;
//     FLOATING_SAVE_AREA FloatSave;
//     let mut SegGs: u32;
//     let mut SegFs: u32;
//     let mut SegEs: u32;
//     let mut SegDs: u32;
//     let mut Edi: u32;
//     let mut Esi: u32;
//     let mut Ebx: u32;
//     let mut Edx: u32;
//     let mut Ecx: u32;
//     let mut Eax: u32;
//     let mut Ebp: u32;
//     let mut Eip: u32;
//     let mut SegCs: u32;
//     let mut EFlags: u32;
//     let mut Esp: u32;
//     let mut SegSs: u32;
//     BYTE ExtendedRegisters[512];
// } CONTEXT;


pub struct tagRECT {
    // LONG left;
    pub left: i32,
    // LONG top;
    pub top: i32,
    // LONG right;
    pub right: i32,
    // LONG bottom;
    pub bottom: i32,
} //RECT;

// typedef WORD ATOM;
pub type ATOM = u16;

// typedef void * PVOID;

// typedef LONG_PTR LRESULT;
pub type LRESULT = *mut i32;

// typedef let mut COLORREF: u32;
pub type COLORREF = u32;

// #define NULL 0

// #endif //OUTPOST_C_PROJ__TYPES_H_
