
pub unsafe fn pass1_1010_503e(param_1: *mut u8, param_2: *mut Struct19, mut param_3: u16) {
    struct_op_1018_4cda(param_2, param_3);
    // just 0x5099
    // 0x1010:509a = ptr to fn ptr in table
    param_2.offset_0x0 = s_SCInternalPutBldg2_site_0x_08lx__1050_5099 + 1;
    (param_2 + 0x2) = 0x1010;
    pass1_1018_4dce(param_1, param_2, 0x1b3);
    _PTR_LOOP_1050_4230 = param_2;
    return;
}


pub unsafe fn struct_1010_50b2(param_1: *mut Struct19, mut param_2: u16) {
    struct_op_1010_1d48(param_1, param_2);
    (param_1 + 0xa) = 0;
    (param_1 + 0xc) = 0;
    (param_1 + 0x10) = 0;
    (param_1 + 0x12) = 0;
    (param_1 + 0x16) = 0;
    param_1.offset_0x0 = 0x53f4;
    (param_1 + 0x2) = 0x1010;
    return;
}






pub unsafe fn pass1_1010_519a(param_1: *mut u8, mut param_2: u32, param_3: *mut c_char) {
    let mut uVar1: u16;
    let mut uVar2: u32;
    let mut uVar3: u16;
    let mut paVar4: *mut astruct_92;
    let mut uVar5: u16;
    let mut in_register_0000000a: u16;
    let mut paVar6: *mut Struct57;
    let mut iVar5: *mut astruct_246;
    let mut iVar6: *mut astruct_247;
    let mut iVar7: i16;
    let mut uVar8: u16;
    let mut uVar9: u16;
    let mut piStack44: *mut i16;
    let mut local_18: *mut astruct_92;

    paVar6 = CONCAT22(in_register_0000000a, param_1);
    pass1_1028_dc52(CONCAT22(0x1050, &local_18), 0x1, 0x0, 0x400);
    uVar8 = (param_2 >> 0x10);
    iVar5 = param_2;
    iVar5.field14_0x10 = local_18.field5_0xc;
    fn_ptr_1000_17ce(*&iVar5.field12_0xc);
    uVar3 = iVar5.field14_0x10 << 0x2;
    mem_op_1000_179c(uVar3, paVar6);
    iVar5.field12_0xc = uVar3;
    iVar5.field13_0xe = paVar6;
    iVar5.field14_0x10 = 0;
    loop {
        uVar5 = paVar6;
        paVar4 = &local_18;
        pass1_1028_e4ec(CONCAT22(0x1050, paVar4));
        paVar6 = (uVar5 | paVar4);
        if ((uVar5 | paVar4) == 0) {
            break;
        }
        if (paVar4[0x1c].field4_0x8 != 0x8000002) {
            uVar1 = (&paVar4.field3_0x4 + 2);
            paVar6 = uVar1;
            uVar2 = &iVar5.field12_0xc;
            uVar9 = (uVar2 >> 0x10);
            iVar7 = uVar2;
            iVar6 = (iVar5.field14_0x10 * 0x4);
            piStack44 = (param_2 & 0xffff0000 | ZEXT24(&iVar5.field14_0x10));
            (iVar6 + iVar7) = &paVar4.field3_0x4;
            (iVar6 + iVar7 + 0x2) = uVar1;
            *piStack44 = *piStack44 + 1;
        }
    }
    param_3 = iVar5.field14_0x10;
    return;
}



pub unsafe fn string_1010_5286(
    param_1: *mut c_char,
    mut param_2: u16,
    mut param_3: u16,
    mut param_4: u16,
    mut param_5: u32,
) -> u32 {
    let mut in_buf_len_5: i16;
    let mut in_register_0000000a: u16;
    let mut paVar1: *mut Struct57;
    let mut pcVar2: *mut c_char;
    let mut UStack10: u32;
    let mut pcStack6: *mut c_char;

    pass1_1028_e1ec(_PTR_LOOP_1050_65e2, param_5);
    pcStack6 = CONCAT22(param_2, param_1);
    paVar1 = CONCAT22(in_register_0000000a, param_2 | param_1);
    if ((param_2 | param_1) == 0) {
        return 0x0;
    }
    mem_op_1000_179c(0x80, paVar1);
    in_buf_len_5 = paVar1;
    UStack10 = CONCAT22(in_buf_len_5, param_1);
    load_string_1010_84e0(
        _u16_1050_14cc,
        (_u16_1050_14cc >> 0x10),
        0x80,
        param_1,
        in_buf_len_5,
    );
    pass1_1000_3cea(UStack10, 0x105013ac);
    pcVar2 = pass1_1038_4d28(pcStack6);
    pass1_1000_3cea(UStack10, pcVar2);
    return CONCAT22(in_buf_len_5, param_1);
}
