
pub fn pass1_1000_3024()
{
    pass1_1000_3038(0x1);
    return;
}

pub fn pass1_1000_3038(mut param_1: i16) -> i16
{
    let mut uVar1: u16;
    let mut puVar2: *mut u8;
    let mut iVar3: i16;
    let mut iStack4: i16;

    iVar3 = 0x0;
    iStack4 = 0x0;
    // for (puVar2 =  &PTR_LOOP_1050_6210; puVar2 <= PTR_LOOP_1050_5ff0; puVar2 = puVar2 + 0xc) {
    for puVar2 in (PTR_LOOP_1050_6210 .. PTR_LOOP_1050_5ff0).step_by(0xc) {
        if ((param_1 == 0x1) && ((puVar2[0xa] & 0x83) != 0x0)) {
            uVar1 = pass1_1000_2f48(CONCAT22(0x1050,
                                             puVar2));
            if (uVar1 != 0xffff) {
                iVar3 += 0x1;
            }
        } else if ((param_1 == 0x0) && (((puVar2[0xa] & 0x2) != 0x0 && (uVar1 = pass1_1000_2f48(CONCAT22(0x1050,
                                                                                                         puVar2)), uVar1
            == 0xffff)))) {
            iStack4 = -0x1;
        }
    }
    if (param_1 == 0x1) {
        iStack4 = iVar3;
    }
    return iStack4;
}

pub fn FUN_1000_30b4() -> u16
{
    let mut bVar1: u8;
    let mut bVar2: u8;
    let mut unaff_BP: i16;
    let mut iVar3: i16;
    let mut unaff_SI: u16;
    let mut unaff_CS: u16;
    let mut in_stack_00000008: *mut u8;
    let mut uVar4: u16;

    iVar3 = unaff_BP + 0x1;
    uVar4 = SUB42(&DAT_1050_1050,
                  0x0);
    exit_1000_25f2(0x214,
                   0x30c5,
                   unaff_CS,
                    &DAT_1050_1050);
    bVar1 = *in_stack_00000008;
    if (bVar1 == 0x0) {
        return 0x0;
    }
    if ( (bVar1 - 0x20) < 0x59) {
        bVar2 = *  ( (bVar1 - 0x20) + 0x5ffe) & 0xf;
    } else {
        bVar2 = 0x0;
    }
    // WARNING: Could not emulate address calculation at 0x10003101
    // WARNING: Treating indirect jump as call
    uVar4 =  ( (*  ( (bVar2 * '\b') + 0x5ffe) >> 0x4) * 0x2 + 0x30a4)(unaff_SI & 0xff00 |  bVar1, uVar4, iVar3);
    return uVar4;
}



// WARNING (jumptable): Unable to track spacebase fully for stack

u16 pass1_1000_3113()
{
    let mut cVar1: u8;
    let mut pcVar2: *mut c_char;
    let mut bVar3: u8;
    let mut uVar4: u16;
    let mut unaff_BP: i16;

    FUN_1000_3552();
    pcVar2 = * (unaff_BP + 0xa);
    cVar1 = *pcVar2;
    (unaff_BP + 0xa) =  pcVar2 + 0x1;
    (unaff_BP + -0x4) = cVar1;
    if ((cVar1 != '\0') && (-0x1 < (unaff_BP + -0xa))) {
        if ( (cVar1 - 0x20U) < 0x59) {
            bVar3 = *  ( (cVar1 - 0x20U) + 0x5ffe) & 0xf;
        } else {
            bVar3 = 0x0;
        }
        bVar3 = *  ( (bVar3 * '\b' + (unaff_BP + -0x7)) + 0x5ffe) >> 0x4;
        * (unaff_BP + -0x7) = bVar3;
        // WARNING: Could not recover jumptable at 0x1000310e. Too many branches
        // WARNING: Treating indirect jump as call
        uVar4 =  ( bVar3 * 0x2 + 0x30a4))();
        return uVar4;
    }
    return (unaff_BP + -0xa);
}



// WARNING (jumptable): Stack frame is not setup normally: Input value of stackpointer is not used

u16 pass1_1000_311e()
{
    let mut cVar1: u8;
    let mut pcVar2: *mut c_char;
    let mut bVar3: u8;
    let mut uVar4: u16;
    let mut unaff_BP: i16;

    (unaff_BP + -0x12) = 0x0;
    (unaff_BP + -0xc) = 0x0;
    (unaff_BP + -0x14) = 0x0;
    (unaff_BP + -0x6) = 0x20;
    (unaff_BP + -0xe) = 0xffff;
    pcVar2 = * (unaff_BP + 0xa);
    cVar1 = *pcVar2;
    (unaff_BP + 0xa) =  pcVar2 + 0x1;
    (unaff_BP + -0x4) = cVar1;
    if ((cVar1 != '\0') && (-0x1 < (unaff_BP + -0xa))) {
        if ( (cVar1 - 0x20U) < 0x59) {
            bVar3 = *  ( (cVar1 - 0x20U) + 0x5ffe) & 0xf;
        } else {
            bVar3 = 0x0;
        }
        bVar3 = *  ( (bVar3 * '\b' + (unaff_BP + -0x7)) + 0x5ffe) >> 0x4;
        * (unaff_BP + -0x7) = bVar3;
        // WARNING: Could not recover jumptable at 0x1000310e. Too many branches
        // WARNING: Treating indirect jump as call
        uVar4 =  ( bVar3 * 0x2 + 0x30a4))();
        return uVar4;
    }
    return (unaff_BP + -0xa);
}



// WARNING (jumptable): Stack frame is not setup normally: Input value of stackpointer is not used

u16 pass1_1000_3134()
{
    let mut pbVar1: *mut u8;
    let mut cVar2: u8;
    let mut pcVar3: *mut c_char;
    let mut bVar4: u8;
    let mut uVar5: u16;
    let mut unaff_BP: i16;

    cVar2 = (unaff_BP + -0x4);
    if (cVar2 == '-') {
        pbVar1 =  (unaff_BP + -0x6);
        *pbVar1 = *pbVar1 | 0x4;
    } else if (cVar2 == '+') {
        pbVar1 =  (unaff_BP + -0x6);
        *pbVar1 = *pbVar1 | 0x1;
    } else if (cVar2 == ' ') {
        pbVar1 =  (unaff_BP + -0x6);
        *pbVar1 = *pbVar1 | 0x2;
    } else if (cVar2 == '#') {
        pbVar1 =  (unaff_BP + -0x6);
        *pbVar1 = *pbVar1 | 0x80;
    } else {
        pbVar1 =  (unaff_BP + -0x6);
        *pbVar1 = *pbVar1 | 0x8;
    }
    pcVar3 = * (unaff_BP + 0xa);
    cVar2 = *pcVar3;
    (unaff_BP + 0xa) =  pcVar3 + 0x1;
    (unaff_BP + -0x4) = cVar2;
    if ((cVar2 != '\0') && (-0x1 < (unaff_BP + -0xa))) {
        if ( (cVar2 - 0x20U) < 0x59) {
            bVar4 = *  ( (cVar2 - 0x20U) + 0x5ffe) & 0xf;
        } else {
            bVar4 = 0x0;
        }
        bVar4 = *  ( (bVar4 * '\b' + (unaff_BP + -0x7)) + 0x5ffe) >> 0x4;
        * (unaff_BP + -0x7) = bVar4;
        // WARNING: Could not recover jumptable at 0x1000310e. Too many branches
        // WARNING: Treating indirect jump as call
        uVar5 =  ( bVar4 * 0x2 + 0x30a4))();
        return uVar5;
    }
    return (unaff_BP + -0xa);
}



// WARNING (jumptable): Unable to track spacebase fully for stack

u16 pass1_1000_3168()
{
    let mut pbVar1: *mut u8;
    let mut cVar2: u8;
    let mut pcVar3: *mut c_char;
    let mut bVar4: u8;
    let mut uVar5: u16;
    let mut unaff_BP: i16;

    cVar2 = (unaff_BP + -0x4);
    if (cVar2 == '*') {
        uVar5 = pass1_1000_34cf();
        if ( uVar5 < 0x0) {
            uVar5 = -uVar5;
            pbVar1 =  (unaff_BP + -0x6);
            *pbVar1 = *pbVar1 | 0x4;
        }
    } else {
        uVar5 = (unaff_BP + -0xc) * 0xa +   (cVar2 - 0x30);
    }
    (unaff_BP + -0xc) = uVar5;
    pcVar3 = * (unaff_BP + 0xa);
    cVar2 = *pcVar3;
    (unaff_BP + 0xa) =  pcVar3 + 0x1;
    (unaff_BP + -0x4) = cVar2;
    if ((cVar2 != '\0') && (-0x1 < (unaff_BP + -0xa))) {
        if ( (cVar2 - 0x20U) < 0x59) {
            bVar4 = *  ( (cVar2 - 0x20U) + 0x5ffe) & 0xf;
        } else {
            bVar4 = 0x0;
        }
        bVar4 = *  ( (bVar4 * '\b' + (unaff_BP + -0x7)) + 0x5ffe) >> 0x4;
        * (unaff_BP + -0x7) = bVar4;
        // WARNING: Could not recover jumptable at 0x1000310e. Too many branches
        // WARNING: Treating indirect jump as call
        uVar5 =  ( bVar4 * 0x2 + 0x30a4))();
        return uVar5;
    }
    return (unaff_BP + -0xa);
}



// WARNING (jumptable): Stack frame is not setup normally: Input value of stackpointer is not used

u16 pass1_1000_3194()
{
    let mut cVar1: u8;
    let mut pcVar2: *mut c_char;
    let mut bVar3: u8;
    let mut uVar4: u16;
    let mut unaff_BP: i16;

    (unaff_BP + -0xe) = 0x0;
    pcVar2 = * (unaff_BP + 0xa);
    cVar1 = *pcVar2;
    (unaff_BP + 0xa) =  pcVar2 + 0x1;
    (unaff_BP + -0x4) = cVar1;
    if ((cVar1 != '\0') && (-0x1 < (unaff_BP + -0xa))) {
        if ( (cVar1 - 0x20U) < 0x59) {
            bVar3 = *  ( (cVar1 - 0x20U) + 0x5ffe) & 0xf;
        } else {
            bVar3 = 0x0;
        }
        bVar3 = *  ( (bVar3 * '\b' + (unaff_BP + -0x7)) + 0x5ffe) >> 0x4;
        * (unaff_BP + -0x7) = bVar3;
        // WARNING: Could not recover jumptable at 0x1000310e. Too many branches
        // WARNING: Treating indirect jump as call
        uVar4 =  ( bVar3 * 0x2 + 0x30a4))();
        return uVar4;
    }
    return (unaff_BP + -0xa);
}



// WARNING (jumptable): Unable to track spacebase fully for stack

u16 pass1_1000_319c()
{
    let mut cVar1: u8;
    let mut pcVar2: *mut c_char;
    let mut bVar3: u8;
    let mut uVar4: u16;
    let mut unaff_BP: i16;

    cVar1 = (unaff_BP + -0x4);
    if (cVar1 == '*') {
        uVar4 = pass1_1000_34cf();
        if ( uVar4 < 0x0) {
            uVar4 = 0xffff;
        }
    } else {
        uVar4 = (unaff_BP + -0xe) * 0xa +   (cVar1 - 0x30);
    }
    (unaff_BP + -0xe) = uVar4;
    pcVar2 = * (unaff_BP + 0xa);
    cVar1 = *pcVar2;
    (unaff_BP + 0xa) =  pcVar2 + 0x1;
    (unaff_BP + -0x4) = cVar1;
    if ((cVar1 != '\0') && (-0x1 < (unaff_BP + -0xa))) {
        if ( (cVar1 - 0x20U) < 0x59) {
            bVar3 = *  ( (cVar1 - 0x20U) + 0x5ffe) & 0xf;
        } else {
            bVar3 = 0x0;
        }
        bVar3 = *  ( (bVar3 * '\b' + (unaff_BP + -0x7)) + 0x5ffe) >> 0x4;
        * (unaff_BP + -0x7) = bVar3;
        // WARNING: Could not recover jumptable at 0x1000310e. Too many branches
        // WARNING: Treating indirect jump as call
        uVar4 =  ( bVar3 * 0x2 + 0x30a4))();
        return uVar4;
    }
    return (unaff_BP + -0xa);
}



// WARNING (jumptable): Stack frame is not setup normally: Input value of stackpointer is not used

u16 pass1_1000_31c5()
{
    let mut pbVar1: *mut u8;
    let mut cVar2: u8;
    let mut pcVar3: *mut c_char;
    let mut bVar4: u8;
    let mut uVar5: u16;
    let mut unaff_BP: i16;

    cVar2 = (unaff_BP + -0x4);
    if (cVar2 == 'l') {
        pbVar1 =  (unaff_BP + -0x6);
        *pbVar1 = *pbVar1 | 0x10;
    } else if (cVar2 == 'F') {
        pbVar1 =  (unaff_BP + -0x6);
        *pbVar1 = *pbVar1 | 0x20;
    } else if (cVar2 == 'N') {
        pbVar1 =  (unaff_BP + -0x5);
        *pbVar1 = *pbVar1 | 0x10;
    } else if (cVar2 == 'L') {
        pbVar1 =  (unaff_BP + -0x5);
        *pbVar1 = *pbVar1 | 0x4;
    } else {
        pbVar1 =  (unaff_BP + -0x5);
        *pbVar1 = *pbVar1 | 0x8;
    }
    pcVar3 = * (unaff_BP + 0xa);
    cVar2 = *pcVar3;
    (unaff_BP + 0xa) =  pcVar3 + 0x1;
    (unaff_BP + -0x4) = cVar2;
    if ((cVar2 != '\0') && (-0x1 < (unaff_BP + -0xa))) {
        if ( (cVar2 - 0x20U) < 0x59) {
            bVar4 = *  ( (cVar2 - 0x20U) + 0x5ffe) & 0xf;
        } else {
            bVar4 = 0x0;
        }
        bVar4 = *  ( (bVar4 * '\b' + (unaff_BP + -0x7)) + 0x5ffe) >> 0x4;
        * (unaff_BP + -0x7) = bVar4;
        // WARNING: Could not recover jumptable at 0x1000310e. Too many branches
        // WARNING: Treating indirect jump as call
        uVar5 =  ( bVar4 * 0x2 + 0x30a4))();
        return uVar5;
    }
    return (unaff_BP + -0xa);
}



// WARNING (jumptable): Unable to track spacebase fully for stack

u16 pass1_1000_31f7(mut param_1: u16 )
{
    let mut piVar1: *mut i16;
    let mut pbVar2: *mut u8;
    let mut puVar3: *mut u16;
    let mut cVar4: u8;
    let mut pcVar5: *mut c_char;
    let mut bVar6: u8;
    let mut uVar7: u16;
    let mut iVar8: i16;
    let mut pcVar9: *mut c_char;
    let mut uVar10: u16;
    let mut extraout_DX: u16;
    let mut extraout_DX_00: u16;
    let mut extraout_DX_01: u16;
    let mut extraout_DX_02: u16;
    let mut extraout_DX_03: u16;
    let mut extraout_DX_04: u16;
    let mut unaff_BP: i16;
    let mut unaff_DI: *mut u16;
    let mut puVar11: *mut u16;
    let mut unaff_ES: i16;
    let mut in_AF: u8;
    let mut bVar12: bool;
    let mut uVar13: u32;

    cVar4 = (unaff_BP + -0x4);
    if ((cVar4 == 'd') || (cVar4 == 'i')) {
        pbVar2 =  (unaff_BP + -0x6);
        *pbVar2 = *pbVar2 | 0x40;//
//        LAB_1000_3399:
        * (unaff_BP + -0x8) = 0xa;//
//        LAB_1000_33d4:
        if ((* (unaff_BP + -0x6) & 0x10) == 0x0) {
            uVar7 = pass1_1000_34cf();
            if ((* (unaff_BP + -0x6) & 0x40) == 0x0) {
                uVar13 =  uVar7;
            } else {
                uVar13 =   uVar7;
            }
        } else {
            uVar13 = pass1_1000_34d8();
        }
        if (((* (unaff_BP + -0x6) & 0x40) != 0x0) && ((long) uVar13 < 0x0)) {
            pbVar2 =  (unaff_BP + -0x5);
            *pbVar2 = *pbVar2 | 0x1;
            uVar13 = CONCAT22(-( (uVar13 >> 0x10) +  ( uVar13 != 0x0)),
                              - uVar13);
        }
        param_1 =  (uVar13 >> 0x10);
        if ((unaff_BP + -0xe) < 0x0) {
            (unaff_BP + -0xe) = 0x1;
        } else {
            pbVar2 =  (unaff_BP + -0x6);
            *pbVar2 = *pbVar2 & 0xf7;
        }
        if (uVar13 == 0x0) {
            (unaff_BP + -0x12) = 0x0;
        }
        unaff_DI =  (unaff_BP + -0x17);
        unaff_ES =  &DAT_1050_1050;
        pcVar9 =   * (unaff_BP + -0x8);
        pass1_1000_356e( uVar13,
                         pcVar9,
                        param_1);
        if (((* (unaff_BP + -0x5) & 0x2) != 0x0) && ((pcVar9 == NULL || (unaff_DI != '0')))) {
            unaff_DI =  (unaff_BP + -0x18);
            unaff_DI = '0';
            pcVar9 = pcVar9 + 0x1;
        }
    } else {
        if (cVar4 == 'u') {
        // TODO: goto LAB_1000_3399;
        }
        if (cVar4 == 'X') {
            * (unaff_BP + -0x3) = 0x7;//
//            LAB_1000_33a9:
            if ((* (unaff_BP + -0x6) & 0x80) != 0x0) {
                (unaff_BP + -0x12) = 0x2;
                * (unaff_BP + -0x10) = 0x30;
                (unaff_BP + -0xf) = (unaff_BP + -0x3) + 'Q';
            }
            * (unaff_BP + -0x8) = 0x10;
        // TODO: goto LAB_1000_33d4;
        }
        if (cVar4 == 'x') {
            * (unaff_BP + -0x3) = 0x27;
        // TODO: goto LAB_1000_33a9;
        }
        if (cVar4 == 'o') {
            if ((* (unaff_BP + -0x6) & 0x80) != 0x0) {
                pbVar2 =  (unaff_BP + -0x5);
                *pbVar2 = *pbVar2 | 0x2;
            }
            * (unaff_BP + -0x8) = 0x8;
        // TODO: goto LAB_1000_33d4;
        }
        if (cVar4 == 'c') {
            uVar7 = pass1_1000_34cf();
            unaff_ES =  &DAT_1050_1050;
            * (unaff_BP + -0x216) =  uVar7;
            unaff_DI =  (unaff_BP + -0x216);
            pcVar9 =  0x1;
        } else if (cVar4 == 's') {
            uVar13 = pass1_1000_34e6(param_1);
            param_1 =  (uVar13 >> 0x10);
            if ((unaff_DI == NULL) && (unaff_ES == 0x0)) {
                unaff_ES =  &DAT_1050_1050;
                unaff_DI =  0x6057;
                pcVar9 = DAT_1050_605d;
            } else {
                iVar8 = (unaff_BP + -0xe);
                puVar11 = unaff_DI;
                if (iVar8 != 0x0) {
                    bVar12 = true;
                    loop {
                        if (iVar8 == 0x0) {
                            break;
                        }
                        iVar8 += -0x1;
                        puVar3 = puVar11;
                        puVar11 =  ( puVar11 + 0x1);
                        bVar12 = puVar3 == '\0';
                    } while (!bVar12);
                    if (bVar12) {
                        puVar11 =  ( puVar11 + -0x1);
                    }
                }
                pcVar9 =  ( puVar11 -  unaff_DI);
            }
        } else {
            if (cVar4 == 'n') {
                pass1_1000_34e6(param_1);
                *unaff_DI = (unaff_BP + -0xa);
                if ((* (unaff_BP + -0x6) & 0x10) != 0x0) {
                    unaff_DI[0x1] = 0x0;
                }
            // TODO: goto LAB_1000_30cf;
            }
            if (cVar4 != 'p') {
                if ((cVar4 == 'E') || (cVar4 == 'G')) {
                    piVar1 =  (unaff_BP + -0x14);
                    *piVar1 = *piVar1 + 0x1;
                }
                pbVar2 =  (unaff_BP + -0x6);
                *pbVar2 = *pbVar2 | 0x40;
                bVar6 = * (unaff_BP + -0x4) | 0x20;
                iVar8 = (unaff_BP + -0xe);
                if (iVar8 < 0x1) {
                    if (iVar8 == 0x0) {
                        if (bVar6 == 0x67) {
                            (unaff_BP + -0xe) = 0x1;
                        }
                    } else {
                        (unaff_BP + -0xe) = 0x6;
                    }
                }
                unaff_DI =  (unaff_BP + -0x216);
                if ((* (unaff_BP + -0x5) & 0x4) == 0x0) {
                    ( PTR_s_3_wav_1050_25cc_1050_6068)();
                    piVar1 =  (unaff_BP + 0xe);
                    *piVar1 = *piVar1 + 0x8;
                    param_1 = extraout_DX_00;
                } else {
                    ( PTR_s_3_wav_1050_25cc_1050_607c)();
                    piVar1 =  (unaff_BP + 0xe);
                    *piVar1 = *piVar1 + 0xa;
                    param_1 = extraout_DX;
                }
                if (((* (unaff_BP + -0x6) & 0x80) != 0x0) && ((unaff_BP + -0xe) == 0x0)) {
                    ( PTR_s_3_wav_1050_25cc_1050_6074)();
                    param_1 = extraout_DX_01;
                }
                if ((bVar6 == 0x67) && (((unaff_BP + -0x6) & 0x80) == 0x0)) {
                    ( PTR_s_3_wav_1050_25cc_1050_6070)();
                    param_1 = extraout_DX_02;
                }
                unaff_ES =  &DAT_1050_1050;
                if (unaff_DI == '-') {
                    unaff_DI =  (unaff_BP + -0x215);
                    pbVar2 =  (unaff_BP + -0x5);
                    *pbVar2 = *pbVar2 | 0x1;
                }
                iVar8 = -0x1;
                puVar11 = unaff_DI;
                loop {
                    if (iVar8 == 0x0) {
                        break;
                    }
                    iVar8 += -0x1;
                    puVar3 = puVar11;
                    puVar11 =  ( puVar11 + 0x1);
                } while (puVar3 != '\0');
                pcVar9 =  ( puVar11 + (-0x1 -  unaff_DI));
            // TODO: goto LAB_1000_3444;
            }
            if ((* (unaff_BP + -0x6) & 0x30) == 0x0) {
                uVar7 = pass1_1000_34cf();
                uVar13 =  uVar7;//
//                LAB_1000_32d8:
                * (unaff_BP + -0x3) = 0x7;
                param_1 = 0x0;
                pass1_1000_356e( uVar13,
                                0x10,
                                0x0);
                pcVar9 =  0x4;
            } else {
                uVar13 = pass1_1000_34d8();
                uVar10 =  (uVar13 >> 0x10);
                if ((* (unaff_BP + -0x5) & 0x18) != 0x0) {
                // TODO: goto LAB_1000_32d8;
                }
                * (unaff_BP + -0x3) = 0x7;
                pass1_1000_356e( uVar13,
                                0x10,
                                0x0);
                param_1 = 0x0;
                pass1_1000_356e(uVar10,
                                0x10,
                                0x0);
                * (unaff_BP + -0x212) = 0x3a;
                pcVar9 =  0x9;
            }
            unaff_ES =  &DAT_1050_1050;
            unaff_DI =  (unaff_BP + -0x216);
        }
    }//
//    LAB_1000_3444:
    if ((* (unaff_BP + -0x6) & 0x40) != 0x0) {
        if ((* (unaff_BP + -0x5) & 0x1) == 0x0) {
            if ((* (unaff_BP + -0x6) & 0x1) == 0x0) {
                if ((* (unaff_BP + -0x6) & 0x2) != 0x0) {
                    * (unaff_BP + -0x10) = 0x20;
                    (unaff_BP + -0x12) = 0x1;
                }
            } else {
                * (unaff_BP + -0x10) = 0x2b;
                (unaff_BP + -0x12) = 0x1;
            }
        } else {
            * (unaff_BP + -0x10) = 0x2d;
            (unaff_BP + -0x12) = 0x1;
        }
    }
    if ((* (unaff_BP + -0x6) & 0xc) == 0x0) {
        FUN_1000_3552(pcVar9,
                      unaff_DI,
                      unaff_ES);
        param_1 = extraout_DX_03;
    }
    pass1_1000_3534(in_AF,
                    (unaff_BP + -0x12),
                    param_1);
    if (((* (unaff_BP + -0x6) & 0x8) != 0x0) && ((* (unaff_BP + -0x6) & 0x4) == 0x0)) {
        FUN_1000_3552(pcVar9,
                      unaff_DI,
                      unaff_ES);
        param_1 = extraout_DX_04;
    }
    pass1_1000_3534(in_AF,
                    pcVar9,
                    param_1);
    if ((* (unaff_BP + -0x6) & 0x4) != 0x0) {
        FUN_1000_3552();
    }//
//    LAB_1000_30cf:
    pcVar5 = * (unaff_BP + 0xa);
    cVar4 = *pcVar5;
    (unaff_BP + 0xa) =  pcVar5 + 0x1;
    (unaff_BP + -0x4) = cVar4;
    if ((cVar4 != '\0') && (-0x1 < (unaff_BP + -0xa))) {
        if ( (cVar4 - 0x20U) < 0x59) {
            bVar6 = *  ( (cVar4 - 0x20U) + 0x5ffe) & 0xf;
        } else {
            bVar6 = 0x0;
        }
        bVar6 = *  ( (bVar6 * '\b' + (unaff_BP + -0x7)) + 0x5ffe) >> 0x4;
        * (unaff_BP + -0x7) = bVar6;
        // WARNING: Could not recover jumptable at 0x1000310e. Too many branches
        // WARNING: Treating indirect jump as call
        uVar7 =  ( bVar6 * 0x2 + 0x30a4))();
        return uVar7;
    }
    return (unaff_BP + -0xa);
}

u16 pass1_1000_34cf()
{
    let mut uVar1: u16;
    let mut puVar2: *mut u16;
    let mut unaff_BP: i16;

    puVar2 =  (unaff_BP + 0xe);
    uVar1 = *puVar2;
    (unaff_BP + 0xe) =  puVar2 + 0x2;
    return uVar1;
}

pub fn pass1_1000_34d8() -> u32
{
    let mut u_var1: u16;
    let mut u_var2: u16;
    let mut pu_var3: *mut u16;
    let mut unaff_bp: i16;

    pu_var3 =   (unaff_bp + 0xe);
    u_var1 = *pu_var3;
    u_var2 = ( pu_var3 + 0x2);
    (unaff_bp + 0xe) =  pu_var3 + 0x4;
    return CONCAT22(u_var2,
                    u_var1);
}

pub fn pass1_1000_34e6(mut param_1: u16 ) -> u32
{
    let mut u_var1: u16;
    let mut unaff_bp: i16;
    let mut u_var2: u32;

    if ((* (unaff_bp + -0x6) & 0x20) != 0x0) {
        u_var2 = pass1_1000_34d8();
        return u_var2;
    }
    u_var1 = pass1_1000_34cf();
    if (u_var1 == 0x0) {
        return  param_1 << 0x10;
    }
    return CONCAT22(param_1,
                    u_var1);
}

i16 pass1_1000_3503(mut param_1: u16 ,
                    mut param_2: u16 )
{
    let mut piVar1: *mut i16;
    let mut pcVar2: *mut c_char;
    char **ppcVar3;
    let mut uVar4: u16;
    let mut piVar5: *mut i16;
    let mut unaff_BP: i16;
    let mut uVar6: u16;

    ppcVar3 =   (unaff_BP + 0x6);
    uVar6 =  ( ppcVar3 >> 0x10);
    piVar5 =  ppcVar3;
    piVar1 = piVar5 + 0x2;
    *piVar1 = *piVar1 + -0x1;
    if (*piVar1 < 0x0) {
        uVar4 = mem_1000_2bb6(param_2,
                                param_1,
                              piVar5);
        if (uVar4 == 0xffff) {
            return -0x1;
        }
    } else {
        pcVar2 = *ppcVar3;
        *ppcVar3 = *ppcVar3 + 0x1;
        *pcVar2 =  param_1;
    }
    return 0x0;
}
pub fn pass1_1000_3534(undefined1 param_1,
                     mut param_2: i16,
                     mut param_3: u16 )
{
    let mut piVar1: *mut i16;
    let mut pbVar2: *mut u8;
    let mut in_ax: u16;
    let mut unaff_bp: i16;
    let mut unaff_DI: *mut u8;
    let mut u_var3: u16;
    let mut unaff_es: u16;

    if (param_2 != 0x0) {
        piVar1 =  (unaff_bp + -0xa);
        *piVar1 = *piVar1 + param_2;
        u_var3 = 0x0;
        loop {
            pbVar2 = unaff_DI;
            unaff_DI = unaff_DI + 0x1;
            in_ax = pass1_1000_3503(in_ax & 0xff00 |  *pbVar2,
                                    param_3);
            u_var3 |= in_ax;
            param_2 += -0x1;
        } while (param_2 != 0x0);
        if (u_var3 != 0x0) {
            (unaff_bp + -0xa) = 0xffff;
        }
    }
    return;
}
pub fn FUN_1000_3552()
{
    let mut piVar1: *mut i16;
    mut param_3: u16 ;
    let mut param_1: i16;
    mut param_2: u16 ;
    let mut unaff_bp: i16;
    let mut u_var2: u16;

    if (param_1 != 0x0) {
        piVar1 =  (unaff_bp + -0xa);
        *piVar1 = *piVar1 + param_1;
        u_var2 = 0x0;
        loop {
            param_3 = pass1_1000_3503(param_3 & 0xff00 | param_2 & 0xff,
                                      param_2);
            u_var2 |= param_3;
            param_1 += -0x1;
        } while (param_1 != 0x0);
        if (u_var2 != 0x0) {
            (unaff_bp + -0xa) = 0xffff;
        }
    }
    return;
}
pub fn pass1_1000_356e(mut param_1: u16 ,
                     mut param_2: u16 ,
                     mut param_3: u16 )
{
    let mut pbVar1: *mut u8;
    let mut u_var2: u32;
    let mut bVar3: u8;
    let mut unaff_bp: i16;
    let mut unaff_si: i16;
    let mut unaff_DI: *mut u8;
    let mut unaff_es: u16;

    while (((0x0 < unaff_si || (param_1 != 0x0)) || (param_3 != 0x0))) {
        u_var2 =  param_3;
        param_3 /= param_2;
        u_var2 = u_var2 %  param_2 << 0x10 |  param_1;
        param_1 =  (u_var2 / param_2);
        bVar3 =  (u_var2 %  param_2) + 0x30;
        if (0x39 < bVar3) {
            bVar3 += (unaff_bp + -0x3);
        }
        pbVar1 = unaff_DI;
        unaff_DI = unaff_DI + -0x1;
        *pbVar1 = bVar3;
        unaff_si += -0x1;
    }
    return;
}

u16 *pass1_1000_35aa()
{
    let mut puVar1: *mut u16;

    puVar1 =  &PTR_LOOP_1050_6210;
    loop {
        if (PTR_LOOP_1050_5ff0 < puVar1) {
            return NULL;
        }
        if ((* (puVar1 + 0x5) & 0x83) == 0x0) {
            break;
        }
        puVar1 = puVar1 + 0x6;
    }
    * (puVar1 + 0x5) = 0x0;
    puVar1[0x2] = 0x0;
    puVar1[0x4] = 0x0;
    puVar1[0x3] = 0x0;
    puVar1[0x1] = 0x0;
    *puVar1 = 0x0;
    * ( puVar1 + 0xb) = 0xff;
    return puVar1;
}



// WARNING: Removing unreachable block (ram,0x10003622)

u16 dos3_call_op_1000_35fe(mut param_1: u16 ,
                           mut param_2: i16)
{
//    code *pcVar1;
    let mut uVar2: u16;
    let mut u16_var2: bool;

    if (param_1 < u16_1050_5f8a) {
        u16_var2 = false;
        code7 fn_ptr_1 = (code7) swi(0x21);
        uVar2 = fn_ptr_1(param_2 + 0x1);
        if (!u16_var2) {
            * (param_1 + 0x5f90) = 0x0;
        }
    } else {
        uVar2 = 0x900;
        u16_var2 = true;
    }
    if (u16_var2) {
        pass1_1000_29b5(uVar2);
        return 0xffff;
    }
    return 0x0;
}



// WARNING: Removing unreachable block (ram,0x100036b5)
// WARNING: Removing unreachable block (ram,0x10003681)
// WARNING: Removing unreachable block (ram,0x100036f7)
// WARNING: Removing unreachable block (ram,0x100036d8)
pub fn mixed_dos3_call_1000_3636(mut param_1: u16 ,
                               mut param_2: u16 ,
                               mut param_3: u16 ,
                               mut param_4: u16 )
{
    let mut pbVar1: *mut u8;
    code *pcVar2;
    let mut u_var3: u16;
    let mut unaff_bp: i16;
    let mut i_var4: i16;
    let mut b_var5: bool;
    let mut u_var6: u32;

    i_var4 = unaff_bp + 0x1;
    if (((param_1 < u16_1050_5f8a) || (u16_1050_61ec == 0x0)) || (0x2 < param_1)) {
        if ((u16_1050_6064 == 0x0) || ((param_3 & 0x8000) == 0x0)) {
        // TODO: goto LAB_1000_36e3;
        }
        if (param_4 == 0x0) {
        // TODO: goto LAB_1000_369b;
        }
        b_var5 = false;
        pcVar2 =  swi(0x21);
        u_var6 = (*pcVar2)();
        u_var3 = u_var6;
        if (b_var5) {
        // TODO: goto LAB_1000_299d;
        }
        if ((param_4 & 0x2) == 0x0) {
            if (-0x1 <  ( ( u_var6 >> 0x10) + param_3 +  CARRY2(u_var3,
                                                                     param_2))) {//
//                LAB_1000_36e3:
                bVar5 = false;
                pcVar2 =  swi(0x21);
                uVar3 = (*pcVar2)(iVar4);
                if (!bVar5) {
                    pbVar1 =  (param_1 + 0x5f90);
                    bVar5 = false;
                    *pbVar1 = *pbVar1 & 0xfd;
                }
            // TODO: goto LAB_1000_299d;
            }
        } else {
            pcVar2 =  swi(0x21);
            u_var6 = (*pcVar2)();
            if (-0x1 <  ( ( u_var6 >> 0x10) + param_3 +  CARRY2(u_var6,
                                                                     param_2))) {
            // TODO: goto LAB_1000_36e3;
            }
            pcVar2 =  swi(0x21);
            (*pcVar2)();
        }//
//        LAB_1000_369b:
        u_var3 =  s_471_bmp_1050_1600;
    } else {
        u_var3 = 0x900;
    }
    b_var5 = true;//
//    LAB_1000_299d:
    if (b_var5) {
        block_1000_2000::pass1_1000_29b5(u_var3);
    }
    return;
}



// WARNING: Removing unreachable block (ram,0x10003989)
// WARNING: Removing unreachable block (ram,0x100038a1)
// WARNING: Removing unreachable block (ram,0x10003867)
// WARNING: Removing unreachable block (ram,0x10003967)
// WARNING: Removing unreachable block (ram,0x1000391a)
// WARNING: Removing unreachable block (ram,0x10003803)
// WARNING: Removing unreachable block (ram,0x100037b7)
// WARNING: Removing unreachable block (ram,0x10003799)
// WARNING: Removing unreachable block (ram,0x10003765)
// WARNING: Removing unreachable block (ram,0x100037ec)
// WARNING: Removing unreachable block (ram,0x100038d9)
// WARNING: Removing unreachable block (ram,0x100038f2)
// WARNING: Removing unreachable block (ram,0x1000393a)
// WARNING: Removing unreachable block (ram,0x1000384b)
// WARNING: Removing unreachable block (ram,0x1000388b)
// WARNING: Removing unreachable block (ram,0x100038ba)
// WARNING: Removing unreachable block (ram,0x100039b9)
// WARNING: Removing unreachable block (ram,0x1000381c)
// WARNING: Could not reconcile some variable overlaps

u16 mixed_dos3_call_1000_370a(mut param_1: u16 ,
                              mut param_2: u16 ,
                              mut param_3: u16 ,
                              mut param_4: u16 ,
                              param_5: u8,
                              mut param_6: u16 )
{
    code *fn_ptr_1;
    let mut uVar3: u16;
    let mut uVar2: u16;
    let mut iVar3: i16;
    let mut bVar2: u8;
    let mut uVar7: u16;
    let mut extraout_DX: u16;
    let mut uVar4: u16;
    let mut uVar5: u16;
    let mut unaff_BP: i16;
    let mut uVar6: u16;
    let mut uVar8: u16;
    let mut bVar10: bool;
    let mut uStack14: u16;
    let mut bVar9: u8;
    let mut in_stack_0000fffa: u16;
    let mut uVar9: u16;
    let mut uVar10: u16;
    let mut cVar12: u8;

    uVar6 = unaff_BP + 0x1;
    param_5 = param_6;
    uVar3 = param_1 & 0xff00;
    param_1 = uVar3 | param_5;
    uVar9 = 0x0;
    if (((param_4 & 0x8000) == 0x0) && (((param_4 & 0x4000) != 0x0 || ((DAT_1050_6061 & 0x80) == 0x0)))) {
        uVar9 = 0x80;
    }
    bVar10 = false;
    fn_ptr_1 =  swi(0x21);
    uVar7 = param_4;
    uVar2 = (*fn_ptr_1)();
    if (bVar10) {
        if ((uVar2 == 0x2) && ((uVar7 & 0x100) != 0x0)) {
            uVar10 = uVar9 & 0xff;
            // s_____s__lu_1050_38cd + 0x3
            param_1 =  0x38d0;
            pass1_1000_39e1();
            uVar7 = 0x0;
            _param_5 = param_6;//
//            LAB_1000_38e3:
            bVar10 = false;
            fn_ptr_1 =  swi(0x21);
            uVar2 = (*fn_ptr_1)();
            if (bVar10) {
            // TODO: goto LAB_1000_299d;
            }
            if (( param_1 != '\0') || (uVar5 = uVar2, uVar8 = uStack14, (param_4 & 0x2) == 0x0)) {
                fn_ptr_1 =  swi(0x21);
                (*fn_ptr_1)();
                bVar10 = false;
                fn_ptr_1 =  swi(0x21);
                uVar2 = (*fn_ptr_1)();
                if (bVar10) {
                // TODO: goto LAB_1000_299d;
                }
                uVar5 = uVar2;
                uVar8 = param_1;
                if (((uVar10 & 0x100) == 0x0) && (uVar8 = param_1, (_param_5 & 0x1) != 0x0)) {
                    uVar7 =   ( uVar7 | 0x1);
                    bVar10 = false;
                    fn_ptr_1 =  swi(0x21);
                    uVar4 = uVar2;
                    uVar2 = (*fn_ptr_1)();
                    uVar5 = uVar4;
                    uVar8 = uVar6;
                    if (bVar10) {
                    // TODO: goto LAB_1000_299d;
                    }
                }
            }//
//            LAB_1000_3973:
            bVar9 =  uVar10;
            if ((uVar10 & 0x40) == 0x0) {
                fn_ptr_1 =  swi(0x21);
                (*fn_ptr_1)();
                bVar2 = 0x0;
                if ((uVar7 & 0x1) != 0x0) {
                    bVar2 = 0x10;
                }
                if ((param_4 & 0x8) != 0x0) {
                    bVar2 |= 0x20;
                }
            } else {
                bVar2 = 0x0;
            }
            if (uVar5 < &u16_1050_5f8a) {
                * (uVar5 + 0x5f90) = bVar2 | bVar9 | 0x1;
                return uVar5;
            }
            fn_ptr_1 =  swi(0x21);
            (*fn_ptr_1)();
            uVar2 = 0x1800;
        }
    } else {
        if ((uVar7 & 0x500) != 0x500) {
            uVar10 = CONCAT11(0x1,
                               uVar9);
            fn_ptr_1 =  swi(0x21);
            (*fn_ptr_1)();
            if ((extraout_DX & 0x80) != 0x0) {
                uVar10 |= 0x40;
            }
            uVar5 = uVar2;
            uVar8 = param_1;
            if ((uVar10 & 0x40) == 0x0) {
                if ((param_4 & 0x200) == 0x0) {
                    if (((uVar10 & 0x80) != 0x0) && ((param_4 & 0x2) != 0x0)) {
                        code fn_ptr_1 =  swi(0x21);
                        (fn_ptr_1)();
                        code5 fn_ptr_2 = (code5) swi(0x21);
                        iVar3 = (fn_ptr_2)();
                        if ((iVar3 != 0x0) && (param_1 =  (uVar3 >> 0x8), param_1 == '\x1a')) {
                            fn_ptr_1 =  swi(0x21);
                            (*fn_ptr_1)(uVar6);
                            fn_ptr_1 =  swi(0x21);
                            (*fn_ptr_1)();
                        }
                        uVar7 = 0x0;
                        fn_ptr_1 =  swi(0x21);
                        (*fn_ptr_1)();
                        uVar5 = uVar2;
                        uVar8 = uStack14;
                    }
                } else {
                    if ((param_4 & 0x3) == 0x0) {
                        fn_ptr_1 =  swi(0x21);
                        (*fn_ptr_1)();
                        fn_ptr_1 =  swi(0x21);
                        (*fn_ptr_1)();
                    // TODO: goto LAB_1000_38e3;
                    }
                    uVar7 = 0x0;
                    fn_ptr_1 =  swi(0x21);
                    (*fn_ptr_1)();
                    uVar5 = uVar2;
                    uVar8 = param_1;
                }
            }
        // TODO: goto LAB_1000_3973;
        }
        fn_ptr_1 =  swi(0x21);
        (*fn_ptr_1)();
        uVar2 = 0x1100;
    }
    bVar10 = true;//
//    LAB_1000_299d:
    if (bVar10) {
        pass1_1000_29b5(uVar2);
        uVar2 = 0xffff;
    }
    return uVar2;
}
pub fn pass1_1000_39e1()
{
}



// WARNING: Unable to track spacebase fully for stack
// WARNING: Removing unreachable block (ram,0x10003afe)
// WARNING: Removing unreachable block (ram,0x10003a40)
// WARNING: Removing unreachable block (ram,0x10003b7e)
// WARNING: Unable to use type for symbol uVar5

u8 *mixed_dos3_call_1000_39f2(u8 *param_1,
                              char *param_2,
                              u8 *param_3)
{
    let mut pbVar2: *mut u8;
    let mut puVar3: *mut u8;
    let mut uVar4: u16;
    code *pcVar5;
    let mut puVar6: *mut u8;
    let mut uVar6: u16;
    let mut uVar7: u16;
    let mut piVar7: *mut i16;
    let mut piVar8: *mut i16;
    let mut piVar9: *mut i16;
    let mut pbVar10: *mut u8;
    let mut iVar10: i16;
    let mut puVar11: *mut u8;
    let mut pbVar12: *mut u8;
    let mut puVar12: *mut u8;
    let mut unaff_SI: *mut u8;
    let mut pbVar13: *mut u8;
    let mut uVar14: u16;
    let mut unaff_CS: u16;
    let mut uVar15: u8;
    let mut bVar16: u8;
    let mut cVar17: u8;
    let mut in_AF: u8;
    let mut bVar18: bool;
    let mut cVar19: u8;
    let mut cVar20: u8;
    let mut uVar21: u32;
    let mut pcVar22: *mut c_char;
    let mut puVar23: *mut u8;
    let mut in_stack_0000fff6: i16;
    let mut piStack8: *mut i16;
    let mut piStack6: *mut i16;
    let mut puVar2: *mut u8;
    let mut uVar5: u16;
    let mut pbVar1: *mut u8;

    puVar6 =  u16_1050_5f8a;
    if ((u16_1050_61ec != 0x0)
        && (puVar6 = PTR_s_ed_in_Op_Op_1050_0028_1050_5f8e, param_1 <  ( &u16_1050_0002 + 0x1))) {
        param_1 =  u16_1050_5f8a;
    }
    if (puVar6 <= param_1) {
        uVar15 = true;
        puVar6 =  0x900;
    // TODO: goto LAB_1000_299d;
    }
    puVar12 = param_1;
    puVar23 =  u16_1050_5f8a;
    if ((param_1[0x5f90] & 0x20) != 0x0) {
        uVar15 = false;
        pcVar5 =  swi(0x21);
        puVar6 =  (*pcVar5)();
        unaff_CS = 0x1000;
        if ( uVar15) {
        // TODO: goto LAB_1000_299d;
        }
    }
    pbVar12 =  param_2;
    if ((puVar12[0x5f90] & 0x80) == 0x0) {//
//        LAB_1000_3acf:
        uVar15 = false;
        puVar6 = param_3;
        if (param_3 != NULL) {
            uVar15 = puVar12 < puVar23;
            if ( uVar15) {
                uVar15 = 0x0;
                pcVar5 =  swi(0x21);
                uVar21 = (*pcVar5)();
            } else {
                piVar8 = pass1_1000_55b1( &DAT_1050_1050,
                                         in_stack_0000fff6);
                uVar21 = CONCAT22(pbVar12,
                                  piVar8);
            }
            puVar6 =  uVar21;
            if ( uVar15) {
                puVar6 =  CONCAT11(0x9,
                                          uVar21);
            } else {
                uVar15 = false;
                if (puVar6 == NULL) {
                    if (((puVar12[0x5f90] & 0x40) == 0x0) || (( uVar21 >> 0x10) != '\x1a')) {
                        uVar15 = true;
                        puVar6 =  0x1c00;
                    } else {
                        uVar15 = false;
                    }
                }
            }
        }
    } else {
        in_stack_0000fff6 =  &DAT_1050_1050;
        bVar18 = true;
        piStack6 = NULL;
        piStack8 = NULL;
        piVar9 =  param_3;
        pbVar13 = pbVar12;
        if (param_3 != NULL) {
            loop {
                if (piVar9 == NULL) {
                    break;
                }
                piVar9 =  ( piVar9 + -0x1);
                pbVar1 = pbVar13;
                pbVar13 = pbVar13 + 0x1;
                bVar18 = *pbVar1 == '\n';
            } while (!bVar18);
            puVar23 = unaff_SI;
            if (!bVar18) {
            // TODO: goto LAB_1000_3acf;
            }
            pcVar22 = param_2;
            uVar6 = pass1_1000_3bac();
            pcVar22 =  ( pcVar22 >> 0x10);
            pbVar10 =  pcVar22;
            if (uVar6 < 0xa9) {
                exit_1000_25f2(-0x4,
                               0x3ad9,
                               unaff_CS,
                               pcVar22);
                if ( pbVar13 -  pbVar10 == 0x0) {
                    return unaff_SI;
                }
                bVar16 = param_1 < unaff_SI;
                uVar4 =  param_1 -  unaff_SI;
                cVar20 =  uVar4 < 0x0;
                cVar19 = uVar4 == 0x0;
                cVar17 = (POPCOUNT(uVar4 & 0xff) & 0x1) == 0x0;
                if ( bVar16) {
                    bVar16 = 0x0;
                    cVar20 = '\0';
                    cVar19 = '\x01';
                    cVar17 = '\x01';
                    pcVar5 =  swi(0x21);
                    piVar7 =  (*pcVar5)( &DAT_1050_1050,
                                               piVar9,
                                               puVar12);
                } else {
                    piVar7 = pass1_1000_55b1( pbVar13 -  pbVar10,
                                              &DAT_1050_1050);
                }
                if (! bVar16) {
                    bVar16 = piVar9 < piVar7;
                    uVar4 =  piVar9 -  piVar7;
                    cVar20 =  uVar4 < 0x0;
                    cVar19 = uVar4 == 0x0;
                    cVar17 = (POPCOUNT(uVar4 & 0xff) & 0x1) == 0x0;
                    piStack6 = piVar7;
                    if ( bVar16 ||  cVar19) {
                        return unaff_SI;
                    }
                }
                uVar4 =
                      (cVar20 << 0x7 | cVar19 << 0x6 | in_AF << 0x4 | cVar17 << 0x2 | 0x2U | bVar16) << 0x8;
                puVar6 =  ( piVar7 & 0xff | uVar4);
                if (piStack6 == NULL) {
                    uVar15 = (uVar4 & 0x100) != 0x0;
                    if ( uVar15) {
                        puVar6 =  CONCAT11(0x9,
                                                  ( piVar7 & 0xff));
                    } else if (((param_1[0x5f90] & 0x40) == 0x0) || (*param_2 != '\x1a')) {
                        uVar15 = true;
                        puVar6 =  0x1c00;
                    } else {
                        uVar15 = false;
                    }
                // TODO: goto LAB_1000_299d;
                }
            } else {
                puVar6 = &stack0xfff0;
                iVar10 = 0x200;
                if (uVar6 < 0x228) {
                    iVar10 = 0x80;
                }
                iVar10 = -iVar10;
                puVar11 = &stack0xfff0 + iVar10;
                puVar12 = &stack0xfff0 + iVar10;
                (&stack0xffee + iVar10) =  &DAT_1050_1050;
                uVar14 = (&stack0xffee + iVar10);
                loop {
                    pbVar2 = pbVar12;
                    pbVar12 = pbVar12 + 0x1;
                    bVar16 = *pbVar2;
                    uVar5 = uVar6 & 0xff00;
                    uVar6 = uVar5 | bVar16;
                    if (bVar16 == 0xa) {
                        uVar7 = CONCAT11( (uVar5 >> 0x8),
                                         0xd);
                        if (puVar12 == puVar6) {
                            (&stack0xffee + iVar10) = 0x3abd;
                            uVar7 = mixed_dos3_call_1000_3ad9(uVar7,
                                                               puVar11,
                                                              param_3,
                                                              (&stack0xfff0 + iVar10));
                        }
                        puVar3 = puVar12;
                        puVar12 = puVar12 + 0x1;
                        *puVar3 =  uVar7;
                        uVar6 = CONCAT11( (uVar7 >> 0x8),
                                         0xa);
                        piStack8 =  ( piStack8 + 0x1);
                    }
                    if (puVar12 == puVar6) {
                        (&stack0xffee + iVar10) = 0x3ac8;
                        uVar6 = mixed_dos3_call_1000_3ad9(uVar6,
                                                           puVar11,
                                                          param_3,
                                                          (&stack0xfff0 + iVar10));
                    }
                    puVar2 = puVar12;
                    puVar12 = puVar12 + 0x1;
                    *puVar2 =  uVar6;
                    param_3 = param_3 + -0x1;
                } while (param_3 != NULL);
                (&stack0xffee + iVar10) = 0x3ab1;
                mixed_dos3_call_1000_3ad9(uVar6,
                                           puVar11,
                                          0x0,
                                          (&stack0xfff0 + iVar10));
            }
        }
        uVar15 = piStack6 < piStack8;
        puVar6 =  ( piStack6 -  piStack8);
    }//
//    LAB_1000_299d:
    if ( uVar15) {
        pass1_1000_29b5(puVar6);
        puVar6 =  0xffff;
    }
    return puVar6;
}



// WARNING: Unable to track spacebase fully for stack
// WARNING: Removing unreachable block (ram,0x10003afe)

u16 mixed_dos3_call_1000_3ad9(mut param_1: u16 ,
                              mut param_2: i16,
                              param_3: *mut i16,
                              mut param_4: u16 )
{
    let mut uVar2: u16;
    code *pcVar3;
    let mut uVar4: u16;
    let mut piVar5: *mut i16;
    let mut uVar5: u16;
    let mut unaff_BP: i16;
    let mut unaff_DI: i16;
    let mut bVar5: u8;
    let mut bVar6: bool;
    let mut cVar7: u8;
    let mut in_AF: u8;
    let mut cVar8: u8;
    let mut cVar9: u8;
    let mut puVar1: *mut u16;
    let mut piVar1: *mut i16;
    let mut puVar2: *mut u16;
    let mut uVar1: u16;

    if (unaff_DI - param_2 == 0x0) {
        return param_4;
    }
    uVar5 = (unaff_BP + 0x6);
    puVar1 =  (unaff_BP + -0xc);
    bVar5 = uVar5 < *puVar1;
    uVar1 = uVar5 - *puVar1;
    cVar9 =  uVar1 < 0x0;
    cVar8 = uVar1 == 0x0;
    cVar7 = (POPCOUNT(uVar1 & 0xff) & 0x1) == 0x0;
    if ( bVar5) {
        bVar5 = 0x0;
        cVar9 = '\0';
        cVar8 = '\x01';
        cVar7 = '\x01';
        pcVar3 =  swi(0x21);
        piVar5 =  (*pcVar3)();
    } else {
        piVar5 = pass1_1000_55b1(unaff_DI - param_2,
                                  &DAT_1050_1050);
    }
    if (! bVar5) {
        piVar1 =  (unaff_BP + -0x4);
        *piVar1 = *piVar1 +  piVar5;
        bVar5 = param_3 < piVar5;
        uVar2 =  param_3 -  piVar5;
        cVar9 =  uVar2 < 0x0;
        cVar8 = uVar2 == 0x0;
        cVar7 = (POPCOUNT(uVar2 & 0xff) & 0x1) == 0x0;
        if ( bVar5 ||  cVar8) {
            return param_4;
        }
    }
    uVar2 =   (cVar9 << 0x7 | cVar8 << 0x6 | in_AF << 0x4 | cVar7 << 0x2 | 0x2 | bVar5) << 0x8;
    uVar4 =  piVar5 & 0xff | uVar2;
    if ((unaff_BP + -0x4) == 0x0) {
        bVar6 = (uVar2 & 0x100) != 0x0;
        if (bVar6) {
            uVar4 = CONCAT11(0x9,
                              ( piVar5 & 0xff));
        } else if (((* (uVar5 + 0x5f90) & 0x40) == 0x0) || (** (unaff_BP + 0x8) != '\x1a')) {
            bVar6 = true;
            uVar4 = 0x1c00;
        } else {
            bVar6 = false;
        }
    } else {
        uVar2 = (unaff_BP + -0x4);
        puVar2 =  (unaff_BP + -0x6);
        bVar6 = uVar2 < *puVar2;
        uVar4 = uVar2 - *puVar2;
    }
    if (bVar6) {
        ((unaff_BP + -0xa) + 0x2) = 0x29a2;
        pass1_1000_29b5(uVar4);
        uVar4 = 0xffff;
    }
    return uVar4;
}

i16 pass1_1000_3bac()
{
    let mut iVar1: i16;

    if (PTR_LOOP_1050_5f48 < &stack0x0004) {
        iVar1 = -( PTR_LOOP_1050_5f48 -  &stack0x0004);
    } else {
        iVar1 = 0x0;
    }
    return iVar1;
}
pub fn pass1_1000_3bc0(mut param_1: i16,
                     mut param_2: i16)
{
    let mut piVar1: *mut i16;
    let mut u_var2: u16;
    let mut uVar3: u16;
    let mut u_var4: u16;
    let mut u_var5: u16;
    let mut i_var6: i16;
    let mut unaff_si: *mut u16;
    let mut pu_var7: *mut u16;
    let mut unaff_di: u16;
    let mut b_var8: bool;
    let mut u_var9: u32;

    if ((* (param_2 + 0x2) & 0x1) != 0x0) {
        pass1_1000_3cb7(param_2);
        u_var5 = *unaff_si;
        if ((u_var5 & 0x1) != 0x0) {
            param_1 = (param_1 - u_var5) + -0x1;
        }
        u_var5 = (param_2 + 0x4);
        if (u_var5 != 0x0) {
            u_var4 = param_1 + 0x2U + u_var5;
            if (!CARRY2(param_1 + 0x2U,
                        u_var5)) {
                uVar3 = block_1000_2000::pass1_1000_29dc( &DAT_1050_1050);
                u_var5 = &PTR_LOOP_1050_6066;
                if (u_var5 == 0x1000) {
                // TODO: goto LAB_1000_3c12;
                }
                u_var2 = 0x8000;
                while (u_var5 <= u_var2) {
                    u_var2 >>= 0x1;
                    if (u_var2 == 0x0) {
                    // TODO: goto LAB_1000_3c2b;
                    }
                }
                if (u_var2 < 0x8) {
                // TODO: goto LAB_1000_3c2b;
                }
                u_var5 = u_var2 << 0x1;
            // TODO: goto LAB_1000_3c12;
            }
            u_var2 = 0x0;
            u_var5 = 0xfff0;
            if (u_var4 == 0x0) {
                while (true) {
                    b_var8 = false;
                    u_var9 = mixed_mem_op_1000_3c51(u_var2,
                                                    u_var4,
                                                    param_2,
                                                    0x3c23,
                                                    unaff_di);
                    if (!b_var8) {
                        break;
                    }
                    if (u_var5 == 0xfff0) {
                        return;
                    }//
//                    LAB_1000_3c2b:
                    u_var5 = 0x10;//
//                    LAB_1000_3c12:
                    u_var5 -= 0x1;
                    u_var2 = u_var5 + u_var4;
                    if (CARRY2(u_var5,
                               u_var4)) {
                        u_var2 = 0x0;
                    }
                    u_var5 = ~u_var5;
                    u_var2 &= u_var5;
                }
                i_var6 =  u_var9 - (param_2 + 0x4);
                (param_2 + 0x4) = u_var9;
                 (param_2 + 0xa) = unaff_si;
                piVar1 =  (param_2 + 0xc);
                *piVar1 = i_var6 + -0x1;
                pu_var7 =  ( piVar1 + i_var6);
                *pu_var7 = 0xfffe;
                 (param_2 + 0xc) = pu_var7;
            }
        }
    }
    return;
}

/*
Unable to decompile 'mixed_mem_op_1000_3c51'
Cause:
Low-level Error: Overlapping input varnodes
*/
pub fn pass1_1000_3cb7(mut param_1: i16)
{
    let mut u_var1: u16;
    let mut pu_var2: *mut u16;

    pu_var2 =  (param_1 + 0xa);
    if (pu_var2 ==  (param_1 + 0xc)) {
        pu_var2 =  (param_1 + 0x8);
    }
    while (true) {
        u_var1 = *pu_var2;
        if (u_var1 == 0xfffe) {
            break;
        }
        pu_var2 =  ( pu_var2 + (u_var1 & 0xfffe) + 0x2);
    }
    return;
}
pub fn pass1_1000_3cd8(mut param_1: u16 ,
                     mut param_2: u16 )
{
    free_mem_1000_407a(param_1,
                       param_2);
    return;
}

u16 *pass1_1000_3cea(mut param_1: u32,
                     char *param_2)
{
    let mut pUVar1: *mut u16;
    let mut pcVar2: *mut c_char;
    let mut pUVar3: *mut u16;
    let mut iVar4: i16;
    let mut uVar5: u16;
    let mut uVar6: u16;
    let mut pUVar7: *mut u16;
    let mut pcVar8: *mut c_char;
    let mut pUVar9: *mut u16;
    let mut pUVar10: *mut u16;
    let mut uVar11: u16;
    let mut uVar12: u16;
    let mut bVar13: bool;

    uVar11 =  (param_1 >> 0x10);
    bVar13 = true;
    iVar4 = -0x1;
    pUVar7 =  param_1;
    loop {
        if (iVar4 == 0x0) {
            break;
        }
        iVar4 += -0x1;
        pUVar1 = pUVar7;
        pUVar7 =  ( pUVar7 + 0x1);
        bVar13 = pUVar1 == '\0';
    } while (!bVar13);
    pUVar10 =  ( pUVar7 + -0x1);
    uVar12 =  ( param_2 >> 0x10);
    pcVar8 =  param_2;
    uVar5 = 0xffff;
    loop {
        if (uVar5 == 0x0) {
            break;
        }
        uVar5 -= 0x1;
        pcVar2 = pcVar8;
        pcVar8 = pcVar8 + 0x1;
        bVar13 = *pcVar2 == '\0';
    } while (!bVar13);
    uVar5 = ~uVar5;
    if (!bVar13) {
        pcVar8 = pcVar8 + -uVar5;
        uVar5 += 0x1;
    }
    pUVar9 =  (pcVar8 + -uVar5);
    if (uVar5 == 0x0) {
        pUVar1 = pUVar9;
        pUVar9 = pUVar9 + 0x1;
        *pUVar10 = *pUVar1;
        uVar5 = 0xfffe;
        pUVar10 =  ( pUVar7 + 0x1);
    } else if (( pUVar9 & 0x1) != 0x0) {
        pUVar1 = pUVar9;
        pUVar9 =  ( pUVar9 + 0x1);
        * pUVar10 = * pUVar1;
        uVar5 -= 0x1;
        pUVar10 = pUVar7;
    }
    for (uVar6 = uVar5 >> 0x1; uVar6 != 0x0; uVar6 -= 0x1) {
        pUVar3 = pUVar10;
        pUVar10 = pUVar10 + 0x1;
        pUVar1 = pUVar9;
        pUVar9 = pUVar9 + 0x1;
        *pUVar3 = *pUVar1;
    }
    for (uVar5 =  ((uVar5 & 0x1) != 0x0); uVar5 != 0x0; uVar5 -= 0x1) {
        pUVar3 = pUVar10;
        pUVar10 =  ( pUVar10 + 0x1);
        pUVar1 = pUVar9;
        pUVar9 =  ( pUVar9 + 0x1);
        * pUVar3 = * pUVar1;
    }
    return  param_1;
}
pub fn unk_str_op_1000_3d3e(char *param_1,
                          char *in_string_2)
{
    let mut pu_var4: *mut u16;
    let mut pu_var5: *mut u16;
    let mut u_var6: u16;
    let mut u_var7: u16;
    let mut l_string_2: *mut c_char;
    let mut puVar6: *mut c_char;
    let mut puVar7: *mut c_char;
    let mut u_var8: u16;
    let mut l_string_1: *mut c_char;
    let mut l_b_var8: bool;
    let mut puVar3: *mut c_char;
    let mut puVar2: *mut c_char;
    let mut puVar1: *mut c_char;

    l_string_1 =  ( in_string_2 >> 0x10);
    l_string_2 =  in_string_2;
    l_b_var8 = true;
    u_var6 = 0xffff;
    puVar6 = l_string_2;
    loop {
        if (u_var6 == 0x0) {
            break;
        }
        u_var6 -= 0x1;
        puVar1 = puVar6;
        puVar6 = puVar6 + 0x1;
        l_b_var8 = *puVar1 == '\0';
    } while (!l_b_var8);
    u_var6 = ~u_var6;
    u_var8 =  ( param_1 >> 0x10);
    puVar7 =  param_1;
    if (l_b_var8) {
        if (( param_1 & 0x1) != 0x0) {
            puVar7 = puVar7 + 0x1;
            l_string_2 = l_string_2 + 0x1;
            *param_1 = *in_string_2;
            u_var6 -= 0x1;
        }
    } else {
        puVar7 = puVar7 + 0x2;
        l_string_2 = l_string_2 + 0x2;
        param_1 = in_string_2;
        u_var6 -= 0x1;
    }
    for (uVar7 = uVar6 >> 0x1; uVar7 != 0x0; uVar7 -= 0x1) {
        pu_var5 =  puVar7;
        puVar7 =  ( puVar7 + 0x2);
        pu_var4 =  l_string_2;
        l_string_2 =  ( l_string_2 + 0x2);
        *pu_var5 = *pu_var4;
    }
    for (uVar6 =  ((uVar6 & 0x1) != 0x0); uVar6 != 0x0; uVar6 -= 0x1) {
        pu_var5 =  puVar7;
        puVar7 =  ( puVar7 + 0x1);
        pu_var4 =  l_string_2;
        l_string_2 =  ( l_string_2 + 0x1);
        * pu_var5 = * pu_var4;
    }
    return;
}

i16 pass1_1000_3d7a(char *param_1,
                    char *param_2)
{
    let mut pbVar2: *mut u8;
    let mut pbVar3: *mut u8;
    let mut iVar4: i16;
    let mut uVar5: u16;
    let mut string_4: *mut c_char;
    let mut string_1: *mut c_char;
    let mut string_2: *mut c_char;
    let mut string_6: *mut c_char;
    let mut uVar6: u16;
    let mut bool_1: bool;
    let mut bool_2: bool;
    let mut pbVar4: *mut c_char;
    let mut pbVar1: *mut c_char;
    let mut string_3: *mut c_char;

    string_1 =  param_1;
    uVar6 =  ( param_2 >> 0x10);
    string_2 =  param_2;
    iVar4 = 0x0;
    uVar5 = 0xffff;
    loop {
        if (uVar5 == 0x0) {
            break;
        }
        uVar5 -= 0x1;
        string_3 = string_2;
        string_2 = string_2 + 0x1;
    } while (*string_3 != '\0');
    string_4 =  ~uVar5;
    bool_1 = string_2 < string_4;
    string_6 = string_2 + - string_4;
    bool_2 = string_6 == NULL;
    loop {
        if (string_4 == NULL) {
            break;
        }
        string_4 = string_4 + -0x1;
        pbVar3 =  string_6;
        string_6 =  ( string_6 + 0x1);
        pbVar2 =  string_1;
        string_1 =  ( string_1 + 0x1);
        bool_1 = *pbVar2 < *pbVar3;
        bool_2 = *pbVar2 == *pbVar3;
    } while (bool_2);
    if (!bool_2) {
        iVar4 = (0x1 -  bool_1) -  (bool_1 != 0x0);
    }
    return iVar4;
}

u16 str_op_1000_3da4(char *param_1)
{
    let mut pcVar1: *mut c_char;
    let mut uVar2: u16;
    let mut pcVar3: *mut c_char;
    let mut bVar4: bool;

    pcVar3 =  param_1;
    bVar4 = true;
    uVar2 = 0xffff;
    loop {
        if (uVar2 == 0x0) {
            break;
        }
        uVar2 -= 0x1;
        pcVar1 = pcVar3;
        pcVar3 = pcVar3 + 0x1;
        bVar4 = *pcVar1 == '\0';
    } while (!bVar4);
    uVar2 = ~uVar2;
    if (bVar4) {
        uVar2 -= 0x1;
    }
    return uVar2;
}

uchar str_op_1000_3dbe(char *param_1,
                       char *param_2,
                       mut param_3: u16 )
{
    let mut pcVar1: *mut c_char;
    let mut cVar2: u8;
    let mut pcVar3: *mut c_char;
    let mut pcVar4: *mut c_char;
    let mut uVar5: u16;

    uVar5 =  ( param_1 >> 0x10);
    pcVar4 =  param_1;
    pcVar3 =  param_2;
    if (param_3 != 0x0) {
        loop {
            pcVar1 = pcVar3;
            pcVar3 = pcVar3 + 0x1;
            cVar2 = *pcVar1;
            if (cVar2 == '\0') {
                break;
            }
            pcVar1 = pcVar4;
            pcVar4 = pcVar4 + 0x1;
            *pcVar1 = cVar2;
            param_3 -= 0x1;
        } while (param_3 != 0x0);
        for (; param_3 != 0x0; param_3 -= 0x1) {
            pcVar1 = pcVar4;
            pcVar4 = pcVar4 + 0x1;
            *pcVar1 = '\0';
        }
    }
    return (uchar) param_1;
}

u16 pass1_1000_3de8(char *param_1,
                    char *param_2,
                    mut param_3: u16 ,
                    u16_t param_4,
                    u16_t param_5)
{
    let mut pbVar1: *mut u8;
    let mut pcVar2: *mut c_char;
    let mut pcVar3: *mut c_char;
    let mut bVar4: u8;
    let mut uVar5: u16;
    let mut iVar6: i16;
    let mut pcVar7: *mut c_char;
    let mut pcVar8: *mut c_char;
    let mut uVar9: u16;
    let mut uVar10: u16;
    let mut bVar11: bool;

    if (param_3 != 0x0) {
        uVar9 =  ( param_1 >> 0x10);
        pcVar8 =  param_1;
        uVar5 = param_3;
        pcVar7 = pcVar8;
        loop {
            if (uVar5 == 0x0) {
                break;
            }
            uVar5 -= 0x1;
            pcVar2 = pcVar7;
            pcVar7 = pcVar7 + 0x1;
        } while (*pcVar2 != '\0');
        iVar6 = param_3 - uVar5;
        uVar10 =  ( param_2 >> 0x10);
        pcVar7 =  param_2;
        loop {
            if (iVar6 == 0x0) {
                break;
            }
            iVar6 += -0x1;
            pcVar3 = pcVar8;
            pcVar8 = pcVar8 + 0x1;
            pcVar2 = pcVar7;
            pcVar7 = pcVar7 + 0x1;
        } while (*pcVar2 == *pcVar3);
        bVar4 = pcVar7[-0x1];
        uVar5 = 0x0;
        pbVar1 =  (pcVar8 + -0x1);
        bVar11 = bVar4 == *pbVar1;
        if (bVar4 < *pbVar1 || bVar11) {
            if (bVar11) {
                return 0x0;
            }
            uVar5 = 0xfffe;
        }
        param_3 = ~uVar5;
    }
    return param_3;
}

i16 pass1_1000_3e2c(mut param_1: u32)
{
    let mut pbVar1: *mut u8;
    let mut bVar2: u8;
    let mut bVar3: u8;
    let mut iVar4: i16;
    let mut pbVar5: *mut u8;
    let mut uVar6: u16;

    uVar6 =  (param_1 >> 0x10);
    pbVar5 =  param_1;
    iVar4 = 0x0;
    loop {
        loop {
            pbVar1 = pbVar5;
            pbVar5 = pbVar5 + 0x1;
            bVar2 = *pbVar1;
        } while (bVar2 == 0x20);
    } while (bVar2 == 0x9);
    if ((bVar2 != 0x2d) && (bVar3 = bVar2, bVar2 != 0x2b)) {
    // TODO: goto LAB_1000_3e4d;
    }
    loop {
        pbVar1 = pbVar5;
        pbVar5 = pbVar5 + 0x1;
        bVar3 = *pbVar1;//
//        LAB_1000_3e4d:
        if ((0x39 < bVar3) || (bVar3 < 0x30)) {
            break;
        }
        iVar4 = iVar4 * 0xa +   (bVar3 - 0x30);
    }
    if (bVar2 == 0x2d) {
        iVar4 = -iVar4;
    }
    return iVar4;
}

i16 pass1_1000_3e2c(mut param_1: u32)
{
    let mut pbVar1: *mut u8;
    let mut bVar2: u8;
    let mut bVar3: u8;
    let mut iVar4: i16;
    let mut pbVar5: *mut u8;
    let mut uVar6: u16;

    uVar6 =  (param_1 >> 0x10);
    pbVar5 =  param_1;
    iVar4 = 0x0;
    loop {
        loop {
            pbVar1 = pbVar5;
            pbVar5 = pbVar5 + 0x1;
            bVar2 = *pbVar1;
        } while (bVar2 == 0x20);
    } while (bVar2 == 0x9);
    if ((bVar2 != 0x2d) && (bVar3 = bVar2, bVar2 != 0x2b)) {
    // TODO: goto LAB_1000_3e4d;
    }
    loop {
        pbVar1 = pbVar5;
        pbVar5 = pbVar5 + 0x1;
        bVar3 = *pbVar1;//
//        LAB_1000_3e4d:
        if ((0x39 < bVar3) || (bVar3 < 0x30)) {
            break;
        }
        iVar4 = iVar4 * 0xa +   (bVar3 - 0x30);
    }
    if (bVar2 == 0x2d) {
        iVar4 = -iVar4;
    }
    return iVar4;
}

i16 pass1_1000_3e2c(mut param_1: u32)
{
    let mut pbVar1: *mut u8;
    let mut bVar2: u8;
    let mut bVar3: u8;
    let mut iVar4: i16;
    let mut pbVar5: *mut u8;
    let mut uVar6: u16;

    uVar6 =  (param_1 >> 0x10);
    pbVar5 =  param_1;
    iVar4 = 0x0;
    loop {
        loop {
            pbVar1 = pbVar5;
            pbVar5 = pbVar5 + 0x1;
            bVar2 = *pbVar1;
        } while (bVar2 == 0x20);
    } while (bVar2 == 0x9);
    if ((bVar2 != 0x2d) && (bVar3 = bVar2, bVar2 != 0x2b)) {
    // TODO: goto LAB_1000_3e4d;
    }
    loop {
        pbVar1 = pbVar5;
        pbVar5 = pbVar5 + 0x1;
        bVar3 = *pbVar1;//
//        LAB_1000_3e4d:
        if ((0x39 < bVar3) || (bVar3 < 0x30)) {
            break;
        }
        iVar4 = iVar4 * 0xa +   (bVar3 - 0x30);
    }
    if (bVar2 == 0x2d) {
        iVar4 = -iVar4;
    }
    return iVar4;
}

u8 *pass1_1000_3e82(mut param_1: u16 ,
                    u8 *param_2,
                    mut param_3: u16 )
{
    let mut pbVar1: *mut u8;
    let mut uVar2: u32;
    let mut bVar3: u8;
    let mut uVar5: u16;
    let mut uVar6: u16;
    let mut uVar7: u16;
    let mut pbVar8: *mut u8;
    let mut pbVar9: *mut u8;
    let mut pbVar10: *mut u8;
    let mut pbVar11: *mut u8;
    let mut uVar12: u16;
    let mut bVar13: bool;
    let mut cVar4: u8;

    uVar6 = 0x0;
    if (param_3 == 0xa) {
        uVar6 =  param_1 >> 0xf;
    }
    uVar12 =  ( param_2 >> 0x10);
    pbVar9 =  param_2;
    pbVar10 = pbVar9;
    pbVar8 = pbVar9;
    if ((param_3 == 0xa) && ( uVar6 < 0x0)) {
        pbVar10 = pbVar9 + 0x1;
        *param_2 = '-';
        bVar13 = param_1 != 0x0;
        param_1 = -param_1;
        uVar6 = -(uVar6 + bVar13);
        pbVar8 = pbVar10;
    }
    loop {
        uVar7 = 0x0;
        uVar5 = uVar6;
        if (uVar6 != 0x0) {
            uVar5 = uVar6 / param_3;
            uVar7 = uVar6 % param_3;
        }
        uVar2 = CONCAT22(uVar7,
                         param_1);
        param_1 =  (uVar2 / param_3);
        cVar4 =  (uVar2 %  param_3);
        bVar3 = cVar4 + 0x30;
        if (0x39 < bVar3) {
            bVar3 = cVar4 + 0x57;
        }
        pbVar11 = pbVar10 + 0x1;
        *pbVar10 = bVar3;
        uVar6 = uVar5;
        pbVar10 = pbVar11;
    } while ((uVar5 | param_1) != 0x0);
    *pbVar11 = 0x0;
    loop {
        pbVar11 = pbVar11 + -0x1;
        pbVar1 = pbVar11;
        bVar3 = *pbVar1;
        *pbVar1 = *pbVar8;
        *pbVar8 = bVar3;
        pbVar10 = pbVar8 + 0x2;
        pbVar8 = pbVar8 + 0x1;
    } while (pbVar10 < pbVar11);
    return pbVar9;
}
pub fn fatal_app_exit_1000_3e9e()
{
    FatalAppExit16(s_ABNORMAL_PROGRAM_TERMINATION_1050_6544,
                   0x0);
    return;
}

i16 pass1_1000_3ec0(mut param_1: u16 ,
                    mut param_2: u16 )
{
    let mut uVar1: u16;
    let mut uVar2: u16;
    let mut uVar3: u16;
    let mut unaff_SI: u16;
    let mut uVar4: u16;
    let mut puVar4: *mut u32;

    puVar4 =  CONCAT22(PTR_LOOP_1050_5fc0,
                              PTR_LOOP_1050_5fbe);
    if ((( PTR_LOOP_1050_5fc0 |  PTR_LOOP_1050_5fbe) != 0x0) && ((param_2 | param_1) != 0x0)) {
        uVar1 = str_op_1000_3da4( CONCAT22(param_2,
                                                   param_1));
        loop {
            uVar4 = (u16_t)( puVar4 >> 0x10);
            uVar3 = (u16_t) puVar4;
            if (((uVar3 + 0x2) | puVar4) == 0x0) {
                break;
            }
            uVar2 = str_op_1000_3da4( CONCAT22((uVar3 + 0x2),
                                                       puVar4));
            if (((uVar1 < uVar2) && (( *puVar4 + uVar1) == '=')) && (uVar2 =
                                                                                        pass1_1000_3de8( CONCAT22((uVar3
                                                                                                                              + 0x2),
                                                                                                                          puVar4),
                                                                                                         CONCAT22(param_2,
                                                                                                                          param_1),
                                                                                                        uVar1,
                                                                                                        unaff_SI,
                                                                                                        uVar3), uVar2
                == 0x0)) {
                return puVar4 + uVar1 + 0x1;
            }
            puVar4 =  ( puVar4 & 0xffff0000 |  (uVar3 + 0x4));
        }
    }
    return 0x0;
}

i16 pass1_1000_3f5c()
{
    let mut uVar1: u16;
    let mut puVar2: *mut u16;
    let mut iVar3: i16;

    iVar3 = 0x0;
    if (u16_1050_61ec == 0x0) {
        puVar2 =  &PTR_LOOP_1050_6210;
    } else {
        puVar2 =  0x6234;
    }
    for (; puVar2 <= PTR_LOOP_1050_5ff0; puVar2 = puVar2 + 0x6) {
        uVar1 = pass1_1000_2a00(puVar2);
        if (uVar1 != 0xffff) {
            iVar3 += 0x1;
        }
    }
    return iVar3;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

u16 sys_1000_3f9c(char *param_1,
                  char *param_2,
                  mut param_3: u16 )
{
    let mut pcVar1: *mut c_char;
    let mut uVar2: u16;
    let mut unaff_CS: u16;
    let mut local_4: u16;

    PTR_LOOP_1050_68b2._0_1_ = 0x42;
    _u16_1050_68a8 = param_1;
    PTR_LOOP_1050_68ac =  0x7fff;
    _PTR_LOOP_1050_68ae = param_1;
    uVar2 = FUN_1000_30b4();
    pcVar1 = _u16_1050_68a8;
    PTR_LOOP_1050_68ac = PTR_LOOP_1050_68ac + -0x1;
    if ( PTR_LOOP_1050_68ac < 0x0) {
        mem_1000_2bb6(param_1,
                      0x0,
                       &u16_1050_68a8);
    } else {
        _u16_1050_68a8 =  ( _u16_1050_68a8 & 0xffff0000 |  (u16_1050_68a8 + 0x1));
        *pcVar1 = '\0';
    }
    PTR_LOOP_1050_68b0 =  ( _PTR_LOOP_1050_68ae >> 0x10);
    return uVar2;
}
