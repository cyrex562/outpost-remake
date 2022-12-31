
use std::ffi::c_void;
use std::io::stdout;
use std::os::raw::c_char;
use std::os::windows::raw::HANDLE;
use crate::app_context::AppContext;
use crate::unk::block_1000_1000::{msg_box_op_1000_1f24, pass1_1000_1f68};
use crate::unk::block_1000_2000::mem_op_1000_21b6;
use crate::windef16::{ATOM, BOOL16, COLORREF, CONTEXT, DEVMODEA, DLGPROC16, FARPROC16, FLOATING_SAVE_AREA, GRAYSTRINGPROC16, HACCEL16, HANDLE16, HBRUSH16, HCURSOR16, HDC16, HFILE16, HGDIOBJ16, HGLOBAL16, HICON16, HINSTANCE16, HMENU16, HMODULE16, HPALETTE16, HPEN16, HRSRC16, HTASK16, HWND16, LOGPALETTE, LPARAM, LRESULT, MSG16, PAINTSTRUCT16, PALETTEENTRY, POINT16, RECT16, SEGPTR, TIMERPROC16, WINDOWPLACEMENT16, WNDCLASS16, WNDENUMPROC16, WNDPROC16, WPARAM16}; // CONTEXT;
/* SS:SP, CS:IP, FLAGS, BP */ /* AX, BX, CX, DX, SI, DI */ /* DS, ES, FS, GS */ /* 387 state */ /* DB 0-3,6,7 */


// HGLOBAL16 WINAPI LockSegment16	(	HGLOBAL16 		)
pub fn LockSegment16(a: HGLOBAL16) -> HGLOBAL16 {
    todo!()
}

pub fn WinAPI16_WaitEvent16(a: HTASK16) -> bool {
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

// HGLOBAL16 GLobalAlloc16(Dsize: u16, Uflags: INT16);
pub fn GlobalAlloc16(size: u32, flags: u16) -> HGLOBAL16
{
    todo!()
}

// HGLOBAL16 GlobalReAlloc16(Uflags: INT16, Dsize: u16, HGLOBAL16 handle);
pub fn GlobalReAlloc16(flags: u16, size: u16, handle: u32) -> HGLOBAL16 {todo!()}

// HGLOBAL16 GlobalFree16(HGLOBAL16 handle);
pub fn GlobalFree16(handle: HGLOBAL16) -> HGLOBAL16 {todo!()}

// pub fn * WIN16_GlobalLock16(HGLOBAL16 handle);
pub fn GlobalLock16(handle: HGLOBAL16) -> *mut c_void {todo!()}

// BOOL16 GlobalUnlock16(HGLOBAL16 handle);
pub fn GlobalUnlock16(handle: HGLOBAL16) -> bool {todo!()}

// DGlobalSize16: u16(HGLOBAL16 handle);
pub fn GlobalSize16(handle: HGLOBAL16) -> u32 {todo!()}

// DGlobalHandle16: u16(sel: u16);
pub fn GlobalHandle16(selector: u16) -> u32 {todo!()}

// i16 GetModuleFileName16(i16 n_size, char * lp_file_name, HINSTANCE16 h_module);
pub fn GetModuleFileName16(size: i16, filename: *mut c_char, h_module: HINSTANCE16) -> i16 {todo!()}

// pub fn * MakeProcInstance16(HANDLE16 h_instance, void * func);
pub fn MakeProcInstance16(h_instance: HANDLE16, func: FARPROC16) -> FARPROC16 {todo!()}

// pub fn FreeProcInstance16(void * func);
pub fn FreeProcInstance(func: FARPROC16) {todo!()}

// HRSRC16 FindResource16(char * type, char * name, HMODULE16 h_module);
pub fn FindResource16(rsrc_tyoe: *mut c_char, name: *mut c_char, h_module: HMODULE16) -> HRSRC16 {todo!()}

// HGLOBAL16 LoadResource16(HRSRC16 h_rsrc, HMODULE16 h_module);
pub fn LoadResource16(h_rsrc: HRSRC16, h_module: HMODULE16) -> HGLOBAL16 { todo!()}

// SEGPTR WIN16_LockResource16(HGLOBAL16 handle);
pub fn LockResource16(handle: HGLOBAL16) -> SEGPTR {todo!()}

// BOOL16 FreeResource16(HGLOBAL16 handle);
pub fn FreeResource16(handle: HGLOBAL16) -> bool {todo!()}

// _lclose16: HFILE16(h_file: HFILE16);
pub fn _lclose16(h_file: HFILE16) -> HFILE16 {todo!()}

// _lcreat16: HFILE16(i16 attr, char * path);
// HFILE16     WINAPI _lcreat16(LPCSTR,INT16);
pub fn _lcreat16(attr: i16, path: *mut c_char) -> HFILE16 {todo!()}

// _llseek16: i32(i16 n_origin, l_offset: i32, h_file: HFILE16);
pub fn _llseek16(n_origin: i16, l_offset: i32, h_file: HFILE16) -> i32 {todo!()}

// _lopen16: HFILE16(i16 mode, char * path);
pub fn _lopen16(mode: i16, path: *mut c_char) -> HFILE16 {todo!()}

// i16 lstrlen16(char * in_string);
pub fn lstrlen16(in_str: *const c_char) -> i16 {todo!()}

// pub fn DOS3Call(CONTEXT * context);
pub fn DOS3Call(context: *mut CONTEXT) {todo!()}

// u16 SetErrorMode16(u16 mode);
pub fn SetErrorMode16(mode: u16) -> u16 {todo!()}

// pub fn __AHSHIFT();
pub fn __AHSHIFT() {todo!()}

// pub fn __AHINCR();
pub fn __AHINCR() {todo!()}

// pub fn OutputDebugString16(char * str);
pub fn OutputDebugString16(in_str: *const c_char) {
    todo!()
}

// i16 GetPrivateProfileString16(char * filename, len: u16, char * buffer, char * def_val, char * entry, char * section);
pub fn GetPrivateProfileString16(filename: *const c_char, len: u16, buffer: *mut c_char, def_val: *const c_char, entry: *mut c_char, section: *mut c_char) -> i16 {todo!()}

// BOOL16 WritePrivateProfileString16(char * filename, char * string, char * entry, char * section);
pub fn WritePrivateProfileString16(filename: *const c_char, str_to_write: *const c_char, entry: *const c_char, section: *const c_char) -> bool {todo!()}

// pub fn * GetDOSEnvironment16();
pub fn GetDOSEnvironment16() -> SEGPTR {
    todo!()
}

// HINSTANCE16 WinExec16(n_cmd_show: u16, char * lp_cmd_line);
pub fn WinExec16(n_cmd_show: u16, lp_cmd_line: *const c_char) -> HINSTANCE16 {todo!()}

// pub fn __WINFLAGS();
pub fn __WINFLAGS() {todo!()}

// pub fn GlobalDOSAlloc16(size: u32) -> u32;
pub fn GlobalDOSAlloc16(size: u32) -> u32 {todo!()}

// u16 GlobalDOSFree16(u16 sel);
pub fn GlobalDOSFree16(sel: u16) -> u16 {todo!()}

// u16 GlobalPageLock16(HGLOBAL16 handle);
pub fn GlobalPageLock16(handle: HGLOBAL16) -> u16 {todo!()}

// u16 GlobalPageUnlock16(HGLOBAL16 handle);
pub fn GlobalPageUnlock16(handle: HGLOBAL16) -> u16 {todo!()}

// pub fn hmemcpy16(count: i32, void * src, void * dst);
pub fn hmemcpy16(count: i32, src: *mut c_void, dst: *c_void) {todo!()}

// WIN16_hread: i32(count: i32, void * buffer, h_file: HFILE16);
pub fn hread(count: i32, buffer: *mut c_void, h_file: HFILE16) -> i32 {todo!()}

// pub fn _hwrite16(count: u32, u8 * buffer, h_file: HFILE16) -> u32;
pub fn _hwrite16(count: u32, buffer: *mut u8, h_file: HFILE16) -> u32 {todo!()}

// SetBkColor16: COLORREF(color: COLORREF, hdc: HDC16);
pub fn SetBkColor16(color: COLORREF, hdc: HDC16) -> COLORREF {todo!()}

// i16 SetMapMode16(i16 mode, hdc: HDC16);
pub fn SetMapMode16(mode: i16, hdc: HDC16) -> i16 {todo!()}

// u8 SetTextColor16(color: COLORREF, hdc: HDC16);
pub fn SetTextColor16(color: COLORREF, hdc: HDC16) -> u8 {
    todo!()
}

// BOOL16 LineTo16(y: INT16, x: INT16, hdc: HDC16);
pub fn LineTo16(y: i16, x: i16, hdc: HDC16) -> bool {
    todo!()
}

// DMoveTo16: u16(y: INT16, x: INT16, hdc: HDC16);
pub fn MoveTo16(y: i16, x: i16, hdc: HDC16) -> u16 { todo!()}

// BOOL16 Ellipse16(bottom: INT16, right: INT16, top: INT16, left: INT16, hdc: HDC16);
pub fn Ellipse16(bottom: i16, right: i16, top: i16, left: i16, hdc: HDC16) -> bool {
    todo!()
}

// BOOL16 Rectangle16(bottom: INT16, right: INT16, top: INT16, left: INT16, hdc: HDC16);
pub fn Rectangle16(bottom: i16, right: i16, top: i16, left: i16, hdc: HDC16) -> bool {
    todo!()
}

// BOOL16 TextOut16(i16 count, char * str, i16 y, i16 x, hdc: HDC16);
pub fn TextOut16(count: i16, text: *mut c_char, y: i16, x: i16, hdc: HDC16) -> bool {
    todo!()
}

// BOOL16 Polygon16(i16 count, POINT16 * pt, hdc: HDC16);
pub fn Polygon16(count: i16, pt: *const POINT16, hdc: HDC16) -> bool {todo!()}

// HGDIOBJ16 SelectObject16(HGDIOBJ16 handle, hdc: HDC16);
pub fn SelectObject16(handle: HGDIOBJ16, hdc: HDC16) -> HGDIOBJ16 {todo!()}

// CreateDC16: HDC16(DEVMODEA * init_data, char * output, char * device, char * driver);
pub fn CreateDC16(init_data: *const DEVMODEA, output: *mut c_char, device: *const c_char, driver: *const c_char) -> HDC16 {todo!()}

// CreatePen16: HPEN16(color: COLORREF, width: INT16, style: INT16);
pub fn CreatePen16(color: COLORREF, width: i16, style: i16) -> HPEN16 {todo!()}

// HBRUSH16 CreateSolidBrush16(color: COLORREF);
pub fn CreateSolidBrush16(color: COLORREF) -> HBRUSH16 {todo!()}

// BOOL16 DeleteDC16(hdc: HDC16);
pub fn DeleteDC16(hdc: HDC16) -> bool {todo!()}

// BOOL16 DeleteObject16(HGDIOBJ16 obj);
pub fn DeleteObject16(obj: HGDIOBJ16) -> bool {todo!()}

// DGetCurrentPosition16: u16(hdc: HDC16);
pub fn GetCurrentPosition16(hdc: HDC16) -> u16 { todo!()}

// GetDeviceCaps16: INT16(cap: INT16, hdc: HDC16);
pub fn GetDeviceCaps16(cap: i16, hdc: HDC16) -> i16 {todo!()}

// HGDIOBJ16 GetStockObject16(obj: INT16);
pub fn GetStockObject16(obj: i16) -> HGDIOBJ16 {
    todo!()
}

// DGetTextExtent16: u16(count: INT16, LPCSTR str, hdc: HDC16);
pub fn GetTextExtent16(count: i16, text: *const c_char, hdc: HDC16) -> u16 {todo!()}

// BOOL16 UnrealizeObject16(HGDIOBJ16 obj);
pub fn UnrealizeObject16(obj: HGDIOBJ16) -> bool {todo!()}

// CreatePalette16: HPALETTE16(LOGPALETTE * palette);
pub fn CreatePalette16(palette: *mut LOGPALETTE) -> HPALETTE16 {todo!()}

// UGetSystemPaletteEntries: INT16(PALETTEENTRY * entries, Ucount: INT16, Ustart: INT16, hdc: HDC16);
pub fn GetSystemPaletteEntries(entries: *mut PALETTEENTRY, count: i16, start: i16, hdc: HDC16) -> i16 { todo!()}

// StretchDIBits16: INT16(Ddw_rop: u16, Uw_usage: INT16, BITMAPINFO * info, bits: mut PVOID, height_src: INT16, width_src: INT16, y_src: INT16, x_src: INT16, height_dst: INT16, width_dst: INT16, y_dst: INT16, x_dst: INT16, hdc: HDC16);
pub fn StrechDIBits16(dw_rop: u16, w_usage: i16, info: *mut BITMAPINFO, bits: *mut c_void, height_src: i16, width_src: i16, y_src: i16, x_src: i16, height_dst: i16, width_dst: i16, y_dst: i16, x_dst: i16, hdc: HDC16) -> i16 {
    todo!()
}

// SetDIBitsToDevice: INT16(Ucoloruse: INT16, BITMAPINFO * info, void * bits, Ulines: INT16, Ustartscan: INT16, y_src: INT16, x_src: INT16, cy: INT16, cx: INT16, y_dest: INT16, x_dest: INT16, hdc: HDC16);
pub fn SetDiBitsToDevice(coloruse: i16, info: *mut BITMAPINFO, bits: *mut c_void, lines: i16, startscan: i16, y_src: i16, x_src: i16, cy: i16, cx: i16, y_dest: i16, x_dest: i16, hdc: HDC16) -> i16 {
    todo!()
}

// BOOL16 MoveToEx16(POINT16 * pt, y: INT16, x: INT16, hdc: HDC16);
pub fn MoveToEx16(pt: *mut POINT16, y: i16, x: i16, hdc: HDC16) -> bool { todo!()}

// MessageBox16: INT16(Utype: INT16, char * title, char * text, hwnd: HWND16);
pub fn WinAPI16_MessageBox16(msg_type: i16, title: *mut c_char, text: c_char, hwnd: HWND16) -> i16 { todo!()}

// pub fn PostQuitMessage16(exit_code: INT16);
pub fn PostQuitMessage16(exit_code: i16) {todo!()}

// USetTimer16: INT16(void * proc, Utimeout: INT16, Uid: INT16, hwnd: HWND16);
pub fn SetTimer16(proc: TIMERPROC16, timeout: i16, id: i16, hwnd: HWND16) -> i16 {todo!()}

// BOOL16 KillTimer16(Uid: INT16, hwnd: HWND16);
pub fn KillTimer16(id: i16, hwnd: HWND16) -> bool {todo!()}

// BOOL16 GetCursorPos16(POINT16 * pt);
pub fn GetCursorPos16(pt: *mut POINT16) -> bool {todo!()}

// SetCapture16: HWND16(hwnd: HWND16);
pub fn SetCapture16(hwnd: HWND16) -> HWND16 {todo!()}

// BOOL16 ReleaseCapture16();
pub fn ReleaseCapture16() -> bool {todo!()}

// SetFocus16: HWND16(hwnd: HWND16);
pub fn SetFocus16(hwnd: HWND16) -> HWND16 {
    todo!()
}

// HANDLE16 RemoveProp16(LPCSTR str, hwnd: HWND16);
pub fn RemoveProp16(str_prop: *mut c_char, hwnd: HWND16) -> HANDLE16 {todo!()}

// HANDLE16 GetProp16(LPCSTR in_string, hwnd: HWND16);
pub fn GetProp16(in_string: *mut c_char, hwnd: HWND16) -> HANDLE16 {todo!()}

// BOOL16 SetProp16(HANDLE16 handle, char * str, hwnd: HWND16);
pub fn SetProp16(handle: HANDLE16, in_str: *mut c_char, hwnd: HWND16) -> bool {todo!()}

// pub fn ClientToScreen16(POINT16 * lppnt, hwnd: HWND16);
pub fn ClientToScreen16(lppnt: *mut POINT16, hwnd: HWND16) {todo!()}

// pub fn ScreenToClient16(POINT16 * lppnt, hwnd: HWND16);
pub fn ScreenToClient16(lppnt: *mut POINT16, hwnd: HWND16) {todo!()}

// BOOL16 IsIconic16(hwnd: HWND16);
pub fn IsIconic16(hwnd: HWND16) -> bool {todo!()}

// pub fn GetWindowRect16(RECT16 * rect, hwnd: HWND16);
pub fn GetWindowRect16(rect: *mut RECT16, hwnd: HWND16) { todo!()}

// pub fn GetClientRect16(RECT16 * rect, hwnd: HWND16);
pub fn GetClientRect16(rect: *mut RECT16, hwnd: HWND16) {
    todo!()
}

// BOOL16 EnableWindow16(BOOL16 enable, hwnd: HWND16);
pub fn EnableWindow16(enable: BOOL16, hwnd: HWND16) -> bool {todo!()}

// BOOL16 IsWindowEnabled16(hwnd: HWND16);
pub fn IsWindowEnabled(hwnd: HWND16) -> bool {todo!()}

// GetWindowText16: INT16(n_max_count: INT16, lp_string: u32, hwnd: HWND16);
pub fn GetWindowText16(n_max_count: i16, lp_string: SEGPTR, hwnd: HWND16) -> i16 {todo!()}

// BOOL16 SetWindowText16(lp_string: u32, hwnd: HWND16);
pub fn SetWindowText16(lp_string: SEGPTR, hwnd: HWND16) -> bool {todo!()}

// BeginPaint16: HDC16(PAINTSTRUCT16 * lps, hwnd: HWND16);
pub fn BeginPaint16(lps: *mut PAINTSTRUCT16, hwnd: HWND16) -> HDC16 {todo!()}

// BOOL16 EndPaint16(PAINTSTRUCT16 * lps, hwnd: HWND16);
pub fn EndPaint16(lps: *mut PAINTSTRUCT16, hwnd: HWND16) -> bool {todo!()}

// CreateWindow16: HWND16(Dstyle: u16, void * data, HINSTANCE16 instance, HMENU16 hmenu, parent: HWND16, height: INT16, width: INT16, y: INT16, x: INT16, char * window_name, char * class_name);
pub fn CreateWindow16(style: u32, data: *mut c_void, instance: HINSTANCE16, hmenu: HMENU16, parent: HWND16, height: i16, width: i16, y: i16, x: i16, window_name: *const c_char, class_name: *const c_char) -> HWND16 {
    todo!()
}

// BOOL16 ShowWindow16(cmd: INT16, hwnd: HWND16);
pub fn ShowWindow16(cmd: i16, hwnd: HWND16) -> bool {
    todo!()
}

// BOOL16 BringWindowToTop16(hwnd: HWND16);
pub fn BringWindowToTop16(hwnd: HWND16) -> bool {todo!()}

// BOOL16 IsWindow16(hwnd: HWND16);
pub fn IsWindow16(hwnd: HWND16) -> bool {todo!()}

// BOOL16 DestroyWindow16(hwnd: HWND16);
pub fn DestroyWindow16(hwnd: HWND16) -> bool {
todo!()
}

// BOOL16 EnumChildWindows1(lparam: LPARAM, void * func, parent: HWND16);
pub fn EnumChildWindows16(lparam: LPARAM, func: WNDENUMPROC16, parent: HWND16) -> bool {todo!()}

// BOOL16 MoveWindow16(BOOL16 repaint, cy: INT16, cx: INT16, y: INT16, x: INT16, hwnd: HWND16);
pub fn MoveWindow16(repaint: BOOL16, cy: i16, cx: i16, y: i16, x: i16, hwnd: HWND16) -> bool {todo!()}

// RegisterClass16: ATOM(WNDCLASS16 * wc);
pub fn RegisterClass16(wc: *mut WNDCLASS16) -> ATOM {
    todo!()
}

// GetDC16: HDC16(hwnd: HWND16);
pub fn GetDC16(hwnd: HWND16) -> HDC16 {todo!()}

// GetWindowDC16: HDC16(hwnd: HWND16);
pub fn GetWindowDC16(hwnd: HWND16) -> HDC16 {todo!()}

// ReleaseDC16: INT16(hdc: HDC16, hwnd: HWND16);
pub fn ReleeaseDC16(hdc: HDC16, hwnd: HWND16) -> i16 {todo!()}

// HCURSOR16 SetCursor16(HCURSOR16 hcursor);
pub fn SetCursor16(hcursor: HCURSOR16) -> HCURSOR16 {todo!()}

// ShowCursor16: INT16(BOOL16 b_show);
pub fn ShowCursor16(b_show: bool) -> i16 {todo!()}

// BOOL16 PtInRect16(POpt: INT16, RECT16 * rect);
pub fn WinAPI16_PtInRect16(point: POINT16, rect: *mut RECT16) -> bool {todo!()}

// i16 FillRect16(HBRUSH16 hbrush, RECT16 * rect, hdc: HDC16);
pub fn FillRect16(hbrush: HBRUSH16, rect: *mut RECT16, hdc: HDC16) -> i16 {todo!()}

// FrameRect16: INT16(HBRUSH16 hbrush, RECT16 * rect, hdc: HDC16);
pub fn FrameRect16(hbrush: HBRUSH16, rect: *mut RECT16, hdc: HDC16) -> i16 {todo!()}

// BOOL16 DrawIcon16(HICON16 h_icon, y: INT16, x: INT16, hdc: HDC16);
pub fn DrawIcon16(h_icon: HICON16, y: i16, x: i16, hdc: HDC16) -> bool {todo!()}

// DrawText16: INT16(Uflags: INT16, RECT16 * rect, count: INT16, LPCSTR in_string, hdc: HDC16);
pub fn DrawText16(flags: i16, rect: *mut RECt16, count: i16, in_str: *mut c_char, hdc: HDC16) -> i16 {todo!()}

// CreateDialog16: HWND16(void * dlg_proc, owner: HWND16, char * dlg_template, HINSTANCE16 hinst);
pub fn CreateDialog16(dlg_proc: DLGPROC16, owner: HWND16, dlg_template: *mut c_char, hinst: HINSTANCE16) -> HWND16 {todo!()}

// BOOL16 IsDialogMessage16(MSG16 * msg16, hwnd_dlg: HWND16);
pub fn IsDialogMessage16(msg16: *mut MSG16, hwnd_dlg: HWND16) -> bool {todo!()}

// GetDlgItem16: HWND16(id: INT16, hwnd_dlg: HWND16);
pub fn GetDlgItem16(id: i16, hwnd_dlg: HWND16) -> HWND16 {
    todo!();
}

// pub fn SetDlgItemText16(lp_string: u32, id: INT16, hwnd: HWND16);
pub fn SetDlgItemText16(lp_string: SEGPTR, id: i16, hwnd: HWND16) {todo!()}

// pub fn SetDlgItemInt16(BOOL16 f_signed, Uvalue: INT16, id: INT16, hwnd: HWND16);
pub fn SetDlgItemInt16(f_signed: bool, value: i16, id: i16, hwnd: HWND16) {todo!()}

// UGetDlgItemInt16: INT16(BOOL16 f_signed, BOOL16 * translated, id: INT16, hwnd: HWND16);
pub fn GetDlgItemInt16(f_signed: bool, translated: *mut bool, id: i16, hwnd: HWND16) -> i16 {todo!()}

// BOOL16 CheckRadioButton16(Ucheck_id: INT16, Ulast_id: INT16, Ufirst_id: INT16, hwnd_dlg: HWND16);
pub fn CheckRadioButton16(check_id: i16, last_id: i16, first_id: i16, hwnd_dlg: HWND16) -> bool {todo!()}

// BOOL16 CheckDlgButton16(Ucheck: INT16, id: INT16, hwnd: HWND16);
pub fn CheckDlgButton16(check: i16, id: i16, hwnd: HWND16) -> bool {todo!()}

// UIsDlgButtonChecked: INT16(Uid: INT16, hwnd: HWND16);
pub fn IsDlgButtonChecked(id: i16, hwnd: HWND16) -> i16 {todo!()}

// LRESULT SendDlgItemMessage16(l_param: LPARAM, w_param: WPARAM16, Umsg: INT16, id: INT16, hwnd: HWND16);
pub fn SendDlgItemMessage16(l_param: LPARAM, w_param: WPARAM16, msg: i16, id: i16, hwnd: HWND16) -> LRESULT {todo!()}

// pub fn MapDialogRect16(RECT16 * rect, hwnd: HWND16);
pub fn MapDialogRect16(rect: *mut RECT16, hwnd: HWND16) {todo!()}

// pub fn MessageBeep16(Ui: INT16);
pub fn MessageBeep16(i: i16) {todo!()}

// LRESULT DefWindowProc16(lparam: LPARAM, wparam: WPARAM16, Umsg: INT16, hwnd: HWND16);
pub fn DefWindowProc16(lparam: LPARAM, wparam: WPARAM16, msg: i16, hwnd: HWND16) -> LRESULT {todo!()}

// BOOL16 GetMessage16(Ulast: INT16, Ufirst: INT16, hwnd: HWND16, MSG16 * msg);
pub fn GetMessage16(last: i16, first: i16, hwnd: i16, msg: *mut MSG16) -> bool {todo!()}

// BOOL16 PostMessage16(lparam: LPARAM, wparam: WPARAM16, Umsg: INT16, hwnd: HWND16);
pub fn PostMessage16(lparam: LPARAM, wparam: WPARAM16, msg: i16, hwnd: HWND16) -> bool {todo!()}

// LRESULT SendMessage16(lparam: LPARAM, wparam: WPARAM16, Umsg: INT16, hwnd: HWND16);
pub fn SendMessage16(lparam: LPARAM, wparam: WPARAM16, msg: u16, hwnd: HWND16) -> LRESULT {
    todo!()
}

// BOOL16 TranslateMessage16(MSG16 * msg);
pub fn TranslateMessage16(msg: *mut MSG16) -> bool {todo!()}

// long DispatchMessage16(MSG16 * msg);
pub fn DispatchMessage16(msg: *mut MSG16) -> i32 {todo!()}

// LRESULT CallWindowProc16(lparam: LPARAM, wparam: WPARAM16, Umsg: INT16, hwnd: HWND16, LPVOID func);
pub fn CallWindowProc16(lparam: LPARAM, wparam: WPARAM16, msg: i16, hwnd: HWND16, func: WNDPROC16) -> LRESULT {todo!()}

// pub fn UpdateWindow16(hwnd: HWND16);
pub fn UpdateWindow16(hwnd: HWND16) {todo!()}

// pub fn InvalidateRect16(BOOL16 erase, RECT16 * rect, hwnd: HWND16);
pub fn InvalidateRect16(erase: bool, rect: *mut RECT16, hwnd: HWND16) {todo!()}

// pub fn ValidateRect16(RECT16 * rect, hwnd: HWND16);
pub fn ValidateRect16(rect: *mut RECt16, hwnd: HWND16) {todo!()}

// GetWindowWord16: u16(offset: INT16, hwnd: HWND16);
pub fn GetWindowWord16(offset: i16, hwnd: HWND16) -> u16 {todo!()}

// SetWindowWord16: u16(newval: u16, offset: INT16, hwnd: HWND16);
pub fn SetWindowWord16(newval: u16, offset: i16, hwnd: HWND16) -> u16 {todo!()}

// long GetWindowLong16(offset: INT16, hwnd: HWND16);
pub fn GetWindowLong16(offset: i16, hwnd: HWND16) -> i32 {todo!()}

// long SetWindowLong16(long newval, i16 offset, hwnd: HWND16);
pub fn SetWindowLong16(newval: i32, offset: i16, hwnd: HWND16) -> i32 {todo!()}

// HMENU16 LoadMenu16(char * name, HINSTANCE16 instance);
pub fn LoadMenu16(name: *mut c_char, instance: HINSTANCE16) -> HMENU16 {todo!()}

// BOOL16 DestroyMenu16(HMENU16 menu);
pub fn DestroyMenu16(hmenu: HMENU16) -> bool {todo!()}

// BOOL16 CheckMenuItem16(Uw_flags: INT16, Uw_item_id: INT16, HMENU16 hmenu);
pub fn CheckMenuItem16(flags: i16, item_id: i16, menu: HEMNU16) -> bool {todo!()}

// BOOL16 EnableMenuItem16(Uw_flags: INT16, Uw_item_id: INT16, HMENU16 hmenu);
pub fn EnableMenuItem16(flags: i16, item_id: i16, hmenu: HMENU16) -> bool {todo!()}

// HMENU16 GetSubMenu16(n_pos: INT16, HMENU16 h_menu);
pub fn GetSubMenu16(n_pos: i16, h_menu: HMENU16) -> HMENU16 {todo!()}

// BOOL16 WinHelp16(Ddw_data: u16, Uw_command: INT16, char * lp_help_file, hwnd: HWND16);
pub fn WinHelp16(data: u16, command: i16, lp_help_file: *mut c_char, hwnd: HWND16) -> bool {todo!()}

// HCURSOR16 LoadCursor16(char * name, HINSTANCE16 h_instance);
pub fn LoadCursor16(name: *mut c_char, h_instance: HINSTANCE16) -> HCURSOR16 {todo!()}

// HICON16 LoadIcon16(char * name, HINSTANCE16 h_instance);
pub fn LoadIcon16(name: *mut c_char, h_instance: HINSTANCE16) -> HICON16 {todo!()}

// LoadString16: INT16(buf_len: INT16, char * buffer, Uresource_id: INT16, HINSTANCE16 instance);
pub fn LoadString16(buf_len: i16, buffer: *mut c_char, resource_id: i16, instance: HINSTANCE16) -> i16 {todo!()}

// HACCEL16 LoadAccelerators16(char * lp_table_name, HINSTANCE16 instance);
pub fn LoadAccelerator16(lp_table_name: *mut c_char, instance: HINSTANCE16) -> HACCEL16 {todo!()}

// TranslateAccelerator16: INT16(MSG16 * msg, HACCEL16 haccel, hwnd: HWND16);
pub fn TranslateAccelerator16(msg: *mut MSG16, haccel: HACCEL16, hwnd: HWND16) -> i16 {todo!()}

// GetSystemMetrics16: INT16(index: INT16);
pub fn GetSystemMetrics16(index: i16) -> i16 {todo!()}

// GetSysColor16: COLORREF(index: INT16);
pub fn GetSysColor16(index: i16) -> COLORREF {todo!()}

// pub fn SetSysColors16(COLORREF * values, INT16 * list, count: INT16);
pub fn SetSysColors16(values: *mut COLORREF, list: *mut i16, count: i16) { todo!()}

// BOOL16 GrayString16(cy: INT16, cx: INT16, y: INT16, x: INT16, cch: INT16, lparam: LPARAM, void * gsprc, HBRUSH16 param_8, hdc: HDC16);
pub fn GrayString16(cy: i16, cx: i16, y: i16, x: i16, cch: i16, lparam: LPARAM, proc: GRAYSTRINGPROC16, brush: HBRUSH16, hdc: HDC16) -> bool {todo!()}

// SetSysModalWindow: HWND16(hwnd: HWND16);
pub fn SetSysModalWindow(hwnd: HWND16) -> HWND16 {todo!()}

// GetNextDlgTabItem16: HWND16(BOOL16 f_previous, hwnd_ctrl: HWND16, hwnd_dlg: HWND16);
pub fn GetNextDlgTabItem16(f_previous: bool, hwnd_ctrl: HWND16, hwnd_dlg: HWND16) -> HWND16 {todo!()}

// BOOL16 SetWindowPos16(flags: u16, cy: INT16, cx: INT16, y: INT16, x: INT16, hwnd_insert_after: HWND16, hwnd: HWND16);
pub fn SetWindowPos16(flags: u16, cy: i16, cx: i16, y: i16, x: i16, hwnd_insert_after: HWND16, hwnd: HWND16) -> bool {todo!()}

// UGetMenuState16: INT16(Uw_flags: INT16, Uw_item_id: INT16, HMENU16 hmenu);
pub fn GetMenuState16(flags: i16, item_id: i16, hmenu: HMENU16) -> i16 {todo!()}

// GetDlgCtrlID16: INT16(hwnd: HWND16);
pub fn GetDlgCtrlID16(hwnd: HWND16) -> i16 {todo!()}

// SelectPalette16: HPALETTE16(BOOL16 b_force_background, hpal: HPALETTE16, hdc: HDC16);
pub fn SelectPalette16(b_force_background: bool, hpal: HPALETTE16, hdc: HDC16) -> HPALETTE16 {todo!()}

// URealizePalette16: INT16(hdc: HDC16);
pub fn RealizePalette16(hdc: HDC16) -> i16 {todo!()}

// BOOL16 GetWindowPlacement16(WINDOWPLACEMENT16 * wp16, hwnd: HWND16);
pub fn GetWindowPlacement16(wp16: *mut WINDOWPLACEMENT16, hwnd: HWND16) -> bool {todo!()}

// BOOL16 SetWindowPlacement16(WINDOWPLACEMENT16 * wp16, hwnd: HWND16);
pub fn SetWindowPlacement16(wp16: *mut WINDOWPLACEMENT16, hwnd: HWND16) -> bool {todo!()}

// BOOL16 GetClassInfo16(WNDCLASS16 * wc, char * name, HINSTANCE16 h_inst16);
pub fn GetClassInfo16(wc: *mut WNDCLASS16, name: *const c_char, h_inst16: HINSTANCE16) -> bool {
    todo!()
}

// BOOL16 InsertMenu16(data: u32, Uid: INT16, Uflags: INT16, Upos: INT16, HMENU16 hmenu);
pub fn InsertMenu16(data: u32, id: i16, flags: i16, pos: i16, menu: HMENU16) -> bool {todo!()}

// BOOL16 DeleteMenu16(Uwflags: INT16, Unpos: INT16, HMENU16 hmenu);
pub fn DeleteMenu16(flags: i16, pos: i16, menu: HMENU16) -> bool {todo!()}

// BOOL16 ModifyMenu16(data: u32, Uid: INT16, Uflags: INT16, Upos: INT16, HMENU16 hmenu);
pub fn ModifyMenu16(data: u32, id: i16, flags: i16, pos: i16, menu: HMENU16) -> bool {todo!()}

// BOOL16 TrackPopupMenu16(RECT16 * lp_rect, hwnd: HWND16, n_reserved: INT16, y: INT16, x: INT16, Uwflags: INT16, HMENU16 hmenu);
pub fn TrackPopupMenu16(lp_rect: *mut RECT16, hwnd: HWND16, n_reserved: i16, y: i16, x: i16, flags: i16, hmenu: HMENU16) -> bool {todo!()}

// wsprintf16: INT16(WORD * valist, char * spec, char * buffer, ...);

// wvsprintf16: INT16(WORD * args, char * spec, char * buffer);

// CreateWIndowEx16: HWND16(void * data, HINSTANCE16 instance, HMENU16 hmenu, parent: HWND16, height: INT16, width: INT16, y: INT16, x: INT16, Dstyle: u16, char * window_name, char * class_name, Dex_style: u16);
pub fn CreateWindowEx16(data: *mut c_void, instance: HINSTANCE16, hmenu: HMENU16, parent: HWND16, height: i16, width: i16, y: i16, x: i16, style: i16, window_name: *mut c_char, class_name: *mut c_char, ex_style: u16) -> HWND16 {todo!()}

// BOOL16 DestroyIcon16(HICON16 h_icon);
pub fn DestroyIcon16(h_icon: HICON16) -> bool {todo!()}

// BOOL16 DestroyCursor16(HCURSOR16 h_cursor);
pub fn DestroyCursor16(h_cursor: HCURSOR16) -> bool {todo!()}

// DmciSendCommand16: u16(Dp2: u16, Ddw_param1: u16, Uw_msg: INT16, Uw_dev_id: INT16);
pub fn mciSendCommand16(p2: u32, dw_param1: u32, msg: i16, uw_dev_id: i16) -> u32 {
    todo!()
}

// BOOL16 mciGetErrorString16(Uu_length: INT16, char * lp_str_buffer, Dw_error: u16);
pub fn mciGetErrorString(length: i16, buffer: *mut c_char, error: u16) -> bool {todo!()}

// BOOL16 GetOpenFileName16(ofn: u32);
pub fn GetOpenFileName16(ofn: SEGPTR) -> bool {todo!()}

// BOOL16 GetSaveFileName16(ofn: u32);
pub fn GetSaveFileName16(ofn: SEGPTR) -> bool {todo!()}

// pub fn SegmentLimit(a: u32) -> u32;
pub fn SegmentLimit(a: SEGPTR) -> SEGPTR {todo!()}
