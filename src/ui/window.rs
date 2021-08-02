use crate::defines::{Struct1, U32Ptr};
use crate::global::AppContext;
use crate::pass::pass_1010::pass1_1010_4f30;
use crate::string::string_1010::load_string_1010_847e;
use crate::ui::ui_1040;
use crate::util::{CONCAT22, ZEXT24};
use crate::win_struct::{HINSTANCE16, HWND16};
use crate::winapi::{CreateWindow16, EnableWindow16, GetDlgItem16, IsWindow16, ShowWindow16};

pub fn create_window_1040_7620(
    ctx: &mut AppContext,
    param_1: u32,
    param_2: i16,
    n_menu_handle: U32Ptr,
    param_4: u16,
    param_5: i16
)

{
    let i_var1: i16;
    let u_var2: u16;
    let h_instance: HINSTANCE16;

    load_string_1010_847e(ctx.PTR_LOOP_1050_14cc, (ctx.PTR_LOOP_1050_14cc >> 0x10), 0x1010);
    _h_instance = 0x50000009;
    if param_2 != 0x0 {
        _h_instance = 0x50020009;
    }
    // u_var2 = (in_menu_handle >> 0x10);
    i_var1 = in_menu_handle;
    CreateWindow16(0x1010, 0x0, ZEXT24(ctx.PTR_LOOP_1050_038c) << 0x10, param_5,
                   (param_1 + 0x6), (i_var1 + 0x6),
                   (i_var1 + 0x4), (i_var1 + 0x2), *in_menu_handle,
                   _h_instance, (_h_instance >> 0x10));
    return;
}


pub fn enable_window_1040_0acc(
    ctx: &mut AppContext,
    param_1: u16,
    param_2: u16,
    enable_window: bool,
    win_handle: HWND16
) {
    let b_var1: bool;

    b_var1 = IsWindow16(win_handle);
    if b_var1 != false {
        GetDlgItem16(ctx.s_tile2_bmp_1050_1538 as HWND16, 0x64);
        b_var1 = IsWindow16(ctx.s_tile2_bmp_1050_1538 as HWND16);
        if b_var1 != false {
            EnableWindow16(ctx.s_tile2_bmp_1050_1538 as HWND16, enable_window);
            GetDlgItem16(ctx.s_tile2_bmp_1050_1538 as HWND16, 0x74);
            EnableWindow16(ctx.s_tile2_bmp_1050_1538 as HWND16, enable_window);
            GetDlgItem16(ctx.s_tile2_bmp_1050_1538 as HWND16, 0x73);
            EnableWindow16(ctx.s_tile2_bmp_1050_1538 as HWND16, enable_window);
            GetDlgItem16(ctx.s_tile2_bmp_1050_1538 as HWND16, 0x6e);
            EnableWindow16(ctx.s_tile2_bmp_1050_1538 as HWND16, enable_window);
            GetDlgItem16(ctx.s_tile2_bmp_1050_1538 as HWND16, 0xee);
            EnableWindow16(ctx.s_tile2_bmp_1050_1538 as HWND16, enable_window);
        }
    }
    return;
}


pub fn show_win_1040_0c7c(param_1: &mut Struct1, param_2: u16, param_3: u16) {
    let u_var1: u32;
    let local_6: u32;

    ui_1040::dialog_ui_fn_1040_78e2(param_1, param_2);
    u_var1 = (param_1 + 0x8e);
    pass1_1010_4f30(u_var1,
                    CONCAT22(param_3, &local_6),
                    CONCAT22(param_3, &local_6 + 0x2));
    ui_1040::move_win_1040_826c(param_1, local_6);
    ShowWindow16(0x1010, 0x5);
    return;
}
