









pub fn pass1_1030_e410(mut param_1: u16, mut param_2: u16, mut param_3: u32) {
    let mut puVar1: *mut u8;
    let mut uVar2: u16;
    let mut puVar3: *mut u16;
    let mut in_stack_0000fe9c: u16;
    let mut local_10: [u8; 0x6] = [0; 0x6];
    let mut local_a: [u8; 0x4] = [0; 0x4];
    let mut uStack6: u16;
    let mut uStack4: u16;

    pass1_1028_e1ec(_PTR_LOOP_1050_65e2, (param_3 + 0x10c));
    puVar1 = (param_2 | param_1);
    if (puVar1.is_null() == false) {
        uStack6 = param_1;
        uStack4 = param_2;
        pass1_1038_4fd8(param_1, CONCAT22(param_2, param_1), 0x21);
        if (param_1 == 0) {
            pass1_1020_a43e(puVar1, CONCAT22(0x1050, local_a));
            puVar3 = pass1_1008_3e54(CONCAT22(0x1050, local_10), 0x0, 0x2, 0xfffd);
            uVar2 = (puVar3 >> 0x10);
            pass1_1020_a49a(
                uVar2,
                in_stack_0000fe9c,
                CONCAT22(0x1050, local_a),
                CONCAT22(0x1050, local_10),
                0x7a,
            );
            pass1_1008_3e76(CONCAT22(0x1050, local_10), 0x0, 0x3, 0xfffe);
            pass1_1020_a49a(
                uVar2,
                in_stack_0000fe9c,
                CONCAT22(0x1050, local_a),
                CONCAT22(0x1050, local_10),
                0x7a,
            );
            pass1_1008_3e76(CONCAT22(0x1050, local_10), 0x0, 0x3, 0xfffd);
            pass1_1020_a49a(
                uVar2,
                in_stack_0000fe9c,
                CONCAT22(0x1050, local_a),
                CONCAT22(0x1050, local_10),
                0x21,
            );
        }
    }
    return;
}
pub fn pass1_1030_e4ba() {
    return;
}


pub fn struct_1030_e4fa(param_1: *mut astruct_97, mut param_2: u32) {
    let mut iVar1: *mut astruct_97;
    let mut uVar1: u16;

    struct_op_1028_d1dc(param_1, 0x3e80);
    uVar1 = (param_1 >> 0x10);
    iVar1 = param_1;
    iVar1.field259_0x108 = param_2;
    param_1.offset_0x0 = 0xe62e;
    iVar1.segment_0x2 = 0x1030;
    sys_1000_3f9c(
        (param_1 & 0xffff0000 | ZEXT24(&iVar1.string_0x8)),
        s_SCKillBldg__0x_08lx_1050_597c,
        iVar1.field259_0x108,
    );
    return;
}








pub fn pass1_1030_e63e(param_1: *mut astruct_97, mut param_2: u16) -> *mut astruct_97 {
    let mut iVar1: *mut astruct_97;
    let mut uVar1: *mut astruct_97;

    iVar1 = param_1;
    uVar1 = (param_1 >> 0x10);
    struct_op_1028_d1dc(param_1, 0xf9f);
    iVar1.field259_0x108 = param_2;
    param_1.offset_0x0 = 0xe78a;
    iVar1.segment_0x2 = 0x1030;
    unk_str_op_1000_3d3e(
        (param_1 & 0xffff0000 | ZEXT24(&iVar1.string_0x8)),
        s_SCKillColony_1050_5990,
    );
    return param_1;
}




pub fn pass1_1030_e79a(param_1: *mut astruct_97) -> *mut astruct_97 {
    struct_op_1028_d1dc(param_1, 0xf9f);
    param_1.offset_0x0 = 0xe890;
    (param_1 + 0x2) = 0x1030;
    unk_str_op_1000_3d3e(
        (param_1 & 0xffff0000 | (param_1 + 0x8)),
        s_SCKillRebelColony_1050_599e,
    );
    return param_1;
}



pub fn pass1_1030_e8a0(
    param_1: *mut astruct_97,
    mut param_2: u32,
    mut param_3: u16,
    mut param_4: u32,
) {
    let mut iVar1: *mut astruct_97;
    let mut uVar1: u16;

    struct_op_1028_d1dc(param_1, 0x2710);
    uVar1 = (param_1 >> 0x10);
    iVar1 = param_1;
    iVar1.field259_0x108 = param_2;
    iVar1.field262_0x10c = param_4;
    iVar1.field264_0x110 = param_3;
    param_1.offset_0x0 = 0xeb40;
    iVar1.segment_0x2 = 0x1030;
    sys_1000_3f9c(
        (param_1 & 0xffff0000 | ZEXT24(&iVar1.string_0x8)),
        s_SCMoveBas_to_0x_08lx_1050_59b0,
        iVar1.field262_0x10c,
    );
    return;
}





pub fn pass1_1030_ea50(mut param_1: u32, mut param_2: u32) {
    let mut uVar1: u32;
    let mut BVar2: bool;
    let mut in_EDX: *mut Struct57;
    let mut uVar3: u32;
    let mut uVar4: u16;
    let mut iVar5: i16;
    let mut uVar6: u16;
    let mut uVar7: u32;
    let mut local_12: u32;
    let mut local_e: u16;
    let mut iStack12: i16;
    let mut uStack10: u16;
    let mut uStack8: u16;
    let mut uStack6: u32;

    uStack6 = 0x1869f;
    uVar6 = (param_1 >> 0x10);
    iVar5 = param_1;
    BVar2 = pass1_1008_c6ae(_u16_1050_06e0, (iVar5 + 0x110), 0x3);
    if (BVar2 != 0) {
        uVar7 = struct_op_1030_73a8(param_2, BVar2, in_EDX);
        uVar3 = in_EDX & 0xffff0000;
        local_e = uVar7;
        iStack12 = (uVar7 >> 0x10);
        uStack6 = pass1_1028_45e2(local_e, iStack12, uVar7);
        in_EDX = (uVar3 & 0xffff0000);
    }
    uVar1 = (iVar5 + 0x108);
    uStack8 = (uVar1 + 0x4);
    uStack10 = 0;
    loop {
        uVar4 = (in_EDX >> 0x10);
        if (uStack8 <= uStack10) {
            return;
        }
        pass1_1020_bb16(
            (iVar5 + 0x108),
            CONCAT22(0x1050, &local_12),
            CONCAT22(0x1050, &local_e),
            uStack10,
        );
        in_EDX = CONCAT22(uVar4, uStack6);
        if (uStack6 < local_12) {
            pass1_1030_7ddc(uStack6, in_EDX, param_2, uStack6, local_e);
            uStack6 = 0;
        } else {
            uStack6 -= local_12;
            pass1_1030_7ddc(local_12, in_EDX, param_2, local_12, local_e);
        }
        if ((uStack6 | uStack6) == 0) {
            break;
        }
        uStack10 += 0x1;
    }
    return;
