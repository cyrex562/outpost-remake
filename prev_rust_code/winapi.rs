use crate::sys_structs::{LOGPALETTE, PAINTSTRUCT16, POINT16, RECT16, WNDCLASS16, MSG16};
use crate::typedefs::{ATOM, COLORREF, HANDLE16, HBRUSH16, HCURSOR16, HDC16, HFILE16, HGDIOBJ16, HGLOBAL16, HICON16, HINSTANCE16, HMENU16, HPALETTE16, HPEN16, HTASK16, HWND16, LPARAM, LRESULT, SEGPTR, WPARAM16, HMODULE16, HRSRC16};
use crate::mem_funcs::mem_ops_1::StructuredData;

/*
ATOM WINAPI RegisterClass16( const WNDCLASS16 *wc )
 */
pub fn RegisterClass16(wc: &WNDCLASS16) -> ATOM {
    unimplemented!()
}


/*
BOOL16 WINAPI GrayString16( HDC16 hdc, HBRUSH16 hbr, GRAYSTRINGPROC16 gsprc,
                            LPARAM lParam, INT16 cch, INT16 x, INT16 y,
                            INT16 cx, INT16 cy )
*/

/* BOOL16 WINAPI TrackPopupMenu16( HMENhMenu: u16, UINT16 wFlags, INT16 x, INT16 y,
INT16 nReserved, HWND16 hwnd, const RECT16 *lpRect )
*/

/* BOOL16 WINAPI MoveWindow16( HWND16 hwnd, INT16 x, INT16 y, INT16 cx, INT16 cy, BOOL16 repaint ) */

/* void WINAPI UpdateWindow16( HWND16 hwnd ) */

/* HWND16 WINAPI CreateWindow16( LPCSTR className, LPCSTR windowName,
DWORD style, INT16 x, INT16 y, INT16 width,
INT16 height, HWND16 parent, HMENmenu: u16,
HINSTANCE16 instance, LPVOID data ) */

/* HWND16 WINAPI SetFocus16( HWND16 hwnd ) */

/* BOOL16 WINAPI DrawIcon16(HDC16 hdc, INT16 x, INT16 y, HICON16 hIcon) */

/* BOOL16 WINAPI ReleaseCapture16(void) */

/* HCURSOR16 WINAPI SetCursor16(HCURSOR16 hCursor) */

/* HWND16 WINAPI SetCapture16( HWND16 hwnd ) */

/* BOOL16 WINAPI ModifyMenu16( HMENhMenu: u16, UINT16 pos, UINT16 flags,
UINT16 id, SEGPTR data ) */

/* UINT16 WINAPI GetMenuState16( HMENhMenu: u16, UINT16 wItemID, UINT16 wFlags ) */

/* BOOL16 WINAPI DeleteMenu16( HMENhMenu: u16, UINT16 nPos, UINT16 wFlags ) */

/*
BOOL16 WINAPI InsertMenu16( HMENhMenu: u16, UINT16 pos, UINT16 flags,
                            UINT16 id, SEGPTR data )
*/

/*
BOOL16 WINAPI CheckMenuItem16( HMENhMenu: u16, UINT16 id, UINT16 flags )
*/

/*
BOOL16 WINAPI BringWindowToTop16( HWND16 hwnd )
*/

/*
BOOL16 WINAPI Ellipse16( HDC16 hdc, INT16 left, INT16 top,
                         INT16 right, INT16 bottom )
*/

/*
void WINAPI ValidateRect16( HWND16 hwnd, const RECT16 *rect )
*/

/*
BOOL16 WINAPI SetWindowPos16( HWND16 hwnd, HWND16 hwndInsertAfter,
                              INT16 x, INT16 y, INT16 cx, INT16 cy, WORD flags)
*/

/*
BOOL16 WINAPI SetWindowText16( HWND16 hwnd, SEGPTR lpString )
*/

/*
FARPROC16 WINAPI MakeProcInstance16( FARPROC16 func, HANDLE16 hInstance )
*/

/*
BOOL16 WINAPI EnumChildWindows16( HWND16 parent, WNDENUMPROC16 func, LPARAM lParam )
*/

/*
void WINAPI FreeProcInstance16( FARPROC16 func )
*/
// pub fn FreeProcInstance16(func: FARPROC16) {
//     unimplemented!()
// }

/*
WORD WINAPI GetWindowWord16( HWND16 hwnd, INT16 offset )
*/

/*
HFILE16 WINAPI _lclose16(HFILE16);
*/

/*
void WINAPI SetDlgItemText16( HWND16 hwnd, INT16 id, SEGPTR lpString )
*/

/*
INT16 WINAPI LoadString16( HINSTANCE16 instance, UINT16 resource_id, LPSTR buffer, INT16 buflen )
*/

/*
INT16 WINAPI MessageBox16( HWND16 hwnd, LPCSTR text, LPCSTR title, UINT16 type )
*/

/*
UINT16 WINAPI SetErrorMode16( UINT16 mode )
*/

/*
void WINAPI InitTask16( CONTEXT *context )
*/

/*
HGLOBAL16 WINAPI LockSegment16( HGLOBAL16 handle )
*/

/*
BOOL16 WINAPI WaitEvent16( HTASK16 hTask )
*/

/*
INT16 WINAPI InitApp16( HINSTANCE16 hInstance )
*/

/*
DWORD WINAPI GetVersion16(void)
*/

/*
void WINAPI FatalAppExit16( UINT16 action, LPCSTR str )
*/

/*
void WINAPI FatalExit(int ExitCode)
*/

/*
HFILE16 WINAPI _lopen16( LPCSTR path, INT16 mode )
*/

/*
LONG WINAPI _hread( HFILE hFile, LPVOID buffer, LONG count)
*/

/*
LONG WINAPI _llseek16( HFILE16 hFile, LONG lOffset, INT16 nOrigin )
*/

/*
HFILE16 WINAPI _lcreat16( LPCSTR path, INT16 attr )
*/

/*
LONG WINAPI _hwrite16( HFILE16 hFile, LPCSTR buffer, LONG count )
*/


/*
INT16 WINAPI GetModuleFileName16( HINSTANCE16 hModule, LPSTR lpFileName,
                                  INT16 nSize )
 */


pub fn DestroyWindow16(handle: HWND16) -> bool {
    todo!()
}

pub fn IsWindow16(handle: HWND16) -> bool {
    todo!()
}

pub fn DeleteObject16(obj: HGDIOBJ16) -> bool {
    todo!()
}

pub fn DestroyMenu16(hmenu: HMENU16) -> bool {
    todo!()
}

pub fn RemoveProp16(hwnd: HWND16, str: &String) -> HANDLE16 {
    todo!()
}

pub fn GetClientRect16(hwnd: HWND16, rect: &mut RECT16) {
    todo!()
}

pub fn InvalidateRect16(hwnd: HWND16, rect: &mut RECT16, erase: bool) {
    todo!()
}

pub fn GetDlgItem16(hwnd_dlg: HWND16, id: i16) -> HWND16 {
    todo!()
}

pub fn EnableWindow16(hwnd: HWND16, enable: bool) -> bool {
    todo!()
}

pub fn ShowWindow16(hwnd: HWND16, cmd: i16) -> bool {
    todo!()
}

pub fn SendMessage16(hwnd: HWND16, msg: u16, wparam: WPARAM16, lparam: LPARAM) -> LRESULT {
    todo!()
}

pub fn FreeProcInstance16(func: fn() -> LRESULT) {
    todo!()
}

pub fn BeginPaint16(hwnd: HWND16, lps: &PAINTSTRUCT16) -> HDC16 {
    todo!()
}

pub fn BeginPaint(hwnd: HWND16, lps: PAINTSTRUCT16) -> HDC16 {todo!()}

// HPEN16 WINAPI CreatePen16( INT16 style, INT16 width, COLORREF color )
pub fn CreatePen16(style: i16, width: i16, color: COLORREF) -> HPEN16 {
    todo!()
}

// HGDIOBJ16 WINAPI SelectObject16( HDC16 hdc, HGDIOBJ16 handle )
pub fn SelectObject16(hdc: HDC16, handle: HGDIOBJ16) -> HGDIOBJ16 {
    todo!()
}

// HBRUSH16 WINAPI CreateSolidBrush16( COLORREF color )
pub fn CreateSolidBrush16(color: COLORREF) -> HBRUSH16 {
    unimplemented!()
}

// HPALETTE16 WINAPI SelectPalette16( HDC16 hdc, HPALETTE16 hpal, BOOL16 bForceBackground )
pub fn SelectPalette16(hdc: HDC16, hpal: HPALETTE16, bForceBackground: bool) -> HPALETTE16 {
    unimplemented!()
}

// BOOL16 WINAPI EndPaint16( HWND16 hwnd, const PAINTSTRUCT16* lps )
pub fn EndPaint16(hwnd: HWND16, lps: &mut PAINTSTRUCT16) -> bool {
    unimplemented!()
}

// BOOL16 WINAPI Polygon16( HDC16 hdc, const POINT16* pt, INT16 count )
pub fn Polygon16(hdc: &HDC16, pt: &POINT16, count: i16) -> bool {
    unimplemented!()
}

// BOOL16 WINAPI PtInRect16( const RECT16 *rect, POINT16 pt )
pub fn PtInRect16(rect: RECT16, pt: POINT16) -> bool {
    unimplemented!()
}

// HDC16 WINAPI GetDC16( HWND16 hwnd )
pub fn GetDC16(hwnd: HWND16) -> HDC16 {
    unimplemented!()
}

// HDC16 WINAPI CreateDC16( LPCSTR driver, LPCSTR device, LPCSTR output, const DEVMODEA  *initData )
pub fn CreateDC16(driver: &String, device: &String, output: &String, init_data: DEVMODEA) -> HDC16 {
    unimplemented!()
}

// HGDIOBJ16 WINAPI GetStockObject16( INT16 obj )
pub fn GetStockObject16(obj: i16) -> HGDIOBJ16 {
    unimplemented!()
}

// BOOL16 WINAPI Rectangle16( HDC16 hdc, INT16 left, INT16 top,                           INT16 right, INT16 bottom )
pub fn Rectangle16(hdc: HDC16, left: i16, top: i16, right: i16, bottom: i16) -> bool {
    unimplemented!()
}

// BOOL16 WINAPI DeleteDC16( HDC16 hdc )
pub fn DeleteDC16(hdc: HDC16) -> bool {
    unimplemented!()
}

// INT16 WINAPI GetDlgCtrlID16( HWND16 hwnd )
pub fn GetDlgCtrlID16(hwnd: HWND16) -> i16 {
    unimplemented!()
}

// COLORREF WINAPI SetTextColor16( HDC16 hdc, COLORREF color )
pub fn SetTextColor16(hdc: HDC16, color: COLORREF) {
    unimplemented!()
}

// COLORREF WINAPI SetBkColor16( HDC16 hdc, COLORREF color )
pub fn SetBkColor16(hdc: HDC16, color: COLORREF) -> COLORREF {
    unimplemented!()
}

// DWORD WINAPI MoveTo16( HDC16 hdc, INT16 x, INT16 y )
pub fn MoveTo16(hdc: HDC16, x: i16, y: i16) -> u32 {
    unimplemented!()
}

// BOOL16 WINAPI LineTo16( HDC16 hdc, INT16 x, INT16 y )
pub fn LineTo16(hdc: HDC16, x: i16, y: i16) -> bool {
    unimplemented!()
}

// DWORD WINAPI GetCurrentPosition16( HDC16 hdc )
pub fn GetCurrentPosition16(hdc: HDC16) -> u32 {
    unimplemented!()
}

// INT16 WINAPI FrameRect16( HDC16 hdc, const RECT16 *rect16, HBRUSH16 hbrush )
pub fn FrameRect16(hdc: HDC16, rect: &mut RECT16, hbrush: HBRUSH16) -> i16 {
    unimplemented!()
}

// UINT16 WINAPI RealizePalette16( HDC16 hdc )
pub fn RealizePalette16(hdc: HDC16) -> u16 {
    unimplemented!()
}

// HPALETTE16 WINAPI CreatePalette16( const LOGPALETTE* palette )
pub fn CreatePalette16(palette: &LOGPALETTE) -> HPALETTE16 {
    unimplemented!()
}

// BOOL16 WINAPI TextOut16( HDC16 hdc, INT16 x, INT16 y, LPCSTR str, INT16 count )
pub fn TextOut16(hdc: HDC16, x: i16, y: i16, _str: &String, count: i16) -> bool {
    unimplemented!()
}

// BOOL16 WINAPI PostMessage16( HWND16 hwnd, UINT16 msg, WPARAM16 wparam, LPARAM lparam )
pub fn PostMessage16(hwnd: HWND16, msg: u16, wparam: WPARAM16, lparam: LPARAM) -> bool {
    unimplemented!()
}

// INT16 WINAPI FillRect16( HDC16 hdc, const RECT16 *rect, HBRUSH16 hbrush )
pub fn FillRect16(hdc: HDC16, rect: &RECT16, hbrush: HBRUSH16) -> i16 {
    unimplemented!()
}

// INT16 WINAPI DrawText16( HDC16 hdc, LPCSTR str, INT16 count, LPRECT16 rect, UINT16 flags )
pub fn DrawText16(hdc: HDC16, _str: &String, count: i16, rect: RECT16, flags: u16) -> i16 {
    unimplemented!()
}

// INT16 WINAPI ReleaseDC16( HWND16 hwnd, HDC16 hdc )
pub fn ReleaseDC16(hwnd: HWND16, hdc: HDC16) -> i16 {
    unimplemented!()
}

// BOOL16 WINAPI UnrealizeObject16( HGDIOBJ16 obj )
pub fn UnrealizeObject16(obj: HGDIOBJ16) -> bool {
    unimplemented!()
}

// INT16 WINAPI SetMapMode16( HDC16 hdc, INT16 mode )
pub fn SetMapMode16(hdc: HDC16, mode: i16) -> i16 {
    unimplemented!()
}

pub fn GrayString16(
    hdc: HDC16,
    hbr: HBRUSH16,
    gsprc: GRAYSTRINGPROC16,
    lparam: LPARAM,
    cch: i16,
    x: i16,
    y: i16,
    cx: i16,
    cy: i16,
) -> bool {
    unimplemented!()
}

// INT16 WINAPI GetSystemMetrics16( INT16 index )
pub fn GetSystemMetrics16(index: i16) -> i16 {
    unimplemented!()
}

// HANDLE16 WINAPI GetProp16( HWND16 hwnd, LPCSTR str )
pub fn GetProp16(hwnd: HWND16, str: &String) -> HANDLE16 {
    unimplemented!()
}

// void WINAPI ScreenToClient16( HWND16 hwnd, LPPOINT16 lppnt )
pub fn ScreenToClient16(hwnd: HWND16, lppnt: &POINT16) {
    unimplemented!()
}

// BOOL16 WINAPI IsIconic16(HWND16 hwnd)
pub fn IsIconic16(hwnd: HWND16) -> bool {
    unimplemented!()
}

// HDC16 WINAPI GetWindowDC16( HWND16 hwnd )
pub fn GetWindowDC16(hwnd: HWND16) -> HDC16 {
    unimplemented!()
}

// void WINAPI GetWindowRect16( HWND16 hwnd, LPRECT16 rect )
pub fn GetWindowRect16(hwnd: HWND16, rect: &RECT16) {
    unimplemented!()
}

// LONG WINAPI GetWindowLong16( HWND16 hwnd16, INT16 offset )'
pub fn GetWindowLong16(hwnd16: HWND16, offset: i16) -> libc::c_long {
    unimplemented!()
}

// INT16 WINAPI lstrlen16( LPCSTR str )
pub fn lstrlen16(_str: &String) -> i16 {
    unimplemented!()
}

// DWORD WINAPI GetTextExtent16( HDC16 hdc, LPCSTR str, INT16 count )
pub fn GetTextExtent16(hdc: HDC16, _str: &String, count: i16) -> u32 {
    unimplemented!()
}

// BOOL16 WINAPI MoveToEx16( HDC16 hdc, INT16 x, INT16 y, LPPOINT16 pt )
pub fn MoveToEx16(hdc: HDC16, x: i16, y: i16, pt: &POINT16) -> bool {
    unimplemented!()
}

// HCURSOR16 WINAPI LoadCursor16(HINSTANCE16 hInstance, LPCSTR name)
pub fn LoadCursor16(hinstance: HINSTANCE16, name: &String) -> HCURSOR16 {
    unimplemented!()
}

// HMENU16 WINAPI: u16 LoadMenu16( HINSTANCE16 instance, LPCSTR name )
pub fn LoadMenu16(instance: HINSTANCE16, name: &String) -> HMENU16 {
    unimplemented!()
}

// HMENU16 WINAPI: u16 GetSubMenu16( HMENhMenu: u16, INT16 nPos )
pub fn GetSubMenu16(hmenu: HMENU16, npos: i16) -> HMENU16 {
    unimplemented!()
}

// void WINAPI ClientToScreen16( HWND16 hwnd, LPPOINT16 lppnt )
pub fn ClientToScreen16(hwnd: HWND16, point: &mut POINT16) {
    unimplemented!()
}

pub fn TrackPopupMenu16(
    menu: HMENU16,
    wflags: u16,
    x: i16,
    y: i16,
    reserved: i16,
    hwnd: HWND16,
    rec: &mut RECT16,
) -> bool {
    unimplemented!()
}

// BOOL16 WINAPI WinHelp16( HWND16 hWnd, LPCSTR lpHelpFile, UINT16 wCommand, DWORD dwData)
pub fn WinHelp16(hwnd: HWND16, lphelpfile: &mut String, wcommand: u16, dwdata: u32) -> bool {
    unimplemented!()
}

// BOOL16 WINAPI EnableMenuItem16( HMENhMenu: u16, UINT16 wItemID, UINT16 wFlags )
pub fn EnableMenuItem16(hmenu: HMENU16, witemid: u16, wflags: u16) -> bool {
    unimplemented!()
}

// BOOL16 WINAPI DestroyIcon16(HICON16 hIcon)
pub fn DestroyIcon16(hicon: HICON16) -> bool {
    unimplemented!()
}

// HICON16 WINAPI LoadIcon16(HINSTANCE16 hInstance, LPCSTR name)
pub fn LoadIcon16(hinstance: HINSTANCE16, name: &String) -> HICON16 {
    unimplemented!()
}

pub fn MoveWindow16(hwnd: HWND16, x: i16, y: i16, cx: i16, cy: i16, repaint: bool) -> bool {
    unimplemented!()
}

pub fn UpdateWindow16(hwnd: HWND16) {
    unimplemented!()
}

pub fn CreateWindow16(
    classname: &String,
    window_name: &String,
    style: u32,
    x: i16,
    y: i16,
    width: i16,
    height: i16,
    parent: HWND16,
    menu: HMENU16,
    instance: HINSTANCE16,
    data: &Vec<u8>,
) -> HWND16 {
    unimplemented!()
}

pub fn SetFocus16(hwnd: HWND16) -> HWND16 {
    unimplemented!()
}

pub fn DrawIcon16(hdc: HDC16, x: i16, y: i16, hicon: HICON16) -> bool {
    unimplemented!()
}

pub fn ReleaseCapture16() -> bool {
    unimplemented!()
}

pub fn SetCursor16(h_cursor: HCURSOR16) -> HCURSOR16 {
    unimplemented!()
}

pub fn SetCapture16(hwnd: HWND16) -> HWND16 {
    unimplemented!()
}

pub fn ModifyMenu16(hmenu: HMENU16, pos: u16, flags: u16, id: u16, data: SEGPTR) -> bool {
    unimplemented!()
}

pub fn GetMenuState16(hmenu: HMENU16, witemid: u16, wflags: u16) -> u16 {
    unimplemented!()
}

pub fn DeleteMenu16(hmenu: HMENU16, npos: u16, wflags: u16) -> bool {
    unimplemented!()
}

pub fn InsertMenu16(hmenu: HMENU16, pos: u16, flags: u16, id: u16, data: SEGPTR) -> bool {
    unimplemented!()
}

pub fn CheckMenuItem16(menu: HMENU16, id: u16, flags: u16) -> bool {
    unimplemented!()
}

pub fn BringWindowToTop16(hwnd: HWND16) -> bool {
    unimplemented!()
}

pub fn Ellipse16(hdc: HDC16, left: i16, top: i16, right: i16, bottom: i16) -> bool {
    unimplemented!()
}

pub fn ValidateRect16(hwnd: HWND16, rect: &RECT16) {
    unimplemented!()
}

pub fn SetWindowPos16(
    hwnd: HWND16,
    hwind_insert_after: HWND16,
    x: i16,
    y: i16,
    cx: i16,
    cy: i16,
    flags: u16,
) -> bool {
    unimplemented!()
}

pub fn SetWindowText16(hwnd: HWND16, _string: SEGPTR) -> bool {
    unimplemented!()
}

pub fn MakeProcInstance16(func: FARPROC16, h_instance: HANDLE16) -> FARPROC16 {
    unimplemented!()
}

pub fn EnumChildWindows16(parent: HWND16, func: WDENUMPROC16, lparam: LPARAM) -> bool {
    unimplemented!()
}

pub fn GetWindowWord16(hwnd: HWND16, offset: i16) -> u16 {
    unimplemented!()
}

pub fn _lclose16(_file: HFILE16) -> HFILE16 {
    unimplemented!()
}

pub fn SetDlgItemText16(hwnd: HWND16, id: i16, in_string: SEGPTR) {
    unimplemented!()
}

pub fn LoadString16(instance: HINSTANCE16, resource_id: u16, buffer: &String, d: i16) -> i16 {
    unimplemented!()
}

pub fn MessageBox16(hwnd: HWND16, text: &String, title: &String, _type: u16) -> i16 {
    unimplemented!()
}

pub fn SetErrorMode16(mode: u16) -> u16 {
    unimplemented!()
}

pub fn InitTask16(context: Option<&mut CONTEXT>) {
    unimplemented!()
}

pub fn LockSegment16(handle: HGLOBAL16) -> HGLOBAL16 {
    unimplemented!()
}

pub fn WaitEvent16(task_handle: HTASK16) -> bool {
    unimplemented!()
}

pub fn make_htask(a: u16) -> HTASK16 {
    a as HTASK16
}

pub fn InitApp16(instance_handle: HINSTANCE16) -> i16 {
    unimplemented!()
}

pub fn GetVersion16() -> u32 {
    unimplemented!()
}

pub fn swi(interrupt_val: u16) -> fn() {
    unimplemented!()
}

pub fn FatalAppExit16(action: u16, in_str: &String) {
    unimplemented!()
}

pub fn FatalExit(exit_code: i32) {
    unimplemented!()
}

pub fn _lopen16(path: &String, mode: i16) -> HFILE16 {
    unimplemented!()
}

pub fn _hread(h_file: &HFILE16, buffer: &mut Vec<u8>, count: u32) -> u32 {
    unimplemented!()
}

pub fn _llseek16(h_file: &HFILE16, offset: u32, origin: i16) -> u32 {
    unimplemented!()
}

pub fn _lcreat16(path: &String, attr: i16) -> HFILE16 {
    unimplemented!()
}

pub fn _hwrite16(h_file: &HFILE16, buffer: &Vec<u8>, count: usize) -> usize {
    unimplemented!()
}

pub fn GetModuleFileName16(h_module: &HINSTANCE16, lp_file_name: &String, n_size: i16) -> i16 {
    unimplemented!()
}


/*
HWND16 WINAPI CreateWindowEx16( DWORD exStyle, LPCSTR className,
                                LPCSTR windowName, DWORD style, INT16 x,
                                INT16 y, INT16 width, INT16 height,
                                HWND16 parent, HMENmenu: u16,
                                HINSTANCE16 instance, LPVOID data )
 */
pub fn CreateWindowEx16(exStyle: u32, className: &String, windowName: &String, style: u32, x: i16, y: i16, width: i16, height: i16, parent: HWND16, menu: HMENU16, instance: HINSTANCE16, data: &Vec<u8>) -> HWND16 {
    unimplemented!()
}

/*
BOOL16 WINAPI GetClassInfo16( HINSTANCE16 hInst16, SEGPTR name, WNDCLASS16 *wc )
 */
pub fn GetClassInfo16(h_inst: HINSTANCE16, name: SEGPTR, wc: &WNDCLASS16) -> bool {
    unimplemented!()
}

/*
GetDOSEnviornment16
 */
pub fn GetDOSEnviornment16() -> SEGPTR {
    unimplemented!()
}

/*
void WINAPI MessageBeep16( UINT16 i )
 */
pub fn MessageBeep16(i: u16) {
    unimplemented!()
}

/*
BOOL16 WINAPI WritePrivateProfileString16( LPCSTR section, LPCSTR entry,
                                           LPCSTR string, LPCSTR filename )
 */
pub fn WritePrivateProfileString16(section: &String, entry: &String, _string: &String, filename: &String) -> bool {
    unimplemented!()
}

/*
BOOL16 WINAPI GlobalUnlock16(
              HGLOBAL16 handle /* [in] Handle of global memory object */
)
 */
pub fn GlobalUnlock16(handle: &HGLOBAL16) -> bool {
    unimplemented!()
}

/*
BOOL16 WINAPI FreeResource16( HGLOBAL16 handle )
 */
pub fn FreeResource16(handle: &HGLOBAL16) -> bool {
    unimplemented!()
}

/*
HGLOBAL16 WINAPI LoadResource16( HMODULE16 hModule, HRSRC16 hRsrc )
 */
pub fn LoadResource16(h_module: &HMODULE16, h_rsrc: &HRSRC16) -> HGLOBAL16 {
    unimplemented!()
}

/*
DWORD WINAPI GlobalDOSAlloc16(
             DWORD size /* [in] Number of bytes to be allocated */
 */
pub fn GlobalDOSAlloc16(size: u32) -> u32 {
    unimplemented!()
}

/*
HGLOBAL16 WINAPI GlobalAlloc16(
                 UINT16 flags, /* [in] Object allocation attributes */
                 DWORD size    /* [in] Number of bytes to allocate */
 */
pub fn GlobalAlloc16(flags: u16, size: u32) -> HGLOBAL16 {
    unimplemented!()
}

/*
HGLOBAL16 WINAPI GlobalReAlloc16(
                 HGLOBAL16 handle, /* [in] Handle of global memory object */
                 DWORD size,       /* [in] New size of block */
                 UINT16 flags      /* [in] How to reallocate object */
)
 */
pub fn GlobalReAlloc16(
    handle: HGLOBAL16,
    size: u32,
    flags: u16
) -> HGLOBAL16 {
    unimplemented!()
}

/*
WORD WINAPI GlobalPageUnlock16( HGLOBAL16 handle )
 */
pub fn GlobalPageUnlock16(handle: HGLOBAL16) {
    unimplemented!()
}

/*
WORD WINAPI GlobalPageLock16( HGLOBAL16 handle )
 */
pub fn GlobalPageLock16(handle: HGLOBAL16) -> u16 {
    unimplemented!()
}

/*
LPVOID WINAPI GlobalLock16(
              HGLOBAL16 handle /* [in] Handle of global memory object */
)
 */
pub fn GlobalLock16(handle: HGLOBAL16) -> SEGPTR {
    unimplemented!()
}

/*
DWORD WINAPI GlobalHandle16(
             WORD sel /* [in] Address of global memory block */
)
 */
pub fn GlobalHandle16(sel: u16) -> u32 {
    unimplemented!()
}

/*
DWORD WINAPI mciSendCommand16(UINT16 wDevID, UINT16 wMsg, DWORD dwParam1, DWORD p2)
 */
pub fn mciSendCommand16(w_dev_id: u16, w_msg: u16, dw_param_1: u32, p2: u32) -> u32 {
    unimplemented!()
}

/*
BOOL16 WINAPI mciGetErrorString16(DWORD wError, LPSTR lpstrBuffer, UINT16 uLength)
 */
pub fn mciGetErrorString16(w_error: u32, buffer: &mut String, u_length: u16) -> bool {
    unimplemented!()
}

/*
void WINAPI OutputDebugString16( LPCSTR str )
 */
pub fn OutputDebugString16(str: &String) {
    unimplemented!()
}

/*
BOOL16 WINAPI SetProp16( HWND16 hwnd, LPCSTR str, HANDLE16 handle )
 */
pub fn SetProp16(hwnd: HWND16, str: &String, handle: HANDLE16) -> bool {
    unimplemented!()
}

/*
LRESULT WINAPI SendDlgItemMessage16( HWND16 hwnd, INT16 id, UINT16 msg,
                                     WPARAM16 wParam, LPARAM lParam )
 */
pub fn SendDlgItemMessage16(hwnd: HWND16, id: i16, msg: u16, w_param: WPARAM16, l_param: LPARAM) -> LRESULT {
    unimplemented!()
}

/*
BOOL16 WINAPI GetMessage16( MSG16 *msg, HWND16 hwnd, UINT16 first, UINT16 last )
 */
pub fn GetMessage16(msg: &MSG16, hwnd: &HWND16, first: u16, last: u16) -> bool {
    unimplemented!()
}

/*
BOOL16 WINAPI IsDialogMessage16( HWND16 hwndDlg, MSG16 *msg16 )
 */
pub fn IsDialogMessage16(hwnd_dlg: HWND16, msg: &MSG16) -> bool {
    unimplemented!()
}

/*
void WINAPI PostQuitMessage16( INT16 exitCode )
 */
pub fn PostQuitMessage16(exit_code: u16) {
    unimplemented!()
}
