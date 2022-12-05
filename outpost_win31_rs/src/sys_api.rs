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
use crate::prog_types::{ATOM, FLOATING_SAVE_AREA, HGDIOBJ16, HGLOBAL16, HINSTANCE16, HMENU16, HTASK16, HWND16, LPARAM, LRESULT, RECT16, WNDCLASS16, WPARAM16};

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

// DWORD WINAPI GetVersion16(void)
pub fn GetVersion16() -> u32 {
    todo!()
}

// INT16       WINAPI InitApp16(HINSTANCE16);
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

// DWORD GetVersion16(void);
// HGLOBAL16 GLobalAlloc16(DWORD size, UINT16 flags);
// HGLOBAL16 GlobalReAlloc16(UINT16 flags, DWORD size, HGLOBAL16 handle);
// HGLOBAL16 GlobalFree16(HGLOBAL16 handle);
// pub fn * WIN16_GlobalLock16(HGLOBAL16 handle);
// BOOL16 GlobalUnlock16(HGLOBAL16 handle);
// DWORD GlobalSize16(HGLOBAL16 handle);
// DWORD GlobalHandle16(WORD sel);
// HGLOBAL16 LockSegment16(HGLOBAL16 handle);
// BOOL16 WaitEvent16(HTASK16 h_task);
// i16 GetModuleFileName16(i16 n_size, char * lp_file_name, HINSTANCE16 h_module);
// pub fn * MakeProcInstance16(HANDLE16 h_instance, void * func);
// pub fn FreeProcInstance16(void * func);
// HRSRC16 FindResource16(char * type, char * name, HMODULE16 h_module);
// HGLOBAL16 LoadResource16(HRSRC16 h_rsrc, HMODULE16 h_module);
// SEGPTR WIN16_LockResource16(HGLOBAL16 handle);
// BOOL16 FreeResource16(HGLOBAL16 handle);
// HFILE16 _lclose16(HFILE16 h_file);
// HFILE16 _lcreat16(i16 attr, char * path);
// i32 _llseek16(i16 n_origin, i32 l_offset, HFILE16 h_file);
// HFILE16 _lopen16(i16 mode, char * path);
// i16 lstrlen16(char * in_string);
// pub fn InitTask16(CONTEXT * context);
// pub fn DOS3Call(CONTEXT * context);
// u16 SetErrorMode16(u16 mode);
// pub fn __AHSHIFT(void);
// pub fn __AHINCR(void);
// pub fn OutputDebugString16(char * str);
pub fn OutputDebugString16(in_str: *const c_char) {
    todo!()
}
// i16 GetPrivateProfileString16(char * filename, u16 len, char * buffer, char * def_val, char * entry, char * section);
// BOOL16 WritePrivateProfileString16(char * filename, char * string, char * entry, char * section);
// pub fn * GetDOSEnvironment16(void);
// pub fn FatalAppExit16(char * str, u16 action);
// HINSTANCE16 WinExec16(u16 n_cmd_show, char * lp_cmd_line);
// pub fn __WINFLAGS(void);
// pub fn GlobalDOSAlloc16(u32 size) -> u32;
// u16 GlobalDOSFree16(u16 sel);
// u16 GlobalPageLock16(HGLOBAL16 handle);
// u16 GlobalPageUnlock16(HGLOBAL16 handle);
// pub fn hmemcpy16(i32 count, void * src, void * dst);
// i32 WIN16_hread(i32 count, void * buffer, HFILE16 h_file);
// pub fn _hwrite16(count: u32, u8 * buffer, HFILE16 h_file) -> u32;
// COLORREF SetBkColor16(COLORREF color, HDC16 hdc);
// i16 SetMapMode16(i16 mode, HDC16 hdc);
// u8 SetTextColor16(COLORREF color, HDC16 hdc);
// BOOL16 LineTo16(INT16 y, INT16 x, HDC16 hdc);
// DWORD MoveTo16(INT16 y, INT16 x, HDC16 hdc);
// BOOL16 Ellipse16(INT16 bottom, INT16 right, INT16 top, INT16 left, HDC16 hdc);
// BOOL16 Rectangle16(INT16 bottom, INT16 right, INT16 top, INT16 left, HDC16 hdc);
// BOOL16 TextOut16(i16 count, char * str, i16 y, i16 x, HDC16 hdc);
// BOOL16 Polygon16(i16 count, POINT16 * pt, HDC16 hdc);
// HGDIOBJ16 SelectObject16(HGDIOBJ16 handle, HDC16 hdc);
// HDC16 CreateDC16(DEVMODEA * init_data, char * output, char * device, char * driver);
// HPEN16 CreatePen16(COLORREF color, INT16 width, INT16 style);
// HBRUSH16 CreateSolidBrush16(COLORREF color);
// BOOL16 DeleteDC16(HDC16 hdc);
// BOOL16 DeleteObject16(HGDIOBJ16 obj);
// DWORD GetCurrentPosition16(HDC16 hdc);
// INT16 GetDeviceCaps16(INT16 cap, HDC16 hdc);
// HGDIOBJ16 GetStockObject16(INT16 obj);
pub fn GetStockObject16(obj: i16) -> HGDIOBJ16 {
    todo!()
}
// DWORD GetTextExtent16(INT16 count, LPCSTR str, HDC16 hdc);
// BOOL16 UnrealizeObject16(HGDIOBJ16 obj);
// HPALETTE16 CreatePalette16(LOGPALETTE * palette);
// UINT16 GetSystemPaletteEntries(PALETTEENTRY * entries, UINT16 count, UINT16 start, HDC16 hdc);
// INT16 StretchDIBits16(DWORD dw_rop, UINT16 w_usage, BITMAPINFO * info, PVOID bits, INT16 height_src, INT16 width_src, INT16 y_src, INT16 x_src, INT16 height_dst, INT16 width_dst, INT16 y_dst, INT16 x_dst, HDC16 hdc);
// INT16 SetDIBitsToDevice(UINT16 coloruse, BITMAPINFO * info, void * bits, UINT16 lines, UINT16 startscan, INT16 y_src, INT16 x_src, INT16 cy, INT16 cx, INT16 y_dest, INT16 x_dest, HDC16 hdc);
// BOOL16 MoveToEx16(POINT16 * pt, INT16 y, INT16 x, HDC16 hdc);
// INT16 MessageBox16(UINT16 type, char * title, char * text, HWND16 hwnd);
// INT16 InitApp16(HINSTANCE16 h_instance);
// pub fn PostQuitMessage16(INT16 exit_code);
// UINT16 SetTimer16(void * proc, UINT16 timeout, UINT16 id, HWND16 hwnd);
// BOOL16 KillTimer16(UINT16 id, HWND16 hwnd);
// BOOL16 GetCursorPos16(POINT16 * pt);
// HWND16 SetCapture16(HWND16 hwnd);
// BOOL16 ReleaseCapture16(void);
// HWND16 SetFocus16(HWND16 hwnd);
pub fn SetFocus16(hwnd: HWND16) -> HWND16 {
    todo!()
}
// HANDLE16 RemoveProp16(LPCSTR str, HWND16 hwnd);
// HANDLE16 GetProp16(LPCSTR in_string, HWND16 hwnd);
// BOOL16 SetProp16(HANDLE16 handle, char * str, HWND16 hwnd);
// pub fn ClientToScreen16(POINT16 * lppnt, HWND16 hwnd);
// pub fn ScreenToClient16(POINT16 * lppnt, HWND16 hwnd);
// BOOL16 IsIconic16(HWND16 hwnd);
// pub fn GetWindowRect16(RECT16 * rect, HWND16 hwnd);
// pub fn GetClientRect16(RECT16 * rect, HWND16 hwnd);
pub fn GetClientRect16(rect: *mut RECT16, hwnd: HWND16) {
    todo!()
}
// BOOL16 EnableWindow16(BOOL16 enable, HWND16 hwnd);
// BOOL16 IsWindowEnabled16(HWND16 hwnd);
// INT16 GetWindowText16(INT16 n_max_count, lp_string: u32, HWND16 hwnd);
// BOOL16 SetWindowText16(lp_string: u32, HWND16 hwnd);
// HDC16 BeginPaint16(PAINTSTRUCT16 * lps, HWND16 hwnd);
// BOOL16 EndPaint16(PAINTSTRUCT16 * lps, HWND16 hwnd);
// HWND16 CreateWindow16(DWORD style, void * data, HINSTANCE16 instance, HMENU16 hmenu, HWND16 parent, INT16 height, INT16 width, INT16 y, INT16 x, char * window_name, char * class_name);
pub fn CreateWindow16(style: u32, data: *mut c_void, instance: HINSTANCE16, hmenu: HMENU16, parent: HWND16, height: i16, width: i16, y: i16, x: i16, window_name: *const c_char, class_name: *const c_char) -> HWND16 {
    todo!()
}

// BOOL16 ShowWindow16(INT16 cmd, HWND16 hwnd);
pub fn ShowWindow16(cmd: i16, hwnd: HWND16) -> bool {
    todo!()
}
// BOOL16 BringWindowToTop16(HWND16 hwnd);
// BOOL16 IsWindow16(HWND16 hwnd);
// BOOL16 DestroyWindow16(HWND16 hwnd);
pub fn DestroyWindow16(hwnd: HWND16) -> bool {
todo!()
}
// BOOL16 EnumChildWindows1(LPARAM lparam, void * func, HWND16 parent);
// BOOL16 MoveWindow16(BOOL16 repaint, INT16 cy, INT16 cx, INT16 y, INT16 x, HWND16 hwnd);
// ATOM RegisterClass16(WNDCLASS16 * wc);
pub fn RegisterClass16(wc: *mut WNDCLASS16) -> ATOM {
    todo!()
}
// HDC16 GetDC16(HWND16 hwnd);
// HDC16 GetWindowDC16(HWND16 hwnd);
// INT16 ReleaseDC16(HDC16 hdc, HWND16 hwnd);
// HCURSOR16 SetCursor16(HCURSOR16 hcursor);
// INT16 ShowCursor16(BOOL16 b_show);
// BOOL16 PtInRect16(POINT16 pt, RECT16 * rect);
// i16 FillRect16(HBRUSH16 hbrush, RECT16 * rect, HDC16 hdc);
// INT16 FrameRect16(HBRUSH16 hbrush, RECT16 * rect, HDC16 hdc);
// BOOL16 DrawIcon16(HICON16 h_icon, INT16 y, INT16 x, HDC16 hdc);
// INT16 DrawText16(UINT16 flags, RECT16 * rect, INT16 count, LPCSTR in_string, HDC16 hdc);
// HWND16 CreateDialog16(void * dlg_proc, HWND16 owner, char * dlg_template, HINSTANCE16 hinst);
// BOOL16 IsDialogMessage16(MSG16 * msg16, HWND16 hwnd_dlg);
// HWND16 GetDlgItem16(INT16 id, HWND16 hwnd_dlg);
pub fn GetDlgItem16(id: i16, hwnd_dlg: HWND16) -> HWND16 {
    todo!();
}
// pub fn SetDlgItemText16(lp_string: u32, INT16 id, HWND16 hwnd);
// pub fn SetDlgItemInt16(BOOL16 f_signed, UINT16 value, INT16 id, HWND16 hwnd);
// UINT16 GetDlgItemInt16(BOOL16 f_signed, BOOL16 * translated, INT16 id, HWND16 hwnd);
// BOOL16 CheckRadioButton16(UINT16 check_id, UINT16 last_id, UINT16 first_id, HWND16 hwnd_dlg);
// BOOL16 CheckDlgButton16(UINT16 check, INT16 id, HWND16 hwnd);
// UINT16 IsDlgButtonChecked(UINT16 id, HWND16 hwnd);
// LRESULT SendDlgItemMessage16(LPARAM l_param, w_param: WPARAM16, UINT16 msg, INT16 id, HWND16 hwnd);
// pub fn MapDialogRect16(RECT16 * rect, HWND16 hwnd);
// pub fn MessageBeep16(UINT16 i);
// LRESULT DefWindowProc16(LPARAM lparam, wparam: WPARAM16, UINT16 msg, HWND16 hwnd);
// BOOL16 GetMessage16(UINT16 last, UINT16 first, HWND16 hwnd, MSG16 * msg);
// BOOL16 PostMessage16(LPARAM lparam, wparam: WPARAM16, UINT16 msg, HWND16 hwnd);
// LRESULT SendMessage16(LPARAM lparam, wparam: WPARAM16, UINT16 msg, HWND16 hwnd);
pub fn SendMessage16(lparam: LPARAM, wparam: WPARAM16, msg: u16, hwnd: HWND16) -> LRESULT {
    todo!()
}
// BOOL16 TranslateMessage16(MSG16 * msg);
// long DispatchMessage16(MSG16 * msg);
// LRESULT CallWindowProc16(LPARAM lparam, wparam: WPARAM16, UINT16 msg, HWND16 hwnd, LPVOID func);
// pub fn UpdateWindow16(HWND16 hwnd);
// pub fn InvalidateRect16(BOOL16 erase, RECT16 * rect, HWND16 hwnd);
// pub fn ValidateRect16(RECT16 * rect, HWND16 hwnd);
// WORD GetWindowWord16(INT16 offset, HWND16 hwnd);
// WORD SetWindowWord16(WORD newval, INT16 offset, HWND16 hwnd);
// long GetWindowLong16(INT16 offset, HWND16 hwnd);
// long SetWindowLong16(long newval, i16 offset, HWND16 hwnd);
// HMENU16 LoadMenu16(char * name, HINSTANCE16 instance);
// BOOL16 DestroyMenu16(HMENU16 menu);
// BOOL16 CheckMenuItem16(UINT16 w_flags, UINT16 w_item_id, HMENU16 hmenu);
// BOOL16 EnableMenuItem16(UINT16 w_flags, UINT16 w_item_id, HMENU16 hmenu);
// HMENU16 GetSubMenu16(INT16 n_pos, HMENU16 h_menu);
// BOOL16 WinHelp16(DWORD dw_data, UINT16 w_command, char * lp_help_file, HWND16 hwnd);
// HCURSOR16 LoadCursor16(char * name, HINSTANCE16 h_instance);
// HICON16 LoadIcon16(char * name, HINSTANCE16 h_instance);
// INT16 LoadString16(INT16 buf_len, char * buffer, UINT16 resource_id, HINSTANCE16 instance);
// HACCEL16 LoadAccelerators16(char * lp_table_name, HINSTANCE16 instance);
// INT16 TranslateAccelerator16(MSG16 * msg, HACCEL16 haccel, HWND16 hwnd);
// INT16 GetSystemMetrics16(INT16 index);
// COLORREF GetSysColor16(INT16 index);
// pub fn SetSysColors16(COLORREF * values, INT16 * list, INT16 count);
// BOOL16 GrayString16(INT16 cy, INT16 cx, INT16 y, INT16 x, INT16 cch, LPARAM lparam, void * gsprc, HBRUSH16 param_8, HDC16 hdc);
// HWND16 SetSysModalWindow(HWND16 hwnd);
// HWND16 GetNextDlgTabItem16(BOOL16 f_previous, HWND16 hwnd_ctrl, HWND16 hwnd_dlg);
// BOOL16 SetWindowPos16(WORD flags, INT16 cy, INT16 cx, INT16 y, INT16 x, HWND16 hwnd_insert_after, HWND16 hwnd);
// UINT16 GetMenuState16(UINT16 w_flags, UINT16 w_item_id, HMENU16 hmenu);
// INT16 GetDlgCtrlID16(HWND16 hwnd);
// HPALETTE16 SelectPalette16(BOOL16 b_force_background, HPALETTE16 hpal, HDC16 hdc);
// UINT16 RealizePalette16(HDC16 hdc);
// BOOL16 GetWindowPlacement16(WINDOWPLACEMENT16 * wp16, HWND16 hwnd);
// BOOL16 SetWindowPlacement16(WINDOWPLACEMENT16 * wp16, HWND16 hwnd);
// BOOL16 GetClassInfo16(WNDCLASS16 * wc, char * name, HINSTANCE16 h_inst16);
pub fn GetClassInfo16(wc: *mut WNDCLASS16, name: *const c_char, h_inst16: HINSTANCE16) -> bool {
    todo!()
}
// BOOL16 InsertMenu16(data: u32, UINT16 id, UINT16 flags, UINT16 pos, HMENU16 hmenu);
// BOOL16 DeleteMenu16(UINT16 wflags, UINT16 npos, HMENU16 hmenu);
// BOOL16 ModifyMenu16(data: u32, UINT16 id, UINT16 flags, UINT16 pos, HMENU16 hmenu);
// BOOL16 TrackPopupMenu16(RECT16 * lp_rect, HWND16 hwnd, INT16 n_reserved, INT16 y, INT16 x, UINT16 wflags, HMENU16 hmenu);
// INT16 wsprintf16(WORD * valist, char * spec, char * buffer, ...);
// INT16 wvsprintf16(WORD * args, char * spec, char * buffer);
// HWND16 CreateWIndowEx16(void * data, HINSTANCE16 instance, HMENU16 hmenu, HWND16 parent, INT16 height, INT16 width, INT16 y, INT16 x, DWORD style, char * window_name, char * class_name, DWORD ex_style);
// BOOL16 DestroyIcon16(HICON16 h_icon);
// BOOL16 DestroyCursor16(HCURSOR16 h_cursor);
// DWORD mciSendCommand16(DWORD p2, DWORD dw_param1, UINT16 w_msg, UINT16 w_dev_id);
// BOOL16 mciGetErrorString16(UINT16 u_length, char * lp_str_buffer, DWORD w_error);
// BOOL16 GetOpenFileName16(u32 ofn);
// BOOL16 GetSaveFileName16(u32 ofn);
// void* swi(u16);
// pub fn SegmentLimit(u32 a) -> u32;
