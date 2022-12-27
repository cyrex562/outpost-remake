use crate::unk::block_1028_0000::{pass1_1028_0138, pass1_1028_081e};
use crate::unk::block_1028_1000::{pass1_1028_11de, pass1_1028_121e};

pub fn pass1_1028_0374(mut param_1: i16, param_2: *mut u8, param_3: *mut astruct_373, param_4: *mut HFILE16)

{
    let mut ppcVar1: *mut *mut code;
    let mut BVar2: bool;
    let mut uVar3: u16;
    let mut uVar4: u16;
    let mut uVar5: u16;
    let mut local_18: [u16; 0x2] = [0; 0x2];
    let mut paStack20: *mut astruct_99;
    let mut local_10: [u16; 0x2] = [0; 0x2];
    let mut local_c: [u16; 0x3] = [0; 0x3];
    let mut uStack6: u16;
    let mut local_4: u16;
    let mut uVar2: *mut Struct728;

    file_1028_b81a(param_1, param_2, param_3, param_4);
    if (param_1 != 0) {
        BVar2 = read_file_1008_7dee(param_4, (param_3 & 0xffff0000 | (param_3 + 0x20)), 0x2);
        if (BVar2 != 0) {
            BVar2 = read_file_1008_7dee(param_4, CONCAT22(0x1050, &local_4), 0x2);
            if (BVar2 != 0) {
                uStack6 = 0;
                loop {
                    if (local_4 <= uStack6) {
                        return;
                    }
                    paStack20 = pass1_1000_07fc(_PTR_LOOP_1050_68a2);
                    uVar4 = (paStack20 >> 0x10);
                    uVar2 = paStack20;
                    if ((uVar4 | uVar2) == 0) {
                        paStack20 = null_mut();
                    } else {
                        paStack20.field0_0x0 = 0x389a;
                        uVar2.field2_0x2 = 0x1008;
                        uVar2.field3_0x4 = 0;
                        uVar2.field4_0x6 = 0;
                        uVar2.field5_0x8 = 0;
                        uVar2.field6_0xa = 0;
                        uVar2.field7_0xc = 0;
                        paStack20.field0_0x0 = 0x56ce;
                        uVar2.field2_0x2 = 0x1018;
                    }
                    BVar2 = read_file_1008_7dee(param_4, CONCAT22(0x1050, local_10), 0x2);
                    if (BVar2 == 0) { break; }
                    BVar2 = read_file_1008_7dee(param_4, CONCAT22(0x1050, local_c), 0x2);
                    if (BVar2 == 0) { break; }
                    BVar2 = read_file_1008_7dee(param_4, CONCAT22(0x1050, local_18), 0x2);
                    if (BVar2 == 0) { break; }
                    BVar2 = read_file_1008_7dee(param_4, (paStack20 & 0xffff0000 | (paStack20 + 0xa)), 0x2);
                    if (BVar2 == 0) { break; }
                    BVar2 = read_file_1008_7dee(param_4, (paStack20 & 0xffff0000 | (paStack20 + 0xc)), 0x2);
                    if (BVar2 == 0) { break; }
                    (paStack20 + 0x4) = local_10[0];
                    uVar3 = switch_1008_72bc(param_4, local_c[0]);
                    uVar5 = (paStack20 >> 0x10);
                    (paStack20 + 0x6) = uVar3;
                    (paStack20 + 0x8) = local_18[0];
                    ppcVar1 = ((param_3 + 0x22) + 0x8);
                    (**ppcVar1)();
                    uStack6 += 0x1;
                }
            }
        }
        u16_1050_0310 = 0x6d2;
    }
    return;
}


pub fn pass1_1028_04ee(mut param_1: u32, param_2: *mut u32) -> u16

{
    let mut piVar1: *mut i16;
    let mut uVar2: u16;
    let mut uVar3: u16;
    let mut uVar4: u16;
    let mut lVar5: i32;
    let mut local_a: [u8; 0x8] = [0; 0x8];

    *param_2 = 0;
    pass1_1008_5784(CONCAT22(0x1050, local_a), (param_1 + 0x22));
    loop {
        lVar5 = pass1_1008_5b12(CONCAT22(0x1050, local_a));
        if (lVar5 == 0) {
            return 0x0;
        }
        uVar2 = (lVar5 + 0xc);
        uVar4 = (param_2 >> 0x10);
        uVar3 = param_2;
        param_2 = param_2 + uVar2;
        piVar1 = (param_2 + 2);
        *piVar1 = *piVar1 + CARRY2(uVar3, uVar2);
        if !(((param_2 + 0x2) == 0) && (param_2 < 0x1e)) { break; }
    }
    return 0x1;
}

pub fn pass1_1028_0582(param_1: u32, mut param_2: u16, param_3: *mut astruct_15)

{
    let mut ppuVar1: *mut *mut u32 = null_mut();
    let mut plVar2: *mut i32;
    let mut uVar3: u32;
    let mut ppcVar4: *mut *mut code;
    let mut puVar5: *mut u8;
    let mut uVar6: u16;
    let mut uVar7: u16;
    let mut uVar8: u32;
    let mut extraout_DX: u16;
    let mut uVar9: u16;
    let mut extraout_DX_00: u16;
    let mut extraout_DX_01: u16;
    let mut iVar10: i16;
    let mut iVar11: i16;
    let mut uVar12: u16;
    let mut uVar13: u16;
    let mut uVar14: u16;
    let mut unaff_CS: u16;
    let mut local_138: [u8; 0x10e] = [0; 0x10e];
    let mut local_2a: u32;
    let mut paStack38: *mut astruct_99;
    let mut paStack34: *mut astruct_99;
    let mut uStack30: u32;
    let mut uStack18: u32;
    let mut uStack14: u32;
    let mut local_a: [u8; 0x4] = [0; 0x4];
    let mut uStack6: u32;

    uVar12 = (param_3 >> 0x10);
    iVar10 = param_3;
    uVar8 = (iVar10 + 0x14);
    uVar13 = (uVar8 >> 0x10);
    iVar11 = uVar8;
    uStack6 = uVar8 & 0xffff0000 | (iVar11 + 0xa4);
    if (((iVar11 + 0xa6) != 0) && ((iVar11 + 0xac) != 0)) {
        pass1_1028_081e(param_1, param_2, param_3);
        param_1 = (iVar10 + 0x20);
        ppuVar1 = (uStack6 + 0x8);
        if (*ppuVar1 < param_1 || *ppuVar1 == param_1) {
            puVar5 = local_a;
            ppcVar4 = (param_3 + 0x40);
            (**ppcVar4)();
            uVar8 = ZEXT24(puVar5);
            param_2 = extraout_DX;
            if (puVar5.is_null()) {
                if ((uStack6 + 0x2) == 0xc) {
                    uStack14 = pass1_1028_b4f2(param_3);
                    param_2 = (uStack14 >> 0x10);
                    uVar8 = (uStack14 + 0x1f6);
                    plVar2 = (uVar8 + 0x170);
                    *plVar2 = *plVar2 + 1;
                    uStack18 = uVar8;
                } else {
                    uStack18 = _PTR_LOOP_1050_68a2;
                    paStack38 = pass1_1000_07fc(_PTR_LOOP_1050_68a2);
                    uVar9 = (paStack38 >> 0x10);
                    uVar6 = paStack38;
                    if ((uVar9 | uVar6) == 0) {
                        paStack34 = null_mut();
                    } else {
                        paStack38.field0_0x0 = 0x389a;
                        (uVar6 + 0x2) = 0x1008;
                        (uVar6 + 0x4) = 0;
                        (uVar6 + 0x6) = 0;
                        (uVar6 + 0x8) = 0;
                        (uVar6 + 0xa) = 0;
                        (uVar6 + 0xc) = 0;
                        paStack38.field0_0x0 = 0x56ce;
                        (uVar6 + 0x2) = 0x1018;
                        paStack34 = paStack38;
                    }
                    uVar13 = (uStack6 >> 0x10);
                    iVar11 = uStack6;
                    uVar14 = (paStack34 >> 0x10);
                    (paStack34 + 0x6) = (iVar11 + 2);
                    (paStack34 + 0xa) = (iVar11 + 0x6);
                    unaff_CS = 0x1020;
                    uVar7 = switch_1020_c3b4((iVar11 + 0x2));
                    uVar6 = uVar7 * (uStack6 + 0x6);
                    uVar8 = uVar6;
                    (paStack34 + 0xc) = uVar6;
                    uVar3 = (iVar10 + 0x22);
                    ppcVar4 = ((iVar10 + 0x22) + 0x4);
                    (**ppcVar4)(0x1020, uVar3, (uVar3 >> 0x10));
                    param_2 = extraout_DX_00;
                }
            }
            param_1 = uVar8;
            (iVar10 + 0x20) = 0;
        }
    }
    uVar13 = (uStack6 >> 0x10);
    if (((uStack6 + 0x4) != 0) && ((uStack6 + 0x8) != 0)) {
        pass1_1028_081e(param_1, param_2, param_3);
        param_1 = (iVar10 + 0x20);
        ppuVar1 = (uStack6 + 0x8);
        if (*ppuVar1 < param_1 || *ppuVar1 == param_1) {
            param_1 = &local_2a;
            ppcVar4 = (param_3 + 0x40);
            (**ppcVar4)(unaff_CS, param_3);
            if (param_1.is_null()) {
                uStack18 = _PTR_LOOP_1050_68a2;
                paStack38 = pass1_1000_07fc(_PTR_LOOP_1050_68a2);
                uVar9 = (paStack38 >> 0x10);
                uVar6 = paStack38;
                if ((uVar9 | uVar6) == 0) {
                    paStack34 = null_mut();
                } else {
                    paStack38.field0_0x0 = 0x389a;
                    (uVar6 + 0x2) = 0x1008;
                    (uVar6 + 0x4) = 0;
                    (uVar6 + 0x6) = 0;
                    (uVar6 + 0x8) = 0;
                    (uVar6 + 0xa) = 0;
                    (uVar6 + 0xc) = 0;
                    paStack38.field0_0x0 = 0x56ce;
                    (uVar6 + 0x2) = 0x1018;
                    paStack34 = paStack38;
                }
                uVar13 = (uStack6 >> 0x10);
                iVar11 = uStack6;
                uVar14 = (paStack34 >> 0x10);
                (paStack34 + 0x8) = (iVar11 + 0x4);
                (paStack34 + 0xa) = (iVar11 + 0x6);
                uVar7 = pass1_1020_c42e((iVar11 + 0x4));
                param_1 = (uVar7 * (uStack6 + 0x6));
                (paStack34 + 0xc) = param_1;
                uVar3 = (iVar10 + 0x22);
                ppcVar4 = ((iVar10 + 0x22) + 0x4);
                (**ppcVar4)(0x1020, uVar3, (uVar3 >> 0x10));
            }
            (iVar10 + 0x20) = 0;
        }
    }
    if ((iVar10 + 0xc) != 0xe) {
        pass1_1028_b58e((param_3 & 0xffff | uVar12 << 0x10));
        local_2a = CONCAT22(extraout_DX_01, param_1);
        paStack34 = (param_1 + 0x2e);
        uStack30 = (paStack34 + 0x4);
        pass1_1028_68de(CONCAT22(0x1050, local_138), 0x1, uStack30);
        fn_ptr_1030_835a(_u16_1050_5748, CONCAT22(0x1050, local_138));
    }
    return;
}

pub fn pass1_1028_08c6(param_1: *mut StructD, param_2: u8) -> *mut StructD

{
    pass1_1028_0138(&param_1.address_offset_field_0x0);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub fn FUN_1028_0b8e()

{
    return;
}

pub fn FUN_1028_0b92()

{
    return;
}

pub fn pass1_1028_0b96(param_1: *mut StructD, param_2: u8) -> *mut StructD

{
    pass1_1028_b418(&param_1.address_offset_field_0x0);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1028_0c84(mut param_1: i16, param_2: *mut astruct_15, mut param_3: u32)

{
    let mut ppcVar1: *mut *mut code;
    let mut uVar2: u16;
    let mut puVar3: *mut u32;
    let mut extraout_DX: u16;
    let mut uVar4: u16;
    let mut uVar5: u32;
    let mut puVar6: *mut u32;
    let mut puVar7: *mut u32;
    let mut bStack55: u8;
    let mut local_32: [u8; 0xa] = [0; 0xa];
    let mut uStack40: u32;
    let mut uStack36: u32;
    let mut puStack28: *mut u32;
    let mut local_1a: u32;
    let mut iStack22: i16;
    let mut uStack20: u16;
    let mut iStack18: i16;
    let mut uStack16: u16;
    let mut iStack14: i16;
    let mut local_c: u32;
    let mut iStack8: i16;
    let mut paStack6: *mut astruct_292;

    pass1_1028_b58e(param_2);
    paStack6 = CONCAT22(extraout_DX, param_1);
    local_c = (param_1 + 0xc);
    iStack18 = (param_1 + 0x10);
    puStack28 = &local_c;
    uStack16 = extraout_DX;
    iStack14 = iStack18;
    iStack8 = iStack18;
    pass1_1028_bab6(iStack18, extraout_DX, param_2);
    uVar2 = pass1_1030_2fac(CONCAT22(uStack16, iStack18));
    local_1a = local_c;
    iStack22 = iStack8;
    uStack36 = CONCAT22(uStack36, &local_1a);
    iStack14 += 0x1;
    uStack20 = uVar2;
    if (iStack14 <= uVar2) {
        puVar7 = CONCAT22(0x1050, local_32);
        iStack22 = iStack14;
        uVar5 = pass1_1028_bb24(param_2);
        uVar4 = (uVar5 >> 0x10);
        puVar3 = &local_1a;
        pass1_1030_64ce(puVar3, uVar4, _PTR_LOOP_1050_5740, CONCAT22(0x1050, puVar3),
                        uVar5 & 0xffff | uVar4 << 0x10, puVar7);
        uStack40 = *puVar3;
        uVar4 = (puVar3 + 2);
        bStack55 = (uStack40 >> 0x18);
        uVar2 = bStack55;
        uStack36 = uStack40;
        if (bStack55 != 0) {
            pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uStack40 & 0xffff | uVar4 << 0x10);
            puVar6 = struct_op_1030_73a8(CONCAT22(uVar4, uVar2), uVar2, uVar4);
            uVar2 = puVar6;
            ppcVar1 = (*puVar6 + 0x58);
            (**ppcVar1)();
        }
    }
    pass1_1028_b46e(uVar2, param_2, param_3);
    fn_ptr_1030_7296(paStack6);
    return;
}

pub fn pass1_1028_0d9c(mut param_1: i16, param_2: *mut astruct_15)

{
    let mut ppcVar1: *mut *mut code;
    let mut puVar2: *mut u32;
    let mut uVar3: u16;
    let mut uVar4: u16;
    let mut BVar5: bool;
    let mut extraout_DX: u16;
    let mut uVar6: u16;
    let mut uVar7: u32;
    let mut puVar8: *mut u32;
    let mut uStack58: u32;
    let mut local_32: [u8; 0x6] = [0; 0x6];
    let mut puStack44: *mut u32;
    let mut uStack40: u32;
    let mut uStack36: u32;
    let mut puStack28: *mut u32;
    let mut local_1a: u32;
    let mut iStack22: i16;
    let mut uStack20: u16;
    let mut iStack18: i16;
    let mut uStack16: u16;
    let mut iStack14: i16;
    let mut local_c: u32;
    let mut iStack8: i16;
    let mut iStack6: i16;
    let mut uStack4: u16;

    pass1_1028_b514(param_2);
    pass1_1028_b58e(param_2);
    local_c = (param_1 + 0xc);
    iStack18 = (param_1 + 0x10);
    puStack28 = &local_c;
    uStack16 = extraout_DX;
    iStack14 = iStack18;
    iStack8 = iStack18;
    iStack6 = param_1;
    uStack4 = extraout_DX;
    pass1_1028_bab6(iStack18, extraout_DX, param_2);
    uStack20 = pass1_1030_2fac(CONCAT22(uStack16, iStack18));
    local_1a = local_c;
    uStack36 = CONCAT22(uStack36, &local_1a);
    iStack22 = iStack14 + 1;
    if (iStack22 <= uStack20) {
        puVar8 = CONCAT22(0x1050, local_32);
        iStack14 = iStack22;
        uVar7 = pass1_1028_bb24(param_2);
        uVar6 = (uVar7 >> 0x10);
        puVar2 = &local_1a;
        pass1_1030_64ce(puVar2, uVar6, _PTR_LOOP_1050_5740, CONCAT22(0x1050, puVar2),
                        uVar7 & 0xffff | uVar6 << 0x10, puVar8);
        uStack40 = *puVar2;
        uVar6 = (puVar2 + 2);
        uStack58._3_1_ = (uStack40 >> 0x18);
        uVar3 = uStack58._3_1_;
        if (uStack58._3_1_ != 0) {
            uStack36 = uStack40;
            pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uStack40 & 0xffff | uVar6 << 0x10);
            uStack58 = CONCAT22(uVar6, uVar3);
            uVar4 = pass1_1030_6fa0(CONCAT22(uVar6, uVar3));
            BVar5 = pass1_1008_c6ae(_u16_1050_06e0, uVar4, 0x13);
            if (BVar5 != 0) {
                puStack44 = struct_op_1030_73a8(uStack58, BVar5, uVar6);
                ppcVar1 = (*puStack44 + 0x24);
                (**ppcVar1)();
            }
        }
    }
    return;
}

pub fn pass1_1028_0ea6(mut param_1: u16, param_2: *mut astruct_15)

{
    let mut puVar1: *mut u16;
    let mut BVar2: bool;
    let mut uVar3: u16;
    let mut iVar3: *mut astruct_15;
    let mut uVar4: *mut astruct_15;

    uVar4 = (param_2 >> 0x10);
    iVar3 = param_2;
    if (iVar3.field10_0xc != 0x10) {
        BVar2 = pass1_1008_c6ae(_u16_1050_06e0, iVar3.field10_0xc, 0x13);
        if (BVar2 == 0) {
            BVar2 = pass1_1008_c6ae(_u16_1050_06e0, iVar3.field10_0xc, 0x2);
            if (((BVar2 != 0) && (iVar3.field15_0x12 != 0x7)) && (iVar3.field15_0x12 != 0x4)) {
                uVar3 = pass1_1028_1556(BVar2, param_1, (param_2 & 0xffff | ZEXT24(uVar4) << 0x10));
//        if (uVar3 == 0) goto LAB_1028_0f0a;
                if (iVar3.field15_0x12 == 0x9) {
                    iVar3.field15_0x12 = 0x5;
                }
            }
        } else if (iVar3.field25_0x22 < 1) {
            if ((iVar3.field15_0x12 != 0x5) && (iVar3.field15_0x12 != 0x6)) {
                return;
            }
            fn_ptr_1000_17ce(iVar3.field16_0x14);
            iVar3.field16_0x14 = null_mut();//
// LAB_1028_0f0a:
            iVar3.field15_0x12 = 0x9;
            return;
        }
        pass1_1028_be2a(param_2);
        if (iVar3.field15_0x12 == 0x5) {
            puVar1 = &iVar3.field25_0x22;
            *puVar1 = *puVar1 - 0x1;
            pass1_1028_b58e((param_2 & 0xffff | ZEXT24(uVar4) << 0x10));
        }
    }
    return;
}

pub fn pass1_1028_0fa4(mut param_1: u16, mut param_2: u16, param_3: *mut astruct_15)

{
    let mut BVar1: bool;
    let mut uVar3: u16;
    let mut in_register_0000000a: u16;
    let mut paVar4: *mut Struct57;
    let mut iVar2: *mut astruct_15;
    let mut uVar2: *mut astruct_15;
    let mut puVar5: *mut u32;
    let mut in_stack_0000fe9c: u16;
    let mut in_stack_0000ffc0: u16;
    let mut in_stack_0000ffc6: u16;
    let mut in_stack_0000ffca: u16;
    let mut uVar6: u16;
    let mut uVar7: u16;
    let mut iVar8: i16;

    paVar4 = CONCAT22(in_register_0000000a, param_1);
    pass1_1028_be9e(param_3);
    puVar5 = mixed_1010_20ba(paVar4, _u16_1050_0ed0, CONCAT22(param_2, 0x2), in_stack_0000fe9c, in_stack_0000ffc0, in_stack_0000ffc6, in_stack_0000ffca);
    paVar4 = (paVar4 & 0xffff0000 | puVar5 >> 0x10);
    iVar8 = (puVar5 + 0x82);
    uVar2 = (param_3 >> 0x10);
    iVar2 = param_3;
    if (iVar2.field15_0x12 == 0x5) {
        BVar1 = pass1_1008_c6ae(_u16_1050_06e0, iVar2.field10_0xc, 0x2);
        if ((BVar1 != 0) && ((PTR_LOOP_1050_4fbc.is_null() || (iVar8 != 0)))) {
            PTR_LOOP_1050_4fbc = (&PTR_LOOP_1050_0000 + 1);
            uVar7 = 0;
            iVar8 = 0x4;
            uVar6 = 0x1;
            uVar3 = pass1_1028_b58e(param_3);
            pass1_1030_7c50(uVar3, paVar4, CONCAT22(paVar4, uVar3), CONCAT22(uVar7, uVar6), iVar8);
        }
        iVar2.field25_0x22 = 0x64;
    }
    return;
}


pub fn pass1_1028_12be(param_1: *mut astruct_15, param_2: *mut u32) -> u16 {
    let mut piVar1: *mut i16;
    let mut uVar2: u16;
    let mut ppcVar3: *mut *mut code;
    let mut bVar4: bool;
    let mut extraout_AH: u8;
    let mut uVar5: u16;
    let mut paVar6: *mut astruct_15;
    let mut uVar7: u32;
    let mut uVar8: u32;
    let mut uStack8: u16;

    bVar4 = pass1_1028_11de(param_1);
    if (CONCAT11(extraout_AH, bVar4) == 0) {
        paVar6 = pass1_1028_121e(0x1050, param_1);
        ppcVar3 = (paVar6 + 0x40);
        uVar5 = (**ppcVar3)();
        return uVar5;
    }
    *param_2 = 0;
    uVar7 = pass1_1028_b58e(param_1);
    uStack8 = 0x4;
    uVar8 = uVar7;
    loop {
        uVar8 = pass1_1030_7c28(uVar8, (uVar8 >> 0x10), uVar7, uStack8);
        uVar2 = param_2;
        param_2 = param_2 + uVar8;
        piVar1 = (param_2 + 2);
        *piVar1 = *piVar1 + (uVar8 >> 0x10) + CARRY2(uVar2, uVar8);
        uStack8 += 0x1;
        if uStack8 >= 0xe {
            break;
        }
    }
    if (0x1f4 < *param_2) {
        return 0x1;
    }
    return 0x0;
}

pub fn pass1_1028_134a(param_1: *mut astruct_15) {
    let mut piVar1: *mut i16;
    let mut ppcVar2: *mut *mut code;
    let mut BVar3: bool;
    let mut plVar4: *mut i32;
    let mut uVar5: u16;
    let mut paVar6: *mut Struct57;
    let mut uVar7: u16;
    let mut paVar8: *mut astruct_15;
    let mut lStack26: i32;
    let mut iStack22: i16;
    let mut uStack18: u32;
    let mut uStack10: u32;
    let mut local_6: i32;

    uVar7 = (param_1 >> 0x10);
    BVar3 = pass1_1008_c6ae(_u16_1050_06e0, (param_1 + 0xc), 0x13);
    if (BVar3 != 0) {
        plVar4 = &local_6;
        ppcVar2 = (param_1 + 0x40);
        (**ppcVar2)(0x1008, param_1, plVar4, 0x1050);
        if (plVar4.is_null() == false) {
            piVar1 = (param_1 + 0x22);
            *piVar1 = *piVar1 + 1;
            return;
        }
        uStack10 = 0x1f4 - local_6;
        paVar8 = pass1_1028_121e(0x1050, param_1);
        uVar5 = (paVar8 >> 0x10);
        uVar7 = SUB42(paVar8, 0x0);
        pass1_1028_b58e(paVar8);
        uStack18 = CONCAT22(uVar5, uVar7);
        for iStack22 in 0..0xa {
            uStack10 = (iStack22 * 0x2 + 0x4fbe);
            paVar6 = (uStack10 >> 0x10);
            if (uStack10 < uStack10) {
                paVar6 = (uStack10 >> 0x10);
            }
            lStack26 = CONCAT22(paVar6, uStack10);
            pass1_1030_7ddc(
                uStack10,
                paVar6,
                uStack18,
                CONCAT13((paVar6 >> 0x8), CONCAT12(paVar6, uStack10)),
                iStack22 + 0x4,
            );
            uStack10 -= lStack26;
            if (uStack10 < 1) {
                return;
            }
        }
    }
    return;
}


pub fn write_to_file_1028_1452(param_1: *mut astruct_731, param_2: *mut u8) -> u16 {
    let mut BVar1: bool;
    let mut uVar2: u16;
    let mut in_stack_0000ffda: HFILE16;
    let mut local_c: [u16; 3] = [0; 3];
    let mut local_6: [*mut u8; 2] = [null_mut(); 2];

    BVar1 = write_to_file_1028_b5ec(param_1, param_2);
    if (BVar1 != 0) {
        uVar2 = (param_1 >> 0x10);
        local_c[0] = (param_1 + 0x22);
        BVar1 = write_to_file_1008_7e1c(param_2, CONCAT22(0x1050, local_c), 0x2, in_stack_0000ffda);
        if (BVar1 != 0) {
            local_6[0] = (param_1 + 0x20);
            BVar1 = write_to_file_1008_7e1c(param_2, CONCAT22(0x1050, local_6), 0x2, in_stack_0000ffda);
            if (BVar1 != 0) {
                local_6[0] = PTR_LOOP_1050_4fbc;
                BVar1 = write_to_file_1008_7e1c(
                    param_2,
                    CONCAT22(0x1050, local_6),
                    0x2,
                    in_stack_0000ffda,
                );
                if (BVar1 != 0) {
                    return 0x1;
                }
            }
        }
        u16_1050_0310 = 0x6d0;
    }
    return 0x0;
}


pub fn pass1_1028_14d8(
    mut param_1: i16,
    param_2: *mut u8,
    param_3: *mut astruct_373,
    param_4: *mut HFILE16,
) {
    let mut BVar1: bool;
    let mut local_4: u16;

    file_1028_b81a(param_1, param_2, param_3, param_4);
    if (param_1 != 0) {
        BVar1 = read_file_1008_7dee(param_4, (param_3 & 0xffff0000 | (param_3 + 0x22)), 0x2);
        if ((BVar1 != 0) && (
            BVar1 = read_file_1008_7dee(param_4, CONCAT22(0x1050, &local_4), 0x2),
            BVar1 != 0,
        )) {
            (param_3 + 0x20) = local_4;
            if (u16_1050_0312 < 0x2) {
                return;
            }
            BVar1 = read_file_1008_7dee(param_4, &PTR_LOOP_1050_4fbc, 0x2);
            if (BVar1 != 0) {
                return;
            }
        }
        u16_1050_0310 = 0x6d2;
    }
    return;
}


pub fn pass1_1028_16fe(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    pass1_1028_b418(&param_1.address_offset_field_0x0);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1028_17d8(mut param_1: u16, param_2: *mut astruct_15) {
    let mut extraout_DX: u16;

    pass1_1030_df0c(param_1, param_2);
    pass1_1028_b58e(param_2);
    pass1_1038_57dc((param_1 + 0x2e), 0x1, 0x3);
    return;
}

pub fn pass1_1028_1812(param_1: u32) {
    pass1_1028_bdac(param_1, 0x2);
    return;
}

pub fn pass1_1028_1824(
    mut param_1: i16,
    mut param_2: u16,
    mut param_3: u32,
    param_4: *mut u32,
    mut param_5: u32,
    mut param_6: u32,
) {
    let mut BVar1: bool;
    let mut puVar2: *mut u32;
    let mut uVar3: u16;
    let mut uVar4: u16;
    let mut in_register_0000000a: u16;
    let mut paVar5: *mut Struct57;
    let mut unaff_SI: u16;
    let mut uVar6: u16;
    let mut uVar7: u32;
    let mut in_stack_0000fe6c: u16;
    let mut in_stack_0000ff90: u16;
    let mut in_stack_0000ff96: u16;
    let mut in_stack_0000ff9a: u16;
    let mut uVar8: u16;
    let mut uVar9: u16;
    let mut local_2a: u32;
    let mut iStack38: i16;
    let mut iStack36: i16;
    let mut uStack34: u16;
    let mut uStack32: u16;
    let mut uStack30: u16;
    let mut uStack28: u16;
    let mut iStack24: i16;
    let mut puStack22: *mut u32;
    let mut uStack16: u16;
    let mut uStack14: u16;
    let mut local_c: u32;
    let mut iStack8: i16;
    let mut uStack6: u32;

    uVar8 = param_3;
    uVar9 = (param_3 >> 0x10);
    pass1_1028_c3aa(uVar8, uVar9, param_4, param_5, param_6);
    if (param_1 == 0) {
        return;
    }
    BVar1 = pass1_1028_c314(
        param_1,
        param_2,
        uVar8,
        uVar9,
        param_4,
        param_5,
        (param_5 >> 0x10),
        param_6,
    );
    if (BVar1 == 0) {
        return;
    }
    puVar2 = &local_c;
    pass1_1030_64ce(
        puVar2,
        param_2,
        _PTR_LOOP_1050_5740,
        param_4,
        param_6,
        CONCAT22(0x1050, puVar2),
    );
    uStack6 = *puVar2;
    uStack30 = (puVar2 + 2);
    paVar5 = CONCAT22(in_register_0000000a, uStack30);
    uVar6 = (param_4 >> 0x10);
    iStack8 = (param_4 + 0x4);
    puStack22 = (uStack6 & 0xffff | uStack30 << 0x10);
    uStack32 = uStack6;
    uVar3 = uStack30 >> 0x8;
    if ((uStack30 >> 0x8) != '\0') {
        pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uStack6 & 0xffff | uStack30 << 0x10);
        uStack30 = paVar5;
        uStack32 = uVar3;
        uStack28 = pass1_1030_6fa0(CONCAT22(uStack30, uVar3));
        if (uStack28 == 0x10) {
            if (iStack8 != 0) {
                PTR_LOOP_1050_50ca = 0x6ab;
                return;
            }
            return;
        }
        if ((uStack28 == 0x60) || (uStack28 == 0x61)) {
            puStack22 = mixed_1010_20ba(
                paVar5,
                _u16_1050_0ed0,
                CONCAT22(unaff_SI, 0x2f),
                in_stack_0000fe6c,
                in_stack_0000ff90,
                in_stack_0000ff96,
                in_stack_0000ff9a,
            );
            uVar7 = pass1_1018_04b8(puStack22);
            uStack34 = (uVar7 >> 0x10);
            uStack16 = uVar7;
            iStack36 = (puStack22 + 0x1e);
            iStack24 = iStack36;
            uStack14 = uStack34;
            pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uVar7);
            uVar4 = pass1_1030_2fac(CONCAT22(uStack34, iStack36));
            if (uVar4 <= iStack24) {
                PTR_LOOP_1050_50ca = 0x6ac;
                return;
            }
            local_2a = *param_4;
            iStack38 = iStack8 + 1;
            puVar2 = &local_2a;
            pass1_1028_c7b6(uVar6, uVar8, uVar9, CONCAT22(0x1050, puVar2), param_6);
            if (puVar2.is_null()) {
                return;
            }
            if (puVar2 == (&u32_1050_0004 + 0x2)) {
                return;
            }
            return;
        }
    }
    PTR_LOOP_1050_50ca = 0x6aa;
    return;
}

pub fn pass1_1028_199a(mut param_1: i16, param_2: *mut astruct_15) {
    let mut piVar1: *mut i16;
    let mut uVar2: u32;
    let mut in_EDX: u32;
    let mut paVar3: *mut Struct57;
    let mut unaff_SI: u16;
    let mut in_stack_0000fd42: u16;
    let mut in_stack_0000fe66: u16;
    let mut in_stack_0000fe6c: u16;
    let mut in_stack_0000fe70: u16;
    let mut piVar4: *mut i16;
    let mut uVar5: u16;
    let mut puVar6: *mut u16;
    let mut uVar7: u16;
    let mut local_15e: u16;
    let mut uStack348: u16;
    let mut puStack50: *mut u32;
    let mut uStack42: u32;
    let mut uStack38: u16;
    let mut piStack36: *mut i16;
    let mut local_22: i16;
    let mut local_20: u16;
    let mut uStack30: u32;
    let mut puStack26: *mut u32;
    let mut local_16: i16;
    let mut local_14: u32;
    let mut local_10: u32;
    let mut uStack12: u16;
    let mut uStack10: u32;
    let mut iStack6: i16;
    let mut uStack4: u16;

    piVar1 = (param_2 + 0x14);
    *piVar1 = *piVar1 - 0x1;
    if (*piVar1 == 0) {
        pass1_1028_b58e(param_2);
        uStack4 = in_EDX;
        uStack10 = (param_1 + 0x2e);
        iStack6 = param_1;
        pass1_1038_5804(uStack10, 0x1, 0x3);
        paVar3 = (in_EDX & 0xffff0000 | uStack4);
        local_10 = (iStack6 + 0xc);
        uStack12 = (iStack6 + 0x10);
        puStack50 = &local_10;
        pass1_1008_3eb4(
            CONCAT22(0x1050, &local_10),
            CONCAT22(0x1050, &local_16),
            CONCAT22(0x1050, &local_14),
            CONCAT22(0x1050, &local_14 + 0x2),
        );
        puStack26 = mixed_1010_20ba(
            paVar3,
            _u16_1050_0ed0,
            CONCAT22(unaff_SI, 0x2f),
            in_stack_0000fd42,
            in_stack_0000fe66,
            in_stack_0000fe6c,
            in_stack_0000fe70,
        );
        uVar2 = (puStack26 + 0x20);
        puVar6 = &local_20;
        uVar7 = SUB42(0x1050, 0x0);
        piStack36 = &local_22;
        uVar5 = SUB42(0x1050, 0x0);
        piVar4 = piStack36;
        uStack30 = uVar2;
        pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uVar2);
        uStack38 = uVar2;
        pass1_1030_5b1c(
            uVar2 & 0xffff | ZEXT24(piStack36) << 0x10,
            CONCAT22(uVar5, piVar4),
            CONCAT22(uVar7, puVar6),
        );
        if (local_22 < local_16 + 1) {
            pass1_1030_5b3e(CONCAT22(piStack36, uStack38), local_16 + 0x1, local_20);
            pass1_1030_5b1c(
                CONCAT22(piStack36, uStack38),
                CONCAT22(0x1050, &local_22),
                CONCAT22(0x1050, &local_20),
            );
        }
        uVar5 = (uStack10 >> 0x10);
        uStack42 = (uStack10 + 0x4);
        struct_op_1028_87f0(
            CONCAT22(0x1050, &local_15e),
            0x0,
            0x0,
            (-(local_16 == 0) & 0xffd3) + 0x60,
            &local_10,
            0x1050,
            uStack42 & 0xffff | (uStack10 + 0x6) << 0x10,
            uStack30,
        );
        fn_ptr_1030_835a(_u16_1050_5748, CONCAT22(0x1050, &local_15e));
        local_15e = 0x389a;
        uStack348 = 0x1008;
        pass1_1008_3e76(
            CONCAT22(0x1050, &local_10),
            local_16 + 0x1,
            local_14,
            (local_14 >> 0x10),
        );
        struct_op_1028_87f0(
            CONCAT22(0x1050, &local_15e),
            0x0,
            0x0,
            0x60,
            &local_10,
            0x1050,
            uStack42,
            uStack30,
        );
        fn_ptr_1030_835a(_u16_1050_5748, CONCAT22(0x1050, &local_15e));
    }
    return;
}
