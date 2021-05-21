use crate::app_context::AppContext;
use crate::draw::{draw2, misc};
use crate::pass::pass14_funcs::pass1_1008_4480;
use crate::pass::pass8_funcs::{pass1_1010_454a, process_struct_1010_20ba};
use crate::prog_structs::prog_structs_24::Struct129;
use crate::prog_structs::prog_structs_30::{Struct14, Struct417};
use crate::sound_funcs::win_fn_1018_6adc;
use crate::sys_ops::win::win_func_1018_6bb6;
use crate::sys_structs::PAINTSTRUCT16;
use crate::typedefs::HWND16;
use crate::util::{CONCAT22, SUB42};
use crate::winapi::{BeginPaint16, EndPaint16, IsIconic16};

pub unsafe fn begin_end_Paint_1018_6a7a(ctx: &mut AppContext, param_1: &mut Struct129) {
    // let local_p_Struct129: *mut Struct129;
    // let mut u_var1: u16;
    let mut unaff_ss: HWND16;
    let ppVar2: *mut pass1_struct_1;
    let mut HVar3: HWND16;
    let mut local_28: u16;
    let mut local_26: u16;
    let mut local_24: u16;
    let mut struct_var_1: PAINTSTRUCT16 = PAINTSTRUCT16::new();

    // u_var1 = (param_1 >> 0x10);
    // local_p_Struct129 = param_1;
    BeginPaint16(param_1.h_wnd_0x8, &struct_var_1);
    HVar3 = param_1.h_wnd_0x8;
    EndPaint16(struct_var_1, unaff_ss);
    ppVar2 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(HVar3, 2));
    if ((ppVar2 + 0x20) == 0) {
        win_func_1018_6bb6(param_1);
        return;
    }
    win_fn_1018_6adc(param_1);
    return;
}

pub unsafe fn call_Paint_fn_1020_1022(in_struct_1: *mut Struct14) {
    paint_func_1020_15de((in_struct_1 + 0xf6));
    return;
}

pub unsafe fn paint_func_1020_15de(ctx: &mut AppContext, in_struct_1: *mut Struct14) {
    let pvVar1: &mut Vec<u8>;
    let bool_result_1: bool;
    let mut u_var2: i32;
    let local_struct_1: *mut Struct14;
    let local_struct_1_hi: *mut Struct14;
    let mut u_var3: u16;
    let mut h_window: u16;
    let mut u_var4: u32;
    let mut local_28: u16;
    let mut local_26: u16;
    let mut local_24: u16;
    let mut Paint_struct: PAINTSTRUCT16;
    let fn_ptr_1: fn();
    let local_struct_2: *mut Struct417;

    local_struct_1_hi = (in_struct_1 >> 0x10);
    local_struct_1 = in_struct_1;
    BeginPaint16(CONCAT22(h_window, Paint_struct), local_struct_1.h_wnd);
    bool_result_1 = IsIconic16(local_struct_1.h_wnd);
    if (bool_result_1 == 0) {
        u_var3 = 0x1010;
        u_var4 = pass1_1010_454a(local_struct_1.struct_ptr_0x14);
        u_var2 = (u_var4 >> 0x10);
        if ((u_var2 | u_var4) != 0) {
            local_struct_2 = local_struct_1.struct_ptr_0x14;
            u_var3 = SUB42(&ctx.PTR_LOOP_1050_1008, 0);
            pass1_1008_4480(
                local_struct_1.field_0x18,
                (local_struct_2 & 0xffff0000 | (local_struct_2 + 0x76)),
                u_var4 & 0xffff | u_var2 << 0x10,
            );
        }
        pvVar1 = local_struct_1.field_0x18;
        // WARNING: Load size is inaccurate
        fn_ptr_1 = (*local_struct_1.field_0x18 + 4);
        (**fn_ptr_1)(u_var3, pvVar1, (pvVar1 >> 0x10), 0, 0);
    } else {
        misc::draw_icon_1020_1674(in_struct_1);
    }
    EndPaint16(Paint_struct, h_window);
    return;
}
