use crate::block_1018::block_1018_e000::{pass1_1018_e2a0, pass1_1018_e4f2, pass1_1018_e57a, pass1_1018_e64c, pass1_1018_e834, pass1_1018_e8bc, pass1_1018_e9de, pass1_1018_ec74};
use crate::block_1020::block_1020_0000::{pass1_1020_01d8, pass1_1020_022c, pass1_1020_04f6, pass1_1020_05d6};

pub unsafe fn pass1_1018_d2a2(param_1: *mut astruct_29, param_2: u8) -> *mut astruct_29

{
    destroy_window_1018_c518(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1018_d2c8(param_1: *mut astruct_29, param_2: u8) -> *mut astruct_29

{
    destroy_window_1018_c518(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1018_d2ee(param_1: *mut astruct_29, param_2: u8) -> *mut astruct_29

{
    destroy_window_1018_c518(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1018_d314(param_1: *mut astruct_29, param_2: u8) -> *mut astruct_29

{
    destroy_window_1018_c518(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1018_d33a(param_1: *mut astruct_29, param_2: u8) -> *mut astruct_29

{
    destroy_window_1018_c518(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1018_d360(param_1: *mut astruct_29, param_2: u8) -> *mut astruct_29

{
    destroy_window_1018_c518(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1018_d386(param_1: *mut astruct_29, param_2: u8) -> *mut astruct_29

{
    destroy_window_1018_c518(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1018_d3ac(param_1: *mut astruct_29, param_2: u8) -> *mut astruct_29

{
    destroy_window_1018_c518(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}


pub unsafe fn pass1_1018_df10(param_1: *mut StructD, param_2: u8) -> *mut StructD

{
    param_1.address_offset_field_0x0 = 0x389a;
    (param_1 + 0x2) = 0x1008;
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn win_1018_df40(mut param_1: u16, param_2: *mut u8, param_3: *mut StructA)

{
    let mut puVar2: *mut u8;
    let mut in_register_0000000a: u16;
    let mut paVar3: *mut Struct57;
    let struct_1: *mut StructA;
    let mut struct_1_hi: u16;
    let mut puVar1: *mut u16;

    paVar3 = CONCAT22(in_register_0000000a, param_2);
    create_window_ex_1008_9760(param_3);
    mem_op_1000_179c(0xa, paVar3);
    puVar2 = (paVar3 | param_1);
    struct_1 = param_3;
    struct_1_hi = (param_3 >> 0x10);
    if (puVar2.is_null() == false) {
        puVar1 = struct_1018_e100(puVar2, CONCAT22(paVar3, param_1), struct_1.field4_0x8);
        struct_1[0x1].field11_0x16 = puVar1;
        struct_1[0x1].field12_0x18 = (puVar1 >> 0x10);
        return;
    }
    struct_1[0x1].field11_0x16 = 0;
    return;
}


pub unsafe fn pass1_1018_df92(mut param_1: u32)

{
    let mut puVar1: *mut u32;
    let mut uVar2: u16;
    let mut ppcVar3: *mut *mut code;
    let mut iVar4: i16;
    let mut uVar5: u16;

    destroy_win_1008_628e(param_1);
    uVar5 = (param_1 >> 0x10);
    iVar4 = param_1;
    puVar1 = (iVar4 + 0xe2);
    uVar2 = (iVar4 + 0xe4);
    if ((uVar2 | puVar1) != 0) {
        ppcVar3 = *puVar1;
        (**ppcVar3)(0x1008, puVar1, uVar2, 1);
    }
    (iVar4 + 0xe2) = 0;
    return;
}



pub unsafe fn pass1_1018_dfd4(param_1: *mut u8, mut param_2: u32)

{
    let mut in_register_0000000a: u16;
    let mut paVar1: *mut Struct57;
    let mut uVar2: u16;
    let mut uVar3: u16;
    let mut puVar4: *mut u32;
    let mut in_stack_0000fe9c: u16;
    let mut in_stack_0000ffc0: u16;
    let mut in_stack_0000ffc6: u16;
    let mut in_stack_0000ffca: u16;
    let mut in_stack_0000fff4: u16;

    paVar1 = CONCAT22(in_register_0000000a, param_1);
    uVar3 = (param_2 >> 0x10);
    uVar2 = param_2;
    delete_palette_1018_e16c((uVar2 + 0xe2));
    if ((uVar2 + 0xe6) == 0) {
        (uVar2 + 0xe6) = 0x1;
        puVar4 = mixed_1010_20ba(paVar1, _u16_1050_0ed0, CONCAT22(in_stack_0000fff4, 0x36), in_stack_0000fe9c,
                                 in_stack_0000ffc0, in_stack_0000ffc6, in_stack_0000ffca);
        pass1_1038_af40(uVar2, (puVar4 >> 0x10), _PTR_LOOP_1050_5b7c, (uVar2 + 0x8), 0x8);
    }
    return;
}

pub unsafe fn pass1_1018_e01c(param_1: *mut StructD, param_2: u8) {
    let mut iVar1: *mut StructD;
    let mut uVar1: u16;

    iVar1 = param_1;
    iVar1 = &iVar1.field192_0xd2;
    pass1_1008_57c4((param_1 & 0xffff0000 | ZEXT24(iVar1)));
    uVar1 = (param_1 >> 0x10);
    param_1.address_offset_field_0x0 = 0x380a;
    iVar1.address_offset_field_0x2 = 0x1008;
    param_1.address_offset_field_0x0 = 0x389a;
    iVar1.address_offset_field_0x2 = 0x1008;
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return;
}


pub unsafe fn pass1_1018_e1ee(param_1: *mut StructD, param_2: u8) -> *mut StructD {
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

pub unsafe fn pass1_1018_e2cc(mut param_1: u32) {
    let mut piVar1: *mut i16;
    let mut ppcVar2: *mut *mut code;
    let mut uVar3: u32;
    let mut uVar4: u16;
    let mut uVar5: u16;
    let mut in_EDX: u32;
    let mut uVar8: u16;
    let mut iVar7: *mut astruct_269;
    let mut uVar9: u16;
    let mut puVar10: *mut u16;
    let mut in_stack_0000ff4c: u16;
    let mut in_stack_0000ff62: u16;
    let mut uStack10: u32;
    let mut local_6: [u8; 0x4] = [0; 0x4];
    let mut paVar6: *mut Struct57;
    let mut uVar7: u32;

    uVar8 = (in_EDX >> 0x10);
    uVar9 = (param_1 >> 0x10);
    iVar7 = param_1;
    if (iVar7.field235_0xee.is_null() == false) {
        ppcVar2 = (*iVar7.field235_0xee + 0x8);
        (**ppcVar2)();
    }
    if (iVar7.field233_0xea == 0) {
        iVar7.field233_0xea = 0x1;
        puVar10 = pass1_1008_941a(CONCAT22(0x1050, local_6), 0x1, 0x7a);
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
            piVar1 = &iVar7.field204_0xcc;
            *piVar1 = *piVar1 + 1;
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


pub unsafe fn window_op_1018_e384(
    param_1: *mut astruct_659,
    param_2: *mut Struct57,
    param_3: *mut StructA,
) {
    let mut uVar1: u16;
    let mut struct_1: *mut Struct57;
    let mut uVar2: *mut Struct57;

    create_window_ex_1008_9760(param_3);
    uVar2 = (param_3 >> 0x10);
    struct_1 = param_3;
    get_dc_1018_4db0(&struct_1[0x1].field80_0x64, struct_1.field4_0x8);
    mem_op_1000_179c(0x18, param_2);
    uVar1 = param_2 | param_1;
    if (uVar1 != 0) {
        pass1_1018_e4f2(param_1, (param_2 & 0xffff | struct_1.field4_0x8 << 0x10));
        struct_1[0x1].field78_0x60 = param_1;
        struct_1[0x1].field79_0x62 = uVar1;
        return;
    }
    struct_1[0x1].field78_0x60 = 0;
    return;
}

pub unsafe fn pass1_1018_e3e8(mut param_1: u32) {
    let mut puVar1: *mut u32;
    let mut uVar2: u16;
    let mut ppcVar3: *mut *mut code;
    let mut uVar4: u16;

    uVar4 = (param_1 >> 0x10);
    puVar1 = (param_1 + 0xee);
    uVar2 = (param_1 + 0xf0);
    if ((uVar2 | puVar1) != 0) {
        ppcVar3 = *puVar1;
        (**ppcVar3)();
    }
    destroy_win_1008_628e(param_1);
    return;
}

pub unsafe fn pass1_1018_e41a(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    param_1 = (param_1 & 0xffff0000 | (param_1 - 0xe2));
    pass1_1018_e2a0(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1018_e5aa(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    pass1_1018_e57a(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}


pub unsafe fn pass1_1018_e678(mut param_1: u16, mut param_2: u16, mut param_3: u32) -> u16 {
    let mut ppcVar1: *mut *mut code;
    let mut uVar2: u16;
    let mut uVar3: u16;
    let mut uVar4: u16;
    let mut uVar5: u32;

    uVar4 = (param_3 >> 0x10);
    uVar3 = param_3;
    uVar2 = (uVar3 + 0xf0) | (uVar3 + 0xee);
    if (uVar2 != 0) {
        ppcVar1 = ((uVar3 + 0xee) + 0x8);
        uVar5 = (**ppcVar1)();
        param_1 = (uVar5 >> 0x10);
        uVar2 = uVar5;
    }
    if ((uVar3 + 0xea) == 0) {
        (uVar3 + 0xea) = 0x1;
        uVar5 = pass1_1038_af40(uVar3, param_1, _PTR_LOOP_1050_5b7c, (uVar3 + 0x8), 0x15);
        uVar2 = uVar5;
    }
    return uVar2;
}


pub unsafe fn window_op_1018_e6c6(
    param_1: *mut astruct_666,
    mut param_2: u16,
    param_3: *mut StructA,
) {
    let mut uVar1: u16;
    let mut in_register_0000000a: u16;
    let mut paVar2: *mut Struct57;
    let iVar2: *mut StructA;
    let mut uVar4: u16;
    let mut uVar3: u32;

    paVar2 = CONCAT22(in_register_0000000a, param_2);
    create_window_ex_1008_9760(param_3);
    uVar4 = (param_3 >> 0x10);
    iVar2 = param_3;
    get_dc_1018_4db0(&iVar2[0x1].field20_0x26, iVar2.field4_0x8);
    mem_op_1000_179c(0x18, paVar2);
    uVar1 = paVar2 | param_1;
    uVar3 = paVar2 & 0xffff0000 | uVar1;
    if (uVar1 != 0) {
        pass1_1018_e834(CONCAT22(paVar2, param_1), iVar2.field4_0x8, uVar3);
        iVar2[0x1].field18_0x22 = param_1;
        iVar2[0x1].field19_0x24 = uVar3;
        return;
    }
    iVar2[0x1].field18_0x22 = 0;
    return;
}


pub unsafe fn pass1_1018_e72a(mut param_1: u32) {
    let mut puVar1: *mut u32;
    let mut uVar2: u16;
    let mut ppcVar3: *mut *mut code;
    let mut uVar4: u16;

    uVar4 = (param_1 >> 0x10);
    puVar1 = (param_1 + 0xee);
    uVar2 = (param_1 + 0xf0);
    if ((uVar2 | puVar1) != 0) {
        ppcVar3 = *puVar1;
        (**ppcVar3)();
    }
    destroy_win_1008_628e(param_1);
    return;
}


pub unsafe fn pass1_1018_e75c(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    param_1 = (param_1 & 0xffff0000 | (param_1 - 0xe2));
    pass1_1018_e64c(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}


pub unsafe fn FUN_1018_e76a(mut param_1: u16, param_2: *mut StructD, param_3: u8) -> *mut StructD {
    pass1_1018_e64c(param_2);
    if ((param_3 & 1) != 0) {
        fn_ptr_1000_17ce(param_2);
    }
    return param_2;
}


pub unsafe fn pass1_1018_e8ec(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    pass1_1018_e8bc(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn post_win_msg_1018_ea0a(mut param_1: u16, mut param_2: u16, mut param_3: i16) {
    if (param_3 == 0xed) {
        PostMessage16(0x0, 0xc6, 0x111, HWND16_1050_0396);
    }
    return;
}


pub unsafe fn FUN_1018_ea2c(mut param_1: u16, mut param_2: u32, mut param_3: i16) {
    let mut uVar1: u32;
    let mut iVar2: i16;
    let mut uVar3: u16;

    iVar2 = param_2;
    uVar3 = (param_2 >> 0x10);
    if (param_3 == 1) {
        (iVar2 + 0x14) = 0;
        return;
    }
    if (param_3 != 0xb) {
        return;
    }
    uVar1 = (iVar2 + 0x14);
    (uVar1 + 0xe) = (iVar2 - 0xda);
    return;
}

pub unsafe fn pass1_1018_ea66(mut param_1: u32) {
    let mut ppcVar1: *mut *mut code;
    let mut puVar2: *mut u8;
    let mut uVar3: u16;
    let mut in_EDX: u32;
    let mut uVar5: u16;
    let mut iVar6: i16;
    let mut uVar7: u16;
    let mut puVar8: *mut u16;
    let mut local_6: [u8; 0x4] = [0; 0x4];
    let mut uVar4: u32;

    uVar5 = (in_EDX >> 0x10);
    uVar7 = (param_1 >> 0x10);
    iVar6 = param_1;
    if ((iVar6 + 0xee) != 0) {
        ppcVar1 = ((iVar6 + 0xee) + 0x8);
        (**ppcVar1)();
    }
    if ((iVar6 + 0xea) == 0) {
        (iVar6 + 0xea) = 0x1;
        puVar8 = pass1_1008_941a(CONCAT22(0x1050, local_6), 0x1, 0x95);
        uVar3 = (puVar8 >> 0x10);
        uVar4 = CONCAT22(uVar5, uVar3);
        puVar2 = local_6;
        win_1008_5c9e(puVar2, uVar3, _u16_1050_02a0, CONCAT22(0x1050, puVar2));
        (iVar6 + 0xec) = puVar2;
        unk_win_op_1010_7300(uVar4, (iVar6 + 0xf6), 0x0, 0x8, 0x0);
    }
    return;
}


pub unsafe fn window_op_1018_eada(
    param_1: *mut astruct_661,
    param_2: *mut Struct57,
    param_3: *mut StructA,
) {
    let mut uVar1: u16;
    let struct_1: *mut StructA;
    let mut uVar2: u16;

    create_window_ex_1008_9760(param_3);
    uVar2 = (param_3 >> 0x10);
    struct_1 = param_3;
    get_dc_1018_4db0(&struct_1[0x1].field20_0x26, struct_1.field4_0x8);
    mem_op_1000_179c(0x28, param_2);
    uVar1 = param_2 | param_1;
    if (uVar1 != 0) {
        pass1_1018_ec74(uVar1, param_1, param_2, struct_1.field4_0x8);
        struct_1[0x1].field18_0x22 = param_1;
        struct_1[0x1].field19_0x24 = uVar1;
        return;
    }
    struct_1[0x1].field18_0x22 = 0;
    return;
}


pub unsafe fn pass1_1018_eb3e(mut param_1: u32) {
    let mut puVar1: *mut u32;
    let mut uVar2: u16;
    let mut ppcVar3: *mut *mut code;
    let mut iVar4: i16;
    let mut uVar5: u16;
    let mut iVar6: i16;
    let mut uVar7: u16;

    uVar7 = (param_1 >> 0x10);
    iVar6 = param_1;
    puVar1 = (iVar6 + 0xee);
    uVar2 = (iVar6 + 0xf0);
    if ((uVar2 | puVar1) != 0) {
        ppcVar3 = *puVar1;
        (**ppcVar3)();
    }
    if ((iVar6 + 0xf6) != 0) {
        if (param_1 == 0) {
            iVar4 = 0;
            uVar5 = 0;
        } else {
            iVar4 = iVar6 + 0xe2;
            uVar5 = uVar7;
        }
        pass1_1010_1ea6((iVar6 + 0xf6), CONCAT22(uVar5, iVar4));
    }
    destroy_win_1008_628e(param_1);
    return;
}


pub unsafe fn pass1_1018_eb9c(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    param_1 = (param_1 & 0xffff0000 | (param_1 - 0xe2));
    pass1_1018_e9de(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}


pub unsafe fn FUN_1018_ebaa(mut param_1: u16, param_2: *mut StructD, param_3: u8) -> *mut StructD {
    pass1_1018_e9de(param_2);
    if ((param_3 & 1) != 0) {
        fn_ptr_1000_17ce(param_2);
    }
    return param_2;
}

pub unsafe fn invalidate_rect_1018_edd8(mut param_1: u32, mut param_2: i16) {
    let mut iVar1: i16;
    let mut uVar2: u16;
    let mut uVar3: u32;
    let mut local_16: RECT16;
    let mut iStack18: i16;
    let mut iStack16: i16;
    let mut uStack14: u32;
    let mut uStack10: u16;
    let mut uStack8: u16;
    let mut local_6: i16;
    let mut local_4: i16;

    iVar1 = param_1;
    uVar2 = (param_1 >> 0x10);
    if (param_2 == 1) {
        (iVar1 + 0x14) = 0;
        return;
    }
    if (param_2 != 0xc) {
        return;
    }
    pass1_1008_3e94(
        (param_1 & 0xffff0000 | (iVar1 + 0x18)),
        CONCAT22(0x1050, &local_6),
        CONCAT22(0x1050, &local_4),
    );
    uVar3 = pass1_1010_2b66((iVar1 + 0x14));
    uStack8 = (uVar3 >> 0x10);
    uStack10 = uVar3;
    uStack14 = pass1_1008_4772((uVar3 & 0xffff | uStack8 << 0x10));
    uVar2 = (uStack14 >> 0x10);
    local_16.x = local_4;
    local_16.y = local_6;
    iStack18 = local_4 + (uStack14 + 0x4);
    iStack16 = local_6 + (uStack14 + 0x8);
    InvalidateRect16(0x0, &local_16, &DAT_1050_1050);
    return;
}

pub unsafe fn unk_draw_op_1020_0000(param_1: *mut astruct_840) {
    let mut piVar1: *mut i16;
    let mut ppcVar2: *mut *mut code;
    let mut iVar4: *mut astruct_840;
    let mut uVar5: u16;
    let mut uVar4: u16;
    let mut local_c4: [u8; 0x6] = [0; 0x6];
    let mut local_be: *mut i16;
    let mut piStack184: *mut i16;
    let mut uStack182: u16;
    let mut local_b4: i16;
    let mut iStack178: i16;
    let mut aiStack176: [i16; 0x3c] = [0; 0x3c];
    let mut iStack56: *mut astruct_841;
    let mut iStack48: i16;
    let mut paStack46: *mut astruct_76;
    let mut local_2a: i16;
    let mut local_28: i16;
    let mut puStack38: *mut u32;
    let mut local_22: [u8; 0x20] = [0; 0x20];
    let mut uVar3: u32;
    let mut uVar2: u32;
    let mut iVar3: *mut astruct_841;

    // Segment:    5
    // Offset:     00033420
    // Length:     efba
    // Min Alloc:  efba
    // Flags:      0d50
    //     Code
    //     Moveable
    //     Preload
    //     Impure (Non-shareable)
    //
    uVar5 = (param_1 >> 0x10);
    iVar4 = param_1;
    BeginPaint16(CONCAT22(0x1050, local_22), iVar4.field4_0x4);
    uVar3 = iVar4.field19_0x14;
    puStack38 = (uVar3 + 0xa);
    pass1_1008_3e94(
        (param_1 & 0xffff0000 | ZEXT24(&iVar4.field_0x18)),
        CONCAT22(0x1050, &local_2a),
        CONCAT22(0x1050, &local_28),
    );
    uVar4 = 0x1008;
    pass1_1008_4480(
        puStack38,
        (param_1 & 0xffff0000 | ZEXT24(&iVar4.field_0x18)),
        iVar4.field32_0x24,
    );
    paStack46 = null_mut();
    for iStack48 in 0..0x6 {
        uVar2 = iVar4.field19_0x14;
        uVar4 = 0x1010;
        pass1_1010_2b78(
            uVar2,
            (uVar2 >> 0x10),
            iStack48,
            CONCAT22(0x1050, &local_b4),
        );
        if (local_b4 == 0) {
            //   for (iStack56 = null_mut(); iVar3 = iStack56, iStack56 <= iStack178; iStack56 = iStack56 + 1)
            iStack56 = null_mut();
            iVar3 = iStack56;
            while iStack56 <= iStack178 {
                piVar1 = aiStack176 + iStack56 * 0x3;
                uStack182 = &DAT_1050_1050;
                piStack184 = piVar1;
                if (aiStack176[iStack56 * 0x3 + 0x2] != 0) {
                    paStack46 = pass1_1010_2b98(iVar4.field19_0x14, aiStack176[iStack56 * 0x3 + 0x2]);
                    pass1_1008_3e54(
                        CONCAT22(0x1050, &local_be),
                        0x0,
                        aiStack176[iVar3 * 0x3 + 0x1] + local_2a,
                        *piVar1 + local_28,
                    );
                    uVar4 = 0x1008;
                    pass1_1008_4480(puStack38, CONCAT22(0x1050, &local_be), paStack46);
                }
                iStack56 += 1;
            }
        } else {
            local_be = CONCAT22(0x1050, aiStack176 + iStack178 * 0x3);
            if (aiStack176[iStack178 * 0x3 + 0x2] != 0) {
                paStack46 = pass1_1010_2b98(iVar4.field19_0x14, aiStack176[iStack178 * 0x3 + 0x2]);
                pass1_1008_3e54(
                    CONCAT22(0x1050, local_c4),
                    0x0,
                    (local_be + 0x2) + local_2a,
                    *local_be + local_28,
                );
                uVar4 = 0x1008;
                pass1_1008_4480(puStack38, CONCAT22(0x1050, local_c4), paStack46);
            }
        }
    }
    ppcVar2 = (*puStack38 + 0x4);
    (**ppcVar2)(
        uVar4,
        puStack38,
        (puStack38 >> 0x10),
        0x0,
        0x0,
        &iVar4.field_0xa,
        uVar5,
    );
    EndPaint16(CONCAT22(0x1050, local_22), iVar4.field4_0x4);
    return;
}

pub unsafe fn pass1_1020_01a6(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    pass1_1018_ed98(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1020_028c(mut param_1: u16, mut param_2: u16, mut param_3: i16) {
    let mut in_DX: u16;

    pass1_1010_3c9e((param_1 + 0xe2));
    pass1_1008_68c6(in_DX, param_1, param_2, param_3);
    return;
}


pub unsafe fn pass1_1020_02ae(mut param_1: u32) {
    let mut puVar1: *mut u32;
    let mut uVar2: u16;
    let mut ppcVar3: *mut *mut code;
    let mut iVar4: i16;
    let mut uVar5: u16;

    uVar5 = (param_1 >> 0x10);
    iVar4 = param_1;
    pass1_1010_3cd0((iVar4 + 0xe2));
    destroy_win_1008_628e(param_1);
    puVar1 = (iVar4 + 0xe6);
    uVar2 = (iVar4 + 0xe8);
    if ((uVar2 | puVar1) != 0) {
        ppcVar3 = *puVar1;
        (**ppcVar3)(0x1008, puVar1, uVar2, 1);
    }
    (iVar4 + 0xe6) = 0;
    pass1_1010_1dda((iVar4 + 0xe2));
    (iVar4 + 0xe2) = 0;
    return;
}

pub unsafe fn win_1020_0316(mut param_1: u16, param_2: *mut StructA) {
    let mut uVar1: u32;
    let mut paVar2: *mut astruct_666;
    let mut puVar3: *mut u8;
    let mut in_register_0000000a: u16;
    let mut paVar4: *mut Struct57;
    let iVar1: *mut StructA;
    let mut uVar5: u16;
    let mut puVar6: *mut u32;
    let mut in_stack_0000fe9a: u16;
    let mut in_stack_0000ffbe: u16;
    let mut in_stack_0000ffc4: u16;
    let mut in_stack_0000ffc8: u16;
    let mut in_stack_0000fff2: u16;

    paVar4 = CONCAT22(in_register_0000000a, param_1);
    create_window_ex_1008_9760(param_2);
    puVar6 = mixed_1010_20ba(
        paVar4,
        _u16_1050_0ed0,
        CONCAT22(in_stack_0000fff2, 1),
        in_stack_0000fe9a,
        in_stack_0000ffbe,
        in_stack_0000ffc4,
        in_stack_0000ffc8,
    );
    paVar4 = (paVar4 & 0xffff0000 | puVar6 >> 0x10);
    uVar5 = (param_2 >> 0x10);
    iVar1 = param_2;
    iVar1[0x1].field11_0x16 = puVar6;
    iVar1[0x1].field12_0x18 = (puVar6 >> 0x10);
    uVar1 = &iVar1[0x1].field11_0x16;
    (uVar1 + 0x16) = iVar1[0x1].field15_0x1e;
    paVar2 = iVar1[0x1].field18_0x22;
    uVar1 = &iVar1[0x1].field11_0x16;
    (uVar1 + 0x12) = paVar2;
    struct_1010_3c52(paVar4, &iVar1[0x1].field11_0x16, &iVar1[0x1].field_0x20);
    mem_op_1000_179c(0x12, paVar4);
    puVar3 = (paVar4 | paVar2);
    if (puVar3.is_null() == false) {
        pass1_1020_04f6(puVar3, CONCAT22(paVar4, paVar2), iVar1.field4_0x8);
        iVar1[0x1].field13_0x1a = paVar2;
        iVar1[0x1].field14_0x1c = puVar3;
        return;
    }
    iVar1[0x1].field13_0x1a = 0;
    return;
}

pub unsafe fn post_msg_1020_03b2(mut param_1: u32) {
    let mut uVar1: u32;

    uVar1 = (param_1 + 0xe2);
    PostMessage16(0x0, (uVar1 + 0x16), 0x111, HWND16_1050_0396);
    return;
}

pub unsafe fn post_msg_1020_03d6(mut param_1: u32) {
    let mut uVar1: u32;

    uVar1 = (param_1 + 0xe2);
    PostMessage16(0x0, (uVar1 + 0x16), 0x111, HWND16_1050_0396);
    return;
}

pub unsafe fn post_msg_1020_03fa(mut param_1: u32) {
    let mut uVar1: u32;

    uVar1 = (param_1 + 0xe2);
    PostMessage16(0x0, (uVar1 + 0x16), 0x111, HWND16_1050_0396);
    return;
}

pub unsafe fn pass1_1020_0434(param_1: *mut astruct_29, param_2: u8) -> *mut astruct_29 {
    pass1_1020_022c(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn post_win_msg_1020_061c(mut param_1: u32, mut param_2: i16) {
    let mut uVar1: u32;
    let mut uVar2: u16;

    uVar2 = (param_1 >> 0x10);
    if (param_2 == 1) {
        (param_1 + 0x6) = 0;
        return;
    }
    if (param_2 != 0x2) {
        return;
    }
    uVar1 = (param_1 + 0x6);
    PostMessage16(0x0, (uVar1 + 0x16), 0x111, HWND16_1050_0396);
    return;
}

pub unsafe fn pass1_1020_0734(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    pass1_1020_05d6(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn struct_1020_0762(
    param_1: *mut astruct_20,
    mut param_2: u32,
    param_3: *mut u32,
    mut param_4: u16,
    mut param_5: u32,
    mut param_6: u32,
) {
    let mut iVar1: *mut astruct_20;
    let mut uVar1: *mut astruct_20;

    pass1_1020_01d8(
        param_1,
        param_2,
        (param_2 >> 0x10),
        param_4,
        param_5,
        (param_5 >> 0x10),
        param_6,
    );
    uVar1 = (param_1 >> 0x10);
    iVar1 = param_1;
    iVar1[0x1].field6_0xe = 0;
    iVar1[0x1].field7_0x10 = *param_3;
    param_1.offset_0x0 = 0x81a;
    iVar1.base_0x2 = 0x1020;
    return;
}
