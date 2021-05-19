use crate::bad_funcs::bad1::halt_baddata;
use crate::draw::draw2::draw_1020_3fa0;
use crate::err_funcs::error_check_1000_17ce;
use crate::list_funcs::zero_list_1008_6c90;
use crate::mem_funcs::{alloc_mem_1000_07fc, alloc_mem_1000_0ed4, alloc_mem_1000_1708, free_mem_1000_093a};
use crate::mixed_fn_1010_830a;
use crate::other_funcs::{modify_list_1008_3f62, return_1_1020_79ae, zero_list_1008_3e38};
use crate::pass::pass12_funcs::{pass1_1008_c6ae, pass1_1008_c6fa};
use crate::pass::pass13_funcs::bad_func_1008_8fc4;
use crate::pass::pass14_funcs::{pass1_1008_3e54, pass1_1008_3e76, pass1_1008_3e94, pass1_1008_3eb4, pass1_1008_3f32, pass1_1008_4480, pass1_1008_5118, pass1_1008_5134, pass1_1008_5236, pass1_1008_5784, pass1_1008_5b12, pass1_1008_6cec, pass1_1008_8b20, pass1_1008_8c4e, pass1_fn_1008_612e};
use crate::pass::pass17_funcs::{pass1_1030_1cd8, pass1_1030_1d28, pass1_1030_1d58, pass1_1030_1d7c, pass1_1030_25b2, pass1_1030_2fac, pass1_1030_5b1c, pass1_1030_5b3e, pass1_1030_5b5c, pass1_1030_627e, pass1_1030_64ce, pass1_1030_6522, pass1_1030_6fa0, pass1_1030_73a8, pass1_1030_7ddc, pass1_1030_8308, pass1_1030_835a};
use crate::pass::pass18_funcs::{pass1_1030_dc96, pass1_1030_dcc2, pass1_1030_dcf4, pass1_1030_df0c};
use crate::pass::pass20_funcs::{pass1_1010_9674, pass1_1010_96a8, pass1_1010_96c2, pass1_1010_988c, pass1_1018_04b8, pass1_1018_0902};
use crate::pass::pass3_funcs;
use crate::pass::pass4_funcs::{pass1_1028_b354, pass1_1028_b39e, pass1_1028_b418, pass1_1028_b46e, pass1_1028_b4f2, pass1_1028_b514, pass1_1028_b58e, pass1_1028_bab6, pass1_1028_bb24, pass1_1028_bdac, pass1_1028_be9e, pass1_1028_c1f8, pass1_1028_c23e, pass1_1028_c314, pass1_1028_c3aa, pass1_1028_c7b6, pass1_1028_c89c, pass1_1028_c8ee, pass1_1028_c952, pass1_1028_cb04, pass1_1028_ccd0, pass1_1028_dc52, pass1_1028_e1ec, pass1_1028_e4ec};
use crate::pass::pass5_funcs::{pass1_1030_bcae, pass1_1030_bcbc, pass1_1030_bcde, pass1_1030_bd74};
use crate::pass::pass6_funcs::{pass1_1038_3fb0, pass1_1038_43cc, pass1_1038_4e78, pass1_1038_57dc, pass1_1038_5804};
use crate::pass::pass7_funcs::{pass1_1018_1662, pass1_1018_1b02, pass1_1018_2548, pass1_1018_2646, pass1_1018_26d8, pass1_1018_26f8, pass1_1018_280c, pass1_1018_2862};
use crate::pass::pass8_funcs::{pass1_1010_043a, pass1_1010_65d0};
use crate::pass::pass_funcs::{pass1_1000_4906, pass1_1000_49c6, pass1_1000_4aea};
use crate::prog_structs::prog_structs_1::{Struct104, Struct393, Struct690, Struct692, Struct695, Struct697, Struct698, Struct699};
use crate::prog_structs::prog_structs_11::Struct706;
use crate::prog_structs::prog_structs_12::Struct94;
use crate::prog_structs::prog_structs_13::Struct709;
use crate::prog_structs::prog_structs_16::Struct493;
use crate::prog_structs::prog_structs_18::Struct743;
use crate::prog_structs::prog_structs_19::Struct501;
use crate::prog_structs::prog_structs_2::{Struct199, Struct318, Struct667, Struct696, Struct705};
use crate::prog_structs::prog_structs_20::{Struct712, Struct725};
use crate::prog_structs::prog_structs_23::Struct726;
use crate::prog_structs::prog_structs_24::Struct2111;
use crate::prog_structs::prog_structs_26::{Struct700, Struct704};
use crate::prog_structs::prog_structs_27::Struct713;
use crate::prog_structs::prog_structs_28::Struct1012;
use crate::prog_structs::prog_structs_29::{Struct701, Struct702, Struct703, Struct710, Struct714, Struct715, Struct716, Struct717, Struct718, Struct719, Struct720, Struct721, Struct722};
use crate::prog_structs::prog_structs_30::Struct417;
use crate::prog_structs::prog_structs_4::{Struct502, Struct651, Struct654, Struct655, Struct657};
use crate::prog_structs::prog_structs_5::{Struct658, Struct680, Struct681, Struct682, Struct683, Struct684, Struct685, Struct687, Struct688, Struct689};
use crate::prog_structs::prog_structs_7::{Struct376, Struct44};
use crate::prog_structs::prog_structs_9::{Struct723, Struct724};
use crate::struct_funcs::{process_struct_1000_179c, process_struct_1008_4772, process_struct_1008_50c2, process_struct_1020_2594, process_struct_1020_808e, struct_fn_1000_160a};
use crate::sys_funcs::{post_win_msg_1008_a0e4, process_win_msg_1008_9510};
use crate::sys_structs::RECT16;
use crate::ui_funcs::ui2::{gui_window_func_1038_b72e, pass1_1038_af40};
use crate::util::{CARRY1, CARRY2, CONCAT11, CONCAT12, CONCAT13, CONCAT22, CONCAT31, SBORROW2, SUB42, ZEXT24};
use crate::winapi_funcs::InvalidateRect16;

pub fn pass1_1020_e76c(param_1: &mut Struct44, param_2: u8) -> &mut Struct44 {
    pass1_1030_dcf4(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1020_e7fa(param_1: *mut u16) {
    pass1_1028_b354(param_1);
    unsafe {
        *param_1 = 0xe88e;
        (param_1 + 2) = 0x1020
    };
    return param_1;
}

pub fn pass1_1020_e81c(param_1: i32, param_2: u16, param_3: u16, param_3_00: u32) {
    pass1_1028_b39e(CONCAT22(param_2, param_1), param_3, param_3_00);
    CONCAT22(param_2, param_1) = 0xe88e;
    (param_1 + 2) = 0x1020;
    return CONCAT22(param_2, param_1);
}

pub fn pass1_1020_e846(param_1: &mut Struct44) {
    param_1.ptr_a_lo = 0xe88e;
    (param_1 + 2) = 0x1020;
    pass1_1028_b418(param_1);
    return;
}

pub fn pass1_1020_e868(param_1: &mut Struct44, param_2: u8) -> &mut Struct44 {
    pass1_1020_e846(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1020_e8f6(param_1: *mut u16) {
    let mut u_var1: u16;

    pass1_1030_dc96(param_1);
    u_var1 = (param_1 >> 0x10);
    (param_1 + 0x24) = 0;
    unsafe {
        *param_1 = 0xeef6;
        (param_1 + 2) = 0x1020
    };
    return param_1;
}

pub fn pass1_1020_e91e(param_1: *mut Struct1012, param_2: u16, param_3: u16, param_3_00: u32) {
    pass1_1030_dcc2(param_1, param_2, param_3, param_3_00);
    (param_1 + 1) = 0;
    CONCAT22(param_2, param_1) = 0xeef6;
    param_1.field_0x2 = 0x1020;
    return CONCAT22(param_2, param_1);
}

pub fn pass1_1020_d4ca(param_1: &mut Struct44) {
    let u_var1: u8;
    let mut u_var2: u16;
    let extraout_var: u32;
    let pu_var3: Vec<u8>;
    let mut in_dx: u16;

    let mut i_var4: i32;
    let mut local_6: u16;
    let mut local_4: u16;

    if ((param_1 + 0x20) == 2) {
        return;
    }
    u_var1 = pass1_1028_b58e(param_1);
    pu_var3 = *(CONCAT31(extraout_var, u_var1) + 0x2e);
    i_var4 = 99;
    pass1_1038_3fb0(pu_var3);
    u_var2 = pass1_1030_25b2(pu_var3 & 0xffff | ctx.dx_reg << 0x10, i_var4);
    if (u_var2 != 0) {
        return;
    }
    return;
}

pub fn pass1_1020_d518(param_1: &mut Struct44, param_2: u8) -> &mut Struct44 {
    pass1_1028_b418(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1020_d5a6(param_1: Vec<u8>) {
    pass1_1028_b354(param_1);
    param_1 = 0xd7fe;
    (param_1 + 2) = 0x1020;
    return param_1;
}

pub fn pass1_1020_d5c8(param_1: i32, param_2: u16, param_3: u16, param_3_00: Vec<u8>) {
    pass1_1028_b39e(CONCAT22(param_2, param_1), param_3, param_3_00);
    CONCAT22(param_2, param_1) = 0xd7fe;
    (param_1 + 2) = 0x1020;
    return CONCAT22(param_2, param_1);
}

pub fn pass1_1020_d5f2(param_1: Vec<u8>, param_2: Vec<u8>) {
    let pp_var1: fn();
    let u_var2: u8;
    let pu_var3: *mut u32;
    let paVar4: *mut Struct493;
    let extraout_var: u32;
    let mut in_dx: u16;

    let mut u_var5: u16;
    let mut unaff_ss: u16;
    let mut u_var6: u32;
    let pu_var7: *mut u32;
    let mut u_var8: u32;
    let mut local_3a: u32;
    let mut local_36: u16;
    let mut local_34: u16;
    let mut local_32: [u8; 10];
    let mut local_28: u32;
    let mut local_24: u32;
    let mut local_1c: u16;
    let mut local_1a: u32;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u32;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    u_var2 = pass1_1028_b58e(param_1);
    local_6 = CONCAT31(extraout_var, u_var2);
    local_c = (local_6 + 0xc);
    local_12 = (local_6 + 0x10);
    local_1c = &local_c;
    local_e = local_12;
    local_8 = local_12;
    pass1_1028_bab6(param_1);
    local_14 = pass1_1030_2fac(CONCAT22(in_dx, local_12));
    local_1a = local_c;
    local_16 = local_8;
    local_24 = CONCAT22(local_24._2_2_, &local_1a);
    local_e = local_e + 1;
    if (local_e < local_14) {
        u_var8 = CONCAT22(unaff_ss, local_32);
        local_16 = local_e;
        u_var6 = pass1_1028_bb24(param_1);
        pu_var3 = &local_1a;
        pass1_1030_64ce(
            _PTR_LOOP_1050_5740,
            pu_var3,
            unaff_ss,
            u_var6,
            (u_var6 >> 0x10),
            u_var8,
        );
        unsafe { local_28 = *pu_var3 };
        u_var5 = (pu_var3 + 2);
        local_3a._3_1_ = (local_28 >> 0x18);
        local_24 = local_28;
        if (local_3a._3_1_ != '\0') {
            paVar4 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, local_28, u_var5);
            pu_var7 = pass1_1030_73a8(CONCAT22(u_var5, paVar4));
            unsafe { pp_var1 = (*pu_var7 + 0x58) };
            (**pp_var1)(0x1030, pu_var7, (pu_var7 >> 0x10), param_2);
        }
    }
    pass1_1028_b46e(param_1, param_2);
    return;
}

pub fn pass1_1020_d6e6(param_1: &mut Struct44) {
    let pp_var1: fn();
    let u_var2: u8;
    let pu_var3: *mut u32;
    let paVar4: *mut Struct493;
    let extraout_var: u32;
    let mut in_dx: u16;

    let mut u_var5: u16;
    let mut unaff_ss: u16;
    let mut u_var6: u32;
    let pu_var7: *mut u32;
    let mut u_var8: u32;
    let mut local_3a: u32;
    let mut local_36: u16;
    let mut local_34: u16;
    let mut local_32: [u8; 10];
    let mut local_28: u32;
    let mut local_24: u32;
    let mut local_1c: u16;
    let mut local_1a: u32;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u32;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    pass1_1028_b514(param_1);
    u_var2 = pass1_1028_b58e(param_1);
    local_6 = CONCAT31(extraout_var, u_var2);
    local_c = (local_6 + 0xc);
    local_12 = (local_6 + 0x10);
    local_1c = &local_c;
    local_e = local_12;
    local_8 = local_12;
    pass1_1028_bab6(param_1);
    local_14 = pass1_1030_2fac(CONCAT22(in_dx, local_12));
    local_1a = local_c;
    local_24 = CONCAT22(local_24._2_2_, &local_1a);
    local_16 = local_e + 1;
    if (local_16 < local_14) {
        u_var8 = CONCAT22(unaff_ss, local_32);
        local_e = local_16;
        u_var6 = pass1_1028_bb24(param_1);
        pu_var3 = &local_1a;
        pass1_1030_64ce(
            _PTR_LOOP_1050_5740,
            pu_var3,
            unaff_ss,
            u_var6,
            (u_var6 >> 0x10),
            u_var8,
        );
        unsafe { local_28 = *pu_var3 };
        u_var5 = (pu_var3 + 2);
        local_3a._3_1_ = (local_28 >> 0x18);
        if (local_3a._3_1_ != '\0') {
            local_24 = local_28;
            paVar4 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, local_28, u_var5);
            pu_var7 = pass1_1030_73a8(CONCAT22(u_var5, paVar4));
            if ((pu_var7 + 0xc) == 0x6a) {
                unsafe {
                    pp_var1 = (*pu_var7 + 0x24);
                    (**pp_var1)()
                };
            }
        }
    }
    return;
}

pub fn pass1_1020_d7d8(param_1: &mut Struct44, param_2: u8) -> &mut Struct44 {
    pass1_1028_b418(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1020_d866(param_1: *mut u16) {
    pass1_1028_b354(param_1);
    unsafe {
        *param_1 = 0xd8ec;
        (param_1 + 2) = 0x1020
    };
    return param_1;
}

pub fn pass1_1020_d888(param_1: i32, param_2: u16, param_3: u16, param_3_00: u32) -> i32 {
    let mut in_stack_0000000c: u16;

    pass1_1028_b39e(CONCAT22(param_2, param_1), param_3, _param_3_00);
    *CONCAT22(param_2, param_1) = 0xd8ec;
    (param_1 + 2) = 0x1020;
    return CONCAT22(param_2, param_1);
}

pub fn pass1_1020_d8c6(param_1: &mut Struct44, param_2: u8) -> &mut Struct44 {
    pass1_1028_b418(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1020_d954(param_1: *mut Struct723) {
    let mut unaff_bp: u16;
    let ppVar1: *mut Struct2111;
    let local_6: *mut Struct723;
    let local_4: *mut Struct723;

    local_6 = param_1;
    local_4 = (param_1 >> 0x10);
    pass1_1030_dc96(param_1);
    local_6.field_0x24 = 0;
    local_6.field_0x26 = 0;
    &local_6.field_0x28 = 0;
    param_1.field_0x0 = 0xe792;
    local_6.field_0x2 = 0x1020;
    ppVar1 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(unaff_bp, 0x2f));
    local_6.field_0x28 = ppVar1;
    local_6.field_0x2a = (ppVar1 >> 0x10);
    return;
}

pub fn pass1_1020_d99e(
    param_1: *mut Struct724,
    param_2: u16,
    param_3: u16,
    param_4: u32,
) -> *mut Struct724 {
    let local_bx_22: *mut Struct724;
    let mut unaff_bp: u16;
    let ppVar1: *mut Struct2111;
    let paVar2: *mut Struct1012;
    let mut u_var3: u16;

    paVar2 = param_1;
    u_var3 = (param_1 >> 0x10);
    pass1_1030_dcc2(paVar2, u_var3, param_3, param_4);
    (paVar2 + 1) = param_2;
    paVar2[1].field_0x2 = 0;
    &paVar2[1].field_0x4 = 0;
    param_1 = 0xe792;
    paVar2.field_0x2 = 0x1020;
    ppVar1 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(unaff_bp, 0x2f));
    &paVar2[1].field_0x4 = ppVar1;
    &paVar2[1].field_0x6 = (ppVar1 >> 0x10);
    &paVar2.field_0x10 = 0x49;
    return param_1;
}

pub fn pass1_1020_d9fa(param_1: Vec<u8>, param_2: u16) {
    let u_var1: u8;
    let extraout_var: u32;

    let mut u_var2: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    if ((param_1 + 0xc) != 0x79) {
        pass1_1030_df0c(param_1, param_2);
        u_var2 = ctx.dx_reg;
        u_var1 = pass1_1028_b58e(param_1);
        pass1_1038_57dc(
            *(CONCAT31(extraout_var, u_var1) + 0x2e),
            0x1,
            &dos_alloc_addr_1050_0002,
        );
    }
    return;
}

pub fn pass1_1020_da3c(param_1: u32) {
    pass1_1028_bdac(param_1, 2);
    return;
}

pub fn pass1_1020_da4e(param_1: *mut u32, param_2: u32, param_3: u32, param_4: u32) {
    let pp_var1: fn();
    let pu_var2: *mut u32;
    let paVar3: *mut Struct493;
    let mut u_var4: u16;

    let mut u_var5: i32;
    let mut u_var6: i32;
    let mut unaff_ss: u16;
    let mut u_var7: u32;
    let mut u_var8: u32;
    let mut u_var9: u32;
    let mut u_var10: u32;
    let mut local_22: u32;
    let mut local_1e: u16;
    let mut local_1c: u16;
    let mut local_18: u32;
    let mut local_e: u32;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;

    pu_var2 = &local_e;
    pass1_1030_64ce(_PTR_LOOP_1050_5740, param_2, param_4, pu_var2, unaff_ss);
    unsafe { local_6 = *pu_var2 };
    u_var4 = (pu_var2 + 2);
    local_22._3_1_ = (local_6 >> 0x18);
    u_var6 = local_22._3_1_;
    if (local_22._3_1_ != 0) {
        paVar3 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, local_6, u_var4);
        u_var7 = pass1_1030_73a8(CONCAT22(u_var4, paVar3));
        u_var6 = u_var7;
        if ((u_var6 + 0xc) == 0x10) {
            PTR_LOOP_1050_50ca = 0x6a9;
            return;
        }
    }
    pass1_1028_c7b6(param_1, param_2, param_4);
    u_var5 = (param_1 >> 0x10);
    u_var8 = param_1 & 0xffff | u_var5 << 0x10;
    unsafe { pp_var1 = (*param_1 + 0x60) };
    u_var7 = param_2;
    u_var9 = param_3;
    u_var10 = param_4;
    local_8 = u_var6;
    (**pp_var1)();
    if (((u_var6 != 0)
        && (
            pass1_1028_c23e(param_1, u_var5, param_2, param_3, param_4),
            u_var6 != 0,
        ))
        && (
            u_var4 = pass1_1028_c314(
                param_1,
                u_var5,
                param_2,
                param_3,
                (param_3 >> 0x10),
                param_4,
            ),
            u_var4 != 0,
        ))
    {
        u_var6 = (param_2 >> 0x10);
        if ((((param_2 + 4) == 0) && (local_8 != 4))
            && (
                unsafe { pp_var1 = (*param_1 + 0x5c) },
                (**pp_var1)(
                    &PTR_LOOP_1050_1028,
                    param_1,
                    param_2,
                    param_3,
                    param_4,
                    u_var8,
                    u_var7,
                    u_var9,
                    u_var10,
                ),
                u_var4 == 0,
            ))
        {
            return;
        }
        local_a = (param_2 + 4);
        if (local_a != 0) {
            win_fn_1020_df10(param_1, param_2 & 0xffff | u_var6 << 0x10, param_4);
            return;
        }
        pass1_1020_deac(param_1, param_2 & 0xffff | u_var6 << 0x10, param_4);
        return;
    }
    return;
}

pub fn pass1_1020_db86(
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
    let mut local_14: u16;
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
    u_var5 = param_1_00;
    paVar2 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, u_var1, (u_var1 >> 0x10));
    pu_var3 = local_4;
    pass1_1030_bcde(pu_var3, unaff_ss, CONCAT22(u_var4, paVar2), u_var5, param_5);
    if (pu_var3 < 0) {
        PTR_LOOP_1050_50ca = 0x6af;
    } else {
        if ((pu_var3 < 0x1f) || ((param_1_00 + 4) < 1)) {
            return;
        }
        PTR_LOOP_1050_50ca = 0x6b6;
        PTR_LOOP_1050_50cc = pu_var3 + -0x1e;
    }
    return;
}

pub fn pass1_1020_dc1c(param_1: u32, param_2: u32) {
    let mut i_var1: i32;
    let ppc_var2: fn();
    let pu_var3: *mut u32;
    let paVar4: *mut Struct493;

    let mut u_var5: u16;
    let mut unaff_ss: u16;
    let mut u_var6: u32;
    let pu_var7: *mut u32;
    let mut u_var8: u32;
    let mut local_1e: u32;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_14: u32;
    let mut local_a: [u8; 4];
    let mut local_6: u32;

    u_var8 = CONCAT22(unaff_ss, local_a);
    u_var6 = pass1_1028_bb24(param_1);
    pu_var3 = u_var6;
    pass1_1030_64ce(
        _PTR_LOOP_1050_5740,
        param_2,
        pu_var3,
        (u_var6 >> 0x10),
        u_var8,
    );
    unsafe { local_6 = *pu_var3 };
    u_var5 = (pu_var3 + 2);
    local_1e._3_1_ = (local_6 >> 0x18);
    if (local_1e._3_1_ != '\0') {
        paVar4 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, local_6, u_var5);
        pu_var7 = pass1_1030_73a8(CONCAT22(u_var5, paVar4));
        i_var1 = (pu_var7 + 0xc);
        if (((i_var1 < 1) || (SBORROW2(i_var1, 1)))
            || (i_var1 != 9 && 7 < i_var1 + -1 && (i_var1 + -9 < 0x6a || (6 < i_var1 + -0x73))))
        {
            unsafe {
                ppc_var2 = (*pu_var7 + 0x24);
                ppc_var2()
            };
        }
    }
    return;
}

pub fn pass1_1020_dca8(param_1: &mut Struct44) {
    let mut u_var1: u16;
    let mut u_var2: u32;
    let mut u_var3: u16;
    let mut u_var4: u16;
    let mut u_var5: u16;
    let mut u_var6: u16;

    let mut u_var7: u16;
    let mut u_var8: u16;

    let mut local_DXAX_29: u32;
    let mut local_2e: [u8; 14];
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

    pass1_1028_c1f8(
        param_1,
        (param_1 >> 0x10),
        local_6,
        ctx.stack_seg_reg,
        &local_4,
        ctx.stack_seg_reg,
    );
    local_DXAX_29._2_2_ = ctx.dx_reg;
    local_DXAX_29._0_1_ = pass1_1028_b58e(param_1);
    u_var2 = (CONCAT11(local_DXAX_29._1_1_, local_DXAX_29) + 0xc);
    u_var1 = (CONCAT11(local_DXAX_29._1_1_, local_DXAX_29) + 0x10);
    local_10 = u_var2;
    u_var3 = (u_var2 >> 0x10);
    u_var4 = local_10 - 1;
    if (u_var4 < 0) {
        u_var4 = 0;
    }
    u_var7 = local_4 - 1;
    u_var5 = local_10 + 1;
    if (u_var7 < (local_10 + 1)) {
        u_var5 = u_var7;
    }
    u_var6 = u_var3 - 1;
    if (u_var6 < 0) {
        u_var6 = 0;
    }
    u_var8 = u_var3 + 1;
    if (u_var7 < (u_var3 + 1)) {
        u_var8 = u_var7;
    }
    pass1_1008_3e54(CONCAT22(ctx.stack_seg_reg, local_2e), u_var1, u_var6, u_var4);
    pass1_1020_dc1c(param_1, CONCAT22(ctx.stack_seg_reg, local_2e));
    pass1_1008_3e54(CONCAT22(ctx.stack_seg_reg, local_2e), u_var1, u_var6, local_10);
    pass1_1020_dc1c(param_1, CONCAT22(ctx.stack_seg_reg, local_2e));
    pass1_1008_3e54(CONCAT22(ctx.stack_seg_reg, local_2e), u_var1, u_var6, u_var5);
    pass1_1020_dc1c(param_1, CONCAT22(ctx.stack_seg_reg, local_2e));
    pass1_1008_3e54(CONCAT22(ctx.stack_seg_reg, local_2e), u_var1, u_var3, u_var4);
    pass1_1020_dc1c(param_1, CONCAT22(ctx.stack_seg_reg, local_2e));
    pass1_1008_3e54(CONCAT22(ctx.stack_seg_reg, local_2e), u_var1, u_var3, u_var5);
    pass1_1020_dc1c(param_1, CONCAT22(ctx.stack_seg_reg, local_2e));
    pass1_1008_3e54(CONCAT22(ctx.stack_seg_reg, local_2e), u_var1, u_var8, u_var4);
    pass1_1020_dc1c(param_1, CONCAT22(ctx.stack_seg_reg, local_2e));
    pass1_1008_3e54(CONCAT22(ctx.stack_seg_reg, local_2e), u_var1, u_var8, local_10);
    pass1_1020_dc1c(param_1, CONCAT22(ctx.stack_seg_reg, local_2e));
    pass1_1008_3e54(CONCAT22(ctx.stack_seg_reg, local_2e), u_var1, u_var8, u_var5);
    pass1_1020_dc1c(param_1, CONCAT22(ctx.stack_seg_reg, local_2e));
    return;
}

pub fn win_fn_1020_de32(param_1: u32, param_2: u16) {
    let mut i_var1: i32;
    let mut u_var2: u16;
    let mut u_var3: u16;
    let ppVar4: *mut Struct2111;
    let mut in_stack_0000ffee: u32;
    let in_string_1: String;
    let mut local_6: u16;
    let mut local_4: u16;

    in_string_1 = CONCAT22((in_stack_0000ffee >> 0x10), 5);
    ppVar4 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, in_string_1);
    u_var2 = (ppVar4 >> 0x10);
    (ppVar4 + 0x12) = param_2;
    u_var3 = (in_string_1 >> 0x10);
    i_var1 = gui_window_func_1038_b72e(ctx._g_Struct112_a, (ctx._g_Struct112_a >> 0x10), 4);
    if (i_var1 == 0) {
        pass1_1038_af40(ctx._g_Struct112_a, *(_PTR_LOOP_1050_4230 + 0x16), 4);
    }
    PTR_LOOP_1050_5b80 = (&ctx.PTR_LOOP_1050_0000 + 1);
    process_win_msg_1008_9510(&PTR_LOOP_1050_5b80, &ctx.g_alloc_addr_1050_1050, u_var3);
    u_var3 = (param_1 >> 0x10);
    (param_1 + 0x24) = (ppVar4 + 10);
    if ((param_1 + 0x24) == 0) {
        PTR_LOOP_1050_50ca = 0x6b2;
    }
    return;
}

pub fn pass1_1020_deac(param_1: u32, param_2: u32, param_3: u32) -> bool {
    let mut i_var1: i32;
    let mut u_var2: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    i_var1 = pass1_1028_c7b6(param_1, param_2, param_3);
    if (i_var1 < 1) {
        return 0;
    }
    if (SBORROW2(i_var1, 1)) {
        return 0;
    }
    u_var2 = (param_1 >> 0x10);
    if (i_var1 != 3 && 0 < i_var1 + -2) {
        if (i_var1 == 4) {
            win_fn_1020_de32(param_1, 4);
            if ((param_1 + 0x24) == 6) {
                return 1;
            }
            return 0;
        }
        if (i_var1 != 5) {
            return 0;
        }
    }
    (param_1 + 0x24) = 1;
    return 1;
}

pub fn win_fn_1020_df10(param_1: u32, param_2: u32, param_3: u32) {

    let pu_var1: *mut u32;
    let paVar2: *mut Struct493;
    let mut u_var3: u16;

    let mut u_var4: u16;
    let mut unaff_ss: u16;
    let mut u_var5: u32;
    let mut local_22: u32;
    let mut local_1e: u16;
    let mut local_1c: u16;
    let mut local_18: u32;
    let mut local_e: u32;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    local_4 = 0;
    pass1_1028_c7b6(param_1, param_2, param_3);
    u_var4 = (param_1 >> 0x10);
    if (in_ax == 0) {
        pu_var1 = &local_e;
        pass1_1030_64ce(_PTR_LOOP_1050_5740, param_2, param_3, pu_var1, unaff_ss);
        unsafe { local_a = *pu_var1 };
        u_var3 = (pu_var1 + 2);
        local_22._3_1_ = (local_a >> 0x18);
        if (local_22._3_1_ != '\0') {
            paVar2 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, local_a, u_var3);
            u_var5 = pass1_1030_73a8(CONCAT22(u_var3, paVar2));
            if ((u_var5 + 0xc) == 0x6a) {
                u_var3 = pass1_1020_e044(param_1);
                if (u_var3 == 0) {
                    (param_1 + 0x24) = 1;
                } else {
                    PTR_LOOP_1050_50ca = 0x6ac;
                }
            }
        }
    } else {
        if (((5 < in_ax) && (!SBORROW2(in_ax, 6))) && ((in_ax - 6) < 4)) {
            win_fn_1020_de32(param_1, in_ax);
            match (param_1 + 0x24) {
                1 => {
                    u_var3 = pass1_1020_e044(param_1);
                    if (u_var3 != 0) {
                        PTR_LOOP_1050_50ca = 0x6ac;
                    }
                }
                2 | 3 | 4 | 5 => {
                    pass1_1020_e652(param_1, param_2, (param_2 >> 0x10), param_3);
                }
            }
        }
    }
    return;
}

pub fn pass1_1020_e044(param_1: u32) {
    let mut u_var1: u32;
    let paVar2: *mut Struct493;
    let mut u_var3: u16;
    let mut u_var4: u16;
    let mut u_var5: u32;
    let mut local_e: u16;
    let mut local_c: u16;

    u_var4 = (param_1 >> 0x10);
    u_var5 = pass1_1018_04b8((param_1 + 0x28));
    u_var3 = (u_var5 >> 0x10);
    paVar2 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, u_var5, u_var3);
    u_var3 = pass1_1030_2fac(CONCAT22(u_var3, paVar2));
    u_var1 = (param_1 + 0x28);
    if (u_var3 <= (u_var1 + 0x1e)) {
        return 1;
    }
    return 0;
}

pub fn pass1_1020_e08e(param_1: &mut Struct44) {
    let mut i_var1: i32;
    let mut u_var2: u16;
    let u_var3: u8;
    let mut i_var4: i32;
    let pa_var5: *mut Struct493;
    let extraout_var: u32;

    let mut local_DX_47: u16;


    let mut local_DX_314: u16;
    let local_bx_6: *mut Struct725;
    let mut local_es_6: u16;

    let local_16c: u8;
    let uStack363: u8;
    let mut local_16a: u16;
    let puVar6: *mut u16;
    let mut local_166: u16;
    let mut local_158: u16;
    let mut local_156: u16;
    let mut local_32: u32;
    let mut local_2a: u16;
    let mut local_22: u16;
    let mut local_20: [u8; 2];
    let mut local_1e: [u8; 2];
    let mut local_1c: u16;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: u32;
    let mut local_10: u32;
    let mut local_c: u16;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;
    let mut temp_5ffa9150db: u32;
    let mut temp_5f0d0368a6: u32;

    local_es_6 = (param_1 >> 0x10);
    local_bx_6 = param_1;
    i_var1 = local_bx_6.field_0xc;
    if (i_var1 == 0x74) {
        i_var1 = local_bx_6.field_0x24;
        if (i_var1 == 1) {}
        // goto LAB_1020_e0ae;
        if (i_var1 != 6) {}
        // goto LAB_1020_e0b9;
        local_166 = 1;
    } else {
        if (i_var1 == 0x78) {
            i_var1 = local_bx_6.field_0x24;
            i_var4 = i_var1 + -1;
            if (i_var4 != 0) {
                if ((0 < i_var4) && (!SBORROW2(i_var4, 1))) {
                    if (i_var1 == 5 || i_var1 + -2 < 3) {
                        pass1_1020_e49a(local_bx_6, local_es_6);
                        local_DX_47 = ctx.dx_reg;
                    } else {
                        if (i_var1 == 6) {
                            pass1_1020_e39c(param_1, 6);
                            pass1_1020_dca8(local_bx_6, local_es_6);
                            local_DX_47 = ctx.dx_reg;
                        }
                    }
                }
                // goto LAB_1020_e0b9;
            }
            local_166 = 0x6a;
        } else {
            if (i_var1 == 0x79) {
                pass1_1020_e49a(local_bx_6, local_es_6);
                return;
            }
            // LAB_1020_e0ae:
            local_166 = 0x47;
        }
    }
    pass1_1020_e39c(param_1, local_166);
    local_DX_47 = ctx.dx_reg;
    // LAB_1020_e0b9:
    u_var3 = pass1_1028_b58e(param_1);
    i_var1 = CONCAT31(extraout_var, u_var3);
    _local_6 = CONCAT31(extraout_var, u_var3) & 0xffff | local_DX_47 << 0x10;
    local_a = (i_var1 + 0x2e);
    u_var2 = (i_var1 + 0x30);
    if (local_bx_6.field_0xc != 0x79) {
        local_16c = u_var2;
        uStack363 = (u_var2 >> 8);
        pass1_1038_5804(
            CONCAT13(uStack363, CONCAT12(local_16c, local_a)),
            1,
            &dos_alloc_addr_1050_0002,
        );
    }
    if (local_bx_6.field_0x24 == 6) {
        local_16a = (local_a >> 0x10);
        pass1_1038_43cc(local_a, CONCAT22(1, local_16a), 2);
    }
    local_10 = (local_6 + 0xc);
    local_c = (local_6 + 0x10);
    local_2a = &local_10;
    if ((local_bx_6.field_0x24 == 6) && (local_c == 0)) {
        return;
    }
    temp_5ffa9150db = local_bx_6.field_0x28;
    local_14 = (temp_5ffa9150db + 0x20);
    puVar6 = &local_16;
    local_DX_314 = &local_18;
    pa_var5 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, local_14, (local_14 >> 0x10));
    pass1_1030_5b1c(
        CONCAT22(local_DX_314, pa_var5),
        CONCAT22(ctx.stack_seg_reg, &local_18),
        CONCAT22(ctx.stack_seg_reg, puVar6),
    );
    pass1_1028_c8ee(
        local_bx_6,
        local_es_6,
        local_bx_6.field_0x24,
        CONCAT22(ctx.stack_seg_reg, &local_10),
    );
    pass1_1008_3eb4(
        CONCAT22(ctx.stack_seg_reg, &local_10),
        CONCAT22(ctx.stack_seg_reg, &local_22),
        CONCAT22(ctx.stack_seg_reg, local_20),
        CONCAT22(ctx.stack_seg_reg, local_1e),
    );
    if (local_bx_6.field_0x24 == 1) {
        if (local_18 < local_22) {
            pass1_1030_5b3e(CONCAT22(local_DX_314, pa_var5), local_22, local_16);
            pass1_1030_5b1c(
                CONCAT22(local_DX_314, pa_var5),
                CONCAT22(ctx.stack_seg_reg, &local_18),
                CONCAT22(ctx.stack_seg_reg, &local_16),
            );
        }
        local_32 = (local_a + 4);
        pass1_1028_87f0(
            CONCAT22(ctx.stack_seg_reg, &local_158),
            0,
            0,
            0x6a,
            &local_10,
            ctx.stack_seg_reg,
            local_32,
            local_14,
        );
        pass1_1030_835a(ctx._g_bool_1050_5748, CONCAT22(ctx.stack_seg_reg, &local_158));
        local_158 = ctx.s_1_1050_389a;
        local_156 = &ctx.PTR_LOOP_1050_1008;
    }
    pass1_1028_ccd0(param_1, CONCAT22(ctx.stack_seg_reg, &local_10));
    return;
}

pub fn pass1_1020_e294(param_1: &mut Struct44) {
    let u_var1: u8;
    let extraout_var: u32;
    let mut local_eax_110: u32;
    let mut local_DX_44: u16;
    let mut local_DX_110: u16;
    let mut u_var2: u16;
    let mut local_es_6: u16;

    let mut local_15e: u32;
    let mut local_154: u32;
    let mut local_150: [u8; 12];
    let mut local_144: u16;
    let mut local_140: u16;
    let mut local_13e: u16;
    let mut local_1c: u32;
    let mut local_18: u32;
    let mut local_14: u32;
    let mut local_10: u32;
    let mut u_stack12: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;
    let mut temp_5fc08ad9c9: u32;
    let mut temp_5f52d5638b: u32;

    local_es_6 = (param_1 >> 0x10);
    u_var2 = param_1;
    if ((1 < (u_var2 + 0x24)) && ((u_var2 + 0x24) < 6)) {
        temp_5f52d5638b = (u_var2 + 0x28);
        local_6 = (temp_5f52d5638b + 0x20);
        u_var1 = pass1_1028_b58e(param_1);
        local_a = CONCAT31(extraout_var, u_var1);
        local_10 = (local_a + 0xc);
        u_stack12 = (local_a + 0x10);
        local_8 = local_DX_44;
        pass1_1028_c8ee(
            u_var2,
            local_es_6,
            (u_var2 + 0x24),
            CONCAT22(ctx.stack_seg_reg, &local_10),
        );
        local_eax_110._0_2_ = &local_10;
        local_DX_110 = local_DX_44;
        pass1_1028_c89c(
            param_1,
            CONCAT22(ctx.stack_seg_reg, local_eax_110),
            CONCAT22(ctx.stack_seg_reg, local_150),
        );
        local_14 = local_eax_110;
        local_15e._3_1_ = (local_14 >> 0x18);
        if ((local_15e._3_1_ == '\0') && (local_14 == 9)) {
            (u_var2 + 0x14) = 1;
        }
        local_18 = (local_a + 0x2e);
        local_1c = (local_18 + 4);
        pass1_1028_87f0(
            CONCAT22(ctx.stack_seg_reg, &local_140),
            0,
            (u_var2 + 0x14) + 1,
            0x79,
            &local_10,
            ctx.stack_seg_reg,
            local_1c,
            local_6,
        );
        pass1_1030_835a(ctx._g_bool_1050_5748, CONCAT22(ctx.stack_seg_reg, &local_140));
    }
    (u_var2 + 0x26) = 1;
    return;
}

pub fn pass1_1020_e39c(param_1: u32, param_2: u16) {
    let mut u_var1: u32;
    let u_var2: u8;
    let mut i_var3: i32;
    let mut u_var4: u16;
    let extraout_var: u32;
    let mut in_dx: i32;
    let mut unaff_ss: u16;
    let mut local_14e: u16;
    let mut local_13e: u16;
    let mut local_13c: u16;
    let mut local_13a: u16;
    let mut local_18: u32;
    let mut local_14: u32;
    let mut local_10: u32;
    let mut local_c: u32;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    u_var2 = pass1_1028_b58e(param_1);
    i_var3 = CONCAT31(extraout_var, u_var2);
    _local_6 = CONCAT31(extraout_var, u_var2) & 0xffff | in_dx << 0x10;
    local_c = (i_var3 + 0xc);
    local_8 = (i_var3 + 0x10);
    if (local_8 < 1) {
        u_var4 = 5;
    } else {
        u_var4 = 6;
    }
    (i_var3 + 0x14) = u_var4;
    u_var1 = (param_1 + 0x28);
    local_10 = (u_var1 + 0x20);
    local_14 = (i_var3 + 0x2e);
    local_18 = (local_14 + 4);
    pass1_1028_87f0(
        CONCAT22(unaff_ss, &local_13c),
        0,
        1,
        param_2,
        &local_c,
        unaff_ss,
        local_18,
        local_10,
    );
    pass1_1030_835a(ctx._g_bool_1050_5748, CONCAT22(unaff_ss, &local_13c));
    return;
}

pub fn pass1_1020_e44c(param_1: *mut Struct726) {
    let pi_var1: *mut i32;
    let local_bx_3: *mut Struct726;
    let local_es_3: *mut Struct726;

    local_es_3 = (param_1 >> 0x10);
    local_bx_3 = param_1;
    if (local_bx_3.field_0x12 == 2) {
        pi_var1 = &local_bx_3.field_0x14;
        unsafe { *pi_var1 = *pi_var1 + -1 };
        if ((local_bx_3.field_0x26 == 0) && (local_bx_3.field_0xc == 0x78)) {
            pass1_1020_e294(param_1);
        }
        if (local_bx_3.field_0x14 == 0) {
            pass1_1020_e08e(param_1);
            return;
        }
        if (local_bx_3.field_0x24 == 6) {
            local_bx_3.field_0xe = 0x49;
        }
    }
    return;
}

pub fn pass1_1020_e49a(param_1: &mut Struct44) {
    let mut i_var1: i32;
    let u_var2: u8;
    let extraout_AH: u8;
    let mut in_dx: u16;
    let mut local_a: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    u_var2 = pass1_1028_b58e(param_1);
    i_var1 = (CONCAT11(extraout_AH, u_var2) + 0x14);
    local_a = 0;
    if (i_var1 == 6) {
        local_a = 9;
    } else {
        if (i_var1 == 7) {
            local_a = 6;
        } else {
            if (i_var1 == 8) {
                local_a = 7;
            } else {
                if (i_var1 == 9) {
                    local_a = 8;
                }
            }
        }
    }
    pass1_1020_e39c(param_1, local_a);
    return;
}

pub fn pass1_1020_e4fa(param_1: u32, param_2: u16) {
    let u_var1: u8;
    let extraout_AH: u8;
    let mut in_dx: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    match (param_2) {
        2 | 5 | 6 | 7 => local_4 = 4,
        3 | 8 => local_4 = 5,
        // default:
        _ => {
            u_var1 = pass1_1028_b58e(param_1);
            local_4 = (CONCAT11(extraout_AH, u_var1) + 0x14) + 2;
        }
    }
    return local_4;
}

pub fn pass1_1020_e558(param_1: &mut Struct44) {
    let u_var1: u8;
    let pu_var2: *mut u32;
    let paVar3: *mut Struct493;
    let extraout_var: u32;
    let mut in_dx: u16;
    let mut u_var4: u16;
    let mut u_var5: u16;
    let mut u_var6: u16;
    let mut unaff_ss: u16;
    let mut local_30: u32;
    let mut local_24: [u8; 12];
    let mut local_18: u32;
    let mut local_14: u32;
    let mut local_10: u32;
    let mut u_stack12: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    u_var6 = (param_1 >> 0x10);
    u_var5 = param_1;
    if ((u_var5 + 0xc) == 0x79) {
        (u_var5 + 0x14) = (u_var5 + 0x24);
        (u_var5 + 0x24) = 0;
    }
    if ((u_var5 + 0x24) != 6) {
        u_var1 = pass1_1028_b58e(param_1);
        local_6 = CONCAT31(extraout_var, u_var1);
        local_a = (local_6 + 0x14);
        local_8 = local_a;
        local_4 = in_dx;
        pass1_1020_e4fa(param_1, local_a);
        local_10 = (local_6 + 0xc);
        u_stack12 = (local_6 + 0x10);
        local_18 = CONCAT22(local_18._2_2_, &local_10);
        pass1_1028_c8ee(
            u_var5,
            u_var6,
            (u_var5 + 0x24),
            CONCAT22(unaff_ss, &local_10),
        );
        pu_var2 = &local_10;
        pass1_1028_c89c(
            param_1,
            CONCAT22(unaff_ss, pu_var2),
            CONCAT22(unaff_ss, local_24),
        );
        unsafe { local_18 = *pu_var2 };
        u_var4 = (pu_var2 + 2);
        local_30._3_1_ = (local_18 >> 0x18);
        local_14._0_2_ = local_18;
        local_14 = local_18;
        if (local_30._3_1_ != '\0') {
            paVar3 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, local_14, u_var4);
            local_14._0_2_ = &paVar3.field_0x14;
        }
        local_8 = local_14;
        pass1_1020_e4fa(param_1, local_14);
        (u_var5 + 0x14) = local_a + local_14;
        return;
    }
    (u_var5 + 0x14) = 1;
    return;
}

pub fn pass1_1020_e652(param_1: u32, param_2: u16, param_3: u16, param_4: u32) -> i32 {
    let mut i_var1: i32;
    let mut u_var2: u16;
    let mut unaff_ss: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u32;
    let mut uStack4: u16;

    local_8 = param_2;
    uStack4 = (param_2 + 4);
    u_var2 = (param_1 >> 0x10);
    pass1_1028_c8ee(
        param_1,
        u_var2,
        (param_1 + 0x24),
        CONCAT22(unaff_ss, &local_8),
    );
    i_var1 = pass1_1028_c7b6(param_1, &local_8, unaff_ss, param_4);
    if (i_var1 != 0) {
        i_var1 = 1;
    }
    return i_var1;
}

pub fn ret_one_1020_c3ae() -> u32 {
    return 1;
}

pub fn switch_statement_1020_c3b4(param_1: u16) -> u32 {
    let mut local_6: u32;

    local_6._0_2_ = 1;
    match (param_1) {
        1 | 2 | 3 | 5 | 8 | 9 | 10 | 0xb | 0xc => local_6._0_2_ = 3,
        4 => local_6._0_2_ = 6,
        6 | 0xf | 0x10 | 0x11 | 0x12 | 0x13 => local_6._0_2_ = 10,
        7 => local_6._0_2_ = 2,
        0xd | 0xe => {
            local_6._0_2_ = 1;
        }
    }
    return local_6;
}

pub fn pass1_1020_c42e(param_1: u16) {
    let mut u_var1: u16;

    if (param_1 == 0xf) {
        u_var1 = 1;
    } else {
        u_var1 = 3;
    }
    return u_var1;
}

pub unsafe fn pass1_1020_c444(ctx: &mut AppContext, param_1: &mut Struct706, param_2: u32, param_3: u32) {
    pass1_1030_1cd8(ctx, param_1, param_2, param_3);
    param_1.field_0x18 = 0;
    param_1.field_0x1c = 0;
    param_1.field_0x0 = 0xc834;
    param_1.field_0x2 = 0x1020;
}

pub fn pass1_1020_c47a(param_1: &mut Struct44) {
    Struct44 * *local_es_4;

    local_es_4 = (param_1 >> 0x10);
    param_1.ptr_a_lo = 0xc834;
    (param_1 + 2) = 0x1020;
    error_check_1000_17ce((param_1 + 0x18));
    pass1_1030_1d28(param_1);
    return;
}

pub fn pass1_1020_c4a8(param_1: *mut Struct709, param_2: u32, param_3: u32, param_4: u16) {
    let mut u_var1: u32;
    let local_bx_4: *mut Struct709;
    let local_bx_39: *mut Struct710;
    let local_es_4: *mut Struct709;
    let mut u_var2: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    local_es_4 = (param_1 >> 0x10);
    local_bx_4 = param_1;
    if (local_bx_4.field_0x1c != 0) {
        pass1_1020_c6a4((param_1 & 0xffff | ZEXT24(local_es_4) << 0x10));
    }
    u_var1 = local_bx_4.field_0x18;
    u_var2 = (u_var1 >> 0x10);
    local_bx_39 = (u_var1 + param_4 * 6);
    param_3 = (local_bx_39).field_0x0;
    param_2 = local_bx_39.field_0x4;
    return;
}

pub fn pass1_1020_c4f4(param_1: u32, param_2: u16, param_3: u16, param_2_00: u32) {
    let paVar1: *mut Struct493;
    let local_AX_43: *mut Struct712;
    let mut u_var2: i32;
    let mut u_var3: i32;
    let lVar4: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    lVar4 = pass1_1020_c6de(param_1, param_2_00);
    u_var2 = (lVar4 >> 0x10);
    u_var3 = u_var2 | lVar4;
    if (lVar4 != 0) {
        paVar1 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, param_2_00, (param_2_00 >> 0x10));
        local_AX_43._0_1_ = pass1_1030_6fa0(CONCAT22(u_var3, paVar1));
        local_AX_43 = CONCAT11(local_AX_43._1_1_, local_AX_43);
        (lVar4 + 4) = (local_AX_43 * 2 + 0x4ea4);
    }
    return;
}

pub fn pass1_1020_c538(param_1: Vec<u8>) {
    let mut u_var1: u16;

    u_var1 = (param_1 >> 0x10);
    return CONCAT22((param_1 + 0x12), (param_1 + 0x10));
}

pub fn pass1_1020_c54a(param_1: *mut Struct709) {
    let local_es_5: *mut Struct709;
    let mut local_6: u32;

    local_es_5 = (param_1 >> 0x10);
    if ((param_1 + 0x1c) != 0) {
        pass1_1020_c6a4((param_1 & 0xffff | ZEXT24(local_es_5) << 0x10));
    }
    return;
}

pub fn pass1_1020_c5b4(param_1: *mut u32, param_2: u32) {
    let plVar1: *mut long;
    let ppc_var2: fn();
    let u_var3: u8;
    let paVar4: *mut Struct493;
    let mut u_var5: i32;
    let mut u_var6: i32;
    let extraout_var: u32;
    let mut in_dx: u16;


    let mut u_var7: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;

    paVar4 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, param_2, (param_2 >> 0x10));
    u_var3 = pass1_1030_6fa0(CONCAT22(in_dx, paVar4));
    u_var5 = CONCAT31(extraout_var, u_var3);
    u_var6 = u_var5;
    pass1_1020_c6de(param_1, 0);
    _local_c = CONCAT22(ctx.dx_reg, u_var6);
    u_var7 = (param_1 >> 0x10);
    if ((ctx.dx_reg | u_var6) == 0) {
        unsafe {
            ppc_var2 = (*param_1 + 0x20);
            ppc_var2()
        };
        pass1_1020_c6de(param_1, 0);
        _local_c = CONCAT22(ctx.dx_reg, u_var6);
        if ((ctx.dx_reg | u_var6) == 0) {
            return;
        }
    }
    (param_1 + 0x1c) = 1;
    plVar1 = (param_1 + 8);
    unsafe {
        *plVar1 = *plVar1 + 1;
        *_local_c = param_2;
        (_local_c + 4) = (u_var5 * 2 + 0x4ea4);
    }
    return;
}

pub fn pass1_1020_c644(in_struct_1: *mut Struct713, param_2: u16, param_3: u32) {
    let plVar1: *mut long;
    let mut u_var2: u16;
    let local_AX_39: *mut Struct714;
    let local_struct_1: *mut Struct713;
    let local_struct_1_hi: *mut Struct713;
    let mut local_6: u16;
    let mut local_4: u16;
    let fn_ptr_1: fn();

    local_struct_1_hi = (in_struct_1 >> 0x10);
    local_struct_1 = in_struct_1;
    if (&local_struct_1.field_0x18 == 0) {
        fn_ptr_1 = (in_struct_1 + 0x20);
        (**fn_ptr_1)();
    }
    local_AX_39 = (&local_struct_1.field_0x8 * 6 + local_struct_1.field_0x18);
    u_var2 = local_struct_1.field_0x1a;
    _local_6 = CONCAT22(u_var2, local_AX_39);
    plVar1 = &local_struct_1.field_0x8;
    unsafe {
        *plVar1 = *plVar1 + 1;
        *_local_6 = param_3
    };
    local_AX_39.field_0x4 = param_2;
    return;
}

pub fn pass1_1020_c694(param_1: *mut Struct709) {
    pass1_1020_c6a4(param_1);
    return;
}

pub fn pass1_1020_c6a4(param_1: *mut Struct709) {
    let local_struct_1: *mut Struct709;
    let local_es_3: *mut Struct709;

    local_es_3 = (param_1 >> 0x10);
    local_struct_1._0_2_ = param_1;
    if (((local_struct_1 + 0x18) != 0) && ((local_struct_1 + 8) != 0)) {
        pass1_1000_4aea(
            (local_struct_1 + 0x18),
            (local_struct_1 + 0x10),
            6,
            0xc7fa,
            0x1020,
        );
        (local_struct_1 + 0x1c) = 0;
    }
    return;
}

pub fn pass1_1020_c6de(in_struct_1: *mut Struct715, param_2: u32) {
    let pu_var1: *mut u32;
    let mut u_var2: u32;
    let local_struct_1: *mut Struct715;
    let local_struct_1_hi: *mut Struct715;
    let mut local_6: u32;

    local_struct_1_hi = (in_struct_1 >> 0x10);
    local_struct_1 = in_struct_1;
    if (local_struct_1.field_0x1c != 0) {
        pass1_1020_c6a4((in_struct_1 & 0xffff | ZEXT24(local_struct_1_hi) << 0x10));
    }
    local_6 = 0;
    while (true) {
        pu_var1 = &local_struct_1.field_0x10;
        let pu_var1_val = unsafe { *pu_var1 };
        if (pu_var1_val < local_6 || pu_var1_val == local_6) {
            return;
        }
        u_var2 = local_struct_1.field_0x18;
        if ((u_var2 + local_6 * 6) == param_2) {
            break;
        }
        local_6 = local_6 + 1;
    }
    return;
}

pub fn pass1_1020_c73a(in_struct_1: *mut Struct716) {
    let pu_var1: *mut u32;
    let paVar2: *mut Struct94;
    let mut u_var3: i32;
    let ctx.dx_reg: *mut u16;
    let local_struct_1: *mut Struct716;
    let local_struct_1_hi: *mut Struct716;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;
    let temp_5ffbaa3a02: *mut Struct717;

    local_struct_1_hi = (in_struct_1 >> 0x10);
    local_struct_1 = in_struct_1;
    if (&local_struct_1.field_0x10 == 0) {
        paVar2 = local_struct_1.field_0xc;
        ctx.g_u16_ptr_1050_5f2e = local_struct_1.field_0xe;
    } else {
        temp_5ffbaa3a02 = local_struct_1.field_0x10;
        pu_var1 = &local_struct_1.field_0x14;
        let pu_var1_val = unsafe { *pu_var1 };
        paVar2 = (temp_5ffbaa3a02 + pu_var1_val);
        ctx.g_u16_ptr_1050_5f2e = (local_struct_1.field_0x12
            + local_struct_1.field_0x16
            + CARRY2(temp_5ffbaa3a02, pu_var1_val));
    }
    _local_6 = CONCAT22(ctx.g_u16_ptr_1050_5f2e, paVar2);
    if (&local_struct_1.field_0x18 == 0) {
        if (ctx.__g_Struct94_ptr_1 == 0) {
            _g_Struct94_ptr_1 = paVar2;
            struct_fn_1000_160a();
        } else {
        }
        u_var3 = paVar2 * 6;
        alloc_mem_1000_1708(u_var3, 0, 1, _g_Struct94_ptr_1, ctx.g_u16_ptr_1050_5f2e);
    } else {
        u_var3 = paVar2 * 6;
        alloc_mem_1000_0ed4(
            1,
            u_var3,
            (ctx.g_u16_ptr_1050_5f2e * 3 + CARRY2(paVar2, paVar2) + CARRY2(paVar2 * 2, paVar2)) * 2
                + CARRY2(paVar2 * 3, paVar2 * 3),
            &local_struct_1.field_0x18,
        );
        ctx.g_u16_ptr_1050_5f2e = ctx.dx_reg;
    }
    _local_a = CONCAT22(ctx.g_u16_ptr_1050_5f2e, u_var3);
    if ((ctx.g_u16_ptr_1050_5f2e | u_var3) != 0) {
        &local_struct_1.field_0x10 = _local_6;
        &local_struct_1.field_0x18 = _local_a;
    }
    local_struct_1.field_0x1c = 1;
    return;
}

pub fn pass1_1020_c7fa(param_1: Vec<u8>, param_2: u32) -> i32 {
    return (param_1 + 4) - (param_2 + 4);
}

pub fn pass1_1020_c80e(param_1: &mut Struct44, param_2: u8) -> &mut Struct44 {
    pass1_1020_c47a(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1020_c835() {
    let pu8_var1: Vec<u8>;
    let pc_var2: String;
    let mut u8_var3: u8;
    let mut cVar5: u8;
    let mut u_var6: u32;
    let mut in_CH: u8;
    let mut in_DL: u8;
    let mut in_DH: u8;
    let mut in_bx: i32;
    let pu_var7: *mut u16;
    let mut i_var8: i32;
    let unaff_bp: *mut u16;
    let mut unaff_si: i32;
    let mut unaff_ss: u16;
    let mut u_var9: u16;
    let mut in_CF: u8;
    let mut bVar10: bool;
    let mut in_stack_000020c3: u8;
    let mut u8_var4: u8;

    pu_var7 = &stack0xfffe;
    cVar5 = '\t';
    while {
        unaff_bp = unaff_bp + -1;
        pu_var7 = pu_var7 + -1;
        unsafe { *pu_var7 = *unaff_bp };
        cVar5 = cVar5 + -1;
        '\0' < cVar5
    } {}
    u_var9 = (*(in_bx + unaff_si) >> 0x10);
    bVar10 = CARRY1(in_stack_000020c3, in_CH) || CARRY1(in_stack_000020c3 + in_CH, in_CF);
    pu8_var1 = (s_514a_bmp_1050_20c5 + unaff_si);
    unsafe {
        u8_var3 = *pu8_var1;
        u8_var4 = *pu8_var1;
        *pu8_var1 = u8_var4 + in_DH + bVar10;
        pc_var2 = (in_bx + unaff_si);
        *pc_var2 =
            *pc_var2 + (in_bx >> 8) + (CARRY1(u8_var3, in_DH) || CARRY1(u8_var4 + in_DH, bVar10));
        u_var6 = (in_bx + unaff_si);
        u_var9 = (u_var6 >> 0x10);
        i_var8 = u_var6;
        (i_var8 + -2) = u_var9;
        pu8_var1 = (in_bx + unaff_si);
        *pu8_var1 = *pu8_var1 ^ in_DL;
        (i_var8 + -4) = unaff_ss;
        pu8_var1 = (in_bx + unaff_si);
        *pu8_var1 = *pu8_var1 ^ in_DL;
    }
    // WARNING: Bad instruction - Truncating control flow here
    halt_baddata();
}

pub fn pass1_1020_c860(param_1: Vec<u8>) {
    let mut local_es_3: u16;

    local_es_3 = (param_1 >> 0x10);
    return CONCAT22((param_1 + 6), (param_1 + 4));
}

pub fn pass1_1020_c872(param_1: u32, param_2: u32, param_3: u32) {
    let pu_var1: *mut u32;
    let pu_var2: *mut u32;
    let pi_var3: *mut i32;
    let local_struct_1: *mut Struct501;
    let mut u_var4: i32;

    let local_bx_126: *mut Struct502;
    let mut i32_var6: i32;
    let mut u_var7: u16;
    let mut u_var8: u16;
    let mut u_var9: u16;
    let mut bVar10: bool;
    let mut local_2a: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u32;
    let mut local_a: u32;
    let mut local_6: u32;
    let pu_var5: Vec<u8>;

    pu_var5 = _PTR_LOOP_1050_4fb8;
    alloc_mem_1000_07fc(_PTR_LOOP_1050_4fb8);
    u_var4 = pu_var5;
    local_6 = pu_var5 & 0xffff | ctx.dx_reg << 0x10;
    if ((ctx.dx_reg | u_var4) == 0) {
        local_6 = 0;
    } else {
        local_6 = ctx.s_1_1050_389a;
        (u_var4 + 2) = &ctx.PTR_LOOP_1050_1008;
        (u_var4 + 4) = 0;
        (u_var4 + 8) = 0;
        local_6 = s__s__s__1050_5bc0;
        (u_var4 + 2) = &ctx.PTR_LOOP_1050_1008;
        (u_var4 + 0xe) = 0;
        (u_var4 + 0xc) = 0;
        local_6 = 0xc9e6;
        (u_var4 + 2) = 0x1020;
    }
    if (local_6 == 0) {
        return;
    }
    u_var7 = (local_6 >> 0x10);
    local_bx_126 = local_6;
    local_bx_126.field_0x8 = param_3;
    local_bx_126.field_0xc = param_2;
    u_var8 = (param_1 >> 0x10);
    i32_var6 = param_1;
    local_e = (i32_var6 + 4);
    u_var9 = (i32_var6 + 6);
    if ((i32_var6 + 8) == 0) {
        // LAB_1020_c92d:
        local_bx_126.field_0x4 = (i32_var6 + 4);
    } else {
        pu_var1 = (local_e + 0xe);
        unsafe {
            bVar10 = *pu_var1 < param_2._2_2_;
            if ((bVar10 || *pu_var1 == param_2._2_2_)
                && (bVar10
                    || (
                        pu_var1 = (local_e + 0xc),
                        *pu_var1 < param_2 || *pu_var1 == param_2,
                    )))
            {}
        }
        // goto LAB_1020_c92d;
        bVar10 = false;
        while (true) {
            if (local_e == 0) {
                break;
            }
            u_var9 = (local_e >> 0x10);
            pu_var2 = (local_e + 0xc);
            let pu_var2_val = unsafe { *pu_var2 };
            if (pu_var2_val < param_2 || pu_var2_val == param_2) {
                bVar10 = true;
                local_bx_126.field_0x4 = local_e;
                (local_a + 4) = local_6;
                break;
            }
            local_a = local_e;
            local_e = (local_e + 4);
        }
        param_1 = local_a;
        if (bVar10) {}
        // goto LAB_1020_c9ab;
    }
    u_var9 = (param_1 >> 0x10);
    (param_1 + 4) = local_bx_126;
    (param_1 + 6) = u_var7;
    // LAB_1020_c9ab:
    unsafe {
        pi_var3 = (i32_var6 + 8);
        *pi_var3 = *pi_var3 + 1;
    }
    return;
}

pub fn call_free_mem_1020_c9ba(param_1: *mut u16, param_2: u8) {
    unsafe {
        *param_1 = ctx.s_1_1050_389a;
    }
    (param_1 + 2) = &ctx.PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0) {
        free_mem_1000_093a(param_1);
    }
    return param_1;
}

pub fn pass1_1020_c9ea(param_1: *mut u16) {
    pass3_funcs::pass1_1028_0954(param_1);
    unsafe {
        *param_1 = 0xcc7c;
        (param_1 + 2) = 0x1020;
    }
    return param_1;
}

pub fn pass1_1020_ca0c(param_1: *mut Struct743, param_2: u16, param_3: u16, param_3_00: Vec<u8>) {
    pass3_funcs::pass1_1028_0982(param_1, CONCAT22(param_3, param_2), param_3_00);
    CONCAT22(param_2, param_1) = 0xcc7c;
    param_1.field_0x2 = 0x1020;
    return CONCAT22(param_2, param_1);
}

pub fn pass1_1020_ca36(param_1: i32, param_2: u16, param_3: u16) {
    let mut u_var1: u32;
    let local_struct_2: *mut Struct2111;
    let mut in_stack_0000ffee: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    pass3_funcs::pass1_1028_09b8(param_1, param_2, param_3);
    u_var1 = pass1_1028_b4f2(CONCAT22(param_2, param_1));
    if ((u_var1 + 0x200) != 0x8000002) {
        local_struct_2 =
            process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(in_stack_0000ffee, 8));
        pass1_1010_988c(local_struct_2, (param_1 + 0xc));
    }
    return;
}

pub fn pass1_1020_ca82(param_1: Vec<u8>) {
    let mut u_var1: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    pass1_1028_be9e(param_1);
    u_var1 = pass1_1028_b4f2(param_1);
    if ((u_var1 + 0x200) != 0x8000002) {
        if ((param_1 + 0x12) == 5) {
            pass1_1020_cac2(param_1);
        }
    }
    return;
}

pub fn pass1_1020_cac2(param_1: &mut Struct44) {
    let pu_var1: *mut u16;
    let mut i_var2: i32;
    let ppc_var3: fn();
    let u_var4: u8;
    let local_AX_96: *mut Struct718;
    let mut u_var5: u16;
    let extraout_var: u32;

    let mut u_var6: i32;
    let mut u_var7: u16;
    let mut unaff_ss: u16;
    let pp_var8: *mut Struct2111;
    let mut in_stack_0000ffca: u16;
    let mut local_34: u16;
    let mut local_30: u16;
    let mut local_2e: u16;
    let mut local_24: u16;
    let mut local_20: u16;
    let mut local_1e: u16;
    let mut local_18: u32;
    let mut local_14: u32;
    let mut local_10: u32;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    pp_var8 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(in_stack_0000ffca, 2));
    local_4 = (pp_var8 >> 0x10);
    local_6 = pp_var8;
    local_8 = u16_1050_13ae;
    if (u16_1050_13ae == 1) {
        local_8 = 2;
    }
    _local_c = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(in_stack_0000ffca, 8));
    u_var7 = (_local_c >> 0x10);
    local_10 = (_local_c + 10);
    local_14 = (_local_c + 0xe);
    pass1_1008_5784(CONCAT22(unaff_ss, &stack0xffe4), local_10);
    loop {
        while {
            loop {
                while {
                    local_AX_96 = &stack0xffe4;
                    pass1_1008_5b12(CONCAT22(unaff_ss, local_AX_96));
                    u_var6 = ctx.dx_reg | local_AX_96;
                    if (u_var6 == 0) {
                        return;
                    }
                    i_var2 = local_AX_96.field_0x4;
                    (i_var2 < 0x12) || (SBORROW2(i_var2, 0x12))
                } {}
                if (i_var2 != 0x13 && 0 < i_var2 + -0x12) {
                    break;
                }
                local_34 = 0;
                if (local_8 == 3) {
                    local_34 = local_AX_96.field_0x6 / 2;
                } else {
                    if (local_8 == 4) {
                        i_var2 = local_AX_96.field_0x6 * 3;
                        local_34 = (i_var2 + (i_var2 >> 0xf & 3)) >> 2;
                    }
                }
                pu_var1 = &local_AX_96.field_0x6;
                unsafe { *pu_var1 = *pu_var1 - local_34 };
                u_var7 = (local_10 >> 0x10);
                (local_10 + 10) = 0;
                ppc_var3 = (local_10 + 0xc);
                (**ppc_var3)(
                    &ctx.PTR_LOOP_1050_1008,
                    local_10,
                    u_var7,
                    local_AX_96,
                    ctx.dx_reg,
                );
                (local_10 + 10) = 1;
                local_18 = 0;
                ppc_var3 = (local_14 + 4);
                (**ppc_var3)(
                    &ctx.PTR_LOOP_1050_1008,
                    local_14,
                    (local_14 >> 0x10),
                    local_AX_96,
                    ctx.dx_reg,
                );
            }
            i_var2 != 0x81
        } {}
        local_24 = 0;
        if (local_8 == 2) {
            u_var5 = local_AX_96.field_0x6;
            // LAB_1020_cba7:
            u_var6 = u_var5 >> 0xf & 3;
            local_24 = (u_var5 + u_var6) >> 2;
        } else {
            if (local_8 == 3) {
                u_var5 = local_AX_96.field_0x6;
                u_var6 = u_var5 >> 0xf;
                local_24 = u_var5 / 2;
            } else {
                if (local_8 == 4) {
                    u_var5 = local_AX_96.field_0x6 * 3;
                    // goto LAB_1020_cba7;
                }
            }
        }
        u_var4 = pass1_1028_b58e(param_1);
        pass1_1030_7ddc(
            CONCAT31(extraout_var, u_var4) & 0xffff | u_var6 << 0x10,
            (local_AX_96.field_0x6 - local_24),
            0x1e,
        );
        ppc_var3 = (local_10 + 0xc);
        (**ppc_var3)(
            0x1030,
            local_10,
            (local_10 >> 0x10),
            local_AX_96,
            ctx.dx_reg,
        );
        local_18 = 0;
    }
}

pub fn pass1_1020_cc56(param_1: &mut Struct44, param_2: u8) -> &mut Struct44 {
    pass1_1028_b418(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1020_cce4(param_1: *mut u16) {
    pass1_1028_b354(param_1);
    unsafe {
        *param_1 = 0xcd7e;
        (param_1 + 2) = 0x1020
    };
    return param_1;
}

pub fn pass1_1020_cd06(param_1: i32, param_2: u16, param_3: u16, param_3_00: Vec<u8>) {
    pass1_1028_b39e(CONCAT22(param_2, param_1), param_3, param_3_00);
    CONCAT22(param_2, param_1) = 0xcd7e;
    (param_1 + 2) = 0x1020;
    return CONCAT22(param_2, param_1);
}

pub fn pass1_1020_cd30(param_1: *mut Struct719) {
    let local_bx_3: *mut Struct719;
    let local_es_3: *mut Struct719;

    local_es_3 = (param_1 >> 0x10);
    local_bx_3 = param_1;
    if (((local_bx_3.field_0x12 == 6) || (local_bx_3.field_0x12 == 5))
        && ((local_bx_3.field_0x1a & 2) != 0))
    {
        return 1;
    }
    return 0;
}

pub fn pass1_1020_cd58(param_1: &mut Struct44, param_2: u8) -> &mut Struct44 {
    pass1_1028_b418(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1020_cde6(param_1: Vec<u8>) {
    pass3_funcs::pass1_1028_0954(param_1);
    param_1 = 0xd004;
    (param_1 + 2) = 0x1020;
    return param_1;
}

pub fn pass1_1020_ce08(param_1: *mut Struct743, param_2: u16, param_3: u16, param_3_00: Vec<u8>) {
    pass3_funcs::pass1_1028_0982(param_1, CONCAT22(param_3, param_2), param_3_00);
    CONCAT22(param_2, param_1) = 0xd004;
    param_1.field_0x2 = 0x1020;
    return CONCAT22(param_2, param_1);
}

pub fn pass1_1020_ce32(param_1: i32, param_2: u16, param_3: u16) {
    let mut u_var1: u32;
    let ppVar2: *mut Struct2111;
    let mut u_var3: u16;
    let mut u_var4: u16;
    let u_var5: u8;
    let u_var6: u8;
    let mut u_var7: u16;
    let mut u_var8: u16;
    let mut u_var9: u16;
    let mut u_var10: u16;
    let mut in_stack_0000ffea: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    pass3_funcs::pass1_1028_09b8(param_1, param_2, param_3);
    u_var1 = pass1_1028_b4f2(CONCAT22(param_2, param_1));
    if ((u_var1 + 0x200) != 0x8000002) {
        ppVar2 =
            process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(in_stack_0000ffea, 8));
        pass1_1010_988c(ppVar2, (param_1 + 0xc));
        u_var9 = 0;
        u_var10 = 9;
        u_var7 = 1;
        u_var8 = 0;
        u_var4 = 0;
        u_var5 = 0;
        u_var6 = 0;
        u_var3 = 0;
        ppVar2 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, 0x37);
        post_win_msg_1008_a0e4(
            ppVar2,
            CONCAT22(u_var4, u_var3),
            CONCAT11(u_var6, u_var5),
            u_var7,
            CONCAT22(u_var9, u_var8),
            u_var10,
        );
    }
    return;
}

pub fn pass1_1020_ce9e(param_1: Vec<u8>) {
    let mut u_var1: u16;
    let mut u_var2: u32;
    let ppVar3: *mut Struct2111;
    let mut in_stack_0000ffea: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    pass1_1028_be9e(param_1);
    if ((param_1 + 0x12) == 5) {
        pass1_1020_cefc(param_1);
        u_var2 = pass1_1028_b4f2(param_1);
        u_var1 = (u_var2 >> 0x10);
        if ((u_var2 + 0x200) != 0x8000002) {
            ppVar3 = process_struct_1010_20ba(
                ctx._g_Struct372_1050_0ed0,
                CONCAT22(in_stack_0000ffea, 0x2b),
            );
            pass1_1010_043a(ppVar3, (u_var2 + 4), 5);
        }
    }
    return;
}

pub fn pass1_1020_cefc(param_1: Vec<u8>) {
    let pu_var1: Vec<u8>;
    let ppVar2: *mut Struct2111;
    let mut in_stack_0000ffec: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    pu_var1 = pass1_1028_b4f2(param_1);
    if ((pu_var1 + 0x200) == 0x8000002) {
        local_c = 0x32;
    } else {
        ppVar2 =
            process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(in_stack_0000ffec, 8));
        local_c = pass1_1010_96c2(ppVar2);
        if (0x32 < local_c) {
            local_c = 0x32;
        }
        pass1_1010_96a8(ppVar2, local_c);
    }
    pass1_1020_cf6c(param_1, (param_1 >> 0x10), local_c, pu_var1);
    return;
}

pub fn pass1_1020_cf6c(param_1: u16, param_2: u16, param_1_00: i32, param_2_00: Vec<u8>) {
    Struct721 * *ppaVar1;
    let pu_var2: *mut u32;
    Struct722 * *ppaVar3;
    let paVar4: *mut Struct721;
    let mut u_var5: i32;
    let mut u_var6: u32;
    let mut u_var7: u16;
    let local_AX_14: *mut Struct720;
    let mut u_var8: i32;
    let local_AX_42: *mut Struct721;
    let local_DX_60: *mut Struct722;
    let mut u_var9: u16;
    let mut local_a: u32;

    u_var9 = (param_2_00 >> 0x10);
    u_var6 = (param_2_00 + 0x1f6);
    local_AX_14 = u_var6;
    u_var7 = (u_var6 >> 0x10);
    u_var8 = param_1_00 / 5;
    local_AX_42 = (u_var8 * -4 + param_1_00);
    ppaVar1 = &local_AX_14.field_0x50;
    paVar4 = *ppaVar1;
    *ppaVar1 = local_AX_42 + *ppaVar1;
    pu_var2 = &local_AX_14.field_0x52;
    unsafe {
        *pu_var2 = *pu_var2 + (local_AX_42 >> 0xf) + CARRY2(paVar4, local_AX_42);
        local_DX_60 = (u_var8 >> 0xf);
        pu_var2 = &local_AX_14.field_0x78;
        u_var5 = *pu_var2;
        *pu_var2 = *pu_var2 + u_var8;
        ppaVar3 = &local_AX_14.field_0x7a;
        *ppaVar3 = local_DX_60 + (*ppaVar3 + CARRY2(u_var5, u_var8));
        pu_var2 = &local_AX_14.field_0xa0;
        u_var5 = *pu_var2;
        *pu_var2 = *pu_var2 + u_var8;
        ppaVar3 = &local_AX_14.field_0xa2;
        *ppaVar3 = local_DX_60 + (*ppaVar3 + CARRY2(u_var5, u_var8));
        pu_var2 = &local_AX_14.field_0xc8;
        u_var5 = *pu_var2;
        *pu_var2 = *pu_var2 + u_var8;
        ppaVar3 = &local_AX_14.field_0xca;
        *ppaVar3 = local_DX_60 + (*ppaVar3 + CARRY2(u_var5, u_var8));
        pu_var2 = &local_AX_14.field_0xf0;
        u_var5 = *pu_var2;
        *pu_var2 = *pu_var2 + u_var8;
        ppaVar3 = &local_AX_14.field_0xf2;
        *ppaVar3 = local_DX_60 + (*ppaVar3 + CARRY2(u_var5, u_var8));
        (param_2_00 + 0x1fe) = 1;
    }
    return;
}

pub fn pass1_1020_cfde(param_1: &mut Struct44, param_2: u8) -> &mut Struct44 {
    pass1_1028_b418(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1020_d06c(param_1: Vec<u8>) {
    pass1_1028_b354(param_1);
    param_1 = 0xd314;
    (param_1 + 2) = 0x1020;
    return param_1;
}

pub fn pass1_1020_d08e(param_1: i32, param_2: u16, param_3: u16, param_3_00: Vec<u8>) {
    pass1_1028_b39e(CONCAT22(param_2, param_1), param_3, param_3_00);
    CONCAT22(param_2, param_1) = 0xd314;
    (param_1 + 2) = 0x1020;
    return CONCAT22(param_2, param_1);
}

pub fn pass1_1020_d0b8(param_1: Vec<u8>) {
    let mut i_var1: i32;
    let mut u_var2: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    if ((param_1 + 0x12) != 6) {
        return;
    }
    u_var2 = pass1_1028_b4f2(param_1);
    if ((u_var2 + 0x200) != 0x8000002) {
        i_var1 = pass1_1028_cb04(param_1);
        if ((i_var1 == 0) || (i_var1 = pass1_1020_d194(param_1), i_var1 == 0)) {
            i_var1 = 6;
            // goto LAB_1020_d10b;
        }
        pass1_1028_c952(param_1);
    }
    i_var1 = 5;
    // LAB_1020_d10b:
    pass1_1028_bdac(param_1, i_var1);
    return;
}

pub fn pass1_1020_d118(param_1: Vec<u8>, param_2: u32, param_3: u32, param_4: u32) {
    let mut i_var1: i32;
    let mut u_var2: u16;
    let mut u_var3: u16;

    i_var1 = pass1_1028_c7b6(param_1, param_2, param_4);
    if (i_var1 == 5) {
        u_var2 = param_1;
        u_var3 = (param_1 >> 0x10);
        pass1_1028_c23e(u_var2, u_var3, param_2, param_3, param_4);
        if (i_var1 != 0) {
            pass1_1028_c3aa(u_var2, u_var3, param_2, param_3, param_4);
            if (i_var1 != 0) {
                u_var2 =
                    pass1_1028_c314(u_var2, u_var3, param_2, param_3, (param_3 >> 0x10), param_4);
                if (u_var2 != 0) {
                    return 1;
                }
            }
        }
    } else {
        PTR_LOOP_1050_50ca = 0x6a8;
    }
    return 0;
}

pub fn pass1_1020_d194(param_1: &mut Struct44) {
    let mut u_var1: u32;
    let ppc_var2: fn();
    let mut u_var3: u32;
    let u_var4: u8;
    let pa_var5: *mut Struct493;
    let puVar6: Vec<u8>;
    let mut i_var7: i32;
    let mut u_var8: u16;
    let mut u_var9: u16;
    let extraout_var: u32;
    let pu_var10: Vec<u8>;
    let mut u_var11: u32;
    let mut u_var12: i32;
    let mut u_var13: i32;




    let mut u_var14: u16;
    let mut unaff_ss: u16;
    let mut uVar15: u32;
    let ppVar16: *mut Struct2111;
    let uVar17: u8;
    let uVar18: u8;
    let mut uVar19: u16;
    let mut u_var20: u16;
    let mut u_var21: u16;
    let mut local_36: u32;
    let mut local_32: u32;
    let mut local_2e: u16;
    let mut local_2c: u16;
    let mut local_2a: u32;
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
    let mut local_c: u32;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: [u8; 2];

    pass1_1030_bcae(local_4, unaff_ss);
    uVar15 = pass1_1028_b4f2(param_1);
    u_var12 = (uVar15 >> 0x10);
    u_var1 = (uVar15 + 0x10);
    pa_var5 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, u_var1, (u_var1 >> 0x10));
    u_var13 = u_var12;
    u_var4 = pass1_1028_b58e(param_1);
    u_var3 = CONCAT31(extraout_var, u_var4) & 0xffff;
    puVar6 = local_4;
    pass1_1030_bd74(
        puVar6,
        unaff_ss,
        CONCAT22(u_var12, pa_var5),
        (CONCAT31(extraout_var, u_var4) & 0xffff | u_var13 << 0x10),
    );
    if (puVar6 < 0) {
        return;
    }
    if (0x1e < puVar6) {
        u_var20 = 0x87;
        ppVar16 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, 0x870009);
        i_var7 = ppVar16;
        pass1_1010_65d0(ppVar16, u_var20);
        if (i_var7 == 0) {
            u_var12 = ctx.dx_reg;
            pu_var10 = pass1_1008_c6fa(ctx._PTR_LOOP_1050_06e0, 0x15);
            u_var8 = SUB42(pu_var10, 0);
            u_var14 = SUB42(&PTR_LOOP_1050_1038, 0);
            pass1_1038_4e78(uVar15, pu_var10 & 0xffff | u_var12 << 0x10);
            _local_22 = CONCAT22(ctx.dx_reg, u_var8);
            ppc_var2 = (*_local_22 + 0x10);
            u_var9 = u_var8;
            uVar19 = u_var8;
            u_var21 = ctx.dx_reg;
            ppc_var2(&PTR_LOOP_1050_1038, u_var8, ctx.dx_reg);
            _local_26 = CONCAT22(ctx.dx_reg, u_var9);
            local_2a = 0;
            while (true) {
                if (_local_26 <= local_2a) {
                    if (_local_22 == 0x0) {
                        return;
                    }
                    ppc_var2 = *_local_22;
                    ppc_var2(
                        u_var14,
                        u_var8,
                        ctx.dx_reg,
                        1,
                        uVar19,
                        u_var21,
                        _local_22,
                        _local_22,
                    );
                    return;
                }
                uVar17 = u_var3;
                uVar18 = (u_var3 >> 8);
                u_var11 = _local_26;
                u_var12 = u_var13;
                pass1_1030_1d58(_local_22);
                puVar6 = local_4;
                u_var14 = 0x1030;
                pass1_1030_bd74(
                    puVar6,
                    unaff_ss,
                    (u_var11 & 0xffff | ctx.dx_reg << 0x10),
                    CONCAT22(u_var12, CONCAT11(uVar18, uVar17)),
                );
                if ((0 < puVar6) && (puVar6 < 0x1f)) {
                    break;
                }
                local_2a = local_2a + 1;
            }
            if (_local_22 == 0x0) {
                return;
            }
            ppc_var2 = *_local_22;
            ppc_var2(
                0x1030,
                u_var8,
                ctx.dx_reg,
                1,
                uVar19,
                u_var21,
                _local_22,
                _local_22,
                u_var11,
                ctx.dx_reg,
            );
            return;
        }
    }
    return;
}

pub fn pass1_1020_d2ee(param_1: &mut Struct44, param_2: u8) -> &mut Struct44 {
    pass1_1028_b418(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1020_d37c(param_1: *mut u16) {
    let mut u_var1: u16;

    pass1_1028_b354(param_1);
    u_var1 = (param_1 >> 0x10);
    unsafe {
        (param_1 + 0x20) = 0;
        *param_1 = 0xd53e;
        (param_1 + 2) = 0x1020;
    }
    return param_1;
}

pub fn pass1_1020_d3a4(param_1: *mut u16, param_2: u16, param_3: u16, param_4: u32) {
    let mut u_var1: u16;

    pass1_1028_b39e(param_1, param_3, param_4);
    u_var1 = (param_1 >> 0x10);
    unsafe {
        (param_1 + 0x20) = param_2;
        *param_1 = 0xd53e;
        (param_1 + 2) = 0x1020;
    }
    return param_1;
}

pub fn pass1_1020_a6ee(param_1: Vec<u8>, param_2: u16) {
    let ctx.ax_reg: *mut Struct698;
    let paVar1: *mut Struct493;
    let local_AX_84: *mut Struct699;
    let mut in_dx: i32;
    let mut unaff_ss: u16;
    let mut in_stack_0000fea0: u16;
    let mut local_142: u32;
    let mut local_13e: u16;
    let mut local_13c: u16;
    let mut local_1e: u32;
    let mut local_1a: u16;
    let mut local_18: u32;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u32;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    paVar1 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, 2, 0x400);
    _local_6 = CONCAT22(in_dx, paVar1);
    if (((in_dx | paVar1) == 0) || (&paVar1[0x11].field_0x2 == 0x8000002)) {
        _local_a = process_struct_1010_20ba(
            ctx._g_Struct372_1050_0ed0,
            CONCAT22(in_stack_0000fea0, 0x2f),
        );
        local_10 = (_local_a >> 0x10);
        local_e = (_local_a + 0x20);
        local_AX_84 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, local_e, (local_e >> 0x10));
        local_12 = local_AX_84;
        zero_list_1008_3e38(CONCAT22(unaff_ss, &local_18));
        local_1a = pass1_1008_c6ae(ctx._PTR_LOOP_1050_06e0, param_2, 0x28);
        if (local_1a != 0) {
            local_14 = 1;
        }
        pass1_1020_b2da(
            param_1,
            (local_1a != 0),
            CONCAT22(unaff_ss, &local_18),
            CONCAT22(local_10, local_12),
        );
        pass1_1028_87f0(
            CONCAT22(unaff_ss, &local_13e),
            0,
            0,
            param_2,
            &local_18,
            unaff_ss,
            (_PTR_LOOP_1050_4e70 + 4),
            (local_12 + 4),
        );
        pass1_1030_835a(ctx._g_bool_1050_5748, CONCAT22(unaff_ss, &local_13e));
        if (local_1a != 0) {
            pass1_1020_ad90(CONCAT22(unaff_ss, &local_18), (local_12 + 4));
        }
        (local_1e + 0x1c) = 0x8000001;
    }
    return;
}

pub fn pass1_1020_a80e(param_1: u16, param_2: u16, param_2_00: i32) {
    let mut u_var1: u32;
    let ctx.ax_reg: *mut Struct700;
    let paVar2: *mut Struct493;
    let paVar3: *mut Struct493;
    let mut in_dx: i32;
    let mut u_var4: u16;
    let ppVar5: *mut Struct2111;
    let mut in_stack_0000ffe4: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    paVar2 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, 2, 0x400);
    if (((in_dx | paVar2) == 0) || (&paVar2[0x11].field_0x2 == 0x8000002)) {
        ppVar5 = process_struct_1010_20ba(
            ctx._g_Struct372_1050_0ed0,
            CONCAT22(in_stack_0000ffe4, 0x2f),
        );
        u_var4 = (ppVar5 >> 0x10);
        u_var1 = (ppVar5 + 0x20);
        paVar2 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, u_var1, (u_var1 >> 0x10));
        if (param_2_00 == 10) {
            call_infinite_loop_1020_b872(param_1, param_2, paVar2, u_var4);
            return;
        }
        paVar3 = paVar2;
        pass1_1020_b0aa(param_1, param_2, param_2_00);
        if (paVar3 != 0x0) {
            pass1_1020_abc0(param_1, param_2, paVar3, paVar2, u_var4);
        }
    }
    return;
}

pub fn pass1_1020_a89e(param_1: Vec<u8>, param_2: *mut u32, param_3: u16) {
    let pi_var1: *mut i32;
    let ctx.ax_reg: *mut Struct701;
    let paVar2: *mut Struct493;
    let pu_var3: Vec<u8>;
    let mut u_var4: u16;
    let mut u_var5: u16;
    let mut u_var6: u16;

    let mut unaff_si: u16;
    let mut unaff_ss: u16;
    let mut local_5ee: u16;
    let mut local_5ec: u16;
    let mut local_4c2: u16;
    let mut local_4be: u16;
    let mut local_4bc: u16;
    let mut local_4ba: u16;
    let mut local_4b8: [u8; 8];
    let mut local_4b0: u32;
    let mut local_4ac: u16;
    let mut local_4aa: u16;
    let mut local_4a8: u16;
    let mut local_4a6: u16;
    let mut local_384: u16;
    let mut local_382: u16;
    let mut local_260: u16;
    let mut local_25e: u16;
    let mut local_13c: u16;
    let mut local_13a: u16;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: [u8; 6];
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    _local_6 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(unaff_si, 0x2f));
    u_var6 = (_local_6 >> 0x10);
    local_a = (_local_6 + 0x20);
    paVar2 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, local_a, (local_a >> 0x10));
    _local_e = CONCAT22(u_var6, paVar2);
    unsafe {
        local_14._0_4_ = *param_2;
        local_14._4_2_ = (param_2 + 1);
    }
    local_4c2 = local_14;
    pass1_1008_3e94(
        local_14,
        CONCAT22(unaff_ss, &local_18),
        CONCAT22(unaff_ss, &local_16),
    );
    pass1_1008_3e76(CONCAT22(unaff_ss, local_14), 0, local_18, local_16 + 2);
    pass1_1028_8888(
        CONCAT22(unaff_ss, &local_13c),
        0,
        0x7a,
        local_14,
        unaff_ss,
        0x8000002,
        0x4000002,
        local_a,
    );
    pass1_1030_835a(ctx._g_bool_1050_5748, CONCAT22(unaff_ss, &local_13c));
    pass1_1008_3e76(CONCAT22(unaff_ss, local_14), 0, local_18 - 2, local_16);
    pass1_1028_8888(
        CONCAT22(unaff_ss, &local_260),
        0,
        0x47,
        local_14,
        unaff_ss,
        0x8000002,
        0x4000002,
        local_a,
    );
    pass1_1030_835a(ctx._g_bool_1050_5748, CONCAT22(unaff_ss, &local_260));
    pass1_1008_3e76(CONCAT22(unaff_ss, local_14), 1, local_18 - 2, local_16);
    pass1_1028_8888(
        CONCAT22(unaff_ss, &local_384),
        0,
        0x6a,
        local_14,
        unaff_ss,
        0x8000002,
        0x4000002,
        local_a,
    );
    pass1_1030_835a(ctx._g_bool_1050_5748, CONCAT22(unaff_ss, &local_384));
    pass1_1020_ad90(CONCAT22(unaff_ss, local_14), local_a);
    pass1_1008_3e76(CONCAT22(unaff_ss, local_14), 1, local_18 - 2, local_16 + 1);
    pass1_1028_8888(
        CONCAT22(unaff_ss, &local_4a8),
        0,
        0x7f,
        local_14,
        unaff_ss,
        0x8000002,
        0x4000002,
        local_a,
    );
    pass1_1030_835a(ctx._g_bool_1050_5748, CONCAT22(unaff_ss, &local_4a8));
    pass1_1020_ad90(CONCAT22(unaff_ss, local_14), local_a);
    _local_4ac = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(unaff_si, 8));
    local_4b0 = (_local_4ac + 0x12);
    pass1_1008_5784(CONCAT22(unaff_ss, local_4b8), local_4b0);
    local_4be = 0;
    loop {
        while {
            pu_var3 = local_4b8;
            pass1_1008_5b12(CONCAT22(unaff_ss, pu_var3));
            _local_4bc = CONCAT22(ctx.dx_reg, pu_var3);
            if ((ctx.dx_reg | pu_var3) == 0) {
                pass1_1010_9674(_local_4ac);
                return;
            }
            ((pu_var3 + 4) != 0x3e) && ((pu_var3 + 4) != 0x41)
        } {}
        while (0 < (_local_4bc + 6)) {
            if (local_4be == 0) {
                u_var5 = local_16 - 2;
                // LAB_1020_ab4a:
                u_var4 = local_18 - 2;
                // LAB_1020_ab51:
                local_4be = local_4be + 1;
                pass1_1008_3e76(CONCAT22(unaff_ss, local_14), 0, u_var4, u_var5);
            } else {
                if (local_4be == 1) {
                    u_var5 = local_16 + 2;
                    // goto LAB_1020_ab4a;
                }
                if (local_4be == 2) {
                    u_var5 = local_16 + 2;
                    // LAB_1020_ab6e:
                    u_var4 = local_18 + 2;
                    // goto LAB_1020_ab51;
                }
                if (local_4be == 3) {
                    u_var5 = local_16 - 2;
                    // goto LAB_1020_ab6e;
                }
                local_4be = local_4be + 1;
                pass1_1020_b2da(param_1, 0, CONCAT22(unaff_ss, local_14), _local_e);
            }
            pass1_1028_8888(
                CONCAT22(unaff_ss, &local_5ee),
                0,
                (_local_4bc + 4),
                local_14,
                unaff_ss,
                0x8000002,
                0x4000002,
                local_a,
            );
            pass1_1030_835a(ctx._g_bool_1050_5748, CONCAT22(unaff_ss, &local_5ee));
            pi_var1 = (_local_4bc + 6);
            unsafe {
                *pi_var1 = *pi_var1 + -1;
            }
            local_5ee = ctx.s_1_1050_389a;
            local_5ec = &ctx.PTR_LOOP_1050_1008;
        }
    }
}

pub fn pass1_1020_abc0(param_1: Vec<u8>, param_2: u16, param_3: u32) {
    let mut u_var1: u16;
    let mut unaff_ss: u16;
    let mut local_12e: u16;
    let mut local_12c: u16;
    let mut local_a: u16;
    let mut local_8: u32;
    let mut local_4: u16;

    zero_list_1008_3e38(CONCAT22(unaff_ss, &local_8));
    local_a = pass1_1008_c6ae(ctx._PTR_LOOP_1050_06e0, param_2, 0x28);
    if (local_a != 0) {
        local_4 = 1;
    }
    pass1_1020_b2da(
        param_1,
        (local_a != 0),
        CONCAT22(unaff_ss, &local_8),
        param_3,
    );
    u_var1 = (param_3 >> 0x10);
    pass1_1028_87f0(
        CONCAT22(unaff_ss, &local_12e),
        0,
        0,
        param_2,
        &local_8,
        unaff_ss,
        (_PTR_LOOP_1050_4e70 + 4),
        (param_3 + 4),
    );
    pass1_1030_835a(ctx._g_bool_1050_5748, CONCAT22(unaff_ss, &local_12e));
    if (local_a != 0) {
        pass1_1020_ad90(CONCAT22(unaff_ss, &local_8), (param_3 + 4));
    }
    return;
}

pub fn pass1_1020_ac6e(param_1: Vec<u8>, param_2: u16, param_3: u16, param_4: u16) {
    let b_var1: bool;
    let pu_var2: Vec<u8>;
    let mut local_DX_124: u16;
    let mut local_DX_154: u16;

    let local_162: u8;
    let puStack350: *mut u16;
    let puStack342: *mut u16;
    let mut local_152: u16;
    let mut local_146: u16;
    let mut local_144: u16;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_16: u32;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: [u8; 6];
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    if (param_2 == 0) {
        _local_162 = SUB42(&PTR_LOOP_1050_4230, 0);
    } else {
        _local_162 = SUB42(s_dib_1050_4234 + 2, 0);
    }
    puStack342 = &local_4;
    puStack350 = &local_8;
    pass1_1008_3eb4(
        CONCAT22(0x1048, _local_162),
        CONCAT22(ctx.stack_seg_reg, puStack350),
        CONCAT22(ctx.stack_seg_reg, &local_6),
        CONCAT22(ctx.stack_seg_reg, puStack342),
    );
    if (param_4 == 0) {
        _local_6 = _local_6 & 0xffff | (local_4 + param_3) << 0x10;
    } else {
        if (param_4 == 1) {
            _local_6 = _local_6 & 0xffff0000 | (local_6 + param_3);
        } else {
            if (param_4 == 2) {
                _local_6 = _local_6 & 0xffff | (local_4 - param_3) << 0x10;
            }
        }
    }
    puStack342 = _local_6;
    pass1_1008_3e54(
        CONCAT22(ctx.stack_seg_reg, local_e),
        local_8,
        puStack342,
        (_local_6 >> 0x10),
    );
    _local_12 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(local_152, 0x2f));
    local_DX_124 = (_local_12 >> 0x10);
    local_16 = (_local_12 + 0x20);
    puStack342 = local_16;
    local_DX_154 = local_DX_124;
    local_1a = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, puStack342, (local_16 >> 0x10));
    local_18 = local_DX_154;
    b_var1 = pass1_1020_b1ae(param_1, CONCAT22(ctx.stack_seg_reg, local_e), *(local_1a + 4));
    if (b_var1 != 0) {
        pu_var2 = local_e;
        pass1_1020_b240(
            param_1,
            CONCAT22(ctx.stack_seg_reg, pu_var2),
            CONCAT22(local_18, local_1a),
        );
        if (pu_var2 != 0x0) {
            pass1_1028_87f0(
                CONCAT22(ctx.stack_seg_reg, &local_146),
                0,
                0,
                (-(param_2 == 0) & 0xfffb) + 0x7f,
                local_e,
                ctx.stack_seg_reg,
                (_PTR_LOOP_1050_4e70 + 4),
                local_16,
            );
            pass1_1030_835a(ctx._g_bool_1050_5748, CONCAT22(ctx.stack_seg_reg, &local_146));
        }
    }
    return;
}

pub fn pass1_1020_ad90(param_1: Vec<u8>, param_2: u32) {
    let pp_var1: fn();
    let paVar2: *mut Struct493;
    let pu_var3: Vec<u8>;
    let mut i_var4: i32;
    let mut i_var5: i32;


    let mut u_var6: i32;

    let mut unaff_ss: u16;
    let pu_var7: *mut u32;
    let mut u_var8: u16;
    let u_var9: u8;
    let u_var10: u8;
    let mut u_var11: u16;
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
        param_1,
        CONCAT22(unaff_ss, local_14),
        CONCAT22(unaff_ss, &local_6),
        CONCAT22(unaff_ss, &local_4),
    );
    pass1_1030_627e(_PTR_LOOP_1050_5740, param_1, param_2);
    local_26 = ctx.dx_reg;
    local_12 = ctx.dx_reg;
    paVar2 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, local_14, ctx.dx_reg);
    _local_18 = CONCAT22(local_26, paVar2);
    local_1c = &paVar2[1].field_0x10;
    local_20 = (local_1c + 4);
    local_28 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, param_2, (param_2 >> 0x10));
    pu_var7 = pass1_1030_5b5c(local_28, local_26);
    unsafe { local_2e._0_4_ = *pu_var7 };
    local_2e._4_2_ = (pu_var7 + 4);
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
    pass1_1008_6cec(
        CONCAT22(unaff_ss, local_3a),
        local_8,
        _local_10,
        local_8,
        _local_c,
    );
    pu_var3 = local_3a;
    u_var6 = ctx.dx_reg;
    pass1_1030_6522(_PTR_LOOP_1050_5740, CONCAT22(unaff_ss, pu_var3), param_2);
    _local_3e = CONCAT22(u_var6, pu_var3);
    if ((u_var6 | pu_var3) != 0) {
        local_42 = 0;
        local_44 = 0;
        local_46 = local_c;
        while (local_46 <= local_10) {
            local_4e = local_a;
            while (local_4e <= local_e) {
                pp_var1 = (*_local_3e + 4);
                u_var8 = local_44;
                local_44 = local_44 + 1;
                (**pp_var1)(0x1030, _local_3e, (_local_3e >> 0x10));
                local_42 = CONCAT22(ctx.dx_reg, u_var8);
                local_42._3_1_ = (ctx.dx_reg >> 8);
                if (local_42._3_1_ == '\0') {
                    local_5a = u_var8;
                    if (u_var8 == 7) {
                        pass1_1008_3e76(param_1, local_8, local_46, local_4e);
                        u_var9 = local_20;
                        u_var10 = (local_20 >> 8);
                        u_var11 = (local_20 >> 0x10);
                        u_var8 = 6;
                    } else {
                        if (u_var8 == 8) {
                            pass1_1008_3e76(param_1, local_8, local_46, local_4e);
                            u_var9 = local_20;
                            u_var10 = (local_20 >> 8);
                            u_var11 = (local_20 >> 0x10);
                            u_var8 = 7;
                        } else {
                            if (u_var8 != 9) {}
                            // goto LAB_1020_af1c;
                            pass1_1008_3e76(param_1, local_8, local_46, local_4e);
                            u_var9 = local_20;
                            u_var10 = (local_20 >> 8);
                            u_var11 = (local_20 >> 0x10);
                            u_var8 = 8;
                        }
                    }
                    pass1_1028_87f0(
                        CONCAT22(unaff_ss, &local_17e),
                        0,
                        0,
                        u_var8,
                        param_1,
                        (param_1 >> 0x10),
                        CONCAT22(u_var11, CONCAT11(u_var10, u_var9)),
                        param_2,
                    );
                    pass1_1030_835a(ctx._g_bool_1050_5748, CONCAT22(unaff_ss, &local_17e));
                    local_17e = ctx.s_1_1050_389a;
                    local_17c = &ctx.PTR_LOOP_1050_1008;
                }
                // LAB_1020_af1c:
                local_4e = local_4e + 1;
            }
            local_46 = local_46 + 1;
        }
    }
    return;
}

pub fn pass1_1020_afc4(param_1: u16, param_2: u16, param_1_00: Vec<u8>, param_2_00: Vec<u8>) {
    let paVar1: *mut Struct493;
    let local_eax_22: Vec<u8>;
    let mut local_DX_22: u16;
    let mut local_DX_71: u16;
    let mut u_var2: i32;

    let mut u_var3: u32;
    let mut local_1e: u32;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: u32;
    let mut local_a: u32;
    let mut local_6: u32;

    local_eax_22._0_2_ = &local_a;
    pass1_1030_64ce(
        _PTR_LOOP_1050_5740,
        param_1_00,
        param_2_00,
        local_eax_22,
        ctx.stack_seg_reg,
    );
    local_6 = local_eax_22;
    local_DX_71 = (local_eax_22 + 2);
    local_1e._3_1_ = (local_6 >> 0x18);
    if (local_1e._3_1_ == '\0') {
        return;
    }
    paVar1 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, local_6, local_DX_71);
    u_var3 = pass1_1030_73a8(CONCAT22(local_DX_71, paVar1));
    u_var2 = (u_var3 >> 0x10);
    if ((u_var2 | u_var3) != 0) {
        match (u_var3 + 0xc) {
            1 => {}
            2 => {}
            3 => {}
            4 => {}
            5 => {}
            6 => {}
            7 => return,
            8 => return,
            9 => return,

            _ => return,
        }
    }
    return;
}

pub fn pass1_1020_b0aa(param_1: u16, param_2: u16, param_1_00: i32) {
    let mut i_var1: i32;
    let pu_var2: *mut u32;
    let ppc_var3: fn();
    let mut u_var4: u32;
    let local_AX_71: *mut Struct702;
    let local_AX_192: *mut Struct703;
    let pu_var5: *mut u32;
    let mut in_dx: u16;


    let mut u_var7: u16;
    let mut local_1c: u32;
    let mut local_14: u32;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u32;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut u_var6: u32;

    u_var7 = (_PTR_LOOP_1050_4e74 >> 0x10);
    if ((_PTR_LOOP_1050_4e74 + param_1_00 * 6 + 4) == 0) {
        return;
    }
    if ((_PTR_LOOP_1050_4e74 + param_1_00 * 6 + 4) != -1) {
        if (PTR_LOOP_1050_4e78 == 0x0) {
            local_AX_71 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, 2, 0x400);
            pu_var2 = local_AX_71.field_0xc;
            unsafe { ppc_var3 = (*pu_var2 + 0x10) };
            pu_var5 = pu_var2;
            (**ppc_var3)();
            u_var4 = pu_var5 & 0xffff | ctx.dx_reg << 0x10;
            local_14 = 0;
            while (local_14 < u_var4) {
                u_var6 = u_var4;
                pass1_1030_1d7c(pu_var2, local_14);
                if (((ctx.dx_reg | u_var6) != 0)
                    && ((i_var1 = (u_var6 + 0xc), i_var1 == 0x2a || (i_var1 == 0x2b))))
                {
                    PTR_LOOP_1050_4e78 = (&ctx.PTR_LOOP_1050_0000 + 1);
                    break;
                }
                local_14 = local_14 + 1;
            }
            if (PTR_LOOP_1050_4e78 == 0x0) {
                PTR_LOOP_1050_4e78 = (&ctx.PTR_LOOP_1050_0000 + 1);
                return;
            }
        }
        pass1_fn_1008_612e(0, (_PTR_LOOP_1050_4e74 + param_1_00 * 6 + 4) + -1);
    }
    return;
}

pub fn pass1_1020_b1ae(param_1: Vec<u8>, param_1_00: Vec<u8>, param_2_00: Vec<u8>) -> bool {
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
    pu_var1 = pass1_1030_5b5c(local_6, in_dx);
    unsafe { local_c = *pu_var1 };
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
    if ((((0xb < local_e) && (0xb < local_10)) && (local_e < (local_12 - 0xb)))
        && (local_10 < (local_14 - 0xb)))
    {
        return 1;
    }
    return 0;
}

pub fn pass1_1020_b240(param_1: Vec<u8>, param_2: Vec<u8>, param_3: Vec<u8>) {
    let pu_var1: *mut u32;
    let paVar2: *mut Struct493;
    let BVar3: bool;

    let mut u_var4: u16;
    let mut u_var5: i32;
    let mut u_var6: u16;
    let mut unaff_ss: u16;
    let mut u_var7: u32;
    let mut local_22: u32;
    let mut local_1e: u16;
    let mut local_1c: u16;
    let mut local_1a: u16;
    let mut local_18: u32;
    let mut local_a: u32;
    let mut local_6: u32;

    pu_var1 = &local_a;
    u_var6 = (param_3 >> 0x10);
    pass1_1030_64ce(
        _PTR_LOOP_1050_5740,
        param_2,
        (param_3 + 4),
        pu_var1,
        unaff_ss,
    );
    unsafe { local_6 = *pu_var1 };
    u_var4 = (pu_var1 + 2);
    local_22._3_1_ = (local_6 >> 0x18);
    if (local_22._3_1_ != '\0') {
        paVar2 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, local_6, u_var4);
        u_var7 = pass1_1030_73a8(CONCAT22(u_var4, paVar2));
        u_var5 = (u_var7 >> 0x10);
        if (((u_var5 | u_var7) != 0) && (9 < (u_var7 + 0xc))) {
            return;
        }
    }
    BVar3 = pass1_1020_b1ae(param_1, param_2, *(param_3 + 4));
    if (BVar3 == 0) {
        return;
    }
    return;
}

pub fn pass1_1020_b2da(param_1: Vec<u8>, param_2: u16, param_3: u32, param_4: u32) {
    let mut u_var1: u16;
    let mut u_var2: u16;
    let BVar3: bool;
    let mut i_var4: i32;
    let mut unaff_ss: u16;
    word * *ppwVar5;
    let mut local_1c: u16;
    let mut local_1a: [u8; 6];
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    if (param_2 == 0) {
        u_var2 = 0x4e6a;
    } else {
        u_var2 = 0x4e6e;
    }
    _local_c = CONCAT22(0x1050, u_var2);
    if (param_2 == 0) {
        local_14 = 0x4e68;
    } else {
        local_14 = 0x4e6c;
    }
    local_12 = &ctx.g_alloc_addr_1050_1050;
    _local_10 = CONCAT22(0x1050, local_14);
    loop {
        if (param_2 == 0) {
            ppwVar5 = &PTR_LOOP_1048_4230;
        } else {
            ppwVar5 = 0x10484236;
        }
        pass1_1008_3eb4(
            ppwVar5,
            CONCAT22(unaff_ss, &local_8),
            CONCAT22(unaff_ss, &local_6),
            CONCAT22(unaff_ss, &local_4),
        );
        u_var1 = *_local_c;
        if (u_var1 == 0) {
            _local_6 = CONCAT22(local_4 + *_local_10, local_6 - 1);
        } else {
            if (u_var1 == 1) {
                _local_6 = CONCAT22(local_4 - 1, local_6 + *_local_10);
            } else {
                if (u_var1 == 2) {
                    _local_6 = CONCAT22(local_4 - *_local_10, local_6 - 1);
                }
            }
        }
        pass1_1008_3e54(
            CONCAT22(unaff_ss, local_1a),
            local_8,
            _local_6,
            (_local_6 >> 0x10),
        );
        u_var2 = (param_4 >> 0x10);
        BVar3 = pass1_1020_b1ae(param_1, CONCAT22(unaff_ss, local_1a), *(param_4 + 4));
        if (BVar3 != 0) {
            i_var4 = pass1_1020_b240(param_1, CONCAT22(unaff_ss, local_1a), param_4);
            if (i_var4 != 0) {
                // LAB_1020_b46e:
                pass1_1008_3e76(param_3, local_8, _local_6, (_local_6 >> 0x10));
                return;
            }
        }
        u_var1 = *_local_c;
        if (u_var1 == 0) {
            // LAB_1020_b45e:
            _local_6 = _local_6 & 0xffff0000 | (local_6 + 2);
        } else {
            if (u_var1 == 1) {
                _local_6 = _local_6 & 0xffff | (local_4 + 2) << 0x10;
            } else {
                if (u_var1 == 2) {}
                // goto LAB_1020_b45e;
            }
        }
        pass1_1008_3e76(
            CONCAT22(unaff_ss, local_1a),
            local_8,
            _local_6,
            (_local_6 >> 0x10),
        );
        BVar3 = pass1_1020_b1ae(param_1, CONCAT22(unaff_ss, local_1a), *(param_4 + 4));
        if (BVar3 != 0) {
            i_var4 = pass1_1020_b240(param_1, CONCAT22(unaff_ss, local_1a), param_4);
            if (i_var4 != 0) {}
            // goto LAB_1020_b46e;
        }
        local_1c = *_local_c + 1;
        if (2 < local_1c) {
            local_1c = 0;
            *_local_10 = *_local_10 + 1;
        }
        *_local_c = local_1c;
        pass1_1020_ac6e(param_1, param_2, *_local_10, local_1c);
    }
}

pub fn infinite_loop_1020_b482(param_1: Vec<u8>, param_2: *mut Vec<u8>, param_3: Vec<u8>) {
    let mut u_var1: u32;
    let paVar2: *mut Struct493;
    let pu_var3: Vec<u8>;
    let pu_var4: *mut u16;
    let pu_var5: *mut u32;

    let mut u_var6: i32;
    let mut unaff_ss: u16;
    let mut u_var7: u16;
    let mut u_var8: u16;
    // ppu_var9: *mut Vec<u8>;
    let pu_var10: Vec<u8>;
    let mut local_42: u16;
    let mut local_3e: u16;
    let mut local_38: u16;
    let mut local_36: u16;
    let mut local_34: u16;
    let mut local_32: u32;
    let mut local_2e: u16;
    let mut local_2c: u16;
    let mut local_2a: u16;
    let mut local_28: u16;
    let mut local_26: u16;
    let mut local_24: u32;
    let mut uStack32: u16;
    let mut local_1e: u32;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_4: [u8; 2];

    pass1_1030_bcae(local_4, unaff_ss);
    pass1_1028_dc52(
        CONCAT22(unaff_ss, &local_16),
        (&ctx.PTR_LOOP_1050_0000 + 1),
        0,
        0x400,
    );
    while (true) {
        pu_var4 = &local_16;
        pass1_1028_e4ec(CONCAT22(unaff_ss, pu_var4));
        _local_1a = CONCAT22(ctx.dx_reg, pu_var4);
        u_var6 = ctx.dx_reg | pu_var4;
        if (u_var6 == 0) {
            pass1_1020_b240(param_1, param_2, param_3);
            if (pu_var4 != 0x0) {
                local_1e = (param_3 + 4);
                local_24 = param_2;
                uStack32 = (param_2 + 4);
                pass1_1008_3eb4(
                    CONCAT22(unaff_ss, &local_24),
                    CONCAT22(unaff_ss, &local_2a),
                    CONCAT22(unaff_ss, &local_28),
                    CONCAT22(unaff_ss, &local_26),
                );
                pass1_1008_3e76(
                    CONCAT22(unaff_ss, &local_24),
                    local_2a,
                    local_28 - 1,
                    local_26 - 1,
                );
                pu_var5 = &local_24;
                u_var7 = param_1;
                u_var8 = (param_1 >> 0x10);
                pass1_1020_afc4(u_var7, u_var8, CONCAT22(unaff_ss, pu_var5), local_1e);
                if (pu_var5 != 0x0) {
                    pass1_1008_3e76(
                        CONCAT22(unaff_ss, &local_24),
                        _local_2a,
                        (_local_2a >> 0x10),
                        local_26 - 1,
                    );
                    pu_var5 = &local_24;
                    pass1_1020_afc4(u_var7, u_var8, CONCAT22(unaff_ss, pu_var5), local_1e);
                    if (pu_var5 != 0x0) {
                        pass1_1008_3e76(
                            CONCAT22(unaff_ss, &local_24),
                            local_2a,
                            local_28 + 1,
                            local_26 - 1,
                        );
                        pu_var5 = &local_24;
                        pass1_1020_afc4(u_var7, u_var8, CONCAT22(unaff_ss, pu_var5), local_1e);
                        if (pu_var5 != 0x0) {
                            pass1_1008_3e76(
                                CONCAT22(unaff_ss, &local_24),
                                local_2a,
                                local_28 - 1,
                                local_26,
                            );
                            pu_var5 = &local_24;
                            pass1_1020_afc4(u_var7, u_var8, CONCAT22(unaff_ss, pu_var5), local_1e);
                            if (pu_var5 != 0x0) {
                                pass1_1008_3e76(
                                    CONCAT22(unaff_ss, &local_24),
                                    local_2a,
                                    local_28 + 1,
                                    local_26,
                                );
                                pu_var5 = &local_24;
                                pass1_1020_afc4(
                                    u_var7,
                                    u_var8,
                                    CONCAT22(unaff_ss, pu_var5),
                                    local_1e,
                                );
                                if (pu_var5 != 0x0) {
                                    pass1_1008_3e76(
                                        CONCAT22(unaff_ss, &local_24),
                                        local_2a,
                                        local_28 + 1,
                                        local_26 + 1,
                                    );
                                    pu_var5 = &local_24;
                                    pass1_1020_afc4(
                                        u_var7,
                                        u_var8,
                                        CONCAT22(unaff_ss, pu_var5),
                                        local_1e,
                                    );
                                    if (pu_var5 != 0x0) {
                                        pass1_1008_3e76(
                                            CONCAT22(unaff_ss, &local_24),
                                            _local_2a,
                                            (_local_2a >> 0x10),
                                            local_26 + 1,
                                        );
                                        pu_var5 = &local_24;
                                        pass1_1020_afc4(
                                            u_var7,
                                            u_var8,
                                            CONCAT22(unaff_ss, pu_var5),
                                            local_1e,
                                        );
                                        if (pu_var5 != 0x0) {
                                            pass1_1008_3e76(
                                                CONCAT22(unaff_ss, &local_24),
                                                local_2a,
                                                local_28 - 1,
                                                local_26 + 1,
                                            );
                                            pu_var5 = &local_24;
                                            pass1_1020_afc4(
                                                u_var7,
                                                u_var8,
                                                CONCAT22(unaff_ss, pu_var5),
                                                local_1e,
                                            );
                                            if (pu_var5 != 0x0) {
                                                pass1_1008_3e76(
                                                    CONCAT22(unaff_ss, &local_24),
                                                    local_2a,
                                                    local_28 - 2,
                                                    local_26 - 2,
                                                );
                                                pu_var5 = &local_24;
                                                pass1_1020_afc4(
                                                    u_var7,
                                                    u_var8,
                                                    CONCAT22(unaff_ss, pu_var5),
                                                    local_1e,
                                                );
                                                if (pu_var5 != 0x0) {
                                                    pass1_1008_3e76(
                                                        CONCAT22(unaff_ss, &local_24),
                                                        local_2a,
                                                        local_28 + 2,
                                                        local_26 - 2,
                                                    );
                                                    pu_var5 = &local_24;
                                                    pass1_1020_afc4(
                                                        u_var7,
                                                        u_var8,
                                                        CONCAT22(unaff_ss, pu_var5),
                                                        local_1e,
                                                    );
                                                    if (pu_var5 != 0x0) {
                                                        pass1_1008_3e76(
                                                            CONCAT22(unaff_ss, &local_24),
                                                            local_2a,
                                                            local_28 - 2,
                                                            local_26 + 2,
                                                        );
                                                        pu_var5 = &local_24;
                                                        pass1_1020_afc4(
                                                            u_var7,
                                                            u_var8,
                                                            CONCAT22(unaff_ss, pu_var5),
                                                            local_1e,
                                                        );
                                                        if (pu_var5 != 0x0) {
                                                            pass1_1008_3e76(
                                                                CONCAT22(unaff_ss, &local_24),
                                                                local_2a,
                                                                local_28 + 2,
                                                                local_26 + 2,
                                                            );
                                                            pu_var5 = &local_24;
                                                            pass1_1020_afc4(
                                                                u_var7,
                                                                u_var8,
                                                                CONCAT22(unaff_ss, pu_var5),
                                                                local_1e,
                                                            );
                                                            if (pu_var5 != 0x0) {
                                                                pass1_1008_3e76(
                                                                    CONCAT22(unaff_ss, &local_24),
                                                                    local_2a,
                                                                    local_28 - 1,
                                                                    local_26 + 2,
                                                                );
                                                                pu_var5 = &local_24;
                                                                pass1_1020_afc4(
                                                                    u_var7,
                                                                    u_var8,
                                                                    CONCAT22(unaff_ss, pu_var5),
                                                                    local_1e,
                                                                );
                                                                if (pu_var5 != 0x0) {
                                                                    pass1_1008_3e76(
                                                                        CONCAT22(
                                                                            unaff_ss, &local_24,
                                                                        ),
                                                                        local_2a,
                                                                        local_28 - 1,
                                                                        local_26 + 3,
                                                                    );
                                                                    pu_var5 = &local_24;
                                                                    pass1_1020_afc4(
                                                                        u_var7,
                                                                        u_var8,
                                                                        CONCAT22(unaff_ss, pu_var5),
                                                                        local_1e,
                                                                    );
                                                                    if (pu_var5 != 0x0) {
                                                                        local_2e = 3;
                                                                        while (true) {
                                                                            if (9 < local_2e) {
                                                                                return;
                                                                            }
                                                                            pass1_1008_3e76(
                                                                                CONCAT22(
                                                                                    unaff_ss,
                                                                                    &local_24,
                                                                                ),
                                                                                0,
                                                                                local_28 - local_2e,
                                                                                local_26,
                                                                            );
                                                                            pu_var5 = &local_24;
                                                                            pass1_1020_afc4(
                                                                                u_var7,
                                                                                u_var8,
                                                                                CONCAT22(
                                                                                    unaff_ss,
                                                                                    pu_var5,
                                                                                ),
                                                                                local_1e,
                                                                            );
                                                                            if (pu_var5 == 0x0) {
                                                                                break;
                                                                            }
                                                                            local_2e = local_2e + 1;
                                                                        }
                                                                        return;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            return;
        }
        u_var1 = (pu_var4 + 8);
        ppu_var9 = param_2;
        pu_var10 = param_3;
        paVar2 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, u_var1, (u_var1 >> 0x10));
        pu_var3 = local_4;
        pass1_1030_bcbc(
            CONCAT22(unaff_ss, pu_var3),
            CONCAT22(u_var6, paVar2),
            ppu_var9,
            pu_var10,
        );
        if (pu_var3 < 0) {
            break;
        }
        if (pu_var3 < 0x65) {
            return;
        }
    }
    return;
}

pub fn call_infinite_loop_1020_b872(param_1: Vec<u8>, param_2: Vec<u8>) {
    let mut u_var1: u16;
    let mut u_var2: u16;
    let mut i_var3: i32;
    let mut unaff_ss: u16;
    let pu_var4: *mut u32;
    let u_var5: u8;
    let mut u_var6: u16;
    let mut local_13a: u16;
    let mut local_138: u16;
    let mut local_136: u16;
    let mut local_134: u16;
    let mut local_12: u32;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u32;
    let mut uStack4: u16;

    u_var6 = (param_2 >> 0x10);
    pu_var4 = pass1_1030_5b5c(param_2, u_var6);
    unsafe { local_8 = *pu_var4 };
    uStack4 = (pu_var4 + 4);
    u_var5 = (unaff_ss >> 8);
    pass1_1008_3e94(
        &local_8,
        CONCAT22(unaff_ss, &local_c),
        CONCAT22(unaff_ss, &local_a),
    );
    u_var1 = pass1_fn_1008_612e(10, local_a - 10);
    u_var2 = pass1_fn_1008_612e(10, local_c - 10);
    pass1_1008_3e54(
        CONCAT13(u_var5, CONCAT12(unaff_ss, &local_12)),
        0,
        u_var2,
        u_var1,
    );
    while (true) {
        i_var3 = infinite_loop_1020_b482(param_1, CONCAT22(unaff_ss, &local_12), param_2);
        if (i_var3 != 0) {
            break;
        }
        u_var1 = pass1_fn_1008_612e(10, local_a - 10);
        u_var2 = pass1_fn_1008_612e(10, local_c - 10);
        pass1_1008_3e76(
            CONCAT13(u_var5, CONCAT12(unaff_ss, &local_12)),
            0,
            u_var2,
            u_var1,
        );
    }
    pass1_1028_8888(
        CONCAT22(unaff_ss, &local_136),
        0,
        10,
        &local_12,
        unaff_ss,
        0x8000002,
        0,
        (param_2 + 4),
    );
    pass1_1030_835a(ctx._g_bool_1050_5748, CONCAT22(unaff_ss, &local_136));
    pass1_1020_b97e(param_1, (param_1 >> 0x10), 1);
    return;
}

pub fn pass1_1020_b97e(param_1: u16, param_2: u16, param_1_00: i32) {
    let paVar1: *mut Struct493;
    let mut in_dx: u16;
    let mut unaff_ss: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;

    paVar1 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, 2, 0x400);
    _PTR_LOOP_1050_4e70 = CONCAT22(in_dx, paVar1);
    local_6 = &paVar1.field_0x10;
    local_a = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, local_6, (local_6 >> 0x10));
    modify_list_1008_3f62(&PTR_LOOP_1048_4230, CONCAT22(in_dx, local_a + 0xc));
    pass1_1008_3e94(
        &PTR_LOOP_1050_4230,
        CONCAT22(unaff_ss, &local_e),
        CONCAT22(unaff_ss, &local_c),
    );
    if (param_1_00 == 0) {
        pass1_1008_3e76(&PTR_LOOP_1048_4230, 0, local_e + 1, local_c - 1);
        pass1_1008_3e94(
            &PTR_LOOP_1050_4230,
            CONCAT22(unaff_ss, &local_e),
            CONCAT22(unaff_ss, &local_c),
        );
    }
    pass1_1008_3e76(0x10484236, 1, local_e - 2, local_c);
    return;
}

pub fn pass1_1020_ba3e(param_1: *mut Struct704, param_2: u16, param_3: u16) {
    let local_bx_3: *mut Struct704;
    let local_es_3: *mut Struct704;

    local_es_3 = (param_1 >> 0x10);
    local_bx_3 = param_1;
    param_1 = 0;
    local_bx_3.field_0x4 = 0;
    local_bx_3.field_0x6 = param_3;
    local_bx_3.field_0x8 = param_2;
    if (local_bx_3.field_0x6 == 0) {
        local_bx_3.field_0x6 = 5;
    }
    call_alloc_mem_fn_1020_bcc4(param_1);
    return;
}

pub fn pass1_1020_ba7e(param_1: *mut &mut Struct44) {
    error_check_1000_17ce(param_1);
    return;
}

pub fn infinite_loop_1020_ba94(param_1: *mut long) {
    let pu_var1: *mut u32;
    let mut local_8: u16;
    let mut local_6: u32;

    let param_1_val = unsafe { *param_1 };
    if (param_1_val == 0) {
        return;
    }
    local_8 = 0;
    while (true) {
        pu_var1 = (param_1 + 4);
        let pu_var1_val = unsafe { *pu_var1 };
        if (pu_var1_val < local_8 || pu_var1_val == local_8) {
            break;
        }
        local_8 = local_8 + 1;
    }
    return;
}

pub fn pass1_1020_bae6(param_1: u16, param_2: u32) {
    let mut in_eax: u32;
    let local_DXAX_13: *mut u16;
    let mut local_6: u16;
    let mut local_4: u16;

    pass1_1020_bc92(param_1, param_2);
    if ((local_DXAX_13._2_2_ | in_eax) != 0) {
        return in_eax & 0xffff0000 | *(in_eax & 0xffff | local_DXAX_13._2_2_ << 0x10);
    }
    return in_eax & 0xffff0000;
}

pub fn pass1_1020_bb16(param_1: *mut Vec<u8>, param_2: u32, param_3: u32, param_4: u16) {
    if ((param_1 + 4) < param_4) {
        param_3 = 0;
        param_2 = 0;
        return;
    }
    param_3 = (param_4 * 6 + param_1 + 4);
    param_2 = (param_1 + param_4 * 6);
    return;
}

pub fn pass1_1020_bb70(param_1: Vec<u8>, param_2: u16, param_3: Vec<u8>) {
    pass1_1020_bba4(param_1, 1, param_2, param_3);
    return;
}

pub fn pass1_1020_bb8a(param_1: Vec<u8>, param_2: u16, param_3: Vec<u8>) {
    pass1_1020_bba4(param_1, 0, param_2, param_3);
    return;
}

pub fn pass1_1020_bba4(param_1: Vec<u8>, param_2: u16, param_3: u16, param_4: Vec<u8>) {
    let mut u_var1: u16;
    let mut b_var2: bool;
    let pu_var3: *mut u32;
    let mut local_6: u16;
    let mut local_4: u16;

    pu_var3 = pass1_1020_bc92(param_1, param_4._2_2_);
    u_var1 = (pu_var3 >> 0x10);
    if (pu_var3 == 0x0) {
        pu_var3 = pass1_1020_bc92(param_1, 0);
        if (pu_var3 == 0x0) {
            call_alloc_mem_fn_1020_bcc4(param_1);
            pu_var3 = pass1_1020_bc92(param_1, 0);
            if (pu_var3 == 0x0) {
                return 0;
            }
            (pu_var3 + 4) = param_4._2_2_;
        } else {
            (pu_var3 + 4) = param_4._2_2_;
        }
        u_var1 = (pu_var3 >> 0x10);
        if (param_2 != 0) {
            unsafe {
                b_var2 = CARRY2(*pu_var3, param_3);
                param_3 = *pu_var3 + param_3;
            }
            param_4._0_2_ = (pu_var3 + 2) + param_4 + b_var2;
        }
        unsafe {
            *pu_var3 = param_3;
            (pu_var3 + 2) = param_4
        };
        pass1_1020_bc72(param_1);
    } else {
        if (param_2 != 0) {
            unsafe {
                b_var2 = CARRY2(*pu_var3, param_3);
                param_3 = *pu_var3 + param_3;
            }
            param_4._0_2_ = (pu_var3 + 2) + param_4 + b_var2;
        }
        unsafe {
            *pu_var3 = param_3;
            (pu_var3 + 2) = param_4;
        }
    }
    return 1;
}

pub fn pass1_1020_bc72(param_1: *mut u16) {
    let param_1_val = unsafe { *param_1 };
    pass1_1000_4aea(param_1_val, (param_1 + 2), 6, 0xbd6c, 0x1020);
    return;
}

pub fn pass1_1020_bc92(param_1: *mut u16, param_2: u16) {
    let mut u_var1: u32;
    let mut unaff_ss: u16;
    let mut local_c: [u8; 4];
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    local_8 = param_2;
    u_var1 = (param_1 + 2);
    let param_1_val = unsafe { *param_1 };
    pass1_1000_49c6(
        local_c,
        unaff_ss,
        param_1_val,
        u_var1,
        (u_var1 >> 0x10),
        6,
        0xbd6c,
    );
    return;
}

pub fn call_alloc_mem_fn_1020_bcc4(in_struct_1: *mut Struct705) {
    let pu_var1: *mut u32;
    let mut i_var2: i32;
    let mut u_var3: i32;
    let ctx.dx_reg: *mut u16;
    let local_struct_1: *mut Struct705;
    let local_struct_1_hi: *mut Struct705;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;

    local_struct_1_hi = (in_struct_1 >> 0x10);
    local_struct_1 = in_struct_1;
    if (local_struct_1.field_0x4 == 0) {
        ctx.g_u16_ptr_1050_5f2e = 0x0;
        i_var2 = local_struct_1.field_0x6;
    } else {
        u_var3 = local_struct_1.field_0x4;
        pu_var1 = &local_struct_1.field_0x8;
        unsafe {
            i_var2 = u_var3 + *pu_var1;
            ctx.g_u16_ptr_1050_5f2e = CARRY2(u_var3, *pu_var1)
        };
    }
    if (ctx.g_u16_ptr_1050_5f2e == 0x0) {
        if (in_struct_1 == 0) {
            if (ctx.__g_Struct94_ptr_1 == 0) {
                struct_fn_1000_160a();
            } else {
            }
            u_var3 = i_var2 * 6;
            alloc_mem_1000_1708(u_var3, 0, 1);
        } else {
            u_var3 = i_var2 * 6;
            alloc_mem_1000_0ed4(1, u_var3, 0, in_struct_1);
            ctx.g_u16_ptr_1050_5f2e = ctx.dx_reg;
        }
        _local_c = CONCAT22(ctx.g_u16_ptr_1050_5f2e, u_var3);
        if ((ctx.g_u16_ptr_1050_5f2e | u_var3) != 0) {
            local_struct_1.field_0x4 = i_var2;
            in_struct_1 = _local_c;
            pass1_1020_bc72((in_struct_1 & 0xffff | ZEXT24(local_struct_1_hi) << 0x10));
        }
    }
    return;
}

pub fn pass1_1020_bd6c(param_1: Vec<u8>, param_2: Vec<u8>) -> i32 {
    return (param_1 + 4) - (param_2 + 4);
}

pub fn pass1_1020_a43e(param_1: *mut u16) {
    let mut in_stack_0000fff2: u32;

    unsafe {
        *param_1 = 0xba36;
        (param_1 + 2) = 0x1020;
    }
    if (_PTR_LOOP_1050_4e74 != 0) {
        return param_1;
    }
    process_struct_1010_20ba(
        ctx._g_Struct372_1050_0ed0,
        CONCAT22((in_stack_0000fff2 >> 0x10), 2),
    );
    if ((0 < u16_1050_13ae) && (!SBORROW2(u16_1050_13ae, 1))) {
        if (u16_1050_13ae == 2 || (u16_1050_13ae - 1) < 1) {
            PTR_LOOP_1050_4e74 = 0x44b4;
            // goto LAB_1020_a482;
        }
        if (u16_1050_13ae == 4) {
            PTR_LOOP_1050_4e74 = 0x4b2c;
            // goto LAB_1020_a482;
        }
    }
    PTR_LOOP_1050_4e74 = 0x47f0;
    // LAB_1020_a482:
    _PTR_LOOP_1050_4e74 = CONCAT22(0x1050, PTR_LOOP_1050_4e74);
    return param_1;
}

pub fn pass1_1020_a49a(param_1: *mut u16, param_2: u32, param_3: u16) {
    let mut u_var1: u16;
    let mut unaff_ss: u16;
    let mut u_var2: u16;
    let mut in_stack_0000feba: u16;
    let mut local_136: u16;
    let mut local_134: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    _local_6 = process_struct_1010_20ba(
        ctx._g_Struct372_1050_0ed0,
        CONCAT22(in_stack_0000feba, 0x2f),
    );
    local_c = (_local_6 >> 0x10);
    local_a = (_local_6 + 0x20);
    local_e = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, local_a, (local_a >> 0x10));
    if (param_2 != 0) {
        u_var1 = (param_2 >> 0x10);
        if ((param_2 + 1) == 0) {
            u_var2 = SUB42(&PTR_LOOP_1050_4230, 0);
        } else {
            u_var2 = SUB42(s_dib_1050_4234 + 2, 0);
        }
        pass1_1008_3f32(param_2, u_var2, &PTR_LOOP_1050_1048);
        pass1_1028_87f0(
            CONCAT22(unaff_ss, &local_136),
            0,
            0,
            param_3,
            param_2,
            u_var1,
            (_PTR_LOOP_1050_4e70 + 4),
            local_a,
        );
        pass1_1030_835a(ctx._g_bool_1050_5748, CONCAT22(unaff_ss, &local_136));
        return;
    }
    pass1_1020_abc0(param_1, param_3, local_e, local_c);
    return;
}

pub fn pass1_1020_a54c(param_1: u16, param_2: u16, param_1_00: i32) {
    let mut u_var1: u32;
    let mut unaff_si: u16;
    let mut unaff_ss: u16;
    let mut u_var2: u16;
    let mut u_var3: u16;
    let mut u_var4: u16;
    let mut u_var5: u16;
    let mut u_var6: u16;
    let mut local_140: u16;
    let mut local_13e: u16;
    let mut local_1c: u16;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: [u8; 6];
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    _local_6 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(unaff_si, 0x2f));
    local_c = (_local_6 >> 0x10);
    local_a = (_local_6 + 0x20);
    local_e = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, local_a, (local_a >> 0x10));
    local_14._0_4_ = _PTR_LOOP_1048_4230;
    local_14._4_2_ = PTR_LOOP_1048_4234;
    local_1c = local_14;
    pass1_1008_3e94(
        local_14,
        CONCAT22(unaff_ss, &local_18),
        CONCAT22(unaff_ss, &local_16),
    );
    if ((param_1_00 < 0) || (5 < param_1_00)) {
        pass1_1008_3e76(CONCAT22(unaff_ss, local_14), 0, local_18 - 9, local_16);
        u_var5 = local_a;
        u_var6 = (local_a >> 0x10);
        u_var1 = (_PTR_LOOP_1050_4e70 + 4);
        u_var3 = u_var1;
        u_var4 = (u_var1 >> 0x10);
        u_var2 = 0x14;
    } else {
        pass1_1008_3e76(
            CONCAT22(unaff_ss, local_14),
            0,
            (local_18 - param_1_00) - 3,
            local_16,
        );
        u_var5 = local_a;
        u_var6 = (local_a >> 0x10);
        u_var1 = (_PTR_LOOP_1050_4e70 + 4);
        u_var3 = u_var1;
        u_var4 = (u_var1 >> 0x10);
        u_var2 = 0x7b;
    }
    pass1_1028_87f0(
        CONCAT22(unaff_ss, &local_140),
        0,
        0,
        u_var2,
        local_14,
        unaff_ss,
        CONCAT22(u_var4, u_var3),
        CONCAT22(u_var6, u_var5),
    );
    pass1_1030_835a(ctx._g_bool_1050_5748, CONCAT22(unaff_ss, &local_140));
    return;
}

pub fn pass1_1020_8e6c(param_1: &mut Struct44, param_2: u8) -> &mut Struct44 {
    pass1_1020_8bae(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1020_8eaa(param_1: *mut Struct393) {
    let mut u_var1: i32;
    let mut u_var2: u16;
    let mut u_var3: u16;

    let local_bx_17: *mut Struct695;
    let mut unaff_si: u16;
    let mut u_var4: i32;
    let mut unaff_ss: u16;
    let ppVar5: *mut Struct2111;

    process_struct_1020_847a(param_1, 0x25);
    u_var4 = (param_1 >> 0x10);
    local_bx_17 = param_1;
    &local_bx_17.field_0x16 = 0;
    local_bx_17.field_0xaa = 0;
    u_var1 = &local_bx_17.field_0xae;
    zero_list_1008_3e38((param_1 & 0xffff0000 | u_var1));
    &local_bx_17.field_0xb4 = 0;
    local_bx_17.field_0xb8 = 0xffff;
    &local_bx_17.field_0xba = 0;
    param_1.field_0x0 = 0x9204;
    local_bx_17.field_0x2 = 0x1020;
    ppVar5 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(unaff_si, 0x29));
    u_var3 = (ppVar5 >> 0x10);
    u_var2 = ppVar5;
    local_bx_17.field_0x16 = u_var2;
    local_bx_17.field_0x18 = u_var3;
    pass1_1018_2646(
        local_bx_17.field_0x16,
        u_var3,
        param_1 & 0xffff0000 | u_var1,
    );
    mixed_fn_1010_830a(ctx._g_struct_73_1050_14cc, 0x1ce);
    local_bx_17.field_0xb4 = u_var2;
    local_bx_17.field_0xb6 = ctx.dx_reg;
    pass1_1020_8712(
        (param_1 & 0xffff | u_var4 << 0x10),
        &stack0xfff6,
        unaff_ss,
        CONCAT22(ctx.dx_reg, local_bx_17.field_0xb4),
        param_1 & 0xffff0000 | u_var1,
    );
    ppVar5 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(unaff_si, 2));
    local_bx_17.field_0xba = ppVar5;
    local_bx_17.field_0xbc = (ppVar5 >> 0x10);
    return;
}

pub fn pass1_1020_8f74(param_1: &mut Struct44) {
    let pu_var1: *mut u32;
    let mut u_var2: i32;
    let local_bx_5: &mut Struct44;
    let local_es_5: &mut Struct44;
    let fn_ptr_1: fn();

    local_es_5 = (param_1 >> 0x10);
    local_bx_5 = param_1;
    param_1.ptr_a_lo = 0x9204;
    local_bx_5.ptr_a_hi = 0x1020;
    pu_var1 = local_bx_5.field_0xb4;
    u_var2 = local_bx_5.field_0xb6;
    if ((u_var2 | pu_var1) != 0) {
        unsafe {
            fn_ptr_1 = *pu_var1;
            (**fn_ptr_1)()
        };
    }
    pass1_1020_8556(param_1);
    return;
}

pub fn invalidate_rect_1020_8fb4(param_1: u32) {
    let mut i_var1: i32;
    let mut u_var2: u32;

    let rect: *mut RECT16;
    let mut u_var3: u32;


    let mut hwnd: i32;
    let mut i_var4: i32;
    let mut u_var5: u16;
    let mut local_16: u16;
    let mut local_e: u32;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    u_var5 = (param_1 >> 0x10);
    i_var4 = param_1;
    u_var2 = (i_var4 + 0xba);
    if ((u_var2 + 0x1e) != 0) {
        pass1_1018_2862((i_var4 + 0x16));
        (i_var4 + 0xaa) = in_ax;
        (i_var4 + 0xac) = ctx.dx_reg;
        if ((ctx.dx_reg | (i_var4 + 0xaa)) != 0) {
            u_var2 = (i_var4 + 0xaa);
            i_var1 = (u_var2 + 10);
            local_8 = 0;
            while (local_8 < i_var1) {
                u_var3 = SEXT24(local_8);
                bad_func_1008_8fc4(*(i_var4 + 0xaa), u_var3);
                rect = u_var3;
                if ((((ctx.dx_reg | rect) != 0) && (9 < rect[5].bottom))
                    && (
                        pass1_1008_8b20((u_var3 & 0xffff | ctx.dx_reg << 0x10)),
                        (hwnd | rect) != 0,
                    ))
                {
                    InvalidateRect16(0, rect, hwnd);
                }
                local_8 = local_8 + 1;
            }
        }
    }
    return;
}

pub fn pass1_1020_9068(in_struct_1: *mut Struct696) {
    let mut i_var1: i32;
    let mut u_var2: u32;
    let paVar3: *mut Struct318;
    let mut u_var4: u32;
    let mut u_var5: i32;
    let mut u_var6: u32;




    let local_struct_1: *mut Struct696;
    let mut i_var7: i32;
    let local_struct_1_hi: *mut Struct696;
    let mut u_var8: u16;
    let mut local_10: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;
    let fn_ptr_1: fn();

    local_struct_1_hi = (in_struct_1 >> 0x10);
    local_struct_1 = in_struct_1;
    paVar3 = local_struct_1.struct318_ptr_0x16;
    u_var2 = (paVar3 + 10);
    u_var6 = u_var2;
    pass1_1018_280c(local_struct_1.struct318_ptr_0x16);
    local_struct_1.field_0xaa = u_var6;
    &local_struct_1.field_0xac = ctx.dx_reg;
    u_var5 = ctx.dx_reg | local_struct_1.field_0xaa;
    if (u_var5 == 0) {
        pass1_1018_2862(local_struct_1.struct318_ptr_0x16);
        local_struct_1.field_0xaa = u_var5;
        &local_struct_1.field_0xac = ctx.dx_reg;
    }
    if ((&local_struct_1.field_0xac | local_struct_1.field_0xaa) != 0) {
        pass1_1020_915a((in_struct_1 & 0xffff | ZEXT24(local_struct_1_hi) << 0x10));
        pass1_1008_4480(
            u_var2,
            (in_struct_1 & 0xffff0000 | &local_struct_1.field_0xae),
            local_struct_1.field_0xb4,
        );
        fn_ptr_1 = (in_struct_1 + 0x10);
        (**fn_ptr_1)();
        u_var4 = &local_struct_1.field_0xaa;
        i_var1 = (u_var4 + 10);
        local_a = 0;
        while (local_a < i_var1) {
            u_var6 = SEXT24(local_a);
            bad_func_1008_8fc4(*&local_struct_1.field_0xaa, u_var6);
            u_var5 = u_var6;
            if ((ctx.dx_reg | u_var5) != 0) {
                pass1_1008_8c4e((u_var6 & 0xffff | ctx.dx_reg << 0x10), u_var2);
                u_var4 = local_struct_1.field_0xc;
                u_var8 = (u_var4 >> 0x10);
                i_var7 = u_var4;
                (i_var7 + local_a * 4) = u_var5;
                (i_var7 + local_a * 4 + 2) = ctx.dx_reg;
            }
            local_a = local_a + 1;
        }
    }
    return;
}

pub fn pass1_1020_915a(param_1: *mut Struct697) {
    let mut i_var1: i32;
    let local_struct_1: *mut Struct697;
    let local_struct_1_hi: *mut Struct697;
    let ppVar2: *mut Struct2111;
    let mut u_var3: u32;
    let mut local_c: u16;
    let pcStack14: String;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    pcStack14 = CONCAT22(local_c, 0x2f);
    ppVar2 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, pcStack14);
    i_var1 = (ppVar2 + 0x1e);
    local_struct_1_hi = (param_1 >> 0x10);
    local_struct_1 = param_1;
    if (local_struct_1.field_0xb8 != i_var1) {
        local_c = 0x1ce;
        if (i_var1 == 1) {
            local_c = 0x1cf;
        } else {
            if (i_var1 == 2) {
                local_c = 0x1d0;
            } else {
                if (i_var1 == 3) {
                    local_c = 0x1d1;
                } else {
                    if (i_var1 == 4) {
                        local_c = 0x1d2;
                    }
                }
            }
        }
        u_var3 = mixed_fn_1010_830a(ctx._g_struct_73_1050_14cc, local_c);
        local_struct_1.field_0xb4 = u_var3;
        local_struct_1.field_0xb6 = (u_var3 >> 0x10);
        local_struct_1.field_0xb8 = i_var1;
    }
    return;
}

pub fn pass1_1020_91de(param_1: &mut Struct44, param_2: u8) -> &mut Struct44 {
    pass1_1020_8f74(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn process_struct_1020_8296(param_1: &mut Struct44, param_2: u8) -> &mut Struct44 {
    process_struct_1020_808e(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn process_struct_1020_8360(in_struct_1: *mut Struct680) {
    let mut u_var1: u32;
    let mut u_var2: u32;
    let mut u_var3: u16;
    let mut unaff_si: u16;
    let ppVar4: *mut Struct2111;
    let local_struct_1: *mut Struct680;
    let local_struct_1_hi: *mut Struct680;
    let mut local_6: u32;

    local_struct_1 = in_struct_1;
    local_struct_1_hi = (in_struct_1 >> 0x10);
    process_struct_1020_847a(in_struct_1, 1);
    zero_list_1008_3e38((in_struct_1 & 0xffff0000 | &local_struct_1.field_0x16));
    &local_struct_1.field_0x1c = 0;
    in_struct_1.field_0x0 = 0x8462;
    local_struct_1.field_0x2 = 0x1020;
    ppVar4 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(unaff_si, 0x29));
    u_var3 = (ppVar4 >> 0x10);
    local_struct_1.field_0x1c = ppVar4;
    &local_struct_1.field_0x1e = u_var3;
    pass1_1018_26f8(
        local_struct_1.field_0x1c,
        u_var3,
        in_struct_1 & 0xffff0000 | &local_struct_1.field_0x16,
    );
    u_var2 = &local_struct_1.field_0x1c;
    u_var1 = local_struct_1.field_0x8;
    pass1_1020_8712(
        (in_struct_1 & 0xffff | ZEXT24(local_struct_1_hi) << 0x10),
        u_var1,
        (u_var1 >> 0x10),
        (u_var2 + 0x2a),
        in_struct_1 & 0xffff0000 | &local_struct_1.field_0x16,
    );
    return;
}

pub fn pass1_1020_83f8(param_1: *mut Struct417) {
    let mut u_var1: u32;
    let mut u_var2: u32;
    let local_struct_1: *mut Struct417;
    let local_struct_1_hi: *mut Struct417;
    let mut local_6: u32;

    local_struct_1_hi = (param_1 >> 0x10);
    local_struct_1 = param_1;
    if (&local_struct_1.field_0x4 != 0) {
        u_var1 = &local_struct_1.field_0x1c;
        u_var2 = &local_struct_1.field_0x1c;
        pass1_1008_4480(
            (u_var1 + 10),
            (param_1 & 0xffff0000 | &local_struct_1.field_0x16),
            (u_var2 + 0x2a),
        );
    }
    return;
}

pub fn pass1_1020_843c(param_1: &mut Struct44, param_2: u8) -> &mut Struct44 {
    pass1_1020_8556(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn process_struct_1020_847a(param_1: *mut Struct393, param_2: u16) {
    let mut u_var1: u16;
    let struct_a: *mut Struct199;
    let struct_a_00: *mut Struct199;
    let local_struct_1: *mut Struct393;
    let mut unaff_si: u16;
    let local_struct_1_hi: *mut Struct393;
    let ppVar2: *mut Struct2111;

    local_struct_1_hi = (param_1 >> 0x10);
    local_struct_1._0_2_ = param_1;
    param_1.field_0x0 = ctx.s_1_1050_389a;
    (local_struct_1 + 2) = &ctx.PTR_LOOP_1050_1008;
    (local_struct_1 + 4) = 0;
    (local_struct_1 + 6) = param_2;
    (local_struct_1 + 8) = 0;
    (local_struct_1 + 0xc) = 0;
    zero_list_1008_3e38((param_1 & 0xffff0000 | (local_struct_1 + 0x10)));
    param_1.field_0x0 = 0x87aa;
    (local_struct_1 + 2) = 0x1020;
    ppVar2 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(unaff_si, 0x48));
    modify_list_1008_3f62(
        (param_1 & 0xffff0000 | (local_struct_1 + 0x10)),
        ppVar2 & 0xffff0000 | (ppVar2 + 0xe),
    );
    u_var1 = (local_struct_1 + 6) << 3;
    struct_a_00 = struct_a;
    process_struct_1000_179c(u_var1, struct_a);
    (local_struct_1 + 8) = u_var1;
    (local_struct_1 + 10) = struct_a_00;
    u_var1 = (local_struct_1 + 6) << 2;
    process_struct_1000_179c(u_var1, struct_a_00);
    (local_struct_1 + 0xc) = u_var1;
    (local_struct_1 + 0xe) = struct_a_00;
    pass1_1000_4906((local_struct_1 + 8), 0, (local_struct_1 + 6) << 3);
    pass1_1000_4906((local_struct_1 + 0xc), 0, (local_struct_1 + 6) << 2);
    return;
}

pub fn pass1_1020_8556(param_1: &mut Struct44) {
    let pu_var1: *mut u16;
    let mut u_var2: i32;
    let in_struct_1: &mut Struct44;
    let mut u_var3: u32;
    let local_bx_5: *mut Struct684;
    let mut i_var4: i32;
    let mut i_var5: i32;
    let mut u_var6: u16;
    let mut u_var7: u16;
    let mut local_c: u16;
    let mut local_6: u32;
    let local_struct_1: &mut Struct44;

    u_var6 = (param_1 >> 0x10);
    local_bx_5 = param_1;
    param_1.ptr_a_lo = 0x87aa;
    local_bx_5.field_0x2 = 0x1020;
    error_check_1000_17ce(local_bx_5.field_0x8);
    if ((&local_bx_5.field_0xe | local_bx_5.field_0xc) != 0) {
        local_c = 0;
        while (true) {
            pu_var1 = &local_bx_5.field_0x6;
            let pu_var1_val = unsafe { *pu_var1 };
            if (pu_var1_val == local_c || pu_var1_val < local_c) {
                break;
            }
            i_var5 = local_c * 4;
            u_var3 = &local_bx_5.field_0xc;
            u_var7 = (u_var3 >> 0x10);
            i_var4 = u_var3;
            if ((i_var4 + i_var5) != 0) {
                in_struct_1 = (i_var4 + i_var5);
                u_var2 = (i_var4 + i_var5 + 2);
                if ((u_var2 | in_struct_1) != 0) {
                    pass1_1008_5118((in_struct_1 & 0xffff | u_var2 << 0x10));
                    error_check_1000_17ce(in_struct_1);
                }
            }
            local_c = local_c + 1;
        }
        error_check_1000_17ce(&local_bx_5.field_0xc);
    }
    param_1.ptr_a_lo = ctx.s_1_1050_389a;
    local_bx_5.field_0x2 = &ctx.PTR_LOOP_1050_1008;
    return;
}

pub fn pass1_1020_85f6(param_1: Vec<u8>) {
    let pu_var1: *mut u16;
    let mut u_var2: i32;
    let in_struct_1: &mut Struct44;
    let mut u_var3: u32;
    let mut i_var4: i32;
    let local_bx_85: Vec<u8>;
    let mut u_var5: u16;
    let local_es_85: Vec<u8>;
    let mut local_8: u32;
    let mut local_4: u16;
    let temp_5f20445f9b: &mut Struct44;

    local_4 = 0;
    while (true) {
        local_es_85 = (param_1 >> 0x10);
        local_bx_85 = param_1;
        pu_var1 = (local_bx_85 + 6);
        let pu_var1_val = unsafe { *pu_var1 };
        if (pu_var1_val == local_4 || pu_var1_val < local_4) {
            break;
        }
        u_var3 = (local_bx_85 + 0xc);
        u_var5 = (u_var3 >> 0x10);
        i_var4 = u_var3;
        in_struct_1 = (i_var4 + local_4 * 4);
        u_var2 = (i_var4 + local_4 * 4 + 2);
        if ((u_var2 | in_struct_1) != 0) {
            pass1_1008_5118((in_struct_1 & 0xffff | u_var2 << 0x10));
            error_check_1000_17ce(in_struct_1);
        }
        u_var3 = (local_bx_85 + 0xc);
        (u_var3 + local_4 * 4) = 0;
        local_4 = local_4 + 1;
    }
    return;
}

pub fn pass1_1020_865a(param_1: *mut Struct681) {
    let pu_var1: *mut u16;
    let in_struct_1: &mut Struct44;
    let mut u_var2: u32;
    let local_bx_39: *mut Struct681;
    let local_bx_53: *mut Struct682;
    let mut i_var3: i32;
    let local_SI_50: *mut Struct683;
    let mut u_var4: u16;
    let mut u_var5: u16;
    let mut local_8: u32;
    let mut local_4: u16;
    let local_struct_1: &mut Struct44;
    let local_struct_1_1: &mut Struct44;

    local_4 = 0;
    while (true) {
        u_var4 = (param_1 >> 0x10);
        local_bx_39 = param_1;
        pu_var1 = &local_bx_39.field_0x6;
        let pu_var1_val = unsafe { *pu_var1 };
        if (pu_var1_val == local_4 || pu_var1_val < local_4) {
            break;
        }
        local_SI_50 = (local_4 * 4);
        u_var2 = local_bx_39.field_0xc;
        u_var5 = (u_var2 >> 0x10);
        local_bx_53 = u_var2;
        if ((local_bx_53 + local_SI_50) != 0) {
            pass1_1008_5236((local_bx_53 + local_SI_50));
            u_var2 = local_bx_39.field_0xc;
            u_var5 = (u_var2 >> 0x10);
            i_var3 = u_var2;
            in_struct_1 = (local_SI_50 + i_var3);
            local_struct_1 = (local_SI_50 + i_var3 + 2);
            if ((local_struct_1 | in_struct_1) != 0) {
                pass1_1008_5118((in_struct_1 & 0xffff | ZEXT24(local_struct_1) << 0x10));
                error_check_1000_17ce(in_struct_1);
            }
            u_var2 = local_bx_39.field_0xc;
            (u_var2 + local_4 * 4) = 0;
        }
        local_4 = local_4 + 1;
    }
    return;
}

pub fn pass1_1020_86d8(param_1: Vec<u8>) {
    let pu_var1: *mut u16;
    let local_bx_17: *mut Struct685;
    let mut u_var2: u16;
    let mut local_4: u16;
    let mut temp_5f84f21f47: u32;

    local_4 = 0;
    while (true) {
        u_var2 = (param_1 >> 0x10);
        pu_var1 = (param_1 + 6);
        let pu_var1_val = unsafe { *pu_var1 };
        if (pu_var1_val == local_4 || pu_var1_val < local_4) {
            break;
        }
        temp_5f84f21f47 = (param_1 + 0xc);
        u_var2 = (temp_5f84f21f47 >> 0x10);
        local_bx_17 = temp_5f84f21f47;
        if ((local_bx_17 + local_4 * 4) != 0) {
            pass1_1008_5236((local_bx_17 + local_4 * 4));
        }
        local_4 = local_4 + 1;
    }
    return;
}

pub fn pass1_1020_8712(
    param_1: *mut Struct393,
    param_2: *mut Struct393,
    param_3: u16,
    param_4: u32,
    param_5: u32,
) -> i32 {
    let mut u_var1: u16;
    let mut u_var2: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    pass1_1008_3f32(param_5, param_1 & 0xffff0000 | (param_1 + 0x10));
    u_var2 = process_struct_1008_4772(param_4);
    u_var1 = (u_var2 >> 0x10);
    pass1_1008_3e94(
        param_5,
        CONCAT22(param_3, &param_2.u16_0x2),
        CONCAT22(param_3, param_2),
    );
    param_2.u16_0x4 = (u_var2 + 4) + *_param_2;
    param_2.u16_0x6 = (u_var2 + 8) + param_2.u16_0x2;
    return;
}

pub fn pass1_1020_8784(param_1: &mut Struct44, param_2: u8) -> &mut Struct44 {
    pass1_1020_8556(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1020_87c2(param_1: *mut Struct393) {
    let mut u_var1: u32;
    let local_AX_25: *mut Struct393;
    let mut unaff_si: u16;
    let mut u_var2: i32;
    let mut unaff_ss: u16;
    let ppVar3: *mut Struct2111;
    let mut local_16: u32;
    let mut local_a: u16;
    let local_8: *mut Struct393;
    let mut local_4: u16;

    process_struct_1020_847a(param_1, 4);
    local_4 = 4;
    local_AX_25 = param_1;
    local_AX_25 = (&local_AX_25.field_0x14 + 2);
    local_8 = (param_1 & 0xffff0000 | ZEXT24(local_AX_25));
    while {
        zero_list_1008_3e38(local_8);
        local_8 = (local_8 & 0xffff0000 | (local_8 + 6));
        local_4 = local_4 - 1;
        local_4 != 0
    } {}
    u_var2 = (param_1 >> 0x10);
    &local_AX_25.field_0x2e = 0;
    zero_list_1008_3e38((param_1 & 0xffff0000 | &local_AX_25.field_0x32));
    &local_AX_25.field_0x38 = 0;
    param_1.field_0x0 = 0x8a84;
    local_AX_25.u16_0x2 = 0x1020;
    ppVar3 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(unaff_si, 0x29));
    &local_AX_25.field_0x2e = ppVar3;
    &local_AX_25.field_0x30 = (ppVar3 >> 0x10);
    local_a = 0;
    while {
        u_var1 = &local_AX_25.field_0x2e;
        pass1_1018_26d8(
            u_var1,
            (u_var1 >> 0x10),
            local_a,
            param_1 & 0xffff0000 | (&local_AX_25.field_0x14 + local_a * 6 + 2),
        );
        u_var1 = &local_AX_25.field_0x2e;
        pass1_1020_8712(
            (param_1 & 0xffff | u_var2 << 0x10),
            (&local_AX_25.field_0x8 + local_a * 8),
            &local_AX_25.field_0xa,
            (u_var1 + 0x2e + local_a * 4),
            param_1 & 0xffff0000 | (&local_AX_25.field_0x14 + local_a * 6 + 2),
        );
        local_a = local_a + 1;
        local_a < 4
    } {}
    u_var1 = &local_AX_25.field_0x2e;
    pass1_1018_2548(
        u_var1,
        (u_var1 >> 0x10),
        param_1 & 0xffff0000 | &local_AX_25.field_0x32,
    );
    u_var1 = &local_AX_25.field_0x2e;
    &local_AX_25.field_0x38 = (u_var1 + 0x6e);
    pass1_1020_8712(
        (param_1 & 0xffff | u_var2 << 0x10),
        &stack0xffee,
        unaff_ss,
        &local_AX_25.field_0x38,
        param_1 & 0xffff0000 | &local_AX_25.field_0x32,
    );
    return;
}

pub fn pass1_1020_8908(param_1: *mut Struct690, param_2: u32) {
    let in_struct_104_ptr: *mut Struct104;
    let mut u_var1: u32;
    let mut u_var2: i32;
    let local_DX_188: *mut Struct692;
    let paVar3: *mut Struct692;

    let mut i_var4: i32;
    let local_bx_151: *mut Struct690;
    let local_bx_294: *mut Struct688;
    let mut i_var5: i32;
    let local_SI_209: *mut Struct689;
    let mut u_var6: u16;
    let mut u_var7: u16;
    let mut u_var8: u32;
    let mut local_1c: u16;
    let mut local_1a: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u32;
    let local_4: *mut Struct687;
    let mut temp_7ffdca9fab4: i32;

    local_4 = 0x0;
    while (
        local_bx_151 = param_1,
        u_var6 = (param_1 >> 0x10),
        local_4 < 4,
    ) {
        if (local_bx_151.field_0x4 == 0) {
            u_var1 = local_bx_151.field_0xc;
            u_var6 = (u_var1 >> 0x10);
            i_var4 = u_var1;
            i_var5 = local_4 * 4;
            if (((i_var4 + i_var5 + 2) | (i_var4 + i_var5)) != 0) {
                pass1_1008_5236((i_var4 + i_var5));
            }
        } else {
            u_var1 = local_bx_151.field_0x2e;
            in_struct_104_ptr = (u_var1 + 0x2e + local_4 * 4);
            u_var8 = process_struct_1008_4772(in_struct_104_ptr);
            local_DX_188 = (u_var8 >> 0x10);
            temp_7ffdca9fab4 = u_var8;
            u_var1 = local_bx_151.field_0xc;
            local_SI_209 = (local_4 * 4);
            if ((&local_SI_209.field_0x0 + u_var1) == 0) {
                paVar3 = local_DX_188;
                u_var2 = temp_7ffdca9fab4;
                process_struct_1000_179c(0x14, local_DX_188);
                _local_1c = CONCAT22(paVar3, u_var2);
                if ((paVar3 | u_var2) == 0) {
                    u_var1 = local_bx_151.field_0xc;
                    (u_var1 + local_4 * 4) = 0;
                } else {
                    u_var2 = &local_bx_151.field_0x16 + local_4 * 6;
                    process_struct_1008_50c2(
                        _local_1c,
                        (temp_7ffdca9fab4 + 8),
                        (temp_7ffdca9fab4 + 4),
                        param_1 & 0xffff0000 | u_var2,
                        param_2,
                    );
                    u_var1 = local_bx_151.field_0xc;
                    u_var7 = (u_var1 >> 0x10);
                    local_bx_294 = u_var1;
                    (local_bx_294 + local_SI_209) = u_var2;
                    (local_bx_294 + local_SI_209 + 2) = ctx.dx_reg;
                }
                u_var1 = local_bx_151.field_0xc;
                pass1_1008_5134((u_var1 + local_4 * 4));
            }
            u_var1 = local_bx_151.field_0xc;
            pass1_1008_5236((u_var1 + local_4 * 4));
            pass1_1008_4480(
                param_2,
                (param_1 & 0xffff0000 | (&local_bx_151.field_0x16 + local_4 * 6)),
                in_struct_104_ptr,
            );
        }
        local_4 = &local_4.field_0x1;
    }
    if (local_bx_151.field_0x4 != 0) {
        pass1_1008_4480(
            param_2,
            (param_1 & 0xffff0000 | &local_bx_151.field_0x32),
            local_bx_151.field_0x38,
        );
    }
    return;
}

pub fn pass1_1020_8a5e(param_1: &mut Struct44, param_2: u8) -> &mut Struct44 {
    pass1_1020_8556(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1020_8bae(param_1: &mut Struct44) {
    param_1.ptr_a_lo = 0x8e92;
    (param_1 + 2) = 0x1020;
    pass1_1020_8556(param_1);
    return;
}

pub fn pass1_1020_6498(param_1: Vec<u8>, param_2: u16) {
    let mut u_var1: u32;
    let mut i_var2: i32;
    let mut u_var3: u16;

    u_var3 = (param_1 >> 0x10);
    if ((param_1 + 0x18 + param_2 * 4) != 0) {
        u_var1 = (param_1 + 0x18 + param_2 * 4);
        u_var3 = (u_var1 >> 0x10);
        i_var2 = u_var1;
        return CONCAT22((i_var2 + 10), (i_var2 + 8));
    }
    return 0;
}

pub fn pass1_1020_64d4(param_1: Vec<u8>, param_2: u16) {
    let mut local_es_5: u16;
    let mut temp_5ff6edc30e: u32;

    local_es_5 = (param_1 >> 0x10);
    if ((param_1 + 0x18 + param_2 * 4) != 0) {
        temp_5ff6edc30e = (param_1 + 0x18 + param_2 * 4);
        return (temp_5ff6edc30e + 4);
    }
    return 0;
}

pub fn pass1_1020_61c4(param_1: u16, param_2: u16, param_1_00: u32, param_2_00: *mut u16) {
    let mut u_var1: u16;
    let ctx.ax_reg: *mut Struct667;
    let mut local_DX_11: u16;
    let ppVar2: *mut Struct2111;
    let local_string_1: String;
    let mut local_6: u16;
    let mut local_4: u16;
    let mut temp_5f50cbac33: u32;

    ppVar2 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(local_string_1, 0x2f));
    u_var1 = (ppVar2 >> 0x10);
    pass1_1030_8308(
        ctx._g_bool_1050_5748,
        (ctx._g_bool_1050_5748 >> 0x10),
        param_1_00,
        param_2_00,
        (ppVar2 + 0x20),
    );
    unsafe { *param_2_00 = (ppVar2 + 0x1e) };
    return;
}

pub fn pass1_1020_5d56(param_1: *mut u32, param_2: u32) -> bool {
    let pp_var1: fn();
    let mut unaff_ss: u16;
    let mut u_var2: u16;
    let mut u_var3: u16;
    let mut local_12: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    local_6 = (param_2 + 0x2e);
    u_var2 = param_1;
    u_var3 = (param_1 >> 0x10);
    if (local_6 == 0x47) {
        pass1_1020_61c4(
            u_var2,
            u_var3,
            CONCAT22(unaff_ss, &local_c),
            CONCAT22(unaff_ss, &local_a),
        );
        if (local_a == 0) {}
        // goto LAB_1020_5d8b;
        if (local_c <= local_a) {
            return 1;
        }
    } else {
        if (local_6 != 0x6a) {
            return 0;
        }
        pass1_1020_61c4(
            u_var2,
            u_var3,
            CONCAT22(unaff_ss, &local_e),
            CONCAT22(unaff_ss, &local_12),
        );
        if (local_e <= local_12) {
            // LAB_1020_5d8b:
            unsafe {
                pp_var1 = (*param_1 + 0x40);
                (**pp_var1)();
            }
            return 1;
        }
    }
    pass1_1038_af40(ctx._g_Struct112_a, *(u_var2 + 8), 9);
    return 1;
}

pub fn call_draw_fn_1020_3bd6(in_struct_1: *mut Struct657) {
    let mut u_var1: u32;
    let mut u_var2: u16;
    let local_struct_1: *mut Struct657;
    let local_struct_1_hi: *mut Struct657;
    let pu_var3: Vec<u8>;

    local_struct_1_hi = (in_struct_1 >> 0x10);
    local_struct_1 = in_struct_1;
    draw_1020_3fa0(local_struct_1.field_0xf6);
    if (local_struct_1.field_0x100 == 0) {
        local_struct_1.field_0x100 = 1;
        u_var1 = local_struct_1.field_0xfa;
        if ((u_var1 + 0x56) == 0) {
            u_var2 = 5;
        } else {
            u_var2 = 8;
        }
        pu_var3 = pass1_1038_af40(ctx._g_Struct112_a, local_struct_1.field_0x8, u_var2);
        local_struct_1.field_0x10e = pu_var3;
        local_struct_1.field_0x110 = (pu_var3 >> 0x10);
    }
    return;
}

pub fn pass1_1020_3c32(param_1: *mut Struct658, param_2: u16, uparam_2_00: i32) {
    let mut u_var1: u32;
    let mut c_var2: u8;
    let mut u_var3: u16;

    if (param_2_00 == 0xf5) {
        u_var3 = 1;
        // LAB_1020_3c52:
        u_var1 = param_1.field_0xfa;
        pass1_1018_1b02(u_var1, (u_var1 >> 0x10), u_var3);
        return;
    }
    if ((param_2_00 < 0xf6) && (c_var2 = param_2_00, c_var2 != '\0')) {
        if (c_var2 == 0x1 || c_var2 == 0x2) {
            return;
        }
        if (c_var2 == -0xc) {
            u_var3 = 0;
            // goto LAB_1020_3c52;
        }
    }
    pass1_1020_3c32(param_1, param_2, param_2_00);
    return;
}

pub fn pass1_1020_3540(param_1: u16, param_2: u16, param_1_00: i32, param_2_00: u32) {
    let local_struct_2: *mut Struct655;
    let struct_a: *mut Struct199;
    let paVar1: *mut Struct199;
    let local_struct_1: *mut Struct654;
    let mut unaff_ss: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    pass1_1008_3e94(
        param_2_00,
        CONCAT22(unaff_ss, &local_6),
        CONCAT22(unaff_ss, &local_4),
    );
    if (param_1_00 == 0) {
        local_c = 3;
        local_a._0_2_ = 0x42a6;
    } else {
        if (param_1_00 == 1) {
            local_c = 4;
            local_a._0_2_ = (s_SITEICON_1050_428d + 9);
        } else {
            if (param_1_00 != 2) {
                return;
            }
            local_c = 4;
            local_a._0_2_ = 0x42b2;
        }
    }
    local_struct_2 = (local_c << 2);
    paVar1 = struct_a;
    process_struct_1000_179c(local_struct_2, struct_a);
    local_12 = 0;
    while (local_12 < local_c) {
        local_struct_1 = (local_12 * 4);
        (local_struct_1 + local_struct_2) = (local_struct_1 + local_a) + local_4;
        (local_struct_2 + local_struct_1 + 2) = (local_struct_1 + local_a + 2) + local_6;
        local_12 = local_12 + 1;
    }
    return;
}

pub fn pass1_1020_2a94(param_1: &mut Vec<u8>, param_2: u32) {
    pass1_1018_1662((param_1 + 0xf2), param_2);
    return;
}

pub fn pass1_1020_2936(param_1: u16, param_2: u32) -> u8 {
    let mut u_var1: u16;

    u_var1 = return_1_1020_79ae();
    return u_var1;
}

pub fn pass1_1020_294a(in_struct_1: *mut Struct651, param_2: u32, param_3: u16) {
    let mut u_var1: u16;
    let local_struct_1: *mut Struct651;
    let mut unaff_bp: u16;
    let local_struct_1_hi: *mut Struct651;
    let ppVar2: *mut Struct2111;

    local_struct_1_hi = (in_struct_1 >> 0x10);
    local_struct_1 = in_struct_1;
    local_struct_1.field_0xfc = param_3;
    ppVar2 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(unaff_bp, param_3));
    u_var1 = (ppVar2 >> 0x10);
    local_struct_1.field_0xf2 = ppVar2;
    &local_struct_1.field_0xf4 = u_var1;
    local_struct_1.field_0xe0 = local_struct_1.field_0xf2;
    local_struct_1.field_0xe2 = u_var1;
    pass1_1018_0902(&local_struct_1.field_0xf2, param_2);
    return;
}

pub fn process_struct_1020_26e6(param_1: *mut Struct376, param_2: u8) -> *mut Struct376 {
    process_struct_1020_2594(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}
