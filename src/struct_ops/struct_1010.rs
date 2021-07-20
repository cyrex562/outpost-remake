use crate::defines::{Struct79, Struct648, Struct19};
use crate::mixed::mixed_1010_20ba;
use crate::pass::pass_1008::{pass1_1008_3e94, pass1_1008_612e};
use crate::util::{CONCAT22, CONCAT13, CONCAT12, struct_from_addr, CARRY2};
use crate::pass::pass_1000::pass1_1000_4906;
use crate::global::AppContext;
use crate::mem_1000::mem_op_1000_179c;
use crate::struct_ops::struct_1008::set_struct_1008_574a;
use crate::pass::pass_1028::{pass1_1028_dc52, pass1_1028_e4ec, pass1_1028_b58e};
use crate::fn_ptr::util::get_fn_ptr_1;
use crate::pass::pass_1018::pass1_1018_dcf6;
use crate::struct_ops::struct_1030::{struct_op_1030_73a8, struct_op_1030_1cd8};
use crate::pass::pass_1010::{pass1_1010_d710, pass1_1010_d24a, pass1_1010_cf36, pass1_1010_c864, pass1_1010_c7e2, pass1_1010_37d4};
use crate::string::string_1008::str_op_1008_60e8;
use crate::fn_ptr::fn_ptr_1000::fn_ptr_1000_17ce;
use crate::win_struct::WNDCLASS16;

pub unsafe fn struct_1010_02e0(
    ctx: &mut AppContext,
    param_1: &mut Struct19,
    param_3: u16,
    extraout_dx: u16) {
    let u_var1: u16;
    let pu_var2: U32Ptr;
    // let extraout_dx: u16;
    // let paVar3: &mut Struct79;

    set_struct_fields_1010_1d48(param_1, param_3);
    // pu_var2 = (paVar3 >> 0x10);
    u_var1 = 0x0;
    // (param_1 + 0x1) = 0x0;
    param_1.field_0x0 = 0;
    // param_1[0x1].field_0x4 = 0x0;
    param_1.field_0x8 = 0;
    // (param_1[0x1].field_0x4 + 0x2) = 0x0;
    param_1.field_0xa = 0;
    // param_1[0x2].field_0x4 = 0x0;
    param_1.field_0xc = 0;
    param_1.field_0x0 = 0xe98;
    param_1.field_0x2 = 0x1010;
    mem_op_1000_179c(ctx, 0xc, param_1, 0x1000);
    // if ((pu_var2 | u_var1) == 0x0) {
    //   (param_1 + 0x1) = 0x0;
    // }
    // else {
    //   set_struct_1008_574a(param_1);
    //   // ((param_1 + 0x1)).field_0x0 = u_var1;
    //   // TODO:
    //   // param_1.field_0x2.field_0x0 = u_var1;
    //   // param_1[0x1].field_0x2 = extraout_dx;
    //   param_1.field_0x4 = extraout_dx;
    // }
    return;
}


pub unsafe fn struct_1010_0f9c(
    ctx: &mut AppContext,
    param_1: U32Ptr,
    param_2: u16,
    param_3: u16,
    extraout_dx: u32,
    extraout_DX_00: u16,
    extraout_DX_01: u16,
    extraout_DX_02: u16,
    extraout_DX_03: u16,
    extraout_DX_04: u16) {
    let fn_ptr_1: u32;
    let u_var2: u16;
    let pu_var3: U32Ptr;
    let pu_var4: U32Ptr;
    let u_var5: u32;
    // let extraout_DX: U32Ptr;
    let pu_var6: U32Ptr;
    // let pu_var7: Struct79;
    // let extraout_DX_00: U32Ptr;
    // let extraout_DX_01: u16;
    // let extraout_DX_02: u16;
    // let extraout_DX_03: u16;
    // let extraout_DX_04: U32Ptr;
    // let iVar8: &mut Struct232;
    let i_var9: &mut Struct231;
    let i_var10: &mut Struct230;
    let i_var11: &mut Struct233;
    // let uVar8: u16;
    let u_var9: u16;
    let pu_var10: u32;
    let u_var11: u32;
    let pu_var12: u32;
    let u_var13: u8;
    let u_stack36: u32;
    let i_stack32: i16;
    let u_stack30: u16;
    let pu_stack28: U32Ptr;
    let u_stack24: u32;
    let mut local_14: [u8; 12] = [0; 12];

    // uVar8 = (param_1 >> 0x10);
    // iVar8 = param_1;
    fn_ptr_1 = (*param_1 + 0x38);
    (**fn_ptr_1)();
    iVar8.field_0x68 = param_2;
    if (&iVar8.field_0x60 != 0x0) && (iVar8.field_0x68 == 0x1) {
        return;
    }
    if iVar8.field_0x68 == 0x0 {
        return;
    }
    let mut pu_var7 = struct_from_addr::<Struct79>(extraout_dx);
    pass1_1028_dc52(CONCAT22(param_3, local_14), 0x1, 0x0, 0x700);
    u_var2 = iVar8.field_0x68 * 0x18;
    mem_op_1000_179c(ctx, u_var2, &mut pu_var7, 0x1000);
    iVar8.field_0x60 = u_var2;
    iVar8.field_0x62 = pu_var7;
    pu_stack28 = CONCAT22(pu_var7, iVar8.field_0x60);
    u_stack30 = iVar8.field_0x68;
    loop {
        loop {
            pu_var6 = pu_var7;
            pu_var3 = local_14;
            pass1_1028_e4ec(CONCAT13((param_3 >> 0x8), CONCAT12(param_3, pu_var3)));
            u_stack24 = CONCAT22(pu_var6, pu_var3);
            pu_var7 = (pu_var6 | pu_var3);
            if pu_var7 == 0x0 {
                // goto
                // LAB_1010_10ca;
            }
            i_var9 = *param_1;
            fn_ptr_1 = &i_var9.field_0x40;
            pu_var4 = pu_var3;
            func = get_fn_ptr_1(fn_ptr_1);
            // (**fn_ptr_1)();
            func();
            pu_var7 = extraout_DX_00;
            if pu_var4 != 0 {
                break;
            }
        }
        u_var13 = SUB21(pu_var6, 0x0);
        pass1_1028_b58e(CONCAT13((pu_var6 >> 0x8), CONCAT12(u_var13, pu_var3)));
        u_stack36 = CONCAT22(extraout_DX_01, pu_var4);
        fn_ptr_1 = &i_var9.field_0x2c;
        pu_var12 = param_1;
        (**fn_ptr_1)();
        // u_var9 = (pu_stack28 >> 0x10);
        i_var10 = pu_stack28;
        *pu_stack28 = pu_var4;
        i_var10.field_0x2 = extraout_DX_02;
        fn_ptr_1 = &i_var9.field_0x30;
        pu_var10 = param_1;
        u_var11 = u_stack24;
        (**fn_ptr_1)();
        i_var10.field_0x8 = pu_var4;
        i_var10.field_0xa = extraout_DX_03;
        i_var10.field_0xc = u_stack36;
        fn_ptr_1 = &i_var9.field_0x3c;
        u_var5 = u_stack36;
        (**fn_ptr_1)(&USHORT_1050_1028, param_1, u_stack24, pu_var10, u_var11, pu_var12, pu_var3,
                     u_var13);
        i_var10.field_0x10 = u_var5;
        i_var10.field_0x12 = extraout_DX_04;
        i_var10.field_0x14 = u_stack36;
        pu_stack28 = (pu_stack28 & 0xffff0000) | (i_var10 + 0x1);
        u_stack30 -= 0x1;
        pu_var7 = extraout_DX_04;
        if u_stack30 == 0 {
            break;
        }
    }
//LAB_1010_10ca:
    u_var2 = iVar8.field_0x68 << 0x2;
    mem_op_1000_179c(u_var2, pu_var7, 0x1000);
    iVar8.field_0x64 = u_var2;
    iVar8.field_0x66 = pu_var7;
    i_stack32 = 0x0;
    u_stack30 = 0x0;
    loop {
        if ((iVar8.field_0x68 * 0x3) <= u_stack30) { break; }
        pu_var7 = iVar8.field_0x62;
        u_var5 = &iVar8.field_0x64;
        // u_var9 = (u_var5 >> 0x10);
        i_var11 = u_var5;
        (i_var11 + i_stack32 * 0x4) = iVar8.field_0x60 + u_stack30 * 0x8;
        (i_var11 + i_stack32 * 0x4 + 0x2) = pu_var7;
        u_stack30 += 0x3;
        i_stack32 += 0x1;
    }
    return;
}


pub fn set_struct_fields_1010_1d48(param_1: &mut Struct19, param_2: u16)

{
    param_1.field_0x0 = 0x389a;
    param_1.field_0x2 = 0x1008;
    param_1.field_0x4 = 0x0;
    param_1.field_0x8 = param_2;
    param_1.field_0x0 = 0x2014;
    param_1.field_0x2 = 0x1010;
}


pub unsafe fn struct_1010_2cd2(param_1: &mut Struct19, param_3: u16, param_4: u16, unaff_di: i16, extraout_dx: u16) {
    // let unaff_DI: i16;
    let paVar1: &mut Struct79;
    let puVar2: U32Ptr;
    let piVar3: U32Ptr;
    let piVar4: U32Ptr;
    let uVar5: u16;
    let local_6: i16;
    let local_4: i16;

    set_struct_fields_1010_1d48(param_1, param_3);
    &param_1[0x1].field_0x8 = 0x0;
    param_1[0x2].field_0x2 = 0x0;
    &param_1[0x2].field_0x4 = 0x0;
    &param_1[0x3].field_0x4 = 0x0;
    (&param_1[0x3].field_0x4 + 0x2) = 0x0;
    param_1[0x3].field_0x8 = 0x0;
    (param_1 + 0x4).field_0x0 = 0x0;
    &param_1[0x8].field_0x2 = 0x0;
    (&param_1[0x8].field_0x4 + 0x2) = 0x0;
    (param_1 + 0x9).field_0x0 = 0x0;
    &param_1[0x9].field_0x4 = 0x0;
    param_1[0x9].field_0x2 = 0x0;
    param_1.field_0x0 = 0x36da;
    param_1.field_0x2 = 0x1010;
    let pi_var4 = &local_4;
    let pi_var3 = &local_6;
    let u_var5 = param_4;
    let pu_var2 = mixed_1010_20ba(
        ctx, ctx.PTR__LOOP_1050_0ed0, 0x48, param_4,
        param_1, unaff_di, extraout_dx);
    pass1_1008_3e94((pu_var2 & 0xffff0000 | (pu_var2 + 0xe)),
                    CONCAT22(param_4, pi_var3), CONCAT22(u_var5, pi_var4));
    param_1[0x1].field_0x4 = 0x19001db;
    (param_1 + 0x1).field_0x0 = 0x140 - (&param_1[0x1].field_0x4 / 0x2 - local_4);
    param_1[0x1].field_0x2 = 0xf0 - ((&param_1[0x1].field_0x4 + 0x2) / 0x2 - local_6);
    (&param_1[0x2].field_0x4 + 0x2) = 0xa006e;
    (param_1 + 0x3) = 0xa012c;
    pass1_1000_4906(CONCAT22(param_2, &param_1[0x4].field_0x2),
                    0x0, 0x28);
    return;
}


pub fn struct_1010_383a(param_1: U32Ptr) {
    let iVar1: &mut Struct223;
    let uVar1: u16;

    // uVar1 = (param_1 >> 0x10);
    iVar1 = param_1;
    *param_1 = 0x389a;
    iVar1.field_0x2 = 0x1008;
    iVar1.field_0x4 = 0x0;
    iVar1.field_0x8 = 0x0;
    iVar1.field_0xc = 0x0;
    iVar1.field_0x10 = 0x0;
    iVar1.field_0x12 = 0x0;
    iVar1.field_0x14 = 0x0;
    *param_1 = 0x3b5e;
    iVar1.field_0x2 = 0x1010;
    return;
}


pub fn struct_1010_38f8(param_1: u32, param_2: i16, param_3: u16, param_4: U32Ptr) -> u16

{
    let uVar1: u16;
    let iVar2: &mut Struct240;
    let uVar2: u16;
    let puVar3: U32Ptr;

    if (param_2 != 0x0) {
        uVar1 = param_2 << 0x2;
        mem_op_1000_179c(uVar1, param_4, 0x1000);
        // uVar2 = (param_1 >> 0x10);
        iVar2 = param_1;
        iVar2.field_0x8 = uVar1;
        iVar2.field_0xa = param_4;
        return CONCAT22(param_4, iVar2.field_0x8);
    }
    mem_op_1000_179c(0x1a, param_4, 0x1000);
    if ((param_4 | param_3) != 0x0) {
        puVar3 = pass1_1010_37d4(CONCAT22(param_4, param_3));
        return puVar3;
    }
    return 0x0;
}


pub fn struct_1010_3b7a(
    param_1: &mut Struct19,
    param_3: u16) {
    set_struct_fields_1010_1d48(param_1, param_3);
    param_1.field_0xa = 0x389a;
    param_1.field_0xc = 0x1008;
    param_1.field_0xa = 0x3aa8;
    param_1.field_0xc = 0x1008;
    param_1.field_0xe = 0x0;
    param_1.field_0x12 = 0x0;
    param_1.field_0x14 = 0x0;
    param_1.field_0x16 = 0x0;
    param_1.field_0x0 = 0x3d6a;
    param_1.field_0x2 = 0x1010;
    param_1.field_0xa = 0x3d7a;
    param_1.field_0xc = 0x1010;
    return;
}


pub fn struct_1010_3c52(ctx: &mut AppContext, param_1: u32, param_2: u16, param_3: u16) {
    let puVar1: u32;
    let uVar2: u16;
    let ppcVar3: u32;
    let iVar4: &mut Struct274;
    let uVar4: u16;
    let paVar5: &mut Struct43;

    // uVar4 = (param_1 >> 0x10);
    iVar4 = param_1;
    iVar4.field_0x14 = param_2;
    puVar1 = iVar4.field_0xe;
    uVar2 = iVar4.field_0x10;
    if ((uVar2 | puVar1) != 0x0) {
        ppcVar3 = *puVar1;
        (**ppcVar3)();
    }
    paVar5 = unk_io_op_1010_830a(ctx.PTR__LOOP_1050_14cc, iVar4.field_0x14, param_3);
    iVar4.field_0xe = paVar5;
    iVar4.field_0x10 = (paVar5 >> 0x10);
    return;
}


pub unsafe fn struct_1010_4d5c(
    ctx: &mut AppContext,
    param_1: u32,
    param_2: u16,
    param_3: u16,
    param_4: u16,
    param_5: u16,
    param_6: i16,
    param_7: u16)

{
    let uVar1: u32;
    let uVar2: u16;
    // let iVar3: &mut Struct245;
    let iVar4: i16;
    // let uVar5: u16;

    // uVar5 = (param_1 >> 0x10);
    // iVar3 = param_1;
    if &iVar3.field_0x1a == 0x0 {
        uVar2 = iVar3.field_0x30 << 0x3;
        mem_op_1000_179c(ctx, uVar2, param_7, 0x1000);
        &iVar3.field_0x1a = uVar2;
        iVar3.field_0x1c = param_7;
    }
    uVar1 = &iVar3.field_0x1a;
    iVar4 = param_6 * 0x8;
    (uVar1 + iVar4) = param_5;
    uVar1 = &iVar3.field_0x1a;
    (uVar1 + iVar4 + 0x2) = param_4;
    uVar1 = &iVar3.field_0x1a;
    (uVar1 + iVar4 + 0x4) = param_3;
    uVar1 = &iVar3.field_0x1a;
    (uVar1 + iVar4 + 0x6) = param_2;
    return;
}


pub fn struct_1010_50b2(param_1: &mut Struct646, param_2: u16, param_3: u16) {
    set_struct_fields_1010_1d48(CONCAT22(param_2, param_1), param_3);
    param_1.field_0xa = 0x0;
    param_1.field_0xc = 0x0;
    param_1.field_0x10 = 0x0;
    param_1.field_0x12 = 0x0;
    param_1.field_0x16 = 0x0;
    CONCAT22(param_2, param_1) = 0x53f4;
    param_1.field_0x2 = 0x1010;
    return;
}


pub fn struct_1010_5f1e(param_1: &mut Struct73, param_2: &mut String, param_3: u16) {
    let uVar1: u16;
    let iVar3: &mut Struct73;
    let uVar3: &mut Struct73;

    // uVar3 = (param_1 >> 0x10);
    iVar3 = param_1;
    fn_ptr_1000_17ce(ctx, &iVar3.field_0x16, 0x1000);
    uVar1 = str_op_1008_60e8(param_2, param_3);
    iVar3.field_0x16 = uVar1;
    iVar3.field_0x18 = param_3;
    return;
}


pub fn struct_1010_6326(param_1: &mut Struct630, param_2: &mut Struct19, param_3: u16) {
    set_struct_fields_1010_1d48(CONCAT22(param_2, param_1), param_3);
    param_1.field_0xa = 0x0;
    param_1.field_0xe = 0x0;
    param_1.field_0x12 = 0x0;
    param_1.field_0x16 = 0x0;
    param_1.field_0x1a = 0x0;
    param_1.field_0x1e = 0x0;
    param_1.field_0x22 = 0x0;
    CONCAT22(param_2, param_1) = 0x66f0;
    param_1.field_0x2 = 0x1010;
    return;
}


pub fn struct_1010_9172(param_1: u32) {
    let puVar1: u32;
    let uVar2: u16;
    let ppcVar3: u32;
    let iVar4: &mut Struct249;
    let uVar4: u16;
    let paVar5: &mut Struct75;
    let uVar6: u32;

    // uVar4 = (param_1 >> 0x10);
    iVar4 = param_1;
    puVar1 = iVar4.field_0x4;
    uVar2 = iVar4.field_0x6;
    paVar5 = CONCAT22(uVar2, puVar1);
    if ((uVar2 | puVar1) != 0x0) {
        ppcVar3 = *puVar1;
        paVar5 = (**ppcVar3)();
    }
    mem_op_1000_179c(0x18, (paVar5 >> 0x10), 0x1000);
    if (paVar5 == 0x0) {
        uVar6 = 0x0;
    } else {
        uVar6 = struct_op_1030_1cd8(paVar5, 0x5, 0x5);
    }
    iVar4.field_0x4 = uVar6;
    iVar4.field_0x6 = (uVar6 >> 0x10);
    return;
}


pub fn struct_1010_95aa(param_1: &mut Struct629, param_2: &mut Struct19, param_3: u16) {
    set_struct_fields_1010_1d48(CONCAT22(param_2, param_1), param_3);
    param_1.field_0xa = 0x0;
    param_1.field_0xe = 0x0;
    param_1.field_0x12 = 0x0;
    param_1.field_0x16 = 0x0;
    param_1.field_0x18 = 0x0;
    param_1.field_0x1a = 0x0;
    param_1.field_0x1c = 0xa;
    param_1.field_0x1e = 0x0;
    CONCAT22(param_2, param_1) = 0xa1c8;
    param_1.field_0x2 = 0x1010;
    return;
}


pub fn struct_1010_a1d8(param_1: &mut Struct627, param_2: &mut Struct19, param_3: u16, param_4: &mut WNDCLASS16) {
    let iVar1: i16;
    let ppcVar2: u32;
    let unaff_DI: i16;
    let paVar3: &mut Struct79;
    let puVar4: U32Ptr;
    let uStack4: u16;

    paVar3 = set_struct_fields_1010_1d48(CONCAT22(param_2, param_1), param_3);
    param_1.field_0xa = 0x389a;
    param_1.field_0xc = 0x1008;
    param_1.field_0xa = 0x3aa8;
    param_1.field_0xc = 0x1008;
    param_1.field_0x138 = 0x0;
    CONCAT22(param_2, param_1) = 0xe9cc;
    param_1.field_0x2 = 0x1010;
    param_1.field_0xa = 0xe9dc;
    param_1.field_0xc = 0x1010;
    puVar4 = mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0, 0x2f, param_4,
                             (paVar3 >> 0x10), unaff_DI);
    &param_1.field_0x138 = puVar4;
    (&param_1.field_0x138 + 0x2) = (puVar4 >> 0x10);
    ppcVar2 = (*param_1.field_0x138 + 0x4);
    (**ppcVar2)();
    pass1_1000_4906(CONCAT22(param_2, &param_1.field_0xa4), 0x0,
                    0x94);
    pass1_1000_4906(CONCAT22(param_2, &param_1.field_0xe), 0x0,
                    0x96);
    uStack4 = 0x0;
    loop {
        iVar1 = &param_1.field_0x0 + uStack4 * 0x6;
        *(iVar1 + 0xe) = pass1_1010_c7e2;
        (iVar1 + 0x12) = 0x0;
        uStack4 += 0x1;
        while uStack4 >= 0x19 {
            break;
        }
    }
    param_1.field_0x4a = pass1_1010_c864;
    param_1.field_0x4e = 0x0;
    param_1.field_0x50 = pass1_1010_cc56;
    param_1.field_0x54 = 0x0;
    param_1.field_0x56 = pass1_1010_cf36;
    param_1.field_0x5a = 0x0;
    param_1.field_0x2c = pass1_1010_d24a;
    param_1.field_0x30 = 0x0;
    param_1.field_0x6e = pass1_1010_d448;
    param_1.field_0x72 = 0x0;
    param_1.field_0x74 = pass1_1010_d5ae;
    param_1.field_0x78 = 0x0;
    param_1.field_0x98 = pass1_1010_d710;
    param_1.field_0x9c = 0x0;
    return;
}


pub fn struct_1010_dd5e(param_1: u16, param_2: u16, param_3: u32) {
    let i_var1: i16;
    let i_var2: i16;
    let u_var3: u16 = 0;
    let u_var4: u32;
    let mut pl_stack16: long;

    if param_3 != 0x0 {
        u_var4 = struct_op_1030_73a8(param_3);
        // u_var3 = (u_var4 >> 0x10);
        i_var2 = u_var4;
        pl_stack16 = (u_var4 & 0xffff0000 | (i_var2 + 0x14));
        if ((u_var3 | i_var2 + 0x14) != 0x0) {
            i_var1 = (i_var2 + 0x12);
            i_var2 = (i_var2 + 0x18);
            if (((((i_var1 == 0x4) || ((((i_var1 == 0x6 && (i_var2 == 0x4)) || (i_var1 == 0x5)) || ((i_var1 == 0x6 && (i_var2 == 0x5)))))) || (i_var1 == 0x8)) || ((i_var1 == 0x6 && (i_var2 == 0x8)))) && (*pl_stack16 != 0x0)) {
                return;
            }
        }
    }
    return;
}


pub fn struct_1010_e9e4(param_1: &mut Struct261, param_2: &mut Struct19, param_3: u16) {
    let puVar1: U32Ptr;
    let uVar2: u16;
    let iVar3: i16;
    let uVar4: u16;
    let uVar5: u16;
    let uVar6: u32;
    let puVar7: U32Ptr;
    let iVar8: i16;
    let paVar9: &mut Struct79;
    let puVar10: U32Ptr;
    let iStack4: i16;

    paVar9 = set_struct_fields_1010_1d48(CONCAT22(param_2, param_1), param_3);
    // puVar7 = (paVar9 >> 0x10);
    param_1.field_0xa = 0x389a;
    param_1.field_0xc = 0x1008;
    param_1.field_0xa = 0x3aa8;
    param_1.field_0xc = 0x1008;
    uVar5 = 0x0;
    &param_1.field_0xe = 0x0;
    param_1.field_0x12 = 0x0;
    param_1.field_0x16 = 0x0;
    param_1.field_0x1a = 0x0;
    param_1.field_0x1e = 0x0;
    param_1.field_0x20 = 0x0;
    param_1.field_0x24 = 0x0;
    param_1.field_0x28 = 0x0;
    param_1.field_0x2c = 0x0;
    param_1.field_0x30 = 0x0;
    param_1.field_0x32 = 0x0;
    CONCAT22(param_2, param_1) = 0x558;
    param_1.field_0x2 = 0x1018;
    param_1.field_0xa = 0x568;
    param_1.field_0xc = 0x1018;
    mem_op_1000_179c(0x4, puVar7, 0x1000);
    if ((puVar7 | uVar5) == 0x0) {
        &param_1.field_0xe = 0x0;
    } else {
        puVar10 = pass1_1018_dcf6(CONCAT22(puVar7, uVar5));
        param_1.field_0xe = puVar10;
        param_1.field_0x10 = (puVar10 >> 0x10);
    }
    pass1_1000_4906(CONCAT22(param_2, &param_1.field_0x34), 0x0,
                    0x24);
    param_1.field_0x38 = 0xfa;
    param_1.field_0x3c = 0x15e;
    uVar6 = 0x1c2;
    param_1.field_0x40 = 0x1c2;
    param_1.field_0x44 = 0x1c2;
    param_1.field_0x46 = 0x2260000;
    param_1.field_0x4a = 0x28a0000;
    param_1.field_0x4e = 0x730000;
    param_1.field_0x52 = 0x960000;
    param_1.field_0x56 = 0x0;
    // for (iStack4 = 0x1; iStack4 < 0x9; iStack4 += 0x1) {
    //   pass1_1008_612e(0x0,0x1d,uVar6);
    //   uVar5 = uVar6;
    //   pass1_1008_612e(0x1,0x2,uVar5);
    //   if ((uVar6 & 0x1) != 0x0) {
    //     uVar5 = -uVar5;
    //   }
    //   iVar8 = iStack4 * 0x4;
    //   puVar1 = (&param_1.field_0x34 + iVar8);
    //   uVar2 = *puVar1;
    //   uVar4 = uVar5 + *puVar1;
    //   uVar6 = uVar4;
    //   iVar3 = (&param_1.field_0x34 + iVar8 + 0x2);
    //   (&param_1.field_0x34 + iVar8) = uVar4;
    //   (&param_1.field_0x36 + iVar8) =
    //        (uVar5 >> 0xf) + iVar3 + CARRY2(uVar5,uVar2);
    // }
    return;
}
