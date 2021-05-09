use crate::{
    defines::Struct307,
    other_funcs::big_fn_1010_b038,
    pass5_funcs::pass1_1030_835a,
    pass6_funcs::pass1_1030_e2be,
    pass7_funcs::pass1_1040_a626,
    pass_funcs::{pass1_1000_4906, pass1_1008_5784, pass1_1008_5b12},
    util::CONCAT22,
};

pub unsafe fn pass1_1008_e05e(param_1: u32, param_2: u16, param_3: u32, param_4: u32) {
    let mut unaff_ss: u16;
    let mut u_var1: u32;
    let lVar2: u32;
    let mut local_122: u16;
    let mut local_120: u16;
    let mut local_10: u16;
    let mut local_e: [u8; 8];
    let local_6: *mut Struct307;
    let mut local_4: u16;

    local_6 = pass1_1008_e8cc(param_1, param_3, param_4);
    if (local_6 != 0) {
        u_var1 = pass1_1030_8326();
        local_6.field_0xe = u_var1;
        local_6.field_0x10 = (u_var1 >> 0x10);
        local_6.field_0xc = param_2;
    }
    pass1_1008_5784(CONCAT22(unaff_ss, local_e), (param_1 + 10));
    local_10 = 0;
    lVar2 = local_6;
    while {
        local_6 = lVar2;
        lVar2 = pass1_1008_5b12(CONCAT22(unaff_ss, local_e));
        if (lVar2 == 0) {}
        // goto LAB_1008_e0d3;
        (lVar2 + 0xc) != 1
    } {}
    local_10 = 1;
    // LAB_1008_e0d3:
    if (local_10 == 0) {
        local_6 = lVar2;
        pass1_1030_e2be(CONCAT22(unaff_ss, &local_122), 0, 0, 0);
        pass1_1030_835a(ctx._g_bool_1050_5748, CONCAT22(unaff_ss, &local_122));
    }
    return;
}

pub unsafe fn pass1_1008_e10c(param_1: u32, param_2: u32, param_3: u32) -> i32 {
    let mut iVar1: i32;
    let mut i_var2: i32;
    let mut u_var3: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    u_var3 = pass1_1008_e8cc(param_1, param_2, param_3);
    if (u_var3 == 0) {
        return 1;
    }
    iVar1 = (u_var3 + 0xc);
    if (-1 < iVar1) {
        if (iVar1 < 2) {
            return 1;
        }
        if ((0 < iVar1 + -1) && (i_var2 = iVar1 + -3, i_var2 == 0 || iVar1 + -2 < 1)) {
            pass1_1008_e9a4(param_1, (param_1 >> 0x10), u_var3);
            return i_var2;
        }
    }
    return 0;
}

// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn pass1_1008_e164(param_1: *mut Struct308) {
    let mut iVar1: i32;
    let ppc_var2: fn();
    let mut i_var3: i32;
    let mut i_var4: i32;
    let mut u_var5: u16;
    let local_bx_4: *mut Struct308;
    let in_struct_a_2: *mut Struct298;
    let mut unaff_ss: u16;
    let paVar6: *mut Struct301;
    let in_struct_1: *mut Struct301;
    let lVar7: u32;
    let mut u_var8: u32;
    let mut local_128: u16;
    let mut local_126: u16;
    let mut local_124: u16;
    let mut local_122: u16;
    let mut local_120: u16;
    let mut local_11e: u16;
    let mut local_11c: u16;
    let mut local_11a: u16;
    let mut local_118: u16;
    let mut local_116: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    in_struct_a_2 = (param_1 >> 0x10);
    local_bx_4 = param_1;
    local_6 = pass1_1008_e8cc(param_1, local_bx_4.field_0x1a, local_bx_4.field_0x16);
    u_var5 = (local_6 >> 0x10);
    i_var3 = local_6;
    if (local_6 == 0) {
        _local_120 = pass1_1008_e852(local_bx_4, in_struct_a_2, local_bx_4.field_0x16);
        paVar6 = pass1_1008_e852(local_bx_4, in_struct_a_2, local_bx_4.field_0x1a);
        in_struct_1 = paVar6;
        process_struct_1000_179c(0x14, (paVar6 >> 0x10));
        if (in_struct_1 == 0x0) {
            lVar7 = 0;
        } else {
            lVar7 = pass1_1008_dc90(in_struct_1, paVar6, _local_120);
        }
        (lVar7 + 0xc) = 1;
        local_6 = lVar7;
        u_var8 = pass1_1030_8326();
        (local_6 + 0xe) = u_var8;
        (local_6 + 0x10) = (u_var8 >> 0x10);
        ppc_var2 = (local_bx_4.field_0xa + 4);
        ppc_var2(0x1030, local_bx_4.field_0xa, local_6);
    } else {
        iVar1 = (i_var3 + 0xc);
        i_var4 = iVar1 + -1;
        if (i_var4 == 0) {
            return;
        }
        if (((0 < i_var4) && (!SBORROW2(i_var4, 1))) && (iVar1 + -2 < 2)) {
            (i_var3 + 0x12) = 1;
        }
        (i_var3 + 0xc) = 1;
    }
    local_4 = (local_6 >> 0x10);
    pass1_1030_e2be(
        CONCAT13((unaff_ss >> 8), CONCAT12(unaff_ss, &local_118)),
        1,
        (local_6 + 8),
        (local_6 + 4),
    );
    u_var8 = pass1_1030_8326();
    pass1_1030_8372(
        ctx._g_bool_1050_5748,
        (ctx._g_bool_1050_5748 >> 0x10),
        u_var8 + 1,
        &local_118,
        unaff_ss,
    );
    return;
}

pub unsafe fn pass1_1008_e320(param_1: *mut Struct309, param_2: u32, param_3: u32) {
    let mut u_var1: i32;
    let u_var2: u8;
    let mut u_var3: i32;
    let extraout_var: u32;
    
    
    
    let local_bx_4: *mut Struct309;
    let mut u_var4: u16;
    let mut u_var5: u32;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;

    u_var4 = (param_1 >> 0x10);
    local_bx_4 = param_1;
    u_var2 = error_check_1000_17ce(&local_bx_4.field_0x1e);
    u_var3 = CONCAT31(extraout_var, u_var2);
    &local_bx_4.field_0x1e = 0;
    u_var5 = param_2;
    load_string_1010_847e(
        ctx._g_struct_73_1050_14cc,
        (ctx._g_struct_73_1050_14cc >> 0x10),
        0x531,
    );
    pass1_1000_3d7a(u_var3, ctx.dx_reg, u_var5);
    if ((u_var3 == 0) || (pass1_1000_3d7a(param_3, param_2), u_var3 == 0)) {
        local_c = 0x443;
    } else {
        pass1_1008_e8cc(param_1, param_2, param_3);
        if ((ctx.dx_reg | u_var3) == 0) {
            local_c = 0x444;
        } else {
            local_c = 0x443;
            u_var1 = (u_var3 + 0xc);
            u_var3 = u_var1;
            if (u_var1 != 0) {
                u_var3 = u_var1 - 1;
                if (u_var3 == 0) {
                    local_c = 0x445;
                    // goto LAB_1008_e378;
                }
                u_var3 = u_var1 - 2;
                if (u_var3 != 0) {
                    u_var3 = u_var1 - 3;
                    if (u_var3 == 0) {
                        local_c = 0x446;
                    }
                    // goto LAB_1008_e378;
                }
            }
            local_c = 0x444;
        }
    }
    // LAB_1008_e378:
    load_str_1010_84ac(
        ctx._g_struct_73_1050_14cc,
        (ctx._g_struct_73_1050_14cc >> 0x10),
        local_c,
    );
    local_bx_4.field_0x1e = u_var3;
    local_bx_4.field_0x20 = ctx.dx_reg;
    return;
}

pub unsafe fn pass1_1008_e852(
    in_struct_a_1: *mut Struct298,
    in_struct_a_2: *mut Struct298,
    param_1_00: u32,
) -> i32 {
    let pu_var1: *mut u16;
    let mut i_var2: i32;
    
    let mut unaff_ss: u16;
    let mut u_var3: u32;
    let mut local_1c: u32;
    let mut local_18: u32;
    let mut local_14: u16;
    let mut local_12: u16;

    pass1_1028_dc52(
        CONCAT22(unaff_ss, &local_14),
        (&ctx.PTR_LOOP_1050_0000 + 1),
        0,
        0x400,
    );
    while {
        pu_var1 = &local_14;
        pass1_1028_e4ec(CONCAT22(unaff_ss, pu_var1));
        if ((ctx.dx_reg | pu_var1) == 0) {
            return;
        }
        u_var3 = pass1_1038_4d28(CONCAT22(ctx.dx_reg, pu_var1));
        i_var2 = u_var3;
        pass1_1000_3d7a(param_1_00);
        i_var2 != 0
    } {}
    return;
}

// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn pass1_1008_e8cc(param_1: u32, param_2: u32, param_3: u32) -> libc::c_long {
    let mut u_var1: u32;
    let mut u_var2: i32;
    let paVar3: *mut Struct493;
    let mut i_var4: i32;
    let mut u_var5: i32;
    let mut u_var6: i32;
    let mut unaff_ss: u16;
    let lVar7: u32;
    let mut u_var8: u32;
    let mut u_var9: u32;
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
    let mut local_a: [u8; 8];

    pass1_1008_5784(CONCAT22(unaff_ss, local_a), (param_1 + 10));
    loop {
        lVar7 = pass1_1008_5b12(CONCAT22(unaff_ss, local_a));
        u_var5 = (lVar7 >> 0x10);
        u_var2 = lVar7;
        u_var6 = u_var5 | u_var2;
        if (lVar7 == 0) {
            return 0;
        }
        u_var1 = (u_var2 + 4);
        paVar3 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, u_var1, (u_var1 >> 0x10));
        _local_12 = CONCAT22(u_var6, paVar3);
        u_var1 = (u_var2 + 8);
        paVar3 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, u_var1, (u_var1 >> 0x10));
        _local_16 = CONCAT22(u_var6, paVar3);
        u_var8 = pass1_1038_4d28(_local_12);
        u_var9 = pass1_1038_4d28(_local_16);
        i_var4 = pass1_1000_3d7a(param_3, u_var8);
        if ((i_var4 == 0) && (i_var4 = pass1_1000_3d7a(param_2, u_var9), i_var4 == 0)) {
            break;
        }
        i_var4 = pass1_1000_3d7a(param_2);
        if ((i_var4 == 0) && (i_var4 = pass1_1000_3d7a(param_3), i_var4 == 0)) {
            return lVar7;
        }
    }
    return lVar7;
}

// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn pass1_1008_e9a4(param_1: u16, param_2: u16, param_1_00: u32) {
    let pu_var1: *mut u32;
    let mut i_var2: i32;
    let mut u_var3: i32;
    let local_bx_14: *mut Struct311;
    let mut u_var4: u16;
    let mut u_var5: u32;
    let mut in_stack_0000ffe8: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u32;
    let mut local_c: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    u_var5 = pass1_1030_8326();
    u_var4 = (param_1_00 >> 0x10);
    local_bx_14 = param_1_00;
    pu_var1 = &local_bx_14.field_0xe;
    unsafe {
        local_6 = CONCAT22(
            ((u_var5 >> 0x10) - local_bx_14.field_0x10) - (u_var5 < *pu_var1),
            u_var5 - *pu_var1,
        );
    }
    process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(in_stack_0000ffe8, 2));
    local_10 = 0;
    if ((u16_1050_13ae < 1) || (SBORROW2(u16_1050_13ae, 1))) {}
    // goto LAB_1008_ea2b;
    if (u16_1050_13ae == 2 || (u16_1050_13ae - 1) < 1) {
        if (local_bx_14.field_0x12 == 0) {
            // LAB_1008_ea20:
            u_var3 = 0x1e;
        } else {
            u_var3 = 10;
        }
    } else {
        if (u16_1050_13ae == 3) {
            if (local_bx_14.field_0x12 == 0) {
                u_var3 = 0x28;
            } else {
                u_var3 = 0x14;
            }
        } else {
            if (u16_1050_13ae != 4) {}
            // goto LAB_1008_ea2b;
            if (local_bx_14.field_0x12 != 0) {}
            // goto LAB_1008_ea20;
            u_var3 = 0x32;
        }
    }
    local_10 = u_var3;
    // LAB_1008_ea2b:
    if (local_10 < local_6) {
        pass1_fn_1008_612e(1, 100);
        local_14 = 0;
        i_var2 = local_bx_14.field_0xc;
        if (i_var2 == 2) {
            local_14 = 0x32;
        } else {
            if (i_var2 == 3) {
                local_14 = 0x19;
            }
        }
        if (local_6 < local_14) {
            return;
        }
    }
    return;
}

pub unsafe fn pass1_1008_eabc(param_1: i32, param_2: u16, param_3: u16) {
    process_struct_1010_1d48(CONCAT22(param_2, param_1), param_3);
    (param_1 + 10) = 0;
    zero_list_1008_3e38(CONCAT22(param_2, param_1 + 0xc));
    CONCAT22(param_2, param_1) = 0xeb1a;
    (param_1 + 2) = &ctx.PTR_LOOP_1050_1008;
    return CONCAT22(param_2, param_1);
}

pub unsafe fn pass1_1008_eaf4(param_1: u32, param_2: u8) {
    pass1_1010_1d80(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1008_eb2a(in_struct_1: *mut Struct312, param_2: u16, param_3: u16) {
    process_struct_1010_1d48(CONCAT22(param_2, in_struct_1), param_3);
    in_struct_1.field_0xa = 0;
    in_struct_1.field_0xc = 0;
    CONCAT22(param_2, in_struct_1) = 0xec00;
    in_struct_1.field_0x2 = &ctx.PTR_LOOP_1050_1008;
    return;
}

pub unsafe fn pass1_1008_eb5c(param_1: u16, param_2: u16, param_1_00: i32) {
    return CONCAT22(0x1050, param_1_00 * 0x10 + 0xd0e);
}

pub unsafe fn ret_5_1008_eb6e() -> i32 {
    return 5;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn pass1_1008_eb74(param_1: u32, param_2: u16) {
    let mut unaff_si: u16;
    let mut local_a: u16;
    let mut local_8: u16;

    (param_1 + 10) = param_2;
    if (param_2 != 0) {
        process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(unaff_si, 3));
        pass1_1010_c312();
    }
    return;
}

pub unsafe fn pass1_1008_ebda(param_1: u32, param_2: u8) {
    pass1_1010_1d80(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1008_ec10(param_1: i32, param_2: u16, param_3: u16) {
    process_struct_1010_1d48(CONCAT22(param_2, param_1), param_3);
    (param_1 + 10) = 0;
    CONCAT22(param_2, param_1) = 0xec62;
    (param_1 + 2) = &ctx.PTR_LOOP_1050_1008;
    return CONCAT22(param_2, param_1);
}

pub unsafe fn pass1_1008_ec3c(param_1: u32, param_2: u8) {
    pass1_1010_1d80(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1008_ec72(param_1: *mut u16) {
    pass1_1010_383a(param_1);
    unsafe {
        *param_1 = 0xefc4;
    }
    (param_1 + 2) = &ctx.PTR_LOOP_1050_1008;
    return param_1;
}

pub unsafe fn pass1_1008_ec94(param_1: *mut Struct313) {
    let mut in_stack_00000006: u16;

    _param_1.a = 0xefc4;
    param_1.field_0x2 = &ctx.PTR_LOOP_1050_1008;
    pass1_1010_3880(_param_1);
    return;
}

pub unsafe fn pass1_1008_ecb2(param_1: *mut Struct314, param_2: u16) {
    let u_var1: u8;
    let extraout_AH: u8;
    let struct_a: *mut Struct199;
    let in_stack_00000008: *mut u8;
    let mut u_var2: i32;

    u_var1 = process_struct_1010_2cd2(param_1, param_2, in_stack_00000008);
    u_var2 = CONCAT11(extraout_AH, u_var1);
    CONCAT22(param_2, param_1) = 0xef9c;
    param_1.field_0x2 = &ctx.PTR_LOOP_1050_1008;
    process_struct_1000_179c(0x20c, struct_a);
    param_1.field_0x5c = u_var2;
    param_1.field_0x5e = struct_a;
    pass1_1000_4906(CONCAT22(struct_a, param_1.field_0x5c), 0, 0x20c);
    return CONCAT22(param_2, param_1);
}

pub unsafe fn pass1_1008_ed00(param_1: *mut u16) {
    unsafe {
        *param_1 = 0xef9c;
    }
    (param_1 + 2) = &ctx.PTR_LOOP_1050_1008;
    pass1_1010_2db2(param_1);
    return;
}

pub unsafe fn pass1_1008_ed62(param_1: u32, param_2: u16) {
    let mut iVar1: i32;
    let mut u_var2: u16;

    u_var2 = (param_1 >> 0x10);
    iVar1 = param_1;
    (iVar1 + 0x16) = param_2 * 8 + 0xd5e;
    (iVar1 + 0x18) = &ctx.g_alloc_addr_1050_1050;
    (iVar1 + 0x12) = (param_2 * 8 + 0xd64);
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn pass1_1008_ed8a(
    param_1: *mut u32,
    param_2: i32,
    param_3: i32,
    param_4: i32,
    param_5: i32,
) {
    let pp_var1: fn();
    let mut u_var2: u16;
    let mut cVar3: u8;
    let mut u_var4: i32;
    let mut unaff_si: u16;
    let mut u_var5: i32;
    let mut bVar6: bool;
    let mut u_var7: u32;
    let mut local_6: u16;

    ppRam10500df6 = (ZEXT24(PTR_LOOP_1050_0df8) << 0x10);
    if (0 < param_4) {
        unsafe {
            pp_var1 = (*param_1 + 0x18);
        }
        u_var2 = (**pp_var1)();
        ppRam10500df6 =
            process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(unaff_si, u_var2));
        // WARNING: Read-only address (ram,0x10500df6) is written
        u_var7 = (param_1 + 0xc);
        u_var7 = pass1_1010_2e02(0, (u_var7 + 0x12));
        u_var4 = param_2 + 1;
        u_var5 = param_3 + (0xfffe < param_2);
        cVar3 = (param_4 + -1) * 0x4;
        while (cVar3 != '\0') {
            bVar6 = CARRY2(u_var4, u_var4);
            u_var4 = u_var4 * 2;
            u_var5 = u_var5 * 2 + bVar6;
            cVar3 = cVar3 + -1;
        }
        pass1_1010_2e30(
            0,
            u_var4 | u_var7,
            u_var5 | (u_var7 >> 0x10),
            (param_5 * 8 + 0xd64),
        );
    }
    PTR_LOOP_1050_0df8 = (ppRam10500df6 >> 0x10);
    // WARNING: Read-only address (ram,0x10500df6) is written
    return;
}

pub unsafe fn pass1_1008_ee14(param_1: u32) {
    let mut iVar1: i32;
    let mut u_var2: u16;
    let mut unaff_ss: u16;
    let mut u_var3: u32;
    let local_1c: [Struct313; 6];

    u_var2 = (param_1 >> 0x10);
    iVar1 = param_1;
    if ((iVar1 + 0x56) == 0) {
        pass1_1008_ec72(CONCAT22(unaff_ss, local_1c));
        u_var3 = pass1_1010_398e(CONCAT22(unaff_ss, local_1c), 0, 0, 0);
        (iVar1 + 0x56) = u_var3;
        (iVar1 + 0x58) = (u_var3 >> 0x10);
        pass1_1008_ec94(local_1c);
    }
    return;
}

pub unsafe fn pass1_1008_ee72(param_1: u16, param_2: u16) {
    let pp_var1: fn();
    let mut u_var2: u32;
    let mut in_stack_00000008: u16;

    if ((param_1 + 0x56) == 0) {
        pp_var1 = (CONCAT22(param_2, param_1) + 0x10);
        (**pp_var1)();
    }
    u_var2 = pass1_1010_2e02(CONCAT22(param_2, param_1), in_stack_00000008);
    pass1_1010_2e5c(param_1, param_2, u_var2);
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn pass1_1008_eeac(param_1: u16, param_2: u16, param_1_00: u32) {
    let mut u_var1: u16;
    let mut c_var2: u8;
    let mut i_var3: i32;
    let mut u_var4: u16;
    let ppVar5: *mut pass1_struct_1;
    let mut u_var6: i32;
    let mut in_stack_0000ffee: u32;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    u_var6 = (param_1_00 + 0x12);
    ppVar5 = process_struct_1010_20ba(
        ctx._g_Struct372_1050_0ed0,
        CONCAT22((in_stack_0000ffee >> 0x10), 3),
    );
    u_var4 = (ppVar5 >> 0x10);
    u_var1 = ppVar5;
    if (u_var6 == 0x7d) {
        i_var3 = pass1_1010_a5ca(u_var1, u_var4, 0x7c);
        if (i_var3 != 0) {
            return 0;
        }
        i_var3 = pass1_1010_a5ca(u_var1, u_var4, 0x7d);
        if (i_var3 != 0) {
            return 0;
        }
        u_var6 = 0x78;
    } else {
        if (u_var6 < 0x7e) {
            c_var2 = u_var6;
            if (c_var2 == 's') {
                u_var6 = 9;
            } else {
                if (c_var2 == 'w') {
                    u_var6 = 0x2e;
                } else {
                    if (c_var2 == 'y') {
                        u_var6 = 0x5b;
                    }
                }
            }
        }
    }
    i_var3 = pass1_1010_a5ca(u_var1, u_var4, u_var6);
    local_4 = (i_var3 == 0);
    return local_4;
}

pub unsafe fn pass1_1008_ef38(param_1: u32) {
    let mut u_var1: u32;

    u_var1 = (param_1 + 0x16);
    return (u_var1 + 2);
}

pub unsafe fn pass1_1008_ef50(param_1: u32, param_2: u8) {
    pass1_1008_ec94(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1008_ef76(param_1: u32, param_2: u8) {
    pass1_1008_ed00(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn pass1_1010_0000(param_1: i32, param_2: u16) {
    let ppVar1: *mut pass1_struct_1;
    let mut in_stack_00000008: u16;
    let mut i_var2: i32;
    let mut u_var3: u16;
    let mut i_var4: i32;
    let mut u_var5: u16;

    // Segment:    3
    // Offset:     00015420
    // Length:     ee9f
    // Min Alloc:  ee9f
    // Flags:      0d50
    //     Code
    //     Moveable
    //     Preload
    //     Impure (Non-shareable)
    //
    process_struct_1010_1d48(CONCAT22(param_2, param_1), in_stack_00000008);
    (param_1 + 10) = 0;
    (param_1 + 0xc) = 0;
    CONCAT22(param_2, param_1) = 0x2c8;
    (param_1 + 2) = 0x1010;
    i_var4 = param_1 + 10;
    i_var2 = param_1 + 0xc;
    u_var3 = param_2;
    u_var5 = param_2;
    ppVar1 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(i_var2, 0x48));
    pass1_1008_3e94(
        (ppVar1 + 0xe),
        CONCAT22(u_var3, i_var2),
        CONCAT22(u_var5, i_var4),
    );
    return CONCAT22(param_2, param_1);
}

pub unsafe fn pass1_1010_0052(param_1: *mut u16) {
    unsafe {
        *param_1 = 0x2c8;
    }
    (param_1 + 2) = 0x1010;
    pass1_1010_1d80(param_1);
    return;
}

pub unsafe fn pass1_1010_01f8(param_1: u32, param_2: u32, param_3: *mut u16) {
    let mut iVar1: i32;
    let local_bx_20: *mut Struct315;
    let mut u_var2: u16;
    let mut u_var3: u16;
    let mut local_a: u16;

    local_bx_20 = ((param_3 * 4 + 0xe02) * 4);
    iVar1 = local_bx_20.field_0xdfc;
    u_var2 = (param_1 >> 0x10);
    u_var3 = (param_2 >> 0x10);
    (param_2 + 6) = (param_3 * 4 + 0xe04) * 0x28 + local_bx_20.field_0xdfa + (param_1 + 10);
    (param_2 + 8) = (param_1 + 0xc) + iVar1;
    return;
}

pub unsafe fn pass1_1010_02a2(param_1: *mut Struct44, param_2: u8) -> *mut Struct44 {
    pass1_1010_0052(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1010_0350(param_1: *mut Struct317) {
    let pu_var1: *mut u32;
    let mut u_var2: i32;
    let ppc_var3: fn();
    let local_bx_5: *mut Struct317;
    let mut u_var4: u16;

    u_var4 = (param_1 >> 0x10);
    local_bx_5 = param_1;
    param_1 = 0xe98;
    local_bx_5.field_0x2 = 0x1010;
    pu_var1 = local_bx_5.field_0xa;
    u_var2 = local_bx_5.field_0xc;
    if ((u_var2 | pu_var1) != 0) {
        unsafe {
            ppc_var3 = *pu_var1;
        }
        (**ppc_var3)();
    }
    pass1_1010_1d80(param_1);
    return;
}

pub unsafe fn pass1_1010_038e(param_1: *mut Struct318, param_2: bool) {
    let mut bVar1: bool;
    let local_bx_13: *mut Struct318;
    let mut u_var2: u16;
    let mut local_4: u16;

    bVar1 = false;
    local_bx_13 = param_1;
    u_var2 = (param_1 >> 0x10);
    if (param_2 != false) {
        if (local_bx_13.d == 0) {
            local_bx_13.a = u16_1050_0e28;
            local_bx_13.b = PTR_LOOP_1050_0e30;
            local_bx_13.c = PTR_LOOP_1050_0ea8;
            u16_1050_0e28 = 0;
            PTR_LOOP_1050_0e30 = 0x0;
            PTR_LOOP_1050_0ea8 = 0x0;
            local_bx_13.d = 1;
            bVar1 = true;
            // goto LAB_1010_0404;
        }
    }
    if (param_2 == false) {
        if (local_bx_13.d != 0) {
            u16_1050_0e28 = local_bx_13.a;
            PTR_LOOP_1050_0e30 = local_bx_13.b;
            PTR_LOOP_1050_0ea8 = local_bx_13.c;
            local_bx_13.d = 0;
            bVar1 = true;
        }
    }
    // LAB_1010_0404:
    if (bVar1) {
        pass1_1010_1f62(param_1, 3);
    }
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn pass1_1010_041a() -> bool {
    let mut u_var1: u32;

    u_var1 = pass1_1030_8326();
    if (((u_var1 >> 0x10) == 0) && (u_var1 < 100)) {
        return 0;
    }
    return 1;
}

// WARNING: Could not reconcile some variable overlaps

pub unsafe fn pass1_1010_043a(param_1: u32, param_2: u32, param_3: u16) {
    let local_AX_146: *mut Struct320;
    
    let mut u_var1: i32;
    let local_bx_8: *mut Struct319;
    let local_bx_213: *mut Struct321;
    let mut u_var2: u16;
    let mut u_var3: u16;
    let mut unaff_ss: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: [u8; 8];
    let fn_ptr_1: fn();

    local_bx_8 = param_1;
    u_var2 = (param_1 >> 0x10);
    if (param_3 == 0xc) {
        if (local_bx_8.field_0xe != 0) {
            return;
        }
        local_bx_8.field_0xe = 1;
    } else {
        if (param_3 == 0xd) {
            if (local_bx_8.field_0x10 != 0) {
                return;
            }
            local_bx_8.field_0x10 = 1;
        } else {
            if (param_3 == 0x12) {
                return;
            }
        }
    }
    pass1_1010_089e(param_1, 1, 6);
    pass1_1008_5784(CONCAT22(unaff_ss, local_a), local_bx_8.field_0xa);
    while {
        local_AX_146 = local_a;
        pass1_1008_5b12(CONCAT22(unaff_ss, local_AX_146));
        u_var1 = ctx.dx_reg | local_AX_146;
        if (u_var1 == 0) {
            process_struct_1000_179c(10, 0x0);
            _local_12 = CONCAT22(u_var1, local_AX_146);
            if ((u_var1 | local_AX_146) == 0) {
                _local_e = 0x0;
            } else {
                *_local_12 = ctx.s_1_1050_389a;
                local_AX_146.field_0x2 = &ctx.PTR_LOOP_1050_1008;
                *_local_12 = 0xea8;
                local_AX_146.field_0x2 = 0x1010;
                _local_e = _local_12;
            }
            u_var3 = (_local_e >> 0x10);
            local_bx_213 = _local_e;
            local_bx_213.field_0x4 = param_3;
            local_bx_213.field_0x6 = param_2;
            fn_ptr_1 = (local_bx_8.field_0xa + 8);
            (**fn_ptr_1)(0x1000, local_bx_8.field_0xa, local_bx_213, u_var3);
            return;
        }
        (local_AX_146.field_0x4 != param_3) || (local_AX_146.field_0x6 != param_2)
    } {}
    return;
}

// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn pass1_1010_0538(
    in_struct_1: *mut Struct318,
    in_str_list_1: *mut *mut Struct167,
    in_str_list_2: *mut *mut Struct167,
) -> i32 {
    let mut iVar1: i32;
    let mut u_var2: u32;
    let ppc_var3: fn();
    let mut u_var4: u32;
    let u_var5: u8;
    let local_AX_36: *mut Struct323;
    let mut u_var6: i32;
    let mut u_var7: i32;
    let mut u_var8: u32;
    let extraout_var_00: u32;
    let extraout_var_01: u32;
    let extraout_var_02: u32;
    
    
    let local_bx_21: *mut Struct322;
    let mut u_var9: u16;
    let mut u_var10: u16;
    let pa_var11: *mut Struct613;
    let u_var12: u8;
    let mut local_12: u16;
    let mut local_c: u32;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;
    let temp_5f37093992: *mut Struct324;
    let extraout_var: u32;

    local_AX_36 = 0x0;
    unsafe {
        *in_str_list_2 = 0x0;
    }
    unsafe {
        *in_str_list_1 = 0x0;
    }
    u_var9 = (in_struct_1 >> 0x10);
    local_bx_21 = in_struct_1;
    ppc_var3 = (local_bx_21.field_0xa + 0x10);
    (**ppc_var3)();
    local_6 = CONCAT22(ctx.dx_reg, local_AX_36);
    if ((ctx.dx_reg | local_AX_36) == 0) {
        return;
    }
    iVar1 = local_AX_36.field_0x4;
    u_var2 = local_AX_36.field_0x6;
    if ((ctx.dx_reg | local_AX_36) != 0) {
        ppc_var3 = *local_6;
        (**ppc_var3)();
    }
    u_var4 = local_bx_21.field_0xa;
    if ((u_var4 + 8) == 0) {
        pass1_1010_089e(in_struct_1, 0, 6);
        pass1_1010_1f62(in_struct_1, 3);
    }
    u_var6 = iVar1 + 0x757;
    u_var8 = u_var6;
    load_str_1010_84ac(
        ctx._g_struct_73_1050_14cc,
        (ctx._g_struct_73_1050_14cc >> 0x10),
        u_var6,
    );
    in_str_list_2 = u_var8;
    (in_str_list_2 + 2) = ctx.dx_reg;
    u_var6 = ctx.dx_reg;
    loop {
        u_var7 = u_var8;
        let in_str_list_2_val = unsafe { *in_str_list_2 };
        process_string_1000_472c(in_str_list_2_val, '%');
        u_var8 = (u_var6 | u_var7);
        if ((u_var6 | u_var7) == 0) {
            break;
        }
        pass1_1010_09b4(
            local_bx_21,
            u_var9,
            CONCAT22(u_var6, u_var7),
            in_str_list_2,
            u_var2,
        );
    }
    u_var7 = iVar1 - 1;
    u_var8 = u_var7;
    if (0x1e < u_var7) {}
    // goto LAB_1010_0850;
    u_var10 = (in_str_list_1 >> 0x10);
    match (u_var7) {
        0 => {
            u_var5 = -0x6a;
            u_var12 = 0x77;
        }
        // goto LAB_1010_0619;
        1 => {
            u_var5 = -0x69;
            u_var12 = 0x78;
        }
        // goto LAB_1010_0619;
        2 => {
            u_var12 = 0x79;
            u_var5 = -0x68
        }
        3 => {
            u_var5 = -0x66;
            u_var12 = 0x7a;
        }
        // goto LAB_1010_0619;
        4 => {
            u_var5 = -0x65;
            u_var12 = 0x7b;
        }
        // goto LAB_1010_0619;
        5 => {
            u_var12 = 0x7c;
            u_var5 = -100
        }
        6 => {
            u_var5 = -0x62;
            u_var12 = 0x7d;
        }
        // goto LAB_1010_0619;
        7 => {
            u_var5 = -0x61;
            u_var12 = 0x7e;
        }
        // goto LAB_1010_0619;
        8 => {
            u_var12 = 0x7f;
            u_var5 = -0x60
        }
        9 => {
            u_var5 = -0x5e;
            u_var12 = 0x80;
        }
        // goto LAB_1010_0619;
        10 => {
            u_var5 = -0x5d;
            u_var12 = 0x81;
        }
        // goto LAB_1010_0619;
        0xb => {
            u_var12 = 0x82;
            u_var5 = -0x5c
        }
        0xc => {
            u_var5 = -0x5a;
            u_var12 = 0x83;
        }
        // goto LAB_1010_0619;
        0xd => {
            u_var12 = 0x84;
            u_var5 = -0x59
        }
        0xe => {
            u_var5 = -0x57;
            u_var12 = 0x85;
        }
        // goto LAB_1010_0619;
        0xf => {
            u_var12 = 0x86;
            u_var5 = -0x56
        }
        0x10 => {
            u_var12 = 0x87;
            u_var5 = -0x54
        }
        0x11 => {
            u_var12 = 0x88;
            u_var5 = -0x52
        }
        0x12 => {
            u_var12 = 0x89;
            u_var5 = -0x50
        }
        0x13 => {
            u_var12 = 0x8a;
            u_var5 = -0x4e
        }
        0x14 => {
            u_var12 = 0x8b;
            u_var5 = -0x4c
        }
        0x15 => {
            u_var5 = -0x4a;
            u_var12 = 0x8c;
        }
        _ => {
            // LAB_1010_0619:
            u_var5 = string_fn_1008_5fd8(CONCAT11(7, u_var12), u_var5);
            u_var8 = CONCAT31(extraout_var, u_var5);
        }
        // goto LAB_1010_0621;
        0x16 => {
            u_var12 = 0x8d;
            u_var5 = -0x49
        }
        0x17 => {
            u_var12 = 0x8e;
            u_var5 = -0x47
        }
        0x18 => {
            u_var5 = -0x45;
            pa_var11 = 0x78f;
        }
        // goto LAB_1010_0785;
        0x19 => {
            u_var5 = -0x42;
            pa_var11 = 0x790;
        }
        // goto LAB_1010_0785;
        0x1a => {
            u_var5 = -0x3f;
            pa_var11 = 0x791;
        }
        // goto LAB_1010_0785;
        0x1b => {
            u_var12 = 0x92;
            u_var5 = -0x3c
        }
        0x1c => {
            u_var12 = 0x93;
            u_var5 = -0x3a
        }
        0x1d => {
            u_var5 = string_fn_1008_5fd8(0x794, -0x38);
            in_str_list_1 = CONCAT31(extraout_var_02, u_var5);
            (in_str_list_1 + 2) = u_var6;
        }
        0x1e => {
            u_var5 = -0x36;
            pa_var11 = 0x795;
            // LAB_1010_0785:
            u_var5 = string_fn_1008_5fd8(pa_var11, u_var5);
            u_var8 = CONCAT31(extraout_var_01, u_var5);
            // goto LAB_1010_0621;
        }
    }
    u_var5 = string_fn_1008_5fd8(CONCAT11(7, u_var12), u_var5);
    u_var8 = CONCAT31(extraout_var_00, u_var5);
    // LAB_1010_0621:
    in_str_list_1 = u_var8;
    (in_str_list_1 + 2) = u_var6;
    // LAB_1010_0850:
    loop {
        let in_str_list_1_val = unsafe { *in_str_list_1 };
        u_var7 = u_var8;
        process_string_1000_472c(in_str_list_1_val, '%');
        u_var8 = (u_var6 | u_var7);
        if ((u_var6 | u_var7) == 0) {
            break;
        }
        pass1_1010_09b4(
            local_bx_21,
            u_var9,
            CONCAT22(u_var6, u_var7),
            in_str_list_1,
            u_var2,
        );
    }
    return;
}

pub unsafe fn pass1_1010_089e(param_1: u32, param_2: u16, param_3: u16) {
    ((param_3 - 1) * 8 + 0xe28) = param_2;
    pass1_1010_1f62(param_1, 3);
    return;
}

pub unsafe fn pass1_1010_08c0(struct_a: *mut pass1_struct_1, param_2: u16, param_3: u16) {
    ((param_3 - 1) * 8 + 0xea8) = param_2;
    pass1_1010_1f62(struct_a, 3);
    return;
}

pub unsafe fn pass1_1010_08e2(param_1: u16, param_2: u16, param_1_00: i32) {
    let mut local_4: u16;

    if (PTR_LOOP_1050_4fe8 != 0x0) {
        u16_1050_0e28 = 0;
        PTR_LOOP_1050_0e30 = 0x0;
        PTR_LOOP_1050_0e38 = 0x0;
        PTR_LOOP_1050_0e40 = 0x0;
        PTR_LOOP_1050_0e48 = 0x0;
        u16_1050_0e58 = 0;
        u16_1050_0e60 = 0;
        PTR_LOOP_1050_0e70 = 0x0;
    }
    return CONCAT22(0x1050, (param_1_00 + -1) * 8 + 0xe22);
}

pub unsafe fn pass1_1010_091e(param_1: u16, param_2: u16, param_1_00: i32) {
    return CONCAT22(0x1050, (param_1_00 + -1) * 8 + 0xe72);
}

pub unsafe fn pass1_1010_0932(param_1: u16, param_2: u16, param_1_00: i32) {
    return CONCAT22(0x1050, (param_1_00 + -1) * 8 + 0xe8a);
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn pass1_1010_0946(param_1: u16, param_2: u16, param_1_00: i32) {
    let ppVar1: *mut pass1_struct_2;
    let mut u_var2: u16;
    let ppVar3: *mut pass1_struct_1;
    let mut u_var4: u32;
    let mut u_var5: u16;
    let mut u_var6: u32;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_4: u16;

    PTR_LOOP_1050_0ea8 = 0x0;
    u_var6 = 0x4000001;
    u_var2 = 2;
    u_var5 = 0x400;
    ppVar3 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, 0x2003b);
    u_var4 = pass1_1008_dfa6(ppVar3, CONCAT22(u_var5, u_var2), u_var6);
    u_var2 = (u_var4 >> 0x10);
    if (u_var4 != 0) {
        ppVar1 = pass1_1030_8344(
            ctx._g_bool_1050_5748,
            (ctx._g_bool_1050_5748 >> 0x10),
            0x4000002,
        );
        if (&ppVar1[1].field_0x9e == 0x8000002) {
            PTR_LOOP_1050_0ea8 = (&ctx.PTR_LOOP_1050_0000 + 1);
        }
    }
    return CONCAT22(0x1050, (param_1_00 + -1) * 8 + 0xea2);
}

// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn pass1_1010_09b4(
    param_1: u16,
    param_2: u16,
    param_1_00: *mut u8,
    param_2_00: *mut *mut Struct167,
    param_5: u32,
) -> i32 {
    let mut bVar1: u8;
    let mut b_var2: bool;
    let mut u8_var3: bool;
    let mut u_var4: u16;
    let ppVar5: *mut pass1_struct_2;
    let mut u_var6: i32;
    let mut u_var7: i32;
    let mut u_var8: u16;
    let in_dx: *mut Struct199;
    let pp_var9: *mut pass1_struct_1;
    let mut in_stack_0000ffde: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_c: u16;
    let mut local_a: u32;
    let mut local_6: u32;

    u8_var3 = false;
    b_var2 = false;
    bVar1 = *(param_1_00 + 1);
    if (bVar1 == 0x70) {
        // LAB_1010_0a06:
        u8_var3 = false;
        b_var2 = true;
    } else {
        if (bVar1 < 0x71) {
            if (bVar1 != 0x43) {
                if (bVar1 == 0x50) {}
                // goto LAB_1010_0a06;
                if (bVar1 != 99) {}
                // goto LAB_1010_09db;
            }
            u8_var3 = true;
            b_var2 = false;
        }
    }
    // LAB_1010_09db:
    local_a = 0;
    if (b_var2) {
        pp_var9 =
            process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(in_stack_0000ffde, 2));
        u_var4 = (pp_var9 >> 0x10);
        local_a = CONCAT22((pp_var9 + 0x6e), (pp_var9 + 0x6c));
    } else {
        if (!u8_var3) {}
        // goto LAB_1010_0a36;
        ppVar5 = pass1_1030_8344(
            ctx._g_bool_1050_5748,
            (ctx._g_bool_1050_5748 >> 0x10),
            param_5,
        );
        local_a = pass1_1038_4d28(CONCAT22(in_dx, ppVar5));
    }
    in_dx = (local_a >> 0x10);
    // LAB_1010_0a36:
    if ((local_a._2_2_ | local_a) != 0) {
        let param_2_00_val = unsafe { *param_2_00 };
        u_var6 = get_string_index_1000_3da4(param_2_00_val);
        u_var7 = get_string_index_1000_3da4(local_a);
        u_var8 = u_var7 + 10 + u_var6;
        process_struct_1000_179c(u_var8, in_dx);
        _local_12 = CONCAT22(in_dx, u_var8);
        let param_1_00_val = unsafe { *param_1_00 };
        unsafe {
            *param_1_00 = '\0';
        }
        copy_string_1000_3d3e(CONCAT22(in_dx, u_var8), param_2_00_val);
        process_string_1000_3cea(CONCAT22(in_dx, u_var8), local_a);
        process_string_1000_3cea(
            CONCAT22(in_dx, u_var8),
            (param_1_00 & 0xffff0000 | (param_1_00 + 2)),
        );
        unsafe {
            error_check_1000_17ce(*param_2_00);
            *param_2_00 = _local_12;
        }
    }
    return;
}

pub unsafe fn pass1_1010_0e46(param_1: *mut Struct44, param_2: u8) -> *mut Struct44 {
    pass1_1010_0350(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1010_0e6c(param_1: *mut u16, param_2: u8) {
    unsafe {
        *param_1 = ctx.s_1_1050_389a;
    }
    (param_1 + 2) = &ctx.PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1010_0eac(param_1: *mut u8, param_2: *mut u8, param_3: u16) {
    pass1_1018_4cda(param_1, CONCAT22(param_3, param_2));
    CONCAT22(param_2, param_1) = 0xf0c;
    (param_1 + 2) = 0x1010;
    PTR_LOOP_1050_4230 = param_1;
    PTR_LOOP_1050_4232 = param_2;
    pass1_1018_4dce(CONCAT22(param_2, param_1), 0xff);
    return CONCAT22(param_2, param_1);
}

pub unsafe fn pass1_1010_0f76(param_1: *mut u16) {
    unsafe {
        *param_1 = (s_648_bmp_1050_1919 + 1);
    }
    (param_1 + 2) = 0x1010;
    cleanup_1010_17c0(param_1);
    pass1_1010_2db2(param_1);
    return;
}

// WARNING: Could not reconcile some variable overlaps

pub unsafe fn pass1_1010_0f9c(param_1: *mut u32) {
    let pp_var1: fn();
    let mut u_var2: u32;
    let mut u_var3: u32;
    let u_var4: u8;
    
    let mut u_var5: u16;
    let puVar6: *mut u16;
    let pu_var7: *mut u16;
    let mut u_var8: u16;
    let extraout_var: u32;
    let mut u_var9: u32;
    
    
    let pa_var10: *mut Struct199;
    
    let mut u_var11: i32;
    
    
    let extraout_dx_04: *mut Struct199;
    let mut iVar12: i32;
    let mut iVar13: i32;
    let mut iVar14: i32;
    let mut uVar15: u16;
    let mut uVar16: u16;
    let mut unaff_ss: u16;
    let pu_var17: *mut u32;
    let uVar18: u8;
    let mut local_26: u16;
    let mut local_24: u16;
    let mut local_22: u16;
    let mut local_20: u16;
    let mut local_1e: u16;
    let mut local_1c: u16;
    let mut local_1a: u16;
    let mut local_18: u32;
    let mut local_14: u16;
    let mut local_12: u16;

    uVar15 = (param_1 >> 0x10);
    iVar12 = param_1;
    unsafe {
        pp_var1 = (*param_1 + 0x38);
    }
    (**pp_var1)();
    (iVar12 + 0x68) = in_ax;
    if (((iVar12 + 0x60) != 0) && ((iVar12 + 0x68) == 1)) {
        return;
    }
    if ((iVar12 + 0x68) == 0) {
        return;
    }
    pa_var10 = ctx.dx_reg;
    pass1_1028_dc52(
        CONCAT22(unaff_ss, &local_14),
        (&ctx.PTR_LOOP_1050_0000 + 1),
        0,
        0x700,
    );
    u_var5 = (iVar12 + 0x68) * 0x18;
    process_struct_1000_179c(u_var5, pa_var10);
    (iVar12 + 0x60) = u_var5;
    (iVar12 + 0x62) = pa_var10;
    _local_1c = CONCAT22(pa_var10, (iVar12 + 0x60));
    local_1e = (iVar12 + 0x68);
    while {
        while {
            puVar6 = &local_14;
            pass1_1028_e4ec(CONCAT22(unaff_ss, puVar6));
            local_18 = CONCAT22(ctx.dx_reg, puVar6);
            pa_var10 = (ctx.dx_reg | puVar6);
            if (pa_var10 == 0x0) {}
            // goto LAB_1010_10ca;
            unsafe {
                iVar13 = *param_1;
            }
            pp_var1 = (iVar13 + 0x40);
            pu_var7 = puVar6;
            (**pp_var1)(&PTR_LOOP_1050_1028, param_1);
            pu_var7 == 0x0
        } {}
        u_var11 = ctx.dx_reg;
        u_var4 = pass1_1028_b58e(local_18);
        u_var8 = CONCAT31(extraout_var, u_var4);
        u_var3 = CONCAT31(extraout_var, u_var4) & 0xffff | u_var11 << 0x10;
        pp_var1 = (iVar13 + 0x2c);
        (**pp_var1)(&PTR_LOOP_1050_1028, param_1);
        uVar16 = (_local_1c >> 0x10);
        iVar14 = _local_1c;
        *_local_1c = u_var8;
        (iVar14 + 2) = ctx.dx_reg;
        uVar18 = SUB21(puVar6, 0);
        pp_var1 = (iVar13 + 0x30);
        pu_var17 = param_1;
        u_var11 = ctx.dx_reg;
        (**pp_var1)();
        (iVar14 + 8) = u_var8;
        (iVar14 + 10) = ctx.dx_reg;
        (iVar14 + 0xc) = u_var3;
        pp_var1 = (iVar13 + 0x3c);
        u_var9 = u_var3;
        (**pp_var1)(
            &PTR_LOOP_1050_1028,
            param_1,
            local_18,
            pu_var17,
            uVar18,
            u_var11,
        );
        (iVar14 + 0x10) = u_var9;
        (iVar14 + 0x12) = extraout_dx_04;
        (iVar14 + 0x14) = u_var3;
        _local_1c = (_local_1c & 0xffff0000 | (iVar14 + 0x18));
        local_1e = local_1e - 1;
        pa_var10 = extraout_dx_04;
        local_1e != 0
    } {}
    // LAB_1010_10ca:
    u_var5 = (iVar12 + 0x68) << 2;
    process_struct_1000_179c(u_var5, pa_var10);
    (iVar12 + 100) = u_var5;
    (iVar12 + 0x66) = pa_var10;
    local_20 = 0;
    local_1e = 0;
    loop {
        if ((iVar12 + 0x68) * 3 <= local_1e) {
            break;
        }
        u_var8 = (iVar12 + 0x62);
        u_var2 = (iVar12 + 100);
        uVar16 = (u_var2 >> 0x10);
        iVar13 = u_var2;
        (iVar13 + local_20 * 4) = (iVar12 + 0x60) + local_1e * 8;
        (iVar13 + local_20 * 4 + 2) = u_var8;
        local_1e = local_1e + 3;
        local_20 = local_20 + 1;
    }
    return;
}

pub unsafe fn pass1_1010_1146(param_1: u32, param_2: u16) {
    let mut u_var1: u16;

    u16_1050_0ecc = param_2;
    u_var1 = (param_1 >> 0x10);
    pass1_1000_4aea(
        (param_1 + 100),
        (param_1 + 0x66),
        4,
        (s_dibtext_bmp_1050_1844 + 6),
        0x1010,
    );
    return;
}

pub unsafe fn pass1_1010_116c(param_1: *mut Struct364) {
    let mut iVar1: i32;
    let local_bx_4: *mut Struct364;
    let mut u_var2: u16;
    let mut u_var3: u32;
    let mut local_4: u16;
    let fn_ptr_1: fn();

    u_var2 = (param_1 >> 0x10);
    local_bx_4 = param_1;
    if (&local_bx_4.field_0x56 != 0) {
        fn_ptr_1 = (param_1 + 0x34);
        (**fn_ptr_1)();
    }
    fn_ptr_1 = (param_1 + 0x28);
    iVar1 = (**fn_ptr_1)();
    if (iVar1 != 0) {
        local_4 = u16_1050_0ecc;
        if (u16_1050_0ecc == 0xffff) {
            local_4 = 0;
        }
        pass1_1010_1146(param_1, local_4);
        u_var3 = loop_1010_11c6(param_1);
        local_bx_4.field_0x56 = u_var3;
        local_bx_4.field_0x58 = (u_var3 >> 0x10);
    }
    return;
}

pub unsafe fn pass1_1010_16ee(param_1: u16, param_2: u16, param_3: u16, param_4: u16) {
    let mut in_ax: i32;
    let in_dx: *mut Struct199;
    let mut local_4: u16;

    process_struct_1000_179c(0x4a, in_dx);
    if ((in_dx | in_ax) != 0) {
        pass1_1040_c54a(CONCAT22(in_dx, in_ax), 0, CONCAT22(param_2_00, param_1_00));
        return;
    }
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn pass1_1010_1722(param_1: u16, param_2: u16, param_1_00: u32) {
    let u_var1: u8;
    let mut u_var2: i32;
    let pu_var3: *mut u8;
    let extraout_var: u32;
    let mut in_dx: i32;
    
    let mut unaff_ss: u16;
    let mut u_var4: u32;
    let mut local_62: u16;
    let mut local_60: u16;
    let mut local_5e: u16;
    let mut local_5c: u16;
    let mut local_5a: u32;
    let mut local_56: u16;
    let mut local_54: u16;
    let mut local_52: [u8; 80];

    u_var1 = pass1_1028_b58e(param_1_00);
    u_var2 = CONCAT31(extraout_var, u_var1);
    if ((in_dx | u_var2) == 0) {
        load_string_1010_847e(
            ctx._g_struct_73_1050_14cc,
            (ctx._g_struct_73_1050_14cc >> 0x10),
            0x424,
        );
        copy_string_1000_3d3e(CONCAT22(unaff_ss, local_52), CONCAT22(ctx.dx_reg, u_var2));
        pu_var3 = local_52;
    } else {
        u_var4 = pass1_1038_4d28((u_var2 + 0x2e));
        pu_var3 = u_var4;
    }
    pass1_fn_1008_60e8(pu_var3);
    return;
}

pub unsafe fn pass1_1010_18f4(param_1: *mut Struct44, param_2: u8) -> *mut Struct44 {
    pass1_1010_0f76(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1010_19a4(param_1: *mut u32) {
    let pp_var1: fn();
    let p_uvar2: *mut u16;
    
    let mut unaff_ss: u16;
    let mut local_1c: u16;
    let mut local_1a: u16;
    let mut local_18: u32;
    let mut local_14: u16;
    let mut local_12: u16;

    pass1_1028_dc52(
        CONCAT22(unaff_ss, &local_14),
        (&ctx.PTR_LOOP_1050_0000 + 1),
        0,
        0x700,
    );
    loop {
        pu_var2 = &local_14;
        pass1_1028_e4ec(CONCAT22(unaff_ss, pu_var2));
        if ((ctx.dx_reg | pu_var2) == 0) {
            break;
        }
        unsafe {
            pp_var1 = (*param_1 + 0x40);
        }
        (**pp_var1)(&PTR_LOOP_1050_1028, param_1);
    }
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn pass1_1010_1a06(param_1: u32, param_2: u32) {
    let long_byte_ptr: *mut u328_t;
    let u_var1: u8;
    let mut u_var2: u16;
    let mut u_var3: u16;
    let mut u_var4: u16;
    let ppVar5: *mut pass1_struct_1;
    let mut u_var6: u32;
    let mut local_8: u16;

    u_var1 = pass1_1028_b58e(param_2);
    u_var4 = (param_1 >> 0x10);
    u_var6 = (param_1 + 0x6e);
    long_byte_ptr = big_fn_1010_b038(u_var6, (u_var6 >> 0x10), u_var1);
    u_var2 = pass1_fn_1000_3e2c(long_byte_ptr);
    ppVar5 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(param_2, 0x32));
    u_var3 = pass1_1010_7818(ppVar5, param_2);
    u_var6 = (param_1 + 0x6e);
    u_var6 = pass1_1010_ada6(u_var6, (u_var6 >> 0x10), u_var2, u_var3);
    pass1_fn_1008_60e8(u_var6);
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn pass1_1010_1a66(param_1: u32, param_2: u32) {
    let mut u_var1: u32;
    let mut b_var2: u8;
    let u_var3: u8;
    let extraout_AH: u8;
    let mut u_var4: u16;
    let b_var5: bool;
    let mut u_var6: i32;
    let mut u_var7: i32;
    let mut u_var8: u16;
    let mut u_var9: u32;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_4: u16;

    u_var7 = (param_2 >> 0x10);
    u_var6 = (param_2 + 0x1e) & 0xff;
    if (((param_2 + 0x1c) != 2) || (u_var6 != 0)) {
        u_var3 = pass1_1028_b58e((param_2 & 0xffff | u_var7 << 0x10));
        u_var8 = (param_1 >> 0x10);
        u_var1 = (param_1 + 0x6e);
        u_var9 = pass1_1010_c2d8(
            u_var1,
            (u_var1 >> 0x10),
            CONCAT22(u_var6, CONCAT11(extraout_AH, u_var3)),
        );
        if ((u_var9 != 2) || ((u_var9 & 0xff0000) != 0)) {
            u_var1 = (param_1 + 0x6e);
            u_var4 = pass1_1010_b028(u_var1, (u_var1 >> 0x10), param_2);
            b_var5 = pass1_1008_c6ae(ctx._PTR_LOOP_1050_06e0, u_var4, 5);
            if ((b_var5 == 0)
                && (
                    b_var5 = pass1_1008_c6ae(ctx._PTR_LOOP_1050_06e0, u_var4, 6),
                    b_var5 == 0,
                ))
            {
                b_var2 = 0;
            } else {
                b_var2 = 1;
            }
            local_4 = b_var2;
            return local_4;
        }
    }
    return 0;
}

pub unsafe fn pass1_1010_1b04(param_1: *mut Struct44, param_2: u8) -> *mut Struct44 {
    pass1_1010_0f76(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn pass1_1010_1b6e(param_1: *mut Struct314, param_2: *mut Struct314, param_3: *mut u8) {
    let mut unaff_bp: u16;
    let ppVar1: *mut pass1_struct_1;

    modify_struct_1010_0f24(param_1, param_2, param_3);
    (param_1 + 1) = 0;
    CONCAT22(param_2, param_1) = (s_330_flc_1050_1cfe + 6);
    param_1.field_0x2 = 0x1010;
    ppVar1 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(unaff_bp, 3));
    (param_1 + 1) = ppVar1;
    param_1[1].field_0x2 = (ppVar1 >> 0x10);
    return CONCAT22(param_2, param_1);
}

pub unsafe fn pass1_1010_1bb4(param_1: *mut u32) {
    let pu_var1: *mut u16;
    
    let mut unaff_ss: u16;
    let mut local_1c: u16;
    let mut local_1a: u16;
    let mut local_18: u32;
    let mut local_14: u16;
    let mut local_12: u16;
    let fn_ptr_1: fn();

    pass1_1028_dc52(
        CONCAT22(unaff_ss, &local_14),
        (&ctx.PTR_LOOP_1050_0000 + 1),
        0,
        0x700,
    );
    loop {
        pu_var1 = &local_14;
        pass1_1028_e4ec(CONCAT22(unaff_ss, pu_var1));
        if ((ctx.dx_reg | pu_var1) == 0) {
            break;
        }
        unsafe {
            fn_ptr_1 = (*param_1 + 0x40);
        }
        (**fn_ptr_1)(&PTR_LOOP_1050_1028, param_1);
    }
    return;
}

pub unsafe fn pass1_1010_1c40(param_1: u32, param_2: u32) {
    let mut u_var1: u32;
    let mut b_var2: u8;
    let u_var3: u8;
    let extraout_AH: u8;
    let mut u_var4: u16;
    let b_var5: bool;
    let mut u_var6: i32;
    let mut u_var7: i32;
    let mut u_var8: u16;
    let mut u_var9: u32;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_4: u16;

    u_var7 = (param_2 >> 0x10);
    u_var6 = (param_2 + 0x1e) & 0xff;
    if (((param_2 + 0x1c) != 2) || (u_var6 != 0)) {
        u_var3 = pass1_1028_b58e((param_2 & 0xffff | u_var7 << 0x10));
        u_var8 = (param_1 >> 0x10);
        u_var1 = (param_1 + 0x6e);
        u_var9 = pass1_1010_c2d8(
            u_var1,
            (u_var1 >> 0x10),
            CONCAT22(u_var6, CONCAT11(extraout_AH, u_var3)),
        );
        if ((u_var9 != 2) || ((u_var9 & 0xff0000) != 0)) {
            u_var1 = (param_1 + 0x6e);
            u_var4 = pass1_1010_b028(u_var1, (u_var1 >> 0x10), param_2);
            b_var5 = pass1_1008_c6ae(ctx._PTR_LOOP_1050_06e0, u_var4, 0x11);
            if ((b_var5 == 0)
                && (
                    b_var5 = pass1_1008_c6ae(ctx._PTR_LOOP_1050_06e0, u_var4, 0x12),
                    b_var5 == 0,
                ))
            {
                b_var2 = 0;
            } else {
                b_var2 = 1;
            }
            local_4 = b_var2;
            return local_4;
        }
    }
    return 0;
}

pub unsafe fn pass1_1010_1cde(param_1: *mut Struct44, param_2: u8) -> *mut Struct44 {
    pass1_1010_0f76(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1010_1d80(param_1: *mut Struct376) {
    let pu_var1: *mut u32;
    let mut u_var2: i32;
    let ppc_var3: fn();
    let local_bx_5: *mut Struct376;
    let mut u_var4: i32;

    u_var4 = (param_1 >> 0x10);
    local_bx_5 = param_1;
    param_1.ptr_a_lo = (s_573_bmp_1050_200f + 5);
    local_bx_5.ptr_a_hi = 0x1010;
    pass1_1010_1f62((param_1 & 0xffff | u_var4 << 0x10), 1);
    pu_var1 = local_bx_5.u16_x4;
    u_var2 = local_bx_5.u16_x6;
    if ((u_var2 | pu_var1) != 0) {
        unsafe {
            ppc_var3 = *pu_var1;
        }
        (**ppc_var3)();
    }
    param_1.ptr_a_lo = ctx.s_1_1050_389a;
    local_bx_5.ptr_a_hi = &ctx.PTR_LOOP_1050_1008;
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn pass1_1010_1dda(param_1: *mut u8) {
    pass1_1010_209e(ctx._g_Struct372_1050_0ed0, (param_1 + 8));
    return;
}

pub unsafe fn pass1_1010_1ea6(param_1: *mut u8, param_2: *mut Struct45) {
    let pu_var1: *mut u32;
    let mut u_var2: i32;
    let ppc_var3: fn();
    let mut u_var4: u32;
    let pu_var5: *mut u8;
    
    let mut i32_var6: i32;
    let mut u_var7: u16;
    let mut unaff_ss: u16;
    let mut local_1e: u16;
    let mut local_1c: u16;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: [u8; 4];
    let mut local_8: u32;
    let mut local_4: u16;

    u_var7 = (param_1 >> 0x10);
    i32_var6 = param_1;
    if ((i32_var6 + 4) == 0) {
        return '\0';
    }
    local_4 = 0;
    pass1_1008_5784(CONCAT22(unaff_ss, local_c), (i32_var6 + 4));
    loop {
        pu_var5 = local_c;
        pass1_1008_5b12(CONCAT22(unaff_ss, pu_var5));
        if ((ctx.dx_reg | pu_var5) == 0) {
            break;
        }
        if ((pu_var5 + 4) == param_2) {
            local_4 = 1;
            ppc_var3 = ((i32_var6 + 4) + 0xc);
            (**ppc_var3)(&ctx.PTR_LOOP_1050_1008);
            local_8 = 0;
        }
    }
    u_var4 = (i32_var6 + 4);
    if ((u_var4 + 8) == 0) {
        pu_var1 = (i32_var6 + 4);
        u_var2 = (i32_var6 + 6);
        if ((u_var2 | pu_var1) != 0) {
            unsafe {
                ppc_var3 = *pu_var1;
            }
            (**ppc_var3)(
                &ctx.PTR_LOOP_1050_1008,
                pu_var1,
                u_var2,
                1,
                pu_var1,
                u_var2,
                pu_var1,
                u_var2,
            );
        }
        (i32_var6 + 4) = 0;
    }
    return local_4;
}

pub unsafe fn pass1_1010_1f62(param_1: *mut Struct318, param_2: u16) {
    let mut u_var1: u32;
    let ppc_var2: fn();
    let mut i_var3: i32;
    let mut u_var4: u16;
    let mut unaff_ss: u16;
    let lVar5: u32;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: [u8; 8];

    pass1_1008_5784(CONCAT22(unaff_ss, local_a), (param_1 + 4));
    loop {
        lVar5 = pass1_1008_5b12(CONCAT22(unaff_ss, local_a));
        u_var4 = (lVar5 >> 0x10);
        i_var3 = lVar5;
        if (lVar5 == 0) {
            break;
        }
        if ((((i_var3 + 8) == 0) || (param_2 == 0)) || ((i_var3 + 8) == param_2)) {
            u_var1 = (i_var3 + 4);
            ppc_var2 = ((i_var3 + 4) + 4);
            ppc_var2(&ctx.PTR_LOOP_1050_1008, u_var1, (u_var1 >> 0x10), param_2);
        }
    }
    return '\0';
}

pub unsafe fn pass1_1010_1fbe(param_1: *mut u16, param_2: u8) {
    unsafe {
        *param_1 = ctx.s_1_1050_389a;
    }
    (param_1 + 2) = &ctx.PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1010_1fea(param_1: u32, param_2: u8) {
    pass1_1010_1d80(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1010_2024(param_1: u32) {
    PTR_LOOP_1050_0ed2 = (param_1 >> 0x10);
    g_Struct372_1050_0ed0 = param_1;
    g_Struct372_1050_0ed0.field_0x124 = 0;
    pass1_1000_4906(
        (param_1 & 0xffff | ZEXT24(PTR_LOOP_1050_0ed2) << 0x10),
        0,
        0x124,
    );
    return param_1;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn pass1_1010_2050(param_1: *mut Struct372) {
    let pu_var1: *mut u32;
    let mut u_var2: i32;
    let ppc_var3: fn();
    let mut u_var4: u16;
    let mut local_4: u16;

    pass1_1010_2816(param_1);
    local_4 = 0;
    while {
        u_var4 = (param_1 >> 0x10);
        pu_var1 = (local_4 * 4 + param_1);
        u_var2 = (local_4 * 4 + param_1 + 2);
        if ((u_var2 | pu_var1) != 0) {
            unsafe {
                ppc_var3 = *pu_var1;
            }
            (**ppc_var3)();
        }
        local_4 = local_4 + 1;
        local_4 < 0x49
    } {}
    ctx._g_Struct372_1050_0ed0 = 0;
    return;
}

pub unsafe fn pass1_1010_209e(param_1: *mut Struct372, param_2: u16) {
    pass1_1010_2816(param_1);
    (param_1 + 0x124) = param_2;
    return;
}

pub unsafe fn process_struct_1010_20ba(
    in_struct_372_ptr: *mut Struct372,
    in_string_1: *mut libc::c_char,
) -> *mut pass1_struct_1 {
    let local_Struct64_ptr: *mut Struct64;
    let pu_var1: *mut u16;
    let paVar2: *mut Struct384;
    let struct_a: *mut Struct475;
    
    
    let mut u_var3: i32;
    
    
    
    let mut extraout_dx_04: i32;
    let mut extraout_dx_05: i32;
    let mut extraout_dx_06: i32;
    let mut extraout_dx_07: i32;
    let mut extraout_dx_08: i32;
    let mut extraout_dx_09: i32;
    let mut extraout_dx_10: i32;
    let mut extraout_dx_11: i32;
    let mut extraout_dx_12: i32;
    let mut extraout_dx_13: i32;
    let mut extraout_dx_14: i32;
    let paVar4: *mut Struct475;
    let mut extraout_dx_15: i32;
    let mut extraout_dx_16: i32;
    let mut extraout_dx_17: i32;
    let mut extraout_dx_18: i32;
    let mut extraout_dx_19: i32;
    let mut extraout_dx_20: i32;
    let mut extraout_dx_21: i32;
    let mut extraout_dx_22: i32;
    let mut local_DX_1240: u16;
    let mut extraout_dx_23: u16;
    let mut u_var5: u16;
    let mut extraout_dx_24: i32;
    let mut extraout_dx_25: i32;
    let mut extraout_dx_26: i32;
    let mut extraout_dx_27: i32;
    let mut extraout_dx_28: i32;
    let struct_373_ptr: *mut Struct373;
    let mut local_es_20: u16;
    let mut local_CS_1470: u16;
    let paVar6: *mut Struct375;
    let mut u_var7: u32;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_8: u16;
    let mut local_6: u32;
    let fn_ptr_1: fn();

    pass1_1010_2816(in_struct_372_ptr);
    local_Struct64_ptr = (in_string_1 * 4);
    local_es_20 = (in_struct_372_ptr >> 0x10);
    struct_373_ptr = in_struct_372_ptr;
    local_6 = (struct_373_ptr + &(local_Struct64_ptr).field_0x0);
    if (local_6 != 0x0) {
        return local_6;
    }
    match (in_string_1) {
        1 => {
            paVar4 = struct_a;
            process_struct_1000_179c(0x18, struct_a);
            if ((paVar4 | local_Struct64_ptr) == 0) {
                // LAB_1010_2126:
                local_Struct64_ptr = 0x0;
                u_var3 = 0;
            } else {
                process_struct_1010_3b7a(local_Struct64_ptr, CONCAT22(in_string_1, paVar4));
                u_var3 = extraout_dx_14;
            }
        }
        2 => {
            paVar4 = struct_a;
            process_struct_1000_179c(0x84, struct_a);
            if ((paVar4 | local_Struct64_ptr) == 0) {}
            // goto LAB_1010_2126;
            get_private_profile_str_1010_5404(local_Struct64_ptr, CONCAT22(in_string_1, paVar4));
            u_var3 = extraout_dx_11
        }
        3 => {
            paVar4 = struct_a;
            process_struct_1000_179c(0x53c, struct_a);
            if ((paVar4 | local_Struct64_ptr) == 0) {}
            // goto LAB_1010_2126;
            process_struct_1010_a1d8(local_Struct64_ptr, paVar4, in_string_1);
            u_var3 = extraout_dx_13
        }
        4 => {
            paVar4 = struct_a;
            process_struct_1000_179c(0x18a, struct_a);
            if ((paVar4 | local_Struct64_ptr) == 0) {}
            // goto LAB_1010_2126;
            pass1_1018_2b10(CONCAT22(paVar4, local_Struct64_ptr), in_string_1);
            u_var3 = extraout_dx_12
        }
        5 => {
            paVar4 = struct_a;
            process_struct_1000_179c(0x14, struct_a);
            if ((paVar4 | local_Struct64_ptr) == 0) {}
            // goto LAB_1010_2126;
            pu_var1 = pass1_1008_eabc(local_Struct64_ptr, paVar4, in_string_1);
            u_var3 = (pu_var1 >> 0x10);
            local_Struct64_ptr = pu_var1
        }
        6 => {
            paVar4 = struct_a;
            process_struct_1000_179c(0x58, struct_a);
            if ((paVar4 | local_Struct64_ptr) == 0) {}
            // goto LAB_1010_2126;
            pass1_1018_18b8(CONCAT22(paVar4, local_Struct64_ptr), in_string_1);
            u_var3 = extraout_dx_08
        }
        7 => {
            paVar4 = struct_a;
            process_struct_1000_179c(0xe, struct_a);
            if ((paVar4 | local_Struct64_ptr) == 0) {}
            // goto LAB_1010_2126;
            pass1_1010_3d82(local_Struct64_ptr, CONCAT22(in_string_1, paVar4));
            u_var3 = extraout_dx_07
        }
        8 => {
            paVar4 = struct_a;
            process_struct_1000_179c(0x20, struct_a);
            if ((paVar4 | local_Struct64_ptr) == 0) {}
            // goto LAB_1010_2126;
            process_struct_1010_95aa(local_Struct64_ptr, CONCAT22(in_string_1, paVar4));
            u_var3 = extraout_dx_05
        }
        9 => {
            paVar4 = struct_a;
            process_struct_1000_179c(0x26, struct_a);
            if ((paVar4 | local_Struct64_ptr) == 0) {}
            // goto LAB_1010_2126;
            modify_struct_1010_6326(local_Struct64_ptr, CONCAT22(in_string_1, paVar4));
            u_var3 = extraout_dx_06
        }
        10 => {
            paVar4 = struct_a;
            process_struct_1000_179c(0x1c, struct_a);
            if ((paVar4 | local_Struct64_ptr) == 0) {}
            // goto LAB_1010_2126;
            u_var7 = pass1_1010_0eac(local_Struct64_ptr, paVar4, in_string_1);
            u_var3 = (u_var7 >> 0x10);
            local_Struct64_ptr = u_var7
        }
        0xb => {
            paVar4 = struct_a;
            process_struct_1000_179c(0x1c, struct_a);
            if ((paVar4 | local_Struct64_ptr) == 0) {}
            // goto LAB_1010_2126;
            u_var7 = pass1_1008_aefe(local_Struct64_ptr, paVar4, in_string_1);
            u_var3 = (u_var7 >> 0x10);
            local_Struct64_ptr = u_var7
        }
        0xc | 0xd | 0xe | 0xf | 0x10 | 0x11 | 0x12 | 0x13 | 0x14 | 0x15 | 0x16 | 0x17 | 0x18
        | 0x19 | 0x1a | 0x1b | 0x1c | 0x1d | 0x1e | 0x1f | 0x20 | 0x21 | 0x22 | 0x23 | 0x24 => {
            paVar4 = struct_a;
            process_struct_1000_179c(0xaa, struct_a);
            if ((paVar4 | local_Struct64_ptr) == 0) {}
            // goto LAB_1010_2126;
            process_struct_1018_0570(CONCAT22(paVar4, local_Struct64_ptr), in_string_1);
            u_var3 = extraout_dx_09
        }
        0x25 => {
            paVar4 = struct_a;
            process_struct_1000_179c(0x1c, struct_a);
            if ((paVar4 | local_Struct64_ptr) == 0) {}
            // goto LAB_1010_2126;
            pass1_1018_4aaa(local_Struct64_ptr, paVar4, in_string_1);
            u_var3 = extraout_dx_18
        }
        0x26 => {
            paVar4 = struct_a;
            process_struct_1000_179c(0x1c, struct_a);
            if ((paVar4 | local_Struct64_ptr) == 0) {}
            // goto LAB_1010_2126;
            pass1_1008_d99e(local_Struct64_ptr, paVar4, in_string_1);
            u_var3 = extraout_dx_19
        }
        0x27 => {
            paVar4 = struct_a;
            process_struct_1000_179c(0x58, struct_a);
            u_var3 = paVar4 | local_Struct64_ptr;
            if (u_var3 == 0) {}
            // goto LAB_1010_2126;
            process_struct_1008_9d36(local_Struct64_ptr, paVar4, in_string_1)
        }
        0x28 => {
            paVar4 = struct_a;
            process_struct_1000_179c(0x2c, struct_a);
            if ((paVar4 | local_Struct64_ptr) == 0) {}
            // goto LAB_1010_2126;
            pass1_1010_28e6(local_Struct64_ptr, CONCAT22(in_string_1, paVar4));
            u_var3 = extraout_dx_16
        }
        0x29 => {
            paVar4 = struct_a;
            process_struct_1000_179c(0x72, struct_a);
            if ((paVar4 | local_Struct64_ptr) == 0) {}
            // goto LAB_1010_2126;
            pass1_1018_229c(local_Struct64_ptr, CONCAT22(in_string_1, paVar4));
            u_var3 = extraout_dx_17
        }
        0x2a => {
            paVar4 = struct_a;
            process_struct_1000_179c(0x1c, struct_a);
            if ((paVar4 | local_Struct64_ptr) == 0) {}
            // goto LAB_1010_2126;
            pass1_1010_503e(local_Struct64_ptr, paVar4, in_string_1);
            u_var3 = extraout_dx_20
        }
        0x2b => {
            paVar4 = struct_a;
            process_struct_1000_179c(0x1a, struct_a);
            if ((paVar4 | local_Struct64_ptr) == 0) {}
            // goto LAB_1010_2126;
            process_struct_1010_02e0(local_Struct64_ptr, paVar4, in_string_1);
            u_var3 = extraout_dx_04
        }
        0x2c => {
            paVar4 = struct_a;
            process_struct_1000_179c(0x10, struct_a);
            if ((paVar4 | local_Struct64_ptr) == 0) {}
            // goto LAB_1010_2126;
            pass1_1008_eb2a(local_Struct64_ptr, paVar4, in_string_1);
            u_var3 = extraout_dx_28
        }
        0x2d => {
            paVar4 = struct_a;
            process_struct_1000_179c(0x80, struct_a);
            if ((paVar4 | local_Struct64_ptr) == 0) {}
            // goto LAB_1010_2126;
            process_struct_1010_3e3c(CONCAT22(paVar4, local_Struct64_ptr), in_string_1);
            u_var3 = extraout_dx_10
        }
        0x2e => {
            paVar4 = struct_a;
            process_struct_1000_179c(0x806, struct_a);
            u_var3 = paVar4 | local_Struct64_ptr;
            if (u_var3 == 0) {}
            // goto LAB_1010_2126;
            u_var7 = pass1_1018_1ff4(local_Struct64_ptr, CONCAT22(in_string_1, paVar4));
            local_Struct64_ptr = u_var7
        }
        0x2f => {
            paVar4 = struct_a;
            process_struct_1000_179c(0x58, struct_a);
            if ((paVar4 | local_Struct64_ptr) == 0) {}
            // goto LAB_1010_2126;
            pass1_1010_e9e4(local_Struct64_ptr, paVar4, in_string_1);
            u_var3 = extraout_dx_21
        }
        0x30 => {
            paVar4 = struct_a;
            process_struct_1000_179c(0xe, struct_a);
            if ((paVar4 | local_Struct64_ptr) == 0) {}
            // goto LAB_1010_2126;
            paVar6 = pass1_1010_3702(local_Struct64_ptr, paVar4, in_string_1);
            u_var3 = (paVar6 >> 0x10);
            local_Struct64_ptr = paVar6
        }
        0x31 => {
            local_CS_1470 = 0x1000;
            paVar4 = struct_a;
            process_struct_1000_179c(0x60, struct_a);
            if ((paVar4 | local_Struct64_ptr) == 0) {
                local_CS_1470 = 0x1000;
                local_Struct64_ptr = 0x0;
                u_var3 = 0;
            } else {
                u_var7 = process_struct_1010_9298(local_Struct64_ptr, paVar4, in_string_1);
                u_var3 = (u_var7 >> 0x10);
                local_Struct64_ptr = u_var7;
            }
        }
        // goto LAB_1010_2683;
        0x32 => {
            paVar4 = struct_a;
            process_struct_1000_179c(0x26, struct_a);
            if ((paVar4 | local_Struct64_ptr) == 0) {}
            // goto LAB_1010_2126;
            process_struct_1010_6abc(local_Struct64_ptr, CONCAT22(in_string_1, paVar4));
            u_var3 = extraout_dx_22
        }
        0x33 => {
            paVar4 = struct_a;
            process_struct_1000_179c(0x72, struct_a);
            if ((paVar4 | local_Struct64_ptr) == 0) {
                // LAB_1010_25b8:
                local_Struct64_ptr = 0x0;
                u_var5 = 0;
            } else {
                modify_struct_1010_195e(local_Struct64_ptr, paVar4, in_string_1);
                u_var5 = local_DX_1240;
            }
        }
        // goto LAB_1010_25bb;
        0x34 => {
            paVar4 = struct_a;
            process_struct_1000_179c(0x72, struct_a);
            if ((paVar4 | local_Struct64_ptr) == 0) {}
            // goto LAB_1010_25b8;
            pass1_1010_1b6e(local_Struct64_ptr, paVar4, in_string_1);
            u_var5 = extraout_dx_23;
            // LAB_1010_25bb:
            local_6 = CONCAT22(u_var5, local_Struct64_ptr);
            pass1_1010_1146(CONCAT22(u_var5, local_Struct64_ptr), 0);
        }
        // goto switchD_1010_2765_caseD_38;
        0x35 => {
            paVar4 = struct_a;
            process_struct_1000_179c(0x14a, struct_a);
            u_var3 = paVar4 | local_Struct64_ptr;
            if (u_var3 == 0) {}
            // goto LAB_1010_2126;
            u_var7 = modify_struct_1010_6700(local_Struct64_ptr, CONCAT22(in_string_1, paVar4));
            local_Struct64_ptr = u_var7
        }
        0x36 => {
            paVar4 = struct_a;
            process_struct_1000_179c(0x10, struct_a);
            if ((paVar4 | local_Struct64_ptr) == 0) {}
            // goto LAB_1010_2126;
            pass1_1008_d790(local_Struct64_ptr, CONCAT22(in_string_1, paVar4));
            u_var3 = extraout_dx_24
        }
        0x37 => {
            paVar4 = struct_a;
            process_struct_1000_179c(0x420, struct_a);
            if ((paVar4 | local_Struct64_ptr) == 0) {}
            // goto LAB_1010_2126;
            process_struct_1008_9fd2(local_Struct64_ptr, CONCAT22(in_string_1, paVar4));
            u_var3 = extraout_dx_25
        }
        // default:
        // goto switchD_1010_2765_caseD_38;
        0x39 => {
            paVar4 = struct_a;
            process_struct_1000_179c(0x36, struct_a);
            if ((paVar4 | local_Struct64_ptr) == 0) {}
            // goto LAB_1010_2126;
            process_struct_1010_4a8a(local_Struct64_ptr, paVar4, in_string_1);
            u_var3 = extraout_dx_26
        }
        0x3a => {
            paVar4 = struct_a;
            process_struct_1000_179c(0xc, struct_a);
            if ((paVar4 | local_Struct64_ptr) == 0) {}
            // goto LAB_1010_2126;
            pu_var1 = pass1_1008_d72e(local_Struct64_ptr, paVar4, in_string_1);
            u_var3 = (pu_var1 >> 0x10);
            local_Struct64_ptr = pu_var1
        }
        0x3b => {
            paVar4 = struct_a;
            process_struct_1000_179c(0x22, struct_a);
            if ((paVar4 | local_Struct64_ptr) == 0) {}
            // goto LAB_1010_2126;
            process_struct_1008_dd4e(local_Struct64_ptr, CONCAT22(in_string_1, paVar4));
            u_var3 = ctx.dx_reg
        }
        0x3c => {
            paVar4 = struct_a;
            process_struct_1000_179c(0x146, struct_a);
            if ((paVar4 | local_Struct64_ptr) == 0) {}
            // goto LAB_1010_2126;
            pass1_1018_331c(local_Struct64_ptr, CONCAT22(in_string_1, paVar4));
            u_var3 = ctx.dx_reg
        }
        0x3d => {
            paVar4 = struct_a;
            process_struct_1000_179c(0xe, struct_a);
            u_var3 = paVar4 | local_Struct64_ptr;
            if (u_var3 == 0) {}
            // goto LAB_1010_2126;
            u_var7 = pass1_1010_8c32(local_Struct64_ptr, CONCAT22(in_string_1, paVar4));
            local_Struct64_ptr = u_var7
        }
        0x3e => {
            paVar4 = struct_a;
            process_struct_1000_179c(0x18, struct_a);
            if ((paVar4 | local_Struct64_ptr) == 0) {}
            // goto LAB_1010_2126;
            pass1_1018_5070(local_Struct64_ptr, CONCAT22(in_string_1, paVar4));
            u_var3 = ctx.dx_reg
        }
        0x3f => {
            paVar4 = struct_a;
            process_struct_1000_179c(0x12, struct_a);
            if ((paVar4 | local_Struct64_ptr) == 0) {}
            // goto LAB_1010_2126;
            pass1_1008_c72a(local_Struct64_ptr, paVar4, in_string_1);
            u_var3 = ctx.dx_reg
        }
        0x40 => {
            paVar4 = struct_a;
            process_struct_1000_179c(0x24, struct_a);
            if ((paVar4 | local_Struct64_ptr) == 0) {}
            // goto LAB_1010_2126;
            pass1_1008_af94(local_Struct64_ptr, CONCAT22(in_string_1, paVar4));
            u_var3 = ctx.dx_reg
        }
        0x41 => {
            paVar4 = struct_a;
            process_struct_1000_179c(0x60, struct_a);
            if ((paVar4 | local_Struct64_ptr) == 0) {}
            // goto LAB_1010_2680;
            local_CS_1470 = &ctx.PTR_LOOP_1050_1008;
            u_var7 = pass1_1008_ecb2(local_Struct64_ptr, paVar4);
            u_var3 = (u_var7 >> 0x10);
            local_Struct64_ptr = u_var7;
            // LAB_1010_2683:
            (struct_373_ptr + in_string_1 * 4) = local_Struct64_ptr;
            (struct_373_ptr + in_string_1 * 4 + 2) = u_var3;
            fn_ptr_1 = (local_Struct64_ptr + 0x10);
            (**fn_ptr_1)(local_CS_1470, local_Struct64_ptr, u_var3)
        }
        0x42 => {
            paVar4 = struct_a;
            process_struct_1000_179c(0xc, struct_a);
            if ((paVar4 | local_Struct64_ptr) == 0) {}
            // goto LAB_1010_2126;
            pu_var1 = pass1_1008_ec10(local_Struct64_ptr, paVar4, in_string_1);
            u_var3 = (pu_var1 >> 0x10);
            local_Struct64_ptr = pu_var1
        }
        0x43 => {
            paVar4 = struct_a;
            process_struct_1000_179c(0x12, struct_a);
            u_var3 = paVar4 | local_Struct64_ptr;
            if (u_var3 == 0) {}
            // goto LAB_1010_2126;
            paVar2 = process_struct_1010_2bfc(local_Struct64_ptr, CONCAT22(in_string_1, paVar4));
            local_Struct64_ptr = paVar2
        }
        0x45 => {
            paVar4 = struct_a;
            process_struct_1000_179c(0xe, struct_a);
            if ((paVar4 | local_Struct64_ptr) == 0) {}
            // goto LAB_1010_2126;
            u_var7 = pass1_1010_0000(local_Struct64_ptr, paVar4);
            u_var3 = (u_var7 >> 0x10);
            local_Struct64_ptr = u_var7
        }
        0x46 => {
            paVar4 = struct_a;
            process_struct_1000_179c(0x1a, struct_a);
            if ((paVar4 | local_Struct64_ptr) == 0) {}
            // goto LAB_1010_2126;
            pass1_1010_50b2(local_Struct64_ptr, CONCAT22(in_string_1, paVar4));
            u_var3 = extraout_dx_27
        }
        0x47 => {
            paVar4 = struct_a;
            process_struct_1000_179c(0xe, struct_a);
            u_var3 = paVar4 | local_Struct64_ptr;
            if (u_var3 == 0) {}
            // goto LAB_1010_2126;
            pu_var1 = pass1_1018_56e6(local_Struct64_ptr, CONCAT22(in_string_1, paVar4));
            local_Struct64_ptr = pu_var1
        }
        0x48 => {
            paVar4 = struct_a;
            process_struct_1000_179c(0x1c, struct_a);
            if ((paVar4 | local_Struct64_ptr) == 0) {}
            // goto LAB_1010_2126;
            gui_get_info_func_1008_da12(local_Struct64_ptr, CONCAT22(in_string_1, paVar4));
            u_var3 = extraout_dx_15;
        }
    }
    local_6 = CONCAT22(u_var3, local_Struct64_ptr);
    // switchD_1010_2765_caseD_38:
    (struct_373_ptr + in_string_1 * 4) = local_6;
    return local_6;
}

pub unsafe fn pass1_1010_2816(in_struct: *mut Struct372) {
    let pu_var1: *mut u32;
    let mut u_var2: i32;
    let ppc_var3: fn();
    let local_bx_5: *mut Struct372;
    let mut i_var4: i32;
    let mut u_var5: u16;

    u_var5 = (in_struct >> 0x10);
    local_bx_5 = in_struct;
    if (local_bx_5.field_0x124 != 0) {
        i_var4 = local_bx_5.field_0x124 * 4;
        pu_var1 = (&local_bx_5.field_0x0 + i_var4);
        u_var2 = (&local_bx_5.field_0x2 + i_var4);
        if ((u_var2 | pu_var1) != 0) {
            unsafe {
                ppc_var3 = *pu_var1;
                (**ppc_var3)();
            }
        }
        (&local_bx_5.field_0x0 + local_bx_5.field_0x124 * 4) = 0;
        local_bx_5.field_0x124 = 0;
    }
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn pass1_1010_28e6(param_1: *mut Struct378, param_2: u32) {
    let mut u_var1: u32;
    let pu_var2: *mut u16;
    let mut u_var3: u16;
    let ctx.dx_reg: *mut u16;
    
    let mut i_var4: i32;
    let mut u_var5: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    pass1_1018_4cda(param_1, param_2);
    u_var5 = 0;
    &param_1.field_0x1c = 0;
    param_1.field_0x20 = 0;
    param_1.field_0x22 = 0;
    param_1.field_0x24 = 0;
    param_1.field_0x26 = 0;
    CONCAT22(param_2, param_1) = (s_add16_wav_1050_2bdc + 8);
    param_1.field_0x2 = 0x1010;
    PTR_LOOP_1050_4230 = param_1;
    PTR_LOOP_1050_4232 = param_2;
    pass1_1018_4dce(CONCAT13((param_2 >> 8), CONCAT12(param_2, param_1)), 0x56);
    mixed_fn_1010_830a(ctx._g_struct_73_1050_14cc, 4);
    param_1.field_0x1c = u_var5;
    param_1.field_0x1e = ctx.dx_reg;
    if (ctx.__g_Struct94_ptr_1 == 0) {
        ctx.g_u16_ptr_1050_5f2e = ctx.dx_reg;
        struct_fn_1000_160a();
        _g_Struct94_ptr_1._0_1_ = u_var5;
    } else {
    }
    pu_var2 = (s_New_failed_in_Op__Op__ResLibr_1050_0035 + 0xb);
    alloc_mem_1000_1708(0x40, 0, 1, _g_Struct94_ptr_1._0_1_, ctx.g_u16_ptr_1050_5f2e);
    param_1.field_0x28 = pu_var2;
    &param_1.field_0x2a = ctx.g_u16_ptr_1050_5f2e;
    local_6 = 0;
    while (local_6 < 0x10) {
        u_var3 = local_6 + 0x56;
        mixed_fn_1010_830a(ctx._g_struct_73_1050_14cc, u_var3);
        u_var1 = &param_1.field_0x28;
        u_var5 = (u_var1 >> 0x10);
        i_var4 = u_var1;
        (i_var4 + local_6 * 4) = u_var3;
        (i_var4 + local_6 * 4 + 2) = ctx.dx_reg;
        local_6 = local_6 + 1;
    }
    return;
}

pub unsafe fn pass1_1010_29c6(in_struct_a: *mut Struct376) {
    let pu_var1: *mut u32;
    let mut u_var2: i32;
    let struct_a_lo: *mut Struct376;
    let struct_a_hi: *mut Struct376;
    let fn_ptr_1: *mut u32;
    let fn_ptr_2: fn();

    struct_a_hi = (in_struct_a >> 0x10);
    struct_a_lo = in_struct_a;
    in_struct_a.ptr_a_lo = (s_add16_wav_1050_2bdc + 8);
    struct_a_lo.ptr_a_hi = 0x1010;
    if (struct_a_lo.field_0x1c != 0x0) {
        pu_var1 = &struct_a_lo.field_0x1c;
        u_var2 = (&struct_a_lo.field_0x1c + 2);
        if ((u_var2 | pu_var1) != 0) {
            unsafe {
                fn_ptr_2 = *pu_var1;
                (**fn_ptr_2)();
            }
        }
        struct_a_lo.field_0x1c = 0x0;
        error_check_1000_17ce(&struct_a_lo.field_0x28);
        &struct_a_lo.field_0x28 = 0;
    }
    win_cleanup_1018_4d22(in_struct_a);
    return;
}

// WARNING: Instruction at (ram,0x10104b2b) overlaps instruction at (ram,0x10104b2a)
//
// WARNING: Variable defined which should be unmapped: local_36
// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
// WARNING: Restarted to delay deadcode elimination for space: stack

pub unsafe fn pass1_1010_2b66(param_1: u32) {
    let mut u_var1: u16;

    u_var1 = (param_1 >> 0x10);
    return CONCAT22((param_1 + 0x1e), (param_1 + 0x1c));
}

pub unsafe fn pass1_1010_2b78(param_1: u16, param_2: u16, param_1_00: i32, param_2_00: u32) {
    let pu_var1: *mut u32;
    let pu_var2: *mut u32;
    let pu_var3: *mut u32;
    let mut i_var4: i32;
    let pu_var5: *mut u32;

    pu_var3 = (param_1_00 * 0x7c + 0xed4);
    pu_var5 = param_2_00;
    i_var4 = 0x1f;
    while (i_var4 != 0) {
        i_var4 = i_var4 + -1;
        pu_var2 = pu_var5;
        pu_var5 = pu_var5 + 1;
        pu_var1 = pu_var3;
        pu_var3 = pu_var3 + 1;
        unsafe {
            *pu_var2 = *pu_var1;
        }
    }
    return;
}

pub unsafe fn pass1_1010_2b98(param_1: u32, param_2: u16) {
    let mut in_eax: u32;
    let mut temp_5f57b00c45: u32;

    temp_5f57b00c45 = (param_1 + 0x28);
    return in_eax & 0xffff0000 | *(param_2 * 4 + temp_5f57b00c45 + -0x158);
}

pub unsafe fn pass1_1010_2bbe(param_1: *mut Struct376, param_2: u8) -> *mut Struct376 {
    pass1_1010_29c6(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1010_2c9c(param_1: *mut Struct376, param_2: u8) -> *mut Struct376 {
    pass1_1010_1d80(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1010_2db2(param_1: *mut Struct376) {
    let pu_var1: *mut u32;
    let mut u_var2: i32;
    let mut i_var3: i32;
    let mut local_es_5: u16;
    let temp_86259464166: *mut u32;
    let mut temp_5fc2c0f45a: u32;
    let fn_ptr_1: fn();

    local_es_5 = (param_1 >> 0x10);
    i_var3 = param_1;
    param_1.ptr_a_lo = 0x36da;
    (i_var3 + 2) = 0x1010;
    pu_var1 = (i_var3 + 0x56);
    u_var2 = (i_var3 + 0x58);
    if ((u_var2 | pu_var1) != 0) {
        unsafe {
            fn_ptr_1 = *pu_var1;
            (**fn_ptr_1)();
        }
    }
    error_check_1000_17ce((i_var3 + 0x5c));
    pass1_1010_1d80(param_1);
    return;
}

pub unsafe fn pass1_1010_2e02(param_1: u32, param_2: u16) {
    let mut u_var1: u32;
    let mut i_var2: i32;
    let mut u_var3: u16;

    u_var3 = (param_1 >> 0x10);
    if ((param_1 + 0x5c) != 0) {
        u_var1 = (param_1 + 0x5c);
        u_var3 = (u_var1 >> 0x10);
        i_var2 = u_var1;
        return CONCAT22((i_var2 + param_2 * 4 + 2), (i_var2 + param_2 * 4));
    }
    return 0;
}

pub unsafe fn pass1_1010_2e30(param_1: u32, param_2: u16, param_3: u16, param_4: u16) {
    let local_bx_21: *mut Struct386;
    let mut local_es_4: u16;
    let mut local_es_21: u16;
    let mut temp_5fac4a1950: u32;

    local_es_4 = (param_1 >> 0x10);
    if ((param_1 + 0x5c) != 0) {
        temp_5fac4a1950 = (param_1 + 0x5c);
        local_es_21 = (temp_5fac4a1950 >> 0x10);
        local_bx_21 = temp_5fac4a1950;
        (local_bx_21 + param_4 * 4) = param_2;
        (local_bx_21 + param_4 * 4 + 2) = param_3;
    }
    return;
}

pub unsafe fn pass1_1010_2e5c(param_1: u16, param_2: u16, param_2_00: u32) {
    let mut local_c: u32;
    let mut local_8: u32;

    local_c = param_2_00;
    if (param_2_00 == 0) {
        return;
    }
    while ((local_c & 0xf) != 0) {
        local_c = local_c >> 4;
    }
    return;
}

pub unsafe fn pass1_1010_2ee2(in_struct_1: *mut Struct388) {
    let pp_var1: fn();
    let mut u_var2: u32;
    let local_AX_32: *mut Struct381;
    
    
    let local_struct_1: *mut Struct318;
    let mut u_var3: u16;
    let mut local_6: u32;

    u_var3 = (in_struct_1 >> 0x10);
    local_struct_1 = in_struct_1;
    if (&local_struct_1[1].field_0xc != 0) {
        return;
    }
    local_AX_32 = 0x0;
    &local_struct_1.field_0x28 = 0;
    u_var2 = in_struct_1;
    pp_var1 = (u_var2 + 0x20);
    (**pp_var1)();
    if (local_AX_32 == 0x0) {
        local_6 = &local_struct_1[1].field_0x10;
    } else {
        pp_var1 = (u_var2 + 0x14);
        (**pp_var1)();
        local_6 = CONCAT22(ctx.dx_reg, local_AX_32);
        pass1_1010_2e02(in_struct_1, local_AX_32.field_0x12);
        process_struct_1010_35a4(in_struct_1, CONCAT22(ctx.dx_reg, local_AX_32));
    }
    win_gui_fn_1010_32f4(in_struct_1, local_6);
    pass1_1010_1f62(in_struct_1, 8);
    if (&local_struct_1[1].field_0xc != 0) {
        return;
    }
    return;
}

pub unsafe fn pass1_1010_32c0(param_1: u32, param_2: u32) {
    let mut u_var1: u16;

    u_var1 = (param_1 >> 0x10);
    (param_1 + 0x28) = 0;
    (param_1 + 0x12) = param_2;
    return;
}

pub unsafe fn pass1_1010_32da(param_1: u32, param_2: u32) {
    win_gui_fn_1010_32f4(param_1, (param_2 + 0x42));
    return;
}

pub unsafe fn pass1_1010_36b4(param_1: u32, param_2: u8) {
    pass1_1010_2db2(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1010_3702(
    param_1: *mut Struct375,
    param_2: *mut Struct375,
    param_3: u16,
) -> *mut Struct375 {
    process_struct_1010_1d48(CONCAT22(param_2, param_1), param_3);
    &param_1.ptr_2_lo = 0;
    (CONCAT22(param_2, param_1)).ptr_1_lo = 0x37c4;
    param_1.ptr_1_hi = 0x1010;
    return CONCAT22(param_2, param_1);
}

pub unsafe fn pass1_1010_3730(param_1: *mut Struct376) {
    let mut u_var1: u16;

    u_var1 = (param_1 >> 0x10);
    param_1.ptr_a_lo = 0x37c4;
    (param_1 + 2) = 0x1010;
    error_check_1000_17ce((param_1 + 10));
    pass1_1010_1d80(param_1);
    return;
}

pub unsafe fn pass1_1010_375e(param_1: u32) {
    let mut u_var1: u16;

    u_var1 = (param_1 >> 0x10);
    return CONCAT22((param_1 + 0xc), (param_1 + 10));
}

pub unsafe fn pass1_1010_3770(param_1: *mut Struct400, param_2: u32) {
    let u_var1: u8;
    let extraout_var: u32;
    let mut in_dx: u16;
    let local_bx_4: *mut Struct400;
    let mut u_var3: u16;
    let mut u_var2: u16;

    u_var3 = (param_1 >> 0x10);
    local_bx_4 = param_1;
    u_var1 = error_check_1000_17ce(&local_bx_4.field_0xa);
    u_var2 = CONCAT31(extraout_var, u_var1);
    pass1_fn_1008_60e8(param_2);
    &local_bx_4.field_0xa = u_var2;
    local_bx_4.field_0xc = in_dx;
    return;
}

pub unsafe fn pass1_1010_379e(param_1: u32, param_2: u8) {
    pass1_1010_3730(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1010_37d4(param_1: u32) {
    let mut u_var1: u16;

    pass1_1010_383a(param_1);
    u_var1 = (param_1 >> 0x10);
    (param_1 + 0x16) = 0;
    param_1 = (s_6_0_1050_3b3d + 1);
    (param_1 + 2) = 0x1010;
    return param_1;
}

pub unsafe fn pass1_1010_3800(param_1: *mut Struct401) {
    let local_bx_4: *mut Struct401;
    let mut local_es_4: u16;
    let mut temp_5f992dfc86: u32;

    local_es_4 = (param_1 >> 0x10);
    local_bx_4 = param_1;
    param_1 = (s_6_0_1050_3b3d + 1);
    local_bx_4.field_0x2 = 0x1010;
    if (local_bx_4.field_0x16 != 0) {
        error_check_1000_17ce(local_bx_4.field_0x16);
    }
    pass1_1010_3880(param_1);
    return;
}

pub unsafe fn pass1_1010_383a(param_1: *mut Struct402) {
    let local_bx_4: *mut Struct402;
    let mut u_var1: u16;

    u_var1 = (param_1 >> 0x10);
    local_bx_4 = param_1;
    param_1.a = ctx.s_1_1050_389a;
    local_bx_4.b = &ctx.PTR_LOOP_1050_1008;
    &local_bx_4.c = 0;
    local_bx_4.field_0x8 = 0;
    local_bx_4.field_0xc = 0;
    local_bx_4.field_0x10 = 0;
    local_bx_4.field_0x12 = 0;
    local_bx_4.field_0x14 = 0;
    param_1.a = (s_40_33_1050_3b5c + 2);
    local_bx_4.b = 0x1010;
    return;
}

pub unsafe fn pass1_1010_3880(param_1: *mut Struct404) {
    let pu_var1: *mut u16;
    let pu_var2: *mut u32;
    let mut u_var3: i32;
    let ppcVar4: fn();
    let mut u_var5: u32;
    let local_bx_5: *mut Struct404;
    let local_bx_39: *mut Struct403;
    let mut u_var6: u16;
    let mut u_var7: u16;
    let mut local_4: u16;

    u_var6 = (param_1 >> 0x10);
    local_bx_5 = param_1;
    param_1.a = (s_40_33_1050_3b5c + 2);
    local_bx_5.b = 0x1010;
    if (local_bx_5.field_0x8 != 0) {
        local_4 = 0;
        loop {
            pu_var1 = &local_bx_5.field_0x10;
            let pu_var1_val = unsafe { *pu_var1 };
            if (pu_var1_val == local_4 || pu_var1_val < local_4) {
                break;
            }
            u_var5 = local_bx_5.field_0x8;
            u_var7 = (u_var5 >> 0x10);
            local_bx_39 = u_var5;
            pu_var2 = (local_bx_39 + local_4 * 4);
            u_var3 = (local_bx_39 + local_4 * 4 + 2);
            if ((u_var3 | pu_var2) != 0) {
                unsafe {
                    ppcVar4 = *pu_var2;
                }
                (**ppcVar4)();
            }
            local_4 = local_4 + 1;
        }
        error_check_1000_17ce(local_bx_5.field_0x8);
    }
    param_1.a = ctx.s_1_1050_389a;
    local_bx_5.b = &ctx.PTR_LOOP_1050_1008;
    return;
}

pub unsafe fn pass1_1010_398e(param_1: *mut u32, param_2: u16, param_3: u16, param_3_00: u32) {
    let piVar1: *mut i32;
    let ppc_var2: fn();
    let mut u_var3: u32;
    let mut u_var4: u32;
    let mut in_ax: i32;
    let mut i_var5: i32;
    let mut u_var6: u16;
    
    
    
    let local_struct_1_cpy: *mut Struct406;
    let mut i_var7: i32;
    let mut i_var8: i32;
    let mut u_var9: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_8: u16;
    let local_struct_1: *mut Struct406;
    let mut local_4: u16;

    u_var9 = (param_1 >> 0x10);
    unsafe {
        u_var3 = *param_1;
    }
    ppc_var2 = (u_var3 + 8);
    ppc_var2();
    _local_struct_1 = CONCAT22(ctx.dx_reg, in_ax);
    if ((ctx.dx_reg | in_ax) == 0) {
        return;
    }
    (in_ax + 0xc) = param_3_00;
    local_struct_1_cpy = *_local_struct_1;
    ppc_var2 = &local_struct_1_cpy.field_0xc;
    ppc_var2();
    i_var5 = (param_1 + 0x14);
    piVar1 = (param_1 + 0x14);
    unsafe {
        *piVar1 = *piVar1 + 1;
    }
    ppc_var2 = &local_struct_1_cpy.field_0x10;
    ppc_var2();
    ppc_var2 = &local_struct_1_cpy.field_0x4;
    ppc_var2();
    if (i_var5 != 0) {
        ppc_var2 = (u_var3 + 8);
        i_var7 = i_var5;
        ppc_var2();
        (in_ax + 8) = i_var7;
        (in_ax + 10) = ctx.dx_reg;
        PTR_LOOP_1050_11de = PTR_LOOP_1050_11de + 1;
        local_c = 0;
        while (local_c < i_var5) {
            u_var6 = local_c;
            pass1_1010_398e(param_1, local_c, local_c >> 0xf, _local_struct_1);
            u_var4 = (in_ax + 8);
            u_var9 = (u_var4 >> 0x10);
            i_var7 = u_var4;
            i_var8 = local_c * 4;
            (i_var7 + i_var8) = u_var6;
            (i_var7 + i_var8 + 2) = ctx.dx_reg;
            u_var4 = (in_ax + 8);
            if ((u_var4 + i_var8) == 0) {
                break;
            }
            local_c = local_c + 1;
        }
        PTR_LOOP_1050_11de = PTR_LOOP_1050_11de + -1;
    }
    return;
}

pub unsafe fn pass1_1010_3a86(param_1: u32) {
    return (param_1 + 0x10);
}

pub unsafe fn pass1_1010_3a94(param_1: u32, param_2: u16) {
    (param_1 + 0x12) = param_2;
    return;
}

pub unsafe fn pass1_1010_3aaa(param_1: u32) {
    let mut u_var1: u16;

    u_var1 = (param_1 >> 0x10);
    return CONCAT22((param_1 + 6), (param_1 + 4));
}

pub unsafe fn pass1_1010_3ac2(param_1: u32, param_2: u16, param_3: u32) {
    let mut u_var1: u16;

    u_var1 = (param_1 >> 0x10);
    (param_1 + 0x16) = param_3;
    (param_1 + 0x12) = param_2;
    return;
}

pub unsafe fn pass1_1010_3adc(param_1: u32) {
    let pu_var1: *mut u16;

    pu_var1 = (param_1 + 0x16);
    let pu_var1_val = unsafe { *pu_var1 };
    return CONCAT22((pu_var1 + 2), pu_var1_val);
}

pub unsafe fn pass1_1010_3af2(param_1: u32, param_2: u8) {
    pass1_1010_3800(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1010_3b18(param_1: u32, param_2: u8) {
    pass1_1010_3880(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1010_3bde(param_1: *mut Struct376) {
    let pu_var1: *mut u32;
    let mut u_var2: i32;
    let pHVar3: *mut HDC16;
    let local_bx_5: *mut Struct376;
    let mut u_var4: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let fn_ptr_1: fn();

    u_var4 = (param_1 >> 0x10);
    local_bx_5 = param_1;
    param_1.ptr_a_lo = 0x3d6a;
    local_bx_5.ptr_a_hi = 0x1010;
    local_bx_5.dc_handle_x0a = 0x3d7a;
    local_bx_5.u16_x0c = 0x1010;
    pu_var1 = local_bx_5.u16_x0e;
    u_var2 = local_bx_5.u16_x10;
    if ((u_var2 | pu_var1) != 0) {
        unsafe {
            fn_ptr_1 = *pu_var1;
            (**fn_ptr_1)();
        }
    }
    if (param_1 == 0x0) {
        pHVar3 = 0x0;
        u_var4 = 0;
    } else {
        pHVar3 = &local_bx_5.dc_handle_x0a;
    }
    _local_e = CONCAT22(u_var4, pHVar3);
    *_local_e = ctx.s_1_1050_389a;
    pHVar3[1] = (HDC16) & ctx.PTR_LOOP_1050_1008;
    pass1_1010_1d80(param_1);
    return;
}

pub unsafe fn pass1_1010_3c52(param_1: *mut Struct397, param_2: u16) {
    let pu_var1: *mut u32;
    let mut u_var2: i32;
    let ppc_var3: fn();
    let local_bx_8: *mut Struct397;
    let mut u_var4: u16;
    let mut u_var5: u32;

    u_var4 = (param_1 >> 0x10);
    local_bx_8 = param_1;
    local_bx_8.field_0x14 = param_2;
    pu_var1 = local_bx_8.field_0xe;
    u_var2 = local_bx_8.field_0x10;
    if ((u_var2 | pu_var1) != 0) {
        unsafe {
            ppc_var3 = *pu_var1;
        }
        (**ppc_var3)();
    }
    u_var5 = mixed_fn_1010_830a(ctx._g_struct_73_1050_14cc, local_bx_8.field_0x14);
    local_bx_8.field_0xe = u_var5;
    local_bx_8.field_0x10 = (u_var5 >> 0x10);
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn pass1_1010_3c9e(param_1: libc::c_long) {
    let mut iVar1: i32;
    let mut u_var2: u16;

    if (param_1 == 0) {
        iVar1 = 0;
        u_var2 = 0;
    } else {
        iVar1 = param_1 + 10;
        u_var2 = param_1._2_2_;
    }
    process_struct_1008_9262(
        _PTR_LOOP_1050_0388,
        (_PTR_LOOP_1050_0388 >> 0x10),
        *(param_1 + 0x12),
        CONCAT22(u_var2, iVar1),
    );
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn pass1_1010_3cd0(param_1: libc::c_long) {
    let mut iVar1: i32;
    let mut u_var2: u16;

    if (_PTR_LOOP_1050_0388 != 0) {
        if (param_1 == 0) {
            iVar1 = 0;
            u_var2 = 0;
        } else {
            iVar1 = param_1 + 10;
            u_var2 = param_1._2_2_;
        }
        pass1_1008_92b2(_PTR_LOOP_1050_0388, (param_1 + 0x12), 0, iVar1, u_var2);
    }
    return;
}

pub unsafe fn pass1_1010_3d0a(param_1: u16, param_2: u16, param_3: u16) {
    if (param_3 == 2) {
        pass1_1010_3cd0(0);
        pass1_1010_1f62(CONCAT22(param_2, param_1 - 10), 2);
    }
    return;
}

pub unsafe fn pass1_1010_3d44(param_1: u32, param_2: u8) {
    pass1_1010_3bde(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn pass1_1010_3d82(param_1: *mut Struct396, param_2: u32) {
    let mut u_var1: u32;
    let mut u_var2: u16;

    u_var2 = param_2;
    process_struct_1010_1d48(CONCAT22(u_var2, param_1), (param_2 >> 0x10));
    &param_1.field_0xa = 0;
    CONCAT22(u_var2, param_1) = 0x3e2c;
    param_1.field_0x2 = 0x1010;
    u_var1 = mixed_fn_1010_830a(ctx._g_struct_73_1050_14cc, 0x99);
    param_1.field_0xa = u_var1;
    param_1.field_0xc = (u_var1 >> 0x10);
    return CONCAT22(u_var2, param_1);
}

pub unsafe fn pass1_1010_3dc8(param_1: *mut Struct376) {
    let pu_var1: *mut u32;
    let mut u_var2: i32;
    let local_struct_1: *mut Struct395;
    let mut local_struct_1_hi: u16;
    let fn_ptr_1: fn();

    local_struct_1_hi = (param_1 >> 0x10);
    local_struct_1 = param_1;
    param_1.ptr_a_lo = 0x3e2c;
    local_struct_1.field_0x2 = 0x1010;
    pu_var1 = local_struct_1.field_0xa;
    u_var2 = local_struct_1.field_0xc;
    if ((u_var2 | pu_var1) != 0) {
        unsafe {
            fn_ptr_1 = *pu_var1;
            (**fn_ptr_1)();
        }
    }
    pass1_1010_1d80(param_1);
    return;
}

pub unsafe fn pass1_1010_3e06(param_1: u32, param_2: u8) {
    pass1_1010_3dc8(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1010_40cc(param_1: u32) {
    let ppVar1: *mut pass1_struct_2;
    let mut in_dx: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    ppVar1 = pass1_1030_8344(
        ctx._g_bool_1050_5748,
        (ctx._g_bool_1050_5748 >> 0x10),
        (param_1 + 0x6c),
    );
    return CONCAT22(&ppVar1.field_0xe, &ppVar1.field_0xc);
}

pub unsafe fn pass1_1010_41d6(param_1: *mut Struct408, param_2: u32) {
    let pu_var1: *mut u32;
    let p_uvar2: *mut u16;
    let mut u_var3: u32;
    let pu_var4: *mut u16;
    let ppVar5: *mut pass1_struct_2;
    let paVar6: *mut Struct199;
    let mut u_var7: u16;
    let mut u_var8: u32;
    let mut u_var9: i32;
    
    let mut iVar10: i32;
    let local_bx_10: *mut Struct408;
    let mut i_var11: i32;
    let mut unaff_si: u16;
    let mut u_var12: u16;
    let mut u_var13: u16;
    let mut unaff_ss: u16;
    let ppVar14: *mut pass1_struct_1;
    let mut local_36: u32;
    let mut local_32: u16;
    let mut local_30: u16;
    let mut local_2e: u32;
    let mut local_2a: u16;
    let mut local_28: u32;
    let mut local_22: u32;
    let mut local_1e: u32;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: u32;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    u_var12 = (param_1 >> 0x10);
    local_bx_10 = param_1;
    local_bx_10.field_0x6c = param_2;
    ppVar14 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(unaff_si, 0x2f));
    local_4 = (ppVar14 >> 0x10);
    local_6 = ppVar14;
    _local_a = pass1_1010_ec40(local_6, local_4, local_bx_10.field_0x6c);
    paVar6 = (_local_a >> 0x10);
    local_bx_10.field_0x74 = (_local_a + 0x22);
    if (&local_bx_10.field_0x70 != 0) {
        local_22 = &local_bx_10.field_0x70;
        local_1e = local_22;
        error_check_1000_17ce(local_22);
        &local_bx_10.field_0x70 = 0;
    }
    pu_var4 = (local_bx_10.field_0x74 << 7);
    process_struct_1000_179c(pu_var4, paVar6);
    local_bx_10.field_0x70 = pu_var4;
    &local_bx_10.field_0x72 = paVar6;
    ppVar5 = pass1_1030_8344(
        ctx._g_bool_1050_5748,
        (ctx._g_bool_1050_5748 >> 0x10),
        local_bx_10.field_0x6c,
    );
    _local_e = CONCAT22(paVar6, ppVar5);
    local_10 = (&ppVar5.field_0x10 == 9);
    local_16 = (_local_a + 0x22);
    u_var7 = local_16 * 6;
    process_struct_1000_179c(u_var7, paVar6);
    local_1e = CONCAT22(paVar6, u_var7);
    u_var9 = paVar6 | u_var7;
    if (u_var9 == 0) {
        local_14 = 0;
    } else {
        call_fn_ptr_1000_5586(0x3e38, &ctx.PTR_LOOP_1050_1008, local_16, 6, u_var7, paVar6);
        local_14 = local_1e;
    }
    local_18 = 0;
    loop {
        u_var13 = (_local_a >> 0x10);
        pu_var1 = (_local_a + 0x22);
        let pu_var1_val = unsafe { *pu_var1 };
        if (pu_var1_val < local_18 || pu_var1_val == local_18) {
            break;
        }
        u_var3 = (_local_a + 0x24);
        u_var7 = local_18;
        pass1_1028_e0a0(ctx._PTR_LOOP_1050_65e2, *(u_var3 + local_18 * 2) << 0x10);
        local_22 = CONCAT22(u_var9, u_var7);
        modify_list_1008_3f62(
            (local_14 & 0xff000000 | CONCAT12((local_14 >> 0x10), local_18 * 6 + local_14)),
            CONCAT22(u_var9, u_var7 + 8),
        );
        local_28 = local_22;
        local_1e = local_22;
        u_var9 = ctx.dx_reg;
        if (local_22 != 0) {
            pass1_1030_84d0(local_22);
            error_check_1000_17ce(local_22);
        }
        local_18 = local_18 + 1;
    }
    local_1a = 0;
    while (
        pu_var2 = &local_bx_10.field_0x74,
        *pu_var2 != local_1a && local_1a <= *pu_var2,
    ) {
        paVar6 = (local_1a * 6 + local_14);
        u_var8 = ZEXT24(paVar6);
        pass1_1008_3e94(
            paVar6,
            CONCAT13((unaff_ss >> 8), CONCAT12(unaff_ss, &local_2e)),
            CONCAT22(unaff_ss, &local_30),
        );
        xor_1000_49b2(local_2e);
        u_var8 = ((u_var8 >> 0xf) << 0x10 | u_var8 & 0xffff) / 5 & 0xffff;
        u_var7 = u_var8;
        local_2a = u_var7;
        if (0xc < u_var7) {
            local_2a = 0xc;
            xor_1000_49b2(local_2e);
            u_var9 = (local_2e / u_var7) * 0x3c;
            u_var8 = u_var9;
            local_2e = local_2e & 0xffff0000 | u_var9;
        }
        xor_1000_49b2(local_2e);
        u_var8 = ((u_var8 >> 0xf) << 0x10 | u_var8 & 0xffff) % 5;
        iVar10 = u_var8;
        local_22 = local_22 & 0xffff0000 | u_var8 & 0xffff;
        if (local_2e < 0) {
            if (2 < iVar10) {
                iVar10 = iVar10 + -5;
            }
            local_2e = local_2e & 0xffff0000 | (local_2e + iVar10);
        } else {
            if (iVar10 < 3) {
                local_2e = local_2e & 0xffff0000 | (local_2e - iVar10);
            } else {
                local_2e = local_2e & 0xffff0000 | (local_2e + (5 - iVar10));
            }
        }
        local_32 = local_30 / 0x16;
        local_36 = 0;
        while (local_36 < 0x10) {
            if (0xf < local_32) {
                local_32 = 0;
            }
            if (((local_10 != 0) < local_32) && (local_32 < 8)) {
                iVar10 = ((local_2a * 0x10 + local_32) * 2 + 0x11e0);
                i_var11 = (local_1a * 0x10 + local_36) * 8;
                u_var3 = &local_bx_10.field_0x70;
                (i_var11 + u_var3) = iVar10 + 0x49;
                u_var3 = &local_bx_10.field_0x70;
                (u_var3 + i_var11 + 2) = local_2e + 0x49;
                u_var3 = &local_bx_10.field_0x70;
                (u_var3 + i_var11 + 4) = iVar10 + 0x4e;
                u_var3 = &local_bx_10.field_0x70;
                (u_var3 + i_var11 + 6) = local_2e + 0x4e;
            } else {
                iVar10 = (local_1a * 0x10 + local_36) * 8;
                u_var3 = &local_bx_10.field_0x70;
                (iVar10 + u_var3) = 0;
                u_var3 = &local_bx_10.field_0x70;
                (u_var3 + iVar10 + 2) = 0;
                u_var3 = &local_bx_10.field_0x70;
                (u_var3 + iVar10 + 4) = 1;
                u_var3 = &local_bx_10.field_0x70;
                (u_var3 + iVar10 + 6) = 1;
            }
            local_32 = local_32 + 1;
            local_36 = (local_36 + 1);
        }
        local_1a = local_1a + 1;
    }
    local_28 = local_14;
    local_2e = local_14;
    error_check_1000_17ce(local_14);
    draw_1010_47ae(param_1);
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn pass1_1010_451a(param_1: u32) {
    let mut u_var1: u16;
    let ppVar2: *mut pass1_struct_1;
    let mut u_var3: u32;
    let mut in_stack_0000fff6: u16;

    ppVar2 = process_struct_1010_20ba(
        ctx._g_Struct372_1050_0ed0,
        CONCAT22(in_stack_0000fff6, 0x2f),
    );
    u_var3 = pass1_1010_ec40(ppVar2, (ppVar2 >> 0x10), (param_1 + 0x6c));
    u_var1 = (u_var3 >> 0x10);
    return CONCAT22((u_var3 + 4), (u_var3 + 2));
}

pub unsafe fn pass1_1010_454a(param_1: *mut Struct417) {
    let local_bx_4: *mut Struct417;
    let local_SI_11: *mut Struct416;
    let mut u_var1: u16;

    u_var1 = (param_1 >> 0x10);
    local_bx_4 = param_1;
    local_SI_11 = (local_bx_4.field_0x24 * 4);
    return CONCAT22(
        (local_bx_4 + local_SI_11 + 0x28),
        (local_bx_4 + local_SI_11 + 0x26),
    );
}

pub unsafe fn pass1_1010_4566(param_1: u16, param_2: u16, param_3: u16) {
    if (param_3 != 2) {
        return;
    }
    pass1_1010_4956(CONCAT22(param_2, param_1 - 0x20));
    pass1_1010_1f62(CONCAT22(param_2, param_1 - 0x20), 2);
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn pass1_1010_459e(param_1: libc::c_long) {
    let mut iVar1: i32;
    let mut u_var2: u16;

    if (param_1 == 0) {
        iVar1 = 0;
        u_var2 = 0;
    } else {
        iVar1 = param_1 + 0x20;
        u_var2 = param_1._2_2_;
    }
    process_struct_1008_9262(
        _PTR_LOOP_1050_0388,
        (_PTR_LOOP_1050_0388 >> 0x10),
        500,
        CONCAT22(u_var2, iVar1),
    );
    (param_1 + 0x7e) = 1;
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn pass1_1010_45d6(param_1: *mut Struct419) {
    let pu_var1: *mut u32;
    let mut u_var2: i32;
    let ppc_var3: fn();
    let mut i_var4: i32;
    let mut u_var5: u16;
    let local_bx_5: *mut Struct419;
    let mut u_var6: u16;
    let mut unaff_cs: u16;
    let mut local_4: u16;

    u_var6 = (param_1 >> 0x10);
    local_bx_5 = param_1;
    if (local_bx_5.field_0x7e != 0) {
        if (_PTR_LOOP_1050_0388 != 0) {
            if (param_1 == 0x0) {
                i_var4 = 0;
                u_var5 = 0;
            } else {
                i_var4 = &local_bx_5.field_0x20;
                u_var5 = u_var6;
            }
            unaff_cs = SUB42(&ctx.PTR_LOOP_1050_1008, 0);
            pass1_1008_92b2(_PTR_LOOP_1050_0388, 500, i_var4, u_var5);
        }
        local_4 = 0;
        while (local_4 < 0x10) {
            if (local_bx_5.field_0x24 != local_4) {
                pu_var1 = (&local_bx_5.field_0x26 + local_4 * 4);
                u_var2 = (&local_bx_5.field_0x26 + local_4 * 4 + 2);
                if ((u_var2 | pu_var1) != 0) {
                    unsafe {
                        ppc_var3 = *pu_var1;
                    }
                    (**ppc_var3)(unaff_cs, pu_var1, u_var2, 1);
                }
                (&local_bx_5.field_0x26 + local_4 * 4) = 0;
            }
            local_4 = local_4 + 1;
        }
        local_bx_5.field_0x7e = 0;
    }
    return;
}

pub unsafe fn pass1_1010_4674(in_struct_1: *mut Struct419, param_2: i32) {
    Struct30 * *ppaVar1;
    let local_struct_1: *mut Struct420;
    let mut u_var2: u16;

    local_struct_1 = in_struct_1;
    u_var2 = (in_struct_1 >> 0x10);
    if (param_2 == 1) {
        ppaVar1 = &local_struct_1.Struct30_field_0x24;
        *ppaVar1 = &(*ppaVar1).field_0x1;
        if (0xf < local_struct_1.Struct30_field_0x24) {
            local_struct_1.Struct30_field_0x24 = 0x0;
        }
        // LAB_1010_469a:
        draw_1010_47d0(in_struct_1, local_struct_1.Struct30_field_0x24);
    } else {
        if (param_2 != 2) {
            if (param_2 != 3) {
                if ((local_struct_1.field_0x6a != 0) && (local_struct_1.field_0x6a != 4)) {
                    pass1_1010_459e(local_struct_1, u_var2);
                }
                // goto LAB_1010_46e8;
            }
            ppaVar1 = &local_struct_1.Struct30_field_0x24;
            *ppaVar1 = &(*ppaVar1)[-1].field_0x104;
            if (*ppaVar1 < 0) {
                local_struct_1.Struct30_field_0x24 = (&PTR_LOOP_1050_000e + 1);
            }
            // goto LAB_1010_469a;
        }
    }
    pass1_1010_1f62(in_struct_1, 2);
    pass1_1010_45d6(in_struct_1);
    // LAB_1010_46e8:
    local_struct_1.field_0x6a = param_2;
    return;
}

pub unsafe fn pass1_1010_4788(param_1: u32, param_2: u32) {
    let ppVar1: *mut pass1_struct_2;
    let mut in_dx: u16;

    ppVar1 = pass1_1030_8344(
        ctx._g_bool_1050_5748,
        (ctx._g_bool_1050_5748 >> 0x10),
        (param_1 + 0x6c),
    );
    pass1_1030_301a(CONCAT22(in_dx, ppVar1), param_2);
    return;
}

pub unsafe fn pass1_1010_4956(param_1: u32) {
    let piVar1: *mut i32;
    let mut i_var2: i32;
    let local_bx_3: *mut Struct418;
    let mut u_var3: u16;

    u_var3 = (param_1 >> 0x10);
    local_bx_3 = param_1;
    i_var2 = local_bx_3.field_0x6a;
    if (i_var2 == 0) {
        piVar1 = &local_bx_3.field_0x24;
        unsafe {
            *piVar1 = *piVar1 + 1;
        }
        if (0xf < local_bx_3.field_0x24) {
            local_bx_3.field_0x24 = 0;
            return;
        }
    } else {
        if (i_var2 != 4) {
            return;
        }
        piVar1 = &local_bx_3.field_0x24;
        unsafe {
            *piVar1 = *piVar1 + -1;
            if (*piVar1 < 0) {
                local_bx_3.field_0x24 = 0xf;
            }
        }
    }
    return;
}

pub unsafe fn pass1_1010_49a0(param_1: u16, param_2: u16) {
    return CONCAT22(param_2, param_1 + 10);
}

pub unsafe fn pass1_1010_49b0(param_1: u16, param_2: u16) {
    return CONCAT22(param_2, param_1 + 0x18);
}

pub unsafe fn pass1_1010_49c0(param_1: u32) {
    return (param_1 + 0x14);
}

pub unsafe fn pass1_1010_49ce(param_1: u32, param_2: u16) {
    (param_1 + 0x14) = param_2;
    return;
}

pub unsafe fn pass1_1010_49e0(param_1: u32) {
    return (param_1 + 0x16);
}

pub unsafe fn pass1_1010_49ee(param_1: u32, param_2: u16) {
    (param_1 + 0x16) = param_2;
    return;
}

pub unsafe fn pass1_1010_4a00(param_1: u32, param_2: u16) {
    (param_1 + 0x12) = param_2;
    return;
}

pub unsafe fn pass1_1010_4a12(param_1: u32) {
    return (param_1 + 0x12);
}

pub unsafe fn pass1_1010_4a20(param_1: u32, param_2: u8) {
    pass1_1010_3f00(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1010_4c2c(param_1: u32) {
    let mut u_var1: u16;

    u_var1 = (param_1 >> 0x10);
    return CONCAT22((param_1 + 0x18), (param_1 + 0x16));
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn pass1_1010_4c3e(param_1: u32, param_2: u16) {
    let pu_var1: *mut u16;
    let mut i_var2: i32;
    let mut u_var3: u32;
    let mut in_ax: i32;
    let mut u_var4: u32;
    
    
    let local_bx_5: *mut Struct414;
    let local_bx_145: *mut Struct415;
    let mut u_var5: u16;
    let mut u_var6: u16;
    let mut unaff_ss: u16;
    let mut u_var7: u32;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: [u8; 6];
    let mut local_6: u16;
    let mut local_4: u16;

    u_var5 = (param_1 >> 0x10);
    local_bx_5 = param_1;
    pass1_1010_bffa(local_bx_5.field_0x26);
    local_bx_5.field_0x12 = in_ax;
    &local_bx_5.field_0x14 = ctx.dx_reg;
    if ((ctx.dx_reg | local_bx_5.field_0x12) != 0) {
        if (param_2 == 0) {
            u_var4 = &local_bx_5.field_0x12;
            local_bx_5.field_0x30 = (u_var4 + 8);
        } else {
            local_bx_5.field_0x2e = 1;
            u_var4 = &local_bx_5.field_0x12;
            u_var4 = (u_var4 + 4);
            i_var2 = (u_var4 + 2);
            if ((i_var2 == 5) || (i_var2 == 6)) {
                local_bx_5.field_0x30 = 1;
                local_bx_5.field_0x20 = 0;
            } else {
                local_bx_5.field_0x30 = 2;
                u_var4 = &local_bx_5.field_0x12;
                u_var4 = (u_var4 + 4);
                local_bx_5.field_0x32 = u_var4;
                mixed_fn_1010_830a(ctx._g_struct_73_1050_14cc, 0x1bf);
                u_var3 = &local_bx_5.field_0x12;
                u_var6 = (u_var3 >> 0x10);
                local_bx_145 = u_var3;
                local_bx_145.field_0x4 = u_var4;
                local_bx_145.field_0x6 = ctx.dx_reg;
            }
        }
        local_4 = 0x14;
        zero_list_1008_3e38(CONCAT22(unaff_ss, local_c));
        local_6 = 0;
        local_e = 0;
        loop {
            pu_var1 = &local_bx_5.field_0x30;
            unsafe {
                if (*pu_var1 == local_e || *pu_var1 < local_e) {
                    break;
                }
            }
            u_var4 = &local_bx_5.field_0x12;
            u_var7 = process_struct_1008_4772((u_var4 + local_e * 4));
            local_4 = local_4 + (-(local_e == 0) & 5) + 0x14 + (u_var7 + 4);
            local_e = local_e + 1;
        }
        if (local_bx_5.field_0xe < local_4) {
            local_bx_5.field_0xe = local_4;
        }
    }
    return;
}

pub unsafe fn pass1_1010_4dc8(param_1: u32) {
    let local_bx_3: *mut Struct411;
    let mut u_var1: u16;

    u_var1 = (param_1 >> 0x10);
    local_bx_3 = param_1;
    if (local_bx_3.field_0x20 == 0) {
        return 0;
    }
    return CONCAT22(
        local_bx_3.field_0x1c,
        local_bx_3.field_0x20 * 8 + local_bx_3.field_0x1a,
    );
}

pub unsafe fn pass1_1010_4df0(param_1: u32) {
    let mut u_var1: u32;
    let mut u_var2: u16;

    u_var2 = (param_1 >> 0x10);
    u_var1 = (param_1 + 0x26);
    pass1_1010_c1ba(u_var1, (u_var1 >> 0x10), (param_1 + 0x20));
    return;
}

pub unsafe fn pass1_1010_4e8c(param_1: *mut Struct318) {
    pass1_1010_1f62(param_1, 0xd);
    return;
}

pub unsafe fn pass1_1010_4f20(in_struct_60_ptr_1: *mut Struct60, param_2: u16, param_1_00: i32) {
    return (param_1_00 * 2 + 0x139a);
}

pub unsafe fn pass1_1010_4f30(
    param_1: u16,
    param_2: u16,
    param_1_00: *mut u16,
    param_2_00: *mut u16,
) {
    unsafe {
        *param_2_00 = 10;
        *param_1_00 = 0x73;
    }
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn pass1_1010_4f48(param_1: *mut Struct409) {
    let mut u_var1: i32;
    let pu_var2: *mut u32;
    let ppc_var3: fn();
    let mut u_var4: u32;
    let pu_var5: *mut u32;
    
    let local_bx_5: *mut Struct409;
    let local_bx_35: *mut Struct410;
    let mut u_var6: u16;
    let mut u_var7: u16;
    let mut local_6: u32;

    u_var6 = (param_1 >> 0x10);
    local_bx_5 = param_1;
    u_var4 = local_bx_5.field_0x12;
    local_bx_5.field_0x30 = (u_var4 + 8);
    if (local_bx_5.field_0x32 != 0) {
        u_var4 = local_bx_5.field_0x12;
        u_var7 = (u_var4 >> 0x10);
        local_bx_35 = u_var4;
        pu_var2 = local_bx_35.field_0x4;
        local_bx_35.field_0x4 = local_bx_5.field_0x32;
        if (pu_var2 != 0x0) {
            unsafe {
                ppc_var3 = *pu_var2;
                (**ppc_var3)();
            }
        }
        local_bx_5.field_0x32 = 0;
    }
    pu_var5 = local_bx_5.field_0x16;
    u_var1 = local_bx_5.field_0x18;
    if ((u_var1 | pu_var5) != 0) {
        unsafe {
            ppc_var3 = *pu_var5;
            (**ppc_var3)();
        }
    }
    mixed_fn_1010_830a(ctx._g_struct_73_1050_14cc, 0x1b3);
    local_bx_5.field_0x16 = pu_var5;
    local_bx_5.field_0x18 = ctx.dx_reg;
    error_check_1000_17ce(local_bx_5.field_0x1a);
    local_bx_5.field_0x1a = 0;
    local_bx_5.field_0x2e = 0;
    return;
}

pub unsafe fn pass1_1010_5004(param_1: u32, param_2: u8) {
    free_rsrc_1010_4b3e(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn pass1_1010_503e(param_1: *mut Struct534, param_2: u16, param_3: u16) {
    pass1_1018_4cda(param_1, CONCAT22(param_3, param_2));
    CONCAT22(param_2, param_1) = (s_SinternalPutBldg2_site_0x_08lx__1050_5099 + 1);
    param_1.u16_x02 = 0x1010;
    pass1_1018_4dce(CONCAT22(param_2, param_1), 0x1b3);
    _PTR_LOOP_1050_4230 = CONCAT22(param_2, param_1);
    return;
}

pub unsafe fn pass1_1010_5074(param_1: u32, param_2: u8) {
    win_cleanup_1018_4d22(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1010_50b2(param_1: *mut Struct421, param_2: u32) {
    let mut u_var1: u16;

    u_var1 = param_2;
    process_struct_1010_1d48(CONCAT22(u_var1, param_1), (param_2 >> 0x10));
    param_1.field_0xa = 0;
    param_1.field_0xc = 0;
    param_1.field_0x10 = 0;
    param_1.field_0x12 = 0;
    param_1.field_0x16 = 0;
    CONCAT22(u_var1, param_1) = 0x53f4;
    param_1.field_0x2 = 0x1010;
    return;
}

pub unsafe fn pass1_1010_50f2(param_1: *mut Struct376) {
    let mut u_var1: u16;

    u_var1 = (param_1 >> 0x10);
    param_1.ptr_a_lo = 0x53f4;
    (param_1 + 2) = 0x1010;
    error_check_1000_17ce((param_1 + 0xc));
    pass1_1010_1d80(param_1);
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn pass1_1010_5120(param_1: u32, param_2: u16) {
    let mut u_var1: u32;
    let ctx.ax_reg: *mut Struct423;
    let paVar2: *mut Struct493;
    let mut u_var3: i32;
    let mut u_var4: i32;
    let mut u_var5: u32;
    let mut in_dx: i32;
    
    
    let mut i32_var6: i32;
    let local_bx_4: *mut Struct422;
    let mut u_var7: u16;
    let mut u_var8: u16;
    let mut u_var9: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    u_var7 = (param_1 >> 0x10);
    local_bx_4 = param_1;
    if (local_bx_4.field_0x16 != 0) {
        u_var1 = local_bx_4.field_0x16;
        paVar2 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, u_var1, (u_var1 >> 0x10));
        if ((in_dx | paVar2) != 0) {
            u_var5 = paVar2[0x10].field_0x16;
            u_var8 = u_var5;
            u_var9 = (u_var5 >> 0x10);
            pass1_1030_38f2(u_var8, u_var9, 3);
            u_var3 = u_var5;
            u_var4 = u_var3;
            pass1_1030_38f2(u_var8, u_var9, 4);
            i32_var6 = ctx.dx_reg + ctx.dx_reg + CARRY2(u_var4, u_var3);
            if ((0 < i32_var6) || (-1 < i32_var6 && (param_2 <= u_var4 + u_var3))) {
                local_bx_4.field_0xa = param_2;
                return;
            }
        }
    }
    return;
}

// WARNING: Could not reconcile some variable overlaps

pub unsafe fn pass1_1010_519a(param_1: *mut Struct424, param_2: *mut u16) {
    let mut u_var1: u16;
    let mut u_var2: u32;
    let pu_var3: *mut u16;
    let in_dx: *mut Struct199;
    
    let local_bx_32: *mut Struct424;
    let mut i_var4: i32;
    let mut i_var5: i32;
    let mut u_var6: u16;
    let mut u_var7: u16;
    let mut unaff_ss: u16;
    let mut local_2c: u16;
    let mut local_2a: u16;
    let mut local_24: u32;
    let mut local_20: u32;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_c: u16;
    let mut local_6: u32;

    local_6 = 0;
    pass1_1028_dc52(
        CONCAT22(unaff_ss, &local_18),
        (&ctx.PTR_LOOP_1050_0000 + 1),
        0,
        0x400,
    );
    u_var6 = (param_1 >> 0x10);
    local_bx_32 = param_1;
    local_bx_32.field_0x10 = local_c;
    error_check_1000_17ce(&local_bx_32.field_0xc);
    pu_var3 = (local_bx_32.field_0x10 << 2);
    process_struct_1000_179c(pu_var3, in_dx);
    local_bx_32.field_0xc = pu_var3;
    &local_bx_32.field_0xe = in_dx;
    local_bx_32.field_0x10 = 0;
    loop {
        pu_var3 = &local_18;
        pass1_1028_e4ec(CONCAT22(unaff_ss, pu_var3));
        local_6 = CONCAT22(ctx.dx_reg, pu_var3);
        if ((ctx.dx_reg | pu_var3) == 0) {
            break;
        }
        if ((pu_var3 + 0x100) != 0x8000002) {
            u_var1 = pu_var3[3];
            u_var2 = &local_bx_32.field_0xc;
            u_var7 = (u_var2 >> 0x10);
            i_var5 = u_var2;
            i_var4 = local_bx_32.field_0x10 * 4;
            _local_2c = (param_1 & 0xffff0000 | ZEXT24(&local_bx_32.field_0x10));
            (i_var4 + i_var5) = pu_var3[2];
            (i_var4 + i_var5 + 2) = u_var1;
            *_local_2c = *_local_2c + 1;
        }
    }
    unsafe {
        *param_2 = local_bx_32.field_0x10;
    }
    return;
}

pub unsafe fn pass1_1010_52fc(param_1: u32, param_2: u32) {
    let mut u_var1: u16;
    let mut u_var2: u32;

    u_var2 = pass1_1010_533c(param_1, param_2);
    u_var1 = (param_1 >> 0x10);
    (param_1 + 0x12) = u_var2;
    (param_1 + 0x14) = (u_var2 >> 0x10);
    return;
}

pub unsafe fn pass1_1010_531c(param_1: u32, param_2: u32) {
    let mut u_var1: u16;
    let mut u_var2: u32;

    u_var2 = pass1_1010_533c(param_1, param_2);
    u_var1 = (param_1 >> 0x10);
    (param_1 + 0x16) = u_var2;
    (param_1 + 0x18) = (u_var2 >> 0x10);
    return;
}

pub unsafe fn pass1_1010_533c(param_1: u32, param_2: u32) {
    let pu_var1: *mut u32;
    let mut u_var2: u32;
    let mut i_var3: i32;
    let mut u_var4: u32;
    
    let mut u_var5: u16;
    let mut u_var6: u16;
    let mut unaff_ss: u16;
    let mut local_16: u32;
    let mut local_12: u32;
    let mut local_e: u32;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: [u8; 2];

    pass1_1010_519a(param_1, CONCAT22(unaff_ss, local_4));
    local_6 = 0;
    loop {
        u_var6 = (param_1 >> 0x10);
        u_var5 = param_1;
        pu_var1 = (u_var5 + 0x10);
        let pu_var1_val = unsafe { *pu_var1 };
        if (pu_var1_val < local_6 || pu_var1_val == local_6) {
            return;
        }
        u_var2 = (u_var5 + 0xc);
        u_var4 = (u_var2 + local_6 * 4);
        str_fn_1010_5286(u_var5, u_var6, u_var4);
        i_var3 = u_var4;
        pass1_1000_3d7a(param_2, i_var3, ctx.dx_reg);
        if (i_var3 == 0) {
            break;
        }
        error_check_1000_17ce((u_var4 & 0xffff | ctx.dx_reg << 0x10));
        local_6 = local_6 + 1;
    }
    return;
}

pub unsafe fn pass1_1010_53ce(param_1: u32, param_2: u8) {
    pass1_1010_50f2(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1010_5f1e(param_1: *mut Struct426, param_2: u32) {
    let u_var1: u8;
    let extraout_var: u32;
    let mut in_dx: u16;
    let local_bx_4: *mut Struct426;
    let mut u_var3: u16;
    let mut u_var2: u16;

    u_var3 = (param_1 >> 0x10);
    local_bx_4 = param_1;
    u_var1 = error_check_1000_17ce(&local_bx_4.field_0x16);
    u_var2 = CONCAT31(extraout_var, u_var1);
    pass1_fn_1008_60e8(param_2);
    &local_bx_4.field_0x16 = u_var2;
    local_bx_4.field_0x18 = in_dx;
    return;
}

pub unsafe fn pass1_1010_5f4c(param_1: *mut Struct427, param_2: u32) {
    let u_var1: u8;
    let extraout_var: u32;
    let mut in_dx: u16;
    let local_bx_4: *mut Struct427;
    let mut u_var3: u16;
    let mut u_var2: u16;

    u_var3 = (param_1 >> 0x10);
    local_bx_4 = param_1;
    u_var1 = error_check_1000_17ce(&local_bx_4.field_0x12);
    u_var2 = CONCAT31(extraout_var, u_var1);
    pass1_fn_1008_60e8(param_2);
    &local_bx_4.field_0x12 = u_var2;
    local_bx_4.field_0x14 = in_dx;
    return;
}

pub unsafe fn pass1_1010_5f7a(param_1: i32, param_2: u16, param_3: u16, param_3_00: i32) {
    let local_bx_12: *mut Struct428;

    local_bx_12 = (param_3_00 * 8 + param_1);
    if ((local_bx_12.field_0x26 == 0) && (local_bx_12.field_0x28 == 0)) {
        return 0;
    }
    return CONCAT22(param_2, param_3_00 * 8 + param_1 + 0x22);
}

pub unsafe fn pass1_1010_5fb0(
    param_1: u32,
    param_2: u16,
    param_2_00: *mut u32,
    param_4: u16,
    param_5: i32,
) {
    let mut u_var1: u16;
    let temp_27f4921068e: *mut Struct429;

    u_var1 = (param_1 >> 0x10);
    temp_27f4921068e = (param_1 + param_5 * 8);
    unsafe {
        temp_27f4921068e.field_0x22 = *param_2_00;
    }
    temp_27f4921068e.field_0x26 = param_2_00[1];
    return;
}

pub unsafe fn pass1_1010_5fd8(param_1: *mut Struct403, param_2: u32) {
    let u_var1: u8;
    let extraout_var: u32;
    let mut in_dx: u16;
    let local_bx_4: *mut Struct430;
    let mut u_var3: u16;
    let mut u_var2: u16;

    u_var3 = (param_1 >> 0x10);
    local_bx_4 = param_1;
    u_var1 = error_check_1000_17ce(&local_bx_4.field_0x68);
    u_var2 = CONCAT31(extraout_var, u_var1);
    pass1_fn_1008_60e8(param_2);
    &local_bx_4.field_0x68 = u_var2;
    local_bx_4.field_0x6a = in_dx;
    return;
}

pub unsafe fn pass1_1010_6006(param_1: *mut Struct431, param_2: u32) {
    let u_var1: u8;
    let extraout_var: u32;
    let mut in_dx: u16;
    let local_bx_4: *mut Struct431;
    let mut u_var3: u16;
    let mut u_var2: u16;

    u_var3 = (param_1 >> 0x10);
    local_bx_4 = param_1;
    u_var1 = error_check_1000_17ce(&local_bx_4.field_0x6c);
    u_var2 = CONCAT31(extraout_var, u_var1);
    pass1_fn_1008_60e8(param_2);
    &local_bx_4.field_0x6c = u_var2;
    local_bx_4.field_0x6e = in_dx;
    return;
}

pub unsafe fn pass1_1010_60a0(param_1: u32) {
    (param_1 + 0x76) = 5;
    return;
}

pub unsafe fn return_1_1010_60b4() {
    return 1;
}

pub unsafe fn return_1_1010_60ba() {
    return 1;
}

pub unsafe fn return_1_1010_60c0() {
    return 1;
}

pub unsafe fn return_1_1010_60c6() {
    return 1;
}

pub unsafe fn pass1_1010_60cc(param_1: *mut Struct433, param_2: u32) {
    let u_var1: u8;
    let extraout_var: u32;
    let mut in_dx: u16;
    let local_bx_4: *mut Struct433;
    let mut u_var3: u16;
    let mut u_var2: u16;

    u_var3 = (param_1 >> 0x10);
    local_bx_4 = param_1;
    u_var1 = error_check_1000_17ce(&local_bx_4.field_0x1a);
    u_var2 = CONCAT31(extraout_var, u_var1);
    pass1_fn_1008_60e8(param_2);
    &local_bx_4.field_0x1a = u_var2;
    local_bx_4.field_0x1c = in_dx;
    return;
}

pub unsafe fn pass1_1010_62a4(param_1: *mut u16, param_2: u8) {
    let mut iVar1: i32;
    let mut u_var2: u16;

    u_var2 = (param_1 >> 0x10);
    iVar1 = param_1;
    unsafe {
        *param_1 = 0x6322;
    }
    (iVar1 + 2) = 0x1010;
    error_check_1000_17ce((iVar1 + 4));
    unsafe {
        *param_1 = ctx.s_1_1050_389a;
    }
    (iVar1 + 2) = &ctx.PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return;
}

pub unsafe fn pass1_1010_6566(param_1: u32, param_2: u16, param_3: u16, param_4: u16) {
    let mut unaff_ss: u16;
    let mut u_var1: u16;
    let mut u_var2: u16;
    let mut local_4: u16;

    u_var1 = param_1;
    u_var2 = (param_1 >> 0x10);
    process_switch_stmt_1010_6646(u_var1, u_var2, CONCAT22(unaff_ss, &local_4), param_4);
    if (local_4 != 0) {
        (u_var1 + local_4) = param_3;
        (u_var1 + local_4 + 2) = param_2;
    }
    return;
}

pub unsafe fn pass1_1010_659a(param_1: u32, param_2: u16) -> i32 {
    let mut unaff_ss: u16;
    let mut u_var1: u16;
    let mut u_var2: u16;
    let mut local_4: u16;

    u_var1 = param_1;
    u_var2 = (param_1 >> 0x10);
    process_switch_stmt_1010_6646(u_var1, u_var2, CONCAT22(unaff_ss, &local_4), param_2);
    if (local_4 == 0) {
        return 0;
    }
    return (u_var1 + local_4) - (u_var1 + local_4 + 2);
}

pub unsafe fn pass1_1010_65d0(param_1: u32, param_2: u16) {
    
    let mut u_var1: u16;
    let mut local_4: u16;

    u_var1 = (param_1 >> 0x10);
    process_switch_stmt_1010_6646(param_1, u_var1, CONCAT22(ctx.stack_seg_reg, &local_4), param_2);
    if (local_4 == 0) {
        return 0;
    }
    return (param_1 + local_4 + 2);
}

pub unsafe fn pass1_1010_6604(param_1: u32, param_2: u16) {
    let mut iVar1: i32;
    let mut unaff_ss: u16;
    let mut u_var2: u16;
    let mut u_var3: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    u_var2 = param_1;
    u_var3 = (param_1 >> 0x10);
    process_switch_stmt_1010_6646(u_var2, u_var3, CONCAT22(unaff_ss, &local_4), param_2);
    if (local_4 != 0) {
        iVar1 = (u_var2 + local_4 + 2);
        (u_var2 + local_4) = (u_var2 + local_4);
        (u_var2 + local_4 + 2) = iVar1 + 1;
        pass1_1010_1f62((param_1 & 0xffff | u_var3 << 0x10), 0x15);
    }
    return;
}

pub unsafe fn pass1_1010_66ca(param_1: *mut Struct376, param_2: u8) -> *mut Struct376 {
    pass1_1010_1d80(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1010_6814(param_1: *mut pass1_struct_1, param_2: u16, param_3: u16) {
    (param_1 + param_3 * 2 + 0x11e) = param_2;
    return;
}

pub unsafe fn pass1_1010_682e(param_1: *mut pass1_struct_1, param_2: u16, param_3: u16) {
    (param_1 + param_3 * 2 + 10) = param_2;
    return param_2;
}

pub unsafe fn pass1_1010_6a86(param_1: *mut Struct376, param_2: u8) -> *mut Struct376 {
    pass1_1010_1d80(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1010_6bb2(param_1: *mut Struct376) {
    let pu_var1: *mut u32;
    let ppc_var2: fn();
    let pHVar3: *mut HDC16;
    let mut u_var4: i32;
    let local_bx_5: *mut Struct376;
    let mut u_var5: i32;
    let mut local_e: u16;
    let mut local_c: u16;

    u_var5 = (param_1 >> 0x10);
    local_bx_5 = param_1;
    param_1.ptr_a_lo = 0x7e28;
    local_bx_5.ptr_a_hi = 0x1010;
    local_bx_5.dc_handle_x0a = 0x7e38;
    local_bx_5.u16_x0c = 0x1010;
    pu_var1 = &local_bx_5.field_0x1c;
    u_var4 = (&local_bx_5.field_0x1c + 2);
    if ((u_var4 | pu_var1) != 0) {
        unsafe {
            ppc_var2 = *pu_var1;
        }
        ppc_var2();
    }
    local_bx_5.field_0x1c = 0x0;
    if (&local_bx_5.u8_ptr_x14 != 0) {
        if ((u_var5 | local_bx_5) == 0) {
            pHVar3 = 0x0;
            u_var4 = 0;
        } else {
            pHVar3 = &local_bx_5.dc_handle_x0a;
            u_var4 = u_var5;
        }
        // WARNING: Load size is inaccurate
        pass1_1010_1ea6(local_bx_5.u8_ptr_x14, CONCAT22(u_var4, pHVar3));
    }
    if (&local_bx_5.ptr_2_hi != 0) {
        if ((u_var5 | local_bx_5) == 0) {
            pHVar3 = 0x0;
            u_var4 = 0;
        } else {
            pHVar3 = &local_bx_5.dc_handle_x0a;
            u_var4 = u_var5;
        }
        pass1_1010_1ea6(*&local_bx_5.ptr_2_hi, CONCAT22(u_var4, pHVar3));
    }
    &local_bx_5.u8_ptr_x14 = 0;
    &local_bx_5.ptr_2_hi = 0;
    if (param_1 == 0x0) {
        pHVar3 = 0x0;
        u_var5 = 0;
    } else {
        pHVar3 = &local_bx_5.dc_handle_x0a;
    }
    _local_e = CONCAT22(u_var5, pHVar3);
    *_local_e = ctx.s_1_1050_389a;
    pHVar3[1] = (HDC16) & ctx.PTR_LOOP_1050_1008;
    pass1_1010_1d80(param_1);
    return;
}

// WARNING: Variable defined which should be unmapped: local_6
// WARNING: Could not reconcile some variable overlaps

pub unsafe fn pass1_1010_6ca2(param_1: u32, param_2: u16) {
    let mut u_var1: u32;
    let p_uvar2: *mut u16;
    let mut i_var3: i32;
    let mut unaff_ss: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    local_4 = 1;
    _local_8 = CONCAT22(unaff_ss, &stack0x000a);
    local_a = param_2;
    while {
        pu_var2 = _local_8;
        if (local_a == 0) {
            return local_4;
        }
        _local_8 = (_local_8 & 0xffff0000 | (local_8 + 2));
        u_var1 = (param_1 + 0x14);
        local_a = local_a - 1;
        i_var3 = pass1_1010_a5ca(u_var1, (u_var1 >> 0x10), *pu_var2);
        i_var3 == 0
    } {}
    return 0;
}

pub unsafe fn pass1_1010_715c(param_1: u32, param_2: u16) {
    pass1_1010_a69c((param_1 + 0x14), param_2);
    return;
}

pub unsafe fn pass1_1010_7818(param_1: u32, param_2: u32) {
    let mut u_var1: u32;
    let mut u_var2: u16;
    let BVar3: bool;
    let mut u_var4: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    u_var4 = (param_1 >> 0x10);
    u_var1 = (param_1 + 0x14);
    u_var2 = pass1_1010_b028(u_var1, (u_var1 >> 0x10), param_2);
    BVar3 = pass1_1008_c6ae(ctx._PTR_LOOP_1050_06e0, u_var2, 0x1e);
    if (BVar3 == 0) {
        BVar3 = pass1_1008_c6ae(ctx._PTR_LOOP_1050_06e0, u_var2, 0xb);
        if (((BVar3 == 0)
            && (
                BVar3 = pass1_1008_c6ae(ctx._PTR_LOOP_1050_06e0, u_var2, 0x20),
                BVar3 == 0,
            ))
            && (
                BVar3 = pass1_1008_c6ae(ctx._PTR_LOOP_1050_06e0, u_var2, 0x1c),
                BVar3 == 0,
            ))
        {
            BVar3 = pass1_1008_c6ae(ctx._PTR_LOOP_1050_06e0, u_var2, 2);
            if ((BVar3 != 0)
                || (
                    BVar3 = pass1_1008_c6ae(ctx._PTR_LOOP_1050_06e0, u_var2, 0x13),
                    BVar3 != 0,
                ))
            {
                return 5;
            }
            BVar3 = pass1_1008_c6ae(ctx._PTR_LOOP_1050_06e0, u_var2, 0x11);
            if ((BVar3 != 0)
                || (
                    BVar3 = pass1_1008_c6ae(ctx._PTR_LOOP_1050_06e0, u_var2, 0x12),
                    BVar3 != 0,
                ))
            {
                return 4;
            }
            BVar3 = pass1_1008_c6ae(ctx._PTR_LOOP_1050_06e0, u_var2, 5);
            if (BVar3 != 0) {
                return 6;
            }
            BVar3 = pass1_1008_c6ae(ctx._PTR_LOOP_1050_06e0, u_var2, 6);
            if (BVar3 != 0) {
                return 7;
            }
            BVar3 = pass1_1008_c6ae(ctx._PTR_LOOP_1050_06e0, u_var2, 4);
            if (BVar3 != 0) {
                return 0x10;
            }
            BVar3 = pass1_1008_c6ae(ctx._PTR_LOOP_1050_06e0, u_var2, 3);
            if (BVar3 != 0) {
                return 0x11;
            }
            BVar3 = pass1_1008_c6ae(ctx._PTR_LOOP_1050_06e0, u_var2, 0x19);
            if (BVar3 != 0) {
                return 0x15;
            }
            BVar3 = pass1_1008_c6ae(ctx._PTR_LOOP_1050_06e0, u_var2, 0x1d);
            if (BVar3 != 0) {
                return 0x16;
            }
            BVar3 = pass1_1010_7d7e(param_1, u_var4, 1, u_var2);
            if (BVar3 == 0) {
                return 0;
            }
            return 0xc;
        }
        local_6 = 1;
    } else {
        local_6 = 0x18;
    }
    return local_6;
}

pub unsafe fn pass1_1010_7b8c(param_1: *mut Struct440, param_2: u16) {
    let pu_var1: *mut u32;
    let mut u_var2: i32;
    let ppc_var3: fn();
    let mut u_var4: u32;
    let local_AX_47: *mut Struct441;
    
    let local_bx_4: *mut Struct440;
    let mut u_var5: u16;
    let mut unaff_ss: u16;
    let mut local_1e: u16;
    let mut local_1c: u16;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_e: u32;
    let mut local_a: [u8; 8];

    u_var5 = (param_1 >> 0x10);
    local_bx_4 = param_1;
    if ((&local_bx_4.field_0x1e | local_bx_4.field_0x1c) != 0) {
        pass1_1008_5784(CONCAT22(unaff_ss, local_a), &local_bx_4.field_0x1c);
        while {
            local_AX_47 = local_a;
            pass1_1008_5b12(CONCAT22(unaff_ss, local_AX_47));
            local_e = CONCAT22(ctx.dx_reg, local_AX_47);
            if ((ctx.dx_reg | local_AX_47) == 0) {
                break;
            }
            u_var4 = local_AX_47.field_0x8;
            (u_var4 + 6) != param_2
        } {}
        if ((ctx.dx_reg | local_AX_47) != 0) {
            ppc_var3 = (&local_bx_4.field_0x1c + 0xc);
            (**ppc_var3)(&ctx.PTR_LOOP_1050_1008, &local_bx_4.field_0x1c, local_e);
        }
        u_var4 = &local_bx_4.field_0x1c;
        if ((u_var4 + 8) == 0) {
            pu_var1 = local_bx_4.field_0x1c;
            u_var2 = &local_bx_4.field_0x1e;
            if ((u_var2 | pu_var1) != 0) {
                unsafe {
                    ppc_var3 = *pu_var1;
                }
                (**ppc_var3)(
                    &ctx.PTR_LOOP_1050_1008,
                    pu_var1,
                    u_var2,
                    1,
                    pu_var1,
                    u_var2,
                    pu_var1,
                    u_var2,
                );
            }
            &local_bx_4.field_0x1c = 0;
        }
    }
    return;
}

pub unsafe fn pass1_1010_7d38(
    param_1: u16,
    param_2: u16,
    param_1_00: *mut Struct442,
    param_4: u16,
) {
    let mut unaff_ss: u16;
    let mut local_10: u16;
    let mut local_e: u32;
    let mut uStack10: u16;
    let mut local_8: u16;
    let mut local_6: [u8; 2];
    let mut local_4: [u8; 2];

    local_e = param_1_00.field_0xc;
    uStack10 = param_1_00.field_0x10;
    pass1_1008_3eb4(
        CONCAT22(unaff_ss, &local_e),
        CONCAT22(unaff_ss, &local_8),
        CONCAT22(unaff_ss, local_6),
        CONCAT22(unaff_ss, local_4),
    );
    return local_8;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn pass1_1010_7d7e(
    param_1: u16,
    param_2: u16,
    param_1_00: i32,
    param_2_00: i32,
) -> bool {
    let b_var1: bool;

    if (param_1_00 != 3) {
        if ((param_2_00 < 10) || (0x7f < param_2_00)) {
            return 0;
        }
        b_var1 = pass1_1008_c6ae(ctx._PTR_LOOP_1050_06e0, param_2_00, 0x3c);
        if (b_var1 != 0) {
            return 0;
        }
        if (((param_2_00 == 0x6a) && (param_1_00 != 4)) && (param_1_00 != 5)) {
            return 0;
        }
    }
    return 1;
}

pub unsafe fn pass1_1010_7dd2(param_1: *mut u16, param_2: u8) {
    unsafe {
        *param_1 = ctx.s_1_1050_389a;
    }
    (param_1 + 2) = &ctx.PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1010_7dfe(param_1: u32, param_2: u8) {
    pass1_1010_6bb2(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn pass1_1010_7e40(param_1: *mut Struct443) {
    let mut u_var1: u32;
    let local_bx_4: *mut Struct443;
    let mut u_var2: u16;
    let ppVar3: *mut pass1_struct_1;
    let mut in_stack_0000fff6: u16;

    u_var2 = (param_1 >> 0x10);
    local_bx_4 = param_1;
    param_1 = 0;
    local_bx_4.field_0x67c = 0;
    local_bx_4.field_0x680 = 0;
    local_bx_4.field_0xe82 = 0;
    local_bx_4.field_0xe84 = 0;
    &local_bx_4.field_0xe88 = 0;
    pass1_1000_4906((param_1 & 0xffff0000 | &local_bx_4.field_0x4), 0, 0x228);
    pass1_1000_4906((param_1 & 0xffff0000 | &local_bx_4.field_0x22c), 0, 0x228);
    pass1_1000_4906((param_1 & 0xffff0000 | &local_bx_4.field_0x454), 0, 0x228);
    *&local_bx_4.field_0x682 = 0;
    *&local_bx_4.field_0xa82 = 0;
    ctx._g_struct_73_1050_14cc = param_1;
    ppVar3 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(in_stack_0000fff6, 2));
    local_bx_4.field_0xe88 = ppVar3;
    &local_bx_4.field_0xe8a = (ppVar3 >> 0x10);
    u_var1 = &local_bx_4.field_0xe88;
    local_bx_4.field_0xe84 = (u_var1 + 100);
    return;
}

// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn pass1_1010_7efc(param_1: *mut Struct444) {
    let mut u_var1: i32;
    let mut u_var2: i32;
    let pu_var3: *mut u32;
    let ppcVar4: fn();
    let local_bx_5: *mut Struct444;
    let mut u_var5: u16;
    let mut unaff_cs: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    u_var5 = (param_1 >> 0x10);
    local_bx_5 = param_1;
    u_var1 = local_bx_5.field_0x67c;
    u_var2 = local_bx_5.field_0x67e;
    _local_8 = CONCAT22(u_var2, u_var1);
    if ((u_var2 | u_var1) != 0) {
        call_fn_ptr_1008_64a2(u_var1, u_var2);
        unaff_cs = 0x1000;
        error_check_1000_17ce(_local_8);
    }
    local_4 = 0;
    while (local_4 < 0x8a) {
        pu_var3 = (&local_bx_5.field_0x4 + local_4 * 4);
        u_var1 = (&local_bx_5.field_0x4 + local_4 * 4 + 2);
        if ((u_var1 | pu_var3) != 0) {
            unsafe {
                ppcVar4 = *pu_var3;
            }
            (**ppcVar4)(unaff_cs, pu_var3, u_var1, 1);
        }
        pu_var3 = (&local_bx_5.field_0x22c + local_4 * 4);
        u_var1 = (&local_bx_5.field_0x22c + local_4 * 4 + 2);
        if ((u_var1 | pu_var3) != 0) {
            unsafe {
                ppcVar4 = *pu_var3;
            }
            (**ppcVar4)(unaff_cs, pu_var3);
        }
        pu_var3 = (&local_bx_5.field_0x454 + local_4 * 4);
        u_var1 = (&local_bx_5.field_0x454 + local_4 * 4 + 2);
        if ((u_var1 | pu_var3) != 0) {
            unsafe {
                ppcVar4 = *pu_var3;
            }
            (**ppcVar4)(unaff_cs, pu_var3);
        }
        local_4 = local_4 + 1;
    }
    error_check_1000_17ce(param_1);
    ctx._g_struct_73_1050_14cc = 0;
    return;
}

// WARNING: Could not reconcile some variable overlaps

pub unsafe fn pass1_1010_7fd6(param_1: *mut Struct445) {
    let mut u_var1: i32;
    let mut u_var2: i32;
    let local_bx_4: *mut Struct445;
    let mut u_var3: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    u_var3 = (param_1 >> 0x10);
    local_bx_4 = param_1;
    u_var1 = local_bx_4.field_0x67c;
    u_var2 = &local_bx_4.field_0x67e;
    local_6 = CONCAT22(u_var2, u_var1);
    if ((u_var2 | u_var1) != 0) {
        call_fn_ptr_1008_64a2(u_var1, u_var2);
        error_check_1000_17ce(local_6);
    }
    &local_bx_4.field_0x67c = 0;
    local_bx_4.field_0x680 = 0;
    return;
}

// WARNING: Variable defined which should be unmapped: local_c

pub unsafe fn pass1_1010_81f6(param_1: u32, param_2: libc::c_long, param_3: u16) {
    let local_SI_107: *mut Struct448;
    let mut u_var1: u16;
    let mut local_c: u16;
    let mut local_a: u32;
    let mut local_6: u32;

    if (param_2 == 0x8000001) {
        local_a = param_1 & 0xffff0000 | &(param_1).field_0x22c;
        local_c._0_1_ = 0xfa;
    } else {
        if (param_2 == 0x8000002) {
            local_a = param_1 & 0xffff0000 | &(param_1).field_0x454;
            local_c._0_1_ = 0xfc;
        } else {
            local_a = param_1 & 0xffff0000 | &(param_1).field_0x4;
            local_c._0_1_ = 2;
        }
    }
    u_var1 = (local_a >> 0x10);
    local_SI_107 = local_a;
    if ((&local_SI_107.field_0x0 + param_3 * 4) == 0) {
        if ((0 < param_3) && (param_3 < 10)) {
            pass1_1010_89f0(param_1, param_1._2_2_, local_c, local_a);
            return;
        }
        if (param_3 < 10) {
            return;
        }
        if (0x7f < param_3) {
            return;
        }
        if (local_SI_107.field_0x14 == 0) {
            pass1_1010_89f0(param_1, param_1._2_2_, local_c, local_a);
        }
        pass1_1010_887a(param_1, local_a, param_3);
    }
    pass1_1010_866c(param_1, param_1._2_2_, local_a, param_3);
    return;
}

pub unsafe fn pass1_1010_82f8(param_1: u32, param_2: u16) {
    (param_1 + 0xe82) = param_2;
    return;
}

pub unsafe fn pass1_1010_84f8(param_1: u32, param_2: u16) {
    let mut u_var1: u32;
    let mut unaff_ss: u16;
    let mut local_310: u16;
    let mut local_30e: u16;
    let mut local_30c: u16;
    let mut local_30a: u16;
    let mut local_308: [u8; 256];
    let mut local_208: [u8; 256];
    let mut local_108: [u8; 260];
    let mut local_4: u16;

    if ((param_2 * 0x10 + 0x10) != 3) {
        return;
    }
    u_var1 = (param_1 + 0xe88);
    local_4 = (u_var1 + 0x70);
    process_string_1000_4d58((param_2 * 0x10 + 0x12), 0, 0);
    copy_string_1000_3d3e(CONCAT22(unaff_ss, local_108), CONCAT22(unaff_ss, local_208));
    if (local_308[0] == '\0') {
        if (local_4 == 0) {
            local_30c = s__mid_1050_14c0;
        } else {
            local_30c = 0x14ba;
        }
        _local_30c = CONCAT22(0x1050, local_30c);
    } else {
        _local_30c = CONCAT22(unaff_ss, local_308);
    }
    process_string_1000_3cea(CONCAT22(unaff_ss, local_108), _local_30c);
    set_error_mode_1010_8b14(param_1, local_108, unaff_ss);
    return;
}

pub unsafe fn pass1_1010_866c(param_1: u16, param_2: u16, param_1_00: u32, uparam_2_00: i32) {
    let mut cVar1: u8;
    let mut i_var2: i32;
    let mut u8_var3: bool;

    if (param_2_00 < 0x28) {
        if ((param_2_00 < 0x25) && (param_2_00 != 0x23)) {
            if (0x23 < param_2_00) {
                return;
            }
            cVar1 = param_2_00;
            if (((cVar1 != 0xb) && (cVar1 != 0xf)) && (cVar1 != '!')) {
                return;
            }
        }
    } else {
        if (param_2_00 != 0x37) {
            if (param_2_00 < 0x38) {
                if (param_2_00 < 0x33) {
                    return;
                }
                u8_var3 = SBORROW2(param_2_00 - 0x33, 1);
                i_var2 = param_2_00 - 0x34;
            } else {
                if (param_2_00 == 0x49) {}
                // goto LAB_1010_8691;
                if ((param_2_00 - 0x49) < 0x2a) {
                    return;
                }
                u8_var3 = SBORROW2(param_2_00 - 0x73, 5);
                i_var2 = param_2_00 - 0x78;
            }
            if ((i_var2 != 0) && u8_var3 == (i_var2 < 0)) {
                return;
            }
        }
    }
    // LAB_1010_8691:
    copy_mem_1008_676e((param_2_00 * 4 + param_1_00));
    return;
}

// WARNING: Could not reconcile some variable overlaps

pub unsafe fn pass1_1010_86de(
    param_1: *mut u8,
    param_2: *mut u8,
    param_1_00: u8,
    param_2_00: *mut u8,
) {
    let pu_var1: *mut u32;
    let mut i_var2: i32;
    let mut u8_var3: bool;
    let mut u_var4: u16;
    let mut u_var5: u32;
    
    let mut u_var6: u32;
    let mut local_14: u32;
    let mut local_10: u16;
    let mut local_e: u32;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    u_var6 = process_struct_1008_4772(param_2_00);
    u_var4 = (u_var6 >> 0x10);
    local_a = 0;
    loop {
        pu_var1 = (u_var6 + 8);
        let pu_var1_val = unsafe { *pu_var1 };
        if (pu_var1_val == local_a || pu_var1_val < local_a) {
            return;
        }
        u_var5 = local_a;
        process_struct_1008_4544(param_2_00);
        i_var2 = u_var5;
        u8_var3 = false;
        local_14 = 0;
        unsafe {
            while (
                pu_var1 = (u_var6 + 4),
                *pu_var1 != local_14 && local_14 <= *pu_var1,
            ) {
                if (u8_var3) {
                    // LAB_1010_86fc:
                    if (u8_var3) {
                        if (*(local_14 + i_var2) == -1) {
                            *(local_14 + i_var2) = param_1_00;
                            break;
                        }
                    }
                } else {
                    if (*(local_14 + i_var2) == -1) {}
                    // goto LAB_1010_86fc;
                    unsafe {
                        *(local_14 + i_var2 + -1) = param_1_00;
                    }
                    u8_var3 = true;
                }
                local_14 = local_14 + 1;
            }
        }
        local_a = local_a + 1;
    }
}

// WARNING: Could not reconcile some variable overlaps

pub unsafe fn pass1_1010_878c(param_1: *mut *mut Struct117, param_2: i32) {
    let mut u_var1: i32;
    let u_var2: u8;
    let extraout_AH: u8;
    let struct_a: *mut Struct199;
    
    let mut i_var4: i32;
    let mut i_var5: i32;
    let mut u_var6: u16;
    let pa_var7: *mut Struct117;
    let in_Struct117: *mut Struct117;
    let mut u_var8: u32;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;
    let mut u_var3: i32;

    u_var6 = (param_1 >> 0x10);
    i_var4 = param_1;
    if ((i_var4 + 0x680) == param_2) {
        return;
    }
    u_var3 = (i_var4 + 0x67c);
    u_var1 = (i_var4 + 0x67e);
    local_6 = CONCAT22(u_var1, u_var3);
    struct_a = (u_var1 | u_var3);
    if (struct_a != 0x0) {
        call_fn_ptr_1008_64a2(u_var3, u_var1);
        struct_a = ctx.dx_reg;
        u_var2 = error_check_1000_17ce(local_6);
        u_var3 = CONCAT11(extraout_AH, u_var2);
    }
    if ((param_2 == 1) || (param_2 == 2)) {
        process_struct_1000_179c(8, struct_a);
        in_Struct117 = CONCAT22(struct_a, u_var3);
        if ((struct_a | u_var3) == 0) {
            (i_var4 + 0x67c) = 0;
            // goto LAB_1010_8869;
        }
        unsafe {
            pa_var7 = *param_1;
        }
        // LAB_1010_8853:
        u_var8 = file_fn_1008_6414(in_Struct117, pa_var7);
    } else {
        i_var5 = param_2 * 4;
        pa_var7 = set_error_mode_1010_8b14(param_1, (i_var5 + 0x172a));
        in_Struct117 = pa_var7;
        if (((i_var5 + 0x172a) == pa_var7) && ((i_var5 + 0x172c) == (pa_var7 >> 0x10))) {
            in_Struct117 = msg_box_1010_8bb4(param_1, pa_var7);
        }
        process_struct_1000_179c(8, (in_Struct117 >> 0x10));
        if (in_Struct117 != 0x0) {}
        // goto LAB_1010_8853;
        u_var8 = 0;
    }
    (i_var4 + 0x67c) = u_var8;
    (i_var4 + 0x67e) = (u_var8 >> 0x10);
    // LAB_1010_8869:
    (i_var4 + 0x680) = param_2;
    return;
}

// WARNING: Could not reconcile some variable overlaps

pub unsafe fn pass1_1010_887a(param_1: u32, param_2: u32, param_3: u16) {
    let mut u_var1: u16;
    let mut u_var2: i32;
    let mut u_var3: u32;
    let mut u_var4: u32;
    let ctx.dx_reg: *mut u16;
    let pu_var5: *mut u16;
    let struct_a: *mut Struct199;
    
    let mut i32_var6: i32;
    let mut u_var7: u16;
    let mut u_var8: u16;
    let mut unaff_ss: u16;
    let u_var9: u8;
    let mut local_2a: u16;
    let mut local_28: u16;
    let mut local_26: [u8; 6];
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
    let mut local_c: u32;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;
    let temp_7ff107b7cdf: *mut Struct450;

    local_4 = param_3 - 10;
    u_var7 = (param_1 >> 0x10);
    pass1_1010_878c(param_1, (local_4 * 10 + 0x3382));
    if ((param_1 + 0x67c) != 0) {
        i32_var6 = local_4 * 10;
        pu_var5 = ctx.dx_reg;
        u_var1 = local_4;
        pass1_1008_6562(
            (param_1 + 0x67c),
            CONCAT22((i32_var6 + 0x3388), (i32_var6 + 0x338a)),
            (i32_var6 + 0x3386),
            local_4,
            ctx.dx_reg,
        );
        _local_8 = CONCAT22(pu_var5, u_var1);
        u_var7 = (param_2 >> 0x10);
        local_c = *(param_2 + 0x14);
        _local_10 = process_struct_1008_4772(local_c);
        _local_14 = process_struct_1008_4772(_local_8);
        struct_a = (_local_14 >> 0x10);
        u_var3 = (_local_14 + 4);
        u_var8 = (_local_10 >> 0x10);
        i32_var6 = _local_10;
        if (u_var3 < (i32_var6 + 4)) {
            u_var3 = *(i32_var6 + 4);
        }
        u_var4 = (_local_14 + 8);
        if (u_var4 < (i32_var6 + 8)) {
            u_var4 = *(i32_var6 + 8);
        }
        u_var2 = u_var4;
        _local_18 = u_var4 & 0xffff | u_var3 << 0x10;
        u_var9 = -1;
        process_struct_1000_179c(0x1e, struct_a);
        if ((struct_a | u_var2) == 0) {
            u_var2 = 0;
            u_var8 = 0;
        } else {
            pass1_1008_6604(CONCAT22(struct_a, u_var2), _local_18, (_local_18 >> 0x10));
            u_var8 = ctx.dx_reg;
        }
        _local_1c = CONCAT22(u_var8, u_var2);
        pass1_1008_431c(CONCAT22(u_var8, u_var2), u_var9);
        u_var8 = (_local_10 >> 0x10);
        local_1e = (local_16 - (_local_10 + 4)) / 2;
        local_20 = local_18 - (_local_10 + 8);
        pass1_1008_3e54(CONCAT22(unaff_ss, local_26), 0, local_20, local_1e);
        pass1_1008_4480(_local_1c, CONCAT22(unaff_ss, local_26), local_c);
        pass1_1008_3e76(CONCAT22(unaff_ss, local_26), 0, 0, 7);
        pass1_1008_4480(_local_1c, CONCAT22(unaff_ss, local_26), _local_8);
        (param_3 * 4 + param_2) = _local_1c;
    }
    return;
}

// WARNING: Could not reconcile some variable overlaps

pub unsafe fn pass1_1010_89f0(
    param_1: *mut Struct451,
    param_2: *mut u8,
    param_3: u8,
    param_2_00: u32,
) {
    let mut u_var1: i32;
    let u_var2: u8;
    let pcVar3: *mut libc::c_char;
    let extraout_var: u32;
    
    let ctx.dx_reg: *mut Struct199;
    let struct_a: *mut Struct199;
    let ctx.dx_reg: *mut u8;
    let local_DX_148: *mut u8;
    
    let local_SI_89: *mut Struct452;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let local_8: *mut libc::c_char;
    let mut local_6: u16;
    let mut local_4: u16;
    let temp_41ff73ce2e1: *mut libc::c_char;

    pcVar3 = param_1.field_0x67c;
    u_var1 = &param_1.field_0x67e;
    _local_c = CONCAT22(u_var1, pcVar3);
    if ((u_var1 | pcVar3) != 0) {
        call_fn_ptr_1008_64a2(pcVar3, u_var1);
        u_var2 = error_check_1000_17ce(_local_c);
        pcVar3 = CONCAT31(extraout_var, u_var2);
    }
    set_error_mode_1010_8b14(param_1, param_2, (param_1.field_0xe82 * 4 + 0x24be));
    local_6 = CONCAT22(ctx.dx_reg, pcVar3);
    local_SI_89 = (param_1.field_0xe82 * 4);
    struct_a = ctx.dx_reg;
    if ((*(local_SI_89 + 0x24be) == pcVar3) && ((local_SI_89 + 0x24c0) == ctx.dx_reg)) {
        msg_box_1010_8bb4(param_1, param_2, pcVar3, ctx.dx_reg);
        struct_a = ctx.dx_reg;
    }
    process_struct_1000_179c(8, struct_a);
    if ((struct_a | pcVar3) == 0) {
        pcVar3 = 0x0;
        local_DX_148 = 0x0;
    } else {
        file_fn_1008_6414(CONCAT22(struct_a, pcVar3), local_6);
        local_DX_148 = ctx.dx_reg;
    }
    param_1.field_0x67c = pcVar3;
    *&param_1.field_0x67e = local_DX_148;
    param_1.field_0x680 = 0;
    if ((&param_1.field_0x67e | param_1.field_0x67c) != 0) {
        local_8 = (&ctx.PTR_LOOP_1050_0000 + 1);
        while (local_8 < 10) {
            temp_41ff73ce2e1 = (local_8 * 10);
            // WARNING: Load size is inaccurate
            pcVar3 = local_8;
            string_fn_1008_64c8(
                param_1.field_0x67c,
                CONCAT22((temp_41ff73ce2e1 + 0x2558), (temp_41ff73ce2e1 + 0x255a)),
                *(temp_41ff73ce2e1 + 0x2556),
                *(temp_41ff73ce2e1 + 0x2554),
            );
            _local_16 = CONCAT22(ctx.dx_reg, pcVar3);
            pass1_1010_86de(param_1, param_2, param_3, CONCAT22(ctx.dx_reg, pcVar3));
            (local_8 * 4 + param_2_00) = _local_16;
            local_8 = local_8 + 1;
        }
    }
    return;
}

pub unsafe fn pass1_1010_8c32(param_1: *mut Struct453, param_2: u32) {
    let mut in_eax: u32;
    let mut u_var1: u16;
    let mut unaff_bp: u16;
    let ppVar2: *mut pass1_struct_1;
    let mut u_var3: u16;

    u_var1 = (in_eax >> 0x10);
    u_var3 = param_2;
    process_struct_1010_1d48(CONCAT22(u_var3, param_1), (param_2 >> 0x10));
    &param_1.field_0xa = 0;
    CONCAT22(u_var3, param_1) = 0x8ee2;
    param_1.field_0x2 = 0x1010;
    ppVar2 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(unaff_bp, 3));
    param_1.field_0xa = ppVar2;
    param_1.field_0xc = (ppVar2 >> 0x10);
    return CONCAT22(u_var1, param_1);
}

pub unsafe fn pass1_1010_8c78(param_1: *mut Struct376) {
    param_1.ptr_a_lo = 0x8ee2;
    (param_1 + 2) = 0x1010;
    pass1_1010_1d80(param_1);
    return;
}

pub unsafe fn pass1_1010_8ebc(param_1: u32, param_2: u8) {
    pass1_1010_8c78(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn pass1_1010_8ef2(param_1: *mut u16) {
    let mut u_var1: i32;
    let in_dx: *mut Struct199;
    
    let local_bx_4: *mut Struct454;
    let mut u_var2: u16;
    let ppVar3: *mut pass1_struct_1;
    let mut in_stack_0000fff6: u16;
    let mut local_8: u16;

    u_var2 = (param_1 >> 0x10);
    local_bx_4 = param_1;
    unsafe {
        *param_1 = ctx.s_1_1050_389a;
    }
    local_bx_4.field_0x2 = &ctx.PTR_LOOP_1050_1008;
    u_var1 = 0;
    &local_bx_4.field_0x4 = 0;
    &local_bx_4.field_0x8 = 0;
    unsafe {
        *param_1 = 0x9254;
    }
    local_bx_4.field_0x2 = 0x1010;
    process_struct_1000_179c(0x18, in_dx);
    if ((in_dx | u_var1) == 0) {
        &local_bx_4.field_0x4 = 0;
    } else {
        pass1_1030_1cd8(CONCAT22(in_dx, u_var1), 5, 5);
        local_bx_4.field_0x4 = u_var1;
        local_bx_4.field_0x6 = ctx.dx_reg;
    }
    ppVar3 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(in_stack_0000fff6, 3));
    local_bx_4.field_0x8 = ppVar3;
    local_bx_4.field_0xa = (ppVar3 >> 0x10);
    return;
}

pub unsafe fn pass1_1010_8f78(param_1: *mut Struct455) {
    let pu_var1: *mut u32;
    let mut u_var2: i32;
    let ppc_var3: fn();
    let local_bx_5: *mut Struct455;
    let mut u_var4: u16;

    u_var4 = (param_1 >> 0x10);
    local_bx_5 = param_1;
    param_1 = 0x9254;
    local_bx_5.field_0x2 = 0x1010;
    pu_var1 = local_bx_5.field_0x4;
    u_var2 = local_bx_5.field_0x6;
    if ((u_var2 | pu_var1) != 0) {
        unsafe {
            ppc_var3 = *pu_var1;
            (**ppc_var3)();
        }
    }
    param_1 = ctx.s_1_1050_389a;
    local_bx_5.field_0x2 = &ctx.PTR_LOOP_1050_1008;
    return;
}

// WARNING: Could not reconcile some variable overlaps

pub unsafe fn pass1_1010_8fba(param_1: *mut Struct456) {
    let pp_var1: fn();
    
    let mut u_var2: u32;
    
    
    let local_bx_12: *mut Struct456;
    let mut u_var3: u16;
    let mut local_10: u16;
    let mut local_e: u32;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;

    u_var3 = (param_1 >> 0x10);
    local_bx_12 = param_1;
    pp_var1 = (local_bx_12.field_0x4 + 0x10);
    (**pp_var1)();
    _local_a = CONCAT22(ctx.dx_reg, in_ax);
    local_e = 0;
    loop {
        if (_local_a <= local_e) {
            return;
        }
        pp_var1 = (local_bx_12.field_0x4 + 4);
        u_var2 = _local_a;
        (**pp_var1)();
        if ((ctx.dx_reg | u_var2) != 0) {
            break;
        }
        local_e = local_e + 1;
    }
    pp_var1 = (local_bx_12.field_0x4 + 8);
    (**pp_var1)();
    return;
}

pub unsafe fn pass1_1010_9044(param_1: u32) {
    let pp_var1: fn();

    pp_var1 = ((param_1 + 4) + 0x10);
    (**pp_var1)();
    return;
}

pub unsafe fn pass1_1010_905e(param_1: *mut Struct457, param_2: u32) {
    let pu_var1: *mut u32;
    let mut u_var2: i32;
    let ppc_var3: fn();
    let local_bx_4: *mut Struct457;
    let mut u_var4: u16;

    u_var4 = (param_1 >> 0x10);
    local_bx_4 = param_1;
    pu_var1 = local_bx_4.field_0x4;
    u_var2 = &local_bx_4.field_0x6;
    if ((u_var2 | pu_var1) != 0) {
        unsafe {
            ppc_var3 = *pu_var1;
            (**ppc_var3)();
        }
    }
    &local_bx_4.field_0x4 = param_2;
    return;
}

// WARNING: Could not reconcile some variable overlaps

pub unsafe fn pass1_1010_9092(param_1: *mut Struct458) {
    let pp_var1: fn();
    let mut in_ax: i32;
    let mut u_var2: u32;
    let struct_a: *mut Struct199;
    let paVar3: *mut Struct199;
    
    let mut u_var4: u16;
    
    let local_bx_4: *mut Struct458;
    let mut u_var5: u16;
    let mut u_var6: u32;
    let mut local_14: u16;
    let mut local_10: u16;
    let mut local_e: u32;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    u_var5 = (param_1 >> 0x10);
    local_bx_4 = param_1;
    u_var6 = local_bx_4.field_0x4;
    pp_var1 = (local_bx_4.field_0x4 + 0x10);
    (**pp_var1)();
    local_6 = CONCAT22(struct_a, in_ax);
    paVar3 = struct_a;
    process_struct_1000_179c(0xc, struct_a);
    if ((paVar3 | in_ax) == 0) {
        in_ax = 0;
        u_var4 = 0;
    } else {
        pass1_1010_8ef2(CONCAT22(paVar3, in_ax));
        u_var4 = ctx.dx_reg;
    }
    local_e = 0;
    while (local_e < local_6) {
        pp_var1 = (local_bx_4.field_0x4 + 4);
        u_var2 = local_6;
        (**pp_var1)(0x1000, local_bx_4.field_0x4, local_e, u_var6);
        if ((ctx.dx_reg | u_var2) != 0) {
            pp_var1 = ((in_ax + 4) + 0xc);
            (**pp_var1)(0x1000, (in_ax + 4), u_var2, ctx.dx_reg);
        }
        local_e = local_e + 1;
    }
    return;
}

pub unsafe fn pass1_1010_9130(param_1: u32, param_2: u32, param_3: u16) {
    let mut u_var1: u32;
    let mut u_var2: u16;
    let mut u_var3: u32;
    let mut local_4: u16;

    u_var2 = (param_1 >> 0x10);
    u_var3 = pass1_1030_1d58((param_1 + 4));
    if (u_var3 != 0) {
        u_var1 = (param_1 + 8);
        pass1_1010_c3c2(u_var1, (u_var1 >> 0x10), param_2, u_var3);
        return;
    }
    *param_2 = 0;
    return;
}

pub unsafe fn pass1_1010_9172(param_1: *mut Struct459) {
    let pu_var1: *mut u32;
    let mut u_var2: i32;
    let ppc_var3: fn();
    let local_bx_4: *mut Struct459;
    let mut u_var4: u16;
    Struct706 * *ppa_var5;
    let mut u_var6: u32;
    let mut local_8: u16;

    u_var4 = (param_1 >> 0x10);
    local_bx_4 = param_1;
    pu_var1 = local_bx_4.field_0x4;
    u_var2 = local_bx_4.field_0x6;
    ppa_var5 = CONCAT22(u_var2, pu_var1);
    if ((u_var2 | pu_var1) != 0) {
        unsafe {
            ppc_var3 = *pu_var1;
            ppa_var5 = (**ppc_var3)();
        }
    }
    process_struct_1000_179c(0x18, (ppa_var5 >> 0x10));
    if (ppa_var5 == 0x0) {
        u_var6 = 0;
    } else {
        u_var6 = pass1_1030_1cd8(ppa_var5, 5, 5);
    }
    local_bx_4.field_0x4 = u_var6;
    local_bx_4.field_0x6 = (u_var6 >> 0x10);
    return;
}

pub unsafe fn pass1_1010_91cc(param_1: u32) {
    let pp_var1: fn();
    let mut u_var2: u16;
    let lVar3: u32;
    let mut local_4: u16;

    u_var2 = (param_1 >> 0x10);
    pp_var1 = ((param_1 + 4) + 0x10);
    lVar3 = (**pp_var1)();
    if (lVar3 != 0) {
        pp_var1 = ((param_1 + 4) + 8);
        (**pp_var1)();
    }
    return;
}

pub unsafe fn pass1_1010_9210(param_1: u32) {
    let pp_var1: fn();
    let u_var2: u8;

    pp_var1 = ((param_1 + 4) + 0xc);
    u_var2 = (**pp_var1)();
    return u_var2;
}

pub unsafe fn pass1_1010_922e(param_1: *mut Struct44, param_2: u8) -> *mut Struct44 {
    pass1_1010_8f78(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1010_9258(param_1: *mut Struct402) -> *mut Struct402 {
    pass1_1010_383a(param_1);
    param_1.a = 0x958e;
    (param_1 + 2) = 0x1010;
    return param_1;
}

pub unsafe fn pass1_1010_927a(param_1: *mut Struct404) {
    param_1.a = 0x958e;
    (param_1 + 2) = 0x1010;
    pass1_1010_3880(param_1);
    return;
}

pub unsafe fn pass1_1010_92e6(param_1: *mut u16) {
    unsafe {
        *param_1 = 0x9566;
        (param_1 + 2) = 0x1010;
    }
    pass1_1010_2db2(param_1);
    return;
}

pub unsafe fn pass1_1010_9304(param_1: u16, param_2: u16, param_1_00: i32) {
    let mut in_ax: i32;
    let in_dx: *mut Struct199;
    let mut local_4: u16;

    if (param_1_00 != 0) {
        process_struct_1000_179c(param_1_00 << 2, in_dx);
        return;
    }
    process_struct_1000_179c(0x1a, in_dx);
    if ((in_dx | in_ax) != 0) {
        pass1_1010_9258(in_ax, in_dx);
        return;
    }
    return;
}

pub unsafe fn pass1_1010_9372(
    param_1: *mut *mut u8,
    param_2: *mut u8,
    param_3: *mut u8,
    param_4: *mut u8,
    param_5: *mut u8,
) {
    let u16_1: *mut u8;
    let mut char_2: u8;
    let local_struct_461_1: *mut u8;
    let local_SI__1: *mut u8;
    let u16_2: *mut u8;
    let mut bool_1: bool;
    let mut u32_1: u32;
    // fn_ptr_1: *mut *mut u8;
    let mut temp_5ff2a2fc9a: u32;

    if (0 < param_4) {
        if (_g_bool_1050_3528 == 0x0) {
            fn_ptr_1 = (param_1 + 0x18);
            u16_1 = (**fn_ptr_1)();
            _g_bool_1050_3528 =
                process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(local_SI__1, u16_1));
        }
        temp_5ff2a2fc9a = (param_1 + 0xc);
        u32_1 = pass1_1010_2e02(_g_bool_1050_3528, (temp_5ff2a2fc9a + 0x12));
        local_struct_461_1 = param_2 + 1;
        u16_2 = param_3 + (0xfffe < param_2);
        char_2 = (param_4 + -1) * 0x4;
        while (char_2 != '\0') {
            bool_1 = CARRY2(local_struct_461_1, local_struct_461_1);
            local_struct_461_1 = (local_struct_461_1 * 2);
            u16_2 = (u16_2 * 2 + bool_1);
            char_2 = char_2 + -1;
        }
        pass1_1010_2e30(
            _g_bool_1050_3528,
            local_struct_461_1 | u32_1,
            u16_2 | (u32_1 >> 0x10),
            param_5,
        );
    }
    return;
}

pub unsafe fn pass1_1010_93f0(param_1: *mut Struct462) {
    let local_bx_4: *mut Struct462;
    let mut u_var1: u16;
    let mut unaff_ss: u16;
    let mut u_var2: u32;
    let mut local_1c: [u8; 26];

    u_var1 = (param_1 >> 0x10);
    local_bx_4 = param_1;
    if (&local_bx_4.field_0x56 == 0) {
        pass1_1010_9258(local_1c, unaff_ss);
        u_var2 = pass1_1010_398e(CONCAT22(unaff_ss, local_1c), 0, 0, 0);
        local_bx_4.field_0x56 = u_var2;
        local_bx_4.field_0x58 = (u_var2 >> 0x10);
        pass1_1010_927a(local_1c, unaff_ss);
    }
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn pass1_1010_9432(param_1: u32) {
    load_string_1010_847e(ctx._g_struct_73_1050_14cc, (param_1 + 0x16));
    return;
}

pub unsafe fn pass1_1010_944e(param_1: *mut Struct463, param_2: u16, param_3: u16) {
    let mut u_var1: u32;
    let fn_ptr_1: fn();

    if (param_1.field_0x56 == 0) {
        fn_ptr_1 = (CONCAT22(param_2, param_1) + 0x10);
        (**fn_ptr_1)();
    }
    u_var1 = pass1_1010_2e02(CONCAT22(param_2, param_1), param_3);
    pass1_1010_2e5c(param_1, param_2, u_var1);
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn pass1_1010_9488(param_1: u16, param_2: u16, param_1_00: u32) -> bool {
    let mut iVar1: i32;
    let mut u_var2: u16;
    let ppVar3: *mut pass1_struct_1;
    let mut u_var4: u16;
    let mut in_stack_0000ffee: u32;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let bool_1: bool;

    u_var4 = (param_1_00 + 0x12);
    ppVar3 = process_struct_1010_20ba(
        ctx._g_Struct372_1050_0ed0,
        CONCAT22((in_stack_0000ffee >> 0x10), 3),
    );
    u_var2 = (ppVar3 >> 0x10);
    local_a = ppVar3;
    if (u_var4 == 0x32) {
        iVar1 = pass1_1010_a5ca(local_a, u_var2, 0x32);
        if (iVar1 != 0) {
            return 0;
        }
        u_var4 = 0x4d;
    } else {
        if (u_var4 == 0x3f) {
            iVar1 = pass1_1010_a5ca(local_a, u_var2, 0x3f);
            if (iVar1 != 0) {
                return 0;
            }
            u_var4 = 0x4e;
        }
    }
    iVar1 = pass1_1010_a5ca(local_a, u_var2, u_var4);
    bool_1 = (iVar1 == 0);
    return bool_1;
}

pub unsafe fn pass1_1010_9502(param_1: u32) {
    let mut temp_5fd6aa1926: u32;

    temp_5fd6aa1926 = (param_1 + 0x16);
    return (temp_5fd6aa1926 + 2);
}

pub unsafe fn pass1_1010_951a(param_1: u32, param_2: u8) {
    pass1_1010_927a(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1010_9540(param_1: *mut Struct44, param_2: u8) -> *mut Struct44 {
    pass1_1010_92e6(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1010_95f8(in_struct_1: *mut Struct376) {
    let pu_var1: *mut u32;
    let mut u_var2: i32;
    let pu_var3: *mut u8;
    let local_struct_1: *mut Struct376;
    let mut local_es_5: u16;
    let fn_ptr_1: fn();

    local_es_5 = (in_struct_1 >> 0x10);
    local_struct_1 = in_struct_1;
    in_struct_1.ptr_a_lo = 0xa1c8;
    local_struct_1.ptr_a_hi = 0x1010;
    pu_var1 = local_struct_1.dc_handle_x0a;
    u_var2 = local_struct_1.u16_x0c;
    if ((u_var2 | pu_var1) != 0) {
        unsafe {
            fn_ptr_1 = *pu_var1;
        }
        (**fn_ptr_1)();
    }
    pu_var1 = local_struct_1.u16_x0e;
    u_var2 = local_struct_1.u16_x10;
    if ((u_var2 | pu_var1) != 0) {
        unsafe {
            fn_ptr_1 = *pu_var1;
        }
        (**fn_ptr_1)();
    }
    pu_var1 = local_struct_1.palette_handle_x12;
    pu_var3 = local_struct_1.u8_ptr_x14;
    if ((pu_var3 | pu_var1) != 0) {
        unsafe {
            fn_ptr_1 = *pu_var1;
        }
        (**fn_ptr_1)();
    }
    pass1_1010_1d80(in_struct_1);
    return;
}

pub unsafe fn pass1_1010_9674(param_1: u32) {
    let pu_var1: *mut u32;
    let mut u_var2: i32;
    let local_bx_4: *mut Struct466;
    let mut local_es_4: u16;
    let temp_862f6fe88cf: *mut u32;
    let fn_ptr_1: fn();

    local_es_4 = (param_1 >> 0x10);
    local_bx_4 = param_1;
    pu_var1 = local_bx_4.a;
    u_var2 = &local_bx_4.field_0x14;
    if ((u_var2 | pu_var1) != 0) {
        unsafe {
            fn_ptr_1 = *pu_var1;
        }
        (**fn_ptr_1)();
    }
    &local_bx_4.a = 0;
    return;
}

pub unsafe fn pass1_1010_96a8(param_1: u32, param_2: u16) {
    let piVar1: *mut i32;
    let mut local_es_6: u16;

    local_es_6 = (param_1 >> 0x10);
    piVar1 = (param_1 + 0x1e);
    unsafe {
        *piVar1 = *piVar1 - param_2;
        if (*piVar1 < 0) {
            (param_1 + 0x1e) = 0;
        }
    }
    return;
}

pub unsafe fn pass1_1010_96c2(param_1: u32) {
    return (param_1 + 0x1e);
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn pass1_1010_96d0(in_struct_1: *mut Struct467) {
    let piVar1: *mut i32;
    let local_struct_1: *mut Struct467;
    let mut local_es_4: u16;
    let mut local_DXAX_54: u32;
    let mut local_8: u16;

    local_es_4 = (in_struct_1 >> 0x10);
    local_struct_1 = in_struct_1;
    if (local_struct_1.a != 0) {
        if (0 < local_struct_1.b) {
            piVar1 = &local_struct_1.b;
            unsafe {
                *piVar1 = *piVar1 + -1;
            }
        }
        if ((local_struct_1.b == 0) && (local_struct_1.c != 0)) {
            local_8 = 1;
            local_DXAX_54 = pass1_1030_8326();
            local_DXAX_54._2_2_ = (local_DXAX_54 >> 0x10);
            if ((local_DXAX_54._2_2_ != 0) || (0x32 < local_DXAX_54)) {
                local_8 = 5;
            }
            if ((local_DXAX_54._2_2_ != 0) || (0x3c < local_DXAX_54)) {
                local_8 = 10;
            }
            if (local_struct_1.c < local_8) {
                local_8 = local_struct_1.c;
            }
            piVar1 = &local_struct_1.c;
            unsafe {
                *piVar1 = *piVar1 - local_8;
                if (*piVar1 < 0) {
                    local_struct_1.c = 0;
                }
            }
            if (0 < local_struct_1.c) {
                return local_8;
            }
            return 0xffff;
        }
    }
    return 0;
}

pub unsafe fn pass1_1010_9766(param_1: u32) {
    let mut iVar1: i32;
    let mut local_es_4: u16;

    local_es_4 = (param_1 >> 0x10);
    (param_1 + 0x1a) = 1;
    pass1_1010_a0a0();
    iVar1 = pass1_1010_9f8c(param_1, 0x80);
    (param_1 + 0x1e) = iVar1 * 0x32;
    return;
}

pub unsafe fn pass1_1010_9794(in_struct_1: *mut Struct468) {
    let mut iVar1: i32;
    let mut u_var2: u32;
    let pu_var3: *mut u8;
    let mut u_var4: u32;
    let pa_var5: *mut Struct199;
    
    let mut u_var6: i32;
    
    let local_struct_1: *mut Struct468;
    let mut u_var7: u16;
    let mut unaff_ss: u16;
    let mut u_var8: u32;
    let mut local_16: u32;
    let mut local_12: u32;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: [u8; 4];
    let mut local_6: u32;
    let fn_ptr_1: fn();

    u_var7 = (in_struct_1 >> 0x10);
    local_struct_1 = in_struct_1;
    if (local_struct_1.c == 0) {
        local_struct_1.c = 1;
        u_var4 = &local_struct_1.d;
        u_var6 = local_struct_1.b;
        pa_var5 = (u_var6 | u_var4);
        if (pa_var5 != 0x0) {
            fn_ptr_1 = u_var4;
            (**fn_ptr_1)();
            pa_var5 = ctx.dx_reg;
        }
        process_struct_1000_179c(0xc, pa_var5);
        u_var6 = pa_var5 | u_var4;
        if (u_var6 == 0) {
            pa_var5 = 0x0;
            u_var6 = 0;
        } else {
            pa_var5 = process_struct_1008_574a((u_var4 & 0xffff | ZEXT24(pa_var5) << 0x10));
        }
        local_struct_1.d = pa_var5;
        local_struct_1.b = u_var6;
        pass1_1008_5784(CONCAT22(unaff_ss, local_a), local_struct_1.a);
        loop {
            pu_var3 = local_a;
            pass1_1008_5b12(CONCAT22(unaff_ss, pu_var3));
            if ((ctx.dx_reg | pu_var3) == 0) {
                break;
            }
            iVar1 = (pu_var3 + 4);
            if ((iVar1 == 0x3e) || (iVar1 == 0x41)) {
                u_var8 = local_struct_1.a;
                (u_var8 + 10) = 0;
                u_var8 = local_struct_1.a;
                fn_ptr_1 = (local_struct_1.a + 0xc);
                (**fn_ptr_1)();
                u_var2 = local_struct_1.a;
                (u_var2 + 10) = 1;
                local_6 = 0;
                fn_ptr_1 = (&local_struct_1.d + 4);
                (**fn_ptr_1)(
                    &ctx.PTR_LOOP_1050_1008,
                    &local_struct_1.d,
                    CONCAT22(ctx.dx_reg, pu_var3),
                    u_var8,
                );
            }
        }
    }
    return;
}

pub unsafe fn pass1_1010_988c(param_1: *mut Struct469, param_2: u16) {
    let pp_var1: fn();
    let mut u_var2: u32;
    let mut i_var3: i32;
    let mut i_var4: i32;
    let mut u_var5: u16;
    let local_bx_4: *mut Struct469;
    let mut u_var6: u16;
    let mut unaff_ss: u16;
    let lVar7: u32;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: [u8; 8];

    u_var6 = (param_1 >> 0x10);
    local_bx_4 = param_1;
    pass1_1008_5784(CONCAT22(unaff_ss, local_a), local_bx_4.field_0xe);
    while {
        lVar7 = pass1_1008_5b12(CONCAT22(unaff_ss, local_a));
        u_var5 = (lVar7 >> 0x10);
        i_var3 = lVar7;
        if (lVar7 == 0) {
            return;
        }
        (i_var3 + 4) != param_2
    } {}
    i_var4 = (i_var3 + 6) + -1;
    (i_var3 + 6) = i_var4;
    if ((i_var4 < 1)
        && (
            pp_var1 = (local_bx_4.field_0xe + 0xc),
            (**pp_var1)(&ctx.PTR_LOOP_1050_1008, local_bx_4.field_0xe, lVar7),
            u_var2 = local_bx_4.field_0xe,
            (u_var2 + 8) == 0,
        ))
    {
        local_bx_4.field_0x16 = 1;
    }
    return;
}

pub unsafe fn pass1_1010_9f72(param_1: u32, param_2: u16) {
    let mut u_var1: u16;

    u_var1 = (param_1 >> 0x10);
    pass1_1010_9fa6(param_1, u_var1, (param_1 + 0xe), param_2);
    return;
}

pub unsafe fn pass1_1010_9f8c(param_1: u32, param_2: u16) {
    let mut u_var1: u16;

    u_var1 = (param_1 >> 0x10);
    pass1_1010_9fa6(param_1, u_var1, (param_1 + 10), param_2);
    return;
}

pub unsafe fn pass1_1010_9fa6(param_1: u16, param_2: u16, param_1_00: u32, param_2_00: i32) {
    let mut u_var1: u16;
    let mut unaff_ss: u16;
    let lVar2: u32;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: [u8; 8];

    if (param_1_00 != 0) {
        pass1_1008_5784(CONCAT22(unaff_ss, local_a), param_1_00);
        loop {
            lVar2 = pass1_1008_5b12(CONCAT22(unaff_ss, local_a));
            u_var1 = (lVar2 >> 0x10);
            if (lVar2 == 0) {
                break;
            }
            if ((lVar2 + 4) == param_2_00) {
                return (lVar2 + 6);
            }
        }
    }
    return 0;
}

pub unsafe fn pass1_1010_a0a0(param_1: u32) {
    let piVar1: *mut i32;
    let mut i_var2: i32;
    let mut i_var3: i32;
    let mut i_var4: i32;
    let mut u_var5: u16;
    let mut u_var6: u16;
    let mut unaff_ss: u16;
    let mut bVar7: bool;
    let mut bVar8: bool;
    let lVar9: u32;
    let mut local_1c: u16;
    let mut local_1a: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: [u8; 8];

    pass1_1008_5784(CONCAT22(unaff_ss, local_a), (param_1 + 10));
    local_c = 4;
    process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(local_1c, 2));
    if ((u16_1050_13ae != 2) && (u16_1050_13ae != 1)) {
        local_c = 2;
    }
    loop {
        loop {
            lVar9 = pass1_1008_5b12(CONCAT22(unaff_ss, local_a));
            u_var6 = (lVar9 >> 0x10);
            i_var4 = lVar9;
            if (lVar9 == 0) {
                return;
            }
            i_var2 = (i_var4 + 4);
            if (i_var2 != 0x12) {
                break;
            }
            piVar1 = (i_var4 + 6);
            unsafe {
                bVar8 = SBORROW2(*piVar1, 2);
                i_var3 = *piVar1 + -2;
            }
            bVar7 = i_var3 == 0;
            // LAB_1010_a151:
            if (!bVar7 && bVar8 == (i_var3 < 0)) {
                // LAB_1010_a153:
                piVar1 = (i_var4 + 6);
                unsafe {
                    *piVar1 = *piVar1 - (i_var4 + 6) / local_c;
                }
            }
        }
        if (((i_var2 != 0x3e) && (i_var2 != 0x41)) && (i_var2 != 0x80)) {
            if (i_var2 == 0x83) {
                piVar1 = (i_var4 + 6);
                unsafe {
                    bVar8 = SBORROW2(*piVar1, 1);
                    i_var2 = *piVar1;
                }
                i_var3 = i_var2 + -1;
                bVar7 = i_var2 == 1;
                // goto LAB_1010_a151;
            }
            // goto LAB_1010_a153;
        }
        u_var5 = (i_var4 + 6) / 2;
        piVar1 = (i_var4 + 6);
        unsafe {
            *piVar1 = *piVar1 - u_var5;
        }
        if (u_var5 == 0) {
            u_var5 = 1;
        }
        process_struct_1010_9fee(param_1, u_var5, (i_var4 + 4));
    }
}

pub unsafe fn pass1_1010_a172(param_1: u32, param_2: u8) {
    pass1_1010_95f8(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1010_a198(param_1: *mut u16, param_2: u8) {
    unsafe {
        *param_1 = ctx.s_1_1050_389a;
    }
    (param_1 + 2) = &ctx.PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1010_a478(param_1: *mut Struct376) {
    let pHVar1: *mut HDC16;
    let mut u_var2: i32;
    let local_bx_4: *mut Struct376;
    let mut u_var3: i32;
    let mut local_6: u16;
    let mut local_4: u16;

    u_var3 = (param_1 >> 0x10);
    local_bx_4 = param_1;
    param_1.ptr_a_lo = 0xe9cc;
    local_bx_4.ptr_a_hi = 0x1010;
    local_bx_4.dc_handle_x0a = 0xe9dc;
    local_bx_4.u16_x0c = 0x1010;
    if (&local_bx_4.field_0x138 != 0) {
        if ((u_var3 | local_bx_4) == 0) {
            pHVar1 = 0x0;
            u_var2 = 0;
        } else {
            pHVar1 = &local_bx_4.dc_handle_x0a;
            u_var2 = u_var3;
        }
        pass1_1010_1ea6(*&local_bx_4.field_0x138, CONCAT22(u_var2, pHVar1));
    }
    &local_bx_4.field_0x138 = 0;
    if (param_1 == 0x0) {
        pHVar1 = 0x0;
        u_var3 = 0;
    } else {
        pHVar1 = &local_bx_4.dc_handle_x0a;
    }
    local_6 = CONCAT22(u_var3, pHVar1);
    *local_6 = ctx.s_1_1050_389a;
    pHVar1[1] = (HDC16) & ctx.PTR_LOOP_1050_1008;
    pass1_1010_1d80(param_1);
    return;
}

pub unsafe fn pass1_1010_a50c(param_1: u32, param_2: u32, param_3: u32) {
    let local_AX_16: *mut Struct477;
    let local_AX_44: *mut Struct478;
    let mut local_8: u16;
    let mut local_4: u16;

    local_AX_16 = param_1;
    local_AX_16 = local_AX_16 + 1;
    pass1_1000_4906((param_1 & 0xffff0000 | ZEXT24(local_AX_16)), 0, 0x94);
    local_AX_44 = (&local_AX_16.field_0x0 + (param_3 + 10) * 6);
    local_4 = local_AX_44.field_0x12;
    local_8 = local_AX_44.field_0xe;
    (*local_8)(
        0x1000,
        &local_AX_16.field_0x0 + local_4,
        param_1._2_2_,
        param_2,
        param_3,
    );
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn pass1_1010_a568(param_1: u16, param_2: u16, param_3: u16) {
    let ppVar1: *mut pass1_struct_2;
    let mut in_dx: u16;

    ppVar1 = pass1_1030_8344(
        ctx._g_bool_1050_5748,
        (ctx._g_bool_1050_5748 >> 0x10),
        0x8000001,
    );
    pass1_1030_2622(CONCAT22(in_dx, ppVar1), param_1_00);
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn pass1_1010_a58a(param_1: u16, param_2: u16, param_3: u16) {
    let mut local_DXAX_18: u32;

    local_DXAX_18._0_2_ = pass1_1030_8344(
        ctx._g_bool_1050_5748,
        (ctx._g_bool_1050_5748 >> 0x10),
        0x8000001,
    );
    pass1_1030_266c(local_DXAX_18, CONCAT22(param_1_00, local_DXAX_18._2_2_));
    return;
}

pub unsafe fn pass1_1010_a5ac(param_1: u16, param_2: u16, param_1_00: u32) {
    let mut u_var1: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    u_var1 = pass1_1030_73a8(param_1_00);
    return (u_var1 + 0x20);
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn pass1_1010_a5ca(param_1: u16, param_2: u16, param_3: u16) {
    let mut local_DXAX_18: u32;

    local_DXAX_18._0_2_ = pass1_1030_8344(
        ctx._g_bool_1050_5748,
        (ctx._g_bool_1050_5748 >> 0x10),
        0x8000001,
    );
    pass1_1030_2242(CONCAT22(local_DXAX_18._2_2_, local_DXAX_18), param_1_00);
    return;
}

// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn pass1_1010_a5ec(param_1: u16, param_2: u16, param_3: u16, param_4: u32) {
    let pp_var1: fn();
    let ppVar2: *mut pass1_struct_2;
    let mut u_var3: u16;
    let mut u_var4: u16;
    let mut in_dx: u16;
    let pu_var5: *mut u32;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    if (param_4 != 0) {
        ppVar2 = pass1_1030_8344(
            ctx._g_bool_1050_5748,
            (ctx._g_bool_1050_5748 >> 0x10),
            0x8000001,
        );
        local_6 = CONCAT22(in_dx, ppVar2);
        pu_var5 = pass1_1030_73a8(param_4);
        u_var4 = (pu_var5 + 0x20);
        if (u_var4 != param_2_00) {
            u_var3 = param_2_00;
            pass1_1010_a5ca(param_1, param_2, u_var4);
            if ((u_var4 != 0x70) && (u_var3 < 0)) {
                pass1_1030_25d8(CONCAT22(in_dx, ppVar2), u_var3 + 1, u_var4);
            }
            unsafe {
                pp_var1 = (*pu_var5 + 8);
            }
            u_var4 = param_2_00;
            (**pp_var1)();
            if (param_2_00 != 0x70) {
                pass1_1010_a5ca(param_1, param_2, param_2_00);
                if (u_var4 < 0) {
                    pass1_1030_25d8(local_6, u_var4 - 1, param_2_00);
                }
            }
        }
    }
    return;
}

// WARNING: Variable defined which should be unmapped: local_36
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn pass1_1010_a69c(param_1: u32, param_2: u16) {
    let struct_b: *mut pass1_struct_2;
    let mut u_var1: u16;
    let mut in_dx: u16;
    let struct_c: *mut pass1_struct_1;
    let mut u_var2: u16;
    let mut local_36: u16;
    let mut local_34: u16;
    let mut bStack48: u8;
    let local_2f: u8;
    let mut local_2e: u16;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    local_36 = ctx._g_bool_1050_5748;
    local_34 = (ctx._g_bool_1050_5748 >> 0x10);
    struct_b = pass1_1030_8344(local_36, local_34, 0x8000001);
    if (param_2 == 1) {
        local_14 = 0;
        while (local_14 < 0x83) {
            u_var1 = pass1_1030_2242(CONCAT22(in_dx, struct_b), local_14);
            if (0x19 < u_var1) {
                local_16 = u_var1 - 5;
                if (local_16 < 0x19) {
                    local_16 = 0x19;
                }
                pass1_1030_25d8(CONCAT22(in_dx, struct_b), local_16, local_14);
            }
            local_14 = local_14 + 1;
        }
        // goto switchD_1010_aaef_caseD_b;
    }
    pass1_1030_25f0(CONCAT22(in_dx, struct_b), 0, param_2);
    struct_c = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(local_2e, 0x35));
    u_var2 = param_1;
    local_36 = (param_1 >> 0x10);
    u_var1 = local_36;
    match (param_2) {
        10 | 0xc => local_36 = 0x1b,
        _ => {}
        // goto switchD_1010_aaef_caseD_b;
        0x10 => {
            pass1_1010_682e(struct_c, 1, 0x2d);
            if (struct_b.field_0x160 == 0) {}
            // goto switchD_1010_aaef_caseD_b;
            local_34 = 0x2d;
            // goto LAB_1010_a91f;
        }
        0x12 => {
            pass1_1010_682e(struct_c, 1, 0x16);
            pass1_1010_682e(struct_c, 1, 0x17);
            pass1_1010_682e(struct_c, 1, 0x18);
            pass1_1010_682e(struct_c, 1, 0x40);
            bStack48 = 0x3f;
            // goto LAB_1010_a96c;
        }
        0x13 => {
            local_34 = 0x35;
            // goto LAB_1010_a91f;
        }
        0x19 => {
            // goto switchD_1010_aaef_caseD_19;
        }
        0x1a => {
            bStack48 = 0xf;
            // goto LAB_1010_a96c;
        }
        0x1c => {
            bStack48 = 0x11;
            // goto LAB_1010_a96c;
        }
        0x1d | 0x24 => {
            pass1_1010_abd2(u_var2, local_36, 0x1e);
            local_34 = 0x5b;
            // goto LAB_1010_a91f;
        }
        0x1e => {
            struct_c = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, 0x1002b);
            pass1_1010_08c0(struct_c, 1, 2);
            struct_c =
                process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(local_2e, 0x40));
            wsprintf_func_1008_b69c(struct_c);
            // goto switchD_1010_aaef_caseD_b;
        }
        0x22 => {
            bStack48 = 8;
            // goto LAB_1010_aabe;
        }
        0x23 => {
            bStack48 = 0xc;
            // goto LAB_1010_aabe;
        }
        0x25 => {
            pass1_1010_abd2(u_var2, local_36, 0x14);
            pass1_1010_abd2(u_var2, local_36, 0x1b);
            pass1_1010_abd2(u_var2, local_36, 0x1e);
            pass1_1010_abd2(u_var2, local_36, 0x22);
            pass1_1010_abd2(u_var2, local_36, 0x25);
            pass1_1010_abd2(u_var2, local_36, 0x28);
            pass1_1010_abd2(u_var2, local_36, 0x2a);
            pass1_1010_abd2(u_var2, local_36, 0x2d);
            pass1_1010_abd2(u_var2, local_36, 0x2f);
            pass1_1010_abd2(u_var2, local_36, 0x31);
            pass1_1010_abd2(u_var2, local_36, 0x35);
            pass1_1010_abd2(u_var2, local_36, 0x38);
            pass1_1010_abd2(u_var2, local_36, 0x3a);
            pass1_1010_abd2(u_var2, local_36, 0x3c);
            pass1_1010_abd2(u_var2, local_36, 0x48);
            pass1_1010_abd2(u_var2, local_36, 0x4f);
            pass1_1010_abd2(u_var2, local_36, 0x52);
            pass1_1010_abd2(u_var2, local_36, 0x54);
            pass1_1010_abd2(u_var2, local_36, 0x57);
            pass1_1010_abd2(u_var2, local_36, 0x5b);
            pass1_1010_abd2(u_var2, local_36, 0x5d);
            pass1_1010_abd2(u_var2, local_36, 0x62);
            pass1_1010_abd2(u_var2, local_36, 0x66);
            pass1_1010_abd2(u_var2, local_36, 0x68);
            pass1_1010_abd2(u_var2, local_36, 0x6c);
            // goto switchD_1010_aaef_caseD_19;
        }
        0x29 => local_36 = 0x25,
        0x2a => {
            bStack48 = 0xf;
            // goto LAB_1010_aabe;
        }
        0x2b => {
            bStack48 = 0x6e;
            // goto LAB_1010_a96c;
        }
        0x30 => local_36 = 0x54,
        0x33 => {
            pass1_1010_abd2(u_var2, local_36, 0x31);
            local_34 = 0x6c;
            // goto LAB_1010_a91f;
        }
        0x36 => {
            bStack48 = 0x13;
            // goto LAB_1010_aabe;
        }
        0x37 => {
            bStack48 = 0x2c;

            // LAB_1010_a96c:
            pass1_1010_682e(struct_c, 1, bStack48);
            // goto switchD_1010_aaef_caseD_b;
        }
        0x38 => {
            pass1_1010_682e(struct_c, 1, 0x28);
            if (struct_b.field_0x160 == 0) {}
            // goto switchD_1010_aaef_caseD_b;
            local_34 = 0x28;
        }
        // goto LAB_1010_a91f;
        0x39 => {
            bStack48 = 0x10;
            // goto LAB_1010_aabe;
        }
        0x3a => {
            bStack48 = 0x11;
            // goto LAB_1010_aabe;
        }
        0x3b => {
            bStack48 = 0x12;
            // LAB_1010_aabe:
            pass1_1010_6814(struct_c, 1, bStack48);
            // goto switchD_1010_aaef_caseD_b;
        }
        0x3c => {
            pass1_1010_abd2(u_var2, local_36, 0x14);
            local_34 = 0x62;
            // goto LAB_1010_a91f;
        }
        0x3d => {
            pass1_1010_682e(struct_c, 1, 0x66);
            if (struct_b.field_0x160 == 0) {}
            // goto switchD_1010_aaef_caseD_b;
            local_34 = 0x66;
            // LAB_1010_a91f:
            pass1_1010_abd2(u_var2, local_36, local_34);
            // goto switchD_1010_aaef_caseD_b;
        }
        0x3e => local_36 = 0x5d,
        0x3f => local_36 = 0x22,
        0x40 => local_36 = 0x57,
        0x41 => {
            local_36 = 0x4f;
        }
    }
    pass1_1010_abd2(u_var2, u_var1, local_36);
    // switchD_1010_aaef_caseD_b:
    bStack48 = param_2;
    local_2f = (param_2 >> 8);
    struct_c = process_struct_1010_20ba(
        ctx._g_Struct372_1050_0ed0,
        CONCAT13(local_2f, CONCAT12(bStack48, 0x37)),
    );
    u_var1 = pass1_1008_ab12(struct_c, (struct_c >> 0x10), bStack48);
    if (u_var1 != 0) {
        post_win_msg_1008_a0e4(struct_c, 0, 0, 1, 0, u_var1);
    }
    post_win_msg_1008_a0e4(struct_c, 0, 0, 1, 0, 0x3d);
    struct_c = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, 0x1002b);
    pass1_1010_043a(struct_c, 0x4000001, 6);
    return;
    // switchD_1010_aaef_caseD_19:
    (struct_c + 0x148) = 0x34;
    // goto switchD_1010_aaef_caseD_b;
}

// WARNING: Variable defined which should be unmapped: local_12
// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn pass1_1010_abd2(param_1: u16, param_2: u16, param_3: u16) {
    let pu_var1: *mut u16;
    let mut u_var2: i32;
    let mut unaff_si: u16;
    let mut unaff_ss: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    local_6 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(unaff_si, 0x35));
    u_var2 = local_6 + 10;
    _local_a = local_6 & 0xffff0000 | u_var2;
    local_c = 0;
    local_10 = param_1_00;
    _local_14 = CONCAT22(unaff_ss, &stack0x000a);
    loop {
        pu_var1 = _local_14;
        if (local_10 == 0) {
            return;
        }
        if (local_c != 0) {
            break;
        }
        if ((local_10 * 2 + u_var2) != 0) {
            local_c = 1;
            local_e = local_10;
        }
        _local_14 = (_local_14 & 0xffff0000 | (local_14 + 2));
        unsafe {
            local_10 = *pu_var1;
        }
    }
    pass1_1010_682e(local_6, 0, local_e);
    pass1_1010_682e(local_6, 1, local_10);
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn pass1_1010_ac62(param_1: u16, param_2: u16, param_1_00: i32) {
    let ppVar1: *mut pass1_struct_2;
    let mut in_dx: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    ppVar1 = pass1_1030_8344(
        ctx._g_bool_1050_5748,
        (ctx._g_bool_1050_5748 >> 0x10),
        0x8000001,
    );
    return (&ppVar1.field_0x116 + param_1_00 * 2);
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn pass1_1010_ac92(param_1: u16, param_2: u16, param_1_00: i32) {
    if ((0 < param_1_00) && (param_1_00 < 0x43)) {
        load_string_1010_847e(ctx._g_struct_73_1050_14cc, param_1_00 + 0x664);
        return;
    }
    return;
}

pub unsafe fn pass1_1010_acc0(param_1: u16, param_2: u16, param_1_00: u32) {
    let mut u_var1: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    u_var1 = pass1_1030_73a8(param_1_00);
    if ((u_var1 + 0x12) != 4) {
        return 1;
    }
    return 0;
}

pub unsafe fn pass1_1010_acec(param_1: u32, param_2: u16) {
    if (param_2 == 1) {
        (param_1 + 0x12e) = 0;
    } else {
        if (param_2 != 5) {
            return;
        }
    }
    pass1_1010_1f62((param_1 & 0xffff0000 | (param_1 - 10)), param_2);
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn pass1_1010_ad22(param_1: u32) {
    let mut u_var1: u32;
    let pu_var2: *mut u8;
    
    let mut unaff_ss: u16;
    let mut local_8: u16;

    u_var1 = (param_1 + 0x138);
    pu_var2 = &stack0x0008;
    pass1_1030_627e(
        _PTR_LOOP_1050_5740,
        CONCAT22(unaff_ss, pu_var2),
        (u_var1 + 0x20),
    );
    if ((ctx.dx_reg | pu_var2) == 0) {
        return;
    }
    local_8 = ctx.dx_reg;
    pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, pu_var2, ctx.dx_reg);
    return;
}

pub unsafe fn pass1_1010_ad64(param_1: u16, param_2: u32, param_3: u32) {
    let mut u_var1: u32;
    let mut local_a: u32;

    if ((param_3 != 0) && (u_var1 = (param_3 + 0x2e), (u_var1 + 0x200) == 0x8000002)) {
        return;
    }
    pass1_1010_c58a(param_1, param_2, (param_2 >> 0x10), param_3);
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn pass1_1010_ada6(param_1: u16, param_2: u16, param_1_00: *mut u8, param_2_00: i32) {
    let mut u_var1: u16;
    let mut in_dx: u16;
    let mut local_6: u32;

    local_6 = 0;
    if (param_2_00 == 6) {
        if (param_1_00 == 0x0) {}
        // goto LAB_1010_adee;
        u_var1 = big_switch_statement_1020_c222(param_1_00);
    } else {
        if (param_2_00 != 7) {
            return 0;
        }
        if (param_1_00 == 0x0) {}
        // goto LAB_1010_adee;
        u_var1 = big_switch_statement_1020_c2f8(param_1_00);
    }
    local_6 = CONCAT22(in_dx, u_var1);
    // LAB_1010_adee:
    if (local_6 == 0) {
        local_6 = load_string_1010_847e(ctx._g_struct_73_1050_14cc, 0x531);
    }
    return local_6;
}

pub unsafe fn pass1_1010_ae12(param_1: u16, param_2: u16, param_1_00: u32, param_2_00: i32) {
    let mut u_var1: u16;
    let mut i_var2: i32;
    let mut in_dx: u16;
    let mut local_4: u16;

    if (param_2_00 == 6) {
        local_4 = 0;
        while (local_4 < 0x15) {
            u_var1 = big_switch_statement_1020_c222(local_4);
            i_var2 = pass1_1000_3d7a(param_1_00, u_var1, in_dx);
            if (i_var2 == 0) {
                return local_4;
            }
            local_4 = local_4 + 1;
        }
    } else {
        if (param_2_00 == 7) {
            local_4 = 0;
            while (local_4 < 0x11) {
                u_var1 = big_switch_statement_1020_c2f8(local_4);
                i_var2 = pass1_1000_3d7a(param_1_00, u_var1, in_dx);
                if (i_var2 == 0) {
                    return local_4;
                }
                local_4 = local_4 + 1;
            }
        }
    }
    return 0xffff;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn pass1_1010_ae92(param_1: u32, param_2: u16, param_3: u16, param_4: u32) {
    let mut u_var1: u16;
    let mut u_var2: u32;
    let mut u_var3: u32;
    let ppVar4: *mut pass1_struct_1;
    let mut u_var5: u16;
    let mut u_var6: u16;
    let u_var7: u8;
    let u_var8: u8;
    let u_var9: u8;
    let u_var10: u8;
    let mut u_var11: u16;
    let mut u_var12: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_8: u16;

    if (param_3 == 0x15) {
        u_var2 = pass1_1030_73a8(param_4);
        if (u_var2 != 0) {
            (u_var2 + 0x20) = param_2;
            return;
        }
    } else {
        if (param_3 < 0x16) {
            if (param_3 == '\x06') {
                pass1_1030_7f1a(param_4, param_2);
                u_var2 = pass1_1030_73a8(param_4);
                u_var1 = pass1_1010_b028(param_1, (param_1 >> 0x10), u_var2);
                u_var3 = pass1_1030_8326();
                if (((u_var1 == 0xe) && ((u_var3 >> 0x10) != 0 || (0x32 < u_var3)))
                    && (param_2 == 1 || ((param_2 == 2 || (param_2 == 4)) || (param_2 == 3))))
                {
                    u_var12 = 0;
                    u_var1 = 0xb;
                    u_var9 = 1;
                    u_var10 = 0;
                    u_var11 = 0;
                    u_var6 = 0;
                    u_var7 = 0;
                    u_var8 = 0;
                    u_var5 = 0;
                    ppVar4 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, 0x37);
                    post_win_msg_1008_a0e4(
                        ppVar4,
                        CONCAT22(u_var6, u_var5),
                        CONCAT11(u_var8, u_var7),
                        CONCAT11(u_var10, u_var9),
                        CONCAT22(u_var12, u_var11),
                        u_var1,
                    );
                    return;
                }
            } else {
                if (param_3 == 0x7) {
                    pass1_1030_7eda(param_4, param_2);
                    return;
                }
            }
        }
    }
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn pass1_1010_af66(param_1: u32) {
    let mut u_var1: u32;
    let paVar2: *mut Struct493;
    let mut in_dx: i32;
    let mut in_stack_00000008: u16;
    let mut local_8: u16;
    let mut temp_5f97cf777f: u32;

    temp_5f97cf777f = (param_1 + 0x138);
    u_var1 = (temp_5f97cf777f + 0x24);
    paVar2 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, u_var1, (u_var1 >> 0x10));
    if ((in_dx | paVar2) == 0) {
        return;
    }
    pass1_1038_5050(CONCAT22(in_dx, paVar2), in_stack_00000008);
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn pass1_1010_afa2(param_1: *mut u32) {
    let mut u_var1: u32;
    let paVar2: *mut Struct493;
    let mut in_dx: i32;
    let mut in_stack_00000008: u16;
    let mut local_8: u16;
    let mut temp_5fb906f77c: u32;

    temp_5fb906f77c = (param_1 + 0x138);
    u_var1 = (temp_5fb906f77c + 0x24);
    paVar2 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, u_var1, (u_var1 >> 0x10));
    if ((in_dx | paVar2) == 0) {
        return;
    }
    pass1_1038_50e0(CONCAT22(in_dx, paVar2), in_stack_00000008);
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn pass1_1010_afde(param_1: u32, param_2: u16) {
    let mut u_var1: u32;
    let paVar2: *mut Struct493;
    let pu_var3: *mut u8;
    let mut in_dx: i32;
    let mut u_var4: i32;
    let mut local_a: u16;
    let mut local_8: u16;

    u_var1 = (param_1 + 0x138);
    u_var1 = (u_var1 + 0x24);
    paVar2 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, u_var1, (u_var1 >> 0x10));
    u_var4 = in_dx | paVar2;
    if (u_var4 == 0) {
        return;
    }
    pu_var3 = pass1_1008_c6fa(ctx._PTR_LOOP_1050_06e0, param_2);
    pass1_1038_4e78(CONCAT22(in_dx, paVar2), pu_var3 & 0xffff | u_var4 << 0x10);
    return;
}

pub unsafe fn pass1_1010_b028(param_1: u16, param_2: u16, param_1_00: u32) {
    return (param_1_00 + 0xc);
}

/*
Unable to decompile 'big_fn_1010_b038'
// Cause: Exception while decompiling 1010:b038: process: timeout

*/

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn pass1_1010_bf08(param_1: u16, param_2: u16, param_1_00: u32) {
    pass1_1028_e1bc();
    return;
}

// WARNING: Could not reconcile some variable overlaps

pub unsafe fn pass1_1010_bf1e(param_1: u32, param_2: u32) {
    let mut u_var1: u32;
    let mut in_ax: i32;
    let mut in_i16_5: i32;
    let p_uvar2: *mut u16;
    let struct_a: *mut Struct199;
    let in_u16_6: *mut Struct199;
    
    let mut i_var3: i32;
    let mut u_var4: u16;
    let mut unaff_ss: u16;
    let mut local_28: u16;
    let mut local_26: u16;
    let mut local_24: u16;
    let mut local_22: u32;
    let mut local_1e: u16;
    let mut local_1c: u16;
    let mut local_1a: u32;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_4: u16;

    pass1_1010_bf08(param_1, (param_1 >> 0x10), 0x1000000);
    local_4 = in_ax - 1;
    param_2 = local_4;
    in_i16_5 = local_4 * 0x18;
    in_u16_6 = struct_a;
    process_struct_1000_179c(in_i16_5, struct_a);
    _local_28 = CONCAT22(in_u16_6, in_i16_5);
    i_var3 = param_2;
    u_var4 = (param_2 >> 0x10);
    if ((in_u16_6 | in_i16_5) == 0) {
        (i_var3 + 2) = 0;
    } else {
        call_fn_ptr_1000_5586(0x4092, 0x1020, local_4, 0x18, in_i16_5, in_u16_6);
        (i_var3 + 2) = _local_28;
    }
    pass1_1028_dc52(
        CONCAT22(unaff_ss, &local_16),
        (&ctx.PTR_LOOP_1050_0000 + 1),
        0,
        0x100,
    );
    local_1a = (i_var3 + 2);
    local_24 = 0;
    loop {
        pu_var2 = &local_16;
        pass1_1028_e4ec(CONCAT22(unaff_ss, pu_var2));
        if ((ctx.dx_reg | pu_var2) == 0) {
            break;
        }
        u_var1 = (pu_var2 + 8);
        if (u_var1 != 0) {
            modify_list_1008_3f62(local_1a, u_var1 & 0xffff0000 | (u_var1 + 4));
            (local_1a + 0xc) = local_24;
            local_24 = local_24 + 1;
            local_1a = local_1a & 0xffff0000 | (local_1a + 0x18);
        }
    }
    return;
}

// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn pass1_1010_bffa(param_1: u32) {
    let pu_var1: *mut u32;
    let pi_var2: *mut i32;
    let mut u_var3: u32;
    let mut in_ax: i32;
    let mut u_var4: i32;
    let mut u_var5: u16;
    let puVar6: *mut u16;
    let mut u_var7: i32;
    let in_dx: *mut Struct199;
    let struct_a: *mut Struct199;
    let struct_a_00: *mut Struct199;
    
    
    let mut local_30: u16;
    let mut local_31: i32;
    let mut local_32: i32;
    let mut local_33: u16;
    let mut unaff_ss: u16;
    let mut local_2a: u32;
    let mut local_22: u16;
    let mut local_20: u16;
    let mut local_1e: u16;
    let mut local_1c: u32;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    process_struct_1000_179c(10, in_dx);
    local_6 = CONCAT22(in_dx, in_ax);
    pass1_1010_bf08(param_1, (param_1 >> 0x10), 0x2000000);
    local_33 = (local_6 >> 0x10);
    local_31 = local_6;
    (local_31 + 8) = in_ax;
    if (in_ax == 0) {
        (local_31 + 8) = 1;
    }
    u_var4 = (local_31 + 8) << 2;
    struct_a_00 = struct_a;
    process_struct_1000_179c(u_var4, struct_a);
    local_33 = (local_6 >> 0x10);
    local_31 = local_6;
    local_6 = u_var4;
    (local_31 + 2) = struct_a_00;
    if ((struct_a_00 | local_6) == 0) {
        (local_31 + 8) = 0;
    } else {
        u_var5 = (local_31 + 8) * 2;
        process_struct_1000_179c(u_var5, struct_a_00);
        local_33 = (local_6 >> 0x10);
        local_31 = local_6;
        (local_31 + 4) = u_var5;
        (local_31 + 6) = struct_a_00;
        u_var4 = struct_a_00 | (local_31 + 4);
        if (u_var4 != 0) {
            mixed_fn_1010_830a(ctx._g_struct_73_1050_14cc, 0x1b4);
            pu_var1 = *local_6;
            unsafe {
                *pu_var1 = u_var4;
            }
            (pu_var1 + 2) = ctx.dx_reg;
            (local_6 + 4) = 0;
            pass1_1028_dc52(
                CONCAT22(unaff_ss, &local_18),
                (&ctx.PTR_LOOP_1050_0000 + 1),
                0,
                0x200,
            );
            local_1e = 1;
            loop {
                puVar6 = &local_18;
                pass1_1028_e4ec(CONCAT22(unaff_ss, puVar6));
                if ((ctx.dx_reg | puVar6) == 0) {
                    break;
                }
                pi_var2 = (puVar6 + 8);
                unsafe {
                    local_31 = *pi_var2;
                }
                local_22 = 0;
                u_var7 = local_31 - 1;
                u_var4 = u_var7;
                if (u_var7 < 10) {
                    u_var4 = pi_var2;
                    match (u_var7) {
                        0 => {
                            local_22 = 0x1b5;
                        }
                        1 => {
                            local_22 = 0x1b6;
                        }
                        2 => {
                            local_22 = 0x1b7;
                        }
                        3 => {
                            local_22 = 0x1b8;
                        }
                        4 => {
                            local_22 = 0x1b9;
                        }
                        5 => {
                            local_22 = 0x1ba;
                        }
                        6 => {
                            local_22 = 0x1bb;
                        }
                        7 => {
                            local_22 = 0x1bc;
                        }
                        8 => {
                            local_22 = 0x1bd;
                        }
                        9 => {
                            local_22 = 0x1be;
                        }
                    }
                }
                mixed_fn_1010_830a(ctx._g_struct_73_1050_14cc, local_22);
                local_33 = (*local_6 >> 0x10);
                local_32 = *local_6;
                (local_32 + local_1e * 4) = u_var4;
                (local_32 + local_1e * 4 + 2) = ctx.dx_reg;
                u_var3 = (local_6 + 4);
                (u_var3 + local_1e * 2) = local_31;
                local_1e = local_1e + 1;
            }
            return;
        }
    }
    return;
}

pub unsafe fn pass1_1010_c1ba(param_1: u16, param_2: u16, param_3: u16) {
    let mut unaff_ss: u16;
    let lVar1: u32;
    let mut local_22: u16;
    let mut local_1c: u16;
    let mut local_1a: u32;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_4: u16;

    local_4 = 0;
    pass1_1028_dc52(
        CONCAT22(unaff_ss, &local_16),
        (&ctx.PTR_LOOP_1050_0000 + 1),
        0,
        0x200,
    );
    local_1c = 1;
    while ((
        lVar1 = pass1_1028_e4ec(CONCAT22(unaff_ss, &local_16)),
        lVar1 != 0 && (local_1c != param_1_00),
    )) {
        local_1c = local_1c + 1;
    }
    return;
}

pub unsafe fn pass1_1010_c234(param_1: u32) {
    let mut u_var1: u32;
    let mut local_4: u16;

    u_var1 = pass1_1010_c28a(param_1);
    if (u_var1 == 0) {
        return 0;
    }
    u_var1 = pass1_1038_4d28(u_var1);
    return u_var1;
}

pub unsafe fn pass1_1010_c25e(param_1: u32, param_2: u32) {
    let paVar1: *mut Struct1114;
    let mut local_4: u16;

    if (param_2 != 0) {
        paVar1 = pass1_1010_c28a(param_1);
        if (paVar1 != 0x0) {
            pass1_1038_4d3c(paVar1, param_2);
        }
    }
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn pass1_1010_c28a() {
    let mut u_var1: i32;
    let mut u_var2: u16;
    let paVar3: *mut Struct493;
    let mut u_var4: i32;
    let ppVar5: *mut pass1_struct_1;
    let mut in_stack_0000ffec: u32;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    ppVar5 = process_struct_1010_20ba(
        ctx._g_Struct372_1050_0ed0,
        CONCAT22((in_stack_0000ffec >> 0x10), 0x2f),
    );
    u_var2 = (ppVar5 >> 0x10);
    u_var1 = (ppVar5 + 0x24);
    u_var4 = (ppVar5 + 0x26);
    if ((u_var4 | u_var1) != 0) {
        paVar3 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, u_var1, u_var4);
        if ((u_var4 | paVar3) != 0) {
            return;
        }
    }
    return;
}

// WARNING: Could not reconcile some variable overlaps

pub unsafe fn pass1_1010_c2d8(param_1: u16, param_2: u16, param_1_00: libc::c_long) {
    let mut u_var1: u16;
    let mut local_6: u32;

    if ((param_1_00 != 0)
        && (
            u_var1 = (param_1_00 >> 0x10),
            local_6._0_2_ = *(param_1_00 + 0x2e),
            ((param_1_00 + 0x30) | local_6) != 0,
        ))
    {
        return;
    }
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn pass1_1010_c312() {
    return CONCAT22((ctx._PTR_LOOP_1050_65e2 + 2), *ctx._PTR_LOOP_1050_65e2);
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn pass1_1010_c320(param_1: u16, param_2: u16, param_1_00: u32, param_2_00: u32) {
    let in_dx: *mut Struct199;
    let mut u_var1: u32;
    let mut in_resource_id: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;

    local_6 = param_1_00;
    if (param_1_00 == 0) {
        process_struct_1000_179c(0x100, in_dx);
        local_6 = param_1_00 & 0xffff | ZEXT24(in_dx) << 0x10;
    }
    u_var1 = pass1_1030_73a8(param_2_00);
    match (u_var1 + 0x12) {
        1 | 2 | 4 => in_resource_id = 0x594,
        3 | 5 => in_resource_id = 0x593,
        6 => in_resource_id = 0x595,
        7 => in_resource_id = 0x596,
        8 => in_resource_id = 0x5f3,
        9 => in_resource_id = 0x664,
        _ => {
            *local_6 = 0;
            return;
        }
    }
    load_string_1010_84e0(
        ctx._g_struct_73_1050_14cc,
        (ctx._g_struct_73_1050_14cc >> 0x10),
        0x3ff,
        local_6,
        in_resource_id,
    );
    return;
}

// WARNING: Could not reconcile some variable overlaps

pub unsafe fn pass1_1010_c3c2(param_1: u16, param_2: u16, param_1_00: u32, param_2_00: u32) {
    let mut switch_var: u16;
    let in_dx: *mut Struct199;
    let mut u_var1: u16;
    let mut unaff_ss: u16;
    let mut local_410: u16;
    let mut local_40e: u16;
    let mut local_40c: [u8; 1024];
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;

    local_6 = param_1_00;
    if (param_1_00 == 0) {
        process_struct_1000_179c(0x100, in_dx);
        local_6 = param_1_00 & 0xffff | ZEXT24(in_dx) << 0x10;
    }
    _local_a = pass1_1030_73a8(param_2_00);
    u_var1 = (_local_a >> 0x10);
    switch_var = (_local_a + 0xc);
    local_c = switch_var;
    big_switch_statement_1020_bd80(switch_var);
    copy_string_1000_3d3e(CONCAT22(unaff_ss, local_40c), CONCAT22(u_var1, switch_var));
    pass1_1030_8086(param_2_00);
    string_fn_1000_3f9c(
        local_6,
        (local_6 >> 0x10),
        s__s___lu_1050_38c5,
        &ctx.g_alloc_addr_1050_1050,
        local_40c,
    );
    return;
}

pub unsafe fn pass1_1010_c58a(param_1: u16, param_2: u16, param_3: u16, param_3_00: u32) {
    let ctx.ax_reg: *mut Struct479;
    let mut u_var1: i32;
    let mut u_var2: i32;
    let in_dx: *mut Struct199;
    
    let mut u_var3: i32;
    let struct_a: *mut Struct199;
    let ctx.dx_reg: *mut Struct199;
    let mut local_1a: u32;
    let mut local_14: u16;
    let mut local_12: u32;
    let mut local_e: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    process_struct_1000_179c(0x18, in_dx);
    u_var1 = _local_AX__1;
    if ((in_dx | u_var1) == 0) {
        u_var1 = 0;
        u_var3 = 0;
    } else {
        process_struct_1040_a598((_local_AX__1 & 0xffff | ZEXT24(in_dx) << 0x10));
        u_var3 = ctx.dx_reg;
    }
    local_6 = CONCAT22(u_var3, u_var1);
    struct_a = (u_var3 | u_var1);
    if (struct_a == 0x0) {
        return;
    }
    local_e = 0;
    local_12 = 0;
    match (param_3) {
        5 => {
            local_e = &u16_1050_352c;
            local_12 = 0x300fa4
        }
        _ => {
            if (local_6 == 0x0) {
                return;
            }
            pass1_1040_a5d0(CONCAT22(u_var3, u_var1));
            error_check_1000_17ce(local_6);
            return;
        }
        10 => {
            local_e = &u16_1050_358c;
            local_12 = 0x510fb3
        }
        0xb => {
            local_e = &u16_1050_358c;
            local_12 = 0x510fb4
        }
        0xc => {
            local_e = &u16_1050_362e;
            local_12 = 0x300fb6
        }
        0x10 => {
            local_e = &PTR_DAT_1050_1805_1050_368e;
            local_12 = 0x3c0fc4
        }
        0x11 => {
            local_e = &PTR_DAT_1050_1805_1050_3706;
            local_12 = 0x4b0fc1
        }
        0x12 => {
            local_e = &u16_1050_379c;
            local_12 = 0x80fc5
        }
        0x13 => {
            pass1_1010_debe(
                CONCAT22(param_2, param_1),
                param_3,
                CONCAT22(u_var3, u_var1 + 0x10),
                CONCAT22(u_var3, u_var1 + 0xc),
                param_3_00,
            );
            local_e = &u16_1050_37ac;
            local_12 = 0x10fc6;
            struct_a = ctx.dx_reg
        }
        0x15 => {
            (u_var1 + 6) = param_3_00;
            (u_var1 + 10) = param_3
        }
        0x16 => {
            local_e = &u16_1050_37ae;
            local_12 = 0x40157
        }
        0x17 => {
            local_e = &u16_1050_37b6;
            local_12 = 0x2c0fd8;
        }
    }
    if (local_e != 0) {
        local_6.ptr_a_lo = local_12._2_2_;
        u_var2 = local_12._2_2_ * 10 + 2;
        process_struct_1000_179c(u_var2, struct_a);
        local_1a = CONCAT22(struct_a, u_var2);
        if ((struct_a | u_var2) == 0) {
            (u_var1 + 2) = 0;
        } else {
            local_1a = local_12._2_2_;
            call_fn_ptr_1000_5586(
                0xa564,
                &ctx.PTR_LOOP_1050_1040,
                local_12._2_2_,
                10,
                u_var2 + 2,
                struct_a,
            );
            (u_var1 + 2) = u_var2 + 2;
            (u_var1 + 4) = struct_a;
        }
        (u_var1 + 6) = param_3_00;
        (u_var1 + 10) = param_3;
        (u_var1 + 0x12) = local_12;
        pass1_1010_a50c(
            CONCAT22(param_2, param_1),
            local_e,
            CONCAT22(u_var3, u_var1),
        );
    }
    return;
}

pub unsafe fn pass1_1010_c864(param_1: u32, param_2: u32, param_3: u32) {
    let plVar1: *mut long;
    let ppc_var2: fn();
    let mut u_var3: u32;
    let u_var4: u8;
    let pu_var5: *mut u32;
    let mut u_var6: i32;
    let pa_var7: *mut Struct493;
    let mut u_var8: u16;
    let extraout_var: u32;
    
    
    let mut u_var10: i32;
    
    
    let mut i_var11: i32;
    let mut iVar12: i32;
    let mut u_var13: u16;
    let mut u_var14: u16;
    let mut uVar15: u16;
    let mut unaff_ss: u16;
    let mut uVar16: u16;
    let mut uVar17: u16;
    let mut local_1fe: u16;
    let mut local_1fc: u16;
    let mut local_1fa: u16;
    let mut local_1f8: u16;
    let mut local_1f6: u16;
    let mut local_1f0: u16;
    let mut local_1ee: u16;
    let mut local_19c: u32;
    let mut local_198: u16;
    let mut local_196: u16;
    let mut local_194: u16;
    let mut local_192: u16;
    let mut local_190: u16;
    let mut local_18e: u32;
    let mut local_18a: u16;
    let mut local_188: u16;
    let mut local_f6: u32;
    let mut local_62: u32;
    let mut local_5e: u16;
    let mut local_5c: u32;
    let mut local_58: u16;
    let mut local_56: u16;
    let mut local_54: [u8; 82];
    let mut u_var9: u32;

    local_56 = 0;
    u_var13 = (param_3 >> 0x10);
    i_var11 = param_3;
    local_58 = param_3;
    u_var14 = 0;
    local_5c = 0;
    uVar16 = param_1;
    uVar17 = (param_1 >> 0x10);
    pass1_1010_c320(uVar16, uVar17, 0, (i_var11 + 6));
    copy_string_1000_3d3e(CONCAT22(unaff_ss, local_54), CONCAT22(ctx.dx_reg, u_var14));
    local_62 = *(i_var11 + 2);
    (local_62 + 4) = param_2;
    pass1_1040_a626(local_62, CONCAT22(unaff_ss, local_54));
    local_5e = 0;
    local_192 = ctx.dx_reg;
    pass1_1000_4906(CONCAT22(unaff_ss, &local_f6), 0, 0x94);
    u_var4 = pass1_1000_4906(CONCAT22(unaff_ss, &local_18a), 0, 0x94);
    u_var9 = CONCAT31(extraout_var, u_var4);
    local_18e = 0;
    local_190 = 1;
    while (local_190 < 0x25) {
        pass1_1030_7c28((i_var11 + 6), local_190);
        u_var6 = u_var9;
        _local_194 = (u_var9 & 0xffff | local_192 << 0x10);
        local_192 = local_192 | u_var6;
        if (local_192 != 0) {
            big_switch_statement_1020_c0d8(local_190);
            _local_198 = CONCAT22(local_192, u_var6);
            u_var10 = local_192 | u_var6;
            if (u_var10 == 0) {
                copy_string_1000_3d3e((&local_f6)[local_18e], s_Null_Ptr_1050_38e1);
            } else {
                pass1_fn_1008_60e8(u_var6, local_192);
                (&local_f6 + local_18e) = u_var6;
                (&local_f6 + local_18e * 4 + 2) = u_var10;
            }
            u_var9 = _local_194 & 0xffff;
            (&local_18a)[local_18e * 2] = local_194;
            (&local_188)[local_18e * 2] = local_192;
            local_18e = local_18e + 1;
        }
        local_190 = local_190 + 1;
    }
    local_5c = local_18e;
    if (0x13 < local_18e) {
        local_5e = 1;
    }
    pu_var5 = &local_f6;
    pass1_1010_db2e(
        uVar16,
        uVar17,
        1,
        CONCAT22(unaff_ss, pu_var5),
        CONCAT22(unaff_ss, &local_18a),
        param_2,
        param_3,
    );
    local_56 = pu_var5;
    while (u_var9 = local_18e - 1, local_18e != 0) {
        local_18e._0_2_ = u_var9;
        local_19c = (&local_f6)[local_18e];
        _local_194 = local_19c;
        local_18e = u_var9;
        error_check_1000_17ce(local_19c);
    }
    uVar15 = 0x1000;
    local_18e = u_var9;
    pass1_1000_4906(CONCAT22(unaff_ss, &local_18a), 0, 0x54);
    u_var3 = (i_var11 + 6);
    u_var14 = (u_var3 >> 0x10);
    iVar12 = u_var3;
    _local_194 = (iVar12 + 0x1e);
    u_var6 = (iVar12 + 0x20) | local_194;
    u_var9 = u_var6;
    if (u_var6 != 0) {
        local_18e = 0;
        loop {
            u_var3 = _local_194;
            ppc_var2 = (u_var3 + 0x10);
            ppc_var2(uVar15, _local_194, (_local_194 >> 0x10));
            if ((ctx.dx_reg < local_18e._2_2_)
                || (ctx.dx_reg <= local_18e._2_2_ && (u_var9 <= local_18e)))
            {
                break;
            }
            ppc_var2 = (u_var3 + 4);
            ppc_var2(uVar15, _local_194, local_18e, local_18e._2_2_);
            u_var6 = ctx.dx_reg | u_var9;
            if (u_var6 != 0) {
                uVar15 = SUB42(&PTR_LOOP_1050_1028, 0);
                pa_var7 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, u_var9, ctx.dx_reg);
                _local_198 = CONCAT22(u_var6, pa_var7);
                if ((u_var6 | pa_var7) == 0) {
                    return;
                }
                iVar12 = &pa_var7.field_0xc;
                u_var9 = (iVar12 - 1);
                if (((0 < iVar12) && (!SBORROW2(iVar12, 1)))
                    && (u_var9 = (iVar12 - 6), iVar12 - 6 == 0 || (iVar12 - 1) < 5))
                {
                    plVar1 = (&local_18a + iVar12 * 2);
                    *plVar1 = *plVar1 + 1;
                }
            }
            local_18e = local_18e + 1;
        }
        u_var6 = ctx.dx_reg;
        pass1_1000_4906(CONCAT22(unaff_ss, &local_f6), 0, 0x54);
        pass1_1000_4906(CONCAT22(unaff_ss, &local_1f0), 0, 0x54);
        local_18e = 0;
        local_190 = 1;
        while (local_190 < 0x15) {
            if ((&local_18a + local_190 * 2) != 0) {
                u_var8 = big_switch_statement_1020_c222(local_190);
                u_var10 = u_var6 | u_var8;
                if (u_var10 == 0) {
                    copy_string_1000_3d3e((&local_f6)[local_18e], s_Null_Ptr_1050_38ea);
                } else {
                    pass1_fn_1008_60e8(u_var8, u_var6);
                    (&local_f6 + local_18e) = u_var8;
                    (&local_f6 + local_18e * 4 + 2) = u_var10;
                }
                u_var6 = (&local_188)[local_190 * 2];
                (&local_1f0)[local_18e * 2] = (&local_18a)[local_190 * 2];
                (&local_1ee)[local_18e * 2] = u_var6;
                local_18e = local_18e + 1;
            }
            local_190 = local_190 + 1;
        }
        if (local_5e == 0) {
            iVar12 = (local_5c >> 0x10) + CARRY2(local_5c, local_18e);
            local_5c = CONCAT22(iVar12, local_5c + local_18e);
            if ((iVar12 != 0) || (0x13 < local_5c + local_18e)) {
                local_5e = 1;
            }
        }
        if ((local_56 < local_58 - 2) && (_local_1f0 != 0)) {
            iVar12 = pass1_1010_dcac(uVar16, uVar17, local_56, param_2, param_3);
            pu_var5 = &local_f6;
            local_56 = iVar12 + 1;
            pass1_1010_db2e(
                uVar16,
                uVar17,
                local_56,
                CONCAT22(unaff_ss, pu_var5),
                CONCAT22(unaff_ss, &local_1f0),
                param_2,
                param_3,
            );
            local_56 = pu_var5;
        }
        while (u_var9 = local_18e - 1, local_18e != 0) {
            local_18e._0_2_ = u_var9;
            local_19c = (&local_f6)[local_18e];
            local_18e = u_var9;
            error_check_1000_17ce(local_19c);
        }
        if (local_5e != 0) {
            (i_var11 + 0x16) = 1;
        }
        local_18e = u_var9;
        pass1_1010_dc36(uVar16, uVar17, local_56, param_2, param_3);
    }
    return;
}

// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn pass1_1010_cc56(param_1: u32, param_2: u32, param_3: u32) {
    let plVar1: *mut long;
    let paVar2: *mut Struct493;
    let pu_var3: *mut u32;
    let mut u_var4: u16;
    let mut i_var5: i32;
    let lVar6: u32;
    let mut in_dx: i32;
    let mut u_var7: i32;
    
    let mut u_var8: i32;
    let mut u_var9: u16;
    let mut u_var10: u16;
    
    let mut local_1a0: u16;
    let mut local_19e: u16;
    let mut local_14c: u16;
    let mut local_14a: u16;
    let mut local_148: u16;
    let mut local_146: u16;
    let mut local_144: u16;
    let mut local_142: u16;
    let mut local_140: u16;
    let mut local_13e: u16;
    let mut local_13c: u16;
    let mut local_aa: u32;
    let mut local_16: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;
    let mut temp_5f52790f98: u32;

    u_var10 = (param_1 >> 0x10);
    u_var9 = param_1;
    temp_5f52790f98 = (u_var9 + 0x138);
    local_6 = (temp_5f52790f98 + 0x24);
    paVar2 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, local_6, (local_6 >> 0x10));
    _local_a = CONCAT22(in_dx, paVar2);
    local_142 = in_dx | paVar2;
    if (local_142 != 0) {
        local_e = param_3;
        local_12 = 0;
        pass1_1000_4906(CONCAT22(ctx.stack_seg_reg, &local_aa), 0, 0x94);
        pass1_1000_4906(CONCAT22(ctx.stack_seg_reg, &local_13e), 0, 0x94);
        local_c = 0;
        local_10 = 0;
        local_16 = 0;
        local_140 = 1;
        while (local_140 < 0x25) {
            lVar6 = (_local_a + 0x26 + local_140 * 4);
            _local_144 = lVar6;
            if (lVar6 != 0) {
                big_switch_statement_1020_c0d8(local_140);
                u_var8 = lVar6;
                _local_14c = _local_14c & 0xffff | lVar6 << 0x10;
                u_var7 = local_142 | u_var8;
                local_148 = local_142;
                if (u_var7 == 0) {
                    copy_string_1000_3d3e((&local_aa)[local_16], s_Null_Ptr_1050_38f3);
                } else {
                    pass1_fn_1008_60e8();
                    (&local_aa + local_16) = u_var8;
                    (&local_aa + local_16 * 4 + 2) = u_var7;
                }
                (&local_13e)[local_16 * 2] = local_144;
                (&local_13c)[local_16 * 2] = local_142;
                local_16 = local_16 + 1;
            }
            local_140 = local_140 + 1;
        }
        local_10 = local_16;
        if (0x13 < local_16) {
            local_12 = 1;
        }
        pu_var3 = &local_aa;
        pass1_1010_db2e(
            u_var9,
            u_var10,
            1,
            CONCAT22(ctx.stack_seg_reg, pu_var3),
            CONCAT22(ctx.stack_seg_reg, &local_13e),
            param_2,
            param_3,
        );
        u_var8 = ctx.dx_reg;
        local_c = pu_var3;
        pass1_1000_4906(CONCAT22(ctx.stack_seg_reg, &local_13e), 0, 0x54);
        local_14c = 1;
        while (local_14c < 0x15) {
            local_146 = local_14c;
            if ((_local_a + 0x14e + local_14c * 4) != 0) {
                if (((0 < local_14c) && (!SBORROW2(local_14c, 1))) && ((local_14c - 1) < 6)) {
                    plVar1 = (&local_13e + local_14c * 2);
                    *plVar1 = *plVar1 + 1;
                }
            }
            local_14c = local_14c + 1;
        }
        pass1_1000_4906(CONCAT22(ctx.stack_seg_reg, &local_aa), 0, 0x54);
        pass1_1000_4906(CONCAT22(ctx.stack_seg_reg, &local_1a0), 0, 0x54);
        _local_14c = 0x10000;
        while (local_14a < 0x15) {
            if ((&local_13e + local_14a * 2) != 0) {
                u_var4 = big_switch_statement_1020_c222(local_14a);
                _local_144 = CONCAT22(u_var8, u_var4);
                u_var8 = u_var8 | u_var4;
                if (u_var8 == 0) {
                    copy_string_1000_3d3e((&local_aa)[local_14c], s_Null_Ptr_1050_38fc);
                } else {
                    pass1_fn_1008_60e8();
                    (&local_aa + local_14c) = u_var4;
                    (&local_aa + local_14c * 4 + 2) = u_var8;
                }
                u_var8 = (&local_13c)[local_14a * 2];
                (&local_1a0)[local_14c * 2] = (&local_13e)[local_14a * 2];
                (&local_19e)[local_14c * 2] = u_var8;
                _local_14c = _local_14c & 0xffff0000 | (local_14c + 1);
            }
            _local_14c = _local_14c & 0xffff | (local_14a + 1) << 0x10;
        }
        if (local_12 == 0) {
            local_10 = local_10 + local_14c;
            if (0x13 < local_10) {
                local_12 = 1;
            }
        }
        if ((local_c < local_e - 2) && (_local_1a0 != 0)) {
            i_var5 = pass1_1010_dcac(u_var9, u_var10, local_c, param_2, param_3);
            pu_var3 = &local_aa;
            local_c = i_var5 + 1;
            pass1_1010_db2e(
                u_var9,
                u_var10,
                local_c,
                CONCAT22(ctx.stack_seg_reg, pu_var3),
                CONCAT22(ctx.stack_seg_reg, &local_1a0),
                param_2,
                param_3,
            );
            local_c = pu_var3;
        }
        if (local_12 != 0) {
            (param_3 + 0x16) = 1;
        }
        pass1_1010_dc36(u_var9, u_var10, local_c, param_2, param_3);
    }
    return;
}

// WARNING: Could not reconcile some variable overlaps

pub unsafe fn pass1_1010_cf36(param_1: u32, param_2: u32, param_3: u32) {
    let mut u_var1: u32;
    let mut u_var2: i32;
    let mut u_var3: i32;
    let pu_var4: *mut u32;
    let mut u_var5: u32;
    let mut u_var6: u32;
    
    
    let mut u_var7: i32;
    let mut u_var8: i32;
    let mut u_var9: i32;
    let mut iVar10: i32;
    let mut i_var11: i32;
    let mut u_var12: u16;
    let mut u_var13: u16;
    let mut u_var14: u16;
    let mut unaff_ss: u16;
    let mut uVar15: u16;
    let mut uVar16: u16;
    let mut uVar17: u16;
    let mut local_156: u32;
    let mut local_152: u16;
    let mut local_150: u16;
    let mut local_14e: u16;
    let mut local_148: u16;
    let mut local_146: u16;
    let mut local_144: u16;
    let mut local_142: u16;
    let mut local_140: u32;
    let mut local_13c: u16;
    let mut local_13a: u32;
    let mut local_136: u16;
    let mut local_134: u16;
    let mut local_a2: u32;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u32;
    let mut local_4: u16;

    local_4 = 0;
    while {
        u_var12 = (param_2 >> 0x10);
        iVar10 = param_2;
        u_var13 = (param_3 >> 0x10);
        i_var11 = param_3;
        u_var1 = (i_var11 + 2);
        (local_4 * 10 + u_var1 + 4) = (local_4 * 2 + iVar10);
        local_4 = local_4 + 1;
        local_4 < 8
    } {}
    u_var5 = (i_var11 + 2);
    local_4 = 0;
    local_8 = u_var5;
    while {
        uVar15 = param_1;
        uVar16 = (param_1 >> 0x10);
        big_fn_1010_b038(param_1, (i_var11 + 6));
        pass1_1040_a626(local_8, (u_var5 & 0xffff | ctx.dx_reg << 0x10));
        u_var7 = u_var5;
        local_4 = local_4 + 1;
        local_8 = local_8 & 0xffff0000 | (local_8 + 10);
        local_4 < 8
    } {}
    pass1_1010_dd5e(uVar15, uVar16, (i_var11 + 6));
    _local_c = CONCAT22(ctx.dx_reg, u_var7);
    u_var7 = ctx.dx_reg | u_var7;
    if (u_var7 != 0) {
        local_e = 0;
        pass1_1000_4906(CONCAT22(unaff_ss, &local_a2), 0, 0x94);
        pass1_1000_4906(CONCAT22(unaff_ss, &local_136), 0, 0x94);
        local_13a = 0;
        local_13c = 0;
        u_var1 = (i_var11 + 6);
        u_var5 = (u_var1 + 0x26);
        local_146 = 1;
        u_var6 = u_var5;
        while (u_var2 = u_var6, local_146 < 0x25) {
            uVar17 = (u_var5 >> 0x10);
            if ((local_146 * 4 + _local_c) == 0) {
                u_var2 = u_var7;
                if (u_var5 != 0) {
                    u_var6 = pass1_1020_bae6(u_var5, CONCAT22(local_146, uVar17));
                    u_var9 = u_var6;
                    u_var2 = u_var7 | u_var9;
                    if (u_var2 != 0) {
                        if (local_e == 0) {
                            local_e = 1;
                        }
                        u_var3 = u_var9;
                        big_switch_statement_1020_c0d8(local_146);
                        u_var8 = u_var2 | u_var3;
                        if (u_var8 == 0) {
                            copy_string_1000_3d3e(
                                (&local_a2)[local_13a._2_2_],
                                s_Null_Ptr_1050_390e,
                            );
                        } else {
                            pass1_fn_1008_60e8(u_var3, u_var2);
                            (&local_a2 + local_13a._2_2_) = u_var3;
                            (&local_a2 + local_13a._2_2_ * 4 + 2) = u_var8;
                        }
                        (&local_136)[local_13a._2_2_ * 2] = u_var9;
                        (&local_134)[local_13a._2_2_ * 2] = u_var7;
                        local_13a = local_13a & 0xffff | (local_13a._2_2_ + 1) << 0x10;
                        // goto LAB_1010_d11d;
                    }
                }
            } else {
                if (local_e == 0) {
                    local_e = 1;
                }
                big_switch_statement_1020_c0d8(local_146);
                u_var9 = u_var7 | u_var2;
                if (u_var9 == 0) {
                    copy_string_1000_3d3e((&local_a2)[local_13a._2_2_], s_Null_Ptr_1050_3905);
                } else {
                    pass1_fn_1008_60e8(u_var2, u_var7);
                    (&local_a2 + local_13a._2_2_) = u_var2;
                    (&local_a2 + local_13a._2_2_ * 4 + 2) = u_var9;
                }
                u_var14 = (_local_c >> 0x10);
                u_var7 = (local_146 * 4 + _local_c + 2);
                (&local_136)[local_13a._2_2_ * 2] = (local_146 * 4 + _local_c);
                (&local_134)[local_13a._2_2_ * 2] = u_var7;
                local_13a = local_13a & 0xffff | (local_13a._2_2_ + 1) << 0x10;
                if (u_var5 == 0) {
                    u_var2 = 0;
                } else {
                    u_var6 = pass1_1020_bae6(u_var5, CONCAT22(local_146, uVar17));
                    u_var2 = u_var6;
                }
                u_var6 = u_var2;
                if (u_var2 == 0) {
                    local_13c = local_13c + 2;
                    u_var2 = u_var7;
                } else {
                    // LAB_1010_d11d:
                    (uVar15 + local_13a * 2 + 0xa4) = (iVar10 + local_13c * 2 + 0x10);
                    (uVar15 + (local_13a + 1) * 2 + 0xa4) = (iVar10 + (local_13c + 1) * 2 + 0x10);
                    local_13c = local_13c + 2;
                    u_var6 = (local_13a + 2);
                    local_13a = local_13a & 0xffff0000 | (local_13a + 2);
                    u_var2 = u_var7;
                }
            }
            local_146 = local_146 + 1;
            u_var7 = u_var2;
        }
        pu_var4 = &local_a2;
        pass1_1010_db2e(
            uVar15,
            uVar16,
            8,
            CONCAT22(unaff_ss, pu_var4),
            CONCAT13((unaff_ss >> 8), CONCAT12(unaff_ss, &local_136)),
            param_2,
            param_3,
        );
        if (local_e != 0) {
            (i_var11 + 0x16) = 1;
        }
        while (
            local_13a = (local_13a._2_2_ - 1) << 0x10,
            local_13a._2_2_ != 0,
        ) {
            error_check_1000_17ce((&local_a2)[local_13a._2_2_ - 1]);
        }
        pass1_1010_dc36(uVar15, uVar16, pu_var4, param_2, param_3);
    }
    return;
}

// WARNING: Could not reconcile some variable overlaps

pub unsafe fn pass1_1010_d24a(param_1: u32, param_2: u32, param_3: u32) {
    let mut u_var1: u32;
    let u_var2: u8;
    let mut u_var3: i32;
    let pu_var4: *mut u32;
    let extraout_var: u32;
    
    
    
    let mut u_var6: i32;
    let mut u_var7: i32;
    let mut u_var8: i32;
    let mut i_var9: i32;
    let mut u_var10: u16;
    let mut unaff_ss: u16;
    let mut u_var11: u16;
    let mut u_var12: u16;
    let mut local_14e: u32;
    let mut local_14a: u16;
    let mut local_148: u16;
    let mut local_146: u16;
    let mut local_144: u16;
    let mut local_142: u16;
    let mut local_140: u16;
    let mut local_13e: u32;
    let mut local_13a: u16;
    let mut local_138: u16;
    let mut local_a6: u32;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u32;
    let mut local_4: u16;
    let mut u_var5: u32;

    local_4 = 0;
    while {
        u_var10 = (param_3 >> 0x10);
        i_var9 = param_3;
        u_var1 = (i_var9 + 2);
        (local_4 * 10 + u_var1 + 4) = (local_4 * 2 + param_2);
        local_4 = local_4 + 1;
        local_4 < 8
    } {}
    u_var5 = (i_var9 + 2);
    local_4 = 0;
    local_8 = u_var5;
    while {
        u_var11 = param_1;
        u_var12 = (param_1 >> 0x10);
        big_fn_1010_b038(param_1, (i_var9 + 6));
        pass1_1040_a626(local_8, (u_var5 & 0xffff | ctx.dx_reg << 0x10));
        u_var6 = u_var5;
        local_4 = local_4 + 1;
        local_8 = local_8 & 0xffff0000 | (local_8 + 10);
        local_4 < 8
    } {}
    pass1_1010_dd5e(u_var11, u_var12, (i_var9 + 6));
    if ((ctx.dx_reg | u_var6) != 0) {
        local_c = u_var6;
        local_a = ctx.dx_reg;
        pass1_1010_e8f6(u_var11, u_var12, (i_var9 + 6));
        _local_10 = CONCAT22(ctx.dx_reg, u_var6);
        local_12 = 0;
        u_var6 = ctx.dx_reg;
        pass1_1000_4906(CONCAT22(unaff_ss, &local_a6), 0, 0x94);
        u_var2 = pass1_1000_4906(CONCAT22(unaff_ss, &local_13a), 0, 0x94);
        u_var5 = CONCAT31(extraout_var, u_var2);
        local_13e = 0;
        local_140 = 1;
        while (local_140 < 0x25) {
            pass1_1030_7c28(_local_10, local_140);
            u_var3 = u_var5;
            u_var7 = u_var6 | u_var3;
            if (u_var7 != 0) {
                if (local_12 == 0) {
                    local_12 = 1;
                }
                big_switch_statement_1020_c0d8(local_140);
                u_var8 = u_var7 | u_var3;
                if (u_var8 == 0) {
                    copy_string_1000_3d3e((&local_a6)[local_13e], s_Null_Ptr_1050_3917);
                } else {
                    pass1_fn_1008_60e8(u_var3, u_var7);
                    (&local_a6 + local_13e) = u_var3;
                    (&local_a6 + local_13e * 4 + 2) = u_var8;
                }
                (&local_13a)[local_13e * 2] = (u_var5 & 0xffff);
                (&local_138)[local_13e * 2] = u_var6;
                local_13e = local_13e + 1;
                u_var5 = u_var5 & 0xffff;
                u_var7 = u_var6;
            }
            local_140 = local_140 + 1;
            u_var6 = u_var7;
        }
        pu_var4 = &local_a6;
        pass1_1010_db2e(
            u_var11,
            u_var12,
            8,
            CONCAT22(unaff_ss, pu_var4),
            CONCAT13((unaff_ss >> 8), CONCAT12(unaff_ss, &local_13a)),
            param_2,
            param_3,
        );
        if (local_12 != 0) {
            (i_var9 + 0x16) = 1;
        }
        while (local_13e != 0) {
            local_13e._0_2_ = (local_13e - 1);
            error_check_1000_17ce((&local_a6)[local_13e]);
            local_13e = local_13e - 1;
        }
        pass1_1010_dc36(u_var11, u_var12, pu_var4, param_2, param_3);
    }
    return;
}

// WARNING: Could not reconcile some variable overlaps

pub unsafe fn pass1_1010_d448(param_1: u32, param_2: u32, param_3: *mut Struct481) {
    let pu_var1: *mut u16;
    let pu_var2: *mut u8;
    let ctx.ax_reg: *mut Struct480;
    let pu_var3: *mut u16;
    let string_offset_b: *mut libc::c_char;
    let mut i_var4: i32;
    let mut u_var5: i32;
    let string_ptr_base_b: *mut libc::c_char;
    let struct_a_2: *mut Struct481;
    let struct_a_1: *mut Struct481;
    let string_ptr_base_a: *mut libc::c_char;
    let mut u_var6: u32;
    let mut u_var7: u16;
    let mut local_414: u16;
    let mut local_412: u32;
    let mut local_40e: u16;
    let mut local_40c: u16;
    let mut local_40a: u32;
    let mut local_406: u16;
    let mut local_404: u16;
    let mut string_offset_a: [u8; 1024];
    let temp_7ffb9c6ecdb: *mut Struct1176;
    // temp_7e00f6c84af: *mut *mut u8;

    struct_a_1 = (param_3 >> 0x10);
    struct_a_2 = param_3;
    _local_406 = pass1_1030_73a8(struct_a_2.field_0x6);
    u_var5 = (_local_406 >> 0x10);
    temp_7ffb9c6ecdb = _local_406;
    if ((u_var5 | temp_7ffb9c6ecdb) != 0) {
        local_40a = &temp_7ffb9c6ecdb.field_0x20;
        u_var5 = &temp_7ffb9c6ecdb.field_0x22;
        if ((u_var5 | local_40a) != 0) {
            local_40c = 0;
            pu_var3 = &local_40c;
            u_var7 = (param_1 >> 0x10);
            pass1_1010_d984(
                param_1,
                u_var7,
                CONCAT22(string_ptr_base_a, pu_var3),
                3,
                local_40a & 0xffff | u_var5 << 0x10,
                &PTR_DAT_1050_1805_1050_368e,
                param_3,
            );
            string_offset_b = PTR_DAT_1050_1805_1050_368e;
            pu_var1 = struct_a_2.field_0x2;
            (pu_var1 + 4) = PTR_DAT_1050_1805_1050_368e;
            big_fn_1010_b038(param_1, struct_a_2.field_0x6);
            copy_string_1000_3d3e(
                CONCAT22(string_ptr_base_a, string_offset_a),
                CONCAT22(string_ptr_base_b, string_offset_b),
            );
            pass1_1040_a626(pu_var1, CONCAT22(string_ptr_base_a, string_offset_a));
            pu_var2 = struct_a_2.field_0x2;
            i_var4 = pu_var2;
            (i_var4 + 0xe) = PTR_LAB_1050_1821_1_1050_3690;
            string_fn_1000_3f9c(
                string_offset_a,
                string_ptr_base_a,
                s__u_1050_3920,
                &ctx.g_alloc_addr_1050_1050,
                local_40c,
            );
            pass1_1040_a626(
                (pu_var2 & 0xffff0000 | (i_var4 + 10)),
                CONCAT22(string_ptr_base_a, string_offset_a),
            );
            pu_var2 = struct_a_2.field_0x2;
            i_var4 = pu_var2;
            (i_var4 + 0x18) = PTR_LAB_1050_1823_1050_3692;
            u_var6 = pass1_1028_62c8(_local_406);
            string_fn_1000_3f9c(
                string_offset_a,
                string_ptr_base_a,
                s__u_1050_3923,
                &ctx.g_alloc_addr_1050_1050,
                u_var6,
            );
            pass1_1040_a626(
                (pu_var2 & 0xffff0000 | (i_var4 + 0x14)),
                CONCAT22(string_ptr_base_a, string_offset_a),
            );
            pass1_1010_dc36(param_1, u_var7, pu_var3, param_2, param_3);
        }
    }
    return;
}

// WARNING: Could not reconcile some variable overlaps

pub unsafe fn pass1_1010_d5ae(param_1: u32, param_2: u32, param_3: *mut Struct482) {
    let pu_var1: *mut u16;
    let mut u_var2: u32;
    let pu_var3: *mut u16;
    let local_AX_145: *mut u8;
    let local_AX_192: *mut Struct484;
    let local_AX_261: *mut Struct483;
    let mut u_var4: i32;
    
    let mut local_DX_145: u16;
    let local_bx_4: *mut Struct482;
    let mut local_es_4: u16;
    
    let mut u_var5: u32;
    let mut u_var6: u16;
    let mut u_var7: u16;
    let mut local_414: u16;
    let mut local_412: u32;
    let mut local_40e: u16;
    let mut local_40c: u16;
    let mut local_40a: u16;
    let mut local_408: u16;
    let mut local_406: u16;
    let mut local_404: u16;
    let mut local_402: [u8; 1024];
    let temp_5f9bbbd715: *mut Struct483;
    let mut temp_5f3955c6bf: u32;

    local_es_4 = (param_3 >> 0x10);
    local_bx_4 = param_3;
    _local_406 = pass1_1030_73a8(local_bx_4.field_0x6);
    u_var4 = (_local_406 >> 0x10);
    local_40a = _local_406;
    if ((u_var4 | local_40a) != 0) {
        pass1_1028_45fe(local_40a, u_var4);
        if ((ctx.dx_reg | local_40a) != 0) {
            local_40c = 0;
            pu_var3 = &local_40c;
            u_var6 = param_1;
            u_var7 = (param_1 >> 0x10);
            local_408 = ctx.dx_reg;
            pass1_1010_d984(
                u_var6,
                u_var7,
                CONCAT22(ctx.stack_seg_reg, pu_var3),
                3,
                CONCAT22(ctx.dx_reg, local_40a),
                &PTR_DAT_1050_1805_1050_3706,
                param_3,
            );
            local_AX_145 = PTR_DAT_1050_1805_1050_3706;
            pu_var1 = &local_bx_4.field_0x2;
            (pu_var1 + 4) = PTR_DAT_1050_1805_1050_3706;
            big_fn_1010_b038(u_var6, (param_1 >> 0x10), local_bx_4.field_0x6);
            copy_string_1000_3d3e(
                CONCAT22(ctx.stack_seg_reg, local_402),
                CONCAT22(local_DX_145, local_AX_145),
            );
            pass1_1040_a626(pu_var1, CONCAT22(ctx.stack_seg_reg, local_402));
            u_var2 = &local_bx_4.field_0x2;
            local_AX_192 = u_var2;
            local_AX_192 = &local_AX_192.field_0xa;
            local_AX_192.field_0xe = PTR_LAB_1050_1821_1_1050_3708;
            string_fn_1000_3f9c(
                local_402,
                ctx.stack_seg_reg,
                s__u_1050_3926,
                &ctx.g_alloc_addr_1050_1050,
                local_40c,
            );
            pass1_1040_a626(
                (u_var2 & 0xffff0000 | ZEXT24(local_AX_192)),
                CONCAT22(ctx.stack_seg_reg, local_402),
            );
            temp_5f9bbbd715 = &local_bx_4.field_0x2;
            local_AX_261 = temp_5f9bbbd715;
            local_AX_261 = &local_AX_261.field_0x14;
            local_AX_261.field_0x18 = PTR_LAB_1050_1823_1050_370a;
            u_var5 = pass1_1028_45e2(_local_406);
            string_fn_1000_3f9c(
                local_402,
                ctx.stack_seg_reg,
                s__u_1050_3929,
                &ctx.g_alloc_addr_1050_1050,
                u_var5,
            );
            pass1_1040_a626(
                (temp_5f9bbbd715 & 0xffff0000 | ZEXT24(local_AX_261)),
                CONCAT22(ctx.stack_seg_reg, local_402),
            );
            pass1_1010_dc36(u_var6, u_var7, pu_var3, param_2, param_3);
        }
    }
    return;
}

// WARNING: Could not reconcile some variable overlaps

pub unsafe fn pass1_1010_d710(param_1: u32, param_2: *mut u8, param_3: *mut Struct485) {
    let mut u_var1: u32;
    let mut u_var2: i32;
    let pu_var3: *mut u32;
    let mut u_var4: u32;
    let mut u_var5: u32;
    
    
    let mut u_var6: i32;
    let mut u_var7: i32;
    let local_SI_15: *mut u8;
    let struct_a_2: *mut Struct485;
    let local_es_15: *mut u8;
    let struct_a_1: *mut Struct485;
    let mut u_var8: u16;
    
    let mut u_var9: u16;
    let mut u_var10: u16;
    let mut local_152: u32;
    let mut local_14e: u16;
    let mut local_14c: u16;
    let mut local_14a: u16;
    let mut local_144: u16;
    let mut local_142: u16;
    let mut local_140: u32;
    let mut local_13c: u16;
    let mut local_13a: u32;
    let mut local_136: u16;
    let mut local_134: u16;
    let mut local_a2: u32;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u32;
    let local_4: *mut u8;
    let mut temp_5f71e254c5: u32;
    let mut temp_5fa9a51261: u32;

    local_4 = 0x0;
    while {
        local_es_15 = (param_2 >> 0x10);
        local_SI_15 = param_2;
        struct_a_1 = (param_3 >> 0x10);
        struct_a_2 = param_3;
        temp_5fa9a51261 = struct_a_2.field_0x2;
        (local_4 * 10 + temp_5fa9a51261 + 4) = (local_SI_15 + local_4 * 2);
        local_4 = local_4 + 1;
        local_4 < 4
    } {}
    u_var4 = struct_a_2.field_0x2;
    local_4 = 0x0;
    local_8 = u_var4;
    while {
        u_var9 = param_1;
        u_var10 = (param_1 >> 0x10);
        big_fn_1010_b038(param_1, struct_a_2.field_0x6);
        pass1_1040_a626(local_8, (u_var4 & 0xffff | ctx.dx_reg << 0x10));
        u_var6 = u_var4;
        local_4 = local_4 + 1;
        local_8 = local_8 & 0xffff0000 | (local_8 + 10);
        local_4 < 4
    } {}
    pass1_1010_dd5e(u_var9, u_var10, struct_a_2.field_0x6);
    _local_c = CONCAT22(ctx.dx_reg, u_var6);
    u_var6 = ctx.dx_reg | u_var6;
    if (u_var6 != 0) {
        local_e = 0;
        pass1_1000_4906(CONCAT22(ctx.stack_seg_reg, &local_a2), 0, 0x94);
        pass1_1000_4906(CONCAT22(ctx.stack_seg_reg, &local_136), 0, 0x94);
        local_13a = 0;
        local_13c = 0;
        temp_5f71e254c5 = struct_a_2.field_0x6;
        u_var4 = (temp_5f71e254c5 + 0x26);
        local_142 = 1;
        u_var5 = u_var4;
        while (u_var2 = u_var5, local_142 < 0x25) {
            if ((local_142 * 4 + _local_c) != 0) {
                if (local_e == 0) {
                    local_e = 1;
                }
                big_switch_statement_1020_c0d8(local_142);
                u_var7 = u_var6 | u_var2;
                if (u_var7 == 0) {
                    copy_string_1000_3d3e((&local_a2)[local_13a._2_2_], s_Null_Ptr_1050_392c);
                } else {
                    pass1_fn_1008_60e8(u_var2, u_var6);
                    (&local_a2 + local_13a._2_2_) = u_var2;
                    (&local_a2 + local_13a._2_2_ * 4 + 2) = u_var7;
                }
                u_var8 = (_local_c >> 0x10);
                u_var6 = (local_142 * 4 + _local_c + 2);
                (&local_136)[local_13a._2_2_ * 2] = (local_142 * 4 + _local_c);
                (&local_134)[local_13a._2_2_ * 2] = u_var6;
                u_var1 = local_13a & 0xffff;
                local_13a = u_var1 | (local_13a._2_2_ + 1) << 0x10;
                if (u_var4 == 0) {
                    u_var2 = 0;
                } else {
                    u_var5 = pass1_1020_bae6(u_var4, CONCAT22(local_142, (u_var4 >> 0x10)));
                    u_var2 = u_var5;
                }
                u_var5 = u_var2;
                if (u_var2 == 0) {
                    local_13c = local_13c + 2;
                } else {
                    local_13a._0_2_ = u_var1;
                    (u_var9 + local_13a * 2 + 0xa4) = (local_SI_15 + local_13c * 2 + 8);
                    (u_var9 + (local_13a + 1) * 2 + 0xa4) = (local_SI_15 + (local_13c + 1) * 2 + 8);
                    local_13c = local_13c + 2;
                    u_var5 = (local_13a + 2);
                    local_13a = CONCAT22(local_13a._2_2_ + 1, local_13a + 2);
                }
            }
            local_142 = local_142 + 1;
        }
        pu_var3 = &local_a2;
        pass1_1010_db2e(
            u_var9,
            u_var10,
            4,
            CONCAT22(ctx.stack_seg_reg, pu_var3),
            CONCAT13((ctx.stack_seg_reg >> 8), CONCAT12(ctx.stack_seg_reg, &local_136)),
            param_2,
            param_3,
        );
        if (local_e != 0) {
            struct_a_2.field_0x16 = 1;
        }
        while (
            local_13a = (local_13a._2_2_ - 1) << 0x10,
            local_13a._2_2_ != 0,
        ) {
            error_check_1000_17ce((&local_a2)[local_13a._2_2_ - 1]);
        }
        pass1_1010_dc36(u_var9, u_var10, pu_var3, param_2, param_3);
    }
    return;
}

// WARNING: Could not reconcile some variable overlaps

pub unsafe fn pass1_1010_d984(
    param_1: u16,
    param_2: u16,
    param_1_00: *mut i32,
    param_4: u16,
    param_5: u32,
    param_6: u32,
    param_7: u32,
) {
    let mut u_var1: u16;
    let local_AX_30: *mut u8;
    let mut u_var2: u16;
    let mut i_var3: i32;
    
    let mut u_var4: i32;
    let local_bx_142: *mut Struct486;
    let mut i_var5: i32;
    let local_es_142: *mut u8;
    let local_es_179: *mut u8;
    let string_c: *mut libc::c_char;
    let mut local_41c: u16;
    let mut local_41a: u16;
    let mut string_a: [u8; 1024];
    let mut local_18: u16;
    let mut local_16: u32;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut string_b: [u8; 8];
    let mut local_4: u16;
    let temp_5fddd85d53: *mut u8;
    let temp_5f0cc5ef4a: *mut u8;

    local_4 = param_2_00;
    pass1_1008_5784(CONCAT22(string_c, string_b), param_5);
    loop {
        local_AX_30 = string_b;
        pass1_1008_5b12(CONCAT22(string_c, local_AX_30));
        _local_10 = CONCAT22(ctx.dx_reg, local_AX_30);
        u_var4 = ctx.dx_reg | local_AX_30;
        if (u_var4 == 0) {
            return;
        }
        local_12 = (local_AX_30 + 10);
        u_var2 = 0;
        local_16 = 0;
        if ((local_AX_30 + 4) == 0) {
            if ((local_AX_30 + 6) == 0) {
                if ((local_AX_30 + 8) == 0) {
                    return;
                }
                u_var2 = big_switch_statement_1020_c2f8(*(local_AX_30 + 8));
            } else {
                u_var2 = big_switch_statement_1020_c222((local_AX_30 + 6));
            }
        } else {
            big_switch_statement_1020_c0d8((local_AX_30 + 4));
        }
        local_16 = CONCAT22(u_var4, u_var2);
        local_18 = (_local_10 + 0xc);
        *param_1_00 = *param_1_00 + local_18;
        local_es_142 = (param_7 >> 0x10);
        local_bx_142 = param_7;
        temp_5fddd85d53 = local_bx_142.field_0x4;
        i_var3 = local_bx_142.field_0x2 + local_4 * 10;
        _local_41c = CONCAT22(temp_5fddd85d53, i_var3);
        local_es_179 = (param_6 >> 0x10);
        i_var5 = param_6;
        (i_var3 + 4) = (local_4 * 2 + i_var5);
        string_fn_1000_3f9c(
            string_a,
            string_c,
            s__u_1050_3935,
            &ctx.g_alloc_addr_1050_1050,
            local_12,
        );
        pass1_1040_a626(_local_41c, CONCAT22(string_c, string_a));
        temp_5f0cc5ef4a = local_bx_142.field_0x4;
        i_var3 = local_bx_142.field_0x2 + (local_4 + 1) * 10;
        _local_41c = CONCAT22(temp_5f0cc5ef4a, i_var3);
        (i_var3 + 4) = ((local_4 + 1) * 2 + i_var5);
        copy_string_1000_3d3e(CONCAT22(string_c, string_a), local_16);
        pass1_1040_a626(_local_41c, CONCAT22(string_c, string_a));
        i_var3 = (local_4 + 2) * 10 + local_bx_142.field_0x2;
        u_var1 = local_bx_142.field_0x4;
        _local_41c = CONCAT22(u_var1, i_var3);
        (i_var3 + 4) = ((local_4 + 2) * 2 + i_var5);
        local_4 = local_4 + 3;
        string_fn_1000_3f9c(
            string_a,
            string_c,
            s__u_1050_3938,
            &ctx.g_alloc_addr_1050_1050,
            local_18,
        );
        pass1_1040_a626(_local_41c, CONCAT22(string_c, string_a));
    }
}

// WARNING: Could not reconcile some variable overlaps

pub unsafe fn pass1_1010_db2e(
    param_1: u16,
    param_2: u16,
    param_1_00: *mut Struct487,
    param_2_00: u32,
    param_5: u32,
    param_6: u32,
    param_7: *mut i32,
) -> *mut Struct187 {
    let mut u_var1: u16;
    let mut i_var2: i32;
    let mut i_var3: i32;
    let mut u_var4: u16;
    let mut u_var5: u16;
    let mut u_var6: u16;
    let unaff_ss: *mut libc::c_char;
    let local_5e: *mut Struct487;
    let local_5c: *mut Struct489;
    let local_5a: *mut Struct487;
    let mut local_58: u16;
    let mut local_56: u16;
    let mut local_54: [u8; 82];

    local_5e = param_1_00;
    local_5a = param_1_00;
    local_5c = 0x0;
    loop {
        u_var6 = (param_7 >> 0x10);
        i_var3 = param_7;
        if ((*param_7 - 1) < local_5e) {
            return local_5e;
        }
        u_var1 = (i_var3 + 4);
        i_var2 = (i_var3 + 2) + local_5e * 10;
        u_var4 = (param_5 >> 0x10);
        u_var5 = (param_2_00 >> 0x10);
        if (((local_5c * 4 + param_5) == 0) && ((local_5c * 4 + param_2_00) == 0)) {
            break;
        }
        copy_string_1000_3d3e(CONCAT22(unaff_ss, local_54), *(local_5c * 4 + param_2_00));
        u_var5 = (param_6 >> 0x10);
        (i_var2 + 4) = (local_5a * 2 + param_6);
        pass1_1040_a626(CONCAT22(u_var1, i_var2), CONCAT22(unaff_ss, local_54));
        string_fn_1000_3f9c(
            local_54,
            unaff_ss,
            s__lu_1050_393b,
            &ctx.g_alloc_addr_1050_1050,
            (param_5 + local_5c * 4),
        );
        u_var1 = (i_var3 + 4);
        i_var3 = (i_var3 + 2) + &local_5e.field_0x1 * 10;
        _local_58 = CONCAT22(u_var1, i_var3);
        (i_var3 + 4) = (&local_5a.field_0x1 * 2 + param_6);
        pass1_1040_a626(_local_58, CONCAT22(unaff_ss, local_54));
        local_5e = &local_5e.field_0x2;
        local_5a = &local_5a.field_0x2;
        local_5c = &local_5c.field_0x1;
    }
    return local_5e;
}

pub unsafe fn pass1_1010_dc36(
    param_1: u16,
    param_2: u16,
    param_3: u16,
    param_2_00: u32,
    param_5: *mut u32,
) -> i32 {
    let pu_var1: *mut u32;
    let mut u_var2: u32;
    let mut i_var3: i32;
    let mut u_var4: i32;
    let pu_var5: *mut u32;
    let mut u_var6: u16;
    let mut unaff_ss: u16;
    let mut local_5a: u16;
    let mut local_58: u32;
    let string_1: *mut libc::c_char;
    let local_52: [u32; 20];

    string_1 = PTR_s_New_failed_in_Op_Op_1050_0020_1050_393f;
    i_var3 = 0x13;
    pu_var5 = local_52;
    while (i_var3 != 0) {
        i_var3 = i_var3 + -1;
        pu_var1 = pu_var5;
        pu_var5 = pu_var5 + 1;
        *pu_var1 = 0;
    }
    pu_var5 = 0;
    *(pu_var5 + 2) = 0;
    local_5a = param_1_00;
    loop {
        u_var6 = (param_5 >> 0x10);
        if (*param_5 < local_5a || *param_5 == local_5a) {
            break;
        }
        u_var2 = (param_5 + 2);
        u_var4 = u_var2 + local_5a * 10;
        (u_var4 + 4) = (local_5a * 2 + param_2_00);
        local_5a = local_5a + 1;
        pass1_1040_a626(
            (u_var2 & 0xffff0000 | u_var4),
            CONCAT22(unaff_ss, &string_1),
        );
    }
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn pass1_1010_dcac(
    param_1: u16,
    param_2: u16,
    param_1_00: i32,
    param_2_00: u32,
    param_5: *mut Struct503,
) -> i32 {
    let mut iVar1: i32;
    let mut i_var2: i32;
    let local_CX_130: *mut Struct504;
    let local_bx_25: *mut Struct503;
    let local_es_25: *mut u8;
    let local_es_60: *mut u8;
    let in_string_1: *mut libc::c_char;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;
    let temp_5f6227b039: *mut Struct504;
    let temp_5fedbb8a6a: *mut u8;

    in_string_1 = load_string_1010_847e(
        ctx._g_struct_73_1050_14cc,
        (ctx._g_struct_73_1050_14cc >> 0x10),
        0x590,
    );
    local_es_25 = (param_5 >> 0x10);
    local_bx_25 = param_5;
    temp_5fedbb8a6a = *&local_bx_25.field_0x4;
    iVar1 = local_bx_25.field_0x2 + param_1_00 * 10;
    local_es_60 = (param_2_00 >> 0x10);
    (iVar1 + 4) = (param_1_00 * 2 + param_2_00);
    pass1_1040_a626(CONCAT22(temp_5fedbb8a6a, iVar1), in_string_1);
    copy_string_1000_3d3e(in_string_1, s__1050_3941);
    i_var2 = param_1_00 + 1;
    temp_5f6227b039 = &local_bx_25.field_0x2;
    local_CX_130 = temp_5f6227b039;
    local_CX_130 = (local_CX_130 + i_var2 * 10);
    local_CX_130.field_0x4 = (i_var2 * 2 + param_2_00);
    pass1_1040_a626(
        (temp_5f6227b039 & 0xffff0000 | ZEXT24(local_CX_130)),
        in_string_1,
    );
    return i_var2;
}

pub unsafe fn pass1_1010_dd5e(param_1: u16, param_2: u16, param_1_00: *mut Struct493) {
    let mut iVar1: i32;
    let mut i_var2: i32;
    let mut u_var3: i32;
    let mut u_var4: u32;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u32;
    let mut local_a: u16;
    let mut local_6: u32;

    if (param_1_00 != 0x0) {
        u_var4 = pass1_1030_73a8(param_1_00);
        u_var3 = (u_var4 >> 0x10);
        i_var2 = u_var4;
        local_10 = u_var4 & 0xffff0000 | (i_var2 + 0x14);
        if ((u_var3 | i_var2 + 0x14) != 0) {
            iVar1 = (i_var2 + 0x12);
            i_var2 = (i_var2 + 0x18);
            if (((((iVar1 == 4)
                || (((iVar1 == 6 && (i_var2 == 4)) || (iVar1 == 5))
                    || (iVar1 == 6 && (i_var2 == 5))))
                || (iVar1 == 8))
                || (iVar1 == 6 && (i_var2 == 8)))
                && (local_10 != 0))
            {
                return;
            }
        }
    }
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn pass1_1010_ddf6(param_1: u32, param_2: u32) {
    let mut u_var1: u32;
    let mut in_resource_id: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    *(param_1 + 0x13c) = 0;
    u_var1 = pass1_1030_73a8(param_2);
    match (u_var1 + 0x12) {
        1 | 2 | 4 | 7 | 9 => in_resource_id = 0x598,
        3 | 5 => in_resource_id = 0x597,
        6 => in_resource_id = 0x599,
        8 => in_resource_id = 0x5f3,
        _ => {} // goto switchD_1010_de53_caseD_9;
    }
    load_string_1010_84e0(
        ctx._g_struct_73_1050_14cc,
        (ctx._g_struct_73_1050_14cc >> 0x10),
        0x3ff,
        (param_1 & 0xffff0000 | (param_1 + 0x13c)),
        in_resource_id,
    );
    // switchD_1010_de53_caseD_9:
    return;
}

pub unsafe fn pass1_1010_debe(
    param_1: u32,
    param_2: u16,
    param_3: u32,
    param_4: u32,
    param_5: u32,
) -> i32 {
    let local_AX_8: *mut Struct490;
    let mut u_var1: u16;
    let mut u_var2: i32;
    let mut u_var3: u16;
    let mut i_var4: i32;
    let pa_var5: *mut Struct199;
    let struct_a: *mut Struct199;
    let mut i32_var6: i32;
    let mut unaff_si: u16;
    let mut u_var7: u16;
    let mut u_var8: u32;
    let pp_var9: *mut pass1_struct_1;
    let mut u_var10: u16;
    let mut local_22: u16;
    let mut local_1e: u16;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u32;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    param_4 = 0;
    param_3 = 0;
    u_var8 = pass1_1030_73a8(param_5);
    i_var4 = (u_var8 + 0x12);
    u_var3 = param_1;
    u_var10 = (param_1 >> 0x10);
    u_var1 = pass1_1010_b028(u_var3, u_var10, u_var8);
    pp_var9 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(unaff_si, 0x35));
    pa_var5 = (pp_var9 >> 0x10);
    i32_var6 = param_4;
    u_var7 = (param_4 >> 0x10);
    if (param_2 == 0x13) {
        local_22 = 0;
        while (local_22 = local_22 + 1, local_22 < 0x43) {
            u_var1 = pass1_1010_ac62(u_var3, u_var10, local_22);
            if (u_var1 != 0) {
                param_3 = param_3 + 1;
            }
        }
        u_var1 = param_3 * 2;
        process_struct_1000_179c(u_var1, pa_var5);
        param_4 = u_var1;
        (i32_var6 + 2) = pa_var5;
        if ((pa_var5 | param_4) != 0) {
            local_22 = 0;
            local_1e = 0;
            while (param_3 != local_1e && local_1e <= param_3) {
                while {
                    local_22 = local_22 + 1;
                    if (0x42 < local_22) {}
                    // goto LAB_1010_e0d4;
                    u_var1 = pass1_1010_ac62(u_var3, u_var10, local_22);
                    u_var1 == 0
                } {}
                (local_1e * 2 + param_4) = local_22;
                // LAB_1010_e0d4:
                local_1e = local_1e + 1;
            }
        }
    } else {
        if (param_2 < 0x14) {
            if (param_2 == '\x06') {
                if (((i_var4 == 5) || (i_var4 == 6)) || (i_var4 == 8)) {
                    u_var2 = pp_var9 + 0x11e;
                    u_var8 = pp_var9 & 0xffff0000 | u_var2;
                    if (u_var1 == 0xf) {
                        local_14 = 0xf;
                        local_16 = 0x13;
                    } else {
                        if (u_var1 == 0xe) {
                            local_16 = 4;
                            local_14 = 1;
                        } else {
                            local_16 = 0xe;
                            local_14 = 1;
                        }
                    }
                    pass1_1010_e128(u_var3, u_var10, local_16, local_14, u_var8);
                    i_var4 = u_var8 + 1;
                    param_3 = i_var4;
                    if (i_var4 != 0) {
                        u_var1 = param_3 * 2;
                        pa_var5 = struct_a;
                        process_struct_1000_179c(u_var1, struct_a);
                        param_4 = u_var1;
                        (i32_var6 + 2) = pa_var5;
                        local_18 = 0;
                        local_1a = local_14;
                        while (local_1a <= local_16) {
                            if ((local_1a * 2 + u_var2) != 0) {
                                (param_4 + local_18 * 2) = local_1a;
                                local_18 = local_18 + 1;
                            }
                            local_1a = local_1a + 1;
                        }
                        (param_4 + local_18 * 2) = 0x14;
                        return;
                    }
                }
            } else {
                if ((param_2 == 0x7) && ((i_var4 == 5 || (i_var4 == 6)) || (i_var4 == 8))) {
                    u_var3 = pass1_1010_ac62(u_var3, u_var10, 7);
                    i_var4 = -(u_var3 == 0) + 0x10;
                    param_3 = i_var4;
                    u_var1 = i_var4 * 2;
                    process_struct_1000_179c(u_var1, pa_var5);
                    param_4 = u_var1;
                    (i32_var6 + 2) = pa_var5;
                    if ((pa_var5 | param_4) == 0) {
                        param_3 = 0;
                        return;
                    }
                    local_1a = 0;
                    while (local_1a < (-(u_var3 == 0) + 0xf)) {
                        (local_1a * 2 + param_4) = local_1a + 1;
                        local_1a = local_1a + 1;
                    }
                    (local_1a * 2 + param_4) = 0x10;
                    return;
                }
            }
        }
    }
    return;
}

pub unsafe fn pass1_1010_e128(
    param_1: u16,
    param_2: u16,
    param_1_00: i32,
    param_4: u16,
    param_5: u32,
) -> i32 {
    let mut local_6: u16;
    let mut local_4: u16;

    local_4 = 0;
    local_6 = param_2_00;
    while (local_6 <= param_1_00) {
        if ((local_6 * 2 + param_5) != 0) {
            local_4 = local_4 + 1;
        }
        local_6 = local_6 + 1;
    }
    return local_4;
}

// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn pass1_1010_e15e(param_1: u32) {
    let pp_var1: fn();
    
    let mut u_var2: u16;
    let paVar3: *mut Struct493;
    let mut u_var4: u32;
    
    
    
    let mut u_var5: u16;
    let mut unaff_cs: u16;
    let mut u_var6: u16;
    let mut u_var7: u16;
    let mut local_12: u32;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;

    pass1_1010_afde(param_1, 0xc);
    _local_a = CONCAT22(ctx.dx_reg, in_ax);
    pp_var1 = (*_local_a + 0x10);
    u_var2 = in_ax;
    u_var6 = in_ax;
    u_var7 = ctx.dx_reg;
    (**pp_var1)();
    _local_e = CONCAT22(ctx.dx_reg, u_var2);
    local_12 = 0;
    while (local_12 < _local_e) {
        pp_var1 = (*_local_a + 4);
        u_var4 = _local_e;
        (**pp_var1)(unaff_cs, in_ax, ctx.dx_reg, local_12, (local_12 >> 0x10));
        u_var5 = ctx.dx_reg;
        paVar3 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, u_var4, ctx.dx_reg);
        unaff_cs = 0x1030;
        pass1_1030_7c28(CONCAT13((u_var5 >> 8), CONCAT12(u_var5, paVar3)), 0x23);
        local_12 = local_12 + 1;
    }
    if (_local_a != 0x0) {
        pp_var1 = *_local_a;
        (**pp_var1)(unaff_cs, in_ax, ctx.dx_reg, 1, u_var6, u_var7);
    }
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn pass1_1010_ec18(param_1: u16, param_2: u16, param_1_00: u32) {
    let mut local_DXAX_13: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    local_DXAX_13._0_2_ =
        pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, param_1_00, (param_1_00 >> 0x10));
    return CONCAT22(&(local_DXAX_13).field_0x12, &(local_DXAX_13).field_0x10);
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn pass1_1010_ec40(param_1: u16, param_2: u16, param_1_00: u32) {
    let mut local_DXAX_13: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    local_DXAX_13._0_2_ =
        pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, param_1_00, (param_1_00 >> 0x10));
    return CONCAT22(&(local_DXAX_13).field_0x12, &(local_DXAX_13).field_0x10);
}

pub unsafe fn pass1_1010_ec68(param_1: *mut Struct318, param_2: u32) {
    let mut u_var1: i32;

    u_var1 = (param_1 >> 0x10);
    (param_1 + 0x28) = param_2;
    pass1_1010_1f62((param_1 & 0xffff | u_var1 << 0x10), 0x13);
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn pass1_1010_ec84(param_1: *mut Struct318) {
    let mut unaff_ss: u16;
    let mut local_10e: u16;
    let mut local_10c: u16;

    pass1_1010_1f62(param_1, 0x14);
    pass1_1030_532e(CONCAT22(unaff_ss, &local_10e), (param_1 + 0x20));
    pass1_1030_835a(ctx._g_bool_1050_5748, CONCAT22(unaff_ss, &local_10e));
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn pass1_1010_ecc6(param_1: u32, param_2: u32, param_3: u32) {
    let mut u_var1: u32;
    let ctx.ax_reg: *mut Struct505;
    let paVar2: *mut Struct493;
    
    let mut u_var3: u16;
    let mut i_var4: i32;
    let mut u_var5: u16;
    let mut local_e: u32;
    let mut local_a: u16;
    let mut local_8: u16;

    pass1_1030_627e(_PTR_LOOP_1050_5740, param_2, param_3);
    u_var3 = ctx.dx_reg;
    paVar2 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, ctx.ax_reg, ctx.dx_reg);
    u_var1 = &paVar2[1].field_0x10;
    u_var5 = (u_var1 >> 0x10);
    i_var4 = u_var1;
    if ((i_var4 + 0x200) == 0x8000001) {
        pass1_1010_ed22(param_1, (i_var4 + 4));
    }
    return;
}

pub unsafe fn pass1_1010_ed22(param_1: u32, param_2: u32) {
    let mut u_var1: i32;

    u_var1 = (param_1 >> 0x10);
    (param_1 + 0x24) = param_2;
    pass1_1010_1f62((param_1 & 0xffff | u_var1 << 0x10), 0x12);
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn pass1_1010_ed3e(param_1: u32) {
    let mut u_var1: u32;

    u_var1 = (param_1 + 0x16);
    pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, u_var1, (u_var1 >> 0x10));
    return;
}

pub unsafe fn pass1_1018_017c(param_1: u32, param_2: u16) {
    let mut u_var1: i32;

    u_var1 = (param_1 >> 0x10);
    (param_1 + 0x1e) = param_2;
    pass1_1010_1f62((param_1 & 0xffff | u_var1 << 0x10), 4);
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn pass1_1018_0196(param_1: *mut Struct507, param_2: u32, param_3: u32) {
    let piVar1: *mut i32;
    let mut i_var2: i32;
    let mut u_var3: u32;
    let ppVar4: *mut pass1_struct_2;
    let in_dx: *mut u16;
    let ctx.dx_reg: *mut u16;
    let local_bx_18: *mut Struct507;
    let mut u_var5: u16;
    let mut local_2c: u16;
    let mut local_2a: u16;
    let mut local_1e: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_e: u16;
    let mut local_c: u16;

    _g_Struct94_ptr_1 = pass1_1030_8344(
        ctx._g_bool_1050_5748,
        (ctx._g_bool_1050_5748 >> 0x10),
        param_3,
    );
    u_var5 = (param_1 >> 0x10);
    local_bx_18 = param_1;
    if (&local_bx_18.field_0x2c == 0) {
        local_bx_18.field_0x32 = 5;
        if (ctx.__g_Struct94_ptr_1 == 0) {
            struct_fn_1000_160a();
            ctx.g_u16_ptr_1050_5f2e = in_dx;
        } else {
        }
        alloc_mem_1000_1708(0x1e, 0x10000, _g_Struct94_ptr_1, ctx.g_u16_ptr_1050_5f2e);
    } else {
        ctx.g_u16_ptr_1050_5f2e = in_dx;
        if (local_bx_18.field_0x30 + 1 < local_bx_18.field_0x32) {}
        // goto LAB_1018_022a;
        piVar1 = &local_bx_18.field_0x32;
        *piVar1 = *piVar1 + 5;
        _g_Struct94_ptr_1 = (local_bx_18.field_0x32 * 6);
        alloc_mem_1000_0ed4(1, _g_Struct94_ptr_1, 0, &local_bx_18.field_0x2c);
        ctx.g_u16_ptr_1050_5f2e = ctx.dx_reg;
    }
    local_bx_18.field_0x2c = _g_Struct94_ptr_1;
    &local_bx_18.field_0x2e = ctx.g_u16_ptr_1050_5f2e;
    // LAB_1018_022a:
    ppVar4 = pass1_1030_8344(
        ctx._g_bool_1050_5748,
        (ctx._g_bool_1050_5748 >> 0x10),
        param_2,
    );
    ppVar4 = pass1_1030_8344(
        ctx._g_bool_1050_5748,
        (ctx._g_bool_1050_5748 >> 0x10),
        &ppVar4.field_0x10,
    );
    i_var2 = local_bx_18.field_0x30;
    piVar1 = &local_bx_18.field_0x30;
    *piVar1 = *piVar1 + 1;
    u_var3 = &local_bx_18.field_0x2c;
    modify_list_1008_3f62(
        (u_var3 & 0xffff0000 | (u_var3 + i_var2 * 6)),
        CONCAT22(ctx.g_u16_ptr_1050_5f2e, &ppVar4.field_0xc),
    );
    return;
}

// WARNING: Variable defined which should be unmapped: local_32
// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn pass1_1018_028c(param_1: u32, param_2: u32) {
    let mut u_var1: u32;
    let ppc_var2: fn();
    let pu_var3: *mut u8;
    let mut u_var4: i32;
    let mut u_var5: i32;
    let mut i32_var6: i32;
    let mut u_var7: u32;
    let mut in_dx: u16;
    let mut u_var8: i32;
    let struct_a: *mut Struct199;
    let paVar9: *mut Struct199;
    
    
    
    let mut iVar10: i32;
    let mut u_var11: u16;
    let mut u_var12: u16;
    let mut unaff_ss: u16;
    let mut local_32: u16;
    let mut local_30: u16;
    let mut local_2e: u16;
    let mut local_2c: u16;
    let mut local_24: u16;
    let mut local_22: u16;
    let mut local_20: u16;
    let mut local_1e: u16;
    let mut local_1c: u16;
    let mut local_1a: u16;
    let mut local_18: [u8; 4];
    let mut local_14: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    local_6 = pass1_1030_8344(
        ctx._g_bool_1050_5748,
        (ctx._g_bool_1050_5748 >> 0x10),
        param_2,
    );
    local_8 = pass1_1030_5b00(CONCAT22(in_dx, local_6));
    _local_c = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(local_32, local_8));
    zero_list_1008_6c90(local_18, unaff_ss);
    pass1_1018_0b1e(_local_c, (_local_c >> 0x10), local_18, unaff_ss);
    u_var8 = local_14 >> 0xf;
    if ((u_var8 | local_14) == 0) {
        pu_var3 = local_18;
        pass1_1030_6522(_PTR_LOOP_1050_5740, CONCAT22(unaff_ss, pu_var3), param_2);
    } else {
        pu_var3 = local_18;
        pass1_1030_62e4(_PTR_LOOP_1050_5740, CONCAT22(unaff_ss, pu_var3), param_2);
    }
    _local_1c = CONCAT22(u_var8, pu_var3);
    u_var4 = u_var8 | pu_var3;
    if (u_var4 == 0) {
        return;
    }
    pass1_1018_04f2(param_1);
    u_var12 = 0x1000;
    paVar9 = struct_a;
    process_struct_1000_179c(0x1c, struct_a);
    iVar10 = param_1;
    u_var11 = (param_1 >> 0x10);
    u_var5 = u_var4;
    if ((paVar9 | u_var4) == 0) {
        (iVar10 + 0x12) = 0;
    } else {
        u_var12 = SUB42(&ctx.PTR_LOOP_1050_1008, 0);
        process_struct_1008_8e9e(CONCAT22(paVar9, u_var4), 6, 0x24);
        (iVar10 + 0x12) = u_var5;
        (iVar10 + 0x14) = ctx.dx_reg;
    }
    ppc_var2 = (*_local_1c + 0x10);
    ppc_var2(u_var12, pu_var3, u_var8, u_var4);
    local_24 = 0;
    while (local_24 < u_var5) {
        u_var7 = SEXT24(local_24);
        ppc_var2 = (*_local_1c + 4);
        ppc_var2();
        if ((ctx.dx_reg | u_var7) != 0) {
            i32_var6 = local_24 / 6;
            u_var1 = (iVar10 + 0xe);
            pass1_1018_dd7c(
                u_var1,
                (u_var1 >> 0x10),
                CONCAT22(local_24 % 6, i32_var6),
                (u_var7 & 0xffff | ctx.dx_reg << 0x10),
            );
            pass1_1008_8faa((iVar10 + 0x12), CONCAT22(ctx.dx_reg, i32_var6));
        }
        local_24 = local_24 + 1;
    }
    return;
}

pub unsafe fn pass1_1018_03ea(param_1: u32, param_2: i32) {
    if (param_2 != 5) {
        return;
    }
    pass1_1010_1f62((param_1 & 0xffff0000 | (param_1 - 10)), 5);
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn pass1_1018_0412(
    param_1: u32,
    param_2: u16,
    param_3: u32,
    param_4: u16,
    param_5: u32,
) -> i32 {
    let mut iVar1: i32;
    let mut unaff_ss: u16;
    let mut local_12e: u16;
    let mut local_12c: u16;
    let mut local_12a: u16;
    let mut local_128: u16;
    let mut local_126: u16;
    let mut local_4: u16;

    local_4 = 0;
    if (((0x72 < param_4) && (!SBORROW2(param_4, 0x73)))
        && (param_4 == 0x75
            || (param_4 - 0x74) < 1
            || (0 < (param_4 - 0x76) && ((param_4 - 0x77) < 2))))
    {
        local_4 = 1;
    }
    pass1_1028_933c(
        CONCAT22(unaff_ss, &local_128),
        param_2,
        local_4,
        param_4,
        param_3,
        (param_3 >> 0x10),
        (param_1 + 0x24),
        param_5,
    );
    iVar1 = pass1_1030_835a(ctx._g_bool_1050_5748, CONCAT22(unaff_ss, &local_128));
    if (iVar1 != 0) {
        pass1_1010_1f62(param_1, 6);
    }
    return;
}

pub unsafe fn pass1_1018_04a4(param_1: u32, param_2: u32) {
    (param_1 + 0x16) = param_2;
    return;
}

pub unsafe fn pass1_1018_04b8(param_1: u32) {
    let mut u_var1: u16;

    u_var1 = (param_1 >> 0x10);
    return CONCAT22((param_1 + 0x18), (param_1 + 0x16));
}

pub unsafe fn pass1_1018_04ca(param_1: u32, param_2: u32) {
    (param_1 + 0x1a) = param_2;
    return;
}

pub unsafe fn pass1_1018_04de(param_1: u32, param_2: u32) {
    (param_1 + 0x20) = param_2;
    return;
}

pub unsafe fn pass1_1018_04f2(param_1: u32) {
    let pu_var1: *mut u32;
    let mut u_var2: i32;
    let ppc_var3: fn();
    let local_bx_4: *mut Struct498;
    let mut local_es_4: u16;
    let temp_8621d6c8635: *mut u32;

    local_es_4 = (param_1 >> 0x10);
    local_bx_4 = param_1;
    pu_var1 = local_bx_4.field_0x12;
    u_var2 = &local_bx_4.field_0x14;
    if ((u_var2 | pu_var1) != 0) {
        ppc_var3 = *pu_var1;
        (**ppc_var3)();
    }
    &local_bx_4.field_0x12 = 0;
    return;
}

pub unsafe fn pass1_1018_0532(param_1: u32, param_2: u8) {
    pass1_1010_eb66(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1018_078e(param_1: *mut Struct376) {
    let mut u_var1: i32;
    let p_uvar2: *mut u16;
    let mut u_var3: i32;
    let local_bx_5: *mut Struct376;
    let mut u_var4: i32;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_6: u32;

    u_var4 = (param_1 >> 0x10);
    local_bx_5 = param_1;
    param_1.ptr_a_lo = (s_582_bmp_1050_1871 + 3);
    local_bx_5.ptr_a_hi = 0x1018;
    local_bx_5.ptr_2_lo = (s_589_bmp_1050_18a9 + 7);
    local_bx_5.ptr_2_hi = 0x1018;
    PTR_LOOP_1050_3960 = PTR_LOOP_1050_3960 + -1;
    (_PTR_LOOP_1050_3962 + local_bx_5.palette_handle_x12 * 2 + -4) = 0;
    if (PTR_LOOP_1050_3960 == 0x0) {
        error_check_1000_17ce(_PTR_LOOP_1050_3962);
        _PTR_LOOP_1050_3962 = 0x0;
    }
    error_check_1000_17ce(local_bx_5.u32_x94);
    error_check_1000_17ce(local_bx_5.u32_x9a);
    error_check_1000_17ce(local_bx_5.u32_x88);
    error_check_1000_17ce(local_bx_5.u32_x8e);
    if (local_bx_5.u32_x2c != 0) {
        if ((u_var4 | local_bx_5) == 0) {
            pu_var2 = 0x0;
            u_var3 = 0;
        } else {
            pu_var2 = &local_bx_5.ptr_2_lo;
            u_var3 = u_var4;
        }
        pass1_1010_1ea6(local_bx_5.u32_x2c, CONCAT22(u_var3, pu_var2));
    }
    if (local_bx_5.u32_x9e != 0) {
        if ((u_var4 | local_bx_5) == 0) {
            pu_var2 = 0x0;
            u_var3 = 0;
        } else {
            pu_var2 = &local_bx_5.ptr_2_lo;
            u_var3 = u_var4;
        }
        pass1_1010_1ea6(local_bx_5.u32_x9e, CONCAT22(u_var3, pu_var2));
    }
    u_var3 = local_bx_5.u16_x60;
    u_var1 = local_bx_5.u16_x62;
    local_6 = CONCAT22(u_var1, u_var3);
    if ((u_var1 | u_var3) != 0) {
        pass1_1008_5118(CONCAT22(u_var1, u_var3));
        error_check_1000_17ce(local_6);
    }
    local_bx_5.u32_x4c = 0;
    if (param_1 == 0x0) {
        pu_var2 = 0x0;
        u_var4 = 0;
    } else {
        pu_var2 = &local_bx_5.ptr_2_lo;
    }
    _local_1a = CONCAT22(u_var4, pu_var2);
    *_local_1a = ctx.s_1_1050_389a;
    pu_var2[1] = &ctx.PTR_LOOP_1050_1008;
    pass1_1010_1d80(param_1);
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn pass1_1018_0902(param_1: *mut u32, param_2: u32) {
    let mut u_var1: u32;
    let ppc_var2: fn();
    let ppVar3: *mut pass1_struct_2;
    let mut i_var4: i32;
    let mut u_var5: u16;
    let mut u_var6: u32;
    let pu_var7: *mut u32;
    let puVar8: *mut u32;
    let mut local_8: u16;

    puVar8 = (param_1 & 0xffff0000 | (param_1 + 0x28));
    pu_var7 = (param_1 & 0xffff0000 | ZEXT24((param_1 + 0x24)));
    u_var5 = param_1._2_2_;
    ppVar3 = pass1_1030_8344(
        ctx._g_bool_1050_5748,
        (ctx._g_bool_1050_5748 >> 0x10),
        param_2,
    );
    pass1_1030_5a52(CONCAT22(u_var5, ppVar3), pu_var7, puVar8);
    u_var6 = process_struct_1008_4772((param_1 + 0x24));
    i_var4 = u_var6;
    (param_1 + 0x5a) = i_var4;
    (param_1 + 0x5c) = (u_var6 >> 0x10);
    pass1_1018_17f0(param_1);
    (param_1 + 0x12) = i_var4 + 2;
    (i_var4 * 2 + _PTR_LOOP_1050_3962) = 1;
    ppc_var2 = (*param_1 + 0x18);
    ppc_var2();
    (param_1 + 0x3c) = param_2;
    u_var1 = (param_1 + 0x2c);
    u_var6 = pass1_1010_ec18(
        u_var1,
        (u_var1 >> 0x10),
        param_2 & 0xffff0000 | *(param_1 + 0x3c),
    );
    (param_1 + 0x7c) = u_var6;
    (param_1 + 0x7e) = (u_var6 >> 0x10);
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn pass1_1018_0a50(param_1: *mut Struct510) {
    let local_bx_3: *mut Struct510;
    let mut u_var1: u16;

    u_var1 = (param_1 >> 0x10);
    local_bx_3 = param_1;
    if (local_bx_3.field_0x84 == 2) {
        return CONCAT22(local_bx_3.field_0x2a, local_bx_3.field_0x28);
    }
    return CONCAT22(local_bx_3.field_0x26, local_bx_3.field_0x24);
}

pub unsafe fn pass1_1018_0a76(param_1: u32) {
    let mut u_var1: u16;
    let mut u_var2: i32;

    u_var2 = (param_1 >> 0x10);
    if ((param_1 + 0x84) == 1) {
        u_var1 = 2;
    } else {
        u_var1 = 1;
    }
    (param_1 + 0x84) = u_var1;
    pass1_1010_1f62((param_1 & 0xffff | u_var2 << 0x10), 4);
    return;
}

pub unsafe fn pass1_1018_0aa0(param_1: *mut Struct511, param_2: u16) {
    let local_bx_6: *mut Struct511;
    let mut u_var1: u16;

    u_var1 = (param_1 >> 0x10);
    local_bx_6 = param_1;
    local_bx_6.field_0x14 = param_2;
    pass1_1018_04de(local_bx_6.field_0x2c, local_bx_6.field_0x3c);
    return;
}

pub unsafe fn pass1_1018_0ac0(param_1: u32, param_2: u32) {
    (param_1 + 0x80) = param_2;
    return;
}

pub unsafe fn pass1_1018_0ad4(param_1: u32) {
    let mut u_var1: u16;

    u_var1 = (param_1 >> 0x10);
    return CONCAT22((param_1 + 0x82), (param_1 + 0x80));
}

pub unsafe fn pass1_1018_0ae8(param_1: u32, param_2: u16) {
    (param_1 + 0x5e) = param_2;
    return;
}

pub unsafe fn pass1_1018_0afa(param_1: u32) {
    return (param_1 + 0x5e);
}

pub unsafe fn pass1_1018_0b08(param_1: u32) {
    let mut u_var1: u32;
    let local_bx_6: *mut Struct512;
    let mut u_var2: u16;

    u_var1 = (param_1 + 0x7c);
    u_var2 = (u_var1 >> 0x10);
    local_bx_6 = u_var1;
    return CONCAT22(local_bx_6.field_0x6, local_bx_6.field_0x4);
}

pub unsafe fn pass1_1018_0b1e(param_1: u32, param_2: *mut u16) {
    let mut iVar1: i32;
    let mut u_var2: u32;
    let mut i_var3: i32;
    let mut u_var4: u16;
    let mut unaff_ss: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    i_var3 = param_1;
    pass1_1008_3eb4(
        (param_1 & 0xffff0000 | (i_var3 + 0x30)),
        CONCAT22(unaff_ss, &local_8),
        CONCAT22(unaff_ss, &local_6),
        CONCAT22(unaff_ss, &local_4),
    );
    if ((local_4 - 3) < 1) {
        local_4 = 3;
    }
    if ((local_6 - 3) < 1) {
        local_6 = 3;
    }
    u_var4 = (param_1 >> 0x10);
    u_var2 = (i_var3 + 0x5a);
    iVar1 = (u_var2 + 4);
    if (iVar1 <= (local_4 + 2)) {
        local_4 = iVar1 - 3;
    }
    u_var2 = (i_var3 + 0x5a);
    iVar1 = (u_var2 + 8);
    if (iVar1 <= (local_6 + 2)) {
        local_6 = iVar1 - 3;
    }
    pass1_1008_6cec(
        param_1 & 0xffff0000 | (i_var3 + 0x40),
        local_8,
        CONCAT22(local_4 + 2, local_6 + 2),
        local_8,
        CONCAT22(local_4 - 3, local_6 - 3),
    );
    modify_list_1008_3f62(param_2, param_1 & 0xffff0000 | (i_var3 + 0x40));
    modify_list_1008_3f62(
        (param_2 & 0xffff0000 | (param_2 + 6)),
        param_1 & 0xffff0000 | (i_var3 + 0x46),
    );
    return;
}
