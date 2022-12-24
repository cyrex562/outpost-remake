use crate::block_1000::block_1000_1000::fn_ptr_1000_17ce;
use crate::block_1010::block_1010_0000::pass1_1010_038e;
use crate::block_1010::block_1010_2000::mixed_1010_20ba;
use crate::block_1010::block_1010_a000::pass1_1010_a5ca;
use crate::block_1018::block_1018_5000::pass1_1018_5742;
use crate::draw_ops::{draw_line_1040_c302, draw_op_1040_c38e, draw_text_1040_94fc, unk_draw_op_1040_b0f8};
use crate::block_1040::block_1040_b000::{pass1_1040_b316, pass1_1040_b54a, pass1_1040_bfde, struct_1040_bf3e};
use crate::block_1040::block_1040_c000::{pass1_1040_c5ac, pass1_1040_ca74};
use crate::block_1040::block_1040_d000::pass1_1040_d1bc;
use crate::structs::struct_27::Struct27;
use crate::structs::struct_37::Struct37;
use crate::structs::struct_903::Struct903;
use crate::structs::struct_d::StructD;
use crate::utils::CONCAT22;
use crate::win_ui;
use crate::win_ui::destroy_window_1040_b726;
use crate::windef16::{HDC16, LRESULT};

pub unsafe fn pass1_1040_b864(param_1: u32, param_2: *mut i16, mut param_3: u16, mut param_4: u16, mut param_5: i16) -> u16

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
        (**ppcVar1)();
    }
    return 0x1;
}


pub unsafe fn pass1_1040_b8be(param_1: u32)

{
    let mut ppcVar1: *mut *mut code;

    ppcVar1 = (*param_1 + 0x80);
    (**ppcVar1)();
    return;
}


pub unsafe fn win_ui_1040_b8d2(mut param_1: u16, param_2: *mut StructB)

{
    let mut uVar1: u32;
    let mut uVar2: u16;
    let mut uVar3: u16;
    let mut paVar4: *mut Struct57;
    let mut uVar5: u16;
    let mut uVar6: u16;
    let mut iVar7: i16;
    let mut uVar8: u16;
    let mut uVar9: u16;
    let mut in_register_0000000a: u16;
    let mut paVar10: *mut Struct57;
    let mut paVar12: *mut Struct57;
    let mut struct_b_10: *mut StructB;
    let mut struct_b_10_hi: *mut StructB;
    let mut uVar13: u16;
    let mut puVar14: *mut u32;
    let mut puVar15: *mut u16;
    let mut in_stack_0000fe3a: u16;
    let mut in_stack_0000fe3e: u16;
    let mut in_stack_0000fe8c: u16;
    let mut in_stack_0000ff64: u16;
    let mut in_stack_0000ff68: u16;
    let mut in_stack_0000ff6c: u16;
    let mut in_stack_0000ffac: u16;
    let mut in_stack_0000ffb0: u16;
    let mut in_stack_0000ffb6: u16;
    let mut in_stack_0000ffba: u16;
    let mut in_stack_0000ffe4: u16;
    let mut paVar11: *mut Struct57;

    paVar10 = CONCAT22(in_register_0000000a, param_1);
    dialog_ui_fn_1040_78e2(param_2);
    puVar14 = mixed_1010_20ba(paVar10, _u16_1050_0ed0, CONCAT22(in_stack_0000ffe4, 0x31), in_stack_0000fe8c,
                              in_stack_0000ffb0, in_stack_0000ffb6, in_stack_0000ffba);
    paVar10 = (paVar10 & 0xffff0000 | puVar14 >> 0x10);
    paVar4 = puVar14;
    struct_b_10_hi = (param_2 >> 0x10);
    struct_b_10 = param_2;
    struct_b_10[0x7].field6_0xc = paVar4;
    struct_b_10[0x7].field7_0xe = (puVar14 >> 0x10);
    mem_op_1000_179c(0xa, paVar10);
    uVar8 = paVar10 | paVar4;
    paVar11 = (paVar10 & 0xffff0000 | uVar8);
    if (uVar8 == 0) {
        struct_b_10[0x7].max_count_field_0x10 = 0;
    } else {
        puVar15 = struct_1040_bf3e(CONCAT22(paVar10, paVar4), struct_b_10.lpvoid_field_0x8);
        paVar11 = (paVar11 & 0xffff0000 | puVar15 >> 0x10);
        paVar4 = puVar15;
        struct_b_10[0x7].max_count_field_0x10 = paVar4;
        struct_b_10[0x7].field5_0xa = (puVar15 >> 0x10);
    }
    pass1_1040_bfde(&struct_b_10[0x7].max_count_field_0x10, &struct_b_10[0x7].field6_0xc);
    mem_op_1000_179c(0x42, paVar11);
    uVar8 = paVar11 | paVar4;
    paVar10 = (paVar11 & 0xffff0000 | uVar8);
    if (uVar8 != 0) {
        pass1_1008_3bd6(paVar10, paVar4, paVar11, 0x1, 0xa000a, 0x0, 0x800081,
                        CONCAT22(struct_b_10.lpvoid_field_0x8, 0x10a), in_stack_0000ffac, in_stack_0000fe3a, in_stack_0000fe3e,
                        in_stack_0000ff64, in_stack_0000ff68, in_stack_0000ff6c);
    }
    mem_op_1000_179c(0x42, paVar10);
    uVar8 = paVar10 | paVar4;
    paVar11 = (paVar10 & 0xffff0000 | uVar8);
    if (uVar8 != 0) {
        pass1_1008_3bd6(paVar11, paVar4, paVar10, 0x1, 0xa0028, 0x0, 0x840085,
                        CONCAT22(struct_b_10.lpvoid_field_0x8, 0x10b), in_stack_0000ffac, in_stack_0000fe3a, in_stack_0000fe3e,
                        in_stack_0000ff64, in_stack_0000ff68, in_stack_0000ff6c);
    }
    mem_op_1000_179c(0x42, paVar11);
    uVar8 = paVar11 | paVar4;
    paVar10 = (paVar11 & 0xffff0000 | uVar8);
    if (uVar8 != 0) {
        pass1_1008_3bd6(paVar10, paVar4, paVar11, 0x1, 0xa0046, 0x0, 0x860087,
                        CONCAT22(struct_b_10.lpvoid_field_0x8, 0x10d), in_stack_0000ffac, in_stack_0000fe3a, in_stack_0000fe3e,
                        in_stack_0000ff64, in_stack_0000ff68, in_stack_0000ff6c);
    }
    mem_op_1000_179c(0x42, paVar10);
    uVar8 = paVar10 | paVar4;
    paVar11 = (paVar10 & 0xffff0000 | uVar8);
    if (uVar8 != 0) {
        pass1_1008_3bd6(paVar11, paVar4, paVar10, 0x1, 0xa0064, 0x0, 0x880089,
                        CONCAT22(struct_b_10.lpvoid_field_0x8, 0x10e), in_stack_0000ffac, in_stack_0000fe3a, in_stack_0000fe3e,
                        in_stack_0000ff64, in_stack_0000ff68, in_stack_0000ff6c);
    }
    mem_op_1000_179c(0x42, paVar11);
    uVar8 = paVar11 | paVar4;
    paVar10 = (paVar11 & 0xffff0000 | uVar8);
    if (uVar8 != 0) {
        pass1_1008_3bd6(paVar10, paVar4, paVar11, 0x1, 0xa0082, 0x0, 0x820083,
                        CONCAT22(struct_b_10.lpvoid_field_0x8, 0x10c), in_stack_0000ffac, in_stack_0000fe3a, in_stack_0000fe3e,
                        in_stack_0000ff64, in_stack_0000ff68, in_stack_0000ff6c);
    }
    mem_op_1000_179c(0x42, paVar10);
    uVar8 = paVar10 | paVar4;
    paVar11 = (paVar10 & 0xffff0000 | uVar8);
    if (uVar8 != 0) {
        pass1_1008_3bd6(paVar11, paVar4, paVar10, 0x1, 0xa00d2, 0x0, 0x8a008b,
                        CONCAT22(struct_b_10.lpvoid_field_0x8, 0xbbb), in_stack_0000ffac, in_stack_0000fe3a, in_stack_0000fe3e,
                        in_stack_0000ff64, in_stack_0000ff68, in_stack_0000ff6c);
    }
    mem_op_1000_179c(0x42, paVar11);
    uVar8 = paVar11 | paVar4;
    paVar10 = (paVar11 & 0xffff0000);
    paVar12 = (paVar10 | uVar8);
    if (uVar8 == 0) {
        paVar4 = null_mut();
    } else {
        pass1_1008_3bd6(paVar12, paVar4, paVar11, 0x1, 0xa00a0, 0x8e, 0x8c008d,
                        CONCAT22(struct_b_10.lpvoid_field_0x8, 0xbbc), in_stack_0000ffac, in_stack_0000fe3a, in_stack_0000fe3e,
                        in_stack_0000ff64, in_stack_0000ff68, in_stack_0000ff6c);
        paVar10 = paVar12;
    }
    puVar14 = mixed_1010_20ba(paVar10, _u16_1050_0ed0, CONCAT22(in_stack_0000ffe4, 0x3), in_stack_0000fe8c,
                              in_stack_0000ffb0, in_stack_0000ffb6, in_stack_0000ffba);
    uVar3 = (puVar14 >> 0x10);
    uVar2 = puVar14;
    uVar9 = uVar3;
    uVar5 = pass1_1010_a5ac(uVar2, uVar3, struct_b_10[0x8].field8_0x10);
    uVar6 = pass1_1010_ac62(uVar5, uVar9, uVar2, uVar3, 0x1e);
    if (uVar6 != 0) {
        pass1_1010_a5ca(uVar6, uVar9, uVar2, uVar3, uVar5);
        if (0x0 < uVar6) {
            pass1_1010_a58a(uVar6, uVar9, uVar2, uVar3, uVar5);
//      if (uVar6 == 0) goto LAB_1040_bb26;
        }
    }
    enable_win_1040_9234(CONCAT22(paVar10, paVar4), 0x0);//
// LAB_1040_bb26:
    uVar1 = &struct_b_10[0x7].field6_0xc;
    iVar7 = uVar1;
    uVar1 &= 0xffff0000;
    uVar13 = (uVar1 >> 0x10);
    SetWindowPos16(0x40, (iVar7 + 0x10), (iVar7 + 0xe), (iVar7 + 0xc),
                   (uVar1 | iVar7 + 0xa), 0x0, struct_b_10.lpvoid_field_0x8);
    return;
}


pub unsafe fn pass1_1040_bb5a(mut param_1: u32) -> u16

{
    let mut ppcVar1: *mut *mut code;

    ppcVar1 = ((param_1 + 0x94) + 0x8);
    (**ppcVar1)();
    return 0x0;
}


pub unsafe fn pass1_1040_be94(param_1: *mut StructD, param_2: u8) -> *mut StructD

{
    let mut in_stack_0000ffda: u16;

    unk_draw_op_1040_b0f8(in_stack_0000ffda, param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1040_c518(mut param_1: u32, param_2: u8) -> u32

{
    pass1_1040_bf92(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}


pub unsafe fn pass1_1040_c71e(param_1: *mut astruct_65)

{
    let mut iVar1: *mut astruct_65;
    let mut uVar1: *mut astruct_65;

    pass1_1040_9252(param_1);
    uVar1 = (param_1 >> 0x10);
    iVar1 = param_1;
    iVar1[0x1].field1_0x2 = iVar1.field17_0x24 / 0x2 - iVar1[0x1].field3_0x6 / 0x2;
    return;
}


pub unsafe fn pass1_1040_c94a(param_1: *mut u8, pstruct37_param_2: *mut Struct37, hdc16_param_3: HDC16, mut param_4: u16)

{
    let mut var7: u16;
    let mut var8: u16;
    let mut var9: u16;
    let mut var10: u16;

    if (pstruct37_param_2.field_0x48) != 0 {
        let var6 = mixed_1010_20ba(param_1, 0xed0,
                                   CONCAT22(param_4, 0x3), var7, var8, var9,
                                   var10);
        let var3 = (var6 >> 0x10);
        let var2 = (pstruct37_param_2.field_0x42);
        let var1 = (var2 + 0x12);
        let var4 = var1;
        pass1_1010_a5ca(var1, var3, var6, var3, var1);
        if var4 == 0xffff {
            (pstruct37_param_2.field_0x3c) = 0xf9;
        } else if var4 == 0 {
            (pstruct37_param_2.field_0x3c) = 0x25;
            if (var1 == 0x5b) || (var1 == 0x9) {
                (pstruct37_param_2.field_0x3c) = 0xfe;
            }
        } else {
            (pstruct37_param_2.field_0x3c) = 0xfb;
        }
    }
    draw_text_1040_94fc(pstruct37_param_2, hdc16_param_3);
    return;
}


pub unsafe fn pass1_1040_c9cc(param_1: *mut StructD, param_2: u8) -> *mut StructD

{
    pass1_1040_c5ac(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}


pub unsafe fn pass1_1040_caa6(param_1: *mut u8, mut param_2: u16, mut param_3: u32)

{
    let mut in_register_0000000a: u16;
    let mut paVar1: *mut Struct27;
    let mut in_stack_0000fea0: u16;
    let mut in_stack_0000ffc4: u16;
    let mut in_stack_0000ffca: u16;
    let mut in_stack_0000ffce: u16;
    let mut iVar2: i16;

    iVar2 = 0;
    paVar1 = mixed_1010_20ba(CONCAT22(in_register_0000000a, param_1), _u16_1050_0ed0, 0x2b,
                             in_stack_0000fea0, in_stack_0000ffc4, in_stack_0000ffca, in_stack_0000ffce);
    pass1_1010_038e(paVar1, iVar2);
    destroy_window_1040_b726(CONCAT22(param_3, param_2), (param_3 >> 0x10));
    return;
}


pub unsafe fn pass1_1040_cc66(mut param_1: u32) -> LRESULT

{
    let mut ppcVar1: *mut *mut code;
    let mut extraout_DX: u16;
    let mut LVar2: LRESULT;

    ppcVar1 = ((param_1 + 0x98) + 0x10);
    (**ppcVar1)();
    LVar2 = win_ui::send_dlg_msg_1040_cf1c(extraout_DX, param_1);
    return LVar2;
}


pub unsafe fn pass1_1040_cc8c(param_1: *mut u8, param_2: *mut Struct903, mut param_3: u16, mut param_4: u16, mut param_5: u16)

{
    if (param_5 == 0xeb) {
        win_ui::send_dlg_msg_1040_cf1c(param_1, param_2);
    } else {
        // just 0x1756
        if (param_5 == s_vrpal_bmp_1050_183a + 0x7) {
            win_ui::msg_box_op_1040_cce4(0x0, param_1, param_2);
        } else {
            if (param_5 != s_vrpal_bmp_1050_183a + 0x8) {
                pass1_1040_b54a(param_1, param_2, param_3, _param_4);
                return;
            }
            if (param_4 == 1) {
                win_ui::send_dlg_item_1040_ce76(param_2);
            }
        }
    }
    return;
}


pub unsafe fn pass1_1040_d056(param_1: *mut StructD, param_2: u8) -> *mut StructD

{
    pass1_1040_ca74(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}


pub unsafe fn pass1_1040_d29c(mut param_1: u32)

{
    let mut in_DX: u16;

    win_ui::send_dlg_item_msg_1040_d79c(in_DX, param_1);
    return;
}


pub unsafe fn pass1_1040_d76e(mut param_1: u32)

{
    let mut uVar1: u32;
    let mut iVar2: i16;
    let mut uVar3: u16;

    uVar3 = (param_1 >> 0x10);
    iVar2 = param_1;
    uVar1 = (iVar2 + 0x94);
    pass1_1018_5742(uVar1, (uVar1 >> 0x10), (iVar2 + 0x9c), (iVar2 + 0x98),
    );
    (iVar2 + 0x9c) = 0;
    return;
}


pub unsafe fn pass1_1040_d89e(param_1: *mut StructD, param_2: u8) -> *mut StructD

{
    pass1_1040_d1bc(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}
