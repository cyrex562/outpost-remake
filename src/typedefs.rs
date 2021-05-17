use std::convert::From;

// typedef UINT16 HANDLE16;
pub type HANDLE16 = u16;

//typedef pub HCURSOR16: HANDLE16,
pub type HCURSOR16 = HANDLE16;

//typedef pub HBRUSH16: HANDLE16,
pub type HBRUSH16 = HANDLE16;

// typedef pub HMENU16: HANDLE16,
pub type HMENU16 = HANDLE16;

// typedef UINT16 WPARAM16;
pub type WPARAM16 = u16;

//typedef pub HWND16: HANDLE16,
pub type HWND16 = HANDLE16;
pub fn hwnd16_from_le_bytes(bytes: [u8;2]) -> HWND16 {
    u16::from_le_bytes(bytes) as HWND16
}

// typedef pub HMODULE16: HANDLE16,
pub type HMODULE16 = HANDLE16;

// typedef pub HACCEL16: HANDLE16,
pub type HACCEL16 = HANDLE16;

// typedef pub HINSTANCE16: HANDLE16,
pub type HINSTANCE16 = HANDLE16;

// typedef pub SEGPTR: u32,
pub type SEGPTR = u32;

// typedef pub COLORREF: u32,
pub type COLORREF = u32;

// typedef pub HDC16: HANDLE16,
pub type HDC16 = HANDLE16;

// typedef pub HFILE16: HANDLE16,
pub type HFILE16 = HANDLE16;

// typedef pub HPEN16: HANDLE16,
pub type HPEN16 = HANDLE16;

// typedef pub HGLOBAL16: HANDLE16,
pub type HGLOBAL16 = HANDLE16;

// typedef pub HGDIOBJ16: HANDLE16,
pub type HGDIOBJ16 = HANDLE16;

// typedef LPARAM: libc::c_long;
pub type LPARAM = i32;

// typedef ATOM: u16;
pub type ATOM = i16;

//typedef LRESULT: libc::c_long;
pub type LRESULT = i32;

// pub typedef pub HTASK16: HANDLE16,
pub type HTASK16 = HANDLE16;

// pub typedef pub HPALETTE16: HANDLE16,
pub type HPALETTE16 = HANDLE16;

// pub typedef DLGPROC16: *mut libc::c_void,
pub type DLGPROC16 = *mut libc::c_void;

// pub typedef pub HRSRC16: HANDLE16,
pub type HRSRC16 = HANDLE16;

// pub typedef pub HICON16: HANDLE16,
pub type HICON16 = HANDLE16;
