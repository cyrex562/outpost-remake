
pub unsafe fn pass1_1030_e282(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    param_1.address_offset_field_0x0 = 0x389a;
    (param_1 + 0x2) = 0x1008;
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}
pub unsafe fn struct_1030_e2be(
    param_1: *mut astruct_97,
    mut param_2: u16,
    mut param_3: u32,
    mut param_4: u32,
) {
    let mut iVar1: *mut astruct_97;
    let mut uVar1: u16;

    struct_op_1028_d1dc(param_1, 0x2af7);
    uVar1 = (param_1 >> 0x10);
    iVar1 = param_1;
    iVar1.field259_0x108 = param_4;
    iVar1.field262_0x10c = param_3;
    iVar1.field264_0x110 = param_2;
    param_1.offset_0x0 = 0xe4ea;
    iVar1.segment_0x2 = 0x1030;
    return;
}


pub unsafe fn pass1_1030_e300(param_1: *mut u8, mut param_2: u32) -> u16 {
    let mut in_register_0000000a: u16;
    let mut paVar1: *mut Struct27;
    let mut in_stack_0000fea2: u16;
    let mut in_stack_0000ffc6: u16;
    let mut in_stack_0000ffcc: u16;
    let mut in_stack_0000ffd0: u16;
    let mut in_stack_0000fff8: u32;

    paVar1 = mixed_1010_20ba(
        CONCAT22(in_register_0000000a, param_1),
        _u16_1050_0ed0,
        CONCAT22((in_stack_0000fff8 >> 0x10), 0x2b),
        in_stack_0000fea2,
        in_stack_0000ffc6,
        in_stack_0000ffcc,
        in_stack_0000ffd0,
    );
    pass1_1010_089e(paVar1, (param_2 + 0x110), 0x2);
    return 0x1;
}

pub unsafe fn pass1_1030_e328(mut param_1: u16, mut param_2: u16, mut param_3: u32) -> u16 {
    let mut uVar1: u16;

    uVar1 = (param_3 >> 0x10);
    if ((param_3 + 0x110) == 0) {
        pass1_1030_e4ba();
    } else {
        pass1_1030_e410(param_1, param_2, param_3 & 0xffff | uVar1 << 0x10);
    }
    return 0x1;
}


pub unsafe fn pass1_1030_e34e(mut param_1: u16, param_2: *mut u8, param_3: *mut astruct_403) {
    let mut puVar1: *mut u32;
    let mut puVar2: *mut u32;
    let mut iVar3: i16;
    let mut uVar4: u16;
    let mut in_register_0000000a: u16;
    let mut paVar5: *mut Struct57;
    let mut iVar5: *mut astruct_403;
    let mut puVar6: *mut u32;
    let mut puVar7: *mut u32;
    let mut uVar8: u16;
    let mut puStack10: *mut u16;

    paVar5 = CONCAT22(in_register_0000000a, param_2);
    mem_op_1000_179c(0x112, paVar5);
    uVar4 = paVar5;
    puStack10 = CONCAT22(uVar4, param_1);
    if ((uVar4 | param_1) != 0) {
        *puStack10 = 0x389a;
        (param_1 + 0x2) = 0x1008;
        uVar8 = (param_3 >> 0x10);
        iVar5 = param_3;
        (param_1 + 0x4) = iVar5.field4_0x4;
        puVar6 = &iVar5.field5_0x8;
        puVar7 = (param_1 + 0x8);
        for iVar3 in 0x40..0 {
            puVar2 = puVar7;
            puVar7 = puVar7 + 1;
            puVar1 = puVar6;
            puVar6 = puVar6 + 1;
            *puVar2 = *puVar1;
        }
        *puStack10 = 0x6ad2;
        (param_1 + 0x2) = 0x1028;
        (param_1 + 0x108) = iVar5.field258_0x108;
        (param_1 + 0x10c) = iVar5.field259_0x10c;
        (param_1 + 0x110) = iVar5.field260_0x110;
        *puStack10 = 0xe4ea;
        (param_1 + 0x2) = 0x1030;
    }
    return;
}

pub unsafe fn pass1_1030_e4be(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    param_1.address_offset_field_0x0 = 0x389a;
    (param_1 + 0x2) = 0x1008;
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1030_e540() -> u16 {
    return 0x1;
}

pub unsafe fn pass1_1030_e546(mut param_1: u32) -> u16 {
    let mut uVar1: u32;

    uVar1 = (param_1 + 0x108);
    pass1_1028_e332(_PTR_LOOP_1050_65e2, uVar1, (uVar1 >> 0x10));
    return 0x1;
}


pub unsafe fn pass1_1030_e564(mut param_1: u16, param_2: *mut u8, param_3: *mut astruct_405) {
    let mut puVar1: *mut u32;
    let mut puVar2: *mut u32;
    let mut iVar3: i16;
    let mut uVar4: u16;
    let mut in_register_0000000a: u16;
    let mut paVar5: *mut Struct57;
    let mut iVar5: *mut astruct_405;
    let mut puVar6: *mut u32;
    let mut puVar7: *mut u32;
    let mut uVar8: u16;
    let mut puStack10: *mut u16;

    paVar5 = CONCAT22(in_register_0000000a, param_2);
    mem_op_1000_179c(0x10c, paVar5);
    uVar4 = paVar5;
    puStack10 = CONCAT22(uVar4, param_1);
    if ((uVar4 | param_1) != 0) {
        *puStack10 = 0x389a;
        (param_1 + 0x2) = 0x1008;
        uVar8 = (param_3 >> 0x10);
        iVar5 = param_3;
        (param_1 + 0x4) = iVar5.field4_0x4;
        puVar6 = &iVar5.field5_0x8;
        puVar7 = (param_1 + 0x8);
        for iVar3 in 0x40..0 {
            puVar2 = puVar7;
            puVar7 = puVar7 + 1;
            puVar1 = puVar6;
            puVar6 = puVar6 + 1;
            *puVar2 = *puVar1;
        }
        *puStack10 = 0x6ad2;
        (param_1 + 0x2) = 0x1028;
        (param_1 + 0x108) = iVar5.field258_0x108;
        *puStack10 = 0xe62e;
        (param_1 + 0x2) = 0x1030;
    }
    return;
}

pub unsafe fn pass1_1030_e602(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    param_1.address_offset_field_0x0 = 0x389a;
    (param_1 + 0x2) = 0x1008;
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}


pub unsafe fn pass1_1030_e67c(param_1: *mut StructD, mut param_2: u32, mut param_3: u16) -> u16 {
    let mut uVar1: u16;
    let mut paVar2: *mut astruct_67;
    let mut in_stack_0000fea0: u16;
    let mut in_stack_0000ffc4: u16;
    let mut in_stack_0000ffca: u16;
    let mut in_stack_0000ffce: u16;
    let mut uStack8: u16;

    _param_3 = CONCAT22(uStack8, 0x37);
    paVar2 = mixed_1010_20ba(
        param_1,
        _u16_1050_0ed0,
        _param_3,
        in_stack_0000fea0,
        in_stack_0000ffc4,
        in_stack_0000ffca,
        in_stack_0000ffce,
    );
    uVar1 = pass1_1008_aaa8(paVar2, (paVar2 >> 0x10), (param_2 + 0x108));
    if (uVar1 != 0) {
        post_win_msg_1008_a0e4(paVar2, 0x0, 0x0, 0x1, 0x0, uVar1);
    }
    return 0x1;
}


pub unsafe fn pass1_1030_e6c2(mut param_1: u16, param_2: *mut u8, param_3: *mut astruct_406) {
    let mut puVar1: *mut u32;
    let mut puVar2: *mut u32;
    let mut iVar3: i16;
    let mut uVar4: u16;
    let mut in_register_0000000a: u16;
    let mut paVar5: *mut Struct57;
    let mut iVar5: *mut astruct_406;
    let mut puVar6: *mut u32;
    let mut puVar7: *mut u32;
    let mut uVar8: u16;
    let mut puStack10: *mut u16;

    paVar5 = CONCAT22(in_register_0000000a, param_2);
    mem_op_1000_179c(0x10a, paVar5);
    uVar4 = paVar5;
    puStack10 = CONCAT22(uVar4, param_1);
    if ((uVar4 | param_1) != 0) {
        *puStack10 = 0x389a;
        (param_1 + 0x2) = 0x1008;
        uVar8 = (param_3 >> 0x10);
        iVar5 = param_3;
        (param_1 + 0x4) = iVar5.field4_0x4;
        puVar6 = &iVar5.field5_0x8;
        puVar7 = (param_1 + 0x8);
        for iVar3 in 0x40..0 {
            puVar2 = puVar7;
            puVar7 = puVar7 + 1;
            puVar1 = puVar6;
            puVar6 = puVar6 + 1;
            *puVar2 = *puVar1;
        }
        *puStack10 = 0x6ad2;
        (param_1 + 0x2) = 0x1028;
        (param_1 + 0x108) = iVar5.field258_0x108;
        *puStack10 = 0xe78a;
        (param_1 + 0x2) = 0x1030;
    }
    return;
}


pub unsafe fn pass1_1030_e75e(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    param_1.address_offset_field_0x0 = 0x389a;
    (param_1 + 0x2) = 0x1008;
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1030_e7d0() -> u16 {
    return 0x1;
}


pub unsafe fn pass1_1030_e7d6(mut param_1: u16, param_2: *mut u8, mut param_3: u32) {
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
        *puStack10 = 0xe890;
        (param_1 + 0x2) = 0x1030;
    }
    return;
}


pub unsafe fn pass1_1030_e864(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    param_1.address_offset_field_0x0 = 0x389a;
    (param_1 + 0x2) = 0x1008;
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}


pub unsafe fn pass1_1030_e8f8(mut param_1: u16, mut param_2: u16, mut param_3: u32) -> u16 {
    let mut uVar1: u16;
    let mut uVar2: u16;
    let mut iVar3: i16;
    let mut uVar4: u16;
    let mut uVar5: u32;
    let mut pcStack20: *mut c_char;
    let mut uStack6: u32;

    uVar4 = (param_3 >> 0x10);
    iVar3 = param_3;
    if ((iVar3 + 0x108) != 0) {
        pass1_1028_e1ec(_PTR_LOOP_1050_65e2, (iVar3 + 0x10c));
        uStack6 = CONCAT22(param_2, param_1);
        uVar5 = struct_op_1030_73a8(CONCAT22(param_2, param_1), param_1, param_2);
        if ((uVar5 + 0xc) == (iVar3 + 0x110)) {
            pass1_1030_ea50(param_3, uStack6);
        }
        uVar1 = (iVar3 + 0x108);
        uVar2 = (iVar3 + 0x10a);
        pcStack20 = CONCAT22(uVar2, uVar1);
        if ((uVar2 | uVar1) != 0) {
            fn_ptr_1020_ba7e(CONCAT22(uVar2, uVar1));
            fn_ptr_1000_17ce(pcStack20);
        }
        (iVar3 + 0x108) = 0;
    }
    return 0x1;
}


pub unsafe fn pass1_1030_e98e(mut param_1: u16, param_2: *mut u8, param_3: *mut astruct_407) {
    let mut puVar1: *mut u32;
    let mut puVar2: *mut u32;
    let mut iVar3: i16;
    let mut uVar4: u16;
    let mut in_register_0000000a: u16;
    let mut paVar5: *mut Struct57;
    let mut iVar5: *mut astruct_407;
    let mut puVar6: *mut u32;
    let mut puVar7: *mut u32;
    let mut uVar8: u16;
    let mut puStack10: *mut u16;

    paVar5 = CONCAT22(in_register_0000000a, param_2);
    mem_op_1000_179c(0x112, paVar5);
    uVar4 = paVar5;
    puStack10 = CONCAT22(uVar4, param_1);
    if ((uVar4 | param_1) != 0) {
        *puStack10 = 0x389a;
        (param_1 + 0x2) = 0x1008;
        uVar8 = (param_3 >> 0x10);
        iVar5 = param_3;
        (param_1 + 0x4) = iVar5.field4_0x4;
        puVar6 = &iVar5.field5_0x8;
        puVar7 = (param_1 + 0x8);
        for iVar3 in 0x40..0 {
            puVar2 = puVar7;
            puVar7 = puVar7 + 1;
            puVar1 = puVar6;
            puVar6 = puVar6 + 1;
            *puVar2 = *puVar1;
        }
        *puStack10 = 0x6ad2;
        (param_1 + 0x2) = 0x1028;
        (param_1 + 0x108) = iVar5.field258_0x108;
        (param_1 + 0x10c) = iVar5.field259_0x10c;
        (param_1 + 0x110) = iVar5.field260_0x110;
        *puStack10 = 0xeb40;
        (param_1 + 0x2) = 0x1030;
    }
    return;
}

}

pub unsafe fn pass1_1030_eb14(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    param_1.address_offset_field_0x0 = 0x389a;
    (param_1 + 0x2) = 0x1008;
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}


pub unsafe fn pass1_1030_eb50(param_1: *mut astruct_97) -> *mut astruct_97 {
    struct_op_1028_d1dc(param_1, 0x1f3f);
    param_1.offset_0x0 = 0xecb2;
    (param_1 + 0x2) = 0x1030;
    unk_str_op_1000_3d3e(
        (param_1 & 0xffff0000 | (param_1 + 0x8)),
        s_SCMines_1050_59c6,
    );
    return param_1;
}



pub unsafe fn pass1_1030_eb86(mut param_1: u16) -> u16 {
    let mut iVar1: i16;
    let mut ppcVar2: *mut *mut code;
    let mut paVar3: *mut astruct_92;
    let mut uVar4: u16;
    let mut extraout_DX: u16;
    let mut puStack24: *mut u32;
    let mut local_14: *mut astruct_92;

    pass1_1028_dc52(CONCAT22(0x1050, &local_14), 0x1, 0x0, 0x700);
    loop {
        uVar4 = param_1;
        paVar3 = &local_14;
        pass1_1028_e4ec(CONCAT22(0x1050, paVar3));
        puStack24 = CONCAT22(uVar4, paVar3);
        param_1 = uVar4 | paVar3;
        if (param_1 == 0) {
            break;
        }
        if ((paVar3 + 1) == 0x5) {
            iVar1 = &paVar3.field5_0xc;
            if (((0x32 < iVar1) && (!SBORROW2(iVar1, 0x33)))
                && (iVar1 == 0x34
                    || iVar1 -0x33 < 0x1
                    || (0x2b < iVar1 -0x34 && (iVar1 -0x60 < 0x2))))
            {
                ppcVar2 = (*puStack24 + 0x2c);
                (**ppcVar2)(0x1028);
                param_1 = extraout_DX;
            }
        }
    }
    return 0x1;
}


pub unsafe fn pass1_1030_ebf8(mut param_1: u16, mut param_2: u16, mut param_3: u32) {
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
        *puStack10 = 0xecb2;
        (param_1 + 0x2) = 0x1030;
    }
    return;
}


pub unsafe fn pass1_1030_ec86(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    param_1.address_offset_field_0x0 = 0x389a;
    (param_1 + 0x2) = 0x1008;
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}



pub unsafe fn pass1_1030_ecc2(param_1: u8, param_2: *mut astruct_97) -> *mut astruct_97 {
    struct_op_1028_d1dc(param_2, 0xf9f);
    param_2.offset_0x0 = 0xb96;
    (param_2 + 0x2) = &u16_1050_1038;
    unk_str_op_1000_3d3e(
        (param_2 & 0xffff0000 | (param_2 + 0x8)),
        s_SCMorale_1050_59ce,
    );
    return param_2;
}


pub unsafe fn pass1_1030_ecf8(param_1: u8, param_2: *mut Struct57, mut param_3: u32) {
    let mut iVar1: i16;
    let mut puVar2: *mut u32;
    let mut ppcVar3: *mut *mut code;
    let mut uVar4: u16;
    let mut uVar5: u32;
    let mut paVar6: *mut astruct_92;
    let mut iVar7: i16;
    let mut uVar8: u32;
    let mut uVar9: u16;
    let mut uVar10: u16;
    let mut uVar11: u16;
    let mut pSVar13: *mut StructD;
    let mut uVar15: u16;
    let mut bVar16: bool;
    let mut puVar17: *mut u32;
    let mut puVar18: *mut u32;
    let mut in_stack_0000fe40: u16;
    let mut in_stack_0000ff42: u16;
    let mut in_stack_0000ff64: u16;
    let mut in_stack_0000ff6a: u16;
    let mut in_stack_0000ff6e: u16;
    let mut uVar19: u16;
    let mut in_stack_0000ff98: u16;
    let mut uStack64: u32;
    let mut iStack56: i16;
    let mut uStack54: u16;
    let mut paStack38: *mut astruct_685;
    let mut local_22: *mut astruct_92;
    let mut uStack12: u16;
    let mut paVar12: *mut Struct57;
    let mut pSVar14: *mut StructD;

    uStack12 = 0;
    puVar17 = mixed_1010_20ba(
        param_2,
        _u16_1050_0ed0,
        CONCAT22(in_stack_0000ff98, 0x2f),
        in_stack_0000fe40,
        in_stack_0000ff64,
        in_stack_0000ff6a,
        in_stack_0000ff6e,
    );
    uVar8 = param_2 & 0xffff0000 | puVar17 >> 0x10;
    uVar10 = puVar17;
    pass1_1010_ed3e(puVar17);
    uVar9 = uVar8 | uVar10;
    paVar12 = (uVar8 & 0xffff0000 | uVar9);
    if (uVar9 != 0) {
        uStack12 = pass1_1030_2aaa(CONCAT22(uVar8, uVar10));
    }
    if (uStack12 < 0x2) {
        uStack12 = 0;
    }
    puVar17 = mixed_1010_20ba(
        paVar12,
        _u16_1050_0ed0,
        CONCAT22(in_stack_0000ff98, 0x2),
        in_stack_0000fe40,
        in_stack_0000ff64,
        in_stack_0000ff6a,
        in_stack_0000ff6e,
    );
    pSVar13 = (paVar12 & 0xffff0000 | puVar17 >> 0x10);
    if ((0x0 < PTR_LOOP_1050_13ae) && (!SBORROW2(PTR_LOOP_1050_13ae, 1))) {
        if (PTR_LOOP_1050_13ae == &u16_1050_0002 || (PTR_LOOP_1050_13ae -1) < 1) {
            if (0x6 < uStack12) {
                uStack12 -= 0x2;
                // TODO: goto LAB_1030_ed5b;
            }
            bVar16 = SBORROW2(uStack12, 0x4);
            iVar1 = uStack12 - 0x4;
        } else {
            //      if (PTR_LOOP_1050_13ae != (&u16_1050_0002 + 1)) goto LAB_1030_ed5b;
            bVar16 = SBORROW2(uStack12, 0x7);
            iVar1 = uStack12 - 0x7;
        }
        if (bVar16 == (iVar1 < 0x0)) {
            uStack12 -= 1;
        }
    } //
      // LAB_1030_ed5b:
    pass1_1028_dc52(CONCAT13(0x10, CONCAT12(0x50, &local_22)), 0x1, 0x0, 0x400);
    loop {
        paVar6 = &local_22;
        pass1_1028_e4ec(CONCAT22(0x1050, paVar6));
        uVar10 = pSVar13;
        paStack38 = CONCAT22(uVar10, paVar6);
        if ((uVar10 | paVar6) == 0) {
            break;
        }
        uVar9 = &paVar6[0x1b].field6_0x10;
        pSVar13 = (pSVar13 & 0xffff0000 | (paVar6 + 0x1c));
        if (((&paVar6[0x1c].field3_0x4 + 0x2) != 0) && (paVar6[0x1c].field4_0x8 != 0x8000002)) {
            pass1_1030_38b8();
            uVar9 = pSVar13 | uVar9;
            uVar8 = pSVar13 & 0xffff0000;
            pSVar13 = (uVar8 | uVar9);
            if (uVar9 != 0) {
                puVar2 = paVar6.field5_0xc;
                uVar9 = (&paVar6.field5_0xc + 2);
                uVar8 |= uVar9;
                ppcVar3 = (*puVar2 + 0x10);
                puVar18 = puVar2;
                (**ppcVar3)(0x1028, puVar2, uVar9);
                uVar5 = puVar18 & 0xffff | uVar8 << 0x10;
                uStack54 = (&paVar6[0x1].field3_0x4 + 2);
                uVar15 = SUB42(&u16_1050_1038, 0x0);
                pass1_1038_4760(CONCAT22(uVar10, paVar6));
                iVar1 = paVar6[0x1].field6_0x10;
                iStack56 = iVar1 / 0xa;
                iVar7 = (paVar6 + 2);
                if (iVar7 < 0x33) {
                    if (iVar7 < 0x32) {
                        iStack56 += -0x1;
                    }
                } else {
                    uStack54 += 0x1;
                }
                pSVar13 = (uVar8 & 0xffff0000 | iVar1 % 0xa & 0xffff);
                for uStack64 in 0..uVar5 {
                    ppcVar3 = (*puVar2 + 0x4);
                    uVar8 = uVar5;
                    (**ppcVar3)(
                        uVar15,
                        puVar2,
                        (puVar2 >> 0x10),
                        uStack64,
                        (uStack64 >> 0x10),
                    );
                    uVar9 = uVar8;
                    uVar11 = pSVar13 | uVar9;
                    pSVar14 = (pSVar13 & 0xffff0000 | uVar11);
                    if (uVar11 != 0) {
                        uVar15 = 0x1028;
                        pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uVar8 & 0xffff | pSVar13 << 0x10);
                        puVar18 = struct_op_1030_73a8(CONCAT22(pSVar14, uVar9), uVar9, pSVar14);
                        uVar9 = puVar18;
                        uVar11 = (puVar18 >> 0x10);
                        pSVar14 = (pSVar14 & 0xffff0000 | (uVar11 | uVar9));
                        if (((uVar11 | uVar9) != 0) && ((uVar9 + 0x12) == 0x5)) {
                            ppcVar3 = (*puVar18 + 0x48);
                            (**ppcVar3)(0x1028, uVar9, uVar11);
                            if (uVar9 < 0x0) {
                                iStack56 += uVar9;
                            } else {
                                uStack54 += uVar9;
                            }
                        }
                    }
                    pSVar13 = pSVar14;
                }
                iVar1 = (paVar6 + 0x1d);
                uVar19 = (param_3 >> 0x10);
                uVar4 = param_3;
                iVar7 = iVar1;
                pass1_1038_01c0(uVar4, uVar19, paStack38);
                iVar7 -= iVar1;
                iStack56 = (iStack56 - uStack12) - iVar7;
                pass1_1038_008e(pSVar13, uVar4, uVar19, paStack38);
                if (iVar7 < 0x0) {
                    iStack56 += iVar7;
                } else {
                    uStack54 += iVar7;
                }
                if (0x3e8 < uStack54) {
                    uStack54 = 0x3e8;
                }
                if (uStack54 < 0x0) {
                    uStack54 = 0;
                }
                uStack54 += iStack56;
                if (0x3e8 < uStack54) {
                    uStack54 = 0x3e8;
                }
                if (uStack54 < 0x0) {
                    uStack54 = 0;
                }
                pass1_1038_4d0e(paStack38, uStack54);
                if (paVar6.field3_0x4 == 0x4000001) {
                    pass1_1038_08d4(
                        param_1,
                        pSVar13,
                        uVar4,
                        CONCAT22(uStack54, uVar19),
                        paStack38,
                    );
                }
                pass1_1038_095e(
                    pSVar13,
                    uVar4,
                    uVar19,
                    uStack54,
                    paStack38,
                    in_stack_0000ff42,
                );
            }
        }
    }
    return;
}


pub unsafe fn pass1_1038_1220(mut param_1: u32,mut param_2: u32,mut param_3: u32)

{
  let mut ppcVar1: *mut *mut code;
  let mut uVar2: u32;
  let mut uVar3: u16;
  let mut uVar4: u16;
  let mut uVar5: u16;
  let mut puVar6: *mut u8;
  let mut puVar7: *mut u8;
  let mut puVar8: *mut u8;
  let mut uVar10: u16;
  let mut uVar9: u32;
  let mut puVar11: *mut u32;
  let mut uVar12: u8;
  let mut puStack14: *mut u32;
  let mut puStack10: *mut u32;

  uVar10 = (param_1 >> 0x10);
  puVar11 = pass1_1008_c6fa(_u16_1050_06e0,0x4);
  puVar6 = (puVar11 >> 0x10);
  uVar3 = puVar11;
  pass1_1038_4d6e(uVar3,puVar6,param_3,puVar11);
  puStack10 = CONCAT22(puVar6,uVar3);
  ppcVar1 = (*puStack10 + 0x10);
  puVar7 = puVar6;
  uVar4 = uVar3;
  (**ppcVar1)(0x1008,uVar3,puVar6);
  if ((puVar7.is_null() == false) || (uVar4 != 0)) {
    puVar11 = pass1_1008_c6fa(_u16_1050_06e0,0x5);
    puVar8 = (puVar11 >> 0x10);
    uVar4 = puVar11;
    pass1_1038_4d6e(uVar4,puVar8,param_3,puVar11);
    puStack14 = CONCAT22(puVar8,uVar4);
    uVar12 = uVar4;
    uVar2 = *puStack14;
    ppcVar1 = uVar2 + 0x8;
    puVar7 = puVar8;
    uVar5 = uVar4;
    (**ppcVar1)(0x1008,uVar12,puVar8);
    uVar9 = CONCAT22(uVar10,puVar7);
    if (((puVar7.is_null() == false) || (uVar5 != 0)) &&
       (pass1_1038_11b0(uVar5,uVar9,param_2,CONCAT13((puVar6 >> 0x8),CONCAT12(puVar6,uVar3)),
                        CONCAT22(puVar8,uVar4)), uVar5 == 0)) {
      if (puStack14.is_null()) {
        return;
      }
      ppcVar1 = uVar2;
      (**ppcVar1)(0x8,uVar12,puVar8,1);
      return;
    }
    uVar10 = (uVar9 >> 0x10);
    if (puStack14.is_null() == false) {
      ppcVar1 = *puStack14;
      (**ppcVar1)(0x8,uVar12,puVar8,1);
    }
    puVar11 = pass1_1008_c6fa(_u16_1050_06e0,0x6);
    puVar8 = (puVar11 >> 0x10);
    uVar4 = puVar11;
    pass1_1038_4d6e(uVar4,puVar8,param_3,puVar11);
    puStack14 = CONCAT22(puVar8,uVar4);
    ppcVar1 = (*puStack14 + 0x10);
    puVar7 = puVar8;
    uVar5 = uVar4;
    (**ppcVar1)(0x8,uVar4,puVar8);
    if ((puVar7.is_null() == false) || (uVar5 != 0)) {
      pass1_1038_11b0(uVar5,CONCAT22(uVar10,puVar7),param_2,CONCAT22(puVar6,uVar3),
                      CONCAT22(puVar8,uVar4));
    }
    if (puStack14.is_null() == false) {
      ppcVar1 = *puStack14;
      (**ppcVar1)(0x8,uVar4,puVar8,1);
    }
  }
  if (puStack10.is_null() == false) {
    ppcVar1 = *puStack10;
    (**ppcVar1)(0x8,uVar3,puVar6,1);
  }
  return;
}


pub unsafe fn pass1_1038_1940(mut param_1: u32,param_2: *mut u32,mut param_3: u32)

{
  let mut ppcVar1: *mut *mut code;
  let mut uVar2: u16;
  let mut uVar3: u16;
  let mut puVar4: *mut u8;
  let mut extraout_DX: u16;
  let mut puVar5: *mut u32;
  let mut puStack10: *mut u32;

  puVar5 = pass1_1008_c6fa(_u16_1050_06e0,0x3);
  puVar4 = (puVar5 >> 0x10);
  uVar2 = puVar5;
  pass1_1038_4d6e(uVar2,puVar4,param_3,puVar5);
  puStack10 = CONCAT22(puVar4,uVar2);
  ppcVar1 = (*puStack10 + 0x10);
  uVar3 = uVar2;
  (**ppcVar1)(0x1008,uVar2,puVar4);
  if ((extraout_DX | uVar3) != 0) {
    pass1_1038_1482(extraout_DX | uVar3,param_1,param_2,puStack10);
  }
  if (puStack10.is_null() == false) {
    ppcVar1 = *puStack10;
    (**ppcVar1)(0x1008,uVar2,puVar4,1);
  }
  return;
}


pub unsafe fn pass1_1038_19a0(mut param_1: u32,param_2: *mut u32,mut param_3: u32,mut param_4: u16 ,param_5: u8)

{
  let mut ppcVar1: *mut *mut code;
  let mut uVar2: u32;
  let mut uVar3: u16;
  let mut uVar4: u16;
  let mut puVar5: *mut u8;
  let mut extraout_DX: u16;
  let mut puVar6: *mut u32;
  let mut puStack10: *mut u32;

  puVar6 = pass1_1008_c6fa(_u16_1050_06e0,0x2);
  puVar5 = (puVar6 >> 0x10);
  uVar3 = puVar6;
  pass1_1038_4d6e(uVar3,puVar5,param_3,puVar6);
  puStack10 = CONCAT22(puVar5,uVar3);
  uVar2 = *puStack10;
  ppcVar1 = uVar2 + 0x8;
  uVar4 = uVar3;
  (**ppcVar1)(0x1008,uVar3,puVar5);
  if ((extraout_DX | uVar4) == 0) {
    vsprintf_op_1030_840a(0x0,s_mineToSmelter__no_mines_1050_59df);
    if (puStack10.is_null() == false) {
      ppcVar1 = uVar2;
      (**ppcVar1)(0x1030,uVar3,puVar5,1);
      return;
    }
  }
  else {
    pass1_1038_16f2(extraout_DX | uVar4,param_1,puStack10,param_2);
    if (puStack10.is_null() == false) {
      ppcVar1 = *puStack10;
      (**ppcVar1)(0x1008,uVar3,puVar5,1);
    }
  }
  return;
}

pub unsafe fn pass1_1038_1c02(param_1: *mut StructD,param_2: u8) -> *mut StructD

{
  param_1.address_offset_field_0x0 = 0x389a;
  (param_1 + 0x2) = 0x1008;
  if ((param_2 & 1) != 0) {
    fn_ptr_1000_17ce(param_1);
  }
  return param_1;
}


pub unsafe fn pass1_1038_1c3e(mut param_1: u32,mut param_2: u32)

{
  let mut uVar1: u16;
  let mut puVar2: *mut u32;
  let mut ppcVar3: *mut *mut code;
  let mut uVar4: u32;
  let mut uVar5: u16;
  let mut iVar6: i16;
  let mut BVar7: bool;
  let mut puVar8: *mut u32;
  let mut extraout_DX: u16;
  let mut extraout_DX_00: u16;
  let mut uVar9: u16;
  let mut uVar10: u16;
  let mut unaff_CS: u16;
  let mut uVar11: u32;
  let mut uVar12: u16;
  let mut uVar13: u16;
  let mut uVar14: u16;
  let mut paStack26: *mut astruct_419;
  let mut uStack14: u32;

  uVar10 = (param_2 >> 0x10);
  puVar2 = (param_2 + 0xc);
  uVar10 = (param_2 + 0xe);
  ppcVar3 = (*puVar2 + 0x10);
  puVar8 = puVar2;
  uVar14 = puVar2;
  (**ppcVar3)();
  uVar4 = puVar8 & 0xffff | extraout_DX << 0x10;
  uStack14 = 0;
  loop {
    if (uVar4 <= uStack14) {
      return;
    }
    ppcVar3 = (*puVar2 + 0x4);
    uVar11 = uVar4;
    (**ppcVar3)(unaff_CS,puVar2,(puVar2 >> 0x10),uStack14,uVar14,uVar10);
    uVar5 = uVar11;
    uVar9 = extraout_DX_00 | uVar5;
    if (uVar9 != 0) {
      unaff_CS = 0x1028;
      pass1_1028_e1ec(_PTR_LOOP_1050_65e2,uVar11 & 0xffff | extraout_DX_00 << 0x10);
      paStack26 = CONCAT22(uVar9,uVar5);
      iVar6 = (uVar5 + 0x34);
      if ((iVar6 != 0) && ((uVar5 + 0x36) != 0)) {
        uVar12 = param_1;
        uVar13 = (param_1 >> 0x10);
        pass1_1038_201a(iVar6,uVar9,uVar12,uVar13,CONCAT22(uVar9,uVar5));
        if (iVar6 == 0) {
          uVar11 = struct_op_1030_73a8(paStack26,0x0,uVar9);
          uVar1 = (uVar11 + 0xc);
          unaff_CS = 0x1008;
          BVar7 = pass1_1008_c6ae(_u16_1050_06e0,uVar1,1);
          if (BVar7 == 0) {
            unaff_CS = 0x1008;
            BVar7 = pass1_1008_c6ae(_u16_1050_06e0,uVar1,0x2);
            if (BVar7 == 0) {
              BVar7 = pass1_1008_c6ae(_u16_1050_06e0,uVar1,0x5);
              if (BVar7 == 0) {
                unaff_CS = 0x1008;
                BVar7 = pass1_1008_c6ae(_u16_1050_06e0,uVar1,0x6);
//                if (BVar7 == 0) goto LAB_1038_1c76;
              }
              unaff_CS = 0x1008;
              pass1_1038_2306(uVar12,uVar13,paStack26);
            }
            else {
              pass1_1038_26ee(uVar12,uVar13,paStack26);
            }
          }
          else {
            pass1_1038_24e8(uVar12,uVar13,paStack26);
          }
        }
      }
    }//
// LAB_1038_1c76:
    uStack14 += 0x1;
  }
}
