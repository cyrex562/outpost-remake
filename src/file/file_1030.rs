use crate::defines::{
    Struct18, Struct368, Struct369, Struct370, Struct380, Struct381, Struct387, Struct388,
    Struct389, Struct391, Struct430, Struct99, U32Ptr,
};
use crate::file::file_1008::{
    file_1008_76e4, file_1008_77cc, read_file_1008_7bc8, read_file_1008_7c6e, read_file_1008_7dee,
};
use crate::file::write::write_to_file_1008_7e1c;
use crate::fn_ptr::fn_ptr_1000::{fn_ptr_1000_17ce, fn_ptr_op_1000_1708};
use crate::global::AppContext;
use crate::mem_1000::{mem_op_1000_0a48, mem_op_1000_160a, mem_op_1000_179c};
use crate::pass::pass_1000::{pass1_1000_07fc, pass1_1000_5586};
use crate::pass::pass_1008::pass1_1008_766e;
use crate::pass::pass_1028::pass1_1028_e1ec;
use crate::pass::pass_1030::{pass1_1030_4782, pass1_1030_5164, pass1_1030_84ae};
use crate::string::string_1008::str_op_1008_60e8;
use crate::struct_ops::struct_1008::set_struct_1008_574a;
use crate::sys_api::dos3_call_1000_51aa;
use crate::util::{read_struct_from_addr, write_struct_to_addr, CONCAT22, ZEXT24};
use crate::win_struct::HFILE16;
use crate::winapi::{WIN16_hread, _lclose16, _lopen16};

use super::file_1008::read_file_1008_7cfe;

pub fn file_1030_1730(ctx: &mut AppContext, param_1: u32, param_2: u32) {
    let u_var1: u16;
    let b_var2: bool;
    let u_var3: u16;

    // u_var1 = (param_1 >> 0x10);
    // u_var3 = (param_2 >> 0x10);
    b_var2 = read_file_1008_7dee(param_2, u_var3, param_1 + 0x4, 0x0, u_var1, 0x4, 0x1008);
    if (b_var2 != 0x0) {
        b_var2 = read_file_1008_7dee(param_2, u_var3, param_1 + 0x8, 0x0, u_var1, 0x4, 0x1008);
        if (b_var2 != 0x0) {
            return;
        }
    }
    ctx.PTR_LOOP_1050_0310 = 0x6d2;
    return;
}

pub fn file_1030_19b4(
    ctx: &mut AppContext,
    param_1: u32,
    param_2: u32,
    param_3: i16,
    param_4: u16,
    param_5: u16,
) {
    let var1: &i32;

    file_1030_1730(param_1, param_2);
    if (param_3 != 0x0) {
        var1 = (param_1 & 0xffff0000 | (param_1 + 0xc));
        file_1008_76e4(ctx, param_2, var1, 0x1008, param_5, param_4);
        if (var1 == 0x0) {
            ctx.PTR_LOOP_1050_0310 = 0x6d2;
            return;
        }
    }
    return;
}

pub fn file_1030_1b18(
    ctx: &mut AppContext,
    param_1: u32,
    param_2: u32,
    param_3: i16,
    param_4: &mut Struct18,
    param_5: u16,
) -> u16 {
    let u_var1: u32;
    let pi_var2: U32Ptr;
    let u_var3: u16;
    let u_var4: bool;
    let bvar5: bool;
    let u_var6: u16;
    let pu_var7: U32Ptr;
    let i_var7: &mut Struct368;
    let u_var8: u16;
    let u_var9: u16;
    let i_var10: &mut Struct370;
    let i_var9: &mut Struct369;

    i_var10 = param_1;
    // u_var9 = (param_1 >> 0x10);
    file_1030_19b4(ctx, param_1, param_2, param_3, param_4, param_5);
    if param_3 != 0x0 {
        if ctx.PTR_LOOP_1050_5f2c == 0x0 {
            ctx.PTR_LOOP_1050_5f2c = mem_op_1000_160a(ctx, Some(param_4), 0x1000) as u32;
            // ctx.PTR_LOOP_1050_5f2e = param_4;
            write_struct_to_addr::<Struct18>(ctx.PTR_LOOP_1050_5f2e, param_4)
        } else {
        }
        u_var4 = fn_ptr_op_1000_1708(
            ctx,
            0x6,
            0x0,
            0x1,
            read_struct_from_addr::<Struct18>(ctx.PTR_LOOP_1050_5f2c),
            0x1000,
        );
        i_var10.field_0x10 = u_var4;
        (i_var10.field_0x12 + 0x2) = ctx.PTR_LOOP_1050_5f2e;
        pu_var7 = (i_var10.field_0x12);
        u_var4 = param_2;
        bvar5 = read_file_1008_7dee(
            param_2,
            u_var4,
            &i_var10.field_0x10,
            0x0,
            pu_var7,
            0x2,
            0x1008,
        );
        if bvar5 != false {
            pi_var2 = i_var10.field_0x10;
            if *pi_var2 == 0x0 {
                return 0x1;
            }
            u_var3 = *pi_var2 * 0x2;
            u_var6 = u_var3;
            mem_op_1000_179c(u_var3, pu_var7, 0x1000);
            pi_var2 = i_var10.field_0x10;
            // u_var8 = (pi_var2 >> 0x10);
            i_var7 = pi_var2;
            i_var7.field_0x2 = u_var6;
            i_var7.field_0x4 = pu_var7;
            pi_var2 = i_var10.field_0x10;
            u_var1 = (pi_var2 + 0x2);
            bvar5 = read_file_1008_7dee(
                param_2,
                u_var4,
                u_var1,
                0x0,
                (u_var1 >> 0x10),
                u_var3,
                0x1008,
            );
            if (bvar5 != 0x0) {
                return 0x1;
            }
        }
        ctx.PTR_LOOP_1050_0310 = 0x6d2;
    }
    return 0x0;
}

pub fn read_file_1030_33f0(ctx: &mut AppContext, param_1: u32, param_2: u32) {
    let u_var1: u16;
    let i_var2: &mut Struct430;
    let BVar2: bool;
    let u_var3: u16;
    let u_var4: u16;

    i_var2 = param_1;
    i_var2 = &i_var2.field_0x4;
    // u_var1 = (param_1 >> 0x10);
    u_var3 = param_2;
    // u_var4 = (param_2 >> 0x10);
    BVar2 = read_file_1008_7dee(u_var3, u_var4, i_var2, 0x0, u_var1, 0x16c, 0x1008);
    if (((((BVar2 != 0x0)
        && (
            BVar2 = read_file_1008_7dee(
                u_var3,
                u_var4,
                &i_var2.field_0x174,
                0x0,
                u_var1,
                0xc,
                0x1008,
            ),
            BVar2 != 0x0,
        ))
        && (
            BVar2 = read_file_1008_7dee(
                u_var3,
                u_var4,
                &i_var2.field_0x180,
                0x0,
                u_var1,
                0xc,
                0x1008,
            ),
            BVar2 != 0x0,
        ))
        && ((
            BVar2 = read_file_1008_7dee(
                u_var3,
                u_var4,
                &i_var2.field_0x18c,
                0x0,
                u_var1,
                0x18,
                0x1008,
            ),
            BVar2 != 0x0
                && (
                    BVar2 = read_file_1008_7dee(
                        u_var3,
                        u_var4,
                        &i_var2.field_0x1a8,
                        0x0,
                        u_var1,
                        0x2,
                        0x1008,
                    ),
                    BVar2 != 0x0,
                ),
        )))
        && (
            BVar2 = read_file_1008_7dee(
                u_var3,
                u_var4,
                &i_var2.field_0x1aa,
                0x0,
                u_var1,
                0x4,
                0x1008,
            ),
            BVar2 != 0x0,
        ))
    {
        if (ctx.PTR_LOOP_1050_0312 < 0x2) {
            return;
        }
        BVar2 = read_file_1008_7dee(
            u_var3,
            u_var4,
            &i_var2.field_0x170,
            0x0,
            u_var1,
            0x4,
            0x1008,
        );
        if ((BVar2 != 0x0)
            && (
                BVar2 = read_file_1008_7dee(
                    u_var3,
                    u_var4,
                    &i_var2.field_0x1ae,
                    0x0,
                    u_var1,
                    0x2,
                    0x1008,
                ),
                BVar2 != 0x0,
            ))
        {
            return;
        }
    }
    ctx.PTR_LOOP_1050_0310 = 0x6d2;
    return;
}

pub fn read_file_1030_4e70(
    ctx: &mut AppContext,
    param_1: u32,
    param_2: U32Ptr,
    param_3: &mut Vec<u8>,
    param_4: i32,
    param_5: &mut u32,
    stack0xfffe: u16,
    unaff_ss: u16,
) -> u16 {
    let u_var1: u16;
    let file_handle: HFILE16;
    let u_var3: u16;
    let u_var4: u32;
    let l_var5: i32;
    let pb_stack60: U32Ptr;
    let l_stack56: i32;
    let u_stack20: u32;

    *param_3 = 0x0;
    *param_2 = 0x0;
    if param_4 != 0x0 {
        u_var4 = pass1_1030_5164(param_1, param_4, unaff_ss);
        // param_5 = (u_var4 >> 0x10);
        *param_5 = u_var4;
        u_var1 = dos3_call_1000_51aa(stack0xfffe);
        if u_var1 == 0x0 {
            *param_2 = u_stack20;
            file_handle = _lopen16(ctx.PTR_LOOP_1050_1000, 0x0);
            if file_handle != 0xffff {
                l_var5 = mem_op_1000_0a48(ctx, 0x1, param_2, ctx.PTR_LOOP_1050_5f2c, 0x1000);
                // param_5 = (lVar5 >> 0x10);
                param_3 = l_var5;
                (param_3 + 0x2) = param_5;
                if ((param_5 | param_3) != 0x0) {
                    l_stack56 =
                        WIN16_hread(0x1000, *param_2, CONCAT22(*param_3, (*param_2 >> 0x10)));
                    // u_var3 = (lStack56 >> 0x10);
                    _lclose16(ctx.s_tile2_bmp_1050_1538);
                    pb_stack60 = *param_3;
                    while (l_stack56 != 0x0) {
                        if (((*pb_stack60 + 0x608b) & 0x20) == 0x0) {
                            *pb_stack60 = *pb_stack60 + 0x80;
                        }
                        pb_stack60 = (pb_stack60 & 0xffff0000 | (pb_stack60 + 0x1));
                        l_stack56 = l_stack56 + -0x1;
                    }
                    return u_var3;
                }
            }
        }
    }
    return param_5;
}

pub fn file_1030_581e(
    ctx: &mut AppContext,
    param_1: u32,
    param_2: u32,
    param_3: i16,
    param_4: U32Ptr,
    param_5: u16,
) {
    let pi_var1: U32Ptr;
    let i_var2: i16;
    let u_var3: u32;
    let u_var4: u16;
    let BVar5: bool;
    let puVar6: U32Ptr;
    let uVar7: u16;
    let uVar8: u32;
    let puVar9: U32Ptr;
    let i_var9: &mut Struct380;
    let u_var10: u16;
    let in_AF: u8;
    let u_var11: u16;
    let uVar12: u16;
    let uStack1040: u32;
    let iStack1036: i16;
    let mut local_408: Vec<u8>;
    let uStack8: u32;
    let local_4: i16;
    let iVar12: &mut Struct381;

    iVar12 = param_1;
    // uVar12 = (param_1 >> 0x10);
    file_1030_19b4(param_1, param_2, param_3, param_4, param_5);
    if param_3 != 0x0 {
        if ctx.PTR_LOOP_1050_5f2c == 0x0 {
            ctx.PTR_LOOP_1050_5f2c = mem_op_1000_160a(param_4, 0x1000);
            ctx.PTR_LOOP_1050_5f2e = param_4;
        } else {
        }
        u_var4 = fn_ptr_op_1000_1708(
            0x20,
            0x0,
            0x1,
            ctx.PTR_LOOP_1050_5f2c,
            ctx.PTR_LOOP_1050_5f2e,
            0x1000,
        );
        puVar9 = ctx.PTR_LOOP_1050_5f2e | u_var4;
        if puVar9 == 0x0 {
            u_var4 = 0x0;
            puVar9 = 0x0;
        } else {
            pass1_1030_84ae(CONCAT22(ctx.PTR_LOOP_1050_5f2e, u_var4));
        }
        iVar12.field_0x10 = u_var4;
        iVar12.field_0x12 = puVar9;
        u_var4 = param_2;
        // u_var11 = (param_2 >> 0x10);
        BVar5 = read_file_1008_7dee(u_var4, u_var11, &local_4, 0x0, param_5, 0x2, 0x1008);
        if BVar5 != 0x0 {
            uVar8 = ctx.PTR_LOOP_1050_65e2 + 0x52;
            uStack8 = uVar8;
            pass1_1030_4782(
                param_5,
                in_AF,
                puVar9,
                uVar8,
                uVar8 >> 0x10,
                0x0,
                0x1,
                local_4,
            );
            iVar12.field_0x10 = uVar8;
            iVar12.field_0x12 = puVar9;
            BVar5 = read_file_1008_7dee(
                u_var4,
                u_var11,
                iVar12.field_0x10 + 0x2,
                0x0,
                puVar9,
                0x2,
                0x1008,
            );
            if BVar5 != 0x0 {
                puVar6 = local_408;
                read_file_1008_7c6e(u_var4, u_var11, CONCAT22(param_5, puVar6), 0x1008);
                if puVar6 != 0x0 {
                    uVar8 = &iVar12.field_0x10;
                    fn_ptr_1000_17ce(ctx, uVar8 + 0x4, 0x1000);
                    uVar7 = str_op_1008_60e8(CONCAT22(param_5, local_408), puVar9);
                    uVar8 = &iVar12.field_0x10;
                    // u_var10 = (uVar8 >> 0x10);
                    i_var9 = uVar8;
                    i_var9.field_0x4 = uVar7;
                    i_var9.field_0x6 = puVar9;
                    uVar8 = &iVar12.field_0x10;
                    BVar5 = read_file_1008_7dee(
                        u_var4,
                        u_var11,
                        uVar8 + 0x1a,
                        0x0,
                        uVar8 >> 0x10,
                        0x2,
                        0x1008,
                    );
                    if BVar5 != 0x0 {
                        uVar8 = &iVar12.field_0x10;
                        i_var2 = uVar8 + 0x1a;
                        uVar7 = i_var2 * 0x6;
                        mem_op_1000_179c(uVar7, puVar9, 0x1000);
                        uStack1040 = CONCAT22(puVar9, uVar7);
                        if (puVar9 | uVar7) == 0x0 {
                            uVar8 = &iVar12.field_0x10;
                            (uVar8 + 0x16) = 0x0;
                        } else {
                            pass1_1000_5586(0x3e38, 0x1008, i_var2, 0x6, uVar7, puVar9);
                            uVar8 = &iVar12.field_0x10;
                            (uVar8 + 0x16) = uStack1040;
                        }
                        // TODO: refactor for loop
                        // for (iStack1036 = 0x0; uVar8 = &iVar12.field_0x10,
                        //     pi_var1 = (uVar8 + 0x1a),
                        //     *pi_var1 != iStack1036 && iStack1036 <= *pi_var1; iStack1036 += 0x1) {
                        //   uVar8 = &iVar12.field_0x10;
                        //   u_var3 = (uVar8 + 0x16);
                        //   BVar5 = read_file_1008_7bc8(param_2,
                        //                                       (u_var3 & 0xffff0000 |
                        //                                       (u_var3 +
                        //                                                    iStack1036 * 0x6)),0x1008,
                        //                               param_5);
                        //   if (BVar5 == 0x0) goto LAB_1030_58a7;
                        // }
                        BVar5 = read_file_1008_7bc8(
                            param_2,
                            param_1 & 0xffff0000 | &iVar12.field_0x14,
                            0x1008,
                            param_5,
                        );
                        if (BVar5 != 0x0)
                            && (
                                BVar5 = read_file_1008_7dee(
                                    u_var4,
                                    u_var11,
                                    &iVar12.field_0x1c,
                                    0x0,
                                    uVar12,
                                    0x2,
                                    0x1008,
                                ),
                                BVar5 != 0x0,
                            )
                        {
                            return;
                        }
                    }
                }
            }
        }
        //LAB_1030_58a7:
        ctx.PTR_LOOP_1050_0310 = 0x6d2;
    }
    return;
}

pub fn read_file_1030_5c52(
    ctx: &mut AppContext,
    param_1: u32,
    param_2: u32,
    param_3: u16,
    param_4: u16,
) -> bool {
    let b_var1: bool;
    let u_var2: u16;

    // u_var2 = (param_2 >> 0x10);
    read_file_1008_7cfe(param_2, u_var2, 0x9, 0x1008, param_4);
    if (param_3 != 0x0) {
        b_var1 = read_file_1008_7dee(
            param_2,
            u_var2,
            param_1,
            0x0,
            (param_1 >> 0x10),
            0x24,
            0x1008,
        );
        if (b_var1 == 0x0) {
            ctx.PTR_LOOP_1050_0310 = 0x6d2;
            return b_var1;
        }
        param_3 = 0x1;
    }
    return param_3;
}

pub fn file_1030_778c(
    ctx: &mut AppContext,
    param_1: u32,
    param_2: u32,
    param_3: i16,
    param_4: U32Ptr,
    param_5: u16,
) {
    let lVar1: i32;
    let ppcVar2: u32;
    let i_var3: &mut Struct387;
    let BVar3: bool;
    let iVar6: i16;
    let plVar7: &i32;
    let puVar8: u32;
    let extraout_dx: u16;
    let uVar9: u16;
    let puVar10: U32Ptr;
    let extraout_DX_00: u16;
    let i_var4: &mut Struct389;
    let iVar5: &mut Struct391;
    let u_var11: u16;
    let uVar12: u16;
    let uVar13: u16;
    let uVar14: u16;
    let local_56: [u16; 0x2];
    let uStack82: u16;
    let paStack74: &mut Struct99;
    let local_46: [u16; 0x2];
    let local_42: [u16; 0x2];
    let local_3e: [u32; 0x3];
    let paStack50: &mut Struct99;
    let local_2e: u16;
    let paStack44: &mut Struct99;
    let local_28: [u16; 0x2];
    let local_24: [u16; 0x2];
    let local_20: [u16; 0x9];
    let uStack14: u16;
    let local_4: u16;
    let u_var5: &mut Struct388;
    let uVar8: &mut Struct390;

    file_1030_1730(param_1, param_2);
    if (param_3 != 0x0) {
        i_var3 = param_1;
        i_var3 = &i_var3.field_0xc;
        BVar3 = read_file_1008_7bc8(
            param_2,
            (param_1 & 0xffff0000 | ZEXT24(i_var3)),
            0x1008,
            param_5,
        );
        if (BVar3 != 0x0) {
            uVar13 = param_2;
            // uVar14 = (param_2 >> 0x10);
            BVar3 = read_file_1008_7dee(uVar13, uVar14, &local_4, 0x0, param_5, 0x2, 0x1008);
            if (BVar3 != 0x0) {
                // u_var11 = (param_1 >> 0x10);
                i_var3.field_0x12 = local_4;
                BVar3 = read_file_1008_7dee(uVar13, uVar14, &local_4, 0x0, param_5, 0x2, 0x1008);
                if (BVar3 != 0x0) {
                    i_var3.field_0x14 = local_4;
                    BVar3 = read_file_1008_7dee(
                        uVar13,
                        uVar14,
                        &i_var3.field_0x16,
                        0x0,
                        u_var11,
                        0x4,
                        0x1008,
                    );
                    if (BVar3 != 0x0) {
                        plVar7 = (param_1 & 0xffff0000 | &i_var3.field_0x1e);
                        file_1008_76e4(ctx, param_2, plVar7, 0x1008, param_5, param_4);
                        if ((((plVar7 != 0x0)
                            && (
                                iVar6 = file_1008_77cc(
                                    ctx,
                                    param_2,
                                    (param_1 & 0xffff0000 | &i_var3.field_0x22),
                                    param_4,
                                    0x1008,
                                    param_5,
                                ),
                                iVar6 != 0x0,
                            ))
                            && (
                                iVar6 = file_1008_77cc(
                                    param_2,
                                    (param_1 & 0xffff0000 | &i_var3.field_0x26),
                                    param_4,
                                    0x1008,
                                    param_5,
                                ),
                                iVar6 != 0x0,
                            ))
                            && (
                                BVar3 = read_file_1008_7dee(
                                    uVar13,
                                    uVar14,
                                    &i_var3.field_0x2a,
                                    0x0,
                                    u_var11,
                                    0x4,
                                    0x1008,
                                ),
                                BVar3 != 0x0,
                            ))
                        {
                            if (i_var3.field_0x2a != 0x0) {
                                lVar1 = i_var3.field_0x2a;
                                pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, lVar1);
                                i_var3.field_0x2e = BVar3;
                                i_var3.field_0x30 = param_4;
                            }
                            if (ctx.PTR_LOOP_1050_0312 < 0x2) {
                                return;
                            }
                            BVar3 = read_file_1008_7dee(
                                uVar13,
                                uVar14,
                                &i_var3.field_0x32,
                                0x0,
                                u_var11,
                                0x2,
                                0x1008,
                            );
                            if ((BVar3 != 0x0)
                                && (
                                    BVar3 = read_file_1008_7dee(
                                        uVar13,
                                        uVar14,
                                        &i_var3.field_0x34,
                                        0x0,
                                        u_var11,
                                        0x2,
                                        0x1008,
                                    ),
                                    BVar3 != 0x0,
                                ))
                            {
                                puVar8 = (param_1 & 0xffff0000 | &i_var3.field_0x36);
                                pass1_1008_766e(param_2, puVar8, param_5, 0x1008, param_4);
                                if ((puVar8 != 0x0)
                                    && (
                                        BVar3 = read_file_1008_7dee(
                                            uVar13, uVar14, local_20, 0x0, param_5, 0x2, 0x1008,
                                        ),
                                        BVar3 != 0x0,
                                    ))
                                {
                                    // TODO: refactor for loop
                                    // for (uStack14 = 0x0; uStack14 < local_20[0]; uStack14 += 0x1) {
                                    //   local_3e[0] = ctx.PTR_LOOP_1050_68a2;
                                    //   paStack50 = pass1_1000_07fc(0x1000,PTR_LOOP_1050_68a2);
                                    //   uVar9 = (paStack50 >> 0x10);
                                    //   u_var5 = paStack50;
                                    //   puVar10 = (uVar9 | u_var5);
                                    //   if (puVar10 == 0x0) {
                                    //     paStack44 = 0x0;
                                    //   }
                                    //   else {
                                    //     paStack50.field_0x0 = 0x389a;
                                    //     u_var5.field_0x2 = 0x1008;
                                    //     u_var5.field_0x4 = 0x0;
                                    //     u_var5.field_0x6 = 0x0;
                                    //     u_var5.field_0x8 = 0x0;
                                    //     u_var5.field_0xa = 0x0;
                                    //     u_var5.field_0xc = 0x0;
                                    //     paStack50.field_0x0 = 0x56ce;
                                    //     u_var5.field_0x2 = 0x1018;
                                    //     paStack44 = paStack50;
                                    //   }
                                    //   BVar3 = read_file_1008_7dee(uVar13,uVar14,local_28,0x0,param_5
                                    //                               ,0x2,0x1008);
                                    //   if (((BVar3 == 0x0) ||
                                    //       (BVar3 = read_file_1008_7dee(uVar13,uVar14,local_24,0x0,
                                    //                                    param_5,0x2,0x1008), BVar3 == 0x0))
                                    //      || ((BVar3 = read_file_1008_7dee(uVar13,uVar14,&local_2e,
                                    //                                       0x0,param_5,0x2,0x1008),
                                    //          BVar3 == 0x0 ||
                                    //          ((BVar3 = read_file_1008_7dee(uVar13,uVar14,
                                    //                                        paStack44 + 0xa,0x0,
                                    //                                        (paStack44 >> 0x10
                                    //                                                ),0x2,0x1008),
                                    //           BVar3 == 0x0 ||
                                    //           (BVar3 = read_file_1008_7dee(uVar13,uVar14,
                                    //                                        paStack44 + 0xc,0x0,
                                    //                                        (paStack44 >> 0x10
                                    //                                                ),0x2,0x1008),
                                    //           BVar3 == 0x0)))))) goto LAB_1030_77be;
                                    //   uVar12 = (paStack44 >> 0x10);
                                    //   i_var4 = paStack44;
                                    //   i_var4.field_0x4 = local_28[0];
                                    //   i_var4.field_0x6 = local_24[0];
                                    //   i_var4.field_0x8 = local_2e;
                                    //   if (i_var3.field_0x3a == 0x0) {
                                    //     uVar9 = local_2e;
                                    //     mem_op_1000_179c(0xc,puVar10,0x1000);
                                    //     paStack50 = CONCAT22(puVar10,uVar9);
                                    //     if ((puVar10 | uVar9) == 0x0) {
                                    //       i_var3.field_0x3a = 0x0;
                                    //     }
                                    //     else {
                                    //       set_struct_1008_574a(CONCAT22(puVar10,uVar9));
                                    //       &i_var3.field_0x3a = uVar9;
                                    //       (&i_var3.field_0x3a + 0x2) = extraout_dx;
                                    //     }
                                    //   }
                                    //   ppcVar2 = (*i_var3.field_0x3a + 0x8);
                                    //   (**ppcVar2)();
                                    // }
                                    BVar3 = read_file_1008_7dee(
                                        uVar13, uVar14, local_56, 0x0, param_5, 0x2, 0x1008,
                                    );
                                    if (BVar3 != 0x0) {
                                        uStack82 = 0x0;
                                        loop {
                                            if (local_56[0] <= uStack82) {
                                                return;
                                            }
                                            paStack44 = ctx.PTR_LOOP_1050_68a2;
                                            paStack50 =
                                                pass1_1000_07fc(0x1000, ctx.PTR_LOOP_1050_68a2);
                                            // uVar9 = (paStack50 >> 0x10);
                                            uVar8 = paStack50;
                                            puVar10 = (uVar9 | uVar8);
                                            if (puVar10 == 0x0) {
                                                paStack74 = 0x0;
                                            } else {
                                                paStack50.field_0x0 = 0x389a;
                                                uVar8.field_0x2 = 0x1008;
                                                uVar8.field_0x4 = 0x0;
                                                uVar8.field_0x6 = 0x0;
                                                uVar8.field_0x8 = 0x0;
                                                uVar8.field_0xa = 0x0;
                                                uVar8.field_0xc = 0x0;
                                                paStack50.field_0x0 = 0x56ce;
                                                uVar8.field_0x2 = 0x1018;
                                                paStack74 = paStack50;
                                            }
                                            BVar3 = read_file_1008_7dee(
                                                uVar13, uVar14, local_46, 0x0, param_5, 0x2, 0x1008,
                                            );
                                            if ((((BVar3 == 0x0)
                                                || (
                                                    BVar3 = read_file_1008_7dee(
                                                        uVar13, uVar14, local_42, 0x0, param_5,
                                                        0x2, 0x1008,
                                                    ),
                                                    BVar3 == 0x0,
                                                ))
                                                || (
                                                    BVar3 = read_file_1008_7dee(
                                                        uVar13, uVar14, local_3e, 0x0, param_5,
                                                        0x2, 0x1008,
                                                    ),
                                                    BVar3 == 0x0,
                                                ))
                                                || ((
                                                    BVar3 = read_file_1008_7dee(
                                                        uVar13,
                                                        uVar14,
                                                        paStack74 + 0xa,
                                                        0x0,
                                                        (paStack74 >> 0x10),
                                                        0x2,
                                                        0x1008,
                                                    ),
                                                    BVar3 == 0x0
                                                        || (
                                                            BVar3 = read_file_1008_7dee(
                                                                uVar13,
                                                                uVar14,
                                                                paStack74 + 0xc,
                                                                0x0,
                                                                (paStack74 >> 0x10),
                                                                0x2,
                                                                0x1008,
                                                            ),
                                                            BVar3 == 0x0,
                                                        ),
                                                )))
                                            {
                                                break;
                                            }
                                            // uVar12 = (paStack74 >> 0x10);
                                            iVar5 = paStack74;
                                            iVar5.field_0x4 = local_46[0];
                                            iVar5.field_0x6 = local_42[0];
                                            iVar5.field_0x8 = local_3e[0];
                                            if (i_var3.field_0x3e == 0x0) {
                                                mem_op_1000_179c(0xc, puVar10, 0x1000);
                                                paStack50 = CONCAT22(puVar10, local_3e[0]);
                                                if ((puVar10 | local_3e[0]) == 0x0) {
                                                    i_var3.field_0x3e = 0x0;
                                                } else {
                                                    set_struct_1008_574a(CONCAT22(
                                                        puVar10,
                                                        local_3e[0],
                                                    ));
                                                    &i_var3.field_0x3e = local_3e[0];
                                                    (&i_var3.field_0x3e + 0x2) = extraout_DX_00;
                                                }
                                            }
                                            ppcVar2 = (*i_var3.field_0x3e + 0x8);
                                            (**ppcVar2)();
                                            uStack82 += 0x1;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        //LAB_1030_77be:
        ctx.PTR_LOOP_1050_0310 = 0x6d2;
    }
    return;
}
