use std::intrinsics::offset;

use crate::ui_ops::cursor::load_cursor_1020_7f7a;
use crate::pass::pass8_funcs::pass1_1010_1dda;
use crate::prog_structs::prog_structs_25::Struct65;
use crate::prog_structs::prog_structs_29::Struct48;
use crate::prog_structs::prog_structs_4::Struct652;
use crate::struct_ops::process_struct_1010_20ba;
use crate::util::CONCAT22;
use crate::winapi::DestroyIcon16;

pub fn destroy_icon_func_1020_1038(in_struct_1: *mut Struct48) {
    let pu_var1: *mut u32;
    let mut u_var2: i32;
    let local_struct_1: *mut Struct48;
    let local_struct_1_hi: *mut Struct48;
    let fn_ptr_1: fn();

    local_struct_1_hi = (in_struct_1 >> 0x10);
    local_struct_1 = in_struct_1;
    DestroyIcon16(local_struct_1.handle_0xc2);
    local_struct_1.handle_0xc2 = 0;
    local_struct_1.field_0x8 = 0;
    pu_var1 = local_struct_1.fn_ptr_0xf6;
    u_var2 = &local_struct_1.field_0xf8;
    if ((u_var2 | pu_var1) != 0) {
        unsafe {
            fn_ptr_1 = *pu_var1;
        }
        (**fn_ptr_1)(offset, pu_var1, u_var2, 1);
    }
    &local_struct_1.fn_ptr_0xf6 = 0;
    pass1_1010_1dda(local_struct_1.field_0xf2);
    local_struct_1.field_0xf2 = 0;
    return;
}

pub fn destroy_icon_1020_2c88(param_1: &mut Struct652) {
    let pu_var1: &mut u32;
    let mut u_var2: i32;
    let ppc_var3: fn();
    let mut i_var4: i32;
    let mut u_var5: u16;

    u_var5 = (param_1 >> 0x10);
    i_var4 = param_1;
    DestroyIcon16((i_var4 + 0xc2));
    (i_var4 + 0xc2) = 0;
    (i_var4 + 8) = 0;
    pu_var1 = (i_var4 + 0xf6);
    u_var2 = (i_var4 + 0xf8);
    if ((u_var2 | pu_var1) != 0) {
        unsafe {
            ppc_var3 = *pu_var1;
        }
        (**ppc_var3)(offset, pu_var1, u_var2, 1);
    }
    (i_var4 + 0xf6) = 0;
    pass1_1010_1dda(*(i_var4 + 0xf2));
    (i_var4 + 0xf2) = 0;
    return;
}

pub fn call_load_cursor_1020_2524(in_struct_1: &mut Struct65, param_2: u16, param_3: u32) {
    let mut local_AX_24: u16;
    let mut local_DX_71: u16;
    let mut i_var1: i32;
    let mut local_BP__1: u16;
    let mut local_es_21: u16;
    let ppVar2: &mut pass1_struct_1;

    load_cursor_1020_7f7a(in_struct_1, CONCAT22(param_2, 7), param_3);
    local_es_21 = (in_struct_1 >> 0x10);
    i_var1 = in_struct_1;
    (i_var1 + 0xee) = 0;
    (i_var1 + 0xf2) = 0;
    in_struct_1.ptr_a_lo = s_fem36_wav_1050_270c;
    (i_var1 + 2) = 0x1020;
    (i_var1 + 0xe2) = (s_fem51_wav_1050_27a2 + 6);
    (i_var1 + 0xe4) = 0x1020;
    ppVar2 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(local_BP__1, 0x2a));
    local_DX_71 = (ppVar2 >> 0x10);
    (i_var1 + 0xf2) = ppVar2;
    (i_var1 + 0xf4) = local_DX_71;
    (i_var1 + 0xe6) = (i_var1 + 0xf2);
    (i_var1 + 0xe8) = local_DX_71;
    return;
}
