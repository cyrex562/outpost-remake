//
#![allow(non_snake_case)]
// Created by cyrex on 2022-05-22.
//
// //
// // #ifndef OUTPOST_C_PROJ__SYS_API_H_
// // #define OUTPOST_C_PROJ__SYS_API_H_
//
// #include "types.h"

//
// #endif //OUTPOST_C_PROJ__SYS_API_H_





use std::ffi::c_void;
use std::os::raw::c_char;
use crate::block_1000::block_1000_1000::{msg_box_op_1000_1f24, pass1_1000_1f68};
use crate::block_1000::block_1000_2000::mem_op_1000_21b6;
use crate::prog_types::{ATOM, FLOATING_SAVE_AREA, HGDIOBJ16, HGLOBAL16, HINSTANCE16, HMENU16, HTASK16, HWND16, LPARAM, LRESULT, RECT16, WNDCLASS16, WPARAM16, SEGPTR};

pub struct CONTEXT
{
    // DWORD   ContextFlags;  /* 000 */
    pub ContextFlags: u32,
    /* These are selected by CONTEXT_DEBUG_REGISTERS */
    // DWORD   Dr0;           /* 004 */
    pub Dr0: u32,
    // DWORD   Dr1;           /* 008 */
    pub Dr1: u32,
    // DWORD   Dr2;           /* 00c */
    pub Dr2: u32,
    // DWORD   Dr3;           /* 010 */
    pub Dr3: u32,
    // DWORD   Dr6;           /* 014 */
    pub Dr6: u32,
    // DWORD   Dr7;           /* 018 */
    pub Dr7: u32,
    /* These are selected by CONTEXT_FLOATING_POINT */
    // FLOATING_SAVE_AREA FloatSave; /* 01c */
    pub FloatSave: FLOATING_SAVE_AREA,
    /* These are selected by CONTEXT_SEGMENTS */
    // DWORD   SegGs;         /* 08c */
    pub SegGs: u32,
    // DWORD   SegFs;         /* 090 */
    pub SegFs: u32,
    // DWORD   SegEs;         /* 094 */
    pub SegEs: u32,
    // DWORD   SegDs;         /* 098 */
    pub SegDs: u32,
    /* These are selected by CONTEXT_INTEGER */
    // DWORD   Edi;           /* 09c */
    pub Edi: u32,
    // DWORD   Esi;           /* 0a0 */
    pub Esi: u32,
    // DWORD   Ebx;           /* 0a4 */
    pub Ebx: u32,
    // DWORD   Edx;           /* 0a8 */
    pub Edx: u32,
    // DWORD   Ecx;           /* 0ac */
    pub Ecx: u32,
    // DWORD   Eax;           /* 0b0 */
    pub Eax: u32,
    /* These are selected by CONTEXT_CONTROL */
    // DWORD   Ebp;           /* 0b4 */
    pub Ebp: u32,
    // DWORD   Eip;           /* 0b8 */
    pub Eip: u32,
    // DWORD   SegCs;         /* 0bc */
    pub SegCs: u32,
    // DWORD   EFlags;        /* 0c0 */
    pub EFlags: u32,
    // DWORD   Esp;           /* 0c4 */
    pub Esp: u32,
    // DWORD   SegSs;         /* 0c8 */
    pub SegSs: u32,

    // BYTE    ExtendedRegisters[MAXIMUM_SUPPORTED_EXTENSION];  /* 0xcc */
    pub ExtendedRegisters: [u8;0xff]
} // CONTEXT;

pub const CONTEXT_X86:u32 =       0x00010000;
pub const CONTEXT_i386: u32 =     CONTEXT_X86;
pub const CONTEXT_i486: u32 =      CONTEXT_X86;

pub const CONTEXT_CONTROL: u32 =  (CONTEXT_i386 | 0x0001); /* SS:SP, CS:IP, FLAGS, BP */
pub const CONTEXT_INTEGER: u32 =   (CONTEXT_i386 | 0x0002); /* AX, BX, CX, DX, SI, DI */
pub const CONTEXT_SEGMENTS: u32 =  (CONTEXT_i386 | 0x0004); /* DS, ES, FS, GS */
pub const CONTEXT_FLOATING_POINT: u32 =  (CONTEXT_i386 | 0x0008); /* 387 state */
pub const CONTEXT_DEBUG_REGISTERS: u32 = (CONTEXT_i386 | 0x0010); /* DB 0-3,6,7 */
pub const CONTEXT_EXTENDED_REGISTERS: u32 = (CONTEXT_i386 | 0x0020);
pub const CONTEXT_FULL: u32 = (CONTEXT_CONTROL | CONTEXT_INTEGER | CONTEXT_SEGMENTS);
pub const CONTEXT_ALL: u32 = (CONTEXT_CONTROL | CONTEXT_INTEGER | CONTEXT_SEGMENTS | CONTEXT_FLOATING_POINT | CONTEXT_DEBUG_REGISTERS | CONTEXT_EXTENDED_REGISTERS);


// HGLOBAL16 WINAPI LockSegment16	(	HGLOBAL16 		)
pub fn LockSegment16(a: HGLOBAL16) -> HGLOBAL16 {
    todo!()
}

pub fn WaitEvent16(a: HTASK16) -> bool {
    todo!()
}

// void        WINAPI InitTask16(CONTEXT*);
pub fn InitTask16(ctx: *mut CONTEXT) {
    todo!()
}

// DWINAPI: u16 GetVersion16()
pub fn GetVersion16() -> u32 {
    todo!()
}

// INT16       WINAPI InitApp16;
pub fn InitApp16(hinst: HINSTANCE16) -> i16 {
    todo!()
}

// void        WINAPI FatalAppExit16(UINT16,LPCSTR);
pub fn FatalAppExit16(action: u16, msg: *mut c_char) {
    todo!()
}

pub fn FatalExit() {
    todo!()
}

// DGetVersion16: u16();
// HGLOBAL16 GLobalAlloc16(Dsize: u16, Uflags: INT16);
pub fn GLobalAlloc16(size: u32, flags: u16) -> HGLOBAL16
{
    todo!()
}

// HGLOBAL16 GlobalReAlloc16(Uflags: INT16, Dsize: u16, HGLOBAL16 handle);
// HGLOBAL16 GlobalFree16(HGLOBAL16 handle);
// pub fn * WIN16_GlobalLock16(HGLOBAL16 handle);
// BOOL16 GlobalUnlock16(HGLOBAL16 handle);
// DGlobalSize16: u16(HGLOBAL16 handle);
// DGlobalHandle16: u16(sel: u16);
// HGLOBAL16 LockSegment16(HGLOBAL16 handle);
// BOOL16 WaitEvent16(HTASK16 h_task);
// i16 GetModuleFileName16(i16 n_size, char * lp_file_name, HINSTANCE16 h_module);
// pub fn * MakeProcInstance16(HANDLE16 h_instance, void * func);
// pub fn FreeProcInstance16(void * func);
// HRSRC16 FindResource16(char * type, char * name, HMODULE16 h_module);
// HGLOBAL16 LoadResource16(HRSRC16 h_rsrc, HMODULE16 h_module);
// SEGPTR WIN16_LockResource16(HGLOBAL16 handle);
// BOOL16 FreeResource16(HGLOBAL16 handle);
// _lclose16: HFILE16(h_file: HFILE16);
// _lcreat16: HFILE16(i16 attr, char * path);
// _llseek16: i32(i16 n_origin, l_offset: i32, h_file: HFILE16);
// _lopen16: HFILE16(i16 mode, char * path);
// i16 lstrlen16(char * in_string);
// pub fn InitTask16(CONTEXT * context);
// pub fn DOS3Call(CONTEXT * context);
// u16 SetErrorMode16(u16 mode);
// pub fn __AHSHIFT();
// pub fn __AHINCR();
// pub fn OutputDebugString16(char * str);
pub fn OutputDebugString16(in_str: *const c_char) {
    todo!()
}
// i16 GetPrivateProfileString16(char * filename, len: u16, char * buffer, char * def_val, char * entry, char * section);
// BOOL16 WritePrivateProfileString16(char * filename, char * string, char * entry, char * section);
// pub fn * GetDOSEnvironment16();
pub fn GetDOSEnvironment16() -> SEGPTR {
    todo!()
}

// pub fn FatalAppExit16(char * str, u16 action);
// HINSTANCE16 WinExec16(n_cmd_show: u16, char * lp_cmd_line);
// pub fn __WINFLAGS();
// pub fn GlobalDOSAlloc16(size: u32) -> u32;
// u16 GlobalDOSFree16(u16 sel);
// u16 GlobalPageLock16(HGLOBAL16 handle);
// u16 GlobalPageUnlock16(HGLOBAL16 handle);
// pub fn hmemcpy16(count: i32, void * src, void * dst);
// WIN16_hread: i32(count: i32, void * buffer, h_file: HFILE16);
// pub fn _hwrite16(count: u32, u8 * buffer, h_file: HFILE16) -> u32;
// COLORREF SetBkColor16(COLORREF color, hdc: HDC16);
// i16 SetMapMode16(i16 mode, hdc: HDC16);
// u8 SetTextColor16(COLORREF color, hdc: HDC16);
// BOOL16 LineTo16(y: INT16, x: INT16, hdc: HDC16);
// DMoveTo16: u16(y: INT16, x: INT16, hdc: HDC16);
// BOOL16 Ellipse16(bottom: INT16, right: INT16, top: INT16, left: INT16, hdc: HDC16);
// BOOL16 Rectangle16(bottom: INT16, right: INT16, top: INT16, left: INT16, hdc: HDC16);
// BOOL16 TextOut16(i16 count, char * str, i16 y, i16 x, hdc: HDC16);
// BOOL16 Polygon16(i16 count, POINT16 * pt, hdc: HDC16);
// HGDIOBJ16 SelectObject16(HGDIOBJ16 handle, hdc: HDC16);
// CreateDC16: HDC16(DEVMODEA * init_data, char * output, char * device, char * driver);
// CreatePen16: HPEN16(COLORREF color, width: INT16, style: INT16);
// HBRUSH16 CreateSolidBrush16(COLORREF color);
// BOOL16 DeleteDC16(hdc: HDC16);
// BOOL16 DeleteObject16(HGDIOBJ16 obj);
// DGetCurrentPosition16: u16(hdc: HDC16);
// GetDeviceCaps16: INT16(cap: INT16, hdc: HDC16);
// HGDIOBJ16 GetStockObject16(obj: INT16);
pub fn GetStockObject16(obj: i16) -> HGDIOBJ16 {
    todo!()
}
// DGetTextExtent16: u16(count: INT16, LPCSTR str, hdc: HDC16);
// BOOL16 UnrealizeObject16(HGDIOBJ16 obj);
// CreatePalette16: HPALETTE16(LOGPALETTE * palette);
// UGetSystemPaletteEntries: INT16(PALETTEENTRY * entries, Ucount: INT16, Ustart: INT16, hdc: HDC16);
// StretchDIBits16: INT16(Ddw_rop: u16, Uw_usage: INT16, BITMAPINFO * info, bits: mut PVOID, height_src: INT16, width_src: INT16, y_src: INT16, x_src: INT16, height_dst: INT16, width_dst: INT16, y_dst: INT16, x_dst: INT16, hdc: HDC16);
// SetDIBitsToDevice: INT16(Ucoloruse: INT16, BITMAPINFO * info, void * bits, Ulines: INT16, Ustartscan: INT16, y_src: INT16, x_src: INT16, cy: INT16, cx: INT16, y_dest: INT16, x_dest: INT16, hdc: HDC16);
// BOOL16 MoveToEx16(POINT16 * pt, y: INT16, x: INT16, hdc: HDC16);
// MessageBox16: INT16(Utype: INT16, char * title, char * text, hwnd: HWND16);
// InitApp16: INT16(HINSTANCE16 h_instance);
// pub fn PostQuitMessage16(exit_code: INT16);
// USetTimer16: INT16(void * proc, Utimeout: INT16, Uid: INT16, hwnd: HWND16);
// BOOL16 KillTimer16(Uid: INT16, hwnd: HWND16);
// BOOL16 GetCursorPos16(POINT16 * pt);
// SetCapture16: HWND16(hwnd: HWND16);
// BOOL16 ReleaseCapture16();
// SetFocus16: HWND16(hwnd: HWND16);
pub fn SetFocus16(hwnd: HWND16) -> HWND16 {
    todo!()
}
// HANDLE16 RemoveProp16(LPCSTR str, hwnd: HWND16);
// HANDLE16 GetProp16(LPCSTR in_string, hwnd: HWND16);
// BOOL16 SetProp16(HANDLE16 handle, char * str, hwnd: HWND16);
// pub fn ClientToScreen16(POINT16 * lppnt, hwnd: HWND16);
// pub fn ScreenToClient16(POINT16 * lppnt, hwnd: HWND16);
// BOOL16 IsIconic16(hwnd: HWND16);
// pub fn GetWindowRect16(RECT16 * rect, hwnd: HWND16);
// pub fn GetClientRect16(RECT16 * rect, hwnd: HWND16);
pub fn GetClientRect16(rect: *mut RECT16, hwnd: HWND16) {
    todo!()
}
// BOOL16 EnableWindow16(BOOL16 enable, hwnd: HWND16);
// BOOL16 IsWindowEnabled16(hwnd: HWND16);
// GetWindowText16: INT16(n_max_count: INT16, lp_string: u32, hwnd: HWND16);
// BOOL16 SetWindowText16(lp_string: u32, hwnd: HWND16);
// BeginPaint16: HDC16(PAINTSTRUCT16 * lps, hwnd: HWND16);
// BOOL16 EndPaint16(PAINTSTRUCT16 * lps, hwnd: HWND16);
// CreateWindow16: HWND16(Dstyle: u16, void * data, HINSTANCE16 instance, HMENU16 hmenu, parent: HWND16, height: INT16, width: INT16, y: INT16, x: INT16, char * window_name, char * class_name);
pub fn CreateWindow16(style: u32, data: *mut c_void, instance: HINSTANCE16, hmenu: HMENU16, parent: HWND16, height: i16, width: i16, y: i16, x: i16, window_name: *const c_char, class_name: *const c_char) -> HWND16 {
    todo!()
}

// BOOL16 ShowWindow16(cmd: INT16, hwnd: HWND16);
pub fn ShowWindow16(cmd: i16, hwnd: HWND16) -> bool {
    todo!()
}
// BOOL16 BringWindowToTop16(hwnd: HWND16);
// BOOL16 IsWindow16(hwnd: HWND16);
// BOOL16 DestroyWindow16(hwnd: HWND16);
pub fn DestroyWindow16(hwnd: HWND16) -> bool {
todo!()
}
// BOOL16 EnumChildWindows1(lparam: LPARAM, void * func, parent: HWND16);
// BOOL16 MoveWindow16(BOOL16 repaint, cy: INT16, cx: INT16, y: INT16, x: INT16, hwnd: HWND16);
// RegisterClass16: ATOM(WNDCLASS16 * wc);
pub fn RegisterClass16(wc: *mut WNDCLASS16) -> ATOM {
    todo!()
}
// GetDC16: HDC16(hwnd: HWND16);
// GetWindowDC16: HDC16(hwnd: HWND16);
// ReleaseDC16: INT16(hdc: HDC16, hwnd: HWND16);
// HCURSOR16 SetCursor16(HCURSOR16 hcursor);
// ShowCursor16: INT16(BOOL16 b_show);
// BOOL16 PtInRect16(POpt: INT16, RECT16 * rect);
// i16 FillRect16(HBRUSH16 hbrush, RECT16 * rect, hdc: HDC16);
// FrameRect16: INT16(HBRUSH16 hbrush, RECT16 * rect, hdc: HDC16);
// BOOL16 DrawIcon16(HICON16 h_icon, y: INT16, x: INT16, hdc: HDC16);
// DrawText16: INT16(Uflags: INT16, RECT16 * rect, count: INT16, LPCSTR in_string, hdc: HDC16);
// CreateDialog16: HWND16(void * dlg_proc, owner: HWND16, char * dlg_template, HINSTANCE16 hinst);
// BOOL16 IsDialogMessage16(MSG16 * msg16, hwnd_dlg: HWND16);
// GetDlgItem16: HWND16(id: INT16, hwnd_dlg: HWND16);
pub fn GetDlgItem16(id: i16, hwnd_dlg: HWND16) -> HWND16 {
    todo!();
}
// pub fn SetDlgItemText16(lp_string: u32, id: INT16, hwnd: HWND16);
// pub fn SetDlgItemInt16(BOOL16 f_signed, Uvalue: INT16, id: INT16, hwnd: HWND16);
// UGetDlgItemInt16: INT16(BOOL16 f_signed, BOOL16 * translated, id: INT16, hwnd: HWND16);
// BOOL16 CheckRadioButton16(Ucheck_id: INT16, Ulast_id: INT16, Ufirst_id: INT16, hwnd_dlg: HWND16);
// BOOL16 CheckDlgButton16(Ucheck: INT16, id: INT16, hwnd: HWND16);
// UIsDlgButtonChecked: INT16(Uid: INT16, hwnd: HWND16);
// LRESULT SendDlgItemMessage16(l_param: LPARAM, w_param: WPARAM16, Umsg: INT16, id: INT16, hwnd: HWND16);
// pub fn MapDialogRect16(RECT16 * rect, hwnd: HWND16);
// pub fn MessageBeep16(Ui: INT16);
// LRESULT DefWindowProc16(lparam: LPARAM, wparam: WPARAM16, Umsg: INT16, hwnd: HWND16);
// BOOL16 GetMessage16(Ulast: INT16, Ufirst: INT16, hwnd: HWND16, MSG16 * msg);
// BOOL16 PostMessage16(lparam: LPARAM, wparam: WPARAM16, Umsg: INT16, hwnd: HWND16);
// LRESULT SendMessage16(lparam: LPARAM, wparam: WPARAM16, Umsg: INT16, hwnd: HWND16);
pub fn SendMessage16(lparam: LPARAM, wparam: WPARAM16, msg: u16, hwnd: HWND16) -> LRESULT {
    todo!()
}
// BOOL16 TranslateMessage16(MSG16 * msg);
// long DispatchMessage16(MSG16 * msg);
// LRESULT CallWindowProc16(lparam: LPARAM, wparam: WPARAM16, Umsg: INT16, hwnd: HWND16, LPVOID func);
// pub fn UpdateWindow16(hwnd: HWND16);
// pub fn InvalidateRect16(BOOL16 erase, RECT16 * rect, hwnd: HWND16);
// pub fn ValidateRect16(RECT16 * rect, hwnd: HWND16);
// GetWindowWord16: u16(offset: INT16, hwnd: HWND16);
// SetWindowWord16: u16(newval: u16, offset: INT16, hwnd: HWND16);
// long GetWindowLong16(offset: INT16, hwnd: HWND16);
// long SetWindowLong16(long newval, i16 offset, hwnd: HWND16);
// HMENU16 LoadMenu16(char * name, HINSTANCE16 instance);
// BOOL16 DestroyMenu16(HMENU16 menu);
// BOOL16 CheckMenuItem16(Uw_flags: INT16, Uw_item_id: INT16, HMENU16 hmenu);
// BOOL16 EnableMenuItem16(Uw_flags: INT16, Uw_item_id: INT16, HMENU16 hmenu);
// HMENU16 GetSubMenu16(n_pos: INT16, HMENU16 h_menu);
// BOOL16 WinHelp16(Ddw_data: u16, Uw_command: INT16, char * lp_help_file, hwnd: HWND16);
// HCURSOR16 LoadCursor16(char * name, HINSTANCE16 h_instance);
// HICON16 LoadIcon16(char * name, HINSTANCE16 h_instance);
// LoadString16: INT16(buf_len: INT16, char * buffer, Uresource_id: INT16, HINSTANCE16 instance);
// HACCEL16 LoadAccelerators16(char * lp_table_name, HINSTANCE16 instance);
// TranslateAccelerator16: INT16(MSG16 * msg, HACCEL16 haccel, hwnd: HWND16);
// GetSystemMetrics16: INT16(index: INT16);
// COLORREF GetSysColor16(index: INT16);
// pub fn SetSysColors16(COLORREF * values, INT16 * list, count: INT16);
// BOOL16 GrayString16(cy: INT16, cx: INT16, y: INT16, x: INT16, cch: INT16, lparam: LPARAM, void * gsprc, HBRUSH16 param_8, hdc: HDC16);
// SetSysModalWindow: HWND16(hwnd: HWND16);
// GetNextDlgTabItem16: HWND16(BOOL16 f_previous, hwnd_ctrl: HWND16, hwnd_dlg: HWND16);
// BOOL16 SetWindowPos16(flags: u16, cy: INT16, cx: INT16, y: INT16, x: INT16, hwnd_insert_after: HWND16, hwnd: HWND16);
// UGetMenuState16: INT16(Uw_flags: INT16, Uw_item_id: INT16, HMENU16 hmenu);
// GetDlgCtrlID16: INT16(hwnd: HWND16);
// SelectPalette16: HPALETTE16(BOOL16 b_force_background, hpal: HPALETTE16, hdc: HDC16);
// URealizePalette16: INT16(hdc: HDC16);
// BOOL16 GetWindowPlacement16(WINDOWPLACEMENT16 * wp16, hwnd: HWND16);
// BOOL16 SetWindowPlacement16(WINDOWPLACEMENT16 * wp16, hwnd: HWND16);
// BOOL16 GetClassInfo16(WNDCLASS16 * wc, char * name, HINSTANCE16 h_inst16);
pub fn GetClassInfo16(wc: *mut WNDCLASS16, name: *const c_char, h_inst16: HINSTANCE16) -> bool {
    todo!()
}
// BOOL16 InsertMenu16(data: u32, Uid: INT16, Uflags: INT16, Upos: INT16, HMENU16 hmenu);
// BOOL16 DeleteMenu16(Uwflags: INT16, Unpos: INT16, HMENU16 hmenu);
// BOOL16 ModifyMenu16(data: u32, Uid: INT16, Uflags: INT16, Upos: INT16, HMENU16 hmenu);
// BOOL16 TrackPopupMenu16(RECT16 * lp_rect, hwnd: HWND16, n_reserved: INT16, y: INT16, x: INT16, Uwflags: INT16, HMENU16 hmenu);
// wsprintf16: INT16(WORD * valist, char * spec, char * buffer, ...);
// wvsprintf16: INT16(WORD * args, char * spec, char * buffer);
// CreateWIndowEx16: HWND16(void * data, HINSTANCE16 instance, HMENU16 hmenu, parent: HWND16, height: INT16, width: INT16, y: INT16, x: INT16, Dstyle: u16, char * window_name, char * class_name, Dex_style: u16);
// BOOL16 DestroyIcon16(HICON16 h_icon);
// BOOL16 DestroyCursor16(HCURSOR16 h_cursor);
// DmciSendCommand16: u16(Dp2: u16, Ddw_param1: u16, Uw_msg: INT16, Uw_dev_id: INT16);
pub fn mciSendCommand16(p2: u32, dw_param1: u32, i16: uw_msg, i16: uw_dev_id) -> u32 {
    todo!()
}
// BOOL16 mciGetErrorString16(Uu_length: INT16, char * lp_str_buffer, Dw_error: u16);
// BOOL16 GetOpenFileName16(ofn: u32);
// BOOL16 GetSaveFileName16(ofn: u32);
// void* swi;
// pub fn SegmentLimit(a: u32) -> u32;
