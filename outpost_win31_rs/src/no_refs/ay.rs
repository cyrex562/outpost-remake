use crate::block_1000::block_1000_1000::fn_ptr_1000_17ce;
use crate::block_1010::block_1010_0000::pass1_1010_038e;
use crate::block_1010::block_1010_2000::mixed_1010_20ba;
use crate::block_1010::block_1010_a000::pass1_1010_a5ca;
use crate::block_1018::block_1018_5000::pass1_1018_5742;
use crate::block_1040::block_1040_9000::draw_text_1040_94fc;
use crate::block_1040::block_1040_b000::{destroy_window_1040_b726, pass1_1040_b316, pass1_1040_b54a, pass1_1040_bfde, struct_1040_bf3e, unk_draw_op_1040_b0f8};
use crate::block_1040::block_1040_c000::{draw_line_1040_c302, draw_op_1040_c38e, pass1_1040_c5ac, pass1_1040_ca74};
use crate::block_1040::block_1040_d000::pass1_1040_d1bc;
use crate::structs::struct_27::Struct27;
use crate::structs::struct_37::Struct37;
use crate::structs::struct_903::Struct903;
use crate::structs::struct_d::StructD;
use crate::utils::CONCAT22;
use crate::win_ui;
use crate::windef::{HDC16, LRESULT};

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


pub unsafe fn destroy_win_1040_bb78(param_1: *mut astruct_35)

{
    let mut uVar1: u16;
    let mut is_window: bool;
    let mut pstruct35_5: *mut astruct_35;
    let mut pstruct35_hi: *mut astruct_35;
    let mut unaff_CS: u16;
    let mut puVar1: *mut u32;
    let mut fn_ptr_1: *mut *mut code;

    pstruct35_hi = (param_1 >> 0x10);
    pstruct35_5 = param_1;
    if (pstruct35_5.hwnd_0xb6 != 0) {
        // 0x1538
        unaff_CS = SUB42(s_tile2_bmp_1050_1538, 0x0);
        is_window = IsWindow16(pstruct35_5.hwnd_0xb6);
        if (is_window != 0) {
            // 0x1538
            unaff_CS = SUB42(s_tile2_bmp_1050_1538, 0x0);
            DestroyWindow16(pstruct35_5.hwnd_0xb6);
        }
    }
    pstruct35_5.hwnd_0xb6 = 0;
    puVar1 = pstruct35_5.field148_0x94;
    uVar1 = pstruct35_5.field149_0x96;
    if ((uVar1 | puVar1) != 0) {
        fn_ptr_1 = *puVar1;
        (**fn_ptr_1)(unaff_CS, puVar1, uVar1, 1);
    }
    pstruct35_5.field148_0x94 = 0;
    pstruct35_5.field150_0x98 = 0;
    return;
}


pub unsafe fn win_ui_op_1040_bbe2(param_1: *mut u8, param_2: HWND16, param_3: *mut astruct_900, mut param_4: u16, mut param_5: u16, mut param_6: u32)

{
    let mut uVar2: u16;
    let mut ppcVar3: *mut *mut code;
    let mut puVar5: *mut u32;
    let mut uVar6: u16;
    let mut BVar7: bool;
    let mut iVar7: i16;
    let mut uVar8: u16;
    let mut uVar7: u16;
    let mut uVar9: u16;
    let mut hwnd: HWND16;
    let mut uVar10: u16;
    let mut uVar13: u16;
    let mut uVar11: u16;
    let mut uVar12: u16;
    let mut in_register_0000000a: u16;
    let mut paVar14: *mut Struct57;
    let mut uVar15: u32;
    let mut puVar16: *mut u32;
    let mut paVar17: *mut Struct57;
    let mut uVar16: u32;
    let mut in_stack_0000fe84: u16;
    let mut in_stack_0000fe86: u16;
    let mut in_stack_0000ffa8: u16;
    let mut in_stack_0000ffaa: u16;
    let mut in_stack_0000ffae: u16;
    let mut in_stack_0000ffb0: u16;
    let mut in_stack_0000ffb2: u16;
    let mut in_stack_0000ffb4: u16;
    let mut uVar21: u16;
    let mut uStack30: u16;
    let mut local_a: RECT16;
    let mut iStack6: i16;
    let mut iStack4: i16;
    let mut uVar1: u32;
    let mut puVar4: *mut u32;
    let mut uVar17: u16;
    let mut uVar18: u16;
    let mut uVar19: u16;
    let mut in_stack_0000ffde: u16;
    let mut uVar20: u16;

    paVar14 = CONCAT22(in_register_0000000a, param_1);
    if (param_6 != 0x10c) {
        if (param_6 < 0x10d) {
            if (param_6 == 0xfa) {
                ppcVar3 = (*param_3.field148_0x98 + 0x18);
                (**ppcVar3)();
                return;
            }
            if (param_6 == 0x10a) {
                GetClientRect16(&local_a, &DAT_1050_1050);
                puVar5 = param_3.field148_0x98;
                local_a.y += 0x3;
                local_a.x = (puVar5 + 0x1a) - 0x9;
                iStack6 += -0x3;
                iStack4 += -0x3;
                InvalidateRect16(0x1, &local_a, &DAT_1050_1050);
                unk_destroy_win_op_1010_2fa0(param_3.field148_0x98);
                pass1_1010_32c0(param_3.field148_0x98, 0x0);
                pass1_1010_2ee2(param_3.field148_0x98);
                return;
            }
            if (param_6 != 0x10b) {//
// LAB_1040_be78:
                pass1_1040_b54a(param_1, CONCAT22(param_4, param_3), param_5, param_6);
                return;
            }
            puVar4 = param_3.field148_0x98;
            uVar2 = (puVar4 + 0x12);
            uVar21 = uVar2;
            puVar16 = mixed_1010_20ba(paVar14, _u16_1050_0ed0, CONCAT22(uVar2, 0x3), in_stack_0000fe84,
                                      in_stack_0000ffa8, in_stack_0000ffae, in_stack_0000ffb2);
            uVar8 = (puVar16 >> 0x10);
            uStack30 = puVar16;
            uVar6 = uStack30;
            uVar13 = uVar8;
            pass1_1010_a5ca(uStack30, uVar8, uStack30, uVar8, uVar21);
            if ((uVar2 != 0x70) && (uVar6 == 0)) {
                return;
            }
            uVar1 = param_3.field169_0xb0;
            uVar18 = uVar1;
            uVar19 = (uVar1 >> 0x10);
            puVar5 = param_3.field148_0x98;
            uVar17 = (puVar5 + 0x12);
        } else {
            if (param_6 != 0x10d) {
                if (param_6 == 0x10e) {
                    paVar17 = mixed_1010_20ba(paVar14, _u16_1050_0ed0, CONCAT22(param_2, 0x32), in_stack_0000fe86,
                                              in_stack_0000ffaa, in_stack_0000ffb0, in_stack_0000ffb4);
                    uVar15 = paVar14 & 0xffff0000 | paVar17 >> 0x10;
                    iVar7 = paVar17;
                    ui_op_1010_79aa(paVar17, 0xfc6, param_3.field169_0xb0);
                    if (iVar7 != 0) {
                        return;
                    }
                    unk_win_op_1010_7300(uVar15, paVar17, 0x0, 0x13, param_3.field169_0xb0);
                    return;
                }
                if (param_6 != 0xbbb) {
                    if (param_6 == 0xbbc) {
                        puVar16 = mixed_1010_20ba(paVar14, _u16_1050_0ed0, CONCAT22(param_2, 0x3), in_stack_0000fe86,
                                                  in_stack_0000ffaa, in_stack_0000ffb0, in_stack_0000ffb4);
                        uVar2 = (puVar16 >> 0x10);
                        uVar8 = puVar16;
                        uVar13 = uVar2;
                        uVar7 = pass1_1010_a5ac(uVar8, uVar2, param_3.field169_0xb0);
                        uVar9 = uVar7;
                        pass1_1010_a58a(uVar7, uVar13, uVar8, uVar2, uVar7);
                        if (uVar9 == 0) {
                            pass1_1010_a568(0x0, uVar13, uVar8, uVar2, uVar7);
                        }
                        hwnd = GetDlgItem16(0xbbc, param_3.field6_0x6);
                        EnableWindow16(0x0, hwnd);
                        return;
                    }
                    // TODO: goto LAB_1040_be78;
                }
                if ((param_3.field171_0xb6 == 0) || (BVar7 = IsWindow16(param_3.field171_0xb6), BVar7 == 0)) {
                    uVar16 = pass1_1038_af40(param_3, paVar14, _PTR_LOOP_1050_5b7c, param_3.field6_0x6, 0x1b);
                    param_3.field171_0xb6 = (uVar16 + 0x6);
                    ShowWindow16(0x1, param_3.field171_0xb6);
                    return;
                }
                param_2 = param_3.field171_0xb6;
                // TODO: goto LAB_1040_bd39;
            }
            puVar16 = mixed_1010_20ba(paVar14, _u16_1050_0ed0, CONCAT22(param_2, 0x3), in_stack_0000fe86,
                                      in_stack_0000ffaa, in_stack_0000ffb0, in_stack_0000ffb4);
            uVar13 = (puVar16 >> 0x10);
            uStack30 = puVar16;
            uVar15 = param_3.field169_0xb0;
            uVar18 = uVar15;
            uVar19 = (uVar15 >> 0x10);
            uVar17 = 0x71;
            uVar8 = uVar13;
        }
        pass1_1010_a5ec(uVar13, uStack30, uVar8, uVar17, CONCAT22(uVar19, uVar18));
        if ((param_3.field170_0xb4 != 0) && (BVar7 = IsWindow16(param_3.field170_0xb4), BVar7 != 0)) {
            SendMessage16(0x0, 0xeb, 0x111, param_3.field170_0xb4);
        }
    }
    param_2 = param_3.field6_0x6;//
// LAB_1040_bd39:
    DestroyWindow16(param_2);
    return;
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

pub unsafe fn invalidate_rect_1040_c028(mut param_1: u32, mut param_2: i16)

{
    let mut uVar1: u32;
    let mut uVar2: u32;
    let mut uVar3: u32;
    let mut uVar4: u16;
    let mut iVar5: i16;
    let mut iVar6: i16;
    let mut uVar7: u16;
    let mut iVar8: i16;
    let mut uVar10: u16;
    let mut erase: *mut RECT16;
    let mut rect: *mut RECT16;
    let mut hwnd: HWND16;
    let mut local_a: RECT16;
    let mut iStack6: i16;
    let mut iStack4: i16;
    let mut piVar9: *mut i16;

    iVar8 = param_1;
    uVar10 = (param_1 >> 0x10);
    if (param_2 == 0x8) {
        GetClientRect16(&local_a, &DAT_1050_1050);
        uVar1 = (iVar8 + 0x6);
        uVar3 = (iVar8 + 0x6);
        iVar6 = (uVar3 + 0x16);
        uVar3 = (iVar8 + 0x6);
        local_a.x = (uVar3 + 0x1a);
        uVar3 = (iVar8 + 0x6);
        local_a.y = (uVar3 + 0x1c);
        if (iVar6 != 0) {
            if (iVar6 < 0x2) {
                iVar5 = 0x1;
            } else {
                iVar5 = 0x2;
            }
            uVar2 = ((iVar6 - iVar5) * 0x4 + uVar1 + 0x2a);
            iVar6 = uVar2;
            uVar2 &= 0xffff0000;
            local_a.x = (iVar6 + 0x22) + (uVar2 | iVar6 + 0x1e);
        }
        uVar1 = (iVar8 + 0x6);
        iStack6 = (uVar1 + 0x1e);
        iStack4 += -0x5;
    } else {
        if (param_2 != 0x9) {
            if (param_2 != 0xa) {
                return;
            }
            uVar1 = (iVar8 + 0x6);
            uVar7 = uVar1 + 0x2a;
            if (((iVar8 + 0x8) | uVar7) == 0) {
                return;
            }
            uVar3 = (iVar8 + 0x6);
            uVar2 = (((uVar3 + 0x16) - 1) * 0x4 + uVar7);
            iVar8 = uVar2;
            uVar2 &= 0xffff0000;
            piVar9 = (uVar2 | iVar8 + 0x1e);
            uVar10 = (uVar2 >> 0x10);
            local_a.y = (iVar8 + 0x20) - 0x8;
            local_a.x = *piVar9;
            iStack6 = (iVar8 + 0x22) + *piVar9;
            iStack4 = (iVar8 + 0x20);
            rect = &local_a;
            hwnd = DAT_1050_1050;
            erase = null_mut();
            // TODO: goto LAB_1040_c19d;
        }
        local_a.x = 0;
        local_a.y = 0;
        iStack6 = 0;
        iStack4 = 0;
        GetClientRect16(&local_a, &DAT_1050_1050);
        uVar1 = (iVar8 + 0x6);
        local_a.x = (uVar1 + 0x1a);
        uVar1 = (iVar8 + 0x6);
        iStack6 = (uVar1 + 0x1e);
        iStack4 += -0x5;
        uVar1 = (iVar8 + 0x6);
        uVar3 = (iVar8 + 0x6);
        iVar6 = (uVar3 + 0x16);
        if (0x0 < iVar6) {
            uVar1 = (uVar1 + iVar6 * 0x4 + 0x26);
            iVar6 = uVar1;
            uVar4 = (uVar1 >> 0x10);
            local_a.y = (iVar6 + 0x20) + (iVar6 + 0x24);
        }
    }
    hwnd = (iVar8 + 0x4);
    erase = &local_a;
    rect = &DAT_1050_1050;//
// LAB_1040_c19d:
    InvalidateRect16(erase, rect, hwnd);
    return;
}

pub unsafe fn unk_draw_op_1040_c226(struct_param_1: *mut astruct_772)

{
    let mut handle: HPEN16;
    let mut obj_handle_var3: HGDIOBJ16;
    let mut iVar3: *mut astruct_772;
    let mut uVar4: u16;
    let mut hdc: HDC16;
    let mut rect_var_32: RECT16;
    let mut iStack46: i16;
    let mut iStack44: i16;
    let mut uStack42: u16;
    let mut iStack40: i16;
    let mut hbrush_var38: HBRUSH16;
    let mut hdc16_var36: HDC16;
//   PAINTSTRUCT16 *paintstruct_22;
    let mut paintstruct_22: *mut PAINTSTRUCT16;
    let mut uVar1: u32;
    let mut uVar2: u32;

    uVar4 = (struct_param_1 >> 0x10);
    iVar3 = struct_param_1;
    hdc16_var36 = BeginPaint16(CONCAT22(0x1050, &paintstruct_22), iVar3.hwnd_0x4);
    hbrush_var38 = CreateSolidBrush16(0x8000);
    GetClientRect16(&rect_var_32, &DAT_1050_1050);
    uVar1 = iVar3.field5_0x6;
    iStack40 = (uVar1 + 0x1a);
    uVar2 = iVar3.field5_0x6;
    uStack42 = (uVar2 + 0x1c);
    rect_var_32.y += 0x2;
    rect_var_32.x = iStack40 - 0xa;
    iStack46 += -0x2;
    iStack44 += -0x2;
    FrameRect16(hbrush_var38, &rect_var_32, &DAT_1050_1050);
    DeleteObject16(hbrush_var38);
    hdc = hdc16_var36;
    handle = CreatePen16(0x8080, 0x2, 0x0);
    obj_handle_var3 = SelectObject16(handle, hdc);
    draw_line_1040_c302(struct_param_1, hdc16_var36);
    draw_op_1040_c38e(struct_param_1);
    obj_handle_var3 = SelectObject16(obj_handle_var3, hdc16_var36);
    DeleteObject16(obj_handle_var3);
    EndPaint16(CONCAT22(0x1050, &paintstruct_22), iVar3.hwnd_0x4);
    return;
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


pub unsafe fn draw_op_1040_c74c(param_1: *mut astruct_738, mut param_2: u16, hdc16_param_3: HDC16, mut param_4: u16)

{
    let mut uVar2: u16;
    let mut hdc_black_brush_1: HGDIOBJ16;
    let mut pen_handle_1: HPEN16;
    let mut handle: HGDIOBJ16;
    let mut hpalette_1: HPALETTE16;
    let mut struct_1: *mut astruct_738;
    let mut uVar4: u16;
    let mut uVar5: u16;
    let mut uVar3: u32;
    let mut uVar1: u16;
    let mut func_ptr_1: *mut *mut code;

    uVar4 = (_PTR_LOOP_1050_4230 >> 0x10);
    uVar2 = (_PTR_LOOP_1050_4230 + 0x10);
    hpalette_1 = palette_op_1008_4e08(&hdc16_param_3, uVar2,
                                      CONCAT22(uVar2, (_PTR_LOOP_1050_4230 + 0xe)),
                                      CONCAT13(0x10, CONCAT12(0x50, &hdc16_param_3)));
    uVar5 = (param_1 >> 0x10);
    struct_1 = param_1;
    struct_1.field66_0x46 = 0x1;
    hdc_black_brush_1 = GetStockObject16(BLACK_BRUSH);
    pen_handle_1 = CreatePen16(0x1000002, 0x1, 0x0);
    hdc_black_brush_1 = SelectObject16(hdc_black_brush_1, hdc16_param_3);
    handle = SelectObject16(pen_handle_1, hdc16_param_3);
    Rectangle16(struct_1.field35_0x24, struct_1.field34_0x22, 0x0, 0x0, hdc16_param_3);
    MoveTo16(0x0, struct_1.field51_0x36 * 0x2 + struct_1.field40_0x2a, hdc16_param_3);
    LineTo16(struct_1.field35_0x24, struct_1.field51_0x36 * 0x2 + struct_1.field40_0x2a, hdc16_param_3);
    SelectObject16(hdc_black_brush_1, hdc16_param_3);
    hdc_black_brush_1 = SelectObject16(handle, hdc16_param_3);
    DeleteObject16(hdc_black_brush_1);
    uVar3 = param_1;
    func_ptr_1 = (uVar3 + 0x10);
    (**func_ptr_1)(s_tile2_bmp_1050_1538, param_1, _param_2);
    func_ptr_1 = (uVar3 + 0x14);
    (**func_ptr_1)(s_tile2_bmp_1050_1538, struct_1, (param_1 >> 0x10), hdc16_param_3);
    struct_1.field66_0x46 = 0;
    hpalette_1 = SelectPalette16(0x0, hpalette_1, hdc16_param_3);
    DeleteObject16(hpalette_1);
    return;
}


pub unsafe fn palette_op_1040_c886(struct_param_1: *mut astruct_769, param_2: u8, hdc_param_3: HDC16)

{
    let mut uVar1: u16;
    let mut palette_2: HPALETTE16;
    let mut uVar4: u16;
    let mut struct_2: *mut astruct_769;
    let mut uVar2: u16;
    let mut uVar3: u16;
    let mut unaff_CS: u16;
    let mut puStack8: *mut u32;
    let mut palette: HPALETTE16;
    let mut fn_ptr_1: *mut *mut code;

    uVar2 = (struct_param_1 >> 0x10);
    struct_2 = struct_param_1;
    if (((&struct_2.field8_0x8 + 0x2) | &struct_2.field8_0x8) != 0) {
        palette = 0;
        if (struct_2.field59_0x46 == 0) {
            uVar3 = (_PTR_LOOP_1050_4230 >> 0x10);
            uVar1 = (_PTR_LOOP_1050_4230 + 0x10);
            unaff_CS = 0x1008;
            palette = palette_op_1008_4e08(&hdc_param_3, uVar1,
                                           CONCAT22(uVar1, (_PTR_LOOP_1050_4230 + 0xe)),
                                           CONCAT22(0x1050, &hdc_param_3));
        }
        puStack8 = struct_2.field8_0x8;
        uVar4 = (&struct_2.field8_0x8 + 2);
        if ((((&struct_2.field9_0xc + 0x2) | &struct_2.field9_0xc) != 0) && ((param_2 & 1) != 0)) {
            puStack8 = struct_2.field9_0xc;
            uVar4 = (&struct_2.field9_0xc + 2);
        }
        if ((struct_2.field10_0x10.is_null() == false) && ((param_2 & 0x4) != 0)) {
            puStack8 = struct_2.field10_0x10;
            uVar4 = (&struct_2.field10_0x10 + 2);
        }
        fn_ptr_1 = (*puStack8 + 0x4);
        (**fn_ptr_1)(unaff_CS, puStack8, uVar4, struct_2.field30_0x28, struct_2.field29_0x26, &hdc_param_3,
                     &DAT_1050_1050);
        if (struct_2.field59_0x46 == 0) {
            palette_2 = SelectPalette16(0x0, palette, hdc_param_3);
            DeleteObject16(palette_2);
        }
    }
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
