use crate::defines::{Struct18, Struct19, Struct4, Struct43, Struct5, Struct_1010_4e08, U32Ptr};
use crate::file::file_1008::read_file_1008_7dee;
use crate::file::write::write_to_file_1008_7e1c;
use crate::global::AppContext;
use crate::pass::pass_1018::pass1_1018_4dce;
use crate::struct_ops::struct_1018::struct_op_1018_4cda;
use crate::sys_api::sys_1000_3f9c;
use crate::util::{
    read_func_from_addr, read_string_from_addr, read_struct_from_addr, write_string_to_addr,
    CARRY1, CONCAT11, SBORROW2,
};
use crate::win_struct::{DEVMODEA, HPEN16, WNDCLASS16};
use crate::winapi::{swi, CreatePen16, GetStockObject16, WritePrivateProfileString16};
use crate::{
    mem_1000::mem_op_1000_179c,
    mixed::mixed_1010_20ba,
    pass::{
        pass_1000::pass1_1000_484c,
        pass_1008::pass1_1008_4772,
        pass_1010::{
            pass1_1010_1f62, pass1_1010_209e, pass1_1010_6ca2, pass1_1010_715c, pass1_1010_8170,
        },
        pass_1018::pass1_1018_0afa,
    },
    string::string_1040::string_1040_8520,
    switch_ops::switch_1008::switch_1008_73ea,
    ui::ui_1008::palette_op_1008_4e08,
    util::{CONCAT12, CONCAT13, CONCAT22, SUB42},
    win_struct::{HDC16, HGDIOBJ16, HPALETTE16, POINT16, RECT16},
    winapi::{
        CreateDC16, DeleteDC16, DeleteObject16, PtInRect16, Rectangle16, SelectObject16,
        SelectPalette16,
    },
};

pub fn draw_fn_1010_2a32(
    ctx: &mut AppContext,
    param_1: u16,
    param_2: u16,
    mut ret_val: U32Ptr,
    param_4: u16,
    param_5: &mut u16,
    param_6: u32,
    param_7: &mut u16,
    param_8: u16,
    param_9: u16,
    param_10: u16,
    unaff_ss: u16,
    unaff_si: u16,
    unaff_di: u16,
    in_af: u8,
    extraout_dx: u16,
    in_stack_0000ffca: &mut u16,
    in_stack_0000ffde: u16,
    stack0xffec: bool,
    extraout_DX_00: u16,
    extraout_DX_01: u16,
    extraout_DX_02: u16,
    stack0xfffa: u16,
    stack0xffde: u16,
) -> u16 {
    let mut string_2: String;
    let var_3: U32Ptr;
    let u_var4: u32;
    let var_5: u8;
    let u_var6: u16;
    let func_ptr_1: fn();
    let var_8: u32;
    let var_9: U32Ptr;
    let b_force_background: bool;
    let handle: HGDIOBJ16;
    let var_12: u16;
    let var_13: u16;
    let b_var14: bool;
    let var_15: i16;
    let var_16: u8;
    let mut var_17: &mut String;
    let var_18: U32Ptr;
    let var_20: i16;
    let hdc: HDC16;
    let bool_24: bool;
    let mut struct_25: Struct19;
    let var_26: u16;
    let pu_var27: u32;
    let handle_29: HGDIOBJ16;

    let mut var_10 = ret_val;
    let mut var_31 = param_9 as u8;
    let mut var32 = (param_9 >> 0x8) as u8;
    let mut var_23 = 0;
    let mut var_30 = 0x0;
    let mut var_28 = (param_4 >> 0x8) as u8;
    if (param_6 + 0xec76 & 0x3) != 0x0 {
        // goto LAB_1010_2ad8;
    }
    let mut u_var11 = (param_6 + 0xec76 >> 0x1) as u16;
    if 0x1c < u_var11 {
        //goto LAB_1010_2ad8;
    }
    match u_var11 {
        0x1 | 0x3 | 0xb => (u_var11 + 0xa) = param_8,
        0x9 | 0xf | 0x15 | 0x1b => {
            (u_var11 + 0xa) = param_8;
            (u_var11 + 0x10) = param_8;
            (u_var11 + 0xc) = param_8;
            return param_10;
        }
        0x5 => {
            b_var14 = write_to_file_1008_7e1c(
                *param_5,
                param_6 as u16,
                param_8 as usize,
                0x0,
                read_string_from_addr(CONCAT22(param_1, param_9)),
                0x1008,
            );
            if b_var14 != false {
                return *param_7;
            }
            ctx.PTR_LOOP_1050_0310 = 0x6d0;
            return *param_7;
        }
        0x6 => {
            var_23 = 0
            //     TODO: goto LAB_1010_2ad8;
        }
        0x7 => {
            func_ptr_1 = read_func_from_addr::<fn()>(param_8 as u32);
            func_ptr_1();
            var_15 = param_5 + 0x105;
            var_17 = read_string_from_addr(extraout_dx as u32);
            pass1_1010_8170(
                ctx.PTR_LOOP_1050_14cc as i16,
                var_15,
                extraout_dx as u32,
                0x1010,
            );
            var_20 = param_5 * 0x4;
            (ret_val + var_20 + 0x26) = var_15 as u32;
            // (ret_val + var_20 + 0x28) = var_17;
            write_string_to_addr((ret_val + var_20 + 0x28), &var_17);
            handle_29 = ctx.data_seg;
            struct_25 = pass1_1008_4772(Some(read_struct_from_addr::<Struct43>(
                ret_val + var_20 + 0x26,
            )));
            // puVar17 = (uVar25 >> 0x10);
            CreateDC16(
                read_string_from_addr(0x1008),
                read_string_from_addr(struct_25.field_0x0 as u32),
                &var_17,
                var_17,
            );

            b_force_background = palette_op_1008_4e08(
                (ctx.PTR_LOOP_1050_4230.field_0xe),
                stack0xffec,
                var_17,
                0x1008,
            );
            handle = SelectObject16(0x1008, CONCAT11(var_30, var_23));
            hdc = ctx.s_tile2_bmp_1050_1538 as HDC16;
            handle_29 = SelectObject16(ctx.s_tile2_bmp_1050_1538 as HDC16, handle_29);
            // TODO: refactor for loop
            // for (iVar15 = 0x0; pi_var1 = (__return_storage_ptr__ + 0x74),
            //     *pi_var1 != iVar15 && iVar15 <= *pi_var1; iVar15 += 0x1) {
            //   iVar20 = (iVar15 * 0x10 + param_5) * 0x8;
            //   puVar17 = (__return_storage_ptr__ + 0x72);
            //   hdc = 0x1000;
            //   b_force_background = 0x48e1;
            //   u_var11 = pass1_1000_484c(CONCAT13((unaff_SS >> 0x8),
            //                                     CONCAT12(unaff_SS,&stack0xfff2)),
            //                            CONCAT13((puVar17 >> 0x8),
            //                                     CONCAT12(puVar17,
            //                                              iVar20 + (__return_storage_ptr__
            //                                                               + 0x7))),0x8);
            //   if (u_var11 != 0x0) {
            //     u_var4 = (__return_storage_ptr__ + 0x7);
            //     uVar22 = (u_var4 >> 0x10);
            //     iVar19 = u_var4;
            //     iVar21 = iVar20 + iVar19;
            //     hdc = ctx.s_tile2_bmp_1050_1538;
            //     b_force_background = (HPALETTE16)&ctx.PTR_LOOP_1050_4908;
            //     Rectangle16(0x1000,(iVar21 + 0x6),(iVar21 + 0x4),
            //                 (iVar21 + 0x2),(iVar19 + iVar20));
            //   }
            // }
            SelectPalette16(hdc, 0x0, b_force_background);
            DeleteObject16(ctx.s_tile2_bmp_1050_1538 as u16);
            SelectObject16(ctx.s_tile2_bmp_1050_1538 as u16, handle);
            SelectObject16(ctx.s_tile2_bmp_1050_1538 as u16, handle_29);
            DeleteDC16(ctx.s_tile2_bmp_1050_1538 as u16);
            DeleteObject16(ctx.s_tile2_bmp_1050_1538 as u16);
            return var_17 as u16;
        }
        0x8 => {
            var_23 = 0x3;
            //     TODO: goto LAB_1010_2ad8;
        }
        0xd => {
            var_3 = (u_var11 + unaff_si) as u32;
            var_23 = *var_3;
            var_5 = *var_3 + param_7;
            *var_3 = var_5 + (u_var11 < 0x1c);
            var_26 = (CARRY1(var_23, param_7) || CARRY1(var_5, u_var11 < 0x1c));
            u_var6 = param_8 + 0xeff0;
            var_23 = param_8 < 0x1010 || u_var6 < var_26;
            var_12 = u_var6 - var_26;
            var_8 = swi(0x4);
            if (SBORROW2(param_8 as u32, 0x1010) != SBORROW2(u_var6 as u32, var_26 as u32)) {
                (*var_8)();
                *param_7 = extraout_DX_00;
            }
            bool_24 = var_12 < 0x1010 || var_12 + 0xeff0 < var_23;
            var_3 = (u_var11 + unaff_si) as u32;
            var_23 = *var_3;
            var_16 = *param_7 as u8;
            var_5 = *var_3;
            *var_3 = var_5 + var_16 + bool_24;
            string_2 = (u_var11 + unaff_si);
            *string_2 =
                *string_2 + var_16 + (CARRY1(var_23, var_16) || CARRY1(var_5 + var_16, bool_24));
            struct_op_1018_4cda(
                CONCAT11(var_31, var_30) as i16,
                CONCAT11(param_1 as u8, var32),
                CONCAT11(param_2 as u8, param_1._1_1_),
            );
            var_15 = CONCAT11(var_31, var_30) as i16;
            var_9 = CONCAT13(param_1, CONCAT12(var32, var_15 as u16));
            var_9 = (ctx.s_SCInternalPutBldg2_site_0x_08lx__1050_5099 + 0x1);
            (var_15 + 0x2) = 0x1010;
            pass1_1018_4dce(
                CONCAT13(param_1, CONCAT12(var32, var_15 as u16)),
                0x1b3,
                param_7,
                unaff_ss,
            );
            ctx.PTR_LOOP_1050_4230 = CONCAT13(param_1, CONCAT12(var32, CONCAT11(var_31, var_30)));
            return CONCAT11(param_1 as u8, var32);
        }
        0xe => {
            (ret_val + 0x2) = param_5;
        }
        0x11 => {}
        0x12 => {
            ctx.PTR_LOOP_1050_10c6 = (0x0 < param_5);
            ctx.PTR_LOOP_1050_1142 = (0x2 < param_5);
        }

        0x13 => {
            var_15 = (ret_val * 0x8 + param_1) as i16;
            if ((((var_15 + 0x22) != 0x0) || ((var_15 + 0x24) != 0x0))
                || ((var_15 + 0x26) != 0x0 || ((var_15 + 0x28) != 0x0)))
            {
                u_var4 = (param_1 + 0xe) as u32;
                sys_1000_3f9c(
                    ctx,
                    u_var4,
                    (u_var4 >> 0x10),
                    ctx.s__d__d__d__d_1050_14ae as u16,
                    ctx.data_seg,
                    ((ret_val * 0x8 + param_1 + 0x22) as u16),
                    &stack0xfffa,
                    param_2 as i16,
                    0x1000,
                    unaff_ss,
                    in_af as u32,
                );
                u_var4 = (param_1 + 0xa) as u32;
                WritePrivateProfileString16(
                    read_string_from_addr(ctx.PTR_LOOP_1050_1000),
                    u_var4,
                    (u_var4 >> 0x10),
                    (param_1 + 0xe),
                );
            }
            return param_7;
        }
        0x14 => {
            (ret_val + 0x24) = param_5;
            // break;
        }
        0x17 => {
            var_17 = (param_7 - 0x1);
            var_3 = (u_var11 + unaff_si) as u32;
            *var_3 = *var_3 | var_17;
            (ret_val + 0x12) = param_8 as u32;
            (ret_val + 0x14) = var_17;
            u_var11 = 0x0;
            loop {
                if (*in_stack_0000ffca <= u_var11) {
                    b_var14 = read_file_1008_7dee(
                        param_5,
                        param_6 as u16,
                        (ret_val + 0x1a) as u16,
                        0x0,
                        param_4,
                        0x2,
                        0x1008,
                    );
                    if (((b_var14 != 0x0)
                        && (
                            b_var14 = read_file_1008_7dee(
                                param_5,
                                param_6 as u16,
                                (ret_val + 0x1c) as u16,
                                0x0,
                                param_4,
                                0x2,
                                0x1008,
                            ),
                            b_var14 != 0x0,
                        ))
                        && (
                            b_var14 = read_file_1008_7dee(
                                param_5,
                                param_6 as u16,
                                (ret_val + 0x1e) as u16,
                                0x0,
                                param_4,
                                0x2,
                                0x1008,
                            ),
                            b_var14 != 0x0,
                        ))
                    {
                        return var_17 as u16;
                    }
                    ctx.PTR_LOOP_1050_0310 = 0x6d2;
                    return var_17 as u16;
                }
                var_26 = in_stack_0000ffca;
                mem_op_1000_179c(0x8, var_17, 0x1000);
                pu_var27 = CONCAT22(var_17 as u16, in_stack_0000ffca);
                var_18 = (var_17 | in_stack_0000ffca);
                if (var_18 == 0x0) {
                    pu_var27 = 0x0;
                } else {
                    pu_var27 = 0x389a;
                    (in_stack_0000ffca + 0x2) = 0x1008;
                    pu_var27 = 0xa1c4;
                    (in_stack_0000ffca + 0x2) = 0x1010;
                }
                b_var14 = read_file_1008_7dee(
                    param_5,
                    param_6 as u16,
                    &stack0xffde,
                    0x0,
                    unaff_ss,
                    0x2,
                    0x1008,
                );
                if ((b_var14 == 0x0)
                    || (
                        b_var14 = read_file_1008_7dee(
                            param_5,
                            param_6 as u16,
                            (pu_var27 + 0x6) as u16,
                            0x0,
                            ((pu_var27 >> 0x10) as u16),
                            0x2,
                            0x1008,
                        ),
                        b_var14 == 0x0,
                    ))
                {
                    // break
                };
                var_15 = switch_1008_73ea(param_5, param_6 as u16, in_stack_0000ffde as i16);
                (pu_var27 + 0x4) = var_15 as u32;
                func_ptr_1 = ((ret_val + 0x12) + 0x4);
                (**func_ptr_1)();
                u_var11 += 0x1;
                var_17 = extraout_DX_02;
                *in_stack_0000ffca = var_26;
            }
            if (pu_var27 == 0x0) {
                ctx.PTR_LOOP_1050_0310 = 0x6d2;
                return var_18 as u16;
            }
            func_ptr_1 = *pu_var27;
            (**func_ptr_1)();
            ctx.PTR_LOOP_1050_0310 = 0x6d2;
            return extraout_DX_01;
        }
        0x18 => {
            var_23 = 0x2
            //     TODO: goto LAB_1010_2ad8;
        }
        0x19 => {
            var_13 = pass1_1010_6ca2(
                CONCAT13(var_28 as u16, CONCAT12(param_4 as u8, *ret_val)),
                0x8,
                *param_7,
                unaff_ss,
            );
            if var_13 != 0x0 {
                ret_val = (ctx.s_version__d__d_1050_0012.field_0x8);
                pass1_1010_715c(
                    CONCAT22(0x1a, var_10 as u16),
                    0x1a,
                    var_13,
                    *param_7,
                    unaff_di,
                    unaff_ss,
                );
            }
            if param_5 == 0x2c {
                pass1_1010_715c(
                    CONCAT22(0x1d, ret_val as u16),
                    0x1d,
                    var_13,
                    *param_7,
                    unaff_di,
                    unaff_ss,
                );
            }
            var_13 = pass1_1010_6ca2(0x5a, 0x2, *param_7, unaff_ss);
            if var_13 != 0x0 {
                pass1_1010_715c(0x1c005a, 0x1c, var_13, *param_7, unaff_di, unaff_ss);
            }
            return *param_7;
        }
        0x1a => {
            (ret_val + 0x26) = *param_5;

            var_23 = 0x1;
            //LAB_1010_2ad8:
            if (var_23 == 0x1) || (var_23 == 0x2) {
                if var_23 == 0x1 {
                    *param_5 =
                        ((ret_val + 0x2) + (ret_val + 0x22) + (ret_val + 0x24) + (ret_val + 0x26))
                            as u16;
                }
                if param_5 != 0x0 {
                    *param_7 = *param_5 >> 0xf;
                    *param_5 = *param_5 / 0x2 + 0x1;
                    if 0x5 < *param_5 {
                        *param_5 = 0x5;
                    }
                    if ((var_23 == 0x1) && (ctx.PTR_LOOP_1050_10c6 != 0x0)) && (0x4 < param_5) {
                        *param_5 = 0x4;
                    }
                }
            }
            (var_23 * 0x7c + 0xed6) = *param_5;
            pass1_1010_1f62(
                unaff_ss,
                CONCAT13(var_28 as u16, CONCAT12(param_4 as u8, *ret_val)),
                0xc,
            );
            // switchD_1010_2ab5_caseD_0:
            return param_7;
        }
        _ => {
            //     TODO: goto switchD_1010_2ab5_caseD_0;
        }
    }
}

pub unsafe fn pt_in_rect_1010_40f8(
    ctx: &mut AppContext,
    param_1: u32,
    point_2: &POINT16,
    mut rect_3: &mut RECT16,
    in_dx: &mut Struct19,
    unaff_di: i16,
    unaff_ss: &mut WNDCLASS16,
) -> i16 {
    let pi_var1: U32Ptr;
    let ppc_var2: u32;
    let b_var3: bool;
    let u_var4: u16;
    let u_var5: u16;
    let i_var6: i16;
    let struct_7: &mut Struct18 = &mut Struct18 {
        field_0x0: 0,
        field_0x1: 0,
        field_0x2: 0,
        field_0x6: 0,
        field_0x12: 0,
        field_0x16: 0,
        field_0x18: 0,
        field_0x2a: 0,
        field_0x8e: 0,
        field_0x92: 0,
        field_0x94: 0,
    };
    let mut string_8: String;
    let u_var9: u16;
    let pu_var10: U32Ptr;
    let pu_stack16: u32;

    let mut i_stack6 = 0x0;
    let mut u_stack4 = 0x0;
    loop {
        // uVar9 = (param_1 >> 0x10);
        pi_var1 = (param_1 + 0x74);
        if *pi_var1 == i_stack6 || *pi_var1 < i_stack6 {
            //LAB_1010_413e:
            if (u_stack4 != 0x0) && (0x3 < ctx.PTR_LOOP_1050_3960) {
                pu_var10 = mixed_1010_20ba(
                    ctx,
                    ctx.PTR_LOOP_1050_0ed0,
                    (i_stack6 + 0xc) as u32,
                    unaff_ss,
                    in_dx,
                    unaff_di,
                    0,
                );
                // puVar7 = (puVar10 >> 0x10);
                u_var4 = pass1_1018_0afa(pu_var10);
                if u_var4 == 0x0 {
                    u_var9 = 0x1000;
                    u_var5 = u_var4;
                    let mut var_503 = 0xb4;
                    mem_op_1000_179c(ctx, &mut var_503, struct_7, 0x1000);
                    string_8 = (struct_7 | u_var5);
                    if string_8 == 0x0 {
                        i_var6 = 0x0;
                        string_8 = 0x0;
                    } else {
                        u_var9 = ctx.PTR_LOOP_1050_1040 as u16;
                        i_var6 = string_1040_8520(
                            struct_7,
                            ctx.PTR_LOOP_1050_0396,
                            0x30,
                            0x2,
                            0x643,
                            0x645,
                            &mut string_8,
                            unaff_ss,
                        );
                    }
                    pu_stack16 = CONCAT22(string_8 as u16, i_var6 as u16);
                    ppc_var2 = (*pu_stack16 + 0x74);
                    (**ppc_var2)(u_var9, i_var6, string_8);
                    pass1_1010_209e(ctx.PTR_LOOP_1050_0ed0, (i_stack6 + 0xc) as u16);
                    u_stack4 = u_var4;
                }
            }
            if u_stack4 != 0x0 {
                return i_stack6;
            }
            return -0x1;
        }
        *in_dx = (param_1 + 0x72);
        b_var3 = PtInRect16(rect_3, point_2);
        if b_var3 != false {
            u_stack4 = 0x1;
            //       TODO: goto LAB_1010_413e;
        }
        i_stack6 += 0x1;
        rect_3 = read_struct_from_addr::<RECT16>(ctx.s_tile2_bmp_1050_1538);
    }
}

pub unsafe fn draw_1010_47ae(ctx: &mut AppContext, param_1: u32, param_2: u16, param_3: u16) {
    let UStack4: u16;

    UStack4 = 0x0;
    loop {
        draw_op_1010_47d0(
            ctx,
            param_1,
            (param_1 >> 0x10),
            UStack4,
            param_2,
            param_3 as i16,
        );
        UStack4 += 0x1;
        if UStack4 >= 0x10 {
            break;
        }
    }
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn draw_op_1010_47d0(
    ctx: &mut AppContext,
    param_1: u32,
    param_2: u16,
    param_3: u16,
    in_style_3: i16,
    param_5: u16,
) {
    let pi_var1: U32Ptr;
    let pu_var2: u32;
    let ppc_var3: u32;
    let i_var4: i16;
    let b_force_background: HPALETTE16;
    let handle: HGDIOBJ16;
    let handle_00: HGDIOBJ16;
    let u_var5: u16;
    let extraout_dx: U32Ptr;
    let puVar6: U32Ptr;
    let mut output: String;
    let iVar6: &mut Struct5;
    let iVar7: i16;
    let i_var9: &mut Struct4;
    let uVar8: u16;
    let hdc: HDC16;
    let uVar9: u32;
    let init_data = 0;
    let u_var10: u32;
    let iStack32: i16;
    let local_14: HDC16;
    let mut var_18: String;
    let mut var_16: String;
    let local_e: u16;
    let uStack12: u16;
    let uStack10: u16;
    let uStack8: u16;
    let stock_obj_handle: HGDIOBJ16;
    let pen_handle: HPEN16;
    let mut u_var10 = 0x1;
    let mut pen_handle = CreatePen16(in_style_3, -0x2805, 0x77);
    let mut uVar8 = 0x5;
    let mut stock_obj_handle = GetStockObject16(ctx.s_tile2_bmp_1050_1538);
    let mut local_e = 0x0;
    let mut uStack12 = 0x0;
    let mut uStack10 = 0x1;
    let mut uStack8 = 0x1;
    let mut pu_var2 = (param_1 + 0x26 + param_3 * 0x4);
    let mut puVar6 = (param_1 + 0x26 + param_3 * 0x4 + 0x2);
    if (puVar6 | pu_var2) != 0x0 {
        ppc_var3 = *pu_var2;
        (**ppc_var3)(
            ctx.s_tile2_bmp_1050_1538,
            pu_var2,
            puVar6,
            0x1,
            uVar8,
            u_var10,
        );
        puVar6 = extraout_dx;
    }
    let mut i_var4 = param_3 + 0x105;
    pass1_1010_8170(
        ctx.PTR_LOOP_1050_14cc,
        i_var4,
        puVar6 as i16,
        ctx.s_tile2_bmp_1050_1538,
    );
    iVar7 = (param_3 * 0x4) as i16;
    (param_1 + iVar7 + 0x26) = i_var4 as u32;
    (param_1 + iVar7 + 0x28) = puVar6;
    let mut init_data = DEVMODEA::new();
    uVar9 = pass1_1008_4772((param_1 + 0x26 + iVar7));
    // output = (uVar9 >> 0x10);
    var_18 = uVar9;
    var_16 = output;
    local_14 = CreateDC16(0x1008, &var_18, &output, &init_data);
    b_force_background =
        palette_op_1008_4e08((ctx.PTR_LOOP_1050_4230 + 0xe), &local_14, output, 0x1008);
    handle = SelectObject16(0x1008, pen_handle);
    hdc = ctx.s_tile2_bmp_1050_1538 as u16;
    handle_00 = SelectObject16(ctx.s_tile2_bmp_1050_1538 as u16, stock_obj_handle);
    iStack32 = 0x0;
    loop {
        pi_var1 = (param_1 + 0x74);
        if (*pi_var1 == iStack32 || *pi_var1 < iStack32) {
            break;
        }
        i_var4 = ((iStack32 * 0x10 + param_3) * 0x8) as u16;
        hdc = 0x1000;
        u_var5 = pass1_1000_484c(
            &mut CONCAT22(param_5, &local_e),
            &mut CONCAT22(((param_1 + 0x72) as u16), i_var4 + (param_1 + 0x70)),
            0x8,
        );
        if (u_var5 != 0x0) {
            u_var10 = (param_1 + 0x70);
            // uVar8 = (u_var10 >> 0x10);
            iVar7 = u_var10 as i16;
            i_var9 = (i_var4 + iVar7);
            hdc = ctx.s_tile2_bmp_1050_1538 as u16;
            Rectangle16(
                0x1000,
                i_var9.field_0x6,
                i_var9.field_0x4,
                i_var9.field_0x2,
                (iVar7 + i_var4),
            );
        }
        iStack32 += 0x1;
    }
    SelectPalette16(hdc, 0x0, b_force_background);
    DeleteObject16(ctx.s_tile2_bmp_1050_1538 as u16);
    SelectObject16(ctx.s_tile2_bmp_1050_1538 as u16, handle);
    SelectObject16(ctx.s_tile2_bmp_1050_1538 as u16, handle_00);
    DeleteDC16(ctx.s_tile2_bmp_1050_1538 as u16);
    DeleteObject16(ctx.s_tile2_bmp_1050_1538 as u16);
    return;
}

pub fn pt_in_rect_1010_4e08(
    ctx: &mut AppContext,
    param_1: &mut Struct_1010_4e08,
    point: &POINT16,
    mut rect: &mut RECT16,
) {
    let var1: U32Ptr;
    let var2: bool;
    let b_result: bool;
    let var4: i16;
    let var5: u16;
    let var12: i16;
    let var10: i16;
    let var8: i16;

    // u_var5 = (param_1 >> 0x10);
    let mut i_var4 = param_1;
    (i_var4.field_0x22) = (i_var4.field_0x20);
    var2 = false;
    (i_var4.field_0x24) = 0x0;
    let mut var12 = 0x0;
    let mut iStack10 = 0x0;
    loop {
        let var1 = (i_var4.field_0x30) as u32;
        if *var1 == var12 || *var1 < var12 {
            //LAB_1010_4e67:
            if iStack10 != 0x0 {
                (i_var4.field_0x20) = iStack10;
            }
            if var2 {
                return;
            }
            return;
        }
        let b_result = PtInRect16(rect, point);
        if b_result != false {
            iStack10 = var12;
            var2 = true;
            //       TODO: goto LAB_1010_4e67;
        }
        var12 += 0x1;
        rect = read_struct_from_addr::<RECT16>(ctx.s_tile2_bmp_1050_1538);
    }
}
