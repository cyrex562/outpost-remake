













pub unsafe fn pass1_1010_e8f6(
    mut param_1: u16,
    mut param_2: u16,
    mut param_3: u16,
    mut param_4: u16,
    mut param_5: u32,
) {
    let mut uVar2: u16;
    let mut BVar3: bool;
    let mut uVar6: u16;
    let mut uVar5: u32;
    let mut paVar4: *mut astruct_15;
    let mut uVar7: u32;
    let mut uVar1: u16;

    uVar5 = struct_op_1030_73a8(param_5, param_1, param_2);
    uVar1 = (uVar5 + 0xc);
    BVar3 = pass1_1008_c6ae(_u16_1050_06e0, uVar1, 0x13);
    if (BVar3 == 0) {
        BVar3 = pass1_1008_c6ae(_u16_1050_06e0, uVar1, 0x14);
        if (BVar3 == 0) {
            return;
        }
        uVar7 = pass1_1028_4faa(uVar5, 0x1050);
        uVar6 = (uVar7 >> 0x10);
        uVar2 = uVar7;
    } else {
        paVar4 = pass1_1028_121e(0x1050, uVar5);
        uVar6 = (paVar4 >> 0x10);
        uVar2 = SUB42(paVar4, 0x0);
    }
    pass1_1028_b58e(CONCAT22(uVar6, uVar2));
    return;
}


pub unsafe fn pass1_1010_e964(mut param_1: u16) {
    let mut uVar1: u32;
    let mut uVar2: u16;
    let mut in_register_0000000a: u16;
    let mut puVar3: *mut u32;
    let mut in_stack_0000fe92: u16;
    let mut in_stack_0000ffb6: u16;
    let mut in_stack_0000ffbc: u16;
    let mut in_stack_0000ffc0: u16;
    let mut in_stack_0000ffea: u16;

    puVar3 = mixed_1010_20ba(
        CONCAT22(in_register_0000000a, param_1),
        _u16_1050_0ed0,
        CONCAT22(in_stack_0000ffea, 0x2f),
        in_stack_0000fe92,
        in_stack_0000ffb6,
        in_stack_0000ffbc,
        in_stack_0000ffc0,
    );
    uVar2 = (puVar3 >> 0x10);
    uVar1 = (puVar3 + 0x24);
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uVar1);
    pass1_1038_4d28((uVar1 & 0xffff | uVar2 << 0x10));
    return;
}


pub unsafe fn struct_1010_e9e4(param_1: *mut Struct19, mut param_2: u16) {
    let mut puVar1: *mut u16;
    let mut uVar2: u16;
    let mut iVar3: i16;
    let mut uVar4: u16;
    let mut uVar5: u16;
    let mut uVar6: u32;
    let mut in_EDX: u32;
    let mut uVar8: u16;
    let mut paVar7: *mut Struct57;
    let mut iVar9: i16;
    let mut paVar10: *mut Struct19;
    let mut puVar11: *mut u16;
    let mut iStack4: i16;

    uVar8 = (in_EDX >> 0x10);
    paVar10 = struct_op_1010_1d48(param_1, param_2);
    paVar7 = CONCAT22(uVar8, (paVar10 >> 0x10));
    (param_1 + 0xa) = 0x389a;
    (param_1 + 0xc) = 0x1008;
    (param_1 + 0xa) = 0x3aa8;
    (param_1 + 0xc) = 0x1008;
    uVar5 = 0;
    (param_1 + 0xe) = 0;
    (param_1 + 0x12) = 0;
    (param_1 + 0x16) = 0;
    (param_1 + 0x1a) = 0;
    (param_1 + 0x1e) = 0;
    (param_1 + 0x20) = 0;
    (param_1 + 0x24) = 0;
    (param_1 + 0x28) = 0;
    (param_1 + 0x2c) = 0;
    (param_1 + 0x30) = 0;
    (param_1 + 0x32) = 0;
    param_1.offset_0x0 = 0x558;
    (param_1 + 0x2) = 0x1018;
    (param_1 + 0xa) = 0x568;
    (param_1 + 0xc) = 0x1018;
    mem_op_1000_179c(0x4, paVar7);
    if ((paVar7 | uVar5) == 0) {
        (param_1 + 0xe) = 0;
    } else {
        puVar11 = pass1_1018_dcf6(CONCAT22(paVar7, uVar5));
        (param_1 + 0xe) = puVar11;
        (param_1 + 0x10) = (puVar11 >> 0x10);
    }
    pass1_1000_4906((param_1 & 0xffff0000 | (param_1 + 0x34)), NULL, 0x24);
    (param_1 + 0x38) = 0xfa;
    (param_1 + 0x3c) = 0x15e;
    uVar6 = 0x1c2;
    (param_1 + 0x40) = 0x1c2;
    (param_1 + 0x44) = 0x1c2;
    (param_1 + 0x46) = 0x2260000;
    (param_1 + 0x4a) = 0x28a0000;
    (param_1 + 0x4e) = 0x730000;
    (param_1 + 0x52) = 0x960000;
    (param_1 + 0x56) = 0;
    //   for (iStack4 = 0x1; iStack4 < 0x9; iStack4 += 1)
    for iStack4 in 1..9 {
        pass1_1008_612e(uVar6, 0x0, 0x1d);
        uVar5 = uVar6;
        pass1_1008_612e(uVar5, 0x1, 0x2);
        if ((uVar6 & 1) != 0) {
            uVar5 = -uVar5;
        }
        iVar9 = iStack4 * 0x4;
        puVar1 = (param_1 + 0x34 + iVar9);
        uVar2 = *puVar1;
        uVar4 = uVar5 + *puVar1;
        uVar6 = uVar4;
        iVar3 = (param_1 + 0x34 + iVar9 + 2);
        (param_1 + iVar9 + 0x34) = uVar4;
        (param_1 + iVar9 + 0x36) = (uVar5 >> 0xf) + iVar3 + CARRY2(uVar5, uVar2);
    }
    return;
}







pub unsafe fn pass1_1010_ec40(
    mut param_1: i16,
    mut param_2: u16,
    mut param_3: u16,
    mut param_4: u16,
    mut param_5: u32,
) -> u32 {
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2, param_5);
    return CONCAT22((param_1 + 0x12), (param_1 + 0x10));
}
