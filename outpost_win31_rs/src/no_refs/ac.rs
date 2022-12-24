use crate::unk::block_1028_6000::pass1_1028_6186;

pub unsafe fn pass1_1028_4af6(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    pass1_1028_b418(&param_1.address_offset_field_0x0);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1028_4bd0(mut param_1: u32) -> u16 {
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    if (((param_1 + 0x12) != 0x6) && ((param_1 + 0x12) != 0x5)) {
        return 0x0;
    }
    return 0x1;
}

pub unsafe fn pass1_1028_4bf2(mut param_1: i16, param_2: *mut astruct_15, mut param_3: u32) {
    let mut ppcVar1: *mut *mut code;
    let mut puVar2: *mut u32;
    let mut uVar3: u16;
    let mut extraout_DX: u16;
    let mut uVar4: u16;
    let mut uVar5: u32;
    let mut puVar6: *mut u32;
    let mut uStack54: u32;
    let mut local_2c: [u8; 0x6] = [0; 0x6];
    let mut puStack38: *mut u32;
    let mut uStack34: u32;
    let mut puStack26: *mut u32;
    let mut uStack24: u32;
    let mut local_14: u32;
    let mut iStack16: i16;
    let mut iStack14: i16;
    let mut local_c: u32;
    let mut iStack8: i16;
    let mut iStack6: i16;
    let mut uStack4: u16;

    pass1_1028_b58e(param_2);
    local_14 = (param_1 + 0xc);
    iStack8 = (param_1 + 0x10);
    puStack26 = &local_c;
    uStack34 = CONCAT22(uStack34, &local_14);
    iStack16 = iStack8 + 1;
    puVar6 = CONCAT22(0x1050, local_2c);
    iStack14 = iStack16;
    local_c = local_14;
    iStack6 = param_1;
    uStack4 = extraout_DX;
    uVar5 = pass1_1028_bb24(param_2);
    uVar4 = (uVar5 >> 0x10);
    puVar2 = &local_14;
    pass1_1030_64ce(
        puVar2,
        uVar4,
        _PTR_LOOP_1050_5740,
        CONCAT22(0x1050, puVar2),
        uVar5 & 0xffff | uVar4 << 0x10,
        puVar6,
    );
    uStack34 = *puVar2;
    uVar4 = (puVar2 + 2);
    uStack54._3_1_ = (uStack34 >> 0x18);
    uVar3 = uStack54._3_1_;
    uStack24 = uStack34;
    if (uStack54._3_1_ != 0) {
        pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uStack34 & 0xffff | uVar4 << 0x10);
        uStack54 = CONCAT22(uVar4, uVar3);
        uVar3 = pass1_1030_6fa0(CONCAT22(uVar4, uVar3));
        if ((uVar3 == 0x62) || (uVar3 == 0x63)) {
            puStack38 = struct_op_1030_73a8(uStack54, uVar3, uVar4);
            uVar3 = puStack38;
            ppcVar1 = (*puStack38 + 0x58);
            (**ppcVar1)();
        }
    }
    pass1_1028_b46e(uVar3, param_2, param_3);
    return;
}

pub unsafe fn pass1_1028_4cd6(mut param_1: i16, param_2: *mut astruct_15) {
    let mut ppcVar1: *mut *mut code;
    let mut puVar2: *mut u32;
    let mut uVar3: u16;
    let mut uVar4: u16;
    let mut extraout_DX: u16;
    let mut uVar5: u16;
    let mut uVar6: u32;
    let mut puVar7: *mut u32;
    let mut uStack54: u32;
    let mut local_2c: [u8; 0x6] = [0; 0x6];
    let mut puStack38: *mut u32;
    let mut uStack34: u32;
    let mut puStack26: *mut u32;
    let mut uStack24: u32;
    let mut local_14: u32;
    let mut iStack16: i16;
    let mut iStack14: i16;
    let mut local_c: u32;
    let mut iStack8: i16;
    let mut iStack6: i16;
    let mut uStack4: u16;

    pass1_1028_b514(param_2);
    pass1_1028_b58e(param_2);
    local_14 = (param_1 + 0xc);
    iStack8 = (param_1 + 0x10);
    puStack26 = &local_c;
    uStack34 = CONCAT22(uStack34, &local_14);
    iStack16 = iStack8 + 1;
    puVar7 = CONCAT22(0x1050, local_2c);
    iStack14 = iStack16;
    local_c = local_14;
    iStack6 = param_1;
    uStack4 = extraout_DX;
    uVar6 = pass1_1028_bb24(param_2);
    uVar5 = (uVar6 >> 0x10);
    puVar2 = &local_14;
    pass1_1030_64ce(
        puVar2,
        uVar5,
        _PTR_LOOP_1050_5740,
        CONCAT22(0x1050, puVar2),
        uVar6 & 0xffff | uVar5 << 0x10,
        puVar7,
    );
    uStack34 = *puVar2;
    uVar5 = (puVar2 + 2);
    uStack54._3_1_ = (uStack34 >> 0x18);
    uVar3 = uStack54._3_1_;
    if (uStack54._3_1_ != 0) {
        uStack24 = uStack34;
        pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uStack34 & 0xffff | uVar5 << 0x10);
        uStack54 = CONCAT22(uVar5, uVar3);
        uVar4 = pass1_1030_6fa0(CONCAT22(uVar5, uVar3));
        if ((uVar4 == 0x62) || (uVar4 == 0x63)) {
            puStack38 = struct_op_1030_73a8(uStack54, uVar4, uVar5);
            ppcVar1 = (*puStack38 + 0x24);
            (**ppcVar1)();
        }
    }
    return;
}

pub unsafe fn pass1_1028_4db2(
    param_1: u8,
    param_2: *mut u8,
    param_3: *mut astruct_15,
    mut param_4: i16,
) {
    let mut BVar1: bool;
    let mut piVar2: *mut i16;
    let mut uVar3: u16;
    let mut in_register_0000000a: u16;
    let mut paVar4: *mut Struct57;
    let mut unaff_SI: u16;
    let mut puVar5: *mut u32;
    let mut in_stack_0000fd42: u16;
    let mut in_stack_0000fe66: u16;
    let mut in_stack_0000fe6c: u16;
    let mut in_stack_0000fe70: u16;
    let mut piVar6: *mut i16;
    let mut uVar7: u16;
    let mut puVar8: *mut u16;
    let mut uVar9: u16;
    let mut local_14e: [u8; 0x124] = [0; 0x124];
    let mut uStack42: u32;
    let mut uStack38: u32;
    let mut local_22: i16;
    let mut local_20: [u8; 0x2] = [0; 0x2];
    let mut local_1e: [u8; 0x2] = [0; 0x2];
    let mut local_1c: u32;
    let mut iStack24: i16;
    let mut uStack22: u32;
    let mut piStack18: *mut i16;
    let mut uStack16: u16;
    let mut local_e: i16;
    let mut local_c: u16;
    let mut uStack10: u32;
    let mut puStack6: *mut u32;

    paVar4 = CONCAT22(in_register_0000000a, param_2);
    BVar1 = pass1_1008_c6ae(_u16_1050_06e0, (param_3 + 0xc), 0x29);
    if (BVar1 != 0) {
        pass1_1028_bd38(paVar4, param_3);
        if ((param_4 == 0) && ((param_3 + 0xc) == 0x13)) {
            puVar5 = mixed_1010_20ba(
                paVar4,
                _u16_1050_0ed0,
                CONCAT22(unaff_SI, 0x8),
                in_stack_0000fd42,
                in_stack_0000fe66,
                in_stack_0000fe6c,
                in_stack_0000fe70,
            );
            paVar4 = (paVar4 & 0xffff0000 | puVar5 >> 0x10);
            pass1_1010_988c(puVar5, (param_3 + 0xc));
        }
        puStack6 = mixed_1010_20ba(
            paVar4,
            _u16_1050_0ed0,
            CONCAT22(unaff_SI, 0x2f),
            in_stack_0000fd42,
            in_stack_0000fe66,
            in_stack_0000fe6c,
            in_stack_0000fe70,
        );
        uVar3 = (puStack6 >> 0x10);
        uStack10 = (puStack6 + 0x20);
        puVar8 = &local_c;
        uVar9 = SUB42(0x1050, 0x0);
        piVar2 = &local_e;
        uVar7 = SUB42(0x1050, 0x0);
        piVar6 = piVar2;
        pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uStack10);
        piStack18 = piVar2;
        uStack16 = uVar3;
        pass1_1030_5b1c(
            CONCAT22(uVar3, piVar2),
            CONCAT22(uVar7, piVar6),
            CONCAT22(uVar9, puVar8),
        );
        pass1_1028_b58e(param_3);
        uStack22 = CONCAT22(uVar3, piVar2);
        local_1c = (piVar2 + 0x6);
        iStack24 = piVar2[0x8];
        pass1_1028_c8ee(param_3, 0x1, CONCAT22(0x1050, &local_1c));
        pass1_1008_3eb4(
            CONCAT22(0x1050, &local_1c),
            CONCAT22(0x1050, &local_22),
            CONCAT22(0x1050, local_20),
            CONCAT22(0x1050, local_1e),
        );
        if (local_e < local_22) {
            pass1_1030_5b3e(CONCAT22(uStack16, piStack18), local_22, local_c);
            pass1_1030_5b1c(
                CONCAT22(uStack16, piStack18),
                CONCAT22(0x1050, &local_e),
                CONCAT22(0x1050, &local_c),
            );
        }
        uStack38 = (uStack22 + 0x2e);
        uStack42 = (uStack38 + 0x4);
        struct_op_1028_87f0(
            CONCAT22(0x1050, local_14e),
            0x0,
            0x0,
            0x62,
            &local_1c,
            0x1050,
            uStack42,
            uStack10,
        );
        fn_ptr_1030_835a(_u16_1050_5748, CONCAT22(0x1050, local_14e));
        pass1_1028_ccd0(param_3, CONCAT22(0x1050, &local_1c));
    }
    return;
}

pub unsafe fn pass1_1028_4f30(param_1: *mut astruct_15, mut param_2: i16) {
    let mut paVar1: *mut astruct_397;
    let mut uVar2: u16;

    paVar1 = pass1_1028_b58e(param_1);
    if (param_2 == 0) {
        uVar2 = 0;
    } else {
        uVar2 = 0x3e8;
    }
    pass1_1030_7d1c(paVar1, (paVar1 >> 0x10), paVar1, uVar2, 0x230000);
    return;
}

pub unsafe fn pass1_1028_504a(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    pass1_1028_b418(&param_1.address_offset_field_0x0);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn FUN_1028_5124() {
    return;
}

pub unsafe fn pass1_1028_5128(param_1: u8, mut param_2: u16, param_3: *mut astruct_15) {
    let mut piVar1: *mut i16;
    let mut uVar2: u16;
    let mut in_register_0000000a: u16;
    let mut paVar3: *mut Struct57;
    let mut unaff_SI: u16;
    let mut in_stack_0000fd42: u16;
    let mut in_stack_0000fe66: u16;
    let mut in_stack_0000fe6c: u16;
    let mut in_stack_0000fe70: u16;
    let mut piVar4: *mut i16;
    let mut uVar5: u16;
    let mut puVar6: *mut u16;
    let mut uVar7: u16;
    let mut local_14e: [u8; 0x124] = [0; 0x124];
    let mut uStack42: u32;
    let mut uStack38: u32;
    let mut local_22: i16;
    let mut local_20: [u8; 0x2] = [0; 0x2];
    let mut local_1e: [u8; 0x2] = [0; 0x2];
    let mut local_1c: u32;
    let mut iStack24: i16;
    let mut uStack22: u32;
    let mut piStack18: *mut i16;
    let mut uStack16: u16;
    let mut local_e: i16;
    let mut local_c: u16;
    let mut uStack10: u32;
    let mut puStack6: *mut u32;

    paVar3 = CONCAT22(in_register_0000000a, param_2);
    pass1_1028_bd38(param_2, param_3);
    puStack6 = mixed_1010_20ba(
        paVar3,
        _u16_1050_0ed0,
        CONCAT22(unaff_SI, 0x2f),
        in_stack_0000fd42,
        in_stack_0000fe66,
        in_stack_0000fe6c,
        in_stack_0000fe70,
    );
    uVar2 = (puStack6 >> 0x10);
    uStack10 = (puStack6 + 0x20);
    puVar6 = &local_c;
    uVar7 = SUB42(0x1050, 0x0);
    piVar1 = &local_e;
    uVar5 = SUB42(0x1050, 0x0);
    piVar4 = piVar1;
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uStack10);
    piStack18 = piVar1;
    uStack16 = uVar2;
    pass1_1030_5b1c(
        CONCAT22(uVar2, piVar1),
        CONCAT22(uVar5, piVar4),
        CONCAT22(uVar7, puVar6),
    );
    pass1_1028_b58e(param_3);
    uStack22 = CONCAT22(uVar2, piVar1);
    local_1c = (piVar1 + 0x6);
    iStack24 = piVar1[0x8];
    pass1_1028_c8ee(param_3, 0x1, CONCAT22(0x1050, &local_1c));
    pass1_1008_3eb4(
        CONCAT22(0x1050, &local_1c),
        CONCAT22(0x1050, &local_22),
        CONCAT22(0x1050, local_20),
        CONCAT22(0x1050, local_1e),
    );
    if (local_e < local_22) {
        pass1_1030_5b3e(CONCAT22(uStack16, piStack18), local_22, local_c);
        pass1_1030_5b1c(
            CONCAT22(uStack16, piStack18),
            CONCAT22(0x1050, &local_e),
            CONCAT22(0x1050, &local_c),
        );
    }
    uStack38 = (uStack22 + 0x2e);
    uStack42 = (uStack38 + 0x4);
    struct_op_1028_87f0(
        CONCAT22(0x1050, local_14e),
        0x0,
        0x0,
        0x6f,
        &local_1c,
        0x1050,
        uStack42,
        uStack10,
    );
    fn_ptr_1030_835a(_u16_1050_5748, CONCAT22(0x1050, local_14e));
    pass1_1028_ccd0(param_3, CONCAT22(0x1050, &local_1c));
    return;
}

pub unsafe fn pass1_1028_525a(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    pass1_1028_b418(&param_1.address_offset_field_0x0);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn FUN_1028_5334() {
    return;
}

pub unsafe fn FUN_1028_5338(mut param_1: u16, param_2: *mut StructD, param_3: u8) -> *mut StructD {
    pass1_1028_b418(&param_2.address_offset_field_0x0);
    if ((param_3 & 1) != 0) {
        fn_ptr_1000_17ce(param_2);
    }
    return param_2;
}

pub unsafe fn pass1_1028_533c() -> u32 {
    let mut unaff_BP: i16;

    pass1_1028_b418((unaff_BP + 0x6));
    if ((*(unaff_BP + 0xa) & 1) != 0) {
        fn_ptr_1000_17ce(*(unaff_BP + 0x6));
    }
    return CONCAT22((unaff_BP + 0x8), (unaff_BP + 0x6));
}

pub unsafe fn pass1_1028_5412(param_1: *mut astruct_15) {
    let mut ppcVar1: *mut *mut code;
    let mut uVar2: u16;
    let mut uVar3: u32;
    let mut iVar4: i16;

    uVar2 = (param_1 >> 0x10);
    if ((param_1 + 0x12) != 0x6) {
        return;
    }
    uVar3 = pass1_1028_b4f2(param_1);
    if ((uVar3 + 0x200) != 0x8000002) {
        if ((param_1 + 0x1c) == 0x8000002) {
            iVar4 = 0x6;
            // TODO: goto code_r0x1028548e;
        }
        ppcVar1 = (param_1 + 0x64);
        iVar4 = (**ppcVar1)();
        if (iVar4 == 0) {
            return;
        }
        pass1_1028_c0f0(iVar4, param_1, 1);
        if (iVar4 == 0) {
            iVar4 = 0x6;
            // TODO: goto code_r0x1028548e;
        }
        pass1_1028_c952(param_1);
        pass1_1028_c00a(iVar4, param_1, 1);
    }
    iVar4 = 0x5;
    // code_r0x1028548e:
    pass1_1028_bdac(param_1, iVar4);
    return;
}

pub unsafe fn pass1_1028_5496(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    pass1_1028_b418(&param_1.address_offset_field_0x0);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1028_5570(param_1: *mut astruct_15) {
    let mut in_EDX: *mut Struct57;
    let mut uVar1: u16;
    let mut uVar2: u16;
    let mut uVar3: u16;
    let mut iVar4: i16;

    pass1_1028_0550(param_1);
    uVar1 = (param_1 >> 0x10);
    if ((param_1 + 0x12) == 0x5) {
        uVar3 = 0;
        iVar4 = 0x4;
        uVar2 = 0x1;
        uVar1 = pass1_1028_b58e((param_1 & 0xffff | uVar1 << 0x10));
        pass1_1030_7c50(
            uVar1,
            in_EDX,
            CONCAT22(in_EDX, uVar1),
            CONCAT22(uVar3, uVar2),
            iVar4,
        );
    }
    return;
}

pub unsafe fn pass1_1028_55a2(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    pass1_1028_0138(&param_1.address_offset_field_0x0);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn FUN_1028_567c() -> u16 {
    return 0x0;
}

pub unsafe fn FUN_1028_5682() {
    return;
}

pub unsafe fn FUN_1028_5686(mut param_1: u16, param_2: *mut StructD, param_3: u8) -> *mut StructD {
    pass1_1028_b418(&param_2.address_offset_field_0x0);
    if ((param_3 & 1) != 0) {
        fn_ptr_1000_17ce(param_2);
    }
    return param_2;
}

pub unsafe fn FUN_1028_5714() {
    return;
}

pub unsafe fn FUN_1028_5718(mut param_1: u16, param_2: *mut StructD, param_3: u8) -> *mut StructD {
    pass1_1028_b418(&param_2.address_offset_field_0x0);
    if ((param_3 & 1) != 0) {
        fn_ptr_1000_17ce(param_2);
    }
    return param_2;
}

pub unsafe fn FUN_1028_57f2() {
    return;
}

pub unsafe fn FUN_1028_57f6(mut param_1: u16, param_2: *mut StructD, param_3: u8) -> *mut StructD {
    pass1_1028_b418(&param_2.address_offset_field_0x0);
    if ((param_3 & 1) != 0) {
        fn_ptr_1000_17ce(param_2);
    }
    return param_2;
}

pub unsafe fn pass1_1028_57fa() -> u32 {
    let mut unaff_BP: i16;

    pass1_1028_b418((unaff_BP + 0x6));
    if ((*(unaff_BP + 0xa) & 1) != 0) {
        fn_ptr_1000_17ce(*(unaff_BP + 0x6));
    }
    return CONCAT22((unaff_BP + 0x8), (unaff_BP + 0x6));
}

pub unsafe fn FUN_1028_58d0() {
    return;
}

pub unsafe fn FUN_1028_58d4() {
    return;
}

pub unsafe fn FUN_1028_58d8(mut param_1: u16, param_2: *mut StructD, param_3: u8) -> *mut StructD {
    pass1_1028_b418(&param_2.address_offset_field_0x0);
    if ((param_3 & 1) != 0) {
        fn_ptr_1000_17ce(param_2);
    }
    return param_2;
}


pub unsafe fn pass1_1028_58dc() -> u32 {
    let mut unaff_BP: i16;

    pass1_1028_b418((unaff_BP + 0x6));
    if ((*(unaff_BP + 0xa) & 1) != 0) {
        fn_ptr_1000_17ce(*(unaff_BP + 0x6));
    }
    return CONCAT22((unaff_BP + 0x8), (unaff_BP + 0x6));
}

pub unsafe fn FUN_1028_59b2() {
    return;
}

pub unsafe fn FUN_1028_59b6() {
    return;
}


pub unsafe fn FUN_1028_59ba(mut param_1: u16, param_2: *mut StructD, param_3: u8) -> *mut StructD {
    pass1_1028_b418(&param_2.address_offset_field_0x0);
    if ((param_3 & 1) != 0) {
        fn_ptr_1000_17ce(param_2);
    }
    return param_2;
}


pub unsafe fn pass1_1028_59be() -> u32 {
    let mut unaff_BP: i16;

    pass1_1028_b418((unaff_BP + 0x6));
    if ((*(unaff_BP + 0xa) & 1) != 0) {
        fn_ptr_1000_17ce(*(unaff_BP + 0x6));
    }
    return CONCAT22((unaff_BP + 0x8), (unaff_BP + 0x6));
}

pub unsafe fn pass1_1028_5b42(param_1: *mut astruct_15) {
    let mut ppcVar1: *mut *mut code;
    let mut uVar2: u16;
    let mut uVar3: u32;
    let mut iVar4: i16;

    uVar2 = (param_1 >> 0x10);
    if ((param_1 + 0x12) != 0x6) {
        return;
    }
    uVar3 = pass1_1028_b4f2(param_1);
    if ((uVar3 + 0x200) != 0x8000002) {
        if ((param_1 + 0x1c) == 0x8000002) {
            iVar4 = 0x6;
            // TODO: goto code_r0x10285bbe;
        }
        ppcVar1 = (param_1 + 0x64);
        iVar4 = (**ppcVar1)();
        if (iVar4 == 0) {
            return;
        }
        pass1_1028_c0f0(iVar4, param_1, 0x2);
        if (iVar4 == 0) {
            iVar4 = 0x6;
            // TODO: goto code_r0x10285bbe;
        }
        pass1_1028_c952(param_1);
        pass1_1028_c00a(iVar4, param_1, 0x2);
    }
    iVar4 = 0x5;
    // code_r0x10285bbe:
    pass1_1028_bdac(param_1, iVar4);
    return;
}

pub unsafe fn FUN_1028_5d0e(mut param_1: i16, mut param_2: u16, param_3: *mut astruct_15) {
    let mut uVar1: u32;
    let mut extraout_DX: u16;
    let mut local_11c: [u8; 0x10e] = [0; 0x10e];

    pass1_1028_b58e(param_3);
    uVar1 = (param_1 + 0x2e);
    pass1_1028_68de(CONCAT22(0x1050, local_11c), 0x1, (uVar1 + 0x4));
    fn_ptr_1030_835a(_u16_1050_5748, CONCAT22(0x1050, local_11c));
    return;
}

pub unsafe fn pass1_1028_5d68(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    pass1_1028_b418(&param_1.address_offset_field_0x0);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn FUN_1028_5e42() {
    return;
}

pub unsafe fn FUN_1028_5e46() {
    return;
}

pub unsafe fn FUN_1028_5e4a(mut param_1: u16, param_2: *mut StructD, param_3: u8) -> *mut StructD {
    pass1_1028_b418(&param_2.address_offset_field_0x0);
    if ((param_3 & 1) != 0) {
        fn_ptr_1000_17ce(param_2);
    }
    return param_2;
}

pub unsafe fn FUN_1028_5f30(mut param_1: i16, mut param_2: u16, param_3: *mut astruct_15) {
    let mut BVar1: bool;
    let mut uVar3: u32;
    let mut extraout_DX: u16;
    let mut uVar2: u16;
    let mut pstruct15_5: *mut astruct_15;
    let mut uVar5: u16;
    let mut ppuVar1: *mut *mut u32 = null_mut();
    let mut iVar3: i16;

    pass1_1028_be9e(param_3);
    uVar5 = (param_3 >> 0x10);
    pstruct15_5 = param_3;
    if (pstruct15_5.field15_0x12 == 0x5) {
        pstruct15_5.field24_0x20 = (s_New_failed_in_Op__Op__DialogCtr_1050_0053 + 0x11);
        pass1_1028_b58e(param_3);
        uVar3 = (param_1 + 0x2e);
        iVar3 = 0x61;
        uVar2 = extraout_DX;
        pass1_1038_3fb0(uVar3);
        BVar1 = pass1_1030_25b2(uVar3 & 0xffff | uVar2 << 0x10, iVar3);
        if (BVar1 != 0) {
            ppuVar1 = &pstruct15_5.field24_0x20;
            *ppuVar1 = *ppuVar1 + 0x19;
        }
    }
    return;
}


pub unsafe fn write_to_file_1028_5f82(param_1: *mut astruct_731, param_2: *mut u8) -> BOOL16 {
    let mut BVar1: bool;
    let mut in_stack_0000ffde: HFILE16;
    let mut local_c: [u16; 0x5] = [0; 0x5];

    BVar1 = write_to_file_1028_b5ec(param_1, param_2);
    if (BVar1 != 0) {
        local_c[0] = (param_1 + 0x20);
        BVar1 = write_to_file_1008_7e1c(param_2, CONCAT22(0x1050, local_c), 0x2, in_stack_0000ffde);
        if (BVar1 == 0) {
            u16_1050_0310 = 0x6d0;
            return BVar1;
        }
        BVar1 = 0x1;
    }
    return BVar1;
}

pub unsafe fn FUN_1028_5fc8(mut param_1: u16, param_2: *mut astruct_373, param_3: *mut HFILE16) {
    let mut in_AX: i16;
    let mut BVar1: bool;
    let mut in_DX: *mut u8;

    file_1028_b81a(in_AX, in_DX, param_2, param_3);
    if ((in_AX != 0) && (
        BVar1 = read_file_1008_7dee(param_3, (param_2 & 0xffff0000 | (param_2 + 0x20)), 0x2),
        BVar1 == 0,
    )) {
        u16_1050_0310 = 0x6d2;
        return;
    }
    return;
}

pub unsafe fn pass1_1028_6008(param_1: u32) {
    let mut piVar1: *mut i16;
    let mut iVar2: i16;
    let mut uVar3: u16;

    pass1_1028_be2a(param_1);
    uVar3 = (param_1 >> 0x10);
    iVar2 = param_1;
    if (((iVar2 + 0x12) == 0x5) && (0x0 < (iVar2 + 0x20))) {
        piVar1 = (iVar2 + 0x20);
        *piVar1 = *piVar1 - 0x1;
    }
    return;
}

pub unsafe fn pass1_1028_602e(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    pass1_1028_b418(&param_1.address_offset_field_0x0);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1028_6850(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    pass1_1028_6186(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1028_6926(mut param_1: u16, mut param_2: u16, mut param_3: u32) {
    let mut uVar1: u16;
    let mut ppcVar2: *mut *mut code;
    let mut uVar3: u32;
    let mut uVar4: u16;
    let mut uVar5: u16;
    let mut puVar6: *mut u8;
    let mut puVar7: *mut u8;
    let mut in_register_0000000a: u16;
    let mut paVar8: *mut Struct57;
    let mut uVar9: u16;
    let mut uVar10: u16;
    let mut puVar11: *mut u32;
    let mut puStack14: *mut u32;

    uVar9 = (param_3 >> 0x10);
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2, (param_3 + 0x108));
    puVar11 = pass1_1008_c6fa(_u16_1050_06e0, 0xa);
    puVar6 = (puVar11 >> 0x10);
    uVar4 = puVar11;
    uVar10 = SUB42(&u16_1050_1038, 0x0);
    pass1_1038_4d6e(uVar4, puVar6, CONCAT22(param_2, param_1), puVar11);
    puStack14 = CONCAT22(puVar6, uVar4);
    uVar3 = *puStack14;
    ppcVar2 = (uVar3 + 0x10);
    puVar7 = puVar6;
    uVar5 = uVar4;
    (**ppcVar2)(&u16_1050_1038, uVar4, puVar6);
    paVar8 = CONCAT22(in_register_0000000a, puVar7 | uVar5);
    if ((puVar7 | uVar5) != 0) {
        ppcVar2 = (uVar3 + 0x4);
        (**ppcVar2)(0x38, uVar4, puVar6, 0x0, 0x0);
        pass1_1028_e1ec(_PTR_LOOP_1050_65e2, CONCAT22(paVar8, uVar5));
        uVar1 = (param_3 + 0x10c);
        uVar10 = 0x1030;
        pass1_1030_7ddc(
            uVar1,
            paVar8,
            CONCAT22(paVar8, uVar5),
            CONCAT13((uVar1 >> 0xf), uVar1),
            0x1f,
        );
    }
    if (puStack14.is_null() == false) {
        ppcVar2 = *puStack14;
        (**ppcVar2)(uVar10, uVar4, puVar6, 1);
    }
    return;
}

pub unsafe fn pass1_1028_69cc(
    param_1: *mut astruct_317,
    param_2: *mut Struct57,
    param_3: *mut astruct_316,
) {
    let mut puVar1: *mut u32;
    let mut puVar2: *mut u32;
    let mut iVar3: i16;
    let mut uVar4: u16;
    let mut iVar5: *mut astruct_316;
    let mut puVar5: *mut u32;
    let mut puVar6: *mut u32;
    let mut uVar7: u16;
    let mut puStack10: *mut u16;

    mem_op_1000_179c(0x10e, param_2);
    uVar4 = param_2;
    puStack10 = CONCAT22(uVar4, param_1);
    if ((uVar4 | param_1) != 0) {
        *puStack10 = 0x389a;
        param_1.field2_0x2 = 0x1008;
        uVar7 = (param_3 >> 0x10);
        iVar5 = param_3;
        param_1.field3_0x4 = iVar5.field4_0x4;
        puVar5 = &iVar5.field5_0x8;
        puVar6 = &param_1.field4_0x8;
        // for (iVar3 = 0x40; iVar3 != 0; iVar3 += -1)
        for iVar3 in 0x40..0 {
            puVar2 = puVar6;
            puVar6 = puVar6 + 1;
            puVar1 = puVar5;
            puVar5 = puVar5 + 1;
            *puVar2 = *puVar1;
        }
        *puStack10 = 0x6ad2;
        param_1.field2_0x2 = 0x1028;
        param_1.field257_0x108 = iVar5.field258_0x108;
        param_1.field258_0x10c = iVar5.field259_0x10c;
        *puStack10 = 0x6ae2;
        param_1.field2_0x2 = 0x1028;
    }
    return;
}

pub unsafe fn pass1_1028_6a7a(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    param_1.address_offset_field_0x0 = 0x389a;
    (param_1 + 0x2) = 0x1008;
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1028_6aa6(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    param_1.address_offset_field_0x0 = 0x389a;
    (param_1 + 0x2) = 0x1008;
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1028_6e24(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    param_1.address_offset_field_0x0 = 0x389a;
    (param_1 + 0x2) = 0x1008;
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}
