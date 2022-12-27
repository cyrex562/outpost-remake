pub fn pass1_1010_9298(
    param_1: *mut StructD,
    param_2: *mut Struct19,
    mut param_3: u16,
) -> u32 {
    let mut uVar1: u16;
    let mut paVar2: *mut Struct57;

    uVar1 = (param_1 >> 0x10);
    paVar2 = (param_1 & 0xffff0000 | param_1 & 0xffff);
    struct_1010_2cd2(param_2, param_3);
    //        1010:9566  40  95  10  10      addr         pass1_1010_9540
    param_2.offset_0x0 = 0x9566;
    (param_2 + 0x2) = 0x1010;
    mem_op_1000_179c(0x20c, paVar2);
    (param_2 + 0x5c) = uVar1;
    (param_2 + 0x5e) = paVar2;
    pass1_1000_4906(CONCAT22(paVar2, (param_2 + 0x5c)), NULL, 0x20c);
    return param_2;
}

pub fn struct_1010_95aa(param_1: *mut Struct19, mut param_2: u16) {
    struct_op_1010_1d48(param_1, param_2);
    (param_1 + 0xa) = 0;
    (param_1 + 0xe) = 0;
    (param_1 + 0x12) = 0;
    (param_1 + 0x16) = 0;
    (param_1 + 0x18) = 0;
    (param_1 + 0x1a) = 0;
    (param_1 + 0x1c) = 0xa;
    (param_1 + 0x1e) = 0;
    param_1.offset_0x0 = 0xa1c8;
    (param_1 + 0x2) = 0x1010;
    return;
}

pub fn pass1_1010_9fee(
    param_1: *mut StructD,
    param_2: *mut astruct_252,
    mut param_3: u16,
    mut param_4: u16,
) {
    let mut iVar5: *mut StructD;
    let mut uVar1: u16;
    let mut uVar2: u16;
    let mut paVar3: *mut Struct57;
    let mut iVar3: *mut astruct_252;
    let mut iVar4: *mut astruct_253;
    let mut uVar4: u16;
    let mut uVar5: u16;
    let mut puStack10: *mut StructD;
    let mut pSStack6: *mut StructD;
    let mut fn_ptr_1: *mut *mut code;

    iVar5 = (param_1 >> 0x10);
    paVar3 = (param_1 & 0xffff0000 | param_1 & 0xffff);
    uVar4 = (param_2 >> 0x10);
    iVar3 = param_2;
    if (iVar3.field18_0x12.is_null()) {
        mem_op_1000_179c(0xc, paVar3);
        uVar1 = paVar3;
        uVar2 = uVar1 | iVar5;
        paVar3 = (paVar3 & 0xffff0000 | uVar2);
        if (uVar2 == 0) {
            iVar3.field18_0x12 = null_mut();
        } else {
            set_struct_1008_574a(CONCAT22(uVar1, iVar5));
            iVar3.field18_0x12 = iVar5;
            (&iVar3.field18_0x12 + 0x2) = paVar3;
        }
    }
    mem_op_1000_179c(0x8, paVar3);
    uVar1 = paVar3;
    puStack10 = CONCAT22(uVar1, iVar5);
    if ((uVar1 | iVar5) == 0) {
        pSStack6 = null_mut();
    } else {
        puStack10.address_offset_field_0x0 = 0x389a;
        iVar5.address_offset_field_0x2 = 0x1008;
        puStack10.address_offset_field_0x0 = 0xa1c4;
        iVar5.address_offset_field_0x2 = 0x1010;
        pSStack6 = puStack10;
    }
    uVar5 = (pSStack6 >> 0x10);
    iVar4 = pSStack6;
    iVar4.field4_0x4 = param_4;
    iVar4.field5_0x6 = param_3;
    fn_ptr_1 = (*iVar3.field18_0x12 + 0x4);
    (**fn_ptr_1)(0x1000, iVar3.field18_0x12, iVar4, uVar5);
    return;
}
