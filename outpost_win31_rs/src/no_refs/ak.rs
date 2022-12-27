
pub fn pass1_1030_c2fa(mut param_1: i16, param_2: *mut u8, mut param_3: u32) {
    let mut uVar1: u32;
    let mut uVar2: u32;
    let mut uVar3: u32;
    let mut iVar4: i16;
    let mut uVar5: u16;
    let mut uVar6: u32;
    let mut uVar7: u16;
    let mut in_register_0000000a: u16;
    let mut paVar8: *mut Struct57;
    let mut unaff_SI: u16;
    let mut uVar9: u16;
    let mut uVar10: u16;
    let mut puVar11: *mut u32;
    let mut paVar12: *mut Struct27;
    let mut in_stack_0000fe4e: u16;
    let mut in_stack_0000ff72: u16;
    let mut in_stack_0000ff78: u16;
    let mut in_stack_0000ff7c: u16;
    let mut uVar13: u16;
    let mut uStack84: u16;
    let mut lStack80: i32;
    let mut iStack56: i16;
    let mut paStack10: *mut astruct_305;
    let mut uStack6: u32;
    let mut iVar5: *mut astruct_698;

    paVar8 = CONCAT22(in_register_0000000a, param_2);
    uVar9 = (param_3 >> 0x10);
    if ((param_3 + 0xc) != 0xb) {
        pass1_1028_bd38(param_2, param_3);
        pass1_1028_e1ec(_PTR_LOOP_1050_65e2, (param_3 + 0x20));
        uVar7 = paVar8;
        uStack6 = CONCAT22(uVar7, param_1);
        iVar4 = param_1;
        pass1_1028_b58e(param_3);
        paStack10 = CONCAT22(paVar8, iVar4);
        uVar1 = (iVar4 + 0x2e);
        puVar11 = mixed_1010_20ba(
            paVar8,
            _u16_1050_0ed0,
            CONCAT22(unaff_SI, 0x2f),
            in_stack_0000fe4e,
            in_stack_0000ff72,
            in_stack_0000ff78,
            in_stack_0000ff7c,
        );
        paVar8 = (paVar8 & 0xffff0000 | puVar11 >> 0x10);
        uVar10 = (uVar1 >> 0x10);
        pass1_1010_ed22(puVar11, (uVar1 + 0x4));
        uVar2 = (param_1 + 0x1f6);
        uVar6 = uVar2;
        pass1_1030_3694(paVar8, uVar2, 0x3, 0x2);
        uVar9 = (uVar2 >> 0x10);
        uVar3 = (uVar1 + 0x1f6);
        pass1_1030_355c(uVar3, uVar6 & 0xffff | paVar8 << 0x10);
        uVar10 = (uVar3 >> 0x10);
        iStack56 = 0;
        loop {
            iVar5 = (iStack56 * 0x2);
            (iVar5 + uVar3 + 0x174) = (iVar5 + uVar2 + 0x174);
            uVar5 = (iVar5 + uVar2 + 0x180);
            uVar6 = uVar5;
            (iVar5 + uVar3 + 0x180) = uVar5;
            iStack56 += 0x1;
            if iStack56 >= 6 {
                break;
            }
        }
        // for (uStack84 = 0x11; uVar5 = uVar6, uStack84 < 0x25; uStack84 += 1)
        uStack84 = 0x11;
        uVar5 = uVar6;
        while uStack84 < 0x25 {
            if (0x0 < (uStack84 * 0x2 + _PTR_LOOP_1050_580e)) {
                empty_1038_540a();
                lStack80 = CONCAT22(paVar8, uVar5);
                uVar9 = (_PTR_LOOP_1050_580e >> 0x10);
                iVar4 = (uStack84 * 0x2 + _PTR_LOOP_1050_580e);
                paVar8 = (iVar4 >> 0x10);
                uVar13 = uStack84;
                if (lStack80 < iVar4) {
                    iVar4 = (uStack84 * 0x2 + _PTR_LOOP_1050_580e);
                    paVar8 = (iVar4 >> 0xf);
                    uVar13 = 0x21;
                }
                pass1_1038_52b8(uStack6, CONCAT22(paVar8, iVar4), uVar13);
                uVar5 = (uStack84 * 0x2 + _PTR_LOOP_1050_580e);
                pass1_1030_7ddc(uVar5, paVar8, paStack10, uVar5, uStack84);
                iVar4 = (_PTR_LOOP_1050_580e + uStack84 * 0x2);
                uVar6 = iVar4;
                pass1_1038_5694(uVar1, iVar4, uStack84);
            }
            uStack84 += 1;
        }
        pass1_1030_7c50(uVar5, paVar8, paStack10, 0x2, 1);
        pass1_1030_7c50(uVar5, paVar8, paStack10, 0x2, 0x2);
        pass1_1030_7c50(uVar5, paVar8, paStack10, 0x2, 0x3);
        pass1_1030_7c50(uVar5, paVar8, paStack10, 0x2, 0x4);
        pass1_1038_44d8(uVar5, paVar8, param_1, uVar7, 0x2, 1);
        pass1_1038_44d8(uVar5, paVar8, param_1, uVar7, 0x2, 0x2);
        pass1_1038_44d8(uVar5, paVar8, param_1, uVar7, 0x2, 0x3);
        pass1_1038_44d8(uVar5, paVar8, param_1, uVar7, 0x2, 0x4);
        paVar12 = mixed_1010_20ba(
            paVar8,
            _u16_1050_0ed0,
            CONCAT22(unaff_SI, 0x2b),
            in_stack_0000fe4e,
            in_stack_0000ff72,
            in_stack_0000ff78,
            in_stack_0000ff7c,
        );
        pass1_1010_043a(paVar12, (param_1 + 0x4), 0x7);
    }
    return;
}


pub fn pass1_1030_c52e(
    mut param_1: i16,
    mut param_2: u16,
    mut param_3: u32,
    param_4: *mut u16,
    mut param_5: u32,
    mut param_6: u32,
) {
    let mut BVar1: bool;
    let mut puVar2: *mut u32;
    let mut paVar3: *mut astruct_92;
    let mut puVar4: *mut u32;
    let mut uVar5: u32;
    let mut uVar6: u16;
    let mut uVar7: u16;
    let mut uVar8: u32;
    let mut uVar9: u16;
    let mut puVar10: *mut u16;
    let mut local_32: *mut astruct_92;
    let mut local_20: u32;
    let mut uStack28: u32;
    let mut puStack24: *mut u32;
    let mut uStack22: u32;
    let mut uStack18: u16;
    let mut uStack16: u16;
    let mut local_c: u32;
    let mut uStack8: u16;
    let mut uStack6: u32;

    uVar9 = (param_3 >> 0x10);
    BVar1 = pass1_1028_c314(
        param_1,
        param_2,
        param_3,
        uVar9,
        param_4,
        param_5,
        (param_5 >> 0x10),
        param_6,
    );
    if (BVar1 != 0) {
        puVar2 = &local_c;
        pass1_1030_64ce(
            puVar2,
            param_2,
            _PTR_LOOP_1050_5740,
            param_4,
            param_6,
            CONCAT22(0x1050, puVar2),
        );
        local_20 = *puVar2;
        local_20._3_1_ = (local_20 >> 0x18);
        uStack8 = local_20._3_1_;
        if (local_20._3_1_ == 0) {
            uStack22 = local_20;
            uStack6 = local_20;
            pass1_1028_c7b6(param_2, param_3, uVar9, param_4, param_6);
            if ((uStack8 != 0x4) && (uStack8 != 0)) {
                uVar8 = pass1_1030_bcae(&local_20, 0x1050);
                uVar6 = (uVar8 >> 0x10);
                pass1_1028_dc52(CONCAT22(0x1050, &local_32), 0x1, 0x0, 0x400);
                loop {
                    paVar3 = &local_32;
                    pass1_1028_e4ec(CONCAT22(0x1050, paVar3));
                    uStack28 = CONCAT22(uVar6, paVar3);
                    uVar7 = uVar6 | paVar3;
                    if (uVar7 == 0) {
                        return;
                    }
                    uVar5 = &paVar3.field6_0x10;
                    uVar8 = param_6;
                    uStack22 = uVar5;
                    puVar10 = param_4;
                    pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uVar5);
                    uStack18 = uVar5;
                    puVar4 = &local_20;
                    uStack16 = uVar7;
                    pass1_1030_bcde(
                        puVar4,
                        0x1050,
                        uVar5 & 0xffff | uVar7 << 0x10,
                        puVar10,
                        uVar8,
                    );
                    if (puVar4 < 0x0) {
                        break;
                    }
                    uVar6 = uVar7;
                    puStack24 = puVar4;
                    if (puVar4 < 0x1f) {
                        PTR_LOOP_1050_50ca = 0x6ae;
                        return;
                    }
                }
                PTR_LOOP_1050_50ca = 0x6af;
                return;
            }
            PTR_LOOP_1050_50ca = 0x6a8;
        }
    }
    return;
}

pub fn pass1_1030_c668(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    pass1_1028_b418(&param_1.address_offset_field_0x0);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1030_c74e(mut param_1: u16, param_2: *mut astruct_15, mut param_3: u32) {
    pass1_1028_b46e(param_1, param_2, param_3);
    (param_2 + 0x20) = 0x70;
    return;
}

pub fn pass1_1030_c76c(param_1: u32) {
    let mut iVar1: i16;
    let mut uVar2: u16;

    uVar2 = (param_1 >> 0x10);
    iVar1 = param_1;
    if (((iVar1 + 0x12) != 0x6) && ((iVar1 + 0x12) != 0x5)) {
        return;
    }
    iVar1 = (iVar1 + 0x20);
    if (iVar1 != 0) {
        if (((iVar1 < 0x70) || (SBORROW2(iVar1, 0x70))) || (0x1 < iVar1 -0x70)) {
            pass1_1028_be2a(param_1);
            return;
        }
    }
    pass1_1028_bdac(param_1, 0x6);
    return;
}


pub fn pass1_1030_c7b0(mut param_1: u32) {
    let mut iVar1: i16;
    let mut uVar2: u32;
    let mut uVar3: u32;
    let mut iVar4: i16;
    let mut iVar5: i16;
    let mut BVar6: bool;
    let mut uVar7: u32;
    let mut extraout_DX: *mut u8;
    let mut puVar8: *mut u8;
    let mut iVar9: i16;
    let mut uVar10: u16;

    uVar10 = (param_1 >> 0x10);
    iVar9 = param_1;
    iVar1 = (iVar9 + 0x20);
    if (iVar1 != 0) {
        iVar4 = iVar1 -0x70;
        iVar5 = iVar4;
        if (((iVar1 < 0x70) || (SBORROW2(iVar1, 0x70)))
            || (iVar5 = iVar1 -0x71, iVar5 != 0x0 && 0x0 < iVar4))
        {
            pass1_1028_b58e((param_1 & 0xffff | uVar10 << 0x10));
            uVar2 = (iVar5 + 0x2e);
            uVar7 = (uVar2 + 0x200);
            puVar8 = extraout_DX;
            pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uVar7);
            uVar3 = uVar7 & 0xffff | ZEXT24(puVar8) << 0x10;
            BVar6 = pass1_1008_c6ae(_u16_1050_06e0, (iVar9 + 0xc), 0x11);
            pass1_1030_23e2(BVar6, puVar8, uVar3, BVar6, (iVar9 + 0x20));
            if (BVar6 != 0) {
                if ((iVar9 + 0x20) == 1) {
                    pass1_1030_25d8(uVar3, 0x64, (iVar9 + 0x20));
                    return;
                }
                (iVar9 + 0x20) = 0x70;
            }
        }
    }
    return;
}

pub fn pass1_1030_c84e(mut param_1: u32, mut param_2: u32) -> BOOL16 {
    let mut BVar1: bool;
    let mut in_stack_0000ffde: HFILE16;
    let mut local_c: [u16; 0x5] = [0; 0x5];

    BVar1 = write_to_file_1028_b5ec(param_1, param_2);
    if (BVar1 != 0) {
        local_c[0] = (param_1 + 0x20);
        BVar1 = write_to_file_1008_7e1c(param_2, CONCAT22(0x1050, local_c), 0x2, in_stack_0000ffde);
        if (BVar1 == 0) {
            u16_1050_0310 = 0x6d0;
            return BVar1;
        }
        BVar1 = 0x1;
    }
    return BVar1;
}

pub fn pass1_1030_c894(
    param_1: BOOL16,
    param_2: *mut u8,
    param_3: *mut astruct_373,
    param_4: *mut HFILE16,
) -> BOOL16 {
    let mut BVar1: bool;
    let mut local_4: u16;

    file_1028_b81a(param_1, param_2, param_3, param_4);
    if (param_1 != 0) {
        BVar1 = read_file_1008_7dee(param_4, CONCAT22(0x1050, &local_4), 0x2);
        if (BVar1 == 0) {
            u16_1050_0310 = 0x6d2;
            return BVar1;
        }
        (param_3 + 0x20) = local_4;
        param_1 = 0x1;
    }
    return param_1;
}

pub fn pass1_1030_c8da(mut param_1: u32, mut param_2: u32, mut param_3: u32) -> u32 {
    let mut uVar1: u32;

    uVar1 = 0;
    if (param_3 == 1) {
        (param_1 + 0x20) = param_2;
    } else {
        uVar1 = FUN_1030_178e();
    }
    return uVar1;
}

pub fn pass1_1030_c91a(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    pass1_1028_b418(&param_1.address_offset_field_0x0);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1030_ca26(mut param_1: u16, param_2: *mut astruct_15, mut param_3: u32) {
    let mut uVar1: u16;
    let mut extraout_DX: u16;
    let mut iVar2: i16;
    let mut uVar3: u16;
    let mut uStack4: u16;

    //   for (uStack4 = 0; iVar2 = param_2, uVar3 = (param_2 >> 0x10), uStack4 < 0xa;
    //       uStack4 += 1)
    uStack4 = 0;
    iVar2 = param_2;
    uVar3 = param_2 >> 0x10;
    while uStack4 < 0xa {
        if (((iVar2 + uStack4 * 0xc + 0x26) == 0x2) || ((iVar2 + uStack4 * 0xc + 0x26) == 1)) {
            (iVar2 + uStack4 * 0xc + 0x26) = 0x4;
            param_1 = uStack4;
        } else {
            uVar1 = uStack4;
            pass1_1028_b58e(param_2);
            iVar2 = uStack4 * 0xc + iVar2;
            pass1_1030_6e9c(CONCAT22(extraout_DX, uVar1), 0x1, (iVar2 + 0x24));
            param_1 = 0;
            (iVar2 + 0x20) = 0;
            (iVar2 + 0x24) = 0;
            (iVar2 + 0x26) = 0;
        }
        uStack4 += 1;
    }
    pass1_1028_b46e(param_1, param_2, param_3);
    return;
}


pub fn pass1_1030_cac2(mut param_1: i16, param_2: *mut u32) {
    let mut uVar1: u32;
    let mut puVar2: *mut u32;
    let mut ppcVar3: *mut *mut code;
    let mut uVar4: u32;
    let mut uVar5: u16;
    let mut uVar6: u32;
    let mut puVar7: *mut u32;
    let mut uVar8: u32;
    let mut extraout_DX: u16;
    let mut extraout_DX_00: u16;
    let mut extraout_DX_01: u16;
    let mut uVar9: u16;
    let mut uVar10: u16;
    let mut uStack34: u32;
    let mut iStack30: i16;
    let mut iStack28: i16;

    pass1_1028_be9e(param_2);
    uVar10 = (param_2 >> 0x10);
    if (((param_2 + 0x12) == 0x5) && (PTR_LOOP_1050_5812.is_null())) {
        PTR_LOOP_1050_5812 = (&PTR_LOOP_1050_0000 + 1);
        pass1_1028_b58e((param_2 & 0xffff | uVar10 << 0x10));
        uVar1 = (param_1 + 0x2e);
        uVar6 = (uVar1 + 0x10);
        uVar10 = extraout_DX;
        pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uVar6);
        puVar2 = (uVar6 + 0x1e);
        ppcVar3 = (*puVar2 + 0x10);
        puVar7 = puVar2;
        (**ppcVar3)(0x1028, puVar2, (uVar6 + 0x20));
        uVar4 = puVar7 & 0xffff | extraout_DX_00 << 0x10;
        iStack28 = 0;
        iStack30 = pass1_1030_d144(param_2);
        uStack34 = 0;
        while (uStack34 < uVar4 && (iStack30 != 0)) {
            ppcVar3 = (*puVar2 + 0x4);
            uVar8 = uVar4;
            (**ppcVar3)(
                0x1028,
                puVar2,
                (puVar2 >> 0x10),
                uStack34,
                (uStack34 >> 0x10),
            );
            uVar5 = uVar8;
            uVar9 = extraout_DX_01 | uVar5;
            if (uVar9 != 0) {
                pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uVar8 & 0xffff | extraout_DX_01 << 0x10);
                uVar5 = (uVar5 + 0xc);
                if ((0x0 < uVar5) && (!SBORROW2(uVar5, 1))) {
                    if (uVar5 != 0x3 && 0x0 < (uVar5 - 0x2)) {
                        //            if (uVar5 != 0x4) goto LAB_1030_cbbc;
                        iStack28 += 0x1;
                    }
                    pass1_1030_6e9c((uVar6 & 0xffff | uVar10 << 0x10), 0x1, uVar5);
                    pass1_1030_d180(param_2, uVar5);
                    iStack30 += -0x1;
                }
            } //
              // LAB_1030_cbbc:
            uStack34 += 0x1;
        }
        while (iStack28 < 0x4) {
            pass1_1030_d180(param_2, 0x4);
            iStack28 = iStack28 + 1;
        }
    }
    return;
}

pub fn pass1_1030_d0a8(mut param_1: u32) -> u16 {
    let mut uVar1: u16;
    let mut uVar2: u16;

    uVar2 = (param_1 >> 0x10);
    uVar1 = (param_1 + 0x98);
    pass1_1030_d56a(param_1 & 0xffff | uVar2 << 0x10);
    return uVar1;
}

pub fn pass1_1030_d230(mut param_1: u32) -> u16 {
    let mut iStack4: i16;

    iStack4 = 0;
    loop {
        if (0x9 < iStack4) {
            return 0x1;
        }
        if ((param_1 + iStack4 * 0xc + 0x20) == 0) {
            break;
        }
        iStack4 += 0x1;
    }
    return 0x0;
}


pub fn pass1_1030_d26c(mut param_1: u32) {
    let mut puVar1: *mut u32;
    let mut uVar2: u32;
    let mut iVar3: i16;
    let mut uVar4: u32;
    let mut extraout_DX: u16;
    let mut iVar5: i16;
    let mut iStack8: i16;

    uVar2 = *_PTR_LOOP_1050_65e2;
    for iStack8 in 0..0xa {
        iVar5 = iStack8 * 0xc + param_1;
        if ((((iVar5 + 0x22) | (iVar5 + 0x20)) != 0)
            && (puVar1 = (iVar5 + 0x20), *puVar1 < uVar2 || *puVar1 == uVar2))
        {
            uVar4 = uVar2;
            pass1_1030_d3b2(uVar2, param_1, param_1, iStack8);
            iVar3 = uVar4;
            if (iVar3 == 0) {
                pass1_1028_b58e(param_1);
                if ((iVar5 + 0x24) == 0x5) {
                    pass1_1038_4900((iVar3 + 0x2e));
                } else {
                    pass1_1030_6e9c(
                        CONCAT22(extraout_DX, iVar3),
                        0x1,
                        (param_1 + iStack8 * 0xc + 0x24),
                    );
                }
                iVar5 = iStack8 * 0xc + param_1;
                (iVar5 + 0x20) = 0;
                (iVar5 + 0x24) = 0;
                (iVar5 + 0x26) = 0;
            }
        }
    }
    return;
}


pub fn pass1_1030_d61c(mut param_1: u32, mut param_2: u32) {
    let mut BVar1: bool;
    let mut iVar2: i16;
    let mut uVar3: u16;
    let mut in_stack_0000ffcc: HFILE16;
    let mut local_1a: u32;
    let mut local_16: *mut u8;
    let mut local_14: u16;
    let mut local_12: [u32; 0x3] = [0; 0x3];
    let mut iStack4: i16;

    BVar1 = write_to_file_1028_b5ec(param_1, param_2);
    if (BVar1 != 0) {
        for iStack4 in 0..0xa {
            uVar3 = (param_1 >> 0x10);
            iVar2 = param_1;
            local_12[0] = (iVar2 + iStack4 * 0xc + 0x20);
            BVar1 = write_to_file_1008_7e1c(
                param_2,
                CONCAT22(0x1050, local_12),
                0x4,
                in_stack_0000ffcc,
            );
            //      if (BVar1 == 0) goto LAB_1030_d701;
            local_14 = (iVar2 + iStack4 * 0xc + 0x24);
            BVar1 = write_to_file_1008_7e1c(
                param_2,
                CONCAT22(0x1050, &local_14),
                0x2,
                in_stack_0000ffcc,
            );
            //      if (BVar1 == 0) goto LAB_1030_d701;
            local_16 = (iVar2 + iStack4 * 0xc + 0x26);
            BVar1 = write_to_file_1008_7e1c(
                param_2,
                CONCAT22(0x1050, &local_16),
                0x2,
                in_stack_0000ffcc,
            );
            //      if (BVar1 == 0) goto LAB_1030_d701;
            local_1a = (iVar2 + iStack4 * 0xc + 0x28);
            BVar1 = write_to_file_1008_7e1c(
                param_2,
                CONCAT22(0x1050, &local_1a),
                0x4,
                in_stack_0000ffcc,
            );
            //      if (BVar1 == 0) goto LAB_1030_d701;
        }
        local_16 = PTR_LOOP_1050_5812;
        BVar1 =
            write_to_file_1008_7e1c(param_2, CONCAT22(0x1050, &local_16), 0x2, in_stack_0000ffcc);
        if (BVar1 != 0) {
            return;
        } //
          // LAB_1030_d701:
        u16_1050_0310 = 0x6d0;
    }
    return;
}


pub fn pass1_1030_d72e(
    mut param_1: i16,
    param_2: *mut u8,
    param_3: *mut astruct_373,
    param_4: *mut HFILE16,
) {
    let mut uVar1: u16;
    let mut BVar2: bool;
    let mut iVar3: i16;
    let mut iStack10: i16;
    let mut local_8: u32;
    let mut local_4: u16;

    file_1028_b81a(param_1, param_2, param_3, param_4);
    if (param_1 == 0) {
        return;
    }
    iStack10 = 0;
    loop {
        if (0x9 < iStack10) {
            // just 0x5812
            if ((0x3 < u16_1050_0312)
                && (
                    BVar2 = read_file_1008_7dee(param_4, &PTR_LOOP_1050_5812, 0x2),
                    BVar2 == 0,
                ))
            {
                u16_1050_0310 = 0x6d2;
                return;
            }
            return;
        }
        BVar2 = read_file_1008_7dee(param_4, CONCAT22(0x1050, &local_8), 0x4);
        if (BVar2 == 0) {
            u16_1050_0310 = 0x6d2;
            return;
        }
        BVar2 = read_file_1008_7dee(param_4, CONCAT22(0x1050, &local_4), 0x2);
        if (BVar2 == 0) {
            break;
        }
        iVar3 = iStack10 * 0xc + param_3;
        (iVar3 + 0x20) = local_8;
        (iVar3 + 0x22) = local_8;
        uVar1 = switch_1008_72bc(param_4, local_4);
        (iVar3 + 0x24) = uVar1;
        if (u16_1050_0312 < 0x2) {
            iVar3 = iStack10 * 0xc + param_3;
            (iVar3 + 0x26) = 0x3;
            (iVar3 + 0x28) = 0;
        } else {
            BVar2 = read_file_1008_7dee(param_4, CONCAT22(0x1050, &local_4), 0x2);
            if (BVar2 == 0) {
                u16_1050_0310 = 0x6d2;
                return;
            }
            BVar2 = read_file_1008_7dee(param_4, CONCAT22(0x1050, &local_8), 0x4);
            if (BVar2 == 0) {
                u16_1050_0310 = 0x6d2;
                return;
            }
            iVar3 = iStack10 * 0xc + param_3;
            (iVar3 + 0x26) = local_4;
            (iVar3 + 0x28) = local_8;
        }
        iStack10 += 0x1;
    }
    u16_1050_0310 = 0x6d2;
    return;
}

pub fn pass1_1030_d868(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    pass1_1028_b418(&param_1.address_offset_field_0x0);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1030_d994(param_1: u32) {
    let mut piVar1: *mut i16;
    let mut uVar2: u32;
    let mut iVar3: i16;
    let mut iVar4: i16;
    let mut uVar5: u16;
    let mut uVar6: u32;

    uVar5 = (param_1 >> 0x10);
    iVar4 = param_1;
    if ((iVar4 + 0x12) != 0x4) {
        return;
    }
    uVar6 = pass1_1028_b4f2(param_1);
    iVar3 = uVar6;
    if ((iVar3 + 0x200) == 0x8000002) {
        uVar2 = (iVar4 + 0x14);
        piVar1 = (uVar2 + 0x94);
        *piVar1 = *piVar1 -0x1;
    } else {
        pass1_1028_cb04(param_1);
        if (iVar3 == 0) {
            return;
        }
        pass1_1030_dace(param_1);
        if (iVar3 == 0) {
            return;
        }
        uVar2 = (iVar4 + 0x14);
        piVar1 = (uVar2 + 0x94);
        *piVar1 = *piVar1 -0x1;
        pass1_1028_c952(param_1);
        pass1_1030_da22(param_1);
    }
    uVar2 = (iVar4 + 0x14);
    if ((uVar2 + 0x94) < 1) {
        pass1_1028_bdac(param_1, 0x5);
    }
    return;
}

pub fn pass1_1030_db72() -> u16 {
    return 0x1;
}

pub fn pass1_1030_db78(mut param_1: u32) {
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    if ((param_1 + 0x12) == 0x6) {
        pass1_1028_bdac((param_1 & 0xffff | uVar1 << 0x10), 0x5);
    }
    return;
}

pub fn pass1_1030_dc02() -> u16 {
    return 0x1;
}

pub fn pass1_1030_dc08(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    pass1_1028_b418(&param_1.address_offset_field_0x0);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1030_e010(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    let mut in_AX: u16;

    pass1_1030_dcf4(in_AX, param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1030_e09e(param_1: *mut astruct_97) -> *mut astruct_97 {
    struct_op_1028_d1dc(param_1, 0x2af7);
    param_1.offset_0x0 = 0xe2ae;
    (param_1 + 0x2) = 0x1030;
    unk_str_op_1000_3d3e(
        (param_1 & 0xffff0000 | (param_1 + 0x8)),
        s_SCAiInput_1050_5972,
    );
    return param_1;
}


pub fn pass1_1030_e0d4(param_1: *mut u8) {
    let mut piVar1: *mut i16;
    let mut uVar2: u16;
    let mut paVar3: *mut astruct_425;
    let mut paVar4: *mut astruct_425;
    let mut uVar5: u16;
    let mut in_register_0000000a: u16;
    let mut iVar7: i16;
    let mut unaff_SI: u16;
    let mut uVar8: u16;
    let mut puVar9: *mut u32;
    let mut in_stack_0000fe7c: u16;
    let mut in_stack_0000ffa0: u16;
    let mut in_stack_0000ffa6: u16;
    let mut in_stack_0000ffaa: u16;
    let mut uStack42: u32;
    let mut local_1c: [u8; 0x8] = [0; 0x8];
    let mut uStack20: u32;
    let mut uStack16: u16;
    let mut uStack14: u32;
    let mut uStack10: u32;
    let mut iStack6: i16;
    let mut uStack4: u16;
    let mut uVar6: u32;

    puVar9 = mixed_1010_20ba(
        CONCAT22(in_register_0000000a, param_1),
        _u16_1050_0ed0,
        CONCAT22(unaff_SI, 0x40),
        in_stack_0000fe7c,
        in_stack_0000ffa0,
        in_stack_0000ffa6,
        in_stack_0000ffaa,
    );
    uStack4 = (puVar9 >> 0x10);
    iStack6 = puVar9;
    uStack10 = pass1_1008_b820(iStack6, uStack4, puVar9);
    uVar2 = uStack10;
    uVar5 = (uStack10 >> 0x10) | uVar2;
    uVar6 = uVar5;
    if (uVar5 != 0) {
        pass1_1028_e1ec(_PTR_LOOP_1050_65e2, 0x8000001);
        uStack14 = CONCAT22(uVar6, uVar2);
        uStack16 = ((uVar2 + 0x154) != 0);
        pass1_1008_5784(CONCAT22(0x1050, local_1c), uStack10);
        loop {
            uVar2 = uVar6;
            paVar3 = local_1c;
            pass1_1008_5b12(CONCAT22(0x1050, paVar3));
            uStack20 = CONCAT22(uVar2, paVar3);
            uVar6 = (uVar2 | paVar3);
            if ((uVar2 | paVar3) == 0) {
                break;
            }
            if (&paVar3.field_0x8 != 0) {
                pass1_1028_e1ec(_PTR_LOOP_1050_65e2, &paVar3.field_0xa);
                paVar4 = paVar3;
                pass1_1038_354a(CONCAT22(uVar6, paVar3), paVar3, uVar6);
                if (paVar4.is_null() == false) {
                    uVar8 = (uStack20 >> 0x10);
                    if (uStack16 == 0) {
                        iVar7 = (uStack20 + 0xe) * 0xc;
                        uStack42 = (iVar7 + 0x58c4);
                        uVar2 = (iVar7 + 0x58c8);
                    } else {
                        iVar7 = (uStack20 + 0xe) * 0xc;
                        uStack42 = (iVar7 + 0x58be);
                        uVar2 = (iVar7 + 0x58c2);
                    }
                    uVar5 = uVar2;
                    pass1_1038_35a8(uVar2, uVar6, ((uStack20 + 0x10) * 0x2 + uStack42), paVar3);
                    if (uVar5 != 0) {
                        uVar8 = (uStack20 >> 0x10);
                        iVar7 = uStack20;
                        piVar1 = (iVar7 + 0x10);
                        *piVar1 = *piVar1 + 1;
                        if (uVar2 <= (iVar7 + 0x10)) {
                            (iVar7 + 0x10) = 0;
                        }
                    }
                }
            }
        }
    }
    return;
}


pub fn pass1_1030_e1f4(mut param_1: u16, param_2: *mut u8, mut param_3: u32) {
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
        *puStack10 = 0xe2ae;
        (param_1 + 0x2) = 0x1030;
    }
    return;
}
