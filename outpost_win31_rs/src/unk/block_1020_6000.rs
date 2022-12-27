pub fn pass1_1020_61c4(
    mut param_1: u16,
    mut param_2: u16,
    mut param_3: u32,
    param_4: *mut u16,
) {
    let mut uVar1: u32;
    let mut uVar2: u16;
    let mut in_EDX: *mut Struct57;
    let mut puVar3: *mut u32;
    let mut in_stack_0000fe9a: u16;
    let mut in_stack_0000ffbe: u16;
    let mut in_stack_0000ffc4: u16;
    let mut in_stack_0000ffc8: u16;
    let mut in_stack_0000fff2: u16;

    puVar3 = mixed_1010_20ba(
        in_EDX,
        _u16_1050_0ed0,
        CONCAT22(in_stack_0000fff2, 0x2f),
        in_stack_0000fe9a,
        in_stack_0000ffbe,
        in_stack_0000ffc4,
        in_stack_0000ffc8,
    );
    uVar2 = (puVar3 >> 0x10);
    uVar1 = (puVar3 + 0x20);
    pass1_1030_8308(
        uVar1,
        uVar2,
        _u16_1050_5748,
        (_u16_1050_5748 >> 0x10),
        param_3,
        param_4,
        uVar1,
    );
    *param_4 = (puVar3 + 0x1e);
    return;
}



pub fn pass1_1020_62e0(mut param_1: i16, mut param_2: u16, mut param_3: u16) {
    let mut puVar1: *mut u32;
    let mut ppcVar2: *mut *mut code;
    let mut puVar3: *mut u16;
    let mut uVar4: u32;
    let mut uVar5: u16;
    let mut uVar6: u16;
    let mut in_EDX: *mut Struct57;
    let mut paVar8: *mut Struct57;
    let mut paVar9: *mut Struct57;
    let mut puVar10: *mut u32;
    let mut in_stack_0000fe3e: u16;
    let mut in_stack_0000fe84: u16;
    let mut in_stack_0000fe96: u16;
    let mut in_stack_0000ff62: u16;
    let mut in_stack_0000ff68: u16;
    let mut in_stack_0000ff6c: u16;
    let mut in_stack_0000ffa8: u16;
    let mut in_stack_0000ffae: u16;
    let mut in_stack_0000ffb2: u16;
    let mut in_stack_0000ffba: u16;
    let mut in_stack_0000ffc0: u16;
    let mut in_stack_0000ffc4: u16;
    let mut in_stack_0000ffc6: u32;
    let mut uVar11: u16;
    let mut uVar12: u16;
    let mut uVar13: u16;
    let mut iVar14: i16;
    let mut uVar15: u16;
    let mut in_stack_0000ffee: u16;
    let mut paVar7: *mut Struct57;

    set_struct_op_1020_921c(
        in_EDX,
        CONCAT22(param_2, param_1),
        param_3,
        in_stack_0000ffc6,
    );
    (param_1 + 0x14) = 0;
    (param_1 + 0x2c) = 0;
    CONCAT22(param_2, param_1) = 0x67c2;
    (param_1 + 0x2) = 0x1020;
    puVar3 = pass1_1000_4906(CONCAT22(param_2, param_1 + 0x18), NULL, 0x14);
    mem_op_1000_179c(0x3c, in_EDX);
    uVar5 = in_EDX | puVar3;
    paVar7 = (in_EDX & 0xffff0000 | uVar5);
    if (uVar5 == 0) {
        (param_1 + 0x1c) = 0;
    } else {
        pass1_1020_87c2(CONCAT22(in_EDX, puVar3));
        (param_1 + 0x1c) = puVar3;
        (param_1 + 0x1e) = paVar7;
    }
    mem_op_1000_179c(0x26, paVar7);
    uVar5 = paVar7 | puVar3;
    paVar9 = (paVar7 & 0xffff0000);
    paVar8 = (paVar9 | uVar5);
    if (uVar5 == 0) {
        puVar3 = null_mut();
    } else {
        pass1_1020_8a9c(
            CONCAT22(paVar7, puVar3),
            paVar8,
            in_stack_0000ff68,
            in_stack_0000ff6c,
            in_stack_0000ff62,
            in_stack_0000fe3e,
        );
        paVar9 = paVar8;
    }
    (param_1 + 0x20) = puVar3;
    (param_1 + 0x22) = paVar9;
    mem_op_1000_179c(0xbe, paVar9);
    uVar5 = paVar9 | puVar3;
    paVar7 = (paVar9 & 0xffff0000);
    paVar8 = (paVar7 | uVar5);
    if (uVar5 == 0) {
        puVar3 = null_mut();
    } else {
        pass1_1020_8eaa(CONCAT22(paVar9, puVar3), paVar8);
        paVar7 = paVar8;
    }
    (param_1 + 0x24) = puVar3;
    (param_1 + 0x26) = paVar7;
    mem_op_1000_179c(0x20, paVar7);
    uVar5 = paVar7 | puVar3;
    paVar9 = (paVar7 & 0xffff0000);
    paVar8 = (paVar9 | uVar5);
    if (uVar5 == 0) {
        puVar3 = null_mut();
    } else {
        pass1_1020_8360(CONCAT22(paVar7, puVar3));
        paVar9 = paVar8;
    }
    (param_1 + 0x28) = puVar3;
    (param_1 + 0x2a) = paVar9;
    pass1_1020_6746(CONCAT22(param_2, param_1), 0x1, 0x4);
    puVar10 = mixed_1010_20ba(
        paVar9,
        _u16_1050_0ed0,
        CONCAT22(in_stack_0000ffee, 0x29),
        in_stack_0000fe96,
        in_stack_0000ffba,
        in_stack_0000ffc0,
        in_stack_0000ffc4,
    );
    paVar7 = (paVar9 & 0xffff0000 | puVar10 >> 0x10);
    (param_1 + 0x14) = puVar10;
    uVar6 = (puVar10 >> 0x10);
    (param_1 + 0x16) = uVar6;
    uVar13 = 0;
    uVar12 = (param_1 + 0x14);
    ppcVar2 = ((param_1 + 0x14) + 0x4);
    iVar14 = param_1;
    uVar15 = param_2;
    (**ppcVar2)();
    (param_1 + 0x6) = (param_1 + 0x14);
    uVar4 = (param_1 + 0x14);
    puVar1 = (uVar4 + 0xa);
    uVar4 = CONCAT22(param_2, param_1 + 0xa);
    uVar11 = SUB42(puVar1, 0x0);
    ppcVar2 = (*puVar1 + 0x8);
    (**ppcVar2)(
        0x1010,
        uVar11,
        (puVar1 >> 0x10),
        uVar4,
        uVar12,
        uVar6,
        uVar13,
        iVar14,
        uVar15,
    );
    (param_1 + 0x12) = uVar4;
    (param_1 + 0x10) = 0x1;
    puVar10 = mixed_1010_20ba(
        paVar7,
        _u16_1050_0ed0,
        CONCAT22(uVar11, 0x2),
        in_stack_0000fe84,
        in_stack_0000ffa8,
        in_stack_0000ffae,
        in_stack_0000ffb2,
    );
    (param_1 + 0x2c) = puVar10;
    (param_1 + 0x2e) = (puVar10 >> 0x10);
    return;
}

pub fn pass1_1020_6466(param_1: *mut StructD) {
    let mut iVar1: *mut StructD;
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    iVar1 = param_1;
    param_1.address_offset_field_0x0 = 0x67c2;
    iVar1.address_offset_field_0x2 = 0x1020;
    if (iVar1.field12_0x14 != 0) {
        pass1_1010_1ea6(iVar1.field12_0x14, (param_1 & 0xffff | uVar1 << 0x10));
    }
    palette_op_1020_92c4(param_1);
    return;
}

pub fn pass1_1020_6498(mut param_1: u32, mut param_2: i16) -> u32 {
    let mut uVar1: u32;
    let mut iVar2: i16;
    let mut uVar3: u16;

    uVar3 = (param_1 >> 0x10);
    if ((param_1 + 0x18 + param_2 * 0x4) != 0) {
        uVar1 = (param_1 + 0x18 + param_2 * 0x4);
        uVar3 = (uVar1 >> 0x10);
        iVar2 = uVar1;
        return CONCAT22((iVar2 + 0xa), (iVar2 + 0x8));
    }
    return 0x0;
}

pub fn pass1_1020_64d4(mut param_1: u32, mut param_2: i16) -> u16 {
    let mut uVar1: u32;
    let mut uVar2: u16;

    uVar2 = (param_1 >> 0x10);
    if ((param_1 + 0x18 + param_2 * 0x4) != 0) {
        uVar1 = (param_1 + 0x18 + param_2 * 0x4);
        return (uVar1 + 0x4);
    }
    return 0x0;
}

pub fn pass1_1020_6746(mut param_1: u32, mut param_2: i16, mut param_3: i16) {
    let mut ppcVar1: *mut *mut code;
    let mut uVar2: u32;
    let mut iVar3: i16;
    let mut uVar4: u16;

    if (param_3 != 0) {
        uVar4 = (param_1 >> 0x10);
        iVar3 = param_1;
        if ((iVar3 + 0x18 + param_3 * 0x4) != 0) {
            uVar2 = (iVar3 + 0x18 + param_3 * 0x4);
            (uVar2 + 0x4) = param_2;
            (iVar3 + 0x10) = 0x1;
            if (param_2 == 0) {
                ppcVar1 = ((iVar3 + 0x18 + param_3 * 0x4) + 0x14);
                (**ppcVar1)();
            }
        }
    }
    return;
}




pub fn pass1_1020_68de(mut param_1: u32) {
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    if ((param_1 + 0xf6) != 0) {
        invalidate_rect_1020_735a((param_1 + 0xf6));
    }
    return;
}
