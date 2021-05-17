use crate::{
    defines::{Struct1104, Struct1105, Struct1106, Struct318, Struct44, Struct872},
    err_funcs::error_check_1000_17ce,
    pass4_funcs::pass1_1028_e1ec,
    pass5_funcs::{
        pass1_1030_1d7c, pass1_1030_326a, pass1_1030_38b8, pass1_1030_cf78, pass1_1030_d00c,
    },
    pass_funcs::pass1_1008_c6fa,
    util::{CONCAT22, SUB42},
};
use crate::pass4_funcs::{pass1_1028_b46e, pass1_1028_b58e, pass1_1028_b39e, pass1_1028_b354, pass1_1028_b418, pass1_1028_6744, pass1_1028_b4f2, pass1_1028_6228, pass1_1028_bdac, pass1_1028_c952, pass1_1028_cb04, pass1_1030_1312, pass1_1028_d1dc, pass1_1028_e332, pass1_1028_e4ec, pass1_1028_dc52, pass1_1028_9992, pass1_1028_9944, pass1_1028_6408, pass1_1028_62c8, pass1_1028_d52c, pass1_1030_1358, pass1_1030_11aa, pass1_1028_6356, pass1_1028_678c, pass1_1028_bb24};
use crate::util::{CONCAT31, CONCAT12, CONCAT13, ZEXT24, SBORROW2, SUB41, CARRY2, CONCAT11, SUB21, CARRY1, SBORROW1, POPCOUNT, SCARRY1};
use crate::pass5_funcs::{pass1_1030_6e9c, ret_1030_178e, pass1_1030_bcde, pass1_1030_bcae, pass1_1030_cc44, pass1_1030_cbf0, pass1_1030_cef8, pass1_1030_cde8, pass1_1030_73a8, pass1_1030_7ddc, pass1_1030_2aaa, pass1_1030_835a, pass1_1030_bd74, pass1_1030_1d58, pass1_1030_6c4c, pass1_1030_7d1c, pass1_1030_7c28, pass1_1030_6c66, pass1_1030_6b16, pass1_1030_d0a8, pass1_1030_d180, pass1_1030_d144, pass1_1030_314c, pass1_1030_17ce, pass1_1030_355c, pass1_1030_5b00, pass1_1030_8344, pass1_1030_3694, pass1_1030_9f40, pass1_1030_9ecc, pass1_1030_9ef2, pass1_1030_18b2, pass1_1030_183c, pass1_1030_bfb8, pass1_1030_ce2e, pass1_1030_6b86, pass1_1030_6fa0, pass1_1030_ce72, pass1_1030_7c50, pass1_1030_375a, pass1_1030_5b1c, pass1_1030_73ee, pass1_1030_1cd8, pass1_1030_64ce, pass1_1030_3258, pass1_1030_38f2, pass1_1030_72d0, pass1_1030_8e3c, pass1_1030_627e, pass1_1030_6d4e};
use crate::prog_structs::prog_structs_17::{Struct983, Struct850, Struct1115};
use crate::prog_structs::prog_structs_16::{Struct982, Struct1010, Struct493, Struct1088, Struct1097, Struct1118};
use crate::prog_structs::prog_structs_31::{Struct981, Struct987, Struct348, Struct112};
use crate::prog_structs::prog_structs_7::Struct44;
use crate::pass_funcs::{pass1_1000_4906, pass1_1008_c6ae, pass1_1008_5b12, pass1_1008_5784, pass1_1008_b820, pass1_1008_3e76, pass1_1008_3e54, pass1_fn_1008_612e, pass1_1008_de58, pass1_fn_1008_60e8, pass1_1008_3eb4, pass1_1008_b340, pass1_1040_b040, pass1_1008_4d84};
use crate::prog_structs::prog_structs_29::{Struct764, Struct763, Struct990, Struct998};
use crate::bad_funcs::halt_baddata;
use crate::prog_structs::prog_structs_28::{Struct1009, Struct1008, Struct1013, Struct1014, Struct1012, Struct1015, Struct1018, Struct1017, Struct1045, Struct1070, Struct1069, Struct918, Struct1066, Struct1064, Struct1063, Struct1062, Struct1061, Struct1059, Struct1058, Struct1077, Struct1076, Struct1081, Struct1080, Struct1079, Struct1089, Struct1087, Struct1084, Struct1095, Struct1091, Struct1093, Struct1094, Struct1092};
use crate::string_funcs::{pass1_1030_dfcc, copy_string_1000_3d3e, pas1_1030_e8a0, wvsprintf_FUN_1030_840a, string_fn_1000_3f9c, pass1_1028_87f0};
use crate::prog_structs::prog_structs_24::{Struct2111, Struct1068, Struct1100, Struct1124, Struct1131, Struct1170};
use crate::other_funcs::{ret_1030_e4ba, modify_list_1008_3f62, zero_list_1008_3e38};
use crate::prog_structs::prog_structs_25::{Struct1021, Struct1107, Struct1105, Struct1106, Struct1104, Struct1101, Struct1102, Struct1103};
use crate::pass8_funcs::{pass1_1010_089e, pass1_1010_ed3e, pass1_1008_eb74, pass1_1010_043a, pass1_1010_8fba, pass1_1010_3770, pass1_1010_038e};
use crate::prog_structs::prog_structs_19::{Struct500, Struct1050, Struct1086, Struct1109};
use crate::pass3_funcs::{pass1_1020_a49a, pass1_1020_a43e, pass1_1020_ba7e, pass1_1020_bb16, pass1_1028_45e2, infinite_loop_1020_ba94, pass1_1028_0d80, pass1_1020_ba3e, pass1_1020_bb70, pass1_1020_bb8a, switch_statement_1020_c3b4, pass1_1020_c42e, pass1_1020_bae6, pass1_1028_3c32, pass1_1020_c444, pass1_1020_a6ee, pass1_1020_c4f4, pass1_1028_5a94, ret_one_1020_c3ae};
use crate::prog_structs::prog_structs_14::Struct1036;
use crate::sys_funcs::{pass1_1038_095e, pass1_1038_7356, win_cleanup_func_1040_b0f8};
use crate::prog_structs::prog_structs_26::{Struct1052, Struct1096, Struct1121, Struct1119, Struct1117, Struct1116, Struct1114, Struct1110, Struct1123, Struct1122, Struct1128, Struct1127, Struct1129, Struct1132, Struct1134, Struct1133, Struct1135, Struct1136, Struct1137, Struct1147, Struct1146, Struct1145, Struct1151, Struct1153, Struct1152, Struct1154, Struct1155};
use crate::func_ptr_funcs::pass1_1038_7a5a;
use crate::struct_funcs::{process_struct_1000_179c, struct_fn_1000_160a, process_struct_1040_b082, process_struct_1040_7728, pass1_1038_a122, process_struct_1008_4c58};
use crate::prog_structs::prog_structs_2::{Struct199, Struct306, Struct599, Struct318};
use crate::prog_structs::prog_structs_22::Struct922;
use crate::mem_funcs::{alloc_mem_1000_07fc, alloc_mem_1000_1708, Address};
use crate::prog_structs::prog_structs_1::{Struct1065, Struct1067, Struct393, Struct1130};
use crate::prog_structs::prog_structs_21::{Struct1060, Struct1078, Struct1120};
use crate::prog_structs::prog_structs_27::{Struct1083, Struct1082, pass1_struct_2, Struct848};
use crate::prog_structs::prog_structs_6::Struct1098;
use crate::prog_structs::prog_structs_18::Struct1099;
use crate::prog_structs::prog_structs_8::{Struct1108, Struct515, Struct68};
use crate::prog_structs::prog_structs_9::Struct872;
use crate::prog_structs::prog_structs_12::Struct1125;
use crate::prog_structs::prog_structs_23::{Struct1113, Struct1111, Struct1126, Struct1148, Struct1150, Struct1149};
use crate::prog_structs::prog_structs_13::Struct1112;
use crate::prog_structs::prog_structs_10::Struct1139;
use crate::prog_structs::prog_structs_11::Struct1138;
use crate::ui_funcs::{pass1_1038_af40, destroy_win_1040_7b98};
use crate::cleanup::{pass1_1038_7d5c, pass1_1038_9a48, win_cleanup_func_1040_782c, pass1_1038_a156, pass1_1038_ae08};
use crate::app_context::AppContext;

pub unsafe fn pass1_1038_c410(
    ctx: &mut AppContext,
    param_1: &mut Struct44,
    param_2: u8,
) -> &mut Struct44 {
    pass1_1038_be4a(ctx, param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1038_be76(param_1: Vec<u8>, param_2: Vec<u8>) {
    let paVar1: *mut Struct318;
    let BVar2: bool;

    if (param_2._2_2_ == 0) {
        BVar2 = 0;
        paVar1 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, 0x2b);
        pass1_1010_038e(paVar1, BVar2);
    }
    destroy_win_1040_7b98(param_1, param_2);
    return;
}

pub unsafe fn pass1_1038_be4a(ctx: &mut AppContext, param_1: &mut Struct44) {
    let local_es_3: Vec<u8>;

    local_es_3 = (param_1 >> 0x10);
    param_1.field_0x0 = 0xc436;
    (param_1 + 2) = &ctx.PTR_LOOP_1050_1038;
    pass1_1038_b6e0(ctx._g_Struct112_a, *(param_1 + 6));
    win_cleanup_func_1040_782c(param_1);
    return;
}

pub unsafe fn pass1_1038_bd4a(param_1: &mut Struct44, param_2: u8) -> &mut Struct44 {
    pass1_1038_b7f0(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1038_bca8(param_1: Vec<u8>) {
    let mut u_var1: i32;
    let ppc_var2: fn();
    let mut u_var3: u32;
    let pu_var4: *mut u32;
    let pu_var5: *mut u32;
    
    let struct_a: *mut Struct199;
    let ctx.dx_reg: *mut Struct199;
    
    let mut u_var6: u16;
    let mut i_var7: i32;
    let mut u_var8: u16;
    let mut local_18: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;

    u_var8 = (param_1 >> 0x10);
    i_var7 = param_1;
    u_var3 = (i_var7 + 0x8e);
    pu_var5 = (u_var3 + 10);
    unsafe {
        ppc_var2 = (*pu_var5 + 0x14);
    }
    ppc_var2();
    struct_a = ctx.dx_reg;
    pu_var4 = pu_var5;
    if ((i_var7 + 0x70) != 0) {
        pu_var4 = (i_var7 + 0x70);
        u_var1 = (i_var7 + 0x72);
        struct_a = (u_var1 | pu_var4);
        if (struct_a != 0x0) {
            unsafe {
                ppc_var2 = *pu_var4;
            }
            ppc_var2();
            struct_a = ctx.dx_reg;
        }
    }
    process_struct_1000_179c(0x14, struct_a);
    if ((struct_a | pu_var4) == 0) {
        pu_var4 = 0x0;
        u_var6 = 0;
    } else {
        process_struct_1008_4c58(CONCAT22(struct_a, pu_var4));
        u_var6 = ctx.dx_reg;
    }
    (i_var7 + 0x70) = pu_var4;
    (i_var7 + 0x72) = u_var6;
    pass1_1008_4d84((i_var7 + 0x70), pu_var5);
    return;
}

pub unsafe fn pass1_1038_b7f0(param_1: *mut Struct599) {
    let mut u_var1: u16;

    u_var1 = (param_1 >> 0x10);
    param_1.field_0x0 = 0xbd70;
    (param_1 + 2) = &PTR_LOOP_1050_1038;
    pass1_1038_b6e0(ctx._g_Struct112_a, *(param_1 + 6));
    win_cleanup_func_1040_782c(param_1);
    return;
}

pub unsafe fn pass1_1038_b6e0(ctx: &mut AppContext,
                              param_1: &mut Address<Struct112>,
                              param_2: &mut Vec<u8>) -> u16 {
    let mut u_var1: u32;
    let local_astruct: *mut Struct112;
    let mut u_var2: u16;
    let mut u16_var3: u16;

    u16_var3 = 1;
    loop {
        if 0x2a < u16_var3 {
            return ctx.ax_reg;
        }
        u_var2 = (param_1 >> 0x10);
        local_astruct = param_1;
        in_ax = ((local_astruct + u16_var3 * 4 + 2) | (local_astruct + u16_var3 * 4));
        let uvar1_6_val = *(u_var1 + 6);
        if (ctx.ax_reg != 0x0)
            && (
            u_var1 = (local_astruct + u16_var3 * 4),
            ctx.ax_reg = param_2,
            uvar1_6_val == param_2,
            )
        {
            break;
        }
        u16_var3 = u16_var3 + 1;
    }
    (local_astruct + u16_var3 * 4) = 0;
    return 0;
}

pub unsafe fn pass1_1038_ae28(param_1: &mut Struct44, param_2: u8) -> &mut Struct44 {
    pass1_1038_ae08(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1038_ad4c(param_1: &mut Struct44, param_2: u8) -> &mut Struct44 {
    pass1_1038_abb0(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1038_adde(
    param_1: i32,
    param_2: Vec<u8>,
    param_3: Vec<u8>,
    param_3_00: Vec<u8>,
) {
    pass1_1038_9b72(param_1, param_2, param_3, param_3_00);
    CONCAT22(param_2, param_1) = 0xae4e;
    (param_1 + 2) = &PTR_LOOP_1050_1038;
    return CONCAT22(param_2, param_1);
}

pub unsafe fn pass1_1038_abb0(param_1: *mut Struct599) {
    let mut u_var1: u16;

    u_var1 = (param_1 >> 0x10);
    param_1.field_0x0 = 0xad72;
    (param_1 + 2) = &PTR_LOOP_1050_1038;
    pass1_1038_b6e0(ctx._g_Struct112_a, *(param_1 + 6));
    win_cleanup_func_1040_782c(param_1);
    return;
}

pub unsafe fn pass1_1038_aaf0(param_1: &mut Struct44, param_2: u8) -> &mut Struct44 {
    pass1_1038_a8cc(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1038_a80c(param_1: &mut Struct44, param_2: u8) -> &mut Struct44 {
    pass1_1038_a6c8(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1038_a89e(param_1: *mut u16, param_2: u16) {
    let pu_var1: Vec<u8>;

    pu_var1 = (param_1 >> 0x10);
    pass1_1038_a122(param_1, pu_var1, 1, 0, CONCAT22(param_2, 0xfca));
    unsafe {
        *param_1 = 0xab16;
        (param_1 + 2) = &PTR_LOOP_1050_1038;
    }
    return param_1;
}

pub unsafe fn pass1_1038_a8cc(param_1: *mut Struct599) {
    let mut u_var1: u16;

    u_var1 = (param_1 >> 0x10);
    param_1.field_0x0 = 0xab16;
    (param_1 + 2) = &PTR_LOOP_1050_1038;
    pass1_1038_b6e0(ctx._g_Struct112_a, *(param_1 + 6));
    pass1_1038_a156(param_1);
    return;
}

pub unsafe fn pass1_1038_a608(param_1: &mut Struct44, param_2: u8) -> &mut Struct44 {
    pass1_1038_a4c2(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1038_a69a(param_1: *mut u16, param_2: Vec<u8>) {
    let pu_var1: Vec<u8>;

    pu_var1 = (param_1 >> 0x10);
    pass1_1038_a122(param_1, pu_var1, 1, 0, CONCAT22(param_2, 0xfc9));
    unsafe {
        *param_1 = 0xa832;
        (param_1 + 2) = &PTR_LOOP_1050_1038;
    }
    return param_1;
}

pub unsafe fn pass1_1038_a6c8(param_1: *mut Struct599) {
    let mut u_var1: u16;

    u_var1 = (param_1 >> 0x10);
    param_1.field_0x0 = 0xa832;
    (param_1 + 2) = &PTR_LOOP_1050_1038;
    pass1_1038_b6e0(ctx._g_Struct112_a, *(param_1 + 6));
    pass1_1038_a156(param_1);
    return;
}

pub unsafe fn pass1_1038_a494(param_1: *mut u16, param_2: u16) {
    let pu_var1: Vec<u8>;

    pu_var1 = (param_1 >> 0x10);
    pass1_1038_a122(param_1, pu_var1, 1, 0, CONCAT22(param_2, 0xfc8));
    unsafe {
        *param_1 = 0xa62e;
        (param_1 + 2) = &PTR_LOOP_1050_1038;
    }
    return param_1;
}

pub unsafe fn pass1_1038_a4c2(param_1: *mut Struct599) {
    let mut u_var1: u16;

    u_var1 = (param_1 >> 0x10);
    param_1.field_0x0 = 0xa62e;
    (param_1 + 2) = &PTR_LOOP_1050_1038;
    pass1_1038_b6e0(ctx._g_Struct112_a, *(param_1 + 6));
    pass1_1038_a156(param_1);
    return;
}

pub unsafe fn pass1_1038_a402(param_1: &mut Struct44, param_2: u8) -> &mut Struct44 {
    pass1_1038_a36a(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1038_a2aa(param_1: &mut Struct44, param_2: u8) -> &mut Struct44 {
    pass1_1038_a156(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1038_a33c(param_1: *mut u16, param_2: Vec<u8>) -> *mut u16 {
    let pu_var1: Vec<u8>;

    pu_var1 = (param_1 >> 0x10);
    pass1_1038_a122(param_1, pu_var1, 1, 0, CONCAT22(param_2, 0xfc7));
    unsafe {
        *param_1 = 0xa428;
        (param_1 + 2) = &PTR_LOOP_1050_1038;
    }
    return param_1;
}

pub unsafe fn pass1_1038_a36a(param_1: *mut Struct599) {
    let mut u_var1: u16;

    u_var1 = (param_1 >> 0x10);
    param_1.field_0x0 = 0xa428;
    (param_1 + 2) = &PTR_LOOP_1050_1038;
    pass1_1038_b6e0(ctx._g_Struct112_a, *(param_1 + 6));
    pass1_1038_a156(param_1);
    return;
}

pub unsafe fn pass1_1038_a174(param_1: Vec<u8>, param_2: i32) {
    if (param_2 == 1) {
        (param_1 + 0x8e) = 0;
    }
    return;
}

pub unsafe fn pass1_1038_a090(param_1: &mut Struct44, param_2: u8) -> &mut Struct44 {
    pass1_1038_9fa4(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1038_9fa4(param_1: *mut Struct599) {
    let mut u_var1: u16;

    u_var1 = (param_1 >> 0x10);
    param_1.field_0x0 = 0xa0b6;
    (param_1 + 2) = &PTR_LOOP_1050_1038;
    pass1_1038_b6e0(ctx._g_Struct112_a, *(param_1 + 6));
    win_cleanup_func_1040_782c(param_1);
    return;
}

pub unsafe fn pass1_1038_9f56(param_1: &mut Struct44) -> &mut Struct44 {
    byte * *ppu8_var1;
    let pu8_var2: Vec<u8>;
    let mut u8_var3: u8;
    let mut u8_var4: u8;
    let mut cVar5: u8;
    let mut u_var6: i32;
    let mut in_CL: u8;
    let mut bVar7: u8;
    let mut in_dx: i32;
    let mut bVar8: u8;
    let mut in_bx: i32;
    let mut bVar9: u8;
    let pu_var10: *mut u16;
    let unaff_bp: *mut u16;
    let unaff_si: Vec<u8>;
    let mut unaff_ss: u16;
    let mut in_CF: u8;
    let mut bVar11: bool;
    let mut bVar12: bool;
    let in_struct_1: &mut Struct44;
    let in_stack_0000bf2a: Vec<u8>;
    let mut bStack78: u8;
    let puStack34: Vec<u8>;

    puStack34 = &stack0xfffe;
    pu_var10 = &stack0xfffe;
    cVar5 = '\x0f';
    while {
        unaff_bp = unaff_bp + -1;
        pu_var10 = pu_var10 + -1;
        unsafe { *pu_var10 = *unaff_bp };
        cVar5 = cVar5 + -1;
        '\0' < cVar5
    } {}
    bVar9 = (in_bx >> 8);
    unaff_si[in_bx] = bVar9;
    bVar8 = (in_dx + 1 >> 8);
    u8_var3 = bVar8 + bVar9;
    bVar11 = CARRY1(bVar8, bVar9) || CARRY1(u8_var3, in_CF);
    u_var6 = in_dx + 1 & 0xff;
    in_struct_1 = CONCAT22(&stack0xbf2a, &stack0xfffe);
    pu8_var2 = unaff_si + in_bx;
    bVar7 = u_var6;
    unsafe {
        bVar8 = *pu8_var2;
        u8_var4 = *pu8_var2 - bVar7;
        bVar12 = *pu8_var2 < bVar7 || u8_var4 < bVar11;
        *pu8_var2 = u8_var4 - bVar11;
        if ((*pu8_var2 != 0)
            && (SBORROW1(bVar8, bVar7) != SBORROW1(u8_var4, bVar11)) == (*pu8_var2 < '\0'))
        {
            pu8_var2 = unaff_si;
            bVar11 = CARRY1(*pu8_var2, bVar9) || CARRY1(*pu8_var2 + bVar9, bVar12);
            *pu8_var2 = *pu8_var2 + bVar9 + bVar12;
            bVar8 = bStack78 + in_bx;
            bVar12 = CARRY1(bStack78, in_bx) || CARRY1(bVar8, bVar11);
            bStack78 = bVar8 + bVar11;
            pu8_var2 = unaff_si + -0x4f;
            bVar8 = *pu8_var2;
            u8_var4 = *pu8_var2;
            *pu8_var2 = u8_var4 + bVar9 + bVar12;
            pu8_var2 = unaff_si + -0x78;
            *pu8_var2 =
                *pu8_var2 + in_CL + (CARRY1(bVar8, bVar9) || CARRY1(u8_var4 + bVar9, bVar12));
            puStack34 = &stack0xfffe;
            process_struct_1040_7728(
                in_struct_1,
                (&ctx.PTR_LOOP_1050_0000 + 1),
                CONCAT22(u_var6 | (u8_var3 + in_CF) << 8, in_bx),
                0xfba,
                in_stack_0000bf2a,
            );
            in_struct_1.ptr_a_lo = 0xa0b6;
            (in_struct_1 + 2) = &PTR_LOOP_1050_1038;
            return in_struct_1;
        }
        ppu8_var1 = (unaff_si + 9);
        *ppu8_var1 = unaff_si + *ppu8_var1;
    }
    error_check_1000_17ce(param_1);
    return param_1;
}

pub unsafe fn pass1_1038_9f02(param_1: &mut Struct44) -> &mut Struct44 {
    let pu8_var1: Vec<u8>;
    byte * *ppu8_var2;
    let pcVar3: String;
    let pu_var4: *mut u32;
    let mut b_var5: u8;
    let mut u_var6: i32;
    let mut u_var7: i32;
    let mut bVar8: u8;
    let mut cVar9: u8;
    let mut in_ax: i32;
    let mut bVar10: u8;
    let mut cVar12: u8;
    let mut in_cx: u16;
    let mut bVar13: u8;
    let mut b_var14: u8;
    let mut in_dx: i32;
    let mut bVar16: u8;
    let mut iVar15: i32;
    let mut bVar17: u8;
    let mut in_bx: u16;
    let pu_var18: *mut u16;
    let unaff_bp: *mut u16;
    let unaff_si: Vec<u8>;
    let mut unaff_es: u16;
    let mut unaff_ss: u16;
    let mut in_CF: u8;
    let mut bVar19: bool;
    let mut b_var20: bool;
    let mut b_var21: bool;
    let mut in_af: u8;
    let mut in_TF: u8;
    let mut in_IF: u8;
    let mut in_NT: u8;
    let mut in_stack_0000007c: u16;
    let mut bStack007d: u8;
    let in_struct_1: *mut Struct68;
    let local_4e: u8;
    let puStack34: Vec<u8>;
    let mut bVar11: u8;

    puStack34 = &stack0xfffe;
    pu_var18 = &stack0xfffe;
    cVar12 = '\x0f';
    while {
        unaff_bp = unaff_bp + -1;
        pu_var18 = pu_var18 + -1;
        unsafe { *pu_var18 = *unaff_bp };
        cVar12 = cVar12 + -1;
        '\0' < cVar12
    } {}
    bVar11 = (in_ax >> 8);
    bVar13 = (in_cx >> 8);
    bVar17 = bVar11 + bVar13;
    bVar10 = bVar17 + in_CF;
    b_var14 = in_dx;
    bVar19 = CARRY1(bStack007d, b_var14)
        || CARRY1(
            bStack007d + b_var14,
            unaff_si[CONCAT11((in_bx >> 8), 0x40)] < b_var14,
        );
    pu8_var1 = &stack0x4079 + unaff_si;
    bVar16 = (in_dx >> 8);
    unsafe {
        b_var20 = CARRY1(*pu8_var1, bVar16) || CARRY1(*pu8_var1 + bVar16, bVar19);
        *pu8_var1 = *pu8_var1 + bVar16 + bVar19;
        pu8_var1 = unaff_si;
        b_var5 = *pu8_var1;
        bVar8 = *pu8_var1 + 0x40;
        bVar19 = 0xbf < *pu8_var1 || CARRY1(bVar8, b_var20);
        *pu8_var1 = bVar8 + b_var20;
        cVar12 = in_cx;
        if ((*pu8_var1 == 0)
            || (SCARRY1(b_var5, '@') != SCARRY1(bVar8, b_var20)) != (*pu8_var1 < '\0'))
        {
            pu8_var1 = unaff_si + 0x3fc8;
            *pu8_var1 = *pu8_var1 + cVar12 + bVar19;
            cVar9 = PTR_LOOP_1050_4080;
            PTR_LOOP_1050_4080._0_1_ = '@';
            pu_var18 = CONCAT11(cVar9, 0x40);
            pu8_var1 = unaff_si;
            *pu8_var1 = *pu8_var1 + cVar12 + (unaff_si[0x4040] < b_var14);
            pcVar3 = (pu_var18 + unaff_si + 0x10);
            *pcVar3 = *pcVar3 + 'T';
            pcVar3 = (pu_var18 + unaff_si + 0x10);
            *pcVar3 = *pcVar3 + -0x34;
            pu_var4 = (pu_var18 + unaff_si + 0x10);
            u_var6 = *pu_var4;
            *pu_var4 = *pu_var4 + 0x81b6;
            pu_var4 = (pu_var18 + unaff_si + 0x10);
            u_var7 = *pu_var4;
            *pu_var4 = *pu_var4 + 0x60ea;
            pcVar3 = (pu_var18 + unaff_si);
            *pcVar3 = (*pcVar3 - (in_dx & 0xff)) - (0x9f15 < u_var7);
            iVar15 = (in_dx & 0xff | (bVar16 + cVar9 + (0x7e49 < u_var6)) << 8) - 1;
            pcVar3 = (pu_var18 + unaff_si + 0x10);
            *pcVar3 = *pcVar3 + 'f';
            pu8_var1 = (pu_var18 + unaff_si + 0x10);
            b_var5 = *pu8_var1;
            *pu8_var1 = *pu8_var1 - 0x22;
            if (-1 < *pu8_var1) {
                bVar17 = (iVar15 >> 8);
                pcVar3 = (pu_var18 + unaff_si);
                *pcVar3 = (*pcVar3 - iVar15)
                    - (CARRY1(bVar17, bVar13) || CARRY1(bVar17 + bVar13, 0x21 < b_var5));
                loop {
                    // WARNING: Do nothing block with infinite loop
                }
            }
        } else {
            b_var20 = 0xbf < bVar16 || CARRY1(bVar16 + 0x40, bVar19);
            in_struct_1 = CONCAT22(&stack0xc73f, &stack0xfffe);
            pu8_var1 = unaff_si + 0x4040;
            b_var14 = (in_dx & 0xff);
            b_var5 = *pu8_var1;
            bVar8 = *pu8_var1 - b_var14;
            b_var21 = *pu8_var1 < b_var14 || bVar8 < b_var20;
            *pu8_var1 = bVar8 - b_var20;
            if ((*pu8_var1 == 0)
                || (SBORROW1(b_var5, b_var14) != SBORROW1(bVar8, b_var20)) != (*pu8_var1 < '\0'))
            {
                ppu8_var2 = (unaff_si + 9);
                *ppu8_var2 = unaff_si + *ppu8_var2;
                error_check_1000_17ce(param_1);
                return param_1;
            }
            pu8_var1 = unaff_si;
            b_var20 = 0xbf < *pu8_var1 || CARRY1(*pu8_var1 + 0x40, b_var21);
            *pu8_var1 = *pu8_var1 + 0x40 + b_var21;
            b_var21 = 0xbf < local_4e || CARRY1(local_4e + 0x40, b_var20);
            local_4e = local_4e + 0x40 + b_var20;
            pu8_var1 = unaff_si + -0x4f;
            b_var5 = *pu8_var1;
            bVar8 = *pu8_var1;
            *pu8_var1 = bVar8 + 0x40 + b_var21;
            pu8_var1 = unaff_si + -0x78;
            *pu8_var1 = *pu8_var1 + cVar12 + (0xbf < b_var5 || CARRY1(bVar8 + 0x40, b_var21));
            puStack34 = &stack0xfffe;
            process_struct_1040_7728(
                in_struct_1,
                (&ctx.PTR_LOOP_1050_0000 + 1),
                CONCAT22(in_dx & 0xff | (bVar16 + 0x40 + bVar19) << 8, 0x4040),
                0xfba,
                ((in_NT & 1) * 0x4000
                    | (SCARRY1(bVar11, bVar13) != SCARRY1(bVar17, in_CF)) * 0x800
                    | (in_IF & 1) * 0x200
                    | (in_TF & 1) * 0x100
                    | ((in_ax & 0xff | bVar10 << 8) < 0) * 0x80
                    | (bVar10 == 0) * 0x40
                    | (in_af & 1) * 0x10
                    | ((POPCOUNT(bVar10) & 1) == 0) * 4
                    | (CARRY1(bVar11, bVar13) || CARRY1(bVar17, in_CF))),
            );
            unaff_es = (in_struct_1 >> 0x10);
            pu_var18 = in_struct_1;
        }
        *pu_var18 = 0xa0b6;
    }
    pu_var18[1] = &PTR_LOOP_1050_1038;
    return CONCAT22(unaff_es, pu_var18);
}

pub unsafe fn pass1_1038_9b72(
    param_1: i32,
    param_2: Vec<u8>,
    param_3: Vec<u8>,
    param_3_00: Vec<u8>,
) {
    let mut local_4: u16;

    pass1_1040_b040(
        CONCAT22(param_2, param_1),
        CONCAT22(param_3_00, param_3),
        (param_3_00 >> 0x10),
    );
    (param_1 + 0x128) = 0;
    CONCAT22(param_2, param_1) = 0x9efa;
    (param_1 + 2) = &PTR_LOOP_1050_1038;
    local_4 = 0;
    while {
        (param_1 + local_4 * 2 + 0x94) = 0;
        local_4 = local_4 + 1;
        local_4 < 0x4a
    } {}
    return CONCAT22(param_2, param_1);
}

pub unsafe fn pass1_1038_9b52(param_1: &mut Struct44) -> &mut Struct44 {
    let pu8_var1: Vec<u8>;
    let mut b_var2: u8;
    let mut u8_var3: u8;
    let mut cVar4: u8;
    let mut u_var5: i32;
    let mut in_cx: u16;
    let mut bVar6: u8;
    let mut in_dx: i32;
    let mut bVar7: u8;
    let mut in_bx: i32;
    let mut bVar8: u8;
    let pu_var9: *mut u16;
    let unaff_bp: *mut u16;
    let unaff_si: Vec<u8>;
    let mut unaff_ss: u16;
    let mut in_CF: u8;
    let mut bVar10: bool;
    let mut bVar11: bool;
    let mut iVar12: i32;
    let pu_var13: Vec<u8>;
    let pu_var14: Vec<u8>;
    let local_4e: u8;
    let puStack34: Vec<u8>;

    puStack34 = &stack0xfffe;
    pu_var9 = &stack0xfffe;
    pu_var13 = &stack0xfffe;
    cVar4 = '\x0f';
    while {
        unaff_bp = unaff_bp + -1;
        pu_var9 = pu_var9 + -1;
        unsafe { *pu_var9 = *unaff_bp };
        cVar4 = cVar4 + -1;
        '\0' < cVar4
    } {}
    pu_var14 = &stack0xbf2a;
    bVar8 = (in_bx >> 8);
    unaff_si[in_bx] = bVar8;
    bVar7 = (in_dx + 1 >> 8);
    b_var2 = bVar7 + bVar8;
    bVar10 = CARRY1(bVar7, bVar8) || CARRY1(b_var2, in_CF);
    u_var5 = in_dx + 1 & 0xff;
    pu8_var1 = unaff_si + in_bx;
    bVar6 = u_var5;
    unsafe {
        bVar7 = *pu8_var1;
        u8_var3 = *pu8_var1 - bVar6;
        bVar11 = *pu8_var1 < bVar6 || u8_var3 < bVar10;
        *pu8_var1 = u8_var3 - bVar10;
        if ((*pu8_var1 != 0)
            && (SBORROW1(bVar7, bVar6) != SBORROW1(u8_var3, bVar10)) == (*pu8_var1 < '\0'))
        {
            pu8_var1 = unaff_si;
            bVar10 = CARRY1(*pu8_var1, bVar8) || CARRY1(*pu8_var1 + bVar8, bVar11);
            *pu8_var1 = *pu8_var1 + bVar8 + bVar11;
            bVar7 = local_4e + in_bx;
            bVar11 = CARRY1(local_4e, in_bx) || CARRY1(bVar7, bVar10);
            local_4e = bVar7 + bVar10;
            pu8_var1 = unaff_si + -0x4f;
            bVar7 = *pu8_var1;
            u8_var3 = *pu8_var1;
            *pu8_var1 = u8_var3 + bVar8 + bVar11;
            pu8_var1 = unaff_si + -0x78;
            *pu8_var1 =
                *pu8_var1 + in_cx + (CARRY1(bVar7, bVar8) || CARRY1(u8_var3 + bVar8, bVar11));
            puStack34 = &stack0xfffe;
            pass1_1040_b040(
                CONCAT22(&stack0xbf2a, &stack0xfffe),
                CONCAT22(u_var5 | (b_var2 + in_CF) << 8, in_bx),
                in_cx,
            );
            (pu_var13 + 0x128) = 0;
            CONCAT22(pu_var14, pu_var13) = 0x9efa;
            (pu_var13 + 2) = &PTR_LOOP_1050_1038;
            iVar12 = 0;
            while {
                (pu_var13 + iVar12 * 2 + 0x94) = 0;
                iVar12 = iVar12 + 1;
                iVar12 < 0x4a
            } {}
            return CONCAT22(pu_var14, pu_var13);
        }
        if (*pu8_var1 != 0) {
            error_check_1000_17ce(param_1);
        }
    }
    return param_1;
}

pub unsafe fn pass1_1038_9ad0(param_1: &mut Struct44, param_2: u8) -> &mut Struct44 {
    pass1_1038_9a48(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1038_9a1e(
    param_1: i32,
    param_2: Vec<u8>,
    param_3: Vec<u8>,
    param_3_00: Vec<u8>,
) -> *mut u32 {
    pass1_1040_b040(
        CONCAT22(param_2, param_1),
        CONCAT22(param_3_00, param_3),
        (param_3_00 >> 0x10),
    );
    *CONCAT22(param_2, param_1) = 0x9af6;
    (param_1 + 2) = &PTR_LOOP_1050_1038;
    return CONCAT22(param_2, param_1);
}

pub unsafe fn pass1_1038_99fe(param_1: &mut Struct44) -> &mut Struct44 {
    byte * *ppu8_var1;
    let pu8_var2: Vec<u8>;
    let mut u8_var3: u8;
    let mut u8_var4: u8;
    let mut cVar5: u8;
    let mut u_var6: i32;
    let mut in_cx: u16;
    let mut bVar7: u8;
    let mut in_dx: i32;
    let mut bVar8: u8;
    let mut in_bx: i32;
    let mut bVar9: u8;
    let pu_var10: *mut u16;
    let unaff_bp: *mut u16;
    let unaff_si: Vec<u8>;
    let mut unaff_ss: u16;
    let mut in_CF: u8;
    let mut bVar11: bool;
    let mut bVar12: bool;
    let pu_var13: Vec<u8>;
    let pu_var14: Vec<u8>;
    let local_4e: u8;
    let puStack34: Vec<u8>;

    puStack34 = &stack0xfffe;
    pu_var10 = &stack0xfffe;
    pu_var13 = &stack0xfffe;
    cVar5 = '\x0f';
    while {
        unaff_bp = unaff_bp + -1;
        pu_var10 = pu_var10 + -1;
        unsafe { *pu_var10 = *unaff_bp };
        cVar5 = cVar5 + -1;
        '\0' < cVar5
    } {}
    pu_var14 = &stack0xbf2a;
    bVar9 = (in_bx >> 8);
    unaff_si[in_bx] = bVar9;
    bVar8 = (in_dx + 1 >> 8);
    u8_var3 = bVar8 + bVar9;
    bVar11 = CARRY1(bVar8, bVar9) || CARRY1(u8_var3, in_CF);
    u_var6 = in_dx + 1 & 0xff;
    pu8_var2 = unaff_si + in_bx;
    bVar7 = u_var6;
    unsafe {
        bVar8 = *pu8_var2;
        u8_var4 = *pu8_var2 - bVar7;
        bVar12 = *pu8_var2 < bVar7 || u8_var4 < bVar11;
        *pu8_var2 = u8_var4 - bVar11;
        if ((*pu8_var2 != 0)
            && (SBORROW1(bVar8, bVar7) != SBORROW1(u8_var4, bVar11)) == (*pu8_var2 < '\0'))
        {
            pu8_var2 = unaff_si;
            bVar11 = CARRY1(*pu8_var2, bVar9) || CARRY1(*pu8_var2 + bVar9, bVar12);
            *pu8_var2 = *pu8_var2 + bVar9 + bVar12;
            bVar12 = CARRY1(local_4e, in_bx);
            bVar8 = local_4e + in_bx;
            local_4e = bVar8 + bVar11;
            pu8_var2 = unaff_si + -0x4f;
            *pu8_var2 = *pu8_var2 + bVar9 + (bVar12 || CARRY1(bVar8, bVar11));
            puStack34 = &stack0xfffe;
            pass1_1040_b040(
                CONCAT22(&stack0xbf2a, &stack0xfffe),
                CONCAT22(u_var6 | (u8_var3 + in_CF) << 8, in_bx),
                in_cx,
            );
            (CONCAT22(pu_var14, pu_var13)).ptr_a_lo = 0x9af6;
            (pu_var13 + 2) = &PTR_LOOP_1050_1038;
            return CONCAT22(pu_var14, pu_var13);
        }
        ppu8_var1 = (unaff_si + 9);
        *ppu8_var1 = unaff_si + *ppu8_var1;
    }
    error_check_1000_17ce(param_1);
    return param_1;
}

pub unsafe fn pass1_1038_993a(param_1: Vec<u8>, param_2: Vec<u8>, param_1_00: i32) -> i16 {
    let mut local_6: u16;
    let mut local_4: u16;

    local_6 = 0;
    loop {
        if (0xe < local_6) {
            return 0xffff;
        }
        if ((local_6 * 0xe + 0x5a70) == param_1_00) {
            break;
        }
        local_6 = local_6 + 1;
    }
    return local_6;
}

pub unsafe fn pass1_1038_90a2(param_1: &mut Struct44, param_2: u8) -> &mut Struct44 {
    pass1_1038_8cf6(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1038_8cf6(param_1: &mut Struct44) {
    let mut u_var1: u16;

    u_var1 = (param_1 >> 0x10);
    param_1.ptr_a_lo = 0x90c8;
    (param_1 + 2) = &PTR_LOOP_1050_1038;
    pass1_1038_b6e0(ctx._g_Struct112_a, *(param_1 + 6));
    win_cleanup_func_1040_b0f8(param_1);
    return;
}

pub unsafe fn pass1_1038_8c8a(param_1: &mut Struct44, param_2: Vec<u8>) -> &mut Struct44 {
    let pu8_var1: Vec<u8>;
    let pc_var2: String;
    let mut u8_var3: u8;
    let mut cVar4: u8;
    let mut u_var5: i32;
    let mut bVar6: u8;
    let mut in_dx: i32;
    let mut bVar7: u8;
    let mut in_bx: i32;
    let mut bVar10: u8;
    let mut i_var9: i32;
    let pu_var11: *mut u16;
    let unaff_bp: *mut u16;
    let unaff_si: Vec<u8>;
    let mut unaff_DI: i32;
    let mut u_var12: u16;
    let mut unaff_ss: u16;
    let mut in_CF: u8;
    let mut bVar13: bool;
    let mut b_var14: bool;
    let ppVar15: *mut Struct2111;
    let local_4e: u8;
    let mut bVar8: u8;

    pu_var11 = &stack0xfffe;
    cVar4 = '\x0f';
    while {
        unaff_bp = unaff_bp + -1;
        pu_var11 = pu_var11 + -1;
        unsafe {
            *pu_var11 = *unaff_bp;
        }

        cVar4 = cVar4 + -1;
        '\0' < cVar4
    } {}
    bVar10 = (in_bx >> 8);
    unaff_si[in_bx] = bVar10;
    bVar8 = (in_dx + 1 >> 8);
    bVar7 = bVar8 + bVar10;
    bVar13 = CARRY1(bVar8, bVar10) || CARRY1(bVar7, in_CF);
    bVar7 = bVar7 + in_CF;
    u_var5 = in_dx + 1 & 0xff;
    pu8_var1 = unaff_si + in_bx;
    bVar6 = u_var5;
    unsafe {
        bVar8 = *pu8_var1;
        u8_var3 = *pu8_var1 - bVar6;
        b_var14 = *pu8_var1 < bVar6 || u8_var3 < bVar13;
        *pu8_var1 = u8_var3 - bVar13;
        if ((*pu8_var1 != 0)
            && (SBORROW1(bVar8, bVar6) != SBORROW1(u8_var3, bVar13)) == (*pu8_var1 < '\0'))
        {
            pu8_var1 = unaff_si;
            bVar8 = *pu8_var1;
            u8_var3 = *pu8_var1;
            *pu8_var1 = u8_var3 + bVar10 + b_var14;
            bVar13 = CARRY1(local_4e, in_bx)
                || CARRY1(
                    local_4e + in_bx,
                    CARRY1(bVar8, bVar10) || CARRY1(u8_var3 + bVar10, b_var14),
                );
            pu8_var1 = unaff_si + -0x4f;
            b_var14 = CARRY1(*pu8_var1, bVar10) || CARRY1(*pu8_var1 + bVar10, bVar13);
            *pu8_var1 = *pu8_var1 + bVar10 + bVar13;
            bVar8 = bVar7 + bVar10;
            pc_var2 = (unaff_DI + -0x75);
            *pc_var2 = *pc_var2 + bVar6 + (CARRY1(bVar7, bVar10) || CARRY1(bVar8, b_var14));
            _in(u_var5 | (bVar8 + b_var14) << 8);
            process_struct_1040_b082(param_1, CONCAT22(param_2, 0x185a));
            u_var12 = (param_1 >> 0x10);
            i_var9 = param_1;
            (i_var9 + 0x94) = 0;
            param_1.ptr_a_lo = 0x90c8;
            (i_var9 + 2) = &PTR_LOOP_1050_1038;
            ppVar15 =
                process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(unaff_DI, 0x3f));
            (i_var9 + 0x94) = ppVar15;
            (i_var9 + 0x96) = (ppVar15 >> 0x10);
            return param_1;
        }
        if (*pu8_var1 != 0) {
            error_check_1000_17ce(param_1);
        }
    }
    return param_1;
}

pub unsafe fn pass1_1038_8c08(param_1: &mut Struct44, param_2: u8) -> &mut Struct44 {
    pass1_1038_893a(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1038_893a(param_1: *mut Struct348) {
    let mut u_var1: u16;

    u_var1 = (param_1 >> 0x10);
    param_1.field_0x0 = 0x8c2e;
    (param_1 + 2) = &PTR_LOOP_1050_1038;
    pass1_1038_b6e0(ctx._g_Struct112_a, *(param_1 + 6));
    win_cleanup_func_1040_b0f8(param_1);
    return;
}

pub unsafe fn pass1_1038_8850(param_1: &mut Struct44, param_2: u8) -> &mut Struct44 {
    pass1_1038_7d5c(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1038_801a(param_1: Vec<u8>) {
    let mut u_var1: u16;
    let ppVar2: *mut Struct2111;
    let mut u_var3: u32;
    let pu_var4: Vec<u8>;
    let mut in_stack_0000fff0: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    ppVar2 = process_struct_1010_20ba(
        ctx._g_Struct372_1050_0ed0,
        CONCAT22(in_stack_0000fff0, 0x30),
    );
    u_var1 = (param_1 >> 0x10);
    u_var3 = pass1_1008_b340((param_1 + 0x94));
    pu_var4 = (u_var3 & 0xffff | ((u_var3 >> 0x10) | u_var3) << 0x10);
    if (u_var3 != 0) {
        pass1_1010_3770(ppVar2, u_var3);
        pu_var4 = pass1_1038_af40(ctx._g_Struct112_a, *(param_1 + 6), 3);
    }
    return pu_var4;
}

pub unsafe fn pass1_1038_7a76(param_1: *mut Vec<u8>) {
    let pp_var1: fn();
    let BVar2: bool;
    let struct_a: *mut Struct306;
    let lVar3: u32;
    let paVar4: *mut Struct1152;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: [u8; 4];
    let mut local_6: u32;

    pass1_1008_5784(CONCAT22(struct_a, local_a), param_1);
    while (
        lVar3 = pass1_1008_5b12(CONCAT22(struct_a, local_a)),
        lVar3 != 0,
    ) {
        pass1_1038_6a0e(lVar3);
    }
    while (
        paVar4 = pass1_1008_5b12(CONCAT22(struct_a, local_a)),
        paVar4 != 0x0,
    ) {
        BVar2 = pass1_1038_6b3c(paVar4);
        if (BVar2 != 0) {
            pp_var1 = (param_1 + 0xc);
            (**pp_var1)(&ctx.PTR_LOOP_1050_1008);
        }
    }
    pass1_1008_5784(CONCAT22(struct_a, local_a), (param_1 + 4));
    while (
        lVar3 = pass1_1008_5b12(CONCAT22(struct_a, local_a)),
        lVar3 != 0,
    ) {
        pass1_1030_affc(lVar3);
    }
    return;
}

pub unsafe fn pass1_1038_79f2(param_1: Vec<u8>, param_2: u32) {
    let pp_var1: fn();
    let pu_var2: Vec<u8>;
    
    let mut i_var3: i32;
    let mut u_var4: u16;
    let mut unaff_ss: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: [u8; 8];
    let mut local_6: u32;

    local_6 = (param_2 + 4);
    u_var4 = (param_1 >> 0x10);
    i_var3 = param_1;
    pass1_1008_5784(CONCAT22(unaff_ss, local_e), (i_var3 + 4));
    while {
        pu_var2 = local_e;
        pass1_1008_5b12(CONCAT22(unaff_ss, pu_var2));
        if ((ctx.dx_reg | pu_var2) == 0) {
            return;
        }
        (pu_var2 + 4) != local_6
    } {}
    pp_var1 = ((i_var3 + 4) + 0xc);
    (**pp_var1)(&ctx.PTR_LOOP_1050_1008, (i_var3 + 4), pu_var2, ctx.dx_reg);
    return;
}

pub unsafe fn pass1_1038_79b2(param_1: Vec<u8>, param_2: u32) {
    let pp_var1: fn();
    let struct_a_hi: *mut Struct199;
    let struct_a: *mut Struct199;
    let mut local_es_41: u16;
    let mut local_CS_6: u16;
    let mut local_DXAX_29: u32;
    let mut local_8: u16;

    local_CS_6 = 0x1000;
    process_struct_1000_179c(0x14, struct_a);
    if ((struct_a | struct_a_hi) == 0) {
        local_DXAX_29 = 0;
    } else {
        local_CS_6 = 0x1030;
        local_DXAX_29 = pass1_1030_aefa(CONCAT22(struct_a, struct_a_hi), param_2);
    }
    local_es_41 = (param_1 >> 0x10);
    pp_var1 = ((param_1 + 4) + 4);
    (**pp_var1)(local_CS_6, (param_1 + 4), local_DXAX_29);
    return;
}

pub unsafe fn pass1_1038_7964(struct_a: *mut *mut Struct1170) {
    let pu_var1: *mut u32;
    let struct_b: *mut Struct1170;
    let struct_b_hi: *mut Struct1170;
    let fn_ptr_1: fn();
    let mut uint_a: i32;

    _PTR_LOOP_1050_5a64 = 0;
    struct_b_hi = (struct_a >> 0x10);
    struct_b = struct_a;
    uint_a = struct_b.field_0x2;
    let struct_a_val = unsafe { *struct_a };
    if ((uint_a | struct_a_val) != 0) {
        fn_ptr_1 = struct_a_val;
        (**fn_ptr_1)();
    }
    pu_var1 = struct_b.field_0x4;
    uint_a = struct_b.field_0x6;
    if ((uint_a | pu_var1) != 0) {
        unsafe {
            fn_ptr_1 = *pu_var1;
            (**fn_ptr_1)();
        }
    }
    return;
}

pub unsafe fn pass1_1038_78b8(param_1: &mut Struct44, param_2: u8) -> &mut Struct44 {
    pass1_1038_6912(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1038_6e1a(param_1: u16, param_2: u16, param_1_00: *mut long) {
    let paVar1: *mut Struct493;
    let local_AX_76: *mut Struct1155;
    let BVar2: bool;
    let mut u_var3: u16;
    let mut u_var4: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_4: u16;

    local_4 = 0;
    let param_1_00_val = unsafe { *param_1_00 };
    if ((param_1_00_val == 0) && (param_1_00 == 0)) {
        return 1;
    }
    u_var4 = (param_1_00 + 2);
    local_16._1_1_ = (u_var4 >> 8);
    if (local_16._1_1_ == '\0') {
        local_4 = param_1_00;
        // goto switchD_1038_6eab_caseD_9;
    }

    paVar1 = pass1_1028_e1ec(
        ctx._PTR_LOOP_1050_65e2,
        param_1_00_val,
        (param_1_00_val >> 0x10),
    );
    local_AX_76._0_1_ = pass1_1030_6fa0(CONCAT22(u_var4, paVar1));
    local_AX_76 = CONCAT11(local_AX_76._1_1_, local_AX_76);
    if (local_AX_76 < 10) {
        match (local_AX_76) {
            0x1 => {
                local_4 = 1;
            }
            0x2 | 0x6 => {
                local_4 = 2;
            }
            0x3 | 0x7 => {
                local_4 = 3;
            }
            0x4 | 0x8 => {
                local_4 = 4;
            }
            0x5 | 0x9 => {
                // goto switchD_1038_6eab_caseD_5;
            }
        }
    } else {
        BVar2 = pass1_1008_c6ae(ctx._PTR_LOOP_1050_06e0, local_AX_76, 0x41);
        if (BVar2 != 0) {
            local_4 = 10;
            // goto switchD_1038_6eab_caseD_9;
        }
        BVar2 = pass1_1008_c6ae(ctx._PTR_LOOP_1050_06e0, local_AX_76, 0x42);
        if ((BVar2 != 0) || (local_AX_76 == (s_New_failed_in_Op__Op__ResLibr_1050_0035 + 10))) {
            local_4 = 0xb;
            // goto switchD_1038_6eab_caseD_9;
        }
        // switchD_1038_6eab_caseD_5:
        local_4 = 5;
    }
    // switchD_1038_6eab_caseD_9:
    match (local_4) {
        1 => {
            return 0x14;
        }
        2 | 7 => {
            return 0x3c;
        }
        3 | 8 => {
            return 0x78;
        }
        4 | 9 => {
            return 0xf0;
        }
        5 | 6 => {
            return 0xf;
        }
        10 => u_var3 = 0xc,
        0xb => u_var3 = 10,
        _ => {
            u_var3 = 0xffff;
        }
    }
    return u_var3;
}

pub unsafe fn pass1_1038_6d24(
    param_1: *mut Struct1153,
    param_2: *mut Vec<u8>,
    param_3: *mut u16,
    param_4: *mut Struct1154,
    param_5: u16,
) -> i32 {
    let mut iVar1: i32;
    let mut u_var2: u16;
    let mut i_var3: i32;
    let mut unaff_ss: u16;
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
    let mut local_8: u32;
    let mut uStack4: u16;

    unsafe {
        *param_2 = 0x0;
    }
    local_8 = param_4.field_0xc;
    uStack4 = param_4.field_0x10;
    pass1_1008_3eb4(
        CONCAT22(unaff_ss, &local_8),
        CONCAT22(unaff_ss, &local_e),
        CONCAT22(unaff_ss, &local_c),
        CONCAT22(unaff_ss, &local_a),
    );
    pass1_1008_3eb4(
        (param_1 & 0xffff0000 | (param_1 + 0x1a)),
        CONCAT22(unaff_ss, &local_14),
        CONCAT22(unaff_ss, &local_12),
        CONCAT22(unaff_ss, &local_10),
    );
    iVar1 = local_c - local_12;
    u_var2 = local_e - local_14;
    i_var3 = local_a - local_10;
    if (((i_var3 == 0) && (iVar1 == 0)) && (u_var2 == 0)) {
        return 0;
    }
    if ((iVar1 != 0) || (i_var3 == 0)) {
        if ((i_var3 == 0) && (iVar1 != 0)) {
            pass1_1038_6c1c(param_1, param_3, param_2, iVar1);
            return 1;
        }
        if (((i_var3 == 0) && (iVar1 == 0)) && (u_var2 != 0)) {
            iVar1 = pass1_1038_6c68(param_1, param_3, param_2, u_var2);
            if (iVar1 != 0) {
                return 1;
            }
            return iVar1;
        }
    }
    pass1_1038_6bd4(param_1, param_3, param_2, i_var3);
    return 1;
}

pub unsafe fn pass1_1038_6c68(param_1: *mut Struct1153, param_2: u32, param_3: u32, param_4: u16) {
    let mut iVar1: i32;
    let local_AX_13: *mut Struct1153;
    let paVar2: *mut Struct493;
    
    let mut u_var3: i32;
    let mut u_var4: u16;
    let ppVar5: *mut Struct2111;
    let mut u_var6: u32;
    let mut in_stack_0000ffd8: u16;
    let mut local_1e: u16;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_e: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    local_AX_13 = param_1;
    local_AX_13 = &local_AX_13.field_0x1a;
    modify_list_1008_3f62(param_2, param_1 & 0xffff0000 | ZEXT24(local_AX_13));
    ppVar5 = process_struct_1010_20ba(
        ctx._g_Struct372_1050_0ed0,
        CONCAT22(in_stack_0000ffd8, 0x2f),
    );
    u_var6 = param_1 & 0xffff0000 | &local_AX_13.field_0x1a;
    pass1_1030_627e(_PTR_LOOP_1050_5740, u_var6, (ppVar5 + 0x20));
    u_var3 = ctx.dx_reg | u_var6;
    if (u_var3 != 0) {
        paVar2 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, u_var6, ctx.dx_reg);
        u_var6 = pass1_1030_73a8(CONCAT22(u_var3, paVar2));
        iVar1 = (u_var6 + 0xc);
        if ((iVar1 == 0x47) || (iVar1 == 0x6a)) {
            u_var4 = (param_1 >> 0x10);
            iVar1 = local_AX_13.field_0x1e;
            if (param_4 < 0) {
                local_1e = iVar1 - 1;
            } else {
                local_1e = iVar1 + 1;
            }
            (param_2 + 4) = local_1e;
            pass1_1038_6b88(local_AX_13, u_var4, param_2, param_3);
        }
    }
    return;
}

pub unsafe fn pass1_1038_6bd4(
    param_1: u32,
    param_2: *mut u16,
    param_3: *mut Vec<u8>,
    param_4: i32,
) {
    let mut local_4: u16;

    modify_list_1008_3f62(param_2, param_1 & 0xffff0000 | (param_1 + 0x1a));
    if (param_4 < 0) {
        unsafe {
            local_4 = *param_2 - 1;
        }
    } else {
        unsafe {
            local_4 = *param_2 + 1;
        }
    }
    unsafe {
        *param_2 = local_4;
    }
    pass1_1038_6b88(param_1, (param_1 >> 0x10), param_2, param_3);
    return;
}

pub unsafe fn pass1_1038_6c1c(
    param_1: u32,
    param_2: *mut u16,
    param_3: *mut Vec<u8>,
    param_4: i32,
) {
    let mut iVar1: i32;
    let mut u_var2: u16;
    let mut local_4: u16;

    modify_list_1008_3f62(param_2, param_1 & 0xffff0000 | (param_1 + 0x1a));
    u_var2 = (param_2 >> 0x10);
    iVar1 = (param_2 + 2);
    if (param_4 < 0) {
        local_4 = iVar1 - 1;
    } else {
        local_4 = iVar1 + 1;
    }
    (param_2 + 2) = local_4;
    pass1_1038_6b88(param_1, (param_1 >> 0x10), param_2, param_3);
    return;
}

pub unsafe fn pass1_1038_6b88(
    param_1: u16,
    param_2: u16,
    param_1_00: u32,
    param_2_00: *mut Vec<u8>,
) {
    // ppu_var1: *mut Vec<u8>;

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
    ppu_var1 = &stack0xffee;
    pass1_1030_64ce(
        _PTR_LOOP_1050_5740,
        param_1_00,
        local_a,
        (local_a >> 0x10),
        ppu_var1,
        unaff_ss,
    );
    unsafe {
        *param_2_00 = *ppu_var1;
    }
    return;
}

pub unsafe fn pass1_1038_6b3c(param_1: *mut Struct1152) -> bool {
    let local_bx_3: *mut Struct1152;
    let mut u_var1: u16;

    u_var1 = (param_1 >> 0x10);
    local_bx_3 = param_1;
    if ((((local_bx_3.field_0xc == 0) && (local_bx_3.field_0x12 == 0))
        && (local_bx_3.field_0x14 == 0))
        && (local_bx_3.field_0xe == 0 && (local_bx_3.field_0x16 != 0)))
    {
        local_bx_3.field_0x16 = 0;
    }
    if (local_bx_3.field_0x16 == 0) {
        return 1;
    }
    return 0;
}

pub unsafe fn pass1_1038_6912(param_1: *mut *mut Struct1149) {
    let mut u_var1: i32;
    let mut u_var2: i32;
    let ppc_var3: fn();
    let pu_var4: *mut u32;
    let local_bx_5: *mut Struct1149;
    let mut u_var5: u16;
    let mut local_a: u32;

    u_var5 = (param_1 >> 0x10);
    local_bx_5 = param_1;
    unsafe {
        *param_1 = 0x78de;
    }
    local_bx_5.field_0x2 = &PTR_LOOP_1050_1038;
    u_var1 = &local_bx_5.field_0x6;
    pu_var4 = &local_bx_5.field_0x4;
    if ((u_var1 | pu_var4) != 0) {
        unsafe {
            ppc_var3 = *pu_var4;
        }
        (**ppc_var3)();
    }
    u_var1 = local_bx_5.field_0xe;
    u_var2 = local_bx_5.field_0x10;
    local_a = CONCAT22(u_var2, u_var1);
    if ((u_var2 | u_var1) != 0) {
        pass1_1020_ba7e(CONCAT22(u_var2, u_var1));
        error_check_1000_17ce(local_a);
    }
    unsafe {
        *param_1 = ctx.s_1_1050_389a;
    }
    local_bx_5.field_0x2 = &ctx.PTR_LOOP_1050_1008;
    return;
}

pub unsafe fn pass1_1038_6984(param_1: *mut Struct1150) {
    let local_bx_3: *mut Struct1150;
    let mut u_var1: u16;

    u_var1 = (param_1 >> 0x10);
    local_bx_3 = param_1;
    if (local_bx_3.field_0xc != 0) {
        ret_one_1020_c3ae();
        return;
    }
    if (local_bx_3.field_0xe != 0) {
        infinite_loop_1020_ba94(local_bx_3.field_0xe);
        return;
    }
    if (local_bx_3.field_0x12 == 0) {
        if (local_bx_3.field_0x14 == 0) {
            return;
        }
        pass1_1020_c42e(local_bx_3.field_0x14);
    } else {
        switch_statement_1020_c3b4(local_bx_3.field_0x12);
    }
    return;
}

pub unsafe fn pass1_1038_69fe(param_1: Vec<u8>) {
    (param_1 + 0x28) = 0;
    return;
}

pub unsafe fn pass1_1038_6a0e(param_1: *mut Struct1153) {
    let piVar1: *mut i32;
    let mut u_var2: u32;
    let paVar3: *mut Struct493;
    let pu_var4: Vec<u8>;
    let b_var5: bool;
    let mut u_var6: u16;
    let mut in_dx: u16;
    
    let mut u_var7: i32;
    let local_bx_4: *mut Struct1151;
    let mut u_var8: u16;
    let mut unaff_ss: u16;
    let mut u_var9: u32;
    let mut local_20: u16;
    let mut local_1e: u16;
    let mut local_1c: u16;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: [u8; 4];
    let mut local_c: [u8; 6];
    let mut local_6: u16;
    let mut local_4: u16;

    u_var8 = (param_1 >> 0x10);
    local_bx_4 = param_1;
    if (local_bx_4.field_0x28 == 0) {
        u_var2 = local_bx_4.field_0x20;
        paVar3 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, u_var2, (u_var2 >> 0x10));
        _local_6 = CONCAT22(in_dx, paVar3);
        piVar1 = &local_bx_4.field_0x24;
        unsafe {
            *piVar1 = *piVar1 + 0x3c;
        }
        zero_list_1008_3e38(CONCAT22(unaff_ss, local_c));
        loop {
            pu_var4 = local_10;
            pass1_1038_6d24(
                param_1,
                CONCAT22(unaff_ss, pu_var4),
                CONCAT22(unaff_ss, local_c),
                _local_6,
                (_local_6 >> 0x10),
            );
            if (pu_var4 == 0x0) {
                pass1_1010_8fba(local_bx_4.field_0x4);
                _local_16 = CONCAT22(ctx.dx_reg, pu_var4);
                u_var7 = ctx.dx_reg | pu_var4;
                if (u_var7 == 0) {
                    u_var2 = local_bx_4.field_0x20;
                    paVar3 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, u_var2, (u_var2 >> 0x10));
                    pass1_1038_7356(param_1, paVar3, u_var7);
                    return;
                }
                u_var9 = pass1_1030_73a8(_local_6);
                b_var5 = pass1_1008_c6ae(ctx._PTR_LOOP_1050_06e0, (u_var9 + 0xc), 0x40);
                if (b_var5 != 0) {
                    local_bx_4.field_0x28 = 1;
                    local_bx_4.field_0x20 = _local_16;
                    return;
                }
                local_bx_4.field_0x20 = _local_16;
                u_var7 = ctx.dx_reg;
                paVar3 =
                    pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, &local_bx_4.field_0x20, ctx.dx_reg);
                _local_6 = CONCAT22(u_var7, paVar3);
            }
            u_var6 = pass1_1038_6e1a(local_bx_4, u_var8, CONCAT22(unaff_ss, local_10));
            if (local_bx_4.field_0x24 < u_var6) {
                break;
            }
            piVar1 = &local_bx_4.field_0x24;
            unsafe {
                *piVar1 = *piVar1 - u_var6;
            }
            modify_list_1008_3f62(
                (param_1 & 0xffff0000 | &local_bx_4.field_0x1a),
                CONCAT22(unaff_ss, local_c),
            );
        }
    }
    return;
}

pub unsafe fn pass1_1038_6838(
    param_1: *mut *mut Struct1148,
    param_2: u32,
    param_3: u16,
    param_4: u16,
    param_5: u32,
) -> i32 {
    let paVar1: *mut Struct493;
    let paVar2: *mut Struct493;
    let mut u_var3: i32;
    let pu_var4: *mut u16;
    let mut in_dx: u16;

    
    let local_bx_5: *mut Struct1148;
    let mut u_var5: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    u_var5 = (param_1 >> 0x10);
    local_bx_5 = param_1;
    unsafe {
        *param_1 = ctx.s_1_1050_389a;
    }
    local_bx_5.field_0x2 = &ctx.PTR_LOOP_1050_1008;
    &local_bx_5.field_0x4 = 0;
    local_bx_5.field_0x8 = param_5;
    local_bx_5.field_0xc = 0;
    local_bx_5.field_0xe = 0;
    local_bx_5.field_0x12 = 0;
    local_bx_5.field_0x14 = param_3;
    local_bx_5.field_0x16 = param_2;
    zero_list_1008_3e38((param_1 & 0xffff0000 | &local_bx_5.field_0x1a));
    &local_bx_5.field_0x20 = 0;
    local_bx_5.field_0x24 = 0;
    local_bx_5.field_0x26 = param_4;
    local_bx_5.field_0x28 = 0;
    unsafe {
        *param_1 = 0x78de;
    }
    local_bx_5.field_0x2 = &PTR_LOOP_1050_1038;
    paVar1 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, param_5, (param_5 >> 0x10));
    paVar2 = paVar1;
    pass1_1030_6d4e(paVar1, in_dx);
    local_bx_5.field_0x4 = paVar2;
    &local_bx_5.field_0x6 = ctx.dx_reg;
    pu_var4 = (param_1 & 0xffff0000 | &local_bx_5.field_0x1a);
    modify_list_1008_3f62(pu_var4, CONCAT22(in_dx, &paVar1.field_0xc));
    u_var3 = pu_var4;
    pass1_1010_8fba(&local_bx_5.field_0x4);
    local_bx_5.field_0x20 = u_var3;
    local_bx_5.field_0x22 = ctx.dx_reg;
    return;
}

pub unsafe fn pass1_1038_6590(
    param_1: *mut *mut Struct1145,
    param_2: Vec<u8>,
    param_3: Vec<u8>,
    param_4: Vec<u8>,
    param_5: Vec<u8>,
    param_6: Vec<u8>,
) -> i32 {
    let paVar1: *mut Struct493;
    let paVar2: *mut Struct493;
    let mut u_var3: i32;
    let pu_var4: *mut u16;
    let mut in_dx: u16;

    
    let local_bx_6: *mut Struct1145;
    let mut u_var5: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    u_var5 = (param_1 >> 0x10);
    local_bx_6 = param_1;
    unsafe {
        *param_1 = ctx.s_1_1050_389a;
    }
    local_bx_6.field_0x2 = &ctx.PTR_LOOP_1050_1008;
    &local_bx_6.field_0x4 = 0;
    local_bx_6.field_0x8 = param_6;
    local_bx_6.field_0xc = param_4;
    local_bx_6.field_0xe = 0;
    local_bx_6.field_0x12 = 0;
    local_bx_6.field_0x14 = 0;
    local_bx_6.field_0x16 = param_2;
    local_bx_6.field_0x18 = param_3;
    zero_list_1008_3e38((param_1 & 0xffff0000 | &local_bx_6.field_0x1a));
    &local_bx_6.field_0x20 = 0;
    local_bx_6.field_0x24 = 0;
    local_bx_6.field_0x26 = param_5;
    local_bx_6.field_0x28 = 0;
    unsafe {
        *param_1 = 0x78de;
    }
    local_bx_6.field_0x2 = &PTR_LOOP_1050_1038;
    paVar1 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, param_6, (param_6 >> 0x10));
    paVar2 = paVar1;
    pass1_1030_6d4e(paVar1, in_dx);
    local_bx_6.field_0x4 = paVar2;
    &local_bx_6.field_0x6 = ctx.dx_reg;
    pu_var4 = (param_1 & 0xffff0000 | &local_bx_6.field_0x1a);
    modify_list_1008_3f62(pu_var4, CONCAT22(in_dx, &paVar1.field_0xc));
    u_var3 = pu_var4;
    pass1_1010_8fba(&local_bx_6.field_0x4);
    local_bx_6.field_0x20 = u_var3;
    local_bx_6.field_0x22 = ctx.dx_reg;
    return;
}

pub unsafe fn pass1_1038_666e(
    param_1: *mut *mut Struct1146,
    param_2: u32,
    param_3: u16,
    param_4: u32,
) {
    let paVar1: *mut Struct493;
    let paVar2: *mut Struct493;
    let mut u_var3: i32;
    let pu_var4: *mut u16;
    let mut in_dx: u16;

    
    
    let local_bx_5: *mut Struct1146;
    let mut u_var5: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    u_var5 = (param_1 >> 0x10);
    local_bx_5 = param_1;
    unsafe { *param_1 = ctx.s_1_1050_389a };
    local_bx_5.field_0x2 = &ctx.PTR_LOOP_1050_1008;
    &local_bx_5.field_0x4 = 0;
    local_bx_5.field_0x8 = param_4;
    local_bx_5.field_0xc = 0;
    local_bx_5.field_0xe = param_2;
    local_bx_5.field_0x12 = 0;
    local_bx_5.field_0x14 = 0;
    local_bx_5.field_0x18 = 0;
    local_bx_5.field_0x16 = 0;
    zero_list_1008_3e38((param_1 & 0xffff0000 | &local_bx_5.field_0x1a));
    &local_bx_5.field_0x20 = 0;
    local_bx_5.field_0x24 = 0;
    local_bx_5.field_0x26 = param_3;
    local_bx_5.field_0x28 = 0;
    unsafe {
        *param_1 = 0x78de;
    }
    local_bx_5.field_0x2 = &PTR_LOOP_1050_1038;
    paVar1 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, param_4, (param_4 >> 0x10));
    paVar2 = paVar1;
    pass1_1030_6d4e(paVar1, in_dx);
    local_bx_5.field_0x4 = paVar2;
    &local_bx_5.field_0x6 = ctx.dx_reg;
    pu_var4 = (param_1 & 0xffff0000 | &local_bx_5.field_0x1a);
    modify_list_1008_3f62(pu_var4, CONCAT22(in_dx, &paVar1.field_0xc));
    u_var3 = pu_var4;
    pass1_1010_8fba(&local_bx_5.field_0x4);
    local_bx_5.field_0x20 = u_var3;
    local_bx_5.field_0x22 = ctx.dx_reg;
    infinite_loop_1020_ba94(param_2);
    local_bx_5.field_0x16 = u_var3;
    local_bx_5.field_0x18 = ctx.dx_reg;
    return;
}

pub unsafe fn pass1_1038_675c(
    param_1: *mut *mut Struct1147,
    param_2: u32,
    param_3: u16,
    param_4: u16,
    param_5: u32,
) -> i32 {
    let paVar1: *mut Struct493;
    let paVar2: *mut Struct493;
    let mut u_var3: i32;
    let pu_var4: *mut u16;
    let mut in_dx: u16;

    
    let local_bx_5: *mut Struct1147;
    let mut u_var5: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    u_var5 = (param_1 >> 0x10);
    local_bx_5 = param_1;
    unsafe {
        *param_1 = ctx.s_1_1050_389a;
    }
    local_bx_5.field_0x2 = &ctx.PTR_LOOP_1050_1008;
    &local_bx_5.field_0x4 = 0;
    local_bx_5.field_0x8 = param_5;
    local_bx_5.field_0xc = 0;
    local_bx_5.field_0xe = 0;
    local_bx_5.field_0x12 = param_3;
    local_bx_5.field_0x14 = 0;
    local_bx_5.field_0x16 = param_2;
    zero_list_1008_3e38((param_1 & 0xffff0000 | &local_bx_5.field_0x1a));
    &local_bx_5.field_0x20 = 0;
    local_bx_5.field_0x24 = 0;
    local_bx_5.field_0x26 = param_4;
    local_bx_5.field_0x28 = 0;
    unsafe {
        *param_1 = 0x78de;
    }
    local_bx_5.field_0x2 = &PTR_LOOP_1050_1038;
    paVar1 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, param_5, (param_5 >> 0x10));
    paVar2 = paVar1;
    pass1_1030_6d4e(paVar1, in_dx);
    local_bx_5.field_0x4 = paVar2;
    &local_bx_5.field_0x6 = ctx.dx_reg;
    pu_var4 = (param_1 & 0xffff0000 | &local_bx_5.field_0x1a);
    modify_list_1008_3f62(pu_var4, CONCAT22(in_dx, &paVar1.field_0xc));
    u_var3 = pu_var4;
    pass1_1010_8fba(&local_bx_5.field_0x4);
    local_bx_5.field_0x20 = u_var3;
    local_bx_5.field_0x22 = ctx.dx_reg;
    return;
}

pub unsafe fn pass1_1038_64de(param_1: &mut Struct44, param_2: u8) -> &mut Struct44 {
    pass1_1038_33f8(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1038_5b3c(
    param_1: Vec<u8>,
    param_2: Vec<u8>,
    param_1_00: u32,
    param_2_00: Vec<u8>,
) {
    let pp_var1: fn();
    let mut u_var2: u32;
    let mut u_var3: u32;
    let b_var4: bool;
    let local_SI_152: *mut Struct1138;
    let mut u_var5: u16;
    let mut local_6: u32;

    local_6 = 0;
    while (local_6 < param_1_00) {
        u_var5 = (param_2_00 >> 0x10);
        local_SI_152 = param_2_00;
        if ((((local_SI_152 + local_6 * 4) != 0)
            && (
                u_var2 = (local_SI_152 + local_6 * 4),
                b_var4 = pass1_1008_c6ae(ctx._PTR_LOOP_1050_06e0, (u_var2 + 0xc), 0x2d),
                b_var4 != 0,
            ))
            && (
                pp_var1 = ((local_SI_152 + local_6 * 4) + 0x50),
                (**pp_var1)(),
                b_var4 != 0,
            ))
        {
            u_var2 = (local_SI_152 + local_6 * 4);
            u_var3 = (local_SI_152 + local_6 * 4);
            (u_var3 + 0x1a) = (u_var2 + 0x1a) | 1;
            pp_var1 = ((local_SI_152 + local_6 * 4) + 0x28);
            (**pp_var1)();
        }
        local_6 = local_6 + 1;
    }
    return;
}

pub unsafe fn pass1_1038_5be8(param_1: Vec<u8>, param_2: i32, param_3: u16, param_4: u32) {
    let mut u_var1: u16;
    let mut u_var2: u16;
    let paVar3: *mut Struct493;
    let b_var4: bool;
    let mut u_var5: i32;
    let mut u_var6: i32;
    let lVar7: u32;
    let mut u_var8: u32;
    let mut local_1e: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_4: u16;
    let temp_7ffe889d3b9: *mut Struct1139;

    lVar7 = pass1_1030_627e(_PTR_LOOP_1050_5740, param_4, (param_1 + 8));
    u_var5 = (lVar7 >> 0x10);
    u_var6 = u_var5 | lVar7;
    if (lVar7 != 0) {
        paVar3 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, lVar7, u_var5);
        local_a = CONCAT22(u_var6, paVar3);
        local_e = 0x7a;
        if (0 < (param_4 + 4)) {
            if (param_3 == 0x7b) {
                param_3 = 0x7e;
            } else {
                if (param_3 == 0x7c) {
                    param_3 = 0x7d;
                }
            }
            local_e = 0x7f;
        }
        u_var8 = pass1_1030_73a8(local_a);
        u_var2 = (u_var8 >> 0x10);
        temp_7ffe889d3b9 = u_var8;
        if ((((temp_7ffe889d3b9.field_0x1a & param_2) == 0)
            && ((
                u_var1 = temp_7ffe889d3b9.field_0xc,
                u_var1 == local_e || (u_var1 == param_3),
            ) || (
                b_var4 = pass1_1008_c6ae(ctx._PTR_LOOP_1050_06e0, u_var1, 0x2b),
                b_var4 != 0,
            )))
            && (temp_7ffe889d3b9.field_0x12 != 7))
        {
            temp_7ffe889d3b9.field_0x1a = temp_7ffe889d3b9.field_0x1a | param_2;
            return (&ctx.PTR_LOOP_1050_0000 + 1);
        }
    }
    return 0x0;
}

pub unsafe fn pass1_1038_5cc6(
    param_1: Vec<u8>,
    param_2: u32,
    param_3: Vec<u8>,
    param_4: Vec<u8>,
    param_5: u16,
    uparam_6: i32,
) -> i32 {
    let mut u_var1: u32;
    let mut u_var2: u32;
    let pu_var3: Vec<u8>;
    let mut u_var4: u16;
    let mut unaff_ss: u16;
    let mut local_26: u16;
    let mut local_20: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u32;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_4: u16;

    zero_list_1008_3e38(CONCAT22(unaff_ss, &local_a));
    while {
        local_4 = 0;
        local_e = 0;
        while (local_e < param_2) {
            u_var4 = (param_4 >> 0x10);
            if ((local_e * 4 + param_4) != 0) {
                u_var1 = (local_e * 4 + param_4);
                modify_list_1008_3f62(
                    CONCAT22(unaff_ss, &local_a),
                    u_var1 & 0xffff0000 | (u_var1 + 0xc),
                );
                pass1_1008_3eb4(
                    CONCAT22(unaff_ss, &local_a),
                    CONCAT22(unaff_ss, &local_14),
                    CONCAT22(unaff_ss, &local_12),
                    CONCAT22(unaff_ss, &local_10),
                );
                if (local_14 == param_5) {
                    u_var4 = (param_3 >> 0x10);
                    if (((local_e * 4 + param_3) != 0)
                        && (
                            u_var2 = (local_e * 4 + param_3),
                            ((u_var2 + 0x1a) & param_6) != 0,
                        ))
                    {
                        local_8 = local_12 - 1;
                        pu_var3 =
                            pass1_1038_5be8(param_1, param_6, 0x7b, CONCAT22(unaff_ss, &local_a));
                        if (pu_var3 != 0x0) {
                            local_4 = 1;
                        }
                        local_8 = local_12 + 1;
                        pu_var3 =
                            pass1_1038_5be8(param_1, param_6, 0x7b, CONCAT22(unaff_ss, &local_a));
                        if (pu_var3 != 0x0) {
                            local_4 = 1;
                        }
                        local_8 = local_12;
                        local_a = local_10 - 1;
                        pu_var3 =
                            pass1_1038_5be8(param_1, param_6, 0x7c, CONCAT22(unaff_ss, &local_a));
                        if (pu_var3 != 0x0) {
                            local_4 = 1;
                        }
                        local_a = local_10 + 1;
                        pu_var3 =
                            pass1_1038_5be8(param_1, param_6, 0x7c, CONCAT22(unaff_ss, &local_a));
                        if (pu_var3 != 0x0) {
                            local_4 = 1;
                        }
                    }
                }
            }
            local_e = local_e + 1;
        }
        local_4 != 0
    } {}
    return;
}

pub unsafe fn pass1_1038_5a96(
    param_1: Vec<u8>,
    param_2: Vec<u8>,
    param_1_00: u32,
    param_2_00: Vec<u8>,
) {
    let pp_var1: fn();
    let mut u_var2: u32;
    let BVar3: bool;
    let local_SI_146: *mut Struct1137;
    let mut local_es_146: u16;
    let mut local_6: u32;
    let mut temp_5fa6353e1c: u32;

    local_6 = 0;
    while (local_6 < param_1_00) {
        local_es_146 = (param_2_00 >> 0x10);
        local_SI_146 = param_2_00;
        if (((local_SI_146 + local_6 * 4) != 0)
            && (
                temp_5fa6353e1c = (local_SI_146 + local_6 * 4),
                BVar3 = pass1_1008_c6ae(ctx._PTR_LOOP_1050_06e0, (temp_5fa6353e1c + 0xc), 0x2c),
                BVar3 != 0,
            ))
        {
            pp_var1 = ((local_SI_146 + local_6 * 4) + 0x54);
            (**pp_var1)();
            if (BVar3 != 0) {
                u_var2 = (local_SI_146 + local_6 * 4);
                (u_var2 + 0x1a) = 3;
                pp_var1 = ((local_SI_146 + local_6 * 4) + 0x28);
                (**pp_var1)();
                u_var2 = (local_SI_146 + local_6 * 4);
                (u_var2 + 0x1a) = 2;
            }
        }
        local_6 = local_6 + 1;
    }
    return;
}

pub unsafe fn pass1_1038_5a16(
    param_1: u16,
    param_2: u16,
    param_1_00: u32,
    param_2_00: *mut Struct1135,
) -> i32 {
    let pp_var1: fn();
    let mut u_var2: u32;
    let local_AX_36: *mut Struct1136;
    let local_SI_109: *mut Struct1135;
    let mut u_var3: u16;
    let mut local_6: u32;

    local_6 = 0;
    while (local_6 < param_1_00) {
        u_var3 = (param_2_00 >> 0x10);
        local_SI_109 = param_2_00;
        if (((local_SI_109 + local_6 * 4) != 0)
            && (
                u_var2 = (local_SI_109 + local_6 * 4),
                local_AX_36 = pass1_1008_c6ae(ctx._PTR_LOOP_1050_06e0, (u_var2 + 0xc), 0x2f),
                local_AX_36 != 0x0,
            ))
        {
            u_var2 = (local_SI_109 + local_6 * 4);
            (u_var2 + 0x1a) = 3;
            pp_var1 = ((local_SI_109 + local_6 * 4) + 0x28);
            (**pp_var1)();
        }
        local_6 = local_6 + 1;
    }
    return;
}

pub unsafe fn pass1_1038_58e6(
    param_1: u16,
    param_2: u16,
    param_1_00: u32,
    param_2_00: *mut Struct1133,
    param_5: Vec<u8>,
    param_6: u16,
) {
    let mut u_var1: i32;
    let ppc_var2: fn();
    let mut u_var3: u32;
    let b_var4: bool;
    let pu_var5: *mut u32;
    let paVar6: *mut Struct493;

    let mut u_var7: u16;
    let local_SI_123: *mut Struct1133;
    let mut u_var8: u16;
    let mut u_var9: u16;
    let mut unaff_ss: u16;
    let mut u_var10: u32;
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
    let mut local_12: u32;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_6: u32;
    let temp_5fbadbcbba: *mut Struct1134;

    local_6 = 0;
    while (local_6 < param_1_00) {
        u_var9 = (param_2_00 >> 0x10);
        local_SI_123 = param_2_00;
        if (((local_SI_123 + local_6 * 4) != 0)
            && (
                u_var3 = (local_SI_123 + local_6 * 4),
                b_var4 = pass1_1008_c6ae(ctx._PTR_LOOP_1050_06e0, (u_var3 + 0xc), 0x2e),
                b_var4 != 0,
            ))
        {
            u_var8 = (param_5 >> 0x10);
            temp_5fbadbcbba = (local_6 * 4 + param_5);
            u_var8 = (local_6 * 4 + param_5 + 2);
            local_12 = temp_5fbadbcbba.field_0xc;
            local_c = temp_5fbadbcbba.field_0x10;
            local_e = local_c;
            if (local_c == param_6) {
                local_e = local_c - 1;
                u_var10 = pass1_1028_bb24(*(local_SI_123 + local_6 * 4));
                pu_var5 = &local_12;
                pass1_1030_627e(_PTR_LOOP_1050_5740, CONCAT22(unaff_ss, pu_var5), u_var10);
                u_var7 = ctx.dx_reg;
                paVar6 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, pu_var5, ctx.dx_reg);
                if ((u_var7 | paVar6) != 0) {
                    u_var10 = pass1_1030_73a8(CONCAT22(u_var7, paVar6));
                    u_var1 = (u_var10 + 0x1a);
                    if (((u_var1 & 2) != 0) && ((u_var1 & 1) != 0)) {
                        u_var3 = (local_SI_123 + local_6 * 4);
                        (u_var3 + 0x1a) = 3;
                        ppc_var2 = ((local_SI_123 + local_6 * 4) + 0x28);
                        ppc_var2();
                    }
                }
            }
        }
        local_6 = local_6 + 1;
    }
    return;
}

pub unsafe fn pass1_1038_5860(param_1: Vec<u8>, param_2: u16, param_2_00: Vec<u8>, param_4: i32) {
    let pp_var1: fn();
    let mut u_var2: u32;
    let pu_var3: Vec<u8>;
    let mut u_var4: u32;
    
    
    let local_bx_19: *mut Struct1132;
    let mut u_var5: u16;
    let mut local_e: u32;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;

    if (param_4 == 0) {
        u_var5 = (param_1 >> 0x10);
        local_bx_19 = param_1;
        pp_var1 = (local_bx_19.field_0xc + 0x10);
        pu_var3 = param_2_00;
        (**pp_var1)();
        u_var2 = pu_var3 & 0xffff | ctx.dx_reg << 0x10;
        local_e = 0;
        while (local_e < u_var2) {
            pp_var1 = (local_bx_19.field_0xc + 4);
            u_var4 = u_var2;
            (**pp_var1)();
            local_6._0_2_ = param_2_00;
            if ((u_var4 == local_6)
                && (
                    local_6._2_2_ = (param_2_00 >> 0x10),
                    ctx.dx_reg == local_6._2_2_,
                ))
            {
                return;
            }
            local_e = local_e + 1;
        }
        pp_var1 = (local_bx_19.field_0xc + 0xc);
        (**pp_var1)();
    }
    return;
}

pub unsafe fn pass1_1038_57dc(param_1: Vec<u8>, param_2: Vec<u8>, param_3: *mut Struct1129) {
    let mut local_es_4: u16;

    local_es_4 = (param_1 >> 0x10);
    *(param_1 + param_3 * 4 + 0x1a2) = param_2 + (param_1 + 0x1a2 + param_3 * 4);
    return;
}

pub unsafe fn pass1_1038_5804(param_1: Vec<u8>, param_2: libc::c_long, param_3: *mut Struct1130) {
    let mut u_var1: u16;

    u_var1 = (param_1 >> 0x10);
    (param_1 + param_3 * 4 + 0x1a2) = (param_1 + 0x1a2 + param_3 * 4) - param_2;
    return;
}

pub unsafe fn pass1_1038_582c(param_1: *mut Struct1131, param_2: u32) {
    let pu_var1: *mut u32;
    let mut u_var2: i32;
    let ppc_var3: fn();
    let local_bx_4: *mut Struct1131;
    let mut u_var4: u16;

    u_var4 = (param_1 >> 0x10);
    local_bx_4 = param_1;
    pu_var1 = local_bx_4.field_0x14;
    u_var2 = &local_bx_4.field_0x16;
    if ((u_var2 | pu_var1) != 0) {
        unsafe {
            ppc_var3 = *pu_var1;
        }
        (**ppc_var3)();
    }
    &local_bx_4.field_0x14 = param_2;
    return;
}

pub unsafe fn pass1_1038_57c0(param_1: Vec<u8>) {
    pass1_1000_4906((param_1 & 0xffff0000 | (param_1 + 0x14e)), 0, 0x54);
    return;
}

pub unsafe fn pass1_1038_5770(param_1: Vec<u8>, param_2: libc::c_long, param_3: *mut Struct1127) {
    let mut local_es_4: u16;

    local_es_4 = (param_1 >> 0x10);
    (param_1 + param_3 * 4 + 0xba) = (param_1 + 0xba + param_3 * 4) + param_2;
    return;
}

pub unsafe fn pass1_1038_5798(param_1: Vec<u8>, param_2: u32, param_3: *mut Struct1128) {
    let mut local_es_4: u16;

    local_es_4 = (param_1 >> 0x10);
    (param_1 + param_3 * 4 + 0x14e) = (param_1 + 0x14e + param_3 * 4) + param_2;
    return;
}

pub unsafe fn pass1_1038_565e(param_1: *mut Struct1124) {
    let local_bx_4: *mut Struct1124;
    let mut u_var1: u16;
    let mut unaff_ss: u16;
    let mut u_var2: u32;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: [u8; 2];

    u_var1 = (param_1 >> 0x10);
    local_bx_4 = param_1;
    u_var2 = pass1_1030_8e3c(CONCAT22(unaff_ss, local_4), local_bx_4.field_0x4);
    pass1_1038_582c(param_1, u_var2);
    return CONCAT22(local_bx_4.field_0x16, local_bx_4.field_0x14);
}

pub unsafe fn pass1_1038_5694(param_1: u32, param_2: u32, param_3: *mut Struct1125) {
    let mut u_var1: u16;

    u_var1 = (param_1 >> 0x10);
    (param_1 + param_3 * 4 + 0x26) = (param_1 + 0x26 + param_3 * 4) + param_2;
    return;
}

pub unsafe fn pass1_1038_56ba(param_1: Vec<u8>) {
    pass1_1000_4906((param_1 & 0xffff0000 | (param_1 + 0x26)), 0, 0x94);
    return;
}

pub unsafe fn pass1_1038_56d6(param_1: Vec<u8>, param_2: u16) {
    let pp_var1: fn();
    let u_var2: u8;
    let local_AX_14: *mut Struct1126;
    let mut u_var3: u16;
    let paVar4: *mut Struct493;
    let extraout_var: u32;
    let mut u_var5: u32;

    let mut u_var6: u16;
    
    let mut u_var7: i32;
    let mut u_var8: u16;
    let mut u_var9: u16;
    let mut local_c: u16;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    local_AX_14 = param_1;
    local_AX_14 = &local_AX_14.field_0xba;
    u_var9 = 0x1000;
    u_var2 = pass1_1000_4906((param_1 & 0xffff0000 | ZEXT24(local_AX_14)), 0, 0x94);
    u_var3 = CONCAT31(extraout_var, u_var2);
    if (param_2 != 0) {
        u_var8 = (param_1 >> 0x10);
        if (local_AX_14.field_0xc == 0) {
            u_var3 = 0;
            u_var6 = 0;
        } else {
            pp_var1 = (local_AX_14.field_0xc + 0x10);
            (**pp_var1)();
            u_var6 = ctx.dx_reg;
        }
        _local_6 = CONCAT22(u_var6, u_var3);
        local_a = 0;
        while (local_a < _local_6) {
            pp_var1 = (local_AX_14.field_0xc + 4);
            u_var5 = _local_6;
            (**pp_var1)(u_var9, local_AX_14.field_0xc);
            u_var7 = ctx.dx_reg | u_var5;
            if (u_var7 != 0) {
                paVar4 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, u_var5, ctx.dx_reg);
                u_var9 = 0x1030;
                pass1_1030_72d0(CONCAT22(u_var7, paVar4));
            }
            local_a = local_a + 1;
        }
    }
    return;
}

pub unsafe fn pass1_1038_53ba(param_1: Vec<u8>, param_2: *mut Struct1122) {
    let mut u_var1: u16;
    let mut local_a: u32;
    let mut local_6: u32;

    u_var1 = (param_1 >> 0x10);
    if ((param_1 + 0x1a2 + param_2 * 4) < (param_1 + 0x14e + param_2 * 4)) {
        return;
    }
    return;
}

pub unsafe fn pass1_1038_540a() {
    let mut local_e: u32;
    let mut local_a: u32;
    let mut local_6: u32;

    return;
}

pub unsafe fn pass1_1038_5464(param_1: *mut Struct1123) {
    let mut u_var1: u32;
    let ppc_var2: fn();

    let mut u_var3: i32;
    let mut u_var4: u16;
    let pa_var5: *mut Struct493;
    let mut u_var6: u32;

    
    
    let mut u_var7: i32;
    
    
    let mut extraout_dx_04: u16;
    let mut i_var8: i32;
    let mut i_var9: i32;
    let mut u_var10: u16;
    let mut u_var11: u16;
    let mut unaff_cs: u16;
    let mut unaff_ss: u16;
    let local_42: *mut Struct1123;
    let mut uStack64: u16;
    let mut local_34: u16;
    let mut local_32: u16;
    let mut local_30: u16;
    let mut local_2e: u32;
    let mut local_2a: u16;
    let mut local_28: u16;
    let mut local_22: u32;
    let mut local_1e: u16;
    let mut local_1c: u16;
    let mut local_1a: u32;
    let mut local_16: u32;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u32;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;
    let temp_3fa8023d0d: *mut Struct1123;

    pass1_1038_56ba(param_1);
    pass1_1038_57c0(param_1);
    u_var10 = (param_1 >> 0x10);
    i_var8 = param_1;
    if ((i_var8 + 0xc) == 0) {
        in_ax = 0;
        u_var11 = 0;
    } else {
        ppc_var2 = ((i_var8 + 0xc) + 0x10);
        ppc_var2();
        u_var11 = ctx.dx_reg;
    }
    local_a = CONCAT22(u_var11, in_ax);
    local_e = 0;
    while (local_e < local_a) {
        local_42 = local_e;
        uStack64 = (local_e >> 0x10);
        u_var1 = (i_var8 + 0xc);
        ppc_var2 = ((i_var8 + 0xc) + 4);
        u_var6 = local_a;
        ppc_var2(unaff_cs, u_var1, (u_var1 >> 0x10), local_42, uStack64);
        local_12 = u_var6;
        u_var7 = ctx.dx_reg | local_12;
        local_10 = ctx.dx_reg;
        if (u_var7 != 0) {
            unaff_cs = SUB42(&PTR_LOOP_1050_1028, 0);
            pa_var5 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, local_12, ctx.dx_reg);
            local_16 = CONCAT22(u_var7, pa_var5);
            local_1a = &pa_var5[1].field_0x4;
            if ((&pa_var5[1].field_0x6 | local_1a) == 0) {
                local_1c = 0;
            } else {
                local_1c = (local_1a + 4);
            }
            local_1e = 0;
            while (local_1e < local_1c) {
                unaff_cs = 0x1020;
                pass1_1020_bb16(
                    local_1a,
                    CONCAT13((unaff_ss >> 8), CONCAT12(unaff_ss, &local_2e)),
                    CONCAT22(unaff_ss, &local_2a),
                    local_1e,
                );
                if (CONCAT22(local_2e._2_2_, local_2e) != 0) {
                    pass1_1038_5694(param_1, CONCAT22(local_2e._2_2_, local_2e), local_2a);
                }
                local_1e = local_1e + 1;
            }
            u_var11 = (local_16 >> 0x10);
            local_22 = (local_16 + 0x1e);
            u_var7 = (local_16 + 0x20);
            u_var3 = u_var7 | local_22;
            if (u_var3 == 0) {
                u_var3 = 0;
            } else {
                ppc_var2 = (local_22 + 0x10);
                ppc_var2(unaff_cs, local_22, u_var7);
            }
            local_1e = 0;
            local_1c = u_var3;
            while (local_1e < local_1c) {
                ppc_var2 = (local_22 + 4);
                u_var4 = local_1c;
                ppc_var2(unaff_cs, local_22, (local_22 >> 0x10), local_1e, 0);
                u_var7 = ctx.dx_reg | u_var4;
                local_2a = u_var4;
                local_28 = ctx.dx_reg;
                if (u_var7 != 0) {
                    unaff_cs = SUB42(&PTR_LOOP_1050_1028, 0);
                    pa_var5 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, u_var4, ctx.dx_reg);
                    i_var9 = &pa_var5.field_0xc * 4;
                    (i_var8 + i_var9 + 0x14e) = (i_var8 + 0x14e + i_var9) + 1;
                }
                local_1e = local_1e + 1;
            }
        }
        local_e = local_e + 1;
    }
    u_var1 = (i_var8 + 0x1f6);
    local_42 = (u_var1 >> 0x10);
    u_var6 = local_a;
    pass1_1030_38f2(u_var1, local_42, 3);
    u_var7 = u_var6;
    u_var1 = (i_var8 + 0x1f6);
    local_42 = (u_var1 >> 0x10);
    local_6 = u_var7;
    local_4 = ctx.dx_reg;
    pass1_1030_38f2(u_var1, local_42, 4);
    _local_6 = CONCAT22(
        local_4 + ctx.dx_reg + CARRY2(local_6, u_var7),
        local_6 + u_var7,
    );
    if (_local_6 == 0) {
        u_var1 = (i_var8 + 0x1f6);
        local_42 = (u_var1 >> 0x10);
        pass1_1030_38f2(u_var1, local_42, 2);
        _local_6 = CONCAT22(extraout_dx_04, u_var7);
    }
    u_var1 = (i_var8 + 0x1f6);
    _local_6 = _local_6 + (u_var1 + 0x170);
    pass1_1038_5694(param_1, _local_6, (s_New_failed_in_Op__Op_1050_0020 + 4));
    return;
}

pub unsafe fn pass1_1038_45e4(param_1: *mut Struct1109) {
    let piVar1: *mut i32;
    let mut u_var2: u32;
    let ppc_var3: fn();
    let mut u_var4: u32;
    let mut in_ax: i32;
    let mut u_var5: i32;
    let mut u_var6: i32;
    let mut u_var7: i32;
    let mut i_var8: i32;
    let mut i_var9: i32;
    let pu_var10: Vec<u8>;
    
    
    
    
    let mut u_var11: i32;
    
    let local_bx_8: *mut Struct1109;
    let mut u_var12: u16;
    let mut bVar13: bool;
    let mut local_1c: u16;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    u_var12 = (param_1 >> 0x10);
    local_bx_8 = param_1;
    u_var2 = local_bx_8.field_0x1f6;
    pass1_1030_38f2(u_var2, (u_var2 >> 0x10), 2);
    u_var2 = local_bx_8.field_0x1f6;
    u_var5 = in_ax;
    pass1_1030_38f2(u_var2, (u_var2 >> 0x10), 1);
    bVar13 = in_ax < u_var5;
    in_ax = in_ax - u_var5;
    u_var2 = local_bx_8.field_0x1f6;
    pass1_1030_38f2(u_var2, (u_var2 >> 0x10), 4);
    u_var2 = local_bx_8.field_0x1f6;
    u_var6 = u_var5;
    pass1_1030_38f2(u_var2, (u_var2 >> 0x10), 3);
    u_var11 = local_bx_8.field_0x24;
    u_var7 = u_var11 + (u_var5 - u_var6);
    u_var11 = (u_var11 >> 0xf)
        + ((ctx.dx_reg - ctx.dx_reg) - (u_var5 < u_var6))
        + CARRY2(u_var11, u_var5 - u_var6)
        + ((ctx.dx_reg - ctx.dx_reg) - bVar13)
        + CARRY2(u_var7, in_ax);
    if ((u_var11 < 0) || (u_var11 < 1 && (u_var7 + in_ax == 0))) {
        i_var9 = -1;
    } else {
        i_var9 = 1;
    }
    piVar1 = &local_bx_8.field_0x24;
    unsafe {
        *piVar1 = *piVar1 + i_var9;
    }
    pu_var10 = pass1_1008_c6fa(ctx._PTR_LOOP_1050_06e0, 0x16);
    i_var8 = pu_var10;
    pass1_1038_4d6e(param_1, pu_var10 & 0xffff | u_var11 << 0x10);
    _local_16 = CONCAT22(ctx.dx_reg, i_var8);
    u_var4 = *_local_16;
    ppc_var3 = u_var4 + 8;
    i_var9 = i_var8;
    (**ppc_var3)(&ctx.PTR_LOOP_1050_1008, i_var8, ctx.dx_reg);
    if (_local_16 != 0x0) {
        ppc_var3 = u_var4;
        (**ppc_var3)(&ctx.PTR_LOOP_1050_1008, i_var8, ctx.dx_reg, 1);
    }
    piVar1 = &local_bx_8.field_0x24;
    unsafe {
        *piVar1 = *piVar1 + i_var9 * 2;
    }
    i_var9 = local_bx_8.field_0x24;
    if (100 < i_var9) {
        i_var9 = 100;
    }
    local_bx_8.field_0x24 = i_var9;
    if (i_var9 < 0) {
        i_var9 = 0;
    }
    local_bx_8.field_0x24 = i_var9;
    i_var9 = i_var9 / 10;
    local_1c = 0x10;
    if (i_var9 < 0xb) {
        local_1c = 0x14;
    } else {
        if (i_var9 < 0x15) {
            local_1c = 0x13;
        } else {
            if (i_var9 < 0x1f) {
                local_1c = 0x12;
            } else {
                if (i_var9 < 0x29) {
                    local_1c = 0x11;
                } else {
                    if (i_var9 < 0x33) {
                        local_1c = 0x10;
                    } else {
                        if (i_var9 < 0x3d) {
                            local_1c = 0xf;
                        } else {
                            if (i_var9 < 0x47) {
                                local_1c = 0xe;
                            } else {
                                if (i_var9 < 0x51) {
                                    local_1c = 0xd;
                                } else {
                                    if (i_var9 < 0x5b) {
                                        local_1c = 0xc;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    pass1_1030_3258(local_bx_8.field_0x1f6, local_1c);
    return;
}

pub unsafe fn pass1_1038_4760(param_1: *mut Struct1109) {
    let piVar1: *mut i32;
    let ppc_var2: fn();
    let mut u_var3: u16;
    let mut u_var4: i32;
    let mut i_var5: i32;
    let pu_var6: Vec<u8>;
    let mut u_var7: u32;
    let mut in_dx: i32;
    
    let mut u_var8: i32;
    
    
    
    
    let mut i_var9: i32;
    let mut u_var10: u16;
    let mut u_var11: u16;
    let pu_var12: *mut u32;
    let u_var13: u8;
    let mut u_var14: i32;
    let mut local_22: u16;
    let mut local_1a: u32;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;

    u_var10 = (param_1 >> 0x10);
    i_var9 = param_1;
    piVar1 = (i_var9 + 0x22);
    unsafe {
        *piVar1 = *piVar1 + (i_var9 + 0x20c);
    }
    pu_var6 = pass1_1008_c6fa(ctx._PTR_LOOP_1050_06e0, 0x26);
    u_var3 = SUB42(pu_var6, 0);
    pass1_1038_4d6e(param_1, pu_var6 & 0xffff | in_dx << 0x10);
    local_a = CONCAT22(ctx.dx_reg, u_var3);
    u_var11 = SUB42(&ctx.PTR_LOOP_1050_1008, 0);
    u_var8 = ctx.dx_reg;
    pu_var6 = pass1_1008_c6fa(ctx._PTR_LOOP_1050_06e0, 0x1a);
    u_var4 = pu_var6;
    pass1_1038_4d6e(param_1, pu_var6 & 0xffff | u_var8 << 0x10);
    local_e = CONCAT22(ctx.dx_reg, u_var4);
    ppc_var2 = (*local_e + 0x10);
    u_var8 = u_var4;
    ppc_var2(&ctx.PTR_LOOP_1050_1008, u_var4, ctx.dx_reg);
    u_var13 = u_var3;
    u_var14 = ctx.dx_reg;
    if ((ctx.dx_reg | u_var8) == 0) {
        ppc_var2 = (*local_a + 0x10);
        ppc_var2();
        piVar1 = (i_var9 + 0x22);
        unsafe {
            *piVar1 = *piVar1 + u_var8;
        }
    } else {
        ppc_var2 = (*local_a + 0x10);
        ppc_var2();
        _local_16 = CONCAT22(ctx.dx_reg, u_var8);
        local_1a = 0;
        while (local_1a < _local_16) {
            u_var7 = _local_16;
            pu_var12 = local_e;
            pass1_1030_1d7c(local_a, local_1a);
            i_var5 = u_var7;
            u_var11 = SUB42(&PTR_LOOP_1050_1028, 0);
            pass1_1028_5a94(i_var5, ctx.dx_reg, pu_var12);
            if (i_var5 == 2) {
                if ((*ctx._PTR_LOOP_1050_65e2 & 1) == 0) {}
                // goto LAB_1038_485e;
            } else {
                if (i_var5 != 3) {
                    // LAB_1038_485e:
                    piVar1 = (i_var9 + 0x22);
                    unsafe {
                        *piVar1 = *piVar1 + 1;
                    }
                }
            }
            local_1a = local_1a + 1;
        }
    }
    if (local_a != 0x0) {
        ppc_var2 = *local_a;
        ppc_var2(u_var11, u_var3, ctx.dx_reg, 1, u_var13, u_var14);
    }
    if (local_e != 0x0) {
        ppc_var2 = *local_e;
        ppc_var2(u_var11, u_var4, ctx.dx_reg, 1);
    }
    pass1_1038_45e4(param_1);
    unsafe {
        if (0x32 < (i_var9 + 0x24)) {
            piVar1 = (i_var9 + 0x22);
            *piVar1 = *piVar1 + -1;
        }
        if ((i_var9 + 0x24) < 0x32) {
            piVar1 = (i_var9 + 0x22);
            *piVar1 = *piVar1 + 1;
        }
        if ((i_var9 + 0x18) < 0xfa) {
            piVar1 = (i_var9 + 0x22);
            *piVar1 = *piVar1 + 2;
        } else {
            if ((i_var9 + 0x18) < 0x1c2) {
                piVar1 = (i_var9 + 0x22);
                *piVar1 = *piVar1 + 1;
            } else {
                if (0x225 < (i_var9 + 0x18)) {
                    if ((i_var9 + 0x18) < 0x2ee) {
                        piVar1 = (i_var9 + 0x22);
                        *piVar1 = *piVar1 + -1;
                    } else {
                        piVar1 = (i_var9 + 0x22);
                        *piVar1 = *piVar1 + -2;
                    }
                }
            }
        }
    }
    i_var5 = (i_var9 + 0x22);
    if (100 < i_var5) {
        i_var5 = 100;
    }
    (i_var9 + 0x22) = i_var5;
    if (i_var5 < 0) {
        i_var5 = 0;
    }
    (i_var9 + 0x22) = i_var5;
    return;
}

pub unsafe fn pass1_1038_48e0(param_1: Vec<u8>, param_2: i32) {
    let mut iVar1: i32;
    let mut u_var2: u16;

    u_var2 = (param_1 >> 0x10);
    iVar1 = (param_1 + 0x20e) + param_2;
    if (10 < iVar1) {
        iVar1 = 10;
    }
    (param_1 + 0x20e) = iVar1;
    return;
}

pub unsafe fn pass1_1038_4900(param_1: Vec<u8>) {
    let piVar1: *mut i32;
    let mut u_var2: u16;

    u_var2 = (param_1 >> 0x10);
    piVar1 = (param_1 + 0x20e);
    unsafe {
        *piVar1 = *piVar1 + -1;
        if (*piVar1 < 0) {
            (param_1 + 0x20e) = 0;
        }
    }
    return;
}

pub unsafe fn pass1_1038_4918(param_1: *mut Struct1110) {
    let piVar1: *mut i32;
    let mut u_var2: u32;
    let mut u_var3: i32;
    let paVar4: *mut Struct493;
    let mut i_var5: i32;
    let pu_var6: *mut u32;
    let mut in_dx: u16;
    let mut u_var7: u16;
    let local_bx_6: *mut Struct1110;
    let mut i_var8: i32;
    let mut u_var9: u16;
    let mut u_var10: u16;
    let mut unaff_ss: u16;
    let mut u_var11: u32;
    let mut local_15e: u32;
    let mut local_15a: u16;
    let mut local_158: u16;
    let mut local_156: u16;
    let mut local_154: u32;
    let mut local_14a: [u8; 4];
    let mut local_146: u16;
    let mut local_144: u16;
    let mut local_142: u16;
    let mut local_20: u32;
    let mut uStack28: u16;
    let mut local_1a: u32;
    let mut local_12: u32;
    let mut local_e: u32;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    u_var9 = (param_1 >> 0x10);
    local_bx_6 = param_1;
    if (local_bx_6.field_0x4 != 0x4000001) {
        return;
    }
    u_var2 = local_bx_6.field_0x8;
    paVar4 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, u_var2, (u_var2 >> 0x10));
    _local_6 = CONCAT22(in_dx, paVar4);
    local_a = &paVar4.field_0x10;
    u_var10 = (local_a >> 0x10);
    i_var8 = local_a;
    if ((i_var8 + 0x1c) == 0) {
        return;
    }
    local_e = 0;
    match (local_bx_6.field_0x20e) {
        1 => local_e._0_2_ = 0x1e,
        2 => local_e._0_2_ = 0x1c,
        3 => local_e._0_2_ = 0x1a,
        4 => local_e._0_2_ = 0x18,
        5 => local_e._0_2_ = 0x16,
        6 => local_e._0_2_ = 0x14,
        7 => local_e._0_2_ = 0x12,
        8 => local_e._0_2_ = 0x10,
        9 => local_e._0_2_ = 0xe,
        10 => local_e._0_2_ = 0xc,
        // default:
        // goto switchD_1038_49cf_caseD_a;
    }
    local_e = local_e;
    // switchD_1038_49cf_caseD_a:
    local_12 = *ctx._PTR_LOOP_1050_65e2;
    if ((local_e != 0)
        && (((local_12 & 0xffff | *(ctx._PTR_LOOP_1050_65e2 + 2) << 0x10) % local_e) == 0))
    {
        piVar1 = (i_var8 + 0x1c);
        unsafe {
            *piVar1 = *piVar1 + -1;
            piVar1 = (i_var8 + 0x1a);
            *piVar1 = *piVar1 + 1;
        }
        i_var5 = (i_var8 + 0x1a) * 6 + (i_var8 + 0x16);
        u_var10 = (i_var8 + 0x18);
        local_20 = (i_var5 + -6);
        uStack28 = (i_var5 + -2);
        local_146 = &local_20;
        pu_var6 = &local_20;
        pass1_1030_64ce(
            _PTR_LOOP_1050_5740,
            pu_var6,
            unaff_ss,
            local_bx_6.field_0x8,
            local_14a,
            unaff_ss,
        );
        unsafe {
            local_1a = *pu_var6;
        }
        u_var7 = (pu_var6 + 2);
        local_15e._3_1_ = (local_1a >> 0x18);
        if (local_15e._3_1_ != '\0') {
            paVar4 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, local_1a, u_var7);
            u_var11 = pass1_1030_73a8(CONCAT22(u_var7, paVar4));
            u_var3 = (u_var11 >> 0x10);
            if ((u_var3 | u_var11) != 0) {
                i_var8 = (u_var11 + 0xc);
                if (i_var8 < 1) {
                    return;
                }
                if (SBORROW2(i_var8, 1)) {
                    return;
                }
                if (8 < i_var8 + -1) {
                    return;
                }
            }
        }
        pass1_1028_87f0(
            CONCAT22(unaff_ss, &local_144),
            0,
            0,
            0x10,
            &local_20,
            unaff_ss,
            local_bx_6.field_0x4,
            local_bx_6.field_0x8,
        );
        pass1_1030_835a(ctx._g_bool_1050_5748, CONCAT22(unaff_ss, &local_144));
    }
    return;
}

pub unsafe fn pass1_1038_4b20(param_1: Vec<u8>, param_2: Vec<u8>, param_3: u32) {
    pass1_1020_c4f4((param_1 + 0xc), param_2, (param_2 >> 0x10), param_3);
    return;
}

pub unsafe fn pass1_1038_4b40(param_1: *mut Struct1111) {
    let pp_var1: fn();

    let paVar2: *mut Struct493;
    let mut u_var3: u32;

    let mut u_var4: u16;
    
    let mut u_var5: i32;
    let local_bx_12: *mut Struct1111;
    let mut u_var6: u16;
    let mut unaff_cs: u16;
    let mut local_2c: u16;
    let mut local_2a: u16;
    let mut local_24: u16;
    let mut local_22: u16;
    let mut local_1e: u32;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_10: u16;
    let mut local_e: u32;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;

    u_var6 = (param_1 >> 0x10);
    local_bx_12 = param_1;
    if (local_bx_12.field_0xc == 0) {
        in_ax = 0;
        u_var4 = 0;
    } else {
        pp_var1 = (local_bx_12.field_0xc + 0x10);
        (**pp_var1)();
        u_var4 = ctx.dx_reg;
    }
    local_a = CONCAT22(u_var4, in_ax);
    local_e = 0;
    while (local_e < local_a) {
        pp_var1 = (local_bx_12.field_0xc + 4);
        u_var3 = local_a;
        (**pp_var1)(unaff_cs, local_bx_12.field_0xc);
        u_var5 = ctx.dx_reg | u_var3;
        if (u_var5 != 0) {
            paVar2 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, u_var3, ctx.dx_reg);
            unaff_cs = 0x1030;
            pass1_1030_73a8(CONCAT22(u_var5, paVar2));
        }
        local_e = local_e + 1;
    }
    return;
}

pub unsafe fn pass1_1038_4c1a(param_1: *mut Struct1112) {
    let pp_var1: fn();
    let u_var2: u8;

    let paVar3: *mut Struct493;
    let mut u_var4: u32;
    let extraout_var: u32;
    let mut u_var5: i32;
    let mut u_var6: i32;
    let mut in_edx: u32;
    let local_bx_12: *mut Struct1112;
    let local_es_12: *mut Struct1112;
    let mut unaff_cs: u16;
    let mut u_var7: u32;
    let mut local_10: u16;
    let mut local_e: u32;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;

    local_es_12 = (param_1 >> 0x10);
    local_bx_12 = param_1;
    u_var7 = local_bx_12.field_0xc;
    pp_var1 = (local_bx_12.field_0xc + 0x10);
    (**pp_var1)();
    local_a = CONCAT22(in_edx, in_ax);
    local_e = 0;
    while (u_var5 = in_edx, local_e < local_a) {
        pp_var1 = (local_bx_12.field_0xc + 4);
        u_var4 = local_a;
        (**pp_var1)(unaff_cs, local_bx_12.field_0xc, local_e, u_var7);
        u_var6 = u_var5 | u_var4;
        in_edx = u_var6;
        if (u_var6 != 0) {
            paVar3 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, u_var4, u_var5);
            u_var2 = pass1_1030_6fa0(CONCAT22(in_edx, paVar3));
            unaff_cs = SUB42(&ctx.PTR_LOOP_1050_1008, 0);
            pass1_1008_c6ae(ctx._PTR_LOOP_1050_06e0, CONCAT31(extraout_var, u_var2), 0xe);
        }
        local_e = local_e + 1;
    }
    return;
}

pub unsafe fn pass1_1038_4cba(param_1: Vec<u8>) {
    pass1_1030_38b8((param_1 + 0x1f6));
    return;
}

pub unsafe fn pass1_1038_4cd0(param_1: Vec<u8>, param_2: u32, param_3: u16) {
    let local_es_6: Vec<u8>;

    local_es_6 = (param_1 >> 0x10);
    (param_1 + 0x1c) = param_3;
    (param_1 + 0x1e) = param_2;
    return;
}

pub unsafe fn pass1_1038_4cea(param_1: Vec<u8>, param_2: u32, param_3: u32) {
    let mut local_es_3: u16;

    local_es_3 = (param_1 >> 0x10);
    param_3 = (param_1 + 0x1c);
    param_2 = (param_1 + 0x1e);
    return;
}

pub unsafe fn pass1_1038_4d0e(param_1: *mut Struct1113, param_2: u16) {
    let local_bx_3: *mut Struct1113;
    let mut u_var1: u16;

    u_var1 = (param_1 >> 0x10);
    local_bx_3 = param_1;
    local_bx_3.field_0x1a = local_bx_3.field_0x18;
    local_bx_3.field_0x18 = param_2;
    return;
}

pub unsafe fn pass1_1038_4d28(param_1: u32) {
    let mut u_var1: u16;

    u_var1 = (param_1 >> 0x10);
    return CONCAT22((param_1 + 0x1fc), (param_1 + 0x1fa));
}

pub unsafe fn pass1_1038_4d3c(param_1: *mut Struct1114, param_2: u32) {
    let u_var1: u8;
    let mut local_register1_15: u16;
    let mut local_DX__1: u16;
    let local_bx_4: *mut Struct1114;
    let mut u_var3: u16;
    let paVar2: &mut Struct44;

    u_var3 = (param_1 >> 0x10);
    local_bx_4 = param_1;
    u_var1 = error_check_1000_17ce(&local_bx_4.field_0x1fa);
    paVar2 = CONCAT31(_local_register1_15, u_var1);
    pass1_fn_1008_60e8(param_2);
    local_bx_4.field_0x1fa = paVar2;
    local_bx_4.field_0x1fc = local_DX__1;
    return;
}

pub unsafe fn pass1_1038_4d6e(param_1: *mut Struct1115, param_2: u32) {
    let pu_var1: *mut u16;
    let ppc_var2: fn();
    let u_var3: u8;
    let mut in_ax: i32;
    let paVar4: *mut Struct493;
    let extraout_var: u32;
    let in_dx: *mut Struct199;

    
    let mut u_var5: u16;
    
    let mut u_var6: i32;
    let local_bx_49: *mut Struct1115;
    let mut u_var7: u16;
    let mut u_var8: u32;
    let mut local_22: u16;
    let mut local_20: u16;
    let mut local_1e: u16;
    let mut local_1c: u16;
    let mut local_1a: u32;
    let mut local_16: u32;
    let mut local_10: u16;
    let mut local_e: u32;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    process_struct_1000_179c(0x18, in_dx);
    if ((in_dx | in_ax) == 0) {
        in_ax = 0;
        u_var7 = 0;
    } else {
        pass1_1030_1cd8(CONCAT22(in_dx, in_ax), 5, 5);
        u_var7 = ctx.dx_reg;
    }
    _local_6 = CONCAT22(u_var7, in_ax);
    u_var7 = (param_1 >> 0x10);
    local_bx_49 = param_1;
    if (local_bx_49.field_0xc == 0) {
        in_ax = 0;
        u_var5 = 0;
    } else {
        ppc_var2 = (local_bx_49.field_0xc + 0x10);
        ppc_var2();
        u_var5 = ctx.dx_reg;
    }
    local_a = CONCAT22(u_var5, in_ax);
    local_e = 0;
    loop {
        if (local_a <= local_e) {
            return;
        }
        ppc_var2 = (local_bx_49.field_0xc + 4);
        u_var8 = local_a;
        ppc_var2();
        u_var6 = ctx.dx_reg | u_var8;
        if (u_var6 != 0) {
            paVar4 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, u_var8, ctx.dx_reg);
            local_1a = CONCAT22(u_var6, paVar4);
            u_var3 = pass1_1030_6fa0(CONCAT22(u_var6, paVar4));
            local_1e = 0;
            loop {
                pu_var1 = (param_2 + 4);
                let pu_var1_val = unsafe { *pu_var1 };
                if (pu_var1_val == local_1e || pu_var1_val < local_1e) {
                    break;
                }
                if ((param_2 + local_1e * 2) == CONCAT31(extraout_var, u_var3)) {
                    u_var8 = pass1_1030_73a8(local_1a);
                    if ((u_var8 + 0x12) == 5) {
                        ppc_var2 = (*_local_6 + 0xc);
                        ppc_var2();
                    }
                    break;
                }
                local_1e = local_1e + 1;
            }
        }
        local_e = local_e + 1;
    }
}

pub unsafe fn pass1_1038_4e78(param_1: u32, param_2: u32) {
    let pu_var1: *mut u16;
    let ppc_var2: fn();
    let u_var3: u8;
    let mut in_ax: i32;
    let local_AX_174: *mut Struct515;
    let extraout_var: u32;
    let in_dx: *mut Struct199;

    
    let mut u_var5: u16;
    
    let mut i32_var6: i32;
    let mut u_var7: u16;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_16: u32;
    let mut local_10: u16;
    let mut local_e: u32;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;
    let mut u_var4: u32;

    process_struct_1000_179c(0x18, in_dx);
    if ((in_dx | in_ax) == 0) {
        in_ax = 0;
        u_var7 = 0;
    } else {
        pass1_1030_1cd8(CONCAT22(in_dx, in_ax), 5, 5);
        u_var7 = ctx.dx_reg;
    }
    _local_6 = CONCAT22(u_var7, in_ax);
    u_var7 = (param_1 >> 0x10);
    i32_var6 = param_1;
    if ((i32_var6 + 0xc) == 0) {
        in_ax = 0;
        u_var5 = 0;
    } else {
        ppc_var2 = ((i32_var6 + 0xc) + 0x10);
        ppc_var2();
        u_var5 = ctx.dx_reg;
    }
    local_a = CONCAT22(u_var5, in_ax);
    local_e = 0;
    loop {
        if (local_a <= local_e) {
            return;
        }
        u_var4 = local_a;
        pass1_1030_1d58((i32_var6 + 0xc));
        if ((ctx.dx_reg | u_var4) != 0) {
            u_var3 = pass1_1030_6fa0((u_var4 & 0xffff | ctx.dx_reg << 0x10));
            local_1a = 0;
            loop {
                pu_var1 = (param_2 + 4);
                let pu_var1_val = unsafe { *pu_var1 };
                if (pu_var1_val == local_1a || pu_var1_val < local_1a) {
                    break;
                }
                if ((param_2 + local_1a * 2) == CONCAT31(extraout_var, u_var3)) {
                    ppc_var2 = (*_local_6 + 0xc);
                    ppc_var2();
                    break;
                }
                local_1a = local_1a + 1;
            }
        }
        local_e = local_e + 1;
    }
}

pub unsafe fn pass1_1038_4f54(param_1: *mut Struct1116, param_2: u16) {
    let pp_var1: fn();
    let u_var2: u8;

    let BVar3: bool;
    let mut u_var4: u32;
    let extraout_var: u32;

    let mut u_var5: u16;
    
    let local_bx_4: *mut Struct1116;
    let mut u_var6: u16;
    let mut local_c: u16;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    u_var6 = (param_1 >> 0x10);
    local_bx_4 = param_1;
    if (local_bx_4.field_0xc == 0x0) {
        in_ax = 0;
        u_var5 = 0;
    } else {
        pp_var1 = (*local_bx_4.field_0xc + 0x10);
        (**pp_var1)();
        u_var5 = ctx.dx_reg;
    }
    _local_6 = CONCAT22(u_var5, in_ax);
    local_a = 0;
    loop {
        if (_local_6 <= local_a) {
            return;
        }
        u_var4 = _local_6;
        pass1_1030_1d58(local_bx_4.field_0xc);
        if ((ctx.dx_reg | u_var4) != 0) {
            u_var2 = pass1_1030_6fa0((u_var4 & 0xffff | ctx.dx_reg << 0x10));
            BVar3 = pass1_1008_c6ae(
                ctx._PTR_LOOP_1050_06e0,
                CONCAT31(extraout_var, u_var2),
                param_2,
            );
            if (BVar3 != 0) {
                return;
            }
        }
        local_a = local_a + 1;
    }
}

pub unsafe fn pass1_1038_4fd8(param_1: *mut Struct1117, param_2: u16) {
    let pp_var1: fn();
    let u_var2: u8;

    let mut u_var3: u32;
    let extraout_var: u32;

    let mut u_var4: u16;
    
    let local_bx_4: *mut Struct1117;
    let mut u_var5: u16;
    let mut local_c: u16;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    u_var5 = (param_1 >> 0x10);
    local_bx_4 = param_1;
    if (local_bx_4.field_0xc == 0x0) {
        in_ax = 0;
        u_var4 = 0;
    } else {
        pp_var1 = (*local_bx_4.field_0xc + 0x10);
        (**pp_var1)();
        u_var4 = ctx.dx_reg;
    }
    _local_6 = CONCAT22(u_var4, in_ax);
    local_a = 0;
    loop {
        if (_local_6 <= local_a) {
            return;
        }
        u_var3 = _local_6;
        pass1_1030_1d58(local_bx_4.field_0xc);
        if ((ctx.dx_reg | u_var3) != 0) {
            u_var2 = pass1_1030_6fa0((u_var3 & 0xffff | ctx.dx_reg << 0x10));
            if (CONCAT31(extraout_var, u_var2) == param_2) {
                return;
            }
        }
        local_a = local_a + 1;
    }
}

pub unsafe fn pass1_1038_5050(param_1: *mut Struct1118, param_2: u16) {
    let pp_var1: fn();
    let u_var2: u8;

    let mut u_var3: u32;
    let extraout_var: u32;

    let mut u_var4: u16;
    
    let local_bx_12: *mut Struct1118;
    let mut u_var5: u16;
    let mut local_10: u16;
    let mut local_e: u32;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;

    u_var5 = (param_1 >> 0x10);
    local_bx_12 = param_1;
    if (local_bx_12.field_0xc == 0x0) {
        in_ax = 0;
        u_var4 = 0;
    } else {
        pp_var1 = (*local_bx_12.field_0xc + 0x10);
        (**pp_var1)();
        u_var4 = ctx.dx_reg;
    }
    local_a = CONCAT22(u_var4, in_ax);
    local_e = 0;
    while (local_e < local_a) {
        u_var3 = local_a;
        pass1_1030_1d58(local_bx_12.field_0xc);
        if ((ctx.dx_reg | u_var3) != 0) {
            u_var2 = pass1_1030_6fa0((u_var3 & 0xffff | ctx.dx_reg << 0x10));
            pass1_1008_c6ae(
                ctx._PTR_LOOP_1050_06e0,
                CONCAT31(extraout_var, u_var2),
                param_2,
            );
        }
        local_e = local_e + 1;
    }
    return;
}

pub unsafe fn pass1_1038_50e0(param_1: *mut Struct1119, param_2: u16) {
    let pp_var1: fn();
    let u_var2: u8;

    let BVar3: bool;
    let mut u_var4: u32;
    let extraout_var: u32;

    let mut u_var5: u16;
    
    let local_bx_12: *mut Struct1119;
    let mut u_var6: u16;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u32;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;

    u_var6 = (param_1 >> 0x10);
    local_bx_12 = param_1;
    if (local_bx_12.field_0xc == 0x0) {
        in_ax = 0;
        u_var5 = 0;
    } else {
        pp_var1 = (*local_bx_12.field_0xc + 0x10);
        (**pp_var1)();
        u_var5 = ctx.dx_reg;
    }
    local_a = CONCAT22(u_var5, in_ax);
    local_e = 0;
    while (local_e < local_a) {
        u_var4 = local_a;
        pass1_1030_1d58(local_bx_12.field_0xc);
        if ((ctx.dx_reg | u_var4) != 0) {
            u_var2 = pass1_1030_6fa0((u_var4 & 0xffff | ctx.dx_reg << 0x10));
            BVar3 = pass1_1008_c6ae(
                ctx._PTR_LOOP_1050_06e0,
                CONCAT31(extraout_var, u_var2),
                param_2,
            );
            if (BVar3 != 0) {
                pass1_1030_73a8((u_var4 & 0xffff | ctx.dx_reg << 0x10));
            }
        }
        local_e = local_e + 1;
    }
    return;
}

pub unsafe fn pass1_1038_518c(param_1: *mut Struct1120) {
    let pu_var1: *mut u32;
    let mut u_var2: u32;
    let ppc_var3: fn();

    let paVar4: *mut Struct493;
    let mut u_var5: u32;

    
    let mut u_var6: i32;
    
    let local_bx_5: *mut Struct1120;
    let mut i_var7: i32;
    let mut i_var8: i32;
    let mut u_var9: u16;
    let mut u_var10: u16;
    let unaff_cs: u8;
    let mut bVar11: bool;
    let mut u_var12: u32;
    let mut local_22: u16;
    let mut local_20: u32;
    let mut local_1c: u32;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_c: u16;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    u_var9 = (param_1 >> 0x10);
    local_bx_5 = param_1;
    if (local_bx_5.field_0x206 == 0) {
        if (local_bx_5.field_0xc == 0) {
            in_ax = 0;
            u_var10 = 0;
        } else {
            ppc_var3 = (local_bx_5.field_0xc + 0x10);
            (**ppc_var3)();
            u_var10 = ctx.dx_reg;
        }
        _local_6 = CONCAT22(u_var10, in_ax);
        local_a = 0;
        while (local_a < _local_6) {
            u_var2 = local_bx_5.field_0xc;
            ppc_var3 = (local_bx_5.field_0xc + 4);
            u_var5 = _local_6;
            (**ppc_var3)(
                unaff_cs,
                u_var2,
                (u_var2 >> 0x10),
                local_a,
                (local_a >> 0x10),
            );
            u_var6 = ctx.dx_reg | u_var5;
            if (u_var6 != 0) {
                paVar4 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, u_var5, ctx.dx_reg);
                unaff_cs = 0x30;
                u_var12 = pass1_1030_73a8(CONCAT22(u_var6, paVar4));
                i_var7 = (u_var12 + 0x12);
                u_var6 = u_var12 + 0x14;
                u_var5 = u_var6;
                local_1c = u_var12 & 0xffff0000 | u_var6;
                local_20 = 0;
                if ((i_var7 == 4) || (i_var7 == 5)) {
                    u_var5 = local_1c;
                    local_20 = u_var5;
                }
                if (local_20 != 0) {
                    local_22 = 0x11;
                    while (local_22 < 0x25) {
                        if (((local_bx_5.field_0x204 == 0) || (local_22 == 0x23))
                            || (local_22 == 0x24))
                        {
                            pass1_1038_540a(param_1, local_22);
                            i_var7 = local_22 * 4;
                            u_var10 = (local_20 >> 0x10);
                            i_var8 = local_20;
                            pu_var1 = (i_var7 + i_var8 + 2);
                            unsafe {
                                bVar11 = *pu_var1 < ctx.dx_reg;
                                if ((bVar11 || *pu_var1 == ctx.dx_reg)
                                    && (bVar11
                                        || (
                                            pu_var1 = (i_var7 + i_var8),
                                            *pu_var1 < u_var5 || *pu_var1 == u_var5,
                                        )))
                                {
                                    pass1_1038_5770(param_1, (i_var7 + i_var8), local_22);
                                }
                            }
                        }
                        local_22 = local_22 + 1;
                    }
                }
            }
            local_a = local_a + 1;
        }
    }
    return;
}

pub unsafe fn pass1_1038_52b8(param_1: *mut Struct1121, param_2: u32, param_3: *mut Struct1125) {
    let mut u_var1: u32;
    let ppc_var2: fn();
    let mut i_var3: i32;
    let paVar4: *mut Struct493;
    let mut u_var5: u32;

    let mut u_var6: u16;
    
    let mut u_var7: i32;
    let mut i_var8: i32;
    let mut u_var9: u16;
    let mut unaff_cs: u16;
    let local_24: *mut Struct1121;
    let uStack34: u8;
    let mut uStack30: u16;
    let mut uStack28: u16;
    let mut local_1a: u32;
    let mut local_16: u32;
    let mut local_10: u16;
    let mut local_e: u32;
    let mut local_a: u16;
    let mut local_8: u16;

    i_var3 = -param_2;
    pass1_1038_5694(
        param_1,
        CONCAT22(-(param_2._2_2_ + (param_2 != 0)), i_var3),
        param_3,
    );
    if (param_3 != (s_New_failed_in_Op__Op_1050_0020 + 4)) {
        u_var9 = (param_1 >> 0x10);
        i_var8 = param_1;
        if ((i_var8 + 0xc) == 0) {
            i_var3 = 0;
            u_var6 = 0;
        } else {
            ppc_var2 = ((i_var8 + 0xc) + 0x10);
            ppc_var2();
            u_var6 = ctx.dx_reg;
        }
        local_a = CONCAT22(u_var6, i_var3);
        local_e = 0;
        while (local_e < local_a) {
            uStack30 = local_e;
            uStack28 = (local_e >> 0x10);
            u_var1 = (i_var8 + 0xc);
            uStack34 = u_var1;
            ppc_var2 = ((i_var8 + 0xc) + 4);
            u_var5 = local_a;
            ppc_var2(unaff_cs, uStack34, (u_var1 >> 0x10), uStack30, uStack28);
            u_var7 = ctx.dx_reg | u_var5;
            if (u_var7 != 0) {
                paVar4 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, u_var5, ctx.dx_reg);
                local_16 = CONCAT22(u_var7, paVar4);
                unaff_cs = 0x1030;
                pass1_1030_7c28(CONCAT22(u_var7, paVar4), param_3);
                local_1a = CONCAT22(u_var7, paVar4);
                if ((u_var7 | paVar4) != 0) {
                    if (local_1a < param_2) {
                        param_2 = param_2 - local_1a;
                        local_1a = 0;
                    } else {
                        local_1a = CONCAT22(
                            (u_var7 - param_2._2_2_) - (paVar4 < param_2),
                            (paVar4 - param_2),
                        );
                        param_2 = 0;
                    }
                    uStack30 = (local_1a >> 0x10);
                    unaff_cs = 0x1030;
                    pass1_1030_7d1c(local_16, local_1a, CONCAT22(param_3, uStack30));
                    if (param_2 == 0) {
                        return;
                    }
                }
            }
            local_e = local_e + 1;
        }
    }
    return;
}

pub unsafe fn pass1_1038_3f38(param_1: *mut u32, param_2: *mut u32, param_3: u32) {
    let pp_var1: fn();
    let u_var2: u8;
    let paVar3: *mut Struct493;
    let extraout_var: u32;
    let mut in_dx: i32;
    let mut u_var4: i32;
    let pu_var5: *mut u32;
    let mut u_var6: u16;
    let mut u_var7: u32;
    let mut u_var8: u16;
    let mut local_e: u32;
    let mut local_a: u16;
    let mut local_8: u16;
    let local_6: *mut Struct1100;
    let mut local_4: u16;

    paVar3 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, param_3, (param_3 >> 0x10));
    _local_6 = CONCAT22(in_dx, paVar3);
    u_var4 = in_dx;
    u_var2 = pass1_1028_b58e(CONCAT22(in_dx, paVar3));
    u_var7 = (CONCAT31(extraout_var, u_var2) + 4);
    unsafe {
        pp_var1 = (*param_1 + 0x18);
    }
    (**pp_var1)(&PTR_LOOP_1050_1028, param_1, u_var7);
    u_var8 = 0;
    u_var6 = 0;
    unsafe {
        pp_var1 = (*param_2 + 8);
    }
    pu_var5 = param_2;
    (**pp_var1)();
    pass1_1030_73ee(
        (CONCAT31(extraout_var, u_var2) & 0xffff | u_var4 << 0x10),
        (param_2 + 4),
    );
    pp_var1 = (*_local_6 + 0x58);
    (**pp_var1)(
        0x1030, paVar3, in_dx, param_2, pu_var5, u_var6, u_var7, u_var8,
    );
    return;
}

pub unsafe fn pass1_1038_3fb0(param_1: Vec<u8>) {
    let mut u_var1: u32;

    u_var1 = (param_1 + 0x200);
    pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, u_var1, (u_var1 >> 0x10));
    return;
}

pub unsafe fn pass1_1038_3fca(param_1: u32) {
    let mut u_var1: u32;
    let ppc_var2: fn();
    let mut in_ax: i32;
    let local_AX_84: *mut Struct1103;
    let local_AX_140: *mut Struct1102;
    let paVar3: *mut Struct493;
    
    
    let mut u_var4: i32;
    let local_bx_5: *mut Struct1101;
    let mut i_var5: i32;
    let mut unaff_si: u16;
    let mut i32_var6: i32;
    let pu_var7: Vec<u8>;
    let mut u_var8: u16;
    let mut u_var9: u16;
    let mut unaff_ss: u16;
    let mut u_var10: u32;
    let pp_var11: *mut Struct2111;
    let u_var12: u8;
    let u_var13: u8;
    let u_var14: u8;
    let u_var15: u8;
    let mut local_3a: u32;
    let mut local_36: u16;
    let mut local_34: u16;
    let mut local_32: u16;
    let mut local_30: u16;
    let mut local_26: u16;
    let mut local_24: u16;
    let mut local_22: [u8; 2];
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
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    pu_var7 = (param_1 >> 0x10);
    local_bx_5 = param_1;
    if (local_bx_5.field_0xc == 0) {
        in_ax = 0;
        u_var4 = 0;
    } else {
        ppc_var2 = (local_bx_5.field_0xc + 0x10);
        ppc_var2();
        u_var4 = ctx.dx_reg;
    }
    _local_6 = CONCAT22(u_var4, in_ax);
    ctx.g_u16_ptr_1050_5f2e = (u_var4 | in_ax);
    if (ctx.g_u16_ptr_1050_5f2e != 0x0) {
        if (ctx.__g_Struct94_ptr_1 == 0) {
            struct_fn_1000_160a();
            _g_Struct94_ptr_1._0_1_ = in_ax;
        } else {
        }
        local_AX_84 = (local_6 << 2);
        alloc_mem_1000_1708(
            local_AX_84,
            0,
            1,
            _g_Struct94_ptr_1._0_1_,
            ctx.g_u16_ptr_1050_5f2e,
        );
        local_a = CONCAT22(ctx.g_u16_ptr_1050_5f2e, local_AX_84);
        if (ctx.__g_Struct94_ptr_1 == 0) {
            struct_fn_1000_160a();
            _g_Struct94_ptr_1._0_1_ = SUB21(local_AX_84, 0);
        } else {
        }
        local_AX_140 = (local_6 << 2);
        u_var9 = 0x1000;
        alloc_mem_1000_1708(
            local_AX_140,
            0,
            1,
            _g_Struct94_ptr_1._0_1_,
            ctx.g_u16_ptr_1050_5f2e,
        );
        local_e = CONCAT22(ctx.g_u16_ptr_1050_5f2e, local_AX_140);
        local_16 = 0;
        while (local_16 < _local_6) {
            u_var1 = local_bx_5.field_0xc;
            ppc_var2 = (local_bx_5.field_0xc + 4);
            u_var10 = _local_6;
            ppc_var2(
                u_var9,
                u_var1,
                (u_var1 >> 0x10),
                local_16,
                (local_16 >> 0x10),
            );
            local_12 = u_var10;
            u_var4 = ctx.dx_reg | local_12;
            local_10 = ctx.dx_reg;
            if (u_var4 != 0) {
                paVar3 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, local_12, ctx.dx_reg);
                i_var5 = local_16 * 4;
                u_var8 = (local_a >> 0x10);
                i32_var6 = local_a;
                (i_var5 + i32_var6) = paVar3;
                (i_var5 + i32_var6 + 2) = u_var4;
                u_var9 = 0x1030;
                u_var10 = pass1_1030_73a8(CONCAT22(u_var4, (i_var5 + i32_var6)));
                u_var8 = (local_e >> 0x10);
                (local_e + i_var5) = u_var10;
                (local_e + i_var5 + 2) = (u_var10 >> 0x10);
            }
            local_16 = local_16 + 1;
        }
        local_16 = 0;
        while (local_16 < _local_6) {
            u_var9 = (local_e >> 0x10);
            i_var5 = local_e;
            if (((local_16 * 4 + i_var5) != 0)
                && (
                    u_var1 = (local_16 * 4 + i_var5),
                    (u_var1 + 0x1a) = 0,
                    u_var1 = (local_16 * 4 + i_var5),
                    (u_var1 + 0x12) == 5,
                ))
            {
                pass1_1028_bdac((local_16 * 4 + i_var5), 6);
            }
            local_16 = local_16 + 1;
        }
        local_bx_5.field_0x204 = 0;
        pp_var11 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(unaff_si, 2));
        local_1e = (pp_var11 >> 0x10);
        local_1a = pp_var11;
        local_1c = u16_1050_13ae;
        if (u16_1050_13ae == 1) {
            local_bx_5.field_0x204 = 1;
        }
        local_18 = local_1e;
        pass1_1038_5a96(local_bx_5, pu_var7, _local_6, local_e);
        pass1_1038_5cc6(param_1, _local_6, local_e, local_a, 0, 2);
        pass1_1038_5b3c(local_bx_5, pu_var7, _local_6, local_e);
        pass1_1038_5cc6(param_1, _local_6, local_e, local_a, 0, 1);
        u_var14 = SUB21(local_22, 0);
        u_var15 = (local_22 >> 8);
        u_var12 = SUB21(&local_24, 0);
        u_var13 = (&local_24 >> 8);
        u_var1 = local_bx_5.field_0x8;
        u_var9 = unaff_ss;
        local_20 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, u_var1, (u_var1 >> 0x10));
        pass1_1030_5b1c(
            CONCAT22(local_1e, local_20),
            CONCAT22(unaff_ss, CONCAT11(u_var13, u_var12)),
            CONCAT22(u_var9, CONCAT11(u_var15, u_var14)),
        );
        local_26 = 1;
        while (local_26 <= local_24) {
            pass1_1038_58e6(local_bx_5, pu_var7, _local_6, local_e, local_a, local_26);
            pass1_1038_5cc6(param_1, _local_6, local_e, local_a, local_26, 3);
            local_26 = local_26 + 1;
        }
        pass1_1038_5a16(local_bx_5, pu_var7, _local_6, local_e);
        local_16 = 0;
        while (local_16 < _local_6) {
            u_var9 = (local_e >> 0x10);
            i_var5 = local_e;
            if (((local_16 * 4 + i_var5) != 0)
                && (u_var1 = (local_16 * 4 + i_var5), (u_var1 + 0x12) != 5))
            {
                u_var1 = (local_16 * 4 + i_var5);
                ppc_var2 = ((local_16 * 4 + i_var5) + 0x28);
                ppc_var2(0x1030, u_var1, (u_var1 >> 0x10));
            }
            local_16 = local_16 + 1;
        }
        error_check_1000_17ce(local_a);
        error_check_1000_17ce(local_e);
    }
    return;
}

pub unsafe fn pass1_1038_42cc(param_1: *mut Struct1104) {
    let paVar1: *mut Struct872;
    let ppc_var2: fn();
    let mut u8_var3: bool;
    let mut u_var4: u16;
    let mut u_var5: u16;
    let mut u_var6: i32;
    let pu_var7: Vec<u8>;
    let mut u_var8: u32;
    let mut in_dx: i32;

    
    
    
    let local_bx_4: *mut Struct1104;
    let local_es_4: *mut Struct1104;
    let mut u_var9: u16;
    let mut local_1c: u16;
    let mut local_1a: u16;
    let mut local_18: u32;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u32;
    let mut local_a: u16;
    let mut local_8: u16;

    local_es_4 = (param_1 >> 0x10);
    local_bx_4 = param_1;
    if (local_bx_4.field_0x1f6 == 0x0) {
        return;
    }
    u_var9 = SUB42(&ctx.PTR_LOOP_1050_1008, 0);
    pu_var7 = pass1_1008_c6fa(ctx._PTR_LOOP_1050_06e0, 0x2d);
    u_var4 = SUB42(pu_var7, 0);
    pass1_1038_4d6e(param_1, pu_var7 & 0xffff | in_dx << 0x10);
    local_a = CONCAT22(ctx.dx_reg, u_var4);
    ppc_var2 = (*local_a + 0x10);
    u_var5 = u_var4;
    ppc_var2(&ctx.PTR_LOOP_1050_1008, u_var4, ctx.dx_reg);
    _local_12 = CONCAT22(ctx.dx_reg, u_var5);
    u8_var3 = false;
    local_18 = 0;
    while (local_18 < _local_12) {
        u_var9 = 0x1030;
        u_var8 = _local_12;
        pass1_1030_1d7c(local_a, local_18, (local_18 >> 0x10));
        u_var6 = u_var8;
        if ((ctx.dx_reg | u_var6) != 0) {
            ppc_var2 = ((u_var8 & 0xffff | ctx.dx_reg << 0x10) + 0x50);
            ppc_var2();
            if (u_var6 != 0) {
                u8_var3 = true;
            }
        }
        local_18 = local_18 + 1;
    }
    if (u8_var3) {
        paVar1 = local_bx_4.field_0x1f6;
        (paVar1 + 0x1aa) = 0;
    } else {
        paVar1 = local_bx_4.field_0x1f6;
        u_var9 = 0x1030;
        pass1_1030_38b8(paVar1, (paVar1 >> 0x10));
        if ((ctx.dx_reg | _local_12) != 0) {
            u_var9 = 0x1030;
            pass1_1030_326a(local_bx_4.field_0x1f6);
        }
    }
    if (local_a != 0x0) {
        ppc_var2 = *local_a;
        ppc_var2(u_var9, u_var4, ctx.dx_reg, 1);
    }
    return;
}

pub unsafe fn pass1_1038_43cc(param_1: *mut Struct1106, param_2: u32, param_3: u16) {
    let pp_var1: fn();
    let mut in_ax: i32;
    let mut u_var2: u16;
    let mut u_var3: u16;
    let mut u_var4: i32;
    let pu_var5: Vec<u8>;
    let mut u_var6: u32;
    let extraout_DL: u8;
    let mut in_dx: i32;
    let mut u_var7: i32;

    
    let local_SI_56: *mut Struct1105;
    let mut i_var8: i32;
    let mut u_var9: u16;
    let mut local_1a: u32;
    let mut local_16: u32;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;

    if (param_3 == 5) {
        pass1_1038_4900(CONCAT22(param_2, param_1));
        return;
    }
    pass1_1038_53ba(CONCAT22(param_2, param_1), param_3);
    if ((in_dx != 0) || (in_ax != 0)) {
        local_SI_56 = (param_3 * 4);
        u_var4 = (param_1 + local_SI_56 + 0x14e);
        u_var7 = param_2._2_2_ >> 0xf;
        i_var8 = ((param_1 + local_SI_56 + 0x150) - u_var7) - (u_var4 < param_2._2_2_);
        (param_1 + local_SI_56 + 0x14e) = u_var4 - param_2._2_2_;
        (param_1 + local_SI_56 + 0x150) = i_var8;
        if (i_var8 < 0) {
            (param_1 + local_SI_56 + 0x14e) = 0;
        }
        u_var9 = SUB42(&ctx.PTR_LOOP_1050_1008, 0);
        pu_var5 = pass1_1008_c6fa(ctx._PTR_LOOP_1050_06e0, 0x1e);
        u_var2 = SUB42(pu_var5, 0);
        pass1_1038_4e78(
            CONCAT22(param_2, param_1),
            pu_var5 & 0xffff | u_var7 << 0x10,
        );
        local_e = CONCAT22(ctx.dx_reg, u_var2);
        pp_var1 = (*local_e + 0x10);
        u_var3 = u_var2;
        (**pp_var1)(&ctx.PTR_LOOP_1050_1008, u_var2, ctx.dx_reg);
        _local_12 = CONCAT22(ctx.dx_reg, u_var3);
        local_16 = 0;
        while (local_16 < _local_12) {
            u_var6 = _local_12;
            pass1_1030_1d7c(u_var2, ctx.dx_reg, local_16, (local_16 >> 0x10));
            param_2._2_2_ = (param_2 >> 0x10);
            u_var3 = u_var6;
            while (u_var4 = u_var6, param_2._2_2_ != 0) {
                pass1_1030_cf78(u_var3, extraout_DL, param_3);
                u_var6 = u_var4;
                if (u_var4 == 0) {
                    break;
                }
                param_2._2_2_ = param_2._2_2_ - 1;
            }
            param_2 = param_2._2_2_ << 0x10;
            u_var9 = 0x1030;
            if (param_2._2_2_ == 0) {
                break;
            }
            local_16 = local_16 + 1;
        }
        if (local_e != 0x0) {
            pp_var1 = *local_e;
            (**pp_var1)(u_var9, u_var2, ctx.dx_reg, 1);
            return;
        }
    }
    return;
}

pub unsafe fn pass1_1038_44d8(
    param_1: *mut Struct998,
    param_2: Vec<u8>,
    uparam_3: i32,
    param_3_00: Vec<u8>,
) {
    let pp_var1: fn();
    let paVar2: &mut Struct44;
    let mut in_ax: i32;
    let local_AX_121: Vec<u8>;
    let pu_var3: Vec<u8>;
    let mut u_var4: i32;
    let pu_var5: Vec<u8>;
    let mut u_var6: u32;
    let mut in_dx: i32;
    let mut u_var7: i32;
    let local_DX_128: Vec<u8>;
    let local_DX_145: Vec<u8>;
    let ctx.dx_reg: &mut Struct44;
    let local_SI_56: *mut Struct1107;
    let mut i_var8: i32;
    let local_CS_115: Vec<u8>;
    let mut local_1a: u32;
    let mut local_16: u32;
    let mut local_12: u16;
    let mut local_10: u16;
    let local_e: *mut Struct1108;
    let mut local_c: u16;

    if (param_3_00 == (&PTR_DAT_0005_0000_1050_0004 + 1)) {
        pass1_1038_4900(CONCAT22(param_2, param_1));
        return;
    }
    pass1_1038_53ba(CONCAT22(param_2, param_1), param_3_00);
    if ((in_dx != 0) || (in_ax != 0)) {
        local_SI_56 = (param_3_00 * 4);
        u_var4 = (param_1 + local_SI_56 + 0x14e);
        u_var7 = param_3 >> 0xf;
        i_var8 = ((param_1 + local_SI_56 + 0x150) - u_var7) - (u_var4 < param_3);
        (param_1 + local_SI_56 + 0x14e) = u_var4 - param_3;
        (param_1 + local_SI_56 + 0x150) = i_var8;
        if (i_var8 < 0) {
            (param_1 + local_SI_56 + 0x14e) = 0;
        }
        local_CS_115 = &ctx.PTR_LOOP_1050_1008;
        pu_var5 = pass1_1008_c6fa(ctx._PTR_LOOP_1050_06e0, 0x1e);
        local_AX_121 = pu_var5;
        pass1_1038_4e78(
            CONCAT22(param_2, param_1),
            pu_var5 & 0xffff | u_var7 << 0x10,
        );
        local_e = CONCAT22(local_DX_128, local_AX_121);
        pp_var1 = (*local_e + 0x10);
        pu_var3 = local_AX_121;
        (**pp_var1)(&ctx.PTR_LOOP_1050_1008, local_AX_121, local_DX_128);
        _local_12 = CONCAT22(local_DX_145, pu_var3);
        local_16 = 0;
        while (local_16 < _local_12) {
            u_var6 = _local_12;
            pass1_1030_1d7c(local_AX_121, local_DX_128, local_16, (local_16 >> 0x10));
            paVar2 = u_var6;
            while (u_var4 = u_var6, param_3 != 0) {
                pass1_1030_d00c(paVar2, ctx.dx_reg, param_3_00);
                u_var6 = u_var4;
                if (u_var4 == 0) {
                    break;
                }
                param_3 = param_3 - 1;
            }
            local_CS_115 = 0x1030;
            if (param_3 == 0) {
                break;
            }
            local_16 = local_16 + 1;
        }
        if (local_e != 0x0) {
            pp_var1 = *local_e;
            (**pp_var1)(local_CS_115, local_AX_121, local_DX_128, 1);
            return;
        }
    }
    return;
}

pub unsafe fn pass1_1038_3698(param_1: *mut Struct1091) {
    let piVar1: *mut i32;
    Struct1093 * *ppaVar2;
    Struct1094 * *ppaVar3;
    let paVar4: *mut Struct1093;
    let mut u_var5: u32;
    let ppcVar6: fn();

    let mut u_var7: u16;
    let BVar8: bool;
    let mut u_var9: i32;
    let local_AX_338: *mut Struct1092;
    let local_AX_462: *mut Struct1092;
    let mut u_var10: i32;
    let mut u_var11: u32;
    let mut in_dx: u16;
    let mut u_var13: i32;
    let local_DX_239: *mut Struct1094;
    let local_DX_299: *mut Struct1093;
    let mut u_var14: i32;
    let mut u_var15: u32;
    let local_bx_4: *mut Struct1091;
    let local_es_4: *mut Struct1091;
    let mut u_var16: u32;
    let u_var17: u8;
    let mut local_20: u16;
    let mut local_1e: u16;
    let mut local_1c: u32;
    let mut local_16: u32;
    let mut local_12: u32;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;
    let mut u_var12: u32;

    local_es_4 = (param_1 >> 0x10);
    local_bx_4 = param_1;
    if (local_bx_4.field_0x214 == 0) {
        return;
    }
    u_var5 = local_bx_4.field_0x1f6;
    pass1_1030_38b8(u_var5, (u_var5 >> 0x10));
    _local_6 = CONCAT22(in_dx, in_ax);
    _local_6 = _local_6 - &local_bx_4.field_0x216;
    if (0 < _local_6) {
        _local_6 = _local_6 + 3;
        local_a = _local_6 / 5;
        u_var15 = _local_6 % 5;
        if (local_bx_4.field_0xc == 0) {
            u_var7 = 0;
            u_var15 = 0;
        } else {
            u_var5 = local_bx_4.field_0xc;
            ppcVar6 = (local_bx_4.field_0xc + 0x10);
            u_var11 = local_a;
            (**ppcVar6)(0x1030, u_var5, (u_var5 >> 0x10));
            u_var7 = u_var11;
        }
        local_e = CONCAT22(u_var15, u_var7);
        local_12 = 0;
        while (u_var14 = u_var15, u_var11 = local_e, local_12 < local_e) {
            u_var5 = local_bx_4.field_0xc;
            u_var12 = local_e;
            pass1_1030_1d7c(u_var5, (u_var5 >> 0x10), local_12, (local_12 >> 0x10));
            u_var10 = u_var12;
            u_var15 = (u_var14 | u_var10);
            if ((u_var14 | u_var10) != 0) {
                BVar8 = pass1_1008_c6ae(ctx._PTR_LOOP_1050_06e0, (u_var10 + 0xc), 4);
                u_var11 = BVar8;
                if (BVar8 != 0) {
                    u_var16 = pass1_1028_678c(u_var12 & 0xffff | u_var14 << 0x10, 0xf);
                    u_var9 = (u_var16 >> 0x10);
                    u_var13 = u_var9 | (u_var16 & 0xffff);
                    u_var15 = u_var13;
                    u_var11 = u_var16 & 0xffff;
                    if (u_var13 != 0) {
                        u_var17 = (u_var14 >> 8);
                        if (local_a < u_var16) {
                            u_var9 = local_a;
                            pass1_1028_6356(
                                CONCAT13(u_var17, CONCAT12(u_var14, u_var10)),
                                0xf,
                                u_var9,
                                local_a._2_2_,
                            );
                            u_var14 = u_var9 * 5;
                            local_DX_239 = (local_a._2_2_ * 5
                                + CARRY2(u_var9, u_var9) * 2
                                + CARRY2(u_var9 * 2, u_var9 * 2)
                                + CARRY2(u_var9 * 4, u_var9));
                            u_var15 = ZEXT24(local_DX_239);
                            ppaVar2 = &local_bx_4.field_0x216;
                            paVar4 = *ppaVar2;
                            *ppaVar2 = *ppaVar2 + u_var14;
                            ppaVar3 = &local_bx_4.field_0x218;
                            *ppaVar3 = local_DX_239 + (*ppaVar3 + CARRY2(paVar4, u_var14));
                            local_a = 0;
                            u_var11 = u_var14;
                        } else {
                            u_var13 = u_var16;
                            pass1_1028_6356(
                                CONCAT13(u_var17, CONCAT12(u_var14, u_var10)),
                                0xf,
                                u_var13,
                                u_var9,
                            );
                            local_DX_299 = (u_var9 * 5
                                + CARRY2(u_var13, u_var13) * 2
                                + CARRY2(u_var13 * 2, u_var13 * 2)
                                + CARRY2(u_var13 * 4, u_var13));
                            u_var15 = ZEXT24(local_DX_299);
                            ppaVar2 = &local_bx_4.field_0x216;
                            paVar4 = *ppaVar2;
                            *ppaVar2 = *ppaVar2 + u_var13 * 5;
                            ppaVar2 = &local_bx_4.field_0x218;
                            *ppaVar2 = local_DX_299 + (*ppaVar2 + CARRY2(paVar4, u_var13 * 5));
                            local_a = local_a - u_var16;
                            u_var11 = u_var16;
                        }
                    }
                }
                u_var14 = u_var15;
                if (local_a == 0) {
                    break;
                }
            }
            local_12 = local_12 + 1;
        }
        local_AX_338 = u_var11;
        u_var5 = local_bx_4.field_0x1f6;
        pass1_1030_38b8(u_var5, (u_var5 >> 0x10));
        _local_6 = CONCAT22(u_var14, local_AX_338);
        _local_6 = _local_6 - &local_bx_4.field_0x216;
        local_4 = (_local_6 >> 0x10);
        if ((local_4 | local_6) != 0) {
            _local_20 = _local_6 / local_bx_4.field_0x214;
            if (_local_20 < 1) {
                _local_20 = 1;
            }
            u_var5 = local_bx_4.field_0x1f6;
            pass1_1030_375a(u_var5, (u_var5 >> 0x10), 0, _local_20, (_local_20 >> 0x10));
        }
    }
    piVar1 = &local_bx_4.field_0x214;
    unsafe {
        *piVar1 = *piVar1 + -1;
    }
    return;
}

pub unsafe fn pass1_1038_387e(param_1: *mut Struct1095, param_2: u16, param_3: u16, param_4: u32) {
    let pp_var1: fn();
    let mut u_var2: u32;
    let u_var3: u8;
    let mut u_var4: u16;
    let pa_var5: *mut Struct987;
    let mut u_var6: i32;
    let extraout_var: u32;
    let mut u_var7: u32;
    let extraout_var_00: u32;
    let mut u_var8: u32;

    let mut u_var9: u16;
    
    
    
    
    let mut extraout_dx_04: i32;
    let local_bx_14: *mut Struct1095;
    let local_es_14: *mut Struct1095;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    if (param_2 != param_3) {
        local_bx_14 = param_1;
        local_es_14 = (param_1 >> 0x10);
        if (param_2 < param_3) {
            local_c = param_3 - param_2;
            if ((local_bx_14.field_0x210 == 0)
                || (u_var2 = local_bx_14.field_0x210, (u_var2 + 10) == 0))
            {
                if (local_bx_14.field_0xc == 0x0) {
                    u_var4 = 0;
                    u_var9 = 0;
                } else {
                    pp_var1 = (*local_bx_14.field_0xc + 0x10);
                    u_var4 = local_c;
                    (**pp_var1)();
                    u_var9 = ctx.dx_reg;
                }
                _local_6 = CONCAT22(u_var9, u_var4);
                local_a = 0;
                while (local_a < _local_6) {
                    u_var7 = _local_6;
                    pass1_1030_1d58(local_bx_14.field_0xc);
                    if (((ctx.dx_reg | u_var7) != 0)
                        && (
                            u_var3 = pass1_1030_6fa0((u_var7 & 0xffff | ctx.dx_reg << 0x10)),
                            CONCAT31(extraout_var, u_var3) == 0xb,
                        ))
                    {
                        pass1_1030_7c50(
                            CONCAT13((ctx.dx_reg >> 8), CONCAT12(ctx.dx_reg, u_var7)),
                            local_c,
                            4,
                        );
                        return;
                    }
                    local_a = local_a + 1;
                }
            } else {
                u_var2 = local_bx_14.field_0x210;
                u_var7 = (u_var2 + 10);
                local_a = 0;
                while (local_a < u_var7) {
                    u_var2 = local_bx_14.field_0x210;
                    u_var8 = u_var7;
                    pass1_1030_1312(u_var2, (u_var2 >> 0x10), local_a, (local_a >> 0x10));
                    pa_var5 = u_var8;
                    if ((((ctx.dx_reg | pa_var5) != 0)
                        && (
                            pass1_1030_cc44(pa_var5, ctx.dx_reg, local_c, param_4, 4),
                            pa_var5 != 0x0,
                        ))
                        && (local_c = local_c - pa_var5, local_c == 0))
                    {
                        return;
                    }
                    local_a = local_a + 1;
                }
            }
        } else {
            local_16 = param_2 - param_3;
            if ((local_bx_14.field_0x210 == 0)
                || (u_var2 = local_bx_14.field_0x210, (u_var2 + 10) == 0))
            {
                if (local_bx_14.field_0xc == 0x0) {
                    u_var4 = 0;
                    u_var9 = 0;
                } else {
                    pp_var1 = (*local_bx_14.field_0xc + 0x10);
                    u_var4 = local_16;
                    (**pp_var1)();
                    u_var9 = ctx.dx_reg;
                }
                _local_6 = CONCAT22(u_var9, u_var4);
                local_a = 0;
                while (local_a < _local_6) {
                    u_var7 = _local_6;
                    pass1_1030_1d58(local_bx_14.field_0xc);
                    if (((ctx.dx_reg | u_var7) != 0)
                        && (
                            u_var3 = pass1_1030_6fa0((u_var7 & 0xffff | ctx.dx_reg << 0x10)),
                            CONCAT31(extraout_var_00, u_var3) == 0xb,
                        ))
                    {
                        pass1_1030_6e9c(
                            CONCAT13((ctx.dx_reg >> 8), CONCAT12(ctx.dx_reg, u_var7)),
                            local_16,
                            4,
                        );
                        return;
                    }
                    local_a = local_a + 1;
                }
            } else {
                u_var2 = local_bx_14.field_0x210;
                u_var7 = (u_var2 + 10);
                local_a = 0;
                while (local_a < u_var7) {
                    u_var2 = local_bx_14.field_0x210;
                    u_var8 = u_var7;
                    pass1_1030_1312(u_var2, (u_var2 >> 0x10), local_a, (local_a >> 0x10));
                    u_var6 = u_var8;
                    if ((extraout_dx_04 | u_var6) != 0) {
                        pass1_1030_ce72(
                            extraout_dx_04 << 0x10 | u_var8 & 0xffff,
                            local_16,
                            param_4,
                            4,
                        );
                        local_16 = local_16 - u_var6;
                        if (local_16 == 0) {
                            return;
                        }
                    }
                    local_a = local_a + 1;
                }
            }
        }
    }
    return;
}

pub unsafe fn pass1_1038_3aa6(param_1: *mut Struct1096) {
    let pp_var1: fn();
    let mut u_var2: u32;
    let u_var3: u8;

    let mut u_var4: u32;
    let extraout_var: u32;
    let mut u_var5: u32;

    let mut u_var6: u16;
    
    
    let local_bx_9: *mut Struct1096;
    let mut u_var7: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_c: u32;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    u_var7 = (param_1 >> 0x10);
    local_bx_9 = param_1;
    if ((local_bx_9.field_0x210 == 0) || (u_var2 = local_bx_9.field_0x210, (u_var2 + 10) == 0)) {
        if (local_bx_9.field_0xc == 0x0) {
            in_ax = 0;
            u_var6 = 0;
        } else {
            pp_var1 = (*local_bx_9.field_0xc + 0x10);
            (**pp_var1)();
            u_var6 = ctx.dx_reg;
        }
        _local_8 = CONCAT22(u_var6, in_ax);
        local_c = 0;
        while (local_c < _local_8) {
            u_var4 = _local_8;
            pass1_1030_1d58(local_bx_9.field_0xc);
            if (((ctx.dx_reg | u_var4) != 0)
                && (
                    u_var3 = pass1_1030_6fa0((u_var4 & 0xffff | ctx.dx_reg << 0x10)),
                    CONCAT31(extraout_var, u_var3) == 0xb,
                ))
            {
                pass1_1030_6b86(u_var4 & 0xffff | ctx.dx_reg << 0x10);
                return;
            }
            local_c = local_c + 1;
        }
    } else {
        u_var2 = local_bx_9.field_0x210;
        u_var4 = (u_var2 + 10);
        local_c = 0;
        while (local_c < u_var4) {
            u_var2 = local_bx_9.field_0x210;
            u_var5 = u_var4;
            pass1_1030_1312(u_var2, (u_var2 >> 0x10), local_c, (local_c >> 0x10));
            if ((ctx.dx_reg | u_var5) != 0) {
                pass1_1030_ce2e(u_var5, ctx.dx_reg, 4);
            }
            local_c = local_c + 1;
        }
    }
    return;
}

pub unsafe fn pass1_1038_3ba0(param_1: *mut Struct1097) {
    let pu_var1: *mut u32;
    let ppc_var2: fn();
    let mut u_var3: u32;
    let pu_var4: *mut u32;
    let mut u_var5: u32;
    let mut u_var6: u32;
    let mut u_var7: i32;
    let mut u_var8: i32;
    let pu_var9: Vec<u8>;
    let mut u_var10: u32;
    
    
    let struct_a: *mut Struct199;
    let pa_var11: *mut Struct199;
    
    
    let local_bx_5: *mut Struct1097;
    let mut u_var12: u16;
    let mut u_var13: u16;
    let mut local_22: u16;
    let mut local_20: u16;
    let mut local_1c: u16;
    let mut local_1a: u16;
    let mut local_14: u32;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;

    u_var12 = (param_1 >> 0x10);
    local_bx_5 = param_1;
    pu_var1 = local_bx_5.field_0x210;
    u_var7 = &local_bx_5.field_0x212;
    if ((u_var7 | pu_var1) != 0) {
        unsafe {
            ppc_var2 = *pu_var1;
        }
        ppc_var2();
        u_var7 = ctx.dx_reg;
    }
    pu_var9 = pass1_1008_c6fa(ctx._PTR_LOOP_1050_06e0, 0x1e);
    pass1_1038_4d6e(param_1, pu_var9 & 0xffff | u_var7 << 0x10);
    u_var5 = pu_var9 & 0xffff;
    pu_var4 = (u_var5 | ctx.dx_reg << 0x10);
    unsafe {
        ppc_var2 = (*pu_var4 + 0x10);
    }
    ppc_var2(&ctx.PTR_LOOP_1050_1008, pu_var9, ctx.dx_reg);
    u_var7 = pu_var9;
    u_var6 = pu_var9 & 0xffff | ZEXT24(struct_a) << 0x10;
    if ((struct_a == 0x0) && (u_var7 < 5)) {
        u_var7 = 5;
    }
    u_var7 = u_var7 + 1;
    u_var13 = 0x1000;
    pa_var11 = struct_a;
    u_var8 = u_var7;
    process_struct_1000_179c(0x1c, struct_a);
    if ((pa_var11 | u_var8) == 0) {
        &local_bx_5.field_0x210 = 0;
    } else {
        u_var13 = 0x1030;
        pass1_1030_11aa(CONCAT22(pa_var11, u_var8), 5, u_var7, u_var7 >> 0xf);
        local_bx_5.field_0x210 = u_var7;
        &local_bx_5.field_0x212 = ctx.dx_reg;
    }
    u_var3 = &local_bx_5.field_0x210;
    (u_var3 + 0x1a) = 0;
    local_14 = 0;
    while (local_14 < u_var6) {
        u_var10 = u_var6;
        pass1_1030_1d7c(pu_var4, local_14, (local_14 >> 0x10));
        if ((ctx.dx_reg | u_var10) != 0) {
            pass1_1030_1358(
                &local_bx_5.field_0x210,
                u_var10,
                ctx.dx_reg,
                local_14 + 1,
            );
        }
        u_var13 = 0x1030;
        local_14 = local_14 + 1;
    }
    if (pu_var4 != 0x0) {
        unsafe {
            ppc_var2 = *pu_var4;
        }
        ppc_var2(u_var13, u_var5, ctx.dx_reg, 1);
    }
    return;
}

pub unsafe fn pass1_1038_3cc0(param_1: u32, param_2: u16, param_3: u16, param_4: u16) {
    let lVar1: u32;
    let ppc_var2: fn();
    let u_var3: u8;
    let mut u_var4: u16;
    let pu_var5: *mut u32;
    let extraout_AH: u8;
    let pu_var6: Vec<u8>;
    let mut u_var7: u32;
    let extraout_var: u32;
    let extraout_var_00: u32;
    let mut in_dx: i32;

    
    
    
    
    let mut extraout_dx_04: u16;
    let mut extraout_dx_05: u16;
    let mut extraout_dx_06: i32;
    let mut extraout_dx_07: i32;
    let mut u_var8: i32;
    let pu_var9: *mut u32;
    let mut u_var10: u16;
    let mut u_var11: u16;
    let mut u_var12: u32;
    let u_var13: u8;
    let u_var14: u8;
    let u_var15: u8;
    let mut local_1e: u16;
    let mut local_1c: u16;
    let local_1a: *mut Struct1099;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u32;
    let mut local_e: u16;
    let mut local_c: u16;
    let local_a: *mut Struct1098;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    if (param_3_00 == 0x1e) {
        u_var11 = SUB42(&ctx.PTR_LOOP_1050_1008, 0);
        pu_var6 = pass1_1008_c6fa(ctx._PTR_LOOP_1050_06e0, 0x27);
        pu_var9 = pu_var6;
        pass1_1038_4e78(param_1, pu_var6 & 0xffff | in_dx << 0x10);
        local_a = CONCAT22(ctx.dx_reg, pu_var9);
        ppc_var2 = (*local_a + 0x10);
        pu_var5 = pu_var9;
        ppc_var2(&ctx.PTR_LOOP_1050_1008, pu_var9, ctx.dx_reg);
        local_e = CONCAT22(ctx.dx_reg, pu_var5);
        local_12 = 0;
        while (local_12 < local_e) {
            u_var7 = local_e;
            pass1_1030_1d7c(pu_var9, ctx.dx_reg, local_12, (local_12 >> 0x10));
            if ((ctx.dx_reg | u_var7) != 0) {
                u_var12 = pass1_1030_bfb8((u_var7 & 0xffff | ctx.dx_reg << 0x10));
                u_var8 = (u_var12 >> 0x10) | u_var12;
                if (u_var8 != 0) {
                    u_var3 = pass1_1028_b58e((u_var7 & 0xffff | ctx.dx_reg << 0x10));
                    if (CONCAT22(param_3, param_2) <= u_var12) {
                        u_var11 = 0x1030;
                        pass1_1030_7ddc(
                            CONCAT31(extraout_var, u_var3) & 0xffff | u_var8 << 0x10,
                            CONCAT13((param_3 >> 8), CONCAT12(param_3, param_2)),
                            0x1e,
                        );
                        break;
                    }
                    pass1_1030_7ddc(
                        CONCAT31(extraout_var, u_var3) & 0xffff | u_var8 << 0x10,
                        u_var12,
                        0x1e,
                    );
                    lVar1 = CONCAT22(param_3, param_2) - u_var12;
                    param_2 = lVar1;
                    param_3 = (lVar1 >> 0x10);
                }
            }
            u_var11 = 0x1030;
            local_12 = local_12 + 1;
        }
        _local_1a = local_a;
        u_var10 = ctx.dx_reg;
        if (local_a == 0x0) {
            return;
        }
    } else {
        if (param_3_00 != 0x21) {
            u_var11 = SUB42(&ctx.PTR_LOOP_1050_1008, 0);
            pu_var6 = pass1_1008_c6fa(ctx._PTR_LOOP_1050_06e0, 3);
            u_var4 = SUB42(pu_var6, 0);
            pass1_1038_4e78(param_1, pu_var6 & 0xffff | in_dx << 0x10);
            _local_1a = CONCAT22(ctx.dx_reg, u_var4);
            ppc_var2 = (*_local_1a + 0x10);
            u_var10 = u_var4;
            ppc_var2(&ctx.PTR_LOOP_1050_1008, u_var4, ctx.dx_reg);
            _local_16 = CONCAT22(ctx.dx_reg, u_var10);
            local_12 = 0;
            // LAB_1038_3e9c:
            if (local_12 < _local_16) {
                u_var11 = 0x1030;
                u_var7 = _local_16;
                pass1_1030_1d7c(u_var4, ctx.dx_reg, local_12, (local_12 >> 0x10));
                if ((extraout_dx_07 | u_var7) == 0) {}
                // goto LAB_1038_3e98;
                u_var11 = SUB42(&PTR_LOOP_1050_1028, 0);
                u_var12 = pass1_1028_45e2(u_var7 & 0xffff | extraout_dx_07 << 0x10);
                u_var8 = (u_var12 >> 0x10) | u_var12;
                if (u_var8 == 0) {}
                // goto LAB_1038_3e98;
                u_var3 = pass1_1028_b58e((u_var7 & 0xffff | extraout_dx_07 << 0x10));
                if (u_var12 < CONCAT22(param_3, param_2)) {
                    u_var11 = 0x1030;
                    pass1_1030_7ddc(
                        CONCAT31(extraout_var_00, u_var3) & 0xffff | u_var8 << 0x10,
                        u_var12,
                        param_3_00,
                    );
                    lVar1 = CONCAT22(param_3, param_2) - u_var12;
                    param_2 = lVar1;
                    param_3 = (lVar1 >> 0x10);
                    // goto LAB_1038_3e98;
                }
                u_var14 = param_3;
                u_var15 = (param_3 >> 8);
                u_var13 = extraout_var_00;
                // LAB_1038_3e67:
                u_var11 = 0x1030;
                pass1_1030_7ddc(
                    CONCAT22(u_var8, CONCAT11(u_var13, u_var3)),
                    CONCAT13(u_var15, CONCAT12(u_var14, param_2)),
                    param_3_00,
                );
            }
            // goto LAB_1038_3e6c;
        }
        u_var11 = SUB42(&ctx.PTR_LOOP_1050_1008, 0);
        pu_var6 = pass1_1008_c6fa(ctx._PTR_LOOP_1050_06e0, 10);
        u_var4 = SUB42(pu_var6, 0);
        pass1_1038_4e78(param_1, pu_var6 & 0xffff | in_dx << 0x10);
        _local_1a = CONCAT22(extraout_dx_04, u_var4);
        ppc_var2 = (*_local_1a + 0x10);
        u_var10 = u_var4;
        ppc_var2(&ctx.PTR_LOOP_1050_1008, u_var4, extraout_dx_04);
        _local_16 = CONCAT22(extraout_dx_05, u_var10);
        local_12 = 0;
        while (local_12 < _local_16) {
            u_var11 = 0x1030;
            u_var7 = _local_16;
            pass1_1030_1d7c(u_var4, extraout_dx_04, local_12, (local_12 >> 0x10));
            u_var8 = extraout_dx_06 | u_var7;
            if (u_var8 != 0) {
                u_var14 = param_3;
                u_var15 = (param_3 >> 8);
                u_var3 = pass1_1028_b58e((u_var7 & 0xffff | extraout_dx_06 << 0x10));
                u_var13 = extraout_AH;
                // goto LAB_1038_3e67;
            }
            local_12 = local_12 + 1;
        }
        // LAB_1038_3e6c:
        if (_local_1a == 0x0) {
            return;
        }
        u_var10 = (_local_1a >> 0x10);
        pu_var9 = _local_1a;
    }
    unsafe {
        ppc_var2 = *pu_var9;
    }
    ppc_var2(u_var11, _local_1a, u_var10, 1);
    return;
    // LAB_1038_3e98:
    local_12 = local_12 + 1;
    // goto LAB_1038_3e9c;
}

pub unsafe fn pass1_1038_3efc(
    param_1: u16,
    param_2: u16,
    param_1_00: Vec<u8>,
    param_2_00: Vec<u8>,
) {
    let pp_var1: fn();
    let paVar2: *mut Struct493;
    let mut in_dx: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    paVar2 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, param_2_00, (param_2_00 >> 0x10));
    _local_6 = CONCAT22(in_dx, paVar2);
    &paVar2.field_0x1c = (param_1_00 + 4);
    pp_var1 = (*_local_6 + 0x58);
    (**pp_var1)(&PTR_LOOP_1050_1028, paVar2, in_dx, param_1_00);
    return;
}

pub unsafe fn pass1_1038_3222(param_1: *mut Struct848, param_2: u32, param_3: u32) {
    let mut u_var1: u32;
    let u_var2: u8;
    let mut u_var3: i32;
    let pcVar4: String;
    let extraout_var: u32;
    let pa_var5: *mut Struct199;
    let mut u_var6: i32;

    let local_bx_33: *mut Struct1084;
    let mut u_var7: u16;
    let unaff_ss: String;
    let pa_var8: *mut Struct848;
    let mut local_26: u32;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_16: [u8; 20];

    pa_var8 = pass1_1030_183c(param_1, 0, 0, 0x4000000, param_3);
    pa_var5 = (pa_var8 >> 0x10);
    u_var7 = (param_1 >> 0x10);
    local_bx_33 = param_1;
    local_bx_33.field_0x10 = param_2;
    local_bx_33.field_0x14 = 0;
    local_bx_33.field_0x18 = 600;
    local_bx_33.field_0x1a = 600;
    local_bx_33.field_0x1c = 0;
    local_bx_33.field_0x1e = 0;
    local_bx_33.field_0x22 = 0;
    local_bx_33.field_0x24 = 0x32;
    &local_bx_33.field_0x1f6 = 0;
    &local_bx_33.field_0x1fa = 0;
    local_bx_33.field_0x1fe = 0;
    local_bx_33.field_0x200 = 0x8000001;
    local_bx_33.field_0x204 = 0;
    local_bx_33.field_0x206 = 0;
    local_bx_33.field_0x208 = 1;
    local_bx_33.field_0x20a = 0;
    local_bx_33.field_0x20c = 0;
    local_bx_33.field_0x20e = 0;
    local_bx_33.field_0x210 = 0;
    local_bx_33.field_0x214 = 0;
    local_bx_33.field_0x216 = 0;
    local_bx_33.field_0x21a = 0;
    param_1.field_0x0 = 0x6504;
    local_bx_33.field_0x2 = &PTR_LOOP_1050_1038;
    pass1_1000_4906((param_1 & 0xffff0000 | &local_bx_33.field_0x26), 0, 0x94);
    pass1_1000_4906((param_1 & 0xffff0000 | &local_bx_33.field_0xba), 0, 0x94);
    pass1_1000_4906((param_1 & 0xffff0000 | &local_bx_33.field_0x14e), 0, 0x54);
    u_var2 = pass1_1000_4906((param_1 & 0xffff0000 | &local_bx_33.field_0x1a2), 0, 0x54);
    u_var3 = CONCAT31(extraout_var, u_var2);
    process_struct_1000_179c(0x1b0, pa_var5);
    u_var6 = pa_var5 | u_var3;
    if (u_var6 == 0) {
        &local_bx_33.field_0x1f6 = 0;
    } else {
        pass1_1030_314c(CONCAT22(pa_var5, u_var3), &local_bx_33.field_0x4);
        local_bx_33.field_0x1f6 = u_var3;
        local_bx_33.field_0x1f8 = u_var6;
    }
    u_var1 = &local_bx_33.field_0x4;
    pa_var5 = (&local_bx_33.field_0x6 & 0xff);
    string_fn_1000_3f9c(
        local_16,
        unaff_ss,
        s__5lu_1050_5a1a,
        &ctx.g_alloc_addr_1050_1050,
        u_var1,
    );
    pcVar4 = local_16;
    pass1_fn_1008_60e8(pcVar4, unaff_ss, u_var1);
    local_bx_33.field_0x1fa = pcVar4;
    local_bx_33.field_0x1fc = pa_var5;
    process_struct_1000_179c(0x1e, pa_var5);
    if ((pa_var5 | pcVar4) == 0) {
        &local_bx_33.field_0xc = 0;
    } else {
        pass1_1020_c444(CONCAT22(pa_var5, pcVar4), 100, 200);
        local_bx_33.field_0xc = pcVar4;
        local_bx_33.field_0xe = ctx.dx_reg;
    }
    return;
}

pub unsafe fn pass1_1038_33f8(param_1: &mut Struct44) {
    let pu_var1: *mut u32;
    let mut u_var2: i32;
    let ppc_var3: fn();
    let local_bx_5: *mut Struct850;
    let mut u_var4: u16;

    u_var4 = (param_1 >> 0x10);
    local_bx_5 = param_1;
    param_1.ptr_a_lo = 0x6504;
    local_bx_5.field_0x2 = &PTR_LOOP_1050_1038;
    pu_var1 = (local_bx_5 + 1).field_0x0;
    u_var2 = local_bx_5[1].field_0x2;
    if ((u_var2 | pu_var1) != 0) {
        unsafe {
            ppc_var3 = *pu_var1;
        }
        (**ppc_var3)();
    }
    pu_var1 = local_bx_5[0x19].field_0x2;
    u_var2 = &local_bx_5[0x19].field_0x4;
    if ((u_var2 | pu_var1) != 0) {
        unsafe {
            ppc_var3 = *pu_var1;
        }
        (**ppc_var3)();
    }
    error_check_1000_17ce(&local_bx_5[0x19].field_0x6);
    pu_var1 = &local_bx_5[0x1a].field_0x8;
    u_var2 = &local_bx_5[0x1a].field_0xa;
    if ((u_var2 | pu_var1) != 0) {
        unsafe {
            ppc_var3 = *pu_var1;
        }
        (**ppc_var3)(0x1000, pu_var1, u_var2, 1);
    }
    error_check_1000_17ce((&local_bx_5[0x1a].field_0x10 + 2));
    pass1_1030_18b2(param_1);
    return;
}

pub unsafe fn pass1_1038_349e(param_1: *mut Struct1086, param_2: u32) {
    let mut u_var1: u32;
    let ppc_var2: fn();
    let mut u_var3: u16;
    let mut u_var4: i32;
    let mut u_var5: u32;

    
    let local_bx_8: *mut Struct1086;
    let mut u_var6: u16;
    let mut u_var7: u16;
    let mut u_var8: u16;
    let local_16: *mut Struct1087;
    let mut local_14: u16;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    u_var6 = (param_1 >> 0x10);
    local_bx_8 = param_1;
    local_bx_8.field_0x200 = param_2;
    pass1_1038_4d0e(param_1, 600);
    u_var3 = param_2;
    pass1_1038_4d0e(param_1, 600);
    local_bx_8.field_0x204 = 0;
    local_bx_8.field_0x206 = 0;
    u_var1 = local_bx_8.field_0xc;
    u_var7 = u_var1;
    u_var8 = (u_var1 >> 0x10);
    ppc_var2 = (local_bx_8.field_0xc + 0x10);
    ppc_var2();
    _local_6 = CONCAT22(ctx.dx_reg, u_var3);
    local_a = 0;
    while (local_a < _local_6) {
        u_var5 = _local_6;
        pass1_1030_1d7c(local_bx_8.field_0xc, local_a, (local_a >> 0x10));
        u_var4 = u_var5;
        if ((ctx.dx_reg | u_var4) != 0) {
            ppc_var2 = ((u_var5 & 0xffff | ctx.dx_reg << 0x10) + 0x58);
            ppc_var2(
                0x1030,
                u_var4,
                ctx.dx_reg,
                param_1,
                u_var6,
                u_var7,
                u_var8,
            );
            (u_var4 + 0x1c) = 0;
        }
        local_a = local_a + 1;
    }
    return;
}

pub unsafe fn pass1_1038_354a(param_1: *mut Struct1088) {
    let in_ax: *mut Struct493;
    let in_dx: *mut Struct199;
    let mut u_var1: i32;
    let local_bx_4: *mut Struct1088;
    let mut u_var2: u16;
    let mut local_4: u16;

    u_var2 = (param_1 >> 0x10);
    local_bx_4 = param_1;
    if (&local_bx_4.field_0x21a == 0) {
        process_struct_1000_179c(10, in_dx);
        u_var1 = in_dx | in_ax;
        if (u_var1 == 0) {
            &local_bx_4.field_0x21a = 0;
        } else {
            pass1_1030_9ecc(CONCAT22(in_dx, in_ax), param_1);
            local_bx_4.field_0x21a = in_ax;
            &local_bx_4.field_0x21c = u_var1;
        }
    }
    pass1_1030_9ef2(&local_bx_4.field_0x21a);
    return;
}

pub unsafe fn pass1_1038_35a8(param_1: *mut Struct1089, param_2: u16) {
    let mut in_ax: i32;
    let in_dx: *mut Struct199;
    let mut u_var1: i32;
    let local_bx_4: *mut Struct1089;
    let mut u_var2: u16;
    let mut local_4: u16;

    u_var2 = (param_1 >> 0x10);
    local_bx_4 = param_1;
    if (&local_bx_4.field_0x21a == 0) {
        process_struct_1000_179c(10, in_dx);
        u_var1 = in_dx | in_ax;
        if (u_var1 == 0) {
            &local_bx_4.field_0x21a = 0;
        } else {
            pass1_1030_9ecc(CONCAT22(in_dx, in_ax), param_1);
            local_bx_4.field_0x21a = in_ax;
            &local_bx_4.field_0x21c = u_var1;
        }
    }
    pass1_1030_9f40(&local_bx_4.field_0x21a, param_2);
    return;
}

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
        pass1_1038_52b8(param_5, &local_bx_35.field_0x8, &local_bx_35.field_0xe);
        return;
    }
    if (i_var9 == 2) {
        local_e = param_1_00;
        if (&local_bx_35.field_0xe != 0) {
            pass1_1038_3efc(local_bx_4, u_var13, param_2_00, *&local_bx_35.field_0xe);
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
            pass1_1038_3f38(param_5, param_2_00, &local_bx_35.field_0xe);
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
                    pass1_1038_43cc(
                        local_bx_4,
                        CONCAT13((u_var16 >> 8), CONCAT12(u_var16, u_var13)),
                        &local_bx_35.field_0xe,
                    );
                }
            } else {
                if (i_var9 == 7) {
                    u_var4 = &local_bx_35.field_0xe;
                    pa_var7 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, u_var4, (u_var4 >> 0x10));
                    pass1_1038_349e(CONCAT22(u_var16, pa_var7), (i_var11 + 0x200));
                    u_var18 = (u_var16 >> 8);
                    pass1_1038_4d0e(CONCAT13(u_var18, CONCAT12(u_var16, pa_var7)), 600);
                    pass1_1038_4d0e(CONCAT13(u_var18, CONCAT12(u_var16, pa_var7)), 600);
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
        pass1_1038_3cc0(
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
            pass1_1038_5798(
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
    pass1_1038_4d6e(param_2, pu_var7 & 0xffff | in_dx << 0x10);
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
        pass1_1038_4d6e(param_2, pu_var7 & 0xffff | u_var8 << 0x10);
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
    pass1_1038_4d6e(param_2, pu_var6 & 0xffff | in_dx << 0x10);
    local_a = CONCAT22(ctx.dx_reg, i_var3);
    pp_var1 = (*local_a + 0x10);
    i_var4 = i_var3;
    (**pp_var1)(&ctx.PTR_LOOP_1050_1008, i_var3, ctx.dx_reg);
    if ((ctx.dx_reg != 0) || (i_var4 != 0)) {
        u_var7 = ctx.dx_reg;
        pu_var6 = pass1_1008_c6fa(ctx._PTR_LOOP_1050_06e0, 5);
        i_var5 = pu_var6;
        pass1_1038_4d6e(param_2, pu_var6 & 0xffff | u_var7 << 0x10);
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
        pass1_1038_4d6e(param_2, pu_var6 & 0xffff | u_var7 << 0x10);
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
    pass1_1038_4d6e(param_3, pu_var4 & 0xffff | in_dx << 0x10);
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
    pass1_1038_4d6e(param_3, pu_var5 & 0xffff | in_dx << 0x10);
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
            pass1_1038_6838(
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
            pass1_1038_675c(
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
        pass1_1038_6590(
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
                    pass1_1038_6838(
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
                    pass1_1038_675c(
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
                    pass1_1038_666e(
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
            pass1_1038_666e(
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
                    pass1_1038_666e(
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
            pass1_1038_666e(
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
        pass1_1038_4918(CONCAT22(in_dx, paVar1));
    }
    pass1_1038_7a76(_PTR_LOOP_1050_5a64);
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
        pass1_1038_4d6e(
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
    pass1_1038_4e78(param_1_00, pu_var9 & 0xffff | in_dx << 0x10);
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
    pass1_1038_4cea(
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
        pass1_1038_4cd0(param_2_00, local_a, local_c);
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
    pass1_1038_4cea(
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
        pass1_1038_4cd0(param_2_00, local_a, local_c);
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
            pass1_1038_4760(local_AX_122, ctx.dx_reg);
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
            pass1_1038_4d0e(_local_26, local_36);
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
        paVar2 = pass1_1038_4fd8(CONCAT22(in_dx, paVar2), 0x21);
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
                pass1_1038_354a(CONCAT22(u_var5, paVar4));
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
                    pass1_1038_35a8(CONCAT22(u_var5, paVar4), ((local_14 + 0x10) * 2 + local_2a));
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
            pass1_1038_4d6e(CONCAT22(in_dx, paVar4), pu_var7 & 0xffff | u_var9 << 0x10);
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

// WARNING: This function may have set the stack p i32er

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
