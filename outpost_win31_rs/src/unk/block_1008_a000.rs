pub unsafe fn pass1_1008_a086(param_1: *mut astruct_455) {
    let mut puVar1: *mut u32;
    let mut puVar2: *mut u32;
    let mut uVar3: u16;
    let mut ppcVar4: *mut *mut code;
    let mut iVar4: *mut astruct_455;
    let mut uVar4: *mut astruct_455;

    uVar4 = (param_1 >> 0x10);
    iVar4 = param_1;
    param_1.field0_0x0 = 0xad92;
    iVar4.field1_0x2 = 0x1008;
    puVar1 = iVar4[0x1].field1_0x2;
    puVar2 = iVar4[0x1].field2_0x4;
    if ((puVar2 | puVar1) != 0) {
        ppcVar4 = *puVar1;
        (**ppcVar4)();
    }
    puVar1 = (iVar4 + 0x82).field0_0x0;
    uVar3 = iVar4[0x82].field1_0x2;
    if ((uVar3 | puVar1) != 0) {
        ppcVar4 = *puVar1;
        (**ppcVar4)();
    }
    pass1_1010_1d80(param_1);
    return;
}


pub unsafe fn pass1_1008_a1f0(
    param_1: u8,
    mut param_2: u32,
    param_3: *mut u16,
    param_4: *mut u16,
    param_5: *mut u16,
    param_6: *mut u16,
) {
    let mut ppcVar1: *mut *mut code;
    let mut uVar2: u16;
    let mut uVar3: u16;
    let mut uVar4: u16;
    let mut in_EDX: u32;
    let mut uVar5: u32;
    let mut paVar6: *mut Struct57;
    let mut uVar7: u16;
    let mut iVar8: i16;
    let mut in_buf_len_5: u16;
    let mut uVar9: u16;
    let mut uVar10: u16;
    let mut paVar11: *mut Struct27;
    let mut pcVar12: *mut c_char;
    let mut in_stack_0000fd8a: u16;
    let mut in_stack_0000feae: u16;
    let mut in_stack_0000feb4: u16;
    let mut in_stack_0000feb8: u16;
    let mut uVar13: u16;
    let mut uVar14: u8;
    let mut uVar15: u8;
    let mut uVar16: u32;
    let mut local_106: [u8; 0x100] = [0; 0x100];
    let mut puStack6: *mut u32;

    uVar2 = 0;
    *param_6 = 0;
    *param_5 = 0;
    *param_4 = 0;
    *param_3 = 0;
    in_buf_len_5 = (param_2 >> 0x10);
    uVar7 = param_2;
    *(uVar7 + 0xe) = 0;
    uVar16 = (uVar7 + 0xa);
    ppcVar1 = ((uVar7 + 0xa) + 0x10);
    (**ppcVar1)();
    uVar4 = in_EDX;
    puStack6 = CONCAT22(uVar4, uVar2);
    uVar5 = in_EDX & 0xffff0000 | (uVar4 | uVar2);
    if ((uVar4 | uVar2) == 0) {
        return;
    }
    *param_6 = (uVar2 + 0x4);
    *param_4 = (uVar2 + 0xa);
    uVar3 = pass1_1008_ab80(uVar7, in_buf_len_5, *param_6);
    *param_3 = uVar3;
    uVar9 = (puStack6 >> 0x10);
    iVar8 = puStack6;
    uVar10 = 0x1008;
    uVar13 = _u16_1050_14cc;
    uVar3 = (_u16_1050_14cc >> 0x10);
    //   switch((iVar8 + 0x4)) {
    match (iVar8 + 0x4) {
        // case 0x1:
        1 => {
            load_string_1010_84e0(uVar13, uVar3, 0x3ff, (uVar7 + 0xe), in_buf_len_5);
            *param_5 = 0xd1;
        }
        // TODO: goto LAB_1008_a2b1;
        //   case 0x2:
        2 => {
            pass1_1028_e1ec(_PTR_LOOP_1050_65e2, (iVar8 + 0x6));
            load_string_1010_84e0(
                _u16_1050_14cc,
                (_u16_1050_14cc >> 0x10),
                0x100,
                local_106,
                0x1050,
            );
            pcVar12 = pass1_1038_4d28(CONCAT13((uVar5 >> 0x8), CONCAT12(uVar5, iVar8)));
            uVar10 = 0x1000;
            sys_1000_3f9c(
                (param_2 & 0xffff0000 | (uVar7 + 0xe)),
                CONCAT22(0x1050, local_106),
                pcVar12,
            );
        }
        // break;
        //   case 0x5:
        5 => {}
        // TODO: goto LAB_1008_a277;
        //   case 0x6:
        6 => {
            load_string_1010_84e0(uVar13, uVar3, 0x3ff, (uVar7 + 0xe), in_buf_len_5);
            *param_5 = 0xd4; //
                             // LAB_1008_a2b1:
            uVar10 = 0x1010;
            *param_4 = 0x1;
        }
        // break;
        //   case 0x7://
        7 => {
            // LAB_1008_a277:
            uVar10 = 0x1010;
            load_string_1010_84e0(uVar13, uVar3, 0x3ff, (uVar7 + 0xe), in_buf_len_5);
        }
        // break;
        //   case 0x9:
        9 => {
            if ((uVar7 + 0x416) == 0) {
                (uVar7 + 0x416) = 0x1;
            }
            //   break;
        }
        // TODO: goto LAB_1008_a35a;
        //   case 0xb:
        0xb => {
            if ((uVar7 + 0x41a) == 0) {
                (uVar7 + 0x41a) = 0x1;
                //   break;
            }
        }
        // TODO: goto LAB_1008_a35a;
        //   case 0xe:
        0xe => {
            if ((uVar7 + 0x41c) == 0) {
                (uVar7 + 0x41c) = 0x1;
                //   break;
            }
        }
        // TODO: goto LAB_1008_a35a;
        //   case 0x14:
        0x14 => {
            if ((uVar7 + 0x418) == 0) {
                (uVar7 + 0x418) = 0x1;
                load_string_1010_84e0(
                    _u16_1050_14cc,
                    (_u16_1050_14cc >> 0x10),
                    0x3ff,
                    (uVar7 + 0xe),
                    in_buf_len_5,
                );
                uVar9 = (uVar5 >> 0x10);
                pcVar12 = load_string_1010_847e(_u16_1050_14cc, 0x72b);
                paVar6 = CONCAT22(uVar9, (pcVar12 >> 0x10));
                pass1_1000_3cea(param_2 & 0xffff0000 | ZEXT24((uVar7 + 0xe)), pcVar12);
                *param_5 = 0x4c;
                uVar14 = 0x1;
                uVar15 = 0;
                iVar8 = 0xa;
                paVar11 = mixed_1010_20ba(
                    paVar6,
                    _u16_1050_0ed0,
                    0x1002b,
                    in_stack_0000fd8a,
                    in_stack_0000feae,
                    in_stack_0000feb4,
                    in_stack_0000feb8,
                );
                uVar10 = 0x1010;
                pass1_1010_089e(paVar11, CONCAT11(uVar15, uVar14), iVar8);
                //   break;
            }
        }
        // TODO: goto LAB_1008_a35a;
        //   case 0x16:
        0x16 => {
            uVar10 = 0x1010;
            load_string_1010_84e0(uVar13, uVar3, 0x3ff, (uVar7 + 0xe), in_buf_len_5);
            *param_5 = 0x28;
        }
        // break;
        //   case 0x17:
        0x17 => {
            uVar10 = 0x1010;
            load_string_1010_84e0(uVar13, uVar3, 0x3ff, (uVar7 + 0xe), in_buf_len_5);
            *param_5 = 0x2c;
        }
        // break;
        //   case 0x18:
        0x18 => {
            uVar10 = 0x1010;
            load_string_1010_84e0(uVar13, uVar3, 0x3ff, (uVar7 + 0xe), in_buf_len_5);
            *param_5 = 0x2e;
        }
        // break;
        //   case 0x1b:
        0x1b => {
            uVar10 = 0x1010;
            load_string_1010_84e0(uVar13, uVar3, 0x3ff, (uVar7 + 0xe), in_buf_len_5);
            *param_5 = 0x30;
        }
        // break;
        //   case 0x1c:
        0x1c => {
            uVar10 = 0x1010;
            load_string_1010_84e0(uVar13, uVar3, 0x3ff, (uVar7 + 0xe), in_buf_len_5);
            *param_5 = 0x32;
        }
        // break;
        //   case 0x1f:
        0x1f => {
            uVar10 = 0x1010;
            load_string_1010_84e0(uVar13, uVar3, 0x3ff, (uVar7 + 0xe), in_buf_len_5);
            *param_5 = 0x34;
        }
        // break;
        //   case 0x21:
        0x21 => {
            if ((uVar7 + 0x41e) == 0) {
                (uVar7 + 0x41e) = 0x1;
            //   break;
            } else {
                // LAB_1008_a35a:
                *param_3 = 0;
            }
        }
        // break;
        //   case 0x24:
        0x24 => {
            uVar10 = 0x1010;
            load_string_1010_84e0(uVar13, uVar3, 0x3ff, (uVar7 + 0xe), in_buf_len_5);
            *param_5 = 0x2a;
        }
        // break;
        //   case 0x31:
        0x31 => {
            uVar10 = 0x1010;
            load_string_1010_84e0(uVar13, uVar3, 0x3ff, (uVar7 + 0xe), in_buf_len_5);
            *param_5 = 0x27;
        }
        // break;
        //   case 0x32:
        0x32 => {
            uVar10 = 0x1010;
            load_string_1010_84e0(uVar13, uVar3, 0x3ff, (uVar7 + 0xe), in_buf_len_5);
            *param_5 = 0x29;
        }
        // break;
        //   case 0x33:
        0x33 => {
            uVar10 = 0x1010;
            load_string_1010_84e0(uVar13, uVar3, 0x3ff, (uVar7 + 0xe), in_buf_len_5);
            *param_5 = 0x2b;
        }
        // break;
        //   case 0x34:
        0x34 => {
            uVar10 = 0x1010;
            load_string_1010_84e0(uVar13, uVar3, 0x3ff, (uVar7 + 0xe), in_buf_len_5);
            *param_5 = 0x2d;
        }
        // break;
        //   case 0x35:
        0x35 => {
            uVar10 = 0x1010;
            load_string_1010_84e0(uVar13, uVar3, 0x3ff, (uVar7 + 0xe), in_buf_len_5);
            *param_5 = 0x2f;
        }
        // break;
        //   case 0x36:
        0x36 => {
            uVar10 = 0x1010;
            load_string_1010_84e0(uVar13, uVar3, 0x3ff, (uVar7 + 0xe), in_buf_len_5);
            *param_5 = 0x31;
        }
        // break;
        //   case 0x37:
        0x37 => {
            load_string_1010_84e0(uVar13, uVar3, 0x3ff, (uVar7 + 0xe), in_buf_len_5);
            pcVar12 = load_string_1010_847e(_u16_1050_14cc, 0x71f);
            uVar10 = 0x1000;
            pass1_1000_3cea(param_2 & 0xffff0000 | ZEXT24((uVar7 + 0xe)), pcVar12);
            *param_5 = 0x33;
        }
        // break;
        //   case 0x38:
        0x38 => {
            uVar10 = 0x1010;
            load_string_1010_84e0(uVar13, uVar3, 0x3ff, (uVar7 + 0xe), in_buf_len_5);
            *param_5 = 0x35;
        }
        // break;
        //   case 0x39:
        0x39 => {
            uVar10 = 0x1010;
            load_string_1010_84e0(uVar13, uVar3, 0x3ff, (uVar7 + 0xe), in_buf_len_5);
            *param_5 = 0x36;
        }
        // break;
        //   case 0x3a:
        0x3a => {
            uVar10 = 0x1010;
            load_string_1010_84e0(uVar13, uVar3, 0x3ff, (uVar7 + 0xe), in_buf_len_5);
            *param_5 = 0x37;
        }
        // break;
        //   case 0x3b:
        0x3b => {
            uVar10 = 0x1010;
            load_string_1010_84e0(uVar13, uVar3, 0x3ff, (uVar7 + 0xe), in_buf_len_5);
            *param_5 = 0x38;
        }
        // break;
        //   case 0x3c:
        0x3c => {
            uVar10 = 0x1010;
            load_string_1010_84e0(uVar13, uVar3, 0x3ff, (uVar7 + 0xe), in_buf_len_5);
            *param_5 = 0x39;
        }
        // break;
        //   case 0x3d:
        0x3d => {
            uVar10 = 0x1010;
            load_string_1010_84e0(uVar13, uVar3, 0x3ff, (uVar7 + 0xe), in_buf_len_5);
            *param_5 = 0xce;
        }
        // break;
        //   case 0x3e:
        0x3e => {
            uVar10 = 0x1010;
            load_string_1010_84e0(uVar13, uVar3, 0x3ff, (uVar7 + 0xe), in_buf_len_5);
            *param_5 = 0xcf;
        }
        // break;
        //   case 0x3f:
        0x3f => {
            uVar10 = 0x1010;
            load_string_1010_84e0(uVar13, uVar3, 0x3ff, (uVar7 + 0xe), in_buf_len_5);
            *param_5 = 0xd0;
        }
        // break;
        //   case 0x40:
        0x40 => {
            uVar10 = 0x1010;
            load_string_1010_84e0(uVar13, uVar3, 0x3ff, (uVar7 + 0xe), in_buf_len_5);
            *param_5 = 0xd1;
        }
        // break;
        //   case 0x41:
        0x41 => {
            uVar10 = 0x1010;
            load_string_1010_84e0(uVar13, uVar3, 0x3ff, (uVar7 + 0xe), in_buf_len_5);
            *param_5 = 0xd2;
        }
        // break;
        //   case 0x42:
        0x42 => {
            uVar10 = 0x1010;
            load_string_1010_84e0(uVar13, uVar3, 0x3ff, (uVar7 + 0xe), in_buf_len_5);
            *param_5 = 0xd3;
        }
        // break;
        //   case 0x43:
        0x43 => {
            uVar10 = 0x1010;
            load_string_1010_84e0(uVar13, uVar3, 0x3ff, (uVar7 + 0xe), in_buf_len_5);
            *param_5 = 0xd5;
        }
        // break;
        //   case 0x44:
        0x44 => {
            uVar10 = 0x1010;
            load_string_1010_84e0(uVar13, uVar3, 0x3ff, (uVar7 + 0xe), in_buf_len_5);
            *param_5 = 0xd6;
        }
        // break;
        //   case 0x45:
        0x45 => {
            uVar10 = 0x1010;
            load_string_1010_84e0(uVar13, uVar3, 0x3ff, (uVar7 + 0xe), in_buf_len_5);
            *param_5 = 0xd7;
        }
    };
    if (puStack6.is_null() == false) {
        ppcVar1 = *puStack6;
        (**ppcVar1)(uVar10, puStack6, (puStack6 >> 0x10), 0x1, uVar16);
    }
    return;
}

pub unsafe fn pass1_1008_a8f4(
    mut param_1: u16,
    mut param_2: u32,
    param_3: *mut u16,
    param_4: *mut u16,
    param_5: *mut u16,
) -> u32 {
    let mut iVar1: i16;
    let mut in_AF: u8;
    let mut local_6: u32;

    iVar1 = &local_6 + 2;
    pass1_1008_a1f0(
        in_AF,
        param_2,
        param_3,
        CONCAT22(0x1050, &local_6),
        CONCAT22(0x1050, iVar1),
        param_5,
    );
    pass1_1008_944e(param_4, local_6, (local_6 >> 0x10));
    return CONCAT22(param_1, iVar1);
}
pub unsafe fn pass1_1008_a930(mut param_1: u32, mut param_2: i16) {
    let mut ppcVar1: *mut *mut code;
    let mut puVar2: *mut u8;
    let mut uVar3: u16;
    let mut uVar4: u16;
    let mut in_EDX: *mut Struct57;
    let mut iVar5: i16;
    let mut uVar6: u16;
    let mut puStack24: *mut u16;
    let mut puStack18: *mut u16;
    let mut local_a: [u8; 0x8] = [0; 0x8];

    if (param_2 == 0) {
        return;
    }
    uVar6 = (param_1 >> 0x10);
    iVar5 = param_1;
    pass1_1008_5784(CONCAT22(0x1050, local_a), (iVar5 + 0x410));
    loop {
        puVar2 = local_a;
        pass1_1008_5b12(CONCAT22(0x1050, puVar2));
        uVar3 = in_EDX;
        uVar4 = uVar3 | puVar2;
        in_EDX = (in_EDX & 0xffff0000 | uVar4);
        if (uVar4 == 0) {
            mem_op_1000_179c(0x6, in_EDX);
            uVar3 = in_EDX;
            puStack24 = CONCAT22(uVar3, puVar2);
            if ((uVar3 | puVar2) == 0) {
                puStack18 = null_mut();
            } else {
                *puStack24 = 0x389a;
                (puVar2 + 0x2) = 0x1008;
                (puVar2 + 0x4) = param_2;
                *puStack24 = 0xad8a;
                (puVar2 + 0x2) = 0x1008;
                puStack18 = puStack24;
            }
            ppcVar1 = ((iVar5 + 0x410) + 0x8);
            (**ppcVar1)(0x1000, (iVar5 + 0x410), puStack18);
            return;
        }
        if (puVar2 + 0x4) == param_2 {
            break;
        }
    }
    return;
}

pub unsafe fn pass1_1008_a9ec(mut param_1: u32) -> u16 {
    let mut uVar1: u32;
    let mut in_AX: u16;
    let mut iVar2: i16;
    let mut uVar3: u16;
    let mut uStack4: u16;

    uStack4 = 0;
    uVar3 = (param_1 >> 0x10);
    iVar2 = param_1;
    if (((iVar2 + 0x414) == 0) && (uVar1 = (iVar2 + 0x410), (uVar1 + 0x8) != 0)) {
        (iVar2 + 0x414) = 0x1;
        pass1_1008_aa28(in_AX, param_1 & 0xffff | uVar3 << 0x10);
        uStack4 = in_AX;
    }
    return uStack4;
}


pub unsafe fn pass1_1008_aa28(mut param_1: u16, mut param_2: u32) {
    let mut ppcVar1: *mut *mut code;
    let mut uVar2: u32;
    let mut extraout_DX: u16;
    let mut iVar3: i16;
    let mut uVar4: u16;
    let mut puStack6: *mut u32;

    uVar4 = (param_2 >> 0x10);
    iVar3 = param_2;
    if ((iVar3 + 0x414) != 0) {
        uVar2 = (iVar3 + 0x410);
        if ((uVar2 + 0x8) == 0) {
            (iVar3 + 0x414) = 0;
            return;
        }
        ppcVar1 = ((iVar3 + 0x410) + 0x10);
        (**ppcVar1)();
        puStack6 = CONCAT22(extraout_DX, param_1);
        if ((extraout_DX | param_1) != 0) {
            win_1008_5c5c(
                param_1,
                extraout_DX | param_1,
                _u16_1050_02a0,
                (param_1 + 0x4),
            );
            if (puStack6.is_null() == false) {
                ppcVar1 = *puStack6;
                (**ppcVar1)();
            }
            return;
        }
    }
    return;
}

pub unsafe fn pass1_1008_aaa8(mut param_1: u16, mut param_2: u16, mut param_3: u16) -> u16 {
    let mut u_stack4: u16;

    u_stack4 = 0;
    //   switch(param_3)
    match param_3 {
        //   case 0x1:
        1 => {
            u_stack4 = 0x24;
        }
        // break;
        //   case 0x2:
        2 => {
            u_stack4 = 0x16;
        }
        // break;
        //   case 0x3:
        3 => {
            u_stack4 = 0x17;
        }
        // break;
        //   case 0x4:
        4 => {
            u_stack4 = 0x18;
        }
        // break;
        //   case 0x5:
        5 => {
            u_stack4 = 0x1b;
        }
        // break;
        //   case 0x6:
        6 => {
            u_stack4 = 0x1c;
        }
        // break;
        //   case 0x7:
        7 => {
            u_stack4 = 0x1f;
        }
    };
    return u_stack4;
}

pub unsafe fn pass1_1008_ab12(mut param_1: u16, mut param_2: u16, mut param_3: u16) -> u16 {
    if (param_3 == 0x37) {
        return 0x22;
    }
    if (param_3 < 0x38) {
        if (param_3 == '\r') {
            return 0xf;
        }
        if (param_3 == '*') {
            return 0x2b;
        }
    }
    return 0x0;
}

pub unsafe fn pass1_1008_ab54(mut param_1: u32) -> u16 {
    let mut uVar1: u32;
    let mut uVar2: u16;
    let mut uStack4: u16;

    uStack4 = 0;
    uVar2 = (param_1 >> 0x10);
    if (((param_1 + 0xa) != 0) && (uVar1 = (param_1 + 0xa), (uVar1 + 0x8) != 0)) {
        uStack4 = 0x1;
    }
    return uStack4;
}

pub unsafe fn pass1_1008_ab80(mut param_1: u16, mut param_2: u16, mut param_3: u16) -> u16 {
    let mut uStack4: u16;

    uStack4 = 0;
    //   switch(param_3)
    match param_3 {
        //   case 0x8:
        8 => {
            uStack4 = 0x82;
        }
        // break;
        //   case 0x9:
        9 => {
            uStack4 = 0x7f;
        }
        // break;
        //   case 0xa:
        0xa => {
            uStack4 = 0x80;
        }
        // break;
        //   case 0xb:
        0xb => {
            uStack4 = 0x84;
        }
        // break;
        //   case 0xc:
        0xc => {
            uStack4 = 0x89;
        }
        // break;
        //   case 0xd:
        0xd => {
            uStack4 = 0x8a;
        }
        // break;
        //   case 0xe:
        0xe => {
            uStack4 = 0x8c;
        }
        // break;
        //   case 0xf:
        0xf => {
            uStack4 = 0x8e;
        }
        // break;
        //   case 0x10:
        0x10 => {
            uStack4 = 0x8f;
        }
        // break;
        //   case 0x11:
        0x11 => {
            uStack4 = 0x90;
        }
        // break;
        //   case 0x12:
        0x12 => {
            uStack4 = 0x91;
        }
        // break;
        //   case 0x13:
        0x13 => {
            uStack4 = 0x95;
        }
        // break;
        //   case 0x14:
        0x14 => {
            uStack4 = 0x96;
        }
        // break;
        //   case 0x16:
        0x16 => {
            uStack4 = 0x9b;
        }
        // break;
        //   case 0x17:
        0x17 => {
            uStack4 = 0x9f;
        }
        // break;
        //   case 0x18:
        0x18 => {
            uStack4 = 0xa2;
        }
        // break;
        //   case 0x19:
        0x19 => {
            uStack4 = 0xa4;
        }
        // break;
        //   case 0x1b:
        //   case 0x1c:
        0x1b | 0x1c => {
            uStack4 = 0xa7;
        }
        // break;
        //   case 0x1d:
        0x1d => {
            uStack4 = 0xaa;
        }
        // break;
        //   case 0x1e:
        0x1e => {
            uStack4 = 0xac;
        }
        // break;
        //   case 0x1f:
        0x1f => {
            uStack4 = 0xad;
        }
        // break;
        //   case 0x20:
        0x20 => {
            uStack4 = 0xae;
        }
        // break;
        //   case 0x21:
        0x1 => {
            uStack4 = 0xb1;
        }
        // break;
        //   case 0x22:
        0x22 => {
            uStack4 = 0xb3;
        }
        // break;
        //   case 0x23:
        0x23 => {
            uStack4 = 0xb4;
        }
        // break;
        //   case 0x24:
        0x24 => {
            uStack4 = 0xb5;
        }
        // break;
        //   case 0x25:
        0x25 => {
            uStack4 = 0xb6;
        }
        // break;
        //   case 0x26:
        0x26 => {
            uStack4 = 0xb7;
        }
        // break;
        //   case 0x27:
        0x27 => {
            uStack4 = 0xab;
        }
        // break;
        //   case 0x28:
        0x28 => {
            uStack4 = 0xb9;
        }
        // break;
        //   case 0x29:
        0x29 => {
            uStack4 = 0xba;
        }
        // break;
        //   case 0x2a:
        0x2a => {
            uStack4 = 0xbc;
        }
        // break;
        //   case 0x2b:
        0x2b => {
            uStack4 = 0xbe;
        }
        // break;
        //   case 0x2c:
        0x2c => {
            uStack4 = 0xdf;
        }
        // break;
        //   case 0x2d:
        0x2d => {
            uStack4 = 0xe0;
        }
    };
    return uStack4;
}







pub unsafe fn pass1_1008_ada2(param_1: *mut u16, mut param_2: i16) -> *mut u16 {
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    *param_1 = 0;
    (param_1 + 0x2) = 0;
    (param_1 + 0x4) = param_2;
    *param_1 = (param_2 * 0x6 + 0x3a4);
    return param_1;
}
pub unsafe fn pass1_1008_add2(param_1: *mut u16) {
    *param_1 = ((param_1 + 0x4) * 0x6 + 0x3a4);
    return;
}

pub unsafe fn pass1_1008_adf2(mut param_1: u32) -> u16 {
    return ((param_1 + 0x4) * 0x6 + 0x3a4);
}

pub unsafe fn pass1_1008_ae0c(mut param_1: u32) -> u16 {
    return ((param_1 + 0x4) * 0x6 + 0x3a6);
}
pub unsafe fn pass1_1008_ae26(param_1: *mut i16) {
    let mut piVar1: *mut i16;
    let mut iVar2: i16;
    let mut iVar3: i16;
    let mut uVar4: u16;

    uVar4 = (param_1 >> 0x10);
    iVar3 = param_1;
    iVar2 = ((iVar3 + 0x4) * 0x6 + 0x3a8);
    if (iVar2 == 0x2) {
        if ((iVar3 + 0x2) == 1) {
            *param_1 = *param_1 -0x1;
            iVar2 = (iVar3 + 0x4) * 0x6;
            piVar1 = (iVar2 + 0x3a4);
            if (*piVar1 != *param_1 && *param_1 <= *piVar1) {
                *param_1 = (iVar2 + 0x3a4) + 1;
                (iVar3 + 0x2) = 0;
                return;
            }
        } else {
            *param_1 = *param_1 + 1;
            iVar2 = (iVar3 + 0x4) * 0x6;
            if ((iVar2 + 0x3a6) < *param_1) {
                *param_1 = (iVar2 + 0x3a6) -0x1;
                (iVar3 + 0x2) = 0x1;
                return;
            }
        }
    } else if ((iVar2 != 0x3) && (iVar2 != 0x4)) {
        *param_1 = *param_1 + 1;
        iVar2 = (iVar3 + 0x4) * 0x6;
        if ((iVar2 + 0x3a6) < *param_1) {
            *param_1 = (iVar2 + 0x3a4);
        }
    }
    return;
}

pub unsafe fn pass1_1008_aed8(mut param_1: u32) -> BOOL16 {
    if (((param_1 + 0x4) * 0x6 + 0x3a4) != 0) {
        return 0x1;
    }
    return 0x0;
}



pub unsafe fn pass1_1008_aefe(param_1: *mut u8, param_2: *mut Struct19, mut param_3: u16) -> u32 {
    struct_op_1018_4cda(param_2, param_3);
    param_2.offset_0x0 = 0xaf7c;
    (param_2 + 0x2) = 0x1008;
    _PTR_LOOP_1050_4230 = param_2;
    pass1_1018_4dce(param_1, param_2, 0x1b3);
    return param_2;
}
pub unsafe fn pass1_1008_af38(param_1: *mut StructD) {
    param_1.address_offset_field_0x0 = 0xaf7c;
    (param_1 + 0x2) = 0x1008;
    clenaup_win_ui_1018_4d22(param_1);
    return;
}

pub unsafe fn pass1_1008_af94(param_1: *mut Struct19, mut param_2: u16, mut param_3: u16) {
    struct_op_1010_1d48(param_1, param_2);
    (param_1 + 0xa) = 0;
    (param_1 + 0xe) = 0;
    (param_1 + 0x12) = 0;
    (param_1 + 0x16) = 0;
    (param_1 + 0x1a) = 0;
    (param_1 + 0x1e) = 0;
    (param_1 + 0x22) = 0;
    param_1.offset_0x0 = 0xbdcc;
    (param_1 + 0x2) = 0x1008;
    return;
}
pub unsafe fn pass1_1008_afde(param_1: *mut astruct_455) {
    let mut puVar1: *mut u32;
    let mut puVar2: *mut u32;
    let mut uVar3: u16;
    let mut ppcVar4: *mut *mut code;
    let mut iVar4: *mut astruct_455;
    let mut uVar4: *mut astruct_455;

    uVar4 = (param_1 >> 0x10);
    iVar4 = param_1;
    param_1.field0_0x0 = 0xbdcc;
    iVar4.field1_0x2 = 0x1008;
    puVar1 = iVar4[0x1].field1_0x2;
    puVar2 = iVar4[0x1].field2_0x4;
    if ((puVar2 | puVar1) != 0) {
        ppcVar4 = *puVar1;
        (**ppcVar4)();
    }
    puVar1 = iVar4[0x1].field3_0x6;
    uVar3 = (iVar4 + 0x2).field0_0x0;
    if ((uVar3 | puVar1) != 0) {
        ppcVar4 = *puVar1;
        (**ppcVar4)();
    }
    puVar1 = iVar4[0x2].field1_0x2;
    puVar2 = iVar4[0x2].field2_0x4;
    if ((puVar2 | puVar1) != 0) {
        ppcVar4 = *puVar1;
        (**ppcVar4)();
    }
    pass1_1010_1d80(param_1);
    return;
}