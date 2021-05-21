use std::intrinsics::offset;

use crate::{
    defines::{AppContext, Struct150},
    util::CONCAT22,
};
use crate::app_context::AppContext;
use crate::bad_funcs::bad1::halt_baddata;
use crate::err_funcs::error_check_1000_17ce;
use crate::other_funcs::{switch_stmt_1008_ab80, zero_list_1008_3e38};
use crate::pass::pass11_funcs::pass1_1008_cfa0;
use crate::pass::pass12_funcs::{pass1_1008_b9ce, pass1_1008_c6ae, pass1_1008_c6fa};
use crate::pass::pass13_funcs::{pass1_1008_944e, pass1_1008_b0bc};
use crate::pass::pass14_funcs::{pass1_1008_3e76, pass1_1008_5784, pass1_1008_5b12, pass1_1008_6604, pass1_fn_1008_60e8, pass1_fn_1008_612e};
use crate::pass::pass15_funcs::{pass1_1020_ba3e, pass1_1020_ba7e, pass1_1020_bae6, pass1_1020_bb8a};
use crate::pass::pass17_funcs::{pass1_1030_1cd8, pass1_1030_1d7c, pass1_1030_2690, pass1_1030_627e, pass1_1030_73a8, pass1_1030_7bee, pass1_1030_7c28, pass1_1030_7d1c, pass1_1030_7ddc, pass1_1030_809c, pass1_1030_8326, pass1_1030_8344};
use crate::pass::pass20_funcs::{pass1_1010_c3c2, pass1_1010_dd5e};
use crate::pass::pass2_funcs::pass1_1010_e964;
use crate::pass::pass4_funcs::{pass1_1028_bb24, pass1_1028_d1dc, pass1_1028_dc52, pass1_1028_e1ec, pass1_1028_e2e0, pass1_1028_e4ec};
use crate::pass::pass5_funcs::pass1_1030_b9b2;
use crate::pass::pass6_funcs::{pass1_1038_4d28, pass1_1038_4e78, pass1_1038_5464, pass1_1038_56d6};
use crate::pass::pass7_funcs::{pass1_1018_3e8c, pass1_1018_427c, pass1_1018_435e, pass1_1018_47c8, pass1_1018_4808};
use crate::pass::pass8_funcs::{pass1_1008_e852, pass1_1008_e8cc, pass1_1010_089e, pass1_1010_60a0, pass1_1010_878c};
use crate::pass::pass_funcs::{pass1_1000_3d7a, pass1_1000_4906, pass1_fn_1000_25a8, pass1_fn_1000_2913, pass1_fn_1000_2b3c, pass1_fn_1000_2b5c, pass1_fn_1000_2f48, pass1_fn_1000_2fa4, pass1_fn_1000_3024, pass1_fn_1000_30b4, pass1_fn_1000_3e2c, pass1_fn_1000_3e82, pass1_fn_1000_3f5c, pass1_fn_1000_48a8, pass1_fn_1000_52be};
use crate::prog_structs::prog_structs_10::Struct73;
use crate::prog_structs::prog_structs_16::Struct493;
use crate::prog_structs::prog_structs_18::Struct566;
use crate::prog_structs::prog_structs_19::Struct500;
use crate::prog_structs::prog_structs_1::Struct393;
use crate::prog_structs::prog_structs_20::Struct965;
use crate::prog_structs::prog_structs_24::{pass1_struct_3, Struct2111, Struct432, Struct894};
use crate::prog_structs::prog_structs_26::{Struct1123, Struct883, Struct891};
use crate::prog_structs::prog_structs_27::{pass1_struct_2, Struct298};
use crate::prog_structs::prog_structs_28::{Struct1016, Struct1053, Struct346, Struct912, Struct913, Struct915};
use crate::prog_structs::prog_structs_29::{Struct1035, Struct166};
use crate::prog_structs::prog_structs_2::Struct199;
use crate::prog_structs::prog_structs_30::Struct3;
use crate::prog_structs::prog_structs_31::Struct2;
use crate::prog_structs::prog_structs_3::Struct446;
use crate::prog_structs::prog_structs_5::Struct150;
use crate::prog_structs::prog_structs_7::Struct613;
use crate::struct_ops1::{process_struct_1000_179c, process_struct_1000_2cb0, process_struct_1008_4544, process_struct_1008_574a, process_struct_1008_cbc4, process_struct_1008_cda2, process_struct_1008_d3ae, process_struct_1040_b0bc};
use crate::sys_ops::dos_ops::{dos3_call_1000_2bb6, dos3_call_1000_35fe, dos3_call_1000_514e, dos3_call_1000_51aa};
use crate::sys_ops::private_profile_str::write_private_profile_str_1010_5b10;
use crate::util::{CARRY1, CARRY2, CONCAT11, CONCAT12, CONCAT13, CONCAT21, CONCAT31, SBORROW2, SUB42, ZEXT24};
use crate::winapi_funcs::{FatalAppExit16, FatalExit, LoadString16, MessageBox16, swi};

pub unsafe fn process_string_1000_28dc(ctx: &mut AppContext, in_string_1: &String) -> String {
    let mut i32_1: i32;
    let mut pch_2: String;
    let out_string_1: String;
    let string_1: String;
    let string_2: String;
    let mut string_4: String;

    pch_2 = (ctx.s___NMSG___1050_63f6[8..].clone());
    loop {
        string_2 = pch_2.clone();
        pch_2 = pch_2[2..].clone();
        string_1 = string_2[0];
        out_string_1 = pch_2;
        // TODO
        // if (string_1 == in_string_1) || (out_string_1 = string_1 + 1, out_string_1 == 0x0) {
        //     return out_string_1;
        // }
        i32_1 = -1;
        while {
            if i32_1 == 0 {
                break;
            }
            i32_1 = i32_1 + -1;
            string_4 = pch_2.clone();
            pch_2 = pch_2[1..].clone();
            *string_4 != '\0'
        } {}
    }
}

pub unsafe fn str_fn_1000_28e0(ctx: &mut AppContext, param_1: i32, param_2: u16) -> String {
    let string_4: String;
    let mut i_var2: i32;
    let string_1: String;
    let string_2: String;
    let string_3: String;

    string_1 = ctx.s___NMSG___1050_63f6[8..].clone();
    loop {
        string_4 = string_1.clone();
        string_1 = string_1[2..].clone();
        i_var2 = string_4[0];
        string_2 = string_1;
        // TODO
        // if (i_var2 == param_1) || (string_2 = (i_var2 + 1), string_2 == 0x0) {
        //     return string_2;
        // }
        i_var2 = -1;
        while {
            if i_var2 == 0 {
                break;
            }
            i_var2 = i_var2 + -1;
            string_4 = string_1.clone();
            string_1 = string_1[1..].clone();
            string_4[0] != '\0'
        } {}
    }
}

pub unsafe fn process_string_1000_2917(ctx: &mut AppContext, param_1: &String, param_2: u16) {
    let mut string2: String;
    let mut iVar1: i32;
    let mut string1: String;
    let mut string2_res: String;

    if &ctx.PTR_LOOP_1050_61ec != 0 {
        string2_res = process_string_1000_28dc(ctx, param_1);
        if string2_res.is_some() {
            let string_2 = string2_res.unwrap();
            iVar1 = -1;
            while {
                if iVar1 == 0 {
                    break;
                }
                iVar1 = iVar1 + -1;
                string1 = string2.clone();
                string2 = string2[1..].clone();
                string1[0] != '\0'
            } {}
            process_string_1000_55b1(ctx);
        }
    }
    return;
}

// WARNING: Variable defined which should be unmapped: local_12

pub unsafe fn process_string_1000_2a00(ctx: &mut AppContext, param_1: *mut Struct150) -> u16 {
    let mut bVar1: bool;
    let mut i_var2: i32;
    let mut u_var3: u16;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u8;
    let uStack15: u8;
    let mut local_e: [u8; 8];
    let mut local_6: u16;
    let mut local_4: u16;
    let mut iStack2: i32;

    iStack2 = ctx.bp_reg + 1;
    // TODO
    //local_4 = &ctx.g_alloc_addr_1050_1050;
    u_var3 = 0xffff;
    if (param_1.field_0xa & 0x40) != 0 {
        param_1.field_0xa = 0;
        return 0xffff;
    }
    if (*&param_1.field_0xa & 0x83) == 0 {}
    // goto LAB_1000_2af2;
    u_var3 = pass1_fn_1000_2fa4(param_1);
    local_6 = param_1.field_0xf4;
    process_struct_1000_2cb0(param_1);
    if u16_1050_5f8a < param_1.field_0xb {
        i_var2 = process_string_1000_55b1();
        if i_var2 < 0 {}
        // goto LAB_1000_2a6a;
        // LAB_1000_2a82:
        bVar1 = false;
    } else {
        i_var2 = dos3_call_1000_35fe();
        if -1 < i_var2 {}
        // goto LAB_1000_2a82;
        // LAB_1000_2a6a:
        bVar1 = true;
    }
    if !bVar1 {
        if local_6 == 0 {}
        // goto LAB_1000_2af2;
        copy_string_1000_3d3e(CONCAT22(ctx.stack_seg_reg, local_10), ctx.s___1050_5fea);
        local_14 = local_e;
        if local_10 == '\\' {
            local_14 = &uStack15;
        } else {
            process_string_1000_3cea(CONCAT22(ctx.stack_seg_reg, local_10), ctx.s___1050_5fec);
        }
        pass1_fn_1000_3e82(local_6, local_14, unaff_ss, 10);
        i_var2 = dos3_call_1000_514e(&local_10, unaff_ss);
        if i_var2 == 0 {}
        // goto LAB_1000_2af2;
    }
    u_var3 = 0xffff;
    // LAB_1000_2af2:
    param_1.field_0xa = 0;
    return u_var3;
}

pub unsafe fn process_string_1000_2ba0(ctx: &mut AppContext) {
    pass1_fn_1000_3024();
    if (ctx.PTR_LOOP_1050_5fc9 != '\0') {
        pass1_fn_1000_3f5c();
    }
    return;
}

pub fn process_string_1000_3cea(string_a: String, string_b: String) {
    let pu_var1: *mut u16;
    let pc_var2: String;
    let pu_var3: *mut u16;
    let mut i_var4: i32;
    let mut u_var5: i32;
    let mut u_var6: i32;
    let local_str_1_1: String;
    let pcVar7: String;
    let puVar8: *mut u16;
    let pu_var9: *mut u16;
    let local_str_1: String;
    let mut local_es_21: u16;
    let mut bVar10: bool;
    let temp_1087f15098c83: String;
    let temp_87f9662d6e1: *mut u16;

    local_str_1 = (string_a >> 0x10);
    local_str_1_1 = string_a;
    bVar10 = true;
    i_var4 = -1;
    while {
        if (i_var4 == 0) {
            break;
        }
        i_var4 = i_var4 + -1;
        pu_var1 = local_str_1_1;
        local_str_1_1 = (local_str_1_1 + 1);
        unsafe {
            bVar10 = *pu_var1 == '\0';
        }
        !bVar10
    } {}
    pu_var9 = (local_str_1_1 + -1);
    local_es_21 = (string_b >> 0x10);
    pcVar7 = string_b;
    u_var5 = 0xffff;
    while {
        if (u_var5 == 0) {
            break;
        }
        u_var5 = u_var5 - 1;
        pc_var2 = pcVar7;
        pcVar7 = pcVar7 + 1;
        unsafe {
            bVar10 = *pc_var2 == '\0';
        }
        !bVar10
    } {}
    //u_var5 = ~u_var5;
    if (!bVar10) {
        pcVar7 = pcVar7 + -u_var5;
        u_var5 = u_var5 + 1;
    }
    puVar8 = (pcVar7 + -u_var5);
    if (u_var5 == 0) {
        pu_var1 = puVar8;
        puVar8 = puVar8 + 1;
        unsafe {
            *pu_var9 = *pu_var1;
        }
        u_var5 = 0xfffe;
        pu_var9 = (local_str_1_1 + 1);
    } else {
        if ((puVar8 & 1) != 0) {
            pu_var1 = puVar8;
            puVar8 = (puVar8 + 1);
            unsafe {
                *pu_var9 = *pu_var1;
            }
            u_var5 = u_var5 - 1;
            pu_var9 = local_str_1_1;
        }
    }
    u_var6 = u_var5 >> 1;
    while (u_var6 != 0) {
        u_var6 = u_var6 - 1;
        temp_87f9662d6e1 = pu_var9;
        pu_var9 = pu_var9 + 1;
        pu_var1 = puVar8;
        puVar8 = puVar8 + 1;
        unsafe {
            *temp_87f9662d6e1 = *pu_var1;
        }
    }
    u_var5 = ((u_var5 & 1) != 0);
    while u_var5 != 0 {
        u_var5 = u_var5 - 1;
        pu_var3 = pu_var9;
        pu_var9 = (pu_var9 + 1);
        pu_var1 = puVar8;
        puVar8 = (puVar8 + 1);
        unsafe {
            *pu_var3 = *pu_var1;
        }
    }
    return;
}

pub unsafe fn copy_string_1000_3d3e(
    in_string_1: &mut String,
    in_string_2: &String,
) -> u16 {
    let string_var1: String;
    let struct_var2: Struct166;
    let mut u_var3: i32;
    let mut u_var4: i32;
    let string_var4: String;
    let string_var5: String;
    let mut b_var6: bool;
    // temp_1087faeaca0cc: *mut Vec<u8>;
    let string_var7: String;

    // local_DS_8 = (in_string_2 >> 0x10);
    // string_var4 = in_string_2;
    b_var6 = true;
    u_var3 = 0xffff;
    // string_var5 = string_var4;
    while {
        if u_var3 == 0 {
            break;
        }
        u_var3 = u_var3 - 1;
        string_var7 = in_string_2.clone();
        string_var5 = in_string_2[1..].clone();
        unsafe {
            b_var6 = *string_var7 == '\0';
        }
        !b_var6
    } {}
    u_var3 = !u_var3;
    ctx.es_reg = (in_string_1 >> 0x10);
    local_DI_26 = in_string_1;
    if b_var6 {
        if (in_string_1 & 1) != 0 {
            local_DI_26 = &local_DI_26.field_0x1;
            string_var4 = in_string_2[1..].clone();
            in_string_1[0] = in_string_2[0];
            u_var3 = u_var3 - 1;
        }
    } else {
        local_DI_26 = &local_DI_26.field_0x2;
        string_var4 = string_var4[2..].clone();
        in_string_1 = in_string_2.clone();
        u_var3 = u_var3 - 1;
    }
    u_var4 = u_var3 >> 1;
    while u_var4 != 0 {
        u_var4 = u_var4 - 1;
        struct_var2 = local_DI_26;
        local_DI_26 = &local_DI_26.field_0x2;
        string_var1 = string_var4.clone();
        string_var4 = string_var4[2..].clone();
        unsafe {
            struct_var2 = *string_var1;
        }
    }
    // u_var3 = ((u_var3 & 1) != 0);
    while u_var3 != 0 {
        u_var3 = u_var3 - 1;
        struct_var2 = local_DI_26;
        local_DI_26 = &local_DI_26.field_0x1;
        string_var1 = string_var4;
        string_var4 = (string_var4 + 1);
        unsafe {
            *struct_var2 = *string_var1;
        }
    }
    return ctx.es_reg;
}

pub fn get_string_index_1000_3da4(in_string_1: &mut String) -> u16 {
    let mut string3 = in_string_1.clone();
    let mut string1: String = String::new();
    let mut bool2 = true;
    let mut u_var1: u16 = 0xffff;
    while {
        if u_var1 == 0 {
            break;
        }
        u_var1 = u_var1 - 1;
        string1 = string3.clone();
        string3 = string3[1..].clone();
        unsafe {
            bool2 = string1[0] == '\0';
        }
        !bool2
    } {}
    u_var1 = !u_var1;
    if bool2 {
        u_var1 = u_var1 - 1;
    }
    return u_var1;
}

pub fn process_string_1000_3dbe(
    in_string_1: String,
    in_string_2: String,
    param_3: u16,
) -> Vec<u8> {
    let pu_var1: Vec<u8>;
    let pu_var2: Vec<u8>;
    let mut string2: u16;
    let mut string3: u16;
    let mut u_var3: u16;
    let mut string1: u32;
    let char1: u8;

    u_var3 = (in_string_1 >> 0x10);
    string2 = in_string_2;
    string3 = in_string_1;
    if (param_3 != 0) {
        while {
            string1 = string2;
            string2 = string2 + 1;
            char1 = *string1;
            if (char1 == '\0') {
                break;
            }
            pu_var1 = string3;
            string3 = string3 + 1;
            unsafe {
                *pu_var1 = char1;
            }
            *param_3 = *param_3 - 1;
            *param_3 != 0
        } {}
        while *param_3 != 0 {
            *param_3 = *param_3 - 1;
            pu_var2 = string3;
            string3 = string3 + 1;
            unsafe {
                *pu_var2 = 0;
            }
        }
    }
    return in_string_1;
}

pub fn process_string_1000_3de8(
    in_string_1: String,
    param_2: String,
    param_3: u16,
) -> u8 {
    let pu8_var1: Vec<u8>;
    let pc_var2: String;
    let mut u8_var3: u8;
    let mut u_var4: u16;
    let mut i_var5: i32;
    let mut string_3: u16;
    let mut string_4: u16;
    let mut u_var6: u16;
    let mut u_var7: u16;
    let mut u_var8: u16;
    let bool1: u8;
    let mut string_1: String = String::new();
    let mut string_2: String = String::new();

    if param_3 != 0 {
        // u_var7 = (in_string_1 >> 0x10);
        string_4 = in_string_1.clone();
        u_var4 = param_3.clone();
        u_var6 = string_4.clone();
        while {
            if u_var4 == 0 {
                break;
            }
            u_var4 = u_var4 - 1;
            string_1 = u_var6;
            u_var6 = u_var6 + 1;
            unsafe { *string_1 != '\0' }
        } {}
        i_var5 = param_3 - u_var4;
        u_var8 = (param_2 >> 0x10);
        string_3 = param_2;
        while {
            if (i_var5 == 0) {
                break;
            }
            i_var5 = i_var5 + -1;
            string_2 = string_4;
            string_4 = string_4 + 1;
            pc_var2 = string_3;
            string_3 = string_3 + 1;
            unsafe { *pc_var2 == *string_2 }
        } {}
        u8_var3 = *(string_3 + -1);
        param_3 = 0;
        pu8_var1 = (string_4 - 1);
        unsafe {
            bool1 = u8_var3 == *pu8_var1;
        }
        let pb_var1_val = unsafe { *pu8_var1 };
        if (u8_var3 < pb_var1_val || bool1) {
            if (bool1) {}
            // goto LAB_1000_3e1a;
            param_3 = 0xfffe;
        }
        //param_3 = ~param_3;
    }
    // LAB_1000_3e1a:
    return param_3;
}

pub fn process_string_1000_3ec0(ctx: &mut AppContext, param_1: i32, param_2: &String) -> u8 {
    let u_var1: u8;
    let mut str_index: u16;
    let mut u_var2: i32;
    // let extraout_AH: u8;
    let mut i_var3: i32;
    let mut u_var4: u16;
    // let mut local_8: u16;
    let mut local_6: u16;

    let _local_8 = CONCAT22(ctx.PTR_LOOP_1050_5fc0, PTR_LOOP_1050_5fbe);
    if ((PTR_LOOP_1050_5fc0 | PTR_LOOP_1050_5fbe) != 0) && ((param_2 | param_1) != 0) {
        str_index = get_string_index_1000_3da4(CONCAT22(param_2, param_1));
        loop {
            // u_var4 = (_local_8 >> 0x10);
            // i_var3 = _local_8;
            if ((_local_8 + 2) | _local_8) == 0 {
                break;
            }
            u_var2 = get_string_index_1000_3da4(CONCAT22((i_var3 + 2), _local_8));
            if ((str_index < u_var2) && (*(*_local_8 + str_index) == '='))
                && (
                    u_var1 = process_string_1000_3de8(
                        CONCAT22((i_var3 + 2), _local_8),
                        CONCAT22(param_2, param_1),
                        str_index,
                    ),
                    CONCAT11(extraout_AH, u_var1) == 0,
                )
            {
                return _local_8 + str_index + 0x1;
            }
            _local_8 = (_local_8 & 0xffff0000 | (i_var3 + 4));
        }
    }
    return 0;
}

pub fn string_fn_1000_3f9c(
    ctx: &mut AppContext,
    param_1: &String,
    param_3: &String,
    param_4: &String,
    param_5: &String,
) -> u8 {
    let iStack2 = ctx.bp_reg + 1;
    let local_4 = &ctx.g_alloc_addr_1050_1050;
    ctx.PTR_LOOP_1050_68b2._0_1_ = 0x42;
    ctx.PTR_LOOP_1050_68ae = param_1;
    ctx.PTR_LOOP_1050_68b0 = param_2;
    ctx._PTR_LOOP_1050_68a8 = param_1;
    ctx.PTR_LOOP_1050_68ac = 0x7fff;
    let ppcStack16 = param_5;
    let u_var2 = pass1_fn_1000_30b4(
        ctx,
        &ctx.PTR_LOOP_1050_68a8,
        &ctx.g_alloc_addr_1050_1050,
        &param_3,
    );
    let mut string_1 = ctx._PTR_LOOP_1050_68a8.clone();
    // ctx.PTR_LOOP_1050_68ac = ctx.PTR_LOOP_1050_68ac + -1;
    if ctx.PTR_LOOP_1050_68ac < 0 {
        var_3 = ctx.PTR_LOOP_1050_68a8.clone();
        dos3_call_1000_2bb6(0, None);
    } else {
        // ctx._PTR_LOOP_1050_68a8 = (ctx._PTR_LOOP_1050_68a8 & 0xffff0000 | ctx.PTR_LOOP_1050_68a8 + 1);
        string_1[0] = 0;
    }
    return var_2;
}

pub fn process_string_1000_440c(param_1: u16) {
    let pc_var1: String;
    let mut c_var2: u8;
    let u_var3: u8;
    let i_var4: u16;
    let extraout_AH_00: u8;
    let extraout_AH_01: u8;
    let mut u_var5: i32;
    let in_i16_2: Vec<u8>;
    let mut in_i16_2_00: i32;
    let mut b_var6: bool;
    let mut u_var7: u16;
    let mut u_var8: u16;

    let c_var2 = process_string_1000_3ec0(0x61ca, &ctx.g_alloc_addr_1050_1050);
    let u_var5 = CONCAT11(ctx.ah_reg, c_var2);
    let mut _local_8 = CONCAT22(param_1, u_var5);
    if ((param_1 | u_var5) != 0) && (*_local_8 != '\0') {
        process_string_1000_3dbe(
            CONCAT22(
                ctx.PTR_LOOP_1050_61de,
                ctx.PTR_PTR_LAB_1050_534f_1_1050_61d4_1050_61dc,
            ),
            CONCAT22(param_1, u_var5),
            3,
        );
        _local_8 = CONCAT22(param_1, u_var5 + 3);
        c_var2 = *_local_8;
        if c_var2 == '-' as u8 {
            _local_8 = CONCAT22(param_1, u_var5 + 4);
        }
        in_i16_2 = 0x0;
        u_var8 = 0;
        u_var7 = 0xe10;

        let var9 = (_local_8 & 0xffff | param_1 << 0x10);
        let i_var4 = pass1_fn_1000_3e2c(var9);
        u_var3 = pass1_fn_1000_52be(i_var4, in_i16_2, u_var7, u_var8);
        u16_1050_61ce = CONCAT11(extraout_AH_00, u_var3);
        while (
            pc_var1 = _local_8,
            *_local_8 == '+' || ('/' < *_local_8 && (*_local_8 < ':')),
        ) {
            _local_8 = (_local_8 & 0xffff0000 | (local_8 + 1));
        }
        PTR_LOOP_1050_61d0 = in_i16_2;
        if *_local_8 == ':' {
            in_i16_2_00 = 0;
            u_var8 = 0;
            u_var7 = 0x3c;
            _local_8 = (_local_8 & 0xffff0000 | (local_8 + 1));
            i_var4 = pass1_fn_1000_3e2c((pc_var1 & 0xffff0000 | (local_8 + 1)));
            u_var3 = pass1_fn_1000_52be(i_var4, in_i16_2_00, u_var7, u_var8);
            b_var6 = CARRY2(u16_1050_61ce, CONCAT11(extraout_AH_01, u_var3));
            u16_1050_61ce = u16_1050_61ce + CONCAT11(extraout_AH_01, u_var3);
            PTR_LOOP_1050_61d0 = PTR_LOOP_1050_61d0 + b_var6 + in_i16_2_00;
            while ((pc_var1 = _local_8, '/' < *_local_8 && (*_local_8 < ':'))) {
                _local_8 = (_local_8 & 0xffff0000 | (local_8 + 1));
            }
            if (*_local_8 == ':') {
                _local_8 = (_local_8 & 0xffff0000 | (local_8 + 1));
                u_var5 = pass1_fn_1000_3e2c((pc_var1 & 0xffff0000 | (local_8 + 1)));
                b_var6 = CARRY2(u16_1050_61ce, u_var5);
                u16_1050_61ce = u16_1050_61ce + u_var5;
                PTR_LOOP_1050_61d0 = PTR_LOOP_1050_61d0 + b_var6 + in_i16_2_00;
                while ('/' < *_local_8 && (*_local_8 < ':')) {
                    _local_8 = (_local_8 & 0xffff0000 | (local_8 + 1));
                }
            }
        }
        if (c_var2 == '-') {
            b_var6 = u16_1050_61ce != 0;
            u16_1050_61ce = -u16_1050_61ce;
            PTR_LOOP_1050_61d0 = -(PTR_LOOP_1050_61d0 + b_var6);
        }
        u16_1050_61d2 = SEXT12(*_local_8);
        if (u16_1050_61d2 == 0) {
            *_PTR_PTR_1050_61e0 = '\0';
        } else {
            process_string_1000_3dbe(_PTR_PTR_1050_61e0, _local_8, 3);
        }
    }
    return;
}

pub fn process_string_1000_472c(in_string_1: String, in_char_2: u8) -> Vec<u8> {
    let pu_var1: Vec<u8>;
    let mut u_var2: i32;
    let mut string2: u16;
    let mut string3: u16;
    let mut local_es_4: u16;
    let bool1: u8;
    let mut string1: u32;

    local_es_4 = (in_string_1 >> 0x10);
    string2 = in_string_1;
    bool1 = 0x1;
    u_var2 = 0xffff;
    string3 = string2;
    while {
        if (u_var2 == 0) {
            break;
        }
        u_var2 = u_var2 - 1;
        string1 = string3;
        string3 = string3 + 1;
        unsafe {
            bool1 = *string1 == '\0';
        }
        !bool1
    } {}
    u_var2 = !u_var2;
    while {
        if (u_var2 == 0) {
            break;
        }
        u_var2 = u_var2 - 1;
        pu_var1 = string2;
        string2 = string2 + 1;
        unsafe {
            bool1 = in_char_2 == *pu_var1;
        }
        !bool1
    } {}
    if (!bool1) {
        if (in_char_2 != '\0') {
            return 0x0;
        }
        string2 = string2 + 1;
    }
    return (string2 + -1);
}

pub fn process_string_1000_475e(
    in_string_1: String,
    in_string_2: String,
) -> i32 {
    let u_var1: u8;
    let char_2: u8;
    let byte_1: u8;
    let string_1: String;
    let mut string_3: u16;
    let mut string_4: u16;
    let mut string_2: u32;
    let char_1: u8;

    string_4 = in_string_2;
    string_3 = in_string_1;
    string_1 = (s_You_may_not_run_a_turn__The_game_1050_00df + 0x20);
    while {
        while {
            char_2 = string_1;
            if (char_2 == '\0') {}
            // goto LAB_1000_479d;
            string_2 = string_4;
            string_4 = string_4 + 1;
            u_var1 = *string_2;
            char_1 = *string_3;
            string_1 = CONCAT11(char_1, u_var1);
            string_3 = string_3 + 1;
            char_1 == u_var1
        } {}
        byte_1 = u_var1 + 0xbf + (-((u_var1 + 0xbf) < 0x1a) & 0x20) + 'A';
        string_1._0_1_ = char_1 + 0xbf + (-((char_1 + 0xbf) < 0x1a) & 0x20) + 0x41;
        string_1 = CONCAT11(byte_1, string_1);
        string_1 == byte_1
    } {}
    char_2 = (string_1 < byte_1) * -2 + 0x1;
    // LAB_1000_479d:
    return char_2;
}

pub fn process_string_1000_4d58(
    in_string_1: String,
    in_string_2: String,
    param_3: Vec<u8>,
    param_4: u32,
    param_5: Vec<u8>,
) {
    let mut iVar1: i32;
    let mut u_var2: u16;
    let mut u_var3: u16;
    let mut u_var4: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;

    local_a = 0;
    local_c = 0;
    u_var3 = (in_string_1 >> 0x10);
    iVar1 = in_string_1;
    unsafe {
        if ((*in_string_1 == '\0') || (*(iVar1 + 1) != ':')) {
            if ((in_string_2._2_2_ | in_string_2) != 0) {
                *in_string_2 = '\0';
            }
        } else {
            if ((in_string_2._2_2_ | in_string_2) != 0) {
                *in_string_2 = *in_string_1;
                *(in_string_2 + 1) = *(iVar1 + 1);
                *(in_string_2 + 2) = 0;
            }
            in_string_1 = (in_string_1 & 0xffff0000 | (iVar1 + 2));
        }
    }
    local_6 = 0;
    local_8 = 0;
    _local_12 = in_string_1;
    loop {
        u_var4 = (_local_12 >> 0x10);
        u_var2 = _local_12;
        if (*_local_12 == '\0') {
            break;
        }
        if ((*_local_12 == '/') || (*_local_12 == '\\')) {
            local_8 = u_var2 + 1;
            local_6 = u_var4;
        } else {
            if (*_local_12 == '.') {
                local_c = u_var2;
                local_a = u_var4;
            }
        }
        _local_12 = (_local_12 & 0xffff0000 | (u_var2 + 1));
    }
    if ((local_6 | local_8) == 0) {
        if ((param_3._2_2_ | param_3) != 0) {
            unsafe {
                *param_3 = 0;
            }
        }
    } else {
        if ((param_3._2_2_ | param_3) != 0) {
            u_var4 = local_8 - in_string_1;
            if (0xff < u_var4) {
                u_var4 = 0xff;
            }
            process_string_1000_3dbe(
                (param_3 & 0xffff | param_3._2_2_ << 0x10),
                in_string_1,
                u_var4,
            );
            *(param_3 + u_var4) = 0;
        }
        in_string_1 = CONCAT22(local_6, local_8);
    }
    if (((local_a | local_c) != 0) && (in_string_1 <= local_c)) {
        if ((param_4._2_2_ | param_4) != 0) {
            u_var4 = local_c - in_string_1;
            if (0xff < u_var4) {
                u_var4 = 0xff;
            }
            process_string_1000_3dbe(
                (param_4 & 0xffff | param_4._2_2_ << 0x10),
                (in_string_1 & 0xffff | in_string_1._2_2_ << 0x10),
                u_var4,
            );
            *(param_4 + u_var4) = 0;
        }
        if ((param_5._2_2_ | param_5) == 0) {
            return;
        }
        u_var2 = u_var2 - local_c;
        if (0xff < u_var2) {
            u_var2 = 0xff;
        }
        process_string_1000_3dbe(
            (param_5 & 0xffff | param_5._2_2_ << 0x10),
            CONCAT22(local_a, local_c),
            u_var2,
        );
        *(param_5 + u_var2) = 0;
        return;
    }
    if ((param_4._2_2_ | param_4) != 0) {
        u_var2 = u_var2 - in_string_1;
        if (0xff < u_var2) {
            u_var2 = 0xff;
        }
        process_string_1000_3dbe(
            (param_4 & 0xffff | param_4._2_2_ << 0x10),
            (in_string_1 & 0xffff | in_string_1._2_2_ << 0x10),
            u_var2,
        );
        *(param_4 + u_var2) = 0;
    }
    if ((param_5._2_2_ | param_5) != 0) {
        unsafe {
            *param_5 = 0;
        }
    }
    return;
}

pub fn process_string_1000_545a(
    in_string_1: String,
    in_string_2: String,
) -> u8 {
    let mut char_3: u8;
    let string_1: String;
    let mut string_2: u16;
    let mut string_3: u16;
    let mut string_4: u32;
    let char_2: u8;
    let char_1: u8;

    string_3 = in_string_2;
    string_2 = in_string_1;
    string_1 = (s_You_may_not_run_a_turn__The_game_1050_00df + 0x20);
    while {
        while {
            if (string_1 == '\0') {
                return '\0';
            }
            string_4 = string_3;
            string_3 = string_3 + 1;
            char_2 = *string_4;
            char_1 = *string_2;
            string_1 = CONCAT11(char_1, char_2);
            string_2 = string_2 + 1;
            char_1 == char_2
        } {}
        char_3 = char_2 + 0xbf + (-((char_2 + 0xbf) < 0x1a) & 0x20) + 'A';
        string_1._0_1_ = char_1 + 0xbf + (-((char_1 + 0xbf) < 0x1a) & 0x20) + 0x41;
        string_1 = CONCAT11(char_3, string_1);
        string_1 == char_3
    } {}
    return (string_1 < char_3) * -2 + 0x1;
}

pub unsafe fn process_string_1000_55b1(ctx: &mut AppContext) -> String {
    let pc_var1: String;
    let mut string_2: String;
    let mut i16_1: u16;
    let mut i_var2: i32;
    let mut string_6: String;
    let mut string_7: String;
    let mut string_3: String;
    let mut in_stack_00000008: i32;
    let mut string_4: String;
    let mut string_1: String;
    let string_5: String;
    let string_2_res: Option<String>;

    pass1_fn_1000_25a8(ctx);
    pass1_fn_1000_2913(ctx, 0);
    string_2_res = process_string_1000_28dc(ctx, ctx.s_version__d__d_1050_0012[2..]);
    if string_2_res.is_some() {
        let mut string_2 = string_2_res.unwrap();
        i16_1 = 9;
        if string_2[0] == 'M' {
            i16_1 = 0xf;
        }
        string_2 = string_2[i16_1..].clone();
        i_var2 = 0x22;
        string_3 = string_2.clone();
        while {
            if i_var2 == 0 {
                break;
            }
            i_var2 = i_var2 - 1;
            string_1 = string_3.clone();
            string_3 = string_3[1..].clone();
            string_1[0] != '\r'
        } {}
        // *(string_3 - 1) = 0;
    }
    //FatalAppExit16(0, CONCAT22(0x1050, string_2));
    FatalExit(0xff);
    string_6 = ctx.s___NMSG___1050_63f6[8..].clone();
    loop {
        string_5 = string_6.clone();
        string_6 = string_6[2..].clone();
        string_7 = string_6.clone();
        if (string_5 == in_stack_00000008) || (string_7 = string_5 + 1, string_7 == 0) {
            return string_7;
        }
        i_var2 = -1;
        while {
            if i_var2 == 0 {
                break;
            }
            i_var2 = i_var2 + -1;
            pc_var1 = string_6;
            string_6 = string_6[1..].clone();
            pc_var1[0] != '\0'
        } {}
    }
}

pub fn process_string_1008_049c(param_1: u16, param_2: u16, in_string_1: String) {
    let mut str_index: u16;
    let string_1: String;
    let string_2: String;
    let extraout_var: u32;

    if (in_string_1 != 0x0) {
        str_index = get_string_index_1000_3da4(in_string_1);
        if (str_index != 0) {
            string_1._0_1_ = process_string_1000_545a(
                (in_string_1 & 0xffff0000 | (in_string_1 + 1)),
                s_nomono2_1050_00cc,
            );
            string_2 = CONCAT31(extraout_var, string_1);
            if (string_2 == 0x0) {
                g_string_1050_02ec = string_2;
            }
        }
    }
    return;
}

pub fn string_fn_1008_5fd8(param_1: *mut Struct613, param_2: u8) -> u8 {
    let pc_var1: String;
    let pi_var2: *mut i32;
    let in_dx: *mut Struct199;
    let mut unaff_ss: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    local_a = &param_2;
    _local_6 = CONCAT22(unaff_ss, local_a);
    process_struct_1000_179c(0x1000, in_dx);
    local_8 = in_dx;
    pc_var1 = load_string_1010_847e(
        ctx._g_struct_73_1050_14cc,
        (ctx._g_struct_73_1050_14cc >> 0x10),
        param_1,
    );
    copy_string_1000_3d3e(CONCAT22(local_8, local_a), pc_var1);
    loop {
        pi_var2 = _local_6;
        _local_6 = (_local_6 & 0xffff0000 | (local_6 + 2));
        let pi_var2_val = unsafe { *pi_var2 };
        if (pi_var2_val == 0) {
            break;
        }
        pc_var1 = load_string_1010_847e(
            ctx._g_struct_73_1050_14cc,
            (ctx._g_struct_73_1050_14cc >> 0x10),
            pi_var2_val,
        );
        process_string_1000_3cea(CONCAT22(local_8, local_a), pc_var1);
    }
    return local_a;
}

pub fn fn_1008_6048(ctx: &mut AppContext, in_string_1: String, param_2: u16, param_3: bool) -> u8 {
    let mut c_var1: u8 = 0;
    let mut pu_var2: Vec<u8> = Vec::new();
    let mut i_var3: u16 = 0;
    let mut i_var4: i32 = 0;
    let mut u16_var5: u16 = 0;
    let mut u16_var6: u16 = 0;
    let mut string_var7: String = String::new();

    if ctx.g_string_1050_02ec != 0x0 {
        pu_var2 = stack0x0008;
        if u16_1050_02ee == 0xffff {
            c_var1 = process_string_1000_3ec0(ctx, 0x2f4, &ctx.g_alloc_addr_1050_1050);
            u16_var5 = CONCAT11(ctx.ah_reg, c_var1);
            pu_var2 = ((param_2 | u16_var5) != 0);
            u16_1050_02ee = pu_var2;
            u16_var6 = param_2;
        }
        param_3 = pu_var2;
        if u16_1050_02ee != 0 {
            wvsprintf16(
                &stack0x0008,
                CONCAT22(in_string_1, ctx.stack_seg_reg),
                CONCAT22(string_var7, (in_string_1 >> 0x10)),
            );
            OutputDebugString16(CONCAT22(ctx.stack_seg_reg, string_var7));
            i_var3 = OutputDebugString16(0x105002fa);
            param_3 = i_var3;
            if (_PTR_LOOP_1050_02f0 != 0) {
                pass1_fn_1000_2b5c(
                    ctx._PTR_LOOP_1050_02f0,
                    (ctx._PTR_LOOP_1050_02f0 >> 0x10),
                    0x2fd,
                    &ctx.g_alloc_addr_1050_1050,
                );
                i_var4 = pass1_fn_1000_2f48(ctx._PTR_LOOP_1050_02f0, (ctx._PTR_LOOP_1050_02f0 >> 0x10));
                param_3 = i_var4;
            }
        }
    }
    return param_3;
}

pub fn string_fn_1008_64c8(
    param_1: String,
    param_2: String,
    param_3: String,
    param_4: String,
) {
    let pc_var1: String;
    let mut in_ax: i32;
    let in_dx: *mut Struct199;
    let mut local_DX_49: u16;
    let mut u_var2: u16;
    let mut local_DX_95: u16;
    let mut local_DX_121: u16;
    let mut u_var3: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    if (param_1 == 0) {
        return;
    }
    process_struct_1000_179c(0x1e, in_dx);
    if ((in_dx | in_ax) == 0) {
        in_ax = 0;
        u_var2 = 0;
    } else {
        pass1_1008_6604(CONCAT22(in_dx, in_ax), param_2, (param_2 >> 0x10));
        u_var2 = local_DX_49;
    }
    _local_6 = CONCAT22(u_var2, in_ax);
    local_8 = 0;
    while (
        param_2 = (param_2 & 0xffff0000 | (param_2 - 1)),
        param_2 != 0,
    ) {
        pc_var1 = param_3 + 1;
        process_struct_1008_4544(param_1);
        u_var2 = local_8 + 1;
        u_var3 = local_DX_95;
        process_struct_1008_4544(_local_6);
        pass1_fn_1000_48a8(
            CONCAT22(local_DX_121, local_8),
            CONCAT22(u_var3, param_3),
            param_2._2_2_,
        );
        param_3 = pc_var1;
        local_8 = u_var2;
    }
    return;
}

pub unsafe fn process_string_1008_7e4a() -> bool {
    let u_var1: u8;
    let mut buf_size: i32;
    let local_AH_52: u8;
    
    let in_stack_0000000a: String;
    let mut char_buf: u8;

    string_fn_1000_3f9c(
        &char_buf,
        ctx.stack_seg_reg,
        s__s_02x_1050_0347,
        &ctx.g_alloc_addr_1050_1050,
        _PTR_s_dcbSC_1050_0336_1050_033c,
    );
    buf_size = get_string_index_1000_3da4(CONCAT22(ctx.stack_seg_reg, &char_buf));
    u_var1 = process_string_1000_3de8(
        in_stack_0000000a,
        CONCAT22(ctx.stack_seg_reg, &char_buf),
        buf_size,
    );
    if (CONCAT11(local_AH_52, u_var1) == 0) {
        return 1;
    }
    return 0;
}

pub fn process_string_1008_9c86(param_1: u32, param_2: String, param_3: i32) {
    let mut u_var1: u16;
    let mut local_4: u16;

    u_var1 = get_string_index_1000_3da4((param_1 & 0xffff0000 | (param_1 + 10)));
    if (param_3 < u_var1) {
        u_var1 = param_3 - 1;
    }
    process_string_1000_3dbe(param_2, (param_1 & 0xffff0000 | (param_1 + 10)), u_var1);
    return;
}

pub fn load_string_switch_1008_a1f0(
    str_buffer_1: String,
    param_2: *mut u16,
    param_3: u32,
    param_4: u32,
    param_5: u32,
) {
    let mut u_var1: u32;
    let mut b: u16;
    let mut u_var2: i32;
    let mut u_var3: u16;
    let paVar4: *mut Struct493;
    let pcVar5: String;
    let pcVar6: String;
    let local_DL_217: u8;
    
    
    let mut local_DX_469: u16;
    let mut local_DX_507: u16;
    let mut local_DX_1061: u16;
    let string_base: String;
    let mut local_es_32: u16;
    let mut local_es_121: u16;
    let mut local_CS_1605: u16;
    
    let mut u_var7: u32;
    let pp_var8: *mut Struct2111;
    let mut a: u16;
    let local_11e: u8;
    let local_11d: u8;
    let mut resource_id_1: u16;
    let mut local_112: u32;
    let mut local_10e: u32;
    let mut local_10a: u16;
    let mut local_108: u16;
    let mut local_106: [u8; 256];
    let mut local_6: u16;
    let mut local_4: u16;
    let mut temp_5f95f699dc: u32;
    let mut temp_5f759b5e79: u32;
    let fn_ptr_1: fn();

    u_var2 = 0;
    param_5 = 0;
    param_4 = 0;
    param_3 = 0;
    unsafe {
        *param_2 = 0;
    }
    local_es_32 = (str_buffer_1 >> 0x10);
    string_base = str_buffer_1;
    string_base[0xe] = '\0';
    temp_5f759b5e79 = (string_base + 10);
    fn_ptr_1 = ((string_base + 10) + 0x10);
    (**fn_ptr_1)();
    _local_6 = CONCAT22(ctx.dx_reg, u_var2);
    if ((ctx.dx_reg | u_var2) == 0) {
        return;
    }
    param_5 = (u_var2 + 4);
    u_var3 = (u_var2 + 10);
    param_3 = u_var3;
    switch_stmt_1008_ab80(string_base, local_es_32, param_5);
    unsafe {
        *param_2 = u_var3;
    }
    local_CS_1605 = &ctx.PTR_LOOP_1050_1008;
    a = ctx._g_struct_73_1050_14cc;
    local_11e = (str_buffer_1 >> 0x10);
    b = (ctx._g_struct_73_1050_14cc >> 0x10);
    match (u_var2 + 4) {
        1 => {
            load_string_1010_84e0(
                a,
                b,
                0x3ff,
                (str_buffer_1 & 0xffff0000 | ZEXT24(string_base + 0xe)),
                0x59a,
            );
            param_4 = 0xd1;
            // goto LAB_1008_a2b1;
        }
        2 => {
            u_var1 = (u_var2 + 6);
            resource_id_1 = (u_var1 >> 0x10);
            _local_DL_217 = ctx.dx_reg;
            paVar4 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, u_var1, resource_id_1);
            local_11e = ctx.stack_seg_reg;
            local_11d = (ctx.stack_seg_reg >> 8);
            load_string_1010_84e0(
                ctx._g_struct_73_1050_14cc,
                (ctx._g_struct_73_1050_14cc >> 0x10),
                0x100,
                CONCAT13(local_11d, CONCAT12(local_11e, local_106)),
                0x3ff,
            );
            local_11e = _local_DL_217;
            local_11d = (_local_DL_217 >> 8);
            u_var7 = pass1_1038_4d28(CONCAT13(local_11d, CONCAT12(local_11e, paVar4)));
            local_CS_1605 = 0x1000;
            string_fn_1000_3f9c(
                string_base + 0xe,
                local_es_32,
                local_106,
                ctx.stack_seg_reg,
                u_var7,
            )
        }
        5 => {
            resource_id_1 = 0x59b;
            // goto LAB_1008_a277;
        }
        6 => {
            load_string_1010_84e0(
                a,
                b,
                0x3ff,
                (str_buffer_1 & 0xffff0000 | ZEXT24(string_base + 0xe)),
                0x631,
            );
            param_4 = 0xd4;
            // LAB_1008_a2b1:
            local_CS_1605 = 0x1010;
            param_3 = 1
        }
        7 => {
            resource_id_1 = 0x400;
            // LAB_1008_a277:
            local_CS_1605 = 0x1010;
            load_string_1010_84e0(
                a,
                b,
                0x3ff,
                (str_buffer_1 & 0xffff0000 | ZEXT24(string_base + 0xe)),
                resource_id_1,
            )
        }
        9 => {
            if ((string_base + 0x416) == 0) {
                (string_base + 0x416) = 1;
            }
            // goto LAB_1008_a35a;
        }
        0xb => {
            if ((string_base + 0x41a) == 0) {
                (string_base + 0x41a) = 1;
            }
            // goto LAB_1008_a35a;
        }
        0xe => {
            if ((string_base + 0x41c) == 0) {
                (string_base + 0x41c) = 1;
            }
            // goto LAB_1008_a35a;
        }
        0x14 => {
            if ((string_base + 0x418) == 0) {
                (string_base + 0x418) = 1;
                pcVar6 = string_base + 0xe;
                pcVar5 = pcVar6;
                load_string_1010_84e0(
                    ctx._g_struct_73_1050_14cc,
                    (ctx._g_struct_73_1050_14cc >> 0x10),
                    0x3ff,
                    (str_buffer_1 & 0xff000000 | CONCAT12(local_11e, pcVar6)),
                    0x72a,
                );
                local_11e = (ctx._g_struct_73_1050_14cc >> 0x10);
                load_string_1010_847e(ctx._g_struct_73_1050_14cc, local_11e, 0x72b);
                process_string_1000_3cea(
                    (str_buffer_1 & 0xffff0000 | ZEXT24(pcVar6)),
                    CONCAT22(local_DX_469, pcVar5),
                );
                param_4 = 0x4c;
                pp_var8 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, 0x1002b);
                local_CS_1605 = 0x1010;
                pass1_1010_089e(pp_var8, 1, 10);
            }
            // goto LAB_1008_a35a;
        }
        0x16 => {
            local_CS_1605 = 0x1010;
            load_string_1010_84e0(
                a,
                b,
                0x3ff,
                (str_buffer_1 & 0xffff0000 | ZEXT24(string_base + 0xe)),
                0x713,
            );
            param_4 = 0x28
        }
        0x17 => {
            local_CS_1605 = 0x1010;
            load_string_1010_84e0(
                a,
                b,
                0x3ff,
                (str_buffer_1 & 0xffff0000 | ZEXT24(string_base + 0xe)),
                0x717,
            );
            param_4 = 0x2c
        }
        0x18 => {
            local_CS_1605 = 0x1010;
            load_string_1010_84e0(
                a,
                b,
                0x3ff,
                (str_buffer_1 & 0xffff0000 | ZEXT24(string_base + 0xe)),
                0x719,
            );
            param_4 = 0x2e
        }
        0x1b => {
            local_CS_1605 = 0x1010;
            load_string_1010_84e0(
                a,
                b,
                0x3ff,
                (str_buffer_1 & 0xffff0000 | ZEXT24(string_base + 0xe)),
                0x71b,
            );
            param_4 = 0x30
        }
        0x1c => {
            local_CS_1605 = 0x1010;
            load_string_1010_84e0(
                a,
                b,
                0x3ff,
                (str_buffer_1 & 0xffff0000 | ZEXT24(string_base + 0xe)),
                0x71d,
            );
            param_4 = 0x32
        }
        0x1f => {
            local_CS_1605 = 0x1010;
            load_string_1010_84e0(
                a,
                b,
                0x3ff,
                (str_buffer_1 & 0xffff0000 | ZEXT24(string_base + 0xe)),
                0x720,
            );
            param_4 = 0x34
        }
        0x21 => {
            if ((string_base + 0x41e) == 0) {
                (string_base + 0x41e) = 1;
            }
            // LAB_1008_a35a:
            unsafe { *param_2 = 0 }
        }
        0x24 => {
            local_CS_1605 = 0x1010;
            load_string_1010_84e0(
                a,
                b,
                0x3ff,
                (str_buffer_1 & 0xffff0000 | ZEXT24(string_base + 0xe)),
                0x715,
            );
            param_4 = 0x2a
        }
        0x31 => {
            local_CS_1605 = 0x1010;
            load_string_1010_84e0(
                a,
                b,
                0x3ff,
                (str_buffer_1 & 0xffff0000 | ZEXT24(string_base + 0xe)),
                0x712,
            );
            param_4 = 0x27
        }
        0x32 => {
            local_CS_1605 = 0x1010;
            load_string_1010_84e0(
                a,
                b,
                0x3ff,
                (str_buffer_1 & 0xffff0000 | ZEXT24(string_base + 0xe)),
                0x714,
            );
            param_4 = 0x29
        }
        0x33 => {
            local_CS_1605 = 0x1010;
            load_string_1010_84e0(
                a,
                b,
                0x3ff,
                (str_buffer_1 & 0xffff0000 | ZEXT24(string_base + 0xe)),
                0x716,
            );
            param_4 = 0x2b
        }
        0x34 => {
            local_CS_1605 = 0x1010;
            load_string_1010_84e0(
                a,
                b,
                0x3ff,
                (str_buffer_1 & 0xffff0000 | ZEXT24(string_base + 0xe)),
                0x718,
            );
            param_4 = 0x2d
        }
        0x35 => {
            local_CS_1605 = 0x1010;
            load_string_1010_84e0(
                a,
                b,
                0x3ff,
                (str_buffer_1 & 0xffff0000 | ZEXT24(string_base + 0xe)),
                0x71a,
            );
            param_4 = 0x2f
        }
        0x36 => {
            local_CS_1605 = 0x1010;
            load_string_1010_84e0(
                a,
                b,
                0x3ff,
                (str_buffer_1 & 0xffff0000 | ZEXT24(string_base + 0xe)),
                0x71c,
            );
            param_4 = 0x31
        }
        0x37 => {
            pcVar5 = string_base + 0xe;
            pcVar6 = pcVar5;
            load_string_1010_84e0(
                a,
                b,
                0x3ff,
                (str_buffer_1 & 0xff000000 | CONCAT12(local_11e, pcVar5)),
                0x71e,
            );
            local_11e = (ctx._g_struct_73_1050_14cc >> 0x10);
            load_string_1010_847e(ctx._g_struct_73_1050_14cc, local_11e, 0x71f);
            local_CS_1605 = 0x1000;
            process_string_1000_3cea(
                (str_buffer_1 & 0xffff0000 | ZEXT24(pcVar5)),
                CONCAT22(local_DX_1061, pcVar6),
            );
            param_4 = 0x33
        }
        0x38 => {
            local_CS_1605 = 0x1010;
            load_string_1010_84e0(
                a,
                b,
                0x3ff,
                (str_buffer_1 & 0xffff0000 | ZEXT24(string_base + 0xe)),
                0x721,
            );
            param_4 = 0x35
        }
        0x39 => {
            local_CS_1605 = 0x1010;
            load_string_1010_84e0(
                a,
                b,
                0x3ff,
                (str_buffer_1 & 0xffff0000 | ZEXT24(string_base + 0xe)),
                0x722,
            );
            param_4 = 0x36
        }
        0x3a => {
            local_CS_1605 = 0x1010;
            load_string_1010_84e0(
                a,
                b,
                0x3ff,
                (str_buffer_1 & 0xffff0000 | ZEXT24(string_base + 0xe)),
                0x723,
            );
            param_4 = 0x37
        }
        0x3b => {
            local_CS_1605 = 0x1010;
            load_string_1010_84e0(
                a,
                b,
                0x3ff,
                (str_buffer_1 & 0xffff0000 | ZEXT24(string_base + 0xe)),
                0x724,
            );
            param_4 = 0x38
        }
        0x3c => {
            local_CS_1605 = 0x1010;
            load_string_1010_84e0(
                a,
                b,
                0x3ff,
                (str_buffer_1 & 0xffff0000 | ZEXT24(string_base + 0xe)),
                0x725,
            );
            param_4 = 0x39
        }
        0x3d => {
            local_CS_1605 = 0x1010;
            load_string_1010_84e0(
                a,
                b,
                0x3ff,
                (str_buffer_1 & 0xffff0000 | ZEXT24(string_base + 0xe)),
                0x742,
            );
            param_4 = 0xce
        }
        0x3e => {
            local_CS_1605 = 0x1010;
            load_string_1010_84e0(
                a,
                b,
                0x3ff,
                (str_buffer_1 & 0xffff0000 | ZEXT24(string_base + 0xe)),
                0x743,
            );
            param_4 = 0xcf
        }
        0x3f => {
            local_CS_1605 = 0x1010;
            load_string_1010_84e0(
                a,
                b,
                0x3ff,
                (str_buffer_1 & 0xffff0000 | ZEXT24(string_base + 0xe)),
                0x744,
            );
            param_4 = 0xd0
        }
        0x40 => {
            local_CS_1605 = 0x1010;
            load_string_1010_84e0(
                a,
                b,
                0x3ff,
                (str_buffer_1 & 0xffff0000 | ZEXT24(string_base + 0xe)),
                0x745,
            );
            param_4 = 0xd1
        }
        0x41 => {
            local_CS_1605 = 0x1010;
            load_string_1010_84e0(
                a,
                b,
                0x3ff,
                (str_buffer_1 & 0xffff0000 | ZEXT24(string_base + 0xe)),
                0x746,
            );
            param_4 = 0xd2
        }
        0x42 => {
            local_CS_1605 = 0x1010;
            load_string_1010_84e0(
                a,
                b,
                0x3ff,
                (str_buffer_1 & 0xffff0000 | ZEXT24(string_base + 0xe)),
                0x747,
            );
            param_4 = 0xd3
        }
        0x43 => {
            local_CS_1605 = 0x1010;
            load_string_1010_84e0(
                a,
                b,
                0x3ff,
                (str_buffer_1 & 0xffff0000 | ZEXT24(string_base + 0xe)),
                0x748,
            );
            param_4 = 0xd5
        }
        0x44 => {
            local_CS_1605 = 0x1010;
            load_string_1010_84e0(
                a,
                b,
                0x3ff,
                (str_buffer_1 & 0xffff0000 | ZEXT24(string_base + 0xe)),
                0x749,
            );
            param_4 = 0xd6
        }
        0x45 => {
            local_CS_1605 = 0x1010;
            load_string_1010_84e0(
                a,
                b,
                0x3ff,
                (str_buffer_1 & 0xffff0000 | ZEXT24(string_base + 0xe)),
                0x74a,
            );
            param_4 = 0xd7;
        }
    }
    if (_local_6 != 0x0) {
        local_11e = ctx.dx_reg;
        fn_ptr_1 = *_local_6;
        (**fn_ptr_1)(local_CS_1605, u_var2, local_11e, 1, temp_5f759b5e79);
    }
    return;
}

pub fn load_string_1008_a8f4(
    param_1: String,
    param_2: *mut u16,
    param_3: *mut u16,
    param_4: u32,
) {
    let mut unaff_ss: u16;
    let mut u_var1: u32;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;

    u_var1 = load_string_switch_1008_a1f0(
        param_1,
        param_2,
        CONCAT22(unaff_ss, &local_6),
        CONCAT22(unaff_ss, &local_6 + 2),
        param_4,
    );
    pass1_1008_944e(param_3, local_6, (local_6 >> 0x10));
    return u_var1;
}

pub fn load_string_1008_b1f0() {
    load_string_1010_847e(ctx._g_struct_73_1050_14cc, 0x531, 0);
    return;
}

pub unsafe fn load_string_1008_b65a(param_1: u32, param_2: &mut string, param_3: u32) {
    let mut in_ax: i32;
    let mut in_dx: i32;
    let mut in_resource_id: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    pass1_1008_b9ce(param_1, param_3);
    if (((in_dx | in_ax) == 0) || ((in_ax + 8) != 1)) {
        in_resource_id = 0x434;
    } else {
        in_resource_id = 0x435;
    }
    load_string_1010_84e0(
        ctx._g_struct_73_1050_14cc,
        (ctx._g_struct_73_1050_14cc >> 0x10),
        0x3ff,
        param_2,
        in_resource_id,
    );
    return;
}

// WARNING: Variable defined which should be unmapped: u16_d
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn wsprintf_func_1008_b69c(struct_a: *mut Struct2111) {
    let struct_e_lo: *mut Struct199;
    let struct_e_a: *mut Struct199;
    let struct_c: *mut Struct915;
    let mut u16_j: u16;
    let struct_e_hi: *mut Struct199;
    let struct_e_b: *mut Struct199;
    let mut u16_a: u16;
    let mut u16_b: u16;
    let struct_b: *mut Struct2111;
    let struct_b_hi: *mut Struct2111;
    let mut u16_c: u16;
    let struct_d: *mut Struct199;
    let mut u32_a: u32;
    let mut u16_d: u16;
    let mut u16_e: u16;
    let mut u16_f: u16;
    let mut u16_g: u16;
    let mut u16_h: u16;
    let mut va_args: [u8; 256];
    let fn_ptr_a: fn();

    struct_e_lo = &stack0xfdfe;
    load_string_1010_84e0(
        ctx._g_struct_73_1050_14cc,
        (ctx._g_struct_73_1050_14cc >> 0x10),
        0x100,
        CONCAT22(u16_c, struct_e_lo),
        0x6e7,
    );
    struct_b_hi = (struct_a >> 0x10);
    struct_b = struct_a;
    if (&struct_b.field_0xa == 0) {
        process_struct_1000_179c(0xc, struct_e_hi);
        struct_e_b = (struct_e_hi | struct_e_lo);
        if (struct_e_b == 0x0) {
            struct_e_a = 0x0;
            struct_e_b = 0x0;
        } else {
            struct_e_a = process_struct_1008_574a(CONCAT22(struct_e_hi, struct_e_lo));
        }
        struct_d = CONCAT22(struct_e_b, struct_e_a);
        struct_b.field_0xa = struct_e_a;
        struct_b.field_0xc = struct_e_b;
        u16_h = 1;
        while (u16_h < 6) {
            process_struct_1000_179c(0x12, (struct_d >> 0x10));
            u32_a._2_2_ = (struct_d >> 0x10) | struct_d;
            if (struct_d == 0x0) {
                struct_c = 0x0;
                u16_a = 0;
            } else {
                u32_a._0_1_ = pass1_1008_b0bc(struct_d);
                struct_c = CONCAT11(u32_a._1_1_, u32_a);
                u16_a = u32_a._2_2_;
            }
            u16_b = u16_a;
            wsprintf16(
                va_args,
                CONCAT22(&stack0xfdfe, u16_c),
                CONCAT22(u16_h, u16_c),
            );
            u16_j = pass1_fn_1008_60e8(va_args);
            struct_c.field_0x4 = u16_j;
            struct_c.field_0x6 = u16_b;
            fn_ptr_a = (&struct_b.field_0xa + 8);
            struct_d = (**fn_ptr_a)();
            u16_h = &struct_c.field_0x1;
        }
        struct_b.field_0x22 = 5;
    }
    return;
}

// WARNING: Variable defined which should be unmapped: local_20a
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn wsprintf_FUN_1008_b78a(param_1: u32) {
    let piVar1: *mut i32;
    let ppc_var2: fn();
    let u_var3: u8;
    let mut in_ax: i32;
    let extraout_AH: u8;
    let mut i_var4: i32;
    let mut u_var5: u16;
    let in_dx: *mut Struct199;
    let mut u_var6: i32;
    let mut u_var7: i32;
    let mut i_var8: i32;
    let mut u_var9: u16;
    let mut unaff_ss: u16;
    let mut local_20a: u16;
    let mut local_208: u16;
    let mut local_206: u32;
    let puStack514: Vec<u8>;
    let mut va_args: [u8; 256];
    let mut local_6: u16;
    let mut local_4: u16;

    process_struct_1000_179c(0x12, in_dx);
    u_var6 = in_dx | in_ax;
    if (u_var6 == 0) {
        i_var4 = 0;
        u_var6 = 0;
    } else {
        u_var3 = pass1_1008_b0bc(CONCAT22(in_dx, in_ax));
        i_var4 = CONCAT11(extraout_AH, u_var3);
    }
    u_var7 = u_var6;
    load_string_1010_84e0(
        ctx._g_struct_73_1050_14cc,
        (ctx._g_struct_73_1050_14cc >> 0x10),
        0x100,
        CONCAT22(unaff_ss, &local_206),
        0x6e7,
    );
    u_var9 = (param_1 >> 0x10);
    i_var8 = param_1;
    piVar1 = (i_var8 + 0x22);
    unsafe {
        *piVar1 = *piVar1 + 1;
    }
    wsprintf16(
        va_args,
        CONCAT22(&local_206, unaff_ss),
        CONCAT22((i_var8 + 0x22), unaff_ss),
    );
    puStack514 = va_args;
    local_206 = 0x1538b7f9;
    u_var5 = pass1_fn_1008_60e8();
    (i_var4 + 4) = u_var5;
    (i_var4 + 6) = u_var7;
    local_206 = (i_var8 + 10);
    ppc_var2 = ((i_var8 + 10) + 8);
    puStack514 = i_var4;
    ppc_var2(offset);
    return;
}

pub unsafe fn wsprintf_1008_d1c6(in_struct_a: *mut pass1_struct_3, param_2: u32) {
    let mut iVar1: i32;
    let ppc_var2: fn();
    let mut u_var3: u32;
    let u_var4: u8;
    let local_AL_436: u8;
    let struct_a_hi: *mut Struct199;
    let pa_var5: *mut Struct199;
    let local_AX_93: *mut astruct;
    let mut struct_d: i32;
    let mut struct_e: i32;
    let struct_h: *mut astruct;
    let mut local_AX_234: u16;
    let mut struct_c: i32;
    let struct_f: *mut Struct199;
    let struct_g: *mut Struct199;
    
    let mut local_DX_121: u16;
    
    let struct_a: *mut Struct199;
    
    let mut local_DX_436: u16;
    let struct_b: *mut pass1_struct_3;
    let struct_b_hi: *mut pass1_struct_3;
    let mut u_var8: i32;
    let local_46: u8;
    let mut u_stack68: i32;
    let mut local_42: u16;
    let mut local_40: u16;
    let mut local_34: u16;
    let mut local_32: u16;
    let mut local_30: u16;
    let mut local_20: u32;
    let mut local_1a: u16;
    let mut local_16: u16;
    let mut local_14: u32;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut temp_5fd6f85ef2: u32;
    u16 * *fn_ptr_1;
    let puVar6: Vec<u8>;
    let mut u_var7: u32;
    let mut bool_a: bool;

    struct_b_hi = (in_struct_a >> 0x10);
    struct_b = in_struct_a;
    struct_a_hi = struct_b.field_0x12;
    pa_var5 = struct_b.field_0x14;
    if ((pa_var5 | struct_a_hi) != 0) {
        fn_ptr_1 = struct_a_hi;
        (**fn_ptr_1)();
        pa_var5 = struct_f;
    }
    process_struct_1000_179c(0xc, pa_var5);
    struct_g = (pa_var5 | struct_a_hi);
    if (struct_g == 0x0) {
        pa_var5 = 0x0;
        struct_g = 0x0;
    } else {
        pa_var5 = process_struct_1008_574a(CONCAT22(pa_var5, struct_a_hi));
    }
    struct_b.field_0x12 = pa_var5;
    struct_b.field_0x14 = struct_g;
    puVar6 = pass1_1008_c6fa(ctx._PTR_LOOP_1050_06e0, 2);
    struct_d = puVar6;
    u_var8 = &PTR_LOOP_1050_1038;
    pass1_1038_4e78(param_2, puVar6 & 0xffff | ZEXT24(struct_g) << 0x10);
    _local_a = CONCAT22(ctx.dx_reg, struct_d);
    local_46 = struct_d;
    u_var4 = local_46;
    ppc_var2 = (*_local_a + 0x10);
    struct_e = struct_d;
    ppc_var2(&PTR_LOOP_1050_1038, local_46, ctx.dx_reg);
    _local_e = CONCAT22(local_DX_121, struct_e);
    bool_a = false;
    local_14 = 0;
    u_stack68 = ctx.dx_reg;
    while (local_14 < _local_e) {
        u_var8 = 0x1030;
        u_var7 = _local_e;
        pass1_1030_1d7c(struct_d, ctx.dx_reg, local_14, (local_14 >> 0x10));
        struct_e = u_var7;
        if ((((ctx.dx_reg | struct_e) != 0) && ((struct_e + 0x1c) != 0x8000002))
            && ((iVar1 = (struct_e + 0x12), iVar1 == 5 || (iVar1 == 6))))
        {
            local_AX_234 = (struct_e + 4);
            pa_var5 = ((struct_e + 6) & 0xff);
            big_switch_statement_1020_bd80((struct_e + 0xc));
            wsprintf16(
                struct_b + 1,
                CONCAT13(0xc, CONCAT12(0xea, struct_b_hi)),
                CONCAT22(local_AX_234, 0x1050),
            );
            u_var7 = ZEXT24(struct_b + 1);
            pass1_fn_1008_60e8();
            struct_c = u_var7;
            u_var3 = ZEXT24(pa_var5);
            u_var8 = 0x1000;
            process_struct_1000_179c(0x12, pa_var5);
            if ((pa_var5 | struct_c) != 0) {
                u_var8 = 0x1018;
                pass1_1018_4808(
                    CONCAT22(pa_var5, struct_c),
                    1,
                    u_var7 & 0xffff | u_var3 << 0x10,
                    (struct_e + 4),
                );
            }
            local_46 = '0';
            ppc_var2 = (&struct_b.field_0x12 + 4);
            ppc_var2();
            bool_a = true;
            u_stack68 = u_var8;
        }
        local_14 = local_14 + 1;
    }
    if (!bool_a) {
        load_str_1010_84ac(
            ctx._g_struct_73_1050_14cc,
            (ctx._g_struct_73_1050_14cc >> 0x10),
            0x441,
        );
        u_var8 = 0x1000;
        pa_var5 = struct_a;
        struct_e = _local_e;
        process_struct_1000_179c(0x12, struct_a);
        if ((pa_var5 | struct_e) == 0) {
            local_AL_436 = '\0';
            local_DX_436 = 0;
        } else {
            u_var8 = 0x1018;
            pass1_1018_4808(
                CONCAT22(pa_var5, struct_e),
                0,
                CONCAT13((struct_a >> 8), CONCAT12(struct_a, _local_e)),
                0,
            );
            local_AL_436 = struct_e;
            local_DX_436 = ctx.dx_reg;
        }
        temp_5fd6f85ef2 = &struct_b.field_0x12;
        ppc_var2 = (&struct_b.field_0x12 + 4);
        ppc_var2(
            u_var8,
            temp_5fd6f85ef2,
            (temp_5fd6f85ef2 >> 0x10),
            local_AL_436,
            local_DX_436,
        );
    }
    if ((ctx.dx_reg | struct_d) != 0) {
        ppc_var2 = *_local_a;
        ppc_var2(u_var8, u_var4, ctx.dx_reg, 1, local_46, u_stack68);
    }
    return;
}

pub unsafe fn wsprintf_1008_d4f6(param_1: *mut Struct298, in_struct_b: *mut Struct298) -> u8 {
    let mut iVar1: i32;
    let mut switch_var: u16;
    let lVar2: u32;
    let pu_var3: *mut u32;
    let mut u_var4: u32;
    let ppcVar5: fn();
    let mut bVar6: bool;
    let pu_var7: *mut u32;
    let pa_var8: *mut Struct199;
    let BVar9: bool;
    let mut u_var10: i32;
    let mut u_var11: i32;
    let pu_var12: *mut u32;
    let mut u_var13: u32;
    let mut u_var14: u32;
    
    let mut uVar15: i32;
    
    
    let paVar16: *mut Struct199;
    let struct_a: *mut Struct199;
    
    let mut uVar17: u16;
    let local_bx_6: *mut Struct3;
    let local_bx_19: *mut Struct2;
    let mut uVar18: u16;
    let paVar19: *mut Struct199;
    let mut local_4e: u16;
    let mut local_4c: u16;
    let mut local_3a: u16;
    let mut local_38: u16;
    let mut local_36: u16;
    let mut local_2c: u16;
    let mut local_1e: u16;
    let mut local_1a: u16;
    let mut local_18: u32;
    let mut local_14: u32;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u32;
    let mut local_6: u32;

    uVar18 = (in_struct_b >> 0x10);
    local_bx_6 = in_struct_b;
    lVar2 = local_bx_6.field_0x200;
    paVar19 = (param_1 >> 0x10);
    local_bx_19 = param_1;
    pu_var7 = local_bx_19.field_0xe;
    pa_var8 = local_bx_19.field_0x10;
    if ((pa_var8 | pu_var7) != 0) {
        unsafe {
            ppcVar5 = *pu_var7;
        }
        (**ppcVar5)();
        pa_var8 = ctx.dx_reg;
    }
    process_struct_1000_179c(0xc, pa_var8);
    uVar15 = pa_var8 | pu_var7;
    if (uVar15 == 0) {
        pa_var8 = 0x0;
        uVar15 = 0;
    } else {
        pa_var8 = process_struct_1008_574a(CONCAT22(pa_var8, pu_var7));
    }
    local_bx_19.field_0xe = pa_var8;
    local_bx_19.field_0x10 = uVar15;
    pu_var3 = &local_bx_6.field_0xc;
    unsafe {
        ppcVar5 = (*pu_var3 + 0x10);
    }
    pu_var12 = pu_var3;
    (**ppcVar5)(0x1000, pu_var3, &local_bx_6.field_0xe);
    u_var14 = pu_var12 & 0xffff | ctx.dx_reg << 0x10;
    bVar6 = false;
    local_14 = 0;
    while (local_14 < u_var14) {
        u_var13 = u_var14;
        pass1_1030_1d7c(pu_var3, (pu_var3 >> 0x10), local_14, (local_14 >> 0x10));
        uVar15 = u_var13;
        if ((((ctx.dx_reg | uVar15) != 0) && ((uVar15 + 0x1c) != 0x8000002))
            && ((iVar1 = (uVar15 + 0x12), iVar1 == 5 || (iVar1 == 6))))
        {
            switch_var = (uVar15 + 0xc);
            BVar9 = pass1_1008_c6ae(ctx._PTR_LOOP_1050_06e0, switch_var, 0x34);
            if ((BVar9 == 0) && ((uVar15 + 0x1c) != lVar2)) {
                uVar18 = (uVar15 + 4);
                big_switch_statement_1020_bd80(switch_var);
                pa_var8 = paVar19;
                u_var10 = wsprintf16(
                    local_bx_19 + 1,
                    CONCAT22(0xcf4, paVar19),
                    CONCAT22(uVar18, 0x1050),
                );
                pass1_fn_1008_60e8();
                uVar18 = 0x1000;
                paVar16 = pa_var8;
                u_var11 = u_var10;
                process_struct_1000_179c(0x14, pa_var8);
                if ((paVar16 | u_var11) != 0) {
                    uVar18 = 0x1018;
                    pass1_1018_47c8(
                        CONCAT13((paVar16 >> 8), CONCAT12(paVar16, u_var11)),
                        1,
                        CONCAT13((pa_var8 >> 8), CONCAT12(pa_var8, u_var10)),
                        (uVar15 + 0xc),
                        (uVar15 + 4),
                    );
                }
                u_var4 = &local_bx_19.field_0xe;
                // WARNING: Load size is inaccurate
                ppcVar5 = (*local_bx_19.field_0xe + 4);
                (**ppcVar5)(uVar18, u_var4, (u_var4 >> 0x10));
                bVar6 = true;
            }
        }
        local_14 = local_14 + 1;
    }
    if (!bVar6) {
        load_str_1010_84ac(
            ctx._g_struct_73_1050_14cc,
            (ctx._g_struct_73_1050_14cc >> 0x10),
            0x442,
        );
        uVar18 = 0x1000;
        u_var13 = u_var14;
        pa_var8 = struct_a;
        process_struct_1000_179c(0x14, struct_a);
        if ((pa_var8 | u_var13) == 0) {
            u_var14 = 0;
            uVar17 = 0;
        } else {
            uVar18 = 0x1018;
            pass1_1018_47c8(
                u_var13 & 0xffff | ZEXT24(pa_var8) << 0x10,
                0,
                u_var14 & 0xffff | ZEXT24(struct_a) << 0x10,
                0,
                0,
            );
            u_var14 = u_var13;
            uVar17 = ctx.dx_reg;
        }
        u_var4 = &local_bx_19.field_0xe;
        // WARNING: Load size is inaccurate
        ppcVar5 = (*local_bx_19.field_0xe + 4);
        (**ppcVar5)(uVar18, u_var4, (u_var4 >> 0x10), u_var14, uVar17);
    }
    return u_var14;
}

pub fn load_Str_1038_8dda(param_1: u32) {

    let in_dx: &mut Struct199;
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
