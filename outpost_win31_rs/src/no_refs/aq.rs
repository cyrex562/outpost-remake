use std::os::raw::c_char;
use std::ptr::null_mut;
use crate::app_context::AppContext;
use crate::block_1000::block_1000_1000::{fn_ptr_1000_17ce, mem_op_1000_179c};
use crate::block_1008::block_1008_3000::{pass1_1008_3bd6, pass1_1008_3e94};
use crate::block_1008::block_1008_4000::{pass1_1008_4d84, struct_1008_4c58};
use crate::block_1008::block_1008_5000::{pass1_1008_5fd8, win_1008_5c5c, win_1008_5c7c};
use crate::block_1010::block_1010_2000::mixed_1010_20ba;
use crate::block_1010::block_1010_8000::{load_string_1010_847e, load_string_1010_84e0};
use crate::block_1018::block_1018_7000::set_window_text_1018_6086;
use crate::block_1030::block_1030_2000::{pass1_1030_2f1a, pass1_1030_2fac};
use crate::block_1030::block_1030_5000::pass1_1030_532e;
use crate::block_1030::block_1030_8000::{fn_ptr_1030_835a, pass1_1030_8334, pass1_1030_8344, pass1_1030_838e};
use crate::block_1038::block_1038_a000::pass1_1038_af40;
use crate::block_1038::block_1038_e000::{destroy_win_1038_ef3a, pass1_1038_e9ec, pass1_1038_ebd6};
use crate::block_1040::block_1040_0000::{enable_win_1040_060e, enable_window_1040_0acc, pass1_1040_073a, win_ui_op_1040_0558};
use crate::block_1040::block_1040_7000::{dialog_ui_fn_1040_78e2, post_win_msg_1040_7b3c};
use crate::block_1040::block_1040_8000::move_win_1040_826c;
use crate::globals::u32_1050_0004;
use crate::structs::struct_394::astruct_394;
use crate::structs::struct_57::Struct57;
use crate::structs::struct_915::astruct_915;
use crate::structs::struct_d::StructD;
use crate::utils::{CONCAT11, CONCAT22, SUB42};
use crate::winbase::{DestroyWindow16, GetDlgItem16, GetWindowRect16, MapDialogRect16, SetFocus16, SetWindowPos16, ShowWindow16};
use crate::windef::{HCURSOR16, HWND16, LRESULT, WPARAM16};

pub unsafe fn win_ui_op_1038_ea18(param_1: *mut StructB)

{
    let mut hwnd: HWND16;
    let mut IVar1: i16;
    let mut iVar2: *mut StructB;
    let mut uVar2: u16;
    let mut lparam: u32;
    let mut in_stack_0000fff0: bool;
    let mut iStack14: i16;

    dialog_ui_fn_1040_78e2(param_1);
    uVar2 = (param_1 >> 0x10);
    iVar2 = param_1;
    lparam = pass1_1010_375e(&iVar2[0x7].field1_0x2);
    hwnd = GetDlgItem16(0xfa5, iVar2.lpvoid_field_0x8);
    SendMessage16(lparam, 0x0, 0xc, hwnd);
    GetWindowRect16(CONCAT22(0x1050, &stack0xfff0), iVar2.max_count_field_0x10);
    IVar1 = GetSystemMetrics16(SM_CYCAPTION);
    move_win_1040_826c(param_1, IVar1 + iStack14 + 0x5, in_stack_0000fff0);
    ShowWindow16(0x5, iVar2.lpvoid_field_0x8);
    return;
}

pub unsafe fn win_ui_op_1038_eaa2(param_1: *mut astruct_888, mut param_2: i16)

{
    let mut hwnd: HWND16;
    let mut struct_1: *mut astruct_888;
    let mut struct_1_lo: u16;
    let mut LVar1: LRESULT;

    struct_1 = param_1;
    struct_1_lo = (param_1 >> 0x10);
    if (param_2 != 0) {
        hwnd = GetDlgItem16(0xfa5, struct_1.field6_0x6);
        LVar1 = SendMessage16(CONCAT22(0x1050, &stack0xffac), 0x50, 0xd, hwnd);
        pass1_1010_3770((LVar1 >> 0x10), struct_1.field140_0x8e,
                        CONCAT22(0x1050, &stack0xffac));
        PostMessage16(0x0, 0xfb, 0x111, struct_1.field7_0x8);
    }
    DestroyWindow16(struct_1.field6_0x6);
    return;
}

pub unsafe fn FUN_1038_eb08()

{
    return;
}


pub unsafe fn pass1_1038_eb0c(param_1: *mut StructD, param_2: u8) -> *mut StructD

{
    pass1_1038_e9ec(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}


pub unsafe fn FUN_1038_ec16(mut param_1: u16, param_2: *mut StructB, param_3: *mut Struct57, mut param_4: u16)

{
    let mut uVar1: u32;
    let mut uVar2: u16;
    let mut rect: *mut Struct57;
    let mut uVar3: u16;
    let mut uVar4: u16;
    let mut pSVar5: *mut StructD;
    let mut iVar7: i16;
    let mut iVar8: i16;
    let mut unaff_SI: u16;
    let mut uVar9: u16;
    let mut uVar10: u16;
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
    let mut paVar6: *mut Struct57;

    dialog_ui_fn_1040_78e2(param_2);
    puStack6 = mixed_1010_20ba(param_3, _u16_1050_0ed0, CONCAT22(unaff_SI, 0x2b), in_stack_0000fe7e,
                               in_stack_0000ffa2, in_stack_0000ffa8, in_stack_0000ffac);
    pSVar5 = (param_3 & 0xffff0000 | puStack6 >> 0x10);
    uStack8 = pass1_1010_0892();
    if (_PTR_LOOP_1050_5f2c == 0) {
        PTR_LOOP_1050_5f2c = mem_op_1000_160a(pSVar5);
    } else {
        pSVar5 = (pSVar5 & 0xffff0000 | _PTR_LOOP_1050_5f2c >> 0x10);
    }
    puStack26 = CONCAT22(pSVar5, PTR_LOOP_1050_5f2c);
    uVar2 = fn_ptr_op_1000_1708((uStack8 + 0x2) * 0x4, 0x0, 0x1, PTR_LOOP_1050_5f2c, pSVar5);
    uVar9 = (param_2 >> 0x10);
    iVar7 = param_2;
    (iVar7 + 0x8e) = uVar2;
    (iVar7 + 0x90) = pSVar5;
//   for (iStack10 = 0x1; uVar10 = (pSVar5 >> 0x10), iStack10 <= uStack8; iStack10 += 1)
    iStack10 = 1;
    uVar10 = pSVar5 >> 0x10;
    while iStack10 <= uStack8 {
        puStack26 = pass1_1010_0932(puStack6, (puStack6 >> 0x10), iStack10);
        uVar3 = (puStack26 >> 0x10);
        paVar6 = CONCAT22(uVar10, uVar3);
        local_22 = *puStack26;
        uStack32 = (puStack26 + 2);
        uStack30 = 0x1;
        uStack28 = 0x1;
        rect = &local_22;
        MapDialogRect16(rect, &DAT_1050_1050);
        mem_op_1000_179c(0x42, paVar6);
        uVar4 = paVar6 | rect;
        pSVar5 = (paVar6 & 0xffff0000 | uVar4);
        if (uVar4 == 0) {
            uVar1 = (iVar7 + 0x8e);
            (uVar1 + iStack10 * 0x4) = 0;
        } else {
            uVar10 = (iVar7 + 0x6);
            pass1_1008_3bd6(pSVar5, rect, paVar6, 0x0, CONCAT22(local_22, uStack32), 0x101, 0xff0100,
                            CONCAT13((uVar10 >> 0x8), CONCAT12(uVar10, (puStack26 + 0x4))), param_4, in_stack_0000fe2a, in_stack_0000fe2e, in_stack_0000ff54, in_stack_0000ff58, in_stack_0000ff5c,
            );
            uVar1 = (iVar7 + 0x8e);
            uVar10 = (uVar1 >> 0x10);
            iVar8 = uVar1;
            (iVar8 + iStack10 * 0x4) = rect;
            (iVar8 + iStack10 * 0x4 + 0x2) = pSVar5;
        }
        uVar1 = (iVar7 + 0x8e);
        uVar10 = (uVar1 >> 0x10);
        iVar8 = uVar1;
        if ((iVar8 + iStack10 * 0x4) != 0) {
            enable_win_1040_9234((iVar8 + iStack10 * 0x4), (puStack26 + 0x6));
        }
        iStack10 += 1;
    }
    move_win_1040_826c(param_2, -0x1, 0xffff);
    ShowWindow16(0x5, (iVar7 + 0x6));
    return;
}


pub unsafe fn send_msg_1038_ed8a(mut param_1: u16, mut param_2: u16, mut param_3: u32, mut param_4: u32)

{
    let mut uVar1: u16;
    let mut uVar2: u16;
    let mut uVar3: u16;
    let mut iVar4: i16;
    let mut uVar5: u16;
    let mut in_register_0000000a: u16;
    let mut paVar6: *mut Struct57;
    let mut puVar7: *mut u32;
    let mut uVar8: u32;
    let mut in_stack_0000fe8a: u16;
    let mut in_stack_0000ffae: u16;
    let mut in_stack_0000ffb4: u16;
    let mut in_stack_0000ffb8: u16;
    let mut hwnd: HWND16;
    let mut in_stack_0000ffe2: u16;

    paVar6 = CONCAT22(in_register_0000000a, param_1);
    hwnd = HWND16_1050_0396;
    if (param_4 != 0x1c8) {
        if (param_4 == 0x1c9) {
            puVar7 = mixed_1010_20ba(paVar6, _u16_1050_0ed0, CONCAT22(in_stack_0000ffe2, 0x2f), in_stack_0000fe8a,
                                     in_stack_0000ffae, in_stack_0000ffb4, in_stack_0000ffb8);
            uVar2 = (puVar7 >> 0x10);
            uVar5 = (puVar7 + 0x20);
            uVar1 = (puVar7 + 0x22);
            uVar8 = paVar6 & 0xffff0000 | uVar1;
            uVar3 = uVar1 | uVar5;
            if (uVar3 == 0) {
                return;
            }
            pass1_1028_e1ec(_PTR_LOOP_1050_65e2, CONCAT22(uVar1, uVar5));
            uVar5 = uVar8 | uVar3;
            paVar6 = (uVar8 & 0xffff0000 | uVar5);
            if (uVar5 == 0) {
                return;
            }
            iVar4 = pass1_1030_5b00(CONCAT22(uVar8, uVar3));
            puVar7 = mixed_1010_20ba(paVar6, _u16_1050_0ed0, CONCAT22(in_stack_0000ffe2, iVar4), in_stack_0000fe8a,
                                     in_stack_0000ffae, in_stack_0000ffb4, in_stack_0000ffb8);
            if (((puVar7 >> 0x10) | puVar7) == 0) {
                return;
            }
            uVar8 = pass1_1018_0ad4(puVar7);
            uVar5 = (uVar8 >> 0x10);
            if ((uVar5 | uVar8) == 0) {
                return;
            }
            param_4 = 0x72;
            hwnd = (uVar8 + 0x8);
        } else if (param_4 != 0x1ca) {
            post_win_msg_1040_7b3c(CONCAT22(param_3, param_2), (param_3 >> 0x10), param_4, param_4);
            return;
        }
    }
    SendMessage16(0x0, param_4, 0x111, hwnd);
    return;
}


pub unsafe fn pass1_1038_ee48(param_1: *mut StructD, param_2: u8) -> *mut StructD

{
    pass1_1038_ebd6(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn win_ui_op_1040_0000(
    ctx: &mut AppContext,
    pstruct57_param_1: *mut Struct57,
    pstructb_param_2: *mut StructB,
    mut param_3: u16,
) {
    let mut rect: *mut Struct57;
    let mut uVar1: u16;
    let mut uVar2: u16;
    let mut uVar3: u16;
    // let mut unaff_SI: u16;
    let mut uVar5: u16;
    let mut puVar8: *mut u32;
    let mut uVar9: u32;
    let mut in_stack_0000fe16: u16;
    let mut in_stack_0000fe1a: u16;
    let mut in_stack_0000fe6a: u16;
    let mut in_stack_0000ff40: u16;
    let mut in_stack_0000ff44: u16;
    let mut in_stack_0000ff48: u16;
    let mut in_stack_0000ff8e: u16;
    let mut in_stack_0000ff94: u16;
    let mut in_stack_0000ff98: u16;
    let mut local_24: u16;
    let mut uStack34: u16;
    let mut uStack32: u16;
    let mut uStack30: u16;
    let mut uStack28: u16;
    let mut local_1a: u16;
    let mut uStack24: u16;
    let mut uStack22: u16;
    let mut uStack18: u16;
    let mut uStack16: u16;
    let mut uStack14: u16;
    let mut iStack12: i16;
    let mut paStack10: *mut astruct_915;
    let mut paStack8: *mut Struct57;
    let mut uStack6: u16;
    let mut iStack4: i16;
    let mut iVar1: u16;
    let mut uVar4: u32;

    dialog_ui_fn_1040_78e2(pstructb_param_2);
    let iStack4 = 0x8u16;
    //   for (paStack10 = null_mut(); uVar5 = struct_b_param_1, uVar6 = (struct_b_param_1 >> 0x10),
    //   paStack10 < iStack4; paStack10 = paStack10 + 1)
    paStack10 = null_mut();
    // uVar5 = pstructb_param_2;
    // uVar6 = pstructb_param_2 >> 0x10;
    while paStack10 < iStack4 {
        iVar1 = paStack10 * 0xe;
        local_24 = (iVar1 + 0x5c60);
        uStack34 = (iVar1 + 0x5c62);
        uStack32 = 0x1;
        uStack30 = 0x1;
        rect = &local_24;
        MapDialogRect16(rect, &DAT_1050_1050);
        mem_op_1000_179c(0x42, pstruct57_param_1);
        uVar2 = pstruct57_param_1 | rect;
        uVar4 = pstruct57_param_1 & 0xffff0000 | uVar2;
        if uVar2 == 0 {
            rect = null_mut();
            uVar4 = pstruct57_param_1 & 0xffff0000;
        } else {
            pass1_1008_3bd6(
                uVar4,
                rect,
                pstruct57_param_1,
                0x1,
                CONCAT22(local_24, uStack34),
                0x104,
                0x1020103,
                CONCAT22((uVar5 + 0x6), (iVar1 + 0x5c64)),
                param_3,
                in_stack_0000fe16,
                in_stack_0000fe1a,
                in_stack_0000ff40,
                in_stack_0000ff44,
                in_stack_0000ff48,
            );
        }
        uStack6 = uVar4;
        paStack8 = rect;
        let uVar7 = win_ui_op_1040_0558(pstructb_param_2, paStack10);
        pstruct57_param_1 = (uVar4 & 0xffff0000 | uVar7 >> 0x10);
        paStack10 += 1;
    }
    move_win_1040_826c(pstructb_param_2, -0x1, 0xffff);
    puVar8 = mixed_1010_20ba(
        pstruct57_param_1,
        _u16_1050_0ed0,
        CONCAT22(unaff_SI, 0x48),
        in_stack_0000fe6a,
        in_stack_0000ff8e,
        in_stack_0000ff94,
        in_stack_0000ff98,
    );
    uStack16 = (puVar8 >> 0x10);
    uStack18 = puVar8;
    iStack12 = (uStack18 + 0xa);
    uStack14 = (uStack18 + 0xc);
    GetWindowRect16(CONCAT22(0x1050, &local_1a), (uVar5 + 0x6));
    uVar3 = iStack12 >> 0xf;
    uStack28 = uStack22 - local_1a;
    local_1a = (iStack12 / 0x2 - uStack28) - 0x3;
    if (local_1a < 0x0) {
        local_1a = 0;
    }
    SetWindowPos16(0x41, 0x0, 0x0, uStack24, local_1a, 0x0, (uVar5 + 0x6));
    uVar9 = pass1_1038_af40(uVar5, uVar3, _PTR_LOOP_1050_5b7c, (uVar5 + 0x6), 0x17);
    uVar3 = (uVar9 >> 0x10);
    uVar1 = uVar9;
    (uVar5 + 0x96) = uVar1;
    (uVar5 + 0x98) = uVar3;
    win_1008_5c7c(uVar1, uVar3, _u16_1050_02a0, 0x9e0001);
    (uVar5 + 0x8c) = uVar1;
    return;
}


pub unsafe fn win_ui_op_1040_0170(
    param_1: u8,
    mut param_2: u16,
    param_3: *mut StructD,
    mut param_4: u16,
    mut param_5: u16,
    mut param_6: i16,
) {
    let mut iVar1: i16;
    let mut hwnd_1: HWND16;
    let mut BVar2: bool;
    let mut paVar3: *mut astruct_915;
    let mut uVar4: u16;
    let mut in_register_0000000a: u16;
    let mut paVar5: *mut Struct57;
    let mut unaff_SI: u16;
    let mut uVar6: u32;
    let mut LVar7: LRESULT;
    let mut puVar8: *mut u32;
    let mut l_param: *mut c_char;
    let mut uVar9: u32;
    let mut in_stack_0000fd7c: u16;
    let mut in_stack_0000fd86: u16;
    let mut in_stack_0000fea0: u16;
    let mut in_stack_0000fea6: u16;
    let mut in_stack_0000feaa: u16;
    let mut in_stack_0000feb0: u16;
    let mut in_stack_0000feb4: u16;
    let mut pHVar10: *mut HCURSOR16;
    let mut uVar11: u16;
    let mut uVar12: u8;
    let mut uVar13: u8;
    let mut uVar14: u16;
    let mut w_param: WPARAM16;
    let mut msg: u16;
    let mut id: i16;
    let mut in_stack_0000fedc: u32;
    let mut uVar15: u32;
    let mut local_18: HCURSOR16;
    let mut local_16: u16;
    let mut paStack20: *mut astruct_598;
    let mut paStack16: *mut astruct_915;
    let mut uStack14: u16;
    let mut puStack12: *mut u32;
    let mut paStack8: *mut astruct_915;
    let mut WStack6: WPARAM16;
    let mut iStack4: i16;
    let mut iVar2: *mut astruct_890;
    let mut iVar3: *mut astruct_891;

    paVar5 = CONCAT22(in_register_0000000a, param_2);
    iStack4 = 0x8;
    WStack6 = 0;
    match param_6 {
        0x167 => {
            enable_win_1040_060e(param_3, 0x3);
            hwnd_1 = GetDlgItem16(0x16b, (param_3 + 0x6));
            EnableWindow16(0x1, hwnd_1);
            iStack4 = 0;
        }
        // break;
        0x168 => {
            enable_win_1040_060e(param_3, 0x3);
            hwnd_1 = GetDlgItem16(0x16b, (param_3 + 0x6));
            EnableWindow16(0x1, hwnd_1);
            iStack4 = 0x1;
        }
        // break;
        0x169 => {
            enable_win_1040_060e(param_3, 0x3);
            hwnd_1 = GetDlgItem16(0x16b, (param_3 + 0x6));
            EnableWindow16(0x1, hwnd_1);
            iStack4 = 0x2;
        }
        // break;
        0x16a => {
            enable_win_1040_060e(param_3, 0x3);
            hwnd_1 = GetDlgItem16(0x16b, (param_3 + 0x6));
            EnableWindow16(0x1, hwnd_1);
            iStack4 = 0x3;
        }
        // break;
        0x16b => {
            hwnd_1 = GetDlgItem16(0x16b, (param_3 + 0x6));
            BVar2 = EnableWindow16(0x0, hwnd_1);
            if ((param_3 + 0x92) != 0x3) {
                win_1008_5c5c(BVar2, paVar5, _u16_1050_02a0, 0x1de);
            }
            if ((param_3 + 0x92) != 0x8) {
                iVar2 = ((param_3 + 0x92) * 0xe);
                WStack6 = (iVar2 + 0x5c6c);
                pass1_1010_6604((param_3 + 0x8e), (iVar2 + 0x5c66));
                (param_3 + 0x92) = 0x8;
            }
            // for (paStack8 = null_mut(); paStack8 < 0x4; paStack8 = paStack8 + 1)

            {
                uVar6 = win_ui_op_1040_0558(param_3, paStack8);
                paVar5 = (paVar5 & 0xffff0000 | uVar6 >> 0x10);
            }
        }
        // TODO: goto LAB_1040_04da;
        0x16c => {
            hwnd_1 = GetDlgItem16(0x16d, (param_3 + 0x6));
            EnableWindow16(0x1, hwnd_1);
            iStack4 = 0x5;
            (param_3 + 0x94) = 0x5;
        }
        // TODO: goto LAB_1040_04da;
        0x16d => {
            hwnd_1 = GetDlgItem16(0x16d, (param_3 + 0x6));
            BVar2 = EnableWindow16(0x0, hwnd_1);
            win_1008_5c5c(BVar2, paVar5, _u16_1050_02a0, 0x1de);
            uVar11 = (paVar5 >> 0x10);
            if ((param_3 + 0x94) != 0x8) {
                iVar3 = ((param_3 + 0x94) * 0xe);
                WStack6 = (iVar3 + 0x5c6c);
                pass1_1010_6604((param_3 + 0x8e), (iVar3 + 0x5c66));
                (param_3 + 0x94) = 0x8;
            }
            LVar7 = win_ui_op_1040_0558(param_3, (&u32_1050_0004 + 1));
            paVar5 = CONCAT22(uVar11, (LVar7 >> 0x10));
            puStack12 = mixed_1010_20ba(
                paVar5,
                _u16_1050_0ed0,
                CONCAT22(unaff_SI, 0x39),
                in_stack_0000fd7c,
                in_stack_0000fea0,
                in_stack_0000fea6,
                in_stack_0000feaa,
            );
            paVar3 = (puStack12 + 0x20);
            uVar14 = SUB42(&DAT_1050_1050, 0x0);
            uVar12 = SUB21(&local_16, 0x0);
            uVar13 = (&local_16 >> 0x8);
            pHVar10 = &local_18;
            uVar11 = SUB42(&DAT_1050_1050, 0x0);
            uStack14 = (paVar3 >> 0xf) + 0x200;
            uVar6 = paVar5 & 0xffff0000 | uStack14;
            paStack16 = paVar3;
            paStack8 = paVar3;
            pass1_1030_8344(_u16_1050_5748, CONCAT22(uStack14, paVar3));
            paStack20 = CONCAT22(uVar6, paVar3);
            pass1_1030_2f1a(
                CONCAT22(uVar6, paVar3),
                CONCAT22(uVar11, pHVar10),
                CONCAT22(uVar14, CONCAT11(uVar13, uVar12)),
            );
            paVar5 = (uVar6 & 0xffff0000 | ((local_18 - local_16) >> 0xf));
            local_16 += (local_18 - local_16) / 0x2;
            uVar4 = pass1_1030_2fac(paStack20);
            set_window_text_1018_6086((param_3 + 0x96), uVar4, local_16);
        }
        // TODO: goto LAB_1040_04da;
        0x16e => {
            puVar8 = mixed_1010_20ba(
                paVar5,
                _u16_1050_0ed0,
                CONCAT22(unaff_SI, 0x39),
                in_stack_0000fd7c,
                in_stack_0000fea0,
                in_stack_0000fea6,
                in_stack_0000feaa,
            );
            paVar5 = (paVar5 & 0xffff0000 | puVar8 >> 0x10);
            iVar1 = (puVar8 + 0x20);
            local_18 = LoadCursor16(0x7f02, 0x0);
            local_16 = SetCursor16(local_18);
            pass1_1030_532e(CONCAT22(0x1050, &stack0xfed6), iVar1 + 0x2000000);
            fn_ptr_1030_835a(_u16_1050_5748, CONCAT22(0x1050, &stack0xfed6));
            pass1_1030_838e(_u16_1050_5748);
            pass1_1030_8334();
            SetCursor16(local_16);
            PostMessage16(0x0, 0x7e, 0x111, HWND16_1050_0396);
            DestroyWindow16((param_3 + 0x6));
        }
        // TODO: goto LAB_1040_04da;
        _ => {
            post_win_msg_1040_7b3c(param_3, param_4, param_5, param_6);
            return;
        }
    };
    (param_3 + 0x92) = iStack4; //
    // LAB_1040_04da:
    uVar11 = (in_stack_0000fedc >> 0x10);
    if (iStack4 != 0x8) {
        uVar15 = in_stack_0000fedc & 0xffff0000 | (param_3 + 0x6);
        id = (iStack4 * 0xe + 0x5c68);
        w_param = 0;
        msg = 0xc;
        l_param = load_string_1010_847e(_u16_1050_14cc, (iStack4 * 0xe + 0x5c6a));
        uVar6 = paVar5 & 0xffff0000;
        uVar9 = SendDlgItemMessage16(l_param, w_param, msg, id, uVar15);
        uVar11 = (uVar15 >> 0x10);
        paVar5 = (uVar6 & 0xffff0000 | uVar9 >> 0x10);
    }
    if ((WStack6 != 0) && (
        puVar8 = mixed_1010_20ba(
            paVar5,
            _u16_1050_0ed0,
            CONCAT22(uVar11, 0x2),
            in_stack_0000fd86,
            in_stack_0000feaa,
            in_stack_0000feb0,
            in_stack_0000feb4,
        ),
        (puVar8 + 0x20) != 0,
    )) {
        PostMessage16(0x0, WStack6, 0x111, HWND16_1050_0396);
    }
    return;
}


pub unsafe fn pass1_1040_0656(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    destroy_win_1038_ef3a(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}


pub unsafe fn show_win_1040_0766(struct_b_param_1: *mut StructB, mut param_2: u16) {
    let mut in_register_0000000a: u16;
    let mut paVar1: *mut Struct57;
    let mut puVar2: *mut u32;
    let mut in_stack_0000fe7e: u16;
    let mut in_stack_0000fe86: u16;
    let mut in_stack_0000ffa2: u16;
    let mut in_stack_0000ffa8: u16;
    let mut in_stack_0000ffaa: u16;
    let mut in_stack_0000ffac: u16;
    let mut in_stack_0000ffb0: u16;
    let mut in_stack_0000ffb4: u16;
    let mut piVar3: *mut i16;
    let mut uVar4: u16;
    let mut piVar5: *mut i16;
    let mut uVar6: u16;
    let mut in_stack_0000ffde: u16;
    let mut local_a: i16;
    let mut local_8: i16;
    let mut uStack6: u16;
    let mut uStack4: u16;

    paVar1 = CONCAT22(in_register_0000000a, param_2);
    dialog_ui_fn_1040_78e2(struct_b_param_1);
    puVar2 = mixed_1010_20ba(
        paVar1,
        _u16_1050_0ed0,
        CONCAT22(in_stack_0000ffde, 0x2),
        in_stack_0000fe86,
        in_stack_0000ffaa,
        in_stack_0000ffb0,
        in_stack_0000ffb4,
    );
    paVar1 = (paVar1 & 0xffff0000 | puVar2 >> 0x10);
    uStack6 = SUB42(puVar2, 0x0);
    uStack4 = (puVar2 >> 0x10);
    pass1_1010_6118(puVar2);
    piVar5 = &local_8;
    uVar6 = SUB42(&DAT_1050_1050, 0x0);
    piVar3 = &local_a;
    uVar4 = SUB42(&DAT_1050_1050, 0x0);
    puVar2 = mixed_1010_20ba(
        paVar1,
        _u16_1050_0ed0,
        CONCAT22(piVar3, 0x48),
        in_stack_0000fe7e,
        in_stack_0000ffa2,
        in_stack_0000ffa8,
        in_stack_0000ffac,
    );
    pass1_1008_3e94(
        (puVar2 & 0xffff0000 | (puVar2 + 0xe)),
        CONCAT22(uVar4, piVar3),
        CONCAT22(uVar6, piVar5),
    );
    move_win_1040_826c(struct_b_param_1, local_a + 0x8c, local_8 + 0xb9);
    ShowWindow16(0x5, (struct_b_param_1 + 0x6));
    return;
}


pub unsafe fn win_ui_op_1040_07dc(
    mut param_1: u16,
    pstruct_c_param_2: *mut StructC,
    mut param_3: u16,
    mut param_4: u16,
    mut param_5: u16,
) {
    let mut ppcVar1: *mut *mut code;
    let mut IVar2: i16;
    let mut puVar3: *mut u8;
    let mut puVar4: *mut u8;
    let mut puVar5: *mut u8;
    let mut in_register_0000000a: u16;
    let mut paVar6: *mut Struct57;
    let mut puVar7: *mut u32;
    let mut puVar8: *mut u32;
    let mut in_stack_0000f69a: u16;
    let mut in_stack_0000f7be: u16;
    let mut in_stack_0000f7c4: u16;
    let mut in_stack_0000f7c8: u16;
    let mut BVar9: bool;
    let mut in_stack_0000f7f2: u16;
    let mut uStack2060: u32;
    let mut local_806: [u8; 0x400] = [0; 0x400];
    let mut local_406: [u32; 0x100] = [0; 0x100];
    let mut uStack6: u32;

    paVar6 = CONCAT22(in_register_0000000a, param_1);
    uStack6 = 0;
    if (param_5 == 0x73) {
        enable_window_1040_0acc(pstruct_c_param_2, 0x0);
        puVar4 = paVar6;
        puVar3 = pass1_1008_5fd8(puVar4);
        uStack2060 = CONCAT22(puVar4, puVar3);
        puVar5 = puVar4;
        load_string_1010_84e0(
            _u16_1050_14cc,
            (_u16_1050_14cc >> 0x10),
            0x3ff,
            local_806,
            &DAT_1050_1050,
        );
        IVar2 = MessageBox16(
            0x34,
            CONCAT13(0x10, CONCAT12(0x50, local_806)),
            CONCAT22(puVar4, puVar3),
            HWND16_1050_0396,
        );
        local_406[0] = uStack2060;
        fn_ptr_1000_17ce(CONCAT22(puVar4, puVar3));
        if (IVar2 == 0x6) {
            PostMessage16(0x0, 0xcb, 0x111, HWND16_1050_0396);
            BVar9 = post_win_msg_1040_7b3c(pstruct_c_param_2, param_3, param_4, 1);
            uStack6 = CONCAT22(puVar5, BVar9);
        }
    } else {
        if (param_5 < 0x74) {
            if (param_5 == 0x6e) {
                (_PTR_LOOP_1050_5b7c + 0xae) = 0x99;
                puVar8 = pass1_1038_af40(
                    pstruct_c_param_2,
                    param_1,
                    _PTR_LOOP_1050_5b7c,
                    (pstruct_c_param_2 + 0x6),
                    0x2,
                );
                ppcVar1 = (*puVar8 + 0x3c);
                (**ppcVar1)(&u16_1050_1038, puVar8, (puVar8 >> 0x10));
                SetFocus16((pstruct_c_param_2 + 0x6));
                return;
            }
            if (0x6e < param_5) {
                //
                // LAB_1040_09f9:
                post_win_msg_1040_7b3c(pstruct_c_param_2, param_3, param_4, param_5);
                return;
            }
            if (param_5 == '\x02') {
                //
                // LAB_1040_09b4:
                post_win_msg_1040_7b3c(pstruct_c_param_2, 0x0, 0x0, 0x2);
                PostMessage16(0x0, 0xee, 0x111, HWND16_1050_0396);
                return;
            }
            //      if (param_5 != 'd') goto LAB_1040_09f9;
            PostMessage16(0x0, 0x64, 0x111, HWND16_1050_0396);
            BVar9 = 0;
            // TODO: goto LAB_1040_0821;
        }
        if (param_5 != 0x74) {
            //      if (param_5 == 0xee) goto LAB_1040_09b4;
            if (param_5 == 0x13d) {
                enable_window_1040_0acc(pstruct_c_param_2, 1);
                return;
            }
            // TODO: goto LAB_1040_09f9;
        }
        enable_window_1040_0acc(pstruct_c_param_2, 0x0);
        load_string_1010_84e0(
            _u16_1050_14cc,
            (_u16_1050_14cc >> 0x10),
            0x3ff,
            local_406,
            &DAT_1050_1050,
        );
        load_string_1010_84e0(
            _u16_1050_14cc,
            (_u16_1050_14cc >> 0x10),
            0x3ff,
            local_806,
            &DAT_1050_1050,
        );
        IVar2 = MessageBox16(
            0x34,
            CONCAT13(0x10, CONCAT12(0x50, local_406)),
            CONCAT22(0x1050, local_806),
            HWND16_1050_0396,
        );
        if (IVar2 == 0x6) {
            PostMessage16(0x0, 0x7a, 0x111, HWND16_1050_0396);
            BVar9 = post_win_msg_1040_7b3c(pstruct_c_param_2, param_3, param_4, 1);
            uStack6 = CONCAT22(paVar6, BVar9);
            puVar7 = mixed_1010_20ba(
                paVar6,
                _u16_1050_0ed0,
                CONCAT22(in_stack_0000f7f2, 0x2),
                in_stack_0000f69a,
                in_stack_0000f7be,
                in_stack_0000f7c4,
                in_stack_0000f7c8,
            );
            pass1_1010_60fa(puVar7);
        }
    }
    BVar9 = 0x1; //
    // LAB_1040_0821:
    enable_window_1040_0acc(pstruct_c_param_2, BVar9);
    return;
}


pub unsafe fn pass1_1040_0a1a(mut param_1: u32) {
    let mut uVar1: u16;
    let mut puVar2: *mut u32;
    let mut ppcVar3: *mut *mut code;
    let mut uVar4: u32;
    let mut paVar5: *mut astruct_394;
    let mut uVar6: u16;
    let mut puVar7: *mut u8;
    let mut in_EDX: u32;
    let mut paVar8: *mut Struct57;
    let mut iVar9: i16;
    let mut iVar10: i16;
    let mut uVar11: u16;
    let mut uVar12: u16;
    let mut uStack10: u32;
    let mut uStack6: u16;

    uVar11 = (param_1 >> 0x10);
    iVar9 = param_1;
    uVar4 = (iVar9 + 0x8e);
    uVar12 = (uVar4 >> 0x10);
    iVar10 = uVar4;
    puVar2 = (iVar10 + 0xa);
    uVar1 = (iVar10 + 0xc);
    paVar8 = (in_EDX & 0xffff0000 | uVar1);
    uStack6 = puVar2;
    paVar5 = (uVar1 | uStack6);
    if (paVar5.is_null()) {
        return;
    }
    ppcVar3 = (*puVar2 + 0x14);
    (**ppcVar3)();
    uStack10 = CONCAT22(paVar8, paVar5);
    if ((iVar9 + 0x70) != 0) {
        paVar5 = (iVar9 + 0x70);
        uVar1 = (iVar9 + 0x72);
        uVar6 = uVar1 | paVar5;
        paVar8 = (paVar8 & 0xffff0000 | uVar6);
        if (uVar6 != 0) {
            ppcVar3 = paVar5;
            (**ppcVar3)();
        }
    }
    mem_op_1000_179c(0x14, paVar8);
    puVar7 = (paVar8 | paVar5);
    if (puVar7.is_null()) {
        paVar5 = null_mut();
        puVar7 = null_mut();
    } else {
        struct_1008_4c58(paVar5);
    }
    (iVar9 + 0x70) = paVar5;
    (iVar9 + 0x72) = puVar7;
    pass1_1008_4d84(puVar7, (iVar9 + 0x70), uStack10);
    return;
}


pub unsafe fn pass1_1040_0b6a(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    pass1_1040_073a(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn show_win_1040_0c7c(param_1: *mut StructB) {
    let mut uVar1: u32;
    let mut uVar2: u16;
    let mut local_6: u32;

    dialog_ui_fn_1040_78e2(param_1);
    uVar2 = (param_1 >> 0x10);
    uVar1 = (param_1 + 0x8e);
    pass1_1010_4f30(
        uVar1,
        (uVar1 >> 0x10),
        CONCAT22(0x1050, &local_6),
        CONCAT22(0x1050, &local_6 + 0x2),
    );
    move_win_1040_826c(param_1, local_6, (local_6 >> 0x10));
    ShowWindow16(0x5, (param_1 + 0x6));
    return;
}
