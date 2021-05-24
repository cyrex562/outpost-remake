use crate::pass::pass4_funcs;

pub fn pass1_1028_a61e(param_1: u16, param_2: u16, param_1_00: u32, param_2_00: u32) {
    let mut in_ax: i32;
    let mut u_var1: i32;
    let mut u_var2: u32;
    let mut in_dx: i32;
    let mut i_var3: i32;
    let mut u_var4: u16;
    let ppVar5: &mut  Struct2551;
    let local_28: &mut  Struct818;
    let mut uStack38: u16;
    let mut uStack36: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;

    local_28 = param_1_00;
  // uStack38 = (param_1_00  >> 0x10);
    pass1_1030_38b8(param_1_00);
    if ((in_dx < 0x3fff) || (in_dx < 0x4000 && (in_ax != 0xffff))) {
        pass1_1030_38f2(param_1_00, 3);
        u_var1 = in_ax;
        i_var3 = in_dx;
        pass1_1030_38f2(param_1_00, 4);
        _local_e = CONCAT22(in_dx + i_var3 + CARRY2(in_ax, u_var1), in_ax + u_var1);
        local_10 = local_28.field_0x1a8;
        if (local_10 == 0) {
            local_10 = 5;
        }
        u_var2 = _local_e / local_10;
      // local_c = (u_var2  >> 0x10);
        if (((local_c | u_var2) != 0)
            && (
              // u_var4 = (param_2_00  >> 0x10),
                (param_2_00 + 0x200) != 0x8000002,
            ))
        {
            ppVar5 = process_struct_1010_20ba(ctx.g_struct_var_1050_0ed0, CONCAT22(uStack36, 0x2b));
            pass1_1010_043a(ppVar5, (param_2_00 + 4), 0xc);
            pass1_1030_3534(param_1_00, u_var2);
        }
    }
    return;
}

pub fn pass1_1028_a6ca(param_1: &mut Struct44, param_2: u8) -> &mut Struct44 {
    param_1.ptr_a_lo = ctx.s_1_1050_389a;
    (param_1 + 2) = &ctx.PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1028_9efc(param_1: u32) {
    let lVar1: u32;
    let mut u_var2: u16;
    let pu_var3: &mut  u16;
    let mut u_var4: i32;
    let local_AX_291: &mut  Struct814;
    let lVar5: u32;
    let mut u_var6: u32;
    let mut in_dx: i32;


    let mut u_var7: u16;

    let mut unaff_ss: u16;
    let pp_var8: &mut  Struct2551;
    let mut u_var9: u16;
    let mut local_3a: u16;
    let mut local_38: u16;
    let mut local_36: u16;
    let mut local_34: u16;
    let mut local_30: u16;
    let mut local_2e: u16;
    let mut local_2c: u16;
    let mut local_2a: u16;
    let mut local_28: u16;
    let mut local_26: u16;
    let mut local_20: u16;
    let mut local_1e: u16;
    let mut local_1c: u32;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    local_6 = pass4_funcs::pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, 1, 0x400);
    if ((in_dx | local_6) != 0) {
        local_4 = in_dx;
        pass4_funcs::pass1_1028_dc52(
            CONCAT13((unaff_ss >> 8), CONCAT12(unaff_ss, &local_18)),
            (&ctx.PTR_LOOP_1050_0000 + 1),
            0,
            0x400,
        );
        while (true) {
            pu_var3 = &local_18;
            pass4_funcs::pass1_1028_e4ec(CONCAT22(unaff_ss, pu_var3));
            local_6 = pu_var3;
            local_4 = ctx.dx_reg;
            if ((ctx.dx_reg | pu_var3) == 0) {
                break;
            }
            lVar1 = (pu_var3 + 0x100);
            u_var7 = pu_var3[0x101];
            if (pu_var3[0xff] != 0) {
              // u_var9 = (param_1  >> 0x10);
                lVar5 = lVar1;
                if ((lVar1 != 2) || (u_var7 != 0x800)) {
                    pass4_funcs::pass1_1028_a3ae(param_1, u_var9, CONCAT22(ctx.dx_reg, pu_var3));
                }
                u_var4 = lVar5;
                pass4_funcs::pass1_1028_a28a(param_1, u_var9, CONCAT22(local_4, local_6));
                if ((u_var7 < 1) && (u_var7 < 0 || (u_var4 < 100))) {
                    pass4_funcs::pass1_1028_a4ee(param_1, CONCAT22(local_4, local_6));
                }
                if (lVar1 != 0x8000002) {
                    pass1_1038_42cc(CONCAT22(local_4, local_6));
                    if ((ctx.dx_reg | u_var4) != 0) {
                        pp_var8 = process_struct_1010_20ba(
                            ctx.g_struct_var_1050_0ed0,
                            CONCAT22(local_3a, 0x37),
                        );
                        post_win_msg_1008_a0e4(
                            pp_var8,
                            0,
                            u_var4,
                            (local_6 + 0x208),
                            (local_6 + 4),
                            2,
                        );
                    }
                }
            }
        }
        local_18 = ctx.s_1_1050_389a;
        local_16 = &ctx.PTR_LOOP_1050_1008;
        pp_var8 = process_struct_1010_20ba(ctx.g_struct_var_1050_0ed0, CONCAT22(local_3a, 8));
      // u_var7 = (pp_var8  >> 0x10);
        u_var2 = SUB42(pp_var8, 0);
        local_AX_291 = pass4_funcs::pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, 1, 0x400);
        local_6 = local_AX_291;
        local_4 = u_var7;
        pass1_1010_9f72(pp_var8, 0x3e);
        if (local_AX_291 != 0x0) {
            pass1_1010_96d0(pp_var8);
            if (local_AX_291 < 1) {
                if (local_AX_291 < 0) {
                    u_var6 = (local_6 + 0x1f6);
                    pass1_1030_38b8(u_var6, (u_var6 >> 0x10));
                    if ((ctx.dx_reg < 1) && (ctx.dx_reg < 0 || (u_var6 == 0))) {
                        pp_var8 = process_struct_1010_20ba(
                            ctx.g_struct_var_1050_0ed0,
                            CONCAT22(u_var2, 0x37),
                        );
                        post_win_msg_1008_a0e4(pp_var8, 0, 0, 1, (local_6 + 4), 6);
                    }
                }
            } else {
                pp_var8 =
                    process_struct_1010_20ba(ctx.g_struct_var_1050_0ed0, CONCAT22(u_var2, 0x37));
                post_win_msg_1008_a0e4(pp_var8, 0, local_AX_291, (local_6 + 0x208), 0x4000001, 2);
                pp_var8 =
                    process_struct_1010_20ba(ctx.g_struct_var_1050_0ed0, CONCAT22(u_var2, 0x2b));
                pass1_1010_043a(pp_var8, (local_6 + 4), 0x14);
            }
        }
    }
    return;
}

pub fn pass1_1028_a0fa(param_1: u32) {
    let pu_var1: &mut  u32;
    let pu_var2: &mut  u32;
    let ctx.ax_reg: &mut  Struct815;
    let pu_var3: &mut  u32;
    let mut i_var4: i32;
    let in_dx: &mut  Struct199;
    let pu_var5: &mut  u32;
    let mut u_var6: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;

    process_struct_1000_179c(0x108, in_dx);
    _local_a = CONCAT22(in_dx, ctx.ax_reg);
    if ((in_dx | ctx.ax_reg) != 0) {
        *_local_a = ctx.s_1_1050_389a;
        ctx.ax_reg.field_0x2 = &ctx.PTR_LOOP_1050_1008;
      // u_var6 = (param_1  >> 0x10);
        ctx.ax_reg.field_0x4 = (param_1 + 4);
        pu_var3 = (param_1 + 8);
        pu_var5 = &ctx.ax_reg.field_0x8;
        i_var4 = 0x40;
        while (i_var4 != 0) {
            i_var4 = i_var4 + -1;
            pu_var2 = pu_var5;
            pu_var5 = pu_var5 + 1;
            pu_var1 = pu_var3;
            pu_var3 = pu_var3 + 1;
            unsafe { *pu_var2 = *pu_var1 };
        }
        *_local_a = 0x6ad2;
        ctx.ax_reg.field_0x2 = &PTR_LOOP_1050_1028;
        *_local_a = 0xa6f6;
        ctx.ax_reg.field_0x2 = &PTR_LOOP_1050_1028;
    }
    return;
}

pub fn pass1_1028_9b48(param_1: &mut  Struct808) {
    let pu_var1: &mut  u32;
    let pu_var2: &mut  u32;
    let ctx.ax_reg: &mut  Struct809;
    let mut i_var3: i32;
    let in_dx: &mut  Struct199;
    let local_bx_25: &mut  Struct808;
    let pu_var4: &mut  u32;
    let pu_var5: &mut  u32;
    let mut u_var6: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;

    process_struct_1000_179c(0x118, in_dx);
    _local_a = CONCAT22(in_dx, ctx.ax_reg);
    local_bx_25 = param_1;
  // u_var6 = (param_1  >> 0x10);
    if ((in_dx | ctx.ax_reg) != 0) {
        *_local_a = ctx.s_1_1050_389a;
        ctx.ax_reg.field_0x2 = &ctx.PTR_LOOP_1050_1008;
        ctx.ax_reg.field_0x4 = local_bx_25.field_0x4;
        pu_var4 = &local_bx_25.field_0x8;
        pu_var5 = &ctx.ax_reg.field_0x8;
        i_var3 = 0x40;
        while (i_var3 != 0) {
            i_var3 = i_var3 + -1;
            pu_var2 = pu_var5;
            pu_var5 = pu_var5 + 1;
            pu_var1 = pu_var4;
            pu_var4 = pu_var4 + 1;
            unsafe { *pu_var2 = *pu_var1 };
        }
        *_local_a = 0x6ad2;
        ctx.ax_reg.field_0x2 = &PTR_LOOP_1050_1028;
        ctx.ax_reg.field_0x108 = local_bx_25.field_0x108;
        ctx.ax_reg.field_0x10c = local_bx_25.field_0x10c;
        ctx.ax_reg.field_0x110 = local_bx_25.field_0x110;
        ctx.ax_reg.field_0x114 = local_bx_25.field_0x114;
        *_local_a = 0x9c52;
        ctx.ax_reg.field_0x2 = &PTR_LOOP_1050_1028;
    }
    local_bx_25.field_0x114 = 0;
    return;
}

pub fn pass1_1028_9c2c(param_1: &mut  Struct805, param_2: u8) -> &mut  Struct805 {
    pass1_1028_9992(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1028_9c62(param_1: &mut  Struct811, param_2: u16, param_3: u16) {
    pass4_funcs::pass1_1028_d1dc(CONCAT22(param_2, param_1), param_3);
    param_1.field_0x108 = param_3;
    CONCAT22(param_2, param_1) = 0x9eb6;
    param_1.field_0x2 = &PTR_LOOP_1050_1028;
    return CONCAT22(param_2, param_1);
}

pub fn pass1_1028_9c90(param_1: u32) {
    let mut u_var1: i32;
    let mut u_var2: u16;

    u_var1 = (param_1 + 0x108) - 1000;
    if ((u_var1 < s_K1_1050_3a99) && (u_var1 % 1000 == 0)) {
        // WARNING: Could not recover jumptable at 0x10289dc0. Too many branches
        // WARNING: Treating indirect jump as call
        u_var2 = (**((u_var1 / 1000) * 2 + -0x623a))();
        return u_var2;
    }
    return 1;
}

pub fn pass1_1028_9dee(param_1: &mut  Struct812) {
    let pu_var1: &mut  u32;
    let pu_var2: &mut  u32;
    let ctx.ax_reg: &mut  Struct813;
    let mut i_var3: i32;
    let in_dx: &mut  Struct199;
    let local_bx_41: &mut  Struct812;
    let pu_var4: &mut  u32;
    let pu_var5: &mut  u32;
    let mut u_var6: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;

    process_struct_1000_179c(0x10a, in_dx);
    _local_a = CONCAT22(in_dx, ctx.ax_reg);
    if ((in_dx | ctx.ax_reg) != 0) {
        *_local_a = ctx.s_1_1050_389a;
        ctx.ax_reg.field_0x2 = &ctx.PTR_LOOP_1050_1008;
      // u_var6 = (param_1  >> 0x10);
        local_bx_41 = param_1;
        ctx.ax_reg.field_0x4 = local_bx_41.field_0x4;
        pu_var4 = &local_bx_41.field_0x8;
        pu_var5 = &ctx.ax_reg.field_0x8;
        i_var3 = 0x40;
        while (i_var3 != 0) {
            i_var3 = i_var3 + -1;
            pu_var2 = pu_var5;
            pu_var5 = pu_var5 + 1;
            pu_var1 = pu_var4;
            pu_var4 = pu_var4 + 1;
            unsafe { *pu_var2 = *pu_var1 };
        }
        *_local_a = 0x6ad2;
        ctx.ax_reg.field_0x2 = &PTR_LOOP_1050_1028;
        ctx.ax_reg.field_0x108 = local_bx_41.field_0x108;
        *_local_a = 0x9eb6;
        ctx.ax_reg.field_0x2 = &PTR_LOOP_1050_1028;
    }
    return;
}

pub fn pass1_1028_9e8a(param_1: &mut Struct44, param_2: u8) -> &mut Struct44 {
    param_1.ptr_a_lo = ctx.s_1_1050_389a;
    (param_1 + 2) = &ctx.PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1028_93d4(param_1: u32) {
    let pp_var1: fn();
    let paVar2: &mut  Struct493;
    let mut i_var3: i32;
    let mut u_var4: u32;
    let mut in_dx: i32;
    let local_bx_20: &mut  Struct801;
    let mut u_var5: u16;
    let mut unaff_ss: u16;
    let mut local_114: u16;
    let mut local_112: u16;
    let mut local_110: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    PTR_LOOP_1050_50ca = 0x0;
    PTR_LOOP_1050_50cc = 0x0;
  // u_var5 = (param_1  >> 0x10);
    local_bx_20 = param_1;
    u_var4 = SEXT24(local_bx_20.field_0x11a);
    pass4_funcs::pass1_1028_e2e0(ctx._PTR_LOOP_1050_65e2);
    _local_6 = u_var4 & 0xffff | in_dx << 0x10;
    paVar2 = pass4_funcs::pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, u_var4, in_dx);
    &local_bx_20.field_0x11e = paVar2;
    local_bx_20.field_0x120 = in_dx;
    i_var3 = &local_bx_20.field_0x114;
    pp_var1 = (&local_bx_20.field_0x11e + 0x1c);
    (**pp_var1)();
    if (i_var3 != 0) {
        pass1_1028_9624(param_1);
        pp_var1 = (&local_bx_20.field_0x11e + 0x20);
        (**pp_var1)();
        pp_var1 = (&local_bx_20.field_0x11e + 0x18);
        (**pp_var1)();
        pass1_1028_9600(param_1);
        return;
    }
    &local_bx_20.field_0x11e = 0;
    pass1_1030_e4fa(CONCAT22(unaff_ss, &local_112), _local_6);
    pass1_1030_835a(ctx._g_bool_1050_5748, CONCAT22(unaff_ss, &local_112));
    if (PTR_LOOP_1050_50ca == 0x0) {
        PTR_LOOP_1050_50ca = 0x6ad;
    }
    return;
}

pub fn pass1_1028_94e4(param_1: &mut  Struct803) {
    let pu_var1: &mut  u32;
    let pu_var2: &mut  u32;
    let ctx.ax_reg: &mut  Struct802;
    let mut i_var3: i32;
    let in_dx: &mut  Struct199;
    let local_bx_43: &mut  Struct803;
    let pu_var4: &mut  u32;
    let pu_var5: &mut  u32;
    let mut u_var6: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;

    process_struct_1000_179c(0x124, in_dx);
    _local_a = CONCAT22(in_dx, ctx.ax_reg);
    if ((in_dx | ctx.ax_reg) != 0) {
        *_local_a = ctx.s_1_1050_389a;
        ctx.ax_reg.field_0x2 = &ctx.PTR_LOOP_1050_1008;
      // u_var6 = (param_1  >> 0x10);
        local_bx_43 = param_1;
        ctx.ax_reg.field_0x4 = local_bx_43.field_0x4;
        pu_var4 = &local_bx_43.field_0x8;
        pu_var5 = &ctx.ax_reg.field_0x8;
        i_var3 = 0x40;
        while (i_var3 != 0) {
            i_var3 = i_var3 + -1;
            pu_var2 = pu_var5;
            pu_var5 = pu_var5 + 1;
            pu_var1 = pu_var4;
            pu_var4 = pu_var4 + 1;
            unsafe { *pu_var2 = *pu_var1 };
        }
        *_local_a = 0x6ad2;
        ctx.ax_reg.field_0x2 = &PTR_LOOP_1050_1028;
        ctx.ax_reg.field_0x108 = local_bx_43.field_0x108;
        ctx.ax_reg.field_0x10c = local_bx_43.field_0x10c;
        ctx.ax_reg.field_0x110 = local_bx_43.field_0x110;
        ctx.ax_reg.field_0x114 = local_bx_43.field_0x114;
        ctx.ax_reg.field_0x118 = local_bx_43.field_0x118;
        ctx.ax_reg.field_0x11a = local_bx_43.field_0x11a;
        ctx.ax_reg.field_0x11c = local_bx_43.field_0x11c;
        ctx.ax_reg.field_0x11e = local_bx_43.field_0x11e;
        ctx.ax_reg.field_0x122 = local_bx_43.field_0x122;
        *_local_a = 0x9934;
        ctx.ax_reg.field_0x2 = &PTR_LOOP_1050_1028;
    }
    return;
}

pub fn pass1_1028_9600(param_1: u32) {
    let mut unaff_ss: u16;
    let mut local_6: [u8; 4];

    pass1_1020_a43e(CONCAT22(unaff_ss, local_6));
    pass1_1020_a80e(local_6, unaff_ss, (param_1 + 0x11a));
    return;
}

pub fn pass1_1028_9624(param_1: u32) {
    let mut u_var1: u32;
    let ppc_var2: fn();
    let paVar3: &mut  Struct493;
    let pu_var4: &mut  u32;
    let mut u_var5: u16;
    let mut u_var6: i32;
    let mut i_var7: i32;
    let BVar8: bool;
    let mut in_dx: u16;





    let mut extraout_dx_04: u16;
    let mut i_var9: i32;
    let mut unaff_si: u16;
    let mut u_var10: u16;
    let mut u_var11: u16;
    let mut unaff_ss: u16;
    let mut uStack332: u16;
    let mut uStack330: u16;
    let mut uStack64: u16;
    let mut uStack62: u32;
    let mut iStack58: i32;
    let mut local_38: u32;
    let mut local_2e: u32;
    let mut uStack42: u32;
    let mut local_26: [u8; 4];
    let paStack34: &mut  Struct493;
    let mut uStack32: u16;
    let mut uStack30: u32;
    let mut uStack26: u32;
    let mut local_16: u32;
    let mut local_12: [u8; 2];
    let mut local_10: [u8; 2];
    let mut local_e: [u8; 2];
    let mut local_c: u16;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

  // u_var10 = (param_1  >> 0x10);
    i_var9 = param_1;
    u_var1 = (i_var9 + 0x10c);
  // paVar3 = pass4_funcs::pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, u_var1, (u_var1  >> 0x10));
    (i_var9 + 0x110) = paVar3;
    (i_var9 + 0x112) = in_dx;
    _local_6 = process_struct_1010_20ba(ctx.g_struct_var_1050_0ed0, CONCAT22(unaff_si, 0x2f));
    u_var1 = (i_var9 + 0x108);
    pu_var4 = (i_var9 + 0x114);
    pass1_1030_64ce(
        _PTR_LOOP_1050_5740,
        (_PTR_LOOP_1050_5740 >> 0x10),
        pu_var4,
        u_var10,
        u_var1,
        (u_var1 >> 0x10),
        local_26,
        unaff_ss,
    );
    unsafe { local_38 = *pu_var4 };
    local_38._3_1_ = (local_38 >> 0x18);
    local_c = (local_38._3_1_ != '\0');
    u_var11 = SUB42(&ctx.PTR_LOOP_1050_1008, 0);
    local_2e = local_38;
    local_a = local_38;
    pass1_1008_3eb4(
        (param_1 & 0xffff0000 | (i_var9 + 0x114)),
        CONCAT22(unaff_ss, local_12),
        CONCAT22(unaff_ss, local_10),
        CONCAT22(unaff_ss, local_e),
    );
    if (local_c == 0) {
        local_a = ctx.dx_reg;
        u_var6 = i_var9 + 0x114;
        pass4_funcs::pass1_1028_e2ac(ctx._PTR_LOOP_1050_65e2);
        local_16 = CONCAT22(local_a, u_var6);
        u_var11 = 0x1030;
        pass1_1030_61fe(
            _PTR_LOOP_1050_5740,
            CONCAT22(local_a, u_var6),
            param_1 & 0xffff0000 | (i_var9 + 0x114),
            (i_var9 + 0x108),
        );
        if (((i_var9 + 0x11a) == 10) || ((i_var9 + 0x11a) == 0x37)) {
            if ((i_var9 + 0x11a) == 0x37) {
                local_38 = (i_var9 + 0x11e);
                local_a = (i_var9 + 0x120);
                uStack42 = (i_var9 + 0x10c);
                (local_38 + 0x20) = uStack42;
            }
            i_var7 = i_var9 + 0x114;
            pass4_funcs::pass1_1028_e2ac(ctx._PTR_LOOP_1050_65e2);
            (i_var9 + 0x10c) = i_var7;
            (i_var9 + 0x10e) = local_a;
            u_var11 = 0x1018;
            pass1_1018_0196(
                _local_6,
                CONCAT22(local_a, (i_var9 + 0x10c)),
                (i_var9 + 0x108),
            );
            local_a = ctx.dx_reg;
            if ((i_var9 + 0x11a) == 10) {
                u_var11 = 0x1010;
                pass1_1010_ed22(_local_6, (i_var9 + 0x10c));
                local_a = ctx.dx_reg;
            }
        }
        u_var1 = (i_var9 + 0x10c);
      // paVar3 = pass4_funcs::pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, u_var1, (u_var1  >> 0x10));
        (i_var9 + 0x110) = paVar3;
        (i_var9 + 0x112) = local_a;
        if ((local_a | (i_var9 + 0x110)) == 0) {}
        // goto LAB_1028_9807;
        u_var5 = local_16;
      // u_var6 = (local_16  >> 0x10);
    } else {
        local_16 = local_a;
        if ((i_var9 + 0x11a) != 0x75) {}
        // goto LAB_1028_9807;
        u_var5 = local_a;
        u_var6 = local_a;
        local_a = (i_var9 + 0x112);
    }
    ppc_var2 = ((i_var9 + 0x110) + 8);
    ppc_var2(
        u_var11,
        (i_var9 + 0x110),
        local_a,
        0,
        u_var5,
        u_var6,
        0,
    );
    local_a = ctx.dx_reg;
    // LAB_1028_9807:
  // paVar3 = pass4_funcs::pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, local_16, (local_16  >> 0x10));
    uStack26 = CONCAT22(local_a, paVar3);
    pass1_1030_73ee(CONCAT22(local_a, paVar3), (i_var9 + 0x10c));
    uStack32 = extraout_dx_04;
    BVar8 = pass1_1008_c6ae(ctx._PTR_LOOP_1050_06e0, (i_var9 + 0x11a), 0x31);
    if ((BVar8 == 0) && ((i_var9 + 0x122) == 0)) {
        uStack62 = (uStack26 + 0xc);
        iStack58 = (uStack26 + 0x10);
        local_38 = local_38 & 0xffff0000 | ZEXT24(&uStack62);
        if (iStack58 < 1) {
            uStack64 = 5;
        } else {
            uStack64 = 6;
        }
        (uStack26 + 0x14) = uStack64;
        uStack32 = uStack26;
    }
    uStack30 = (uStack26 + 0x16);
  // paStack34 = pass4_funcs::pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, uStack30, (uStack30  >> 0x10));
    if (uStack30 != 0) {
        pass1_1030_e4fa(CONCAT22(unaff_ss, &uStack332), uStack30);
        pass1_1030_835a(ctx._g_bool_1050_5748, CONCAT22(unaff_ss, &uStack332));
        uStack332 = SUB42(ctx.s_1_1050_389a, 0);
        uStack330 = SUB42(&ctx.PTR_LOOP_1050_1008, 0);
    }
    ppc_var2 = ((i_var9 + 0x11e) + 4);
    ppc_var2();
    u_var1 = (i_var9 + 0x11e);
    pass1_1030_7e5a(uStack26, (u_var1 + 4));
    return;
}

pub fn pass1_1028_9908(param_1: &mut Struct44, param_2: u8) -> &mut Struct44 {
    param_1.ptr_a_lo = ctx.s_1_1050_389a;
    (param_1 + 2) = &ctx.PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1028_9944(param_1: &mut  Struct500, param_2: u32, param_3: u32, param_4: u32) {
    let local_bx_19: &mut  Struct500;
    let mut u_var1: u16;

    pass4_funcs::pass1_1028_d1dc(param_1, 0x1387);
  // u_var1 = (param_1  >> 0x10);
    local_bx_19 = param_1;
    local_bx_19.field_0x108 = param_4;
    local_bx_19.field_0x10c = param_3;
    local_bx_19.field_0x110 = param_2;
    local_bx_19.field_0x114 = 0;
    param_1.a = 0x9c52;
    local_bx_19.b = &PTR_LOOP_1050_1028;
    return;
}

pub fn pass1_1028_9992(param_1: &mut  Struct805) {
    let local_bx_4: &mut  Struct805;
    let mut u_var1: u16;

  // u_var1 = (param_1  >> 0x10);
    local_bx_4 = param_1;
    param_1 = 0x9c52;
    local_bx_4.field_0x2 = &PTR_LOOP_1050_1028;
    error_check_1000_17ce(local_bx_4.field_0x114);
    param_1 = ctx.s_1_1050_389a;
    local_bx_4.field_0x2 = &ctx.PTR_LOOP_1050_1008;
    return;
}

pub fn pass1_1028_99c4(param_1: u32) {
    let mut u_var1: u32;
    let paVar2: &mut  Struct493;
    let mut in_dx: u16;
    let mut u_var3: u16;
    let mut local_6: u16;
    let mut local_4: u16;

  // u_var3 = (param_1  >> 0x10);
    u_var1 = (param_1 + 0x10c);
  // paVar2 = pass4_funcs::pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, u_var1, (u_var1  >> 0x10));
    pass1_1030_355c(paVar2[0x10].field_0x16, (param_1 + 0x114));
    return;
}

pub fn pass1_1028_8920(param_1: u32) {
    let pu_var1: &mut  u32;
    let mut u_var2: u32;
    let ppc_var3: fn();
    let pu_var4: &mut  u32;
    let b_var5: bool;
    let paVar6: &mut  Struct493;
    let mut u_var7: u32;




    let mut u_var8: i32;
    let mut i_var9: i32;
    let mut iVar10: i32;
    let mut unaff_si: u16;
    let mut u_var11: u16;
    let mut u_var12: u16;
    let mut unaff_ss: u16;
    let mut u_var13: u16;
    let mut local_156: u16;
    let mut local_154: u16;
    let mut local_4a: u32;
    let mut local_46: u16;
    let mut local_44: u32;
    let mut local_40: u32;
    let mut local_38: u32;
    let mut local_34: u16;
    let mut local_32: u16;
    let mut local_30: u16;
    let mut local_2e: u32;
    let mut local_2a: u16;
    let mut local_28: u16;
    let mut local_26: [u8; 4];
    let mut local_22: u16;
    let mut local_20: u16;
    let mut local_1e: u16;
    let mut local_1c: u16;
    let mut local_1a: u32;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: [u8; 2];
    let mut local_a: [u8; 2];
    let mut local_8: [u8; 2];
    let mut local_6: u32;

  // u_var11 = (param_1  >> 0x10);
    i_var9 = param_1;
    u_var2 = (i_var9 + 0x108);
    pu_var1 = (i_var9 + 0x114);
    pu_var4 = pu_var1;
    pass1_1030_64ce(
        _PTR_LOOP_1050_5740,
        (_PTR_LOOP_1050_5740 >> 0x10),
        pu_var1,
        u_var11,
        u_var2,
        (u_var2 >> 0x10),
        local_26,
        unaff_ss,
    );
    unsafe { local_6 = *pu_var4 };
    pass1_1008_3eb4(
        (param_1 & 0xffff0000 | ZEXT24(pu_var1)),
        CONCAT22(unaff_ss, local_c),
        CONCAT22(unaff_ss, local_a),
        CONCAT22(unaff_ss, local_8),
    );
    local_2e = local_6;
    local_38 = local_6;
    local_38._3_1_ = (local_6 >> 0x18);
    local_e = (local_38._3_1_ != '\0');
    if (local_e == 0) {
        u_var8 = i_var9 + 0x114;
        pass4_funcs::pass1_1028_e2ac(ctx._PTR_LOOP_1050_65e2);
        _local_12 = CONCAT22(local_6, u_var8);
        u_var12 = 0x1030;
        pass1_1030_61fe(
            _PTR_LOOP_1050_5740,
            CONCAT22(local_6, u_var8),
            param_1 & 0xffff0000 | (i_var9 + 0x114),
            (i_var9 + 0x108),
        );
        local_38 = 0;
        if (((i_var9 + 0x11a) == 10) || ((i_var9 + 0x11a) == 0x37)) {
            if ((i_var9 + 0x11a) == 0x37) {
                local_38 = (i_var9 + 0x10c);
            }
            iVar10 = i_var9 + 0x114;
            pass4_funcs::pass1_1028_e2ac(ctx._PTR_LOOP_1050_65e2);
            (i_var9 + 0x10c) = iVar10;
            (i_var9 + 0x10e) = local_6;
            local_2e =
                process_struct_1010_20ba(ctx.g_struct_var_1050_0ed0, CONCAT22(unaff_si, 0x2f));
            u_var12 = 0x1018;
            pass1_1018_0196(local_2e, (i_var9 + 0x10c), (i_var9 + 0x108));
            local_6 = ctx.dx_reg;
            if ((i_var9 + 0x110) != 0) {
                u_var2 = (i_var9 + 0x10c);
              // paVar6 = pass4_funcs::pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, u_var2, (u_var2  >> 0x10));
                _local_2a = CONCAT22(local_6, paVar6);
                local_44 = (i_var9 + 0x110);
                &paVar6[0x11].field_0x2 = local_44;
            }
        }
        u_var2 = (i_var9 + 0x10c);
      // paVar6 = pass4_funcs::pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, u_var2, (u_var2  >> 0x10));
        _local_34 = CONCAT22(local_6, paVar6);
        local_14 = local_6 | paVar6;
        if (local_14 != 0) {
            ppc_var3 = (*_local_34 + 8);
            (**ppc_var3)(
                u_var12,
                paVar6,
                local_6,
                0,
                _local_12,
                (_local_12 >> 0x10),
                0,
            );
            local_14 = ctx.dx_reg;
        }
    } else {
        _local_12 = local_6;
        local_14 = local_6;
    }
  // local_16 = pass4_funcs::pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, _local_12, (_local_12  >> 0x10));
    pass1_1030_73ee(CONCAT22(local_14, local_16), (i_var9 + 0x10c));
    b_var5 = pass1_1008_c6ae(ctx._PTR_LOOP_1050_06e0, (i_var9 + 0x11a), 0x31);
    if ((b_var5 == 0) && ((i_var9 + 0x11c) == 0)) {
        local_4a = (local_16 + 0xc);
        local_46 = (local_16 + 0x10);
        local_44 = local_44 & 0xffff0000 | ZEXT24(&local_4a);
        if (local_46 < 1) {
            local_30 = 5;
        } else {
            local_30 = 6;
        }
        (local_16 + 0x14) = local_30;
    }
    local_1a = (local_16 + 0x16);
    local_1c = (local_16 + 0x18);
    if ((local_1c | local_1a) != 0) {
        pass1_1030_e4fa(
            CONCAT22(unaff_ss, &local_156),
            local_1a & 0xffff | local_1c << 0x10,
        );
        pass1_1030_835a(ctx._g_bool_1050_5748, CONCAT22(unaff_ss, &local_156));
        _local_156 = 0x1008389a;
        local_1c = ctx.dx_reg;
    }
    u_var7 = SEXT24((i_var9 + 0x11a));
    pass4_funcs::pass1_1028_e2e0(ctx._PTR_LOOP_1050_65e2);
    local_1e = u_var7;
    u_var8 = local_1c | local_1e;
    if (u_var8 == 0) {
        return;
    }
    pass1_1030_7e5a(
        CONCAT22(local_14, local_16),
        u_var7 & 0xffff | local_1c << 0x10,
    );
    paVar6 = pass4_funcs::pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, local_1e, local_1c);
    _local_22 = CONCAT22(u_var8, paVar6);
    u_var12 = _local_12;
  // u_var13 = (_local_12  >> 0x10);
    iVar10 = *_local_22;
    ppc_var3 = (iVar10 + 4);
    (**ppc_var3)();
    ppc_var3 = (iVar10 + 0x20);
    (**ppc_var3)(0x1030, _local_22, paVar6, u_var8, u_var12, u_var13);
    ppc_var3 = (iVar10 + 0x18);
    (**ppc_var3)(0x1030, _local_22, (_local_22 >> 0x10), 1);
    if ((i_var9 + 0x11a) == 0x37) {
        (_local_22 + 0x20) = (i_var9 + 0x10c);
    }
    (i_var9 + 0x120) = _local_22;
    return;
}

pub fn pass1_1028_8c46(param_1: &mut  Struct794) {
    let pu_var1: &mut  u32;
    let pu_var2: &mut  u32;
    let ctx.ax_reg: &mut  Struct793;
    let mut i_var3: i32;
    let in_dx: &mut  Struct199;
    let local_bx_43: &mut  Struct794;
    let pu_var4: &mut  u32;
    let pu_var5: &mut  u32;
    let mut u_var6: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;

    process_struct_1000_179c(0x124, in_dx);
    _local_a = CONCAT22(in_dx, ctx.ax_reg);
    if ((in_dx | ctx.ax_reg) != 0) {
        *_local_a = ctx.s_1_1050_389a;
        ctx.ax_reg.field_0x2 = &ctx.PTR_LOOP_1050_1008;
      // u_var6 = (param_1  >> 0x10);
        local_bx_43 = param_1;
        ctx.ax_reg.field_0x4 = local_bx_43.field_0x4;
        pu_var4 = &local_bx_43.field_0x8;
        pu_var5 = &ctx.ax_reg.field_0x8;
        i_var3 = 0x40;
        while (i_var3 != 0) {
            i_var3 = i_var3 + -1;
            pu_var2 = pu_var5;
            pu_var5 = pu_var5 + 1;
            pu_var1 = pu_var4;
            pu_var4 = pu_var4 + 1;
            unsafe { *pu_var2 = *pu_var1 };
        }
        *_local_a = 0x6ad2;
        ctx.ax_reg.field_0x2 = &PTR_LOOP_1050_1028;
        ctx.ax_reg.field_0x108 = local_bx_43.field_0x108;
        ctx.ax_reg.field_0x10c = local_bx_43.field_0x10c;
        ctx.ax_reg.field_0x110 = local_bx_43.field_0x110;
        ctx.ax_reg.field_0x114 = local_bx_43.field_0x114;
        ctx.ax_reg.field_0x118 = local_bx_43.field_0x118;
        ctx.ax_reg.field_0x11a = local_bx_43.field_0x11a;
        ctx.ax_reg.field_0x11c = local_bx_43.field_0x11c;
        ctx.ax_reg.field_0x11e = local_bx_43.field_0x11e;
        ctx.ax_reg.field_0x120 = local_bx_43.field_0x120;
        *_local_a = 0x8d8e;
        ctx.ax_reg.field_0x2 = &PTR_LOOP_1050_1028;
    }
    return;
}

pub fn pass1_1028_8d62(param_1: &mut Struct44, param_2: u8) -> &mut Struct44 {
    param_1.ptr_a_lo = ctx.s_1_1050_389a;
    (param_1 + 2) = &ctx.PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1028_8d9e(param_1: &mut  Struct500, param_2: u32, param_3: u32, param_4: u32) {
    let local_bx_19: &mut  Struct795;
    let mut u_var1: u16;

    pass4_funcs::pass1_1028_d1dc(param_1, 0x3e8);
  // u_var1 = (param_1  >> 0x10);
    local_bx_19 = param_1;
    local_bx_19.field_0x108 = param_4;
    local_bx_19.field_0x10c = param_3;
    local_bx_19.field_0x110 = param_2;
    local_bx_19.field_0x114 = 0;
    param_1.a = 0x8fb0;
    local_bx_19.field_0x2 = &PTR_LOOP_1050_1028;
    return;
}

pub fn pass1_1028_8dec(param_1: &mut  Struct796) {
    let local_bx_4: &mut  Struct796;
    let mut u_var1: u16;

  // u_var1 = (param_1  >> 0x10);
    local_bx_4 = param_1;
    param_1 = 0x8fb0;
    local_bx_4.field_0x2 = &PTR_LOOP_1050_1028;
    error_check_1000_17ce(local_bx_4.field_0x114);
    param_1 = ctx.s_1_1050_389a;
    local_bx_4.field_0x2 = &ctx.PTR_LOOP_1050_1008;
    return;
}

pub fn pass1_1028_8e1e(param_1: u32) {
    let mut u_var1: u32;
    let paVar2: &mut  Struct493;
    let mut in_dx: u16;
    let mut u_var3: u16;
    let mut local_6: u16;
    let mut local_4: u16;

  // u_var3 = (param_1  >> 0x10);
    u_var1 = (param_1 + 0x10c);
  // paVar2 = pass4_funcs::pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, u_var1, (u_var1  >> 0x10));
    pass1_1030_355c(paVar2[0x10].field_0x16, (param_1 + 0x114));
    return;
}

pub fn pass1_1028_8e5c(param_1: u32) {
    let mut u_var1: u32;
    let paVar2: &mut  Struct493;
    let struct_a: &mut  Struct393;
    let mut in_dx: u16;
    let mut i_var3: i32;
    let mut u_var4: u16;
    let mut local_6: u16;
    let mut local_4: u16;

  // u_var4 = (param_1  >> 0x10);
    i_var3 = param_1;
    u_var1 = (i_var3 + 0x108);
  // paVar2 = pass4_funcs::pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, u_var1, (u_var1  >> 0x10));
    struct_a = paVar2[0x10].field_0x16;
    pass1_1030_35a4(struct_a, (i_var3 + 0x110));
    (i_var3 + 0x114) = struct_a;
    (i_var3 + 0x116) = in_dx;
    return;
}

pub fn pass1_1028_8ea6(param_1: &mut  Struct798) {
    let pu_var1: &mut  u32;
    let pu_var2: &mut  u32;
    let ctx.ax_reg: &mut  Struct797;
    let mut i_var3: i32;
    let in_dx: &mut  Struct199;
    let local_bx_25: &mut  Struct798;
    let pu_var4: &mut  u32;
    let pu_var5: &mut  u32;
    let mut u_var6: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;

    process_struct_1000_179c(0x118, in_dx);
    _local_a = CONCAT22(in_dx, ctx.ax_reg);
    local_bx_25 = param_1;
  // u_var6 = (param_1  >> 0x10);
    if ((in_dx | ctx.ax_reg) != 0) {
        *_local_a = ctx.s_1_1050_389a;
        ctx.ax_reg.field_0x2 = &ctx.PTR_LOOP_1050_1008;
        ctx.ax_reg.field_0x4 = local_bx_25.field_0x4;
        pu_var4 = &local_bx_25.field_0x8;
        pu_var5 = &ctx.ax_reg.field_0x8;
        i_var3 = 0x40;
        while (i_var3 != 0) {
            i_var3 = i_var3 + -1;
            pu_var2 = pu_var5;
            pu_var5 = pu_var5 + 1;
            pu_var1 = pu_var4;
            pu_var4 = pu_var4 + 1;
            unsafe { *pu_var2 = *pu_var1 };
        }
        *_local_a = 0x6ad2;
        ctx.ax_reg.field_0x2 = &PTR_LOOP_1050_1028;
        ctx.ax_reg.field_0x108 = local_bx_25.field_0x108;
        ctx.ax_reg.field_0x10c = local_bx_25.field_0x10c;
        ctx.ax_reg.field_0x110 = local_bx_25.field_0x110;
        ctx.ax_reg.field_0x114 = local_bx_25.field_0x114;
        *_local_a = 0x8fb0;
        ctx.ax_reg.field_0x2 = &PTR_LOOP_1050_1028;
    }
    local_bx_25.field_0x114 = 0;
    return;
}

pub fn pass1_1028_8f8a(param_1: u32, param_2: u8) {
    pass1_1028_8dec(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1028_8fc0(param_1: &mut  Struct500, param_2: u32, param_3: u32) -> &mut  Struct500 {
    pass1_1028_6af2(param_1, param_2, param_3);
    param_1.a = 0x90d6;
    (param_1 + 2) = &PTR_LOOP_1050_1028;
    return param_1;
}

pub fn pass1_1028_8fea(param_1: &mut  Struct800) {
    let pu_var1: &mut  u32;
    let pu_var2: &mut  u32;
    let ctx.ax_reg: &mut  Struct799;
    let mut i_var3: i32;
    let in_dx: &mut  Struct199;
    let local_bx_43: &mut  Struct800;
    let pu_var4: &mut  u32;
    let pu_var5: &mut  u32;
    let mut u_var6: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;

    process_struct_1000_179c(0x110, in_dx);
    _local_a = CONCAT22(in_dx, ctx.ax_reg);
    if ((in_dx | ctx.ax_reg) != 0) {
        *_local_a = ctx.s_1_1050_389a;
        ctx.ax_reg.field_0x2 = &ctx.PTR_LOOP_1050_1008;
      // u_var6 = (param_1  >> 0x10);
        local_bx_43 = param_1;
        ctx.ax_reg.field_0x4 = local_bx_43.field_0x4;
        pu_var4 = &local_bx_43.field_0x8;
        pu_var5 = &ctx.ax_reg.field_0x8;
        i_var3 = 0x40;
        while (i_var3 != 0) {
            i_var3 = i_var3 + -1;
            pu_var2 = pu_var5;
            pu_var5 = pu_var5 + 1;
            pu_var1 = pu_var4;
            pu_var4 = pu_var4 + 1;
            unsafe { *pu_var2 = *pu_var1 };
        }
        *_local_a = 0x6ad2;
        ctx.ax_reg.field_0x2 = &PTR_LOOP_1050_1028;
        ctx.ax_reg.field_0x108 = local_bx_43.field_0x108;
        ctx.ax_reg.field_0x10c = local_bx_43.field_0x10c;
        *_local_a = 0x6e50;
        ctx.ax_reg.field_0x2 = &PTR_LOOP_1050_1028;
        *_local_a = 0x90d6;
        ctx.ax_reg.field_0x2 = &PTR_LOOP_1050_1028;
    }
    return;
}

pub fn pass1_1028_90aa(param_1: &mut Struct44, param_2: u8) -> &mut Struct44 {
    param_1.ptr_a_lo = ctx.s_1_1050_389a;
    (param_1 + 2) = &ctx.PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1028_90e6(param_1: &mut  Struct500, param_2: u16) -> &mut  Struct500 {
    let mut u_var1: u16;

    pass4_funcs::pass1_1028_d1dc(param_1, 0x1387);
  // u_var1 = (param_1  >> 0x10);
    (param_1 + 0x108) = param_2;
    param_1.a = 0x932c;
    (param_1 + 2) = &PTR_LOOP_1050_1028;
    return param_1;
}

pub fn pass1_1028_9114(param_1: u32) {
    let mut iVar1: i32;
    let mut i_var2: i32;
    let paVar3: &mut  Struct493;
    let paVar4: &mut  Struct1106;


    let mut u_var5: u16;
    let mut u_var6: i32;
    let pp_var7: &mut  Struct2551;
    let pp_var8: &mut  Struct2551;
    let mut u_var9: u16;
    let mut u_var10: u16;
    let mut u_var11: u16;
    let mut u_var12: u16;
    let mut local_16: u16;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    pp_var7 = process_struct_1010_20ba(ctx.g_struct_var_1050_0ed0, CONCAT22(local_16, 0x37));
    u_var6 = param_1;
    i_var2 = (u_var6 + 0x108);
    if (i_var2 - 1 < 8) {
        local_a._0_2_ = *ctx._PTR_LOOP_1050_65e2;
      // iVar1 = (*ctx._PTR_LOOP_1050_65e2  >> 0x10);
        match (i_var2) {
            1 => {
                u_var12 = 0x16;
            }
            2 => {
                u_var12 = 0x17;
            }
            3 => {
                u_var12 = 0x18;
            }
            4 => {
                u_var12 = 0x1b;
            }
            5 => {
                u_var12 = 0x1f;
            }
            6 => {
                u_var12 = 0x24;
            }
            7 => {
                pass1_fn_1008_612e(0, 0x14);
                u_var12 = local_a + u_var6 + 0x6e;
                u_var9 =
                    iVar1 + (u_var6 >> 0xf) + (0xff91 < u_var6) + CARRY2(local_a, u_var6 + 0x6e);
                u_var11 = 7;
                pp_var8 =
                    process_struct_1010_20ba(ctx.g_struct_var_1050_0ed0, CONCAT22(u_var12, 0x2f));
                i_var2 = pp_var8;
                pass1_1010_ebf8(pp_var8, u_var12, u_var9, u_var11);
                u_var5 = ctx.dx_reg;
                pass1_fn_1008_612e(1, 100);
                if (0x32 < i_var2) {
                    return;
                }
                paVar3 = pass4_funcs::pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, 1, 0x400);
                pass1_1038_4900(CONCAT22(u_var5, paVar3));
                u_var12 = 0x2c;
            }
            8 => {
                pass1_fn_1008_612e(0, 0x14);
                u_var12 = local_a + u_var6 + 100;
                u_var9 =
                    iVar1 + (u_var6 >> 0xf) + (0xff9b < u_var6) + CARRY2(local_a, u_var6 + 100);
                u_var11 = 8;
                pp_var8 =
                    process_struct_1010_20ba(ctx.g_struct_var_1050_0ed0, CONCAT22(u_var12, 0x2f));
                pass1_1010_ebf8(pp_var8, u_var12, u_var9, u_var11);
                if (0x19 < u_var6) {
                    return;
                }
                u_var10 = 1;
                u_var12 = 2;
                u_var5 = ctx.dx_reg;
                paVar4 = pass4_funcs::pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, 1, 0x400);
                pass1_1038_43cc(paVar4, CONCAT22(u_var10, u_var5), u_var12);
                u_var12 = 0x2d;
            }
        }
        post_win_msg_1008_a0e4(pp_var7, 0, 0, 1, 0, u_var12);
    }
    return;
}

pub fn pass1_1028_9264(param_1: u32) {
    let pu_var1: &mut  u32;
    let pu_var2: &mut  u32;
    let mut in_ax: i32;
    let pu_var3: &mut  u32;
    let mut i_var4: i32;
    let in_dx: &mut  Struct199;
    let mut i_var5: i32;
    let pu_var6: &mut  u32;
    let mut u_var7: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;

    process_struct_1000_179c(0x10a, in_dx);
    _local_a = CONCAT22(in_dx, in_ax);
    if ((in_dx | in_ax) != 0) {
        *_local_a = ctx.s_1_1050_389a;
        (in_ax + 2) = &ctx.PTR_LOOP_1050_1008;
      // u_var7 = (param_1  >> 0x10);
        i_var5 = param_1;
        (in_ax + 4) = (i_var5 + 4);
        pu_var3 = (i_var5 + 8);
        pu_var6 = (in_ax + 8);
        i_var4 = 0x40;
        while (i_var4 != 0) {
            i_var4 = i_var4 + -1;
            pu_var2 = pu_var6;
            pu_var6 = pu_var6 + 1;
            pu_var1 = pu_var3;
            pu_var3 = pu_var3 + 1;
            unsafe { *pu_var2 = *pu_var1 };
        }
        *_local_a = 0x6ad2;
        (in_ax + 2) = &PTR_LOOP_1050_1028;
        (in_ax + 0x108) = (i_var5 + 0x108);
        *_local_a = 0x932c;
        (in_ax + 2) = &PTR_LOOP_1050_1028;
    }
    return;
}

pub fn pass1_1028_9300(param_1: &mut Struct44, param_2: u8) -> &mut Struct44 {
    param_1.ptr_a_lo = ctx.s_1_1050_389a;
    (param_1 + 2) = &ctx.PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1028_83b4() {
    let mut unaff_ss: u16;
    let lVar1: u32;
    let mut local_22: u16;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u16;

    pass4_funcs::pass1_1028_dc52(
        CONCAT22(unaff_ss, &local_14),
        (&ctx.PTR_LOOP_1050_0000 + 1),
        0,
        0x400,
    );
    while (true) {
        lVar1 = pass4_funcs::pass1_1028_e4ec(CONCAT22(unaff_ss, &local_14));
        if (lVar1 == 0) {
            break;
        }
        (lVar1 + 0x206) = 1;
    }
    return 1;
}

pub fn pass1_1028_8400(param_1: u32) {
    let pu_var1: &mut  u32;
    let pu_var2: &mut  u32;
    let mut in_ax: i32;
    let pu_var3: &mut  u32;
    let mut i_var4: i32;
    let in_dx: &mut  Struct199;
    let pu_var5: &mut  u32;
    let mut u_var6: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;

    process_struct_1000_179c(0x108, in_dx);
    _local_a = CONCAT22(in_dx, in_ax);
    if ((in_dx | in_ax) != 0) {
        *_local_a = ctx.s_1_1050_389a;
        (in_ax + 2) = &ctx.PTR_LOOP_1050_1008;
      // u_var6 = (param_1  >> 0x10);
        (in_ax + 4) = (param_1 + 4);
        pu_var3 = (param_1 + 8);
        pu_var5 = (in_ax + 8);
        i_var4 = 0x40;
        while (i_var4 != 0) {
            i_var4 = i_var4 + -1;
            pu_var2 = pu_var5;
            pu_var5 = pu_var5 + 1;
            pu_var1 = pu_var3;
            pu_var3 = pu_var3 + 1;
            unsafe { *pu_var2 = *pu_var1 };
        }
        *_local_a = 0x6ad2;
        (in_ax + 2) = &PTR_LOOP_1050_1028;
        *_local_a = 0x84ba;
        (in_ax + 2) = &PTR_LOOP_1050_1028;
    }
    return;
}

pub fn pass1_1028_848e(param_1: &mut Struct44, param_2: u8) -> &mut Struct44 {
    param_1.ptr_a_lo = ctx.s_1_1050_389a;
    (param_1 + 2) = &ctx.PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1028_84ca(
    param_1: &mut  Struct500,
    param_2: u32,
    param_3: u16,
    param_4: u16,
    param_5: u16,
) -> i32 {
    let pc_var1: String;
    let mut i_var2: i32;
    let pcVar3: String;

    pass4_funcs::pass1_1028_d1dc(param_1, 0x3e7);
  // pcVar3 = (param_1  >> 0x10);
    i_var2 = param_1;
    (i_var2 + 0x108) = param_5;
    (i_var2 + 0x10a) = param_4;
    (i_var2 + 0x10c) = param_3;
    (i_var2 + 0x10e) = param_2;
    param_1.a = 0x8688;
    (i_var2 + 2) = &PTR_LOOP_1050_1028;
    if ((i_var2 + 0x108) == 1) {
        pc_var1 = s_max_1050_501c;
    } else {
        pc_var1 = s_min_1050_5020;
    }
    string_fn_1000_3f9c(
        (i_var2 + 8),
        pcVar3,
        s_SCForceMorale__s_for_colony__08l_1050_5024,
        &ctx.g_alloc_addr_1050_1050,
        pc_var1,
    );
    return;
}

pub fn pass1_1028_853e(param_1: u32) {
    let mut u_var1: u32;
    let paVar2: &mut  Struct493;
    let mut u_var3: u16;
    let mut in_dx: u16;
    let mut i_var4: i32;
    let mut u_var5: u16;
    let mut local_6: u16;

  // u_var5 = (param_1  >> 0x10);
    i_var4 = param_1;
    if ((i_var4 + 0x108) == 0) {
        return 0;
    }
    u_var1 = (i_var4 + 0x10e);
  // paVar2 = pass4_funcs::pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, u_var1, (u_var1  >> 0x10));
    if ((i_var4 + 0x108) == 1) {
        u_var3 = 1000;
    } else {
        u_var3 = 0;
    }
    pass1_1038_4d0e(CONCAT22(in_dx, paVar2), u_var3);
    return 1;
}

pub fn pass1_1028_858c(param_1: u32) {
    let pu_var1: &mut  u32;
    let pu_var2: &mut  u32;
    let mut in_ax: i32;
    let pu_var3: &mut  u32;
    let mut i_var4: i32;
    let in_dx: &mut  Struct199;
    let mut i_var5: i32;
    let pu_var6: &mut  u32;
    let mut u_var7: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;

    process_struct_1000_179c(0x112, in_dx);
    _local_a = CONCAT22(in_dx, in_ax);
    if ((in_dx | in_ax) != 0) {
        *_local_a = ctx.s_1_1050_389a;
        (in_ax + 2) = &ctx.PTR_LOOP_1050_1008;
      // u_var7 = (param_1  >> 0x10);
        i_var5 = param_1;
        (in_ax + 4) = (i_var5 + 4);
        pu_var3 = (i_var5 + 8);
        pu_var6 = (in_ax + 8);
        i_var4 = 0x40;
        while (i_var4 != 0) {
            i_var4 = i_var4 + -1;
            pu_var2 = pu_var6;
            pu_var6 = pu_var6 + 1;
            pu_var1 = pu_var3;
            pu_var3 = pu_var3 + 1;
            unsafe { *pu_var2 = *pu_var1 };
        }
        *_local_a = 0x6ad2;
        (in_ax + 2) = &PTR_LOOP_1050_1028;
        (in_ax + 0x108) = (i_var5 + 0x108);
        (in_ax + 0x10a) = (i_var5 + 0x10a);
        (in_ax + 0x10c) = (i_var5 + 0x10c);
        (in_ax + 0x10e) = (i_var5 + 0x10e);
        *_local_a = 0x8688;
        (in_ax + 2) = &PTR_LOOP_1050_1028;
    }
    return;
}

pub fn pass1_1028_865c(param_1: &mut Struct44, param_2: u8) -> &mut Struct44 {
    param_1.ptr_a_lo = ctx.s_1_1050_389a;
    (param_1 + 2) = &ctx.PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1028_8698(param_1: &mut  Struct500, param_2: u32, param_3: u32) -> &mut  Struct500 {
    pass1_1028_6af2(param_1, param_2, param_3);
    param_1.a = 0x87e0;
    (param_1 + 2) = &PTR_LOOP_1050_1028;
    return param_1;
}

pub fn pass1_1028_86c2(param_1: u32) -> u8 {
    let mut u_var1: u16;
    let ppVar2: &mut  Struct2551;
    let mut u_var3: u16;
    let mut u_var4: u16;
    let mut u_var5: u16;
    let mut u_var6: u16;
    let mut u_var7: u16;
    let mut u_var8: u16;

    u_var7 = 0;
    u_var8 = 0x1d;
    u_var5 = 1;
    u_var6 = 0;
    u_var4 = 0;
    u_var1 = 0;
    u_var3 = 0;
    ppVar2 = process_struct_1010_20ba(ctx.g_struct_var_1050_0ed0, 0x37);
    post_win_msg_1008_a0e4(
        ppVar2,
        CONCAT22(u_var4, u_var3),
        u_var1,
        u_var5,
        CONCAT22(u_var7, u_var6),
        u_var8,
    );
    u_var1 = pass1_1028_6b2c(param_1);
    return u_var1;
}

pub fn pass1_1028_86f4(param_1: u32) {
    let pu_var1: &mut  u32;
    let pu_var2: &mut  u32;
    let mut in_ax: i32;
    let pu_var3: &mut  u32;
    let mut i_var4: i32;
    let in_dx: &mut  Struct199;
    let mut i_var5: i32;
    let pu_var6: &mut  u32;
    let mut u_var7: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;

    process_struct_1000_179c(0x110, in_dx);
    _local_a = CONCAT22(in_dx, in_ax);
    if ((in_dx | in_ax) == 0) {
        local_6._0_2_ = 0;
    } else {
        *_local_a = ctx.s_1_1050_389a;
        (in_ax + 2) = &ctx.PTR_LOOP_1050_1008;
      // u_var7 = (param_1  >> 0x10);
        i_var5 = param_1;
        (in_ax + 4) = (i_var5 + 4);
        pu_var3 = (i_var5 + 8);
        pu_var6 = (in_ax + 8);
        i_var4 = 0x40;
        while (i_var4 != 0) {
            i_var4 = i_var4 + -1;
            pu_var2 = pu_var6;
            pu_var6 = pu_var6 + 1;
            pu_var1 = pu_var3;
            pu_var3 = pu_var3 + 1;
            unsafe { *pu_var2 = *pu_var1 };
        }
        *_local_a = 0x6ad2;
        (in_ax + 2) = &PTR_LOOP_1050_1028;
        (in_ax + 0x108) = (i_var5 + 0x108);
        (in_ax + 0x10c) = (i_var5 + 0x10c);
        *_local_a = 0x6e50;
        (in_ax + 2) = &PTR_LOOP_1050_1028;
        *_local_a = 0x87e0;
        (in_ax + 2) = &PTR_LOOP_1050_1028;
        local_6._0_2_ = in_ax;
    }
    return local_6;
}

pub fn pass1_1028_87b4(param_1: &mut Struct44, param_2: u8) -> &mut Struct44 {
    param_1.ptr_a_lo = ctx.s_1_1050_389a;
    (param_1 + 2) = &ctx.PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1028_81e0() {
    let mut iVar1: i32;
    let ppc_var2: fn();
    let mut u_var3: u16;
    let mut unaff_cs: u16;
    let mut unaff_ss: u16;
    let pu_var4: &mut  u32;
    let mut local_24: u16;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u16;

    pass4_funcs::pass1_1028_dc52(
        CONCAT22(unaff_ss, &local_14),
        (&ctx.PTR_LOOP_1050_0000 + 1),
        0,
        0x700,
    );
    // switchD_1028_8225_caseD_0:
    while {
        loop {
            pu_var4 = pass4_funcs::pass1_1028_e4ec(CONCAT22(unaff_ss, &local_14));
          // u_var3 = (pu_var4  >> 0x10);
            if (pu_var4 == 0x0) {
                return 1;
            }
            iVar1 = (pu_var4 + 0xc);
            if (iVar1 < 0x35) {}
            // goto code_r0x10288222;
            if (0x61 < iVar1) {
                break;
            }
            if ((iVar1 < 0x5d) && (iVar1 != 0x37 && (iVar1 != 0x47))) {}
            // goto switchD_1028_8225_caseD_1;
        }
        (iVar1 == 0x6a)
            || (8 < iVar1 + -0x6a
                && (iVar1 == 0x75
                    || iVar1 + -0x74 < 1
                    || (0 < iVar1 + -0x76 && (iVar1 + -0x78 < 2))))
    } {}
    // goto switchD_1028_8225_caseD_1;
    // code_r0x10288222:
    unaff_cs = SUB42(&PTR_LOOP_1050_1028, 0);
    match (iVar1) {
        1 | 2 | 3 | 4 | 6 | 7 | 8 | 10 | 0xb | 0xc | 0xd | 0xe | 0xf | 0x11 => {
            // switchD_1028_8225_caseD_1:
            if ((pu_var4 + 0x12) == 5) {
                unsafe {
                    ppc_var2 = (*pu_var4 + 0x30);
                    ppc_var2(unaff_cs);
                }
            }
        } // goto switchD_1028_8225_caseD_0;
    }
}

pub fn pass1_1028_82b4(param_1: u32) {
    let pu_var1: &mut  u32;
    let pu_var2: &mut  u32;
    let mut in_ax: i32;
    let pu_var3: &mut  u32;
    let mut i_var4: i32;
    let in_dx: &mut  Struct199;
    let pu_var5: &mut  u32;
    let mut u_var6: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;

    process_struct_1000_179c(0x108, in_dx);
    _local_a = CONCAT22(in_dx, in_ax);
    if ((in_dx | in_ax) != 0) {
        *_local_a = ctx.s_1_1050_389a;
        (in_ax + 2) = &ctx.PTR_LOOP_1050_1008;
      // u_var6 = (param_1  >> 0x10);
        (in_ax + 4) = (param_1 + 4);
        pu_var3 = (param_1 + 8);
        pu_var5 = (in_ax + 8);
        i_var4 = 0x40;
        while (i_var4 != 0) {
            i_var4 = i_var4 + -1;
            pu_var2 = pu_var5;
            pu_var5 = pu_var5 + 1;
            pu_var1 = pu_var3;
            pu_var3 = pu_var3 + 1;
            unsafe { *pu_var2 = *pu_var1 };
        }
        *_local_a = 0x6ad2;
        (in_ax + 2) = &PTR_LOOP_1050_1028;
        *_local_a = 0x836e;
        (in_ax + 2) = &PTR_LOOP_1050_1028;
    }
    return;
}

pub fn pass1_1028_8342(param_1: &mut Struct44, param_2: u8) -> &mut Struct44 {
    param_1.ptr_a_lo = ctx.s_1_1050_389a;
    (param_1 + 2) = &ctx.PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn post_msg_1028_76da() {
    let lVar1: u32;
    let mut u_var2: u16;
    let ppVar3: &mut  Struct2551;
    let mut in_stack_0000ffe4: u32;
    let mut local_e: u32;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    ppVar3 = process_struct_1010_20ba(
        ctx.g_struct_var_1050_0ed0,
        CONCAT22((in_stack_0000ffe4 >> 0x10), 0x2c),
    );
  // u_var2 = (ppVar3  >> 0x10);
    lVar1 = (ppVar3 + 0xc);
    local_a = (lVar1 >> 0x10);
    local_a._0_2_ = lVar1;
    if (((local_a | local_a) != 0) && (*ctx._PTR_LOOP_1050_65e2 == lVar1)) {
        PostMessage16(0, 0x106, 0x111, ctx.g_h_window);
        (ppVar3 + 0xc) = 0;
    }
    return;
}

pub fn pass1_1028_7742(param_1: u16, param_2: u16, param_1_00: i32, param_2_00: &mut Struct44) {
    let pp_var1: fn();
    let u_var2: u8;
    let mut u_var3: u16;
    let mut u_var4: u16;
    let pu_var5: Vec<u8>;
    let pu_var6: Vec<u8>;
    let mut u_var7: u32;
    let extraout_var: u32;
    let mut in_dx: u16;



    let mut u_var8: i32;
    let mut unaff_ss: u16;
    let paVar9: &mut  Struct1115;
    let u_var10: u8;
    let u_var11: u8;
    let mut u_var12: i32;
    let mut u_var13: u16;
    let mut local_28: u32;
    let mut local_24: u32;
    let mut local_20: u16;
    let mut local_1e: u16;
    let mut local_1c: u16;
    let mut local_1a: u32;
    let mut local_16: [u8; 2];
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    pu_var6 = pass1_1008_c6fa(ctx._PTR_LOOP_1050_06e0, 0x18);
    u_var3 = pu_var6;
    local_6 = u_var3;
    paVar9 = pass4_funcs::pass1_1028_b4f2(param_2_00);
  // local_8 = (paVar9  >> 0x10);
    u_var4 = paVar9;
    local_a = u_var4;
    pass1_1038_4d6e(paVar9, CONCAT22(in_dx, u_var3));
    _local_e = CONCAT22(ctx.dx_reg, u_var4);
    local_10 = 0;
    pp_var1 = (*_local_e + 0x10);
    u_var3 = u_var4;
    u_var13 = ctx.dx_reg;
    (**pp_var1)(&PTR_LOOP_1050_1038, u_var4, ctx.dx_reg);
    _local_14 = CONCAT22(ctx.dx_reg, u_var4);
    pass1_1030_bcae(local_16, unaff_ss);
    local_1a = 0;
    loop {
        if (_local_14 <= local_1a) {
            // LAB_1028_77e7:
            if (_local_e != 0x0) {
                pp_var1 = *_local_e;
                (**pp_var1)(
                    0x1030,
                    _local_e,
                    (_local_e >> 0x10),
                    1,
                    u_var3,
                    u_var13,
                    _local_e,
                    _local_e,
                );
            }
            return;
        }
        u_var7 = _local_14;
        pass1_1030_1d58(_local_e);
        u_var10 = u_var7;
        u_var11 = (u_var7 >> 8);
        u_var8 = ctx.dx_reg;
        u_var12 = ctx.dx_reg;
        u_var2 = pass4_funcs::pass1_1028_b58e(param_2_00);
        pu_var5 = local_16;
        pass1_1030_bd74(
            pu_var5,
            unaff_ss,
            (CONCAT31(extraout_var, u_var2) & 0xffff | u_var8 << 0x10),
            CONCAT22(u_var12, CONCAT11(u_var11, u_var10)),
        );
        if (pu_var5 <= param_1_00) {
            local_10 = 1;
            // goto LAB_1028_77e7;
        }
        local_1a = local_1a + 1;
    }
}

pub fn pass1_1028_780c(param_1: u16, param_2: u16, param_1_00: u32) {
    let pp_var1: fn();
    let mut u_var2: i32;
    let mut u_var3: i32;
    let paVar4: &mut  Struct493;
    let pu_var5: Vec<u8>;
    let mut u_var6: u32;
    let mut in_dx: i32;



    let mut u_var7: u16;
    let mut u_var8: u16;
    let pu_var9: &mut  u32;
    let mut local_1e: u16;
    let mut local_1c: u16;
    let mut local_12: u32;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;

    pu_var5 = pass1_1008_c6fa(ctx._PTR_LOOP_1050_06e0, 0x25);
    u_var2 = pu_var5;
    u_var8 = SUB42(&PTR_LOOP_1050_1038, 0);
    pass1_1038_4e78(param_1_00, pu_var5 & 0xffff | in_dx << 0x10);
    _local_a = CONCAT22(ctx.dx_reg, u_var2);
    pp_var1 = (*_local_a + 0x10);
    u_var3 = u_var2;
    (**pp_var1)(&PTR_LOOP_1050_1038, u_var2, ctx.dx_reg);
    _local_e = CONCAT22(ctx.dx_reg, u_var3);
    if ((ctx.dx_reg | u_var3) == 0) {
        return;
    }
    local_12 = 0;
    while (local_12 < _local_e) {
        pp_var1 = (*_local_a + 4);
        u_var6 = _local_e;
        (**pp_var1)();
        u_var7 = ctx.dx_reg;
        paVar4 = pass4_funcs::pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, u_var6, ctx.dx_reg);
        u_var8 = 0x1030;
        pu_var9 = pass1_1030_73a8(CONCAT22(u_var7, paVar4));
        unsafe { pp_var1 = (*pu_var9 + 0x24) };
        (**pp_var1)();
        local_12 = local_12 + 1;
    }
    if (_local_a != 0x0) {
        pp_var1 = *_local_a;
        (**pp_var1)(u_var8, u_var2, ctx.dx_reg, 1);
    }
    return;
}

pub fn pass1_1028_78b8(param_1: u32) {
    let paVar1: &mut  Struct493;
    let pu_var2: &mut  u32;
    let paVar3: &mut  Struct1106;
    let pu_var4: &mut  u16;
    let pu_var5: &mut  u16;
    let pu_var6: &mut  u16;
    let mut in_dx: u16;
    let mut u_var7: i32;
    let mut u_var8: i32;
    let mut i_var9: i32;
    let mut u_var10: u32;
    let mut unaff_si: u16;
    let mut unaff_ss: u16;
    let mut bVar11: bool;
    let mut bVar12: bool;
    let ppVar13: &mut  Struct2551;
    let u_var14: u8;
    let u_var15: u8;
    let mut u_var16: u16;
    let mut u_var17: u16;
    let mut u_var18: u16;
    let mut u_var19: u16;
    let mut uStack340: u16;
    let mut uStack338: u16;
    let mut local_4a: u16;
    let mut local_48: u16;
    let mut uStack70: u16;
    let mut u_stack68: i32;
    let mut local_42: u16;
    let mut local_40: u16;
    let mut local_30: u16;
    let mut local_2e: u16;
    let mut local_1e: u16;
    let mut local_18: u32;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut u_stack12: u16;
    let mut uStack10: u16;
    let mut uStack8: u16;
    let mut local_6: u32;

    local_6 = *ctx._PTR_LOOP_1050_65e2;
    u_var14 = unaff_ss;
    u_var15 = (unaff_ss >> 8);
    if (local_6 == 0x98) {
        paVar1 = pass4_funcs::pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, 2, 0x400);
        _local_10 = CONCAT22(in_dx, paVar1);
        if (&paVar1[0x11].field_0x2 == 0x8000002) {
            pass1_1020_a43e(CONCAT22(unaff_ss, &local_18));
            zero_list_1008_3e38(CONCAT22(unaff_ss, &local_1e));
            pu_var2 = &local_18;
            pass1_1020_a49a(
                CONCAT13(u_var15, CONCAT12(u_var14, pu_var2)),
                CONCAT22(unaff_ss, &local_1e),
                0x7a,
            );
            pass1_1038_4f54(_local_10, 1);
            if (pu_var2 == 0x0) {
                pass1_1020_a49a(CONCAT13(u_var15, CONCAT12(u_var14, &local_18)), 0, 0x35);
            }
        }
    }
    if ((0xe < local_6) && (local_6 < 0x16)) {
        pass1_1020_a43e(CONCAT22(unaff_ss, &local_1e));
        local_18 = local_6 - 0xf;
        pass1_1020_a54c(&local_1e, unaff_ss, local_18);
    }
    u_var10 = local_6 % 0x7d;
    u_var8 = u_var10;
    if (u_var10 == 0) {
        local_1e = u_var8;
        pass1_fn_1008_612e(1, 100);
        u_var8 = u_var10;
        if (local_1e < 0x1a) {
            pass4_funcs::pass1_1028_dc52(
                CONCAT13(u_var15, CONCAT12(u_var14, &local_30)),
                (&ctx.PTR_LOOP_1050_0000 + 1),
                0,
                0x400,
            );
            while {
                u_var7 = u_var10;
                paVar3 = &local_30;
                pass4_funcs::pass1_1028_e4ec(CONCAT22(unaff_ss, paVar3));
                local_18 = CONCAT22(u_var7, paVar3);
                u_var8 = u_var7 | paVar3;
                u_var10 = u_var8;
                if (u_var8 == 0) {}
                // goto LAB_1028_79d6;
                &paVar3[1].field_0xb0 == 0x8000002
            } {}
            pass1_1038_43cc(paVar3, CONCAT22(1, u_var7), 4);
            // LAB_1028_79d6:
            local_30 = ctx.s_1_1050_389a;
            local_2e = &ctx.PTR_LOOP_1050_1008;
        }
    }
    if (local_6 == 5) {
        u_var17 = SUB42(&ctx.g_alloc_addr_1050_1050, 0);
        u_var16 = SUB42(s_Rebel_1050_4ffc, 0);
        local_30 = pass4_funcs::pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, 2, 0x400);
        local_2e = u_var8;
        pass1_1038_4d3c(CONCAT22(u_var8, local_30), CONCAT22(u_var17, u_var16));
    }
    if (local_6 == 300) {
        u_var17 = 0x400;
        u_var19 = 0xf;
        u_var16 = 1;
        ppVar13 = process_struct_1010_20ba(ctx.g_struct_var_1050_0ed0, 0x1002b);
      // u_var8 = (ppVar13  >> 0x10);
        local_30 = ppVar13;
        local_2e = u_var8;
        pass1_1010_043a(ppVar13, CONCAT22(u_var17, u_var16), u_var19);
    }
    if (local_6 == 0x3d) {
        ppVar13 = process_struct_1010_20ba(ctx.g_struct_var_1050_0ed0, CONCAT22(unaff_si, 2));
      // u_var10 = ppVar13  >> 0x10;
        local_30 = ppVar13;
      // u_var8 = (ppVar13  >> 0x10);
        local_1e = u16_1050_13ae;
        local_2e = u_var8;
        if (u16_1050_13ae != 1) {
            pass4_funcs::pass1_1028_dc52(
                CONCAT13(u_var15, CONCAT12(u_var14, &local_42)),
                (&ctx.PTR_LOOP_1050_0000 + 1),
                0,
                0x400,
            );
            while (true) {
                u_var8 = u_var10;
                pu_var4 = &local_42;
                pass4_funcs::pass1_1028_e4ec(CONCAT22(unaff_ss, pu_var4));
                local_18 = CONCAT22(u_var8, pu_var4);
                u_var10 = (u_var8 | pu_var4);
                if ((u_var8 | pu_var4) == 0) {
                    break;
                }
                _local_10 = (pu_var4 + 0xfb);
                pass1_1030_34da(_local_10);
            }
            u_var17 = 0x400;
            u_var19 = 0x10;
            u_var16 = 1;
            ppVar13 = process_struct_1010_20ba(ctx.g_struct_var_1050_0ed0, 0x1002b);
          // u_var8 = (ppVar13  >> 0x10);
            local_14 = ppVar13;
            local_12 = u_var8;
            pass1_1010_043a(ppVar13, CONCAT22(u_var17, u_var16), u_var19);
            _local_42 = 0x1008389a;
        }
    }
    if (local_6 == 0x96) {
        paVar1 = pass4_funcs::pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, 1, 0x400);
        local_4a = CONCAT22(u_var8, paVar1);
      // u_var19 = (param_1  >> 0x10);
        pass1_1028_780c(param_1, u_var19, CONCAT22(u_var8, paVar1));
        if (paVar1 != 0x0) {
            u_var17 = 0x400;
            u_var18 = 0x1d;
            u_var16 = 1;
            ppVar13 = process_struct_1010_20ba(ctx.g_struct_var_1050_0ed0, 0x1002b);
          // u_var8 = (ppVar13  >> 0x10);
            uStack70 = ppVar13;
            u_stack68 = u_var8;
            pass1_1010_043a(ppVar13, CONCAT22(u_var17, u_var16), u_var18);
        }
        paVar1 = pass4_funcs::pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, 2, 0x400);
        local_4a = CONCAT22(u_var8, paVar1);
        pass1_1028_780c(param_1, u_var19, CONCAT22(u_var8, paVar1));
    }
    ppVar13 = process_struct_1010_20ba(ctx.g_struct_var_1050_0ed0, CONCAT22(unaff_si, 2));
  // uStack8 = (ppVar13  >> 0x10);
    uStack10 = SUB42(ppVar13, 0);
    u_stack12 = u16_1050_13ae;
    if (2 < u16_1050_13ae) {
        local_4a = process_struct_1010_20ba(ctx.g_struct_var_1050_0ed0, CONCAT22(unaff_si, 0x2f));
        uStack70 = 1;
        while (uStack70 < 9) {
            _local_42 = (local_4a + 0x34 + uStack70 * 4);
            if (_local_42 == local_6) {
                u_var19 = 1;
                local_30 = 1;
                pass1_fn_1008_612e(1, 100);
                pu_var6 = (uStack70 - 7);
                if (pu_var6 == 0x0) {
                    bVar12 = SBORROW2(u_var19, 0x32);
                    i_var9 = u_var19 - 0x32;
                    bVar11 = u_var19 == 0x32;
                    // LAB_1028_7b74:
                    if ((!bVar11 && bVar12) == (i_var9 < 0)) {
                        local_30 = 0;
                    }
                } else {
                    pu_var6 = (uStack70 - 8);
                    if (pu_var6 == 0x0) {
                        bVar12 = SBORROW2(u_var19, 0x19);
                        i_var9 = u_var19 - 0x19;
                        bVar11 = i_var9 == 0;
                        // goto LAB_1028_7b74;
                    }
                }
                local_1e = u_var19;
                if (local_30 != 0) {
                    pass1_1028_90e6(CONCAT22(unaff_ss, &uStack340), uStack70);
                    pu_var6 = &uStack340;
                    pass1_1030_835a(ctx._g_bool_1050_5748, CONCAT22(unaff_ss, pu_var6));
                    uStack340 = SUB42(ctx.s_1_1050_389a, 0);
                    uStack338 = SUB42(&ctx.PTR_LOOP_1050_1008, 0);
                }
                pass1_fn_1008_612e(0, 10);
                local_18 = local_18 & 0xffff0000 | ZEXT24(pu_var6);
                if (uStack70 == 7) {
                    u_var19 = 7;
                    pu_var5 = pu_var6 + 0x37;
                    i_var9 = pu_var5 >> 0xf;
                } else {
                    if (uStack70 != 8) {}
                    // goto LAB_1028_7ba0;
                    u_var19 = 8;
                    pu_var5 = pu_var6 + 0x32;
                    i_var9 = (pu_var6 >> 0xf) + (0xff9b < pu_var6);
                }
              // u_var18 = (_local_42  >> 0x10) + i_var9 + CARRY2(local_42, pu_var5);
                _local_42 = CONCAT22(u_var18, local_42 + pu_var5);
                pass1_1010_ebf8(local_4a, local_42 + pu_var5, u_var18, u_var19);
            }
            // LAB_1028_7ba0:
            uStack70 = uStack70 + 1;
        }
    }
    return;
}

pub fn pass1_1028_7c4e(param_1: u32) {
    let mut u_var1: u32;
    let mut u_var2: u32;
    let ppc_var3: fn();
    let pu_var4: &mut  u16;
    let mut u_var5: u16;
    let mut u_var6: u16;
    let pu_var7: Vec<u8>;
    let mut u_var8: u32;
    let mut u_var9: i32;
    let mut u_var10: u32;
    let u_var11: u8;
    let mut unaff_ss: u16;
    let ppVar12: &mut  Struct2551;
    let mut u_var13: u16;
    let mut local_176: u32;
    let mut local_172: u32;
    let mut local_156: u16;
    let mut local_154: u16;
    let mut local_46: u16;
    let mut local_44: u16;
    let mut local_42: u16;
    let mut local_40: u32;
    let mut local_38: u32;
    let mut local_34: u16;
    let mut local_32: u16;
    let mut local_30: u16;
    let mut local_2e: u16;
    let mut local_2c: u16;
    let mut local_2a: u16;
    let mut local_28: u16;
    let mut local_26: u16;
    let mut local_24: u16;
    let mut local_22: u16;
    let mut local_20: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u32;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    ppVar12 = process_struct_1010_20ba(ctx.g_struct_var_1050_0ed0, CONCAT22(local_176, 2));
  // local_4 = (ppVar12  >> 0x10);
    local_6 = ppVar12;
    local_8 = u16_1050_13ae;
    if (2 < u16_1050_13ae) {
        local_c = *ctx._PTR_LOOP_1050_65e2;
        local_c = (local_c >> 0x10);
        if (2 < local_c) {
            local_10 = local_c - 2;
            local_e = local_c - (local_c < 2);
            u_var10 = CONCAT22(local_e, local_10) % 0x14;
            if (u_var10 == 0) {
                pass4_funcs::pass1_1028_dc52(
                    CONCAT13((unaff_ss >> 8), CONCAT12(unaff_ss, &local_22)),
                    (&ctx.PTR_LOOP_1050_0000 + 1),
                    0,
                    0x400,
                );
                while (true) {
                    u_var9 = u_var10;
                    pu_var4 = &local_22;
                    pass4_funcs::pass1_1028_e4ec(CONCAT22(unaff_ss, pu_var4));
                    _local_26 = CONCAT22(u_var9, pu_var4);
                    u_var10 = (u_var9 | pu_var4);
                    if ((u_var9 | pu_var4) == 0) {
                        break;
                    }
                    if ((pu_var4 + 0x100) != 0x8000002) {
                        pu_var7 = pass1_1008_c6fa(ctx._PTR_LOOP_1050_06e0, 0x2a);
                        u_var5 = pu_var7;
                        local_28 = u_var10;
                        u_var11 = 0x38;
                        local_2a = u_var5;
                        pass1_1038_4d6e(_local_26, pu_var7 & 0xffff | u_var10 << 0x10);
                        _local_2e = CONCAT22(u_var10, u_var5);
                        ppc_var3 = (*_local_2e + 0x10);
                        (**ppc_var3)(&PTR_LOOP_1050_1038, u_var5, u_var10);
                        _local_32 = CONCAT22(u_var10, u_var5);
                        if (local_8 == 3) {
                            local_34 = 6;
                        } else {
                            local_34 = 0xc;
                        }
                        local_38 = 0;
                        while// u_var13 = (_local_2e  >> 0x10), local_38 < _local_32) {
                            u_var8 = _local_32;
                            pass1_1030_1d7c(_local_2e, u_var13, local_38, (local_38 >> 0x10));
                            u_var6 = u_var8;
                            local_40 = u_var8 & 0xffff | u_var10 << 0x10;
                            pass1_1028_7742(
                                param_1,
                                (param_1 >> 0x10),
                                4,
                                (u_var8 & 0xffff | u_var10 << 0x10),
                            );
                            u_var5 = local_34;
                            if (u_var6 == 0) {
                                u_var5 = 0x19;
                            }
                            u_var11 = 8;
                            local_44 = u_var5;
                            local_42 = u_var6;
                            pass1_fn_1008_612e(1, 100);
                            local_46 = u_var5;
                            if (u_var5 <= local_44) {
                                u_var1 = (_local_26 + 4);
                                u_var2 = (local_40 + 4);
                                pass1_1028_8fc0(
                                    &local_156,
                                    unaff_ss,
                                    u_var2,
                                    (u_var2 >> 0x10),
                                    u_var1,
                                    (u_var1 >> 0x10),
                                );
                                u_var11 = 0x30;
                                pass1_1030_835a(
                                    ctx._g_bool_1050_5748,
                                    CONCAT22(unaff_ss, &local_156),
                                );
                                local_156 = ctx.s_1_1050_389a;
                                local_154 = &ctx.PTR_LOOP_1050_1008;
                            }
                            local_38 = local_38 + 1;
                        }
                        local_176._0_2_ = SUB42(_local_2e, 0);
                        if (_local_2e != 0x0) {
                            ppc_var3 = *_local_2e;
                            (**ppc_var3)(
                                u_var11, local_176, u_var13, 1, local_176, u_var13, _local_2e,
                            );
                        }
                    }
                }
            }
        }
    }
    return;
}

pub fn pass1_1028_7dfc(param_1: u32) {
    let mut u_var1: u32;
    let mut u_var2: u32;
    let ppc_var3: fn();
    let pu_var4: &mut  u16;
    let mut u_var5: u16;
    let pu_var6: Vec<u8>;
    let mut u_var7: u32;
    let mut u_var8: i32;
    let mut u_var9: i32;
    let mut u_var10: u32;
    let u_var11: u8;
    let mut unaff_ss: u16;
    let ppVar12: &mut  Struct2551;
    let mut u_var13: u16;
    let mut local_178: u32;
    let mut local_174: u32;
    let mut local_158: u16;
    let mut local_156: u16;
    let mut local_48: u16;
    let mut local_46: u16;
    let mut local_44: u16;
    let mut local_42: u16;
    let mut local_3c: u32;
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
    let mut local_24: u16;
    let mut local_22: u16;
    let mut local_20: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u32;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    ppVar12 = process_struct_1010_20ba(ctx.g_struct_var_1050_0ed0, CONCAT22(local_178, 2));
  // local_4 = (ppVar12  >> 0x10);
    local_6 = ppVar12;
    local_8 = u16_1050_13ae;
    if (2 < u16_1050_13ae) {
        local_c = *ctx._PTR_LOOP_1050_65e2;
        local_c = (local_c >> 0x10);
        if (3 < local_c) {
            local_10 = local_c - 3;
            local_e = local_c - (local_c < 3);
            u_var10 = local_c % 0x14;
            if (u_var10 == 0) {
                pass4_funcs::pass1_1028_dc52(
                    CONCAT13((unaff_ss >> 8), CONCAT12(unaff_ss, &local_22)),
                    (&ctx.PTR_LOOP_1050_0000 + 1),
                    0,
                    0x400,
                );
                while (true) {
                    u_var8 = u_var10;
                    pu_var4 = &local_22;
                    pass4_funcs::pass1_1028_e4ec(CONCAT22(unaff_ss, pu_var4));
                    _local_26 = CONCAT22(u_var8, pu_var4);
                    u_var9 = u_var8 | pu_var4;
                    u_var10 = u_var9;
                    if (u_var9 == 0) {
                        break;
                    }
                    if ((pu_var4 + 0x100) != 0x8000002) {
                        pu_var6 = pass1_1008_c6fa(ctx._PTR_LOOP_1050_06e0, 0x29);
                        u_var5 = pu_var6;
                        local_2a = u_var5;
                        local_28 = u_var9;
                        pass1_1038_4d6e(_local_26, pu_var6 & 0xffff | u_var9 << 0x10);
                        _local_2e = CONCAT22(u_var9, u_var5);
                        ppc_var3 = (*_local_2e + 0x10);
                        (**ppc_var3)(&PTR_LOOP_1050_1038, u_var5, u_var9);
                        _local_32 = CONCAT22(u_var9, u_var5);
                        u_var11 = 0x10;
                        ppVar12 = process_struct_1010_20ba(
                            ctx.g_struct_var_1050_0ed0,
                            CONCAT22(local_178, 2),
                        );
                      // u_var10 = ppVar12  >> 0x10;
                        local_38 = ppVar12;
                      // local_36 = (ppVar12  >> 0x10);
                        if (local_8 == 3) {
                            local_34 = 5;
                        } else {
                            local_34 = 0x1e;
                        }
                        local_3c = 0;
                        while// u_var13 = (_local_2e  >> 0x10), local_3c < _local_32) {
                            u_var7 = _local_32;
                            pass1_1030_1d7c(_local_2e, u_var13, local_3c, (local_3c >> 0x10));
                            u_var5 = u_var7;
                            _local_44 = (u_var7 & 0xffff | u_var10 << 0x10);
                            u_var11 = 8;
                            pass1_fn_1008_612e(1, 100);
                            local_46 = u_var5;
                            if ((u_var5 <= local_34)
                                && (
                                    pass1_1028_7742(param_1, (param_1 >> 0x10), 4, _local_44),
                                    local_48 = u_var5,
                                    u_var5 == 0,
                                ))
                            {
                                u_var1 = (_local_26 + 4);
                                u_var2 = (_local_44 + 4);
                                pass4_funcs::pass1_1028_b0de(
                                    &local_158,
                                    unaff_ss,
                                    u_var2,
                                    (u_var2 >> 0x10),
                                    u_var1,
                                    (u_var1 >> 0x10),
                                );
                                u_var11 = 0x30;
                                pass1_1030_835a(
                                    ctx._g_bool_1050_5748,
                                    CONCAT22(unaff_ss, &local_158),
                                );
                                local_158 = ctx.s_1_1050_389a;
                                local_156 = &ctx.PTR_LOOP_1050_1008;
                            }
                            local_3c = local_3c + 1;
                        }
                        local_178._0_2_ = SUB42(_local_2e, 0);
                        if (_local_2e != 0x0) {
                            ppc_var3 = *_local_2e;
                            (**ppc_var3)(
                                u_var11, local_178, u_var13, 1, local_178, u_var13, _local_2e,
                            );
                        }
                    }
                }
            }
        }
    }
    return;
}

pub fn pass1_1028_7fb6(param_1: u32) {
    let pp_var1: fn();
    let p_uvar2: &mut  u16;
    let mut u_var3: u16;
    let pu_var4: Vec<u8>;
    let mut u_var5: u32;
    let mut u_var6: i32;
    let mut u_var7: i32;
    let mut u_var8: u32;
    let u_var9: u8;
    let mut unaff_ss: u16;
    let ppVar10: &mut  Struct2551;
    let pa_var11: &mut  Struct500;
    let u_var12: u8;
    let mut u_var13: u16;
    let mut local_178: u32;
    let mut local_174: u32;
    let mut local_158: u16;
    let mut local_156: u16;
    let mut local_48: u16;
    let mut local_44: u16;
    let mut local_42: u16;
    let mut local_40: u32;
    let mut local_38: u32;
    let mut local_34: u16;
    let mut local_32: u16;
    let mut local_30: u16;
    let mut local_2e: u16;
    let mut local_2c: u16;
    let mut local_2a: u16;
    let mut local_28: u16;
    let mut local_26: u16;
    let mut local_24: u16;
    let mut local_22: u16;
    let mut local_20: u16;
    let mut local_1e: u16;
    let mut local_1c: u16;
    let mut local_1a: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;

    local_6 = *ctx._PTR_LOOP_1050_65e2;
    local_6 = (local_6 >> 0x10);
    if (0xb < local_6) {
        local_a = local_6 - 0xb;
        local_8 = local_6 - (local_6 < 0xb);
        u_var8 = local_6 % 0x32;
        if (u_var8 == 0) {
            u_var12 = (unaff_ss >> 8);
            pass4_funcs::pass1_1028_dc52(
                CONCAT13(u_var12, CONCAT12(unaff_ss, &local_1c)),
                (&ctx.PTR_LOOP_1050_0000 + 1),
                0,
                0x400,
            );
            while (true) {
                u_var6 = u_var8;
                pu_var2 = &local_1c;
                pass4_funcs::pass1_1028_e4ec(CONCAT22(unaff_ss, pu_var2));
                _local_20 = CONCAT22(u_var6, pu_var2);
                u_var7 = u_var6 | pu_var2;
                u_var8 = u_var7;
                if (u_var7 == 0) {
                    break;
                }
                if ((pu_var2 + 0x100) != 0x8000002) {
                    pu_var4 = pass1_1008_c6fa(ctx._PTR_LOOP_1050_06e0, 0x11);
                    u_var3 = pu_var4;
                    local_24 = u_var3;
                    local_22 = u_var7;
                    pass1_1038_4d6e(_local_20, pu_var4 & 0xffff | u_var7 << 0x10);
                    _local_28 = CONCAT22(u_var7, u_var3);
                    pp_var1 = (*_local_28 + 0x10);
                    (**pp_var1)(&PTR_LOOP_1050_1038, u_var3, u_var7);
                    _local_2c = CONCAT22(u_var7, u_var3);
                    u_var9 = 0x10;
                    ppVar10 = process_struct_1010_20ba(
                        ctx.g_struct_var_1050_0ed0,
                        CONCAT22(local_178, 2),
                    );
                  // u_var8 = ppVar10  >> 0x10;
                    local_30 = ppVar10;
                  // local_2e = (ppVar10  >> 0x10);
                    local_32 = u16_1050_13ae;
                    if (u16_1050_13ae < 3) {
                        local_34 = 5;
                    } else {
                        local_34 = 0x14;
                    }
                    local_38 = 0;
                    while// u_var13 = (_local_28  >> 0x10), local_38 < _local_2c) {
                        u_var9 = 0x30;
                        u_var5 = _local_2c;
                        pass1_1030_1d7c(_local_28, u_var13, local_38, (local_38 >> 0x10));
                        local_40 = u_var5 & 0xffff | u_var8 << 0x10;
                        u_var3 = (u_var5 + 0x20);
                        local_42 = u_var3;
                        if (((u_var3 != 0) && (u_var3 != 0x70)) && (u_var3 != 0x71)) {
                            u_var9 = 8;
                            pass1_fn_1008_612e(1, 100);
                            local_44 = u_var3;
                            if ((u_var3 <= local_34)
                                && (
                                    pass1_1028_7742(param_1, (param_1 >> 0x10), 4, local_40),
                                    local_48 = u_var3,
                                    u_var3 == 0,
                                ))
                            {
                                pa_var11 = pass1_1028_8698(
                                    CONCAT13(u_var12, CONCAT12(unaff_ss, &local_158)),
                                    (local_40 + 4),
                                    (_local_20 + 4),
                                );
                              // u_var8 = pa_var11  >> 0x10;
                                u_var9 = 0x30;
                                pass1_1030_835a(
                                    ctx._g_bool_1050_5748,
                                    CONCAT22(unaff_ss, &local_158),
                                );
                                local_158 = ctx.s_1_1050_389a;
                                local_156 = &ctx.PTR_LOOP_1050_1008;
                            }
                        }
                        local_38 = local_38 + 1;
                    }
                    local_178._0_2_ = SUB42(_local_28, 0);
                    if (_local_28 != 0x0) {
                        pp_var1 = *_local_28;
                        (**pp_var1)(u_var9, local_178, u_var13, 1, local_178, u_var13, _local_28);
                    }
                }
            }
        }
    }
    return;
}

pub fn pass1_1028_816e(param_1: &mut Struct44, param_2: u8) -> &mut Struct44 {
    param_1.ptr_a_lo = ctx.s_1_1050_389a;
    (param_1 + 2) = &ctx.PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1028_752e(param_1: u32) {
    let pu_var1: &mut  u32;
    let pu_var2: &mut  u32;
    let ctx.ax_reg: &mut  Struct791;
    let pu_var3: &mut  u32;
    let mut i_var4: i32;
    let in_dx: &mut  Struct199;
    let pu_var5: &mut  u32;
    let mut u_var6: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;

    process_struct_1000_179c(0x108, in_dx);
    _local_a = CONCAT22(in_dx, ctx.ax_reg);
    if ((in_dx | ctx.ax_reg) != 0) {
        *_local_a = ctx.s_1_1050_389a;
        ctx.ax_reg.field_0x2 = &ctx.PTR_LOOP_1050_1008;
      // u_var6 = (param_1  >> 0x10);
        ctx.ax_reg.field_0x4 = (param_1 + 4);
        pu_var3 = (param_1 + 8);
        pu_var5 = &ctx.ax_reg.field_0x8;
        i_var4 = 0x40;
        while (i_var4 != 0) {
            i_var4 = i_var4 + -1;
            pu_var2 = pu_var5;
            pu_var5 = pu_var5 + 1;
            pu_var1 = pu_var3;
            pu_var3 = pu_var3 + 1;
            unsafe { *pu_var2 = *pu_var1 };
        }
        *_local_a = 0x6ad2;
        ctx.ax_reg.field_0x2 = &PTR_LOOP_1050_1028;
        *_local_a = 0x819a;
        ctx.ax_reg.field_0x2 = &PTR_LOOP_1050_1028;
    }
    return;
}

pub fn pass1_1028_75bc() {
    let mut u_var1: i32;
    let p_uvar2: &mut  u16;
    let mut u_var3: u32;
    let mut unaff_ss: u16;
    let mut local_20: u16;
    let mut local_1c: u16;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;

    local_6 = *ctx._PTR_LOOP_1050_65e2;
    u_var1 = (local_6 % 0x7b);
    u_var3 = u_var1;
    if ((u_var1 == 0) && (0x95 < local_6)) {
        pass4_funcs::pass1_1028_dc52(
            CONCAT22(unaff_ss, &local_18),
            (&ctx.PTR_LOOP_1050_0000 + 1),
            0,
            0x400,
        );
        while (true) {
            u_var1 = u_var3;
            pu_var2 = &local_18;
            pass4_funcs::pass1_1028_e4ec(CONCAT22(unaff_ss, pu_var2));
            _local_1c = CONCAT22(u_var1, pu_var2);
            u_var3 = (u_var1 | pu_var2);
            if ((u_var1 | pu_var2) == 0) {
                break;
            }
            pass1_fn_1008_612e();
            if (pu_var2 < 6) {
                win_fn_1038_362e(_local_1c);
            }
        }
        if (local_8 != 0) {
            local_c = 1;
            local_a = 0;
        }
        u_var3 = local_a;
        local_10 = local_c;
        local_e = local_a;
        while (true) {
            u_var1 = u_var3;
            pu_var2 = &local_18;
            pass4_funcs::pass1_1028_e4ec(CONCAT22(unaff_ss, pu_var2));
            u_var3 = (u_var1 | pu_var2);
            if ((u_var1 | pu_var2) == 0) {
                break;
            }
            pass1_1038_3698(CONCAT22(u_var1, pu_var2));
        }
    }
    return;
}

pub fn pass1_1028_737e(param_1: u32) {
    let pu_var1: &mut  u32;
    let pu_var2: &mut  u32;
    let mut in_ax: i32;
    let pu_var3: &mut  u32;
    let mut i_var4: i32;
    let in_dx: &mut  Struct199;
    let pu_var5: &mut  u32;
    let mut u_var6: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;

    process_struct_1000_179c(0x108, in_dx);
    _local_a = CONCAT22(in_dx, in_ax);
    if ((in_dx | in_ax) != 0) {
        *_local_a = ctx.s_1_1050_389a;
        (in_ax + 2) = &ctx.PTR_LOOP_1050_1008;
      // u_var6 = (param_1  >> 0x10);
        (in_ax + 4) = (param_1 + 4);
        pu_var3 = (param_1 + 8);
        pu_var5 = (in_ax + 8);
        i_var4 = 0x40;
        while (i_var4 != 0) {
            i_var4 = i_var4 + -1;
            pu_var2 = pu_var5;
            pu_var5 = pu_var5 + 1;
            pu_var1 = pu_var3;
            pu_var3 = pu_var3 + 1;
            unsafe { *pu_var2 = *pu_var1 };
        }
        *_local_a = 0x6ad2;
        (in_ax + 2) = &PTR_LOOP_1050_1028;
        *_local_a = 0x749e;
        (in_ax + 2) = &PTR_LOOP_1050_1028;
    }
    return;
}

pub fn pass1_1028_740c(param_1: u16, param_2: u16, param_3: u16, param_2_00: u32) {
    let pp_var1: fn();
    let mut u_var2: u32;
    let mut u_var3: u16;
    let mut u_var4: u16;
    let pu_var5: Vec<u8>;
    let mut in_dx: i32;


    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;

    pu_var5 = pass1_1008_c6fa(ctx._PTR_LOOP_1050_06e0, param_1_00);
    u_var3 = SUB42(pu_var5, 0);
    pass1_1038_4d6e(param_2_00, pu_var5 & 0xffff | in_dx << 0x10);
    _local_a = CONCAT22(ctx.dx_reg, u_var3);
    u_var2 = *_local_a;
    pp_var1 = u_var2 + 8;
    u_var4 = u_var3;
    (**pp_var1)(&PTR_LOOP_1050_1038, u_var3, ctx.dx_reg);
    _local_e = CONCAT22(ctx.dx_reg, u_var4);
    if (_local_a != 0x0) {
        pp_var1 = u_var2;
        (**pp_var1)(&PTR_LOOP_1050_1038, u_var3, ctx.dx_reg, 1);
    }
    if (_local_e != 0) {
        return;
    }
    return;
}

pub fn pass1_1028_7472(param_1: &mut Struct44, param_2: u8) -> &mut Struct44 {
    param_1.ptr_a_lo = ctx.s_1_1050_389a;
    (param_1 + 2) = &ctx.PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1028_6e96() {
    let mut iVar1: i32;
    let ppc_var2: fn();
    let mut unaff_ss: u16;
    let pu_var3: &mut  u32;
    let mut local_20: u16;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u16;

    pass4_funcs::pass1_1028_dc52(
        CONCAT22(unaff_ss, &local_14),
        (&ctx.PTR_LOOP_1050_0000 + 1),
        0,
        0x700,
    );
    while (true) {
        pu_var3 = pass4_funcs::pass1_1028_e4ec(CONCAT22(unaff_ss, &local_14));
        if (pu_var3 == 0x0) {
            break;
        }
        iVar1 = (pu_var3 + 0x12);
        if (((0 < iVar1) && (!SBORROW2(iVar1, 1))) && (iVar1 + -1 < 4)) {
            unsafe {
                ppc_var2 = (*pu_var3 + 0x38);
                ppc_var2();
            }
        }
    }
    return 1;
}

pub fn pass1_1028_6ef6(param_1: u32) {
    let pu_var1: &mut  u32;
    let pu_var2: &mut  u32;
    let mut in_ax: i32;
    let pu_var3: &mut  u32;
    let mut i_var4: i32;
    let in_dx: &mut  Struct199;
    let pu_var5: &mut  u32;
    let mut u_var6: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;

    process_struct_1000_179c(0x108, in_dx);
    _local_a = CONCAT22(in_dx, in_ax);
    if ((in_dx | in_ax) != 0) {
        *_local_a = ctx.s_1_1050_389a;
        (in_ax + 2) = &ctx.PTR_LOOP_1050_1008;
      // u_var6 = (param_1  >> 0x10);
        (in_ax + 4) = (param_1 + 4);
        pu_var3 = (param_1 + 8);
        pu_var5 = (in_ax + 8);
        i_var4 = 0x40;
        while (i_var4 != 0) {
            i_var4 = i_var4 + -1;
            pu_var2 = pu_var5;
            pu_var5 = pu_var5 + 1;
            pu_var1 = pu_var3;
            pu_var3 = pu_var3 + 1;
            unsafe { *pu_var2 = *pu_var1 };
        }
        *_local_a = 0x6ad2;
        (in_ax + 2) = &PTR_LOOP_1050_1028;
        *_local_a = 0x6fb0;
        (in_ax + 2) = &PTR_LOOP_1050_1028;
    }
    return;
}

pub fn pass1_1028_6f84(param_1: &mut Struct44, param_2: u8) -> &mut Struct44 {
    param_1.ptr_a_lo = ctx.s_1_1050_389a;
    (param_1 + 2) = &ctx.PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1028_6926(param_1: u32) {
    let mut iVar1: i32;
    let mut u_var2: u32;
    let ppc_var3: fn();
    let paVar4: &mut  Struct493;
    let mut u_var5: i32;
    let pu_var6: Vec<u8>;
    let mut in_dx: i32;
    let mut u_var7: i32;



    let mut u_var8: u16;
    let mut u_var9: u16;
    let mut u_var10: u16;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_6: u16;
    let mut local_4: u16;

  // u_var9 = (param_1  >> 0x10);
    u_var2 = (param_1 + 0x108);
  // paVar4 = pass4_funcs::pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, u_var2, (u_var2  >> 0x10));
    u_var7 = in_dx;
    pu_var6 = pass1_1008_c6fa(ctx._PTR_LOOP_1050_06e0, 10);
    u_var5 = pu_var6;
    u_var10 = SUB42(&PTR_LOOP_1050_1038, 0);
    pass1_1038_4d6e(CONCAT22(in_dx, paVar4), pu_var6 & 0xffff | u_var7 << 0x10);
    _local_e = CONCAT22(ctx.dx_reg, u_var5);
    u_var2 = *_local_e;
    ppc_var3 = (u_var2 + 0x10);
    u_var7 = u_var5;
    (**ppc_var3)(&PTR_LOOP_1050_1038, u_var5, ctx.dx_reg);
    if ((ctx.dx_reg | u_var7) != 0) {
        ppc_var3 = (u_var2 + 4);
        (**ppc_var3)(0x38, u_var5, ctx.dx_reg, 0, 0);
        u_var8 = ctx.dx_reg;
        paVar4 = pass4_funcs::pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, u_var7, ctx.dx_reg);
        iVar1 = (param_1 + 0x10c);
        u_var10 = 0x1030;
        pass1_1030_7ddc(
            CONCAT22(u_var8, paVar4),
            CONCAT13(iVar1 >> 0xf, iVar10, 0x1f),
        );
    }
    if (_local_e != 0x0) {
        ppc_var3 = *_local_e;
        (**ppc_var3)(u_var10, u_var5, ctx.dx_reg, 1);
    }
    return;
}

pub fn pass1_1028_69cc(param_1: &mut  Struct788) {
    let pu_var1: &mut  u32;
    let pu_var2: &mut  u32;
    let local_a_x__1: &mut  Struct787;
    let mut i_var3: i32;
    let in_d_x: &mut  Struct199;
    let local_b_x_41: &mut  Struct788;
    let pu_var4: &mut  u32;
    let pu_var5: &mut  u32;
    let mut u_var6: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;

    process_struct_1000_179c(0x10e, in_d_x);
    _local_a = CONCAT22(in_d_x, local_a_x__1);
    if ((in_d_x | local_a_x__1) != 0) {
        *_local_a = ctx.s_1_1050_389a;
        local_a_x__1.field_0x2 = &ctx.PTR_LOOP_1050_1008;
      // u_var6 = (param_1  >> 0x10);
        local_b_x_41 = param_1;
        local_a_x__1.field_0x4 = local_b_x_41.field_0x4;
        pu_var4 = &local_b_x_41.field_0x8;
        pu_var5 = &local_a_x__1.field_0x8;
        i_var3 = 0x40;
        while (i_var3 != 0) {
            i_var3 = i_var3 + -1;
            pu_var2 = pu_var5;
            pu_var5 = pu_var5 + 1;
            pu_var1 = pu_var4;
            pu_var4 = pu_var4 + 1;
            unsafe { *pu_var2 = *pu_var1 };
        }
        *_local_a = 0x6ad2;
        local_a_x__1.field_0x2 = &PTR_LOOP_1050_1028;
        local_a_x__1.field_0x108 = local_b_x_41.field_0x108;
        local_a_x__1.field_0x10c = local_b_x_41.field_0x10c;
        *_local_a = 0x6ae2;
        local_a_x__1.field_0x2 = &PTR_LOOP_1050_1028;
    }
    return;
}

pub fn pass1_1028_6a7a(param_1: &mut Struct44, param_2: u8) -> &mut Struct44 {
    param_1.ptr_a_lo = ctx.s_1_1050_389a;
    (param_1 + 2) = &ctx.PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1028_6aa6(param_1: &mut Struct44, param_2: u8) -> &mut Struct44 {
    param_1.ptr_a_lo = ctx.s_1_1050_389a;
    (param_1 + 2) = &ctx.PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1028_6af2(param_1: &mut  Struct500, param_2: u32, param_3: u32) {
    let local_bx_19: &mut  Struct500;
    let mut u_var1: u16;

    pass4_funcs::pass1_1028_d1dc(param_1, 0x1387);
  // u_var1 = (param_1  >> 0x10);
    local_bx_19 = param_1;
    local_bx_19.field_0x108 = param_3;
    local_bx_19.field_0x10c = param_2;
    param_1.a = 0x6e50;
    local_bx_19.b = &PTR_LOOP_1050_1028;
    return;
}

pub fn pass1_1028_6b2c(param_1: u32) {
    pass1_1028_6b40(param_1);
    return 1;
}

pub fn pass1_1028_6b40(param_1: u32) {
    let mut u_var1: u32;
    let ppc_var2: fn();
    let u_var3: u8;
    let paVar4: &mut  Struct493;
    let extraout_ah: u8;
    let mut in_dx: u16;

    let mut u_var5: u16;
    let mut u_var6: u16;
    let mut u_var7: u16;
    let mut unaff_ss: u16;
    let mut local_36: [u8; 14];
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
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: [u8; 2];
    let mut local_4: u16;

    pass1_1028_6daa(
        param_1,
        CONCAT22(unaff_ss, local_6),
        CONCAT22(unaff_ss, &local_4),
    );
  // u_var7 = (param_1  >> 0x10);
    u_var6 = param_1;
    u_var1 = (u_var6 + 0x10c);
  // paVar4 = pass4_funcs::pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, u_var1, (u_var1  >> 0x10));
    _local_a = CONCAT22(in_dx, paVar4);
    ppc_var2 = (_local_a + 0x24);
    ppc_var2();
    local_c = ctx.dx_reg;
    u_var3 = pass4_funcs::pass1_1028_b58e(_local_a);
    local_e = CONCAT11(extraout_ah, u_var3);
    _local_12 = pass4_funcs::pass1_1028_bb24(_local_a);
    _local_18 = (local_e + 0xc);
    local_14 = (local_e + 0x10);
    local_28 = &local_18;
    local_1a = local_18;
  // local_1c = (_local_18  >> 0x10);
    local_20 = local_18 - 1;
    if (local_20 < 0) {
        local_20 = 0;
    }
    u_var5 = local_4 - 1;
    local_22 = local_18 + 1;
    if (u_var5 < (local_18 + 1)) {
        local_22 = u_var5;
    }
    local_24 = local_1c - 1;
    if (local_24 < 0) {
        local_24 = 0;
    }
    local_26 = local_1c + 1;
    if (u_var5 < (local_1c + 1)) {
        local_26 = u_var5;
    }
    local_1e = local_14;
    pass1_1008_3e54(CONCAT22(unaff_ss, local_36), local_14, local_24, local_20);
    pass1_1028_6d24(u_var6, u_var7, CONCAT22(unaff_ss, local_36), _local_12);
    pass1_1008_3e54(CONCAT22(unaff_ss, local_36), local_1e, local_24, local_1a);
    pass1_1028_6d24(u_var6, u_var7, CONCAT22(unaff_ss, local_36), _local_12);
    pass1_1008_3e54(CONCAT22(unaff_ss, local_36), local_1e, local_24, local_22);
    pass1_1028_6d24(u_var6, u_var7, CONCAT22(unaff_ss, local_36), _local_12);
    pass1_1008_3e54(CONCAT22(unaff_ss, local_36), local_1e, local_1c, local_20);
    pass1_1028_6d24(u_var6, u_var7, CONCAT22(unaff_ss, local_36), _local_12);
    pass1_1008_3e54(CONCAT22(unaff_ss, local_36), local_1e, local_1c, local_22);
    pass1_1028_6d24(u_var6, u_var7, CONCAT22(unaff_ss, local_36), _local_12);
    pass1_1008_3e54(CONCAT22(unaff_ss, local_36), local_1e, local_26, local_20);
    pass1_1028_6d24(u_var6, u_var7, CONCAT22(unaff_ss, local_36), _local_12);
    pass1_1008_3e54(CONCAT22(unaff_ss, local_36), local_1e, local_26, local_1a);
    pass1_1028_6d24(u_var6, u_var7, CONCAT22(unaff_ss, local_36), _local_12);
    pass1_1008_3e54(CONCAT22(unaff_ss, local_36), local_1e, local_26, local_22);
    pass1_1028_6d24(u_var6, u_var7, CONCAT22(unaff_ss, local_36), _local_12);
    return;
}

pub fn pass1_1028_6d24(param_1: u16, param_2: u16, param_1_00: u32, param_2_00: u32) {
    let pp_var1: fn();
    let pu_var2: &mut  u32;
    let paVar3: &mut  Struct493;

    let mut u_var4: u16;
    let mut unaff_ss: u16;
    let pu_var5: &mut  u32;
    let mut local_1e: u32;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_14: u32;
    let mut local_a: u32;
    let mut local_6: u32;
    let temp_5f5b5c35b8: &mut  Struct790;

    pu_var2 = &local_a;
    pass1_1030_64ce(
        _PTR_LOOP_1050_5740,
        param_1_00,
        param_2_00,
        pu_var2,
        unaff_ss,
    );
    unsafe { local_6 = *pu_var2 };
    u_var4 = (pu_var2 + 2);
    local_1e._3_1_ = (local_6 >> 0x18);
    if (local_1e._3_1_ != '\0') {
        paVar3 = pass4_funcs::pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, local_6, u_var4);
        pu_var5 = pass1_1030_73a8(CONCAT22(u_var4, paVar3));
        temp_5f5b5c35b8 = (pu_var5 + 0xc);
        if (((temp_5f5b5c35b8 < 1) || (SBORROW2(temp_5f5b5c35b8, 1)))
            || (temp_5f5b5c35b8 != &BYTE_1050_0009
                && 7 < (temp_5f5b5c35b8 + -1)
                && ((temp_5f5b5c35b8 + -9) < 0x6a || (6 < (temp_5f5b5c35b8 + -0x73)))))
        {
            unsafe {
                pp_var1 = (*pu_var5 + 0x24);
                (**pp_var1)()
            };
        }
    }
    return;
}

pub fn pass1_1028_6daa(param_1: u32, param_2: u32, param_3: u32) {
    let mut u_var1: u32;
    let mut in_dx: u16;
    let pu_var2: &mut  u32;
    let mut local_1c: u16;
    let mut local_18: u32;
    let mut uStack20: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u32;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    u_var1 = (param_1 + 0x10c);
  // local_6 = pass4_funcs::pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, u_var1, (u_var1  >> 0x10));
    _local_a = pass4_funcs::pass1_1028_b4f2(CONCAT22(in_dx, local_6));
  // local_10 = (_local_a  >> 0x10);
    local_e = (_local_a + 8);
  // local_12 = pass4_funcs::pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, local_e, (local_e  >> 0x10));
    pu_var2 = pass1_1030_5b5c(local_12, local_10);
    unsafe { local_18 = *pu_var2 };
    uStack20 = (pu_var2 + 4);
    pass1_1008_3e94(&local_18, param_2, param_3);
    return;
}

pub fn pass1_1028_6e24(param_1: &mut Struct44, param_2: u8) -> &mut Struct44 {
    param_1.ptr_a_lo = ctx.s_1_1050_389a;
    (param_1 + 2) = &ctx.PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1028_6744(param_1: u32, param_2: u16) {
    let mut u_var1: u16;
    let mut unaff_ss: u16;
    let lVar2: u32;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: [u8; 8];

    pass1_1008_5784(CONCAT22(unaff_ss, local_a), (param_1 + 0x20));
    while {
        lVar2 = pass1_1008_5b12(CONCAT22(unaff_ss, local_a));
      // u_var1 = (lVar2  >> 0x10);
        if (lVar2 == 0) {
            return 0;
        }
        (lVar2 + 6) != param_2
    } {}
    return *(lVar2 + 10);
}

pub fn pass1_1028_678c(param_1: u32, param_2: u16) {
    let mut u_var1: u16;
    let mut unaff_ss: u16;
    let lVar2: u32;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: [u8; 8];

    pass1_1008_5784(CONCAT22(unaff_ss, local_a), (param_1 + 0x20));
    while {
        lVar2 = pass1_1008_5b12(CONCAT22(unaff_ss, local_a));
      // u_var1 = (lVar2  >> 0x10);
        if (lVar2 == 0) {
            return 0;
        }
        (lVar2 + 8) != param_2
    } {}
    return *(lVar2 + 10);
}

pub fn pass1_1028_67d4(param_1: u32) {
    let mut u_var1: i32;
    let mut unaff_ss: u16;
    let lVar2: u32;
    let mut local_12: u32;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: [u8; 8];

    pass1_1008_5784(CONCAT22(unaff_ss, local_a), (param_1 + 0x20));
    local_12 = 0;
    while (true) {
        lVar2 = pass1_1008_5b12(CONCAT22(unaff_ss, local_a));
        if (lVar2 == 0) {
            break;
        }
        u_var1 = (lVar2 + 0xc);
        local_12 = CONCAT22(
            (local_12 >> 0x10) + CARRY2(local_12, u_var1),
            local_12 + u_var1,
        );
    }
    return local_12;
}

pub fn pass1_1028_6822(param_1: u32, param_2: &mut  u32) {
    let mut iVar1: i32;
    let mut u_var2: u32;

    u_var2 = pass1_1028_67d4(param_1);
  // iVar1 = (u_var2  >> 0x10);
    unsafe {
        *param_2 = u_var2;
        (param_2 + 2) = iVar1;
        if ((iVar1 == 0) && (*param_2 < 100)) {
            return 0;
        }
    }
    return 1;
}

pub fn pass1_1028_6850(param_1: &mut Struct44, param_2: u8) -> &mut Struct44 {
    pass1_1028_6186(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1028_6008(param_1: u32) {
    let piVar1: &mut  i32;
    let mut i_var2: i32;
    let mut u_var3: u16;

    pass4_funcs::pass1_1028_be2a(param_1);
  // u_var3 = (param_1  >> 0x10);
    i_var2 = param_1;
    if (((i_var2 + 0x12) == 5) && (0 < (i_var2 + 0x20))) {
        piVar1 = (i_var2 + 0x20);
        unsafe { *piVar1 = *piVar1 + -1 };
    }
    return;
}

pub fn pass1_1028_602e(param_1: &mut Struct44, param_2: u8) -> &mut Struct44 {
    pass4_funcs::pass1_1028_b418(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1028_6064(param_1: u32, param_2: u8) {
    let pc_var1: String;
    let pu8_var2: Vec<u8>;
    let mut u8_var3: u8;
    let mut in_ax: i32;
    let mut u8_var4: u8;
    let mut b_var5: u8;
    let mut in_dx: u16;
    let mut in_bx: i32;
    let mut cVar6: u8;
    let pu_var7: &mut  u16;
    let unaff_bp: &mut  u16;
    let mut unaff_si: i32;
    let mut unaff_DI: i32;
    let mut unaff_cs: u16;
    let mut unaff_ss: u16;
    let mut in_CF: u8;
    let mut bVar8: bool;

    pu_var7 = &stack0xfffe;
    cVar6 = '\x0f';
    while {
        unaff_bp = unaff_bp + -1;
        pu_var7 = pu_var7 + -1;
        unsafe { *pu_var7 = *unaff_bp };
        cVar6 = cVar6 + -1;
        '\0' < cVar6
    } {}
    pc_var1 = (in_bx + unaff_si);
    b_var5 = in_dx;
    unsafe { *pc_var1 = *pc_var1 - b_var5 };
    pc_var1 = (&PTR_LOOP_1050_1028 + unaff_DI);
    cVar6 = (in_bx >> 8);
    unsafe {
        if (*pc_var1 != cVar6 && cVar6 <= *pc_var1) {
            pu8_var2 = (in_bx + unaff_si);
            u8_var3 = *pu8_var2;
            *pu8_var2 = *pu8_var2 - b_var5;
            pc_var1 = (in_bx + unaff_si);
            *pc_var1 = *pc_var1 - b_var5;
            pu8_var2 = (in_bx + unaff_si + 0x28);
            u8_var4 = ((in_ax & 0xff00) >> 8);
            *pu8_var2 = *pu8_var2 | u8_var4;
            bVar8 = CARRY1(s_fem79_wav_1050_28ba[5], b_var5);
            s_fem79_wav_1050_28ba[5] = s_fem79_wav_1050_28ba[5] + b_var5;
            pc_var1 = &stack0xfffe + unaff_si;
            *pc_var1 = *pc_var1 + in_bx + bVar8;
            0x1026 = &ctx.g_alloc_addr_1050_1050;
            pu8_var2 = (in_bx + 0x28);
            *pu8_var2 = *pu8_var2 ^ in_bx;
            pc_var1 = &stack0xfffe + unaff_si;
            *pc_var1 = *pc_var1 + u8_var4;
            LOCK();
            pc_var1 = (CONCAT11(*(&PTR_LOOP_1050_1028 + unaff_si) + '\x10', 0x28) + unaff_si);
            *pc_var1 = *pc_var1 - b_var5;
            return CONCAT22(
                in_dx,
                in_ax & 0xff00 | ((in_ax - in_CF) + -7 + (u8_var3 < b_var5)),
            );
        }
    }
    0x1024 = param_1;
    0x1022 = unaff_cs;
    0x1020 = 0x603b;
    pass4_funcs::pass1_1028_b418(paRam00001024);
    if ((param_2 & 1) != 0) {
        0x1024 = param_1;
        0x1022 = unaff_cs;
        0x1020 = 0x604a;
        error_check_1000_17ce(paRam00001024);
    }
    return param_1;
}

pub fn pass1_1028_60bc(param_1: &mut  Struct763) -> &mut  Struct763 {
    let paVar1: &mut  Struct199;
    let mut u_var2: i32;
    let local_bx_13: &mut  Struct784;
    let mut u_var3: u16;
    let paVar4: &mut  Struct199;
    let mut local_4: u16;

    paVar4 = pass4_funcs::pass1_1028_b354(param_1);
  // u_var3 = (param_1  >> 0x10);
    local_bx_13 = param_1;
    &local_bx_13.field_0x20 = 0;
    param_1.field_0x0 = 0x6876;
    local_bx_13.field_0x2 = &PTR_LOOP_1050_1028;
    process_struct_1000_179c(0xc, (paVar4 >> 0x10));
  // u_var2 = (paVar4  >> 0x10) | paVar4;
    if (paVar4 == 0x0) {
        &local_bx_13.field_0x20 = 0;
    } else {
        paVar1 = process_struct_1008_574a(paVar4);
        local_bx_13.field_0x20 = paVar1;
        local_bx_13.field_0x22 = u_var2;
    }
    return param_1;
}

pub fn pass1_1028_611e(param_1: i32, param_2: u16, param_3: u16, param_3_00: u32) {
    let paVar1: &mut  Struct199;
    let mut u_var2: i32;
    let paVar3: &mut  Struct199;
    let mut local_4: u16;

    paVar3 = pass4_funcs::pass1_1028_b39e(CONCAT22(param_2, param_1), param_3, param_3_00);
    (param_1 + 0x20) = 0;
    CONCAT22(param_2, param_1) = 0x6876;
    (param_1 + 2) = &PTR_LOOP_1050_1028;
    process_struct_1000_179c(0xc, (paVar3 >> 0x10));
  // u_var2 = (paVar3  >> 0x10) | paVar3;
    if (paVar3 == 0x0) {
        (param_1 + 0x20) = 0;
    } else {
        paVar1 = process_struct_1008_574a(paVar3);
        (param_1 + 0x20) = paVar1;
        (param_1 + 0x22) = u_var2;
    }
    return CONCAT22(param_2, param_1);
}

pub fn pass1_1028_6186(param_1: &mut Struct44) {
    let pu_var1: &mut  u32;
    let mut u_var2: i32;
    let ppc_var3: fn();
    let mut i_var4: i32;
    let mut u_var5: u16;

  // u_var5 = (param_1  >> 0x10);
    i_var4 = param_1;
    param_1.ptr_a_lo = 0x6876;
    (i_var4 + 2) = &PTR_LOOP_1050_1028;
    pu_var1 = (i_var4 + 0x20);
    u_var2 = (i_var4 + 0x22);
    if ((u_var2 | pu_var1) != 0) {
        unsafe {
            ppc_var3 = *pu_var1;
            (**ppc_var3)();
        }
    }
    pass4_funcs::pass1_1028_b418(param_1);
    return;
}

pub fn pass1_1028_61c4(param_1: &mut  Struct781, param_2: Vec<u8>) {
    let pu_var1: &mut  u32;
    let ppc_var2: fn();
    let paVar3: &mut  Struct199;
    let mut u_var4: i32;
    let mut u_var5: i32;
    let mut i32_var6: i32;
    let mut u_var7: u16;
    let pa_var8: &mut  Struct199;
    let mut local_8: u16;

    pass4_funcs::pass1_1028_b46e(param_1, param_2);
  // u_var7 = (param_1  >> 0x10);
    i32_var6 = param_1;
    pu_var1 = (i32_var6 + 0x20);
    u_var5 = (i32_var6 + 0x22);
    u_var4 = u_var5 | pu_var1;
    pa_var8 = CONCAT22(u_var4, pu_var1);
    if (u_var4 != 0) {
        unsafe {
            ppc_var2 = *pu_var1;
            pa_var8 = ppc_var2();
        }
    }
    process_struct_1000_179c(0xc, (pa_var8 >> 0x10));
  // u_var5 = (pa_var8  >> 0x10) | pa_var8;
    if (pa_var8 == 0x0) {
        paVar3 = 0x0;
        u_var5 = 0;
    } else {
        paVar3 = process_struct_1008_574a(pa_var8);
    }
    (i32_var6 + 0x20) = paVar3;
    (i32_var6 + 0x22) = u_var5;
    return;
}

pub fn pass1_1028_6228(param_1: u32, param_2: u16, param_3: u16, param_4: u16) {
    let mut u_var1: i32;
    let mut u_var2: i32;
    let ppc_var3: fn();
    let mut i_var4: i32;
    let mut u_var5: u16;
    let mut i32_var6: i32;
    let mut u_var7: u16;
    let mut unaff_ss: u16;
    let mut bVar8: bool;
    let lVar9: u32;
    let mut local_14: u16;
    let mut local_12: u32;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: [u8; 4];
    let mut local_6: u32;

  // u_var7 = (param_1  >> 0x10);
    i32_var6 = param_1;
    pass1_1008_5784(CONCAT22(unaff_ss, local_a), (i32_var6 + 0x20));
    loop {
        while {
            lVar9 = pass1_1008_5b12(CONCAT22(unaff_ss, local_a));
          // u_var5 = (lVar9  >> 0x10);
            i_var4 = lVar9;
            if (lVar9 == 0) {
                return;
            }
            (i_var4 + 6) != param_4
        } {}
        u_var1 = (i_var4 + 10);
        if ((param_3 == 0) && (param_2 < u_var1)) {
            break;
        }
        bVar8 = param_2 < u_var1;
        param_2 = param_2 - u_var1;
        param_3 = param_3 - bVar8;
        ppc_var3 = ((i32_var6 + 0x20) + 0xc);
        (**ppc_var3)(&ctx.PTR_LOOP_1050_1008, (i32_var6 + 0x20));
        local_6 = 0;
    }
    u_var2 = (i_var4 + 0xc);
    (i_var4 + 10) = u_var1 - param_2;
    (i_var4 + 0xc) = -(param_2 * (u_var2 / u_var1) - (i_var4 + 0xc));
    return;
}

pub fn pass1_1028_62c8(param_1: u32) {
    let mut u_var1: i32;
    let mut u_var2: u32;
    let mut local_6: u16;
    let mut local_4: u16;

  // u_var1 = (param_1  >> 0x10);
    if ((param_1 + 0x12) == 5) {
        u_var2 = pass1_1028_67d4(param_1 & 0xffff | u_var1 << 0x10);
        u_var1 = u_var2;
        if (((u_var2 >> 0x10) == 0) && (u_var1 < 100)) {
            return CONCAT22(-(100 < u_var1), 100 - u_var1);
        }
    }
    return 0;
}

pub fn pass1_1028_6302(param_1: u32) {
    let mut u_var1: i32;
    let mut u_var2: u16;
    let mut unaff_ss: u16;
    let lVar3: u32;
    let mut local_12: u32;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: [u8; 8];

    pass1_1008_5784(CONCAT22(unaff_ss, local_a), (param_1 + 0x20));
    local_12 = 0;
    loop {
        lVar3 = pass1_1008_5b12(CONCAT22(unaff_ss, local_a));
      // u_var2 = (lVar3  >> 0x10);
        if (lVar3 == 0) {
            break;
        }
        if ((lVar3 + 8) != 0) {
            u_var1 = (lVar3 + 10);
            local_12 = CONCAT22(
                (local_12 >> 0x10) + CARRY2(local_12, u_var1),
                local_12 + u_var1,
            );
        }
    }
    return local_12;
}

pub fn pass1_1028_6356(param_1: u32, param_2: i32, uparam_3: i32, param_4: i32) {
    let piVar1: &mut  i32;
    let mut u_var2: i32;
    let mut u_var3: i32;
    let ppcVar4: fn();
    let mut i_var5: i32;
    let mut u_var6: u16;
    let mut i_var7: i32;
    let mut u_var8: u16;
    let mut unaff_ss: u16;
    let mut bVar9: bool;
    let lVar10: u32;
    let mut local_14: u16;
    let mut local_12: u32;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: [u8; 4];
    let mut local_6: u32;

  // u_var8 = (param_1  >> 0x10);
    i_var7 = param_1;
    pass1_1008_5784(CONCAT22(unaff_ss, local_a), (i_var7 + 0x20));
    loop {
        while {
            lVar10 = pass1_1008_5b12(CONCAT22(unaff_ss, local_a));
          // u_var6 = (lVar10  >> 0x10);
            i_var5 = lVar10;
            if (lVar10 == 0) {
                return;
            }
            (((i_var5 + 8) == 0) || (param_2 != 0 && ((i_var5 + 8) != param_2)))
                || ((i_var5 + 8) == 0xf && (param_2 != 0xf))
        } {}
        u_var2 = (i_var5 + 10);
        if ((param_4 == 0) && (param_3 < u_var2)) {
            break;
        }
        bVar9 = param_3 < u_var2;
        param_3 = param_3 - u_var2;
        param_4 = param_4 - bVar9;
        ppcVar4 = ((i_var7 + 0x20) + 0xc);
        (**ppcVar4)(&ctx.PTR_LOOP_1050_1008, (i_var7 + 0x20));
        local_6 = 0;
    }
    u_var3 = (i_var5 + 0xc);
    piVar1 = (i_var5 + 10);
    unsafe {
        *piVar1 = *piVar1 - param_3;
        piVar1 = (i_var5 + 0xc);
        *piVar1 = *piVar1 - param_3 * (u_var3 / u_var2);
    }
    return;
}

pub fn pass1_1028_6408(param_1: u32, param_2: &mut  u32) {
    let pp_var1: fn();
    let mut b_var2: bool;
    let pu_var3: Vec<u8>;

    let mut i_var4: i32;
    let mut i_var5: i32;
    let mut u_var6: u16;
    let mut u_var7: u16;
    let mut unaff_ss: u16;
    let mut local_18: u32;
    let mut local_14: u32;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: [u8; 8];

  // u_var6 = (param_1  >> 0x10);
    i_var4 = param_1;
    pass1_1008_5784(CONCAT22(unaff_ss, local_a), (i_var4 + 0x20));
    b_var2 = false;
    loop {
        pu_var3 = local_a;
        pass1_1008_5b12(CONCAT22(unaff_ss, pu_var3));
        i_var5 = param_2;
      // u_var7 = (param_2  >> 0x10);
        if ((ctx.dx_reg | pu_var3) == 0) {
            break;
        }
        if (((pu_var3 + 4) == (i_var5 + 4)) && ((pu_var3 + 6) == (i_var5 + 6))) {
            if ((pu_var3 + 8) == (i_var5 + 8)) {
                b_var2 = true;
                (pu_var3 + 10) = (pu_var3 + 10) + (i_var5 + 10);
                (pu_var3 + 0xc) = (pu_var3 + 0xc) + (i_var5 + 0xc);
            }
        }
    }
    if (b_var2) {
        if (param_2 != 0x0) {
            unsafe {
                pp_var1 = *param_2;
                (**pp_var1)(&ctx.PTR_LOOP_1050_1008, param_2, 1, param_2, param_2);
            }
            return;
        }
    } else {
        pp_var1 = ((i_var4 + 0x20) + 4);
        (**pp_var1)(&ctx.PTR_LOOP_1050_1008, (i_var4 + 0x20), param_2);
    }
    return;
}

pub fn pass1_1028_4a9a(param_1: u32, param_2: i32) {
    return (param_1 + 0x20 + param_2 * 2);
}

pub fn pass1_1028_4ab2(param_1: u32, param_2: u16, param_3: i32) {
    (param_1 + param_3 * 2 + 0x20) = param_2;
    return;
}

pub fn pass1_1028_4aca(param_1: u32) {
    let paVar1: &mut  Struct865;
    let ppVar2: &mut  Struct2551;
    let mut in_stack_0000fff6: u16;

    if ((param_1 + 0x20) != 0) {
        ppVar2 = process_struct_1010_20ba(
            ctx.g_struct_var_1050_0ed0,
            CONCAT22(in_stack_0000fff6, 0x2f),
        );
        paVar1 = pass1_1010_ed3e(ppVar2);
        pass1_1030_2a2c(paVar1);
    }
    return;
}

pub fn pass1_1028_4af6(param_1: &mut Struct44, param_2: u8) -> &mut Struct44 {
    pass4_funcs::pass1_1028_b418(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1028_4b84(param_1: &mut  Struct763) -> &mut  Struct763 {
    pass4_funcs::pass1_1028_b354(param_1);
    param_1.field_0x0 = (s_SinternalPutBldg2_site_0x_08lx__1050_506f + 1);
    (param_1 + 2) = &PTR_LOOP_1050_1028;
    return param_1;
}

pub fn pass1_1028_4ba6(param_1: i32, param_2: u16, param_3: u16, param_3_00: u32) {
    pass4_funcs::pass1_1028_b39e(CONCAT22(param_2, param_1), param_3, param_3_00);
    CONCAT22(param_2, param_1) = (s_SinternalPutBldg2_site_0x_08lx__1050_506f + 1);
    (param_1 + 2) = &PTR_LOOP_1050_1028;
    return CONCAT22(param_2, param_1);
}

pub fn pass1_1028_4bd0(param_1: u32) {
    let mut u_var1: u16;

  // u_var1 = (param_1  >> 0x10);
    if (((param_1 + 0x12) != 6) && ((param_1 + 0x12) != 5)) {
        return 0;
    }
    return 1;
}

pub fn pass1_1028_4bf2(param_1: &mut Struct44, param_2: Vec<u8>) {
    let pp_var1: fn();
    let u_var2: u8;
    let pu_var3: &mut  u32;
    let paVar4: &mut  Struct493;
    let mut i_var5: i32;
    let extraout_var: u32;
    let extraout_var_00: u32;
    let mut in_dx: u16;

    let mut u_var6: u16;
    let mut unaff_ss: u16;
    let mut u_var7: u32;
    let mut u_var8: u32;
    let mut local_36: u32;
    let mut local_32: u16;
    let mut local_2c: [u8; 6];
    let mut local_26: u16;
    let mut local_24: u16;
    let mut local_22: u32;
    let mut local_1a: u16;
    let mut local_18: u32;
    let mut local_14: u32;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u32;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    u_var2 = pass4_funcs::pass1_1028_b58e(param_1);
    local_6 = CONCAT31(extraout_var, u_var2);
    local_14 = (local_6 + 0xc);
    local_8 = (local_6 + 0x10);
    local_1a = &local_c;
    local_22 = CONCAT22(local_22, &local_14);
    local_10 = local_8 + 1;
    u_var8 = CONCAT22(unaff_ss, local_2c);
    local_e = local_10;
    local_c = local_14;
    u_var7 = pass4_funcs::pass1_1028_bb24(param_1);
    pu_var3 = &local_14;
    pass1_1030_64ce(
        _PTR_LOOP_1050_5740,
        pu_var3,
        unaff_ss,
        u_var7,
        (u_var7 >> 0x10),
        u_var8,
    );
    unsafe { local_22 = *pu_var3 };
    u_var6 = (pu_var3 + 2);
    local_36._3_1_ = (local_22 >> 0x18);
    local_18 = local_22;
    if (local_36._3_1_ != '\0') {
        paVar4 = pass4_funcs::pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, local_22, u_var6);
        local_36 = CONCAT22(u_var6, paVar4);
        u_var2 = pass1_1030_6fa0(CONCAT22(u_var6, paVar4));
        i_var5 = CONCAT31(extraout_var_00, u_var2);
        if ((i_var5 == 0x62) || (i_var5 == 99)) {
            _local_26 = pass1_1030_73a8(local_36);
            pp_var1 = (*_local_26 + 0x58);
            (**pp_var1)(0x1030, _local_26, (_local_26 >> 0x10), param_2);
        }
    }
    pass4_funcs::pass1_1028_b46e(param_1, param_2);
    return;
}

pub fn pass1_1028_4cd6(param_1: &mut Struct44) {
    let pp_var1: fn();
    let u_var2: u8;
    let pu_var3: &mut  u32;
    let paVar4: &mut  Struct493;
    let mut i_var5: i32;
    let extraout_var: u32;
    let extraout_var_00: u32;
    let mut in_dx: u16;

    let mut u_var6: u16;
    let mut unaff_ss: u16;
    let mut u_var7: u32;
    let mut u_var8: u32;
    let mut local_36: u32;
    let mut local_32: u16;
    let mut local_2c: [u8; 6];
    let mut local_26: u16;
    let mut local_24: u16;
    let mut local_22: u32;
    let mut local_1a: u16;
    let mut local_18: u32;
    let mut local_14: u32;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u32;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    pass4_funcs::pass1_1028_b514(param_1);
    u_var2 = pass4_funcs::pass1_1028_b58e(param_1);
    local_6 = CONCAT31(extraout_var, u_var2);
    local_14 = (local_6 + 0xc);
    local_8 = (local_6 + 0x10);
    local_1a = &local_c;
    local_22 = CONCAT22(local_22, &local_14);
    local_10 = local_8 + 1;
    u_var8 = CONCAT22(unaff_ss, local_2c);
    local_e = local_10;
    local_c = local_14;
    u_var7 = pass4_funcs::pass1_1028_bb24(param_1);
    pu_var3 = &local_14;
    pass1_1030_64ce(
        _PTR_LOOP_1050_5740,
        pu_var3,
        unaff_ss,
        u_var7,
        (u_var7 >> 0x10),
        u_var8,
    );
    unsafe { local_22 = *pu_var3 };
    u_var6 = (pu_var3 + 2);
    local_36._3_1_ = (local_22 >> 0x18);
    if (local_36._3_1_ != '\0') {
        local_18 = local_22;
        paVar4 = pass4_funcs::pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, local_22, u_var6);
        local_36 = CONCAT22(u_var6, paVar4);
        u_var2 = pass1_1030_6fa0(CONCAT22(u_var6, paVar4));
        i_var5 = CONCAT31(extraout_var_00, u_var2);
        if ((i_var5 == 0x62) || (i_var5 == 99)) {
            _local_26 = pass1_1030_73a8(local_36);
            pp_var1 = (*_local_26 + 0x24);
            (**pp_var1)();
        }
    }
    return;
}

pub fn pass1_1028_4db2(param_1: i32, param_2: u16, param_2_00: i32) {
    let u_var1: u8;
    let BVar2: bool;
    let mut i_var3: i32;
    let extraout_var: u32;
    let mut u_var4: i32;
    let mut unaff_si: u16;
    let mut unaff_ss: u16;
    let ppVar5: &mut  Struct2551;
    let pu_var6: &mut  u16;
    let mut u_var7: u16;
    let pu_var8: &mut  u16;
    let mut u_var9: u16;
    let mut local_152: u16;
    let mut local_150: u16;
    let mut local_14e: u16;
    let mut local_14c: u16;
    let mut local_2a: u32;
    let mut local_26: u32;
    let mut local_22: u16;
    let mut local_20: [u8; 2];
    let mut local_1e: [u8; 2];
    let mut local_1c: u32;
    let mut uStack24: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    BVar2 = pass1_1008_c6ae(ctx._PTR_LOOP_1050_06e0, (param_1 + 0xc), 0x29);
    if (BVar2 != 0) {
        pass4_funcs::pass1_1028_bd38(CONCAT22(param_2, param_1));
        if ((param_2_00 == 0) && ((param_1 + 0xc) == 0x13)) {
            ppVar5 = process_struct_1010_20ba(ctx.g_struct_var_1050_0ed0, CONCAT22(unaff_si, 8));
            pass1_1010_988c(ppVar5, (param_1 + 0xc));
        }
        _local_6 = process_struct_1010_20ba(ctx.g_struct_var_1050_0ed0, CONCAT22(unaff_si, 0x2f));
      // u_var4 = (_local_6  >> 0x10);
        local_a = (_local_6 + 0x20);
        pu_var8 = &local_c;
        pu_var6 = &local_e;
        u_var7 = unaff_ss;
        u_var9 = unaff_ss;
      // local_12 = pass4_funcs::pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, local_a, (local_a  >> 0x10));
        local_10 = u_var4;
        pass1_1030_5b1c(
            CONCAT22(u_var4, local_12),
            CONCAT22(u_var7, pu_var6),
            CONCAT22(u_var9, pu_var8),
        );
        u_var1 = pass4_funcs::pass1_1028_b58e(CONCAT22(param_2, param_1));
        i_var3 = CONCAT31(extraout_var, u_var1);
        _local_16 = CONCAT31(extraout_var, u_var1) & 0xffff | u_var4 << 0x10;
        local_1c = (i_var3 + 0xc);
        uStack24 = (i_var3 + 0x10);
        pass4_funcs::pass1_1028_c8ee(param_1, param_2, 1, CONCAT22(unaff_ss, &local_1c));
        pass1_1008_3eb4(
            CONCAT22(unaff_ss, &local_1c),
            CONCAT22(unaff_ss, &local_22),
            CONCAT22(unaff_ss, local_20),
            CONCAT22(unaff_ss, local_1e),
        );
        if (local_e < local_22) {
            pass1_1030_5b3e(CONCAT22(local_10, local_12), local_22, local_c);
            pass1_1030_5b1c(
                CONCAT22(local_10, local_12),
                CONCAT22(unaff_ss, &local_e),
                CONCAT22(unaff_ss, &local_c),
            );
        }
        local_26 = (_local_16 + 0x2e);
        local_2a = (local_26 + 4);
        pass1_1028_87f0(
            CONCAT22(unaff_ss, &local_14e),
            0,
            0,
            0x62,
            &local_1c,
            unaff_ss,
            local_2a,
            local_a,
        );
        pass1_1030_835a(ctx._g_bool_1050_5748, CONCAT22(unaff_ss, &local_14e));
        pass4_funcs::pass1_1028_ccd0(CONCAT22(param_2, param_1), CONCAT22(unaff_ss, &local_1c));
    }
    return;
}

pub fn pass1_1028_4f30(param_1: &mut Struct44, param_2: i32) {
    let u_var1: u8;
    let extraout_ah: u8;
    let mut in_dx: u16;
    let mut u_var2: u16;

    u_var1 = pass4_funcs::pass1_1028_b58e(param_1);
    if (param_2 == 0) {
        u_var2 = 0;
    } else {
        u_var2 = 1000;
    }
    pass1_1030_7d1c(
        CONCAT22(in_dx, CONCAT11(extraout_ah, u_var1)),
        u_var2,
        0x230000,
    );
    return;
}

pub fn pass1_1028_4f62(param_1: u32) -> bool {
    let u_var1: u8;
    let extraout_ah: u8;
    let mut in_dx: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    u_var1 = pass4_funcs::pass1_1028_b58e(param_1);
    if ((CONCAT11(extraout_ah, u_var1) + 0x10) == 0) {
        return 1;
    }
    return 0;
}

pub fn pass1_1028_4faa(param_1: u32) {
    let u_var1: u8;
    let BVar2: bool;
    let extraout_ah: u8;
    let paVar3: &mut  Struct493;
    let mut in_dx: u16;
    let mut u_var4: i32;
    let mut unaff_ss: u16;
    let mut u_var5: u32;
    let mut u_var6: u32;
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

    BVar2 = pass1_1028_4f62(param_1);
    if (BVar2 != 0) {
        return param_1;
    }
    u_var1 = pass4_funcs::pass1_1028_b58e(param_1);
    local_6 = CONCAT11(extraout_ah, u_var1);
    local_c = (local_6 + 0xc);
    local_8 = 0;
    u_var5 = pass4_funcs::pass1_1028_bb24(param_1);
    u_var6 = pass1_1030_627e(_PTR_LOOP_1050_5740, CONCAT22(unaff_ss, &local_c), u_var5);
  // u_var4 = (u_var6  >> 0x10);
    paVar3 = pass4_funcs::pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, u_var6, u_var4);
    if ((u_var4 | paVar3) == 0) {
        return 0;
    }
    u_var5 = pass1_1030_73a8(CONCAT22(u_var4, paVar3));
    return u_var5;
}

pub fn pass1_1028_504a(param_1: &mut Struct44, param_2: u8) -> &mut Struct44 {
    pass4_funcs::pass1_1028_b418(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1028_50d8(param_1: &mut  Struct763) -> &mut  Struct763 {
    pass4_funcs::pass1_1028_b354(param_1);
    param_1.field_0x0 = 0x5280;
    (param_1 + 2) = &PTR_LOOP_1050_1028;
    return param_1;
}

pub fn pass1_1028_50fa(param_1: i32, param_2: u16, param_3: u16, param_3_00: u32) -> u32 {
    pass4_funcs::pass1_1028_b39e(CONCAT22(param_2, param_1), param_3, param_3_00);
    *CONCAT22(param_2, param_1) = "\x02";
    (param_1 + 2) = &PTR_LOOP_1050_1028;
    return CONCAT22(param_2, param_1);
}

pub fn pass1_1028_5128(param_1: u16, param_2: u16, param_3: u16) {
    let u_var1: u8;
    let mut i_var2: i32;
    let extraout_var: u32;
    let mut u_var3: i32;
    let mut unaff_si: u16;
    let mut unaff_ss: u16;
    let pu_var4: &mut  u16;
    let mut u_var5: u16;
    let pu_var6: &mut  u16;
    let mut u_var7: u16;
    let mut local_152: u16;
    let mut local_14e: u16;
    let mut local_14c: u16;
    let mut local_2a: u32;
    let mut local_26: u32;
    let mut local_22: u16;
    let mut local_20: [u8; 2];
    let mut local_1e: [u8; 2];
    let mut local_1c: u32;
    let mut uStack24: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    pass4_funcs::pass1_1028_bd38(CONCAT22(param_2, param_1));
    _local_6 = process_struct_1010_20ba(ctx.g_struct_var_1050_0ed0, CONCAT22(unaff_si, 0x2f));
  // u_var3 = (_local_6  >> 0x10);
    local_a = (_local_6 + 0x20);
    pu_var6 = &local_c;
    pu_var4 = &local_e;
    u_var5 = unaff_ss;
    u_var7 = unaff_ss;
  // local_12 = pass4_funcs::pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, local_a, (local_a  >> 0x10));
    local_10 = u_var3;
    pass1_1030_5b1c(
        CONCAT22(u_var3, local_12),
        CONCAT22(u_var5, pu_var4),
        CONCAT22(u_var7, pu_var6),
    );
    u_var1 = pass4_funcs::pass1_1028_b58e(CONCAT22(param_2, param_1));
    i_var2 = CONCAT31(extraout_var, u_var1);
    _local_16 = CONCAT31(extraout_var, u_var1) & 0xffff | u_var3 << 0x10;
    local_1c = (i_var2 + 0xc);
    uStack24 = (i_var2 + 0x10);
    pass4_funcs::pass1_1028_c8ee(param_1, param_2, 1, CONCAT22(unaff_ss, &local_1c));
    pass1_1008_3eb4(
        CONCAT22(unaff_ss, &local_1c),
        CONCAT22(unaff_ss, &local_22),
        CONCAT22(unaff_ss, local_20),
        CONCAT22(unaff_ss, local_1e),
    );
    if (local_e < local_22) {
        pass1_1030_5b3e(CONCAT22(local_10, local_12), local_22, local_c);
        pass1_1030_5b1c(
            CONCAT22(local_10, local_12),
            CONCAT22(unaff_ss, &local_e),
            CONCAT22(unaff_ss, &local_c),
        );
    }
    local_26 = (_local_16 + 0x2e);
    local_2a = (local_26 + 4);
    pass1_1028_87f0(
        CONCAT22(unaff_ss, &local_14e),
        0,
        0,
        0x6f,
        &local_1c,
        unaff_ss,
        local_2a,
        local_a,
    );
    pass1_1030_835a(ctx._g_bool_1050_5748, CONCAT22(unaff_ss, &local_14e));
    pass4_funcs::pass1_1028_ccd0(CONCAT22(param_2, param_1), CONCAT22(unaff_ss, &local_1c));
    return;
}
