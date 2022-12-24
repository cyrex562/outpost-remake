use std::ptr::null_mut;
use crate::draw_ops::unk_win_ui_op_1020_67ce;
use crate::draw_ops::draw_d::load_draw_op_1020_2ede;
use crate::draw_ops::draw_f::unk_win_ui_op_1020_717e;
use crate::globals::PTR_LOOP_1050_1040;
use crate::no_refs::m::pass1_1010_ecc6;
use crate::structs::struct_27::Struct27;
use crate::structs::struct_57::Struct57;
use crate::structs::struct_a::StructA;
use crate::structs::struct_d::StructD;
use crate::unk::block_1000_1000::{fn_ptr_op_1000_1708, mem_op_1000_160a, mem_op_1000_179c};
use crate::unk::block_1008_3000::{pass1_1008_3bd6, pass1_1008_3e0e, pass1_1008_3e38};
use crate::unk::block_1008_5000::{pass1_1008_5784, pass1_1008_5b12, win_1008_5c9e};
use crate::unk::block_1008_6000::pass1_1008_6978;
use crate::unk::block_1008_9000::{create_window_ex_1008_9760, pass1_1008_941a};
use crate::unk::block_1010_0000::{pass1_1010_088c, pass1_1010_0898, pass1_1010_089e, pass1_1010_091e, pass1_1010_0946};
use crate::unk::block_1010_2000::{mixed_1010_20ba, pass1_1010_2ee2};
use crate::unk::block_1010_3000::{pass1_1010_32c0, win_ui_op_1010_3202};
use crate::unk::block_1018_0000::{pass1_1018_0ac0, pass1_1018_0ad4, pass1_1018_0afa, pass1_1018_0b08};
use crate::unk::block_1018_1000::{pass1_1018_161c, pass1_1018_1662};
use crate::unk::block_1018_2000::{pass1_1018_25d2, pass1_1018_2d22};
use crate::unk::block_1020_0000::pass1_1020_0dc4;
use crate::unk::block_1020_2000::{pass1_1020_289a, pass1_1020_294a, pass1_1020_2a94};
use crate::unk::block_1020_5000;
use crate::unk::block_1020_6000::{pass1_1020_6498, pass1_1020_64d4};
use crate::unk::block_1040_b000::pass1_1040_b54a;
use crate::utils::CONCAT22;
use crate::gui::{cleanup, dialog, window};
use crate::gui::window::win_e;
use crate::winapi16::{BringWindowToTop16, DestroyWindow16, GetClientRect16, InvalidateRect16, LoadIcon16, MapDialogRect16, MoveWindow16, PostMessage16, PtInRect16, SetFocus16, ShowWindow16, UpdateWindow16};
use crate::winapp;
use crate::winapp::winapp_b::post_win_msg_1040_7b3c;
use crate::winapp::winapp_b::unk_win_msg_op_1008_9510;
use crate::winapp::winapp_e;
use crate::windef16::{HICON16, RECT16, WPARAM16};

pub unsafe fn ui_op_1020_536e(
    mut param_1: u16,
    mut param_2: u32,
    mut param_3: u32,
    mut param_4: i16,
    mut param_5: i16,
) {
    let mut piVar1: *mut i16;
    let mut UVar2: u16;
    let mut uVar3: u32;
    let mut ppcVar4: *mut *mut code;
    let mut uVar5: u16;
    let mut uVar6: u16;
    let mut UVar7: u16;
    let mut uVar8: u16;
    let mut puVar9: *mut u8;
    let mut uVar10: u16;
    let mut in_register_0000000a: u16;
    let mut paVar11: *mut Struct57;
    let mut uVar12: u32;
    let mut paVar13: *mut Struct57;
    let mut paVar14: *mut Struct57;
    let mut puVar15: *mut u32;
    let mut unaff_SI: u16;
    let mut uVar16: u16;
    let mut puVar17: *mut u32;
    let pSVar18: *mut StructA;
    let mut paVar19: *mut Struct27;
    let mut in_stack_0000fe88: u16;
    let mut in_stack_0000fe8a: u16;
    let mut in_stack_0000ff5c: u16;
    let mut in_stack_0000ff60: u16;
    let mut in_stack_0000ffac: u16;
    let mut in_stack_0000ffae: u16;
    let mut in_stack_0000ffb2: u16;
    let mut in_stack_0000ffb4: u16;
    let mut in_stack_0000ffb6: u16;
    let mut in_stack_0000ffb8: u16;
    let mut uVar20: u8;
    let mut uVar21: u8;
    let mut uVar22: u16;
    let mut uVar23: u16;
    let mut iVar24: i16;
    let mut puStack16: *mut u32;

    paVar11 = CONCAT22(in_register_0000000a, param_1);
    uVar8 = param_5 - 0x1;
    uVar16 = (param_2 >> 0x10);
    iVar24 = param_2;
    if (uVar8 == 0) {
        if ((iVar24 + 0xfe) == 0) {
            mem_op_1000_179c(0xfc, paVar11);
            uVar10 = paVar11 | uVar8;
            uVar12 = paVar11 & 0xffff0000 | uVar10;
            if (uVar10 == 0) {
                (iVar24 + 0xfe) = 0;
            } else {
                piVar1 = (iVar24 + 0xcc);
                *piVar1 = *piVar1 + 1;
                unk_win_ui_op_1020_67ce(
                    CONCAT13((paVar11 >> 0x8), CONCAT12(paVar11, uVar8)),
                    (iVar24 + 0xcc),
                    param_2,
                    uVar12,
                );
                (iVar24 + 0xfe) = uVar8;
                (iVar24 + 0x100) = uVar12;
            }
            pass1_1008_6978(uVar8, uVar12, param_2, 0x0, (iVar24 + 0xfe));
            uVar3 = (iVar24 + 0xfe);
            uVar22 = uVar3;
            uVar23 = (uVar3 >> 0x10);
            uVar3 = (iVar24 + 0xfe);
            uVar16 = (uVar3 >> 0x10);
            puVar15 = uVar3;
            // TODO: goto LAB_1020_53f3;
        }
    } else {
        if (param_5 == 0x2) {
            uVar5 = param_4 + 0xc;
            puVar17 = mixed_1010_20ba(
                paVar11,
                _u16_1050_0ed0,
                CONCAT22(unaff_SI, uVar5),
                in_stack_0000fe8a,
                in_stack_0000ffae,
                in_stack_0000ffb4,
                in_stack_0000ffb8,
            );
            paVar11 = (paVar11 & 0xffff0000 | puVar17 >> 0x10);
            uVar6 = pass1_1018_0afa(puVar17);
            if (uVar6 == 0) {
                piVar1 = (iVar24 + 0xcc);
                *piVar1 = *piVar1 + 1;
                UVar2 = (iVar24 + 0xcc);
                UVar7 = UVar2;
                mem_op_1000_179c(0xfe, paVar11);
                uVar8 = paVar11 | UVar7;
                paVar14 = (paVar11 & 0xffff0000);
                paVar13 = (paVar14 | uVar8);
                if (uVar8 == 0) {
                    UVar7 = 0;
                } else {
                    pass1_1020_289a(
                        CONCAT13((paVar11 >> 0x8), CONCAT12(paVar11, UVar7)),
                        UVar2,
                        param_2,
                    );
                    paVar14 = paVar13;
                }
                puVar9 = paVar14;
                puStack16 = CONCAT22(puVar9, UVar7);
                uVar20 = SUB41(paVar14, 0x0);
                uVar21 = (paVar14 >> 0x8);
                pass1_1020_294a(
                    puVar9,
                    CONCAT13(uVar21, CONCAT12(uVar20, UVar7)),
                    param_3,
                    uVar5,
                );
                uVar3 = *puStack16;
                ppcVar4 = (uVar3 + 0x8);
                uVar8 = (**ppcVar4)(0x1000, UVar7, puVar9);
                pass1_1008_3e0e(CONCAT13(uVar21, CONCAT12(uVar20, UVar7)));
                pass1_1008_6978(uVar8, paVar14, param_2, UVar2, CONCAT22(puVar9, UVar7));
                ppcVar4 = (uVar3 + 0xc);
                (**ppcVar4)(0x1008, UVar7, uVar20, 1);
            } else {
                pSVar18 = pass1_1018_0ad4(puVar17);
                paVar14 = (paVar11 & 0xffff0000 | pSVar18 >> 0x10);
                pass1_1008_3e0e(pSVar18);
            }
            pass1_1018_1662(puVar17, 0x0, 0x0);
            uVar3 = (iVar24 + 0xce);
            BringWindowToTop16((uVar3 + 0x8));
            uVar5 = 0x1;
            iVar24 = 0x4;
            paVar19 = mixed_1010_20ba(
                paVar14,
                _u16_1050_0ed0,
                0x1002b,
                in_stack_0000fe88,
                in_stack_0000ffac,
                in_stack_0000ffb2,
                in_stack_0000ffb6,
            );
            pass1_1010_089e(paVar19, uVar5, iVar24);
            pass1_1010_089e(paVar19, 0x1, 0x3);
            return;
        }
        uVar8 = param_5 - 0x3;
        if ((uVar8 == 0) && ((iVar24 + 0x102) == 0)) {
            mem_op_1000_179c(0xfc, paVar11);
            uVar10 = paVar11 | uVar8;
            uVar12 = paVar11 & 0xffff0000 | uVar10;
            if (uVar10 == 0) {
                (iVar24 + 0x102) = 0;
            } else {
                piVar1 = (iVar24 + 0xcc);
                *piVar1 = *piVar1 + 1;
                pass1_1020_0dc4(
                    CONCAT13((paVar11 >> 0x8), CONCAT12(paVar11, uVar8)),
                    (iVar24 + 0xcc),
                    param_2,
                    uVar12,
                    in_stack_0000ff5c,
                    in_stack_0000ff60,
                );
                (iVar24 + 0x102) = uVar8;
                (iVar24 + 0x104) = uVar12;
            }
            pass1_1008_6978(uVar8, uVar12, param_2, 0x0, (iVar24 + 0x102));
            uVar3 = (iVar24 + 0x102);
            uVar22 = uVar3;
            uVar23 = (uVar3 >> 0x10);
            uVar3 = (iVar24 + 0x102);
            uVar16 = (uVar3 >> 0x10);
            puVar15 = uVar3; //
            // LAB_1020_53f3:
            ppcVar4 = (*puVar15 + 0xc);
            (**ppcVar4)(0x8, uVar22, uVar23, 0x5);
            return;
        }
    }
    return;
}

pub unsafe fn destroy_window_1038_cd88(struct_b_param_1: *mut StructB)

{
   let mut struct_1: *mut StructB;
  let mut uVar1: u16;

  dialog::dialog_ui_fn_1040_78e2(struct_b_param_1);
  window::move_win_1040_826c(struct_b_param_1, -0x1, 0xffff);
  uVar1 = (struct_b_param_1 >> 0x10);
  struct_1 = struct_b_param_1;
  ShowWindow16(0x5,struct_1.lpvoid_field_0x8);
  struct_1[0x7].lpvoid_field_0x8 = (&PTR_LOOP_1050_0000 + 1);
  unk_win_msg_op_1008_9510((struct_b_param_1 & 0xffff0000 | ZEXT24(&struct_1[0x7].lpvoid_field_0x8)));
  DestroyWindow16(struct_1.lpvoid_field_0x8);
  return;
}

pub unsafe fn destroy_window_1038_c836(param_1: *mut astruct_881, mut param_2: u32, mut param_3: u32)

{
    let mut puVar1: *mut u16;
    let mut local_6: [u8; 0x4] = [0; 0x4];
    let mut uVar1: u32;

    if (param_3 == 0xfce) {
        puVar1 = pass1_1008_941a(CONCAT22(0x1050, local_6), 0x1, 0xac);
        win_1008_5c9e(local_6, (puVar1 >> 0x10), _u16_1050_02a0, CONCAT22(0x1050, local_6));
        uVar1 = param_1.field141_0x8e;
        (uVar1 + 0xa) = 0x6;
        DestroyWindow16(param_1.field6_0x6);
        PTR_LOOP_1050_5b80 = null_mut();
        return;
    }
    post_win_msg_1040_7b3c(CONCAT22(param_2, param_1), (param_2 >> 0x10), param_3, param_3);
    return;
}


pub unsafe fn win_cleanup_op_1040_748c(param_1: *mut u8, param_2: *mut astruct_898, mut param_3: u16, mut param_4: u16, mut param_5: u32)

{
    let mut puVar1: *mut u32;
    let mut iVar2: i16;
    let mut local_a: RECT16;
    let mut iStack6: i16;
    let mut iStack4: i16;
    let mut fn_ptr_1: *mut *mut code;

    match param_5 {
        0xfa => {
            fn_ptr_1 = (*param_2.field144_0x94 + 0x18);
            (**fn_ptr_1)();
        }
        // break;
        _ => {
            pass1_1040_b54a(param_1, CONCAT13((param_3 >> 0x8), CONCAT12(param_3, param_2)), param_4,
                            param_5);
            return;
        }
        0xfd => {
            if DAT_1050_0ecc == 0 {
                return;
            }
            DAT_1050_0ecc = 0;
        }
// TODO: goto LAB_1040_755d;
        0xfe => {
            if DAT_1050_0ecc == 1 {
                return;
            }
            DAT_1050_0ecc = 0x1;
        }
// TODO: goto LAB_1040_755d;
        0xff => {
            if DAT_1050_0ecc == 0x2 {
                return;
            }
            DAT_1050_0ecc = 0x2;//
// LAB_1040_755d:
            puVar1 = param_2.field144_0x94;
            fn_ptr_1 = (*param_2.field144_0x94 + 0x10);
            (**fn_ptr_1)(&PTR_LOOP_1050_1040, puVar1, (puVar1 >> 0x10));
            pass1_1010_2ee2(param_2.field144_0x94);
            PostMessage16(0x0, 0x10a, 0x111, param_2.field6_0x6);
        }
        // break;
        0x107 => {
            iVar2 = 0;
        }
// TODO: goto LAB_1040_75ba;
        0x108 => {
            iVar2 = 0x1;//
// LAB_1040_75ba:
            win_ui_op_1010_3202(param_2.field144_0x94, iVar2);
        }
        // break;
        0x10a => {
            GetClientRect16(&local_a, 0x1050);
            puVar1 = param_2.field144_0x94;
            local_a.y += 0x3;
            local_a.x = (puVar1 + 0x1a) - 0x9;
            iStack6 += -0x3;
            iStack4 += -0x3;
            InvalidateRect16(0x1, &local_a, 0x1050);
            cleanup::unk_destroy_win_op_1010_2fa0(param_2.field144_0x94);
            pass1_1010_32c0(param_2.field144_0x94, 0x0);
            pass1_1010_2ee2(param_2.field144_0x94);
        }
        // break;
        0x10c => {
            DestroyWindow16(param_2.field6_0x6);
        }
    }
    return;
}


pub unsafe fn window_op_1020_6c3a(
    param_1: *mut Struct57,
    struct_param_1: *mut StructA,
    mut param_3: u16,
    mut param_4: u16,
    mut param_5: u16,
    mut param_6: u16,
    mut param_7: u16,
    mut param_8: u16,
    mut param_9: u16,
    mut param_10: u16,
    mut param_11: u16,
    mut param_12: u16,
) {
    let mut uVar1: u32;
    let mut ppcVar2: *mut *mut code;
    let mut HVar3: HICON16;
    let mut paVar4: *mut Struct57;
    let mut pIVar5: *mut INT16 = null_mut();
    let mut uVar6: u32;
    let mut uVar7: u16;
    let mut uVar8: u16;
    let mut paVar9: *mut Struct57;
    let mut unaff_SI: u16;
    let mut puVar11: *mut u32;
    let mut uVar12: u8;
    let mut local_6: u32;
    let struct_a_1: *mut StructA;
    let mut paVar10: *mut Struct57;
    let mut struct_a_1_hi: u16;

    struct_a_1 = struct_param_1;
    struct_a_1_hi = (struct_param_1 >> 0x10);
    create_window_ex_1008_9760(struct_param_1);
    puVar11 = mixed_1010_20ba(
        param_1,
        _u16_1050_0ed0,
        CONCAT22(unaff_SI, 0x4),
        param_3,
        param_5,
        param_6,
        param_7,
    );
    paVar9 = (param_1 & 0xffff0000 | puVar11 >> 0x10);
    struct_a_1[0x1].field20_0x26 = puVar11;
    uVar7 = (puVar11 >> 0x10);
    struct_a_1[0x1].field21_0x28 = uVar7;
    struct_a_1[0x1].field10_0x14 = struct_a_1[0x1].field20_0x26;
    struct_a_1[0x1].field11_0x16 = uVar7;
    HVar3 = LoadIcon16(s_TILEICON_1050_440c, HINSTANCE16_1050_038c);
    struct_a_1.field_0xc2 = HVar3;
    uVar6 = &struct_a_1[0x1].field20_0x26;
    ppcVar2 = (*&struct_a_1[0x1].field20_0x26 + 0x30);
    (**ppcVar2)(0x1538, uVar6, (uVar6 >> 0x10), HVar3);
    paVar4 = (&local_6 + 2);
    pass1_1018_2d22(
        &struct_a_1[0x1].field20_0x26,
        CONCAT13(0x10, CONCAT12(0x50, &local_6)),
        CONCAT13(0x10, CONCAT12(0x50, paVar4)),
        0xbb8,
    );
    mem_op_1000_179c(0x42, paVar9);
    uVar8 = paVar9 | paVar4;
    paVar10 = (paVar9 & 0xffff0000 | uVar8);
    if (uVar8 != 0) {
        pass1_1008_3bd6(
            paVar10,
            paVar4,
            paVar9,
            0x0,
            local_6,
            0x0,
            0x7c007d,
            CONCAT22(struct_a_1.field4_0x8, 0xbb8),
            param_4,
            param_8,
            param_9,
            param_10,
            param_11,
            param_12,
        );
    }
    paVar4 = (&local_6 + 2);
    pass1_1018_2d22(
        &struct_a_1[0x1].field20_0x26,
        CONCAT13(0x10, CONCAT12(0x50, &local_6)),
        CONCAT13(0x10, CONCAT12(0x50, paVar4)),
        0xbb9,
    );
    mem_op_1000_179c(0x42, paVar10);
    uVar8 = paVar10 | paVar4;
    paVar9 = (paVar10 & 0xffff0000 | uVar8);
    if (uVar8 != 0) {
        pass1_1008_3bd6(
            paVar9,
            paVar4,
            paVar10,
            0x0,
            local_6,
            0x0,
            0x7e007f,
            CONCAT22(struct_a_1.field4_0x8, 0xbb9),
            param_4,
            param_8,
            param_9,
            param_10,
            param_11,
            param_12,
        );
    }
    paVar4 = (&local_6 + 2);
    pass1_1018_2d22(
        &struct_a_1[0x1].field20_0x26,
        CONCAT13(0x10, CONCAT12(0x50, &local_6)),
        CONCAT13(0x10, CONCAT12(0x50, paVar4)),
        0xbba,
    );
    mem_op_1000_179c(0x42, paVar9);
    uVar8 = paVar9 | paVar4;
    paVar10 = (paVar9 & 0xffff0000 | uVar8);
    if (uVar8 != 0) {
        pass1_1008_3bd6(
            paVar10,
            paVar4,
            paVar9,
            0x0,
            local_6,
            0x1b2,
            0x1b001b1,
            CONCAT22(struct_a_1.field4_0x8, 0xbba),
            param_4,
            param_8,
            param_9,
            param_10,
            param_11,
            param_12,
        );
    }
    mem_op_1000_179c(0x22, paVar10);
    uVar8 = paVar10 | paVar4;
    if (uVar8 == 0) {
        struct_a_1[0x1].field22_0x2a = 0;
    } else {
        unk_win_ui_op_1020_717e(uVar8, param_5, CONCAT22(paVar10, paVar4), struct_param_1);
        struct_a_1[0x1].field22_0x2a = paVar4;
        struct_a_1[0x1].field_0x2c = uVar8;
    }
    uVar6 = &struct_a_1[0x1].field22_0x2a;
    struct_a_1[0x1].field14_0x1c = uVar6;
    uVar1 = &struct_a_1[0x1].field20_0x26;
    ppcVar2 = (*&struct_a_1[0x1].field20_0x26 + 0x10);
    (**ppcVar2)(0x1000, uVar1, (uVar1 >> 0x10));
    pIVar5 = uVar6;
    MoveWindow16(
        0x1,
        pIVar5[0x3],
        pIVar5[0x2],
        pIVar5[0x1],
        *pIVar5,
        struct_a_1.field4_0x8,
    );
    uVar12 = (struct_param_1 >> 0x10);
    uVar6 = struct_param_1;
    ppcVar2 = (uVar6 + 0x94);
    (**ppcVar2)(0x1538, struct_a_1, uVar12, 0x0);
    ppcVar2 = (uVar6 + 0x10);
    (**ppcVar2)(0x1538, struct_a_1, uVar12, 1);
    UpdateWindow16(struct_a_1.field4_0x8);
    return;
}

pub unsafe fn mov_update_win_1040_93aa(param_1: *mut Struct65, param_2: INT16, mut param_3: u16 )

{
  let mut iVar1: *mut Struct65;
  let mut uVar1: u16;

  uVar1 = (param_1 >> 0x10);
  iVar1 = param_1;
  iVar1.field14_0x1e = param_3;
  iVar1.field15_0x20 = param_2;
  MoveWindow16(0x1,iVar1.field17_0x24,iVar1.field16_0x22,param_2,iVar1.field14_0x1e,iVar1.field11_0x18);
  UpdateWindow16(iVar1.field11_0x18);
  return;
}

pub unsafe fn win_ui_op_1020_2cf0(
    param_1: *mut Struct57,
    param_2: *mut StructA,
    mut param_3: u16,
    mut param_4: u16,
    mut param_5: u16,
    mut param_6: u16,
    mut param_7: u16,
) {
    let mut uVar1: u32;
    let mut ppcVar2: *mut *mut code;
    let mut paVar3: *mut astruct_160;
    let mut pIVar4: *mut INT16 = null_mut();
    let mut uVar5: u16;
    let mut uVar6: u16;
    let mut uVar7: u16;
    let mut paVar8: *mut Struct57;
    let mut unaff_SI: u16;
    let mut puVar9: *mut u32;
    let mut uVar10: u32;
    let pSVar11: *mut StructA;
    let mut uVar12: u16;
    let iVar10: *mut StructA;

    pSVar11 = param_2;
    uVar12 = (param_2 >> 0x10);
    create_window_ex_1008_9760(param_2);
    puVar9 = mixed_1010_20ba(
        param_1,
        _u16_1050_0ed0,
        CONCAT22(unaff_SI, pSVar11[0x1].field26_0x30),
        param_3,
        param_4,
        param_5,
        param_6,
    );
    paVar8 = (param_1 & 0xffff0000 | puVar9 >> 0x10);
    pSVar11[0x1].field20_0x26 = puVar9;
    uVar5 = (puVar9 >> 0x10);
    pSVar11[0x1].field21_0x28 = uVar5;
    pSVar11[0x1].field10_0x14 = pSVar11[0x1].field20_0x26;
    pSVar11[0x1].field11_0x16 = uVar5;
    paVar3 = LoadIcon16(s_SITEICON_1050_428d, HINSTANCE16_1050_038c);
    pSVar11.field_0xc2 = paVar3;
    uVar1 = &pSVar11[0x1].field20_0x26;
    ppcVar2 = (*&pSVar11[0x1].field20_0x26 + 0x30);
    (**ppcVar2)(0x1538, uVar1, (uVar1 >> 0x10), paVar3);
    mem_op_1000_179c(0x22, paVar8);
    uVar6 = paVar8 | paVar3;
    uVar10 = paVar8 & 0xffff0000 | uVar6;
    if (uVar6 == 0) {
        pSVar11[0x1].field22_0x2a = 0;
    } else {
        load_draw_op_1020_2ede(uVar10, param_7, CONCAT22(paVar8, paVar3), pSVar11, uVar12);
        pSVar11[0x1].field22_0x2a = paVar3;
        pSVar11[0x1].field_0x2c = uVar10;
    }
    pSVar11[0x1].field14_0x1c = &pSVar11[0x1].field22_0x2a;
    pass1_1018_0ac0(&pSVar11[0x1].field20_0x26, param_2);
    uVar10 = pass1_1018_0b08(&pSVar11[0x1].field20_0x26);
    uVar7 = (uVar10 >> 0x10);
    pIVar4 = uVar10;
    ppcVar2 = (param_2 + 0x14);
    (**ppcVar2)();
    ppcVar2 = (*&pSVar11[0x1].field20_0x26 + 0x10);
    (**ppcVar2)();
    MoveWindow16(
        0x1,
        pIVar4[0x3],
        pIVar4[0x2],
        pIVar4[0x1],
        *pIVar4,
        pSVar11.field4_0x8,
    );
    pass1_1008_3e0e(param_2);
    UpdateWindow16(pSVar11.field4_0x8);
    return;
}


pub unsafe fn pt_in_rect_op_1020_58ce(
    mut param_1: u16,
    mut param_2: u32,
    mut param_3: u16,
    mut param_4: u16,
    param_5: u8,
) {
    let mut uVar1: u16;
    let mut uVar2: u32;
    let mut ppcVar3: *mut *mut code;
    let mut uVar4: u32;
    let mut uVar5: u16;
    let mut BVar6: bool;
    let mut puVar7: *mut u16;
    let mut uVar8: u16;
    let mut in_register_0000000a: u16;
    let mut paVar9: *mut Struct57;
    let mut iVar10: i16;
    let mut iVar11: i16;
    let mut uVar12: u16;
    let mut uVar13: u16;
    let mut uVar14: u32;
    let mut puVar15: *mut u16;
    let mut puVar16: *mut u16;
    let mut puVar17: *mut u32;
    let mut in_stack_0000fe74: u16;
    let mut in_stack_0000ff98: u16;
    let mut in_stack_0000ff9e: u16;
    let mut in_stack_0000ffa2: u16;
    let mut wparam: WPARAM16;
    let mut in_stack_0000ffcc: u16;
    let mut uStack46: u16;
    let mut iStack26: i16;
    let mut local_18: [u16; 0x2] = [0; 0x2];
    let mut uStack20: u16;
    let mut uStack18: u32;
    let mut pRStack14: *mut RECT16;
    let mut uStack12: u16;
    let mut uStack10: u16;
    let mut uStack8: u16;
    let mut local_6: u16;
    let mut uStack4: u16;

    paVar9 = CONCAT22(in_register_0000000a, param_1);
    local_6 = param_4;
    uStack4 = param_3;
    uStack8 = param_5 & 0x8;
    uStack10 = param_5 & 0x4;
    uVar12 = (param_2 >> 0x10);
    iVar10 = param_2;
    uVar5 = pass1_1020_64d4((iVar10 + 0xf6), 0x2);
    if (uVar5 == 0) {
        //
        // LAB_1020_5942:
        uVar5 = pass1_1020_64d4((iVar10 + 0xf6), 0x4);
        if (uVar5 == 0) {
            //
            // LAB_1020_5a16:
            uVar5 = pass1_1020_64d4((iVar10 + 0xf6), 1);
            if (uVar5 != 0) {
                uVar14 = pass1_1020_6498((iVar10 + 0xf6), 1);
                paVar9 = (paVar9 & 0xffff0000 | uVar14 >> 0x10);
                for iStack26 in 0..0x4 {
                    paVar9 = (paVar9 & 0xffff0000 | uVar14 >> 0x10);
                    BVar6 = PtInRect16(CONCAT22(uStack4, local_6), (iStack26 * 0x8 + uVar14));
                    if (BVar6 != 0) {
                        local_18[0] = 0;
                        uStack20 = 0;
                        if (iStack26 == 0) {
                            uStack20 = (-(uStack10 == 0) & 0x4) - 0x5;
                        } else if (iStack26 == 1) {
                            uStack20 = (-(uStack10 == 0) & 0xfffc) + 0x5;
                        } else if (iStack26 == 0x2) {
                            local_18[0] = (-(uStack10 == 0) & 0x4) - 0x5;
                        } else if (iStack26 == 0x3) {
                            local_18[0] = (-(uStack10 == 0) & 0xfffc) + 0x5;
                        }
                        pass1_1020_2a94((iVar10 + 0xce), CONCAT22(local_18[0], uStack20));
                        return;
                    }
                }
            }
            uVar5 = pass1_1020_64d4((iVar10 + 0xf6), 0x3);
            if (uVar5 != 0) {
                puVar7 = &local_6;
                winapp_e::pt_in_rect_1020_5856(puVar7, paVar9, param_2, CONCAT22(0x1050, puVar7));
                uVar8 = paVar9;
                uVar14 = (uVar8 | puVar7);
                if ((uVar8 | puVar7) != 0) {
                    uVar5 = puVar7[0x17];
                    if (((uStack8 == 0) || (uStack10 == 0)) && (uStack10 == 0)) {
                        local_18[0] = 0x1;
                    } else {
                        local_18[0] = 0x2;
                    }
                    uStack20 = puVar7[0x6];
                    uStack18 = CONCAT22(uStack18, puVar7[0x7]);
                    if ((uVar5 == 0xb) || (uVar5 == 0x37)) {
                        uVar4 = (iVar10 + 0xfa);
                        uVar13 = (uVar4 >> 0x10);
                        iVar11 = uVar4;
                        uVar2 = (iVar11 + 0x20);
                        uVar1 = (iVar11 + 0x22);
                        uVar14 = paVar9 & 0xffff0000 | uVar1;
                        uStack46 = uVar2;
                        if ((uVar1 | uStack46) != 0) {
                            puVar16 = pass1_1008_3e38(CONCAT22(0x1050, &stack0xffcc));
                            paVar9 = (uVar14 & 0xffff0000 | puVar16 >> 0x10);
                            pass1_1018_161c(
                                uVar2,
                                CONCAT22(0x1050, &stack0xffcc),
                                uStack18,
                                uStack20,
                            );
                            puVar17 = mixed_1010_20ba(
                                paVar9,
                                _u16_1050_0ed0,
                                CONCAT22(in_stack_0000ffcc, 0x2f),
                                in_stack_0000fe74,
                                in_stack_0000ff98,
                                in_stack_0000ff9e,
                                in_stack_0000ffa2,
                            );
                            uVar14 = puVar17 >> 0x10;
                            pass1_1010_ecc6(
                                &stack0xffcc,
                                (puVar17 >> 0x10),
                                puVar17,
                                CONCAT22(0x1050, &stack0xffcc),
                                (uStack46 + 0x3c),
                            );
                        }
                    }
                    uVar13 = uVar14;
                    uVar5 = pass1_1018_25d2(
                        (iVar10 + 0xfa),
                        local_18[0],
                        uStack18 & 0xffff | uStack20 << 0x10,
                    );
                    if (uVar5 != 0) {
                        return;
                    }
                    uVar5 = block_1020_5000::pass1_1020_5d56(uVar13, param_2, CONCAT22(uVar8, puVar7));
                    if (uVar5 != 0) {
                        return;
                    }
                }
            }
            return;
        }
        uVar14 = pass1_1020_6498((iVar10 + 0xf6), 0x4);
        paVar9 = (paVar9 & 0xffff0000 | uVar14 >> 0x10);
        pRStack14 = uVar14;
        uStack12 = (uVar14 >> 0x10);
        BVar6 = PtInRect16(CONCAT22(uStack4, local_6), pRStack14);
        //    if (BVar6 == 0) goto LAB_1020_5a16;
        uStack18 = mixed_1010_20ba(
            paVar9,
            _u16_1050_0ed0,
            CONCAT22(in_stack_0000ffcc, 0x2),
            in_stack_0000fe74,
            in_stack_0000ff98,
            in_stack_0000ff9e,
            in_stack_0000ffa2,
        );
        if ((uStack18 + 0x72) != 0) {
            (iVar10 + 0x116) = 0x1;
            if (param_2 == 0) {
                iVar10 = 0;
                uVar12 = 0;
            } else {
                iVar10 += 0xe2;
            }
            ppcVar3 = (*_u16_1050_02a0 + 0x4);
            (**ppcVar3)(
                0x1010,
                _u16_1050_02a0,
                (_u16_1050_02a0 >> 0x10),
                0x10,
                iVar10,
                uVar12,
            );
            puVar15 = pass1_1008_941a(CONCAT22(0x1050, local_18), 0x1, 0x86);
            puVar7 = local_18;
            win_1008_5c9e(
                puVar7,
                (puVar15 >> 0x10),
                _u16_1050_02a0,
                CONCAT22(0x1050, puVar7),
            );
            if (puVar7.is_null() == false) {
                return;
            }
            wparam = 0xf6;
            // TODO: goto LAB_1020_5936;
        }
        wparam = 0xf6;
    } else {
        uVar14 = pass1_1020_6498((iVar10 + 0xf6), 0x2);
        paVar9 = (paVar9 & 0xffff0000 | uVar14 >> 0x10);
        pRStack14 = uVar14;
        uStack12 = (uVar14 >> 0x10);
        BVar6 = PtInRect16(CONCAT22(uStack4, local_6), pRStack14);
        //    if (BVar6 == 0) goto LAB_1020_5942;
        wparam = 0x68;
    }
    puVar7 = null_mut(); //
    // LAB_1020_5936:
    PostMessage16(CONCAT22(puVar7, puVar7), wparam, 0x111, HWND16_1050_0396);
    return;
}

pub unsafe fn win_ui_op_1038_e348(param_1: *mut StructB, param_2: u8, param_3: *mut StructD, mut param_4: u16)

{
    let mut pvVar1: LPVOID = null_mut();
    let mut uVar3: u32;
    let mut uVar5: u16;
    let mut uVar4: u16;
    let mut rect: *mut Struct57;
    let mut uVar7: u16;
    let mut uVar6: *mut StructD;
    let mut uVar11: u16;
    let mut struct_b_5: *mut StructB;
    let mut iVar12: i16;
    let mut unaff_SI: u16;
    let mut uVar8: u16;
    let mut uVar10: u16;
    let mut uVar9: u16;
    let mut in_stack_0000fe2a: u16;
    let mut in_stack_0000fe2e: u16;
    let mut in_stack_0000fe7e: u16;
    let mut in_stack_0000ff54: u16;
    let mut in_stack_0000ff58: u16;
    let mut in_stack_0000ff5c: u16;
    let mut in_stack_0000ffa2: u16;
    let mut in_stack_0000ffa8: u16;
    let mut in_stack_0000ffac: u16;
    let mut local_22: u16;
    let mut uStack32: u16;
    let mut uStack30: u16;
    let mut uStack28: u16;
    let mut puStack26: *mut u16;
    let mut iStack10: i16;
    let mut uStack8: u16;
    let mut puStack6: *mut u32;
    let mut uVar2: u32;
    let mut paVar8: *mut Struct57;

    dialog::dialog_ui_fn_1040_78e2(param_1);
    uVar6 = param_3;
    puStack6 = mixed_1010_20ba(param_3, _u16_1050_0ed0, CONCAT22(unaff_SI, 0x2b),
                               in_stack_0000fe7e, in_stack_0000ffa2, in_stack_0000ffa8, in_stack_0000ffac);
    uVar6 = (uVar6 & 0xffff0000 | puStack6 >> 0x10);
    uStack8 = pass1_1010_088c();
    if (_PTR_LOOP_1050_5f2c == 0) {
        PTR_LOOP_1050_5f2c = mem_op_1000_160a(uVar6);
    } else {
        uVar6 = (uVar6 & 0xffff0000 | _PTR_LOOP_1050_5f2c >> 0x10);
    }
    puStack26 = CONCAT22(uVar6, PTR_LOOP_1050_5f2c);
    uVar4 = fn_ptr_op_1000_1708((uStack8 + 0x2) * 0x4, 0x0, 0x1, PTR_LOOP_1050_5f2c, uVar6);
    uVar8 = (param_1 >> 0x10);
    struct_b_5 = param_1;
    struct_b_5[0x7].field1_0x2 = uVar4;
    struct_b_5[0x7].hwnd_0x6 = uVar6;
//   for (iStack10 = 0x1; uVar11 = (uVar6 >> 0x10), iStack10 <= uStack8; iStack10 += 1)
    iStack10 = 1;
    uVar11 = uVar6 >> 0x10;
    while iStack10 <= uStack8 {
        puStack26 = pass1_1010_091e(puStack6, (puStack6 >> 0x10), iStack10);
        uVar5 = (puStack26 >> 0x10);
        paVar8 = CONCAT22(uVar11, uVar5);
        local_22 = *puStack26;
        uStack32 = (puStack26 + 2);
        uStack30 = 0x1;
        uStack28 = 0x1;
        rect = &local_22;
        MapDialogRect16(rect, 0x1050);
        mem_op_1000_179c(0x42, paVar8);
        uVar7 = paVar8 | rect;
        uVar6 = (paVar8 & 0xffff0000 | uVar7);
        if (uVar7 == 0) {
            uVar3 = &struct_b_5[0x7].field1_0x2;
            (uVar3 + iStack10 * 0x4) = 0;
        } else {
            pvVar1 = struct_b_5.lpvoid_field_0x8;
            pass1_1008_3bd6(uVar6, rect, paVar8, 0x0, CONCAT22(local_22, uStack32), 0x101, 0xff0100,
                            CONCAT13((pvVar1 >> 0x8), CONCAT12(pvVar1, (puStack26 + 0x4))), param_4, in_stack_0000fe2a, in_stack_0000fe2e, in_stack_0000ff54, in_stack_0000ff58, in_stack_0000ff5c,
            );
            uVar2 = &struct_b_5[0x7].field1_0x2;
            uVar9 = (uVar2 >> 0x10);
            iVar12 = uVar2;
            (iVar12 + iStack10 * 0x4) = rect;
            (iVar12 + iStack10 * 0x4 + 0x2) = uVar6;
        }
        uVar3 = &struct_b_5[0x7].field1_0x2;
        uVar10 = (uVar3 >> 0x10);
        iVar12 = uVar3;
        if ((iVar12 + iStack10 * 0x4) != 0) {
            win_e::enable_win_1040_9234((iVar12 + iStack10 * 0x4), (puStack26 + 0x6));
        }
        iStack10 += 1;
    }
    window::move_win_1040_826c(param_1, -0x1, 0xffff);
    ShowWindow16(0x5, struct_b_5.lpvoid_field_0x8);
    return;
}


pub unsafe fn show_win_1010_7a76(mut param_1: u32) {
    let mut uVar1: u32;
    let mut iVar2: i16;
    let mut uVar3: u16;
    let mut lVar4: i32;
    let mut local_a: [u8; 0x8] = [0; 0x8];

    uVar3 = (param_1 >> 0x10);
    iVar2 = param_1;
    if ((iVar2 + 0x20) == 0) {
        (iVar2 + 0x20) = 0x1;
        pass1_1008_5784(CONCAT22(0x1050, local_a), (iVar2 + 0x1c));
        loop {
            lVar4 = pass1_1008_5b12(CONCAT22(0x1050, local_a));
            if (lVar4 == 0) {
                break;
            }
            uVar1 = (lVar4 + 0x8);
            ShowWindow16(0x0, (uVar1 + 0x6));
        }
    }
    return;
}


pub unsafe fn show_win_1040_355a(param_1: *mut StructB)

{
    let mut uVar1: u16;

    dialog::dialog_ui_fn_1040_78e2(param_1);
    window::move_win_1040_826c(param_1, -0x1, 0xffff);
    uVar1 = (param_1 >> 0x10);
    ShowWindow16(0x5, (param_1 + 0x6));
    SetFocus16((param_1 + 0x6));
    return;
}


pub unsafe fn show_win_1040_b43c(param_1: u32)

{
    let mut ppcVar1: *mut *mut code;

    ppcVar1 = (*param_1 + 0x70);
    (**ppcVar1)();
    ShowWindow16(0x5, (param_1 + 0x6));
    return;
}


pub unsafe fn show_win_1040_65ba(param_1: *mut StructD, struct_b_param_1: *mut StructB, mut param_3: u16)

{
    let mut pvVar1: LPVOID = null_mut();
    let mut uVar2: u32;
    let mut uVar3: u16;
    let mut rect: *mut Struct57;
    let mut uVar4: u16;
    let mut uVar5: *mut StructD;
    let mut paVar5: *mut Struct57;
    let mut struct_b_4: *mut StructB;
    let mut iVar6: i16;
    let mut unaff_SI: u16;
    let mut unaff_DI: i16;
    let mut uVar7: u16;
    let mut uVar8: u16;
    let mut in_stack_0000fe2a: u16;
    let mut in_stack_0000fe2e: u16;
    let mut in_stack_0000fe7e: u16;
    let mut in_stack_0000ff54: u16;
    let mut in_stack_0000ff58: u16;
    let mut in_stack_0000ff5c: u16;
    let mut in_stack_0000ffa2: u16;
    let mut in_stack_0000ffa8: u16;
    let mut in_stack_0000ffac: u16;
    let mut local_22: u16;
    let mut uStack32: u16;
    let mut uStack30: u16;
    let mut uStack28: u16;
    let mut puStack26: *mut u16;
    let mut iStack10: i16;
    let mut uStack8: u16;
    let mut puStack6: *mut u32;

    dialog::dialog_ui_fn_1040_78e2(struct_b_param_1);
    uVar5 = param_1;
    puStack6 = mixed_1010_20ba(param_1, _u16_1050_0ed0, CONCAT22(unaff_SI, 0x2b),
                               in_stack_0000fe7e, in_stack_0000ffa2, in_stack_0000ffa8, in_stack_0000ffac);
    uVar5 = (uVar5 & 0xffff0000 | puStack6 >> 0x10);
    uStack8 = pass1_1010_0898();
    if (_PTR_LOOP_1050_5f2c == 0) {
        PTR_LOOP_1050_5f2c = mem_op_1000_160a(uVar5);
    } else {
        uVar5 = (uVar5 & 0xffff0000 | _PTR_LOOP_1050_5f2c >> 0x10);
    }
    puStack26 = CONCAT22(uVar5, PTR_LOOP_1050_5f2c);
    uVar3 = fn_ptr_op_1000_1708((uStack8 + 0x2) * 0x4, 0x0, 0x1, PTR_LOOP_1050_5f2c, uVar5);
    uVar7 = (struct_b_param_1 >> 0x10);
    struct_b_4 = struct_b_param_1;
    struct_b_4[0x7].field1_0x2 = uVar3;
    struct_b_4[0x7].hwnd_0x6 = uVar5;
//   for (iStack10 = 0x1; iStack10 <= uStack8; iStack10 += 1)
    for iStack10 in 1..uStack8 {
        puStack26 = pass1_1010_0946(puStack6, (puStack6 >> 0x10), iStack10, uVar5, unaff_DI,
                                    0x1050);
        paVar5 = (uVar5 & 0xffff0000 | puStack26 >> 0x10);
        local_22 = *puStack26;
        uStack32 = (puStack26 + 2);
        uStack30 = 0x1;
        uStack28 = 0x1;
        rect = &local_22;
        MapDialogRect16(rect, 0x1050);
        mem_op_1000_179c(0x42, paVar5);
        uVar4 = paVar5 | rect;
        uVar5 = (paVar5 & 0xffff0000 | uVar4);
        if (uVar4 == 0) {
            uVar2 = &struct_b_4[0x7].field1_0x2;
            (uVar2 + iStack10 * 0x4) = 0;
        } else {
            pvVar1 = struct_b_4.lpvoid_field_0x8;
            pass1_1008_3bd6(uVar5, rect, paVar5, 0x0, CONCAT22(local_22, uStack32), 0x101, 0xff0100,
                            CONCAT13((pvVar1 >> 0x8), CONCAT12(pvVar1, (puStack26 + 0x4))), param_3, in_stack_0000fe2a, in_stack_0000fe2e, in_stack_0000ff54, in_stack_0000ff58, in_stack_0000ff5c,
            );
            uVar2 = &struct_b_4[0x7].field1_0x2;
            uVar8 = (uVar2 >> 0x10);
            iVar6 = uVar2;
            (iVar6 + iStack10 * 0x4) = rect;
            (iVar6 + iStack10 * 0x4 + 0x2) = uVar5;
        }
        uVar2 = &struct_b_4[0x7].field1_0x2;
        uVar8 = (uVar2 >> 0x10);
        iVar6 = uVar2;
        if ((iVar6 + iStack10 * 0x4) != 0) {
            unaff_DI = puStack26;
            win_e::enable_win_1040_9234((iVar6 + iStack10 * 0x4), (unaff_DI + 0x6));
        }
    }
    window::move_win_1040_826c(struct_b_param_1, -0x1, 0xffff);
    ShowWindow16(0x5, struct_b_4.lpvoid_field_0x8);
    return;
}
