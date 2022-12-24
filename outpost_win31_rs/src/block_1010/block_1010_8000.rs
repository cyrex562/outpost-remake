use crate::sys_ops;

pub unsafe fn pass1_1010_8096(param_1: u32, mut param_2: i16) {
    let mut uVar1: u16;
    let mut uVar2: u16;
    let mut uVar3: u16;
    let mut uVar4: u16;
    let mut pcVar5: *mut c_char;
    let mut puVar6: *mut u16;
    let mut local_306: [u8; 0x100] = [0; 0x100];
    let mut local_206: [u8; 0x100] = [0; 0x100];
    let mut local_106: [u8; 0x104] = [0; 0x104];

    uVar4 = (param_1 >> 0x10);
    uVar3 = param_1;
    str_1000_4d58(
        *((uVar3 + 0xe82) * 0x4 + 0x2526),
        NULL,
        0x0,
        CONCAT22(0x1050, local_206),
        CONCAT22(0x1050, local_306),
    );
    unk_str_op_1000_3d3e(CONCAT22(0x1050, local_106), CONCAT22(0x1050, local_206));
    if (param_2 == 0x2) {
        puVar6 = &u16_1050_3194;
    } else {
        puVar6 = &u16_1050_3196;
    }
    pass1_1000_3cea(CONCAT22(0x1050, local_106), puVar6);
    pass1_1000_3cea(CONCAT22(0x1050, local_106), CONCAT22(0x1050, local_306));
    pcVar5 = sys_ops::set_err_mode_1010_8b14(param_1, CONCAT22(0x1050, local_106));
    uVar2 = (pcVar5 >> 0x10);
    if ((pcVar5 == local_106) && (uVar2 == &DAT_1050_1050)) {
        msg_box_op_1010_8bb4(uVar3, uVar4, pcVar5 & 0xffff | 0x10500000);
    }
    fn_ptr_1000_17ce(*param_1);
    uVar1 = str_op_1008_60e8(uVar2, pcVar5);
    param_1 = uVar1;
    (uVar3 + 0x2) = uVar2;
    return;
}
pub unsafe fn pass1_1010_8170(param_1: *mut u8, param_2: *mut astruct_87, mut param_3: i16) {
    let mut iVar1: i16;
    let mut iVar2: i16;
    let mut iVar3: i16;
    let mut uVar4: u16;

    uVar4 = (param_2 >> 0x10);
    iVar2 = param_2;
    iVar1 = (iVar2 + 0x680);
    iVar3 = param_3 * 0x10;
    if ((iVar3 + 0x16) != iVar1) {
        pass1_1010_8096(param_2, (iVar3 + 0x16));
        pass1_1010_878c(param_2, (iVar3 + 0x16));
        if ((iVar2 + 0x67c) == 0) {
            return;
        }
    }
    iVar3 = param_3 * 0x10;
    pass1_1008_6562(
        CONCAT22(iVar1, param_1),
        (iVar2 + 0x67c),
        CONCAT22((iVar3 + 0x1c), (iVar3 + 0x1e)),
        (iVar3 + 0x1a),
    );
    return;
}




pub unsafe fn FUN_1010_830a(
    mut param_1: u16,
    mut param_2: u32,
    mut param_3: u16,
    param_4: *mut astruct_87,
    mut param_5: i16,
) -> u16 {
    let mut uVar2: u16;
    let mut puVar2: *mut astruct_81;
    let mut paVar3: *mut astruct_81;
    let mut uVar4: u16;
    let mut paVar5: *mut Struct57;
    let mut iVar6: i16;
    let mut unaff_SS: u16;
    let mut unaff_DS: u16;
    let mut pcVar7: *mut c_char;
    let mut uVar8: u32;
    let mut local_2e: astruct_81;
    let mut iStack10: i16;
    let mut iStack8: i16;
    let mut uStack6: u32;
    let mut uVar1: u16;
    let mut uVar9: *mut astruct_87;
    let mut uVar10: *mut astruct_87;

    uStack6 = 0;
    uVar1 = 0x63f0;
    iVar6 = param_5 * 0x10;
    uVar9 = param_4;
    uVar10 = (param_4 >> 0x10);
    if ((iVar6 + 0x10) == 1) {
        pcVar7 = sys_ops::set_err_mode_1010_8b14(param_4, (iVar6 + 0x12));
        paVar5 = (param_2 & 0xffff0000 | pcVar7 >> 0x10);
        iStack10 = pcVar7;
        iStack8 = (pcVar7 >> 0x10);
        uVar2 = 0x63f0;
        if (((iVar6 + 0x12) == iStack10) && ((iVar6 + 0x14) == iStack8)) {
            msg_box_op_1010_8bb4(uVar9, uVar10, pcVar7);
            return 0x0;
        }
        puVar2 = &local_2e;
        struct_op_1008_48fe(paVar5, CONCAT22(unaff_SS, puVar2), 0x1, pcVar7);
        mem_op_1000_179c(0x1e, paVar5);
        uVar4 = paVar5 | puVar2;
        if (uVar4 == 0) {
            paVar3 = null_mut();
            uVar4 = 0;
        } else {
            paVar3 = &local_2e;
            struct_op_1008_3f92(CONCAT22(paVar5, puVar2), CONCAT22(unaff_SS, paVar3));
        }
        uStack6 = CONCAT22(uVar4, paVar3);
        close_file_1008_496c(CONCAT22(unaff_SS, &local_2e));
    } else {
        if ((param_5 * 0x10 + 0x10) == 0x2) {
            pass1_1010_878c(param_4, (param_5 * 0x10 + 0x16));
            if (&uVar9.field1660_0x67c == 0) {
                return 0x0;
            }
            uVar2 = 0x63f0;
            iVar6 = param_5 * 0x10;
            pass1_1008_6562(
                (param_2 & 0xffff | param_1 << 0x10),
                &uVar9.field1660_0x67c,
                CONCAT22((iVar6 + 0x1c), (iVar6 + 0x1e)),
                (iVar6 + 0x1a),
            );
        } else {
            iVar6 = param_5 * 0x10;
            if ((iVar6 + 0x10) == 0x3) {
                uVar8 = sys_ops::set_err_mode_1010_8b14(param_4, (iVar6 + 0x12));
                local_2e.field1_0x2 = (uVar8 >> 0x10);
                param_1 = uVar8;
                uVar2 = 0x63f0;
                if (((iVar6 + 0x12) == param_1) && ((iVar6 + 0x14) == local_2e.field1_0x2)) {
                    local_2e.field0_0x0 = param_1;
                    msg_box_op_1010_8bb4(uVar9, uVar10, uVar8);
                    param_1 = local_2e.field0_0x0;
                }
            } else {
                //        if ((param_5 * 0x10 + 0x10) != 0x4) goto LAB_1010_8473;
                uVar8 = sys_ops::set_err_mode_1010_8b14(param_4, (param_5 * 0x10 + 0x12));
                param_1 = uVar8;
            }
        }
        uStack6 = param_1;
    } //
      // LAB_1010_8473:
    return uStack6;
}

pub unsafe fn load_string_1010_847e(mut param_1: u32, mut param_2: u16) -> *mut c_char {
    LoadString16(
        0x3ff,
        (param_1 & 0xffff0000 | (param_1 + 0x682)),
        param_2,
        HINSTANCE16_1050_038c,
    );
    return (param_1 & 0xffff0000 | (param_1 + 0x682));
}
pub unsafe fn load_string_1010_84ac(mut param_1: i16, param_2: INT16, mut param_3: u16) {
    let mut uVar1: u16;

    uVar1 = param_2;
    LoadString16(
        0x3ff,
        CONCAT22(param_2, param_1 + 0x682),
        param_3,
        HINSTANCE16_1050_038c,
    );
    str_op_1008_60e8(uVar1, CONCAT22(param_2, param_1 + 0x682));
    return;
}
pub unsafe fn load_string_1010_84e0(
    mut param_1: u16,
    mut param_2: u16,
    in_resc_id_3: u16,
    in_buffer_4: *mut c_char,
    in_buf_len_5: i16,
) {
    let mut in_stack_0000000e: u16;

    LoadString16(
        in_resc_id_3,
        CONCAT22(in_buf_len_5, in_buffer_4),
        in_stack_0000000e,
        HINSTANCE16_1050_038c,
    );
    return;
}








pub unsafe fn pass1_1010_878c(param_1: *mut *mut astruct_87, mut param_2: i16) {
    let mut uVar4: u16;
    let mut uVar1: u16;
    let mut puVar2: *mut u8;
    let mut in_EDX: u32;
    let mut paVar3: *mut Struct57;
    let mut uVar6: *mut astruct_87;
    let mut iVar4: i16;
    let mut uVar5: u16;
    let mut paVar6: *mut astruct_87;
    let mut pcStack6: *mut c_char;

    uVar5 = (param_1 >> 0x10);
    uVar6 = param_1;
    if (uVar6.field1662_0x680 == param_2) {
        return;
    }
    uVar4 = uVar6.field1660_0x67c;
    puVar2 = uVar6.field1661_0x67e;
    pcStack6 = CONCAT22(puVar2, uVar4);
    paVar3 = (in_EDX & 0xffff0000 | (puVar2 | uVar4));
    if ((puVar2 | uVar4) != 0) {
        pass1_1008_64a2(CONCAT22(puVar2, uVar4));
        fn_ptr_1000_17ce(pcStack6);
    }
    if ((param_2 == 1) || (param_2 == 0x2)) {
        mem_op_1000_179c(0x8, paVar3);
        uVar1 = paVar3;
        paVar3 = (paVar3 & 0xffff0000 | (uVar1 | uVar4));
        if ((uVar1 | uVar4) == 0) {
            uVar6.field1660_0x67c = 0;
            // TODO: goto LAB_1010_8869;
        }
        paVar6 = *param_1; //
                           // LAB_1010_8853:
        file_1008_6414(paVar3, CONCAT22(uVar1, uVar4), paVar6);
        puVar2 = paVar3;
    } else {
        iVar4 = param_2 * 0x4;
        paVar6 = sys_ops::set_err_mode_1010_8b14(param_1, (iVar4 + 0x172a));
        paVar3 = (paVar3 & 0xffff0000 | paVar6 >> 0x10);
        uVar4 = paVar6;
        uVar1 = (paVar6 >> 0x10);
        if (((iVar4 + 0x172a) == uVar4) && ((iVar4 + 0x172c) == uVar1)) {
            msg_box_op_1010_8bb4(uVar6, uVar5, paVar6 & 0xffff | uVar1 << 0x10);
        }
        mem_op_1000_179c(0x8, paVar3);
        uVar1 = paVar3;
        paVar3 = (paVar3 & 0xffff0000 | (uVar1 | uVar4));
        //    if ((uVar1 | uVar4) != 0) goto LAB_1010_8853;
        uVar4 = 0;
        puVar2 = null_mut();
    }
    uVar6.field1660_0x67c = uVar4;
    uVar6.field1661_0x67e = puVar2; //
                                    // LAB_1010_8869:
    uVar6.field1662_0x680 = param_2;
    return;
}


pub unsafe fn msg_box_op_1010_8bb4(mut param_1: u16, mut param_2: u16, mut param_3: u32) {
    let mut pcVar1: *mut c_char;
    let mut local_402: [u8; 0x400] = [0; 0x400];

    pcVar1 = load_string_1010_847e(_u16_1050_14cc, 0x3fa);
    unk_str_op_1000_3d3e(CONCAT22(0x1050, local_402), pcVar1);
    pass1_1000_3cea(CONCAT22(0x1050, local_402), param_3);
    pcVar1 = load_string_1010_847e(_u16_1050_14cc, 0x57b);
    MessageBox16(
        0x1010,
        pcVar1,
        CONCAT22(0x1050, local_402),
        HWND16_1050_0396,
    );
    PostMessage16(0x0, 0xee, 0x111, HWND16_1050_0396);
    return;
}



pub unsafe fn pass1_1010_8c32(param_1: *mut Struct19, mut param_2: u16) -> u32 {
    let mut in_EDX: u32;
    let mut uVar1: u16;
    let mut unaff_BP: u16;
    let mut paVar2: *mut Struct19;
    let mut puVar3: *mut u32;
    let mut in_stack_0000fea6: u16;
    let mut in_stack_0000ffca: u16;
    let mut in_stack_0000ffd0: u16;
    let mut in_stack_0000ffd4: u16;

    uVar1 = (in_EDX >> 0x10);
    paVar2 = struct_op_1010_1d48(param_1, param_2);
    (param_1 + 0xa) = 0;
    //        1010:8ee2  bc  8e  10  10      addr         pass1_1010_8ebc
    param_1.offset_0x0 = 0x8ee2;
    (param_1 + 0x2) = 0x1010;
    puVar3 = mixed_1010_20ba(
        CONCAT22(uVar1, (paVar2 >> 0x10)),
        _u16_1050_0ed0,
        CONCAT22(unaff_BP, 0x3),
        in_stack_0000fea6,
        in_stack_0000ffca,
        in_stack_0000ffd0,
        in_stack_0000ffd4,
    );
    (param_1 + 0xa) = puVar3;
    (param_1 + 0xc) = (puVar3 >> 0x10);
    return param_1;
}







pub unsafe fn pass1_1010_8ef2(
    param_1: *mut Struct57,
    param_2: *mut astruct_170,
    mut param_3: u16,
    mut param_4: u16,
    mut param_5: u16,
    mut param_6: u16,
    mut param_7: u16,
) {
    let mut uVar1: u16;
    let mut uVar2: u16;
    let mut iVar3: *mut astruct_170;
    let mut uVar4: u16;
    let mut puVar5: *mut u32;
    let mut paVar3: *mut Struct57;

    uVar4 = (param_2 >> 0x10);
    iVar3 = param_2;
    param_2.field0_0x0 = 0x389a;
    iVar3.field1_0x2 = 0x1008;
    uVar1 = 0;
    iVar3.field2_0x4 = 0;
    iVar3.field4_0x8 = 0;
    param_2.field0_0x0 = 0x9254;
    iVar3.field1_0x2 = 0x1010;
    mem_op_1000_179c(0x18, param_1);
    uVar2 = param_1 | uVar1;
    paVar3 = (param_1 & 0xffff0000 | uVar2);
    if (uVar2 == 0) {
        iVar3.field2_0x4 = 0;
    } else {
        struct_op_1030_1cd8(CONCAT22(param_1, uVar1), 0x5, 0x5);
        iVar3.field2_0x4 = uVar1;
        iVar3.field3_0x6 = paVar3;
    }
    puVar5 = mixed_1010_20ba(
        paVar3,
        _u16_1050_0ed0,
        CONCAT22(param_3, 0x3),
        param_4,
        param_5,
        param_6,
        param_7,
    );
    iVar3.field4_0x8 = puVar5;
    iVar3.field5_0xa = (puVar5 >> 0x10);
    return;
}
