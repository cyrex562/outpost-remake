use crate::app_context::AppContext;
use crate::err_funcs::error_check_1000_17ce;
use crate::other_funcs::zero_list_1008_3e38;
use crate::pass::pass11_funcs::pass1_1008_cfa0;
use crate::pass::pass14_funcs::{pass1_1008_3e76, pass1_1008_5784, pass1_1008_5b12, pass1_fn_1008_60e8, pass1_fn_1008_612e};
use crate::pass::pass15_funcs::{pass1_1020_ba3e, pass1_1020_ba7e, pass1_1020_bae6, pass1_1020_bb8a};
use crate::pass::pass17_funcs::{pass1_1030_1cd8, pass1_1030_2690, pass1_1030_627e, pass1_1030_73a8, pass1_1030_7bee, pass1_1030_7c28, pass1_1030_7d1c, pass1_1030_7ddc, pass1_1030_809c, pass1_1030_8326, pass1_1030_8344};
use crate::pass::pass20_funcs::{pass1_1010_c3c2, pass1_1010_dd5e};
use crate::pass::pass2_funcs::pass1_1010_e964;
use crate::pass::pass4_funcs::{pass1_1028_bb24, pass1_1028_d1dc, pass1_1028_dc52, pass1_1028_e1ec, pass1_1028_e2e0, pass1_1028_e4ec};
use crate::pass::pass5_funcs::pass1_1030_b9b2;
use crate::pass::pass6_funcs::{pass1_1038_4d28, pass1_1038_5464, pass1_1038_56d6};
use crate::pass::pass7_funcs::{pass1_1018_3e8c, pass1_1018_427c, pass1_1018_435e};
use crate::pass::pass8_funcs::{pass1_1008_e852, pass1_1008_e8cc, pass1_1010_60a0, pass1_1010_878c};
use crate::pass::pass_funcs::{pass1_1000_3d7a, pass1_1000_4906, pass1_fn_1000_2b3c, pass1_fn_1000_2b5c, pass1_fn_1000_2f48};
use crate::prog_structs::prog_structs_10::Struct73;
use crate::prog_structs::prog_structs_16::Struct493;
use crate::prog_structs::prog_structs_18::Struct566;
use crate::prog_structs::prog_structs_19::Struct500;
use crate::prog_structs::prog_structs_1::Struct393;
use crate::prog_structs::prog_structs_20::Struct965;
use crate::prog_structs::prog_structs_24::{Struct2111, Struct432, Struct894};
use crate::prog_structs::prog_structs_26::{Struct1123, Struct883, Struct891};
use crate::prog_structs::prog_structs_27::{pass1_struct_2, Struct298};
use crate::prog_structs::prog_structs_28::{Struct1016, Struct1053, Struct346, Struct912, Struct913};
use crate::prog_structs::prog_structs_29::Struct1035;
use crate::prog_structs::prog_structs_2::Struct199;
use crate::prog_structs::prog_structs_3::Struct446;
use crate::string_ops1;
use crate::struct_ops1::{process_struct_1000_179c, process_struct_1008_cbc4, process_struct_1008_cda2, process_struct_1008_d3ae, process_struct_1040_b0bc};
use crate::sys_ops::dos_ops::dos3_call_1000_51aa;
use crate::sys_ops::private_profile_str::write_private_profile_str_1010_5b10;
use crate::util::{CARRY1, CONCAT12, CONCAT13, CONCAT22, CONCAT31, SBORROW2, SUB42, ZEXT24};
use crate::winapi_funcs::{LoadString16, MessageBox16};

pub fn modify_string_11d8_024f(param_1: u8, param_2: u16, param_3: u16) {
    let pu8_var1: Vec<u8>;
    let pi_var2: *mut i32;
    char * *ppc_var3;
    let pu_var4: *mut u32;
    let pu_var5: Vec<u8>;
    byte * *ppbVar6;
    let pcVar7: String;
    let puVar8: *mut u16;
    char * *ppc_var9;
    let mut cVar10: u8;
    let pc_var11: *mut code;
    let mut char8: u8;
    let mut bVar12: u8;
    let mut cVar13: u8;
    let mut b_var14: u8;
    let mut bVar15: u8;
    let mut cVar16: u8;
    let mut cVar18: u8;
    let mut cVar19: u8;
    let mut b_var20: u8;
    let mut u_var21: i32;
    let mut extraout_DL: u8;
    let mut extraout_DL_00: u8;
    let mut c_var22: u8;
    let mut c_var23: u8;
    let mut u_var25: i32;
    let pu_var26: *mut u32;
    let pi_var27: *mut i32;
    let mut local_BP__1: u16;
    let unaff_si: String;
    let pc_var29: String;
    let unaff_DI: String;
    let mut local_ES__1: u16;
    let mut local_DS__1: u16;
    let mut local_FS__1: u16;

    let mut local_resc: u16;
    let mut local_res18: u32;
    let mut in_stack_00000063: u8;
    let mut uStack23: i32;
    let mut cStack17: u8;
    let local_3: u8;
    let mut uVar31: i32;
    let temp_86276906b60: *mut u16;
    let mut iVar17: i32;
    let mut u_var24: i32;
    let mut b_var28: u8;
    let mut uVar30: u32;
    let string_1: String;

// pub fn modify_string_11b8_02b9(param_1: u8, param_2: u16, param_3: u16) {
//     let mut byte_2: u8;
//     let mut byte_3: u8;
//     let mut unaff_bp: i32;
//     let mut unaff_si: i32;
//     let mut unaff_DI: i32;
//     let local_DS__1: *mut u8;
//     let local_res0: *mut u8;
//     let mut byte_1: u8;
//     let bytes_1: *mut u8;
//     let mut char_3: u8;
//     let string_1: *mut libc::c_char;
//
//     byte_3 = param_2 + *(param_3 + unaff_si + -0x7e);
//     string_1 = (param_3 + unaff_si);
//     unsafe {
//         *string_1 = *string_1 + param_1;
//         out(param_2 & 0xff00 | byte_3, param_1);
//         bytes_1 = (param_3 + unaff_DI);
//         byte_1 = *bytes_1;
//         *bytes_1 = *bytes_1 + byte_3;
//         byte_2 = param_1 + CARRY1(byte_1, byte_3);
//         bytes_1 = (param_3 + unaff_si);
//         *bytes_1 = *bytes_1 | byte_2;
//         string_1 = (param_3 + unaff_si);
//         *string_1 = *string_1 + byte_2;
//         char_3 = *(param_3 + unaff_si + -0x7e);
//         string_1 = (param_3 + unaff_si);
//         *string_1 = *string_1 + byte_2;
//         out(param_2 & 0xff00 | (byte_3 + char_3), byte_2);
//         bytes_1 = (unaff_bp + unaff_si);
//         byte_1 = *bytes_1;
//         *bytes_1 = *bytes_1 + param_3;
//         bytes_1 = (param_3 + unaff_si);
//         *bytes_1 = *bytes_1 | byte_2 + CARRY1(byte_1, param_3);
//     }
//     return;
// }

pub fn modify_string_11b8_02b9(param_1: u8, param_2: u16, param_3: u16) {
    let mut byte_2: u8;
    let mut byte_3: u8;
    let mut unaff_bp: i32;
    let mut unaff_si: i32;
    let mut unaff_DI: i32;
    let local_DS__1: Vec<u8>;
    let local_res0: Vec<u8>;
    let mut byte_1: u8;
    let bytes_1: Vec<u8>;
    let mut char_3: u8;
    let string_1: String;

    byte_3 = param_2 + *(param_3 + unaff_si + -0x7e);
    string_1 = (param_3 + unaff_si);
    unsafe {
        *string_1 = *string_1 + param_1;
        out(param_2 & 0xff00 | byte_3, param_1);
        bytes_1 = (param_3 + unaff_DI);
        byte_1 = *bytes_1;
        *bytes_1 = *bytes_1 + byte_3;
        byte_2 = param_1 + CARRY1(byte_1, byte_3);
        bytes_1 = (param_3 + unaff_si);
        *bytes_1 = *bytes_1 | byte_2;
        string_1 = (param_3 + unaff_si);
        *string_1 = *string_1 + byte_2;
        char_3 = *(param_3 + unaff_si + -0x7e);
        string_1 = (param_3 + unaff_si);
        *string_1 = *string_1 + byte_2;
        out(param_2 & 0xff00 | (byte_3 + char_3), byte_2);
        bytes_1 = (unaff_bp + unaff_si);
        byte_1 = *bytes_1;
        *bytes_1 = *bytes_1 + param_3;
        bytes_1 = (param_3 + unaff_si);
        *bytes_1 = *bytes_1 | byte_2 + CARRY1(byte_1, param_3);
    }
    return;
}

pub fn string_fn_1008_e2a4(param_1: u32, param_2: u32, param_3: u32) -> u16 {
    let mut iVar1: i32;
    let mut i_var2: i32;
    let mut u_var3: u32;
    let lVar4: u32;
    let mut u_var5: u32;
    let mut local_a: u16;
    let mut local_8: u16;

    u_var5 = param_2;
    u_var3 = load_string_1010_847e(
        ctx._g_struct_73_1050_14cc,
        (ctx._g_struct_73_1050_14cc >> 0x10),
        0x531,
    );
    iVar1 = pass1_1000_3d7a(u_var3, u_var5);
    if ((iVar1 == 0) || (iVar1 = pass1_1000_3d7a(param_3, param_2), iVar1 == 0)) {
        return 0;
    }
    lVar4 = pass1_1008_e8cc(param_1, param_2, param_3);
    if (lVar4 != 0) {
        iVar1 = (lVar4 + 0xc);
        i_var2 = iVar1 + -1;
        if (i_var2 == 0) {
            return 2;
        }
        if (i_var2 < 1) {
            return 0;
        }
        if (SBORROW2(i_var2, 1)) {
            return 0;
        }
        if (1 < iVar1 + -2) {
            return 0;
        }
    }
    return 1;
}

pub fn load_string_1008_ee56(param_1: u32) {
    load_string_1010_847e(ctx._g_struct_73_1050_14cc, (param_1 + 0x16));
    return;
}

pub fn process_string_1010_184a(param_1: *mut u32, param_2: u32) {
    let mut u_var1: u16;
    let mut i_var2: i32;
    let mut local_6: u16;

    u_var1 = u16_1050_0ecc;
    i_var2 = (u16_1050_0ecc * 6 + 0xeba) * 8;
    unsafe {
        i_var2 = string_ops1::process_string_1000_475e(*(i_var2 + *param_1), *(i_var2 + param_2));
    }
    if (i_var2 == 0) {
        i_var2 = (u_var1 * 6 + 0xebc) * 8;
        unsafe {
            i_var2 = string_ops1::process_string_1000_475e(*(i_var2 + *param_1), *(i_var2 + param_2));
        }
        if (i_var2 == 0) {
            i_var2 = (u_var1 * 6 + 0xebe) * 8;
            unsafe {
                string_ops1::process_string_1000_475e(*(i_var2 + *param_1), *(i_var2 + param_2));
            }
        }
    }
    return;
}

pub fn string_fn_1010_2c34() -> *mut Struct2111 {
    let string_b: String;
    let mut u_var1: u16;
    let struct_a: *mut Struct199;
    let out_buffer: *mut Struct2111;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    out_buffer = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(local_a, 3));
    struct_a = (out_buffer >> 0x10);
    u_var1 = SUB42(out_buffer, 0);
    process_struct_1000_179c(0x80, struct_a);
    load_string_1010_84e0(
        ctx._g_struct_73_1050_14cc,
        (ctx._g_struct_73_1050_14cc >> 0x10),
        0x80,
        out_buffer,
        0x5eb,
    );
    string_ops1::process_string_1000_3cea(out_buffer, 0x105011c8);
    string_b = pass1_1010_e964(u_var1, struct_a);
    string_ops1::process_string_1000_3cea(out_buffer, string_b);
    return out_buffer;
}

pub unsafe fn str_fn_1010_5286(param_1: u16, param_2: u16, param_1_00: u32) {
    let paVar1: *mut Struct493;
    let paVar2: *mut Struct493;
    let mut in_dx: i32;
    let struct_a: *mut Struct199;
    let string_b: String;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    paVar1 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, param_1_00, (param_1_00 >> 0x10));
    struct_a = (in_dx | paVar1);
    if (struct_a == 0x0) {
        return 0;
    }
    paVar2 = paVar1;
    process_struct_1000_179c(0x80, struct_a);
    load_string_1010_84e0(
        ctx._g_struct_73_1050_14cc,
        (ctx._g_struct_73_1050_14cc >> 0x10),
        0x80,
        CONCAT22(struct_a, paVar2),
        0x5eb,
    );
    string_ops1::process_string_1000_3cea(CONCAT22(struct_a, paVar2), s__1050_13ac);
    string_b = pass1_1038_4d28(CONCAT22(in_dx, paVar1));
    string_ops1::process_string_1000_3cea(CONCAT22(struct_a, paVar2), string_b);
    return CONCAT22(struct_a, paVar2);
}

pub unsafe fn str_fn_1010_6034(param_1: *mut Struct432) {
    let u_var1: u8;
    let mut u_var2: i32;
    let extraout_var: u32;


    let local_bx_7: *mut Struct432;
    let mut u_var3: u16;

    u_var3 = (param_1 >> 0x10);
    local_bx_7 = param_1;
    local_bx_7.field_0x1e = 1;
    local_bx_7.field_0x20 = 1;
    local_bx_7.field_0x72 = 1;
    local_bx_7.field_0x74 = 1;
    pass1_1010_60a0(param_1);
    u_var1 = pass1_1000_4906((param_1 & 0xffff0000 | &local_bx_7.field_0x22), 0, 0x40);
    u_var2 = CONCAT31(extraout_var, u_var1);
    load_str_1010_84ac(
        ctx._g_struct_73_1050_14cc,
        (ctx._g_struct_73_1050_14cc >> 0x10),
        0x630,
    );
    local_bx_7.field_0x68 = u_var2;
    local_bx_7.field_0x6a = ctx.dx_reg;
    load_str_1010_84ac(
        ctx._g_struct_73_1050_14cc,
        (ctx._g_struct_73_1050_14cc >> 0x10),
        0x62f,
    );
    local_bx_7.field_0x6c = u_var2;
    local_bx_7.field_0x6e = ctx.dx_reg;
    return;
}

pub unsafe fn write_private_profile_str_1010_62ec(param_1: u32, param_2: u8) {
    write_private_profile_str_1010_5b10(param_1);
    if (param_2 & 1) != 0 {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn string_fn_1010_8018(param_1: &mut Struct446, param_2: u16) {
    let mut iVar1: i32;
    let local_c: &mut Struct446;
    let mut uStack10: u16;

    if (((s_559_bmp_1050_1f9f + 1) + param_2 * 10) != 0) {
        local_c = param_1;
        uStack10 = (param_1 >> 0x10);
        pass1_1010_878c(param_1, ((s_559_bmp_1050_1f9f + 1) + param_2 * 10));
        if (local_c.field_0x67c != 0x0) {
            iVar1 = param_2 * 10;
            string_ops1::string_fn_1008_64c8(
                local_c.field_0x67c,
                CONCAT22(
                    ((s_559_bmp_1050_1f9f + 7) + iVar1),
                    ((s_560_bmp_1050_1fa7 + 1) + iVar1),
                ),
                *((s_559_bmp_1050_1f9f + 5) + iVar1),
                *((s_559_bmp_1050_1f9f + 3) + iVar1),
            );
            return;
        }
    }
    return;
}

/*

u32 __stdcall16far load_string_1010_847e(u16 offset_base,u16 segment,u16 resource_id)
{
  LoadString16(0x3ff,(LPSTR)CONCAT22(segment,offset_base + 0x682),resource_id,g_h_instance_1050_038c);
  return CONCAT22(segment,offset_base + 0x682);
}
*/
pub fn load_string_1010_847e(ctx: &mut AppContext, address: &u32, resource_id: u16) -> String {
    let mut buffer: String = String::new();
    // address + 0x682
    LoadString16(
        ctx.g_h_instance_1050_038c,
        resource_id,
        &buffer,
        0x3ff
    );
    return buffer;
}

pub unsafe fn load_str_1010_84ac(
    in_struct_73_low: *mut Struct73,
    in_struct_73_hi: *mut Struct73,
    resource_id: u16,
) {
    LoadString16(
        0x3ff,
        CONCAT22(in_struct_73_hi, in_struct_73_low + 1),
        resource_id,
        ctx.g_h_instance_1050_038c,
    );
    pass1_fn_1008_60e8(in_struct_73_low + 1, in_struct_73_hi);
    return;
}

pub fn load_string_1010_84e0(a: u16, b: u16, buf_lenout_buffer: &mut string, in_resource_id: u16) {
    let mut resource_id: u16;

    LoadString16(
        buf_len,
        out_buffer,
        in_resource_id,
        ctx.g_h_instance_1050_038c,
    );
    return;
}

pub unsafe fn wsprintf_1010_8c96(param_1: u32, param_2: &mut string, param_3: u32) {
    let mut iVar1: i32;
    let mut u_var2: u32;
    let mut u_var3: u16;
    let paVar4: *mut Struct493;
    let pu_var5: *mut u32;
    let mut in_dx: u16;
    let mut u_var6: i32;
    let mut extraout_dx_04: u16;
    let mut i_var7: i32;
    let mut u_var8: u16;
    let mut u_var9: u16;
    let mut unaff_ss: u16;
    let pu_var10: Vec<u8>;
    let mut u_var11: u32;
    // va_list valist;
    let mut u_var12: u16;
    let mut local_38: u32;
    let mut local_32: u32;
    let mut local_2e: u16;
    let mut local_2c: u16;
    let mut local_2a: u16;
    let mut local_28: u16;
    let mut local_26: u16;
    let mut local_24: u16;
    let mut local_22: u16;
    let mut local_20: u16;
    let mut local_1e: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u32;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    u_var8 = (param_3 >> 0x10);
    i_var7 = param_3;
    iVar1 = (i_var7 + 6);
    valist = param_2;
    if (iVar1 == 0) {
        u_var12 = 0x436;
    } else {
        u_var9 = (param_1 >> 0x10);
        u_var3 = (param_2 >> 0x10);
        if (iVar1 == 1) {
            match (i_var7 + 4) {
                1 | 2 => {
                    u_var2 = (i_var7 + 8);
                    local_a = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, u_var2, (u_var2 >> 0x10));
                    local_10 = (local_a + 0xc);
                    local_c = (local_a + 0x10);
                    local_6 = &local_10;
                    if (0 < local_c) {
                        load_string_1010_847e(
                            ctx._g_struct_73_1050_14cc,
                            (ctx._g_struct_73_1050_14cc >> 0x10),
                            0x437,
                        );
                        local_4 = ctx.dx_reg;
                        wsprintf16(
                            valist,
                            CONCAT22(local_6, u_var3),
                            CONCAT22(local_c, ctx.dx_reg),
                        );
                        return;
                    }
                }
                3 => {
                    u_var2 = (i_var7 + 8);
                    paVar4 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, u_var2, (u_var2 >> 0x10));
                    local_10 = &paVar4.field_0xc;
                    local_c = &paVar4.field_0x10;
                    if (0 < local_c) {
                        local_c = 0;
                        pu_var10 = pass1_1030_73a8(CONCAT22(in_dx, paVar4));
                        u_var11 = pass1_1028_bb24(pu_var10);
                        local_8 = (u_var11 >> 0x10);
                        local_a = u_var11;
                        pu_var5 = &local_10;
                        pass1_1030_627e(_PTR_LOOP_1050_5740, CONCAT22(unaff_ss, pu_var5), u_var11);
                        u_var12 = ctx.dx_reg;
                        local_6 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, pu_var5, ctx.dx_reg);
                        u_var2 = (param_1 + 10);
                        pass1_1010_c3c2(u_var2, (u_var2 >> 0x10), 0, CONCAT22(u_var12, local_6));
                        _local_2e = CONCAT22(ctx.dx_reg, local_6);
                        paVar4 = local_6;
                        load_string_1010_847e(
                            ctx._g_struct_73_1050_14cc,
                            (ctx._g_struct_73_1050_14cc >> 0x10),
                            0x439,
                        );
                        local_4 = ctx.dx_reg;
                        wsprintf16(
                            valist,
                            CONCAT22(local_6, u_var3),
                            CONCAT22(paVar4, ctx.dx_reg),
                        );
                        u_var6 = ctx.dx_reg;
                        // goto LAB_1010_8def;
                    }
                }
                _ => {}
                // goto switchD_1010_8e11_caseD_4;
                5 | 8 | 9 | 0xb => {
                    u_var12 = 0x43a;
                    // goto LAB_1010_8ea5;
                }
            }
            u_var12 = 0x438;
        } else {
            if (iVar1 == 2) {
                iVar1 = (i_var7 + 4);
                if ((iVar1 == 4) || (iVar1 == 0xc)) {
                    u_var2 = (i_var7 + 8);
                    local_6 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, u_var2, (u_var2 >> 0x10));
                    u_var2 = (param_1 + 10);
                    pass1_1010_c3c2(u_var2, (u_var2 >> 0x10), 0, CONCAT22(in_dx, local_6));
                    _local_2e = CONCAT22(ctx.dx_reg, local_6);
                    paVar4 = local_6;
                    load_string_1010_847e(
                        ctx._g_struct_73_1050_14cc,
                        (ctx._g_struct_73_1050_14cc >> 0x10),
                        0x43b,
                    );
                    local_4 = extraout_dx_04;
                    wsprintf16(
                        valist,
                        CONCAT22(local_6, u_var3),
                        CONCAT22(paVar4, extraout_dx_04),
                    );
                    u_var6 = ctx.dx_reg;
                    // LAB_1010_8def:
                    error_check_1000_17ce((_local_2e & 0xffff | u_var6 << 0x10));
                    return;
                }
                u_var12 = 0x43c;
            } else {
                u_var12 = 0x5d9;
            }
        }
    }
    // LAB_1010_8ea5:
    load_string_1010_84e0(
        ctx._g_struct_73_1050_14cc,
        (ctx._g_struct_73_1050_14cc >> 0x10),
        0x3ff,
        param_2,
        u_var12,
    );
    // switchD_1010_8e11_caseD_4:
    return;
}

pub unsafe fn str_fn_1010_c446(param_1: u32, param_2: u32, param_3: u32) {
    let mut iVar1: i32;
    let pc_var2: String;
    let in_dx: *mut Struct199;
    let mut u_var3: u32;
    let pcVar4: String;
    let mut a: u16;
    let mut b: u16;
    let mut in_resource_id: u16;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: u32;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;

    local_6 = param_2;
    if (param_2 == 0) {
        process_struct_1000_179c(0x100, in_dx);
        local_6 = param_2 & 0xffff | ZEXT24(in_dx) << 0x10;
    }
    u_var3 = pass1_1030_73a8(param_3);
    pass1_1010_dd5e(param_1, (param_1 >> 0x10), param_3);
    iVar1 = (u_var3 + 0x12);
    if (6 < iVar1 - 3) {
        return;
    }
    a = ctx._g_struct_73_1050_14cc;
    b = (ctx._g_struct_73_1050_14cc >> 0x10);
    match (iVar1) {
        _ => in_resource_id = 0x5f4,
        6 => {
            load_string_1010_84e0(a, b, 0x3ff, local_6, 0x531);
            local_16 = string_ops1::get_string_index_1000_3da4(local_6);
            pc_var2 = local_16;
            load_string_1010_847e(
                ctx._g_struct_73_1050_14cc,
                (ctx._g_struct_73_1050_14cc >> 0x10),
                0x61e,
            );
            pcVar4 = s_____s__lu_1050_38d7;
            // goto LAB_1010_c4f9;
        }
        7 | 9 => in_resource_id = 0x531,
        8 => {
            load_string_1010_84e0(a, b, 0x3ff, local_6, 0x5f5);
            local_16 = string_ops1::get_string_index_1000_3da4(local_6);
            pc_var2 = local_16;
            load_string_1010_847e(
                ctx._g_struct_73_1050_14cc,
                (ctx._g_struct_73_1050_14cc >> 0x10),
                0x61e,
            );
            pcVar4 = s_____s__lu_1050_38cd;
            // LAB_1010_c4f9:
            string_ops1::string_fn_1000_3f9c(
                (local_6 + local_16),
                (local_6 >> 0x10),
                pcVar4,
                &ctx.g_alloc_addr_1050_1050,
                pc_var2,
            );
            return;
        }
    }
    load_string_1010_84e0(a, b, 0x3ff, local_6, in_resource_id);
    return;
}

pub fn load_string_1010_de78(param_1: String, param_2: u32) {
    let mut in_ax: i32;
    let mut in_resource_id: u16;

    *(param_1 + 0x13c) = 0;
    pass1_1030_809c(param_2);
    if (in_ax == 0) {
        in_resource_id = 0x531;
    } else {
        in_resource_id = 0x5f4;
    }
    load_string_1010_84e0(
        ctx._g_struct_73_1050_14cc,
        (ctx._g_struct_73_1050_14cc >> 0x10),
        0x3ff,
        (param_1 & 0xffff0000 | (param_1 + 0x13c)),
        in_resource_id,
    );
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn wsprintf_1018_35b0(param_1: *mut Struct298) {
    let pu_var1: *mut u32;
    let pi_var2: *mut i32;
    let mut u_var3: i32;
    let ppcVar4: fn();
    let mut u_var5: u32;
    let mut u_var6: u16;
    let pu_var7: *mut u32;
    let mut u_var8: u16;
    let mut u_var9: u16;
    let mut unaff_ss: u16;
    let mut bVar10: bool;
    let mut u_var11: u32;
    let mut local_1c: u32;
    let mut local_18: u32;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    u_var11 = pass1_1030_8326();
    local_4 = (u_var11 >> 0x10);
    local_6 = u_var11;
    u_var9 = (param_1 >> 0x10);
    u_var8 = param_1;
    pu_var1 = (u_var8 + 0x140);
    unsafe {
        bVar10 = *pu_var1 < local_4;
    }
    let pu_var1_val = unsafe { *pu_var1 };
    if ((bVar10) || (bVar10 || pu_var1_val == local_4 && ((u_var8 + 0x13e) < local_6))) {
        u_var6 = (u_var8 + 0x13c);
        if ((u_var8 + 0x13a) < u_var6) {
            string_fn_1018_3b9e(param_1, (u_var8 + 0x12e));
            local_a = u_var6;
            local_8 = ctx.dx_reg;
            pass1_1018_427c(u_var8, u_var9);
            _local_e = CONCAT22(ctx.dx_reg, u_var6);
            pass1_1018_3e8c(
                u_var8,
                u_var9,
                CONCAT22(unaff_ss, &local_12),
                CONCAT22(unaff_ss, &local_10),
            );
            if (_local_e < local_12) {
                local_12 = local_e;
            }
            u_var3 = (u_var8 + 0x138);
            pu_var7 = (u_var8 + 0x136);
            if ((u_var3 | pu_var7) != 0) {
                unsafe {
                    ppcVar4 = *pu_var7;
                }
                (**ppcVar4)(0x30, pu_var7, u_var3, 1);
            }
            pass1_1018_435e(
                u_var8,
                (param_1 >> 0x10),
                _local_e,
                (_local_e >> 0x10),
                local_12,
                local_10,
            );
            (u_var8 + 0x136) = pu_var7;
            (u_var8 + 0x138) = ctx.dx_reg;
            pi_var2 = (u_var8 + 0x13a);
            unsafe {
                *pi_var2 = *pi_var2 + 1;
            }
            u_var5 = (u_var8 + 0x136);
            u_var5 = (u_var5 + 8);
            wsprintf16(
                (u_var8 + 0x22),
                CONCAT22(0x4165, u_var9),
                CONCAT13((u_var5 >> 8), CONCAT12(u_var5, 0x1050)),
            );
            return;
        }
        (u_var8 + 0x13e) = local_6;
        (u_var8 + 0x140) = local_4;
        (u_var8 + 0x13a) = 0;
        pass1_fn_1008_612e(8, 0xc);
        (u_var8 + 0x13c) = local_6;
    }
    return;
}

pub fn string_fn_1018_3b9e(in_struct_a: *mut Struct298, in_struct_b: *mut Struct566) {
    let ppVar1: *mut pass1_struct_2;
    let local_AX_89: Vec<u8>;
    let ptr_a_1: Vec<u8>;
    let mut u_var2: u16;
    let struct_a_2: *mut Struct298;
    let struct_a_1: *mut Struct298;
    let mut u_var3: u32;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_6: u32;
    let struct_b_1: *mut Struct199;

    let mut local_6: u32 = 0;
    // struct_a_1 = (in_struct_a >> 0x10);
    // struct_a_2 = in_struct_a;
    struct_b_1 = in_struct_a.Struct199_ptr_x122;
    u_var3 = pass1_1008_e852(struct_b_1, (struct_b_1 >> 0x10), struct_a_2.u32_x126);
    ptr_a_1 = (u_var3 >> 0x10);
    ptr_var1 = pass1_1030_8344(
        ctx._g_bool_1050_5748,
        (ctx._g_bool_1050_5748 >> 0x10),
        u_var3,
    );
    _local_e = CONCAT22(ptr_a_1, ptr_var1);
    match (in_struct_b) {
        0x188 => {
            if (&struct_a_2.astruct99_0xa == 0) {
                process_struct_1008_d3ae(
                    (in_struct_a & 0xffff | ZEXT24(struct_a_1) << 0x10),
                    _local_e,
                );
            }
            local_AX_89 = struct_a_2.astruct99_0xa;
            u_var2 = struct_a_2.field_0xc
        }
        0x189 => {
            if (&struct_a_2.field_0xe == 0) {
                string_ops1::wsprintf_1008_d4f6(
                    (in_struct_a & 0xffff | ZEXT24(struct_a_1) << 0x10),
                    _local_e,
                );
            }
            local_AX_89 = struct_a_2.field_0xe;
            u_var2 = struct_a_2.field_0x10
        }
        0x18a => {
            if (&struct_a_2.field_0x12 == 0) {
                string_ops1::wsprintf_1008_d1c6(
                    (in_struct_a & 0xffff | ZEXT24(struct_a_1) << 0x10),
                    _local_e,
                );
            }
            local_AX_89 = struct_a_2.field_0x12;
            u_var2 = struct_a_2.field_0x14
        }
        0x18b => {
            if (&struct_a_2.field_0x16 == 0) {
                pass1_1008_cfa0(
                    (in_struct_a & 0xffff | ZEXT24(struct_a_1) << 0x10),
                    _local_e,
                );
            }
            local_AX_89 = struct_a_2.field_0x16;
            u_var2 = struct_a_2.field_0x18
        }
        0x18c => {
            if (&struct_a_2.field_0x1a == 0) {
                process_struct_1008_cda2(
                    (in_struct_a & 0xffff | ZEXT24(struct_a_1) << 0x10),
                    _local_e,
                );
            }
            local_AX_89 = struct_a_2.field_0x1a;
            u_var2 = struct_a_2.field_0x1c
        }
        0x18d => {
            if (&struct_a_2.field_0x1e == 0) {
                process_struct_1008_cbc4(
                    (in_struct_a & 0xffff | ZEXT24(struct_a_1) << 0x10),
                    _local_e,
                );
            }
            local_AX_89 = struct_a_2.field_0x1e;
            u_var2 = struct_a_2.field_0x20
        } // default:
          // goto switchD_1018_3cbf_caseD_6;
    }
    local_6 = CONCAT22(u_var2, local_AX_89);
    // switchD_1018_3cbf_caseD_6:
    return local_6;
}

pub fn pass1_1040_29c2(param_1: *mut Struct346, param_2: u32, param_3: Vec<u8>) -> *mut Struct346 {
    let mut iVar1: i32;
    let mut u_var2: u16;
    let mut u_var3: u32;

    process_struct_1040_b0bc(param_1, param_2, CONCAT22(param_3, 0x157));
    u_var2 = (param_1 >> 0x10);
    iVar1 = param_1;
    param_1 = (s_add74_wav_1050_2e20 + 6);
    (iVar1 + 2) = &ctx.PTR_LOOP_1050_1040;
    u_var3 = load_str_1010_84ac(
        ctx._g_struct_73_1050_14cc,
        (ctx._g_struct_73_1050_14cc >> 0x10),
        0x64b,
    );
    (iVar1 + 0x94) = u_var3;
    (iVar1 + 0x96) = (u_var3 >> 0x10);
    u_var3 = load_str_1010_84ac(
        ctx._g_struct_73_1050_14cc,
        (ctx._g_struct_73_1050_14cc >> 0x10),
        0x64a,
    );
    (iVar1 + 0x98) = u_var3;
    (iVar1 + 0x9a) = (u_var3 >> 0x10);
    return param_1;
}

pub unsafe fn load_str_1038_81be(param_1: u32) {
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
        0x3ff,
        CONCAT22(unaff_ss, local_206),
        0x57b,
    );
    load_string_1010_84e0(
        ctx._g_struct_73_1050_14cc,
        (ctx._g_struct_73_1050_14cc >> 0x10),
        0x3ff,
        CONCAT22(in_dx, in_ax),
        0x80b,
    );
    load_string_1010_84e0(
        ctx._g_struct_73_1050_14cc,
        (ctx._g_struct_73_1050_14cc >> 0x10),
        0x3ff,
        CONCAT22(unaff_ss, local_104),
        0x80c,
    );
    string_ops1::process_string_1000_3cea(CONCAT22(in_dx, in_ax), CONCAT22(unaff_ss, local_104));
    load_string_1010_84e0(
        ctx._g_struct_73_1050_14cc,
        (ctx._g_struct_73_1050_14cc >> 0x10),
        0x3ff,
        CONCAT22(unaff_ss, local_104),
        0x80d,
    );
    string_ops1::process_string_1000_3cea(CONCAT22(in_dx, in_ax), CONCAT22(unaff_ss, local_104));
    load_string_1010_84e0(
        ctx._g_struct_73_1050_14cc,
        (ctx._g_struct_73_1050_14cc >> 0x10),
        0x3ff,
        CONCAT22(unaff_ss, local_104),
        0x80e,
    );
    string_ops1::process_string_1000_3cea(CONCAT22(in_dx, in_ax), CONCAT22(unaff_ss, local_104));
    load_string_1010_84e0(
        ctx._g_struct_73_1050_14cc,
        (ctx._g_struct_73_1050_14cc >> 0x10),
        0x3ff,
        CONCAT22(unaff_ss, local_104),
        0x80f,
    );
    string_ops1::process_string_1000_3cea(CONCAT22(in_dx, in_ax), CONCAT22(unaff_ss, local_104));
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
        0x810,
    );
    load_string_1010_84e0(
        ctx._g_struct_73_1050_14cc,
        (ctx._g_struct_73_1050_14cc >> 0x10),
        0x3ff,
        CONCAT22(unaff_ss, local_104),
        0x811,
    );
    string_ops1::process_string_1000_3cea(CONCAT22(in_dx, in_ax), CONCAT22(unaff_ss, local_104));
    load_string_1010_84e0(
        ctx._g_struct_73_1050_14cc,
        (ctx._g_struct_73_1050_14cc >> 0x10),
        0x3ff,
        CONCAT22(unaff_ss, local_104),
        0x812,
    );
    string_ops1::process_string_1000_3cea(CONCAT22(in_dx, in_ax), CONCAT22(unaff_ss, local_104));
    MessageBox16(
        0,
        CONCAT22(unaff_ss, local_206),
        CONCAT22(in_dx, in_ax),
        (param_1 + 6),
    );
    error_check_1000_17ce(CONCAT22(in_dx, in_ax));
    return;
}

pub fn pass1_1038_0ba6(param_1: *mut Struct500) -> *mut Struct500 {
    let local_bx_15: *mut Struct1053;
    let mut unaff_bp: u16;
    let mut u_var1: u16;
    let ppVar2: *mut Struct2111;

    pass1_1028_d1dc(param_1, (s_fem36_wav_1050_270c + 3));
    u_var1 = (param_1 >> 0x10);
    local_bx_15 = param_1;
    &local_bx_15.field_0x108 = 0;
    param_1.a = s_198_flc_1050_1c2e;
    local_bx_15.field_0x2 = &PTR_LOOP_1050_1038;
    string_ops1::copy_string_1000_3d3e(
        (param_1 & 0xffff0000 | &local_bx_15.field_0x8),
        s_SCMove_1050_59d8,
    );
    ppVar2 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(unaff_bp, 2));
    local_bx_15.field_0x108 = ppVar2;
    local_bx_15.field_0x10a = (ppVar2 >> 0x10);
    return param_1;
}

pub fn pass1_1030_eb50(struct_a: *mut Struct500) -> *mut Struct500 {
    pass1_1028_d1dc(struct_a, s_547_bmp_1050_1f3f);
    struct_a.a = 0xecb2;
    (struct_a + 2) = 0x1030;
    string_ops1::copy_string_1000_3d3e(
        (struct_a & 0xffff0000 | (struct_a + 8)),
        s_SCMines_1050_59c6,
    );
    return struct_a;
}

pub unsafe fn pas1_1030_e8a0(param_1: *mut Struct500, param_2: u32, param_3: u16, param_4: u32) {
    let local_bx_19: *mut Struct1035;
    let pc_var1: String;

    pass1_1028_d1dc(param_1, (s_fem36_wav_1050_270c + 4));
    pc_var1 = (param_1 >> 0x10);
    local_bx_19 = param_1;
    local_bx_19.field_0x108 = param_2;
    local_bx_19.field_0x10c = param_4;
    local_bx_19.field_0x110 = param_3;
    param_1.a = 0xeb40;
    local_bx_19.field_0x2 = 0x1030;
    string_ops1::string_fn_1000_3f9c(
        &local_bx_19.field_0x8,
        pc_var1,
        s_SCMoveBas_to_0x_08lx_1050_59b0,
        &ctx.g_alloc_addr_1050_1050,
        local_bx_19.field_0x10c,
    );
    return;
}

pub fn pass1_1030_e79a(param_1: *mut Struct500) -> *mut Struct500 {
    pass1_1028_d1dc(param_1, 0xf9f);
    param_1.a = 0xe890;
    (param_1 + 2) = 0x1030;
    string_ops1::copy_string_1000_3d3e(
        (param_1 & 0xffff0000 | (param_1 + 8)),
        s_SCKillRebelColony_1050_599e,
    );
    return param_1;
}

pub fn pass1_1030_e63e(param_1: *mut Struct500, param_2: u16) -> *mut Struct500 {
    let local_8: *mut Struct500;
    let local_6: *mut Struct500;

    local_8 = param_1;
    local_6 = (param_1 >> 0x10);
    pass1_1028_d1dc(param_1, 0xf9f);
    &local_8.field_0x108 = param_2;
    param_1.a = 0xe78a;
    local_8.b = 0x1030;
    string_ops1::copy_string_1000_3d3e(
        (param_1 & 0xffff0000 | ZEXT24(&local_8.field_0x8)),
        s_SCKillColony_1050_5990,
    );
    return param_1;
}

pub unsafe fn pass1_1030_e4fa(param_1: *mut Struct500, param_2: u32) {
    let local_bx_19: *mut Struct500;
    let pc_var1: String;

    pass1_1028_d1dc(param_1, 0x3e80);
    pc_var1 = (param_1 >> 0x10);
    local_bx_19 = param_1;
    local_bx_19.field_0x108 = param_2;
    param_1.a = 0xe62e;
    local_bx_19.b = 0x1030;
    string_ops1::string_fn_1000_3f9c(
        &local_bx_19.field_0x8,
        pc_var1,
        s_SCKillBldg__0x_08lx_1050_597c,
        &ctx.g_alloc_addr_1050_1050,
        local_bx_19.field_0x108,
    );
    return;
}

pub fn pass1_1030_e09e(param_1: *mut Struct500) -> *mut Struct500 {
    pass1_1028_d1dc(param_1, s_fem133_wav_1050_2af7);
    param_1.a = 0xe2ae;
    (param_1 + 2) = 0x1030;
    string_ops1::copy_string_1000_3d3e(
        (param_1 & 0xffff0000 | (param_1 + 8)),
        s_SCAiInput_1050_5972,
    );
    return param_1;
}

pub fn pass1_1030_dfcc(param_1: Vec<u8>) -> u16 {
    let mut local_4: u16;
    let temp_5f890d5eff: *mut Struct1016;

    temp_5f890d5eff = (param_1 + 0xc);
    if (temp_5f890d5eff == s_New_failed_in_Op__Op__DialogHand_1050_0073) {
        // LAB_1030_dfde:
        local_4 = 1;
    } else {
        if (temp_5f890d5eff != (s_New_failed_in_Op__Op__DialogHand_1050_0073 + 1)) {
            if (temp_5f890d5eff == (s_New_failed_in_Op__Op__DialogHand_1050_0073 + 2)) {
                return 3;
            }
            if (temp_5f890d5eff == (s_New_failed_in_Op__Op__DialogHand_1050_0073 + 4)) {}
            // goto LAB_1030_dfde;
            if (temp_5f890d5eff != (s_New_failed_in_Op__Op__DialogHand_1050_0073 + 5)) {
                return 0;
            }
        }
        local_4 = 2;
    }
    return local_4;
}

pub unsafe fn pass1_1030_b9da(param_1: *mut Struct965, param_2: u32) {
    let plVar1: *mut long;
    let mut u_var2: u32;
    let mut in_eax: u32;
    let mut u_var3: u32;
    let struct_a: *mut Struct199;
    let mut u_var4: i32;
    let mut u_var5: i32;
    let mut i32_var6: i32;
    let mut in_edx: u32;
    let local_bx_5: *mut Struct965;
    let mut u_var7: u16;
    let mut u_var8: u16;
    let mut u_var9: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    struct_a = in_edx;
    u_var7 = (param_1 >> 0x10);
    local_bx_5 = param_1;
    if (&local_bx_5.field_0xe == 0) {
        process_struct_1000_179c(10, struct_a);
        u_var4 = struct_a | in_eax;
        in_edx = u_var4;
        if (u_var4 == 0) {
            &local_bx_5.field_0xe = 0;
        } else {
            pass1_1020_ba3e((in_eax & 0xffff | ZEXT24(struct_a) << 0x10), 5, 5);
            &local_bx_5.field_0xe = in_eax;
            local_bx_5.field_0x10 = in_edx;
        }
        local_bx_5.field_0x12 = 0;
    }
    local_4 = 4;
    while (u_var4 = in_edx, local_4 < 0xe) {
        pass1_1030_7c28(param_2, local_4);
        u_var5 = u_var4 | in_eax;
        in_edx = u_var5;
        if u_var5 != 0 {
            u_var3 = 100 - local_bx_5.field_0x12;
            in_edx = u_var3 >> 0x10;
            local_c = (in_eax & 0xffff);
            if (in_eax & 0xffff | u_var4 << 0x10) < u_var3 {
                u_var3 = in_eax & 0xffff;
                in_edx = u_var4;
            }
            u_var5 = u_var3;
            i32_var6 = in_edx;
            in_eax = u_var3 & 0xffff | in_edx << 0x10;
            pass1_1030_7d1c(
                param_2,
                local_c - u_var5,
                CONCAT22(local_4, (u_var4 - i32_var6) - (local_c < u_var5)),
            );
            u_var2 = &local_bx_5.field_0xe;
            pass1_1020_bb8a(u_var2, (u_var2 >> 0x10), u_var5, i32_var6, local_4);
            plVar1 = &local_bx_5.field_0x12;
            unsafe {
                *plVar1 = *plVar1 + in_eax;
            }
            u_var2 = (param_2 + 4);
            u_var8 = u_var2;
            u_var9 = (u_var2 >> 0x10);
            pass1_1020_c0ca(local_4);
            wvsprintf_FUN_1030_840a(
                s_truck_0x_08lx_loaded__ld_of__s_f_1050_576c,
                &ctx.g_alloc_addr_1050_1050,
                local_bx_5.field_0x4,
                u_var5,
                i32_var6,
                in_eax,
                in_edx,
                u_var8,
                u_var9,
            );
            if 99 < local_bx_5.field_0x12 {
                break;
            }
        }
        local_4 = local_4 + 1;
    }
    if local_bx_5.field_0x12 != 0 {
        return;
    }
    return;
}

pub unsafe fn pass1_1030_bb0e(param_1: u32, param_2: *mut Struct493) {
    let mut u_var1: u32;
    let mut u_var2: i32;
    let mut u_var3: u16;
    let mut u_var4: u32;
    let mut in_dx: i32;

    let mut u_var5: i32;
    let mut u_var6: i32;
    let mut u_var7: u16;
    let mut u_var8: u16;
    let mut local_c: u32;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    u_var2 = pass1_1030_7bee(param_2);
    if (u_var2 != 0) {
        return;
    }
    pass1_1030_b9b2(param_1);
    _local_6 = CONCAT22(in_dx, u_var2);
    if (in_dx | u_var2) != 0 {
        local_8 = 4;
        u_var5 = in_dx | u_var2;
        while local_8 < 0x25 {
            u_var4 = pass1_1020_bae6(u_var2, CONCAT22(local_8, in_dx));
            u_var6 = u_var5 | u_var4;
            if u_var6 != 0 {
                pass1_1030_7ddc(param_2, u_var4 & 0xffff | u_var5 << 0x10, local_8);
                u_var3 = pass1_1030_7bee(param_2);
                if (u_var3 != 0) {
                    return;
                }
                u_var1 = (param_2 + 4);
                u_var7 = u_var1;
                u_var8 = (u_var1 >> 0x10);
                pass1_1020_c0ca(local_8);
                wvsprintf_FUN_1030_840a(
                    s_truck_0x_08lx_unloaded__ld_of__s_1050_5798,
                    &ctx.g_alloc_addr_1050_1050,
                    (param_1 + 4),
                    u_var4,
                    u_var5,
                    u_var3,
                    u_var6,
                    u_var7,
                    u_var8,
                );
                pass1_1020_bb8a(u_var2, in_dx, 0, 0, local_8);
                u_var6 = ctx.dx_reg;
            }
            local_8 = local_8 + 1;
            u_var5 = u_var6;
        }
        if _local_6 != 0x0 {
            pass1_1020_ba7e(_local_6);
            error_check_1000_17ce(_local_6);
        }
    }
    return;
}

pub fn wvsprintf_FUN_1030_840a(param_1: u32) {
    let pu_var1: Vec<u8>;
    let mut in_dx: u16;
    let mut unaff_ss: u16;
    let mut local_106: [u8; 256];
    let mut local_6: u16;
    let mut local_4: u16;

    if ctx.PTR_LOOP_1050_574c != 0x0 {
        pu_var1 = &stack0x0008;
        local_6 = pu_var1;
        local_4 = ctx.stack_seg_reg;
        if (PTR_LOOP_1050_5750 == 0x0) {
            pass1_fn_1000_2b3c(
                ctx.s_simres_out_1050_5758,
                &ctx.g_alloc_addr_1050_1050,
                0x5756,
                &ctx.g_alloc_addr_1050_1050,
                ctx.dx_reg,
            );
            ctx._PTR_LOOP_1050_5752 = CONCAT22(ctx.dx_reg, pu_var1);
            ctx.PTR_LOOP_1050_5750 = (&ctx.PTR_LOOP_1050_0000 + 1);
        }
        wvsprintf16(
            local_6,
            CONCAT22(param_1, local_4),
            CONCAT22(local_106, (param_1 >> 0x10)),
        );
        pass1_fn_1000_2b5c(
            ctx._PTR_LOOP_1050_5752,
            ctx.s__s_1050_5763,
            &ctx.g_alloc_addr_1050_1050,
        );
        pass1_fn_1000_2f48(ctx._PTR_LOOP_1050_5752, (ctx._PTR_LOOP_1050_5752 >> 0x10));
    }
    return;
}

pub unsafe fn pass1_1030_5ff6(struct_a: &mut Address<Struct912>) {
    let mut ppaVar1: Address<Struct913>;
    let mut ppc_var2: fn();
    let mut u_var3: u32;
    let in_ax: *mut u16;
    let ppVar4: *mut pass1_struct_2;
    let pcVar5: String;
    let mut u_var6: u32;
    let pa_var7: *mut Struct199;

    let struct_b: *mut Struct912;
    let struct_b_hi: *mut Struct912;
    let mut u_var8: u16;
    let mut local_6c: [u8; 88];
    let mut local_14: u32;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let local_4: *mut Struct913;

    struct_b_hi = (struct_a >> 0x10);
    struct_b = struct_a;
    pa_var7 = in_dx;
    if (&struct_b.field_0xc == 0) {
        process_struct_1000_179c(0x18, in_dx);
        pa_var7 = (in_dx | in_ax);
        if (pa_var7 == 0x0) {
            &struct_b.field_0xc = 0;
        } else {
            pass1_1030_1cd8(CONCAT22(in_dx, in_ax), 5, 5);
            struct_b.field_0xc = in_ax;
            &struct_b.field_0xe = ctx.dx_reg;
            pa_var7 = ctx.dx_reg;
        }
    }
    local_4 = 0x0;
    while (
        u_var3 = struct_b.field_0x10,
        ppaVar1 = (u_var3 + 10),
        local_4 <= *ppaVar1 && *ppaVar1 != local_4,
    ) {
        u_var3 = struct_b.field_0x10;
        u_var3 = (u_var3 + 0xc);
        u_var6 = SEXT24((u_var3 + local_4 * 2));
        pass1_1028_e2e0(ctx._PTR_LOOP_1050_65e2);
        _local_c = u_var6 & 0xffff | ZEXT24(pa_var7) << 0x10;
        ppc_var2 = (&struct_b.field_0xc + 8);
        ppc_var2(
            &PTR_LOOP_1050_1028,
            &struct_b.field_0xc,
            u_var6,
            pa_var7,
            local_4,
            0,
        );
        pa_var7 = ctx.dx_reg;
        ppVar4 = pass1_1030_8344(
            ctx._g_bool_1050_5748,
            (ctx._g_bool_1050_5748 >> 0x10),
            _local_c,
        );
        _local_10 = CONCAT22(pa_var7, ppVar4);
        local_14 = &ppVar4.field_0x10;
        if ((local_14 + 2) == 0) {
            string_ops1::string_fn_1000_3f9c(
                local_6c,
                unaff_ss,
                ctx.s__s__d_1050_573a,
                &ctx.g_alloc_addr_1050_1050,
                struct_b.field_0x10,
            );
            pcVar5 = local_6c;
            pass1_fn_1008_60e8(pcVar5);
            u_var8 = (local_14 >> 0x10);
            *(local_14 + 2) = pcVar5;
            (local_14 + 4) = pa_var7;
        }
        local_4 = &local_4.field_0x1;
    }
    return;
}

pub unsafe fn pass1_1030_532e(param_1: *mut Struct500, param_2: u32) {
    let local_struct_1: *mut Struct500;
    let pc_var1: String;
    let mut local_a: u16;

    pass1_1028_d1dc(param_1, 0x32c7);
    pc_var1 = (param_1 >> 0x10);
    local_struct_1 = param_1;
    local_struct_1.field_0x108 = param_2;
    param_1.a = 0x55ee;
    local_struct_1.b = 0x1030;
    string_ops1::string_fn_1000_3f9c(
        &local_struct_1.field_0x8,
        pc_var1,
        s_SCSelect__u___d_1050_5726,
        &ctx.g_alloc_addr_1050_1050,
        &local_struct_1.c,
    );
    return;
}

pub fn pass1_1030_521c(struct_a: *mut Struct500, param_2: u32) {
    let struct_b: &Struct894;
    let pc_var1: String;

    pass1_1028_d1dc(struct_a, 0x32c7);
    pc_var1 = (struct_a >> 0x10);
    struct_b = struct_a;
    struct_b.field_0x108 = param_2;
    struct_a.a = 0x55fe;
    struct_b.field_0x2 = 0x1030;
    string_ops1::string_fn_1000_3f9c(
        &struct_b.field_0x8,
        pc_var1,
        s_SCGenKids_0x_08lx_1050_5714,
        &ctx.g_alloc_addr_1050_1050,
        param_2,
    );
    return;
}

pub unsafe fn pass1_1030_5164(
    string_a: &mut String,
    string_b: &mut String,
) -> String {
    let mut u_var1: i32 = 0;
    let mut i_var2: i32 = 0;
    let mut lVar3: u32 = 0;
    let mut local_e: u16 = 0;
    let mut local_c: u16 = 0;
    let mut array_a: Vec<u8> = Vec::new();

    pass1_1008_5784(CONCAT22(ctx.stack_seg_reg, array_a), (string_a + 0x568));
    while {
        let var7 = CONCAT(ctx.stack_seg_reg, array_a);
        lVar3 = pass1_1008_5b12(var7);
        if (lVar3 == 0) {
            return string_b.clone();
        }
        u_var1 = string_a + 0x168;
        string_ops1::copy_string_1000_3d3e((string_a & 0xffff0000 | u_var1), *(lVar3 + 4));
        string_ops1::process_string_1000_3cea((string_a & 0xffff0000 | u_var1), string_b);
        i_var2 = dos3_call_1000_51aa(u_var1);
        i_var2 != 0
    } {}
    return (string_a & 0xffff0000 | u_var1);
}

pub fn pass1_1030_4dbc(param_1: u32, param_2: u32, param_3: libc::c_long) {
    let plVar1: *mut long;
    let pu_var2: *mut u32;
    let lVar3: u32;
    let mut u_var4: i32;
    let local_bx_9: &Struct891;
    let mut u_var5: u16;
    let mut local_6: u32;

    local_bx_9 = param_1;
    u_var5 = (param_1 >> 0x10);
    if (0 < param_3) {
        &local_bx_9.field_0x160 = param_2;
        local_bx_9.field_0x164 = param_3;
    }
    if ((&local_bx_9.field_0x160 == 0)
        || (
            lVar3 = local_bx_9.field_0x164,
            plVar1 = &local_bx_9.field_0x164,
            unsafe { *plVar1 = *plVar1 + -1 },
            lVar3 == 0,
        ))
    {
        &local_bx_9.field_0x160 = 0;
    } else {
        u_var4 = string_ops1::get_string_index_1000_3da4(*&local_bx_9.field_0x160);
        pu_var2 = &local_bx_9.field_0x160;
        unsafe {
            *pu_var2 = *pu_var2 + u_var4 + 2;
        }
    }
    return;
}

pub unsafe fn pass1_1030_4594(param_1: u16, param_2: u16, param_1_00: i32) {
    let mut u_var1: u16;
    let mut u_var2: u32;
    let in_dx: *mut Struct199;

    let local_bx_64: *mut Struct883;
    let mut i_var3: i32;
    let mut u_var4: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u32;
    let mut local_4: u16;

    u_var2 = (param_1_00 - 1);
    process_struct_1000_179c(0x10, in_dx);
    local_8 = u_var2 & 0xffff | ZEXT24(in_dx) << 0x10;
    if ((in_dx | u_var2) == 0) {
        local_8 = 0;
    } else {
        zero_list_1008_3e38(CONCAT22(in_dx, u_var2 + 4));
        u_var2 = local_8;
    }
    u_var1 = u_var2;
    local_bx_64 = ((param_1_00 - 1) * 0x12);
    load_str_1010_84ac(
        ctx._g_struct_73_1050_14cc,
        (ctx._g_struct_73_1050_14cc >> 0x10),
        (local_bx_64 + 0x51b8),
    );
    u_var4 = (local_8 >> 0x10);
    i_var3 = local_8;
    local_8 = u_var1;
    (i_var3 + 2) = ctx.dx_reg;
    (i_var3 + 10) = (local_bx_64 + 0x51ba);
    pass1_1008_3e76(
        (local_8 & 0xffff0000 | (i_var3 + 4)),
        (local_bx_64 + 0x51c0),
        (local_bx_64 + 0x51be),
        (local_bx_64 + 0x51bc),
    );
    (i_var3 + 0xc) = local_bx_64 + 0x51c2;
    (i_var3 + 0xe) = &ctx.g_alloc_addr_1050_1050;
    return;
}

pub fn pass1_1028_ae66(param_1: *mut Struct500, param_2: u32, param_3: u32, param_4: u32) {
    let local_bx_19: *mut Struct500;
    let mut u_var1: u16;

    pass1_1028_d1dc(param_1, 0x1387);
    u_var1 = (param_1 >> 0x10);
    local_bx_19 = param_1;
    local_bx_19.field_0x108 = param_4;
    local_bx_19.field_0x10c = param_3;
    local_bx_19.field_0x110 = param_2;
    &local_bx_19.field_0x114 = 0;
    param_1.a = 0xb0ce;
    local_bx_19.b = &PTR_LOOP_1050_1028;
    string_ops1::copy_string_1000_3d3e(
        (param_1 & 0xffff0000 | ZEXT24(&local_bx_19.field_0x8)),
        s_SCStarve_1050_5156,
    );
    return;
}

pub fn pass1_1028_acb6(param_1: *mut Struct500) -> *mut Struct500 {
    pass1_1028_d1dc(param_1, 0x3e7f);
    param_1.a = 0xae56;
    (param_1 + 2) = &PTR_LOOP_1050_1028;
    string_ops1::copy_string_1000_3d3e((param_1 & 0xffff0000 | (param_1 + 8)), s_SCSetup_1050_5124);
    return param_1;
}

pub unsafe fn pass1_1028_acec() -> u16 {
    let mut unaff_ss: u16;
    let paVar1: *mut Struct1123;
    let paVar2: *mut Struct393;
    let mut local_24: u16;
    let mut local_22: u16;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u16;

    pass1_1028_dc52(
        CONCAT22(unaff_ss, &local_14),
        (&ctx.PTR_LOOP_1050_0000 + 1),
        0,
        0x400,
    );
    loop {
        paVar1 = pass1_1028_e4ec(CONCAT22(unaff_ss, &local_14));
        if (paVar1 == 0x0) {
            break;
        }
        wvsprintf_FUN_1030_840a();
        if ((paVar1 + 0x200) != 0x8000002) {
            pass1_1038_5464(paVar1);
            pass1_1038_56d6(paVar1, 1);
        }
    }
    local_14 = ctx.s_1_1050_389a;
    local_12 = &ctx.PTR_LOOP_1050_1008;
    pass1_1028_dc52(
        CONCAT22(unaff_ss, &local_14),
        (&ctx.PTR_LOOP_1050_0000 + 1),
        0,
        0x800,
    );
    loop {
        paVar2 = pass1_1028_e4ec(CONCAT22(unaff_ss, &local_14));
        if (paVar2 == 0x0) {
            break;
        }
        pass1_1030_2690(paVar2);
    }
    return 1;
}

pub fn pass1_1028_ab32(param_1: *mut Struct500) -> *mut Struct500 {
    pass1_1028_d1dc(
        param_1,
        (s__P__P__P__P__P__P__P__P__P__P__P_1050_2e35 + 0xaa),
    );
    param_1.a = 0xaca6;
    (param_1 + 2) = &PTR_LOOP_1050_1028;
    string_ops1::copy_string_1000_3d3e(
        (param_1 & 0xffff0000 | (param_1 + 8)),
        s_SCRchSched_1050_5118,
    );
    return param_1;
}

pub fn pass1_1028_a9be(param_1: *mut Struct500) -> *mut Struct500 {
    pass1_1028_d1dc(param_1, 0x176f);
    param_1.a = 0xab22;
    (param_1 + 2) = &PTR_LOOP_1050_1028;
    string_ops1::copy_string_1000_3d3e((param_1 & 0xffff0000 | (param_1 + 8)), s_SCPower_1050_5110);
    return param_1;
}

pub fn pass1_1028_a866(param_1: *mut Struct500) -> *mut Struct500 {
    pass1_1028_d1dc(param_1, 0x36af);
    param_1.a = 0xa9ae;
    (param_1 + 2) = &PTR_LOOP_1050_1028;
    string_ops1::copy_string_1000_3d3e(
        (param_1 & 0xffff0000 | (param_1 + 8)),
        s_SCProdSched_1050_5104,
    );
    return param_1;
}

pub fn pass1_1028_a706(param_1: *mut Struct500) -> *mut Struct500 {
    pass1_1028_d1dc(param_1, 0xbb7);
    param_1.a = 0xa856;
    (param_1 + 2) = &PTR_LOOP_1050_1028;
    string_ops1::copy_string_1000_3d3e(
        (param_1 & 0xffff0000 | (param_1 + 8)),
        s_SCPrelimAlloc_1050_50f6,
    );
    return param_1;
}

pub fn pass1_1028_9ec6(param_1: *mut Struct500) -> *mut Struct500 {
    pass1_1028_d1dc(param_1, (s_noth_bmp_1050_2321 + 6));
    param_1.a = 0xa6f6;
    (param_1 + 2) = &PTR_LOOP_1050_1028;
    string_ops1::copy_string_1000_3d3e((param_1 & 0xffff0000 | (param_1 + 8)), s_SCPop_1050_50f0);
    return param_1;
}

pub unsafe fn pass1_1028_933c(
    param_1: *mut Struct500,
    param_2: u16,
    param_3: u16,
    param_4: u16,
    param_5: *mut u32,
    param_6: u16,
    param_7: u32,
    param_8: u32,
) {
    let local_bx_24: *mut Struct500;
    let pc_var1: String;

    pass1_1028_d1dc(param_1, 0x3e8);
    pc_var1 = (param_1 >> 0x10);
    local_bx_24 = param_1;
    local_bx_24.field_0x108 = param_8;
    local_bx_24.field_0x10c = param_7;
    local_bx_24.field_0x110 = 0;
    unsafe {
        local_bx_24.field_0x114 = *param_5;
    }
    local_bx_24.field_0x118 = (param_5 + 1);
    local_bx_24.field_0x11a = param_4;
    local_bx_24.field_0x11c = param_2;
    local_bx_24.field_0x120 = 0;
    local_bx_24.field_0x11e = 0;
    local_bx_24.field_0x122 = param_3;
    param_1.a = 0x9934;
    local_bx_24.b = &PTR_LOOP_1050_1028;
    string_ops1::string_fn_1000_3f9c(
        &local_bx_24.field_0x8,
        pc_var1,
        s_SCPutBldg_site_0x_08lx__bldg__u__1050_50ce,
        &ctx.g_alloc_addr_1050_1050,
        param_8,
    );
    return;
}

pub unsafe fn pass1_1028_87f0(
    param_1: *mut Struct500,
    param_2: u16,
    param_3: u16,
    param_4: u16,
    param_5: *mut u32,
    param_6: u16,
    param_7: u32,
    param_8: u32,
) {
    let local_bx_24: *mut Struct500;
    let pc_var1: String;

    pass1_1028_d1dc(param_1, 0x3e8);
    pc_var1 = (param_1 >> 0x10);
    local_bx_24 = param_1;
    local_bx_24.field_0x108 = param_8;
    local_bx_24.field_0x10c = param_7;
    local_bx_24.field_0x110 = 0;
    unsafe {
        local_bx_24.field_0x114 = *param_5;
    }
    local_bx_24.field_0x118 = (param_5 + 1);
    local_bx_24.field_0x11a = param_4;
    local_bx_24.field_0x11c = param_3;
    local_bx_24.field_0x11e = param_2;
    local_bx_24.field_0x122 = 0;
    local_bx_24.field_0x120 = 0;
    param_1.a = 0x8d8e;
    local_bx_24.b = &PTR_LOOP_1050_1028;
    string_ops1::string_fn_1000_3f9c(
        &local_bx_24.field_0x8,
        pc_var1,
        s_SinternalPutBldg_site_0x_08lx__b_1050_5046,
        &ctx.g_alloc_addr_1050_1050,
        param_8,
    );
    return;
}

pub unsafe fn pass1_1028_8888(
    param_1: *mut Struct500,
    param_2: u16,
    param_3: u16,
    param_4: *mut u32,
    param_5: u16,
    param_6: u32,
    param_7: u32,
    param_8: u32,
) {
    let local_bx_24: *mut Struct500;
    let pc_var1: String;

    pass1_1028_d1dc(param_1, 0x3e8);
    pc_var1 = (param_1 >> 0x10);
    local_bx_24 = param_1;
    local_bx_24.field_0x108 = param_8;
    local_bx_24.field_0x10c = param_7;
    local_bx_24.field_0x110 = param_6;
    unsafe {
        local_bx_24.field_0x114 = *param_4;
    }
    local_bx_24.field_0x118 = (param_4 + 1);
    local_bx_24.field_0x11a = param_3;
    local_bx_24.field_0x11c = 0;
    local_bx_24.field_0x11e = param_2;
    local_bx_24.field_0x122 = 0;
    local_bx_24.field_0x120 = 0;
    param_1.a = 0x8d8e;
    local_bx_24.b = &PTR_LOOP_1050_1028;
    string_ops1::string_fn_1000_3f9c(
        &local_bx_24.field_0x8,
        pc_var1,
        s_SinternalPutBldg2_site_0x_08lx__1050_506f,
        &ctx.g_alloc_addr_1050_1050,
        param_8,
    );
    return;
}

pub fn pass1_1028_837e(param_1: *mut Struct500) -> *mut Struct500 {
    pass1_1028_d1dc(param_1, 0xf9f);
    param_1.a = 0x84ba;
    (param_1 + 2) = &PTR_LOOP_1050_1028;
    string_ops1::copy_string_1000_3d3e(
        (param_1 & 0xffff0000 | (param_1 + 8)),
        s_SCFillResources_1050_500c,
    );
    return param_1;
}

pub fn pass1_1028_81aa(param_1: *mut Struct500) -> *mut Struct500 {
    pass1_1028_d1dc(param_1, (s_42_flc_1050_1b54 + 3));
    param_1.a = 0x836e;
    (param_1 + 2) = &PTR_LOOP_1050_1028;
    string_ops1::copy_string_1000_3d3e(
        (param_1 & 0xffff0000 | (param_1 + 8)),
        s_SCFactory_1050_5002,
    );
    return param_1;
}

pub unsafe fn pass1_1028_767e() {
    let paVar1: *mut Struct493;
    let mut in_dx: u16;
    let ppVar2: *mut Struct2111;
    let mut in_stack_0000ffec: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    paVar1 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, 1, 0x800);
    if ((&paVar1[0xb].field_0x8 != 0) && ((*ctx._PTR_LOOP_1050_65e2 % 100) == 0)) {
        ppVar2 = process_struct_1010_20ba(
            ctx._g_Struct372_1050_0ed0,
            CONCAT22(in_stack_0000ffec, 0x40),
        );
        string_ops1::wsprintf_FUN_1008_b78a(ppVar2, (ppVar2 >> 0x10));
    }
    return;
}

pub fn pass1_1028_74ae(param_1: *mut Struct500) -> *mut Struct500 {
    pass1_1028_d1dc(param_1, 0x1387);
    param_1.a = 0x819a;
    (param_1 + 2) = &PTR_LOOP_1050_1028;
    string_ops1::copy_string_1000_3d3e((param_1 & 0xffff0000 | (param_1 + 8)), s_SCEvent_1050_4ff4);
    return param_1;
}

pub fn pass1_1028_6fc0(param_1: *mut Struct500) -> *mut Struct500 {
    pass1_1028_d1dc(param_1, 0x3e7);
    param_1.a = 0x749e;
    (param_1 + 2) = &PTR_LOOP_1050_1028;
    string_ops1::copy_string_1000_3d3e((param_1 & 0xffff0000 | (param_1 + 8)), s_SCEndSim_1050_4fea);
    return param_1;
}

pub fn pass1_1028_6e60(param_1: *mut Struct500) -> *mut Struct500 {
    pass1_1028_d1dc(param_1, 0x32c7);
    param_1.a = 0x6fb0;
    (param_1 + 2) = &PTR_LOOP_1050_1028;
    string_ops1::copy_string_1000_3d3e(
        (param_1 & 0xffff0000 | (param_1 + 8)),
        s_SCConstruct_1050_4fdc,
    );
    return param_1;
}
pub fn pass1_1028_68de(param_1: *mut Struct500, param_2: u16, param_3: u32) {
    let mut iVar1: i32;
    let mut u_var2: u16;

    pass1_1028_d1dc(param_1, 0x3e8);
    u_var2 = (param_1 >> 0x10);
    iVar1 = param_1;
    (iVar1 + 0x108) = param_3;
    (iVar1 + 0x10c) = param_2;
    param_1.a = 0x6ae2;
    (iVar1 + 2) = &PTR_LOOP_1050_1028;
    string_ops1::copy_string_1000_3d3e((param_1 & 0xffff0000 | (iVar1 + 8)), s_SCAddSpew_1050_4fd2);
    return;
}

pub fn big_switch_statement_1020_bd80(switch_var: u16) -> u16 {
    let mut u_var1: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    match (switch_var) {
        1 | 6 => local_8 = 0x427,
        2 => local_8 = 0x428,
        3 | 7 => local_8 = 0x429,
        4 | 8 => local_8 = 0x425,
        5 | 9 => local_8 = 0x426,
        10 => local_8 = 0x402,
        0xb | 0x37 => local_8 = 0x418,
        0xc | 0x35 | 0x36 => local_8 = 0x42a,
        0xd => local_8 = 0x5f7,
        0xe => local_8 = 0x5f6,
        0xf => local_8 = 0x403,
        0x10 => local_8 = 0x5f8,
        0x11 => local_8 = 0x404,
        0x12 => local_8 = 0x405,
        0x13 | 0x14 | 0x15 => local_8 = 0x406,
        0x16 | 0x19 => local_8 = 0x5f9,
        0x17 | 0x1a => local_8 = 0x5fa,
        0x18 => local_8 = 0x5fb,
        0x1b | 0x1c | 0x1d => local_8 = 0x408,
        0x1e | 0x1f | 0x20 => local_8 = 0x409,
        0x21 => local_8 = 0x42c,
        0x22 | 0x23 | 0x24 => local_8 = 0x40a,
        0x25 | 0x26 | 0x27 => local_8 = 0x40b,
        0x28 | 0x29 => local_8 = 0x40c,
        0x2a | 0x2b => local_8 = 0x40d,
        0x2c => local_8 = 0x40e,
        0x2d | 0x2e => local_8 = 0x40f,
        0x2f | 0x30 => local_8 = 0x410,
        0x31 | 0x32 => local_8 = 0x411,
        0x33 | 0x34 => local_8 = 0x416,
        0x38 | 0x39 => local_8 = 0x5fc,
        0x3a | 0x3b => local_8 = 0x419,
        0x3c | 0x3d => local_8 = 0x5fd,
        0x3e => local_8 = 0x5fe,
        0x3f => local_8 = 0x5ff,
        0x40 => local_8 = 0x600,
        0x41 => local_8 = 0x601,
        0x42 | 0x46 | 0x6b => local_8 = 0x602,
        0x43 => {
            local_6 = s_bidLRoadConst_1050_4e7a;
            return local_6;
        }
        0x44 => {
            local_6 = s_bidRRoadConst_1050_4e88;
            return local_6;
        }
        0x45 => {
            local_6 = s_bidXRoadConst_1050_4e96;
            return local_6;
        }
        0x47 => local_8 = 0x42b,
        0x48 | 0x49 | 0x4a | 0x70 | 0x71 | 0x72 => local_8 = 0x603,
        0x4b => local_8 = 0x42d,
        0x4c => local_8 = 0x604,
        0x4d => local_8 = 0x605,
        0x4e => local_8 = 0x606,
        0x4f | 0x50 | 0x51 => local_8 = 0x41a,
        0x52 | 0x53 => local_8 = 0x41b,
        0x54 | 0x55 | 0x56 => local_8 = 0x41d,
        0x57 | 0x58 | 0x59 => local_8 = 0x41e,
        0x5a => local_8 = 0x41f,
        0x5b | 0x5c => local_8 = 0x421,
        0x5d | 0x5e | 0x5f => local_8 = 0x420,
        0x60 | 0x61 => local_8 = 0x607,
        0x62 | 99 => local_8 = 0x608,
        100 => local_8 = 0x609,
        0x65 => local_8 = 0x422,
        0x66 | 0x67 => local_8 = 0x423,
        0x68 | 0x69 => local_8 = 0x60a,
        0x6a => local_8 = 0x60b,
        0x6c | 0x6d => local_8 = 0x41c,
        0x6e => local_8 = 0x60c,
        0x6f => local_8 = 0x60d,
        0x73 | 0x77 => local_8 = 0x415,
        0x74 | 0x78 | 0x79 => local_8 = 0x412,
        0x75 => local_8 = 0x413,
        0x76 => local_8 = 0x414,
        0x7a => local_8 = 0x60e,
        0x7b => local_8 = 0x60f,
        0x7c => local_8 = 0x610,
        0x7d => local_8 = 0x611,
        0x7e => local_8 = 0x612,
        0x7f => local_8 = 0x613,
        0x80 => local_8 = 0x614,
        0x81 => local_8 = 0x615,
        0x82 => local_8 = 0x616,
        0x83 => local_8 = 0x617,
        0x84 => local_8 = 0x618,
        0x85 => local_8 = 0x619,
        0x86 => local_8 = 0x61a,
        0x87 => local_8 = 0x61b,
        0x88 => local_8 = 0x61c,
        0x89 => local_8 = 0x61d,
        _ => {
            local_8 = 0x424;
        }
    }
    u_var1 = load_string_1010_847e(ctx._g_struct_73_1050_14cc, local_8);
    return u_var1;
}

pub fn pass1_1020_c0ca(param_1: u16) {
    big_switch_statement_1020_c0d8(param_1);
    return;
}

pub fn big_switch_statement_1020_c0d8(switch_var: u16) -> u16 {
    let mut u_var1: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    match (switch_var) {
        1 => u_var1 = 0x5b7,
        2 => u_var1 = 0x5b8,
        3 => u_var1 = 0x5b9,
        4 => u_var1 = 0x5ba,
        5 => u_var1 = 0x5bb,
        6 => u_var1 = 0x5bc,
        7 => u_var1 = 0x5bd,
        8 => u_var1 = 0x5be,
        9 => u_var1 = 0x5bf,
        10 => u_var1 = 0x5c0,
        0xb => u_var1 = 0x5c1,
        0xc => u_var1 = 0x5c2,
        0xd => u_var1 = 0x5c3,
        0xe => u_var1 = 0x5c4,
        0xf => u_var1 = 0x5c5,
        0x10 => u_var1 = 0x5c6,
        0x11 => u_var1 = 0x5c7,
        0x12 => u_var1 = 0x5c8,
        0x13 => u_var1 = 0x5c9,
        0x14 => u_var1 = 0x5ca,
        0x15 => u_var1 = 0x5cb,
        0x16 => u_var1 = 0x5cc,
        0x17 => u_var1 = 0x5cd,
        0x18 => u_var1 = 0x5ce,
        0x19 => u_var1 = 0x5cf,
        0x1a => u_var1 = 0x5d0,
        0x1b => u_var1 = 0x5d1,
        0x1c => u_var1 = 0x5d2,
        0x1d => u_var1 = 0x5d3,
        0x1e => u_var1 = 0x5d4,
        0x1f => u_var1 = 0x5d5,
        _ => u_var1 = 0x5d9,
        0x21 => u_var1 = 0x5d6,
        0x23 => u_var1 = 0x5d7,
        0x24 => {
            u_var1 = 0x5e5;
        }
    }
    u_var1 = load_string_1010_847e(ctx._g_struct_73_1050_14cc, u_var1);
    return u_var1;
}

pub fn big_switch_statement_1020_c222(param_1: u16) -> u16 {
    let mut u_var1: u16;
    let mut u_var2: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    match (param_1) {
        1 => u_var2 = 0x57f,
        2 => u_var2 = 0x580,
        3 => u_var2 = 0x581,
        4 => u_var2 = 0x5e7,
        5 => u_var2 = 0x57e,
        6 => u_var2 = 0x582,
        7 => u_var2 = 1000,
        8 => u_var2 = 0x3e9,
        9 => u_var2 = 0x3ea,
        10 => u_var2 = 0x3eb,
        0xb => u_var2 = 0x3ec,
        0xc => u_var2 = 0x3ed,
        0xd => u_var2 = 0x3ee,
        0xe => u_var2 = 0x3ef,
        0xf => u_var2 = 0x3f0,
        0x10 => u_var2 = 0x3f1,
        0x11 => u_var2 = 0x3f2,
        0x12 => u_var2 = 0x3f4,
        0x13 => u_var2 = 0x3f5,
        0x14 => u_var2 = 0x532,
        _ => {
            u_var2 = 0x5d9;
        }
    }
    u_var1 = load_string_1010_847e(ctx._g_struct_73_1050_14cc, u_var2);
    return u_var1;
}
