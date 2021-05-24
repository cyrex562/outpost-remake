use std::intrinsics::offset;

use crate::app_context::AppContext;
use crate::err_ops::error_check_1000_17ce;
use crate::mem_funcs::mem_ops_1::get_string_from_address;
use crate::other_funcs::{get_private_profile_str_1010_6132, write_private_profile_str_1010_622a};
use crate::pass::pass14_funcs::pass1_fn_1008_60e8;
use crate::pass::pass8_funcs::pass1_1010_1d80;
use crate::pass::pass_funcs::{pass1_1000_4906, pass1_fn_1000_3e2c, pass1_fn_1000_47a4, pass1_fn_1000_5008};
use crate::string_ops::misc::{copy_string_1000_3d3e, get_string_index_1000_3da4, process_string_1000_3cea, process_string_1000_475e, string_fn_1000_3f9c};
use crate::struct_ops::struct_ops_2;
use crate::struct_ops::process_struct_1010_20ba;
use crate::struct_ops::struct_ops_2::{process_struct_1000_179c, process_struct_1008_574a, process_struct_1010_1d48};
use crate::structs::prog_structs_24::{Struct103, Struct2111};
use crate::structs::prog_structs_25::Struct64;
use crate::structs::prog_structs_7::Struct376;
use crate::util::{CONCAT11, CONCAT12, CONCAT13, CONCAT22, CONCAT31, SUB42, ZEXT24};
use crate::winapi;
use crate::winapi::WritePrivateProfileString16;

pub unsafe fn write_private_profile_str_1010_5b10(ctx: &mut AppContext, param_1: &mut Struct376) {
    let pu_var1: u32;
    let mut u_var2: i32;
    let mut u_var3: u32;
    let ppcVar4: fn();
    let mut u_var5: u16;
    // let mut u_var6: u16;
    let pp_var7: Struct2111;
    let mut local_c: u16;
    let mut local_8: u16;

    //// _var6 = (param_1  >> 0x10);
    // local_bx_5 = param_1;
    param_1.ptr_a_lo = 0x6312;
    param_1.field_0x2 = 0x1010;
    pp_var7 = struct_ops_2::process_struct_1010_20ba(ctx.g_struct_var_1050_0ed0, CONCAT22(unaff_si, 0x48));
    u_var3 = param_1.field_0xe;
    string_fn_1000_3f9c(ctx,
        param_1,
        &ctx.s__d__d_1050_149c,
        &ctx.g_alloc_addr_1050_1050,
        pp_var7.field_0xa // *(pp_var7 + 10),
    );
    if local_bx_5.field_0x80 == 0 {
        u_var5 = ctx.s_off_1050_13c8;
    } else {
        u_var5 = ctx.s_on_1050_13c4;
    }
    let entry_addr: u32 = CONCAT22(0x0105, u_var5);
    let entry_str: String = get_string_from_address(entry_addr);
    WritePrivateProfileString16(
        param_1.field_0xa,
        &entry_str,
        ctx.s_falseMap_1050_13de,
        ctx.s_general_1050_13b0,
    );
    WritePrivateProfileString16(
        param_1.field_0xa,
        param_1.field_0xe,
        ctx.s_rez_1050_13c0,
        ctx.s_general_1050_13b0,
    );
    if local_bx_5.field_0x1e == 0 {
        u_var5 = SUB42(s_off_1050_13c8, 0);
    } else {
        u_var5 = SUB42(s_on_1050_13c4, 0);
    }
    WritePrivateProfileString16(
        local_bx_5.field_0xa,
        CONCAT22(0x1050, u_var5),
        s_anims_1050_13cc,
        s_general_1050_13b0,
    );
    if (local_bx_5.field_0x74 == 0) {
        u_var5 = SUB42(s_off_1050_13c8, 0);
    } else {
        u_var5 = SUB42(s_on_1050_13c4, 0);
    }
    WritePrivateProfileString16(
        local_bx_5.field_0xa,
        CONCAT22(0x1050, u_var5),
        s_music_1050_13d2,
        s_general_1050_13b0,
    );
    if (local_bx_5.field_0x72 == 0) {
        u_var5 = SUB42(s_off_1050_13c8, 0);
    } else {
        u_var5 = SUB42(s_on_1050_13c4, 0);
    }
    WritePrivateProfileString16(
        local_bx_5.field_0xa,
        CONCAT22(0x1050, u_var5),
        s_sound_1050_13d8,
        s_general_1050_13b0,
    );
    if (local_bx_5.field_0x20 == 0) {
        u_var5 = SUB42(s_off_1050_13c8, 0);
    } else {
        u_var5 = SUB42(s_on_1050_13c4, 0);
    }
    WritePrivateProfileString16(
        local_bx_5.field_0xa,
        CONCAT22(0x1050, u_var5),
        s_movies_1050_13e8,
        s_general_1050_13b0,
    );
    u_var3 = local_bx_5.field_0xe;
    string_fn_1000_3f9c(ctx,
        u_var3,
        &ctx.s__lu_1050_14a2,
        &ctx.g_alloc_addr_1050_1050,
        local_bx_5.field_0x76,
    );
    WritePrivateProfileString16(
        local_bx_5.field_0xa,
        local_bx_5.field_0xe,
        s_turns_1050_1466,
        s_general_1050_13b0,
    );
    if local_bx_5.field_0x7a == 0 {
        u_var5 = ctx.s_off_1050_13c8;
    } else {
        u_var5 = ctx.s_on_1050_13c4;
    }
    WritePrivateProfileString16(
        local_bx_5.field_0xa,
        CONCAT22(0x1050, u_var5),
        s_turnsDlgStatus_1050_146c,
        s_general_1050_13b0,
    );
    WritePrivateProfileString16(
        local_bx_5.field_0xa,
        local_bx_5.field_0x1a,
        s_savePath_1050_147c,
        s_general_1050_13b0,
    );
    WritePrivateProfileString16(
        local_bx_5.field_0xa,
        local_bx_5.field_0x68,
        s_aiName_1050_1486,
        s_general_1050_13b0,
    );
    WritePrivateProfileString16(
        local_bx_5.field_0xa,
        local_bx_5.field_0x6c,
        s_playerName_1050_148e,
        s_general_1050_13b0,
    );
    local_c = 1;
    while {
        // switchD_1010:2ab5:
        write_private_profile_str_1010_622a(local_bx_5, u_var6, local_c);
        local_c = local_c + 1;
        local_c < 8
    } {}
    error_check_1000_17ce(ctx, local_bx_5.field_0xa);
    error_check_1000_17ce(ctx, local_bx_5.field_0xe);
    error_check_1000_17ce(ctx, local_bx_5.field_0x12);
    error_check_1000_17ce(ctx, local_bx_5.field_0x16);
    error_check_1000_17ce(ctx, local_bx_5.field_0x1a);
    pu_var1 = local_bx_5.field_0x64;
    u_var2 = local_bx_5.field_0x66;
    if ((u_var2 | pu_var1) != 0) {
        unsafe {
            ppcVar4 = *pu_var1;
        }
        (**ppcVar4)(0x1000, pu_var1, u_var2, 1);
    }
    error_check_1000_17ce(local_bx_5.field_0x68);
    error_check_1000_17ce(local_bx_5.field_0x6c);
    pass1_1010_1d80(param_1);
    return;
}

pub unsafe fn get_private_profile_str_1010_5404(ctx: &mut AppContext, param_1: &mut Struct64, param_2: u32) {
    let pi_var1: i32;
    let pu_var2: u32;
    let mut u_var3: u32;
    let ppcVar4: fn();
    let mut u_var5: u32;
    let u_var6: u8;
    let mut u_var7: i32;
    let mut u_var8: u16;
    let mut i_var9: i32;
    let i_var10: u16;
    let mut pa_var11: Struct103;
    let mut u_var12: i32;
    let mut struct_a: Struct103 = Struct103::new();
    let mut pa_var14: Struct103;
    let pu_var15: Vec<u8>;
    let mut u_var16: u16;
    let mut unaff_ss: u16;
    let pp_var17: Struct2111;
    let u_var18: u8;
    let u_var19: u8;
    let mut u_var20: u16;
    let mut u_var21: u16;
    let mut u_var22: u16;
    let mut u_var23: u16;
    let local_135: u8;
    let local_134: u8;
    let local_133: u8;
    let mut local_2e: u16;
    let mut local_2c: u16;
    let mut local_2a: u16;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: u32;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;
    let pu_var13: u16;

    // u_var22 = param_2;
    process_struct_1010_1d48(CONCAT22(u_var22, param_1), (param_2 >> 0x10));
    u_var8 = 0;
    param_1.field_0xa = 0;
    param_1.str_field_0xe = 0;
    param_1.field_0x12 = 0;
    param_1.field_0x16 = 0;
    param_1.field_0x1a = 0;
    param_1.field_0x62 = 0;
    param_1.field_0x64 = 0;
    param_1.field_0x68 = 0;
    param_1.field_0x6c = 0;
    param_1.field_0x70 = 1;
    param_1.field_0x7a = 0;
    param_1.field_0x7c = 0;
    param_1.field_0x7e = 0;
    param_1.field_0x80 = 0;
    param_1.field_0x82 = 1;
    CONCAT22(u_var22, param_1) = 0x6312;
    param_1.field_0x2 = 0x1010;
    str_fn_1010_6034(CONCAT22(u_var22, param_1));
    pa_var14 = struct_a;
    process_struct_1000_179c(0x101, &mut struct_a);
    param_1.str_field_0xe = u_var8;
    &param_1.field_0x10 = pa_var14;
    pass1_fn_1000_5008(ctx, &param_1.str_field_0xe, &pa_var14, 0x100);
    u_var7 = get_string_index_1000_3da4(&mut param_1.str_field_0xe);
    u_var5 = &param_1.str_field_0xe;
  // u_var16 = (u_var5  >> 0x10);
    pu_var15 = (u_var5 + u_var7);
    if pu_var15[-1] != '\\' {
        unsafe {
            *pu_var15 = 0x5c;
        }
        u_var5 = &param_1.str_field_0xe;
        *(u_var5 + u_var7 + 1) = 0;
    }
    local_4 = u_var7;
    load_string_1010_847e(
        ctx._g_struct_73_1050_14cc,
        (ctx._g_struct_73_1050_14cc >> 0x10),
        0x578,
    );
    pa_var14 = ctx.dx_reg;
    local_8 = u_var7;
    local_6 = ctx.dx_reg;
    process_string_1000_3cea(&param_1.str_field_0xe, CONCAT22(ctx.dx_reg, u_var7));
    u_var5 = &param_1.str_field_0xe;
    pass1_fn_1008_60e8(u_var5);
    param_1.field_0xa = u_var7;
    &param_1.field_0xc = pa_var14;
    u_var23 = SUB42(&ctx.g_alloc_addr_1050_1050, 0);
    u_var5 = &param_1.str_field_0xe;
    u_var16 = SUB42(offset, 0);
    u_var8 = GetPrivateProfileString16(
        CONCAT22(param_1.field_0xa, 0x1008),
        pa_var14,
        CONCAT22(u_var5, 0x100),
        CONCAT22(0x149a, (u_var5 >> 0x10)),
        0x13c01050,
        0x13b01050,
    );
    if (*&param_1.str_field_0xe != '\0') {
        u_var16 = 0x1000;
        pass1_fn_1000_3e2c(&param_1.str_field_0xe);
        local_16 = u_var8;
        pp_var17 = struct_ops_2::process_struct_1010_20ba(ctx.g_struct_var_1050_0ed0, CONCAT22(u_var23, 0x48));
      // pa_var14 = (pp_var17  >> 0x10);
        local_1a = pp_var17;
        local_a = (local_1a + 10);
        local_c = (local_1a + 0xc);
        param_1.field_0x62 = (local_16 != local_a);
        local_18 = pa_var14;
    }
    u_var5 = &param_1.str_field_0xe;
    u_var3 = &param_1.field_0xa;
    u_var23 = SUB42(offset, 0);
    GetPrivateProfileString16(
        CONCAT22(u_var3, u_var16),
        (u_var3 >> 0x10),
        CONCAT22(u_var5, 0x100),
        CONCAT22(0x149a, (u_var5 >> 0x10)),
        0x13de1050,
        0x13b01050,
    );
    if (*&param_1.str_field_0xe != '\0') {
        u_var23 = 0x1000;
        i_var9 = process_string_1000_475e(&param_1.str_field_0xe, s_on_1050_13c4);
        if (i_var9 == 0) {
            param_1.field_0x80 = 1;
        }
    }
    u_var5 = &param_1.str_field_0xe;
    u_var3 = &param_1.field_0xa;
    u_var16 = SUB42(offset, 0);
    GetPrivateProfileString16(
        CONCAT13((u_var3 >> 8), CONCAT12(u_var3, u_var23)),
        (u_var3 >> 0x10),
        CONCAT22(u_var5, 0x100),
        CONCAT22(0x149a, (u_var5 >> 0x10)),
        0x13d21050,
        0x13b01050,
    );
    if (*&param_1.str_field_0xe != '\0') {
        u_var16 = 0x1000;
        i_var9 = process_string_1000_475e(&param_1.str_field_0xe, s_off_1050_13c8);
        if (i_var9 == 0) {
            param_1.field_0x74 = 0;
        }
    }
    u_var5 = &param_1.str_field_0xe;
    u_var3 = &param_1.field_0xa;
    u_var23 = SUB42(offset, 0);
    GetPrivateProfileString16(
        CONCAT13((u_var3 >> 8), CONCAT12(u_var3, u_var16)),
        (u_var3 >> 0x10),
        CONCAT22(u_var5, 0x100),
        CONCAT22(0x149a, (u_var5 >> 0x10)),
        0x13d81050,
        0x13b01050,
    );
    if (*&param_1.str_field_0xe != '\0') {
        u_var23 = 0x1000;
        i_var9 = process_string_1000_475e(&param_1.str_field_0xe, s_off_1050_13c8);
        if (i_var9 == 0) {
            param_1.field_0x72 = 0;
        }
    }
    u_var5 = &param_1.str_field_0xe;
    u_var3 = &param_1.field_0xa;
    u_var16 = SUB42(offset, 0);
    GetPrivateProfileString16(
        CONCAT13((u_var3 >> 8), CONCAT12(u_var3, u_var23)),
        (u_var3 >> 0x10),
        CONCAT22(u_var5, 0x100),
        CONCAT22(0x149a, (u_var5 >> 0x10)),
        0x13cc1050,
        0x13b01050,
    );
    if (*&param_1.str_field_0xe != '\0') {
        u_var16 = 0x1000;
        i_var9 = process_string_1000_475e(&param_1.str_field_0xe, s_off_1050_13c8);
        if (i_var9 == 0) {
            param_1.field_0x1e = 0;
        }
    }
    u_var5 = &param_1.str_field_0xe;
    u_var3 = &param_1.field_0xa;
    u_var23 = SUB42(offset, 0);
    GetPrivateProfileString16(
        CONCAT13((u_var3 >> 8), CONCAT12(u_var3, u_var16)),
        (u_var3 >> 0x10),
        CONCAT13((u_var5 >> 8), CONCAT12(u_var5, 0x100)),
        CONCAT22(0x149a, (u_var5 >> 0x10)),
        0x13e81050,
        0x13b01050,
    );
    if (*&param_1.str_field_0xe != '\0') {
        u_var23 = 0x1000;
        i_var9 = process_string_1000_475e(&param_1.str_field_0xe, s_off_1050_13c8);
        if (i_var9 == 0) {
            param_1.field_0x20 = 0;
        }
    }
    u_var5 = &param_1.str_field_0xe;
    u_var3 = &param_1.field_0xa;
    u_var16 = SUB42(offset, 0);
    u_var7 = GetPrivateProfileString16(
        CONCAT22(u_var3, u_var23),
        (u_var3 >> 0x10),
        CONCAT13((u_var5 >> 8), CONCAT12(u_var5, 0x100)),
        CONCAT22(0x149a, (u_var5 >> 0x10)),
        0x14661050,
        0x13b01050,
    );
    pa_var11 = pa_var14;
    if (*&param_1.str_field_0xe != '\0') {
        u_var16 = 0x1000;
        pass1_fn_1000_3e2c(&param_1.str_field_0xe);
        pa_var11 = (pa_var14 | u_var7);
        local_2e = u_var7;
        local_2c = pa_var14;
        if ((pa_var14 | u_var7) != 0x0) {
            param_1.field_0x76 = u_var7;
            param_1.field_0x78 = pa_var14;
            pa_var11 = pa_var14;
        }
    }
    u_var21 = SUB42(&ctx.g_alloc_addr_1050_1050, 0);
    u_var5 = &param_1.str_field_0xe;
    u_var3 = &param_1.field_0xa;
    u_var23 = SUB42(offset, 0);
    GetPrivateProfileString16(
        CONCAT22(u_var3, u_var16),
        (u_var3 >> 0x10),
        CONCAT13((u_var5 >> 8), CONCAT12(u_var5, 0x100)),
        CONCAT13(0x14, CONCAT12(0x9a, (u_var5 >> 0x10))),
        0x146c1050,
        0x13b01050,
    );
    if (*&param_1.str_field_0xe != '\0') {
        u_var23 = 0x1000;
        i_var9 = process_string_1000_475e(&param_1.str_field_0xe, s_on_1050_13c4);
        if (i_var9 == 0) {
            param_1.field_0x7a = 1;
        }
    }
    u_var20 = SUB42(&ctx.g_alloc_addr_1050_1050, 0);
    u_var5 = &param_1.str_field_0xe;
    u_var3 = &param_1.field_0xa;
    u_var16 = SUB42(offset, 0);
    i_var10 = GetPrivateProfileString16(
        CONCAT13((u_var3 >> 8), CONCAT12(u_var3, u_var23)),
        (u_var3 >> 0x10),
        CONCAT13((u_var5 >> 8), CONCAT12(u_var5, 0x100)),
        CONCAT13(0x14, CONCAT12(0x9a, (u_var5 >> 0x10))),
        0x147c1050,
        0x13b01050,
    );
    if (*&param_1.str_field_0xe != '\0') {
        u_var5 = &param_1.str_field_0xe;
        u_var16 = SUB42(&ctx.PTR_LOOP_1050_1008, 0);
        pass1_fn_1008_60e8(u_var5, (u_var5 >> 0x10), u_var20, u_var21);
        param_1.field_0x1a = i_var10;
        param_1.field_0x1c = pa_var11;
    }
    u_var21 = SUB42(&ctx.g_alloc_addr_1050_1050, 0);
    u_var5 = &param_1.str_field_0xe;
    u_var3 = &param_1.field_0xa;
    u_var23 = SUB42(offset, 0);
    i_var10 = GetPrivateProfileString16(
        CONCAT22(u_var3, u_var16),
        (u_var3 >> 0x10),
        CONCAT22(u_var5, 0x100),
        CONCAT13(0x14, CONCAT12(0x9a, (u_var5 >> 0x10))),
        0x14861050,
        0x13b01050,
    );
    if (*&param_1.str_field_0xe != '\0') {
        u_var5 = &param_1.str_field_0xe;
        u_var23 = SUB42(&ctx.PTR_LOOP_1050_1008, 0);
        pass1_fn_1008_60e8(u_var5, (u_var5 >> 0x10), u_var21);
        param_1.field_0x68 = i_var10;
        param_1.field_0x6a = pa_var11;
    }
    u_var16 = SUB42(&ctx.g_alloc_addr_1050_1050, 0);
    u_var5 = &param_1.str_field_0xe;
    u_var3 = &param_1.field_0xa;
    u_var7 = GetPrivateProfileString16(
        CONCAT13((u_var3 >> 8), CONCAT12(u_var3, u_var23)),
        (u_var3 >> 0x10),
        CONCAT22(u_var5, 0x100),
        CONCAT13(0x14, CONCAT12(0x9a, (u_var5 >> 0x10))),
        0x148e1050,
        0x13b01050,
    );
    if (*&param_1.str_field_0xe != '\0') {
        u_var5 = &param_1.str_field_0xe;
        pass1_fn_1008_60e8(u_var5, (u_var5 >> 0x10), u_var16);
        param_1.field_0x6c = u_var7;
        param_1.field_0x6e = pa_var11;
    }
    if (param_1.field_0x62 == 0) {
        local_2e = winapi::GetSystemMetrics16(4);
        local_2a = 1;
        while {
            get_private_profile_str_1010_6132(param_1, u_var22, local_2a);
            i_var9 = &param_1.field_0x0 + local_2a * 8;
            pa_var11 = ctx.dx_reg;
            let pi_var1_val = unsafe { *pi_var1 };
            let pu_var2_val = unsafe { *pu_var2 };

            if (((((i_var9 + 0x22) < 0) || ((i_var9 + 0x24) < 0))
                || (
                    pi_var1 = (i_var9 + 0x22),
                    pi_var1_val != local_a - local_2e && (local_a - local_2e) <= pi_var1_val,
                ))
                || (
                    u_var7 = local_c - local_2e,
                    pu_var2 = (i_var9 + 0x24),
                    pu_var2_val != u_var7 && u_var7 <= pu_var2_val,
                ))
            {
                u_var6 = pass1_1000_4906(
                    CONCAT13(
                        (param_2 >> 8),
                        CONCAT12(param_2, &param_1.field_0x22 + local_2a * 8),
                    ),
                    0,
                    8,
                );
                u_var7 = CONCAT31(extraout_var, u_var6);
            }
            local_2a = local_2a + 1;
            local_2a < 8
        } {}
    }
    process_struct_1000_179c(0xc, &mut pa_var11);
    _local_32 = CONCAT22(pa_var11, u_var7);
    pa_var14 = (pa_var11 | u_var7);
    if (pa_var14 == 0x0) {
        pa_var11 = 0x0;
        pa_var14 = 0x0;
    } else {
        pa_var11 = process_struct_1008_574a(ctx, CONCAT22(pa_var11, u_var7));
    }
    param_1.field_0x64 = pa_var11;
    &param_1.field_0x66 = pa_var14;
    u_var5 = &param_1.str_field_0xe;
    pass1_fn_1000_5008(ctx, u_var5, (u_var5 >> 0x10), 0x100);
    u_var7 = get_string_index_1000_3da4(&mut param_1.str_field_0xe);
    u_var5 = &param_1.str_field_0xe;
  // u_var16 = (u_var5  >> 0x10);
    pu_var15 = (u_var5 + u_var7);
    if (pu_var15[-1] != '\\') {
        unsafe {
            *pu_var15 = 0x5c;
        }
        u_var5 = &param_1.str_field_0xe;
        *(u_var5 + u_var7 + 1) = 0;
    }
    u_var5 = &param_1.str_field_0xe;
    local_4 = u_var7;
    pass1_fn_1008_60e8(u_var5, (u_var5 >> 0x10));
    _local_10 = CONCAT22(pa_var14, u_var7);
    process_struct_1000_179c(8, &mut pa_var14);
    _local_32 = CONCAT22(pa_var14, u_var7);
    if ((pa_var14 | u_var7) == 0) {
        local_14 = 0;
    } else {
        _local_32 = ctx.s_1_1050_389a.clone();
        (u_var7 + 2) = ctx.PTR_LOOP_1050_1008;
        (u_var7 + 4) = _local_10;
        _local_32 = 0x6322;
        (u_var7 + 2) = 0x1010;
        local_14 = _local_32;
    }
    u_var5 = &param_1.field_0x64;
    ppcVar4 = (&param_1.field_0x64 + 4);
    (**ppcVar4)(0, u_var5, (u_var5 >> 0x10), local_14, (local_14 >> 0x10));
    u_var5 = &param_1.str_field_0xe;
    u_var3 = &param_1.field_0xa;
    u_var7 = ctx.dx_reg;
    u_var12 = GetPrivateProfileString16(
        CONCAT22(u_var3, 0x1000),
        (u_var3 >> 0x10),
        CONCAT22(u_var5, 0x100),
        CONCAT22(0x149a, (u_var5 >> 0x10)),
        0x13f01050,
        0x13b01050,
    );
    pu_var13 = u_var12;
    if (*&param_1.str_field_0xe != '\0') {
        u_var5 = &param_1.str_field_0xe;
        u_var18 = u_var5;
        u_var19 = (u_var5 >> 8);
      // u_var16 = (u_var5  >> 0x10);
        while (true) {
            u_var12 = pu_var13;
            pass1_fn_1000_47a4(CONCAT22(u_var16, CONCAT11(u_var19, u_var18)), s___1050_13f8);
            if ((u_var7 | u_var12) == 0) {
                break;
            }
            local_2e = u_var12;
            local_2c = u_var7;
            pa_var14 = copy_string_1000_3d3e(
                CONCAT22(unaff_ss, &local_134),
                CONCAT13((u_var7 >> 8), CONCAT12(u_var7, u_var12)),
            );
            local_4 = get_string_index_1000_3da4(CONCAT13(
                (unaff_ss >> 8),
                CONCAT12(unaff_ss, &local_134),
            ));
            if ((&local_135)[local_4] != '\\') {
                (&local_134)[local_4] = 0x5c;
                (&local_133)[local_4] = '\0';
            }
            pu_var13 = ZEXT24(&local_134);
            pass1_fn_1008_60e8(&local_134);
            _local_10 = pu_var13 & 0xffff | ZEXT24(pa_var14) << 0x10;
            process_struct_1000_179c(8, &mut pa_var14);
            u_var7 = pu_var13;
            _local_32 = (pu_var13 & 0xffff | ZEXT24(pa_var14) << 0x10);
            if ((pa_var14 | u_var7) == 0) {
                local_14 = 0;
            } else {
                _local_32 = ctx.s_1_1050_389a.clone();
                (u_var7 + 2) = ctx.PTR_LOOP_1050_1008;
                (u_var7 + 4) = _local_10;
                _local_32 = 0x6322;
                (u_var7 + 2) = 0x1010;
                pu_var13 = _local_32;
                local_14 = _local_32;
            }
            u_var5 = &param_1.field_0x64;
            ppcVar4 = (&param_1.field_0x64 + 8);
            (**ppcVar4)(
                0x1000,
                u_var5,
                (u_var5 >> 0x10),
                local_14,
                (local_14 >> 0x10),
            );
            u_var18 = 0;
            u_var19 = 0;
            u_var16 = 0;
            u_var7 = ctx.dx_reg;
        }
    }
    return;
}
