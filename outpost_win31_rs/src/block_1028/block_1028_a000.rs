pub unsafe fn pass1_1028_a0fa(param_1: *mut astruct_334, param_2: *mut u8, mut param_3: u32) {
    let mut puVar1: *mut u32;
    let mut puVar2: *mut u32;
    let mut puVar3: *mut u32;
    let mut iVar4: i16;
    let mut uVar5: u16;
    let mut in_register_0000000a: u16;
    let mut paVar6: *mut Struct57;
    let mut puVar7: *mut u32;
    let mut uVar8: u16;
    let mut puStack10: *mut u16;

    paVar6 = CONCAT22(in_register_0000000a, param_2);
    mem_op_1000_179c(0x108, paVar6);
    uVar5 = paVar6;
    puStack10 = CONCAT22(uVar5, param_1);
    if ((uVar5 | param_1) != 0) {
        *puStack10 = 0x389a;
        param_1.field2_0x2 = 0x1008;
        uVar8 = (param_3 >> 0x10);
        param_1.field3_0x4 = (param_3 + 0x4);
        puVar3 = (param_3 + 0x8);
        puVar7 = &param_1.field4_0x8;
        for iVar4 in 0x40..0 {
            puVar2 = puVar7;
            puVar7 = puVar7 + 1;
            puVar1 = puVar3;
            puVar3 = puVar3 + 1;
            *puVar2 = *puVar1;
        }
        *puStack10 = 0x6ad2;
        param_1.field2_0x2 = 0x1028;
        *puStack10 = 0xa6f6;
        param_1.field2_0x2 = 0x1028;
    }
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn pass1_1028_a188(
    mut param_1: u16,
    mut param_2: u16,
    mut param_3: i16,
    mut param_4: i16,
    mut param_5: u32,
) {
    let mut uVar1: u32;
    let mut lVar2: i32;
    let mut uVar3: u32;
    let mut uVar4: u16;
    let mut uVar5: u16;
    let mut uVar6: u16;
    let mut uVar7: u32;
    let mut lVar8: i32;
    let mut lVar9: i32;
    let mut uVar10: u16;
    let mut paVar11: *mut Struct57;
    let mut iVar12: i16;
    let mut iVar13: i16;
    let mut unaff_SI: u16;
    let mut uVar14: u16;
    let mut paVar15: *mut astruct_67;
    let mut paVar16: *mut Struct27;
    let mut in_stack_0000fe70: u16;
    let mut in_stack_0000ff94: u16;
    let mut in_stack_0000ff9a: u16;
    let mut in_stack_0000ff9e: u16;
    let mut uStack18: u16;
    let mut uStack16: u16;
    let mut uStack14: u16;
    let mut iStack12: i16;

    uVar14 = (param_5 >> 0x10);
    iVar12 = param_5;
    uVar1 = (iVar12 + 0x1f6);
    uVar6 = (iVar12 + 0x1f8);
    uVar5 = uVar1 + 0x18c;
    uVar4 = (uVar1 >> 0x10);
    uVar7 = uVar5;
    pass1_1030_38f2(uVar1 & 0xffff | uVar6 << 0x10, param_4);
    uVar3 = 0x64 / param_3;
    uVar10 = uVar3 >> 0xf;
    iVar13 = param_4 * 0x4;
    lVar2 = (uVar7 & 0xffff | uVar6 << 0x10) + (iVar13 + uVar5);
    lVar8 = lVar2 / (uVar3 & 0xffff | uVar10 << 0x10);
    lVar9 = lVar8 * (uVar3 & 0xffff | uVar10 << 0x10);
    uStack14 = lVar2;
    iStack12 = (lVar2 >> 0x10);
    uVar6 = lVar9;
    uVar10 = (iStack12 - (lVar9 >> 0x10)) - (uStack14 < uVar6);
    paVar11 = uVar10;
    (uVar5 + iVar13) = uStack14 - uVar6;
    (uVar5 + iVar13 + 0x2) = uVar10;
    uStack16 = (lVar8 >> 0x10);
    uStack18 = lVar8;
    if ((uStack16 | uStack18) != 0) {
        pass1_1030_375a(uVar1, param_4, lVar8);
        if ((iVar12 + 0x200) != 0x8000002) {
            paVar15 = mixed_1010_20ba(
                paVar11,
                _u16_1050_0ed0,
                CONCAT22(unaff_SI, 0x37),
                in_stack_0000fe70,
                in_stack_0000ff94,
                in_stack_0000ff9a,
                in_stack_0000ff9e,
            );
            paVar11 = (paVar11 & 0xffff0000 | paVar15 >> 0x10);
            post_win_msg_1008_a0e4(
                paVar15,
                0x0,
                uStack18,
                (iVar12 + 0x208),
                (iVar12 + 0x4),
                0x2,
            );
            paVar16 = mixed_1010_20ba(
                paVar11,
                _u16_1050_0ed0,
                CONCAT22(unaff_SI, 0x2b),
                in_stack_0000fe70,
                in_stack_0000ff94,
                in_stack_0000ff9a,
                in_stack_0000ff9e,
            );
            pass1_1010_043a(paVar16, (iVar12 + 0x4), 0xd);
        }
    }
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn pass1_1028_a28a(mut param_1: u16, mut param_2: u16, param_3: *mut astruct_691) {
    let mut ppcVar1: *mut *mut code;
    let mut uVar2: u16;
    let mut uVar3: u16;
    let mut uVar4: u32;
    let mut puVar5: *mut u8;
    let mut puVar6: *mut u8;
    let mut puVar7: *mut u8;
    let mut uVar8: u16;
    let mut iVar9: *mut astruct_691;
    let mut uVar9: u16;
    let mut puVar10: *mut u32;
    let mut puStack10: *mut u32;

    puVar10 = pass1_1008_c6fa(_u16_1050_06e0, 0xe);
    puVar5 = (puVar10 >> 0x10);
    uVar2 = puVar10;
    pass1_1038_4d6e(uVar2, puVar5, param_3, puVar10);
    puStack10 = CONCAT22(puVar5, uVar2);
    uVar9 = (param_3 >> 0x10);
    iVar9 = param_3;
    uVar4 = iVar9.field502_0x1f6;
    ppcVar1 = (*puStack10 + 0x10);
    puVar6 = puVar5;
    (**ppcVar1)(&u16_1050_1038, uVar2, puVar5);
    uVar3 = uVar4;
    puVar7 = puVar6;
    pass1_1030_38b8();
    if ((uVar4 & 0xffff | ZEXT24(puVar6) << 0x10) == 0) {
        uVar4 = 0x64;
        uVar8 = 0;
    } else {
        uVar4 = CONCAT22(puVar7, uVar3) / (uVar4 & 0xffff | ZEXT24(puVar6) << 0x10);
        uVar8 = (uVar4 >> 0x10);
    }
    uVar4 = uVar4 & 0xffff | uVar8 << 0x10;
    if (puStack10.is_null() == false) {
        ppcVar1 = *puStack10;
        (**ppcVar1)(0x1030, uVar2, puVar5, 1);
    }
    if (uVar4 < 0x64) {
        if (uVar4 < 0x55) {
            if (uVar4 < 0x4b) {
                if (uVar4 < 0x32) {
                    if (uVar4 < 0x19) {
                        iVar9.field519_0x20a = 0x1;
                        iVar9.field520_0x20c = 0xffff;
                        return;
                    }
                    iVar9.field519_0x20a = 0;
                    iVar9.field520_0x20c = 0;
                    return;
                }
                iVar9.field519_0x20a = 0xfffb;
            } else {
                iVar9.field519_0x20a = 0xfff6;
            }
        } else {
            iVar9.field519_0x20a = 0xfff1;
        }
    } else {
        iVar9.field519_0x20a = 0xffec;
    }
    iVar9.field520_0x20c = 0x1;
    return;
}

// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn pass1_1028_a3ae(
    mut param_1: i16,
    param_2: i32,
    mut param_3: u16,
    mut param_4: u16,
    mut param_5: u32,
) {
    let mut BVar1: bool;
    let mut uVar2: u16;
    let mut uVar3: u32;
    let mut paVar4: *mut Struct57;
    let mut uVar5: u16;
    let mut unaff_SI: u16;
    let mut paVar6: *mut Struct27;
    let mut in_stack_0000fd5a: u16;
    let mut in_stack_0000fe7e: u16;
    let mut in_stack_0000fe84: u16;
    let mut in_stack_0000fe88: u16;
    let mut iVar7: i16;
    let mut uVar8: u16;
    let mut local_146: u16;
    let mut uStack324: u16;
    let mut uStack32: u16;
    let mut uStack30: u16;
    let mut uStack26: u32;
    let mut uStack22: u32;
    let mut uStack18: u16;
    let mut uStack16: u16;
    let mut uStack14: u32;
    let mut uStack10: u32;
    let mut iStack6: i16;
    let mut uStack4: u16;

    iVar7 = param_5;
    uVar8 = (param_5 >> 0x10);
    pass1_1038_3fb0(param_5);
    uStack4 = param_2;
    iStack6 = param_1;
    if (((iVar7 + 0x204) != 0)
        && (
            BVar1 = pass1_1030_25b2(CONCAT22(uStack4, param_1), 0x82),
            BVar1 != 0,
        ))
    {
        return;
    }
    uVar3 = (iVar7 + 0x1f6);
    uStack10 = uVar3;
    pass1_1030_38b8();
    uVar2 = uVar3;
    uStack14 = uVar3 & 0xffff | param_2 << 0x10;
    empty_1038_540a();
    uStack16 = param_2;
    paVar4 = (param_2 & 0xffff0000 | (uStack16 | uVar2));
    uStack18 = uVar2;
    if (((uStack16 | uVar2) == 0) && ((iVar7 + 0x200) != 0x8000002)) {
        pass1_1030_38b8();
        if ((-0x1 < paVar4) && (0x0 < paVar4 || (uVar2 != 0))) {
            paVar6 = mixed_1010_20ba(
                paVar4,
                _u16_1050_0ed0,
                CONCAT22(unaff_SI, 0x2b),
                in_stack_0000fd5a,
                in_stack_0000fe7e,
                in_stack_0000fe84,
                in_stack_0000fe88,
            );
            uStack30 = (paVar6 >> 0x10);
            uStack32 = SUB42(paVar6, 0x0);
            pass1_1010_043a(paVar6, (iVar7 + 0x4), 0x11);
        }
    }
    uStack26 = uStack14;
    uVar2 = uStack18 * 0xa;
    uVar5 = (uStack16 * 0x5
        + CARRY2(uStack18, uStack18) * 0x2
        + CARRY2(uStack18 * 0x2, uStack18 * 0x2)
        + CARRY2(uStack18 * 0x4, uStack18))
        * 0x2
        + CARRY2(uStack18 * 0x5, uStack18 * 0x5);
    uStack22 = CONCAT22(uVar5, uVar2);
    if ((uVar5 <= uStack14) && (uVar5 < uStack14 || (uVar2 < uStack14))) {
        pass1_1028_ae66(
            CONCAT22(0x1050, &local_146),
            uStack14,
            CONCAT22(uVar5, uVar2),
            (iVar7 + 0x4),
        );
        fn_ptr_1030_835a(_u16_1050_5748, CONCAT22(0x1050, &local_146));
        uStack26 = uStack22;
        local_146 = 0x389a;
        uStack324 = 0x1008;
    }
    uStack26 += 0x9;
    pass1_1038_52b8(param_5, uStack26 / 0xa, 0x1e);
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn pass1_1028_a4ee(mut param_1: u32, mut param_2: u32) {
    let mut uVar1: u32;
    let mut ppcVar2: *mut *mut code;
    let mut uVar3: u16;
    let mut BVar4: bool;
    let mut uVar5: u16;
    let mut uVar6: u32;
    let mut puVar7: *mut u8;
    let mut puVar8: *mut u8;
    let mut uVar9: u16;
    let mut uVar10: u16;
    let mut puVar11: *mut u32;
    let mut uVar12: u16;
    let mut iStack50: i16;
    let mut puStack18: *mut u32;

    uVar9 = (param_2 >> 0x10);
    uVar1 = (param_2 + 0x1f6);
    uVar6 = *_PTR_LOOP_1050_65e2;
    puVar11 = pass1_1008_c6fa(_u16_1050_06e0, 0x26);
    puVar7 = (puVar11 >> 0x10);
    uVar5 = puVar11;
    uVar10 = SUB42(&u16_1050_1038, 0x0);
    pass1_1038_4d6e(uVar5, puVar7, param_2, puVar11);
    puStack18 = CONCAT22(puVar7, uVar5);
    ppcVar2 = (*puStack18 + 0x10);
    uVar3 = uVar5;
    puVar8 = puVar7;
    (**ppcVar2)(&u16_1050_1038, uVar5, puVar7);
    if ((puVar8 | uVar3) != 0) {
        uVar10 = 0x1030;
        pass1_1030_3548(uVar1, CONCAT22(puVar8, uVar3));
    }
    if (puStack18.is_null() == false) {
        ppcVar2 = *puStack18;
        (**ppcVar2)(uVar10, uVar5, puVar7, 1);
    }
    uVar3 = (uVar6 % 0xc);
    uVar12 = (param_1 >> 0x10);
    uVar5 = uVar3;
    if (uVar6 % 0xc == 0) {
        pass1_1030_387c(uVar1);
        pass1_1028_a61e(uVar5, uVar3, param_1, uVar12, uVar1, param_2);
    }
    pass1_1038_3fb0(param_2);
    if (((param_2 + 0x204) != 0)
        && (
            BVar4 = pass1_1030_25b2(CONCAT13((uVar3 >> 0x8), CONCAT12(uVar3, uVar5)), 0x80),
            BVar4 != 0,
        ))
    {
        return;
    }
    uVar9 = (uVar1 >> 0x10);
    uVar5 = uVar1 + 0x180;
    uVar6 = uVar5;
    iStack50 = 0x1;
    loop {
        if ((iStack50 * 0x2 + uVar5) != 0) {
            pass1_1008_612e(uVar6, 0x1, 0x64);
            if (uVar6 <= (iStack50 * 0x2 + uVar5)) {
                pass1_1028_a188(
                    param_1,
                    uVar12,
                    (iStack50 * 0x2 + uVar1 + 0x174),
                    iStack50,
                    param_2,
                );
            }
        }
        iStack50 += 0x1;
        if iStack50 >= 6 {
            break;
        }
    }
    return;
}

// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn pass1_1028_a61e(
    mut param_1: u16,
    mut param_2: i16,
    mut param_3: u16,
    mut param_4: u16,
    mut param_5: u32,
    mut param_6: u32,
) {
    let mut uVar1: u16;
    let mut uVar2: u32;
    let mut iVar3: i16;
    let mut uVar4: u16;
    let mut paVar5: *mut Struct27;
    let mut in_stack_0000fe84: u16;
    let mut in_stack_0000ffa8: u16;
    let mut in_stack_0000ffae: u16;
    let mut in_stack_0000ffb2: u16;
    let mut in_stack_0000ffdc: u16;
    let mut uStack16: u16;
    let mut uStack14: u32;

    pass1_1030_38b8();
    if ((param_2 < 0x3fff) || (param_2 < 0x4000 && (param_1 != 0xffff))) {
        pass1_1030_38f2(param_5, 0x3);
        uVar1 = param_1;
        iVar3 = param_2;
        pass1_1030_38f2(param_5, 0x4);
        uStack14 = CONCAT22(param_2 + iVar3 + CARRY2(param_1, uVar1), param_1 + uVar1);
        uStack16 = (param_5 + 0x1a8);
        if (uStack16 == 0) {
            uStack16 = 0x5;
        }
        uVar2 = uStack14 / uStack16;
        uStack14 = (uVar2 >> 0x10);
        uStack14 |= uVar2;
        if ((uStack14 != 0) && (uVar4 = (param_6 >> 0x10), (param_6 + 0x200) != 0x8000002)) {
            paVar5 = mixed_1010_20ba(
                (uStack14 % uStack16 & 0xffff0000 | uStack14),
                _u16_1050_0ed0,
                CONCAT22(in_stack_0000ffdc, 0x2b),
                in_stack_0000fe84,
                in_stack_0000ffa8,
                in_stack_0000ffae,
                in_stack_0000ffb2,
            );
            pass1_1010_043a(paVar5, (param_6 + 0x4), 0xc);
            pass1_1030_3534(param_5, uVar2);
        }
    }
    return;
}

pub unsafe fn pass1_1028_a6ca(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    param_1.address_offset_field_0x0 = 0x389a;
    (param_1 + 0x2) = 0x1008;
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1028_a706(param_1: *mut astruct_97) -> *mut astruct_97 {
    struct_op_1028_d1dc(param_1, 0xbb7);
    param_1.offset_0x0 = 0xa856;
    (param_1 + 0x2) = 0x1028;
    unk_str_op_1000_3d3e(
        (param_1 & 0xffff0000 | (param_1 + 0x8)),
        s_SCPrelimAlloc_1050_50f6,
    );
    return param_1;
}

pub unsafe fn pass1_1028_a73c(mut param_1: u16) -> u16 {
    let mut paVar1: *mut astruct_92;
    let mut paVar2: *mut astruct_92;
    let mut uVar3: u16;
    let mut local_14: *mut astruct_92;

    pass1_1028_dc52(CONCAT22(0x1050, &local_14), 0x1, 0x0, 0x400);
    loop {
        paVar1 = &local_14;
        pass1_1028_e4ec(CONCAT22(0x1050, paVar1));
        uVar3 = param_1 | paVar1;
        if (uVar3 == 0) {
            break;
        }
        paVar2 = paVar1;
        pass1_1038_5464(paVar1, CONCAT22(param_1, paVar1));
        pass1_1038_56d6(CONCAT22(param_1, paVar1), 0x0);
        pass1_1038_518c(paVar2, CONCAT22(param_1, paVar1));
        param_1 = uVar3;
    }
    return 0x1;
}
pub unsafe fn pass1_1028_a79c(mut param_1: u16, param_2: *mut u8, mut param_3: u32) {
    let mut puVar1: *mut u32;
    let mut puVar2: *mut u32;
    let mut puVar3: *mut u32;
    let mut iVar4: i16;
    let mut uVar5: u16;
    let mut in_register_0000000a: u16;
    let mut paVar6: *mut Struct57;
    let mut puVar7: *mut u32;
    let mut uVar8: u16;
    let mut puStack10: *mut u16;

    paVar6 = CONCAT22(in_register_0000000a, param_2);
    mem_op_1000_179c(0x108, paVar6);
    uVar5 = paVar6;
    puStack10 = CONCAT22(uVar5, param_1);
    if ((uVar5 | param_1) != 0) {
        *puStack10 = 0x389a;
        (param_1 + 0x2) = 0x1008;
        uVar8 = (param_3 >> 0x10);
        (param_1 + 0x4) = (param_3 + 0x4);
        puVar3 = (param_3 + 0x8);
        puVar7 = (param_1 + 0x8);
        for iVar4 in 0x40..0 {
            puVar2 = puVar7;
            puVar7 = puVar7 + 1;
            puVar1 = puVar3;
            puVar3 = puVar3 + 1;
            *puVar2 = *puVar1;
        }
        *puStack10 = 0x6ad2;
        (param_1 + 0x2) = 0x1028;
        *puStack10 = 0xa856;
        (param_1 + 0x2) = 0x1028;
    }
    return;
}

pub unsafe fn pass1_1028_a82a(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    param_1.address_offset_field_0x0 = 0x389a;
    (param_1 + 0x2) = 0x1008;
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1028_a866(param_1: *mut astruct_97) -> *mut astruct_97 {
    struct_op_1028_d1dc(param_1, 0x36af);
    param_1.offset_0x0 = 0xa9ae;
    (param_1 + 0x2) = 0x1028;
    unk_str_op_1000_3d3e(
        (param_1 & 0xffff0000 | (param_1 + 0x8)),
        s_SCProdSched_1050_5104,
    );
    return param_1;
}

pub unsafe fn pass1_1028_a89c(mut param_1: u16) -> u16 {
    let mut paVar1: *mut astruct_92;
    let mut uVar2: u16;
    let mut local_14: *mut astruct_92;

    pass1_1028_dc52(CONCAT22(0x1050, &local_14), 0x1, 0x0, 0x400);
    loop {
        uVar2 = param_1;
        paVar1 = &local_14;
        pass1_1028_e4ec(CONCAT22(0x1050, paVar1));
        param_1 = uVar2 | paVar1;
        if (param_1 == 0) {
            break;
        }
        if (paVar1[0x1c].field4_0x8 != 0x8000002) {
            pass1_1038_3fca(paVar1, CONCAT22(uVar2, paVar1));
        }
    }
    return 0x1;
}
pub unsafe fn pass1_1028_a8f4(param_1: *mut astruct_335, mut param_2: u16, mut param_3: u32) {
    let mut puVar1: *mut u32;
    let mut puVar2: *mut u32;
    let mut puVar3: *mut u32;
    let mut iVar4: i16;
    let mut uVar5: u16;
    let mut in_register_0000000a: u16;
    let mut paVar6: *mut Struct57;
    let mut puVar7: *mut u32;
    let mut uVar8: u16;
    let mut puStack10: *mut u16;

    paVar6 = CONCAT22(in_register_0000000a, param_2);
    mem_op_1000_179c(0x108, paVar6);
    uVar5 = paVar6;
    puStack10 = CONCAT22(uVar5, param_1);
    if ((uVar5 | param_1) != 0) {
        *puStack10 = 0x389a;
        param_1.field2_0x2 = 0x1008;
        uVar8 = (param_3 >> 0x10);
        param_1.field3_0x4 = (param_3 + 0x4);
        puVar3 = (param_3 + 0x8);
        puVar7 = &param_1.field4_0x8;
        for iVar4 in 0x40..0 {
            puVar2 = puVar7;
            puVar7 = puVar7 + 1;
            puVar1 = puVar3;
            puVar3 = puVar3 + 1;
            *puVar2 = *puVar1;
        }
        *puStack10 = 0x6ad2;
        param_1.field2_0x2 = 0x1028;
        *puStack10 = 0xa9ae;
        param_1.field2_0x2 = 0x1028;
    }
    return;
}

pub unsafe fn pass1_1028_a982(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    param_1.address_offset_field_0x0 = 0x389a;
    (param_1 + 0x2) = 0x1008;
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1028_a9be(param_1: *mut astruct_97) -> *mut astruct_97 {
    struct_op_1028_d1dc(param_1, 0x176f);
    param_1.offset_0x0 = 0xab22;
    (param_1 + 0x2) = 0x1028;
    unk_str_op_1000_3d3e(
        (param_1 & 0xffff0000 | (param_1 + 0x8)),
        s_SCPower_1050_5110,
    );
    return param_1;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn pass1_1028_a9f4(mut param_1: u16) -> u16 {
    let mut ppcVar1: *mut *mut code;
    let mut paVar2: *mut astruct_92;
    let mut BVar3: bool;
    let mut uVar4: u16;
    let mut extraout_DX: u16;
    let mut puStack24: *mut u32;
    let mut local_14: *mut astruct_92;

    pass1_1028_dc52(CONCAT22(0x1050, &local_14), 0x1, 0x0, 0x700);
    loop {
        paVar2 = &local_14;
        pass1_1028_e4ec(CONCAT22(0x1050, paVar2));
        puStack24 = CONCAT22(param_1, paVar2);
        uVar4 = param_1 | paVar2;
        if (uVar4 == 0) {
            break;
        }
        BVar3 = pass1_1008_c6ae(_u16_1050_06e0, &paVar2.field5_0xc, 0xc);
        param_1 = uVar4;
        if (BVar3 != 0) {
            ppcVar1 = (*puStack24 + 0x34);
            (**ppcVar1)(0x1008, paVar2);
            param_1 = extraout_DX;
        }
    }
    return 0x1;
}
pub unsafe fn pass1_1028_aa68(param_1: *mut astruct_336, param_2: *mut u8, mut param_3: u32) {
    let mut puVar1: *mut u32;
    let mut puVar2: *mut u32;
    let mut puVar3: *mut u32;
    let mut iVar4: i16;
    let mut uVar5: u16;
    let mut in_register_0000000a: u16;
    let mut paVar6: *mut Struct57;
    let mut puVar7: *mut u32;
    let mut uVar8: u16;
    let mut puStack10: *mut u16;

    paVar6 = CONCAT22(in_register_0000000a, param_2);
    mem_op_1000_179c(0x108, paVar6);
    uVar5 = paVar6;
    puStack10 = CONCAT22(uVar5, param_1);
    if ((uVar5 | param_1) != 0) {
        *puStack10 = 0x389a;
        param_1.field2_0x2 = 0x1008;
        uVar8 = (param_3 >> 0x10);
        param_1.field3_0x4 = (param_3 + 0x4);
        puVar3 = (param_3 + 0x8);
        puVar7 = &param_1.field4_0x8;
        for iVar4 in 0x40..0 {
            puVar2 = puVar7;
            puVar7 = puVar7 + 1;
            puVar1 = puVar3;
            puVar3 = puVar3 + 1;
            *puVar2 = *puVar1;
        }
        *puStack10 = 0x6ad2;
        param_1.field2_0x2 = 0x1028;
        *puStack10 = 0xab22;
        param_1.field2_0x2 = 0x1028;
    }
    return;
}

pub unsafe fn pass1_1028_aaf6(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    param_1.address_offset_field_0x0 = 0x389a;
    (param_1 + 0x2) = 0x1008;
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1028_ab32(param_1: *mut astruct_97) -> *mut astruct_97 {
    struct_op_1028_d1dc(param_1, 0x2edf);
    param_1.offset_0x0 = 0xaca6;
    (param_1 + 0x2) = 0x1028;
    unk_str_op_1000_3d3e(
        (param_1 & 0xffff0000 | (param_1 + 0x8)),
        s_SCRchSched_1050_5118,
    );
    return param_1;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn pass1_1028_ab68(mut param_1: u16) -> u16 {
    let mut uVar1: u16;
    let mut ppcVar2: *mut *mut code;
    let mut paVar3: *mut astruct_92;
    let mut BVar4: bool;
    let mut uVar5: u16;
    let mut extraout_DX: u16;
    let mut puStack24: *mut u32;
    let mut local_14: *mut astruct_92;

    pass1_1028_dc52(CONCAT13(0x10, CONCAT12(0x50, &local_14)), 0x1, 0x0, 0x700); //
                                                                                 // LAB_1028_ab7e:
    uVar5 = param_1;
    paVar3 = &local_14;
    pass1_1028_e4ec(CONCAT22(0x1050, paVar3));
    puStack24 = CONCAT22(uVar5, paVar3);
    param_1 = uVar5 | paVar3;
    if (param_1 == 0) {
        return 0x1;
    }
    uVar1 = &paVar3.field5_0xc;
    BVar4 = pass1_1008_c6ae(_u16_1050_06e0, uVar1, 0x11);
    //  if (BVar4 == 0) goto code_r0x1028abad;
    //   goto LAB_1028_abc0;
    // code_r0x1028abad:
    BVar4 = pass1_1008_c6ae(_u16_1050_06e0, uVar1, 0x12);
    if (BVar4 != 0) {
        //
        // LAB_1028_abc0:
        if ((paVar3 + 1) == 0x5) {
            ppcVar2 = (*puStack24 + 0x30);
            (**ppcVar2)(0x1008);
            param_1 = extraout_DX;
        }
    }
    //   goto LAB_1028_ab7e;
}
pub unsafe fn pass1_1028_abec(param_1: *mut astruct_337, param_2: *mut u8, mut param_3: u32) {
    let mut puVar1: *mut u32;
    let mut puVar2: *mut u32;
    let mut puVar3: *mut u32;
    let mut iVar4: i16;
    let mut uVar5: u16;
    let mut in_register_0000000a: u16;
    let mut paVar6: *mut Struct57;
    let mut puVar7: *mut u32;
    let mut uVar8: u16;
    let mut puStack10: *mut u16;

    paVar6 = CONCAT22(in_register_0000000a, param_2);
    mem_op_1000_179c(0x108, paVar6);
    uVar5 = paVar6;
    puStack10 = CONCAT22(uVar5, param_1);
    if ((uVar5 | param_1) != 0) {
        *puStack10 = 0x389a;
        param_1.field2_0x2 = 0x1008;
        uVar8 = (param_3 >> 0x10);
        param_1.field3_0x4 = (param_3 + 0x4);
        puVar3 = (param_3 + 0x8);
        puVar7 = &param_1.field4_0x8;
        for iVar4 in 0x40..0 {
            puVar2 = puVar7;
            puVar7 = puVar7 + 1;
            puVar1 = puVar3;
            puVar3 = puVar3 + 1;
            *puVar2 = *puVar1;
        }
        *puStack10 = 0x6ad2;
        param_1.field2_0x2 = 0x1028;
        *puStack10 = 0xaca6;
        param_1.field2_0x2 = 0x1028;
    }
    return;
}

pub unsafe fn pass1_1028_ac7a(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    param_1.address_offset_field_0x0 = 0x389a;
    (param_1 + 0x2) = 0x1008;
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1028_acb6(param_1: *mut astruct_97) -> *mut astruct_97 {
    struct_op_1028_d1dc(param_1, 0x3e7f);
    param_1.offset_0x0 = 0xae56;
    (param_1 + 0x2) = 0x1028;
    unk_str_op_1000_3d3e(
        (param_1 & 0xffff0000 | (param_1 + 0x8)),
        s_SCSetup_1050_5124,
    );
    return param_1;
}

// WARNING: Could not reconcile some variable overlaps

pub unsafe fn pass1_1028_acec(mut param_1: u16) -> u16 {
    let mut paVar1: *mut astruct_92;
    let mut paVar2: *mut astruct_92;
    let mut uVar3: u16;
    let mut local_14: *mut astruct_92;

    pass1_1028_dc52(CONCAT22(0x1050, &local_14), 0x1, 0x0, 0x400);
    loop {
        uVar3 = param_1;
        paVar1 = &local_14;
        pass1_1028_e4ec(CONCAT22(0x1050, paVar1));
        param_1 = uVar3 | paVar1;
        if (param_1 == 0) {
            break;
        }
        paVar2 = paVar1;
        vsprintf_op_1030_840a(param_1, s_SCSetup__calcMe_clearing_colony_0_1050_512c);
        if (paVar1[0x1c].field4_0x8 != 0x8000002) {
            pass1_1038_5464(paVar2, CONCAT22(uVar3, paVar1));
            pass1_1038_56d6(CONCAT22(uVar3, paVar1), 1);
        }
    }
    local_14 = 0x389a;
    local_14.field2_0x2 = 0x1008;
    pass1_1028_dc52(CONCAT22(0x1050, &local_14), 0x1, 0x0, 0x800);
    loop {
        paVar1 = &local_14;
        pass1_1028_e4ec(CONCAT22(0x1050, paVar1));
        uVar3 = param_1 | paVar1;
        if (uVar3 == 0) {
            break;
        }
        pass1_1030_2690(CONCAT22(param_1, paVar1));
        param_1 = uVar3;
    }
    return 0x1;
}
pub unsafe fn pass1_1028_ad9c(param_1: *mut astruct_338, mut param_2: u16, mut param_3: u32) {
    let mut puVar1: *mut u32;
    let mut puVar2: *mut u32;
    let mut puVar3: *mut u32;
    let mut iVar4: i16;
    let mut uVar5: u16;
    let mut in_register_0000000a: u16;
    let mut paVar6: *mut Struct57;
    let mut puVar7: *mut u32;
    let mut uVar8: u16;
    let mut puStack10: *mut u16;

    paVar6 = CONCAT22(in_register_0000000a, param_2);
    mem_op_1000_179c(0x108, paVar6);
    uVar5 = paVar6;
    puStack10 = CONCAT22(uVar5, param_1);
    if ((uVar5 | param_1) != 0) {
        *puStack10 = 0x389a;
        param_1.field2_0x2 = 0x1008;
        uVar8 = (param_3 >> 0x10);
        param_1.field3_0x4 = (param_3 + 0x4);
        puVar3 = (param_3 + 0x8);
        puVar7 = &param_1.field4_0x8;
        for iVar4 in 0x40..0 {
            puVar2 = puVar7;
            puVar7 = puVar7 + 1;
            puVar1 = puVar3;
            puVar3 = puVar3 + 1;
            *puVar2 = *puVar1;
        }
        *puStack10 = 0x6ad2;
        param_1.field2_0x2 = 0x1028;
        *puStack10 = 0xae56;
        param_1.field2_0x2 = 0x1028;
    }
    return;
}

pub unsafe fn pass1_1028_ae2a(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    param_1.address_offset_field_0x0 = 0x389a;
    (param_1 + 0x2) = 0x1008;
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}
pub unsafe fn pass1_1028_ae66(
    param_1: *mut astruct_97,
    mut param_2: u32,
    mut param_3: u32,
    mut param_4: u32,
) {
    let mut iVar1: *mut astruct_97;
    let mut uVar1: u16;

    struct_op_1028_d1dc(param_1, 0x1387);
    uVar1 = (param_1 >> 0x10);
    iVar1 = param_1;
    iVar1.field259_0x108 = param_4;
    iVar1.field262_0x10c = param_3;
    iVar1.field264_0x110 = param_2;
    iVar1.field265_0x114 = 0;
    param_1.offset_0x0 = 0xb0ce;
    iVar1.segment_0x2 = 0x1028;
    unk_str_op_1000_3d3e(
        (param_1 & 0xffff0000 | ZEXT24(&iVar1.string_0x8)),
        s_SCStarve_1050_5156,
    );
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn pass1_1028_aec0(mut param_1: i16, mut param_2: u16, mut param_3: u32) {
    let mut uVar1: u16;

    uVar1 = (param_3 >> 0x10);
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2, (param_3 + 0x108));
    pass1_1030_375a((param_1 + 0x1f6), 0x0, (param_3 + 0x114));
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn pass1_1028_af08(
    param_1: *mut Struct57,
    param_2: *mut astruct_693,
    mut param_3: u16,
) -> u16 {
    let mut puVar1: *mut u8;
    let mut puVar2: *mut u8;
    let mut iVar3: i16;
    let mut uVar4: u16;
    let mut paVar5: *mut Struct57;
    let mut iVar6: *mut astruct_693;
    let mut uVar6: u16;
    let mut puVar7: *mut u32;
    let mut paVar8: *mut astruct_67;
    let mut paVar9: *mut Struct27;
    let mut in_stack_0000fe86: u16;
    let mut in_stack_0000ffaa: u16;
    let mut in_stack_0000ffb0: u16;
    let mut in_stack_0000ffb4: u16;
    let mut iStack12: i16;
    let mut iStack10: i16;

    puVar7 = mixed_1010_20ba(
        param_1,
        _u16_1050_0ed0,
        CONCAT22(param_3, 0x2),
        in_stack_0000fe86,
        in_stack_0000ffaa,
        in_stack_0000ffb0,
        in_stack_0000ffb4,
    );
    paVar5 = (param_1 & 0xffff0000 | puVar7 >> 0x10);
    puVar1 = PTR_LOOP_1050_13ae -0x1;
    if ((PTR_LOOP_1050_13ae < 1) || (SBORROW2(PTR_LOOP_1050_13ae, 1))) {
        //
        // LAB_1028_af27:
        iStack10 = 0x1;
    } else {
        puVar2 = PTR_LOOP_1050_13ae -0x2;
        if (puVar2.is_null() || puVar1 < 1) {
            iStack12 = 0x1;
            iStack10 = 0x1;
            // TODO: goto LAB_1028_af42;
        }
        puVar1 = PTR_LOOP_1050_13ae -0x4;
        //    if (puVar1.is_null() == false) goto LAB_1028_af27;
        iStack10 = 0x2;
    }
    iStack12 = 0x3;
    puVar2 = puVar1; //
                     // LAB_1028_af42:
    pass1_1008_612e(puVar2, iStack10, iStack12);
    uVar6 = (param_2 >> 0x10);
    iVar6 = param_2;
    iVar6.field273_0x114 = puVar2;
    paVar8 = mixed_1010_20ba(
        paVar5,
        _u16_1050_0ed0,
        CONCAT22(param_3, 0x37),
        in_stack_0000fe86,
        in_stack_0000ffaa,
        in_stack_0000ffb0,
        in_stack_0000ffb4,
    );
    paVar5 = (paVar5 & 0xffff0000 | paVar8 >> 0x10);
    iVar3 = paVar8;
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2, iVar6.field264_0x108);
    uVar4 = SUB42(paVar5, 0x0);
    post_win_msg_1008_a0e4(
        paVar8,
        0x0,
        iVar6.field273_0x114,
        (iVar3 + 0x208),
        iVar6.field264_0x108,
        0x2,
    );
    paVar9 = mixed_1010_20ba(
        paVar5,
        _u16_1050_0ed0,
        CONCAT22(param_3, 0x2b),
        in_stack_0000fe86,
        in_stack_0000ffaa,
        in_stack_0000ffb0,
        in_stack_0000ffb4,
    );
    pass1_1010_043a(paVar9, (iVar3 + 0x4), 0xd);
    return 0x1;
}
pub unsafe fn pass1_1028_afce(
    param_1: *mut astruct_339,
    mut param_2: u16,
    param_3: *mut astruct_340,
) {
    let mut puVar1: *mut u32;
    let mut puVar2: *mut u32;
    let mut iVar3: i16;
    let mut uVar4: u16;
    let mut in_register_0000000a: u16;
    let mut paVar5: *mut Struct57;
    let mut iVar5: *mut astruct_340;
    let mut puVar6: *mut u32;
    let mut puVar7: *mut u32;
    let mut uVar8: u16;
    let mut puStack10: *mut u16;

    paVar5 = CONCAT22(in_register_0000000a, param_2);
    mem_op_1000_179c(0x116, paVar5);
    uVar4 = paVar5;
    puStack10 = CONCAT22(uVar4, param_1);
    if ((uVar4 | param_1) != 0) {
        *puStack10 = 0x389a;
        param_1.field2_0x2 = 0x1008;
        uVar8 = (param_3 >> 0x10);
        iVar5 = param_3;
        param_1.field3_0x4 = iVar5.field4_0x4;
        puVar6 = &iVar5.field5_0x8;
        puVar7 = &param_1.field4_0x8;
        for iVar3 in 0x40..0 {
            puVar2 = puVar7;
            puVar7 = puVar7 + 1;
            puVar1 = puVar6;
            puVar6 = puVar6 + 1;
            *puVar2 = *puVar1;
        }
        *puStack10 = 0x6ad2;
        param_1.field2_0x2 = 0x1028;
        param_1.field257_0x108 = iVar5.field258_0x108;
        param_1.field258_0x10c = iVar5.field259_0x10c;
        param_1.field259_0x110 = iVar5.field260_0x110;
        param_1.field260_0x114 = iVar5.field261_0x114;
        *puStack10 = 0xb0ce;
        param_1.field2_0x2 = 0x1028;
    }
    return;
}
