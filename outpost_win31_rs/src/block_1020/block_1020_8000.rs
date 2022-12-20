pub unsafe fn pass1_1020_808e(param_1: *mut StructD) {
    let mut puVar1: *mut u8;
    let mut uVar2: u16;
    let mut iVar3: *mut StructD;
    let mut uVar3: u16;
    let mut puStack6: *mut u16;

    uVar3 = (param_1 >> 0x10);
    iVar3 = param_1;
    param_1.address_offset_field_0x0 = 0x82bc;
    iVar3.address_offset_field_0x2 = 0x1020;
    iVar3.field_0xe2 = 0x8358;
    iVar3.field_0xe4 = 0x1020;
    if (param_1.is_null()) {
        puVar1 = null_mut();
        uVar2 = 0;
    } else {
        puVar1 = &iVar3.field_0xe2;
        uVar2 = uVar3;
    }
    puStack6 = CONCAT22(uVar2, puVar1);
    *puStack6 = 0x389a;
    (puVar1 + 0x2) = 0x1008;
    pass1_1008_57c4((param_1 & 0xffff0000 | ZEXT24(&iVar3.field192_0xd2)));
    param_1.address_offset_field_0x0 = 0x380a;
    iVar3.address_offset_field_0x2 = 0x1008;
    param_1.address_offset_field_0x0 = 0x389a;
    iVar3.address_offset_field_0x2 = 0x1008;
    return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address


// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn pass1_1020_8360(param_1: *mut astruct_20) {
    let mut uVar1: u32;
    let mut uVar2: u16;
    let mut in_EDX: u32;
    let mut uVar3: u16;
    let mut unaff_SI: u16;
    let mut puVar4: *mut u16;
    let mut puVar5: *mut u32;
    let mut in_stack_0000fe96: u16;
    let mut in_stack_0000ffba: u16;
    let mut in_stack_0000ffc0: u16;
    let mut in_stack_0000ffc4: u16;
    let mut uVar6: u16;
    let mut iVar4: *mut astruct_20;

    uVar3 = (in_EDX >> 0x10);
    iVar4 = param_1;
    uVar6 = (param_1 >> 0x10);
    struct_1020_847a(param_1, 1);
    puVar4 = pass1_1008_3e38((param_1 & 0xffff0000 | ZEXT24(&iVar4.field9_0x16)));
    iVar4.field12_0x1c = 0;
    param_1.offset_0x0 = 0x8462;
    iVar4.base_0x2 = 0x1020;
    puVar5 = mixed_1010_20ba(
        CONCAT22(uVar3, (puVar4 >> 0x10)),
        _u16_1050_0ed0,
        CONCAT22(unaff_SI, 0x29),
        in_stack_0000fe96,
        in_stack_0000ffba,
        in_stack_0000ffc0,
        in_stack_0000ffc4,
    );
    uVar2 = (puVar5 >> 0x10);
    iVar4.field12_0x1c = puVar5;
    iVar4.field13_0x1e = uVar2;
    pass1_1018_26f8(
        iVar4.field12_0x1c,
        uVar2,
        (param_1 & 0xffff0000 | ZEXT24(&iVar4.field9_0x16)),
    );
    uVar1 = &iVar4.field12_0x1c;
    pass1_1020_8712(
        param_1 & 0xffff | uVar6 << 0x10,
        &iVar4.field3_0x8,
        (uVar1 + 0x2a),
        (param_1 & 0xffff0000 | ZEXT24(&iVar4.field9_0x16)),
    );
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn struct_1020_847a(param_1: *mut astruct_20, mut param_2: i16) {
    let mut uVar1: u16;
    let mut paVar2: *mut astruct_20;
    let mut in_EDX: u32;
    let mut uVar4: u16;
    let mut paVar3: *mut Struct57;
    let mut iVar3: *mut astruct_20;
    let mut unaff_SI: u16;
    let mut uVar5: u16;
    let mut puVar6: *mut u16;
    let mut puVar7: *mut u32;
    let mut in_stack_0000fe92: u16;
    let mut in_stack_0000ffb6: u16;
    let mut in_stack_0000ffbc: u16;
    let mut in_stack_0000ffc0: u16;

    uVar4 = (in_EDX >> 0x10);
    uVar5 = (param_1 >> 0x10);
    iVar3 = param_1;
    param_1.offset_0x0 = 0x389a;
    iVar3.base_0x2 = 0x1008;
    iVar3.field2_0x4 = 0;
    (&iVar3.field2_0x4 + 0x2) = param_2;
    iVar3.field3_0x8 = 0;
    iVar3.field5_0xc = 0;
    puVar6 = pass1_1008_3e38((param_1 & 0xffff0000 | ZEXT24(&iVar3.field7_0x10)));
    paVar3 = CONCAT22(uVar4, (puVar6 >> 0x10));
    param_1.offset_0x0 = 0x87aa;
    iVar3.base_0x2 = 0x1020;
    puVar7 = mixed_1010_20ba(
        paVar3,
        _u16_1050_0ed0,
        CONCAT22(unaff_SI, 0x48),
        in_stack_0000fe92,
        in_stack_0000ffb6,
        in_stack_0000ffbc,
        in_stack_0000ffc0,
    );
    paVar3 = (paVar3 & 0xffff0000 | puVar7 >> 0x10);
    pass1_1008_3f62(
        (param_1 & 0xffff0000 | ZEXT24(&iVar3.field7_0x10)),
        (puVar7 & 0xffff0000 | (puVar7 + 0xe)),
    );
    uVar1 = (&iVar3.field2_0x4 + 0x2) << 0x3;
    mem_op_1000_179c(uVar1, paVar3);
    iVar3.field3_0x8 = uVar1;
    iVar3.field4_0xa = paVar3;
    paVar2 = ((&iVar3.field2_0x4 + 0x2) << 0x2);
    mem_op_1000_179c(paVar2, paVar3);
    iVar3.field5_0xc = paVar2;
    iVar3.field6_0xe = paVar3;
    pass1_1000_4906(&iVar3.field3_0x8, NULL, (&iVar3.field2_0x4 + 0x2) << 0x3);
    pass1_1000_4906(&iVar3.field5_0xc, NULL, (&iVar3.field2_0x4 + 0x2) << 0x2);
    return;
}

// WARNING: Unable to use type for symbol uVar4
pub unsafe fn pass1_1020_8556(param_1: *mut StructD) {
    let mut piVar1: *mut i16;
    let mut uVar2: u16;
    let mut pcVar3: *mut c_char;
    let mut iVar5: *mut StructD;
    let mut iVar4: *mut astruct_589;
    let mut iVar6: i16;
    let mut uVar7: u16;
    let mut uVar8: u16;
    let mut iStack12: i16;
    let mut uVar4: u32;

    uVar7 = (param_1 >> 0x10);
    iVar5 = param_1;
    param_1.address_offset_field_0x0 = 0x87aa;
    iVar5.address_offset_field_0x2 = 0x1020;
    fn_ptr_1000_17ce(*&iVar5.field5_0x8);
    if ((iVar5.field8_0xe | iVar5.field7_0xc) != 0) {
        iStack12 = 0;
        loop {
            piVar1 = &iVar5.field_0x6;
            if (*piVar1 == iStack12 || *piVar1 < iStack12) {
                break;
            }
            iVar6 = iStack12 * 0x4;
            uVar4 = &iVar5.field7_0xc;
            uVar8 = (uVar4 >> 0x10);
            iVar4 = uVar4;
            if ((iVar4 + iVar6) != 0) {
                pcVar3 = *(iVar4 + iVar6);
                uVar2 = (iVar4 + iVar6 + 2);
                if ((uVar2 | pcVar3) != 0) {
                    pass1_1008_5118(pcVar3 & 0xffff | uVar2 << 0x10);
                    fn_ptr_1000_17ce(pcVar3);
                }
            }
            iStack12 += 0x1;
        }
        fn_ptr_1000_17ce(*&iVar5.field7_0xc);
    }
    param_1.address_offset_field_0x0 = 0x389a;
    iVar5.address_offset_field_0x2 = 0x1008;
    return;
}


pub unsafe fn pass1_1020_8712(
    mut param_1: u32,
    param_2: *mut i16,
    param_3: *mut astruct_76,
    param_4: *mut u16,
) {
    let mut uVar1: u16;
    let mut uVar2: u32;

    pass1_1008_3f32(param_4, (param_1 & 0xffff0000 | (param_1 + 0x10)));
    uVar2 = pass1_1008_4772(param_3);
    uVar1 = (uVar2 >> 0x10);
    pass1_1008_3e94(
        param_4,
        (param_2 & 0xffff0000 | ZEXT24((param_2 + 0x2))),
        (param_2 & 0xffff | param_2 << 0x10),
    );
    (param_2 + 0x4) = (uVar2 + 0x4) + *param_2;
    (param_2 + 0x6) = (uVar2 + 0x8) + (param_2 + 2);
    return;
}


// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn pass1_1020_87c2(param_1: *mut astruct_20) {
    let mut uVar1: u32;
    let mut iVar2: *mut astruct_20;
    let mut in_EDX: i32;
    let mut uVar2: u16;
    let mut unaff_SI: u16;
    let mut uVar3: u16;
    let mut puVar4: *mut u16;
    let mut puVar5: *mut u32;
    let mut in_stack_0000fe80: u16;
    let mut in_stack_0000ffa4: u16;
    let mut in_stack_0000ffaa: u16;
    let mut in_stack_0000ffae: u16;
    let mut local_12: [u8; 0x8] = [0; 0x8];
    let mut iStack10: i16;
    let mut paStack8: *mut Struct19;
    let mut iStack4: i16;

    struct_1020_847a(param_1, 0x4);
    iStack4 = 0x4;
    iVar2 = param_1;
    iVar2 = &iVar2.field9_0x16;
    paStack8 = (param_1 & 0xffff0000 | ZEXT24(iVar2));
    loop {
        uVar2 = (in_EDX >> 0x10);
        pass1_1008_3e38(paStack8);
        in_EDX = uVar2 << 0x10;
        paStack8 = (paStack8 & 0xffff0000 | (paStack8 + 0x6));
        iStack4 += -0x1;
        if iStack4 == 0 {
            break;
        }
    }
    uVar3 = (param_1 >> 0x10);
    (&iVar2.field17_0x2c + 0x2) = 0;
    puVar4 = pass1_1008_3e38((param_1 & 0xffff0000 | ZEXT24(&iVar2.field_0x32)));
    iVar2.field_0x38 = 0;
    param_1.offset_0x0 = 0x8a84;
    iVar2.base_0x2 = 0x1020;
    puVar5 = mixed_1010_20ba(
        CONCAT22(uVar2, (puVar4 >> 0x10)),
        _u16_1050_0ed0,
        CONCAT22(unaff_SI, 0x29),
        in_stack_0000fe80,
        in_stack_0000ffa4,
        in_stack_0000ffaa,
        in_stack_0000ffae,
    );
    (&iVar2.field17_0x2c + 0x2) = puVar5;
    iVar2.field18_0x30 = (puVar5 >> 0x10);
    iStack10 = 0;
    loop {
        uVar1 = (&iVar2.field17_0x2c + 2);
        pass1_1018_26d8(
            uVar1,
            (uVar1 >> 0x10),
            iStack10,
            (param_1 & 0xffff0000 | ZEXT24(&iVar2.field9_0x16 + iStack10 * 0x3)),
        );
        uVar1 = (&iVar2.field17_0x2c + 2);
        pass1_1020_8712(
            param_1 & 0xffff | uVar3 << 0x10,
            CONCAT22(iVar2.field4_0xa, iVar2.field3_0x8 + iStack10 * 0x8),
            (uVar1 + 0x2e + iStack10 * 0x4),
            (param_1 & 0xffff0000 | ZEXT24(&iVar2.field9_0x16 + iStack10 * 0x3)),
        );
        iStack10 += 0x1;
        if iStack4 >= 4 {
            break;
        }
    }
    uVar1 = (&iVar2.field17_0x2c + 2);
    pass1_1018_2548(
        uVar1,
        (uVar1 >> 0x10),
        (param_1 & 0xffff0000 | ZEXT24(&iVar2.field_0x32)),
    );
    uVar1 = (&iVar2.field17_0x2c + 2);
    iVar2.field_0x38 = (uVar1 + 0x6e);
    pass1_1020_8712(
        param_1 & 0xffff | uVar3 << 0x10,
        CONCAT22(0x1050, local_12),
        iVar2.field_0x38,
        (param_1 & 0xffff0000 | ZEXT24(&iVar2.field_0x32)),
    );
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn pass1_1020_8a9c(
    param_1: *mut astruct_20,
    mut param_2: u32,
    mut param_3: u16,
    mut param_4: u16,
    mut param_5: u16,
    mut param_6: u16,
) {
    let mut paVar1: *mut astruct_287;
    let mut uVar2: u32;
    let mut uVar3: *mut Struct19;
    let mut uVar4: u16;
    let mut uVar5: u16;
    let mut paVar6: *mut Struct57;
    let mut unaff_SI: u16;
    let mut puVar7: *mut u16;
    let mut puVar8: *mut u32;
    let mut paVar9: *mut astruct_76;
    let mut uVar10: u16;
    let mut puStack76: *mut u16;
    let mut local_48: [u8; 0x1e] = [0; 0x1e];
    let mut local_2a: [u8; 0x24] = [0; 0x24];
    let mut uStack6: u16;
    let mut uStack4: u16;
    let mut iVar9: *mut astruct_20;
    let mut uVar9: *mut astruct_20;

    uVar4 = (param_2 >> 0x10);
    iVar9 = param_1;
    uVar10 = (param_1 >> 0x10);
    struct_1020_847a(param_1, 0x2);
    uVar3 = &iVar9.field9_0x16;
    pass1_1008_3e38((param_1 & 0xffff0000 | ZEXT24(uVar3)));
    puStack76 = (param_1 & 0xffff0000 | ZEXT24(&iVar9.field12_0x1c));
    puVar7 = pass1_1008_3e38((param_1 & 0xffff0000 | ZEXT24(&iVar9.field12_0x1c)));
    paVar6 = CONCAT22(uVar4, (puVar7 >> 0x10));
    iVar9.field14_0x22 = null_mut();
    param_1.offset_0x0 = 0x8e92;
    iVar9.base_0x2 = 0x1020;
    puVar8 = mixed_1010_20ba(
        paVar6,
        _u16_1050_0ed0,
        CONCAT22(unaff_SI, 0x29),
        param_6,
        param_5,
        param_3,
        param_4,
    );
    uVar4 = (paVar6 >> 0x10);
    iVar9.field14_0x22 = puVar8;
    uVar5 = (puVar8 >> 0x10);
    (&iVar9.field14_0x22 + 0x2) = uVar5;
    pass1_1018_2678(
        &iVar9.field14_0x22,
        uVar5,
        (param_1 & 0xffff0000 | ZEXT24(uVar3)),
    );
    paVar9 = pass1_1018_268e(iVar9.field14_0x22);
    uStack4 = (paVar9 >> 0x10);
    paVar6 = CONCAT22(uVar4, uStack4);
    uVar4 = SUB42(paVar9, 0x0);
    uStack6 = uVar4;
    pass1_1020_8712(
        param_1,
        &iVar9.field3_0x8,
        paVar9,
        (param_1 & 0xffff0000 | ZEXT24(uVar3)),
    );
    paVar1 = iVar9.field14_0x22;
    pass1_1018_26c2(paVar1, (paVar1 >> 0x10), puStack76);
    uVar4 = FUN_1010_830a(uVar4, paVar6, 0x1018, _u16_1050_14cc, 0x2);
    struct_op_1008_48fe(
        paVar6,
        CONCAT22(0x1050, local_2a),
        0x1,
        CONCAT22(paVar6, uVar4),
    );
    struct_op_1008_3f92(CONCAT22(0x1050, local_48), CONCAT22(0x1050, local_2a));
    uVar2 = &iVar9.field3_0x8;
    pass1_1020_8712(
        param_1,
        (uVar2 & 0xffff0000 | (uVar2 + 0x8)),
        CONCAT22(0x1050, local_48),
        puStack76,
    );
    pass1_1008_41bc(CONCAT22(0x1050, local_48));
    close_file_1008_496c(CONCAT22(0x1050, local_2a));
    return;
}
pub unsafe fn pass1_1020_8bae(param_1: *mut u16) {
    *param_1 = 0x8e92;
    (param_1 + 0x2) = 0x1020;
    pass1_1020_8556(param_1);
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn invalidate_rect_1020_8d90(
    mut param_1: u16,
    mut param_2: u16,
    mut param_3: u32,
    mut param_4: u16,
    mut param_5: u32,
    mut param_6: u16,
    mut param_7: u16,
) {
    let mut uVar1: u32;
    let mut uVar2: u16;
    let mut in_register_0000000a: u16;
    let mut uVar3: u32;
    let mut iVar4: i16;
    let mut uVar5: u16;
    let mut uVar6: u32;
    let mut local_48: RECT16;
    let mut iStack68: i16;
    let mut iStack66: i16;
    let mut local_40: i16;
    let mut local_3e: i16;
    let mut uStack60: u32;
    let mut local_38: astruct_288 = astruct_288::default();
    let mut local_10: [u8; 0xa] = [0; 0xa];
    let mut uStack6: u16;
    let mut uStack4: u16;

    uVar5 = (param_3 >> 0x10);
    iVar4 = param_3;
    uVar2 = pass1_1018_266a((iVar4 + 0x22));
    if (uVar2 != 0) {
        uVar6 = pass1_1018_265c();
        uStack4 = (uVar6 >> 0x10);
        uStack6 = uVar6;
        uVar3 = CONCAT22(in_register_0000000a, uStack4 | uStack6);
        if ((uStack4 | uStack6) != 0) {
            sys_1000_3f9c(CONCAT22(0x1050, local_10), s__03ld_1050_442a, uStack6);
            uVar1 = (iVar4 + 0x22);
            file_and_draw_op_1008_4f20(
                uVar3,
                CONCAT22(0x1050, &local_38),
                (uVar1 + 0xe),
                0x25,
                CONCAT22(0x1050, local_10),
                param_6,
                param_7,
            );
            pass1_1008_4480(
                param_5,
                (param_3 & 0xffff0000 | (iVar4 + 0x1c)),
                CONCAT22(0x1050, &local_38),
            );
            uStack60 = pass1_1008_4772(CONCAT22(0x1050, &local_38));
            pass1_1008_3e94(
                (param_3 & 0xffff0000 | (iVar4 + 0x1c)),
                CONCAT22(0x1050, &local_40),
                CONCAT22(0x1050, &local_3e),
            );
            local_48.x = local_3e;
            local_48.y = local_40;
            uVar5 = (uStack60 >> 0x10);
            iStack68 = local_3e + (uStack60 + 0x4);
            iStack66 = local_40 + (uStack60 + 0x8);
            // just 0x0
            InvalidateRect16(0x0, &local_48, &DAT_1050_1050);
            pass1_1008_41bc(CONCAT22(0x1050, &local_38));
        }
    }
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn pass1_1020_8eaa(param_1: *mut astruct_20, mut param_2: u32) {
    let mut puVar1: *mut u8;
    let mut paVar2: *mut Struct19;
    let mut uVar3: u16;
    let mut uVar5: u16;
    let mut paVar6: *mut Struct57;
    let mut iVar4: *mut astruct_20;
    let mut unaff_SI: u16;
    let mut unaff_DI: u16;
    let mut uVar4: *mut astruct_20;
    let mut puVar8: *mut u16;
    let mut puVar9: *mut u32;
    let mut in_stack_0000fe9a: u16;
    let mut in_stack_0000fe9c: u16;
    let mut in_stack_0000ffbe: u16;
    let mut in_stack_0000ffc0: u16;
    let mut in_stack_0000ffc4: u16;
    let mut in_stack_0000ffc6: u16;
    let mut in_stack_0000ffc8: u16;
    let mut in_stack_0000ffca: u16;
    let mut local_a: [u8; 0x8] = [0; 0x8];
    let mut uVar7: u16;

    uVar7 = (param_2 >> 0x10);
    struct_1020_847a(param_1, 0x25);
    uVar4 = (param_1 >> 0x10);
    iVar4 = param_1;
    iVar4.field9_0x16 = 0;
    iVar4.field138_0xaa = 0;
    puVar1 = (&iVar4.field139_0xac + 2);
    puVar8 = pass1_1008_3e38((param_1 & 0xffff0000 | ZEXT24(puVar1)));
    paVar6 = CONCAT22(uVar7, (puVar8 >> 0x10));
    iVar4.field141_0xb4 = 0;
    iVar4.field143_0xb8 = 0xffff;
    iVar4.field144_0xba = 0;
    param_1.offset_0x0 = 0x9204;
    iVar4.base_0x2 = 0x1020;
    puVar9 = mixed_1010_20ba(
        paVar6,
        _u16_1050_0ed0,
        CONCAT22(unaff_SI, 0x29),
        in_stack_0000fe9a,
        in_stack_0000ffbe,
        in_stack_0000ffc4,
        in_stack_0000ffc8,
    );
    paVar6 = (paVar6 & 0xffff0000 | puVar9 >> 0x10);
    paVar2 = puVar9;
    iVar4.field9_0x16 = paVar2;
    uVar5 = (puVar9 >> 0x10);
    iVar4.field10_0x18 = uVar5;
    pass1_1018_2646(
        iVar4.field9_0x16,
        uVar5,
        (param_1 & 0xffff0000 | ZEXT24(puVar1)),
    );
    uVar3 = FUN_1010_830a(paVar2, paVar6, 0x1018, _u16_1050_14cc, 0x1ce);
    iVar4.field141_0xb4 = uVar3;
    iVar4.field142_0xb6 = paVar6;
    pass1_1020_8712(
        param_1 & 0xffff | ZEXT24(uVar4) << 0x10,
        CONCAT22(0x1050, local_a),
        CONCAT22(paVar6, iVar4.field141_0xb4),
        (param_1 & 0xffff0000 | ZEXT24(puVar1)),
    );
    puVar9 = mixed_1010_20ba(
        paVar6,
        _u16_1050_0ed0,
        CONCAT22(unaff_DI, 0x2),
        in_stack_0000fe9c,
        in_stack_0000ffc0,
        in_stack_0000ffc6,
        in_stack_0000ffca,
    );
    iVar4.field144_0xba = puVar9;
    iVar4.field145_0xbc = (puVar9 >> 0x10);
    return;
}
pub unsafe fn pass1_1020_8f74(param_1: *mut StructD) {
    let mut puVar1: *mut u32;
    let mut uVar2: u16;
    let mut ppcVar3: *mut *mut code;
    let mut iVar4: *mut StructD;
    let mut uVar4: u16;

    uVar4 = (param_1 >> 0x10);
    iVar4 = param_1;
    param_1.address_offset_field_0x0 = 0x9204;
    iVar4.address_offset_field_0x2 = 0x1020;
    puVar1 = &iVar4.field_0xb4;
    uVar2 = &iVar4.field_0xb6;
    if ((uVar2 | puVar1) != 0) {
        ppcVar3 = *puVar1;
        (**ppcVar3)();
    }
    pass1_1020_8556(param_1);
    return;
}
