pub unsafe fn pass1_1028_e0a0(param_1: *mut u8, mut param_2: u32, mut param_3: u32) {
    let mut uVar1: u32;
    let mut in_register_0000000a: u16;
    let mut in_stack_0000ff10: u16;

    uVar1 = (param_2 + 0x52);
    pass1_1030_4782(
        CONCAT22(in_register_0000000a, param_1),
        uVar1,
        (uVar1 >> 0x10),
        0x1,
        param_3,
        (param_3 >> 0x10),
        in_stack_0000ff10,
    );
    return;
}

pub unsafe fn pass1_1028_e0bc(
    param_1: u32,
    param_2: *mut Struct57,
    mut param_3: u32,
    mut param_4: i16,
) -> u32 {
    let mut puVar1: *mut u32;
    let mut puVar2: *mut u32;
    let mut uVar5: *mut Struct57;
    let mut puVar3: *mut u32;
    let mut iVar4: i16;
    let mut paVar5: *mut Struct57;
    let mut puVar6: *mut u32;

    mem_op_1000_179c(0x98, param_2);
    uVar5 = param_2;
    puVar3 = param_1;
    paVar5 = uVar5;
    pass1_1030_4bbe(uVar5, (param_3 + 0x52), param_4);
    puVar6 = param_1;
    for iVar4 in 0x26..0 {
        puVar2 = puVar6;
        puVar6 = puVar6 + 1;
        puVar1 = puVar3;
        puVar3 = puVar3 + 1;
        *puVar2 = *puVar1;
    }
    return param_2 << 0x10 | ZEXT24(param_1);
}


pub unsafe fn pass1_1028_e100(param_1: *mut u8, mut param_2: u32, mut param_3: u16) {
    let mut puVar1: *mut u32;
    let mut puVar2: *mut u32;
    let mut uVar4: *mut astruct_311;
    let mut iVar4: i16;
    let mut uVar5: u16;
    let mut in_register_0000000a: u16;
    let mut pSVar6: *mut StructD;
    let mut puVar7: *mut u32;
    let mut puVar8: *mut u32;
    let mut uVar9: u16;
    let mut uStack10: u32;
    let mut uStack6: u32;
    let mut uVar3: u32;

    pSVar6 = CONCAT22(in_register_0000000a, param_1);
    if (_PTR_LOOP_1050_5f2c == 0) {
        PTR_LOOP_1050_5f2c = mem_op_1000_160a(pSVar6);
        PTR_LOOP_1050_5f2e = pSVar6;
    } else {}
    uVar4 = fn_ptr_op_1000_1708(0xae, 0x0, 0x1, PTR_LOOP_1050_5f2c, PTR_LOOP_1050_5f2e);
    uVar3 = ZEXT24(uVar4);
    uStack10 = CONCAT22(PTR_LOOP_1050_5f2e, uVar4);
    uVar5 = PTR_LOOP_1050_5f2e | uVar4;
    if (uVar5 == 0) {
        uStack6 = 0;
    } else {
        uVar4.field164_0xa4 = 0;
        uVar4.field165_0xa8 = 0;
        uVar4.field166_0xac = 0;
        uStack6 = uStack10;
        uVar3 = uStack10;
    }
    puVar7 = uVar3;
    pass1_1030_4c06((param_2 + 0x52), param_3, uVar5);
    uVar9 = (uStack6 >> 0x10);
    puVar8 = uStack6;
    for iVar4 in 0x2b..0 {
        puVar2 = puVar8;
        puVar8 = puVar8 + 1;
        puVar1 = puVar7;
        puVar7 = puVar7 + 1;
        *puVar2 = *puVar1;
    }
    puVar8 = puVar7;
    return;
}

pub unsafe fn pass1_1028_e198(
    mut param_1: u16,
    mut param_2: u16,
    mut param_3: u32,
    param_4: *mut u16,
    param_5: *mut u16,
    mut param_6: u32,
) {
    pass1_1028_e1ec(param_3, param_6);
    pass1_1030_5b1c(CONCAT22(param_2, param_1), param_4, param_5);
    return;
}

pub unsafe fn bad_1028_e1bc() {
    return;
}

pub unsafe fn pass1_1028_e1ec(mut param_1: u32, mut param_2: u32) {
    if (param_2._3_1_ == '\0') {
        return;
    }
    if (param_2._3_1_ == -1) {
        return;
    }
    bad_1030_1312();
    return;
}

/*
Unable to decompile 'pass1_1028_e28a'
Cause:
Low-level Error: Symbol $$undef00000009 extends beyond the end of the address space
*/


pub unsafe fn pass1_1028_e2ac(mut param_1: u32, mut param_2: u16) {
    let mut uStack6: u32;

    uStack6 = (param_1 + (param_2 >> 0x8) * 0x4 + 0x2e);
    (uStack6)();
    return;
}

pub unsafe fn pass1_1028_e2e0(param_1: *mut Struct57, mut param_2: u32, param_3: u8) -> u32 {
    let mut iVar1: i16;
    let mut uVar2: u16;
    let mut uVar3: u32;
    let mut auStack10: [u16; 0x3] = [0; 0x3];
    let mut uStack4: u16;

    uStack4 = param_3;
    if (uStack4 == 0xff) {
        uVar3 = pass1_1028_ebee(0x0, param_1, param_2);
        return uVar3;
    }
    uVar2 = (param_2 >> 0x10);
    iVar1 = param_2 + 0x2e;
    auStack10[0] = (iVar1 + uStack4 * 0x4 + 2);
    uVar3 = auStack10[0];
    uVar2 = (iVar1 + uStack4 * 0x4);
    return CONCAT22(uVar3, uVar2);
}

pub unsafe fn pass1_1028_e332(mut param_1: u32, mut param_2: u16, mut param_3: u16) {
    if ((param_3._1_1_ != 0) && (param_3._1_1_ < 0xa)) {
        pass1_1030_13f6(
            param_2,
            param_3 & 0xff,
            (param_1 + 0xa + param_3._1_1_ * 0x4),
            CONCAT22(param_3, param_2) & 0xffffff,
        );
    }
    return;
}

pub unsafe fn pass1_1028_e372(mut param_1: u32, mut param_2: u16, mut param_3: u16) {
    let mut paVar1: *mut astruct_291;
    let mut uVar2: u32;
    let mut uVar3: u32;
    let mut ppcVar4: *mut *mut code;
    let mut uVar5: u32;
    let mut uVar6: u32;
    let mut uVar7: u16;
    let mut uVar8: u16;
    let mut uStack32: u32;
    let mut uStack16: u32;
    let mut uStack10: u16;

    if (param_3 >> 0x8 != 0xff) {
        paVar1 = (param_1 + 0xa + (param_3 >> 0x8) * 0x4);
        uVar2 = (paVar1 + 0xa);
        uVar7 = param_3 & 0xff;
        uStack16 = CONCAT22(param_3, param_2) & 0xffffff;
        pass1_1028_e1ec(param_1, CONCAT22(param_3, param_2));
        uVar5 = (param_2 + 0x8);
        pass1_1028_e1ec(param_1, uVar5);
        // for (uStack32 = 0x1; uStack10 = (uVar2 >> 0x10), uStack32 < uVar2; uStack32 += 1)
        uStack32 = 1;
        uStack10 = uVar2 >> 0x10;
        while uStack32 < uVar2 {
            if (uStack32 != uStack16) {
                uVar6 = uStack16;
                bad_1030_1312();
                uVar8 = uStack10 | uVar6;
                if (uVar8 != 0) {
                    uVar3 = (uVar6 + 0x4);
                    pass1_1030_13f6(uVar3, uVar8, paVar1, uStack32);
                    ppcVar4 = ((uVar5 & 0xffff | uVar7 << 0x10) + 0x18);
                    (**ppcVar4)(0x1030, (uVar5 & 0xffff), uVar7, uVar3);
                }
            }
            uStack32 += 1;
        }
    }
    return;
}

pub unsafe fn pass1_1028_e44a(mut param_1: u32, param_2: i32) {
    let mut paVar1: *mut astruct_291;
    let mut paVar2: *mut astruct_291;
    let mut uVar3: u32;
    let mut uVar4: u16;
    let mut uVar5: u16;
    let mut uVar6: u32;
    let mut uVar7: u16;
    let mut uVar8: u16;
    let mut uStack18: u32;
    let mut uStack12: u16;

    pass1_1028_e372(param_1, param_2, (param_2 >> 0x10));
    uVar8 = (param_1 >> 0x10);
    paVar1 = (param_1 + 0x26);
    paVar2 = (param_1 + 0x1e);
    uVar3 = (paVar2 + 0xa);
    //   for (uStack18 = 0x1; uStack12 = (uVar3 >> 0x10), uStack18 < uVar3; uStack18 += 1)
    uStack18 = 1;
    uStack12 = uVar3 >> 0x10;
    while uStack18 < uVar3 {
        uVar6 = uVar3;
        bad_1030_1312();
        uVar5 = uVar6;
        if (((uStack12 | uVar5) != 0) && ((uVar5 + 0x8) != param_2)) {
            uVar8 = (uVar5 + 0x16);
            uVar5 = (uVar5 + 0x18);
            uVar7 = uVar5 & 0xff;
            uVar4 = pass1_1030_13f6(uVar8, uVar7, paVar1, CONCAT22(uVar5, uVar8) & 0xffffff);
            pass1_1030_13f6(uVar4, uVar7, paVar2, uStack18);
        }
        uStack18 += 1;
    }
    return;
}

pub unsafe fn pass1_1028_e4ec(param_1: *mut astruct_92) {
    let mut puVar1: *mut u32;
    let mut plVar2: *mut i32;
    let mut uVar3: u32;
    let mut uVar4: u16;
    let mut uVar5: u32;
    let mut in_DX: u16;
    let mut iVar6: i16;
    let mut uVar7: u16;

    uVar5 = 0;
    uVar7 = (param_1 >> 0x10);
    iVar6 = param_1;
    if ((iVar6 + 0x10) == 0) {
        loop {
            if ((iVar6 + 0x8) == 0) {
                return;
            }
            plVar2 = (iVar6 + 0x8);
            *plVar2 = *plVar2 - 0x1;
            bad_1030_1312();
            in_DX |= uVar5;
            if in_DX != 0 {
                break;
            }
        }
    } else {
        loop {
            uVar3 = (iVar6 + 0xc);
            puVar1 = (iVar6 + 0x8);
            if (uVar3 <= *puVar1 && *puVar1 != uVar3) {
                return;
            }
            uVar4 = (iVar6 + 0x8);
            plVar2 = (iVar6 + 0x8);
            *plVar2 = *plVar2 + 1;
            bad_1030_1312();
            in_DX |= uVar4;
            if in_DX != 0 {
                break;
            }
        }
    }
    return;
}

pub unsafe fn write_file_fn_1028_e56c(
    param_1: u16,
    mut param_2: u16,
    mut param_3: u16,
    param_4: *mut u8,
    mut param_5: u32,
) -> u16 {
    let mut ppcVar1: *mut *mut code;
    let mut paVar2: *mut astruct_92;
    let mut BVar3: bool;
    let mut extraout_DX: u16;
    let mut in_stack_0000ffbe: HFILE16;
    let mut local_2a: [u32; 0x3] = [0; 0x3];
    let mut puStack28: *mut u32;
    let mut uStack24: u32;
    let mut local_14: *mut astruct_92;

    pass1_1028_dc52(CONCAT22(0x1050, &local_14), 0x1, param_5, (param_5 >> 0x10));
    uStack24 = 0;
    loop {
        paVar2 = &local_14;
        pass1_1028_e4ec(CONCAT22(0x1050, paVar2));
        puStack28 = CONCAT22(param_1, paVar2);
        param_1 |= paVar2;
        if (param_1 == 0) {
            break;
        }
        uStack24 += 0x1;
    }
    local_2a[0] = uStack24;
    BVar3 = write_to_file_1008_7e1c(param_4, CONCAT22(0x1050, local_2a), 0x4, in_stack_0000ffbe);
    if (BVar3 == 0) {
        u16_1050_0310 = 0x6d0;
    } else {
        local_14.field4_0x8 = local_14.field5_0xc;
        local_14.field4_0x8 = local_14.field5_0xc;
        if (local_14.field6_0x10 != 0) {
            local_14.field4_0x8 = 0x1;
            local_14.field5_0xc = 0;
            local_14.field4_0x8 = local_14.field5_0xc;
        }
        loop {
            paVar2 = &local_14;
            pass1_1028_e4ec(CONCAT22(0x1050, paVar2));
            puStack28 = CONCAT22(local_14.field5_0xc, paVar2);
            if ((local_14.field5_0xc | paVar2) == 0) {
                return 0x0;
            }
            ppcVar1 = (*puStack28 + 0xc);
            (**ppcVar1)(0x1008, paVar2, local_14.field5_0xc);
            local_2a[0] = local_2a[0] & 0xffff0000 | ZEXT24(paVar2);
            local_14.field5_0xc = extraout_DX;
            param_1 = extraout_DX;
            if paVar2.is_null() {
                break;
            }
        }
    }
    return param_1;
}

// WARNING: Instruction at (ram,0x10287af1) overlaps instruction at (ram,0x10287af0)
//
// WARNING: Control flow encountered bad instruction data
// WARNING: Removing unreachable block (ram,0x1028e2f6)


// WARNING: Restarted to delay deadcode elimination for space: ram
pub unsafe fn pass1_1028_e628(
    mut param_1: u32,
    mut param_2: u16,
    mut param_3: u16,
    mut param_4: i16,
    mut param_5: i16,
) {
    let mut pcVar1: *mut c_char;
    let mut piVar2: *mut i16;
    let mut uVar3: u32;
    let mut cVar4: u8;
    let mut uVar5: u32;
    let mut uVar6: u32;
    let mut lVar7: i32;
    let mut ppcVar8: *mut *mut code;
    let mut uVar9: u16;
    let mut puVar10: *mut u8;
    let mut BVar11: bool;
    let mut uVar12: u16;
    let mut uVar13: u32;
    let mut iVar14: i16;
    let mut puVar15: *mut u8;
    let mut puVar16: *mut u16;
    let mut paVar17: *mut Struct57;
    let mut in_EDX: u32;
    let mut uVar19: u32;
    let mut uVar18: *mut astruct_348;
    let mut paVar20: *mut astruct_349;
    let mut uVar21: u16;
    let mut uVar20: *mut astruct_349;
    let mut unaff_SI: u16;
    let mut unaff_DI: u16;
    let mut unaff_ES: u16;
    let mut uVar22: u16;
    let mut uVar23: u16;
    let mut bVar24: bool;
    let mut bVar25: bool;
    let mut paVar26: *mut Struct27;
    let mut puVar27: *mut u32;
    let mut paVar28: *mut astruct_180;
    let mut puVar29: *mut u16;
    let mut puVar30: *mut u32;
    let mut in_stack_0000fe64: u16;
    let mut in_stack_0000fe6a: u16;
    let mut local_154: u16;
    let mut uStack338: u16;
    let mut local_14c: u16;
    let mut uStack330: u16;
    let mut in_stack_0000ff88: u16;
    let mut in_stack_0000ff8e: u16;
    let mut in_stack_0000ff92: u16;
    let mut in_stack_0000ff94: u16;
    let mut in_stack_0000ff98: u16;
    let mut uVar31: u8;
    let mut uVar32: u8;
    let mut uVar33: u8;
    let mut uVar34: u8;
    let mut uVar35: u16;
    let mut uVar36: u8;
    let mut uVar37: u8;
    let mut iVar38: i16;
    let mut uVar39: u16;
    let mut in_stack_0000ffca: u16;
    let mut in_stack_0000ffcc: u16;
    let mut local_30: u32;
    let mut uStack44: u16;
    let mut uStack42: u16;
    let mut uStack40: u16;
    let mut uStack38: u16;
    let mut puStack36: *mut u32;
    let mut puStack32: *mut u8;
    let mut uStack30: u16;
    let mut uStack28: u16;
    let mut uStack26: u16;
    let mut ppuStack24: *mut *mut u8;
    let mut local_16: *mut u8;
    let mut local_14: *mut u8;
    let mut local_12: i16;
    let mut local_10: *mut u8;
    let mut puStack14: *mut u8;
    let mut pcStack12: *mut code;
    let mut puStack10: *mut u32;
    let mut local_6: *mut u32;

    uVar39 = (in_EDX >> 0x10);
    uVar23 = SUB42(&DAT_1050_1050, 0x0);
    uVar21 = unaff_SI;
    BVar11 = read_file_1008_7dee(CONCAT22(param_3, param_2), CONCAT22(0x1050, &local_6), 0x4);
    if (BVar11 == 0) {
        u16_1050_0310 = 0x6d2;
        return;
    }
    puStack10 = null_mut();
    if ((param_4 == 0) && ((param_5 - 0x100) == '\0')) {
        puVar16 = (param_5 - 0x100 >> 0x7);
        paVar17 = CONCAT22(uVar39, puVar16);
        if (PTR_LOOP_1050_000e < puVar16) {
            return;
        }
        uVar35 = (param_1 >> 0x10);
        uVar20 = param_1;
        match *puVar16 {
            0 => {
                pass1_1030_145a(uVar20.field14_0xe, local_6);
                uStack28 = 0;
                uStack26 = 0;
                while (CONCAT22(uStack26, uStack28) < local_6) {
                    puVar30 = local_6;
                    mem_op_1000_179c(0x14, paVar17);
                    puStack32 = puVar30;
                    uStack30 = paVar17;
                    uVar19 = paVar17 & 0xffff0000;
                    if ((uStack30 | puStack32) == 0) {
                        puVar16 = null_mut();
                        uVar19 = paVar17 & 0xffff0000;
                    } else {
                        puVar29 = pass1_1030_5d0a((puVar30 & 0xffff | paVar17 << 0x10));
                        uVar19 = uVar19 & 0xffff0000 | puVar29 >> 0x10;
                        puVar16 = puVar29;
                    }
                    local_16 = uVar19;
                    ppcVar8 = (CONCAT22(local_16, puVar16) + 0x10);
                    ppuStack24 = puVar16;
                    (**ppcVar8)();
                    if (puVar16.is_null()) {
                        return;
                    }
                    uVar6 = (ppuStack24 + 2);
                    uVar5 = ZEXT24(ppuStack24[0x3]);
                    puStack14 = uVar6;
                    pcStack12 = (uVar6 >> 0x10);
                    paVar17 = (uVar19 & 0xffff0000 | uVar5 & 0xffff00ff);
                    pass1_1030_14b4(
                        uVar20.field14_0xe,
                        ppuStack24,
                        local_16,
                        uVar6 & 0xffff | (uVar5 & 0xff) << 0x10,
                    );
                    lVar7 = CONCAT22(uStack26, uStack28) + 1;
                    uStack28 = lVar7;
                    uStack26 = (lVar7 >> 0x10);
                }
            }
            //   break;
            0x1 => {
                // WARNING: Bad instruction - Truncating control flow here
                halt_baddata();
            }
            0x2 => {
                pass1_1030_145a(uVar20.field15_0x12, local_6);
                uStack40 = 0;
                uStack38 = 0;
                while (CONCAT22(uStack38, uStack40) < local_6) {
                    puVar30 = local_6;
                    mem_op_1000_179c(0x1c, paVar17);
                    puStack32 = puVar30;
                    uStack30 = paVar17;
                    uVar19 = paVar17 & 0xffff0000 | (uStack30 | puStack32);
                    if ((uStack30 | puStack32) == 0) {
                        uVar9 = 0;
                        uVar19 = paVar17 & 0xffff0000;
                    } else {
                        uVar9 = puStack32;
                        pass1_1030_2958((puVar30 & 0xffff | paVar17 << 0x10), uVar19);
                    }
                    puStack36 = CONCAT22(uVar19, uVar9);
                    ppcVar8 = (*puStack36 + 0x10);
                    (**ppcVar8)();
                    if (uVar9 == 0) {
                        return;
                    }
                    uVar21 = (puStack36 >> 0x10);
                    uVar18 = puStack36;
                    uVar6 = &uVar18.field_0x4;
                    uVar5 = uVar18.field6_0x6;
                    puStack14 = uVar6;
                    pcStack12 = (uVar6 >> 0x10);
                    paVar17 = (uVar19 & 0xffff0000 | uVar5 & 0xffff00ff);
                    pass1_1030_14b4(
                        uVar20.field15_0x12,
                        uVar18,
                        uVar21,
                        uVar6 & 0xffff | (uVar5 & 0xff) << 0x10,
                    );
                    lVar7 = CONCAT22(uStack38, uStack40) + 1;
                    uStack40 = lVar7;
                    uStack38 = (lVar7 >> 0x10);
                }
            }
            //   break;
            0x3 => {
                puVar10 = &uVar20.field_0x114;
                pass1_1028_e2ac(_PTR_LOOP_1050_65e2, 0x500);
                puVar15 = paVar17;
                local_16 = puVar10;
                local_14 = puVar15;
                pass1_1030_61fe(
                    puVar10,
                    puVar15,
                    _PTR_LOOP_1050_5740,
                    CONCAT22(puVar15, puVar10),
                    param_1 & 0xffff0000 | ZEXT24(&uVar20.field_0x114),
                    &uVar20.field_0x108,
                );
                if ((uVar20.field250_0x11a == 0xa) || (uVar20.field250_0x11a == 0x37)) {
                    if (uVar20.field250_0x11a == 0x37) {
                        puVar15 = (&uVar20.field253_0x11e + 2);
                        uVar19 = uVar20.field242_0x10c;
                        uStack42 = uVar19;
                        uStack40 = (uVar19 >> 0x10);
                        (uVar20.field253_0x11e + 0x20) = uVar19;
                    }
                    puVar10 = &uVar20.field_0x114;
                    pass1_1028_e2ac(_PTR_LOOP_1050_65e2, 0x400);
                    uVar20.field242_0x10c = puVar10;
                    (&uVar20.field242_0x10c + 0x2) = puVar15;
                    pass1_1018_0196(
                        puVar10,
                        puVar15,
                        local_6,
                        CONCAT22(puVar15, &uVar20.field242_0x10c),
                        &uVar20.field_0x108,
                    );
                    if (uVar20.field250_0x11a == 0xa) {
                        pass1_1010_ed22(local_6, uVar20.field242_0x10c);
                    }
                }
                pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uVar20.field242_0x10c);
                uVar20.field243_0x110 = puVar10;
                (&uVar20.field243_0x110 + 0x2) = puVar15;
                uStack26 = puVar15 | &uVar20.field243_0x110;
                if (uStack26 != 0) {
                    ppcVar8 = (*uVar20.field243_0x110 + 0x8);
                    (**ppcVar8)();
                }
                pass1_1028_e1ec(_PTR_LOOP_1050_65e2, CONCAT22(local_14, local_16));
                ppuStack24 = puVar15;
                pass1_1030_73ee(puVar15, CONCAT22(puVar15, uStack26), uVar20.field242_0x10c);
                BVar11 = pass1_1008_c6ae(_u16_1050_06e0, uVar20.field250_0x11a, 0x31);
                if ((BVar11 == 0) && (uVar20.field254_0x122 == 0)) {
                    uVar23 = ((uStack26 + 0xc) >> 0x10);
                    if ((uStack26 + 0x10) < 1) {
                        uVar39 = 0x5;
                    } else {
                        uVar39 = 0x6;
                    }
                    (uStack26 + 0x14) = uVar39;
                    puVar15 = ppuStack24;
                }
                uVar13 = (uStack26 + 0x16);
                uStack30 = uVar13;
                uStack28 = (uVar13 >> 0x10);
                pass1_1028_e1ec(&PTR_LOOP_1050_65e2, uVar13);
                puStack36 = CONCAT22(uVar13, puStack36);
                puStack32 = puVar15;
                if (CONCAT22(uStack28, uStack30) != 0) {
                    struct_1030_e4fa(CONCAT22(0x1050, &local_14c), CONCAT22(uStack28, uStack30));
                    fn_ptr_1030_835a(&u16_1050_5748, CONCAT22(0x1050, &local_14c));
                    local_14c = 0x389a;
                    uStack330 = 0x1008;
                }
                ppcVar8 = (*uVar20.field253_0x11e + 0x4);
                (**ppcVar8)();
                puVar30 = uVar20.field253_0x11e;
                pass1_1030_7e5a(puVar15, CONCAT22(ppuStack24, uStack26), (puVar30 + 0x4));
                return;
            }
            0x4 => {
                pass1_1030_145a(uVar20.field16_0x16, local_6);
                uStack40 = 0;
                uStack38 = 0;
                while (CONCAT22(uStack38, uStack40) < local_6) {
                    puVar30 = local_6;
                    mem_op_1000_179c(0x1e, paVar17);
                    puStack32 = puVar30;
                    uStack30 = paVar17;
                    uVar19 = paVar17 & 0xffff0000;
                    if ((uStack30 | puStack32) == 0) {
                        iVar14 = 0;
                        uVar19 = paVar17 & 0xffff0000;
                    } else {
                        puVar29 = pass1_1030_560e((puVar30 & 0xffff | paVar17 << 0x10));
                        uVar19 = uVar19 & 0xffff0000 | puVar29 >> 0x10;
                        iVar14 = puVar29;
                    }
                    puStack36 = CONCAT22(uVar19, iVar14);
                    ppcVar8 = (*puStack36 + 0x10);
                    (**ppcVar8)();
                    if (iVar14 == 0) {
                        return;
                    }
                    uVar39 = (puStack36 >> 0x10);
                    uVar6 = (puStack36 + 0x4);
                    puStack14 = uVar6;
                    pcStack12 = (uVar6 >> 0x10);
                    uVar5 = (puStack36 + 0x10);
                    uStack28 = uVar5;
                    uStack26 = (uVar5 >> 0x10);
                    pass1_1030_6222(uStack28, uVar19, _PTR_LOOP_1050_5740, 0x0, uVar5, uVar6);
                    paVar17 = (uVar19 & 0xffff0000 | ZEXT24(pcStack12) & 0xffff00ff);
                    pass1_1030_14b4(
                        uVar20.field16_0x16,
                        puStack36,
                        (puStack36 >> 0x10),
                        CONCAT22((ZEXT24(pcStack12) & 0xffff00ff), puStack14),
                    );
                    lVar7 = CONCAT22(uStack38, uStack40) + 1;
                    uStack40 = lVar7;
                    uStack38 = (lVar7 >> 0x10);
                }
            }
            //   break;
            0x5 => {
                *puVar16 = 0x5280;
                puVar16[0x1] = 0x1028;
                return;
            }
            0x6 => {
                pass1_1030_145a(uVar20.field17_0x1a, local_6);
                //   for (local_30 = null_mut(); local_30 < local_6; local_30 = (local_30 + 1))
                local_30 = null_mut();
                while local_30 < local_6 {
                    puVar30 = local_6;
                    mem_op_1000_179c(0x21e, paVar17);
                    puStack32 = puVar30;
                    uStack30 = paVar17;
                    uVar19 = paVar17 & 0xffff0000 | (uStack30 | puStack32);
                    if ((uStack30 | puStack32) == 0) {
                        uVar9 = 0;
                        uVar19 = paVar17 & 0xffff0000;
                    } else {
                        uVar9 = puStack32;
                        pass1_1038_30aa((puVar30 & 0xffff | paVar17 << 0x10), uVar19);
                    }
                    uStack42 = uVar19;
                    ppcVar8 = (CONCAT22(uStack42, uVar9) + 0x10);
                    uStack44 = uVar9;
                    (**ppcVar8)();
                    if (uVar9 == 0) {
                        return;
                    }
                    uVar6 = (uStack44 + 0x4);
                    uVar5 = (uStack44 + 0x6);
                    puStack14 = uVar6;
                    pcStack12 = (uVar6 >> 0x10);
                    paVar17 = (uVar19 & 0xffff0000 | uVar5 & 0xffff00ff);
                    pass1_1030_14b4(
                        uVar20.field17_0x1a,
                        uStack44,
                        uStack42,
                        uVar6 & 0xffff | (uVar5 & 0xff) << 0x10,
                    );
                    local_30 += 1;
                }
            }
            //   break;
            _ => {
                pass1_1030_145a(uVar20.field18_0x1e, local_6);
                pass1_1030_66de(_PTR_LOOP_1050_5740, local_6);
                local_30 = null_mut();
                loop {
                    if (local_6 <= local_30) {
                        pass1_1030_154c();
                        pass1_1030_6740(_PTR_LOOP_1050_5740);
                        return;
                    }
                    local_14 = _PTR_LOOP_1050_5744;
                    local_12 = (_PTR_LOOP_1050_5744 >> 0x10);
                    paVar28 = pass1_1000_07fc(_PTR_LOOP_1050_5744);
                    uStack30 = (paVar28 >> 0x10);
                    puStack32 = paVar28;
                    uVar9 = uStack30 | puStack32;
                    if (uVar9 == 0) {
                        uVar12 = 0;
                        uVar9 = 0;
                    } else {
                        uVar12 = puStack32;
                        pass1_1030_67cc(paVar28);
                    }
                    ppcVar8 = (CONCAT22(uVar9, uVar12) + 0x10);
                    uStack44 = uVar12;
                    uStack42 = uVar9;
                    (**ppcVar8)();
                    if (uVar12 == 0) {
                        break;
                    }
                    uVar19 = (uStack44 + 0x4);
                    puStack14 = uVar19;
                    pcStack12 = (uVar19 >> 0x10);
                    lVar7 = (uStack44 + 0x8);
                    uStack40 = lVar7;
                    uStack38 = (lVar7 >> 0x10);
                    puStack36 = (puStack36 & 0xffff0000 | ZEXT24(&stack0xffca));
                    pass1_1030_671c(
                        &stack0xffca,
                        uStack42,
                        _PTR_LOOP_1050_5740,
                        uVar19,
                        CONCAT22(0x1050, &stack0xffca),
                        lVar7,
                    );
                    pass1_1030_14b4(
                        uVar20.field18_0x1e,
                        uStack44,
                        uStack42,
                        CONCAT22(pcStack12, puStack14) & 0xffffff,
                    );
                    local_30 = (local_30 + 1);
                }
                return;
            }
            0x9 => {
                local_6 = (local_6 & 0xffff);
                pcStack12 = uVar20.field22_0x2e;
                puStack10 = uVar20.field23_0x30;
                (*pcStack12)();
                return;
            }
            0xa => {
                pass1_1030_145a(uVar20.field19_0x22, local_6);
                uVar39 = 0;
                uVar23 = 0;
                while (CONCAT22(uVar23, uVar39) < local_6) {
                    puVar30 = local_6;
                    mem_op_1000_179c(0xe, paVar17);
                    puStack32 = puVar30;
                    uStack30 = paVar17;
                    uVar19 = paVar17 & 0xffff0000;
                    if ((uStack30 | puStack32) == 0) {
                        iVar14 = 0;
                        uVar19 = paVar17 & 0xffff0000;
                    } else {
                        puVar29 = pass1_1028_b204((puVar30 & 0xffff | paVar17 << 0x10));
                        uVar19 = uVar19 & 0xffff0000 | puVar29 >> 0x10;
                        iVar14 = puVar29;
                    }
                    local_30 = CONCAT22(uVar19, iVar14);
                    ppcVar8 = (*local_30 + 0x10);
                    (**ppcVar8)();
                    if (iVar14 == 0) {
                        return;
                    }
                    uVar22 = (local_30 >> 0x10);
                    uVar21 = local_30;
                    uVar6 = (uVar21 + 0x4);
                    uVar5 = (uVar21 + 0x6);
                    puStack14 = uVar6;
                    pcStack12 = (uVar6 >> 0x10);
                    paVar17 = (uVar19 & 0xffff0000 | uVar5 & 0xffff00ff);
                    pass1_1030_14b4(
                        uVar20.field19_0x22,
                        uVar21,
                        uVar22,
                        uVar6 & 0xffff | (uVar5 & 0xff) << 0x10,
                    );
                    lVar7 = CONCAT22(uVar23, uVar39) + 1;
                    uVar39 = lVar7;
                    uVar23 = (lVar7 >> 0x10);
                }
            }
            //   break;
            0xb => {
                if (puVar16 < (&PTR_LOOP_1050_000e + 1)) {
                    pcVar1 = (unaff_SI + 0x23);
                    cVar4 = *pcVar1;
                    *pcVar1 = *pcVar1 << 0x6;
                    uVar39 = 0x2b;
                    piVar2 = (puVar16 + unaff_SI);
                    *piVar2 = *piVar2 + (-0x6600 - ((cVar4 << 0x5) < '\0'));
                } else {
                    uVar39 = 0x7af0;
                    pass1_1028_780c(
                        uVar21,
                        unaff_DI,
                        CONCAT22(in_stack_0000ffcc, in_stack_0000ffca),
                    );
                    //        if (param_4 == 0) goto code_r0x10287b17;
                }
                uVar31 = 0;
                uVar32 = 0x4;
                iVar14 = 0x1d;
                paVar26 = mixed_1010_20ba(
                    paVar17,
                    _u16_1050_0ed0,
                    0x1002b,
                    in_stack_0000fe64,
                    in_stack_0000ff88,
                    in_stack_0000ff8e,
                    in_stack_0000ff92,
                );
                paVar17 = (paVar17 & 0xffff0000 | paVar26 >> 0x10);
                param_4 = paVar26;
                pass1_1010_043a(
                    paVar26,
                    CONCAT13(uVar32, CONCAT12(uVar31, (paVar26 >> 0x10))),
                    iVar14,
                );
                // code_r0x10287b17:
                pass1_1028_e1ec(_PTR_LOOP_1050_65e2, 0x4000002);
                pass1_1028_780c(uVar20, uVar35, CONCAT22(paVar17, param_4));
                puStack10 = mixed_1010_20ba(
                    paVar17,
                    _u16_1050_0ed0,
                    CONCAT22(uVar39, 0x2),
                    in_stack_0000fe6a,
                    in_stack_0000ff8e,
                    in_stack_0000ff94,
                    in_stack_0000ff98,
                );
                pcStack12 = PTR_LOOP_1050_13ae;
                if (0x2 < PTR_LOOP_1050_13ae) {
                    puVar27 = mixed_1010_20ba(
                        (paVar17 & 0xffff0000 | puStack10 >> 0x10),
                        _u16_1050_0ed0,
                        CONCAT22(uVar39, 0x2f),
                        in_stack_0000fe6a,
                        in_stack_0000ff8e,
                        in_stack_0000ff94,
                        in_stack_0000ff98,
                    );
                    uVar31 = SUB41(puVar27, 0x0);
                    uVar32 = (puVar27 >> 0x8);
                    uVar33 = 0x1;
                    uVar34 = 0;
                    uVar39 = (puVar27 >> 0x10);
                    while (CONCAT11(uVar34, uVar33) < 0x9) {
                        uVar23 = uVar39;
                        if ((CONCAT11(uVar32, uVar31) + 0x34 + CONCAT11(uVar34, uVar33) * 0x4) == local_6) {
                            uVar9 = 0x1;
                            local_30 = CONCAT22(local_30, 1);
                            uVar33 = 0xd7;
                            uVar34 = 0x7b;
                            pass1_1008_612e(0x1, 0x1, 0x64);
                            puVar16 = (CONCAT11(uVar34, uVar33) - 0x7);
                            if (puVar16.is_null()) {
                                bVar25 = SBORROW2(uVar9, 0x32);
                                iVar14 = uVar9 - 0x32;
                                bVar24 = uVar9 == 0x32; //
                                // LAB_1028_7b74:
                                if (!bVar24 && bVar25 == (iVar14 < 0x0)) {
                                    local_30 = (local_30 & 0xffff0000);
                                }
                            } else {
                                puVar16 = (CONCAT11(uVar34, uVar33) - 0x8);
                                if (puVar16.is_null()) {
                                    bVar25 = SBORROW2(uVar9, 0x19);
                                    iVar14 = uVar9 - 0x19;
                                    bVar24 = iVar14 == 0;
                                    // TODO: goto LAB_1028_7b74;
                                }
                            }
                            uStack30 = uVar9;
                            if (local_30 != 0) {
                                pass1_1028_90e6(
                                    CONCAT13(0x10, CONCAT12(0x50, &local_154)),
                                    CONCAT11(uVar34, uVar33),
                                );
                                puVar16 = &local_154;
                                uVar39 = 0x1008;
                                uVar31 = 0xc;
                                uVar32 = 0x7c;
                                fn_ptr_1030_835a(_u16_1050_5748, CONCAT22(0x1050, puVar16));
                                local_154 = 0x389a;
                                uStack338 = 0x1008;
                            }
                            uVar36 = 0;
                            uVar37 = 0;
                            uVar33 = 0x23;
                            uVar34 = 0x7c;
                            pass1_1008_612e(puVar16, 0x0, 0xa);
                            ppuStack24 = puVar16;
                            if (CONCAT11(uVar34, uVar33) == 0x7) {
                                iVar38 = 0x7;
                                puVar16 = puVar16 + 0x37;
                                iVar14 = puVar16 >> 0xf;
                            } else {
                                uVar23 = uVar39;
                                //              if (CONCAT11(uVar34,uVar33) != 0x8) goto LAB_1028_7ba0;
                                iVar38 = 0x8;
                                puVar16 = puVar16 + 0x32;
                                iVar14 = (puVar16 >> 0xf) + (0xff9b < puVar16);
                            }
                            uVar21 = iVar38 + iVar14 + CARRY2(CONCAT11(uVar37, uVar36), puVar16);
                            uVar23 = CONCAT11(uVar32, uVar31);
                            uVar33 = uVar39;
                            uVar34 = (uVar39 >> 0x8);
                            uVar31 = 0x8;
                            uVar32 = 0x10;
                            pass1_1010_ebf8(
                                CONCAT13(uVar34, CONCAT12(uVar33, uVar23)),
                                CONCAT11(uVar37, uVar36) + puVar16,
                                uVar21,
                                uVar21,
                            );
                        } //
                        // LAB_1028_7ba0:
                        iVar14 = CONCAT11(uVar34, uVar33) + 1;
                        uVar33 = iVar14;
                        uVar39 = uVar23;
                        uVar34 = (iVar14 >> 0x8);
                    }
                }
                return;
            }
            0xc => {
                paVar20 = uVar20;
                pass1_1030_145a(uVar20.field20_0x26, local_6);
                uVar39 = 0;
                uVar23 = 0;
                while (CONCAT22(uVar23, uVar39) < local_6) {
                    BVar11 = read_file_1008_7dee(
                        CONCAT22(param_3, param_2),
                        CONCAT22(0x1050, &local_30),
                        0x2,
                    );
                    if (BVar11 == 0) {
                        u16_1050_0310 = 0x6d2;
                        return;
                    }
                    uStack44 = switch_1008_73ea(param_2, param_3, local_30);
                    puVar30 = switch_1030_0000(paVar20, paVar17, uVar20, uVar35, uStack44);
                    uVar19 = paVar17 & 0xffff0000;
                    uVar21 = puVar30;
                    uStack38 = (puVar30 >> 0x10);
                    ppcVar8 = (*puVar30 + 0x10);
                    uStack40 = uVar21;
                    (**ppcVar8)();
                    if (uVar21 == 0) {
                        return;
                    }
                    uVar6 = (uStack40 + 0x4);
                    uVar5 = (uStack40 + 0x6);
                    puStack14 = uVar6;
                    pcStack12 = (uVar6 >> 0x10);
                    paVar17 = (uVar19 & 0xffff0000 | uVar5 & 0xffff00ff);
                    paVar20 = uVar20;
                    pass1_1030_14b4(
                        uVar20.field20_0x26,
                        uStack40,
                        uStack38,
                        uVar6 & 0xffff | (uVar5 & 0xff) << 0x10,
                    );
                    lVar7 = CONCAT22(uVar23, uVar39) + 1;
                    uVar39 = lVar7;
                    uVar23 = (lVar7 >> 0x10);
                }
            }
            //   break;
            0xd => {
                puStack10 = (ZEXT24(puVar16) << 0x10);
                uVar3 = &PTR_LOOP_1050_000c;
                local_10 = uVar3;
                puStack14 = (uVar3 >> 0x10);
                pcStack12 = *&PTR_LOOP_1050_0010;
                ppuStack24 = &local_10;
                pass1_1008_3eb4(
                    CONCAT22(0x1050, &local_10),
                    CONCAT22(0x1050, &local_16),
                    CONCAT22(0x1050, &local_14),
                    CONCAT22(0x1050, &local_12),
                );
                ppuStack24 = (local_14 - 1);
                puVar15 = paVar17;
                puStack14 = ppuStack24;
                uVar9 = pass1_1028_21ba(
                    &local_10,
                    puVar15,
                    uVar20,
                    uVar35,
                    CONCAT22(0x1050, &local_10),
                    local_6,
                );
                if (uVar9 == 0) {
                    ppuStack24 = (local_14 + 1);
                    puStack14 = ppuStack24;
                    uVar9 = pass1_1028_21ba(
                        &local_10,
                        puVar15,
                        uVar20,
                        uVar35,
                        CONCAT22(0x1050, &local_10),
                        local_6,
                    );
                    if (uVar9 == 0) {
                        puStack14 = local_14;
                        ppuStack24 = (local_12 - 1);
                        local_10 = ppuStack24;
                        uVar9 = pass1_1028_21ba(
                            &local_10,
                            puVar15,
                            uVar20,
                            uVar35,
                            CONCAT22(0x1050, &local_10),
                            local_6,
                        );
                        if (uVar9 == 0) {
                            ppuStack24 = (local_12 + 1);
                            local_10 = ppuStack24;
                            uVar9 = pass1_1028_21ba(
                                &local_10,
                                puVar15,
                                uVar20,
                                uVar35,
                                CONCAT22(0x1050, &local_10),
                                local_6,
                            );
                            if (uVar9 == 0) {
                                return;
                            }
                        }
                    }
                }
                pass1_1038_79b2(uVar9, puVar15, _PTR_LOOP_1050_5a64, puStack10);
                return;
            }
            0xe => {
                pass1_1030_145a(uVar20.field21_0x2a, local_6);
                uVar39 = 0;
                uVar23 = 0;
                while (CONCAT22(uVar23, uVar39) < local_6) {
                    puVar30 = local_6;
                    mem_op_1000_179c(0x3b2, paVar17);
                    puStack32 = puVar30;
                    uStack30 = paVar17;
                    uVar19 = paVar17 & 0xffff0000 | (uStack30 | puStack32);
                    if ((uStack30 | puStack32) == 0) {
                        uVar9 = 0;
                        uVar19 = paVar17 & 0xffff0000;
                    } else {
                        uVar9 = puStack32;
                        pass1_1030_2068((puVar30 & 0xffff | paVar17 << 0x10));
                    }
                    local_30 = CONCAT22(uVar19, uVar9);
                    ppcVar8 = (*local_30 + 0x10);
                    (**ppcVar8)();
                    if (uVar9 == 0) {
                        return;
                    }
                    uVar22 = (local_30 >> 0x10);
                    uVar21 = local_30;
                    uVar6 = (uVar21 + 0x4);
                    uVar5 = (uVar21 + 0x6);
                    puStack14 = uVar6;
                    pcStack12 = (uVar6 >> 0x10);
                    paVar17 = (uVar19 & 0xffff0000 | uVar5 & 0xffff00ff);
                    pass1_1030_14b4(
                        uVar20.field21_0x2a,
                        uVar21,
                        uVar22,
                        uVar6 & 0xffff | (uVar5 & 0xff) << 0x10,
                    );
                    lVar7 = CONCAT22(uVar23, uVar39) + 1;
                    uVar39 = lVar7;
                    uVar23 = (lVar7 >> 0x10);
                }
            }
        };
        pass1_1030_154c();
    }
    return;
}


pub unsafe fn pass1_1028_ebee(mut param_1: u16, param_2: *mut Struct57, mut param_3: u32) -> u32 {
    let mut puVar1: *mut u8;
    let mut iVar2: i16;
    let mut uVar3: u16;
    let mut uVar4: u32;

    mem_op_1000_179c(0x14, param_2);
    puVar1 = (param_2 | param_1);
    if (puVar1.is_null() == false) {
        pass1_1030_1a32(CONCAT22(param_2, param_1), param_1, puVar1);
    }
    uVar4 = struct_1030_4574((param_3 + 0x52));
    uVar3 = (_PTR_LOOP_1050_5166 >> 0x10);
    iVar2 = _PTR_LOOP_1050_5166;
    (iVar2 + 0x10) = uVar4;
    (iVar2 + 0x12) = (uVar4 >> 0x10);
    uVar3 = (_PTR_LOOP_1050_5166 >> 0x10);
    return CONCAT22((_PTR_LOOP_1050_5166 + 0x6), (_PTR_LOOP_1050_5166 + 0x4));
}


// WARNING: Unable to use type for symbol uVar1
