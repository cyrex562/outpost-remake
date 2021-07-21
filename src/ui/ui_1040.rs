use crate::cleanup::unk_destroy_win_op_1010_2fa0;
use crate::debug::debug_print_1008_6048;
use crate::defines::{Struct18, Struct19, Struct20, Struct65};
use crate::draw::draw_1040::draw_op_1040_9948;
use crate::fn_ptr::fn_ptr_1000::{fn_ptr_1000_17ce, fn_ptr_op_1000_1708};
use crate::fn_ptr::fn_ptr_1028::fn_ptr_1030_835a;
use crate::global::AppContext;
use crate::mem_1000::{mem_op_1000_160a, mem_op_1000_179c};
use crate::mixed::mixed_1010_20ba;
use crate::pass::pass_1000::{pass1_1000_4906, pass1_1000_5586};
use crate::pass::pass_1008::{pass1_1008_3bd6, pass1_1008_3e94, pass1_1008_4772, pass1_1008_4d72, pass1_1008_5784, pass1_1008_5b12, pass1_1008_5fd8};
use crate::pass::pass_1010::{pass1_1010_038e, pass1_1010_0898, pass1_1010_2ee2, pass1_1010_32c0, pass1_1010_4f30, pass1_1010_5120, pass1_1010_519a, pass1_1010_52fc, pass1_1010_531c, pass1_1010_5d9c, pass1_1010_5f7a, pass1_1010_5fb0, pass1_1010_60a0, pass1_1010_60b4, pass1_1010_60ba, pass1_1010_60c0, pass1_1010_60c6, pass1_1010_60fa, pass1_1010_6118, pass1_1010_9044, pass1_1010_91cc, pass1_1010_9210, pass1_1010_a50c, pass1_1010_a568, pass1_1010_a58a, pass1_1010_a5ac, pass1_1010_a5ca, pass1_1010_a5ec, pass1_1010_ac62, pass1_1010_acc0, pass1_1010_c320, pass1_1010_c3c2};
use crate::pass::pass_1018::{pass1_1018_1c9a, pass1_1018_2678, pass1_1018_3424, pass1_1018_36e6, pass1_1018_3710, pass1_1018_3a42, pass1_1018_3a5c, pass1_1018_3a7a, pass1_1018_3a94, pass1_1018_3ab2, pass1_1018_3cda, pass1_1018_3d44, pass1_1018_50ea, pass1_1018_5206, pass1_1018_57d2};
use crate::pass::pass_1020::{pass1_1020_bae6, string_1020_c0d8, string_op_1020_c222};
use crate::pass::pass_1028::{pass1_1028_4a9a, pass1_1028_8d9e, pass1_1028_8dec};
use crate::pass::pass_1030::{pass1_1030_6ddc, pass1_1030_6e14, pass1_1030_8326, pass1_1030_8344};
use crate::pass::pass_1040::{pass1_1040_5cd6, pass1_1040_5dc4, pass1_1040_9824, pass1_1040_a5d0, pass1_1040_b45e, pass1_1040_b54a, pass1_1040_bfde, struct_1040_bf3e};
use crate::string::string_1000::{str_op_1000_3da4, string_1000_3cea, string_1000_3d3e};
use crate::string::string_1010::{load_string_1010_847e, load_string_1010_84e0, load_string_1010_ac92, string_1010_5286, string_op_1010_c446};
use crate::string::string_1018::{string_1018_39d8, unk_str_op_1018_35b0};
use crate::string::string_1040::string_1040_8520;
use crate::struct_ops::struct_1008::clear_struct_1008_3e38;
use crate::struct_ops::struct_1010::{struct_1010_9172, struct_1010_dd5e};
use crate::struct_ops::struct_1030::struct_op_1030_73a8;
use crate::struct_ops::struct_1040::struct_1040_a598;
use crate::switch_ops::switch_1018::switch_1018_3b9e;
use crate::sys_api::{reg_class_1040_98c0, sprintf_op_1018_34b6};
use crate::ui::ui_1008::pass1_1008_a930;
use crate::ui::ui_1010::{ui_op_1010_79aa, win_ui_op_1010_3202};
use crate::ui::ui_1038::set_win_pos_1038_abdc;
use crate::util::{CONCAT11, CONCAT12, CONCAT13, CONCAT22, SUB42, ZEXT24};
use crate::win_struct::{COLORREF, HANDLE16, HBRUSH16, HCURSOR16, HDC16, HGDIOBJ16, HINSTANCE16, HMENU16, HWND16, LPARAM, LRESULT, MSG16, POINT16, RECT16, SEGPTR, WNDCLASS16, WPARAM16};
use crate::winapi::{CallWindowProc16, CheckDlgButton16, CheckRadioButton16, ClientToScreen16, CreateDialog16, CreateSolidBrush16, CreateWindow16, DefWindowProc16, DeleteObject16, DestroyMenu16, DestroyWindow16, EnableWindow16, FreeProcInstance16, GetClientRect16, GetCursorPos16, GetDC16, GetDlgCtrlID16, GetDlgItem16, GetDlgItemInt16, GetMessage16, GetNextDlgTabItem16, GetProp16, GetStockObject16, GetSubMenu16, GetSystemMetrics16, GetTextExtent16, GetWindowLong16, GetWindowRect16, GetWindowText16, InvalidateRect16, IsDialogMessage16, IsDlgButtonChecked, IsWindow16, LoadCursor16, LoadMenu16, MapDialogRect16, MessageBox16, MoveWindow16, PostMessage16, PtInRect16, ReleaseCapture16, ReleaseDC16, RemoveProp16, ScreenToClient16, SendDlgItemMessage16, SendMessage16, SetBkColor16, SetCapture16, SetDlgItemInt16, SetDlgItemText16, SetFocus16, SetProp16, SetSysModalWindow, SetTextColor16, SetWindowLong16, SetWindowPos16, SetWindowText16, ShowWindow16, TrackPopupMenu16, UpdateWindow16, WinHelp16, wsprintf16};

pub unsafe fn show_win_1040_0766(
    ctx: &mut AppContext,
    struct_1: &mut Struct1,
    param_2: u16,
    struct_2: &mut Struct19,
    unaff_di: i16,
    wndclass_1: &mut WNDCLASS16
) {
    let pu_var1: U32Ptr = ();
    let pu_var2: U32Ptr;
    let pi_var3: U32Ptr;
    let pi_var4: U32Ptr;
    let u_var5: u16;
    let local_a: i16;
    let local_8: i16;
    let struct_3: &mut Struct20;

    dialog_ui_fn_1040_78e2(struct_1, param_2);
    struct_3 = mixed_1010_20ba(
        ctx,
        ctx.PTR__LOOP_1050_0ed0,
        0x2,
        wndclass_1,
        struct_2,
        unaff_di
    );
    // pu_var1 = (u_stack6 >> 0x10);
    pass1_1010_6118(struct_3);
    pi_var4 = &local_8;
    pi_var3 = &local_a;
    u_var5 = wndclass_1;
    pu_var2 = mixed_1010_20ba(
        ctx,
        ctx.PTR__LOOP_1050_0ed0,
        0x48,
        wndclass_1,
        pu_var1,
        unaff_di
    );
    pass1_1008_3e94((pu_var2 & 0xffff0000 | (pu_var2 + 0xe)),
                    CONCAT22(wndclass_1, pi_var3), CONCAT22(u_var5, pi_var4));
    move_win_1040_826c(struct_1, local_a + 0x8c, local_8 + 0xb9);
    ShowWindow16(0x1008, 0x5);
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn win_ui_op_1040_07dc(
    ctx: &mut AppContext,
    param_1: u16,
    param_2: u16,
    param_3: u16,
    param_4: u16,
    param_5: u16,
    param_6: u16,
    param_7: HWND16,
    param_8: u16
)

{
    let ppc_var1: u32;
    let i_var2: i16;
    let b_var3: bool;
    let pu_var4: U32Ptr;
    let pu_var5: U32Ptr;
    let unaff_di: i16;
    let u_var6: u16;
    let pu_var7: U32Ptr;
    let pu_var8: u32;
    let u_var9: u8;
    let u_var10: u8;
    let u_stack2060: u32;
    let mut local_806: String = String::new();
    let local_406: [u32; 0x100] = [0; 0x100];
    let u_stack6: u32;

    u_stack6 = 0x0;
    if param_5 == 0x73 {
        enable_window_1040_0acc(param_1, param_2, false, param_7);
        pu_var4 = pass1_1008_5fd8(param_8, param_6);
        u_stack2060 = CONCAT22(param_6, pu_var4);
        pu_var5 = param_6;
        load_string_1010_84e0(
            0x1010,
            ctx._PTR_LOOP_1050_14cc,
            0x3ff,
            local_806,
            param_8
        );
        i_var2 = MessageBox16(
            0x1010,
            (ctx.s_New_failed_in_Op__Op_1050_0020 + 0x14),
            local_806,
            param_8
        );
        local_406[0] = u_stack2060;
        u_var6 = 0x1000;
        fn_ptr_1000_17ce(ctx, CONCAT22(param_6, pu_var4), 0x1000);
        if (i_var2 == 0x6) {
            u_var6 = ctx.s_tile2_bmp_1050_1538;
            PostMessage16(0x1000, 0x0, 0x0, 0x11100cb);
            b_var3 = post_win_msg_1040_7b3c(CONCAT22(param_2, param_1), param_3, param_4, 0x1,
                                            ctx.s_tile2_bmp_1050_1538);
            u_stack6 = CONCAT22(pu_var5, b_var3);
        }
    } else {
        u_var9 = (param_2 >> 0x8);
        if (param_5 < 0x74) {
            if (param_5 == 0x6e) {
                (ctx.PTR__LOOP_1050_5b7c + 0xae) = 0x99;
                pu_var8 = pass1_1038_af40(ctx.PTR__LOOP_1050_5b7c, (param_1 + 0x6), 0x2,
                                          param_6, param_1, &ctx.PTR_LOOP_1050_1038, param_8);
                ppc_var1 = (*pu_var8 + 0x3c);
                (**ppc_var1)(&ctx.PTR_LOOP_1050_1038, pu_var8, (pu_var8 >> 0x10));
                SetFocus16(&ctx.PTR_LOOP_1050_1038);
                return;
            }
            if (0x6e < param_5) {
//LAB_1040_09f9:
                post_win_msg_1040_7b3c(CONCAT13(u_var9, CONCAT12(param_2, param_1)), param_3,
                                       param_4, param_5, param_7);
                return;
            }
            if (param_5 == '\x02') {
//LAB_1040_09b4:
                post_win_msg_1040_7b3c(CONCAT13(u_var9, CONCAT12(param_2, param_1)), 0x0, 0x0, 0x2,
                                       param_7);
                PostMessage16(param_7, 0x0, 0x0, 0x11100ee);
                return;
            }
            if (param_5 != 'd') {
                // goto
                // LAB_1040_09f9;
            }
            u_var9 = 0x0;
            u_var10 = 0x0;
            u_var6 = SUB42(ctx.s_tile2_bmp_1050_1538, 0x0);
            PostMessage16(param_7, 0x0, 0x0, 0x1110064);
//       TODO: goto LAB_1040_0821;
        }
        if (param_5 != 0x74) {
            if (param_5 == 0xee) {
                // goto
                // LAB_1040_09b4;
            }
            if (param_5 == 0x13d) {
                enable_window_1040_0acc(param_1, param_2, 0x1, param_7);
                return;
            }
//       TODO: goto LAB_1040_09f9;
        }
        enable_window_1040_0acc(param_1, param_2, 0x0, param_7);
        load_string_1010_84e0(0x1010, _PTR_LOOP_1050_14cc, 0x3ff, local_406,
                              param_8);
        load_string_1010_84e0(0x1010, _PTR_LOOP_1050_14cc, 0x3ff, local_806, param_8);
        u_var6 = SUB42(ctx.s_tile2_bmp_1050_1538, 0x0);
        i_var2 = MessageBox16(0x1010, (s_New_failed_in_Op__Op_1050_0020 + 0x14),
                              local_406, param_8);
        if (i_var2 == 0x6) {
            PostMessage16(ctx.s_tile2_bmp_1050_1538, 0x0, 0x0, 0x111007a);
            b_var3 = post_win_msg_1040_7b3c(CONCAT22(param_2, param_1), param_3, param_4, 0x1,
                                            ctx.s_tile2_bmp_1050_1538);
            u_stack6 = CONCAT22(param_6, b_var3);
            pu_var7 = mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0, 0x2, param_8, param_6, unaff_di);
            u_var6 = 0x1010;
            pass1_1010_60fa(pu_var7);
        }
    }
    u_var9 = 0x1;
    u_var10 = 0x0;
//LAB_1040_0821:
    enable_window_1040_0acc(param_1, param_2, CONCAT11(u_var10, u_var9), u_var6);
    return;
}


pub fn enable_window_1040_0acc(param_1: u16, param_2: u16, param_3: bool, param_4: HWND16) {
    let BVar1: bool;

    BVar1 = IsWindow16(param_4);
    if (BVar1 != 0x0) {
        GetDlgItem16(ctx.s_tile2_bmp_1050_1538, 0x64);
        BVar1 = IsWindow16(ctx.s_tile2_bmp_1050_1538);
        if (BVar1 != 0x0) {
            EnableWindow16(ctx.s_tile2_bmp_1050_1538, param_3);
            GetDlgItem16(ctx.s_tile2_bmp_1050_1538, 0x74);
            EnableWindow16(ctx.s_tile2_bmp_1050_1538, param_3);
            GetDlgItem16(ctx.s_tile2_bmp_1050_1538, 0x73);
            EnableWindow16(ctx.s_tile2_bmp_1050_1538, param_3);
            GetDlgItem16(ctx.s_tile2_bmp_1050_1538, 0x6e);
            EnableWindow16(ctx.s_tile2_bmp_1050_1538, param_3);
            GetDlgItem16(ctx.s_tile2_bmp_1050_1538, 0xee);
            EnableWindow16(ctx.s_tile2_bmp_1050_1538, param_3);
        }
    }
    return;
}


pub fn show_win_1040_0c7c(param_1: &mut Struct1, param_2: u16, param_3: u16) {
    let uVar1: u32;
    let local_6: u32;

    dialog_ui_fn_1040_78e2(param_1, param_2);
    uVar1 = (param_1 + 0x8e);
    pass1_1010_4f30(uVar1, (uVar1 >> 0x10),
                    CONCAT22(param_3, &local_6),
                    CONCAT22(param_3, &local_6 + 0x2));
    move_win_1040_826c(param_1, local_6, (local_6 >> 0x10));
    ShowWindow16(0x1010, 0x5);
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn set_text_bk_color_1040_0cc0(param_1: U32Ptr, param_2: u16, param_3: u16, param_4: i16) -> u32

{
    let ppcVar1: u32;
    let iVar2: i16;
    let obj: HDC16;
    let hdc: HDC16;
    let uVar3: u32;
    let uVar4: u16;
    let HStack4: HGDIOBJ16;

    uVar4 = 0x4;
    obj = ctx.s_tile2_bmp_1050_1538;
    HStack4 = GetStockObject16(param_4);
    if (ctx.PTR__LOOP_1050_5cd0 == 0x0) {
        ppcVar1 = (*param_1 + 0x68);
        uVar3 = (**ppcVar1)(ctx.s_tile2_bmp_1050_1538, param_1,
                            (param_1 + 0x6e), uVar4);
        obj = 0x1008;
        uVar3 = pass1_1008_4d72(uVar3);
        // uVar4 = (uVar3 >> 0x10);
        iVar2 = uVar3;
        ctx._PTR_LOOP_1050_5cd0 = CONCAT22(CONCAT11(0x2, *(iVar2 + 0x94)),
                                           CONCAT11(*(iVar2 + 0x95), *(iVar2 + 0x96)));
    }
    hdc = obj;
    if (0x3 < param_3) {
        if (param_3 == 0x4) {
            hdc = ctx.s_tile2_bmp_1050_1538;
            HStack4 = GetStockObject16(obj);
        } else {
            if ((param_3 == 0x4) || (0x1 < param_3 - 0x5)) {
                return 0x0;
            }
        }
    }
    SetTextColor16(hdc, _PTR_LOOP_1050_5cd0);
    SetBkColor16(ctx.s_tile2_bmp_1050_1538, 0x0);
    return CONCAT22(0x1050, HStack4);
}


pub fn post_win_msg_1040_0d5e(param_1: u16, param_2: u16, param_3: i16, param_4: HWND16) {
    if (param_3 != 0x0) {
        PostMessage16(param_4, 0x0, 0x0, 0x1110001);
    }
    return;
}


pub fn set_win_pos_1040_0f10(param_1: HWND16, param_2: u16, param_3: i16) {
    let piVar1: U32Ptr;
    let uVar2: u32;
    let iVar3: i16;
    let unaff_DI: i16;
    let uVar4: u16;
    let uVar5: u32;
    let puVar6: U32Ptr;
    let check: u16;

    dialog_ui_fn_1040_78e2((param_3 + 0x6), param_1);
    uVar2 = (param_3 + 0x6);
    // uVar4 = (uVar2 >> 0x10);
    iVar3 = uVar2;
    if ((iVar3 + 0x98) == 0x0) {
        GetWindowRect16(param_1, (param_3 + -0x24));
        GetDlgItem16(ctx.s_tile2_bmp_1050_1538, 0x1830);
        GetWindowRect16(ctx.s_tile2_bmp_1050_1538, (param_3 + -0x2c));
        piVar1 = (param_3 + -0x20);
        *piVar1 = *piVar1 - (param_3 + -0x24);
        iVar3 = ((param_3 + -0x2a) - (param_3 + -0x22)) + -0x2;
        (param_3 + -0x1e) = iVar3;
        SetWindowPos16(ctx.s_tile2_bmp_1050_1538, 0x6, iVar3, (param_3 + -0x20), 0x0, 0x0, 0x0);
        CheckDlgButton16(ctx.s_tile2_bmp_1050_1538, 0x1, 0x1c1);
        uVar2 = (param_3 + 0x6);
        uVar2 = (uVar2 + 0x8e);
        (uVar2 + 0xa) = 0x2;
        GetDlgItem16(ctx.s_tile2_bmp_1050_1538, 0x1830);
        EnableWindow16(ctx.s_tile2_bmp_1050_1538, 0x0);
    } else {
        uVar2 = (iVar3 + 0x92);
        uVar5 = struct_op_1030_73a8((uVar2 + 0x6));
        (param_3 + -0x32) = uVar5;
        (param_3 + -0x30) = (uVar5 >> 0x10);
        uVar2 = (param_3 + -0x32);
        if ((uVar2 + 0x20) == 0x2) {
            check = 0x1c1;
        } else {
            check = 0x1c2;
        }
        CheckDlgButton16(0x1030, 0x1, check);
    }
    GetCursorPos16(s_tile2_bmp_1050_1538);
    GetWindowRect16(ctx.s_tile2_bmp_1050_1538, (param_3 + -0xc));
    iVar3 = (param_3 + -0x8) - (param_3 + -0xc);
    (param_3 + -0x12) = iVar3;
    (param_3 + -0xe) = -(iVar3 / 0x2 - (param_3 + -0x4));
    iVar3 = (param_3 + -0x6) - (param_3 + -0xa);
    (param_3 + -0x14) = iVar3;
    (param_3 + -0x10) = -(iVar3 / 0x2 - (param_3 + -0x2));
    puVar6 = mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0, 0x48, param_2, (iVar3 >> 0xf),
                             unaff_DI);
    // uVar4 = (puVar6 >> 0x10);
    iVar3 = puVar6;
    (param_3 + -0x1c) = iVar3;
    (param_3 + -0x1a) = uVar4;
    (param_3 + -0x16) = (iVar3 + 0xa);
    (param_3 + -0x18) = (iVar3 + 0xc);
    if ((param_3 + -0x16) < (param_3 + -0x12) + (param_3 + -0xe)) {
        (param_3 + -0xe) = (param_3 + -0x16) - (param_3 + -0x12);
    }
    if ((param_3 + -0x18) < (param_3 + -0x14) + (param_3 + -0x10)) {
        (param_3 + -0x10) = (param_3 + -0x18) - (param_3 + -0x14);
    }
    uVar2 = (param_3 + -0x10);
    SetWindowPos16(0x1010, 0x45, 0x0, 0x0, uVar2, (uVar2 >> 0x10), 0x0);
    return;
}


pub fn win_ui_op_1040_12bc(param_1: &mut Struct1, param_2: u16, param_3: U32Ptr) {
    let uVar1: u32;
    wparam: WPARAM16;
    let HVar2: HWND16;
    let uVar3: u16;
    let in_AF: u8;
    let mut pcVar4: String;
    ulocal_54: u8[0x52];

    dialog_ui_fn_1040_78e2(param_1, param_2);
    uVar1 = (param_1 + 0x8e);
    // uVar3 = (uVar1 >> 0x10);
    sys_1000_3f9c(local_54, param_3, 0x5cd4, ctx.data_seg,
                  (uVar1 + 0xa), &stack0xfffe, uVar3, 0x1000, param_3, in_AF);
    GetDlgItem16(0x1000, 0xfd2);
    SendMessage16(ctx.s_tile2_bmp_1050_1538, local_54, param_3, 0xc0000);
    SetFocus16(ctx.s_tile2_bmp_1050_1538);
    SendMessage16(ctx.s_tile2_bmp_1050_1538, 0x0, 0xffff, 0x4010000);
    move_win_1040_826c(param_1, -0x1, 0xffff);
    pcVar4 = load_string_1010_847e(ctx.PTR__LOOP_1050_14cc, (ctx.PTR__LOOP_1050_14cc >> 0x10), 0x1010);
    // wparam = (pcVar4 >> 0x10);
    HVar2 = GetDlgItem16(0x1010, (s_vrpal_bmp_1050_183a + 0x5));
    send_msg_1040_1696(param_1, HVar2, param_3, s_tile2_bmp_1050_1538);
    SendMessage16(ctx.s_tile2_bmp_1050_1538, pcVar4, wparam, 0x40dffff);
    HVar2 = GetDlgItem16(ctx.s_tile2_bmp_1050_1538, (s_vrpal_bmp_1050_183a + 0x4));
    send_msg_1040_1696(param_1, HVar2, param_3, s_tile2_bmp_1050_1538);
    SendMessage16(ctx.s_tile2_bmp_1050_1538, pcVar4, wparam, 0x40dffff);
    ShowWindow16(ctx.s_tile2_bmp_1050_1538, 0x5);
    return;
}


// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn win_msg_op_1040_13b2(param_1: u32, param_2: i16, param_3: HWND16, param_4: u16) {
    let ppcVar1: u32;
    let uVar2: u32;
    let uVar3: u16;
    let iVar4: i16;
    let puVar5: U32Ptr;
    let puVar6: U32Ptr;
    let puVar7: U32Ptr;
    let iVar8: i16;
    let uVar9: u16;
    let uVar10: u16;
    let in_AF: u8;
    let LVar11: LRESULT;
    let puStack562: u32;
    let local_22e: [u8; 118];
    let uStack278: u32;
    let uStack274: u32;
    let puStack270: U32Ptr;
    let puStack268: U32Ptr;
    let uStack266: u32;
    let uStack262: u16;
    let mut pcStack260: String;
    let local_100: [u8; 52];
    let iStack174: i16;
    let HStack172: HWND16;
    ulocal_aa: u8[0x52];
    let uStack88: u16;
    let HStack86: HWND16;
    let local_54: [u8; 52];

    iVar4 = param_1;
    // uVar9 = (param_1 >> 0x10);
    if (param_2 != 0x0) {
        HStack86 = GetDlgItem16(param_3, 0xfd2);
        SendMessage16(ctx.s_tile2_bmp_1050_1538, local_54, param_4, 0xd0051);
        uStack88 = pass1_1000_3e2c(CONCAT22(param_4, local_54));
        HStack172 = GetDlgItem16(0x1000, (s_vrpal_bmp_1050_183a + 0x4));
        LVar11 = SendMessage16(ctx.s_tile2_bmp_1050_1538, 0x0, 0x0, 0x4070000);
        iStack174 = LVar11;
        if (iStack174 != -0x1) {
            SendMessage16(ctx.s_tile2_bmp_1050_1538, local_aa, param_4,
                          CONCAT22(0x408, iStack174));
        }
        HStack172 = GetDlgItem16(ctx.s_tile2_bmp_1050_1538,
                                 (s_vrpal_bmp_1050_183a + 0x5));
        LVar11 = SendMessage16(ctx.s_tile2_bmp_1050_1538, 0x0, 0x0, 0x4070000);
        iStack174 = LVar11;
        if (iStack174 != -0x1) {
            SendMessage16(ctx.s_tile2_bmp_1050_1538, local_100, param_4,
                          CONCAT22(0x408, iStack174));
        }
        pcStack260 = load_string_1010_847e(ctx.PTR__LOOP_1050_14cc,
                                           (ctx.PTR__LOOP_1050_14cc >> 0x10), 0x1010);
        puVar6 = local_aa;
        uVar3 = pass1_1000_3d7a(CONCAT22(param_4, puVar6), CONCAT22(param_4, local_100));
        if (uVar3 != 0x0) {
            uVar3 = pass1_1000_3d7a(CONCAT22(param_4, local_aa), pcStack260);
            if (uVar3 != 0x0) {
                uVar3 = pass1_1000_3d7a(CONCAT22(param_4, local_100), pcStack260);
                if (uVar3 != 0x0) {
                    pass1_1010_531c((iVar4 + 0x8e), CONCAT22(param_4, local_aa),
                                    local_aa, puVar6, param_4);
                    puVar5 = local_100;
                    pass1_1010_52fc((iVar4 + 0x8e), CONCAT22(param_4, puVar5), puVar5, puVar6, param_4);
                    pass1_1010_5120((iVar4 + 0x8e), uStack88, puVar5, puVar6,
                                    param_4);
                    uStack266 = CONCAT22(uStack266._2_2_, puVar5);
                    if (puVar5 == 0x0) {
                        mem_op_1000_179c(0xb4, puVar6, 0x1000);
                        puVar7 = (puVar6 | puVar5);
                        puStack270 = puVar5;
                        puStack268 = puVar6;
                        if (puVar7 == 0x0) {
                            iVar4 = 0x0;
                            puVar7 = 0x0;
                        } else {
                            iVar4 = string_1040_8520(CONCAT22(puVar6, puVar5),
                                                     ctx.PTR_LOOP_1050_0396, 0x30, 0x2, 0x57b, 0x7d2,
                                                     puVar7, param_4);
                        }
                        puStack562 = CONCAT22(puVar7, iVar4);
                        ppcVar1 = (*puStack562 + 0x74);
                        (**ppcVar1)(0x1000, iVar4, puVar7);
                        return;
                    }
                    uVar2 = (iVar4 + 0x8e);
                    uStack274 = (uVar2 + 0x12);
                    uVar2 = (iVar4 + 0x8e);
                    // uVar10 = (uVar2 >> 0x10);
                    iVar8 = uVar2;
                    uStack278 = (iVar8 + 0x16);
                    uVar2 = (iVar4 + 0x8e);
                    uStack262 = (uVar2 + 0xa);
                    pass1_1028_8d9e(CONCAT22(param_4, local_22e), uStack262,
                                    uStack274,
                                    uStack278 & 0xffff | (iVar8 + 0x18) << 0x10,
                                    param_4, in_AF);
                    fn_ptr_1030_835a(ctx.PTR__LOOP_1050_5748, CONCAT22(param_4, local_22e));
                    param_3 = &USHORT_1050_1028;
                    pass1_1028_8dec(CONCAT22(param_4, local_22e));
//           TODO: goto LAB_1040_1619;
                }
            }
        }
        param_3 = 0x1000;
        mem_op_1000_179c(0xb4, puVar6, 0x1000);
        puVar7 = (puVar6 | uVar3);
        puStack270 = uVar3;
        puStack268 = puVar6;
        if (puVar7 == 0x0) {
            iVar4 = 0x0;
            puVar7 = 0x0;
        } else {
            iVar4 = string_1040_8520(CONCAT22(puVar6, uVar3),
                                     ctx.PTR_LOOP_1050_0396, 0x30, 0x2, 0x57b, 0x755, puVar7,
                                     param_4);
        }
        uStack266 = CONCAT22(puVar7, iVar4);
        ppcVar1 = (*uStack266 + 0x74);
        (**ppcVar1)(0x1000, iVar4, puVar7);
    }
//LAB_1040_1619:
    DestroyWindow16(param_3);
    return;
}



u32
set_win_pos_1040_162a
(param_1: u16,param_2: u32,param_3: u32,param_4: u16,param_5: HWND16)

{
let uVar1: u16; let BVar2: bool;
let local_a: RECT16; let iStack6: i16;

if ((param_3._2_2_ != (s_vrpal_bmp_1050_183a + 0x5)) & & (param_3._2_2_ != (s_vrpal_bmp_1050_183a + 0x4))) {
BVar2 = post_win_msg_1040_7b3c
(CONCAT22(param_2, param_1), param_2._2_2_,
param_3, param_3._2_2_, param_5); return CONCAT22(param_4, BVar2);
}
if (param_3 == 0x7) {
GetWindowRect16(param_5, & local_a); iStack6 -= local_a.x; SetWindowPos16(ctx.s_tile2_bmp_1050_1538,0x2, 0x50, iStack6, 0x0, 0x0,0x0);
}
else {
if ((param_3 != 0x9) & & (param_3 != 0xa)) {
uVar1 = 0x0;
//       TODO: goto LAB_1040_164d;
}
}
uVar1 = 0x1;
//LAB_1040_164d: return uVar1;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn send_msg_1040_1696(param_1: u32, param_2: u16, param_3: u16, param_4: HWND16) {
    let uVar1: u32;
    let uVar2: u32;
    let puVar3: U32Ptr;
    let puVar4: U32Ptr;
    let puVar5: U32Ptr;
    let uVar6: u16;
    let LVar7: LRESULT;
    let paVar8: &mut Struct18;
    let mut pcVar9: String;
    let uVar10: u16;
    let uVar11: u16;
    let uStack18: u16;
    let local_4: u16;

    SendMessage16(param_4, 0x0, 0x0, 0x40b0000);
    LVar7 = SendMessage16(ctx.s_tile2_bmp_1050_1538, 0x0, 0x0, 0xb0000);
    // puVar4 = (LVar7 >> 0x10);
    local_4 = 0x0;
    puVar3 = &local_4;
    // uVar6 = (param_1 >> 0x10);
    pass1_1010_519a((param_1 + 0x8e), CONCAT22(param_3, puVar3), puVar4,
                    param_3);
    puVar5 = puVar4;
    // for (uStack18 = 0x0; uStack18 < local_4; uStack18 += 0x1) {
    //   uVar1 = (puVar3 + uStack18 * 0x2);
    //   uVar10 = 0x0;
    //   uVar11 = 0x403;
    //   uVar2 = (param_1 + 0x8e);
    //   paVar8 =
    //            string_1010_5286(uVar2,(uVar2 >> 0x10),uVar1,
    //                             uVar1,puVar5);
    //   LVar7 = SendMessage16(0x1010,paVar8,(WPARAM16)(paVar8 >> 0x10),
    //                         CONCAT22(uVar11,uVar10));
    //   puVar5 = (LVar7 >> 0x10);
    //   fn_ptr_1000_17ce(paVar8,0x1000);
    // }
    uVar6 = 0x0;
    uVar10 = 0x40a;
    pcVar9 = load_string_1010_847e(ctx.PTR__LOOP_1050_14cc, (ctx.PTR__LOOP_1050_14cc >> 0x10), 0x1010);
    SendMessage16(0x1010, pcVar9, (pcVar9 >> 0x10),
                  CONCAT22(uVar10, uVar6));
    SendMessage16(ctx.s_tile2_bmp_1050_1538, 0x0, 0x0, 0xb0001);
    return;
}


pub fn show_win_1040_18a2(param_1: &mut Struct1, param_2: HWND16, param_3: U32Ptr) {
    let uVar1: u32;
    local_304: u8[0x100];
    local_204: u8[0x100];
    local_104: u8[0x100];
    let uStack4: u16;

    dialog_ui_fn_1040_78e2(param_1, param_2);
    check_dialog_btn_1040_1afe(param_1);
    if (ctx.PTR_LOOP_1050_13ae != 0x0) {
        if (ctx.PTR_LOOP_1050_13ae == &ctx.PTR_LOOP_1050_0002) {
            uStack4 = 0x621;
        } else {
            if (ctx.PTR_LOOP_1050_13ae == (&ctx.PTR_LOOP_1050_0002 + 0x1)) {
                uStack4 = 0x622;
            } else {
                if (ctx.PTR_LOOP_1050_13ae == &DAT_1050_0004) {
                    uStack4 = 0x623;
                } else {
                    uStack4 = 0x620;
                }
            }
        }
        load_string_1010_84e0(0x1010, _PTR_LOOP_1050_14cc,
                              (ctx.PTR__LOOP_1050_14cc >> 0x10), 0x3ff, local_104, (short)param_3,
        );
        load_string_1010_84e0(0x1010, _PTR_LOOP_1050_14cc,
                              (ctx.PTR__LOOP_1050_14cc >> 0x10), 0x3ff, local_204, (short)param_3,
        );
        wsprintf16(0x1010, local_304, param_3);
        SetDlgItemText16(ctx.s_tile2_bmp_1050_1538, local_304, param_3);
        uVar1 = (param_1 + 0x8e);
        if ((uVar1 + 0x82) == 0x0) {
            uStack4 = 0x627;
        } else {
            uStack4 = 0x626;
        }
        load_string_1010_84e0(0x1010, _PTR_LOOP_1050_14cc,
                              (ctx.PTR__LOOP_1050_14cc >> 0x10), 0x3ff, local_104, (short)param_3,
        );
        load_string_1010_84e0(0x1010, _PTR_LOOP_1050_14cc,
                              (ctx.PTR__LOOP_1050_14cc >> 0x10), 0x3ff, local_204, (short)param_3,
        );
        wsprintf16(0x1010, local_304, param_3);
        param_2 = ctx.s_tile2_bmp_1050_1538;
        SetDlgItemText16(ctx.s_tile2_bmp_1050_1538, local_304, param_3);
    }
    move_win_1040_826c(param_1, -0x1, 0xffff);
    ShowWindow16(param_2, 0x5);
    return;
}


pub fn unk_win_ui_op_1040_19ea(param_1: &mut Struct32, param_2: i16, param_3: HWND16) {
    let uVar1: u32;
    let UVar2: u16;
    let in_DX: U32Ptr;
    let iVar4: &mut Struct32;
    let unaff_DI: i16;
    let uVar3: &mut Struct32;
    let unaff_SS: u16;

    iVar4 = param_1;
    // uVar3 = (param_1 >> 0x10);
    if (param_2 != 0x0) {
        UVar2 = IsDlgButtonChecked(param_3, 0xfdb);
        pass1_1010_5d9c(iVar4.field_0x8e, UVar2, in_DX, unaff_DI, unaff_SS);
        UVar2 = IsDlgButtonChecked(0x1010, 0xfdc);
        uVar1 = iVar4.field_0x8e;
        (uVar1 + 0x20) = UVar2;
        UVar2 = IsDlgButtonChecked(ctx.s_tile2_bmp_1050_1538, 0xfdd);
        uVar1 = iVar4.field_0x8e;
        (uVar1 + 0x74) = UVar2;
        param_3 = ctx.s_tile2_bmp_1050_1538;
        UVar2 = IsDlgButtonChecked(ctx.s_tile2_bmp_1050_1538, 0xfde);
        uVar1 = iVar4.field_0x8e;
        (uVar1 + 0x72) = UVar2;
        if (iVar4.field_0x92 != 0x0) {
            uVar1 = iVar4.field_0x8e;
            param_3 = 0x1000;
            pass1_1000_4906((uVar1 & 0xffff0000 | (uVar1 + 0x22)),
                            0x0, 0x40);
        }
        if (iVar4.field_0x94 != 0x0) {
            param_3 = 0x1010;
            pass1_1010_60a0(iVar4.field_0x8e);
        }
    }
    DestroyWindow16(param_3);
    return;
}


pub fn check_dialog_btn_1040_1afe(param_1: u32) {
    let id: i16;
    let id_00: i16;
    let id_01: i16;
    let uVar1: u32;
    let uVar2: u32;
    let iVar3: i16;
    let uVar4: u16;
    let unaff_CS: HWND16;

    // uVar4 = (param_1 >> 0x10);
    iVar3 = param_1;
    uVar1 = (iVar3 + 0x8e);
    uVar2 = (iVar3 + 0x8e);
    id = (uVar2 + 0x20);
    uVar2 = (iVar3 + 0x8e);
    id_00 = (uVar2 + 0x74);
    uVar2 = (iVar3 + 0x8e);
    id_01 = (uVar2 + 0x72);
    CheckDlgButton16(unaff_CS, (uVar1 + 0x1e), 0xfdb);
    CheckDlgButton16(ctx.s_tile2_bmp_1050_1538, id_00, 0xfdd);
    CheckDlgButton16(ctx.s_tile2_bmp_1050_1538, id_01, 0xfde);
    CheckDlgButton16(ctx.s_tile2_bmp_1050_1538, id, 0xfdc);
    return;
}


pub fn check_dialog_btn_1040_1b8a() {
    let id: u16;
    let id_00: u16;
    let id_01: u16;
    let id_02: u16;

    id = pass1_1010_60b4();
    id_00 = pass1_1010_60c6();
    id_01 = pass1_1010_60c0();
    id_02 = pass1_1010_60ba();
    CheckDlgButton16(0x1010, id, 0xfdb);
    CheckDlgButton16(ctx.s_tile2_bmp_1050_1538, id_01, 0xfdd);
    CheckDlgButton16(ctx.s_tile2_bmp_1050_1538, id_02, 0xfde);
    CheckDlgButton16(ctx.s_tile2_bmp_1050_1538, id_00, 0xfdc);
    return;
}


pub fn show_win_1040_1d50(param_1: &mut Struct1, param_2: HWND16) {
    dialog_ui_fn_1040_78e2(param_1, param_2);
    move_win_1040_826c(param_1, -0x1, 0xffff);
    ShowWindow16(param_2, 0x5);
    return;
}


pub fn unk_win_ui_op_1040_1d7a(param_1: &mut Struct33, param_2: i16, param_3: HWND16) {
    let uVar1: u32;
    let UVar2: u16;
    let iVar3: &mut Struct33;
    let uVar3: &mut Struct33;
    let HVar3: HWND16;
    let HVar4: HWND16;
    let unaff_SS: u16;

    iVar3 = param_1;
    // uVar3 = (param_1 >> 0x10);
    if ((param_2 != 0x0) && (uVar1 = iVar3.field_0x8e, (uVar1 + 0x72) != 0x0)) {
        HVar3 = ctx.s_tile2_bmp_1050_1538;
        UVar2 = IsDlgButtonChecked(param_3, 0xe1);
        if (UVar2 != 0x0) {
            HVar3 = 0x1008;
            pass1_1008_a930(iVar3.field_0x92, 0x1d5, unaff_SS);
        }
        HVar4 = ctx.s_tile2_bmp_1050_1538;
        UVar2 = IsDlgButtonChecked(HVar3, 0xe2);
        if (UVar2 != 0x0) {
            HVar4 = 0x1008;
            pass1_1008_a930(iVar3.field_0x92, 0x1d6, unaff_SS);
        }
        HVar3 = ctx.s_tile2_bmp_1050_1538;
        UVar2 = IsDlgButtonChecked(HVar4, 0xe3);
        if (UVar2 != 0x0) {
            HVar3 = 0x1008;
            pass1_1008_a930(iVar3.field_0x92, 0x1d7, unaff_SS);
        }
        HVar4 = ctx.s_tile2_bmp_1050_1538;
        UVar2 = IsDlgButtonChecked(HVar3, 0xe5);
        if (UVar2 != 0x0) {
            HVar4 = 0x1008;
            pass1_1008_a930(iVar3.field_0x92, 0x1d8, unaff_SS);
        }
        HVar3 = ctx.s_tile2_bmp_1050_1538;
        UVar2 = IsDlgButtonChecked(HVar4, 0xe6);
        if (UVar2 != 0x0) {
            HVar3 = 0x1008;
            pass1_1008_a930(iVar3.field_0x92, 0x1e2, unaff_SS);
        }
        UVar2 = IsDlgButtonChecked(HVar3, 0xe7);
        if (UVar2 != 0x0) {
            pass1_1008_a930(iVar3.field_0x92, 0x1dc, unaff_SS);
        }
        return;
    }
    DestroyWindow16(param_3);
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn create_win_1040_20d4(param_1: u16, param_2: u16, param_3: u16, param_4: &mut Struct1) {
    let y: i16;
    let unaff_DI: i16;
    let uVar1: u16;
    let puVar2: U32Ptr;
    let local_1e: RECT16;
    let iStack26: i16;
    let iStack24: i16;
    let uStack22: u32;
    let uStack18: u32;
    let iStack14: i16;
    let uStack12: u16;
    let uStack10: i16;
    let iStack8: i16;
    let uStack6: u16;
    let iStack4: i16;

    dialog_ui_fn_1040_78e2(param_4, param_2);
    puVar2 = mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0, 0x48, param_1, param_3, unaff_DI);
    // uStack12 = (puVar2 >> 0x10);
    iStack14 = puVar2;
    iStack8 = (iStack14 + 0xa);
    iStack10 = (iStack14 + 0xc);
    // uVar1 = (param_4 >> 0x10);
    uStack18 = pass1_1008_4772((param_4 + 0x8e));
    y = (uStack18 + 0x4);
    iStack4 = (iStack8 - y) / 0x2;
    uStack6 = 0x5;
    SetWindowPos16(0x1008, 0x6, 0x1d6, y, 0x5, iStack4, 0x0);
    GetClientRect16(ctx.s_tile2_bmp_1050_1538, &local_1e);
    load_string_1010_847e(ctx.PTR__LOOP_1050_14cc, (ctx.PTR__LOOP_1050_14cc >> 0x10), 0x1010);
    uStack22 = 0x50010001;
    CreateWindow16(0x1010, 0x0, ZEXT24(ctx.PTR_LOOP_1050_038c) << 0x10, 0x1,
                   (param_4 + 0x6), 0x19, 0x58, iStack24 - 0x28,
                   (iStack26 + -0x58) / 0x2, 0x1, (s_Rebel_1050_4ffc + 0x5));
    SetWindowPos16(ctx.s_tile2_bmp_1050_1538, 0x45, iStack10 + -0xa,
                   (uStack18 + 0x4), 0x5, iStack4, 0x0);
    return;
}


pub fn show_win_1040_2490(param_1: &mut Struct1, param_2: HWND16) {
    let ppcVar1: u32;
    let uVar2: u16;
    let iVar4: &mut Struct1;
    let uVar3: u16;
    let piVar4: U32Ptr;

    dialog_ui_fn_1040_78e2(param_1, param_2);
    // uVar3 = (param_1 >> 0x10);
    iVar4 = param_1;
    GetDlgItem16(param_2, 0xfb1);
    EnableWindow16(ctx.s_tile2_bmp_1050_1538, 0x0);
    // WARNING: Load size is inaccurate
    ppcVar1 = (*iVar4.field_0x8e + 0x10);
    piVar4 = (**ppcVar1)(ctx.s_tile2_bmp_1050_1538, &iVar4.field_0x8e,
    );
    // uVar2 = (piVar4 >> 0x10);
    move_win_1040_826c(param_1, (piVar4 + 0x2) + -0x2,
                       (piVar4 + 0x4) + *piVar4 + 0x3);
    ShowWindow16(ctx.s_tile2_bmp_1050_1538, 0x5);
    pass1_1018_1c9a(&iVar4.field_0x8e, 0x1a0);
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

u32
win_ui_op_1040_2512(param_1: * mut u32,param_2: u32,param_3: u16,param_4: HWND16,
param_5: & WNDCLASS16,param_6: * mut u8)

{
let piVar1: * mut i16; let ppcVar2: u32; let uVar3: u16; let Bvar4: bool; let iVar5: i16; let iVar6: i16; let UVar7: u16; let puVar8: * mut u8; let unaff_DI: i16; let uVar9: u16; let uVar10: u16; let hwnd: HWND16; let in_AF: u8; let uVar11: u32; let local_1e: [u8;4]; let uStack26: u16; let puStack24: * mut u8; local_16: * mut u16 [0x2]; let uStack12: u16; let puStack10: u32; let BStack6: bool; let puStack4: * mut u8;

BStack6 = 0x0;
puStack4 = 0x0; if (param_3 == 0x2) {
//LAB_1040_266d:
BStack6 = 0x1; puStack4 = 0x0;
}
else {
// uVar9 = (param_1 >> 0x10); if (0x19d < param_3 - 0x2) {
iVar5 = param_1; if (param_3 - 0x1a0 < 0x14 | | param_3 == 0x1b4) {
UVar7 = IsDlgButtonChecked(param_4, param_3); if (UVar7 == 0x0) {
piVar1 = (iVar5 + 0x92); * piVar1 = * piVar1 + 0x1; if (0x0 < (iVar5 + 0x92)) {
(iVar5 + 0x94) = 0x0;
}
uVar11 = (iVar5 + 0x8e); if ((uVar11 + 0x28) == (iVar5 + 0x92)) {
GetDlgItem16(ctx.s_tile2_bmp_1050_1538, 0xfb1); EnableWindow16(ctx.s_tile2_bmp_1050_1538, 0x0);
}
}
else {
piVar1 = (iVar5 + 0x92); * piVar1 = * piVar1 + - 0x1; GetDlgItem16(ctx.s_tile2_bmp_1050_1538, 0xfb1); BVar4 = IsWindowEnabled16(ctx.s_tile2_bmp_1050_1538); if (BVar4 == 0x0) {
GetDlgItem16(ctx.s_tile2_bmp_1050_1538, 0xfb1); EnableWindow16(ctx.s_tile2_bmp_1050_1538, 0x1);
}
if ((iVar5 + 0x92) < 0x1) {
(iVar5 + 0x94) = 0x1;
}
pass1_1018_1c9a((iVar5 + 0x8e), param_3); puStack10 = pass1_1018_1e78((iVar5 + 0x8e), - 0x1);
// uVar3 = (puStack10 >> 0x10); if (puStack10 == 0x0) {
uStack12 = 0x0;
}
else {
uStack12 = (puStack10 + 0x1c);
}
win_1008_5c7c(ctx.PTR__LOOP_1050_02a0, CONCAT22(uStack12,0x1), param_5, uStack12,
uVar3 | puStack10);
}
if (( - 0x1 < (iVar5 + 0x92)) & & (uVar11 = (iVar5 + 0x8e),
(iVar5 + 0x92) < = (uVar11 + 0x28))) {
sys_1000_3f9c(local_16, param_5, 0x5cf4,
ctx.data_seg, (iVar5 + 0x92), & stack0xfffe,
uVar9, 0x1000, param_5, in_AF);
SetDlgItemText16(0x1000, local_16, param_5);
}
//         TODO: goto LAB_1040_266d;
}
uVar3 = param_3 - 0xfb1; if (uVar3 == 0x0) {
if ((iVar5 + 0x92) < 0x0) {
mem_op_1000_179c(0xb4, param_6, 0x1000); puVar8 = (param_6 | uVar3); uStack26 = uVar3; puStack24 = param_6; if (puVar8 == 0x0) {
iVar5 = 0x0; puVar8 = 0x0;
}
else {
iVar5 = string_1040_8520(CONCAT22(param_6, uVar3),
ctx.PTR_LOOP_1050_0396, 0x30, 0x2, 0x57b, 0x57c,
puVar8, param_5);
}
puStack10 = CONCAT22(puVar8, iVar5); ppcVar2 = ( * puStack10 + 0x74); ( ** ppcVar2)(0x1000, iVar5, puVar8);
//           TODO: goto LAB_1040_27c0;
}
if (0x0 < (iVar5 + 0x92)) {
mem_op_1000_179c(0xb4, param_6, 0x1000); puVar8 = (param_6 | uVar3); uStack26 = uVar3; puStack24 = param_6; if (puVar8 == 0x0) {
iVar6 = 0x0; puVar8 = 0x0;
}
else {
iVar6 = string_1040_8520(CONCAT22(param_6, uVar3),
ctx.PTR_LOOP_1050_0396, 0x21, 0x2, 0x57b, 0x57d,
puVar8, param_5);
}
puStack10 = CONCAT22(puVar8, iVar6); pass1_1008_941a(CONCAT22(param_5, local_1e), 0x1,0xc2); ppcVar2 = ( * puStack10 + 0x6c); uVar11 = ( * * ppcVar2)(0x1008, puStack10, (puStack10 > > 0x10),
local_1e, param_5);
// param_6 = (uVar11 >> 0x10); if (uVar11 == 0x2) {
// goto LAB_1040_27c0;
}
}
local_16[0] = mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0, 0x6, param_5, param_6,
unaff_DI);
param_6 = (local_16[0] > > 0x10); uStack12 = 0x1a0; hwnd = 0x1010;
loop {
UVar7 = IsDlgButtonChecked(hwnd, uStack12); if (UVar7 == 0x1) {
uVar10 = (local_16[0] > > 0x10); iVar6 = local_16[0]; (iVar6 + (iVar6 + 0x56) * 0x2 + 0x4e) = uStack12; piVar1 = (iVar6 + 0x56); * piVar1 = * piVar1 + 0x1;
}
uStack12 += 0x1; hwnd = ctx.s_tile2_bmp_1050_1538;
if (uStack12 < 0x1b5) == false { break; }
}
uVar3 = (iVar5 + 0x92); puStack10 = (puStack10 & 0xffff0000 | uVar3); uVar11 = (iVar5 + 0x8e); (uVar11 + 0x28) = uVar3; param_4 = ctx.s_tile2_bmp_1050_1538;
PostMessage16(ctx.s_tile2_bmp_1050_1538, 0x0, 0x0,0x11100c8); param_3 = 0x1;
}
}
BStack6 = post_win_msg_1040_7b3c
(param_1, param_2, (param_2 > > 0x10), param_3, param_4
);
puStack4 = param_6;
}
//LAB_1040_27c0: return CONCAT22(puStack4, BStack6);
}


pub fn dlg_ui_op_1040_2a64(param_1: &mut Struct1, param_2: u16, param_3: u16) {
    let uVar1: u32;
    let paVar2: &mut Struct160;
    let uVar3: u16;
    let puVar4: U32Ptr;
    let puVar5: U32Ptr;
    let iVar6: i16;
    let uVar7: u16;
    let hwnd: HWND16;
    let hwnd_00: HWND16;
    let iVar8: i16;
    let local_16: RECT16;
    let uStack18: u16;
    let uStack16: u16;
    let iStack14: i16;
    let uStack12: u32;
    let uStack8: u32;
    let iStack4: i16;

    unk_win_ui_op_1040_b230(param_1, param_2, param_3);
    iStack4 = 0x5;
    iVar8 = 0x0;
    // uVar7 = (param_1 >> 0x10);
    iVar6 = param_1;
    uVar1 = (iVar6 + 0x90);
    uStack12 = struct_op_1030_73a8((uVar1 + 0x6));
    // puVar4 = (uStack12 >> 0x10);
    hwnd = &USHORT_1050_1028;
    ctx.PTR_LOOP_1050_5d04 = pass1_1028_4a9a(uStack12, iVar8);
    // for (iStack14 = 0x0; iStack14 < iStack4; iStack14 += 0x1) {
    //   if (iStack14 != 0x0) {
    //     (&ctx.PTR_LOOP_1050_5d04 + iStack14 * 0xc) = 0x0;
    //   }
    //   iVar8 = iStack14 * 0xc;
    //   local_16.x = (iVar8 + 0x5cfc);
    //   local_16.y = (iVar8 + 0x5cfe);
    //   paVar2 = (&ctx.PTR_LOOP_1050_0000 + 0x1);
    //   uStack18 = 0x1;
    //   uStack16 = 0x1;
    //   MapDialogRect16(hwnd,&local_16);
    //   hwnd_00 = 0x1000;
    //   mem_op_1000_179c(0x42,puVar4,0x1000);
    //   puVar5 = (puVar4 | paVar2);
    //   if (puVar5 == 0x0) {
    //     paVar2 = 0x0;
    //     puVar4 = 0x0;
    //   }
    //   else {
    //     hwnd_00 = 0x1008;
    //     pass1_1008_3bd6(paVar2,puVar4,0x1,CONCAT22(local_16.x,local_16.y),0x101,
    //                     0xff0100,CONCAT22((iVar6 + 0x6),
    //                                       (iVar8 + 0x5d00)),puVar5,
    //                     param_3);
    //     puVar4 = puVar5;
    //   }
    //   uStack8 = CONCAT22(puVar4,paVar2);
    //   if (ctx.PTR_LOOP_1050_5d04 == 0x0) {
    //     hwnd = hwnd_00;
    //     if ((iStack14 != 0x0) && ((puVar4 | paVar2) != 0x0)) {
    //       hwnd = ctx.s_tile2_bmp_1050_1538;
    //       EnableWindow16(hwnd_00,0x0);
    //     }
    //   }
    //   else {
    //     iVar8 = iStack14 * 0xc;
    //     uVar3 = pass1_1028_4a9a(uStack12,(iVar8 + 0x5d02));
    //     hwnd = &USHORT_1050_1028;
    //     if (uVar3 != 0x0) {
    //       (&ctx.PTR_LOOP_1050_5d04 + iVar8) = 0x1;
    //       uVar1 = (iVar6 + 0x98);
    //       SetDlgItemText16(&USHORT_1050_1028,uVar1,
    //                        (uVar1 >> 0x10));
    //       hwnd = ctx.s_tile2_bmp_1050_1538;
    //     }
    //   }
    // }
    return;
}


pub fn win_ui_op_1040_2bb2(param_1: i16, param_2: u16, param_3: u16, param_4: u32, param_5: HWND16,
)

{
    let uVar1: u16;
    let in_DX: U32Ptr;
    let unaff_SS: u16;
    let uVar2: u32;
    let iStack8: i16;
    let iStack4: i16;

    if (param_4._2_2_ == 0x158) {
        ctx.PTR_LOOP_1050_5d04 = (ctx.PTR_LOOP_1050_5d04 == 0x0);
        if (ctx.PTR_LOOP_1050_5d04 == 0x0) {
            // for (iStack8 = 0x1; iStack8 < 0x5; iStack8 += 0x1) {
            //   GetDlgItem16(param_5,(iStack8 * 0xc + 0x5d00));
            //   EnableWindow16(ctx.s_tile2_bmp_1050_1538,0x0);
            //   (&ctx.PTR_LOOP_1050_5d04 + iStack8 * 0xc) = 0x0;
            //   uVar2 = (param_1 + 0x94);
            //   param_5 = ctx.s_tile2_bmp_1050_1538;
            //   SetDlgItemText16(ctx.s_tile2_bmp_1050_1538,uVar2,
            //                    (uVar2 >> 0x10));
            // }
            uVar2 = (param_1 + 0x94);
//       TODO: goto LAB_1040_2ccc;
        }
        // for (iStack8 = 0x1; iStack8 < 0x5; iStack8 += 0x1) {
        //   GetDlgItem16(param_5,(iStack8 * 0xc + 0x5d00));
        //   EnableWindow16(ctx.s_tile2_bmp_1050_1538,0x1);
        //   (&ctx.PTR_LOOP_1050_5d04 + iStack8 * 0xc) = 0x0;
        //   uVar2 = (param_1 + 0x94);
        //   param_5 = ctx.s_tile2_bmp_1050_1538;
        //   SetDlgItemText16(ctx.s_tile2_bmp_1050_1538,uVar2,
        //                    (uVar2 >> 0x10));
        // }
    } else {
        if (param_4._2_2_ == 0x159) {
            iStack4 = 0x1;
        } else {
            if (param_4._2_2_ == 0x15a) {
                iStack4 = 0x2;
            } else {
                if (param_4._2_2_ == 0x15b) {
                    iStack4 = 0x3;
                } else {
                    if (param_4._2_2_ != 0x15c) {
                        pass1_1040_b54a(param_1, param_2, param_3, param_4, in_DX, param_5, unaff_SS);
                        return;
                    }
                    iStack4 = 0x4;
                }
            }
        }
        if (iStack4 == 0x0) {
            return;
        }
        uVar1 = ((&ctx.PTR_LOOP_1050_5d04 + iStack4 * 0xc) == 0x0);
        (&ctx.PTR_LOOP_1050_5d04 + iStack4 * 0xc) = uVar1;
        if (uVar1 == 0x0) {
            uVar2 = (param_1 + 0x94);
//       TODO: goto LAB_1040_2ccc;
        }
    }
    uVar2 = (param_1 + 0x98);
//LAB_1040_2ccc:
    SetDlgItemText16(param_5, uVar2, (uVar2 >> 0x10));
    return;
}


pub fn win_dlg_item_1040_2d48(param_1: u32, param_2: HWND16, param_3: bool) {
    let UVar1: u16;
    let value: u16;
    let local_4: bool;

    pass1_1040_b45e(param_1, param_2);
    UVar1 = GetDlgItemInt16(param_2, 0x1, &local_4, param_3);
    value = GetDlgItemInt16(ctx.s_tile2_bmp_1050_1538, 0x1, &local_4, param_3);
    if (UVar1 != 0x0) {
        value /= UVar1;
    }
    SetDlgItemInt16(ctx.s_tile2_bmp_1050_1538, 0x1, value, 0x165);
    return;
}


pub fn show_win_1040_2f5a(param_1: &mut Struct1, param_2: HWND16) {
    dialog_ui_fn_1040_78e2(param_1, param_2);
    move_win_1040_826c(param_1, -0x1, 0xffff);
    ShowWindow16(param_2, 0x5);
    SetFocus16(ctx.s_tile2_bmp_1050_1538);
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn win_dlg_op_1040_2f90(param_1: u32, param_2: U32Ptr) {
    let HVar1: HWND16;
    let in_DX: U32Ptr;
    let puVar2: U32Ptr;
    let uVar3: u16;
    let msg: u16;
    let iVar4: i16;
    let unaff_DI: i16;
    let uVar5: u16;
    let puVar6: U32Ptr;
    let uVar7: u32;
    let mut pcVar8: String;
    let local_116: u32;
    let local_112: u32;
    local_10e: u8[0x82];
    let local_8c: [u8; 82];
    let uStack10: u32;
    let puStack6: U32Ptr;

    puStack6 = mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0, 0x2, param_2, in_DX, unaff_DI);
    // puVar2 = (puStack6 >> 0x10);
    uStack10 = (puStack6 + 0x68);
    // uVar5 = (param_1 >> 0x10);
    iVar4 = param_1;
    GetWindowText16(0x1010, 0x80, local_8c);
    wsprintf16(s_tile2_bmp_1050_1538, local_10e, param_2);
    SetWindowText16(ctx.s_tile2_bmp_1050_1538, local_10e);
    HVar1 = GetDlgItem16(ctx.s_tile2_bmp_1050_1538, 0x182);
    (iVar4 + 0x92) = HVar1;
    pass1_1018_3a94((iVar4 + 0x96), CONCAT22(param_2, &local_116),
                    CONCAT22(param_2, &local_112), param_2);
    send_msg_1040_3374(param_1, local_112, (iVar4 + 0x92), 0x1018);
    puVar6 = mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0, 0x2f, param_2, puVar2, unaff_DI);
    // uVar3 = (puVar6 >> 0x10);
    uVar7 = (puVar6 + 0x24);
    uVar7 = pass1_1018_3a7a((iVar4 + 0x96), uVar7, uVar7, uVar3);
    SendMessage16(0x1018, uVar7, (uVar7 >> 0x10), 0x40dffff);
    HVar1 = GetDlgItem16(ctx.s_tile2_bmp_1050_1538, 0x183);
    (iVar4 + 0x94) = HVar1;
    send_msg_1040_3374(param_1, local_116, HVar1, s_tile2_bmp_1050_1538);
    pcVar8 = load_string_1010_847e(ctx.PTR__LOOP_1050_14cc, (ctx.PTR__LOOP_1050_14cc >> 0x10), 0x1010);
    // msg = (pcVar8 >> 0x10);
    SendDlgItemMessage16(0x1010, pcVar8, msg, 0x0, 0x1830403);
    SendDlgItemMessage16(ctx.s_tile2_bmp_1050_1538, pcVar8, msg, 0xffff, 0x183040d);
    HVar1 = GetDlgItem16(ctx.s_tile2_bmp_1050_1538, 0x181);
    (iVar4 + 0x8e) = HVar1;
    HVar1 = GetDlgItem16(ctx.s_tile2_bmp_1050_1538, 0x184);
    (iVar4 + 0x90) = HVar1;
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn win_ui_op_1040_311a(param_1: i16, param_2: u16, param_3: u16, param_4: u32) {
    let i_var1: i16;
    let uVar2: u32;
    let puVar3: U32Ptr;
    let unaff_CS: u16;
    let unaff_SS: u16;
    let LVar4: LRESULT;
    let puVar5: U32Ptr;
    let iVar6: i16;

    send_msg_1040_323c(CONCAT22(param_2, param_1), unaff_CS);
    load_string_1010_847e(ctx.PTR__LOOP_1050_14cc, (ctx.PTR__LOOP_1050_14cc >> 0x10), 0x1010);
    if (param_4._2_2_ == 0x181) {
        i_var1 = param_1 + 0x9a;
        puVar3 = param_2;
        iVar6 = i_var1;
        pass1_1018_3cda((param_1 + 0x96), CONCAT22(param_2, param_1 + 0x19a),
                        CONCAT22(param_2, i_var1));
        pass1_1018_3424((param_1 + 0x96), iVar6, puVar3, unaff_SS);
        if (iVar6 == 0x0) {
            iVar6 = 0x21;
        } else {
            pass1_1018_3a42((param_1 + 0x96), CONCAT22(param_2, i_var1), puVar3, unaff_SS);
            pass1_1030_8344(
                ctx, ctx.PTR__LOOP_1050_5748, CONCAT22(puVar3, iVar6));
            uVar2 = (iVar6 + 0x10);
            pass1_1030_8344(
                ctx, ctx.PTR__LOOP_1050_5748, uVar2);
            ctx.PTR_LOOP_1050_5f0c = uVar2;
            ctx.PTR_LOOP_1050_5f10 = (&ctx.PTR_LOOP_1050_0000 + 0x1);
            iVar6 = 0x25;
            ctx.PTR_LOOP_1050_5f0e = puVar3;
        }
        pass1_1038_af40(ctx.PTR__LOOP_1050_5b7c, (param_1 + 0x8), iVar6, puVar3,
                        param_1, &ctx.PTR_LOOP_1050_1038, unaff_SS);
        LVar4 = SendMessage16(&ctx.PTR_LOOP_1050_1038, 0x0, 0x0, 0x1110002);
        iVar6 = 0x1;
        puVar5 = mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0, 0x2b, unaff_SS,
                                 (LVar4 >> 0x10), param_2);
        pass1_1010_038e(puVar5, iVar6, unaff_SS);
    } else {
        if ((param_4._2_2_ == 0x181) || (0x1 < param_4._2_2_ - 0x182)) {
            post_win_msg_1040_7b3c(CONCAT22(param_2, param_1), param_3, param_4, param_4._2_2_,
                                   0x1010);
            return;
        }
        set_win_pos_1040_331a(CONCAT22(param_2, param_1), param_3, param_4, 0x1010);
    }
    return;
}



LRESULT  send_msg_1040_323c(param_1: u32,param_2: HWND16)

{
wparam: WPARAM16; let LVar1: LRESULT; let LVar2: LRESULT;

// wparam = (param_1 >> 0x10);
LVar1 = SendMessage16(param_2,0x0, 0x0, 0x4070000); LVar2 = SendMessage16(ctx.s_tile2_bmp_1050_1538, 0x0, 0x0, 0x4070000); SendMessage16(ctx.s_tile2_bmp_1050_1538, param_1 + 0x9a, wparam,
CONCAT22(0x408, LVar1)); LVar1 = SendMessage16(ctx.s_tile2_bmp_1050_1538,param_1 + 0x19a, wparam,
CONCAT22(0x408,LVar2)); return LVar1;
}



pub fn enable_win_1040_32a8(param_1: u32) {
    lp_string: SEGPTR;
    let BVar1: bool;
    let unaff_SS: u16;
    let uStack12: u32;

    lp_string = param_1 + 0x19a;
    uStack12 = param_1 & 0xffff0000 | lp_string;
    pass1_1018_3a5c((param_1 + 0x96),
                    param_1 & 0xffff0000 | (param_1 + 0x9a),
                    param_1 & 0xffff0000 | lp_string, unaff_SS);
    SetWindowText16(0x1018, lp_string);
    BVar1 = string_1018_39d8(unaff_SS, (param_1 + 0x96),
                             param_1 & 0xffff0000 | (param_1 + 0x9a), uStack12);
    EnableWindow16(0x1018, BVar1 & 0x1);
    return;
}


pub fn set_win_pos_1040_331a(param_1: u32, param_2: u16, param_3: i16, param_4: HWND16) -> bool

{
    let local_e: RECT16;
    let uStack10: i16;
    let uStack6: u16;
    let iStack4: i16;

    iStack4 = param_3;
    uStack6 = param_2;
    if (param_3 == 0x1) {
        enable_win_1040_32a8(param_1);
    } else {
        if (param_3 != 0x7) {
            return 0x0;
        }
        GetWindowRect16(param_4, &local_e);
        iStack10 -= local_e.x;
        SetWindowPos16(ctx.s_tile2_bmp_1050_1538, 0x2, 0x50, iStack10, 0x0, 0x0, 0x0);
    }
    return 0x1;
}


pub fn send_msg_1040_3374(param_1: u32, param_2: U32Ptr, param_3: u16, param_4: HWND16) {
    let ppcVar1: u32;
    let uVar2: u16;
    let uVar3: u32;
    let extraout_DX: u16;
    let extraout_DX_00: u16;
    let uVar4: u16;
    let uVar5: u16;
    let LVar6: LRESULT;
    let paVar7: &mut Struct18;
    let uVar8: u16;
    let uStack10: u32;
    let uStack6: u32;

    uVar5 = SUB42(ctx.s_tile2_bmp_1050_1538, 0x0);
    uVar8 = param_3;
    LVar6 = SendMessage16(param_4, 0x0, 0x0, 0x40b0000);
    uVar2 = LVar6;
    uVar4 = param_2;
    ppcVar1 = (*param_2 + 0x10);
    (**ppcVar1)(ctx.s_tile2_bmp_1050_1538, param_2, uVar8);
    uStack6 = CONCAT22(extraout_DX, uVar2);
    uStack10 = 0x0;
    loop {
        if (uStack6 <= uStack10) {
            return;
        }
        ppcVar1 = (*param_2 + 0x4);
        uVar3 = uStack6;
        (**ppcVar1)(uVar5, param_2, uStack10, (uStack10 >> 0x10), uVar4);
        paVar7 = pass1_1018_3a7a((param_1 + 0x96),
                                 uVar3 & 0xffff | extraout_DX_00 << 0x10, uVar3,
                                 extraout_DX_00);
        uVar4 = param_3;
        LVar6 = SendMessage16(0x1018, paVar7, (paVar7 >> 0x10),
                              0x4030000);
        uVar5 = 0x1000;
        fn_ptr_1000_17ce(ctx, paVar7, 0x1000);
        if (LVar6 == -0x1) { break; }
        if (LVar6 == -0x2) {
            return;
        }
        uStack10 += 0x1;
    }
    return;
}


pub fn show_win_1040_355a(param_1: &mut Struct1, param_2: HWND16) {
    dialog_ui_fn_1040_78e2(param_1, param_2);
    move_win_1040_826c(param_1, -0x1, 0xffff);
    ShowWindow16(param_2, 0x5);
    SetFocus16(ctx.s_tile2_bmp_1050_1538);
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn set_win_text_1040_3590(param_1: u32, param_2: U32Ptr) {
    let HVar1: HWND16;
    lp_string: SEGPTR;
    let lp_string_00: u16;
    let in_DX: U32Ptr;
    let uVar2: u16;
    let iVar3: i16;
    let unaff_DI: i16;
    let uVar4: u16;
    let local_59a: [u8; 4];
    let local_596: [u8; 4];
    let BStack1426: bool;
    let uStack1424: u16;
    local_58e: u8[0x82];
    local_50c: u8[0x100];
    let uStack1036: u32;
    let puStack1032: U32Ptr;
    local_404: u8[0x402];

    puStack1032 = mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0, 0x2, param_2, in_DX, unaff_DI);
    // uVar2 = (puStack1032 >> 0x10);
    uStack1036 = (puStack1032 + 0x68);
    // uVar4 = (param_1 >> 0x10);
    iVar3 = param_1;
    GetWindowText16(0x1010, 0x80, local_50c);
    wsprintf16(s_tile2_bmp_1050_1538, local_58e, param_2);
    BStack1426 = SetWindowText16(ctx.s_tile2_bmp_1050_1538, local_58e);
    sprintf_op_1018_34b6((iVar3 + 0x8e), (uchar)BStack1426);
    uStack1424 = uVar2;
    pass1_1018_3d44((iVar3 + 0x8e), CONCAT22(param_2, local_59a),
                    CONCAT22(param_2, local_596));
    HVar1 = GetDlgItem16(0x1018, 0x193);
    (iVar3 + 0x98) = HVar1;
    EnableWindow16(ctx.s_tile2_bmp_1050_1538, 0x1);
    load_string_1010_84e0(0x1010, _PTR_LOOP_1050_14cc,
                          (ctx.PTR__LOOP_1050_14cc >> 0x10), 0x3ff, local_404, (short)param_2);
    wsprintf16(0x1010, local_50c, param_2);
    GetDlgItem16(ctx.s_tile2_bmp_1050_1538, 0x195);
    SetWindowText16(ctx.s_tile2_bmp_1050_1538, local_50c);
    lp_string = GetDlgItem16(ctx.s_tile2_bmp_1050_1538, 0x196);
    sprintf_op_1018_34b6((iVar3 + 0x8e), (uchar)lp_string);
    SetWindowText16(0x1018, lp_string);
    GetDlgItem16(ctx.s_tile2_bmp_1050_1538, 0x197);
    load_string_1010_84e0(0x1010, _PTR_LOOP_1050_14cc,
                          (ctx.PTR__LOOP_1050_14cc >> 0x10), 0x3ff, local_404, (short)param_2);
    SetWindowText16(0x1010, local_404);
    load_string_1010_84e0(0x1010, _PTR_LOOP_1050_14cc,
                          (ctx.PTR__LOOP_1050_14cc >> 0x10), 0x3ff, local_404, (short)param_2);
    wsprintf16(0x1010, local_50c, param_2);
    GetDlgItem16(ctx.s_tile2_bmp_1050_1538, 0x198);
    SetWindowText16(ctx.s_tile2_bmp_1050_1538, local_50c);
    lp_string_00 = GetDlgItem16(ctx.s_tile2_bmp_1050_1538, 0x199);
    unk_str_op_1018_35b0((iVar3 + 0x8e), param_2, lp_string_00);
    if ((uVar2 | lp_string_00) == 0x0) {
        load_string_1010_84e0(0x1010, _PTR_LOOP_1050_14cc,
                              (ctx.PTR__LOOP_1050_14cc >> 0x10), 0x3ff, local_404, (short)param_2,
        );
        SetWindowText16(0x1010, local_404);
        GetDlgItem16(ctx.s_tile2_bmp_1050_1538, 0x19a);
        load_string_1010_84e0(0x1010, _PTR_LOOP_1050_14cc,
                              (ctx.PTR__LOOP_1050_14cc >> 0x10), 0x3ff, local_404, (short)param_2,
        );
        SetWindowText16(0x1010, local_404);
        EnableWindow16(ctx.s_tile2_bmp_1050_1538, 0x0);
        return;
    }
    SetWindowText16(0x1018, lp_string_00);
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn message_box_op_1040_37f0(param_1: i16, param_2: u16, param_3: u16, param_4: u32, param_5: u16,
                                param_6: u16)

{
    let uVar1: u16;
    let in_DX: U32Ptr;
    let uVar2: u16;
    let unaff_DI: i16;
    let LVar3: LRESULT;
    let iVar4: i16;
    local_40c: u8[0x402];
    let uStack10: u32;
    let puStack6: U32Ptr;

    if (param_4._2_2_ == 0x193) {
        puStack6 = mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0, 0x2, param_6, in_DX, unaff_DI);
        // uVar2 = (puStack6 >> 0x10);
        uStack10 = (puStack6 + 0x68);
        load_string_1010_84e0(0x1010, _PTR_LOOP_1050_14cc, 0x3ff, local_40c, param_6);
        uVar1 = MessageBox16(0x1010, (s_New_failed_in_Op__Op_1050_0020 + 0x10),
                             uStack10, (uStack10 >> 0x10));
        pass1_1018_3710((param_1 + 0x8e), param_6, uVar1, uVar2);
        PostMessage16(0x1018, 0x0, 0x0, 0x1110002);
    } else {
        if (param_4._2_2_ != 0x194) {
            post_win_msg_1040_7b3c(CONCAT22(param_2, param_1), param_3, param_4, param_4._2_2_,
                                   param_5);
            return;
        }
        pass1_1038_af40(ctx.PTR__LOOP_1050_5b7c, (param_1 + 0x8), 0x21, in_DX,
                        param_1, &ctx.PTR_LOOP_1050_1038, param_6);
        LVar3 = SendMessage16(&ctx.PTR_LOOP_1050_1038, 0x0, 0x0, 0x1110002);
        iVar4 = 0x1;
        puStack6 = mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0, 0x2b, param_6,
                                   (LVar3 >> 0x10), unaff_DI);
        pass1_1010_038e(puStack6, iVar4, param_6);
    }
    return;
}


u16
enable_win_1040_3a36
(param_1: u32,param_2: u16,param_3: u16,param_4: i16,param_5: HWND16)

{
let piVar1: * mut i16; let bVar2: bool; let iVar3: i16; let uVar4: u16; let hwnd: HWND16; let hwnd_00: HWND16;

bVar2 = false; iVar3 = param_1;
// uVar4 = (param_1 >> 0x10); if (param_4 == 0x0) {
if ((iVar3 + 0x9e) < = (iVar3 + 0x9c)) {
// goto LAB_1040_3a79;
}
piVar1 = (iVar3 + 0x9c); * piVar1 = * piVar1 + 0x1;
}
else {
if (param_4 != 0x1) {
// goto LAB_1040_3a79;
}
if ((iVar3 + 0x9c) == 0x0) {
// goto LAB_1040_3a79;
}
piVar1 = (iVar3 + 0x9c); * piVar1 = * piVar1 + - 0x1;
}
bVar2 = true;
//LAB_1040_3a79:
hwnd = param_5; if (bVar2) {
hwnd = ctx.s_tile2_bmp_1050_1538; SetDlgItemInt16(param_5,0x0, (iVar3 + 0x9c), 0x18e);
}
hwnd_00 = hwnd; if (((iVar3 + 0x9c) != 0x0) & & ((iVar3 + 0xa2) == 0x0)) {
(iVar3 + 0xa2) = 0x1; hwnd_00 = ctx.s_tile2_bmp_1050_1538; EnableWindow16(hwnd, 0x1);
}
if (((iVar3 + 0x9c) == 0x0) & & ((iVar3 + 0xa2) != 0x0)) {
(iVar3 + 0xa2) = 0x0; EnableWindow16(hwnd_00, 0x0);
}
return 0x0;
}



pub fn show_win_1040_3ae8(param_1: &mut Struct1, param_2: u16) {
    dialog_ui_fn_1040_78e2(param_1, param_2);
    move_win_1040_826c(param_1, -0x1, 0xffff);
    ShowWindow16(param_2, 0x5);
    SetFocus16(ctx.s_tile2_bmp_1050_1538);
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn win_ui_op_1040_3b1e(param_1: &mut Struct2, param_2: U32Ptr) {
    let BVar1: bool;
    let HVar2: HWND16;
    let in_DX: U32Ptr;
    let uVar3: u16;
    let uVar4: u16;
    let unaff_DI: i16;
    let uVar5: u16;
    let uVar6: u32;
    local_10e: u8[0x82];
    local_8c: u8[0x82];
    let uStack10: u32;
    let puStack6: U32Ptr;

    puStack6 = mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0, 0x2, param_2, in_DX, unaff_DI);
    uStack10 = (puStack6 + 0x68);
    // uVar5 = (param_1 >> 0x10);
    uVar4 = param_1;
    GetWindowText16(0x1010, 0x80, local_8c);
    wsprintf16(s_tile2_bmp_1050_1538, local_10e, param_2);
    SetWindowText16(ctx.s_tile2_bmp_1050_1538, local_10e);
    uVar3 = uVar5;
    pass1_1018_3d44((uVar4 + 0x8e),
                    (param_1 & 0xffff0000 | (uVar4 + 0x92)),
                    (param_1 & 0xffff0000 | (uVar4 + 0x96)));
    load_string_1010_84e0(0x1010, _PTR_LOOP_1050_14cc,
                          (ctx.PTR__LOOP_1050_14cc >> 0x10), 0x80, local_10e, (short)param_2);
    wsprintf16(0x1010, local_8c, param_2);
    SetDlgItemText16(ctx.s_tile2_bmp_1050_1538, local_8c, param_2);
    BVar1 = CheckRadioButton16(ctx.s_tile2_bmp_1050_1538, 0x188, 0x18d, 0x188);
    (uVar4 + 0xa0) = 0x188;
    uVar6 = switch_1018_3b9e((uVar4 + 0x8e), (uVar4 + 0xa0), BVar1, uVar3,
                             param_2);
    send_dlg_item_msg_1040_3f12(uVar4, uVar5, uVar6, 0x1018, param_2);
    dialog_item_ui_op_1040_3e08(param_1, 0x1018);
    HVar2 = GetDlgItem16(0x1018, 0x186);
    (uVar4 + 0x9a) = HVar2;
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn unk_win_ui_op_1040_3c64(param_1: i16, param_2: u16, param_3: u16, param_4: u32, param_5: u16)

{
    let UVar1: u16;
    let in_DX: u16;
    let uVar2: u16;
    let unaff_DI: i16;
    let unaff_SS: u16;
    let uVar3: u32;
    let LVar4: LRESULT;
    let puVar5: U32Ptr;
    let iVar6: i16;

    if (param_4._2_2_ == 0x186) {
        LVar4 = SendDlgItemMessage16(param_5, 0x0, 0x0, 0x0, 0x1900409);
        // uVar2 = (LVar4 >> 0x10);
        UVar1 = GetDlgItemInt16(ctx.s_tile2_bmp_1050_1538, 0x0, (bool *)0x0, 0x0);
        pass1_1018_36e6((param_1 + 0x8e), UVar1, LVar4,
                        (param_1 + 0xa0));
        pass1_1038_af40(ctx.PTR__LOOP_1050_5b7c, (param_1 + 0x8), 0x22, uVar2, param_1,
                        &ctx.PTR_LOOP_1050_1038, unaff_SS);
        LVar4 = SendMessage16(&ctx.PTR_LOOP_1050_1038, 0x0, 0x0, 0x1110002);
        iVar6 = 0x1;
        puVar5 = mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0, 0x2b, unaff_SS,
                                 (LVar4 >> 0x10), unaff_DI);
        pass1_1010_038e(puVar5, iVar6, unaff_SS);
    } else {
        if (param_4._2_2_ - 0x186 < 0x2) {
//LAB_1040_3c7f:
            post_win_msg_1040_7b3c(CONCAT22(param_2, param_1), param_3, param_4, param_4._2_2_,
                                   param_5);
            return;
        }
        if (param_4._2_2_ - 0x188 < 0x5 || param_4._2_2_ == 0x18d) {
            (param_1 + 0xa0) = param_4._2_2_;
            param_5 = 0x1018;
            uVar3 = switch_1018_3b9e((param_1 + 0x8e), param_4._2_2_, param_4._2_2_,
                                     in_DX, unaff_SS);
            send_dlg_item_msg_1040_3f12(param_1, param_2, uVar3, 0x1018, unaff_SS);
        } else {
            if (param_4._2_2_ - 0x188 != 0x8) {
                // goto
                // LAB_1040_3c7f;
            }
            if (param_4 != 0x1) {
                return;
            }
        }
        dialog_item_ui_op_1040_3e08(CONCAT22(param_2, param_1), param_5);
    }
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn get_dc_op_1040_3d5e(param_1: u32, param_2: HWND16, param_3: u16) -> u16

{
    let ppcVar1: u32;
    let iVar2: i16;
    let uVar3: u16;
    let paVar4: &mut Struct43;
    let uVar5: u16;
    let local_4: HDC16;

    // uVar3 = (param_1 >> 0x10);
    uVar5 = (param_1 + 0x6);
    local_4 = GetDC16(param_2);
    paVar4 = unk_io_op_1010_830a(ctx.PTR__LOOP_1050_14cc, (param_1 + 0xa4),
                                 param_3);
    iVar2 = paVar4;
    ppcVar1 = (iVar2 + 0x8);
    (**ppcVar1)(0x1010, paVar4, (paVar4 >> 0x10), &local_4, param_3, uVar5);
    ppcVar1 = (iVar2 + 0x4);
    (**ppcVar1)(0x1010, paVar4, 0x50078, &local_4, param_3);
    ppcVar1 = (iVar2 + 0xc);
    (**ppcVar1)(0x1010, paVar4, &local_4, param_3);
    ReleaseDC16(0x1010, local_4);
    return 0x0;
}


pub fn invalidate_rect_1040_3ddc(in_struct_1: &mut Struct2, in_win_handle_2: HWND16) {
    let local_b_erase: u32;
    let uStack6: u32;

    local_b_erase = 0x780005;
    uStack6 = 0xdc0069;
    InvalidateRect16(in_win_handle_2, 0x0, &local_b_erase);
    return;
}


pub fn dialog_item_ui_op_1040_3e08(in_struct_1: &mut Struct2, param_2: u16) {
    let UVar1: u16;
    let uVar2: u16;
    let local_struct_1: &mut Struct2;
    let var3: u16;
    let HVar3: HWND16;
    let unaff_SS: u16;
    let LVar4: LRESULT;

    // var3 = (in_struct_1 >> 0x10);
    local_struct_1 = in_struct_1;
    CheckRadioButton16(param_2, local_struct_1.field_0xa0, 0x18d, 0x188);
    local_struct_1.field_0x9c = 0x0;
    local_struct_1.field_0x9e = 0x0;
    HVar3 = ctx.s_tile2_bmp_1050_1538;
    LVar4 = SendDlgItemMessage16(ctx.s_tile2_bmp_1050_1538, 0x0, 0x0, 0x0, 0x1900409);
    if ((LVar4 != -0x1) || (false)) {
        HVar3 = 0x1018;
        uVar2 = pass1_1018_3ab2(local_struct_1.field_0x8e, LVar4,
                                local_struct_1.field_0xa0, unaff_SS);
        local_struct_1.field_0x9e = uVar2;
    }
    SetDlgItemInt16(HVar3, 0x0, local_struct_1.field_0x9c, 0x18e);
    HVar3 = ctx.s_tile2_bmp_1050_1538;
    SetDlgItemInt16(ctx.s_tile2_bmp_1050_1538, 0x0, local_struct_1.field_0x9e, 0x191);
    UVar1 = local_struct_1.field_0xa0;
    if (UVar1 - 0x188 < 0x6) {
        HVar3 = &ctx.PTR_LOOP_1050_1040;
        switch(UVar1)
        {
            0x188 => local_struct_1.field_0xa4 = 0x5;
            break;
            0x189 => local_struct_1.field_0xa4 = 0x6;
            break;
            0x18a => local_struct_1.field_0xa4 = 0x7;
            break;
            0x18b => local_struct_1.field_0xa4 = 0x8;
            break;
            0x18c => local_struct_1.field_0xa4 = 0x9;
            break;
            0x18d => local_struct_1.field_0xa4 = 0xa;
        }
    }
    invalidate_rect_1040_3ddc(in_struct_1, HVar3);
    return;
}


pub fn send_dlg_item_msg_1040_3f12(param_1: u16, param_2: u16, param_3: u32, param_4: HWND16, param_5: u16)

{
    let uVar1: u32;
    let puVar2: U32Ptr;
    let extraout_DX: u16;
    let iVar3: i16;
    let hwnd: HWND16;
    let LVar4: LRESULT;
    let local_a: [u8; 8];

    SendDlgItemMessage16(param_4, 0x0, 0x0, 0x0, 0x190000b);
    SendDlgItemMessage16(ctx.s_tile2_bmp_1050_1538, 0x0, 0x0, 0x0, 0x1900405);
    pass1_1008_5784(CONCAT22(param_5, local_a), param_3);
    loop {
        puVar2 = local_a;
        hwnd = 0x1008;
        pass1_1008_5b12(puVar2, param_5);
        if ((extraout_DX | puVar2) == 0x0) { break; }
        uVar1 = (puVar2 + 0x4);
        hwnd = ctx.s_tile2_bmp_1050_1538;
        LVar4 = SendDlgItemMessage16(0x1008, uVar1, (uVar1 >> 0x10), 0x0, 0x1900401);
        // iVar3 = (LVar4 >> 0x10);
        if (((LVar4 == -0x1) && (iVar3 == -0x1)) || ((LVar4 == -0x2 && (iVar3 == -0x1)))) { break; }
    }
    SendDlgItemMessage16(hwnd, 0x0, 0x0, 0x0, 0x1900407);
    SendDlgItemMessage16(ctx.s_tile2_bmp_1050_1538, 0x0, 0x0, 0x1, 0x190000b);
    return;
}


// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn win_ui_op_1040_410e(param_1: &mut Struct1, param_2: u16, param_3: U32Ptr) {
    let uVar1: u32;
    let puVar2: U32Ptr;
    let iVar3: i16;
    let unaff_DI: *mut RECT16;
    let uVar4: u16;
    let uVar5: u16;
    let hwnd: HWND16;
    let in_AF: u8;
    let puVar6: U32Ptr;
    let piVar7: U32Ptr;
    let piVar8: U32Ptr;
    let puVar9: U32Ptr;
    let local_36: i16;
    let local_34: i16;
    let local_32: i16;
    let local_30: [u8; 6];
    i16
    local_2a[0x4];
    let uStack34: u32;
    let local_1e: u32;
    let uStack26: u32;
    let local_16: RECT16;
    let iStack18: i16;
    let iStack16: i16;
    let HStack14: HWND16;
    ulocal_c: u8[0xa];

    dialog_ui_fn_1040_78e2(param_1, param_2);
    pass1_1000_4906(CONCAT22(param_3, local_c), 0x0, 0xa);
    // uVar4 = (param_1 >> 0x10);
    iVar3 = param_1;
    uVar1 = (iVar3 + 0x8e);
    // uVar5 = (uVar1 >> 0x10);
    sys_1000_3f9c(local_c, param_3, 0x5d38, ctx.data_seg,
                  (uVar1 + 0x76), &stack0xfffe, uVar5, 0x1000,
                  param_3, in_AF);
    HStack14 = GetDlgItem16(0x1000, 0xfb5);
    SendMessage16(ctx.s_tile2_bmp_1050_1538, local_c, param_3, 0xc0000);
    SetFocus16(ctx.s_tile2_bmp_1050_1538);
    SendMessage16(ctx.s_tile2_bmp_1050_1538, 0x0, 0xffff, 0x4010000);
    GetWindowRect16(ctx.s_tile2_bmp_1050_1538, &local_16);
    pass1_1000_4906(CONCAT22(param_3, &local_1e), 0x0, 0x8);
    uVar1 = (iVar3 + 0x8e);
    hwnd = 0x1010;
    uStack34 = pass1_1010_5f7a(uVar1, (uVar1 >> 0x10), 0x0, 0x7);
    if (uStack34 != 0x0) {
        local_1e = *uStack34;
        unaff_DI = &local_16;
        uStack26 = (uStack34 + 0x4);
    }
    if ((local_1e._2_2_ == 0x0) && (local_1e == 0x0)) {
        puVar6 = clear_struct_1008_3e38(CONCAT22(param_3, local_30));
        // puVar2 = (puVar6 >> 0x10);
        uVar1 = (iVar3 + 0x96);
        pass1_1018_2678(uVar1, (uVar1 >> 0x10),
                        CONCAT22(param_3, local_30));
        pass1_1008_3e94(CONCAT22(param_3, local_30),
                        CONCAT22(param_3, &local_32),
                        CONCAT22(param_3, local_2a));
        piVar8 = &local_34;
        piVar7 = &local_36;
        puVar9 = param_3;
        puVar6 = mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0, 0x48, param_3, puVar2, unaff_DI,
        );
        hwnd = 0x1008;
        pass1_1008_3e94((puVar6 & 0xffff0000 | (puVar6 + 0xe)),
                        CONCAT22(param_3, piVar7), CONCAT22(puVar9, piVar8));
        uStack26 = CONCAT22(iStack16 - local_16.y, iStack18 - local_16.x);
        local_1e = CONCAT22((((puVar6 + 0xc) * -0x14) / 0x258 - (iStack16 - local_16.y)) + local_36 + local_32,
                            local_34 + local_2a[0]);
    }
    move_win_1040_826c(param_1, local_1e._2_2_, local_1e);
    ShowWindow16(hwnd, 0x5);
    return;
}


pub fn win_ui_op_1040_42b2(param_1: u32, param_2: i16, param_3: HWND16, param_4: U32Ptr) {
    let uVar1: u32;
    let uVar2: u16;
    let uVar3: u16;
    let iVar4: i16;
    let uVar5: u16;
    let LVar6: LRESULT;
    local_54: u8[0x52];

    iVar4 = param_1;
    // uVar5 = (param_1 >> 0x10);
    if (param_2 == 0x0) {
        (iVar4 + 0x9a) = 0x1;
        DestroyWindow16(param_3);
        return;
    }
    pass1_1000_4906(CONCAT22(param_4, local_54), 0x0, 0x51);
    GetDlgItem16(0x1000, 0xfb5);
    LVar6 = SendMessage16(ctx.s_tile2_bmp_1050_1538, local_54, param_4,
                          0xd0051);
    // uVar3 = (LVar6 >> 0x10);
    uVar2 = pass1_1000_3e2c(CONCAT22(param_4, local_54));
    if ((uVar3 | uVar2) != 0x0) {
        (iVar4 + 0x92) = uVar2;
        (iVar4 + 0x94) = uVar3;
    }
    if (uVar3 < 0x0) {
        wsprintf16(&ctx.PTR_LOOP_1050_1000, local_54, param_4);
        SendMessage16(ctx.s_tile2_bmp_1050_1538, local_54, param_4, 0xc0000,
        );
        SetFocus16(ctx.s_tile2_bmp_1050_1538);
        SendMessage16(ctx.s_tile2_bmp_1050_1538, 0x0, 0xffff, 0x4010000);
        return;
    }
    GetDlgItem16(0x1000, 0x1);
    EnableWindow16(ctx.s_tile2_bmp_1050_1538, 0x0);
    uVar1 = (iVar4 + 0x8e);
    (uVar1 + 0x76) = (iVar4 + 0x92);
    uVar1 = (iVar4 + 0x92);
    PostMessage16(ctx.s_tile2_bmp_1050_1538, uVar1,
                  (uVar1 >> 0x10), 0x4000000);
    GetDlgItem16(ctx.s_tile2_bmp_1050_1538, 0x1);
    EnableWindow16(ctx.s_tile2_bmp_1050_1538, 0x1);
    return;
}


pub fn get_win_rect_1040_43ea(param_1: i16, param_2: HWND16, param_3: u16, param_4: u16) {
    let uVar1: u32;
    let local_a: RECT16;
    let iStack6: i16;
    let iStack4: i16;

    GetWindowRect16(param_2, &local_a);
    iStack6 -= local_a.x;
    iStack4 -= local_a.y;
    pass1_1010_5fb0((param_1 + 0x8e), 0x0, &local_a, param_3, 0x7);
    uVar1 = (param_1 + 0x8e);
    (uVar1 + 0x7a) = ((param_1 + 0x9a) == 0x0);
    return;
}


LRESULT  send_win_msg_1040_4a0a(astruct_48 * * param_1,param_2: HWND16)

{
let piVar1: * mut i16; let ppcVar2: u32; let uVar3: u32; let uVar4: u32; let uVar5: u16; let iVar5: & mut Struct48; let uVar6: u16; let LVar7: LRESULT; let mut pcVar8: String;
let uVar9: u16; let uVar10: u16;
let uStack10: i16;

// uVar6 = (param_1 >> 0x10);
iVar5 = param_1; ppcVar2 = ( * param_1 + 0x74); ( ** ppcVar2)(param_2, param_1, 0x5d6a,ctx.data_seg); GetDlgItem16(param_2, 0x1770); SendMessage16(ctx.s_tile2_bmp_1050_1538, 0x0,0x0, 0x40b0000); LVar7 = SendMessage16(ctx.s_tile2_bmp_1050_1538, 0x0, 0x0, 0xb0000);
// uVar5 = (LVar7 >> 0x10);
// for (iStack10 = 0x0; uVar3 = iVar5.field_0x90, piVar1 = (uVar3 + 0x10),
//     *piVar1 != iStack10 && iStack10 <= *piVar1; iStack10 += 0x1) {
//   uVar10 = 0x0;
//   uVar9 = 0x403;
//   uVar3 = iVar5.field_0x90;
//   uVar3 = (uVar3 + 0xc);
//   pcVar8 = pass1_1040_4dcc(param_1,(uVar3 + iStack10 * 0x2),uVar5);
//   LVar7 = SendMessage16(ctx.s_tile2_bmp_1050_1538,pcVar8,
//                         (WPARAM16)(pcVar8 >> 0x10),CONCAT22(uVar9,uVar10));
//   uVar5 = (LVar7 >> 0x10);
// }
pass1_1040_4d7e(param_1); if (iStack10 == 0x0) {
uVar10 = 0x40a; uVar4 = iVar5.field_0x90;
uVar3 = iVar5.field_0x94; pcVar8 = string_op_1010_ada6(0x1010, uVar5, uVar3, (uVar3 > > 0x10), 0x0, (uVar4 + 0xa)); SendMessage16(0x1010, pcVar8, (pcVar8 > > 0x10),
CONCAT22(uVar10, iStack10));
}
LVar7 = SendMessage16(ctx.s_tile2_bmp_1050_1538, 0x0,0x0, 0xb0001); return LVar7;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn set_win_pos_1040_4ae4(param_1: i16, param_2: u16, param_3: u16, param_4: u32, param_5: u16)

{
    let ppcVar1: u32;
    let uVar2: u32;
    let uVar3: u16;
    let paVar4: &mut Struct18;
    let in_DX: U32Ptr;
    let puVar5: U32Ptr;
    let puVar6: U32Ptr;
    let iVar7: i16;
    let unaff_DI: i16;
    let uVar8: u16;
    let unaff_SS: u16;
    let local_24: RECT16;
    let iStack32: i16;
    let paStack20: &mut Struct18;
    let paStack16: &mut Struct18;
    let iStack12: i16;
    let paStack10: &mut Struct18;
    let paStack6: &mut Struct20;

    if (param_4._2_2_ == 0xeb) {
        paStack6 = mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0, 0x3, unaff_SS, in_DX, unaff_DI);
        // puVar5 = (paStack6 >> 0x10);
        paVar4 = (param_1 + 0x90);
        if (paVar4 != 0x0) {
            paStack10 = paVar4;
            mem_op_1000_179c(0x18, puVar5, 0x1000);
            uVar3 = paVar4;
            paStack16 = (paVar4 & 0xffff | ZEXT24(puVar5) << 0x10);
            puVar6 = (puVar5 | uVar3);
            if (puVar6 == 0x0) {
                uVar3 = 0x0;
                puVar6 = 0x0;
            } else {
                struct_1040_a598((paVar4 & 0xffff | ZEXT24(puVar5) << 0x10));
            }
            (param_1 + 0x90) = uVar3;
            (param_1 + 0x92) = puVar6;
            (param_1 + 0x90) = 0x7;
            iStack12 = *(param_1 + 0x90);
            uVar3 = iStack12 * 0xa + 0x2;
            mem_op_1000_179c(uVar3, puVar6, 0x1000);
            paStack16 = CONCAT22(puVar6, uVar3);
            if ((puVar6 | uVar3) == 0x0) {
                uVar2 = (param_1 + 0x90);
                (uVar2 + 0x2) = 0x0;
            } else {
                paStack16 = iStack12;
                pass1_1000_5586(0xa564, &ctx.PTR_LOOP_1050_1040, iStack12, 0xa,
                                uVar3 + 0x2, puVar6);
                uVar2 = (param_1 + 0x90);
                // uVar8 = (uVar2 >> 0x10);
                iVar7 = uVar2;
                (iVar7 + 0x2) = uVar3 + 0x2;
                (iVar7 + 0x4) = puVar6;
            }
            // uVar8 = (paStack10 >> 0x10);
            iVar7 = paStack10;
            uVar2 = (param_1 + 0x90);
            (uVar2 + 0x6) = (iVar7 + 0x6);
            uVar2 = (param_1 + 0x90);
            (uVar2 + 0xa) = (iVar7 + 0xa);
            uVar2 = (param_1 + 0x90);
            (uVar2 + 0x12) = (iVar7 + 0x12);
            pass1_1010_a50c(paStack6, 0x10505d6a, (param_1 + 0x90));
            paStack20 = paStack10;
            paStack16 = paStack10;
            if (paStack10 != 0x0) {
                pass1_1040_a5d0(paStack10);
                fn_ptr_1000_17ce(ctx, paStack10, 0x1000);
            }
            ppcVar1 = (CONCAT22(param_2, param_1) + 0x70);
            (**ppcVar1)();
        }
    } else {
        if (param_4._2_2_ != 0x1770) {
            pass1_1040_b54a(param_1, param_2, param_3, param_4, in_DX, param_5, unaff_SS);
            return;
        }
        if (param_4 == 0x7) {
            GetWindowRect16(param_5, &local_24);
            iStack32 -= local_24.x;
            SetWindowPos16(ctx.s_tile2_bmp_1050_1538, 0x2, 0x50, iStack32, 0x0, 0x0, 0x0);
        }
    }
    return;
}



LRESULT  send_msg_1040_4cb2(param_1: u32,param_2: HWND16)

{
let uVar1: u8; let HVar1: HWND16;
let in_DX: u16; let uVar2: u32;
let LVar2: LRESULT; let uVar3: u16;
let uVar4: u16;

pass1_1040_b45e(param_1, param_2); HVar1 = GetDlgItem16(param_2, 0x1770); uVar3 = 0xffff; uVar4 = 0x40d; pass1_1040_4d7e(param_1); uVar2 = pass1_1040_4dcc(param_1,HVar1, in_DX); LVar2 = SendMessage16(ctx.s_tile2_bmp_1050_1538, uVar2,
(uVar2 > > 0x10), CONCAT22(uVar4, uVar3));
return LVar2;
}


pub fn set_win_pos_1040_4f96(param_1: &mut Struct1, param_2: u16, param_3: u16, param_4: U32Ptr) {
    let pu_var1: u32;
    let ppcVar2: u32;
    let uVar3: u16;
    let uVar4: u32;
    let paVar5: &mut Struct160;
    let uVar6: u16;
    let uVar7: u16;
    let iVar8: i16;
    let extraout_DX: U32Ptr;
    let puVar9: U32Ptr;
    let puVar10: U32Ptr;
    let uVar11: u16;
    let uVar12: u16;
    let iVar11: &mut Struct443;
    let unaff_DI: i16;
    let uVar13: u16;
    let uVar14: u16;
    let puVar15: U32Ptr;
    let BVar16: bool;

    dialog_ui_fn_1040_78e2(param_1, param_2);
    puVar15 = mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0, 0x41, param_3, param_4, unaff_DI);
    // uVar14 = (puVar15 >> 0x10);
    paVar5 = puVar15;
    // uVar13 = (param_1 >> 0x10);
    iVar11 = param_1;
    &iVar11.field_0x98 = paVar5;
    (&iVar11.field_0x98 + 0x2) = uVar14;
    ppcVar2 = (*iVar11.field_0x98 + 0x10);
    (**ppcVar2)(0x1010, &iVar11.field_0x98, uVar14);
    puVar10 = extraout_DX;
    mem_op_1000_179c(0xa, extraout_DX, 0x1000);
    puVar9 = (puVar10 | paVar5);
    if (puVar9 == 0x0) {
        iVar11.field_0x94 = 0x0;
    } else {
        puVar15 = struct_1040_bf3e(CONCAT22(puVar10, paVar5), iVar11.field_0x6);
        // puVar9 = (puVar15 >> 0x10);
        paVar5 = puVar15;
        &iVar11.field_0x94 = paVar5;
        (&iVar11.field_0x94 + 0x2) = puVar9;
    }
    pass1_1040_bfde(iVar11.field_0x94, iVar11.field_0x98, param_3);
    mem_op_1000_179c(0x42, puVar9, 0x1000);
    puVar10 = (puVar9 | paVar5);
    if (puVar10 != 0x0) {
        pass1_1008_3bd6(paVar5, puVar9, 0x1, 0xa000a, 0x0, 0x800081,
                        CONCAT22(iVar11.field_0x6, 0x10a), puVar10, param_3);
    }
    mem_op_1000_179c(0x42, puVar10, 0x1000);
    puVar9 = (puVar10 | paVar5);
    if (puVar9 != 0x0) {
        pass1_1008_3bd6(paVar5, puVar10, 0x1, 0xa0028, 0x0, 0x840085,
                        CONCAT22(iVar11.field_0x6, 0x10b), puVar9, param_3);
    }
    mem_op_1000_179c(0x42, puVar9, 0x1000);
    puVar10 = (puVar9 | paVar5);
    if (puVar10 != 0x0) {
        pass1_1008_3bd6(paVar5, puVar9, 0x1, 0xa0046, 0x0, 0x860087,
                        CONCAT22(iVar11.field_0x6, 0x10d), puVar10, param_3);
    }
    mem_op_1000_179c(0x42, puVar10, 0x1000);
    puVar9 = (puVar10 | paVar5);
    if (puVar9 != 0x0) {
        pass1_1008_3bd6(paVar5, puVar10, 0x1, 0xa0064, 0x0, 0x880089,
                        CONCAT22(iVar11.field_0x6, 0x10e), puVar9, param_3);
    }
    mem_op_1000_179c(0x42, puVar9, 0x1000);
    puVar10 = (puVar9 | paVar5);
    if (puVar10 != 0x0) {
        pass1_1008_3bd6(paVar5, puVar9, 0x1, 0xa0082, 0x0, 0x820083,
                        CONCAT22(iVar11.field_0x6, 0x10c), puVar10, param_3);
    }
    mem_op_1000_179c(0x42, puVar10, 0x1000);
    puVar9 = (puVar10 | paVar5);
    if (puVar9 != 0x0) {
        pass1_1008_3bd6(paVar5, puVar10, 0x1, 0xa00d2, 0x0, 0x8a008b,
                        CONCAT22(iVar11.field_0x6, 0xbbb), puVar9, param_3);
    }
    BVar16 = 0x42;
    uVar14 = 0x1000;
    mem_op_1000_179c(0x42, puVar9, 0x1000);
    puVar10 = (puVar9 | paVar5);
    if (puVar10 == 0x0) {
        paVar5 = 0x0;
        puVar10 = 0x0;
    } else {
        uVar14 = 0x1008;
        pass1_1008_3bd6(paVar5, puVar9, 0x1, 0xa00a0, 0x8e, 0x8c008d,
                        CONCAT22(iVar11.field_0x6, 0xbbc), puVar10, param_3);
    }
    puVar9 = puVar10;
    enable_win_1040_9234(CONCAT22(puVar10, paVar5), BVar16, uVar14);
    puVar15 = mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0, 0x3, param_3, puVar9, unaff_DI);
    // uVar11 = (puVar15 >> 0x10);
    uVar3 = puVar15;
    uVar12 = uVar11;
    uVar6 = pass1_1010_a5ac(uVar3, uVar11, iVar11.field_0xb0);
    uVar7 = pass1_1010_ac62(uVar3, uVar11, 0x1e, uVar6, uVar12);
    if (uVar7 != 0x0) {
        pass1_1010_a5ca(uVar3, uVar11, uVar6, uVar7, uVar12);
        if (0x0 < uVar7) {
            pass1_1010_a58a(uVar3, uVar11, uVar6, uVar7, uVar12);
            if (uVar7 == 0x0) {
                enable_win_1040_9234(CONCAT22(puVar10, paVar5), 0x1, 0x1010);
            }
        }
    }
    pu_var1 = iVar11.field_0x98;
    iVar8 = pu_var1;
    uVar4 = pu_var1 & 0xffff0000;
    // uVar14 = (uVar4 >> 0x10);
    SetWindowPos16(0x1010, 0x40, (iVar8 + 0x10), (iVar8 + 0xe),
                   (iVar8 + 0xc), (uVar4 | iVar8 + 0xa), 0x0);
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn win_ui_op_1040_52c0(param_1: i16, param_2: u16, param_3: u16, param_4: u32, param_5: HWND16, param_6: u16)

{
    let ppcVar1: u32;
    let BVar2: bool;
    let iVar3: i16;
    let uVar4: u16;
    let in_DX: U32Ptr;
    let uVar5: u16;
    let uVar6: u16;
    let uVar7: u16;
    let unaff_DI: i16;
    let puVar8: U32Ptr;
    let puVar9: U32Ptr;
    let uVar10: u32;
    let uVar11: u16;
    let uVar12: u16;
    let uVar13: u16;
    let uStack30: u16;
    let local_a: RECT16;
    let iStack6: i16;
    let iStack4: i16;

    if (param_4._2_2_ != 0x10c) {
        if (param_4._2_2_ < 0x10d) {
            if (param_4._2_2_ == 0xfa) {
                uVar10 = (param_1 + 0x98);
                ppcVar1 = ((param_1 + 0x98) + 0x18);
                (**ppcVar1)(param_5, uVar10, (uVar10 >> 0x10), 0x0,
                            ctx._PTR_LOOP_1050_5efe, (ctx.PTR__LOOP_1050_5efe >> 0x10));
                return;
            }
            if (param_4._2_2_ == 0x10a) {
                GetClientRect16(param_5, &local_a);
                uVar10 = (param_1 + 0x98);
                local_a.y += 0x3;
                local_a.x = (uVar10 + 0x1a) + -0x9;
                iStack6 += -0x3;
                iStack4 += -0x3;
                InvalidateRect16(ctx.s_tile2_bmp_1050_1538,
                                 (&ctx.PTR_LOOP_1050_0000 + 0x1), &local_a);
                unk_destroy_win_op_1010_2fa0((param_1 + 0x98), 0x1010);
                pass1_1010_32c0((param_1 + 0x98), 0x0);
                pass1_1010_2ee2((param_1 + 0x98), param_6, 0x1010);
                return;
            }
            if (param_4._2_2_ != 0x10b) {
                // goto
                // LAB_1040_5560;
            }
            uVar10 = (param_1 + 0x98);
            uVar11 = (uVar10 + 0x12);
            uVar6 = uVar11;
            puVar8 = mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0, 0x3, param_6, in_DX, unaff_DI);
            // uVar5 = (puVar8 >> 0x10);
            puVar9 = puVar8;
            pass1_1010_a5ca(puVar8, uVar5, uVar6, puVar8, uVar5);
            // uVar6 = (puVar9 >> 0x10);
            if ((uVar11 != 0x70) && (puVar9 == 0x0)) {
                return;
            }
            uVar10 = (param_1 + 0xb0);
            uVar12 = uVar10;
            // uVar13 = (uVar10 >> 0x10);
            uVar10 = (param_1 + 0x98);
            uVar11 = (uVar10 + 0x12);
        } else {
            if (param_4._2_2_ != 0x10d) {
                if (param_4._2_2_ == 0x10e) {
                    puVar8 = mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0, 0x32, param_6, in_DX, unaff_DI);
                    iVar3 = puVar8;
                    ui_op_1010_79aa(puVar8, 0xfc6, (param_1 + 0xb0), param_6);
                    if (iVar3 != 0x0) {
                        return;
                    }
                    unk_win_op_1010_7300(puVar8, 0x0, 0x13, (param_1 + 0xb0));
                    return;
                }
                if (param_4._2_2_ == 0xbbb) {
                    if ((param_1 + 0xb6) != 0x0) {
                        BVar2 = IsWindow16(param_5);
                        param_5 = ctx.s_tile2_bmp_1050_1538;
                        if (BVar2 != 0x0) {
                            // goto
                            // LAB_1040_5417;
                        }
                    }
                    uVar10 = pass1_1038_af40(ctx.PTR__LOOP_1050_5b7c, (param_1 + 0x6), 0x1b,
                                             in_DX, param_1, &ctx.PTR_LOOP_1050_1038,
                                             param_6);
                    (param_1 + 0xb6) = (uVar10 + 0x6);
                    set_win_pos_1038_abdc(&ctx.PTR_LOOP_1050_1038);
                    ShowWindow16(&ctx.PTR_LOOP_1050_1038, 0x1);
                    return;
                }
                if (param_4._2_2_ == 0xbbc) {
                    puVar8 = mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0, 0x3, param_6, in_DX, unaff_DI);
                    // uVar7 = (puVar8 >> 0x10);
                    uVar6 = puVar8;
                    uVar5 = uVar7;
                    uVar4 = pass1_1010_a5ac(uVar6, uVar7, (param_1 + 0xb0));
                    uVar11 = uVar4;
                    pass1_1010_a58a(uVar6, uVar7, uVar4, uVar4, uVar5);
                    if (uVar11 == 0x0) {
                        pass1_1010_a568(uVar6, uVar7, uVar4, 0x0, uVar5);
                    }
                    GetDlgItem16(0x1010, 0xbbc);
                    EnableWindow16(ctx.s_tile2_bmp_1050_1538, 0x0);
                    return;
                }
//LAB_1040_5560:
                pass1_1040_b54a(param_1, param_2, param_3, param_4, in_DX, param_5, param_6);
                return;
            }
            puVar8 = mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0, 0x3, param_6, in_DX, unaff_DI);
            // uVar6 = (puVar8 >> 0x10);
            uVar10 = (param_1 + 0xb0);
            uVar12 = uVar10;
            // uVar13 = (uVar10 >> 0x10);
            uVar11 = 0x71;
            uVar5 = uVar6;
        }
        uStack30 = puVar8;
        param_5 = 0x1010;
        pass1_1010_a5ec(uStack30, uVar5, uVar11, CONCAT22(uVar13, uVar12), uVar6);
        if ((param_1 + 0xb4) != 0x0) {
            param_5 = ctx.s_tile2_bmp_1050_1538;
            BVar2 = IsWindow16(0x1010);
            if (BVar2 != 0x0) {
                param_5 = ctx.s_tile2_bmp_1050_1538;
                SendMessage16(ctx.s_tile2_bmp_1050_1538, 0x0, 0x0, 0x11100eb);
            }
        }
    }
//LAB_1040_5417:
    DestroyWindow16(param_5);
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn enable_win_1040_5780(param_1: U32Ptr) {
    let ppcVar1: u32;
    let uVar2: u32;
    let uVar3: u16;
    let extraout_DX: U32Ptr;
    let unaff_DI: i16;
    let unaff_SS: u16;
    let puVar4: U32Ptr;

    ppcVar1 = (*param_1 + 0x74);
    (**ppcVar1)();
    puVar4 = mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0, 0x3, unaff_SS, extraout_DX, unaff_DI);
    uVar2 = (param_1 + 0x90);
    uVar3 = pass1_1010_acc0(puVar4, (puVar4 >> 0x10),
                            (uVar2 + 0x6));
    if (uVar3 != 0x0) {
        GetDlgItem16(0x1010, 0x1790);
        EnableWindow16(ctx.s_tile2_bmp_1050_1538, 0x1);
    }
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn win_ui_op_1040_5800(param_1: i16, param_2: u16, param_3: u16, param_4: u32, param_5: u16,
)

{
    let ppcVar1: u32;
    let uVar2: u32;
    let uVar3: u16;
    let uVar4: u16;
    let paVar5: &mut Struct18;
    let in_DX: U32Ptr;
    let puVar6: U32Ptr;
    let puVar7: U32Ptr;
    let extraout_DX: U32Ptr;
    let iVar8: i16;
    let unaff_DI: U32Ptr;
    let uVar9: u16;
    let hwnd: HWND16;
    let unaff_SS: u16;
    let piStack24: U32Ptr;
    let local_14: [RECT16; 0x2];
    let iStack12: i16;
    let paStack10: &mut Struct18;
    let paStack6: &mut Struct20;

    if (param_4._2_2_ == 0xeb) {
        paStack6 = mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0, 0x3, unaff_SS, in_DX, unaff_DI);
        // puVar6 = (paStack6 >> 0x10);
        paVar5 = (param_1 + 0x90);
        if (paVar5 != 0x0) {
            paStack10 = paVar5;
            mem_op_1000_179c(0x18, puVar6, 0x1000);
            uVar3 = paVar5;
            puVar7 = (puVar6 | uVar3);
            if (puVar7 == 0x0) {
                uVar3 = 0x0;
                puVar7 = 0x0;
            } else {
                struct_1040_a598((paVar5 & 0xffff | ZEXT24(puVar6) << 0x10));
            }
            (param_1 + 0x90) = uVar3;
            (param_1 + 0x92) = puVar7;
            (param_1 + 0x90) = 0x6;
            iStack12 = *(param_1 + 0x90);
            uVar3 = iStack12 * 0xa + 0x2;
            mem_op_1000_179c(uVar3, puVar7, 0x1000);
            piStack24 = CONCAT22(puVar7, uVar3);
            if ((puVar7 | uVar3) == 0x0) {
                uVar2 = (param_1 + 0x90);
                (uVar2 + 0x2) = 0x0;
            } else {
                *piStack24 = iStack12;
                pass1_1000_5586(0xa564, &ctx.PTR_LOOP_1050_1040, iStack12, 0xa,
                                uVar3 + 0x2, puVar7);
                uVar2 = (param_1 + 0x90);
                // uVar9 = (uVar2 >> 0x10);
                iVar8 = uVar2;
                (iVar8 + 0x2) = uVar3 + 0x2;
                (iVar8 + 0x4) = puVar7;
                unaff_DI = puVar7;
            }
            uVar2 = (param_1 + 0x90);
            (uVar2 + 0x6) = (paStack10 + 0x6);
            uVar2 = (param_1 + 0x90);
            (uVar2 + 0xa) = 0x4;
            uVar2 = (param_1 + 0x90);
            (uVar2 + 0x12) = (param_1 + 0xa);
            hwnd = 0x1010;
            pass1_1010_a50c(paStack6, 0x10505d78, (param_1 + 0x90));
            if (paStack10 != 0x0) {
                pass1_1040_a5d0(paStack10);
                hwnd = 0x1000;
                fn_ptr_1000_17ce(ctx, paStack10, 0x1000);
            }
            ppcVar1 = (CONCAT22(param_2, param_1) + 0x70);
            (**ppcVar1)();
            puVar6 = extraout_DX;
            uVar4 = pass1_1040_5cd6(CONCAT22(param_2, param_1));
            if (uVar4 != 0x0) {
                pass1_1040_5eaa(CONCAT22(param_2, param_1));
                (param_1 + 0x94) = 0x0;
            }
            pass1_1040_5dc4(CONCAT22(param_2, param_1), puVar6, unaff_DI, unaff_SS);
            GetWindowRect16(hwnd, local_14);
            InvalidateRect16(ctx.s_tile2_bmp_1050_1538, (param_1 + 0x9c), 0x0);
            if ((param_1 + 0x9c) != 0x0) {
                (param_1 + 0x9c) = 0x0;
            }
        }
    } else {
        if (param_4._2_2_ != 0x13b) {
            pass1_1040_b54a(param_1, param_2, param_3, param_4, in_DX, param_5, unaff_SS);
            return;
        }
        GetDlgItem16(param_5, 0x1790);
        EnableWindow16(ctx.s_tile2_bmp_1050_1538, 0x1);
    }
    return;
}


pub fn msg_box_ui_op_1040_64ca(param_1: u32, param_2: &mut String, param_3: U32Ptr, param_4: u16) {
    local_206: u8[0x102];
    local_104: u8[0x102];

    mem_op_1000_179c(0x1000, param_3, 0x1000);
    load_string_1010_84e0(0x1010, _PTR_LOOP_1050_14cc, 0x100, local_206, param_4);
    load_string_1010_84e0(0x1010, _PTR_LOOP_1050_14cc,
                          (ctx.PTR__LOOP_1050_14cc >> 0x10), 0x3ff, param_2, (short)param_3);
    load_string_1010_84e0(0x1010, _PTR_LOOP_1050_14cc, 0x3ff, local_104, param_4);
    string_1000_3cea(CONCAT22(param_3, param_2), CONCAT22(param_4, local_104));
    load_string_1010_84e0(0x1010, _PTR_LOOP_1050_14cc, 0x3ff, local_104, param_4);
    string_1000_3cea(CONCAT22(param_3, param_2), CONCAT22(param_4, local_104));
    load_string_1010_84e0(0x1010, _PTR_LOOP_1050_14cc, 0x3ff, local_104, param_4);
    string_1000_3cea(CONCAT22(param_3, param_2), CONCAT22(param_4, local_104));
    MessageBox16(0x1000, 0x0, local_206, param_4);
    fn_ptr_1000_17ce(ctx, CONCAT22(param_3, param_2), 0x1000);
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn show_win_1040_65ba(param_1: &mut Struct1, param_2: u16) {
    let uVar1: u32;
    let uVar2: u16;
    let rect: &mut Struct160;
    let in_DX: U32Ptr;
    let puVar3: U32Ptr;
    let iVar4: i16;
    let iVar5: i16;
    let unaff_DI: i16;
    let uVar6: u16;
    let uVar7: u16;
    let hwnd: HWND16;
    let unaff_SS: u16;
    let local_22: u16;
    let uStack32: u16;
    let uStack30: u16;
    let uStack28: u16;
    let puStack26: U32Ptr;
    let uStack10: i16;
    let uStack8: u16;
    let puStack6: U32Ptr;

    dialog_ui_fn_1040_78e2(param_1, param_2);
    puStack6 = mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0, 0x2b, unaff_SS, in_DX, unaff_DI);
    ctx.PTR_LOOP_1050_5f2e = (puStack6 >> 0x10);
    uStack8 = pass1_1010_0898();
    if (ctx.PTR__LOOP_1050_5f2c == 0x0) {
        ctx.PTR_LOOP_1050_5f2c = mem_op_1000_160a(ctx.PTR_LOOP_1050_5f2e, 0x1000);
    } else {}
    puStack26 = CONCAT22(ctx.PTR_LOOP_1050_5f2e, PTR_LOOP_1050_5f2c);
    hwnd = 0x1000;
    uVar2 = fn_ptr_op_1000_1708((uStack8 + 0x2) * 0x4, 0x0, 0x1, PTR_LOOP_1050_5f2c,
                                ctx.PTR_LOOP_1050_5f2e, 0x1000);
    // uVar6 = (param_1 >> 0x10);
    iVar4 = param_1;
    (iVar4 + 0x8e) = uVar2;
    (iVar4 + 0x90) = ctx.PTR_LOOP_1050_5f2e;
    // for (iStack10 = 0x1; iStack10 <= uStack8; iStack10 += 0x1) {
    //   puStack26 =
    //               pass1_1010_0946(puStack6,(puStack6 >> 0x10),iStack10,
    //                               ctx.PTR_LOOP_1050_5f2e,unaff_DI,unaff_SS);
    //   puVar3 = (puStack26 >> 0x10);
    //   local_22 = *puStack26;
    //   uStack32 = (puStack26 + 0x2);
    //   uStack30 = 0x1;
    //   uStack28 = 0x1;
    //   rect = &local_22;
    //   MapDialogRect16(0x1010,rect);
    //   hwnd = 0x1000;
    //   mem_op_1000_179c(0x42,puVar3,0x1000);
    //   ctx.PTR_LOOP_1050_5f2e = (puVar3 | rect);
    //   if (ctx.PTR_LOOP_1050_5f2e == 0x0) {
    //     uVar1 = (iVar4 + 0x8e);
    //     (uVar1 + iStack10 * 0x4) = 0x0;
    //   }
    //   else {
    //     hwnd = 0x1008;
    //     pass1_1008_3bd6(rect,puVar3,0x0,CONCAT22(local_22,uStack32),0x101,0xff0100,
    //                     CONCAT22((iVar4 + 0x6),
    //                              (puStack26 + 0x4)),
    //                     ctx.PTR_LOOP_1050_5f2e,unaff_SS);
    //     uVar1 = (iVar4 + 0x8e);
    //     uVar7 = (uVar1 >> 0x10);
    //     iVar5 = uVar1;
    //     *(astruct_160 **)(iVar5 + iStack10 * 0x4) = rect;
    //     (iVar5 + iStack10 * 0x4 + 0x2) = ctx.PTR_LOOP_1050_5f2e;
    //   }
    //   uVar1 = (iVar4 + 0x8e);
    //   uVar7 = (uVar1 >> 0x10);
    //   iVar5 = uVar1;
    //   if ((iVar5 + iStack10 * 0x4) != 0x0) {
    //     unaff_DI = puStack26;
    //     enable_win_1040_9234
    //               ((iVar5 + iStack10 * 0x4),*(bool *)(unaff_DI + 0x6),hwnd);
    //   }
    // }
    move_win_1040_826c(param_1, -0x1, 0xffff);
    ShowWindow16(hwnd, 0x5);
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn post_win_msg_1040_672e(param_1: i16, param_2: u16, param_3: u16, param_4: u32, param_5: u16,
                              param_6: u16)

{
    let unaff_CS: u16;
    let i_var1: i16;

    if (param_4._2_2_ == (s_vrpal_bmp_1050_183a + 0x7)) {
        msg_box_ui_op_1040_64ca(CONCAT22(param_2, param_1), 0x0, param_5, param_6);
    } else {
        if (param_4._2_2_ == 0x1851) {
            i_var1 = 0x2a;
        } else {
            if (param_4._2_2_ != 0x1852) {
                post_win_msg_1040_7b3c(CONCAT22(param_2, param_1), param_3, param_4,
                                       param_4._2_2_, unaff_CS);
                return;
            }
            i_var1 = 0x29;
        }
        pass1_1038_af40(ctx.PTR__LOOP_1050_5b7c, (param_1 + 0x8), i_var1, param_5, param_1,
                        &ctx.PTR_LOOP_1050_1038, param_6);
        PostMessage16(&ctx.PTR_LOOP_1050_1038, 0x0, 0x0, 0x1110002);
    }
    return;
}


pub fn enable_win_1040_6880(param_1: u32, param_2: i16, param_3: HWND16) {
    let uVar1: u32;
    let uVar2: u16;

    if (param_2 == 0x8) {
        // uVar2 = (param_1 >> 0x10);
        GetDlgItem16(param_3, 0x107);
        uVar1 = (param_1 + 0x94);
        EnableWindow16(ctx.s_tile2_bmp_1050_1538, (uVar1 + 0x24));
        GetDlgItem16(ctx.s_tile2_bmp_1050_1538, 0x108);
        uVar1 = (param_1 + 0x94);
        EnableWindow16(ctx.s_tile2_bmp_1050_1538, (uVar1 + 0x26));
    }
    return;
}


pub fn mixed_win_ui_op_1040_6942(param_1: &mut Struct1, param_2: u16, param_3: u16) {
    let uVar1: u32;
    let uVar2: u32;
    let ppcVar3: u32;
    let paVar4: &mut Struct160;
    let mut pCVar5: String;
    let puVar6: u32;
    let iVar7: i16;
    let in_DX: U32Ptr;
    let extraout_DX: U32Ptr;
    let puVar8: U32Ptr;
    let puVar9: U32Ptr;
    let uVar10: u16;
    let iVar11: i16;
    let unaff_DI: i16;
    let uVar12: u16;
    let uVar13: u16;
    let uVar14: u16;
    let hwnd: HWND16;
    let puVar15: U32Ptr;
    let DVar16: u32;
    let mut pcVar17: String;
    let BVar18: bool;
    let local_64: u32;
    let uStack96: u32;
    let IStack92: i16;
    let IStack90: i16;
    local_58: u8[0x50];
    let HStack8: HDC16;
    let paStack6: &mut Struct160;
    let puStack4: U32Ptr;

    dialog_ui_fn_1040_78e2(param_1, param_3);
    puVar15 = mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0, 0x33, param_2, in_DX, unaff_DI);
    // uVar14 = (puVar15 >> 0x10);
    paVar4 = puVar15;
    // uVar12 = (param_1 >> 0x10);
    iVar11 = param_1;
    (iVar11 + 0x94) = paVar4;
    (iVar11 + 0x96) = uVar14;
    ppcVar3 = ((iVar11 + 0x94) + 0x4);
    (**ppcVar3)(0x1010, (iVar11 + 0x94), uVar14, 0x0, param_1);
    puVar9 = extraout_DX;
    mem_op_1000_179c(0xa, extraout_DX, 0x1000);
    puVar8 = (puVar9 | paVar4);
    if (puVar8 == 0x0) {
        (iVar11 + 0x98) = 0x0;
    } else {
        puVar15 = struct_1040_bf3e(CONCAT22(puVar9, paVar4), (iVar11 + 0x6),
        );
        // puVar8 = (puVar15 >> 0x10);
        paVar4 = puVar15;
        (iVar11 + 0x98) = paVar4;
        (iVar11 + 0x9a) = puVar8;
    }
    pass1_1040_bfde((iVar11 + 0x98), (iVar11 + 0x94), param_2);
    mem_op_1000_179c(0x42, puVar8, 0x1000);
    puVar9 = (puVar8 | paVar4);
    if (puVar9 != 0x0) {
        pass1_1008_3bd6(paVar4, puVar8, 0x1, 0xa000a, 0x0, 0x800081,
                        CONCAT22((iVar11 + 0x6), 0x10a), puVar9, param_2);
    }
    mem_op_1000_179c(0x42, puVar9, 0x1000);
    puVar8 = (puVar9 | paVar4);
    if (puVar8 != 0x0) {
        pass1_1008_3bd6(paVar4, puVar9, 0x1, 0xa0028, 0x0, 0x820083,
                        CONCAT22((iVar11 + 0x6), 0x10c), puVar8, param_2);
    }
    BVar18 = 0x42;
    uVar14 = 0x1000;
    mem_op_1000_179c(0x42, puVar8, 0x1000);
    puVar9 = (puVar8 | paVar4);
    if (puVar9 == 0x0) {
        paVar4 = 0x0;
        puVar9 = 0x0;
    } else {
        uVar14 = 0x1008;
        pass1_1008_3bd6(paVar4, puVar8, 0x1, 0xa00aa, 0x101, 0xff0100,
                        CONCAT22((iVar11 + 0x6), 0x107), puVar9, param_2);
    }
    paStack6 = paVar4;
    puStack4 = puVar9;
    enable_win_1040_9234(CONCAT22(puVar9, paVar4), BVar18, uVar14);
    BVar18 = 0x42;
    hwnd = 0x1000;
    mem_op_1000_179c(0x42, puVar9, 0x1000);
    uVar10 = puVar9 | paVar4;
    if (uVar10 == 0x0) {
        paVar4 = 0x0;
        uVar10 = 0x0;
    } else {
        hwnd = 0x1008;
        pass1_1008_3bd6(paVar4, puVar9, 0x1, 0xa00c2, 0x101, 0xff0100,
                        CONCAT22((iVar11 + 0x6), 0x108), uVar10, param_2);
    }
    paStack6 = paVar4;
    puStack4 = uVar10;
    enable_win_1040_9234(CONCAT22(uVar10, paVar4), BVar18, hwnd);
    HStack8 = GetDC16(hwnd);
    load_string_1010_84e0(0x1010, _PTR_LOOP_1050_14cc, 0x50, local_58, param_2);
    pcVar17 = local_58;
    pCVar5 = str_op_1000_3da4(CONCAT22(param_2, pcVar17));
    DVar16 = GetTextExtent16(0x1000, pCVar5, pcVar17);
    // IStack90 = (DVar16 >> 0x10);
    IStack92 = DVar16;
    CreateWindow16(ctx.s_tile2_bmp_1050_1538, 0x0,
                   ZEXT24(ctx.PTR_LOOP_1050_038c) << 0x10, 0x7cd, (iVar11 + 0x6),
                   IStack90, IStack92, 0xad, 0x22, 0x0, (s_Rebel_1050_4ffc + 0x4));
    load_string_1010_84e0(0x1010, _PTR_LOOP_1050_14cc, 0x50, local_58, param_2);
    pcVar17 = local_58;
    pCVar5 = str_op_1000_3da4(CONCAT22(param_2, pcVar17));
    DVar16 = GetTextExtent16(0x1000, pCVar5, pcVar17);
    // IStack90 = (DVar16 >> 0x10);
    IStack92 = DVar16;
    ReleaseDC16(ctx.s_tile2_bmp_1050_1538, HStack8);
    CreateWindow16(ctx.s_tile2_bmp_1050_1538, 0x0,
                   ZEXT24(ctx.PTR_LOOP_1050_038c) << 0x10, 0x7ce, (iVar11 + 0x6),
                   IStack90, IStack92, 0xc5, 0x22, 0x0, (s_Rebel_1050_4ffc + 0x4));
    local_64 = 0x5a000a;
    uStack96 = 0x140050;
    puVar6 = &local_64;
    create_window_1040_6eae();
    local_64 = local_64 & 0xffff | (local_64._2_2_ + 0x14) << 0x10;
    create_window_1040_6eae();
    local_64 = local_64 & 0xffff | (local_64._2_2_ + 0x14) << 0x10;
    create_window_1040_6eae();
    SendMessage16(ctx.s_tile2_bmp_1050_1538, 0x0, 0x0, 0x4010001);
    uVar1 = (iVar11 + 0x94);
    iVar7 = uVar1;
    uVar1 &= 0xffff0000;
    uVar14 = (iVar11 + 0x6);
    // uVar13 = (uVar1 >> 0x10);
    SetWindowPos16(ctx.s_tile2_bmp_1050_1538, 0x40, (iVar7 + 0x10),
                   (iVar7 + 0xe), (iVar7 + 0xc),
                   (uVar1 | iVar7 + 0xa), 0x0);
    ctx.DAT_1050_0ecc = 0x0;
    uVar2 = (iVar11 + 0x94);
    ppcVar3 = ((iVar11 + 0x94) + 0x10);
    (**ppcVar3)(ctx.s_tile2_bmp_1050_1538, uVar2, (uVar2 >> 0x10), uVar14,
                puVar6);
    pass1_1010_2ee2((iVar11 + 0x94), param_2, 0x1010);
    PostMessage16(0x1010, 0x0, 0x0, 0x111010a);
    return;
}


pub fn win_ui_op_1040_6d1a(param_1: i16, param_2: u16, param_3: u16, param_4: u32) {
    let ppcVar1: u32;
    let uVar2: u32;
    let in_DX: U32Ptr;
    let unaff_CS: u16;
    let unaff_SS: u16;
    let iVar3: i16;
    let local_a: RECT16;
    let iStack6: i16;
    let iStack4: i16;

    if (false) {
        switchD_1040_6e7b_caseD_fb: pass1_1040_b54a(param_1, param_2, param_3, param_4, in_DX, unaff_CS, unaff_SS);
        return;
    }
    unaff_CS = &ctx.PTR_LOOP_1050_1040;
    match (param_4._2_2_) {
        0xfa => {
            ppcVar1 = ((param_1 + 0x94) + 0x18);
            (**ppcVar1)();
        }
        _ =>
//     TODO: goto switchD_1040_6e7b_caseD_fb;
            0xfd => {if (ctx.DAT_1050_0ecc == 0x0) {
        return;
        }
        ctx.DAT_1050_0ecc = 0x0;
//     TODO: goto LAB_1040_6deb;
        }
        0xfe => {
            if (ctx.DAT_1050_0ecc == 0x1) {
                return;
            }
            ctx.DAT_1050_0ecc = 0x1;
        }
//     TODO: goto LAB_1040_6deb;
        0xff => {
            if (ctx.DAT_1050_0ecc == 0x2) {
                return;
            }
            ctx.DAT_1050_0ecc = 0x2;
//LAB_1040_6deb:
            uVar2 = (param_1 + 0x94);
            ppcVar1 = ((param_1 + 0x94) + 0x10);
            (**ppcVar1)(&ctx.PTR_LOOP_1050_1040, uVar2, (uVar2 >> 0x10));
            pass1_1010_2ee2((param_1 + 0x94), unaff_SS, 0x1010);
            PostMessage16(0x1010, 0x0, 0x0, 0x111010a);
        }
        0x107 => { iVar3 = 0x0; }
//     TODO: goto LAB_1040_6e48;
        0x108 => {
            iVar3 = 0x1;
//LAB_1040_6e48:
            win_ui_op_1010_3202((param_1 + 0x94), iVar3, 0x1010);
        }
        0x10a => {
            GetClientRect16(&ctx.PTR_LOOP_1050_1040, &local_a);
            uVar2 = (param_1 + 0x94);
            local_a.y += 0x3;
            local_a.x = (uVar2 + 0x1a) + -0x9;
            iStack6 += -0x3;
            iStack4 += -0x3;
            InvalidateRect16(ctx.s_tile2_bmp_1050_1538,
                             (&ctx.PTR_LOOP_1050_0000 + 0x1), &local_a);
            unk_destroy_win_op_1010_2fa0((param_1 + 0x94), 0x1010);
            pass1_1010_32c0((param_1 + 0x94), 0x0);
            pass1_1010_2ee2((param_1 + 0x94), unaff_SS, 0x1010);
        }
        0x10c => { DestroyWindow16(&ctx.PTR_LOOP_1050_1040); }
    }
    return;
}


// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn create_window_1040_6eae(
    ctx: &mut AppContext,
    param_1: u32,
    param_2: i16,
    in_menu_handle: HMENU16,
    param_4: u16,
    param_5: i16)

{
    let i_var1: i16;
    let uVar2: u16;
    let h_instance: HINSTANCE16;

    load_string_1010_847e(ctx.PTR__LOOP_1050_14cc, (ctx.PTR__LOOP_1050_14cc >> 0x10), 0x1010);
    _h_instance = 0x50000009;
    if param_2 != 0x0 {
        _h_instance = 0x50020009;
    }
    // uVar2 = (in_menu_handle >> 0x10);
    i_var1 = in_menu_handle;
    CreateWindow16(0x1010, 0x0, ZEXT24(ctx.PTR_LOOP_1050_038c) << 0x10, param_5,
                   (param_1 + 0x6), (i_var1 + 0x6),
                   (i_var1 + 0x4), (i_var1 + 0x2), *in_menu_handle,
                   _h_instance, (_h_instance >> 0x10));
    return;
}


pub fn enable_win_1040_6ff2(param_1: u32, param_2: i16, param_3: HWND16) {
    let uVar1: u32;
    let uVar2: u16;

    if (param_2 == 0x8) {
        // uVar2 = (param_1 >> 0x10);
        GetDlgItem16(param_3, 0x107);
        uVar1 = (param_1 + 0x94);
        EnableWindow16(ctx.s_tile2_bmp_1050_1538, (uVar1 + 0x24));
        GetDlgItem16(ctx.s_tile2_bmp_1050_1538, 0x108);
        uVar1 = (param_1 + 0x94);
        EnableWindow16(ctx.s_tile2_bmp_1050_1538, (uVar1 + 0x26));
    }
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn mixed_win_ui_op_1040_70b4(param_1: &mut Struct1, param_2: u16, param_3: u16) {
    let uVar1: u32;
    let uVar2: u32;
    let ppcVar3: u32;
    let paVar4: &mut Struct160;
    let mut pCVar5: String;
    let puVar6: u32;
    let iVar7: i16;
    let in_DX: U32Ptr;
    let extraout_DX: U32Ptr;
    let puVar8: U32Ptr;
    let puVar9: U32Ptr;
    let uVar10: u16;
    let iVar11: i16;
    let unaff_DI: i16;
    let uVar12: u16;
    let uVar13: u16;
    let uVar14: u16;
    let hwnd: HWND16;
    let puVar15: U32Ptr;
    let DVar16: u32;
    let mut pcVar17: String;
    let BVar18: bool;
    let local_64: u32;
    let uStack96: u32;
    let IStack92: i16;
    let IStack90: i16;
    local_58: u8[0x50];
    let HStack8: HDC16;
    let paStack6: &mut Struct160;
    let puStack4: U32Ptr;

    dialog_ui_fn_1040_78e2(param_1, param_2);
    puVar15 = mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0, 0x34, param_3, in_DX, unaff_DI);
    // uVar14 = (puVar15 >> 0x10);
    paVar4 = puVar15;
    // uVar12 = (param_1 >> 0x10);
    iVar11 = param_1;
    (iVar11 + 0x94) = paVar4;
    (iVar11 + 0x96) = uVar14;
    ppcVar3 = ((iVar11 + 0x94) + 0x4);
    (**ppcVar3)(0x1010, (iVar11 + 0x94), uVar14, 0x0, param_1);
    puVar9 = extraout_DX;
    mem_op_1000_179c(0xa, extraout_DX, 0x1000);
    puVar8 = (puVar9 | paVar4);
    if (puVar8 == 0x0) {
        (iVar11 + 0x98) = 0x0;
    } else {
        puVar15 = struct_1040_bf3e(CONCAT22(puVar9, paVar4), (iVar11 + 0x6),
        );
        // puVar8 = (puVar15 >> 0x10);
        paVar4 = puVar15;
        (iVar11 + 0x98) = paVar4;
        (iVar11 + 0x9a) = puVar8;
    }
    pass1_1040_bfde((iVar11 + 0x98), (iVar11 + 0x94), param_3);
    mem_op_1000_179c(0x42, puVar8, 0x1000);
    puVar9 = (puVar8 | paVar4);
    if (puVar9 != 0x0) {
        pass1_1008_3bd6(paVar4, puVar8, 0x1, 0xa000a, 0x0, 0x800081,
                        CONCAT22((iVar11 + 0x6), 0x10a), puVar9, param_3);
    }
    mem_op_1000_179c(0x42, puVar9, 0x1000);
    puVar8 = (puVar9 | paVar4);
    if (puVar8 != 0x0) {
        pass1_1008_3bd6(paVar4, puVar9, 0x1, 0xa0028, 0x0, 0x820083,
                        CONCAT22((iVar11 + 0x6), 0x10c), puVar8, param_3);
    }
    BVar18 = 0x42;
    uVar14 = 0x1000;
    mem_op_1000_179c(0x42, puVar8, 0x1000);
    puVar9 = (puVar8 | paVar4);
    if (puVar9 == 0x0) {
        paVar4 = 0x0;
        puVar9 = 0x0;
    } else {
        uVar14 = 0x1008;
        pass1_1008_3bd6(paVar4, puVar8, 0x1, 0xa00aa, 0x101, 0xff0100,
                        CONCAT22((iVar11 + 0x6), 0x107), puVar9, param_3);
    }
    paStack6 = paVar4;
    puStack4 = puVar9;
    enable_win_1040_9234(CONCAT22(puVar9, paVar4), BVar18, uVar14);
    BVar18 = 0x42;
    hwnd = 0x1000;
    mem_op_1000_179c(0x42, puVar9, 0x1000);
    uVar10 = puVar9 | paVar4;
    if (uVar10 == 0x0) {
        paVar4 = 0x0;
        uVar10 = 0x0;
    } else {
        hwnd = 0x1008;
        pass1_1008_3bd6(paVar4, puVar9, 0x1, 0xa00c2, 0x101, 0xff0100,
                        CONCAT22((iVar11 + 0x6), 0x108), uVar10, param_3);
    }
    paStack6 = paVar4;
    puStack4 = uVar10;
    enable_win_1040_9234(CONCAT22(uVar10, paVar4), BVar18, hwnd);
    HStack8 = GetDC16(hwnd);
    load_string_1010_84e0(0x1010, _PTR_LOOP_1050_14cc, 0x50, local_58, param_3);
    pcVar17 = local_58;
    pCVar5 = str_op_1000_3da4(CONCAT22(param_3, pcVar17));
    DVar16 = GetTextExtent16(0x1000, pCVar5, pcVar17);
    // IStack90 = (DVar16 >> 0x10);
    IStack92 = DVar16;
    CreateWindow16(ctx.s_tile2_bmp_1050_1538, 0x0,
                   ZEXT24(ctx.PTR_LOOP_1050_038c) << 0x10, 0x7cd, (iVar11 + 0x6),
                   IStack90, IStack92, 0xad, 0x22, 0x0, (s_Rebel_1050_4ffc + 0x4));
    load_string_1010_84e0(0x1010, _PTR_LOOP_1050_14cc, 0x50, local_58, param_3);
    pcVar17 = local_58;
    pCVar5 = str_op_1000_3da4(CONCAT22(param_3, pcVar17));
    DVar16 = GetTextExtent16(0x1000, pCVar5, pcVar17);
    // IStack90 = (DVar16 >> 0x10);
    IStack92 = DVar16;
    ReleaseDC16(ctx.s_tile2_bmp_1050_1538, HStack8);
    CreateWindow16(ctx.s_tile2_bmp_1050_1538, 0x0,
                   ZEXT24(ctx.PTR_LOOP_1050_038c) << 0x10, 0x7ce, (iVar11 + 0x6),
                   IStack90, IStack92, 0xc5, 0x22, 0x0, (s_Rebel_1050_4ffc + 0x4));
    local_64 = 0x5a000a;
    uStack96 = 0x140050;
    puVar6 = &local_64;
    create_window_1040_7620();
    local_64 = local_64 & 0xffff | (local_64._2_2_ + 0x14) << 0x10;
    create_window_1040_7620();
    local_64 = local_64 & 0xffff | (local_64._2_2_ + 0x14) << 0x10;
    create_window_1040_7620();
    SendMessage16(ctx.s_tile2_bmp_1050_1538, 0x0, 0x0, 0x4010001);
    uVar1 = (iVar11 + 0x94);
    iVar7 = uVar1;
    uVar1 &= 0xffff0000;
    uVar14 = (iVar11 + 0x6);
    // uVar13 = (uVar1 >> 0x10);
    SetWindowPos16(ctx.s_tile2_bmp_1050_1538, 0x40, (iVar7 + 0x10),
                   (iVar7 + 0xe), (iVar7 + 0xc),
                   (uVar1 | iVar7 + 0xa), 0x0);
    ctx.DAT_1050_0ecc = 0x0;
    uVar2 = (iVar11 + 0x94);
    ppcVar3 = ((iVar11 + 0x94) + 0x10);
    (**ppcVar3)(ctx.s_tile2_bmp_1050_1538, uVar2, (uVar2 >> 0x10), uVar14,
                puVar6);
    pass1_1010_2ee2((iVar11 + 0x94), param_3, 0x1010);
    PostMessage16(0x1010, 0x0, 0x0, 0x111010a);
    return;
}


pub fn win_cleanup_op_1040_748c(param_1: i16, param_2: u16, param_3: u16, param_4: u32) {
    let ppcVar1: u32;
    let uVar2: u32;
    let in_DX: U32Ptr;
    let unaff_CS: u16;
    let unaff_SS: u16;
    let iVar3: i16;
    let local_a: RECT16;
    let iStack6: i16;
    let iStack4: i16;

    if (false) {
        switchD_1040_75ed_caseD_fb: pass1_1040_b54a(param_1, param_2, param_3, param_4, in_DX, unaff_CS, unaff_SS);
        return;
    }
    unaff_CS = &ctx.PTR_LOOP_1050_1040;
    match (param_4._2_2_) {
        0xfa => {
            ppcVar1 = ((param_1 + 0x94) + 0x18);
            (**ppcVar1)();
        }
        _ => {}
//     TODO: goto switchD_1040_75ed_caseD_fb;
        0xfd => {
            if (ctx.DAT_1050_0ecc == 0x0) {
                return;
            }
            ctx.DAT_1050_0ecc = 0x0;
        }
//     TODO: goto LAB_1040_755d;
        0xfe => {
            if (ctx.DAT_1050_0ecc == 0x1) {
                return;
            }
            ctx.DAT_1050_0ecc = 0x1;
        }
//     TODO: goto LAB_1040_755d;
        0xff => {
            if (ctx.DAT_1050_0ecc == 0x2) {
                return;
            }
            ctx.DAT_1050_0ecc = 0x2;
//LAB_1040_755d:
            uVar2 = (param_1 + 0x94);
            ppcVar1 = ((param_1 + 0x94) + 0x10);
            (**ppcVar1)(&ctx.PTR_LOOP_1050_1040, uVar2, (uVar2 >> 0x10));
            pass1_1010_2ee2((param_1 + 0x94), unaff_SS, 0x1010);
            PostMessage16(0x1010, 0x0, 0x0, 0x111010a);
        }
        0x107 => { iVar3 = 0x0; }
//     TODO: goto LAB_1040_75ba;
        0x108 => {
            iVar3 = 0x1;
//LAB_1040_75ba:
            win_ui_op_1010_3202((param_1 + 0x94), iVar3, 0x1010);
            break;
        }
        0x10a => {
            GetClientRect16(&ctx.PTR_LOOP_1050_1040, &local_a);
            uVar2 = (param_1 + 0x94);
            local_a.y += 0x3;
            local_a.x = (uVar2 + 0x1a) + -0x9;
            iStack6 += -0x3;
            iStack4 += -0x3;
            InvalidateRect16(ctx.s_tile2_bmp_1050_1538,
                             (&ctx.PTR_LOOP_1050_0000 + 0x1), &local_a);
            unk_destroy_win_op_1010_2fa0((param_1 + 0x94), 0x1010);
            pass1_1010_32c0((param_1 + 0x94), 0x0);
            pass1_1010_2ee2((param_1 + 0x94), unaff_SS, 0x1010);
        }
        0x10c => { DestroyWindow16(&ctx.PTR_LOOP_1050_1040); }
    }
    return;
}


// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn create_window_1040_7620(param_1: u32, param_2: i16, HMENin_menu_handle: U32Ptr, param_4: u16,
                               param_5: i16)

{
    let i_var1: i16;
    let uVar2: u16;
    let h_instance: HINSTANCE16;

    load_string_1010_847e(ctx.PTR__LOOP_1050_14cc, (ctx.PTR__LOOP_1050_14cc >> 0x10), 0x1010);
    _h_instance = 0x50000009;
    if (param_2 != 0x0) {
        _h_instance = 0x50020009;
    }
    // uVar2 = (in_menu_handle >> 0x10);
    i_var1 = in_menu_handle;
    CreateWindow16(0x1010, 0x0, ZEXT24(ctx.PTR_LOOP_1050_038c) << 0x10, param_5,
                   (param_1 + 0x6), (i_var1 + 0x6),
                   (i_var1 + 0x4), (i_var1 + 0x2), *in_menu_handle,
                   _h_instance, (_h_instance >> 0x10));
    return;
}


pub fn ui_cleanup_op_1040_782c(param_1: &mut Struct18, param_2: HGDIOBJ16) {
    let pu_var1: u32;
    let uVar2: u16;
    let ppcVar3: u32;
    let iVar4: i16;
    let uVar5: u16;
    let menu: HGDIOBJ16;
    let hwnd: HMENU16;

    // uVar5 = (param_1 >> 0x10);
    iVar4 = param_1;
    param_1.field_0x0 = 0x840c;
    (iVar4 + 0x2) = &ctx.PTR_LOOP_1050_1040;
    pu_var1 = (iVar4 + 0x70);
    uVar2 = (iVar4 + 0x72);
    if ((uVar2 | pu_var1) != 0x0) {
        ppcVar3 = *pu_var1;
        (**ppcVar3)(param_2, pu_var1, uVar2, 0x1);
    }
    menu = param_2;
    if ((iVar4 + 0x4) != 0x0) {
        menu = ctx.s_tile2_bmp_1050_1538;
        DeleteObject16(param_2);
        (iVar4 + 0x4) = 0x0;
    }
    hwnd = menu;
    if ((iVar4 + 0x68) != 0x0) {
        hwnd = ctx.s_tile2_bmp_1050_1538;
        DestroyMenu16(menu);
    }
    RemoveProp16(hwnd, s_thisLo_1050_5db1);
    RemoveProp16(ctx.s_tile2_bmp_1050_1538, s_thisHi_1050_5db8);
    RemoveProp16(ctx.s_tile2_bmp_1050_1538, s_procLo_1050_5dbf);
    RemoveProp16(ctx.s_tile2_bmp_1050_1538, s_procHi_1050_5dc6);
    param_1.field_0x0 = 0x389a;
    (iVar4 + 0x2) = 0x1008;
    return;
}


pub fn dialog_ui_fn_1040_78e2(in_struct_1: &mut Struct1, in_instance_handle: HINSTANCE16) {
    let ppcVar1: u32;
    let mut dlg_template: String;
    let dialog_handle: HWND16;
    let local_struct_1: &mut Struct1;
    let local_string_1: &mut Struct1;
    let uVar2: u16;
    let lVar3: i32;
    let uVar4: u16;
    let uVar5: u16;
    let uVar6: u16;
    let uVar7: u16;
    let uVar8: u16;
    let uVar9: u16;
    let uVar10: u16;
    let mut local_string_2: String;
    let mut pCStack8: String;

    // local_string_1 = (in_struct_1 >> 0x10);
    local_struct_1 = in_struct_1;
    if (&local_struct_1.field_0xc == 0x0) {
        uVar2 = (ctx.PTR__LOOP_1050_5bc8 >> 0x10);
        dlg_template = (ctx.PTR__LOOP_1050_5bc8 + 0x4);
        dialog_handle = (ctx.PTR__LOOP_1050_5bc8 + 0x6);
    } else {
        dlg_template = &local_struct_1.field_0xc;
        dialog_handle = &local_struct_1.field_0xe;
    }
    dialog_handle = CreateDialog16(in_instance_handle, dlg_template, dialog_handle,
                                   local_struct_1.lpvoid_field_0x8);
    &local_struct_1.field_0x6 = dialog_handle;
    GetWindowText16(ctx.s_tile2_bmp_1050_1538, 0x50,
                    &local_struct_1.max_count_field_0x10);
    lVar3 = GetWindowLong16(ctx.s_tile2_bmp_1050_1538, -0x4);
    SetWindowLong16(ctx.s_tile2_bmp_1050_1538, _PTR_LOOP_1050_5bcc,
                    CONCAT22(0xfffc, (ctx.PTR__LOOP_1050_5bcc >> 0x10)));
    uVar2 = &local_struct_1.field_0x6;
    uVar10 = SUB42(ctx.data_seg, 0x0);
    SetProp16(ctx.s_tile2_bmp_1050_1538, local_struct_1,
              (HANDLE16)s_thisLo_1050_5dcd);
    uVar9 = &local_struct_1.field_0x6;
    uVar8 = SUB42(ctx.data_seg, 0x0);
    SetProp16(ctx.s_tile2_bmp_1050_1538, local_string_1,
              (HANDLE16)s_thisHi_1050_5dd4);
    local_string_2 = lVar3;
    uVar7 = &local_struct_1.field_0x6;
    uVar6 = SUB42(ctx.data_seg, 0x0);
    SetProp16(ctx.s_tile2_bmp_1050_1538, local_string_2, (HANDLE16)s_procLo_1050_5ddb);
    // pCStack8 = (lVar3 >> 0x10);
    uVar5 = &local_struct_1.field_0x6;
    uVar4 = SUB42(ctx.data_seg, 0x0);
    SetProp16(ctx.s_tile2_bmp_1050_1538, pCStack8, (HANDLE16)s_procHi_1050_5de2);
    ppcVar1 = (in_struct_1.field_0x0 + 0x50);
    (**ppcVar1)(ctx.s_tile2_bmp_1050_1538, in_struct_1, uVar4, uVar5, uVar6, uVar7, uVar8, uVar9,
                uVar10, uVar2);
    return;
}


pub fn post_win_msg_1040_7b3c(
    param_1: U32Ptr,
    param_2: u16,
    param_3: u16,
    param_4: i16,
    win_handle: HWND16,
) -> bool

{
    let ppc_var1: u32;

    if (param_4 == 0x1) || (param_4 == 0x2) {
        ppc_var1 = (*param_1 + 0x14);
        (**ppc_var1)();
    } else {
        if param_4 == 0x6f {
            ppc_var1 = (*param_1 + 0x2c);
            (**ppc_var1)(win_handle, param_1);
        } else {
            if param_4 != 0x12e {
                return false;
            }
            PostMessage16(win_handle, 0x0, 0x0, 0x112f060);
        }
    }
    return true;
}


pub fn post_win_msg_1040_7f1c(param_1: u32, param_2: i16, param_3: HWND16) -> bool

{
    let i_var1: i16;
    let uVar2: u16;

    // uVar2 = (param_1 >> 0x10);
    i_var1 = param_1;
    if ((i_var1 + 0x78) != 0x0) {
        return 0x0;
    }
    if ((i_var1 + 0x60) != param_2) {
        (i_var1 + 0x60) = param_2;
        PostMessage16(param_3, 0x0, 0x0, 0x850000);
    }
    return 0x1;
}


pub fn post_win_msg_1040_7f56(param_1: u32, param_2: &mut String) {
    string_1000_3d3e((param_1 & 0xffff0000 | (param_1 + 0x10)), param_2);
    PostMessage16(0x1000, 0x0, 0x0, 0x850000);
    return;
}


pub fn menu_ui_op_1040_7f86(param_1: u32, param_2: HWND16, param_3: &RECT16) {
    let HVar1: HMENU16;
    let iVar2: i16;
    let uVar3: u16;
    let unaff_CS: HWND16;
    POlet
    local_6: i16;

    // uVar3 = (param_1 >> 0x10);
    iVar2 = param_1;
    if (((iVar2 + 0x6a) != 0x0) && ((iVar2 + 0x68) == 0x0)) {
        HVar1 = LoadMenu16(unaff_CS, (iVar2 + 0x6a));
        (iVar2 + 0x68) = HVar1;
        if (HVar1 == 0x0) {
            return;
        }
        unaff_CS = ctx.s_tile2_bmp_1050_1538;
        HVar1 = GetSubMenu16(ctx.s_tile2_bmp_1050_1538, 0x0);
        (iVar2 + 0x68) = HVar1;
        if (HVar1 == 0x0) {
            return;
        }
    }
    local_6.x = param_3;
    local_6.y = param_2;
    ClientToScreen16(unaff_CS, &local_6);
    TrackPopupMenu16(ctx.s_tile2_bmp_1050_1538, 0x0, 0x0, (iVar2 + 0x6), 0x0,
                     local_6.y, local_6.x);
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn win_help_1040_800c(param_1: u32, param_2: u16) {
    let uVar1: u16;
    let paVar2: &mut Struct43;
    let mut lp_help_file: String;
    let w_command: u16;
    let uVar3: u16;

    paVar2 = unk_io_op_1010_830a(ctx.PTR__LOOP_1050_14cc, 0x1f8, param_2);
    // uVar1 = (param_1 >> 0x10);
    if ((param_1 + 0x8a) == 0x0) {
        w_command = 0x0;
        uVar3 = 0x3;
        lp_help_file = 0x0;
    } else {
        uVar3 = 0x1;
        lp_help_file = (param_1 + 0x8a);
        w_command = lp_help_file >> 0xf;
    }
    WinHelp16(0x1010, lp_help_file, w_command, CONCAT22(paVar2, uVar3));
    return;
}


pub fn unk_win_ui_op_1040_8158(param_1: U32Ptr, POparam_2: i16, param_3: i16, param_4: HWND16) {
    let ppcVar1: u32;
    let IVar2: i16;
    let BVar3: bool;
    let uVar4: u32;
    let iVar5: i16;
    let uVar6: u16;
    let uVar7: u16;
    POlet
    local_6: i16;

    if (param_3 == 0x2) {
        // uVar6 = (param_1 >> 0x10);
        iVar5 = param_1;
        if ((iVar5 + 0x76) != 0x0) {
            local_6 = param_2;
            uVar6 = (iVar5 + 0x6);
            ScreenToClient16(param_4, &local_6);
            uVar7 = 0x4;
            IVar2 = GetSystemMetrics16(s_tile2_bmp_1050_1538);
            local_6 = (local_6 & 0xffff | (local_6.y + IVar2) << 0x10);
            uVar4 = param_1 & 0xffff0000 | (iVar5 + 0x82);
            BVar3 = PtInRect16(s_tile2_bmp_1050_1538, local_6);
            if (BVar3 != 0x0) {
                ppcVar1 = (*param_1 + 0x14);
                (**ppcVar1)(ctx.s_tile2_bmp_1050_1538, param_1, 0x0, uVar4, uVar7, uVar6);
            }
        }
    }
    return;
}


pub fn check_dialog_msg_1040_81b6(param_1: u32, param_2: HWND16) {
    let BVar1: bool;
    MSG16
    local_14;

    (param_1 + 0x78) = 0x1;
    loop {
        BVar1 = IsWindow16(param_2);
        if (BVar1 == 0x0) {
            return;
        }
        BVar1 = GetMessage16(s_tile2_bmp_1050_1538, 0x0, 0x0, 0x0);
        if (BVar1 == 0x0) { break; }
        param_2 = ctx.s_tile2_bmp_1050_1538;
        IsDialogMessage16(ctx.s_tile2_bmp_1050_1538, &local_14);
    }
    return;
}


pub fn win_ui_op_1040_81fe(param_1: HWND16) {
    SetSysModalWindow(param_1);
    return;
}


pub fn move_win_1040_826c(param_1: &mut Struct1, param_2: i16, param_3: bool) {
    let IVar1: i16;
    let unaff_CS: HWND16;
    let local_e: RECT16;
    let uStack10: i16;
    let iStack8: i16;
    let IStack6: i16;
    let BStack4: bool;

    GetWindowRect16(unaff_CS, &local_e);
    if ((param_3 == 0xffff) || (param_2 == -0x1)) {
        IVar1 = GetSystemMetrics16(s_tile2_bmp_1050_1538);
        BStack4 = (IVar1 - (iStack10 - local_e.x)) / 0x2;
        IVar1 = GetSystemMetrics16(s_tile2_bmp_1050_1538);
        param_2 = (IVar1 - (iStack8 - local_e.y)) / 0x2;
    } else {
        BStack4 = param_3;
    }
    IStack6 = param_2;
    MoveWindow16(ctx.s_tile2_bmp_1050_1538, 0x0, iStack8 - local_e.y, iStack10 - local_e.x,
                 param_2, BStack4);
    return;
}


pub fn enable_win_1040_86dc(param_1: HWND16) {
    let HVar1: HWND16;

    HVar1 = GetDlgItem16(param_1, 0x1);
    if (HVar1 != 0x0) {
        EnableWindow16(ctx.s_tile2_bmp_1050_1538, 0x1);
        HVar1 = GetDlgItem16(ctx.s_tile2_bmp_1050_1538, 0x2);
        if (HVar1 != 0x0) {
            EnableWindow16(ctx.s_tile2_bmp_1050_1538, 0x1);
        }
    }
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

uchar *  win_ui_op_1040_8718(param_1: & mut Struct37,param_2: u16)

{
let piVar1: * mut i16; let iVar2: i16; let uVar3: u16; let extraout_DX: * mut u8; let puVar4: * mut u8; let unaff_DI: i16; let uVar5: u16; let puVar6: * mut u16; let uVar7: u16; let uVar9: u16; let UVar10: u32; i16 local_104 [0x80]; let uStack4: u16; let uVar8: u16;

uVar5 = 0x1008; unk_win_msg_op_1008_9510( & ctx.PTR_LOOP_1050_5df4,0x1008, param_2); UVar10 = param_1;
// uVar8 = (param_1 >> 0x10);
dialog_ui_fn_1040_78e2(param_1, 0x1008); ctx.PTR_LOOP_1050_5df6 = (UVar10 + 0x6); if ((UVar10 + 0x94) != 0x0) {
uVar5 = 0x1000; unk_str_op_1000_3d3e
((param_1 & 0xffff0000 | (UVar10 + 0x10)),
(UVar10 + 0x94));
}
get_sys_metrics_1040_8c66(param_1, uVar5); uStack4 = (UVar10 + 0x98) & 0xf; if (uStack4 == 0x1) {
(UVar10 + 0xae) = ((UVar10 + 0xaa) + - 0xc4) / 0x2; load_string_1010_84e0
(0x1010, _PTR_LOOP_1050_14cc,
(ctx.PTR__LOOP_1050_14cc >> 0x10), 0xff, local_104, param_2
); create_window_1040_8bea(UVar10, uVar8, 0x1, 0x1, (UVar10 + 0xae)); piVar1 = (UVar10 + 0xae); * piVar1 = *piVar1 + 0x6c; load_string_1010_84e0
(0x1010, _PTR_LOOP_1050_14cc,
(ctx.PTR__LOOP_1050_14cc > > 0x10), 0xff, local_104, param_2
); uVar9 = (UVar10 + 0xae); uVar7 = 0x2;
}
else {
if (uStack4 != 0x4) {
(UVar10 + 0xae) = ((UVar10 + 0xaa) + - 0x58) / 0x2; load_string_1010_84e0
(0x1010, _PTR_LOOP_1050_14cc,
(ctx.PTR__LOOP_1050_14cc >> 0x10), 0xff, local_104,
param_2); uVar9 = (UVar10 + 0xae); uVar5 = 0x1; uVar7 = 0x1;
//       TODO: goto LAB_1040_88a5;
}
(UVar10 + 0xae) = ((UVar10 + 0xaa) + - 0xc4) / 0x2; load_string_1010_84e0
(0x1010, _PTR_LOOP_1050_14cc,
(ctx.PTR__LOOP_1050_14cc >> 0x10), 0xff, local_104, param_2
); create_window_1040_8bea(UVar10, uVar8, 0x1, 0x6, (UVar10 + 0xae)); piVar1 = (UVar10 + 0xae); * piVar1 = *piVar1 + 0x6c; load_string_1010_84e0
(0x1010, _PTR_LOOP_1050_14cc,
(ctx.PTR__LOOP_1050_14cc > > 0x10), 0xff, local_104, param_2
); uVar9 = (UVar10 + 0xae); uVar7 = 0x7;
}
uVar5 = 0x0;
//LAB_1040_88a5:
create_window_1040_8bea(UVar10, uVar8,uVar5, uVar7, uVar9); puVar6 = mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0, 0x48, param_2, extraout_DX, unaff_DI);
// uVar5 = (puVar6 >> 0x10);
local_104[0] = (puVar6 + 0xa); uStack4 = (puVar6 + 0xc); iVar2 = uStack4 - (UVar10 + 0xac); puVar4 = (iVar2 > > 0xf); SetWindowPos16(0x1010, 0x40, (UVar10 + 0xac), (UVar10 + 0xaa),
iVar2 / 0x2, (local_104[0] - (UVar10 + 0xaa)) / 0x2, 0x0); ctx.PTR_LOOP_1050_5df4 = ( & ctx.PTR_LOOP_1050_0000 + 0x1); unk_win_msg_op_1008_9510( & ctx.PTR_LOOP_1050_5df4, 0x1008, param_2); destroy_win_1040_8b7e(0x1008); ctx.PTR_LOOP_1050_5df6 = 0x0; if ((UVar10 + 0xb2) != 0x0) {
puVar6 = mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0, 0x37,param_2, puVar4, unaff_DI); uVar3 = pass1_1008_ab54(puVar6); if (uVar3 != 0x0) {
PostMessage16(0x1008, 0x0, 0x0, 0x11100fc);
}
}
return ctx.PTR_LOOP_1050_5df8;
}



HANDLE16
create_window_1040_8bea
(param_1: u32,param_2: u16,param_3: i16,param_4: i16,HMENparam_5: u16)

{
let HVar1: HANDLE16; let mut unaff_CS: String; let LVar2: LRESULT; let in_stack_0000000e: HWND16; let uStack6: u32;

uStack6 = 0x50010000;
if (param_3 != 0x0) {
uStack6 = 0x50010001;
}
if ((param_1 + 0x74) != 0x0) {
uStack6 |= 0x8000000;
}
CreateWindow16(unaff_CS, 0x0, ZEXT24(ctx.PTR_LOOP_1050_038c) < < 0x10, param_4,
(param_1 + 0x6), 0x17, 0x58, in_stack_0000000e,param_5,
uStack6, (uStack6 > > 0x10)); HVar1 = GetProp16(ctx.s_tile2_bmp_1050_1538,0x5e09); if (HVar1 != 0x0) {
LVar2 = SendMessage16(ctx.s_tile2_bmp_1050_1538, 0x1,0x0, CONCAT22(0x30, HVar1));
HVar1 = (HANDLE16)LVar2;
}
return HVar1;
}


pub fn enable_window_1040_8ea0(param_1: u16, param_2: u16, param_3: u16, param_4: u32, param_5: u16)

{
    let enable: HWND16;
    let in_DX: U32Ptr;
    let unaff_SS: u16;

    if (param_4._2_2_ == 0xf8) {
        GetDlgItem16(param_5, 0x17d8);
        enable = 0x1;
    } else {
        if (param_4._2_2_ != 0x17d8) {
            pass1_1040_b54a(param_1, param_2, param_3, param_4, in_DX, param_5, unaff_SS);
            return;
        }
        SetWindowPos16(param_5, 0x6, 0xf6, 0x269, 0x0, 0x0, 0x0);
        enable = GetDlgItem16(ctx.s_tile2_bmp_1050_1538, 0x17d8);
    }
    EnableWindow16(ctx.s_tile2_bmp_1050_1538, enable);
    return;
}


pub fn mix_win_ui_op_1040_911e(param_1: &mut Struct18) {
    let pu_var1: u32;
    let uVar2: u16;
    let uVar3: u32;
    let ppcVar4: u32;
    let iVar5: i16;
    let uVar6: u16;

    // uVar6 = (param_1 >> 0x10);
    iVar5 = param_1;
    *param_1 = 0x9800;
    (iVar5 + 0x2) = &ctx.PTR_LOOP_1050_1040;
    if ((iVar5 + 0x38) != 0x0) {
        pu_var1 = (iVar5 + 0x8);
        uVar2 = (iVar5 + 0xa);
        if ((uVar2 | pu_var1) != 0x0) {
            ppcVar4 = *pu_var1;
            (**ppcVar4)();
        }
        pu_var1 = (iVar5 + 0xc);
        uVar2 = (iVar5 + 0xe);
        if ((uVar2 | pu_var1) != 0x0) {
            ppcVar4 = *pu_var1;
            (**ppcVar4)();
        }
        pu_var1 = (iVar5 + 0x10);
        uVar2 = (iVar5 + 0x12);
        if ((uVar2 | pu_var1) != 0x0) {
            ppcVar4 = *pu_var1;
            (**ppcVar4)();
        }
    }
    fn_ptr_1000_17ce(ctx, (iVar5 + 0x4), 0x1000);
    uVar3 = (iVar5 + 0x14);
    SetWindowLong16(0x1000, uVar3, CONCAT22(0xfffc, (uVar3 >> 0x10)));
    RemoveProp16(ctx.s_tile2_bmp_1050_1538, s_thisLo_1050_5e1c);
    RemoveProp16(ctx.s_tile2_bmp_1050_1538, s_thisHi_1050_5e23);
    RemoveProp16(ctx.s_tile2_bmp_1050_1538, s_procLo_1050_5e2a);
    RemoveProp16(ctx.s_tile2_bmp_1050_1538, s_procHi_1050_5e31);
    RemoveProp16(ctx.s_tile2_bmp_1050_1538, 0x5e38);
    ctx.PTR_LOOP_1050_5e16 = ctx.PTR_LOOP_1050_5e16 + -0x1;
    if (ctx.PTR_LOOP_1050_5e16 == 0x0) {
        FreeProcInstance16(s_tile2_bmp_1050_1538);
        ctx._PTR_LOOP_1050_5e18 = 0x0;
    }
    *param_1 = 0x389a;
    (iVar5 + 0x2) = 0x1008;
    return;
}


pub fn enable_win_1040_9234(param_1: u32, param_2: bool, param_3: HWND16) {
    if ((param_1 + 0x18) != 0x0) {
        EnableWindow16(param_3, param_2);
    }
    return;
}


pub fn create_window_1040_92dc(param_1: u32, param_2: u16) {
    let HVar1: HWND16;
    let mut str: String;
    let mut str_00: String;
    let mut str_01: String;
    let lVar2: i32;

    // str_01 = (param_1 >> 0x10);
    str_00 = param_1;
    if ((str_00 + 0x18) == 0x0) {
        HVar1 = CreateWindow16(param_2, 0x0, ZEXT24(ctx.PTR_LOOP_1050_038c) << 0x10,
                               (str_00 + 0x1c), (str_00 + 0x1a), 0x0, 0x0,
                               (str_00 + 0x20), (str_00 + 0x1e), 0xb,
                               0x4000);
        (str_00 + 0x18) = HVar1;
        lVar2 = SetWindowLong16(ctx.s_tile2_bmp_1050_1538, _PTR_LOOP_1050_5e18,
                                CONCAT22(0xfffc, (ctx.PTR__LOOP_1050_5e18 >> 0x10)));
        // str = (lVar2 >> 0x10);
        (str_00 + 0x14) = lVar2;
        (str_00 + 0x16) = str;
        SetProp16(ctx.s_tile2_bmp_1050_1538, str, (HANDLE16)s_procHi_1050_5e46);
        SetProp16(ctx.s_tile2_bmp_1050_1538, (str_00 + 0x14),
                  (HANDLE16)s_procLo_1050_5e4d);
        SetProp16(ctx.s_tile2_bmp_1050_1538, str_01, (HANDLE16)s_thisHi_1050_5e54);
        SetProp16(ctx.s_tile2_bmp_1050_1538, str_00, (HANDLE16)s_thisLo_1050_5e5b);
        if ((str_00 + 0x40) != 0x0) {
            SetProp16(ctx.s_tile2_bmp_1050_1538, (&ctx.PTR_LOOP_1050_0000 + 0x1),
                      0x5e62);
        }
        ShowWindow16(ctx.s_tile2_bmp_1050_1538, 0x5);
    }
    return;
}

pub fn mov_update_win_1040_93aa(
    ctx: &mut AppContext,
    param_1: &mut Struct65,
    param_2: i16,
    param_3: u16,
    param_4: HWND16) {
    param_1.field_0x1e = param_3;
    param_1.field_0x20 = param_2;
    MoveWindow16(param_4, 0x1, param_1.field_0x24, param_1.field_0x22, param_2, param_1.field_0x1e);
    UpdateWindow16(ctx.s_tile2_bmp_1050_1538 as HWND16);
    return;
}


pub fn send_msg_1040_9404(param_1: u32, param_2: HWND16) -> LRESULT

{
    let l_var1: LRESULT;

    l_var1 = SendMessage16(param_2, 0x0, 0x0,
                           CONCAT22(0x111, (param_1 + 0x1c)));
    return l_var1;
}


pub fn win_ui_get_prop_op_1040_9566(param_1: &mut i16, param_2: HWND16) {
    let uVar1: u16;
    let iVar2: i16;
    let ppcVar3: u32;
    let HVar4: HANDLE16;
    let HVar5: HANDLE16;
    let iVar6: i16;
    let uVar7: u16;
    let uVar8: u16;
    let uVar9: u16;
    let puStack12: u32;

    // uVar7 = (param_1 >> 0x10);
    iVar6 = param_1;
    if (*param_1 == 0x4) {
        uVar1 = (iVar6 + 0xc);
        uVar9 = (iVar6 + 0xa);
        HVar4 = GetProp16(param_2, s_thisHi_1050_5e6f);
        uVar8 = (iVar6 + 0xa);
        HVar5 = GetProp16(ctx.s_tile2_bmp_1050_1538, s_thisLo_1050_5e68);
        puStack12 = CONCAT22(HVar4, HVar5);
        if ((HVar4 | HVar5) != 0x0) {
            iVar2 = (iVar6 + 0x6);
            if (iVar2 == 0x1) {
                ppcVar3 = (*puStack12 + 0xc);
                (**ppcVar3)(ctx.s_tile2_bmp_1050_1538, HVar5, HVar4, (iVar6 + 0x8),
                            uVar1, uVar8, uVar9);
                return;
            }
            if (iVar2 == 0x2) {
                ppcVar3 = (*puStack12 + 0x10);
                (**ppcVar3)(ctx.s_tile2_bmp_1050_1538, HVar5, HVar4, (iVar6 + 0x8),
                            uVar1);
                return;
            }
            if (iVar2 == 0x4) {
                ppcVar3 = (*puStack12 + 0x18);
                (**ppcVar3)(ctx.s_tile2_bmp_1050_1538, HVar5, HVar4, (iVar6 + 0x8) & 0x10,
                            uVar1);
                return;
            }
        }
    }
    return;
}


pub fn call_win_proc_1040_9684(param_1: HWND16, param_2: u16, param_3: WPARAM16, param_4: LPARAM, param_5: HWND16,
                               param_6: u16)

{
    let ppcVar1: u32;
    let HVar2: HANDLE16;
    let HVar3: HANDLE16;
    let Bvar4: bool;
    let pRVar5: *mut RECT16;
    let uVar6: u32;
    let uVar7: u16;
    let uVar8: u16;
    let uVar9: u16;
    let uVar10: u16;
    let uVar11: u16;
    let local_1a: [RECT16; 0x2];
    let puStack18: u32;
    let puStack14: u32;
    let puStack10: u32;
    let lStack6: i32;

    uVar11 = SUB42(ctx.data_seg, 0x0);
    uVar10 = param_4._2_2_;
    HVar2 = GetProp16(param_5, s_procHi_1050_5e7d);
    uVar9 = param_4._2_2_;
    HVar3 = GetProp16(ctx.s_tile2_bmp_1050_1538, s_procLo_1050_5e76);
    lStack6 = CONCAT22(HVar2, HVar3);
    uVar8 = param_4._2_2_;
    HVar2 = GetProp16(ctx.s_tile2_bmp_1050_1538, s_thisHi_1050_5e8b);
    uVar7 = param_4._2_2_;
    HVar3 = GetProp16(ctx.s_tile2_bmp_1050_1538, s_thisLo_1050_5e84);
    puStack10 = CONCAT22(HVar2, HVar3);
    if ((HVar2 | HVar3) != 0x0) {
        if (param_4 == 0x2) {
            puStack18 = puStack10;
            puStack14 = puStack10;
            if (puStack10 != 0x0) {
                ppcVar1 = *puStack10;
                (**ppcVar1)(ctx.s_tile2_bmp_1050_1538, HVar3, HVar2, 0x1, uVar7, uVar8, uVar9, uVar10,
                            uVar11);
            }
        } else {
            if (param_4 == 0x201) {
                HVar2 = GetProp16(ctx.s_tile2_bmp_1050_1538, 0x5e92);
                if (HVar2 == 0x0) {
                    uVar7 = (puStack10 + 0x18);
                    GetClientRect16(ctx.s_tile2_bmp_1050_1538, local_1a);
                    pRVar5 = local_1a;
                    uVar6 = CONCAT22(uVar7, param_6);
                    BVar4 = PtInRect16(s_tile2_bmp_1050_1538,
                                       CONCAT13((param_2 >> 0x8),
                                                CONCAT12(param_2, param_1)));
                    if (BVar4 == 0x0) {
                        return;
                    }
                    debug_print_1008_6048(CONCAT22(param_6, 0x5e98), 0x1008, param_6);
                    ppcVar1 = (*puStack10 + 0x1c);
                    (**ppcVar1)(0x1008, puStack10, (puStack10 >> 0x10), param_2,
                                param_1, param_3, pRVar5, uVar6, param_4._2_2_);
                    return;
                }
            } else {
                if (param_4 == 0x204) {
                    uVar7 = (HVar3 + 0x18);
                    GetClientRect16(ctx.s_tile2_bmp_1050_1538, local_1a);
                    uVar6 = CONCAT22(param_6, local_1a);
                    BVar4 = PtInRect16(s_tile2_bmp_1050_1538,
                                       CONCAT22(param_2, param_1));
                    if (BVar4 == 0x0) {
                        return;
                    }
                    debug_print_1008_6048(CONCAT22(param_6, 0x5eab), 0x1008, param_6);
                    ppcVar1 = (*puStack10 + 0x20);
                    (**ppcVar1)(0x1008, puStack10, (puStack10 >> 0x10), param_2,
                                param_1, param_3, uVar6, uVar7);
                    return;
                }
            }
        }
    }
    if (lStack6 != 0x0) {
        CallWindowProc16(s_tile2_bmp_1050_1538, param_1, param_2, param_3, param_4);
    }
    return;
}


pub fn unk_win_ui_op_1040_9854(param_1: U32Ptr, param_2: u16) -> u16

{
    let HVar1: HCURSOR16;
    let HVar2: HGDIOBJ16;
    let iVar3: i16;
    let uVar4: u16;

    // uVar4 = (param_1 >> 0x10);
    iVar3 = param_1;
    *param_1 = 0x389a;
    (iVar3 + 0x2) = 0x1008;
    *param_1 = 0xa230;
    (iVar3 + 0x2) = &ctx.PTR_LOOP_1050_1040;
    string_1000_3d3e((param_1 & 0xffff0000 | (iVar3 + 0x4)),
                     s_OPButton_1050_5ece);
    (iVar3 + 0x54) = 0x3;
    HVar1 = LoadCursor16(0x1000, 0x7f00);
    (iVar3 + 0x58) = HVar1;
    HVar2 = GetStockObject16(s_tile2_bmp_1050_1538);
    (iVar3 + 0x56) = HVar2;
    reg_class_1040_98c0(param_1 & 0xffff | uVar4 << 0x10,
                        ctx.s_tile2_bmp_1050_1538, param_2);
    return param_1;
}


pub fn win_op_1040_9cde(param_1: u16, param_2: WPARAM16, param_3: i16, param_4: u32, param_5: HWND16,
                        param_6: u16)

{
    let pbVar1: U32Ptr;
    let iVar2: i16;
    let uVar3: u16;
    let uVar4: u16;
    let iVar5: i16;
    let IVar6: i16;
    let BVar7: bool;
    let offset: u16;
    let puVar8: U32Ptr;
    let uVar9: u16;
    let uVar10: u16;
    let HVar11: HWND16;
    let paVar12: &mut Struct18;
    INT16 * pIVar13;
    let LVar14: LRESULT;
    let uVar15: u32;
    let bVar16: u8;
    let uStack30: u16;
    let local_a: [RECT16; 0x2];

    HVar11 = ctx.s_tile2_bmp_1050_1538;
    paVar12 = GetWindowLong16(param_5, 0x0);
    // puVar8 = (paVar12 >> 0x10);
    iVar5 = paVar12;
    uVar10 = ((paVar12 & 0xffff0000) >> 0x10);
    if (param_4 == 0x30) {
        (iVar5 + 0x5a) = param_3;
    } else {
        bVar16 = param_4;
        if (param_4 < 0x31) {
            if (param_4 == 0x1f) {
                (iVar5 + 0x4) = 0x0;
                ReleaseCapture16();
                return;
            }
            if (0x1f < param_4) {
                // goto
                // LAB_1040_a1ae;
            }
            if (bVar16 == 0x8) {
                pbVar1 = (iVar5 + 0x4);
                *pbVar1 = *pbVar1 & 0xf7;
                uStack30 = 0x0;
                BVar7 = IsWindow16(ctx.s_tile2_bmp_1050_1538);
                if (BVar7 != 0x0) {
                    uVar15 = SendMessage16(ctx.s_tile2_bmp_1050_1538, 0x0, 0x0, 0x870000);
                    uStack30 = ((uVar15 & 0x20) == 0x0);
                }
                (iVar5 + 0x56) = 0x0;
                SendMessage16(ctx.s_tile2_bmp_1050_1538, 0x0, 0x0,
                              CONCAT22(0x401, (iVar5 + 0x5c)));
                if (((iVar5 + 0x5c) != 0x0) && ((iVar5 + 0x5c) != paVar12.field_0x0)) {
                    uVar3 = (iVar5 + 0x5c);
                    SendDlgItemMessage16(ctx.s_tile2_bmp_1050_1538, uStack30, 0x0, 0x1,
                                         CONCAT13((uVar3 >> 0x8), CONCAT12(uVar3, 0x404)));
                }
                HVar11 = ctx.s_tile2_bmp_1050_1538;
                (iVar5 + 0x5c) = 0x0;
            } else {
                if (bVar16 < 0x9) {
                    if (bVar16 == 0x1) {
                        pIVar13 = (INT16 *)
                        GetWindowLong16(ctx.s_tile2_bmp_1050_1538, 0x0);
                        iVar5 = pIVar13;
                        uVar10 = ((pIVar13 & 0xffff0000) >> 0x10);
                        (iVar5 + 0x2) = (param_1 + 0x8);
                        IVar6 = GetDlgCtrlID16(ctx.s_tile2_bmp_1050_1538);
                        *pIVar13 = IVar6;
                        (iVar5 + 0x56) = (param_1 + 0x12);
                        string_1000_3d3e((pIVar13 & 0xffff0000 | (iVar5 + 0x6)),
                                         (param_1 + 0x16));
                        if (((param_1 + 0x12) & 0x1) != 0x0) {
                            SendMessage16(0x1000, 0x0, 0x0, CONCAT22(0x401, *pIVar13));
                        }
                        if (((param_1 + 0x14) & 0x800) == 0x0) {
                            return;
                        }
                        pbVar1 = (iVar5 + 0x4);
                        *pbVar1 = *pbVar1 | 0x4;
                        return;
                    }
                    if (bVar16 == 0x2) {
                        fn_ptr_1000_17ce(ctx, paVar12, 0x1000);
                        SetWindowLong16(0x1000, 0x0, 0x0);
                        return;
                    }
                    if (bVar16 != 0x7) {
                        // goto
                        // LAB_1040_a1ae;
                    }
                    pbVar1 = (iVar5 + 0x4);
                    *pbVar1 = *pbVar1 | 0x8;
                    LVar14 = SendMessage16(ctx.s_tile2_bmp_1050_1538, 0x0, 0x0, 0x4000000);
                    uVar4 = LVar14;
                    if (((LVar14 >> 0x10) == 0x534b) && ((iVar5 + 0x5c) = uVar4, uVar4 != paVar12.field_0x0)) {
                        SendDlgItemMessage16(ctx.s_tile2_bmp_1050_1538, 0x1, 0x0, 0x0, CONCAT22(uVar4, 0x404));
                    }
                    HVar11 = ctx.s_tile2_bmp_1050_1538;
                    SendMessage16(ctx.s_tile2_bmp_1050_1538, 0x0, 0x0,
                                  CONCAT22(0x401, paVar12.field_0x0));
                    (iVar5 + 0x56) = 0x1;
                } else {
                    if (bVar16 == 0xa) {
                        pbVar1 = (iVar5 + 0x4);
                        *pbVar1 = *pbVar1 & 0xfb;
                        if (param_3 == 0x0) {
                            pbVar1 = (iVar5 + 0x4);
                            *pbVar1 = *pbVar1 | 0x4;
                        }
                    } else {
                        if (bVar16 != 0xc) {
                            if (bVar16 == 0xf) {
                                draw_op_1040_9948(param_4._2_2_, paVar12, s_tile2_bmp_1050_1538,
                                                  param_6);
                                return;
                            }
//               TODO: goto LAB_1040_a1ae;
                        }
                        if (CONCAT22(param_2, param_1) != 0x0) {
                            HVar11 = 0x1000;
                            string_1000_3d3e((paVar12 & 0xffff0000 | (iVar5 + 0x6)),
                                             CONCAT22(param_2, param_1));
                        }
                    }
                }
            }
//       TODO: goto LAB_1040_9e20;
        }
        if (param_4 == 0x200) {
            if (((iVar5 + 0x4) & 0x1) == 0x0) {
                return;
            }
            GetClientRect16(ctx.s_tile2_bmp_1050_1538, local_a);
            iVar2 = (iVar5 + 0x4);
            BVar7 = PtInRect16(s_tile2_bmp_1050_1538,
                               CONCAT22(param_2, param_1));
            if (BVar7 == 0x0) {
                pbVar1 = (iVar5 + 0x4);
                *pbVar1 = *pbVar1 & 0xfd;
            } else {
                pbVar1 = (iVar5 + 0x4);
                *pbVar1 = *pbVar1 | 0x2;
            }
            param_1 = (iVar5 + 0x4) - iVar2;
        } else {
            if (param_4 < 0x201) {
                offset = param_4 - 0x81;
                if (offset == 0x0) {
                    uVar10 = 0x5e;
                    mem_op_1000_179c(0x5e, puVar8, 0x1000);
                    uVar9 = puVar8 | offset;
                    if (uVar9 == 0x0) {
                        offset = 0x0;
                        uVar9 = 0x0;
                    } else {
                        pass1_1040_9824(CONCAT22(puVar8, offset));
                    }
                    SetWindowLong16(0x1000, offset, CONCAT22(uVar10, uVar9));
                    return;
                }
                if (param_4 == 0x87) {
                    return;
                }
                if (param_4 == 0x100) {
                    if ((param_3 == 0x26) || (param_3 == 0x25)) {
                        HVar11 = 0x1;
                    } else {
                        if ((param_3 != 0x28) && (param_3 != 0x27)) {
                            if (((param_3 == 0x20) || (param_3 == 0xd)) && (&ctx.PTR_LOOP_1050_5ed8 == 0x0)) {
                                &ctx.PTR_LOOP_1050_5ed8 = 0x1;
                                pbVar1 = (iVar5 + 0x4);
                                *pbVar1 = *pbVar1 | 0x2;
//                 TODO: goto LAB_1040_9e20;
                            }
//               TODO: goto LAB_1040_a1ae;
                        }
                        HVar11 = 0x0;
                    }
                    GetNextDlgTabItem16(ctx.s_tile2_bmp_1050_1538, HVar11, param_4._2_2_);
                    SetFocus16(ctx.s_tile2_bmp_1050_1538);
                    return;
                }
                if ((param_4 == 0x101) && (&ctx.PTR_LOOP_1050_5ed8 != 0x0)) {
                    &ctx.PTR_LOOP_1050_5ed8 = 0x0;
                    pbVar1 = (iVar5 + 0x4);
                    *pbVar1 = *pbVar1 & 0xfd;
                    InvalidateRect16(ctx.s_tile2_bmp_1050_1538,
                                     (&ctx.PTR_LOOP_1050_0000 + 0x1), 0x0);
                    UpdateWindow16(ctx.s_tile2_bmp_1050_1538);
                    SendMessage16(ctx.s_tile2_bmp_1050_1538, 0x0, 0x0,
                                  CONCAT22(0x111, paVar12.field_0x0));
                    return;
                }
//LAB_1040_a1ae:
                DefWindowProc16(ctx.s_tile2_bmp_1050_1538, param_1, param_2,
                                CONCAT13((param_4 >> 0x8), CONCAT12(bVar16, param_3)));
                return;
            }
            if (param_4 == 0x201) {
//LAB_1040_9e74:
                SetFocus16(ctx.s_tile2_bmp_1050_1538);
                pbVar1 = (iVar5 + 0x4);
                *pbVar1 = *pbVar1 | 0x3;
                InvalidateRect16(ctx.s_tile2_bmp_1050_1538,
                                 (&ctx.PTR_LOOP_1050_0000 + 0x1), 0x0);
                UpdateWindow16(ctx.s_tile2_bmp_1050_1538);
                SetCapture16(ctx.s_tile2_bmp_1050_1538);
                return;
            }
            if (param_4 == 0x202) {
                ReleaseCapture16();
                GetClientRect16(ctx.s_tile2_bmp_1050_1538, local_a);
                if (((iVar5 + 0x4) & 0x1) == 0x0) {
                    return;
                }
                pbVar1 = (iVar5 + 0x4);
                *pbVar1 = *pbVar1 & 0xfc;
                InvalidateRect16(ctx.s_tile2_bmp_1050_1538,
                                 (&ctx.PTR_LOOP_1050_0000 + 0x1), 0x0);
                UpdateWindow16(ctx.s_tile2_bmp_1050_1538);
                BVar7 = PtInRect16(s_tile2_bmp_1050_1538,
                                   CONCAT22(param_2, param_1));
                if (BVar7 == 0x0) {
                    return;
                }
                PostMessage16(ctx.s_tile2_bmp_1050_1538, 0x0, 0x0,
                              CONCAT22(0x111, paVar12.field_0x0));
                return;
            }
            if (param_4 == 0x203) {
                // goto
                // LAB_1040_9e74;
            }
            if (param_4 != 0x404) {
                // goto
                // LAB_1040_a1ae;
            }
            if (param_3 == 0x1) {
                (iVar5 + 0x56) = 0x1;
            } else {
                (iVar5 + 0x56) = 0x0;
            }
        }
    }
    HVar11 = ctx.s_tile2_bmp_1050_1538;
    if (param_1 == 0x0) {
        return;
    }
//LAB_1040_9e20:
    InvalidateRect16(HVar11, (&ctx.PTR_LOOP_1050_0000 + 0x1), 0x0);
    UpdateWindow16(ctx.s_tile2_bmp_1050_1538);
    return;
}


pub fn win_msg_1040_a308(param_1: u32, param_2: i16, param_3: HWND16, param_4: u16) -> u32

{
    let piVar1: U32Ptr;
    let uVar2: u32;
    let iVar3: i16;
    let uVar4: u16;
    let hwnd: HWND16;
    let LVar5: LRESULT;
    let puVar6: U32Ptr;
    let mut pcVar7: String;
    let uVar8: u16;
    let uVar9: u16;
    let iStack12: i16;

    SendMessage16(param_3, 0x0, 0x0, 0x4050000);
    LVar5 = SendMessage16(ctx.s_tile2_bmp_1050_1538, 0x0, 0x0, 0xb0000);
    // uVar4 = (param_1 >> 0x10);
    iVar3 = param_1;
    uVar2 = (iVar3 + 0x90);
    if ((uVar2 + 0x10) == 0x0) {
        uVar4 = 0x0;
        uVar8 = 0x401;
        pcVar7 = load_string_1010_847e(ctx.PTR__LOOP_1050_14cc,
                                       (ctx.PTR__LOOP_1050_14cc >> 0x10), 0x1010);
        hwnd = ctx.s_tile2_bmp_1050_1538;
        SendMessage16(0x1010, pcVar7, (pcVar7 >> 0x10),
                      CONCAT22(uVar8, uVar4));
    } else {
        hwnd = 0x1010;
        puVar6 = mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0, 0x3, param_4,
                                 (LVar5 >> 0x10), param_2);
        // for (iStack12 = 0x0; uVar2 = (iVar3 + 0x90),
        //     piVar1 = (uVar2 + 0x10), *piVar1 != iStack12 && iStack12 <= *piVar1;
        //     iStack12 += 0x1) {
        //   uVar8 = 0x0;
        //   uVar9 = 0x401;
        //   uVar2 = (iVar3 + 0x90);
        //   uVar2 = (uVar2 + 0xc);
        //   pcVar7 = load_string_1010_ac92
        //                      (0x1010,puVar6,(puVar6 >> 0x10),
        //                       (uVar2 + iStack12 * 0x2));
        //   hwnd = ctx.s_tile2_bmp_1050_1538;
        //   SendMessage16(0x1010,pcVar7,(WPARAM16)(pcVar7 >> 0x10),
        //                 CONCAT22(uVar9,uVar8));
        // }
    }
    LVar5 = SendMessage16(hwnd, 0x0, 0x0, 0xb0001);
    return CONCAT22((LVar5 >> 0x10), 0x1);
}


pub fn get_dlg_item_1040_a3d0(param_1: u32, param_2: HWND16) {
    let lVar1: i32;
    let iVar3: &mut Struct49;
    let unaff_DI: i16;
    let uVar2: u16;
    let unaff_SS: u16;

    // uVar2 = (param_1 >> 0x10);
    iVar3 = param_1;
    if (iVar3.field_0x90 != 0x0) {
        lVar1 = iVar3.field_0x90;
        (lVar1 + 0x14) = iVar3.field_0x6;
        GetDlgItem16(param_2, 0x1826);
        win_msg_1040_a308(param_1, unaff_DI, s_tile2_bmp_1050_1538, unaff_SS);
    }
    return;
}


pub fn win_ui_op_1040_a784(param_1: i16, param_2: i16, param_3: u16, param_4: u32, param_5: u16,
                           param_6: u16, param_7: u16)

{
    let i_var1: i16;

    if (param_4._2_2_ != 0xeb) {
        if (param_4._2_2_ == 0x1f4) {
            msg_box_op_1040_a85a(CONCAT22(param_2, param_1), 0x0, param_5, param_7);
            return;
        }
        if (param_4._2_2_ == 0x1f7) {
            ctx._PTR_LOOP_1050_5ef0 = (param_1 + 0x94);
            pass1_1038_af40(ctx.PTR__LOOP_1050_5b7c, (param_1 + 0x8), 0x23, param_5, param_1,
                            &ctx.PTR_LOOP_1050_1038, param_7);
            PostMessage16(&ctx.PTR_LOOP_1050_1038, 0x0, 0x0, 0x1110002);
            return;
        }
        if (param_4._2_2_ != 0x17d8) {
            pass1_1040_b54a(param_1, param_2, param_3, param_4, param_5, param_6, param_7);
            return;
        }
        i_var1 = (param_1 + 0x6);
        SetWindowPos16(param_6, 0x6, 0xed, 0x237, 0x0, 0x0, 0x0);
        GetDlgItem16(ctx.s_tile2_bmp_1050_1538, 0x17d8);
        EnableWindow16(ctx.s_tile2_bmp_1050_1538, 0x0);
        (param_1 + 0x98) = 0x1;
        param_1 = param_2;
        param_2 = i_var1;
    }
    win_ui_dlg_op_1040_a94a(ctx, CONCAT22(param_2, param_1), param_7);
    return;
}


pub fn msg_box_op_1040_a85a(param_1: u32, param_2: &mut String, param_3: U32Ptr, param_4: u16) {
    local_206: u8[0x102];
    local_104: u8[0x102];

    mem_op_1000_179c(0x1000, param_3, 0x1000);
    load_string_1010_84e0(0x1010, _PTR_LOOP_1050_14cc, 0x100, local_206, param_4);
    load_string_1010_84e0(0x1010, _PTR_LOOP_1050_14cc,
                          (ctx.PTR__LOOP_1050_14cc >> 0x10), 0x3ff, param_2, (short)param_3);
    load_string_1010_84e0(0x1010, _PTR_LOOP_1050_14cc, 0x3ff, local_104, param_4);
    string_1000_3cea(CONCAT22(param_3, param_2), CONCAT22(param_4, local_104));
    load_string_1010_84e0(0x1010, _PTR_LOOP_1050_14cc, 0x3ff, local_104, param_4);
    string_1000_3cea(CONCAT22(param_3, param_2), CONCAT22(param_4, local_104));
    load_string_1010_84e0(0x1010, _PTR_LOOP_1050_14cc, 0x3ff, local_104, param_4);
    string_1000_3cea(CONCAT22(param_3, param_2), CONCAT22(param_4, local_104));
    MessageBox16(0x1000, 0x0, local_206, param_4);
    fn_ptr_1000_17ce(ctx, CONCAT22(param_3, param_2), 0x1000);
    return;
}


pub unsafe fn win_ui_dlg_op_1040_a94a(
    ctx: &mut AppContext,
    param_1: u32,
    param_2: u16,
    in_af: u8,
    unaff_di: i16,
    in_dx: u16,
) {
    let pi_var1: U32Ptr;
    let u_var2: u32;
    let u_var3: u32;
    let u_var4: u16;
    let value: U32Ptr;
    let mut pc_var5: String;
    let u_var6: u16;
    let pu_var8: U32Ptr;
    let pu_var9: U32Ptr;
    let lp_string: u16;
    let i_var10: i16;
    let i_var11: i16;
    let u_var12: u16;
    let u_var13: u16;
    let hvar14: HWND16;
    let b_var15: bool;
    let pu_var16: U32Ptr;
    let l_var17: i32;
    let u_stack288: u16;
    let u_stack286: u16;
    let bstack278: bool;
    let i_stack276: i16;
    let local_102: [u8; 100];
    let u_var7: u32;

    pu_var16 = mixed_1010_20ba(ctx, ctx.PTR__LOOP_1050_0ed0, 0x3, param_2, in_dx, unaff_di);
    // pu_var8 = (pu_var16 >> 0x10);
    u_var4 = pu_var16;
    // u_var12 = (param_1 >> 0x10);
    i_var10 = param_1;
    pu_var9 = pu_var8;
    pass1_1010_c3c2(u_var4, pu_var8, CONCAT22(param_2, local_102),
                    (i_var10 + 0x94), pu_var8, in_af, param_2);
    SetDlgItemText16(0x1010, local_102, param_2);
    pass1_1010_c320(u_var4, pu_var8, CONCAT22(param_2, local_102),
                    (i_var10 + 0x94), pu_var9);
    SetDlgItemText16(0x1010, local_102, param_2);
    string_op_1010_c446(param_2, in_af, pu_var9, pu_var16, CONCAT22(param_2, local_102),
                        (i_var10 + 0x94));
    value = local_102;
    SetDlgItemText16(0x1010, value, param_2);
    pass1_1030_6ddc((i_var10 + 0x94));
    SetDlgItemInt16(0x1030, 0x0, value, 0x1f5);
    pass1_1030_6e14((i_var10 + 0x94));
    SetDlgItemInt16(0x1030, 0x0, value, 0x1f6);
    if (i_var10 + 0x98) != 0x0 {
        hvar14 = 0x1010;
        struct_1010_dd5e(u_var4, pu_var8, (i_var10 + 0x94));
        if (pu_var9 | value) != 0x0 {
            u_var3 = (i_var10 + 0x94);
            // u_var13 = (u_var3 >> 0x10);
            i_var11 = u_var3;
            u_var2 = (i_var11 + 0x26);
            lp_string = (i_var11 + 0x28);
            i_stack276 = 0x1798;
            bstack278 = 0x17c3;
            (i_var10 + 0xea) = 0x0;
            u_var7 = u_var2;
            // for (u_stack288 = 0x1; u_stack288 < 0x25; u_stack288 += 0x1) {
            //   if (u_var2 == 0x0) {
            //     l_var17 = 0x0;
            //   }
            //   else {
            //     hvar14 = 0x1020;
            //     l_var17 = pass1_1020_bae6(u_var2,CONCAT22(u_stack288,(u_var2 >> 0x10)),
            //                              u_var7,lp_string,param_2);
            //   }
            //   u_var6 = (l_var17 >> 0x10);
            //   b_var15 = (value + u_stack288 * 0x4) != 0x0;
            //   lp_string = u_var6;
            //   if (b_var15) {
            //     pc_var5 = string_1020_c0d8(u_stack288);
            //     SetDlgItemText16(0x1020,pc_var5,lp_string);
            //     hvar14 = ctx.s_tile2_bmp_1050_1538;
            //     SetDlgItemInt16(ctx.s_tile2_bmp_1050_1538,0x0,
            //                     (value + u_stack288 * 0x4),bstack278);
            //   }
            //   u_stack286 = l_var17;
            //   u_var6 |= u_stack286;
            //   if (l_var17 != 0x0) {
            //     if (!b_var15) {
            //       pc_var5 = string_1020_c0d8(u_stack288);
            //       hvar14 = ctx.s_tile2_bmp_1050_1538;
            //       SetDlgItemText16(0x1020,pc_var5,lp_string);
            //     }
            //     SetDlgItemInt16(hvar14,0x0,u_stack286,bstack278);
            //     i_var11 = (i_var10 + 0xea);
            //     hvar14 = GetDlgItem16(ctx.s_tile2_bmp_1050_1538,i_stack276);
            //     *(HWND16 *)(i_var10 + i_var11 * 0x2 + 0x9a) = hvar14;
            //     pi_var1 = (i_var10 + 0xea);
            //     *pi_var1 = *pi_var1 + 0x1;
            //     i_var11 = (i_var10 + 0xea);
            //     hvar14 = ctx.s_tile2_bmp_1050_1538;
            //     u_var6 = GetDlgItem16(ctx.s_tile2_bmp_1050_1538,bstack278);
            //     *(HWND16 *)(i_var10 + i_var11 * 0x2 + 0x9a) = u_var6;
            //     pi_var1 = (i_var10 + 0xea);
            //     *pi_var1 = *pi_var1 + 0x1;
            //     b_var15 = true;
            //   }
            //   u_var7 = u_var6;
            //   if (b_var15) {
            //     i_stack276 += 0x1;
            //     bstack278 += 0x1;
            //   }
            // }
        }
    }
    return;
}


pub fn msg_box_ui_op_1040_ad66(param_1: u32, param_2: &mut String, param_3: U32Ptr, param_4: u16) {
    local_206: u8[0x102];
    local_104: u8[0x102];

    mem_op_1000_179c(0x1000, param_3, 0x1000);
    load_string_1010_84e0(0x1010, _PTR_LOOP_1050_14cc, 0x100, local_206, param_4);
    load_string_1010_84e0(0x1010, _PTR_LOOP_1050_14cc,
                          (ctx.PTR__LOOP_1050_14cc >> 0x10), 0x3ff, param_2, (short)param_3);
    load_string_1010_84e0(0x1010, _PTR_LOOP_1050_14cc, 0x3ff, local_104, param_4);
    string_1000_3cea(CONCAT22(param_3, param_2), CONCAT22(param_4, local_104));
    MessageBox16(0x1000, 0x0, local_206, param_4);
    fn_ptr_1000_17ce(ctx, CONCAT22(param_3, param_2), 0x1000);
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn win_ui_op_1040_ae04(
    ctx: &mut AppContext,
    param_1: u32,
    param_2: U32Ptr,
    in_dx: u16,
    unaff_di: u16,
    in_af: u8) {
    let b_var1: bool;
    let i_var2: i16;
    let mut id: String;
    // let in_DX: U32Ptr;
    let u_var3: u16;
    let lp_string: SEGPTR;

    let u_var5: u16;
    // let unaff_DI: i16;
    let u_var7: u16;
    // let in_AF: u8;
    let pu_var8: U32Ptr;
    let u_var9: u32;
    let u_var10: u32;
    let mut pc_var11: String;
    let i_stack280: i16;
    local_102: u8[0x100];

    pu_var8 = mixed_1010_20ba(ctx, ctx.PTR__LOOP_1050_0ed0, 0x3, param_2, in_dx, unaff_di);
    // u_var3 = (pu_var8 >> 0x10);
    // u_var6 = (param_1 >> 0x10);
    //  i_var4 = param_1;
    pass1_1010_c3c2(pu_var8, u_var3, CONCAT22(param_2, local_102),
                    (param_1 + 0x94), u_var3, in_af, param_2);
    SetDlgItemText16(0x1010, local_102, param_2);
    u_var9 = struct_op_1030_73a8((param_1 + 0x94));
    i_var2 = u_var9 + 0x20;
    u_var10 = pass1_1030_8326(ctx);
    // lp_string = (u_var10 >> 0x10);
    b_var1 = false;
    // for (i_stack280 = 0x0; i_stack280 < 0xa; i_stack280 += 0x1) {
    //   u_var7 = ((u_var9 & 0xffff0000) >> 0x10);
    //   if (((i_stack280 * 0xc + i_var2 + 0x2) | (i_stack280 * 0xc + i_var2)) !=
    //       0x0) {
    //     u_var5 = i_stack280 * 0xc + i_var2;
    //     id = string_op_1020_c222((u_var5 + 0x4));
    //     SetDlgItemText16(0x1020,id,lp_string);
    //     wsprintf16(s_tile2_bmp_1050_1538,local_102,param_2);
    //     SetDlgItemText16(ctx.s_tile2_bmp_1050_1538,local_102,param_2);
    //     u_var10 = unk_load_str_op_1010_8c96
    //                        ((param_1 + 0x98),CONCAT22(param_2,local_102),
    //                         u_var9 & 0xffff0000 | u_var5,0x1010,param_2);
    //     lp_string = u_var10;
    //     SetDlgItemText16(0x1010,local_102,param_2);
    //     b_var1 = true;
    //   }
    // }
    if !b_var1 {
        pc_var11 = load_string_1010_847e(ctx.PTR__LOOP_1050_14cc,
                                         (ctx.PTR__LOOP_1050_14cc >> 0x10), 0x1010);
        SetDlgItemText16(0x1010, pc_var11, (pc_var11 >> 0x10));
    }
    return;
}


pub fn unk_win_ui_op_1040_b230(param_1: &mut Struct1, param_2: u16, param_3: u16) {
    let ppcVar1: u32;
    let IVar2: i16;
    let in_DX: U32Ptr;
    let unaff_DI: i16;
    let uVar3: u16;
    let puVar4: U32Ptr;
    let puVar5: U32Ptr;
    let uVar7: u16;
    let uVar6: u32;
    let local_1a: RECT16;
    let iStack22: i16;
    let iStack20: i16;
    let iStack18: i16;
    let iStack16: i16;
    let iStack14: i16;
    let iStack12: i16;
    let puStack10: U32Ptr;
    let local_6: i16;
    let local_4: i16;

    dialog_ui_fn_1040_78e2(param_1, param_2);
    if (ctx.PTR_LOOP_1050_5ef8 == (&DAT_1050_0004 + 0x1)) {
        ctx.PTR_LOOP_1050_5ef8 = 0x0;
    }
    puVar5 = CONCAT22(param_3, &local_4);
    puVar4 = CONCAT22(param_3, &local_6);
    puStack10 = mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0, 0x48, param_3, in_DX, unaff_DI);
    pass1_1008_3e94(
        (puStack10 & 0xffff0000 | (puStack10 + 0xe)), puVar4,
        puVar5);
    // uVar3 = (puStack10 >> 0x10);
    iStack12 = (puStack10 + 0xa);
    iStack14 = (puStack10 + 0xc);
    uVar7 = 0x4;
    IVar2 = GetSystemMetrics16(0x1008);
    iStack16 = IVar2 * ctx.PTR_LOOP_1050_5ef8 + 0xa;
    ctx.PTR_LOOP_1050_5ef8 = ctx.PTR_LOOP_1050_5ef8 + 0x1;
    iStack18 = iStack16 + local_6;
    iStack16 += local_4;
    // uVar3 = (param_1 >> 0x10);
    uVar6 = CONCAT22(uVar7, (param_1 + 0x6));
    GetWindowRect16(ctx.s_tile2_bmp_1050_1538, &local_1a);
    if (iStack14 < (iStack20 - local_1a.y) + iStack18) {
        iStack18 = -0x2 - ((iStack20 - local_1a.y) - iStack14);
    }
    if (iStack12 < (iStack22 - local_1a.x) + iStack16) {
        iStack16 = -0x2 - ((iStack22 - local_1a.x) - iStack12);
    }
    uVar3 = (param_1 + 0x6);
    SetWindowPos16(ctx.s_tile2_bmp_1050_1538, 0x1, 0x0, 0x0, iStack18, iStack16, 0x0);
    ppcVar1 = (param_1.field_0x0 + 0x6c);
    (**ppcVar1)(ctx.s_tile2_bmp_1050_1538, param_1, uVar3, uVar6);
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn win_ui_op_1040_b372(param_1: i32, param_2: u16, param_3: u16, in_colorref_4: COLORREF) {
    let uVar1: u16;
    let iVar2: i16;
    let local_brush_handle: HBRUSH16;
    let IVar3: i16;
    let uVar4: i32;
    let extraout_DX: u16;
    let uVar5: u16;
    let local_win_handle: HWND16;
    let uVar6: u32;
    COLORREF
    local_colorref;

    // uVar5 = (param_1 >> 0x10);
    local_colorref = in_colorref_4;
    if ((param_1 + 0x8e) == 0x0) {
        local_colorref = s_tile2_bmp_1050_1538;
        local_brush_handle = CreateSolidBrush16(in_colorref_4);
        (param_1 + 0x8e) = local_brush_handle;
    }
    if (ctx.PTR__LOOP_1050_5efa == 0x0) {
        local_colorref = 0x1008;
        uVar6 = pass1_1008_4d72((ctx.PTR__LOOP_1050_4230 + 0xe));
        // uVar1 = (uVar6 >> 0x10);
        iVar2 = uVar6;
        ctx._PTR_LOOP_1050_5efa = CONCAT12(*(iVar2 + 0x94),
                                           CONCAT11(*(iVar2 + 0x95),
                                                    *(iVar2 + 0x96)));
    }
    if (param_3 < 0x4) {
//LAB_1040_b3ea:
        local_win_handle = ctx.s_tile2_bmp_1050_1538;
        IVar3 = GetDlgCtrlID16(local_colorref);
        if (IVar3 == 0x14c) {
            local_colorref = 0x0;
//       TODO: goto LAB_1040_b41a;
        }
        if (IVar3 == 0x175) {
            local_colorref = 0x0;
//       TODO: goto LAB_1040_b41a;
        }
    } else {
        local_win_handle = local_colorref;
        if (param_3 != 0x4) {
            if ((param_3 == 0x4) || (0x1 < param_3 - 0x5)) {
                return;
            }
//       TODO: goto LAB_1040_b3ea;
        }
    }
    local_colorref = _PTR_LOOP_1050_5efa;
//LAB_1040_b41a:
    SetTextColor16(local_win_handle, local_colorref);
    SetBkColor16(ctx.s_tile2_bmp_1050_1538, 0x0);
    return;
}


pub fn show_win_1040_b43c(param_1: U32Ptr, param_2: HWND16) {
    let ppcVar1: u32;

    ppcVar1 = (*param_1 + 0x70);
    (**ppcVar1)(param_2, param_1);
    ShowWindow16(param_2, 0x5);
    return;
}


pub fn win_ui_1040_b8d2(param_1: &mut Struct1, param_2: u16, param_3: u16, param_4: u16) {
    let pu_var1: u32;
    let uVar2: u16;
    let uVar3: u32;
    let paVar4: &mut Struct160;
    let uVar5: u16;
    let uVar6: u16;
    let iVar7: i16;
    let puVar8: U32Ptr;
    let puVar9: U32Ptr;
    let uVar10: u16;
    let uVar11: u16;
    let iVar10: &mut Struct167;
    let unaff_DI: i16;
    let uVar12: u16;
    let puVar13: U32Ptr;

    dialog_ui_fn_1040_78e2(param_1, param_3);
    puVar13 = mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0, 0x31, param_4, param_2, unaff_DI);
    // puVar9 = (puVar13 >> 0x10);
    paVar4 = puVar13;
    // uVar12 = (param_1 >> 0x10);
    iVar10 = param_1;
    &iVar10.field_0x98 = paVar4;
    (&iVar10.field_0x98 + 0x2) = puVar9;
    mem_op_1000_179c(0xa, puVar9, 0x1000);
    puVar8 = (puVar9 | paVar4);
    if (puVar8 == 0x0) {
        iVar10.field_0x94 = 0x0;
    } else {
        puVar13 = struct_1040_bf3e(CONCAT22(puVar9, paVar4), iVar10.field_0x6);
        // puVar8 = (puVar13 >> 0x10);
        paVar4 = puVar13;
        &iVar10.field_0x94 = paVar4;
        (&iVar10.field_0x94 + 0x2) = puVar8;
    }
    pass1_1040_bfde(iVar10.field_0x94, iVar10.field_0x98, param_4);
    mem_op_1000_179c(0x42, puVar8, 0x1000);
    puVar9 = (puVar8 | paVar4);
    if (puVar9 != 0x0) {
        pass1_1008_3bd6(paVar4, puVar8, 0x1, 0xa000a, 0x0, 0x800081,
                        CONCAT22(iVar10.field_0x6, 0x10a), puVar9, param_4);
    }
    mem_op_1000_179c(0x42, puVar9, 0x1000);
    puVar8 = (puVar9 | paVar4);
    if (puVar8 != 0x0) {
        pass1_1008_3bd6(paVar4, puVar9, 0x1, 0xa0028, 0x0, 0x840085,
                        CONCAT22(iVar10.field_0x6, 0x10b), puVar8, param_4);
    }
    mem_op_1000_179c(0x42, puVar8, 0x1000);
    puVar9 = (puVar8 | paVar4);
    if (puVar9 != 0x0) {
        pass1_1008_3bd6(paVar4, puVar8, 0x1, 0xa0046, 0x0, 0x860087,
                        CONCAT22(iVar10.field_0x6, 0x10d), puVar9, param_4);
    }
    mem_op_1000_179c(0x42, puVar9, 0x1000);
    puVar8 = (puVar9 | paVar4);
    if (puVar8 != 0x0) {
        pass1_1008_3bd6(paVar4, puVar9, 0x1, 0xa0064, 0x0, 0x880089,
                        CONCAT22(iVar10.field_0x6, 0x10e), puVar8, param_4);
    }
    mem_op_1000_179c(0x42, puVar8, 0x1000);
    puVar9 = (puVar8 | paVar4);
    if (puVar9 != 0x0) {
        pass1_1008_3bd6(paVar4, puVar8, 0x1, 0xa0082, 0x0, 0x820083,
                        CONCAT22(iVar10.field_0x6, 0x10c), puVar9, param_4);
    }
    mem_op_1000_179c(0x42, puVar9, 0x1000);
    puVar8 = (puVar9 | paVar4);
    if (puVar8 != 0x0) {
        pass1_1008_3bd6(paVar4, puVar9, 0x1, 0xa00d2, 0x0, 0x8a008b,
                        CONCAT22(iVar10.field_0x6, 0xbbb), puVar8, param_4);
    }
    mem_op_1000_179c(0x42, puVar8, 0x1000);
    puVar9 = (puVar8 | paVar4);
    if (puVar9 == 0x0) {
        paVar4 = 0x0;
        puVar9 = 0x0;
    } else {
        pass1_1008_3bd6(paVar4, puVar8, 0x1, 0xa00a0, 0x8e, 0x8c008d,
                        CONCAT22(iVar10.field_0x6, 0xbbc), puVar9, param_4);
    }
    puVar13 = mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0, 0x3, param_4, puVar9, unaff_DI);
    // uVar10 = (puVar13 >> 0x10);
    uVar2 = puVar13;
    uVar11 = uVar10;
    uVar5 = pass1_1010_a5ac(uVar2, uVar10, iVar10.field_0xb0);
    uVar6 = pass1_1010_ac62(uVar2, uVar10, 0x1e, uVar5, uVar11);
    if (uVar6 != 0x0) {
        pass1_1010_a5ca(uVar2, uVar10, uVar5, uVar6, uVar11);
        if (0x0 < uVar6) {
            pass1_1010_a58a(uVar2, uVar10, uVar5, uVar6, uVar11);
            if (uVar6 == 0x0) {
                // goto
                // LAB_1040_bb26;
            }
        }
    }
    enable_win_1040_9234(CONCAT22(puVar9, paVar4), 0x0, 0x1010);
//LAB_1040_bb26:
    pu_var1 = iVar10.field_0x98;
    iVar7 = pu_var1;
    uVar3 = pu_var1 & 0xffff0000;
    // uVar12 = (uVar3 >> 0x10);
    SetWindowPos16(0x1010, 0x40, (iVar7 + 0x10), (iVar7 + 0xe),
                   (iVar7 + 0xc), (uVar3 | iVar7 + 0xa), 0x0);
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn win_ui_op_1040_bbe2(param_1: i16, param_2: u16, param_3: u16, param_4: u32, param_5: HWND16, param_6: u16)

{
    let ppcVar1: u32;
    let BVar2: bool;
    let iVar3: i16;
    let uVar4: u16;
    let in_DX: U32Ptr;
    let uVar5: u16;
    let uVar6: u16;
    let uVar7: u16;
    let unaff_DI: i16;
    let puVar8: U32Ptr;
    let puVar9: U32Ptr;
    let uVar10: u32;
    let uVar11: u16;
    let uVar12: u16;
    let uVar13: u16;
    let uStack30: u16;
    let local_a: RECT16;
    let iStack6: i16;
    let iStack4: i16;

    if (param_4._2_2_ != 0x10c) {
        if (param_4._2_2_ < 0x10d) {
            if (param_4._2_2_ == 0xfa) {
                uVar10 = (param_1 + 0x98);
                ppcVar1 = ((param_1 + 0x98) + 0x18);
                (**ppcVar1)(param_5, uVar10, (uVar10 >> 0x10), 0x0,
                            ctx._PTR_LOOP_1050_5efe, (ctx.PTR__LOOP_1050_5efe >> 0x10));
                return;
            }
            if (param_4._2_2_ == 0x10a) {
                GetClientRect16(param_5, &local_a);
                uVar10 = (param_1 + 0x98);
                local_a.y += 0x3;
                local_a.x = (uVar10 + 0x1a) + -0x9;
                iStack6 += -0x3;
                iStack4 += -0x3;
                InvalidateRect16(ctx.s_tile2_bmp_1050_1538,
                                 (&ctx.PTR_LOOP_1050_0000 + 0x1), &local_a);
                unk_destroy_win_op_1010_2fa0((param_1 + 0x98), 0x1010);
                pass1_1010_32c0((param_1 + 0x98), 0x0);
                pass1_1010_2ee2((param_1 + 0x98), param_6, 0x1010);
                return;
            }
            if (param_4._2_2_ != 0x10b) {
                // goto
                // LAB_1040_be78;
            }
            uVar10 = (param_1 + 0x98);
            uVar11 = (uVar10 + 0x12);
            uVar6 = uVar11;
            puVar8 = mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0, 0x3, param_6, in_DX, unaff_DI);
            // uVar5 = (puVar8 >> 0x10);
            puVar9 = puVar8;
            pass1_1010_a5ca(puVar8, uVar5, uVar6, puVar8, uVar5);
            // uVar6 = (puVar9 >> 0x10);
            if ((uVar11 != 0x70) && (puVar9 == 0x0)) {
                return;
            }
            uVar10 = (param_1 + 0xb0);
            uVar12 = uVar10;
            // uVar13 = (uVar10 >> 0x10);
            uVar10 = (param_1 + 0x98);
            uVar11 = (uVar10 + 0x12);
        } else {
            if (param_4._2_2_ != 0x10d) {
                if (param_4._2_2_ == 0x10e) {
                    puVar8 = mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0, 0x32, param_6, in_DX, unaff_DI);
                    iVar3 = puVar8;
                    ui_op_1010_79aa(puVar8, 0xfc6, (param_1 + 0xb0), param_6);
                    if (iVar3 != 0x0) {
                        return;
                    }
                    unk_win_op_1010_7300(puVar8, 0x0, 0x13, (param_1 + 0xb0));
                    return;
                }
                if (param_4._2_2_ == 0xbbb) {
                    if ((param_1 + 0xb6) != 0x0) {
                        BVar2 = IsWindow16(param_5);
                        param_5 = ctx.s_tile2_bmp_1050_1538;
                        if (BVar2 != 0x0) {
                            // goto
                            // LAB_1040_bd39;
                        }
                    }
                    uVar10 = pass1_1038_af40(ctx.PTR__LOOP_1050_5b7c, (param_1 + 0x6), 0x1b,
                                             in_DX, param_1, &ctx.PTR_LOOP_1050_1038,
                                             param_6);
                    (param_1 + 0xb6) = (uVar10 + 0x6);
                    ShowWindow16(&ctx.PTR_LOOP_1050_1038, 0x1);
                    return;
                }
                if (param_4._2_2_ == 0xbbc) {
                    puVar8 = mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0, 0x3, param_6, in_DX, unaff_DI);
                    // uVar7 = (puVar8 >> 0x10);
                    uVar6 = puVar8;
                    uVar5 = uVar7;
                    uVar4 = pass1_1010_a5ac(uVar6, uVar7, (param_1 + 0xb0));
                    uVar11 = uVar4;
                    pass1_1010_a58a(uVar6, uVar7, uVar4, uVar4, uVar5);
                    if (uVar11 == 0x0) {
                        pass1_1010_a568(uVar6, uVar7, uVar4, 0x0, uVar5);
                    }
                    GetDlgItem16(0x1010, 0xbbc);
                    EnableWindow16(ctx.s_tile2_bmp_1050_1538, 0x0);
                    return;
                }
//LAB_1040_be78:
                pass1_1040_b54a(param_1, param_2, param_3, param_4, in_DX, param_5, param_6);
                return;
            }
            puVar8 = mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0, 0x3, param_6, in_DX, unaff_DI);
            // uVar6 = (puVar8 >> 0x10);
            uVar10 = (param_1 + 0xb0);
            uVar12 = uVar10;
            // uVar13 = (uVar10 >> 0x10);
            uVar11 = 0x71;
            uVar5 = uVar6;
        }
        uStack30 = puVar8;
        param_5 = 0x1010;
        pass1_1010_a5ec(uStack30, uVar5, uVar11, CONCAT22(uVar13, uVar12), uVar6);
        if ((param_1 + 0xb4) != 0x0) {
            param_5 = ctx.s_tile2_bmp_1050_1538;
            BVar2 = IsWindow16(0x1010);
            if (BVar2 != 0x0) {
                param_5 = ctx.s_tile2_bmp_1050_1538;
                SendMessage16(ctx.s_tile2_bmp_1050_1538, 0x0, 0x0, 0x11100eb);
            }
        }
    }
//LAB_1040_bd39:
    DestroyWindow16(param_5);
    return;
}


pub fn send_msg_1040_c85a(param_1: u32, param_2: HWND16) {
    ctx._PTR_LOOP_1050_5efe = param_1;
    SendMessage16(param_2, 0x0, 0x0, 0x11100fa);
    return;
}


pub fn win_ui_op_1040_cace(param_1: u32, param_2: HWND16, param_3: u16) {
    let uVar1: u32;
    let bVar2: bool;
    let iVar3: i16;
    let IVar4: i16;
    let in_DX: u16;
    let uVar5: u16;
    let uVar6: u16;
    let bVar7: bool;
    let uVar8: u16;
    local_208: u8[0x100];
    local_108: u8[0x100];
    let UStack8: u16;
    let uStack6: u16;
    let local_4: bool;

    // uVar6 = (param_1 >> 0x10);
    uVar5 = param_1;
    uStack6 = GetDlgItemInt16(param_2, 0x0, &local_4, param_3);
    UStack8 = GetDlgItemInt16(ctx.s_tile2_bmp_1050_1538, 0x0, &local_4, param_3);
    if (uStack6 == 0x0) {
        return;
    }
    pass1_1018_50ea((uVar5 + 0x98), uStack6, (uVar5 + 0x94));
    load_string_1010_84e0(0x1010, _PTR_LOOP_1050_14cc, 0x3ff, local_208, param_3);
    uVar1 = (uVar5 + 0x94);
    uVar8 = (ctx.PTR__LOOP_1050_14cc >> 0x10);
    if ((uVar1 + 0x36) == 0x0) {
        load_string_1010_84e0(0x1010, _PTR_LOOP_1050_14cc, 0x3ff, local_108, param_3);
        iVar3 = MessageBox16(0x1010, (s_New_failed_in_Op__Op_1050_0020 + 0x14),
                             local_208, param_3);
    } else {
        load_string_1010_84e0(0x1010, _PTR_LOOP_1050_14cc, 0x3ff, local_108, param_3);
        iVar3 = MessageBox16(0x1010, (s_New_failed_in_Op__Op_1050_0020 + 0x14),
                             local_208, param_3);
    }
    bVar2 = iVar3 == 0x6;
    bVar7 = false;
    if ((!bVar2) && (uVar1 = (uVar5 + 0x94), (uVar1 + 0x34) < 0x1)) {
        load_string_1010_84e0(0x1010, _PTR_LOOP_1050_14cc, 0x3ff, local_108, param_3);
        IVar4 = MessageBox16(0x1010, (s_New_failed_in_Op__Op_1050_0020 + 0x14),
                             local_208, param_3);
        bVar7 = IVar4 == 0x6;
        bVar2 = false;
    }
    if (bVar2) {
        ctx._PTR_LOOP_1050_5f16 = (uVar5 + 0x94);
        iVar3 = 0x26;
    } else {
        if (!bVar7) {
            return;
        }
        ctx._PTR_LOOP_1050_5a68 = (uVar5 + 0x94);
        iVar3 = 0x27;
    }
    pass1_1038_af40(ctx.PTR__LOOP_1050_5b7c, (uVar5 + 0x8), iVar3, in_DX, uVar5,
                    &ctx.PTR_LOOP_1050_1038, param_3);
    return;
}


pub fn msg_box_op_1040_cce4(param_1: u32, param_2: &mut String, param_3: U32Ptr, param_4: u16) {
    let uStack522: u32;
    local_206: u8[0x102];
    local_104: u8[0x102];
    let uVar2: u16;
    let uVar3: u16;

    mem_op_1000_179c(0x1000, param_3, 0x1000);
    load_string_1010_84e0(0x1010, _PTR_LOOP_1050_14cc, 0x100, local_206, param_4);
    load_string_1010_84e0(0x1010, _PTR_LOOP_1050_14cc,
                          (ctx.PTR__LOOP_1050_14cc >> 0x10), 0x3ff, param_2, (short)param_3);
    load_string_1010_84e0(0x1010, _PTR_LOOP_1050_14cc, 0x3ff, local_104, param_4);
    string_1000_3cea(CONCAT22(param_3, param_2), CONCAT22(param_4, local_104));
    load_string_1010_84e0(0x1010, _PTR_LOOP_1050_14cc, 0x3ff, local_104, param_4);
    string_1000_3cea(CONCAT22(param_3, param_2), CONCAT22(param_4, local_104));
    MessageBox16(0x1000, 0x0, local_206, param_4);
    fn_ptr_1000_17ce(ctx, CONCAT22(param_3, param_2), 0x1000);
    return;
}


pub fn send_dlg_item_msg_1040_ce12(param_1: u16, param_2: u16, param_3: u32, param_4: u16, param_5: U32Ptr)

{
    let lVar1: i32;
    local_10a: u8[0x100];
    let local_a: [u8; 8];

    pass1_1008_5784(CONCAT22(param_5, local_a), param_3);
    loop {
        lVar1 = pass1_1008_5b12(local_a, param_5);
        if (lVar1 == 0x0) { break; }
        wsprintf16(0x1008, local_10a, param_5);
        SendDlgItemMessage16(ctx.s_tile2_bmp_1050_1538, local_10a, param_5, 0x0,
                             CONCAT22(param_4, 0x401));
    }
    return;
}


pub fn send_dlg_item_1040_ce76(param_1: u32, param_2: HWND16, param_3: u16) {
    let i_var1: i16;
    let uVar2: u16;
    let LVar3: LRESULT;
    let uVar4: u32;
    let local_106: [u8; 100];
    WStack6: WPARAM16;
    let iStack4: i16;

    // uVar2 = (param_1 >> 0x10);
    i_var1 = param_1;
    LVar3 = SendDlgItemMessage16(param_2, 0x0, 0x0, 0x0, 0x18420409);
    WStack6 = LVar3;
    iStack4 = WStack6 >> 0xf;
    if ((WStack6 != 0xffff) || (false)) {
        SendDlgItemMessage16(ctx.s_tile2_bmp_1050_1538, local_106, param_3, WStack6, 0x1842040a);
        uVar4 = pass1_1018_5206((i_var1 + 0x98), CONCAT22(param_3, local_106), param_3);
        if (uVar4 != 0x0) {
            (i_var1 + 0x9c) = (uVar4 + 0x8);
            (i_var1 + 0x9e) = 0x0;
            SetDlgItemInt16(0x1018, 0x0, 0x0, 0x18e);
            SetDlgItemInt16(ctx.s_tile2_bmp_1050_1538, 0x0, (i_var1 + 0x9c), 0x191);
        }
    }
    return;
}


// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

LRESULT  send_dlg_msg_1040_cf1c(param_1: u32,param_2: u16)

{
let in_DX: * mut u8; let uVar1: u16; let unaff_DI: i16; let uVar2: u16; let in_AF: u8; let LVar3: LRESULT; let enable: bool; let uVar4: u16; local_50c: u8 [0x402]; let uStack266: u32; let local_106: [u8;100]; let puStack6: * mut u16;

puStack6 = mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0, 0x3,param_2, in_DX, unaff_DI);
// uVar4 = (puStack6 >> 0x10);
// uVar2 = (param_1 >> 0x10);
uVar1 = param_1; pass1_1010_c3c2(puStack6, uVar4, CONCAT22(param_2, local_106),
(uVar1 + 0x94),uVar4, in_AF, param_2); SendDlgItemMessage16(0x1010, local_106, param_2, 0x0, 0x1846000c);
SendDlgItemMessage16(ctx.s_tile2_bmp_1050_1538, 0x0, 0x0,0x0, 0x1842000b); SendDlgItemMessage16(ctx.s_tile2_bmp_1050_1538, 0x0, 0x0, 0x0, 0x18420405);
uVar4 = (s_vrpal_bmp_1050_183a + 0x8); uStack266 = pass1_1018_526a((uVar1 + 0x98), (uVar1 + 0x94), param_2); send_dlg_item_msg_1040_ce12(uVar1, uVar2, uStack266, uVar4, param_2);
LVar3 = SendDlgItemMessage16(0x1018, 0x0, 0x0,0x0, 0x1842040c); if (((LVar3 > > 0x10) != 0x0 & & - 0x1 < LVar3) | | (( - 0x1 < LVar3 & & (LVar3 != 0x0)))) {
GetDlgItem16(ctx.s_tile2_bmp_1050_1538, 0x1); enable = 0x1;
}
else {
load_string_1010_84e0
(0x1010, _PTR_LOOP_1050_14cc,
(ctx.PTR__LOOP_1050_14cc > > 0x10), 0x3ff, local_50c, param_2); SendDlgItemMessage16(0x1010, local_50c, param_2,0x0, 0x18420401); GetDlgItem16(ctx.s_tile2_bmp_1050_1538, 0x1); enable = 0x0;
}
EnableWindow16(ctx.s_tile2_bmp_1050_1538, enable); LVar3 = SendDlgItemMessage16(ctx.s_tile2_bmp_1050_1538, 0x0, 0x0, 0x1, 0x1842000b); return LVar3;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn send_dlg_item_msg_1040_d20c(param_1: u32, param_2: i32, param_3: u16, param_4: u16, in_AF: u8, unaff_DI: i16, in_DX: U32Ptr, in_AX: u16) {
    let uVar2: u16;
    let puVar3: U32Ptr;
    let puVar4: U32Ptr;
    let uVar5: u16;
    let local_106: [u8; 104];

    if (param_2 == 0x0) {
        enable_win_1040_d60e(param_1, param_3);
        return;
    }
    // uVar2 = (param_1 >> 0x10);
    if ((param_1 + 0xa0) != 0x0) {
        pass1_1010_9210((param_1 + 0x9c));
        enable_win_1040_d60e(param_1, 0x1010);
        pass1_1030_8344(
            ctx, ctx.PTR__LOOP_1050_5748, param_2);
        puVar4 = local_106;
        uVar5 = param_4;
        puVar3 = mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0, 0x3, param_4, in_DX, unaff_DI);
        // uVar1 = (puVar3 >> 0x10);
        pass1_1010_c3c2(puVar3, uVar1, CONCAT22(uVar5, puVar4), CONCAT22(in_DX, in_AX),
                        uVar1, in_AF, param_4);
        SendDlgItemMessage16(0x1010, local_106, param_4, 0x0, 0x18470401);
    }
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn win_ui_op_1040_d2ac(param_1: i16, param_2: u16, param_3: u16, param_4: u32, param_5: u16, param_6: u16, param_7: u16)

{
    let LVar1: LRESULT;

    if (param_4._2_2_ == (s_dibtext_bmp_1050_1844 + 0x4)) {
        SendDlgItemMessage16(param_6, 0x0, 0x0, 0x0, 0x18470405);
        struct_1010_9172((param_1 + 0x9c));
    } else {
        if ((s_dibtext_bmp_1050_1844 + 0x4) < param_4._2_2_) {
            if (param_4._2_2_ == (s_dibtext_bmp_1050_1844 + 0x5)) {
                LVar1 = SendDlgItemMessage16(param_6, 0x0, 0x0, 0x0, 0x1847040c);
                if ((LVar1 != -0x1) || ((LVar1 >> 0x10) != -0x1)) {
                    SendDlgItemMessage16(ctx.s_tile2_bmp_1050_1538, 0x0, 0x0, LVar1 - 0x1, 0x18470403);
                    pass1_1010_91cc((param_1 + 0x9c));
                }
            } else {
                if (param_4._2_2_ == (s_dibtext_bmp_1050_1844 + 0x6)) {
                    enable_win_1040_d6be(CONCAT22(param_2, param_1), param_6);
                    pass1_1018_57d2((param_1 + 0x94), CONCAT22(param_2, param_1));
                    PostMessage16(0x1018, 0x0, 0x0, 0x1110203);
                } else {
                    if (param_4._2_2_ != (s_dibtext_bmp_1050_1844 + 0x7)) {
                        // goto
                        // LAB_1040_d3b3;
                    }
                    ctx._PTR_LOOP_1050_5a68 = (param_1 + 0x98);
                    pass1_1038_af40(ctx.PTR__LOOP_1050_5b7c, (param_1 + 0x6), 0x27, param_5,
                                    param_1, &ctx.PTR_LOOP_1050_1038, param_7);
                }
            }
        } else {
            if (param_4._2_2_ == 0xeb) {
                send_ldg_item_msg_1040_d79c(CONCAT22(param_2, param_1), param_7);
            } else {
                if (param_4._2_2_ != (s_vrpal_bmp_1050_183a + 0x7)) {
//LAB_1040_d3b3:
                    pass1_1040_b54a(param_1, param_2, param_3, param_4, param_5, param_6, param_7,
                    );
                    return;
                }
                msg_box_op_1040_d3d0(CONCAT22(param_2, param_1), 0x0, param_5, param_7);
            }
        }
    }
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn msg_box_op_1040_d3d0(param_1: u32, param_2: &mut String, param_3: U32Ptr, param_4: u16) {
    local_206: u8[0x102];
    local_104: u8[0x102];

    mem_op_1000_179c(0x1000, param_3, 0x1000);
    load_string_1010_84e0(0x1010, _PTR_LOOP_1050_14cc, 0x100, local_206, param_4);
    load_string_1010_84e0(0x1010, _PTR_LOOP_1050_14cc,
                          (ctx.PTR__LOOP_1050_14cc >> 0x10), 0x3ff, param_2, (short)param_3);
    load_string_1010_84e0(0x1010, _PTR_LOOP_1050_14cc, 0x3ff, local_104, param_4);
    string_1000_3cea(CONCAT22(param_3, param_2), CONCAT22(param_4, local_104));
    load_string_1010_84e0(0x1010, _PTR_LOOP_1050_14cc, 0x3ff, local_104, param_4);
    string_1000_3cea(CONCAT22(param_3, param_2), CONCAT22(param_4, local_104));
    load_string_1010_84e0(0x1010, _PTR_LOOP_1050_14cc, 0x3ff, local_104, param_4);
    string_1000_3cea(CONCAT22(param_3, param_2), CONCAT22(param_4, local_104));
    load_string_1010_84e0(0x1010, _PTR_LOOP_1050_14cc, 0x3ff, local_104, param_4);
    string_1000_3cea(CONCAT22(param_3, param_2), CONCAT22(param_4, local_104));
    load_string_1010_84e0(0x1010, _PTR_LOOP_1050_14cc, 0x3ff, local_104, param_4);
    string_1000_3cea(CONCAT22(param_3, param_2), CONCAT22(param_4, local_104));
    MessageBox16(0x1000, 0x0, local_206, param_4);
    load_string_1010_84e0(0x1010, _PTR_LOOP_1050_14cc,
                          (ctx.PTR__LOOP_1050_14cc >> 0x10), 0x3ff, param_2, (short)param_3);
    load_string_1010_84e0(0x1010, _PTR_LOOP_1050_14cc, 0x3ff, local_104, param_4);
    string_1000_3cea(CONCAT22(param_3, param_2), CONCAT22(param_4, local_104));
    load_string_1010_84e0(0x1010, _PTR_LOOP_1050_14cc, 0x3ff, local_104, param_4);
    string_1000_3cea(CONCAT22(param_3, param_2), CONCAT22(param_4, local_104));
    load_string_1010_84e0(0x1010, _PTR_LOOP_1050_14cc, 0x3ff, local_104, param_4);
    string_1000_3cea(CONCAT22(param_3, param_2), CONCAT22(param_4, local_104));
    load_string_1010_84e0(0x1010, _PTR_LOOP_1050_14cc, 0x3ff, local_104, param_4);
    string_1000_3cea(CONCAT22(param_3, param_2), CONCAT22(param_4, local_104));
    load_string_1010_84e0(0x1010, _PTR_LOOP_1050_14cc, 0x3ff, local_104, param_4);
    string_1000_3cea(CONCAT22(param_3, param_2), CONCAT22(param_4, local_104));
    MessageBox16(0x1000, 0x0, local_206, param_4);
    fn_ptr_1000_17ce(ctx, CONCAT22(param_3, param_2), 0x1000);
    return;
}


pub fn enable_win_1040_d60e(param_1: u32, param_2: HWND16) {
    GetDlgItem16(param_2, 0x1);
    EnableWindow16(ctx.s_tile2_bmp_1050_1538, 0x1);
    GetDlgItem16(ctx.s_tile2_bmp_1050_1538, 0x2);
    EnableWindow16(ctx.s_tile2_bmp_1050_1538, 0x1);
    GetDlgItem16(ctx.s_tile2_bmp_1050_1538, (s_vrpal_bmp_1050_183a + 0x7));
    EnableWindow16(ctx.s_tile2_bmp_1050_1538, 0x1);
    GetDlgItem16(ctx.s_tile2_bmp_1050_1538, (s_dibtext_bmp_1050_1844 + 0x4));
    EnableWindow16(ctx.s_tile2_bmp_1050_1538, 0x1);
    GetDlgItem16(ctx.s_tile2_bmp_1050_1538, (s_dibtext_bmp_1050_1844 + 0x5));
    EnableWindow16(ctx.s_tile2_bmp_1050_1538, 0x1);
    GetDlgItem16(ctx.s_tile2_bmp_1050_1538, (s_dibtext_bmp_1050_1844 + 0x6));
    EnableWindow16(ctx.s_tile2_bmp_1050_1538, 0x1);
    GetDlgItem16(ctx.s_tile2_bmp_1050_1538, (s_dibtext_bmp_1050_1844 + 0x7));
    EnableWindow16(ctx.s_tile2_bmp_1050_1538, 0x1);
    (param_1 + 0xa0) = 0x0;
    return;
}


pub fn enable_win_1040_d6be(param_1: u32, param_2: HWND16) {
    GetDlgItem16(param_2, 0x1);
    EnableWindow16(ctx.s_tile2_bmp_1050_1538, 0x0);
    GetDlgItem16(ctx.s_tile2_bmp_1050_1538, 0x2);
    EnableWindow16(ctx.s_tile2_bmp_1050_1538, 0x0);
    GetDlgItem16(ctx.s_tile2_bmp_1050_1538, (s_vrpal_bmp_1050_183a + 0x7));
    EnableWindow16(ctx.s_tile2_bmp_1050_1538, 0x0);
    GetDlgItem16(ctx.s_tile2_bmp_1050_1538, (s_dibtext_bmp_1050_1844 + 0x4));
    EnableWindow16(ctx.s_tile2_bmp_1050_1538, 0x0);
    GetDlgItem16(ctx.s_tile2_bmp_1050_1538, (s_dibtext_bmp_1050_1844 + 0x5));
    EnableWindow16(ctx.s_tile2_bmp_1050_1538, 0x0);
    GetDlgItem16(ctx.s_tile2_bmp_1050_1538, (s_dibtext_bmp_1050_1844 + 0x6));
    EnableWindow16(ctx.s_tile2_bmp_1050_1538, 0x0);
    GetDlgItem16(ctx.s_tile2_bmp_1050_1538, (s_dibtext_bmp_1050_1844 + 0x7));
    EnableWindow16(ctx.s_tile2_bmp_1050_1538, 0x0);
    (param_1 + 0xa0) = 0x1;
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn send_ldg_item_msg_1040_d79c(param_1: u32, param_2: u16) {
    let uVar1: u16;
    let in_DX: U32Ptr;
    let uVar2: u16;
    let uVar3: u16;
    let iVar4: i16;
    let unaff_DI: i16;
    let uVar5: u16;
    let hwnd: HWND16;
    let in_AF: u8;
    let LVar6: LRESULT;
    let uStack270: u32;
    let uStack266: u32;
    local_106: u8[0x100];
    let puStack6: U32Ptr;

    puStack6 = mixed_1010_20ba(ctx.PTR__LOOP_1050_0ed0, 0x3, param_2, in_DX, unaff_DI);
    // uVar2 = (puStack6 >> 0x10);
    // uVar5 = (param_1 >> 0x10);
    iVar4 = param_1;
    pass1_1010_c3c2(puStack6, uVar2, CONCAT22(param_2, local_106),
                    (iVar4 + 0x98), uVar2, in_AF, param_2);
    SendDlgItemMessage16(0x1010, local_106, param_2, 0x0, 0x1846000c);
    SendDlgItemMessage16(ctx.s_tile2_bmp_1050_1538, 0x0, 0x0, 0x0, 0x1847000b);
    LVar6 = SendDlgItemMessage16(ctx.s_tile2_bmp_1050_1538, 0x0, 0x0, 0x0, 0x18470405);
    // uVar3 = (LVar6 >> 0x10);
    uVar1 = LVar6;
    hwnd = 0x1010;
    pass1_1010_9044((iVar4 + 0x9c));
    uStack266 = CONCAT22(uVar3, uVar1);
    // for (uStack270 = 0x0; uStack270 < uStack266; uStack270 += 0x1) {
    //   hwnd = 0x1010;
    //   pass1_1010_9130((iVar4 + 0x9c),CONCAT22(param_2,local_106),
    //                   local_106,uVar3,param_2,in_AF);
    //   if (local_106[0] != '\0') {
    //     hwnd = ctx.s_tile2_bmp_1050_1538;
    //     LVar6 = SendDlgItemMessage16(0x1010,local_106,param_2,0x0,0x18470401);
    //     uVar3 = (LVar6 >> 0x10);
    //   }
    // }
    SendDlgItemMessage16(hwnd, 0x0, 0x0, 0x1, 0x1847000b);
    return;
}

