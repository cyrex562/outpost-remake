pub mod a;
pub mod b;
pub mod c;
pub mod d;
pub mod e;
pub mod f;
pub mod g;
pub mod h;
pub mod i;
pub mod j;
pub mod k;
pub mod l;
pub mod m;
pub mod n;
pub mod o;
pub mod p;
pub mod q;
pub mod r;
pub mod s;
pub mod t;
pub mod u;
pub mod v;
pub mod w;
pub mod x;
pub mod y;
pub mod z;
pub mod aa;
pub mod ab;
pub mod ac;
pub mod ad;
pub mod ae;
pub mod af;
pub mod ag;
pub mod ah;
pub mod ai;
pub mod aj;
pub mod ak;
pub mod al;
pub mod am;
pub mod an;
pub mod ao;
pub mod ap;

pub unsafe fn pass1_1000_0000(mut param_1: *mut u16, mut param_2: *mut u16, mut param_3: u16) {
    let mut pu_var1: *mut u16;
    let mut pu_var2: *mut u16;
    let mut u_var3: u16;

    // for (u_var3 = param_3 >> 0x1; u_var3 != 0; u_var3 -= 1) {
    u_var3 = param_3 >> 0x1;
    while u_var3 != 0 {
        pu_var2 = param_2;
        param_2 = param_2 + 1;
        pu_var1 = param_1;
        param_1 = param_1 + 1;
        *pu_var2 = *pu_var1;
        u_var3 -= 1;
    }
}

pub unsafe fn pass1_1000_201c(mut param_1: u16, mut param_2: u16) {
    let mut u_var1: u16;
    let mut u_var2: u32;
    let mut u_var3: u16;
    let mut bvar4: BOOL16;
    let mut i_var5: i16;
    let mut u_var6: u16;

    if param_1 == 0 {
        (param_2 + 0x6) = 0;
        (param_2 + 0x4) = 0;
    }
    u_var3 = (param_2 + 0x6) | (param_2 + 0x4);
    while u_var3 != 0 {
        bvar4 = pass1_1000_206c((param_2 + 0x4), (param_2 + 0x6));
        if bvar4 == 0 {
            u_var2 = (param_2 + 0x4);
            u_var6 = (u_var2 >> 0x10);
            i_var5 = u_var2;
            u_var1 = (i_var5 + 0x2c);
            (param_2 + 0x4) = (i_var5 + 0x2a);
            (param_2 + 0x6) = u_var1;
        } else {
            mem_op_1000_1b9a(0x1, (param_2 + 0x4), (param_2 + 0x6));
        }
        u_var3 = (param_2 + 0x6) | (param_2 + 0x4);
    }
    return;
}

pub unsafe fn mem_1000_0668() -> u32 {
    let mut u_var1: u32;

    u_var1 = mem_op_1000_0510(0x0, ptr::null_mut());
    return u_var1;
}


pub unsafe fn pass1_1000_09a0(mut param_1: *mut u8) -> *mut u8 {
    let mut puVar1: *mut u8 = null_mut();
    let mut uVar2: u32;

    param_1 = PTR_LOOP_1050_000e;
    if PTR_LOOP_1050_000e.is_null() {
        u32_1050_0004 = 0x1;
    }
    PTR_LOOP_1050_000a = PTR_LOOP_1050_000a -0x1;
    puVar1 = PTR_LOOP_1050_000e;
    PTR_LOOP_1050_000e = param_1;
    if PTR_LOOP_1050_000a.is_null() {
        uVar2 = mem_op_1000_0510(0x1, null_mut());
        // TODO: is this setting the pointer's address?
        // puVar1 = uVar2;
    }
    return puVar1;
}

pub unsafe fn pass1_1000_17e8(param_1: *const u8, param_2: *const u8) -> *const u8 {
    let mut puVar1: *mut u8;

    puVar1 = PTR_LOOP_1050_5f34;
    PTR_LOOP_1050_5f34 = param_1;
    PTR_LOOP_1050_5f36 = param_2;
    return puVar1;
}


pub unsafe fn pass1_1000_183c(mut param_1: u16, mut param_2: u16) -> u16 {
    let mut in_EDX: u32;
    let mut pSVar1: *mut StructD;
    let mut lVar2: i32;

    pSVar1 = (in_EDX & 0xffff0000);
    if ((param_2 * param_1 >> 0x10) != 0) {
        return 0x0;
    }
    if ((PTR_LOOP_1050_5f2e | PTR_LOOP_1050_5f2c) == 0) {
        PTR_LOOP_1050_5f2c = mem_op_1000_160a(pSVar1);
        PTR_LOOP_1050_5f2e = pSVar1;
        if ((PTR_LOOP_1050_5f2e | PTR_LOOP_1050_5f2c) == 0) {
            return 0x0;
        }
    }
    lVar2 = mem_op_1000_0a48(
        0x1,
        (param_2 * param_1),
        0x0,
        CONCAT22(PTR_LOOP_1050_5f2e, PTR_LOOP_1050_5f2c),
    );
    return lVar2;
}


pub unsafe fn pass1_1000_188e(
    mut param_1: u16,
    param_2: *mut u16,
    mut param_3: u16,
    mut param_4: u16,
) -> u16 {
    let mut uVar1: u16;
    let mut lVar2: i32;

    if ((param_3 | param_2) == 0) {
        uVar1 = pass1_1000_180c(param_1, param_4);
        return uVar1;
    }
    if (param_4 == 0) {
        pass1_1000_18d2(param_2, param_3);
        return 0x0;
    }
    lVar2 = pass1_1000_0ed4(0x0, param_4, 0x0, param_2, param_3);
    return lVar2;
}


pub unsafe fn pass1_1000_1f7e(param_1: *mut u16) -> bool {
    let mut cVar1: u8;
    let mut BVar2: bool;
    let mut uVar3: u16;
    let mut iVar4: i16;
    let mut pcVar5: *mut c_char;

    uVar3 = *param_1;
    if (uVar3 == 0xf) {
        //
        //        LAB_1000_1fb6:
        iVar4 = 0x1;
    } else {
        if (uVar3 < 0x10) {
            cVar1 = uVar3;
            if (cVar1 == '\x02') {
                // TODO: goto LAB_1000_1fb6;
            }
            if (('\0' < (cVar1 -0x2)) && ((cVar1 -0x3) < '\x0f')) {
                iVar4 = 0;
                // TODO: goto LAB_1000_1fbe;
            }
        }
        iVar4 = 0;
        uVar3 = 0x1;
    } //
      //    LAB_1000_1fbe:
    pcVar5 = pass1_1000_1fd2(uVar3);
    BVar2 = msg_box_op_1000_214c(0x0, iVar4, pcVar5, (pcVar5 >> 0x10));
    return BVar2;
}

pub unsafe fn pass1_1000_2ba0(param_1: u8) {
    pass1_1000_3024();
    if (u8_1050_5fc9 != '\0') {
        pass1_1000_3f5c();
    }
    return;
}


pub unsafe fn pass1_1000_3113() -> u16 {
    let mut byte1: u8;
    let mut string2: *mut c_char;
    let mut bool3: u8;
    let mut short4: u16;
    let mut short5: i16;

    FUN_1000_3552(0, 0, 0);
    string2 = *(short5 + 0xa);
    byte1 = *string2;
    (short5 + 0xa) = string2 + 1;
    (short5 -0x4) = byte1;
    if ((byte1 != '\0') && (-0x1 < (short5 -0xa))) {
        if ((byte1 - 0x20) < 0x59) {
            bool3 = *((byte1 - 0x20) + 0x5ffe) & 0xf;
        } else {
            bool3 = 0;
        }
        bool3 = *((bool3 * '\x0b' + (short5 -0x7)) + 0x5ffe) >> 0x4;
        *(short5 -0x7) = bool3;
        // WARNING: Could not recover jumptable at 0x1000310e. Too many branches
        // WARNING: Treating indirect jump as call
        short4 = (bool3 * 0x2 + 0x30a4)();
        return short4;
    }
    return (short5 -0xa);
}

pub unsafe fn pass1_1000_311e() -> u16 {
    let mut cVar1: u8;
    let mut pcVar2: *mut c_char;
    let mut bVar3: u8;
    let mut uVar4: u16;
    let mut unaff_BP: i16;

    (unaff_BP -0x12) = 0;
    (unaff_BP -0xc) = 0;
    (unaff_BP -0x14) = 0;
    (unaff_BP -0x6) = 0x20;
    (unaff_BP -0xe) = 0xffff;
    pcVar2 = *(unaff_BP + 0xa);
    cVar1 = *pcVar2;
    (unaff_BP + 0xa) = pcVar2 + 1;
    (unaff_BP -0x4) = cVar1;
    if ((cVar1 != '\0') && (-0x1 < (unaff_BP -0xa))) {
        if ((cVar1 - 0x20) < 0x59) {
            bVar3 = *((cVar1 - 0x20) + 0x5ffe) & 0xf;
        } else {
            bVar3 = 0;
        }
        bVar3 = *((bVar3 * '\x0b' + (unaff_BP -0x7)) + 0x5ffe) >> 0x4;
        *(unaff_BP -0x7) = bVar3;
        // WARNING: Could not recover jumptable at 0x1000310e. Too many branches
        // WARNING: Treating indirect jump as call
        uVar4 = (bVar3 * 0x2 + 0x30a4)();
        return uVar4;
    }
    return (unaff_BP -0xa);
}

pub unsafe fn pass1_1000_3134() -> u16 {
    let mut pbVar1: *mut u8;
    let mut cVar2: u8;
    let mut pcVar3: *mut c_char;
    let mut bVar4: u8;
    let mut uVar5: u16;
    let mut unaff_BP: i16;

    cVar2 = (unaff_BP -0x4);
    if (cVar2 == '-') {
        pbVar1 = (unaff_BP -0x6);
        *pbVar1 = *pbVar1 | 0x4;
    } else if (cVar2 == '+') {
        pbVar1 = (unaff_BP -0x6);
        *pbVar1 = *pbVar1 | 0x1;
    } else if (cVar2 == ' ') {
        pbVar1 = (unaff_BP -0x6);
        *pbVar1 = *pbVar1 | 0x2;
    } else if (cVar2 == '#') {
        pbVar1 = (unaff_BP -0x6);
        *pbVar1 = *pbVar1 | 0x80;
    } else {
        pbVar1 = (unaff_BP -0x6);
        *pbVar1 = *pbVar1 | 0x8;
    }
    pcVar3 = *(unaff_BP + 0xa);
    cVar2 = *pcVar3;
    (unaff_BP + 0xa) = pcVar3 + 1;
    (unaff_BP -0x4) = cVar2;
    if ((cVar2 != '\0') && (-0x1 < (unaff_BP -0xa))) {
        if ((cVar2 - 0x20) < 0x59) {
            bVar4 = *((cVar2 - 0x20) + 0x5ffe) & 0xf;
        } else {
            bVar4 = 0;
        }
        bVar4 = *((bVar4 * '\x0b' + (unaff_BP -0x7)) + 0x5ffe) >> 0x4;
        *(unaff_BP -0x7) = bVar4;
        // WARNING: Could not recover jumptable at 0x1000310e. Too many branches
        // WARNING: Treating indirect jump as call
        uVar5 = (bVar4 * 0x2 + 0x30a4)();
        return uVar5;
    }
    return (unaff_BP -0xa);
}


pub unsafe fn pass1_1000_3168() -> u16 {
    let mut var1: *mut u8;
    let mut var2: u8;
    let mut var3: *mut c_char;
    let mut var4: u8;
    let mut var5: u16;
    let mut unaff_BP: i16;

    var2 = (unaff_BP -0x4);
    if (var2 == '*') {
        var5 = pass1_1000_34cf();
        if (var5 < 0x0) {
            var5 = -var5;
            var1 = (unaff_BP -0x6);
            *var1 = *var1 | 0x4;
        }
    } else {
        var5 = (unaff_BP -0xc) * 0xa + (var2 - 0x30);
    }
    (unaff_BP -0xc) = var5;
    var3 = *(unaff_BP + 0xa);
    var2 = *var3;
    (unaff_BP + 0xa) = var3 + 1;
    (unaff_BP -0x4) = var2;
    if ((var2 != '\0') && (-0x1 < (unaff_BP -0xa))) {
        if ((var2 - 0x20) < 0x59) {
            var4 = *((var2 - 0x20) + 0x5ffe) & 0xf;
        } else {
            var4 = 0;
        }
        var4 = *((var4 * '\x0b' + (unaff_BP -0x7)) + 0x5ffe) >> 0x4;
        *(unaff_BP -0x7) = var4;
        // WARNING: Could not recover jumptable at 0x1000310e. Too many branches
        // WARNING: Treating indirect jump as call
        var5 = (var4 * 0x2 + 0x30a4)();
        return var5;
    }
    return (unaff_BP -0xa);
}



pub unsafe fn pass1_1000_3194() -> u16 {
    let mut cVar1: u8;
    let mut pcVar2: *mut c_char;
    let mut bVar3: u8;
    let mut uVar4: u16;
    let mut unaff_BP: i16;

    (unaff_BP -0xe) = 0;
    pcVar2 = *(unaff_BP + 0xa);
    cVar1 = *pcVar2;
    (unaff_BP + 0xa) = pcVar2 + 1;
    (unaff_BP -0x4) = cVar1;
    if ((cVar1 != '\0') && (-0x1 < (unaff_BP -0xa))) {
        if ((cVar1 - 0x20) < 0x59) {
            bVar3 = *((cVar1 - 0x20) + 0x5ffe) & 0xf;
        } else {
            bVar3 = 0;
        }
        bVar3 = *((bVar3 * '\x0b' + (unaff_BP -0x7)) + 0x5ffe) >> 0x4;
        *(unaff_BP -0x7) = bVar3;
        // WARNING: Could not recover jumptable at 0x1000310e. Too many branches
        // WARNING: Treating indirect jump as call
        uVar4 = (bVar3 * 0x2 + 0x30a4)();
        return uVar4;
    }
    return (unaff_BP -0xa);
}


pub unsafe fn pass1_1000_319c() -> u16 {
    let mut cVar1: u8;
    let mut pcVar2: *mut c_char;
    let mut bVar3: u8;
    let mut uVar4: u16;
    let mut unaff_BP: i16;

    cVar1 = (unaff_BP -0x4);
    if (cVar1 == '*') {
        uVar4 = pass1_1000_34cf();
        if (uVar4 < 0x0) {
            uVar4 = 0xffff;
        }
    } else {
        uVar4 = (unaff_BP -0xe) * 0xa + (cVar1 - 0x30);
    }
    (unaff_BP -0xe) = uVar4;
    pcVar2 = *(unaff_BP + 0xa);
    cVar1 = *pcVar2;
    (unaff_BP + 0xa) = pcVar2 + 1;
    (unaff_BP -0x4) = cVar1;
    if ((cVar1 != '\0') && (-0x1 < (unaff_BP -0xa))) {
        if ((cVar1 - 0x20) < 0x59) {
            bVar3 = *((cVar1 - 0x20) + 0x5ffe) & 0xf;
        } else {
            bVar3 = 0;
        }
        bVar3 = *((bVar3 * '\x0b' + (unaff_BP -0x7)) + 0x5ffe) >> 0x4;
        *(unaff_BP -0x7) = bVar3;
        // WARNING: Could not recover jumptable at 0x1000310e. Too many branches
        // WARNING: Treating indirect jump as call
        uVar4 = (bVar3 * 0x2 + 0x30a4)();
        return uVar4;
    }
    return (unaff_BP -0xa);
}


pub unsafe fn pass1_1000_31c5() -> u16 {
    let mut pbVar1: *mut u8;
    let mut cVar2: u8;
    let mut pcVar3: *mut c_char;
    let mut bVar4: u8;
    let mut uVar5: u16;
    let mut unaff_BP: i16;

    cVar2 = (unaff_BP -0x4);
    if (cVar2 == 'l') {
        pbVar1 = (unaff_BP -0x6);
        *pbVar1 = *pbVar1 | 0x10;
    } else if (cVar2 == 'F') {
        pbVar1 = (unaff_BP -0x6);
        *pbVar1 = *pbVar1 | 0x20;
    } else if (cVar2 == 'N') {
        pbVar1 = (unaff_BP -0x5);
        *pbVar1 = *pbVar1 | 0x10;
    } else if (cVar2 == 'L') {
        pbVar1 = (unaff_BP -0x5);
        *pbVar1 = *pbVar1 | 0x4;
    } else {
        pbVar1 = (unaff_BP -0x5);
        *pbVar1 = *pbVar1 | 0x8;
    }
    pcVar3 = *(unaff_BP + 0xa);
    cVar2 = *pcVar3;
    (unaff_BP + 0xa) = pcVar3 + 1;
    (unaff_BP -0x4) = cVar2;
    if ((cVar2 != '\0') && (-0x1 < (unaff_BP -0xa))) {
        if ((cVar2 - 0x20) < 0x59) {
            bVar4 = *((cVar2 - 0x20) + 0x5ffe) & 0xf;
        } else {
            bVar4 = 0;
        }
        bVar4 = *((bVar4 * '\x0b' + (unaff_BP -0x7)) + 0x5ffe) >> 0x4;
        *(unaff_BP -0x7) = bVar4;
        // WARNING: Could not recover jumptable at 0x1000310e. Too many branches
        // WARNING: Treating indirect jump as call
        uVar5 = (bVar4 * 0x2 + 0x30a4)();
        return uVar5;
    }
    return (unaff_BP -0xa);
}


pub unsafe fn pass1_1000_31f7(mut param_1: u16) -> u16 {
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

    cVar4 = (unaff_BP -0x4);
    if ((cVar4 == 'd') || (cVar4 == 'i')) {
        pbVar2 = (unaff_BP -0x6);
        *pbVar2 = *pbVar2 | 0x40; //
                                  //        LAB_1000_3399:
        *(unaff_BP -0x8) = 0xa; //
                                  //        LAB_1000_33d4:
        if ((*(unaff_BP -0x6) & 0x10) == 0) {
            uVar7 = pass1_1000_34cf();
            if ((*(unaff_BP -0x6) & 0x40) == 0) {
                uVar13 = uVar7;
            } else {
                uVar13 = uVar7;
            }
        } else {
            uVar13 = pass1_1000_34d8();
        }
        if (((*(unaff_BP -0x6) & 0x40) != 0) && (uVar13 < 0x0)) {
            pbVar2 = (unaff_BP -0x5);
            *pbVar2 = *pbVar2 | 0x1;
            uVar13 = CONCAT22(-((uVar13 >> 0x10) + (uVar13 != 0)), -uVar13);
        }
        param_1 = (uVar13 >> 0x10);
        if ((unaff_BP -0xe) < 0x0) {
            (unaff_BP -0xe) = 0x1;
        } else {
            pbVar2 = (unaff_BP -0x6);
            *pbVar2 = *pbVar2 & 0xf7;
        }
        if (uVar13 == 0) {
            (unaff_BP -0x12) = 0;
        }
        unaff_DI = (unaff_BP -0x17);
        unaff_ES = &DAT_1050_1050;
        pcVar9 = *(unaff_BP -0x8);
        pass1_1000_356e(uVar13, pcVar9, param_1);
        if (((*(unaff_BP -0x5) & 0x2) != 0) && (pcVar9.is_null() || (unaff_DI != '0'))) {
            unaff_DI = (unaff_BP -0x18);
            unaff_DI = '0';
            pcVar9 = pcVar9 + 1;
        }
    } else {
        if (cVar4 == 'u') {
            // TODO: goto LAB_1000_3399;
        }
        if (cVar4 == 'X') {
            *(unaff_BP -0x3) = 0x7; //
                                      //            LAB_1000_33a9:
            if ((*(unaff_BP -0x6) & 0x80) != 0) {
                (unaff_BP -0x12) = 0x2;
                *(unaff_BP -0x10) = 0x30;
                (unaff_BP -0xf) = (unaff_BP -0x3) + 'Q';
            }
            *(unaff_BP -0x8) = 0x10;
            // TODO: goto LAB_1000_33d4;
        }
        if (cVar4 == 'x') {
            *(unaff_BP -0x3) = 0x27;
            // TODO: goto LAB_1000_33a9;
        }
        if (cVar4 == 'o') {
            if ((*(unaff_BP -0x6) & 0x80) != 0) {
                pbVar2 = (unaff_BP -0x5);
                *pbVar2 = *pbVar2 | 0x2;
            }
            *(unaff_BP -0x8) = 0x8;
            // TODO: goto LAB_1000_33d4;
        }
        if (cVar4 == 'c') {
            uVar7 = pass1_1000_34cf();
            unaff_ES = &DAT_1050_1050;
            *(unaff_BP -0x216) = uVar7;
            unaff_DI = (unaff_BP -0x216);
            pcVar9 = 0x1;
        } else if (cVar4 == 's') {
            uVar13 = pass1_1000_34e6(param_1);
            param_1 = (uVar13 >> 0x10);
            if ((unaff_DI.is_null()) && (unaff_ES == 0)) {
                unaff_ES = &DAT_1050_1050;
                unaff_DI = 0x6057;
                pcVar9 = DAT_1050_605d;
            } else {
                iVar8 = (unaff_BP -0xe);
                puVar11 = unaff_DI;
                if (iVar8 != 0) {
                    bVar12 = true;
                    loop {
                        if (iVar8 == 0) {
                            break;
                        }
                        iVar8 += -0x1;
                        puVar3 = puVar11;
                        puVar11 = (puVar11 + 1);
                        bVar12 = puVar3 == '\0';
                        if bVar12 == true {
                            break;
                        }
                    }
                    if (bVar12) {
                        puVar11 = (puVar11 -1);
                    }
                }
                pcVar9 = (puVar11 - unaff_DI);
            }
        } else {
            if (cVar4 == 'n') {
                pass1_1000_34e6(param_1);
                *unaff_DI = (unaff_BP -0xa);
                if ((*(unaff_BP -0x6) & 0x10) != 0) {
                    unaff_DI[0x1] = 0;
                }
                // TODO: goto LAB_1000_30cf;
            }
            if (cVar4 != 'p') {
                if ((cVar4 == 'E') || (cVar4 == 'G')) {
                    piVar1 = (unaff_BP -0x14);
                    *piVar1 = *piVar1 + 1;
                }
                pbVar2 = (unaff_BP -0x6);
                *pbVar2 = *pbVar2 | 0x40;
                bVar6 = *(unaff_BP -0x4) | 0x20;
                iVar8 = (unaff_BP -0xe);
                if (iVar8 < 1) {
                    if (iVar8 == 0) {
                        if (bVar6 == 0x67) {
                            (unaff_BP -0xe) = 0x1;
                        }
                    } else {
                        (unaff_BP -0xe) = 0x6;
                    }
                }
                unaff_DI = (unaff_BP -0x216);
                if ((*(unaff_BP -0x5) & 0x4) == 0) {
                    (PTR_s_3_wav_1050_25cc_1050_6068)();
                    piVar1 = (unaff_BP + 0xe);
                    *piVar1 = *piVar1 + 0x8;
                    param_1 = extraout_DX_00;
                } else {
                    (PTR_s_3_wav_1050_25cc_1050_607c)();
                    piVar1 = (unaff_BP + 0xe);
                    *piVar1 = *piVar1 + 0xa;
                    param_1 = extraout_DX;
                }
                if (((*(unaff_BP -0x6) & 0x80) != 0) && ((unaff_BP -0xe) == 0)) {
                    (PTR_s_3_wav_1050_25cc_1050_6074)();
                    param_1 = extraout_DX_01;
                }
                if ((bVar6 == 0x67) && (((unaff_BP -0x6) & 0x80) == 0)) {
                    (PTR_s_3_wav_1050_25cc_1050_6070)();
                    param_1 = extraout_DX_02;
                }
                unaff_ES = &DAT_1050_1050;
                if (unaff_DI == '-') {
                    unaff_DI = (unaff_BP -0x215);
                    pbVar2 = (unaff_BP -0x5);
                    *pbVar2 = *pbVar2 | 0x1;
                }
                iVar8 = -0x1;
                puVar11 = unaff_DI;
                loop {
                    if (iVar8 == 0) {
                        break;
                    }
                    iVar8 += -0x1;
                    puVar3 = puVar11;
                    puVar11 = (puVar11 + 1);
                    if puVar3 == '\0' {
                        break;
                    }
                }
                pcVar9 = (puVar11 + (-0x1 - unaff_DI));
                // TODO: goto LAB_1000_3444;
            }
            if ((*(unaff_BP -0x6) & 0x30) == 0) {
                uVar7 = pass1_1000_34cf();
                uVar13 = uVar7; //
                                //                LAB_1000_32d8:
                *(unaff_BP -0x3) = 0x7;
                param_1 = 0;
                pass1_1000_356e(uVar13, 0x10, 0x0);
                pcVar9 = 0x4;
            } else {
                uVar13 = pass1_1000_34d8();
                uVar10 = (uVar13 >> 0x10);
                if ((*(unaff_BP -0x5) & 0x18) != 0) {
                    // TODO: goto LAB_1000_32d8;
                }
                *(unaff_BP -0x3) = 0x7;
                pass1_1000_356e(uVar13, 0x10, 0x0);
                param_1 = 0;
                pass1_1000_356e(uVar10, 0x10, 0x0);
                *(unaff_BP -0x212) = 0x3a;
                pcVar9 = 0x9;
            }
            unaff_ES = &DAT_1050_1050;
            unaff_DI = (unaff_BP -0x216);
        }
    } //
      //    LAB_1000_3444:
    if ((*(unaff_BP -0x6) & 0x40) != 0) {
        if ((*(unaff_BP -0x5) & 1) == 0) {
            if ((*(unaff_BP -0x6) & 1) == 0) {
                if ((*(unaff_BP -0x6) & 0x2) != 0) {
                    *(unaff_BP -0x10) = 0x20;
                    (unaff_BP -0x12) = 0x1;
                }
            } else {
                *(unaff_BP -0x10) = 0x2b;
                (unaff_BP -0x12) = 0x1;
            }
        } else {
            *(unaff_BP -0x10) = 0x2d;
            (unaff_BP -0x12) = 0x1;
        }
    }
    if ((*(unaff_BP -0x6) & 0xc) == 0) {
        FUN_1000_3552(pcVar9, unaff_DI, unaff_ES);
        param_1 = extraout_DX_03;
    }
    pass1_1000_3534(in_AF, (unaff_BP -0x12), param_1);
    if (((*(unaff_BP -0x6) & 0x8) != 0) && ((*(unaff_BP -0x6) & 0x4) == 0)) {
        FUN_1000_3552(pcVar9, unaff_DI, unaff_ES);
        param_1 = extraout_DX_04;
    }
    pass1_1000_3534(in_AF, pcVar9, param_1);
    if ((*(unaff_BP -0x6) & 0x4) != 0) {
        FUN_1000_3552(0, 0, 0);
    } //
      //    LAB_1000_30cf:
    pcVar5 = *(unaff_BP + 0xa);
    cVar4 = *pcVar5;
    (unaff_BP + 0xa) = pcVar5 + 1;
    (unaff_BP -0x4) = cVar4;
    if ((cVar4 != '\0') && (-0x1 < (unaff_BP -0xa))) {
        if ((cVar4 - 0x20) < 0x59) {
            bVar6 = *((cVar4 - 0x20) + 0x5ffe) & 0xf;
        } else {
            bVar6 = 0;
        }
        bVar6 = *((bVar6 * '\x0b' + (unaff_BP -0x7)) + 0x5ffe) >> 0x4;
        *(unaff_BP -0x7) = bVar6;
        // WARNING: Could not recover jumptable at 0x1000310e. Too many branches
        // WARNING: Treating indirect jump as call
        uVar7 = (bVar6 * 0x2 + 0x30a4)();
        return uVar7;
    }
    return (unaff_BP -0xa);
}


pub unsafe fn pass1_1000_3bc0(mut param_1: i16, mut param_2: i16) {
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

    if ((*(param_2 + 0x2) & 1) != 0) {
        pass1_1000_3cb7(param_2);
        u_var5 = *unaff_si;
        if ((u_var5 & 1) != 0) {
            param_1 = (param_1 - u_var5) -0x1;
        }
        u_var5 = (param_2 + 0x4);
        if (u_var5 != 0) {
            u_var4 = param_1 + 0x2 + u_var5;
            if (!CARRY2(param_1 + 0x2, u_var5)) {
                uVar3 = pass1_1000_29dc(&DAT_1050_1050);
                u_var5 = &PTR_LOOP_1050_6066;
                if (u_var5 == 0x1000) {
                    // TODO: goto LAB_1000_3c12;
                }
                u_var2 = 0x8000;
                while (u_var5 <= u_var2) {
                    u_var2 >>= 0x1;
                    if (u_var2 == 0) {
                        // TODO: goto LAB_1000_3c2b;
                    }
                }
                if (u_var2 < 0x8) {
                    // TODO: goto LAB_1000_3c2b;
                }
                u_var5 = u_var2 << 0x1;
                // TODO: goto LAB_1000_3c12;
            }
            u_var2 = 0;
            u_var5 = 0xfff0;
            if (u_var4 == 0) {
                while (true) {
                    b_var8 = false;
                    u_var9 = mixed_mem_op_1000_3c51(u_var2, u_var4, param_2, 0x3c23, unaff_di);
                    if (!b_var8) {
                        break;
                    }
                    if (u_var5 == 0xfff0) {
                        return;
                    } //
                      //                    LAB_1000_3c2b:
                    u_var5 = 0x10; //
                                   //                    LAB_1000_3c12:
                    u_var5 -= 1;
                    u_var2 = u_var5 + u_var4;
                    if (CARRY2(u_var5, u_var4)) {
                        u_var2 = 0;
                    }
                    u_var5 = !u_var5;
                    u_var2 &= u_var5;
                }
                i_var6 = u_var9 - (param_2 + 0x4);
                (param_2 + 0x4) = u_var9;
                (param_2 + 0xa) = unaff_si;
                piVar1 = (param_2 + 0xc);
                *piVar1 = i_var6 -0x1;
                pu_var7 = (piVar1 + i_var6);
                *pu_var7 = 0xfffe;
                (param_2 + 0xc) = pu_var7;
            }
        }
    }
    return;
}

pub unsafe fn pass1_1000_3cd8(mut param_1: u16, mut param_2: u16) {
    free_mem_1000_407a(param_1, param_2);
    return;
}


pub unsafe fn pass1_1000_41e0(mut param_1: i16) -> u16 {
    let mut pi_stack6: *mut i16;

    pi_stack6 = CONCAT22(PTR_LOOP_1050_6192, PTR_LOOP_1050_6190);
    loop {
        if (PTR_LOOP_1050_6190 + (PTR_LOOP_1050_6194 & 0xfffc) <= pi_stack6) {
            return 0x0;
        }
        if (*pi_stack6 == param_1) {
            break;
        }
        pi_stack6 = (pi_stack6 & 0xffff0000 | ZEXT24(pi_stack6 + 0x4));
    }
    *pi_stack6 = 0;
    return (pi_stack6 + 2);
}
