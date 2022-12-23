use crate::block_1020::block_1020_8000::{invalidate_rect_1020_8d90, pass1_1020_8556, pass1_1020_8bae};
use crate::block_1020::block_1020_9000::{palette_op_1020_92c4, pass1_1020_915a};
use crate::block_1020::block_1020_c000::{pass1_1020_c47a, pass1_1020_c6a4, pass1_1020_c6de, pass1_1020_cac2, pass1_1020_cefc};
use crate::block_1020::block_1020_d000::pass1_1020_d194;

pub unsafe fn pass1_1020_8a5e(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    pass1_1020_8556(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1020_8bcc(
    param_1: *mut astruct_285,
    mut param_2: u32,
    mut param_3: u16,
    mut param_4: u16,
) {
    let mut uVar1: u32;
    let mut paVar2: *mut astruct_109;
    let mut puVar3: *mut u16;
    let mut puVar4: *mut u16;
    let mut uVar5: u16;
    let mut uVar6: u16;
    let mut paVar7: *mut Struct57;
    let mut iVar9: *mut astruct_285;
    let mut iVar10: *mut astruct_286;
    let mut uVar9: u16;
    let mut uVar10: u16;
    let mut local_58: [u8; 0x1e] = [0; 0x1e];
    let mut local_3a: [u8; 0x26] = [0; 0x26];
    let mut uStack20: u32;
    let mut uStack12: u16;
    let mut paStack10: *mut astruct_76;
    let mut paStack6: *mut astruct_76;
    let mut paVar8: *mut Struct57;

    uVar10 = (param_2 >> 0x10);
    uVar9 = (param_1 >> 0x10);
    iVar9 = param_1;
    if (iVar9.field4_0x4 != 0) {
        uVar1 = iVar9.field30_0x22;
        paStack6 = (uVar1 + 0xa);
        paStack10 = pass1_1018_268e(iVar9.field30_0x22);
        uVar6 = (paStack10 >> 0x10);
        uVar1 = iVar9.field30_0x22;
        uStack12 = (uVar1 + 0x16);
        if (iVar9.field11_0xc == 0) {
            uStack20 = pass1_1008_4772(paStack10);
            paVar7 = CONCAT22(uVar10, (uStack20 >> 0x10));
            uVar6 = uStack20;
            mem_op_1000_179c(0x14, paVar7);
            uVar5 = paVar7 | uVar6;
            paVar8 = (paVar7 & 0xffff0000 | uVar5);
            if (uVar5 == 0) {
                iVar9.field11_0xc = 0;
            } else {
                puVar3 = (param_1 & 0xffff0000 | ZEXT24(&iVar9.field_0x16));
                uVar10 = (uStack20 >> 0x10);
                pass1_1008_50c2(
                    CONCAT22(paVar7, uVar6),
                    (uStack20 + 0x8),
                    (uStack20 + 0x4),
                    puVar3,
                    paStack6,
                );
                paVar2 = iVar9.field11_0xc;
                uVar6 = puVar3;
                paVar2 = uVar6;
                (paVar2 + 0x2) = paVar8;
            }
            pass1_1008_5134(iVar9.field11_0xc);
            uVar10 = FUN_1010_830a(uVar6, paVar8, 0x1008, _u16_1050_14cc, 0x2);
            struct_op_1008_48fe(
                paVar8,
                CONCAT22(0x1050, local_3a),
                0x1,
                CONCAT22(paVar8, uVar10),
            );
            uVar10 = (paVar8 >> 0x10);
            struct_op_1008_3f92(CONCAT22(0x1050, local_58), CONCAT22(0x1050, local_3a));
            uStack20 = pass1_1008_4772(CONCAT22(0x1050, local_58));
            paVar7 = CONCAT22(uVar10, (uStack20 >> 0x10));
            uVar6 = uStack20;
            mem_op_1000_179c(0x14, paVar7);
            uVar6 = paVar7 | uVar6;
            if (uVar6 == 0) {
                paVar2 = iVar9.field11_0xc;
                (paVar2 + 0x4) = 0;
            } else {
                puVar3 = (param_1 & 0xffff0000 | ZEXT24(&iVar9.field_0x16));
                uVar10 = (uStack20 >> 0x10);
                pass1_1008_50c2(
                    CONCAT22(paVar7, (paStack6 >> 0x10)),
                    (uStack20 + 0x8),
                    (uStack20 + 0x4),
                    puVar3,
                    paStack6,
                );
                paVar2 = iVar9.field11_0xc;
                uVar10 = (paVar2 >> 0x10);
                iVar10 = paVar2;
                iVar10.field4_0x4 = puVar3;
                iVar10.field5_0x6 = uVar6;
            }
            paVar2 = iVar9.field11_0xc;
            pass1_1008_5134((paVar2 + 0x4));
            pass1_1008_41bc(CONCAT22(0x1050, local_58));
            close_file_1008_496c(CONCAT22(0x1050, local_3a));
        }
        paVar2 = iVar9.field11_0xc;
        pass1_1008_5236((paVar2 + 0x4));
        pass1_1008_5236(iVar9.field11_0xc);
        puVar4 = (param_1 & 0xffff0000 | ZEXT24(&iVar9.field_0x16));
        pass1_1008_4480(paStack6, puVar4, paStack10);
        invalidate_rect_1020_8d90(puVar4, uVar6, param_1, uStack12, paStack6, param_3, param_4);
    }
    return;
}

pub unsafe fn pass1_1020_8e6c(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    pass1_1020_8bae(&param_1.address_offset_field_0x0);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn invalidate_rect_1020_8fb4(mut param_1: u16, mut param_2: u16, mut param_3: u32) {
    let mut uVar1: u32;
    let mut rect: *mut RECT16;
    let mut uVar2: u32;
    let mut extraout_DX: u16;
    let mut hwnd: u16;
    let mut iVar3: i16;
    let mut uVar4: u16;
    let mut iStack8: i16;

    uVar4 = (param_3 >> 0x10);
    iVar3 = param_3;
    uVar1 = (iVar3 + 0xba);
    if ((uVar1 + 0x1e) != 0) {
        pass1_1018_2862((iVar3 + 0x16));
        (iVar3 + 0xaa) = param_1;
        (iVar3 + 0xac) = param_2;
        if ((param_2 | (iVar3 + 0xaa)) != 0) {
            uVar1 = (iVar3 + 0xaa);
            iVar3 = (uVar1 + 0xa);
            for iStack8 in 0..iVar3 {
                uVar2 = iStack8;
                empty_1008_8fc4();
                rect = uVar2;
                hwnd = extraout_DX | rect;
                if (((hwnd != 0) && (0x9 < rect[0xb].y)) && (
                    pass1_1008_8b20(uVar2 & 0xffff | extraout_DX << 0x10),
                    (hwnd | rect) != 0,
                )) {
                    InvalidateRect16(0x0, rect, hwnd);
                }
            }
        }
    }
    return;
}

pub unsafe fn pass1_1020_9068(mut param_1: u32, param_2: *mut u32, mut param_3: u32) {
    let mut iVar1: i16;
    let mut paVar2: *mut astruct_76;
    let mut ppcVar3: *mut *mut code;
    let mut uVar4: u32;
    let mut uVar5: u16;
    let mut paVar6: *mut astruct_76;
    let mut uVar7: u32;
    let mut uVar8: u16;
    let mut iVar10: i16;
    let mut iVar11: i16;
    let mut uVar12: u16;
    let mut uVar13: u16;
    let mut iStack10: i16;
    let mut uVar9: u32;

    uVar12 = (param_2 >> 0x10);
    iVar10 = param_2;
    uVar4 = (iVar10 + 0x16);
    paVar2 = (uVar4 + 0xa);
    paVar6 = paVar2;
    pass1_1018_280c((iVar10 + 0x16));
    (iVar10 + 0xaa) = paVar6;
    (iVar10 + 0xac) = param_1;
    uVar5 = param_1 | (iVar10 + 0xaa);
    if (uVar5 == 0) {
        pass1_1018_2862((iVar10 + 0x16));
        (iVar10 + 0xaa) = uVar5;
        (iVar10 + 0xac) = param_1;
    }
    if (((iVar10 + 0xac) | (iVar10 + 0xaa)) != 0) {
        pass1_1020_915a(param_1, (param_2 & 0xffff | uVar12 << 0x10), param_3);
        pass1_1008_4480(
            paVar2,
            (param_2 & 0xffff0000 | (iVar10 + 0xae)),
            (iVar10 + 0xb4),
        );
        ppcVar3 = (*param_2 + 0x10);
        (**ppcVar3)();
        uVar4 = (iVar10 + 0xaa);
        iVar1 = (uVar4 + 0xa);
        for iStack10 in 0..iVar1 {
            uVar7 = iStack10;
            empty_1008_8fc4();
            uVar5 = uVar7;
            uVar8 = param_1 | uVar5;
            uVar9 = param_1 & 0xffff0000 | uVar8;
            if (uVar8 != 0) {
                pass1_1008_8c4e(uVar7 & 0xffff | param_1 << 0x10, paVar2, uVar9);
                uVar4 = (iVar10 + 0xc);
                uVar13 = (uVar4 >> 0x10);
                iVar11 = uVar4;
                (iVar11 + iStack10 * 0x4) = uVar5;
                (iVar11 + iStack10 * 0x4 + 0x2) = uVar9;
            }
            param_1 = uVar9;
        }
    }
    return;
}

pub unsafe fn pass1_1020_91de(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    pass1_1020_8f74(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn mix_draw_op_1020_9312(mut param_1: u32) {
    let mut puVar1: *mut u32;
    let mut ppcVar2: *mut *mut code;
    let mut uVar3: u32;
    let mut iVar4: i16;
    let mut uVar5: u16;
    let mut local_22: [u8; 0x20] = [0; 0x20];

    uVar5 = (param_1 >> 0x10);
    iVar4 = param_1;
    BeginPaint16(CONCAT22(0x1050, local_22), (iVar4 + 0x4));
    uVar3 = (iVar4 + 0x6);
    puVar1 = (uVar3 + 0xa);
    ppcVar2 = (*puVar1 + 0x4);
    (**ppcVar2)(
        s_tile2_bmp_1050_1538,
        puVar1,
        (puVar1 >> 0x10),
        0x0,
        param_1 & 0xffff0000 | (iVar4 + 0xa),
    );
    EndPaint16(CONCAT22(0x1050, local_22), (iVar4 + 0x4));
    return;
}

pub unsafe fn pass1_1020_96a2(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    palette_op_1020_92c4(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1020_ba2b() {
    init_globals_1020_96d4();
    pass1_1020_a426();
    return;
}

pub unsafe fn pass1_1020_bd6c(mut param_1: u32, mut param_2: u32) -> i16 {
    return (param_1 + 0x4) - (param_2 + 0x4);
}

pub unsafe fn pass1_1020_c538(mut param_1: u32) -> u32 {
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    return CONCAT22((param_1 + 0x12), (param_1 + 0x10));
}

pub unsafe fn pass1_1020_c54a(mut param_1: u32) {
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    if ((param_1 + 0x1c) != 0) {
        pass1_1020_c6a4((param_1 & 0xffff | uVar1 << 0x10));
    }
    return;
}

pub unsafe fn FUN_1020_c5ae() -> u16 {
    return 0x0;
}

pub unsafe fn FUN_1020_c5b4(mut param_1: u16, param_2: *mut u32, mut param_3: u32) {
    let mut plVar1: *mut i32;
    let mut ppcVar2: *mut *mut code;
    let mut in_AX: u16;
    let mut uVar3: u16;
    let mut uVar4: u16;
    let mut in_DX: u16;
    let mut extraout_DX: u16;
    let mut uVar5: u16;
    let mut uVar6: u16;
    let mut puStack12: *mut u32;

    pass1_1028_e1ec(_PTR_LOOP_1050_65e2, param_3);
    uVar3 = pass1_1030_6fa0(CONCAT22(in_DX, in_AX));
    uVar4 = uVar3;
    pass1_1020_c6de(param_2, 0x0);
    puStack12 = CONCAT22(in_DX, uVar4);
    uVar6 = (param_2 >> 0x10);
    if ((in_DX | uVar4) == 0) {
        ppcVar2 = (*param_2 + 0x20);
        (**ppcVar2)();
        uVar5 = extraout_DX;
        pass1_1020_c6de(param_2, 0x0);
        puStack12 = CONCAT22(uVar5, uVar4);
        if ((uVar5 | uVar4) == 0) {
            return;
        }
    }
    (param_2 + 0x1c) = 0x1;
    plVar1 = (param_2 + 0x8);
    *plVar1 = *plVar1 + 1;
    *puStack12 = param_3;
    (puStack12 + 0x4) = (uVar3 * 0x2 + 0x4ea4);
    return;
}

pub unsafe fn FUN_1020_c640() {
    return;
}

pub unsafe fn pass1_1020_c644(param_1: u32, mut param_2: u16, mut param_3: u32) {
    let mut plVar1: *mut i32;
    let mut uVar2: u16;
    let mut ppcVar3: *mut *mut code;
    let mut iVar4: i16;
    let mut iVar5: i16;
    let mut uVar6: u16;
    let mut puStack6: *mut u32;

    uVar6 = (param_1 >> 0x10);
    iVar5 = param_1;
    if ((iVar5 + 0x18) == 0) {
        ppcVar3 = (*param_1 + 0x20);
        (**ppcVar3)();
    }
    iVar4 = (iVar5 + 0x8) * 0x6 + (iVar5 + 0x18);
    uVar2 = (iVar5 + 0x1a);
    puStack6 = CONCAT22(uVar2, iVar4);
    plVar1 = (iVar5 + 0x8);
    *plVar1 = *plVar1 + 1;
    *puStack6 = param_3;
    (iVar4 + 0x4) = param_2;
    return;
}

pub unsafe fn pass1_1020_c694(mut param_1: u32) {
    pass1_1020_c6a4(param_1);
    return;
}

pub unsafe fn pass1_1020_c73a(mut param_1: u32) {
    let mut puVar1: *mut u16;
    let mut uVar2: u16;
    let mut uVar3: u32;
    let mut uVar4: u16;
    let mut in_EDX: u32;
    let mut pSVar5: *mut StructD;
    let mut iVar6: i16;
    let mut uVar7: u16;
    let mut lVar8: i32;
    let mut uStack10: u32;
    let mut uStack6: u32;

    uVar7 = (param_1 >> 0x10);
    iVar6 = param_1;
    if ((iVar6 + 0x10) == 0) {
        uVar4 = (iVar6 + 0xc);
        pSVar5 = (in_EDX & 0xffff0000 | (iVar6 + 0xe));
    } else {
        uVar2 = (iVar6 + 0x10);
        puVar1 = (iVar6 + 0x14);
        uVar4 = uVar2 + *puVar1;
        pSVar5 = (in_EDX & 0xffff0000 | ((iVar6 + 0x12) + (iVar6 + 0x16) + CARRY2(uVar2, *puVar1)));
    }
    uStack6 = CONCAT22(pSVar5, uVar4);
    if ((iVar6 + 0x18) == 0) {
        if (_PTR_LOOP_1050_5f2c == 0) {
            PTR_LOOP_1050_5f2c = mem_op_1000_160a(pSVar5);
            PTR_LOOP_1050_5f2e = pSVar5;
        } else {}
        uVar4 = fn_ptr_op_1000_1708(
            uVar4 * 0x6,
            0x0,
            0x1,
            PTR_LOOP_1050_5f2c,
            PTR_LOOP_1050_5f2e,
        );
    } else {
        uVar3 = (iVar6 + 0x18);
        lVar8 = pass1_1000_0ed4(
            0x1,
            uVar4 * 0x6,
            (pSVar5 * 0x3 + CARRY2(uVar4, uVar4) + CARRY2(uVar4 * 0x2, uVar4)) * 0x2 + CARRY2(uVar4 * 0x3, uVar4 * 0x3),
            uVar3,
            (uVar3 >> 0x10),
        );
        PTR_LOOP_1050_5f2e = (lVar8 >> 0x10);
        uVar4 = lVar8;
    }
    uStack10 = CONCAT22(PTR_LOOP_1050_5f2e, uVar4);
    if ((PTR_LOOP_1050_5f2e | uVar4) != 0) {
        (iVar6 + 0x10) = uStack6;
        (iVar6 + 0x18) = uStack10;
    }
    (iVar6 + 0x1c) = 0x1;
    return;
}

pub unsafe fn pass1_1020_c7fa(mut param_1: u32, mut param_2: u32) -> i16 {
    return (param_1 + 0x4) - (param_2 + 0x4);
}

pub unsafe fn pass1_1020_c80e(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    pass1_1020_c47a(&param_1.address_offset_field_0x0);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1020_c9ba(param_1: *mut u16, param_2: u8) -> *mut u16 {
    *param_1 = 0x389a;
    (param_1 + 0x2) = 0x1008;
    if ((param_2 & 1) != 0) {
        pass1_1000_093a(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1020_ca36(mut param_1: u16, param_2: *mut astruct_15, mut param_3: u16) {
    let mut uVar1: u16;
    let mut in_EDX: u32;
    let mut uVar2: u16;
    let mut uVar3: u32;
    let mut puVar4: *mut u32;
    let mut in_stack_0000fe96: u16;
    let mut in_stack_0000ffba: u16;
    let mut in_stack_0000ffc0: u16;
    let mut in_stack_0000ffc4: u16;

    uVar2 = (in_EDX >> 0x10);
    pass1_1028_09b8(param_2);
    uVar3 = pass1_1028_b4f2(param_2);
    uVar1 = (uVar3 >> 0x10);
    if ((uVar3 + 0x200) != 0x8000002) {
        puVar4 = mixed_1010_20ba(
            CONCAT22(uVar2, uVar1),
            _u16_1050_0ed0,
            CONCAT22(param_1, 0x8),
            in_stack_0000fe96,
            in_stack_0000ffba,
            in_stack_0000ffc0,
            in_stack_0000ffc4,
        );
        pass1_1010_988c(puVar4, (param_2 + 0xc));
    }
    return;
}

pub unsafe fn pass1_1020_ca82(param_1: *mut astruct_15) {
    let mut uVar1: u16;
    let mut uVar3: u32;
    let mut in_stack_0000ffb4: u16;

    pass1_1028_be9e(param_1);
    uVar3 = pass1_1028_b4f2(param_1);
    uVar3 = (uVar3 >> 0x10);
    if ((uVar3 + 0x200) != 0x8000002) {
        uVar1 = (param_1 >> 0x10);
        if ((param_1 + 0x12) == 0x5) {
            pass1_1020_cac2(uVar3, in_stack_0000ffb4, (param_1 & 0xffff | uVar1 << 0x10));
        }
    }
    return;
}

pub unsafe fn pass1_1020_cc56(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    pass1_1028_b418(&param_1.address_offset_field_0x0);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1020_cd30(mut param_1: u32) -> u16 {
    let mut iVar1: i16;
    let mut uVar2: u16;

    uVar2 = (param_1 >> 0x10);
    iVar1 = param_1;
    if ((((iVar1 + 0x12) == 0x6) || ((iVar1 + 0x12) == 0x5)) && ((*(iVar1 + 0x1a) & 0x2) != 0)) {
        return 0x1;
    }
    return 0x0;
}

pub unsafe fn pass1_1020_cd58(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    pass1_1028_b418(&param_1.address_offset_field_0x0);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1020_ce9e(mut param_1: u16, param_2: *mut astruct_15) {
    let mut uVar1: u16;
    let mut in_EDX: u32;
    let mut uVar2: u16;
    let mut uVar3: u32;
    let mut paVar4: *mut Struct27;
    let mut in_stack_0000fe92: u16;
    let mut in_stack_0000ffb6: u16;
    let mut in_stack_0000ffbc: u16;
    let mut in_stack_0000ffc0: u16;

    uVar2 = (in_EDX >> 0x10);
    pass1_1028_be9e(param_2);
    if ((param_2 + 0x12) == 0x5) {
        pass1_1020_cefc(param_2);
        uVar3 = pass1_1028_b4f2(param_2);
        uVar1 = (uVar3 >> 0x10);
        if ((uVar3 + 0x200) != 0x8000002) {
            paVar4 = mixed_1010_20ba(
                CONCAT22(uVar2, uVar1),
                _u16_1050_0ed0,
                CONCAT22(param_1, 0x2b),
                in_stack_0000fe92,
                in_stack_0000ffb6,
                in_stack_0000ffbc,
                in_stack_0000ffc0,
            );
            pass1_1010_043a(paVar4, (uVar3 + 0x4), 0x5);
        }
    }
    return;
}

pub unsafe fn pass1_1020_cfde(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    pass1_1028_b418(&param_1.address_offset_field_0x0);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1020_d0b8(param_1: *mut astruct_15) {
    let mut uVar1: u32;
    let mut iVar2: i16;

    if ((param_1 + 0x12) != 0x6) {
        return;
    }
    uVar1 = pass1_1028_b4f2(param_1);
    iVar2 = uVar1;
    if ((iVar2 + 0x200) != 0x8000002) {
        pass1_1028_cb04(param_1);
        if ((iVar2 == 0) || (pass1_1020_d194(param_1), iVar2 == 0)) {
            iVar2 = 0x6;
            // TODO: goto LAB_1020_d10b;
        }
        pass1_1028_c952(param_1);
    }
    iVar2 = 0x5; //
    // LAB_1020_d10b:
    pass1_1028_bdac(param_1, iVar2);
    return;
}


pub unsafe fn pass1_1020_d118(
    mut param_1: i16,
    mut param_2: u16,
    mut param_3: u32,
    param_4: *mut u16,
    mut param_5: u32,
    mut param_6: u32,
) -> u16 {
    let mut BVar1: bool;
    let mut uVar2: u16;
    let mut uVar3: u16;

    uVar2 = param_3;
    uVar3 = (param_3 >> 0x10);
    pass1_1028_c7b6(param_2, uVar2, uVar3, param_4, param_6);
    if (param_1 == 0x5) {
        pass1_1028_c23e(0x5, param_2, uVar2, uVar3, param_4, param_5, param_6);
        if (param_1 != 0) {
            pass1_1028_c3aa(uVar2, uVar3, param_4, param_5, param_6);
            if (param_1 != 0) {
                BVar1 = pass1_1028_c314(
                    param_1,
                    param_2,
                    uVar2,
                    uVar3,
                    param_4,
                    param_5,
                    (param_5 >> 0x10),
                    param_6,
                );
                if (BVar1 != 0) {
                    return 0x1;
                }
            }
        }
    } else {
        PTR_LOOP_1050_50ca = 0x6a8;
    }
    return 0x0;
}

pub unsafe fn pass1_1020_d2ee(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    pass1_1028_b418(&param_1.address_offset_field_0x0);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn write_to_file_1020_d3d4(param_1: *mut astruct_731, param_2: *mut u8) -> BOOL16 {
    let mut BVar1: bool;
    let mut in_stack_0000ffde: HFILE16;
    let mut local_c: [u16; 0x5] = [0; 0x5];

    BVar1 = write_to_file_1028_b5ec(param_1, param_2);
    if (BVar1 != 0) {
        local_c[0] = (param_1 + 0x20);
        BVar1 = write_to_file_1008_7e1c(param_2, CONCAT22(0x1050, local_c), 0x2, in_stack_0000ffde);
        if (BVar1 == 0) {
            u16_1050_0310 = 0x6d0;
            return BVar1;
        }
        BVar1 = 0x1;
    }
    return BVar1;
}

pub unsafe fn pass1_1020_d41a(
    param_1: BOOL16,
    param_2: *mut u8,
    param_3: *mut astruct_373,
    param_4: *mut HFILE16,
) -> BOOL16 {
    let mut BVar1: bool;
    let mut local_4: u16;

    file_1028_b81a(param_1, param_2, param_3, param_4);
    if (param_1 != 0) {
        BVar1 = read_file_1008_7dee(param_4, CONCAT22(0x1050, &local_4), 0x2);
        if (BVar1 == 0) {
            u16_1050_0310 = 0x6d2;
            return BVar1;
        }
        (param_3 + 0x20) = local_4;
        param_1 = 0x1;
    }
    return param_1;
}


pub unsafe fn pass1_1020_d460(
    mut param_1: i16,
    mut param_2: u16,
    param_3: *mut u32,
    param_4: *mut u16,
    mut param_5: u32,
    mut param_6: u32,
) -> u16 {
    let mut uVar1: u16;
    let mut in_register_0000000a: u16;
    let mut paVar2: *mut Struct57;
    let mut uVar3: u32;
    let mut puVar4: *mut u32;
    let mut in_stack_0000fe90: u16;
    let mut in_stack_0000ffb4: u16;
    let mut in_stack_0000ffba: u16;
    let mut in_stack_0000ffbe: u16;
    let mut in_stack_0000ffe8: u16;

    uVar1 = pass1_1028_bc90(param_1, param_2, param_3, param_4, param_5, param_6);
    if (uVar1 != 0) {
        uVar3 = pass1_1038_af40(
            _PTR_LOOP_1050_4230,
            param_2,
            _PTR_LOOP_1050_5b7c,
            (_PTR_LOOP_1050_4230 + 0x16),
            0x11,
        );
        paVar2 = CONCAT22(in_register_0000000a, (uVar3 >> 0x10));
        PTR_LOOP_1050_5b80 = (&PTR_LOOP_1050_0000 + 1);
        unk_win_msg_op_1008_9510(&PTR_LOOP_1050_5b80);
        puVar4 = mixed_1010_20ba(
            paVar2,
            _u16_1050_0ed0,
            CONCAT22(in_stack_0000ffe8, 0x3a),
            in_stack_0000fe90,
            in_stack_0000ffb4,
            in_stack_0000ffba,
            in_stack_0000ffbe,
        );
        (param_3 + 0x20) = (puVar4 + 0xa);
        uVar1 = 0x1;
    }
    return uVar1;
}


pub unsafe fn pass1_1020_d4ca(mut param_1: i16, param_2: *mut astruct_15) {
    let mut BVar1: bool;
    let mut uVar2: u32;
    let mut extraout_DX: u16;
    let mut uVar3: u16;
    let mut iVar4: i16;

    if ((param_2 + 0x20) == 0x2) {
        return;
    }
    pass1_1028_b58e(param_2);
    uVar2 = (param_1 + 0x2e);
    iVar4 = 0x63;
    uVar3 = extraout_DX;
    pass1_1038_3fb0(uVar2);
    BVar1 = pass1_1030_25b2(uVar2 & 0xffff | uVar3 << 0x10, iVar4);
    if (BVar1 != 0) {
        return;
    }
    return;
}

pub unsafe fn pass1_1020_d518(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    pass1_1028_b418(&param_1.address_offset_field_0x0);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1020_d5f2(mut param_1: i16, param_2: *mut astruct_15, mut param_3: u32) {
    let mut ppcVar1: *mut *mut code;
    let mut uVar2: u16;
    let mut puVar3: *mut u32;
    let mut extraout_DX: u16;
    let mut uVar4: u16;
    let mut uVar5: u32;
    let mut puVar6: *mut u32;
    let mut puVar7: *mut u32;
    let mut bStack55: u8;
    let mut local_32: [u8; 0xa] = [0; 0xa];
    let mut uStack40: u32;
    let mut uStack36: u32;
    let mut puStack28: *mut u32;
    let mut local_1a: u32;
    let mut iStack22: i16;
    let mut uStack20: u16;
    let mut iStack18: i16;
    let mut uStack16: u16;
    let mut iStack14: i16;
    let mut local_c: u32;
    let mut iStack8: i16;
    let mut iStack6: i16;
    let mut uStack4: u16;

    pass1_1028_b58e(param_2);
    local_c = (param_1 + 0xc);
    iStack18 = (param_1 + 0x10);
    puStack28 = &local_c;
    uStack16 = extraout_DX;
    iStack14 = iStack18;
    iStack8 = iStack18;
    iStack6 = param_1;
    uStack4 = extraout_DX;
    pass1_1028_bab6(iStack18, extraout_DX, param_2);
    uVar2 = pass1_1030_2fac(CONCAT22(uStack16, iStack18));
    local_1a = local_c;
    iStack22 = iStack8;
    uStack36 = CONCAT22(uStack36, &local_1a);
    iStack14 += 0x1;
    uStack20 = uVar2;
    if (iStack14 < uVar2) {
        puVar7 = CONCAT22(0x1050, local_32);
        iStack22 = iStack14;
        uVar5 = pass1_1028_bb24(param_2);
        uVar4 = (uVar5 >> 0x10);
        puVar3 = &local_1a;
        pass1_1030_64ce(
            puVar3,
            uVar4,
            _PTR_LOOP_1050_5740,
            CONCAT22(0x1050, puVar3),
            uVar5 & 0xffff | uVar4 << 0x10,
            puVar7,
        );
        uStack40 = *puVar3;
        uVar4 = (puVar3 + 2);
        bStack55 = (uStack40 >> 0x18);
        uVar2 = bStack55;
        uStack36 = uStack40;
        if (bStack55 != 0) {
            pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uStack40 & 0xffff | uVar4 << 0x10);
            puVar6 = struct_op_1030_73a8(CONCAT22(uVar4, uVar2), uVar2, uVar4);
            uVar2 = puVar6;
            ppcVar1 = (*puVar6 + 0x58);
            (**ppcVar1)();
        }
    }
    pass1_1028_b46e(uVar2, param_2, param_3);
    return;
}

pub unsafe fn pass1_1020_d6e6(mut param_1: i16, param_2: *mut astruct_15) {
    let mut ppcVar1: *mut *mut code;
    let mut puVar2: *mut u32;
    let mut uVar3: u16;
    let mut extraout_DX: u16;
    let mut uVar4: u16;
    let mut uVar5: u32;
    let mut puVar6: *mut u32;
    let mut puVar7: *mut u32;
    let mut bStack55: u8;
    let mut local_32: [u8; 0xa] = [0; 0xa];
    let mut uStack40: u32;
    let mut uStack36: u32;
    let mut puStack28: *mut u32;
    let mut local_1a: u32;
    let mut iStack22: i16;
    let mut uStack20: u16;
    let mut iStack18: i16;
    let mut uStack16: u16;
    let mut iStack14: i16;
    let mut local_c: u32;
    let mut iStack8: i16;
    let mut iStack6: i16;
    let mut uStack4: u16;

    pass1_1028_b514(param_2);
    pass1_1028_b58e(param_2);
    local_c = (param_1 + 0xc);
    iStack18 = (param_1 + 0x10);
    puStack28 = &local_c;
    uStack16 = extraout_DX;
    iStack14 = iStack18;
    iStack8 = iStack18;
    iStack6 = param_1;
    uStack4 = extraout_DX;
    pass1_1028_bab6(iStack18, extraout_DX, param_2);
    uStack20 = pass1_1030_2fac(CONCAT22(uStack16, iStack18));
    local_1a = local_c;
    uStack36 = CONCAT22(uStack36, &local_1a);
    iStack22 = iStack14 + 1;
    if (iStack22 < uStack20) {
        puVar7 = CONCAT22(0x1050, local_32);
        iStack14 = iStack22;
        uVar5 = pass1_1028_bb24(param_2);
        uVar4 = (uVar5 >> 0x10);
        puVar2 = &local_1a;
        pass1_1030_64ce(
            puVar2,
            uVar4,
            _PTR_LOOP_1050_5740,
            CONCAT22(0x1050, puVar2),
            uVar5 & 0xffff | uVar4 << 0x10,
            puVar7,
        );
        uStack40 = *puVar2;
        uVar4 = (puVar2 + 2);
        bStack55 = (uStack40 >> 0x18);
        uVar3 = bStack55;
        if (bStack55 != 0) {
            uStack36 = uStack40;
            pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uStack40 & 0xffff | uVar4 << 0x10);
            puVar6 = struct_op_1030_73a8(CONCAT22(uVar4, uVar3), uVar3, uVar4);
            if ((puVar6 + 0xc) == 0x6a) {
                ppcVar1 = (*puVar6 + 0x24);
                (**ppcVar1)();
            }
        }
    }
    return;
}


