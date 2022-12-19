pub unsafe fn pass1_1010_9044(mut param_1: u32) {
    let mut ppcVar1: *mut *mut code;

    ppcVar1 = ((param_1 + 0x4) + 0x10);
    (**ppcVar1)();
    return;
}
pub unsafe fn fn_ptr_1010_905e(param_1: *mut astruct_169, mut param_2: u32) {
    let mut puVar1: *mut u32;
    let mut uVar2: u16;
    let mut ppcVar3: *mut *mut code;
    let mut iVar4: *mut astruct_169;
    let mut uVar4: u16;

    uVar4 = (param_1 >> 0x10);
    iVar4 = param_1;
    puVar1 = &iVar4.field4_0x4;
    uVar2 = (&iVar4.field4_0x4 + 2);
    if ((uVar2 | puVar1) != 0) {
        ppcVar3 = *puVar1;
        (**ppcVar3)();
    }
    iVar4.field4_0x4 = param_2;
    return;
}
pub unsafe fn pass1_1010_9092(mut param_1: u16, mut param_2: u32) {
    let mut ppcVar1: *mut *mut code;
    let mut uVar2: u32;
    let mut uVar3: u16;
    let mut uVar4: u16;
    let mut uVar5: u16;
    let mut in_EDX: *mut Struct57;
    let mut iVar7: i16;
    let mut uVar8: u16;
    let mut in_stack_0000fe7c: u16;
    let mut in_stack_0000ffa0: u16;
    let mut in_stack_0000ffa6: u16;
    let mut in_stack_0000ffaa: u16;
    let mut in_stack_0000ffd4: u16;
    let mut uVar9: u32;
    let mut uStack14: u32;
    let mut uStack6: u32;
    let mut paVar6: *mut Struct57;

    uVar8 = (param_2 >> 0x10);
    iVar7 = param_2;
    uVar9 = (iVar7 + 0x4);
    ppcVar1 = ((iVar7 + 0x4) + 0x10);
    (**ppcVar1)();
    uStack6 = CONCAT22(in_EDX, param_1);
    mem_op_1000_179c(0xc, in_EDX);
    uVar3 = in_EDX | param_1;
    paVar6 = (in_EDX & 0xffff0000 | uVar3);
    if (uVar3 == 0) {
        param_1 = 0;
        paVar6 = null_mut();
    } else {
        pass1_1010_8ef2(
            paVar6,
            CONCAT22(in_EDX, param_1),
            in_stack_0000ffd4,
            in_stack_0000fe7c,
            in_stack_0000ffa0,
            in_stack_0000ffa6,
            in_stack_0000ffaa,
        );
    }
    uVar4 = SUB42(paVar6, 0x0);
    uStack14 = 0;
    loop {
        uVar3 = paVar6;
        if (uStack6 <= uStack14) {
            break;
        }
        ppcVar1 = ((iVar7 + 0x4) + 0x4);
        uVar2 = uStack6;
        (**ppcVar1)(0x1000, (iVar7 + 0x4), uStack14, uVar9);
        uVar5 = uVar3 | uVar2;
        paVar6 = uVar5;
        if (uVar5 != 0) {
            ppcVar1 = ((param_1 + 0x4) + 0xc);
            (**ppcVar1)(0x1000, (param_1 + 0x4), uVar2, uVar3);
        }
        uStack14 += 0x1;
    }
    return;
}
pub unsafe fn pass1_1010_9130(
    mut param_1: u16,
    mut param_2: u16,
    mut param_3: u32,
    param_4: *mut u8,
) {
    let mut uVar1: u32;
    let mut uVar2: u16;

    uVar2 = (param_3 >> 0x10);
    pass1_1030_1d58((param_3 + 0x4));
    if ((param_2 | param_1).is_null() == false) {
        uVar1 = (param_3 + 0x8);
        pass1_1010_c3c2(
            (param_2 | param_1),
            uVar1,
            (uVar1 >> 0x10),
            param_4,
            CONCAT22(param_2, param_1),
        );
        return;
    }
    *param_4 = '\0';
    return;
}
pub unsafe fn struct_1010_9172(param_1: *mut astruct_249, mut param_2: u32) {
    let mut ppcVar1: *mut *mut code;
    let mut puVar2: *mut u32;
    let mut uVar3: u16;
    let mut paVar4: *mut Struct57;
    let mut iVar4: *mut astruct_249;
    let mut uVar5: u16;

    uVar5 = (param_1 >> 0x10);
    iVar4 = param_1;
    puVar2 = iVar4.field4_0x4;
    uVar3 = iVar4.field5_0x6;
    paVar4 = (param_2 & 0xffff0000 | uVar3);
    if ((uVar3 | puVar2) != 0) {
        ppcVar1 = *puVar2;
        puVar2 = (**ppcVar1)();
    }
    mem_op_1000_179c(0x18, paVar4);
    uVar3 = paVar4 | puVar2;
    if (uVar3 == 0) {
        puVar2 = null_mut();
        uVar3 = 0;
    } else {
        puVar2 = struct_op_1030_1cd8(CONCAT22(paVar4, puVar2), 0x5, 0x5);
    }
    iVar4.field4_0x4 = puVar2;
    iVar4.field5_0x6 = uVar3;
    return;
}
pub unsafe fn pass1_1010_91cc(mut param_1: u32) {
    let mut ppcVar1: *mut *mut code;
    let mut uVar2: u16;
    let mut lVar3: i32;

    uVar2 = (param_1 >> 0x10);
    ppcVar1 = ((param_1 + 0x4) + 0x10);
    lVar3 = (**ppcVar1)();
    if (lVar3 != 0) {
        ppcVar1 = ((param_1 + 0x4) + 0x8);
        (**ppcVar1)();
    }
    return;
}
pub unsafe fn pass1_1010_9210(mut param_1: u32) {
    let mut ppcVar1: *mut *mut code;

    ppcVar1 = ((param_1 + 0x4) + 0xc);
    (**ppcVar1)();
    return;
}



pub unsafe fn pass1_1010_9258(param_1: *mut astruct_223) -> *mut astruct_223 {
    struct_1010_383a(param_1);
    param_1.field0_0x0 = 0x958e;
    (param_1 + 0x2) = 0x1010;
    return param_1;
}
pub unsafe fn pass1_1010_927a(param_1: *mut u16) {
    *param_1 = 0x958e;
    (param_1 + 0x2) = 0x1010;
    pass1_1010_3880(param_1);
    return;
}

pub unsafe fn pass1_1010_9298(
    param_1: *mut StructD,
    param_2: *mut Struct19,
    mut param_3: u16,
) -> u32 {
    let mut uVar1: u16;
    let mut paVar2: *mut Struct57;

    uVar1 = (param_1 >> 0x10);
    paVar2 = (param_1 & 0xffff0000 | param_1 & 0xffff);
    struct_1010_2cd2(param_2, param_3);
    //        1010:9566  40  95  10  10      addr         pass1_1010_9540
    param_2.offset_0x0 = 0x9566;
    (param_2 + 0x2) = 0x1010;
    mem_op_1000_179c(0x20c, paVar2);
    (param_2 + 0x5c) = uVar1;
    (param_2 + 0x5e) = paVar2;
    pass1_1000_4906(CONCAT22(paVar2, (param_2 + 0x5c)), NULL, 0x20c);
    return param_2;
}
pub unsafe fn pass1_1010_92e6(param_1: *mut u16) {
    *param_1 = 0x9566;
    (param_1 + 0x2) = 0x1010;
    pass1_1010_2db2(param_1);
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

// WARNING: Globals starting with '_' overlap smaller symbols at the same address





// WARNING: Globals starting with '_' overlap smaller symbols at the same address







pub unsafe fn struct_1010_95aa(param_1: *mut Struct19, mut param_2: u16) {
    struct_op_1010_1d48(param_1, param_2);
    (param_1 + 0xa) = 0;
    (param_1 + 0xe) = 0;
    (param_1 + 0x12) = 0;
    (param_1 + 0x16) = 0;
    (param_1 + 0x18) = 0;
    (param_1 + 0x1a) = 0;
    (param_1 + 0x1c) = 0xa;
    (param_1 + 0x1e) = 0;
    param_1.offset_0x0 = 0xa1c8;
    (param_1 + 0x2) = 0x1010;
    return;
}
pub unsafe fn pass1_1010_95f8(param_1: *mut astruct_455) {
    let mut puVar1: *mut u32;
    let mut puVar2: *mut u32;
    let mut uVar3: u16;
    let mut ppcVar4: *mut *mut code;
    let mut iVar4: *mut astruct_455;
    let mut uVar5: u16;

    uVar5 = (param_1 >> 0x10);
    iVar4 = param_1;
    param_1.field0_0x0 = 0xa1c8;
    iVar4.field1_0x2 = 0x1010;
    puVar1 = iVar4[0x1].field1_0x2;
    puVar2 = iVar4[0x1].field2_0x4;
    if ((puVar2 | puVar1) != 0) {
        ppcVar4 = *puVar1;
        (**ppcVar4)();
    }
    puVar1 = iVar4[0x1].field3_0x6;
    uVar3 = (iVar4 + 0x2).field0_0x0;
    if ((uVar3 | puVar1) != 0) {
        ppcVar4 = *puVar1;
        (**ppcVar4)();
    }
    puVar1 = iVar4[0x2].field1_0x2;
    puVar2 = iVar4[0x2].field2_0x4;
    if ((puVar2 | puVar1) != 0) {
        ppcVar4 = *puVar1;
        (**ppcVar4)();
    }
    pass1_1010_1d80(param_1);
    return;
}
pub unsafe fn pass1_1010_9674(mut param_1: u32) {
    let mut puVar1: *mut u32;
    let mut uVar2: u16;
    let mut ppcVar3: *mut *mut code;
    let mut iVar4: i16;
    let mut uVar5: u16;

    uVar5 = (param_1 >> 0x10);
    iVar4 = param_1;
    puVar1 = (iVar4 + 0x12);
    uVar2 = (iVar4 + 0x14);
    if ((uVar2 | puVar1) != 0) {
        ppcVar3 = *puVar1;
        (**ppcVar3)();
    }
    (iVar4 + 0x12) = 0;
    return;
}
pub unsafe fn pass1_1010_96a8(mut param_1: u32, mut param_2: i16) {
    let mut piVar1: *mut i16;
    let mut uVar2: u16;

    uVar2 = (param_1 >> 0x10);
    piVar1 = (param_1 + 0x1e);
    *piVar1 = *piVar1 - param_2;
    if (*piVar1 < 0x0) {
        (param_1 + 0x1e) = 0;
    }
    return;
}

pub unsafe fn pass1_1010_96c2(mut param_1: u32) -> u16 {
    return (param_1 + 0x1e);
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn pass1_1010_96d0(param_1: *mut astruct_690) -> i16 {
    let mut piVar1: *mut i16;
    let mut iVar2: i16;
    let mut iVar3: *mut astruct_690;
    let mut uVar3: u16;
    let mut uVar4: u32;
    let mut iStack8: i16;

    uVar3 = (param_1 >> 0x10);
    iVar3 = param_1;
    if (iVar3.field26_0x1a != 0) {
        if (0x0 < iVar3.field27_0x1c) {
            piVar1 = &iVar3.field27_0x1c;
            *piVar1 = *piVar1 -0x1;
        }
        if ((iVar3.field27_0x1c == 0) && (iVar3.field28_0x1e != 0)) {
            iStack8 = 0x1;
            uVar4 = pass1_1030_8326();
            iVar2 = (uVar4 >> 0x10);
            if ((iVar2 != 0) || (0x32 < uVar4)) {
                iStack8 = 0x5;
            }
            if ((iVar2 != 0) || (0x3c < uVar4)) {
                iStack8 = 0xa;
            }
            if (iVar3.field28_0x1e < iStack8) {
                iStack8 = iVar3.field28_0x1e;
            }
            piVar1 = &iVar3.field28_0x1e;
            *piVar1 = *piVar1 - iStack8;
            if (*piVar1 < 0x0) {
                iVar3.field28_0x1e = 0;
            }
            if (0x0 < iVar3.field28_0x1e) {
                return iStack8;
            }
            return -0x1;
        }
    }
    return 0x0;
}
pub unsafe fn pass1_1010_9766(mut param_1: u16, mut param_2: u32) {
    let mut in_AX: i16;
    let mut uVar1: u16;

    uVar1 = (param_2 >> 0x10);
    (param_2 + 0x1a) = 0x1;
    pass1_1010_a0a0(param_1, param_2);
    pass1_1010_9f8c(param_2, 0x80);
    (param_2 + 0x1e) = in_AX * 0x32;
    return;
}

// WARNING: Could not reconcile some variable overlaps
pub unsafe fn pass1_1010_9794(param_1: *mut astruct_250) {
    let mut iVar1: i16;
    let mut ppcVar2: *mut *mut code;
    let mut pcVar3: *mut c_char;
    let mut uVar4: u16;
    let mut pchar_5: *mut c_char;
    let mut puVar5: *mut u32;
    let mut uVar6: u16;
    let mut in_EDX: u32;
    let mut uVar8: u32;
    let mut iVar9: *mut astruct_250;
    let mut uVar9: u16;
    let mut pcVar10: *mut c_char;
    let mut local_a: [u8; 0x8] = [0; 0x8];
    let mut paVar7: *mut Struct57;

    uVar9 = (param_1 >> 0x10);
    iVar9 = param_1;
    if (iVar9.field18_0x18 == 0) {
        iVar9.field18_0x18 = 0x1;
        puVar5 = iVar9.field11_0xe;
        uVar4 = (&iVar9.field11_0xe + 2);
        uVar6 = uVar4 | puVar5;
        paVar7 = (in_EDX & 0xffff0000 | uVar6);
        if (uVar6 != 0) {
            ppcVar2 = puVar5;
            (**ppcVar2)();
        }
        mem_op_1000_179c(0xc, paVar7);
        uVar4 = puVar5;
        uVar6 = paVar7 | uVar4;
        uVar8 = uVar6;
        if (uVar6 == 0) {
            uVar4 = 0;
            uVar8 = 0;
        } else {
            set_struct_1008_574a((puVar5 & 0xffff | paVar7 << 0x10));
        }
        iVar9.field11_0xe = uVar4;
        (&iVar9.field11_0xe + 0x2) = uVar8;
        pass1_1008_5784(CONCAT22(0x1050, local_a), iVar9.field10_0xa);
        loop {
            uVar4 = uVar8;
            pchar_5 = local_a;
            pass1_1008_5b12(CONCAT22(0x1050, pchar_5));
            uVar8 = (uVar4 | pchar_5);
            if ((uVar4 | pchar_5) == 0) {
                break;
            }
            iVar1 = (pchar_5 + 0x4);
            if ((iVar1 == 0x3e) || (iVar1 == 0x41)) {
                pcVar10 = iVar9.field10_0xa;
                (pcVar10 + 0xa) = 0;
                pcVar10 = iVar9.field10_0xa;
                ppcVar2 = (iVar9.field10_0xa + 0xc);
                (**ppcVar2)();
                pcVar3 = iVar9.field10_0xa;
                (pcVar3 + 0xa) = 0x1;
                local_a._4_4_ = 0;
                ppcVar2 = (*iVar9.field11_0xe + 0x4);
                (**ppcVar2)(0x1008, iVar9.field11_0xe, CONCAT22(uVar4, pchar_5), pcVar10);
            }
        }
    }
    return;
}
pub unsafe fn pass1_1010_988c(mut param_1: u32, mut param_2: i16) {
    let mut ppcVar1: *mut *mut code;
    let mut uVar2: u32;
    let mut iVar3: i16;
    let mut iVar4: i16;
    let mut uVar5: u16;
    let mut iVar6: i16;
    let mut uVar7: u16;
    let mut lVar8: i32;
    let mut local_a: [u8; 0x8] = [0; 0x8];

    uVar7 = (param_1 >> 0x10);
    iVar6 = param_1;
    pass1_1008_5784(CONCAT22(0x1050, local_a), (iVar6 + 0xe));
    loop {
        lVar8 = pass1_1008_5b12(CONCAT22(0x1050, local_a));
        uVar5 = (lVar8 >> 0x10);
        iVar3 = lVar8;
        if (lVar8 == 0) {
            return;
        }
        if !((iVar3 + 0x4) != param_2) {
            break;
        }
    }
    iVar4 = (iVar3 + 0x6) -0x1;
    (iVar3 + 0x6) = iVar4;
    if ((iVar4 < 1)
        && (
            ppcVar1 = ((iVar6 + 0xe) + 0xc),
            (**ppcVar1)(0x1008, (iVar6 + 0xe), lVar8),
            uVar2 = (iVar6 + 0xe),
            (uVar2 + 0x8) == 0,
        ))
    {
        (iVar6 + 0x16) = 0x1;
    }
    return;
}

pub unsafe fn pass1_1010_9f72(mut param_1: u32, mut param_2: i16) {
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    pass1_1010_9fa6(param_1, uVar1, (param_1 + 0xe), param_2);
    return;
}
pub unsafe fn pass1_1010_9f8c(mut param_1: u32, mut param_2: i16) {
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    pass1_1010_9fa6(param_1, uVar1, (param_1 + 0xa), param_2);
    return;
}

pub unsafe fn pass1_1010_9fa6(
    mut param_1: u16,
    mut param_2: u16,
    mut param_3: u32,
    mut param_4: i16,
) -> u16 {
    let mut uVar1: u16;
    let mut lVar2: i32;
    let mut local_a: [u8; 0x8] = [0; 0x8];

    if (param_3 != 0) {
        pass1_1008_5784(CONCAT22(0x1050, local_a), param_3);
        loop {
            lVar2 = pass1_1008_5b12(CONCAT22(0x1050, local_a));
            uVar1 = (lVar2 >> 0x10);
            if (lVar2 == 0) {
                break;
            }
            if ((lVar2 + 0x4) == param_4) {
                return (lVar2 + 0x6);
            }
        }
    }
    return 0x0;
}
pub unsafe fn pass1_1010_9fee(
    param_1: *mut StructD,
    param_2: *mut astruct_252,
    mut param_3: u16,
    mut param_4: u16,
) {
    let mut iVar5: *mut StructD;
    let mut uVar1: u16;
    let mut uVar2: u16;
    let mut paVar3: *mut Struct57;
    let mut iVar3: *mut astruct_252;
    let mut iVar4: *mut astruct_253;
    let mut uVar4: u16;
    let mut uVar5: u16;
    let mut puStack10: *mut StructD;
    let mut pSStack6: *mut StructD;
    let mut fn_ptr_1: *mut *mut code;

    iVar5 = (param_1 >> 0x10);
    paVar3 = (param_1 & 0xffff0000 | param_1 & 0xffff);
    uVar4 = (param_2 >> 0x10);
    iVar3 = param_2;
    if (iVar3.field18_0x12.is_null()) {
        mem_op_1000_179c(0xc, paVar3);
        uVar1 = paVar3;
        uVar2 = uVar1 | iVar5;
        paVar3 = (paVar3 & 0xffff0000 | uVar2);
        if (uVar2 == 0) {
            iVar3.field18_0x12 = null_mut();
        } else {
            set_struct_1008_574a(CONCAT22(uVar1, iVar5));
            iVar3.field18_0x12 = iVar5;
            (&iVar3.field18_0x12 + 0x2) = paVar3;
        }
    }
    mem_op_1000_179c(0x8, paVar3);
    uVar1 = paVar3;
    puStack10 = CONCAT22(uVar1, iVar5);
    if ((uVar1 | iVar5) == 0) {
        pSStack6 = null_mut();
    } else {
        puStack10.address_offset_field_0x0 = 0x389a;
        iVar5.address_offset_field_0x2 = 0x1008;
        puStack10.address_offset_field_0x0 = 0xa1c4;
        iVar5.address_offset_field_0x2 = 0x1010;
        pSStack6 = puStack10;
    }
    uVar5 = (pSStack6 >> 0x10);
    iVar4 = pSStack6;
    iVar4.field4_0x4 = param_4;
    iVar4.field5_0x6 = param_3;
    fn_ptr_1 = (*iVar3.field18_0x12 + 0x4);
    (**fn_ptr_1)(0x1000, iVar3.field18_0x12, iVar4, uVar5);
    return;
}
