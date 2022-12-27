use crate::winapp;

pub fn pass1_1008_80d2(param_1: u32) -> *mut u32 {
    *param_1 = 0;
    (param_1 + 0x4) = 0;
    return param_1;
}

pub fn pass1_1008_8168(param_1: *mut c_char) {
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    param_1 = 0x87c8;
    (param_1 + 0x2) = 0x1008;
    param_1 = 0x389a;
    (param_1 + 0x2) = 0x1008;
    return;
}


pub fn pass1_1008_87a2(mut param_1: u32, param_2: u8) -> u32 {
    pass1_1008_8168(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}



pub fn pass1_1008_87cc(
    param_1: *mut astruct_86,
    mut param_2: i16,
    mut param_3: i16,
    mut param_4: u16,
    param_5: *mut astruct_76,
    mut param_6: u32,
    mut param_7: u32,
) {
    let mut lVar1: i32;
    let mut uVar2: u16;
    let mut BVar3: bool;
    let mut piVar4: *mut i16;
    let mut uVar5: u16;
    let mut paVar6: *mut Struct57;
    let mut iVar5: *mut astruct_86;
    let mut iVar7: i16;
    let mut unaff_SI: u16;
    let mut uVar8: u16;
    let mut uVar9: u16;
    let mut uVar10: u16;
    let mut puVar11: *mut u16;
    let mut puVar12: *mut u16;
    let mut in_stack_0000fe70: u16;
    let mut in_stack_0000ff94: u16;
    let mut in_stack_0000ff9a: u16;
    let mut in_stack_0000ff9e: u16;
    let mut piStack48: *mut i16;
    let mut local_24: *mut Struct19;
    let mut uStack32: u16;
    let mut uStack30: u32;
    let mut pcStack26: *mut c_char;
    let mut uStack18: u32;
    let mut iStack14: i16;
    let mut iStack12: i16;
    let mut iStack10: i16;
    let mut iStack8: i16;
    let mut uStack6: u32;

    uVar10 = (param_7 >> 0x10);
    uVar8 = (param_1 >> 0x10);
    iVar5 = param_1;
    param_1.field0_0x0 = 0x389a;
    iVar5.field1_0x2 = 0x1008;
    iVar5.field2_0x4 = param_5;
    iVar5.field3_0x8 = 0;
    iVar5.field4_0xc = param_3;
    iVar5.field5_0xe = param_2;
    iVar5.field6_0x10 = 0;
    iVar5.field7_0x12 = 0;
    pass1_1008_3e38((param_1 & 0xffff0000 | ZEXT24(&iVar5.field12_0x1c)));
    pass1_1008_3e38((param_1 & 0xffff0000 | ZEXT24(&iVar5.field14_0x22)));
    puVar11 = pass1_1008_3e38((param_1 & 0xffff0000 | ZEXT24(&iVar5.field17_0x28)));
    paVar6 = CONCAT22(uVar10, (puVar11 >> 0x10));
    iVar5.field20_0x2e = param_4;
    iVar5.field21_0x30 = 0xffff;
    iVar5.field27_0x3a = 0;
    iVar5.field28_0x3e = 0x1;
    iVar5.field29_0x40 = 0x1;
    iVar5.field30_0x42 = param_6;
    param_1.field0_0x0 = 0x8e9a;
    iVar5.field1_0x2 = 0x1008;
    if (_PTR_LOOP_1050_0382.is_null()) {
        _PTR_LOOP_1050_0382 = mixed_1010_20ba(
            paVar6,
            _u16_1050_0ed0,
            CONCAT22(unaff_SI, 0x2e),
            in_stack_0000fe70,
            in_stack_0000ff94,
            in_stack_0000ff9a,
            in_stack_0000ff9e,
        );
        paVar6 = (paVar6 & 0xffff0000);
    }
    uVar10 = (paVar6 >> 0x10);
    uStack6 = pass1_1008_4772(iVar5.field2_0x4);
    iVar5.field7_0x12 = 0x2f - (uStack6 + 0x8);
    uVar9 = (_PTR_LOOP_1050_0382 >> 0x10);
    iVar7 = _PTR_LOOP_1050_0382;
    iStack8 = (iVar7 + 0xa);
    iStack10 = (iVar7 + 0xc);
    iStack12 = (iVar7 + 0xe);
    iStack14 = (iVar7 + 0x10);
    iVar7 = iVar5.field4_0xc;
    lVar1 = (iVar7 + iVar5.field5_0xe) * iStack14;
    paVar6 = CONCAT22(uVar10, (lVar1 >> 0x10));
    pass1_1008_3e76(
        (param_1 & 0xffff0000 | ZEXT24(&iVar5.field12_0x1c)),
        0x0,
        lVar1 + iVar5.field7_0x12 + iStack10,
        (iVar7 - iVar5.field5_0xe) * iStack12 + iVar5.field6_0x10 + iStack8,
    );
    iVar5.field8_0x14 = &iVar5.field12_0x1c + 0x20;
    iVar5.field9_0x16 = (uStack6 + 0x8) + (&iVar5.field12_0x1c + 0x2) -0x25;
    iVar5.field10_0x18 = iVar5.field8_0x14 + 0x32;
    uVar2 = iVar5.field9_0x16 + 0x19;
    iVar5.field11_0x1a = uVar2;
    mem_op_1000_179c(0x6, paVar6);
    uVar5 = paVar6;
    pcStack26 = CONCAT22(uVar5, uVar2);
    uStack18 = uVar5 | uVar2;
    if (uStack18 == 0) {
        iVar5.field3_0x8 = 0;
    } else {
        puVar12 = pass1_1008_ada2(CONCAT22(uVar5, uVar2), iVar5.field20_0x2e);
        uStack18 = (puVar12 >> 0x10);
        iVar5.field3_0x8 = puVar12;
        (iVar5.field3_0x8 + 0x2) = uStack18;
    }
    BVar3 = pass1_1008_aed8(iVar5.field3_0x8);
    if (BVar3 == 0) {
        pcStack26 = iVar5.field3_0x8;
        uStack18 = pcStack26;
        fn_ptr_1000_17ce(pcStack26);
        iVar5.field3_0x8 = 0;
    } else {
        piVar4 = iVar5.field3_0x8;
        pass1_1018_20ee(_PTR_LOOP_1050_0382, piVar4);
        uStack18 = SUB42(piVar4, 0x0);
        pass1_1008_add2(iVar5.field3_0x8);
        uStack30 = pass1_1008_4772(CONCAT22(uStack18, uStack18));
        pass1_1018_214e(
            _PTR_LOOP_1050_0382,
            (_PTR_LOOP_1050_0382 >> 0x10),
            (param_1 & 0xffff0000 | ZEXT24(&iVar5.field17_0x28)),
            iVar5.field20_0x2e,
        );
        local_24 = iVar5.field12_0x1c;
        uStack32 = iVar5.field13_0x20;
        pass1_1008_3f32(
            CONCAT22(0x1050, &local_24),
            (param_1 & 0xffff0000 | ZEXT24(&iVar5.field17_0x28)),
        );
        piStack48 = (param_1 & 0xffff0000 | ZEXT24(&iVar5.field_0x32));
        pass1_1008_3e94(
            CONCAT22(0x1050, &local_24),
            (param_1 & 0xffff0000 | ZEXT24(&iVar5.field24_0x34)),
            (param_1 & 0xffff0000 | ZEXT24(&iVar5.field_0x32)),
        );
        uVar10 = (uStack30 >> 0x10);
        iVar5.field25_0x36 = (uStack30 + 0x4) + *piStack48;
        uVar2 = (uStack30 + 0x8) + iVar5.field24_0x34;
        iVar5.field26_0x38 = uVar2;
        pass1_1008_612e(uVar2, 0x2, 0x5);
        iVar5.field28_0x3e = uVar2;
    }
    return;
}
pub fn pass1_1008_8aa2(param_1: *mut astruct_462) {
    let mut puVar1: *mut u32;
    let mut uVar2: u16;
    let mut uVar3: u16;
    let mut ppcVar4: *mut *mut code;
    let mut uVar5: u32;
    let mut iVar6: *mut astruct_462;
    let mut uVar6: u16;
    let mut pcStack16: *mut c_char;

    uVar6 = (param_1 >> 0x10);
    iVar6 = param_1;
    param_1 = 0x8e9a;
    iVar6.field2_0x2 = 0x1008;
    uVar5 = &iVar6.field3_0x4;
    if ((uVar5 + 0x1c) != 0) {
        puVar1 = iVar6.field3_0x4;
        uVar2 = iVar6.field4_0x6;
        if ((uVar2 | puVar1) != 0) {
            ppcVar4 = *puVar1;
            (**ppcVar4)();
        }
    }
    uVar2 = iVar6.field55_0x3a;
    uVar3 = iVar6.field56_0x3c;
    pcStack16 = CONCAT22(uVar3, uVar2);
    if ((uVar3 | uVar2) != 0) {
        pass1_1008_5118(CONCAT22(uVar3, uVar2));
        fn_ptr_1000_17ce(pcStack16);
    }
    param_1 = 0x389a;
    iVar6.field2_0x2 = 0x1008;
    return;
}


pub fn pass1_1008_8b20(mut param_1: u32) {
    let mut iVar1: i16;
    let mut piVar2: *mut i16;
    let mut in_EDX: u32;
    let mut uVar3: u32;
    let mut uVar4: u32;
    let mut iVar5: i16;
    let mut uVar6: u16;
    let mut local_a: [u8; 0x2] = [0; 0x2];
    let mut local_8: [u8; 0x2] = [0; 0x2];
    let mut paStack6: *mut astruct_76;

    uVar6 = (param_1 >> 0x10);
    iVar5 = param_1;
    if ((iVar5 + 0x8) != 0) {
        iVar1 = (iVar5 + 0x40);
        piVar2 = (iVar5 + 0x40);
        *piVar2 = *piVar2 + 1;
        uVar4 = iVar1 % (iVar5 + 0x3e) & 0xffff;
        uVar3 = in_EDX & 0xffff0000 | uVar4;
        if (uVar4 == 0) {
            (iVar5 + 0x40) = 0x1;
            piVar2 = (iVar5 + 0x8);
            pass1_1018_20ee(_PTR_LOOP_1050_0382, piVar2);
            paStack6 = (piVar2 & 0xffff | uVar3 << 0x10);
            uVar4 = uVar3 & 0xffff0000 | uVar6;
            pass1_1008_3e94(
                (param_1 & 0xffff0000 | (iVar5 + 0x28)),
                CONCAT22(0x1050, local_a),
                CONCAT22(0x1050, local_8),
            );
            pass1_1008_8d8a(
                (param_1 & 0xffff | uVar6 << 0x10),
                paStack6,
                (iVar5 + 0x4),
                uVar4,
            );
            pass1_1008_4480(
                (iVar5 + 0x4),
                (param_1 & 0xffff0000 | (iVar5 + 0x28)),
                paStack6,
            );
            return;
        }
    }
    return;
}


pub fn pass1_1008_8bc6(mut param_1: u16, mut param_2: u32) {
    let mut piVar1: *mut i16;
    let mut in_register_0000000a: u16;
    let mut uVar2: u32;
    let mut iVar3: i16;
    let mut uVar4: u16;
    let mut local_a: [u8; 0x2] = [0; 0x2];
    let mut local_8: [u8; 0x2] = [0; 0x2];
    let mut paStack6: *mut astruct_76;

    uVar4 = (param_2 >> 0x10);
    iVar3 = param_2;
    if ((iVar3 + 0x8) == 0) {
        return;
    }
    piVar1 = (iVar3 + 0x8);
    pass1_1018_20ee(_PTR_LOOP_1050_0382, piVar1);
    paStack6 = (piVar1 & 0xffff | param_1 << 0x10);
    uVar2 = CONCAT22(in_register_0000000a, uVar4);
    pass1_1008_3e94(
        (param_2 & 0xffff0000 | (iVar3 + 0x28)),
        CONCAT22(0x1050, local_a),
        CONCAT22(0x1050, local_8),
    );
    pass1_1008_8d8a(
        (param_2 & 0xffff | uVar4 << 0x10),
        paStack6,
        (iVar3 + 0x4),
        uVar2,
    );
    pass1_1008_4480(
        (iVar3 + 0x4),
        (param_2 & 0xffff0000 | (iVar3 + 0x28)),
        paStack6,
    );
    return;
}
pub fn pass1_1008_8c4e(mut param_1: u32, mut param_2: u32, mut param_3: u32) {
    let mut puVar1: *mut u16;
    let mut uVar2: u16;
    let mut uVar3: u16;
    let mut uVar5: u16;
    let mut iVar6: i16;
    let mut uVar7: u16;
    let mut uVar8: u32;
    let mut paStack14: *mut astruct_110;
    let mut paVar4: *mut Struct57;

    uVar5 = (param_3 >> 0x10);
    uVar7 = (param_1 >> 0x10);
    iVar6 = param_1;
    uVar8 = pass1_1008_4772((iVar6 + 0x4));
    uVar2 = (uVar8 >> 0x10);
    paVar4 = CONCAT22(uVar5, uVar2);
    uVar3 = 0;
    if (((iVar6 + 0xc) == 0) || ((iVar6 + 0xe) == 0)) {
        mem_op_1000_179c(0x14, paVar4);
        paStack14 = CONCAT22(paVar4, uVar3);
        uVar3 = paVar4 | uVar3;
        if (uVar3 == 0) {
            uVar2 = 0;
            uVar3 = 0;
        } else {
            puVar1 = (param_1 & 0xffff0000 | (iVar6 + 0x1c));
            pass1_1008_50c2(paStack14, (uVar8 + 0x8), (uVar8 + 0x4), puVar1, param_2);
            uVar2 = SUB42(puVar1, 0x0);
        }
        pass1_1008_5134(CONCAT22(uVar3, uVar2));
    }
    pass1_1008_4480(
        param_2,
        (param_1 & 0xffff0000 | (iVar6 + 0x1c)),
        (iVar6 + 0x4),
    );
    return;
}
pub fn pass1_1008_8ce4(mut param_1: u32, param_2: *mut u16, mut param_3: u32, mut param_4: u32) {
    let mut puVar1: *mut u8;
    let mut uVar2: u16;
    let mut paVar3: *mut Struct57;
    let mut iVar4: i16;
    let mut uVar5: u16;
    let mut uVar6: u16;
    let mut puVar7: *mut u16;
    let mut local_10: [u8; 0x6] = [0; 0x6];
    let mut uStack10: u32;
    let mut uStack6: u32;

    uVar6 = (param_4 >> 0x10);
    uVar5 = (param_1 >> 0x10);
    iVar4 = param_1;
    uStack6 = pass1_1008_4772((iVar4 + 0x4));
    uStack10 = 0;
    puVar7 = pass1_1008_3e54(
        CONCAT22(0x1050, local_10),
        0x0,
        (iVar4 + 0x12),
        (iVar4 + 0x10),
    );
    paVar3 = CONCAT22(uVar6, (puVar7 >> 0x10));
    puVar1 = local_10;
    pass1_1008_3f32(param_2, CONCAT22(0x1050, puVar1));
    mem_op_1000_179c(0x14, paVar3);
    uVar2 = paVar3 | puVar1;
    if (uVar2 == 0) {
        puVar1 = null_mut();
        uVar2 = 0;
    } else {
        uVar6 = (uStack6 >> 0x10);
        pass1_1008_50c2(
            CONCAT22(paVar3, puVar1),
            (uStack6 + 0x8),
            (uStack6 + 0x4),
            param_2,
            param_3,
        );
    }
    uStack10 = CONCAT22(uVar2, puVar1);
    pass1_1008_5134(CONCAT22(uVar2, puVar1));
    pass1_1008_4480(param_3, param_2, (iVar4 + 0x4));
    return;
}
pub fn pass1_1008_8d8a(
    param_1: *mut astruct_76,
    param_2: *mut astruct_76,
    mut param_3: u32,
    mut param_4: u32,
) {
    let mut uVar1: u16;
    let mut cVar2: u8;
    let mut puVar3: *mut u16;
    let mut uVar4: u16;
    let mut uVar5: u16;
    let mut uVar8: u16;
    let mut pstruct76_var7: *mut astruct_76;
    let mut uVar7: *mut astruct_76;
    let mut uVar9: u32;
    let mut paStack10: *mut astruct_110;
    let mut paVar6: *mut Struct57;

    uVar8 = (param_4 >> 0x10);
    uVar7 = (param_1 >> 0x10);
    pstruct76_var7 = param_1;
    uVar1 = pstruct76_var7[0x1].field3_0x6;
    if (uVar1 < 0x28) {
        if ((uVar1 < 0x25) && (uVar1 != 0x23)) {
            if (0x23 < uVar1) {
                return;
            }
            cVar2 = uVar1;
            // (cVar2 != '\v') &&
            if ((cVar2 != '\x0f') && (cVar2 != '!')) {
                return;
            }
        }
    } else if (uVar1 < 0x46) {
        if (uVar1 < 0x43) {
            if (uVar1 < 0x33) {
                return;
            }
            if ((uVar1 != 0x34 && 0x0 < (uVar1 - 0x33)) && (uVar1 != 0x37)) {
                return;
            }
        }
    } else if (uVar1 != 0x49) {
        if ((uVar1 - 0x49) < 0x2a) {
            return;
        }
        if (0x5 < (uVar1 - 0x73)) {
            return;
        }
    }
    if ((&pstruct76_var7[0x1].field8_0x10 + 0x2) == 0) {
        uVar9 = pass1_1008_4772(param_2);
        uVar4 = (uVar9 >> 0x10);
        paVar6 = CONCAT22(uVar8, uVar4);
        uVar1 = uVar9;
        uVar5 = uVar1;
        mem_op_1000_179c(0x14, paVar6);
        paStack10 = CONCAT22(paVar6, uVar5);
        uVar5 = paVar6 | uVar5;
        if (uVar5 == 0) {
            (&pstruct76_var7[0x1].field8_0x10 + 0x2) = 0;
        } else {
            puVar3 = (param_1 & 0xffff0000 | ZEXT24(pstruct76_var7 + 1));
            pass1_1008_50c2(paStack10, (uVar1 + 0x8), (uVar1 + 0x4), puVar3, param_3);
            (&pstruct76_var7[0x1].field8_0x10 + 0x2) = puVar3;
            pstruct76_var7[0x1].field9_0x14 = uVar5;
        }
        pass1_1008_5134((&pstruct76_var7[0x1].field8_0x10 + 0x2));
        return;
    }
    pass1_1008_5236((&pstruct76_var7[0x1].field8_0x10 + 0x2));
    return;
}

pub fn pass1_1008_8e74(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    pass1_1008_8aa2(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}
pub fn struct_op_1008_8e9e(param_1: *mut astruct_78, mut param_2: u32, mut param_3: u32) {
    let mut iVar1: *mut astruct_78;
    let mut uVar1: *mut astruct_78;

    uVar1 = (param_1 >> 0x10);
    iVar1 = param_1;
    param_1.field0_0x0 = 0x389a;
    iVar1.field1_0x2 = 0x1008;
    iVar1.field2_0x4 = 0;
    iVar1.field3_0x6 = null_mut();
    iVar1.field4_0xa = 0;
    iVar1.field5_0xe = param_3;
    iVar1.field6_0x12 = 0;
    iVar1.field7_0x16 = param_2;
    iVar1.field8_0x1a = 0x1;
    param_1.field0_0x0 = 0x9170;
    iVar1.field1_0x2 = 0x1008;
    if (iVar1.field5_0xe < 0x7) {
        iVar1.field5_0xe = 0x6;
    }
    pass1_1008_909c(param_1);
    iVar1.field3_0x6 = 0;
    return;
}
pub fn pass1_1008_8f24(param_1: *mut astruct_463) {
    let mut puVar1: *mut u32;
    let mut puVar2: *mut u32;
    let mut uVar3: u16;
    let mut ppcVar4: *mut *mut code;
    let mut uVar5: u32;
    let mut iVar6: *mut astruct_463;
    let mut iVar7: i16;
    let mut iVar8: i16;
    let mut uVar9: u16;
    let mut uVar10: u16;
    let mut uStack6: u32;

    uVar9 = (param_1 >> 0x10);
    iVar6 = param_1;
    param_1 = 0x9170;
    iVar6.field2_0x2 = 0x1008;
    if (iVar6.field19_0x1a != 0) {
        uStack6 = 0;
        loop {
            puVar1 = &iVar6.field6_0xa;
            if (*puVar1 < uStack6 || *puVar1 == uStack6) {
                break;
            }
            iVar8 = uStack6 * 0x4;
            uVar5 = iVar6.field5_0x6;
            uVar10 = (uVar5 >> 0x10);
            iVar7 = uVar5;
            puVar2 = (iVar7 + iVar8);
            uVar3 = (iVar7 + iVar8 + 2);
            if ((uVar3 | puVar2) != 0) {
                ppcVar4 = *puVar2;
                (**ppcVar4)();
            }
            uStack6 += 0x1;
        }
    }
    fn_ptr_1000_17ce(iVar6.field5_0x6);
    param_1 = 0x389a;
    iVar6.field2_0x2 = 0x1008;
    return;
}
pub fn pass1_1008_8faa(param_1: *mut astruct_78, mut param_2: u32) {
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    pass1_1008_9004(
        (param_1 & 0xffff | uVar1 << 0x10),
        param_2,
        (param_2 >> 0x10),
        (param_1 + 0xa),
    );
    return;
}
pub fn empty_1008_8fc4() {
    return;
}
