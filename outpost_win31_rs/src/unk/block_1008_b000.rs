pub unsafe fn pass1_1008_b05a(param_1: *mut StructD) -> *mut u16 {
    let mut iVar1: *mut StructD;
    let mut uVar1: *mut StructD;

    uVar1 = (param_1 >> 0x10);
    iVar1 = param_1;
    param_1.address_offset_field_0x0 = 0x389a;
    iVar1.address_offset_field_0x2 = 0x1008;
    iVar1.hfile_0x4 = 0;
    param_1.address_offset_field_0x0 = 0xbdc8;
    iVar1.address_offset_field_0x2 = 0x1008;
    return &param_1.address_offset_field_0x0;
}
pub unsafe fn pass1_1008_b08c(param_1: *mut u16) {
    let mut iVar1: i16;
    let mut uVar2: u16;

    uVar2 = (param_1 >> 0x10);
    iVar1 = param_1;
    *param_1 = 0xbdc8;
    (iVar1 + 0x2) = 0x1008;
    fn_ptr_1000_17ce(*(iVar1 + 0x4));
    *param_1 = 0x389a;
    (iVar1 + 0x2) = 0x1008;
    return;
}
pub unsafe fn set_stuct_1008_b0bc(param_1: *mut StructD) {
    let mut iVar1: *mut StructD;
    let mut uVar1: *mut StructD;

    pass1_1008_b05a(param_1);
    uVar1 = (param_1 >> 0x10);
    iVar1 = param_1;
    iVar1.field5_0x8 = 0;
    iVar1.field6_0xa = 0;
    iVar1.field8_0xe = 0;
    iVar1.field_0x10 = 0;
    param_1.address_offset_field_0x0 = 0xbdc4;
    iVar1.address_offset_field_0x2 = 0x1008;
    return;
}

pub unsafe fn pass1_1008_b0f2(param_1: *mut StructD) -> *mut u16 {
    let mut uVar1: *mut StructD;

    pass1_1008_b05a(param_1);
    uVar1 = (param_1 >> 0x10);
    (param_1 + 0x8) = 0;
    param_1.address_offset_field_0x0 = 0xbdc0;
    (param_1 + 0x2) = 0x1008;
    return &param_1.address_offset_field_0x0;
}

pub unsafe fn pass1_1008_b11e(param_1: *mut StructD) -> *mut u16 {
    let mut uVar1: *mut StructD;

    pass1_1008_b05a(param_1);
    uVar1 = (param_1 >> 0x10);
    (param_1 + 0x8) = 0;
    param_1.address_offset_field_0x0 = 0xbddc;
    (param_1 + 0x2) = 0x1008;
    return &param_1.address_offset_field_0x0;
}


pub unsafe fn pass1_1008_b146(mut param_1: u16, mut param_2: u16, mut param_3: u32) {
    let mut uVar1: u32;
    let mut iVar2: i16;
    let mut uVar3: u16;

    uVar3 = (param_3 >> 0x10);
    iVar2 = param_3;
    if ((iVar2 + 0x16) != 0) {
        uVar1 = (iVar2 + 0x16);
        pass1_1030_8344(_u16_1050_5748, (uVar1 + 0xa));
        pass1_1038_3608(CONCAT22(param_2, param_1));
        uVar1 = (iVar2 + 0x16);
        (uVar1 + 0x8) = 0;
        uVar1 = (iVar2 + 0x16);
        (uVar1 + 0xa) = 0;
        uVar1 = (iVar2 + 0x16);
        (uVar1 + 0xe) = 0;
        uVar1 = (iVar2 + 0x16);
        (uVar1 + 0x10) = 0;
    }
    return;
}
pub unsafe fn pass1_1008_b1a6(mut param_1: u32, param_2: *mut c_char) {
    let mut lVar1: i32;
    let mut uVar2: u16;
    let mut in_DX: u16;
    let mut iVar3: *mut astruct_467;
    let mut iVar4: *mut astruct_466;
    let mut uVar3: u16;
    let mut uVar4: u16;

    uVar3 = (param_1 >> 0x10);
    iVar3 = param_1;
    if (iVar3.field22_0x16 != 0) {
        lVar1 = iVar3.field22_0x16;
        fn_ptr_1000_17ce(*(lVar1 + 0x4));
        uVar2 = str_op_1008_60e8(in_DX, param_2);
        lVar1 = iVar3.field22_0x16;
        uVar4 = (lVar1 >> 0x10);
        iVar4 = lVar1;
        iVar4.field4_0x4 = uVar2;
        iVar4.field5_0x6 = in_DX;
        iVar3.field22_0x16 = 0;
    }
    return;
}



pub unsafe fn load_string_1008_b1f0() -> *mut c_char {
    let mut pcVar1: *mut c_char;

    pcVar1 = load_string_1010_847e(_u16_1050_14cc, 0x531);
    return pcVar1;
}
pub unsafe fn pass1_1008_b200(param_1: *mut astruct_194) {
    let mut uVar1: u32;
    let mut puVar2: *mut u32;
    let mut ppcVar3: *mut *mut code;
    let mut puVar4: *mut u32;
    let mut paVar5: *mut astruct_92;
    let mut uVar5: *mut astruct_195;
    let mut iVar6: i16;
    let mut uVar7: u16;
    let mut uVar8: u32;
    let mut uVar9: u16;
    let mut uVar10: u16;
    let mut in_EDX: u32;
    let mut paVar11: *mut Struct57;
    let mut paVar13: *mut Struct57;
    let mut uVar14: u32;
    let mut iVar12: *mut astruct_194;
    let mut uVar15: u16;
    let mut puVar16: *mut u16;
    let mut pcVar17: *mut c_char;
    let mut pcStack24: *mut c_char;
    let mut local_14: *mut astruct_92;
    let mut paVar12: *mut Struct57;

    uVar15 = (param_1 >> 0x10);
    iVar12 = param_1;
    if (iVar12.field14_0xe.is_null() == false) {
        return;
    }
    // WARNING: Load size is inaccurate
    puVar4 = iVar12.field14_0xe;
    uVar9 = (&iVar12.field14_0xe + 2);
    paVar11 = (in_EDX & 0xffff0000 | uVar9);
    if ((uVar9 | puVar4) != 0) {
        ppcVar3 = *puVar4;
        (**ppcVar3)();
    }
    mem_op_1000_179c(0xc, paVar11);
    uVar9 = paVar11 | puVar4;
    paVar13 = (paVar11 & 0xffff0000);
    paVar12 = (paVar13 | uVar9);
    if (uVar9 == 0) {
        puVar4 = null_mut();
    } else {
        set_struct_1008_574a(CONCAT22(paVar11, puVar4));
        paVar13 = paVar12;
    }
    iVar12.field14_0xe = puVar4;
    (&iVar12.field14_0xe + 0x2) = paVar13;
    pass1_1028_dc52(CONCAT22(0x1050, &local_14), 0x1, 0x0, 0x400);
    loop {
        paVar5 = &local_14;
        pass1_1028_e4ec(CONCAT22(0x1050, paVar5));
        uVar9 = paVar13;
        pcStack24 = CONCAT22(uVar9, paVar5);
        paVar13 = (paVar13 & 0xffff0000 | (uVar9 | paVar5));
        if ((uVar9 | paVar5) == 0) {
            break;
        }
        uVar1 = paVar5.field3_0x4;
        if (paVar5[0x1c].field4_0x8 == 0x8000001) {
            uVar8 = uVar1;
            mem_op_1000_179c(0xc, paVar13);
            uVar5 = uVar8;
            uVar14 = paVar13 & 0xffff0000;
            if ((paVar13 | uVar5) == 0) {
                iVar6 = 0;
            } else {
                puVar16 = pass1_1008_b0f2((uVar8 & 0xffff | paVar13 << 0x10));
                uVar14 = uVar14 & 0xffff0000 | puVar16 >> 0x10;
                iVar6 = puVar16;
            }
            uVar10 = uVar14;
            pcVar17 = pass1_1038_4d28(pcStack24);
            paVar13 = (uVar14 & 0xffff0000 | pcVar17 >> 0x10);
            uVar7 = (pcVar17 >> 0x10);
            uVar7 = str_op_1008_60e8(uVar7, (pcVar17 & 0xffff | uVar7 << 0x10));
            (iVar6 + 0x4) = uVar7;
            (iVar6 + 0x6) = paVar13;
            (iVar6 + 0x8) = uVar1;
            puVar2 = iVar12.field14_0xe;
            ppcVar3 = (*iVar12.field14_0xe + 0x8);
            (**ppcVar3)(0x38, puVar2, (puVar2 >> 0x10));
        }
    }
    return;
}

pub unsafe fn pass1_1008_b340(mut param_1: u32) -> u32 {
    let mut uVar1: u32;
    let mut iVar2: i16;
    let mut uVar3: u16;

    uVar3 = (param_1 >> 0x10);
    if ((param_1 + 0x16) != 0) {
        uVar1 = (param_1 + 0x16);
        uVar3 = (uVar1 >> 0x10);
        iVar2 = uVar1;
        return CONCAT22((iVar2 + 0x6), (iVar2 + 0x4));
    }
    return 0x0;
}

pub unsafe fn pass1_1008_b366(mut param_1: u32) -> u32 {
    let mut uVar1: u32;
    let mut iVar2: i16;
    let mut uVar3: u16;

    uVar3 = (param_1 >> 0x10);
    if ((param_1 + 0x1a) != 0) {
        uVar1 = (param_1 + 0x1a);
        uVar3 = (uVar1 >> 0x10);
        iVar2 = uVar1;
        return CONCAT22((iVar2 + 0x6), (iVar2 + 0x4));
    }
    return 0x0;
}



pub unsafe fn pass1_1008_b38c(param_1: *mut StructD, param_2: *mut astruct_196) -> u32 {
    let mut puVar1: *mut u32;
    let mut ppcVar2: *mut *mut code;
    let mut uVar3: u16;
    let mut iVar3: *mut astruct_197;
    let mut paVar4: *mut astruct_197;
    let mut uVar5: u16;
    let mut uVar6: u16;
    let mut paVar7: *mut Struct57;
    let mut uVar9: u32;
    let mut iVar4: *mut astruct_196;
    let mut uVar4: *mut astruct_196;
    let mut puVar10: *mut u16;
    let mut uStack4: u16;
    let mut paVar8: *mut Struct57;

    uVar3 = (param_1 >> 0x10);
    paVar7 = (param_1 & 0xffff0000 | param_1 & 0xffff);
    uVar4 = (param_2 >> 0x10);
    iVar4 = param_2;
    if (iVar4.field18_0x12.is_null()) {
        mem_op_1000_179c(0xc, paVar7);
        uVar5 = paVar7 | uVar3;
        paVar8 = (paVar7 & 0xffff0000 | uVar5);
        if (uVar5 == 0) {
            iVar4.field18_0x12 = null_mut();
        } else {
            uVar3 = set_struct_1008_574a(CONCAT22(paVar7, uVar3));
            iVar4.field18_0x12 = uVar3;
            (&iVar4.field18_0x12 + 0x2) = paVar8;
        }
        // for (uStack4 = 0x6d9; uStack4 < 0x6e7; uStack4 += 1)
        for uStack4 in 0x6d9..0x6e7 {
            if (uStack4 == 0x6e3) {
                pass1_1030_8344(_u16_1050_5748, 0x8000001);
                // if ((uVar3 + 0x136) != 0) goto LAB_1008_b44a;
            } else {
                //
                // LAB_1008_b44a:
                mem_op_1000_179c(0xa, paVar8);
                uVar9 = paVar8 & 0xffff0000;
                if ((paVar8 | uVar3) == 0) {
                    iVar3 = null_mut();
                    paVar8 = (paVar8 & 0xffff0000);
                } else {
                    puVar10 = pass1_1008_b11e(CONCAT22(paVar8, uVar3));
                    paVar8 = (uVar9 & 0xffff0000 | puVar10 >> 0x10);
                    iVar3 = puVar10;
                }
                uVar6 = SUB42(paVar8, 0x0);
                paVar4 = iVar3;
                load_string_1010_84ac(_u16_1050_14cc, (_u16_1050_14cc >> 0x10), uStack4);
                iVar3.field4_0x4 = paVar4;
                iVar3.field5_0x6 = paVar8;
                iVar3.field6_0x8 = uStack4 - 0x6d8;
                puVar1 = iVar4.field18_0x12;
                ppcVar2 = (*iVar4.field18_0x12 + 0x8);
                uVar3 = (**ppcVar2)(0x1010, puVar1, (puVar1 >> 0x10), iVar3, uVar6);
            }
        }
    }
    return CONCAT22((&iVar4.field18_0x12 + 0x2), &iVar4.field18_0x12);
}

pub unsafe fn pass1_1008_b47a(mut param_1: u32) -> u32 {
    let mut uVar1: u32;
    let mut iVar2: i16;
    let mut uVar3: u16;

    uVar3 = (param_1 >> 0x10);
    if ((param_1 + 0x1e) != 0) {
        uVar1 = (param_1 + 0x1e);
        uVar3 = (uVar1 >> 0x10);
        iVar2 = uVar1;
        return CONCAT22((iVar2 + 0x6), (iVar2 + 0x4));
    }
    return 0x0;
}
pub unsafe fn pass1_1008_b4a0(mut param_1: u16, param_2: *mut u8, mut param_3: u32, param_4: i32) {
    let mut uVar1: u32;
    let mut uVar2: u16;
    let mut uVar3: u16;
    let mut iVar4: i16;
    let mut uVar5: u16;
    let mut uVar6: u32;
    let mut lVar7: i32;

    uVar6 = param_1;
    iVar4 = param_3;
    uVar5 = (param_3 >> 0x10);
    if (param_4 == 0) {
        (iVar4 + 0x16) = 0;
    } else {
        pass1_1008_b9ce(param_3, param_4);
        (iVar4 + 0x16) = uVar6;
        (iVar4 + 0x18) = param_2;
    }
    uVar1 = (iVar4 + 0x16);
    if ((uVar1 + 0x8) != 0) {
        pass1_1008_b200(param_3);
        uVar6 = pass1_1008_b38c(CONCAT22(uVar6, param_2), param_3);
        uVar3 = (uVar6 >> 0x10);
        uVar2 = uVar6;
        uVar1 = (iVar4 + 0x16);
        pass1_1008_b85c(param_3, (uVar1 + 0xa));
        (iVar4 + 0x1a) = uVar2;
        (iVar4 + 0x1c) = uVar3;
        uVar1 = (iVar4 + 0x16);
        lVar7 = pass1_1008_b8ac(param_3, (uVar1 + 0xe));
        (iVar4 + 0x1e) = lVar7;
        (iVar4 + 0x20) = (lVar7 >> 0x10);
        return;
    }
    (iVar4 + 0x1a) = 0;
    (iVar4 + 0x1e) = 0;
    return;
}


pub unsafe fn pass1_1008_b544(mut param_1: u16, mut param_2: u32, mut param_3: i16) {
    let mut puVar1: *mut u32;
    let mut uVar2: u16;
    let mut ppcVar3: *mut *mut code;
    let mut uVar4: u32;
    let mut uVar5: u32;
    let mut uVar6: u16;
    let mut iVar7: i16;
    let mut uVar8: u16;
    let mut unaff_CS: u16;

    iVar7 = param_2;
    uVar8 = (param_2 >> 0x10);
    if (param_3 != 0) {
        if ((iVar7 + 0x1a) != 0) {
            uVar4 = (iVar7 + 0x16);
            (uVar4 + 0x8) = 0x1;
            uVar4 = (iVar7 + 0x1a);
            uVar5 = (iVar7 + 0x16);
            (uVar5 + 0xa) = (uVar4 + 0x8);
            uVar4 = (iVar7 + 0x1e);
            uVar6 = (uVar4 + 0x8);
            uVar4 = (iVar7 + 0x16);
            (uVar4 + 0xe) = uVar6;
            uVar4 = (iVar7 + 0x16);
            pass1_1030_8344(_u16_1050_5748, (uVar4 + 0xa));
            unaff_CS = SUB42(&u16_1050_1038, 0x0);
            pass1_1038_3608(CONCAT22(param_1, uVar6));
        }
    }
    (iVar7 + 0x1e) = 0;
    (iVar7 + 0x1a) = 0;
    (iVar7 + 0x16) = 0;
    puVar1 = (iVar7 + 0xe);
    uVar2 = (iVar7 + 0x10);
    if ((uVar2 | puVar1) != 0) {
        ppcVar3 = *puVar1;
        (**ppcVar3)(unaff_CS, puVar1, uVar2, 1);
    }
    (iVar7 + 0xe) = 0;
    puVar1 = (iVar7 + 0x12);
    uVar2 = (iVar7 + 0x14);
    if ((uVar2 | puVar1) != 0) {
        ppcVar3 = *puVar1;
        (**ppcVar3)(unaff_CS, puVar1, uVar2, 1);
    }
    (iVar7 + 0x12) = 0;
    return;
}
pub unsafe fn pass1_1008_b61a(mut param_1: u16, mut param_2: u16, mut param_3: u32, mut param_4: u32) {
    let mut uVar1: u16;

    pass1_1008_b8fa(param_2, param_3, param_4);
    uVar1 = (param_3 >> 0x10);
    (param_3 + 0x1a) = param_1;
    (param_3 + 0x1c) = param_2;
    return;
}
pub unsafe fn pass1_1008_b63a(mut param_1: u32, mut param_2: u32) {
    let mut in_AX: u16;
    let mut in_DX: u16;
    let mut uVar1: u16;

    pass1_1008_b964(param_1, param_2);
    uVar1 = (param_1 >> 0x10);
    (param_1 + 0x1e) = in_AX;
    (param_1 + 0x20) = in_DX;
    return;
}


pub unsafe fn load_string_1008_b65a(
    mut param_1: u32,
    in_string_2: LPSTR,
    mut param_3: u32,
    mut param_4: u16,
) {
    pass1_1008_b9ce(param_1, CONCAT22(param_4, param_3));
    load_string_1010_84e0(
        _u16_1050_14cc,
        (_u16_1050_14cc >> 0x10),
        0x3ff,
        in_string_2,
        param_3,
    );
    return;
}


pub unsafe fn load_str_and_sprintf_1008_b69c(mut param_1: u16, param_2: *mut astruct_25) {
    let mut ppcVar1: *mut *mut code;
    let mut in_buffer_4: *mut c_char;
    let mut uVar2: u16;
    let mut iVar3: i16;
    let mut uVar4: u16;
    let mut uVar6: u16;
    let mut uVar7: u16;
    let mut in_register_0000000a: u16;
    let mut paVar8: *mut Struct57;
    let mut paVar10: *mut Struct57;
    let mut iVar5: *mut astruct_25;
    let mut uVar5: *mut astruct_25;
    let mut iStack516: i16;
    let mut local_202: [u8; 0x100] = [0; 0x100];
    let mut local_102: [u16; 0x80] = [0; 0x80];
    let mut paVar9: *mut Struct57;

    paVar8 = CONCAT22(in_register_0000000a, param_1);
    in_buffer_4 = local_202;
    load_string_1010_84e0(
        _u16_1050_14cc,
        (_u16_1050_14cc >> 0x10),
        0x100,
        in_buffer_4,
        0x1050,
    );
    uVar5 = (param_2 >> 0x10);
    iVar5 = param_2;
    if (iVar5.field10_0xa.is_null()) {
        mem_op_1000_179c(0xc, paVar8);
        uVar4 = paVar8 | in_buffer_4;
        paVar10 = (paVar8 & 0xffff0000);
        paVar9 = (paVar10 | uVar4);
        if (uVar4 == 0) {
            uVar4 = 0;
        } else {
            uVar4 = set_struct_1008_574a(CONCAT22(paVar8, in_buffer_4));
            paVar10 = paVar9;
        }
        iVar5.field10_0xa = uVar4;
        (iVar5.field10_0xa + 0x2) = paVar10;
        // for (iStack516 = 0x1; iStack516 < 0x6; iStack516 += 1)
        for iStack516 in 0..6 {
            mem_op_1000_179c(0x12, paVar10);
            uVar7 = paVar10 | uVar4;
            paVar8 = (paVar10 & 0xffff0000 | uVar7);
            if (uVar7 == 0) {
                iVar3 = 0;
                paVar10 = (paVar10 & 0xffff0000);
            } else {
                iVar3 = set_stuct_1008_b0bc(CONCAT22(paVar10, uVar4));
                paVar10 = paVar8;
            }
            uVar6 = SUB42(paVar10, 0x0);
            wsprintf16(
                local_102,
                CONCAT22(local_202, 0x1050),
                CONCAT22(iStack516, 0x1050),
                iVar3,
                uVar6,
                uVar4,
            );
            uVar2 = str_op_1008_60e8(paVar10, CONCAT22(0x1050, local_102));
            (iVar3 + 0x4) = uVar2;
            (iVar3 + 0x6) = paVar10;
            ppcVar1 = (*iVar5.field10_0xa + 0x8);
            uVar4 = (**ppcVar1)();
        }
        iVar5.field31_0x22 = 0x5;
    }
    return;
}


pub unsafe fn load_str_and_sprintf_1008_b78a(param_1: *mut StructD, mut param_2: u32) {
    let mut piVar1: *mut i16;
    let mut ppcVar2: *mut *mut code;
    let mut uVar3: u16;
    let mut uVar4: u16;
    let mut uVar5: u16;
    let mut paVar6: *mut Struct57;
    let mut iVar7: i16;
    let mut uVar8: u16;
    let mut local_206: [u8; 0x100] = [0; 0x100];
    let mut local_106: [u16; 0x80] = [0; 0x80];
    let mut iStack6: i16;
    let mut uStack4: u16;

    uVar3 = (param_1 >> 0x10);
    paVar6 = (param_1 & 0xffff0000 | param_1 & 0xffff);
    mem_op_1000_179c(0x12, paVar6);
    uVar5 = paVar6 | uVar3;
    if (uVar5 == 0) {
        iStack6 = 0;
        uVar5 = 0;
    } else {
        iStack6 = set_stuct_1008_b0bc(CONCAT22(paVar6, uVar3));
    }
    uStack4 = uVar5;
    load_string_1010_84e0(
        _u16_1050_14cc,
        (_u16_1050_14cc >> 0x10),
        0x100,
        local_206,
        0x1050,
    );
    uVar8 = (param_2 >> 0x10);
    iVar7 = param_2;
    piVar1 = (iVar7 + 0x22);
    *piVar1 = *piVar1 + 1;
    wsprintf16(
        local_106,
        CONCAT22(local_206, 0x1050),
        CONCAT22((iVar7 + 0x22), 0x1050),
        uVar3,
    );
    uVar4 = str_op_1008_60e8(uVar5, CONCAT22(0x1050, local_106));
    (iStack6 + 0x4) = uVar4;
    (iStack6 + 0x6) = uVar5;
    ppcVar2 = ((iVar7 + 0xa) + 0x8);
    (**ppcVar2)();
    return;
}



pub unsafe fn pass1_1008_b820(mut param_1: i16, mut param_2: u16, mut param_3: u32) -> u32 {
    let mut uVar1: u16;

    pass1_1030_8344(_u16_1050_5748, 0x8000001);
    if ((param_1 + 0x152) == 0) {
        return 0x0;
    }
    uVar1 = (param_3 >> 0x10);
    return CONCAT22((param_3 + 0xc), (param_3 + 0xa));
}
pub unsafe fn pass1_1008_b85c(mut param_1: u32, param_2: i32) {
    let mut puVar1: *mut u8;
    let mut extraout_DX: u16;
    let mut local_a: [u8; 0x8] = [0; 0x8];

    pass1_1008_5784(CONCAT22(0x1050, local_a), (param_1 + 0xe));
    loop {
        puVar1 = local_a;
        pass1_1008_5b12(CONCAT22(0x1050, puVar1));
        if ((extraout_DX | puVar1) == 0) {
            return;
        }
        if (puVar1 + 8) == param_2 {
            break;
        }
    }
    return;
}

pub unsafe fn pass1_1008_b8ac(mut param_1: u32, mut param_2: i16) -> u32 {
    let mut lVar1: i32;
    let mut local_a: [u8; 0x8] = [0; 0x8];

    pass1_1008_5784(CONCAT22(0x1050, local_a), (param_1 + 0x12));
    loop {
        lVar1 = pass1_1008_5b12(CONCAT22(0x1050, local_a));
        if (lVar1 == 0) {
            return 0x0;
        }
        if (lVar1 + 8) == param_2 {
            break;
        }
    }
    return lVar1;
}
pub unsafe fn pass1_1008_b8fa(mut param_1: u16, mut param_2: u32, param_3: *mut c_char) {
    let mut puVar1: *mut u8;
    let mut iVar2: i16;
    let mut extraout_DX: u16;
    let mut local_a: [u8; 0x8] = [0; 0x8];

    if (param_3.is_null()) {
        return;
    }
    pass1_1008_5784(CONCAT22(0x1050, local_a), (param_2 + 0xe));
    loop {
        puVar1 = local_a;
        pass1_1008_5b12(CONCAT22(0x1050, puVar1));
        if ((extraout_DX | puVar1) == 0) {
            return;
        }
        iVar2 = pass1_1000_3d7a(*(puVar1 + 0x4), param_3);
        if iVar2 == 0 {
            break;
        }
    }
    return;
}
pub unsafe fn pass1_1008_b964(mut param_1: u32, param_2: *mut c_char) {
    let mut string_1: *mut c_char;
    let mut iVar1: i16;
    let mut extraout_DX: u16;
    let mut local_a: [u8; 0x8] = [0; 0x8];

    if (param_2.is_null()) {
        return;
    }
    pass1_1008_5784(CONCAT22(0x1050, local_a), (param_1 + 0x12));
    loop {
        string_1 = local_a;
        pass1_1008_5b12(CONCAT22(0x1050, string_1));
        if ((extraout_DX | string_1) == 0) {
            return;
        }
        iVar1 = pass1_1000_3d7a(*(string_1 + 0x4), param_2);
        if iVar1 == 0 {
            break;
        }
    }
    return;
}
pub unsafe fn pass1_1008_b9ce(mut param_1: u32, param_2: *mut c_char) {
    let mut puVar1: *mut u8;
    let mut iVar2: i16;
    let mut extraout_DX: u16;
    let mut local_a: [u8; 0x8] = [0; 0x8];

    if (param_2.is_null()) {
        return;
    }
    pass1_1008_5784(CONCAT22(0x1050, local_a), (param_1 + 0xa));
    loop {
        puVar1 = local_a;
        pass1_1008_5b12(CONCAT22(0x1050, puVar1));
        if ((extraout_DX | puVar1) == 0) {
            return;
        }
        iVar2 = pass1_1000_3d7a(*(puVar1 + 0x4), param_2);
        if iVar2 == 0 {
            break;
        }
    }
    return;
}


pub unsafe fn struct_1008_bde0(param_1: *mut StructD, param_2: *mut astruct_139) {
    let mut uVar1: u16;
    let mut in_register_0000000a: u16;
    let mut pSVar2: *mut StructD;
    let mut iVar2: *mut astruct_139;
    let mut iVar3: *mut astruct_139;
    let mut iVar4: *mut astruct_139;
    let mut iVar5: *mut astruct_139;
    let mut iVar6: *mut astruct_139;
    let mut iVar7: *mut astruct_139;
    let mut iVar8: *mut astruct_139;
    let mut iVar9: *mut astruct_139;
    let mut iVar10: *mut astruct_139;
    let mut iVar11: *mut astruct_139;
    let mut iVar12: *mut astruct_139;
    let mut iVar2_00: *mut astruct_139;
    let mut iVar2_01: *mut astruct_139;
    let mut iVar2_02: *mut astruct_139;
    let mut iVar2_03: *mut astruct_139;
    let mut iVar2_04: *mut astruct_139;
    let mut iVar2_05: *mut astruct_139;
    let mut iVar2_06: *mut astruct_139;
    let mut iVar2_07: *mut astruct_139;
    let mut iVar2_08: *mut astruct_139;
    let mut iVar2_09: *mut astruct_139;
    let mut iVar13: i16;
    let mut iVar2_10: *mut astruct_139;
    let mut iVar2_11: *mut astruct_139;
    let mut iVar2_12: *mut astruct_139;
    let mut iVar2_13: *mut astruct_139;
    let mut iVar2_14: *mut astruct_139;
    let mut iVar2_15: *mut astruct_139;
    let mut iVar2_16: *mut astruct_139;
    let mut iVar2_17: *mut astruct_139;
    let mut iVar2_18: *mut astruct_139;
    let mut iVar2_19: *mut astruct_139;
    let mut iVar2_20: *mut astruct_139;
    let mut iVar2_21: *mut astruct_139;
    let mut iVar2_22: *mut astruct_139;
    let mut iVar2_23: *mut astruct_139;
    let mut iVar2_24: *mut astruct_139;
    let mut iVar2_25: *mut astruct_139;
    let mut iVar2_26: *mut astruct_139;
    let mut iVar2_27: *mut astruct_139;
    let mut iVar2_28: *mut astruct_139;
    let mut iVar2_29: *mut astruct_139;
    let mut iVar2_30: *mut astruct_139;
    let mut iVar2_31: *mut astruct_139;
    let mut iVar2_32: *mut astruct_139;
    let mut iVar2_33: *mut astruct_139;
    let mut iVar2_34: *mut astruct_139;
    let mut iVar2_35: *mut astruct_139;
    let mut iVar2_36: *mut astruct_139;
    let mut iVar2_37: *mut astruct_139;
    let mut iVar2_38: *mut astruct_139;
    let mut iVar2_39: *mut astruct_139;
    let mut iVar2_40: *mut astruct_139;
    let mut iVar2_41: *mut astruct_139;
    let mut iVar2_42: *mut astruct_139;
    let mut iVar2_43: *mut astruct_139;
    let mut iVar2_44: *mut astruct_139;
    let mut iVar2_45: *mut astruct_139;
    let mut iVar2_46: *mut astruct_139;
    let mut iVar2_47: *mut astruct_139;
    let mut iVar2_48: *mut astruct_139;
    let mut iVar2_49: *mut astruct_139;
    let mut iVar2_50: *mut astruct_139;
    let mut uVar3: u16;
    let mut uVar14: u16;

    pSVar2 = CONCAT22(in_register_0000000a, param_1);
    _u16_1050_06e0 = param_2;
    if (_PTR_LOOP_1050_5f2c == 0) {
        PTR_LOOP_1050_5f2c = mem_op_1000_160a(pSVar2);
        PTR_LOOP_1050_5f2e = pSVar2;
    } else {
    }
    uVar1 = fn_ptr_op_1000_1708(0x1aa, 0x0, 0x1, PTR_LOOP_1050_5f2c, PTR_LOOP_1050_5f2e);
    param_2 = uVar1;
    (param_2 + 0x2) = PTR_LOOP_1050_5f2e;
    uVar3 = (param_2 >> 0x10);
    let mut iVar2 = param_2;
    iVar2.field5_0x6 = 0x6e4;
    iVar2.field6_0x8 = 0x1050;
    (param_2 + 0xa) = 0x3;
    uVar14 = (param_2 >> 0x10);
    iVar3 = param_2;
    iVar3[0x1].field2_0x2 = 0x6ea;
    iVar3[0x1].field_0x4 = 0x1050;
    (param_2 + 0x10) = 0x2;
    uVar14 = (param_2 >> 0x10);
    iVar4 = param_2;
    iVar4[0x1].field6_0x8 = 0x6ee;
    (iVar4 + 0x2) = 0x1050;
    (param_2 + 0x16) = 0x2;
    uVar14 = (param_2 >> 0x10);
    iVar5 = param_2;
    iVar5[0x2].field_0x4 = 0x6f2;
    iVar5[0x2].field5_0x6 = 0x1050;
    (param_2 + 0x1c) = 0x2;
    uVar14 = (param_2 >> 0x10);
    iVar6 = param_2;
    (iVar6 + 0x3) = 0x6f6;
    iVar6[0x3].field2_0x2 = 0x1050;
    (param_2 + 0x22) = 0x4;
    uVar14 = (param_2 >> 0x10);
    iVar7 = param_2;
    iVar7[0x3].field5_0x6 = 0x6fe;
    iVar7[0x3].field6_0x8 = 0x1050;
    (param_2 + 0x28) = 0x2;
    uVar14 = (param_2 >> 0x10);
    iVar8 = param_2;
    iVar8[0x4].field2_0x2 = 0x702;
    iVar8[0x4].field_0x4 = 0x1050;
    (param_2 + 0x2e) = 0x3;
    uVar14 = (param_2 >> 0x10);
    iVar9 = param_2;
    iVar9[0x4].field6_0x8 = 0x708;
    (iVar9 + 0x5) = 0x1050;
    (param_2 + 0x34) = 0x3;
    uVar14 = (param_2 >> 0x10);
    iVar10 = param_2;
    iVar10[0x5].field_0x4 = 0x70e;
    iVar10[0x5].field5_0x6 = 0x1050;
    (param_2 + 0x3a) = 0x3;
    uVar14 = (param_2 >> 0x10);
    iVar11 = param_2;
    (iVar11 + 0x6) = 0x714;
    iVar11[0x6].field2_0x2 = 0x1050;
    (param_2 + 0x40) = 0x3;
    uVar14 = (param_2 >> 0x10);
    iVar12 = param_2;
    iVar12[0x6].field5_0x6 = 0x71a;
    iVar12[0x6].field6_0x8 = 0x1050;
    (param_2 + 0x46) = 0x2;
    uVar14 = (param_2 >> 0x10);
    iVar2_00 = param_2;
    iVar2_00[0x7].field2_0x2 = 0x71e;
    iVar2_00[0x7].field_0x4 = 0x1050;
    (param_2 + 0x4c) = 0x7;
    uVar14 = (param_2 >> 0x10);
    iVar2_01 = param_2;
    iVar2_01[0x7].field6_0x8 = 0x72c;
    (iVar2_01 + 0x8) = 0x1050;
    (param_2 + 0x52) = 0x6;
    uVar14 = (param_2 >> 0x10);
    iVar2_02 = param_2;
    iVar2_02[0x8].field_0x4 = 0x738;
    iVar2_02[0x8].field5_0x6 = 0x1050;
    (param_2 + 0x58) = 0x3;
    uVar14 = (param_2 >> 0x10);
    iVar2_03 = param_2;
    (iVar2_03 + 0x9) = 0x73e;
    iVar2_03[0x9].field2_0x2 = 0x1050;
    (param_2 + 0x5e) = 0x3;
    uVar14 = (param_2 >> 0x10);
    iVar2_04 = param_2;
    iVar2_04[0x9].field5_0x6 = 0x744;
    iVar2_04[0x9].field6_0x8 = 0x1050;
    (param_2 + 0x64) = 0x4;
    uVar14 = (param_2 >> 0x10);
    iVar2_05 = param_2;
    iVar2_05[0xa].field2_0x2 = 0x74c;
    iVar2_05[0xa].field_0x4 = 0x1050;
    (param_2 + 0x6a) = 0x2;
    uVar14 = (param_2 >> 0x10);
    iVar2_06 = param_2;
    iVar2_06[0xa].field6_0x8 = 0x750;
    (iVar2_06 + 0xb) = 0x1050;
    (param_2 + 0x70) = 0x3;
    uVar14 = (param_2 >> 0x10);
    iVar2_07 = param_2;
    iVar2_07[0xb].field_0x4 = 0x756;
    iVar2_07[0xb].field5_0x6 = 0x1050;
    (param_2 + 0x76) = 0x2;
    uVar14 = (param_2 >> 0x10);
    iVar2_08 = param_2;
    (iVar2_08 + 0xc) = 0x75a;
    iVar2_08[0xc].field2_0x2 = 0x1050;
    (param_2 + 0x7c) = 0x2;
    uVar14 = (param_2 >> 0x10);
    iVar2_09 = param_2;
    iVar2_09[0xc].field5_0x6 = 0x75e;
    iVar2_09[0xc].field6_0x8 = 0x1050;
    (param_2 + 0x82) = 0x3;
    uVar14 = (param_2 >> 0x10);
    iVar13 = param_2;
    (iVar13 + 0x84) = 0x764;
    (iVar13 + 0x86) = 0x1050;
    (param_2 + 0x88) = 0x3;
    uVar14 = (param_2 >> 0x10);
    iVar2_10 = param_2;
    iVar2_10[0xd].field6_0x8 = 0x76a;
    (iVar2_10 + 0xe) = 0x1050;
    (param_2 + 0x8e) = 0x3;
    uVar14 = (param_2 >> 0x10);
    iVar2_11 = param_2;
    iVar2_11[0xe].field_0x4 = 0x770;
    iVar2_11[0xe].field5_0x6 = 0x1050;
    (param_2 + 0x94) = 0x2;
    uVar14 = (param_2 >> 0x10);
    iVar13 = param_2;
    (iVar13 + 0x96) = 0x774;
    (iVar13 + 0x98) = 0x1050;
    (param_2 + 0x9a) = 0x4;
    uVar14 = (param_2 >> 0x10);
    iVar2_12 = param_2;
    iVar2_12[0xf].field5_0x6 = 0x77c;
    iVar2_12[0xf].field6_0x8 = 0x1050;
    (param_2 + 0xa0) = 0x2;
    uVar14 = (param_2 >> 0x10);
    iVar2_13 = param_2;
    iVar2_13[0x10].field2_0x2 = 0x780;
    iVar2_13[0x10].field_0x4 = 0x1050;
    (param_2 + 0xa6) = 0x1;
    uVar14 = (param_2 >> 0x10);
    iVar2_14 = param_2;
    iVar2_14[0x10].field6_0x8 = 0x782;
    (iVar2_14 + 0x11) = 0x1050;
    (param_2 + 0xac) = 0x2;
    uVar14 = (param_2 >> 0x10);
    iVar13 = param_2;
    (iVar13 + 0xae) = 0x786;
    (iVar13 + 0xb0) = 0x1050;
    (param_2 + 0xb2) = 0x2;
    uVar14 = (param_2 >> 0x10);
    iVar2_15 = param_2;
    (iVar2_15 + 0x12) = 0x78a;
    iVar2_15[0x12].field2_0x2 = 0x1050;
    (param_2 + 0xb8) = 0x2;
    uVar14 = (param_2 >> 0x10);
    iVar2_16 = param_2;
    iVar2_16[0x12].field5_0x6 = 0x78e;
    iVar2_16[0x12].field6_0x8 = 0x1050;
    (param_2 + 0xbe) = 0x2;
    uVar14 = (param_2 >> 0x10);
    iVar2_17 = param_2;
    iVar2_17[0x13].field2_0x2 = 0x792;
    iVar2_17[0x13].field_0x4 = 0x1050;
    (param_2 + 0xc4) = 0x2;
    uVar14 = (param_2 >> 0x10);
    iVar13 = param_2;
    (iVar13 + 0xc6) = 0x796;
    (iVar13 + 0xc8) = 0x1050;
    (param_2 + 0xca) = 0x4;
    uVar14 = (param_2 >> 0x10);
    iVar2_18 = param_2;
    iVar2_18[0x14].field_0x4 = 0x79e;
    iVar2_18[0x14].field5_0x6 = 0x1050;
    (param_2 + 0xd0) = 0x1;
    uVar14 = (param_2 >> 0x10);
    iVar13 = param_2;
    (iVar13 + 0xd2) = 0x7a0;
    (iVar13 + 0xd4) = 0x1050;
    (param_2 + 0xd6) = 0x2;
    uVar14 = (param_2 >> 0x10);
    iVar2_19 = param_2;
    iVar2_19[0x15].field5_0x6 = 0x7a4;
    iVar2_19[0x15].field6_0x8 = 0x1050;
    (param_2 + 0xdc) = 0x1;
    uVar14 = (param_2 >> 0x10);
    iVar2_20 = param_2;
    iVar2_20[0x16].field2_0x2 = 0x7a6;
    iVar2_20[0x16].field_0x4 = 0x1050;
    (param_2 + 0xe2) = 0x6;
    uVar14 = (param_2 >> 0x10);
    iVar2_21 = param_2;
    iVar2_21[0x16].field6_0x8 = 0x7b2;
    (iVar2_21 + 0x17) = 0x1050;
    (param_2 + 0xe8) = 0x1;
    uVar14 = (param_2 >> 0x10);
    iVar2_22 = param_2;
    iVar2_22[0x17].field_0x4 = 0x7b4;
    iVar2_22[0x17].field5_0x6 = 0x1050;
    (param_2 + 0xee) = 0x3;
    uVar14 = (param_2 >> 0x10);
    iVar2_23 = param_2;
    (iVar2_23 + 0x18) = 0x7ba;
    iVar2_23[0x18].field2_0x2 = 0x1050;
    (param_2 + 0xf4) = 0x2d;
    uVar14 = (param_2 >> 0x10);
    iVar2_24 = param_2;
    iVar2_24[0x18].field5_0x6 = 0x814;
    iVar2_24[0x18].field6_0x8 = 0x1050;
    (param_2 + 0xfa) = 0x3;
    uVar14 = (param_2 >> 0x10);
    iVar2_25 = param_2;
    iVar2_25[0x19].field2_0x2 = 0x81a;
    iVar2_25[0x19].field_0x4 = 0x1050;
    (param_2 + 0x100) = 0x1;
    uVar14 = (param_2 >> 0x10);
    iVar2_26 = param_2;
    iVar2_26[0x19].field6_0x8 = 0x81c;
    (iVar2_26 + 0x1a) = 0x1050;
    (param_2 + 0x106) = 0x4b;
    uVar14 = (param_2 >> 0x10);
    iVar2_27 = param_2;
    iVar2_27[0x1a].field_0x4 = 0x8b2;
    iVar2_27[0x1a].field5_0x6 = 0x1050;
    (param_2 + 0x10c) = 0x6;
    uVar14 = (param_2 >> 0x10);
    iVar2_28 = param_2;
    (iVar2_28 + 0x1b) = 0x8be;
    iVar2_28[0x1b].field2_0x2 = 0x1050;
    (param_2 + 0x112) = 0x4;
    uVar14 = (param_2 >> 0x10);
    iVar2_29 = param_2;
    iVar2_29[0x1c].field2_0x2 = 0x8c6;
    iVar2_29[0x1c].field_0x4 = 0x1050;
    (param_2 + 0x11e) = 0x35;
    uVar14 = (param_2 >> 0x10);
    iVar2_30 = param_2;
    iVar2_30[0x1c].field6_0x8 = 0x930;
    (iVar2_30 + 0x1d) = 0x1050;
    (param_2 + 0x124) = 0x2e;
    uVar14 = (param_2 >> 0x10);
    iVar2_31 = param_2;
    iVar2_31[0x1b].field5_0x6 = 0x98c;
    iVar2_31[0x1b].field6_0x8 = 0x1050;
    (param_2 + 0x118) = 0x1;
    uVar14 = (param_2 >> 0x10);
    iVar2_32 = param_2;
    iVar2_32[0x1d].field_0x4 = 0x98e;
    iVar2_32[0x1d].field5_0x6 = 0x1050;
    (param_2 + 0x12a) = 0x9;
    uVar14 = (param_2 >> 0x10);
    iVar2_33 = param_2;
    (iVar2_33 + 0x1e) = 0x9a0;
    iVar2_33[0x1e].field2_0x2 = 0x1050;
    (param_2 + 0x130) = 0x1a;
    uVar14 = (param_2 >> 0x10);
    iVar13 = param_2;
    (iVar13 + 0x132) = 0x9d4;
    (iVar13 + 0x134) = 0x1050;
    (param_2 + 0x136) = 0x8;
    uVar14 = (param_2 >> 0x10);
    iVar2_34 = param_2;
    iVar2_34[0x1f].field2_0x2 = 0x9e4;
    iVar2_34[0x1f].field_0x4 = 0x1050;
    (param_2 + 0x13c) = 0x4a;
    uVar14 = (param_2 >> 0x10);
    iVar2_35 = param_2;
    iVar2_35[0x20].field_0x4 = 0xa78;
    iVar2_35[0x20].field5_0x6 = 0x1050;
    (param_2 + 0x148) = 0x2;
    uVar14 = (param_2 >> 0x10);
    iVar2_36 = param_2;
    (iVar2_36 + 0x21) = 0xa7c;
    iVar2_36[0x21].field2_0x2 = 0x1050;
    (param_2 + 0x14e) = 0x1;
    uVar14 = (param_2 >> 0x10);
    iVar2_37 = param_2;
    iVar2_37[0x21].field5_0x6 = 0xa7e;
    iVar2_37[0x21].field6_0x8 = 0x1050;
    (param_2 + 0x154) = 0x1;
    uVar14 = (param_2 >> 0x10);
    iVar2_38 = param_2;
    iVar2_38[0x22].field2_0x2 = 0xa80;
    iVar2_38[0x22].field_0x4 = 0x1050;
    (param_2 + 0x15a) = 0x3;
    uVar14 = (param_2 >> 0x10);
    iVar2_39 = param_2;
    iVar2_39[0x22].field6_0x8 = 0xa86;
    (iVar2_39 + 0x23) = 0x1050;
    (param_2 + 0x160) = 0x2;
    uVar14 = (param_2 >> 0x10);
    iVar2_40 = param_2;
    (iVar2_40 + 0x24) = 0xa8a;
    iVar2_40[0x24].field2_0x2 = 0x1050;
    (param_2 + 0x16c) = 0x1b;
    uVar14 = (param_2 >> 0x10);
    iVar2_41 = param_2;
    iVar2_41[0x24].field5_0x6 = 0xac0;
    iVar2_41[0x24].field6_0x8 = 0x1050;
    (param_2 + 0x172) = 0x16;
    uVar14 = (param_2 >> 0x10);
    iVar2_42 = param_2;
    iVar2_42[0x25].field2_0x2 = 0xaec;
    iVar2_42[0x25].field_0x4 = 0x1050;
    (param_2 + 0x178) = 0x3e;
    uVar14 = (param_2 >> 0x10);
    iVar2_43 = param_2;
    iVar2_43[0x25].field6_0x8 = 0xb68;
    (iVar2_43 + 0x26) = 0x1050;
    (param_2 + 0x17e) = 0x46;
    uVar14 = (param_2 >> 0x10);
    iVar2_44 = param_2;
    iVar2_44[0x26].field_0x4 = 0xbf4;
    iVar2_44[0x26].field5_0x6 = 0x1050;
    (param_2 + 0x184) = 0x1;
    uVar14 = (param_2 >> 0x10);
    iVar2_45 = param_2;
    (iVar2_45 + 0x27) = 0xbf6;
    iVar2_45[0x27].field2_0x2 = 0x1050;
    (param_2 + 0x18a) = 0x3;
    uVar14 = (param_2 >> 0x10);
    iVar2_46 = param_2;
    iVar2_46[0x27].field5_0x6 = 0xbfc;
    iVar2_46[0x27].field6_0x8 = 0x1050;
    (param_2 + 0x190) = 0x3;
    uVar14 = (param_2 >> 0x10);
    iVar2_47 = param_2;
    iVar2_47[0x28].field2_0x2 = 0xc02;
    iVar2_47[0x28].field_0x4 = 0x1050;
    (param_2 + 0x196) = 0xa;
    uVar14 = (param_2 >> 0x10);
    iVar2_48 = param_2;
    iVar2_48[0x28].field6_0x8 = 0xc16;
    (iVar2_48 + 0x29) = 0x1050;
    (param_2 + 0x19c) = 0x24;
    uVar14 = (param_2 >> 0x10);
    iVar2_49 = param_2;
    iVar2_49[0x29].field_0x4 = 0xc5e;
    iVar2_49[0x29].field5_0x6 = 0x1050;
    (param_2 + 0x1a2) = 0x2;
    uVar14 = (param_2 >> 0x10);
    iVar2_50 = param_2;
    (iVar2_50 + 0x2a) = 0xc62;
    iVar2_50[0x2a].field2_0x2 = 0x1050;
    (param_2 + 0x1a8) = 0x44;
    return;
}