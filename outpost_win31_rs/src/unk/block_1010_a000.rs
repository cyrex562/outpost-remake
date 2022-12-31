pub fn struct_1010_a1d8(param_1: *mut Struct19, mut param_2: u16) {
    let mut iVar1: i16;
    let mut ppcVar2: *mut *mut code;
    let mut in_EDX: u32;
    let mut uVar3: u16;
    let mut unaff_SI: u16;
    let mut paVar4: *mut Struct19;
    let mut puVar5: *mut u32;
    let mut in_stack_0000fe4e: u16;
    let mut in_stack_0000ff72: u16;
    let mut in_stack_0000ff78: u16;
    let mut in_stack_0000ff7c: u16;
    let mut uStack4: u16;

    uVar3 = (in_EDX >> 0x10);
    paVar4 = struct_op_1010_1d48(param_1, param_2);
    (param_1 + 0xa) = 0x389a;
    (param_1 + 0xc) = 0x1008;
    (param_1 + 0xa) = 0x3aa8;
    (param_1 + 0xc) = 0x1008;
    (param_1 + 0x138) = 0;
    param_1.offset_0x0 = 0xe9cc;
    (param_1 + 0x2) = 0x1010;
    (param_1 + 0xa) = 0xe9dc;
    (param_1 + 0xc) = 0x1010;
    puVar5 = mixed_1010_20ba(
        CONCAT22(uVar3, (paVar4 >> 0x10)),
        _u16_1050_0ed0,
        CONCAT22(unaff_SI, 0x2f),
        in_stack_0000fe4e,
        in_stack_0000ff72,
        in_stack_0000ff78,
        in_stack_0000ff7c,
    );
    (param_1 + 0x138) = puVar5;
    (param_1 + 0x13a) = (puVar5 >> 0x10);
    ppcVar2 = ((param_1 + 0x138) + 0x4);
    (**ppcVar2)();
    pass1_1000_4906((param_1 & 0xffff0000 | (param_1 + 0xa4)), NULL, 0x94);
    pass1_1000_4906((param_1 & 0xffff0000 | (param_1 + 0xe)), NULL, 0x96);
    uStack4 = 0;
    loop {
        iVar1 = param_1 + uStack4 * 0x6;
        *(iVar1 + 0xe) = pass1_1010_c7e2;
        (iVar1 + 0x12) = 0;
        uStack4 += 0x1;
        if uStack4 >= 0x19 {
            break;
        }
    }
    *(param_1 + 0x4a) = pass1_1010_c864;
    (param_1 + 0x4e) = 0;
    *(param_1 + 0x50) = pass1_1010_cc56;
    (param_1 + 0x54) = 0;
    *(param_1 + 0x56) = pass1_1010_cf36;
    (param_1 + 0x5a) = 0;
    *(param_1 + 0x2c) = pass1_1010_d24a;
    (param_1 + 0x30) = 0;
    *(param_1 + 0x6e) = pass1_1010_d448;
    (param_1 + 0x72) = 0;
    *(param_1 + 0x74) = pass1_1010_d5ae;
    (param_1 + 0x78) = 0;
    *(param_1 + 0x98) = pass1_1010_d710;
    (param_1 + 0x9c) = 0;
    return;
}

pub fn pass1_1010_a478(param_1: *mut StructD) {
    let mut puVar1: *mut u16;
    let mut uVar2: u16;
    let mut uVar3: *mut StructD;
    let mut uVar4: u16;
    let mut puStack6: *mut u16;

    uVar4 = (param_1 >> 0x10);
    uVar3 = param_1;
    param_1.address_offset_field_0x0 = 0xe9cc;
    uVar3.address_offset_field_0x2 = 0x1010;
    uVar3.field6_0xa = 0xe9dc;
    uVar3.field7_0xc = 0x1010;
    if (&uVar3[0x1].field_0x4a != 0) {
        if (param_1.is_null()) {
            puVar1 = null_mut();
            uVar2 = 0;
        } else {
            puVar1 = &uVar3.field6_0xa;
            uVar2 = uVar4;
        }
        pass1_1010_1ea6(&uVar3[0x1].field_0x4a, CONCAT22(uVar2, puVar1));
    }
    uVar3[0x1].field_0x4a = 0;
    if (param_1.is_null()) {
        puVar1 = null_mut();
        uVar4 = 0;
    } else {
        puVar1 = &uVar3.field6_0xa;
    }
    puStack6 = CONCAT22(uVar4, puVar1);
    *puStack6 = 0x389a;
    puVar1[0x1] = 0x1008;
    pass1_1010_1d80(param_1);
    return;
}


pub fn pass1_1010_a50c(
    param_1: *mut Struct20,
    param_2: *mut *mut u8,
    param_3: *mut StructD,
) {
    let mut iVar1: i16;
    let mut struct_1: *mut Struct20;
    let mut local_8: u32;
    let mut iStack4: i16;

    struct_1 = param_1;
    struct_1 = &struct_1.field133_0xa4;
    pass1_1000_4906((param_1 & 0xffff0000 | ZEXT24(struct_1)), NULL, 0x94);
    iVar1 = (param_3 + 0xa);
    local_8 = (&struct_1.field6_0xe + iVar1 * 0x3);
    iStack4 = (&struct_1.field7_0x10 + iVar1 * 0x6 + 2);
    (local_8)(
        0x1000,
        &struct_1.offset_0x0 + iStack4,
        param_1,
        param_2,
        param_3,
    );
    return;
}


pub fn pass1_1010_a5ca(
    mut param_1: u16,
    mut param_2: u16,
    mut param_3: u16,
    mut param_4: u16,
    mut param_5: u16,
) {
    pass1_1030_8344(_u16_1050_5748, 0x8000001);
    pass1_1030_2242(CONCAT22(param_2, param_1), param_5);
    return;
}


pub fn pass1_1010_a69c(mut param_1: i16, mut param_2: u16, param_3: u32, mut param_4: u16) {
    let mut iVar1: i16;
    let mut uVar2: u16;
    let mut uVar3: u16;
    let mut puVar4: *mut u8;
    let mut in_register_0000000a: u16;
    let mut paVar5: *mut Struct57;
    let mut puVar6: *mut u32;
    let mut paVar7: *mut astruct_25;
    let mut paVar8: *mut astruct_67;
    let mut paVar9: *mut Struct27;
    let mut in_stack_0000fe74: u16;
    let mut in_stack_0000fe76: u16;
    let mut in_stack_0000fe78: u16;
    let mut in_stack_0000fe7a: u16;
    let mut in_stack_0000ff98: u16;
    let mut in_stack_0000ff9a: u16;
    let mut in_stack_0000ff9c: u16;
    let mut in_stack_0000ff9e: u16;
    let mut in_stack_0000ffa0: u16;
    let mut in_stack_0000ffa2: u16;
    let mut in_stack_0000ffa4: u16;
    let mut in_stack_0000ffa6: u16;
    let mut in_stack_0000ffa8: u16;
    let mut uVar10: u16;
    let mut uVar11: u16;
    let mut in_stack_0000ffd2: u16;
    let mut uStack22: u16;
    let mut iStack20: i16;

    paVar5 = CONCAT22(in_register_0000000a, param_2);
    pass1_1030_8344(_u16_1050_5748, 0x8000001);
    uVar3 = SUB42(paVar5, 0x0);
    if (param_4 == 1) {
        // for (iStack20 = 0; iStack20 < 0x83; iStack20 += 1)
        for iStack20 in 0..0x83 {
            iVar1 = pass1_1030_2242(CONCAT22(uVar3, param_1), iStack20);
            if (0x19 < iVar1) {
                uStack22 = iVar1 - 0x5;
                if (uStack22 < 0x19) {
                    uStack22 = 0x19;
                }
                pass1_1030_25d8(CONCAT22(uVar3, param_1), uStack22, iStack20);
            }
        }
        // TODO: goto switchD_1010_aaef_caseD_b;
    }
    pass1_1030_25f0(CONCAT22(uVar3, param_1), 0x0, param_4);
    puVar6 = mixed_1010_20ba(
        paVar5,
        _u16_1050_0ed0,
        CONCAT22(in_stack_0000ffd2, 0x35),
        in_stack_0000fe7a,
        in_stack_0000ff9e,
        in_stack_0000ffa4,
        in_stack_0000ffa8,
    );
    paVar5 = (paVar5 & 0xffff0000 | puVar6 >> 0x10);
    puVar4 = (puVar6 >> 0x10);
    uVar2 = param_3;
    uVar10 = (param_3 >> 0x10);
    match param_4 {
        0xa | 0xc => {
            iVar1 = 0x1b;
        }
        // break;
        _ => {}
        // TODO: goto switchD_1010_aaef_caseD_b;
        0x10 => {
            pass1_1010_682e(puVar6, 0x1, 0x2d);
            //    if ((param_1 + 0x160) == 0) goto switchD_1010_aaef_caseD_b;
            iVar1 = 0x2d;
        }
        // TODO: goto LAB_1010_a91f;
        0x12 => {
            pass1_1010_682e(puVar6, 0x1, 0x16);
            pass1_1010_682e(puVar6, 0x1, 0x17);
            pass1_1010_682e(puVar6, 0x1, 0x18);
            pass1_1010_682e(puVar6, 0x1, 0x40);
            iVar1 = 0x3f;
        }
        // TODO: goto LAB_1010_a96c;
        0x13 => {
            iVar1 = 0x35;
        }
        // TODO: goto LAB_1010_a91f;
        0x19 => {}
        // TODO: goto switchD_1010_aaef_caseD_19;
        0x1a => {
            iVar1 = 0xf;
        }
        // TODO: goto LAB_1010_a96c;
        0x1c => {
            iVar1 = 0x11;
        }
        // TODO: goto LAB_1010_a96c;
        0x1d | 0x24 => {
            pass1_1010_abd2(puVar4, uVar2, uVar10, 0x1e);
            iVar1 = 0x5b;
        }
        // TODO: goto LAB_1010_a91f;
        0x1e => {
            uVar2 = 0x1;
            iVar1 = 0x2;
            puVar6 = mixed_1010_20ba(
                paVar5,
                _u16_1050_0ed0,
                0x1002b,
                in_stack_0000fe76,
                in_stack_0000ff9a,
                in_stack_0000ffa0,
                in_stack_0000ffa4,
            );
            paVar5 = (paVar5 & 0xffff0000 | puVar6 >> 0x10);
            pass1_1010_08c0(puVar6, uVar2, iVar1);
            paVar7 = mixed_1010_20ba(
                paVar5,
                _u16_1050_0ed0,
                CONCAT22(in_stack_0000ffd2, 0x40),
                in_stack_0000fe7a,
                in_stack_0000ff9e,
                in_stack_0000ffa4,
                in_stack_0000ffa8,
            );
            paVar5 = (paVar5 & 0xffff0000 | paVar7 >> 0x10);
            load_str_and_sprintf_1008_b69c((paVar7 >> 0x10), paVar7);
        }
        // TODO: goto switchD_1010_aaef_caseD_b;
        0x22 => {
            iVar1 = 0x8;
        }
        // TODO: goto LAB_1010_aabe;
        0x23 => {
            iVar1 = 0xc;
        }
        // TODO: goto LAB_1010_aabe;
        0x25 => {
            pass1_1010_abd2(puVar4, uVar2, uVar10, 0x14);
            pass1_1010_abd2(paVar5, uVar2, uVar10, 0x1b);
            pass1_1010_abd2(paVar5, uVar2, uVar10, 0x1e);
            pass1_1010_abd2(paVar5, uVar2, uVar10, 0x22);
            pass1_1010_abd2(paVar5, uVar2, uVar10, 0x25);
            pass1_1010_abd2(paVar5, uVar2, uVar10, 0x28);
            pass1_1010_abd2(paVar5, uVar2, uVar10, 0x2a);
            pass1_1010_abd2(paVar5, uVar2, uVar10, 0x2d);
            pass1_1010_abd2(paVar5, uVar2, uVar10, 0x2f);
            pass1_1010_abd2(paVar5, uVar2, uVar10, 0x31);
            pass1_1010_abd2(paVar5, uVar2, uVar10, 0x35);
            pass1_1010_abd2(paVar5, uVar2, uVar10, 0x38);
            pass1_1010_abd2(paVar5, uVar2, uVar10, 0x3a);
            pass1_1010_abd2(paVar5, uVar2, uVar10, 0x3c);
            pass1_1010_abd2(paVar5, uVar2, uVar10, 0x48);
            pass1_1010_abd2(paVar5, uVar2, uVar10, 0x4f);
            pass1_1010_abd2(paVar5, uVar2, uVar10, 0x52);
            pass1_1010_abd2(paVar5, uVar2, uVar10, 0x54);
            pass1_1010_abd2(paVar5, uVar2, uVar10, 0x57);
            pass1_1010_abd2(paVar5, uVar2, uVar10, 0x5b);
            pass1_1010_abd2(paVar5, uVar2, uVar10, 0x5d);
            pass1_1010_abd2(paVar5, uVar2, uVar10, 0x62);
            pass1_1010_abd2(paVar5, uVar2, uVar10, 0x66);
            pass1_1010_abd2(paVar5, uVar2, uVar10, 0x68);
            pass1_1010_abd2(paVar5, uVar2, uVar10, 0x6c);
        }
        // TODO: goto switchD_1010_aaef_caseD_19;
        0x29 => {
            iVar1 = 0x25;
        }
        // break;
        0x2a => {
            iVar1 = 0xf;
        }
        // TODO: goto LAB_1010_aabe;
        0x2b => {
            iVar1 = 0x6e;
        }
        // TODO: goto LAB_1010_a96c;
        0x30 => {
            iVar1 = 0x54;
        }

        0x33 => {
            pass1_1010_abd2(puVar4, uVar2, uVar10, 0x31);
            iVar1 = 0x6c;
        }
        // TODO: goto LAB_1010_a91f;
        0x36 => {
            iVar1 = 0x13;
        }
        // TODO: goto LAB_1010_aabe;
        0x37 => {
            iVar1 = 0x2c;
            // LAB_1010_a96c:
            pass1_1010_682e(puVar6, 0x1, iVar1);
        }
        // TODO: goto switchD_1010_aaef_caseD_b;
        0x38 => {
            pass1_1010_682e(puVar6, 0x1, 0x28);
            //    if ((param_1 + 0x160) == 0) goto switchD_1010_aaef_caseD_b;
            iVar1 = 0x28;
        }
        // TODO: goto LAB_1010_a91f;
        0x39 => {
            iVar1 = 0x10;
        }
        // TODO: goto LAB_1010_aabe;
        0x3a => {
            iVar1 = 0x11;
        }
        // TODO: goto LAB_1010_aabe;
        0x3b => {
            iVar1 = 0x12; //
            // LAB_1010_aabe:
            pass1_1010_6814(puVar6, 0x1, iVar1);
        }
        // TODO: goto switchD_1010_aaef_caseD_b;
        0x3c => {
            pass1_1010_abd2(puVar4, uVar2, uVar10, 0x14);
            iVar1 = 0x62;
        }
        // TODO: goto LAB_1010_a91f;
        0x3d => {
            pass1_1010_682e(puVar6, 0x1, 0x66);
            //    if ((param_1 + 0x160) == 0) goto switchD_1010_aaef_caseD_b;
            iVar1 = 0x66; //
            // LAB_1010_a91f:
            pass1_1010_abd2(paVar5, uVar2, uVar10, iVar1);
        }
        // TODO: goto switchD_1010_aaef_caseD_b;
        0x3e => {
            iVar1 = 0x5d;
        }
        // break;
        0x3f => {
            iVar1 = 0x22;
        }
        // break;
        0x40 => {
            iVar1 = 0x57;
        }
        // break;
        0x41 => {
            iVar1 = 0x4f;
        }
    };
    pass1_1010_abd2(puVar4, uVar2, uVar10, iVar1);
    // switchD_1010_aaef_caseD_b:
    paVar8 = mixed_1010_20ba(
        paVar5,
        _u16_1050_0ed0,
        CONCAT22(param_4, 0x37),
        in_stack_0000fe78,
        in_stack_0000ff9c,
        in_stack_0000ffa2,
        in_stack_0000ffa6,
    );
    paVar5 = (paVar5 & 0xffff0000 | paVar8 >> 0x10);
    uVar2 = pass1_1008_ab12(paVar8, (paVar8 >> 0x10), param_4);
    if (uVar2 != 0) {
        post_win_msg_1008_a0e4(paVar8, 0x0, 0x0, 0x1, 0x0, uVar2);
    }
    post_win_msg_1008_a0e4(paVar8, 0x0, 0x0, 0x1, 0x0, 0x3d);
    uVar11 = 0x400;
    iVar1 = 0x6;
    uVar3 = 0x1;
    paVar9 = mixed_1010_20ba(
        paVar5,
        _u16_1050_0ed0,
        0x1002b,
        in_stack_0000fe74,
        in_stack_0000ff98,
        in_stack_0000ff9e,
        in_stack_0000ffa2,
    );
    pass1_1010_043a(paVar9, CONCAT22(uVar11, uVar3), iVar1);
    return;
    // switchD_1010_aaef_caseD_19:
    (puVar6 + 0x148) = 0x34;
    //   goto switchD_1010_aaef_caseD_b;
}

pub fn pass1_1010_abd2(
    param_1: *mut u8,
    mut param_2: u16,
    mut param_3: u16,
    mut param_4: i16,
) {
    let mut bVar1: bool;
    let mut piVar2: *mut i16;
    let mut in_register_0000000a: u16;
    let mut unaff_SI: u16;
    let mut puVar3: *mut u32;
    let mut in_stack_0000fe8a: u16;
    let mut in_stack_0000ffae: u16;
    let mut in_stack_0000ffb4: u16;
    let mut in_stack_0000ffb8: u16;
    let mut piStack20: *mut i16;
    let mut iStack16: i16;
    let mut iStack14: i16;

    puVar3 = mixed_1010_20ba(
        CONCAT22(in_register_0000000a, param_1),
        _u16_1050_0ed0,
        CONCAT22(unaff_SI, 0x35),
        in_stack_0000fe8a,
        in_stack_0000ffae,
        in_stack_0000ffb4,
        in_stack_0000ffb8,
    );
    bVar1 = false;
    iStack16 = param_4;
    piStack20 = CONCAT22(0x1050, &stack0x000a);
    loop {
        piVar2 = piStack20;
        if (iStack16 == 0) {
            return;
        }
        if (bVar1) {
            break;
        }
        if ((iStack16 * 0x2 + puVar3 + 0xa) != 0) {
            bVar1 = true;
            iStack14 = iStack16;
        }
        piStack20 = (piStack20 & 0xffff0000 | (piStack20 + 0x2));
        iStack16 = *piVar2;
    }
    pass1_1010_682e(puVar3, 0x0, iStack14);
    pass1_1010_682e(puVar3, 0x1, iStack16);
    return;
}

pub fn pass1_1010_ac62(
    mut param_1: u16,
    mut param_2: u16,
    mut param_3: u16,
    mut param_4: u16,
    mut param_5: i16,
) -> u16 {
    pass1_1030_8344(_u16_1050_5748, 0x8000001);
    return (param_1 + 0x116 + param_5 * 0x2);
}

pub fn pass1_1010_afde(mut param_1: u32, mut param_2: i16) {
    let mut uVar1: u32;
    let mut uVar2: u32;
    let mut in_EDX: u32;
    let mut uVar3: u32;
    let mut puVar4: *mut u32;

    uVar1 = (param_1 + 0x138);
    uVar2 = (uVar1 + 0x24);
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uVar2);
    uVar3 = in_EDX & 0xffff0000;
    if ((in_EDX | uVar2) == 0) {
        return;
    }
    puVar4 = pass1_1008_c6fa(_u16_1050_06e0, param_2);
    pass1_1038_4e78(
        puVar4,
        (uVar3 & 0xffff0000 | puVar4 >> 0x10),
        uVar2 & 0xffff | in_EDX << 0x10,
        puVar4,
    );
    return;
}
