use std::os::raw::c_char;
use std::ffi::c_void;
use crate::prog_types::{_union_655, _union_658, HANDLE16};

pub type HRSRC16 = HANDLE16;

pub type HANDLE16 = u16;
pub type HGLOBAL16 = HANDLE16;
pub type HTASK16 = HANDLE16;
pub type HINSTANCE16 = HANDLE16;
pub type HWND16 = HANDLE16;
pub type HMENU16 = HANDLE16;

// typedef SEGPTR: u32;
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


// typedef char * LPCSTR;
// typedef let mut BYTE: u8;
// typedef i16 WORD;
// typedef i16 INT16;
// typedef let mut u16: u16;
// typedef HANDLE16 HICON16;
pub type HICON16 = HANDLE16;

pub struct LOGPALETTE {
    // pal_version: u16;
    pub pal_version: u16,
    // pal_num_entries: u16;
    pub pal_num_entries: u16,
}

pub struct PALETTEENTRY {
    // BYTE pe_red;
    pub pe_red: u8,
    // BYTE pe_green;
    pub pre_green: u8,
    // BYTE pe_blue;
    pub pe_blue: u8,
    // BYTE pe_flags;
    pub pe_flags: u8,
 }

pub struct RECT16 {
    // x: INT16;
    pub x: i16,
    // y: INT16;
    pub y: i16,
}

pub struct POINT16 {
    // x: INT16;
    pub x: i16,
    // y: INT16;
    pub y: i16,
 }

pub struct WINDOWPLACEMENT16 {
    // let mut length: u16;
    pub length: u16,
    // let mut flags: u16;
    pub flags: u16,
    // let mut show_cmd: u16;
    pub show_cmd: u16,
    // struct POpt_min_position: INT16;
    pub pt_min_position: POINT16,
    // struct POpt_max_position: INT16;
    pub pt_max_position: POINT16,
    // struct let mut rc_normal_position: RECT16;
    pub rc_normal_position: RECT16,
 }

pub struct WNDCLASS16 {
    // let mut style: u16;
    pub sytle: u16,
    // pub unsafe fn * lpfn_wnd_proc;
    pub lpfn_wnd_proc: fn(u16, u16, LPARAM, WPARAM16, u16, HWND16) -> LRESULT,
    // cb_cls_extra: INT16;
    pub cb_cls_extra: i16,
    // cb_wnd_extra: INT16;
    pub cb_wnd_extra: i16,
    // HANDLE16 h_instance;
    pub h_instance: HANDLE16,
    // let mut h_icon: HICON16;
    pub h_icon: HICON16,
    // let mut h_cursor: HCURSOR16;
    pub h_cursor: HCURSOR16,
    // let mut hbr_background: HBRUSH16;
    pub hbr_background: HBRUSH16,
    // let mut lpsz_menu_name: u32;
    pub lpsz_menu_name: *mut c_char,
    // let mut lpsz_class_name: u32;
    pub lpsz_class_name: *mut c_char,
}

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


#[allow(non_snake_case)]
pub struct tagBITMAPINFOHEADER {
    // let mut biSize: u32;
    pub biSize: u32,
    // LONG biWidth;
    pub biWidth: i32,
    // LONG biHeight;
    pub biHeight: i32,
    // biPlanes: u16;
    pub biPlanes: u8,
    // biBitCount: u16;
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
}

pub struct MSG16 {
    // hwnd: HWND16;
    pub hwnd: HWND16,
    // let mut message: u16;
    pub message: u16,
    // let mut wparam: WPARAM16;
    pub wparam: WPARAM16,
    // lparam: LPARAM;
    pub lparam: LPARAM,
    // let mut time: u32;
    pub time: u32,
    // struct POpt: INT16;
    pub pt: POINT16,
}

pub struct PAINTSTRUCT16 {
    // let mut hdc: HDC16;
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
 }

pub struct tagPOINT {
    // LONG x;
    pub x: i32,
    // LONG y;
    pub y: i32,
}

#[allow(non_snake_case)]
pub struct tagMSG {
    // hwnd: HWND16;
    pub hwnd: HWND16,
    // let mut message: u16;
    pub message: u16,
    // let mut wParam: WPARAM16;
    pub wParam: WPARAM16,
    // lParam: LPARAM;
    pub lParam: LPARAM,
    // let mut time: u32;
    pub time: u32,
    // struct POpt: INT16;
    pub pt: POINT16
}

pub struct POINTL {
    // LONG x;
    pub x: i32,
    // LONG y;
    pub y: i32,
}

#[allow(non_snake_case)]
pub struct DEVMODEA {
    // BYTE dmDeviceName[32];
    pub dmDeviceName: [u8;32],
    // dmSpecVersion: u16;
    pub dmSpecVersion: u16,
    // dmDriverVersion: u16;
    pub dmDriverVersion: u16,
    // dmSize: u16;
    pub dmSize: u16,
    // dmDriverExtra: u16;
    pub dmDriverExtra: u16,
    // let mut dmFields: u32;
    pub dmFields: u32,
    // union _union_655 field6_0x2c;
    pub field6_0x2c: _union_655,
    // let mut dmColor: i16;
    pub dmColor: i16,
    // let mut dmDuplex: i16;
    pub dmDuplex: i16,
    // let mut dmYResolution: i16;
    pub dmYResolution: i16,
    // let mut dmTTOption: i16;
    pub dmTTOption: i16,
    // let mut dmCollate: i16;
    pub dmCollate: i16,
    // BYTE dmFormName[32];
    pub dmFormName: [u8;32],
    // dmLogPixels: u16;
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
}


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
 }


pub struct tagRECT {
    // LONG left;
    pub left: i32,
    // LONG top;
    pub top: i32,
    // LONG right;
    pub right: i32,
    // LONG bottom;
    pub bottom: i32,
}

// typedef ATOM: u16;
pub type ATOM = u16;

// typedef LONG_PTR LRESULT;
pub type LRESULT = *mut i32;

// typedef let mut COLORREF: u32;
pub type COLORREF = u32;
