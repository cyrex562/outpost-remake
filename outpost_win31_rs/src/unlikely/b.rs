
pub unsafe fn window_op_1008_0af8(mut param_1: u16, struct_param_1: *mut StructA) {
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
    let mut struct_20_v6: *mut astruct_20;
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
    (**fn_ptr_1)(0x1000, struct_param_1, 0x0, 0x15a, &DAT_1050_1050);
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


pub unsafe fn mixed_win_op_1008_0c60(
    mut param_1: u16,
    param_2: *mut StructD,
    mut param_3: u16,
    mut param_4: u16,
    param_5: *mut Struct72,
    mut param_6: u16,
    mut param_7: u16,
    mut param_8: u16,
) -> BOOL16 {
    let mut ppcVar1: *mut *mut code;
    let mut HVar2: HISTANCE16;
    let mut iVar3: i16;
    let mut BVar4: bool;
    let mut uVar7: u16;
    let mut pSVar8: *mut StructD;
    let mut uVar15: u16;
    let mut struct_var5: *mut Struct72;
    let mut uVar6: u16;
    let mut in_AF: u8;
    let mut uVar5: u32;
    let mut lresult_6: LRESULT;
    let mut pcVar16: *mut c_char;
    let mut puVar17: *mut u32;
    let mut in_stack_0000fcd2: u16;
    let mut in_stack_0000fce4: u16;
    let mut in_stack_0000fcf8: u16;
    let mut in_stack_0000fd58: *mut WNDCLASS16;
    let mut in_stack_0000fe34: u16;
    let mut in_stack_0000fe3a: u16;
    let mut in_stack_0000ff58: u16;
    let mut in_stack_0000ff5e: u16;
    let mut in_stack_0000ff62: u16;
    let mut in_stack_0000ff64: u16;
    let mut in_stack_0000ff68: u16;
    let mut WVar18: WPARAM16;
    let mut uVar19: u16;
    let mut local_64: [u8; 0x50] = [0; 0x50];
    let mut uStack20: u32;
    let mut HStack16: HCURSOR16;
    let mut HStack14: HCURSOR16;
    let mut uStack6: u32;
    let mut puVar1: *mut u32;
    let mut uVar9: u8;
    let mut uVar10: u8;
    let mut iVar12: i16;
    let mut uVar13: u16;
    let mut struct_var15: *mut Struct72;
    let mut uVar14: u16;
    let mut in_stack_0000ff92: u16;
    let mut uVar11: u8;
    let mut uVar12: u16;
    let mut paVar9: *mut Struct57;

    struct_var5 = param_5;
    uVar6 = (param_5 >> 0x10);
    pSVar8 = param_2;

    //   switch(param_6) {
    match param_6 {
        // case 0x64:
        0x64 => {
            BVar4 = pass1_1008_07d8(param_1, param_2, struct_var5);
            win_ui_cursor_op_1008_2e9a(
                param_2,
                param_5,
                in_stack_0000fd58,
                in_stack_0000fcd2,
                in_stack_0000fce4,
                in_stack_0000fcf8,
            );
            return BVar4;
        }
        //   case 0x65:
        0x65 => {
            pass1_1008_3018(pSVar8, param_5);
            return param_1;
        }
        //   case 0x66:
        0x66 => {
            pass1_1008_30cc(param_1, pSVar8, param_5);
            return param_1;
        }
        //   case 0x67:
        0x67 => {
            iVar3 = win_ui_op_1008_2b54(param_1, pSVar8, param_5);
            if (iVar3 == 0) {
                return 0x0;
            }
        }
        //   case 0xee:
        0xee => {
            uVar19 = struct_var5.field7_0x8;
            WVar18 = 0;
            uVar9 = '\x10';
            uVar10 = '\0';
        }
        // TODO: goto LAB_1008_0d18;
        //   case 0x68:
        0x68 => {
            pass1_1030_8344(_u16_1050_5748, 0x4000001);
            uVar7 = param_2 | param_1;
            paVar9 = (param_2 & 0xffff0000 | uVar7);
            if (uVar7 == 0) {
                return param_1;
            }
            if (PTR_LOOP_1050_4fe8.is_null() == false) {
                pcVar16 = load_string_1010_847e(_u16_1050_14cc, 0x57b);
                BVar4 = MessageBox16(
                    0x10,
                    pcVar16,
                    s_You_may_not_run_a_turn__The_game_1050_0172,
                    struct_var5.field7_0x8,
                );
                return BVar4;
            }
            HStack14 = LoadCursor16(0x7f02, 0x0);
            HStack16 = SetCursor16(HStack14);
            puVar17 = mixed_1010_20ba(
                paVar9,
                _u16_1050_0ed0,
                CONCAT22(param_3, 0x29),
                in_stack_0000fe3a,
                in_stack_0000ff5e,
                in_stack_0000ff64,
                in_stack_0000ff68,
            );
            uVar15 = (paVar9 >> 0x10);
            uStack20 = SUB42(puVar17, 0x0);
            uStack20 = (puVar17 >> 0x10);
            pass1_1018_262e(puVar17);
            pass1_1030_838e(_u16_1050_5748);
            (_u16_1050_5748 + 0x8) = 0x1;
            pass1_1030_8326();
            pcVar16 = load_string_1010_847e(_u16_1050_14cc, 0x5dc);
            paVar9 = CONCAT22(uVar15, (pcVar16 >> 0x10));
            sys_1000_3f9c(
                CONCAT13(0x10, CONCAT12(0x50, local_64)),
                s__s__ld_1050_019c,
                pcVar16,
            );
            uVar12 = 0;
            ppcVar1 = (param_5 + 0x14);
            (**ppcVar1)(
                0x0,
                struct_var5,
                (param_5 >> 0x10),
                0x0,
                local_64,
                &DAT_1050_1050,
            );
            puVar17 = mixed_1010_20ba(
                paVar9,
                _u16_1050_0ed0,
                CONCAT22(uVar12, 0x37),
                in_stack_0000fe34,
                in_stack_0000ff58,
                in_stack_0000ff5e,
                in_stack_0000ff62,
            );
            pass1_1008_a9ec(puVar17);
            SetCursor16(HStack16);
            uVar19 = struct_var5.field7_0x8;
            WVar18 = 0xfc;
            uVar11 = '\x11';
        }
        // TODO: goto LAB_1008_0e3d;
        //   _ =>
        _ => {
            if (((&struct_var5.field227_0xe8 + 0x2) | &struct_var5.field227_0xe8) == 0) {
                return 0x0;
            }
            puVar1 = struct_var5.field227_0xe8;
            ppcVar1 = (*struct_var5.field227_0xe8 + 0x40);
            BVar4 = (**ppcVar1)(0x8, puVar1, (puVar1 >> 0x10), param_6);
            return BVar4;
        }
        //   case 0x6e:
        0x6e => {
            iVar12 = 0x2;
        }
        // TODO: goto LAB_1008_0cba;
        //   case 0x6f:
        0x6f => {
            uStack6 = FUN_1010_830a(param_1, param_2, 0x1008, _u16_1050_14cc, 0x1f8);
            uStack6 = SUB42(param_2, 0x0);
            BVar4 = WinHelp16(0x0, 0x3, CONCAT22(uStack6, uStack6), struct_var5.field7_0x8);
            return BVar4;
        }
        //   case 0x70:
        0x70 => {
            iVar12 = 0x1; //
                          // LAB_1008_0cba:
            uVar5 = pass1_1038_af40(
                struct_var5,
                pSVar8,
                _PTR_LOOP_1050_5b7c,
                struct_var5.field7_0x8,
                iVar12,
            );
            return uVar5;
        }
        //   case 0x71:
        0x71 => {
            HVar2 = WinExec16(0x3, s_notepad_read_me_1050_0162);
            return HVar2;
        }
        //   case 0x79:
        0x79 => {
            BVar4 = post_msg_1008_2d22(param_5);
            return BVar4;
        }
        //   case 0x7a:
        0x7a => {
            uVar13 = 0xb;
        }
        // TODO: goto LAB_1008_0f3e;
        //   case 0x7b:
        0x7b => {
            uVar13 = 0x1e;
        }
        // TODO: goto LAB_1008_0f3e;
        //   case 0x7c:
        0x7c => {
            uVar13 = 0x1f;
        }
        // TODO: goto LAB_1008_0f3e;
        //   case 0x7d:
        0x7d => {
            uVar13 = 0x21;
        }
        // TODO: goto LAB_1008_0f3e;
        //   case 0x7e:
        0x7e => {
            uVar13 = 0x35;
        }
        // TODO: goto LAB_1008_0f3e;
        //   case 0x7f:
        0x7f => {
            uVar14 = 0x39;
        }
        // break;
        //   case 0x80:
        0x80 => {
            uVar13 = 0x22;
        }
        // TODO: goto LAB_1008_0f3e;
        //   case 0x81:
        0x81 => {
            uVar14 = 0x36;
        }
        // break;
        //   case 0x82:
        0x82 => {
            uVar14 = 0x37;
        }
        // break;
        //   case 0x83:
        0x83 => {
            uVar14 = 0x38;
        }
        // break;
        //   case 0x84:
        0x84 => {
            uVar14 = 0x3a;
        }
        // break;
        //   case 0x85:
        0x85 => {
            uVar14 = 0x3b;
        }
        // break;
        //   case 0x86:
        0x86 => {
            uVar14 = 0x3c;
        }
        // break;
        //   case 0x87:
        0x87 => {
            uVar14 = 0x3d;
        }
        // break;
        //   case 0x88:
        0x88 => {
            uVar14 = 0x3e;
        }
        // break;
        //   case 0x89:
        0x89 => {
            uVar14 = 0x3f;
        }
        // break;
        //   case 0x8a:
        0x8a => {
            uVar14 = 0x40;
        }
        // break;
        //   case 0x8b:
        0x8b => {
            uVar13 = 0xc;
        }
        // TODO: goto LAB_1008_0f3e;
        //   case 0x8c:
        0x8c => {
            uVar14 = 0x41;
        }
        // break;
        //   case 0x8d:
        0x8d => {
            uVar14 = 0x42;
        }
        // break;
        //   case 0x8e:
        0x8e => {
            uVar14 = 0x43;
        }
        // break;
        //   case 0x8f:
        0x8f => {
            uVar14 = 0x44;
        }
        // break;
        //   case 0x90:
        0x90 => {
            uVar14 = 0x45;
        }
        // break;
        //   case 0x91:
        0x91 => {
            uVar14 = 0x46;
        }
        // break;
        //   case 0x92:
        0x92 => {
            uVar14 = 0x47;
        }
        // break;
        //   case 0x93:
        0x93 => {
            uVar13 = 0x23;
        }
        // TODO: goto LAB_1008_0f3e;
        //   case 0x94:
        0x94 => {
            uVar13 = 0x24;
        }
        // TODO: goto LAB_1008_0f3e;
        //   case 0x95:
        0x95 => {
            uVar14 = 0x48;
        }
        // break;
        //   case 0x96:
        0x96 => {
            uVar14 = 0x49;
        }
        // break;
        //   case 0x97:
        0x97 => {
            uVar14 = 0x4a;
        }
        // break;
        //   case 0x98:
        0x98 => {
            uVar14 = 0x4b;
        }
        // break;
        //   case 0x99:
        0x99 => {
            uVar14 = 0x4c;
        }
        // break;
        //   case 0x9a:
        0x9a => {
            uVar13 = 0xd;
        }
        // TODO: goto LAB_1008_0f3e;
        //   case 0x9b:
        0x9b => {
            uVar14 = 0x4d;
        }
        // break;
        //   case 0x9c:
        0x9c => {
            uVar14 = 0x4e;
        }
        // break;
        //   case 0x9d:
        0x9d => {
            uVar14 = 0x4f;
        }
        // break;
        //   case 0x9e:
        0x9e => {
            uVar14 = 0x50;
        }
        // break;
        //   case 0x9f:
        0x9f => {
            uVar14 = 0x51;
        }
        // break;
        //   case 0xa0:
        0xa0 => {
            uVar13 = 0xe;
        }
        // TODO: goto LAB_1008_0f3e;
        //   case 0xa1:
        0xa1 => {
            uVar13 = 0xf;
        }
        // TODO: goto LAB_1008_0f3e;
        //   case 0xa2:
        0xa2 => {
            uVar14 = 0x52;
        }
        // break;
        //   case 0xa3:
        0xa3 => {
            uVar13 = 0x10;
        }
        // TODO: goto LAB_1008_0f3e;
        //   case 0xa4:
        0xa4 => {
            uVar14 = 0x53;
        }
        // break;
        //   case 0xa5:
        0xa5 => {
            uVar13 = 0x11;
        }
        // TODO: goto LAB_1008_0f3e;
        //   case 0xa6:
        0xa6 => {
            uVar13 = 0x12;
        }
        // TODO: goto LAB_1008_0f3e;
        //   case 0xa7:
        0xa7 => {
            uVar14 = 0x57;
        }
        // break;
        //   case 0xa8:
        0xa8 => {
            uVar13 = 0x13;
        }
        // TODO: goto LAB_1008_0f3e;
        //   case 0xa9:
        0xa9 => {
            uVar13 = 0x14;
        }
        // TODO: goto LAB_1008_0f3e;
        //   case 0xaa:
        0xaa => {
            uVar14 = 0x58;
        }
        // break;
        //   case 0xab:
        0xab => {
            uVar14 = 0x63;
        }
        // break;
        //   case 0xac:
        0xac => {
            uVar14 = 0x59;
        }
        // break;
        //   case 0xad:
        0xad => {
            uVar14 = 0x5a;
        }
        // break;
        //   case 0xae:
        0xae => {
            uVar14 = 0x5b;
        }
        // break;
        //   case 0xaf:
        0xaf => {
            uVar14 = 0x15;
        }
        // break;
        //   case 0xb0:
        0xb0 => {
            uVar13 = 0x25;
        }
        // TODO: goto LAB_1008_0f3e;
        //   case 0xb1:
        0xb1 => {
            uVar14 = 0x5c;
        }
        // break;
        //   case 0xb2:
        0xb2 => {
            uVar14 = 0x16;
        }
        // break;
        //   case 0xb3:
        0xb3 => {
            uVar14 = 0x5d;
        }
        // break;
        //   case 0xb4:
        0xb4 => {
            uVar13 = 0x5e;
        }
        // TODO: goto LAB_1008_0f3e;
        //   case 0xb5:
        0xb5 => {
            uVar14 = 0x5f;
        }
        // break;
        //   case 0xb6:
        0xb6 => {
            uVar14 = 0x60;
        }
        // break;
        //   case 0xb7:
        0xb7 => {
            uVar14 = 0x61;
        }
        // break;
        //   case 0xb8:
        0xb8 => {
            uVar14 = 0x62;
        }
        // break;
        //   case 0xb9:
        0xb9 => {
            uVar14 = 0x64;
        }
        // break;
        //   case 0xba:
        0xba => {
            uVar14 = 0x65;
        }
        // break;
        //   case 0xbb:
        0xbb => {
            uVar14 = 0x66;
        }
        // break;
        //   case 0xbc:
        0xbc => {
            uVar14 = 0x67;
        }
        // break;
        //   case 0xbd:
        0xbd => {
            uVar14 = 0x68;
        }
        // break;
        //   case 0xbe:
        0xbe => {
            uVar14 = 0x69;
        }
        // break;
        //   case 0xbf:
        0xbf => {
            uVar13 = 0x17;
        }
        // TODO: goto LAB_1008_0f3e;
        //   case 0xc0:
        0xc0 => {
            uVar13 = 0x18;
        }
        // TODO: goto LAB_1008_0f3e;
        //   case 0xc1:
        0xc1 => {
            uVar13 = 0x19;
        }
        // TODO: goto LAB_1008_0f3e;
        //   case 0xc2:
        0xc2 => {
            uVar13 = 0x1a;
        }
        // TODO: goto LAB_1008_0f3e;
        //   case 0xc3:
        0xc3 => {
            uVar13 = 0x1b;
        }
        // TODO: goto LAB_1008_0f3e;
        //   case 0xc4:
        0xc4 => {
            uVar13 = 0x1c;
        }
        // TODO: goto LAB_1008_0f3e;
        //   case 0xc5:
        0xc5 => {
            uVar13 = 0x1d;
        }
        // TODO: goto LAB_1008_0f3e;
        //   case 0xc6:
        0xc6 => {
            uVar13 = 0x4;
        }
        // TODO: goto LAB_1008_0f3e;
        //   case 0xc8:
        0xc8 => {
            uVar13 = 0x3;
        }
        // TODO: goto LAB_1008_0f3e;
        //   case 0xc9:
        0xc9 => {
            uVar13 = 0x1;
        }
        // TODO: goto LAB_1008_0f3e;
        //   case 0xca:
        0xca => {
            uVar13 = 0x5;
        }
        // TODO: goto LAB_1008_0f3e;
        //   case 0xcb:
        0xcb => {
            pass1_1008_087e(in_AF, param_1, pSVar8);
            uVar13 = 0x6;
        }
        // TODO: goto LAB_1008_0f3e;
        //   case 0xcc:
        0xcc => {
            uVar13 = 0x7;
        }
        // TODO: goto LAB_1008_0f3e;
        //   case 0xcd:
        0xcd => {
            uVar13 = 0x8;
        }
        // TODO: goto LAB_1008_0f3e;
        //   case 0xce:
        0xce => {
            uVar13 = 0x9;
        }
        // TODO: goto LAB_1008_0f3e;
        //   case 0xcf:
        0xcf => {
            uVar13 = 0xa;
        }
        // TODO: goto LAB_1008_0f3e;
        //   case 0xd0:
        0xd0 => {
            uVar13 = 0x26;
        }
        // TODO: goto LAB_1008_0f3e;
        //   case 0xd1:
        0xd1 => {
            uVar13 = 0x27;
        }
        // TODO: goto LAB_1008_0f3e;
        //   case 0xd2:
        0xd2 => {
            uVar13 = 0x28;
        }
        // TODO: goto LAB_1008_0f3e;
        //   case 0xd3:
        0xd3 => {
            uVar13 = 0x29;
        }
        // TODO: goto LAB_1008_0f3e;
        //   case 0xd4:
        0xd4 => {
            uVar13 = 0x2a;
        }
        // TODO: goto LAB_1008_0f3e;
        //   case 0xd5:
        0xd5 => {
            uVar13 = 0x2b;
        }
        // TODO: goto LAB_1008_0f3e;
        //   case 0xd6:
        0xd6 => {
            uVar13 = 0x2c;
        }
        // TODO: goto LAB_1008_0f3e;
        //   case 0xd7:
        0xd7 => {
            uVar13 = 0xd7;
        }
        // TODO: goto LAB_1008_0f3e;
        //   case 0xd8:
        0xd8 => {
            uVar13 = 0x2e;
        }
        // TODO: goto LAB_1008_0f3e;
        //   case 0xd9:
        0xd9 => {
            uVar13 = 0x2f;
        }
        // TODO: goto LAB_1008_0f3e;
        //   case 0xda:
        0xda => {
            uVar13 = 0x30;
        }
        // TODO: goto LAB_1008_0f3e;
        //   case 0xdb:
        0xdb => {
            uVar13 = 0x31;
        }
        // TODO: goto LAB_1008_0f3e;
        //   case 0xdc:
        0xdc => {
            uVar13 = 0x32;
        }
        // TODO: goto LAB_1008_0f3e;
        //   case 0xdd:
        0xdd => {
            uVar13 = 0x33;
        }
        // TODO: goto LAB_1008_0f3e;
        //   case 0xde:
        0xde => {
            uVar13 = 0x34; //
                           // LAB_1008_0f3e:
            cursor_op_1008_2dcc(param_2, param_5, uVar13, param_7, param_8);
            return param_1;
        }
        //   case 0xdf:
        0xdf => {
            uVar14 = 0x55;
        }
        // break;
        //   case 0xe0:
        0xe0 => {
            uVar14 = 0x56;
        }
        // break;
        //   case 0x100:
        0x100 => {
            win_1008_5c5c(param_1, pSVar8, _u16_1050_02a0, 0x1dc);
            return param_1;
        }
        //   case 0x12c:
        0x12c => {
            uVar19 = struct_var5.field7_0x8;
            WVar18 = 0xf020;
            uVar9 = '\x12';
            uVar10 = '\x01'; //
                             // LAB_1008_0d18:
            lresult_6 = SendMessage16(0x0, WVar18, CONCAT11(uVar10, uVar9), uVar19);
            return lresult_6;
        }
        //   case 0x12e:
        0x12e => {
            uVar19 = struct_var5.field7_0x8;
            WVar18 = 0xf060;
            uVar11 = '\x12'; //
                             // LAB_1008_0e3d:
            BVar4 = PostMessage16(0x0, WVar18, CONCAT11(0x1, uVar11), uVar19);
            return BVar4;
        }
    };
    ui_op_1008_2c4e(pSVar8, param_8, param_5, uVar14);
    return param_1;
}

pub unsafe fn caseD_aa() {
    let mut in_DX: u16;
    let mut unaff_BP: i16;
    let mut in_stack_0000ffee: u16;

    ui_op_1008_2c4e(in_DX, in_stack_0000ffee, (unaff_BP + 0x6), 0x58);
    return;
}

pub unsafe fn caseD_ac() {
    let mut in_DX: u16;
    let mut unaff_BP: i16;
    let mut in_stack_0000ffee: u16;

    ui_op_1008_2c4e(in_DX, in_stack_0000ffee, (unaff_BP + 0x6), 0x59);
    return;
}

pub unsafe fn caseD_ad() {
    let mut in_DX: u16;
    let mut unaff_BP: i16;
    let mut in_stack_0000ffee: u16;

    ui_op_1008_2c4e(in_DX, in_stack_0000ffee, (unaff_BP + 0x6), 0x5a);
    return;
}

pub unsafe fn caseD_ae() {
    let mut in_DX: u16;
    let mut unaff_BP: i16;
    let mut in_stack_0000ffee: u16;

    ui_op_1008_2c4e(in_DX, in_stack_0000ffee, (unaff_BP + 0x6), 0x5b);
    return;
}

pub unsafe fn caseD_b1() {
    let mut in_DX: u16;
    let mut unaff_BP: i16;
    let mut in_stack_0000ffee: u16;

    ui_op_1008_2c4e(in_DX, in_stack_0000ffee, (unaff_BP + 0x6), 0x5c);
    return;
}
