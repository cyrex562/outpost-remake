use std::intrinsics::offset;

use crate::app_context::AppContext;
use crate::big_funcs::big_fn_1008_15d4;
use crate::draw::{draw2, rect};
use crate::mem_funcs::mem_ops_1::get_fn_ptr_at_address;
use crate::pass::pass14_funcs::pass1_1008_687a;
use crate::pass::pass17_funcs::{pass1_1030_8308, pass1_1030_8334, pass1_1030_835a};
use crate::pass::pass20_funcs::pass1_1018_017c;
use crate::pass::pass7_funcs::pass1_1018_57e6;
use crate::pass::pass8_funcs::{pass1_1010_5f1e, process_struct_1010_20ba};
use crate::string_ops::misc::{copy_string_1000_3d3e, get_string_index_1000_3da4, process_string_1000_3cea, process_string_1000_4d58};
use crate::structs::prog_structs_17::Struct590;
use crate::structs::prog_structs_23::WinStruct42;
use crate::structs::prog_structs_25::Struct65;
use crate::structs::prog_structs_2::Struct665;
use crate::structs::prog_structs_31::Struct16;
use crate::structs::prog_structs_3::Struct664;
use crate::structs::prog_structs_6::Struct673;
use crate::sys_ops::{pass1_1030_838e, reg_class_1008_818c, reg_class_1008_96d2, reg_class_1040_98c0};
use crate::sys_ops::win_msg::send_win_msg_1020_097e;
use crate::typedefs::{HCURSOR16, HGDIOBJ16, HWND16};
use crate::ui_ops::{file_menu, misc, window};
use crate::util::{CONCAT12, CONCAT13, CONCAT22, ZEXT24};
use crate::winapi::{GetStockObject16, LoadCursor16, PostMessage16, ReleaseCapture16, SetCursor16, UpdateWindow16};

pub unsafe fn set_cursor_1008_2dcc(ctx: &mut AppContext, param_1: &mut Struct16, param_2: u16, param_3: u16) {
    let mut u_var1: u32;
    let mut ppc_var2: fn();
    let mut hcursor_var3: HCURSOR16;
    let mut hcursor_var4: HCURSOR16;
    let mut local_6: u16;

    hcursor_var3 = LoadCursor16(0, 0x7f02, );
    hcursor_var4 = SetCursor16(hcursor_var3);
    hcursor_var3 = hcursor_var4;
    if param_1.field_0xe8 != 0 {
        u_var1 = param_1.field_0xe8;
        ppc_var2 = get_fn_ptr_at_address(param_1.field_0xe8 + 0x90);
        ppc_var2(offset, u_var1, (u_var1 >> 0x10));
    }
    big_fn_1008_15d4(ctx, CONCAT22(param_2_00, param_1), param_2);
    param_1.field_0xe8 = hcursor_var3;
    (param_1.field_0xe8 + 2) = ctx.dx_reg;
    u_var1 = param_1.field_0xe8;
    if (u_var1 + 0xe0) == 0 {
        u_var1 = param_1.field_0xe8;
        ppc_var2 = (param_1.field_0xe8 + 8);
        ppc_var2(offset, u_var1, (u_var1 >> 0x10));
        u_var1 = param_1.field_0xe8;
        ppc_var2 = (param_1.field_0xe8 + 0xc);
        ppc_var2(offset, u_var1, (u_var1 >> 0x10), 3);
        &param_1.field_0xce = param_1.field_0xe8;
    } else {
        param_1.field_0xe8 = 0;
        misc::gui_fn_1008_2c4e(param_1, param_2_00, param_2);
        param_1.field_0xce = 0;
    }
    SetCursor16(hcursor_var4);
    return;
}

pub fn set_cursor_1008_2e9a(ctx: &mut AppContext, param_1: u32) {
    let mut u_var1: u32;
    let mut u_var2: u16;
    let mut u_var3: i32;
    let mut i_var4: i32;
    let mut unaff_si: u16;
    let mut unaff_ss: u16;
    let u_var5: u8;
    let u_var6: u8;
    let mut u_var7: u16;
    let mut in_stack_0000fdd2: u8;
    let mut local_224: [u8; 264];
    let mut local_11c: u16;
    let mut local_11a: u32;
    let mut local_112: u16;
    let mut local_110: u16;
    let mut local_10e: u32;
    let mut local_10a: u32;
    let mut local_106: u16;
    let mut local_104: u16;
    let local_102: u8;
    let local_101: u8;

    local_102 = '\0';
    _local_106 = process_struct_1010_20ba(ctx.g_struct_var_1050_0ed0, CONCAT22(unaff_si, 2));
  // u_var2 = (_local_106  >> 0x10);
    i_var4 = _local_106;
    local_10a = (i_var4 + 0x16);
    u_var3 = (i_var4 + 0x18) | local_10a;
    u_var5 = unaff_ss;
    u_var6 = (unaff_ss >> 8);
  // u_var7 = (param_1  >> 0x10);
    if u_var3 == 0 {
        file_menu::open_save_fn_1008_3178(param_1, u_var7, 1);
        local_10a = CONCAT22(ctx.dx_reg, u_var3);
        if (ctx.dx_reg | u_var3) == 0 {
            PostMessage16(0, 0x13d, 0x111, ctx.g_h_window);
            return;
        }
        copy_string_1000_3d3e(
            CONCAT13(u_var6, CONCAT12(u_var5, &local_102)),
            CONCAT22(ctx.dx_reg, u_var3),
        );
        process_string_1000_4d58(&local_102);
        if in_stack_0000fdd2 != '\0' {
            process_string_1000_3cea(
                CONCAT13(u_var6, CONCAT12(u_var5, local_224)),
                CONCAT22(unaff_ss, &stack0xfdd2),
            );
        }
        pass1_1010_5f1e(_local_106, CONCAT22(unaff_ss, local_224));
    } else {
        local_11a = *(i_var4 + 0x1a);
        copy_string_1000_3d3e(CONCAT13(u_var6, CONCAT12(u_var5, &local_102)), local_11a);
        local_11c = get_string_index_1000_3da4(CONCAT22(unaff_ss, &local_102));
        if *(&local_104 + local_11c + 1) != '\\' {
            (&local_102)[local_11c] = '\\';
            (&local_101)[local_11c] = '\0';
        }
        process_string_1000_3cea(CONCAT13(u_var6, CONCAT12(u_var5, &local_102)), local_10a);
    }
    if local_102 != '\0' {
        i_var4 = param_1;
        local_10e = (i_var4 + 0xe8);
        send_win_msg_1020_097e(local_10e, (local_10e >> 0x10));
        u_var1 = (i_var4 + 0xe8);
        UpdateWindow16((u_var1 + 8));
        local_110 = LoadCursor16(0x7f02, 0);
        local_112 = SetCursor16(local_110);
        misc::win_fn_1008_1414(i_var4, u_var7);
        SetCursor16(local_112);
    }
    return;
}

pub fn load_cursor_1008_61b2(
    struct_param_1: &mut Struct65,
    in_u16_2: u16,
    param_3: u16,
    param_4: &mut Vec<u8>,
) -> &mut Struct65 {
    let mut u_var1: u16;
    let HVar2: HGDIOBJ16;
    let HVar3: HCURSOR16;
    let mut local_struct_65_ptr_1:  Struct65;
    let mut u_var4: u16;
    let mut ppVar5:  Struct2551;
    let mut in_stack_0000ffe8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    pass1_1008_687a(struct_param_1, param_4);
  // u_var4 = (struct_param_1  >> 0x10);
    local_struct_65_ptr_1 = struct_param_1;
    local_struct_65_ptr_1.u16_xde = in_u16_2;
    local_struct_65_ptr_1.u16_xe0 = 0;
    struct_param_1.ptr_a_lo = 0x6378;
    local_struct_65_ptr_1.ptr_a_hi = &ctx.PTR_LOOP_1050_1008;
    copy_string_1000_3d3e(
        (struct_param_1 & 0xffff0000 | ZEXT24(&local_struct_65_ptr_1.char_ptr_x5b)),
        s_DanBrotherton_1050_0302,
    );
    HVar2 = GetStockObject16(4);
    local_struct_65_ptr_1.h_gdi_obj_xc6 = HVar2;
    HVar3 = LoadCursor16(0x7f00, 0);
    local_struct_65_ptr_1.h_cursor_xc4 = HVar3;
    local_struct_65_ptr_1.u16_xc8 = (s_572_bmp_1050_2007 + 4);
    local_struct_65_ptr_1.u32_xac = 0x45000000;
    local_struct_65_ptr_1.Struct590_ptr_xbc = (param_4 + 8);
    ppVar5 = process_struct_1010_20ba(
        ctx.g_struct_var_1050_0ed0,
        CONCAT22(in_stack_0000ffe8, 0x48),
    );
  // u_var1 = (ppVar5  >> 0x10);
    local_struct_65_ptr_1.u16_xb4 = 0;
    local_struct_65_ptr_1.u16_xb6 = 0;
    local_struct_65_ptr_1.u16_xb8 = (ppVar5 + 10);
    local_struct_65_ptr_1.u16_xba = (ppVar5 + 0xc);
    &local_struct_65_ptr_1.u32_xca = param_3;
    reg_class_1008_96d2(struct_param_1, in_stack_0000ffe8);
    return struct_param_1;
}

pub fn load_cursor_1008_7f62(param_1: &mut Struct65, param_2: u16, param_3: u32) -> &mut Struct65 {
    let HVar1: HGDIOBJ16;
    let HVar2: HCURSOR16;
    let mut i_var3: i32;
    let mut u_var4: u16;
    let mut in_stack_0000fffc: u16;

    pass1_1008_687a(param_1, param_3);
  // u_var4 = (param_1  >> 0x10);
    i_var3 = param_1;
    (i_var3 + 0xde) = param_2;
    param_1.ptr_a_lo = 0x8042;
    (i_var3 + 2) = &ctx.PTR_LOOP_1050_1008;
    copy_string_1000_3d3e(
        (param_1 & 0xffff0000 | (i_var3 + 0x5b)),
        s_SOLChildPar_1050_0358,
    );
    HVar1 = GetStockObject16(5);
    (i_var3 + 0xc6) = HVar1;
    HVar2 = LoadCursor16(0x7f00, 0);
    (i_var3 + 0xc4) = HVar2;
    (i_var3 + 200) = (s_572_bmp_1050_2007 + 1);
    (i_var3 + 0xac) = 0x44000000;
    (i_var3 + 0xbc) = (param_3 + 8);
    (i_var3 + 0xca) = (i_var3 + 0xde);
    reg_class_1008_96d2(param_1, in_stack_0000fffc);
    return param_1;
}

pub fn load_cursor_1008_80ee(param_1: &mut u16) -> &mut u16 {
    let HVar1: HCURSOR16;
    let HVar2: HGDIOBJ16;
    let mut i_var3: i32;
    let mut u_var4: u16;

  // u_var4 = (param_1  >> 0x10);
    i_var3 = param_1;
    unsafe {
        *param_1 = ctx.s_1_1050_389a;
    }
    (i_var3 + 2) = &ctx.PTR_LOOP_1050_1008;
    (i_var3 + 0x54) = 0;
    (i_var3 + 0x56) = 0;
    (i_var3 + 0x58) = 0;
    unsafe {
        *param_1 = 0x87c8;
    }
    (i_var3 + 2) = &ctx.PTR_LOOP_1050_1008;
    copy_string_1000_3d3e(
        (param_1 & 0xffff0000 | (i_var3 + 4)),
        s_MicroSpinControl_1050_0370,
    );
    (i_var3 + 0x54) = 3;
    HVar1 = LoadCursor16(0x7f00, 0);
    (i_var3 + 0x58) = HVar1;
    HVar2 = GetStockObject16(4);
    (i_var3 + 0x56) = HVar2;
    reg_class_1008_818c(i_var3, u_var4);
    return param_1;
}

pub fn load_cursor_1018_5840(param_1: &mut Struct65, param_2: u16, param_3: u32) {
    let mut u_var1: u16;
    let mut i_var2: i32;
    let mut unaff_bp: u16;
    let mut u_var3: u16;
    let mut ppVar4:  Struct2551;

    load_cursor_1020_7f7a(param_1, CONCAT22(param_2, 6), param_3);
  // u_var3 = (param_1  >> 0x10);
    i_var2 = param_1;
    (i_var2 + 0xee) = 0;
    (i_var2 + 0xf2) = 0;
    (i_var2 + 0xf6) = 0;
    param_1.ptr_a_lo = (s_Alloc__s_1050_5a5b + 7);
    (i_var2 + 2) = 0x1018;
    (i_var2 + 0xe2) = 0x5afe;
    (i_var2 + 0xe4) = 0x1018;
    ppVar4 = process_struct_1010_20ba(ctx.g_struct_var_1050_0ed0, CONCAT22(unaff_bp, 0x27));
  // u_var1 = (ppVar4  >> 0x10);
    (i_var2 + 0xf2) = ppVar4;
    (i_var2 + 0xf4) = u_var1;
    (i_var2 + 0xe6) = (i_var2 + 0xf2);
    (i_var2 + 0xe8) = u_var1;
    return;
}

pub fn load_cursor_fn_1020_01d8(
    in_struct_1: &mut Struct65,
    in_struct_1_hi: &mut Struct65,
    param_3: u16,
    param_4: u16,
    param_5: u16,
    param_6: u16,
    param_7: u16,
    param_8: &mut Vec<u8>,
) {
    load_cursor_1008_61b2(
        CONCAT22(in_struct_1_hi, in_struct_1),
        param_3,
        param_7,
        param_8,
    );
    &in_struct_1.ptr_b_lo = 0;
    &in_struct_1.u16_xe6 = 0;
    in_struct_1.u16_xea = param_6;
    in_struct_1.u16_xec = param_5;
    in_struct_1.u16_xee = param_3_00;
    CONCAT22(in_struct_1_hi, in_struct_1) = 0x45a;
    in_struct_1.ptr_a_hi = 0x1020;
    return;
}

pub fn load_cursor_1040_9854(param_1: &mut u16) -> &mut u16 {
    let HVar1: HCURSOR16;
    let HVar2: HGDIOBJ16;
    let mut i_var3: i32;
    let mut u_var4: u16;

  // u_var4 = (param_1  >> 0x10);
    i_var3 = param_1;
    unsafe {
        *param_1 = ctx.s_1_1050_389a;
    }
    (i_var3 + 2) = &ctx.PTR_LOOP_1050_1008;
    unsafe {
        *param_1 = 0xa230;
    }
    (i_var3 + 2) = &ctx.PTR_LOOP_1050_1040;
    copy_string_1000_3d3e((param_1 & 0xffff0000 | (i_var3 + 4)), s_OPButton_1050_5ece);
    (i_var3 + 0x54) = 3;
    HVar1 = LoadCursor16(0x7f00, 0);
    (i_var3 + 0x58) = HVar1;
    HVar2 = GetStockObject16(4);
    (i_var3 + 0x56) = HVar2;
    reg_class_1040_98c0(i_var3, u_var4);
    return param_1;
}

pub fn set_cursor_1038_bc30(param_1: u32) {
    let mut u_var1: u32;
    let mut unaff_ss: u16;
    let mut local_112: u16;
    let mut local_110: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    local_4 = LoadCursor16(0x7f02, 0);
    local_6 = SetCursor16(local_4);
    u_var1 = (param_1 + 0x8e);
    pass1_1030_532e(CONCAT22(unaff_ss, &local_112), (u_var1 + 0xe) + 0x1000000);
    pass1_1030_835a(ctx._g_bool_1050_5748, CONCAT22(unaff_ss, &local_112));
    pass1_1030_838e(ctx._g_bool_1050_5748);
    local_112 = ctx.s_1_1050_389a;
    local_110 = &ctx.PTR_LOOP_1050_1008;
    pass1_1030_8334();
    SetCursor16(local_6);
    return;
}

pub fn call_load_cursor_1020_790e(
    in_struct_1: &mut WinStruct42,
    param_2: String,
    param_3: u16,
    param_4: u32,
) {
    let local_struct_1: &mut WinStruct42;
    let local_struct_1_hi: &mut WinStruct42;

    load_cursor_1008_7f62(in_struct_1, param_3, param_4);
  // local_struct_1_hi = (in_struct_1  >> 0x10);
    local_struct_1 = in_struct_1;
    &local_struct_1.field_0xe0 = 0;
    &local_struct_1.u16_xe4 = 0;
    &local_struct_1.field_0xe8 = 0;
    &local_struct_1.field_0xec = 0;
    *&local_struct_1.char_ptr_16_0xee = param_2;
    in_struct_1.u16_x0 = 0x7b86;
    local_struct_1.u16_x2 = 0x1020;
    return;
}

pub fn load_cursor_fn_1018_6a0e(
    param_1: &mut Struct65,
    param_2: u16,
    param_3: u16,
    param_4: u16,
    param_5: u16,
    param_6: u16,
    param_7: &mut libc::c_void,
) -> &mut Struct65 {
    let mut local_bx_25:  Struct65;
    let mut local_es_25: u16;

    load_cursor_1008_61b2(param_1, param_3, param_6, param_7);
  // local_es_25 = (param_1  >> 0x10);
    local_bx_25 = param_1;
    local_bx_25.u16_xea = param_5;
    local_bx_25.u16_xec = param_4;
    local_bx_25.u16_xee = param_2;
    local_bx_25.u16_xf0 = 0;
    param_1.ptr_a_lo = 0x6c66;
    local_bx_25.ptr_a_hi = 0x1018;
    local_bx_25.u16_xe0 = 1;
    local_bx_25.ptr_b_lo = 0;
    local_bx_25.ptr_b_hi = 0;
    &local_bx_25.u16_xe6 = 0x1df027f;
    return param_1;
}

pub fn load_cursor_fn_1018_6d02(
    in_struct_65: &mut Struct65,
    param_2: u16,
    param_3: &mut Vec<u8>,
) -> &mut Struct65 {
    load_cursor_fn_1018_6a0e(in_struct_65, 0, 0xb, 0x9c, 0x8b, param_2, param_3);
    in_struct_65.ptr_a_lo = 0xa27e;
    (in_struct_65 + 2) = 0x1018;
    return in_struct_65;
}

pub fn load_cursor_fn_1018_6d38(
    param_1: &mut Struct65,
    param_2: u16,
    param_3: &mut Vec<u8>,
) -> &mut Struct65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0xc, 0x9d, 0xd0, param_2, param_3);
    param_1.ptr_a_lo = 0xb562;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_6d6e(
    param_1: &mut Struct65,
    param_2: u16,
    param_3: &mut Vec<u8>,
) -> &mut Struct65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0xd, 0x9e, 0xd1, param_2, param_3);
    param_1.ptr_a_lo = 0x9822;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_6da4(
    param_1: &mut Struct65,
    param_2: u16,
    param_3: &mut Vec<u8>,
) -> &mut Struct65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0xe, 0x9f, 0xd2, param_2, param_3);
    param_1.ptr_a_lo = 0xab06;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_6dda(
    param_1: &mut Struct65,
    param_2: u16,
    param_3: &mut Vec<u8>,
) -> &mut Struct65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0xf, 0xa0, 0xd4, param_2, param_3);
    param_1.ptr_a_lo = 0xbdea;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_6e10(
    param_1: &mut Struct65,
    param_2: u16,
    param_3: &mut Vec<u8>,
) -> &mut Struct65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0x10, 0xa1, 0xda, param_2, param_3);
    param_1.ptr_a_lo = 0xa0aa;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_6e46(
    param_1: &mut Struct65,
    param_2: u16,
    param_3: &mut Vec<u8>,
) -> &mut Struct65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0x11, 0xa2, 0xdc, param_2, param_3);
    param_1.ptr_a_lo = 0xb38e;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_6e7c(
    param_1: &mut Struct65,
    param_2: u16,
    param_3: &mut Vec<u8>,
) -> &mut Struct65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0x12, 0xa3, 0xd3, param_2, param_3);
    param_1.ptr_a_lo = 0x964e;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_6eb2(
    param_1: &mut Struct65,
    param_2: u16,
    param_3: &mut Vec<u8>,
) -> &mut Struct65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0x13, 0xa4, 0xdb, param_2, param_3);
    param_1.ptr_a_lo = 0xa932;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_6ee8(
    param_1: &mut Struct65,
    param_2: u16,
    param_3: &mut Vec<u8>,
) -> &mut Struct65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0x14, 0xa5, 0xa5, param_2, param_3);
    param_1.ptr_a_lo = 0xbc16;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_6f1e(
    param_1: &mut Struct65,
    param_2: u16,
    param_3: &mut Vec<u8>,
) -> &mut Struct65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0x15, 0xa7, 0xb2, param_2, param_3);
    param_1.ptr_a_lo = 0x9e3a;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_6f54(
    param_1: &mut Struct65,
    param_2: u16,
    param_3: &mut Vec<u8>,
) -> &mut Struct65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0x16, 0xa8, 0, param_2, param_3);
    param_1.ptr_a_lo = 0xb11e;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_6f8a(
    param_1: &mut Struct65,
    param_2: u16,
    param_3: &mut Vec<u8>,
) -> &mut Struct65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0x17, 0xaf, 0xc0, param_2, param_3);
    param_1.ptr_a_lo = 0x93de;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_6fc0(
    param_1: &mut Struct65,
    param_2: u16,
    param_3: &mut Vec<u8>,
) -> &mut Struct65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0x18, 0xb0, 0xc1, param_2, param_3);
    param_1.ptr_a_lo = 0xa6c2;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_6ff6(
    param_1: &mut Struct65,
    param_2: u16,
    param_3: &mut Vec<u8>,
) -> &mut Struct65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0x19, 0xb1, 0x80, param_2, param_3);
    param_1.ptr_a_lo = 0xb9a6;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_702c(
    param_1: &mut Struct65,
    param_2: u16,
    param_3: &mut Vec<u8>,
) -> &mut Struct65 {
    load_cursor_fn_1018_6a0e(param_1, 0x1ec, 0x1a, 0xb2, 0xc3, param_2, param_3);
    param_1.ptr_a_lo = 0x9c66;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_7062(
    param_1: &mut Struct65,
    param_2: u16,
    param_3: &mut Vec<u8>,
) -> &mut Struct65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0x1b, 0xb3, 0xc4, param_2, param_3);
    param_1.ptr_a_lo = 0xaf4a;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_7098(
    param_1: &mut Struct65,
    param_2: u16,
    param_3: &mut Vec<u8>,
) -> &mut Struct65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0x1c, 0xb4, 0xd8, param_2, param_3);
    param_1.ptr_a_lo = 0xc22e;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_70ce(
    param_1: &mut Struct65,
    param_2: u16,
    param_3: &mut Vec<u8>,
) -> &mut Struct65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0x1d, 0xb5, 0x7b, param_2, param_3);
    param_1.ptr_a_lo = 0xa4ee;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_7104(
    param_1: &mut Struct65,
    param_2: u16,
    param_3: &mut Vec<u8>,
) -> &mut Struct65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0x1e, 0xb6, 0xd9, param_2, param_3);
    param_1.ptr_a_lo = 0xb7d2;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_713a(
    param_1: &mut Struct65,
    param_2: u16,
    param_3: &mut Vec<u8>,
) -> &mut Struct65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0x1f, 0xb7, 0x7d, param_2, param_3);
    param_1.ptr_a_lo = 0x9a92;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_7170(
    param_1: &mut Struct65,
    param_2: u16,
    param_3: &mut Vec<u8>,
) -> &mut Struct65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0x21, 0xb9, 0xdd, param_2, param_3);
    param_1.ptr_a_lo = 0xad76;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_71a6(
    param_1: &mut Struct65,
    param_2: u16,
    param_3: &mut Vec<u8>,
) -> &mut Struct65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0x23, 0xd3, 0xd6, param_2, param_3);
    param_1.ptr_a_lo = 0xb69a;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_71dc(
    param_1: &mut Struct65,
    param_2: u16,
    param_3: &mut Vec<u8>,
) -> &mut Struct65 {
    load_cursor_fn_1018_6a0e(param_1, 0x1ed, 0x24, 0xd4, 0xd7, param_2, param_3);
    param_1.ptr_a_lo = 0x995a;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_7212(
    param_1: &mut Struct65,
    param_2: u16,
    param_3: &mut Vec<u8>,
) -> &mut Struct65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0x25, 0xe9, 0xee, param_2, param_3);
    param_1.ptr_a_lo = 0xa452;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_1018_7248(
    param_1: &mut Struct65,
    param_2: u16,
    param_3: &mut Vec<u8>,
) -> &mut Struct65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 99, 0xa6, 0, param_2, param_3);
    param_1.ptr_a_lo = 0xc05a;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_727e(
    param_1: &mut Struct65,
    param_2: u16,
    param_3: &mut Vec<u8>,
) -> &mut Struct65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 100, 0xa9, 0, param_2, param_3);
    param_1.ptr_a_lo = 0xa31a;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_1018_72b4(
    param_1: &mut Struct65,
    param_2: u16,
    param_3: &mut Vec<u8>,
) -> &mut Struct65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0x65, 0xaa, 0xbb, param_2, param_3);
    param_1.ptr_a_lo = 0xb5fe;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_72ea(
    param_1: &mut Struct65,
    param_2: u16,
    param_3: &mut Vec<u8>,
) -> &mut Struct65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0x66, 0xab, 0, param_2, param_3);
    param_1.ptr_a_lo = 0x98be;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_7320(
    param_1: &mut Struct65,
    param_2: u16,
    param_3: &mut Vec<u8>,
) -> &mut Struct65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0x67, 0xac, 0xbd, param_2, param_3);
    param_1.ptr_a_lo = 0xaba2;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_7356(
    param_1: &mut Struct65,
    param_2: u16,
    param_3: &mut Vec<u8>,
) -> &mut Struct65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0x68, 0xad, 0, param_2, param_3);
    param_1.ptr_a_lo = 0xbe86;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_738c(
    param_1: &mut Struct65,
    param_2: u16,
    param_3: &mut Vec<u8>,
) -> &mut Struct65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0x69, 0xae, 0, param_2, param_3);
    param_1.ptr_a_lo = 0xac3e;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_73c2(
    param_1: &mut Struct65,
    param_2: u16,
    param_3: &mut Vec<u8>,
) -> &mut Struct65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0x35, 0xba, 0x81, param_2, param_3);
    param_1.ptr_a_lo = 0xbf22;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_73f8(
    param_1: &mut Struct65,
    param_2: u16,
    param_3: &mut Vec<u8>,
) -> &mut Struct65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0x39, 0xbb, 0, param_2, param_3);
    param_1.ptr_a_lo = 0xa146;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_745e(
    param_1: &mut Struct65,
    param_2: u16,
    param_3: &mut Vec<u8>,
) -> &mut Struct65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0x22, 0xbc, 0xd5, param_2, param_3);
    param_1.ptr_a_lo = 0xb42a;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_7494(
    param_1: &mut Struct65,
    param_2: u16,
    param_3: &mut Vec<u8>,
) -> &mut Struct65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0x36, 0xbd, 0xcd, param_2, param_3);
    param_1.ptr_a_lo = 0x96ea;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_74ca(
    param_1: &mut Struct65,
    param_2: u16,
    param_3: &mut Vec<u8>,
) -> &mut Struct65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0x37, 0xbe, 0x83, param_2, param_3);
    param_1.ptr_a_lo = 0xa9ce;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_7500(
    param_1: &mut Struct65,
    param_2: u16,
    param_3: &mut Vec<u8>,
) -> &mut Struct65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0x38, 0xbf, 0, param_2, param_3);
    param_1.ptr_a_lo = 0xbcb2;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_7536(
    param_1: &mut Struct65,
    param_2: u16,
    param_3: &mut Vec<u8>,
) -> &mut Struct65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0x3a, 0xc0, 0x85, param_2, param_3);
    param_1.ptr_a_lo = 0x9f72;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_756c(
    param_1: &mut Struct65,
    param_2: u16,
    param_3: &mut Vec<u8>,
) -> &mut Struct65 {
    load_cursor_fn_1018_6a0e(param_1, 0x1e2, 0x3b, 0xc1, 0x86, param_2, param_3);
    param_1.ptr_a_lo = 0xb256;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_75a2(
    param_1: &mut Struct65,
    param_2: u16,
    param_3: &mut Vec<u8>,
) -> &mut Struct65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0x3c, 0xc2, 0x87, param_2, param_3);
    param_1.ptr_a_lo = 0x9516;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_1018_75d8(
    param_1: &mut Struct65,
    param_2: u16,
    param_3: &mut Vec<u8>,
) -> &mut Struct65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0x3d, 0xc3, 0x88, param_2, param_3);
    param_1.ptr_a_lo = 0xa7fa;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_760e(
    param_1: &mut Struct65,
    param_2: u16,
    param_3: &mut Vec<u8>,
) -> &mut Struct65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0x3e, 0xc4, 0, param_2, param_3);
    param_1.ptr_a_lo = 0xbade;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_7644(
    param_1: &mut Struct65,
    param_2: u16,
    param_3: &mut Vec<u8>,
) -> &mut Struct65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0x3f, 0xc5, 0, param_2, param_3);
    param_1.ptr_a_lo = 0x9d02;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_767a(
    param_1: &mut Struct65,
    param_2: u16,
    param_3: &mut Vec<u8>,
) -> &mut Struct65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0x40, 0xc6, 0, param_2, param_3);
    param_1.ptr_a_lo = 0xafe6;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_76b0(
    param_1: &mut Struct65,
    param_2: u16,
    param_3: &mut Vec<u8>,
) -> &mut Struct65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0x41, 199, 0x8d, param_2, param_3);
    param_1.ptr_a_lo = 0xc2ca;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_76e6(
    param_1: &mut Struct65,
    param_2: u16,
    param_3: &mut Vec<u8>,
) -> &mut Struct65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0x42, 200, 0, param_2, param_3);
    param_1.ptr_a_lo = 0xa58a;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_771c(
    param_1: &mut Struct65,
    param_2: u16,
    param_3: &mut Vec<u8>,
) -> &mut Struct65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0x43, 0xc9, 0, param_2, param_3);
    param_1.ptr_a_lo = 0xb86e;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_7752(
    param_1: &mut Struct65,
    param_2: u16,
    param_3: &mut Vec<u8>,
) -> &mut Struct65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0x44, 0xcc, 0, param_2, param_3);
    param_1.ptr_a_lo = 0x9b2e;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_7788(
    param_1: &mut Struct65,
    param_2: u16,
    param_3: &mut Vec<u8>,
) -> &mut Struct65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0x45, 0xcd, 0, param_2, param_3);
    param_1.ptr_a_lo = 0xae12;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_77be(
    param_1: &mut Struct65,
    param_2: u16,
    param_3: &mut Vec<u8>,
) -> &mut Struct65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0x46, 0xd1, 0x92, param_2, param_3);
    param_1.ptr_a_lo = 0xc0f6;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_77f4(
    param_1: &mut Struct65,
    param_2: u16,
    param_3: &mut Vec<u8>,
) -> &mut Struct65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0x47, 0xd2, 0, param_2, param_3);
    param_1.ptr_a_lo = 0xa3b6;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_1018_782a(
    param_1: &mut Struct65,
    param_2: u16,
    param_3: &mut Vec<u8>,
) -> &mut Struct65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0x48, 0xd5, 0, param_2, param_3);
    param_1.ptr_a_lo = 0xacda;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn(
    param_1: &mut Struct65,
    param_2: u16,
    param_3: &mut Vec<u8>,
) -> &mut Struct65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0x49, 0xd6, 0, param_2, param_3);
    param_1.ptr_a_lo = 0xbfbe;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_7896(
    param_1: &mut Struct65,
    param_2: u16,
    param_3: &mut Vec<u8>,
) -> &mut Struct65 {
    load_cursor_fn_1018_6a0e(param_1, 500, 0x4a, 0xd7, 0x98, param_2, param_3);
    param_1.ptr_a_lo = 0xa1e2;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_78cc(
    param_1: &mut Struct65,
    param_2: u16,
    param_3: &mut Vec<u8>,
) -> &mut Struct65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0x4b, 0xd8, 0x99, param_2, param_3);
    param_1.ptr_a_lo = 0xb4c6;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_7902(
    param_1: &mut Struct65,
    param_2: u16,
    param_3: &mut Vec<u8>,
) -> &mut Struct65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0x4c, 0xd9, 0xee, param_2, param_3);
    param_1.ptr_a_lo = 0x9786;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_7938(
    param_1: &mut Struct65,
    param_2: u16,
    param_3: &mut Vec<u8>,
) -> &mut Struct65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0x4d, 0xda, 0x9c, param_2, param_3);
    param_1.ptr_a_lo = 0xaa6a;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_796e(
    param_1: &mut Struct65,
    param_2: u16,
    param_3: &mut Vec<u8>,
) -> &mut Struct65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0x4e, 0xdb, 0x9d, param_2, param_3);
    param_1.ptr_a_lo = 0xbd4e;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_79a4(
    param_1: &mut Struct65,
    param_2: u16,
    param_3: &mut Vec<u8>,
) -> &mut Struct65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0x4f, 0xdc, 0x9e, param_2, param_3);
    param_1.ptr_a_lo = 0xa00e;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_79da(
    param_1: &mut Struct65,
    param_2: u16,
    param_3: &mut Vec<u8>,
) -> &mut Struct65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0x50, 0xdd, 0, param_2, param_3);
    param_1.ptr_a_lo = 0xb2f2;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_7a10(
    param_1: &mut Struct65,
    param_2: u16,
    param_3: &mut Vec<u8>,
) -> &mut Struct65 {
    load_cursor_fn_1018_6a0e(param_1, 0x1d9, 0x51, 0xde, 0, param_2, param_3);
    param_1.ptr_a_lo = 0x95b2;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_7a46(
    param_1: &mut Struct65,
    param_2: u16,
    param_3: &mut Vec<u8>,
) -> &mut Struct65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0x52, 0xdf, 0, param_2, param_3);
    param_1.ptr_a_lo = 0xa896;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_7a7c(
    param_1: &mut Struct65,
    param_2: u16,
    param_3: &mut Vec<u8>,
) -> &mut Struct65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0x53, 0xe0, 0, param_2, param_3);
    param_1.ptr_a_lo = 0xbb7a;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_7ab2(
    param_1: &mut Struct65,
    param_2: u16,
    param_3: &mut Vec<u8>,
) -> &mut Struct65 {
    load_cursor_fn_1018_6a0e(param_1, 0x1e4, 0x55, 0xe2, 0, param_2, param_3);
    param_1.ptr_a_lo = 0xb082;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_7ae8(
    param_1: &mut Struct65,
    param_2: u16,
    param_3: &mut Vec<u8>,
) -> &mut Struct65 {
    load_cursor_fn_1018_6a0e(param_1, 0x1e4, 0x56, 0xe3, 0, param_2, param_3);
    param_1.ptr_a_lo = 0xc366;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_7b1e(
    param_1: &mut Struct65,
    param_2: u16,
    param_3: &mut Vec<u8>,
) -> &mut Struct65 {
    load_cursor_fn_1018_6a0e(param_1, 0x1da, 0x57, 0xe4, 0, param_2, param_3);
    param_1.ptr_a_lo = 0xa626;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_7b54(
    param_1: &mut Struct65,
    param_2: u16,
    param_3: &mut Vec<u8>,
) -> &mut Struct65 {
    load_cursor_fn_1018_6a0e(param_1, 0x1d8, 0x58, 0xe5, 0, param_2, param_3);
    param_1.ptr_a_lo = 0xb90a;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_7b8a(
    param_1: &mut Struct65,
    param_2: u16,
    param_3: &mut Vec<u8>,
) -> &mut Struct65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0x59, 0xe6, 0, param_2, param_3);
    param_1.ptr_a_lo = 0x9bca;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_7bc0(
    param_1: &mut Struct65,
    param_2: u16,
    param_3: &mut Vec<u8>,
) -> &mut Struct65 {
    load_cursor_fn_1018_6a0e(param_1, 0x1ef, 0x5a, 0xe7, 0, param_2, param_3);
    param_1.ptr_a_lo = 0xaeae;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_7bf6(
    param_1: &mut Struct65,
    param_2: u16,
    param_3: &mut Vec<u8>,
) -> &mut Struct65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0x5b, 0xe8, 0, param_2, param_3);
    param_1.ptr_a_lo = 0xc192;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_7c2c(
    param_1: &mut Struct65,
    param_2: u16,
    param_3: &mut Vec<u8>,
) -> &mut Struct65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0x5c, 0xea, 0, param_2, param_3);
    param_1.ptr_a_lo = 0xb736;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_7c62(
    param_1: &mut Struct65,
    param_2: u16,
    param_3: &mut Vec<u8>,
) -> &mut Struct65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0x5d, 0xeb, 0, param_2, param_3);
    param_1.ptr_a_lo = 0x99f6;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_7c98(
    param_1: &mut Struct65,
    param_2: u16,
    param_3: &mut Vec<u8>,
) -> &mut Struct65 {
    load_cursor_fn_1018_6a0e(param_1, 0x1e6, 0x5e, 0xec, 0xee, param_2, param_3);
    param_1.ptr_a_lo = 0xba42;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_7cce(
    param_1: &mut Struct65,
    param_2: u16,
    param_3: &mut Vec<u8>,
) -> &mut Struct65 {
    load_cursor_fn_1018_6a0e(param_1, 0x1da, 0x5f, 0xed, 0, param_2, param_3);
    param_1.ptr_a_lo = 0x9ed6;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_7d04(
    param_1: &mut Struct65,
    param_2: u16,
    param_3: &mut Vec<u8>,
) -> &mut Struct65 {
    load_cursor_fn_1018_6a0e(param_1, 0, 0x60, 0xee, 0, param_2, param_3);
    param_1.ptr_a_lo = 0xb1ba;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_7d3a(
    param_1: &mut Struct65,
    param_2: u16,
    param_3: &mut Vec<u8>,
) -> &mut Struct65 {
    load_cursor_fn_1018_6a0e(param_1, 0x1f0, 0x61, 0xef, 0, param_2, param_3);
    param_1.ptr_a_lo = 0x947a;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub fn load_cursor_fn_1018_7d70(
    param_1: &mut Struct65,
    param_2: u16,
    param_3: &mut Vec<u8>,
) -> &mut Struct65 {
    load_cursor_fn_1018_6a0e(param_1, 0x1f7, 0x62, 0xf0, 0xcc, param_2, param_3);
    param_1.ptr_a_lo = 0xa75e;
    (param_1 + 2) = 0x1018;
    return param_1;
}

pub unsafe fn load_cursor_1020_7f7a(
    ctx: &mut AppContext,
    in_struct_65_1: &mut Struct65,
    param_2: u32,
    param_3: u32,
) {
    let mut u_var1: u16;
    let mut h_gdi_obj: HGDIOBJ16;
    let mut h_cursor: HCURSOR16;
    let mut local_AX_192:  Struct590;
    let mut struct_65_1:  Struct65;
    let mut u_var2: u16;
    let mut ppVar3:  Struct2551;
    let mut u_var4: u16;
    let mut in_stack_0000ffe8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

  // u_var4 = (param_2  >> 0x10);
    load_cursor_1008_61b2(in_struct_65_1, param_2, u_var4, param_3);
  // u_var2 = (in_struct_65_1  >> 0x10);
    struct_65_1 = in_struct_65_1;
    struct_65_1.ptr_b_lo = ctx.s_1_1050_389a;
    struct_65_1.ptr_b_hi = &ctx.PTR_LOOP_1050_1008;
    struct_65_1.ptr_b_lo = (ctx.ctx.s_18_2_1050_3aa5 + 3);
    struct_65_1.ptr_b_hi = &ctx.PTR_LOOP_1050_1008;
    &struct_65_1.u16_xe6 = 0;
    struct_65_1.u16_xea = 0;
    struct_65_1.u16_xec = 0;
    in_struct_65_1.ptr_a_lo = 0x82bc;
    struct_65_1.ptr_a_hi = 0x1020;
    struct_65_1.ptr_b_lo = 0x8358;
    struct_65_1.ptr_b_hi = 0x1020;
    copy_string_1000_3d3e(
        (in_struct_65_1 & 0xffff0000 | ZEXT24(&struct_65_1.char_ptr_x5b)),
        ctx.s_VrMode_1050_4422,
    );
    h_gdi_obj = GetStockObject16(5);
    struct_65_1.h_gdi_obj_xc6 = h_gdi_obj;
    h_cursor = LoadCursor16(0x7f00, 0);
    struct_65_1.h_cursor_xc4 = h_cursor;
    struct_65_1.u16_xc8 = (ctx.s_576_bmp_1050_2027 + 1);
    struct_65_1.u32_xac = 0x47000000;
    struct_65_1.astruct_590_ptr_xbc = (param_3 + 8);
    ppVar3 = process_struct_1010_20ba(
        ctx._g_astruct_372_1050_0ed0,
        CONCAT22(in_stack_0000ffe8, 0x48),
    );
  // u_var1 = (ppVar3  >> 0x10);
    struct_65_1.u16_xb4 = 0;
    struct_65_1.u16_xb6 = 0;
    struct_65_1.u16_xb8 = (ppVar3 + 10);
    struct_65_1.u16_xba = (ppVar3 + 0xc);
    &struct_65_1.u32_xca = u_var4;
    reg_class_1008_96d2(in_struct_65_1, in_stack_0000ffe8);
    return;
}

pub unsafe fn call_load_cursor_fn_1020_7554(
    ctx: &mut AppContext,
    param_1: &mut Struct65,
    param_2: u16,
    param_3: u32,
) {
    let mut local_AX_24: u16;
    let mut local_DX_71: u16;
    let mut iVar1: i32;
    let mut local_BP__1: u16;
    let mut local_struct_1_hi:  Struct65;
    let mut ppVar2:  Struct2551;

    load_cursor_1020_7f7a(ctx, param_1, CONCAT22(param_2, 5), param_3);
  // local_struct_1_hi = (param_1  >> 0x10);
    iVar1 = param_1;
    (iVar1 + 0xee) = 0;
    (iVar1 + 0xf2) = 0;
    param_1.ptr_a_lo = 0x7780;
    (iVar1 + 2) = 0x1020;
    (iVar1 + 0xe2) = 0x781c;
    (iVar1 + 0xe4) = 0x1020;
    ppVar2 = process_struct_1010_20ba(ctx._g_astruct_372_1050_0ed0, CONCAT22(local_BP__1, 0x25));
  // local_DX_71 = (ppVar2  >> 0x10);
    (iVar1 + 0xf2) = ppVar2;
    (iVar1 + 0xf4) = local_DX_71;
    (iVar1 + 0xe6) = (iVar1 + 0xf2);
    (iVar1 + 0xe8) = local_DX_71;
    return;
}

pub unsafe fn load_cursor_1020_67ce(
    ctx: &mut AppContext,
    param_1: &mut Struct65,
    param_2: u16,
    param_3: u32,
) {
    let mut HVar1: HGDIOBJ16;
    let mut HVar2: HCURSOR16;
    let mut i_var3: i32;
    let mut u_var4: u16;
    let mut in_stack_0000fffc: u16;

    call_load_cursor_1020_790e(param_1, ctx.s_TPPOPMENU_1050_43fa, param_2, param_3);
  // u_var4 = (param_1  >> 0x10);
    i_var3 = param_1;
    (i_var3 + 0xf2) = 0;
    (i_var3 + 0xf6) = 0;
    param_1.ptr_a_lo = 0x70e6;
    (i_var3 + 2) = 0x1020;
    copy_string_1000_3d3e(
        (param_1 & 0xffff0000 | (i_var3 + 0x5b)),
        ctx.s_VrMode2_1050_4404,
    );
    HVar1 = GetStockObject16(5);
    (i_var3 + 0xc6) = HVar1;
    HVar2 = LoadCursor16(0x7f00, 0);
    (i_var3 + 0xc4) = HVar2;
    (i_var3 + 0xac) = 0x44c00000;
    (i_var3 + 200) = (ctx.s_575_bmp_1050_201f + 1);
    (i_var3 + 0xbc) = (param_3 + 8);
    (i_var3 + 0xca) = param_2;
    reg_class_1008_96d2(param_1, in_stack_0000fffc);
    window::update_window_1020_6c3a(i_var3, u_var4);
    return;
}

pub unsafe fn set_cursor_1020_5de8(
    ctx: &mut AppContext,
    in_struct_1: &mut Struct664,
    param_2: u16,
    in_struct_2: &mut Struct665,
) {
    let mut u_var1: u32;
    let mut local_struct_2_1:  Struct665;

    let mut local_struct_1:  Struct664;
    let mut local_struct_1_hi: u16;
    let mut unaff_ss: u16;
    let mut ppVar2:  Struct2551;
    let mut in_stack_0000ffe2: HWND16;
    let mut in_stack_0000ffe8: u16;
    let mut local_12: u32;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_struct_2:  Struct665;
    let mut local_4: u16;

    ReleaseCapture16(in_stack_0000ffe2);
  // local_struct_1_hi = (in_struct_1  >> 0x10);
    local_struct_1 = in_struct_1;
    SetCursor16(local_struct_1.cursor_handle_0xee);
    local_struct_1.cursor_handle_0xee = 0;
    local_struct_1.field_0xf4 = 1;
    local_struct_2 = in_struct_2;
    local_4 = param_2;
    ppVar2 = process_struct_1010_20ba(
        ctx._g_astruct_372_1050_0ed0,
        CONCAT22(in_stack_0000ffe8, 0x47),
    );
    local_struct_2_1 = &local_struct_2;
    rect::pt_in_rect_1020_5856(in_struct_1, CONCAT22(unaff_ss, local_struct_2_1));
    if ((ctx.dx_reg | local_struct_2_1) != 0) {
        u_var1 = &local_struct_2_1.field_0x42;
        local_struct_1_hi = &local_struct_2_1.field_0x44;
        local_12._3_1_ = (u_var1 >> 0x18);
        if (local_12._3_1_ == '\x05') {
            local_12._0_2_ = u_var1;
            // goto LAB_1020_5e62;
        }
    }
    local_12._0_2_ = 0;
    local_struct_1_hi = 0;
    // LAB_1020_5e62:
    pass1_1018_57e6(ppVar2, CONCAT22(local_struct_1_hi, local_12));
    return;
}

pub unsafe fn set_cursor_1020_5764(
    ctx: &mut AppContext,
    in_struct_1: &mut Struct673,
    param_2: u16,
) {
    let mut u_var1: i32;
    let pu_var2: Vec<u8>;
    let mut u_var3: u32;
    let mut u_var4: u16;
    let mut h_cursor: HCURSOR16;
    let mut local_struct_1:  Struct673;
    let mut local_struct_1_hi:  Struct673;
    let mut unaff_ss: u16;
    let mut in_stack_0000ffe6: u16;
    let mut in_stack_0000ffe8: HWND16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: [u8; 2];
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    local_6 = process_struct_1010_20ba(
        ctx._g_astruct_372_1050_0ed0,
        CONCAT22(in_stack_0000ffe6, 0x2f),
    );
  // u_var4 = (local_6  >> 0x10);
    local_a = (local_6 + 0x20);
    u_var1 = (local_6 + 0x22);
    if ((u_var1 | local_a) != 0) {
        pass1_1030_8308(
            ctx._g_bool_1050_5748,
            (ctx._g_bool_1050_5748 >> 0x10),
            CONCAT22(unaff_ss, &local_e),
            CONCAT22(unaff_ss, local_c),
            local_a & 0xffff | u_var1 << 0x10,
        );
        if (param_2 <= local_e) {
          // local_struct_1_hi = (in_struct_1  >> 0x10);
            local_struct_1 = in_struct_1;
            if (local_struct_1.field_0xf4 != 1) {
                SetCursor16(local_struct_1.field_0xee);
                local_struct_1.field_0xee = 0;
                local_struct_1.field_0xf4 = 1;
                &local_struct_1.field_0x10c = 0;
                ReleaseCapture16(in_stack_0000ffe8);
            }
            h_cursor = LoadCursor16(0x7f02, 0);
            SetCursor16(h_cursor);
            local_12 = param_2;
            pass1_1018_017c(local_6, param_2);
            pu_var2 = local_struct_1.field_0xf6;
            (pu_var2 + 0x10) = 1;
            if (&local_struct_1.field_0xfe != 0) {
                rect::call_invalidate_rect_1020_68de(*&local_struct_1.field_0xfe);
                u_var3 = &local_struct_1.field_0xfe;
                local_12 = (u_var3 + 8);
                PostMessage16(0, 0xeb, 0x111, local_12);
            }
            SetCursor16(local_12);
        }
    }
    return;
}
