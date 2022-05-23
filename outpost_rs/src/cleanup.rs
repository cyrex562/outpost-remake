use crate::defines::{
    Struct1, Struct11, Struct18, Struct27, Struct28, Struct29, Struct3, Struct30, Struct31,
    Struct34, Struct35, Struct43, Struct65, StructB, Struct_1008_628e, Struct_1010_2fa0,
    Struct_1010_7b26, U32Ptr,
};
use crate::draw::draw_1020::invalidate_rect_1020_735a;
use crate::file::file_1010::unk_io_op_1010_830a;
use crate::fn_ptr::fn_ptr_1000::fn_ptr_1000_17ce;
use crate::fn_ptr::util::get_fn_ptr_2;
use crate::global::AppContext;
use crate::pass::pass_1008::{
    pass1_1008_3e0e, pass1_1008_5784, pass1_1008_57c4, pass1_1008_5b12, pass1_1008_b544,
    pass1_1008_eb74,
};
use crate::pass::pass_1010::{
    pass1_1010_1d80, pass1_1010_1dda, pass1_1010_1ea6, pass1_1010_1f62, pass1_1010_2ee2,
    pass1_1010_32c0, pass1_1010_32da,
};
use crate::pass::pass_1018::{
    pass1_1018_2d9a, pass1_1018_2dde, pass1_1018_2e28, pass1_1018_31d0, pass1_1020_022c,
};
use crate::pass::pass_1020::pass1_1020_808e;
use crate::pass::pass_1038::pass1_1038_b6e0;
use crate::pass::pass_1040::{pass1_1040_b54a, pass1_1040_c60e};
use crate::sys_api::unk_win_msg_op_1008_9510;
use crate::ui::ui_1008::{set_sys_color_1008_357e, win_1008_5c7c};
use crate::ui::ui_1010::win_ui_op_1010_3202;
use crate::ui::ui_1020::{post_win_msg_1020_1ca4, win_ui_fn_1020_6e98};
use crate::ui::ui_1040::{
    dialog_ui_fn_1040_78e2, mov_update_win_1040_93aa, move_win_1040_826c, post_win_msg_1040_7b3c,
};
use crate::util::{read_struct_from_addr, CONCAT22};
use crate::win_platform::types::{HDC16, HGDIOBJ16, HICON16, HMENU16, HWND16};
use crate::win_platform::winapi::DeleteObject16;
use crate::win_struct::{
    HDC16, HGDIOBJ16, HICON16, HMENU16, HWND16, LPARAM, PAINTSTRUCT16, RECT16, WNDCLASS16,
};
use crate::winapi::{
    BeginPaint16, DeleteDC16, DeleteObject16, DestroyCursor16, DestroyIcon16, DestroyMenu16,
    DestroyWindow16, EndPaint16, GetClientRect16, GetDlgItem16, GetWindowWord16, InvalidateRect16,
    IsDlgButtonChecked, IsWindow16, PostMessage16, RemoveProp16, SelectObject16, SelectPalette16,
    SendMessage16, ShowWindow16, WinHelp16,
};

pub fn cleanup_ui_op_1008_0618(
    ctx: &mut AppContext,
    param_1: &mut StructB,
    unaff_cs: u16,
    unaff_ss: u16,
) {
    let pu_var1: u32;
    let u_var2: u16;
    // let paVar3: &mut Struct18;
    let mut pa_var3: &mut Struct18;
    // let ppc_var4: u32;
    let mut ppc_var4: u32;
    // let i_var5: i16;
    let u_var6: u16;
    // let unaff_cs: u16;
    // let h_icon: HICON16;
    let mut h_icon: HICON16;
    // let unaff_ss: u16;
    let u_var7: u16;
    let u_var8: u16;

    // u_var6 = (param_1 >> 0x10);
    // i_var5 = param_1;
    param_1.field_0x0 = 0x389e;
    param_1.field_0x2 = 0x1008;
    set_sys_color_1008_357e(param_1, 0x0, unaff_cs as i16, unaff_ss);
    pa_var3 = &mut param_1.field_0xf8; // (i_var5 + 0xf8);
                                       // u_var8 = (pa_var3 >> 0x10);
    h_icon = 0x1000;
    fn_ptr_1000_17ce(ctx, pa_var3, 0x1000);
    if (param_1.field_0xec) != 0x0 {
        u_var8 = param_1.field_0xec;
        h_icon = ctx.s_tile2_bmp_1050_1538 as HICON16;
        DestroyMenu16(0x1000);
    }
    u_var7 = param_1.field_0xc2;
    DestroyIcon16(h_icon);
    (param_1.field_0xc2) = 0x0;
    pu_var1 = param_1.field_0xe0;
    u_var2 = param_1.field_0xe2;
    if (u_var2 | pu_var1) != 0x0 {
        ppc_var4 = *pu_var1;
        (**ppc_var4)(
            ctx.s_tile2_bmp_1050_1538,
            pu_var1,
            u_var2,
            0x1,
            u_var7,
            pa_var3,
        );
    }
    pass1_1008_57c4((param_1 & 0xffff0000 | (param_1.field_0xd2)));
    param_1.field_0x0 = 0x380a;
    (param_1.field_0x2) = 0x1008;
    param_1.field_0x0 = 0x389a;
    (param_1.field_0x2) = 0x1008;
    return;
}

pub fn destroy_win_1008_628e(param_1: &mut Struct_1008_628e, param_2: HWND16) {
    let mut ppc_var1: u32;
    ppc_var1 = param_1.field_0xd2.field_0x14;
    let fn_ptr = get_fn_ptr_2(ppc_var1);
    fn_ptr(param_2, (param_1.field_0xd2), param_1.field_0x2);
    DestroyWindow16(param_2);
    (param_1.field_0x8) = 0x0;
    return;
}

pub fn destroy_win_1008_9698(param_1: HWND16) {
    DestroyWindow16(param_1);
    return;
}

pub fn unk_destroy_win_op_1010_2fa0(
    ctx: &mut AppContext,
    struct_1: &mut Struct_1010_2fa0,
    win_handle_1: &mut HWND16,
) {
    let pi_var1: U32Ptr;
    let u_var2: u32;
    let struct_2: &mut Struct_1010_2fa0;
    let u_var4: u16;
    let mut win_handle_2: HWND16;
    let i_stack4: i16;

    // u_var4 = (param_1 >> 0x10);
    struct_2 = struct_1;
    (struct_2.field_0x28) = 0x0;
    i_stack4 = 0x0;
    loop {
        pi_var1 = struct_2.field_0x16;
        if *pi_var1 == i_stack4 || *pi_var1 < i_stack4 {
            break;
        }
        DestroyWindow16(*win_handle_1);
        i_stack4 += 0x1;
        *win_handle_1 = ctx.s_tile2_bmp_1050_1538 as HWND16;
    }
    (struct_2.field_0x16) = 0x0;
    if ((struct_2.field_0x54) | (struct_2.field_0x52)) != 0x0 {
        i_stack4 = 0x0;
        loop {
            u_var2 = struct_2.field_0x52;
            win_handle_2 = *win_handle_1;
            if (u_var2 + i_stack4 * 0x4) != 0x0 {
                win_handle_2 = ctx.s_tile2_bmp_1050_1538 as HWND16;
                DestroyWindow16(*win_handle_1);
                u_var2 = struct_2.field_0x52;
                (u_var2 + i_stack4 * 0x4) = 0x0;
            }
            i_stack4 += 0x1;
            *win_handle_1 = win_handle_2;
            if i_stack4 >= 0xa {
                break;
            }
        }
        fn_ptr_1000_17ce(ctx, struct_2 + 0x52, 0x1000);
        (struct_2 + 0x52) = 0x0;
    }
    return;
}

pub fn unk_destroy_win_op_1010_305a(
    ctx: &mut AppContext,
    struct_1: &mut Struct27,
    param_2: i16,
    struct_2: &mut Struct65,
    param_4: u16,
    unaff_ss: u16,
) {
    let pi_var1: U32Ptr;
    let u_var2: i32;
    let l_var3: i32;
    let b_var4: bool;
    let u_var5: u16;
    // Struct27 *i_var4;
    let mut struct_3: &mut Struct27;
    let i_var6: i16;
    // Struct27 *uVar7;
    let mut struct_4: &mut Struct27;
    let u_var8: u16;
    // let hwnd: HWND16;
    let mut win_handle: HWND16;
    // let hwnd_00: HWND16;
    let mut hwnd_00: HWND16;
    // let unaff_SS: u16;
    let u_stack10: i16;
    let i_stack8: i16;
    let i_stack6: i16;

    win_handle = ctx.PTR_LOOP_1050_1040 as HWND16;
    u_var5 = pass1_1040_c60e(struct_2);
    // uVar7 = (param_1 >> 0x10);
    struct_3 = struct_1;
    struct_3.field_0x12 = u_var5;
    struct_3.field_0x14 = 0x0;
    i_stack6 = 0x0;
    b_var4 = false;
    struct_3.field_0x28 = 0x0;
    i_stack8 = 0x0;
    loop {
        pi_var1 = &struct_3.field_0x16;
        if *pi_var1 == i_stack8 || *pi_var1 < i_stack8 {
            //LAB_1010_30ad:
            i_var6 = i_stack6;
            if b_var4 {
                while (
                    i_stack8 = i_var6 + 0x1,
                    pi_var1 = &struct_3.field_0x16,
                    *pi_var1 != i_stack8 && i_stack8 <= *pi_var1,
                ) {
                    DestroyWindow16(win_handle);
                    (&struct_3.field_0x2e)[i_var6] = 0x0;
                    win_handle = ctx.s_tile2_bmp_1050_1538;
                    i_var6 = i_stack8;
                }
                struct_3.field_0x16 = i_stack6 + 0x1;
                pass1_1010_1f62(unaff_ss, struct_1, 0x9);
            } else {
                i_var6 = struct_3.field_0x16;
                (&struct_3.field_0x2a)[i_var6 * 0x2] = struct_2;
                (&struct_3.field_0x2c)[i_var6 * 0x2] = (struct_2 >> 0x10);
                let mut i_stack10 = 0xa;
                pi_var1 = &struct_3.field_0x16;
                *pi_var1 = *pi_var1 + 0x1;
                if 0x1 < struct_3.field_0x16 {
                    u_var2 = (&struct_3.field_0x22)[struct_3.field_0x16];
                    i_var6 = u_var2;
                    // u_var8 = (u_var2 >> 0x10);
                    i_stack10 = (i_var6 + 0x20) + (i_var6 + 0x24) + 0x8;
                }
                win_handle = &ctx.PTR_LOOP_1050_1040;
                mov_update_win_1040_93aa(
                    ctx,
                    struct_2,
                    i_stack10,
                    struct_3.field_0x1a,
                    ctx.PTR_LOOP_1050_1040 as u16,
                );
            }
            if !b_var4 {
                pass1_1010_1f62(unaff_ss, struct_1, 0xa);
            }
            if param_2 == 0x0 {
                if struct_3.field_0x52 != 0x0 {
                    i_stack8 = 0x0;
                    hwnd_00 = win_handle;
                    loop {
                        l_var3 = struct_3.field_0x52;
                        // u_var8 = (l_var3 >> 0x10);
                        i_var6 = l_var3 as i16;
                        win_handle = hwnd_00;
                        if ((i_var6 + i_stack8 * 0x4) != 0x0)
                            && ((i_var6 + i_stack8 * 0x4) != struct_2)
                        {
                            win_handle = ctx.s_tile2_bmp_1050_1538 as HWND16;
                            DestroyWindow16(hwnd_00);
                        }
                        l_var3 = struct_3.field_0x52;
                        (l_var3 + i_stack8 * 0x4) = 0x0;
                        i_stack8 += 0x1;
                        hwnd_00 = win_handle;
                        if i_stack8 >= 0xa {
                            break;
                        }
                    }
                }
                pass1_1010_32da(struct_1, struct_2, win_handle, unaff_ss);
                pass1_1010_1f62(unaff_ss, struct_1, 0x8);
            }
            return;
        }
        if (&struct_3.field_0x2a + i_stack8 * 0x2) == struct_2 {
            b_var4 = true;
            i_stack6 = i_stack8;
            // goto LAB_1010_30ad;
        }
        i_stack8 += 0x1;
    }
}

pub fn destroy_window_1010_7b26(
    struct_1: &mut Struct_1010_7b26,
    param_2: i32,
    param_3: u16,
    param_4: &mut u16,
    extraout_dx: u16,
) -> u32 {
    let u_var1: u16;
    let pu_var2: U32Ptr;
    let struct_2: &mut Struct_1010_7b26;
    let u_var4: u16;
    let local_a: [u8; 0x8];

    struct_2 = struct_1;
    u_var1 = (struct_2.field_0x1e) | (struct_2.field_0x1c);
    if u_var1 != 0x0 {
        pass1_1008_5784(CONCAT22(param_3, local_a), (struct_2 + 0x1c));
        loop {
            pu_var2 = local_a;
            pass1_1008_5b12(pu_var2, param_3);
            param_4 = extraout_dx | pu_var2;
            if (param_4 == 0x0) {
                break;
            }
            if (pu_var2 + 0x4) == param_2 {
                break;
            }
        }
        u_var1 = extraout_dx | pu_var2;
        if (u_var1 != 0x0) {
            u_var1 = DestroyWindow16(0x1008);
        }
    }
    return CONCAT22(u_var1, param_4);
}

pub fn clenaup_win_ui_1018_4d22(
    ctx: &mut AppContext,
    struct_1: &mut Struct18,
    draw_ctx_1: &mut HDC16,
    unaff_ss: u16,
) {
    let u_var1: u16;
    let func_ptr_1: u32;
    let struct_2: &mut Struct18;
    let struct_3: &mut Struct11;
    let pu_var2: U32Ptr;
    let pu_var1: U32Ptr;

    // u_var4 = (in_struct_1 >> 0x10);
    struct_2 = struct_1;
    *struct_1 =
        read_struct_from_addr::<Struct18>(ctx.s_SCInternalPutBldg_site_0x_08lx__b_1050_5046 + 0x12)
            .clone();
    struct_2.field_0x2 = 0x1018;
    if struct_2.field_0x12 != 0x0 {
        SelectPalette16(*draw_ctx_1, 0x0, struct_2.field_0x1a);
        DeleteObject16(ctx.s_tile2_bmp_1050_1538 as HGDIOBJ16);
        *draw_ctx_1 = ctx.s_tile2_bmp_1050_1538 as HDC16;
        DeleteDC16(ctx.s_tile2_bmp_1050_1538 as HDC16);
    }
    pu_var1 = struct_2.field_0xa;
    u_var1 = struct_2.field_0xc;
    if (u_var1 | pu_var1) != 0x0 {
        func_ptr_1 = *pu_var1;
        (**func_ptr_1)(draw_ctx_1, pu_var1, u_var1, 0x1);
    }
    pu_var2 = struct_2.field_0xe;
    u_var1 = struct_2.field_0x10;
    if (u_var1 | pu_var2) != 0x0 {
        func_ptr_1 = *pu_var2;
        (**func_ptr_1)(draw_ctx_1, pu_var2, u_var1, 0x1);
    }
    ctx.PTR_LOOP_1050_4230 = 0x0;
    pass1_1010_1d80(struct_1, unaff_ss);
    return;
}

pub fn unk_destroy_window_op_1018_6bb6(
    ctx: &mut AppContext,
    param_1: &mut Struct28,
    param_2: HWND16,
) {
    let b_var1: bool;
    let i_var2: &mut Struct28;
    let u_var2: u16;
    // let hwnd: HWND16;
    let hwnd: HWND16;

    // u_var2 = (param_1 >> 0x10);
    i_var2 = param_1;
    hwnd = param_2;
    if (i_var2.field_0xea != 0x0) {
        hwnd = ctx.s_tile2_bmp_1050_1538;
        PostMessage16(param_2, 0x0, 0x0, CONCAT22(0x111, i_var2.field_0xea));
    }
    PostMessage16(hwnd, 0x0, 0x0, 0x1110079);
    if (i_var2.field_0xf0 != 0x0) {
        b_var1 = IsWindow16(ctx.s_tile2_bmp_1050_1538);
        if (b_var1 != 0x0) {
            DestroyWindow16(ctx.s_tile2_bmp_1050_1538);
            i_var2.field_0xf0 = 0x0;
        }
    }
    return;
}

pub fn destroy_window_1018_c518(ctx: &mut AppContext, param_1: &mut Struct29) {
    let b_var1: bool;
    let i_var2: &mut Struct29;
    let u_var3: u16;

    // u_var3 = (param_1 >> 0x10);
    i_var2 = param_1;
    param_1.field_0x0 = 0xc8bc;
    i_var2.field_0x2 = 0x1018;
    fn_ptr_1000_17ce(ctx, &mut i_var2.field_0x108, 0x1000);
    if i_var2.field_0x112 != 0x0 {
        b_var1 = IsWindow16(0x1000);
        if b_var1 != false {
            DestroyWindow16(ctx.s_tile2_bmp_1050_1538);
            i_var2.field_0x112 = 0x0;
        }
    }
    pass1_1020_022c(param_1);
    return;
}

pub fn delete_palette_1018_e16c(ctx: &mut AppContext, param_1: u32, param_2: HWND16) {
    let pu_var1: u32;
    let func_ptr_2: u32;
    let u_var3: u32;
    let b_force_background: *mut HDC16;
    let local_24: HDC16;
    let local_22: PAINTSTRUCT16;

    local_24 = BeginPaint16(param_2, &local_22);
    u_var3 = (param_1 + 0x6);
    pu_var1 = (u_var3 + 0xa);
    b_force_background = &local_24;
    u_var3 = *pu_var1;
    func_ptr_2 = (u_var3 + 0x8);
    (**func_ptr_2)(
        ctx.s_tile2_bmp_1050_1538,
        pu_var1,
        (pu_var1 >> 0x10),
        b_force_background,
    );
    func_ptr_2 = (u_var3 + 0x4);
    (**func_ptr_2)(ctx.s_tile2_bmp_1050_1538, pu_var1, 0x0, &local_24);
    SelectPalette16(ctx.s_tile2_bmp_1050_1538, 0x0, b_force_background);
    DeleteObject16(ctx.s_tile2_bmp_1050_1538);
    EndPaint16(ctx.s_tile2_bmp_1050_1538, &local_22);
    return;
}

pub fn cleanup_ui_op_1020_1038(ctx: &mut AppContext, param_1: u32, unaff_CS: HICON16) {
    let pu_var1: u32;
    let u_var2: u16;
    let ppc_var3: u32;
    let i_var4: i16;
    let u_var5: u16;
    // let unaff_CS: HICON16;
    let u_var6: u16;

    // u_var5 = (param_1 >> 0x10);
    i_var4 = param_1;
    u_var6 = (i_var4 + 0xc2);
    DestroyIcon16(unaff_CS);
    (i_var4 + 0xc2) = 0x0;
    (i_var4 + 0x8) = 0x0;
    pu_var1 = (i_var4 + 0xf6);
    u_var2 = (i_var4 + 0xf8);
    if ((u_var2 | pu_var1) != 0x0) {
        ppc_var3 = *pu_var1;
        (**ppc_var3)(ctx.s_tile2_bmp_1050_1538, pu_var1, u_var2, 0x1, u_var6);
    }
    (i_var4 + 0xf6) = 0x0;
    pass1_1010_1dda((i_var4 + 0xf2));
    (i_var4 + 0xf2) = 0x0;
    return;
}

pub fn destroy_window_1020_1d4a(ctx: &mut AppContext, param_1: i32, param_2: i16, param_3: HWND16) {
    let b_var1: bool;
    let hwnd: HWND16;

    if (param_2 != 0x0) {
        b_var1 = post_win_msg_1020_1ca4(param_1);
        if (b_var1 != 0x0) {
            hwnd = param_3;
            if ((param_1 + 0x96) != 0x0) {
                hwnd = ctx.s_tile2_bmp_1050_1538;
                PostMessage16(param_3, 0x0, 0x0, 0x11100ee);
            }
            DestroyWindow16(hwnd);
        }
    }
    return;
}

pub fn destroy_win_1020_1dea(ctx: &mut AppContext, param_1: HWND16) -> bool {
    let b_var1: bool;
    let w_var2: u16;

    b_var1 = IsWindow16(param_1);
    if (b_var1 != 0x0) {
        w_var2 = GetWindowWord16(ctx.s_tile2_bmp_1050_1538, -0xc);
        if (w_var2 == 0x175) {
            DestroyWindow16(ctx.s_tile2_bmp_1050_1538);
            return 0x0;
        }
    }
    return 0x1;
}

pub fn destroy_win_1020_1e1e(ctx: &mut AppContext, param_1: HWND16) -> u16 {
    let b_var1: bool;
    let w_var2: u16;

    b_var1 = IsWindow16(param_1);
    if (b_var1 != 0x0) {
        w_var2 = GetWindowWord16(ctx.s_tile2_bmp_1050_1538, -0xc);
        if ((w_var2 != 0x1) && (w_var2 != 0x175)) {
            DestroyWindow16(ctx.s_tile2_bmp_1050_1538);
        }
    }
    return 0x1;
}

pub fn destroy_icon_1020_2c88(ctx: &mut AppContext, param_1: u32, param_2: HICON16) {
    let pu_var1: u32;
    let u_var2: u16;
    let ppc_var3: u32;
    let i_var4: i16;
    let u_var5: u16;
    let u_var6: u16;

    // u_var5 = (param_1 >> 0x10);
    i_var4 = param_1;
    u_var6 = (i_var4 + 0xc2);
    DestroyIcon16(param_2);
    (i_var4 + 0xc2) = 0x0;
    (i_var4 + 0x8) = 0x0;
    pu_var1 = (i_var4 + 0xf6);
    u_var2 = (i_var4 + 0xf8);
    if ((u_var2 | pu_var1) != 0x0) {
        ppc_var3 = *pu_var1;
        (**ppc_var3)(ctx.s_tile2_bmp_1050_1538, pu_var1, u_var2, 0x1, u_var6);
    }
    (i_var4 + 0xf6) = 0x0;
    pass1_1010_1dda((i_var4 + 0xf2));
    (i_var4 + 0xf2) = 0x0;
    return;
}

pub fn cleanup_win_ui_1020_2fea(
    ctx: &mut AppContext,
    in_struct_1: &mut Struct11,
    mut in_dc_handle_2: HDC16,
    unaff_SS: u16,
) {
    let i_var1: &mut Struct11;
    let var2: u16;
    // let unaff_SS: u16;

    // var2 = (in_struct_1 >> 0x10);
    i_var1 = in_struct_1;
    in_struct_1.offset_field_0x0 = 0x363c;
    i_var1.offset_field_0x2 = 0x1020;
    if i_var1.field_0x14 != 0x0 {
        in_dc_handle_2 = 0x1010;
        pass1_1010_1ea6(
            i_var1.field_0x14,
            in_struct_1 & 0xffff | var2 << 0x10,
            unaff_SS,
        );
    }
    SelectObject16(in_dc_handle_2, i_var1.field_0x1c);
    SelectObject16(ctx.s_tile2_bmp_1050_1538, i_var1.field_0x1e);
    DeleteObject16(ctx.s_tile2_bmp_1050_1538);
    SelectPalette16(ctx.s_tile2_bmp_1050_1538, 0x0, i_var1.field_0x20);
    DeleteObject16(ctx.s_tile2_bmp_1050_1538);
    DeleteDC16(ctx.s_tile2_bmp_1050_1538);
    in_struct_1.offset_field_0x0 = 0x3ab0;
    i_var1.offset_field_0x2 = 0x1008;
    in_struct_1.offset_field_0x0 = 0x389a;
    i_var1.offset_field_0x2 = 0x1008;
    return;
}

pub fn destroy_window_1020_3b3e(
    ctx: &mut AppContext,
    param_1: &mut Struct30,
    param_2: HWND16,
    unaff_ss: u16,
) {
    let pu_var1: u32;
    let ppc_var2: u32;
    let u_var3: u16;
    let struct_1: Option<&mut Struct30>;
    let struct_2: &mut Struct30;
    let struct_3: &mut Struct30;
    let win_handle_1: HWND16;
    // let unaff_ss: u16;

    // u_var6 = (param_1 >> 0x10);
    struct_2 = param_1;
    struct_2.field_0x10e = 0x0;
    win_handle_1 = param_2;
    if struct_2.field_0x10a != 0x0 {
        win_handle_1 = ctx.s_tile2_bmp_1050_1538;
        DestroyWindow16(param_2);
    }
    pu_var1 = struct_2.field_0xf6;
    u_var3 = struct_2.field_0xf8;
    if (u_var3 | pu_var1) != 0x0 {
        ppc_var2 = *pu_var1;
        (**ppc_var2)(win_handle_1, pu_var1);
    }
    struct_2.field_0xf6 = 0x0;
    if struct_2.field_0xfa != 0x0 {
        u_var3 = struct_3 | struct_2;
        if param_1 == 0x0 {
            struct_1 = None;
        } else {
            u_var3 = struct_2.field_0xf2;
            struct_1 = struct_3;
        }
        pass1_1010_1ea6(struct_2.field_0xfa, CONCAT22(struct_1, u_var3), unaff_ss);
    }
    struct_2.field_0xfa = 0x0;
    return;
}

pub unsafe fn destroy_cursor_1020_42f4(ctx: &mut AppContext, param_1: U32Ptr, param_2: u16) {
    let i_var1: i16;
    let u_var2: u16;
    let h_cursor: HMENU16;

    // u_var2 = (param_1 >> 0x10);
    i_var1 = param_1;
    *param_1 = 0x623c;
    (i_var1 + 0x2) = 0x1020;
    (i_var1 + 0xe2) = 0x62d8;
    (i_var1 + 0xe4) = 0x1020;
    h_cursor = param_2;
    if ((i_var1 + 0x106) != 0x0) {
        h_cursor = ctx.s_tile2_bmp_1050_1538;
        DestroyMenu16(param_2);
    }
    DestroyCursor16(h_cursor);
    DestroyCursor16(ctx.s_tile2_bmp_1050_1538);
    pass1_1020_808e(param_1);
    return;
}

pub fn unk_destroy_win_op_1020_694c(
    ctx: &mut AppContext,
    param_1: i32,
    param_2: u16,
    param_3: HWND16,
    param_4: u16,
) -> u16 {
    let u_var1: u32;
    let u_var2: u16;
    let HVar3: HWND16;
    let i_var4: i16;
    let paVar5: &mut Struct43;
    let u_var6: u16;

    u_var2 = param_2;
    if (param_2 != 0x12b) {
        i_var4 = param_1;
        // u_var6 = (param_1 >> 0x10);
        if (param_2 < 0x12c) {
            if (param_2 == 0x6f) {
                paVar5 = unk_io_op_1010_830a(ctx.PTR_LOOP_1050_14cc, 0x1f8, param_4);
                u_var2 = WinHelp16(
                    0x1010,
                    (ctx.s_New_failed_in_Op__Op_1050_0020 + 0x9),
                    0x0,
                    CONCAT22(paVar5, 0x1),
                );
                return u_var2;
            }
            if (param_2 == 0xeb) {
                u_var2 = GetDlgItem16(param_3, 0x1797);
                if (u_var2 != 0x0) {
                    //LAB_1020_6a6f:
                    win_ui_fn_1020_6e98(param_1, ctx.s_tile2_bmp_1050_1538, param_4);
                    return u_var2;
                }
            } else {
                u_var2 = param_2 - 0xef;
                if (u_var2 == 0x0) {
                    pass1_1018_2e28((i_var4 + 0xf2));
                    pass1_1008_3e0e(param_1);
                } else {
                    u_var2 = param_2 - 0x129;
                    if ((u_var2 != 0x0) && (u_var2 = param_2 - 0x12a, u_var2 == 0x0)) {
                        u_var6 = 0xf012;
                        //LAB_1020_69c3:
                        u_var2 = PostMessage16(param_3, 0x0, 0x0, CONCAT22(0x112, u_var6));
                        return u_var2;
                    }
                }
            }
        } else {
            if (param_2 == 0xbb8) {
                HVar3 = GetDlgItem16(param_3, 0x1797);
                if (HVar3 != 0x0) {
                    DestroyWindow16(ctx.s_tile2_bmp_1050_1538);
                }
                u_var2 = pass1_1018_31d0((i_var4 + 0xf2));
                if (u_var2 != 0x0) {
                    u_var1 = (i_var4 + 0xf2);
                    u_var2 = pass1_1018_2d9a(u_var1, (u_var1 >> 0x10));
                    //LAB_1020_6a0b:
                    invalidate_rect_1020_735a((i_var4 + 0xf6), 0x1018);
                    return u_var2;
                }
            } else {
                if (param_2 < 0xbb9) {
                    if (param_2 == 0x12c) {
                        u_var6 = 0xf020;
                        //             TODO: goto LAB_1020_69c3;
                    }
                    u_var2 = param_2 - 0x12d;
                    if (param_2 != 0x12c) {
                        u_var2 = param_2 - 0x12e;
                    }
                } else {
                    if (param_2 == 0xbb9) {
                        HVar3 = GetDlgItem16(param_3, 0x1797);
                        if (HVar3 != 0x0) {
                            DestroyWindow16(ctx.s_tile2_bmp_1050_1538);
                        }
                        u_var2 = pass1_1018_31d0((i_var4 + 0xf2));
                        if (u_var2 != 0x0) {
                            u_var1 = (i_var4 + 0xf2);
                            u_var2 = pass1_1018_2dde(u_var1, (u_var1 >> 0x10));
                            //               TODO: goto LAB_1020_6a0b;
                        }
                    } else {
                        u_var2 = param_2 - 0xbba;
                        if (u_var2 == 0x0) {
                            u_var2 = GetDlgItem16(param_3, 0x1797);
                            if (u_var2 != 0x0) {
                                u_var2 = DestroyWindow16(ctx.s_tile2_bmp_1050_1538);
                                return u_var2;
                            }
                            //               TODO: goto LAB_1020_6a6f;
                        }
                    }
                }
            }
        }
    }
    return u_var2;
}

pub fn destroy_icon_1020_6bd2(ctx: &mut AppContext, param_1: u32, param_2: u8, param_3: HICON16) {
    let pu_var1: u32;
    let u_var2: u16;
    let ppc_var3: u32;
    let i_var4: i16;
    let u_var5: u16;
    let u_var6: u16;

    // u_var5 = (param_1 >> 0x10);
    i_var4 = param_1;
    u_var6 = (i_var4 + 0xc2);
    DestroyIcon16(param_3);
    (i_var4 + 0xc2) = 0x0;
    (i_var4 + 0x8) = 0x0;
    pu_var1 = (i_var4 + 0xf6);
    u_var2 = (i_var4 + 0xf8);
    if ((u_var2 | pu_var1) != 0x0) {
        ppc_var3 = *pu_var1;
        (**ppc_var3)(ctx.s_tile2_bmp_1050_1538, pu_var1, u_var2, 0x1, u_var6);
    }
    (i_var4 + 0xf6) = 0x0;
    pass1_1010_1dda((i_var4 + 0xf2));
    (i_var4 + 0xf2) = 0x0;
    return;
}

pub fn cleanup_menu_ui_op_1020_795c(in_struct_1: &mut Struct3, in_menu_handle_2: HMENU16) {
    let local_struct_1: &mut Struct3;
    let u_var1: &mut Struct3;

    // u_var1 = (in_struct_1 >> 0x10);
    local_struct_1 = in_struct_1;
    in_struct_1.address_offset_field_0x0 = 0x7b86;
    local_struct_1.address_offset_field_0x2 = 0x1020;
    if (local_struct_1.field_0xec != 0x0) {
        DestroyMenu16(in_menu_handle_2);
    }
    pass1_1008_57c4((in_struct_1 & 0xffff0000 | &local_struct_1.field_0xd2));
    in_struct_1.address_offset_field_0x0 = 0x380a;
    local_struct_1.address_offset_field_0x2 = 0x1008;
    in_struct_1.address_offset_field_0x0 = 0x389a;
    local_struct_1.address_offset_field_0x2 = 0x1008;
    return;
}

pub fn destroy_window_1020_8250(ctx: &mut AppContext, param_1: u32, param_2: HWND16) {
    let b_var1: bool;
    let u_var2: u16;

    // u_var2 = (param_1 >> 0x10);
    if ((param_1 + 0xec) != 0x0) {
        b_var1 = IsWindow16(param_2);
        if (b_var1 != 0x0) {
            DestroyWindow16(ctx.s_tile2_bmp_1050_1538);
            (param_1 + 0xec) = 0x0;
        }
    }
    return;
}

pub fn destroy_window_1038_7d88(param_1: i32, param_2: u16) {
    let in_DX: u16;

    pass1_1008_b544((param_1 + 0x94), param_2, in_DX, 0x1008);
    DestroyWindow16(0x1008);
    return;
}

pub fn destroy_window_1038_a072(param_1: u16, param_2: u16, param_3: i16, param_4: HWND16) {
    if (param_3 != 0x0) {
        DestroyWindow16(param_4);
    }
    return;
}

pub fn destroy_win_1038_a3d2(ctx: &mut AppContext, param_1: i32, param_2: HWND16) {
    GetWindowWord16(param_2, -0x8);
    PostMessage16(ctx.s_tile2_bmp_1050_1538, 0x0, 0x0, 0x1110105);
    destroy_win_1040_7b98(param_1, &ctx.PTR_LOOP_1050_1040);
    return;
}

pub fn destroy_window_1038_cc00(
    ctx: &mut AppContext,
    param_1: i16,
    param_2: u16,
    param_3: u16,
    param_4: i32,
) {
    let u_var1: u16;
    let in_DX: U32Ptr;
    let unaff_DI: i16;
    let unaff_SS: &mut WNDCLASS16;
    let i_var2: i16;

    u_var1 = param_4._2_2_ - 0x1cd;
    if (u_var1 == 0x0) {
        i_var2 = 0x1;
    } else {
        u_var1 = param_4._2_2_ - 0x1ce;
        if (u_var1 == 0x0) {
            i_var2 = 0x2;
        } else {
            u_var1 = param_4._2_2_ - 0x1cf;
            if (u_var1 == 0x0) {
                i_var2 = 0x3;
            } else {
                u_var1 = param_4._2_2_ - 0x1d0;
                if (u_var1 == 0x0) {
                    i_var2 = 0x4;
                } else {
                    u_var1 = param_4._2_2_ - 0x1d1;
                    if (u_var1 != 0x0) {
                        post_win_msg_1040_7b3c(
                            CONCAT22(param_2, param_1),
                            param_3,
                            param_4,
                            param_4._2_2_,
                            &ctx.PTR_LOOP_1050_1040,
                        );
                        return;
                    }
                    i_var2 = 0x5;
                }
            }
        }
    }
    pass1_1008_eb74((param_1 + 0x8e), i_var2, in_DX, unaff_DI, unaff_SS);
    if (u_var1 != 0x0) {
        win_1008_5c7c(
            ctx.PTR_LOOP_1050_02a0,
            CONCAT22(u_var1, 0x1),
            unaff_SS,
            u_var1,
            in_DX,
        );
        DestroyWindow16(0x1008);
    }
    return;
}

pub fn destroy_window_1038_cd88(ctx: &mut AppContext, param_1: &mut Struct1) {
    let unaff_SS: u16;

    dialog_ui_fn_1040_78e2(param_1, &ctx.PTR_LOOP_1050_1040);
    move_win_1040_826c(param_1, -0x1);
    ShowWindow16(&ctx.PTR_LOOP_1050_1040, 0x5);
    (param_1 + 0x92) = 0x1;
    unk_win_msg_op_1008_9510((param_1 & 0xffff0000 | (param_1 + 0x92)), 0x1008, unaff_SS);
    DestroyWindow16(0x1008);
    return;
}

pub fn destroy_win_1038_e1dc(
    ctx: &mut AppContext,
    aram_1: u16,
    param_2: u16,
    param_3: i16,
    param_4: &mut HWND16,
) {
    let uvar1: u16;
    let lparam: LPARAM;

    if param_3 != 0x0 {
        uvar1 = IsDlgButtonChecked(*param_4, 0x1807);
        if uvar1 == 0x0 {
            *param_4 = ctx.s_tile2_bmp_1050_1538;
            uvar1 = IsDlgButtonChecked(ctx.s_tile2_bmp_1050_1538, 0x1806);
            if uvar1 == 0x0 {
                // TODO: goto LAB_1038_e229;
            }
            lparam = 0x1110130;
        } else {
            lparam = 0x111012f;
        }
        *param_4 = ctx.s_tile2_bmp_1050_1538;
        SendMessage16(ctx.s_tile2_bmp_1050_1538, 0x0, 0x0, lparam);
    }
    //LAB_1038_e229:
    DestroyWindow16(*param_4);
    return;
}

pub fn destroy_win_1038_ef3a(ctx: &mut AppContext, param_1: &mut Struct31, param_2: HWND16) {
    let i_var1: &mut Struct31;
    let u_var1: &mut Struct31;

    // u_var1 = (param_1 >> 0x10);
    i_var1 = param_1;
    *param_1.field_0x0 = 0x67c;
    i_var1.field_0x2 = &ctx.PTR_LOOP_1050_1040;
    if i_var1.field_0x96 != 0x0 {
        DestroyWindow16(param_2);
        i_var1.field_0x96 = 0x0;
    }
    pass1_1038_b6e0(ctx.PTR_LOOP_1050_5b7c, i_var1.field_0x6);
    ui_cleanup_op_1040_782c(ctx, param_1, ctx.PTR_LOOP_1050_1040);
    return;
}

pub fn destroy_win_1040_5256(
    ctx: &mut AppContext,
    param_1: &mut Struct34,
    window_handle_2: HWND16,
) {
    let pUVar1: &mut u32;
    let u_var2: u16;
    let func_ptr_1: u32;
    // let Bvar4: bool;
    let iVar5: &mut Struct34;
    let u_var5: u16;
    let window_handle_1: HWND16;

    // u_var5 = (param_1 >> 0x10);
    iVar5 = param_1;
    window_handle_1 = window_handle_2;
    if iVar5.field_0xb6 != 0x0 {
        window_handle_1 = ctx.s_tile2_bmp_1050_1538;
        let is_window = IsWindow16(window_handle_2);
        if is_window {
            window_handle_1 = ctx.s_tile2_bmp_1050_1538;
            DestroyWindow16(ctx.s_tile2_bmp_1050_1538);
        }
    }
    iVar5.field_0xb6 = 0x0;
    pUVar1 = iVar5.field_0x94;
    u_var2 = iVar5.field_0x96;
    if ((u_var2 | pUVar1) != 0x0) {
        func_ptr_1 = *pUVar1;
        (**func_ptr_1)(window_handle_1, pUVar1, u_var2, 0x1);
    }
    iVar5.field_0x94 = 0x0;
    iVar5.field_0x98 = 0x0;
    return;
}

pub fn destroy_win_1040_7b98(param_1: i32, param_2: HWND16) {
    if (param_1 + 0x74) == 0x0 {
        DestroyWindow16(param_2);
    }
    return;
}

pub fn destroy_win_1040_8212(ctx: &mut AppContext, param_1: i32, param_2: HWND16) {
    let is_window: bool;
    let u_var1: u16;

    // u_var1 = (param_1 >> 0x10);
    if ((param_1 + 0x8c) != 0x0) {
        is_window = IsWindow16(param_2);
        if (is_window != 0x0) {
            DestroyWindow16(ctx.s_tile2_bmp_1050_1538);
            (param_1 + 0x8c) = 0x0;
        }
    }
    return;
}

pub fn destroy_win_1040_8b7e(param_1: HWND16) {
    DestroyWindow16(param_1);
    return;
}

pub fn destroy_window_1040_b726(param_1: U32Ptr, param_2: i16, in_win_handle_3: HWND16) {
    let ppc_var1: u32;

    if param_2 != 0x0 {
        ppc_var1 = (*param_1 + 0x78);
        (**ppc_var1)(in_win_handle_3, param_1);
    }
    DestroyWindow16(in_win_handle_3);
    return;
}

pub fn destroy_win_1040_bb78(ctx: &mut AppContext, param_1: &mut Struct35, param_2: HWND16) {
    let pu_var1: u32;
    let u_var2: u16;
    let ppc_var3: u32;
    let is_window: bool;
    let iVar5: &mut Struct35;
    let u_var5: u16;
    let HVar6: HWND16;

    // u_var5 = (param_1 >> 0x10);
    iVar5 = param_1;
    HVar6 = param_2;
    if (iVar5.field_0xb6 != 0x0) {
        HVar6 = ctx.s_tile2_bmp_1050_1538;
        is_window = IsWindow16(param_2);
        if (is_window != false) {
            HVar6 = ctx.s_tile2_bmp_1050_1538;
            DestroyWindow16(ctx.s_tile2_bmp_1050_1538);
        }
    }
    iVar5.field_0xb6 = 0x0;
    pu_var1 = iVar5.field_0x94;
    u_var2 = iVar5.field_0x96;
    if (u_var2 | pu_var1) != 0x0 {
        ppc_var3 = *pu_var1;
        (**ppc_var3)(HVar6, pu_var1, u_var2, 0x1);
    }
    iVar5.field_0x94 = 0x0;
    iVar5.field_0x98 = 0x0;
    return;
}

pub fn win_cleanup_op_1040_748c(
    ctx: &mut AppContext,
    param_1: i16,
    param_2: u16,
    param_3: u16,
    param_4: u32,
    in_dx: u16,
    mut unaff_cs: u16,
    unaff_ss: u16,
) {
    let func_ptr_1: u32;
    let u_var2: u32;
    let i_var3: i16;
    let mut rect_1: RECT16;
    let i_stack6: i16;
    let i_stack4: i16;

    if false {
        // switchD_1040_75ed_caseD_fb:
        pass1_1040_b54a(
            param_1, param_2, param_3, param_4, in_dx, unaff_cs, unaff_ss,
        );
        return;
    }
    unaff_cs = ctx.PTR_LOOP_1050_1040 as u16;
    match param_4 {
        0xfa => {
            func_ptr_1 = ((param_1 + 0x94) + 0x18);
            (**func_ptr_1)();
        }
        //     TODO: goto switchD_1040_75ed_caseD_fb;
        0xfd => {
            if ctx.DAT_1050_0ecc == 0x0 {
                return;
            }
            ctx.DAT_1050_0ecc = 0x0;
        }
        //     TODO: goto LAB_1040_755d;
        0xfe => {
            if ctx.DAT_1050_0ecc == 0x1 {
                return;
            }
            ctx.DAT_1050_0ecc = 0x1;
        }
        //     TODO: goto LAB_1040_755d;
        0xff => {
            if ctx.DAT_1050_0ecc == 0x2 {
                return;
            }
            ctx.DAT_1050_0ecc = 0x2;
            //LAB_1040_755d:
            u_var2 = (param_1 + 0x94);
            func_ptr_1 = ((param_1 + 0x94) + 0x10);
            (**func_ptr_1)(&ctx.PTR_LOOP_1050_1040, u_var2, (u_var2 >> 0x10));
            pass1_1010_2ee2((param_1 + 0x94), unaff_ss, 0x1010);
            PostMessage16(0x1010, 0x0, 0x0, 0x111010a);
        }
        0x107 => {
            i_var3 = 0x0;
        }
        //     TODO: goto LAB_1040_75ba;
        0x108 => {
            i_var3 = 0x1;
            //LAB_1040_75ba:
            win_ui_op_1010_3202((param_1 + 0x94), i_var3, 0x1010);
        }
        0x10a => {
            GetClientRect16(&ctx.PTR_LOOP_1050_1040, &rect_1);
            u_var2 = (param_1 + 0x94);
            rect_1.y += 0x3;
            rect_1.x = (u_var2 + 0x1a) + -0x9;
            i_stack6 += -0x3;
            i_stack4 += -0x3;
            InvalidateRect16(
                ctx.s_tile2_bmp_1050_1538,
                (&ctx.PTR_LOOP_1050_0000 + 0x1),
                &rect_1,
            );
            unk_destroy_win_op_1010_2fa0(ctx, (param_1 + 0x94), 0x1010);
            pass1_1010_32c0((param_1 + 0x94), 0x0);
            pass1_1010_2ee2((param_1 + 0x94), unaff_ss, 0x1010);
        }
        0x10c => {
            DestroyWindow16(ctx.PTR_LOOP_1050_1040 as HWND16);
        }
        _ => {}
    }
    return;
}

pub fn ui_cleanup_op_1040_782c(ctx: &mut AppContext, struct_1: &mut Struct18, handle_1: HGDIOBJ16) {
    let pu_var1: u32;
    let u_var2: u16;
    let func_ptr_1: u32;
    let struct_2: &mut Struct18;
    let u_var5: u16;
    let menu: HGDIOBJ16;
    let hwnd: HMENU16;

    // u_var5 = (param_1 >> 0x10);
    struct_2 = struct_1;
    struct_1.field_0x0 = 0x840c;
    (struct_2.field_0x2) = ctx.PTR_LOOP_1050_1040;
    pu_var1 = (struct_2.field_0x70);
    u_var2 = (struct_2.field_0x72);
    if (u_var2 | pu_var1) != 0x0 {
        func_ptr_1 = *pu_var1;
        (**func_ptr_1)(handle_1, pu_var1, u_var2, 0x1);
    }
    menu = handle_1;
    if (struct_2.field_0x4) != 0x0 {
        menu = ctx.s_tile2_bmp_1050_1538 as HMENU16;
        DeleteObject16(handle_1);
        (struct_2.field_0x4) = 0x0;
    }
    hwnd = menu;
    if (struct_2.field_0x68) != 0x0 {
        hwnd = ctx.s_tile2_bmp_1050_1538 as HWND16;
        DestroyMenu16(menu);
    }
    RemoveProp16(hwnd, &ctx.s_thisLo_1050_5db1);
    RemoveProp16(ctx.s_tile2_bmp_1050_1538 as HWND16, &ctx.s_thisHi_1050_5db8);
    RemoveProp16(ctx.s_tile2_bmp_1050_1538 as HWND16, &ctx.s_procLo_1050_5dbf);
    RemoveProp16(ctx.s_tile2_bmp_1050_1538 as HWND16, &ctx.s_procHi_1050_5dc6);
    struct_1.field_0x0 = 0x389a;
    (struct_2 + 0x2) = 0x1008;
    return;
}
