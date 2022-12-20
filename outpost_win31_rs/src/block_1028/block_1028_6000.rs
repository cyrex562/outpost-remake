pub unsafe fn struct_1028_60bc(
    param_1: *mut astruct_180,
    param_2: *mut Struct57,
    mut param_3: u16,
) -> *mut u16 {
    let mut uVar1: u16;
    let mut uVar2: u16;
    let mut iVar1: *mut astruct_180;
    let mut uVar3: u16;

    struct_1028_b354(param_1);
    uVar3 = (param_1 >> 0x10);
    iVar1 = param_1;
    (iVar1 + 1) = 0;
    param_1.field0_0x0 = 0x6876;
    iVar1.field1_0x2 = 0x1028;
    mem_op_1000_179c(0xc, param_2);
    uVar2 = param_2 | param_3;
    if (uVar2 == 0) {
        (iVar1 + 1) = 0;
    } else {
        uVar1 = set_struct_1008_574a(CONCAT22(param_2, param_3));
        (iVar1 + 1).field0_0x0 = uVar1;
        iVar1[0x1].field1_0x2 = uVar2;
    }
    return &param_1.field0_0x0;
}

pub unsafe fn pass1_1028_611e(
    param_1: *mut StructD,
    param_2: *mut astruct_12,
    mut param_3: i16,
    mut param_4: u32,
) -> u32 {
    let mut uVar1: u16;
    let mut uVar2: u16;
    let mut uVar3: u16;
    let mut paVar4: *mut Struct57;

    uVar1 = (param_1 >> 0x10);
    paVar4 = (param_1 & 0xffff0000 | param_1 & 0xffff);
    pass1_1028_b39e((param_1 & 0xffff), param_2, param_3, param_4);
    (param_2 + 0x20) = 0;
    param_2.field0_0x0 = 0x6876;
    (param_2 + 0x2) = 0x1028;
    mem_op_1000_179c(0xc, paVar4);
    uVar3 = paVar4 | uVar1;
    if (uVar3 == 0) {
        (param_2 + 0x20) = 0;
    } else {
        uVar2 = set_struct_1008_574a(CONCAT22(paVar4, uVar1));
        (param_2 + 0x20) = uVar2;
        (param_2 + 0x22) = uVar3;
    }
    return param_2;
}

pub unsafe fn pass1_1028_6186(param_1: *mut StructD) {
    let mut puVar1: *mut u32;
    let mut uVar2: u16;
    let mut ppcVar3: *mut *mut code;
    let mut iVar4: *mut StructD;
    let mut uVar4: u16;

    uVar4 = (param_1 >> 0x10);
    iVar4 = param_1;
    param_1.address_offset_field_0x0 = 0x6876;
    iVar4.address_offset_field_0x2 = 0x1028;
    puVar1 = iVar4.field19_0x20;
    uVar2 = iVar4.field20_0x22;
    if ((uVar2 | puVar1) != 0) {
        ppcVar3 = *puVar1;
        (**ppcVar3)();
    }
    pass1_1028_b418(&param_1.address_offset_field_0x0);
    return;
}

pub unsafe fn pass1_1028_61c4(mut param_1: u16, param_2: *mut astruct_15, mut param_3: u32) {
    let mut paVar1: *mut astruct_21;
    let mut uVar4: *mut astruct_21;
    let mut uVar2: u16;
    let mut in_EDX: u32;
    let mut uVar5: u16;
    let mut iVar5: *mut astruct_15;
    let mut uVar6: u16;
    let mut puVar1: *mut astruct_21;
    let mut paVar3: *mut Struct57;
    let mut fn_ptr_1: *mut *mut code;

    uVar5 = (in_EDX >> 0x10);
    pass1_1028_b46e(param_1, param_2, param_3);
    uVar6 = (param_2 >> 0x10);
    iVar5 = param_2;
    paVar1 = iVar5.field24_0x20;
    uVar2 = iVar5.field25_0x22;
    uVar4 = (uVar2 | paVar1);
    paVar3 = CONCAT22(uVar5, uVar4);
    if (uVar4.is_null() == false) {
        fn_ptr_1 = paVar1;
        paVar1 = (**fn_ptr_1)();
    }
    mem_op_1000_179c(0xc, paVar3);
    uVar2 = paVar3 | paVar1;
    if (uVar2 == 0) {
        paVar1 = null_mut();
        uVar2 = 0;
    } else {
        paVar1 = set_struct_1008_574a(CONCAT22(paVar3, paVar1));
    }
    iVar5.field24_0x20 = paVar1;
    iVar5.field25_0x22 = uVar2;
    return;
}

pub unsafe fn pass1_1028_6228(
    mut param_1: u32,
    mut param_2: u16,
    mut param_3: i16,
    mut param_4: i16,
) {
    let mut uVar1: u16;
    let mut uVar2: u16;
    let mut ppcVar3: *mut *mut code;
    let mut iVar4: i16;
    let mut uVar5: u16;
    let mut iVar6: i16;
    let mut uVar7: u16;
    let mut bVar8: bool;
    let mut lVar9: i32;
    let mut local_a: [u8; 0x4] = [0; 0x4];
    let mut uStack6: u32;

    uVar7 = (param_1 >> 0x10);
    iVar6 = param_1;
    pass1_1008_5784(CONCAT22(0x1050, local_a), (iVar6 + 0x20));
    loop {
        loop {
            lVar9 = pass1_1008_5b12(CONCAT22(0x1050, local_a));
            uVar5 = (lVar9 >> 0x10);
            iVar4 = lVar9;
            if (lVar9 == 0) {
                return;
            }
            if (iVar4 + 6) == param_4 {
                break;
            }
        }
        uVar1 = (iVar4 + 0xa);
        if ((param_3 == 0) && (param_2 < uVar1)) {
            break;
        }
        bVar8 = param_2 < uVar1;
        param_2 -= uVar1;
        param_3 -= bVar8;
        ppcVar3 = ((iVar6 + 0x20) + 0xc);
        (**ppcVar3)(0x1008, (iVar6 + 0x20));
        uStack6 = 0;
    }
    uVar2 = (iVar4 + 0xc);
    (iVar4 + 0xa) = uVar1 - param_2;
    (iVar4 + 0xc) = -(param_2 * (uVar2 / uVar1) - (iVar4 + 0xc));
    return;
}

pub unsafe fn pass1_1028_62c8(mut param_1: u32) -> u32 {
    let mut uVar1: u16;
    let mut uVar2: u32;

    uVar1 = (param_1 >> 0x10);
    if ((param_1 + 0x12) == 0x5) {
        uVar2 = pass1_1028_67d4(param_1 & 0xffff | uVar1 << 0x10);
        uVar1 = uVar2;
        if (((uVar2 >> 0x10) == 0) && (uVar1 < 0x64)) {
            return CONCAT22(-(0x64 < uVar1), 0x64 - uVar1);
        }
    }
    return 0x0;
}

// WARNING: Could not reconcile some variable overlaps

pub unsafe fn pass1_1028_6302(mut param_1: u32) -> u32 {
    let mut uVar1: u16;
    let mut uVar2: u16;
    let mut lVar3: i32;
    let mut uStack18: u32;
    let mut local_a: [u8; 0x8] = [0; 0x8];

    pass1_1008_5784(CONCAT22(0x1050, local_a), (param_1 + 0x20));
    uStack18 = 0;
    loop {
        lVar3 = pass1_1008_5b12(CONCAT22(0x1050, local_a));
        uVar2 = (lVar3 >> 0x10);
        if (lVar3 == 0) {
            break;
        }
        if ((lVar3 + 0x8) != 0) {
            uVar1 = (lVar3 + 0xa);
            uStack18 = CONCAT22(
                (uStack18 >> 0x10) + CARRY2(uStack18, uVar1),
                uStack18 + uVar1,
            );
        }
    }
    return uStack18;
}

pub unsafe fn pass1_1028_6356(
    mut param_1: u32,
    mut param_2: i16,
    mut param_3: u16,
    mut param_4: i16,
) {
    let mut piVar1: *mut i16;
    let mut uVar2: u16;
    let mut uVar3: u16;
    let mut ppcVar4: *mut *mut code;
    let mut iVar5: i16;
    let mut uVar6: u16;
    let mut iVar7: i16;
    let mut uVar8: u16;
    let mut bVar9: bool;
    let mut lVar10: i32;
    let mut local_a: [u8; 0x4] = [0; 0x4];
    let mut uStack6: u32;

    uVar8 = (param_1 >> 0x10);
    iVar7 = param_1;
    pass1_1008_5784(CONCAT22(0x1050, local_a), (iVar7 + 0x20));
    loop {
        loop {
            lVar10 = pass1_1008_5b12(CONCAT22(0x1050, local_a));
            uVar6 = (lVar10 >> 0x10);
            iVar5 = lVar10;
            if (lVar10 == 0) {
                return;
            }
            if !((((iVar5 + 0x8) == 0) || (param_2 != 0x0 && ((iVar5 + 0x8) != param_2))) || ((iVar5 + 0x8) == 0xf && (param_2 != 0xf))) {
                break;
            }
        }
        uVar2 = (iVar5 + 0xa);
        if ((param_4 == 0) && (param_3 < uVar2)) {
            break;
        }
        bVar9 = param_3 < uVar2;
        param_3 -= uVar2;
        param_4 -= bVar9;
        ppcVar4 = ((iVar7 + 0x20) + 0xc);
        (**ppcVar4)(0x1008, (iVar7 + 0x20));
        uStack6 = 0;
    }
    uVar3 = (iVar5 + 0xc);
    piVar1 = (iVar5 + 0xa);
    *piVar1 = *piVar1 - param_3;
    piVar1 = (iVar5 + 0xc);
    *piVar1 = *piVar1 - param_3 * (uVar3 / uVar2);
    return;
}

pub unsafe fn pass1_1028_6408(mut param_1: u32, param_2: *mut u32) {
    let mut ppcVar1: *mut *mut code;
    let mut bVar2: bool;
    let mut puVar3: *mut u8;
    let mut extraout_DX: u16;
    let mut iVar4: i16;
    let mut iVar5: i16;
    let mut uVar6: u16;
    let mut uVar7: u16;
    let mut local_a: [u8; 0x8] = [0; 0x8];

    uVar6 = (param_1 >> 0x10);
    iVar4 = param_1;
    pass1_1008_5784(CONCAT22(0x1050, local_a), (iVar4 + 0x20));
    bVar2 = false;
    loop {
        puVar3 = local_a;
        pass1_1008_5b12(CONCAT22(0x1050, puVar3));
        iVar5 = param_2;
        uVar7 = (param_2 >> 0x10);
        if ((extraout_DX | puVar3) == 0) {
            break;
        }
        if (((puVar3 + 0x4) == (iVar5 + 0x4)) && ((puVar3 + 0x6) == (iVar5 + 0x6))) {
            if ((puVar3 + 0x8) == (iVar5 + 0x8)) {
                bVar2 = true;
                (puVar3 + 0xa) = (puVar3 + 0xa) + (iVar5 + 0xa);
                (puVar3 + 0xc) = (puVar3 + 0xc) + (iVar5 + 0xc);
            }
        }
    }
    if (bVar2) {
        if (param_2.is_null() == false) {
            ppcVar1 = *param_2;
            (**ppcVar1)(0x1008, param_2, 0x1, param_2, param_2);
            return;
        }
    } else {
        ppcVar1 = ((iVar4 + 0x20) + 0x4);
        (**ppcVar1)(0x1008, (iVar4 + 0x20), param_2);
    }
    return;
}

pub unsafe fn pass1_1028_64d6(mut param_1: u32, mut param_2: u32) -> u16 {
    let mut uVar1: u32;
    let mut BVar2: bool;
    let mut puVar3: *mut u16;
    let mut uVar4: u16;
    let mut in_stack_0000ffc4: HFILE16;
    let mut local_26: u16;
    let mut local_24: u16;
    let mut local_22: u16;
    let mut local_20: u16;
    let mut local_1e: u16;
    let mut local_1c: [u16; 0x6] = [0; 0x6];
    let mut uStack16: u16;
    let mut lStack14: i32;
    let mut local_a: [u8; 0x8] = [0; 0x8];

    BVar2 = write_to_file_1028_b5ec(param_1, param_2);
    if (BVar2 != 0) {
        uVar4 = (param_1 >> 0x10);
        pass1_1008_5784(CONCAT22(0x1050, local_a), (param_1 + 0x20));
        uVar1 = (param_1 + 0x20);
        local_1c[0] = (uVar1 + 0x8);
        puVar3 = local_1c;
        uStack16 = local_1c[0];
        loop {
            BVar2 = write_to_file_1008_7e1c(param_2, CONCAT22(0x1050, puVar3), 0x2, in_stack_0000ffc4);
            if (BVar2 == 0) {
                break;
            }
            lStack14 = pass1_1008_5b12(CONCAT22(0x1050, local_a));
            if (lStack14 == 0) {
                return 0x1;
            }
            local_1e = (lStack14 + 0x4);
            BVar2 = write_to_file_1008_7e1c(
                param_2,
                CONCAT22(0x1050, &local_1e),
                0x2,
                in_stack_0000ffc4,
            );
            if (BVar2 == 0) {
                break;
            }
            local_20 = (lStack14 + 0x6);
            BVar2 = write_to_file_1008_7e1c(
                param_2,
                CONCAT22(0x1050, &local_20),
                0x2,
                in_stack_0000ffc4,
            );
            if (BVar2 == 0) {
                break;
            }
            local_22 = (lStack14 + 0x8);
            BVar2 = write_to_file_1008_7e1c(
                param_2,
                CONCAT22(0x1050, &local_22),
                0x2,
                in_stack_0000ffc4,
            );
            if (BVar2 == 0) {
                break;
            }
            local_24 = (lStack14 + 0xa);
            BVar2 = write_to_file_1008_7e1c(
                param_2,
                CONCAT22(0x1050, &local_24),
                0x2,
                in_stack_0000ffc4,
            );
            if (BVar2 == 0) {
                break;
            }
            local_26 = (lStack14 + 0xc);
            puVar3 = &local_26;
        }
        u16_1050_0310 = 0x6d0;
    }
    return 0x0;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn pass1_1028_65e2(
    mut param_1: i16,
    param_2: *mut u8,
    param_3: *mut astruct_373,
    param_4: *mut HFILE16,
) {
    let mut ppcVar1: *mut *mut code;
    let mut uVar2: u16;
    let mut BVar3: bool;
    let mut uVar4: u16;
    let mut uVar5: u16;
    let mut uVar6: u16;
    let mut local_16: u16;
    let mut paStack20: *mut astruct_99;
    let mut local_10: [u16; 0x2] = [0; 0x2];
    let mut local_c: [u16; 0x3] = [0; 0x3];
    let mut uStack6: u16;
    let mut local_4: u16;

    file_1028_b81a(param_1, param_2, param_3, param_4);
    if (param_1 != 0) {
        BVar3 = read_file_1008_7dee(param_4, CONCAT22(0x1050, &local_4), 0x2);
        if (BVar3 != 0) {
            uStack6 = 0;
            loop {
                if (local_4 <= uStack6) {
                    return;
                }
                paStack20 = pass1_1000_07fc(_PTR_LOOP_1050_68a2);
                uVar5 = (paStack20 >> 0x10);
                uVar2 = paStack20;
                if ((uVar5 | uVar2) == 0) {
                    paStack20 = null_mut();
                } else {
                    paStack20.field0_0x0 = 0x389a;
                    (uVar2 + 0x2) = 0x1008;
                    (uVar2 + 0x4) = 0;
                    (uVar2 + 0x6) = 0;
                    (uVar2 + 0x8) = 0;
                    (uVar2 + 0xa) = 0;
                    (uVar2 + 0xc) = 0;
                    paStack20.field0_0x0 = 0x56ce;
                    (uVar2 + 0x2) = 0x1018;
                }
                BVar3 = read_file_1008_7dee(param_4, CONCAT22(0x1050, local_10), 0x2);
                if (BVar3 == 0) {
                    break;
                }
                BVar3 = read_file_1008_7dee(param_4, CONCAT22(0x1050, local_c), 0x2);
                if (BVar3 == 0) {
                    break;
                }
                BVar3 = read_file_1008_7dee(param_4, CONCAT22(0x1050, &local_16), 0x2);
                if (BVar3 == 0) {
                    break;
                }
                BVar3 = read_file_1008_7dee(param_4, (paStack20 & 0xffff0000 | (paStack20 + 0xa)), 0x2);
                if (BVar3 == 0) {
                    break;
                }
                BVar3 = read_file_1008_7dee(param_4, (paStack20 & 0xffff0000 | (paStack20 + 0xc)), 0x2);
                if (BVar3 == 0) {
                    break;
                }
                (paStack20 + 0x4) = local_10[0];
                uVar4 = switch_1008_72bc(param_4, local_c[0]);
                uVar6 = (paStack20 >> 0x10);
                (paStack20 + 0x6) = uVar4;
                (paStack20 + 0x8) = local_16;
                ppcVar1 = ((param_3 + 0x20) + 0x8);
                (**ppcVar1)();
                uStack6 += 0x1;
            }
        }
        u16_1050_0310 = 0x6d2;
    }
    return;
}

pub unsafe fn pass1_1028_6744(mut param_1: u32, mut param_2: i16) -> u16 {
    let mut uVar1: u16;
    let mut lVar2: i32;
    let mut local_a: [u8; 0x8] = [0; 0x8];

    pass1_1008_5784(CONCAT22(0x1050, local_a), (param_1 + 0x20));
    loop {
        lVar2 = pass1_1008_5b12(CONCAT22(0x1050, local_a));
        uVar1 = (lVar2 >> 0x10);
        if (lVar2 == 0) {
            return 0x0;
        }
        if (lVar2 + 6) == param_2 {
            break;
        }
    }
    return (lVar2 + 0xa);
}

pub unsafe fn pass1_1028_678c(mut param_1: u32, mut param_2: i16) -> u16 {
    let mut uVar1: u16;
    let mut lVar2: i32;
    let mut local_a: [u8; 0x8] = [0; 0x8];

    pass1_1008_5784(CONCAT22(0x1050, local_a), (param_1 + 0x20));
    loop {
        lVar2 = pass1_1008_5b12(CONCAT22(0x1050, local_a));
        uVar1 = (lVar2 >> 0x10);
        if (lVar2 == 0) {
            return 0x0;
        }
        if (lvar2 + 0x8) == param_2 {
            break;
        }
    }
    return (lVar2 + 0xa);
}

// WARNING: Could not reconcile some variable overlaps

pub unsafe fn pass1_1028_67d4(mut param_1: u32) -> u32 {
    let mut uVar1: u16;
    let mut lVar2: i32;
    let mut uStack18: u32;
    let mut local_a: [u8; 0x8] = [0; 0x8];

    pass1_1008_5784(CONCAT22(0x1050, local_a), (param_1 + 0x20));
    uStack18 = 0;
    loop {
        lVar2 = pass1_1008_5b12(CONCAT22(0x1050, local_a));
        if (lVar2 == 0) {
            break;
        }
        uVar1 = (lVar2 + 0xc);
        uStack18 = CONCAT22(
            (uStack18 >> 0x10) + CARRY2(uStack18, uVar1),
            uStack18 + uVar1,
        );
    }
    return uStack18;
}

pub unsafe fn pass1_1028_6822(mut param_1: u32, param_2: *mut u16) -> u16 {
    let mut iVar1: i16;
    let mut uVar2: u32;

    uVar2 = pass1_1028_67d4(param_1);
    iVar1 = (uVar2 >> 0x10);
    *param_2 = uVar2;
    (param_2 + 0x2) = iVar1;
    if ((iVar1 == 0) && (*param_2 < 0x64)) {
        return 0x0;
    }
    return 0x1;
}


pub unsafe fn pass1_1028_68de(param_1: *mut astruct_97, mut param_2: u16, mut param_3: u32) {
    let mut iVar1: *mut astruct_97;
    let mut uVar1: u16;

    struct_op_1028_d1dc(param_1, 0x3e8);
    uVar1 = (param_1 >> 0x10);
    iVar1 = param_1;
    iVar1.field259_0x108 = param_3;
    iVar1.field262_0x10c = param_2;
    param_1.offset_0x0 = 0x6ae2;
    iVar1.segment_0x2 = 0x1028;
    unk_str_op_1000_3d3e(
        (param_1 & 0xffff0000 | ZEXT24(&iVar1.string_0x8)),
        s_SCAddSpew_1050_4fd2,
    );
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address


pub unsafe fn pass1_1028_6af2(param_1: *mut astruct_97, mut param_2: u32, mut param_3: u32) {
    let mut iVar1: *mut astruct_97;
    let mut uVar1: u16;

    struct_op_1028_d1dc(param_1, 0x1387);
    uVar1 = (param_1 >> 0x10);
    iVar1 = param_1;
    iVar1.field259_0x108 = param_3;
    iVar1.field262_0x10c = param_2;
    param_1.offset_0x0 = 0x6e50;
    iVar1.segment_0x2 = 0x1028;
    return;
}

pub unsafe fn pass1_1028_6b2c(mut param_1: u32) -> u16 {
    let mut in_DX: u16;

    pass1_1028_6b40(in_DX, param_1);
    return 0x1;
}

// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn pass1_1028_6b40(mut param_1: u16, mut param_2: u32) {
    let mut ppcVar1: *mut *mut code;
    let mut puVar2: *mut u8;
    let mut uVar3: u16;
    let mut uVar4: u16;
    let mut uVar5: u16;
    let mut puVar6: *mut u16;
    let mut local_36: [u8; 0xe] = [0; 0xe];
    let mut puStack40: *mut u32;
    let mut uStack38: u16;
    let mut uStack36: u16;
    let mut uStack34: u16;
    let mut uStack32: u16;
    let mut uStack30: u16;
    let mut uStack28: u16;
    let mut uStack26: u16;
    let mut local_18: u32;
    let mut uStack20: u16;
    let mut uStack18: u32;
    let mut uStack14: u32;
    let mut pstruct15_10: *mut astruct_15;
    let mut local_6: [u8; 0x2] = [0; 0x2];
    let mut local_4: i16;

    puVar2 = local_6;
    pass1_1028_6daa(
        CONCAT22(puVar2, param_1),
        param_2,
        CONCAT22(0x1050, puVar2),
        CONCAT22(0x1050, &local_4),
        &DAT_1050_1050,
    );
    uVar5 = (param_2 >> 0x10);
    uVar4 = param_2;
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2, (uVar4 + 0x10c));
    pstruct15_10 = CONCAT22(param_1, puVar2);
    ppcVar1 = (pstruct15_10 + 0x24);
    (**ppcVar1)();
    uStack14 = pass1_1028_b58e(pstruct15_10);
    uStack18 = pass1_1028_bb24(pstruct15_10);
    local_18 = (uStack14 + 0xc);
    uStack20 = (uStack14 + 0x10);
    puStack40 = &local_18;
    uStack26 = local_18;
    uStack28 = (local_18 >> 0x10);
    uStack32 = local_18 - 0x1;
    if (uStack32 < 0x0) {
        uStack32 = 0;
    }
    uVar3 = local_4 - 0x1;
    uStack34 = local_18 + 1;
    if (uVar3 < (local_18 + 1)) {
        uStack34 = uVar3;
    }
    uStack36 = uStack28 - 0x1;
    if (uStack36 < 0x0) {
        uStack36 = 0;
    }
    uStack38 = uStack28 + 1;
    if (uVar3 < (uStack28 + 1)) {
        uStack38 = uVar3;
    }
    uStack30 = uStack20;
    puVar6 = pass1_1008_3e54(CONCAT22(0x1050, local_36), uStack20, uStack36, uStack32);
    pass1_1028_6d24(
        (puVar6 >> 0x10),
        uVar4,
        uVar5,
        CONCAT22(0x1050, local_36),
        uStack18,
    );
    puVar6 = pass1_1008_3e54(CONCAT22(0x1050, local_36), uStack30, uStack36, uStack26);
    pass1_1028_6d24(
        (puVar6 >> 0x10),
        uVar4,
        uVar5,
        CONCAT22(0x1050, local_36),
        uStack18,
    );
    puVar6 = pass1_1008_3e54(CONCAT22(0x1050, local_36), uStack30, uStack36, uStack34);
    pass1_1028_6d24(
        (puVar6 >> 0x10),
        uVar4,
        uVar5,
        CONCAT22(0x1050, local_36),
        uStack18,
    );
    puVar6 = pass1_1008_3e54(CONCAT22(0x1050, local_36), uStack30, uStack28, uStack32);
    pass1_1028_6d24(
        (puVar6 >> 0x10),
        uVar4,
        uVar5,
        CONCAT22(0x1050, local_36),
        uStack18,
    );
    puVar6 = pass1_1008_3e54(CONCAT22(0x1050, local_36), uStack30, uStack28, uStack34);
    pass1_1028_6d24(
        (puVar6 >> 0x10),
        uVar4,
        uVar5,
        CONCAT22(0x1050, local_36),
        uStack18,
    );
    puVar6 = pass1_1008_3e54(CONCAT22(0x1050, local_36), uStack30, uStack38, uStack32);
    pass1_1028_6d24(
        (puVar6 >> 0x10),
        uVar4,
        uVar5,
        CONCAT22(0x1050, local_36),
        uStack18,
    );
    puVar6 = pass1_1008_3e54(CONCAT22(0x1050, local_36), uStack30, uStack38, uStack26);
    pass1_1028_6d24(
        (puVar6 >> 0x10),
        uVar4,
        uVar5,
        CONCAT22(0x1050, local_36),
        uStack18,
    );
    puVar6 = pass1_1008_3e54(CONCAT22(0x1050, local_36), uStack30, uStack38, uStack34);
    pass1_1028_6d24(
        (puVar6 >> 0x10),
        uVar4,
        uVar5,
        CONCAT22(0x1050, local_36),
        uStack18,
    );
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn pass1_1028_6d24(
    mut param_1: u16,
    mut param_2: u16,
    mut param_3: u16,
    param_4: *mut u16,
    param_5: i32,
) {
    let mut iVar1: i16;
    let mut ppcVar2: *mut *mut code;
    let mut puVar3: *mut u32;
    let mut uVar4: u16;
    let mut uVar5: u16;
    let mut puVar6: *mut u32;
    let mut bStack27: u8;
    let mut local_a: u32;
    let mut uStack6: u32;

    puVar3 = &local_a;
    pass1_1030_64ce(
        puVar3,
        param_1,
        _PTR_LOOP_1050_5740,
        param_4,
        param_5,
        CONCAT22(0x1050, puVar3),
    );
    uStack6 = *puVar3;
    uVar5 = (puVar3 + 2);
    bStack27 = (uStack6 >> 0x18);
    uVar4 = bStack27;
    if (bStack27 != 0) {
        pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uStack6 & 0xffff | uVar5 << 0x10);
        puVar6 = struct_op_1030_73a8(CONCAT22(uVar5, uVar4), uVar4, uVar5);
        iVar1 = (puVar6 + 0xc);
        if (((iVar1 < 1) || (SBORROW2(iVar1, 1))) || (iVar1 != 0x9 && 0x7 < iVar1 - 0x1 && (iVar1 - 0x9 < 0x6a || (0x6 < iVar1 - 0x73)))) {
            ppcVar2 = (*puVar6 + 0x24);
            (**ppcVar2)();
        }
    }
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn pass1_1028_6daa(
    param_1: *mut astruct_15,
    mut param_2: u32,
    param_3: *mut u16,
    param_4: *mut u16,
    mut param_5: u16,
) {
    let mut uVar1: u32;
    let mut puVar2: *mut u32;
    let mut local_18: u32;
    let mut uStack20: u16;
    let mut iStack18: i16;
    let mut uStack16: u16;
    let mut uStack14: u32;
    let mut uStack10: u32;
    let mut uStack6: u16;
    let mut uStack4: u16;

    pass1_1028_e1ec(_PTR_LOOP_1050_65e2, (param_2 + 0x10c));
    uStack10 = pass1_1028_b4f2(CONCAT22(param_1, (param_1 >> 0x10)));
    uStack6 = (param_1 >> 0x10);
    uStack4 = SUB42(param_1, 0x0);
    uStack16 = (uStack10 >> 0x10);
    uVar1 = (uStack10 + 0x8);
    uStack14 = uVar1;
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uVar1);
    uStack6 = (param_1 >> 0x10);
    uStack4 = SUB42(param_1, 0x0);
    iStack18 = uVar1;
    puVar2 = pass1_1030_5b5c(iStack18, uStack16);
    uStack6 = (param_1 >> 0x10);
    uStack4 = SUB42(param_1, 0x0);
    local_18 = *puVar2;
    uStack20 = (puVar2 + 0x4);
    pass1_1008_3e94(CONCAT22(0x1050, &local_18), param_3, param_4);
    uStack6 = (param_1 >> 0x10);
    uStack4 = SUB42(param_1, 0x0);
    return;
}
