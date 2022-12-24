use crate::unk::block_1040_3000::pass1_1040_39e2;
use crate::unk::block_1040_4000::{pass1_1040_40e2, pass1_1040_4d7e, pass1_1040_4dcc};
use crate::gui::dialog::dlg_a::send_dlg_item_msg_1040_3f12;
use crate::gui::dialog::dlg_a::dialog_item_ui_op_1040_3e08;


pub unsafe fn pass1_1040_3fd6(param_1: *mut StructD, param_2: u8) -> *mut StructD

{
    pass1_1040_39e2(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}


pub unsafe fn pass1_1040_4440(param_1: *mut StructD, param_2: u8) -> *mut StructD

{
    pass1_1040_40e2(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}


pub unsafe fn pass1_1040_45e8(param_1: *mut u8, mut param_2: i16, mut param_3: u16, mut param_4: u16, mut param_5: u32)

{
    let mut pSVar1: *mut StructD;
    let mut ppcVar2: *mut *mut code;
    let mut uVar3: u32;
    let mut uVar4: u16;
    let mut pSVar5: *mut StructD;
    let mut uVar6: u16;
    let mut in_register_0000000a: u16;
    let mut paVar7: *mut Struct57;
    let mut paVar9: *mut Struct57;
    let mut iVar10: i16;
    let mut unaff_SI: u16;
    let mut uVar11: u16;
    let mut paVar12: *mut astruct_20;
    let mut in_stack_0000fe88: u16;
    let mut in_stack_0000ffac: u16;
    let mut in_stack_0000ffb2: u16;
    let mut in_stack_0000ffb6: u16;
    let mut piStack16: *mut i16;
    let mut paVar8: *mut Struct57;

    paVar7 = CONCAT22(in_register_0000000a, param_1);
    if (param_5 != 0xeb) {
        pass1_1040_b54a(param_1, CONCAT13((param_3 >> 0x8), CONCAT12(param_3, param_2)), param_4,
                        param_5);
        return;
    }
    paVar12 = mixed_1010_20ba(paVar7, _u16_1050_0ed0, CONCAT22(unaff_SI, 0x3), in_stack_0000fe88,
                              in_stack_0000ffac, in_stack_0000ffb2, in_stack_0000ffb6);
    paVar7 = (paVar7 & 0xffff0000 | paVar12 >> 0x10);
    pSVar1 = (param_2 + 0x90);
    if (pSVar1.is_null() == false) {
        pSVar5 = pSVar1;
        mem_op_1000_179c(0x18, paVar7);
        uVar4 = pSVar5;
        uVar6 = paVar7 | uVar4;
        paVar9 = (paVar7 & 0xffff0000);
        paVar8 = (paVar9 | uVar6);
        if (uVar6 == 0) {
            uVar4 = 0;
        } else {
            struct_1040_a598((pSVar5 & 0xffff | paVar7 << 0x10));
            paVar9 = paVar8;
        }
        (param_2 + 0x90) = uVar4;
        (param_2 + 0x92) = paVar9;
        (param_2 + 0x90) = 0x14;
        iVar10 = *(param_2 + 0x90);
        uVar4 = iVar10 * 0xa + 2;
        mem_op_1000_179c(uVar4, paVar9);
        uVar6 = paVar9;
        piStack16 = CONCAT22(uVar6, uVar4);
        if ((uVar6 | uVar4) == 0) {
            uVar3 = (param_2 + 0x90);
            (uVar3 + 0x2) = 0;
        } else {
            *piStack16 = iVar10;
            pass1_1000_5586(0xa564, &PTR_LOOP_1050_1040, iVar10, 0xa, uVar4 + 0x2, uVar6);
            uVar3 = (param_2 + 0x90);
            uVar11 = (uVar3 >> 0x10);
            iVar10 = uVar3;
            (iVar10 + 0x2) = uVar4 + 2;
            (iVar10 + 0x4) = uVar6;
        }
        uVar3 = (param_2 + 0x90);
        (uVar3 + 0x6) = (pSVar1 + 0x6);
        uVar3 = (param_2 + 0x90);
        (uVar3 + 0xa) = 0x1;
        uVar3 = (param_2 + 0x90);
        (uVar3 + 0x12) = (param_2 + 0xa);
        uVar11 = 0x1010;
        pass1_1010_a50c(paVar12, 0x10505d40, (param_2 + 0x90));
        if (pSVar1.is_null() == false) {
            pass1_1040_a5d0(pSVar1);
            uVar11 = 0x1000;
            fn_ptr_1000_17ce(pSVar1);
        }
        ppcVar2 = (CONCAT22(param_3, param_2) + 0x70);
        (**ppcVar2)(uVar11, param_2, param_3);
    }
    return;
}


pub unsafe fn pass1_1040_477e(mut param_1: u16, param_2: *mut StructB)

{
    let mut puVar1: *mut u8;
    let mut pUVar2: *mut u16;
    let mut puVar3: *mut u8;
    let mut puVar4: *mut u8;
    let mut in_register_0000000a: u16;
    let mut paVar5: *mut Struct57;
    let mut puVar6: *mut u32;
    let mut in_stack_0000fe96: u16;
    let mut in_stack_0000ffba: u16;
    let mut in_stack_0000ffc0: u16;
    let mut in_stack_0000ffc4: u16;
    let mut uVar7: u16;
    let mut uVar8: u16;
    let mut in_stack_0000ffee: u16;

    paVar5 = CONCAT22(in_register_0000000a, param_1);
    unk_win_ui_op_1040_b230(param_1, param_2);
    puVar6 = mixed_1010_20ba(paVar5, _u16_1050_0ed0, CONCAT22(in_stack_0000ffee, 0x3), in_stack_0000fe96,
                             in_stack_0000ffba, in_stack_0000ffc0, in_stack_0000ffc4);
    puVar3 = (puVar6 >> 0x10);
    uVar8 = SUB42(0x1050, 0x0);
    uVar7 = 0x5d68;
    puVar1 = pass1_1008_5fd8(puVar3);
    puVar4 = puVar3;
    pUVar2 = pass1_1000_3cea(CONCAT22(puVar3, puVar1), CONCAT22(uVar8, uVar7));
    pass1_1010_e964(puVar4);
    pass1_1000_3cea(CONCAT22(puVar3, puVar1), CONCAT22(puVar4, pUVar2));
    unk_str_op_1000_3d3e((param_2 & 0xffff0000 | (param_2 + 0x10)), CONCAT22(puVar3, puVar1));
    fn_ptr_1000_17ce(CONCAT22(puVar3, puVar1));
    return;
}


pub unsafe fn pass1_1040_47fe(param_1: *mut StructD, param_2: u8) -> *mut StructD

{
    let mut in_stack_0000ffda: u16;

    unk_draw_op_1040_b0f8(in_stack_0000ffda, param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}


pub unsafe fn pass1_1040_4df2(param_1: *mut StructD, param_2: u8) -> *mut StructD

{
    let mut in_stack_0000ffda: u16;

    unk_draw_op_1040_b0f8(in_stack_0000ffda, param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}


pub unsafe fn pass1_1040_4f28(param_1: u32, param_2: *mut i16, mut param_3: u16, mut param_4: u16, mut param_5: i16, mut param_6: u16) -> u16

{
    let mut ppcVar1: *mut *mut code;
    let mut uVar2: u16;

    if (param_5 == 0x2b) {
        if (*param_2 == 0x4) {
            win_ui_get_prop_op_1040_9566(CONCAT22(param_3, param_2));
        }
    } else {
        if (param_5 != 0x111) {
            uVar2 = pass1_1040_b316(param_1, param_2, param_3, param_4, param_5);
            return uVar2;
        }
        ppcVar1 = (*param_1 + 0x7c);
        (**ppcVar1)(param_6, param_1, param_2, CONCAT22(param_4, param_3));
    }
    return 0x1;
}


pub unsafe fn pass1_1040_4f82(param_1: u32)

{
    let mut ppcVar1: *mut *mut code;

    ppcVar1 = (*param_1 + 0x80);
    (**ppcVar1)();
    return;
}


pub unsafe fn set_win_pos_1040_4f96(param_1: *mut StructD, struct_b_param_1: *mut StructB, mut param_3: u16, mut param_4: u16, mut param_5: u16)

{
    let mut pvVar1: LPVOID = null_mut();
    let mut uVar2: u32;
    let mut uVar3: u16;
    let mut uVar4: u16;
    let mut paVar5: *mut Struct57;
    let mut uVar6: u16;
    let mut uVar7: u16;
    let mut iVar8: i16;
    let mut uVar9: u16;
    let mut uVar10: u16;
    let mut uVar11: u16;
    let mut paVar12: *mut Struct57;
    let mut paVar14: *mut Struct57;
    let mut struct_b_11: *mut StructB;
    let mut uVar15: u16;
    let mut uVar16: u16;
    let mut puVar17: *mut u32;
    let mut puVar18: *mut u16;
    let mut in_stack_0000fe34: u16;
    let mut in_stack_0000fe36: u16;
    let mut in_stack_0000fe38: u16;
    let mut in_stack_0000fe3a: u16;
    let mut in_stack_0000fe88: u16;
    let mut in_stack_0000fe8c: u16;
    let mut in_stack_0000ff5e: u16;
    let mut in_stack_0000ff60: u16;
    let mut in_stack_0000ff62: u16;
    let mut in_stack_0000ff64: u16;
    let mut in_stack_0000ff66: u16;
    let mut in_stack_0000ff68: u16;
    let mut in_stack_0000ffac: u16;
    let mut in_stack_0000ffb0: u16;
    let mut in_stack_0000ffb2: u16;
    let mut in_stack_0000ffb6: u16;
    let mut in_stack_0000ffba: u16;
    let mut uVar19: u8;
    let mut uVar20: u8;
    let mut BVar21: bool;
    let mut puVar22: *mut u8;
    let mut paVar13: *mut Struct57;
    let mut fn_ptr_1: *mut *mut code;

    dialog_ui_fn_1040_78e2(struct_b_param_1);
    puVar17 = mixed_1010_20ba(param_1, _u16_1050_0ed0, CONCAT22(param_5, 0x41), in_stack_0000fe8c,
                              in_stack_0000ffb0, in_stack_0000ffb6, in_stack_0000ffba);
    paVar12 = (param_1 & 0xffff0000 | puVar17 >> 0x10);
    paVar5 = puVar17;
    uVar15 = (struct_b_param_1 >> 0x10);
    struct_b_11 = struct_b_param_1;
    struct_b_11[0x7].field6_0xc = paVar5;
    uVar9 = (puVar17 >> 0x10);
    struct_b_11[0x7].field7_0xe = uVar9;
    puVar22 = struct_b_11[0x7].field6_0xc;
    fn_ptr_1 = (*&struct_b_11[0x7].field6_0xc + 0x10);
    (**fn_ptr_1)(0x1010, puVar22, uVar9);
    mem_op_1000_179c(0xa, paVar12);
    uVar10 = paVar12 | paVar5;
    paVar13 = (paVar12 & 0xffff0000 | uVar10);
    if (uVar10 == 0) {
        struct_b_11[0x7].max_count_field_0x10 = 0;
    } else {
        puVar18 = struct_1040_bf3e(CONCAT13((paVar12 >> 0x8), CONCAT12(paVar12, paVar5)),
                                   struct_b_11.lpvoid_field_0x8);
        paVar13 = (paVar13 & 0xffff0000 | puVar18 >> 0x10);
        paVar5 = puVar18;
        struct_b_11[0x7].max_count_field_0x10 = paVar5;
        struct_b_11[0x7].field5_0xa = (puVar18 >> 0x10);
    }
    pass1_1040_bfde(&struct_b_11[0x7].max_count_field_0x10, &struct_b_11[0x7].field6_0xc);
    mem_op_1000_179c(0x42, paVar13);
    uVar10 = paVar13 | paVar5;
    paVar12 = (paVar13 & 0xffff0000 | uVar10);
    if (uVar10 != 0) {
        pass1_1008_3bd6(paVar12, paVar5, paVar13, 0x1, 0xa000a, 0x0, 0x800081,
                        CONCAT22(struct_b_11.lpvoid_field_0x8, 0x10a), param_4, in_stack_0000fe36, in_stack_0000fe3a,
                        in_stack_0000ff60, in_stack_0000ff64, in_stack_0000ff68);
    }
    mem_op_1000_179c(0x42, paVar12);
    uVar10 = paVar12 | paVar5;
    paVar13 = (paVar12 & 0xffff0000 | uVar10);
    if (uVar10 != 0) {
        pass1_1008_3bd6(paVar13, paVar5, paVar12, 0x1, 0xa0028, 0x0, 0x840085,
                        CONCAT22(struct_b_11.lpvoid_field_0x8, 0x10b), param_4, in_stack_0000fe36, in_stack_0000fe3a,
                        in_stack_0000ff60, in_stack_0000ff64, in_stack_0000ff68);
    }
    mem_op_1000_179c(0x42, paVar13);
    uVar10 = paVar13 | paVar5;
    paVar12 = (paVar13 & 0xffff0000 | uVar10);
    if (uVar10 != 0) {
        pass1_1008_3bd6(paVar12, paVar5, paVar13, 0x1, 0xa0046, 0x0, 0x860087,
                        CONCAT22(struct_b_11.lpvoid_field_0x8, 0x10d), param_4, in_stack_0000fe36, in_stack_0000fe3a,
                        in_stack_0000ff60, in_stack_0000ff64, in_stack_0000ff68);
    }
    mem_op_1000_179c(0x42, paVar12);
    uVar10 = paVar12 | paVar5;
    paVar13 = (paVar12 & 0xffff0000 | uVar10);
    if (uVar10 != 0) {
        pass1_1008_3bd6(paVar13, paVar5, paVar12, 0x1, 0xa0064, 0x0, 0x880089,
                        CONCAT22(struct_b_11.lpvoid_field_0x8, 0x10e), param_4, in_stack_0000fe36, in_stack_0000fe3a,
                        in_stack_0000ff60, in_stack_0000ff64, in_stack_0000ff68);
    }
    mem_op_1000_179c(0x42, paVar13);
    uVar10 = paVar13 | paVar5;
    paVar12 = (paVar13 & 0xffff0000 | uVar10);
    if (uVar10 != 0) {
        pass1_1008_3bd6(paVar12, paVar5, paVar13, 0x1, 0xa0082, 0x0, 0x820083,
                        CONCAT22(struct_b_11.lpvoid_field_0x8, 0x10c), param_4, in_stack_0000fe36, in_stack_0000fe3a,
                        in_stack_0000ff60, in_stack_0000ff64, in_stack_0000ff68);
    }
    mem_op_1000_179c(0x42, paVar12);
    uVar10 = paVar12 | paVar5;
    paVar13 = (paVar12 & 0xffff0000 | uVar10);
    if (uVar10 != 0) {
        pass1_1008_3bd6(paVar13, paVar5, paVar12, 0x1, 0xa00d2, 0x0, 0x8a008b,
                        CONCAT22(struct_b_11.lpvoid_field_0x8, 0xbbb), param_4, in_stack_0000fe36, in_stack_0000fe3a,
                        in_stack_0000ff60, in_stack_0000ff64, in_stack_0000ff68);
    }
    BVar21 = 0;
    mem_op_1000_179c(0x42, paVar13);
    uVar10 = paVar13 | paVar5;
    paVar12 = (paVar13 & 0xffff0000);
    paVar14 = (paVar12 | uVar10);
    if (uVar10 == 0) {
        paVar5 = null_mut();
    } else {
        pvVar1 = struct_b_11.lpvoid_field_0x8;
        pass1_1008_3bd6(paVar14, paVar5, paVar13, 0x1, 0xa00a0, 0x8e, 0x8c008d,
                        CONCAT13((pvVar1 >> 0x8), CONCAT12(pvVar1, 0xbbc)), param_3, in_stack_0000fe34,
                        in_stack_0000fe38, in_stack_0000ff5e, in_stack_0000ff62, in_stack_0000ff66);
        paVar12 = paVar14;
    }
    uVar19 = SUB41(paVar12, 0x0);
    uVar20 = (paVar12 >> 0x8);
    enable_win_1040_9234(CONCAT13(uVar20, CONCAT12(uVar19, paVar5)), BVar21);
    puVar17 = mixed_1010_20ba(paVar12, _u16_1050_0ed0, CONCAT22(puVar22, 0x3), in_stack_0000fe88,
                              in_stack_0000ffac, in_stack_0000ffb2, in_stack_0000ffb6);
    uVar4 = (puVar17 >> 0x10);
    uVar3 = puVar17;
    uVar11 = uVar4;
    uVar6 = pass1_1010_a5ac(uVar3, uVar4, struct_b_11[0x8].field8_0x10);
    uVar7 = pass1_1010_ac62(uVar6, uVar11, uVar3, uVar4, 0x1e);
    if (uVar7 != 0) {
        pass1_1010_a5ca(uVar7, uVar11, uVar3, uVar4, uVar6);
        if (0x0 < uVar7) {
            pass1_1010_a58a(uVar7, uVar11, uVar3, uVar4, uVar6);
            if (uVar7 == 0) {
                enable_win_1040_9234(CONCAT13(uVar20, CONCAT12(uVar19, paVar5)), 1);
            }
        }
    }
    uVar2 = &struct_b_11[0x7].field6_0xc;
    iVar8 = uVar2;
    uVar2 &= 0xffff0000;
    uVar16 = (uVar2 >> 0x10);
    SetWindowPos16(0x40, (iVar8 + 0x10), (iVar8 + 0xe), (iVar8 + 0xc),
                   (uVar2 | iVar8 + 0xa), 0x0, struct_b_11.lpvoid_field_0x8);
    return;
}

pub unsafe fn pass1_1040_5238(mut param_1: u32) -> u16

{
    let mut ppcVar1: *mut *mut code;

    ppcVar1 = ((param_1 + 0x94) + 0x8);
    (**ppcVar1)();
    return 0x0;
}
