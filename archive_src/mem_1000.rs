use crate::defines::{
    Struct13_1, Struct18, Struct19, Struct79, Struct99, Struct_1000_05e2, U32Ptr,
};
use crate::pass::pass_1000::{pass1_1000_010c, pass1_1000_05b4, pass1_1000_0782, pass1_1000_09ca};
use crate::util::read_struct_from_addr;
use crate::winapi::GlobalDOSAlloc16;
use crate::{
    defines::StructA,
    fn_ptr::fn_ptr_1000::fn_ptr_op_1000_1708,
    global::AppContext,
    misc::{empty_fn_1000_214a, ret_true_1000_2146},
    pass::pass_1000::{
        pass1_1000_0368, pass1_1000_0c32, pass1_1000_15ce, pass1_1000_1a54, pass1_1000_1afe,
        pass1_1000_1e61, pass1_1000_20a2, pass1_1000_25a8, pass1_1000_2913, pass1_1000_41e0,
        pass1_1000_52be, pass1_1000_5390,
    },
    string::string_1000::str_op_1000_28dc,
    sys_api::{mixed_dos3_call_1000_3636, mixed_dos3_call_1000_39f2, _SHI_INVOKEERRORHANDLER1},
    util::{make_u16_ptr, CARRY2, CONCAT11, CONCAT22, SUB42},
    win_struct::HGLOBAL16,
    winapi::{
        FatalAppExit16, FatalExit, GLobalAlloc16, GlobalFree16, GlobalHandle16, GlobalPageLock16,
        GlobalPageUnlock16, GlobalReAlloc16, GlobalSize16, GlobalUnlock16, SegmentLimit,
        WIN16_GlobalLock16,
    },
};

pub fn mem_1000_0016(a: u32, b: u16) {
    unimplemented!()
}

pub fn mem_op_1000_0052(param_1: u32, param_2: u32) {
    unimplemented!()
}

pub fn mem_op_1000_01b0(ctx: &mut AppContext, param_1: &mut StructA, param_2: u16) -> bool {
    let ptr1: U32Ptr;
    let ptr2: U32Ptr;
    let b_var3: bool;
    let u_var4: u16;
    let u_var5: u32;
    let d_var6: u32;
    let d_var7: u32;
    let u_var8: u32;
    let u_var9: u32;
    let u_var10: u16;
    let u_stack14: u16;
    let u_stack12: u16;
    let u_stack10: i16 = 0;
    let u_stack6: u16;
    let i_stack4: i16;

    u_stack14 = 0x0;
    // if (((param_1.offset(0x40)).read() | (param_1.offset(0x3e)).read()) == 0x0) {
    if param_1.field_0x40 | param_1.field_0x3e == 0 {
        u_var5 = param_1.field_0x36;
        d_var6 = mem_op_1000_1532(ctx, param_2);
        d_var7 = d_var6;
    } else {
        d_var6 = mem_op_1000_1532(ctx, param_2);
        u_var5 = d_var6;
        if ((d_var6 >> 0x10) != 0x0) || (0xffef < u_var5) {
            pass1_1000_1e61(ctx, param_2, 0x8, Some(param_1), ctx.data_seg);
            return false;
        }
        if 0x1fff < u_var5 {
            u_var5 = 0x2000;
        }
        loop {
            u_var9 = u_var5;
            d_var7 = d_var6 + 0x18;
            if ((d_var7 >> 0x10) != 0x0) || (0xfff0 < d_var7) {
                d_var7 = 0xfff0;
            }
            b_var3 = mem_op_1000_14f2(ctx,
                param_1.field_0x16 | 0x1000,
                d_var7,
                param_1,
                ctx.data_seg,
                0,
                0,
                0,
            );
            i_stack4 = (d_var6 >> 0x10) as i16;
            u_stack6 = d_var6 as u16;
            if b_var3 {
                break;
            }
            u_var5 = u_var9 >> 0x1;
            if u_var5 < 0xc {
                u_var4 = pass1_1000_1e61(ctx, param_2, 0x2, Some(param_1), ctx.data_seg);
                if u_var4 == 0x0 {
                    return '\x01' as u32 - (param_1.field_0xa) == 0x0;
                }
                d_var6 = mem_op_1000_1532(ctx, param_2);
                u_var5 = u_var9 & 0xfffe;
            }
        }
        u_var8 = pass1_1000_5390(
            u_stack6 - 0x42,
            (i_stack4 - ((u_stack6 < 0x42) as i16)) as u16,
            0xc,
            0x0,
        );
        u_var5 = u_var8 * 0xc + param_1.field_0x42;
    }
    ptr1 = param_1.field_0x1e;
    u_var9 = *ptr1 as u32;
    *ptr1 = *ptr1 - d_var6;
    ptr2 = (param_1.field_0x20);
    *ptr2 = (*ptr2 - (d_var6 >> 0x10)) - (u_var9 < d_var6);
    if u_var5 != 0x0 {
        u_var10 = 0x0;
        u_var9 = 0xc;
        d_var7 = mem_op_1000_1532(ctx, param_2);
        u_var8 = pass1_1000_5390(
            (d_var7 - 0x42) as u16,
            (d_var7 >> 0x10) - (d_var7 < 0x42),
            u_var9 as u16,
            u_var10,
        );
        u_stack14 = u_var8 * 0xc + param_1.field_0x36;
    }
    // u_stack10 = (d_var7 >> 0x10);
    u_stack12 = d_var7 as u16;
    ptr1 = (param_1.field_0x1e);
    u_var9 = *ptr1;
    *ptr1 = *ptr1 + u_stack12;
    ptr2 = (param_1.field_0x20);
    *ptr2 = *ptr2 + u_stack10 + CARRY2(u_var9 as u16, u_stack12);
    u_var9 = (param_1 + 0xa);
    loop {
        u_var10 = u_var5 as u16;
        (u_var10 + 0x4) = u_var9 as u16;
        u_var9 = u_var10 as u32;
        u_var5 = (u_var10 + 0xc) as u32;
        if u_var10 >= u_stack14 {
            break;
        }
    }
    (param_1 + 0xa) = u_var10;
    return true;
}

pub fn mem_op_1000_0308(
    ctx: &mut AppContext,
    param_1: i16,
    struct_2: &mut StructA,
    param_3: u16,
) -> Option<Struct13_1> {
    let struct_1: &mut Struct13_1;
    let i_var2: i16;
    let b_var3: bool;
    let extraout_a_h: u8 = 0;
    let pi_var4: U32Ptr;

    if (struct_2 + 0xa) == 0x0 {
        b_var3 = mem_op_1000_01b0(ctx, struct_2, param_3);
        // if CONCAT11(extraout_a_h, b_var3) == 0x0 {
        //     return 0x0;
        // }
        if b_var3 == false {
            return None;
        }
    }
    struct_1 = &mut struct_2.field_0xa;
    (struct_2.field_0xa) = read_struct_from_addr::<Struct13_1>(struct_1.field_0x4 as u32).clone();
    pi_var4 = (param_1 * 0x2 + struct_2);
    if *pi_var4 == 0x0 {
        (struct_1.field_0x6) = struct_1.field_0x0;
        (struct_1.field_0x4) = struct_1.field_0x0;
    } else {
        i_var2 = *pi_var4;
        (struct_1.field_0x6) = i_var2 as u16;
        (struct_1.field_0x4) = (i_var2 + 0x4) as u16;
        ((i_var2 + 0x4) + 0x6) = struct_1.field_0x0 as i16;
        (i_var2 + 0x4) = struct_1.field_0x0 as i16;
    }
    *pi_var4 = struct_1;
    return Some(struct_1);
}

pub fn mem_op_1000_03c6(
    ctx: &mut AppContext,
    param_1: &mut u16,
    param_2: &mut u16,
    param_3: u16,
    param_4: &mut StructA,
    param_5: &mut u16,
    param_6: u8,
    param_7: u8,
) -> u32 {
    let pu_var1: U32Ptr;
    let pi_var2: U32Ptr;
    let u_var3: u16;
    let u_var4: u16;
    let pu_var5: U32Ptr;
    let uvar6: u16;
    let u_var7: u16;
    let b_var8: bool;
    let dvar9: u32;
    let u_stack20: u16;
    let u_var9: u16;

    u_var7 = CONCAT11(param_7, param_6);
    u_var3 = param_1 + 0xfff & 0xf000;
    pu_var1 = (param_4 + 0x1e);
    u_var4 = u_var3 + *pu_var1;
    u_var3 = param_2 + (0xf000 < param_1) + (param_4 + 0x20) + CARRY2(u_var3, *pu_var1);
    pu_var1 = (param_4 + 0x28);
    b_var8 = u_var3 < *pu_var1;
    if (b_var8)
        || (b_var8
            || u_var3 == *pu_var1
                && (
                    pu_var1 = (param_4 + 0x26),
                    u_var4 < *pu_var1 || u_var4 == *pu_var1,
                ))
    {
        if param_3 == 0x3 {
            u_stack20 = ((-((param_6 & 0x1) != 0x0) >> 0x8) & 0x1 | 0x20) << 0x8;
        } else {
            u_stack20 = 0x1000;
        }
        u_stack20 = (param_4 + 0x16) | u_stack20;
        mem_op_1000_131c(ctx, u_stack20, param_1, param_2, param_5);
        if (u_var3 | u_stack20) != 0x0 {
            pu_var5 = mem_op_1000_0308(ctx, param_3 as i16, param_4, 0) as u32;
            if pu_var5 != 0x0 {
                pu_var5[0x4] = u_stack20;
                pu_var5[0x5] = u_var3;
                u_var9 = ctx.data_seg;
                ctx.PTR_LOOP_1050_000c = param_3 | 0xcad0;
                // 0x0 = param_4;
                ctx.PTR_LOOP_1050_0002 = ctx.data_seg;
                ctx.DAT_1050_0004 = pu_var5;
                (ctx.DAT_1050_0004 + 0x2) = ctx.data_seg;
                ctx.PTR_LOOP_1050_000a = 0x0;
                dvar9 = mem_op_1000_1532(ctx, *param_5);
                uvar6 = dvar9 as u16;
                if param_3 == 0x1 {
                    u_var7 = pass1_1000_0782(param_4, uvar6, 0x0, 0);
                } else {
                    if param_3 == 0x3 {
                        pass1_1000_05b4(param_6, 0x0);
                    } else {
                        u_var7 = pass1_1000_09ca(uvar6 as i16, None);
                    }
                }
                // param_2 = (dvar9 >> 0x10);
                *pu_var5 = u_var7;
                pu_var5[0x1] = 0x8000;
                pu_var1 = (param_4 + 0x1e);
                u_var4 = *pu_var1;
                *pu_var1 = *pu_var1 + uvar6;
                pi_var2 = (param_4 + 0x20);
                *pi_var2 = *pi_var2 + param_2 + CARRY2(u_var4, uvar6);
                return u_var3 as u32;
            }
            mem_op_1000_13ce(*param_5, ctx);
        }
    } else {
        pass1_1000_1e61(ctx, *param_5, 0x7, Some(param_4), ctx.data_seg);
    }
    return 0x0;
}

pub fn mem_op_1000_0510(ctx: &mut AppContext, param_1: u16, param_2: u16, param_3: u16) -> u32 {
    let var_1: U32Ptr;
    let pi_var2: String;
    let b_var3: u8;
    let i_var4: i16;
    let u_var6: u16;
    let u_var7: u16;
    let u_var8: u16;
    let u_var9: u16;
    let u_var10: u16;
    let b_var11: bool;
    let d_var12: u32;
    let l_var13: i32;
    let u_var5: u16;

    i_var4 = param_2 as i16;
    u_var5 = (param_2 + 0x2);
    u_var6 = (param_2 + 0x4);
    b_var3 = (param_2 + 0xc) as u8;
    d_var12 = mem_op_1000_1532(ctx, param_3);
    u_var9 = (d_var12 >> 0x10) as u16;
    u_var8 = d_var12 as u16;
    if param_1 != 0x0 {
        u_var7 = (i_var4 + 0x1e) as u16;
        u_var10 = ((i_var4 + 0x20) - u_var9) - (u_var7 < u_var8);
        var_1 = (i_var4 + 0x24) as u32;
        b_var11 = u_var10 < *var_1;
        if (b_var11 || u_var10 == *var_1) && (b_var11 || (u_var7 - u_var8 < (i_var4 + 0x22) as u16))
        {
            b_var11 = false;
            u_var9 = u_var10;
            // TODO: goto LAB_1000_0595;
        }
    }
    pass1_1000_0368(u_var6, (b_var3 & 0x7) as u16, 0x0);
    var_1 = (ctx.s_version__d__d_1050_0012.field_0xc);
    u_var7 = *var_1;
    *var_1 = *var_1 - u_var8;
    pi_var2 = ctx.s_New_failed_in_Op__Op_1050_0020.clone();
    pi_var2[0] = (pi_var2[0] - u_var9) - (u_var7 < u_var8);
    b_var11 = true;
    // TODO: LAB_1000_0595:
    if b_var11 {
        (param_2 + 0xc) = 0x0;
        l_var13 = mem_op_1000_13ce(param_3, ctx);
        return CONCAT22(((l_var13 >> 0x10) as u16), 0x1);
    } else {
    }
    return (u_var9 << 0x10) as u32;
}

pub fn mem_op_1000_05e2(
    ctx: &mut AppContext,
    param_1: &mut Struct_1000_05e2,
    param_2: i16,
    param_3: u16,
    param_4: &mut StructA,
    param_5: &mut u16,
) -> u32 {
    let pu_var1: U32Ptr;
    let i_var2: i16;
    let u_var3: u16;
    let u_var4: u16;
    let u_var5: u16;
    let u_var6: u32;
    let b_var5: bool;
    let u_var6: u32;

    i_var2 = param_2 + (0xffeb < param_1);
    while ((b_var5 || u_var3 == *pu_var1)
        && (b_var5
            || (
                pu_var1 = (param_4 + 0x26),
                u_var4 < *pu_var1 || u_var4 == *pu_var1,
            )))
        && (u_var6 != 0x0
            || (
                u_var5 = pass1_1000_1e61(ctx, param_5, 0x2, param_4, ctx.data_seg),
                u_var5 != 0x0,
            ))
    {
        u_var3 = 0x3;
        u_var6._0_1_ = param_3;
        u_var6._1_1_ = (param_3 >> 0x8);
        u_var6._0_2_ = mem_op_1000_03c6(
            ctx,
            param_1 + 0x14,
            i_var2,
            0x3,
            param_4,
            param_5,
            u_var6 as u8,
            u_var6._1_1_,
        );
        if ((u_var6 | u_var3) != 0x0) {
            return CONCAT22(u_var6 as u16, u_var3 + 0x14);
        }
        u_var6 = mem_op_1000_0052(param_4, param_5);
        u_var3 = param_1 + 0x1013 & 0xf000;
        pu_var1 = (param_4 + 0x1e);
        u_var4 = u_var3 + *pu_var1;
        u_var3 = i_var2 + (0xf000 < param_1 + 0x14) + (param_4 + 0x20) + CARRY2(u_var3, *pu_var1);
        pu_var1 = (param_4 + 0x28);
        b_var5 = u_var3 < *pu_var1;
    }
    return 0x0;
}

pub unsafe fn mem_1000_0668(ctx: &mut AppContext, param_1: u16) -> u32 {
    let u_var1: u32;

    u_var1 = mem_op_1000_0510(ctx, 0x0, 0x0, param_1);
    return u_var1;
}

pub fn mem_1000_0670(
    ctx: &mut AppContext,
    param_1: u16,
    param_2: U32Ptr,
    param_3: u16,
    param_4: U32Ptr,
    param_5: i16,
    param_6: u16,
) -> u16 {
    let pu_var1: U32Ptr;
    let piVar2: U32Ptr;
    let UVar3: &mut StructA;
    let UVar4: u16;
    let iVar5: i16;
    let UVar6: &mut StructA;
    let uVar7: u32;
    let uVar8: u16;
    let uVar9: u16;
    let UVar10: u16;
    let BVar11: bool;
    let uVar12: u16;
    let uVar13: u16;
    let uVar14: u16;
    let DVar15: u32;
    let DVar16: u32;

    UVar3 = param_4;
    UVar4 = (param_4 + 0x2);
    DVar15 = mem_op_1000_1532(ctx, param_6);
    UVar6 = param_5 + (0xffeb < param_3);
    uVar7 = *param_4;
    uVar8 = -((param_1 & 0x1) != 0x0) & 0x100 | -((param_1 & 0x4) != 0x0) & 0x400 | (uVar7 + 0x16);
    if param_2 == 0x0 {
        BVar11 = mem_op_1000_14f2(
            ctx,
            uVar8 | 0x2000,
            (param_3 + 0x14) as u32,
            UVar6,
            param_4 as u16,
            ctx.data_seg,
            0,
            0,

        );
        if BVar11 == 0x0 {
            return 0x0;
        }
    } else {
        iVar5 = (param_4 + 0x1) as i16;
        uVar12 = (param_4 + 0x6) as u16;
        uVar14 = uVar12;
        loop {
            uVar13 = uVar14;
            uVar9 = uVar8 | 0x2000;
            mem_op_1000_1408(ctx, uVar9, &mut param_3 + 0x14, UVar6, param_6);
            uVar14 = uVar13 | uVar9;
            if (uVar14 != 0x0) {
                break;
            }
            UVar10 = pass1_1000_1e61(ctx, param_6, 0x2, UVar3, UVar4);
            if UVar10 == 0 {
                break;
            }
        }
        if ((uVar13 | uVar9) == 0x0) {
            (param_2 + 0x2) = 0x0;
            *param_2 = 0x0;
            return 0x0;
        }
        (iVar5 + 0x8) = uVar9 as i16;
        (iVar5 + 0xa) = uVar13 as i16;
        *param_2 = uVar9 + 0x14;
        (param_2 + 0x2) = uVar13 as u32;
    }
    DVar16 = mem_op_1000_1532(ctx, param_6);
    uVar12 = (DVar16 - DVar15) as u16;
    pu_var1 = (UVar3 + 0x1e);
    uVar8 = *pu_var1;
    *pu_var1 = *pu_var1 + uVar12;
    piVar2 = (UVar3 + 0x20);
    *piVar2 = *piVar2 + (DVar16 - DVar15 >> 0x10) + CARRY2(uVar8, uVar12);
    return 0x1;
}

pub fn mem_op_1000_0838(
    ctx: &mut AppContext,
    param_1: Option<&mut StructA>,
    param_2: u16,
) -> Struct99 {
    let pu_var1: U32Ptr;
    let pi_var2: U32Ptr;
    let i_var3: i16;
    let pu_var4: U32Ptr;
    let u_var5: u16;
    let u_var6: u16;
    let u_var7: u16;
    let u_var8: u32;
    let pi_var9: U32Ptr;
    let b_var10: bool;
    let u_stack6: u16;
    let pi_stack4: U32Ptr;

    pi_var9 = (param_1 + 0x2);
    pi_stack4 = pi_var9;
    if (param_1 + 0x2) == 0x0 {
        // TODO: goto LAB_1000_085b;
    }
    loop {
        loop {
            if *pi_var9 != 0x0 {
                i_var3 = pi_var9[0x5];
                pu_var4 = ctx.PTR_LOOP_1050_000e;
                if pu_var4 != 0x0 {
                    ctx.PTR_LOOP_1050_000e = pu_var4;
                    pi_var2 = ctx.PTR_LOOP_1050_000a;
                    *pi_var2 = *pi_var2 + 0x1;
                    (param_1 + 0x2) = pi_var9;
                    return CONCAT22(i_var3 as u16, pu_var4 as u16);
                }
                *pi_var9 = 0x0;
            }
            pi_var9 = pi_var9[0x2];
            if pi_var9 == pi_stack4 {
                break;
            }
        }
        // TODO: LAB_1000_085b:
        if (param_1 + 0x18) == 0x0 {
            pass1_1000_1e61(ctx, param_2, 0x4, param_1, ctx.data_seg);
            return 0x0;
        }
        u_var5 = (param_1 + 0x1a);
        loop {
            u_stack6 = u_var5;
            u_var5 = 0x1;
            u_var8 = mem_op_1000_03c6(ctx,&mut u_stack6, 0x0, 0x1, param_1, &mut param_2, 0x0, '\0');
            if (u_var8 | u_var5) != 0x0 {
                break;
            }
            u_var5 = (param_1 + 0x1e);
            u_var6 = u_var5 + u_stack6;
            u_var5 = (param_1 + 0x20) + CARRY2(u_var5, u_stack6);
            pu_var1 = (param_1 + 0x28);
            b_var10 = *pu_var1 <= u_var5;
            if b_var10 {
                if b_var10 && u_var5 != *pu_var1 {
                    return 0x0;
                }
                pu_var1 = (param_1 + 0x26);
                if *pu_var1 <= u_var6 && u_var6 != *pu_var1 {
                    return 0x0;
                }
            }
            u_var5 = u_stack6 >> 0x1;
            if u_stack6 >> 0x1 < (param_1 + 0x18) + 0x14 {
                u_var7 = pass1_1000_1e61(ctx, param_2, 0x2, param_1, ctx.data_seg);
                u_var5 = u_stack6 & 0xfffe;
                if u_var7 == 0x0 {
                    return 0x0;
                }
            }
        }
        pi_var9 = (param_1 + 0x2);
        pi_stack4 = pi_var9[0x2];
    }
}

pub fn mem_op_1000_0a48(
    ctx: &mut AppContext,
    param_1: u8,
    param_2: u32,
    param_3: u32,
    param_4: u32,
    param_5: U32Ptr,
    in_stack_00000005: u8,
) -> i32 {
    let u_var1: u16;
    let pu_var2: U32Ptr;
    let u_var4: u16;
    let u_var3: u16;
    let u_var4: u16;
    let u_var5: u32;
    let pu_var1: U32Ptr;

    // u_var4 = (param_4 >> 0x10);
    if (param_4 + 0x14) == -0x4153 {
        if (param_3 != 0x0) || (true && ((ctx.s_version__d__d_1050_0012 + 0x6) < param_2)) {
            if (param_3 != 0x0) || (true && ((ctx.s_version__d__d_1050_0012 + 0xa) < param_2)) {
                u_var5 = mem_op_1000_05e2(ctx, param_2, param_3, param_1 & 0xfffd, 0x0, param_5);
            } else {
                u_var5 = mem_op_1000_0b20(ctx, param_1 & 0xfffd, 0x0, param_2, param_5);
            }
        } else {
            if ((false) || (param_2 != 0x0)) {
                u_var5 = mem_op_1000_0838(ctx, 0x0, param_5 as u16);
                // u_var3 = (u_var5 >> 0x10);
                pu_var2 = u_var5;
                if ((u_var5 != 0x0) && ((param_1 & 0x1) != 0x0)) {
                    u_var1 = (ctx.s_version__d__d_1050_0012 + 0x6);
                    // TODO: refactor for loop
                    // for (u_var4 = u_var1 >> 0x1; u_var4 != 0x0; u_var4 -= 0x1) {
                    //   pu_var1 = pu_var2;
                    //   pu_var2 = pu_var2 + 0x1;
                    //   *pu_var1 = 0x0;
                    // }
                    if ((u_var1 & 0x1) != 0x0) {
                        *pu_var2 = 0x0;
                    }
                }
            } else {
                pass1_1000_1e61(ctx, param_5 as u16, 0x4, param_4, u_var4);
                u_var5 = 0x0;
            }
        }
        return u_var5 as i32;
    }
    pass1_1000_1e61(ctx, param_5 as u16, 0xa, 0x0, 0x0);
    return 0x0;
}

pub fn mem_op_1000_0b20(
    ctx: &mut AppContext,
    param_1: u16,
    param_2: &mut StructA,
    param_3: u16,
    param_4: &mut u16,
) -> u16 {
    let pu_var1: U32Ptr;
    let u_var2: u16;
    let u_var3: u16;
    let u_var4: u16;
    let u_var5: u16;
    let u_var6: u32;
    let pu_var7: U32Ptr;
    let u_var8: u16;
    let b_var9: bool;
    let u_var10: u32;
    let u_stack20: u16;
    let pu_stack6: U32Ptr;

    u_var8 = SUB42(ctx.data_seg, 0x0) as u16;
    u_var2 = param_1 & 0x2;
    u_var4 = param_3 + 0x5 & 0xfffc;
    u_var4 = u_var4 - 0x8 & !-(u_var4 < 0x8);
    u_var5 = u_var4 + 0x8;
    pu_var7 = (u_var2 * 0x2 + param_2);
    u_stack20 = param_1;
    pu_stack6 = pu_var7;
    if (pu_var7 == 0x0) {
        // TODO: goto LAB_1000_0b64;
    }
    loop {
        loop {
            if ((u_var5 <= *pu_var7)
                && (
                    u_var10 = pass1_1000_0c32(u_var5, u_stack20, 0x0),
                    u_var10 != 0x0,
                ))
            {
                (u_var2 * 0x2 + param_2) = pu_var7;
                return u_var10 as u16;
            }
            pu_var7 = pu_var7[0x2];
            if pu_var7 == pu_stack6 {
                break;
            }
        }
        //LAB_1000_0b64:
        if (((u_stack20 & 0x2) == 0x0) || ((u_stack20 & 0x40) != 0x0))
            || ((param_2 + 0x32) == 0x0)
        {
            //LAB_1000_0b9e:
            if ((u_stack20 & 0x10) != 0x0)
                || (
                    u_var3 = u_var2,
                    u_var6 = mem_op_1000_03c6(
                        ctx,
                        (param_2 + 0x1a),
                        0x0,
                        &mut u_var2,
                        param_2,
                        param_4,
                        0x0,
                        '\0',
                    ),
                    (u_var6 | u_var3) == 0x0,
                )
            {
                if (u_stack20 & 0x20) == 0x0 {
                    u_var2 = u_var4 + 0x1007 & 0xf000;
                    pu_var1 = (param_2 + 0x1e);
                    u_var4 = u_var2 + *pu_var1;
                    u_var2 = (param_2 + 0x20) + CARRY2(u_var2, *pu_var1);
                    pu_var1 = (param_2 + 0x28);
                    b_var9 = u_var2 < *pu_var1;
                    if (b_var9 || u_var2 == *pu_var1)
                        && (b_var9
                            || (
                                pu_var1 = (param_2 + 0x26),
                                u_var4 < *pu_var1 || u_var4 == *pu_var1,
                            ))
                    {
                        u_var10 = mem_op_1000_05e2(ctx, u_var5, 0x0, u_stack20, param_2, param_4);
                        return u_var10 as u16;
                    }
                }
                return 0x0;
            }
        } else {
            *param_4 = 0x1000;
            u_var3 = (param_2 + 0x32)();
            if (u_var3 < u_var5) {
                // TODO: goto LAB_1000_0b9e;
            }
            u_stack20 |= 0x40;
        }
        pu_var7 = (u_var2 * 0x2 + param_2);
        pu_stack6 = pu_var7[0x2];
    }
}

pub fn mem_op_1000_131c(
    ctx: &mut AppContext,
    param_1: u16,
    param_2: &mut u16,
    param_3: &mut u16,
    param_4: &mut u16,
) {
    let h_var1: HGLOBAL16;
    let b_var2: bool;
    let l_var3: i32;
    let u_stack10: u16;
    let u_stack8: u16;
    let i_stack6: i16;

    l_var3 = CONCAT22(u_stack8, u_stack10) as i32;
    i_stack6 = 0x1;
    if ((param_1 & 0x1000) != 0x0) && (param_3 != 0x0 || (0xfff0 < param_2)) {
        *param_2 = 0xfff0;
        *param_3 = 0x0;
    }
    if (param_1 & 0x4) != 0x0 {
        l_var3 = alloc_mem_1000_1558(ctx, param_2, param_3, param_4);
    }
    loop {
        h_var1 = GLobalAlloc16(param_4, CONCAT22(param_3, param_2));
        u_stack10 = l_var3 as u16;
        if h_var1 != 0x0 {
            break;
        }
        b_var2 = i_stack6 != 0x0;
        i_stack6 = i_stack6 + -0x1;
        *param_4 = ctx.s_tile2_bmp_1050_1538 as u16;
        if b_var2 == false {
            break;
        }
    }
    if (param_1 & 0x4) != 0x0 {
        if h_var1 != 0x0 {
            GlobalPageLock16(ctx.s_tile2_bmp_1050_1538 as u16);
        }
        pass1_1000_15ce(
            ctx,
            &mut u_stack10,
            (l_var3 >> 0x10),
            ctx.s_tile2_bmp_1050_1538,
        );
    }
    if h_var1 != 0x0 {
        WIN16_GlobalLock16(ctx.s_tile2_bmp_1050_1538 as u16);
        return;
    }
    return;
}

pub fn mem_op_1000_13ce(param_1: u16, ctx: &mut AppContext) -> i32 {
    let HVar1: HGLOBAL16;
    let u_var2: u16;
    let DVar3: u32;

    DVar3 = GlobalHandle16(param_1);
    // u_var2 = (DVar3 >> 0x10);
    if (DVar3 != 0x0) {
        HVar1 = GlobalFree16(ctx.s_tile2_bmp_1050_1538);
        return CONCAT22(u_var2, (HVar1 == 0x0));
    }
    return (u_var2 << 0x10) as i32;
}

pub fn mem_op_1000_1408(
    ctx: &mut AppContext,
    param_1: u16,
    param_2: &mut u16,
    param_3: &mut StructA,
    param_4: u16,
) {
    let h_var1: HGLOBAL16;
    let b_var2: bool;
    let d_var3: u32;
    let i_stack12: i16;
    let u_stack8: u16;

    d_var3 = GlobalHandle16(param_4);
    u_stack8 = 0x32;
    i_stack12 = 0x1;
    if (((param_1 & 0x1000) != 0x0) && (param_3 != 0x0 || (0xfff0 < param_2))) {
        *param_2 = 0xfff0;
        *param_3 = 0x0;
    }
    if ((param_1 & 0x100) != 0x0) {
        u_stack8 = 0x72;
    }
    if ((param_1 & 0x804) != 0x0) {
        u_stack8 &= 0xfffd;
    }
    if (d_var3 != 0x0) {
        if ((param_1 & 0x4) != 0x0) {
            GlobalPageUnlock16(ctx.s_tile2_bmp_1050_1538 as u16);
        }
        loop {
            h_var1 = GlobalReAlloc16(
                ctx.s_tile2_bmp_1050_1538 as u16,
                CONCAT22(param_2, u_stack8),
                param_3,
            );
            if h_var1 != 0x0 {
                break;
            }
            u_stack8 &= 0xffcf;
            b_var2 = i_stack12 != 0x0;
            i_stack12 = i_stack12 + -0x1;
            if !b_var2 {
                break;
            }
        }
        if (h_var1 != 0x0) && ((param_1 & 0x4) != 0x0) {
            GlobalPageLock16(ctx.s_tile2_bmp_1050_1538 as HGLOBAL16);
        }
        if (h_var1 != 0x0) {
            WIN16_GlobalLock16(ctx.s_tile2_bmp_1050_1538 as u16);
            return;
        }
    }
    return;
}

pub fn mem_op_1000_14f2(
    ctx: &mut AppContext,
    param_1: u16,
    param_2: u32,
    param_3: &mut StructA,
    param_4: u16,
    param_5: u16,
    param_6: u16,
    param_7: u16,
) -> bool {
    // if (((param_1 & 0x1000) != 0x0) || ((param_3 == 0x0 && (param_2 < 0xfff1)))) {
    if (param_1 & 0x1000) || (param_2 < 0xfff10000) {
        mem_op_1000_1408(ctx, param_1 & 0xfdff | 0x800, param_2, param_7, 0);
        if ((param_6 | param_5) != 0x0) {
            return true;
        }
    }
    return false;
}

pub fn mem_op_1000_1532(ctx: &mut AppContext, param_1: u16) -> u32 {
    let d_var1: u32;

    d_var1 = GlobalHandle16(param_1);
    if (d_var1 != 0x0) {
        d_var1 = GlobalSize16(ctx.s_tile2_bmp_1050_1538 as u16);
        return d_var1;
    }
    return 0x0;
}

pub fn alloc_mem_1000_1558(
    ctx: &mut AppContext,
    param_1: u16,
    param_2: u16,
    param_3: &mut U32Ptr,
) -> u32 {
    let mut u_var1: u32 = 0;
    let mut alloc_size: u32 = 0;
    let mut u_stack12: u32 = 0;
    let mut u_stack10: u16 = 0;
    let mut u_stack8: u16 = 0x8;

    if (param_2 < 0x9) && (param_2 < 0x8 || (param_1 == 0x0)) {
        while (param_2 < u_stack8) || (param_2 <= u_stack8 && (param_1 <= u_stack10)) {
            loop {
                alloc_size = CONCAT22(u_stack10, param_3);
                *param_3 = ctx.s_tile2_bmp_1050_1538;
                alloc_size = GlobalDOSAlloc16(alloc_size as usize) as u32;
                u_var1 = alloc_size;
                if u_var1 == 0x0 {
                    break;
                }
                0x0 = 0x0;
                ctx.PTR_LOOP_1050_0002 = u_stack12 as u32;
                u_stack12 = u_var1;
            }
            u_var1 = u_stack8 & 0x1;
            u_stack8 >>= 0x1;
            u_stack10 = u_stack10 >> 0x1 | (u_var1 != 0x0) << 0xf;
        }
    }
    return u_stack12;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn mem_op_1000_160a(ctx: &mut AppContext, param_1: Option<&mut Struct18>, param_2: u16) -> u16 {
    let pu_var1 = ret_true_1000_2146();
    if pu_var1 == 0x0 {
        return pu_var1;
    }
    if (ctx.PTR_LOOP_1050_5f2e | ctx.PTR_LOOP_1050_5f2c) == 0x0 {
        ctx.DAT_1050_5f30 = 0x1;
        ctx.DAT_1050_5f32 = 0x1;
        ctx.PTR_LOOP_1050_5f2c =
            mem_op_1000_18ec(ctx, ctx.DAT_1050_5f46, (param_1).unwrap(), param_2);
        if ctx.PTR_LOOP_1050_5f2c != 0x0 {
            if ctx.PTR_LOOP_1050_5f42 != 0x0 {
                pass1_1000_1a54(ctx, ctx.PTR_LOOP_1050_5f42, ctx.PTR_LOOP_1050_5f2c, param_2, 0);
            }
            ctx.PTR_LOOP_1050_5f2e = (ctx.PTR_LOOP_1050_5f2c >> 0x10);
            if ctx.DAT_1050_5f44 != 0xffff {
                pass1_1000_1afe(
                    ctx,
                    ctx.DAT_1050_5f44 as u16,
                    read_struct_from_addr::<StructA>(ctx.PTR_LOOP_1050_5f2c),
                    0x1000,
                );
            }
        }
    }
    empty_fn_1000_214a();
    return ctx.PTR_LOOP_1050_5f2c as u16;
}

pub fn mem_1000_167a(ctx: &mut AppContext, param_1: u16, param_2: u16, param_3: u16) -> u16 {
    let pu_var1: U32Ptr;
    let l_var2: i32;

    if (ctx.PTR_LOOP_1050_5f2e | ctx.PTR_LOOP_1050_5f2c) == 0x0 {
        pu_var1 = mem_op_1000_160a(ctx, param_3, param_2);
        if ((param_3 | pu_var1) == 0x0) {
            return 0x0;
        }
    }
    l_var2 = mem_op_1000_0a48(
        ctx,
        0x0,
        param_1 as u32,
        CONCAT22(ctx.PTR_LOOP_1050_5f2e as u16, ctx.PTR_LOOP_1050_5f2c),
        param_2 as u8,
        0,
        0,
    );
    return l_var2 as u16;
}

pub fn mem_op_1000_179c(ctx: &mut AppContext, param_1: u16, param_2: &mut Struct18, param_3: u16) {
    let pu_var1: &mut Struct18;
    let pu_var2: &mut Struct18;
    pu_var1 = read_struct_from_addr::<Struct18>(ctx.PTR_LOOP_1050_5f2c);
    pu_var2 = read_struct_from_addr::<Struct18>(ctx.PTR_LOOP_1050_5f2e);
    if (ctx.PTR_LOOP_1050_5f2e | ctx.PTR_LOOP_1050_5f2c) == 0x0 {
        pu_var1 = mem_op_1000_160a(ctx, Some(param_2), param_3);
        pu_var2 = param_2;
    }
    let mut var3 = 0u16;
    fn_ptr_op_1000_1708(ctx, param_1, &mut var3, 0x0, pu_var1, param_3);
    return;
}

pub fn mem_op_1000_18ec(
    ctx: &mut AppContext,
    param_1: u16,
    param_2: &mut Struct18,
    param_3: u16,
) -> u32 {
    let u_var1: u32;
    u_var1 = mem_op_1000_1902(ctx, &mut param_1, 0x0, 0x0, 0xc, param_3, param_2);
    return u_var1;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn mem_op_1000_1902(
    ctx: &mut AppContext,
    param_1: &mut u16,
    param_2: u16,
    param_3: u16,
    param_4: u16,
    param_5: u16,
    param_6: &mut u16,
) -> u32 {
    let p_uvar1: u16;
    let uvar2: u16;
    let bvar3: bool;
    let u_var3: u16;
    let uvar5: u16;
    let p_uvar6: U32Ptr;
    let dvar7: u32;
    let u_var8: u32;
    let pu_var1: U32Ptr;

    if ((param_1 & 0x8000) != 0x0) && (_SHI_INVOKEERRORHANDLER1 != -0x6f70) {
        *param_1 |= 0x1;
    }
    uvar5 = param_6;
    if true {
        loop {
            u_var3 = uvar5;
            // p_uvar1 = make_u16_ptr((*param_1 & 0xfffb | 0x1000) as u32);
            p_uvar1 = (*param_1 & 0xfffb | 0x1000);
            mem_op_1000_131c(ctx, p_uvar1, 0x100, 0x0, &mut param_5);
            uvar5 = u_var3 | p_uvar1;
            if uvar5 != 0x0 {
                break;
            };
            uvar2 = pass1_1000_1e61(ctx, param_5, 0x2, 0x0, 0x0);
            if uvar2 == 0 {
                break;
            }
        }
        if (u_var3 | p_uvar1) != 0x0 {
            p_uvar1[0x17] = &ctx.PTRPTR_1050_5f1a;
            p_uvar1[0x18] = ctx.data_seg;
            p_uvar1[0x15] = ctx.PTR_LOOP_1050_5f1e;
            p_uvar1[0x16] = ctx.PTR_LOOP_1050_5f20;
            p_uvar6 = p_uvar1 as u32;
            ctx.PTR_LOOP_1050_5f1e = p_uvar1 as u32;
            ctx.PTR_LOOP_1050_5f20 = u_var3 as u32;
            // TODO: refactor for loop
            // for (i_var4 = 0x5; i_var4 != 0x0; i_var4 += -0x1) {
            //   pu_var1 = p_uvar6;
            //   p_uvar6 = p_uvar6 + 0x1;
            //   *pu_var1 = 0x0;
            // }
            p_uvar1[0x5] = 0x0;
            p_uvar1[0x7] = 0x0;
            p_uvar1[0x6] = 0x0;
            p_uvar1[0x9] = 0x0;
            p_uvar1[0x8] = 0x0;
            p_uvar1[0xa] = 0xbead;
            p_uvar1[0xb] = param_1 & 0xfffd;
            p_uvar1[0xc] = 0x0;
            p_uvar1[0xd] = 0x2000;
            p_uvar1[0xe] = 0x800;
            dvar7 = mem_op_1000_1532(ctx, param_5);
            p_uvar1[0xf] = dvar7;
            p_uvar1[0x10] = (dvar7 >> 0x10);
            p_uvar1[0x12] = 0x0;
            p_uvar1[0x11] = 0x0;
            p_uvar1[0x13] = 0xfffe;
            p_uvar1[0x14] = 0xffff;
            p_uvar1[0x19] = 0x0;
            p_uvar1[0x1a] = 0x0;
            p_uvar1[0x20] = 0x0;
            p_uvar1[0x1f] = 0x0;
            bvar3 = pass1_1000_1afe(param_4, p_uvar1, u_var3);
            if bvar3 != 0x0 {
                if (param_3 | param_2) != 0x0 {
                    p_uvar6 = p_uvar1 as u32;
                    uvar5 = u_var3;
                    u_var8 = pass1_1000_52be(param_2, param_3, param_4, 0x0);
                    pass1_1000_010c(0x1, u_var8, (u_var8 >> 0x10), p_uvar6, uvar5, param_5);
                }
                return CONCAT22(u_var3, p_uvar1);
            }
            mem_op_1000_1b9a(ctx, 0x0, p_uvar1 as u32, u_var3, param_5);
        }
    }
    return 0x0;
}

pub unsafe fn mem_op_1000_1b68(
    ctx: &mut AppContext,
    param_1: u16,
    param_2: u16,
    param_3: u16,
    param_4: u16,
) -> u32 {
    let u_var1: u32;

    if (param_3 + 0x14) != -0x4153 {
        pass1_1000_1e61(ctx, param_2, 0xa, 0x0, 0x0);
        return (param_1 << 0x10) as u32;
    }
    u_var1 = mem_op_1000_1b9a(ctx, 0x0, param_3 as u32, param_4, param_2);
    return u_var1;
}

pub fn mem_op_1000_1b9a(
    ctx: &mut AppContext,
    param_1: u16,
    param_2: u32,
    param_3: u16,
    param_4: u16,
) -> u32 {
    let u_var1: u16;
    let u_var2: u32;
    let u_var3: u16;
    let u_var4: u16;
    let iVar5: i16;
    let lVar6: i32;
    let puStack8: U32Ptr;
    let uStack4: u16;

    (param_2 + 0x14) = 0x0;
    uStack4 = 0x0;
    loop {
        iVar5 = (uStack4 * 0x2) as i16;
        if (iVar5 != 0x0) {
            loop {
                u_var2 = (iVar5 + 0x8) as u32;
                (u_var2 + 0xc) = 0x0;
                mem_op_1000_13ce(param_4, ctx);
                iVar5 = (iVar5 + 0x4);
                if (uStack4 * 0x2) == iVar5 as u16 {
                    break;
                }
            }
        }
        uStack4 += 0x1;
        if uStack4 >= 0x5 {
            break;
        }
    }
    u_var4 = (param_2 + 0x12) as u16;
    u_var3 = (param_2 + 0x10) as u16;
    while (true) {
        puStack8 = CONCAT22(u_var4, u_var3);
        if ((u_var4 | u_var3) == 0x0) {
            break;
        }
        u_var1 = *puStack8;
        u_var4 = (u_var3 + 0x2);
        mem_op_1000_13ce(param_4, ctx);
        u_var3 = u_var1;
    }
    pass1_1000_20a2(param_2 as u16, param_3);
    lVar6 = mem_op_1000_13ce(param_4, ctx);
    return CONCAT22(((lVar6 >> 0x10) as u16), 0x1);
}

pub fn mem_op_1000_1dfa(param_1: i16, param_2: u8, param_3: u16, param_4: u16) -> bool {
    let u_var1: u32;
    let u_var2: u16;

    if ((param_2 & 0x4) == 0x0) {
        u_var2 = (((-((param_2 & 0x2) == 0x0) >> 0x8) & 0xfe) + 0x92) << 0x8;
    } else {
        u_var2 = 0x1800;
    }
    if (((param_4 == 0x0) || (false))
        || ((param_4 & 0xff00 & (((-((param_2 & 0x4) == 0x0) >> 0x8) & 0x82) + 0x18) << 0x8)
            != u_var2))
    {
        return 0x1;
    }
    if (param_1 != 0x0) {
        u_var1 = SegmentLimit(param_4) as u32;
        if (CARRY2(param_3, (param_1 - 0x1) as u16)) {
            return 0x1;
        }
        if (u_var1 < (param_3 + (param_1 - 0x1)) as u32) {
            return 0x1;
        }
    }
    return 0x0;
}

pub fn mem_op_1000_21b6(param_1: u16, param_2: u16) -> bool {
    mem_op_1000_1dfa(0x0, 0x4, param_1, param_2)
}

pub fn mem_1000_2bb6(
    ctx: &mut AppContext,
    param_1: u16,
    param_2: &String,
    param_3: i16,
    param_4: u16,
    param_5: u16,
    param_6: u16,
    param_7: u8,
    param_8: u16,
) -> u16 {
    let pi_var1: U32Ptr;
    let i_var2: i16;
    let pi_var3: U32Ptr;
    let bVar4: u8;
    let pu_var5: U32Ptr;
    let puVar6: U32Ptr;
    let puVar7: U32Ptr;
    let uStack4: u16;
    let iStack2: i16;

    pi_var3 = param_2;
    iStack2 = param_3 + 0x1;
    uStack4 = SUB42(ctx.data_seg, 0x0) as u16;
    bVar4 = (param_2 + 0x5);
    if ((bVar4 & 0x82) != 0x0) && ((bVar4 & 0x40) == 0x0) {
        param_2[0x2] = 0x0;
        if (bVar4 & 0x1) != 0x0 {
            if (bVar4 & 0x10) == 0x0 {
                // TODO: goto LAB_1000_2c37;
            }
            param_2[0] = param_2[0x3];
            bVar4 &= 0xfe;
        }
        (param_2 + 0x5) = bVar4 & 0xef | 0x2;
        puVar7 = (param_2 + 0xb);
        if ((bVar4 & 0x8) == 0x0)
            && ((bVar4 & 0x4) != 0x0
                || (((param_2 + 0x78) & 0x1) == 0x0
                    && ((ctx.PTR_LOOP_1050_61ec != 0x0
                        && ((param_2 == 0x621c || (param_2 == 0x6228))
                            && ((puVar7[0x5f90] & 0x40) != 0x0)))
                        || (
                            mem_1000_2ce8(ctx, param_2, &mut param_8, param_5),
                            ((pi_var3 + 0x5) & 0x8) == 0x0,
                        ))))
        {
            pu_var5 = mixed_dos3_call_1000_39f2(
                ctx,
                puVar7,
                CONCAT22(param_6, param_1),
                (ctx.PTR_LOOP_1050_0000 + 0x1),
                param_4,
                param_5,
                param_6,
                param_7,
            );
            puVar6 = (&ctx.PTR_LOOP_1050_0000 + 0x1);
        } else {
            i_var2 = pi_var3[0x3];
            puVar6 = (*pi_var3 - i_var2);
            *pi_var3 = i_var2 + 0x1;
            pi_var3[0x2] = pi_var3[0x79] + -0x1;
            if (puVar6 == 0x0) {
                pu_var5 = 0x0;
                if ((puVar7[0x5f90] & 0x20) != 0x0) {
                    mixed_dos3_call_1000_3636(puVar7 as u16, 0x0, 0x0, 0x2, &iStack2);
                    pu_var5 = 0x0;
                    puVar6 = pu_var5;
                }
            } else {
                pu_var5 = mixed_dos3_call_1000_39f2(
                    ctx,
                    puVar7,
                    CONCAT22(pi_var3[0x4], pi_var3[0x3]),
                    puVar6,
                    param_4,
                    param_5,
                    param_6,
                    param_7,
                );
            }
            *(pi_var3 + 0x3) = param_1;
        }
        if (pu_var5 == puVar6) {
            return param_1 & 0xff;
        }
    }
    //LAB_1000_2c37:
    pi_var1 = pi_var3 + 0x5;
    pi_var1 = pi_var1 | 0x20;
    return 0xffff;
}

pub fn mem_1000_2ce8(ctx: &mut AppContext, param_1: &mut i16, param_2: &mut u16, param_3: u16) {
    let pi_var1: U32Ptr;
    let u_var2: u16;

    u_var2 = mem_1000_167a(ctx, 0x200, param_3, param_2);
    if (param_2 == 0x0) {
        pi_var1 = param_1 + 0x5;
        pi_var1 = pi_var1 | 0x4;
        param_1[0x79] = 0x1;
        *param_2 = ctx.data_seg;
        u_var2 = param_1 + 0xf1;
    } else {
        pi_var1 = param_1 + 0x5;
        pi_var1 = pi_var1 | 0x8;
        param_1[0x79] = 0x200;
    }
    param_1[0x1] = param_2;
    *param_1 = u_var2 as i16;
    param_1[0x4] = param_2;
    param_1[0x3] = u_var2;
    param_1[0x2] = 0x0;
    return;
}

pub unsafe fn mixed_mem_op_1000_3c51(
    ctx: &mut AppContext,
    param_1: &mut HGLOBAL16,
    param_2: &mut HGLOBAL16,
    param_3: i16,
    param_4: &mut String,
    param_5: &mut u16,
    param_6: i16,
) -> u32 {
    let pi_var1: U32Ptr;
    let mut pcVar2: String;
    let mut string_1: String;
    let pi_var3: U32Ptr;
    let u_var4: u16;
    let flags: u16;
    let HVar5: HGLOBAL16;
    let piVar6: U32Ptr;
    let mut pcVar7: String;
    let DVar8: u32;
    let HVar9: HGLOBAL16;
    let iVar10: i16;
    let iVar11: i16;

    if (((param_3 + 0x2) & 0x4) == 0x0) {
        HVar9 = (param_3 + 0x6) as u16;
        flags = 0x0;
        HVar5 = param_1;
        if (param_1 == 0x0) {
            if (false) {
                // goto LAB_1000_3cb6;
            }
            flags = 0x1;
        }
        u_var4 = 0x2;
        if (true) {
            u_var4 = 0x20;
        }
        *param_5 = ctx.s_tile2_bmp_1050_1538;
        *param_1 = GlobalReAlloc16(0x1000, CONCAT22(param_1, u_var4), flags);
        if (param_1 == 0x0) {
            //LAB_1000_3cb6:
            return CONCAT22(HVar5, param_1);
        }
        if (param_1 == HVar9) {
            *param_5 = ctx.s_tile2_bmp_1050_1538;
            *param_1 = *param_2;
            DVar8 = GlobalSize16(ctx.s_tile2_bmp_1050_1538);
            if (DVar8 != 0x0) {
                HVar5 = param_1;
                if (((HVar9 + 0x2) & 0x4) != 0x0) {
                    HVar5 = param_1 - 0x1;
                    (HVar9 - 0x2) = HVar5;
                }
                //         TODO: goto LAB_1000_3cb6;
            }
        }
    }
    iVar11 = 0x12;
    iVar10 = 0x12;
    pass1_1000_25a8(ctx, param_4, param_5);
    pass1_1000_2913(ctx, iVar10, param_4, param_5);
    string_1 = str_op_1000_28dc(ctx, iVar11);
    if (string_1 != 0x0) {
        iVar10 = 0x9;
        if (*string_1 == 'M') {
            iVar10 = 0xf;
        }
        string_1 = string_1 + iVar10;
        iVar10 = 0x22;
        pcVar7 = string_1;
        loop {
            if (iVar10 == 0x0) {
                break;
            }
            iVar10 += -0x1;
            pcVar2 = pcVar7;
            pcVar7 = pcVar7 + 0x1;
            if *pcVar2 == '\r' {
                break;
            }
        }
        pcVar7[-0x1] = '\0';
    }
    FatalAppExit16(param_5, string_1);
    FatalExit();
    piVar6 = ctx.PTR_LOOP_1050_63fe;
    loop {
        pi_var1 = piVar6;
        piVar6 = piVar6 + 0x1;
        iVar10 = *pi_var1;
        pi_var3 = piVar6;
        if (iVar10 == param_6) || (pi_var3 = (iVar10 + 0x1) as u32, pi_var3 == 0x0) {
            return CONCAT22(param_6 as u16, pi_var3 as u16);
        }
        iVar10 = -0x1;
        loop {
            if (iVar10 == 0x0) {
                break;
            }
            iVar10 += -0x1;
            pi_var1 = piVar6;
            piVar6 = (piVar6 + 0x1);
            if *pi_var1 == '\0' {
                break;
            }
        }
    }
}

pub fn free_mem_1000_407a(ctx: &mut AppContext, param_1: u16, param_2: u16, param_3: u16) {
    let u_var1: u16;
    let handle: HGLOBAL16;

    handle = 0x1000;
    if false {
        u_var1 = pass1_1000_41e0(param_2 as i16);
        if u_var1 == 0x0 {
            return;
        }
        handle = ctx.s_tile2_bmp_1050_1538;
        GlobalUnlock16(0x1000);
    }
    GlobalFree16(handle);
    return;
}
