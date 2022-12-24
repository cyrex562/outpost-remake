pub unsafe fn struct_1028_0068(param_1: *mut Struct57, param_2: *mut astruct_180)

{
    let mut uVar1: u16;
    let mut uVar2: u16;
    let mut iVar2: *mut astruct_180;
    let mut uVar3: u16;

    struct_1028_b354(param_2);
    uVar3 = (param_2 >> 0x10);
    iVar2 = param_2;
    uVar1 = 0;
    (iVar2 + 1).field0_0x0 = 0;
    iVar2[0x1].field1_0x2 = 0;
    param_2.field0_0x0 = 0x8ec;
    iVar2.field1_0x2 = 0x1028;
    mem_op_1000_179c(0xc, param_1);
    uVar2 = param_1 | uVar1;
    if (uVar2 == 0) {
        iVar2[0x1].field1_0x2 = 0;
    } else {
        set_struct_1008_574a(CONCAT22(param_1, uVar1));
        iVar2[0x1].field1_0x2 = uVar1;
        iVar2[0x1].field_0x4 = uVar2;
    }
    return;
}

pub unsafe fn pass1_1028_00cc(param_1: *mut StructD, param_2: *mut astruct_12, mut param_3: i16, mut param_4: u32)

{
    let mut uVar1: u16;
    let mut uVar2: u16;
    let mut in_register_0000000a: u16;
    let mut paVar3: *mut Struct57;

    paVar3 = CONCAT22(in_register_0000000a, param_1);
    pass1_1028_b39e(param_1, param_2, param_3, param_4);
    uVar1 = 0;
    (param_2 + 0x20) = 0;
    (param_2 + 0x22) = 0;
    param_2.field0_0x0 = 0x8ec;
    (param_2 + 0x2) = 0x1028;
    mem_op_1000_179c(0xc, paVar3);
    uVar2 = paVar3 | uVar1;
    if (uVar2 == 0) {
        (param_2 + 0x22) = 0;
    } else {
        set_struct_1008_574a(CONCAT22(paVar3, uVar1));
        (param_2 + 0x22) = uVar1;
        (param_2 + 0x24) = uVar2;
    }
    return;
}

pub unsafe fn pass1_1028_0138(param_1: *mut u16)

{
    let mut puVar1: *mut u32;
    let mut uVar2: u16;
    let mut ppcVar3: *mut *mut code;
    let mut iVar4: i16;
    let mut uVar5: u16;

    uVar5 = (param_1 >> 0x10);
    iVar4 = param_1;
    *param_1 = 0x8ec;
    (iVar4 + 0x2) = 0x1028;
    puVar1 = (iVar4 + 0x22);
    uVar2 = (iVar4 + 0x24);
    if ((uVar2 | puVar1) != 0) {
        ppcVar3 = *puVar1;
        (**ppcVar3)();
    }
    pass1_1028_b418(param_1);
    return;
}





pub unsafe fn pass1_1028_0550(param_1: *mut astruct_15)

{
    let mut in_EDX: *mut Struct57;
    let mut uVar1: u16;
    let mut uVar2: u16;
    let mut uVar3: u16;
    let mut iVar4: i16;

    pass1_1028_be9e(param_1);
    uVar1 = (param_1 >> 0x10);
    if ((param_1 + 0x12) == 0x5) {
        uVar3 = 0;
        iVar4 = 0x4;
        uVar2 = 0x1;
        uVar1 = pass1_1028_b58e((param_1 & 0xffff | uVar1 << 0x10));
        pass1_1030_7c50(uVar1, in_EDX, CONCAT22(in_EDX, uVar1), CONCAT22(uVar3, uVar2), iVar4);
    }
    return;
}






pub unsafe fn pass1_1028_081e(mut param_1: i16, mut param_2: u16, param_3: *mut astruct_15)

{
    let mut piVar1: *mut i16;
    let mut iVar2: i16;
    let mut uVar3: u16;
    let mut uVar4: u32;
    let mut uVar5: u16;
    let mut iVar6: i16;
    let mut uVar7: u16;

    pass1_1028_b58e(param_3);
    uVar4 = (param_1 + 0x2e);
    iVar2 = (uVar4 + 0x18);
    uVar7 = (param_3 >> 0x10);
    iVar6 = param_3;
    piVar1 = (iVar6 + 0x20);
    *piVar1 = *piVar1 + 1;
    uVar5 = *_PTR_LOOP_1050_65e2;
    uVar3 = (_PTR_LOOP_1050_65e2 + 2);
    if (iVar2 < 0xfa) {
        uVar5 &= 0x1;
    } else {
        if (0x1c1 < iVar2) {
            if (iVar2 < 0x226) {
                return;
            }
            if ((iVar2 < 0x2ee) && (CONCAT22(uVar3, uVar5) % 0x3 != 0)) {
                return;
            }
            piVar1 = (iVar6 + 0x20);
            *piVar1 = *piVar1 + 1;
            return;
        }
        uVar5 = (CONCAT22(uVar3, uVar5) % 0x3);
    }
    if (uVar5 != 0) {
        return;
    }
    piVar1 = (iVar6 + 0x20);
    *piVar1 = *piVar1 - 0x1;
    return;
}


pub unsafe fn struct_1028_0954(param_1: *mut astruct_180) -> *mut astruct_180

{
    let mut iVar1: *mut astruct_180;
    let mut uVar1: u16;

    struct_1028_b354(param_1);
    uVar1 = (param_1 >> 0x10);
    iVar1 = param_1;
    (iVar1 + 1).field0_0x0 = 0;
    param_1.field0_0x0 = 0xada;
    iVar1.field1_0x2 = 0x1028;
    iVar1.field11_0xe = 0x4b;
    return param_1;
}


pub unsafe fn pass1_1028_0982(mut param_1: u16, param_2: *mut astruct_12, mut param_3: i16, mut param_4: u32) -> *mut u16

{
    pass1_1028_b39e(param_1, param_2, param_3, param_4);
    (param_2 + 0x20) = 0;
    param_2.field0_0x0 = 0xada;
    (param_2 + 0x2) = 0x1028;
    (param_2 + 0xe) = 0x4b;
    return &param_2.field0_0x0;
}

pub unsafe fn pass1_1028_09b8(param_1: *mut astruct_15)

{
    let mut uVar1: u32;

    uVar1 = pass1_1028_b58e(param_1);
    (uVar1 + 0x14) = 0x1;
    return;
}



pub unsafe fn pass1_1028_09d4(mut param_1: i16, mut param_2: u16, mut param_3: u32, param_4: *mut u16, mut param_5: u32, param_6: i32)

{
    let mut iVar1: i16;
    let mut puVar2: *mut u8;
    let mut uVar3: u32;
    let mut uVar4: u16;
    let mut uVar5: u32;
    let mut uVar6: u16;
    let mut uVar7: u16;
    let mut local_6: [u8; 0x2] = [0; 0x2];
    let mut BStack4: bool;

    uVar6 = param_3;
    uVar7 = (param_3 >> 0x10);
    BStack4 = pass1_1028_c314(param_1, param_2, uVar6, uVar7, param_4, param_5, (param_5 >> 0x10), param_6);
    if (BStack4 == 0) {
        return;
    }
    pass1_1028_c7b6(param_2, uVar6, uVar7, param_4, param_6);
    if ((BStack4 != 0) && (BStack4 != 0x4)) {
        if (((BStack4 - 0x5) < 1) || ((SBORROW2(BStack4 - 0x5, 1) || (0x3 < (BStack4 - 0x6))))) {
            if (((uVar6 + 0xc) != 0x3e) && ((uVar6 + 0xc) != 0x41)) {
                return;
            }
            uVar5 = pass1_1030_bcae(local_6, 0x1050);
            uVar4 = (uVar5 >> 0x10);
            iVar1 = uVar5;
            pass1_1028_e1ec(_PTR_LOOP_1050_65e2, param_5);
            uVar3 = (iVar1 + 0x10);
            pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uVar3);
            puVar2 = local_6;
            pass1_1030_bcde(puVar2, 0x1050, uVar3 & 0xffff | uVar4 << 0x10, param_4, param_6);
            if (puVar2 < 0x0) {
                PTR_LOOP_1050_50ca = 0x6af;
                return;
            }
            if (0x5 < puVar2) {
                PTR_LOOP_1050_50ca = 0x6b5;
                return;
            }
            return;
        }
    }
    PTR_LOOP_1050_50ca = 0x6a8;
    return;
}


pub unsafe fn pass1_1028_0ab4(param_1: *mut StructD, param_2: u8) -> *mut StructD

{
    pass1_1028_b418(&param_1.address_offset_field_0x0);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}


pub unsafe fn struct_1028_0b42(param_1: *mut astruct_180) -> *mut u16

{
    struct_1028_b354(param_1);
    param_1.field0_0x0 = 0xbbc;
    (param_1 + 0x2) = 0x1028;
    return &param_1.field0_0x0;
}


pub unsafe fn pass1_1028_0b64(mut param_1: u16, param_2: *mut astruct_12, mut param_3: i16, mut param_4: u32) -> *mut u16

{
    pass1_1028_b39e(param_1, param_2, param_3, param_4);
    param_2.field0_0x0 = 0xbbc;
    (param_2 + 0x2) = 0x1028;
    return &param_2.field0_0x0;
}


pub unsafe fn struct_1028_0c24(param_1: *mut astruct_180) -> *mut u16

{
    let mut iVar1: *mut astruct_180;
    let mut uVar1: u16;

    struct_1028_b354(param_1);
    uVar1 = (param_1 >> 0x10);
    iVar1 = param_1;
    (iVar1 + 1).field0_0x0 = 0;
    iVar1[0x1].field1_0x2 = 0;
    param_1.field0_0x0 = s_480_bmp_1050_1721 + 0x3;
    iVar1.field1_0x2 = 0x1028;
    return &param_1.field0_0x0;
}


pub unsafe fn pass1_1028_0c50(mut param_1: u16, param_2: *mut astruct_12, mut param_3: i16, mut param_4: u32) -> *mut u16

{
    pass1_1028_b39e(param_1, param_2, param_3, param_4);
    (param_2 + 0x20) = 0;
    (param_2 + 0x22) = 0;
    param_2.field0_0x0 = s_480_bmp_1050_1721 + 0x3;
    (param_2 + 0x2) = 0x1028;
    return &param_2.field0_0x0;
}






pub unsafe fn pass1_1028_0d80(mut param_1: u32) -> u16

{
    let mut uVar1: u16;
    let mut uVar2: u16;

    uVar2 = (param_1 >> 0x10);
    uVar1 = (param_1 + 0x20);
    pass1_1028_1646((param_1 & 0xffff | uVar2 << 0x10));
    return uVar1;
}
