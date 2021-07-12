#![allow(non_snake_case)]

use crate::win_struct::{ATOM, BITMAPINFO, COLORREF, CONTEXT, HACCEL16, HANDLE16, HBRUSH16, HCURSOR16, HDC16, HFILE16, HGDIOBJ16, HGLOBAL16, HICON16, HINSTANCE16, HMENU16, HMODULE16, HPALETTE16, HPEN16, HRSRC16, HTASK16, HWND16, LOGPALETTE, LPARAM, LRESULT, MSG16, POINT16, RECT16, SEGPTR, WINDOWPLACEMENT16, WNDCLASS16, WPARAM16, PAINTSTRUCT16};
use std::hint::unreachable_unchecked;

// void FatalExit(void)
pub fn FatalExit() {
    unimplemented!()
}
// DWORD GetVersion16(void)
pub fn GetVersion16() -> u32 {
    unimplemented!()
}
// HGLOBAL16 GLobalAlloc16(Uflags: i16, DWORD size)
pub fn GLobalAlloc16(flags: u16, size: u32) -> HGLOBAL16 {
    unimplemented!()
}
// HGLOBAL16 GlobalReAlloc16(HGLOBAL16 handle, DWORD size, Uflags: i16)
pub fn GlobalReAlloc16(handle: HGLOBAL16, size: u32, flags: u16) -> HGLOBAL16 {
    unimplemented!()
}
// HGLOBAL16 GlobalFree16(HGLOBAL16 handle)
pub fn GlobalFree16(handle: HGLOBAL16) -> HGLOBAL16 {
    unimplemented!()
}
// WIN16_GlobalLock16: SEGPTR(HGLOBAL16 handle)
pub fn WIN16_GlobalLock16(handle: HGLOBAL16) -> SEGPTR {
    unimplemented!()
}
// BOOL16 GlobalUnlock16(HGLOBAL16 handle)
pub fn GlobalUnlock16(handle: HGLOBAL16) -> bool {
    unimplemented!()
}
// DWORD GlobalSize16(HGLOBAL16 handle)
pub fn GlobalSize16(handle: HGLOBAL16) -> u32 {
    unimplemented!()
}
// DWORD GlobalHandle16(WORD sel)
pub fn GlobalHandle16(sel: u16) -> u32 {
    unimplemented!()
}
// HGLOBAL16 LockSegment16(HGLOBAL16 handle)
pub fn LockSegment16(handle: HGLOBAL16) -> HGLOBAL16 {
    unimplemented!()
}
// BOOL16 WaitEvent16(HTASK16 h_task)
pub fn WaitEvent16(task: HTASK16) -> bool {
    unimplemented!()
}
// GetModuleFileName16: i16(HINSTANCE16 h_module, LPSTR lp_file_name, n_size: i16)
pub fn GetModuleFileName16(h_module: HINSTANCE16, lp_file_name: &String, n_size: i16) -> i16 {
    unimplemented!()
}
// LPVOID MakeProcInstance16(LPVOID func, HANDLE16 h_instance)
pub fn MakeProcInstance16(func: fn(), h_instance: HANDLE16) -> *mut u8 {
    unimplemented!()
}
// void FreeProcInstance16(LPVOID func)
pub fn FreeProcInstance16(func: fn()) {
    unimplemented!()
}
// HRSRC16 FindResource16(HMODULE16 h_module, LPCSTR name, LPCSTR type)
pub fn FindResource16(h_module: HMODULE16, name: &String, rsrc_type: &String) -> HRSRC16 {
    unimplemented!()
}
// HGLOBAL16 LoadResource16(HMODULE16 h_module, HRSRC16 h_rsrc)
pub fn LoadResource16(h_module: HMODULE16, h_rsrc: HRSRC16) -> HGLOBAL16 {
    unimplemented!()
}
// WIN16_LockResource16: SEGPTR(HGLOBAL16 handle)
pub fn WIN16_LockResource16(handle: HGLOBAL16) -> SEGPTR {
    unimplemented!()
}
// BOOL16 FreeResource16(HGLOBAL16 handle)
pub fn FreeResource16(handle: HGLOBAL16) -> bool {
    unimplemented!()
}
// HFILE16 _lclose16(HFILE16 h_file)
pub fn _lclose16(h_file: HFILE16) -> HFILE16 {
    unimplemented!()
}
// HFILE16 _lcreat16(LPCSTR path, attr: i16)
pub fn _lcreat16(path: &String, attr: i16) -> HFILE16 {
    unimplemented!()
}
// long _llseek16(HFILE16 h_file, long l_offset, n_origin: i16)
pub fn _llseek16(h_file: HFILE16, l_offset: libc::c_long, n_origin: i16) -> libc::c_long {
    unimplemented!()
}
// HFILE16 _lopen16(LPCSTR path, mode: i16)
pub fn _lopen16(path: &String, mode: i16) -> HFILE16 {
    unimplemented!()
}
// lstrlen16: i16(LPCSTR str)
pub fn lstrlen16(a: &String) -> i16 {
    unimplemented!()
}
// void InitTask16(CONTEXT * context)
pub fn InitTask16(context: &mut CONTEXT) {
    unimplemented!()
}
// void DOS3Call(CONTEXT * context)
pub fn DOS3Call(context: &mut CONTEXT) {
    unimplemented!()
}
// USetErrorMode16: i16(Umode: i16)
pub fn SetErrorMode16(mode: u16) -> u16 {
    unimplemented!()
}
// void __AHSHIFT(void)
pub fn __AHSHIFT() {
    unimplemented!()
}
// void __AHINCR(void)
pub fn __AHINCR() {
    unimplemented!()
}
// void OutputDebugString16(LPCSTR str)
pub fn OutputDebugString16(str: &mut String) {
    unimplemented!()
}
// GetPrivateProfileString16: i16(LPCSTR section, LPCSTR entry, LPCSTR def_val, LPSTR buffer, Ulen: i16, LPCSTR filename)
pub fn GetPrivateProfileString16(
    section: &String,
    entry: &String,
    def_val: &String,
    buffer: &mut String,
    len: usize,
    filename: &String,
) -> i16 {
    unimplemented!()
}
// BOOL16 WritePrivateProfileString16(LPCSTR section, LPCSTR entry, LPCSTR string, LPCSTR filename)
pub fn WritePrivateProfileString16(
    section: &String,
    entry: &String,
    str_to_write: &String,
    filename: &String,
) -> bool {
    unimplemented!()
}
// GetDOSEnvironment16: SEGPTR(void)
pub fn GetDOSEnvironment16() -> SEGPTR {
    unimplemented!()
}
// void FatalAppExit16(Uaction: i16, LPCSTR str)
pub fn FatalAppExit16(action: u16, reason: &str) {
    unimplemented!()
}
// HINSTANCE16 WinExec16(LPCSTR lp_cmd_line, Un_cmd_show: i16)
pub fn WinExec16(lp_cmd_line: &String, n_cmd_show: u16) -> HINSTANCE16 {
    unimplemented!()
}
// void __WINFLAGS(void)
pub fn __WINFLAGS() {
    unimplemented!()
}
// DWORD GlobalDOSAlloc16(DWORD size)
pub fn GlobalDOSAlloc16(size: usize) -> usize {
    unimplemented!()
}
// WORD GlobalDOSFree16(WORD sel)
pub fn GlobalDOSFree16(sel: u16) -> u16 {
    unimplemented!()
}
// WORD GlobalPageLock16(HGLOBAL16 handle)
pub fn GlobalPageLock16(handle: HGLOBAL16) -> u16 {
    unimplemented!()
}
// WORD GlobalPageUnlock16(HGLOBAL16 handle)
pub fn GlobalPageUnlock16(handle: HGLOBAL16) -> u16 {
    unimplemented!()
}
// void hmemcpy16(LPVOID dst, LPCVOID src, long count)
pub fn hmemcpy16(dst: *mut u8, src: *mut u8, count: libc::c_long) {
    unimplemented!()
}
// long WIN16_hread(HFILE16 h_file, buffer: SEGPTR, long count)
pub fn WIN16_hread(h_file: HFILE16, buffer: &mut Vec<u8>, count: usize) -> usize {
    unimplemented!()
}
// long _hwrite16(HFILE16 h_file, LPCSTR buffer, long count)
pub fn _hwrite16(h_file: HFILE16, buffer: &mut String, count: usize) -> usize {
    unimplemented!()
}
// COLORREF SetBkColor16(HDC16 hdc, COLORREF color)
pub fn SetBkColor16(hdc: HDC16, color: COLORREF) -> COLORREF {
    unimplemented!()
}
// int16_t SetMapMode16(HDC16 hdc, int16_t mode)
pub fn SetMapMode16(hdc: HDC16, mode: i16) -> i16 {
    unimplemented!()
}
// COLORREF SetTextColor16(HDC16 hdc, COLORREF color)
pub fn SetTextColor16(hdc: HDC16, color: COLORREF) -> COLORREF {
    unimplemented!()
}
// BOOL16 LineTo16(HDC16 hdc, x: i16, y: i16)
pub fn LineTo16(hdc: HDC16, x: i16, y: i16) -> bool {
    unimplemented!()
}
// DWORD MoveTo16(HDC16 hdc, x: i16, y: i16)
pub fn MoveTo16(hdc: HDC16, x: i16, y: i16) -> u32 {
    unimplemented!()
}
// BOOL16 Ellipse16(HDC16 hdc, left: i16, top: i16, right: i16, bottom: i16)
pub fn Ellipse16(hdc: HDC16, left: i16, top: i16, right: i16, bottom: i16) -> bool {
    unimplemented!()
}
// BOOL16 Rectangle16(HDC16 hdc, left: i16, top: i16, right: i16, bottom: i16)
pub fn Rectangle16(hdc: HDC16, left: i16, top: i16, right: i16, bottom: i16) -> bool {
    unimplemented!()
}
// BOOL16 TextOut16(HDC16 hdc, x: i16, y: i16, char * str, count: i16)
pub fn TextOut16(hdc: HDC16, x: i16, y: i16, out_text: &String, count: usize) -> bool {
    unimplemented!()
}
// BOOL16 Polygon16(HDC16 hdc, POINT16 * pt, count: i16)
pub fn Polygon16(hdc: HDC16, pt: &[POINT16], count: i16) -> bool {
    unimplemented!()
}
// HGDIOBJ16 SelectObject16(HDC16 hdc, HGDIOBJ16 handle)
pub fn SelectObject16(hdc: HDC16, handle: HGDIOBJ16) -> HGDIOBJ16 {
    unimplemented!()
}
// HDC16 CreateDC16(LPCSTR driver, LPCSTR device, LPCSTR output, DEVMODEA * init_data)
pub fn CreateDC16(
    driver: &String,
    device: &String,
    output: &String,
    init_data: &_devicemodeA,
) -> HDC16 {
    unimplemented!()
}
// HPEN16 CreatePen16(style: i16, width: i16, COLORREF color)
pub fn CreatePen16(style: i16, width: i16, color: COLORREF) -> HPEN16 {
    unimplemented!()
}
// HBRUSH16 CreateSolidBrush16(COLORREF color)
pub fn CreateSolidBrush16(color: COLORREF) -> HBRUSH16 {
    unimplemented!()
}
// BOOL16 DeleteDC16(HDC16 hdc)
pub fn DeleteDC16(hdc: HDC16) -> bool {
    unimplemented!()
}
// BOOL16 DeleteObject16(HGDIOBJ16 obj)
pub fn DeleteObject16(obj: HGDIOBJ16) -> bool {
    unimplemented!()
}
// DWORD GetCurrentPosition16(HDC16 hdc)
pub fn GetCurrentPosition16(hdc: HDC16) -> u32 {
    unimplemented!()
}
// GetDeviceCaps16: i16(HDC16 hdc, cap: i16)
pub fn GetDeviceCaps16(hdc: HDC16, cap: i16) -> i16 {
    unimplemented!()
}
// HGDIOBJ16 GetStockObject16(obj: i16)
pub fn GetStockObject16(obj: i16) -> HGDIOBJ16 {
    unimplemented!()
}
// DWORD GetTextExtent16(HDC16 hdc, LPCSTR str, count: i16)
pub fn GetTextExtent16(hdc: HDC16, text: &String, count: usize) -> u32 {
    unimplemented!()
}
// BOOL16 UnrealizeObject16(HGDIOBJ16 obj)
pub fn UnrealizeObject16(obj: HGDIOBJ16) -> bool {
    unimplemented!()
}
// HPALETTE16 CreatePalette16(LOGPALETTE * palette)
pub fn CreatePalette16(palette: &mut LOGPALETTE) -> HPALETTE16 {
    unimplemented!()
}
// UGetSystemPaletteEntries: i16(HDC16 hdc, Ustart: i16, Ucount: i16, PALETTEENTRY * entries)
pub fn GetSystemPaletteEntries(
    hdc: HDC16,
    start: u16,
    count: u16,
    entries: &[PALETTEENTRY],
) -> u16 {
}
// StretchDIBits16: i16(HDC16 hdc, x_dst: i16, y_dst: i16, width_dst: i16, height_dst: i16, x_src: i16, y_src: i16, width_src: i16, height_src: i16, PVOID bits, BITMAPINFO * info, Uw_usage: i16, DWORD dw_rop)
pub fn StretchDIBits16(
    hdc: HDC16,
    x_dst: i16,
    y_dst: i16,
    width_dst: i16,
    height_dst: i16,
    x_src: i16,
    width_src: i16,
    height_src: i16,
    bits: *mut u8,
    info: &BITMAPINFO,
    w_usage: u16,
    dw_rop: u32,
) -> i16 {
    unimplemented!()
}
// SetDIBitsToDevice: i16(HDC16 hdc, x_dest: i16, y_dest: i16, cx: i16, cy: i16, x_src: i16, y_src: i16, Ustartscan: i16, Ulines: i16, LPCVOID bits, BITMAPINFO * info, Ucoloruse: i16)
pub fn SetDIBitsToDevice(
    hdc: HDC16,
    x_dest: i16,
    y_dest: i16,
    cx: i16,
    cy: i16,
    x_src: i16,
    y_src: i16,
    startscan: u16,
    lines: u16,
    bits: *mut u8,
    info: &BITMAPINFO,
    coloruse: u16,
) -> i16 {
    unimplemented!()
}
// BOOL16 MoveToEx16(HDC16 hdc, x: i16, y: i16, POINT16 * pt)
pub fn MoveToEx16(hdc: HDC16, x: i16, y: i16, pt: &POINT16) -> bool {
    unimplemented!()
}
// MessageBox16: i16(HWND16 hwnd, LPCSTR text, LPCSTR title, Utype: i16)
pub fn MessageBox16(hwnd: HWND16, text: &String, title: &String, box_type: u16) -> i16 {
    unimplemented!()
}
// InitApp16: i16(HINSTANCE16 h_instance)
pub fn InitApp16(h_inst: HINSTANCE16) -> i16 {
    unimplemented!()
}
// void PostQuitMessage16(exit_code: i16)
pub fn PostQuitMessage16(exit_code: i16) {
    unimplemented!()
}
// USetTimer16: i16(HWND16 hwnd, Uid: i16, Utimeout: i16, LPVOID proc)
pub fn SetTimer16(hwnd: HWND16, id: u16, timeout: u16, proc: u32) -> u16 {
    unimplemented!()
}
// BOOL16 KillTimer16(HWND16 hwnd, Uid: i16)
pub fn KillTimer16(hwnd: HWND16, id: u16) -> bool {
    unimplemented!()
}
// BOOL16 GetCursorPos16(POINT16 * pt)
pub fn GetCursorPos16(pt: &POINT16) -> bool {
    unimplemented!()
}
// HWND16 SetCapture16(HWND16 hwnd)
pub fn SetCapture16(hwnd: HWND16) -> HWND16 {
    unimplemented!()
}
// BOOL16 ReleaseCapture16(void)
pub fn ReleaseCapture16() -> bool {
    unimplemented!()
}
// HWND16 SetFocus16(HWND16 hwnd)
pub fn SetFocus16(hwnd: HWND16) -> HWND16 {
    unimplemented!()
}
// HANDLE16 RemoveProp16(HWND16 hwnd, LPCSTR str)
pub fn RemoveProp16(hwnd: HWND16, text: &String) -> HANDLE16 {
    unimplemented!()
}
// HANDLE16 GetProp16(HWND16 hwnd, LPCSTR str)
pub fn GetProp16(hwnd: HWND16, text: &String) -> HANDLE16 {
    unimplemented!()
}
// BOOL16 SetProp16(HWND16 hwnd, LPCSTR str, HANDLE16 handle)
pub fn SetProp16(hwnd: HWND16, text: &String, handle: HANDLE16) -> bool {
    unimplemented!()
}
// void ClientToScreen16(HWND16 hwnd, POINT16 * lppnt)
pub fn ClientToScreen16(hwnd: HWND16, lppnt: &POINT16) {
    unimplemented!()
}
// void ScreenToClient16(HWND16 hwnd, POINT16 * lppnt)
pub fn ScreenToClient16(hwnd: HWND16, lppnt: &POINT16) {
    unimplemented!()
}
// BOOL16 IsIconic16(HWND16 hwnd)
pub fn IsIconic16(hwnd: HWND16) -> bool {
    unimplemented!()
}
// void GetWindowRect16(HWND16 hwnd, RECT16 * rect)
pub fn GetWindowRect16(hwnd: HWND16, rect: &mut RECT16) {
    unimplemented!()
}
// void GetClientRect16(HWND16 hwnd, RECT16 * rect)
pub fn GetClientRect16(hwnd: HWND16, rect: &mut RECT16) {
    unimplemented!()
}
// BOOL16 EnableWindow16(HWND16 hwnd, BOOL16 enable)
pub fn EnableWindow16(hwnd: HWND16, enable: bool) -> bool {
    unimplemented!()
}
// BOOL16 IsWindowEnabled16(HWND16 hwnd)
pub fn IsWindowEnabled16(hwnd: HWND16) -> bool {
    unimplemented!()
}
// GetWindowText16: i16(HWND16 hwnd, lp_string: SEGPTR, n_max_count: i16)
pub fn GetWindowText16(hwnd: HWND16, lp_string: &SEGPTR, n_max_count: i16) -> i16 {
    unimplemented!()
}
// BOOL16 SetWindowText16(HWND16 hwnd, lp_string: SEGPTR)
pub fn SetWindowText16(hwnd: HWND16, lp_string: &SEGPTR) -> bool {
    unimplemented!()
}
// HDC16 BeginPaint16(HWND16 hwnd, PAINTSTRUCT16 * lps)
pub fn BeginPaint16(hwnd: HWND16, lps: &mut PAINTSTRUCT16) -> HDC16 {
    unimplemented!()
}
// BOOL16 EndPaint16(HWND16 hwnd, PAINTSTRUCT16 * lps)
pub fn EndPaint16(hwnd: HWND16, lps: &mut PAINTSTRUCT16) -> bool {
    unimplemented!()
}
// HWND16 CreateWindow16(LPCSTR class_name, LPCSTR window_name, DWORD style, x: i16, y: i16, width: i16, height: i16, HWND16 parent, HMENU16 hmenu, HINSTANCE16 instance, LPVOID data)
pub fn CreateWindow16(
    class_name: &String,
    window_name: &String,
    style: u32,
    x: i16,
    y: i16,
    width: i16,
    height: i16,
    parent: HWND16,
    hmenu: HMENU16,
    instance: HINSTANCE16,
    data: *mut u8,
) -> HWND16 {
    unimplemented!()
}
// BOOL16 ShowWindow16(HWND16 hwnd, cmd: i16)
pub fn ShowWindow16(hwnd: HWND16, cmd: i16) -> bool {
    unimplemented!()
}
// BOOL16 BringWindowToTop16(HWND16 hwnd)
pub fn BringWindowToTop16(hwnd: HWND16) -> bool {
    unimplemented!()
}
// BOOL16 IsWindow16(HWND16 hwnd)
pub fn IsWindow16(hwnd: HWND16) -> bool {
    unimplemented!()
}
// BOOL16 DestroyWindow16(HWND16 hwnd)
pub fn DestroyWindow16(hwnd: HWND16) -> bool {}
// BOOL16 EnumChildWindows1(HWND16 parent, LPVOID func, LPARAM lparam)
pub fn EnumChildWindows1(parent: HWND16, func: fn(), lparam: LPARAM) -> bool {
    unimplemented!()
}
// BOOL16 MoveWindow16(HWND16 hwnd, x: i16, y: i16, cx: i16, cy: i16, BOOL16 repaint)
pub fn MoveWindow16(hwnd: HWND16, x: i16, y: i16, cx: i16, cy: i16, repaint: bool) -> bool {
    unimplemented!()
}
// ATOM RegisterClass16(WNDCLASS16 * wc)
pub fn RegisterClass16(wc: &WNDCLASS16) -> ATOM {
    unimplemented!()
}
// HDC16 GetDC16(HWND16 hwnd)
pub fn GetDC16(hwnd: HWND16) -> HDC16 {
    unimplemented!()
}
// HDC16 GetWindowDC16(HWND16 hwnd)
pub fn GetWindowDC16(hwnd: HWND16) -> HDC16 {
    unimplemented!()
}
// ReleaseDC16: i16(HWND16 hwnd, HDC16 hdc)
pub fn ReleaseDC16(hwnd: HWND16, hdc: HDC16) -> i16 {
    unimplemented!()
}
// HCURSOR16 SetCursor16(HCURSOR16 hcursor)
pub fn SetCursor16(hcursor: HCURSOR16) -> HCURSOR16 {
    unimplemented!()
}
// ShowCursor16: i16(BOOL16 b_show)
pub fn ShowCursor16(b_show: bool) -> i16 {
    unimplemented!()
}
// BOOL16 PtInRect16(RECT16 * rect, POpt: i16)
pub fn PtInRect16(rect: &mut RECT16, pt: POINT16) -> bool {
    unimplemented!()
}
// FillRect16: i16(HDC16 hdc, RECT16 * rect, HBRUSH16 hbrush)
pub fn FillRect16(hdc: HDC16, rect: &RECT16, hbrush: HBRUSH16) -> i16 {
    unimplemented!()
}
// FrameRect16: i16(HDC16 hdc, RECT16 * rect, HBRUSH16 hbrush)
pub fn FrameRect16(hdc: HDC16, rect: &RECT16, hbrush: HBRUSH16) -> i16 {
    unimplemented!()
}
// BOOL16 DrawIcon16(HDC16 hdc, x: i16, y: i16, HICON16 h_icon)
pub fn DrawIcon16(hdc: HDC16, x: i16, y: i16, h_icon: HICON16) -> bool {
    unimplemented!()
}
// DrawText16: i16(HDC16 hdc, LPCSTR str, count: i16, RECT16 * rect, Uflags: i16)
pub fn DrawText16(hdc: HDC16, text: &String, count: i16, rect: &RECT16, flags: u16) -> i16 {
    unimplemented!()
}
// HWND16 CreateDialog16(HINSTANCE16 hinst, LPCSTR dlg_template, HWND16 owner, LPVOID dlg_proc)
pub fn CreateDialog16(
    h_inst: HINSTANCE16,
    dlg_template: &String,
    owner: HWND16,
    dlg_proc: fn(),
) -> HWND16 {
    unimplemented!()
}
// BOOL16 IsDialogMessage16(HWND16 hwnd_dlg, MSG16 * msg16)
pub fn IsDialogMessage16(hwnd_dlg: HWND16, msg: &mut MSG16) -> bool {
    unimplemented!()
}
// HWND16 GetDlgItem16(HWND16 hwnd_dlg, id: i16)
pub fn GetDlgItem16(hwnd_dlg: HWND16, id: i16) -> HWND16 {
    unimplemented!()
}
// void SetDlgItemText16(HWND16 hwnd, id: i16, lp_string: SEGPTR)
pub fn SetDlgItemText16(hwnd: HWND16, id: i16, lp_string: &SEGPTR) {
    unimplemented!()
}
// void SetDlgItemInt16(HWND16 hwnd, id: i16, Uvalue: i16, BOOL16 f_signed)
pub fn SetDlgItemInt16(hwnd: HWND16, id: i16, value: u16, f_signed: bool) {
    unimplemented!()
}
// UGetDlgItemInt16: i16(HWND16 hwnd, id: i16, BOOL16 * translated, BOOL16 f_signed)
pub fn GetDlgItemInt16(hwnd: HWND16, id: i16, translated: &mut bool, f_signed: bool) -> u16 {
    unimplemented!()
}
// BOOL16 CheckRadioButton16(HWND16 hwnd_dlg, Ufirst_id: i16, Ulast_id: i16, Ucheck_id: i16)
pub fn CheckRadioButton16(hwnd_dlg: HWND16, first_id: u16, last_id: u16, check_id: u16) -> bool {
    unimplemented!()
}
// BOOL16 CheckDlgButton16(HWND16 hwnd, id: i16, Ucheck: i16)
pub fn CheckDlgButton16(hwnd: HWND16, id: i16, check: u16) -> bool {
    unimplemented!()
}
// UIsDlgButtonChecked: i16(HWND16 hwnd, Uid: i16)
pub fn IsDlgButtonChecked(hwnd: HWND16, id: u16) -> u16 {
    unimplemented!()
}
// LRESULT SendDlgItemMessage16(HWND16 hwnd, id: i16, Umsg: i16, WPARAM16 w_param, LPARAM l_param)
pub fn SendDlgItemMessage16(
    hwnd: HWND16,
    id: i16,
    msg: u16,
    w_param: WPARAM16,
    lparam: LPARAM,
) -> LRESULT {
    unimplemented!()
}
// void MapDialogRect16(HWND16 hwnd, RECT16 * rect)
pub fn MapDialogRect16(hwnd: HWND16, rect: &RECT16) {
    unimplemented!()
}
// void MessageBeep16(Ui: i16)
pub fn MessageBeep16(i: u16) {
    unimplemented!()
}
// LRESULT DefWindowProc16(HWND16 hwnd, Umsg: i16, WPARAM16 wparam, LPARAM lparam)
pub fn DefWindowProc16(hwnd: HWND16, msg: u16, wparam: WPARAM16, lparam: LPARAM) -> LRESULT {
    unimplemented!()
}
// BOOL16 GetMessage16(MSG16 * msg, HWND16 hwnd, Ufirst: i16, Ulast: i16)
pub fn GetMessage16(msg: &mut MSG16, hwnd: HWND16, first: u16, last: u16) -> bool {
    unimplemented!()
}
// BOOL16 PostMessage16(HWND16 hwnd, Umsg: i16, WPARAM16 wparam, LPARAM lparam)
pub fn PostMessage16(hwnd: HWND16, msg: u16, wparam: WPARAM16, lparam: LPARAM) -> bool {
    unimplemented!()
}
// LRESULT SendMessage16(HWND16 hwnd, Umsg: i16, WPARAM16 wparam, LPARAM lparam)
pub fn SendMessage16(hwnd: HWND16, msg: u16, wparam: WPARAM16, lparam: LPARAM) -> LRESULT {
    unimplemented!()
}
// BOOL16 TranslateMessage16(MSG16 * msg)
pub fn TranslateMessage16(msg: &mut MSG16) -> bool {
    unimplemented!()
}

// long DispatchMessage16(MSG16 * msg)
pub fn DispatchMessage16(msg: &mut MSG16) -> usize {
    unimplemented!()
}
// LRESULT CallWindowProc16(LPVOID func, HWND16 hwnd, Umsg: i16, WPARAM16 wparam, LPARAM lparam)
pub fn CallWindowProc16(
    func: fn(),
    hwnd: HWND16,
    msg: u16,
    wparam: WPARAM16,
    lparam: LPARAM,
) -> LRESULT {
    unimplemented!()
}
// void UpdateWindow16(HWND16 hwnd)
pub fn UpdateWindow16(hwnd: HWND16) {
    unimplemented!()
}
// void InvalidateRect16(HWND16 hwnd, RECT16 * rect, BOOL16 erase)
pub fn InvalidateRect16(hwnd: HWND16, rect: &RECT16, erase: bool) {
    unimplemented!()
}
// void ValidateRect16(HWND16 hwnd, RECT16 * rect)
pub fn ValidateRect16(hwnd: HWND16, rect: &RECT16) {
    unimplemented!()
}
// WORD GetWindowWord16(HWND16 hwnd, offset: i16)
pub fn GetWindowWord16(hwnd: HWND16, offset: i16) -> i16 {
    unimplemented!()
}
// WORD SetWindowWord16(HWND16 hwnd, offset: i16, WORD newval)
pub fn SetWindowWord16(hwnd: HWND16, offset: i16, newval: i16) -> i16 {
    unimplemented!()
}
// long GetWindowLong16(HWND16 hwnd, offset: i16)
pub fn GetWindowLong16(hwnd: HWND16, offset: i16) -> libc::c_long {
    unimplemented!()
}
// long SetWindowLong16(HWND16 hwnd, offset: i16, long newval)
pub fn SetWindowLong16(hwnd: HWND16, offset: i16, newval: libc::c_long) -> libc::c_long {
    unimplemented!()
}
// HMENU16 LoadMenu16(HINSTANCE16 instance, LPCSTR name)
pub fn LoadMenu16(instance: HINSTANCE16, name: &String) -> HMENU16 {
    unimplemented!()
}
// BOOL16 DestroyMenu16(HMENU16 menu)
pub fn DestroyMenu16(menu: HMENU16) -> bool {
    unimplemented!()
}
// BOOL16 CheckMenuItem16(HMENU16 hmenu, Uw_item_id: i16, Uw_flags: i16)
pub fn CheckMenuItem16(hmenu: HMENU16, w_item_id: u16, w_flags: u16) -> bool {
    unimplemented!()
}
// BOOL16 EnableMenuItem16(HMENU16 hmenu, Uw_item_id: i16, Uw_flags: i16)
pub fn EnableMenuItem16(hmenu: HMENU16, w_item_id: u16, w_flags: u16) -> bool {
    unimplemented!()
}
// HMENU16 GetSubMenu16(HMENU16 h_menu, n_pos: i16)
pub fn GetSubMenu16(h_menu: HMENU16, n_pos: i16) -> HMENU16 {
    unimplemented!()
}
// BOOL16 WinHelp16(HWND16 hwnd, LPCSTR lp_help_file, Uw_command: i16, DWORD dw_data)
pub fn WinHelp16(hwnd: HWND16, lp_help_file: &String, w_command: i16, dw_data: u32) -> bool {
    unimplemented!()
}
// HCURSOR16 LoadCursor16(HINSTANCE16 h_instance, LPCSTR name)
pub fn LoadCursor16(h_instance: HINSTANCE16, name: &String) -> HCURSOR16 {
    unimplemented!()
}
// HICON16 LoadIcon16(HINSTANCE16 h_instance, LPCSTR name)
pub fn LoadIcon16(h_instance: HINSTANCE16, name: &String) -> HICON16 {
    unimplemented!()
}
// LoadString16: i16(HINSTANCE16 instance, Uresource_id: i16, LPSTR buffer, buf_len: i16)
pub fn LoadString16(
    instance: HINSTANCE16,
    resource_id: u16,
    buffer: &mut String,
    buf_len: u16,
) -> i16 {
    unimplemented!()
}
// HACCEL16 LoadAccelerators16(HINSTANCE16 instance, LPCSTR lp_table_name)
pub fn LoadAccelerators16(instance: HINSTANCE16, lp_table_name: &String) -> HACCEL16 {
    unimplemented!()
}
// TranslateAccelerator16: i16(HWND16 hwnd, HACCEL16 haccel, MSG16 * msg)
pub fn TranslateAccelerator16(hwnd: HWND16, haccel: HACCEL16, msg: &MSG) -> i16 {
    unimplemented!()
}
// GetSystemMetrics16: i16(index: i16)
pub fn GetSystemMetrics16(index: i16) -> i16 {
    unimplemented!()
}
// COLORREF GetSysColor16(index: i16)
pub fn GetSysColor16(index: i16) -> COLORREF {
    unimplemented!()
}
// void SetSysColors16(count: i16, INT16 * list, COLORREF * values)
pub fn SetSysColors16(count: i16, list: &[i16], values: &[COLORREF]) {
    unimplemented!()
}
// BOOL16 GrayString16(HDC16 hdc, HBRUSH16 param_2, LPVOID gsprc, LPARAM lparam, cch: i16, x: i16, y: i16, cx: i16, cy: i16)
pub fn GrayString16(
    hdc: HDC16,
    param_2: HBRUSH16,
    gsprc: *mut u8,
    lparam: LPARAM,
    cch: i16,
    x: i16,
    y: i16,
    cx: i16,
    cy: i16,
) -> bool {
    unimplemented!()
}
// HWND16 SetSysModalWindow(HWND16 hwnd)
pub fn SetSysModalWindow(hwnd: HWND16) -> HWND16 {
    unimplemented!()
}
// HWND16 GetNextDlgTabItem16(HWND16 hwnd_dlg, HWND16 hwnd_ctrl, BOOL16 f_previous)
pub fn GetNextDlgTabItem16(hwnd_dlg: HWND16, hwnd_ctrl: HWND16, f_previous: bool) -> HWND16 {
    unimplemented!()
}
// BOOL16 SetWindowPos16(HWND16 hwnd, HWND16 hwnd_insert_after, x: i16, y: i16, cx: i16, cy: i16, WORD flags)
pub fn SetWindowPos16(
    hwnd: HWND16,
    hwnd_insert_after: HWND16,
    x: i16,
    y: i16,
    cx: i16,
    cy: i16,
    flags: u16,
) -> bool {
    unimplemented!()
}
// UGetMenuState16: i16(HMENU16 hmenu, Uw_item_id: i16, Uw_flags: i16)
pub fn GetMenuState16(hmenu: HMENU16, w_item_id: u16, w_flags: u16) -> u16 {
    unimplemented!()
}
// GetDlgCtrlID16: i16(HWND16 hwnd)
pub fn GetDlgCtrlID16(hwnd: HWND16) -> i16 {
    unimplemented!()
}
// HPALETTE16 SelectPalette16(HDC16 hdc, HPALETTE16 hpal, BOOL16 b_force_background)
pub fn SelectPalette16(hdc: HDC16, hpal: HPALETTE16, b_force_background: bool) -> HPALETTE16 {
    unimplemented!()
}
// URealizePalette16: i16(HDC16 hdc)
pub fn RealizePalette16(hdc: HDC16) -> u16 {
    unimplemented!()
}
// BOOL16 GetWindowPlacement16(HWND16 hwnd, WINDOWPLACEMENT16 * wp16)
pub fn GetWindowPlacement16(hwnd: HWND16, wp16: &WINDOWPLACEMENT16) -> bool {
    unimplemented!()
}
// BOOL16 SetWindowPlacement16(HWND16 hwnd, WINDOWPLACEMENT16 * wp16)
pub fn SetWindowPlacement16(hwnd: HWND16, wp16: &WINDOWPLACEMENT16) -> bool {
    unimplemented!()
}
// BOOL16 GetClassInfo16(HINSTANCE16 h_inst16, name: SEGPTR, WNDCLASS16 * wc)
pub fn GetClassInfo16(h_inst: HINSTANCE16, name: &String, wc: &WNDCLASS16) -> bool {
    unimplemented!()
}
// BOOL16 InsertMenu16(HMENU16 hmenu, Upos: i16, Uflags: i16, Uid: i16, data: SEGPTR)
pub fn InsertMenu16(hmenu: HMENU16, pos: u16, flags: u16, id: u16, data: SEGPTR) -> bool {
    unimplemented!()
}
// BOOL16 DeleteMenu16(HMENU16 hmenu, Unpos: i16, Uwflags: i16)
pub fn DeleteMenu16(hmenu: HMENU16, npos: u16, w_flags: u16) -> bool {
    unimplemented!()
}
// BOOL16 ModifyMenu16(HMENU16 hmenu, Upos: i16, Uflags: i16, Uid: i16, data: SEGPTR)
pub fn ModifyMenu16(hmenu: HMENU16, pos: u16, flags: u16, id: u16, data: SEGPTR) -> bool {
    unimplemented!()
}
// BOOL16 TrackPopupMenu16(HMENU16 hmenu, Uwflags: i16, x: i16, y: i16, n_reserved: i16, HWND16 hwnd, RECT16 * lp_rect)
pub fn TrackPopupMenu16(
    hmenu: HMENU16,
    w_flags: u16,
    x: i16,
    y: i16,
    n_reserved: i16,
    hwnd: HWND16,
    lp_rect: &RECT16,
) -> bool {
    unimplemented!()
}
// wsprintf16: i16(LPSTR buffer, LPCSTR spec, WORD * valist)
pub fn wsprintf16(buffer: &String, spec: &String, valist: *mut u16) -> i16 {
    unimplemented!()
}
// wvsprintf16: i16(LPSTR buffer, LPCSTR spec, WORD * args)
pub fn wvsprintf16(buffer: &mut String, spec: &mut LPCSTR, args: *mut ushort) -> i16 {
    unimplemented!()
}
// HWND16 CreateWIndowEx16(DWORD ex_style, LPCSTR class_name, LPCSTR window_name, DWORD style, x: i16, y: i16, width: i16, height: i16, HWND16 parent, HMENU16 hmenu, HINSTANCE16 instance, LPVOID data)
pub fn CreateWIndowEx16(
    ex_style: u32,
    class_name: &String,
    window_name: &String,
    style: u32,
    x: i16,
    y: i16,
    width: i16,
    height: i16,
    parent: HWND16,
    hmenu: HMENU16,
    instance: HINSTANCE16,
    data: *mut u8,
) -> HWND16 {
    unimplemented!()
}
// BOOL16 DestroyIcon16(HICON16 h_icon)
pub fn DestroyIcon16(h_icon: HICON16) -> bool {
    unimplemented!()
}
// BOOL16 DestroyCursor16(HCURSOR16 h_cursor)
pub fn DestroyCursor16(h_cursor: HCURSOR16) -> bool {
    unimplemented!()
}
// DWORD mciSendCommand16(Uw_dev_id: i16, Uw_msg: i16, DWORD dw_param1, DWORD p2)
pub fn mciSendCommand16(w_dev_id: u16, w_msg: u16, dw_parm1: u32, p2: u32) -> u32 {
    unimplemented!()
}
// BOOL16 mciGetErrorString16(DWORD w_error, LPSTR lp_str_buffer, Uu_length: i16)
pub fn mciGetErrorString16(w_error: u32, lp_str_buffer: &mut String, u_length: u16) -> bool {
    unimplemented!()
}
// BOOL16 GetOpenFileName16(ofn: SEGPTR)
pub fn GetOpenFileName16(ofn: SEGPTR) -> bool {
    unimplemented!()
}
// BOOL16 GetSaveFileName16(ofn: SEGPTR)
pub fn GetSaveFileName16(ofn: SEGPTR) -> bool {
    unimplemented!()
}

pub fn swi(a: u16) -> u32 {
    unimplemented!()
}

pub fn SegmentLimit(param_1: u16) -> u16 {
    unimplemented!()
}
