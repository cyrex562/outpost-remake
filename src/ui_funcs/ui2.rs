use crate::app_context::AppContext;
use crate::string_funcs::{copy_string_1000_3d3e, process_string_1000_3cea};
use crate::ui_funcs::ui1;
use crate::util::{CONCAT22, SBORROW2};
use crate::winapi_funcs::{MessageBox16, PostMessage16};

pub fn create_win_1040_7620(
    param_1: u32,
    param_2: i32,
    param_3: *mut u16,
    param_4: u16,
    HMENmenu: u16,
) {
    let mut window_name: string;
    let local_bx_49: *mut Struct39;
    let mut u_var1: u16;
    let mut local_a: u32;

    window_name = load_string_1010_847e(
        ctx._g_struct_73_1050_14cc,
        (ctx._g_struct_73_1050_14cc >> 0x10),
        param_4,
    );
    local_a = 0x50000009;
    if (param_2 != 0) {
        local_a = 0x50020009;
    }
    u_var1 = (param_3 >> 0x10);
    local_bx_49 = param_3;
    CreateWindow16(
        0,
        ctx.g_h_instance_1050_038c,
        menu,
        (param_1 + 6),
        local_bx_49.field_0x6,
        local_bx_49.field_0x4,
        local_bx_49.field_0x2,
        unsafe { *param_3 },
        local_a,
        window_name,
        s_button_1050_5da8,
    );
    return;
}

pub fn create_win_1040_70b4(param_1: *mut Struct20) {
    let mut u_var1: u32;
    let mut u_var2: u32;
    let ppc_var3: fn();
    let mut u_var4: i32;
    let h_wnd: *mut u32;
    let mut i_var5: i32;
    let struct_a: *mut Struct199;

    let struct_a_00: *mut Struct199;
    let struct_a_01: *mut Struct199;
    let ctx.dx_reg: *mut Struct199;
    let paVar6: *mut Struct199;
    let ctx.dx_reg: *mut Struct199;

    let struct_a_02: *mut Struct199;

    let mut i_var7: i32;
    let mut u_var8: u16;
    let mut u_var9: u16;
    let mut unaff_ss: u16;
    let ppVar10: *mut pass1_struct_1;
    let dVar11: u32;
    let u_var12: u8;
    let u_var13: u8;
    let u_var14: u8;
    let u_var15: u8;
    let u_var16: u8;
    let u_var17: u8;
    let mut hdc: u16;
    let mut in_stack_0000ff8a: u16;
    let mut local_6e: u16;
    let mut local_6c: u16;
    let mut local_6a: u32;
    let mut local_66: u16;
    let mut local_64: u32;
    let mut local_60: u32;
    let mut local_5c: u16;
    let mut local_5a: u16;
    let mut local_58: [u8; 80];
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    ui1::win_gui_func_1040_78e2(param_1);
    ppVar10 = process_struct_1010_20ba(
        ctx._g_Struct372_1050_0ed0,
        CONCAT22(in_stack_0000ff8a, 0x34),
    );
    u_var4 = ppVar10;
    u_var8 = (param_1 >> 0x10);
    i_var7 = param_1;
    (i_var7 + 0x94) = u_var4;
    (i_var7 + 0x96) = (ppVar10 >> 0x10);
    u_var9 = (i_var7 + 0x94);
    ppc_var3 = ((i_var7 + 0x94) + 4);
    (**ppc_var3)(0x10, u_var9, (ppVar10 >> 0x10), 0, i_var7, u_var8);
    paVar6 = struct_a;
    process_struct_1000_179c(10, struct_a);
    if ((paVar6 | u_var4) == 0) {
        (i_var7 + 0x98) = 0;
    } else {
        process_struct_1040_bf3e(
            CONCAT13((paVar6 >> 8), CONCAT12(paVar6, u_var4)),
            (i_var7 + 6),
        );
        (i_var7 + 0x98) = u_var4;
        (i_var7 + 0x9a) = ctx.dx_reg;
    }
    pass1_1040_bfde((i_var7 + 0x98), (i_var7 + 0x94));
    paVar6 = struct_a_00;
    process_struct_1000_179c(0x42, struct_a_00);
    struct_a_01 = (paVar6 | u_var4);
    if (struct_a_01 != 0x0) {
        ui1::win_fn_1008_3bd6(
            u_var4,
            paVar6,
            1,
            0xa000a,
            0x810000,
            0x10a0080,
            CONCAT22(u_var9, (i_var7 + 6)),
        );
        struct_a_01 = ctx.dx_reg;
    }
    process_struct_1000_179c(0x42, struct_a_01);
    paVar6 = (struct_a_01 | u_var4);
    if (paVar6 != 0x0) {
        ui1::win_fn_1008_3bd6(
            u_var4,
            struct_a_01,
            1,
            0xa0028,
            0x830000,
            0x10c0082,
            CONCAT22(u_var9, (i_var7 + 6)),
        );
        paVar6 = ctx.dx_reg;
    }
    u_var12 = 0;
    u_var16 = 0;
    process_struct_1000_179c(0x42, paVar6);
    if ((paVar6 | u_var4) == 0) {
        u_var4 = 0;
        local_4 = 0;
    } else {
        ui1::win_fn_1008_3bd6(
            u_var4,
            paVar6,
            1,
            0xa00aa,
            0x1000101,
            0x10700ff,
            CONCAT13(u_var16, CONCAT12(u_var12, (i_var7 + 6))),
        );
        local_4 = ctx.dx_reg;
    }
    local_6 = u_var4;
    ui1::enable_window_1040_9234(u_var4, local_4, u_var12);
    u_var12 = 0;
    u_var16 = 0;
    paVar6 = struct_a_02;
    process_struct_1000_179c(0x42, struct_a_02);
    if ((paVar6 | u_var4) == 0) {
        u_var4 = 0;
        local_4 = 0;
    } else {
        ui1::win_fn_1008_3bd6(
            u_var4,
            paVar6,
            1,
            0xa00c2,
            0x1000101,
            0x10800ff,
            CONCAT13(u_var16, CONCAT12(u_var12, (i_var7 + 6))),
        );
        local_4 = ctx.dx_reg;
    }
    local_6 = u_var4;
    ui1::enable_window_1040_9234(u_var4, local_4, u_var12);
    local_8 = GetDC16((i_var7 + 6));
    u_var15 = unaff_ss;
    u_var17 = (unaff_ss >> 8);
    load_string_1010_84e0(
        ctx._g_struct_73_1050_14cc,
        (ctx._g_struct_73_1050_14cc >> 0x10),
        0x50,
        CONCAT13(u_var17, CONCAT12(u_var15, local_58)),
        0x7cd,
    );
    u_var13 = SUB21(local_58, 0);
    u_var14 = (local_58 >> 8);
    u_var12 = u_var15;
    u_var16 = u_var17;
    hdc = local_8;
    u_var4 = get_string_index_1000_3da4(CONCAT22(unaff_ss, local_58));
    dVar11 = GetTextExtent16(
        u_var4,
        CONCAT13(u_var16, CONCAT12(u_var12, CONCAT11(u_var14, u_var13))),
        hdc,
    );
    local_5a = (dVar11 >> 0x10);
    local_5c = dVar11;
    CreateWindow16(
        0,
        ctx.g_h_instance_1050_038c,
        0x7cd,
        (i_var7 + 6),
        local_5a,
        local_5c,
        0xad,
        0x22,
        0x50000000,
        CONCAT13(u_var17, CONCAT12(u_var15, local_58)),
        s_static_1050_5d9a,
    );
    load_string_1010_84e0(
        ctx._g_struct_73_1050_14cc,
        (ctx._g_struct_73_1050_14cc >> 0x10),
        0x50,
        CONCAT22(unaff_ss, local_58),
        0x7ce,
    );
    u_var13 = local_8;
    u_var14 = (local_8 >> 8);
    u_var12 = SUB21(local_58, 0);
    u_var16 = (local_58 >> 8);
    u_var4 = get_string_index_1000_3da4(CONCAT13(u_var17, CONCAT12(u_var15, local_58)));
    dVar11 = GetTextExtent16(
        u_var4,
        CONCAT22(unaff_ss, CONCAT11(u_var16, u_var12)),
        CONCAT11(u_var14, u_var13),
    );
    local_5a = (dVar11 >> 0x10);
    local_5c = dVar11;
    ReleaseDC16(local_8, (i_var7 + 6));
    CreateWindow16(
        0,
        ctx.g_h_instance_1050_038c,
        0x7ce,
        (i_var7 + 6),
        local_5a,
        local_5c,
        0xc5,
        0x22,
        0x50000000,
        CONCAT13(u_var17, CONCAT12(u_var15, local_58)),
        s_static_1050_5da1,
    );
    local_64 = 0x5a000a;
    local_60 = 0x140050;
    h_wnd = &local_64;
    u_var12 = SUB41(param_1, 0);
    create_win_1040_7620(
        u_var12,
        u_var8,
        1,
        h_wnd,
        u_var15,
        0x5eb,
        0xfd,
        (i_var7 + 6),
    );
    local_64 = local_64 & 0xffff | (local_64._2_2_ + 0x14) << 0x10;
    create_win_1040_7620(u_var12, u_var8, 0, 0x9c, u_var15, 0x5ed, 0xfe, (i_var7 + 6));
    local_64 = local_64 & 0xffff | (local_64._2_2_ + 0x14) << 0x10;
    create_win_1040_7620(u_var12, u_var8, 0, 0x9c, u_var15, 0x5ef, 0xff, (i_var7 + 6));
    SendMessage16(0, 1, 0x401, h_wnd);
    u_var1 = (i_var7 + 0x94);
    i_var5 = u_var1;
    u_var1 = u_var1 & 0xffff0000;
    u_var9 = (u_var1 >> 0x10);
    SetWindowPos16(
        0x40,
        (i_var5 + 0x10),
        (i_var5 + 0xe),
        (i_var5 + 0xc),
        (u_var1 | i_var5 + 10),
        0,
        (i_var7 + 6),
    );
    u16_1050_0ecc = 0;
    u_var2 = (i_var7 + 0x94);
    ppc_var3 = ((i_var7 + 0x94) + 0x10);
    (**ppc_var3)(offset, u_var2, (u_var2 >> 0x10));
    u_var2 = (i_var7 + 0x94);
    pass1_1010_2ee2(u_var2, (u_var2 >> 0x10));
    PostMessage16(0, 0x10a, 0x111, (i_var7 + 6));
    return;
}

pub fn enable_window_1040_6ff2(param_1: u32, param_2: i32) {
    let mut u_var1: u32;
    let HVar2: HWND16;
    let mut i_var3: i32;
    let mut u_var4: u16;

    if (param_2 == 8) {
        u_var4 = (param_1 >> 0x10);
        i_var3 = param_1;
        HVar2 = GetDlgItem16(0x107, (i_var3 + 6));
        u_var1 = (i_var3 + 0x94);
        EnableWindow16((u_var1 + 0x24), HVar2);
        HVar2 = GetDlgItem16(0x108, (i_var3 + 6));
        u_var1 = (i_var3 + 0x94);
        EnableWindow16((u_var1 + 0x26), HVar2);
    }
    return;
}

pub fn create_win_1040_6eae(
    param_1: u32,
    param_2: u16,
    param_3: *mut Struct38,
    param_4: u16,
    menu: u16,
) {
    let mut window_name: string;
    let local_bx_49: *mut Struct38;
    let mut in_stack_0000000c: u16;
    let mut local_a: u32;

    window_name = load_string_1010_847e(
        ctx._g_struct_73_1050_14cc,
        (ctx._g_struct_73_1050_14cc >> 0x10),
        param_4,
    );
    local_a = 0x50000009;
    if (param_2 != 0) {
        local_a = 0x50020009;
    }
    CreateWindow16(
        0,
        ctx.g_h_instance_1050_038c,
        menu,
        (param_1 + 6),
        param_3.field_0x6,
        param_3.field_0x4,
        param_3.field_0x2,
        *_param_3,
        local_a,
        window_name,
        s_button_1050_5d92,
    );
    return;
}

pub fn destroy_win_1040_6d1a(param_1: *mut Struct124, param_2: u16, param_3: u16, param_4: u32) {
    let pp_var1: fn();
    let mut u_var2: u32;
    let unaff_ss: HWND16;
    let mut u_var3: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    match (param_2._2_2_) {
        0xfa => {
            pp_var1 = ((param_1 + 1) + 0x18);
            (**pp_var1)()
        }
        _ => {
            ui1::win_gui_fn_1040_b54a(param_1, param_2_00, param_3, param_2);
            return;
        }
        0xfd => {
            if (u16_1050_0ecc == 0) {
                return;
            }
            u16_1050_0ecc = 0;
            // goto LAB_1040_6deb;
        }
        0xfe => {
            if (u16_1050_0ecc == 1) {
                return;
            }
            u16_1050_0ecc = 1;
            // goto LAB_1040_6deb;
        }
        0xff => {
            if (u16_1050_0ecc == 2) {
                return;
            }
            u16_1050_0ecc = 2;
            // LAB_1040_6deb:
            u_var2 = (param_1 + 1);
            pp_var1 = ((param_1 + 1) + 0x10);
            (**pp_var1)(0x40, u_var2, (u_var2 >> 0x10));
            u_var2 = (param_1 + 1);
            pass1_1010_2ee2(u_var2, (u_var2 >> 0x10));
            PostMessage16(0, 0x10a, 0x111, &param_1.field_0x6)
        }
        0x107 => {
            u_var3 = 0;
            // goto LAB_1040_6e48;
        }
        0x108 => {
            u_var3 = 1;
            // LAB_1040_6e48:
            u_var2 = (param_1 + 1);
            destroy_win_1010_3202(u_var2, (u_var2 >> 0x10), u_var3)
        }
        0x10a => {
            GetClientRect16(CONCAT22(unaff_ss, &local_a), &param_1.field_0x6);
            u_var2 = (param_1 + 1);
            local_8 = local_8 + 3;
            local_a = (u_var2 + 0x1a) - 9;
            local_6 = local_6 - 3;
            local_4 = local_4 - 3;
            InvalidateRect16(1, &local_a, unaff_ss);
            ui1::destroy_win_1010_2fa0((param_1 + 1));
            pass1_1010_32c0((param_1 + 1), 0);
            u_var2 = (param_1 + 1);
            pass1_1010_2ee2(u_var2, (u_var2 >> 0x10))
        }
        0x10c => {
            DestroyWindow16(&param_1.field_0x6);
        }
    }
    return;
}

pub fn create_win_1040_6942(param_1: *mut Struct20) {
    let mut u_var1: u32;
    let mut u_var2: u32;
    let ppc_var3: fn();
    let mut u_var4: i32;
    let h_wnd: *mut Struct38;
    let mut i_var5: i32;
    let struct_a: *mut Struct199;

    let struct_a_00: *mut Struct199;
    let struct_a_01: *mut Struct199;
    let ctx.dx_reg: *mut Struct199;
    let paVar6: *mut Struct199;
    let ctx.dx_reg: *mut Struct199;

    let struct_a_02: *mut Struct199;

    let mut i_var7: i32;
    let mut u_var8: u16;
    let mut u_var9: u16;
    let mut unaff_ss: u16;
    let ppVar10: *mut pass1_struct_1;
    let dVar11: u32;
    let u_var12: u8;
    let u_var13: u8;
    let u_var14: u8;
    let u_var15: u8;
    let u_var16: u8;
    let u_var17: u8;
    let mut hdc: u16;
    let mut in_stack_0000ff8a: u16;
    let mut local_6e: u16;
    let mut local_6c: u16;
    let mut local_6a: u32;
    let mut local_66: u16;
    let mut local_64: u32;
    let mut local_60: u32;
    let mut local_5c: u16;
    let mut local_5a: u16;
    let mut local_58: [u8; 80];
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    ui1::win_gui_func_1040_78e2(param_1);
    ppVar10 = process_struct_1010_20ba(
        ctx._g_Struct372_1050_0ed0,
        CONCAT22(in_stack_0000ff8a, 0x33),
    );
    u_var4 = ppVar10;
    u_var8 = (param_1 >> 0x10);
    i_var7 = param_1;
    (i_var7 + 0x94) = u_var4;
    (i_var7 + 0x96) = (ppVar10 >> 0x10);
    u_var9 = (i_var7 + 0x94);
    ppc_var3 = ((i_var7 + 0x94) + 4);
    (**ppc_var3)(0x10, u_var9, (ppVar10 >> 0x10), 0, i_var7, u_var8);
    paVar6 = struct_a;
    process_struct_1000_179c(10, struct_a);
    if ((paVar6 | u_var4) == 0) {
        (i_var7 + 0x98) = 0;
    } else {
        process_struct_1040_bf3e(
            CONCAT13((paVar6 >> 8), CONCAT12(paVar6, u_var4)),
            (i_var7 + 6),
        );
        (i_var7 + 0x98) = u_var4;
        (i_var7 + 0x9a) = ctx.dx_reg;
    }
    pass1_1040_bfde((i_var7 + 0x98), (i_var7 + 0x94));
    paVar6 = struct_a_00;
    process_struct_1000_179c(0x42, struct_a_00);
    struct_a_01 = (paVar6 | u_var4);
    if (struct_a_01 != 0x0) {
        ui1::win_fn_1008_3bd6(
            u_var4,
            paVar6,
            1,
            0xa000a,
            0x810000,
            0x10a0080,
            CONCAT22(u_var9, (i_var7 + 6)),
        );
        struct_a_01 = ctx.dx_reg;
    }
    process_struct_1000_179c(0x42, struct_a_01);
    paVar6 = (struct_a_01 | u_var4);
    if (paVar6 != 0x0) {
        ui1::win_fn_1008_3bd6(
            u_var4,
            struct_a_01,
            1,
            0xa0028,
            0x830000,
            0x10c0082,
            CONCAT22(u_var9, (i_var7 + 6)),
        );
        paVar6 = ctx.dx_reg;
    }
    u_var14 = 0;
    u_var16 = 0;
    process_struct_1000_179c(0x42, paVar6);
    if ((paVar6 | u_var4) == 0) {
        u_var4 = 0;
        local_4 = 0;
    } else {
        ui1::win_fn_1008_3bd6(
            u_var4,
            paVar6,
            1,
            0xa00aa,
            0x1000101,
            0x10700ff,
            CONCAT13(u_var16, CONCAT12(u_var14, (i_var7 + 6))),
        );
        local_4 = ctx.dx_reg;
    }
    local_6 = u_var4;
    ui1::enable_window_1040_9234(u_var4, local_4, u_var14);
    u_var14 = 0;
    u_var16 = 0;
    paVar6 = struct_a_02;
    process_struct_1000_179c(0x42, struct_a_02);
    if ((paVar6 | u_var4) == 0) {
        u_var4 = 0;
        local_4 = 0;
    } else {
        ui1::win_fn_1008_3bd6(
            u_var4,
            paVar6,
            1,
            0xa00c2,
            0x1000101,
            0x10800ff,
            CONCAT13(u_var16, CONCAT12(u_var14, (i_var7 + 6))),
        );
        local_4 = ctx.dx_reg;
    }
    local_6 = u_var4;
    ui1::enable_window_1040_9234(u_var4, local_4, u_var14);
    local_8 = GetDC16((i_var7 + 6));
    u_var15 = unaff_ss;
    u_var17 = (unaff_ss >> 8);
    load_string_1010_84e0(
        ctx._g_struct_73_1050_14cc,
        (ctx._g_struct_73_1050_14cc >> 0x10),
        0x50,
        CONCAT13(u_var17, CONCAT12(u_var15, local_58)),
        0x7cd,
    );
    u_var12 = SUB21(local_58, 0);
    u_var13 = (local_58 >> 8);
    u_var14 = u_var15;
    u_var16 = u_var17;
    hdc = local_8;
    u_var4 = get_string_index_1000_3da4(CONCAT22(unaff_ss, local_58));
    dVar11 = GetTextExtent16(
        u_var4,
        CONCAT13(u_var16, CONCAT12(u_var14, CONCAT11(u_var13, u_var12))),
        hdc,
    );
    local_5a = (dVar11 >> 0x10);
    local_5c = dVar11;
    CreateWindow16(
        0,
        ctx.g_h_instance_1050_038c,
        0x7cd,
        (i_var7 + 6),
        local_5a,
        local_5c,
        0xad,
        0x22,
        0x50000000,
        CONCAT13(u_var17, CONCAT12(u_var15, local_58)),
        s_static_1050_5d84,
    );
    load_string_1010_84e0(
        ctx._g_struct_73_1050_14cc,
        (ctx._g_struct_73_1050_14cc >> 0x10),
        0x50,
        CONCAT22(unaff_ss, local_58),
        0x7ce,
    );
    u_var12 = local_8;
    u_var13 = (local_8 >> 8);
    u_var14 = SUB21(local_58, 0);
    u_var16 = (local_58 >> 8);
    u_var4 = get_string_index_1000_3da4(CONCAT13(u_var17, CONCAT12(u_var15, local_58)));
    dVar11 = GetTextExtent16(
        u_var4,
        CONCAT22(unaff_ss, CONCAT11(u_var16, u_var14)),
        CONCAT11(u_var13, u_var12),
    );
    local_5a = (dVar11 >> 0x10);
    local_5c = dVar11;
    ReleaseDC16(local_8, (i_var7 + 6));
    CreateWindow16(
        0,
        ctx.g_h_instance_1050_038c,
        0x7ce,
        (i_var7 + 6),
        local_5a,
        local_5c,
        0xc5,
        0x22,
        0x50000000,
        CONCAT13(u_var17, CONCAT12(u_var15, local_58)),
        s_static_1050_5d8b,
    );
    local_64 = 0x5a000a;
    local_60 = 0x140050;
    h_wnd = &local_64;
    create_win_1040_6eae(param_1, 1, h_wnd, 0x5eb, 0xfd);
    local_64 = local_64 & 0xffff | (local_64._2_2_ + 0x14) << 0x10;
    create_win_1040_6eae(param_1, 0, &local_64, 0x5ec, 0xfe);
    local_64 = local_64 & 0xffff | (local_64._2_2_ + 0x14) << 0x10;
    create_win_1040_6eae(param_1, 0, &local_64, 0x5ee, 0xff);
    SendMessage16(0, 1, 0x401, h_wnd);
    u_var1 = (i_var7 + 0x94);
    i_var5 = u_var1;
    u_var1 = u_var1 & 0xffff0000;
    u_var9 = (u_var1 >> 0x10);
    SetWindowPos16(
        0x40,
        (i_var5 + 0x10),
        (i_var5 + 0xe),
        (i_var5 + 0xc),
        (u_var1 | i_var5 + 10),
        0,
        (i_var7 + 6),
    );
    u16_1050_0ecc = 0;
    u_var2 = (i_var7 + 0x94);
    ppc_var3 = ((i_var7 + 0x94) + 0x10);
    (**ppc_var3)(offset, u_var2, (u_var2 >> 0x10));
    u_var2 = (i_var7 + 0x94);
    pass1_1010_2ee2(u_var2, (u_var2 >> 0x10));
    PostMessage16(0, 0x10a, 0x111, (i_var7 + 6));
    return;
}

pub fn enable_window_1040_6880(param_1: u32, param_2: i32) {
    let mut u_var1: u32;
    let HVar2: HWND16;
    let mut i_var3: i32;
    let mut u_var4: u16;

    if (param_2 == 8) {
        u_var4 = (param_1 >> 0x10);
        i_var3 = param_1;
        HVar2 = GetDlgItem16(0x107, (i_var3 + 6));
        u_var1 = (i_var3 + 0x94);
        EnableWindow16((u_var1 + 0x24), HVar2);
        HVar2 = GetDlgItem16(0x108, (i_var3 + 6));
        u_var1 = (i_var3 + 0x94);
        EnableWindow16((u_var1 + 0x26), HVar2);
    }
    return;
}

pub fn msg_box_1040_64ca(param_1: u32) {

    let in_dx: *mut Struct199;
    let mut unaff_ss: u16;
    let mut local_212: u32;
    let mut local_20e: u32;
    let mut local_20a: u16;
    let mut local_208: u16;
    let mut local_206: [u8; 258];
    let mut local_104: [u8; 258];

    process_struct_1000_179c(0x1000, in_dx);
    load_string_1010_84e0(
        ctx._g_struct_73_1050_14cc,
        (ctx._g_struct_73_1050_14cc >> 0x10),
        0x100,
        CONCAT22(unaff_ss, local_206),
        0x57b,
    );
    load_string_1010_84e0(
        ctx._g_struct_73_1050_14cc,
        (ctx._g_struct_73_1050_14cc >> 0x10),
        0x3ff,
        CONCAT22(in_dx, in_ax),
        0x7ff,
    );
    load_string_1010_84e0(
        ctx._g_struct_73_1050_14cc,
        (ctx._g_struct_73_1050_14cc >> 0x10),
        0x3ff,
        CONCAT22(unaff_ss, local_104),
        0x800,
    );
    process_string_1000_3cea(CONCAT22(in_dx, in_ax), CONCAT22(unaff_ss, local_104));
    load_string_1010_84e0(
        ctx._g_struct_73_1050_14cc,
        (ctx._g_struct_73_1050_14cc >> 0x10),
        0x3ff,
        CONCAT22(unaff_ss, local_104),
        0x801,
    );
    process_string_1000_3cea(CONCAT22(in_dx, in_ax), CONCAT22(unaff_ss, local_104));
    load_string_1010_84e0(
        ctx._g_struct_73_1050_14cc,
        (ctx._g_struct_73_1050_14cc >> 0x10),
        0x3ff,
        CONCAT22(unaff_ss, local_104),
        0x802,
    );
    process_string_1000_3cea(CONCAT22(in_dx, in_ax), CONCAT22(unaff_ss, local_104));
    MessageBox16(
        0,
        CONCAT22(unaff_ss, local_206),
        CONCAT22(in_dx, in_ax),
        (param_1 + 6),
    );
    error_check_1000_17ce(CONCAT22(in_dx, in_ax));
    return;
}

pub fn win_fn_1040_65ba(param_1: *mut Struct20) {
    let mut u_var1: u32;
    let mut u_var2: u16;
    let pu_var3: *mut u16;
    let struct_a: *mut Struct199;
    let mut i_var4: i32;
    let mut i_var5: i32;
    let mut unaff_si: u16;
    let mut u_var6: u16;
    let unaff_ss: HWND16;
    let mut u_var7: u32;
    let mut local_26: u16;
    let mut local_24: u16;
    let mut local_22: u16;
    let mut local_20: u16;
    let mut local_1e: u16;
    let mut local_1c: u16;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    ui1::win_gui_func_1040_78e2(param_1);
    _local_6 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(unaff_si, 0x2b));
    ctx.g_u16_ptr_1050_5f2e = (_local_6 >> 0x10);
    local_8 = return3_1010_0898();
    if (ctx.__g_Struct94_ptr_1 == 0) {
        _g_Struct94_ptr_1 = struct_fn_1000_160a();
    } else {
    }
    _local_1a = CONCAT22(ctx.g_u16_ptr_1050_5f2e, _g_Struct94_ptr_1);
    u_var2 = alloc_mem_1000_1708(
        (local_8 + 2) * 4,
        0,
        1,
        _g_Struct94_ptr_1,
        ctx.g_u16_ptr_1050_5f2e,
    );
    u_var6 = (param_1 >> 0x10);
    i_var4 = param_1;
    (i_var4 + 0x8e) = u_var2;
    (i_var4 + 0x90) = ctx.g_u16_ptr_1050_5f2e;
    local_a = 1;
    while (local_a <= local_8) {
        _local_1a = pass1_1010_0946(_local_6, (_local_6 >> 0x10), local_a);
        struct_a = (_local_1a >> 0x10);
        local_22 = *_local_1a;
        local_20 = (_local_1a + 2);
        local_1e = 1;
        local_1c = 1;
        u_var2 = (i_var4 + 6);
        pu_var3 = &local_22;
        MapDialogRect16(CONCAT22(pu_var3, 0x1010), unaff_ss);
        process_struct_1000_179c(0x42, struct_a);
        if ((struct_a | pu_var3) == 0) {
            u_var1 = (i_var4 + 0x8e);
            (u_var1 + local_a * 4) = 0;
        } else {
            u_var7 = ui1::win_fn_1008_3bd6(
                pu_var3,
                struct_a,
                0,
                CONCAT22(local_22, local_20),
                0x1000101,
                CONCAT22((_local_1a + 4), 0xff),
                CONCAT22(u_var2, (i_var4 + 6)),
            );
            u_var1 = (i_var4 + 0x8e);
            u_var2 = (u_var1 >> 0x10);
            i_var5 = u_var1;
            (i_var5 + local_a * 4) = u_var7;
            (i_var5 + local_a * 4 + 2) = (u_var7 >> 0x10);
        }
        u_var1 = (i_var4 + 0x8e);
        u_var2 = (u_var1 >> 0x10);
        i_var5 = u_var1;
        if ((i_var5 + local_a * 4) != 0) {
            u_var1 = (i_var5 + local_a * 4);
            ui1::enable_window_1040_9234(u_var1, (u_var1 >> 0x10), (_local_1a + 6));
        }
        local_a = local_a + 1;
    }
    ui1::move_window_1040_826c(param_1, u_var6, 0xff, 0xffff);
    ShowWindow16(5, (i_var4 + 6));
    return;
}

pub fn win_fn_1040_5800(param_1: *mut Struct124, param_2: u16, param_3: u16, param_4: u32) {
    let pp_var1: fn();
    let mut u_var2: u32;
    let mut u_var3: i32;
    let pu_var4: Vec<u8>;
    let h_wnd: HWND16;
    let mut u_var5: u32;

    let paVar6: *mut Struct199;
    let mut i_var7: i32;
    let mut unaff_si: u16;
    let mut u_var8: u16;
    let unaff_ss: HWND16;
    let mut local_24: u16;
    let mut local_1c: u32;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: [u8; 8];
    let mut local_c: u16;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    if (param_2._2_2_ == 0xeb) {
        _local_6 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(unaff_si, 3));
        paVar6 = (_local_6 >> 0x10);
        u_var5 = &param_1.field_0x90;
        if (u_var5 != 0) {
            local_a = u_var5;
            process_struct_1000_179c(0x18, paVar6);
            u_var3 = u_var5;
            if ((paVar6 | u_var3) == 0) {
                u_var3 = 0;
                paVar6 = 0x0;
            } else {
                process_struct_1040_a598((u_var5 & 0xffff | ZEXT24(paVar6) << 0x10));
                paVar6 = ctx.dx_reg;
            }
            param_1.field_0x90 = u_var3;
            &param_1.field_0x92 = paVar6;
            *&param_1.field_0x90 = 6;
            local_c = *&param_1.field_0x90;
            u_var3 = local_c * 10 + 2;
            process_struct_1000_179c(u_var3, paVar6);
            _local_18 = CONCAT22(paVar6, u_var3);
            if ((paVar6 | u_var3) == 0) {
                u_var2 = &param_1.field_0x90;
                (u_var2 + 2) = 0;
            } else {
                *_local_18 = local_c;
                call_fn_ptr_1000_5586(
                    0xa564,
                    &ctx.PTR_LOOP_1050_1040,
                    local_c,
                    10,
                    u_var3 + 2,
                    paVar6,
                );
                u_var2 = &param_1.field_0x90;
                u_var8 = (u_var2 >> 0x10);
                i_var7 = u_var2;
                (i_var7 + 2) = u_var3 + 2;
                (i_var7 + 4) = paVar6;
            }
            u_var2 = &param_1.field_0x90;
            (u_var2 + 6) = (local_a + 6);
            u_var2 = &param_1.field_0x90;
            (u_var2 + 10) = 4;
            u_var2 = &param_1.field_0x90;
            (u_var2 + 0x12) = &param_1.field_0xa;
            u_var8 = 0x1010;
            pass1_1010_a50c(_local_6, 0x10505d78, &param_1.field_0x90);
            if (local_a != 0) {
                pass1_1040_a5d0(local_a);
                u_var8 = 0x1000;
                error_check_1000_17ce(local_a);
            }
            pp_var1 = (CONCAT22(param_2_00, param_1) + 0x70);
            (**pp_var1)(u_var8, param_1, param_2_00);
            pu_var4 = pass1_1040_5cd6(CONCAT22(param_2_00, param_1));
            if (pu_var4 != 0x0) {
                pass1_1040_5eaa(CONCAT22(param_2_00, param_1));
                (param_1 + 1) = 0;
            }
            pass1_1040_5dc4(CONCAT22(param_2_00, param_1));
            GetWindowRect16(CONCAT22(local_14, u_var8), unaff_ss);
            InvalidateRect16(&param_1[1].field_0x8, 0x0, 0);
            if (&param_1[1].field_0x8 != 0) {
                &param_1[1].field_0x8 = 0;
            }
        }
    } else {
        if (param_2._2_2_ != 0x13b) {
            ui1::win_gui_fn_1040_b54a(param_1, param_2_00, param_3, param_2);
            return;
        }
        h_wnd = GetDlgItem16(0x1790, &param_1.field_0x6);
        EnableWindow16(1, h_wnd);
    }
    return;
}

pub fn enable_window_1040_5780(param_1: *mut u32) {
    let pp_var1: fn();
    let mut u_var2: u32;
    let mut u_var3: u16;
    let h_wnd: HWND16;
    let mut i_var4: i32;
    let mut u_var5: u16;
    let ppVar6: *mut pass1_struct_1;
    let mut i_var7: i32;

    u_var5 = (param_1 >> 0x10);
    i_var4 = param_1;
    unsafe {
        pp_var1 = (*param_1 + 0x74);
    }
    i_var7 = i_var4;
    (**pp_var1)();
    ppVar6 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(i_var7, 3));
    u_var2 = (i_var4 + 0x90);
    u_var3 = pass1_1010_acc0(ppVar6, (ppVar6 >> 0x10), (u_var2 + 6));
    if (u_var3 != 0) {
        h_wnd = GetDlgItem16(0x1790, (i_var4 + 6));
        EnableWindow16(1, h_wnd);
    }
    return;
}

pub fn pass1_1040_57d4(param_1: Vec<u8>) {
    pass1_1040_5d42(param_1);
    pass1_1040_5eaa(param_1);
    pass1_1040_5dc4(param_1);
    ui1::set_window_pos_1040_b230(param_1);
    return;
}

pub fn set_window_pos_1040_4f96(param_1: *mut Struct20) {
    let mut u_var1: u32;
    let ppc_var2: fn();
    let mut u_var3: u16;
    let mut u_var4: i32;
    let mut u_var5: u16;
    let mut u_var6: u16;
    let mut i_var7: i32;
    let extraout_DL: u8;
    let u_var8: u8;
    let mut u_var9: u16;
    let struct_a: *mut Struct199;

    let struct_a_00: *mut Struct199;
    let pa_var10: *mut Struct199;
    let ctx.dx_reg: *mut Struct199;
    let pa_var11: *mut Struct199;
    let ctx.dx_reg: *mut Struct199;
    let ctx.dx_reg: *mut Struct199;
    let ctx.dx_reg: *mut Struct199;
    let extraout_dx_04: *mut Struct199;
    let extraout_dx_05: *mut Struct199;
    let mut u_var12: u16;
    let mut i_var13: i32;
    let mut u_var14: u16;
    let mut u_var15: u16;
    let ppVar16: *mut pass1_struct_1;
    let mut in_stack_0000ffe4: u16;
    let mut local_12: u16;
    let mut local_10: u32;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    ui1::win_gui_func_1040_78e2(param_1);
    ppVar16 = process_struct_1010_20ba(
        ctx._g_Struct372_1050_0ed0,
        CONCAT22(in_stack_0000ffe4, 0x41),
    );
    u_var9 = (ppVar16 >> 0x10);
    u_var4 = ppVar16;
    u_var14 = (param_1 >> 0x10);
    i_var13 = param_1;
    (i_var13 + 0x98) = u_var4;
    (i_var13 + 0x9a) = u_var9;
    u_var15 = (i_var13 + 0x98);
    ppc_var2 = ((i_var13 + 0x98) + 0x10);
    ppc_var2(0x1010, u_var15, u_var9);
    pa_var11 = struct_a;
    process_struct_1000_179c(10, struct_a);
    if ((pa_var11 | u_var4) == 0) {
        (i_var13 + 0x94) = 0;
    } else {
        process_struct_1040_bf3e(
            CONCAT13((pa_var11 >> 8), CONCAT12(pa_var11, u_var4)),
            (i_var13 + 6),
        );
        (i_var13 + 0x94) = u_var4;
        (i_var13 + 0x96) = ctx.dx_reg;
    }
    pass1_1040_bfde((i_var13 + 0x94), (i_var13 + 0x98));
    pa_var11 = struct_a_00;
    process_struct_1000_179c(0x42, struct_a_00);
    pa_var10 = (pa_var11 | u_var4);
    if (pa_var10 != 0x0) {
        ui1::win_fn_1008_3bd6(
            u_var4,
            pa_var11,
            1,
            0xa000a,
            0x810000,
            0x10a0080,
            CONCAT22(u_var15, (i_var13 + 6)),
        );
        pa_var10 = ctx.dx_reg;
    }
    process_struct_1000_179c(0x42, pa_var10);
    pa_var11 = (pa_var10 | u_var4);
    if (pa_var11 != 0x0) {
        ui1::win_fn_1008_3bd6(
            u_var4,
            pa_var10,
            1,
            0xa0028,
            0x850000,
            0x10b0084,
            CONCAT22(u_var15, (i_var13 + 6)),
        );
        pa_var11 = ctx.dx_reg;
    }
    process_struct_1000_179c(0x42, pa_var11);
    pa_var10 = (pa_var11 | u_var4);
    if (pa_var10 != 0x0) {
        ui1::win_fn_1008_3bd6(
            u_var4,
            pa_var11,
            1,
            0xa0046,
            0x870000,
            0x10d0086,
            CONCAT22(u_var15, (i_var13 + 6)),
        );
        pa_var10 = ctx.dx_reg;
    }
    process_struct_1000_179c(0x42, pa_var10);
    pa_var11 = (pa_var10 | u_var4);
    if (pa_var11 != 0x0) {
        ui1::win_fn_1008_3bd6(
            u_var4,
            pa_var10,
            1,
            0xa0064,
            0x890000,
            0x10e0088,
            CONCAT22(u_var15, (i_var13 + 6)),
        );
        pa_var11 = ctx.dx_reg;
    }
    process_struct_1000_179c(0x42, pa_var11);
    pa_var10 = (pa_var11 | u_var4);
    if (pa_var10 != 0x0) {
        ui1::win_fn_1008_3bd6(
            u_var4,
            pa_var11,
            1,
            0xa0082,
            0x830000,
            0x10c0082,
            CONCAT22(u_var15, (i_var13 + 6)),
        );
        pa_var10 = extraout_dx_04;
    }
    process_struct_1000_179c(0x42, pa_var10);
    pa_var11 = (pa_var10 | u_var4);
    if (pa_var11 != 0x0) {
        ui1::win_fn_1008_3bd6(
            u_var4,
            pa_var10,
            1,
            0xa00d2,
            0x8b0000,
            0xbbb008a,
            CONCAT22(u_var15, (i_var13 + 6)),
        );
        pa_var11 = extraout_dx_05;
    }
    u_var9 = 0;
    process_struct_1000_179c(0x42, pa_var11);
    if ((pa_var11 | u_var4) == 0) {
        u_var4 = 0;
        u_var8 = 0;
    } else {
        ui1::win_fn_1008_3bd6(
            u_var4,
            pa_var11,
            1,
            0xa00a0,
            0x8d008e,
            0xbbc008c,
            CONCAT22(u_var9, (i_var13 + 6)),
        );
        u_var8 = extraout_DL;
    }
    ui1::enable_window_1040_9234(u_var4, u_var8, u_var9);
    ppVar16 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(u_var15, 3));
    u_var12 = (ppVar16 >> 0x10);
    u_var3 = ppVar16;
    u_var5 = pass1_1010_a5ac(u_var3, u_var12, (i_var13 + 0xb0));
    u_var6 = pass1_1010_ac62(u_var3, u_var12, 0x1e);
    if (u_var6 != 0) {
        pass1_1010_a5ca(u_var3, u_var12, u_var5);
        if (0 < u_var6) {
            pass1_1010_a58a(u_var3, u_var12, u_var5);
            if (u_var6 == 0) {
                ui1::enable_window_1040_9234(u_var4, u_var8, 1);
            }
        }
    }
    u_var1 = (i_var13 + 0x98);
    i_var7 = u_var1;
    u_var1 = u_var1 & 0xffff0000;
    u_var15 = (u_var1 >> 0x10);
    SetWindowPos16(
        0x40,
        (i_var7 + 0x10),
        (i_var7 + 0xe),
        (i_var7 + 0xc),
        (u_var1 | i_var7 + 10),
        0,
        (i_var13 + 6),
    );
    return;
}

pub fn pass1_1040_5238(param_1: Vec<u8>) -> Vec<u8> {
    let pp_var1: fn();

    pp_var1 = ((param_1 + 0x94) + 8);
    (**pp_var1)();
    return 0x0;
}

pub fn send_win_msg_1040_4a0a(param_1: *mut u32) -> LRESULT {
    let pu_var1: *mut u16;
    let ppc_var2: fn();
    let mut u_var3: u32;
    let mut u_var4: u32;
    let h_wnd: HWND16;
    let mut u_var5: u16;
    let w_var6: WPARAM;
    let mut u_var7: u16;
    let mut i_var8: i32;
    let mut u_var9: u16;
    let LVar10: LRESULT;
    let l_param: LPARAM;
    let mut u_var11: u16;
    let h_var12: HWND16;
    let mut local_a: u16;
    let mut local_4: u16;

    u_var9 = (param_1 >> 0x10);
    i_var8 = param_1;
    unsafe {
        ppc_var2 = (*param_1 + 0x74);
    }
    ppc_var2();
    h_wnd = GetDlgItem16(6000, (i_var8 + 6));
    SendMessage16(0, 0, 0x40b, h_wnd);
    LVar10 = SendMessage16(0, 0, 0xb, h_wnd);
    u_var7 = (LVar10 >> 0x10);
    local_a = 0;
    while (
        u_var3 = (i_var8 + 0x90),
        pu_var1 = (u_var3 + 0x10),
        unsafe { *pu_var1 != local_a } && unsafe { local_a <= *pu_var1 },
    ) {
        w_var6 = 0;
        u_var11 = 0x403;
        u_var3 = (i_var8 + 0x90);
        u_var3 = (u_var3 + 0xc);
        u_var5 = local_a;
        h_var12 = h_wnd;
        pass1_1040_4dcc(param_1, *(u_var3 + local_a * 2));
        LVar10 = SendMessage16(
            CONCAT13((u_var7 >> 8), CONCAT12(u_var7, u_var5)),
            w_var6,
            u_var11,
            h_var12,
        );
        u_var7 = (LVar10 >> 0x10);
        local_a = local_a + 1;
    }
    w_var6 = pass1_1040_4d7e(param_1);
    if (w_var6 == 0) {
        u_var11 = 0x40a;
        u_var4 = (i_var8 + 0x90);
        u_var3 = (i_var8 + 0x94);
        h_var12 = h_wnd;
        l_param = pass1_1010_ada6(u_var3, (u_var3 >> 0x10), 0, (u_var4 + 10));
        SendMessage16(l_param, w_var6, u_var11, h_var12);
    }
    LVar10 = SendMessage16(0, 1, 0xb, h_wnd);
    return LVar10;
}

pub fn set_win_pos_1040_4ae4(param_1: *mut Struct124, param_2: u16, param_3: u16, param_4: u32) {
    let pp_var1: fn();
    let mut u_var2: u32;
    let mut u_var3: i32;
    let mut u_var4: u32;

    let pa_var5: *mut Struct199;
    let mut i32_var6: i32;
    let mut unaff_si: u16;
    let mut u_var7: u16;
    let mut unaff_cs: u16;
    let unaff_ss: HWND16;
    let mut local_24: u16;
    let mut local_20: u16;
    let mut local_14: u32;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    if (param_3._2_2_ == 0xeb) {
        _local_6 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(unaff_si, 3));
        pa_var5 = (_local_6 >> 0x10);
        u_var4 = &param_1.field_0x90;
        if (u_var4 != 0) {
            local_a = u_var4;
            process_struct_1000_179c(0x18, pa_var5);
            u_var3 = u_var4;
            _local_10 = (u_var4 & 0xffff | ZEXT24(pa_var5) << 0x10);
            if ((pa_var5 | u_var3) == 0) {
                u_var3 = 0;
                pa_var5 = 0x0;
            } else {
                process_struct_1040_a598((u_var4 & 0xffff | ZEXT24(pa_var5) << 0x10));
                pa_var5 = ctx.dx_reg;
            }
            param_1.field_0x90 = u_var3;
            &param_1.field_0x92 = pa_var5;
            *&param_1.field_0x90 = 7;
            local_c = *&param_1.field_0x90;
            u_var3 = local_c * 10 + 2;
            process_struct_1000_179c(u_var3, pa_var5);
            _local_10 = CONCAT22(pa_var5, u_var3);
            if ((pa_var5 | u_var3) == 0) {
                u_var2 = &param_1.field_0x90;
                (u_var2 + 2) = 0;
            } else {
                *_local_10 = local_c;
                call_fn_ptr_1000_5586(
                    0xa564,
                    &ctx.PTR_LOOP_1050_1040,
                    local_c,
                    10,
                    u_var3 + 2,
                    pa_var5,
                );
                u_var2 = &param_1.field_0x90;
                u_var7 = (u_var2 >> 0x10);
                i32_var6 = u_var2;
                (i32_var6 + 2) = u_var3 + 2;
                (i32_var6 + 4) = pa_var5;
            }
            u_var7 = (local_a >> 0x10);
            i32_var6 = local_a;
            u_var2 = &param_1.field_0x90;
            (u_var2 + 6) = (i32_var6 + 6);
            u_var2 = &param_1.field_0x90;
            (u_var2 + 10) = (i32_var6 + 10);
            u_var2 = &param_1.field_0x90;
            (u_var2 + 0x12) = (i32_var6 + 0x12);
            u_var7 = 0x1010;
            pass1_1010_a50c(_local_6, 0x10505d6a, &param_1.field_0x90);
            local_14 = local_a;
            _local_10 = local_a;
            if (local_a != 0) {
                pass1_1040_a5d0(local_a);
                u_var7 = 0x1000;
                error_check_1000_17ce(local_a);
            }
            pp_var1 = (CONCAT22(param_2, param_1) + 0x70);
            (**pp_var1)(u_var7, param_1, param_2);
        }
    } else {
        if (param_3._2_2_ != 6000) {
            ui1::win_gui_fn_1040_b54a(param_1, param_2, param_3_00, param_3);
            return;
        }
        if (param_3 == 7) {
            GetWindowRect16(CONCAT22(&local_24, unaff_cs), unaff_ss);
            local_20 = local_20 - local_24;
            SetWindowPos16(2, 0x50, local_20, 0, 0, 0, param_3_00);
        }
    }
    return;
}

pub fn send_win_fn_1040_4cb2(param_1: Vec<u8>) -> LRESULT {
    let h_wnd: HWND16;
    let l_param: LPARAM;
    let LVar1: LRESULT;
    let w_param: WPARAM16;
    let mut msg: u16;
    let mut u_var2: u16;

    u_var2 = (param_1 >> 0x10);
    ui1::set_dlg_item_txt_1040_b45e(param_1, u_var2);
    h_wnd = GetDlgItem16(6000, (param_1 + 6));
    w_param = 0xffff;
    msg = 0x40d;
    l_param = pass1_1040_4d7e(param_1);
    pass1_1040_4dcc(param_1, l_param);
    LVar1 = SendMessage16(l_param, w_param, msg, h_wnd);
    return LVar1;
}

pub fn win_fn_1040_477e(param_1: u32) {
    let u_var1: u8;
    let mut u_var2: u16;
    let extraout_var: u32;
    let mut u_var4: i32;
    let mut u_var5: i32;

    let ppVar6: *mut pass1_struct_1;
    let mut u_var7: u16;
    let mut u_var8: u16;
    let mut in_stack_0000ffee: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;
    let mut u_var3: u32;

    ui1::set_window_pos_1040_b230(param_1, (param_1 >> 0x10));
    ppVar6 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(in_stack_0000ffee, 3));
    u_var4 = (ppVar6 >> 0x10);
    u_var8 = SUB42(&ctx.g_alloc_addr_1050_1050, 0);
    u_var7 = 0x5d68;
    u_var5 = u_var4;
    u_var1 = string_fn_1008_5fd8(0x7d3, -0x15);
    u_var3 = CONCAT31(extraout_var, u_var1);
    u_var2 = u_var3;
    process_string_1000_3cea((u_var3 & 0xffff | u_var5 << 0x10), CONCAT22(u_var8, u_var7));
    pass1_1010_e964(ppVar6, u_var4);
    process_string_1000_3cea(
        (u_var3 & 0xffff | u_var5 << 0x10),
        CONCAT22(ctx.dx_reg, u_var2),
    );
    copy_string_1000_3d3e(
        (param_1 & 0xffff0000 | (param_1 + 0x10)),
        (u_var3 & 0xffff | u_var5 << 0x10),
    );
    error_check_1000_17ce((u_var3 & 0xffff | u_var5 << 0x10));
    return;
}

pub fn win_gui_fn_1040_45e8(param_1: *mut Struct124, param_2: u16, param_3: u16, param_4: u32) {
    let in_struct_1: &mut Struct44;
    let pp_var1: fn();
    let mut u_var2: u32;
    let mut u_var3: i32;
    let paVar4: &mut Struct44;

    let pa_var5: *mut Struct199;
    let mut i32_var6: i32;
    let mut unaff_si: u16;
    let mut u_var7: u16;
    let pp_var8: *mut pass1_struct_1;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    if (param_2._2_2_ != 0xeb) {
        ui1::win_gui_fn_1040_b54a(param_1, param_2_00, param_3, param_2);
        return;
    }
    pp_var8 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(unaff_si, 3));
    pa_var5 = (pp_var8 >> 0x10);
    in_struct_1 = &param_1.field_0x90;
    if (in_struct_1 != 0x0) {
        paVar4 = in_struct_1;
        process_struct_1000_179c(0x18, pa_var5);
        u_var3 = paVar4;
        if ((pa_var5 | u_var3) == 0) {
            u_var3 = 0;
            pa_var5 = 0x0;
        } else {
            process_struct_1040_a598((paVar4 & 0xffff | ZEXT24(pa_var5) << 0x10));
            pa_var5 = ctx.dx_reg;
        }
        param_1.field_0x90 = u_var3;
        &param_1.field_0x92 = pa_var5;
        *&param_1.field_0x90 = 0x14;
        i32_var6 = &param_1.field_0x90;
        u_var3 = i32_var6 * 10 + 2;
        process_struct_1000_179c(u_var3, pa_var5);
        _local_10 = CONCAT22(pa_var5, u_var3);
        if ((pa_var5 | u_var3) == 0) {
            u_var2 = &param_1.field_0x90;
            (u_var2 + 2) = 0;
        } else {
            *_local_10 = i32_var6;
            call_fn_ptr_1000_5586(
                0xa564,
                &ctx.PTR_LOOP_1050_1040,
                i32_var6,
                10,
                u_var3 + 2,
                pa_var5,
            );
            u_var2 = &param_1.field_0x90;
            u_var7 = (u_var2 >> 0x10);
            i32_var6 = u_var2;
            (i32_var6 + 2) = u_var3 + 2;
            (i32_var6 + 4) = pa_var5;
        }
        u_var2 = &param_1.field_0x90;
        (u_var2 + 6) = (in_struct_1 + 6);
        u_var2 = &param_1.field_0x90;
        (u_var2 + 10) = 1;
        u_var2 = &param_1.field_0x90;
        (u_var2 + 0x12) = &param_1.field_0xa;
        u_var7 = 0x1010;
        pass1_1010_a50c(pp_var8, 0x10505d40, &param_1.field_0x90);
        if (in_struct_1 != 0x0) {
            pass1_1040_a5d0(in_struct_1);
            u_var7 = 0x1000;
            error_check_1000_17ce(in_struct_1);
        }
        pp_var1 = (CONCAT22(param_2_00, param_1) + 0x70);
        (**pp_var1)(u_var7, param_1, param_2_00);
    }
    return;
}

pub fn win_fn_1040_410e(param_1: *mut Struct20) {
    let mut u_var1: u32;
    let mut i_var2: i32;
    let mut u_var3: u16;
    let unaff_ss: String;
    let ppVar4: *mut pass1_struct_1;
    let pu_var5: *mut u16;
    let pu_var6: *mut u16;
    let pcVar7: String;
    let mut local_3e: u16;
    let mut local_3c: u16;
    let mut local_3a: u16;
    let mut local_38: u16;
    let mut local_36: u16;
    let mut local_34: u16;
    let mut local_32: u16;
    let mut local_30: [u8; 6];
    let mut local_2a: u16;
    let mut local_22: u16;
    let mut local_20: u16;
    let mut local_1e: u16;
    let mut local_1c: u16;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: [u8; 10];

    ui1::win_gui_func_1040_78e2(param_1);
    pass1_1000_4906(CONCAT22(unaff_ss, local_c), 0, 10);
    u_var3 = (param_1 >> 0x10);
    i_var2 = param_1;
    u_var1 = (i_var2 + 0x8e);
    string_fn_1000_3f9c(
        local_c,
        unaff_ss,
        0x5d38,
        &ctx.g_alloc_addr_1050_1050,
        (u_var1 + 0x76),
    );
    local_e = GetDlgItem16(0xfb5, (i_var2 + 6));
    SendMessage16(
        CONCAT13((unaff_ss >> 8), CONCAT12(unaff_ss, local_c)),
        0,
        0xc,
        local_e,
    );
    SetFocus16(local_e);
    SendMessage16(-0x10000, 0, 0x401, local_e);
    GetWindowRect16(CONCAT22(&local_16, 0x1538), unaff_ss);
    pass1_1000_4906(CONCAT22(unaff_ss, &local_1e), 0, 8);
    u_var1 = (i_var2 + 0x8e);
    _local_22 = pass1_1010_5f7a(u_var1, (u_var1 >> 0x10), 0, 7);
    if (_local_22 != 0x0) {
        _local_1e = *_local_22;
        _local_1a = (_local_22 + 4);
    }
    if ((local_1c == 0) && (local_1e == 0)) {
        zero_list_1008_3e38(CONCAT22(unaff_ss, local_30));
        u_var1 = (i_var2 + 0x96);
        pass1_1018_2678(u_var1, (u_var1 >> 0x10), CONCAT22(unaff_ss, local_30));
        pass1_1008_3e94(
            local_30,
            CONCAT22(unaff_ss, &local_32),
            CONCAT22(unaff_ss, &local_2a),
        );
        pu_var6 = &local_34;
        pu_var5 = &local_36;
        pcVar7 = unaff_ss;
        ppVar4 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(pu_var5, 0x48));
        pass1_1008_3e94(
            (ppVar4 + 0xe),
            CONCAT22(unaff_ss, pu_var5),
            CONCAT22(pcVar7, pu_var6),
        );
        _local_1a = CONCAT22(local_10 - local_14, local_12 - local_16);
        _local_1e = CONCAT22(
            (((ppVar4 + 0xc) * -0x14) / 600 - (local_10 - local_14)) + local_36 + local_32,
            local_34 + local_2a,
        );
    }
    ui1::move_window_1040_826c(i_var2, u_var3, local_1c, local_1e);
    ShowWindow16(5, (i_var2 + 6));
    return;
}

pub fn enable_win_1040_42b2(param_1: u32, param_2: i32) {
    let mut u_var1: u32;
    let HVar2: HWND16;
    let pu_var3: Vec<u8>;
    let mut u_var4: i32;
    let mut i_var5: i32;
    let mut u_var6: u16;
    let mut unaff_ss: u16;
    let LVar7: LRESULT;
    let mut local_66: u32;
    let mut local_5a: u16;
    let mut local_58: u16;
    let mut local_56: u16;
    let mut local_54: [u8; 82];

    i_var5 = param_1;
    u_var6 = (param_1 >> 0x10);
    if (param_2 == 0) {
        (i_var5 + 0x9a) = 1;
        DestroyWindow16((i_var5 + 6));
        return;
    }
    pass1_1000_4906(CONCAT22(unaff_ss, local_54), 0, 0x51);
    HVar2 = GetDlgItem16(0xfb5, (i_var5 + 6));
    LVar7 = SendMessage16(CONCAT22(unaff_ss, local_54), 0x51, 0xd, HVar2);
    u_var4 = (LVar7 >> 0x10);
    pu_var3 = local_54;
    pass1_fn_1000_3e2c(CONCAT22(unaff_ss, pu_var3));
    if ((u_var4 | pu_var3) != 0) {
        (i_var5 + 0x92) = pu_var3;
        (i_var5 + 0x94) = u_var4;
    }
    if (u_var4 < 0) {
        u_var1 = (i_var5 + 0x8e);
        wsprintf16(
            local_54,
            CONCAT22(0x5d3c, unaff_ss),
            CONCAT22((u_var1 + 0x76), 0x1050),
        );
        SendMessage16(CONCAT22(unaff_ss, local_54), 0, 0xc, HVar2);
        SetFocus16(HVar2);
        SendMessage16(-0x10000, 0, 0x401, HVar2);
        return;
    }
    HVar2 = GetDlgItem16(1, (i_var5 + 6));
    EnableWindow16(0, HVar2);
    u_var1 = (i_var5 + 0x8e);
    (u_var1 + 0x76) = (i_var5 + 0x92);
    PostMessage16((i_var5 + 0x92), 0, 0x400, ctx.g_h_window);
    HVar2 = GetDlgItem16(1, (i_var5 + 6));
    EnableWindow16(1, HVar2);
    return;
}

pub fn get_win_rect_1040_43ea(param_1: u32) {
    let mut u_var1: u32;
    let mut i_var2: i32;
    let mut u_var3: u16;
    let mut unaff_cs: u16;
    let unaff_ss: HWND16;
    let mut local_12: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    u_var3 = (param_1 >> 0x10);
    i_var2 = param_1;
    GetWindowRect16(CONCAT22(&local_a, unaff_cs), unaff_ss);
    local_6 = local_6 - local_a;
    local_4 = local_4 - local_8;
    pass1_1010_5fb0((i_var2 + 0x8e), 0, &local_a, unaff_ss, 7);
    u_var1 = (i_var2 + 0x8e);
    (u_var1 + 0x7a) = ((i_var2 + 0x9a) == 0);
    return;
}

pub fn send_dialog_item_msg_1040_3f12(param_1: u32, param_2: u32) {
    let pu_var1: Vec<u8>;

    let mut i_var2: i32;
    let mut i_var3: i32;
    let mut u_var4: u16;
    let mut unaff_ss: u16;
    let LVar5: LRESULT;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u32;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: [u8; 8];

    u_var4 = (param_1 >> 0x10);
    i_var3 = param_1;
    SendDlgItemMessage16(0, 0, 0xb, 400, (i_var3 + 6));
    SendDlgItemMessage16(0, 0, 0x405, 400, (i_var3 + 6));
    pass1_1008_5784(CONCAT22(unaff_ss, local_a), param_2);
    while (true) {
        pu_var1 = local_a;
        pass1_1008_5b12(CONCAT22(unaff_ss, pu_var1));
        if ((ctx.dx_reg | pu_var1) == 0) {
            break;
        }
        LVar5 = SendDlgItemMessage16((pu_var1 + 4), 0, 0x401, 400, (i_var3 + 6));
        i_var2 = (LVar5 >> 0x10);
        if (((LVar5 == -1) && (i_var2 == -1)) || (LVar5 == -2 && (i_var2 == -1))) {
            break;
        }
    }
    SendDlgItemMessage16(0, 0, 0x407, 400, (i_var3 + 6));
    SendDlgItemMessage16(0, 1, 0xb, 400, (i_var3 + 6));
    return;
}

pub fn win_gui_dialog_func_1040_3e08(param_1: *mut Struct25) {
    let mut u_var1: u16;
    let local_bx_4: *mut Struct25;
    let mut u_var2: u16;
    let LVar3: LRESULT;

    u_var2 = (param_1 >> 0x10);
    local_bx_4 = param_1;
    CheckRadioButton16(local_bx_4.check_id, 0x18d, 0x188, local_bx_4.h_wnd);
    local_bx_4.dlg_item_id = 0;
    local_bx_4.dlg_item_id_2 = 0;
    LVar3 = SendDlgItemMessage16(0, 0, 0x409, 400, local_bx_4.h_wnd);
    if (LVar3 != -1) {
        u_var1 = pass1_1018_3ab2(local_bx_4.field_0x8e, LVar3, local_bx_4.check_id);
        local_bx_4.dlg_item_id_2 = u_var1;
    }
    SetDlgItemInt16(0, local_bx_4.dlg_item_id, 0x18e, local_bx_4.h_wnd);
    SetDlgItemInt16(0, local_bx_4.dlg_item_id_2, 0x191, local_bx_4.h_wnd);
    match (local_bx_4.check_id) {
        0x188 => local_bx_4.field_0xa4 = 5,
        0x189 => local_bx_4.field_0xa4 = 6,
        0x18a => local_bx_4.field_0xa4 = 7,
        0x18b => local_bx_4.field_0xa4 = 8,
        0x18c => local_bx_4.field_0xa4 = 9,
        0x18d => {
            local_bx_4.field_0xa4 = 10;
        }
    }
    invalidate_rect_1040_3ddc(param_1);
    return;
}

pub fn invalidate_rect_1040_3ddc() {
    let unaff_ss: HWND16;
    let mut local_a: u32;
    let mut local_6: u32;

    local_a = 0x780005;
    local_6 = 0xdc0069;
    InvalidateRect16(0, &local_a, unaff_ss);
    return;
}

pub fn release_dc_1040_3d5e(param_1: u32) -> u16 {
    let pp_var1: fn();
    let mut i_var2: i32;
    let mut i_var3: i32;
    let mut u_var4: u16;
    let pu_var5: *mut u32;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    u_var4 = (param_1 >> 0x10);
    i_var2 = param_1;
    local_4 = GetDC16((i_var2 + 6));
    pu_var5 = mixed_fn_1010_830a(ctx._g_struct_73_1050_14cc, (i_var2 + 0xa4));
    unsafe {
        i_var3 = *pu_var5;
    }
    pp_var1 = (i_var3 + 8);
    unsafe {
        (**pp_var1)(0x1010, pu_var5, &local_4);
    }
    pp_var1 = (i_var3 + 4);
    unsafe {
        (**pp_var1)(0x1010, pu_var5, 0x50078, &local_4);
    }
    pp_var1 = (i_var3 + 0xc);
    (**pp_var1)(0x1010, pu_var5, &local_4);
    ReleaseDC16(local_4, (i_var2 + 6));
    return 0;
}

pub fn win_func_1040_3c64(param_1: i32, param_2: u16, param_3: u16, param_4: u32) {
    let mut u_var1: u16;
    let mut u_var2: u32;
    let LVar3: LRESULT;
    let paVar4: *mut Struct318;
    let b_var5: bool;
    let mut local_8: u16;

    if (param_3._2_2_ == (s_You_may_not_run_a_turn__The_game_1050_0172 + 0x14)) {
        LVar3 = SendDlgItemMessage16(0, 0, 0x409, 400, (param_1 + 6));
        u_var1 = GetDlgItemInt16(0, 0x0, 0, 0x18e);
        set_struct_1018_36e6((param_1 + 0x8e), u_var1, LVar3, (param_1 + 0xa0));
        pass1_1038_af40(ctx._g_Struct112_a, *(param_1 + 8), 0x22);
        SendMessage16(0, 2, 0x111, (param_1 + 6));
        b_var5 = 1;
        paVar4 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, 0x1002b);
        pass1_1010_038e(paVar4, b_var5);
    } else {
        if (param_3._2_2_ + -0xc3 < &dos_alloc_addr_1050_0002) {
            // LAB_1040_3c7f:
            post_win_msg_1040_7b3c(param_1, param_2, param_3_00, param_3, param_3._2_2_);
            return;
        }
        if (param_3._2_2_ + -0xc4 < (&PTR_DAT_0005_0000_1050_0004 + 1)
            || param_3._2_2_ == (s_You_may_not_run_a_turn__The_game_1050_0172 + 0x1b))
        {
            (param_1 + 0xa0) = param_3._2_2_;
            u_var2 = string_fn_1018_3b9e((param_1 + 0x8e), param_3._2_2_);
            send_dialog_item_msg_1040_3f12(param_1, param_2, u_var2);
        } else {
            if (param_3._2_2_ + -0xc4 != &BYTE_1050_0008) {}
            // goto LAB_1040_3c7f;
            if (param_3 != 1) {
                return;
            }
        }
        win_gui_dialog_func_1040_3e08(CONCAT22(param_2, param_1));
    }
    return;
}

pub fn gui_win_fn_1040_3b1e(param_1: *mut Struct25) {
    let in_struct_a: *mut Struct298;
    let b_var1: bool;
    let HVar2: HWND16;

    let mut i_var3: i32;
    let mut unaff_si: u16;
    let mut u_var4: i32;
    let mut unaff_ss: u16;
    let mut local_11a: u16;
    let mut local_118: u16;
    let mut local_112: u16;
    let mut local_110: u16;
    let mut local_10e: u16;
    let puStack268: Vec<u8>;
    let BStack264: bool;
    let paStack262: *mut Struct566;
    let mut local_8c: [u8; 130];
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    _local_6 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(unaff_si, 2));
    local_a = (_local_6 + 0x68);
    u_var4 = (param_1 >> 0x10);
    i_var3 = param_1;
    GetWindowText16(0x80, CONCAT22(unaff_ss, local_8c), (i_var3 + 6));
    wsprintf16(
        &local_10e,
        CONCAT22(local_8c, unaff_ss),
        CONCAT22(local_a, unaff_ss),
    );
    SetWindowText16(CONCAT22(unaff_ss, &local_10e), (i_var3 + 6));
    pass1_1018_3d44(
        ((i_var3 + 0x8e) & 0xffff | (i_var3 + 0x96) << 0x10),
        param_1 & 0xffff0000 | u_var4,
        param_1 & 0xffff0000 | (i_var3 + 0x96),
    );
    local_11a = (ctx._g_struct_73_1050_14cc >> 0x10);
    load_string_1010_84e0(
        ctx._g_struct_73_1050_14cc,
        local_11a,
        0x80,
        CONCAT22(unaff_ss, &local_10e),
        0x7d7,
    );
    local_118 = *CONCAT22(0x80, local_11a);
    wsprintf16(
        local_8c,
        CONCAT22(&local_10e, unaff_ss),
        CONCAT22(local_118, unaff_ss),
    );
    paStack262 = (i_var3 + 6);
    BStack264 = 0x187;
    puStack268 = local_8c;
    local_10e = SUB42(offset, 0);
    SetDlgItemText16(CONCAT22(unaff_ss, puStack268), 0x187, paStack262);
    paStack262 = (i_var3 + 6);
    BStack264 = 0x188;
    puStack268 = 0x188;
    local_10e = SUB42(offset, 0);
    b_var1 = CheckRadioButton16(0x188, 0x18d, 0x188, paStack262);
    (i_var3 + 0xa0) = 0x188;
    paStack262 = (i_var3 + 0xa0);
    in_struct_a = (i_var3 + 0x8e);
    BStack264 = (in_struct_a >> 0x10);
    puStack268 = offset;
    local_10e = 0x3c2b;
    string_fn_1018_3b9e(in_struct_a, paStack262);
    local_10e = 0x1018;
    puStack268 = i_var3;
    BStack264 = b_var1;
    paStack262 = ctx.dx_reg;
    send_dialog_item_msg_1040_3f12();
    puStack268 = 0x3c47;
    BStack264 = i_var3;
    paStack262 = u_var4;
    win_gui_dialog_func_1040_3e08(param_1);
    paStack262 = (i_var3 + 6);
    BStack264 = 0x186;
    puStack268 = 0x3c56;
    HVar2 = GetDlgItem16(0x186, paStack262);
    (i_var3 + 0x9a) = HVar2;
    return;
}

pub fn enable_window_1040_3a36(param_1: u32, param_2: u16, param_3: u16, param_4: i32) -> u16 {
    let pi_var1: *mut i32;
    let mut b_var2: bool;
    let mut i_var3: i32;
    let mut u_var4: u16;
    let mut local_4: u16;

    b_var2 = false;
    i_var3 = param_1;
    u_var4 = (param_1 >> 0x10);
    if (param_2 == 0) {
        if ((i_var3 + 0x9e) <= (i_var3 + 0x9c)) {}
        // goto LAB_1040_3a79;
        pi_var1 = (i_var3 + 0x9c);
        unsafe {
            *pi_var1 = *pi_var1 + 1;
        }
    } else {
        if (param_2 != 1) {}
        // goto LAB_1040_3a79;
        if ((i_var3 + 0x9c) == 0) {}
        // goto LAB_1040_3a79;
        pi_var1 = (i_var3 + 0x9c);
        unsafe {
            *pi_var1 = *pi_var1 + -1;
        }
    }
    b_var2 = true;
    // LAB_1040_3a79:
    if (b_var2) {
        SetDlgItemInt16(0, *(i_var3 + 0x9c), 0x18e, (i_var3 + 6));
    }
    if (((i_var3 + 0x9c) != 0) && ((i_var3 + 0xa2) == 0)) {
        (i_var3 + 0xa2) = 1;
        EnableWindow16(1, (i_var3 + 0x9a));
    }
    if (((i_var3 + 0x9c) == 0) && ((i_var3 + 0xa2) != 0)) {
        (i_var3 + 0xa2) = 0;
        EnableWindow16(0, (i_var3 + 0x9a));
    }
    return 0;
}

pub fn win_fn_1040_3ae8(param_1: *mut Struct20) {
    let mut u_var1: u16;

    ui1::win_gui_func_1040_78e2(param_1);
    ui1::move_window_1040_826c(param_1, 0xffffffff);
    u_var1 = (param_1 >> 0x10);
    ShowWindow16(5, (param_1 + 6));
    SetFocus16((param_1 + 6));
    return;
}

pub fn win_fn_1040_37f0(param_1: i32, param_2: u16, param_3: u16, param_4: u32) {
    let mut unaff_ss: u16;
    let mut in_stack_0000fbec: u16;
    let b_var1: bool;
    let mut local_40c: [u8; 1026];
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    if (param_3._2_2_ == 0x193) {
        _local_6 =
            process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(in_stack_0000fbec, 2));
        local_a = (_local_6 + 0x68);
        load_string_1010_84e0(
            ctx._g_struct_73_1050_14cc,
            (ctx._g_struct_73_1050_14cc >> 0x10),
            0x3ff,
            CONCAT22(unaff_ss, local_40c),
            0x44f,
        );
        MessageBox16(0x30, local_a, CONCAT22(unaff_ss, local_40c), (param_1 + 6));
        pass1_1018_3710((param_1 + 0x8e));
        PostMessage16(0, 2, 0x111, (param_1 + 6));
    } else {
        if (param_3._2_2_ != 0x194) {
            post_win_msg_1040_7b3c(param_1, param_2, param_3_00, param_3, param_3._2_2_);
            return;
        }
        pass1_1038_af40(ctx._g_Struct112_a, *(param_1 + 8), 0x21);
        SendMessage16(0, 2, 0x111, (param_1 + 6));
        b_var1 = 1;
        _local_6 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, 0x1002b);
        pass1_1010_038e(_local_6, b_var1);
    }
    return;
}

pub fn set_focus_1040_355a(param_1: *mut Struct20) {
    let mut u_var1: u16;

    ui1::win_gui_func_1040_78e2(param_1);
    ui1::move_window_1040_826c(param_1, 0xffffffff);
    u_var1 = (param_1 >> 0x10);
    ShowWindow16(5, (param_1 + 6));
    SetFocus16((param_1 + 6));
    return;
}

pub fn win_fn_1040_3590(param_1: u32) {
    let mut u_var1: u32;
    let HVar2: HWND16;
    let hwnd: HWND16;


    let mut extraout_d_x_01: i32;
    let mut i_var3: i32;
    let mut u_var4: u16;
    let mut unaff_ss: u16;
    let mut local_5b0: u16;
    let mut uStack1454: u16;
    let uStack1450: u8;
    let uStack1449: u8;
    let mut local_5a8: u16;
    let mut local_5a6: u16;
    let mut local_5a4: u16;
    let mut local_5a2: u16;
    let mut local_5a0: u16;
    let mut local_59e: u16;
    let mut local_59c: u16;
    let mut local_59a: u32;
    let mut local_596: u32;
    let mut local_592: u16;
    let mut local_590: u16;
    let mut local_58e: u16;
    let mut uStack1420: u16;
    let uStack1418: u8;
    let uStack1417: u8;
    let HStack1410: HWND16;
    let HStack1408: HWND16;
    let mut local_50c: [u8; 256];
    let mut local_40c: u32;
    let mut local_408: u16;
    let mut local_406: u16;
    let mut local_404: [u8; 1026];

    _local_408 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(local_5b0, 2));
    local_40c = (_local_408 + 0x68);
    u_var4 = (param_1 >> 0x10);
    i_var3 = param_1;
    GetWindowText16(0x80, CONCAT22(unaff_ss, local_50c), (i_var3 + 6));
    wsprintf16(
        &local_58e,
        CONCAT22(local_50c, unaff_ss),
        CONCAT22(local_40c, unaff_ss),
    );
    local_592 = SetWindowText16(CONCAT22(unaff_ss, &local_58e), (i_var3 + 6));
    u_var1 = (i_var3 + 0x8e);
    local_5a6 = u_var1;
    local_5a4 = (u_var1 >> 0x10);
    wsprintf_1018_34b6(local_5a6, local_5a4);
    local_590 = ctx.dx_reg;
    pass1_1018_3d44(
        *(i_var3 + 0x8e),
        CONCAT22(unaff_ss, &local_59a),
        CONCAT22(unaff_ss, &local_596),
    );
    HVar2 = GetDlgItem16(0x193, (i_var3 + 6));
    (i_var3 + 0x98) = HVar2;
    EnableWindow16(1, HVar2);
    uStack1454 = ctx._g_struct_73_1050_14cc;
    load_string_1010_84e0(
        uStack1454,
        (ctx._g_struct_73_1050_14cc >> 0x10),
        0x3ff,
        CONCAT22(unaff_ss, local_404),
        1099,
    );
    uStack1450 = SUB21(local_404, 0);
    uStack1449 = (local_404 >> 8);
    wsprintf16(
        local_50c,
        CONCAT13(uStack1449, CONCAT12(uStack1450, unaff_ss)),
        CONCAT22(local_596, unaff_ss),
    );
    local_59a._2_2_ = (i_var3 + 6);
    local_59a._0_2_ = 0x195;
    local_59a._2_2_ = GetDlgItem16(0x195, local_59a._2_2_);
    SetWindowText16(CONCAT22(unaff_ss, local_50c), local_59a._2_2_);
    local_596._0_2_ = (i_var3 + 6);
    local_59a._2_2_ = 0x196;
    local_59a._0_2_ = offset;
    HVar2 = GetDlgItem16(0x196, local_596);
    u_var1 = (i_var3 + 0x8e);
    local_59a._0_2_ = u_var1;
    local_59a._2_2_ = (u_var1 >> 0x10);
    local_596._0_2_ = HVar2;
    wsprintf_1018_34b6();
    local_59a._0_2_ = HVar2;
    local_59a._2_2_ = ctx.dx_reg;
    SetWindowText16(CONCAT22(ctx.dx_reg, HVar2), local_596);
    local_596._2_2_ = (i_var3 + 6);
    local_596._0_2_ = 0x197;
    local_59a._2_2_ = offset;
    local_59a._0_2_ = 0x36cb;
    GetDlgItem16(0x197, local_596._2_2_);
    local_596._2_2_ = 0x443;
    local_59a._2_2_ = local_404;
    local_59a._0_2_ = 0x3ff;
    local_59c = (ctx._g_struct_73_1050_14cc >> 0x10);
    load_string_1010_84e0(
        ctx._g_struct_73_1050_14cc,
        local_59c,
        0x3ff,
        CONCAT22(unaff_ss, local_59a._2_2_),
        0x443,
    );
    local_596._2_2_ = offset;
    local_59a._2_2_ = local_404;
    local_59a._0_2_ = 0x1010;
    SetWindowText16(CONCAT22(unaff_ss, local_59a._2_2_), offset);
    local_592 = 0x44c;
    local_596._0_2_ = local_404;
    local_59a._2_2_ = 0x3ff;
    local_59c = ctx._g_struct_73_1050_14cc;
    local_59a._0_2_ = (ctx._g_struct_73_1050_14cc >> 0x10);
    load_string_1010_84e0(
        local_59c,
        local_59a,
        0x3ff,
        CONCAT22(unaff_ss, local_596),
        0x44c,
    );
    local_596._2_2_ = local_59a;
    local_592 = local_59a._2_2_;
    local_59a._2_2_ = local_404;
    wsprintf16(
        local_50c,
        CONCAT22(local_59a._2_2_, unaff_ss),
        CONCAT22(local_59a, unaff_ss),
    );
    uStack1418 = 0x38;
    uStack1417 = 0x15;
    uStack1420 = 0x3732;
    HVar2 = GetDlgItem16(0x198, (i_var3 + 6));
    uStack1418 = SUB21(local_50c, 0);
    uStack1417 = (local_50c >> 8);
    uStack1420 = offset;
    local_58e = 0x3742;
    SetWindowText16(CONCAT22(unaff_ss, local_50c), HVar2);
    uStack1418 = 0x51;
    uStack1417 = 0x37;
    hwnd = GetDlgItem16(0x199, (i_var3 + 6));
    uStack1418 = 99;
    uStack1417 = 0x37;
    HVar2 = hwnd;
    wsprintf_1018_35b0();
    if ((extraout_d_x_01 | HVar2) == 0) {
        uStack1418 = 0xff;
        uStack1417 = 3;
        local_58e = ctx._g_struct_73_1050_14cc;
        uStack1420 = (ctx._g_struct_73_1050_14cc >> 0x10);
        local_590 = 0x1018;
        local_592 = 0x3785;
        load_string_1010_84e0(
            local_58e,
            uStack1420,
            0x3ff,
            CONCAT22(unaff_ss, local_404),
            0x44d,
        );
        uStack1418 = 0x10;
        uStack1417 = 0x10;
        uStack1420 = 0x3794;
        SetWindowText16(CONCAT22(unaff_ss, local_404), hwnd);
        HStack1410 = (i_var3 + 6);
        HVar2 = GetDlgItem16(0x19a, HStack1410);
        HStack1410 = 0x44e;
        uStack1420 = ctx._g_struct_73_1050_14cc;
        uStack1418 = (ctx._g_struct_73_1050_14cc >> 0x10);
        uStack1417 = (ctx._g_struct_73_1050_14cc >> 0x18);
        local_58e = offset;
        local_590 = 0x37bd;
        load_string_1010_84e0(
            uStack1420,
            (ctx._g_struct_73_1050_14cc >> 0x10),
            0x3ff,
            CONCAT22(unaff_ss, local_404),
            0x44e,
        );
        uStack1418 = 0xcc;
        uStack1417 = 0x37;
        HStack1410 = HVar2;
        SetWindowText16(CONCAT22(unaff_ss, local_404), HVar2);
        HStack1408 = (i_var3 + 0x98);
        HStack1410 = 0;
        EnableWindow16(0, HStack1408);
        return;
    }
    uStack1418 = 0x18;
    uStack1417 = 0x10;
    uStack1420 = 0x37eb;
    SetWindowText16(CONCAT22(extraout_d_x_01, HVar2), hwnd);
    return;
}

pub fn enable_window_1040_32a8(param_1: &mut Vec<u8>) {
    let lp_string: SEGPTR;
    let b_var1: bool;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;

    _local_c = param_1 & 0xffff0000 | (param_1 + 0x19a);
    lp_string = pass1_1018_3a5c(
        (param_1 + 0x96),
        param_1 & 0xffff0000 | (param_1 + 0x9a),
        param_1 & 0xffff0000 | (param_1 + 0x19a),
    );
    SetWindowText16(lp_string, (param_1 + 0x90));
    b_var1 = pass1_1018_39d8(
        (param_1 + 0x96),
        param_1 & 0xffff0000 | (param_1 + 0x9a),
        _local_c,
    );
    EnableWindow16(b_var1 & 1, (param_1 + 0x8e));
    return;
}

pub fn set_window_pos_1040_331a(param_1: &mut Vec<u8>, param_2: u16, param_3: u16) -> u16 {
    let mut unaff_cs: u16;
    let unaff_ss: HWND16;
    let mut local_e: u16;
    let mut local_a: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    local_4 = param_3;
    local_6 = param_2;
    if (param_3 == 1) {
        enable_window_1040_32a8(param_1);
    } else {
        if (param_3 != 7) {
            return 0;
        }
        GetWindowRect16(CONCAT22(&local_e, unaff_cs), unaff_ss);
        local_a = local_a - local_e;
        SetWindowPos16(2, 0x50, local_a, 0, 0, 0, local_6);
    }
    return 1;
}

pub fn win_fn_1040_311a(param_1: i32, param_2: u16, param_3: u16, param_4: u32) {
    let mut i_var1: i32;
    let mut i_var2: i32;
    let ppVar3: *mut pass1_struct_2;
    let ctx.dx_reg: Vec<u8>;
    let pu_var4: Vec<u8>;
    let pa_var5: *mut Struct318;
    let mut u_var6: u16;
    let BVar7: bool;
    let mut local_12: u16;
    let mut local_10: u16;

    send_msg_1040_323c(param_1, param_2_00);
    load_string_1010_847e(
        ctx._g_struct_73_1050_14cc,
        (ctx._g_struct_73_1050_14cc >> 0x10),
        0x531,
    );
    if (param_2._2_2_ == 0x181) {
        i_var1 = param_1 + 0x9a;
        i_var2 = i_var1;
        pass1_1018_3cda(
            (param_1 + 0x96),
            CONCAT22(param_2_00, param_1 + 0x19a),
            CONCAT22(param_2_00, i_var1),
        );
        pass1_1018_3424((param_1 + 0x96));
        if (i_var2 == 0) {
            u_var6 = 0x21;
        } else {
            pass1_1018_3a42((param_1 + 0x96), CONCAT22(param_2_00, i_var1));
            pu_var4 = ctx.dx_reg;
            ppVar3 = pass1_1030_8344(
                ctx._g_bool_1050_5748,
                (ctx._g_bool_1050_5748 >> 0x10),
                CONCAT22(ctx.dx_reg, i_var2),
            );
            ctx.PTR_LOOP_1050_5f0c = pass1_1030_8344(
                ctx._g_bool_1050_5748,
                (ctx._g_bool_1050_5748 >> 0x10),
                &ppVar3.field_0x10,
            );
            PTR_LOOP_1050_5f10 = (&ctx.PTR_LOOP_1050_0000 + 1);
            u_var6 = 0x25;
            ctx.PTR_LOOP_1050_5f0e = pu_var4;
        }
        pass1_1038_af40(ctx._g_Struct112_a, *(param_1 + 8), u_var6);
        SendMessage16(0, 2, 0x111, (param_1 + 6));
        BVar7 = 1;
        pa_var5 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, 0x1002b);
        pass1_1010_038e(pa_var5, BVar7);
    } else {
        if ((param_2._2_2_ == 0x181) || (1 < param_2._2_2_ - 0x182)) {
            post_win_msg_1040_7b3c(param_1, param_2_00, param_3, param_2, param_2._2_2_);
            return;
        }
        set_window_pos_1040_331a(param_1, param_2_00, param_3, param_2, param_2._2_2_);
    }
    return;
}

pub fn set_focus_1040_2f5a(param_1: *mut Struct20) {
    let mut u_var1: u16;

    ui1::win_gui_func_1040_78e2(param_1);
    ui1::move_window_1040_826c(param_1, 0xffffffff);
    u_var1 = (param_1 >> 0x10);
    ShowWindow16(5, (param_1 + 6));
    SetFocus16((param_1 + 6));
    return;
}

pub fn win_fn_1040_2f90(param_1: u32) {
    let HVar1: HWND16;
    let mut u_var2: u32;


    let mut i_var3: i32;
    let mut u_var4: u16;
    let mut unaff_ss: u16;
    let ppVar5: *mut pass1_struct_1;
    let in_stack_0000fed2: u8;
    let in_stack_0000fed3: u8;
    let mut local_126: u16;
    let mut local_124: u16;
    let mut local_122: u16;
    let mut local_120: u16;
    let mut local_11e: u32;
    let mut local_11a: u16;
    let mut local_118: u16;
    let mut local_116: u32;
    let mut local_112: u32;
    let mut local_10e: [u8; 130];
    let mut local_8c: [u8; 130];
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    _local_6 = process_struct_1010_20ba(
        ctx._g_Struct372_1050_0ed0,
        CONCAT13(in_stack_0000fed3, CONCAT12(in_stack_0000fed2, 2)),
    );
    local_a = (_local_6 + 0x68);
    u_var4 = (param_1 >> 0x10);
    i_var3 = param_1;
    GetWindowText16(0x80, CONCAT22(unaff_ss, local_8c), (i_var3 + 6));
    wsprintf16(
        local_10e,
        CONCAT22(local_8c, unaff_ss),
        CONCAT22(local_a, unaff_ss),
    );
    SetWindowText16(CONCAT22(unaff_ss, local_10e), (i_var3 + 6));
    HVar1 = GetDlgItem16(0x182, (i_var3 + 6));
    (i_var3 + 0x92) = HVar1;
    pass1_1018_3a94(
        *(i_var3 + 0x96),
        CONCAT22(unaff_ss, &local_116),
        CONCAT22(unaff_ss, &local_112),
    );
    local_126 = local_112;
    local_124 = (local_112 >> 0x10);
    win_fn_1040_3374(
        i_var3,
        (param_1 >> 0x10),
        local_126,
        local_124,
        (i_var3 + 0x92),
    );
    ppVar5 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(local_120, 0x2f));
    u_var2 = (ppVar5 + 0x24);
    pass1_1018_3a7a(*(i_var3 + 0x96), u_var2);
    SendMessage16(
        CONCAT13((ctx.dx_reg >> 8), CONCAT12(ctx.dx_reg, u_var2)),
        0xffff,
        0x40d,
        (i_var3 + 0x92),
    );
    HVar1 = GetDlgItem16(0x183, (i_var3 + 6));
    (i_var3 + 0x94) = HVar1;
    local_124 = local_116;
    local_122 = (local_116 >> 0x10);
    win_fn_1040_3374(param_1, u_var4, local_124, local_122, HVar1);
    local_124 = ctx._g_struct_73_1050_14cc;
    local_122 = (ctx._g_struct_73_1050_14cc >> 0x10);
    load_string_1010_847e(local_124, local_122, 0x531);
    SendDlgItemMessage16(
        CONCAT13((ctx.dx_reg >> 8), CONCAT12(ctx.dx_reg, HVar1)),
        0,
        0x403,
        0x183,
        (i_var3 + 6),
    );
    SendDlgItemMessage16(0x40dffff, 0xffff, 0x40d, 0x183, (i_var3 + 6));
    HVar1 = GetDlgItem16(0x181, (i_var3 + 6));
    (i_var3 + 0x8e) = HVar1;
    HVar1 = GetDlgItem16(0x184, (i_var3 + 6));
    (i_var3 + 0x90) = HVar1;
    return;
}

pub fn enable_window_1040_2a64(param_1: u32) {
    let mut u_var1: u32;
    let mut u_var2: u16;
    let mut u_var3: i32;
    let struct_a: *mut Struct199;
    let mut i_var4: i32;
    let local_bx_215: *mut Struct58;
    let mut u_var5: u16;
    let mut h_wnd: u16;
    let local_28: *mut Struct57;
    let mut uStack38: u16;
    let mut local_1c: u16;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    local_28 = param_1;
    uStack38 = (param_1 >> 0x10);
    ui1::set_window_pos_1040_b230(local_28, uStack38);
    local_4 = 5;
    u_var1 = local_28.field_0x90;
    _local_c = pass1_1030_73a8((u_var1 + 6));
    struct_a = (_local_c >> 0x10);
    u_var5 = SUB42(&PTR_LOOP_1050_1028, 0);
    PTR_LOOP_1050_5d04 = pass1_1028_4a9a(_local_c, 0);
    local_e = 0;
    while (local_e < local_4) {
        if (local_e != 0) {
            (&PTR_LOOP_1050_5d04 + local_e * 0xc) = 0;
        }
        local_bx_215 = (local_e * 0xc);
        local_16 = (local_bx_215 + 0x5cfc);
        local_14 = (local_bx_215 + 0x5cfe);
        u_var3 = 1;
        local_12 = 1;
        local_10 = 1;
        u_var2 = local_28.field_0x6;
        MapDialogRect16(CONCAT22(&local_16, u_var5), h_wnd);
        u_var5 = 0x1000;
        process_struct_1000_179c(0x42, struct_a);
        if ((struct_a | u_var3) == 0) {
            _local_8 = 0;
        } else {
            u_var5 = SUB42(&ctx.PTR_LOOP_1050_1008, 0);
            _local_8 = ui1::win_fn_1008_3bd6(
                u_var3,
                struct_a,
                1,
                CONCAT22(local_16, local_14),
                0x1000101,
                CONCAT22((local_bx_215 + 0x5d00), 0xff),
                CONCAT22(u_var2, local_28.field_0x6),
            );
        }
        struct_a = (_local_8 >> 0x10);
        if (PTR_LOOP_1050_5d04 == 0x0) {
            if ((local_e != 0) && (_local_8 != 0)) {
                u_var5 = SUB42(offset, 0);
                EnableWindow16(0, (_local_8 + 0x18));
            }
        } else {
            i_var4 = local_e * 0xc;
            u_var5 = SUB42(&PTR_LOOP_1050_1028, 0);
            u_var2 = pass1_1028_4a9a(_local_c, (i_var4 + 0x5d02));
            if (u_var2 != 0) {
                (&PTR_LOOP_1050_5d04 + i_var4) = 1;
                u_var5 = SUB42(offset, 0);
                SetDlgItemText16(
                    local_28.field_0x98,
                    (&PTR_s_post_1050_015d_1050_5d06 + i_var4),
                    local_28.field_0x6,
                );
            }
        }
        local_e = local_e + 1;
    }
    return;
}

pub fn enable_window_1040_2bb2(param_1: *mut Struct124, param_2: u16, param_3: u16, param_4: u32) {
    let mut u_var1: i32;
    let HVar2: HWND16;
    let mut i_var3: i32;
    let lp_string: SEGPTR;
    let id: Vec<u8>;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    if (param_3._2_2_ == 0x158) {
        PTR_LOOP_1050_5d04 = (PTR_LOOP_1050_5d04 == 0x0);
        if (PTR_LOOP_1050_5d04 == 0x0) {
            local_8 = 1;
            while (local_8 < 5) {
                i_var3 = local_8 * 0xc;
                HVar2 = GetDlgItem16((i_var3 + 0x5d00), &param_1.field_0x6);
                EnableWindow16(0, HVar2);
                (&PTR_LOOP_1050_5d04 + i_var3) = 0;
                SetDlgItemText16(
                    (param_1 + 1),
                    (&PTR_s_post_1050_015d_1050_5d06 + i_var3),
                    &param_1.field_0x6,
                );
                local_8 = local_8 + 1;
            }
            HVar2 = &param_1.field_0x6;
            lp_string = (param_1 + 1);
            id = PTR_s_post_1050_015d_1050_5d06;
            // goto LAB_1040_2ccc;
        }
        local_8 = 1;
        while (local_8 < 5) {
            i_var3 = local_8 * 0xc;
            HVar2 = GetDlgItem16((i_var3 + 0x5d00), &param_1.field_0x6);
            EnableWindow16(1, HVar2);
            (&PTR_LOOP_1050_5d04 + i_var3) = 0;
            SetDlgItemText16(
                (param_1 + 1),
                (&PTR_s_post_1050_015d_1050_5d06 + i_var3),
                &param_1.field_0x6,
            );
            local_8 = local_8 + 1;
        }
        HVar2 = &param_1.field_0x6;
        id = PTR_s_post_1050_015d_1050_5d06;
    } else {
        if (param_3._2_2_ == 0x159) {
            local_4 = 1;
        } else {
            if (param_3._2_2_ == 0x15a) {
                local_4 = 2;
            } else {
                if (param_3._2_2_ == 0x15b) {
                    local_4 = 3;
                } else {
                    if (param_3._2_2_ != 0x15c) {
                        ui1::win_gui_fn_1040_b54a(param_1, param_2, param_3_00, param_3);
                        return;
                    }
                    local_4 = 4;
                }
            }
        }
        if (local_4 == 0) {
            return;
        }
        i_var3 = local_4 * 0xc;
        u_var1 = ((&PTR_LOOP_1050_5d04 + i_var3) == 0);
        (&PTR_LOOP_1050_5d04 + i_var3) = u_var1;
        if (u_var1 == 0) {
            HVar2 = &param_1.field_0x6;
            lp_string = (param_1 + 1);
            id = (&PTR_s_post_1050_015d_1050_5d06 + i_var3);
            // goto LAB_1040_2ccc;
        }
        HVar2 = &param_1.field_0x6;
        id = (&PTR_s_post_1050_015d_1050_5d06 + local_4 * 0xc);
    }
    lp_string = &param_1[1].field_0x4;
    // LAB_1040_2ccc:
    SetDlgItemText16(lp_string, id, HVar2);
    return;
}

pub fn win_fn_1040_2d48(param_1: u32) {
    let mut u_var1: u16;
    let mut value: u16;
    let unaff_ss: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let local_4: bool;

    ui1::set_dlg_item_txt_1040_b45e(param_1);
    u_var1 = GetDlgItemInt16(1, &local_4, unaff_ss, 0x163);
    value = GetDlgItemInt16(1, &local_4, unaff_ss, 0x165);
    if (u_var1 != 0) {
        value = value / u_var1;
    }
    SetDlgItemInt16(1, value, 0x165, (param_1 + 6));
    return;
}

pub fn enable_window_1040_2490(param_1: *mut Struct20) {
    let pp_var1: fn();
    let h_wnd: HWND16;
    let mut u_var2: u16;
    let mut i_var3: i32;
    let mut u_var4: u16;
    let pi_var5: *mut i32;
    let mut local_6: u16;
    let mut local_4: u16;

    ui1::win_gui_func_1040_78e2(param_1);
    u_var4 = (param_1 >> 0x10);
    i_var3 = param_1;
    h_wnd = GetDlgItem16(0xfb1, (i_var3 + 6));
    EnableWindow16(0, h_wnd);
    pp_var1 = ((i_var3 + 0x8e) + 0x10);
    pi_var5 = (**pp_var1)(offset, (i_var3 + 0x8e));
    u_var2 = (pi_var5 >> 0x10);
    ui1::move_window_1040_826c(
        i_var3,
        u_var4,
        (pi_var5 + 2) + -2,
        (pi_var5 + 4) + unsafe { *pi_var5 } + 3,
    );
    ShowWindow16(5, (i_var3 + 6));
    pass1_1018_1c9a(*(i_var3 + 0x8e), 0x1a0);
    return;
}

pub fn win_fn_1040_2512(param_1: u32, param_2: u32, param_3: u16) {
    let pi_var1: *mut i32;
    let ppc_var2: fn();
    let mut u_var3: u32;
    let mut u_var4: i32;
    let HVar5: HWND16;
    let pu_var6: Vec<u8>;
    let mut u_var7: u16;
    let mut u_var8: u32;
    let in_dx: *mut Struct199;

    let mut u_var9: i32;
    let mut i_var10: i32;
    let mut i_var11: i32;
    let mut u_var12: u16;
    let mut u_var13: u16;
    let unaff_ss: String;
    let u_var14: u8;
    let in_stack_0000ffdc: String;
    let mut local_20: u16;
    let mut local_1e: [u8; 4];
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;

    local_6 = 0;
    if (param_3 != 2) {
        i_var10 = param_1;
        if (0x19d < param_3 - 2) {
            u_var12 = (param_1 >> 0x10);
            u_var14 = (unaff_ss >> 8);
            if (param_3 - 0x1a0 < 0x14 || param_3 == 0x1b4) {
                u_var7 = IsDlgButtonChecked16(param_3, (i_var10 + 6));
                if (u_var7 == 0) {
                    pi_var1 = (i_var10 + 0x92);
                    unsafe {
                        *pi_var1 = *pi_var1 + 1;
                    }
                    if (0 < (i_var10 + 0x92)) {
                        (i_var10 + 0x94) = 0;
                    }
                    u_var3 = (i_var10 + 0x8e);
                    if ((u_var3 + 0x28) == (i_var10 + 0x92)) {
                        HVar5 = GetDlgItem16(0xfb1, (i_var10 + 6));
                        EnableWindow16(0, HVar5);
                    }
                } else {
                    pi_var1 = (i_var10 + 0x92);
                    unsafe {
                        *pi_var1 = *pi_var1 + -1;
                    }
                    HVar5 = GetDlgItem16(0xfb1, (i_var10 + 6));
                    IsWindowEnabled16(offset, HVar5);
                    u_var4 = ctx.dx_reg;
                    if (HVar5 == 0) {
                        HVar5 = GetDlgItem16(0xfb1, (i_var10 + 6));
                        EnableWindow16(1, HVar5);
                    }
                    if ((i_var10 + 0x92) < 1) {
                        (i_var10 + 0x94) = 1;
                    }
                    pass1_1018_1c9a(*(i_var10 + 0x8e), param_3);
                    u_var8 = pass1_1018_1e78((i_var10 + 0x8e), 0xffff);
                    _local_a = (u_var8 & 0xffff | u_var4 << 0x10);
                    if ((u_var4 | u_var8) == 0) {
                        local_c = 0;
                    } else {
                        local_c = (u_var8 + 0x1c);
                    }
                    mci_send_command_1008_5c7c(ctx._g_struct_ptr_1050_02a0, CONCAT22(local_c, 1));
                }
                if ((i_var10 + 0x92) < 0) {
                    return;
                }
                u_var3 = (i_var10 + 0x8e);
                if ((u_var3 + 0x28) < (i_var10 + 0x92)) {
                    return;
                }
                string_fn_1000_3f9c(
                    &local_16,
                    unaff_ss,
                    s__d_1050_5cf4,
                    &ctx.g_alloc_addr_1050_1050,
                    *(i_var10 + 0x92),
                );
                SetDlgItemText16(
                    CONCAT13(u_var14, CONCAT12(unaff_ss, &local_16)),
                    0xfb2,
                    (i_var10 + 6),
                );
                return;
            }
            u_var4 = param_3 - 0xfb1;
            if (u_var4 == 0) {
                if ((i_var10 + 0x92) < 0) {
                    process_struct_1000_179c(0xb4, in_dx);
                    u_var9 = in_dx | u_var4;
                    local_1a = u_var4;
                    if (u_var9 == 0) {
                        u_var4 = 0;
                        u_var9 = 0;
                    } else {
                        ui1::mixed_1040_8520(
                            CONCAT13((in_dx >> 8), CONCAT12(in_dx, u_var4)),
                            ctx.g_h_window,
                            0x30,
                            2,
                            0x57b,
                            0x57c,
                        );
                    }
                    _local_a = CONCAT22(u_var9, u_var4);
                    ppc_var2 = (*_local_a + 0x74);
                    ppc_var2(0, u_var4, u_var9);
                    return;
                }
                if (0 < (i_var10 + 0x92)) {
                    process_struct_1000_179c(0xb4, in_dx);
                    u_var9 = in_dx | u_var4;
                    local_1a = u_var4;
                    if (u_var9 == 0) {
                        u_var4 = 0;
                        u_var9 = 0;
                    } else {
                        ui1::mixed_1040_8520(
                            CONCAT13((in_dx >> 8), CONCAT12(in_dx, u_var4)),
                            ctx.g_h_window,
                            0x21,
                            2,
                            0x57b,
                            0x57d,
                        );
                    }
                    _local_a = CONCAT22(u_var9, u_var4);
                    pass1_1008_941a(CONCAT13(u_var14, CONCAT12(unaff_ss, local_1e)), 1, 0xc2);
                    pu_var6 = local_1e;
                    ppc_var2 = (*_local_a + 0x6c);
                    ppc_var2(
                        &ctx.PTR_LOOP_1050_1008,
                        _local_a,
                        (_local_a >> 0x10),
                        pu_var6,
                    );
                    in_stack_0000ffdc = unaff_ss;
                    if (pu_var6 == &dos_alloc_addr_1050_0002) {
                        return;
                    }
                }
                _local_16 = process_struct_1010_20ba(
                    ctx._g_Struct372_1050_0ed0,
                    CONCAT22(in_stack_0000ffdc, 6),
                );
                local_c = 0x1a0;
                while {
                    u_var7 = IsDlgButtonChecked16(local_c, (i_var10 + 6));
                    if (u_var7 == 1) {
                        u_var13 = (_local_16 >> 0x10);
                        i_var11 = _local_16;
                        (i_var11 + (i_var11 + 0x56) * 2 + 0x4e) = local_c;
                        pi_var1 = (i_var11 + 0x56);
                        unsafe {
                            *pi_var1 = *pi_var1 + 1;
                        }
                    }
                    local_c = local_c + 1;
                    local_c < 0x1b5
                } {}
                u_var4 = (i_var10 + 0x92);
                _local_a = (_local_a & 0xffff0000 | u_var4);
                u_var3 = (i_var10 + 0x8e);
                (u_var3 + 0x28) = u_var4;
                PostMessage16(0, 200, 0x111, ctx.g_h_window);
                param_3 = 1;
            }
        }
        post_win_msg_1040_7b3c(
            i_var10,
            (param_1 >> 0x10),
            param_2,
            (param_2 >> 0x10),
            param_3,
        );
    }
    return;
}

pub fn create_win_1040_20d4(param_1: *mut Struct20) {
    let mut cx: i32;
    let pu_var1: Vec<u8>;

    let mut i_var2: i32;
    let mut u_var3: u16;
    let mut unaff_ss: u16;
    let ppVar4: *mut pass1_struct_1;
    let mut in_stack_0000ffca: u16;
    let mut local_22: u16;
    let mut local_20: u16;
    let mut local_1e: [u8; 4];
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_16: u32;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    ui1::win_gui_func_1040_78e2(param_1);
    ppVar4 = process_struct_1010_20ba(
        ctx._g_Struct372_1050_0ed0,
        CONCAT22(in_stack_0000ffca, 0x48),
    );
    local_c = (ppVar4 >> 0x10);
    local_e = ppVar4;
    local_8 = (local_e + 10);
    local_a = (local_e + 0xc);
    u_var3 = (param_1 >> 0x10);
    i_var2 = param_1;
    _local_12 = process_struct_1008_4772((i_var2 + 0x8e));
    cx = (_local_12 + 4);
    local_4 = (local_8 - cx) / 2;
    local_6 = 5;
    SetWindowPos16(6, 0x1d6, cx, 5, local_4, 0, (i_var2 + 6));
    pu_var1 = local_1e;
    GetClientRect16(CONCAT22(unaff_ss, pu_var1), (i_var2 + 6));
    load_string_1010_847e(
        ctx._g_struct_73_1050_14cc,
        (ctx._g_struct_73_1050_14cc >> 0x10),
        0x592,
    );
    local_16 = 0x50010001;
    CreateWindow16(
        0,
        ctx.g_h_instance_1050_038c,
        1,
        (i_var2 + 6),
        0x19,
        0x58,
        local_18 - 0x28,
        (local_1a - 0x58) / 2,
        0x50010001,
        CONCAT22(ctx.dx_reg, pu_var1),
        s_OPButton_1050_5ce4,
    );
    SetWindowPos16(
        0x45,
        local_a - 10,
        (_local_12 + 4),
        5,
        local_4,
        0,
        (i_var2 + 6),
    );
    return;
}

pub fn show_window_1040_1d50(param_1: *mut Struct20) {
    ui1::win_gui_func_1040_78e2(param_1);
    ui1::move_window_1040_826c(param_1, 0xffffffff);
    ShowWindow16(5, (param_1 + 6));
    return;
}

pub fn check_dialog_btn_1040_1d7a(param_1: u32, param_2: i32) {
    let mut u_var1: u32;
    let mut u_var2: u16;
    let local_bx_8: *mut Struct51;
    let mut u_var3: u16;

    local_bx_8 = param_1;
    u_var3 = (param_1 >> 0x10);
    if ((param_2 != 0) && (u_var1 = local_bx_8.field_0x8e, (u_var1 + 0x72) != 0)) {
        u_var2 = IsDlgButtonChecked16(0xe1, local_bx_8.field_0x6);
        if (u_var2 != 0) {
            pass1_1008_a930(local_bx_8.field_0x92, 0x1d5);
        }
        u_var2 = IsDlgButtonChecked16(0xe2, local_bx_8.field_0x6);
        if (u_var2 != 0) {
            pass1_1008_a930(local_bx_8.field_0x92, 0x1d6);
        }
        u_var2 = IsDlgButtonChecked16(0xe3, local_bx_8.field_0x6);
        if (u_var2 != 0) {
            pass1_1008_a930(local_bx_8.field_0x92, 0x1d7);
        }
        u_var2 = IsDlgButtonChecked16(0xe5, local_bx_8.field_0x6);
        if (u_var2 != 0) {
            pass1_1008_a930(local_bx_8.field_0x92, 0x1d8);
        }
        u_var2 = IsDlgButtonChecked16(0xe6, local_bx_8.field_0x6);
        if (u_var2 != 0) {
            pass1_1008_a930(local_bx_8.field_0x92, 0x1e2);
        }
        u_var2 = IsDlgButtonChecked16(0xe7, local_bx_8.field_0x6);
        if (u_var2 != 0) {
            pass1_1008_a930(local_bx_8.field_0x92, 0x1dc);
        }
        return;
    }
    DestroyWindow16(local_bx_8.field_0x6);
    return;
}

pub fn pass1_1040_1ab0(param_1: i32, param_2: Vec<u8>, param_3: Vec<u8>, param_2_00: Vec<u8>) {
    let mut local_6: u32;

    local_6 = 0;
    if (param_2_00._2_2_ == 0x1831) {
        (param_1 + 0x92) = 1;
        (param_1 + 0x94) = 1;
        check_dialog_func_1040_1b8a(param_1, param_2);
    } else {
        local_6 = post_win_msg_1040_7b3c(param_1, CONCAT22(param_3, param_2), param_2_00);
    }
    return local_6;
}

pub fn check_dialog_func_1040_1afe(param_1: *mut Struct20) {
    let mut check: u16;
    let mut check_00: u16;
    let mut check_01: u16;
    let mut u_var1: u32;
    let mut u_var2: u32;
    let local_Struct20: *mut Struct20;
    let mut local_es_4: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    local_es_4 = (param_1 >> 0x10);
    local_Struct20 = param_1;
    u_var1 = local_Struct20.field_0x8e;
    u_var2 = local_Struct20.field_0x8e;
    check = *(u_var2 + 0x20);
    u_var2 = local_Struct20.field_0x8e;
    check_00 = *(u_var2 + 0x74);
    u_var2 = local_Struct20.field_0x8e;
    check_01 = *(u_var2 + 0x72);
    CheckDlgButton16(*(u_var1 + 0x1e), 0xfdb, local_Struct20.h_dialog_6);
    CheckDlgButton16(check_00, 0xfdd, local_Struct20.h_dialog_6);
    CheckDlgButton16(check_01, 0xfde, local_Struct20.h_dialog_6);
    CheckDlgButton16(check, 0xfdc, local_Struct20.h_dialog_6);
    return;
}

pub fn check_dialog_func_1040_1b8a(param_1: u32) {
    let mut u_var1: u16;
    let mut check: u16;
    let local_bx_4: *mut Struct21;
    let mut u_var2: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    u_var2 = (param_1 >> 0x10);
    local_bx_4 = param_1;
    u_var1 = return_1_1010_60b4(local_bx_4.field_0x8e);
    return_1_1010_60c6(local_bx_4.field_0x8e);
    check = return_1_1010_60c0();
    return_1_1010_60ba(local_bx_4.field_0x8e);
    CheckDlgButton16(u_var1, 0xfdb, local_bx_4.h_wnd);
    CheckDlgButton16(check, 0xfdd, local_bx_4.h_wnd);
    CheckDlgButton16(0xfde, 0xfde, local_bx_4.h_wnd);
    u_var1 = local_bx_4.h_wnd;
    CheckDlgButton16(u_var1, 0xfdc, u_var1);
    return;
}

pub fn set_win_pos_1040_162a(param_1: u16, param_2: u32, param_3: u32) {
    let mut u_var1: i32;
    let mut unaff_cs: u16;
    let unaff_ss: HWND16;
    let mut u_var2: u32;
    let mut local_a: u16;
    let mut local_6: u16;

    if ((param_3._2_2_ != (s_vrpal_bmp_1050_183a + 5))
        && (param_3._2_2_ != (s_vrpal_bmp_1050_183a + 4)))
    {
        u_var2 = post_win_msg_1040_7b3c(param_1, param_2, param_3);
        return u_var2;
    }
    if (param_3 == 7) {
        GetWindowRect16(CONCAT22(&local_a, unaff_cs), unaff_ss);
        local_6 = local_6 - local_a;
        SetWindowPos16(2, 0x50, local_6, 0, 0, 0, param_2._2_2_);
    } else {
        if ((param_3 != 9) && (param_3 != 10)) {
            u_var1 = 0;
            // goto LAB_1040_164d;
        }
    }
    u_var1 = 1;
    // LAB_1040_164d:
    return u_var1;
}

pub fn win_fn_1040_12bc(param_1: *mut Struct20) {
    let mut u_var1: u32;
    let HVar2: HWND16;
    let mut i_var3: i32;
    let mut u_var4: u16;
    let unaff_ss: String;
    let l_param: LPARAM;
    let mut local_5c: u16;
    let mut local_5a: u16;
    let mut local_58: u16;
    let mut local_56: u16;
    let mut local_54: [u8; 82];

    ui1::win_gui_func_1040_78e2(param_1);
    u_var4 = (param_1 >> 0x10);
    i_var3 = param_1;
    u_var1 = (i_var3 + 0x8e);
    string_fn_1000_3f9c(
        local_54,
        unaff_ss,
        s__u_1050_5cd4,
        &ctx.g_alloc_addr_1050_1050,
        *(u_var1 + 10),
    );
    HVar2 = GetDlgItem16(0xfd2, (i_var3 + 6));
    SendMessage16(CONCAT22(unaff_ss, local_54), 0, 0xc, HVar2);
    SetFocus16(HVar2);
    SendMessage16(-0x10000, 0, 0x401, HVar2);
    ui1::move_window_1040_826c(param_1, u_var4, 0xffff, 0xffff);
    l_param = load_string_1010_847e(
        ctx._g_struct_73_1050_14cc,
        (ctx._g_struct_73_1050_14cc >> 0x10),
        0x531,
    );
    HVar2 = GetDlgItem16((s_vrpal_bmp_1050_183a + 5), (i_var3 + 6));
    send_msg_1040_1696(i_var3, u_var4, HVar2);
    SendMessage16(l_param, 0xffff, 0x40d, HVar2);
    HVar2 = GetDlgItem16((s_vrpal_bmp_1050_183a + 4), (i_var3 + 6));
    send_msg_1040_1696(i_var3, u_var4, HVar2);
    SendMessage16(l_param, 0xffff, 0x40d, HVar2);
    ShowWindow16(5, (i_var3 + 6));
    return;
}

pub fn pass1_1040_109c(param_1: i32, param_2: Vec<u8>, param_3: Vec<u8>, param_3_00: Vec<u8>) {
    let mut u_var1: u32;
    let mut b_var2: bool;
    let mut i_var3: i32;
    let ppVar4: *mut pass1_struct_1;
    let mut in_stack_0000fff2: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    b_var2 = false;
    if (param_3_00._2_2_ == 0x1c1) {
        (param_1 + 0x96) = 2;
        b_var2 = true;
    } else {
        if (param_3_00._2_2_ == 0x1c2) {
            (param_1 + 0x96) = 1;
            b_var2 = true;
        } else {
            if (param_3_00._2_2_ != 0x1830) {
                post_win_msg_1040_7b3c(param_1, param_2, param_3, param_3_00);
                return;
            }
            ppVar4 = process_struct_1010_20ba(
                ctx._g_Struct372_1050_0ed0,
                CONCAT22(in_stack_0000fff2, 0x32),
            );
            u_var1 = (param_1 + 0x92);
            i_var3 = ui1::win_gui_fn_1010_79aa(ppVar4, 0xfb6, (u_var1 + 6));
            if (i_var3 == 0) {
                u_var1 = (param_1 + 0x92);
                u_var1 = (u_var1 + 6);
                window_msg_func_1010_7300(ppVar4, 0, 0, 0xc, u_var1, (u_var1 >> 0x10));
            }
        }
    }
    if (b_var2) {
        u_var1 = (param_1 + 0x8e);
        (u_var1 + 10) = (param_1 + 0x96);
    }
    return;
}

pub fn set_win_pos_1040_0f0c(param_1: *mut Struct20) {
    let mut u_var1: u32;
    let HVar2: HWND16;
    let mut u_var3: u16;
    let mut i_var4: i32;
    let mut u_var5: u16;
    let mut unaff_cs: u16;
    let unaff_ss: HWND16;
    let mut u_var6: u32;
    let id: u16;
    let mut local_34: u16;
    let mut local_32: u16;
    let mut local_2e: [u8; 2];
    let mut local_2c: u16;
    let mut local_26: u16;
    let mut local_24: u16;
    let mut local_22: u16;
    let mut local_20: u16;
    let mut local_1e: u16;
    let mut local_1c: u16;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    ui1::win_gui_func_1040_78e2(param_1);
    u_var5 = (param_1 >> 0x10);
    i_var4 = param_1;
    if ((i_var4 + 0x98) == 0) {
        GetWindowRect16(
            CONCAT13((&local_26 >> 8), CONCAT12(&local_26, unaff_cs)),
            unaff_ss,
        );
        GetDlgItem16(0x1830, (i_var4 + 6));
        GetWindowRect16(CONCAT22(local_2e, 0x1538), unaff_ss);
        local_22 = local_22 - local_26;
        local_20 = (local_2c - local_24) - 2;
        SetWindowPos16(6, local_20, local_22, 0, 0, 0, (i_var4 + 6));
        CheckDlgButton16(1, 0x1c1, (i_var4 + 6));
        u_var1 = (i_var4 + 0x8e);
        (u_var1 + 10) = 2;
        HVar2 = GetDlgItem16(0x1830, (i_var4 + 6));
        EnableWindow16(0, HVar2);
    } else {
        u_var1 = (i_var4 + 0x92);
        u_var6 = pass1_1030_73a8((u_var1 + 6));
        if ((u_var6 + 0x20) == 2) {
            HVar2 = (i_var4 + 6);
            id = 0x1c1;
        } else {
            HVar2 = (i_var4 + 6);
            id = 0x1c2;
        }
        CheckDlgButton16(1, id, HVar2);
    }
    GetCursor16(offset, &local_6);
    u_var3 = (i_var4 + 6);
    GetWindowRect16(CONCAT22(&local_e, 0x1538), unaff_ss);
    local_14 = local_a - local_e;
    local_10 = -(local_14 / 2 - local_6);
    local_16 = local_8 - local_c;
    local_12 = -(local_16 / 2 - local_4);
    _local_1e = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(u_var3, 0x48));
    u_var3 = (_local_1e >> 0x10);
    local_18 = (_local_1e + 10);
    local_1a = (_local_1e + 0xc);
    if (local_18 < (local_14 + local_10)) {
        local_10 = local_18 - local_14;
    }
    if (local_1a < (local_16 + local_12)) {
        local_12 = local_1a - local_16;
    }
    SetWindowPos16(0x45, 0, 0, local_12, local_10, 0, (i_var4 + 6));
    return;
}

pub fn set_colors_1040_0cc0(param_1: *mut u32, param_2: u16, param_3: i32, param_4: HDC16) {
    let pp_var1: fn();
    let mut i_var2: i32;
    let mut u_var3: u16;
    let mut u_var4: u32;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_4: u16;

    local_4 = GetStockObject16(4);
    if (_PTR_LOOP_1050_5cd0 == 0) {
        u_var3 = (param_1 >> 0x10);
        unsafe {
            pp_var1 = (*param_1 + 0x68);
        }
        u_var4 = (**pp_var1)(offset, param_1, u_var3, (param_1 + 0x6e));
        u_var4 = pass1_1008_4d72(u_var4);
        u_var3 = (u_var4 >> 0x10);
        i_var2 = u_var4;
        _PTR_LOOP_1050_5cd0 = CONCAT22(
            CONCAT11(2, *(i_var2 + 0x94)),
            CONCAT11(*(i_var2 + 0x95), *(i_var2 + 0x96)),
        );
    }
    if (3 < param_2) {
        if (param_2 == 4) {
            local_4 = GetStockObject16(5);
        } else {
            if ((param_2 == 4) || (1 < param_2 - 5)) {
                return 0;
            }
        }
    }
    SetTextColor16(_PTR_LOOP_1050_5cd0, param_3);
    SetBkColor16(0x1000000, param_3);
    return CONCAT22(0x1050, local_4);
}

pub fn show_window_1040_0c7c(param_1: *mut Struct20) {
    let mut u_var1: u32;
    let mut u_var2: u16;
    let mut unaff_ss: u16;
    let mut local_6: u32;

    ui1::win_gui_func_1040_78e2(param_1);
    u_var2 = (param_1 >> 0x10);
    u_var1 = (param_1 + 0x8e);
    pass1_1010_4f30(
        u_var1,
        (u_var1 >> 0x10),
        CONCAT22(unaff_ss, &local_6),
        CONCAT22(unaff_ss, &local_6 + 2),
    );
    ui1::move_window_1040_826c(param_1, local_6);
    ShowWindow16(5, (param_1 + 6));
    return;
}

pub fn enable_win_1040_0acc(param_1: u32, param_2: bool) {
    let b_var1: bool;
    let HVar2: HWND16;
    let mut i_var3: i32;
    let mut u_var4: u16;
    let mut local_4: u16;

    u_var4 = (param_1 >> 0x10);
    i_var3 = param_1;
    b_var1 = IsWindow16((i_var3 + 6));
    if (b_var1 != 0) {
        HVar2 = GetDlgItem16(100, (i_var3 + 6));
        b_var1 = IsWindow16(HVar2);
        if (b_var1 != 0) {
            EnableWindow16(param_2, HVar2);
            HVar2 = GetDlgItem16(0x74, (i_var3 + 6));
            EnableWindow16(param_2, HVar2);
            HVar2 = GetDlgItem16(0x73, (i_var3 + 6));
            EnableWindow16(param_2, HVar2);
            HVar2 = GetDlgItem16(0x6e, (i_var3 + 6));
            EnableWindow16(param_2, HVar2);
            HVar2 = GetDlgItem16(0xee, (i_var3 + 6));
            EnableWindow16(param_2, HVar2);
        }
    }
    return;
}

pub fn win_fn_1040_07dc(param_1: i32, param_2: u16, param_3: u16, param_4: u16, param_5: i32) {
    let pp_var1: fn();
    let u_var2: u8;
    let i_var3: u16;
    let b_var4: bool;
    let extraout_var: u32;


    let mut u_var6: i32;

    let mut unaff_ss: u16;
    let pa_var7: *mut Struct434;
    let pu_var8: *mut u32;
    let u_var9: u8;
    let u_var10: u8;
    let u_var12: u8;
    let mut local_810: u32;
    let mut local_80c: u16;
    let mut local_80a: u16;
    let mut local_808: u16;
    let mut local_806: [u8; 1024];
    let mut local_406: u32;
    let mut local_6: u32;
    let mut u_var5: u32;
    let u_var11: u8;

    local_6 = 0;
    u_var9 = param_1;
    u_var10 = param_2_00;
    u_var11 = unaff_ss;
    u_var12 = (unaff_ss >> 8);
    if (param_3 == 0x73) {
        enable_win_1040_0acc(param_1, u_var10, 0);
        u_var6 = ctx.dx_reg;
        u_var2 = string_fn_1008_5fd8(0x7d0, -0x2f);
        u_var5 = CONCAT31(extraout_var, u_var2);
        local_406 = u_var5 & 0xffff | u_var6 << 0x10;
        load_string_1010_84e0(
            ctx._g_struct_73_1050_14cc,
            (ctx._g_struct_73_1050_14cc >> 0x10),
            0x3ff,
            CONCAT13(u_var12, CONCAT12(u_var11, local_806)),
            1999,
        );
        i_var3 = MessageBox16(
            0x34,
            CONCAT13(u_var12, CONCAT12(u_var11, local_806)),
            CONCAT13((u_var6 >> 8), CONCAT12(u_var6, u_var5)),
            ctx.g_h_window,
        );
        error_check_1000_17ce((u_var5 & 0xffff | u_var6 << 0x10));
        if (i_var3 == 6) {
            b_var4 = PostMessage16(0, 0xcb, 0x111, ctx.g_h_window);
            post_win_msg_1040_7b3c(u_var9, param_2_00, param_2, param_4, 1);
            local_6 = CONCAT22(ctx.dx_reg, b_var4);
        }
    } else {
        if (param_3 < 0x74) {
            if (param_3 == 0x6e) {
                (ctx._g_Struct112_a + 0xae) = 0x99;
                pu_var8 = pass1_1038_af40(ctx._g_Struct112_a, *(param_1 + 6), 2);
                unsafe {
                    pp_var1 = (*pu_var8 + 0x3c);
                }
                (**pp_var1)(&PTR_LOOP_1050_1038, pu_var8, (pu_var8 >> 0x10));
                SetFocus16((param_1 + 6));
                return;
            }
            if (0x6e < param_3) {
                // LAB_1040_09f9:
                post_win_msg_1040_7b3c(u_var9, u_var10, param_2, param_4, param_3);
                return;
            }
            if (param_3 == 0x2) {
                // LAB_1040_09b4:
                post_win_msg_1040_7b3c(u_var9, u_var10, 0, 0, 2);
                PostMessage16(0, 0xee, 0x111, ctx.g_h_window);
                return;
            }
            if (param_3 != 'd') {}
            // goto LAB_1040_09f9;
            PostMessage16(0, 100, 0x111, ctx.g_h_window);
            local_810._0_2_ = 0;
            // goto LAB_1040_0821;
        }
        if (param_3 != 0x74) {
            if (param_3 == 0xee) {}
            // goto LAB_1040_09b4;
            if (param_3 == 0x13d) {
                enable_win_1040_0acc(param_1, u_var10, 1);
                return;
            }
            // goto LAB_1040_09f9;
        }
        enable_win_1040_0acc(param_1, u_var10, 0);
        load_string_1010_84e0(
            ctx._g_struct_73_1050_14cc,
            (ctx._g_struct_73_1050_14cc >> 0x10),
            0x3ff,
            CONCAT13(u_var12, CONCAT12(u_var11, &local_406)),
            0x756,
        );
        load_string_1010_84e0(
            ctx._g_struct_73_1050_14cc,
            (ctx._g_struct_73_1050_14cc >> 0x10),
            0x3ff,
            CONCAT13(u_var12, CONCAT12(u_var11, local_806)),
            0x757,
        );
        i_var3 = MessageBox16(
            0x34,
            CONCAT13(u_var12, CONCAT12(u_var11, &local_406)),
            CONCAT13(u_var12, CONCAT12(u_var11, local_806)),
            ctx.g_h_window,
        );
        if (i_var3 == 6) {
            b_var4 = PostMessage16(0, 0x7a, 0x111, ctx.g_h_window);
            post_win_msg_1040_7b3c(u_var9, param_2_00, param_2, param_4, 1);
            local_6 = CONCAT22(ctx.dx_reg, b_var4);
            pa_var7 =
                process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(local_810._2_2_, 2));
            process_struct_1010_60fa(pa_var7);
        }
    }
    local_810._0_2_ = 1;
    // LAB_1040_0821:
    enable_win_1040_0acc(u_var9, param_2_00, local_810);
    return;
}

pub fn show_win_1040_0766(param_1: *mut Struct20) {
    let mut unaff_ss: u16;
    let ppVar1: *mut pass1_struct_1;
    let p_uvar2: *mut u16;
    let mut u_var3: u16;
    let pu_var4: *mut u16;
    let mut in_stack_0000ffde: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    ui1::win_gui_func_1040_78e2(param_1);
    _local_6 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(in_stack_0000ffde, 2));
    process_struct_1010_6118(_local_6);
    pu_var4 = &local_8;
    pu_var2 = &local_a;
    u_var3 = unaff_ss;
    ppVar1 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(pu_var2, 0x48));
    pass1_1008_3e94(
        (ppVar1 + 0xe),
        CONCAT22(unaff_ss, pu_var2),
        CONCAT22(u_var3, pu_var4),
    );
    u_var3 = (param_1 >> 0x10);
    ui1::move_window_1040_826c(param_1, u_var3, local_a + 0x8c, local_8 + 0xb9);
    ShowWindow16(5, (param_1 + 6));
    return;
}

pub fn enable_window_1040_060e(param_1: u32, param_2: u16) {
    let pi_var1: *mut u16;
    let h_wnd: HWND16;
    let mut unaff_ss: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;

    _local_8 = CONCAT22(unaff_ss, &stack0x000a);
    local_a = param_2;
    while (true) {
        pi_var1 = _local_8;
        if (local_a == 0) {
            break;
        }
        _local_8 = (_local_8 & 0xffff0000 | (local_8 + 2));
        local_a = local_a - 1;
        h_wnd = GetDlgItem16(unsafe { *pi_var1 }, (param_1 + 6));
        EnableWindow16(0, h_wnd);
    }
    return;
}

pub fn enable_window_1040_0558(param_1: u32, param_2: i32) -> LRESULT {
    let mut i_var1: i32;
    let l_param: LPARAM;
    let HVar2: HWND16;
    let mut i_var3: i32;
    let mut i_var4: i32;
    let mut u_var5: u16;
    let LVar6: LRESULT;
    let w_param: WPARAM16;
    let mut msg: u16;
    let id: u16;
    let mut local_4: u16;

    u_var5 = (param_1 >> 0x10);
    i_var4 = param_1;
    i_var1 = param_2 * 0xe;
    HVar2 = GetDlgItem16((i_var1 + 0x5c64), (i_var4 + 6));
    i_var3 = pass1_1010_659a((i_var4 + 0x8e), (i_var1 + 0x5c66));
    if ((i_var3 == 0) && ((i_var1 + 0x5c66) != 10)) {
        EnableWindow16(0, HVar2);
        HVar2 = (i_var4 + 6);
        id = (param_2 * 0xe + 0x5c68);
        u_var5 = 0x531;
    } else {
        EnableWindow16(1, HVar2);
        HVar2 = (i_var4 + 6);
        id = (param_2 * 0xe + 0x5c68);
        u_var5 = 0x649;
    }
    msg = 0xc;
    w_param = 0;
    l_param = load_string_1010_847e(
        ctx._g_struct_73_1050_14cc,
        (ctx._g_struct_73_1050_14cc >> 0x10),
        u_var5,
    );
    LVar6 = SendDlgItemMessage16(l_param, w_param, msg, id, HVar2);
    return LVar6;
}

pub fn destroy_win_1040_0170(param_1: i32, param_2: u16, param_3: u16, param_4: u32) {
    let mut u_var1: i32;
    let mut u_var2: u32;
    let mut i_var3: i32;
    let h_var4: HWND16;
    let ppVar5: *mut pass1_struct_2;
    let mut u_var6: u16;

    let mut unaff_si: u16;
    let mut u_var7: u16;
    let mut unaff_ss: u16;
    let pp_var8: *mut pass1_struct_1;
    let u_var9: u8;
    let u_var10: u8;
    let u_var11: u8;
    let u_var12: u8;
    let mut u_var13: u16;
    let mut local_12a: u16;
    let mut local_128: u16;
    let HStack294: HWND16;
    let mut uStack292: u32;
    let mut local_1e: u16;
    let mut local_1c: u16;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    local_4 = 8;
    local_6 = 0;
    match (param_2._2_2_) {
        0x167 => {
            enable_window_1040_060e(CONCAT22(param_2_00, param_1), 3, 0x168, 0x69, 0x16a);
            h_var4 = GetDlgItem16(0x16b, (param_1 + 6));
            EnableWindow16(1, h_var4);
            local_4 = 0
        }
        0x168 => {
            enable_window_1040_060e(CONCAT22(param_2_00, param_1), 3, 0x167, 0x69, 0x16a);
            h_var4 = GetDlgItem16(0x16b, (param_1 + 6));
            EnableWindow16(1, h_var4);
            local_4 = 1
        }
        0x169 => {
            enable_window_1040_060e(CONCAT22(param_2_00, param_1), 3, 0x167, 0x68, 0x16a);
            h_var4 = GetDlgItem16(0x16b, (param_1 + 6));
            EnableWindow16(1, h_var4);
            local_4 = 2
        }
        0x16a => {
            enable_window_1040_060e(CONCAT22(param_2_00, param_1), 3, 0x167, 0x68, 0x169);
            h_var4 = GetDlgItem16(0x16b, (param_1 + 6));
            EnableWindow16(1, h_var4);
            local_4 = 3
        }
        0x16b => {
            h_var4 = GetDlgItem16(0x16b, (param_1 + 6));
            u_var7 = offset;
            EnableWindow16(0, h_var4);
            if ((param_1 + 0x92) != 3) {
                u_var7 = &ctx.PTR_LOOP_1050_1008;
                mci_send_cmd_1008_5c5c(ctx._g_struct_ptr_1050_02a0, 0x1de);
            }
            if ((param_1 + 0x92) != 8) {
                i_var3 = (param_1 + 0x92) * 0xe;
                local_6 = (i_var3 + 0x5c6c);
                u_var7 = 0x1010;
                pass1_1010_6604((param_1 + 0x8e), (i_var3 + 0x5c66));
                (param_1 + 0x92) = 8;
            }
            local_8 = 0;
            while (local_8 < 4) {
                enable_window_1040_0558(param_1, param_2_00, local_8);
                local_8 = local_8 + 1;
            }
            // goto LAB_1040_04da;
        }
        0x16c => {
            h_var4 = GetDlgItem16(0x16d, (param_1 + 6));
            EnableWindow16(1, h_var4);
            local_4 = 5;
            (param_1 + 0x94) = 5;
            // goto LAB_1040_01bf;
        }
        0x16d => {
            h_var4 = GetDlgItem16(0x16d, (param_1 + 6));
            EnableWindow16(0, h_var4);
            mci_send_cmd_1008_5c5c(ctx._g_struct_ptr_1050_02a0, 0x1de);
            if ((param_1 + 0x94) != 8) {
                i_var3 = (param_1 + 0x94) * 0xe;
                local_6 = (i_var3 + 0x5c6c);
                pass1_1010_6604((param_1 + 0x8e), (i_var3 + 0x5c66));
                (param_1 + 0x94) = 8;
            }
            enable_window_1040_0558(param_1, param_2_00, 5);
            _local_c =
                process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(unaff_si, 0x39));
            local_10 = (_local_c + 0x20);
            u_var11 = SUB21(&local_16, 0);
            u_var12 = (&local_16 >> 8);
            u_var9 = SUB21(&local_18, 0);
            u_var10 = (&local_18 >> 8);
            u_var7 = (local_10 >> 0xf) + 0x200;
            u_var13 = unaff_ss;
            local_e = u_var7;
            local_8 = local_10;
            ppVar5 = pass1_1030_8344(
                ctx._g_bool_1050_5748,
                (ctx._g_bool_1050_5748 >> 0x10),
                CONCAT22(u_var7, local_10),
            );
            _local_14 = CONCAT22(u_var7, ppVar5);
            pass1_1030_2f1a(
                CONCAT22(u_var7, ppVar5),
                CONCAT22(unaff_ss, CONCAT11(u_var10, u_var9)),
                CONCAT22(u_var13, CONCAT11(u_var12, u_var11)),
            );
            local_16 = local_16 + (local_18 - local_16) / 2;
            local_1a = pass1_1030_2fac(_local_14);
            u_var2 = (param_1 + 0x96);
            u_var7 = 0x1018;
            win_fn_1018_6086(u_var2, (u_var2 >> 0x10), local_1a, local_16);
            // goto LAB_1040_04da;
        }
        0x16e => {
            _local_1e =
                process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(unaff_si, 0x39));
            local_1a = (_local_1e + 0x20);
            local_18 = LoadCursor16(0x7f02, 0);
            local_16 = SetCursor16(local_18);
            local_12a = (local_1a + 0x2000000 >> 0x10);
            pass1_1030_532e(CONCAT22(unaff_ss, &local_12a), local_1a + 0x2000000);
            pass1_1030_835a(ctx._g_bool_1050_5748, CONCAT22(unaff_ss, &local_12a));
            local_12a = (ctx._g_bool_1050_5748 >> 0x10);
            pass1_1030_838e(ctx._g_bool_1050_5748);
            local_12a = (ctx._g_bool_1050_5748 >> 0x10);
            pass1_1030_8334();
            local_12a = local_16;
            SetCursor16(local_16);
            local_128 = ctx.g_h_window;
            local_12a = 0x111;
            PostMessage16(0, 0x7e, 0x111, ctx.g_h_window);
            HStack294 = (param_1 + 6);
            local_128 = offset;
            u_var7 = offset;
            local_12a = 0x495;
            DestroyWindow16(HStack294);
            // goto LAB_1040_04da;
        }
        // default:
        _ => {
            post_win_msg_1040_7b3c();
            return;
        }
    }
    (param_1 + 0x92) = local_4;
    // LAB_1040_01bf:
    u_var7 = offset;
    // LAB_1040_04da:
    if (local_4 != 8) {
        uStack292 = (uStack292 & 0xffff0000 | *(param_1 + 6));
        HStack294 = (local_4 * 0xe + 0x5c68);
        local_12a = 0;
        local_128 = 0xc;
        u_var6 = local_4;
        load_string_1010_847e(
            ctx._g_struct_73_1050_14cc,
            (ctx._g_struct_73_1050_14cc >> 0x10),
            (local_4 * 0xe + 0x5c6a),
        );
        u_var7 = offset;
        SendDlgItemMessage16(
            CONCAT22(ctx.dx_reg, u_var6),
            local_12a,
            local_128,
            HStack294,
            uStack292,
        );
    }
    if (local_6 != 0) {
        uStack292 = CONCAT22(uStack292._2_2_, 2);
        local_128 = ctx._g_Struct372_1050_0ed0;
        HStack294 = (ctx._g_Struct372_1050_0ed0 >> 0x10);
        local_12a = u_var7;
        pp_var8 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, uStack292);
        u_var1 = (pp_var8 + 0x20);
        _local_1e = (_local_1e & 0xffff0000 | u_var1);
        if (u_var1 != 0) {
            uStack292 = (uStack292 & 0xffff0000 | ctx.g_h_window);
            HStack294 = 0x111;
            local_128 = local_6;
            local_12a = 0;
            PostMessage16(0, local_6, 0x111, ctx.g_h_window);
        }
    }
    return;
}

pub fn win_fn_1040_0000(param_1: *mut Struct20) {
    let mut i_var1: i32;
    let p_uvar2: *mut u16;
    let mut u_var3: u16;

    let ctx.dx_reg: *mut Struct199;
    let struct_a: *mut Struct199;
    let mut unaff_si: u16;
    let mut i_var4: i32;
    let mut u_var5: u16;
    let mut unaff_cs: u16;
    let unaff_ss: HWND16;
    let mut u_var6: u32;
    let pu_var7: Vec<u8>;
    let mut local_2a: u16;
    let mut local_28: u16;
    let mut local_26: u16;
    let mut local_24: u16;
    let mut local_22: u16;
    let mut local_20: u16;
    let mut local_1e: u16;
    let mut local_1c: u16;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    // Segment:    9
    // Offset:     0006f820
    // Length:     d974
    // Min Alloc:  d974
    // Flags:      0d50
    //     Code
    //     Moveable
    //     Preload
    //     Impure (Non-shareable)
    //
    ui1::win_gui_func_1040_78e2(param_1);
    local_4 = 8;
    local_a = 0;
    struct_a = ctx.dx_reg;
    while (
        i_var4 = param_1,
        u_var5 = (param_1 >> 0x10),
        local_a < local_4,
    ) {
        i_var1 = local_a * 0xe;
        local_24 = (i_var1 + 0x5c60);
        local_22 = (i_var1 + 0x5c62);
        local_20 = 1;
        local_1e = 1;
        u_var3 = (i_var4 + 6);
        pu_var2 = &local_24;
        MapDialogRect16(CONCAT22(pu_var2, unaff_cs), unaff_ss);
        unaff_cs = 0x1000;
        process_struct_1000_179c(0x42, struct_a);
        if ((struct_a | pu_var2) == 0) {
            u_var6 = 0;
        } else {
            unaff_cs = SUB42(&ctx.PTR_LOOP_1050_1008, 0);
            u_var6 = ui1::win_fn_1008_3bd6(
                pu_var2,
                struct_a,
                1,
                CONCAT22(local_24, local_22),
                0x1030104,
                CONCAT22((i_var1 + 0x5c64), 0x102),
                CONCAT22(u_var3, (i_var4 + 6)),
            );
        }
        _local_8 = u_var6;
        enable_window_1040_0558(i_var4, u_var5, local_a);
        local_a = local_a + 1;
        struct_a = ctx.dx_reg;
    }
    ui1::move_window_1040_826c(i_var4, u_var5, 0xffff, 0xffff);
    _local_12 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(unaff_si, 0x48));
    u_var3 = (_local_12 >> 0x10);
    local_c = (_local_12 + 10);
    local_e = (_local_12 + 0xc);
    GetWindowRect16(CONCAT22(&local_1a, 0x1010), unaff_ss);
    local_1c = local_16 - local_1a;
    local_1a = (local_c / 2 - local_1c) - 3;
    if (local_1a < 0) {
        local_1a = 0;
    }
    SetWindowPos16(0x41, 0, 0, local_18, local_1a, 0, (i_var4 + 6));
    pu_var7 = pass1_1038_af40(ctx._g_Struct112_a, *(i_var4 + 6), 0x17);
    (i_var4 + 0x96) = pu_var7;
    (i_var4 + 0x98) = (pu_var7 >> 0x10);
    u_var3 = mci_send_command_1008_5c7c(ctx._g_struct_ptr_1050_02a0, 0x9e0001);
    (i_var4 + 0x8c) = u_var3;
    return;
}

pub fn win_fn_1038_ec16(param_1: *mut Struct20) {
    let mut u_var1: u32;
    let mut u_var2: u16;
    let pu_var3: *mut u16;
    let struct_a: *mut Struct199;
    let mut i_var4: i32;
    let mut i_var5: i32;
    let mut unaff_si: u16;
    let mut u_var6: u16;
    let unaff_ss: HWND16;
    let mut u_var7: u32;
    let mut local_26: u16;
    let mut local_24: u16;
    let mut local_22: u16;
    let mut local_20: u16;
    let mut local_1e: u16;
    let mut local_1c: u16;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    ui1::win_gui_func_1040_78e2(param_1);
    _local_6 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(unaff_si, 0x2b));
    ctx.g_u16_ptr_1050_5f2e = (_local_6 >> 0x10);
    local_8 = return3_1010_0892();
    if (ctx.__g_Struct94_ptr_1 == 0) {
        _g_Struct94_ptr_1 = struct_fn_1000_160a();
    } else {
    }
    _local_1a = CONCAT22(ctx.g_u16_ptr_1050_5f2e, _g_Struct94_ptr_1);
    u_var2 = alloc_mem_1000_1708(
        (local_8 + 2) * 4,
        0,
        1,
        _g_Struct94_ptr_1,
        ctx.g_u16_ptr_1050_5f2e,
    );
    u_var6 = (param_1 >> 0x10);
    i_var4 = param_1;
    (i_var4 + 0x8e) = u_var2;
    (i_var4 + 0x90) = ctx.g_u16_ptr_1050_5f2e;
    local_a = 1;
    while (local_a <= local_8) {
        _local_1a = pass1_1010_0932(_local_6, (_local_6 >> 0x10), local_a);
        struct_a = (_local_1a >> 0x10);
        local_22 = *_local_1a;
        local_20 = (_local_1a + 2);
        local_1e = 1;
        local_1c = 1;
        u_var2 = (i_var4 + 6);
        pu_var3 = &local_22;
        MapDialogRect16(CONCAT22(pu_var3, 0x1010), unaff_ss);
        process_struct_1000_179c(0x42, struct_a);
        if ((struct_a | pu_var3) == 0) {
            u_var1 = (i_var4 + 0x8e);
            (u_var1 + local_a * 4) = 0;
        } else {
            u_var7 = ui1::win_fn_1008_3bd6(
                pu_var3,
                struct_a,
                0,
                CONCAT22(local_22, local_20),
                0x1000101,
                CONCAT22((_local_1a + 4), 0xff),
                CONCAT22(u_var2, (i_var4 + 6)),
            );
            u_var1 = (i_var4 + 0x8e);
            u_var2 = (u_var1 >> 0x10);
            i_var5 = u_var1;
            (i_var5 + local_a * 4) = u_var7;
            (i_var5 + local_a * 4 + 2) = (u_var7 >> 0x10);
        }
        u_var1 = (i_var4 + 0x8e);
        u_var2 = (u_var1 >> 0x10);
        i_var5 = u_var1;
        if ((i_var5 + local_a * 4) != 0) {
            u_var1 = (i_var5 + local_a * 4);
            ui1::enable_window_1040_9234(u_var1, (u_var1 >> 0x10), (_local_1a + 6));
        }
        local_a = local_a + 1;
    }
    ui1::move_window_1040_826c(param_1, u_var6, 0xff, 0xffff);
    ShowWindow16(5, (i_var4 + 6));
    return;
}

pub fn show_window_1038_ea18(param_1: *mut Struct20) {
    let i_var1: u16;
    let mut i_var2: i32;
    let mut i_var3: i32;
    let mut u_var4: u16;
    let unaff_ss: HWND16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    ui1::win_gui_func_1040_78e2(param_1);
    u_var4 = (param_1 >> 0x10);
    i_var3 = param_1;
    _local_6 = pass1_1010_375e((i_var3 + 0x8e));
    local_8 = GetDlgItem16(0xfa5, (i_var3 + 6));
    SendMessage16(_local_6, 0, 0xc, local_8);
    GetWindowRect16(CONCAT22(&local_10, 0x1538), unaff_ss);
    i_var1 = GetSystemMetrics16(4);
    i_var2 = i_var1 + local_e + 5;
    ui1::move_window_1040_826c(i_var3, u_var4, i_var2, i_var2);
    ShowWindow16(5, (i_var3 + 6));
    return;
}

pub fn destroy_win_1038_eaa2(param_1: u32, param_2: i32) {
    let h_wnd: HWND16;
    let mut i_var1: i32;
    let mut u_var2: u16;
    let mut unaff_ss: u16;
    let mut local_56: u16;
    let local_54: [HWND16; 41];

    i_var1 = param_1;
    u_var2 = (param_1 >> 0x10);
    if (param_2 != 0) {
        h_wnd = GetDlgItem16(0xfa5, (i_var1 + 6));
        SendMessage16(CONCAT22(unaff_ss, local_54), 0x50, 0xd, h_wnd);
        pass1_1010_3770((i_var1 + 0x8e), CONCAT22(unaff_ss, local_54));
        PostMessage16(0, 0xfb, 0x111, (i_var1 + 8));
    }
    local_54[0] = (i_var1 + 6);
    DestroyWindow16(local_54[0]);
    return;
}

pub fn show_destroy_win_1038_e71c(param_1: *mut Struct20) {


    let mut i_var1: i32;
    let mut u_var2: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    ui1::win_gui_func_1040_78e2(param_1);
    u_var2 = (param_1 >> 0x10);
    i_var1 = param_1;
    string_fn_1010_2c34((i_var1 + 0x8e));
    _local_6 = CONCAT22(ctx.dx_reg, in_ax);
    copy_string_1000_3d3e(
        (param_1 & 0xffff0000 | (i_var1 + 0x10)),
        CONCAT22(ctx.dx_reg, in_ax),
    );
    error_check_1000_17ce(_local_6);
    ui1::move_window_1040_826c(param_1, 0xffffffff);
    ShowWindow16(5, (i_var1 + 6));
    (i_var1 + 0x92) = 1;
    process_win_msg_1008_9510();
    DestroyWindow16((i_var1 + 6));
    return;
}

pub fn dialog_button_checked_1038_e7a0(param_1: u32, param_2: i32) {
    let mut u_var1: u32;
    let mut u_var2: u16;
    let mut i_var3: i32;
    let mut u_var4: u16;

    i_var3 = param_1;
    u_var4 = (param_1 >> 0x10);
    if (param_2 == 0) {
        u_var1 = (i_var3 + 0x8e);
        (u_var1 + 0x10) = 1;
        u_var1 = (i_var3 + 0x8e);
        (u_var1 + 10) = 0;
        u_var1 = (i_var3 + 0x8e);
        (u_var1 + 0xc) = 0;
        u_var1 = (i_var3 + 0x8e);
        (u_var1 + 0xe) = 0;
    } else {
        u_var2 = IsDlgButtonChecked16(0x1827, (i_var3 + 6));
        if (u_var2 == 0) {
            u_var2 = IsDlgButtonChecked16(0x1828, (i_var3 + 6));
            if (u_var2 == 0) {
                u_var1 = (i_var3 + 0x8e);
                (u_var1 + 10) = 0;
            } else {
                u_var1 = (i_var3 + 0x8e);
                (u_var1 + 10) = 2;
            }
        } else {
            u_var1 = (i_var3 + 0x8e);
            (u_var1 + 10) = 1;
        }
        u_var2 = IsDlgButtonChecked16(s_vrpal_bmp_1050_183a, (i_var3 + 6));
        if (u_var2 == 0) {
            u_var2 = IsDlgButtonChecked16((s_vrpal_bmp_1050_183a + 1), (i_var3 + 6));
            if (u_var2 == 0) {
                u_var1 = (i_var3 + 0x8e);
                (u_var1 + 0xc) = 0;
            } else {
                u_var1 = (i_var3 + 0x8e);
                (u_var1 + 0xc) = 2;
            }
        } else {
            u_var1 = (i_var3 + 0x8e);
            (u_var1 + 0xc) = 1;
        }
        u_var2 = IsDlgButtonChecked16((s_vrpal_bmp_1050_183a + 2), (i_var3 + 6));
        if (u_var2 == 0) {
            u_var2 = IsDlgButtonChecked16((s_vrpal_bmp_1050_183a + 3), (i_var3 + 6));
            if (u_var2 == 0) {
                u_var1 = (i_var3 + 0x8e);
                (u_var1 + 0xe) = 0;
            } else {
                u_var1 = (i_var3 + 0x8e);
                (u_var1 + 0xe) = 2;
            }
        } else {
            u_var1 = (i_var3 + 0x8e);
            (u_var1 + 0xe) = 1;
        }
        u_var1 = (i_var3 + 0x8e);
        (u_var1 + 0x10) = 0;
    }
    (i_var3 + 0x92) = 0;
    return;
}

pub fn win_fn_1038_e348(param_1: *mut Struct20) {
    let mut u_var1: u32;
    let mut u_var2: u16;
    let pu_var3: *mut u16;
    let struct_a: *mut Struct199;
    let mut i_var4: i32;
    let mut i_var5: i32;
    let mut unaff_si: u16;
    let mut u_var6: u16;
    let unaff_ss: HWND16;
    let mut u_var7: u32;
    let mut local_26: u16;
    let mut local_24: u16;
    let mut local_22: u16;
    let mut local_20: u16;
    let mut local_1e: u16;
    let mut local_1c: u16;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    ui1::win_gui_func_1040_78e2(param_1);
    _local_6 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(unaff_si, 0x2b));
    ctx.g_u16_ptr_1050_5f2e = (_local_6 >> 0x10);
    local_8 = return3_1010_088c();
    if (ctx.__g_Struct94_ptr_1 == 0) {
        _g_Struct94_ptr_1 = struct_fn_1000_160a();
    } else {
    }
    _local_1a = CONCAT22(ctx.g_u16_ptr_1050_5f2e, _g_Struct94_ptr_1);
    u_var2 = alloc_mem_1000_1708(
        (local_8 + 2) * 4,
        0,
        1,
        _g_Struct94_ptr_1,
        ctx.g_u16_ptr_1050_5f2e,
    );
    u_var6 = (param_1 >> 0x10);
    i_var4 = param_1;
    (i_var4 + 0x8e) = u_var2;
    (i_var4 + 0x90) = ctx.g_u16_ptr_1050_5f2e;
    local_a = 1;
    while (local_a <= local_8) {
        _local_1a = pass1_1010_091e(_local_6, (_local_6 >> 0x10), local_a);
        struct_a = (_local_1a >> 0x10);
        local_22 = *_local_1a;
        local_20 = (_local_1a + 2);
        local_1e = 1;
        local_1c = 1;
        u_var2 = (i_var4 + 6);
        pu_var3 = &local_22;
        MapDialogRect16(CONCAT22(pu_var3, 0x1010), unaff_ss);
        process_struct_1000_179c(0x42, struct_a);
        if ((struct_a | pu_var3) == 0) {
            u_var1 = (i_var4 + 0x8e);
            (u_var1 + local_a * 4) = 0;
        } else {
            u_var7 = ui1::win_fn_1008_3bd6(
                pu_var3,
                struct_a,
                0,
                CONCAT22(local_22, local_20),
                0x1000101,
                CONCAT22((_local_1a + 4), 0xff),
                CONCAT22(u_var2, (i_var4 + 6)),
            );
            u_var1 = (i_var4 + 0x8e);
            u_var2 = (u_var1 >> 0x10);
            i_var5 = u_var1;
            (i_var5 + local_a * 4) = u_var7;
            (i_var5 + local_a * 4 + 2) = (u_var7 >> 0x10);
        }
        u_var1 = (i_var4 + 0x8e);
        u_var2 = (u_var1 >> 0x10);
        i_var5 = u_var1;
        if ((i_var5 + local_a * 4) != 0) {
            u_var1 = (i_var5 + local_a * 4);
            ui1::enable_window_1040_9234(u_var1, (u_var1 >> 0x10), (_local_1a + 6));
        }
        local_a = local_a + 1;
    }
    ui1::move_window_1040_826c(param_1, u_var6, 0xff, 0xffff);
    ShowWindow16(5, (i_var4 + 6));
    return;
}

pub fn check_dlg_btn_1038_e1dc(in_struct_1: *mut Struct49, param_2: u16) {
    let mut u_var1: u16;
    let local_bx_7: *mut Struct49;
    let mut u_var2: u16;
    let mut w_param: u32;

    local_bx_7 = in_struct_1;
    u_var2 = (in_struct_1 >> 0x10);
    if (param_2 != 0) {
        u_var1 = IsDlgButtonChecked16(0x1807, local_bx_7.h_win);
        if (u_var1 == 0) {
            u_var1 = IsDlgButtonChecked16(0x1806, local_bx_7.h_win);
            if (u_var1 == 0) {}
            // goto LAB_1038_e229;
            w_param = 0x1110130;
        } else {
            w_param = 0x111012f;
        }
        SendMessage16(0, w_param, w_param._2_2_, ctx.g_h_window);
    }
    // LAB_1038_e229:
    DestroyWindow16(local_bx_7.h_win);
    return;
}

pub fn gui_window_func_1038_e19a(param_1: *mut Struct20) {
    let mut i_var1: i32;
    let mut u_var2: u16;
    let local_6: *mut Struct24;

    ui1::win_gui_func_1040_78e2(param_1);
    u_var2 = (param_1 >> 0x10);
    i_var1 = param_1;
    CheckRadioButton16(0x1807, 0x1807, 0x1807, (i_var1 + 6));
    ui1::move_window_1040_826c(i_var1, u_var2, 200, 200);
    ShowWindow16(5, (i_var1 + 6));
    return;
}

pub fn pass1_1038_e03e(param_1: Vec<u8>) {
    let mut u_var1: u32;
    let mut u_var2: u16;
    let mut i_var3: i32;
    let mut u_var4: u16;
    let mut u_var5: u16;
    let mut u_var6: u32;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    u_var4 = (param_1 >> 0x10);
    u_var2 = return_10_1010_0886();
    local_6 = 1;
    while (local_6 <= u_var2) {
        u_var1 = (param_1 + 0x92);
        u_var6 = pass1_1010_08e2(u_var1, (u_var1 >> 0x10), local_6);
        u_var1 = (param_1 + 0x96);
        u_var5 = (u_var1 >> 0x10);
        i_var3 = u_var1;
        if ((i_var3 + local_6 * 4) != 0) {
            u_var1 = (i_var3 + local_6 * 4);
            ui1::enable_window_1040_9234(u_var1, (u_var1 >> 0x10), (u_var6 + 6));
        }
        local_6 = local_6 + 1;
    }
    return;
}

pub fn win_fn_1038_d8ae(param_1: *mut Struct20) {
    let mut u_var1: u32;
    let mut u_var2: u16;
    let pu_var3: *mut u16;
    let ctx.dx_reg: *mut u16;
    let struct_a: *mut Struct199;
    let mut i_var4: i32;
    let mut i_var5: i32;
    let mut u_var6: u16;
    let unaff_ss: HWND16;
    let mut u_var7: u32;
    let mut in_stack_0000ffd0: u16;
    let mut local_2a: u16;
    let mut local_28: u16;
    let mut local_26: u16;
    let mut local_24: u16;
    let mut local_22: u16;
    let mut local_20: u16;
    let mut local_1e: u16;
    let mut local_1c: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    local_4 = LoadCursor16(0x7f02, 0);
    local_6 = SetCursor16(local_4);
    ui1::win_gui_func_1040_78e2(param_1);
    u_var6 = (param_1 >> 0x10);
    i_var4 = param_1;
    ctx.g_u16_ptr_1050_5f2e = ctx.dx_reg;
    local_8 = return_10_1010_0886();
    if (ctx.__g_Struct94_ptr_1 == 0) {
        _g_Struct94_ptr_1 = struct_fn_1000_160a();
    } else {
    }
    _local_1e = CONCAT22(ctx.g_u16_ptr_1050_5f2e, _g_Struct94_ptr_1);
    u_var2 = alloc_mem_1000_1708(
        (local_8 + 2) * 4,
        0,
        1,
        _g_Struct94_ptr_1,
        ctx.g_u16_ptr_1050_5f2e,
    );
    (i_var4 + 0x96) = u_var2;
    (i_var4 + 0x98) = ctx.g_u16_ptr_1050_5f2e;
    local_a = 1;
    while (local_a <= local_8) {
        u_var1 = (i_var4 + 0x92);
        _local_1e = pass1_1010_08e2(u_var1, (u_var1 >> 0x10), local_a);
        struct_a = (_local_1e >> 0x10);
        local_26 = *_local_1e;
        local_24 = (_local_1e + 2);
        local_22 = 1;
        local_20 = 1;
        u_var2 = (i_var4 + 6);
        pu_var3 = &local_26;
        MapDialogRect16(CONCAT22(pu_var3, 0x1010), unaff_ss);
        process_struct_1000_179c(0x42, struct_a);
        if ((struct_a | pu_var3) == 0) {
            u_var1 = (i_var4 + 0x96);
            (u_var1 + local_a * 4) = 0;
        } else {
            u_var7 = ui1::win_fn_1008_3bd6(
                pu_var3,
                struct_a,
                0,
                CONCAT22(local_26, local_24),
                0x1000101,
                CONCAT22((_local_1e + 4), 0xff),
                CONCAT22(u_var2, (i_var4 + 6)),
            );
            u_var1 = (i_var4 + 0x96);
            u_var2 = (u_var1 >> 0x10);
            i_var5 = u_var1;
            (i_var5 + local_a * 4) = u_var7;
            (i_var5 + local_a * 4 + 2) = (u_var7 >> 0x10);
        }
        u_var1 = (i_var4 + 0x96);
        u_var2 = (u_var1 >> 0x10);
        i_var5 = u_var1;
        if ((i_var5 + local_a * 4) != 0) {
            u_var1 = (i_var5 + local_a * 4);
            (u_var1 + 0x3e) = 1;
            u_var1 = (i_var4 + 0x96);
            u_var1 = (u_var1 + local_a * 4);
            ui1::enable_window_1040_9234(u_var1, (u_var1 >> 0x10), (_local_1e + 6));
        }
        local_a = local_a + 1;
    }
    _local_e = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(in_stack_0000ffd0, 2));
    SetWindowText16((_local_e + 0x68), (i_var4 + 6));
    ShowWindow16(5, (i_var4 + 6));
    SetCursor16(local_6);
    return;
}

pub fn win_fn_1038_d400(param_1: i32, param_2: u16, param_3: u16, param_4: u32) {
    let HVar1: HWND16;
    let BVar2: bool;




    let mut unaff_ss: u16;
    let ppVar3: *mut pass1_struct_1;
    let WVar4: WPARAM16;
    let mut u_var5: u16;
    let i32_var6: u16;
    let mut u_var7: u16;
    let mut in_stack_0000ffe6: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: [u8; 4];
    let mut local_8: u16;
    let mut local_6: u32;

    local_6 = 0;
    local_8 = 0;
    match (param_2._2_2_) {
        0x145 => {
            HVar1 = GetDlgItem16(0x146, (param_1 + 6));
            EnableWindow16(1, HVar1);
            local_6 = 0x13f0647;
            u_var7 = 0x1f1;
            // goto LAB_1038_d490;
        }
        0x146 => {
            local_6 = 0x1400648;
            pass1_1008_941a(CONCAT22(unaff_ss, local_c), 1, 0xc4);
            mci_send_command_1008_5c9e(ctx._g_struct_ptr_1050_02a0, CONCAT22(unaff_ss, local_c));
            u_var7 = 0x86;
            ppVar3 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, 0x860009);
            pass1_1010_6604(ppVar3, u_var7);
            HVar1 = GetDlgItem16(0x145, (param_1 + 6));
            BVar2 = EnableWindow16(0, HVar1);
            HVar1 = (param_1 + 6);
            u_var5 = 0xc;
            i32_var6 = 0x13f;
            WVar4 = 0;
            load_string_1010_847e(
                ctx._g_struct_73_1050_14cc,
                (ctx._g_struct_73_1050_14cc >> 0x10),
                0x649,
            );
            SendDlgItemMessage16(
                CONCAT22(ctx.dx_reg, BVar2),
                WVar4,
                u_var5,
                i32_var6,
                HVar1,
            );
            HVar1 = GetDlgItem16(0x146, (param_1 + 6));
            BVar2 = EnableWindow16(0, HVar1);
            pass1_1010_659a(ppVar3, 0x86);
            if (BVar2 == 0) {
                HVar1 = GetDlgItem16(0x14a, (param_1 + 6));
                BVar2 = EnableWindow16(0, HVar1);
                HVar1 = (param_1 + 6);
                u_var5 = 0xc;
                i32_var6 = 0x144;
                WVar4 = 0;
                load_string_1010_847e(
                    ctx._g_struct_73_1050_14cc,
                    (ctx._g_struct_73_1050_14cc >> 0x10),
                    0x531,
                );
                SendDlgItemMessage16(
                    CONCAT22(ctx.dx_reg, BVar2),
                    WVar4,
                    u_var5,
                    i32_var6,
                    HVar1,
                );
            }
            ppVar3 = process_struct_1010_20ba(
                ctx._g_Struct372_1050_0ed0,
                CONCAT22(in_stack_0000ffe6, 2),
            );
            if ((ppVar3 + 0x20) != 0) {
                PostMessage16(0, 0xaf, 0x111, ctx.g_h_window);
            }
        }
        0x147 => {
            HVar1 = GetDlgItem16(0x148, (param_1 + 6));
            EnableWindow16(1, HVar1);
            local_6 = 0x1410647;
            u_var7 = 0x1f5;
            // goto LAB_1038_d490;
        }
        0x148 => {
            HVar1 = GetDlgItem16(0x149, (param_1 + 6));
            EnableWindow16(1, HVar1);
            local_6 = 0x1420647;
            u_var7 = 0x1f2;
            // LAB_1038_d490:
            mci_send_cmd_1008_5c5c(ctx._g_struct_ptr_1050_02a0, u_var7)
        }
        0x149 => {
            local_6 = 0x1430648;
            PostMessage16(0, 0xb8, 0x111, ctx.g_h_window);
            DestroyWindow16((param_1 + 6))
        }
        0x14a => {
            HVar1 = GetDlgItem16(0x145, (param_1 + 6));
            BVar2 = EnableWindow16(1, HVar1);
            HVar1 = (param_1 + 6);
            u_var5 = 0xc;
            i32_var6 = 0x140;
            WVar4 = 0;
            load_string_1010_847e(
                ctx._g_struct_73_1050_14cc,
                (ctx._g_struct_73_1050_14cc >> 0x10),
                0x649,
            );
            SendDlgItemMessage16(CONCAT22(ctx.dx_reg, BVar2), WVar4, u_var5, i32_var6, HVar1)
        }
        0x14b => {
            HVar1 = GetDlgItem16(0x147, (param_1 + 6));
            EnableWindow16(1, HVar1)
        }
        _ => {
            post_win_msg_1040_7b3c(param_1, param_2_00, param_3, param_2, param_2._2_2_);
            return;
        }
    }
    if (((local_6._2_2_ != 0) && (local_6 != 0)) && (BVar2 = IsWindow16((param_1 + 6)), BVar2 != 0))
    {
        HVar1 = (param_1 + 6);
        WVar4 = 0;
        u_var5 = 0xc;
        load_string_1010_847e(
            ctx._g_struct_73_1050_14cc,
            (ctx._g_struct_73_1050_14cc >> 0x10),
            local_6,
        );
        SendDlgItemMessage16(
            CONCAT22(ctx.dx_reg, BVar2),
            WVar4,
            u_var5,
            local_6._2_2_,
            HVar1,
        );
    }
    if (local_8 != 0) {
        PostMessage16(0, local_8, 0x111, ctx.g_h_window);
    }
    return;
}

pub fn win_fn_1038_d2a2(param_1: *mut Struct20) {
    let l_param: LPARAM;
    let pu_var1: *mut u16;
    let HVar2: HWND16;

    let struct_a: *mut Struct199;
    let mut i_var3: i32;
    let mut i_var4: i32;
    let mut u_var5: u16;
    let mut u_var6: u16;
    let unaff_ss: HWND16;
    let w_param: WPARAM16;
    let mut msg: u16;
    let id: u16;
    let mut u_var7: u16;
    let mut u_var8: u16;
    let mut local_22: u16;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    u_var6 = SUB42(&ctx.PTR_LOOP_1050_1040, 0);
    ui1::win_gui_func_1040_78e2(param_1);
    local_4 = 7;
    local_a = 0;
    struct_a = ctx.dx_reg;
    while (
        i_var4 = param_1,
        u_var5 = (param_1 >> 0x10),
        local_a < local_4,
    ) {
        i_var3 = local_a * 0xc;
        local_16 = (i_var3 + 0x5c0c);
        local_14 = (i_var3 + 0x5c0e);
        local_12 = 1;
        local_10 = 1;
        u_var7 = (i_var4 + 6);
        pu_var1 = &local_16;
        MapDialogRect16(CONCAT22(pu_var1, u_var6), unaff_ss);
        u_var6 = 0x1000;
        process_struct_1000_179c(0x42, struct_a);
        if ((struct_a | pu_var1) == 0) {
            _local_8 = 0;
        } else {
            u_var6 = SUB42(&ctx.PTR_LOOP_1050_1008, 0);
            _local_8 = ui1::win_fn_1008_3bd6(
                pu_var1,
                struct_a,
                1,
                CONCAT22(local_16, local_14),
                0x1030104,
                CONCAT22((i_var3 + 0x5c10), 0x102),
                CONCAT22(u_var7, (i_var4 + 6)),
            );
        }
        struct_a = (_local_8 >> 0x10);
        if ((local_a * 0xc + 0x5c12) == 0) {
            u_var6 = SUB42(offset, 0);
            EnableWindow16(0, (_local_8 + 0x18));
        }
        local_a = local_a + 1;
    }
    u_var8 = 0x86;
    _local_e = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, 0x860009);
    i_var3 = pass1_1010_659a(_local_e, u_var8);
    if (i_var3 == 0) {
        HVar2 = GetDlgItem16(0x14a, (i_var4 + 6));
        EnableWindow16(0, HVar2);
        HVar2 = (i_var4 + 6);
        msg = 0xc;
        id = 0x144;
        w_param = 0;
        l_param = load_string_1010_847e(
            ctx._g_struct_73_1050_14cc,
            (ctx._g_struct_73_1050_14cc >> 0x10),
            0x531,
        );
        SendDlgItemMessage16(l_param, w_param, msg, id, HVar2);
    }
    ui1::move_window_1040_826c(i_var4, u_var5, 0xffff, 0xffff);
    ShowWindow16(5, (i_var4 + 6));
    u_var6 = mci_send_command_1008_5c7c(ctx._g_struct_ptr_1050_02a0, 0x9a0001);
    (i_var4 + 0x8c) = u_var6;
    return;
}

pub fn dialog_button_checked_1038_cdd6(param_1: u32, param_2: i32) {
    let mut u_var1: u32;
    let mut u_var2: u16;
    let local_bx_8: *mut Struct70;
    let mut u_var3: u16;

    local_bx_8 = param_1;
    u_var3 = (param_1 >> 0x10);
    if (param_2 == 0) {
        u_var1 = local_bx_8.field_0x8e;
        (u_var1 + 10) = 0;
    } else {
        u_var2 = IsDlgButtonChecked16(0x182e, local_bx_8.field_0x6);
        if (u_var2 == 0) {
            u_var2 = IsDlgButtonChecked16(0x182f, local_bx_8.field_0x6);
            if (u_var2 == 0) {
                u_var2 = IsDlgButtonChecked16(0x1829, local_bx_8.field_0x6);
                if (u_var2 == 0) {
                    u_var2 = IsDlgButtonChecked16(0x182a, local_bx_8.field_0x6);
                    if (u_var2 == 0) {
                        u_var2 = IsDlgButtonChecked16(0x182c, local_bx_8.field_0x6);
                        if (u_var2 == 0) {
                            u_var2 = IsDlgButtonChecked16(0x182d, local_bx_8.field_0x6);
                            if (u_var2 != 0) {
                                u_var1 = local_bx_8.field_0x8e;
                                (u_var1 + 10) = 7;
                            }
                        } else {
                            u_var1 = local_bx_8.field_0x8e;
                            (u_var1 + 10) = 6;
                        }
                    } else {
                        u_var1 = local_bx_8.field_0x8e;
                        (u_var1 + 10) = 4;
                    }
                } else {
                    u_var1 = local_bx_8.field_0x8e;
                    (u_var1 + 10) = 3;
                }
            } else {
                u_var1 = local_bx_8.field_0x8e;
                (u_var1 + 10) = 2;
            }
        } else {
            u_var1 = local_bx_8.field_0x8e;
            (u_var1 + 10) = 1;
        }
    }
    local_bx_8.field_0x92 = 0;
    return;
}

pub fn win_func_1038_cd88(param_1: *mut Struct20) {
    let mut i_var1: i32;
    let mut u_var2: u16;

    ui1::win_gui_func_1040_78e2(param_1);
    ui1::move_window_1040_826c(param_1, 0xffffffff);
    u_var2 = (param_1 >> 0x10);
    i_var1 = param_1;
    ShowWindow16(5, (i_var1 + 6));
    (i_var1 + 0x92) = 1;
    process_win_msg_1008_9510();
    DestroyWindow16((i_var1 + 6));
    return;
}

pub fn show_window_1038_cb5c(param_1: *mut Struct20) {
    let mut u_var1: u32;
    let mut u_var2: u16;
    let struct_a: *mut Struct199;
    let mut i_var3: i32;
    let mut u_var4: u16;
    let pu_var5: *mut u16;
    let pu_var6: *mut u16;
    let mut in_stack_0000fff2: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    ui1::win_gui_func_1040_78e2(param_1);
    u_var4 = (param_1 >> 0x10);
    i_var3 = param_1;
    u_var2 = ret_5_1008_eb6e();
    local_a = 0;
    while (local_a < u_var2) {
        u_var1 = (i_var3 + 0x8e);
        pu_var5 = pass1_1008_eb5c(u_var1, (u_var1 >> 0x10), local_a);
        struct_a = (pu_var5 >> 0x10);
        pu_var6 = pu_var5;
        process_struct_1000_179c(0x42, struct_a);
        if (pu_var6 != 0x0) {
            ui1::win_fn_1008_3bd6(
                pu_var6,
                (pu_var6 >> 0x10),
                0,
                unsafe { CONCAT22(*pu_var5, (pu_var5 + 2)) },
                0x1000101,
                CONCAT22((pu_var5 + 4), 0xff),
                CONCAT22(in_stack_0000fff2, (i_var3 + 6)),
            );
        }
        local_a = local_a + 1;
    }
    mci_send_command_1008_5c7c(ctx._g_struct_ptr_1050_02a0, 0x90001);
    ShowWindow16(5, (i_var3 + 6));
    return;
}

pub fn destroy_win_1038_c95e(param_1: u32, param_2: i32) {
    let mut u_var1: u32;
    let mut u_var2: u16;
    let mut i_var3: i32;
    let mut u_var4: u16;

    i_var3 = param_1;
    u_var4 = (param_1 >> 0x10);
    if (param_2 == 0) {
        u_var1 = (i_var3 + 0x8e);
        (u_var1 + 10) = 0;
    } else {
        u_var2 = IsDlgButtonChecked16(0xfac, (i_var3 + 6));
        if (u_var2 == 0) {
            u_var2 = IsDlgButtonChecked16(0xfad, (i_var3 + 6));
            if (u_var2 == 0) {
                u_var2 = IsDlgButtonChecked16(0xfae, (i_var3 + 6));
                if (u_var2 == 0) {
                    u_var2 = IsDlgButtonChecked16(0xfaf, (i_var3 + 6));
                    if (u_var2 == 0) {
                        u_var2 = IsDlgButtonChecked16(0xfb0, (i_var3 + 6));
                        if (u_var2 != 0) {
                            u_var1 = (i_var3 + 0x8e);
                            (u_var1 + 10) = 5;
                        }
                    } else {
                        u_var1 = (i_var3 + 0x8e);
                        (u_var1 + 10) = 4;
                    }
                } else {
                    u_var1 = (i_var3 + 0x8e);
                    (u_var1 + 10) = 3;
                }
            } else {
                u_var1 = (i_var3 + 0x8e);
                (u_var1 + 10) = 2;
            }
        } else {
            u_var1 = (i_var3 + 0x8e);
            (u_var1 + 10) = 1;
        }
    }
    DestroyWindow16((i_var3 + 6));
    PTR_LOOP_1050_5b80 = 0x0;
    return;
}

pub fn gui_window_func_1038_c89c(param_1: *mut Struct22) {
    let mut u_var1: u32;
    let h_wnd: HWND16;
    let mut i_var2: i32;
    let mut u_var3: u16;
    let local_10: *mut Struct22;
    let mut h_wnd_2: u16;
    let temp_5f4a343e5a: *mut Struct23;

    ui1::win_gui_func_1040_78e2(param_1);
    u_var3 = (param_1 >> 0x10);
    i_var2 = param_1;
    CheckRadioButton16(0xfac, 0xfad, 0xfac, (i_var2 + 6));
    u_var1 = (i_var2 + 0x8e);
    (u_var1 + 10) = 1;
    u_var1 = (i_var2 + 0x8e);
    temp_5f4a343e5a = (u_var1 + 0x12);
    if (temp_5f4a343e5a == &PTR_DAT_0005_0000_1050_0004) {
        // LAB_1038_c8da:
        h_wnd = GetDlgItem16(0xfce, (i_var2 + 6));
        if (h_wnd != 0) {
            EnableWindow16(1, h_wnd);
        }
        h_wnd_2 = GetDlgItem16(1, (i_var2 + 6));
        if (h_wnd_2 == 0) {}
        // goto LAB_1038_c93c;
        local_10 = 0x0;
    } else {
        if (((temp_5f4a343e5a + -5) < 1) || (SBORROW2((temp_5f4a343e5a + -5), 1))) {}
        // goto LAB_1038_c93c;
        if (temp_5f4a343e5a != &BYTE_1050_0008 && 0 < (temp_5f4a343e5a + -7)) {
            if (temp_5f4a343e5a != &BYTE_1050_0009) {}
            // goto LAB_1038_c93c;
            // goto LAB_1038_c8da;
        }
        h_wnd_2 = GetDlgItem16(0xfce, (i_var2 + 6));
        if (h_wnd_2 == 0) {}
        // goto LAB_1038_c93c;
        local_10 = (&ctx.PTR_LOOP_1050_0000 + 1);
    }
    EnableWindow16(local_10, h_wnd_2);
    // LAB_1038_c93c:
    ui1::move_window_1040_826c(param_1, 200, 0);
    ShowWindow16(5, (i_var2 + 6));
    return;
}

pub fn win_fn_1038_c672(param_1: i32, param_2: u16, param_3: u16, param_4: u32) {
    let mut unaff_ss: u16;
    let mut b: u16;
    let mut local_404: [u8; 1026];

    b = (ctx._g_struct_73_1050_14cc >> 0x10);
    if (param_3._2_2_ == 0x17d) {
        load_string_1010_84e0(
            ctx._g_struct_73_1050_14cc,
            b,
            0x3ff,
            CONCAT22(unaff_ss, local_404),
            0x453,
        );
        MessageBox16(
            0x30,
            (param_1 + 0x92),
            CONCAT22(unaff_ss, local_404),
            (param_1 + 6),
        );
    } else {
        if (param_3._2_2_ != 0x17e) {
            post_win_msg_1040_7b3c(param_1, param_2, param_3_00, param_3, param_3._2_2_);
            return;
        }
        load_string_1010_84e0(
            ctx._g_struct_73_1050_14cc,
            b,
            0x3ff,
            CONCAT22(unaff_ss, local_404),
            0x454,
        );
        MessageBox16(
            0x30,
            (param_1 + 0x92),
            CONCAT22(unaff_ss, local_404),
            (param_1 + 6),
        );
        pass1_1008_e164((param_1 + 0x8e));
    }
    PostMessage16(0, 2, 0x111, (param_1 + 6));
    return;
}

pub fn win_fn_1038_c58e(param_1: u32) {
    let mut i_var1: i32;
    let mut unaff_si: u16;
    let mut u_var2: i32;
    let mut unaff_ss: u16;
    let mut local_816: u16;
    let mut local_814: u16;
    let mut local_80e: i32;
    let mut uStack2060: u16;
    let mut uStack2058: u16;
    let mut uStack2052: u16;
    let HStack2050: HWND16;
    let mut local_40c: [u8; 1026];
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    _local_6 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(unaff_si, 2));
    local_a = (_local_6 + 0x68);
    u_var2 = (param_1 >> 0x10);
    i_var1 = param_1;
    GetWindowText16(0x80, CONCAT22(unaff_ss, local_40c), (i_var1 + 6));
    wsprintf16(
        &local_80e,
        CONCAT22(local_40c, unaff_ss),
        CONCAT22(local_a, unaff_ss),
    );
    SetWindowText16(CONCAT22(unaff_ss, &local_80e), (i_var1 + 6));
    local_80e = u_var2;
    pass1_1008_e038(
        (i_var1 + 0x8e) & 0xffff | (i_var1 + 0x96) << 0x10,
        param_1 & 0xffff0000 | u_var2,
        param_1 & 0xffff0000 | (i_var1 + 0x96),
    );
    local_80e = 0x7d6;
    local_816 = (ctx._g_struct_73_1050_14cc >> 0x10);
    load_string_1010_84e0(
        ctx._g_struct_73_1050_14cc,
        local_816,
        0x400,
        CONCAT22(unaff_ss, &local_80e),
        0x7d6,
    );
    local_80e = (*(i_var1 + 0x92) >> 0x10);
    local_814 = *CONCAT22(0x400, local_816);
    wsprintf16(
        local_40c,
        CONCAT22(&local_80e, unaff_ss),
        CONCAT22(local_814, unaff_ss),
    );
    HStack2050 = (i_var1 + 6);
    uStack2052 = 0x17f;
    uStack2058 = SUB42(offset, 0);
    uStack2060 = 0xc66c;
    SetDlgItemText16(CONCAT22(unaff_ss, local_40c), 0x17f, HStack2050);
    return;
}

pub fn set_focus_1038_c558(param_1: *mut Struct20) {
    let mut u_var1: u16;

    ui1::win_gui_func_1040_78e2(param_1);
    ui1::move_window_1040_826c(param_1, 0xffffffff);
    u_var1 = (param_1 >> 0x10);
    ShowWindow16(5, (param_1 + 6));
    SetFocus16((param_1 + 6));
    return;
}

pub fn enable_window_1038_c294(param_1: u32) {
    let lp_string: SEGPTR;
    let mut u_var1: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;

    _local_c = param_1 & 0xffff0000 | (param_1 + 0x9e);
    lp_string = pass1_1008_e320(
        (param_1 + 0x8e),
        param_1 & 0xffff0000 | (param_1 + 0x19e),
        param_1 & 0xffff0000 | (param_1 + 0x9e),
    );
    SetWindowText16(lp_string, (param_1 + 0x9a));
    u_var1 = string_fn_1008_e2a4(
        (param_1 + 0x8e),
        param_1 & 0xffff0000 | (param_1 + 0x19e),
        _local_c,
    );
    EnableWindow16(u_var1 & 1, (param_1 + 0x96));
    EnableWindow16(u_var1 & 2, (param_1 + 0x98));
    return;
}

pub fn set_window_pos_1038_c31a(param_1: u32, param_2: u16, param_3: u16) -> u16 {
    let mut unaff_cs: u16;
    let unaff_ss: HWND16;
    let mut local_e: u16;
    let mut local_a: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    local_4 = param_3;
    local_6 = param_2;
    if (param_3 == 1) {
        enable_window_1038_c294(param_1);
    } else {
        if (param_3 != 7) {
            return 0;
        }
        GetWindowRect16(CONCAT22(&local_e, unaff_cs), unaff_ss);
        local_a = local_a - local_e;
        SetWindowPos16(2, 0x50, local_a, 0, 0, 0, local_6);
    }
    return 1;
}

pub fn set_focus_1038_c044(param_1: *mut Struct20) {
    let mut u_var1: u16;

    ui1::win_gui_func_1040_78e2(param_1);
    ui1::move_window_1040_826c(param_1, 0xffffffff);
    u_var1 = (param_1 >> 0x10);
    ShowWindow16(5, (param_1 + 6));
    SetFocus16((param_1 + 6));
    return;
}

pub fn win_fn_1038_c07a(param_1: i32, param_2: u16, param_3: u16, param_4: u32) {
    let mut i_var1: i32;
    let unaff_ss: String;
    let u_var2: u8;
    let mut local_70c: [u8; 512];
    let mut local_50c: [u8; 256];
    let mut local_40c: [u8; 1026];
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    send_win_msg_1038_c228(param_1, param_2_00);
    _local_6 = load_string_1010_847e(
        ctx._g_struct_73_1050_14cc,
        (ctx._g_struct_73_1050_14cc >> 0x10),
        0x531,
    );
    u_var2 = (unaff_ss >> 8);
    if (param_2._2_2_ == 0x177) {
        pass1_1008_e05e(
            (param_1 + 0x8e),
            2,
            CONCAT22(param_2_00, (param_1 + 0x19e)),
            CONCAT22(param_2_00, param_1 + 0x9e),
        );
        load_string_1010_84e0(
            ctx._g_struct_73_1050_14cc,
            (ctx._g_struct_73_1050_14cc >> 0x10),
            0x200,
            CONCAT22(unaff_ss, local_40c),
            0x451,
        );
        string_fn_1000_3f9c(local_70c, unaff_ss, local_40c, unaff_ss, (param_1 + 0x19e));
        load_string_1010_84e0(
            ctx._g_struct_73_1050_14cc,
            (ctx._g_struct_73_1050_14cc >> 0x10),
            0x100,
            CONCAT22(unaff_ss, local_50c),
            0x57b,
        );
        MessageBox16(
            0x30,
            CONCAT13(u_var2, CONCAT12(unaff_ss, local_50c)),
            CONCAT22(unaff_ss, local_70c),
            (param_1 + 6),
        );
    } else {
        if (param_2._2_2_ != 0x178) {
            if ((param_2._2_2_ != 0x178) && (param_2._2_2_ - 0x179 < 2)) {
                set_window_pos_1038_c31a(param_1, param_2_00, param_3, param_2, param_2._2_2_);
                return;
            }
            post_win_msg_1040_7b3c(param_1, param_2_00, param_3, param_2, param_2._2_2_);
            return;
        }
        _local_a = CONCAT22(param_2_00, param_1 + 0x9e);
        i_var1 = pass1_1008_e10c(
            (param_1 + 0x8e),
            CONCAT22(param_2_00, param_1 + 0x19e),
            CONCAT22(param_2_00, param_1 + 0x9e),
        );
        if (i_var1 == 0) {
            load_string_1010_84e0(
                ctx._g_struct_73_1050_14cc,
                (ctx._g_struct_73_1050_14cc >> 0x10),
                0x3ff,
                CONCAT22(unaff_ss, local_40c),
                0x450,
            );
            load_string_1010_84e0(
                ctx._g_struct_73_1050_14cc,
                (ctx._g_struct_73_1050_14cc >> 0x10),
                0x3ff,
                CONCAT22(unaff_ss, local_50c),
                0x57b,
            );
            MessageBox16(
                0x30,
                CONCAT13(u_var2, CONCAT12(unaff_ss, local_50c)),
                CONCAT22(unaff_ss, local_40c),
                (param_1 + 6),
            );
            return;
        }
        pass1_1008_e01c(
            (param_1 + 0x8e),
            CONCAT22(param_2_00, param_1 + 0x19e),
            _local_a,
        );
        pass1_1038_af40(ctx._g_Struct112_a, *(param_1 + 8), 0x1f);
    }
    PostMessage16(0, 2, 0x111, (param_1 + 6));
    return;
}

pub fn win_fn_1038_bea4(param_1: u32) {
    let mut u_var1: u32;
    let HVar2: HWND16;

    let mut i_var3: i32;
    let mut u_var4: u16;
    let mut unaff_ss: u16;
    let ppVar5: *mut pass1_struct_1;
    let mut l_param: u32;
    let LVar6: LRESULT;
    let in_stack_0000fed2: u8;
    let in_stack_0000fed3: u8;
    let mut local_126: u16;
    let mut local_124: u16;
    let mut local_122: u16;
    let mut local_120: u16;
    let mut local_11e: u32;
    let mut local_11a: u16;
    let mut local_118: u16;
    let mut local_116: u32;
    let mut local_112: u32;
    let mut local_10e: [u8; 130];
    let mut local_8c: [u8; 130];
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    _local_6 = process_struct_1010_20ba(
        ctx._g_Struct372_1050_0ed0,
        CONCAT13(in_stack_0000fed3, CONCAT12(in_stack_0000fed2, 2)),
    );
    local_a = (_local_6 + 0x68);
    u_var4 = (param_1 >> 0x10);
    i_var3 = param_1;
    GetWindowText16(0x80, CONCAT22(unaff_ss, local_8c), (i_var3 + 6));
    wsprintf16(
        local_10e,
        CONCAT22(local_8c, unaff_ss),
        CONCAT22(local_a, unaff_ss),
    );
    SetWindowText16(CONCAT22(unaff_ss, local_10e), (i_var3 + 6));
    HVar2 = GetDlgItem16(0x179, (i_var3 + 6));
    (i_var3 + 0x92) = HVar2;
    process_struct_1008_e3ec(
        (i_var3 + 0x8e),
        CONCAT22(unaff_ss, &local_116),
        CONCAT22(unaff_ss, &local_112),
    );
    local_126 = local_112;
    local_124 = (local_112 >> 0x10);
    win_fn_1038_c374(
        i_var3,
        (param_1 >> 0x10),
        local_126,
        local_124,
        (i_var3 + 0x92),
    );
    ppVar5 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(local_120, 0x2f));
    u_var1 = (i_var3 + 0x8e);
    local_126 = (u_var1 >> 0x10);
    l_param = process_struct_1008_e586(u_var1, local_126, (ppVar5 + 0x24));
    SendMessage16(l_param, 0xffff, 0x40d, (i_var3 + 0x92));
    HVar2 = GetDlgItem16(0x17a, (i_var3 + 6));
    (i_var3 + 0x94) = HVar2;
    local_124 = local_116;
    local_122 = (local_116 >> 0x10);
    win_fn_1038_c374(param_1, u_var4, local_124, local_122, HVar2);
    local_124 = ctx._g_struct_73_1050_14cc;
    local_122 = (ctx._g_struct_73_1050_14cc >> 0x10);
    load_string_1010_847e(local_124, local_122, 0x531);
    LVar6 = SendMessage16(CONCAT22(ctx.dx_reg, HVar2), 0, 0x403, (i_var3 + 0x94));
    (i_var3 + 0x9c) = LVar6;
    SendMessage16(ctx.dx_reg, 0xffff, 0x40d, (i_var3 + 0x94));
    HVar2 = GetDlgItem16(0x178, (i_var3 + 6));
    (i_var3 + 0x96) = HVar2;
    HVar2 = GetDlgItem16(0x177, (i_var3 + 6));
    (i_var3 + 0x98) = HVar2;
    HVar2 = GetDlgItem16(0x184, (i_var3 + 6));
    (i_var3 + 0x9a) = HVar2;
    return;
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

pub fn gui_dialog_1038_b81c(param_1: *mut Struct20) {
    let mut u_var1: u32;
    let mut u_var2: u32;
    let ppc_var3: fn();
    let mut u_var4: i32;
    let mut h_dialog: u16;
    let mut b_result: u16;
    let hwnd: HWND16;

    let mut i32_var6: i32;
    let mut unaff_si: u16;
    let mut u_var7: u16;
    let pp_var8: *mut pass1_struct_1;
    let local_1e: *mut Struct19;
    let mut uStack28: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut check_id: u16;
    let mut local_a: u16;
    let mut local_6: u32;
    let pu_var5: *mut u16;

    ui1::win_gui_func_1040_78e2(param_1);
    pp_var8 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(unaff_si, 6));
    u_var7 = (param_1 >> 0x10);
    i32_var6 = param_1;
    (i32_var6 + 0x92) = pp_var8;
    (i32_var6 + 0x94) = (pp_var8 >> 0x10);
    u_var1 = (i32_var6 + 0x92);
    u_var4 = u_var1 + 0x4e;
    u_var1 = u_var1 & 0xffff0000;
    pu_var5 = (u_var1 | u_var4);
    local_a = 0;
    check_id = 0x1a0;
    while (check_id < 0x1b5) {
        if ((local_a * 2 + u_var4) == check_id) {
            local_a = local_a + 1;
        } else {
            CheckDlgButton16(2, check_id, (i32_var6 + 6));
        }
        check_id = check_id + 1;
    }
    h_dialog = GetDlgItem16(0xfb1, (i32_var6 + 6));
    b_result = EnableWindow16(0, h_dialog);
    u_var2 = (i32_var6 + 0x92);
    local_1e = u_var2;
    uStack28 = (u_var2 >> 0x10);
    ppc_var3 = ((i32_var6 + 0x92) + 0x10);
    (**ppc_var3)(offset, local_1e, uStack28);
    _local_10 = CONCAT22(ctx.dx_reg, b_result);
    ui1::move_window_1040_826c(
        i32_var6,
        u_var7,
        (b_result + 2) + -2,
        (b_result + 4) + *_local_10 + 3,
    );
    ShowWindow16(5, (i32_var6 + 6));
    unsafe {
        pass1_1018_1c9a(*(i32_var6 + 0x92), *pu_var5);
    }
    unsafe {
        hwnd = GetDlgItem16(*pu_var5, (i32_var6 + 6));
    }
    SetFocus16(hwnd);
    return;
}

pub fn gui_window_func_1038_b72e(param_1: u32, param_2: i32) -> u16 {
    let hwnd: HWND16;
    let mut u_var1: u32;
    let mut u_var2: u16;
    let mut local_4: u16;

    u_var2 = (param_1 >> 0x10);
    if ((param_2 * 4 + param_1) != 0) {
        u_var1 = (param_2 * 4 + param_1);
        hwnd = (u_var1 + 6);
        SetFocus16(hwnd);
        BringWindowToTop16(hwnd);
        return 1;
    }
    return 0;
}

pub fn show_window_1038_b634(param_1: u32) {
    let mut u_var1: u32;
    let mut i_var2: i32;
    let mut u_var3: u16;
    let mut local_4: u16;

    u_var3 = (param_1 >> 0x10);
    i_var2 = param_1;
    if ((i_var2 + 0xac) == 0) {
        (i_var2 + 0xac) = 1;
        local_4 = 1;
        while (local_4 < 0x2b) {
            if (((local_4 * 4 + i_var2 + 2) | (local_4 * 4 + i_var2)) != 0) {
                u_var1 = (local_4 * 4 + i_var2);
                ShowWindow16(0, (u_var1 + 6));
            }
            local_4 = local_4 + 1;
        }
    }
    return;
}

pub fn show_win_1038_b68a(param_1: u32) {
    let mut u_var1: u32;
    let mut i_var2: i32;
    let mut u_var3: u16;
    let mut local_4: u16;

    u_var3 = (param_1 >> 0x10);
    i_var2 = param_1;
    if ((i_var2 + 0xac) != 0) {
        (i_var2 + 0xac) = 0;
        local_4 = 1;
        while (local_4 < 0x2b) {
            if (((local_4 * 4 + i_var2 + 2) | (local_4 * 4 + i_var2)) != 0) {
                u_var1 = (local_4 * 4 + i_var2);
                ShowWindow16(1, (u_var1 + 6));
            }
            local_4 = local_4 + 1;
        }
    }
    return;
}

pub fn pass1_1038_af34() {
    ctx._g_Struct112_a = 0;
    return;
}

pub fn pass1_1038_af40(param_1: *mut Struct112, param_2: Vec<u8>, param_3: u16) -> *mut u16 {
    let pp_var1: fn();
    let mut u_var2: u32;
    let u_var3: u8;
    let extraout_AH: u8;
    let struct_a: *mut Struct199;
    let mut u_var4: i32;
    let mut in_bx: i32;
    let mut u_var5: u16;
    let mut u_var6: u32;
    let pa_var7: *mut Struct68;
    let mut i_var8: i32;
    let mut u_var9: u16;
    let mut local_8: u16;

    i_var8 = param_1;
    u_var9 = (param_1 >> 0x10);
    u_var6 = gui_window_func_1038_b72e(param_1, param_3);
    struct_a = (u_var6 >> 0x10);
    if (u_var6 != 0x0) {}
    // goto LAB_1038_b61f;
    u_var5 = SUB42(&PTR_LOOP_1050_1038, 0);
    PTR_LOOP_1050_5b82 = u_var6;
    match (param_3) {
        1 => {
            u_var5 = 0x1000;
            process_struct_1000_179c(0x8e, struct_a);
            PTR_LOOP_1050_5b82 = u_var6;
            if (((u_var6 >> 0x10) | in_bx) == 0) {
                // LAB_1038_afa0:
                u_var5 = 0x1000;
                pa_var7 = 0x0;
            } else {
                pa_var7 = pass1_1038_9f76((u_var6 & 0xffff0000 | in_bx), 0, 0x0, 0x0, param_2);
            }
        }
        2 => {
            process_struct_1000_179c(0x96, struct_a);
            PTR_LOOP_1050_5b82 = u_var6;
            u_var4 = (u_var6 >> 0x10) | in_bx;
            if (u_var4 == 0) {}
            // goto LAB_1038_afa0;
            u_var5 = SUB42(&ctx.PTR_LOOP_1050_1040, 0);
            pass1_1040_181c((u_var6 & 0xffff0000 | in_bx), 0, 0x0, 0x0, param_2);
            pa_var7 = CONCAT22(u_var4, in_bx)
        }
        3 => {
            u_var5 = 0x1000;
            process_struct_1000_179c(0x92, struct_a);
            PTR_LOOP_1050_5b82 = u_var6;
            if (((u_var6 >> 0x10) | in_bx) == 0) {}
            // goto LAB_1038_afa0;
            pa_var7 = pass1_1038_e99a((u_var6 & 0xffff0000 | in_bx), 0, 0x0, 0x0, param_2)
        }
        4 => {
            u_var5 = 0x1000;
            process_struct_1000_179c(0x92, struct_a);
            PTR_LOOP_1050_5b82 = u_var6;
            if (((u_var6 >> 0x10) | in_bx) == 0) {}
            // goto LAB_1038_afa0;
            pa_var7 = pass1_1038_c7b8((u_var6 & 0xffff0000 | in_bx), 0, 0x0, 0x0, param_2)
        }
        5 => {
            process_struct_1000_179c(0x96, struct_a);
            PTR_LOOP_1050_5b82 = u_var6;
            u_var4 = (u_var6 >> 0x10) | in_bx;
            if (u_var4 == 0) {}
            // goto LAB_1038_afa0;
            u_var5 = SUB42(&ctx.PTR_LOOP_1050_1040, 0);
            pass1_1040_23ea((u_var6 & 0xffff0000 | in_bx), 0, 0x0, 0x0, param_2);
            pa_var7 = CONCAT22(u_var4, in_bx)
        }
        6 => {
            process_struct_1000_179c(0x92, struct_a);
            PTR_LOOP_1050_5b82 = u_var6;
            if (((u_var6 >> 0x10) | in_bx) == 0) {}
            // goto LAB_1038_afa0;
            u_var5 = SUB42(&ctx.PTR_LOOP_1050_1040, 0);
            pa_var7 = pass1_1040_06e8((u_var6 & 0xffff0000 | in_bx), 0, 0x0, 0x0, param_2)
        }
        7 => {
            process_struct_1000_179c(0x9c, struct_a);
            PTR_LOOP_1050_5b82 = u_var6;
            u_var4 = (u_var6 >> 0x10) | in_bx;
            if (u_var4 == 0) {}
            // goto LAB_1038_afa0;
            u_var5 = SUB42(&ctx.PTR_LOOP_1050_1040, 0);
            pass1_1040_4068((u_var6 & 0xffff0000 | in_bx), 0, 0x0, 0x0, param_2);
            pa_var7 = CONCAT22(u_var4, in_bx)
        }
        8 => {
            u_var5 = 0x1000;
            process_struct_1000_179c(0x9a, struct_a);
            u_var4 = (u_var6 >> 0x10);
            PTR_LOOP_1050_5b82 = u_var6;
            if ((u_var4 | in_bx) == 0) {}
            // goto LAB_1038_afa0;
            pa_var7 = pass1_1038_b772(in_bx, u_var4, 0, 0, 0, 0, param_2)
        }
        9 => {
            u_var5 = 0x1000;
            process_struct_1000_179c(0x8e, struct_a);
            PTR_LOOP_1050_5b82 = u_var6;
            if (((u_var6 >> 0x10) | in_bx) == 0) {}
            // goto LAB_1038_afa0;
            pa_var7 = pass1_1038_e140((u_var6 & 0xffff0000 | in_bx), 0, 0x0, 0x0, param_2)
        }
        10 => {
            u_var5 = 0x1000;
            process_struct_1000_179c(0x90, struct_a);
            PTR_LOOP_1050_5b82 = u_var6;
            if (((u_var6 >> 0x10) | in_bx) == 0) {}
            // goto LAB_1038_afa0;
            pa_var7 = pass1_1038_a33c((u_var6 & 0xffff0000 | in_bx), param_2)
        }
        0xb => {
            u_var5 = 0x1000;
            process_struct_1000_179c(0x90, struct_a);
            PTR_LOOP_1050_5b82 = u_var6;
            if (((u_var6 >> 0x10) | in_bx) == 0) {}
            // goto LAB_1038_afa0;
            pa_var7 = pass1_1038_a494((u_var6 & 0xffff0000 | in_bx), param_2)
        }
        0xc => {
            u_var5 = 0x1000;
            process_struct_1000_179c(0x90, struct_a);
            PTR_LOOP_1050_5b82 = u_var6;
            if (((u_var6 >> 0x10) | in_bx) == 0) {}
            // goto LAB_1038_afa0;
            pa_var7 = pass1_1038_a69a((u_var6 & 0xffff0000 | in_bx), param_2)
        }
        0xd => {
            u_var5 = 0x1000;
            process_struct_1000_179c(0x90, struct_a);
            PTR_LOOP_1050_5b82 = u_var6;
            if (((u_var6 >> 0x10) | in_bx) == 0) {}
            // goto LAB_1038_afa0;
            pa_var7 = pass1_1038_a89e((u_var6 & 0xffff0000 | in_bx), param_2)
        }
        0xe => {
            u_var5 = 0x1000;
            process_struct_1000_179c(0x94, struct_a);
            PTR_LOOP_1050_5b82 = u_var6;
            u_var4 = (u_var6 >> 0x10) | in_bx;
            if (u_var4 == 0) {}
            // goto LAB_1038_afa0;
            pass1_1038_e69a((u_var6 & 0xffff0000 | in_bx), 0, 0x0, 0x0, param_2);
            pa_var7 = CONCAT22(u_var4, in_bx)
        }
        0xf => {
            u_var5 = 0x1000;
            process_struct_1000_179c(0x94, struct_a);
            PTR_LOOP_1050_5b82 = u_var6;
            u_var4 = (u_var6 >> 0x10) | in_bx;
            if (u_var4 == 0) {}
            // goto LAB_1038_afa0;
            pass1_1038_cd06((u_var6 & 0xffff0000 | in_bx), 0, 0x0, 0x0, param_2);
            pa_var7 = CONCAT22(u_var4, in_bx)
        }
        0x10 => {
            process_struct_1000_179c(0x92, struct_a);
            PTR_LOOP_1050_5b82 = u_var6;
            if (((u_var6 >> 0x10) | in_bx) == 0) {}
            // goto LAB_1038_afa0;
            u_var5 = SUB42(&ctx.PTR_LOOP_1050_1040, 0);
            pa_var7 = pass1_1040_0bfc((u_var6 & 0xffff0000 | in_bx), 0, 0x0, 0x0, param_2)
        }
        0x11 => {
            process_struct_1000_179c(0x9a, struct_a);
            PTR_LOOP_1050_5b82 = u_var6;
            u_var4 = (u_var6 >> 0x10) | in_bx;
            if (u_var4 == 0) {}
            // goto LAB_1038_afa0;
            u_var5 = SUB42(&ctx.PTR_LOOP_1050_1040, 0);
            pass1_1040_0e1c(
                (u_var6 & 0xff000000 | CONCAT12((u_var6 >> 0x10), in_bx)),
                0x0,
                0x0,
                param_2,
            );
            pa_var7 = CONCAT22(u_var4, in_bx)
        }
        0x12 => {
            u_var5 = 0x1000;
            process_struct_1000_179c(0x9a, struct_a);
            u_var4 = (u_var6 >> 0x10);
            PTR_LOOP_1050_5b82 = u_var6;
            if ((u_var4 | in_bx) == 0) {}
            // goto LAB_1038_afa0;
            pa_var7 = pass1_1038_d756(in_bx, u_var4, param_2)
        }
        0x13 => {
            u_var5 = 0x1000;
            process_struct_1000_179c(0x92, struct_a);
            PTR_LOOP_1050_5b82 = u_var6;
            u_var4 = (u_var6 >> 0x10) | in_bx;
            if (u_var4 == 0) {}
            // goto LAB_1038_afa0;
            u_var3 = pass1_1038_cad8(u_var6 & 0xffff0000 | in_bx, param_2);
            pa_var7 = CONCAT22(u_var4, CONCAT11(extraout_AH, u_var3))
        }
        0x14 => {
            process_struct_1000_179c(0xaa, struct_a);
            u_var4 = (u_var6 >> 0x10);
            PTR_LOOP_1050_5b82 = u_var6;
            if ((u_var4 | in_bx) == 0) {}
            // goto LAB_1038_afa0;
            u_var5 = SUB42(&ctx.PTR_LOOP_1050_1040, 0);
            pa_var7 = pass1_1040_1f5a(in_bx, u_var4, param_2)
        }
        0x15 => {
            u_var5 = 0x1000;
            process_struct_1000_179c(0x8e, struct_a);
            u_var4 = (u_var6 >> 0x10);
            PTR_LOOP_1050_5b82 = u_var6;
            if ((u_var4 | in_bx) == 0) {}
            // goto LAB_1038_afa0;
            pa_var7 = pass1_1038_d242(in_bx, u_var4, param_2)
        }
        0x16 => {
            u_var5 = 0x1000;
            process_struct_1000_179c(0x9a, struct_a);
            u_var4 = (u_var6 >> 0x10);
            PTR_LOOP_1050_5b82 = u_var6;
            if ((u_var4 | in_bx) == 0) {}
            // goto LAB_1038_afa0;
            pa_var7 = pass1_1038_eeda(in_bx, u_var4, param_2)
        }
        0x17 => {
            process_struct_1000_179c(0x96, struct_a);
            PTR_LOOP_1050_5b82 = u_var6;
            if (((u_var6 >> 0x10) | in_bx) == 0) {}
            // goto LAB_1038_afa0;
            u_var5 = 0x1018;
            pa_var7 = pass1_1018_5e26((u_var6 & 0xffff0000 | in_bx), param_2)
        }
        // default:
        // goto switchD_1038_b581_caseD_18;
        0x19 => {
            process_struct_1000_179c(0x96, struct_a);
            PTR_LOOP_1050_5b82 = u_var6;
            u_var4 = (u_var6 >> 0x10) | in_bx;
            if (u_var4 == 0) {}
            // goto LAB_1038_afa0;
            u_var5 = SUB42(&ctx.PTR_LOOP_1050_1040, 0);
            pass1_1040_1cb4((u_var6 & 0xffff0000 | in_bx), 0, 0x0, 0x0, param_2);
            pa_var7 = CONCAT22(u_var4, in_bx)
        }
        0x1a => {
            process_struct_1000_179c(0x92, struct_a);
            PTR_LOOP_1050_5b82 = u_var6;
            if (((u_var6 >> 0x10) | in_bx) == 0) {}
            // goto LAB_1038_afa0;
            u_var5 = SUB42(&ctx.PTR_LOOP_1050_1040, 0);
            pa_var7 = pass1_1040_123e((u_var6 & 0xffff0000 | in_bx), 0, 0x0, 0x0, param_2)
        }
        0x1b => {
            u_var5 = 0x1000;
            process_struct_1000_179c(0x8e, struct_a);
            u_var4 = (u_var6 >> 0x10);
            PTR_LOOP_1050_5b82 = u_var6;
            if ((u_var4 | in_bx) == 0) {}
            // goto LAB_1038_afa0;
            pa_var7 = pass1_1038_ab82(in_bx, u_var4, param_2)
        }
        0x1c => {
            u_var5 = 0x1000;
            process_struct_1000_179c(0x92, struct_a);
            u_var4 = (u_var6 >> 0x10);
            PTR_LOOP_1050_5b82 = u_var6;
            if ((u_var4 | in_bx) == 0) {}
            // goto LAB_1038_afa0;
            pa_var7 = pass1_1038_e2d0(in_bx, u_var4, param_2)
        }
        0x1d => {
            u_var5 = 0x1000;
            process_struct_1000_179c(0x92, struct_a);
            u_var4 = (u_var6 >> 0x10);
            PTR_LOOP_1050_5b82 = u_var6;
            if ((u_var4 | in_bx) == 0) {}
            // goto LAB_1038_afa0;
            pa_var7 = pass1_1038_eb9e(in_bx, u_var4, param_2)
        }
        0x1e => {
            u_var5 = 0x1000;
            process_struct_1000_179c(0x29e, struct_a);
            PTR_LOOP_1050_5b82 = u_var6;
            u_var4 = (u_var6 >> 0x10) | in_bx;
            if (u_var4 == 0) {}
            // goto LAB_1038_afa0;
            pass1_1038_bddc((u_var6 & 0xffff0000 | in_bx), 0, 0x0, 0x0, param_2);
            pa_var7 = CONCAT22(u_var4, in_bx)
        }
        0x1f => {
            u_var5 = 0x1000;
            process_struct_1000_179c(0x9a, struct_a);
            PTR_LOOP_1050_5b82 = u_var6;
            u_var4 = (u_var6 >> 0x10) | in_bx;
            if (u_var4 == 0) {}
            // goto LAB_1038_afa0;
            pass1_1038_c4a2((u_var6 & 0xffff0000 | in_bx), 0, 0x0, 0x0, param_2);
            pa_var7 = CONCAT22(u_var4, in_bx)
        }
        0x20 => {
            process_struct_1000_179c(0x29a, struct_a);
            PTR_LOOP_1050_5b82 = u_var6;
            u_var4 = (u_var6 >> 0x10) | in_bx;
            if (u_var4 == 0) {}
            // goto LAB_1038_afa0;
            u_var5 = SUB42(&ctx.PTR_LOOP_1050_1040, 0);
            pass1_1040_2ea2((u_var6 & 0xffff0000 | in_bx), 0, 0x0, 0x0, param_2);
            pa_var7 = CONCAT22(u_var4, in_bx)
        }
        0x21 => {
            process_struct_1000_179c(0xa6, struct_a);
            PTR_LOOP_1050_5b82 = u_var6;
            u_var4 = (u_var6 >> 0x10) | in_bx;
            if (u_var4 == 0) {}
            // goto LAB_1038_afa0;
            u_var5 = SUB42(&ctx.PTR_LOOP_1050_1040, 0);
            pass1_1040_3966((u_var6 & 0xffff0000 | in_bx), 0, 0x0, 0x0, param_2);
            pa_var7 = CONCAT22(u_var4, in_bx)
        }
        0x22 => {
            process_struct_1000_179c(0x9a, struct_a);
            PTR_LOOP_1050_5b82 = u_var6;
            u_var4 = (u_var6 >> 0x10) | in_bx;
            if (u_var4 == 0) {}
            // goto LAB_1038_afa0;
            u_var5 = SUB42(&ctx.PTR_LOOP_1050_1040, 0);
            pass1_1040_34a2((u_var6 & 0xffff0000 | in_bx), 0, 0x0, 0x0, param_2);
            pa_var7 = CONCAT22(u_var4, in_bx)
        }
        0x23 => {
            process_struct_1000_179c(0x9c, struct_a);
            PTR_LOOP_1050_5b82 = u_var6;
            if (((u_var6 >> 0x10) | in_bx) == 0) {}
            // goto LAB_1038_afa0;
            u_var5 = SUB42(&ctx.PTR_LOOP_1050_1040, 0);
            pa_var7 = pass1_1040_ac84((u_var6 & 0xffff0000 | in_bx), param_2)
        }
        0x25 => {
            process_struct_1000_179c(0xa0, struct_a);
            PTR_LOOP_1050_5b82 = u_var6;
            if (((u_var6 >> 0x10) | in_bx) == 0) {}
            // goto LAB_1038_afa0;
            u_var5 = SUB42(&ctx.PTR_LOOP_1050_1040, 0);
            pa_var7 = pass1_1040_ca16((u_var6 & 0xffff0000 | in_bx), param_2)
        }
        0x26 => {
            process_struct_1000_179c(0xa2, struct_a);
            PTR_LOOP_1050_5b82 = u_var6;
            if (((u_var6 >> 0x10) | in_bx) == 0) {}
            // goto LAB_1038_afa0;
            u_var5 = SUB42(&ctx.PTR_LOOP_1050_1040, 0);
            pa_var7 = pass1_1040_d0f8((u_var6 & 0xffff0000 | in_bx), param_2)
        }
        0x27 => {
            u_var5 = 0x1000;
            process_struct_1000_179c(0xa0, struct_a);
            PTR_LOOP_1050_5b82 = u_var6;
            u_var4 = (u_var6 >> 0x10) | in_bx;
            if (u_var4 == 0) {}
            // goto LAB_1038_afa0;
            pass1_1038_88f2((u_var6 & 0xffff0000 | in_bx), param_2);
            pa_var7 = CONCAT22(u_var4, in_bx)
        }
        0x28 => {
            process_struct_1000_179c(0x96, struct_a);
            u_var4 = (u_var6 >> 0x10);
            PTR_LOOP_1050_5b82 = u_var6;
            if ((u_var4 | in_bx) == 0) {}
            // goto LAB_1038_afa0;
            u_var5 = SUB42(&ctx.PTR_LOOP_1050_1040, 0);
            pa_var7 = pass1_1040_6402(in_bx, u_var4, param_2)
        }
        0x29 => {
            u_var5 = 0x1000;
            process_struct_1000_179c(0x98, struct_a);
            PTR_LOOP_1050_5b82 = u_var6;
            if (((u_var6 >> 0x10) | in_bx) == 0) {}
            // goto LAB_1038_afa0;
            pa_var7 = pass1_1038_7d10(u_var6 & 0xffff0000 | in_bx, param_2)
        }
        0x2a => {
            u_var5 = 0x1000;
            process_struct_1000_179c(0x98, struct_a);
            PTR_LOOP_1050_5b82 = u_var6;
            if (((u_var6 >> 0x10) | in_bx) == 0) {}
            // goto LAB_1038_afa0;
            pa_var7 = pass1_1038_8caa((u_var6 & 0xffff0000 | in_bx), param_2);
        }
    }
    (param_3 * 4 + i_var8) = pa_var7;
    (param_3 * 4 + i_var8 + 2) = (pa_var7 >> 0x10);
    // switchD_1038_b581_caseD_18:
    if ((param_3 * 4 + i_var8) != 0) {
        if ((i_var8 + 0xae) != 0) {
            u_var2 = (param_3 * 4 + i_var8);
            (u_var2 + 0x6e) = (i_var8 + 0xae);
        }
        (i_var8 + 0xae) = 0;
        u_var2 = (param_3 * 4 + i_var8);
        pp_var1 = ((param_3 * 4 + i_var8) + 8);
        (**pp_var1)(u_var5, u_var2, (u_var2 >> 0x10));
    }
    // LAB_1038_b61f:
    return CONCAT22((param_3 * 4 + i_var8 + 2), (param_3 * 4 + i_var8));
}

pub fn pass1_1038_aeca(param_1: *mut Struct65) -> *mut Struct65 {
    let mut u_var1: u16;
    let mut unaff_ss: u16;
    let mut local_b6: u16;
    let mut local_b4: u16;
    let mut local_5c: [u8; 90];

    u_var1 = (param_1 >> 0x10);
    (param_1 + 0xac) = 0;
    (param_1 + 0xae) = 0;
    if (ctx._g_Struct112_a == 0x0) {
        ctx._g_Struct112_a = param_1;
    }
    pass1_1000_4906(param_1, 0, 0xac);
    ui1::load_cursor_1008_80ee(local_5c, unaff_ss);
    ui1::load_cursor_1040_9854(&local_b6, unaff_ss);
    local_b6 = ctx.s_1_1050_389a;
    local_b4 = &ctx.PTR_LOOP_1050_1008;
    modify_list_1008_8168(CONCAT22(unaff_ss, local_5c));
    return param_1;
}

pub fn pass1_1038_aeaa(param_1: &mut Struct44) -> &mut Struct44 {
    let pu8_var1: Vec<u8>;
    let mut b_var2: u8;
    let mut cVar3: u8;
    let mut u8_var4: u8;
    let mut in_CL: u8;
    let mut in_dx: i32;
    let mut b_var5: u8;
    let mut in_bx: i32;
    let mut bVar6: u8;
    let pu_var7: *mut u16;
    let unaff_bp: *mut u16;
    let unaff_si: Vec<u8>;
    let mut unaff_ss: u16;
    let mut in_CF: u8;
    let mut bVar8: bool;
    let mut bVar9: bool;
    let mut uStack00aa: u16;
    let mut uStack00ac: u16;
    let pa_var10: *mut Struct65;
    let mut bStack78: u8;
    let puStack34: Vec<u8>;

    puStack34 = &stack0xfffe;
    pu_var7 = &stack0xfffe;
    cVar3 = '\x0f';
    while {
        unaff_bp = unaff_bp + -1;
        pu_var7 = pu_var7 + -1;
        unsafe {
            *pu_var7 = *unaff_bp;
        }
        cVar3 = cVar3 + -1;
        '\0' < cVar3
    } {}
    bVar6 = (in_bx >> 8);
    unaff_si[in_bx] = bVar6;
    b_var5 = ((in_dx + 1) >> 8);
    bVar8 = CARRY1(b_var5, bVar6) || CARRY1(b_var5 + bVar6, in_CF);
    u8_var4 = (in_dx + 1);
    pa_var10 = CONCAT22(&stack0xbf2a, &stack0xfffe);
    pu8_var1 = unaff_si + in_bx;
    unsafe {
        b_var5 = *pu8_var1;
        b_var2 = *pu8_var1 - u8_var4;
        bVar9 = *pu8_var1 < u8_var4 || b_var2 < bVar8;
        *pu8_var1 = b_var2 - bVar8;
        if (*pu8_var1 != 0
            && (SBORROW1(b_var5, u8_var4) != SBORROW1(b_var2, bVar8)) == (*pu8_var1 < '\0'))
        {
            pu8_var1 = unaff_si;
            bVar8 = CARRY1(*pu8_var1, bVar6) || CARRY1(*pu8_var1 + bVar6, bVar9);
            *pu8_var1 = *pu8_var1 + bVar6 + bVar9;
            b_var5 = bStack78 + in_bx;
            bVar9 = CARRY1(bStack78, in_bx) || CARRY1(b_var5, bVar8);
            bStack78 = b_var5 + bVar8;
            pu8_var1 = unaff_si + -0x4f;
            b_var5 = *pu8_var1;
            b_var2 = *pu8_var1;
            *pu8_var1 = b_var2 + bVar6 + bVar9;
            pu8_var1 = unaff_si + -0x78;
            *pu8_var1 =
                *pu8_var1 + in_CL + (CARRY1(b_var5, bVar6) || CARRY1(b_var2 + bVar6, bVar9));
            uStack00aa = 0;
            uStack00ac = 0;
            if (ctx._g_Struct112_a == 0) {
                ctx._g_Struct112_a = CONCAT22(&stack0xbf2a, &stack0xfffe);
            }
            puStack34 = &stack0xfffe;
            pass1_1000_4906(pa_var10, 0, 0xac);
            ui1::load_cursor_1008_80ee();
            ui1::load_cursor_1040_9854();
            modify_list_1008_8168(CONCAT22(unaff_ss, &stack0xbebe));
            return pa_var10;
        }
        if (*pu8_var1 != 0) {
            error_check_1000_17ce(param_1);
        }
    }
    return param_1;
}

pub fn pass1_1038_ae56(param_1: &mut Struct44) -> &mut Struct44 {
    let pu8_var1: Vec<u8>;
    let pu_var2: *mut u32;
    let pcVar3: String;
    let mut u8_var4: u8;
    let mut u_var5: i32;
    let mut u_var6: i32;
    let mut cVar7: u8;
    let mut cVar8: u8;
    let mut in_cx: u16;
    let mut bVar9: u8;
    let mut bVar10: u8;
    let mut in_dx: i32;
    let mut bVar12: u8;
    let mut i_var11: i32;
    let mut bVar13: u8;
    let mut in_bx: u16;
    let mut i_var14: i32;
    let pu_var15: Vec<u8>;
    let pu_var16: *mut u16;
    let unaff_bp: *mut u16;
    let pu_var17: Vec<u8>;
    let pu_var18: Vec<u8>;
    let unaff_si: Vec<u8>;
    let mut unaff_DI: i32;
    let unaff_es: Vec<u8>;
    let pu_var19: Vec<u8>;
    let mut unaff_ss: u16;
    let mut b_var20: bool;
    let mut b_var21: bool;
    let mut in_stack_0000007c: u16;
    let mut bStack007d: u8;
    let mut in_stack_0000c741: u32;
    let local_4e: u8;
    let puStack34: Vec<u8>;

    puStack34 = &stack0xfffe;
    pu_var16 = &stack0xfffe;
    pu_var15 = &stack0xfffe;
    pu_var17 = &stack0xfffe;
    cVar8 = '\x0f';
    while {
        unaff_bp = unaff_bp + -1;
        pu_var16 = pu_var16 + -1;
        unsafe {
            *pu_var16 = *unaff_bp;
        }
        cVar8 = cVar8 + -1;
        '\0' < cVar8
    } {}
    bVar9 = (in_cx >> 8);
    pu_var19 = &stack0xc73f;
    bVar10 = in_dx;
    b_var20 = CARRY1(bStack007d, bVar10)
        || CARRY1(
            bStack007d + bVar10,
            unaff_si[CONCAT11((in_bx >> 8), 0x40)] < bVar10,
        );
    pu8_var1 = &stack0x4079 + unaff_si;
    bVar12 = (in_dx >> 8);
    unsafe {
        b_var21 = CARRY1(*pu8_var1, bVar12) || CARRY1(*pu8_var1 + bVar12, b_var20);
        *pu8_var1 = *pu8_var1 + bVar12 + b_var20;
        pu8_var1 = unaff_si;
        u8_var4 = *pu8_var1;
        bVar13 = *pu8_var1 + 0x40;
        b_var20 = 0xbf < *pu8_var1 || CARRY1(bVar13, b_var21);
        *pu8_var1 = bVar13 + b_var21;
        cVar8 = in_cx;
        if ((*pu8_var1 == 0)
            || (SCARRY1(u8_var4, '@') != SCARRY1(bVar13, b_var21)) != (*pu8_var1 < '\0'))
        {
            pu8_var1 = unaff_si + 0x3fc8;
            *pu8_var1 = *pu8_var1 + cVar8 + b_var20;
            cVar7 = PTR_LOOP_1050_4080;
            PTR_LOOP_1050_4080._0_1_ = '@';
            i_var14 = CONCAT11(cVar7, 0x40);
            pu8_var1 = unaff_si;
            *pu8_var1 = *pu8_var1 + cVar8 + (unaff_si[0x4040] < bVar10);
            pu8_var1 = unaff_si + i_var14 + 0x10;
            *pu8_var1 = *pu8_var1 + 0x54;
            pu8_var1 = unaff_si + i_var14 + 0x10;
            *pu8_var1 = *pu8_var1 - 0x34;
            pu_var2 = (unaff_si + i_var14 + 0x10);
            u_var5 = *pu_var2;
            *pu_var2 = *pu_var2 + 0x81b6;
            pu_var2 = (unaff_si + i_var14 + 0x10);
            u_var6 = *pu_var2;
            *pu_var2 = *pu_var2 + 0x60ea;
            pu8_var1 = unaff_si + i_var14;
            *pu8_var1 = (*pu8_var1 - (in_dx & 0xff)) - (0x9f15 < u_var6);
            i_var11 = (in_dx & 0xff | (bVar12 + cVar7 + (0x7e49 < u_var5)) << 8) - 1;
            pu8_var1 = unaff_si + i_var14 + 0x10;
            *pu8_var1 = *pu8_var1 + 0x66;
            pu8_var1 = unaff_si + i_var14 + 0x10;
            u8_var4 = *pu8_var1;
            *pu8_var1 = *pu8_var1 - 0x22;
            bVar13 = (i_var11 >> 8);
            if (-1 < *pu8_var1) {
                pu8_var1 = unaff_si + i_var14;
                *pu8_var1 = (*pu8_var1 - i_var11)
                    - (CARRY1(bVar13, bVar9) || CARRY1(bVar13 + bVar9, 0x21 < u8_var4));
                // do
                // {
                //     // WARNING: Do nothing block with infinite loop
                // } while (true);
            }
            pcVar3 = (unaff_DI + 8);
            *pcVar3 = *pcVar3 + bVar13;
            pu_var15 = (in_stack_0000c741 >> 0x10);
            pu_var19 = unaff_es;
        } else {
            b_var20 = 0xbf < bVar12 || CARRY1(bVar12 + 0x40, b_var20);
            pu8_var1 = unaff_si + 0x4040;
            u8_var4 = *pu8_var1;
            bVar13 = *pu8_var1 - bVar10;
            b_var21 = *pu8_var1 < bVar10 || bVar13 < b_var20;
            *pu8_var1 = bVar13 - b_var20;
            if ((*pu8_var1 == 0)
                || (SBORROW1(u8_var4, bVar10) != SBORROW1(bVar13, b_var20)) != (*pu8_var1 < '\0'))
            {
                if (*pu8_var1 != 0) {
                    error_check_1000_17ce(param_1);
                }
                return param_1;
            }
            pu8_var1 = unaff_si;
            b_var20 = 0xbf < *pu8_var1 || CARRY1(*pu8_var1 + 0x40, b_var21);
            *pu8_var1 = *pu8_var1 + 0x40 + b_var21;
            b_var21 = 0xbf < local_4e || CARRY1(local_4e + 0x40, b_var20);
            local_4e = local_4e + 0x40 + b_var20;
            pu8_var1 = unaff_si + -0x4f;
            u8_var4 = *pu8_var1;
            bVar13 = *pu8_var1;
            *pu8_var1 = bVar13 + 0x40 + b_var21;
            pu8_var1 = unaff_si + -0x78;
            *pu8_var1 = *pu8_var1 + cVar8 + (0xbf < u8_var4 || CARRY1(bVar13 + 0x40, b_var21));
            pu_var18 = &stack0xc72d;
            pu_var17 = &stack0xc72d;
            if (ctx._g_Struct112_a != 0) {}
            // goto LAB_1038_aeed;
        }
    }
    ctx._g_Struct112_a = CONCAT22(pu_var19, pu_var15);
    pu_var18 = pu_var17;
    // LAB_1038_aeed:
    puStack34 = &stack0xfffe;
    pass1_1000_4906((pu_var18 + 6), 0, 0xac);
    ui1::load_cursor_1008_80ee();
    ui1::load_cursor_1040_9854();
    (pu_var18 + -0xb4) = ctx.s_1_1050_389a;
    (pu_var18 + -0xb2) = &ctx.PTR_LOOP_1050_1008;
    modify_list_1008_8168(CONCAT22(unaff_ss, pu_var18 + -0x5a));
    return CONCAT22((pu_var18 + 8), (pu_var18 + 6));
}

pub fn set_colors_1038_ac38(
    param_1: u16,
    param_2: u16,
    dialog_handle: HWND16,
    uparam_2_00: i32,
    param_5: HDC16,
) {
    let mut i_var1: i32;
    let local_AX_35: *mut Struct66;
    let i_var2: u16;
    let mut u_var3: u32;
    let mut u_var4: u16;
    let mut u_var5: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_4: u16;

    GetStockObject16(4);
    if (_PTR_LOOP_1050_5b78 == 0) {
        u_var3 = pass1_1008_4d72((_PTR_LOOP_1050_4230 + 0xe));
        u_var4 = (u_var3 >> 0x10);
        i_var1 = u_var3;
        _PTR_LOOP_1050_5b6c = CONCAT12(
            *(i_var1 + 0x3ec),
            CONCAT11(*(i_var1 + 0x3ed), *(i_var1 + 0x3ee)),
        );
        _PTR_LOOP_1050_5b70 = CONCAT12(
            *(i_var1 + 0x3e4),
            CONCAT11(*(i_var1 + 0x3e5), *(i_var1 + 0x3e6)),
        );
        _PTR_LOOP_1050_5b74 = CONCAT12(
            *(i_var1 + 0x3f8),
            CONCAT11(*(i_var1 + 0x3f9), *(i_var1 + 0x3fa)),
        );
        _PTR_LOOP_1050_5b78 = CONCAT12(
            *(i_var1 + 0x94),
            CONCAT11(*(i_var1 + 0x95), *(i_var1 + 0x96)),
        );
    }
    if (param_2_00 < 4) {
        // LAB_1038_acf0:
        i_var2 = GetDlgCtrlID16(dialog_handle);
        if (i_var2 == 0xfd4) {
            u_var4 = _PTR_LOOP_1050_5b70;
            u_var5 = (_PTR_LOOP_1050_5b70 >> 0x10);
            // goto LAB_1038_ad0e;
        }
        if (i_var2 != 0xfd5) {
            if (i_var2 == 0xfd6) {
                u_var4 = _PTR_LOOP_1050_5b6c;
                u_var5 = (_PTR_LOOP_1050_5b6c >> 0x10);
                // goto LAB_1038_ad0e;
            }
            if (i_var2 == 0xfd7) {
                u_var4 = _PTR_LOOP_1050_5b74;
                u_var5 = (_PTR_LOOP_1050_5b74 >> 0x10);
                // goto LAB_1038_ad0e;
            }
        }
    } else {
        if (param_2_00 != 4) {
            if ((param_2_00 == 4) || (1 < param_2_00 - 5)) {
                return;
            }
            // goto LAB_1038_acf0;
        }
    }
    u_var4 = _PTR_LOOP_1050_5b78;
    u_var5 = (_PTR_LOOP_1050_5b78 >> 0x10);
    // LAB_1038_ad0e:
    SetTextColor16(CONCAT22(u_var5, u_var4), param_5);
    SetBkColor16(0x1000000, param_5);
    return;
}

pub fn set_window_pos_1038_abdc(param_1: u32) {
    let mut u_var1: u16;
    let mut unaff_cs: u16;
    let unaff_ss: HWND16;
    let mut local_12: [u8; 2];
    let mut local_10: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    u_var1 = (param_1 >> 0x10);
    GetWindowRect16(CONCAT22(&local_a, unaff_cs), unaff_ss);
    GetDlgItem16(0xfd7, (param_1 + 6));
    GetWindowRect16(CONCAT22(local_12, 0x1538), unaff_ss);
    local_6 = local_6 - local_a;
    local_4 = (local_10 - local_8) - 2;
    SetWindowPos16(6, local_4, local_6, 0, 0, 0, (param_1 + 6));
    return;
}

pub fn enable_window_1038_a8f8(param_1: i32, param_2: u16, param_3: u16, param_4: u32) {
    let h_wnd: HWND16;
    let enable: bool;

    if (param_3._2_2_ == 0x116) {
        SendDlgItemMessage16(0, 1, 0x401, 0x11a, (param_1 + 6));
        h_wnd = GetDlgItem16(0x11a, (param_1 + 6));
        enable = 0;
    } else {
        if ((param_3._2_2_ == 0x116) || (2 < param_3._2_2_ - 0x117)) {
            post_win_msg_1040_7b3c(param_1, param_2, param_3_00, param_3, param_3._2_2_);
            return;
        }
        h_wnd = GetDlgItem16(0x11a, (param_1 + 6));
        enable = 1;
    }
    EnableWindow16(enable, h_wnd);
    return;
}

pub fn enable_window_1038_a972(param_1: *mut Struct20) {
    let h_wnd: HWND16;
    let mut u_var1: u16;
    let mut i_var2: i32;
    let mut u_var3: u16;

    ui1::win_gui_func_1040_78e2(param_1);
    u_var3 = (param_1 >> 0x10);
    i_var2 = param_1;
    SendDlgItemMessage16(0, 1, 0x401, 0x116, (i_var2 + 6));
    SendDlgItemMessage16(0, 1, 0x401, 0x11a, (i_var2 + 6));
    h_wnd = GetDlgItem16(0x11a, (i_var2 + 6));
    EnableWindow16(0, h_wnd);
    u_var1 = mci_send_command_1008_5c7c(ctx._g_struct_ptr_1050_02a0, 0x40001);
    (i_var2 + 0x8c) = u_var1;
    get_system_metrics_1038_a18c(i_var2, u_var3);
    ShowWindow16(5, (i_var2 + 6));
    return;
}

pub fn win_fn_1038_a788(param_1: u32, param_2: i32) {
    let hwnd: HWND16;
    let mut u_var1: i32;
    let h_wnd: u16;
    let mut i_var2: i32;
    let mut u_var3: u16;
    let mut unaff_ss: u16;
    let ppVar4: *mut pass1_struct_1;
    let pu_var5: Vec<u8>;
    let mut local_58: u16;
    let mut local_56: u16;
    let mut local_54: u16;
    let mut local_52: [u8; 80];

    if (param_2 != 0) {
        u_var3 = (param_1 >> 0x10);
        i_var2 = param_1;
        hwnd = GetDlgItem16(0x115, (i_var2 + 6));
        GetWindowText16(0x50, CONCAT22(unaff_ss, local_52), hwnd);
        u_var1 = get_string_index_1000_3da4(CONCAT22(unaff_ss, local_52));
        if (u_var1 != 0) {
            pu_var5 = local_52;
            ppVar4 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(pu_var5, 2));
            pass1_1010_5fd8(ppVar4, CONCAT22(unaff_ss, pu_var5));
            h_wnd = GetWindowWord16(-8, (i_var2 + 6));
            PostMessage16(0, 0x105, 0x111, h_wnd);
            ui1::destroy_win_1040_7b98(i_var2, u_var3, 1);
        }
    }
    return;
}

pub fn show_window_1038_a6f4(param_1: *mut Struct20) {
    let lp_string: SEGPTR;
    let hwnd: HWND16;
    let mut u_var1: u16;
    let mut i_var2: i32;
    let mut u_var3: u16;
    let ppVar4: *mut pass1_struct_1;
    let LVar5: LRESULT;
    let mut in_stack_0000ffec: u16;
    let mut local_c: u16;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    ui1::win_gui_func_1040_78e2(param_1);
    ppVar4 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(in_stack_0000ffec, 2));
    lp_string = (ppVar4 + 0x68);
    u_var3 = (param_1 >> 0x10);
    i_var2 = param_1;
    hwnd = GetDlgItem16(0x115, (i_var2 + 6));
    SetWindowText16(lp_string, hwnd);
    SetFocus16(hwnd);
    LVar5 = SendMessage16(-0x10000, 0, 0x401, hwnd);
    u_var1 = LVar5;
    get_system_metrics_1038_a18c(param_1);
    mci_send_command_1008_5c7c(ctx._g_struct_ptr_1050_02a0, 0x30001);
    (i_var2 + 0x8c) = u_var1;
    ShowWindow16(5, (i_var2 + 6));
    return;
}

pub fn show_window_1038_a4ee(param_1: *mut Struct20) {
    let lp_string: SEGPTR;

    let hwnd: HWND16;
    let mut i_var1: i32;
    let mut u_var2: u16;
    let ppVar3: *mut pass1_struct_1;
    let mut in_stack_0000ffec: u16;
    let mut local_c: u16;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    ui1::win_gui_func_1040_78e2(param_1);
    mci_send_command_1008_5c7c(ctx._g_struct_ptr_1050_02a0, 0x20001);
    u_var2 = (param_1 >> 0x10);
    i_var1 = param_1;
    (i_var1 + 0x8c) = in_ax;
    ppVar3 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(in_stack_0000ffec, 2));
    lp_string = (ppVar3 + 0x6c);
    hwnd = GetDlgItem16(0x114, (i_var1 + 6));
    SetWindowText16(lp_string, hwnd);
    SetFocus16(hwnd);
    SendMessage16(-0x10000, 0, 0x401, hwnd);
    get_system_metrics_1038_a18c(param_1);
    ShowWindow16(5, (i_var1 + 6));
    return;
}

pub fn win_fn_1038_a584(param_1: u32, param_2: i32) {
    let hwnd: HWND16;
    let mut u_var1: i32;
    let h_wnd: u16;
    let mut i_var2: i32;
    let mut u_var3: u16;
    let mut unaff_ss: u16;
    let paVar4: *mut Struct431;
    let pu_var5: Vec<u8>;
    let mut local_58: u16;
    let mut local_56: u16;
    let mut local_54: u16;
    let mut local_52: [u8; 80];

    if (param_2 != 0) {
        u_var3 = (param_1 >> 0x10);
        i_var2 = param_1;
        hwnd = GetDlgItem16(0x114, (i_var2 + 6));
        GetWindowText16(0x50, CONCAT22(unaff_ss, local_52), hwnd);
        u_var1 = get_string_index_1000_3da4(CONCAT22(unaff_ss, local_52));
        if (u_var1 != 0) {
            pu_var5 = local_52;
            paVar4 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(pu_var5, 2));
            pass1_1010_6006(paVar4, CONCAT22(unaff_ss, pu_var5));
            h_wnd = GetWindowWord16(-8, (i_var2 + 6));
            PostMessage16(0, 0x105, 0x111, h_wnd);
            ui1::destroy_win_1040_7b98(i_var2, u_var3, 1);
        }
    }
    return;
}

pub fn show_window_1038_a396(param_1: *mut Struct20) {
    let mut u_var1: u16;
    let mut u_var2: u16;

    ui1::win_gui_func_1040_78e2(param_1);
    get_system_metrics_1038_a18c(param_1);
    u_var1 = mci_send_command_1008_5c7c(ctx._g_struct_ptr_1050_02a0, 0x10001);
    u_var2 = (param_1 >> 0x10);
    (param_1 + 0x8c) = u_var1;
    ShowWindow16(5, (param_1 + 6));
    return;
}

pub fn post_msg_1038_a3d2(param_1: u32) {
    let h_wnd: u16;
    let mut u_var1: u16;

    u_var1 = (param_1 >> 0x10);
    h_wnd = GetWindowWord16(-8, (param_1 + 6));
    PostMessage16(0, 0x105, 0x111, h_wnd);
    ui1::destroy_win_1040_7b98(param_1, u_var1, 1);
    return;
}

pub fn get_system_metrics_1038_a18c(param_1: u32) {
    let pp_var1: fn();
    let paVar2: *mut Struct199;
    let i_var3: u16;

    let mut u_var4: u16;
    let unaff_ss: HWND16;
    let pu_var5: *mut u16;
    let h_var6: HWND16;
    let pu_var7: Vec<u8>;
    let HVar8: HWND16;
    let mut local_4c: u32;
    let mut local_48: u32;
    let mut local_30: u16;
    let mut local_2e: u16;
    let mut local_2c: u16;
    let mut local_28: u16;
    let mut local_24: u16;
    let mut local_22: u16;
    let mut local_20: u16;
    let mut local_1e: u16;
    let mut local_1c: u16;
    let mut local_1a: [u8; 2];
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: [u8; 6];
    let mut local_6: u16;
    let mut local_4: u16;

    _local_6 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(local_4c, 0x27));
    zero_list_1008_3e38(CONCAT22(unaff_ss, local_c));
    modify_list_1008_3f62(
        CONCAT22(unaff_ss, local_c),
        _local_6 & 0xffff0000 | (_local_6 + 0x52),
    );
    paVar2 = local_c;
    pass1_1008_3e94(
        paVar2,
        CONCAT22(unaff_ss, &local_10),
        CONCAT22(unaff_ss, &local_e),
    );
    mixed_fn_1010_830a(ctx._g_struct_73_1050_14cc, 0x1c0);
    _local_14 = CONCAT22(ctx.dx_reg, paVar2);
    _local_18 = process_struct_1008_4772(CONCAT22(ctx.dx_reg, paVar2));
    pu_var7 = local_1a;
    pu_var5 = &local_1c;
    h_var6 = unaff_ss;
    HVar8 = unaff_ss;
    _local_24 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(pu_var5, 0x48));
    pass1_1008_3e94(
        (_local_24 + 0xe),
        CONCAT22(h_var6, pu_var5),
        CONCAT22(HVar8, pu_var7),
    );
    u_var4 = (_local_24 >> 0x10);
    local_1e = (_local_24 + 10);
    local_20 = (_local_24 + 0xc);
    local_10 = local_10 + (local_20 * 10) / 600 + (_local_18 + 8) + local_1c;
    GetWindowRect16(CONCAT22(&local_2c, 0x1008), unaff_ss);
    i_var3 = GetSystemMetrics16(0);
    local_e = (i_var3 - (local_28 - local_2c)) / 2;
    ui1::move_window_1040_826c(param_1, (param_1 >> 0x10), local_10, local_e);
    local_4c._0_2_ = SUB42(_local_14, 0);
    local_4c._2_2_ = (_local_14 >> 0x10);
    if (_local_14 != 0x0) {
        pp_var1 = *_local_14;
        (**pp_var1)(
            &ctx.PTR_LOOP_1050_1040,
            local_4c,
            local_4c._2_2_,
            1,
            local_4c,
            local_4c._2_2_,
            _local_14,
        );
    }
    return;
}

pub fn dc_func_1038_9ffa(param_1: u32) -> u16 {
    let pp_var1: fn();
    let mut i_var2: i32;
    let mut u_var3: u16;
    let pu_var4: *mut u32;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    u_var3 = (param_1 >> 0x10);
    local_4 = GetDC16((param_1 + 6));
    pu_var4 = mixed_fn_1010_830a(ctx._g_struct_73_1050_14cc, 3);
    unsafe { i_var2 = *pu_var4 };
    pp_var1 = (i_var2 + 8);
    (**pp_var1)(0x1010, pu_var4, &local_4);
    pp_var1 = (i_var2 + 4);
    (**pp_var1)(0x1010, pu_var4, 0x50005, &local_4);
    pp_var1 = (i_var2 + 0xc);
    (**pp_var1)(0x1010, pu_var4, &local_4);
    ReleaseDC16(local_4, (param_1 + 6));
    return 0;
}

pub fn show_window_1038_9fd0(param_1: *mut Struct20) {
    ui1::win_gui_func_1040_78e2(param_1);
    ui1::move_window_1040_826c(param_1, 0xffffffff);
    ShowWindow16(5, (param_1 + 6));
    return;
}

pub fn win_fn_1038_9bc8(param_1: *mut Struct20) {
    let pi_var1: *mut i32;
    let ppc_var2: fn();
    let mut i_var3: i32;
    let i_var4: u16;
    let mut i_var5: i32;
    let hdc: HDC16;
    let mut i32_var6: i32;
    let HVar7: HWND16;
    let mut u_var8: u16;
    let mut u_var9: u16;
    let unaff_ss: HWND16;
    let ppVar10: *mut pass1_struct_1;
    let pu_var11: *mut u16;
    let u_var12: u8;
    let u_var13: u8;
    let HVar14: HWND16;
    let mut in_stack_0000ffc2: u32;
    let mut u_var15: u16;
    let in_string_1: String;
    let mut local_24: u16;
    let mut local_22: u16;
    let mut local_20: u16;
    let mut local_1e: u16;
    let mut local_1c: u16;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_16: [u8; 2];
    let mut local_14: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    u_var15 = (in_stack_0000ffc2 >> 0x10);
    ui1::win_gui_func_1040_78e2(param_1);
    if (PTR_LOOP_1050_5ef8 == (&PTR_DAT_0005_0000_1050_0004 + 1)) {
        PTR_LOOP_1050_5ef8 = 0x0;
    }
    u_var12 = SUB21(&local_4, 0);
    u_var13 = (&local_4 >> 8);
    pu_var11 = &local_6;
    HVar7 = unaff_ss;
    HVar14 = unaff_ss;
    _local_a = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(pu_var11, 0x48));
    pass1_1008_3e94(
        (_local_a + 0xe),
        CONCAT22(HVar7, pu_var11),
        CONCAT22(HVar14, CONCAT11(u_var13, u_var12)),
    );
    i_var4 = GetSystemMetrics16(4);
    i_var5 = i_var4 * PTR_LOOP_1050_5ef8 + 10;
    PTR_LOOP_1050_5ef8 = PTR_LOOP_1050_5ef8 + 1;
    _local_e = CONCAT22(i_var5 + local_4, i_var5 + local_6);
    u_var8 = (param_1 >> 0x10);
    i_var5 = param_1;
    GetWindowRect16(
        CONCAT13((local_16 >> 8), CONCAT12(local_16, 0x1538)),
        unaff_ss,
    );
    hdc = GetDC16(0);
    i_var4 = GetDeviceCaps16(10, hdc);
    ReleaseDC16(hdc, 0);
    if (i_var4 < local_10) {
        _local_e = _local_e & 0xffff0000 | ((local_14 - (local_10 - i_var4)) + 1);
    }
    SetWindowPos16(1, 0, 0, _local_e, (_local_e >> 0x10), 0, (i_var5 + 6));
    in_string_1 = CONCAT22(u_var15, 3);
    u_var9 = 0x1010;
    ppVar10 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, in_string_1);
    i32_var6 = ppVar10 + 0xa4;
    u_var15 = (ppVar10 >> 0x10);
    local_24 = 0;
    while (i_var3 = local_24 * 2, (i_var3 + i32_var6) != 0) {
        in_string_1 = (in_string_1 & 0xffff0000);
        u_var9 = SUB42(offset, 0);
        HVar7 = GetDlgItem16((i_var3 + i32_var6), (i_var5 + 6));
        (i_var5 + i_var3 + 0x94) = HVar7;
        local_24 = local_24 + 1;
        pi_var1 = (i_var5 + 0x128);
        unsafe {
            *pi_var1 = *pi_var1 + 1;
        }
    }
    ppc_var2 = (param_1 + 0x6c);
    ppc_var2(u_var9, i_var5, u_var8, (in_string_1 >> 0x10));
    return;
}

pub fn enable_window_1038_9cec(
    param_1: *mut Struct124,
    param_2: u16,
    param_3: u16,
    param_4: u16,
    param_5: i32,
) {
    let paVar1: *mut Struct124;
    let mut i_var2: i32;
    let mut u_var3: u16;
    let mut i_var4: i32;
    let mut unaff_si: u16;
    let ppVar5: *mut pass1_struct_1;
    let enable: bool;
    let h_var6: HWND16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;

    if (param_5 == 0xeb) {
        ui1::win_gui_fn_1040_b54a(param_1, param_2, param_3_00, CONCAT22(0xeb, param_3));
        ppVar5 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(unaff_si, 3));
        i_var4 = ppVar5 + 0xa4;
        u_var3 = (ppVar5 >> 0x10);
        local_c = 0;
        while (i_var2 = local_c * 2, (i_var2 + i_var4) != 0) {
            h_var6 = GetDlgItem16((i_var2 + i_var4), &param_1.field_0x6);
            (&param_1[1].field_0x0 + i_var2) = h_var6;
            local_c = local_c + 1;
            paVar1 = (param_1 + 2);
            paVar1 = paVar1 + 1;
        }
    } else {
        if (param_5 == 0xf8) {
            h_var6 = GetDlgItem16(0x17d8, &param_1.field_0x6);
            enable = 1;
        } else {
            if (param_5 != 0x17d8) {
                ui1::win_gui_fn_1040_b54a(param_1, param_2, param_3_00, CONCAT22(param_5, param_3));
                return;
            }
            SetWindowPos16(6, 0xed, 0x237, 0, 0, 0, &param_1.field_0x6);
            enable = offset;
            GetDlgItem16(0x17d8, &param_1.field_0x6);
            h_var6 = 0;
        }
        EnableWindow16(enable, h_var6);
    }
    return;
}

pub fn enable_window_1038_9a66(param_1: *mut Struct124, param_2: u16, param_3: u16, param_4: u32) {
    let enable: bool;
    let h_wnd: HWND16;

    if (param_3._2_2_ == 0xf8) {
        h_wnd = GetDlgItem16(0x17d9, &param_1.field_0x6);
        enable = 1;
    } else {
        if (param_3._2_2_ != 0x17d9) {
            ui1::win_gui_fn_1040_b54a(param_1, param_2, param_3_00, param_3);
            return;
        }
        enable = 0;
        SetWindowPos16(6, 0x1a0, 300, 0, 0, 0, &param_1.field_0x6);
        h_wnd = 0;
    }
    EnableWindow16(enable, h_wnd);
    return;
}

pub fn win_fn_1038_977a(param_1: i32, param_2: u16) {
    let pp_var1: fn();
    let mut u_var2: i32;
    let mut u_var3: u16;
    let in_dx: *mut Struct199;
    let mut u_var4: i32;
    let unaff_ss: u16;
    let mut u_var5: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: [u8; 4];
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let local_4: bool;

    local_8 = 0;
    u_var5 = (param_1 + 6);
    u_var2 = GetDlgItemInt16(1, &local_4, unaff_ss, 0xfa8);
    local_6 = u_var2;
    if (u_var2 != 0) {
        process_struct_1000_179c(0xb4, in_dx);
        u_var4 = in_dx | u_var2;
        if (u_var4 == 0) {
            u_var3 = 0;
            u_var4 = 0;
        } else {
            u_var3 = ui1::mixed_1040_8520(
                CONCAT22(in_dx, u_var2),
                (param_1 + 6),
                0x41,
                2,
                0x5db,
                0x5da,
            );
        }
        _local_c = CONCAT22(u_var4, u_var3);
        pass1_1008_941a(CONCAT22(unaff_ss, local_10), 1, 0xc3);
        pp_var1 = (*_local_c + 0x6c);
        local_8 = (**pp_var1)(
            &ctx.PTR_LOOP_1050_1008,
            _local_c,
            (_local_c >> 0x10),
            local_10,
            unaff_ss,
            u_var5,
            u_var2,
        );
    }
    if ((local_8 == 1) || (local_6 == 0)) {
        ui1::destroy_window_1040_b726();
    }
    return;
}

pub fn get_dialog_int_1038_9820(
    param_1: u32,
    param_2: i32,
    param_3: i32,
    param_4: i32,
) -> libc::c_long {
    let pi_var1: *mut i32;
    let lVar2: u32;
    let mut u_var3: u16;
    let mut i_var4: i32;
    let mut i_var5: i32;
    let mut u_var6: i32;
    let mut i_var7: i32;
    let mut u_var8: u16;
    let unaff_ss: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let local_6: bool;
    let local_4: bool;

    u_var8 = (param_1 >> 0x10);
    i_var7 = param_1;
    u_var3 = GetDlgItemInt16(1, &local_4, unaff_ss, (param_4 * 0xe + 0x5a74));
    i_var4 = u_var3 * param_2 * param_3;
    u_var3 = GetDlgItemInt16(1, &local_6, unaff_ss, (param_4 * 0xe + 0x5a76));
    lVar2 = (u_var3 * param_2) * param_3;
    u_var6 = (lVar2 >> 0x10);
    i_var5 = lVar2;
    if ((i_var4 - (i_var7 + 0x94) < 1) && (-1 < (i_var7 + 0x96) - i_var5)) {
        pi_var1 = (i_var7 + 0x94);
        unsafe {
            *pi_var1 = *pi_var1 - i_var4;
        }
        pi_var1 = (i_var7 + 0x96);
        unsafe {
            *pi_var1 = *pi_var1 - i_var5;
        }
        return CONCAT22(u_var6, 1);
    }
    return u_var6 << 0x10;
}

pub fn dialog_item_func_1038_98b4(param_1: u32) {
    let mut u_var1: u16;
    let mut i_var2: i32;
    let unaff_ss: u16;
    let mut u_var3: u16;
    let mut u_var4: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    local_4 = 0;
    local_8 = 0;
    while (i_var2 = param_1, u_var3 = (param_1 >> 0x10), local_8 < 0xf) {
        u_var4 = 1;
        u_var3 = (i_var2 + 6);
        u_var1 = GetDlgItemInt16(1, &local_4, unaff_ss, (local_8 * 0xe + 0x5a72));
        get_dialog_int32_1038_9820(param_1, u_var1, u_var3, u_var4);
        local_8 = local_8 + 1;
    }
    SetDlgItemInt16(1, *(i_var2 + 0x94), 0xfa9, (i_var2 + 6));
    SetDlgItemInt16(1, *(i_var2 + 0x96), 0xfa8, (i_var2 + 6));
    return;
}

pub fn win_fn_1038_95fc(param_1: u32) {
    let pp_var1: fn();
    let mut u_var2: i32;
    let paVar3: *mut Struct199;
    let mut u_var4: u16;
    let mut u_var5: u16;
    let struct_a: *mut Struct199;

    let ctx.dx_reg: *mut Struct199;
    let mut i32_var6: i32;
    let mut unaff_si: u16;
    let mut u_var7: u16;
    let unaff_ss: u16;
    let mut u_var8: u16;
    let mut u_var9: u16;
    let mut u_var10: u16;
    let mut local_1e: u32;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let local_10: bool;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    _local_6 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(unaff_si, 8));
    _local_a = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(unaff_si, 9));
    paVar3 = (_local_a >> 0x10);
    u_var2 = _local_a;
    process_struct_1000_179c(0xc, paVar3);
    struct_a = (paVar3 | u_var2);
    if (struct_a == 0x0) {
        paVar3 = 0x0;
        struct_a = 0x0;
    } else {
        paVar3 = process_struct_1008_574a(CONCAT22(paVar3, u_var2));
    }
    _local_e = CONCAT22(struct_a, paVar3);
    local_14 = 0;
    while (local_14 < 0xf) {
        u_var10 = (param_1 + 6);
        u_var4 = GetDlgItemInt16(1, &local_10, unaff_ss, (local_14 * 0xe + 0x5a72));
        if (u_var4 != 0) {
            if ((local_14 * 0xe + 0x5a7c) < 0x83) {
                u_var5 = u_var4;
                process_struct_1000_179c(8, struct_a);
                _local_18 = CONCAT22(struct_a, u_var5);
                if ((struct_a | u_var5) == 0) {
                    local_1e = 0;
                } else {
                    *_local_18 = ctx.s_1_1050_389a;
                    (u_var5 + 2) = &ctx.PTR_LOOP_1050_1008;
                    *_local_18 = 0xa1c4;
                    (u_var5 + 2) = 0x1010;
                    local_1e = _local_18;
                }
                u_var7 = (local_1e >> 0x10);
                i32_var6 = local_1e;
                *(i32_var6 + 6) = u_var4;
                (i32_var6 + 4) = (local_14 * 0xe + 0x5a7c);
                pp_var1 = (*_local_e + 4);
                (**pp_var1)(
                    0x1000,
                    _local_e,
                    (_local_e >> 0x10),
                    i32_var6,
                    u_var7,
                    u_var10,
                );
                struct_a = ctx.dx_reg;
            } else {
                if ((local_14 * 0xe + 0x5a7c) == 0x89) {
                    u_var9 = (local_14 * 0xe + 0x5a7c);
                    u_var8 = u_var4;
                } else {
                    u_var9 = (local_14 * 0xe + 0x5a7c);
                    u_var8 = 0;
                }
                pass1_1010_6566(_local_a, u_var8, u_var4, u_var9);
                struct_a = ctx.dx_reg;
            }
        }
        local_14 = local_14 + 1;
    }
    (_local_6 + 10) = _local_e;
    PostMessage16(0, 0xed, 0x111, ctx.g_h_window);
    return;
}

pub fn win_fn_1038_9294(param_1: u32) {
    let mut u_var1: u16;
    let mut i_var2: i32;
    let mut u_var3: u16;
    let unaff_ss: u16;
    let local_6: bool;
    let local_4: bool;

    ui1::set_window_pos_1040_b230(param_1);
    u_var3 = (param_1 >> 0x10);
    i_var2 = param_1;
    u_var1 = GetDlgItemInt16(1, &local_4, unaff_ss, 0xfa9);
    *(i_var2 + 0x94) = u_var1;
    u_var1 = GetDlgItemInt16(1, &local_6, unaff_ss, 0xfa8);
    *(i_var2 + 0x96) = u_var1;
    dialog_item_func_1038_98b4(i_var2, u_var3);
    mci_send_command_1008_5c7c(ctx._g_struct_ptr_1050_02a0, 0x950001);
    return;
}

pub fn win_gui_fn_1038_92f6(
    param_1: *mut Struct124,
    param_2_00: Vec<u8>,
    param_3: u16,
    param_2: u32,
) {
    let mut u_var1: u32;
    let ppc_var2: fn();
    let mut u_var3: i32;
    let i_var4: i16;
    let mut u_var5: u32;

    let paVar6: *mut Struct199;
    let mut i_var7: i32;
    let mut unaff_si: u16;
    let mut u_var8: u16;
    let unaff_ss: u16;
    let mut local_1c: u16;
    let mut local_1a: u16;
    let mut local_16: u16;
    let mut local_14: u32;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    if (param_2._2_2_ == 0xeb) {
        _local_6 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(unaff_si, 3));
        paVar6 = (_local_6 >> 0x10);
        u_var5 = &param_1.field_0x90;
        if (u_var5 != 0) {
            local_a = u_var5;
            process_struct_1000_179c(0x18, paVar6);
            u_var3 = u_var5;
            _local_10 = (u_var5 & 0xffff | ZEXT24(paVar6) << 0x10);
            if ((paVar6 | u_var3) == 0) {
                u_var3 = 0;
                paVar6 = 0x0;
            } else {
                process_struct_1040_a598((u_var5 & 0xffff | ZEXT24(paVar6) << 0x10));
                paVar6 = ctx.dx_reg;
            }
            param_1.field_0x90 = u_var3;
            &param_1.field_0x92 = paVar6;
            *&param_1.field_0x90 = 0x11;
            local_c = *&param_1.field_0x90;
            u_var3 = local_c * 10 + 2;
            process_struct_1000_179c(u_var3, paVar6);
            _local_10 = CONCAT22(paVar6, u_var3);
            if ((paVar6 | u_var3) == 0) {
                u_var1 = &param_1.field_0x90;
                (u_var1 + 2) = 0;
            } else {
                *_local_10 = local_c;
                call_fn_ptr_1000_5586(
                    0xa564,
                    &ctx.PTR_LOOP_1050_1040,
                    local_c,
                    10,
                    u_var3 + 2,
                    paVar6,
                );
                u_var1 = &param_1.field_0x90;
                u_var8 = (u_var1 >> 0x10);
                i_var7 = u_var1;
                (i_var7 + 2) = u_var3 + 2;
                (i_var7 + 4) = paVar6;
            }
            u_var8 = (local_a >> 0x10);
            u_var1 = &param_1.field_0x90;
            (u_var1 + 6) = (local_a + 6);
            u_var1 = &param_1.field_0x90;
            (u_var1 + 10) = (local_a + 10);
            u_var1 = &param_1.field_0x90;
            (u_var1 + 0x12) = &param_1.field_0xa;
            u_var8 = 0x1010;
            pass1_1010_a50c(_local_6, 0x10505b42, &param_1.field_0x90);
            local_14 = local_a;
            _local_10 = local_a;
            if (local_a != 0) {
                pass1_1040_a5d0(local_a);
                u_var8 = 0x1000;
                error_check_1000_17ce(local_a);
            }
            ppc_var2 = (CONCAT22(param_2_00, param_1) + 0x70);
            ppc_var2(u_var8, param_1, param_2_00);
        }
    } else {
        if (param_2._2_2_ != 0xf9) {
            ui1::win_gui_fn_1040_b54a(param_1, param_2_00, param_3, param_2);
            return;
        }
        i_var4 = pass1_1038_993a(param_1, param_2_00, param_3);
        if (-1 < i_var4) {
            local_16 = GetDlgItemInt16(1, &local_1a, unaff_ss, (i_var4 * 0xe + 0x5a72));
            if (local_1a != 0) {
                u_var1 = &param_1[1].field_0x4;
                ui1::win_gui_fn_1010_2a32(u_var1, (u_var1 >> 0x10), local_16, (i_var4 * 0xe + 0x5a72));
            }
        }
    }
    return;
}

pub fn dialog_fn_1038_94da(
    param_1: Vec<u8>,
    param_2: Vec<u8>,
    param_3: u16,
    param_4: u16,
    param_5: i32,
) -> u16 {
    let pu_var1: *mut u16;
    let mut i_var2: i32;
    let mut u_var3: u16;
    let hwnd: HWND16;
    let mut i_var4: i32;
    let unaff_ss: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    local_4 = 1;
    local_8 = pass1_1038_993a(param_1, param_2, param_3_00);
    if ((-1 < local_8)
        && (
            u_var3 = GetDlgItemInt16(1, &local_c, unaff_ss, (local_8 * 0xe + 0x5a72)),
            local_c != 0,
        ))
    {
        if (param_3 == 0) {
            local_6 = u_var3 + 1;
        } else {
            local_4 = 0xffff;
            local_6 = u_var3 - 1;
        }
        local_a = (local_6 <= (local_8 * 0xe + 0x5a7a));
        pu_var1 = (local_8 * 0xe + 0x5a78);
        if (unsafe { *pu_var1 != local_6 } && unsafe { local_6 <= *pu_var1 }) {
            local_a = 0;
        }
        i_var2 = local_8 * 0xe;
        hwnd = GetDlgItem16((i_var2 + 0x5a72), (param_1 + 6));
        SetFocus16(hwnd);
        if ((local_a != 0)
            && (
                i_var4 =
                    get_dialog_int32_1038_9820(CONCAT22(param_2, param_1), 1, local_4, local_8),
                i_var4 != 0,
            ))
        {
            SetDlgItemInt16(1, local_6, (i_var2 + 0x5a72), (param_1 + 6));
            SetDlgItemInt16(1, *(param_1 + 0x94), 0xfa9, (param_1 + 6));
            SetDlgItemInt16(1, *(param_1 + 0x96), 0xfa8, (param_1 + 6));
        }
    }
    return 1;
}

pub fn win_gui_fn_1038_8d98(param_1: *mut Struct124, param_2: u16, param_3: u16, param_4: u32) {
    if (param_3._2_2_ == 0xeb) {
        win_fn_1038_8f74(CONCAT22(param_2, param_1));
    } else {
        if (param_3._2_2_ != (s_vrpal_bmp_1050_183a + 7)) {
            ui1::win_gui_fn_1040_b54a(param_1, param_2, param_3_00, param_3);
            return;
        }
        load_Str_1038_8dda(CONCAT22(param_2, param_1));
    }
    return;
}

pub fn load_Str_1038_8dda(param_1: u32) {

    let in_dx: *mut Struct199;
    let mut u_var1: u16;
    let mut unaff_ss: u16;
    let mut local_212: u32;
    let mut local_20e: u32;
    let mut local_20a: u16;
    let mut local_208: u16;
    let mut local_206: [u8; 258];
    let mut local_104: [u8; 258];

    process_struct_1000_179c(0x1000, in_dx);
    load_string_1010_84e0(
        ctx._g_struct_73_1050_14cc,
        (ctx._g_struct_73_1050_14cc >> 0x10),
        0x100,
        CONCAT22(unaff_ss, local_206),
        0x57b,
    );
    load_string_1010_84e0(
        ctx._g_struct_73_1050_14cc,
        (ctx._g_struct_73_1050_14cc >> 0x10),
        0x3ff,
        CONCAT22(in_dx, in_ax),
        0x803,
    );
    load_string_1010_84e0(
        ctx._g_struct_73_1050_14cc,
        (ctx._g_struct_73_1050_14cc >> 0x10),
        0x3ff,
        CONCAT22(unaff_ss, local_104),
        0x804,
    );
    process_string_1000_3cea(CONCAT22(in_dx, in_ax), CONCAT22(unaff_ss, local_104));
    load_string_1010_84e0(
        ctx._g_struct_73_1050_14cc,
        (ctx._g_struct_73_1050_14cc >> 0x10),
        0x3ff,
        CONCAT22(unaff_ss, local_104),
        0x805,
    );
    process_string_1000_3cea(CONCAT22(in_dx, in_ax), CONCAT22(unaff_ss, local_104));
    load_string_1010_84e0(
        ctx._g_struct_73_1050_14cc,
        (ctx._g_struct_73_1050_14cc >> 0x10),
        0x3ff,
        CONCAT22(unaff_ss, local_104),
        0x806,
    );
    process_string_1000_3cea(CONCAT22(in_dx, in_ax), CONCAT22(unaff_ss, local_104));
    load_string_1010_84e0(
        ctx._g_struct_73_1050_14cc,
        (ctx._g_struct_73_1050_14cc >> 0x10),
        0x3ff,
        CONCAT22(unaff_ss, local_104),
        0x807,
    );
    process_string_1000_3cea(CONCAT22(in_dx, in_ax), CONCAT22(unaff_ss, local_104));
    load_string_1010_84e0(
        ctx._g_struct_73_1050_14cc,
        (ctx._g_struct_73_1050_14cc >> 0x10),
        0x3ff,
        CONCAT22(unaff_ss, local_104),
        0x808,
    );
    process_string_1000_3cea(CONCAT22(in_dx, in_ax), CONCAT22(unaff_ss, local_104));
    u_var1 = (param_1 >> 0x10);
    MessageBox16(
        0,
        CONCAT22(unaff_ss, local_206),
        CONCAT22(in_dx, in_ax),
        (param_1 + 6),
    );
    load_string_1010_84e0(
        ctx._g_struct_73_1050_14cc,
        (ctx._g_struct_73_1050_14cc >> 0x10),
        0x3ff,
        CONCAT22(in_dx, in_ax),
        0x809,
    );
    load_string_1010_84e0(
        ctx._g_struct_73_1050_14cc,
        (ctx._g_struct_73_1050_14cc >> 0x10),
        0x3ff,
        CONCAT22(unaff_ss, local_104),
        0x80a,
    );
    process_string_1000_3cea(CONCAT22(in_dx, in_ax), CONCAT22(unaff_ss, local_104));
    MessageBox16(
        0,
        CONCAT22(unaff_ss, local_206),
        CONCAT22(in_dx, in_ax),
        (param_1 + 6),
    );
    error_check_1000_17ce(CONCAT22(in_dx, in_ax));
    return;
}

pub fn win_fn_1038_8f74(param_1: Vec<u8>) -> u8 {
    let mut i_var1: i32;
    let h_wnd: HWND16;
    let mut i_var2: i32;
    let mut u_var3: u16;
    let mut unaff_ss: u16;
    let lVar4: u32;
    let LVar5: LRESULT;
    let enable: bool;
    let mut local_510: u16;
    let mut local_50e: u16;
    let mut local_50c: u16;
    let mut u_stack1290: u32;
    let HStack1286: HWND16;
    let mut local_40c: [u8; 8];
    let mut local_404: u16;
    let mut local_402: u16;

    u_var3 = (param_1 >> 0x10);
    i_var2 = param_1;
    SendDlgItemMessage16(0, 0, 0xb, (s_650_bmp_1050_1859 + 2), (i_var2 + 6));
    SendDlgItemMessage16(0, 0, 0x405, (s_650_bmp_1050_1859 + 2), (i_var2 + 6));
    i_var1 = pass1_1008_c83a((i_var2 + 0x94));
    if (i_var1 == 0) {
        _local_404 = pass1_1008_c85e((i_var2 + 0x94));
        pass1_1008_5784(CONCAT22(unaff_ss, local_40c), _local_404);
        while (true) {
            lVar4 = pass1_1008_5b12(CONCAT22(unaff_ss, local_40c));
            if (lVar4 == 0) {
                break;
            }
            wsprintf16(
                &local_50c,
                CONCAT22(0x5a6c, unaff_ss),
                CONCAT22((lVar4 + 4), 0x1050),
            );
            HStack1286 = (i_var2 + 6);
            u_stack1290 = 0x185b0401;
            local_50c = 0;
            _local_510 = CONCAT22(unaff_ss, &local_50c);
            SendDlgItemMessage16(_local_510, 0, 0x401, (s_650_bmp_1050_1859 + 2), HStack1286);
        }
        h_wnd = GetDlgItem16(1, (i_var2 + 6));
        enable = 1;
    } else {
        load_string_1010_84e0(
            ctx._g_struct_73_1050_14cc,
            (ctx._g_struct_73_1050_14cc >> 0x10),
            0x3ff,
            CONCAT22(unaff_ss, &local_404),
            0x452,
        );
        SendDlgItemMessage16(
            CONCAT22(unaff_ss, &local_404),
            0,
            0x401,
            (s_650_bmp_1050_1859 + 2),
            (i_var2 + 6),
        );
        h_wnd = GetDlgItem16(1, (i_var2 + 6));
        enable = 0;
    }
    EnableWindow16(enable, h_wnd);
    LVar5 = SendDlgItemMessage16(0, 1, 0xb, (s_650_bmp_1050_1859 + 2), (i_var2 + 6));
    return LVar5;
}

pub fn send_dlg_item_msg_1038_8d22(param_1: u32) {
    let mut i_var1: i32;
    let mut u_var2: u16;
    let mut unaff_ss: u16;
    let LVar3: LRESULT;
    let mut local_106: [u8; 256];
    let mut local_6: u16;
    let mut local_4: u16;

    u_var2 = (param_1 >> 0x10);
    i_var1 = param_1;
    LVar3 = SendDlgItemMessage16(0, 0, 0x409, (s_650_bmp_1050_1859 + 2), (i_var1 + 6));
    local_6 = LVar3;
    local_4 = local_6 >> 0xf;
    if (local_6 != 0xffff) {
        SendDlgItemMessage16(
            CONCAT22(unaff_ss, local_106),
            local_6,
            0x40a,
            (s_650_bmp_1050_1859 + 2),
            (i_var1 + 6),
        );
        pass1_1008_c79a((i_var1 + 0x94), CONCAT22(unaff_ss, local_106));
    }
    return;
}

pub fn send_dlg_item_msg_1038_8b58(param_1: Vec<u8>) -> u8 {
    let mut u_var1: u32;
    let paVar2: *mut Struct1096;
    let mut i_var3: i32;
    let mut u_var4: u16;
    let mut unaff_ss: u16;
    let mut in_stack_0000feee: u16;
    let mut local_10a: u32;
    let mut local_106: [u8; 256];
    let mut local_6: u16;
    let mut local_4: u16;

    _local_6 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(in_stack_0000feee, 3));
    u_var4 = (param_1 >> 0x10);
    i_var3 = param_1;
    pass1_1010_c3c2(
        _local_6,
        (_local_6 >> 0x10),
        CONCAT22(unaff_ss, local_106),
        (i_var3 + 0x94),
    );
    SendDlgItemMessage16(
        CONCAT22(unaff_ss, local_106),
        0,
        0xc,
        (s_dibtext_bmp_1050_1844 + 2),
        (i_var3 + 6),
    );
    u_var1 = (i_var3 + 0x94);
    (i_var3 + 0x9c) = (u_var1 + 0x32);
    (i_var3 + 0x9a) = (i_var3 + 0x9c);
    SetDlgItemInt16(
        0,
        *(i_var3 + 0x9c),
        (s_dibtext_bmp_1050_1844 + 9),
        (i_var3 + 6),
    );
    u_var1 = (i_var3 + 0x94);
    paVar2 = (u_var1 + 0x2e);
    pass1_1038_3aa6(paVar2);
    *(i_var3 + 0x98) = paVar2;
    SetDlgItemInt16(0, paVar2, (s_dibtext_bmp_1050_1844 + 0xb), (i_var3 + 6));
    return paVar2;
}

pub fn msg_box_1038_8a3a(param_1: u32) {

    let in_dx: *mut Struct199;
    let mut unaff_ss: u16;
    let mut local_212: u32;
    let mut local_20e: u32;
    let mut local_20a: [u8; 258];
    let mut local_108: u16;
    let mut local_106: u16;
    let mut local_104: [u8; 258];

    process_struct_1000_179c(0x1000, in_dx);
    _local_108 = CONCAT22(in_dx, in_ax);
    load_string_1010_84e0(
        ctx._g_struct_73_1050_14cc,
        (ctx._g_struct_73_1050_14cc >> 0x10),
        0x3ff,
        CONCAT22(in_dx, in_ax),
        0x7e6,
    );
    load_string_1010_84e0(
        ctx._g_struct_73_1050_14cc,
        (ctx._g_struct_73_1050_14cc >> 0x10),
        0x3ff,
        CONCAT22(unaff_ss, local_104),
        0x7e7,
    );
    process_string_1000_3cea(_local_108, CONCAT22(unaff_ss, local_104));
    load_string_1010_84e0(
        ctx._g_struct_73_1050_14cc,
        (ctx._g_struct_73_1050_14cc >> 0x10),
        0x3ff,
        CONCAT22(unaff_ss, local_104),
        0x7e8,
    );
    process_string_1000_3cea(_local_108, CONCAT22(unaff_ss, local_104));
    load_string_1010_84e0(
        ctx._g_struct_73_1050_14cc,
        (ctx._g_struct_73_1050_14cc >> 0x10),
        0x101,
        CONCAT22(unaff_ss, local_20a),
        0x57b,
    );
    MessageBox16(0, CONCAT22(unaff_ss, local_20a), _local_108, (param_1 + 6));
    error_check_1000_17ce(_local_108);
    return;
}

pub fn set_dialog_item_1038_8966(param_1: u32, param_2: u16, param_3: u16, param_4: i32) -> u16 {
    let pi_var1: *mut i32;
    let mut b_var2: bool;
    let mut i_var3: i32;
    let mut u_var4: u16;
    let mut local_4: u16;

    b_var2 = false;
    i_var3 = param_1;
    u_var4 = (param_1 >> 0x10);
    if (param_2 == 0) {
        if ((i_var3 + 0x98) < 1) {}
        // goto LAB_1038_89af;
        pi_var1 = (i_var3 + 0x9a);
        unsafe {
            *pi_var1 = *pi_var1 + 1;
        }
        pi_var1 = (i_var3 + 0x98);
        unsafe {
            *pi_var1 = *pi_var1 + -1;
        }
    } else {
        if (param_2 != 1) {}
        // goto LAB_1038_89af;
        if ((i_var3 + 0x9a) < 1) {}
        // goto LAB_1038_89af;
        pi_var1 = (i_var3 + 0x9a);
        unsafe {
            *pi_var1 = *pi_var1 + -1;
        }
        pi_var1 = (i_var3 + 0x98);
        unsafe {
            *pi_var1 = *pi_var1 + 1;
        }
    }
    b_var2 = true;
    // LAB_1038_89af:
    if (b_var2) {
        SetDlgItemInt16(
            0,
            *(i_var3 + 0x9a),
            (s_dibtext_bmp_1050_1844 + 9),
            (i_var3 + 6),
        );
        SetDlgItemInt16(
            0,
            *(i_var3 + 0x98),
            (s_dibtext_bmp_1050_1844 + 0xb),
            (i_var3 + 6),
        );
    }
    return 0;
}

pub fn pass1_1038_89e8(param_1: Vec<u8>) {
    send_dlg_item_msg_1038_8b58(param_1);
    return;
}

pub fn win_gui_fn_1038_89f8(param_1: *mut Struct124, param_2: u16, param_3: u16, param_4: u32) {
    if (param_3._2_2_ == 0xeb) {
        send_dlg_item_msg_1038_8b58(CONCAT22(param_2, param_1));
    } else {
        if (param_3._2_2_ != (s_vrpal_bmp_1050_183a + 7)) {
            ui1::win_gui_fn_1040_b54a(param_1, param_2, param_3_00, param_3);
            return;
        }
        msg_box_1038_8a3a(CONCAT22(param_2, param_1));
    }
    return;
}

pub fn send_dialog_item_msg_1038_844a(param_1: u32) -> LRESULT {
    let hwnd: HWND16;
    let b_var1: bool;
    let mut u_var2: u16;
    let mut i_var3: i32;
    let mut u_var4: u16;
    let mut unaff_ss: u16;
    let LVar5: LRESULT;
    let mut local_10a: u16;
    let mut local_108: [u8; 258];
    let mut local_6: u16;
    let mut local_4: u16;

    u_var4 = (param_1 >> 0x10);
    i_var3 = param_1;
    SendDlgItemMessage16(0, 0, 0xb, (s_logo_bmp_1050_1850 + 4), (i_var3 + 6));
    SendDlgItemMessage16(0, 0, 0xb, (s_logo_bmp_1050_1850 + 5), (i_var3 + 6));
    SendDlgItemMessage16(0, 0, 0xb, (s_logo_bmp_1050_1850 + 6), (i_var3 + 6));
    SendDlgItemMessage16(0, 0, 0x405, (s_logo_bmp_1050_1850 + 4), (i_var3 + 6));
    SendDlgItemMessage16(0, 0, 0x405, (s_logo_bmp_1050_1850 + 5), (i_var3 + 6));
    SendDlgItemMessage16(0, 0, 0x405, (s_logo_bmp_1050_1850 + 6), (i_var3 + 6));
    _local_6 = pass1_1008_b820((i_var3 + 0x94));
    if (_local_6 == 0) {
        load_string_1010_84e0(
            ctx._g_struct_73_1050_14cc,
            (ctx._g_struct_73_1050_14cc >> 0x10),
            0x100,
            CONCAT22(unaff_ss, local_108),
            0x448,
        );
        SendDlgItemMessage16(
            CONCAT22(unaff_ss, local_108),
            0,
            0x401,
            (s_logo_bmp_1050_1850 + 4),
            (i_var3 + 6),
        );
        SendDlgItemMessage16(0, 1, 0xb, (s_logo_bmp_1050_1850 + 4), (i_var3 + 6));
        SendDlgItemMessage16(0, 1, 0xb, (s_logo_bmp_1050_1850 + 5), (i_var3 + 6));
        LVar5 = SendDlgItemMessage16(0, 1, 0xb, (s_logo_bmp_1050_1850 + 6), (i_var3 + 6));
        u_var2 = (LVar5 >> 0x10);
        hwnd = GetDlgItem16((s_logo_bmp_1050_1850 + 7), (i_var3 + 6));
        load_string_1010_84e0(
            ctx._g_struct_73_1050_14cc,
            (ctx._g_struct_73_1050_14cc >> 0x10),
            0x100,
            CONCAT22(unaff_ss, local_108),
            0x449,
        );
        b_var1 = SetWindowText16(CONCAT22(unaff_ss, local_108), hwnd);
        return CONCAT22(u_var2, b_var1);
    }
    send_dialog_item_msg_1038_8400(
        i_var3,
        (param_1 >> 0x10),
        _local_6,
        (s_logo_bmp_1050_1850 + 4),
    );
    set_window_text_1038_8358(i_var3, u_var4);
    SendDlgItemMessage16(0, 1, 0xb, (s_logo_bmp_1050_1850 + 4), (i_var3 + 6));
    SendDlgItemMessage16(0, 1, 0xb, (s_logo_bmp_1050_1850 + 5), (i_var3 + 6));
    LVar5 = SendDlgItemMessage16(0, 1, 0xb, (s_logo_bmp_1050_1850 + 6), (i_var3 + 6));
    return LVar5;
}

pub fn set_window_text_1038_8358(param_1: Vec<u8>) {
    let pu_var1: Vec<u8>;
    let mut i_var2: i32;
    let mut i_var3: i32;
    let mut u_var4: u16;
    let mut unaff_ss: u16;
    let mut local_30a: [u8; 258];
    let mut local_208: [u8; 256];
    let mut local_108: [u8; 256];
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    u_var4 = (param_1 >> 0x10);
    i_var3 = param_1;
    local_4 = GetDlgItem16((s_logo_bmp_1050_1850 + 7), (i_var3 + 6));
    _local_8 = pass1_1008_b820((i_var3 + 0x94));
    if (_local_8 == 0) {
        load_string_1010_84e0(
            ctx._g_struct_73_1050_14cc,
            (ctx._g_struct_73_1050_14cc >> 0x10),
            0x100,
            CONCAT22(unaff_ss, local_30a),
            0x449,
        );
        pu_var1 = local_30a;
    } else {
        i_var2 = send_dlg_item_msg_1038_8164(
            param_1,
            CONCAT22(unaff_ss, local_108),
            (s_logo_bmp_1050_1850 + 4),
        );
        if (i_var2 == 0) {
            load_string_1010_84e0(
                ctx._g_struct_73_1050_14cc,
                (ctx._g_struct_73_1050_14cc >> 0x10),
                0x100,
                CONCAT22(unaff_ss, local_208),
                0x447,
            );
        } else {
            load_string_1008_b65a(
                (i_var3 + 0x94),
                CONCAT22(unaff_ss, local_208),
                CONCAT22(unaff_ss, local_108),
            );
        }
        pu_var1 = local_208;
    }
    SetWindowText16(CONCAT22(unaff_ss, pu_var1), local_4);
    return;
}

pub fn enable_window_1038_806a(param_1: u32) {
    let HVar1: HWND16;
    let mut i_var2: i32;
    let mut u_var3: u16;
    let mut u_var4: u32;
    let mut u_var5: u32;
    let mut u_var6: u32;
    let mut local_12: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;

    u_var3 = (param_1 >> 0x10);
    i_var2 = param_1;
    HVar1 = GetDlgItem16(1, (i_var2 + 6));
    EnableWindow16(0, HVar1);
    HVar1 = GetDlgItem16((s_logo_bmp_1050_1850 + 8), (i_var2 + 6));
    EnableWindow16(0, HVar1);
    HVar1 = GetDlgItem16(s_650_bmp_1050_1859, (i_var2 + 6));
    EnableWindow16(0, HVar1);
    u_var4 = pass1_1008_b820((i_var2 + 0x94));
    if (u_var4 != 0) {
        u_var4 = pass1_1008_b340((i_var2 + 0x94));
        u_var5 = pass1_1008_b366((i_var2 + 0x94));
        u_var6 = pass1_1008_b47a((i_var2 + 0x94));
        if (((u_var4 != 0) && (u_var5 != 0)) && (u_var6 != 0)) {
            HVar1 = GetDlgItem16(1, (i_var2 + 6));
            EnableWindow16(1, HVar1);
            HVar1 = GetDlgItem16((s_logo_bmp_1050_1850 + 8), (i_var2 + 6));
            EnableWindow16(1, HVar1);
        }
        if (u_var4 != 0) {
            HVar1 = GetDlgItem16(s_650_bmp_1050_1859, (i_var2 + 6));
            EnableWindow16(1, HVar1);
        }
    }
    return;
}

pub fn send_dlg_item_msg_1038_8164(param_1: Vec<u8>, param_2: Vec<u8>, param_3: u16) -> u16 {
    let mut u_var1: u16;
    let LVar2: LRESULT;

    unsafe {
        *param_2 = '\0';
    }
    u_var1 = (param_1 >> 0x10);
    LVar2 = SendDlgItemMessage16(0, 0, 0x409, param_3, (param_1 + 6));
    if ((LVar2 != -1)
        && (
            LVar2 = SendDlgItemMessage16(param_2, LVar2, 0x40a, param_3, (param_1 + 6)),
            LVar2 != -1,
        ))
    {
        return 1;
    }
    return 0;
}

pub fn win_gui_fn_1038_7dc6(param_1: *mut Struct124, param_2: u16, param_3: u16, param_4: u32) {
    let mut bVar1: bool;
    let mut local_4: u16;

    bVar1 = false;
    if (param_2._2_2_ == (s_logo_bmp_1050_1850 + 4)) {
        if (param_2 != 1) {}
        // goto LAB_1038_7e8c;
        send_dlg_item_msg_1038_8618(CONCAT22(param_2_00, param_1));
    } else {
        if (param_2._2_2_ < (s_logo_bmp_1050_1850 + 5)) {
            if (param_2._2_2_ == 0xeb) {
                send_dialog_item_msg_1038_844a(CONCAT22(param_2_00, param_1));
            } else {
                if (param_2._2_2_ == 0xfb) {
                    send_dlg_item_msg_1038_7eac(CONCAT22(param_2_00, param_1));
                } else {
                    if (param_2._2_2_ != (s_vrpal_bmp_1050_183a + 7)) {
                        // LAB_1038_7e77:
                        ui1::win_gui_fn_1040_b54a(param_1, param_2_00, param_3, param_2);
                        return;
                    }
                    load_str_1038_81be(CONCAT22(param_2_00, param_1));
                }
            }
            // goto LAB_1038_7e8c;
        }
        if (param_2._2_2_ == (s_logo_bmp_1050_1850 + 5)) {
            if (param_2 != 1) {}
            // goto LAB_1038_7e8c;
            send_dlg_item_msg_1038_87b2(CONCAT22(param_2_00, param_1));
        } else {
            if (param_2._2_2_ == (s_logo_bmp_1050_1850 + 6)) {
                if (param_2 != 1) {}
                // goto LAB_1038_7e8c;
                pass1_1038_8810(CONCAT22(param_2_00, param_1));
            } else {
                if (param_2._2_2_ == (s_logo_bmp_1050_1850 + 8)) {
                    send_dlg_item_msg_1038_7fae(CONCAT22(param_2_00, param_1));
                } else {
                    if (param_2._2_2_ != s_650_bmp_1050_1859) {}
                    // goto LAB_1038_7e77;
                    pass1_1038_801a(CONCAT22(param_2_00, param_1));
                }
            }
        }
    }
    bVar1 = true;
    // LAB_1038_7e8c:
    if (bVar1) {
        set_window_text_1038_8358(CONCAT22(param_2_00, param_1));
        enable_window_1038_806a(CONCAT22(param_2_00, param_1));
    }
    return;
}

pub fn send_dlg_item_msg_1038_7eac(param_1: u32) -> LRESULT {
    let mut i_var1: i32;
    let mut u_var2: u16;
    let ppVar3: *mut pass1_struct_1;
    let mut l_param: u32;
    let LVar4: LRESULT;
    let mut in_stack_0000fff2: u16;
    let mut local_a: u16;
    let mut local_8: u16;

    ppVar3 = process_struct_1010_20ba(
        ctx._g_Struct372_1050_0ed0,
        CONCAT22(in_stack_0000fff2, 0x30),
    );
    l_param = pass1_1010_375e(ppVar3);
    u_var2 = (param_1 >> 0x10);
    i_var1 = param_1;
    pass1_1008_b1a6((i_var1 + 0x94), l_param);
    SendDlgItemMessage16(0, 0, 0xb, (s_logo_bmp_1050_1850 + 4), (i_var1 + 6));
    LVar4 = SendDlgItemMessage16(0, 0, 0x409, (s_logo_bmp_1050_1850 + 4), (i_var1 + 6));
    if (LVar4 != -1) {
        SendDlgItemMessage16(0, LVar4, 0x403, (s_logo_bmp_1050_1850 + 4), (i_var1 + 6));
        SendDlgItemMessage16(l_param, 0, 0x401, (s_logo_bmp_1050_1850 + 4), (i_var1 + 6));
        SendDlgItemMessage16(0, 0xffff, 0x407, (s_logo_bmp_1050_1850 + 4), (i_var1 + 6));
        SendDlgItemMessage16(0, 0, 0x405, (s_logo_bmp_1050_1850 + 5), (i_var1 + 6));
        SendDlgItemMessage16(0, 0, 0x405, (s_logo_bmp_1050_1850 + 6), (i_var1 + 6));
        enable_window_1038_806a(i_var1, u_var2);
    }
    LVar4 = SendDlgItemMessage16(0, 1, 0xb, (s_logo_bmp_1050_1850 + 4), (i_var1 + 6));
    return LVar4;
}

pub fn send_dlg_item_msg_1038_7fae(param_1: u32) {
    let mut i_var1: i32;
    let mut u_var2: u16;

    u_var2 = (param_1 >> 0x10);
    i_var1 = param_1;
    pass1_1008_b146((i_var1 + 0x94));
    SendDlgItemMessage16(0, 0xffff, 0x407, (s_logo_bmp_1050_1850 + 5), (i_var1 + 6));
    SendDlgItemMessage16(0, 0xffff, 0x407, (s_logo_bmp_1050_1850 + 6), (i_var1 + 6));
    pass1_1008_b61a((i_var1 + 0x94), 0);
    pass1_1008_b63a(*(i_var1 + 0x94), 0x0);
    return;
}

pub fn call_load_cursor_1020_790e(
    in_struct_1: *mut win_struct_42,
    param_2: String,
    param_3: u16,
    param_4: u32,
) {
    let local_struct_1: *mut win_struct_42;
    let local_struct_1_hi: *mut win_struct_42;

    ui1::load_cursor_1008_7f62(in_struct_1, param_3, param_4);
    local_struct_1_hi = (in_struct_1 >> 0x10);
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

pub fn destroy_menu_func_1020_795c(in_struct_1: &mut Struct44) {
    let local_struct_1: *mut Struct215;
    let local_struct_1_hi: *mut Struct215;
    let mut menu_handle: u16;

    local_struct_1_hi = (in_struct_1 >> 0x10);
    local_struct_1 = in_struct_1;
    in_struct_1.ptr_a_lo = 0x7b86;
    local_struct_1.ptr_a_hi = 0x1020;
    if (local_struct_1.field_0xec != 0) {
        DestroyMenu16(menu_handle);
    }
    pass1_1008_57c4((in_struct_1 & 0xffff0000 | ZEXT24(&local_struct_1.field_0xd2)));
    in_struct_1.ptr_a_lo = 0x380a;
    local_struct_1.ptr_a_hi = &ctx.PTR_LOOP_1050_1008;
    in_struct_1.ptr_a_lo = ctx.s_1_1050_389a;
    local_struct_1.ptr_a_hi = &ctx.PTR_LOOP_1050_1008;
    return;
}

pub fn win_gui_fn_1020_7824(param_1: *mut Struct622, param_2: u16) {

    let mut i_var1: i32;

    let ppVar2: *mut pass1_struct_1;
    let mut local_e: u16;
    let mut local_6: u32;
    let mut temp_5fa3752684: u32;
    let fn_ptr_1: fn();

    get_dc_1020_921c(param_1, param_2);
    (param_1 + 0x14) = 0;
    param_1.u16_0x0 = 0x7902;
    (param_1 + 2) = 0x1020;
    ppVar2 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(local_e, 0x25));
    ctx.dx_reg = (ppVar2 >> 0x10);
    (param_1 + 0x14) = ppVar2;
    (param_1 + 0x16) = ctx.dx_reg;
    (param_1 + 6) = (param_1 + 0x14);
    (param_1 + 8) = ctx.dx_reg;
    temp_5fa3752684 = (param_1 + 0x14);
    i_var1 = param_1 + 10;
    fn_ptr_1 = ((temp_5fa3752684 + 10) + 8);
    (**fn_ptr_1)();
    (param_1 + 0x12) = i_var1;
    draw_1020_9364((param_1 & 0xffff | param_1._2_2_ << 0x10));
    return;
}

pub fn win_gui_fn_1020_76aa(in_struct_1: *mut win_struct_42) {
    let paVar1: *mut Struct199;
    let struct_a: *mut Struct199;
    let mut u_var2: i32;
    let local_struct_1: *mut win_struct_42;
    let local_struct_1_hi: *mut win_struct_42;
    let paVar3: *mut Struct199;
    let mut local_4: u16;

    paVar3 = create_win_1008_9760(in_struct_1);
    struct_a = (paVar3 >> 0x10);
    local_struct_1_hi = (in_struct_1 >> 0x10);
    local_struct_1 = in_struct_1;
    paVar1 = get_gui_dc_1018_4db0(*&local_struct_1.u32_xf2, local_struct_1.win_handle_0x8);
    process_struct_1000_179c(0x18, struct_a);
    u_var2 = struct_a | paVar1;
    if (u_var2 != 0) {
        win_gui_fn_1020_7824(CONCAT22(struct_a, paVar1), local_struct_1.win_handle_0x8);
        local_struct_1.char_ptr_16_0xee = paVar1;
        local_struct_1.field_0xf0 = u_var2;
        return;
    }
    &local_struct_1.char_ptr_16_0xee = 0;
    return;
}

pub fn win_gui_fn_1020_75f0(param_1: *mut Struct675) {
    let pi_var1: *mut i32;
    let mut u_var2: i32;
    let mut u_var3: u32;
    let struct_a: *mut Struct199;
    let paVar4: *mut Struct199;

    let local_bx_4: *mut Struct675;
    let local_es_4: *mut Struct675;
    let mut unaff_ss: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: [u8; 4];
    let fn_ptr_1: fn();

    local_es_4 = (param_1 >> 0x10);
    local_bx_4 = param_1;
    if (local_bx_4.field_0xee != 0) {
        fn_ptr_1 = (local_bx_4.field_0xee + 8);
        (**fn_ptr_1)();
    }
    if (local_bx_4.field_0xea == 0) {
        local_bx_4.field_0xea = 1;
        pass1_1008_941a(CONCAT22(unaff_ss, local_6), 1, 0x91);
        u_var3 = ZEXT24(local_6);
        mci_send_command_1008_5c9e(ctx._g_struct_ptr_1050_02a0, CONCAT22(unaff_ss, local_6));
        local_bx_4.field_0xec = u_var3;
        paVar4 = struct_a;
        process_struct_1000_179c(0x112, struct_a);
        if ((paVar4 | u_var3) == 0) {
            u_var2 = 0;
            _local_a = 0x0;
        } else {
            pi_var1 = &local_bx_4.field_0xcc;
            unsafe {
                *pi_var1 = *pi_var1 + 1;
            }
            win_gui_fn_1020_3644(u_var3, paVar4, local_bx_4.field_0xcc, param_1);
            u_var2 = u_var3;
            _local_a = (u_var3 & 0xffff | ctx.dx_reg << 0x10);
        }
        pass1_1008_6978(param_1, 0, _local_a & 0xffff0000 | u_var2);
        fn_ptr_1 = (*_local_a + 0xc);
        (**fn_ptr_1)(8, _local_a, local_8, 5);
    }
    return;
}

pub fn win_gui_fn_1020_43f6(in_struct_1: *mut win_struct_42) {
    let ppVar1: *mut pass1_struct_1;
    let lVar2: u32;
    let mut u_var3: u32;
    let local_struct_1: *mut win_struct_42;
    let local_struct_1_hi: *mut win_struct_42;
    let mut uStack10: u16;
    let mut local_8: u16;
    let fn_ptr_1: fn();

    local_struct_1 = in_struct_1;
    local_struct_1_hi = (in_struct_1 >> 0x10);
    create_win_1008_9760(in_struct_1);
    get_gui_dc_1018_4db0(local_struct_1.u32_xfa, local_struct_1.win_handle_0x8);
    ppVar1 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(uStack10, 0x32));
    &local_struct_1.field_0x10e = ppVar1;
    (&local_struct_1.field_0x10e + 2) = (ppVar1 >> 0x10);
    fn_ptr_1 = (local_struct_1.field_0x10e + 4);
    lVar2 = (**fn_ptr_1)();
    process_struct_1000_179c(0x30, (lVar2 >> 0x10));
    if (lVar2 == 0) {
        local_struct_1.u32_xf6 = 0;
    } else {
        u_var3 = process_struct_1020_62e0(
            lVar2,
            CONCAT22(local_struct_1.win_handle_0x8, (lVar2 >> 0x10)),
        );
        &local_struct_1.u32_xf6 = u_var3;
        (&local_struct_1.u32_xf6 + 2) = (u_var3 >> 0x10);
    }
    gui_window_func_1020_536e(in_struct_1, 0, 0x3ffff);
    return;
}

pub fn destroy_win_1020_42f4(in_struct_1: &mut Struct44) {
    let mut i_var1: i32;
    let mut u_var2: u16;
    let mut unaff_cs: u16;
    let mut HVar3: u16;

    u_var2 = (in_struct_1 >> 0x10);
    i_var1 = in_struct_1;
    in_struct_1.ptr_a_lo = 0x623c;
    (i_var1 + 2) = 0x1020;
    (i_var1 + 0xe2) = 0x62d8;
    (i_var1 + 0xe4) = 0x1020;
    HVar3 = unaff_cs;
    if ((i_var1 + 0x106) != 0) {
        HVar3 = offset;
        DestroyMenu16(unaff_cs);
    }
    DestroyCursor16(HVar3, (i_var1 + 0xf0));
    DestroyCursor16();
    process_struct_1020_808e(in_struct_1);
    return;
}

pub fn win_gui_fn_1020_434c(
    in_struct_1_1: *mut Struct661,
    in_struct_1_2: *mut Struct661,
    param_3: *mut u32,
    param_4: u16,
    param_5: u16,
    param_6: i32,
) {
    if (param_6 == 1) {
        set_capture_1020_6184(CONCAT22(in_struct_1_2, in_struct_1_1), param_5);
        return;
    }
    if (param_6 == 2) {
        gui_window_func_1020_536e(
            in_struct_1_1,
            in_struct_1_2,
            param_3,
            param_3_00,
            param_5,
            2,
        );
        return;
    }
    pass1_1008_68ea(
        in_struct_1_1,
        in_struct_1_2,
        param_3,
        param_3_00,
        param_5,
        param_6,
    );
    return;
}

pub fn bring_window_to_top_1020_2aae(param_1: &mut Vec<u8>, param_2: u32) {
    let mut u_var1: u16;

    u_var1 = (param_1 >> 0x10);
    pass1_1008_3e0e(param_1);
    BringWindowToTop16((param_1 + 8));
    pass1_1018_169e((param_1 + 0xf2), param_2);
    return;
}

pub fn win_fun_1020_2ae4(param_1: *mut u32, param_2: i32) {
    let mut u_var1: u32;
    let ppc_var2: fn();
    let mut cVar3: u8;
    let mut u_var4: u16;
    let mut u_var5: i32;



    let local_bx_14: *mut Struct4;
    let mut u_var6: u16;
    let pp_var7: *mut pass1_struct_1;
    let w_param: WPARAM16;
    let mut i_var8: i32;
    let mut h_wnd: u16;
    let mut local_c: u16;
    let mut local_a: u16;

    if (param_2 != 0x129) {
        local_bx_14 = param_1;
        u_var6 = (param_1 >> 0x10);
        if (0x129 < param_2) {
            if (param_2 == 0x12a) {
                h_wnd = local_bx_14.field_0x8;
                w_param = 0xf012;
            } else {
                if (param_2 == 299) {
                    return;
                }
                if (param_2 == 300) {
                    h_wnd = local_bx_14.field_0x8;
                    w_param = 0xf020;
                } else {
                    if (param_2 == 0x12d) {
                        return;
                    }
                    if (param_2 != 0x12e) {
                        return;
                    }
                    h_wnd = local_bx_14.field_0x8;
                    w_param = 0xf060;
                }
            }
            PostMessage16(0, w_param, 0x112, h_wnd);
            return;
        }
        if (param_2 == 0xfb) {
            pp_var7 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(local_c, 0x30));
            u_var4 = SUB42(pp_var7, 0);
            pass1_1010_375e(pp_var7);
            unsafe {
                ppc_var2 = (*param_1 + 0x14);
            }
            ppc_var2(
                0x1010,
                local_bx_14,
                (param_1 >> 0x10),
                1,
                u_var4,
                ctx.dx_reg,
            );
            pass1_1010_375e(pp_var7);
            pass1_1018_181c(local_bx_14.field_0xf2, CONCAT22(ctx.dx_reg, u_var4));
            return;
        }
        if (param_2 < 0xfc) {
            cVar3 = param_2;
            u_var5 = param_2 & 0xff00 | (cVar3 + 0x91);
            if ((cVar3 + 0x91) == 0) {
                mixed_fn_1010_830a(ctx._g_struct_73_1050_14cc, 0x1f8);
                WinHelp16(
                    0x2a,
                    1,
                    CONCAT22(ctx.dx_reg, u_var5),
                    local_bx_14.field_0x8,
                );
                return;
            }
            if (cVar3 == 'r') {
                i_var8 = &local_bx_14.field_0xa;
                u_var4 = u_var6;
                pp_var7 =
                    process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(i_var8, 0x30));
                pass1_1010_3770(pp_var7, CONCAT22(u_var4, i_var8));
                pass1_1038_af40(ctx._g_Struct112_a, local_bx_14.field_0x8, 3);
                return;
            }
            if (cVar3 == 'u') {
                u_var1 = local_bx_14.field_0xf2;
                pass1_1018_0a76(u_var1, (u_var1 >> 0x10));
                InvalidateRect16(0, 0x0, 0);
                return;
            }
        }
    }
    return;
}

pub fn enable_menu_item_1020_2c2a() -> bool {
    let b_var1: bool;
    let mut in_stack_0000000a: i32;
    let mut in_stack_0000000c: u16;
    let mut h_menu: u16;

    if (in_stack_0000000a != 0) {
        return in_stack_0000000a - 1;
    }
    EnableMenuItem16(0x400, 3, in_stack_0000000c);
    if (PTR_LOOP_1050_3960 < 2) {
        h_menu = 0x401;
    } else {
        h_menu = 0x400;
    }
    b_var1 = EnableMenuItem16(h_menu, 5, in_stack_0000000c);
    return b_var1;
}

pub fn call_draw_1020_2c72(param_1: &mut Vec<u8>) {
    draw_1020_30be((param_1 + 0xf6));
    return;
}

pub fn destroy_icon_1020_2c88(param_1: *mut Struct652) {
    let pu_var1: *mut u32;
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

pub fn win_fn_1020_2cf0(param_1: *mut win_struct_42) {
    let pp_var1: fn();
    let mut u_var2: i32;
    let pi_var3: *mut u16;
    let mut u_var4: u16;
    let struct_a: *mut Struct199;
    let pa_var5: *mut Struct199;


    let mut i32_var6: i32;
    let mut unaff_si: u16;
    let mut u_var7: i32;
    let pp_var8: *mut pass1_struct_1;
    let mut u_var9: u32;
    let mut local_4: u16;

    create_win_1008_9760(param_1);
    u_var7 = (param_1 >> 0x10);
    i32_var6 = param_1;
    pp_var8 = process_struct_1010_20ba(
        ctx._g_Struct372_1050_0ed0,
        CONCAT22(unaff_si, (i32_var6 + 0xfc)),
    );
    u_var4 = (pp_var8 >> 0x10);
    (i32_var6 + 0xf2) = pp_var8;
    (i32_var6 + 0xf4) = u_var4;
    u_var2 = (i32_var6 + 0xf2);
    (i32_var6 + 0xe0) = u_var2;
    (i32_var6 + 0xe2) = u_var4;
    LoadIcon16(
        0x1010,
        s_SITEICON_1050_428d,
        &ctx.g_alloc_addr_1050_1050,
        ctx.g_h_instance_1050_038c,
    );
    (i32_var6 + 0xc2) = u_var2;
    pp_var1 = ((i32_var6 + 0xf2) + 0x30);
    (**pp_var1)();
    pa_var5 = struct_a;
    process_struct_1000_179c(0x22, struct_a);
    if ((pa_var5 | u_var2) == 0) {
        (i32_var6 + 0xf6) = 0;
    } else {
        gui_win_fn_1020_2ede(u_var2, pa_var5, param_1);
        (i32_var6 + 0xf6) = u_var2;
        (i32_var6 + 0xf8) = ctx.dx_reg;
    }
    (i32_var6 + 0xe8) = (i32_var6 + 0xf6);
    pass1_1018_0ac0((i32_var6 + 0xf2), param_1 & 0xffff | u_var7 << 0x10);
    u_var9 = pass1_1018_0b08((i32_var6 + 0xf2));
    pi_var3 = u_var9;
    pp_var1 = (param_1 + 0x14);
    (**pp_var1)();
    pp_var1 = ((i32_var6 + 0xf2) + 0x10);
    (**pp_var1)();
    MoveWindow16(
        1,
        pi_var3[3],
        pi_var3[2],
        pi_var3[1],
        unsafe { *pi_var3 },
        (i32_var6 + 8),
    );
    pass1_1008_3e0e(param_1);
    UpdateWindow16((i32_var6 + 8));
    return;
}

pub fn call_cleanup_fn_1020_2e24(in_struct_1: *mut Struct376, param_2: u8) -> *mut Struct376 {
    cleanup_fn_1020_28fc(in_struct_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(in_struct_1);
    }
    return in_struct_1;
}

pub fn gui_win_fn_1020_2ede(param_1: *mut u16, param_2: u32) {
    let mut u_var1: u32;
    let ppc_var2: fn();
    let HVar3: HDC16;
    let mut i_var4: i32;
    let obj_handle: HPEN16;
    let HVar5: HGDIOBJ16;
    let ppVar6: *mut pass1_struct_1;
    let in_struct_104_ptr: *mut Struct104;
    let mut u_var7: u32;
    let u_var8: u8;
    let u_var9: u8;
    let mut u_var10: u16;
    let mut u_var11: u16;
    let mut u_var12: u16;
    let u_var13: u8;
    let u_var14: u8;
    let mut i_var15: i32;
    let mut u_var16: u16;
    let mut in_stack_0000ffea: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    u_var10 = (param_2 >> 0x10);
    i_var15 = param_1;
    u_var16 = (param_1 >> 0x10);
    get_sys_metrics_1020_7c1a(i_var15, u_var16, param_2, u_var10);
    (i_var15 + 0x14) = 0;
    (i_var15 + 0x18) = 0;
    (i_var15 + 0x1a) = 0;
    (i_var15 + 0x1c) = 0;
    (i_var15 + 0x1e) = 0;
    (i_var15 + 0x20) = 0;
    unsafe {
        *param_1 = 0x363c;
    }
    (i_var15 + 2) = 0x1020;
    ppVar6 = process_struct_1010_20ba(
        ctx._g_Struct372_1050_0ed0,
        CONCAT22(in_stack_0000ffea, (param_2 + 0xfc)),
    );
    (i_var15 + 0x14) = ppVar6;
    (i_var15 + 0x16) = (ppVar6 >> 0x10);
    u_var1 = (i_var15 + 0x14);
    ppc_var2 = ((i_var15 + 0x14) + 4);
    ppc_var2(0x1010, u_var1, (u_var1 >> 0x10), 0, i_var15, u_var16);
    u_var13 = 0xc2;
    u_var14 = 0x42;
    u_var11 = 0;
    u_var12 = 0;
    u_var8 = 0;
    u_var9 = 0;
    u_var10 = 0;
    in_struct_104_ptr = pass1_1018_0a50((i_var15 + 0x14));
    u_var7 = process_struct_1008_4772(in_struct_104_ptr);
    HVar3 = CreateDC16(
        u_var7,
        CONCAT13(u_var9, CONCAT12(u_var8, (u_var7 >> 0x10))),
        CONCAT22(u_var11, u_var10),
        CONCAT13(u_var14, CONCAT12(u_var13, u_var12)),
    );
    (i_var15 + 0x18) = HVar3;
    i_var4 = i_var15 + 0x18;
    ppc_var2 = (in_struct_104_ptr + 8);
    ppc_var2(
        offset,
        in_struct_104_ptr,
        (in_struct_104_ptr >> 0x10),
        i_var4,
        u_var16,
    );
    (i_var15 + 0x20) = i_var4;
    u_var1 = (i_var15 + 0x14);
    obj_handle = CreatePen16((u_var1 + 100), 1, 0);
    (i_var15 + 0x1a) = obj_handle;
    HVar5 = SelectObject16(obj_handle, (i_var15 + 0x18));
    (i_var15 + 0x1c) = HVar5;
    HVar5 = GetStockObject16(5);
    HVar5 = SelectObject16(HVar5, (i_var15 + 0x18));
    (i_var15 + 0x1e) = HVar5;
    return;
}

pub fn win_cleanup_func_1020_2fea(param_1: &mut Struct44) {
    let h_gdi_obj: HPALETTE16;
    let local_bx_4: &mut Struct44;
    let mut in_stack_00000006: u16;

    *_param_1 = 0x363c;
    param_1.ptr_a_hi = 0x1020;
    if (param_1.field_0x14 != 0) {
        pass1_1010_1ea6(param_1.field_0x14, CONCAT22(in_stack_00000006, param_1));
    }
    SelectObject16(param_1.field_0x1c, param_1.field_0x18);
    SelectObject16(param_1.field_0x1e, param_1.field_0x18);
    DeleteObject16(param_1.field_0x1a);
    h_gdi_obj = SelectPalette16(0, param_1.field_0x20, param_1.field_0x18);
    DeleteObject16(h_gdi_obj);
    DeleteDC16(param_1.field_0x18);
    *_param_1 = ctx.s_0_020_1050_3ab0;
    param_1.ptr_a_hi = &ctx.PTR_LOOP_1050_1008;
    *_param_1 = ctx.s_1_1050_389a;
    param_1.ptr_a_hi = &ctx.PTR_LOOP_1050_1008;
    return;
}

pub fn invalidate_rect_1020_3080(param_1: u32, param_2: i32) {
    if (param_2 == 1) {
        (param_1 + 0x14) = 0;
        return;
    }
    if ((param_2 != 4) && (param_2 != 6)) {
        return;
    }
    InvalidateRect16(0, 0x0, 0);
    return;
}

pub fn draw_1020_30be(in_struct_1: *mut Struct653) {
    let paVar1: *mut Struct510;
    let is_iconic_result: bool;
    let local_struct_1: *mut Struct653;
    let local_struct_1_hi: *mut Struct653;
    let unaff_ss: HWND16;
    let local_struct_2_16: *mut Struct510;
    let mut local_3a: u16;
    let mut local_38: u16;
    let mut local_36: u16;
    let mut local_34: u16;
    let mut local_32: u16;
    let mut local_30: u16;
    let mut local_2e: u16;
    let mut local_2c: u16;
    let mut local_2a: u16;
    let mut local_28: u16;
    let mut local_26: u16;
    let local_struct_2_16_2: *mut Struct510;
    let local_paint_struct_1: PAINTSTRUCT16;
    let fn_ptr_1: fn();

    local_struct_1_hi = (in_struct_1 >> 0x10);
    local_struct_1 = in_struct_1;
    local_struct_2_16_2 = BeginPaint16(
        CONCAT22(unaff_ss, &local_paint_struct_1),
        local_struct_1.win_handle_0x4,
    );
    local_struct_2_16 = local_struct_1.win_handle_0x4;
    is_iconic_result = IsIconic16(local_struct_2_16);
    if (is_iconic_result == 0) {
        paVar1 = local_struct_1.struct_ptr_0x14;
        local_struct_2_16 = paVar1;
        local_3a = (paVar1 >> 0x10);
        pass1_1018_0a50(paVar1);
        local_struct_2_16 = &local_struct_2_16_2;
        fn_ptr_1 = (CONCAT22(unaff_ss, local_struct_2_16) + 8);
        local_3a = unaff_ss;
        (**fn_ptr_1)(0x1018, local_struct_2_16, unaff_ss);
        paVar1 = local_struct_1.struct_ptr_0x14;
        if ((paVar1 + 0x84) == 1) {
            draw_1020_320e(in_struct_1, local_struct_2_16_2);
        }
        fn_ptr_1 = (CONCAT22(local_3a, local_struct_2_16) + 4);
        (**fn_ptr_1)(0x1018, local_struct_2_16, local_3a, 0, 0, 0xdc);
        paVar1 = local_struct_1.struct_ptr_0x14;
        if ((paVar1 + 0x84) != 1) {
            draw_1020_320e(in_struct_1, local_struct_2_16_2);
        }
        draw_rect_1020_3488(in_struct_1, local_struct_2_16_2);
        fn_ptr_1 = (CONCAT22(local_3a, local_struct_2_16) + 0xc);
        (**fn_ptr_1)(0x1018, local_struct_2_16, local_3a, &local_struct_2_16_2);
    } else {
        if (ctx.PTR_LOOP_1050_0010 == 0x0) {
            paVar1 = local_struct_1.struct_ptr_0x14;
            local_struct_2_16 = paVar1;
            local_3a = (paVar1 >> 0x10);
            fn_ptr_1 = (local_struct_1.struct_ptr_0x14 + 0x2c);
            local_26 = (**fn_ptr_1)(offset);
            if (local_26 != 0) {
                local_3a = 4;
                local_struct_2_16 = offset;
                local_28 = GetStockObject16(4);
                local_38 = local_struct_1.win_handle_0x4;
                local_struct_2_16 = &local_30;
                GetClientRect16(CONCAT22(unaff_ss, local_struct_2_16), local_38);
                local_32 = (local_2c - local_30) - 1;
                local_34 = (local_2a - local_2e) - 1;
                local_36 = local_struct_2_16_2;
                local_3a = &local_struct_2_16;
                local_struct_2_16 = local_28;
                FillRect16(local_28, local_3a, unaff_ss);
                local_36 = local_struct_2_16_2;
                local_3a = 2;
                local_38 = 2;
                local_struct_2_16 = local_26;
                DrawIcon16(
                    local_26,
                    CONCAT214(
                        local_2c,
                        CONCAT212(
                            local_2e,
                            CONCAT210(
                                local_30,
                                CONCAT28(
                                    local_32,
                                    CONCAT26(local_34, CONCAT24(local_struct_2_16_2, 0x20002)),
                                ),
                            ),
                        ),
                    ),
                    CONCAT88(
                        local_paint_struct_1._0_8_,
                        CONCAT26(
                            local_struct_2_16_2,
                            CONCAT24(local_26, CONCAT22(local_28, local_2a)),
                        ),
                    ),
                    local_paint_struct_1.rc_print.right,
                );
            }
        }
    }
    local_3a = local_struct_1.win_handle_0x4;
    EndPaint(&local_paint_struct_1, unaff_ss);
    return;
}

pub fn draw_1020_320e(param_1: *mut Struct653, param_2: u16) {
    let pu_var1: *mut u32;
    let ppc_var2: fn();
    let mut u_var3: u32;
    let h_gdi_obj: HPALETTE16;
    let mut i_var4: i32;
    let mut i_var5: i32;
    let mut u_var6: u16;
    let mut u_var7: u16;
    let mut unaff_ss: u16;
    let mut u_var8: u32;
    let u_var9: u8;
    let u_var10: u8;
    let mut u_var11: u16;
    let mut u_var12: u16;
    let mut u_var13: u16;
    let mut u_var14: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u32;
    let mut local_c: u16;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    local_4 = param_2;
    u_var6 = (param_1 >> 0x10);
    i_var4 = param_1;
    u_var3 = (i_var4 + 0x14);
    if ((u_var3 + 0x84) == 1) {
        u_var3 = (i_var4 + 0x14);
        u_var7 = (u_var3 >> 0x10);
        i_var5 = u_var3;
        pu_var1 = (i_var5 + 0x24);
        u_var14 = SUB42(s_dib_1050_42c6, 0);
        u_var12 = 0;
        u_var13 = 0;
        u_var9 = 0;
        u_var10 = 0;
        u_var11 = 0;
        u_var8 = process_struct_1008_4772((pu_var1 & 0xffff | *(i_var5 + 0x26) << 0x10));
        local_4 = CreateDC16(
            u_var8,
            CONCAT13(u_var10, CONCAT12(u_var9, (u_var8 >> 0x10))),
            CONCAT22(u_var12, u_var11),
            CONCAT22(u_var14, u_var13),
        );
        local_6 = &local_4;
        unsafe {
            ppc_var2 = (*pu_var1 + 8);
        }
        ppc_var2(offset, pu_var1, (pu_var1 >> 0x10), local_6);
    }
    pass1_1018_0d9a(
        (i_var4 + 0x14),
        CONCAT22(unaff_ss, &local_c),
        CONCAT22(unaff_ss, &local_a),
    );
    u_var3 = (i_var4 + 0x14);
    u_var3 = (u_var3 + 0x6c);
    draw_1020_33c0(
        param_1,
        u_var3,
        (u_var3 >> 0x10),
        local_c,
        local_a,
        (local_a >> 0x10),
        1,
        local_4,
    );
    pass1_1018_1054(
        (i_var4 + 0x14),
        CONCAT22(unaff_ss, &local_c),
        CONCAT22(unaff_ss, &local_a),
    );
    u_var3 = (i_var4 + 0x14);
    u_var3 = (u_var3 + 0x74);
    draw_1020_33c0(
        param_1,
        u_var3,
        (u_var3 >> 0x10),
        local_c,
        local_a,
        (local_a >> 0x10),
        2,
        local_4,
    );
    pass1_1018_1320(
        (i_var4 + 0x14),
        CONCAT22(unaff_ss, &local_c),
        CONCAT22(unaff_ss, &local_a),
    );
    u_var3 = (i_var4 + 0x14);
    u_var3 = (u_var3 + 0x68);
    draw_1020_33c0(
        param_1,
        u_var3,
        (u_var3 >> 0x10),
        local_c,
        local_a,
        (local_a >> 0x10),
        1,
        local_4,
    );
    pass1_1018_15f6(
        (i_var4 + 0x14),
        CONCAT22(unaff_ss, &local_c),
        CONCAT22(unaff_ss, &local_a),
    );
    if (local_c != 0) {
        u_var3 = (i_var4 + 0x14);
        u_var3 = (u_var3 + 0x70);
        draw_1020_33c0(
            param_1,
            u_var3,
            (u_var3 >> 0x10),
            local_c,
            local_a,
            (local_a >> 0x10),
            1,
            local_4,
        );
    }
    pass1_1018_108c(
        (i_var4 + 0x14),
        CONCAT22(unaff_ss, &local_c),
        CONCAT22(unaff_ss, &local_a),
    );
    if (local_c != 0) {
        u_var3 = (i_var4 + 0x14);
        u_var3 = (u_var3 + 0x78);
        draw_1020_33c0(
            param_1,
            u_var3,
            (u_var3 >> 0x10),
            local_c,
            local_a,
            (local_a >> 0x10),
            0,
            local_4,
        );
    }
    u_var3 = (i_var4 + 0x14);
    if ((u_var3 + 0x84) == 1) {
        h_gdi_obj = SelectPalette16(0, local_6, local_4);
        DeleteObject16(h_gdi_obj);
        DeleteDC16(local_4);
    }
    return;
}

pub fn draw_1020_33c0(
    param_1: u32,
    color_ref: COLORREF,
    param_3: i32,
    param_4: u32,
    param_5: i32,
    in_dc_handle: HDC16,
) {
    let mut pen: u16;
    let mut pen_obj_handle: u16;
    let mut solid_brush: u16;
    let mut solid_brush_obj_handle: u16;
    let mut i_var1: i32;
    let mut u_var2: u16;
    let h_gdi_obj: HGDIOBJ16;

    let mut u_var3: u16;
    let h_var4: HDC16;
    let mut local_14: u16;
    let mut local_12: u32;
    let mut local_e: u32;
    let mut local_a: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    if (param_3 != 0) {
        h_var4 = in_dc_handle;
        pen = CreatePen16(color_ref, 1, 0);
        pen_obj_handle = SelectObject16(pen, h_var4);
        h_var4 = in_dc_handle;
        solid_brush = CreateSolidBrush16(color_ref);
        solid_brush_obj_handle = SelectObject16(solid_brush, h_var4);
        local_e = param_4;
        local_14 = 0;
        while (local_14 < param_3) {
            u_var3 = (param_1 >> 0x10);
            i_var1 = param_3;
            h_var4 = in_dc_handle;
            pass1_1020_3540(param_1, u_var3, param_5, local_e);
            if (param_5 < 1) {
                u_var2 = 3;
            } else {
                u_var2 = 4;
            }
            polygon_1020_3602(param_1, u_var3, u_var2, i_var1, ctx.dx_reg, h_var4);
            error_check_1000_17ce(CONCAT22(ctx.dx_reg, i_var1));
            local_14 = local_14 + 1;
            local_e = local_e & 0xffff0000 | (local_e + 6);
        }
        h_gdi_obj = SelectObject16(solid_brush_obj_handle, in_dc_handle);
        DeleteObject16(h_gdi_obj);
        SelectObject16(pen_obj_handle, in_dc_handle);
        DeleteObject16(pen);
    }
    return;
}

pub fn draw_rect_1020_3488(param_1: *mut Struct653, in_h_dc: u16) {
    let mut u_var1: i32;
    let mut u_var2: u32;
    let mut u_var3: u32;
    let paVar4: *mut Struct199;
    let mut pen: u16;
    let mut pen_obj_handle: u16;
    let mut stock_obj_5: u16;
    let obj_handle: HGDIOBJ16;
    let mut u_var5: u16;
    let mut unaff_ss: u16;
    let mut h_dc: u16;
    let mut left: i32;
    let mut h_dc_2: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u32;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;

    u_var5 = (param_1 >> 0x10);
    u_var2 = (param_1 + 0x14);
    paVar4 = (u_var2 + 0x36);
    local_6 = u_var2 & 0xffff0000 | ZEXT24(paVar4);
    pass1_1008_3e94(
        paVar4,
        CONCAT22(unaff_ss, &local_a),
        CONCAT22(unaff_ss, &local_8),
    );
    u_var2 = (local_8 - 3) << 0x10;
    if ((local_8 - 3) < 0) {
        u_var2 = 0;
    }
    u_var1 = _local_a - 3;
    _local_a = u_var1;
    if (u_var1 < 0) {
        _local_a = 0;
    }
    _local_a = u_var2 | _local_a;
    u_var3 = (param_1 + 0x14);
    h_dc = in_h_dc;
    pen = CreatePen16((u_var3 + 100), 1, 0);
    pen_obj_handle = SelectObject16(pen, h_dc);
    stock_obj_5 = GetStockObject16(5);
    obj_handle = SelectObject16(stock_obj_5, h_dc_2);
    left = (_local_a >> 0x10);
    Rectangle16(_local_a + 6, left + 6, _local_a, left, in_h_dc);
    SelectObject16(pen_obj_handle, in_h_dc);
    SelectObject16(obj_handle, in_h_dc);
    DeleteObject16(pen);
    return;
}

pub fn palette_func_1020_2992(param_1: u32, param_2: i32) {
    let pp_var1: fn();
    let obj: HGDIOBJ16;
    let hdc: HDC16;
    let mut u_var2: u16;
    let pu_var3: *mut u32;
    let mut local_6: u16;
    let mut local_4: u16;

    if (param_2 != 0) {
        u_var2 = (param_1 >> 0x10);
        pu_var3 = pass1_1018_0a50((param_1 + 0xf2));
        unsafe {
            pp_var1 = (*pu_var3 + 0x18);
        }
        obj = (**pp_var1)(0x1018);
        UnrealizeObject16(obj);
        hdc = GetDC16((param_1 + 8));
        RealizePalette16(hdc);
    }
    return;
}

pub fn send_win_msg_1020_29d8(param_1: i32, param_2: u16, param_3: u16, param_4: u32) {
    let mut u_var1: u32;
    let ppVar2: *mut pass1_struct_1;
    let mut i_var3: i32;
    let mut in_stack_0000fff6: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    i_var3 = (param_3 >> 0x10);
    u_var1 = post_win_msg_1020_79fc(param_1, param_2, param_3_00, param_3, i_var3);
    ppVar2 = process_struct_1010_20ba(
        ctx._g_Struct372_1050_0ed0,
        CONCAT22(in_stack_0000fff6, 0x29),
    );
    if (i_var3 == 0) {
        pass1_1018_270e(ppVar2, 1, (param_1 + 0xfc));
    } else {
        pass1_1018_270e(ppVar2, 0, (param_1 + 0xfc));
        SendMessage16(0, 0x69, 0x111, ctx.g_h_window);
    }
    return u_var1;
}

pub fn win_gui_fn_1020_2a46(in_struct_1: *mut Struct628, param_2: u16, param_3: u16) {
    pass1_1018_0ae8(in_struct_1.field_0xf2, 1);
    ui1::show_window_1008_68c6(in_struct_1, param_2, param_3);
    return;
}

pub fn cleanup_fn_1020_2a6a(in_struct_1: *mut Struct652) {
    let local_struct_1: *mut Struct652;
    let local_struct_1_hi: *mut Struct652;

    get_sys_metrics_1020_7a50(in_struct_1);
    pass1_1018_0ae8((in_struct_1 + 0xf2), 0);
    destroy_icon_1020_2c88(in_struct_1);
    return;
}

pub fn pass1_1020_289a(param_1: *mut win_struct_42, param_2: u16, param_3: u32) {
    let mut i_var1: i32;
    let local_struct_1_hi: *mut win_struct_42;

    call_load_cursor_1020_790e(param_1, s_SCPOPMENU_1050_427c, param_2, param_3);
    local_struct_1_hi = (param_1 >> 0x10);
    i_var1 = param_1;
    (i_var1 + 0xf2) = 0;
    (i_var1 + 0xf6) = 0;
    (i_var1 + 0xfa) = 0;
    (i_var1 + 0xfc) = 0;
    param_1.u16_x0 = (s__P__P__P__P__P__P__P__P__P__P__P_1050_2e35 + 0x15);
    (i_var1 + 2) = 0x1020;
    copy_string_1000_3d3e((param_1 & 0xffff0000 | (i_var1 + 0x5b)), s_VrMode_1050_4286);
    (i_var1 + 0xac) = 0x44c00000;
    return;
}

pub fn win_gui_fn_1020_2642(in_struct_1: *mut win_struct_42) {
    let paVar1: *mut Struct650;
    let struct_a: *mut Struct199;
    let local_struct_1: *mut win_struct_42;
    let local_struct_1_hi: *mut win_struct_42;
    let paVar2: *mut Struct199;
    let mut u_var3: u32;
    let mut local_4: u16;

    paVar2 = create_win_1008_9760(in_struct_1);
    struct_a = (paVar2 >> 0x10);
    local_struct_1_hi = (in_struct_1 >> 0x10);
    local_struct_1 = in_struct_1;
    paVar1 = get_gui_dc_1018_4db0(*&local_struct_1.u32_xf2, local_struct_1.win_handle_0x8);
    process_struct_1000_179c(0x18, struct_a);
    if ((struct_a | paVar1) != 0) {
        u_var3 = call_draw_fn_1020_27b0(paVar1, CONCAT22(local_struct_1.win_handle_0x8, struct_a));
        local_struct_1.char_ptr_16_0xee = u_var3;
        local_struct_1.field_0xf0 = (u_var3 >> 0x10);
        return;
    }
    &local_struct_1.char_ptr_16_0xee = 0;
    return;
}

pub fn call_fn_ptr_1020_26a6(in_struct_1: *mut Struct594) {
    let pu_var1: *mut u32;
    let mut u_var2: i32;
    let mut u_var3: u16;
    let mut in_stack_0000fff6: u16;
    let fn_ptr_1: fn();

    u_var3 = (in_struct_1 >> 0x10);
    pu_var1 = (in_struct_1 + 0xee);
    u_var2 = (in_struct_1 + 0xf0);
    if ((u_var2 | pu_var1) != 0) {
        unsafe {
            fn_ptr_1 = *pu_var1;
        }
        (**fn_ptr_1)();
    }
    ui1::destroy_win_1008_628e(in_struct_1, in_stack_0000fff6);
    return;
}

pub fn call_load_cursor_1020_2524(in_struct_1: *mut Struct65, param_2: u16, param_3: u32) {
    let mut local_AX_24: u16;
    let mut local_DX_71: u16;
    let mut i_var1: i32;
    let mut local_BP__1: u16;
    let mut local_es_21: u16;
    let ppVar2: *mut pass1_struct_1;

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

pub fn win_gui_fn_1020_3644(in_struct_1: *mut win_struct_42, param_2: u16, param_3: u32) {
    let local_struct_1: *mut win_struct_42;
    let local_struct_1_hi: *mut win_struct_42;

    call_load_cursor_1020_790e(in_struct_1, 0x0, param_2, param_3);
    local_struct_1_hi = (in_struct_1 >> 0x10);
    local_struct_1 = in_struct_1;
    local_struct_1.u32_xf2 = ctx.s_1_1050_389a;
    local_struct_1.field_0xf4 = &ctx.PTR_LOOP_1050_1008;
    local_struct_1.u32_xf2 = (ctx.s_18_2_1050_3aa5 + 3);
    local_struct_1.field_0xf4 = &ctx.PTR_LOOP_1050_1008;
    local_struct_1.field_0x100 = 0;
    local_struct_1.field_0x10a = 0;
    local_struct_1.field_0x10e = 0;
    in_struct_1.u16_x0 = 0x3d08;
    local_struct_1.u16_x2 = 0x1020;
    local_struct_1.u32_xf2 = 0x3d9c;
    local_struct_1.field_0xf4 = 0x1020;
    load_string_1010_84e0(
        ctx._g_struct_73_1050_14cc,
        (ctx._g_struct_73_1050_14cc >> 0x10),
        0x3ff,
        (in_struct_1 & 0xffff0000 | ZEXT24(&local_struct_1.u32_0xa)),
        0x5e9,
    );
    copy_string_1000_3d3e(
        (in_struct_1 & 0xffff0000 | ZEXT24(&local_struct_1.class_name_0x5b)),
        s_VrMode_1050_42ca,
    );
    local_struct_1.style_0xac = 0x44c00000;
    set_window_pos_1020_38aa(local_struct_1, local_struct_1_hi);
    return;
}

pub fn win_fn_1020_36f6(ctx: &mut AppContext, param_1: u32, param_2: i32) {
    let mut i_var1: i32;
    let ppc_var2: fn();
    let mut u_var3: u32;
    let pu_var4: Vec<u8>;
    let mut HVar5: HWND16;
    let mut h_var6: HWND16;
    let mut u_var7: u32;
    let mut in_dx: i32;

    let mut i_var8: i32;
    let mut u_var9: u16;
    let mut unaff_ss: u16;
    let mut u_var10: u16;
    let u_var11: u8;
    let u_var12: u8;
    let mut local_40e: u16;
    let mut local_40c: u16;
    let mut local_40a: u16;
    let mut local_408: u16;
    let mut local_406: [u8; 1024];
    let mut local_6: u16;
    let mut local_4: u16;

    i_var8 = param_1;
    u_var9 = (param_1 >> 0x10);
    if (param_2 == 1) {
        (i_var8 + 8) = 0;
        return;
    }
    if (param_2 != 0xd) {
        return;
    }
    u_var7 = pass1_1018_1e78((i_var8 + 8), 0xffff);
    _local_6 = (u_var7 & 0xffff | in_dx << 0x10);
    u_var3 = (i_var8 + 0x18);
    GetWindowText16(0x3ff, CONCAT22(unaff_ss, local_406), (u_var3 + 6));
    pu_var4 = local_406;
    process_string_1000_472c(CONCAT22(unaff_ss, pu_var4), ':');
    _local_40a = CONCAT22(in_dx, pu_var4 + 2);
    *_local_40a = 0;
    load_string_1010_84e0(
        ctx._g_struct_73_1050_14cc,
        (ctx._g_struct_73_1050_14cc >> 0x10),
        0x3ff,
        CONCAT22(unaff_ss, local_406),
        *_local_6,
    );
    u_var3 = (i_var8 + 0x18);
    ppc_var2 = ((i_var8 + 0x18) + 0x18);
    ppc_var2(0x1010, u_var3, (u_var3 >> 0x10), local_406);
    u_var3 = (i_var8 + 8);
    i_var1 = (u_var3 + 0x4a);
    u_var3 = (i_var8 + 0x18);
    h_var6 = (u_var3 + 6);
    HVar5 = h_var6;
    SetDlgItemText16((_local_6 + 2), 0x10f, h_var6);
    SetDlgItemText16((_local_6 + 10), 0x110, h_var6);
    SetDlgItemText16((_local_6 + 0x12), 0x112, h_var6);
    SetDlgItemText16((_local_6 + 0xe), 0x113, h_var6);
    if (i_var1 != 0) {
        HVar5 = pass1_1018_1f1a((i_var8 + 8), (_local_6 + 0x1a));
        if (HVar5 != 0) {
            u_var11 = 0x11;
            u_var12 = 1;
            u_var3 = (_local_6 + 0x16);
            HVar5 = u_var3;
            u_var10 = (u_var3 >> 0x10);
            // goto LAB_1020_3846;
        }
    }
    u_var11 = 0x11;
    u_var12 = 1;
    load_string_1010_847e(
        ctx._g_struct_73_1050_14cc,
        (ctx._g_struct_73_1050_14cc >> 0x10),
        0x5d9,
    );
    u_var10 = ctx.dx_reg;
    // LAB_1020_3846:
    SetDlgItemText16(CONCAT22(u_var10, HVar5), CONCAT11(u_var12, u_var11), h_var6);
    if ((i_var8 + 0x1c) != 0) {
        u_var3 = (i_var8 + 0x1c);
        h_var6 = GetDlgItem16((_local_6 + 0x1a), (u_var3 + 6));
        SetFocus16(h_var6);
        return;
    }
    InvalidateRect16(0, 0x0, 0);
    return;
}
