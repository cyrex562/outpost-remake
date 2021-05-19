use crate::bad_funcs::bad1::halt_baddata;
use crate::err_funcs::error_check_1000_17ce;
use crate::func_ptr_funcs::pass1_1038_7a5a;
use crate::mem_funcs::{alloc_mem_1000_07fc, alloc_mem_1000_1708};
use crate::other_funcs::ret_1030_e4ba;
use crate::pass::pass12_funcs::{pass1_1008_b820, pass1_1008_c6ae, pass1_1008_c6fa};
use crate::pass::pass14_funcs::{pass1_1008_3e54, pass1_1008_3e76, pass1_1008_5784, pass1_1008_5b12, pass1_fn_1008_612e};
use crate::pass::pass15_funcs::{infinite_loop_1020_ba94, pass1_1020_a43e, pass1_1020_a49a, pass1_1020_a6ee, pass1_1020_ba3e, pass1_1020_ba7e, pass1_1020_bae6, pass1_1020_bb16, pass1_1020_bb70, pass1_1020_bb8a, pass1_1020_c42e, pass1_1020_c444, switch_statement_1020_c3b4};
use crate::pass::pass16_funcs::{pass1_1028_6228, pass1_1028_62c8, pass1_1028_6408, pass1_1028_6744, pass1_1028_9944, pass1_1028_9992};
use crate::pass::pass17_funcs::{pass1_1030_17ce, pass1_1030_1d58, pass1_1030_1d7c, pass1_1030_2aaa, pass1_1030_314c, pass1_1030_355c, pass1_1030_3694, pass1_1030_38b8, pass1_1030_5b00, pass1_1030_6b16, pass1_1030_6c4c, pass1_1030_6c66, pass1_1030_6e9c, pass1_1030_73a8, pass1_1030_7c28, pass1_1030_7d1c, pass1_1030_7ddc, pass1_1030_8344, pass1_1030_835a, ret_1030_178e};
use crate::pass::pass20_funcs::pass1_1010_ed3e;
use crate::pass::pass3_funcs::{pass1_1028_0d80, pass1_1028_3c32, pass1_1028_45e2};
use crate::pass::pass4_funcs::{pass1_1028_b354, pass1_1028_b39e, pass1_1028_b418, pass1_1028_b46e, pass1_1028_b4f2, pass1_1028_b58e, pass1_1028_bdac, pass1_1028_c952, pass1_1028_cb04, pass1_1028_d1dc, pass1_1028_d52c, pass1_1028_dc52, pass1_1028_e1ec, pass1_1028_e332, pass1_1028_e4ec, pass1_1030_1312};
use crate::pass::pass5_funcs::{pass1_1030_bcae, pass1_1030_bcde, pass1_1030_bd74, pass1_1030_cbf0, pass1_1030_cc44, pass1_1030_cde8, pass1_1030_cef8, pass1_1030_d0a8, pass1_1030_d144, pass1_1030_d180};
use crate::pass::pass6_funcs;
use crate::pass::pass8_funcs::{pass1_1008_eb74, pass1_1010_043a, pass1_1010_089e};
use crate::pass::pass9_funcs::pass1_1008_de58;
use crate::pass::pass_funcs::pass1_1000_4906;
use crate::prog_structs::prog_structs_1::{Struct1065, Struct1067, Struct393};
use crate::prog_structs::prog_structs_14::Struct1036;
use crate::prog_structs::prog_structs_16::{Struct1010, Struct493, Struct982};
use crate::prog_structs::prog_structs_17::Struct983;
use crate::prog_structs::prog_structs_19::{Struct1050, Struct500};
use crate::prog_structs::prog_structs_21::{Struct1060, Struct1078};
use crate::prog_structs::prog_structs_22::Struct922;
use crate::prog_structs::prog_structs_24::{Struct1068, Struct2111};
use crate::prog_structs::prog_structs_25::Struct1021;
use crate::prog_structs::prog_structs_26::Struct1052;
use crate::prog_structs::prog_structs_27::{pass1_struct_2, Struct1082, Struct1083};
use crate::prog_structs::prog_structs_28::{Struct1008, Struct1009, Struct1012, Struct1013, Struct1014, Struct1015, Struct1017, Struct1018, Struct1045, Struct1058, Struct1059, Struct1061, Struct1062, Struct1063, Struct1064, Struct1066, Struct1069, Struct1070, Struct1076, Struct1077, Struct1079, Struct1080, Struct1081, Struct918};
use crate::prog_structs::prog_structs_29::{Struct763, Struct764, Struct990};
use crate::prog_structs::prog_structs_2::Struct199;
use crate::prog_structs::prog_structs_31::Struct981;
use crate::prog_structs::prog_structs_7::Struct44;
use crate::string_funcs::copy_string_1000_3d3e;
use crate::struct_funcs::{process_struct_1000_179c, struct_fn_1000_160a};
use crate::sys_funcs::pass1_1038_095e;
use crate::util::{CARRY2, CONCAT11, CONCAT12, CONCAT13, CONCAT22, CONCAT31, SBORROW2, SUB41, SUB42, ZEXT24};

pub unsafe fn pass1_1038_3608(param_1: Vec<u8>) {
    let mut u_var1: u16;

    u_var1 = (param_1 >> 0x10);
    error_check_1000_17ce((param_1 + 0x21a));
    (param_1 + 0x21a) = 0;
    return;
}

pub unsafe fn pass1_1038_2c82(
    param_1: u16,
    param_2: u16,
    param_1_00: u32,
    param_2_00: u32,
    param_5: u32,
) -> i32 {
    let pu_var1: *mut u32;
    let pi_var2: *mut i32;
    let mut u_var3: i32;
    let mut u_var4: u32;
    let ppcVar5: fn();
    let mut u_var6: i32;
    let pa_var7: *mut Struct493;
    let pp_var8: *mut pass1_struct_2;
    let mut i_var9: i32;
    let pu_var10: *mut u32;

    let local_bx_4: *mut Struct1079;
    let mut i_var11: i32;
    let local_bx_35: *mut Struct1080;
    let mut iVar12: i32;
    let mut u_var13: u16;
    let mut u_var14: u16;
    let mut u_var15: u16;
    let mut u_var16: u16;
    let mut unaff_ss: u16;
    let ppVar17: *mut Struct2111;
    let u_var18: u8;
    let mut in_stack_0000ffce: u16;
    let mut local_2a: u16;
    let mut local_28: u16;
    let mut local_26: u16;
    let mut local_24: u16;
    let mut local_22: u16;
    let mut local_1e: u16;
    let mut local_1c: u16;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_16: u32;
    let mut local_12: u32;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u32;
    let mut local_6: u32;

    u_var13 = (param_5 >> 0x10);
    local_bx_4 = param_5;
    local_6 = local_bx_4.field_0x200;
    u_var14 = (param_2_00 >> 0x10);
    i_var11 = param_2_00;
    local_a = (i_var11 + 0x200);
    u_var16 = (i_var11 + 0x202);
    u_var15 = (param_1_00 >> 0x10);
    local_bx_35 = param_1_00;
    i_var9 = local_bx_35.field_0xc;
    if (i_var9 == 1) {
        local_e = param_1_00;
        pass6_funcs::pass1_1038_52b8(param_5, &local_bx_35.field_0x8, &local_bx_35.field_0xe);
        return;
    }
    if (i_var9 == 2) {
        local_e = param_1_00;
        if (&local_bx_35.field_0xe != 0) {
            pass6_funcs::pass1_1038_3efc(local_bx_4, u_var13, param_2_00, *&local_bx_35.field_0xe);
            return;
        }
        pass1_1020_a43e(CONCAT22(unaff_ss, &local_12));
        local_16 = (local_e + 8);
        while (local_16 = local_16 - 1, (local_16._2_2_ | local_16) != 0) {
            pass1_1020_a6ee(
                CONCAT13((unaff_ss >> 8), CONCAT12(unaff_ss, &local_12)),
                (local_e + 0x12),
            );
        }
    } else {
        if (i_var9 == 3) {
            pass6_funcs::pass1_1038_3f38(param_5, param_2_00, &local_bx_35.field_0xe);
            return;
        }
        local_6._2_2_ = (local_6 >> 0x10);
        if (i_var9 == 4) {
            ctx.g_u16_ptr_1050_5f2e = (local_6._2_2_ & 0xff);
            if ((local_6 == (&ctx.PTR_LOOP_1050_0000 + 1)) && ((local_6 & 0xff0000) == 0)) {
                local_12 = local_bx_4.field_0x1f6;
                u_var4 = &local_bx_35.field_0x8;
                pass1_1030_3694(
                    local_12,
                    (local_12 >> 0x10),
                    &local_bx_35.field_0xe,
                    u_var4,
                    (u_var4 >> 0x10),
                );
                local_bx_35.field_0x10 = local_12;
                local_bx_35.field_0x12 = ctx.dx_reg;
            } else {
                if (ctx.__g_Struct94_ptr_1 == 0) {
                    _g_Struct94_ptr_1 = local_6;
                    struct_fn_1000_160a();
                } else {
                }
                alloc_mem_1000_1708(0x6c, 0, 1, _g_Struct94_ptr_1, ctx.g_u16_ptr_1050_5f2e);
                local_bx_35.field_0x10 = _g_Struct94_ptr_1;
                local_bx_35.field_0x12 = ctx.g_u16_ptr_1050_5f2e;
                i_var9 = &local_bx_35.field_0xe;
                if (i_var9 != 3) {
                    if (i_var9 != 4) {
                        u_var4 = &local_bx_35.field_0x10;
                        (u_var4 + 0x28) = &local_bx_35.field_0x8;
                        return;
                    }
                    u_var4 = &local_bx_35.field_0x10;
                    (u_var4 + 0xdc) = &local_bx_35.field_0x8;
                    return;
                }
                u_var4 = &local_bx_35.field_0x10;
                (u_var4 + 100) = &local_bx_35.field_0x8;
            }
        } else {
            if (i_var9 == 5) {
                if (&local_bx_35.field_0xe == 0xc) {
                    if ((local_6 == (&ctx.PTR_LOOP_1050_0000 + 1)) && ((local_6 & 0xff0000) == 0)) {
                        u_var4 = local_bx_4.field_0x1f6;
                        i_var9 = local_bx_35.field_0x8;
                        i_var11 = local_bx_35.field_0xa;
                        u_var6 = -i_var9;
                        u_var16 = (u_var4 >> 0x10);
                        iVar12 = u_var4;
                        pu_var1 = (iVar12 + 0x170);
                        unsafe {
                            u_var3 = *pu_var1;
                            *pu_var1 = *pu_var1 + u_var6;
                            pi_var2 = (iVar12 + 0x172);
                            *pi_var2 =
                                (*pi_var2 - (i_var11 + (i_var9 != 0))) + CARRY2(u_var3, u_var6);
                        }
                    }
                } else {
                    u_var16 = local_bx_35.field_0x8;
                    pass6_funcs::pass1_1038_43cc(
                        local_bx_4,
                        CONCAT13((u_var16 >> 8), CONCAT12(u_var16, u_var13)),
                        &local_bx_35.field_0xe,
                    );
                }
            } else {
                if (i_var9 == 7) {
                    u_var4 = &local_bx_35.field_0xe;
                    pa_var7 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, u_var4, (u_var4 >> 0x10));
                    pass6_funcs::pass1_1038_349e(CONCAT22(u_var16, pa_var7), (i_var11 + 0x200));
                    u_var18 = (u_var16 >> 8);
                    pass6_funcs::pass1_1038_4d0e(CONCAT13(u_var18, CONCAT12(u_var16, pa_var7)), 600);
                    pass6_funcs::pass1_1038_4d0e(CONCAT13(u_var18, CONCAT12(u_var16, pa_var7)), 600);
                    ppVar17 = process_struct_1010_20ba(
                        ctx._g_Struct372_1050_0ed0,
                        CONCAT22(in_stack_0000ffce, 0x3b),
                    );
                    pass1_1008_de58(ppVar17, &local_bx_35.field_0xe, 0x4000001);
                    ppVar17 = process_struct_1010_20ba(
                        ctx._g_Struct372_1050_0ed0,
                        CONCAT22(in_stack_0000ffce, 0x2f),
                    );
                    u_var16 = (ppVar17 >> 0x10);
                    pp_var8 = pass1_1030_8344(
                        ctx._g_bool_1050_5748,
                        (ctx._g_bool_1050_5748 >> 0x10),
                        (ppVar17 + 0x20),
                    );
                    local_12 = CONCAT22(u_var16, pp_var8);
                    i_var9 = pass1_1030_5b00(CONCAT22(u_var16, pp_var8));
                    local_e = process_struct_1010_20ba(
                        ctx._g_Struct372_1050_0ed0,
                        CONCAT22(in_stack_0000ffce, i_var9),
                    );
                    pu_var10 = (local_e + 0x20);
                    unsafe {
                        ppcVar5 = (*pu_var10 + 4);
                    }
                    (**ppcVar5)(0x1010, pu_var10, (local_e >> 0x10), 6);
                }
            }
        }
    }
    return;
}

pub unsafe fn pass1_1038_2f92(param_1: u16, param_2: u16, param_3: u16, param_2_00: u32) {
    let pu_var1: *mut u32;
    Struct1082 * *ppaVar2;
    let mut i_var3: i32;
    let mut u_var4: i32;
    let mut u_var5: i32;
    let mut u_var6: u32;
    let mut u_var7: u32;
    let local_bx_189: *mut Struct1081;
    let mut u_var8: u16;
    let mut in_stack_0000000e: u16;
    let mut local_1a: u32;
    let mut local_12: u32;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u32;
    let temp_5f8f06f4df: *mut Struct1082;

    u_var6 = (param_2_00._2_2_ + 0x200);
    i_var3 = (param_1_00 + 0xc);
    if (i_var3 == 1) {
        u_var7 = (param_1_00 + 8);
        pass6_funcs::pass1_1038_3cc0(
            CONCAT22(in_stack_0000000e, param_2_00._2_2_),
            u_var7,
            (u_var7 >> 0x10),
            (param_1_00 + 0xe),
        );
        return;
    }
    if (i_var3 == 4) {
        pass1_1030_355c((param_2_00._2_2_ + 0x1f6), (param_1_00 + 0x10));
        return;
    }
    if (i_var3 == 5) {
        if ((param_1_00 + 0xe) != 0xc) {
            pass6_funcs::pass1_1038_5798(
                CONCAT22(in_stack_0000000e, param_2_00._2_2_),
                (param_1_00 + 8),
                (param_1_00 + 0xe),
            );
            return;
        }
        local_a._0_2_ = u_var6;
        if ((local_a == 1) && ((u_var6 & 0xff0000) == 0)) {
            u_var7 = (param_2_00._2_2_ + 0x1f6);
            u_var4 = (param_1_00 + 8);
            temp_5f8f06f4df = (param_1_00 + 10);
            u_var8 = (u_var7 >> 0x10);
            local_bx_189 = u_var7;
            pu_var1 = &local_bx_189.field_0x170;
            u_var5 = *pu_var1;
            *pu_var1 = *pu_var1 + u_var4;
            ppaVar2 = &local_bx_189.field_0x172;
            *ppaVar2 = temp_5f8f06f4df + (*ppaVar2 + CARRY2(u_var5, u_var4));
            return;
        }
    }
    return;
}

pub unsafe fn pass1_1038_3074(param_1: &mut Struct44, param_2: u8) -> &mut Struct44 {
    pass1_1038_2a5c(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1038_30aa(param_1: *mut Struct393) {
    let u_var1: u8;
    let mut u_var2: i32;
    let extraout_var: u32;
    let in_dx: *mut Struct199;
    let struct_a: *mut Struct199;

    let mut u_var3: u16;
    let local_bx_19: *mut Struct1083;
    let mut u_var4: u16;
    let mut local_4: u16;

    pass1_1030_17ce(param_1, 0, 0);
    u_var4 = (param_1 >> 0x10);
    local_bx_19 = param_1;
    local_bx_19.field_0x10 = 0;
    local_bx_19.field_0x14 = 0;
    local_bx_19.field_0x18 = 600;
    local_bx_19.field_0x1a = 600;
    local_bx_19.field_0x1c = 0;
    local_bx_19.field_0x1e = 0;
    local_bx_19.field_0x22 = 0;
    local_bx_19.field_0x24 = 0x32;
    &local_bx_19.field_0x1f6 = 0;
    local_bx_19.field_0x1fa = 0;
    local_bx_19.field_0x1fe = 0;
    local_bx_19.field_0x200 = 0x8000001;
    local_bx_19.field_0x204 = 0;
    local_bx_19.field_0x206 = 0;
    local_bx_19.field_0x208 = 1;
    local_bx_19.field_0x20a = 0;
    local_bx_19.field_0x20c = 0;
    local_bx_19.field_0x20e = 0;
    local_bx_19.field_0x210 = 0;
    local_bx_19.field_0x214 = 0;
    local_bx_19.field_0x216 = 0;
    local_bx_19.field_0x21a = 0;
    param_1.field_0x0 = 0x6504;
    local_bx_19.field_0x2 = &PTR_LOOP_1050_1038;
    pass1_1000_4906((param_1 & 0xffff0000 | &local_bx_19.field_0x26), 0, 0x94);
    pass1_1000_4906((param_1 & 0xffff0000 | &local_bx_19.field_0xba), 0, 0x94);
    pass1_1000_4906((param_1 & 0xffff0000 | &local_bx_19.field_0x14e), 0, 0x54);
    u_var1 = pass1_1000_4906((param_1 & 0xffff0000 | &local_bx_19.field_0x1a2), 0, 0x54);
    u_var2 = CONCAT31(extraout_var, u_var1);
    process_struct_1000_179c(0x1b0, in_dx);
    struct_a = (in_dx | u_var2);
    if (struct_a == 0x0) {
        &local_bx_19.field_0x1f6 = 0;
    } else {
        pass1_1030_314c(CONCAT22(in_dx, u_var2), local_bx_19.field_0x4);
        local_bx_19.field_0x1f6 = u_var2;
        local_bx_19.field_0x1f8 = struct_a;
    }
    process_struct_1000_179c(0x1e, struct_a);
    if ((struct_a | u_var2) == 0) {
        u_var2 = 0;
        u_var3 = 0;
    } else {
        pass1_1020_c444(CONCAT22(struct_a, u_var2), 100, 200);
        u_var3 = ctx.dx_reg;
    }
    local_bx_19.field_0xc = u_var2;
    local_bx_19.field_0xe = u_var3;
    return;
}

pub unsafe fn pass1_1038_29d2(param_1: &mut Struct44, param_2: u8) -> &mut Struct44 {
    param_1.ptr_a_lo = ctx.s_1_1050_389a;
    (param_1 + 2) = &ctx.PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1038_2a0e(
    param_1: *mut Struct500,
    param_2: u32,
    param_3: u32,
    param_4: u32,
    param_5: u32,
) -> i32 {
    let local_bx_19: *mut Struct500;
    let mut local_es_19: u16;

    pass1_1028_d1dc(param_1, s_fem133_wav_1050_2af7);
    local_es_19 = (param_1 >> 0x10);
    local_bx_19 = param_1;
    local_bx_19.field_0x108 = param_5;
    local_bx_19.field_0x10c = param_4;
    local_bx_19.field_0x110 = param_3;
    local_bx_19.field_0x114 = param_2;
    param_1.a = 0x309a;
    local_bx_19.b = &PTR_LOOP_1050_1038;
    return;
}

pub unsafe fn pass1_1038_2a5c(param_1: *mut u16) {
    let pu_var1: *mut u32;
    let mut u_var2: i32;
    let ppc_var3: fn();
    let local_bx_5: *mut Struct1076;
    let mut u_var4: u16;

    u_var4 = (param_1 >> 0x10);
    local_bx_5 = param_1;
    *param_1 = 0x309a;
    local_bx_5.field_0x2 = &PTR_LOOP_1050_1038;
    pu_var1 = local_bx_5.field_0x114;
    u_var2 = local_bx_5.field_0x116;
    if ((u_var2 | pu_var1) != 0) {
        ppc_var3 = *pu_var1;
        (**ppc_var3)();
    }
    pu_var1 = local_bx_5.field_0x110;
    u_var2 = local_bx_5.field_0x112;
    if ((u_var2 | pu_var1) != 0) {
        ppc_var3 = *pu_var1;
        (**ppc_var3)();
    }
    *param_1 = ctx.s_1_1050_389a;
    local_bx_5.field_0x2 = &ctx.PTR_LOOP_1050_1008;
    return;
}

pub unsafe fn pass1_1038_2ac2(param_1: *mut Struct1077) {
    let mut u_var1: u32;
    let paVar2: *mut Struct493;
    let mut in_dx: u16;
    let local_bx_4: *mut Struct1077;
    let mut u_var3: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    u_var3 = (param_1 >> 0x10);
    local_bx_4 = param_1;
    u_var1 = local_bx_4.field_0x108;
    paVar2 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, u_var1, (u_var1 >> 0x10));
    _local_6 = CONCAT22(in_dx, paVar2);
    u_var1 = local_bx_4.field_0x10c;
    paVar2 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, u_var1, (u_var1 >> 0x10));
    local_a = CONCAT22(in_dx, paVar2);
    pass1_1038_2c82(
        local_bx_4,
        u_var3,
        local_bx_4.field_0x110,
        CONCAT22(in_dx, paVar2),
        _local_6,
    );
    pass1_1038_2c82(
        local_bx_4,
        u_var3,
        local_bx_4.field_0x114,
        _local_6,
        local_a,
    );
    return 1;
}

pub unsafe fn pass1_1038_2b2e(param_1: *mut Struct1078) {
    let mut u_var1: u32;
    let paVar2: *mut Struct493;
    let paVar3: *mut Struct493;
    let local_bx_4: *mut Struct1078;
    let local_es_4: *mut Struct1078;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    local_es_4 = (param_1 >> 0x10);
    local_bx_4 = param_1;
    u_var1 = local_bx_4.field_0x108;
    paVar2 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, u_var1, (u_var1 >> 0x10));
    u_var1 = local_bx_4.field_0x10c;
    paVar3 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, u_var1, (u_var1 >> 0x10));
    u_var1 = local_bx_4.field_0x110;
    pass1_1038_2f92(
        local_bx_4,
        local_es_4,
        u_var1,
        CONCAT22(paVar3, (u_var1 >> 0x10)),
    );
    u_var1 = local_bx_4.field_0x114;
    pass1_1038_2f92(
        local_bx_4,
        local_es_4,
        u_var1,
        CONCAT22(paVar2, (u_var1 >> 0x10)),
    );
    return 1;
}

pub unsafe fn pass1_1038_0d8e(
    param_1: u16,
    param_2: u16,
    param_1_00: u32,
    param_2_00: &mut Struct44,
) {
    let mut u_var1: u16;
    let mut u_var2: i32;
    let mut u_var3: u32;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    u_var1 = pass1_1030_d0a8(param_2_00);
    u_var2 = pass1_1030_d144(param_2_00);
    local_a = u_var2;
    local_4 = u_var1;
    if ((u_var2 >> 0xf | u_var2) != 0) {
        while {
            u_var3 = pass1_1028_6744(param_1_00, local_4);
            if (u_var3 != 0) {
                pass1_1028_6228(param_1_00, 1, 0, local_4);
                local_a = local_a + -1;
                pass1_1030_d180(param_2_00, local_4);
            }
            if (local_a == 0) {
                return;
            }
            local_4 = pass1_1030_d0a8(param_2_00);
            local_4 != u_var1
        } {}
    }
    return;
}

pub unsafe fn pass1_1038_0e00(param_1: u32, param_2: u32, param_3: u32) {
    let pp_var1: fn();

    let paVar2: *mut Struct493;


    let mut u_var3: i32;
    let mut u_var4: u32;
    let mut local_14: u16;
    let mut local_c: u16;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    pp_var1 = (param_2 + 0x10);
    (**pp_var1)();
    _local_6 = CONCAT22(ctx.dx_reg, in_ax);
    local_a = 0;
    while (local_a < _local_6) {
        pp_var1 = (param_2 + 4);
        u_var4 = _local_6;
        (**pp_var1)();
        u_var3 = ctx.dx_reg | u_var4;
        if (u_var3 != 0) {
            paVar2 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, u_var4, ctx.dx_reg);
            u_var4 = pass1_1030_73a8(CONCAT22(u_var3, paVar2));
            u_var3 = (u_var4 >> 0x10);
            if ((u_var3 | u_var4) != 0) {
                pass1_1038_0d8e(
                    param_1,
                    (param_1 >> 0x10),
                    u_var4 & 0xffff | u_var3 << 0x10,
                    param_3,
                );
            }
        }
        local_a = local_a + 1;
    }
    return;
}

pub unsafe fn pass1_1038_0e78(param_1: u32, param_2: u32) {
    let pp_var1: fn();
    let mut u_var2: u32;
    let mut u_var3: i32;
    let mut u_var4: u16;
    let mut u_var5: u16;
    let paVar6: *mut Struct493;
    let pu_var7: Vec<u8>;
    let mut in_dx: i32;


    let mut u_var8: i32;



    let mut u_var9: u16;
    let mut u_var10: u32;
    let mut local_28: u16;
    let mut local_20: u16;
    let mut local_16: u32;
    let mut local_12: u16;
    let mut local_10: u16;
    let local_e: *mut Struct1058;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;

    pu_var7 = pass1_1008_c6fa(ctx._PTR_LOOP_1050_06e0, 4);
    u_var3 = pu_var7;
    pass6_funcs::pass1_1038_4d6e(param_2, pu_var7 & 0xffff | in_dx << 0x10);
    local_a = CONCAT22(ctx.dx_reg, u_var3);
    u_var2 = *local_a;
    pp_var1 = u_var2 + 8;
    u_var8 = u_var3;
    (**pp_var1)(&ctx.PTR_LOOP_1050_1008, u_var3, ctx.dx_reg);
    u_var8 = ctx.dx_reg | u_var8;
    if (u_var8 == 0) {
        if (local_a != 0x0) {
            pp_var1 = u_var2;
            (**pp_var1)(8, u_var3, ctx.dx_reg, 1);
            return;
        }
    } else {
        u_var9 = SUB42(&ctx.PTR_LOOP_1050_1008, 0);
        pu_var7 = pass1_1008_c6fa(ctx._PTR_LOOP_1050_06e0, 0x1e);
        u_var4 = SUB42(pu_var7, 0);
        pass6_funcs::pass1_1038_4d6e(param_2, pu_var7 & 0xffff | u_var8 << 0x10);
        local_e = CONCAT22(ctx.dx_reg, u_var4);
        pp_var1 = (*local_e + 0x10);
        u_var5 = u_var4;
        (**pp_var1)(&ctx.PTR_LOOP_1050_1008, u_var4, ctx.dx_reg);
        _local_12 = CONCAT22(ctx.dx_reg, u_var5);
        local_16 = 0;
        while (local_16 < _local_12) {
            pp_var1 = (*local_e + 4);
            u_var10 = _local_12;
            (**pp_var1)();
            u_var8 = ctx.dx_reg | u_var10;
            if (u_var8 != 0) {
                paVar6 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, u_var10, ctx.dx_reg);
                u_var9 = 0x1030;
                u_var10 = pass1_1030_73a8(CONCAT22(u_var8, paVar6));
                if (((u_var10 >> 0x10) | u_var10) != 0) {
                    pass1_1038_0e00(param_1, local_a, u_var10);
                }
            }
            local_16 = local_16 + 1;
        }
        if (local_a != 0x0) {
            pp_var1 = *local_a;
            (**pp_var1)(u_var9, u_var3, ctx.dx_reg, 1);
        }
        if (local_e != 0x0) {
            pp_var1 = *local_e;
            (**pp_var1)(u_var9, u_var4, ctx.dx_reg, 1);
        }
    }
    return;
}

pub unsafe fn pass1_1038_0f8c(param_1: u16, param_2: u16, param_1_00: *mut u32, param_2_00: u32) {
    let pu_var1: *mut u16;
    let mut u_var2: u16;
    let mut u_var3: u16;
    let mut u_var4: u16;
    let mut u_var5: i32;
    let ppcVar6: fn();
    let mut u_var7: u32;
    let Var8: u16;

    let local_AX_95: *mut Struct1059;
    let pu_var9: Vec<u8>;
    let local_AX_405: *mut Struct1060;
    let mut u_var10: u32;
    let mut u_var12: i32;
    let mut u_var13: i32;
    let mut in_edx: u32;
    let local_bx_491: *mut Struct1061;
    let mut u_var14: u16;
    let mut unaff_cs: u16;
    let mut unaff_ss: u16;
    let pu_var15: *mut u32;
    let mut local_60: u32;
    let mut local_58: u16;
    let mut local_56: u16;
    let mut local_50: u32;
    let mut local_4c: u16;
    let mut local_4a: u16;
    let mut local_48: u16;
    let mut local_46: u16;
    let mut local_44: u16;
    let mut local_42: u16;
    let mut local_40: u16;
    let mut local_3e: u16;
    let mut local_3c: u16;
    let mut local_34: u16;
    let mut local_32: u16;
    let mut local_30: [u8; 4];
    let mut local_2c: u32;
    let mut local_28: u32;
    let mut local_24: u16;
    let mut local_22: u16;
    let mut local_20: [u8; 4];
    let mut local_1c: u32;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u32;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;
    let pu_var11: Vec<u8>;

    local_6 = 100;
    local_8 = 0;
    ppcVar6 = (*param_1_00 + 0x10);
    pu_var15 = param_1_00;
    (**ppcVar6)();
    _local_c = CONCAT22(in_edx, in_ax);
    local_10 = 0;
    loop {
        if (_local_c <= local_10) {
            return;
        }
        ppcVar6 = (*param_1_00 + 4);
        u_var10 = _local_c;
        u_var13 = in_edx;
        (**ppcVar6)(unaff_cs, param_1_00, local_10, (local_10 >> 0x10), pu_var15);
        local_12 = u_var13;
        local_14 = u_var10;
        u_var13 = local_12 | local_14;
        in_edx = u_var13;
        if (u_var13 != 0) {
            local_18 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, local_14, local_12);
            local_16 = u_var13;
            unaff_cs = 0x1030;
            local_1c = pass1_1030_73a8(CONCAT22(local_16, local_18));
            in_edx = local_1c >> 0x10;
            pu_var9 = local_20;
            ppcVar6 = (local_1c + 0x40);
            (**ppcVar6)(0x1030, local_1c, (local_1c >> 0x10), pu_var9);
            if (pu_var9 == 0x0) {
                _local_24 = pass1_1028_62c8(local_1c);
                u_var10 = _local_24 >> 0x10;
                local_8 = 1;
                local_28 = (param_2_00 + 0x22);
                pass1_1008_5784(CONCAT22(unaff_ss, local_30), local_28);
                loop {
                    u_var13 = u_var10;
                    local_AX_95 = local_30;
                    unaff_cs = SUB42(&ctx.PTR_LOOP_1050_1008, 0);
                    pass1_1008_5b12(CONCAT22(unaff_ss, local_AX_95));
                    in_edx = (u_var13 | local_AX_95);
                    if ((u_var13 | local_AX_95) == 0) {
                        break;
                    }
                    u_var2 = local_AX_95.field_0x4;
                    u_var3 = local_AX_95.field_0x6;
                    u_var4 = local_AX_95.field_0x8;
                    u_var5 = local_AX_95.field_0xa;
                    u_var7 = local_AX_95.field_0xc / u_var5;
                    u_var10 = _local_24;
                    if (local_6 < _local_24) {
                        u_var10 = local_6 & 0xffff;
                        local_22 = local_6._2_2_;
                    }
                    u_var12 = local_22 | u_var10;
                    in_edx = u_var12;
                    if (u_var12 == 0) {
                        break;
                    }
                    qVar8 = (u_var10 & 0xffff | local_22 << 0x10) / u_var7;
                    in_edx = qVar8 >> 0x10;
                    u_var12 = (qVar8 >> 0x10);
                    local_4c = qVar8;
                    if (local_4c == 0) {
                        break;
                    }
                    if (local_4c < u_var5) {
                        pu_var1 = &local_AX_95.field_0xc;
                        *pu_var1 = *pu_var1 - u_var10;
                        pu_var1 = &local_AX_95.field_0xa;
                        *pu_var1 = *pu_var1 - local_4c;
                    } else {
                        ppcVar6 = (local_28 + 0xc);
                        (**ppcVar6)(
                            &ctx.PTR_LOOP_1050_1008,
                            local_28,
                            (local_28 >> 0x10),
                            local_AX_95,
                            u_var13,
                        );
                        local_2c = 0;
                        local_4c = u_var5;
                    }
                    pu_var11 = _PTR_LOOP_1050_68a2;
                    alloc_mem_1000_07fc(_PTR_LOOP_1050_68a2);
                    u_var13 = pu_var11;
                    local_50 = pu_var11 & 0xffff | u_var12 << 0x10;
                    if ((u_var12 | u_var13) == 0) {
                        local_50 = 0;
                    } else {
                        local_50 = ctx.s_1_1050_389a;
                        (u_var13 + 2) = &ctx.PTR_LOOP_1050_1008;
                        (u_var13 + 4) = 0;
                        (u_var13 + 6) = 0;
                        (u_var13 + 8) = 0;
                        (u_var13 + 10) = 0;
                        (u_var13 + 0xc) = 0;
                        local_50 = 0x56ce;
                        (u_var13 + 2) = 0x1018;
                    }
                    u_var14 = (local_50 >> 0x10);
                    local_bx_491 = local_50;
                    local_bx_491.field_0xa = local_4c;
                    u_var7 = local_4c * u_var7;
                    u_var10 = u_var7 >> 0x10;
                    local_bx_491.field_0xc = u_var7;
                    local_bx_491.field_0x4 = u_var2;
                    local_bx_491.field_0x6 = u_var3;
                    local_bx_491.field_0x8 = u_var4;
                    pass1_1028_6408(local_1c, local_50);
                }
            } else {
                ppcVar6 = (*param_1_00 + 8);
                (**ppcVar6)(0x1030, param_1_00, 0, 0, local_10, (local_10 >> 0x10));
            }
        }
        local_10 = local_10 + 1;
    }
}

pub unsafe fn pass1_1038_11b0(param_1: u32, param_2: u32, param_3: u32) {
    let pp_var1: fn();

    let paVar2: *mut Struct493;
    let mut i_var3: i32;


    let mut u_var4: u16;
    let mut u_var5: u32;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    pp_var1 = (param_3 + 0x10);
    (**pp_var1)();
    _local_6 = CONCAT22(ctx.dx_reg, in_ax);
    local_a = 0;
    loop {
        if (_local_6 <= local_a) {
            return;
        }
        pp_var1 = (param_3 + 4);
        u_var5 = _local_6;
        (**pp_var1)();
        u_var4 = ctx.dx_reg;
        paVar2 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, u_var5, ctx.dx_reg);
        u_var5 = pass1_1030_73a8(CONCAT22(u_var4, paVar2));
        i_var3 = u_var5;
        pass1_1038_0f8c(param_1, (param_1 >> 0x10), param_2, u_var5);
        if (i_var3 == 0) {
            break;
        }
        local_a = local_a + 1;
    }
    return;
}

pub unsafe fn pass1_1038_1220(param_1: u32, param_2: u32) {
    let pp_var1: fn();
    let mut u_var2: u32;
    let mut i_var3: i32;
    let mut i_var4: i32;
    let mut i_var5: i32;
    let pu_var6: Vec<u8>;
    let mut in_dx: i32;


    let mut u_var7: i32;



    let mut extraout_dx_04: i32;
    let mut extraout_dx_05: u16;
    let mut extraout_dx_06: i32;
    let u_var8: u8;
    let mut local_e: u16;
    let mut local_c: u16;
    let local_a: *mut Struct1062;
    let mut local_8: u16;

    pu_var6 = pass1_1008_c6fa(ctx._PTR_LOOP_1050_06e0, 4);
    i_var3 = pu_var6;
    pass6_funcs::pass1_1038_4d6e(param_2, pu_var6 & 0xffff | in_dx << 0x10);
    local_a = CONCAT22(ctx.dx_reg, i_var3);
    pp_var1 = (*local_a + 0x10);
    i_var4 = i_var3;
    (**pp_var1)(&ctx.PTR_LOOP_1050_1008, i_var3, ctx.dx_reg);
    if ((ctx.dx_reg != 0) || (i_var4 != 0)) {
        u_var7 = ctx.dx_reg;
        pu_var6 = pass1_1008_c6fa(ctx._PTR_LOOP_1050_06e0, 5);
        i_var5 = pu_var6;
        pass6_funcs::pass1_1038_4d6e(param_2, pu_var6 & 0xffff | u_var7 << 0x10);
        local_e = CONCAT22(ctx.dx_reg, i_var5);
        u_var8 = i_var5;
        u_var2 = *local_e;
        pp_var1 = u_var2 + 8;
        i_var4 = i_var5;
        (**pp_var1)(&ctx.PTR_LOOP_1050_1008, u_var8, ctx.dx_reg);
        if (((ctx.dx_reg != 0) || (u_var7 = ctx.dx_reg, i_var4 != 0))
            && (
                pass1_1038_11b0(
                    param_1,
                    CONCAT13((ctx.dx_reg >> 8), CONCAT12(ctx.dx_reg, i_var3)),
                    CONCAT22(ctx.dx_reg, i_var5),
                ),
                u_var7 = ctx.dx_reg,
                i_var4 == 0,
            ))
        {
            if (local_e == 0x0) {
                return;
            }
            pp_var1 = u_var2;
            (**pp_var1)(8, u_var8, ctx.dx_reg, 1);
            return;
        }
        if (local_e != 0x0) {
            pp_var1 = *local_e;
            (**pp_var1)(8, u_var8, ctx.dx_reg, 1);
            u_var7 = extraout_dx_04;
        }
        pu_var6 = pass1_1008_c6fa(ctx._PTR_LOOP_1050_06e0, 6);
        i_var5 = pu_var6;
        pass6_funcs::pass1_1038_4d6e(param_2, pu_var6 & 0xffff | u_var7 << 0x10);
        local_e = CONCAT22(extraout_dx_05, i_var5);
        pp_var1 = (*local_e + 0x10);
        i_var4 = i_var5;
        (**pp_var1)(8, i_var5, extraout_dx_05);
        if ((extraout_dx_06 != 0) || (i_var4 != 0)) {
            pass1_1038_11b0(
                param_1,
                CONCAT22(ctx.dx_reg, i_var3),
                CONCAT22(extraout_dx_05, i_var5),
            );
        }
        if (local_e != 0x0) {
            pp_var1 = *local_e;
            (**pp_var1)(8, i_var5, extraout_dx_05, 1);
        }
    }
    if (local_a != 0x0) {
        pp_var1 = *local_a;
        (**pp_var1)(8, i_var3, ctx.dx_reg, 1);
    }
    return;
}

pub unsafe fn pass1_1038_134a(
    param_1: u16,
    param_2: u16,
    param_1_00: *mut u32,
    param_2_00: *mut u32,
    param_5: *mut u32,
) -> i32 {
    let pp_var1: fn();

    let paVar2: *mut Struct493;


    let mut u_var3: u16;
    let mut u_var4: i32;
    let mut unaff_cs: u16;
    let mut u_var5: u32;
    let mut u_var6: u32;
    let pu_var7: *mut u32;
    let mut local_1a: u32;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    pp_var1 = (*param_5 + 0x10);
    pu_var7 = param_5;
    (**pp_var1)();
    _local_6 = CONCAT22(ctx.dx_reg, in_ax);
    *param_1_00 = 0;
    while {
        if (_local_6 <= *param_2_00) {
            return;
        }
        u_var5 = *param_2_00;
        *param_2_00 = *param_2_00 + 1;
        pp_var1 = (*param_5 + 4);
        (**pp_var1)(unaff_cs, param_5, u_var5, pu_var7);
        u_var3 = ctx.dx_reg;
        paVar2 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, u_var5, ctx.dx_reg);
        u_var5 = pass1_1030_73a8(CONCAT22(u_var3, paVar2));
        unaff_cs = SUB42(&PTR_LOOP_1050_1028, 0);
        u_var6 = pass1_1028_45e2(u_var5);
        u_var4 = (u_var6 >> 0x10);
        param_1_00 = u_var6;
        (param_1_00 + 2) = u_var4;
        (u_var4 | param_1_00) == 0
    } {}
    return;
}

pub unsafe fn pass1_1038_13da(
    param_1: u16,
    param_2: u16,
    param_1_00: *mut u32,
    param_2_00: *mut u32,
    param_5: *mut u32,
) -> i32 {
    let pp_var1: fn();

    let paVar2: *mut Struct493;


    let mut u_var3: u16;
    let mut u_var4: i32;
    let mut unaff_cs: u16;
    let mut u_var5: u32;
    let pu_var6: Vec<u8>;
    let pu_var7: *mut u32;
    let mut local_1a: u32;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_10: u16;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    pp_var1 = (*param_5 + 0x10);
    pu_var7 = param_5;
    (**pp_var1)();
    _local_6 = CONCAT22(ctx.dx_reg, in_ax);
    *param_1_00 = 0;
    while {
        if (_local_6 <= *param_2_00) {
            return;
        }
        u_var5 = *param_2_00;
        *param_2_00 = *param_2_00 + 1;
        pp_var1 = (*param_5 + 4);
        (**pp_var1)(unaff_cs, param_5, u_var5, pu_var7);
        u_var3 = ctx.dx_reg;
        paVar2 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, u_var5, ctx.dx_reg);
        if ((u_var3 | paVar2) == 0) {
            return;
        }
        u_var5 = pass1_1030_73a8(CONCAT22(u_var3, paVar2));
        u_var4 = (u_var5 >> 0x10);
        if ((u_var4 | u_var5) == 0) {
            return;
        }
        unaff_cs = SUB42(&PTR_LOOP_1050_1028, 0);
        pu_var6 = pass1_1028_3c32((u_var5 & 0xffff | u_var4 << 0x10));
        u_var4 = (pu_var6 >> 0x10);
        param_1_00 = pu_var6;
        (param_1_00 + 2) = u_var4;
        (u_var4 | param_1_00) == 0
    } {}
    return;
}

pub unsafe fn pass1_1038_1482(param_1: u32, param_2: u32, param_3: u32) {
    let pp_var1: fn();
    let Var2: u16;
    let mut u_var3: u16;
    let u_var4: u8;
    let pu_var5: *mut u32;
    let mut u_var6: i32;
    let mut u_var7: u16;
    let extraout_var: u32;
    let extraout_var_00: u32;
    let mut u_var8: u32;
    let mut in_dx: u16;
    let paVar9: *mut Struct199;
    let pa_var10: *mut Struct199;
    let mut u_var11: i32;
    let mut unaff_ss: u16;
    let mut u_var12: u16;
    let u_var13: u8;
    let u_var14: u8;
    let mut u_var15: u16;
    let mut local_4e: u16;
    let mut local_4c: u16;
    let mut local_4a: u16;
    let mut local_48: u16;
    let mut local_46: u32;
    let mut local_42: u16;
    let mut local_3a: u16;
    let mut local_38: u16;
    let mut local_36: u32;
    let mut local_32: u32;
    let mut local_2e: u16;
    let mut local_2c: u16;
    let mut local_2a: u16;
    let mut local_28: u16;
    let mut local_26: u16;
    let mut local_24: u16;
    let mut local_22: u32;
    let mut local_1e: u16;
    let mut local_1c: u16;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_16: u32;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u32;
    let mut local_6: u32;

    local_6 = 0;
    local_a = 0;
    pu_var5 = &local_a;
    u_var12 = (param_1 >> 0x10);
    u_var3 = param_1;
    pass1_1038_134a(
        u_var3,
        u_var12,
        CONCAT22(unaff_ss, pu_var5),
        CONCAT22(unaff_ss, &local_6),
        param_3,
    );
    local_e = CONCAT22(in_dx, pu_var5);
    pp_var1 = (param_2 + 0x10);
    (**pp_var1)();
    _local_12 = CONCAT22(in_dx, pu_var5);
    local_16 = 0;
    loop {
        if (_local_12 <= local_16) {
            return;
        }
        if ((local_c | local_e) == 0) {
            return;
        }
        u_var4 = pass1_1028_b58e(local_e);
        u_var11 = CONCAT31(extraout_var_00, u_var4);
        local_1a = u_var11;
        local_18 = local_10;
        pass1_1038_1a30(
            u_var3,
            u_var12,
            CONCAT31(extraout_var_00, u_var4) & 0xffff | local_10 << 0x10,
        );
        local_1e = u_var11;
        local_1c = local_10;
        if ((local_10 | u_var11) != 0) {
            sVar2 = CONCAT22(local_10, u_var11) * 100;
            u_var7 = (sVar2 >> 0x20);
            u_var8 = sVar2 >> 1;
            pp_var1 = (param_2 + 4);
            local_22 = u_var8;
            (**pp_var1)(&PTR_LOOP_1050_1028, param_2, local_16, (local_16 >> 0x10));
            local_26 = u_var8;
            local_24 = u_var7;
            local_2a = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, local_26, u_var7);
            local_28 = u_var7;
            _local_2e = pass1_1030_73a8(CONCAT22(u_var7, local_2a));
            local_32 = (_local_2e + 0x28);
            local_36 = 0;
            local_38 = (local_32 + 4);
            local_3a = 0;
            while (local_3a < local_38) {
                pass1_1020_bb16(
                    local_32,
                    CONCAT22(unaff_ss, &local_46),
                    CONCAT13((unaff_ss >> 8), CONCAT12(unaff_ss, &local_42)),
                    local_3a,
                );
                if (((local_46 != 0) && (0xd < local_42)) && (local_42 < 0x1d)) {
                    u_var8 = local_46;
                    if (local_22 < local_46) {
                        u_var8 = local_22 & 0xffff;
                        local_46._2_2_ = local_22._2_2_;
                    }
                    u_var11 = u_var8;
                    if ((local_a._2_2_ <= local_46._2_2_)
                        && (local_a._2_2_ < local_46._2_2_ || (local_a < u_var11)))
                    {
                        u_var11 = local_a;
                        local_46._2_2_ = local_a._2_2_;
                    }
                    local_4a = CONCAT22(local_46._2_2_, u_var11);
                    local_22 = CONCAT22(
                        local_22._2_2_ + (-(local_22 < u_var11) - local_46._2_2_),
                        local_22 - u_var11,
                    );
                    local_a = CONCAT22(
                        local_a._2_2_ + (-(local_a < u_var11) - local_46._2_2_),
                        local_a - u_var11,
                    );
                    pa_var10 = local_46._2_2_;
                    if (local_36 == 0) {
                        paVar9 = local_46._2_2_;
                        u_var6 = u_var11;
                        process_struct_1000_179c(10, local_46._2_2_);
                        pa_var10 = (paVar9 | u_var6);
                        if (pa_var10 == 0x0) {
                            u_var6 = 0;
                            pa_var10 = 0x0;
                        } else {
                            pass1_1020_ba3e(CONCAT22(paVar9, u_var6), 5, 5);
                        }
                        local_36 = CONCAT22(pa_var10, u_var6);
                    }
                    pass1_1020_bb8a(
                        local_36,
                        (local_36 >> 0x10),
                        u_var11,
                        local_46._2_2_,
                        local_42,
                    );
                    pass1_1020_bb8a(
                        local_32,
                        (local_32 >> 0x10),
                        (local_46 - local_4a),
                        (local_46 - local_4a >> 0x10),
                        local_42,
                    );
                    if (local_a == 0) {
                        pass1_1038_1b3a(u_var3, u_var12, local_e, local_36);
                        local_36 = 0;
                        pu_var5 = &local_a;
                        pass1_1038_134a(
                            u_var3,
                            u_var12,
                            CONCAT22(unaff_ss, pu_var5),
                            CONCAT22(unaff_ss, &local_6),
                            param_3,
                        );
                        local_e = CONCAT22(pa_var10, pu_var5);
                        u_var11 = pa_var10 | pu_var5;
                        if (u_var11 != 0) {
                            u_var13 = 100;
                            u_var14 = 0;
                            u_var15 = 0;
                            u_var4 = pass1_1028_b58e(CONCAT22(pa_var10, pu_var5));
                            u_var7 = CONCAT31(extraout_var, u_var4);
                            local_1a = u_var7;
                            local_18 = u_var11;
                            pass1_1038_1a30(
                                u_var3,
                                u_var12,
                                CONCAT31(extraout_var, u_var4) & 0xffff | u_var11 << 0x10,
                            );
                            local_22 = (CONCAT22(u_var11, u_var7)
                                * CONCAT22(u_var15, CONCAT11(u_var14, u_var13)))
                                >> 1;
                            local_1e = u_var7;
                            local_1c = u_var11;
                        }
                    }
                    if ((local_22 == 0) || (local_a == 0)) {
                        break;
                    }
                }
                local_3a = local_3a + 1;
            }
            if (local_36 != 0) {
                pass1_1038_1b3a(u_var3, u_var12, local_e, local_36);
                local_36 = 0;
            }
        }
        local_16 = local_16 + 1;
    }
}

pub unsafe fn pass1_1038_16f2(param_1: u32, param_2: u32, param_3: u32) {
    let mut u_var1: u32;
    let ppc_var2: fn();
    let mut u_var3: u16;
    let mut u_var4: u16;
    let pu_var5: *mut u32;
    let mut u_var6: i32;
    let pa_var7: *mut Struct493;
    let mut u_var8: u16;
    let mut u_var9: i32;
    let lVar10: u32;
    let mut u_var11: u32;
    let mut in_dx: i32;
    let mut u_var12: i32;
    let struct_a: *mut Struct199;
    let mut u_var13: u32;
    let mut u_var14: u32;
    let mut unaff_cs: u16;
    let mut unaff_ss: u16;
    let mut u_var15: u32;
    let mut u_var16: u16;
    let mut u_var17: u32;
    let mut local_48: u16;
    let mut local_46: u16;
    let mut local_44: u32;
    let mut local_40: u32;
    let mut local_38: u16;
    let mut local_36: u16;
    let mut local_34: u16;
    let mut local_32: u16;
    let mut local_30: u16;
    let mut local_2e: u32;
    let mut local_2a: u32;
    let mut local_26: u16;
    let mut local_24: u16;
    let mut local_22: u16;
    let mut local_20: u16;
    let mut local_1e: u16;
    let mut local_1c: u16;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_16: u32;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u32;
    let mut local_6: u32;

    local_6 = 0;
    local_a = 0;
    pu_var5 = &local_a;
    u_var16 = (param_1 >> 0x10);
    u_var3 = param_1;
    pass1_1038_13da(
        u_var3,
        u_var16,
        CONCAT22(unaff_ss, pu_var5),
        CONCAT22(unaff_ss, &local_6),
        param_3,
    );
    local_e = CONCAT22(in_dx, pu_var5);
    in_dx = in_dx | pu_var5;
    if (in_dx != 0) {
        ppc_var2 = (param_2 + 0x10);
        u_var17 = param_2;
        ppc_var2();
        _local_12 = CONCAT22(in_dx, pu_var5);
        local_16 = 0;
        while (local_16 < _local_12) {
            ppc_var2 = (param_2 + 4);
            u_var15 = _local_12;
            u_var6 = in_dx;
            ppc_var2(unaff_cs, param_2, local_16, (local_16 >> 0x10), u_var17);
            pa_var7 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, u_var15, u_var6);
            unaff_cs = 0x1030;
            u_var15 = pass1_1030_73a8(CONCAT22(u_var6, pa_var7));
            u_var12 = (u_var15 >> 0x10);
            u_var9 = u_var15;
            pass1_1038_1a30(u_var3, u_var16, CONCAT22(u_var6, pa_var7));
            if ((u_var12 | u_var9) != 0) {
                local_2a = (CONCAT22(u_var12, u_var9) * 100) >> 1;
                u_var1 = &pa_var7[1].field_0x4;
                u_var9 = &pa_var7[1].field_0x6;
                u_var13 = u_var9;
                local_2e._0_2_ = u_var1;
                if ((u_var9 | local_2e) != 0) {
                    _local_32 = 0;
                    u_var8 = pass1_1028_0d80(u_var15);
                    local_38 = 0;
                    local_34 = u_var8;
                    loop {
                        u_var12 = u_var13;
                        u_var4 = (u_var1 >> 0x10);
                        u_var11 = pass1_1020_bae6(local_2e, CONCAT22(local_34, u_var4));
                        u_var9 = u_var11;
                        u_var13 = (u_var12 | u_var9);
                        if ((u_var12 | u_var9) != 0) {
                            u_var14 = u_var12;
                            if ((local_2a._2_2_ <= u_var12)
                                && (local_2a._2_2_ < u_var12 || (local_2a < u_var9)))
                            {
                                u_var14 = local_2a._2_2_;
                                u_var9 = local_2a;
                            }
                            if ((local_a._2_2_ <= u_var14)
                                && (local_a._2_2_ < u_var14 || (local_a < u_var9)))
                            {
                                u_var14 = local_a._2_2_;
                                u_var9 = local_a;
                            }
                            struct_a = u_var14;
                            local_44 = CONCAT22(struct_a, u_var9);
                            local_2a = CONCAT22(
                                (local_2a._2_2_ - struct_a) - (local_2a < u_var9),
                                local_2a - u_var9,
                            );
                            local_a = CONCAT22(
                                (local_a._2_2_ - struct_a) - (local_a < u_var9),
                                local_a - u_var9,
                            );
                            u_var13 = u_var14;
                            if (_local_32 == 0) {
                                u_var6 = u_var9;
                                process_struct_1000_179c(10, struct_a);
                                u_var13 = (struct_a | u_var6);
                                if ((struct_a | u_var6) == 0) {
                                    u_var6 = 0;
                                    u_var13 = 0;
                                } else {
                                    pass1_1020_ba3e(CONCAT22(struct_a, u_var6), 5, 5);
                                }
                                _local_32 = CONCAT22(u_var13, u_var6);
                            }
                            pass1_1020_bb8a(
                                _local_32,
                                (_local_32 >> 0x10),
                                u_var9,
                                u_var14,
                                local_34,
                            );
                            lVar10 = (u_var11 & 0xffff | u_var12 << 0x10) - local_44;
                            pass1_1020_bb8a(local_2e, u_var4, lVar10, (lVar10 >> 0x10), local_34);
                            u_var9 = u_var13;
                            local_38 = local_34;
                            if (local_a == 0) {
                                pass1_1038_1ac6(u_var3, u_var16, local_e, _local_32);
                                _local_32 = 0;
                                pu_var5 = &local_a;
                                pass1_1038_13da(
                                    u_var3,
                                    u_var16,
                                    CONCAT22(unaff_ss, pu_var5),
                                    CONCAT22(unaff_ss, &local_6),
                                    param_3,
                                );
                                local_e = CONCAT22(u_var9, pu_var5);
                                u_var13 = (u_var9 | pu_var5);
                                if ((u_var9 | pu_var5) == 0) {
                                    return;
                                }
                            }
                        }
                        unaff_cs = 0x1020;
                        if ((local_2a == 0) || (local_a == 0)) {
                            break;
                        }
                        unaff_cs = SUB42(&PTR_LOOP_1050_1028, 0);
                        local_34 = pass1_1028_0d80(u_var15);
                        if ((local_34 == local_38) || (local_38 == 0 && (local_34 == u_var8))) {
                            break;
                        }
                    }
                    if (_local_32 != 0) {
                        pass1_1038_1ac6(u_var3, u_var16, local_e, _local_32);
                    }
                }
            }
            local_16 = local_16 + 1;
        }
    }
    return;
}

pub unsafe fn pass1_1038_1940(param_1: u32, param_2: u32, param_3: u32) {
    let pp_var1: fn();
    let mut u_var2: i32;
    let mut u_var3: i32;
    let pu_var4: Vec<u8>;
    let mut in_dx: i32;


    let local_a: *mut Struct1063;
    let mut local_8: u16;

    pu_var4 = pass1_1008_c6fa(ctx._PTR_LOOP_1050_06e0, 3);
    u_var2 = pu_var4;
    pass6_funcs::pass1_1038_4d6e(param_3, pu_var4 & 0xffff | in_dx << 0x10);
    local_a = CONCAT22(ctx.dx_reg, u_var2);
    pp_var1 = (*local_a + 0x10);
    u_var3 = u_var2;
    (**pp_var1)(&ctx.PTR_LOOP_1050_1008, u_var2, ctx.dx_reg);
    if ((ctx.dx_reg | u_var3) != 0) {
        pass1_1038_1482(param_1, param_2, local_a);
    }
    if (local_a != 0x0) {
        pp_var1 = *local_a;
        (**pp_var1)(&ctx.PTR_LOOP_1050_1008, u_var2, ctx.dx_reg, 1);
    }
    return;
}

pub unsafe fn pass1_1038_19a0(param_1: u32, param_2: u32, param_3: u32) {
    let pp_var1: fn();
    let mut u_var2: u32;
    let mut u_var3: i32;
    let mut u_var4: i32;
    let pu_var5: Vec<u8>;
    let mut in_dx: i32;


    let mut local_a: u16;
    let mut local_8: u16;

    pu_var5 = pass1_1008_c6fa(ctx._PTR_LOOP_1050_06e0, 2);
    u_var3 = pu_var5;
    pass6_funcs::pass1_1038_4d6e(param_3, pu_var5 & 0xffff | in_dx << 0x10);
    local_a = CONCAT22(ctx.dx_reg, u_var3);
    u_var2 = *local_a;
    pp_var1 = u_var2 + 8;
    u_var4 = u_var3;
    (**pp_var1)(&ctx.PTR_LOOP_1050_1008, u_var3, ctx.dx_reg);
    if ((ctx.dx_reg | u_var4) == 0) {
        wvsprintf_FUN_1030_840a(0xdf, &ctx.g_alloc_addr_1050_1050);
        if (local_a != 0x0) {
            pp_var1 = u_var2;
            (**pp_var1)(0x1030, u_var3, ctx.dx_reg, 1);
            return;
        }
    } else {
        pass1_1038_16f2(param_1, local_a, param_2);
        if (local_a != 0x0) {
            pp_var1 = *local_a;
            (**pp_var1)(&ctx.PTR_LOOP_1050_1008, u_var3, ctx.dx_reg, 1);
        }
    }
    return;
}

pub unsafe fn pass1_1038_1a30(param_1: u16, param_2: u16, param_1_00: u32) {
    let pu_var1: *mut u32;
    let ppc_var2: fn();
    let mut u_var3: i32;
    let mut u_var4: u32;


    let mut u_var5: u16;
    let mut unaff_cs: u16;
    let mut u_var6: i32;
    let mut u_var7: i32;
    let mut local_1e: u16;
    let mut local_1c: u16;
    let mut local_18: u16;
    let mut local_12: u32;
    let mut local_e: u32;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;

    u_var5 = (param_1_00 >> 0x10);
    pu_var1 = (param_1_00 + 0x1e);
    u_var7 = (param_1_00 + 0x20);
    local_6._0_2_ = pu_var1;
    u_var3 = u_var7 | local_6;
    if (u_var3 != 0) {
        ppc_var2 = (*pu_var1 + 0x10);
        u_var6 = local_6;
        ppc_var2();
        local_a = CONCAT22(ctx.dx_reg, u_var3);
        local_12 = 0;
        while (local_12 < local_a) {
            ppc_var2 = (*pu_var1 + 4);
            u_var4 = local_a;
            ppc_var2(
                unaff_cs,
                local_6,
                (pu_var1 >> 0x10),
                local_12,
                u_var6,
                u_var7,
            );
            if ((ctx.dx_reg | u_var4) != 0) {
                unaff_cs = SUB42(&PTR_LOOP_1050_1028, 0);
                pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, u_var4, ctx.dx_reg);
            }
            local_12 = local_12 + 1;
        }
        return;
    }
    return;
}

pub unsafe fn pass1_1038_1ac6(
    param_1: u16,
    param_2: u16,
    param_1_00: &mut Struct44,
    param_2_00: u32,
) {
    let u_var1: u8;
    let extraout_var: u32;
    let mut in_dx: i32;
    let mut unaff_ss: u16;
    let mut local_120: u32;
    let mut local_11c: u32;
    let mut local_118: u16;
    let mut local_116: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    u_var1 = pass1_1028_b58e(param_1_00);
    _local_6 = CONCAT31(extraout_var, u_var1) & 0xffff | in_dx << 0x10;
    pas1_1030_e8a0(
        CONCAT22(unaff_ss, &local_118),
        param_2_00,
        (param_1_00 + 0xc),
        (CONCAT31(extraout_var, u_var1) + 4),
    );
    pass1_1028_d52c(
        *ctx._g_bool_1050_5748,
        *ctx._PTR_LOOP_1050_65e2 + 1,
        CONCAT22(unaff_ss, &local_118),
    );
    return;
}

pub unsafe fn pass1_1038_1b3a(
    param_1: u16,
    param_2: u16,
    param_1_00: &mut Struct44,
    param_2_00: &mut Struct44,
) -> i32 {
    let u_var1: u8;
    let extraout_var: u32;
    let mut in_dx: i32;
    let mut unaff_ss: u16;
    let mut local_22: u32;
    let mut local_1e: u32;
    let mut local_1a: u32;
    let mut local_16: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    u_var1 = pass1_1028_b58e(param_1_00);
    _local_6 = CONCAT31(extraout_var, u_var1) & 0xffff | in_dx << 0x10;
    local_a = param_1_00;
    local_e = pass1_1028_45e2(param_1_00);
    local_10 = (param_2_00 + 4);
    local_12 = 0;
    while (local_12 < local_10) {
        pass1_1020_bb16(
            param_2_00,
            CONCAT22(unaff_ss, &local_1a),
            CONCAT22(unaff_ss, &local_16),
            local_12,
        );
        if (local_e < local_1a) {
            pass1_1030_7ddc(_local_6, local_e, local_16);
            local_e = 0;
        } else {
            local_e = local_e - local_1a;
            pass1_1030_7ddc(_local_6, local_1a, local_16);
        }
        if (local_e == 0) {
            break;
        }
        local_12 = local_12 + 1;
    }
    if (param_2_00 != 0x0) {
        pass1_1020_ba7e(param_2_00);
        error_check_1000_17ce(param_2_00);
    }
    return;
}

pub unsafe fn pass1_1038_1c02(param_1: &mut Struct44, param_2: u8) -> &mut Struct44 {
    param_1.ptr_a_lo = ctx.s_1_1050_389a;
    (param_1 + 2) = &ctx.PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1038_1c3e(param_1: u32, param_2: u32) {
    let pu_var1: *mut u32;
    let ppc_var2: fn();
    let mut u_var3: u32;
    let paVar4: *mut Struct493;
    let mut i_var5: i32;
    let b_var6: bool;
    let pu_var7: *mut u32;


    let mut u_var8: i32;
    let mut u_var9: u16;
    let mut unaff_cs: u16;
    let mut u_var10: u32;
    let mut u_var11: u16;
    let mut u_var12: u16;
    let mut u_var13: u16;
    let mut local_26: u16;
    let mut local_24: u16;
    let mut local_22: u16;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_14: u16;
    let mut local_e: u32;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;

    u_var9 = (param_2 >> 0x10);
    pu_var1 = (param_2 + 0xc);
    u_var9 = (param_2 + 0xe);
    ppc_var2 = (*pu_var1 + 0x10);
    pu_var7 = pu_var1;
    u_var13 = pu_var1;
    ppc_var2();
    u_var3 = pu_var7 & 0xffff | ctx.dx_reg << 0x10;
    local_e = 0;
    loop {
        if (u_var3 <= local_e) {
            return;
        }
        ppc_var2 = (*pu_var1 + 4);
        u_var10 = u_var3;
        ppc_var2(
            unaff_cs,
            pu_var1,
            (pu_var1 >> 0x10),
            local_e,
            u_var13,
            u_var9,
        );
        u_var8 = ctx.dx_reg | u_var10;
        if (u_var8 != 0) {
            unaff_cs = SUB42(&PTR_LOOP_1050_1028, 0);
            paVar4 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, u_var10, ctx.dx_reg);
            _local_1a = CONCAT22(u_var8, paVar4);
            i_var5 = &paVar4[1].field_0x16;
            if ((i_var5 != 0) && ((&paVar4[1].field_0x16 + 2) != 0)) {
                u_var11 = param_1;
                u_var12 = (param_1 >> 0x10);
                pass1_1038_201a(u_var11, u_var12, CONCAT22(u_var8, paVar4));
                if (i_var5 == 0) {
                    u_var10 = pass1_1030_73a8(_local_1a);
                    i_var5 = (u_var10 + 0xc);
                    unaff_cs = SUB42(&ctx.PTR_LOOP_1050_1008, 0);
                    b_var6 = pass1_1008_c6ae(ctx._PTR_LOOP_1050_06e0, i_var5, 1);
                    if (b_var6 == 0) {
                        unaff_cs = SUB42(&ctx.PTR_LOOP_1050_1008, 0);
                        b_var6 = pass1_1008_c6ae(ctx._PTR_LOOP_1050_06e0, i_var5, 2);
                        if (b_var6 == 0) {
                            b_var6 = pass1_1008_c6ae(ctx._PTR_LOOP_1050_06e0, i_var5, 5);
                            if (b_var6 == 0) {
                                unaff_cs = SUB42(&ctx.PTR_LOOP_1050_1008, 0);
                                b_var6 = pass1_1008_c6ae(ctx._PTR_LOOP_1050_06e0, i_var5, 6);
                                if (b_var6 == 0) {}
                                // goto LAB_1038_1c76;
                            }
                            unaff_cs = SUB42(&ctx.PTR_LOOP_1050_1008, 0);
                            pass1_1038_2306(u_var11, u_var12, _local_1a);
                        } else {
                            pass1_1038_26ee(u_var11, u_var12, _local_1a);
                        }
                    } else {
                        pass1_1038_24e8(u_var11, u_var12, _local_1a);
                    }
                }
            }
        }
        // LAB_1038_1c76:
        local_e = local_e + 1;
    }
}

pub unsafe fn pass1_1038_1d68(param_1: u16, param_2: u16, param_1_00: *mut u32, param_2_00: u32) {
    let pu_var1: *mut u16;
    let mut u_var2: u16;
    let mut u_var3: u16;
    let mut u_var4: i32;
    let ppcVar5: fn();
    let Var6: u16;
    let mut u_var7: u32;
    let mut u_var8: i32;
    let mut bVar9: bool;

    let local_AX_95: *mut Struct1064;
    let pu_var10: Vec<u8>;
    let local_AX_435: *mut Struct1066;
    let mut u_var11: u32;
    let mut u_var13: i32;
    let mut u_var14: i32;
    let mut in_edx: u32;
    let local_bx_521: *mut Struct1067;
    let mut unaff_cs: u16;
    let mut unaff_ss: u16;
    let pu_var15: *mut u32;
    let mut local_62: u32;
    let mut local_5a: u16;
    let mut local_58: u16;
    let mut local_52: u32;
    let mut local_4e: u16;
    let mut local_4c: u16;
    let mut local_4a: u16;
    let mut local_48: u16;
    let mut local_46: u16;
    let mut local_44: u16;
    let mut local_42: u16;
    let mut local_40: u16;
    let mut local_3e: u16;
    let mut local_3c: u16;
    let mut local_34: u16;
    let mut local_32: u16;
    let mut local_30: [u8; 4];
    let mut local_2c: u32;
    let mut local_28: u32;
    let mut local_24: u16;
    let mut local_22: u16;
    let mut local_20: [u8; 4];
    let mut local_1c: u32;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u32;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;
    let temp_5f69f6c76a: *mut Struct1065;
    let pu_var12: Vec<u8>;

    local_6 = 100;
    local_8 = 0;
    ppcVar5 = (*param_1_00 + 0x10);
    pu_var15 = param_1_00;
    (**ppcVar5)();
    _local_c = CONCAT22(in_edx, in_ax);
    local_10 = 0;
    loop {
        if (_local_c <= local_10) {
            return;
        }
        ppcVar5 = (*param_1_00 + 4);
        u_var11 = _local_c;
        u_var14 = in_edx;
        (**ppcVar5)(unaff_cs, param_1_00, local_10, pu_var15);
        local_12 = u_var14;
        local_14 = u_var11;
        u_var14 = local_12 | local_14;
        in_edx = u_var14;
        if (u_var14 != 0) {
            local_18 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, local_14, local_12);
            local_16 = u_var14;
            unaff_cs = 0x1030;
            local_1c = pass1_1030_73a8(CONCAT22(local_16, local_18));
            in_edx = local_1c >> 0x10;
            pu_var10 = local_20;
            ppcVar5 = (local_1c + 0x40);
            (**ppcVar5)(0x1030, local_1c, (local_1c >> 0x10), pu_var10, unaff_ss);
            if (pu_var10 == 0x0) {
                _local_24 = pass1_1028_62c8(local_1c);
                u_var11 = _local_24 >> 0x10;
                local_8 = 1;
                local_28 = (param_2_00 + 0x22);
                pass1_1008_5784(CONCAT22(unaff_ss, local_30), local_28);
                loop {
                    u_var14 = u_var11;
                    local_AX_95 = local_30;
                    unaff_cs = SUB42(&ctx.PTR_LOOP_1050_1008, 0);
                    pass1_1008_5b12(CONCAT22(unaff_ss, local_AX_95));
                    _local_34 = CONCAT22(u_var14, local_AX_95);
                    in_edx = (u_var14 | local_AX_95);
                    if ((u_var14 | local_AX_95) == 0) {
                        break;
                    }
                    u_var2 = local_AX_95.field_0x4;
                    temp_5f69f6c76a = local_AX_95.field_0x6;
                    u_var3 = local_AX_95.field_0x8;
                    u_var13 = local_AX_95.field_0xc;
                    u_var4 = local_AX_95.field_0xa;
                    u_var8 = u_var13 / u_var4;
                    u_var11 = u_var13 % u_var4;
                    bVar9 = false;
                    if (((0 < temp_5f69f6c76a) && (!SBORROW2(temp_5f69f6c76a, 1)))
                        && (temp_5f69f6c76a == (&PTR_DAT_0005_0000_1050_0004 + 1)
                            || (temp_5f69f6c76a + -1) < 4
                            || (temp_5f69f6c76a == &BYTE_1050_0008)))
                    {
                        bVar9 = true;
                    }
                    if (bVar9) {
                        u_var11 = _local_24;
                        if (local_6 < _local_24) {
                            u_var11 = local_6 & 0xffff;
                            local_22 = local_6._2_2_;
                        }
                        u_var13 = local_22 | u_var11;
                        in_edx = u_var13;
                        if (u_var13 == 0) {
                            break;
                        }
                        qVar6 = (u_var11 & 0xffff | local_22 << 0x10) / u_var8;
                        u_var13 = (qVar6 >> 0x10);
                        local_4e = qVar6;
                        if (local_4e < u_var4) {
                            pu_var1 = &local_AX_95.field_0xc;
                            *pu_var1 = *pu_var1 - u_var11;
                            pu_var1 = &local_AX_95.field_0xa;
                            *pu_var1 = *pu_var1 - local_4e;
                        } else {
                            ppcVar5 = (local_28 + 0xc);
                            (**ppcVar5)(
                                &ctx.PTR_LOOP_1050_1008,
                                local_28,
                                (local_28 >> 0x10),
                                _local_34,
                            );
                            local_2c = 0;
                            local_4e = u_var4;
                        }
                        pu_var12 = _PTR_LOOP_1050_68a2;
                        alloc_mem_1000_07fc(_PTR_LOOP_1050_68a2);
                        u_var14 = pu_var12;
                        local_52 = pu_var12 & 0xffff | u_var13 << 0x10;
                        if ((u_var13 | u_var14) == 0) {
                            local_52 = 0;
                        } else {
                            local_52 = ctx.s_1_1050_389a;
                            (u_var14 + 2) = &ctx.PTR_LOOP_1050_1008;
                            (u_var14 + 4) = 0;
                            (u_var14 + 6) = 0;
                            (u_var14 + 8) = 0;
                            (u_var14 + 10) = 0;
                            (u_var14 + 0xc) = 0;
                            local_52 = 0x56ce;
                            (u_var14 + 2) = 0x1018;
                        }
                        u_var14 = (local_52 >> 0x10);
                        local_bx_521 = local_52;
                        local_bx_521.field_0xa = local_4e;
                        u_var7 = local_4e * u_var8;
                        u_var11 = u_var7 >> 0x10;
                        local_bx_521.field_0xc = u_var7;
                        local_bx_521.field_0x4 = u_var2;
                        local_bx_521.field_0x6 = temp_5f69f6c76a;
                        local_bx_521.field_0x8 = u_var3;
                        pass1_1028_6408(local_1c, (local_52 & 0xffff | u_var14 << 0x10));
                    }
                }
            } else {
                ppcVar5 = (*param_1_00 + 8);
                (**ppcVar5)(0x1030, param_1_00, 0, local_10);
            }
        }
        local_10 = local_10 + 1;
    }
}

pub unsafe fn pass1_1038_1faa(param_1: u32, param_2: u32, param_3: u32) {
    let pp_var1: fn();

    let paVar2: *mut Struct493;
    let mut i_var3: i32;


    let mut u_var4: u16;
    let mut u_var5: u32;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    pp_var1 = (param_3 + 0x10);
    (**pp_var1)();
    _local_6 = CONCAT22(ctx.dx_reg, in_ax);
    local_a = 0;
    loop {
        if (_local_6 <= local_a) {
            return;
        }
        pp_var1 = (param_3 + 4);
        u_var5 = _local_6;
        (**pp_var1)();
        u_var4 = ctx.dx_reg;
        paVar2 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, u_var5, ctx.dx_reg);
        u_var5 = pass1_1030_73a8(CONCAT22(u_var4, paVar2));
        i_var3 = u_var5;
        pass1_1038_1d68(param_1, (param_1 >> 0x10), param_2, u_var5);
        if (i_var3 == 0) {
            break;
        }
        local_a = local_a + 1;
    }
    return;
}

pub unsafe fn pass1_1038_201a(param_1: u16, param_2: u16, param_1_00: *mut Struct918) {
    let piVar1: *mut i32;
    let mut i_var2: i32;
    let ppc_var3: fn();
    let lVar4: u32;
    let mut u_var5: i32;
    let mut u_var6: i32;
    let pu_var7: Vec<u8>;
    let mut u_var8: i32;
    let mut u_var9: u16;
    let mut u_var10: u32;
    let mut u_var11: i32;
    let mut u_var12: u32;
    let mut u_var13: u32;
    let pu_var14: Vec<u8>;
    let pu_var15: Vec<u8>;
    let mut u_var16: u16;
    let pu_var17: *mut u32;
    let struct_a: *mut Struct922;
    let mut u_var18: u16;
    let mut local_30: u16;
    let mut local_2e: u16;
    let mut local_28: u32;
    let mut local_24: u16;
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

    struct_a = param_1_00;
    u_var18 = (param_1_00 >> 0x10);
    u_var16 = 0x1030;
    pu_var17 = pass1_1030_6b16(struct_a);
    u_var6 = (pu_var17 >> 0x10);
    u_var5 = pu_var17;
    if ((u_var6 | u_var5) == 0) {
        return;
    }
    i_var2 = &struct_a.field_0x34;
    lVar4 = i_var2;
    u_var13 = lVar4 * 100;
    pu_var7 = (u_var13 >> 0x10);
    local_a = 0;
    _local_14 = 0;
    if ((u_var5 + 4) == 0) {
        if ((u_var5 + 6) == 0) {
            if ((u_var5 + 8) == 0) {}
            // goto LAB_1038_2102;
            u_var9 = pass1_1020_c42e((u_var5 + 8));
            u_var12 = *(u_var5 + 10) * u_var9;
            pu_var14 = (u_var12 >> 0x10);
            if (u_var12 + lVar4 * -100 != 0 && u_var13 <= u_var12) {
                u_var12 = u_var13 & 0xffff;
                pu_var14 = pu_var7;
            }
            u_var13 = u_var12 & 0xffff | ZEXT24(pu_var14) << 0x10;
            u_var10 = (u_var12 & 0xffff | ZEXT24(pu_var14) << 0x10) / u_var9;
            piVar1 = (u_var5 + 10);
            *piVar1 = *piVar1 - u_var10;
            local_a = (u_var13 / 100);
            u_var13 = u_var13 % 100;
            u_var12 = u_var13;
            if (u_var13 != 0) {
                local_a = local_a + 1;
                u_var12 = local_a;
            }
            u_var8 = u_var12;
            process_struct_1000_179c(0x2a, u_var13);
            u_var11 = u_var13 | u_var8;
            if (u_var11 == 0) {}
            // goto LAB_1038_20fa;
            pass6_funcs::pass1_1038_6838(
                CONCAT22(u_var13, u_var8),
                u_var10,
                (u_var5 + 8),
                local_a,
                &struct_a.field_0x4,
            );
        } else {
            u_var9 = switch_statement_1020_c3b4((u_var5 + 6));
            u_var12 = *(u_var5 + 10) * u_var9;
            pu_var14 = (u_var12 >> 0x10);
            if (u_var12 + lVar4 * -100 != 0 && u_var13 <= u_var12) {
                u_var12 = u_var13 & 0xffff;
                pu_var14 = pu_var7;
            }
            u_var13 = u_var12 & 0xffff | ZEXT24(pu_var14) << 0x10;
            u_var10 = (u_var12 & 0xffff | ZEXT24(pu_var14) << 0x10) / u_var9;
            piVar1 = (u_var5 + 10);
            *piVar1 = *piVar1 - u_var10;
            local_a = (u_var13 / 100);
            u_var13 = u_var13 % 100;
            u_var12 = u_var13;
            if (u_var13 != 0) {
                local_a = local_a + 1;
                u_var12 = local_a;
            }
            u_var8 = u_var12;
            process_struct_1000_179c(0x2a, u_var13);
            u_var11 = u_var13 | u_var8;
            if (u_var11 == 0) {}
            // goto LAB_1038_20fa;
            pass6_funcs::pass1_1038_675c(
                CONCAT22(u_var13, u_var8),
                u_var10,
                (u_var5 + 6),
                local_a,
                &struct_a.field_0x4,
            );
        }
    } else {
        pu_var14 = *(u_var5 + 10);
        pu_var15 = 0x0;
        if ((pu_var7 < 1) && (0x7fff < pu_var7 || (u_var13 < pu_var14))) {
            pu_var14 = u_var13;
            pu_var15 = pu_var7;
        }
        _local_18 = CONCAT22(pu_var15, pu_var14);
        piVar1 = (u_var5 + 10);
        *piVar1 = *piVar1 - pu_var14;
        local_a = (_local_18 / 100);
        u_var12 = _local_18 % 100;
        u_var13 = u_var12;
        if (u_var12 != 0) {
            local_a = local_a + 1;
            u_var13 = local_a;
        }
        u_var8 = u_var13;
        process_struct_1000_179c(0x2a, u_var12);
        u_var11 = u_var12 | u_var8;
        if (u_var11 == 0) {
            // LAB_1038_20fa:
            u_var16 = 0x1000;
            _local_14 = 0;
            // goto LAB_1038_2102;
        }
        pass6_funcs::pass1_1038_6590(
            CONCAT22(u_var12, u_var8),
            pu_var14,
            pu_var15,
            *(u_var5 + 4),
            local_a,
            *&struct_a.field_0x4,
        );
    }
    u_var16 = 0x1000;
    _local_14 = CONCAT22(u_var11, u_var8);
    // LAB_1038_2102:
    if (_local_14 != 0) {
        pass1_1038_7a5a(_PTR_LOOP_1050_5a64);
        u_var16 = 0x1030;
        pass1_1030_6c4c(param_1_00, i_var2 - local_a);
    }
    if ((u_var5 + 10) == 0) {
        if ((u_var6 | u_var5) != 0) {
            ppc_var3 = *pu_var17;
            (**ppc_var3)(u_var16, u_var5, u_var6, 1);
        }
    } else {
        pass1_1030_6c66(param_1_00, 0, pu_var17);
    }
    return;
}

pub unsafe fn pass1_1038_2306(param_1: u16, param_2: u16, param_1_00: *mut Struct493) {
    let piVar1: *mut i32;
    let pu_var2: *mut u32;
    let ppc_var3: fn();
    let Var4: u16;
    let pu_var5: *mut u32;
    let mut u_var6: u32;
    let pu_var7: *mut u32;
    let mut u_var8: i32;
    let mut u_var9: i32;
    let mut u_var10: u32;
    let local_bx_19: *mut Struct1068;
    let mut i_var11: i32;
    let mut u_var12: u16;
    let mut u_var13: u16;
    let mut u_var14: u32;
    let mut local_30: u16;
    let mut local_2c: u16;
    let mut local_2a: u32;
    let mut local_26: u32;
    let mut local_22: u16;
    let mut local_20: u16;
    let mut local_1e: u16;
    let mut local_18: u32;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u32;
    let mut local_c: u32;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    u_var13 = 0x1030;
    u_var14 = pass1_1030_73a8(param_1_00);
    u_var10 = u_var14 >> 0x10;
    u_var12 = (param_1_00 >> 0x10);
    local_bx_19 = param_1_00;
    local_8 = local_bx_19.field_0x34;
    local_c = 100;
    pu_var2 = (u_var14 + 0x22);
    pu_var7 = pu_var2;
    loop {
        u_var8 = u_var10;
        ppc_var3 = (*pu_var2 + 0x10);
        (**ppc_var3)(u_var13, pu_var2, (pu_var2 >> 0x10));
        u_var9 = pu_var7;
        u_var14 = pu_var7 & 0xffff;
        pu_var5 = (u_var14 | u_var8 << 0x10);
        if ((u_var8 | u_var9) == 0) {
            break;
        }
        if ((u_var9 + 10) == 0) {
            u_var10 = (u_var8 | u_var9);
            if ((u_var8 | u_var9) != 0) {
                ppc_var3 = *pu_var5;
                (**ppc_var3)(u_var13, u_var9, u_var8, 1);
            }
        } else {
            local_18 = 0;
            local_1e = 0;
            if ((u_var9 + 6) == 0) {
                if ((u_var9 + 8) != 0) {
                    local_1e = pass1_1020_c42e((u_var9 + 8));
                    // goto LAB_1038_2385;
                }
            } else {
                local_1e = switch_statement_1020_c3b4((u_var9 + 6));
                // LAB_1038_2385:
                u_var13 = 0x1020;
                local_18 = ((u_var9 + 10) * local_1e);
            }
            local_c._2_2_ = 0;
            if (local_c < local_18) {
                local_18 = local_c & 0xffff;
            }
            _local_22 = local_18 | local_c._2_2_ << 0x10;
            u_var10 = local_18 | local_c._2_2_ << 0x10;
            qVar4 = u_var10 / local_1e;
            u_var6 = qVar4;
            u_var10 = u_var10 % local_1e;
            piVar1 = (u_var9 + 10);
            *piVar1 = *piVar1 - qVar4;
            if (*piVar1 == 0) {
                u_var10 = (u_var8 | u_var9);
                if ((u_var8 | u_var9) != 0) {
                    ppc_var3 = *pu_var5;
                    (**ppc_var3)(u_var13, u_var9, u_var8, 1);
                }
            } else {
                ppc_var3 = (*pu_var2 + 8);
                (**ppc_var3)();
            }
            local_c = local_c - _local_22;
            pu_var7 = 0x0;
            local_2a = 0;
            i_var11 = u_var14;
            if ((i_var11 + 6) == 0) {
                if ((i_var11 + 8) != 0) {
                    process_struct_1000_179c(0x2a, u_var10);
                    u_var9 = u_var10 | pu_var7;
                    u_var14 = u_var9;
                    if (u_var9 == 0) {}
                    // goto LAB_1038_244e;
                    pass6_funcs::pass1_1038_6838(
                        (pu_var7 & 0xffff | u_var10 << 0x10),
                        u_var6,
                        (i_var11 + 8),
                        1,
                        local_bx_19.field_0x4,
                    );
                    // goto LAB_1038_24b3;
                }
            } else {
                process_struct_1000_179c(0x2a, u_var10);
                u_var9 = u_var10 | pu_var7;
                u_var14 = u_var9;
                if (u_var9 == 0) {
                    // LAB_1038_244e:
                    u_var13 = 0x1000;
                    local_2a = 0;
                    u_var10 = u_var14;
                } else {
                    pass6_funcs::pass1_1038_675c(
                        (pu_var7 & 0xffff | u_var10 << 0x10),
                        u_var6,
                        (i_var11 + 6),
                        1,
                        local_bx_19.field_0x4,
                    );
                    // LAB_1038_24b3:
                    u_var13 = 0x1000;
                    local_2a = pu_var7 & 0xffff | u_var14 << 0x10;
                    u_var10 = u_var14;
                }
            }
            if (local_2a != 0) {
                pass1_1038_7a5a(_PTR_LOOP_1050_5a64);
                local_8 = local_8 - 1;
                if (local_8 == 0) {
                    break;
                }
                local_c = 100;
            }
        }
    }
    pass1_1030_6c4c(param_1_00, local_8);
    return;
}

pub unsafe fn pass1_1038_24e8(param_1: u16, param_2: u16, param_1_00: *mut Struct493) {
    let mut iVar1: i32;
    let mut u_var2: u32;

    let mut u_var3: u16;
    let struct_a: *mut Struct199;

    let mut u_var4: i32;
    let struct_a_00: *mut Struct199;

    let pa_var5: *mut Struct199;
    let local_bx_19: *mut Struct1069;
    let mut u_var6: u16;
    let mut unaff_ss: u16;
    let mut local_3e: u32;
    let mut local_3a: u16;
    let mut local_38: u16;
    let mut local_32: u32;
    let mut local_2e: u16;
    let mut local_2c: u16;
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
    let mut local_10: u32;
    let mut local_c: u32;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    _local_6 = pass1_1030_73a8(param_1_00);
    pa_var5 = (_local_6 >> 0x10);
    u_var6 = (param_1_00 >> 0x10);
    local_bx_19 = param_1_00;
    local_8 = local_bx_19.field_0x34;
    local_c = (_local_6 + 0x28);
    local_10 = 100;
    local_12 = (local_c + 4);
    u_var2 = local_12;
    process_struct_1000_179c(10, pa_var5);
    u_var4 = u_var2;
    if ((pa_var5 | u_var4) == 0) {
        u_var4 = 0;
        u_var3 = 0;
    } else {
        pass1_1020_ba3e((u_var2 & 0xffff | ZEXT24(pa_var5) << 0x10), 5, 5);
        u_var3 = ctx.dx_reg;
    }
    _local_1c = CONCAT22(u_var3, u_var4);
    local_1e = 0;
    while (u_var2 = local_12, local_1e < local_12) {
        pass1_1020_bb16(
            local_c,
            CONCAT13((unaff_ss >> 8), CONCAT12(unaff_ss, &local_18)),
            CONCAT22(unaff_ss, &local_14),
            local_1e,
        );
        if (local_18 != 0) {
            u_var2 = local_18;
            local_10._2_2_ = local_18._2_2_;
            if (local_10 < local_18) {
                u_var2 = local_10 & 0xffff;
            }
            u_var4 = u_var2;
            u_var2 = u_var2 & 0xffff | local_10._2_2_ << 0x10;
            iVar1 = (local_18._2_2_ - local_10._2_2_) - (local_18 < u_var4);
            local_18 = CONCAT22(iVar1, local_18 - u_var4);
            pass1_1020_bb8a(
                local_c,
                (local_c >> 0x10),
                local_18 - u_var4,
                iVar1,
                local_14,
            );
            pass1_1020_bb70(_local_1c, u_var4, CONCAT22(local_14, local_10._2_2_));
            local_10 = local_10 - u_var2;
            if (local_10 == 0) {
                pa_var5 = struct_a_00;
                process_struct_1000_179c(0x2a, struct_a_00);
                if ((pa_var5 | u_var2) == 0) {
                    u_var2 = 0;
                } else {
                    pass6_funcs::pass1_1038_666e(
                        (u_var2 & 0xffff | ZEXT24(pa_var5) << 0x10),
                        _local_1c,
                        1,
                        local_bx_19.field_0x4,
                    );
                }
                pass1_1038_7a5a(_PTR_LOOP_1050_5a64);
                pa_var5 = struct_a;
                process_struct_1000_179c(10, struct_a);
                if ((pa_var5 | u_var2) == 0) {
                    u_var2 = 0;
                    u_var4 = 0;
                } else {
                    pass1_1020_ba3e((u_var2 & 0xffff | ZEXT24(pa_var5) << 0x10), 5, 5);
                    u_var4 = ctx.dx_reg;
                }
                _local_1c = (u_var2 & 0xffff | u_var4 << 0x10);
                local_8 = local_8 - 1;
                if (local_8 == 0) {
                    break;
                }
                local_10 = 100;
            }
        }
        local_1e = local_1e + 1;
    }
    infinite_loop_1020_ba94(_local_1c, (_local_1c >> 0x10));
    pa_var5 = (ctx.dx_reg | u_var2);
    if (pa_var5 == 0x0) {
        if (_local_1c != 0x0) {
            pass1_1020_ba7e(_local_1c);
            error_check_1000_17ce(_local_1c);
        }
    } else {
        process_struct_1000_179c(0x2a, pa_var5);
        if ((pa_var5 | u_var2) != 0) {
            pass6_funcs::pass1_1038_666e(
                (u_var2 & 0xffff | ZEXT24(pa_var5) << 0x10),
                _local_1c,
                1,
                local_bx_19.field_0x4,
            );
        }
        pass1_1038_7a5a(_PTR_LOOP_1050_5a64);
    }
    pass1_1030_6c4c(param_1_00, local_8);
    return;
}

pub unsafe fn pass1_1038_26ee(param_1: u16, param_2: u16, param_1_00: *mut Struct493) {
    let mut u_var1: i32;
    let mut u_var2: i32;
    let mut u_var3: u32;

    let paVar4: *mut Struct199;
    let struct_a: *mut Struct199;
    let struct_a_00: *mut Struct199;
    let ctx.dx_reg: *mut Struct199;

    let pa_var5: *mut Struct199;
    let local_bx_19: *mut Struct1070;
    let mut u_var6: u16;
    let mut u_var7: u32;
    let mut local_32: u16;
    let mut local_2e: u16;
    let mut local_26: u16;
    let mut local_24: u16;
    let mut local_22: u16;
    let mut local_1e: u16;
    let mut local_1c: u16;
    let mut local_1a: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u32;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    u_var7 = pass1_1030_73a8(param_1_00);
    pa_var5 = (u_var7 >> 0x10);
    u_var6 = (param_1_00 >> 0x10);
    local_bx_19 = param_1_00;
    local_8 = local_bx_19.field_0x34;
    local_c = pass1_1028_0d80(u_var7);
    u_var3 = local_c;
    local_10 = 100;
    process_struct_1000_179c(10, pa_var5);
    if ((pa_var5 | u_var3) == 0) {
        u_var3 = 0;
        pa_var5 = 0x0;
    } else {
        pass1_1020_ba3e((u_var3 & 0xffff | ZEXT24(pa_var5) << 0x10), 5, 5);
        pa_var5 = ctx.dx_reg;
    }
    _local_14 = (u_var3 & 0xffff | ZEXT24(pa_var5) << 0x10);
    local_a = local_c;
    while {
        u_var1 = u_var3;
        pass1_1030_7c28(param_1_00, local_a);
        paVar4 = (pa_var5 | u_var1);
        if (paVar4 != 0x0) {
            paVar4 = pa_var5;
            u_var2 = u_var1;
            if ((local_10._2_2_ <= pa_var5) && (local_10._2_2_ < pa_var5 || (local_10 < u_var1))) {
                paVar4 = local_10._2_2_;
                u_var2 = local_10;
            }
            _local_24 = CONCAT22(paVar4, u_var2);
            pass1_1030_7d1c(
                param_1_00,
                u_var1 - u_var2,
                CONCAT22(local_a, pa_var5 + (-(u_var1 < u_var2) - paVar4)),
            );
            pass1_1020_bb70(_local_14, u_var2, CONCAT22(local_a, paVar4));
            local_10 = local_10 - _local_24;
            paVar4 = struct_a;
            if (local_10 == 0) {
                pa_var5 = struct_a;
                process_struct_1000_179c(0x2a, struct_a);
                local_a = _local_24;
                if ((pa_var5 | local_a) == 0) {
                    local_a = 0;
                } else {
                    pass6_funcs::pass1_1038_666e(
                        (_local_24 & 0xffff | ZEXT24(pa_var5) << 0x10),
                        _local_14,
                        1,
                        local_bx_19.field_0x4,
                    );
                }
                pass1_1038_7a5a(_PTR_LOOP_1050_5a64);
                pa_var5 = struct_a_00;
                process_struct_1000_179c(10, struct_a_00);
                if ((pa_var5 | local_a) == 0) {
                    local_a = 0;
                    paVar4 = 0x0;
                } else {
                    pass1_1020_ba3e(CONCAT22(pa_var5, local_a), 5, 5);
                    paVar4 = ctx.dx_reg;
                }
                _local_14 = CONCAT22(paVar4, local_a);
                local_8 = local_8 - 1;
                if (local_8 == 0) {
                    break;
                }
                local_10 = 100;
            }
        }
        local_a = pass1_1028_0d80(u_var7);
        u_var3 = local_a;
        if (local_c == 0) {
            local_c = local_a;
        }
        pa_var5 = paVar4;
        local_c != local_a
    } {}
    u_var1 = (_local_14 >> 0x10);
    infinite_loop_1020_ba94(_local_14, u_var1);
    pa_var5 = (ctx.dx_reg | local_a);
    if (pa_var5 == 0x0) {
        if ((u_var1 | _local_14) != 0) {
            pass1_1020_ba7e(_local_14);
            error_check_1000_17ce(_local_14);
        }
    } else {
        process_struct_1000_179c(0x2a, pa_var5);
        if ((pa_var5 | local_a) != 0) {
            pass6_funcs::pass1_1038_666e(
                CONCAT22(pa_var5, local_a),
                _local_14,
                1,
                local_bx_19.field_0x4,
            );
        }
        pass1_1038_7a5a(_PTR_LOOP_1050_5a64);
    }
    pass1_1030_6c4c(param_1_00, local_8);
    return;
}

pub unsafe fn pass1_1038_28d8(param_1: *mut Struct500) -> *mut Struct500 {
    pass1_1028_d1dc(param_1, (s_0_023_1050_3a93 + 4));
    param_1.a = (s_fem110_wav_1050_29fa + 4);
    (param_1 + 2) = &PTR_LOOP_1050_1038;
    copy_string_1000_3d3e(
        (param_1 & 0xffff0000 | (param_1 + 8)),
        s_SCRoboMove_1050_59f8,
    );
    return param_1;
}

pub unsafe fn pass1_1038_290e() {
    let paVar1: *mut Struct493;
    let mut in_dx: i32;
    let mut local_4: u16;

    paVar1 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, 1, 0x400);
    if ((in_dx | paVar1) != 0) {
        pass6_funcs::pass1_1038_4918(CONCAT22(in_dx, paVar1));
    }
    pass6_funcs::pass1_1038_7a76(_PTR_LOOP_1050_5a64);
    return 1;
}

pub unsafe fn pass1_1038_0c00(param_1: u32) {
    let pp_var1: fn();
    let mut u_var2: u32;
    let pu_var3: *mut u16;
    let mut u_var4: i32;
    let pu_var5: Vec<u8>;


    let mut u_var6: i32;


    let mut unaff_ss: u16;
    let mut local_30: u16;
    let mut local_2e: u32;
    let mut local_2a: u32;
    let mut local_20: u16;
    let mut local_1e: u16;
    let mut local_1c: u16;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u16;

    pass1_1028_dc52(
        CONCAT13((unaff_ss >> 8), CONCAT12(unaff_ss, &local_14)),
        (&ctx.PTR_LOOP_1050_0000 + 1),
        0,
        0x400,
    );
    loop {
        pu_var3 = &local_14;
        pass1_1028_e4ec(CONCAT22(unaff_ss, pu_var3));
        _local_18 = CONCAT22(ctx.dx_reg, pu_var3);
        if ((ctx.dx_reg | pu_var3) == 0) {
            break;
        }
        pass1_1038_0e78(param_1, CONCAT22(ctx.dx_reg, pu_var3));
        pass1_1038_1220(param_1, CONCAT22(ctx.dx_reg, pu_var3));
        u_var6 = ctx.dx_reg;
        pu_var5 = pass1_1008_c6fa(ctx._PTR_LOOP_1050_06e0, 1);
        u_var4 = pu_var5;
        pass6_funcs::pass1_1038_4d6e(
            CONCAT22(ctx.dx_reg, pu_var3),
            pu_var5 & 0xffff | u_var6 << 0x10,
        );
        _local_20 = CONCAT22(ctx.dx_reg, u_var4);
        pp_var1 = (*_local_20 + 0x10);
        u_var6 = u_var4;
        (**pp_var1)(&ctx.PTR_LOOP_1050_1008, u_var4, ctx.dx_reg);
        if ((ctx.dx_reg | u_var6) != 0) {
            u_var2 = (param_1 + 0x108);
            if ((u_var2 + 0x82) != 0) {
                pass1_1038_19a0(
                    param_1,
                    CONCAT22(ctx.dx_reg, u_var4),
                    CONCAT22(ctx.dx_reg, pu_var3),
                );
            }
            pass1_1038_1940(param_1, CONCAT22(ctx.dx_reg, u_var4), _local_18);
        }
        if (_local_20 != 0x0) {
            pp_var1 = *_local_20;
            (**pp_var1)(8, u_var4, ctx.dx_reg, 1);
        }
        pass1_1038_1c3e(param_1, _local_18);
    }
    return;
}

pub unsafe fn pass1_1038_0b6a(param_1: &mut Struct44, param_2: u8) -> &mut Struct44 {
    param_1.ptr_a_lo = ctx.s_1_1050_389a;
    (param_1 + 2) = &ctx.PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1038_008e(param_1: u16, param_2: u16, param_1_00: *mut Struct1050) {
    let mut iVar1: i32;
    let mut u_var2: u32;
    let mut u_var3: u16;
    let mut i_var4: i32;
    let local_AX_270: *mut Struct1052;
    let mut u_var5: i32;
    let local_bx_4: *mut Struct1050;
    let local_es_4: *mut Struct1050;
    let pp_var7: *mut Struct2111;
    let pp_var8: *mut Struct2111;
    let mut in_stack_0000ffd4: u16;
    let mut local_22: u32;
    let mut local_10: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;
    let temp_7ff08964032: *mut Struct1052;
    let mut temp_7ff94c183f6: i32;
    let mut u_var6: u32;

    local_es_4 = (param_1_00 >> 0x10);
    local_bx_4 = param_1_00;
    if (local_bx_4.field_0x4 != 0x4000001) {
        return;
    }
    pp_var7 = process_struct_1010_20ba(
        ctx._g_Struct372_1050_0ed0,
        CONCAT22(in_stack_0000ffd4, 0x2c),
    );
    temp_7ff94c183f6 = pp_var7;
    u_var3 = (pp_var7 >> 0x10);
    i_var4 = temp_7ff94c183f6;
    pass1_fn_1008_612e(1, 100);
    local_c = 0;
    iVar1 = (temp_7ff94c183f6 + 10);
    if (iVar1 == 1) {
        local_c = 0x15;
    } else {
        if (iVar1 != 2) {
            if (iVar1 == 3) {
                local_c = 0x16;
            } else {
                if (iVar1 == 4) {
                    if (i_var4 < 0x32) {
                        local_c = 0x17;
                    } else {
                        local_c = 0x18;
                    }
                } else {
                    if (iVar1 == 5) {
                        local_c = 0x19;
                    }
                }
            }
        }
    }
    if (local_c != 0) {
        pp_var8 = process_struct_1010_20ba(
            ctx._g_Struct372_1050_0ed0,
            CONCAT22(in_stack_0000ffd4, 0x2b),
        );
        pass1_1010_043a(pp_var8, local_bx_4.field_0x4, local_c);
    }
    pass1_1008_eb74(pp_var7, 0);
    if ((((temp_7ff94c183f6 + 0xe) | (temp_7ff94c183f6 + 0xc)) == 0)
        && (local_bx_4.field_0x18 < 0xc9))
    {
        u_var2 = *ctx._PTR_LOOP_1050_65e2;
        u_var6 = u_var2;
        pass1_fn_1008_612e(0, 8);
        u_var5 = u_var6;
        local_22._0_2_ = u_var2;
        local_22._2_2_ = (u_var2 >> 0x10);
        (temp_7ff94c183f6 + 0xc) = u_var5 + local_22 + 0x1e;
        (temp_7ff94c183f6 + 0xe) = (u_var5 >> 0xf)
            + local_22._2_2_
            + CARRY2(u_var5, local_22)
            + (0xffe1 < u_var5 + local_22);
    }
    return;
}

pub unsafe fn pass1_1038_01c0(param_1: u16, param_2: u16, param_1_00: u32) {
    let mut iVar1: i32;
    let pu_var2: *mut u32;
    let ppc_var3: fn();
    let mut u_var4: u32;
    let pa_var5: *mut Struct493;
    let mut u_var6: u16;
    let BVar7: bool;
    let pu_var8: Vec<u8>;
    let pu_var9: Vec<u8>;
    let pu_var10: *mut u32;
    let mut u_var11: u32;
    let mut in_dx: i32;





    let mut u_var12: u16;
    let mut u_var13: u16;
    let mut unaff_ss: u16;
    let mut u_var14: u32;
    let mut u_var15: u32;
    let u_var16: u8;
    let mut local_5c: u32;
    let mut local_58: u32;
    let mut local_4c: u16;
    let mut local_4a: u16;
    let mut local_48: u16;
    let mut local_46: u16;
    let mut local_44: u16;
    let mut local_3e: u16;
    let mut local_3c: u16;
    let mut local_3a: u16;
    let mut local_34: u16;
    let mut local_32: u32;
    let mut local_2e: u16;
    let mut local_28: u16;
    let mut local_26: u32;
    let mut local_1e: u32;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_16: u32;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: [u8; 2];
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    local_4 = 0;
    pu_var9 = pass1_1008_c6fa(ctx._PTR_LOOP_1050_06e0, 0x29);
    u_var6 = pu_var9;
    local_8 = u_var6;
    pass6_funcs::pass1_1038_4e78(param_1_00, pu_var9 & 0xffff | in_dx << 0x10);
    _local_c = CONCAT22(ctx.dx_reg, u_var6);
    u_var13 = 0x1030;
    u_var14 = pass1_1030_bcae(local_e, unaff_ss);
    u_var12 = u_var14;
    ppc_var3 = (*_local_c + 0x10);
    (**ppc_var3)(0x1030, _local_c, (_local_c >> 0x10));
    _local_12 = CONCAT22(ctx.dx_reg, u_var12);
    u_var12 = (param_1_00 >> 0x10);
    pu_var2 = (param_1_00 + 0xc);
    u_var12 = (param_1_00 + 0xe);
    u_var16 = SUB41(pu_var2, 0);
    ppc_var3 = (*pu_var2 + 0x10);
    pu_var10 = pu_var2;
    (**ppc_var3)();
    u_var4 = pu_var10 & 0xffff | ctx.dx_reg << 0x10;
    local_1e = 0;
    loop {
        if (_local_12 <= local_1e) {
            if (_local_c != 0x0) {
                ppc_var3 = *_local_c;
                (**ppc_var3)(u_var13, _local_c, (_local_c >> 0x10), 1, u_var16, u_var12);
            }
            return;
        }
        u_var13 = 0x1030;
        u_var11 = _local_12;
        pass1_1030_1d58(_local_c);
        iVar1 = (u_var11 + 0x10);
        local_32 = 0;
        while (local_32 < u_var4) {
            u_var13 = 0x1030;
            u_var15 = u_var4;
            pass1_1030_1d58(pu_var2);
            pa_var5 = (u_var15 & 0xffff | ctx.dx_reg << 0x10);
            if (((ctx.dx_reg | u_var15) != 0) && ((u_var15 + 0x10) == iVar1)) {
                u_var15 = pass1_1030_73a8(pa_var5);
                u_var13 = SUB42(&ctx.PTR_LOOP_1050_1008, 0);
                BVar7 = pass1_1008_c6ae(ctx._PTR_LOOP_1050_06e0, (u_var15 + 0xc), 0x30);
                if (BVar7 == 0) {
                    pu_var8 = local_e;
                    u_var13 = 0x1030;
                    pass1_1030_bd74(
                        pu_var8,
                        unaff_ss,
                        pa_var5,
                        (u_var11 & 0xffff | ctx.dx_reg << 0x10),
                    );
                    if (pu_var8 < 6) {
                        local_4 = local_4 + 1;
                        break;
                    }
                }
            }
            local_32 = local_32 + 1;
        }
        local_1e = local_1e + 1;
    }
}

pub unsafe fn pass1_1038_0340(param_1: u16, param_2: u16, param_1_00: i32, param_2_00: u32) {
    let mut u_var1: i32;
    let mut u_var2: u32;
    let mut in_dx: i32;
    let mut unaff_ss: u16;
    let mut i_var3: i32;
    let mut u_var4: u16;
    let mut local_13a: [u8; 284];
    let mut local_1e: u16;
    let mut local_1c: u16;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_16: u32;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u32;
    let mut local_6: u32;

    local_6 = *ctx._PTR_LOOP_1050_65e2;
    local_a = 0;
    local_c = 0;
    i_var3 = param_2_00;
    u_var4 = (param_2_00 >> 0x10);
    pass6_funcs::pass1_1038_4cea(
        param_2_00,
        CONCAT22(unaff_ss, &local_12),
        CONCAT22(unaff_ss, &local_e),
    );
    u_var2 = (i_var3 + 0x1f6);
    local_16 = u_var2;
    pass1_1030_38b8(u_var2, (u_var2 >> 0x10));
    u_var1 = u_var2;
    _local_1a = u_var2 & 0xffff | in_dx << 0x10;
    if (param_1_00 == 0) {
        if (local_e != 8) {
            local_a = (u_var2 & 0xffff | in_dx << 0x10) / 4;
            local_c = 8;
            // goto LAB_1038_054b;
        }
    } else {
        if (param_1_00 < 0xb) {
            if (local_e != 7) {
                local_a = (u_var2 & 0xffff | in_dx << 0x10) / 10;
                local_c = 7;
                // goto LAB_1038_054b;
            }
        } else {
            if (param_1_00 < 0x1a) {
                if (local_e != 6) {
                    local_a = (u_var2 & 0xffff | in_dx << 0x10) / 0x14;
                    local_c = 6;
                    // goto LAB_1038_054b;
                }
            } else {
                if (param_1_00 < 0x33) {
                    if (local_e != 5) {
                        local_a = (u_var2 & 0xffff | in_dx << 0x10) / 100;
                        local_c = 5;
                        // goto LAB_1038_054b;
                    }
                } else {
                    if (param_1_00 < 0x4c) {
                        if (local_6 % 3 != 0) {}
                        // goto LAB_1038_054b;
                        if (local_e != 4) {
                            local_a = _local_1a / 100;
                            local_c = 4;
                            // goto LAB_1038_054b;
                        }
                    } else {
                        if (param_1_00 < 0x65) {
                            if (local_6 % 5 != 0) {}
                            // goto LAB_1038_054b;
                            if (local_e != 3) {
                                local_a = _local_1a / 100;
                                local_c = 3;
                                // goto LAB_1038_054b;
                            }
                        } else {
                            if (param_1_00 < 0x97) {
                                if (local_6 % 10 != 0) {}
                                // goto LAB_1038_054b;
                                if (local_e != 2) {
                                    local_a = _local_1a / 100;
                                    local_c = 2;
                                    // goto LAB_1038_054b;
                                }
                            } else {
                                if ((200 < param_1_00) || (local_6 % 0x14 != 0)) {}
                                // goto LAB_1038_054b;
                                if (local_e != 1) {
                                    local_a = _local_1a / 100;
                                    local_c = 1;
                                    // goto LAB_1038_054b;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    if ((local_10 <= in_dx) && (local_10 < in_dx || (local_12 < u_var1))) {
        u_var1 = local_12;
        in_dx = local_10;
    }
    local_a = CONCAT22(in_dx, u_var1);
    // LAB_1038_054b:
    if (local_c != 0) {
        if ((_local_1a != 0) && (local_a == 0)) {
            local_a = 1;
        }
        pass6_funcs::pass1_1038_4cd0(param_2_00, local_a, local_c);
    }
    if ((local_a._2_2_ | local_a) != 0) {
        if ((i_var3 + 0x200) == 0x8000001) {
            local_1e = 2;
        } else {
            local_1e = 1;
        }
        _local_1e = CONCAT22(0x400, local_1e);
        pass1_1028_9944(
            CONCAT22(unaff_ss, local_13a),
            local_a,
            CONCAT22(0x400, local_1e),
            (i_var3 + 4),
        );
        pass1_1030_835a(ctx._g_bool_1050_5748, CONCAT22(unaff_ss, local_13a));
        pass1_1028_9992(CONCAT22(unaff_ss, local_13a));
    }
    return;
}

pub unsafe fn pass1_1038_05d8(param_1: u16, param_2: u16, param_1_00: i32, param_2_00: u32) {
    let pu_var1: *mut u16;
    let mut u_var2: u32;
    let mut u_var3: i32;
    let mut u_var4: i32;
    let mut in_edx: u32;
    let mut u_var5: u16;
    let mut unaff_ss: u16;
    let mut local_158: [u8; 280];
    let mut local_40: u32;
    let mut local_34: u16;
    let mut local_32: u16;
    let mut local_22: u16;
    let mut local_20: u16;
    let mut local_1e: u32;
    let mut local_1a: u32;
    let mut local_16: u32;
    let mut local_12: u32;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u32;
    let mut local_6: u32;

    local_6 = *ctx._PTR_LOOP_1050_65e2;
    local_a = 0;
    local_c = 0;
    pass6_funcs::pass1_1038_4cea(
        param_2_00,
        CONCAT22(unaff_ss, &local_12),
        CONCAT22(unaff_ss, &local_e),
    );
    local_16 = 0;
    local_1a = 0;
    local_1e = 0;
    pass1_1028_dc52(
        CONCAT13((unaff_ss >> 8), CONCAT12(unaff_ss, &local_34)),
        (&ctx.PTR_LOOP_1050_0000 + 1),
        0,
        0x400,
    );
    while {
        while {
            u_var3 = in_edx;
            pu_var1 = &local_34;
            pass1_1028_e4ec(CONCAT22(unaff_ss, pu_var1));
            _local_22 = CONCAT22(u_var3, pu_var1);
            u_var4 = u_var3 | pu_var1;
            in_edx = u_var4;
            if (u_var4 == 0) {}
            // goto LAB_1038_0668;
            (pu_var1 + 0x100) != 0x8000002
        } {}
        local_16 = CONCAT22(u_var3, pu_var1);
        u_var2 = (pu_var1 + 0xfb);
        local_1a = u_var2;
        pass1_1030_38b8(u_var2, (u_var2 >> 0x10));
        local_1e = u_var2 & 0xffff | u_var4 << 0x10;
        u_var4 = u_var4 | u_var2;
        in_edx = u_var4;
        u_var4 == 0
    } {}
    // LAB_1038_0668:
    local_34 = ctx.s_1_1050_389a;
    local_32 = &ctx.PTR_LOOP_1050_1008;
    if ((local_16._2_2_ | local_16) == 0) {
        return;
    }
    if (param_1_00 == 1000) {
        if (local_e != 0x10) {
            local_a = local_1e / 4;
            local_c = 0x10;
            // goto LAB_1038_0841;
        }
    } else {
        if (param_1_00 < 0x3de) {
            if (param_1_00 < 0x3cf) {
                if (param_1_00 < 0x3b6) {
                    if (param_1_00 < 0x39d) {
                        if (param_1_00 < 900) {
                            if (param_1_00 < 0x352) {
                                if ((param_1_00 < 800) || (local_6 % 0x14 != 0)) {}
                                // goto LAB_1038_0841;
                                if (local_e != 9) {
                                    local_a = local_1e / 100;
                                    local_c = 9;
                                    // goto LAB_1038_0841;
                                }
                            } else {
                                if (local_6 % 10 != 0) {}
                                // goto LAB_1038_0841;
                                if (local_e != 10) {
                                    local_a = local_1e / 100;
                                    local_c = 10;
                                    // goto LAB_1038_0841;
                                }
                            }
                        } else {
                            if (local_6 % 5 != 0) {}
                            // goto LAB_1038_0841;
                            if (local_e != 0xb) {
                                local_a = local_1e / 100;
                                local_c = 0xb;
                                // goto LAB_1038_0841;
                            }
                        }
                    } else {
                        if (local_6 % 3 != 0) {}
                        // goto LAB_1038_0841;
                        if (local_e != 0xc) {
                            local_a = local_1e / 100;
                            local_c = 0xc;
                            // goto LAB_1038_0841;
                        }
                    }
                } else {
                    if (local_e != 0xd) {
                        local_a = local_1e / 100;
                        local_c = 0xd;
                        // goto LAB_1038_0841;
                    }
                }
            } else {
                if (local_e != 0xe) {
                    local_a = local_1e / 0x14;
                    local_c = 0xe;
                    // goto LAB_1038_0841;
                }
            }
        } else {
            if (local_e != 0xf) {
                local_a = local_1e / 10;
                local_c = 0xf;
                // goto LAB_1038_0841;
            }
        }
    }
    u_var2 = local_1e;
    if (local_12 < local_1e) {
        u_var2 = local_12 & 0xffff;
        local_1e._2_2_ = local_12._2_2_;
    }
    local_a = u_var2 & 0xffff | local_1e._2_2_ << 0x10;
    // LAB_1038_0841:
    if (local_c != 0) {
        if ((local_1e != 0) && (local_a == 0)) {
            local_a = 1;
        }
        pass6_funcs::pass1_1038_4cd0(param_2_00, local_a, local_c);
    }
    if ((local_a._2_2_ | local_a) != 0) {
        u_var5 = (param_2_00 >> 0x10);
        if ((param_2_00 + 0x200) == 0x8000001) {
            local_40 = (local_16 + 4);
        } else {
            local_40 = 0x4000001;
        }
        pass1_1028_9944(
            CONCAT22(unaff_ss, local_158),
            local_a,
            (param_2_00 + 4),
            local_40,
        );
        pass1_1030_835a(ctx._g_bool_1050_5748, CONCAT22(unaff_ss, local_158));
        pass1_1028_9992(CONCAT22(unaff_ss, local_158));
    }
    return;
}

pub unsafe fn pass1_1038_08d4(param_1: u16, param_2: u32, param_3: u32) {
    let mut unaff_ss: u16;
    let lVar1: u32;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_4: u16;

    local_4 = 0;
    pass1_1028_dc52(
        CONCAT22(unaff_ss, &local_16),
        (&ctx.PTR_LOOP_1050_0000 + 1),
        0,
        0x400,
    );
    while {
        lVar1 = pass1_1028_e4ec(CONCAT22(unaff_ss, &local_16));
        if (lVar1 == 0) {}
        // goto LAB_1038_0917;
        (lVar1 + 0x200) != 0x8000002
    } {}
    local_4 = 1;
    // LAB_1038_0917:
    local_16 = ctx.s_1_1050_389a;
    local_14 = &ctx.PTR_LOOP_1050_1008;
    if (local_4 != 0) {
        if (param_2 < 0xc90000) {
            pass1_1038_0340(param_1, param_2, param_2._2_2_, param_3);
            return;
        }
        if (0x31fffff < param_2) {
            pass1_1038_05d8(param_1, param_2, param_2._2_2_, param_3);
        }
    }
    return;
}

pub unsafe fn pass1_1030_ecc2(param_1: *mut Struct500) -> *mut Struct500 {
    pass1_1028_d1dc(param_1, 0xf9f);
    param_1.a = 0xb96;
    (param_1 + 2) = &PTR_LOOP_1050_1038;
    copy_string_1000_3d3e((param_1 & 0xffff0000 | (param_1 + 8)), s_SCMorale_1050_59ce);
    return param_1;
}

pub unsafe fn pass1_1030_ecf8(param_1: u32) {
    let mut u_var1: u16;
    let mut i_var2: i32;
    let pu_var3: *mut u32;
    let ppcVar4: fn();
    let mut u_var5: u16;
    let mut u_var6: u32;
    let local_AX_122: *mut Struct1045;
    let pa_var7: *mut Struct493;
    let mut u_var8: u16;
    let mut i_var9: i32;
    let mut u_var10: u32;





    let mut u_var11: i32;
    let mut u_var12: i32;
    let mut u_var13: u16;
    let mut unaff_ss: u16;
    let mut b_var14: bool;
    let ppVar15: *mut Struct2111;
    let pu_var16: *mut u32;
    let mut u_var17: u16;
    let mut local_68: u16;
    let mut local_66: u16;
    let mut local_60: u16;
    let mut local_5e: u16;
    let mut local_5c: u16;
    let mut local_5a: u16;
    let mut local_58: u16;
    let mut local_56: u16;
    let mut local_44: u16;
    let mut local_42: u16;
    let mut local_40: u32;
    let mut local_3c: u16;
    let mut local_3a: u16;
    let mut local_38: u16;
    let mut local_36: u16;
    let mut local_34: u16;
    let mut local_32: u16;
    let mut local_30: u32;
    let mut local_2c: u32;
    let mut local_26: u16;
    let mut local_24: u16;
    let mut local_22: u16;
    let mut local_20: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    local_c = 0;
    ppVar15 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(local_68, 0x2f));
    local_4 = (ppVar15 >> 0x10);
    local_a = ppVar15;
    local_6 = local_a;
    pass1_1010_ed3e(ppVar15);
    local_8 = ctx.dx_reg;
    if ((ctx.dx_reg | local_a) != 0) {
        local_c = pass1_1030_2aaa(CONCAT22(ctx.dx_reg, local_a));
    }
    if (local_c < 2) {
        local_c = 0;
    }
    ppVar15 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(local_68, 2));
    local_e = (ppVar15 >> 0x10);
    local_10 = ppVar15;
    if ((0 < u16_1050_13ae) && (!SBORROW2(u16_1050_13ae, 1))) {
        if (u16_1050_13ae == 2 || (u16_1050_13ae - 1) < 1) {
            if (6 < local_c) {
                local_c = local_c - 2;
                // goto LAB_1030_ed5b;
            }
            b_var14 = SBORROW2(local_c, 4);
            i_var2 = local_c - 4;
        } else {
            if (u16_1050_13ae != 3) {}
            // goto LAB_1030_ed5b;
            b_var14 = SBORROW2(local_c, 7);
            i_var2 = local_c - 7;
        }
        if (b_var14 == (i_var2 < 0)) {
            local_c = local_c - 1;
        }
    }
    // LAB_1030_ed5b:
    pass1_1028_dc52(
        CONCAT13((unaff_ss >> 8), CONCAT12(unaff_ss, &local_22)),
        (&ctx.PTR_LOOP_1050_0000 + 1),
        0,
        0x400,
    );
    loop {
        local_AX_122 = &local_22;
        pass1_1028_e4ec(CONCAT22(unaff_ss, local_AX_122));
        _local_26 = CONCAT22(ctx.dx_reg, local_AX_122);
        if ((ctx.dx_reg | local_AX_122) == 0) {
            break;
        }
        u_var11 = *&local_AX_122.field_0x1f6;
        if (((local_AX_122.field_0x1fe != 0) && (local_AX_122.field_0x200 != 0x8000002))
            && (
                pass1_1030_38b8(u_var11, &local_AX_122.field_0x1f8),
                (ctx.dx_reg | u_var11) != 0,
            ))
        {
            pu_var3 = &local_AX_122.field_0xc;
            ppcVar4 = (*pu_var3 + 0x10);
            pu_var16 = pu_var3;
            (**ppcVar4)(&PTR_LOOP_1050_1028, pu_var3, &local_AX_122.field_0xe);
            u_var6 = pu_var16 & 0xffff | ctx.dx_reg << 0x10;
            local_36 = local_AX_122.field_0x18;
            u_var13 = SUB42(&PTR_LOOP_1050_1038, 0);
            pass6_funcs::pass1_1038_4760(local_AX_122, ctx.dx_reg);
            local_38 = local_AX_122.field_0x22 / 10;
            u_var1 = local_AX_122.field_0x24;
            if (u_var1 < 0x33) {
                if (u_var1 < 0x32) {
                    local_38 = local_38 - 1;
                }
            } else {
                local_36 = local_36 + 1;
            }
            local_40 = 0;
            while (local_40 < u_var6) {
                ppcVar4 = (*pu_var3 + 4);
                u_var10 = u_var6;
                (**ppcVar4)(
                    u_var13,
                    pu_var3,
                    (pu_var3 >> 0x10),
                    local_40,
                    (local_40 >> 0x10),
                );
                u_var11 = ctx.dx_reg | u_var10;
                if (u_var11 != 0) {
                    u_var13 = SUB42(&PTR_LOOP_1050_1028, 0);
                    pa_var7 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, u_var10, ctx.dx_reg);
                    pu_var16 = pass1_1030_73a8(CONCAT22(u_var11, pa_var7));
                    u_var12 = (pu_var16 >> 0x10);
                    u_var11 = pu_var16;
                    if (((u_var12 | u_var11) != 0) && ((u_var11 + 0x12) == 5)) {
                        ppcVar4 = (*pu_var16 + 0x48);
                        (**ppcVar4)(&PTR_LOOP_1050_1028, u_var11, u_var12);
                        if (u_var11 < 0) {
                            local_38 = local_38 + u_var11;
                        } else {
                            local_36 = local_36 + u_var11;
                        }
                    }
                }
                local_40 = local_40 + 1;
            }
            i_var2 = local_38 - local_c;
            u_var1 = local_AX_122.field_0x20a;
            u_var17 = (param_1 >> 0x10);
            u_var5 = param_1;
            u_var8 = u_var1;
            pass1_1038_01c0(u_var5, u_var17, _local_26);
            i_var9 = u_var8 - u_var1;
            local_38 = i_var2 - i_var9;
            pass1_1038_008e(u_var5, u_var17, _local_26);
            if (i_var9 < 0) {
                local_38 = local_38 + i_var9;
            } else {
                local_36 = local_36 + i_var9;
            }
            if (1000 < local_36) {
                local_36 = 1000;
            }
            if (local_36 < 0) {
                local_36 = 0;
            }
            local_36 = local_36 + local_38;
            if (1000 < local_36) {
                local_36 = 1000;
            }
            if (local_36 < 0) {
                local_36 = 0;
            }
            pass6_funcs::pass1_1038_4d0e(_local_26, local_36);
            if (local_AX_122.field_0x4 == 0x4000001) {
                pass1_1038_08d4(u_var5, CONCAT22(local_36, u_var17), _local_26);
            }
            pass1_1038_095e(u_var5, u_var17, local_36, _local_26);
        }
    }
    return;
}

pub unsafe fn pass1_1030_ec86(param_1: &mut Struct44, param_2: u8) -> &mut Struct44 {
    param_1.ptr_a_lo = ctx.s_1_1050_389a;
    (param_1 + 2) = &ctx.PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1030_eb86() {
    let mut iVar1: i32;
    let ppc_var2: fn();
    let mut u_var3: u16;
    let mut unaff_ss: u16;
    let pu_var4: *mut u32;
    let mut local_22: u16;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u16;

    pass1_1028_dc52(
        CONCAT22(unaff_ss, &local_14),
        (&ctx.PTR_LOOP_1050_0000 + 1),
        0,
        0x700,
    );
    loop {
        pu_var4 = pass1_1028_e4ec(CONCAT22(unaff_ss, &local_14));
        u_var3 = (pu_var4 >> 0x10);
        if (pu_var4 == 0x0) {
            break;
        }
        if ((pu_var4 + 0x12) == 5) {
            iVar1 = (pu_var4 + 0xc);
            if (((0x32 < iVar1) && (!SBORROW2(iVar1, 0x33)))
                && (iVar1 == 0x34
                    || iVar1 + -0x33 < 1
                    || (0x2b < iVar1 + -0x34 && (iVar1 + -0x60 < 2))))
            {
                ppc_var2 = (*pu_var4 + 0x2c);
                ppc_var2(&PTR_LOOP_1050_1028);
            }
        }
    }
    return 1;
}

pub unsafe fn pass1_1030_ea50(param_1: u32, param_2: *mut Struct493) {
    let mut u_var1: u32;
    let BVar2: bool;
    let mut i_var3: i32;
    let mut u_var4: u16;
    let mut unaff_ss: u16;
    let mut u_var5: u32;
    let mut local_12: u32;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;

    local_6 = 99999;
    u_var4 = (param_1 >> 0x10);
    i_var3 = param_1;
    BVar2 = pass1_1008_c6ae(ctx._PTR_LOOP_1050_06e0, (i_var3 + 0x110), 3);
    if (BVar2 != 0) {
        u_var5 = pass1_1030_73a8(param_2);
        local_c = (u_var5 >> 0x10);
        local_e = u_var5;
        local_6 = pass1_1028_45e2(u_var5);
    }
    u_var1 = (i_var3 + 0x108);
    local_8 = (u_var1 + 4);
    local_a = 0;
    loop {
        if (local_8 <= local_a) {
            return;
        }
        pass1_1020_bb16(
            (i_var3 + 0x108),
            CONCAT22(unaff_ss, &local_12),
            CONCAT22(unaff_ss, &local_e),
            local_a,
        );
        if (local_6 < local_12) {
            pass1_1030_7ddc(param_2, local_6, local_e);
            local_6 = 0;
        } else {
            local_6 = local_6 - local_12;
            pass1_1030_7ddc(param_2, local_12, local_e);
        }
        if ((local_6._2_2_ | local_6) == 0) {
            break;
        }
        local_a = local_a + 1;
    }
    return;
}

pub unsafe fn pass1_1030_eb14(param_1: &mut Struct44, param_2: u8) -> &mut Struct44 {
    param_1.ptr_a_lo = ctx.s_1_1050_389a;
    (param_1 + 2) = &ctx.PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1030_e8f8(param_1: *mut Struct1036) {
    let mut u_var1: i32;
    let mut u_var2: i32;
    let mut u_var3: u32;
    let paVar4: *mut Struct493;
    let mut in_dx: u16;
    let local_bx_4: *mut Struct1036;
    let local_es_4: *mut Struct1036;
    let mut u_var5: u32;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    local_es_4 = (param_1 >> 0x10);
    local_bx_4 = param_1;
    if (&local_bx_4.field_0x108 != 0) {
        u_var3 = local_bx_4.field_0x10c;
        paVar4 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, u_var3, (u_var3 >> 0x10));
        _local_6 = CONCAT22(in_dx, paVar4);
        u_var5 = pass1_1030_73a8(CONCAT22(in_dx, paVar4));
        if ((u_var5 + 0xc) == local_bx_4.field_0x110) {
            pass1_1030_ea50(param_1, _local_6);
        }
        u_var1 = local_bx_4.field_0x108;
        u_var2 = &local_bx_4.field_0x10a;
        _local_14 = CONCAT22(u_var2, u_var1);
        if ((u_var2 | u_var1) != 0) {
            pass1_1020_ba7e(CONCAT22(u_var2, u_var1));
            error_check_1000_17ce(_local_14);
        }
        &local_bx_4.field_0x108 = 0;
    }
    return 1;
}

pub unsafe fn pass1_1030_e864(param_1: &mut Struct44, param_2: u8) -> &mut Struct44 {
    param_1.ptr_a_lo = ctx.s_1_1050_389a;
    (param_1 + 2) = &ctx.PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1030_e75e(param_1: &mut Struct44, param_2: u8) -> &mut Struct44 {
    param_1.ptr_a_lo = ctx.s_1_1050_389a;
    (param_1 + 2) = &ctx.PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1030_e602(param_1: &mut Struct44, param_2: u8) -> &mut Struct44 {
    param_1.ptr_a_lo = ctx.s_1_1050_389a;
    (param_1 + 2) = &ctx.PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1030_e546(param_1: u32) {
    let mut u_var1: u32;

    u_var1 = (param_1 + 0x108);
    pass1_1028_e332(ctx._PTR_LOOP_1050_65e2, u_var1, (u_var1 >> 0x10));
    return 1;
}

pub unsafe fn pass1_1030_e410(param_1: *mut Struct1021) {
    let mut u_var1: u32;
    let paVar2: *mut Struct493;
    let mut in_dx: i32;
    let mut unaff_ss: u16;
    let mut local_10: [u8; 6];
    let mut local_a: [u8; 4];
    let mut local_6: u16;
    let mut local_4: u16;

    u_var1 = (param_1 + 0x10c);
    paVar2 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, u_var1, (u_var1 >> 0x10));
    if ((in_dx | paVar2) != 0) {
        local_6 = paVar2;
        paVar2 = pass6_funcs::pass1_1038_4fd8(CONCAT22(in_dx, paVar2), 0x21);
        if (paVar2 == 0x0) {
            pass1_1020_a43e(CONCAT22(unaff_ss, local_a));
            pass1_1008_3e54(CONCAT22(unaff_ss, local_10), 0, 2, 0xfffd);
            pass1_1020_a49a(
                CONCAT22(unaff_ss, local_a),
                CONCAT22(unaff_ss, local_10),
                0x7a,
            );
            pass1_1008_3e76(CONCAT22(unaff_ss, local_10), 0, 3, 0xfffe);
            pass1_1020_a49a(
                CONCAT22(unaff_ss, local_a),
                CONCAT22(unaff_ss, local_10),
                0x7a,
            );
            pass1_1008_3e76(CONCAT22(unaff_ss, local_10), 0, 3, 0xfffd);
            paVar2 = pass1_1020_a49a(
                CONCAT22(unaff_ss, local_a),
                CONCAT22(unaff_ss, local_10),
                0x21,
            );
        }
    }
    return paVar2;
}

pub unsafe fn pass1_1030_e2be(param_1: *mut Struct500, param_2: u16, param_3: u32, param_4: u32) {
    let local_bx_19: *mut Struct500;
    let mut u_var1: u16;

    pass1_1028_d1dc(param_1, s_fem133_wav_1050_2af7);
    u_var1 = (param_1 >> 0x10);
    local_bx_19 = param_1;
    local_bx_19.field_0x108 = param_4;
    local_bx_19.field_0x10c = param_3;
    &local_bx_19.field_0x110 = param_2;
    param_1.a = 0xe4ea;
    local_bx_19.b = 0x1030;
    return;
}

pub unsafe fn pass1_1030_e300(param_1: u32) {
    let ppVar1: *mut Struct2111;
    let mut in_stack_0000fff8: u32;

    ppVar1 = process_struct_1010_20ba(
        ctx._g_Struct372_1050_0ed0,
        CONCAT22((in_stack_0000fff8 >> 0x10), 0x2b),
    );
    pass1_1010_089e(ppVar1, (param_1 + 0x110), 2);
    return 1;
}

pub unsafe fn pass1_1030_e328(param_1: *mut Struct1021) {
    let local_bx_3: *mut Struct1021;
    let local_es_3: *mut Struct1021;

    local_es_3 = (param_1 >> 0x10);
    local_bx_3 = param_1;
    if (local_bx_3.field_0x110 == 0) {
        ret_1030_e4ba(param_1);
    } else {
        pass1_1030_e410((param_1 & 0xffff | ZEXT24(local_es_3) << 0x10));
    }
    return 1;
}

pub unsafe fn pass1_1030_e0d4() {
    let piVar1: *mut i32;
    let mut u_var2: u32;
    let paVar3: *mut Struct493;
    let local_AX_104: *mut Struct1017;
    let paVar4: *mut Struct493;
    let mut u_var5: i32;

    let mut i32_var6: i32;
    let local_bx_211: *mut Struct1018;
    let mut i_var7: i32;
    let mut unaff_si: u16;
    let mut u_var8: u16;
    let mut unaff_ss: u16;
    let pp_var9: *mut Struct2111;
    let mut local_2a: u32;
    let mut local_26: u16;
    let mut local_24: u16;
    let mut local_22: u16;
    let mut local_1c: [u8; 8];
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    pp_var9 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(unaff_si, 0x40));
    local_4 = (pp_var9 >> 0x10);
    local_6 = pp_var9;
    local_a = pass1_1008_b820(pp_var9);
    u_var5 = (local_a >> 0x10) | local_a;
    if (u_var5 != 0) {
        paVar3 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, 1, 0x800);
        local_e = CONCAT22(u_var5, paVar3);
        local_10 = (&paVar3[0xb].field_0xa != 0);
        pass1_1008_5784(CONCAT22(unaff_ss, local_1c), local_a);
        loop {
            local_AX_104 = local_1c;
            pass1_1008_5b12(CONCAT22(unaff_ss, local_AX_104));
            _local_14 = CONCAT22(ctx.dx_reg, local_AX_104);
            u_var5 = ctx.dx_reg | local_AX_104;
            if (u_var5 == 0) {
                break;
            }
            if (local_AX_104.field_0x8 != 0) {
                u_var2 = local_AX_104.field_0xa;
                paVar4 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, u_var2, (u_var2 >> 0x10));
                paVar3 = paVar4;
                pass6_funcs::pass1_1038_354a(CONCAT22(u_var5, paVar4));
                if (paVar3 != 0x0) {
                    u_var8 = (_local_14 >> 0x10);
                    if (local_10 == 0) {
                        local_bx_211 = ((local_14 + 0xe) * 0xc);
                        local_2a = (local_bx_211 + 0x58c4);
                        i32_var6 = (local_bx_211 + 0x58c8);
                    } else {
                        i32_var6 = (local_14 + 0xe) * 0xc;
                        local_2a = (i32_var6 + 0x58be);
                        i32_var6 = (i32_var6 + 0x58c2);
                    }
                    i_var7 = i32_var6;
                    pass6_funcs::pass1_1038_35a8(CONCAT22(u_var5, paVar4), ((local_14 + 0x10) * 2 + local_2a));
                    if (i_var7 != 0) {
                        u_var8 = (_local_14 >> 0x10);
                        i_var7 = _local_14;
                        piVar1 = (i_var7 + 0x10);
                        *piVar1 = *piVar1 + 1;
                        if (i32_var6 <= (i_var7 + 0x10)) {
                            (i_var7 + 0x10) = 0;
                        }
                    }
                }
            }
        }
    }
    return;
}

pub unsafe fn pass1_1030_e010(param_1: &mut Struct44, param_2: u8) -> &mut Struct44 {
    pass1_1030_dcf4(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1030_df0c(param_1: &mut Struct44) {
    let mut u_var1: u32;
    let mut u_var2: u32;
    let lVar3: u32;
    let u_var4: u8;
    let mut u_var5: u16;
    let paVar6: *mut Struct990;
    let extraout_var: u32;
    let mut u_var7: u32;
    let mut in_dx: i32;

    let mut u_var8: u16;
    let mut u_var9: u16;
    let mut local_20: u16;
    let mut local_1e: u16;
    let mut local_18: u32;
    let mut local_14: u16;
    let mut local_12: u32;
    let mut local_e: u32;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;
    let temp_7ff07747202: *mut Struct1015;

    u_var4 = pass1_1028_b58e(param_1);
    temp_7ff07747202 = CONCAT31(extraout_var, u_var4);
    u_var1 = &temp_7ff07747202.field_0x2e;
    local_a._0_2_ = u_var1;
    if ((temp_7ff07747202.field_0x30 | local_a) != 0) {
        u_var8 = (u_var1 >> 0x10);
        u_var1 = (local_a + 0x210);
        local_e._0_2_ = u_var1;
        if (((local_a + 0x212) | local_e) != 0) {
            u_var8 = (u_var1 >> 0x10);
            u_var2 = (local_e + 10);
            u_var5 = pass1_1030_dfcc(param_1);
            if (u_var5 != 0) {
                local_18._0_2_ = 1;
                local_18._2_2_ = 0;
                while (CONCAT22(local_18._2_2_, local_18) < u_var2) {
                    u_var7 = u_var2;
                    u_var9 = u_var5;
                    pass1_1030_1312(local_e, u_var8, local_18);
                    paVar6 = pass1_1030_cde8(u_var7, ctx.dx_reg, u_var9);
                    if (-1 < paVar6) {
                        pass1_1030_cef8(
                            (u_var7 & 0xffff | ctx.dx_reg << 0x10),
                            CONCAT31(extraout_var, u_var4) & 0xffff | in_dx << 0x10,
                            1,
                            paVar6,
                        );
                        (param_1 + 0x20) = (u_var7 + 4);
                        return;
                    }
                    lVar3 = CONCAT22(local_18._2_2_, local_18) + 1;
                    local_18._0_2_ = lVar3;
                    local_18._2_2_ = (lVar3 >> 0x10);
                }
            }
        }
    }
    return;
}

pub unsafe fn pass1_1030_dc08(param_1: &mut Struct44, param_2: u8) -> &mut Struct44 {
    pass1_1028_b418(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1030_dc96(param_1: *mut Struct763) -> *mut Struct763 {
    let mut u_var1: u16;

    pass1_1028_b354(param_1);
    u_var1 = (param_1 >> 0x10);
    (param_1 + 0x20) = 0;
    param_1.field_0x0 = 0xe036;
    (param_1 + 2) = 0x1030;
    return param_1;
}

pub unsafe fn pass1_1030_dcc2(
    param_1: *mut Struct1012,
    param_2: u16,
    param_3: u16,
    param_3_00: u32,
) -> i32 {
    pass1_1028_b39e(CONCAT22(param_2, param_1), param_3, param_3_00);
    param_1.field_0x20 = 0;
    CONCAT22(param_2, param_1) = 0xe036;
    param_1.field_0x2 = 0x1030;
    return CONCAT22(param_2, param_1);
}

pub unsafe fn pass1_1030_dcf4(param_1: &mut Struct44) {
    let mut u_var1: u32;
    let ppc_var2: fn();
    let u_var3: u8;
    let paVar4: *mut Struct493;
    let mut u_var5: u16;
    let mut u_var6: u16;
    let extraout_var: u32;
    let pu_var7: Vec<u8>;
    let mut u_var8: u32;
    let mut in_dx: i32;
    let mut u_var9: i32;



    let mut iVar10: i32;
    let mut u_var11: u16;
    let mut local_24: u16;
    let mut local_22: u16;
    let mut local_1c: u32;
    let mut local_18: u16;
    let mut local_16: u16;
    let local_14: *mut Struct1014;
    let mut local_12: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;
    let temp_7ff4f6878d8: *mut Struct1013;

    u_var11 = (param_1 >> 0x10);
    iVar10 = param_1;
    param_1.ptr_a_lo = 0xe036;
    (iVar10 + 2) = 0x1030;
    if (ctx._PTR_LOOP_1050_65e2 != 0) {
        u_var3 = pass1_1028_b58e(param_1);
        temp_7ff4f6878d8 = CONCAT31(extraout_var, u_var3);
        if ((iVar10 + 0x20) == 0) {
            if ((in_dx | temp_7ff4f6878d8) == 0) {
                paVar4 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, 1, 0x400);
            } else {
                paVar4 = temp_7ff4f6878d8.field_0x2e;
                in_dx = temp_7ff4f6878d8.field_0x30;
            }
            u_var9 = in_dx;
            pu_var7 = pass1_1008_c6fa(ctx._PTR_LOOP_1050_06e0, 0x1e);
            u_var5 = SUB42(pu_var7, 0);
            pass6_funcs::pass1_1038_4d6e(CONCAT22(in_dx, paVar4), pu_var7 & 0xffff | u_var9 << 0x10);
            _local_14 = CONCAT22(ctx.dx_reg, u_var5);
            ppc_var2 = (*_local_14 + 0x10);
            u_var11 = u_var5;
            ppc_var2(&ctx.PTR_LOOP_1050_1038, u_var5, ctx.dx_reg);
            _local_18 = CONCAT22(ctx.dx_reg, u_var11);
            local_1c = 0;
            while (local_1c < _local_18) {
                u_var8 = _local_18;
                pass1_1030_1d7c(u_var5, ctx.dx_reg, local_1c, (local_1c >> 0x10));
                if ((ctx.dx_reg | u_var8) != 0) {
                    u_var6 = pass1_1030_dfcc(param_1);
                    u_var6 = pass1_1030_cbf0(u_var8, ctx.dx_reg, u_var6);
                    if (u_var6 != 0) {
                        break;
                    }
                }
                local_1c = local_1c + 1;
            }
            if (_local_14 != 0x0) {
                ppc_var2 = *_local_14;
                ppc_var2(0x38, u_var5, ctx.dx_reg, 1);
            }
        } else {
            u_var1 = (iVar10 + 0x20);
            u_var9 = in_dx;
            paVar4 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, u_var1, (u_var1 >> 0x10));
            if ((u_var9 | paVar4) != 0) {
                local_c = 0;
                match (iVar10 + 0xc) {
                    0x73 | 0x77 => {
                        local_c = 1;
                    }
                    0x74 | 0x78 => {
                        local_c = 2;
                    }
                    0x75 => {
                        local_c = 3;
                    }
                    0x76 => {
                        local_c = 5;
                    }
                }
                if (local_c != 0) {
                    pass1_1030_cc44(
                        paVar4,
                        u_var9,
                        1,
                        CONCAT31(extraout_var, u_var3) & 0xffff | in_dx << 0x10,
                        local_c,
                    );
                }
            }
        }
    }
    pass1_1028_b418(param_1);
    return;
}

pub unsafe fn pass1_1030_db78(param_1: u32) {
    let mut u_var1: i32;

    u_var1 = (param_1 >> 0x10);
    if ((param_1 + 0x12) == 6) {
        pass1_1028_bdac((param_1 & 0xffff | u_var1 << 0x10), 5);
    }
    return;
}

pub unsafe fn pass1_1030_db92(
    param_1: u16,
    param_2: u16,
    param_1_00: u32,
    param_2_00: u32,
    param_5: u32,
) -> i32 {
    let mut u_var1: u32;
    let paVar2: *mut Struct493;
    let pu_var3: Vec<u8>;
    let mut u_var4: u16;
    let mut unaff_ss: u16;
    let mut u_var5: u32;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u32;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: [u8; 2];

    u_var5 = pass1_1030_bcae(local_4, unaff_ss);
    u_var4 = (u_var5 >> 0x10);
    paVar2 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, param_2_00, (param_2_00 >> 0x10));
    u_var1 = &paVar2.field_0x10;
    paVar2 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, u_var1, (u_var1 >> 0x10));
    pu_var3 = local_4;
    pass1_1030_bcde(
        pu_var3,
        unaff_ss,
        CONCAT22(u_var4, paVar2),
        param_1_00,
        param_5,
    );
    if (pu_var3 < 0) {
        PTR_LOOP_1050_50ca = 0x6af;
        return;
    }
    return;
}

pub unsafe fn pass1_1030_d868(param_1: &mut Struct44, param_2: u8) -> &mut Struct44 {
    pass1_1028_b418(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1030_d8f6(param_1: *mut Struct763) -> *mut Struct763 {
    let local_bx_12: *mut Struct1008;
    let mut u_var1: u16;

    pass1_1028_b354(param_1);
    u_var1 = (param_1 >> 0x10);
    local_bx_12 = param_1;
    param_1.field_0x0 = 0xdc2e;
    local_bx_12.field_0x2 = 0x1030;
    if (local_bx_12.field_0xc == 0x4c) {
        local_bx_12.field_0xe = 0x43;
    } else {
        if (local_bx_12.field_0xc == 0x4d) {
            local_bx_12.field_0xe = 0x44;
        } else {
            local_bx_12.field_0xe = 0x45;
        }
    }
    return param_1;
}

pub unsafe fn pass1_1030_d942(
    param_1: *mut Struct1009,
    param_2: u16,
    param_3: u16,
    param_3_00: u32,
) {
    pass1_1028_b39e(CONCAT22(param_2, param_1), param_3, param_3_00);
    CONCAT22(param_2, param_1) = 0xdc2e;
    param_1.field_0x2 = 0x1030;
    if (param_1.field_0xc == 0x4c) {
        param_1.field_0xe = 0x43;
    } else {
        if (param_1.field_0xc == 0x4d) {
            param_1.field_0xe = 0x44;
        } else {
            param_1.field_0xe = 0x45;
        }
    }
    return CONCAT22(param_2, param_1);
}

pub unsafe fn pass1_1030_d994(param_1: &mut Struct44) {
    let piVar1: *mut i32;
    let mut u_var2: u32;
    let mut i_var3: i32;
    let local_bx_4: *mut Struct1010;
    let mut u_var4: u16;
    let mut u_var5: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    u_var4 = (param_1 >> 0x10);
    local_bx_4 = param_1;
    if (local_bx_4.field_0x12 != 4) {
        return;
    }
    u_var5 = pass1_1028_b4f2(param_1);
    if ((u_var5 + 0x200) == 0x8000002) {
        u_var2 = local_bx_4.field_0x14;
        piVar1 = (u_var2 + 0x94);
        *piVar1 = *piVar1 + -1;
    } else {
        i_var3 = pass1_1028_cb04(param_1);
        if (i_var3 == 0) {
            return;
        }
        pass1_1030_dace(param_1);
        if (i_var3 == 0) {
            return;
        }
        u_var2 = local_bx_4.field_0x14;
        piVar1 = (u_var2 + 0x94);
        *piVar1 = *piVar1 + -1;
        pass1_1028_c952(param_1);
        pass1_1030_da22(param_1);
    }
    u_var2 = local_bx_4.field_0x14;
    if ((u_var2 + 0x94) < 1) {
        pass1_1028_bdac(param_1, 5);
    }
    return;
}

pub unsafe fn pass1_1030_da22(param_1: &mut Struct44) {
    let pu_var1: *mut u32;
    let ppc_var2: fn();
    let mut u_var3: u32;
    let mut u_var4: u16;
    let b_var5: bool;
    let mut u_var6: i32;
    let pu_var7: *mut u32;
    let mut u_var8: u32;


    let mut u_var9: u32;
    let mut local_1a: u32;
    let mut local_12: u32;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    u_var9 = pass1_1028_b4f2(param_1);
    u_var4 = (u_var9 >> 0x10);
    pu_var1 = (u_var9 + 0xc);
    ppc_var2 = (*pu_var1 + 0x10);
    pu_var7 = pu_var1;
    ppc_var2(&PTR_LOOP_1050_1028, pu_var1, (u_var9 + 0xe));
    u_var3 = pu_var7 & 0xffff | ctx.dx_reg << 0x10;
    local_12 = 0;
    loop {
        if (u_var3 <= local_12) {
            return;
        }
        u_var8 = u_var3;
        pass1_1030_1d7c(pu_var1, (pu_var1 >> 0x10), local_12, (local_12 >> 0x10));
        u_var6 = u_var8;
        if ((((ctx.dx_reg | u_var6) != 0)
            && (
                b_var5 = pass1_1008_c6ae(ctx._PTR_LOOP_1050_06e0, (u_var6 + 0xc), 4),
                b_var5 != 0,
            ))
            && (
                u_var9 = pass1_1028_6744(
                    CONCAT13((ctx.dx_reg >> 8), CONCAT12(ctx.dx_reg, u_var6)),
                    0xd,
                ),
                ((u_var9 >> 0x10) | u_var9) != 0,
            ))
        {
            break;
        }
        local_12 = local_12 + 1;
    }
    pass1_1028_6228(u_var8 & 0xffff | ctx.dx_reg << 0x10, 1, 0, 0xd);
    return;
}

pub unsafe fn pass1_1030_dace(param_1: &mut Struct44) {
    let pu_var1: *mut u32;
    let ppc_var2: fn();
    let mut u_var3: u32;
    let mut u_var4: u16;
    let b_var5: bool;
    let mut u_var6: i32;
    let pu_var7: *mut u32;
    let mut u_var8: u32;


    let mut u_var9: u32;
    let mut local_1c: u32;
    let mut local_14: u32;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u32;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    u_var9 = pass1_1028_b4f2(param_1);
    u_var4 = (u_var9 >> 0x10);
    pu_var1 = (u_var9 + 0xc);
    ppc_var2 = (*pu_var1 + 0x10);
    pu_var7 = pu_var1;
    ppc_var2(&PTR_LOOP_1050_1028, pu_var1, (u_var9 + 0xe));
    u_var3 = pu_var7 & 0xffff | ctx.dx_reg << 0x10;
    local_14 = 0;
    loop {
        if (u_var3 <= local_14) {
            return;
        }
        u_var8 = u_var3;
        pass1_1030_1d7c(pu_var1, (pu_var1 >> 0x10), local_14, (local_14 >> 0x10));
        u_var6 = u_var8;
        if ((((ctx.dx_reg | u_var6) != 0)
            && (
                b_var5 = pass1_1008_c6ae(ctx._PTR_LOOP_1050_06e0, (u_var6 + 0xc), 4),
                b_var5 != 0,
            ))
            && (
                u_var9 = pass1_1028_6744(
                    CONCAT13((ctx.dx_reg >> 8), CONCAT12(ctx.dx_reg, u_var6)),
                    0xd,
                ),
                ((u_var9 >> 0x10) | u_var9) != 0,
            ))
        {
            break;
        }
        local_14 = local_14 + 1;
    }
    return;
}

pub unsafe fn pass1_1030_c8da(param_1: u32, param_2: u32, param_3: u32) {
    let mut local_6: u32;

    local_6 = 0;
    if (param_3._2_2_ == 1) {
        (param_1 + 0x20) = param_2._2_2_;
    } else {
        local_6 = ret_1030_178e(param_1, param_2, param_3);
    }
    return local_6;
}

pub unsafe fn pass1_1030_c91a(param_1: &mut Struct44, param_2: u8) -> &mut Struct44 {
    pass1_1028_b418(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn bad_fn_1030_c949() {
    let pc_var1: String;
    let u_var2: u8;
    let mut in_dx: u16;
    let mut in_bx: i32;
    let mut cVar3: u8;
    let pu_var4: *mut u16;
    // ppu_var6: *mut Vec<u8>;
    let unaff_bp: *mut u16;
    let mut unaff_si: i32;
    let unaff_DI: Vec<u8>;
    let mut unaff_es: u16;
    let mut unaff_cs: u16;
    let mut unaff_ss: u16;
    // appuStack8362: *mut Vec<u8>[3];
    let mut uStack8356: u32;
    let mut bStack8352: u8;
    // ppuStack4216: *mut Vec<u8>;
    let apu_stack4176: [u8; 2073];
    let puStack30: Vec<u8>;
    let pu_var5: *mut u16;
    // ppu_var7: *mut Vec<u8>;

    puStack30 = &stack0xfffe;
    pu_var4 = &stack0xfffe;
    apu_stack4176[0] = &stack0xfffe;
    pu_var5 = &stack0xfffe;
    cVar3 = '\r';
    while {
        unaff_bp = unaff_bp + -1;
        pu_var4 = pu_var4 + -1;
        *pu_var4 = *unaff_bp;
        cVar3 = cVar3 + -1;
        '\0' < cVar3
    } {}
    ppuStack4216 = apu_stack4176;
    ppu_var6 = apu_stack4176;
    appuStack8362[0] = apu_stack4176;
    ppu_var7 = apu_stack4176;
    cVar3 = '\x13';
    while {
        pu_var5 = pu_var5 + -1;
        ppu_var6 = ppu_var6 + -1;
        *ppu_var6 = *pu_var5;
        cVar3 = cVar3 + -1;
        '\0' < cVar3
    } {}
    ppu_var6 = appuStack8362;
    cVar3 = '\x1b';
    while {
        ppu_var7 = ppu_var7 + -1;
        ppu_var6 = ppu_var6 + -1;
        *ppu_var6 = *ppu_var7;
        cVar3 = cVar3 + -1;
        '\0' < cVar3
    } {}
    pc_var1 = unaff_DI + 0x1028;
    cVar3 = (in_bx >> 8);
    if (*pc_var1 != cVar3 && cVar3 <= *pc_var1) {
        pc_var1 = (in_bx + unaff_si);
        *pc_var1 = *pc_var1 - in_dx;
        pc_var1 = (in_bx + unaff_si);
        *pc_var1 = *pc_var1 - in_dx;
        u_var2 = _in(in_dx);
        *unaff_DI = u_var2;
        // WARNING: Bad instruction - Truncating control flow here
        halt_baddata();
    }
    0x1024 = uStack8356;
    0x1022 = unaff_cs;
    0x1020 = 0xc927;
    pass1_1028_b418(paRam00001024);
    if ((bStack8352 & 1) != 0) {
        0x1024 = uStack8356;
        0x1022 = &PTR_LOOP_1050_1028;
        0x1020 = 0xc936;
        error_check_1000_17ce(paRam00001024);
    }
    return uStack8356;
}

pub unsafe fn pass1_1030_c9a8(param_1: *mut Struct763) -> *mut Struct763 {
    let mut iVar1: i32;
    let mut u_var2: u16;

    pass1_1028_b354(param_1);
    u_var2 = (param_1 >> 0x10);
    iVar1 = param_1;
    (iVar1 + 0x98) = 1;
    param_1.field_0x0 = 0xd88e;
    (iVar1 + 2) = 0x1030;
    pass1_1000_4906((param_1 & 0xffff0000 | (iVar1 + 0x20)), 0, 0x78);
    return param_1;
}

pub unsafe fn pass1_1030_c9e4(
    param_1: *mut Struct764,
    param_2: *mut Struct764,
    param_3: u16,
    param_3_00: u32,
) -> i32 {
    pass1_1028_b39e(CONCAT22(param_2, param_1), param_3, param_3_00);
    param_1.field_0x98 = 1;
    CONCAT22(param_2, param_1) = 0xd88e;
    param_1.field_0x2 = 0x1030;
    pass1_1000_4906(CONCAT22(param_2, &param_1.field_0x20), 0, 0x78);
    return CONCAT22(param_2, param_1);
}

pub unsafe fn pass1_1030_ca26(param_1: &mut Struct44, param_3: Vec<u8>) {
    let u_var1: u8;
    let extraout_var: u32;
    let mut in_dx: i32;
    let local_bx_39: *mut Struct981;
    let local_bx_87: *mut Struct982;
    let mut u_var2: u16;
    let mut local_8: u16;
    let local_4: *mut Struct983;

    local_4 = 0x0;
    while (
        local_bx_87 = param_1,
        u_var2 = (param_1 >> 0x10),
        local_4 < 10,
    ) {
        if (((local_bx_87 + local_4 * 0xc + 0x26) == 2)
            || ((local_bx_87 + local_4 * 0xc + 0x26) == 1))
        {
            (local_bx_87 + local_4 * 0xc + 0x26) = 4;
        } else {
            u_var1 = pass1_1028_b58e(param_1);
            local_bx_39 = (local_bx_87 + local_4 * 0xc);
            pass1_1030_6e9c(
                CONCAT31(extraout_var, u_var1) & 0xffff | in_dx << 0x10,
                1,
                local_bx_39.field_0x24,
            );
            local_bx_39.field_0x20 = 0;
            local_bx_39.field_0x24 = 0;
            local_bx_39.field_0x26 = 0;
        }
        local_4 = &local_4.field_0x1;
    }
    pass1_1028_b46e(param_1, param_3);
    return;
}
