use std::os::raw::c_char;
use crate::app_context::AppContext;
use crate::unk::block_1000_2000::pass1_1000_29b5;
use crate::dos_interrupt::swi;
use crate::globals::PTR_LOOP_1050_5f48;
use crate::utils::CARRY2;
use crate::winapi16::FatalAppExit16;

pub fn pass1_1000_3024() {
    pass1_1000_3038(1);
    return;
}

pub fn pass1_1000_3038(mut param_1: i16) -> i16 {
    let mut uVar1: u16;
    let mut puVar2: *mut u8;
    let mut iVar3: i16;
    let mut iStack4: i16;

    iVar3 = 0;
    iStack4 = 0;
    // for (puVar2 =  &PTR_LOOP_1050_6210; puVar2 <= PTR_LOOP_1050_5ff0; puVar2 = puVar2 + 0xc) {
    for puVar2 in (PTR_LOOP_1050_6210..PTR_LOOP_1050_5ff0).step_by(0xc) {
        if ((param_1 == 1) && ((puVar2[0xa] & 0x83) != 0)) {
            uVar1 = pass1_1000_2f48(CONCAT22(0x1050, puVar2));
            if (uVar1 != 0xffff) {
                iVar3 += 0x1;
            }
        } else if ((param_1 == 0) && ((puVar2[0xa] & 0x2) != 0x0 && (
            uVar1 = pass1_1000_2f48(CONCAT22(0x1050, puVar2)),
            uVar1 == 0xffff,
        ))) {
            iStack4 = -0x1;
        }
    }
    if (param_1 == 1) {
        iStack4 = iVar3;
    }
    return iStack4;
}

pub fn FUN_1000_30b4() -> u16 {
    let mut bVar1: u8;
    let mut bVar2: u8;
    let mut unaff_BP: i16;
    let mut iVar3: i16;
    let mut unaff_SI: u16;
    let mut unaff_CS: u16;
    let mut in_stack_00000008: *mut u8;
    let mut uVar4: u16;

    iVar3 = unaff_BP + 1;
    uVar4 = SUB42(0x1050, 0x0);
    exit_1000_25f2(0x214, 0x30c5, unaff_CS, 0x1050);
    bVar1 = *in_stack_00000008;
    if (bVar1 == 0) {
        return 0x0;
    }
    if ((bVar1 - 0x20) < 0x59) {
        bVar2 = *((bVar1 - 0x20) + 0x5ffe) & 0xf;
    } else {
        bVar2 = 0;
    }
    // WARNING: Could not emulate address calculation at 0x10003101
    // WARNING: Treating indirect jump as call
    uVar4 = ((*((bVar2 * '\x0b') + 0x5ffe) >> 0x4) * 0x2 + 0x30a4)(
        unaff_SI & 0xff00 | bVar1,
        uVar4,
        iVar3,
    );
    return uVar4;
}

// WARNING (jumptable): Unable to track spacebase fully for stack

// WARNING (jumptable): Stack frame is not setup normally: Input value of stackpointer is not used


// WARNING (jumptable): Stack frame is not setup normally: Input value of stackpointer is not used


// WARNING (jumptable): Unable to track spacebase fully for stack

// WARNING (jumptable): Stack frame is not setup normally: Input value of stackpointer is not used


// WARNING (jumptable): Unable to track spacebase fully for stack

// WARNING (jumptable): Stack frame is not setup normally: Input value of stackpointer is not used

// WARNING (jumptable): Unable to track spacebase fully for stack

pub fn pass1_1000_34cf() -> u16 {
    let mut uVar1: u16;
    let mut puVar2: *mut u16;
    let mut unaff_BP: i16;

    puVar2 = (unaff_BP + 0xe);
    uVar1 = *puVar2;
    (unaff_BP + 0xe) = puVar2 + 2;
    return uVar1;
}

pub fn pass1_1000_34d8() -> u32 {
    let mut u_var1: u16;
    let mut u_var2: u16;
    let mut pu_var3: *mut u16;
    let mut unaff_bp: i16;

    pu_var3 = (unaff_bp + 0xe);
    u_var1 = *pu_var3;
    u_var2 = (pu_var3 + 2);
    (unaff_bp + 0xe) = pu_var3 + 0x4;
    return CONCAT22(u_var2, u_var1);
}

pub fn pass1_1000_34e6(mut param_1: u16) -> u32 {
    let mut u_var1: u16;
    let mut unaff_bp: i16;
    let mut u_var2: u32;

    if ((*(unaff_bp - 0x6) & 0x20) != 0) {
        u_var2 = pass1_1000_34d8();
        return u_var2;
    }
    u_var1 = pass1_1000_34cf();
    if (u_var1 == 0) {
        return param_1 << 0x10;
    }
    return CONCAT22(param_1, u_var1);
}

pub fn pass1_1000_3503(mut param_1: u16, mut param_2: u16) -> i16 {
    let mut piVar1: *mut i16;
    let mut pcVar2: *mut c_char;
    let mut ppcVar3: *mut *mut c_char;
    let mut uVar4: u16;
    let mut piVar5: *mut i16;
    let mut unaff_BP: i16;
    let mut uVar6: u16;

    ppcVar3 = (unaff_BP + 0x6);
    uVar6 = (ppcVar3 >> 0x10);
    piVar5 = ppcVar3;
    piVar1 = piVar5 + 2;
    *piVar1 = *piVar1 - 0x1;
    if (*piVar1 < 0x0) {
        uVar4 = mem_1000_2bb6(param_2, param_1, piVar5);
        if (uVar4 == 0xffff) {
            return -0x1;
        }
    } else {
        pcVar2 = *ppcVar3;
        *ppcVar3 = *ppcVar3 + 1;
        *pcVar2 = param_1;
    }
    return 0x0;
}

pub fn pass1_1000_3534(param_1: u8, mut param_2: i16, mut param_3: u16) {
    let mut piVar1: *mut i16;
    let mut pbVar2: *mut u8;
    let mut in_ax: u16;
    let mut unaff_bp: i16;
    let mut unaff_DI: *mut u8;
    let mut u_var3: u16;
    let mut unaff_es: u16;

    if (param_2 != 0) {
        piVar1 = (unaff_bp - 0xa);
        *piVar1 = *piVar1 + param_2;
        u_var3 = 0;
        loop {
            pbVar2 = unaff_DI;
            unaff_DI = unaff_DI + 1;
            in_ax = pass1_1000_3503(in_ax & 0xff00 | *pbVar2, param_3);
            u_var3 |= in_ax;
            param_2 += -0x1;
            if param_2 == 0 {
                break;
            }
        }
        if (u_var3 != 0) {
            (unaff_bp - 0xa) = 0xffff;
        }
    }
    return;
}

pub fn FUN_1000_3552(param_1: i16, param_2: u16, param_3: u16) {
    let mut pi_var1: *mut i16;
    // let mut param_3: u16 ;
    // let mut param_1: i16;
    // let mut param_2: u16 ;
    let mut unaff_bp: i16;
    let mut u_var2: u16;

    if (param_1 != 0) {
        pi_var1 = (unaff_bp - 0xa);
        *pi_var1 = *pi_var1 + param_1;
        u_var2 = 0;
        loop {
            param_3 = pass1_1000_3503(param_3 & 0xff00 | param_2 & 0xff, param_2);
            u_var2 |= param_3;
            param_1 += -0x1;
            if param_1 == 0 {
                break;
            }
        }
        if (u_var2 != 0) {
            (unaff_bp - 0xa) = 0xffff;
        }
    }
    return;
}

pub fn pass1_1000_356e(mut param_1: u16, mut param_2: u16, mut param_3: u16) {
    let mut pbVar1: *mut u8;
    let mut u_var2: u32;
    let mut bVar3: u8;
    let mut unaff_bp: i16;
    let mut unaff_si: i16;
    let mut unaff_DI: *mut u8;
    let mut unaff_es: u16;

    while ((0x0 < unaff_si || (param_1 != 0)) || (param_3 != 0)) {
        u_var2 = param_3;
        param_3 /= param_2;
        u_var2 = u_var2 % param_2 << 0x10 | param_1;
        param_1 = (u_var2 / param_2);
        bVar3 = (u_var2 % param_2) + 0x30;
        if (0x39 < bVar3) {
            bVar3 += (unaff_bp - 0x3);
        }
        pbVar1 = unaff_DI;
        unaff_DI = unaff_DI - 0x1;
        *pbVar1 = bVar3;
        unaff_si += -0x1;
    }
    return;
}

pub fn pass1_1000_35aa() -> *mut u16 {
    let mut puVar1: *mut u16;

    puVar1 = &PTR_LOOP_1050_6210;
    loop {
        if (PTR_LOOP_1050_5ff0 < puVar1) {
            return NULL;
        }
        if ((*(puVar1 + 0x5) & 0x83) == 0) {
            break;
        }
        puVar1 = puVar1 + 0x6;
    }
    *(puVar1 + 0x5) = 0;
    puVar1[0x2] = 0;
    puVar1[0x4] = 0;
    puVar1[0x3] = 0;
    puVar1[0x1] = 0;
    *puVar1 = 0;
    *(puVar1 + 0xb) = 0xff;
    return puVar1;
}



pub fn dos3_call_op_1000_35fe(ctx: &mut AppContext, mut param_1: u16, mut param_2: i16) -> u16 {
    let mut var1: u16;
    let mut var2: bool;

    if param_1 < u16_1050_5f8a {
        var2 = false;
        // TODO: Find AH register and configure
        swi(ctx, 0x21);
        var1 = fn_ptr_1(param_2 + 1);
        if !var2 {
            *(param_1 + 0x5f90) = 0;
        }
    } else {
        var1 = 0x900;
        var2 = true;
    }
    if var2 {
        pass1_1000_29b5(var1);
        return 0xffff;
    }
    return 0x0;
}

pub fn pass1_1000_39e1() {}

// WARNING: Unable to track spacebase fully for stack





pub fn mixed_dos3_call_1000_39f2(
    param_1: *mut u8,
    param_2: *mut c_char,
    param_3: *mut u8,
) -> *mut u8 {
    let mut pbVar2: *mut u8;
    let mut puVar3: *mut u8;
    let mut uVar4: u16;
    let mut pcVar5: *mut code;
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

    puVar6 = u16_1050_5f8a;
    if ((u16_1050_61ec != 0) && (
        puVar6 = PTR_s_ed_in_Op_Op_1050_0028_1050_5f8e,
        param_1 < (&u16_1050_0002 + 1),
    )) {
        param_1 = u16_1050_5f8a;
    }
    if (puVar6 <= param_1) {
        uVar15 = true;
        puVar6 = 0x900;
        // TODO: goto LAB_1000_299d;
    }
    puVar12 = param_1;
    puVar23 = u16_1050_5f8a;
    if ((param_1[0x5f90] & 0x20) != 0) {
        uVar15 = false;
        pcVar5 = swi(0x21);
        puVar6 = (*pcVar5)();
        unaff_CS = 0x1000;
        if (uVar15) {
            // TODO: goto LAB_1000_299d;
        }
    }
    pbVar12 = param_2;
    if ((puVar12[0x5f90] & 0x80) == 0) {
        //
        //        LAB_1000_3acf:
        uVar15 = false;
        puVar6 = param_3;
        if (param_3.is_null() == false) {
            uVar15 = puVar12 < puVar23;
            if (uVar15) {
                uVar15 = 0;
                pcVar5 = swi(0x21);
                uVar21 = (*pcVar5)();
            } else {
                piVar8 = pass1_1000_55b1(0x1050, in_stack_0000fff6);
                uVar21 = CONCAT22(pbVar12, piVar8);
            }
            puVar6 = uVar21;
            if (uVar15) {
                puVar6 = CONCAT11(0x9, uVar21);
            } else {
                uVar15 = false;
                if (puVar6.is_null()) {
                    if (((puVar12[0x5f90] & 0x40) == 0) || ((uVar21 >> 0x10) != '\x1a')) {
                        uVar15 = true;
                        puVar6 = 0x1c00;
                    } else {
                        uVar15 = false;
                    }
                }
            }
        }
    } else {
        in_stack_0000fff6 = 0x1050;
        bVar18 = true;
        piStack6 = null_mut();
        piStack8 = null_mut();
        piVar9 = param_3;
        pbVar13 = pbVar12;
        if (param_3.is_null() == false) {
            loop {
                if (piVar9.is_null()) {
                    break;
                }
                piVar9 = (piVar9 - 1);
                pbVar1 = pbVar13;
                pbVar13 = pbVar13 + 1;
                bVar18 = *pbVar1 == '\n';
                if bVar18 == true {
                    break;
                }
            }
            puVar23 = unaff_SI;
            if (!bVar18) {
                // TODO: goto LAB_1000_3acf;
            }
            pcVar22 = param_2;
            uVar6 = pass1_1000_3bac();
            pcVar22 = (pcVar22 >> 0x10);
            pbVar10 = pcVar22;
            if (uVar6 < 0xa9) {
                exit_1000_25f2(-0x4, 0x3ad9, unaff_CS, pcVar22);
                if (pbVar13 - pbVar10 == 0) {
                    return unaff_SI;
                }
                bVar16 = param_1 < unaff_SI;
                uVar4 = param_1 - unaff_SI;
                cVar20 = uVar4 < 0x0;
                cVar19 = uVar4 == 0;
                cVar17 = (POPCOUNT(uVar4 & 0xff) & 1) == 0;
                if (bVar16) {
                    bVar16 = 0;
                    cVar20 = '\0';
                    cVar19 = '\x01';
                    cVar17 = '\x01';
                    pcVar5 = swi(0x21);
                    piVar7 = (*pcVar5)(0x1050, piVar9, puVar12);
                } else {
                    piVar7 = pass1_1000_55b1(pbVar13 - pbVar10, 0x1050);
                }
                if (!bVar16) {
                    bVar16 = piVar9 < piVar7;
                    uVar4 = piVar9 - piVar7;
                    cVar20 = uVar4 < 0x0;
                    cVar19 = uVar4 == 0;
                    cVar17 = (POPCOUNT(uVar4 & 0xff) & 1) == 0;
                    piStack6 = piVar7;
                    if (bVar16 || cVar19) {
                        return unaff_SI;
                    }
                }
                uVar4 = (cVar20 << 0x7 | cVar19 << 0x6 | in_AF << 0x4 | cVar17 << 0x2 | 0x2 | bVar16) << 0x8;
                puVar6 = (piVar7 & 0xff | uVar4);
                if (piStack6.is_null()) {
                    uVar15 = (uVar4 & 0x100) != 0;
                    if (uVar15) {
                        puVar6 = CONCAT11(0x9, (piVar7 & 0xff));
                    } else if (((param_1[0x5f90] & 0x40) == 0) || (*param_2 != '\x1a')) {
                        uVar15 = true;
                        puVar6 = 0x1c00;
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
                (&stack0xffee + iVar10) = 0x1050;
                uVar14 = (&stack0xffee + iVar10);
                loop {
                    pbVar2 = pbVar12;
                    pbVar12 = pbVar12 + 1;
                    bVar16 = *pbVar2;
                    uVar5 = uVar6 & 0xff00;
                    uVar6 = uVar5 | bVar16;
                    if (bVar16 == 0xa) {
                        uVar7 = CONCAT11((uVar5 >> 0x8), 0xd);
                        if (puVar12 == puVar6) {
                            (&stack0xffee + iVar10) = 0x3abd;
                            uVar7 = mixed_dos3_call_1000_3ad9(
                                uVar7,
                                puVar11,
                                param_3,
                                (&stack0xfff0 + iVar10),
                            );
                        }
                        puVar3 = puVar12;
                        puVar12 = puVar12 + 1;
                        *puVar3 = uVar7;
                        uVar6 = CONCAT11((uVar7 >> 0x8), 0xa);
                        piStack8 = (piStack8 + 1);
                    }
                    if (puVar12 == puVar6) {
                        (&stack0xffee + iVar10) = 0x3ac8;
                        uVar6 = mixed_dos3_call_1000_3ad9(
                            uVar6,
                            puVar11,
                            param_3,
                            (&stack0xfff0 + iVar10),
                        );
                    }
                    puVar2 = puVar12;
                    puVar12 = puVar12 + 1;
                    *puVar2 = uVar6;
                    param_3 = param_3 - 0x1;
                    if param_3.is_null() {
                        break;
                    }
                }
                (&stack0xffee + iVar10) = 0x3ab1;
                mixed_dos3_call_1000_3ad9(uVar6, puVar11, 0x0, (&stack0xfff0 + iVar10));
            }
        }
        uVar15 = piStack6 < piStack8;
        puVar6 = (piStack6 - piStack8);
    } //
    //    LAB_1000_299d:
    if (uVar15) {
        pass1_1000_29b5(puVar6);
        puVar6 = 0xffff;
    }
    return puVar6;
}

// WARNING: Unable to track spacebase fully for stack


pub fn mixed_dos3_call_1000_3ad9(
    mut param_1: u16,
    mut param_2: i16,
    param_3: *mut i16,
    mut param_4: u16,
) -> u16 {
    let mut uVar2: u16;
    let mut pcVar3: *mut code;
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

    if (unaff_DI - param_2 == 0) {
        return param_4;
    }
    uVar5 = (unaff_BP + 0x6);
    puVar1 = (unaff_BP - 0xc);
    bVar5 = uVar5 < *puVar1;
    uVar1 = uVar5 - *puVar1;
    cVar9 = uVar1 < 0x0;
    cVar8 = uVar1 == 0;
    cVar7 = (POPCOUNT(uVar1 & 0xff) & 1) == 0;
    if (bVar5) {
        bVar5 = 0;
        cVar9 = '\0';
        cVar8 = '\x01';
        cVar7 = '\x01';
        pcVar3 = swi(0x21);
        piVar5 = (*pcVar3)();
    } else {
        piVar5 = pass1_1000_55b1(unaff_DI - param_2, 0x1050);
    }
    if (!bVar5) {
        piVar1 = (unaff_BP - 0x4);
        *piVar1 = *piVar1 + piVar5;
        bVar5 = param_3 < piVar5;
        uVar2 = param_3 - piVar5;
        cVar9 = uVar2 < 0x0;
        cVar8 = uVar2 == 0;
        cVar7 = (POPCOUNT(uVar2 & 0xff) & 1) == 0;
        if (bVar5 || cVar8) {
            return param_4;
        }
    }
    uVar2 = (cVar9 << 0x7 | cVar8 << 0x6 | in_AF << 0x4 | cVar7 << 0x2 | 0x2 | bVar5) << 0x8;
    uVar4 = piVar5 & 0xff | uVar2;
    if ((unaff_BP - 0x4) == 0) {
        bVar6 = (uVar2 & 0x100) != 0;
        if (bVar6) {
            uVar4 = CONCAT11(0x9, (piVar5 & 0xff));
        } else if (((*(uVar5 + 0x5f90) & 0x40) == 0) || (**(unaff_BP + 0x8) != '\x1a')) {
            bVar6 = true;
            uVar4 = 0x1c00;
        } else {
            bVar6 = false;
        }
    } else {
        uVar2 = (unaff_BP - 0x4);
        puVar2 = (unaff_BP - 0x6);
        bVar6 = uVar2 < *puVar2;
        uVar4 = uVar2 - *puVar2;
    }
    if (bVar6) {
        ((unaff_BP - 0xa) + 0x2) = 0x29a2;
        pass1_1000_29b5(uVar4);
        uVar4 = 0xffff;
    }
    return uVar4;
}

pub fn pass1_1000_3bac() -> i16 {
    let mut iVar1: i16;

    if (PTR_LOOP_1050_5f48 < &stack0x0004) {
        iVar1 = -(PTR_LOOP_1050_5f48 - &stack0x0004);
    } else {
        iVar1 = 0;
    }
    return iVar1;
}

/*
Unable to decompile 'mixed_mem_op_1000_3c51'
Cause:
Low-level Error: Overlapping input varnodes
*/
pub fn pass1_1000_3cb7(mut param_1: i16) {
    let mut u_var1: u16;
    let mut pu_var2: *mut u16;

    pu_var2 = (param_1 + 0xa);
    if (pu_var2 == (param_1 + 0xc)) {
        pu_var2 = (param_1 + 0x8);
    }
    while (true) {
        u_var1 = *pu_var2;
        if (u_var1 == 0xfffe) {
            break;
        }
        pu_var2 = (pu_var2 + (u_var1 & 0xfffe) + 2);
    }
    return;
}


pub fn pass1_1000_3cea(mut param_1: u32, param_2: *mut c_char) -> *mut u16 {
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

    uVar11 = (param_1 >> 0x10);
    bVar13 = true;
    iVar4 = -0x1;
    pUVar7 = param_1;
    loop {
        if (iVar4 == 0) {
            break;
        }
        iVar4 += -0x1;
        pUVar1 = pUVar7;
        pUVar7 = (pUVar7 + 1);
        bVar13 = pUVar1 == '\0';
        if bVar13 == true {
            break;
        }
    }
    pUVar10 = (pUVar7 - 1);
    uVar12 = (param_2 >> 0x10);
    pcVar8 = param_2;
    uVar5 = 0xffff;
    loop {
        if (uVar5 == 0) {
            break;
        }
        uVar5 -= 1;
        pcVar2 = pcVar8;
        pcVar8 = pcVar8 + 1;
        bVar13 = *pcVar2 == '\0';
        if bVar13 == true {
            break;
        }
    }
    uVar5 = !uVar5;
    if (!bVar13) {
        pcVar8 = pcVar8 - uVar5;
        uVar5 += 0x1;
    }
    pUVar9 = (pcVar8 - uVar5);
    if (uVar5 == 0) {
        pUVar1 = pUVar9;
        pUVar9 = pUVar9 + 1;
        *pUVar10 = *pUVar1;
        uVar5 = 0xfffe;
        pUVar10 = (pUVar7 + 1);
    } else if ((pUVar9 & 1) != 0) {
        pUVar1 = pUVar9;
        pUVar9 = (pUVar9 + 1);
        *pUVar10 = *pUVar1;
        uVar5 -= 1;
        pUVar10 = pUVar7;
    }
    // for (uVar6 = uVar5 >> 0x1; uVar6 != 0; uVar6 -= 1)
    for uVar6 in uVar5 >> 0x1..0 {
        pUVar3 = pUVar10;
        pUVar10 = pUVar10 + 1;
        pUVar1 = pUVar9;
        pUVar9 = pUVar9 + 1;
        *pUVar3 = *pUVar1;
    }
    // for (uVar5 =  ((uVar5 & 1) != 0); uVar5 != 0; uVar5 -= 1)
    uVar5 = uVar5 & 0x1 != 0;
    while uVar5 != 0 {
        pUVar3 = pUVar10;
        pUVar10 = (pUVar10 + 1);
        pUVar1 = pUVar9;
        pUVar9 = (pUVar9 + 1);
        *pUVar3 = *pUVar1;
        uVar5 -= 1;
    }
    return param_1;
}

pub fn unk_str_op_1000_3d3e(param_1: *mut c_char, in_string_2: *mut c_char) {
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

    l_string_1 = (in_string_2 >> 0x10);
    l_string_2 = in_string_2;
    l_b_var8 = true;
    u_var6 = 0xffff;
    puVar6 = l_string_2;
    loop {
        if (u_var6 == 0) {
            break;
        }
        u_var6 -= 1;
        puVar1 = puVar6;
        puVar6 = puVar6 + 1;
        l_b_var8 = *puVar1 == '\0';
        if l_b_var8 == true {
            break;
        }
    }
    u_var6 = !u_var6;
    u_var8 = (param_1 >> 0x10);
    puVar7 = param_1;
    if (l_b_var8) {
        if ((param_1 & 1) != 0) {
            puVar7 = puVar7 + 1;
            l_string_2 = l_string_2 + 1;
            *param_1 = *in_string_2;
            u_var6 -= 1;
        }
    } else {
        puVar7 = puVar7 + 2;
        l_string_2 = l_string_2 + 2;
        param_1 = in_string_2;
        u_var6 -= 1;
    }
    // for (uVar7 = uVar6 >> 0x1; uVar7 != 0; uVar7 -= 1)
    for uVar7 in uVar6 >> 0x1..0 {
        pu_var5 = puVar7;
        puVar7 = (puVar7 + 2);
        pu_var4 = l_string_2;
        l_string_2 = (l_string_2 + 2);
        *pu_var5 = *pu_var4;
    }
    // for (uVar6 =  ((uVar6 & 1) != 0); uVar6 != 0; uVar6 -= 1) {
    uVar6 = uvar6 & 1 != 0;
    while uVar6 != 0 {
        pu_var5 = puVar7;
        puVar7 = (puVar7 + 1);
        pu_var4 = l_string_2;
        l_string_2 = (l_string_2 + 1);
        *pu_var5 = *pu_var4;
        uVar6 -= 1;
    }
    return;
}

pub fn pass1_1000_3d7a(param_1: *mut c_char, param_2: *mut c_char) -> i16 {
    let mut pb_var2: *mut u8;
    let mut pb_var3: *mut u8;
    let mut i_var4: i16;
    let mut u_var5: u16;
    let mut string_4: *mut c_char;
    let mut string_1: *mut c_char;
    let mut string_2: *mut c_char;
    let mut string_6: *mut c_char;
    let mut u_var6: u16;
    let mut bool_1: bool;
    let mut bool_2: bool;
    let mut pb_var4: *mut c_char;
    let mut pb_var1: *mut c_char;
    let mut string_3: *mut c_char;

    string_1 = param_1;
    u_var6 = (param_2 >> 0x10);
    string_2 = param_2;
    i_var4 = 0;
    u_var5 = 0xffff;
    loop {
        if (u_var5 == 0) {
            break;
        }
        u_var5 -= 1;
        string_3 = string_2;
        string_2 = string_2 + 1;
        if *string_3 == 0 {
            break;
        }
    }
    string_4 = !u_var5;
    bool_1 = string_2 < string_4;
    string_6 = string_2 - string_4;
    bool_2 = string_6.is_null();
    loop {
        if (string_4.is_null()) {
            break;
        }
        string_4 = string_4 - 0x1;
        pb_var3 = string_6;
        string_6 = (string_6 + 1);
        pb_var2 = string_1;
        string_1 = (string_1 + 1);
        bool_1 = *pb_var2 < *pb_var3;
        bool_2 = *pb_var2 == *pb_var3;
        if bool_2 == false {
            break;
        }
    }
    if (!bool_2) {
        i_var4 = (0x1 - bool_1) - (bool_1 != 0);
    }
    return i_var4;
}

pub fn str_op_1000_3da4(param_1: *mut c_char) -> u16 {
    let mut pc_var1: *mut c_char;
    let mut u_var2: u16;
    let mut pc_var3: *mut c_char;
    let mut b_var4: bool;

    pc_var3 = param_1;
    b_var4 = true;
    u_var2 = 0xffff;
    loop {
        if (u_var2 == 0) {
            break;
        }
        u_var2 -= 1;
        pc_var1 = pc_var3;
        pc_var3 = pc_var3 + 1;
        b_var4 = *pc_var1 == '\0';
        if b_var4 == true {
            break;
        }
    }
    u_var2 = !u_var2;
    if (b_var4) {
        u_var2 -= 1;
    }
    return u_var2;
}

pub fn str_op_1000_3dbe(param_1: *mut c_char, param_2: *mut c_char, mut param_3: u16) -> u8 {
    let mut pcVar1: *mut c_char;
    let mut cVar2: u8;
    let mut pcVar3: *mut c_char;
    let mut pcVar4: *mut c_char;
    let mut uVar5: u16;

    uVar5 = (param_1 >> 0x10);
    pcVar4 = param_1;
    pcVar3 = param_2;
    if (param_3 != 0) {
        loop {
            pcVar1 = pcVar3;
            pcVar3 = pcVar3 + 1;
            cVar2 = *pcVar1;
            if (cVar2 == '\0') {
                break;
            }
            pcVar1 = pcVar4;
            pcVar4 = pcVar4 + 1;
            *pcVar1 = cVar2;
            param_3 -= 1;
            if param_3 == 0 {
                break;
            }
        }
        // for (; param_3 != 0; param_3 -= 1)
        while param_3 != 0 {
            pcVar1 = pcVar4;
            pcVar4 = pcVar4 + 1;
            *pcVar1 = '\0';
            param_3 -= 1;
        }
    }
    return param_1;
}

pub fn pass1_1000_3de8(
    param_1: *mut c_char,
    param_2: *mut c_char,
    mut param_3: u16,
    param_4: u16,
    param_5: u16,
) -> u16 {
    let mut pb_var1: *mut u8;
    let mut pc_var2: *mut c_char;
    let mut pc_var3: *mut c_char;
    let mut b_var4: u8;
    let mut u_var5: u16;
    let mut i_var6: i16;
    let mut pc_var7: *mut c_char;
    let mut pc_var8: *mut c_char;
    let mut u_var9: u16;
    let mut u_var10: u16;
    let mut b_var11: bool;

    if (param_3 != 0) {
        u_var9 = (param_1 >> 0x10);
        pc_var8 = param_1;
        u_var5 = param_3;
        pc_var7 = pc_var8;
        loop {
            if (u_var5 == 0) {
                break;
            }
            u_var5 -= 1;
            pc_var2 = pc_var7;
            pc_var7 = pc_var7 + 1;
            if *pc_var2 == '\0' {
                break;
            }
        }
        i_var6 = param_3 - u_var5;
        u_var10 = (param_2 >> 0x10);
        pc_var7 = param_2;
        loop {
            if (i_var6 == 0) {
                break;
            }
            i_var6 += -0x1;
            pc_var3 = pc_var8;
            pc_var8 = pc_var8 + 1;
            pc_var2 = pc_var7;
            pc_var7 = pc_var7 + 1;
            if *pc_var2 != *pc_var3 {
                break;
            }
        }
        b_var4 = pc_var7[-0x1];
        u_var5 = 0;
        pb_var1 = (pc_var8 - 1);
        b_var11 = b_var4 == *pb_var1;
        if (b_var4 < *pb_var1 || b_var11) {
            if (b_var11) {
                return 0x0;
            }
            u_var5 = 0xfffe;
        }
        param_3 = !u_var5;
    }
    return param_3;
}

pub fn pass1_1000_3e2c(mut param_1: u32) -> i16 {
    let mut pb_var1: *mut u8;
    let mut b_var2: u8;
    let mut b_var3: u8;
    let mut i_var4: i16;
    let mut pb_var5: *mut u8;
    let mut u_var6: u16;

    u_var6 = (param_1 >> 0x10);
    pb_var5 = param_1;
    i_var4 = 0;
    loop {
        loop {
            pb_var1 = pb_var5;
            pb_var5 = pb_var5 + 1;
            b_var2 = *pb_var1;
            if b_var2 != 0x20 {
                break;
            }
        }
        if b_var2 != 0x9 {
            break;
        }
    }
    if ((b_var2 != 0x2d) && (b_var3 = b_var2, b_var2 != 0x2b)) {
        // TODO: goto LAB_1000_3e4d;
    }
    loop {
        pb_var1 = pb_var5;
        pb_var5 = pb_var5 + 1;
        b_var3 = *pb_var1; //
        //        LAB_1000_3e4d:
        if ((0x39 < b_var3) || (b_var3 < 0x30)) {
            break;
        }
        i_var4 = i_var4 * 0xa + (b_var3 - 0x30);
    }
    if (b_var2 == 0x2d) {
        i_var4 = -i_var4;
    }
    return i_var4;
}

// pub fn pass1_1000_3e2c(mut param_1: u32) -> i16 {
//     let mut pbVar1: *mut u8;
//     let mut bVar2: u8;
//     let mut bVar3: u8;
//     let mut iVar4: i16;
//     let mut pbVar5: *mut u8;
//     let mut uVar6: u16;

//     uVar6 = (param_1 >> 0x10);
//     pbVar5 = param_1;
//     iVar4 = 0;
//     loop {
//         loop {
//             pbVar1 = pbVar5;
//             pbVar5 = pbVar5 + 1;
//             bVar2 = *pbVar1;
//             if bvar2 != 0x20 {
//                 break;
//             }
//         }
//         if bVar2 != 0x9 {
//             break;
//         }
//     }
//     if ((bVar2 != 0x2d) && (bVar3 = bVar2, bVar2 != 0x2b)) {
//         // TODO: goto LAB_1000_3e4d;
//     }
//     loop {
//         pbVar1 = pbVar5;
//         pbVar5 = pbVar5 + 1;
//         bVar3 = *pbVar1; //
//                          //        LAB_1000_3e4d:
//         if ((0x39 < bVar3) || (bVar3 < 0x30)) {
//             break;
//         }
//         iVar4 = iVar4 * 0xa + (bVar3 - 0x30);
//     }
//     if (bVar2 == 0x2d) {
//         iVar4 = -iVar4;
//     }
//     return iVar4;
// }

// pub fn pass1_1000_3e2c(mut param_1: u32) -> i16 {
//     let mut pbVar1: *mut u8;
//     let mut bVar2: u8;
//     let mut bVar3: u8;
//     let mut iVar4: i16;
//     let mut pbVar5: *mut u8;
//     let mut uVar6: u16;

//     uVar6 = (param_1 >> 0x10);
//     pbVar5 = param_1;
//     iVar4 = 0;
//     loop {
//         loop {
//             pbVar1 = pbVar5;
//             pbVar5 = pbVar5 + 1;
//             bVar2 = *pbVar1;
//             if bVar2 != 0x20 {
//                 break;
//             }
//         }
//         if bVar2 != 0x9 {
//             break;
//         }
//     }
//     if ((bVar2 != 0x2d) && (bVar3 = bVar2, bVar2 != 0x2b)) {
//         // TODO: goto LAB_1000_3e4d;
//     }
//     loop {
//         pbVar1 = pbVar5;
//         pbVar5 = pbVar5 + 1;
//         bVar3 = *pbVar1; //
//                          //        LAB_1000_3e4d:
//         if ((0x39 < bVar3) || (bVar3 < 0x30)) {
//             break;
//         }
//         iVar4 = iVar4 * 0xa + (bVar3 - 0x30);
//     }
//     if (bVar2 == 0x2d) {
//         iVar4 = -iVar4;
//     }
//     return iVar4;
// }

pub fn pass1_1000_3e82(mut param_1: u16, param_2: *mut u8, mut param_3: u16) -> *mut u8 {
    let mut pb_var1: *mut u8;
    let mut u_var2: u32;
    let mut b_var3: u8;
    let mut u_var5: u16;
    let mut u_var6: u16;
    let mut u_var7: u16;
    let mut pb_var8: *mut u8;
    let mut pb_var9: *mut u8;
    let mut pb_var10: *mut u8;
    let mut pb_var11: *mut u8;
    let mut u_var12: u16;
    let mut b_var13: bool;
    let mut c_var4: u8;

    u_var6 = 0;
    if (param_3 == 0xa) {
        u_var6 = param_1 >> 0xf;
    }
    u_var12 = (param_2 >> 0x10);
    pb_var9 = param_2;
    pb_var10 = pb_var9;
    pb_var8 = pb_var9;
    if ((param_3 == 0xa) && (u_var6 < 0x0)) {
        pb_var10 = pb_var9 + 1;
        *param_2 = '-';
        b_var13 = param_1 != 0;
        param_1 = -param_1;
        u_var6 = -(u_var6 + b_var13);
        pb_var8 = pb_var10;
    }
    loop {
        u_var7 = 0;
        u_var5 = u_var6;
        if (u_var6 != 0) {
            u_var5 = u_var6 / param_3;
            u_var7 = u_var6 % param_3;
        }
        u_var2 = CONCAT22(u_var7, param_1);
        param_1 = (u_var2 / param_3);
        c_var4 = (u_var2 % param_3);
        b_var3 = c_var4 + 0x30;
        if (0x39 < b_var3) {
            b_var3 = c_var4 + 0x57;
        }
        pb_var11 = pb_var10 + 1;
        *pb_var10 = b_var3;
        u_var6 = u_var5;
        pb_var10 = pb_var11;
        if u_var5 | param_1 == 0 {
            break;
        }
    }
    *pb_var11 = 0;
    loop {
        pb_var11 = pb_var11 - 0x1;
        pb_var1 = pb_var11;
        b_var3 = *pb_var1;
        *pb_var1 = *pb_var8;
        *pb_var8 = b_var3;
        pb_var10 = pb_var8 + 2;
        pb_var8 = pb_var8 + 1;
        if pb_var10 >= pb_var11 {
            break;
        }
    }
    return pb_var9;
}

pub fn pass1_1000_3ec0(mut param_1: u16, mut param_2: u16) -> i16 {
    let mut u_var1: u16;
    let mut u_var2: u16;
    let mut u_var3: u16;
    let mut unaff_si: u16;
    let mut u_var4: u16;
    let mut pu_var4: *mut u32;

    pu_var4 = CONCAT22(PTR_LOOP_1050_5fc0, PTR_LOOP_1050_5fbe);
    if (((PTR_LOOP_1050_5fc0 | PTR_LOOP_1050_5fbe) != 0) && ((param_2 | param_1) != 0)) {
        u_var1 = str_op_1000_3da4(CONCAT22(param_2, param_1));
        loop {
            u_var4 = (pu_var4 >> 0x10);
            u_var3 = pu_var4;
            if (((u_var3 + 0x2) | pu_var4) == 0) {
                break;
            }
            u_var2 = str_op_1000_3da4(CONCAT22((u_var3 + 0x2), pu_var4));
            if (((u_var1 < u_var2) && ((*pu_var4 + u_var1) == '=')) && (
                u_var2 = pass1_1000_3de8(
                    CONCAT22((u_var3 + 0x2), pu_var4),
                    CONCAT22(param_2, param_1),
                    u_var1,
                    unaff_si,
                    u_var3,
                ),
                u_var2 == 0,
            )) {
                return pu_var4 + u_var1 + 1;
            }
            pu_var4 = (pu_var4 & 0xffff0000 | (u_var3 + 0x4));
        }
    }
    return 0x0;
}

pub fn pass1_1000_3f5c() -> i16 {
    let mut u_var1: u16;
    let mut pu_var2: *mut u16;
    let mut i_var3: i16;

    i_var3 = 0;
    if (u16_1050_61ec == 0) {
        pu_var2 = &PTR_LOOP_1050_6210;
    } else {
        pu_var2 = 0x6234;
    }
    // for (; pu_var2 <= PTR_LOOP_1050_5ff0; pu_var2 = pu_var2 + 0x6) {
    while pu_var2 <= PTR_LOOP_1050_5ff0 {
        u_var1 = pass1_1000_2a00(pu_var2);
        if (u_var1 != 0xffff) {
            i_var3 += 0x1;
        }
        pu_var2 += 6;
    }
    return i_var3;
}



pub fn sys_1000_3f9c(param_1: *mut c_char, param_2: *mut c_char, mut param_3: u16) -> u16 {
    let mut pc_var1: *mut c_char;
    let mut u_var2: u16;
    let mut unaff_cs: u16;
    let mut local_4: u16;

    PTR_LOOP_1050_68b2._0_1_ = 0x42;
    _u16_1050_68a8 = param_1;
    PTR_LOOP_1050_68ac = 0x7fff;
    _PTR_LOOP_1050_68ae = param_1;
    u_var2 = FUN_1000_30b4();
    pc_var1 = _u16_1050_68a8;
    PTR_LOOP_1050_68ac = PTR_LOOP_1050_68ac - 0x1;
    if (PTR_LOOP_1050_68ac < 0x0) {
        mem_1000_2bb6(param_1, 0x0, &u16_1050_68a8);
    } else {
        _u16_1050_68a8 = (_u16_1050_68a8 & 0xffff0000 | (u16_1050_68a8 + 1));
        *pc_var1 = '\0';
    }
    PTR_LOOP_1050_68b0 = (_PTR_LOOP_1050_68ae >> 0x10);
    return u_var2;
}
