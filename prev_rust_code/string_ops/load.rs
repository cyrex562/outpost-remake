use crate::app_context::AppContext;
use crate::bad_funcs::bad1::out;
use crate::err_ops::error_check_1000_17ce;
use crate::mem_funcs::mem_ops_1::{StructuredData, get_fn_ptr_at_address};
use crate::other_funcs::{switch_stmt_1008_ab80, zero_list_1008_3e38};
use crate::pass::pass11_funcs::pass1_1008_cfa0;
use crate::pass::pass12_funcs::pass1_1008_b9ce;
use crate::pass::pass13_funcs::pass1_1008_944e;
use crate::pass::pass14_funcs::{pass1_1008_3e76, pass1_1008_5784, pass1_1008_5b12, pass1_fn_1008_60e8, pass1_fn_1008_612e};
use crate::pass::pass15_funcs::{pass1_1020_ba3e, pass1_1020_ba7e, pass1_1020_bae6, pass1_1020_bb8a};
use crate::pass::pass17_funcs::{pass1_1030_1cd8, pass1_1030_2690, pass1_1030_627e, pass1_1030_73a8, pass1_1030_7bee, pass1_1030_7c28, pass1_1030_7d1c, pass1_1030_7ddc, pass1_1030_809c, pass1_1030_8326, pass1_1030_8344};
use crate::pass::pass20_funcs::{pass1_1010_c3c2, pass1_1010_dd5e};
use crate::pass::pass2_funcs::pass1_1010_e964;
use crate::pass::pass4_funcs::{pass1_1028_bb24, pass1_1028_d1dc, pass1_1028_dc52, pass1_1028_e1ec, pass1_1028_e2e0, pass1_1028_e4ec};
use crate::pass::pass5_funcs::pass1_1030_b9b2;
use crate::pass::pass6_funcs::{pass1_1038_4d28, pass1_1038_5464, pass1_1038_56d6};
use crate::pass::pass7_funcs::{pass1_1018_3e8c, pass1_1018_427c, pass1_1018_435e};
use crate::pass::pass8_funcs::{pass1_1008_e8cc, pass1_1010_089e, pass1_1010_60a0, pass1_1010_878c};
use crate::pass::pass_funcs::{pass1_1000_3d7a, pass1_1000_4906, pass1_fn_1000_2b3c, pass1_fn_1000_2b5c, pass1_fn_1000_2f48};
use crate::string_ops::misc;
use crate::struct_ops::struct_ops_2::{process_struct_1000_179c, process_struct_1008_cbc4, process_struct_1008_cda2, process_struct_1008_d3ae, process_struct_1040_b0bc, process_struct_1010_20ba};
use crate::structs::prog_structs_10::Struct73;
use crate::structs::prog_structs_16::Struct493;
use crate::structs::prog_structs_18::Struct566;
use crate::structs::prog_structs_19::Struct500;
use crate::structs::prog_structs_1::Struct393;
use crate::structs::prog_structs_20::Struct965;
use crate::structs::prog_structs_24::{Struct2111, Struct432, Struct894};
use crate::structs::prog_structs_26::{Struct1123, Struct883, Struct891};
use crate::structs::prog_structs_27::{pass1_struct_2, Struct298};
use crate::structs::prog_structs_28::{Struct1016, Struct1053, Struct346, Struct912, Struct913};
use crate::structs::prog_structs_29::Struct1035;
use crate::structs::prog_structs_2::{Struct199, Struct7};
use crate::sys_ops::dos_ops::dos3_call_1000_51aa;
use crate::sys_ops::private_profile_str::write_private_profile_str_1010_5b10;
use crate::util::{CARRY1, CONCAT12, CONCAT13, CONCAT22, CONCAT31, SUB42, ZEXT24};
use crate::winapi::{LoadString16, MessageBox16};

pub unsafe fn load_string_switch_1008_a1f0(ctx: &mut AppContext,
                                           string_param_1: &mut String,
                                           param_2: &mut u16,
                                           param_3: &mut u32,
                                           param_4: &mut u32,
                                           param_5: &mut u32,
) {
    let mut u_var1: u32;
    let mut var2: u16 = 0;
    let mut var3: i32;
    let mut var4: u16;
    let mut struct_var5: &mut Struct493;
    let mut str_var6: String;
    let mut str_var7: String;
    let mut var9: u16 = 0;
    let mut var11: u16 = 0;
    let mut var12: String = String::new();
    let mut var14: u16;
    let mut var15: String = String::new();
    let mut var16: Struct2111;
    let mut var17: u16;
    let mut var18: Struct73 = Struct73::new();
    let mut var19: u8;
    let mut var20: u16 = 0;
    let mut var25: String = String::new();
    let mut var29: u32;
    let mut fn_ptr_1: fn();

    var3 = 0;
    *param_5 = 0;
    *param_4 = 0;
    *param_3 = 0;
    *param_2 = 0;
    string_param_1[0xe] = '\0';
    var29 = (string_param_1 + 10);
    fn_ptr_1 = ((string_param_1 + 10) + 0x10);
    (**fn_ptr_1)();
    _local_6 = CONCAT22(ctx.dx_reg, var3 as u16);
    if (ctx.dx_reg | var3) == 0 {
        return;
    }
    *param_5 = (var3 + 4) as u32;
    var4 = (var3 + 10) as u16;
    *param_3 = var4 as u32;
    switch_stmt_1008_ab80(string_param_1, &mut var12, param_5);
    *param_2 = var4;
    let mut var14 = ctx.PTR_LOOP_1050_1008;
    let mut var17 = &mut ctx.g_struct_73_1050_14cc;
    match var3 + 4 {
        1 => {
            load_string_1010_84e0(ctx, 0x3ff, string_param_1 + 0xe, 0x59);
            *param_4 = 0xd1;
            // goto LAB_1008_a2b1;
        }
        2 => {
            u_var1 = (var3 + 6) as u32;
            // resource_id_1 = (u_var1  >> 0x10);
            _local_DL_217 = ctx.dx_reg;
            struct_var5 = pass1_1028_e1ec(ctx, &mut ctx._PTR_LOOP_1050_65e2, u_var1 as u16, var20);
            *var18 = ctx.stack_seg_reg;
            var19 = (ctx.stack_seg_reg >> 8) as u8;
            load_string_1010_84e0(ctx, 0x100, &mut var25, 0x3ff);
            *var18 = _local_DL_217;
            var19 = (_local_DL_217 >> 8);
            var15 = pass1_1038_4d28(&mut var18);
            var14 = 0x1000;
            let x: String = String::new();
            misc::string_fn_1000_3f9c(
                string_param_1 + 0xe,
                &var12,
                &var25,
                &x,
                &var15,
            )
        }
        5 => {
            var20 = 0x59b;
            // goto LAB_1008_a277;
        }
        6 => {
            load_string_1010_84e0(ctx,
                                  0x3ff,
                                  (string_param_1 & 0xffff0000 | ZEXT24(string_param_1 + 0xe)),
                                  0x631,
            );
            *param_4 = 0xd4;
            // LAB_1008_a2b1:
            var14 = 0x1010;
            *param_3 = 1
        }
        7 => {
            var20 = 0x400;
            // LAB_1008_a277:
            var14 = 0x1010;
            load_string_1010_84e0(
                ctx,
                0x3ff,
                (string_param_1 & 0xffff0000 | ZEXT24(string_param_1 + 0xe)),
                var20,
            )
        }
        9 => {
            if (string_param_1 + 0x416) == 0 {
                (string_param_1 + 0x416) = 1;
            }
            // goto LAB_1008_a35a;
        }
        0xb => {
            if (string_param_1 + 0x41a) == 0 {
                (string_param_1 + 0x41a) = 1;
            }
            // goto LAB_1008_a35a;
        }
        0xe => {
            if (string_param_1 + 0x41c) == 0 {
                (string_param_1 + 0x41c) = 1;
            }
            // goto LAB_1008_a35a;
        }
        0x14 => {
            if (string_param_1 + 0x418) == 0 {
                (string_param_1 + 0x418) = 1;
                str_var7 = string_param_1 + 0xe;
                str_var6 = str_var7;
                // (string_param_1 & 0xff000000 | CONCAT12(var18, str_var7)),
                load_string_1010_84e0(ctx, 0x3ff, &mut str_var7, 0x72a);
                // local_11e = (ctx.g_struct_73_1050_14cc  >> 0x10);
                misc::load_string_1010_847e(ctx, &mut var18, 0x72b);
                misc::process_string_1000_3cea(&mut str_var7, &mut str_var6);
                *param_4 = 0x4c;
                var16 = process_struct_1010_20ba(&mut ctx.g_struct_var_1050_0ed0, 0x1002b);
                var14 = 0x1010;
                pass1_1010_089e(&mut var16, 1, 10);
            }
            // goto LAB_1008_a35a;
        }
        0x16 => {
            var14 = 0x1010;
            load_string_1010_84e0(
                ctx,
                0x3ff,
                (string_param_1 & 0xffff0000 | ZEXT24(string_param_1 + 0xe)),
                0x713,
            );
            *param_4 = 0x28
        }
        0x17 => {
            var14 = 0x1010;
            load_string_1010_84e0(ctx,
                                  0x3ff,
                                  (string_param_1 & 0xffff0000 | ZEXT24(string_param_1 + 0xe)),
                                  0x717,
            );
            *param_4 = 0x2c
        }
        0x18 => {
            var14 = 0x1010;
            load_string_1010_84e0(ctx,
                                  0x3ff,
                                  (string_param_1 & 0xffff0000 | ZEXT24(string_param_1 + 0xe)),
                                  0x719,
            );
            *param_4 = 0x2e
        }
        0x1b => {
            var14 = 0x1010;
            load_string_1010_84e0(
                ctx,
                0x3ff,
                (string_param_1 & 0xffff0000 | ZEXT24(string_param_1 + 0xe)),
                0x71b,
            );
            *param_4 = 0x30
        }
        0x1c => {
            var14 = 0x1010;
            load_string_1010_84e0(
                ctx,
                0x3ff,
                (string_param_1 & 0xffff0000 | ZEXT24(string_param_1 + 0xe)),
                0x71d,
            );
            *param_4 = 0x32
        }
        0x1f => {
            var14 = 0x1010;
            load_string_1010_84e0(
                ctx,
                0x3ff,
                (string_param_1 & 0xffff0000 | ZEXT24(string_param_1 + 0xe)),
                0x720,
            );
            *param_4 = 0x34
        }
        0x21 => {
            if (string_param_1 + 0x41e) == 0 {
                (string_param_1 + 0x41e) = 1;
            }
            // LAB_1008_a35a:
            *param_2 = 0
        }
        0x24 => {
            var14 = 0x1010;
            load_string_1010_84e0(
                ctx,
                0x3ff,
                (string_param_1 & 0xffff0000 | ZEXT24(string_param_1 + 0xe)),
                0x715,
            );
            *param_4 = 0x2a
        }
        0x31 => {
            var14 = 0x1010;
            load_string_1010_84e0(
                ctx,
                0x3ff,
                (string_param_1 & 0xffff0000 | ZEXT24(string_param_1 + 0xe)),
                0x712,
            );
            *param_4 = 0x27
        }
        0x32 => {
            var14 = 0x1010;
            load_string_1010_84e0(
                ctx,
                0x3ff,
                (string_param_1 & 0xffff0000 | ZEXT24(string_param_1 + 0xe)),
                0x714,
            );
            *param_4 = 0x29
        }
        0x33 => {
            var14 = 0x1010;
            load_string_1010_84e0(
                ctx,
                0x3ff,
                (string_param_1 & 0xffff0000 | ZEXT24(string_param_1 + 0xe)),
                0x716,
            );
            *param_4 = 0x2b
        }
        0x34 => {
            var14 = 0x1010;
            load_string_1010_84e0(
                ctx,
                0x3ff,
                (string_param_1 & 0xffff0000 | ZEXT24(string_param_1 + 0xe)),
                0x718,
            );
            *param_4 = 0x2d
        }
        0x35 => {
            var14 = 0x1010;
            load_string_1010_84e0(
                ctx,
                0x3ff,
                (string_param_1 & 0xffff0000 | ZEXT24(string_param_1 + 0xe)),
                0x71a,
            );
            *param_4 = 0x2f
        }
        0x36 => {
            var14 = 0x1010;
            load_string_1010_84e0(
                ctx,
                0x3ff,
                (string_param_1 & 0xffff0000 | ZEXT24(string_param_1 + 0xe)),
                0x71c,
            );
            *param_4 = 0x31
        }
        0x37 => {
            str_var6 = string_param_1 + 0xe;
            str_var7 = str_var6;
            // (string_param_1 & 0xff000000 | CONCAT12(var18, str_var6))
            load_string_1010_84e0(
                ctx,
                0x3ff,
                &mut str_var6,
                0x71e,
            );
            // local_11e = (ctx.g_struct_73_1050_14cc  >> 0x10);
            misc::load_string_1010_847e(ctx, &mut ctx.g_struct_73_1050_14cc, 0x71f);
            var14 = 0x1000;
            misc::process_string_1000_3cea(
                string_param_1,
                &mut str_var7,
            );
            *param_4 = 0x33
        }
        0x38 => {
            var14 = 0x1010;
            load_string_1010_84e0(ctx, 0x3ff, string_param_1 + 0xe, 0x721);
            *param_4 = 0x35
        }
        0x39 => {
            var14 = 0x1010;
            load_string_1010_84e0(ctx, 0x3ff, string_param_1 + 0xe, 0x722);
            *param_4 = 0x36
        }
        0x3a => {
            var14 = 0x1010;
            load_string_1010_84e0(ctx, 0x3ff, string_param_1 + 0xe, 0x723);
            *param_4 = 0x37
        }
        0x3b => {
            var14 = 0x1010;
            load_string_1010_84e0(ctx, 0x3ff, string_param_1 + 0xe, 0x724);
            *param_4 = 0x38
        }
        0x3c => {
            var14 = 0x1010;
            load_string_1010_84e0(ctx, 0x3ff, string_param_1 + 0xe, 0x725);
            *param_4 = 0x39
        }
        0x3d => {
            var14 = 0x1010;
            load_string_1010_84e0(ctx, 0x3ff, string_param_1 + 0xe, 0x742);
            *param_4 = 0xce
        }
        0x3e => {
            var14 = 0x1010;
            load_string_1010_84e0(ctx, 0x3ff, string_param_1 + 0xe, 0x743);
            *param_4 = 0xcf
        }
        0x3f => {
            var14 = 0x1010;
            load_string_1010_84e0(ctx, 0x3ff, string_param_1 + 0xe, 0x744);
            *param_4 = 0xd0
        }
        0x40 => {
            var14 = 0x1010;
            load_string_1010_84e0(ctx, 0x3ff, string_param_1 + 0xe, 0x745);
            *param_4 = 0xd1
        }
        0x41 => {
            var14 = 0x1010;
            load_string_1010_84e0(ctx, 0x3ff, string_param_1 + 0xe, 0x746);
            *param_4 = 0xd2
        }
        0x42 => {
            var14 = 0x1010;
            load_string_1010_84e0(ctx, 0x3ff, string_param_1 + 0xe, 0x747);
            *param_4 = 0xd3
        }
        0x43 => {
            var14 = 0x1010;
            load_string_1010_84e0(ctx,
                                  0x3ff,
                                  (string_param_1 & 0xffff0000 | ZEXT24(string_param_1 + 0xe)),
                                  0x748,
            );
            *param_4 = 0xd5
        }
        0x44 => {
            var14 = 0x1010;
            load_string_1010_84e0(ctx,
                                  0x3ff,
                                  (string_param_1 & 0xffff0000 | ZEXT24(string_param_1 + 0xe)),
                                  0x749,
            );
            *param_4 = 0xd6
        }
        0x45 => {
            var14 = 0x1010;
            load_string_1010_84e0(ctx,
                                  0x3ff,
                                  (string_param_1 & 0xffff0000 | ZEXT24(string_param_1 + 0xe)),
                                  0x74a,
            );
            *param_4 = 0xd7;
        }
        _ => {}
    }
    if _local_6 != 0x0 {
        // var18 = ctx.dx_reg;
        fn_ptr_1 = get_fn_ptr_at_address(_local_6);
        fn_ptr_1(var14, var3, var18, 1, var29);
    }
    return;
}

/*
1 use
 */
pub unsafe fn load_string_1008_a8f4(
    param_1: &mut String,
    param_2: &mut u16,
    param_3: &mut u16,
    param_4: u32,
) {
    let mut unaff_ss: u16;
    // let mut u_var1: u32;
    // let mut local_a: u16;
    // let mut local_8: u16;
    let mut local_6: u32 = 0;

    // let mut u_var1 = load_string_switch_1008_a1f0(ctx,
    //                                               param_1,
    //                                               param_2,
    //                                               &mut local_6,
    //                                               &local_6 + 2,
    //                                               param_4
    // );
    pass1_1008_944e(param_3, local_6 as u16);
    return u_var1;
}

/*
1 ref
 */
pub fn load_string_1008_b1f0(ctx: &mut AppContext) {
    misc::load_string_1010_847e(ctx, &mut ctx.g_struct_73_1050_14cc, 0x531);
    return;
}

/*
1 ref
 */
pub unsafe fn load_string_1008_b65a(param_1: u32, param_2: &mut string, param_3: i32) {
    let mut in_ax: i32 = 0;
    let mut in_dx: i32 = 0;
    let mut in_resource_id: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    pass1_1008_b9ce(param_1, param_3);
    if ((in_dx | in_ax) == 0) || ((in_ax + 8) != 1) {
        in_resource_id = 0x434;
    } else {
        in_resource_id = 0x435;
    }
    load_string_1010_84e0(ctx,
                          0x3ff,
                          param_2,
                          in_resource_id,
    );
    return;
}

/*
1 ref
 */
pub unsafe fn load_string_1038_8dda(param_1: u32) {
    let mut in_dx: Struct7 = Struct7::new();
    let mut u_var1: u16;
    let mut unaff_ss: u16 = 0;
    let mut local_212: u32;
    let mut local_20e: u32;
    let mut local_20a: u16;
    let mut local_208: u16;
    let mut local_206: String = String::new();
    let mut local_104: String = String::new();

    process_struct_1000_179c(ctx, 0x1000, &mut in_dx);
    load_string_1010_84e0(ctx, 0x100, &mut local_206, 0x57b);
    load_string_1010_84e0(ctx, 0x3ff, &mut in_dx.field_0, 0x803);
    load_string_1010_84e0(
        ctx,
        0x3ff,
        &mut local_104,
        0x804,
    );
    misc::process_string_1000_3cea(&mut in_dx.field_0, &mut local_104);
    load_string_1010_84e0(
        ctx,
        0x3ff,
        &mut local_104,
        0x805,
    );
    misc::process_string_1000_3cea(&mut in_dx.field_0, &mut local_104);
    load_string_1010_84e0(
        ctx,
        0x3ff,
        &mut local_104,
        0x806,
    );
    misc::process_string_1000_3cea(&mut in_dx.field_0, &mut local_104);
    load_string_1010_84e0(
        ctx,
        0x3ff,
        &mut local_104,
        0x807,
    );
    misc::process_string_1000_3cea(&mut in_dx.field_0, &mut local_104);
    load_string_1010_84e0(
        ctx,
        0x3ff,
        &mut local_104,
        0x808,
    );
    misc::process_string_1000_3cea(&mut in_dx.field_0, &mut local_104);
    // u_var1 = (param_1  >> 0x10);
    // MessageBox16(
    //     0,
    //     &mut local_206,
    //     &mut in_dx.field_0,
    //     (param_1 + 6),
    // );
    MessageBox16((param_1 + 6) as u16, &mut in_dx.field_0, &mut local_206, 0);
    load_string_1010_84e0(
        ctx,
        0x3ff,
        &mut in_dx.field_0,
        0x809,
    );
    load_string_1010_84e0(
        ctx,
        0x3ff,
        &mut local_104,
        0x80a,
    );
    misc::process_string_1000_3cea(&mut in_dx.field_0, &mut local_104);
    // MessageBox16(
    //     0,
    //     CONCAT22(unaff_ss, local_206),
    //     CONCAT22(in_dx, in_ax),
    //     (param_1 + 6),
    // );
    MessageBox16((param_1 + 6) as u16, &mut in_dx.field_0, &mut local_206, 0);
    error_check_1000_17ce(ctx, &mut in_dx);
    return;
}

// pub fn modify_string_11b8_02b9(param_1: u8, param_2: u16, param_3: u16) {
//     let mut byte_2: u8;
//     let mut byte_3: u8;
//     let mut unaff_bp: i32;
//     let mut unaff_si: i32;
//     let mut unaff_DI: i32;
//     let local_DS__1: &mut  u8;
//     let local_res0: &mut  u8;
//     let mut byte_1: u8;
//     let bytes_1: &mut  u8;
//     let mut char_3: u8;
//     let string_1: &mut  libc::c_char;
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

/*

u32 __stdcall16far load_string_1010_847e(u16 offset_base,u16 segment,u16 resource_id)
{
  LoadString16(0x3ff,CONCAT22(segment,offset_base + 0x682),resource_id,g_h_instance_1050_038c);
  return CONCAT22(segment,offset_base + 0x682);
}
*/

/*
25 refs
 */
pub unsafe fn load_str_1010_84ac(
    ctx: &mut AppContext,
    in_struct_1: &mut Struct73,
    resource_id: u16,
) {
    // LoadString16(
    //     0x3ff,
    //     CONCAT22(in_struct_73_hi, in_struct_73_low + 1),
    //     resource_id,
    //     ctx.g_h_instance_1050_038c,
    // );
    LoadString16(ctx.g_h_instance_1050_038c, resource_id, &in_struct_1.field_0x0, 0x3ff);
    pass1_fn_1008_60e8(in_struct_1 + 1);
    return;
}

/*
150 refs
 */
pub fn load_string_1010_84e0(ctx: &mut AppContext,
                             buf_len: i16,
                             out_buffer: &mut string,
                             in_resource_id: u16) -> i16 {
    // LoadString16(
    //     buf_len,
    //     out_buffer,
    //     in_resource_id,
    //     ctx.g_h_instance_1050_038c,
    // );
    LoadString16(ctx.g_h_instance_1050_038c, in_resource_id, out_buffer, buf_len)
}

/*
0 refs
 */
pub unsafe fn load_string_1010_de78(param_1: &mut String, param_2: u32) {
    let mut in_ax: i32;
    let mut in_resource_id: u16;
    (param_1 + 0x13c) = 0;
    pass1_1030_809c(param_2);
    if in_ax == 0 {
        in_resource_id = 0x531;
    } else {
        in_resource_id = 0x5f4;
    }
    load_string_1010_84e0(
        ctx,
        0x3ff,
        param_1 + 0x13c,
        in_resource_id,
    );
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

/*
1 ref
 */
pub unsafe fn load_str_1038_81be(param_1: u32, param_2: &mut Struct7) {
    let mut u_var1: u16;
    let mut unaff_ss: u16;
    let mut local_212: u32;
    let mut local_20e: u32;
    let mut local_20a: u16;
    let mut local_208: u16;
    let mut local_206: String = String::new();
    let mut local_104: String = String::new();

    process_struct_1000_179c(ctx, 0x1000, param_2);
    load_string_1010_84e0(
        ctx,
        0x3ff,
        &mut local_206,
        0x57b,
    );
    load_string_1010_84e0(
        ctx,
        0x3ff,
        param_2,
        0x80b,
    );
    load_string_1010_84e0(
        ctx,
        0x3ff,
        &mut local_104,
        0x80c,
    );
    misc::process_string_1000_3cea(&mut param_2.field_0, &mut local_104);
    load_string_1010_84e0(
        ctx,
        0x3ff,
        &mut local_104,
        0x80d,
    );
    misc::process_string_1000_3cea(&mut param_2.field_0, &mut local_104);
    load_string_1010_84e0(
        ctx,
        0x3ff,
        &mut local_104,
        0x80e,
    );
    misc::process_string_1000_3cea(&mut param_2.field_0, &mut local_104);
    load_string_1010_84e0(
        ctx,
        0x3ff,
        &mut local_104,
        0x80f,
    );
    misc::process_string_1000_3cea(&mut param_2.field_0, &mut local_104);
    // u_var1 = (param_1  >> 0x10);
    // MessageBox16(
    //     0,
    //     CONCAT22(unaff_ss, local_206),
    //     CONCAT22(in_dx, in_ax),
    //     (param_1 + 6),
    // );
    MessageBox16((param_1 + 6) as u16, &mut param_2.field_0, &mut local_206, 0);
    load_string_1010_84e0(
        ctx,
        0x3ff,
        param_2,
        0x810,
    );
    load_string_1010_84e0(
        ctx,
        0x3ff,
        &mut local_104,
        0x811,
    );
    misc::process_string_1000_3cea(&mut param_2.field_0, &mut local_104);
    load_string_1010_84e0(
        ctx,
        0x3ff,
        &mut local_104,
        0x812,
    );
    misc::process_string_1000_3cea(&mut param_2.field_0, &mut local_104);
    // MessageBox16(
    //     0,
    //     CONCAT22(unaff_ss, local_206),
    //     CONCAT22(in_dx, in_ax),
    //     (param_1 + 6),
    // );
    MessageBox16((param_1 + 6) as u16, &mut param_2.field_0, &mut local_206, 0);
    error_check_1000_17ce(ctx, param_2);
    return;
}
