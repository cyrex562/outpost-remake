
pub unsafe fn pass1_1030_7064(mut param_1: u32) {
    let mut uVar1: u32;
    let mut in_AX: u16;
    let mut in_DX: u16;
    let mut uVar2: u16;

    uVar2 = (param_1 >> 0x10);
    if ((param_1 + 0x1a) == 0) {
        struct_op_1030_73a8((param_1 & 0xffff | uVar2 << 0x10), in_AX, in_DX);
    }
    uVar1 = (param_1 + 0x1a);
    if ((uVar1 + 0x12) == 0x5) {
        return;
    }
    return;
}
pub unsafe fn pass1_1030_70ac(mut param_1: u32) {
    let mut uVar1: u32;
    let mut in_AX: u16;
    let mut in_DX: u16;
    let mut uVar2: u16;

    uVar2 = (param_1 >> 0x10);
    if ((param_1 + 0x1a) == 0) {
        struct_op_1030_73a8((param_1 & 0xffff | uVar2 << 0x10), in_AX, in_DX);
    }
    uVar1 = (param_1 + 0x1a);
    if ((uVar1 + 0x12) == 0x5) {
        return;
    }
    return;
}


pub unsafe fn pass1_1030_70f4(mut param_1: u32) {
    let mut uVar1: u16;
    let mut uVar2: u32;
    let mut in_AX: u16;
    let mut BVar3: bool;
    let mut in_DX: u16;
    let mut iVar4: i16;
    let mut uVar5: u16;
    let mut plVar6: *mut i32;

    uVar5 = (param_1 >> 0x10);
    iVar4 = param_1;
    if ((iVar4 + 0x1a) == 0) {
        struct_op_1030_73a8((param_1 & 0xffff | uVar5 << 0x10), in_AX, in_DX);
    }
    uVar2 = (iVar4 + 0x1a);
    uVar1 = (uVar2 + 0xc);
    BVar3 = pass1_1008_c6ae(_u16_1050_06e0, uVar1, 1);
    if (BVar3 == 0) {
        BVar3 = pass1_1008_c6ae(_u16_1050_06e0, uVar1, 0x2);
        if ((BVar3 == 0) || ((iVar4 + 0x22) == 0)) {
            return;
        }
        plVar6 = (iVar4 + 0x22);
    } else {
        uVar2 = (iVar4 + 0x1a);
        plVar6 = (uVar2 + 0x28);
    }
    pass1_1020_ba94(plVar6);
    return;
}



pub unsafe fn fn_ptr_1030_7296(param_1: *mut astruct_292) {
    let mut uVar1: u16;
    let mut uVar2: u16;
    let mut iVar3: *mut astruct_292;
    let mut uVar3: *mut astruct_292;
    let mut pcStack6: *mut c_char;

    uVar3 = (param_1 >> 0x10);
    iVar3 = param_1;
    uVar1 = iVar3.field34_0x22;
    uVar2 = iVar3.field35_0x24;
    pcStack6 = CONCAT22(uVar2, uVar1);
    if ((uVar2 | uVar1) != 0) {
        fn_ptr_1020_ba7e(CONCAT22(uVar2, uVar1));
        fn_ptr_1000_17ce(pcStack6);
    }
    iVar3.field34_0x22 = 0;
    return;
}
pub unsafe fn pass1_1030_72d0(param_1: *mut astruct_292) {
    let mut uVar1: u16;
    let mut uVar2: u16;
    let mut iVar3: *mut astruct_292;
    let mut uVar3: u16;
    let mut pcStack6: *mut c_char;

    uVar3 = (param_1 >> 0x10);
    iVar3 = param_1;
    uVar1 = (iVar3 + 1);
    uVar2 = &iVar3[0x1].field_0x2;
    pcStack6 = CONCAT22(uVar2, uVar1);
    if ((uVar2 | uVar1) != 0) {
        fn_ptr_1020_ba7e(CONCAT22(uVar2, uVar1));
        fn_ptr_1000_17ce(pcStack6);
    }
    (iVar3 + 1) = 0;
    return;
}


pub unsafe fn pass1_1030_730a(mut param_1: u16, param_2: *mut astruct_290) {
    let mut puVar1: *mut u32;
    let mut uVar2: u16;
    let mut ppcVar3: *mut *mut code;
    let mut uVar4: u32;
    let mut extraout_DX: u16;
    let mut extraout_DX_00: u16;
    let mut pstruct295_1: *mut astruct_290;
    let mut pstruct295_2: *mut astruct_290;
    let mut unaff_CS: u16;
    let mut puVar5: *mut u32;
    let mut uStack10: u32;
    let mut uStack6: u32;

    pstruct295_2 = (param_2 >> 0x10);
    pstruct295_1 = param_2;
    if (pstruct295_1.field30_0x1e.is_null() == false) {
        puVar5 = pstruct295_1.field30_0x1e;
        ppcVar3 = (*pstruct295_1.field30_0x1e + 0x10);
        (**ppcVar3)();
        uStack6 = CONCAT22(extraout_DX, param_1);
        for uStack10 in 0..uStack6 {
            ppcVar3 = (*pstruct295_1.field30_0x1e + 0x4);
            uVar4 = uStack6;
            (**ppcVar3)(unaff_CS);
            if ((extraout_DX_00 | uVar4) != 0) {
                unaff_CS = 0x1028;
                pass1_1028_e332(_PTR_LOOP_1050_65e2, uVar4, extraout_DX_00);
            }
        }
        // WARNING: Load size is inaccurate
        puVar1 = pstruct295_1.field30_0x1e;
        uVar2 = (&pstruct295_1.field30_0x1e + 2);
        if ((uVar2 | puVar1) != 0) {
            ppcVar3 = *puVar1;
            (**ppcVar3)(unaff_CS, puVar1, uVar2, 0x1, puVar5);
        }
        pstruct295_1.field30_0x1e = null_mut();
    }
    return;
}



pub unsafe fn struct_op_1030_73a8(
    param_1: *mut astruct_419,
    mut param_2: u16,
    mut param_3: u16,
) -> u32 {
    let mut iVar1: i16;
    let mut uVar2: u16;

    uVar2 = (param_1 >> 0x10);
    iVar1 = param_1;
    if ((iVar1 + 0x16) == 0) {
        return 0x0;
    }
    if ((iVar1 + 0x1a) == 0) {
        pass1_1028_e1ec(_PTR_LOOP_1050_65e2, (iVar1 + 0x16));
        (iVar1 + 0x1a) = param_2;
        (iVar1 + 0x1c) = param_3;
    }
    return CONCAT22((iVar1 + 0x1c), (iVar1 + 0x1a));
}


pub unsafe fn pass1_1030_73ee(mut param_1: u16, param_2: *mut astruct_294, mut param_3: u32) {
    let mut iVar1: *mut astruct_294;
    let mut uVar1: u16;

    uVar1 = (param_2 >> 0x10);
    iVar1 = param_2;
    iVar1.field42_0x2a = param_3;
    pass1_1028_e1ec(_PTR_LOOP_1050_65e2, param_3);
    iVar1.field43_0x2e = param_3;
    iVar1.field44_0x30 = param_1;
    return;
}




pub unsafe fn pass1_1030_7bee(mut param_1: u32) -> u16 {
    let mut ppcVar1: *mut *mut code;
    let mut in_AX: u16;
    let mut uVar2: u16;
    let mut in_DX: u16;
    let mut iVar3: i16;
    let mut uVar4: u16;

    uVar4 = (param_1 >> 0x10);
    iVar3 = param_1;
    if ((iVar3 + 0x16) == 0) {
        return 0x0;
    }
    if ((iVar3 + 0x1a) == 0) {
        struct_op_1030_73a8((param_1 & 0xffff | uVar4 << 0x10), in_AX, in_DX);
    }
    ppcVar1 = ((iVar3 + 0x1a) + 0x44);
    uVar2 = (**ppcVar1)();
    return uVar2;
}

pub unsafe fn pass1_1030_7c28(
    mut param_1: u16,
    mut param_2: u16,
    mut param_3: u32,
    mut param_4: u16,
) -> u32 {
    let mut uVar1: u16;
    let mut uVar2: u32;

    uVar1 = (param_3 >> 0x10);
    if ((param_3 + 0x22) == 0) {
        return 0x0;
    }
    uVar2 = (param_3 + 0x22);
    uVar2 = pass1_1020_bae6(param_1, param_2, uVar2, CONCAT22(param_4, (uVar2 >> 0x10)));
    return uVar2;
}


pub unsafe fn pass1_1030_7c50(
    mut param_1: u16,
    param_2: *mut Struct57,
    param_3: *mut astruct_305,
    param_4: i32,
    mut param_5: i16,
) {
    let mut piVar1: *mut i16;
    let mut ppcVar2: *mut *mut code;
    let mut uVar4: *mut Struct57;
    let mut uVar3: u16;
    let mut iVar8: *mut astruct_305;
    let mut uVar5: u16;
    let mut uVar6: u32;
    let mut puVar7: *mut u32;
    let mut uVar8: u32;
    let mut puStack18: *mut u32;

    uVar5 = (param_3 >> 0x10);
    iVar8 = param_3;
    if (iVar8.field30_0x1e.is_null()) {
        mem_op_1000_179c(0x18, param_2);
        uVar4 = param_2;
        param_2 = (param_2 & 0xffff0000 | (uVar4 | param_1));
        if ((uVar4 | param_1) == 0) {
            iVar8.field30_0x1e = null_mut();
        } else {
            struct_op_1030_1cd8(CONCAT22(uVar4, param_1), 0x5, 0x5);
            iVar8.field30_0x1e = param_1;
            (&iVar8.field30_0x1e + 0x2) = param_2;
        }
    }
    if (param_5 == 0x4) {
        piVar1 = &iVar8.field49_0x34;
        *piVar1 = *piVar1 + param_4;
    }
    while (param_4 != 0) {
        uVar6 = pass1_1028_e2e0(param_2, _PTR_LOOP_1050_65e2, 0x6);
        param_2 = (param_2 & 0xffff0000 | uVar6 >> 0x10);
        uVar3 = uVar6;
        puVar7 = iVar8.field30_0x1e;
        ppcVar2 = (*iVar8.field30_0x1e + 0xc);
        uVar8 = uVar6;
        (**ppcVar2)();
        pass1_1028_e1ec(_PTR_LOOP_1050_65e2, uVar6);
        puStack18 = CONCAT22(param_2, uVar3);
        ppcVar2 = (*puStack18 + 0x14);
        (**ppcVar2)(0x1028, uVar3, param_2, param_3, puVar7, uVar8);
        param_4 = param_4 -0x1;
    }
    return;
}
pub unsafe fn pass1_1030_7d1c(
    mut param_1: u16,
    mut param_2: u16,
    param_3: *mut astruct_397,
    mut param_4: u16,
    mut param_5: u32,
) {
    let mut uVar1: u16;
    let mut in_register_0000000a: u16;
    let mut paVar2: *mut Struct57;
    let mut iVar2: *mut astruct_397;
    let mut uVar3: *mut astruct_397;

    paVar2 = CONCAT22(in_register_0000000a, param_2);
    uVar3 = (param_3 >> 0x10);
    iVar2 = param_3;
    if (iVar2.field34_0x22.is_null()) {
        mem_op_1000_179c(0xa, paVar2);
        uVar1 = paVar2 | param_1;
        if (uVar1 == 0) {
            iVar2.field34_0x22 = null_mut();
        } else {
            pass1_1020_ba3e(CONCAT22(paVar2, param_1), 0xa, 0x2);
            iVar2.field34_0x22 = param_1;
            (&iVar2.field34_0x22 + 0x2) = uVar1;
        }
    }
    pass1_1020_bb8a(iVar2.field34_0x22, param_4, param_5);
    return;
}
pub unsafe fn pass1_1030_7d7c(
    mut param_1: u16,
    param_2: *mut u8,
    param_3: *mut astruct_398,
    mut param_4: u16,
    mut param_5: u32,
) {
    let mut uVar1: u16;
    let mut in_register_0000000a: u16;
    let mut paVar2: *mut Struct57;
    let mut iVar2: *mut astruct_398;
    let mut uVar3: *mut astruct_398;

    paVar2 = CONCAT22(in_register_0000000a, param_2);
    uVar3 = (param_3 >> 0x10);
    iVar2 = param_3;
    if (iVar2.field38_0x26.is_null()) {
        mem_op_1000_179c(0xa, paVar2);
        uVar1 = paVar2 | param_1;
        if (uVar1 == 0) {
            iVar2.field38_0x26 = null_mut();
        } else {
            pass1_1020_ba3e(CONCAT22(paVar2, param_1), 0xa, 0x2);
            iVar2.field38_0x26 = param_1;
            (&iVar2.field38_0x26 + 0x2) = uVar1;
        }
    }
    pass1_1020_bb8a(iVar2.field38_0x26, param_4, param_5);
    return;
}
pub unsafe fn pass1_1030_7ddc(
    mut param_1: u16,
    param_2: *mut Struct57,
    mut param_3: u32,
    param_4: i32,
    mut param_5: u16,
) {
    let mut uVar1: u32;
    let mut uVar2: *mut Struct57;
    let mut iVar2: i16;
    let mut uVar3: u16;
    let mut lVar4: i32;

    uVar3 = (param_3 >> 0x10);
    iVar2 = param_3;
    if ((iVar2 + 0x22) == 0) {
        mem_op_1000_179c(0xa, param_2);
        uVar2 = param_2;
        param_2 = (uVar2 | param_1);
        if ((uVar2 | param_1) == 0) {
            (iVar2 + 0x22) = 0;
        } else {
            pass1_1020_ba3e(CONCAT22(uVar2, param_1), 0xa, 0x2);
            (iVar2 + 0x22) = param_1;
            (iVar2 + 0x24) = param_2;
        }
    }
    uVar1 = (iVar2 + 0x22);
    lVar4 = pass1_1020_bae6(param_1, param_2, uVar1, CONCAT22(param_5, (uVar1 >> 0x10)));
    pass1_1020_bb8a(
        (iVar2 + 0x22),
        (lVar4 + param_4),
        CONCAT22(param_5, ((lVar4 + param_4) >> 0x10)),
    );
    return;
}
pub unsafe fn pass1_1030_7e5a(mut param_1: u16, param_2: *mut astruct_358, mut param_3: u32) {
    let mut pstruct_1: *mut astruct_358;
    let mut uVar1: u16;

    uVar1 = (param_2 >> 0x10);
    pstruct_1 = param_2;
    pstruct_1.field19_0x16 = param_3;
    pstruct_1.field20_0x1a = 0;
    pass1_1030_6fa0(param_2 & 0xffff | uVar1 << 0x10);
    if (pstruct_1.field37_0x2e != 0) {
        pass1_1038_4b20(
            param_1,
            pstruct_1.field37_0x2e,
            pstruct_1.field19_0x16,
            pstruct_1.field4_0x4,
        );
    }
    return;
}



pub unsafe fn pass1_1030_7eda(mut param_1: u32, mut param_2: u16) {
    let mut in_DX: u16;
    let mut uVar1: u16;
    let mut local_c: u16;
    let mut uStack10: u16;
    let mut uStack8: u16;
    let mut uStack6: u16;
    let mut uStack4: u16;

    local_c = 0;
    uStack10 = 0;
    uStack6 = 0;
    uStack4 = 0;
    uStack8 = param_2;
    uVar1 = (param_1 >> 0x10);
    if ((param_1 + 0x1a) == 0) {
        struct_op_1030_73a8((param_1 & 0xffff | uVar1 << 0x10), param_2, in_DX);
    }
    pass1_1028_bb96((param_1 + 0x1a), &local_c, 0x1050);
    return;
}
pub unsafe fn pass1_1030_7f1a(mut param_1: u32, mut param_2: u16) {
    let mut in_DX: u16;
    let mut uVar1: u16;
    let mut local_c: u16;
    let mut uStack10: u16;
    let mut uStack8: u16;
    let mut uStack6: u16;
    let mut uStack4: u16;

    local_c = 0;
    uStack8 = 0;
    uStack6 = 0;
    uStack4 = 0;
    uStack10 = param_2;
    uVar1 = (param_1 >> 0x10);
    if ((param_1 + 0x1a) == 0) {
        struct_op_1030_73a8((param_1 & 0xffff | uVar1 << 0x10), param_2, in_DX);
    }
    pass1_1028_bb96((param_1 + 0x1a), &local_c, 0x1050);
    return;
}

pub unsafe fn pass1_1030_7f5a(mut param_1: u32) -> u16 {
    let mut in_AX: u16;
    let mut in_DX: u16;
    let mut uVar1: u16;
    let mut uVar2: u32;

    uVar1 = (param_1 >> 0x10);
    if ((param_1 + 0x1a) == 0) {
        struct_op_1030_73a8((param_1 & 0xffff | uVar1 << 0x10), in_AX, in_DX);
    }
    uVar2 = pass1_1028_bb6a((param_1 + 0x1a));
    if (uVar2 != 0) {
        return (uVar2 + 0x4);
    }
    return 0x0;
}

pub unsafe fn pass1_1030_7f98(mut param_1: u32) -> u16 {
    let mut in_AX: u16;
    let mut in_DX: u16;
    let mut uVar1: u16;
    let mut uVar2: u32;

    uVar1 = (param_1 >> 0x10);
    if ((param_1 + 0x1a) == 0) {
        struct_op_1030_73a8((param_1 & 0xffff | uVar1 << 0x10), in_AX, in_DX);
    }
    uVar2 = pass1_1028_bb6a((param_1 + 0x1a));
    if (uVar2 != 0) {
        return (uVar2 + 2);
    }
    return 0x0;
}
