
pub unsafe fn dos3_calls_1000_5198(mut param_1: u16 ,
                         mut param_2: u16 ,
                         mut param_3: u16 ,
                         mut param_4: u16 ) -> u16
{
    let mut pcVar1 =  swi(0x21);
    (*pcVar1)( &DAT_1050_1050);
    pcVar1 =  swi(0x21);
    (*pcVar1)();
    pcVar1 =  swi(0x21);
    (*pcVar1)();
    pcVar1 =  swi(0x21);
    (*pcVar1)();
    if ((param_2 & 0x100) == 0) {
        return 0x0;
    }
    pass1_1000_29b5(param_3);
    return param_3 & 0xff;
}


pub unsafe fn pass1_1000_5224(mut param_1: u16 ,
                    mut param_2: u16 ,
                    mut param_3: u16 ,
                    mut param_4: u16 ) -> u32
{
    let mut uVar1: u32;
    let mut lVar2: i32;
    let mut uVar3: u16;
    let mut iVar4: i16;
    let mut uVar5: u16;
    let mut uVar6: u16;
    let mut uVar7: u16;
    let mut uVar8: u16;
    let mut bVar10: bool;
    let mut cVar11: u8;
    let mut uVar9: u16;

    cVar11 =  param_2 < 0x0;
    if ( cVar11) {
        bVar10 = param_1 != 0;
        param_1 = -param_1;
        param_2 = - bVar10 - param_2;
    }
    if ( param_4 < 0x0) {
        cVar11 += '\x01';
        bVar10 = param_3 != 0;
        param_3 = -param_3;
        param_4 = - bVar10 - param_4;
    }
    uVar3 = param_1;
    uVar5 = param_3;
    uVar6 = param_2;
    uVar9 = param_4;
    if (param_4 == 0) {
        uVar3 = param_2 / param_3;
        iVar4 =  (( param_2 %  param_3 << 0x10 |  param_1) /  param_3);
    } else {
        loop {
            uVar8 = uVar9 >> 0x1;
            uVar5 = uVar5 >> 0x1 |  ((uVar9 & 1) != 0) << 0xf;
            uVar7 = uVar6 >> 0x1;
            uVar3 = uVar3 >> 0x1 |  ((uVar6 & 1) != 0) << 0xf;
            uVar6 = uVar7;
            uVar9 = uVar8;
            if uVar9 == 0 {
                break;
            }
        }
        uVar1 = CONCAT22(uVar7,
                         uVar3) /  uVar5;
        iVar4 =  uVar1;
        lVar2 =  param_3 * (uVar1 & 0xffff);
        uVar3 =  ( lVar2 >> 0x10);
        uVar5 = uVar3 + iVar4 * param_4;
        if (((CARRY2(uVar3,
                     iVar4 * param_4)) || (param_2 < uVar5)) || ((param_2 <= uVar5 && (param_1 <  lVar2)))) {
            iVar4 += -0x1;
        }
        uVar3 = 0;
    }
    if (cVar11 == '\x01') {
        bVar10 = iVar4 != 0;
        iVar4 = -iVar4;
        uVar3 = - bVar10 - uVar3;
    }
    return CONCAT22(uVar3,
                    iVar4);
}


pub unsafe fn pass1_1000_53f0(mut param_1: u16 ,
                    mut param_2: u16 ,
                    mut param_3: u16 ,
                    mut param_4: u16 ) -> u32
{
    let mut uVar1: u32;
    let mut lVar2: i32;
    let mut uVar3: u16;
    let mut uVar4: u16;
    let mut uVar5: u16;
    let mut iVar6: i16;
    let mut iVar7: i16;
    let mut uVar8: u16;
    let mut uVar9: u16;
    let mut uVar10: u16;
    let mut bVar11: bool;

    uVar3 = param_1;
    uVar4 = param_4;
    uVar9 = param_2;
    uVar10 = param_3;
    if (param_4 == 0) {
        iVar6 =  (( param_2 %  param_3 << 0x10 |  param_1) %  param_3);
        iVar7 = 0;
    } else {
        loop {
            uVar5 = uVar4 >> 0x1;
            uVar10 = uVar10 >> 0x1 |  ((uVar4 & 1) != 0) << 0xf;
            uVar8 = uVar9 >> 0x1;
            uVar3 = uVar3 >> 0x1 |  ((uVar9 & 1) != 0) << 0xf;
            uVar4 = uVar5;
            uVar9 = uVar8;
            if uVar5 == 0 {break;}
        }
        uVar1 = CONCAT22(uVar8,
                         uVar3) /  uVar10;
        uVar3 =  uVar1 * param_4;
        lVar2 = (uVar1 & 0xffff) *  param_3;
        uVar9 =  ( lVar2 >> 0x10);
        uVar4 =  lVar2;
        uVar10 = uVar9 + uVar3;
        if (((CARRY2(uVar9,
                     uVar3)) || (param_2 < uVar10)) || ((param_2 <= uVar10 && (param_1 < uVar4)))) {
            bVar11 = uVar4 < param_3;
            uVar4 -= param_3;
            uVar10 = (uVar10 - param_4) -  bVar11;
        }
        iVar6 = -(uVar4 - param_1);
        iVar7 = - (uVar4 - param_1 != 0) - ((uVar10 - param_2) -  (uVar4 < param_1));
    }
    return CONCAT22(iVar7,
                    iVar6);
}

pub unsafe fn pass1_1000_5512(param_1: *mut u8,
                     mut param_2: u16 ,
                     mut param_3: i16,
                     mut param_4: i16,
                     mut param_5: u16 )
{
    let mut b_var1: bool;
    let mut u_stack4: u16;

    pass1_1000_52be(param_3,
                    param_4,
                    param_5,
                    0x0);
    while (true) {
        b_var1 = param_3 == 0;
        param_3 += -0x1;
        param_4 -= b_var1;
        if (param_4 < 0x0) {
            break;
        }
        u_stack4 = param_2;
        ( param_1)();
    }
}


pub unsafe fn mixed_win_sys_op_1008_016e(param_1: *mut astruct_823) {
    let mut puVar1: *mut u16;
    let mut uVar6: u16;
    let mut iVar3: i16;
    let mut uVar5: u16;
    let mut uVar9: u16;
    let mut uVar11: u16;
    let mut uVar8: u32;
    let mut DVar10: u16;
    let mut puVar4: *mut u8;
    let mut puVar14: u16;
    let mut uVar13: u16;
    let mut puVar12: *mut u8;
    let mut puVar13: *mut u8;
    let mut uVar7: u16;
    let mut in_EDX: u32;
    let mut struct_1: *mut astruct_832;
    let mut unaff_DI: i16;
    let mut uVar10: u16;
    let mut uVar12: u16;
    let mut DVar16: u32;
    let mut puVar17: *mut u32;
    let mut pSVar18: *mut StructD;
    let mut in_stack_0000fe46: u16;
    let mut local_13e: [u8; 0xac] = [0; 0xac];
    let mut local_92: [u8; 0x80] = [0; 0x80];
    let mut uStack18: u16;
    let mut uStack16: u16;
    let mut puStack14: *mut u32;
    let mut uStack10: u16;
    let mut uStack8: u16;
    let mut uStack6: u16;
    let mut uStack4: u16;
    let mut uVar1: u32;
    let mut puVar2: *mut u16;
    let mut uVar4: *mut astruct_20;
    let mut uVar2: *mut astruct_20;
    let mut uVar3: *mut astruct_20;
    let mut paVar14: *mut Struct57;
    let mut paVar15: *mut Struct57;
    let mut fn_ptr: *mut *mut code;

    uVar9 = (in_EDX >> 0x10);
    DVar16 = GetVersion16();
    DVar10 = (DVar16 >> 0x10);
    paVar14 = CONCAT22(uVar9, DVar10);
    uStack6 = (DVar16 & 0xffff);
    uVar6 = DVar16 & 0xff;
    uStack10 = ((DVar16 & 0xffff) >> 0x8);
    uStack8 = uVar6;
    if ((uVar6 < 0x3) || (uVar6 == 0x3 && (uStack10 < 0xa))) {
        // 0x97
        uVar12 = 0x1000;
        mem_op_1000_179c(0xb4, paVar14);
        uStack16 = paVar14;
        puVar4 = (uStack16 | uVar6);
        paVar14 = (paVar14 & 0xffff0000);
        paVar15 = (paVar14 | ZEXT24(puVar4));
        uStack18 = uVar6;
        if (puVar4.is_null()) {
            iVar3 = 0;
        } else {
            uVar12 = &PTR_LOOP_1050_1040;
            iVar3 = string_1040_8520(paVar15, CONCAT22(uStack16, uVar6), 0x0, 0x20010, 0x5dd05de);
            paVar14 = paVar15;
        }
        puStack14 = CONCAT22(paVar14, iVar3);
        fn_ptr = (*puStack14 + 0x74);
        (**fn_ptr)(uVar12, iVar3, paVar14);
        fn_ptr_op_1000_24cd(1);
    }
    debug_print_1008_6048(paVar14, s_version__d__d_1050_0012);
    if ((uStack8 == 0x3) && (0xb < uStack10)) {
        PTR_LOOP_1050_0010 = (&PTR_LOOP_1050_0000 + 1);
    }
    LoadString16(
        0x80,
        CONCAT22(0x1050, local_92),
        0x578,
        HINSTANCE16_1050_038c,
    );
    uVar5 = dos3_call_1000_51aa(local_92, &DAT_1050_1050, 1);
    if (uVar5 != 0) {
        LoadString16(
            0x80,
            CONCAT13(0x10, CONCAT12(0x50, local_13e)),
            0x57b,
            HINSTANCE16_1050_038c,
        );
        LoadString16(
            0x80,
            CONCAT13(0x10, CONCAT12(0x50, &stack0xfe42)),
            0x62e,
            HINSTANCE16_1050_038c,
        );
        uVar5 = MessageBox16(
            0x10,
            CONCAT13(0x10, CONCAT12(0x50, local_13e)),
            CONCAT22(0x1050, &stack0xfe42),
            0x0,
        );
        fn_ptr_op_1000_24cd(1);
    }
    mem_op_1000_179c(0x4, paVar14);
    uStack16 = paVar14;
    paVar14 = (paVar14 & 0xffff0000);
    if ((uStack16 | uVar5) == 0) {
        uVar9 = 0;
        uStack18 = uVar5;
    } else {
        uStack18 = uVar5;
        puVar17 = pass1_1008_5394(CONCAT22(uStack16, uVar5));
        paVar14 = (paVar14 & 0xffff0000 | puVar17 >> 0x10);
        uVar9 = SUB42(puVar17, 0x0);
    }
    uVar10 = (param_1 >> 0x10);
    struct_1 = param_1;
    struct_1.field5_0x8 = uVar9;
    (struct_1.field5_0x8 + 0x2) = paVar14;
    uVar8 = struct_1.field5_0x8;
    puVar1 = struct_1.field5_0x8;
    _PTR_LOOP_1050_0298 = uVar8;
    *puVar1 = 0x70;
    // 0x1538
    (puVar1 + 0x2) = s_tile2_bmp_1050_1538;
    mem_op_1000_179c(0x126, paVar14);
    uVar11 = uVar8;
    uStack16 = paVar14;
    paVar15 = (paVar14 & 0xffff0000 | (uStack16 | uVar11));
    uStack18 = uVar11;
    if ((uStack16 | uVar11) != 0) {
        pSVar18 = pass1_1010_2024((uVar8 & 0xffff | paVar14 << 0x10));
        paVar15 = (paVar15 & 0xffff0000 | pSVar18 >> 0x10);
        uVar11 = pSVar18;
    }
    if (_u16_1050_0ed0 == 0) {
        debug_print_1008_6048(paVar15, s_New_failed_in_Op__Op_1050_0020);
        fn_ptr_op_1000_24cd(1);
    }
    mem_op_1000_179c(0xe8c, paVar15);
    uStack16 = paVar15;
    puVar12 = (uStack16 | uVar11);
    paVar14 = (paVar15 & 0xffff0000 | ZEXT24(puVar12));
    uStack18 = uVar11;
    if (puVar12.is_null() == false) {
        pass1_1010_7e40(puVar12, CONCAT22(uStack16, uVar11));
    }
    if (_u16_1050_14cc == 0) {
        debug_print_1008_6048(paVar14, s_New_failed_in_Op__Op__ResLibr_1050_0035);
        fn_ptr_op_1000_24cd(1);
    }
    mem_op_1000_179c(0xb0, paVar14);
    uStack16 = paVar14;
    paVar14 = (paVar14 & 0xffff0000 | (uStack16 | uVar11));
    uStack18 = uVar11;
    if ((uStack16 | uVar11) != 0) {
        pSVar18 = pass1_1038_aeca(CONCAT22(uStack16, uVar11));
        paVar14 = (paVar14 & 0xffff0000 | pSVar18 >> 0x10);
        uVar11 = pSVar18;
    }
    if (_PTR_LOOP_1050_5b7c == 0) {
        debug_print_1008_6048(paVar14, s_New_failed_in_Op__Op__DialogCtr_1050_0053);
        fn_ptr_op_1000_24cd(1);
    }
    mem_op_1000_179c(0xa, paVar14);
    uStack16 = paVar14;
    puVar13 = (uStack16 | uVar11);
    paVar14 = (paVar14 & 0xffff0000 | ZEXT24(puVar13));
    uStack18 = uVar11;
    if (puVar13.is_null() == false) {
        make_proc_inst_1038_cf6c(puVar13, CONCAT22(uStack16, uVar11));
    }
    if (_u16_1050_5bc8 == 0) {
        debug_print_1008_6048(paVar14, s_New_failed_in_Op__Op__DialogHand_1050_0073);
        fn_ptr_op_1000_24cd(1);
    }
    mem_op_1000_179c(0x14, paVar14);
    uStack16 = paVar14;
    paVar14 = (paVar14 & 0xffff0000 | (uStack16 | uVar11));
    uStack18 = uVar11;
    if ((uStack16 | uVar11) != 0) {
        pass1_1008_5bdc(CONCAT22(uStack16, uVar11));
    }
    if (_u16_1050_02a0 == 0) {
        debug_print_1008_6048(paVar14, s_New_failed_in_Op__Op__Simulator_1050_0097);
        fn_ptr_op_1000_24cd(1);
    }
    mem_op_1000_179c(0xfc, paVar14);
    uStack16 = paVar14;
    uVar7 = uStack16 | uVar11;
    uStack18 = uVar11;
    if (uVar7 == 0) {
        uVar11 = 0;
        uVar7 = 0;
    } else {
        set_struct_op_1008_0536(CONCAT22(uStack16, uVar11), in_stack_0000fe46);
    }
    struct_1.field4_0x4 = uVar11;
    (&struct_1.field4_0x4 + 0x2) = uVar7;
    if (struct_1.field4_0x4.is_null()) {
        debug_print_1008_6048(uVar7, s_New_failed_in_Op__Op_1050_00b7);
        fn_ptr_op_1000_24cd(1);
    }
    win_ui_reg_class_1008_96d2(struct_1.field4_0x4);
    fn_ptr = (struct_1.field4_0x4 + 0x8);
    (**fn_ptr)(0x1000);
    uVar4 = struct_1.field4_0x4;
    HWND16_1050_0396 = (uVar4 + 0x8);
    uVar2 = struct_1.field4_0x4;
    fn_ptr = (struct_1.field4_0x4 + 0xc);
    (**fn_ptr)(0x1000, uVar2, (uVar2 >> 0x10), 0x3);
    uVar3 = struct_1.field4_0x4;
    UpdateWindow16((uVar3 + 0x8));
    return;
}


pub unsafe fn pass1_1008_049c(mut param_1: u16, mut param_2: u16, param_3: *mut c_char) {
    let mut uVar1: u16;
    let mut puVar2: *mut u8;

    if (param_3.is_null() == false) {
        uVar1 = str_op_1000_3da4(param_3);
        if (uVar1 != 0) {
            puVar2 = pass1_1000_545a(param_3 & 0xffff0000 | (param_3 + 1), s_nomono2_1050_00cc);
            if (puVar2.is_null()) {
                PTR_LOOP_1050_02ec = puVar2;
            }
        }
    }
    return;
}


pub unsafe fn pass1_1008_04d2(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    pass1_1008_9466(&param_1.address_offset_field_0x0);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}


pub unsafe fn pass1_1008_04f8(param_1: *mut u16, param_2: u8) -> *mut u16 {
    pass1_1008_0036(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}


pub unsafe fn win_ui_cursor_op_1008_06c0(
    param_1: u32,
    mut param_2: u32,
    mut param_3: u16,
    mut param_4: i16,
) {
    let mut ppcVar1: *mut *mut code;
    let mut in_AX: u16;
    let mut in_EDX: u32;
    let mut paVar2: *mut Struct57;
    let mut uVar3: u16;
    let mut iVar4: i16;
    let mut uVar5: u16;
    let mut in_AF: u8;
    let mut pcVar6: *mut c_char;
    let mut puVar7: *mut u32;
    let mut in_stack_0000fe3a: u16;
    let mut in_stack_0000fe44: u16;
    let mut in_stack_0000ff5e: u16;
    let mut in_stack_0000ff64: u16;
    let mut in_stack_0000ff68: u16;
    let mut in_stack_0000ff6e: u16;
    let mut in_stack_0000ff72: u16;
    let mut iVar8: i16;
    let mut in_stack_0000ff9c: u16;
    let mut local_5a: [u8; 0x50] = [0; 0x50];
    let mut uStack10: u16;
    let mut uStack8: u16;
    let mut HStack6: HCURSOR16;
    let mut HStack4: HCURSOR16;

    if (param_4 == 0x400) {
        pass1_1030_8344(_u16_1050_5748, 0x4000001);
        in_AX = in_EDX | in_AX;
        paVar2 = (in_EDX & 0xffff0000 | in_AX);
        if (in_AX != 0) {
            iVar4 = param_1;
            uVar5 = (param_1 >> 0x10);
            if (PTR_LOOP_1050_4fe8.is_null() == false) {
                pcVar6 = load_string_1010_847e(_u16_1050_14cc, 0x57b);
                MessageBox16(
                    0x10,
                    pcVar6,
                    s_You_may_not_run_a_turn__The_game_1050_00df,
                    (iVar4 + 0x8),
                );
                return;
            }
            HStack4 = LoadCursor16(0x7f02, 0x0);
            HStack6 = SetCursor16(HStack4);
            pass1_1030_83ba(in_AF, _u16_1050_5748, param_2);
            (_u16_1050_5748 + 0x8) = 0x1;
            puVar7 = mixed_1010_20ba(
                paVar2,
                _u16_1050_0ed0,
                CONCAT22(in_stack_0000ff9c, 0x29),
                in_stack_0000fe44,
                in_stack_0000ff68,
                in_stack_0000ff6e,
                in_stack_0000ff72,
            );
            uVar3 = (paVar2 >> 0x10);
            uStack10 = SUB42(puVar7, 0x0);
            uStack8 = (puVar7 >> 0x10);
            pass1_1018_262e(puVar7);
            pass1_1030_8326();
            pcVar6 = load_string_1010_847e(_u16_1050_14cc, 0x5dc);
            paVar2 = CONCAT22(uVar3, (pcVar6 >> 0x10));
            sys_1000_3f9c(
                CONCAT13(0x10, CONCAT12(0x50, local_5a)),
                s__s__ld_1050_0109,
                pcVar6,
            );
            ppcVar1 = (*param_1 + 0x14);
            iVar8 = iVar4;
            (**ppcVar1)(
                0x1000,
                iVar4,
                (param_1 >> 0x10),
                0x0,
                local_5a,
                &DAT_1050_1050,
            );
            puVar7 = mixed_1010_20ba(
                paVar2,
                _u16_1050_0ed0,
                CONCAT22(iVar8, 0x37),
                in_stack_0000fe3a,
                in_stack_0000ff5e,
                in_stack_0000ff64,
                in_stack_0000ff68,
            );
            pass1_1008_a9ec(puVar7);
            SetCursor16(HStack6);
            PostMessage16(0x0, 0xfc, 0x111, (iVar4 + 0x8));
        }
    }
    return;
}


pub unsafe fn pass1_1008_0932() -> u32 {
    let mut uVar1: u32;

    if (_u16_1050_14cc.is_null() == false) {
        pass1_1010_7fd6(_u16_1050_14cc);
    }
    mem_1000_0016(_PTR_LOOP_1050_03a0);
    mem_1000_0016(_PTR_LOOP_1050_029c);
    mem_1000_0016(_PTR_LOOP_1050_4fb8);
    mem_1000_0016(_PTR_LOOP_1050_68a2);
    mem_1000_0016(_PTR_LOOP_1050_5744);
    uVar1 = mem_1000_0016(_PTR_LOOP_1050_5f2c);
    return uVar1;
}


pub unsafe fn pass1_1008_0984(mut param_1: i16, mut param_2: u16, mut param_3: i16) {
    let mut ppcVar1: *mut *mut code;
    let mut in_EDX: u32;

    set_sys_color_1008_357e(CONCAT22(param_2, param_1), param_3, in_EDX);
    if ((param_1 + 0xe8) != 0) {
        ppcVar1 = ((param_1 + 0xe8) + 0x98);
        (**ppcVar1)();
    }
    return;
}


pub unsafe fn menu_ui_op_1008_09ba(param_1: *mut astruct_853, param_2: HWND16, param_3: *mut RECT16) {
    let mut HVar1: HMENU16;
    let mut iVar2: *mut astruct_853;
    let mut uVar2: u16;

    uVar2 = (param_1 >> 0x10);
    iVar2 = param_1;
    if (iVar2.field235_0xec == 0) {
        HVar1 = LoadMenu16(s_OPPOPMENU_1050_0150, HINSTANCE16_1050_038c);
        iVar2.field235_0xec = HVar1;
        if (HVar1 == 0) {
            return;
        }
        HVar1 = GetSubMenu16(0x0, iVar2.field235_0xec);
        iVar2.field235_0xec = HVar1;
        if (HVar1 == 0) {
            return;
        }
    }
    ClientToScreen16(CONCAT22(0x1050, &stack0xfffa), iVar2.field8_0x8);
    HVar1 = iVar2.field235_0xec;
    TrackPopupMenu16(NULL, HWND16_1050_0396, 0x0, HVar1, 0x0, 0x0, HVar1);
    return;
}

pub unsafe fn unk_win_msg_op_1008_0a3c(mut param_1: u32, mut param_2: u16) -> u16 {
    let mut BVar1: bool;
    let mut iVar2: i16;
    let mut uVar3: u16;

    iVar2 = param_1;
    uVar3 = (param_1 >> 0x10);
    if ((param_2 & 0xfff0) == 0xf140) {
        return (iVar2 + 0xde);
    }
    if ((param_2 & 0xfff0) == 0xf060) {
        BVar1 = IsIconic16((iVar2 + 0x8));
        if (BVar1 == 0) {
            PostMessage16(0x0, 0x67, 0x111, (iVar2 + 0x8));
        }
        return 0x0;
    }
    return 0x1;
}

pub unsafe fn pass1_1008_0a92(mut param_1: u32) {
    let mut ppcVar1: *mut *mut code;
    let mut iVar2: i16;
    let mut uVar3: u16;

    uVar3 = (param_1 >> 0x10);
    iVar2 = param_1;
    if ((iVar2 + 0xee) != 0) {
        ppcVar1 = ((iVar2 + 0xee) + 0x90);
        (**ppcVar1)();
    }
    if ((iVar2 + 0xe8) != 0) {
        ppcVar1 = ((iVar2 + 0xe8) + 0x90);
        (**ppcVar1)();
    }
    if (_PTR_LOOP_1050_0388.is_null() == false) {
        ppcVar1 = *_PTR_LOOP_1050_0388;
        (**ppcVar1)();
    }
    post_quit_msg_1008_3af4();
    return;
}
