use crate::block_1000::block_1000_1000::fn_ptr_1000_17ce;
use crate::block_1008::block_1008_4000::pass1_1008_4d72;
use crate::block_1010::block_1010_2000::mixed_1010_20ba;
use crate::block_1030::block_1030_7000::struct_op_1030_73a8;
use crate::block_1040::block_1040_0000::pass1_1040_0c54;
use crate::block_1040::block_1040_1000::{check_dialog_btn_1040_1b8a, pass1_1040_1290, pass1_1040_1876, pass1_1040_1d24, send_msg_1040_1696};
use crate::block_1040::block_1040_2000::pass1_1040_205e;
use crate::block_1040::block_1040_7000::dialog_ui_fn_1040_78e2;
use crate::structs::struct_d::StructD;
use crate::utils::{CONCAT11, CONCAT22};
use crate::winbase::{GetDlgItem16, GetStockObject16, GetWindowRect16, SetWindowPos16};
use crate::windef::{HGDIOBJ16, HWND16};

pub unsafe fn set_text_bk_color_1040_0cc0(
    param_1: u32,
    mut param_2: u16,
    mut param_3: u16,
    hwnd_param_4: HWND16,
) -> u32 {
    let mut iVar1: *mut astruct_783;
    let mut uVar3: u16;
    let mut uVar1: u32;
    let mut hobject: HGDIOBJ16;
    let mut fn_ptr_1: *mut *mut code;

    hobject = GetStockObject16(BLACK_BRUSH);
    if (_u16_1050_5cd0 == 0) {
        fn_ptr_1 = (*param_1 + 0x68);
        uVar1 = (**fn_ptr_1)(s_tile2_bmp_1050_1538, param_1, (param_1 + 0x6e));
        uVar1 = pass1_1008_4d72(uVar1);
        uVar3 = (uVar1 >> 0x10);
        iVar1 = uVar1;
        _u16_1050_5cd0 = CONCAT22(
            CONCAT11(0x2, iVar1.field_0x94),
            CONCAT11(iVar1.field_0x95, iVar1.field_0x96),
        );
    }
    if (0x3 < param_3) {
        if (param_3 == 0x4) {
            hobject = GetStockObject16(HOLLOW_BRUSH);
        } else if ((param_3 == 0x4) || (0x1 < param_3 - 0x5)) {
            return 0x0;
        }
    }
    SetTextColor16(_u16_1050_5cd0, hwnd_param_4);
    SetBkColor16(0x1000000, hwnd_param_4);
    return CONCAT22(0x1050, hobject);
}


pub unsafe fn post_win_msg_1040_0d5e(mut param_1: u16, mut param_2: u16, mut param_3: i16) {
    if (param_3 != 0) {
        PostMessage16(0x0, 0x1, 0x111, (param_1 + 0x8));
    }
    return;
}

pub unsafe fn pass1_1040_0d80() -> u16 {
    return 0x1;
}

pub unsafe fn FUN_1040_0d86() {
    return;
}

pub unsafe fn pass1_1040_0d8a(param_1: *mut StructD, param_2: u8) -> *mut StructD {
    pass1_1040_0c54(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}


pub unsafe fn FUN_1040_0f0c(mut param_1: u16, param_2: *mut StructB) {
    let mut uVar1: u32;
    let mut in_AX: u16;
    let mut HVar2: HWND16;
    let mut in_EDX: u32;
    let mut uVar3: u16;
    let mut iVar4: i16;
    let mut uVar5: u16;
    let mut uVar6: u32;
    let mut puVar7: *mut u32;
    let mut in_stack_0000fe6e: u16;
    let mut in_stack_0000ff92: u16;
    let mut in_stack_0000ff98: u16;
    let mut in_stack_0000ff9c: u16;
    let mut id: i16;
    let mut in_stack_0000ffc6: u16;
    let mut local_2e: [u8; 0x2] = [0; 0x2];
    let mut iStack44: i16;
    let mut local_26: i16;
    let mut iStack36: i16;
    let mut iStack34: i16;
    let mut iStack32: i16;
    let mut iStack30: i16;
    let mut uStack28: u16;
    let mut iStack26: i16;
    let mut iStack24: i16;
    let mut iStack22: i16;
    let mut iStack20: i16;
    let mut iStack18: i16;
    let mut iStack16: i16;
    let mut local_e: i16;
    let mut iStack12: i16;
    let mut iStack10: i16;
    let mut iStack8: i16;
    let mut POlocal_6: INT16;

    dialog_ui_fn_1040_78e2(param_2);
    uVar5 = (param_2 >> 0x10);
    iVar4 = param_2;
    if ((iVar4 + 0x98) == 0) {
        GetWindowRect16(CONCAT22(0x1050, &local_26), (iVar4 + 0x6));
        uVar3 = (in_EDX >> 0x10);
        HVar2 = GetDlgItem16(0x1830, (iVar4 + 0x6));
        GetWindowRect16(CONCAT22(0x1050, local_2e), HVar2);
        iStack34 -= local_26;
        iStack32 = (iStack44 - iStack36) - 0x2;
        SetWindowPos16(0x6, iStack32, iStack34, 0x0, 0x0, 0x0, (iVar4 + 0x6));
        CheckDlgButton16(0x1, 0x1c1, (iVar4 + 0x6));
        uVar1 = (iVar4 + 0x8e);
        (uVar1 + 0xa) = 0x2;
        HVar2 = GetDlgItem16(0x1830, (iVar4 + 0x6));
        in_stack_0000ffc6 = 0;
        EnableWindow16(0x0, HVar2);
    } else {
        uVar1 = (iVar4 + 0x92);
        uVar6 = struct_op_1030_73a8((uVar1 + 0x6), in_AX, in_EDX);
        uVar3 = (in_EDX >> 0x10);
        if ((uVar6 + 0x20) == 0x2) {
            HVar2 = (iVar4 + 0x6);
            id = 0x1c1;
        } else {
            HVar2 = (iVar4 + 0x6);
            id = 0x1c2;
        }
        CheckDlgButton16(0x1, id, HVar2);
    }
    GetCursorPos16(&local_6);
    GetWindowRect16(CONCAT22(0x1050, &local_e), (iVar4 + 0x6));
    iStack20 = iStack10 - local_e;
    iStack16 = -(iStack20 / 0x2 - local_6.x);
    iStack22 = iStack8 - iStack12;
    iStack18 = -(iStack22 / 0x2 - local_6.y);
    puVar7 = mixed_1010_20ba(
        CONCAT22(uVar3, iStack22 >> 0xf),
        _u16_1050_0ed0,
        CONCAT22(in_stack_0000ffc6, 0x48),
        in_stack_0000fe6e,
        in_stack_0000ff92,
        in_stack_0000ff98,
        in_stack_0000ff9c,
    );
    uStack28 = (puVar7 >> 0x10);
    iStack30 = puVar7;
    iStack24 = (iStack30 + 0xa);
    iStack26 = (iStack30 + 0xc);
    if (iStack24 < iStack20 + iStack16) {
        iStack16 = iStack24 - iStack20;
    }
    if (iStack26 < iStack22 + iStack18) {
        iStack18 = iStack26 - iStack22;
    }
    SetWindowPos16(0x45, 0x0, 0x0, iStack18, iStack16, 0x0, (iVar4 + 0x6));
    return;
}


pub unsafe fn pass1_1040_109c(param_1: *mut u8, mut param_2: i16, mut param_3: u16, mut param_4: u16, mut param_5: u32)

{
    let mut uVar1: u32;
    let mut bVar2: bool;
    let mut iVar3: i16;
    let mut in_register_0000000a: u16;
    let mut paVar4: *mut Struct57;
    let mut uVar5: u32;
    let mut paVar6: *mut Struct57;
    let mut in_stack_0000fe9a: u16;
    let mut in_stack_0000ffbe: u16;
    let mut in_stack_0000ffc4: u16;
    let mut in_stack_0000ffc8: u16;
    let mut in_stack_0000fff2: u16;

    paVar4 = CONCAT22(in_register_0000000a, param_1);
    bVar2 = false;
    if (param_5 == 0x1c1) {
        (param_2 + 0x96) = 0x2;
        bVar2 = true;
    } else if (param_5 == 0x1c2) {
        (param_2 + 0x96) = 0x1;
        bVar2 = true;
    } else {
        if (param_5 != 0x1830) {
            post_win_msg_1040_7b3c(CONCAT22(param_3, param_2), param_4, param_5, param_5);
            return;
        }
        paVar6 = mixed_1010_20ba(paVar4, _u16_1050_0ed0, CONCAT22(in_stack_0000fff2, 0x32), in_stack_0000fe9a,
                                 in_stack_0000ffbe, in_stack_0000ffc4, in_stack_0000ffc8);
        uVar5 = paVar4 & 0xffff0000 | paVar6 >> 0x10;
        iVar3 = paVar6;
        uVar1 = (param_2 + 0x92);
        ui_op_1010_79aa(paVar6, 0xfb6, (uVar1 + 0x6));
        if (iVar3 == 0) {
            uVar1 = (param_2 + 0x92);
            unk_win_op_1010_7300(uVar5, paVar6, 0x0, 0xc, (uVar1 + 0x6));
        }
    }
    if (bVar2) {
        uVar1 = (param_2 + 0x8e);
        (uVar1 + 0xa) = (param_2 + 0x96);
    }
    return;
}


pub unsafe fn pass1_1040_1152(param_1: *mut u8, mut param_2: i16, mut param_3: u16)

{
    let mut uVar1: u16;
    let mut uVar2: u32;
    let mut in_register_0000000a: u16;
    let mut paVar3: *mut Struct57;
    let mut iVar4: i16;
    let mut uVar5: u16;
    let mut puVar6: *mut u32;
    let mut in_stack_0000fe9c: u16;
    let mut in_stack_0000ffc0: u16;
    let mut in_stack_0000ffc6: u16;
    let mut in_stack_0000ffca: u16;
    let mut in_stack_0000fff4: u16;

    paVar3 = CONCAT22(in_register_0000000a, param_1);
    if ((param_2 + 0x92) != 0) {
        uVar2 = (param_2 + 0x8e);
        uVar1 = (uVar2 + 0xa);
        puVar6 = mixed_1010_20ba(paVar3, _u16_1050_0ed0, CONCAT22(in_stack_0000fff4, 0x3), in_stack_0000fe9c,
                                 in_stack_0000ffc0, in_stack_0000ffc6, in_stack_0000ffca);
        uVar2 = (param_2 + 0x92);
        uVar5 = (uVar2 >> 0x10);
        iVar4 = uVar2;
        pass1_1010_ae92(puVar6, uVar1, (iVar4 + 0xa), (iVar4 + 0x6),
                        paVar3 & 0xffff0000 | puVar6 >> 0x10);
    }
    destroy_win_1040_7b98(CONCAT22(param_3, param_2));
    PTR_LOOP_1050_5b80 = null_mut();
    return;
}


pub unsafe fn pass1_1040_11ac(param_1: *mut StructD, param_2: u8) -> *mut StructD

{
    pass1_1040_0e86(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}


pub unsafe fn win_ui_op_1040_12bc(param_1: u8, struct_b_param_1: *mut StructB)

{
    let mut uVar1: u32;
    let mut HVar2: HWND16;
    let mut struct_b_3: *mut StructB;
    let mut uVar3: u16;
    let mut lparam: *mut c_char;
    let mut local_54: [u8; 0x52] = [0; 0x52];

    dialog_ui_fn_1040_78e2(struct_b_param_1);
    uVar3 = (struct_b_param_1 >> 0x10);
    struct_b_3 = struct_b_param_1;
    uVar1 = &struct_b_3[0x7].field1_0x2;
    sys_1000_3f9c(CONCAT22(0x1050, local_54), s__u_1050_5cd4, (uVar1 + 0xa));
    HVar2 = GetDlgItem16(0xfd2, struct_b_3.lpvoid_field_0x8);
    SendMessage16(CONCAT22(0x1050, local_54), 0x0, 0xc, HVar2);
    SetFocus16(HVar2);
    SendMessage16(-0x10000, 0x0, 0x401, HVar2);
    move_win_1040_826c(struct_b_param_1, -0x1, 0xffff);
    lparam = load_string_1010_847e(_u16_1050_14cc, 0x531);
    HVar2 = GetDlgItem16(s_vrpal_bmp_1050_183a + 0x5, struct_b_3.lpvoid_field_0x8);
    send_msg_1040_1696(struct_b_param_1, HVar2);
    SendMessage16(lparam, 0xffff, 0x40d, HVar2);
    HVar2 = GetDlgItem16(s_vrpal_bmp_1050_183a + 0x4, struct_b_3.lpvoid_field_0x8);
    send_msg_1040_1696(struct_b_param_1, HVar2);
    SendMessage16(lparam, 0xffff, 0x40d, HVar2);
    ShowWindow16(0x5, struct_b_3.lpvoid_field_0x8);
    return;
}


pub unsafe fn win_msg_op_1040_13b2(param_1: *mut astruct_892, mut param_2: i16)

{
    let mut HVar1: HWND16;
    let mut uVar4: u16;
    let mut iVar4: i16;
    let mut puVar5: *mut u8;
    let mut iVar5: i16;
    let mut puVar4: *mut c_char;
    let mut puVar6: *mut u8;
    let mut puVar7: *mut u8;
    let mut in_EDX: u32;
    let mut uVar5: u16;
    let mut paVar2: *mut Struct57;
    let mut struct_7: *mut astruct_892;
    let mut iVar6: i16;
    let mut struct_5_lo: u16;
    let mut uVar6: u16;
    let mut lresult_4: LRESULT;
    let mut pcVar6: *mut c_char;
    let mut puStack266: *mut u32;
    let mut local_100: [u8; 0x52] = [0; 0x52];
    let mut local_aa: [u8; 0x52] = [0; 0x52];
    let mut uStack88: u16;
    let mut handle_86: HWND16;
    let mut local_54: [u8; 0x52] = [0; 0x52];
    let mut uVar1: u32;
    let mut uVar2: u32;
    let mut uVar3: u32;
    let mut fn_ptr_1: *mut *mut code;

    uVar5 = (in_EDX >> 0x10);
    struct_7 = param_1;
    struct_5_lo = (param_1 >> 0x10);
    if (param_2 != 0) {
        handle_86 = GetDlgItem16(0xfd2, struct_7.hwnd_0x6);
        SendMessage16(CONCAT22(0x1050, local_54), 0x51, 0xd, handle_86);
        uStack88 = pass1_1000_3e2c(CONCAT22(0x1050, local_54));
        HVar1 = GetDlgItem16(s_vrpal_bmp_1050_183a + 0x4, struct_7.hwnd_0x6);
        lresult_4 = SendMessage16(0x0, 0x0, 0x407, HVar1);
        if (lresult_4 != 0xffff) {
            SendMessage16(CONCAT22(0x1050, local_aa), lresult_4, 0x408, HVar1);
        }
        HVar1 = GetDlgItem16(s_vrpal_bmp_1050_183a + 0x5, struct_7.hwnd_0x6);
        lresult_4 = SendMessage16(0x0, 0x0, 0x407, HVar1);
        if (lresult_4 != 0xffff) {
            SendMessage16(CONCAT22(0x1050, local_100), lresult_4, 0x408, HVar1);
        }
        pcVar6 = load_string_1010_847e(_u16_1050_14cc, 0x531);
        paVar2 = CONCAT22(uVar5, local_aa);
        uVar4 = pass1_1000_3d7a(CONCAT22(0x1050, local_aa), CONCAT22(0x1050, local_100));
        if (uVar4 != 0) {
            uVar4 = pass1_1000_3d7a(CONCAT22(0x1050, local_aa), pcVar6);
            if (uVar4 != 0) {
                uVar4 = pass1_1000_3d7a(CONCAT22(0x1050, local_100), pcVar6);
                if (uVar4 != 0) {
                    pass1_1010_531c(local_aa, paVar2, struct_7.field141_0x8e, CONCAT22(0x1050, local_aa));
                    puVar5 = local_100;
                    pass1_1010_52fc(puVar5, paVar2, struct_7.field141_0x8e, CONCAT22(0x1050, puVar5));
                    pass1_1010_5120(puVar5, paVar2, struct_7.field141_0x8e, uStack88);
                    if (puVar5.is_null()) {
                        mem_op_1000_179c(0xb4, paVar2);
                        puVar7 = (paVar2 | puVar5);
                        uVar3 = paVar2 & 0xffff0000 | ZEXT24(puVar7);
                        if (puVar7.is_null()) {
                            iVar5 = 0;
                            uVar5 = 0;
                        } else {
                            iVar5 = string_1040_8520(uVar3, CONCAT22(paVar2, puVar5), HWND16_1050_0396, 0x20030,
                                                     0x7d2057b);
                            uVar5 = uVar3;
                        }
                        fn_ptr_1 = (CONCAT13((uVar5 >> 0x8), CONCAT12(uVar5, iVar5)) + 0x74);
                        (**fn_ptr_1)();
                        return;
                    }
                    uVar1 = struct_7.field141_0x8e;
                    uVar2 = struct_7.field141_0x8e;
                    uVar6 = (uVar2 >> 0x10);
                    iVar6 = uVar2;
                    uVar3 = struct_7.field141_0x8e;
                    pass1_1028_8d9e(CONCAT22(0x1050, &stack0xfdd2), (uVar3 + 0xa),
                                    (uVar1 + 0x12),
                                    (iVar6 + 0x16) & 0xffff | (iVar6 + 0x18) << 0x10);
                    fn_ptr_1030_835a(_u16_1050_5748, CONCAT22(0x1050, &stack0xfdd2));
                    pass1_1028_8dec(CONCAT22(0x1050, &stack0xfdd2));
                    // TODO: goto LAB_1040_1619;
                }
            }
        }
        mem_op_1000_179c(0xb4, paVar2);
        puVar6 = (paVar2 | uVar4);
        uVar3 = paVar2 & 0xffff0000 | ZEXT24(puVar6);
        if (puVar6.is_null()) {
            iVar4 = 0;
            uVar5 = 0;
        } else {
            iVar4 = string_1040_8520(uVar3, CONCAT22(paVar2, uVar4), HWND16_1050_0396, 0x20030, 0x755057b);
            uVar5 = uVar3;
        }
        puStack266 = CONCAT22(uVar5, iVar4);
        fn_ptr_1 = (*puStack266 + 0x74);
        (**fn_ptr_1)();
    }//
// LAB_1040_1619:
    DestroyWindow16(struct_7.hwnd_0x6);
    return;
}


pub unsafe fn set_win_pos_1040_162a(mut param_1: u16, mut param_2: u16, mut param_3: u32, mut param_4: u32) -> u32

{
    let mut uVar1: u16;
    let mut BVar2: bool;
    let mut iStack6: i16;

    if ((param_4 != s_vrpal_bmp_1050_183a + 0x5) && (param_4 != s_vrpal_bmp_1050_183a + 0x4)) {
        BVar2 = post_win_msg_1040_7b3c(CONCAT22(param_3, param_2), param_3, param_4, param_4);
        return CONCAT22(param_1, BVar2);
    }
    if (param_4 == 0x7) {
        GetWindowRect16(CONCAT22(0x1050, &stack0xfff6), param_3);
        SetWindowPos16(0x2, 0x50, iStack6 - param_3, 0x0, 0x0, 0x0, param_3);
    } else if ((param_4 != 0x9) && (param_4 != 0xa)) {
        uVar1 = 0;
// TODO: goto LAB_1040_164d;
    }
    uVar1 = 0x1;//
// LAB_1040_164d:
    return uVar1;
}

pub unsafe fn FUN_1040_1786()

{
    return;
}


pub unsafe fn pass1_1040_178a(param_1: *mut StructD, param_2: u8) -> *mut StructD

{
    pass1_1040_1290(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}


pub unsafe fn unk_win_ui_op_1040_19ea(param_1: *mut astruct_32, mut param_2: i16, param_3: *mut u8)

{
    let mut pSVar1: *mut StructD;
    let mut UVar2: u16;
    let mut pstruct32_1: *mut astruct_32;
    let mut pstruct_32_hi: *mut astruct_32;

    pstruct32_1 = param_1;
    pstruct_32_hi = (param_1 >> 0x10);
    if (param_2 != 0) {
        UVar2 = IsDlgButtonChecked(0xfdb, pstruct32_1.field6_0x6);
        pass1_1010_5d9c(param_3, pstruct32_1.pstructd_0x8e, UVar2);
        UVar2 = IsDlgButtonChecked(0xfdc, pstruct32_1.field6_0x6);
        pSVar1 = pstruct32_1.pstructd_0x8e;
        (pSVar1 + 0x20) = UVar2;
        UVar2 = IsDlgButtonChecked(0xfdd, pstruct32_1.field6_0x6);
        pSVar1 = pstruct32_1.pstructd_0x8e;
        (pSVar1 + 0x74) = UVar2;
        UVar2 = IsDlgButtonChecked(0xfde, pstruct32_1.field6_0x6);
        pSVar1 = pstruct32_1.pstructd_0x8e;
        (pSVar1 + 0x72) = UVar2;
        if (pstruct32_1.field142_0x92 != 0) {
            pSVar1 = pstruct32_1.pstructd_0x8e;
            pass1_1000_4906((pSVar1 & 0xffff0000 | (pSVar1 + 0x22)), NULL, 0x40);
        }
        if (pstruct32_1.field143_0x94 != 0) {
            pass1_1010_60a0(pstruct32_1.pstructd_0x8e);
        }
    }
    DestroyWindow16(pstruct32_1.field6_0x6);
    return;
}


pub unsafe fn pass1_1040_1ab0(mut param_1: u16, mut param_2: i16, mut param_3: u16, mut param_4: u16, mut param_5: u32) -> u32

{
    let mut BStack6: bool;
    let mut uStack4: u16;

    BStack6 = 0;
    uStack4 = 0;
    if (param_5 == 0x1831) {
        (param_2 + 0x92) = 0x1;
        (param_2 + 0x94) = 0x1;
        check_dialog_btn_1040_1b8a(CONCAT22(param_3, param_2));
    } else {
        BStack6 = post_win_msg_1040_7b3c(CONCAT22(param_3, param_2), param_4, param_5, param_5);
        uStack4 = param_1;
    }
    return CONCAT22(uStack4, BStack6);
}

pub unsafe fn FUN_1040_1c1e()

{
    return;
}

pub unsafe fn pass1_1040_1c22(param_1: *mut StructD, param_2: u8) -> *mut StructD

{
    pass1_1040_1876(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn show_win_1040_1d50(param_1: *mut StructB)

{
    dialog_ui_fn_1040_78e2(param_1);
    move_win_1040_826c(param_1, -0x1, 0xffff);
    ShowWindow16(0x5, (param_1 + 0x6));
    return;
}

pub unsafe fn unk_win_ui_op_1040_1d7a(param_1: *mut astruct_33, mut param_2: i16)

{
    let mut UVar2: u16;
    let mut UVar1: u16;
    let mut iVar3: *mut astruct_33;
    let mut uVar3: *mut astruct_33;
    let mut uVar1: u32;

    iVar3 = param_1;
    uVar3 = (param_1 >> 0x10);
    if ((param_2 != 0) && (uVar1 = iVar3.field141_0x8e, (uVar1 + 0x72) != 0)) {
        UVar2 = IsDlgButtonChecked(0xe1, iVar3.hwnd_0x6);
        if (UVar2 != 0) {
            pass1_1008_a930(iVar3.field142_0x92, 0x1d5);
        }
        UVar1 = IsDlgButtonChecked(0xe2, iVar3.hwnd_0x6);
        if (UVar1 != 0) {
            pass1_1008_a930(iVar3.field142_0x92, 0x1d6);
        }
        UVar1 = IsDlgButtonChecked(0xe3, iVar3.hwnd_0x6);
        if (UVar1 != 0) {
            pass1_1008_a930(iVar3.field142_0x92, 0x1d7);
        }
        UVar1 = IsDlgButtonChecked(0xe5, iVar3.hwnd_0x6);
        if (UVar1 != 0) {
            pass1_1008_a930(iVar3.field142_0x92, 0x1d8);
        }
        UVar1 = IsDlgButtonChecked(0xe6, iVar3.hwnd_0x6);
        if (UVar1 != 0) {
            pass1_1008_a930(iVar3.field142_0x92, 0x1e2);
        }
        UVar1 = IsDlgButtonChecked(0xe7, iVar3.hwnd_0x6);
        if (UVar1 != 0) {
            pass1_1008_a930(iVar3.field142_0x92, 0x1dc);
        }
        return;
    }
    DestroyWindow16(iVar3.hwnd_0x6);
    return;
}

pub unsafe fn pass1_1040_1e80(mut param_1: u16, mut param_2: i16, mut param_3: u16, mut param_4: u16, mut param_5: u32) -> u32

{
    let mut BStack6: bool;
    let mut uStack4: u16;

    BStack6 = 0;
    uStack4 = 0;
    if (param_5 == 0xe4) {
        pass1_1008_a9ec((param_2 + 0x92));
    } else {
        BStack6 = post_win_msg_1040_7b3c(CONCAT22(param_3, param_2), param_4, param_5, param_5);
        uStack4 = param_1;
    }
    return CONCAT22(uStack4, BStack6);
}

pub unsafe fn FUN_1040_1ec4()

{
    return;
}

pub unsafe fn pass1_1040_1ec8(param_1: *mut StructD, param_2: u8) -> *mut StructD

{
    pass1_1040_1d24(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1040_1ec8(param_1: *mut StructD, param_2: u8) -> *mut StructD

{
    pass1_1040_1d24(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}


pub unsafe fn create_win_1040_20d4(mut param_1: u32, struct_b_param_2: *mut StructB, mut param_3: u16)

{
    let mut cx: i16;
    let mut struct_b_1: *mut StructB;
    let mut uVar1: u16;
    let mut puVar2: *mut u32;
    let mut window_name: *mut c_char;
    let mut in_stack_0000fe72: u16;
    let mut in_stack_0000ff96: u16;
    let mut in_stack_0000ff9c: u16;
    let mut in_stack_0000ffa0: u16;
    let mut local_1e: RECT16;
    let mut iStack26: i16;
    let mut iStack24: *mut astruct_858;
    let mut uStack22: u32;
    let mut uStack18: u32;
    let mut iStack14: i16;
    let mut uStack12: u16;
    let mut iStack10: i16;
    let mut iStack8: i16;
    let mut uStack6: u16;
    let mut iStack4: i16;

    dialog_ui_fn_1040_78e2(struct_b_param_2);
    puVar2 = mixed_1010_20ba(param_1, _u16_1050_0ed0, CONCAT22(param_3, 0x48), in_stack_0000fe72,
                             in_stack_0000ff96, in_stack_0000ff9c, in_stack_0000ffa0);
    uStack12 = (puVar2 >> 0x10);
    iStack14 = puVar2;
    iStack8 = (iStack14 + 0xa);
    iStack10 = (iStack14 + 0xc);
    uVar1 = (struct_b_param_2 >> 0x10);
    struct_b_1 = struct_b_param_2;
    uStack18 = pass1_1008_4772(&struct_b_1[0x7].field1_0x2);
    cx = (uStack18 + 0x4);
    iStack4 = (iStack8 - cx) / 0x2;
    uStack6 = 0x5;
    SetWindowPos16(0x6, 0x1d6, cx, 0x5, iStack4, 0x0, struct_b_1.lpvoid_field_0x8);
    GetClientRect16(&local_1e, &DAT_1050_1050);
    window_name = load_string_1010_847e(_u16_1050_14cc, 0x592);
    uStack22 = 0x50010001;
    CreateWindow16(0x0, CONCAT22(0x1, HINSTANCE16_1050_038c), struct_b_1.lpvoid_field_0x8, 0x19, 0x58,
                   (iStack24 - 0x28), (iStack26 - 0x58) / 0x2, 0x1, s_Rebel_1050_4ffc + 0x5, window_name,
                   s_OPButton_1050_5ce4);
    SetWindowPos16(0x45, iStack10 - 0xa, (uStack18 + 0x4), 0x5, iStack4, 0x0,
                   struct_b_1.lpvoid_field_0x8);
    return;
}


pub unsafe fn mix_draw_op_1040_21d6(param_1: *mut astruct_763)

{
    let mut paVar1: *mut astruct_13;
    let mut ppcVar2: *mut *mut code;
    let mut uVar4: u8;
    let mut hpalette_7: HPALETTE16;
    let mut uVar7: u16;
    let mut handle: HANDLE16;
    let mut extraout_var: u32;
    let mut in_DX: u16;
    let mut extraout_DX: u16;
    let mut iVar10: *mut astruct_763;
    let mut count: i16;
    let mut uVar5: u32;
    let mut color: COLORREF;
    let mut handle_00: HGDIOBJ16;
    let mut hdc_24: HDC16;
    PAINTSTRUCT16 * paintstruct_22;
    let mut uVar1: u8;
    let mut uVar2: *mut u32;
    let mut uVar3: u16;
    let mut iVar5: *mut astruct_764;
    let mut uVar11: u16;

    count = (param_1 >> 0x10);
    iVar10 = param_1;
    hdc_24 = BeginPaint16(CONCAT22(0x1050, &paintstruct_22), iVar10.field6_0x6);
    paVar1 = (_PTR_LOOP_1050_4230 + 0xe);
    hpalette_7 = palette_op_1008_4e08(&hdc_24, in_DX, paVar1, CONCAT22(0x1050, &hdc_24));
    uVar2 = iVar10.field141_0x8e;
    ppcVar2 = (*iVar10.field141_0x8e + 0x4);
    (**ppcVar2)(0x1008, uVar2, (uVar2 >> 0x10), 0xffff, 0xffff, &hdc_24, &DAT_1050_1050);
    uVar5 = pass1_1008_4d72(paVar1);
    uVar3 = (uVar5 >> 0x10);
    iVar5 = uVar5;
    uVar7 = CONCAT11(iVar5.field_0x3e5, iVar5.field_0x3e6);
    uVar1 = iVar5.field996_0x3e4;
    color = SetBkColor16(0x0, hdc_24);
    extraout_DX = (color >> 0x10);
    uVar4 = SetTextColor16(CONCAT22(CONCAT11(0x2, uVar1), uVar7), hdc_24);
    handle_00 = 0;
    handle = GetProp16(s_hfont_1050_5ced, iVar10.field6_0x6);
    if (handle != 0) {
        handle_00 = SelectObject16(handle, hdc_24);
    }
    DrawText16(0x10,
               (param_1 & 0xff000000 | CONCAT12((param_1 >> 0x10), &iVar10.rect_0x92)),
               -0x1, iVar10.field152_0xa2, hdc_24);
    SetTextColor16(CONCAT22(CONCAT11(0x2, iVar5.field_0x94), CONCAT11(iVar5.field_0x95, iVar5.field_0x96)), hdc_24);
    DrawText16(0x10, (param_1 & 0xffff0000 | ZEXT24(&iVar10.field147_0x9a)), -0x1,
               iVar10.field153_0xa6, hdc_24);
    if (handle != 0) {
        SelectObject16(handle_00, hdc_24);
    }
    SetBkColor16(color, hdc_24);
    SetTextColor16(CONCAT31(extraout_var, uVar4) & 0xffff | extraout_DX << 0x10, hdc_24);
    hpalette_7 = SelectPalette16(0x0, hpalette_7, hdc_24);
    DeleteObject16(hpalette_7);
    EndPaint16(CONCAT22(0x1050, &paintstruct_22), iVar10.field6_0x6);
    return;
}


pub unsafe fn pass1_1040_2358(param_1: *mut StructD, param_2: u8) -> *mut StructD

{
    pass1_1040_205e(param_1);
    if ((param_2 & 1) != 0) {
        fn_ptr_1000_17ce(param_1);
    }
    return param_1;
}


pub unsafe fn pass1_1040_2464(param_1: *mut StructD)

{
    let mut uVar1: u16;

    uVar1 = (param_1 >> 0x10);
    param_1.address_offset_field_0x0 = 0x2956;
    (param_1 + 0x2) = &PTR_LOOP_1050_1040;
    pass1_1038_b6e0(_PTR_LOOP_1050_5b7c, (param_1 + 0x6));
    ui_cleanup_op_1040_782c(param_1);
    return;
}


pub unsafe fn show_win_1040_2490(struct_b_param_1: *mut StructB)

{
    let mut ppcVar1: *mut *mut code;
    let mut hwnd: HWND16;
    let mut struct_b_4: *mut StructB;
    let mut uVar3: u16;
    let mut piVar2: *mut i16;

    dialog_ui_fn_1040_78e2(struct_b_param_1);
    uVar3 = (struct_b_param_1 >> 0x10);
    struct_b_4 = struct_b_param_1;
    hwnd = GetDlgItem16(0xfb1, struct_b_4.lpvoid_field_0x8);
    EnableWindow16(0x0, hwnd);
    ppcVar1 = (*&struct_b_4[0x7].field1_0x2 + 0x10);
    piVar2 = (**ppcVar1)(s_tile2_bmp_1050_1538, &struct_b_4[0x7].field1_0x2);
    piVar2 = (piVar2 >> 0x10);
    move_win_1040_826c(struct_b_param_1, (piVar2 + 0x2) - 0x2, (piVar2 + 0x4) + *piVar2 + 0x3);
    ShowWindow16(0x5, struct_b_4.lpvoid_field_0x8);
    pass1_1018_1c9a(&struct_b_4[0x7].field1_0x2, 0x1a0);
    return;
}


pub unsafe fn win_ui_op_1040_2512(param_1: *mut Struct57, mut param_2: u16, param_3: *mut StructC, mut param_4: u32, mut param_5: u16) -> u32

{
    let mut puVar1: *mut u32;
    let mut uVar2: u16;
    let mut paVar3: *mut astruct_263;
    let mut uVar4: u16;
    let mut UVar4: u16;
    let mut HVar5: HWND16;
    let mut BVar6: bool;
    let mut iVar6: i16;
    let mut iVar7: i16;
    let mut UVar6: u16;
    let mut uVar7: u16;
    let mut uVar11: u16;
    let mut puVar8: *mut u8;
    let mut uVar12: u16;
    let mut puVar9: *mut u8;
    let mut iVar8: *mut StructC;
    let mut iVar9: i16;
    let mut uVar8: u16;
    let mut uVar9: u16;
    let mut puVar15: *mut u16;
    let mut uVar10: u32;
    let mut in_stack_0000fe84: u16;
    let mut in_stack_0000ffa8: u16;
    let mut in_stack_0000ffae: u16;
    let mut in_stack_0000ffb2: u16;
    let mut local_1e: [u8; 0x4] = [0; 0x4];
    let mut uStack26: u16;
    let mut uStack24: u16;
    let mut local_16: [*mut u32; 0x2];
    let mut uStack12: u16;
    let mut puStack10: *mut u32;
    let mut BStack6: bool;
    let mut uStack4: u16;
    let mut piVar1: *mut i16;
    let mut in_stack_0000ffdc: u16;
    let mut uVar13: u32;
    let mut uVar14: u32;
    let mut fn_ptr_21: *mut *mut code;

    BStack6 = 0;
    uStack4 = 0;
    if (param_5 == 0x2) {//
// LAB_1040_266d:
        BStack6 = 0x1;
        uStack4 = 0;
    } else {
        iVar8 = param_3;
        if (param_5 - 0x2 < 0x19e) {//
// LAB_1040_2539:
            param_2 = param_5;
        } else {
            uVar8 = (param_3 >> 0x10);
            if (param_5 - 0x1a0 < 0x14 || param_5 == 0x1b4) {
                UVar4 = IsDlgButtonChecked(param_5, iVar8.field6_0x6);
                if (UVar4 == 0) {
                    puVar1 = &iVar8.field142_0x92;
                    puVar1 = puVar1 + 1;
                    if (0x0 < &iVar8.field142_0x92) {
                        (&iVar8.field142_0x92 + 0x2) = 0;
                    }
                    paVar3 = iVar8.field141_0x8e;
                    if ((paVar3 + 0x28) == &iVar8.field142_0x92) {
                        HVar5 = GetDlgItem16(0xfb1, iVar8.field6_0x6);
                        EnableWindow16(0x0, HVar5);
                    }
                } else {
                    puVar1 = &iVar8.field142_0x92;
                    puVar1 = puVar1 - 0x1;
                    HVar5 = GetDlgItem16(0xfb1, iVar8.field6_0x6);
                    BVar6 = IsWindowEnabled16(HVar5);
                    if (BVar6 == 0) {
                        HVar5 = GetDlgItem16(0xfb1, iVar8.field6_0x6);
                        EnableWindow16(0x1, HVar5);
                    }
                    if (&iVar8.field142_0x92 < 1) {
                        (&iVar8.field142_0x92 + 0x2) = 0x1;
                    }
                    pass1_1018_1c9a(iVar8.field141_0x8e, param_5);
                    puStack10 = pass1_1018_1e78(iVar8.field141_0x8e, -1);
                    uVar2 = (puStack10 >> 0x10);
                    uVar11 = uVar2 | puStack10;
                    if (uVar11 == 0) {
                        uStack12 = 0;
                    } else {
                        uStack12 = (puStack10 + 0x1c);
                    }
                    win_1008_5c7c(uStack12, uVar11, _u16_1050_02a0, CONCAT22(uStack12, 1));
                }
                if ((-0x1 < &iVar8.field142_0x92) && (paVar3 = iVar8.field141_0x8e, &iVar8.field142_0x92 <= (paVar3 + 0x28))) {
                    sys_1000_3f9c(CONCAT13(0x10, CONCAT12(0x50, local_16)), s__d_1050_5cf4, &iVar8.field142_0x92);
                    SetDlgItemText16(CONCAT22(0x1050, local_16), 0xfb2, iVar8.field6_0x6);
                }
                // TODO: goto LAB_1040_266d;
            }
            uVar4 = param_5 - 0xfb1;
//      if (uVar4 != 0) goto LAB_1040_2539;
            if (&iVar8.field142_0x92 < 0x0) {
                mem_op_1000_179c(0xb4, param_1);
                uStack24 = param_1;
                puVar8 = (uStack24 | uVar4);
                uVar13 = param_1 & 0xffff0000 | ZEXT24(puVar8);
                uStack26 = uVar4;
                if (puVar8.is_null()) {
                    iVar6 = 0;
                    uVar12 = 0;
                } else {
                    iVar6 = string_1040_8520(uVar13,
                                             CONCAT13((param_1 >> 0x8), CONCAT12(param_1, uVar4)),
                                             HWND16_1050_0396, 0x20030, 0x57c057b);
                    uVar12 = uVar13;
                }
                puStack10 = CONCAT22(uVar12, iVar6);
                fn_ptr_21 = (*puStack10 + 0x74);
                (**fn_ptr_21)(0x1000, iVar6, uVar12);
                // TODO: goto LAB_1040_27c0;
            }
            if (0x0 < &iVar8.field142_0x92) {
                mem_op_1000_179c(0xb4, param_1);
                uStack24 = param_1;
                puVar9 = (uStack24 | uVar4);
                uVar13 = param_1 & 0xffff0000;
                uVar14 = uVar13 | ZEXT24(puVar9);
                uStack26 = uVar4;
                if (puVar9.is_null()) {
                    iVar7 = 0;
                } else {
                    iVar7 = string_1040_8520(uVar14,
                                             CONCAT13((param_1 >> 0x8), CONCAT12(param_1, uVar4)),
                                             HWND16_1050_0396, 0x20021, 0x57d057b);
                    uVar13 = uVar14;
                }
                puStack10 = CONCAT22(uVar13, iVar7);
                puVar15 = pass1_1008_941a(CONCAT22(0x1050, local_1e), 0x1, 0xc2);
                param_1 = (uVar13 & 0xffff0000 | puVar15 >> 0x10);
                param_2 = &DAT_1050_1050;
                fn_ptr_21 = (*puStack10 + 0x6c);
                uVar10 = (**fn_ptr_21)(0x1008, puStack10, (puStack10 >> 0x10), local_1e);
//        if (uVar10 == 0x2) goto LAB_1040_27c0;
            }
            local_16[0] = mixed_1010_20ba(param_1, _u16_1050_0ed0, CONCAT22(param_2, 0x6), in_stack_0000fe84,
                                          in_stack_0000ffa8, in_stack_0000ffae, in_stack_0000ffb2);
            param_1 = (local_16[0] >> 0x10);
            uStack12 = 0x1a0;
            loop {
                UVar6 = IsDlgButtonChecked(uStack12, iVar8.field6_0x6);
                if (UVar6 == 1) {
                    uVar9 = (local_16[0] >> 0x10);
                    iVar9 = local_16[0];
                    (iVar9 + (iVar9 + 0x56) * 0x2 + 0x4e) = uStack12;
                    piVar1 = (iVar9 + 0x56);
                    *piVar1 = *piVar1 + 1;
                }
                uStack12 += 0x1;
                if uStack12 >= 0x1b5 { break; }
            }
            uVar2 = &iVar8.field142_0x92;
            puStack10 = (puStack10 & 0xffff0000 | uVar2);
            paVar3 = iVar8.field141_0x8e;
            (paVar3 + 0x28) = uVar2;
            PostMessage16(0x0, 0xc8, 0x111, HWND16_1050_0396);
            param_2 = 0x1;
        }
        uVar12 = SUB42(param_1, 0x0);
        BStack6 = post_win_msg_1040_7b3c(param_3, param_4, (param_4 >> 0x10), param_2);
        uStack4 = uVar12;
    }//
// LAB_1040_27c0:
    return CONCAT22(uStack4, BStack6);
}
