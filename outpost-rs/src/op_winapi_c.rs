//
// Created by cyrex on 2/22/2022.
//

// #include "op_winapi.h"

// #include "op_int.h"
// #include "op_windef.h"

// #include <stdarg.h>
// #include <stdbool.h>
// #include <stdint.h>

pub fn InitTask16(CONTEXT *context)
{
    // TODO: implement
}

HGLOBAL16 LockSegment16(HGLOBAL16 handle)
{
    // TODO: implement
    return 0;
}

DWORD GetVersion16(void)
{
    // TODO: implement
    return 0;
}

void *swi(opcode: u8)
{
    // TODO: implement
    return 0;
}

// u16 swi_0x21_fn_ptr swi_0x21()
//{
//     // TODO: implement
//     swi_0x21_fn_ptr result = NULL;
//     return result;
// }

BOOL16 WaitEvent16(HTASK16 h_task)
{
    // TODO: implement
    return false;
}

i16 InitApp16(HINSTANCE16 h_instance)
{
    // TODO: implement
    return 0;
}

pub fn FatalAppExit16(action: u16, cstring str)
{
    // TODO: implement
}

pub fn FatalExit(void)
{
    // TODO: implement
}


BOOL16 DeleteObject16(obj: HGDIOBJ16)
{
    // TODO: implement
    return 0;
}

u16 LoadString16(instance: HINSTANCE16, resource_id: u16, cstring buffer, buf_len: u16)
{
    // TODO: implement
    return 0;
}

// HGLOBAL16                   GLobalAlloc16(flags: u16, DWORD size);
HGLOBAL16 GlobalAlloc16(flags: u16, DWORD size)
{
    // TODO: implement
    return 0;
}

// HGLOBAL16                   GlobalReAlloc16(HGLOBAL16 handle, DWORD size, flags: u16);
HGLOBAL16 GlobalRealloc16(HGLOBAL16 handle, DWORD size, flags: u16)
{
    // TODO: implement
    return 0;
}

// HGLOBAL16                   GlobalFree16(HGLOBAL16 handle);
HGLOBAL16 GlobalFree16(HGLOBAL16 handle)
{
    // TODO: implement
    return 0;
}
// SEGPTR                      WIN16_GlobalLock16(HGLOBAL16 handle);
SEGPTR WIN16_GlobalLock16(HGLOBAL16 handle)
{
    // TODO: implement
    return 0;
}
// BOOL16                      GlobalUnlock16(HGLOBAL16 handle);
BOOL16 GlobalUnlock16(HGLOBAL16 handle)
{
    // TODO: implement
    return false;
}
// DWORD                       GlobalSize16(HGLOBAL16 handle);
DWORD GlobalSize16(HGLOBAL16 handle)
{
    // TODO: implement
    return 0;
}

// DWORD                       GlobalHandle16(WORD sel);
DWORD GlobalHandle16(WORD sel)
{
    // TODO: implement
    return 0;
}

// u16                       GetModuleFileName16(h_module: HINSTANCE16, lp_file_name: *mut c_char,
// n_size: u16);
u16 GetModuleFileName16(h_module: HINSTANCE16, lp_file_name: *mut c_char, n_size: u16)
{
    // TODO: implement
    return 0;
}

// LPVOID                      MakeProcInstance16(LPVOID func, h_instance: HANDLE16);
LPVOID MakeProcInstance16(LPVOID func, h_instance: HANDLE16)
{
    // TODO: implement
    return NULL;
}

// void                        FreeProcInstance16(LPVOID func);
pub fn FreeProcInstance16(LPVOID func)
{
    // TODO: implement
}

// HRSRC16                     FindResource16(HMODULE16 h_module, name: *mut c_char, LPCSTR
// type);
HRSRC16 FindResource16(HMODULE16 h_module, name: *mut c_char, LPCSTR type)
{
    // TODO: implement
    return 0;
}

// HGLOBAL16                   LoadResource16(HMODULE16 h_module, HRSRC16 h_rsrc);
HGLOBAL16 LoadResource16(HMODULE16 h_module, HRSRC16 h_rsrc)
{
    // TODO: implement
    return 0;
}

// SEGPTR                      WIN16_LockResource16(HGLOBAL16 handle);
SEGPTR WIN16_LockResource16(HGLOBAL16 handle)
{
    // TODO: implement
    return 0;
}

// BOOL16                      FreeResource16(HGLOBAL16 handle);
BOOL16 FreeResource16(HGLOBAL16 handle)
{
    // TODO: implement
    return false;
}

// HFILE16                     _lclose16(HFILE16 h_file);
HFILE16 _lclose16(HFILE16 h_file)
{
    // TODO: implement
    return 0;
}

// HFILE16                     _lcreat16(path: *mut c_char, attr: u16);
HFILE16 _lcreat16(path: *mut c_char, attr: u16)
{
    // TODO: implement
    return 0;
}

// long                        _llseek16(HFILE16 h_file, long l_offset, n_origin: u16);
long _llseek16(HFILE16 h_file, long l_offset, n_origin: u16)
{
    // TODO: implement
    return 0;
}

// HFILE16                     _lopen16(path: *mut c_char, mode: u16);
HFILE16 _lopen16(path: *mut c_char, mode: u16)
{
    // TODO: implement
    return 0;
}

// u16                       lstrlen16(LPCSTR str);
u16 lstrlen16(LPCSTR str)
{
    // TODO: implement
    return 0;
}

// void                        DOS3Call(CONTEXT *context);
pub fn DOS3Call(CONTEXT *context)
{
    // TODO: implement
}

// u16                         SetErrorMode16(mode: u16);
u16 SetErrorMode16(mode: u16)
{
    // TODO: implement
    return 0;
}

// void                        __AHSHIFT(void);
pub fn __AHSHIFT(void)
{
    // TODO: implement
}

// void                        __AHINCR(void);
pub fn __AHINCR(void)
{
    // TODO: implement
}

// void                        OutputDebugString16(LPCSTR str);
pub fn OutputDebugString16(LPCSTR str)
{
    // TODO: implement
}

// u16                       GetPrivateProfileString16(section: *mut c_char, entry: *mut c_char,
// def_val: *mut c_char, buffer: *mut c_char, len: u16, LPCSTR filename);
u16 GetPrivateProfileString16(section: *mut c_char,
                              entry: *mut c_char,
                              def_val: *mut c_char,
                              LPSTR  buffer,
                              len: u16,
                              LPCSTR filename)
{
    // TODO: implement
    return 0;
}


// BOOL16                      WritePrivateProfileString16(section: *mut c_char, entry: *mut c_char,
// string: *mut c_char, LPCSTR filename);
BOOL16
WritePrivateProfileString16(section: *mut c_char, entry: *mut c_char, string: *mut c_char, LPCSTR filename)
{
    // TODO: implement
    return false;
}

// SEGPTR                      GetDOSEnvironment16(void);
SEGPTR GetDOSEnvironment16()
{
    // TODO: implement
    return 0;
}

// HINSTANCE16                 WinExec16(lp_cmd_line: *mut c_char, n_cmd_show: u16);
HINSTANCE16 WinExec16(lp_cmd_line: *mut c_char, n_cmd_show: u16)
{
    // TODO: implement
    return 0;
}

// void                        __WINFLAGS(void);
pub fn __WINFLAGS()
{
    // TODO: implement
}

// DWORD                       GlobalDOSAlloc16(DWORD size);
DWORD GlobalDOSAlloc16(DWORD size)
{
    // TODO: implement
    return 0;
}

// WORD                        GlobalDOSFree16(WORD sel);
WORD GlobalDOSFree16(WORD sel)
{
    // TODO: implement
    return 0;
}

// WORD                        GlobalPageLock16(HGLOBAL16 handle);
WORD GlobalPageLock16(HGLOBAL16 handle)
{
    // TODO: implement
    return 0;
}

// WORD                        GlobalPageUnlock16(HGLOBAL16 handle);
WORD GlobalPageUnlock16(HGLOBAL16 handle)
{
    // TODO: implement
    return 0;
}

// void                        hmemcpy16(LPVOID dst, LPCVOID src, long count);
pub fn hmemcpy16(LPVOID dsg, LPCVOID src, long count)
{
    // TODO: implement
}

// long                        WIN16_hread(HFILE16 h_file, buffer: SEGPTR, long count);
long WIN16_hread(HFILE16 h_file, buffer: SEGPTR, long count)
{
    // TODO: implement
    return 0;
}

// long                        _hwrite16(HFILE16 h_file, buffer: *mut c_char, long count);
long _hwrite16(HFILE16 h_file, buffer: *mut c_char, long count)
{
    // TODO: implement
    return 0;
}

// COLORREF                    SetBkColor16(hdc: HDC16, color: COLORREF);
COLORREF SetBkColor16(hdc: HDC16, color: COLORREF)
{
    // TODO: implement
    return 0;
}

// i16                     SetMapMode16(hdc: HDC16, i16 mode);
i16 SetMapMode16(hdc: HDC16, i16 mode)
{
    // TODO: implement
    return 0;
}

// COLORREF                    SetTextColor16(hdc: HDC16, color: COLORREF);
COLORREF SetTextColor16(hdc: HDC16, color: COLORREF)
{
    // TODO: implement
    return 0;
}

// BOOL16                      LineTo16(hdc: HDC16, x: u16, y: u16);
BOOL16 LineTo16(hdc: HDC16, x: u16, y: u16)
{
    // TODO: implement
    return false;
}

// DWORD                       MoveTo16(hdc: HDC16, x: u16, y: u16);
DWORD MoveTo16(hdc: HDC16, x: u16, y: u16)
{
    // TODO: implement
    return 0;
}

// BOOL16                      Ellipse16(hdc: HDC16, left: u16, top: u16, right: u16, u16
// bottom);
BOOL16 Ellipse16(hdc: HDC16, left: u16, top: u16, right: u16, bottom: u16)
{
    // TODO: implement
    return false;
}

// BOOL16                      Rectangle16(hdc: HDC16, left: u16, top: u16, right: u16, u16
// bottom);
BOOL16 Rectangle16(hdc: HDC16, left: u16, top: u16, right: u16, bottom: u16)
{
    // TODO: implement
    return false;
}

// BOOL16                      TextOut16(hdc: HDC16, x: u16, y: u16, char *str, count: u16);
BOOL16 TextOut16(hdc: HDC16, x: u16, y: u16, char *str, count: u16)
{
    // TODO: implement
    return false;
}

// BOOL16                      Polygon16(hdc: HDC16, pt: *mut POINT16, count: u16);
BOOL16 Polygon16(hdc: HDC16, pt: *mut POINT16, count: u16)
{
    // TODO: implement
    return false;
}

// HGDIOBJ16                   SelectObject16(hdc: HDC16, handle: HGDIOBJ16);
HGDIOBJ16 SelectObject16(hdc: HDC16, handle: HGDIOBJ16)
{
    // TODO: implement
    return 0;
}


// HDC16                       CreateDC16(driver: *mut c_char, device: *mut c_char, output: *mut c_char,
// DEVMODEA *init_data);
HDC16 CreateDC16(driver: *mut c_char, device: *mut c_char, output: *mut c_char, DEVMODEA *init_data)
{
    // TODO: implement
    return 0;
}


// HPEN16                      CreatePen16(style: u16, width: u16, color: COLORREF);
HPEN16 CreatePen16(style: u16, width: u16, color: COLORREF)
{
    // TODO: implement
    return 0;
}

// HBRUSH16                    CreateSolidBrush16(color: COLORREF);
HBRUSH16 CreateSolidBrush16(color: COLORREF)
{
    // TODO: implement
    return 0;
}


// BOOL16                      DeleteDC16(hdc: HDC16);
BOOL16 DeleteDC16(hdc: HDC16)
{
    // TODO: implement
    return false;
}


// DWORD                       GetCurrentPosition16(hdc: HDC16);
DWORD GetCurrentPosition16(hdc: HDC16)
{
    // TODO: implement
    return 0;
}


// u16                       GetDeviceCaps16(hdc: HDC16, cap: u16);
u16 GetDeviceCaps16(hdc: HDC16, cap: u16)
{
    // TODO: implement
    return 0;
}


// HGDIOBJ16                   GetStockObject16(obj: u16);
HGDIOBJ16 GetStockObject16(obj: u16)
{
    // TODO: implement
    return 0;
}

// DWORD                       GetTextExtent16(hdc: HDC16, str: *mut c_char, count: u16);
DWORD GetTextExtent16(hdc: HDC16, str: *mut c_char, count: u16)
{
    // TODO: implement
    return 0;
}

// BOOL16                      UnrealizeObject16(obj: HGDIOBJ16);
BOOL16 UnrealizeObject16(obj: HGDIOBJ16)
{
    // TODO: implement
    return false;
}

// HPALETTE16                  CreatePalette16(LOGPALETTE *palette);
HPALETTE16 CreatePalette16(LOGPALETTE *palette)
{
    // TODO: implement
    return 0;
}


// u16                         GetSystemPaletteEntries(hdc: HDC16, start: u16, count: u16,
// PALETTEENTRY *entries);
u16 GetSystemPaletteEntries(hdc: HDC16, start: u16, count: u16, PALETTEENTRY *entries)
{
    // TODO: implement
    return 0;
}

// u16                       StretchDIBits16(hdc: HDC16, x_dst: u16, y_dst: u16, u16
// width_dst, height_dst: u16, x_src: u16, y_src: u16, width_src: u16, height_src: u16, PVOID
// bits, BITMAPINFO *info, w_usage: u16, DWORD dw_rop);
u16 StretchDIBits16(HDC16       hdc,
                    x_dst: u16,
                    y_dst: u16,
                    width_dst: u16,
                    height_dst: u16,
                    x_src: u16,
                    y_src: u16,
                    width_src: u16,
                    height_src: u16,
                    PVOID       bits,
                    BITMAPINFO *info,
                    w_usage: u16,
                    DWORD       dw_rop)
{
    // TODO: implement
    return 0;
}


// u16                       SetDIBitsToDevice(hdc: HDC16, x_dest: u16, y_dest: u16, cx: u16,
// cy: u16, x_src: u16, y_src: u16, startscan: u16, lines: u16, LPCVOID bits, BITMAPINFO *info,
// coloruse: u16);
u16 SetDIBitsToDevice(HDC16       hdc,
                      x_dest: u16,
                      y_dest: u16,
                      cx: u16,
                      cy: u16,
                      x_src: u16,
                      y_src: u16,
                      startscan: u16,
                      lines: u16,
                      LPCVOID     bits,
                      BITMAPINFO *info,
                     coloruse: u16)
{
    // TODO: implement
    return 0;
}

// BOOL16                      MoveToEx16(hdc: HDC16, x: u16, y: u16, POINT16 *pt);
BOOL16 MoveToEx16(hdc: HDC16, x: u16, y: u16, POINT16 *pt)
{
    // TODO: implement
    return false;
}

// u16                       MessageBox16(hwnd: HWND16, text: *mut c_char, title: *mut c_char, u16
// type);
u16 MessageBox16(hwnd: HWND16, text: *mut c_char, title: *mut c_char, type: u16)
{
    // TODO: implement
    return 0;
}


// void                        PostQuitMessage16(exit_code: u16);
pub fn PostQuitMessage16(exit_code: u16)
{
    // TODO: implement
}

// u16                         SetTimer16(hwnd: HWND16, id: u16, timeout: u16, LPVOID proc);
u16 SetTimer16(hwnd: HWND16, id: u16, timeout: u16, LPVOID proc)
{
    // TODO: implement
    return 0;
}

// BOOL16                      KillTimer16(hwnd: HWND16, id: u16);
BOOL16 KillTimer16(hwnd: HWND16, id: u16)
{
    // TODO: implement
    return false;
}


// BOOL16                      GetCursorPos16(POINT16 *pt);
BOOL16 GetCursorPos16(POINT16 *pt)
{
    // TODO: implement
    return false;
}


// HWND16                      SetCapture16(hwnd: HWND16);
HWND16 SetCapture16(hwnd: HWND16)
{
    // TODO: implement
    return 0;
}

// BOOL16                      ReleaseCapture16(void);
BOOL16 ReleaseCapture16()
{
    // TODO: implement
    return false;
}

// HWND16                      SetFocus16(hwnd: HWND16);
HWND16 SetFocus16(hwnd: HWND16)
{
    // TODO: implement
    return 0;
}

// HANDLE16                    RemoveProp16(hwnd: HWND16, LPCSTR str);
HANDLE16 RemoveProp16(hwnd: HWND16, LPCSTR str)
{
    // TODO: implement
    return 0;
}

// HANDLE16                    GetProp16(hwnd: HWND16, LPCSTR str);
HANDLE16 GetProp16(hwnd: HWND16, LPCSTR str)
{
    // TODO: implement
    return 0;
}


// BOOL16                      SetProp16(hwnd: HWND16, str: *mut c_char, handle: HANDLE16);
BOOL16 SetProp16(hwnd: HWND16, str: *mut c_char, handle: HANDLE16)
{
    // TODO: implement
    return false;
}


// void                        ClientToScreen16(hwnd: HWND16, POINT16 *lppnt);
pub fn ClientToScreen16(hwnd: HWND16, POINT16 *lppnt)
{
    // TODO: implement
}

// void                        ScreenToClient16(hwnd: HWND16, POINT16 *lppnt);
pub fn ScreenToClient16(hwnd: HWND16, POINT16 *lppnt)
{
    // TODO: implement
}

// BOOL16                      IsIconic16(hwnd: HWND16);
BOOL16 IsIconic16(hwnd: HWND16)
{
    // TODO: implement
    return false;
}

// void                        GetWindowRect16(hwnd: HWND16, rect: *mut RECT16);
pub fn GetWindowRect16(hwnd: HWND16, rect: *mut RECT16)
{
    // TODO: implement
}


// void                        GetClientRect16(hwnd: HWND16, rect: *mut RECT16);
pub fn GetClientRect16(hwnd: HWND16, rect: *mut RECT16)
{
    // TODO: implement
}

// BOOL16                      EnableWindow16(hwnd: HWND16, BOOL16 enable);
BOOL16 EnableWindow16(hwnd: HWND16, BOOL16 enable)
{
    // TODO: implement
    return false;
}

// BOOL16                      IsWindowEnabled16(hwnd: HWND16);
BOOL16 IsWindowEnabled16(hwnd: HWND16)
{
    // TODO: implement
    return false;
}


// u16                       GetWindowText16(hwnd: HWND16, lp_string: SEGPTR, u16
// n_max_count);
u16 GetWindowText16(hwn: HWND16, lp_string: SEGPTR, n_max_count: u16)
{
    // TODO: implement
    return 0;
}


// BOOL16                      SetWindowText16(hwnd: HWND16, SEGPTR lp_string);
BOOL16 SetWindowText16(hwnd: HWND16, SEGPTR lp_string)
{
    // TODO: implement
    return false;
}


// HDC16                       BeginPaint16(hwnd: HWND16, PAINTSTRUCT16 *lps);
HDC16 BeginPaint16(hwnd: HWND16, PAINTSTRUCT16 *lps)
{
    // TODO: implement
    return 0;
}


// BOOL16                      EndPaint16(hwnd: HWND16, PAINTSTRUCT16 *lps);
BOOL16 EndPaint16(hwnd: HWND16, PAINTSTRUCT16 *lps)
{
    // TODO: implement
    return false;
}

// HWND16                      CreateWindow16(class_name: *mut c_char, window_name: *mut c_char, DWORD
// style, x: u16, y: u16, width: u16, height: u16, parent: HWND16, HMENhmenu: u16, HINSTANCE16
// instance, LPVOID data);
HWND16 CreateWindow16(class_name: *mut c_char,
                      window_name: *mut c_char,
                      DWORD  style,
                      x: u16,
                      y: u16,
                      width: u16,
                      height: u16,
                      parent: HWND16,
                      HMENmenu: u16,
                      instance: HINSTANCE16,
                      LPVOID      data)
{
    // TODO: implement
    return 0;
}


// BOOL16                      ShowWindow16(hwnd: HWND16, cmd: u16);
BOOL16 ShowWindow16(hwnd: HWND16, cmd: u16)
{
    // TODO: implement
    return false;
}

// BOOL16                      BringWindowToTop16(hwnd: HWND16);
BOOL16 BringWindowToTop16(hwnd: HWND16)
{
    // TODO: implement
    return false;
}

// BOOL16                      IsWindow16(hwnd: HWND16);
BOOL16 IsWindow16(hwnd: HWND16)
{
    // TODO: implement
    return false;
}

// BOOL16                      DestroyWindow16(hwnd: HWND16);
BOOL16 DestroyWindow16(hwnd: HWND16)
{
    // TODO: implement
    return false;
}


// BOOL16                      EnumChildWindows1(parent: HWND16, LPVOID func, LPARAM
// lparam);
BOOL16 EnumChildWindows16(parent: HWND16, LPVOID fun, LPARAM lparam)
{
    // TODO: implement
    return false;
}


// BOOL16                      MoveWindow16(hwnd: HWND16, x: u16, y: u16, cx: u16, cy: u16,
// BOOL16 repaint);
BOOL16 MoveWindow16(hwnd: HWND16, x: u16, y: u16, cx: u16, cy: u16, BOOL16 repaint)
{
    // TODO: implement
    return false;
}


// ATOM                        RegisterClass16(WNDCLASS16 *wc);
ATOM RegisterClass16(WNDCLASS16 *wc)
{
    // TODO: implement
    return 0;
}


// HDC16                       GetDC16(hwnd: HWND16);
HDC16 GetDC16(hwnd: HWND16)
{
    // TODO: implement
    return 0;
}


// HDC16                       GetWindowDC16(hwnd: HWND16);
HDC16 GetWindow16(hwnd: HWND16)
{
    // TODO: implement
    return 0;
}

// u16                       ReleaseDC16(hwnd: HWND16, hdc: HDC16);
u16 ReleaseDC16(hwnd: HWND16, hdc: HDC16)
{
    // TODO: implement
    return 0;
}

// HCURSOR16                   SetCursor16(HCURSOR16 hcursor);
HCURSOR16 SetCursor16(HCURSOR16 hcursor)
{
    // TODO: implement
    return 0;
}

// u16                       ShowCursor16(BOOL16 b_show);
u16 ShowCursor16(BOOL16 b_show)
{
    // TODO: implement
    return 0;
}


// BOOL16                      PtInRect16(rect: *mut RECT16, POINT16 pt);
BOOL16 PtInRect16(rect: *mut RECT16, POINT16 pt)
{
    // TODO: implement
    return false;
}


// u16                       FillRect16(hdc: HDC16, rect: *mut RECT16, HBRUSH16 hbrush);
u16 FillRect16(hdc: HDC16, rect: *mut RECT16, HBRUSH16 hbrush)
{
    // TODO: implement
    return 0;
}


// u16                       FrameRect16(hdc: HDC16, rect: *mut RECT16, HBRUSH16 hbrush);
u16 FrameRect16(hdc: HDC16, rect: *mut RECT16, HBRUSH16 hbrush)
{
    // TODO: implement
    return 0;
}


// BOOL16                      DrawIcon16(hdc: HDC16, x: u16, y: u16, h_icon: HICON16);
BOOL16 DrawIcon16(hdc: HDC16, x: u16, y: u16, h_icon: HICON16)
{
    // TODO: implement
    return false;
}


// u16                       DrawText16(hdc: HDC16, str: *mut c_char, count: u16, rect: *mut RECT16,
// flags: u16);
u16 DrawText16(hdc: HDC16, str: *mut c_char, count: u16, rect: *mut RECT16, flags: u16)
{
    // TODO: implement
    return 0;
}


// HWND16                      CreateDialog16(hinst: HINSTANCE16, dlg_template: *mut c_char,
// owner: HWND16, LPVOID dlg_proc);
HWND16
CreateDialog16(hinst: HINSTANCE16, dlg_template: *mut c_char, owner: HWND16, LPVOID dlg_proc)
{
    // TODO: implement
    return 0;
}


// BOOL16                      IsDialogMessage16(hwnd_dlg: HWND16, MSG16 *msg16);
BOOL16 IsDialogMessage16(hwnd_dlg: HWND16, MSG16 *msg16)
{
    // TODO: implement
    return false;
}

// HWND16                      GetDlgItem16(hwnd_dlg: HWND16, id: u16);
HWND16 GetDlgItem16(hwnd_dlg: HWND16, id: u16)
{
    // TODO: implement
    return 0;
}

// void                        SetDlgItemText16(hwnd: HWND16, id: u16, SEGPTR lp_string);
pub fn SetDlgItemText16(hwnd: HWND16, id: u16, SEGPTR lp_string)
{
    // TODO: implement
}

// void                        SetDlgItemInt16(hwnd: HWND16, id: u16, value: u16, BOOL16
// f_signed);
pub fn SetDlgItemInt16(hwnd: HWND16, id: u16, value: u16, BOOL16 f_signed)
{
    // TODO: implement
}

// u16                         GetDlgItemInt16(hwnd: HWND16, id: u16, BOOL16 *translated,
// BOOL16 f_signed);
u16 GetDlgItemInt16(hwnd: HWND16, id: u16, BOOL16 *translated, BOOL16 f_signed)
{
    // TODO: implement
    return 0;
}

// BOOL16                      CheckRadioButton16(hwnd_dlg: HWND16, first_id: u16, u16
// last_id, check_id: u16);
BOOL16 CheckRadioButton16(hwnd_dlg: HWND16, first_id: u16, last_id: u16, check_id: u16)
{
    // TODO: implement
    return false;
}

// BOOL16                      CheckDlgButton16(hwnd: HWND16, id: u16, check: u16);
BOOL16 CheckDlgButton16(hwnd: HWND16, id: u16, check: u16)
{
    // TODO: implement
    return false;
}


// u16                         IsDlgButtonChecked(hwnd: HWND16, id: u16);
u16 IsDlgButtonChecked(hwnd: HWND16, id: u16)
{
    // TODO: implement
    return 0;
}


// LRESULT                     SendDlgItemMessage16(hwnd: HWND16, id: u16, msg: u16, WPARAM16
// w_param, LPARAM l_param);
LRESULT
SendDlgItemMessage16(hwnd: HWND16, id: u16, msg: u16, WPARAM16 w_param, LPARAM l_param)
{
    // TODO: implement
    return 0;
}


// void                        MapDialogRect16(hwnd: HWND16, rect: *mut RECT16);
pub fn MapDialogRect16(hwnd: HWND16, rect: *mut RECT16)
{
    // TODO: implement
}


// void                        MessageBeep16(i: u16);
pub fn MessageBeep16(i: u16)
{
    // TODO: implement
}


// LRESULT                     DefWindowProc16(hwnd: HWND16, msg: u16, WPARAM16 wparam,
// LPARAM lparam);
LRESULT DefWindowProc16(hwnd: HWND16, msg: u16, WPARAM16 wparam, LPARAM lparam)
{
    // TODO: implement
    return 0;
}


// BOOL16                      GetMessage16(MSG16 *msg, hwnd: HWND16, first: u16, last: u16);
BOOL16 GetMessage16(MSG16 *msg, hwnd: HWND16, first: u16, last: u16)
{
    // TODO: implement
    return false;
}


// BOOL16                      PostMessage16(hwnd: HWND16, msg: u16, WPARAM16 wparam, LPARAM
// lparam);
BOOL16 PostMessage16(hwnd: HWND16, msg: u16, WPARAM16 wparam, LPARAM lparam)
{
    // TODO: implement
    return false;
}


// LRESULT                     SendMessage16(hwnd: HWND16, msg: u16, WPARAM16 wparam, LPARAM
// lparam);
LRESULT SendMessage16(hwnd: HWND16, msg: u16, WPARAM16 wparam, LPARAM lparam)
{
    // TODO: implement
    return 0;
}


// BOOL16                      TranslateMessage16(MSG16 *msg);
BOOL16 TranslateMessage16(MSG16 *msg)
{
    // TODO: implement
    return false;
}

// long                        DispatchMessage16(MSG16 *msg);
long DispatchMessage16(MSG16 *msg)
{
    // TODO: implement
    return 0;
}

// LRESULT                     CallWindowProc16(LPVOID func, hwnd: HWND16, msg: u16,
// WPARAM16 wparam, LPARAM lparam);
LRESULT CallWindowProc16(LPVOID fun, hwnd: HWND16, msg: u16, WPARAM16 wparam, LPARAM lparam)
{
    // TODO: implement
    return 0;
}


// void                        UpdateWindow16(hwnd: HWND16);
pub fn UpdateWindow16(hwnd: HWND16)
{
    // TODO: implement
}


// void                        InvalidateRect16(hwnd: HWND16, rect: *mut RECT16, BOOL16 erase);
pub fn InvalidateRect16(hwnd: HWND16, rect: *mut RECT16, BOOL16 erase)
{
    // TODO: implement
}


// void                        ValidateRect16(hwnd: HWND16, rect: *mut RECT16);
pub fn ValidateRect16(hwnd: HWND16, rect: *mut RECT16)
{
    // TODO: implement
}


// WORD                        GetWindowWord16(hwnd: HWND16, offset: u16);
WORD GetWindowWord16(hwnd: HWND16, offset: u16)
{
    // TODO: implement
    return 0;
}

// WORD                        SetWindowWord16(hwnd: HWND16, offset: u16, WORD newval);
WORD SetWindowWord16(hwnd: HWND16, offset: u16, WORD new_val)
{
    // TODO: implement
    return 0;
}

// long                        GetWindowLong16(hwnd: HWND16, offset: u16);
long GetWindowLong16(hwnd: HWND16, offset: u16)
{
    // TODO: implement
    return 0;
}


// long                        SetWindowLong16(hwnd: HWND16, offset: u16, long newval);
long SetWindowLong16(hwnd: HWND16, offset: u16, long newval)
{
    // TODO: implement
    return 0;
}

// HMENU16                     LoadMenu16(instance: HINSTANCE16, LPCSTR name);
HMENU16 LoadMenu16(instance: HINSTANCE16, LPCSTR name)
{
    // TODO: implement
    return 0;
}

// BOOL16                      DestroyMenu16(HMENmenu: u16);
BOOL16 DestroyMenu16(HMENmenu: u16)
{
    // TODO: implement
    return false;
}

// BOOL16                      CheckMenuItem16(HMENhmenu: u16, w_item_id: u16, w_flags: u16);
BOOL16 CheckMenuItem16(HMENhmenu: u16, w_item_id: u16, w_flags: u16)
{
    // TODO: implement
    return false;
}


// BOOL16                      EnableMenuItem16(HMENhmenu: u16, w_item_id: u16, u16
// w_flags);
BOOL16 EnableMenuItem16(HMENhmenu: u16, w_item_id: u16, w_flags: u16)
{
    // TODO: implement
    return false;
}


// HMENU16                     GetSubMenu16(HMENh_menu: u16, n_pos: u16);
HMENU16 GetSubMenu16(HMENh_menu: u16, n_pos: u16)
{
    // TODO: implement
    return 0;
}

// BOOL16                      WinHelp16(hwnd: HWND16, lp_help_file: *mut c_char, w_command: u16,
// DWORD dw_data);
BOOL16 WinHelp16(hwnd: HWND16, lp_help_file: *mut c_char, w_command: u16, DWORD dw_data)
{
    // TODO: false
    return false;
}


// HCURSOR16                   LoadCursor16(h_instance: HINSTANCE16, LPCSTR name);
HCURSOR16 LoadCursor16(h_instance: HINSTANCE16, LPCSTR name)
{
    // TODO: implement
    return 0;
}


// HICON16                     LoadIcon16(h_instance: HINSTANCE16, LPCSTR name);
HICON16 LoadIcon16(h_instance: HINSTANCE16, LPCSTR name)
{
    // TODO: implement
    return 0;
}


// HACCEL16                    LoadAccelerators16(instance: HINSTANCE16, LPCSTR
// lp_table_name);
HACCEL16 LoadAccelerators16(instance: HINSTANCE16, LPCSTR lp_table_name)
{
    // TODO: implement
    return 0;
}

// u16                       TranslateAccelerator16(hwnd: HWND16, HACCEL16 haccel, MSG16
// *msg);
u16 TranslateAccelerator16(hwnd: HWND16, HACCEL16 haccel, MSG16 *msg)
{
    // TODO: implement
    return 0;
}


// u16                       GetSystemMetrics16(index: u16);
u16 GetSystemMetrics16(index: u16)
{
    // TODO: implement
    return 0;
}


// COLORREF                    GetSysColor16(index: u16);
COLORREF GetSysColor16(index: u16)
{
    // TODO: implement
    return 0;
}


// void                        SetSysColors16(count: u16,list: *mut u16, COLORREF *values);
pub fn SetSysColors16(count: u16,list: *mut u16, COLORREF *values)
{
    // TODO: implement
}


// BOOL16                      GrayString16(hdc: HDC16, HBRUSH16 param_2, LPVOID gsprc,
// LPARAM lparam, cch: u16, x: u16, y: u16, cx: u16, cy: u16);
BOOL16 GrayString16(HDC16    hdc,
                    HBRUSH16 hbrush,
                    LPVOID   gsprc,
                    LPARAM   lparam,
                    cch: u16,
                    x: u16,
                    y: u16,
                    cs: u16,
                   cy: u16)
{
    // TODO: implement
    return false;
}


// HWND16                      SetSysModalWindow(hwnd: HWND16);
HWND16 SetSysModalWindow(hwnd: HWND16)
{
    // TODO: implement
    return 0;
}


// HWND16                      GetNextDlgTabItem16(hwnd_dlg: HWND16, hwnd_ctrl: HWND16,
// BOOL16 f_previous);
HWND16 GetNextDlgTabItem16(hwnd_dlg: HWND16, hwnd_ctrl: HWND16, BOOL16 f_previous)
{
    // TODO: implement
    return 0;
}


// BOOL16                      SetWindowPos16(hwnd: HWND16, hwnd_insert_after: HWND16, u16
// x, y: u16, cx: u16, cy: u16, WORD flags);
BOOL16 SetWindowPos16(hwnd: HWND16,
                      hwnd_insert_after: HWND16,
                      x: u16,
                      y: u16,
                      cx: u16,
                      cy: u16,
                      WORD   flags)
{
    // TODO: implement
    return false;
}


// u16                         GetMenuState16(HMENhmenu: u16, w_item_id: u16, w_flags: u16);
u16 GetMenuState16(HMENhmenu: u16, w_item_id: u16, w_flags: u16)
{
    // TODO: implement
    return 0;
}


// u16                       GetDlgCtrlID16(hwnd: HWND16);
u16 GetDlgCtrlID16(hwnd: HWND16)
{
    // TODO: implement
    return 0;
}


// HPALETTE16                  SelectPalette16(hdc: HDC16, HPALETTE16 hpal, BOOL16
// b_force_background);
HPALETTE16 SelectPalette16(hdc: HDC16, HPALETTE16 hpal, BOOL16 b_force_background)
{
    // TODO: implement
    return 0;
}

// u16                         RealizePalette16(hdc: HDC16);
u16 RealizePalette16(hdc: HDC16)
{
    // TODO: implement
    return 0;
}


// BOOL16                      GetWindowPlacement16(hwnd: HWND16, WINDOWPLACEMENT16 *wp16);
BOOL16 GetWindowPlacement16(hwnd: HWND16, WINDOWPLACEMENT16 *wp16)
{
    // TODO: implement
    return false;
}


// BOOL16                      SetWindowPlacement16(hwnd: HWND16, WINDOWPLACEMENT16 *wp16);
BOOL16 SetWindowPlacement16(hwnd: HWND16, WINDOWPLACEMENT16 *wp16)
{
    // TODO: implement
    return false;
}

// BOOL16                      GetClassInfo16(h_inst16: HINSTANCE16, name: SEGPTR,
// WNDCLASS16 *wc);
BOOL16 GetClassInfo16(h_inst: HINSTANCE16, name: SEGPTR, WNDCLASS16 *wc)
{
    // TODO: implement
    return false;
}

// BOOL16                      InsertMenu16(HMENhmenu: u16, pos: u16, flags: u16, id: u16,
// SEGPTR data);
BOOL16 InsertMenu16(HMENhmenu: u16, pos: u16, flags: u16, id: u16, SEGPTR data)
{
    // TODO: implement
    return false;
}


// BOOL16                      DeleteMenu16(HMENhmenu: u16, npos: u16, wflags: u16);
BOOL16 DeleteMenu16(HMENhmenu: u16, npos: u16, wflags: u16)
{
    // TODO: implement
    return false;
}

// BOOL16                      ModifyMenu16(HMENhmenu: u16, pos: u16, flags: u16, id: u16,
// SEGPTR data);
BOOL16 ModifyMenu16(HMENhmenu: u16, pos: u16, flags: u16, id: u16, SEGPTR data)
{
    // TODO: implement
    return false;
}

// BOOL16                      TrackPopupMenu16(HMENhmenu: u16, wflags: u16, x: u16, y: u16,
// n_reserved: u16, hwnd: HWND16, lp_rect: *mut RECT16);
BOOL16 TrackPopupMenu16(HMENhmenu: u16,
                        wflags: u16,
                        x: u16,
                        y: u16,
                        n_reserved: u16,
                        hwnd: HWND16,
                        lp_rect: *mut RECT16)
{
    // TODO: implement
    return false;
}

// u16                       wsprintf16(buffer: *mut c_char, spec: *mut c_char, valist: *mut u16);
u16 wsprintf16(buffer: *mut c_char, spec: *mut c_char, valist: *mut u16)
{
    // TODO: implement
    return 0;
}


// u16                       wvsprintf16(buffer: *mut c_char, spec: *mut c_char, args: *mut u16);
u16 wvsprintf16(buffer: *mut c_char, spec: *mut c_char, args: *mut u16)
{
    // TODO: implement
    return 0;
}

// HWND16                      CreateWIndowEx16(DWORD ex_style, class_name: *mut c_char, LPCSTR
// window_name, DWORD style, x: u16, y: u16, width: u16, height: u16, parent: HWND16, HMENU16
// hmenu, instance: HINSTANCE16, LPVOID data);
HWND16 CreateWindowEx16(DWORD       ex_style,
                        LPCSTR      class_name,
                        LPCSTR      window_name,
                        DWORD       style,
                        x: u16,
                        y: u16,
                        width: u16,
                        height: u16,
                        parent: HWND16,
                        HMENhmenu: u16,
                        instance: HINSTANCE16,
                        LPVOID      data)
{
    // TODO: implement
    return 0;
}


// BOOL16                      DestroyIcon16(h_icon: HICON16);
BOOL16 DestroyIcon16(h_icon: HICON16)
{
    // TODO: implement
    return false;
}

// BOOL16                      DestroyCursor16(HCURSOR16 h_cursor);
BOOL16 DestroyCursor16(HCURSOR16 h_cursor)
{
    // TODO: implement
    return false;
}

// DWORD                       mciSendCommand16(w_dev_id: u16, w_msg: u16, DWORD dw_param1,
// DWORD p2);
DWORD mciSendCommand16(w_dev_id: u16, w_msg: u16, DWORD dw_param1, DWORD p2)
{
    // TODO: implement
    return 0;
}

// BOOL16                      mciGetErrorString16(DWORD w_error, lp_str_buffer: *mut c_char, u16
// u_length);
BOOL16 mciGetErrorString16(DWORD w_error, lp_str_buffer: *mut c_char, u_length: u16)
{
    // TODO: implement
    return false;
}

// BOOL16                      GetOpenFileName16(SEGPTR ofn);
BOOL16 GetOpenFileName16(SEGPTR ofn)
{
    // TODO: implement
    return false;
}

// BOOL16                      GetSaveFileName16(SEGPTR ofn);
BOOL16 GetSaveFileName16(SEGPTR ofn)
{
    // TODO: implement
    return false;
}

SEGPTR SegmentLimit(SEGPTR in_val)
{
    return 0;
}

HDC16 GetWindowDC16(hwnd: HWND16)
{
    return 0;
}