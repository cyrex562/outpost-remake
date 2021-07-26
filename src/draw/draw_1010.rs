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
    util::{CONCAT12, CONCAT13, CONCAT22, SUB42},
    win_struct::{HDC16, HGDIOBJ16, HPALETTE16, POINT16, RECT16},
    winapi::{
        CreateDC16, DeleteDC16, DeleteObject16, PtInRect16, Rectangle16, SelectObject16,
        SelectPalette16,
    },
};
use crate::defines::U32Ptr;
use crate::winapi::{GetStockObject16, CreatePen16, WritePrivateProfileString16, swi};
use crate::win_struct::HPEN16;
use crate::global::AppContext;
use crate::file::file_1008::read_file_1008_7dee;
use crate::sys_api::sys_1000_3f9c;
use crate::util::{CONCAT11, SBORROW2};
use crate::pass::pass_1018::pass1_1018_4dce;
use crate::struct_ops::struct_1018::struct_op_1018_4cda;
use crate::file::write::write_to_file_1008_7e1c;

pub unsafe fn draw_fn_1010_2a32(
    param_1: u16,
    param_2: u16,
    __return_storage_ptr__: U32Ptr,
    param_4: u16,
    param_5: &mut u16,
    param_6: u32,
    param_7: &mut u16,
    param_8: u16,
    param_9: u16,
    param_10: u16,
    unaff_SS: u16,
    unaff_SI: u16,
    unaff_DI: u16,
    in_AF: u8,
) -> u16 {
    let pi_var1: U32Ptr;
    let mut pcVar2: String;
    let pbVar3: U32Ptr;
    let u_var4: u32;
    let bVar5: u8;
    let u_var6: u16;
    let ppcVar7: u32;
    let pcVar8: u32;
    let puVar9: U32Ptr;
    let puVar10: U32Ptr;
    let u_var11: u16;
    let b_force_background: HPALETTE16;
    let handle: HGDIOBJ16;
    let uVar12: u16;
    let uVar13: u16;
    let BVar14: bool;
    let iVar15: i16;
    let bVar16: u8;
    let extraout_dx: U32Ptr;
    let extraout_DX_00: U32Ptr;
    let puVar17: U32Ptr;
    let extraout_DX_01: U32Ptr;
    let extraout_DX_02: U32Ptr;
    let puVar18: U32Ptr;
    let iVar19: i16;
    // let unaff_SI: i16;
    let iVar20: i16;
    let iVar21: i16;
    // let unaff_DI: u16;
    let uVar22: u16;
    let hdc: HDC16;
    // let unaff_SS: u16;
    let bVar23: u8;
    let bVar24: bool;
    // let in_AF: u8;
    let uVar25: u32;
    let in_stack_0000ffca: u16;
    let uVar26: u16;
    let pu_var27: u32;
    let in_stack_0000ffde: i16;
    let uVar28: u8;
    let HVar29: HGDIOBJ16;
    let uVar30: u8;
    let uVar31: u8;
    let uVar32: u8;

    puVar10 = __return_storage_ptr__;
    uVar31 = param_9;
    uVar32 = (param_9 >> 0x8);
    bVar23 = 0x0;
    uVar30 = 0x0;
    uVar28 = (param_4 >> 0x8);
    if (param_6 + 0xec76 & 0x3) != 0x0 {
        // goto LAB_1010_2ad8;
    }
    u_var11 = param_6 + 0xec76 >> 0x1;
    if 0x1c < u_var11 {
        //goto LAB_1010_2ad8;
    }
    match u_var11 {
        _ => {}
        //     TODO: goto switchD_1010_2ab5_caseD_0;
        0x1 | 0x3 | 0xb => {
            (u_var11 + 0xa) = param_8;
        }
        0x9 | 0xf | 0x15 | 0x1b => {
            (u_var11 + 0xa) = param_8;
            (u_var11 + 0x10) = param_8;
            (u_var11 + 0xc) = param_8;
            return param_10;
        }
        0x5 => {
            BVar14 = write_to_file_1008_7e1c(
                param_5,
                param_6,
                param_8,
                0x0,
                CONCAT13(param_1._1_1_, CONCAT12(param_1, param_9)),
                0x1008,
            );
            if (BVar14 != 0x0) {
                return param_7;
            }
            ctx.PTR_LOOP_1050_0310 = 0x6d0;
            return param_7;
        }
        0x6 => {
            bVar23 = 0x0;
            //     TODO: goto LAB_1010_2ad8;
        }
        0x7 => {
            ppcVar7 = param_8;
            (**ppcVar7)();
            iVar15 = param_5 + 0x105;
            puVar17 = extraout_dx;
            pass1_1010_8170(ctx.PTR_LOOP_1050_14cc, iVar15, extraout_dx, 0x1010);
            iVar20 = param_5 * 0x4;
            (__return_storage_ptr__ + iVar20 + 0x26) = iVar15;
            (__return_storage_ptr__ + iVar20 + 0x28) = puVar17;
            HVar29 = ctx.data_seg;
            uVar25 = pass1_1008_4772((__return_storage_ptr__ + iVar20 + 0x26));
            // puVar17 = (uVar25 >> 0x10);
            CreateDC16(0x1008, uVar25, puVar17, puVar17);
            b_force_background = palette_op_1008_4e08(
                (ctx.PTR_LOOP_1050_4230 + 0xe),
                &stack0xffec,
                puVar17,
                0x1008,
            );
            handle = SelectObject16(0x1008, CONCAT11(uVar30, bVar23));
            hdc = ctx.s_tile2_bmp_1050_1538;
            HVar29 = SelectObject16(ctx.s_tile2_bmp_1050_1538, HVar29);
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
            DeleteObject16(ctx.s_tile2_bmp_1050_1538);
            SelectObject16(ctx.s_tile2_bmp_1050_1538, handle);
            SelectObject16(ctx.s_tile2_bmp_1050_1538, HVar29);
            DeleteDC16(ctx.s_tile2_bmp_1050_1538);
            DeleteObject16(ctx.s_tile2_bmp_1050_1538);
            return puVar17;
        }
        0x8 => {
            bVar23 = 0x3;
            //     TODO: goto LAB_1010_2ad8;
        }
        0xd => {
            pbVar3 = (u_var11 + unaff_SI);
            bVar23 = *pbVar3;
            bVar5 = *pbVar3 + param_7;
            *pbVar3 = bVar5 + (u_var11 < 0x1c);
            uVar26 = (CARRY1(bVar23, param_7) || CARRY1(bVar5, u_var11 < 0x1c));
            u_var6 = param_8 + 0xeff0;
            bVar23 = param_8 < 0x1010 || u_var6 < uVar26;
            uVar12 = u_var6 - uVar26;
            pcVar8 = swi(0x4);
            if (SBORROW2(param_8, 0x1010) != SBORROW2(u_var6, uVar26)) {
                (*pcVar8)();
                *param_7 = extraout_DX_00;
            }
            bVar24 = uVar12 < 0x1010 || uVar12 + 0xeff0 < bVar23;
            pbVar3 = (u_var11 + unaff_SI);
            bVar23 = *pbVar3;
            bVar16 = *param_7;
            bVar5 = *pbVar3;
            *pbVar3 = bVar5 + bVar16 + bVar24;
            pcVar2 = (u_var11 + unaff_SI);
            *pcVar2 = *pcVar2 + bVar16 + (CARRY1(bVar23, bVar16) || CARRY1(bVar5 + bVar16, bVar24));
            struct_op_1018_4cda(
                CONCAT11(uVar31, uVar30),
                CONCAT11(param_1, uVar32),
                CONCAT11(param_2, param_1._1_1_),
            );
            iVar15 = CONCAT11(uVar31, uVar30);
            puVar9 = CONCAT13(param_1, CONCAT12(uVar32, iVar15));
            *puVar9 = (s_SCInternalPutBldg2_site_0x_08lx__1050_5099 + 0x1);
            (iVar15 + 0x2) = 0x1010;
            pass1_1018_4dce(
                CONCAT13(param_1, CONCAT12(uVar32, iVar15)),
                0x1b3,
                param_7,
                unaff_SS,
            );
            ctx._PTR_LOOP_1050_4230 = CONCAT13(param_1, CONCAT12(uVar32, CONCAT11(uVar31, uVar30)));
            return CONCAT11(param_1, uVar32);
        }
        0xe => {
            (__return_storage_ptr__ + 0x2) = param_5;
        }
        0x11 => {}
        0x12 => {
            ctx.PTR_LOOP_1050_10c6 = (0x0 < param_5);
            ctx.PTR_LOOP_1050_1142 = (0x2 < param_5);
        }

        0x13 => {
            iVar15 = __return_storage_ptr__ * 0x8 + param_1;
            if ((((iVar15 + 0x22) != 0x0) || ((iVar15 + 0x24) != 0x0))
                || ((iVar15 + 0x26) != 0x0 || ((iVar15 + 0x28) != 0x0)))
            {
                u_var4 = (param_1 + 0xe);
                sys_1000_3f9c(
                    u_var4,
                    (u_var4 >> 0x10),
                    s__d__d__d__d_1050_14ae,
                    ctx.data_seg,
                    (__return_storage_ptr__ * 0x8 + param_1 + 0x22),
                    &stack0xfffa,
                    param_2,
                    0x1000,
                    unaff_SS,
                    in_AF,
                );
                u_var4 = (param_1 + 0xa);
                WritePrivateProfileString16(
                    &ctx.PTR_LOOP_1050_1000,
                    u_var4,
                    (u_var4 >> 0x10),
                    (param_1 + 0xe),
                );
            }
            return param_7;
        }
        0x14 => {
            (__return_storage_ptr__ + 0x24) = param_5;
            // break;
        }
        0x17 => {
            puVar17 = (param_7 - 0x1);
            pbVar3 = (u_var11 + unaff_SI);
            *pbVar3 = *pbVar3 | puVar17;
            (__return_storage_ptr__ + 0x12) = param_8;
            (__return_storage_ptr__ + 0x14) = puVar17;
            u_var11 = 0x0;
            loop {
                if (in_stack_0000ffca <= u_var11) {
                    BVar14 = read_file_1008_7dee(
                        param_5,
                        param_6,
                        __return_storage_ptr__ + 0x1a,
                        0x0,
                        param_4,
                        0x2,
                        0x1008,
                    );
                    if (((BVar14 != 0x0)
                        && (
                            BVar14 = read_file_1008_7dee(
                                param_5,
                                param_6,
                                __return_storage_ptr__ + 0x1c,
                                0x0,
                                param_4,
                                0x2,
                                0x1008,
                            ),
                            BVar14 != 0x0,
                        ))
                        && (
                            BVar14 = read_file_1008_7dee(
                                param_5,
                                param_6,
                                __return_storage_ptr__ + 0x1e,
                                0x0,
                                param_4,
                                0x2,
                                0x1008,
                            ),
                            BVar14 != 0x0,
                        ))
                    {
                        return puVar17;
                    }
                    ctx.PTR_LOOP_1050_0310 = 0x6d2;
                    return puVar17;
                }
                uVar26 = in_stack_0000ffca;
                mem_op_1000_179c(0x8, puVar17, 0x1000);
                pu_var27 = CONCAT22(puVar17, in_stack_0000ffca);
                puVar18 = (puVar17 | in_stack_0000ffca);
                if (puVar18 == 0x0) {
                    pu_var27 = 0x0;
                } else {
                    pu_var27 = 0x389a;
                    (in_stack_0000ffca + 0x2) = 0x1008;
                    pu_var27 = 0xa1c4;
                    (in_stack_0000ffca + 0x2) = 0x1010;
                }
                BVar14 =
                    read_file_1008_7dee(param_5, param_6, &stack0xffde, 0x0, unaff_SS, 0x2, 0x1008);
                if ((BVar14 == 0x0)
                    || (
                        BVar14 = read_file_1008_7dee(
                            param_5,
                            param_6,
                            pu_var27 + 0x6,
                            0x0,
                            (pu_var27 >> 0x10),
                            0x2,
                            0x1008,
                        ),
                        BVar14 == 0x0,
                    ))
                {
                    // break
                };
                iVar15 = switch_1008_73ea(param_5, param_6, in_stack_0000ffde);
                (pu_var27 + 0x4) = iVar15;
                ppcVar7 = ((__return_storage_ptr__ + 0x12) + 0x4);
                (**ppcVar7)();
                u_var11 += 0x1;
                puVar17 = extraout_DX_02;
                in_stack_0000ffca = uVar26;
            }
            if (pu_var27 == 0x0) {
                ctx.PTR_LOOP_1050_0310 = 0x6d2;
                return puVar18;
            }
            ppcVar7 = *pu_var27;
            (**ppcVar7)();
            ctx.PTR_LOOP_1050_0310 = 0x6d2;
            return extraout_DX_01;
        }
        0x18 => {
            bVar23 = 0x2;
            //     TODO: goto LAB_1010_2ad8;
        }
        0x19 => {
            uVar13 = pass1_1010_6ca2(
                CONCAT13(uVar28, CONCAT12(param_4, *__return_storage_ptr__)),
                0x8,
                param_7,
                unaff_SS,
            );
            if (uVar13 != 0x0) {
                *__return_storage_ptr__ = (ctx.s_version__d__d_1050_0012 + 0x8);
                pass1_1010_715c(
                    CONCAT22(0x1a, puVar10),
                    0x1a,
                    uVar13,
                    param_7,
                    unaff_DI,
                    unaff_SS,
                );
            }
            if (param_5 == 0x2c) {
                pass1_1010_715c(
                    CONCAT22(0x1d, __return_storage_ptr__),
                    0x1d,
                    uVar13,
                    param_7,
                    unaff_DI,
                    unaff_SS,
                );
            }
            uVar13 = pass1_1010_6ca2(0x5a, 0x2, param_7, unaff_SS);
            if (uVar13 != 0x0) {
                pass1_1010_715c(0x1c005a, 0x1c, uVar13, param_7, unaff_DI, unaff_SS);
            }
            return param_7;
        }
        0x1a => {
            (__return_storage_ptr__ + 0x26) = param_5;

            bVar23 = 0x1;
            //LAB_1010_2ad8:
            if (bVar23 == 0x1) || (bVar23 == 0x2) {
                if bVar23 == 0x1 {
                    *param_5 = (__return_storage_ptr__ + 0x2)
                        + (__return_storage_ptr__ + 0x22)
                        + (__return_storage_ptr__ + 0x24)
                        + (__return_storage_ptr__ + 0x26);
                }
                if param_5 != 0x0 {
                    *param_7 = *param_5 >> 0xf;
                    *param_5 = *param_5 / 0x2 + 0x1;
                    if 0x5 < *param_5 {
                        *param_5 = 0x5;
                    }
                    if ((bVar23 == 0x1) && (ctx.PTR_LOOP_1050_10c6 != 0x0)) && (0x4 < param_5) {
                        *param_5 = 0x4;
                    }
                }
            }
            (bVar23 * 0x7c + 0xed6) = *param_5;
            pass1_1010_1f62(
                unaff_SS,
                CONCAT13(uVar28, CONCAT12(param_4, *__return_storage_ptr__)),
                0xc,
            );
            // switchD_1010_2ab5_caseD_0:
            return param_7;
        }
    }
}

pub unsafe fn pt_in_rect_1010_40f8(
    ctx: &mut AppContext,
    param_1: u32,
    param_2: &POINT16,
    param_3: &mut RECT16,
    in_dx: U32Ptr,
    unaff_di: i16,
    unaff_ss: u16,
) -> i16 {
    let pi_var1: U32Ptr;
    let ppc_var2: u32;
    let b_var3: bool;
    let u_var4: u16;
    let u_var5: u16;
    let i_var6: i16;
    // let in_DX: U32Ptr;
    let pu_var7: U32Ptr;
    let pu_var8: U32Ptr;
    // let unaff_DI: i16;
    let u_var9: u16;
    // let unaff_SS: u16;
    let pu_var10: U32Ptr;
    let pu_stack16: u32;
    let i_stack6: i16;
    let u_stack4: u16;

    i_stack6 = 0x0;
    u_stack4 = 0x0;
    loop {
        // uVar9 = (param_1 >> 0x10);
        pi_var1 = (param_1 + 0x74);
        if (*pi_var1 == i_stack6 || *pi_var1 < i_stack6) {
            //LAB_1010_413e:
            if ((u_stack4 != 0x0) && (0x3 < ctx.PTR_LOOP_1050_3960)) {
                pu_var10 = mixed_1010_20ba(
                    ctx.PTR_LOOP_1050_0ed0,
                    i_stack6 + 0xc,
                    unaff_ss,
                    in_dx,
                    unaff_di,
                );
                // puVar7 = (puVar10 >> 0x10);
                u_var4 = pass1_1018_0afa(pu_var10);
                if (u_var4 == 0x0) {
                    u_var9 = 0x1000;
                    u_var5 = u_var4;
                    mem_op_1000_179c(0xb4, pu_var7, 0x1000);
                    pu_var8 = (pu_var7 | u_var5);
                    if (pu_var8 == 0x0) {
                        i_var6 = 0x0;
                        pu_var8 = 0x0;
                    } else {
                        u_var9 = SUB42(&ctx.PTR_LOOP_1050_1040, 0x0);
                        i_var6 = string_1040_8520(
                            CONCAT22(pu_var7, u_var5),
                            ctx.PTR_LOOP_1050_0396,
                            0x30,
                            0x2,
                            0x643,
                            0x645,
                            pu_var8,
                            unaff_ss,
                        );
                    }
                    pu_stack16 = CONCAT22(pu_var8, i_var6);
                    ppc_var2 = (*pu_stack16 + 0x74);
                    (**ppc_var2)(u_var9, i_var6, pu_var8);
                    pass1_1010_209e(ctx.PTR_LOOP_1050_0ed0, i_stack6 + 0xc);
                    u_stack4 = u_var4;
                }
            }
            if (u_stack4 != 0x0) {
                return i_stack6;
            }
            return -0x1;
        }
        *in_dx = (param_1 + 0x72);
        b_var3 = PtInRect16(param_3, *param_2);
        if (b_var3 != 0x0) {
            u_stack4 = 0x1;
            //       TODO: goto LAB_1010_413e;
        }
        i_stack6 += 0x1;
        *param_3 = ctx.s_tile2_bmp_1050_1538;
    }
}

pub unsafe fn draw_1010_47ae(param_1: u32, param_2: u16, param_3: u16) {
    let UStack4: u16;

    UStack4 = 0x0;
    loop {
        draw_op_1010_47d0(param_1, (param_1 >> 0x10), UStack4, param_2, param_3);
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
    param_5: u16
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
    let mut pCStack18: String;
    let mut pCStack16: String;
    let local_e: u16;
    let uStack12: u16;
    let uStack10: u16;
    let uStack8: u16;
    let stock_obj_handle: HGDIOBJ16;
    let pen_handle: HPEN16;

    let mut u_var10 = 0x1;
    let mut pen_handle = CreatePen16(in_style_3, -0x2805, 0x77);
    let mut uVar8 = 0x5;
    let mut stock_obj_handle = GetStockObject16(s_tile2_bmp_1050_1538);
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
        puVar6,
        ctx.s_tile2_bmp_1050_1538,
    );
    iVar7 = param_3 * 0x4;
    (param_1 + iVar7 + 0x26) = i_var4;
    (param_1 + iVar7 + 0x28) = puVar6;
    init_data = 0x0;
    uVar9 = pass1_1008_4772((param_1 + 0x26 + iVar7));
    // output = (uVar9 >> 0x10);
    pCStack18 = uVar9;
    pCStack16 = output;
    local_14 = CreateDC16(0x1008, pCStack18, output, init_data);
    b_force_background =
        palette_op_1008_4e08((ctx.PTR_LOOP_1050_4230 + 0xe), &local_14, output, 0x1008);
    handle = SelectObject16(0x1008, pen_handle);
    hdc = ctx.s_tile2_bmp_1050_1538;
    handle_00 = SelectObject16(ctx.s_tile2_bmp_1050_1538, stock_obj_handle);
    iStack32 = 0x0;
    loop {
        pi_var1 = (param_1 + 0x74);
        if (*pi_var1 == iStack32 || *pi_var1 < iStack32) {
            break;
        }
        i_var4 = (iStack32 * 0x10 + param_3) * 0x8;
        hdc = 0x1000;
        u_var5 = pass1_1000_484c(
            CONCAT22(param_5, &local_e),
            CONCAT22((param_1 + 0x72), i_var4 + (param_1 + 0x70)),
            0x8,
        );
        if (u_var5 != 0x0) {
            u_var10 = (param_1 + 0x70);
            // uVar8 = (u_var10 >> 0x10);
            iVar7 = u_var10;
            i_var9 = (i_var4 + iVar7);
            hdc = ctx.s_tile2_bmp_1050_1538;
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
    DeleteObject16(ctx.s_tile2_bmp_1050_1538);
    SelectObject16(ctx.s_tile2_bmp_1050_1538, handle);
    SelectObject16(ctx.s_tile2_bmp_1050_1538, handle_00);
    DeleteDC16(ctx.s_tile2_bmp_1050_1538);
    DeleteObject16(ctx.s_tile2_bmp_1050_1538);
    return;
}

pub unsafe fn pt_in_rect_1010_4e08(param_1: u32, param_2: u16, param_3: u16, param_4: &RECT16) {
    let pi_var1: U32Ptr;
    let bVar2: bool;
    let BVar3: bool;
    let i_var4: i16;
    let u_var5: u16;
    let iStack12: i16;
    let uStack10: i16;
    let PStack8: i16;

    PStack8 = CONCAT22(param_2, param_3);
    // u_var5 = (param_1 >> 0x10);
    i_var4 = param_1;
    (i_var4 + 0x22) = (i_var4 + 0x20);
    bVar2 = false;
    (i_var4 + 0x24) = 0x0;
    iStack12 = 0x0;
    iStack10 = 0x0;
    loop {
        pi_var1 = (i_var4 + 0x30);
        if (*pi_var1 == iStack12 || *pi_var1 < iStack12) {
            //LAB_1010_4e67:
            if (iStack10 != 0x0) {
                (i_var4 + 0x20) = iStack10;
            }
            if (bVar2) {
                return;
            }
            return;
        }
        BVar3 = PtInRect16(param_4, PStack8);
        if (BVar3 != 0x0) {
            iStack10 = iStack12;
            bVar2 = true;
            //       TODO: goto LAB_1010_4e67;
        }
        iStack12 += 0x1;
        param_4 = s_tile2_bmp_1050_1538;
    }
}
