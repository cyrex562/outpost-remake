use crate::{
    defines::{
        pass1_struct_1, AppContext, Struct124, Struct318, Struct340, Struct371, Struct387,
        Struct44, Struct45, Struct46, Struct48, Struct52, Struct53, Struct594, Struct599, HANDLE16,
        HWND16,
    },
    draw::{get_sys_metrics_1020_7a50, select_and_delete_palette_1020_92c4},
    err_funcs::error_check_1000_17ce,
    pass6_funcs::pass1_1038_b6e0,
    pass7_funcs::pass1_1040_a5d0,
    pass8_funcs::{
        pass1_1010_038e, pass1_1010_1dda, pass1_1010_1ea6, pass1_1010_1f62, pass1_1010_2ee2,
        pass1_1010_32c0, pass1_1010_32da, pass1_1010_7b8c, pass1_1010_a568, pass1_1010_a58a,
        pass1_1010_a5ac, pass1_1010_a5ca, pass1_1010_a5ec, pass1_1010_ae92,
    },
    pass_funcs::{pass1_1000_4906, pass1_1008_b544, pass1_1040_c60e},
    struct_funcs::process_struct_1010_20ba,
    sys_funcs::{
        win_cleanup_func_1040_b0f8, DeleteObject16, DestroyMenu16, DestroyWindow16, EnableWindow16,
        FreeProcInstance16, GetClientRect16, GetDlgItem16, InvalidateRect16, IsWindow16,
        RemoveProp16, SendMessage16, ShowWindow16,
    },
    ui_funcs::{
        destroy_icon_func_1020_1038, destroy_menu_func_1020_795c, destroy_win_1008_628e,
        destroy_win_1010_2fa0, destroy_win_1040_7b98, free_proc_inst_1040_911e, pass1_1038_af40,
        set_window_pos_1038_abdc, update_window_1040_93aa, win_cleanup_1018_4d22,
        win_cleanup_func_1020_2fea, win_gui_fn_1010_32f4, win_gui_fn_1010_79aa,
        win_gui_fn_1040_b54a,
    },
    util::{CONCAT22, SUB42, ZEXT24},
    CONCAT11,
};

pub unsafe fn cleanup_1040_abe2(param_1: *mut Struct44, param_2: u8) -> *mut Struct44 {
    win_cleanup_func_1040_b0f8(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn win_cleanup_1010_0ee6(param_1: u32, param_2: u8) {
    win_cleanup_1018_4d22(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn cleanup_1010_17c0(param_1: *mut Struct340) {
    let pu_var1: *mut u32;
    let mut u_var2: i32;
    let local_bx_14: *mut Struct371;
    let mut u_var3: u16;
    let fn_ptr_1: fn();

    destroy_win_1010_2fa0(param_1);
    u_var3 = (param_1 >> 0x10);
    local_bx_14 = param_1;
    pu_var1 = local_bx_14.field_0x56;
    u_var2 = &local_bx_14.field_0x58;
    if ((u_var2 | pu_var1) != 0) {
        fn_ptr_1 = *pu_var1;
        (**fn_ptr_1)();
    }
    &local_bx_14.field_0x56 = 0;
    error_check_1000_17ce(local_bx_14.field_0x60);
    pass1_1000_4906(local_bx_14.field_0x64, 0, local_bx_14.field_0x68 << 2);
    error_check_1000_17ce(local_bx_14.field_0x64);
    local_bx_14.field_0x60 = 0;
    local_bx_14.field_0x64 = 0;
    return;
}

pub unsafe fn win_cleanup_1010_305a(param_1: *mut Struct318, param_2: i32, param_3: u32) {
    let pu_var1: *mut u16;
    let pi_var2: *mut i32;
    let mut u_var3: u32;
    let mut bVar4: bool;
    let mut u_var5: u16;
    let mut u_var6: u16;
    let mut i_var7: i32;
    let mut i_var8: i32;
    let mut u_var9: u16;
    let mut u_var10: u16;
    let mut local_e: u32;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;

    u_var10 = (param_3 >> 0x10);
    u_var6 = pass1_1040_c60e(param_3);
    u_var9 = (param_1 >> 0x10);
    i_var7 = param_1;
    (i_var7 + 0x12) = u_var6;
    (i_var7 + 0x14) = 0;
    local_6._0_2_ = 0;
    bVar4 = false;
    (i_var7 + 0x28) = 0;
    local_8 = 0;
    loop {
        pu_var1 = (i_var7 + 0x16);
        unsafe {
            if (*pu_var1 == local_8 || *pu_var1 < local_8) {
                // LAB_1010_30ad:
                local_8 = local_6;
                if (bVar4) {
                    while (
                        local_8 = local_8 + 1,
                        pu_var1 = (i_var7 + 0x16),
                        *pu_var1 != local_8 && local_8 <= *pu_var1,
                    ) {
                        u_var3 = (i_var7 + 0x2a + local_8 * 4);
                        DestroyWindow16((u_var3 + 0x18));
                        (i_var7 + local_8 * 4 + 0x2a) = 0;
                    }
                    (i_var7 + 0x16) = local_6 + 1;
                    pass1_1010_1f62(param_1, 9);
                } else {
                    i_var8 = (i_var7 + 0x16) * 4;
                    (i_var7 + i_var8 + 0x2a) = param_3;
                    (i_var7 + i_var8 + 0x2c) = u_var10;
                    local_a = 10;
                    pi_var2 = (i_var7 + 0x16);
                    *pi_var2 = *pi_var2 + 1;
                    if (1 < (i_var7 + 0x16)) {
                        u_var3 = (i_var7 + (i_var7 + 0x16) * 4 + 0x22);
                        i_var8 = u_var3;
                        u_var5 = (u_var3 >> 0x10);
                        local_a = (i_var8 + 0x20) + (i_var8 + 0x24) + 8;
                    }
                    update_window_1040_93aa(param_3, u_var10, local_a, (i_var7 + 0x1a));
                }
                if (!bVar4) {
                    pass1_1010_1f62(param_1, 10);
                }
                if (param_2 == 0) {
                    if ((i_var7 + 0x52) != 0) {
                        local_8 = 0;
                        while {
                            u_var3 = (i_var7 + 0x52);
                            u_var10 = (u_var3 >> 0x10);
                            i_var8 = u_var3;
                            if (((i_var8 + local_8 * 4) != 0)
                                && ((i_var8 + local_8 * 4) != param_3))
                            {
                                u_var3 = (i_var7 + 0x52);
                                u_var3 = (u_var3 + local_8 * 4);
                                DestroyWindow16((u_var3 + 0x18));
                            }
                            u_var3 = (i_var7 + 0x52);
                            (u_var3 + local_8 * 4) = 0;
                            local_8 = local_8 + 1;
                            local_8 < 10
                        } {}
                    }
                    pass1_1010_32da(param_1, param_3);
                    pass1_1010_1f62(param_1, 8);
                }
                return;
            }
        }
        if ((i_var7 + 0x2a + local_8 * 4) == param_3) {
            local_6._0_2_ = local_8;
            bVar4 = true;
            // goto LAB_1010_30ad;
        }
        local_8 = local_8 + 1;
    }
}

pub unsafe fn destroy_win_1010_3202(param_1: *mut Struct387, param_2: i32) {
    let pi_var1: *mut i32;
    let mut u_var2: u32;
    let mut i_var3: i32;
    let mut u_var4: u16;
    let mut local_4: u16;

    i_var3 = param_1;
    u_var4 = (param_1 >> 0x10);
    unsafe {
        if (param_2 == 0) {
            pi_var1 = (i_var3 + 0x28);
            *pi_var1 = *pi_var1 + -10;
            if (*pi_var1 < 0) {
                (i_var3 + 0x28) = 0;
            }
        } else {
            pi_var1 = (i_var3 + 0x28);
            *pi_var1 = *pi_var1 + (i_var3 + 0x18);
        }
        if ((i_var3 + 0x52) != 0) {
            local_4 = 0;
            while {
                u_var2 = (i_var3 + 0x52);
                if ((u_var2 + local_4 * 4) != 0) {
                    u_var2 = (i_var3 + 0x52);
                    u_var2 = (u_var2 + local_4 * 4);
                    DestroyWindow16((u_var2 + 0x18));
                    u_var2 = (i_var3 + 0x52);
                    (u_var2 + local_4 * 4) = 0;
                }
                local_4 = local_4 + 1;
                local_4 < 10
            } {}
        }
        if ((i_var3 + 0x16) == 0) {
            win_gui_fn_1010_32f4(param_1, (i_var3 + 0x56));
        } else {
            pass1_1010_32da(param_1, (i_var3 + (i_var3 + 0x16) * 4 + 0x26));
        }
    }
    pass1_1010_1f62(param_1, 8);
    return;
}

pub fn cleanup_1040_97da(param_1: *mut Struct44, param_2: u8) -> *mut Struct44 {
    free_proc_inst_1040_911e(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1040_83e6(param_1: u32, param_2: u8) {
    win_cleanup_func_1040_782c(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn destroy_window_1040_8212(param_1: *mut Struct53) {
    let is_window_result: bool;
    let local_struct_1: *mut Struct53;
    let local_struct_1_hi: *mut Struct53;

    local_struct_1_hi = (param_1 >> 0x10);
    local_struct_1 = param_1;
    if (local_struct_1.win_handle_0x8c != 0) {
        is_window_result = IsWindow16(local_struct_1.win_handle_0x8c);
        if (is_window_result != 0) {
            DestroyWindow16(local_struct_1.win_handle_0x8c);
            local_struct_1.win_handle_0x8c = 0;
        }
    }
    return;
}

pub unsafe fn win_cleanup_func_1040_782c(ctx: &mut AppContext, param_1: *mut Struct599) -> u8 {
    let pu_var1: *mut u8;
    let mut u_var2: i32;
    let ppc_var3: fn();
    let h_var4: HANDLE16;
    let local_bx_5: *mut Struct46;
    let mut u_var5: u16;
    let mut unaff_cs: u16;

    u_var5 = (param_1 >> 0x10);
    local_bx_5 = param_1;
    param_1.offset_1 = 0x840c;
    local_bx_5.field_0x2 = &ctx.PTR_LOOP_1050_1040;
    pu_var1 = local_bx_5.field_0x70;
    u_var2 = local_bx_5.field_0x72;
    if ((u_var2 | pu_var1) != 0) {
        ppc_var3 = *pu_var1;
        (**ppc_var3)();
    }
    if (local_bx_5.field_0x4 != 0) {
        // unaff_cs = offset;
        DeleteObject16(local_bx_5.field_0x4);
        local_bx_5.field_0x4 = 0;
    }
    if (local_bx_5.field_0x68 != 0) {
        DestroyMenu16(unaff_cs);
    }
    RemoveProp16(ctx.s_thisLo_1050_5db1, local_bx_5.h_wnd);
    RemoveProp16(ctx.s_thisHi_1050_5db8, local_bx_5.h_wnd);
    RemoveProp16(ctx.s_procLo_1050_5dbf, local_bx_5.h_wnd);
    h_var4 = RemoveProp16(ctx.s_procHi_1050_5dc6, local_bx_5.h_wnd);
    param_1.offset_1 = ctx.s_1_1050_389a;
    local_bx_5.field_0x2 = &ctx.PTR_LOOP_1050_1008;
    return h_var4;
}

pub fn ret_1040_78de() -> u8 {
    let in_AL: u8;

    return in_AL;
}

pub unsafe fn destroy_win_1040_52c0(
    ctx: &mut AppContext,
    param_1: *mut Struct124,
    param_2: u16,
    param_3: u16,
    param_4: u32,
) {
    let mut u_var1: u16;
    let ppc_var2: fn();
    let mut u_var3: u32;
    let mut i_var4: i32;
    let b_var5: bool;
    let mut u_var6: u16;
    let mut u_var7: u16;
    let mut h_wnd: HWND16;
    let mut u_var8: u16;
    let mut unaff_ss: HWND16;
    let pp_var9: *mut pass1_struct_1;
    let pu_var10: *mut u8;
    let u_var11: u8;
    let u_var12: u8;
    let mut u_var13: u16;
    let mut u_var14: u16;
    let mut local_22: u16;
    let mut local_20: u16;
    let mut local_1e: u16;
    let mut local_1c: u16;
    let mut local_1a: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    if (param_2._2_2_ != 0x10c) {
        if (param_2._2_2_ < 0x10d) {
            if (param_2._2_2_ == 0xfa) {
                ppc_var2 = (&param_1[1].field_0x4 + 0x18);
                ppc_var2();
                return;
            }
            if (param_2._2_2_ == 0x10a) {
                GetClientRect16(CONCAT22(unaff_ss, &local_a), &param_1.field_0x6);
                u_var3 = &param_1[1].field_0x4;
                local_8 = local_8 + 3;
                local_a = (u_var3 + 0x1a) - 9;
                local_6 = local_6 - 3;
                local_4 = local_4 - 3;
                InvalidateRect16(1, &local_a, unaff_ss);
                destroy_win_1010_2fa0(&param_1[1].field_0x4);
                pass1_1010_32c0(&param_1[1].field_0x4, 0);
                u_var3 = &param_1[1].field_0x4;
                local_22 = (u_var3 >> 0x10);
                pass1_1010_2ee2(u_var3, local_22);
                return;
            }
            if (param_2._2_2_ != 0x10b) {
                // LAB_1040_5560:
                win_gui_fn_1040_b54a(param_1, param_4, param_3, param_2);
                return;
            }
            u_var3 = &param_1[1].field_0x4;
            u_var6 = (u_var3 + 0x12);
            u_var7 = u_var6;
            pp_var9 = process_struct_1010_20ba(ctx._g_astruct_372_1050_0ed0, CONCAT22(u_var6, 3));
            u_var8 = (pp_var9 >> 0x10);
            i_var4 = pass1_1010_a5ca(pp_var9, u_var8, u_var7);
            if ((u_var6 != 0x70) && (i_var4 == 0)) {
                return;
            }
            u_var3 = &param_1[1].field_0x1c;
            u_var13 = u_var3;
            u_var14 = (u_var3 >> 0x10);
            u_var3 = &param_1[1].field_0x4;
            u_var1 = (u_var3 + 0x12);
            u_var11 = u_var1;
            u_var12 = (u_var1 >> 8);
        } else {
            if (param_2._2_2_ != 0x10d) {
                if (param_2._2_2_ == 0x10e) {
                    pp_var9 = process_struct_1010_20ba(
                        ctx._g_astruct_372_1050_0ed0,
                        CONCAT22(local_22, 0x32),
                    );
                    i_var4 = win_gui_fn_1010_79aa(pp_var9, 0xfc6, &param_1[1].field_0x1c);
                    if (i_var4 != 0) {
                        return;
                    }
                    u_var3 = &param_1[1].field_0x1c;
                    window_msg_func_1010_7300(pp_var9, 0, 0, 0x13, u_var3, (u_var3 >> 0x10));
                    return;
                }
                if (param_2._2_2_ != 0xbbb) {
                    if (param_2._2_2_ == 0xbbc) {
                        pp_var9 = process_struct_1010_20ba(
                            ctx._g_astruct_372_1050_0ed0,
                            CONCAT22(local_22, 3),
                        );
                        u_var8 = (pp_var9 >> 0x10);
                        u_var6 = pp_var9;
                        u_var7 = pass1_1010_a5ac(u_var6, u_var8, &param_1[1].field_0x1c);
                        i_var4 = pass1_1010_a58a(u_var6, u_var8, u_var7);
                        if (i_var4 == 0) {
                            pass1_1010_a568(u_var6, u_var8, u_var7);
                        }
                        h_wnd = GetDlgItem16(0xbbc, &param_1.field_0x6);
                        EnableWindow16(0, h_wnd);
                        return;
                    }
                    // goto LAB_1040_5560;
                }
                if ((&param_1[1].field_0x22 == 0)
                    || (b_var5 = IsWindow16(&param_1[1].field_0x22), b_var5 == 0))
                {
                    pu_var10 = pass1_1038_af40(ctx._g_astruct_112_a, *&param_1.field_0x6, 0x1b);
                    &param_1[1].field_0x22 = (pu_var10 + 6);
                    set_window_pos_1038_abdc(pu_var10);
                    ShowWindow16(1, &param_1[1].field_0x22);
                    return;
                }
                local_22 = &param_1[1].field_0x22;
                // goto LAB_1040_5417;
            }
            pp_var9 = process_struct_1010_20ba(ctx._g_astruct_372_1050_0ed0, CONCAT22(local_22, 3));
            u_var8 = (pp_var9 >> 0x10);
            u_var3 = &param_1[1].field_0x1c;
            u_var13 = u_var3;
            u_var14 = (u_var3 >> 0x10);
            u_var11 = 0x71;
            u_var12 = 0;
        }
        local_1e = pp_var9;
        pass1_1010_a5ec(
            local_1e,
            u_var8,
            CONCAT11(u_var12, u_var11),
            CONCAT22(u_var14, u_var13),
        );
        if ((&param_1[1].field_0x20 != 0)
            && (b_var5 = IsWindow16(&param_1[1].field_0x20), b_var5 != 0))
        {
            SendMessage16(0, 0xeb, 0x111, &param_1[1].field_0x20);
        }
    }
    local_22 = &param_1.field_0x6;
    // LAB_1040_5417:
    DestroyWindow16(local_22);
    return;
}

fn window_msg_func_1010_7300(
    pp_var9: *mut pass1_struct_1,
    arg_1: i32,
    arg_2: i32,
    arg_3: i32,
    u_var3_1: u32,
    u_var3_2: u32,
) -> () {
    todo!()
}

pub fn destroy_win_1040_5256(ctx: &mut AppContext, param_1: u32) {
    let pu_var1: *mut u32;
    let mut u_var2: i32;
    let ppc_var3: fn();
    let b_var4: bool;
    let local_bx_5: *mut Struct52;
    let mut u_var5: u16;
    let mut unaff_cs: u16;
    let mut offset = 0;

    u_var5 = (param_1 >> 0x10);
    local_bx_5 = param_1;
    if (local_bx_5.h_wnd != 0) {
        unaff_cs = SUB42(offset, 0);
        b_var4 = IsWindow16(local_bx_5.h_wnd);
        if (b_var4 != 0) {
            unaff_cs = SUB42(offset, 0);
            DestroyWindow16(local_bx_5.h_wnd);
        }
    }
    local_bx_5.h_wnd = 0;
    pu_var1 = local_bx_5.field_0x94;
    u_var2 = &local_bx_5.field_0x96;
    if ((u_var2 | pu_var1) != 0) {
        unsafe {
            ppc_var3 = *pu_var1;
            (**ppc_var3)(unaff_cs, pu_var1, u_var2, 1);
        }
    }
    &local_bx_5.field_0x94 = 0;
    local_bx_5.field_0x98 = 0;
    return;
}

pub unsafe fn pass1_1040_3506(ctx: &mut AppContext, param_1: *mut Struct599) {
    let mut u_var1: u16;

    u_var1 = (param_1 >> 0x10);
    param_1.offset_1 = (ctx.s_Null_Ptr_1050_38f3 + 7);
    (param_1 + 2) = &ctx.PTR_LOOP_1050_1040;
    pass1_1038_b6e0(ctx._g_astruct_112_a, *(param_1 + 6));
    win_cleanup_func_1040_782c(param_1);
    return;
}

pub unsafe fn pass1_1040_3532(ctx: &mut AppContext, param_1: *mut u8, param_2: *mut u8) {
    let local_DXAX_12: *mut pass1_struct_1;
    let b_var1: bool;

    b_var1 = 0;
    local_DXAX_12 = process_struct_1010_20ba(ctx._g_astruct_372_1050_0ed0, 0x2b);
    pass1_1010_038e(local_DXAX_12, b_var1);
    destroy_win_1040_7b98(param_1, param_2);
    return;
}

pub unsafe fn pass1_1040_2f06(ctx: &mut AppContext, param_1: *mut Struct599) {
    let mut u_var1: u16;

    u_var1 = (param_1 >> 0x10);
    param_1.offset_1 = 0x3436;
    (param_1 + 2) = &ctx.PTR_LOOP_1050_1040;
    pass1_1038_b6e0(ctx._g_astruct_112_a, *(param_1 + 6));
    win_cleanup_func_1040_782c(param_1);
    return;
}

pub unsafe fn pass1_1040_2f32(ctx: &mut AppContext, param_1: *mut u8, param_2: *mut u8) {
    let paVar1: *mut Struct318;
    let BVar2: bool;

    BVar2 = 0;
    paVar1 = process_struct_1010_20ba(ctx._g_astruct_372_1050_0ed0, 0x2b);
    pass1_1010_038e(paVar1, BVar2);
    destroy_win_1040_7b98(param_1, param_2);
    return;
}

pub unsafe fn pass1_1040_2a22(ctx: &mut AppContext, param_1: *mut Struct44) {
    let mut i_var1: i32;
    let mut u_var2: u16;

    u_var2 = (param_1 >> 0x10);
    i_var1 = param_1;
    param_1.offset = (ctx.s_add74_wav_1050_2e20 + 6);
    (i_var1 + 2) = &ctx.PTR_LOOP_1050_1040;
    error_check_1000_17ce((i_var1 + 0x94));
    error_check_1000_17ce((i_var1 + 0x98));
    win_cleanup_func_1040_b0f8(param_1);
    return;
}

pub unsafe fn pass1_1040_2464(ctx: &mut AppContext, param_1: *mut Struct599) {
    let mut u_var1: u16;

    u_var1 = (param_1 >> 0x10);
    param_1.offset_1 = (ctx.s_fem94_wav_1050_2950 + 6);
    (param_1 + 2) = &ctx.PTR_LOOP_1050_1040;
    pass1_1038_b6e0(ctx._g_astruct_112_a, *(param_1 + 6));
    win_cleanup_func_1040_782c(param_1);
    return;
}

pub unsafe fn pass1_1040_205e(ctx: &mut AppContext, param_1: *mut Struct599) {
    let pu_var1: *mut u32;
    let mut u_var2: i32;
    let ppc_var3: fn();
    let mut i_var4: i32;
    let mut u_var5: u16;

    u_var5 = (param_1 >> 0x10);
    i_var4 = param_1;
    param_1.offset_1 = (ctx.s_alarm_m_1050_2377 + 7);
    (i_var4 + 2) = &ctx.PTR_LOOP_1050_1040;
    pu_var1 = (i_var4 + 0x8e);
    u_var2 = (i_var4 + 0x90);
    if ((u_var2 | pu_var1) != 0) {
        ppc_var3 = *pu_var1;
        (**ppc_var3)();
    }
    error_check_1000_17ce((i_var4 + 0xa2));
    error_check_1000_17ce((i_var4 + 0xa6));
    pass1_1038_b6e0(ctx._g_astruct_112_a, *(i_var4 + 6));
    win_cleanup_func_1040_782c(param_1);
    return;
}

pub unsafe fn pass1_1040_1876(ctx: &mut AppContext, param_1: *mut Struct599) {
    let mut u_var1: u16;

    u_var1 = (param_1 >> 0x10);
    param_1.offset_1 = (ctx.s_202_flc_1050_1c46 + 2);
    (param_1 + 2) = &ctx.PTR_LOOP_1050_1040;
    pass1_1038_b6e0(ctx._g_astruct_112_a, *(param_1 + 6));
    win_cleanup_func_1040_782c(param_1);
    return;
}

pub unsafe fn pass1_1040_1290(ctx: &mut AppContext, param_1: *mut Struct599) {
    let mut u_var1: u16;

    u_var1 = (param_1 >> 0x10);
    param_1.offset_1 = 0x17b0;
    (param_1 + 2) = &ctx.PTR_LOOP_1050_1040;
    pass1_1038_b6e0(ctx._g_astruct_112_a, *(param_1 + 6));
    win_cleanup_func_1040_782c(param_1);
    return;
}

pub unsafe fn pass1_1040_1152(
    ctx: &mut AppContext,
    param_1: i32,
    param_2: *mut u8,
    param_3: *mut u8,
) {
    let mut u_var1: u16;
    let mut u_var2: u32;
    let mut i_var3: i32;
    let mut u_var4: u16;
    let ppVar5: *mut pass1_struct_1;
    let mut in_stack_0000fff4: u16;
    let mut local_4: u16;

    if ((param_1 + 0x92) != 0) {
        u_var2 = (param_1 + 0x8e);
        u_var1 = (u_var2 + 10);
        ppVar5 =
            process_struct_1010_20ba(ctx._g_astruct_372_1050_0ed0, CONCAT22(in_stack_0000fff4, 3));
        u_var2 = (param_1 + 0x92);
        u_var4 = (u_var2 >> 0x10);
        i_var3 = u_var2;
        pass1_1010_ae92(ppVar5, u_var1, (i_var3 + 10), (i_var3 + 6));
    }
    destroy_win_1040_7b98(param_1, param_2, param_3);
    ctx.PTR_LOOP_1050_5b80 = 0x0;
    return;
}

pub unsafe fn pass1_1040_0e86(ctx: &mut AppContext, param_1: *mut Struct599) {
    let mut u_var1: i32;
    let in_struct_1: *mut Struct44;
    let mut i_var2: i32;
    let mut u_var3: u16;
    let ppVar4: *mut pass1_struct_1;
    let mut in_stack_0000ffee: u16;
    let mut local_6: u32;

    u_var3 = (param_1 >> 0x10);
    i_var2 = param_1;
    param_1.offset_1 = (ctx.s_overflow_on_node__d_1050_11ca + 8);
    (i_var2 + 2) = &ctx.PTR_LOOP_1050_1040;
    in_struct_1 = (i_var2 + 0x92);
    u_var1 = (i_var2 + 0x94);
    if ((u_var1 | in_struct_1) != 0) {
        pass1_1040_a5d0((in_struct_1 & 0xffff | u_var1 << 0x10));
        error_check_1000_17ce(in_struct_1);
    }
    ctx.PTR_LOOP_1050_5b82 = (i_var2 + 0x96);
    if ((i_var2 + 0x92) == 0) {
        pass1_1038_b6e0(ctx._g_astruct_112_a, *(i_var2 + 6));
    } else {
        ppVar4 = process_struct_1010_20ba(
            ctx._g_astruct_372_1050_0ed0,
            CONCAT22(in_stack_0000ffee, 0x32),
        );
        pass1_1010_7b8c(ppVar4, (i_var2 + 6));
    }
    win_cleanup_func_1040_782c(param_1);
    return;
}

pub unsafe fn pass1_1040_0c54(ctx: &mut AppContext, param_1: *mut Struct599) {
    let mut u_var1: u16;

    u_var1 = (param_1 >> 0x10);
    param_1.offset_1 = 0xdb0;
    (param_1 + 2) = &ctx.PTR_LOOP_1050_1040;
    (param_1 + 0x8e) = 0;
    win_cleanup_func_1040_782c(param_1);
    return;
}

pub unsafe fn pass1_1040_073a(ctx: &mut AppContext, param_1: *mut Struct599) {
    let mut u_var1: u16;

    u_var1 = (param_1 >> 0x10);
    param_1.offset_1 = 0xb90;
    (param_1 + 2) = &ctx.PTR_LOOP_1050_1040;
    pass1_1038_b6e0(ctx._g_astruct_112_a, *(param_1 + 6));
    win_cleanup_func_1040_782c(ctx, param_1);
    return;
}

pub unsafe fn pass1_1040_0656(param_1: *mut Struct44, param_2: u8) -> *mut Struct44 {
    win_cleanup_1038_ef3a(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn win_cleanup_1038_ef3a(ctx: &mut AppContext, param_1: *mut Struct599) {
    let mut u_var1: u32;
    let mut i_var2: i32;
    let mut u_var3: u16;

    u_var3 = (param_1 >> 0x10);
    i_var2 = param_1;
    param_1.offset_1 = 0x67c;
    (i_var2 + 2) = &ctx.PTR_LOOP_1050_1040;
    if ((i_var2 + 0x96) != 0) {
        u_var1 = (i_var2 + 0x96);
        DestroyWindow16((u_var1 + 6));
        (i_var2 + 0x96) = 0;
    }
    pass1_1038_b6e0(ctx._g_astruct_112_a, *(i_var2 + 6));
    win_cleanup_func_1040_782c(param_1);
    return;
}

pub unsafe fn pass1_1038_ebd6(ctx: &mut AppContext, param_1: *mut Struct599) {
    let mut i_var1: i32;
    let mut u_var2: u16;

    u_var2 = (param_1 >> 0x10);
    i_var1 = param_1;
    param_1.offset_1 = 0xee6e;
    (i_var1 + 2) = &ctx.PTR_LOOP_1050_1038;
    pass1_1038_b6e0(ctx._g_astruct_112_a, *(i_var1 + 6));
    error_check_1000_17ce((i_var1 + 0x8e));
    win_cleanup_func_1040_782c(param_1);
    return;
}

pub unsafe fn pass1_1038_e308(ctx: &mut AppContext, param_1: *mut Struct599) {
    let mut i_var1: i32;
    let mut u_var2: u16;

    u_var2 = (param_1 >> 0x10);
    i_var1 = param_1;
    param_1.offset_1 = 0xe62e;
    (i_var1 + 2) = &ctx.PTR_LOOP_1050_1038;
    pass1_1038_b6e0(ctx._g_astruct_112_a, *(i_var1 + 6));
    error_check_1000_17ce((i_var1 + 0x8e));
    win_cleanup_func_1040_782c(param_1);
    return;
}

pub unsafe fn pass1_1038_e16e(ctx: &mut AppContext, param_1: *mut Struct599) {
    let mut u_var1: u16;

    u_var1 = (param_1 >> 0x10);
    param_1.offset_1 = 0xe264;
    (param_1 + 2) = &ctx.PTR_LOOP_1050_1038;
    pass1_1038_b6e0(ctx._g_astruct_112_a, *(param_1 + 6));
    win_cleanup_func_1040_782c(param_1);
    return;
}

pub unsafe fn pass1_1038_d7d0(ctx: &mut AppContext, param_1: *mut Struct45) {
    let mut i_var1: i32;
    let mut u_var2: u16;

    u_var2 = (param_1 >> 0x10);
    i_var1 = param_1;
    param_1.u16_0x0 = 0xe0d4;
    (i_var1 + 2) = &ctx.PTR_LOOP_1050_1038;
    if ((i_var1 + 0x90) != 0) {
        pass1_1010_1ea6(ctx._g_struct_ptr_1050_02a0, param_1);
    }
    if ((i_var1 + 0x92) != 0) {
        pass1_1010_1ea6(*(i_var1 + 0x92), param_1);
    }
    pass1_1038_b6e0(ctx._g_astruct_112_a, *(i_var1 + 6));
    error_check_1000_17ce((i_var1 + 0x96));
    win_cleanup_func_1040_782c(param_1);
    return;
}

pub unsafe fn pass1_1038_d276(ctx: &mut AppContext, param_1: *mut Struct599) {
    let mut u_var1: u16;

    u_var1 = (param_1 >> 0x10);
    param_1.offset_1 = 0xd6ea;
    (param_1 + 2) = &ctx.PTR_LOOP_1050_1038;
    pass1_1038_b6e0(ctx._g_astruct_112_a, *(param_1 + 6));
    win_cleanup_func_1040_782c(param_1);
    return;
}

pub fn pass1_1038_d218(param_1: *mut Struct44, param_2: u8) -> *mut Struct44 {
    free_proc_inst_1038_cfda(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn free_proc_inst_1038_cfda(ctx: &mut AppContext, param_1: *mut u16) {
    let mut i_var1: i32;
    let mut u_var2: u16;
    let mut unaff_cs: u16;

    u_var2 = (param_1 >> 0x10);
    i_var1 = param_1;
    unsafe { *param_1 = 0xd23e };
    (i_var1 + 2) = &ctx.PTR_LOOP_1050_1038;
    FreeProcInstance16(CONCAT22((i_var1 + 4), unaff_cs));
    FreeProcInstance16(CONCAT22(ctx._PTR_LOOP_1050_5bcc, 0x1538));
    (i_var1 + 4) = 0;
    unsafe { *param_1 = ctx.s_1_1050_389a };
    (i_var1 + 2) = &ctx.PTR_LOOP_1050_1008;
    return;
}

pub unsafe fn pass1_1038_cd5c(ctx: &mut AppContext, param_1: *mut Struct599) {
    let mut u_var1: u16;

    u_var1 = (param_1 >> 0x10);
    param_1.offset_1 = 0xcf00;
    (param_1 + 2) = &ctx.PTR_LOOP_1050_1038;
    pass1_1038_b6e0(ctx._g_astruct_112_a, *(param_1 + 6));
    win_cleanup_func_1040_782c(param_1);
    return;
}

pub unsafe fn pass1_1038_cb30(ctx: &mut AppContext, param_1: *mut Struct599) {
    let mut u_var1: u16;

    u_var1 = (param_1 >> 0x10);
    param_1.offset_1 = 0xcc9a;
    (param_1 + 2) = &ctx.PTR_LOOP_1050_1038;
    pass1_1038_b6e0(ctx._g_astruct_112_a, *(param_1 + 6));
    win_cleanup_func_1040_782c(param_1);
    return;
}

pub fn cleanup_fn_1020_0e2c(param_1: *mut Struct48) {
    get_sys_metrics_1020_7a50(param_1);
    destroy_icon_func_1020_1038(param_1);
    return;
}

pub fn pass1_1038_ae08(ctx: &mut AppContext, param_1: *mut Struct44) {
    param_1.offset = 0xae4e;
    (param_1 + 2) = &ctx.PTR_LOOP_1050_1038;
    win_cleanup_func_1040_b0f8(param_1);
    return;
}

pub unsafe fn pass1_1038_a156(ctx: &mut AppContext, param_1: *mut Struct599) {
    param_1.offset_1 = 0xa2d0;
    (param_1 + 2) = &ctx.PTR_LOOP_1050_1038;
    win_cleanup_func_1040_782c(param_1);
    return;
}

pub fn destroy_win_1038_a072(param_1: u32, param_2: i32) {
    if (param_2 != 0) {
        DestroyWindow16((param_1 + 6));
    }
    return;
}

pub fn pass1_1038_9ed4(param_1: *mut Struct44, param_2: u8) -> *mut Struct44 {
    win_cleanup_func_1040_b0f8(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1038_9a48(ctx: &mut AppContext, param_1: *mut Struct44) {
    param_1.offset = 0x9af6;
    (param_1 + 2) = &ctx.PTR_LOOP_1050_1038;
    win_cleanup_func_1040_b0f8(param_1);
    return;
}

pub fn pass1_1038_997c(param_1: *mut Struct44, param_2: u8) -> *mut Struct44 {
    win_cleanup_func_1040_b0f8(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1038_7d5c(ctx: &mut AppContext, param_1: *mut Struct44) {
    let mut u_var1: u16;

    u_var1 = (param_1 >> 0x10);
    param_1.offset = 0x8876;
    (param_1 + 2) = &ctx.PTR_LOOP_1050_1038;
    pass1_1038_b6e0(ctx._g_astruct_112_a, *(param_1 + 6));
    win_cleanup_func_1040_b0f8(param_1);
    return;
}

pub unsafe fn destroy_win_1038_7d88(param_1: u32, param_2: i32) {
    let mut u_var1: u16;

    u_var1 = (param_1 >> 0x10);
    pass1_1008_b544((param_1 + 0x94), param_2);
    DestroyWindow16((param_1 + 6));
    return;
}

pub fn cleanup_fn_1020_96a2(param_1: *mut Struct44, param_2: u8) -> *mut Struct44 {
    select_and_delete_palette_1020_92c4(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn cleanup_fn_1020_7b60(param_1: *mut Struct44, param_2: u8) -> *mut Struct44 {
    destroy_menu_func_1020_795c(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn cleanup_fn_1020_78ac(param_1: *mut Struct44) {
    let local_struct_1: *mut Struct44;
    let local_struct_1_hi: *mut Struct44;

    local_struct_1_hi = (param_1 >> 0x10);
    local_struct_1 = param_1;
    param_1.offset = 0x7902;
    local_struct_1.ptr_a_hi = 0x1020;
    if (local_struct_1.field_0x14 != 0) {
        pass1_1010_1dda(local_struct_1.field_0x14);
    }
    select_and_delete_palette_1020_92c4(param_1);
    return;
}

pub unsafe fn cleanup_fn_1020_78dc(param_1: *mut Struct44, param_2: u8) -> *mut Struct44 {
    cleanup_fn_1020_78ac(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn win_cleanup_fn_1020_770e(in_struct_1: *mut Struct594) {
    let pu_var1: *mut u32;
    let mut u_var2: i32;
    let local_struct_1: *mut Struct594;
    let local_struct_1_hi: *mut Struct594;
    let mut in_stack_0000fff6: u16;
    let fn_ptr_1_1: fn();

    local_struct_1_hi = (in_struct_1 >> 0x10);
    local_struct_1 = in_struct_1;
    pu_var1 = local_struct_1.u8_ptr_16_xee;
    u_var2 = local_struct_1.field_0xf0;
    if ((u_var2 | pu_var1) != 0) {
        unsafe {
            fn_ptr_1_1 = *pu_var1;
            (**fn_ptr_1_1)();
        }
    }
    &local_struct_1.u8_ptr_16_xee = 0;
    destroy_win_1008_628e(
        (in_struct_1 & 0xffff | ZEXT24(local_struct_1_hi) << 0x10),
        in_stack_0000fff6,
    );
    return;
}

pub fn call_win_cleanup_fn_1020_3616(in_struct_1: *mut Struct44, param_2: u8) -> *mut Struct44 {
    win_cleanup_func_1020_2fea(in_struct_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(in_struct_1);
    }
    return in_struct_1;
}
