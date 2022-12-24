use crate::block_1020::block_1020_6000::{destroy_icon_1020_6bd2, pass1_1020_6466, pass1_1020_64d4, pass1_1020_6746};
use crate::block_1020::block_1020_7000::{pass1_1020_75c4, pass1_1020_7824, pass1_1020_78ac};
use crate::block_1020::block_1020_8000::{pass1_1020_808e, pass1_1020_8556};
use crate::draw_ops::palette_op_1020_7270;
use crate::win_ui::{cleanup_menu_ui_op_1020_795c, invalidate_rect_1020_735a};

pub unsafe fn pass1_1020_679c(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    pass1_1020_6466(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1020_687c(param_1: *mut astruct_868) {
    let mut unaff_BP: u8;

    get_win_ui_info_op_1020_7a50(param_1);
    destroy_icon_1020_6bd2(param_1, unaff_BP);
    return;
}

pub unsafe fn pass1_1020_6bbc(mut param_1: u32) {
    let mut in_DX: u16;

    win_ui_op_1020_737a(in_DX, (param_1 + 0xf6));
    return;
}

pub unsafe fn pass1_1020_6e52(
    mut param_1: u16,
    mut param_2: u16,
    mut param_3: i16,
    mut param_4: u16,
    mut param_5: i16,
) {
    let mut uVar1: u16;
    let mut pcVar2: *mut c_char;

    pass1_1018_2e5e(param_1, param_2, (param_3 + 0xf2));
    uVar1 = param_2 | param_1;
    if (uVar1 == 0) {
        pcVar2 = load_string_1010_847e(_u16_1050_14cc, 0x5a1);
    } else {
        pass1_1018_2d84(param_1, (param_3 + 0xf2));
        pcVar2 = CONCAT22(uVar1, param_1);
    }
    string_1020_79b4(CONCAT22(param_4, param_3), param_5, pcVar2);
    return;
}


pub unsafe fn pass1_1020_70c0(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    cleanup_menu_ui_op_1020_795c(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1020_7526(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    palette_op_1020_7270(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn win_1020_75f0(param_1: *mut astruct_283, mut param_2: u32) {
    let mut pUVar1: *mut u16;
    let mut ppcVar2: *mut *mut code;
    let mut uVar3: u32;
    let mut uVar4: u16;
    let mut uVar5: u16;
    let mut uVar8: u16;
    let mut iVar7: *mut astruct_283;
    let mut uVar9: u16;
    let mut puVar10: *mut u16;
    let mut in_stack_0000ff4c: u16;
    let mut in_stack_0000ff62: u16;
    let mut uStack10: u32;
    let mut local_6: [u8; 0x4] = [0; 0x4];
    let mut paVar6: *mut Struct57;
    let mut uVar7: u32;

    uVar8 = (param_2 >> 0x10);
    uVar9 = (param_1 >> 0x10);
    iVar7 = param_1;
    if (iVar7.field235_0xee.is_null() == false) {
        ppcVar2 = (*iVar7.field235_0xee + 0x8);
        (**ppcVar2)();
    }
    if (iVar7.field233_0xea == 0) {
        iVar7.field233_0xea = 0x1;
        puVar10 = pass1_1008_941a(CONCAT22(0x1050, local_6), 0x1, 0x91);
        uVar4 = (puVar10 >> 0x10);
        paVar6 = CONCAT22(uVar8, uVar4);
        uVar3 = ZEXT24(local_6);
        win_1008_5c9e(local_6, uVar4, _u16_1050_02a0, CONCAT22(0x1050, local_6));
        iVar7.field234_0xec = uVar3;
        mem_op_1000_179c(0x112, paVar6);
        uVar5 = paVar6 | uVar3;
        uVar7 = paVar6 & 0xffff0000 | uVar5;
        if (uVar5 == 0) {
            uVar9 = 0;
            uStack10 = null_mut();
        } else {
            pUVar1 = &iVar7.field204_0xcc;
            *pUVar1 = *pUVar1 + 1;
            struct_1020_3644(
                uVar7,
                CONCAT13((paVar6 >> 0x8), CONCAT12(paVar6, uVar3)),
                iVar7.field204_0xcc,
                param_1 & 0xffff | uVar9 << 0x10,
                in_stack_0000ff4c,
                in_stack_0000ff62,
            );
            uVar9 = uVar3;
            uStack10 = (uVar3 & 0xffff | uVar7 << 0x10);
        }
        pass1_1008_6978(uVar9, uVar7, param_1, 0x0, uStack10 & 0xffff0000 | uVar9);
        ppcVar2 = (*uStack10 + 0xc);
        (**ppcVar2)(0x8, uStack10, uStack10, 0x5);
    }
    return;
}


pub unsafe fn window_op_1020_76aa(param_1: *mut StructA, param_2: *mut astruct_666) {
    let mut uVar3: u16;
    let mut in_EDX: *mut Struct57;
    let iVar1: *mut StructA;
    let mut uVar2: u16;

    create_window_ex_1008_9760(param_1);
    uVar2 = (param_1 >> 0x10);
    iVar1 = param_1;
    get_dc_1018_4db0(&iVar1[0x1].field20_0x26, iVar1.field4_0x8);
    mem_op_1000_179c(0x18, in_EDX);
    uVar3 = in_EDX | param_2;
    if (uVar3 != 0) {
        pass1_1020_7824(uVar3, param_2, in_EDX, iVar1.field4_0x8);
        iVar1[0x1].field18_0x22 = param_2;
        iVar1[0x1].field19_0x24 = uVar3;
        return;
    }
    iVar1[0x1].field18_0x22 = 0;
    return;
}


pub unsafe fn pass1_1020_770e(mut param_1: u32) {
    let mut puVar1: *mut u32;
    let mut uVar2: u16;
    let mut ppcVar3: *mut *mut code;
    let mut iVar4: i16;
    let mut uVar5: u16;

    uVar5 = (param_1 >> 0x10);
    iVar4 = param_1;
    puVar1 = (iVar4 + 0xee);
    uVar2 = (iVar4 + 0xf0);
    if ((uVar2 | puVar1) != 0) {
        ppcVar3 = *puVar1;
        (**ppcVar3)();
    }
    (iVar4 + 0xee) = 0;
    destroy_win_1008_628e(param_1 & 0xffff | uVar5 << 0x10);
    return;
}


pub unsafe fn pass1_1020_774c(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    param_1 = (param_1 & 0xffff0000 | (param_1 - 0xe2));
    pass1_1020_75c4(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1020_78dc(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    pass1_1020_78ac(&param_1.address_offset_field_0x0);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1020_7b60(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    cleanup_menu_ui_op_1020_795c(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1020_7f38(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    param_1.address_offset_field_0x0 = 0x3ab0;
    (param_1 + 0x2) = 0x1008;
    param_1.address_offset_field_0x0 = 0x389a;
    (param_1 + 0x2) = 0x1008;
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1020_8106(mut param_1: u32) {
    let mut ppcVar1: *mut *mut code;

    ppcVar1 = ((param_1 + 0x4) + 0x60);
    (**ppcVar1)();
    return;
}

pub unsafe fn realize_palette_1020_8128(mut param_1: u32, mut param_2: i16) {
    let mut ppcVar1: *mut *mut code;
    let mut uVar2: u32;
    let mut puVar3: *mut u8;
    let mut puVar4: *mut u32;
    let mut puVar5: *mut u32;
    let mut extraout_DX: u16;
    let mut iVar6: i16;
    let mut uVar7: u16;
    let mut local_12: [u8; 0x8] = [0; 0x8];
    let mut hdc_10: HDC16;
    let mut HStack8: HGDIOBJ16;
    let mut puStack6: *mut u32;

    if (param_2 != 0) {
        uVar7 = (param_1 >> 0x10);
        iVar6 = param_1;
        uVar2 = (iVar6 + 0xe6);
        puVar5 = (uVar2 + 0xa);
        ppcVar1 = (*puVar5 + 0x18);
        puStack6 = puVar5;
        (**ppcVar1)();
        HStack8 = puVar5;
        UnrealizeObject16(HStack8);
        uVar2 = (iVar6 + 0xe6);
        hdc_10 = *(uVar2 + 0x14);
        RealizePalette16(hdc_10);
        pass1_1008_57a4(
            CONCAT22(0x1050, local_12),
            param_1 & 0xffff0000 | (iVar6 + 0xd2),
        );
        loop {
            puVar3 = local_12;
            pass1_1008_5b12(CONCAT22(0x1050, puVar3));
            if ((extraout_DX | puVar3) == 0) {
                break;
            }
            uVar2 = (puVar3 + 0x4);
            uVar7 = (puVar3 + 0x6);
            puVar4 = uVar2;
            ppcVar1 = (*puVar4 + 0x90);
            (**ppcVar1)(0x1008, puVar4, uVar7, 0x1, uVar2);
        }
    }
    return;
}

pub unsafe fn pass1_1020_8288(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    param_1 = (param_1 & 0xffff0000 | (param_1 - 0xe2));
    pass1_1020_808e(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn FUN_1020_8296(mut param_1: u16, param_2: *mut StructD, param_3: u8) -> *mut StructD {
    pass1_1020_808e(param_2);
    if ((param_3 & 1) != 0) {
        fn_ptr_1000_17ce(param_2);
    }
    return param_2;
}

pub unsafe fn pass1_1020_83f8(mut param_1: u32) {
    let mut uVar1: u32;
    let mut uVar2: u32;
    let mut iVar3: i16;
    let mut uVar4: u16;

    uVar4 = (param_1 >> 0x10);
    iVar3 = param_1;
    if ((iVar3 + 0x4) != 0) {
        uVar1 = (iVar3 + 0x1c);
        uVar2 = (iVar3 + 0x1c);
        pass1_1008_4480(
            (uVar1 + 0xa),
            (param_1 & 0xffff0000 | (iVar3 + 0x16)),
            (uVar2 + 0x2a),
        );
    }
    return;
}

pub unsafe fn FUN_1020_8438() {
    return;
}

pub unsafe fn pass1_1020_843c(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    pass1_1020_8556(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1020_85f6(param_1: *mut astruct_590) {
    let mut piVar1: *mut i16;
    let mut uVar2: u16;
    let mut pcVar3: *mut c_char;
    let mut uVar4: u32;
    let mut iVar5: i16;
    let mut iVar6: *mut astruct_590;
    let mut uVar6: u16;
    let mut uVar7: u16;
    let mut iStack4: i16;

    iStack4 = 0;
    loop {
        uVar7 = (param_1 >> 0x10);
        iVar6 = param_1;
        piVar1 = &iVar6.field6_0x6;
        if (*piVar1 == iStack4 || *piVar1 < iStack4) {
            break;
        }
        uVar4 = iVar6.field11_0xc;
        uVar6 = (uVar4 >> 0x10);
        iVar5 = uVar4;
        pcVar3 = *(iVar5 + iStack4 * 0x4);
        uVar2 = (iVar5 + iStack4 * 0x4 + 2);
        if ((uVar2 | pcVar3) != 0) {
            pass1_1008_5118(pcVar3 & 0xffff | uVar2 << 0x10);
            fn_ptr_1000_17ce(pcVar3);
        }
        uVar4 = iVar6.field11_0xc;
        (uVar4 + iStack4 * 0x4) = 0;
        iStack4 += 0x1;
    }
    return;
}

pub unsafe fn pass1_1020_865a(mut param_1: u32) {
    let mut piVar1: *mut i16;
    let mut uVar2: u16;
    let mut pcVar3: *mut c_char;
    let mut uVar4: u32;
    let mut iVar5: i16;
    let mut iVar7: *mut astruct_592;
    let mut iVar6: *mut astruct_591;
    let mut iVar8: i16;
    let mut uVar9: u16;
    let mut uVar10: u16;
    let mut iStack4: i16;

    iStack4 = 0;
    loop {
        uVar9 = (param_1 >> 0x10);
        iVar5 = param_1;
        piVar1 = (iVar5 + 0x6);
        if (*piVar1 == iStack4 || *piVar1 < iStack4) {
            break;
        }
        iVar8 = iStack4 * 0x4;
        uVar4 = (iVar5 + 0xc);
        uVar10 = (uVar4 >> 0x10);
        iVar7 = uVar4;
        if ((iVar7 + iVar8) != 0) {
            pass1_1008_5236((iVar7 + iVar8));
            uVar4 = (iVar5 + 0xc);
            uVar10 = (uVar4 >> 0x10);
            iVar6 = uVar4;
            pcVar3 = *(iVar6 + iVar8);
            uVar2 = (iVar6 + iVar8 + 2);
            if ((uVar2 | pcVar3) != 0) {
                pass1_1008_5118(pcVar3 & 0xffff | uVar2 << 0x10);
                fn_ptr_1000_17ce(pcVar3);
            }
            uVar4 = (iVar5 + 0xc);
            (uVar4 + iStack4 * 0x4) = 0;
        }
        iStack4 += 0x1;
    }
    return;
}


pub unsafe fn pass1_1020_86d8(mut param_1: u32) {
    let mut piVar1: *mut i16;
    let mut uVar2: u32;
    let mut iVar3: i16;
    let mut uVar4: u16;
    let mut iStack4: i16;

    iStack4 = 0;
    loop {
        uVar4 = (param_1 >> 0x10);
        piVar1 = (param_1 + 0x6);
        if (*piVar1 == iStack4 || *piVar1 < iStack4) {
            break;
        }
        uVar2 = (param_1 + 0xc);
        uVar4 = (uVar2 >> 0x10);
        iVar3 = uVar2;
        if ((iVar3 + iStack4 * 0x4) != 0) {
            pass1_1008_5236((iVar3 + iStack4 * 0x4));
        }
        iStack4 += 0x1;
    }
    return;
}

pub unsafe fn FUN_1020_8780() {
    return;
}


pub unsafe fn pass1_1020_8784(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    pass1_1020_8556(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1020_8908(param_1: *mut astruct_284, mut param_2: u32, mut param_3: u32) {
    let mut paVar1: *mut astruct_76;
    let mut uVar2: u32;
    let mut uVar3: u16;
    let mut uVar4: u16;
    let mut puVar5: *mut u8;
    let mut uVar6: u16;
    let mut uVar7: u16;
    let mut pstruct284_8: *mut astruct_284;
    let mut iVar9: i16;
    let mut iVar10: i16;
    let mut uVar11: u16;
    let mut uVar12: u16;
    let mut uVar13: u32;
    let mut paStack28: *mut astruct_110;
    let mut iStack4: i16;
    let mut paVar8: *mut Struct57;

    //   for (iStack4 = 0; uVar12 = (param_3 >> 0x10), pstruct284_8 = param_1,
    //   uVar11 = (param_1 >> 0x10), iStack4 < 0x4; iStack4 += 1)
    iStack4 = 0;
    uVar12 = param_3 >> 0x10;
    pstruct284_8 = param_1;
    uVar11 = param_1 >> 0x10;
    while iStack4 < 4 {
        if (pstruct284_8.field4_0x4 == 0) {
            uVar2 = pstruct284_8.field11_0xc;
            uVar12 = (uVar2 >> 0x10);
            iVar10 = uVar2;
            iVar9 = iStack4 * 0x4;
            if (((iVar10 + iVar9 + 0x2) | (iVar10 + iVar9)) != 0) {
                pass1_1008_5236((iVar10 + iVar9));
            }
        } else {
            uVar2 = pstruct284_8.field42_0x2e;
            paVar1 = (uVar2 + 0x2e + iStack4 * 0x4);
            uVar13 = pass1_1008_4772(paVar1);
            uVar6 = (uVar13 >> 0x10);
            paVar8 = CONCAT22(uVar12, uVar6);
            uVar3 = uVar13;
            uVar2 = pstruct284_8.field11_0xc;
            iVar10 = iStack4 * 0x4;
            if ((uVar2 + iVar10) == 0) {
                uVar4 = uVar3;
                mem_op_1000_179c(0x14, paVar8);
                uVar7 = paVar8;
                paStack28 = CONCAT22(uVar7, uVar4);
                paVar8 = (paVar8 & 0xffff0000);
                if ((uVar7 | uVar4) == 0) {
                    uVar2 = pstruct284_8.field11_0xc;
                    (uVar2 + iStack4 * 0x4) = 0;
                } else {
                    paVar8 = (paVar8 | uVar11);
                    puVar5 = &pstruct284_8.field_0x16 + iStack4 * 0x6;
                    pass1_1008_50c2(
                        paStack28,
                        (uVar3 + 0x8),
                        (uVar3 + 0x4),
                        (param_1 & 0xffff0000 | ZEXT24(puVar5)),
                        param_2,
                    );
                    uVar2 = pstruct284_8.field11_0xc;
                    uVar12 = (uVar2 >> 0x10);
                    iVar9 = uVar2;
                    (iVar9 + iVar10) = puVar5;
                    (iVar9 + iVar10 + 0x2) = paVar8;
                }
                uVar2 = pstruct284_8.field11_0xc;
                pass1_1008_5134((uVar2 + iStack4 * 0x4));
            }
            uVar12 = (paVar8 >> 0x10);
            uVar2 = pstruct284_8.field11_0xc;
            pass1_1008_5236((uVar2 + iStack4 * 0x4));
            param_3 = CONCAT22(uVar12, uVar11);
            pass1_1008_4480(
                param_2,
                (param_1 & 0xffff0000 | ZEXT24(&pstruct284_8.field_0x16 + iStack4 * 0x6)),
                paVar1,
            );
        }
        iStack4 += 1;
    }
    if (pstruct284_8.field4_0x4 != 0) {
        pass1_1008_4480(
            param_2,
            (param_1 & 0xffff0000 | ZEXT24(&pstruct284_8.field_0x32)),
            pstruct284_8.field49_0x38,
        );
    }
    return;
}
