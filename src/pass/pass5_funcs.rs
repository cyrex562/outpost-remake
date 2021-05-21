use crate::err_ops::{error_check_1000_0dc6, error_check_1000_17ce};
use crate::file_ops::read::read_from_file_1030_4e70;
use crate::func_ptr_funcs::call_fn_ptr_1000_5586;
use crate::list_funcs::{modify_list_1008_6cb4, modify_list_1008_6d18, modify_list_1008_6d3e, modify_list_1008_6d64, zero_list_1008_6c90};
use crate::mem_funcs::{alloc_mem_1000_07fc, alloc_mem_1000_0ed4, alloc_mem_1000_1708, free_mem_1000_093a};
use crate::mixed_fn_1010_830a;
use crate::other_funcs::{modify_list_1008_3f62, process_switch_stmt_1010_6cf8, xor_1000_49b2, zero_list_1008_3e38};
use crate::pass::pass12_funcs::{pass1_1008_c646, pass1_1008_c6ae, pass1_1008_c6fa};
use crate::pass::pass14_funcs::{pass1_1008_3e54, pass1_1008_3e76, pass1_1008_3e94, pass1_1008_3eb4, pass1_1008_5784, pass1_1008_5b12, pass1_1008_6cec, pass1_fn_1008_60e8, pass1_fn_1008_612e};
use crate::pass::pass15_funcs::{infinite_loop_1020_ba94, pass1_1020_ba3e, pass1_1020_ba7e, pass1_1020_bae6, pass1_1020_bb16, pass1_1020_bb8a, switch_statement_1020_c3b4};
use crate::pass::pass16_funcs::{pass1_1028_6228, pass1_1028_6302, pass1_1028_6356, pass1_1028_6408, pass1_1028_6744, pass1_1028_67d4};
use crate::pass::pass17_funcs;
use crate::pass::pass20_funcs::{pass1_1010_9092, pass1_1010_9794, pass1_1010_9f8c, pass1_1010_ed22, pass1_1010_ed3e, pass1_1018_04a4, pass1_1018_04b8, pass1_1018_04ca};
use crate::pass::pass3_funcs::{pass1_1028_1106, pass1_1028_1416, return_false_1028_1c1c, return_false_1028_20b0};
use crate::pass::pass4_funcs::{pass1_1028_b22c, pass1_1028_b260, pass1_1028_b354, pass1_1028_b39e, pass1_1028_b418, pass1_1028_b46e, pass1_1028_b58e, pass1_1028_bb6a, pass1_1028_bb96, pass1_1028_bd38, pass1_1028_bdac, pass1_1028_be2a, pass1_1028_be9e, pass1_1028_bf22, pass1_1028_c314, pass1_1028_c7b6, pass1_1028_cfd2, pass1_1028_cff2, pass1_1028_d078, pass1_1028_d22e, pass1_1028_d282, pass1_1028_d52c, pass1_1028_d566, pass1_1028_d81c, pass1_1028_daba, pass1_1028_dc52, pass1_1028_e198, pass1_1028_e1ec, pass1_1028_e2ac, pass1_1028_e2e0, pass1_1028_e332, pass1_1028_e372, pass1_1028_e44a, pass1_1028_e4ec, pass1_1030_12ca, pass1_1030_1358, pass1_1030_1628, pass1_1030_165e, pass1_1030_16b2};
use crate::pass::pass6_funcs::{pass1_1038_3fb0, pass1_1038_44d8, pass1_1038_48e0, pass1_1038_4900, pass1_1038_4b20, pass1_1038_4d0e, pass1_1038_4d6e, pass1_1038_4e78, pass1_1038_52b8, pass1_1038_53ba, pass1_1038_540a, pass1_1038_5694, pass1_1038_6984, pass1_1038_69fe, pass1_1038_7964};
use crate::pass::pass8_funcs::{pass1_1010_043a, pass1_1010_65d0, pass1_1010_82f8};
use crate::pass::pass_funcs::{pass1_1000_4906, pass1_1000_49c6, pass1_1000_4aea, pass1_fn_1000_3e2c, pass1_fn_1000_47a4, pass1_fn_1000_54a0};
use crate::prog_structs::prog_structs_1::{Struct104, Struct393};
use crate::prog_structs::prog_structs_10::Struct946;
use crate::prog_structs::prog_structs_11::{Struct1005, Struct706};
use crate::prog_structs::prog_structs_12::{Struct94, Struct947, Struct960};
use crate::prog_structs::prog_structs_13::Struct880;
use crate::prog_structs::prog_structs_14::Struct903;
use crate::prog_structs::prog_structs_15::{Struct707, Struct833, Struct916, Struct921};
use crate::prog_structs::prog_structs_16::{Struct493, Struct520, Struct929};
use crate::prog_structs::prog_structs_17::{Struct1115, Struct850, Struct996};
use crate::prog_structs::prog_structs_18::Struct876;
use crate::prog_structs::prog_structs_19::Struct952;
use crate::prog_structs::prog_structs_2::{Struct199, Struct862, Struct955};
use crate::prog_structs::prog_structs_20::{Struct519, Struct889};
use crate::prog_structs::prog_structs_21::{Struct1006, Struct1007, Struct854, Struct924, Struct930, Struct943, Struct967};
use crate::prog_structs::prog_structs_22::{Struct859, Struct922};
use crate::prog_structs::prog_structs_24::{Struct2111, Struct708, Struct904, Struct917, Struct928, Struct970};
use crate::prog_structs::prog_structs_25::{Struct851, Struct900};
use crate::prog_structs::prog_structs_26::{Struct852, Struct853, Struct855, Struct856, Struct861, Struct865, Struct867, Struct868, Struct869, Struct870, Struct871, Struct877, Struct879, Struct884, Struct885, Struct886, Struct887, Struct890, Struct892, Struct895, Struct896, Struct897, Struct898, Struct899, Struct940, Struct953, Struct984, Struct997};
use crate::prog_structs::prog_structs_27::{pass1_struct_2, Struct848, Struct849, Struct901, Struct902, Struct906, Struct908, Struct909, Struct923, Struct925, Struct926, Struct927, Struct966, Struct974};
use crate::prog_structs::prog_structs_28::{Struct1000, Struct1001, Struct1002, Struct1003, Struct1004, Struct781, Struct910, Struct914, Struct918, Struct919, Struct949, Struct972, Struct985};
use crate::prog_structs::prog_structs_29::{Struct763, Struct764, Struct941, Struct942, Struct954, Struct959, Struct968, Struct990};
use crate::prog_structs::prog_structs_30::{Struct938, Struct939, Struct969, Struct973, Struct986, Struct991};
use crate::prog_structs::prog_structs_31::{Struct931, Struct944, Struct950, Struct956, Struct957, Struct958, Struct963, Struct987, Struct988, Struct989, Struct992, Struct993, Struct994};
use crate::prog_structs::prog_structs_5::{Struct1, Struct951};
use crate::prog_structs::prog_structs_6::{Struct858, Struct888};
use crate::prog_structs::prog_structs_7::Struct44;
use crate::prog_structs::prog_structs_8::Struct964;
use crate::prog_structs::prog_structs_9::{Struct857, Struct872, Struct920};
use crate::string_ops::{fn_1008_6048, load_string_1010_847e, pass1_1028_87f0, pass1_1030_4594, pass1_1030_4dbc, pass1_1030_521c, string_fn_1000_3f9c, wvsprintf_FUN_1030_840a};
use crate::struct_ops::{pass1_1030_3af6, pass1_1030_4574, pass1_1030_8c66, pass1_1038_78e2, process_struct_1000_179c, process_struct_1008_4772, process_struct_1008_574a, struct_fn_1000_160a};
use crate::sys_ops::win_msg::post_win_msg_1008_a0e4;
use crate::util::{CARRY2, CONCAT11, CONCAT12, CONCAT13, CONCAT22, CONCAT31, SBORROW2, SUB21, SUB41, SUB42, ZEXT24};

pub unsafe fn pass1_1030_cac2(param_1: &mut Struct44) {
    let mut u_var1: u16;
    let mut u_var2: u32;
    let pu_var3: *mut u32;
    let ppcVar4: fn();
    let mut u_var5: u32;
    let u_var6: u8;
    let pa_var7: *mut Struct493;
    let pa_var8: *mut Struct493;
    let extraout_var: u32;
    let pu_var9: *mut u32;
    let mut u_var10: u32;
    
    let mut u_var11: u16;
    
    
    let mut u_var12: i32;
    let mut local_34: u16;
    let mut local_32: u16;
    let mut local_30: u16;
    let mut local_2c: u16;
    let mut local_22: u32;
    let mut local_1e: u16;
    let mut local_1c: u16;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_16: u32;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    pass1_1028_be9e(param_1);
    u_var12 = (param_1 >> 0x10);
    if (((param_1 + 0x12) == 5) && (PTR_LOOP_1050_5812 == 0x0)) {
        PTR_LOOP_1050_5812 = (&ctx.PTR_LOOP_1050_0000 + 1);
        u_var11 = ctx.dx_reg;
        u_var6 = pass1_1028_b58e((param_1 & 0xffff | u_var12 << 0x10));
        u_var2 = (CONCAT31(extraout_var, u_var6) + 0x2e);
        u_var2 = (u_var2 + 0x10);
        pa_var7 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, u_var2, (u_var2 >> 0x10));
        pu_var3 = (pa_var7 + 1);
        unsafe { ppcVar4 = (*pu_var3 + 0x10) };
        pu_var9 = pu_var3;
        (**ppcVar4)(&PTR_LOOP_1050_1028, pu_var3, &pa_var7[1].field_0x2);
        u_var5 = pu_var9 & 0xffff | ctx.dx_reg << 0x10;
        local_1c = 0;
        local_1e = pass1_1030_d144(param_1);
        local_22 = 0;
        while (local_22 < u_var5 && (local_1e != 0)) {
            unsafe { ppcVar4 = (*pu_var3 + 4) };
            u_var10 = u_var5;
            (**ppcVar4)(
                &PTR_LOOP_1050_1028,
                pu_var3,
                (pu_var3 >> 0x10),
                local_22,
                (local_22 >> 0x10),
            );
            u_var12 = ctx.dx_reg | u_var10;
            if (u_var12 != 0) {
                pa_var8 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, u_var10, ctx.dx_reg);
                u_var1 = &pa_var8.field_0xc;
                if ((0 < u_var1) && (!SBORROW2(u_var1, 1))) {
                    if (u_var1 != 3 && 0 < (u_var1 - 2)) {
                        if (u_var1 != 4) {}
                        // goto LAB_1030_cbbc;
                        local_1c = local_1c + 1;
                    }
                    pass17_funcs::pass1_1030_6e9c(CONCAT22(u_var11, pa_var7), 1, u_var1);
                    pass1_1030_d180(param_1, u_var1);
                    local_1e = local_1e - 1;
                }
            }
            // LAB_1030_cbbc:
            local_22 = local_22 + 1;
        }
        while (local_1c < 4) {
            pass1_1030_d180(param_1, 4);
            local_1c = local_1c + 1;
        }
    }
    return;
}

pub unsafe fn pass1_1030_cbf0(param_1: i32, param_2: u16, param_3: i32) {
    let local_bx_39: *mut Struct984;
    let local_4: *mut Struct985;

    local_4 = 0x0;
    loop {
        if (9 < local_4) {
            return 0;
        }
        local_bx_39 = (local_4 * 0xc + param_1);
        if ((local_bx_39.field_0x24 == param_3) && (local_bx_39.field_0x26 == 3)) {
            break;
        }
        local_4 = &local_4.field_0x1;
    }
    local_bx_39.field_0x26 = 0;
    local_bx_39.field_0x28 = 0;
    return 1;
}

pub unsafe fn pass1_1030_cc44(
    param_1: *mut Struct987,
    param_2: u16,
    param_3: i32,
    param_4: u32,
    param_5: u16,
) -> i32 {
    let pp_var1: fn();
    let u_var2: u8;
    let pu_var3: Vec<u8>;
    let mut u_var4: u16;
    let mut u_var5: u16;
    let paVar6: *mut Struct493;
    let mut u_var7: u32;
    let extraout_var: u32;
    let pu_var8: Vec<u8>;
    
    
    
    let local_bx_202: *mut Struct986;
    let u_var9: u8;
    let mut local_3a: u32;
    let mut local_36: u16;
    let mut local_34: u16;
    let mut local_32: [u8; 8];
    let mut local_2a: u32;
    let mut local_26: u32;
    let mut local_22: u32;
    let mut local_1e: u32;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u32;
    let mut local_e: u16;
    let mut local_c: u16;
    let local_a: *mut Struct988;
    let mut local_8: u32;
    let mut local_4: u16;

    local_4 = 0;
    local_8 = (param_4 + 4);
    local_a = 0x0;
    loop {
        if (9 < local_a) {
            return;
        }
        local_bx_202 = (param_1 + local_a * 0xc);
        if (((local_bx_202.field_0x28 == local_8) && (local_bx_202.field_0x2a == local_8._2_2_))
            && (local_bx_202.field_0x24 == param_5))
        {
            if (local_bx_202.field_0x26 == 4) {
                u_var2 = pass1_1028_b58e(CONCAT22(param_2, param_1));
                local_e = CONCAT31(extraout_var, u_var2);
                local_c = local_8._2_2_;
                pass17_funcs::pass1_1030_6e9c(
                    CONCAT13((local_8._2_2_ >> 8), CONCAT12(local_8._2_2_, local_e)),
                    1,
                    local_bx_202.field_0x24,
                );
                local_bx_202.field_0x20 = 0;
                local_bx_202.field_0x24 = 0;
                local_bx_202.field_0x26 = 0;
                local_2a = 0;
                local_12 = 0;
                _DAT_0000_0006 = param_5;
                uRam0000000a = 1;
                u_var4 = switch_statement_1020_c3b4(param_5);
                (local_12 + 0xc) = u_var4;
                pu_var8 = pass1_1008_c6fa(ctx._PTR_LOOP_1050_06e0, 4);
                u_var5 = pu_var8;
                u_var4 = local_8._2_2_;
                local_16 = u_var5;
                local_14 = local_8._2_2_;
                paVar6 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, 1, 0x400);
                u_var9 = 0x38;
                local_1a = paVar6;
                local_18 = local_8._2_2_;
                pass1_1038_4d6e(CONCAT22(local_8._2_2_, paVar6), CONCAT22(u_var4, u_var5));
                local_1e = CONCAT22(ctx.dx_reg, paVar6);
                pp_var1 = (local_1e + 0x10);
                (**pp_var1)(&PTR_LOOP_1050_1038, paVar6, ctx.dx_reg);
                local_22 = CONCAT22(ctx.dx_reg, paVar6);
                local_26 = 0;
                while (local_26 < local_22) {
                    u_var7 = local_22;
                    pass17_funcs::pass1_1030_1d7c(local_1e, (local_1e >> 0x10), local_26, (local_26 >> 0x10));
                    if ((ctx.dx_reg | u_var7) != 0) {
                        pu_var3 = local_32;
                        pp_var1 = ((u_var7 & 0xffff | ctx.dx_reg << 0x10) + 0x40);
                        (**pp_var1)(0x38, u_var7, ctx.dx_reg, pu_var3);
                        if (pu_var3 == 0x0) {
                            u_var9 = 0x28;
                            pass1_1028_6408(u_var7 & 0xffff | ctx.dx_reg << 0x10, local_12);
                            break;
                        }
                    }
                    local_26 = local_26 + 1;
                }
                local_2a = local_1e;
                if (local_1e != 0) {
                    pp_var1 = local_1e;
                    (**pp_var1)(u_var9, local_1e, (local_1e >> 0x10), 1);
                }
            } else {
                (param_1 + local_a * 0xc + 0x26) = 0;
                (param_1 + local_a * 0xc + 0x28) = 0;
            }
            local_4 = local_4 + 1;
            param_3 = param_3 + -1;
            if (param_3 == 0) {
                return;
            }
        }
        local_a = &local_a.field_0x1;
    }
}

pub unsafe fn pass1_1030_cde8(param_1: i32, param_2: u16, param_3: i32) {
    let mut iVar1: i32;
    let mut local_4: u16;

    local_4 = 0;
    loop {
        if (9 < local_4) {
            return 0xffff;
        }
        iVar1 = local_4 * 0xc + param_1;
        if (((iVar1 + 0x24) == param_3) && ((iVar1 + 0x26) == 0)) {
            break;
        }
        local_4 = local_4 + 1;
    }
    return local_4;
}

pub unsafe fn pass1_1030_ce2e(param_1: i32, param_2: u16, param_3: i32) -> i32 {
    let mut iVar1: i32;
    let mut local_6: u32;

    local_6 = 0;
    while (local_6 < 10) {
        iVar1 = local_6 * 0xc + param_1;
        if (((iVar1 + 0x24) == param_3) && ((iVar1 + 0x26) == 0)) {
            local_6 = local_6 & 0xffff | (local_6._2_2_ + 1) << 0x10;
        }
        local_6 = local_6 & 0xffff0000 | (local_6 + 1);
    }
    return local_6._2_2_;
}

pub unsafe fn pass1_1030_ce72(param_1: u32, param_2: i32, param_3: u32, param_4: i32) {
    let mut u_var1: u32;
    let mut i_var2: i32;
    let mut local_a: u16;
    let mut local_8: u32;
    let mut local_4: u16;

    u_var1 = (param_3 + 4);
    local_a = 0;
    loop {
        if (9 < local_a) {
            return;
        }
        i_var2 = local_a * 0xc + param_1;
        if (((i_var2 + 0x24) == param_4) && ((i_var2 + 0x28) == 0)) {
            (i_var2 + 0x28) = u_var1;
            if (param_4 == 4) {
                (i_var2 + 0x26) = 2;
            } else {
                (param_1 + local_a * 0xc + 0x26) = 1;
            }
            param_2 = param_2 + -1;
            if (param_2 == 0) {
                return;
            }
        }
        local_a = local_a + 1;
    }
}

pub unsafe fn pass1_1030_cef8(
    param_1: *mut Struct989,
    param_2: u32,
    param_3: u16,
    param_4: *mut Struct990,
) -> i32 {
    let mut u_var1: u16;
    let local_bx_20: *mut Struct989;
    let local_es_20: *mut Struct989;
    let mut u_var2: u16;

    local_es_20 = (param_1 >> 0x10);
    local_bx_20 = param_1;
    (local_bx_20 + param_4 * 0xc + 0x26) = param_3;
    u_var2 = (param_2 >> 0x10);
    u_var1 = (param_2 + 6);
    (local_bx_20 + param_4 * 0xc + 0x28) = (param_2 + 4);
    (local_bx_20 + param_4 * 0xc + 0x2a) = u_var1;
    return;
}

pub unsafe fn pass1_1030_cf3a(param_1: u32, param_2: i32) {
    let mut local_4: u16;

    local_4 = 0;
    loop {
        if (9 < local_4) {
            return 0;
        }
        if ((param_1 + local_4 * 0xc + 0x24) == param_2) {
            break;
        }
        local_4 = local_4 + 1;
    }
    return 1;
}

pub unsafe fn pass1_1030_cf78(param_1: &mut Struct44, param_3: i32) {
    let u_var1: u8;
    let extraout_var: u32;
    let mut in_dx: i32;
    let local_bx_40: *mut Struct991;
    let mut u_var2: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let local_4: *mut Struct992;

    local_4 = 0x0;
    loop {
        if (9 < local_4) {
            return;
        }
        u_var2 = (param_1 >> 0x10);
        if ((param_1 + local_4 * 0xc + 0x24) == param_3) {
            break;
        }
        local_4 = &local_4.field_0x1;
    }
    u_var1 = pass1_1028_b58e(param_1);
    if (param_3 == 5) {
        pass1_1038_4900(*(CONCAT31(extraout_var, u_var1) + 0x2e));
    } else {
        pass17_funcs::pass1_1030_6e9c(
            CONCAT31(extraout_var, u_var1) & 0xffff | in_dx << 0x10,
            1,
            param_3,
        );
    }
    local_bx_40 = (local_4 * 0xc + param_1);
    local_bx_40.field_0x20 = 0;
    local_bx_40.field_0x24 = 0;
    local_bx_40.field_0x26 = 0;
    return;
}

pub unsafe fn pass1_1030_d00c(param_1: &mut Struct44, param_2: &mut Struct44, param_3: i32) {
    let u_var1: u8;
    let extraout_var: u32;
    let mut in_dx: i32;
    let local_bx_40: *mut Struct993;
    let mut local_8: u16;
    let mut local_6: u16;
    let local_4: *mut Struct994;

    local_4 = 0x0;
    loop {
        if (9 < local_4) {
            return;
        }
        if (((&param_1.ptr_a_lo + local_4 * 6)[0x13] == 0)
            && ((&param_1.ptr_a_lo + local_4 * 6)[0x12] == param_3))
        {
            break;
        }
        local_4 = &local_4.field_0x1;
    }
    u_var1 = pass1_1028_b58e(CONCAT22(param_2, param_1));
    if (param_3 == 5) {
        pass1_1038_4900(*(CONCAT31(extraout_var, u_var1) + 0x2e));
    } else {
        pass17_funcs::pass1_1030_6e9c(
            CONCAT31(extraout_var, u_var1) & 0xffff | in_dx << 0x10,
            1,
            param_3,
        );
    }
    local_bx_40 = (&param_1.ptr_a_lo + local_4 * 6);
    local_bx_40.field_0x20 = 0;
    local_bx_40.field_0x24 = 0;
    local_bx_40.field_0x26 = 0;
    return;
}

pub unsafe fn pass1_1030_d0a8(param_1: &mut Struct44) {
    let mut u_var1: u16;
    let mut u_var2: i32;
    let mut local_4: u16;

    u_var2 = (param_1 >> 0x10);
    u_var1 = (param_1 + 0x98);
    pass1_1030_d56a((param_1 & 0xffff | u_var2 << 0x10));
    return u_var1;
}

pub unsafe fn pass1_1030_d0c6(param_1: u32) -> i32 {
    let mut local_6: u32;

    local_6 = 0;
    while (local_6 < 10) {
        if ((param_1 + local_6 * 0xc + 0x20) != 0) {
            local_6 = local_6 & 0xffff | (local_6._2_2_ + 1) << 0x10;
        }
        local_6 = local_6 & 0xffff0000 | (local_6 + 1);
    }
    return local_6._2_2_;
}

pub unsafe fn pass1_1030_d102(param_1: *mut Struct997, param_2: u16) -> i32 {
    let ctx.bx_reg: *mut Struct996;
    let mut local_6: u32;

    local_6 = 0;
    while (local_6 < 10) {
        ctx.bx_reg = (param_1 + local_6 * 0xc);
        if ((ctx.bx_reg.field_0x20 != 0) && (ctx.bx_reg.field_0x26 != 0)) {
            local_6 = local_6 & 0xffff | (local_6._2_2_ + 1) << 0x10;
        }
        local_6 = local_6 & 0xffff0000 | (local_6 + 1);
    }
    return local_6._2_2_;
}

pub unsafe fn pass1_1030_d144(param_1: &mut Struct44) -> i32 {
    let mut local_6: u32;

    local_6 = 0;
    while (local_6 < 10) {
        if ((param_1 + local_6 * 0xc + 0x20) == 0) {
            local_6 = local_6 & 0xffff | (local_6._2_2_ + 1) << 0x10;
        }
        local_6 = local_6 & 0xffff0000 | (local_6 + 1);
    }
    return local_6._2_2_;
}

pub unsafe fn pass1_1030_d180(param_1: &mut Struct44, param_2: u16) {
    let u_var1: u8;
    let extraout_var: u32;
    let paVar2: *mut Struct1002;
    let i_var5: &mut Struct44;
    let local_bx_95: *mut Struct1001;
    let local_es_53: &mut Struct44;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_4: u16;
    let temp_5f4875ce46: *mut Struct1002;
    let temp_5f2362e605: *mut Struct1000;

    local_4 = 0;
    loop {
        if (9 < local_4) {
            return;
        }
        local_es_53 = (param_1 >> 0x10);
        i_var5 = param_1;
        if (((&i_var5.field_0x22 + local_4 * 0xc) | (&i_var5.field_0x20)[local_4 * 6]) == 0) {
            break;
        }
        local_4 = local_4 + 1;
    }
    temp_5f2362e605 = *ctx._PTR_LOOP_1050_65e2;
    temp_5f4875ce46 = (ctx._PTR_LOOP_1050_65e2 + 2);
    paVar2 = temp_5f4875ce46 + (0xff37 < temp_5f2362e605);
    local_bx_95 = (&i_var5.ptr_a_lo + local_4 * 6);
    local_bx_95.field_0x20 = temp_5f2362e605 + 1;
    local_bx_95.field_0x22 = paVar2;
    local_bx_95.field_0x24 = param_2;
    pass1_1030_d340(
        i_var5,
        local_es_53,
        param_1 & 0xffff0000 | ZEXT24(&local_bx_95.field_0x20),
    );
    u_var1 = pass1_1028_b58e(param_1);
    if (param_2 == 5) {
        pass1_1038_48e0(*(CONCAT31(extraout_var, u_var1) + 0x2e), 1);
        return;
    }
    pass17_funcs::pass1_1030_7c50(
        CONCAT31(extraout_var, u_var1) & 0xffff | ZEXT24(paVar2) << 0x10,
        1,
        param_2,
    );
    return;
}

pub unsafe fn pass1_1030_d230(param_1: Vec<u8>) {
    let local_4: *mut Struct1003;

    local_4 = 0x0;
    loop {
        if (9 < local_4) {
            return 1;
        }
        if ((param_1 + local_4 * 0xc + 0x20) == 0) {
            break;
        }
        local_4 = &local_4.field_0x1;
    }
    return 0;
}

pub unsafe fn pass1_1030_d26c(param_1: &mut Struct44) {
    let pu_var1: *mut u32;
    let mut u_var2: u32;
    let u_var3: u8;
    let mut u_var4: u32;
    let extraout_var: u32;
    let mut in_dx: i32;
    let local_bx_112: *mut Struct1004;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;

    u_var2 = *ctx._PTR_LOOP_1050_65e2;
    local_8 = 0;
    while (local_8 < 10) {
        local_bx_112 = (local_8 * 0xc + param_1);
        let pu_var1_val = unsafe { *pu_var1 };
        if (((&local_bx_112.field_0x22 | local_bx_112.field_0x20) != 0)
            && (
                pu_var1 = &local_bx_112.field_0x20,
                pu_var1_val < u_var2 || pu_var1_val == u_var2,
            ))
        {
            u_var4 = u_var2;
            pass1_1030_d3b2(param_1, param_1._2_2_, local_8);
            if (u_var4 == 0) {
                u_var3 = pass1_1028_b58e(param_1);
                _local_c = CONCAT31(extraout_var, u_var3) & 0xffff;
                if (local_bx_112.field_0x24 == 5) {
                    pass1_1038_4900(*(_local_c + 0x2e));
                } else {
                    pass17_funcs::pass1_1030_6e9c(
                        _local_c | in_dx << 0x10,
                        1,
                        (param_1 + local_8 * 0xc + 0x24),
                    );
                }
                param_1._0_2_ = local_8 * 0xc + param_1;
                (param_1 + 0x20) = 0;
                (param_1 + 0x24) = 0;
                (param_1 + 0x26) = 0;
            }
        }
        local_8 = local_8 + 1;
    }
    return;
}

pub unsafe fn pass1_1030_d340(param_1: u16, param_2: u16, param_1_00: u32) {
    let mut iVar1: i32;
    let mut i_var2: i32;
    let mut u_var3: u16;

    u_var3 = (param_1_00 >> 0x10);
    i_var2 = param_1_00;
    iVar1 = (i_var2 + 4);
    if (((0 < iVar1) && (!SBORROW2(iVar1, 1))) && (iVar1 == 4 || iVar1 + -1 < 3 || (iVar1 == 0xc)))
    {
        (i_var2 + 6) = 0;
        return;
    }
    (i_var2 + 6) = 1;
    return;
}

pub unsafe fn pass1_1030_d36e(param_1: u32, param_2: *mut Struct1005) {
    let local_4: *mut Struct1006;

    local_4 = 0x0;
    loop {
        if (9 < local_4) {
            return 0;
        }
        if ((local_4 != param_2) && ((param_1 + local_4 * 0xc + 0x24) == 8)) {
            break;
        }
        local_4 = &local_4.field_0x1;
    }
    return 1;
}

pub unsafe fn pass1_1030_d3b2(param_1: i32, param_2: u16, param_2_00: *mut Struct1005) {
    let mut iVar1: i32;
    let paVar2: *mut Struct1115;
    let ppc_var3: fn();
    let mut u8_var4: bool;
    let u_var5: u8;
    let mut u_var6: u16;
    let mut u_var7: u16;
    let mut u_var8: u16;
    let mut u_var9: i32;
    let extraout_var: u32;
    let pu_var10: Vec<u8>;
    let mut u_var11: u32;
    let mut in_dx: i32;
    
    
    
    
    
    let mut extraout_dx_04: u16;
    let mut extraout_dx_05: i32;
    let u_var12: u8;
    let mut u_var13: u32;
    let u_var14: u8;
    let mut local_30: u16;
    let mut local_2e: u16;
    let mut local_28: u16;
    let mut local_26: u16;
    let mut local_1c: u16;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u32;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    u_var5 = pass1_1028_b58e(CONCAT22(param_2, param_1));
    paVar2 = (CONCAT31(extraout_var, u_var5) + 0x2e);
    u_var6 = pass1_1030_d36e(CONCAT22(param_2, param_1), param_2_00);
    if (u_var6 == 0) {
        pu_var10 = pass1_1008_c6fa(ctx._PTR_LOOP_1050_06e0, 0x1e);
        u_var7 = SUB42(pu_var10, 0);
        pass1_1038_4d6e(paVar2, pu_var10 & 0xffff | in_dx << 0x10);
        _local_1a = CONCAT22(ctx.dx_reg, u_var7);
        ppc_var3 = (*_local_1a + 0x10);
        u_var8 = u_var7;
        (**ppc_var3)(&PTR_LOOP_1050_1038, u_var7, ctx.dx_reg);
        _local_12 = CONCAT22(ctx.dx_reg, u_var8);
        u8_var4 = false;
        local_e = 0;
        while (local_e < _local_12) {
            u_var11 = local_e;
            pass17_funcs::pass1_1030_1d7c(u_var7, ctx.dx_reg, local_e, local_e._2_2_);
            local_e._2_2_ = ctx.dx_reg | u_var11;
            if (((local_e._2_2_ != 0) && ((u_var11 + 4) != (param_1 + 4)))
                && (
                    u_var6 = pass1_1030_cf3a(u_var11 & 0xffff | ctx.dx_reg << 0x10, 8),
                    u_var6 != 0,
                ))
            {
                u8_var4 = true;
                break;
            }
            local_e = local_e + 1;
        }
        if (_local_1a != 0x0) {
            ppc_var3 = *_local_1a;
            (**ppc_var3)(0x38, u_var7, ctx.dx_reg, 1);
            local_e._2_2_ = ctx.dx_reg;
        }
        in_dx = local_e._2_2_;
        if (!u8_var4) {
            return;
        }
    }
    pu_var10 = pass1_1008_c6fa(ctx._PTR_LOOP_1050_06e0, 4);
    u_var7 = SUB42(pu_var10, 0);
    u_var12 = 0x38;
    pass1_1038_4d6e(paVar2, pu_var10 & 0xffff | in_dx << 0x10);
    _local_1a = CONCAT22(ctx.dx_reg, u_var7);
    ppc_var3 = (*_local_1a + 0x10);
    u_var8 = u_var7;
    (**ppc_var3)(&PTR_LOOP_1050_1038, u_var7, ctx.dx_reg);
    _local_12 = CONCAT22(extraout_dx_04, u_var8);
    u8_var4 = false;
    local_e = 0;
    loop {
        if (_local_12 <= local_e) {
            // LAB_1030_d51b:
            if (_local_1a != 0x0) {
                ppc_var3 = *_local_1a;
                (**ppc_var3)(u_var12, u_var7, ctx.dx_reg, 1);
            }
            if (!u8_var4) {
                return;
            }
            u_var9 = *ctx._PTR_LOOP_1050_65e2;
            iVar1 = (ctx._PTR_LOOP_1050_65e2 + 2);
            (param_1 + param_2_00 * 0xc + 0x20) = u_var9 + 200;
            (param_1 + param_2_00 * 0xc + 0x22) = iVar1 + (0xff37 < u_var9);
            return;
        }
        u_var11 = local_e;
        pass17_funcs::pass1_1030_1d7c(u_var7, ctx.dx_reg, local_e, local_e._2_2_);
        u_var9 = u_var11;
        if ((extraout_dx_05 | u_var9) != 0) {
            u_var14 = (extraout_dx_05 >> 8);
            u_var12 = 0x28;
            u_var13 = pass1_1028_6744(CONCAT13(u_var14, CONCAT12(extraout_dx_05, u_var9)), 7);
            if (((u_var13 >> 0x10) | u_var13) != 0) {
                u_var12 = 0x28;
                pass1_1028_6228(CONCAT13(u_var14, CONCAT12(extraout_dx_05, u_var9)), 1, 0, 7);
                u8_var4 = true;
                // goto LAB_1030_d51b;
            }
        }
        local_e = local_e + 1;
    }
}

pub unsafe fn pass1_1030_d56a(param_1: *mut Struct1007) -> *mut Struct1007 {
    let local_bx_3: *mut Struct1007;
    let local_es_3: *mut Struct1007;

    local_es_3 = (param_1 >> 0x10);
    local_bx_3 = param_1;
    match (local_bx_3.field_0x98 + -1) {
        0 => local_bx_3.field_0x98 = 2,
        1 => local_bx_3.field_0x98 = 3,
        2 => local_bx_3.field_0x98 = 4,
        3 => local_bx_3.field_0x98 = 0xc,
        // default:
        _ => {
            local_bx_3.field_0x98 = 1;
            return local_bx_3;
        }
        7 => {
            local_bx_3.field_0x98 = 9;
            return local_bx_3;
        }
        8 => {
            local_bx_3.field_0x98 = 0xb;
            return local_bx_3;
        }
        10 => {
            local_bx_3.field_0x98 = 5;
            return local_bx_3;
        }
        0xb => {
            local_bx_3.field_0x98 = 8;
            return local_bx_3;
        }
    }
    return local_bx_3;
}

pub unsafe fn pass1_1030_c2fa(param_1: &mut Struct44) {
    let mut u_var1: i32;
    let mut u_var2: u32;
    let mut u_var3: u32;
    let mut u_var4: u32;
    let mut u_var5: u32;
    let mut u_var6: u32;
    let u_var7: u8;
    let pa_var8: *mut Struct493;
    let mut i_var9: i32;
    let extraout_var: u32;
    let mut u_var10: u32;
    let mut u_var11: u32;
    let in_dx: Vec<u8>;
    let pu_var12: Vec<u8>;
    let mut u_var13: u16;
    let mut u_var14: u32;
    let mut unaff_si: u16;
    let mut u_var15: u16;
    let ppVar16: *mut Struct2111;
    let mut u_var17: u16;
    let mut local_54: u16;
    let mut local_50: u16;
    let mut local_4e: u16;
    let mut local_38: u16;
    let mut local_36: u32;
    let mut local_32: u32;
    let mut local_2e: u32;
    let mut local_2a: u32;
    let mut local_26: u16;
    let mut local_24: u16;
    let mut local_1e: u32;
    let mut local_e: u32;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    u_var15 = (param_1 >> 0x10);
    if ((param_1 + 0xc) != 0xb) {
        pass1_1028_bd38(param_1);
        u_var2 = (param_1 + 0x20);
        pa_var8 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, u_var2, (u_var2 >> 0x10));
        _local_6 = CONCAT22(in_dx, pa_var8);
        pu_var12 = in_dx;
        u_var7 = pass1_1028_b58e(param_1);
        u_var6 = CONCAT31(extraout_var, u_var7) & 0xffff | ZEXT24(pu_var12) << 0x10;
        u_var3 = (CONCAT31(extraout_var, u_var7) + 0x2e);
        ppVar16 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(unaff_si, 0x2f));
        u_var14 = ppVar16 >> 0x10;
        u_var15 = (u_var3 >> 0x10);
        pass1_1010_ed22(ppVar16, (u_var3 + 4));
        u_var4 = pa_var8[0x10].field_0x16;
        u_var13 = (u_var4 >> 0x10);
        u_var10 = u_var4;
        pass17_funcs::pass1_1030_3694(u_var4, u_var13, 3, 2, 0);
        u_var5 = (u_var3 + 0x1f6);
        pass17_funcs::pass1_1030_355c(u_var5, u_var10 & 0xffff | u_var14 << 0x10);
        u_var15 = (u_var5 >> 0x10);
        local_38 = 0;
        while {
            i_var9 = local_38 * 2;
            (i_var9 + u_var5 + 0x174) = (i_var9 + u_var4 + 0x174);
            u_var1 = (i_var9 + u_var4 + 0x180);
            u_var11 = u_var1;
            (i_var9 + u_var5 + 0x180) = u_var1;
            local_38 = local_38 + 1;
            local_38 < 6
        } {}
        local_54 = 0x11;
        loop {
            u_var13 = u_var14;
            u_var15 = u_var11;
            if (0x24 < local_54) {
                break;
            }
            if (0 < (local_54 * 2 + _PTR_LOOP_1050_580e)) {
                pass1_1038_540a(pa_var8, in_dx, local_54);
                _local_50 = CONCAT22(u_var13, u_var15);
                u_var15 = (_PTR_LOOP_1050_580e >> 0x10);
                i_var9 = (local_54 * 2 + _PTR_LOOP_1050_580e);
                u_var14 = i_var9 >> 0x10;
                u_var17 = local_54;
                if (_local_50 < i_var9) {
                    i_var9 = (local_54 * 2 + _PTR_LOOP_1050_580e);
                    u_var14 = (i_var9 >> 0xf);
                    u_var17 = 0x21;
                }
                pass1_1038_52b8(_local_6, CONCAT22(u_var14, i_var9), u_var17);
                pass17_funcs::pass1_1030_7ddc(u_var6, (local_54 * 2 + _PTR_LOOP_1050_580e), local_54);
                i_var9 = (_PTR_LOOP_1050_580e + local_54 * 2);
                u_var11 = SEXT24(i_var9);
                pass1_1038_5694(u_var3, i_var9, local_54);
            }
            local_54 = local_54 + 1;
        }
        pass17_funcs::pass1_1030_7c50(u_var6, 2, 1);
        pass17_funcs::pass1_1030_7c50(u_var6, 2, 2);
        pass17_funcs::pass1_1030_7c50(u_var6, 2, 3);
        pass17_funcs::pass1_1030_7c50(u_var6, 2, 4);
        pass1_1038_44d8(pa_var8, in_dx, 2, (&ctx.PTR_LOOP_1050_0000 + 1));
        pass1_1038_44d8(pa_var8, in_dx, 2, &dos_alloc_addr_1050_0002);
        pass1_1038_44d8(pa_var8, in_dx, 2, (&dos_alloc_addr_1050_0002 + 1));
        pass1_1038_44d8(pa_var8, in_dx, 2, &PTR_DAT_0005_0000_1050_0004);
        ppVar16 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(unaff_si, 0x2b));
        pass1_1010_043a(ppVar16, &pa_var8.field_0x4, 7);
    }
    return;
}

pub unsafe fn pass1_1030_c52e(param_1: u32, param_2: *mut u16, param_3: u32, param_4: u32) {
    let mut u_var1: u16;
    let pu_var2: *mut u32;
    let pu_var3: *mut u16;
    
    
    let mut u_var4: i32;
    let mut unaff_ss: u16;
    let pu_var5: *mut u16;
    let mut u_var6: u32;
    let mut local_38: u16;
    let mut local_32: u16;
    let mut local_30: u16;
    let mut local_20: u32;
    let mut local_1c: u16;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_16: u32;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_c: u32;
    let mut local_8: u16;
    let mut local_6: u32;

    u_var1 = pass1_1028_c314(
        param_1,
        (param_1 >> 0x10),
        param_2,
        param_3,
        (param_3 >> 0x10),
        param_4,
    );
    if (u_var1 != 0) {
        pu_var2 = &local_c;
        pass17_funcs::pass1_1030_64ce(_PTR_LOOP_1050_5740, param_2, param_4, pu_var2, unaff_ss);
        unsafe { local_20 = *pu_var2 };
        local_20._3_1_ = (local_20 >> 0x18);
        local_8 = local_20._3_1_;
        if (local_20._3_1_ == 0) {
            local_16 = local_20;
            local_6 = local_20;
            pass1_1028_c7b6(param_1, param_2, param_4);
            if ((local_8 != 4) && (local_8 != 0)) {
                pass1_1030_bcae(&local_20, unaff_ss);
                pass1_1028_dc52(
                    CONCAT22(unaff_ss, &local_32),
                    (&ctx.PTR_LOOP_1050_0000 + 1),
                    0,
                    0x400,
                );
                loop {
                    pu_var3 = &local_32;
                    pass1_1028_e4ec(CONCAT22(unaff_ss, pu_var3));
                    _local_1c = CONCAT22(ctx.dx_reg, pu_var3);
                    u_var4 = ctx.dx_reg | pu_var3;
                    if (u_var4 == 0) {
                        return;
                    }
                    local_16 = (pu_var3 + 8);
                    u_var6 = param_4;
                    pu_var5 = param_2;
                    local_12 =
                        pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, local_16, (local_16 >> 0x10));
                    pu_var2 = &local_20;
                    local_10 = u_var4;
                    pass1_1030_bcde(
                        pu_var2,
                        unaff_ss,
                        CONCAT22(u_var4, local_12),
                        pu_var5,
                        u_var6,
                    );
                    if (pu_var2 < 0) {
                        break;
                    }
                    local_18 = pu_var2;
                    if (pu_var2 < 0x1f) {
                        PTR_LOOP_1050_50ca = 0x6ae;
                        return;
                    }
                }
                PTR_LOOP_1050_50ca = 0x6af;
                return;
            }
            PTR_LOOP_1050_50ca = 0x6a8;
        }
    }
    return;
}

// WARNING: Restarted to delay deadcode elimination for space: stack

pub unsafe fn pass1_1030_c652() {
    let in_struct_1: *mut Struct2111;
    let mut in_stack_00000000: u16;

    in_struct_1 =
        process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(in_stack_00000000, 8));
    pass1_1010_9794(in_struct_1);
    return;
}

pub unsafe fn pass1_1030_c668(param_1: &mut Struct44, param_2: u8) -> &mut Struct44 {
    pass1_1028_b418(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1030_c6f6(param_1: *mut Struct763) -> *mut Struct763 {
    let mut u_var1: u16;

    pass1_1028_b354(param_1);
    u_var1 = (param_1 >> 0x10);
    (param_1 + 0x20) = 0;
    param_1.field_0x0 = 0xc940;
    (param_1 + 2) = 0x1030;
    return param_1;
}

pub unsafe fn pass1_1030_c71e(
    param_1: *mut Struct764,
    param_2: *mut Struct764,
    param_3: u16,
    param_3_00: u32,
) -> i32 {
    pass1_1028_b39e(CONCAT22(param_2, param_1), param_3, param_3_00);
    &param_1.field_0x20 = 0;
    CONCAT22(param_2, param_1) = 0xc940;
    param_1.field_0x2 = 0x1030;
    return CONCAT22(param_2, param_1);
}

pub unsafe fn pass1_1030_c74e(param_1: *mut Struct781, param_2: Vec<u8>) {
    pass1_1028_b46e(param_1, param_2);
    (param_1 + 0x20) = 0x70;
    return;
}

pub unsafe fn pass1_1030_c76c(param_1: *mut Struct833) {
    let mut iVar1: i32;
    let local_bx_3: *mut Struct833;
    let mut u_var2: u16;

    u_var2 = (param_1 >> 0x10);
    local_bx_3 = param_1;
    if ((local_bx_3.field_0x12 != 6) && (local_bx_3.field_0x12 != 5)) {
        return;
    }
    iVar1 = local_bx_3.field_0x20;
    if (iVar1 != 0) {
        if (((iVar1 < 0x70) || (SBORROW2(iVar1, 0x70))) || (1 < iVar1 + -0x70)) {
            pass1_1028_be2a();
            return;
        }
    }
    pass1_1028_bdac(param_1, 6);
    return;
}

pub unsafe fn pass1_1030_c7b0(param_1: &mut Struct44) {
    let mut iVar1: i32;
    let mut u_var2: u32;
    let u_var3: u8;
    let paVar4: *mut Struct493;
    let b_var5: bool;
    let extraout_var: u32;
    let mut in_dx: u16;
    let mut i32_var6: i32;
    let mut u_var7: i32;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    u_var7 = (param_1 >> 0x10);
    i32_var6 = param_1;
    iVar1 = (i32_var6 + 0x20);
    if (iVar1 != 0) {
        if (((iVar1 < 0x70) || (SBORROW2(iVar1, 0x70))) || (1 < iVar1 + -0x70)) {
            u_var3 = pass1_1028_b58e((param_1 & 0xffff | u_var7 << 0x10));
            u_var2 = (CONCAT31(extraout_var, u_var3) + 0x2e);
            u_var2 = (u_var2 + 0x200);
            paVar4 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, u_var2, (u_var2 >> 0x10));
            _local_12 = CONCAT22(in_dx, paVar4);
            b_var5 = pass1_1008_c6ae(ctx._PTR_LOOP_1050_06e0, (i32_var6 + 0xc), 0x11);
            pass17_funcs::pass1_1030_23e2(_local_12, b_var5, (i32_var6 + 0x20));
            if (b_var5 != 0) {
                if ((i32_var6 + 0x20) == 1) {
                    pass17_funcs::pass1_1030_25d8(_local_12, 100, (i32_var6 + 0x20));
                    return;
                }
                (i32_var6 + 0x20) = 0x70;
            }
        }
    }
    return;
}

pub unsafe fn pass1_1030_bbe6(param_1: &mut Struct44, param_2: u8) -> &mut Struct44 {
    pass1_1030_b96c(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1030_bc1c() {
    let pu8_var1: Vec<u8>;
    let pc_var2: String;
    let mut cVar3: u8;
    let mut in_dx: u16;
    let mut in_bx: i32;
    let pu_var4: *mut u16;
    let unaff_bp: *mut u16;
    let mut unaff_si: i32;
    let mut unaff_ss: u16;
    let mut in_ZF: bool;
    let mut in_SF: u8;
    let mut in_OF: u8;
    let mut in_stack_0000d730: i32;
    let mut in_stack_0000d732: u16;
    let mut in_stack_0000d734: u16;
    let mut in_stack_0000d736: u32;
    let paStack6: &mut Struct44;

    pu_var4 = &stack0xfffe;
    cVar3 = '\x0f';
    while {
        unaff_bp = unaff_bp + -1;
        pu_var4 = pu_var4 + -1;
        unsafe { *pu_var4 = *unaff_bp };
        cVar3 = cVar3 + -1;
        '\0' < cVar3
    } {}
    if (!in_ZF && in_OF == in_SF) {
        pc_var2 = (in_bx + unaff_si);
        unsafe { *pc_var2 = *pc_var2 - in_dx };
        pass1_1028_b22c(
            CONCAT22(in_stack_0000d732, in_stack_0000d730),
            in_stack_0000d734,
            in_stack_0000d736,
        );
        CONCAT22(in_stack_0000d732, in_stack_0000d730) = 0xbc96;
        (in_stack_0000d730 + 2) = 0x1030;
        return CONCAT22(in_stack_0000d732, in_stack_0000d730);
    }
    pu8_var1 = (in_bx + unaff_si);
    unsafe { *pu8_var1 = *pu8_var1 & in_dx };
    error_check_1000_17ce(paStack6);
    return CONCAT22(in_dx, 1);
}

pub unsafe fn pass1_1030_bc24(param_1: i32, param_2: u16, param_3: u16, param_3_00: u32) {
    pass1_1028_b22c(CONCAT22(param_2, param_1), param_3, param_3_00);
    CONCAT22(param_2, param_1) = 0xbc96;
    (param_1 + 2) = 0x1030;
    return CONCAT22(param_2, param_1);
}

pub unsafe fn pass1_1030_bc4e(param_1: *mut u16) {
    unsafe {
        *param_1 = 0xbc96;
        (param_1 + 2) = 0x1030;
    }
    pass1_1028_b260(param_1);
    return;
}

pub unsafe fn pass1_1030_bc70(param_1: &mut Struct44, param_2: u8) -> &mut Struct44 {
    pass1_1030_bc4e(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1030_bca6() {
    let u_var1: u8;
    let mut c_var2: u8;
    let mut in_dx: u16;
    let pu_var3: *mut u16;
    let unaff_bp: *mut u16;
    let unaff_DI: Vec<u8>;
    let mut unaff_es: u16;
    let mut unaff_ss: u16;

    pu_var3 = &stack0xfffe;
    c_var2 = '\x0f';
    while {
        unaff_bp = unaff_bp + -1;
        pu_var3 = pu_var3 + -1;
        unsafe { *pu_var3 = *unaff_bp };
        c_var2 = c_var2 + -1;
        '\0' < c_var2
    } {}
    u_var1 = _in(in_dx);
    unsafe {
        *unaff_DI = u_var1;
        *0x102e = &stack0xfffe;
    }
    return CONCAT22(0x1036, 0x1034);
}

pub unsafe fn pass1_1030_bcae(param_1: u16, param_2: u16) {
    return CONCAT22(param_2, param_1);
}

pub unsafe fn pass1_1030_bcbc(param_1: u32, param_2: u32, param_3: u32, param_4: u32) {
    pass1_1030_bcde(param_1, (param_1 >> 0x10), param_2, param_3, (param_4 + 4));
    return;
}

pub unsafe fn pass1_1030_bcde(
    param_1: u16,
    param_2: u16,
    param_1_00: *mut Struct966,
    param_2_00: u32,
    param_5: u32,
) -> i32 {
    let local_bx_6: *mut Struct966;
    let mut u_var1: u16;
    let mut unaff_ss: u16;
    let mut local_1c: u16;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u32;
    let mut uStack8: u16;
    let mut local_6: u32;

    u_var1 = (param_1_00 >> 0x10);
    local_bx_6 = param_1_00;
    local_6 = local_bx_6.field_0x8;
    if (local_6 != param_5) {
        return;
    }
    local_c = local_bx_6.field_0xc;
    uStack8 = local_bx_6.field_0x10;
    pass1_1008_3e94(
        param_2_00,
        CONCAT22(unaff_ss, &local_10),
        CONCAT22(unaff_ss, &local_e),
    );
    pass1_1008_3e94(
        &local_c,
        CONCAT22(unaff_ss, &local_14),
        CONCAT22(unaff_ss, &local_12),
    );
    xor_1000_49b2();
    xor_1000_49b2();
    return;
}

pub unsafe fn pass1_1030_bd74(
    param_1: u16,
    param_2: u16,
    param_1_00: *mut Struct968,
    param_2_00: *mut Struct967,
) -> i32 {
    let local_bx_6: *mut Struct967;
    let local_bx_18: *mut Struct968;
    let mut u_var1: u16;
    let mut local_es_18: u16;
    let mut unaff_ss: u16;
    let mut local_2e: u16;
    let mut local_26: u16;
    let mut local_22: u16;
    let mut local_20: u16;
    let mut local_1e: u16;
    let mut local_1c: u16;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_16: u32;
    let mut uStack18: u16;
    let mut local_10: u32;
    let mut u_stack12: u16;
    let mut local_a: u32;
    let mut local_6: u32;

    u_var1 = (param_2_00 >> 0x10);
    local_bx_6 = param_2_00;
    local_6 = local_bx_6.field_0x8;
    local_es_18 = (param_1_00 >> 0x10);
    local_bx_18 = param_1_00;
    local_a = local_bx_18.field_0x8;
    if (local_a != local_6) {
        return;
    }
    local_10 = local_bx_6.field_0xc;
    u_stack12 = local_bx_6.field_0x10;
    local_16 = local_bx_18.field_0xc;
    uStack18 = local_bx_18.field_0x10;
    pass1_1008_3e94(
        &local_10,
        CONCAT22(unaff_ss, &local_1a),
        CONCAT22(unaff_ss, &local_18),
    );
    pass1_1008_3e94(
        &local_16,
        CONCAT22(unaff_ss, &local_1e),
        CONCAT22(unaff_ss, &local_1c),
    );
    xor_1000_49b2();
    xor_1000_49b2();
    return;
}

pub unsafe fn pass1_1030_be34(param_1: *mut Struct763) -> *mut Struct763 {
    pass1_1028_b354(param_1);
    param_1.field_0x0 = 0xc006;
    (param_1 + 2) = 0x1030;
    return param_1;
}

pub unsafe fn pass1_1030_be56(param_1: i32, param_2: u16, param_3: u16, param_3_00: u32) {
    pass1_1028_b39e(CONCAT22(param_2, param_1), param_3, param_3_00);
    CONCAT22(param_2, param_1) = 0xc006;
    (param_1 + 2) = 0x1030;
    return CONCAT22(param_2, param_1);
}

pub unsafe fn pass1_1030_be80(param_1: &mut Struct44) {
    let piVar1: *mut i32;
    let mut u_var2: u32;
    let u_var3: u8;
    let mut u_var4: u16;
    let extraout_var: u32;
    let pu_var5: Vec<u8>;
    let mut in_dx: u16;
    
    let local_bx_13: *mut Struct969;
    let mut u_var6: u16;
    let mut i_var7: i32;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    pass1_1028_bf22(param_1);
    u_var6 = (param_1 >> 0x10);
    local_bx_13 = param_1;
    if (local_bx_13.field_0x12 == 5) {
        u_var2 = local_bx_13.field_0x14;
        (u_var2 + 0xa4) = 0x1e;
        u_var2 = local_bx_13.field_0x14;
        (u_var2 + 0xac) = 1;
        i_var7 = local_bx_13.field_0xc;
        if (i_var7 == 0x1b) {
            u_var2 = local_bx_13.field_0x14;
            (u_var2 + 0xaa) = 10;
        } else {
            if (i_var7 == 0x1c) {
                u_var2 = local_bx_13.field_0x14;
                (u_var2 + 0xaa) = 0xb;
            } else {
                if (i_var7 == 0x1d) {
                    u_var2 = local_bx_13.field_0x14;
                    (u_var2 + 0xaa) = 0xc;
                }
            }
        }
        u_var3 = pass1_1028_b58e(param_1);
        pu_var5 = *(CONCAT31(extraout_var, u_var3) + 0x2e);
        i_var7 = 0xc;
        pass1_1038_3fb0(pu_var5);
        u_var4 = pass17_funcs::pass1_1030_25b2(pu_var5 & 0xffff | ctx.dx_reg << 0x10, i_var7);
        if (u_var4 != 0) {
            u_var2 = local_bx_13.field_0x14;
            piVar1 = (u_var2 + 0xaa);
            unsafe { *piVar1 = *piVar1 + 1 };
        }
        u_var4 = pass17_funcs::pass1_1030_25b2(pu_var5 & 0xffff | ctx.dx_reg << 0x10, 0xe);
        if (u_var4 != 0) {
            u_var2 = local_bx_13.field_0x14;
            piVar1 = (u_var2 + 0xaa);
            unsafe { *piVar1 = *piVar1 + 1 };
        }
        u_var4 = pass17_funcs::pass1_1030_25b2(pu_var5 & 0xffff | ctx.dx_reg << 0x10, 0x76);
        if (u_var4 != 0) {
            u_var2 = local_bx_13.field_0x14;
            piVar1 = (u_var2 + 0xaa);
            unsafe { *piVar1 = *piVar1 + 1 };
        }
    }
    return;
}

pub unsafe fn pass1_1030_bf6e(param_1: &mut Struct44) {
    let pu_var1: *mut u32;
    let u_var2: u8;
    let extraout_AH: u8;
    let mut in_dx: u16;
    let local_bx_36: *mut Struct970;
    let mut u_var4: u16;
    let mut u_var5: u16;
    let mut local_6: u16;
    let mut local_4: u16;
    let mut temp_5ff0eedf2f: u32;
    let mut i_var3: i32;

    u_var5 = 0x1e;
    u_var2 = pass1_1028_b58e(param_1);
    i_var3 = CONCAT11(extraout_AH, u_var2);
    _local_6 = CONCAT22(in_dx, i_var3);
    pass17_funcs::pass1_1030_7c28(CONCAT22(in_dx, i_var3), u_var5);
    temp_5ff0eedf2f = (param_1 + 0x14);
    u_var4 = (temp_5ff0eedf2f >> 0x10);
    local_bx_36 = temp_5ff0eedf2f;
    pu_var1 = &local_bx_36.field_0xaa;
    let pu_var1_val = unsafe { *pu_var1 };
    pass17_funcs::pass1_1030_7ddc(
        _local_6,
        (((1000 - i_var3) - pu_var1_val & -(1000 - i_var3 < pu_var1_val)) + local_bx_36.field_0xaa),
        0x1e,
    );
    return;
}

pub unsafe fn pass1_1030_bfb8(param_1: &mut Struct44) {
    let u_var1: u8;
    let extraout_AH: u8;
    let mut in_dx: i32;
    let mut u_var3: u16;
    let mut u_var2: i32;

    u_var3 = 0x1e;
    u_var1 = pass1_1028_b58e(param_1);
    u_var2 = CONCAT11(extraout_AH, u_var1);
    pass17_funcs::pass1_1030_7c28(CONCAT22(in_dx, u_var2), u_var3);
    return CONCAT22(-(1000 < u_var2) - in_dx, 1000 - u_var2);
}

pub unsafe fn pass1_1030_bfe0(param_1: &mut Struct44, param_2: u8) -> &mut Struct44 {
    pass1_1028_b418(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1030_c06e(param_1: *mut Struct763) {
    let local_bx_12: *mut Struct763;
    let mut u_var1: u16;

    pass1_1028_b354(param_1);
    u_var1 = (param_1 >> 0x10);
    local_bx_12 = param_1;
    local_bx_12.field_0x20 = 0;
    &local_bx_12.field_0x24 = 0;
    param_1.field_0x0 = 0xc68e;
    local_bx_12.field_0x2 = 0x1030;
    return;
}

pub unsafe fn pass1_1030_c09c(param_1: i32, param_2: u16, param_3: u16, param_3_00: u32) {
    pass1_1028_b39e(CONCAT22(param_2, param_1), param_3, param_3_00);
    (param_1 + 0x20) = 0;
    (param_1 + 0x24) = 0;
    CONCAT22(param_2, param_1) = 0xc68e;
    (param_1 + 2) = 0x1030;
    return;
}

pub unsafe fn pass1_1030_c0d2(param_1: u32) {
    if (0 < (param_1 + 0x24)) {
        return 1;
    }
    return 0;
}

pub unsafe fn pass1_1030_c0ec(param_1: u32) {
    let mut u_var1: u16;

    u_var1 = (param_1 >> 0x10);
    if (((param_1 + 0xc) != 0xb) && ((param_1 + 0x24) < 1)) {
        return 0;
    }
    return 1;
}

pub unsafe fn pass1_1030_c10e(param_1: u32) {
    let piVar1: *mut i32;
    let local_bx_3: *mut Struct972;
    let mut u_var2: u16;

    u_var2 = (param_1 >> 0x10);
    local_bx_3 = param_1;
    if (0 < local_bx_3.field_0x24) {
        piVar1 = &local_bx_3.field_0x24;
        unsafe { *piVar1 = *piVar1 + -1 };
        return;
    }
    local_bx_3.field_0xc = 0x37;
    return;
}

pub unsafe fn pass1_1030_c12e(param_1: &mut Struct44, param_2: i32) {
    let piVar1: *mut i32;
    let mut u_var2: u32;
    let mut u_var3: u32;
    let u_var4: u8;
    let extraout_var: u32;
    let mut in_dx: i32;
    let local_bx_31: *mut Struct973;
    let mut u_var5: u16;
    let mut u_var6: u16;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    u_var4 = pass1_1028_b58e(param_1);
    u_var3 = CONCAT31(extraout_var, u_var4) & 0xffff | in_dx << 0x10;
    u_var2 = (CONCAT31(extraout_var, u_var4) + 0x2e);
    u_var5 = (param_1 >> 0x10);
    local_bx_31 = param_1;
    if (local_bx_31.field_0x24 < 1) {
        pass17_funcs::pass1_1030_7d1c(u_var3, 0, 0x230000);
    } else {
        if (param_2 == 0) {
            u_var6 = 0;
        } else {
            u_var6 = 0x32;
        }
        pass17_funcs::pass1_1030_7d1c(u_var3, u_var6, 0x230000);
        piVar1 = &local_bx_31.field_0x24;
        unsafe { *piVar1 = *piVar1 + -1 };
    }
    if ((0 < local_bx_31.field_0x24) && (local_bx_31.field_0x24 < 0x19)) {
        (u_var2 + 0x1fe) = 1;
    }
    return;
}

pub unsafe fn pass1_1030_c1b2(param_1: *mut Struct974) {
    let mut iVar1: i32;
    let ppVar2: *mut Struct2111;
    let local_16: *mut Struct974;
    let local_14: *mut Struct974;
    let mut uStack18: u16;

    local_16 = param_1;
    local_14 = (param_1 >> 0x10);
    pass1_1028_be9e(param_1);
    if (local_16.field_0x12 == 5) {
        if (local_16.field_0xc == 0xb) {
            pass1_1030_c652(param_1);
            ppVar2 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, 0x820008);
            iVar1 = pass1_1010_9f8c(ppVar2, 0x82);
            local_16.field_0x24 = iVar1 * 3;
            process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(uStack18, 2));
            if (u16_1050_13ae < 3) {
                iVar1 = local_16.field_0x24;
                if (iVar1 < 0x32) {
                    iVar1 = 0x32;
                }
                local_16.field_0x24 = iVar1;
                return;
            }
        } else {
            local_16.field_0x24 = 100;
        }
    }
    return;
}

pub unsafe fn pass1_1030_b90c(param_1: &mut Struct44, param_2: u8) -> &mut Struct44 {
    pass1_1030_afa6(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1030_b936(param_1: *mut Struct963, param_2: u32, param_3: u32) {
    let mut u_var1: u16;

    u_var1 = param_2;
    pass1_1028_b22c(CONCAT22(u_var1, param_1), (param_2 >> 0x10), param_3);
    param_1.field_0xe = 0;
    param_1.field_0x12 = 0;
    CONCAT22(u_var1, param_1) = 0xbc0c;
    param_1.field_0x2 = 0x1030;
    return;
}

pub unsafe fn pass1_1030_b96c(param_1: &mut Struct44) {
    let mut u_var1: i32;
    let in_struct_1: &mut Struct44;
    let local_bx_4: *mut Struct964;
    let mut u_var2: u16;
    let mut local_6: u32;

    u_var2 = (param_1 >> 0x10);
    local_bx_4 = param_1;
    param_1.ptr_a_lo = 0xbc0c;
    local_bx_4.field_0x2 = 0x1030;
    in_struct_1 = &local_bx_4.field_0xe;
    u_var1 = local_bx_4.field_0x10;
    if ((u_var1 | in_struct_1) != 0) {
        pass1_1020_ba7e((in_struct_1 & 0xffff | u_var1 << 0x10));
        error_check_1000_17ce(in_struct_1);
    }
    pass1_1028_b260(param_1);
    return;
}

pub unsafe fn pass1_1030_b9b2(param_1: u32) {
    let mut u_var1: u16;
    let mut local_6: u32;

    u_var1 = (param_1 >> 0x10);
    (param_1 + 0xe) = 0;
    (param_1 + 0x12) = 0;
    return;
}

pub unsafe fn pass1_1030_b718(param_1: u16, param_2: u16, param_1_00: u32, param_2_00: *mut u32) {
    let pu_var1: *mut u32;
    
    let mut unaff_ss: u16;
    let mut in_stack_0000ffec: u32;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    _local_6 = process_struct_1010_20ba(
        ctx._g_Struct372_1050_0ed0,
        CONCAT22((in_stack_0000ffec >> 0x10), 0x2f),
    );
    local_a = (_local_6 + 0x20);
    pu_var1 = &stack0xffee;
    pass17_funcs::pass1_1030_64ce(
        _PTR_LOOP_1050_5740,
        param_1_00,
        local_a,
        (local_a >> 0x10),
        pu_var1,
        unaff_ss,
    );
    unsafe { *param_2_00 = *pu_var1 };
    return;
}

pub unsafe fn pass1_1030_9ecc(param_1: *mut u32, param_2: u32) {
    let mut u_var1: u16;

    u_var1 = (param_1 >> 0x10);
    unsafe {
        *param_1 = 0;
        (param_1 + 4) = param_2;
        (param_1 + 8) = 0;
    }
    return;
}

pub unsafe fn pass1_1030_9ef2(param_1: *mut *mut Struct493) {
    let mut iVar1: i32;
    let mut u_var2: u16;
    let mut u_var3: u32;
    let mut local_6: u16;
    let mut local_4: u16;
    let param_1_val = unsafe { *param_1 };
    if (param_1_val != 0x0) {
        u_var3 = pass17_funcs::pass1_1030_73a8(param_1_val);
        u_var2 = (u_var3 >> 0x10);
        iVar1 = (u_var3 + 0xc);
        if (((iVar1 != 5) && (iVar1 != 9)) && ((u_var3 + 0x12) < 5)) {
            return 0;
        }
        pass1_1030_9f64(param_1);
    }
    return 1;
}

pub unsafe fn pass1_1030_9f40(param_1: u32, param_2: u16) {
    let mut u_var1: u16;

    u_var1 = pass1_1008_c646(
        ctx._PTR_LOOP_1050_06e0,
        CONCAT22(param_2, (ctx._PTR_LOOP_1050_06e0 >> 0x10)),
    );
    (param_1 + 8) = u_var1;
    pass1_1030_9f7a(param_1, u_var1);
    return;
}

pub unsafe fn pass1_1030_9f64(param_1: *mut u32) {
    (param_1 + 8) = 0;
    unsafe { *param_1 = 0 };
    return;
}

pub unsafe fn pass1_1030_9f7a(param_1: *mut u16, param_2: u16) {
    let mut u_var1: u32;
    let u_var2: u8;
    let BVar3: bool;
    let pu_var4: *mut u32;
    let extraout_var: u32;
    
    let mut u_var5: u16;
    let mut u_var6: u16;
    let mut u_var7: u16;
    let mut unaff_ss: u16;
    let mut local_148: u16;
    let mut local_134: u32;
    let mut local_130: u16;
    let mut local_12e: u16;
    let mut local_10: u32;
    let mut local_c: u32;
    let mut local_8: u32;
    let mut local_4: u16;

    zero_list_1008_3e38(CONCAT22(unaff_ss, &local_8));
    BVar3 = pass1_1008_c6ae(ctx._PTR_LOOP_1050_06e0, param_2, 0x28);
    if (BVar3 != 0) {
        local_4 = 1;
    }
    pu_var4 = &local_8;
    pass1_1030_a278(param_1, CONCAT22(unaff_ss, pu_var4));
    if (pu_var4 != 0x0) {
        u_var7 = (param_1 >> 0x10);
        u_var6 = param_1;
        u_var1 = (u_var6 + 4);
        local_c = (u_var1 + 8);
        u_var1 = (u_var6 + 4);
        pass1_1028_87f0(
            CONCAT22(unaff_ss, &local_130),
            0,
            0,
            param_2,
            &local_8,
            unaff_ss,
            (u_var1 + 4),
            local_c,
        );
        pass17_funcs::pass1_1030_835a(ctx._g_bool_1050_5748, CONCAT22(unaff_ss, &local_130));
        u_var5 = ctx.dx_reg;
        u_var2 = pass1_1028_b58e(local_10);
        unsafe { *param_1 = CONCAT31(extraout_var, u_var2) };
        (u_var6 + 2) = u_var5;
        if (0 < local_4) {
            pass1_1030_a044(u_var6, u_var7, CONCAT22(unaff_ss, &local_8), local_c);
        }
    }
    return;
}

pub unsafe fn pass1_1030_a044(param_1: u16, param_2: u16, param_1_00: *mut u16, param_2_00: u32) {
    let pp_var1: fn();
    let paVar2: *mut Struct493;
    let pu_var3: Vec<u8>;
    let mut i_var4: i32;
    let mut i_var5: i32;
    
    
    let mut u_var6: i32;
    
    let mut u_var7: u16;
    let mut unaff_ss: u16;
    let pu_var8: *mut u32;
    let mut u_var9: u16;
    let u_var10: u8;
    let u_var11: u8;
    let mut u_var12: u16;
    let mut local_17e: u16;
    let mut local_17c: u16;
    let mut local_5a: u16;
    let mut local_4e: u16;
    let mut local_46: u16;
    let mut local_44: u16;
    let mut local_42: u32;
    let mut local_3e: u16;
    let mut local_3c: u16;
    let mut local_3a: [u8; 12];
    let mut local_2e: [u8; 6];
    let mut local_28: u16;
    let mut local_26: u16;
    let mut local_24: u16;
    let mut local_22: u16;
    let mut local_20: u32;
    let mut local_1c: u32;
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

    local_14 = &local_8;
    pass1_1008_3eb4(
        param_1_00,
        CONCAT22(unaff_ss, local_14),
        CONCAT22(unaff_ss, &local_6),
        CONCAT22(unaff_ss, &local_4),
    );
    pass17_funcs::pass1_1030_627e(_PTR_LOOP_1050_5740, param_1_00, param_2_00);
    local_26 = ctx.dx_reg;
    local_12 = ctx.dx_reg;
    paVar2 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, local_14, ctx.dx_reg);
    _local_18 = CONCAT22(local_26, paVar2);
    local_1c = &paVar2[1].field_0x10;
    local_20 = (local_1c + 4);
    local_28 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, param_2_00, (param_2_00 >> 0x10));
    pu_var8 = pass17_funcs::pass1_1030_5b5c(local_28, local_26);
    unsafe {
        local_2e._0_4_ = *pu_var8;
        local_2e._4_2_ = (pu_var8 + 4);
    }
    local_4e = local_2e;
    pass1_1008_3e94(
        local_2e,
        CONCAT22(unaff_ss, &local_24),
        CONCAT22(unaff_ss, &local_22),
    );
    i_var4 = local_4 + 1;
    _local_c = CONCAT22(local_4 - 1, local_6 - 1);
    i_var5 = local_6 + 1;
    if ((local_4 - 1) < 0) {
        _local_c = (local_6 - 1);
    }
    if (local_22 <= i_var4) {
        i_var4 = local_22 - 1;
    }
    if (local_c < 0) {
        _local_c = _local_c & 0xffff0000;
    }
    if (local_24 <= i_var5) {
        i_var5 = local_24 - 1;
    }
    _local_10 = CONCAT22(i_var4, i_var5);
    zero_list_1008_6c90(local_3a, unaff_ss);
    u_var7 = SUB42(&ctx.PTR_LOOP_1050_1008, 0);
    pass1_1008_6cec(
        CONCAT22(unaff_ss, local_3a),
        local_8,
        _local_10,
        local_8,
        _local_c,
    );
    pu_var3 = local_3a;
    u_var6 = ctx.dx_reg;
    pass17_funcs::pass1_1030_6522(_PTR_LOOP_1050_5740, CONCAT22(unaff_ss, pu_var3), param_2_00);
    _local_3e = CONCAT22(u_var6, pu_var3);
    if ((u_var6 | pu_var3) != 0) {
        local_42 = 0;
        local_44 = 0;
        local_46 = local_c;
        while (local_46 <= local_10) {
            local_4e = local_a;
            while (local_4e <= local_e) {
                pp_var1 = (*_local_3e + 4);
                u_var9 = local_44;
                local_44 = local_44 + 1;
                (**pp_var1)(u_var7, _local_3e, (_local_3e >> 0x10));
                local_42 = CONCAT22(ctx.dx_reg, u_var9);
                local_42._3_1_ = (ctx.dx_reg >> 8);
                if (local_42._3_1_ == '\0') {
                    local_5a = u_var9;
                    if (u_var9 == 7) {
                        pass1_1008_3e76(param_1_00, local_8, local_46, local_4e);
                        u_var10 = local_20;
                        u_var11 = (local_20 >> 8);
                        u_var12 = (local_20 >> 0x10);
                        u_var9 = 6;
                    } else {
                        if (u_var9 == 8) {
                            pass1_1008_3e76(param_1_00, local_8, local_46, local_4e);
                            u_var10 = local_20;
                            u_var11 = (local_20 >> 8);
                            u_var12 = (local_20 >> 0x10);
                            u_var9 = 7;
                        } else {
                            if (u_var9 != 9) {}
                            // goto LAB_1030_a1d0;
                            pass1_1008_3e76(param_1_00, local_8, local_46, local_4e);
                            u_var10 = local_20;
                            u_var11 = (local_20 >> 8);
                            u_var12 = (local_20 >> 0x10);
                            u_var9 = 8;
                        }
                    }
                    u_var7 = SUB42(&PTR_LOOP_1050_1028, 0);
                    pass1_1028_87f0(
                        CONCAT22(unaff_ss, &local_17e),
                        0,
                        0,
                        u_var9,
                        param_1_00,
                        (param_1_00 >> 0x10),
                        CONCAT22(u_var12, CONCAT11(u_var11, u_var10)),
                        param_2_00,
                    );
                    pass17_funcs::pass1_1030_835a(ctx._g_bool_1050_5748, CONCAT22(unaff_ss, &local_17e));
                    local_17e = ctx.s_1_1050_389a;
                    local_17c = &ctx.PTR_LOOP_1050_1008;
                }
                // LAB_1030_a1d0:
                local_4e = local_4e + 1;
            }
            local_46 = local_46 + 1;
        }
    }
    return;
}

pub unsafe fn pass1_1030_a278(param_1: *mut u16, param_2: u32) {
    let mut u_var1: u32;
    let u_var2: u8;
    
    let extraout_var: u32;
    
    let mut u_var3: u16;
    let local_bx_46: *mut Struct951;
    let local_bx_149: *mut Struct952;
    let mut u_var4: i32;
    let mut u_var5: u16;
    let mut unaff_ss: u16;
    let mut local_134: u16;
    let mut local_132: u16;
    let struct_a: &mut Struct44;
    let mut local_10: u32;
    let mut local_c: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    local_4 = 1;
    pass1_1030_a39a(param_1, param_2);
    if (in_ax != 0) {
        return;
    }
    local_6 = in_ax;
    pass1_1030_a3ae(param_1, param_2);
    local_bx_46 = param_2;
    u_var4 = (param_2 >> 0x10);
    if (in_ax == 0) {
        pass1_1030_a57e(param_1, param_2);
        if (in_ax == 0) {
            pass1_1030_a844(param_1, param_2);
            if (in_ax == 0) {
                local_4 = 0;
                // goto LAB_1030_a305;
            }
            u_var5 = local_bx_46.field_0x4;
        } else {
            u_var5 = local_bx_46.field_0x4;
        }
        if (u_var5 < 1) {
            local_6 = 0x73;
        } else {
            local_6 = 0x77;
        }
    } else {
        if (local_bx_46.field_0x4 < 1) {
            local_6 = 0x7a;
        } else {
            local_6 = 0x7f;
        }
    }
    // LAB_1030_a305:
    if (local_6 != 0) {
        u_var5 = (param_1 >> 0x10);
        local_bx_149 = param_1;
        u_var1 = local_bx_149.field_0x4;
        local_10 = (u_var1 + 8);
        u_var1 = local_bx_149.field_0x4;
        pass1_1028_87f0(
            CONCAT22(unaff_ss, &local_134),
            0,
            0,
            local_6,
            local_bx_46,
            u_var4,
            (u_var1 + 4),
            local_10,
        );
        pass17_funcs::pass1_1030_835a(ctx._g_bool_1050_5748, CONCAT22(unaff_ss, &local_134));
        local_c = struct_a;
        u_var3 = ctx.dx_reg;
        u_var2 = pass1_1028_b58e(struct_a);
        unsafe { *param_1 = CONCAT31(extraout_var, u_var2) };
        local_bx_149.field_0x2 = u_var3;
        if (0 < local_bx_46.field_0x4) {
            pass1_1030_a044(
                local_bx_149,
                u_var5,
                (param_2 & 0xffff | u_var4 << 0x10),
                local_10,
            );
        }
    }
    return;
}

pub unsafe fn pass1_1030_a39a(param_1: u32, param_2: u32) {
    pass1_1030_aa18(param_1, param_2);
    return;
}

pub unsafe fn pass1_1030_a3ae(param_1: u32, param_2: *mut u16) {
    let pp_var1: fn();
    let mut u_var2: u32;
    let mut u_var3: u16;
    let b_var4: bool;
    let pu_var5: Vec<u8>;
    let mut u_var6: u32;
    let mut in_dx: i32;
    
    
    
    let mut u_var7: u16;
    let mut i_var8: i32;
    let mut u_var9: u16;
    let mut u_var10: u16;
    let mut unaff_ss: u16;
    let mut u_var11: u16;
    let mut u_var12: u16;
    let mut local_4e: u32;
    let mut local_4a: u32;
    let mut local_46: u16;
    let mut local_3c: u16;
    let mut local_30: u16;
    let mut local_2e: u16;
    let mut local_2c: u32;
    let mut local_28: u16;
    let mut local_26: u16;
    let mut local_24: u16;
    let mut local_22: [u8; 6];
    let mut local_1c: u16;
    let mut local_1a: u16;
    let mut local_16: u32;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    local_4 = 0;
    local_6 = (param_2 + 4);
    pu_var5 = pass1_1008_c6fa(ctx._PTR_LOOP_1050_06e0, 0x45);
    u_var3 = pu_var5;
    u_var9 = (param_1 >> 0x10);
    u_var7 = param_1;
    local_a = u_var3;
    pass1_1038_4e78((u_var7 + 4), pu_var5 & 0xffff | in_dx << 0x10);
    _local_e = CONCAT22(ctx.dx_reg, u_var3);
    pp_var1 = (*_local_e + 0x10);
    u_var11 = u_var3;
    u_var12 = ctx.dx_reg;
    (**pp_var1)(&PTR_LOOP_1050_1038, u_var3, ctx.dx_reg);
    _local_12 = CONCAT22(ctx.dx_reg, u_var3);
    u_var2 = (u_var7 + 4);
    local_16 = (u_var2 + 8);
    zero_list_1008_3e38(CONCAT22(unaff_ss, &local_1c));
    zero_list_1008_3e38(CONCAT22(unaff_ss, local_22));
    local_2c = 0;
    loop {
        if (_local_12 <= local_2c) {
            // LAB_1030_a4e7:
            if (_local_e != 0x0) {
                pp_var1 = *_local_e;
                (**pp_var1)(
                    &ctx.PTR_LOOP_1050_1008,
                    _local_e,
                    (_local_e >> 0x10),
                    1,
                    u_var11,
                    u_var12,
                    _local_e,
                    _local_e,
                );
            }
            return;
        }
        u_var6 = _local_12;
        pass17_funcs::pass1_1030_1d58(_local_e);
        if ((ctx.dx_reg | u_var6) != 0) {
            modify_list_1008_3f62(
                CONCAT22(unaff_ss, &local_1c),
                CONCAT22(ctx.dx_reg, u_var6 + 0xc),
            );
            pass1_1008_3eb4(
                CONCAT22(unaff_ss, &local_1c),
                CONCAT22(unaff_ss, &local_28),
                CONCAT22(unaff_ss, &local_26),
                CONCAT22(unaff_ss, &local_24),
            );
            if ((local_28 == local_6)
                && (
                    u_var2 = (u_var7 + 4),
                    u_var10 = (u_var2 >> 0x10),
                    i_var8 = u_var2,
                    u_var2 = (i_var8 + 4),
                    u_var3 = pass1_1030_addc(
                        u_var7,
                        u_var9,
                        CONCAT22(unaff_ss, &local_1c),
                        u_var2,
                        (u_var2 >> 0x10),
                        (i_var8 + 8),
                    ),
                    u_var3 != 0,
                ))
            {
                modify_list_1008_3f62(CONCAT22(unaff_ss, local_22), CONCAT22(unaff_ss, &local_1c));
                local_1a = local_26 - 1;
                b_var4 = pass1_1030_ad22(u_var7, u_var9, CONCAT22(unaff_ss, &local_1c), local_16);
                if (b_var4 == 0) {
                    local_1a = local_26 + 1;
                    b_var4 =
                        pass1_1030_ad22(u_var7, u_var9, CONCAT22(unaff_ss, &local_1c), local_16);
                    if (b_var4 == 0) {
                        local_1a = local_26;
                        local_1c = local_24 - 1;
                        b_var4 = pass1_1030_ad22(
                            u_var7,
                            u_var9,
                            CONCAT22(unaff_ss, &local_1c),
                            local_16,
                        );
                        if (b_var4 == 0) {
                            local_1c = local_24 + 1;
                            b_var4 = pass1_1030_ad22(
                                u_var7,
                                u_var9,
                                CONCAT22(unaff_ss, &local_1c),
                                local_16,
                            );
                            if (b_var4 == 0) {}
                            // goto LAB_1030_a45b;
                        }
                    }
                }
                modify_list_1008_3f62(param_2, CONCAT22(unaff_ss, local_22));
                local_4 = 1;
                // goto LAB_1030_a4e7;
            }
        }
        // LAB_1030_a45b:
        local_2c = local_2c + 1;
    }
}

pub unsafe fn pass1_1030_a57e(param_1: u32, param_2: *mut u16) {
    let mut u_var1: u32;
    let ppc_var2: fn();
    let mut u_var3: u32;
    let mut u_var4: u32;
    let mut in_ax: i32;
    let mut u_var5: u16;
    let pu_var6: *mut u16;
    let pu_var7: Vec<u8>;
    let mut u_var8: u32;
    let mut in_dx: i32;
    
    
    
    let local_bx_11: *mut Struct953;
    let mut u_var9: u16;
    let mut iVar10: i32;
    let pu_var11: *mut u32;
    let mut u_var12: u16;
    let mut u_var13: u16;
    let mut u_var14: u16;
    let mut u_var15: u16;
    let mut unaff_ss: u16;
    let mut u_var16: u32;
    let u_var17: u8;
    let mut local_4e: u32;
    let mut local_4a: u32;
    let mut local_46: u16;
    let mut local_38: u16;
    let mut local_36: u16;
    let mut local_2c: u16;
    let mut local_2a: u16;
    let mut local_28: u32;
    let mut local_24: u16;
    let mut local_22: u16;
    let mut local_20: u32;
    let mut local_1c: [u8; 2];
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    local_4 = 0;
    u_var12 = (param_1 >> 0x10);
    local_bx_11 = param_1;
    pass1_1038_53ba(local_bx_11.field_0x4, (&ctx.PTR_LOOP_1050_0000 + 1));
    if ((in_dx != 0) || (in_ax != 0)) {
        local_6 = (param_2 + 4);
        local_8 = 8 - (local_6 == 0);
        pu_var7 = pass1_1008_c6fa(ctx._PTR_LOOP_1050_06e0, local_8);
        u_var5 = pu_var7;
        local_c = u_var5;
        pass1_1038_4e78(local_bx_11.field_0x4, pu_var7 & 0xffff | in_dx << 0x10);
        _local_10 = CONCAT22(ctx.dx_reg, u_var5);
        u_var15 = SUB42(&ctx.PTR_LOOP_1050_1008, 0);
        zero_list_1008_3e38(CONCAT22(unaff_ss, &local_16));
        u_var3 = local_bx_11.field_0x4;
        u_var1 = (u_var3 + 8);
        u_var13 = (_local_10 >> 0x10);
        u_var9 = SUB42(_local_10, 0);
        ppc_var2 = (*_local_10 + 0x10);
        u_var8 = u_var1;
        ppc_var2(&ctx.PTR_LOOP_1050_1008, u_var9, u_var13);
        u_var4 = u_var8 & 0xffff | ctx.dx_reg << 0x10;
        local_28 = 0;
        while (local_28 < u_var4) {
            u_var16 = u_var4;
            pass17_funcs::pass1_1030_1d58(_local_10);
            if ((ctx.dx_reg | u_var16) != 0) {
                modify_list_1008_3f62(
                    CONCAT22(unaff_ss, &local_16),
                    CONCAT22(ctx.dx_reg, u_var16 + 0xc),
                );
                u_var15 = SUB42(&ctx.PTR_LOOP_1050_1008, 0);
                pass1_1008_3eb4(
                    CONCAT22(unaff_ss, &local_16),
                    CONCAT22(unaff_ss, local_1c),
                    CONCAT22(unaff_ss, &local_1a),
                    CONCAT22(unaff_ss, &local_18),
                );
                u_var3 = local_bx_11.field_0x4;
                u_var14 = (u_var3 >> 0x10);
                iVar10 = u_var3;
                u_var3 = (iVar10 + 4);
                u_var5 = pass1_1030_addc(
                    local_bx_11,
                    u_var12,
                    CONCAT22(unaff_ss, &local_16),
                    u_var3,
                    (u_var3 >> 0x10),
                    (iVar10 + 8),
                );
                if (u_var5 == 0) {}
                // goto LAB_1030_a660;
                u_var16 = pass17_funcs::pass1_1030_73a8((u_var16 & 0xffff | ctx.dx_reg << 0x10));
                iVar10 = (u_var16 + 0xc);
                if (5 < iVar10 - 0x7a) {}
                // goto LAB_1030_a660;
                u_var15 = 0x1030;
                match (iVar10) {
                    // default:
                    _ => {
                        local_14 = local_1a - 1;
                        pu_var6 = &local_16;
                        pass1_1030_ad86(local_bx_11, u_var12, CONCAT22(unaff_ss, pu_var6), u_var1);
                        if (pu_var6 != 0x0) {}
                        // goto LAB_1030_a7df;
                        local_14 = local_1a + 1;
                        pu_var6 = &local_16;
                        pass1_1030_ad86(local_bx_11, u_var12, CONCAT22(unaff_ss, pu_var6), u_var1);
                        if (pu_var6 == 0x0) {
                            local_14 = local_1a;
                            local_16 = local_18 - 1;
                            pu_var6 = &local_16;
                            pass1_1030_ad86(
                                local_bx_11,
                                u_var12,
                                CONCAT22(unaff_ss, pu_var6),
                                u_var1,
                            );
                            // goto joined_r0x1030a722;
                        }
                        // LAB_1030_a748:
                        modify_list_1008_3f62(param_2, CONCAT22(unaff_ss, &local_16));
                    }
                    0x7b | 0x7e => {
                        local_14 = local_1a - 1;
                        pu_var6 = &local_16;
                        pass1_1030_ad86(local_bx_11, u_var12, CONCAT22(unaff_ss, pu_var6), u_var1);
                        if (pu_var6 == 0x0) {
                            local_14 = local_1a + 1;
                            // goto LAB_1030_a730;
                        }
                        modify_list_1008_3f62(param_2, CONCAT22(unaff_ss, &local_16));
                        if (_local_10 == 0x0) {
                            return;
                        }
                        u_var15 = (_local_10 >> 0x10);
                        pu_var11 = _local_10;
                        u_var17 = (_local_10 >> 0x10);
                        // goto LAB_1030_a6ea;
                    }
                    0x7c | 0x7d => {
                        local_16 = local_18 - 1;
                        pu_var6 = &local_16;
                        pass1_1030_ad86(local_bx_11, u_var12, CONCAT22(unaff_ss, pu_var6), u_var1);
                        // joined_r0x1030a722:
                        if (pu_var6 == 0x0) {
                            local_16 = local_18 + 1;
                            // LAB_1030_a730:
                            pu_var6 = &local_16;
                            pass1_1030_ad86(
                                local_bx_11,
                                u_var12,
                                CONCAT22(unaff_ss, pu_var6),
                                u_var1,
                            );
                            if (pu_var6 != 0x0) {}
                            // goto LAB_1030_a748;
                            // goto LAB_1030_a660;
                        }
                        // LAB_1030_a7df:
                        modify_list_1008_3f62(param_2, CONCAT22(unaff_ss, &local_16));

                        pu_var11 = _local_10;
                        if ((local_e | pu_var11) != 0) {
                            u_var15 = (_local_10 >> 0x10);
                            u_var17 = (_local_10 >> 0x10);
                            // LAB_1030_a6ea:
                            unsafe {
                                ppc_var2 = *pu_var11;
                                ppc_var2(
                                    &ctx.PTR_LOOP_1050_1008,
                                    pu_var11,
                                    u_var17,
                                    1,
                                    u_var9,
                                    u_var13,
                                    _local_10,
                                    _local_10,
                                );
                            }
                        }
                        return;
                    }
                }
                // LAB_1030_a660:
                local_28 = local_28 + 1;
            }
            if (_local_10 != 0x0) {
                ppc_var2 = *_local_10;
                ppc_var2(
                    u_var15,
                    _local_10,
                    (_local_10 >> 0x10),
                    1,
                    u_var9,
                    u_var13,
                    _local_10,
                    _local_10,
                );
            }
        }
        return;
    }

    pub unsafe fn pass1_1030_a844(param_1: *mut Struct954, param_2: *mut u16) {
        let mut u_var1: u16;
        let ppc_var2: fn();
        let mut u_var3: u32;
        let mut in_ax: i32;
        let mut u_var4: i32;
        let mut u_var5: u16;
        let pu_var6: *mut u16;
        let pu_var7: *mut u32;
        let mut u_var8: u32;
        let mut in_dx: i32;
        
        
        
        let local_bx_6: *mut Struct954;
        let mut i_var9: i32;
        let mut u_var10: u16;
        let mut u_var11: u16;
        let mut unaff_ss: u16;
        let mut local_46: u16;
        let mut local_3c: u16;
        let mut local_30: u16;
        let mut local_2e: u16;
        let mut local_2c: u16;
        let mut local_2a: u32;
        let mut local_22: u32;
        let mut local_1e: u16;
        let mut local_1c: u16;
        let mut local_1a: u16;
        let mut local_18: u16;
        let mut local_16: u16;
        let mut local_14: u16;
        let mut local_10: u16;
        let mut local_e: u32;
        let mut local_a: u16;
        let mut local_8: u16;
        let mut local_6: u32;

        u_var10 = (param_1 >> 0x10);
        local_bx_6 = param_1;
        pass1_1038_53ba(local_bx_6.field_0x4, (&ctx.PTR_LOOP_1050_0000 + 1));
        if ((in_dx != 0) || (in_ax != 0)) {
            u_var3 = local_bx_6.field_0x4;
            u_var11 = (u_var3 >> 0x10);
            i_var9 = u_var3;
            pu_var7 = (i_var9 + 0xc);
            ppc_var2 = (*pu_var7 + 0x10);
            local_6 = pu_var7;
            ppc_var2(&PTR_LOOP_1050_1038, pu_var7, (i_var9 + 0xe));
            _local_a = pu_var7 & 0xffff | ctx.dx_reg << 0x10;
            u_var3 = local_bx_6.field_0x4;
            local_e = (u_var3 + 8);
            local_10 = 0;
            zero_list_1008_3e38(CONCAT22(unaff_ss, &local_16));
            u_var1 = (param_2 + 4);
            local_22 = 0;
            while (local_22 < _local_a) {
                u_var8 = _local_a;
                pass17_funcs::pass1_1030_1d7c(local_6, local_22, (local_22 >> 0x10));
                if ((((ctx.dx_reg | u_var8) != 0)
                    && (
                        u_var4 = pass1_1008_c6ae(ctx._PTR_LOOP_1050_06e0, (u_var8 + 0xc), 0x46),
                        u_var4 != 0,
                    ))
                    && (pass17_funcs::pass1_1030_1d58(local_6), (ctx.dx_reg | u_var4) != 0))
                {
                    modify_list_1008_3f62(
                        CONCAT22(unaff_ss, &local_16),
                        CONCAT22(ctx.dx_reg, u_var4 + 0xc),
                    );
                    pass1_1008_3eb4(
                        CONCAT22(unaff_ss, &local_16),
                        CONCAT22(unaff_ss, &local_1c),
                        CONCAT22(unaff_ss, &local_1a),
                        CONCAT22(unaff_ss, &local_18),
                    );
                    if ((u_var1 == local_1c)
                        && (
                            u_var3 = local_bx_6.field_0x4,
                            u_var11 = (u_var3 >> 0x10),
                            i_var9 = u_var3,
                            u_var3 = (i_var9 + 4),
                            u_var5 = pass1_1030_addc(
                                local_bx_6,
                                u_var10,
                                CONCAT22(unaff_ss, &local_16),
                                u_var3,
                                (u_var3 >> 0x10),
                                (i_var9 + 8),
                            ),
                            u_var5 != 0,
                        ))
                    {
                        local_14 = local_1a - 1;
                        pu_var6 = &local_16;
                        pass1_1030_ad86(local_bx_6, u_var10, CONCAT22(unaff_ss, pu_var6), local_e);
                        if (pu_var6 != 0x0) {
                            // LAB_1030_a98e:
                            modify_list_1008_3f62(param_2, CONCAT22(unaff_ss, &local_16));
                            return;
                        }
                        local_14 = local_1a + 1;
                        pu_var6 = &local_16;
                        pass1_1030_ad86(local_bx_6, u_var10, CONCAT22(unaff_ss, pu_var6), local_e);
                        if (pu_var6 != 0x0) {}
                        // goto LAB_1030_a98e;
                        local_14 = local_1a;
                        local_16 = local_18 - 1;
                        pu_var6 = &local_16;
                        pass1_1030_ad86(local_bx_6, u_var10, CONCAT22(unaff_ss, pu_var6), local_e);
                        if (pu_var6 != 0x0) {}
                        // goto LAB_1030_a98e;
                        local_16 = local_18 + 1;
                        pu_var6 = &local_16;
                        pass1_1030_ad86(local_bx_6, u_var10, CONCAT22(unaff_ss, pu_var6), local_e);
                        if (pu_var6 != 0x0) {}
                        // goto LAB_1030_a98e;
                    }
                }
                local_22 = local_22 + 1;
            }
        }
        return;
    }

    pub unsafe fn pass1_1030_aa18(param_1: u32, param_2: *mut u16) {
        let mut u_var1: u32;
        let ppc_var2: fn();
        let mut u_var3: u32;
        let mut u_var4: u16;
        let pu_var5: Vec<u8>;
        let mut u_var6: u32;
        let mut in_dx: i32;
        
        
        
        let mut u_var7: u16;
        let mut u_var8: u16;
        let mut i_var9: i32;
        let pu_var10: *mut u32;
        let mut u_var11: u16;
        let mut u_var12: u16;
        let mut u_var13: u16;
        let mut u_var14: u16;
        let mut unaff_ss: u16;
        let mut u_var15: u32;
        let u_var16: u8;
        let mut local_4c: u32;
        let mut local_48: u32;
        let mut local_44: u16;
        let mut local_36: u16;
        let mut local_34: u16;
        let mut local_2a: u16;
        let mut local_28: u16;
        let mut local_26: u32;
        let mut local_22: u16;
        let mut local_20: u16;
        let mut local_1e: u32;
        let mut local_1a: [u8; 2];
        let mut local_18: u16;
        let mut local_16: u16;
        let mut local_14: u16;
        let mut local_12: u16;
        let mut local_e: u16;
        let mut local_c: u16;
        let mut local_a: u16;
        let mut local_8: u16;
        let mut local_6: u16;
        let mut local_4: u16;

        local_4 = (param_2 + 4);
        local_6 = 8 - (local_4 == 0);
        pu_var5 = pass1_1008_c6fa(ctx._PTR_LOOP_1050_06e0, local_6);
        u_var4 = pu_var5;
        u_var11 = (param_1 >> 0x10);
        u_var7 = param_1;
        local_a = u_var4;
        pass1_1038_4e78((u_var7 + 4), pu_var5 & 0xffff | in_dx << 0x10);
        _local_e = CONCAT22(ctx.dx_reg, u_var4);
        u_var14 = SUB42(&ctx.PTR_LOOP_1050_1008, 0);
        zero_list_1008_3e38(CONCAT22(unaff_ss, &local_14));
        u_var3 = (u_var7 + 4);
        u_var1 = (u_var3 + 8);
        u_var12 = (_local_e >> 0x10);
        u_var8 = SUB42(_local_e, 0);
        ppc_var2 = (*_local_e + 0x10);
        u_var6 = u_var1;
        ppc_var2(&ctx.PTR_LOOP_1050_1008, u_var8, u_var12);
        u_var6 = u_var6 & 0xffff | ctx.dx_reg << 0x10;
        local_26 = 0;
        loop {
            if (u_var6 <= local_26) {
                if (_local_e != 0x0) {
                    ppc_var2 = *_local_e;
                    ppc_var2(
                        u_var14,
                        _local_e,
                        (_local_e >> 0x10),
                        1,
                        u_var8,
                        u_var12,
                        _local_e,
                        _local_e,
                    );
                }
                return;
            }
            u_var15 = u_var6;
            pass17_funcs::pass1_1030_1d58(_local_e);
            if ((ctx.dx_reg | u_var15) != 0) {
                break;
            }
            // LAB_1030_aadc:
            local_26 = local_26 + 1;
        }
        modify_list_1008_3f62(
            CONCAT22(unaff_ss, &local_14),
            CONCAT22(ctx.dx_reg, u_var15 + 0xc),
        );
        u_var14 = SUB42(&ctx.PTR_LOOP_1050_1008, 0);
        pass1_1008_3eb4(
            CONCAT22(unaff_ss, &local_14),
            CONCAT22(unaff_ss, local_1a),
            CONCAT22(unaff_ss, &local_18),
            CONCAT22(unaff_ss, &local_16),
        );
        u_var3 = (u_var7 + 4);
        u_var13 = (u_var3 >> 0x10);
        i_var9 = u_var3;
        u_var3 = (i_var9 + 4);
        u_var4 = pass1_1030_addc(
            u_var7,
            u_var11,
            CONCAT22(unaff_ss, &local_14),
            u_var3,
            (u_var3 >> 0x10),
            (i_var9 + 8),
        );
        if (u_var4 == 0) {}
        // goto LAB_1030_aadc;
        u_var15 = pass17_funcs::pass1_1030_73a8((u_var15 & 0xffff | ctx.dx_reg << 0x10));
        i_var9 = (u_var15 + 0xc);
        if (5 < i_var9 - 0x7a) {}
        // goto LAB_1030_aadc;
        u_var14 = 0x1030;
        match (i_var9) {
            // default:
            _ => {
                local_12 = local_18 - 1;
                u_var4 = pass1_1030_acbe(u_var7, u_var11, CONCAT22(unaff_ss, &local_14), u_var1);
                if (u_var4 != 0) {}
                // goto LAB_1030_ac5b;
                local_12 = local_18 + 1;
                u_var4 = pass1_1030_acbe(u_var7, u_var11, CONCAT22(unaff_ss, &local_14), u_var1);
                if (u_var4 == 0) {
                    local_12 = local_18;
                    local_14 = local_16 - 1;
                    u_var4 =
                        pass1_1030_acbe(u_var7, u_var11, CONCAT22(unaff_ss, &local_14), u_var1);
                    // goto joined_r0x1030ab9e;
                }

                // LAB_1030_abc4:
                modify_list_1008_3f62(param_2, CONCAT22(unaff_ss, &local_14))
            }
            0x7b | 0x7e => {
                local_12 = local_18 - 1;
                u_var4 = pass1_1030_acbe(u_var7, u_var11, CONCAT22(unaff_ss, &local_14), u_var1);
                if (u_var4 == 0) {
                    local_12 = local_18 + 1;
                    // goto LAB_1030_abac;
                }
                modify_list_1008_3f62(param_2, CONCAT22(unaff_ss, &local_14));
                if (_local_e == 0x0) {
                    return;
                }
                u_var14 = (_local_e >> 0x10);
                pu_var10 = _local_e;
                u_var16 = (_local_e >> 0x10);
                // goto LAB_1030_ab66;
            }
            0x7c | 0x7d => {
                local_14 = local_16 - 1;
                u_var4 = pass1_1030_acbe(u_var7, u_var11, CONCAT22(unaff_ss, &local_14), u_var1);
                // joined_r0x1030ab9e:
                if (u_var4 == 0) {
                    local_14 = local_16 + 1;
                    // LAB_1030_abac:
                    u_var4 =
                        pass1_1030_acbe(u_var7, u_var11, CONCAT22(unaff_ss, &local_14), u_var1);
                    if (u_var4 != 0) {}
                    // goto LAB_1030_abc4;
                    // goto LAB_1030_aadc;
                }
                // LAB_1030_ac5b:
                modify_list_1008_3f62(param_2, CONCAT22(unaff_ss, &local_14));

                pu_var10 = _local_e;
                if ((local_c | pu_var10) != 0) {
                    u_var14 = (_local_e >> 0x10);
                    u_var16 = (_local_e >> 0x10);
                    // LAB_1030_ab66:
                    ppc_var2 = *pu_var10;
                    ppc_var2(
                        &ctx.PTR_LOOP_1050_1008,
                        pu_var10,
                        u_var16,
                        1,
                        u_var8,
                        u_var12,
                        _local_e,
                        _local_e,
                    );
                }
            }
        }
        return;
    }

    pub unsafe fn pass1_1030_acbe(param_1: u16, param_2: u16, param_1_00: u32, param_2_00: u32) {
        let mut iVar1: i32;
        let paVar2: *mut Struct493;
        let mut u_var3: i32;
        let mut u_var4: i32;
        let lVar5: u32;
        let mut u_var6: u32;
        let mut local_e: u16;
        let mut local_c: u16;
        let mut local_8: u16;
        let mut local_4: u16;

        lVar5 = pass17_funcs::pass1_1030_627e(_PTR_LOOP_1050_5740, param_1_00, param_2_00);
        u_var3 = (lVar5 >> 0x10);
        u_var4 = u_var3 | lVar5;
        if (lVar5 != 0) {
            paVar2 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, lVar5, u_var3);
            if ((u_var4 | paVar2) != 0) {
                u_var6 = pass17_funcs::pass1_1030_73a8(CONCAT22(u_var4, paVar2));
                if ((u_var6 != 0) && ((iVar1 = (u_var6 + 0xc), iVar1 == 5 || (iVar1 == 9)))) {
                    return 1;
                }
            }
        }
        return 0;
    }

    pub unsafe fn pass1_1030_ad22(
        param_1: u16,
        param_2: u16,
        param_1_00: u32,
        param_2_00: u32,
    ) -> bool {
        let paVar1: *mut Struct493;
        let BVar2: bool;
        let mut u_var3: i32;
        let mut u_var4: i32;
        let lVar5: u32;
        let mut u_var6: u32;
        let mut local_e: u16;
        let mut local_c: u16;
        let mut local_8: u16;
        let mut local_4: u16;

        lVar5 = pass17_funcs::pass1_1030_627e(_PTR_LOOP_1050_5740, param_1_00, param_2_00);
        u_var3 = (lVar5 >> 0x10);
        u_var4 = u_var3 | lVar5;
        if (lVar5 != 0) {
            paVar1 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, lVar5, u_var3);
            if ((u_var4 | paVar1) != 0) {
                u_var6 = pass17_funcs::pass1_1030_73a8(CONCAT22(u_var4, paVar1));
                if (u_var6 != 0) {
                    BVar2 = pass1_1008_c6ae(ctx._PTR_LOOP_1050_06e0, (u_var6 + 0xc), 0x46);
                    return BVar2;
                }
            }
        }
        return 0;
    }

    pub unsafe fn pass1_1030_ad86(param_1: u16, param_2: u16, param_1_00: u32, param_2_00: u32) {
        let mut u_var1: u32;
        let pu_var2: *mut u32;
        
        let mut unaff_ss: u16;
        let mut local_14: u32;
        let mut local_a: u32;
        let mut local_6: u32;

        pu_var2 = &local_a;
        pass17_funcs::pass1_1030_64ce(
            _PTR_LOOP_1050_5740,
            param_1_00,
            param_2_00,
            pu_var2,
            unaff_ss,
        );
        u_var1 = *pu_var2;
        local_14._3_1_ = (u_var1 >> 0x18);
        if (local_14._3_1_ == '\0') {
            local_6._0_2_ = u_var1;
            if (((0 < local_6) && (!SBORROW2(local_6, 1)))
                && (local_6 == 3 || local_6 + -2 < 1 || (3 < local_6 + -3 && (local_6 + -7 < 2))))
            {
                return;
            }
        }
        return;
    }

    pub unsafe fn pass1_1030_addc(
        param_1: u16,
        param_2: u16,
        param_1_00: *mut u16,
        param_4: u16,
        param_5: u16,
        param_2_00: u32,
    ) -> i32 {
        let mut in_dx: u16;
        let mut unaff_ss: u16;
        let pu_var1: *mut u32;
        let mut local_16: u16;
        let mut local_14: u16;
        let mut local_12: u16;
        let mut local_10: u16;
        let mut local_e: u16;
        let mut local_c: u32;
        let mut uStack8: u16;
        let mut local_6: u16;
        let mut local_4: u16;

        local_6 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, param_2_00, (param_2_00 >> 0x10));
        pu_var1 = pass17_funcs::pass1_1030_5b5c(local_6, in_dx);
        local_c = *pu_var1;
        uStack8 = (pu_var1 + 4);
        pass1_1008_3e94(
            param_1_00,
            CONCAT22(unaff_ss, &local_10),
            CONCAT22(unaff_ss, &local_e),
        );
        pass1_1008_3e94(
            &local_c,
            CONCAT22(unaff_ss, &local_14),
            CONCAT22(unaff_ss, &local_12),
        );
        if ((((1 < local_e) && (1 < local_10)) && (local_e < (local_12 - 1)))
            && (local_10 < (local_14 - 1)))
        {
            return 1;
        }
        return 0;
    }

    pub unsafe fn pass1_1030_ae6c(param_1: *mut u16) {
        let mut u_var1: u32;
        let mut u_var2: i32;
        let paVar3: *mut Struct199;
        let in_dx: *mut Struct199;
        let mut u_var4: i32;
        let mut i_var5: i32;
        let mut u_var6: u16;
        let mut local_8: u16;

        u_var6 = (param_1 >> 0x10);
        i_var5 = param_1;
        *param_1 = ctx.s_1_1050_389a;
        (i_var5 + 2) = &ctx.PTR_LOOP_1050_1008;
        (i_var5 + 4) = 0;
        zero_list_1008_3e38((param_1 & 0xffff0000 | (i_var5 + 8)));
        u_var2 = 0;
        (i_var5 + 0xe) = 0;
        (i_var5 + 0x10) = 0;
        *param_1 = 0xb932;
        (i_var5 + 2) = 0x1030;
        process_struct_1000_179c(0xc, in_dx);
        u_var4 = in_dx | u_var2;
        if (u_var4 == 0) {
            (i_var5 + 0x10) = 0;
        } else {
            paVar3 = process_struct_1008_574a(CONCAT22(in_dx, u_var2));
            (i_var5 + 0x10) = paVar3;
            (i_var5 + 0x12) = u_var4;
        }
        u_var1 = (i_var5 + 0x10);
        (u_var1 + 10) = 0;
        return;
    }

    pub unsafe fn pass1_1030_aefa(param_1: *mut Struct955, param_2: u32) {
        let mut u_var1: u32;
        let mut u_var2: i32;
        let paVar3: *mut Struct199;
        let pu_var4: *mut u16;
        let struct_a: *mut Struct199;
        let mut u_var5: i32;
        let local_bx_4: *mut Struct955;
        let mut u_var6: u16;
        let mut local_c: u16;

        u_var6 = (param_1 >> 0x10);
        local_bx_4 = param_1;
        param_1 = ctx.s_1_1050_389a;
        local_bx_4.field_0x2 = &ctx.PTR_LOOP_1050_1008;
        local_bx_4.field_0x4 = 0;
        zero_list_1008_3e38((param_1 & 0xffff0000 | &local_bx_4.field_0x8));
        local_bx_4.field_0xe = 0;
        &local_bx_4.field_0x10 = 0;
        param_1 = 0xb932;
        local_bx_4.field_0x2 = 0x1030;
        local_bx_4.field_0x4 = (param_2 + 4);
        pu_var4 = (param_1 & 0xffff0000 | &local_bx_4.field_0x8);
        modify_list_1008_3f62(pu_var4, param_2 & 0xffff0000 | (param_2 + 0xc));
        u_var2 = pu_var4;
        paVar3 = struct_a;
        process_struct_1000_179c(0xc, struct_a);
        u_var5 = paVar3 | u_var2;
        if (u_var5 == 0) {
            paVar3 = 0x0;
            u_var5 = 0;
        } else {
            paVar3 = process_struct_1008_574a(CONCAT22(paVar3, u_var2));
        }
        local_bx_4.field_0x10 = paVar3;
        &local_bx_4.field_0x12 = u_var5;
        u_var1 = &local_bx_4.field_0x10;
        (u_var1 + 10) = 0;
        return;
    }

    pub unsafe fn pass1_1030_afa6(param_1: *mut Struct956) {
        let pu_var1: *mut u32;
        let mut u_var2: i32;
        let ppc_var3: fn();
        let mut u_var4: u32;
        let local_bx_4: *mut Struct956;
        let mut u_var5: u16;

        u_var5 = (param_1 >> 0x10);
        local_bx_4 = param_1;
        param_1 = 0xb932;
        local_bx_4.field_0x2 = 0x1030;
        if (&local_bx_4.field_0x10 != 0) {
            u_var4 = &local_bx_4.field_0x10;
            (u_var4 + 10) = 1;
        }
        pu_var1 = local_bx_4.field_0x10;
        u_var2 = local_bx_4.field_0x12;
        if ((u_var2 | pu_var1) != 0) {
            ppc_var3 = *pu_var1;
            (**ppc_var3)();
        }
        param_1 = ctx.s_1_1050_389a;
        local_bx_4.field_0x2 = &ctx.PTR_LOOP_1050_1008;
        return;
    }

    pub unsafe fn pass1_1030_affc(param_1: *mut Struct960) {
        let mut iVar1: i32;
        let mut u_var2: u16;
        let paVar3: *mut Struct493;
        let paVar4: *mut Struct493;
        let mut u_var5: i32;
        
        let mut unaff_ss: u16;
        let mut u_var6: u32;
        let mut local_22: u16;
        let mut local_20: u16;
        let mut local_1e: u16;
        let mut local_1c: u16;
        let mut local_1a: u16;
        let mut local_16: u16;
        let mut local_14: u16;
        let mut local_10: u16;
        let mut local_e: u16;
        let mut local_c: u16;
        let mut local_a: u16;
        let mut local_8: u16;
        let mut local_6: u32;
        let temp_5f46e7a27c: *mut Struct957;

        pass1_1030_b718(
            param_1,
            param_1._2_2_,
            param_1 & 0xffff0000 | (param_1 + 8),
            CONCAT22(unaff_ss, &local_6),
        );
        paVar3 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, local_6, (local_6 >> 0x10));
        _local_a = CONCAT22(param_1._2_2_, paVar3);
        u_var5 = param_1._2_2_ | paVar3;
        if (u_var5 != 0) {
            u_var6 = pass17_funcs::pass1_1030_73a8(CONCAT22(param_1._2_2_, paVar3));
            u_var5 = (u_var6 >> 0x10);
            temp_5f46e7a27c = (u_var6 + 0xc);
            paVar4 = (temp_5f46e7a27c + -0x16);
            paVar3 = paVar4;
            if (((0x15 < temp_5f46e7a27c) && (!SBORROW2(temp_5f46e7a27c, 0x16)))
                && ((
                    paVar3 = (temp_5f46e7a27c + -0x17),
                    paVar3 == 0x0
                        || paVar4 < 1
                        || ((
                            paVar3 = (temp_5f46e7a27c + -0x19),
                            0 < (temp_5f46e7a27c + -0x18)
                                && (
                                    paVar3 = (temp_5f46e7a27c + -0x1a),
                                    paVar3 == 0x0 || (temp_5f46e7a27c + -0x19) < 1,
                                ),
                        )),
                )))
            {
                (u_var6 + 0x20) = 0;
            }
        }
        local_c = 6;
        loop {
            if (local_c == 0) {
                // LAB_1030_b0fc:
                if ((local_8 | local_a) != 0) {
                    u_var6 = pass17_funcs::pass1_1030_73a8(_local_a);
                    u_var2 = (u_var6 >> 0x10);
                    iVar1 = (u_var6 + 0xc);
                    if (((0x15 < iVar1) && (!SBORROW2(iVar1, 0x16)))
                        && (iVar1 == 0x17
                            || iVar1 + -0x16 < 1
                            || (0 < iVar1 + -0x18 && (iVar1 + -0x19 < 2))))
                    {
                        (u_var6 + 0x20) = 1;
                    }
                }
                return;
            }
            pass1_1030_b578(param_1);
            if ((u_var5 | paVar3) == 0) {}
            // goto LAB_1030_b0fc;
            _local_a = CONCAT22(u_var5, paVar3);
            u_var6 = pass17_funcs::pass1_1030_73a8(CONCAT22(u_var5, paVar3));
            iVar1 = (u_var6 + 0xc);
            modify_list_1008_3f62(
                (param_1 & 0xffff0000 | (param_1 + 8)),
                CONCAT22(u_var5, &paVar3.field_0xc),
            );
            u_var5 = ctx.dx_reg;
            if ((iVar1 == 0x18) || (iVar1 == 0x3f)) {
                pass1_1030_b142(param_1, _local_a);
            }
            paVar3 = pass1_1008_c6ae(ctx._PTR_LOOP_1050_06e0, iVar1, 0x40);
            if (paVar3 != 0x0) {
                pass1_1030_b454(param_1, _local_a);
                // goto LAB_1030_b0fc;
            }
            local_c = local_c - 1;
        }
    }

    pub unsafe fn ret_1030_b13c() {
        return;
    }

    pub unsafe fn pass1_1030_b142(param_1: *mut Struct959, param_2: *mut Struct493) {
        let mut iVar1: i32;
        let mut u_var2: u16;
        let local_bx_100: *mut Struct959;
        let local_es_100: *mut Struct959;
        let mut u8_var3: bool;
        let mut u_var4: u32;
        let mut local_16: u16;
        let mut local_14: u16;
        let mut local_12: u16;
        let mut local_10: u16;
        let mut local_e: u16;
        let mut local_c: u32;
        let mut local_8: u16;
        let mut local_6: u16;
        let mut local_4: u16;
        let temp_7ff85ff504d: *mut Struct958;

        u_var4 = pass17_funcs::pass1_1030_73a8(param_2);
        u_var2 = (u_var4 >> 0x10);
        temp_7ff85ff504d = u_var4;
        iVar1 = temp_7ff85ff504d.field_0xc;
        local_c = 0;
        if (iVar1 == 0x18) {
            local_c._2_2_ = return_false_1028_1c1c();
            u_var2 = temp_7ff85ff504d.field_0x22;
        } else {
            if (iVar1 != 0x3f) {}
            // goto LAB_1030_b1a6;
            local_c._2_2_ = return_false_1028_20b0();
            u_var2 = temp_7ff85ff504d.field_0x24;
        }
        local_c = CONCAT22(local_c._2_2_, u_var2);
        // LAB_1030_b1a6:
        local_es_100 = (param_1 >> 0x10);
        local_bx_100 = param_1;
        if (local_bx_100.field_0xe == 1) {
            u8_var3 = (local_c & 0x10000) == 0;
        } else {
            if (local_bx_100.field_0xe == 2) {
                u8_var3 = (local_c & 0x20000) == 0;
            } else {
                if (local_bx_100.field_0xe == 3) {
                    u8_var3 = (local_c & 0x40000) == 0;
                } else {
                    u8_var3 = (local_c & 0x80000) == 0;
                }
            }
        }
        if ((u8_var3) || (local_c != 0)) {
            u8_var3 = false;
            loop {
                if (((local_c & 0x10000) != 0) && (local_c == 0)) {}
                // goto LAB_1030_b239;
                if (((local_c & 0x20000) != 0) && (local_c == 0)) {}
                // goto LAB_1030_b247;
                if (((local_c & 0x40000) != 0) && (local_c == 0)) {}
                // goto LAB_1030_b255;
                if (((local_c & 0x80000) != 0) && (local_c == 0)) {}
                // goto LAB_1030_b263;
                if (u8_var3) {
                    break;
                }
                local_c._1_3_ = (local_c >> 8) & 0xffff00;
                iVar1 = local_bx_100.field_0xe;
                if (iVar1 == 1) {
                    local_c = CONCAT31(local_c._1_3_, 4);
                } else {
                    if (iVar1 == 2) {
                        local_c = CONCAT31(local_c._1_3_, 8);
                    } else {
                        if (iVar1 == 3) {
                            local_c = CONCAT31(local_c._1_3_, 1);
                        } else {
                            local_c = CONCAT31(local_c._1_3_, 2);
                        }
                    }
                }
                u8_var3 = true;
            }
            if (local_bx_100.field_0xe == 1) {
                // LAB_1030_b255:
                local_bx_100.field_0xe = 3;
                return;
            }
            if (local_bx_100.field_0xe == 2) {
                // LAB_1030_b263:
                local_bx_100.field_0xe = 4;
                return;
            }
            if (local_bx_100.field_0xe == 3) {
                // LAB_1030_b239:
                local_bx_100.field_0xe = 1;
                return;
            }
            if (local_bx_100.field_0xe == 4) {
                // LAB_1030_b247:
                local_bx_100.field_0xe = 2;
                return;
            }
        }
        return;
    }

    pub unsafe fn pass1_1030_b2aa(param_1: u32, param_2: u32) {
        let paVar1: *mut Struct493;
        let BVar2: bool;
        let mut unaff_ss: u16;
        let mut u_var3: u32;
        let mut local_1a: u32;
        let mut local_16: u16;
        let mut local_14: u16;
        let mut local_12: u16;
        let mut local_10: u32;
        let mut local_6: u32;

        pass1_1030_b718(
            param_1,
            (param_1 >> 0x10),
            param_2,
            CONCAT22(unaff_ss, &local_6),
        );
        local_1a._3_1_ = (local_6 >> 0x18);
        if (local_1a._3_1_ != '\0') {
            paVar1 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, local_6, local_6._2_2_);
            if ((local_6._2_2_ | paVar1) != 0) {
                u_var3 = pass17_funcs::pass1_1030_73a8(CONCAT22(local_6._2_2_, paVar1));
                BVar2 = pass1_1008_c6ae(ctx._PTR_LOOP_1050_06e0, (u_var3 + 0xc), 0x42);
                if (BVar2 != 0) {
                    modify_list_1008_3f62(
                        (param_1 & 0xffff0000 | (param_1 + 8)),
                        CONCAT22(local_6._2_2_, &paVar1.field_0xc),
                    );
                    return;
                }
            }
        }
        return;
    }

    pub unsafe fn pass1_1030_b344(param_1: Vec<u8>) {
        
        let mut u_var1: i32;
        let mut unaff_ss: u16;
        let mut local_1c: u16;
        let mut local_16: u16;
        let mut local_14: u16;
        let mut local_12: u16;
        let mut local_10: u16;
        let mut local_e: [u8; 2];
        let mut local_c: u16;
        let mut local_a: u16;
        let mut local_8: u16;
        let mut local_6: u16;
        let mut uStack4: u16;

        _local_8 = (param_1 + 8);
        uStack4 = (param_1 + 0xc);
        pass1_1008_3eb4(
            CONCAT22(unaff_ss, &local_8),
            CONCAT22(unaff_ss, local_e),
            CONCAT22(unaff_ss, &local_c),
            CONCAT22(unaff_ss, &local_a),
        );
        _local_8 = _local_8 & 0xffff | (local_c - 1) << 0x10;
        local_12 = &local_8;
        u_var1 = ctx.dx_reg;
        pass1_1030_b2aa(param_1, CONCAT22(unaff_ss, local_12));
        local_10 = u_var1 | local_12;
        if (local_10 == 0) {
            _local_8 = _local_8 & 0xffff | (local_c + 1) << 0x10;
            local_12 = &local_8;
            pass1_1030_b2aa(param_1, CONCAT22(unaff_ss, local_12));
            u_var1 = local_10 | local_12;
            if (u_var1 == 0) {
                local_8 = local_a - 1;
                local_6 = local_c;
                local_12 = &local_8;
                pass1_1030_b2aa(param_1, CONCAT22(unaff_ss, local_12));
                local_10 = u_var1 | local_12;
                if (local_10 == 0) {
                    _local_8 = CONCAT22(local_6, local_a + 1);
                    local_12 = &local_8;
                    pass1_1030_b2aa(param_1, CONCAT22(unaff_ss, local_12));
                    if ((local_10 | local_12) == 0) {
                        return 0;
                    }
                    (param_1 + 0xe) = 2;
                } else {
                    (param_1 + 0xe) = 4;
                    local_10 = u_var1;
                }
            } else {
                (param_1 + 0xe) = 3;
            }
        } else {
            (param_1 + 0xe) = 1;
            local_10 = u_var1;
        }
        return CONCAT22(local_10, local_12);
    }

    pub unsafe fn pass1_1030_b454(param_1: *mut Struct960, param_2: *mut Struct493) {
        let pu_var1: *mut u32;
        let ppc_var2: fn();
        let local_AX_39: Vec<u8>;
        let pu_var3: Vec<u8>;
        
        
        
        
        let local_bx_16: *mut Struct960;
        let mut local_es_16: u16;
        
        let mut u_var4: u32;
        let mut local_32: u32;
        let mut local_26: u16;
        let mut local_24: u16;
        let mut local_1e: u16;
        let mut local_1c: u16;
        let mut local_1a: u16;
        let mut local_18: u16;
        let mut local_16: u32;
        let mut local_12: [u8; 4];
        let mut local_e: u32;
        let mut local_a: u16;
        let mut local_8: u16;
        let mut local_6: u32;
        let temp_7e074184931: *mut u32;
        let mut temp_5f5d5590f3: u32;

        local_6 = (param_2 + 4);
        local_es_16 = (param_1 >> 0x10);
        local_bx_16 = param_1;
        local_32 = CONCAT22(ctx.stack_seg_reg, local_12);
        pass1_1008_5784(local_32, local_bx_16.field_0x10);
        loop {
            local_AX_39 = local_12;
            pass1_1008_5b12(CONCAT22(ctx.stack_seg_reg, local_AX_39));
            _local_a = CONCAT22(ctx.dx_reg, local_AX_39);
            break;
        }
        if ((local_AX_39 + 0x20) == local_6) {
            ppc_var2 = (local_bx_16.field_0x10 + 0xc);
            ppc_var2();
            local_e = 0;
            pass1_1038_69fe(_local_a);
        }
    }
    u_var4 = pass17_funcs::pass1_1030_73a8(param_2);
    pu_var1 = (u_var4 + 0x20);
    pu_var3 = local_12;
    local_32 = CONCAT22(ctx.stack_seg_reg, pu_var3);
    pass1_1008_5784(local_32, pu_var1);
    ret_1030_b13c(param_1);
    _local_1e = CONCAT22(
        -((s_Unsupported_FileStructType_in_Op_1050_01ca + 0x2a) < pu_var3) - ctx.dx_reg,
        500 - pu_var3,
    );
    while {
        pu_var3 = local_12;
        pass1_1008_5b12(CONCAT22(ctx.stack_seg_reg, pu_var3));
        _local_a = CONCAT22(ctx.dx_reg, pu_var3);
        if ((ctx.dx_reg | pu_var3) == 0) {
            return;
        }
        pass1_1038_6984(CONCAT22(ctx.dx_reg, pu_var3));
        _local_26 = CONCAT22(ctx.dx_reg, pu_var3);
        if ((ctx.dx_reg <= local_1c) && (ctx.dx_reg < local_1c || (pu_var3 <= local_1e))) {
            temp_5f5d5590f3 = local_bx_16.field_0x10;
            ppc_var2 = (local_bx_16.field_0x10 + 8);
            ppc_var2();
            _local_1e = _local_1e - _local_26;
            ppc_var2 = (*pu_var1 + 0xc);
            ppc_var2(
                &PTR_LOOP_1050_1038,
                pu_var1,
                (pu_var1 >> 0x10),
                _local_a,
                temp_5f5d5590f3,
            );
            local_e = 0;
            0 < _local_1e
        }
    } {}
    return;
}

pub unsafe fn pass1_1030_b578(param_1: u32) {
    let mut iVar1: i32;
    let p_uvar2: *mut u16;
    let mut u_var3: i32;
    
    let mut unaff_ss: u16;
    let mut u8_var4: bool;
    let mut u_var5: u32;
    let mut local_30: u32;
    let mut local_2c: u16;
    let mut local_2a: u16;
    let mut local_28: u16;
    let mut local_26: u32;
    let mut local_1c: [u8; 2];
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut uStack18: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;

    pass1_1030_b718(
        param_1,
        param_1._2_2_,
        param_1 & 0xffff0000 | (param_1 + 8),
        CONCAT22(unaff_ss, &local_6),
    );
    local_30._3_1_ = (local_6 >> 0x18);
    if (local_30._3_1_ == '\0') {
        return;
    }
    local_a = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, local_6, local_6._2_2_);
    local_8 = local_6._2_2_;
    _local_e = pass17_funcs::pass1_1030_73a8(CONCAT22(local_6._2_2_, local_a));
    local_10 = (_local_e + 0xc);
    _local_16 = (param_1 + 8);
    uStack18 = (param_1 + 0xc);
    pass1_1008_3eb4(
        CONCAT22(unaff_ss, &local_16),
        CONCAT22(unaff_ss, local_1c),
        CONCAT22(unaff_ss, &local_1a),
        CONCAT22(unaff_ss, &local_18),
    );
    iVar1 = (param_1 + 0xe);
    if (iVar1 == 0) {
        pass1_1030_b344((param_1 & 0xffff | param_1._2_2_ << 0x10));
        return;
    }
    if (iVar1 == 1) {
        u_var3 = local_1a - 1;
        // LAB_1030_b63e:
        _local_16 = _local_16 & 0xffff | u_var3 << 0x10;
        pu_var2 = &local_16;
        u_var3 = ctx.dx_reg;
        pass1_1030_b2aa(
            param_1 & 0xffff | param_1._2_2_ << 0x10,
            CONCAT22(unaff_ss, pu_var2),
        );
        local_30 = CONCAT22(u_var3, pu_var2);
        if ((u_var3 | pu_var2) == 0) {
            return;
        }
        u_var5 = pass17_funcs::pass1_1030_73a8(CONCAT22(u_var3, pu_var2));
        u_var3 = (u_var5 + 0xc);
        if (u_var3 == 0x3f) {}
        // goto LAB_1030_b6e0;
        if (0x3f < u_var3) {
            return;
        }
        if (u_var3 == '\x16') {}
        // goto LAB_1030_b6e0;
        u8_var4 = u_var3 == '\x18';
    } else {
        if (iVar1 == 2) {
            u_var3 = local_18 + 1;
        } else {
            if (iVar1 == 3) {
                u_var3 = local_1a + 1;
                // goto LAB_1030_b63e;
            }
            if (iVar1 != 4) {
                return;
            }
            u_var3 = local_18 - 1;
        }
        _local_16 = _local_16 & 0xffff0000 | u_var3;
        pu_var2 = &local_16;
        u_var3 = ctx.dx_reg;
        pass1_1030_b2aa(
            param_1 & 0xffff | param_1._2_2_ << 0x10,
            CONCAT22(unaff_ss, pu_var2),
        );
        local_30 = CONCAT22(u_var3, pu_var2);
        if ((u_var3 | pu_var2) == 0) {
            return;
        }
        u_var5 = pass17_funcs::pass1_1030_73a8(CONCAT22(u_var3, pu_var2));
        iVar1 = (u_var5 + 0xc);
        if (iVar1 < 0x17) {
            return;
        }
        if (SBORROW2(iVar1, 0x17)) {
            return;
        }
        if (iVar1 == 0x18 || iVar1 + -0x17 < 1) {}
        // goto LAB_1030_b6e0;
        u8_var4 = iVar1 == 0x3f;
    }
    if (!u8_var4) {
        return;
    }
    // LAB_1030_b6e0:
    modify_list_1008_3f62(
        (param_1 & 0xffff0000 | (param_1 + 8)),
        local_30 & 0xffff0000 | (local_30 + 0xc),
    );
    return;
}

pub unsafe fn pass1_1030_8cd2(param_1: *mut Struct104, param_2: u16) {
    let mut local_6: u32;

    local_6._0_1_ = '\0';
    if ((-1 < param_2) && (param_2 < 6)) {
        local_6._0_1_ = *(param_1 + 0x38 + param_2 * 4);
    }
    return local_6;
}

pub unsafe fn pass1_1030_8d08(param_1: u32) {
    let pu_var1: *mut u16;
    let mut u_var2: u32;
    let mut u_var3: u16;
    let paVar4: *mut Struct493;
    let mut in_dx: u16;
    let mut u_var5: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    local_4 = 0;
    loop {
        u_var5 = (param_1 >> 0x10);
        pu_var1 = (param_1 + 0x1e);
        let pu_var1_val = unsafe { *pu_var1 };
        if (pu_var1_val == local_4 || pu_var1_val < local_4) {
            break;
        }
        u_var3 = local_4 * 6;
        u_var2 = (param_1 + 0x1a);
        (u_var2 + u_var3 + 4) = 0;
        pass1_1028_e2ac(ctx._PTR_LOOP_1050_65e2);
        paVar4 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, u_var3, in_dx);
        _local_10 = CONCAT22(in_dx, paVar4);
        pass1_1028_e2e0(ctx._PTR_LOOP_1050_65e2);
        pass17_funcs::pass1_1030_7e5a(_local_10, CONCAT22(in_dx, paVar4));
        local_4 = local_4 + 1;
    }
    return;
}

pub unsafe fn pass1_1030_8d9e(param_1: *mut Struct104, param_2: u16, param_3: u16) {
    let mut unaff_ss: u16;
    let mut local_22: u16;
    let mut local_20: u16;
    let mut local_1e: u16;
    let mut local_1c: u32;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: u32;
    let mut local_10: u32;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: [u8; 6];

    zero_list_1008_3e38(CONCAT22(unaff_ss, local_8));
    modify_list_1008_6d64(param_1 & 0xffff0000 | (param_1 + 0x28), local_8, unaff_ss);
    pass1_1008_3e94(
        local_8,
        CONCAT22(unaff_ss, &local_c),
        CONCAT22(unaff_ss, &local_a),
    );
    return (local_a * param_2 + param_3);
}

pub unsafe fn pass1_1030_8e12(param_1: &mut Struct44, param_2: u8) -> &mut Struct44 {
    pass17_funcs::pass1_1030_8a2c(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1030_8e3c(param_1: u32, param_2: u32) {
    let mut iVar1: i32;
    let mut in_ax: i32;
    let paVar2: *mut Struct199;
    let paVar3: *mut Struct493;
    let in_dx: *mut Struct199;
    let mut u_var4: i32;
    let mut u_var5: u16;
    let ppVar6: *mut Struct2111;
    let mut u_var7: u16;
    let mut in_stack_0000ffe2: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    process_struct_1000_179c(0xc, in_dx);
    u_var4 = in_dx | in_ax;
    if (u_var4 == 0) {
        paVar2 = 0x0;
        u_var4 = 0;
    } else {
        paVar2 = process_struct_1008_574a(CONCAT22(in_dx, in_ax));
    }
    if (param_2._3_1_ == 0x4) {
        ppVar6 = process_struct_1010_20ba(
            ctx._g_Struct372_1050_0ed0,
            CONCAT22(in_stack_0000ffe2, 0x2f),
        );
        u_var5 = (ppVar6 >> 0x10);
        iVar1 = (ppVar6 + 0x1e);
        paVar3 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, param_2, (param_2 >> 0x10));
        u_var7 = (param_1 >> 0x10);
        if (iVar1 < 1) {
            pass1_1030_9296(param_1, CONCAT22(u_var4, paVar2), CONCAT22(u_var5, paVar3));
            pass1_1030_951a(param_1, CONCAT22(u_var4, paVar2), CONCAT22(u_var5, paVar3));
        } else {
            pass1_1030_9adc(
                param_1,
                u_var7,
                CONCAT22(u_var4, paVar2),
                CONCAT22(u_var5, paVar3),
            );
            pass1_1030_9c1c(param_1, CONCAT22(u_var4, paVar2), CONCAT22(u_var5, paVar3));
        }
        pass1_1030_9d42(
            param_1,
            u_var7,
            CONCAT22(u_var4, paVar2),
            CONCAT22(u_var5, paVar3),
        );
    }
    return CONCAT22(u_var4, paVar2);
}

pub unsafe fn pass1_1030_8f04(param_1: u16, param_2: u16, param_1_00: u32) {
    let mut u_var1: i32;
    let mut u_var2: i32;
    let mut in_eax: u32;
    let mut in_dx: i32;
    let mut u_var3: i32;
    let mut u_var4: i32;
    let mut u_var5: u32;
    let mut u_var6: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    pass1_1038_53ba(param_1_00, (&ctx.PTR_LOOP_1050_0000 + 1));
    if ((((in_dx != 0) || (1 < in_eax))
        && ((
            pass1_1038_53ba(param_1_00, &dos_alloc_addr_1050_0002),
            in_dx != 0 || (1 < in_eax),
        )))
        && ((
            pass1_1038_53ba(param_1_00, (&dos_alloc_addr_1050_0002 + 1)),
            in_dx != 0 || (1 < in_eax),
        )))
    {
        pass1_1038_53ba(param_1_00, &PTR_DAT_0005_0000_1050_0004);
        u_var5 = in_dx;
        if ((in_dx != 0) || (1 < in_eax)) {
            pass1_1038_540a(param_1_00, 0x21);
            _local_6 = in_eax & 0xffff | u_var5 << 0x10;
            local_8 = 0;
            loop {
                u_var3 = u_var5;
                u_var2 = in_eax;
                if (0 < (local_8 * 2 + _PTR_LOOP_1050_580e)) {
                    pass1_1038_540a(param_1_00, local_8);
                    u_var6 = (_PTR_LOOP_1050_580e >> 0x10);
                    u_var1 = (local_8 * 2 + _PTR_LOOP_1050_580e);
                    in_eax = u_var1;
                    u_var4 = u_var1 >> 0xf;
                    u_var5 = u_var4;
                    if ((u_var3 <= u_var4) && (u_var3 < u_var4 || (u_var2 < u_var1))) {
                        if (0x1c < local_8) {
                            return;
                        }
                        u_var2 = (local_8 * 2 + _PTR_LOOP_1050_580e);
                        in_eax = SEXT24(u_var2);
                        u_var5 = in_eax >> 0x10;
                        if (_local_6 < in_eax) {
                            return;
                        }
                        _local_6 = CONCAT22(
                            ((_local_6 >> 0x10) - (u_var2 >> 0xf)) - (local_6 < u_var2),
                            local_6 - u_var2,
                        );
                    }
                }
                local_8 = local_8 + 1;
                if (0x24 < local_8) {
                    return;
                }
            }
        }
    }
    return;
}

pub unsafe fn pass1_1030_8fe4(param_1: u16, param_2: u16, param_1_00: u32, param_2_00: u32) {
    let mut iVar1: i32;
    let struct_b: *mut Struct493;
    let mut u_var2: i32;
    let mut u_var3: i32;
    let lVar4: u32;
    let mut u_var5: u32;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_8: u16;
    let mut local_4: u16;

    lVar4 = pass17_funcs::pass1_1030_627e(_PTR_LOOP_1050_5740, param_1_00, param_2_00);
    u_var2 = (lVar4 >> 0x10);
    u_var3 = u_var2 | lVar4;
    if (lVar4 != 0) {
        struct_b = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, lVar4, u_var2);
        if ((u_var3 | struct_b) != 0) {
            u_var5 = pass17_funcs::pass1_1030_73a8(CONCAT22(u_var3, struct_b));
            if ((u_var5 != 0) && ((iVar1 = (u_var5 + 0xc), iVar1 == 5 || (iVar1 == 9)))) {
                return 1;
            }
        }
    }
    return 0;
}

pub unsafe fn pass1_1030_9048(param_1: u32, param_2: i32, param_3: u32) {
    let mut iVar1: i32;
    let mut u_var2: u32;
    let ppc_var3: fn();
    let mut u_var4: u16;
    let mut u_var5: u16;
    let paVar6: *mut Struct493;
    let pu_var7: Vec<u8>;
    let mut u_var8: u32;
    let mut in_dx: i32;
    
    
    
    let mut u_var9: u16;
    let pu_var10: *mut u32;
    let mut u_var11: u16;
    let mut u_var12: u16;
    let mut unaff_ss: u16;
    let mut u_var13: u32;
    let mut u_var14: u16;
    let u_var15: u8;
    let mut local_46: u32;
    let mut local_42: u32;
    let mut local_3e: u16;
    let mut local_38: u16;
    let mut local_36: u16;
    let mut local_2c: u16;
    let mut local_2a: u16;
    let mut local_28: u16;
    let mut local_26: u16;
    let mut local_24: u32;
    let mut local_20: u16;
    let mut local_1e: u16;
    let mut local_1c: u32;
    let mut local_18: [u8; 2];
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    local_4 = 8 - (param_2 == 0);
    pu_var7 = pass1_1008_c6fa(ctx._PTR_LOOP_1050_06e0, local_4);
    u_var4 = pu_var7;
    local_8 = u_var4;
    pass1_1038_4e78(param_3, pu_var7 & 0xffff | in_dx << 0x10);
    _local_c = CONCAT22(ctx.dx_reg, u_var4);
    u_var12 = SUB42(&ctx.PTR_LOOP_1050_1008, 0);
    zero_list_1008_3e38(CONCAT22(unaff_ss, &local_12));
    u_var2 = (param_3 + 8);
    u_var11 = (_local_c >> 0x10);
    u_var9 = SUB42(_local_c, 0);
    ppc_var3 = (*_local_c + 0x10);
    u_var8 = u_var2;
    (**ppc_var3)(&ctx.PTR_LOOP_1050_1008, u_var9, u_var11);
    u_var8 = u_var8 & 0xffff | ctx.dx_reg << 0x10;
    local_24 = 0;
    loop {
        if (u_var8 <= local_24) {
            if (_local_c != 0x0) {
                ppc_var3 = *_local_c;
                (**ppc_var3)(
                    u_var12,
                    _local_c,
                    (_local_c >> 0x10),
                    1,
                    u_var9,
                    u_var11,
                    _local_c,
                    _local_c,
                );
            }
            return;
        }
        ppc_var3 = (*_local_c + 4);
        u_var13 = u_var8;
        (**ppc_var3)();
        u_var4 = ctx.dx_reg;
        paVar6 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, u_var13, ctx.dx_reg);
        modify_list_1008_3f62(
            CONCAT22(unaff_ss, &local_12),
            CONCAT22(u_var4, &paVar6.field_0xc),
        );
        u_var12 = SUB42(&ctx.PTR_LOOP_1050_1008, 0);
        pass1_1008_3eb4(
            CONCAT22(unaff_ss, &local_12),
            CONCAT22(unaff_ss, local_18),
            CONCAT22(unaff_ss, &local_16),
            CONCAT22(unaff_ss, &local_14),
        );
        u_var13 = pass17_funcs::pass1_1030_73a8(CONCAT22(u_var4, paVar6));
        iVar1 = (u_var13 + 0xc);
        if (iVar1 - 0x7a < 6) {
            break;
        }
        // LAB_1030_91fa:
        local_24 = local_24 + 1;
    }
    u_var12 = 0x1030;
    u_var4 = param_1;
    u_var14 = (param_1 >> 0x10);
    match (iVar1) {
        _ => {
            local_10 = local_16 - 1;
            u_var5 = pass1_1030_8fe4(u_var4, u_var14, CONCAT22(unaff_ss, &local_12), u_var2);
            if (u_var5 != 0) {}
            // goto LAB_1030_91cb;
            local_10 = local_16 + 1;
            u_var5 = pass1_1030_8fe4(u_var4, u_var14, CONCAT22(unaff_ss, &local_12), u_var2);
            if (u_var5 == 0) {
                local_10 = local_16;
                local_12 = local_14 - 1;
                u_var5 = pass1_1030_8fe4(u_var4, u_var14, CONCAT22(unaff_ss, &local_12), u_var2);
                // goto joined_r0x1030911e;
            }
            // LAB_1030_9144:
        }
        0x7b | 0x7e => {
            local_10 = local_16 - 1;
            u_var5 = pass1_1030_8fe4(u_var4, u_var14, CONCAT22(unaff_ss, &local_12), u_var2);
            if (u_var5 == 0) {
                local_10 = local_16 + 1;
                // goto LAB_1030_912c;
            }
            if (_local_c == 0x0) {
                return;
            }
            u_var12 = (_local_c >> 0x10);
            pu_var10 = _local_c;
            u_var15 = (_local_c >> 0x10);
            // goto LAB_1030_90e6;
        }
        0x7c | 0x7d => {
            local_12 = local_14 - 1;
            u_var5 = pass1_1030_8fe4(u_var4, u_var14, CONCAT22(unaff_ss, &local_12), u_var2);
            // joined_r0x1030911e:
            if (u_var5 == 0) {
                local_12 = local_14 + 1;
                // LAB_1030_912c:
                u_var4 = pass1_1030_8fe4(u_var4, u_var14, CONCAT22(unaff_ss, &local_12), u_var2);
                if (u_var4 != 0) {}
                // goto LAB_1030_9144;
                // goto LAB_1030_91fa;
            }
            // LAB_1030_91cb:
        }
    }

    pu_var10 = _local_c;
    if ((local_a | pu_var10) != 0) {
        u_var12 = (_local_c >> 0x10);
        u_var15 = (_local_c >> 0x10);
        // LAB_1030_90e6:
        unsafe {
            ppc_var3 = *pu_var10;
            (**ppc_var3)(
                0x1030, pu_var10, u_var15, 1, u_var9, u_var11, _local_c, _local_c,
            );
        }
    }
    return;
}

pub unsafe fn pass1_1030_9296(param_1: u32, param_2: *mut u32, param_3: u32) {
    let pp_var1: fn();
    let mut u_var2: u16;
    let mut in_ax: i32;
    let mut u_var3: i32;
    let local_AX_355: *mut Struct950;
    let pu_var4: Vec<u8>;
    let pu_var5: *mut u16;
    let mut in_dx: i32;
    
    
    
    
    
    let mut extraout_dx_04: i32;
    let mut extraout_dx_05: i32;
    let mut extraout_dx_06: i32;
    let mut unaff_ss: u16;
    let mut in_stack_0000ffc2: u16;
    let mut local_3a: [u8; 12];
    let mut local_2e: u16;
    let mut local_2c: u16;
    let mut local_24: u32;
    let mut local_1e: u32;
    let mut local_1a: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u32;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;

    pass1_1038_53ba(param_3, (&ctx.PTR_LOOP_1050_0000 + 1));
    in_dx = in_dx | in_ax;
    if (in_dx != 0) {
        local_1e = _PTR_LOOP_1050_5768;
        pu_var5 = _PTR_LOOP_1050_5768;
        alloc_mem_1000_07fc(_PTR_LOOP_1050_5768);
        u_var3 = pu_var5;
        _local_12 = (pu_var5 & 0xffff | ctx.dx_reg << 0x10);
        if ((ctx.dx_reg | u_var3) == 0) {
            local_6 = 0;
        } else {
            *_local_12 = ctx.s_1_1050_389a;
            (u_var3 + 2) = &ctx.PTR_LOOP_1050_1008;
            (u_var3 + 4) = 0x73;
            *_local_12 = 0x9ec8;
            (u_var3 + 2) = 0x1030;
            pu_var5 = _local_12;
            local_6 = _local_12;
        }
        in_ax = pu_var5;
        unsafe {
            pp_var1 = (*param_2 + 4);
            (**pp_var1)(0x1000, param_2, local_6, (local_6 >> 0x10));
        }
        in_dx = ctx.dx_reg;
    }
    pass1_1038_53ba(param_3, &dos_alloc_addr_1050_0002);
    in_dx = in_dx | in_ax;
    if (in_dx != 0) {
        local_1e = _PTR_LOOP_1050_5768;
        pu_var5 = _PTR_LOOP_1050_5768;
        alloc_mem_1000_07fc(_PTR_LOOP_1050_5768);
        u_var3 = pu_var5;
        _local_12 = (pu_var5 & 0xffff | ctx.dx_reg << 0x10);
        if ((ctx.dx_reg | u_var3) == 0) {
            local_6 = 0;
        } else {
            *_local_12 = ctx.s_1_1050_389a;
            (u_var3 + 2) = &ctx.PTR_LOOP_1050_1008;
            (u_var3 + 4) = 0x74;
            *_local_12 = 0x9ec8;
            (u_var3 + 2) = 0x1030;
            pu_var5 = _local_12;
            local_6 = _local_12;
        }
        in_ax = pu_var5;
        unsafe {
            pp_var1 = (*param_2 + 8);
            (**pp_var1)(0x1000, param_2, local_6, (local_6 >> 0x10));
        }
        in_dx = ctx.dx_reg;
    }
    pass1_1038_53ba(param_3, (&dos_alloc_addr_1050_0002 + 1));
    if ((in_dx | in_ax) != 0) {
        local_24 = _PTR_LOOP_1050_5768;
        pu_var5 = _PTR_LOOP_1050_5768;
        alloc_mem_1000_07fc(_PTR_LOOP_1050_5768);
        u_var3 = pu_var5;
        _local_12 = (pu_var5 & 0xffff | ctx.dx_reg << 0x10);
        if ((ctx.dx_reg | u_var3) == 0) {
            local_6 = 0;
        } else {
            *_local_12 = ctx.s_1_1050_389a;
            (u_var3 + 2) = &ctx.PTR_LOOP_1050_1008;
            (u_var3 + 4) = 0x75;
            *_local_12 = 0x9ec8;
            (u_var3 + 2) = 0x1030;
            pu_var5 = _local_12;
            local_6 = _local_12;
        }
        in_ax = pu_var5;
        unsafe {
            pp_var1 = (*param_2 + 8);
            (**pp_var1)(0x1000, param_2, local_6, (local_6 >> 0x10));
        }
    }
    pass1_1030_8f04(param_1, (param_1 >> 0x10), param_3);
    if (in_ax != 0) {
        local_24 = _PTR_LOOP_1050_5768;
        pu_var5 = _PTR_LOOP_1050_5768;
        alloc_mem_1000_07fc(_PTR_LOOP_1050_5768);
        u_var3 = pu_var5;
        _local_12 = (pu_var5 & 0xffff | extraout_dx_04 << 0x10);
        if ((extraout_dx_04 | u_var3) == 0) {
            local_6 = 0;
        } else {
            *_local_12 = ctx.s_1_1050_389a;
            (u_var3 + 2) = &ctx.PTR_LOOP_1050_1008;
            (u_var3 + 4) = 0x37;
            *_local_12 = 0x9ec8;
            (u_var3 + 2) = 0x1030;
            local_6 = _local_12;
        }
        unsafe {
            pp_var1 = (*param_2 + 8);
            (**pp_var1)(0x1000, param_2, local_6, (local_6 >> 0x10));
        }
    }
    _local_a = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(in_stack_0000ffc2, 8));
    u_var2 = (_local_a >> 0x10);
    local_e = (_local_a + 0xe);
    u_var3 = (_local_a + 0x10);
    if ((u_var3 | local_e) != 0) {
        pass1_1008_5784(
            CONCAT22(unaff_ss, local_3a),
            local_e & 0xffff | u_var3 << 0x10,
        );
        loop {
            pu_var4 = local_3a;
            pass1_1008_5b12(CONCAT22(unaff_ss, pu_var4));
            _local_2e = CONCAT22(extraout_dx_05, pu_var4);
            if ((extraout_dx_05 | pu_var4) == 0) {
                break;
            }
            if (((pu_var4 + 4) == 0x3e) || ((pu_var4 + 4) == 0x41)) {
                local_1e = _PTR_LOOP_1050_5768;
                pu_var5 = _PTR_LOOP_1050_5768;
                alloc_mem_1000_07fc(_PTR_LOOP_1050_5768);
                u_var3 = pu_var5;
                _local_12 = (pu_var5 & 0xffff | extraout_dx_06 << 0x10);
                if ((extraout_dx_06 | u_var3) == 0) {
                    local_6 = 0;
                } else {
                    local_1a = (_local_2e + 4);
                    *_local_12 = ctx.s_1_1050_389a;
                    (u_var3 + 2) = &ctx.PTR_LOOP_1050_1008;
                    (u_var3 + 4) = local_1a;
                    *_local_12 = 0x9ec8;
                    (u_var3 + 2) = 0x1030;
                    local_6 = _local_12;
                }
                unsafe {
                    pp_var1 = (*param_2 + 8);
                    (**pp_var1)(0x1000, param_2, local_6, (local_6 >> 0x10));
                }
            }
        }
    }
    return;
}

pub unsafe fn pass1_1030_951a(param_1: u32, param_2: *mut u32, param_3: u32) {
    let pp_var1: fn();
    let mut u_var2: i32;
    let pu_var3: *mut u16;
    let pu_var4: *mut u16;
    let mut u_var5: u16;
    let paVar6: *mut Struct493;
    let pp_var7: *mut Struct2111;
    let pu_var8: Vec<u8>;
    
    let mut u_var9: i32;
    
    
    
    
    let mut extraout_dx_04: i32;
    let mut extraout_dx_05: i32;
    let mut extraout_dx_06: i32;
    let mut extraout_dx_07: i32;
    let mut extraout_dx_08: u16;
    let mut extraout_dx_09: u16;
    let mut extraout_dx_10: i32;
    let mut extraout_dx_11: i32;
    let mut extraout_dx_12: i32;
    let mut extraout_dx_13: i32;
    let mut extraout_dx_14: u16;
    let mut u_var10: u16;
    let mut extraout_dx_15: i32;
    let mut extraout_dx_16: i32;
    let mut i_var11: i32;
    let mut unaff_si: u16;
    let mut u_var12: u16;
    let mut u_var13: u16;
    let u_var14: u8;
    let mut unaff_ss: u16;
    let mut u_var15: u32;
    let mut u_var16: u32;
    let pu_var17: *mut u32;
    let u_var18: u8;
    let u_var19: u8;
    let u_var20: u8;
    let mut u_var21: u16;
    let mut local_5e: u32;
    let mut local_52: u16;
    let mut local_50: u16;
    let mut local_4e: u16;
    let mut local_4c: u16;
    let mut local_4a: u16;
    let mut local_46: u32;
    let mut local_3e: u32;
    let mut local_3a: u16;
    let mut local_38: u16;
    let mut local_36: u16;
    let mut local_34: u16;
    let mut local_2e: u16;
    let mut local_2c: u16;
    let mut local_2a: u16;
    let mut local_28: u16;
    let mut local_26: u16;
    let mut local_24: u32;
    let mut local_20: u16;
    let mut local_1e: u16;
    let mut local_1c: u16;
    let mut local_14: u16;
    let mut local_12: u32;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;

    _local_a = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(unaff_si, 0x35));
    u_var9 = (_local_a >> 0x10);
    u_var2 = _local_a + 10;
    _local_e = _local_a & 0xffff0000 | u_var2;
    pass1_1030_9048(param_1, 0, param_3);
    u_var12 = (param_2 >> 0x10);
    u_var20 = SUB41(param_2, 0);
    if (u_var2 != 0) {
        local_1c = 0;
        _local_20 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(unaff_si, 8));
        u_var13 = (_local_20 >> 0x10);
        local_24 = (_local_20 + 0xe);
        u_var9 = (_local_20 + 0x10);
        if ((u_var9 | local_24) != 0) {
            pass1_1008_5784(
                CONCAT22(unaff_ss, &local_36),
                local_24 & 0xffff | u_var9 << 0x10,
            );
            loop {
                pu_var3 = &local_36;
                pass1_1008_5b12(CONCAT22(unaff_ss, pu_var3));
                _local_2e = CONCAT22(ctx.dx_reg, pu_var3);
                u_var9 = ctx.dx_reg | pu_var3;
                if (u_var9 == 0) {
                    break;
                }
                if ((pu_var3[2] != 0x3e) && (pu_var3[2] != 0x41)) {
                    pp_var7 = _PTR_LOOP_1050_5768;
                    alloc_mem_1000_07fc(_PTR_LOOP_1050_5768);
                    u_var9 = pp_var7;
                    local_6 = pp_var7 & 0xffff | ctx.dx_reg << 0x10;
                    if ((ctx.dx_reg | u_var9) == 0) {
                        local_6 = 0;
                    } else {
                        u_var13 = (_local_2e + 4);
                        local_6 = ctx.s_1_1050_389a;
                        (u_var9 + 2) = &ctx.PTR_LOOP_1050_1008;
                        (u_var9 + 4) = u_var13;
                        local_6 = 0x9ec8;
                        (u_var9 + 2) = 0x1030;
                    }
                    unsafe {
                        pp_var1 = (*param_2 + 8);
                        (**pp_var1)(0, u_var20, u_var12, local_6, (local_6 >> 0x10));
                    }
                    if ((_local_2e + 4) == 0x13) {
                        local_1c = 1;
                    }
                }
            }
        }
        local_26 = 10;
        while (local_26 < 0x41) {
            if ((((((local_26 != 0x37) && (local_26 != 0x35)) && (local_26 != 0x36))
                && (local_26 != 0x25 && (local_26 != 0x26)))
                && (local_26 != 0x27 && ((local_26 * 2 + _local_e) != 0 && (local_26 != 0x13))))
                && (local_26 != 0x14 || (local_1c == 0)))
            {
                pp_var7 = _PTR_LOOP_1050_5768;
                alloc_mem_1000_07fc(_PTR_LOOP_1050_5768);
                u_var9 = pp_var7;
                local_6 = pp_var7 & 0xffff | ctx.dx_reg << 0x10;
                if ((ctx.dx_reg | u_var9) == 0) {
                    local_6 = 0;
                } else {
                    local_6 = ctx.s_1_1050_389a;
                    (u_var9 + 2) = &ctx.PTR_LOOP_1050_1008;
                    (u_var9 + 4) = local_26;
                    local_6 = 0x9ec8;
                    (u_var9 + 2) = 0x1030;
                }
                unsafe {
                    pp_var1 = (*param_2 + 8);
                    (**pp_var1)(0, u_var20, u_var12, local_6, (local_6 >> 0x10));
                }
                u_var9 = ctx.dx_reg;
            }
            local_26 = local_26 + 1;
        }
    }
    u_var13 = (_local_e >> 0x10);
    if ((_local_e + 0x6a) == 0) {
        if ((_local_e + 0x6c) != 0) {
            pp_var7 = _PTR_LOOP_1050_5768;
            alloc_mem_1000_07fc(_PTR_LOOP_1050_5768);
            pu_var4 = pp_var7;
            _local_3a = (pp_var7 & 0xffff | extraout_dx_06 << 0x10);
            if ((extraout_dx_06 | pu_var4) == 0) {}
            // goto LAB_1030_973e;
            unsafe {
                *_local_3a = ctx.s_1_1050_389a;
            }
            pu_var4[1] = &ctx.PTR_LOOP_1050_1008;
            pu_var4[2] = 0x36;
            u_var9 = extraout_dx_06;
            // goto LAB_1030_9728;
        }
    } else {
        pp_var7 = _PTR_LOOP_1050_5768;
        alloc_mem_1000_07fc(_PTR_LOOP_1050_5768);
        pu_var4 = pp_var7;
        _local_3a = (pp_var7 & 0xffff | ctx.dx_reg << 0x10);
        if ((ctx.dx_reg | pu_var4) == 0) {
            // LAB_1030_973e:
            local_6 = 0;
        } else {
            *_local_3a = ctx.s_1_1050_389a;
            pu_var4[1] = &ctx.PTR_LOOP_1050_1008;
            pu_var4[2] = 0x35;
            u_var9 = ctx.dx_reg;
            // LAB_1030_9728:
            unsafe {
                *pu_var4 = 0x9ec8;
                pu_var4[1] = 0x1030;
            }
            local_6 = _local_3a;
        }
        unsafe {
            pp_var1 = (*param_2 + 8);
            (**pp_var1)(0, u_var20, u_var12, local_6, (local_6 >> 0x10));
        }
        u_var9 = extraout_dx_04;
    }
    u_var13 = (_local_e >> 0x10);
    i_var11 = _local_e;
    if ((i_var11 + 0x4a) == 0) {
        if ((i_var11 + 0x4c) == 0) {
            if ((i_var11 + 0x4e) == 0) {}
            // goto LAB_1030_97e8;
            pp_var7 = _PTR_LOOP_1050_5768;
            alloc_mem_1000_07fc(_PTR_LOOP_1050_5768);
            pu_var4 = pp_var7;
            _local_3a = (pp_var7 & 0xffff | extraout_dx_11 << 0x10);
            if ((extraout_dx_11 | pu_var4) != 0) {
                *_local_3a = ctx.s_1_1050_389a;
                pu_var4[1] = &ctx.PTR_LOOP_1050_1008;
                pu_var4[2] = 0x27;
                u_var9 = extraout_dx_11;
                // goto LAB_1030_9879;
            }
        } else {
            pp_var7 = _PTR_LOOP_1050_5768;
            alloc_mem_1000_07fc(_PTR_LOOP_1050_5768);
            pu_var4 = pp_var7;
            _local_3a = (pp_var7 & 0xffff | extraout_dx_10 << 0x10);
            if ((extraout_dx_10 | pu_var4) != 0) {
                *_local_3a = ctx.s_1_1050_389a;
                pu_var4[1] = &ctx.PTR_LOOP_1050_1008;
                pu_var4[2] = 0x26;
                u_var9 = extraout_dx_10;
                // goto LAB_1030_9879;
            }
        }
        // LAB_1030_97d0:
        local_6 = 0;
    } else {
        pp_var7 = _PTR_LOOP_1050_5768;
        alloc_mem_1000_07fc(_PTR_LOOP_1050_5768);
        pu_var4 = pp_var7;
        _local_3a = (pp_var7 & 0xffff | extraout_dx_05 << 0x10);
        if ((extraout_dx_05 | pu_var4) == 0) {}
        // goto LAB_1030_97d0;
        unsafe {
            *_local_3a = ctx.s_1_1050_389a;
        }
        pu_var4[1] = &ctx.PTR_LOOP_1050_1008;
        pu_var4[2] = 0x25;

        u_var9 = extraout_dx_05;
        // LAB_1030_9879:
        unsafe {
            *pu_var4 = 0x9ec8;
            pu_var4[1] = 0x1030;
        }
        local_6 = _local_3a;
        unsafe {
            pp_var1 = (*param_2 + 8);
            (**pp_var1)(0, u_var20, u_var12, local_6, (local_6 >> 0x10));
        }
        u_var9 = extraout_dx_07;
    }
    // LAB_1030_97e8:
    local_12 = _local_a & 0xffff0000 | (_local_a + 0x11e);
    if ((_local_a + 0x138) != 0) {
        pu_var8 = pass1_1008_c6fa(ctx._PTR_LOOP_1050_06e0, 4);
        u_var5 = SUB42(pu_var8, 0);
        u_var14 = 0x38;
        pass1_1038_4d6e(param_3, pu_var8 & 0xffff | u_var9 << 0x10);
        _local_4c = CONCAT22(extraout_dx_08, u_var5);
        pp_var1 = (*_local_4c + 0x10);
        u_var13 = u_var5;
        (**pp_var1)(&PTR_LOOP_1050_1038, u_var5, extraout_dx_08);
        local_46 = CONCAT22(extraout_dx_09, u_var13);
        local_3e = 0;
        while (local_3e < local_46) {
            pp_var1 = (*_local_4c + 4);
            u_var15 = local_46;
            (**pp_var1)(
                u_var14,
                u_var5,
                extraout_dx_08,
                local_3e,
                (local_3e >> 0x10),
            );
            local_36 = u_var15;
            u_var21 = 0xd;
            u_var10 = extraout_dx_14;
            local_34 = extraout_dx_14;
            paVar6 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, local_36, extraout_dx_14);
            _local_2e = CONCAT22(u_var10, paVar6);
            u_var15 = pass17_funcs::pass1_1030_73a8(CONCAT22(u_var10, paVar6));
            local_28 = (u_var15 >> 0x10);
            local_2a = u_var15;
            u_var14 = 0x28;
            u_var16 = pass1_1028_6744(u_var15, u_var21);
            if (((u_var16 >> 0x10) | u_var16) != 0) {
                _local_20 = _PTR_LOOP_1050_5768;
                pp_var7 = _PTR_LOOP_1050_5768;
                alloc_mem_1000_07fc(_PTR_LOOP_1050_5768);
                u_var9 = pp_var7;
                local_6 = pp_var7 & 0xffff | extraout_dx_15 << 0x10;
                if ((extraout_dx_15 | u_var9) == 0) {
                    local_6 = 0;
                } else {
                    local_6 = ctx.s_1_1050_389a;
                    (u_var9 + 2) = &ctx.PTR_LOOP_1050_1008;
                    (u_var9 + 4) = 0x4c;
                    local_6 = 0x9ec8;
                    (u_var9 + 2) = 0x1030;
                }
                unsafe {
                    pp_var1 = (*param_2 + 8);
                    (**pp_var1)(0, u_var20, u_var12, local_6, (local_6 >> 0x10));
                }
                local_24 = _PTR_LOOP_1050_5768;
                pp_var7 = _PTR_LOOP_1050_5768;
                alloc_mem_1000_07fc(_PTR_LOOP_1050_5768);
                u_var9 = pp_var7;
                local_6 = pp_var7 & 0xffff | extraout_dx_12 << 0x10;
                if ((extraout_dx_12 | u_var9) == 0) {
                    local_6 = 0;
                } else {
                    local_6 = ctx.s_1_1050_389a;
                    (u_var9 + 2) = &ctx.PTR_LOOP_1050_1008;
                    (u_var9 + 4) = 0x4d;
                    local_6 = 0x9ec8;
                    (u_var9 + 2) = 0x1030;
                }
                u_var18 = local_6;
                u_var19 = (local_6 >> 0x10);
                unsafe {
                    pp_var1 = (*param_2 + 8);
                    pu_var17 = param_2;
                    (**pp_var1)();
                }
                local_24 = _PTR_LOOP_1050_5768;
                u_var14 = 0;
                pp_var7 = _PTR_LOOP_1050_5768;
                alloc_mem_1000_07fc(_PTR_LOOP_1050_5768);
                u_var9 = pp_var7;
                local_6 = pp_var7 & 0xffff | extraout_dx_13 << 0x10;
                if ((extraout_dx_13 | u_var9) == 0) {
                    local_6 = 0;
                } else {
                    local_6 = ctx.s_1_1050_389a;
                    (u_var9 + 2) = &ctx.PTR_LOOP_1050_1008;
                    (u_var9 + 4) = 0x4e;
                    local_6 = 0x9ec8;
                    (u_var9 + 2) = 0x1030;
                }
                unsafe {
                    pp_var1 = (*param_2 + 8);
                    (**pp_var1)(0x1000, param_2, local_6, pu_var17, u_var18, u_var19);
                }
                break;
            }
            local_3e = local_3e + 1;
        }
        if (_local_4c != 0x0) {
            pp_var1 = *_local_4c;
            (**pp_var1)(u_var14, u_var5, extraout_dx_08, 1);
        }
    }
    local_14 = 0x7a;
    while (local_14 < 0x7d) {
        if ((local_14 * 2 + _local_e) != 0) {
            pp_var7 = _PTR_LOOP_1050_5768;
            alloc_mem_1000_07fc(_PTR_LOOP_1050_5768);
            u_var9 = pp_var7;
            local_6 = pp_var7 & 0xffff | extraout_dx_16 << 0x10;
            if ((extraout_dx_16 | u_var9) == 0) {
                local_6 = 0;
            } else {
                local_6 = ctx.s_1_1050_389a;
                (u_var9 + 2) = &ctx.PTR_LOOP_1050_1008;
                (u_var9 + 4) = local_14;
                local_6 = 0x9ec8;
                (u_var9 + 2) = 0x1030;
            }
            unsafe {
                pp_var1 = (*param_2 + 8);
                (**pp_var1)(0, u_var20, u_var12, local_6, (local_6 >> 0x10));
            }
        }
        local_14 = local_14 + 1;
    }
    return;
}

pub unsafe fn pass1_1030_9adc(param_1: u16, param_2: u16, param_1_00: *mut u32, param_2_00: u32) {
    let pp_var1: fn();
    let mut in_ax: i32;
    let mut u_var2: i32;
    let pu_var3: *mut u16;
    let mut in_dx: i32;
    
    
    
    
    
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;

    pass1_1038_53ba(param_2_00, (&ctx.PTR_LOOP_1050_0000 + 1));
    in_dx = in_dx | in_ax;
    if (in_dx != 0) {
        pu_var3 = _PTR_LOOP_1050_5768;
        alloc_mem_1000_07fc(_PTR_LOOP_1050_5768);
        u_var2 = pu_var3;
        local_6 = pu_var3 & 0xffff | ctx.dx_reg << 0x10;
        if ((ctx.dx_reg | u_var2) == 0) {
            local_6 = 0;
        } else {
            local_6 = ctx.s_1_1050_389a;
            (u_var2 + 2) = &ctx.PTR_LOOP_1050_1008;
            (u_var2 + 4) = 0x77;
            local_6 = 0x9ec8;
            (u_var2 + 2) = 0x1030;
            pu_var3 = local_6;
        }
        in_ax = pu_var3;
        unsafe {
            pp_var1 = (*param_1_00 + 4);
            (**pp_var1)(0x1000, param_1_00, local_6, (local_6 >> 0x10));
        }
        in_dx = ctx.dx_reg;
    }
    pass1_1038_53ba(param_2_00, &dos_alloc_addr_1050_0002);
    in_dx = in_dx | in_ax;
    if (in_dx != 0) {
        pu_var3 = _PTR_LOOP_1050_5768;
        alloc_mem_1000_07fc(_PTR_LOOP_1050_5768);
        u_var2 = pu_var3;
        local_6 = pu_var3 & 0xffff | ctx.dx_reg << 0x10;
        if ((ctx.dx_reg | u_var2) == 0) {
            local_6 = 0;
        } else {
            local_6 = ctx.s_1_1050_389a;
            (u_var2 + 2) = &ctx.PTR_LOOP_1050_1008;
            (u_var2 + 4) = 0x78;
            local_6 = 0x9ec8;
            (u_var2 + 2) = 0x1030;
            pu_var3 = local_6;
        }
        in_ax = pu_var3;
        unsafe {
            pp_var1 = (*param_1_00 + 8);
            (**pp_var1)(0x1000, param_1_00, local_6, (local_6 >> 0x10));
        }
        in_dx = ctx.dx_reg;
    }
    pass1_1038_53ba(param_2_00, (&dos_alloc_addr_1050_0002 + 1));
    if ((in_dx | in_ax) != 0) {
        pu_var3 = _PTR_LOOP_1050_5768;
        alloc_mem_1000_07fc(_PTR_LOOP_1050_5768);
        u_var2 = pu_var3;
        local_6 = pu_var3 & 0xffff | ctx.dx_reg << 0x10;
        if ((ctx.dx_reg | u_var2) == 0) {
            local_6 = 0;
        } else {
            local_6 = ctx.s_1_1050_389a;
            (u_var2 + 2) = &ctx.PTR_LOOP_1050_1008;
            (u_var2 + 4) = 0x75;
            local_6 = 0x9ec8;
            (u_var2 + 2) = 0x1030;
        }
        unsafe {
            pp_var1 = (*param_1_00 + 8);
            (**pp_var1)(0x1000, param_1_00, local_6, (local_6 >> 0x10));
        }
    }
    return;
}

pub unsafe fn pass1_1030_9c1c(param_1: u32, param_2: *mut u32, param_3: u32) {
    let pp_var1: fn();
    let mut u_var2: u16;
    let mut i_var3: i32;
    let mut i_var4: i32;
    let mut u_var5: i32;
    let pu_var6: Vec<u8>;
    
    
    let mut unaff_si: u16;
    let pp_var7: *mut Struct2111;
    let mut local_1c: u16;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_6: u32;

    pp_var7 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(unaff_si, 0x35));
    i_var3 = pp_var7 + 10;
    u_var2 = (pp_var7 >> 0x10);
    i_var4 = i_var3;
    pass1_1030_9048(param_1, 1, param_3);
    if (i_var4 != 0) {
        local_18 = 0x4f;
        while (local_18 < 0x70) {
            if ((local_18 * 2 + i_var3) != 0) {
                pu_var6 = _PTR_LOOP_1050_5768;
                alloc_mem_1000_07fc(_PTR_LOOP_1050_5768);
                u_var5 = pu_var6;
                local_6 = pu_var6 & 0xffff | ctx.dx_reg << 0x10;
                if ((ctx.dx_reg | u_var5) == 0) {
                    local_6 = 0;
                } else {
                    local_6 = ctx.s_1_1050_389a;
                    (u_var5 + 2) = &ctx.PTR_LOOP_1050_1008;
                    (u_var5 + 4) = local_18;
                    local_6 = 0x9ec8;
                    (u_var5 + 2) = 0x1030;
                }
                unsafe {
                    pp_var1 = (*param_2 + 8);
                    (**pp_var1)(0x1000, param_2, local_6, (local_6 >> 0x10));
                }
            }
            local_18 = local_18 + 1;
        }
    }
    local_10 = 0x7d;
    while (local_10 < 0x80) {
        if ((local_10 * 2 + i_var3) != 0) {
            pu_var6 = _PTR_LOOP_1050_5768;
            alloc_mem_1000_07fc(_PTR_LOOP_1050_5768);
            u_var5 = pu_var6;
            local_6 = pu_var6 & 0xffff | ctx.dx_reg << 0x10;
            if ((ctx.dx_reg | u_var5) == 0) {
                local_6 = 0;
            } else {
                local_6 = ctx.s_1_1050_389a;
                (u_var5 + 2) = &ctx.PTR_LOOP_1050_1008;
                (u_var5 + 4) = local_10;
                local_6 = 0x9ec8;
                (u_var5 + 2) = 0x1030;
            }
            unsafe {
                pp_var1 = (*param_2 + 8);
                (**pp_var1)(0x1000, param_2, local_6, (local_6 >> 0x10));
            }
        }
        local_10 = local_10 + 1;
    }
    return;
}

pub unsafe fn pass1_1030_9d42(param_1: u16, param_2: u16, param_1_00: *mut u32, param_2_00: u32) {
    let pu_var1: *mut u32;
    let mut u_var2: u32;
    let mut u_var3: u32;
    let ppcVar4: fn();
    let u_var5: u8;
    let mut u_var6: u16;
    let pu_var7: Vec<u8>;
    let pu_var8: Vec<u8>;
    let extraout_var: u32;
    
    
    
    let mut u_var10: i32;
    let mut i_var11: i32;
    let mut u_var12: u16;
    let mut unaff_ss: u16;
    let mut local_b8: u16;
    let mut local_b6: u16;
    let mut local_b4: u32;
    let mut local_ae: u16;
    let mut local_ac: u16;
    let mut local_aa: u32;
    let mut local_a6: [u8; 4];
    let mut local_a2: u32;
    let mut local_9e: u16;
    let mut local_9c: u16;
    let mut local_9a: u16;
    let mut local_98: u16;
    let mut local_96: u16;
    let mut local_c: u32;
    let mut local_8: u32;
    let mut local_4: u16;
    let mut u_var9: u32;

    u_var12 = (param_2_00 >> 0x10);
    if ((param_2_00 + 0x206) == 0) {
        local_4 = (param_2_00 + 0x204);
        u_var5 = pass1_1000_4906(CONCAT22(unaff_ss, &local_98), 0, 0x94);
        u_var9 = CONCAT31(extraout_var, u_var5);
        local_9a = 0x11;
        while {
            pass1_1038_540a(param_2_00, local_9a);
            u_var6 = u_var9;
            (&local_98)[local_9a * 2] = u_var6;
            (&local_96)[local_9a * 2] = ctx.dx_reg;
            local_9a = local_9a + 1;
            local_9a < 0x25
        } {}
        pass1_1038_540a(param_2_00, 0x21);
        _local_9e = CONCAT22(ctx.dx_reg, u_var6);
        pass1_1008_5784(CONCAT22(unaff_ss, local_a6), param_1_00);
        u_var2 = (ctx._PTR_LOOP_1050_65e2 + 0x52);
        loop {
            pu_var7 = local_a6;
            pass1_1008_5b12(CONCAT22(unaff_ss, pu_var7));
            u_var10 = ctx.dx_reg | pu_var7;
            if (u_var10 == 0) {
                break;
            }
            pu_var8 = pu_var7;
            pass17_funcs::pass1_1030_4bbe(u_var2, (pu_var7 + 4));
            if (local_4 == 0) {
                local_9a = 0x11;
                while (local_9a < 0x25) {
                    i_var11 = local_9a * 4;
                    if (((pu_var8 + i_var11) != 0)
                        && (
                            u_var3 = (&local_98 + local_9a * 2),
                            pu_var1 = (pu_var8 + i_var11),
                            unsafe { u_var3 <= *pu_var1 } && unsafe { *pu_var1 != u_var3 },
                        ))
                    {
                        pu_var1 = (pu_var8 + i_var11);
                        if (unsafe { _local_9e <= *pu_var1 } && unsafe { *pu_var1 != _local_9e }) {}
                        // goto LAB_1030_9e17;
                        _local_9e = _local_9e - (pu_var8 + i_var11);
                    }
                    local_9a = local_9a + 1;
                }
            } else {
                pu_var1 = (pu_var8 + 0x8c);
                if ((unsafe { local_c <= *pu_var1 } && unsafe { *pu_var1 != local_c })
                    || (
                        pu_var1 = (pu_var8 + 0x90),
                        unsafe { local_8 <= *pu_var1 } && unsafe { *pu_var1 != local_8 },
                    ))
                {
                    // LAB_1030_9e17:
                    unsafe {
                        ppcVar4 = (*param_1_00 + 0xc);
                        (**ppcVar4)(&ctx.PTR_LOOP_1050_1008, param_1_00, pu_var7, ctx.dx_reg);
                    }
                    local_a2 = 0;
                }
            }
        }
    }
    return;
}

pub unsafe fn pass1_1030_8480(param_1: &mut Struct44) {
    unsafe {
        error_check_1000_17ce(*param_1);
    }
    return;
}

pub unsafe fn pass1_1030_8496(param_1: Vec<u8>) {
    error_check_1000_17ce((param_1 + 2));
    return;
}

pub unsafe fn pass1_1030_84ae(param_1: u32) {
    zero_list_1008_3e38((param_1 & 0xffff0000 | (param_1 + 8)));
    (param_1 + 0x1e) = 1;
    return;
}

pub unsafe fn pass1_1030_84d0(param_1: &mut Struct44) {
    let pu_var1: *mut u32;
    let mut u_var2: i32;
    let ppc_var3: fn();
    let in_AL: u8;
    let local_bx_5: *mut Struct941;
    let mut u_var4: u16;

    u_var4 = (param_1 >> 0x10);
    local_bx_5 = param_1;
    if (local_bx_5.field_0x1e != 0) {
        pu_var1 = local_bx_5.field_0xe;
        u_var2 = local_bx_5.field_0x10;
        if ((u_var2 | pu_var1) != 0) {
            unsafe {
                ppc_var3 = *pu_var1;
                (**ppc_var3)();
            }
        }
        pu_var1 = local_bx_5.field_0x12;
        u_var2 = local_bx_5.field_0x14;
        if ((u_var2 | pu_var1) != 0) {
            unsafe {
                ppc_var3 = *pu_var1;
                (**ppc_var3)();
            }
        }
        error_check_1000_17ce(local_bx_5.field_0x4);
        in_AL = error_check_1000_17ce(local_bx_5.field_0x16);
    }
    return in_AL;
}

pub unsafe fn pass1_1030_8544(param_1: *mut Struct942, param_2: *mut Struct943) {
    let local_bx_15: *mut Struct943;
    let local_bx_23: *mut Struct942;
    let mut u_var1: u16;
    let mut u_var2: u16;

    param_1.field_0x0 = param_2;
    u_var1 = (param_2 >> 0x10);
    local_bx_15 = param_2;
    u_var2 = (param_1 >> 0x10);
    local_bx_23 = param_1;
    local_bx_23.field_0x4 = local_bx_15.field_0x4;
    modify_list_1008_3f62(
        (param_1 & 0xffff0000 | &local_bx_23.field_0x8),
        param_2 & 0xffff0000 | &local_bx_15.field_0x8,
    );
    local_bx_23.field_0xe = local_bx_15.field_0xe;
    local_bx_23.field_0x12 = local_bx_15.field_0x12;
    local_bx_23.field_0x16 = local_bx_15.field_0x16;
    local_bx_23.field_0x1a = local_bx_15.field_0x1a;
    local_bx_23.field_0x1e = 0;
    return;
}

pub unsafe fn pass1_1030_85be(param_1: *mut Struct944, param_2: u16, param_3: u16) {
    let local_bx_3: *mut Struct944;
    let mut u_var1: u16;

    u_var1 = (param_1 >> 0x10);
    local_bx_3 = param_1;
    param_1 = 0;
    local_bx_3.field_0x4 = 0;
    local_bx_3.field_0x6 = param_3;
    local_bx_3.field_0x8 = param_2;
    local_bx_3.field_0xe = 0;
    if (local_bx_3.field_0x6 == 0) {
        local_bx_3.field_0x6 = 5;
    }
    pass1_1030_878c(param_1);
    return;
}

pub unsafe fn pass1_1030_8604(param_1: *mut &mut Struct44) {
    unsafe {
        error_check_1000_17ce(*param_1);
    }
    return;
}

pub unsafe fn pass1_1030_861a(param_1: i32, param_2: u16, param_3: u16) {
    let mut in_ax: i32;
    let mut in_dx: i32;
    let mut local_6: u16;
    let mut local_4: u16;

    pass1_1030_8854(CONCAT22(param_2, param_1), param_3);
    _local_6 = CONCAT22(in_dx, in_ax);
    if ((in_dx | in_ax) == 0) {
        (param_1 + 10) = 0;
    } else {
        (param_1 + 10) = *_local_6;
    }
    return;
}

pub unsafe fn pass1_1030_8660(param_1: u32, param_2: *mut u32, param_3: u16) {
    let mut in_ax: i32;
    let mut in_dx: i32;
    
    let mut u_var1: i32;
    let mut local_6: u16;
    let mut local_4: u16;

    pass1_1030_8854(param_1, param_3);
    _local_6 = CONCAT22(in_dx, in_ax);
    in_dx = in_dx | in_ax;
    if (in_dx == 0) {
        pass1_1030_8854(param_1, 0);
        _local_6 = CONCAT22(in_dx, in_ax);
        if ((in_dx | in_ax) == 0) {
            pass1_1030_878c(param_1);
            u_var1 = ctx.dx_reg;
            pass1_1030_8854(param_1, 0);
            _local_6 = CONCAT22(u_var1, in_ax);
            if ((u_var1 | in_ax) == 0) {
                return;
            }
        }
        (_local_6 + 4) = param_3;
        unsafe {
            *_local_6 = *param_2;
        }
        pass1_1030_8834(param_1);
    } else {
        unsafe {
            *_local_6 = *param_2;
        }
    }
    return;
}

pub unsafe fn pass1_1030_86ec(param_1: &mut Struct44, param_2: u16) {
    let local_bx_18: &mut Struct44;
    let mut u_var1: u16;

    error_check_1000_17ce(param_1);
    u_var1 = (param_1 >> 0x10);
    local_bx_18 = param_1;
    param_1 = 0;
    local_bx_18.struct_fld_4 = 0x0;
    &local_bx_18.field_0x6 = param_2;
    &local_bx_18.field_0xe = 0;
    return;
}

pub unsafe fn pass1_1030_871e(param_1: *mut Struct946, param_2: *mut u32, param_3: u16) {
    let piVar1: *mut i32;
    let local_bx_5: *mut Struct946;
    let mut u_var2: u16;

    u_var2 = (param_1 >> 0x10);
    local_bx_5 = param_1;
    if (param_1 == 0) {
        pass1_1030_878c(param_1);
    }
    piVar1 = &local_bx_5.field_0xe;
    unsafe { *piVar1 = *piVar1 + 1 };
    (param_1 + local_bx_5.field_0xe * 6 + 4) = param_3;
    unsafe {
        (local_bx_5.field_0xe * 6 + param_1) = *param_2;
    }
    return;
}

pub unsafe fn pass1_1030_877c(param_1: u32) {
    pass1_1030_8834(param_1);
    return;
}

pub unsafe fn pass1_1030_878c(param_1: *mut long) {
    let pu_var1: *mut u32;
    let paVar2: *mut Struct94;
    let paVar3: *mut Struct94;
    let mut u_var4: i32;
    let ctx.dx_reg: *mut u16;
    let local_bx_4: *mut Struct947;
    let mut u_var5: i32;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;

    u_var5 = (param_1 >> 0x10);
    local_bx_4 = param_1;
    if (local_bx_4.field_0x4 == 0x0) {
        ctx.g_u16_ptr_1050_5f2e = 0x0;
        paVar3 = local_bx_4.field_0x6;
    } else {
        paVar2 = local_bx_4.field_0x4;
        pu_var1 = &local_bx_4.field_0x8;
        unsafe {
            paVar3 = (&paVar2.field_0x0 + *pu_var1);
        }
        unsafe {
            ctx.g_u16_ptr_1050_5f2e = CARRY2(paVar2, *pu_var1);
        }
    }
    if (ctx.g_u16_ptr_1050_5f2e == 0x0) {
        if (unsafe { *param_1 == 0 }) {
            if (ctx.__g_Struct94_ptr_1 == 0) {
                struct_fn_1000_160a();
            } else {
            }
            u_var4 = paVar3 * 6;
            alloc_mem_1000_1708(u_var4, 0, 1);
        } else {
            u_var4 = paVar3 * 6;
            unsafe {
                alloc_mem_1000_0ed4(1, u_var4, 0, *param_1);
            }
            ctx.g_u16_ptr_1050_5f2e = ctx.dx_reg;
        }
        _local_c = CONCAT22(ctx.g_u16_ptr_1050_5f2e, u_var4);
        if ((ctx.g_u16_ptr_1050_5f2e | u_var4) != 0) {
            local_bx_4.field_0x4 = paVar3;
            unsafe {
                *param_1 = _local_c;
            }
            pass1_1030_8834((param_1 & 0xffff | u_var5 << 0x10));
        }
    }
    return;
}

pub unsafe fn pass1_1030_8834(param_1: *mut u16) {
    unsafe {
        pass1_1000_4aea(*param_1, (param_1 + 2), 6, 0x888e, 0x1030);
    }
    return;
}

pub unsafe fn pass1_1030_8854(param_1: u32, param_2: u16) {
    let mut u_var1: u32;
    let mut unaff_ss: u16;
    let mut local_c: u32;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    local_8 = param_2;
    local_c = 0;
    u_var1 = (param_1 + 2);
    pass1_1000_49c6(
        &local_c,
        unaff_ss,
        param_1,
        u_var1,
        (u_var1 >> 0x10),
        6,
        0x888e,
    );
    return;
}

// l
