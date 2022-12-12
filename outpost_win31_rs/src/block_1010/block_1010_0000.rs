pub unsafe fn pass1_1010_0000(param_1: *mut astruct_19, mut param_2: u16) -> *mut astruct_19 {
    let mut in_EDX: u32;
    let mut uVar1: u16;
    let mut paVar2: *mut astruct_19;
    let mut puVar3: *mut u32;
    let mut in_stack_0000fe86: u16;
    let mut in_stack_0000ffaa: u16;
    let mut in_stack_0000ffb0: u16;
    let mut in_stack_0000ffb4: u16;
    let mut iVar4: i16;
    let mut iVar5: i16;
    let mut uVar6: u16;

    // Segment:    3
    // Offset:     00015420
    // Length:     ee9f
    // Min Alloc:  ee9f
    // Flags:      0d50
    //     Code
    //     Moveable
    //     Preload
    //     Impure (Non-shareable)
    //
    uVar1 = (in_EDX >> 0x10);
    paVar2 = struct_op_1010_1d48(param_1, param_2);
    (param_1 + 0xa) = 0;
    (param_1 + 0xc) = 0;
    param_1.offset_0x0 = 0x2c8;
    (param_1 + 0x2) = 0x1010;
    iVar5 = param_1 + 0xa;
    iVar4 = param_1 + 0xc;
    uVar6 = param_1;
    puVar3 = mixed_1010_20ba(
        CONCAT22(uVar1, (paVar2 >> 0x10)),
        _u16_1050_0ed0,
        CONCAT22(iVar4, 0x48),
        in_stack_0000fe86,
        in_stack_0000ffaa,
        in_stack_0000ffb0,
        in_stack_0000ffb4,
    );
    pass1_1008_3e94(
        (puVar3 & 0xffff0000 | (puVar3 + 0xe)),
        CONCAT22(param_1, iVar4),
        CONCAT22(uVar6, iVar5),
    );
    return param_1;
}
pub unsafe fn pass1_1010_0052(param_1: *mut u16, mut param_2: u16) {
    *param_1 = 0x2c8;
    (param_1 + 0x2) = 0x1010;
    pass1_1010_1d80(param_1);
    return;
}
pub unsafe fn set_window_placement_1010_0070(mut param_1: u32, mut param_2: i16, mut param_3: u16) {
    let mut ppcVar1: *mut *mut code;
    let mut uVar2: u16;
    let mut puVar3: *mut u32;
    let mut lVar4: i32;
    let mut uVar5: u16;
    let mut local_18: [u8; 0x6] = [0; 0x6];
    let mut IStack18: i16;
    let mut iStack16: i16;
    let mut IStack14: i16;
    let mut IStack12: i16;
    let mut IStack10: i16;
    let mut IStack8: i16;
    let mut uStack6: u16;
    let mut uStack4: u16;

    local_18 = 0x16;
    local_18._2_4_ = 0;
    IStack18 = 0;
    iStack16 = 0;
    IStack14 = 0;
    IStack12 = 0;
    IStack10 = 0;
    IStack8 = 0;
    uStack6 = 0;
    uStack4 = 0;
    uVar5 = param_3;
    GetWindowPlacement16(local_18, &DAT_1050_1050);
    if ((iStack16 == -1) || (param_2 != 0)) {
        local_18._2_4_ = 0x50001;
        lVar4 = GetWindowLong16(0x0, param_3);
        uVar2 = (lVar4 >> 0x10);
        puVar3 = (lVar4 + 0xe0);
        ppcVar1 = (*puVar3 + 0x38);
        (**ppcVar1)(s_tile2_bmp_1050_1538, puVar3, (lVar4 + 0xe2), uVar5);
        pass1_1010_01f8(param_1, CONCAT22(0x1050, local_18), puVar3);
        SetWindowPlacement16(local_18, &DAT_1050_1050);
    }
    return;
}
pub unsafe fn set_win_placement_1010_010e(mut param_1: u16, mut param_2: u16, mut param_3: u16) {
    let mut ppcVar1: *mut *mut code;
    let mut iVar2: i16;
    let mut piVar3: *mut i16;
    let mut uVar4: u16;
    let mut puVar5: *mut u32;
    let mut extraout_DX: u16;
    let mut lVar6: i32;
    let mut uVar7: u16;
    let mut local_18: WINDOWPLACEMENT16;
    let mut iStack6: i16;
    let mut iStack4: i16;

    local_18.length = 0x16;
    local_18.flags = 0;
    local_18.show_cmd = 0;
    local_18.pt_min_position.x = 0;
    local_18.pt_min_position.y = 0;
    local_18.pt_max_position.x = 0;
    local_18.pt_max_position.y = 0;
    local_18.rc_normal_position.x = 0;
    local_18.rc_normal_position.y = 0;
    iStack6 = 0;
    iStack4 = 0;
    uVar7 = param_3;
    GetWindowPlacement16(&local_18, &DAT_1050_1050);
    if (local_18.rc_normal_position.x == -1) {
        lVar6 = GetWindowLong16(0x0, param_3);
        uVar4 = (lVar6 >> 0x10);
        puVar5 = (lVar6 + 0xe0);
        ppcVar1 = (*puVar5 + 0x1c);
        (**ppcVar1)(s_tile2_bmp_1050_1538, puVar5, (lVar6 + 0xe2), uVar7);
        iVar2 = puVar5;
        piVar3 = (puVar5 & 0xffff | extraout_DX << 0x10);
        local_18.show_cmd = 0x9;
        local_18.rc_normal_position.x = *piVar3;
        local_18.rc_normal_position.y = (iVar2 + 2);
        iStack6 = (iVar2 + 0x4) + *piVar3;
        iStack4 = (iVar2 + 0x2) + (iVar2 + 0x6);
        SetWindowPlacement16(&local_18, &DAT_1050_1050);
    }
    return;
}
pub unsafe fn enum_child_windows_1010_01be() {
    if (PTR_LOOP_1050_0010.is_null()) {
        let func = MakeProcInstance16(HINSTANCE16_1050_038c, win_ui_op_1010_0240);
        EnumChildWindows1(0x0, func, func);
        FreeProcInstance16(func);
    }
    return;
}

pub unsafe fn pass1_1010_01f8(mut param_1: u32, mut param_2: u32, mut param_3: i16) {
    let mut iVar1: i16;
    let mut iVar2: i16;
    let mut uVar3: u16;
    let mut uVar4: u16;

    iVar2 = (param_3 * 0x4 + 0xe02) * 0x4;
    iVar1 = (iVar2 + 0xdfc);
    uVar3 = (param_1 >> 0x10);
    uVar4 = (param_2 >> 0x10);
    (param_2 + 0x6) = (param_3 * 0x4 + 0xe04) * 0x28 + (iVar2 + 0xdfa) + (param_1 + 0xa);
    (param_2 + 0x8) = (param_1 + 0xc) + iVar1;
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn win_ui_op_1010_0240(
    mut param_1: u16,
    mut param_2: u16,
    mut param_3: u16,
    mut param_4: u16,
) -> BOOL16 {
    let mut ppcVar1: *mut *mut code;
    let mut BVar2: bool;
    let mut WVar3: u16;
    let mut in_register_0000000a: u16;
    let mut paVar4: *mut Struct57;
    let mut puVar5: *mut u32;
    let mut in_stack_0000fea2: u16;
    let mut in_stack_0000ffc6: u16;
    let mut in_stack_0000ffcc: u16;
    let mut in_stack_0000ffd0: u16;
    let mut in_stack_0000fff8: u32;
    let mut uVar6: u16;

    uVar6 = (in_stack_0000fff8 >> 0x10);
    paVar4 = CONCAT22(in_register_0000000a, param_1);
    BVar2 = IsWindow16(param_4);
    if (BVar2 != 0) {
        WVar3 = GetWindowWord16(-0x6, param_4);
        if (WVar3 == HINSTANCE16_1050_038c) {
            BVar2 = IsIconic16(param_4);
            if (BVar2 != 0) {
                puVar5 = mixed_1010_20ba(
                    paVar4,
                    _u16_1050_0ed0,
                    CONCAT22(uVar6, 0x45),
                    in_stack_0000fea2,
                    in_stack_0000ffc6,
                    in_stack_0000ffcc,
                    in_stack_0000ffd0,
                );
                ppcVar1 = ((puVar5 & 0xffff0000 | param_4) + 0x10);
                (**ppcVar1)(s_tile2_bmp_1050_1538, puVar5, (puVar5 >> 0x10), 1);
            }
        }
    }
    return 0x1;
}

pub unsafe fn pass1_1010_02a2(param_1: *mut u16, param_2: u8) -> *mut u16 {
    pass1_1010_0052(param_1, &DAT_1050_1050);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}
pub unsafe fn struct_1010_02e0(param_1: *mut astruct_19, mut param_2: u16) {
    let mut uVar1: u16;
    let mut uVar2: u16;
    let mut in_EDX: u32;
    let mut uVar4: u16;
    let mut paVar3: *mut Struct57;
    let mut paVar5: *mut astruct_19;

    uVar4 = (in_EDX >> 0x10);
    paVar5 = struct_op_1010_1d48(param_1, param_2);
    paVar3 = CONCAT22(uVar4, (paVar5 >> 0x10));
    uVar1 = 0;
    (param_1 + 0xa) = 0;
    (param_1 + 0xe) = 0;
    (param_1 + 0x10) = 0;
    (param_1 + 0x18) = 0;
    param_1.offset_0x0 = 0xe98;
    (param_1 + 0x2) = 0x1010;
    mem_op_1000_179c(0xc, paVar3);
    uVar2 = paVar3 | uVar1;
    if (uVar2 == 0) {
        (param_1 + 0xa) = 0;
    } else {
        set_struct_1008_574a(CONCAT22(paVar3, uVar1));
        (param_1 + 0xa) = uVar1;
        (param_1 + 0xc) = uVar2;
    }
    return;
}
pub unsafe fn pass1_1010_0350(param_1: *mut astruct_455) {
    let mut puVar1: *mut u32;
    let mut puVar2: *mut u32;
    let mut ppcVar3: *mut *mut code;
    let mut iVar4: *mut astruct_455;
    let mut uVar4: *mut astruct_455;

    uVar4 = (param_1 >> 0x10);
    iVar4 = param_1;
    param_1.field0_0x0 = 0xe98;
    iVar4.field1_0x2 = 0x1010;
    puVar1 = iVar4[0x1].field1_0x2;
    puVar2 = iVar4[0x1].field2_0x4;
    if ((puVar2 | puVar1) != 0) {
        ppcVar3 = *puVar1;
        (**ppcVar3)();
    }
    pass1_1010_1d80(param_1);
    return;
}
pub unsafe fn pass1_1010_038e(param_1: *mut astruct_27, mut param_2: i16) {
    let mut bVar1: bool;
    let mut iVar2: *mut astruct_27;
    let mut uVar2: u16;

    bVar1 = false;
    iVar2 = param_1;
    uVar2 = (param_1 >> 0x10);
    if (param_2 != 0) {
        if (&iVar2.field_0x18 == 0) {
            iVar2.field18_0x12 = DAT_1050_0e28;
            iVar2.field19_0x14 = PTR_LOOP_1050_0e30;
            iVar2.field20_0x16 = PTR_LOOP_1050_0ea8;
            DAT_1050_0e28 = 0;
            PTR_LOOP_1050_0e30 = null_mut();
            PTR_LOOP_1050_0ea8 = null_mut();
            iVar2.field_0x18 = 0x1;
            bVar1 = true;
            // TODO: goto LAB_1010_0404;
        }
    }
    if (param_2 == 0) {
        if (&iVar2.field_0x18 != 0) {
            DAT_1050_0e28 = iVar2.field18_0x12;
            PTR_LOOP_1050_0e30 = iVar2.field19_0x14;
            PTR_LOOP_1050_0ea8 = iVar2.field20_0x16;
            iVar2.field_0x18 = 0;
            bVar1 = true;
        }
    } //
      // LAB_1010_0404:
    if (bVar1) {
        pass1_1010_1f62(param_1, 0x3);
    }
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn pass1_1010_041a() -> BOOL16 {
    let mut uVar1: u32;

    uVar1 = pass1_1030_8326();
    if (((uVar1 >> 0x10) == 0) && (uVar1 < 0x64)) {
        return 0x0;
    }
    return 0x1;
}
pub unsafe fn pass1_1010_043a(param_1: *mut astruct_27, param_2: i32, mut param_3: i16) {
    let mut ppcVar1: *mut *mut code;
    let mut pchar_1: *mut c_char;
    let mut uVar2: u16;
    let mut in_EDX: *mut Struct57;
    let mut iVar4: *mut astruct_27;
    let mut iVar5: *mut astruct_227;
    let mut uVar3: *mut astruct_27;
    let mut uVar4: u16;
    let mut puStack18: *mut u16;
    let mut puStack14: *mut u16;
    let mut local_a: [u8; 0x8] = [0; 0x8];

    iVar4 = param_1;
    uVar3 = (param_1 >> 0x10);
    if (param_3 == 0xc) {
        if (&iVar4.field_0xe != 0) {
            return;
        }
        iVar4.field_0xe = 0x1;
    } else if (param_3 == 0xd) {
        if (&iVar4.field_0x10 != 0) {
            return;
        }
        iVar4.field_0x10 = 0x1;
    } else if (param_3 == 0x12) {
        return;
    }
    pass1_1010_089e(param_1, 0x1, 0x6);
    pass1_1008_5784(CONCAT22(0x1050, local_a), &iVar4.field_0xa);
    loop {
        pchar_1 = local_a;
        pass1_1008_5b12(CONCAT22(0x1050, pchar_1));
        uVar2 = in_EDX;
        in_EDX = (in_EDX & 0xffff0000 | (uVar2 | pchar_1));
        if ((uVar2 | pchar_1) == 0) {
            mem_op_1000_179c(0xa, in_EDX);
            uVar2 = in_EDX;
            puStack18 = CONCAT22(uVar2, pchar_1);
            if ((uVar2 | pchar_1) == 0) {
                puStack14 = null_mut();
            } else {
                *puStack18 = 0x389a;
                (pchar_1 + 0x2) = 0x1008;
                *puStack18 = 0xea8;
                (pchar_1 + 0x2) = 0x1010;
                puStack14 = puStack18;
            }
            uVar4 = (puStack14 >> 0x10);
            iVar5 = puStack14;
            iVar5.field4_0x4 = param_3;
            iVar5.field5_0x6 = param_2;
            ppcVar1 = (*&iVar4.field_0xa + 0x8);
            (**ppcVar1)(0x1000, &iVar4.field_0xa, iVar5, uVar4);
            return;
        }
        if !(((pchar_1 + 0x4) != param_3) || ((pchar_1 + 0x6) != param_2)) {
            break;
        }
    }
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn pass1_1010_0538(
    param_1: *mut astruct_27,
    param_2: *mut *mut c_char,
    param_3: *mut *mut c_char,
) {
    let mut iVar1: i16;
    let mut uVar2: u32;
    let mut ppcVar3: *mut *mut code;
    let mut uVar4: u32;
    let mut uVar5: u16;
    let mut uVar6: u16;
    let mut pcVar7: *mut c_char;
    let mut puVar8: *mut u8;
    let mut extraout_DX: u16;
    let mut puVar9: *mut u8;
    let mut extraout_DX_00: *mut u8;
    let mut uVar10: *mut astruct_27;
    let mut uVar11: *mut astruct_27;
    let mut uVar12: u16;
    let mut puStack6: *mut u32;

    uVar5 = 0;
    *param_3 = null_mut();
    *param_2 = null_mut();
    uVar11 = (param_1 >> 0x10);
    uVar10 = param_1;
    ppcVar3 = (*&uVar10.field_0xa + 0x10);
    (**ppcVar3)();
    puStack6 = CONCAT22(extraout_DX, uVar5);
    puVar9 = (extraout_DX | uVar5);
    if (puVar9.is_null()) {
        return;
    }
    iVar1 = (uVar5 + 0x4);
    uVar2 = (uVar5 + 0x6);
    if ((extraout_DX | uVar5) != 0) {
        ppcVar3 = *puStack6;
        (**ppcVar3)();
        puVar9 = extraout_DX_00;
    }
    uVar4 = &uVar10.field_0xa;
    if ((uVar4 + 0x8) == 0) {
        pass1_1010_089e(param_1, 0x0, 0x6);
        pass1_1010_1f62(param_1, 0x3);
    }
    uVar6 = iVar1 + 0x757;
    load_string_1010_84ac(_u16_1050_14cc, (_u16_1050_14cc >> 0x10), uVar6);
    //   param_3 = uVar6;^(\s+)iVar
    (param_3 + 0x2) = puVar9;
    while (
        pcVar7 = pass1_1000_472c(*param_3, '%'),
        (puVar9 | pcVar7) != 0,
    ) {
        pass1_1010_09b4(
            puVar9,
            uVar10,
            uVar11,
            CONCAT22(puVar9, pcVar7),
            param_3,
            uVar2,
        );
    }
    //   if (0x1e < iVar1 - 1) goto LAB_1010_0850;
    uVar12 = (param_2 >> 0x10);
    //   switch(iVar1)
    match iVar1 {
        0x1 => {}
        // TODO: goto LAB_1010_0619;
        0x2 => {}
        // TODO: goto LAB_1010_0619;
        0x3 => {}
        // break;
        0x4 => {}
        // TODO: goto LAB_1010_0619;
        0x5 => {}
        // TODO: goto LAB_1010_0619;
        0x6 => {}
        // break;
        0x7 => {}
        // TODO: goto LAB_1010_0619;
        0x8 => {}
        // TODO: goto LAB_1010_0619;
        0x9 => {}
        // break;
        0xa => {}
        // TODO: goto LAB_1010_0619;
        0xb => {}
        // TODO: goto LAB_1010_0619;
        0xc => {}
        // break;
        0xd => {}
        // TODO: goto LAB_1010_0619;
        0xe => {}
        // break;
        0xf => {}
        // TODO: goto LAB_1010_0619;
        0x10 => {}
        // break;
        0x11 => {}
        // break;
        0x12 => {}
        // break;
        0x13 => {}
        // break;
        0x14 => {}
        // break;
        0x15 => {}
        // break;
        0x16 => {
            // LAB_1010_0619:
            puVar8 = pass1_1008_5fd8(puVar9);
        }
        // TODO: goto LAB_1010_0621;
        0x17 => {}
        // break;
        0x18 => {}
        // break;
        0x19 | 0x1f => {
            //
            // LAB_1010_0785:
            puVar8 = pass1_1008_5fd8(puVar9);
        }
        // TODO: goto LAB_1010_0621;
        0x1a => {}
        // TODO: goto LAB_1010_0785;
        0x1b => {}
        // TODO: goto LAB_1010_0785;
        0x1c => {}
        // break;
        0x1d => {}
        // break;
        0x1e => {
            puVar8 = pass1_1008_5fd8(puVar9);
            param_2 = puVar8;
            (param_2 + 0x2) = puVar9;
        } // TODO: goto LAB_1010_0785;
    };
    puVar8 = pass1_1008_5fd8(puVar9); //
                                      // LAB_1010_0621:
    param_2 = puVar8;
    (param_2 + 0x2) = puVar9; //
                              // LAB_1010_0850:
    while (
        pcVar7 = pass1_1000_472c(*param_2, '%'),
        (puVar9 | pcVar7) != 0,
    ) {
        pass1_1010_09b4(
            puVar9,
            uVar10,
            uVar11,
            CONCAT22(puVar9, pcVar7),
            param_2,
            uVar2,
        );
    }
    return;
}

pub unsafe fn pass1_1010_0886() -> u16 {
    return 0xa;
}

pub unsafe fn pass1_1010_088c() -> u16 {
    return 0x3;
}

pub unsafe fn pass1_1010_0892() -> u16 {
    return 0x3;
}

pub unsafe fn pass1_1010_0898() -> u16 {
    return 0x3;
}
pub unsafe fn pass1_1010_089e(param_1: *mut astruct_27, mut param_2: u16, mut param_3: i16) {
    ((param_3 + -1) * 0x8 + 0xe28) = param_2;
    pass1_1010_1f62(param_1, 0x3);
    return;
}
pub unsafe fn pass1_1010_08c0(mut param_1: u32, mut param_2: u16, mut param_3: i16) {
    ((param_3 + -1) * 0x8 + 0xea8) = param_2;
    pass1_1010_1f62(param_1, 0x3);
    return;
}

pub unsafe fn pass1_1010_08e2(mut param_1: u16, mut param_2: u16, mut param_3: i16) -> u32 {
    if (PTR_LOOP_1050_4fe8.is_null() == false) {
        DAT_1050_0e28 = 0;
        PTR_LOOP_1050_0e30 = null_mut();
        PTR_LOOP_1050_0e38 = null_mut();
        PTR_LOOP_1050_0e40 = null_mut();
        PTR_LOOP_1050_0e48 = null_mut();
        DAT_1050_0e58 = 0;
        DAT_1050_0e60 = 0;
        PTR_LOOP_1050_0e70 = null_mut();
    }
    return CONCAT22(0x1050, (param_3 + -1) * 0x8 + 0xe22);
}

pub unsafe fn pass1_1010_091e(mut param_1: u16, mut param_2: u16, mut param_3: i16) -> u32 {
    return CONCAT22(0x1050, (param_3 + -1) * 0x8 + 0xe72);
}

pub unsafe fn pass1_1010_0932(mut param_1: u16, mut param_2: u16, mut param_3: i16) -> u32 {
    return CONCAT22(0x1050, (param_3 + -1) * 0x8 + 0xe8a);
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn pass1_1010_0946(
    mut param_1: u16,
    mut param_2: u16,
    mut param_3: i16,
    param_4: *mut u8,
    mut param_5: i16,
    mut param_6: u16,
) -> u32 {
    let mut iVar1: i16;
    let mut uVar2: u16;
    let mut in_register_0000000a: u16;
    let mut puVar3: *mut u32;
    let mut in_stack_0000fe8c: u16;
    let mut in_stack_0000ffb0: u16;
    let mut in_stack_0000ffb6: u16;
    let mut in_stack_0000ffba: u16;
    let mut uVar4: u16;
    let mut uVar5: u16;
    let mut lVar6: i32;

    PTR_LOOP_1050_0ea8 = null_mut();
    lVar6 = 0x4000001;
    uVar4 = 0x2;
    uVar5 = 0x400;
    puVar3 = mixed_1010_20ba(
        CONCAT22(in_register_0000000a, param_4),
        _u16_1050_0ed0,
        0x2003b,
        in_stack_0000fe8c,
        in_stack_0000ffb0,
        in_stack_0000ffb6,
        in_stack_0000ffba,
    );
    uVar2 = (puVar3 >> 0x10);
    iVar1 = puVar3;
    pass1_1008_dfa6(puVar3, CONCAT22(uVar5, uVar4), lVar6);
    if (iVar1 != 0) {
        pass1_1030_8344(_u16_1050_5748, 0x4000002);
        if ((iVar1 + 0x200) == 0x8000002) {
            PTR_LOOP_1050_0ea8 = (&PTR_LOOP_1050_0000 + 1);
        }
    }
    return CONCAT22(0x1050, (param_3 + -1) * 0x8 + 0xea2);
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn pass1_1010_09b4(
    mut param_1: u16,
    mut param_2: u16,
    mut param_3: u16,
    param_4: *mut u8,
    param_5: *mut *mut c_char,
    mut param_6: u32,
) {
    let mut bVar1: u8;
    let mut bVar2: bool;
    let mut bVar3: bool;
    let mut uVar4: u16;
    let mut uVar5: u16;
    let mut iVar6: i16;
    let mut uVar7: u16;
    let mut in_register_0000000a: u16;
    let mut paVar8: *mut Struct57;
    let mut puVar9: *mut u32;
    let mut pcVar10: *mut c_char;
    let mut in_stack_0000fe86: u16;
    let mut in_stack_0000ffaa: u16;
    let mut in_stack_0000ffb0: u16;
    let mut in_stack_0000ffb4: u16;
    let mut in_stack_0000ffde: u16;
    let mut pcStack18: *mut c_char;
    let mut uStack10: u16;
    let mut uStack8: u16;

    paVar8 = CONCAT22(in_register_0000000a, param_1);
    bVar3 = false;
    bVar2 = false;
    bVar1 = *(param_4 + 1);
    if (bVar1 == 0x70) {
        //
        // LAB_1010_0a06:
        bVar3 = false;
        bVar2 = true;
    } else if (bVar1 < 0x71) {
        if (bVar1 != 0x43) {
            //   if (bVar1 == 0x50) goto LAB_1010_0a06;
            //   if (bVar1 != 0x63) goto LAB_1010_09db;
        }
        bVar3 = true;
        bVar2 = false;
    } //
      // LAB_1010_09db:
    uVar7 = 0;
    uStack10 = 0;
    uStack8 = 0;
    if (bVar2) {
        puVar9 = mixed_1010_20ba(
            paVar8,
            _u16_1050_0ed0,
            CONCAT22(in_stack_0000ffde, 0x2),
            in_stack_0000fe86,
            in_stack_0000ffaa,
            in_stack_0000ffb0,
            in_stack_0000ffb4,
        );
        uVar7 = (puVar9 >> 0x10);
        uStack10 = (puVar9 + 0x6c);
        paVar8 = (paVar8 & 0xffff0000 | (puVar9 + 0x6e));
    } else {
        // if (!bVar3) goto LAB_1010_0a36;
        pass1_1030_8344(_u16_1050_5748, param_6);
        pcVar10 = pass1_1038_4d28(CONCAT22(paVar8, uVar7));
        paVar8 = (paVar8 & 0xffff0000 | pcVar10 >> 0x10);
        uStack10 = pcVar10;
    }
    uStack8 = paVar8; //
                      // LAB_1010_0a36:
    if ((uStack8 | uStack10) != 0) {
        uVar4 = str_op_1000_3da4(*param_5);
        uVar5 = str_op_1000_3da4(CONCAT22(uStack8, uStack10));
        iVar6 = uVar5 + 0xa + uVar4;
        mem_op_1000_179c(iVar6, paVar8);
        uVar7 = SUB42(paVar8, 0x0);
        pcStack18 = CONCAT22(uVar7, iVar6);
        *param_4 = '\0';
        unk_str_op_1000_3d3e(CONCAT22(uVar7, iVar6), *param_5);
        pass1_1000_3cea(CONCAT22(uVar7, iVar6), CONCAT22(uStack8, uStack10));
        pass1_1000_3cea(
            CONCAT22(uVar7, iVar6),
            (param_4 & 0xffff0000 | (param_4 + 0x2)),
        );
        fn_ptr_1000_17ce(*param_5);
        *param_5 = pcStack18;
    }
    return;
}
pub unsafe fn pass1_1010_0ad2(mut param_1: u32, mut param_2: u32) {
    let mut uVar1: u32;
    let mut BVar2: bool;
    let mut puVar3: *mut u8;
    let mut extraout_DX: u16;
    let mut iVar4: i16;
    let mut uVar5: u16;
    let mut in_stack_0000ffbc: HFILE16;
    let mut local_2a: [u32; 0x2] = [0; 0x2];
    let mut local_22: [u16; 0x2] = [0; 0x2];
    let mut local_1e: [u16; 0x3] = [0; 0x3];
    let mut local_18: [u16; 0x3] = [0; 0x3];
    let mut uStack18: u32;
    let mut local_e: [u8; 0x8] = [0; 0x8];
    let mut uStack6: u16;
    let mut iStack4: i16;

    BVar2 = write_to_file_1008_7cac(param_2);
    if (BVar2 == 0) {
        return;
    }
    uVar5 = (param_1 >> 0x10);
    iVar4 = param_1;
    if ((iVar4 + 0xa) == 0) {
        uStack6 = 0;
    } else {
        uVar1 = (iVar4 + 0xa);
        uStack6 = (uVar1 + 0x8);
    }
    local_1e[0] = uStack6;
    BVar2 = write_to_file_1008_7e1c(param_2, CONCAT22(0x1050, local_1e), 0x2, in_stack_0000ffbc);
    if (BVar2 != 0) {
        pass1_1008_5784(CONCAT22(0x1050, local_e), (iVar4 + 0xa));
        loop {
            puVar3 = local_e;
            pass1_1008_5b12(CONCAT22(0x1050, puVar3));
            uStack18 = CONCAT22(extraout_DX, puVar3);
            if ((extraout_DX | puVar3) == 0) {
                local_22[0] = (iVar4 + 0xe);
                BVar2 = write_to_file_1008_7e1c(
                    param_2,
                    CONCAT22(0x1050, local_22),
                    0x2,
                    in_stack_0000ffbc,
                );
                if (BVar2 == 0) {
                    u16_1050_0310 = 0x6d0;
                    return;
                }
                local_22[0] = (iVar4 + 0x10);
                BVar2 = write_to_file_1008_7e1c(
                    param_2,
                    CONCAT22(0x1050, local_22),
                    0x2,
                    in_stack_0000ffbc,
                );
                if (BVar2 == 0) {
                    u16_1050_0310 = 0x6d0;
                    return;
                }
                if ((iVar4 + 0x18) != 0) {
                    DAT_1050_0e28 = (iVar4 + 0x12);
                    PTR_LOOP_1050_0e30 = (iVar4 + 0x14);
                    PTR_LOOP_1050_0ea8 = (iVar4 + 0x16);
                }
                iStack4 = 0;
                loop {
                    if (0x9 < iStack4) {
                        iStack4 = 0;
                        loop {
                            if (0x2 < iStack4) {
                                if ((iVar4 + 0x18) != 0) {
                                    DAT_1050_0e28 = 0;
                                    PTR_LOOP_1050_0e30 = null_mut();
                                    PTR_LOOP_1050_0ea8 = null_mut();
                                }
                                return;
                            }
                            local_1e[0] = (iStack4 * 0x8 + 0xea8);
                            BVar2 = write_to_file_1008_7e1c(
                                param_2,
                                CONCAT22(0x1050, local_1e),
                                0x2,
                                in_stack_0000ffbc,
                            );
                            if (BVar2 == 0) {
                                break;
                            }
                            iStack4 += 0x1;
                        }
                        u16_1050_0310 = 0x6d0;
                        return;
                    }
                    local_1e[0] = (iStack4 * 0x8 + 0xe28);
                    BVar2 = write_to_file_1008_7e1c(
                        param_2,
                        CONCAT22(0x1050, local_1e),
                        0x2,
                        in_stack_0000ffbc,
                    );
                    if (BVar2 == 0) {
                        break;
                    }
                    iStack4 += 0x1;
                }
                u16_1050_0310 = 0x6d0;
                return;
            }
            local_18[0] = (puVar3 + 0x4);
            BVar2 = write_to_file_1008_7e1c(
                param_2,
                CONCAT22(0x1050, local_18),
                0x2,
                in_stack_0000ffbc,
            );
            if (BVar2 == 0) {
                u16_1050_0310 = 0x6d0;
                return;
            }
            local_2a[0] = (uStack18 + 0x6);
            BVar2 = write_to_file_1008_7e1c(
                param_2,
                CONCAT22(0x1050, local_2a),
                0x4,
                in_stack_0000ffbc,
            );
            if BVar2 == 0 {
                break;
            }
        }
    }
    u16_1050_0310 = 0x6d0;
    return;
}
pub unsafe fn file_1010_0c7c(
    mut param_1: i16,
    param_2: *mut u8,
    param_3: *mut astruct_228,
    mut param_4: u32,
) {
    let mut ppcVar1: *mut *mut code;
    let mut BVar2: bool;
    let mut uVar4: *mut astruct_229;
    let mut uVar3: u16;
    let mut uVar5: u16;
    let mut in_register_0000000a: u16;
    let mut paVar6: *mut Struct57;
    let mut iVar6: *mut astruct_228;
    let mut uVar7: u16;
    let mut local_2a: [u16; 0x2] = [0; 0x2];
    let mut uStack38: u16;
    let mut puStack26: *mut u32;
    let mut puStack22: *mut u32;
    let mut local_12: [u16; 0x5] = [0; 0x5];
    let mut paStack8: *mut astruct_229;
    let mut local_6: *mut astruct_229;
    let mut uStack4: u16;

    paVar6 = CONCAT22(in_register_0000000a, param_2);
    uVar7 = (param_4 >> 0x10);
    read_file_1008_7cfe(param_4, uVar7, 0x6);
    if (param_1 == 0) {
        u16_1050_0310 = 0x6d4;
    } else {
        BVar2 = read_file_1008_7dee(param_4, CONCAT22(0x1050, &local_6), 0x2);
        paStack8 = null_mut();
        iVar6 = param_3;
        if (BVar2 != 0) {
            //   for (paStack8 = null_mut(); iVar6 = param_3, paStack8 < local_6;
            //   paStack8 = &paStack8.field_1)
            while paStack8 < local_6 {
                uVar4 = local_6;
                mem_op_1000_179c(0xa, paVar6);
                uVar5 = paVar6;
                puStack26 = CONCAT22(uVar5, uVar4);
                paVar6 = (paVar6 & 0xffff0000 | (uVar5 | uVar4));
                if ((uVar5 | uVar4) == 0) {
                    puStack22 = null_mut();
                } else {
                    puStack26 = 0x389a;
                    uVar4.field2_0x2 = 0x1008;
                    puStack26 = 0xea8;
                    uVar4.field2_0x2 = 0x1010;
                    puStack22 = puStack26;
                }
                BVar2 = read_file_1008_7dee(param_4, CONCAT22(0x1050, local_12), 0x2);
                if ((BVar2 == 0)
                    || (
                        BVar2 = read_file_1008_7dee(
                            param_4,
                            (puStack22 & 0xffff0000 | (puStack22 + 0x6)),
                            0x4,
                        ),
                        BVar2 == 0,
                    ))
                {
                    puStack26 = puStack22;
                    if (puStack22.is_null() == false) {
                        ppcVar1 = *puStack22;
                        (**ppcVar1)(0x1008, puStack22, (puStack22 >> 0x10), 1);
                    }
                    // TODO: goto LAB_1010_0cb1;
                }
                (puStack22 + 0x4) = local_12[0];
                ppcVar1 = (*iVar6.field10_0xa + 0x8);
                (**ppcVar1)();
                paStack8 = paStack8.field_1;
            }
            BVar2 = read_file_1008_7dee(
                param_4,
                (param_3 & 0xffff0000 | ZEXT24(&iVar6.field_0xe)),
                0x2,
            );
            if ((BVar2 != 0)
                && (
                    BVar2 = read_file_1008_7dee(
                        param_4,
                        (param_3 & 0xffff0000 | ZEXT24(&iVar6.field_0x10)),
                        0x2,
                    ),
                    BVar2 != 0,
                ))
            {
                // for (uStack4 = 0; uStack4 < 0xa; uStack4 += 1)
                for uStack4 in 0..0xa {
                    BVar2 = read_file_1008_7dee(param_4, CONCAT22(0x1050, local_2a), 0x2);
                    //   if (BVar2 == 0) goto LAB_1010_0cb1;
                    uVar3 = uStack4;
                    if (u16_1050_0312 < 0x2) {
                        uVar3 = pass1_1008_738c(param_4, uVar7, uStack4);
                    }
                    (uVar3 * 0x8 + 0xe28) = local_2a[0];
                    uStack38 = uVar3;
                }
                if (0x2 < u16_1050_0312) {
                    uStack4 = 0;
                    loop {
                        BVar2 = read_file_1008_7dee(param_4, CONCAT22(0x1050, local_2a), 0x2);
                        // if (BVar2 == 0) goto LAB_1010_0cb1;
                        (uStack4 * 0x8 + 0xea8) = local_2a[0];
                        uStack4 += 0x1;
                        if uStack4 >= 3 {
                            break;
                        }
                    }
                }
                return;
            }
        } //
          // LAB_1010_0cb1:
        u16_1050_0310 = 0x6d2;
    }
    return;
}

pub unsafe fn pass1_1010_0e46(param_1: *mut u16, param_2: u8, mut param_3: u16) -> *mut u16 {
    pass1_1010_0350(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1010_0e6c(param_1: *mut u16, param_2: u8) -> *mut u16 {
    *param_1 = 0x389a;
    (param_1 + 0x2) = 0x1008;
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn pass1_1010_0eac(
    param_1: *mut u8,
    param_2: *mut astruct_19,
    mut param_3: u16,
) -> *mut astruct_19 {
    struct_op_1018_4cda(param_2, param_3);
    param_2.offset_0x0 = 0xf0c;
    (param_2 + 0x2) = 0x1010;
    _PTR_LOOP_1050_4230 = param_2;
    pass1_1018_4dce(param_1, param_2, 0xff);
    return param_2;
}

pub unsafe fn pass1_1010_0ee6(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    clenaup_win_ui_1018_4d22(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address
pub unsafe fn pass1_1010_0f24(param_1: *mut u8, param_2: *mut astruct_19, mut param_3: u16) {
    let mut in_register_0000000a: u16;
    let mut paVar1: *mut Struct57;
    let mut unaff_BP: u16;
    let mut puVar2: *mut u32;
    let mut in_stack_0000fea6: u16;
    let mut in_stack_0000ffca: u16;
    let mut in_stack_0000ffd0: u16;
    let mut in_stack_0000ffd4: u16;

    paVar1 = CONCAT22(in_register_0000000a, param_1);
    struct_1010_2cd2(param_2, param_3);
    (param_2 + 0x60) = 0;
    (param_2 + 0x64) = 0;
    (param_2 + 0x68) = 0;
    (param_2 + 0x6a) = 0;
    // 0x191a
    param_2.offset_0x0 = s_648_bmp_1050_1919 + 1;
    (param_2 + 0x2) = 0x1010;
    puVar2 = mixed_1010_20ba(
        paVar1,
        _u16_1050_0ed0,
        CONCAT22(unaff_BP, 0x3),
        in_stack_0000fea6,
        in_stack_0000ffca,
        in_stack_0000ffd0,
        in_stack_0000ffd4,
    );
    (param_2 + 0x6a) = puVar2;
    (param_2 + 0x6c) = (puVar2 >> 0x10);
    return;
}
pub unsafe fn pass1_1010_0f76(param_1: *mut astruct_455) {
    let mut uVar1: *mut astruct_455;

    uVar1 = (param_1 >> 0x10);
    param_1.field0_0x0 = s_648_bmp_1050_1919 + 1;
    (param_1 + 0x2) = 0x1010;
    pass1_1010_17c0((param_1 & 0xffff | ZEXT24(uVar1) << 0x10));
    pass1_1010_2db2(param_1);
    return;
}
pub unsafe fn struct_1010_0f9c(
    mut param_1: u16,
    param_2: *mut astruct_232,
    param_3: *mut astruct_57,
) {
    let mut puVar1: *mut u8;
    let mut ppcVar2: *mut *mut code;
    let mut iVar3: i16;
    let mut paVar4: *mut astruct_92;
    let mut paVar5: *mut astruct_92;
    let mut uVar6: u16;
    let mut uVar7: u32;
    let mut uVar8: u16;
    let mut iVar8: *mut astruct_232;
    let mut iVar9: *mut astruct_231;
    let mut iVar10: *mut astruct_230;
    let mut iVar11: *mut astruct_233;
    let mut uVar9: *mut astruct_232;
    let mut uVar10: u16;
    let mut paVar11: *mut astruct_232;
    let mut uVar12: u8;
    let mut uStack36: u32;
    let mut iStack32: i16;
    let mut uStack30: u16;
    astruct_92 * *ppaStack28;
    let mut paStack24: *mut astruct_15;
    let mut local_14: *mut astruct_92;

    uVar9 = (param_2 >> 0x10);
    iVar8 = param_2;
    ppcVar2 = (param_2 + 0x38);
    (**ppcVar2)();
    iVar8.field100_0x68 = param_1;
    if ((&iVar8.field96_0x60 != 0) && (iVar8.field100_0x68 == 1)) {
        return;
    }
    if (iVar8.field100_0x68 == 0) {
        return;
    }
    pass1_1028_dc52(CONCAT22(0x1050, &local_14), 0x1, 0x0, 0x700);
    iVar3 = iVar8.field100_0x68 * 0x18;
    mem_op_1000_179c(iVar3, param_3);
    iVar8.field96_0x60 = iVar3;
    iVar8.field97_0x62 = param_3;
    ppaStack28 = CONCAT22(param_3, iVar8.field96_0x60);
    uStack30 = iVar8.field100_0x68;
    loop {
        loop {
            paVar4 = &local_14;
            pass1_1028_e4ec(CONCAT22(0x1050, paVar4));
            uVar8 = param_3;
            paStack24 = CONCAT22(uVar8, paVar4);
            param_3 = (param_3 & 0xffff0000 | (uVar8 | paVar4));
            //   if ((uVar8 | paVar4) == 0) goto LAB_1010_10ca;
            iVar9 = param_2;
            ppcVar2 = &iVar9.field58_0x40;
            paVar5 = paVar4;
            (**ppcVar2)(0x1028, param_2);
            if paVar5.is_null() == false {
                break;
            }
        }
        pass1_1028_b58e(paStack24);
        uStack36 = CONCAT22(param_3, paVar5);
        ppcVar2 = &iVar9.field44_0x2c;
        (**ppcVar2)(0x1028, param_2);
        uVar10 = (ppaStack28 >> 0x10);
        iVar10 = ppaStack28;
        *ppaStack28 = paVar5;
        iVar10.field2_0x2 = param_3;
        uVar12 = SUB21(paVar4, 0x0);
        ppcVar2 = &iVar9.field46_0x30;
        paVar11 = param_2;
        (**ppcVar2)();
        iVar10.field7_0x8 = paVar5;
        iVar10.field8_0xa = param_3;
        iVar10.field9_0xc = uStack36;
        ppcVar2 = &iVar9.field56_0x3c;
        uVar7 = uStack36;
        (**ppcVar2)(0x1028, param_2, paStack24, paVar11, uVar12, uVar8);
        iVar10.field10_0x10 = uVar7;
        iVar10.field11_0x12 = param_3;
        iVar10.field12_0x14 = uStack36;
        ppaStack28 = (ppaStack28 & 0xffff0000 | ZEXT24(iVar10 + 1));
        uStack30 -= 1;
        if uStack30 == 0 {
            break;
        }
    }
    // LAB_1010_10ca:
    uVar6 = iVar8.field100_0x68 << 0x2;
    mem_op_1000_179c(uVar6, param_3);
    iVar8.field98_0x64 = uVar6;
    iVar8.field99_0x66 = param_3;
    iStack32 = 0;
    uStack30 = 0;
    loop {
        if ((iVar8.field100_0x68 * 0x3) <= uStack30) {
            break;
        }
        puVar1 = iVar8.field97_0x62;
        uVar7 = &iVar8.field98_0x64;
        uVar10 = (uVar7 >> 0x10);
        iVar11 = uVar7;
        (iVar11 + iStack32 * 0x4) = iVar8.field96_0x60 + uStack30 * 0x8;
        (iVar11 + iStack32 * 0x4 + 0x2) = puVar1;
        uStack30 += 0x3;
        iStack32 += 0x1;
    }
    return;
}
