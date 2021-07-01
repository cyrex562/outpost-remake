use std::intrinsics::offset;

use crate::app_context::AppContext;
use crate::draw::{draw2, rect};
use crate::err_ops::error_check_1000_17ce;
use crate::pass::pass12_funcs::pass1_1008_c6ae;
use crate::pass::pass14_funcs::pass1_1008_57c4;
use crate::pass::pass15_funcs::{pass1_1020_6498, pass1_1020_64d4};
use crate::pass::pass17_funcs::{pass1_1030_69cc, pass1_1030_6fa0, pass1_1030_73a8, pass1_1030_8308, pass1_1030_8344};
use crate::pass::pass20_funcs::pass1_1010_c3c2;
use crate::pass::pass7_funcs::pass1_1018_2504;
use crate::pass::pass8_funcs::process_struct_1010_20ba;
use crate::structs::prog_structs_15::Struct26;
use crate::structs::prog_structs_27::pass1_struct_2;
use crate::structs::prog_structs_2::Struct7;
use crate::structs::prog_structs_31::Struct27;
use crate::structs::prog_structs_7::{Struct215, Struct44};
use crate::sound_funcs::mci_send_command_1008_5c7c;
use crate::string_ops::misc::copy_string_1000_3d3e;
use crate::typedefs::{HMENU16, HWND16};
use crate::ui_ops::ui2;
use crate::util::{CONCAT22, CONCAT31, SUB42, ZEXT24};
use crate::winapi::{CheckMenuItem16, ClientToScreen16, DeleteMenu16, DestroyMenu16, EnableMenuItem16, GetMenuState16, GetSubMenu16, InsertMenu16, LoadMenu16, ModifyMenu16, PostMessage16, PtInRect16, TrackPopupMenu16};
use crate::mem_funcs::mem_ops_1::StructuredData;

pub fn enable_menu_item_1020_1000(param_1: u16, param_2: HMENU16) {
    if param_1 != 0 {
        return;
    }
    EnableMenuItem16(param_2, 3, 0x400);
    return;
}

pub unsafe fn call_destroy_menu_fn_1020_135e(ctx: &mut AppContext, param_1: &mut Struct7, param_2: u8) -> &mut Struct7 {
    destroy_menu_func_1020_795c(param_1);
    if (param_2 & 1) != 0 {
        error_check_1000_17ce(ctx, param_1);
    }
    return param_1;
}

pub fn track_popup_menu_1008_09ba(ctx: &mut AppContext, param_1: u32, param_2: u16) {
    let mut manu_handle_1: HMENU16;
    let mut i_var2: i32;
    let mut u_var3: u16;
    let unaff_ss: HWND16;
    let mut menu_handle_6: HMENU16;
    let mut local_4: u16;

    if (param_1 + 0xec) == 0 {
        manu_handle_1 = LoadMenu16(ctx.g_h_instance_1050_038c, ctx.s_OPPOPMENU_1050_0150);
        (param_1 + 0xec) = manu_handle_1;
        if manu_handle_1 == 0 {
            return;
        }
        menu_handle_6 = (param_1 + 0xec);
        unaff_cs = SUB42(offset, 0);
        manu_handle_1 = GetSubMenu16(menu_handle_6, 0 );
        (param_1 + 0xec) = manu_handle_1;
        if (manu_handle_1 == 0) {
            return;
        }
    }
    local_4 = param_2;
    menu_handle_6 = (param_1 + 8);
    ClientToScreen16( menu_handle_6, ctx.stack_seg_reg);
    TrackPopupMenu16(0x0, 0, ctx.g_h_window, 0, local_4 as i16, menu_handle_6, 0);
    return;
}

pub fn track_popup_menu_1040_7f86(ctx: &mut AppContext, param_1: &mut Struct27, param_2: u16) {
    let mut menu_handle: HMENU16;
    let mut HVar1: HMENU16;
    let mut window_handle: HWND16;
    let mut local_6: HMENU16;
    let mut local_4: u16;

    if (param_1.menu_name != 0x0) && (param_1.menu_handle_2 == 0) {
        menu_handle = LoadMenu16( ctx.g_h_instance_1050_038c, param_1.menu_name);
        param_1.menu_handle_2 = menu_handle;
        if menu_handle == 0 {
            return;
        }
        local_6 = param_1.menu_handle_2;
        HVar1 = GetSubMenu16(local_6, 0 );
        param_1.menu_handle_2 = HVar1;
        if HVar1 == 0 {
            return;
        }
    }
    local_4 = param_2;
    local_6 = param_1.field_0x6;
    ClientToScreen16(window_handle, CONCAT22(&local_6, unaff_cs));
    // TrackPopupMenu16(0x0, 0, local_bx_4.field_0x6, 0, local_4, local_6, 0);
    TrackPopupMenu16(0, local_6, local_4, param_1.field_0x6, 0, 0);
    return;
}

pub fn destroy_menu_func_1020_795c(struct_param_1: &mut Struct7) {
    // let local_struct_1: Struct215;
    // let local_struct_1_hi: Struct215;
    let mut menu_handle: HMENU16;

    //// ocal_struct_1_hi = (in_struct_1  >> 0x10);
    // local_struct_1 = in_struct_1;
    struct_param_1.ptr_a_lo = 0x7b86;
    struct_param_1.ptr_a_hi = 0x1020;
    if local_struct_1.field_0xec != 0 {
        DestroyMenu16(menu_handle);
    }
    pass1_1008_57c4((struct_param_1 & 0xffff0000 | ZEXT24(&struct_param_1.field_0xd2)));
    struct_param_1.ptr_a_lo = 0x380a;
    struct_param_1.ptr_a_hi = &ctx.PTR_LOOP_1050_1008;
    struct_param_1.ptr_a_lo = ctx.s_1_1050_389a;
    struct_param_1.ptr_a_hi = &ctx.PTR_LOOP_1050_1008;
    return;
}

pub fn enable_menu_item_1020_2c2a(ctx: &mut AppContext, param_1: HMENU16, param_2: u16) -> u16 {
    let b_var1: bool;
    let mut in_stack_0000000a: i32;
    let mut in_stack_0000000c: u16;
    let mut flags: u16;

    if (param_2 != 0) {
        return param_2 - 1;
    }
    EnableMenuItem16(param_1, 3, 0x400);
    if ctx.PTR_LOOP_1050_3960 < 2 {
        flags = 0x401;
    } else {
        flags = 0x400;
    }
    b_var1 = EnableMenuItem16(param_1, 5, flags);
    return u16::from(b_var1);
}

pub fn track_popup_menu_1020_7ad2(ctx: &mut AppContext, param_1: &mut StructuredData, param_2: u16) {
    let mut menu_handle_var1: HMENU16;
    let mut i_var2: i32;
    let mut u_var3: u16;
    let mut unaff_cs: u16;
    let mut unaff_ss: HWND16;
    let mut local_6: HMENU16;
    let mut local_4: u16;
    let mut offset: u16;

    //// _var3 = (param_1  >> 0x10);
    // i_var2 = param_1;
    if ((param_1 + 0xee) != 0) && ((param_1 + 0xec) == 0) {
        menu_handle_var1 = LoadMenu16(ctx.g_h_instance_1050_038c, (param_1 + 0xee));
        (param_1 + 0xec) = menu_handle_var1;
        if menu_handle_var1 == 0 {
            return;
        }
        local_6 = (param_1 + 0xec);
        menu_handle_var1 = GetSubMenu16(local_6, 0);
        (param_1 + 0xec) = menu_handle_var1;
        if menu_handle_var1 == 0 {
            return;
        }
    }
    local_4 = param_2;
    local_6 = (param_1 + 8);
    ClientToScreen16(CONCAT22(&local_6, ctx.code_seg_reg), ctx.stack_seg_reg);
    TrackPopupMenu16(0x0, 0, (param_1 + 8), 0, local_4 as i16, local_6, 0);
    return;
}

pub fn enable_menu_item_1020_6b9a(ctx: &mut AppContext, param_1: u16, param_2: HMENU16) {
    let mut in_stack_0000000a: i32;
    let mut in_stack_0000000c: u16;

    if param_1 != 0 {
        return;
    }
    EnableMenuItem16( param_2, 0, 0x400);
    return;
}

pub unsafe fn track_popup_menu_1020_5bf2(
    ctx: &mut AppContext,
    param_1: &mut Struct26,
    param_2: u16,
    param_3: u16,
) -> bool {
    let mut iVar1: i32;
    let mut pt_in_rect: u16;
    let mut tile_menu_handle: u16;
    let mut HVar2: HMENU16;
    let mut u_var3: i32;
    let local_bx_18: &mut  Struct26;
    let mut u_var4: u16;
    let mut unaff_cs: u16;
    let mut h_window: u16;
    let mut u_var5: u32;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut rect: u32;
    let mut local_6: u16;
    let mut local_4: u16;
    let mut offset: u16;
    let mut tile_menu_name: String;

    local_6 = param_3;
    local_4 = param_2;
  // u_var4 = (param_1  >> 0x10);
    local_bx_18 = param_1;
    iVar1 = pass1_1020_64d4(local_bx_18.field_0xf6, 2);
    if (iVar1 != 0) {
        rect = pass1_1020_6498(local_bx_18.field_0xf6, 2);
        unaff_cs = SUB42(offset, 0);
        pt_in_rect = PtInRect16(CONCAT22(local_4, local_6), rect);
        if (pt_in_rect != 0) {
            PostMessage16(0, 0x131, 0x111, ctx.g_h_window);
            return 1;
        }
    }
    iVar1 = pass1_1020_64d4(local_bx_18.field_0xf6, 3);
    if iVar1 == 0 {
        return 0;
    }
    u_var5 = rect::pt_in_rect_1020_5856(param_1, CONCAT22(h_window, &local_6));
  // u_var3 = (u_var5  >> 0x10);
    local_bx_18.field_0x108 = u_var5;
    &local_bx_18.field_0x10a = u_var3;
    if (u_var3 | local_bx_18.field_0x108) == 0 {
        return 0;
    }
    if (local_bx_18.menu_handle == 0) {
        tile_menu_handle = LoadMenu16(tile_menu_name, ctx.g_h_instance_1050_038c);
        local_bx_18.menu_handle = tile_menu_handle;
        if (tile_menu_handle == 0) {
            return 1;
        }
        unaff_cs = SUB42(offset, 0);
        HVar2 = GetSubMenu16(0, local_bx_18.menu_handle);
        local_bx_18.menu_handle = HVar2;
        if (HVar2 == 0) {
            return 1;
        }
    }
    u_var5 = &local_bx_18.field_0x108;
    rect._0_2_ = (u_var5 + 0x2e);
    local_c = 0;
    if (rect == 0x42) {
        local_c = 0xc9;
    } else {
        if (((ctx.s_VrMode_1050_42ca + 8) + rect * 2) == 0) {
            local_c = 200;
        }
    }
    if (local_c != 0) {
        unaff_cs = SUB42(&ctx.PTR_LOOP_1050_1008, 0);
        mci_send_command_1008_5c7c(ctx.g_struct_1050_02a0, CONCAT22(local_c, 1));
    }
    local_10 = param_3;
    local_e = param_2;
    ClientToScreen16(CONCAT22(&local_10, unaff_cs), h_window);
    TrackPopupMenu16(0x0, 0, local_bx_18.field_0x8, 0, local_e, local_10, 0);
    return 1;
}

pub unsafe fn enable_menu_item_1020_44ec(
    ctx: &mut AppContext,
    param_1: u32,
    param_2: u16,
    param_4: i32,
    param_3: u16,
) {
    let mut u_var1: u32;
    let u_var2: u8;
    let mut u_var3: i32;
    let ppVar4: &mut  pass1_struct_2;
    let pu_var5: &mut  u16;
    let b_var6: bool;
    let extraout_var;
    let mut in_dx: i32;
    let mut u_var7: i32;

    let mut i_var8: i32;
    let mut i_var9: i32;
    let mut u_var10: u16;
    let mut u_var11: u16;
    let u_var12: u8;
    let mut unaff_ss: u16;
    let mut h_var13: HMENU16;
    let mut h_menu: u16;
    let mut local_138: u16;
    let pp_stack_310: &mut  pass1_struct_2;
    let mut local_134: u16;
    let mut local_130: u16;
    let mut local_12e: u16;
    let mut local_12c: u32;
    let mut local_128: u32;
    let mut local_124: u32;
    let mut local_11e: u32;
    let mut local_116: u32;
    let mut local_10e: u16;
    let mut local_10c: u32;
    let mut local_108: u16;
    let mut local_106: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;
    let mut offset: u16;

  // u_var10 = (param_1  >> 0x10);
    i_var8 = param_1;
    if ((i_var8 + 0x106) != 0) {
        if (*(i_var8 + 0x106) == param_3) {
            local_6 =
                process_struct_1010_20ba(ctx._g_astruct_372_1050_0ed0, CONCAT22(local_138, 3));
            u_var1 = (i_var8 + 0x108);
            local_8 = (u_var1 + 0x2e);
            u_var1 = (i_var8 + 0x108);
          // u_var11 = (u_var1  >> 0x10);
            i_var9 = u_var1;
            local_128 = (i_var9 + 0x42);
            u_var7 = (i_var9 + 0x44);
            local_128._3_1_ = (local_128 >> 0x18);
            u_var3 = local_128._3_1_;
            local_11e = local_128;
            local_10c = local_128;
            if (local_128._3_1_ == 0) {
                big_switch_statement_1020_bd80(local_8);
                u_var11 = 0x1000;
                copy_string_1000_3d3e(CONCAT22(unaff_ss, &local_108), CONCAT22(u_var7, u_var3));
            } else {
                ppVar4 = pass1_1030_8344(
                    ctx._g_bool_1050_5748,
                    (ctx._g_bool_1050_5748 >> 0x10),
                    local_128 & 0xffff | u_var7 << 0x10,
                );
                local_128 = CONCAT22(u_var7, ppVar4);
                u_var11 = 0x1010;
                pass1_1010_c3c2(
                    local_6,
                    (local_6 >> 0x10),
                    CONCAT22(unaff_ss, &local_108),
                    CONCAT22(u_var7, ppVar4),
                );
            }
            pu_var5 = &local_108;
            ModifyMenu16(u_var11, pu_var5, unaff_ss, 0x76, 0, 0x76, (i_var8 + 0x106));
            local_10e = pu_var5;
            GetMenuState16(offset, 0, 0x3c, (i_var8 + 0x106));
            if (pu_var5 != 0xffff) {
                DeleteMenu16(0x38, 0, 0x13c, (i_var8 + 0x106));
            }
            b_var6 = pass1_1008_c6ae(ctx.ctx._PTR_LOOP_1050_06e0, local_8, 0x20);
            if (b_var6 != 0) {
                load_string_1010_847e(
                    ctx.g_struct_73_1050_14cc,
                    (ctx.g_struct_73_1050_14cc >> 0x10),
                    0x74b,
                );
                local_128 = CONCAT22(ctx.dx_reg, b_var6);
                InsertMenu16(
                    0x1010,
                    b_var6,
                    ctx.dx_reg,
                    0x3c,
                    0x400,
                    0xffff,
                    (i_var8 + 0x106),
                );
            }
            if (((ctx.s_VrMode_1050_42ca + 8) + local_8 * 2) == 0) {
                local_128._0_2_ = *(i_var8 + 0x106);
                h_var13 = 1;
                h_menu = 0x77;
                // goto LAB_1020_464c;
            }
            local_128._0_2_ = *(i_var8 + 0x106);
            h_menu = 0x77;
        } else {
            h_var13 = GetSubMenu16(1, (i_var8 + 0x106));
            local_128 = local_128 & 0xffff0000 | h_var13;
            if (h_var13 != param_3) {}
            // goto LAB_1020_479e;
            EnableMenuItem16(1, 0x200, h_var13);
            local_138 = local_128;
            EnableMenuItem16(1, 0x201, local_128);
            pp_stack_310 = local_128;
            local_138 = 0x202;
            EnableMenuItem16(1, 0x202, local_128);
            u_var1 = (i_var8 + 0x108);
            local_124 = (u_var1 + 0x42);
            pp_stack_310 = local_124;
          // local_134 = (local_124  >> 0x10);
          // local_138 = (ctx._g_bool_1050_5748  >> 0x10);
            pp_stack_310 = pass1_1030_8344(ctx._g_bool_1050_5748, local_138, local_124);
            local_11e = CONCAT22(in_dx, pp_stack_310);
            if ((in_dx | pp_stack_310) == 0) {
                return;
            }
            local_116 = &pp_stack_310.field_0x2e;
            if ((&pp_stack_310.field_0x30 | local_116) == 0) {
                return;
            }
            local_10c = (local_116 + 0x200);
            local_138 = 0x1030;
            local_108 = pass1_1030_73a8(CONCAT22(in_dx, pp_stack_310));
          // u_var10 = (local_108  >> 0x10);
            local_6 = (local_108 + 0x1c);
            u_var7 = (local_108 + 0x1e);
            if ((u_var7 | local_6) != 0) {
                local_10c = local_6 & 0xffff | u_var7 << 0x10;
            }
            if (local_10c != 1) {
                return;
            }
            if ((local_10c & 0xff0000) != 0) {
                return;
            }
            pp_stack_310 = local_11e;
          // local_134 = (local_11e  >> 0x10);
            local_138 = 0x1030;
            u_var2 = pass1_1030_6fa0(local_11e);
            pp_stack_310 = CONCAT31(extraout_var, u_var2);
            local_134 = 0x3f;
          // local_138 = (ctx.ctx._PTR_LOOP_1050_06e0  >> 0x10);
            b_var6 = pass1_1008_c6ae(ctx.ctx._PTR_LOOP_1050_06e0, pp_stack_310, 0x3f);
            if (b_var6 != 0) {
                local_134 = local_128;
                local_138 = 0;
                pp_stack_310 = 0x201;
                b_var6 = EnableMenuItem16(0, 0x201, local_128);
            }
            if ((local_11e + 0x36) != 0) {
                b_var6 = EnableMenuItem16(0, 0x202, local_128);
            }
            pass1_1030_69cc(local_11e);
            if (b_var6 == 0) {
                return;
            }
            h_menu = 0x200;
        }
        h_var13 = 0;
        // goto LAB_1020_464c;
    }
    // LAB_1020_479e:
    i_var9 = param_2 + -1;
    if (i_var9 == 0) {
        u_var1 = (i_var8 + 0xfa);
        pass1_1018_2504(u_var1, (u_var1 >> 0x10));
        if (i_var9 == 0) {
            h_menu = 0;
            EnableMenuItem16(0x401, 0, param_3);
            local_138 = param_3;
            local_128._0_2_ = 1;
            // LAB_1020_47e3:
            h_var13 = 0x401;
            // goto LAB_1020_464c;
        }
        h_menu = 0;
        EnableMenuItem16(0x400, 0, param_3);
        local_138 = param_3;
        local_128._0_2_ = 1;
    } else {
        i_var9 = param_2 + -2;
        if (i_var9 == 0) {
            pass1_1020_64d4(*(i_var8 + 0xf6), 2);
            if (i_var9 == 0) {
                EnableMenuItem16(0x401, 0, param_3);
                h_menu = 0x401;
            } else {
                EnableMenuItem16(0x400, 0, param_3);
                h_menu = 0x400;
            }
            local_128._0_2_ = 1;
            local_138 = param_3;
            EnableMenuItem16(h_menu, 1, param_3);
            if ((ctx.PTR_LOOP_1050_0010 != 0x0) || ((i_var8 + 0x102) == 0)) {
                pp_stack_310 = param_3;
                local_138 = 5;
                // goto LAB_1020_47e3;
            }
            pp_stack_310 = param_3;
            local_138 = 5;
        } else {
            if (param_2 == 3) {
                local_138 = 0;
                local_134 = 0;
                u_var12 = 0x10;
                local_130 = process_struct_1010_20ba(ctx._g_astruct_372_1050_0ed0, 0x2f);
              // u_var10 = (local_130  >> 0x10);
                local_12c = (local_130 + 0x20);
                u_var7 = (local_130 + 0x22);
                if ((u_var7 | local_12c) != 0) {
                    u_var12 = 0x30;
                    pass1_1030_8308(
                        ctx._g_bool_1050_5748,
                        (ctx._g_bool_1050_5748 >> 0x10),
                        CONCAT22(unaff_ss, &local_134),
                        CONCAT22(unaff_ss, &local_138),
                        local_12c & 0xffff | u_var7 << 0x10,
                    );
                    local_138 = (local_130 + 0x1e);
                }
                local_128 = local_128 & 0xffff0000;
                while {
                    CheckMenuItem16(u_var12, 0x400, local_128, param_3);
                    u_var12 = 0x38;
                    EnableMenuItem16(0x401, local_128, param_3);
                    local_128 = local_128 & 0xffff0000 | (local_128 + 1);
                    (local_128 + 1) < 5
                } {}
                CheckMenuItem16(0x38, 0x408, local_138, param_3);
                local_128 = local_128 & 0xffff0000;
                while (local_128 <= local_134) {
                    EnableMenuItem16(0x400, local_128, param_3);
                    local_128 = local_128 & 0xffff0000 | (local_128 + 1);
                }
                return;
            }
            if (param_2 != 4) {
                return;
            }
            h_menu = 2;
            local_128._0_2_ = param_3;
        }
    }
    h_var13 = 0x400;
    // LAB_1020_464c:
    EnableMenuItem16(h_var13, h_menu, local_128);
    return;
}
