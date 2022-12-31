
pub fn window_op_1008_0af8(mut param_1: u16, struct_param_1: *mut StructA) {
    let pSVar1: *mut StructA;
    let mut HVar2: HWND16;
    let mut uVar3: u32;
    let mut uVar4: u16;
    let mut puVar5: *mut u8;
    let mut in_register_0000000a: u16;
    let mut paVar6: *mut Struct57;
    let iVar8: *mut StructA;
    let mut uVar8: u16;
    let mut uVar9: u16;
    let mut puVar10: *mut u16;
    let mut uVar11: u16;
    let mut uVar12: u8;
    let mut uVar13: u16;
    let mut struct_20_v6: *mut Struct20;
    let mut paVar7: *mut Struct57;
    let mut fn_ptr_1: *mut *mut code;

    paVar6 = CONCAT22(in_register_0000000a, param_1);
    create_window_ex_1008_9760(struct_param_1);
    uVar8 = (struct_param_1 >> 0x10);
    iVar8 = struct_param_1;
    HVar2 = iVar8.field4_0x8;
    HWND16_1050_0396 = HVar2;
    mem_op_1000_179c(0x12, paVar6);
    uVar4 = paVar6 | HVar2;
    paVar7 = (paVar6 & 0xffff0000 | uVar4);
    if (uVar4 != 0) {
        puVar10 = pass1_1008_91ba(CONCAT22(paVar6, HVar2));
        paVar7 = (paVar7 & 0xffff0000 | puVar10 >> 0x10);
        HVar2 = puVar10;
    }
    mem_op_1000_179c(0x6, paVar7);
    uVar4 = paVar7 | HVar2;
    paVar6 = (paVar7 & 0xffff0000 | uVar4);
    if (uVar4 == 0) {
        iVar8[0x1].field10_0x14 = 0;
    } else {
        pass1_1008_392e(CONCAT22(paVar7, HVar2), iVar8.field4_0x8);
        iVar8[0x1].field10_0x14 = HVar2;
        iVar8[0x1].field11_0x16 = paVar6;
    }
    fn_ptr_1 = (struct_param_1 + 0x14);
    (**fn_ptr_1)(0x1000, struct_param_1, 0x0, 0x15a, 0x1050);
    uVar9 = 0x1000;
    mem_op_1000_179c(0xec, paVar6);
    struct_20_v6 = CONCAT22(paVar6, HVar2);
    uVar4 = paVar6 | HVar2;
    if (uVar4 == 0) {
        iVar8[0x1].field12_0x18 = 0;
    } else {
        pSVar1 = iVar8 + 1;
        pSVar1.field0_0x0 = pSVar1.field0_0x0 + 1;
        uVar9 = 0x1020;
        pass1_1020_08b6(struct_20_v6, (iVar8 + 1).field0_0x0, struct_param_1);
        iVar8[0x1].field12_0x18 = HVar2;
        iVar8[0x1].field13_0x1a = uVar4;
    }
    if (&iVar8[0x1].field1_0x2 != 0) {
        fn_ptr_1 = (*&iVar8[0x1].field1_0x2 + 0x10);
        (**fn_ptr_1)();
    }
    iVar8[0x1].field1_0x2 = &iVar8[0x1].field12_0x18;
    uVar13 = 0x1;
    uVar3 = &iVar8[0x1].field12_0x18;
    uVar11 = uVar3;
    uVar12 = (uVar3 >> 0x10);
    fn_ptr_1 = (*&iVar8[0x1].field12_0x18 + 0x10);
    (**fn_ptr_1)();
    uVar3 = &iVar8[0x1].field12_0x18;
    puVar5 = iVar8[0x1].field13_0x1a;
    iVar8[0x1].field14_0x1c = uVar3;
    fn_ptr_1 = (*&iVar8[0x1].field14_0x1c + 0x8);
    (**fn_ptr_1)(
        uVar9,
        iVar8[0x1].field14_0x1c,
        puVar5,
        uVar11,
        uVar12,
        uVar13,
    );
    uVar4 = uVar3;
    fn_ptr_1 = (*&iVar8[0x1].field14_0x1c + 0xc);
    (**fn_ptr_1)();
    pass1_1008_6978(
        uVar4,
        puVar5,
        struct_param_1 & 0xffff | uVar8 << 0x10,
        0x0,
        iVar8[0x1].field14_0x1c,
    );
    return;
}

pub fn caseD_aa() {
    let mut in_DX: u16;
    let mut unaff_BP: i16;
    let mut in_stack_0000ffee: u16;

    ui_op_1008_2c4e(in_DX, in_stack_0000ffee, (unaff_BP + 0x6), 0x58);
    return;
}

pub fn caseD_ac() {
    let mut in_DX: u16;
    let mut unaff_BP: i16;
    let mut in_stack_0000ffee: u16;

    ui_op_1008_2c4e(in_DX, in_stack_0000ffee, (unaff_BP + 0x6), 0x59);
    return;
}

pub fn caseD_ad() {
    let mut in_DX: u16;
    let mut unaff_BP: i16;
    let mut in_stack_0000ffee: u16;

    ui_op_1008_2c4e(in_DX, in_stack_0000ffee, (unaff_BP + 0x6), 0x5a);
    return;
}

pub fn caseD_ae() {
    let mut in_DX: u16;
    let mut unaff_BP: i16;
    let mut in_stack_0000ffee: u16;

    ui_op_1008_2c4e(in_DX, in_stack_0000ffee, (unaff_BP + 0x6), 0x5b);
    return;
}

pub fn caseD_b1() {
    let mut in_DX: u16;
    let mut unaff_BP: i16;
    let mut in_stack_0000ffee: u16;

    ui_op_1008_2c4e(in_DX, in_stack_0000ffee, (unaff_BP + 0x6), 0x5c);
    return;
}
