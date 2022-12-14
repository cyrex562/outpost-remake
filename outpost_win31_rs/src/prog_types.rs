//
// Created by cyrex on 2022-05-22.
//

use std::ffi::c_void;
use std::os::raw::c_char;
use crate::windef::POINTL;

// typedef let mut BOOL16: u16;
//LOGPALETTE;
// PALETTEENTRY;
//RECT16;
// POINT16;
//WINDOWPLACEMENT16;
// WNDCLASS16;

// typedef let mut LONG: i32;
// typedef DWORD: u32;
// typedef HANDLE16 HINSTANCE16;
//BITMAPINFO;
//MSG16;

// typedef void * LPVOID;
//PAINTSTRUCT16;



// typedef HANDLE16 HTASK16;

// typedef HANDLE16 HPALETTE16;

// typedef HANDLE16 HRSRC16;
// POINTL;



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
    // let mut dmOrientation: i16;
    pub dmOrientation: i16,
    // let mut dmPaperSize: i16;
    pub dmPaperSize: i16,
    // let mut dmPaperLength: i16;
    pub dmPaperLength: i16,
    // let mut dmPaperWidth: i16;
    pub dmPaperWidth: i16,
    // let mut dmScale: i16;
    pub dmScale: i16,
    // let mut dmCopies: i16;
    pub dmCopies: i16,
    // let mut dmDefaultSource: i16;
    pub dmDefaultSource: i16,
    // let mut dmPrintQuality: i16;
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
} //DEVMODEA;
// FLOATING_SAVE_AREA;

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
//RECT;

// typedef void * PVOID;

// #define NULL 0

// #endif //OUTPOST_C_PROJ__TYPES_H_
