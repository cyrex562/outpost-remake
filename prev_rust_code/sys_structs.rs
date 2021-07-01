use crate::typedefs::{HDC16, LPARAM, WPARAM16, HWND16, SEGPTR, HBRUSH16, HCURSOR16, HANDLE16, HINSTANCE16};

pub struct PALETTEENTRY {
    pub pe_red: u8,
    pub pe_green: u8,
    pub pe_blue: u8,
    pub pe_flags: u8,
}

pub struct RECT16 {
    pub left: i16,
    pub top: i16,
    pub right: i16,
    pub bottom: i16,
}

impl RECT16 {
    pub fn new() -> RECT16 {
        RECT16 {
            left: 0,
            top: 0,
            right: 0,
            bottom: 0
        }
    }
}

pub struct WNDCLASS16 {
    pub style: u16,
    pub lp_fn_wnd_proc: fn(),
    pub cb_cls_extra: i16,
    pub cb_wnd_extra: i16,
    pub h_instance: HINSTANCE16,
    pub h_icon: HANDLE16,
    pub h_cursor: HCURSOR16,
    pub hbr_background: HBRUSH16,
    pub lp_sz_menu_name: SEGPTR,
    pub lp_sz_class_name: SEGPTR,
}

// typedef struct POINT16 POINT16, *PPOINT16;
pub struct POINT16 {
    pub x: i16,
    pub y: i16,
}

// typedef struct LOGPALETTE LOGPALETTE, *PLOGPALETTE;
pub struct LOGPALETTE {
    pub pal_version: i16,
    pub pal_num_entries: i16,
}

// typedef struct MSG16 MSG16, *PMSG16;
pub struct MSG16 {
    pub hwnd: HWND16,
    pub message: u16,
    pub w_param: WPARAM16,
    pub l_param: LPARAM,
    pub time: u32,
    pub point: POINT16,
}

// typedef struct BITMAPINFOHEADER BITMAPINFOHEADER, *PBITMAPINFOHEADER;

pub struct BITMAPINFOHEADER {
    pub bi_size: u32,
    pub bi_width: i32,
    pub bi_height: i32,
    pub bi_planes: i16,
    pub bi_bit_count: i16,
    pub bi_compression: u32,
    pub bi_size_image: u32,
    pub bi_x_pels_per_meter: i32,
    pub bi_y_pels_per_meter: i32,
    pub bi_clr_used: u32,
    pub bi_clr_important: u32,
}

// typedef struct PAINTSTRUCT16 PAINTSTRUCT16, *PPAINTSTRUCT16;
pub struct PAINTSTRUCT16 {
    pub hdc: HDC16,
    pub f_erase: bool,
    pub rc_paint: RECT16,
    pub f_restore: bool,
    pub f_inc_update: bool,
    pub rgb_reserved: [u8; 16],
}

impl PAINTSTRUCT16 {
    pub fn new() -> PAINTSTRUCT16 {
        PAINTSTRUCT16 {
            hdc: 0xffff,
            f_erase: false,
            rc_paint: RECT16::new(),
            f_restore: false,
            f_inc_update: false,
            rgb_reserved: [0;16]
        }
    }
}

// typedef struct RGBQUAD RGBQUAD, *PRGBQUAD;
pub struct RGBQUAD {
    pub rgb_blue: u8,
    pub rgb_green: u8,
    pub rgb_red: u8,
    pub rgb_reserved: u8,
}

// typedef struct BITMAPINFO BITMAPINFO, *PBITMAPINFO;
pub struct BITMAPINFO {
    pub bmi_header: BITMAPINFOHEADER,
    pub bmi_colors: [RGBQUAD; 1],
}
