use crate::cleanup::clenaup_win_ui_1018_4d22;
use crate::debug::debug_print_1008_6048;
use crate::defines::{Struct11, Struct13, Struct18, Struct76, Struct79, Struct_1008_09ba, Struct_1008_0a3c, StructB, U32Ptr};
use crate::draw::draw_1008::draw_op_1008_8288;
use crate::file::file_1008::{close_file_1008_6dd0, file_fn_1008_6e02};
use crate::fn_ptr::fn_ptr_1000::{fn_ptr_1000_17ce, fn_ptr_op_1000_24cd};
use crate::global::AppContext;
use crate::mci::mci_send_command_1008_5cb6;
use crate::mem_1000::mem_op_1000_179c;
use crate::mixed::mixed_1010_20ba;
use crate::pass::pass_1000::{pass1_1000_093a, pass1_1000_4906, pass1_1000_4f2e, pass1_1000_5008};
use crate::pass::pass_1008::{pass1_1008_4016, pass1_1008_47cc, pass1_1008_4834, pass1_1008_5784, pass1_1008_5b12, pass1_1008_a086};
use crate::pass::pass_1010::{pass1_1010_1d80, pass1_1010_1f62, pass1_1010_60cc};
use crate::pass::pass_1030::{pass1_1030_8326, pass1_1030_8344};
use crate::pass::pass_1038::pass1_1038_3608;
use crate::string::string_1000::{string_1000_3d3e, str_op_1000_3da4, string_1000_3cea, string_1000_475e};
use crate::string::string_1008::{str_1008_6d8a, str_op_1008_60e8};
use crate::string::string_1010::load_string_1010_847e;
use crate::struct_ops::struct_1010::struct_1010_5f1e;
use crate::struct_ops::struct_1018::set_struct_1018_262e;
use crate::ui::ui_1010::show_window_1010_7ace;
use crate::ui::ui_1018::send_msg_1020_097e;
use crate::ui::ui_1038::{send_msg_1030_83ba, show_win_1038_b68a};
use crate::util::{address_of, CONCAT12, CONCAT13, CONCAT22, get_struct_ref_from_addr, read_string_from_rsrc, read_struct_from_addr, SUB42, ZEXT24};
use crate::win_struct::{ATOM, BITMAPINFO, COLORREF, DEVMODEA, HCURSOR16, HDC16, HGDIOBJ16, HINSTANCE16, HMENU16, HPALETTE16, HWND16, LOGPALETTE, PAINTSTRUCT16, POINT16, RECT16, WNDCLASS16};
use crate::winapi::{BeginPaint16, ClientToScreen16, CreatePalette16, CreateSolidBrush16, CreateWindow16, CreateWIndowEx16, DefWindowProc16, DeleteObject16, EndPaint16, FillRect16, GetClassInfo16, GetClientRect16, GetOpenFileName16, GetSaveFileName16, GetStockObject16, GetSubMenu16, GetSysColor16, GetWindowLong16, InvalidateRect16, IsIconic16, KillTimer16, LoadCursor16, LoadMenu16, mciSendCommand16, MessageBeep16, MessageBox16, OutputDebugString16, PostMessage16, PostQuitMessage16, PtInRect16, RealizePalette16, RegisterClass16, ReleaseCapture16, SelectPalette16, SendMessage16, SetCapture16, SetCursor16, SetDIBitsToDevice, SetSysColors16, SetTimer16, SetWindowText16, SetWindowWord16, ShowWindow16, StretchDIBits16, TrackPopupMenu16, UpdateWindow16};

pub unsafe fn win_ui_cursor_op_1008_06c0(
    ctx: &mut AppContext,
    param_1: &mut u32,
    param_2: &mut u32,
    param_3: u16,
    param_4: i16,
    in_ax: u16,
    in_dx: u16,
    extraout_dx: u16,
    unaff_di: u16,
    unaff_ss: u16,
    unaff_af: u8,
) {
    let ppc_var1: u32;
    // let in_ax: u16;
    // let in_dx: u16;
    let mut struct_1: &mut Struct79;
    // let extraout_dx: U32Ptr;
    // let unaff_di: i16;
    let u_var3: u16;
    // let unaff_SS: U32Ptr;
    // let in_AF: u8;
    let mut pc_var4: String;
    let pu_var5: U32Ptr;
    let mut ulocal_5a = String::new();
    let u_stack10: u32;
    let hstack6: HCURSOR16;
    let hstack4: HCURSOR16;

    if param_4 == 0x400 {
        pass1_1030_8344(
            ctx, ctx.PTR_LOOP_1050_5748 as u16 as u32, 0x4000001);
        struct_1 = read_struct_from_addr((in_dx | in_ax) as u32) ;
        if struct_1 != 0x0 {
            if ctx.PTR_LOOP_1050_4fe8 != 0x0 {
                pc_var4 = load_string_1010_847e(ctx.PTR_LOOP_1050_14cc as i16,
                                                ((ctx.PTR_LOOP_1050_14cc >> 0x10) as i16), 0x1010);
                MessageBox16(0x1010, &ctx.PTR_LOOP_1050_0010, &pc_var4,
                             (pc_var4 >> 0x10));
                return;
            }
            hstack4 = LoadCursor16(0x1030, 0x7f02);
            hstack6 = SetCursor16(ctx.s_tile2_bmp_1050_1538 as u16);
            send_msg_1030_83ba(ctx, &mut ctx.PTR_LOOP_1050_5748, param_2, unaff_ss, in_AF);
            u_var3 = (ctx.PTR_LOOP_1050_5748 >> 0x10) as u16;
            (ctx.PTR_LOOP_1050_5748 + 0x8) = 0x1;
            u_stack10 = mixed_1010_20ba(ctx, ctx.PTR_LOOP_1050_0ed0, 0x29, unaff_ss, struct_1, unaff_di as i16, 0);
            set_struct_1018_262e(u_stack10);
            pass1_1030_8326(ctx);
            pc_var4 = load_string_1010_847e(ctx.PTR_LOOP_1050_14cc as i16,
                                            ((ctx.PTR_LOOP_1050_14cc >> 0x10) as i16), 0x1010);
            sys_1000_3f9c(local_5a, unaff_SS, 0x109, ctx.data_seg, pc_var4,
                          &stack0xfffe, u_var3, 0x1000, unaff_SS, in_AF);
            ppc_var1 = (*param_1 + 0x14);
            (**ppc_var1)(0x1000, param_1, (param_1 >> 0x10), 0x0, local_5a);
            pu_var5 = mixed_1010_20ba(ctx, ctx.PTR_LOOP_1050_0ed0, 0x37, unaff_SS, extraout_dx, unaff_di, 0);
            pass1_1008_a9ec(pu_var5);
            SetCursor16(0x1010);
            PostMessage16(ctx.s_tile2_bmp_1050_1538 as HWND16, 0x0, 0x0, 0x11100fc);
        }
    }
    return;
}


pub fn menu_ui_op_1008_09ba(
    ctx: &mut AppContext,
    param_1: &mut Struct_1008_09ba,
    y_coord: i16,
    x_coord: i16,
    mut win_handle_1: &mut HWND16) {
    let menu_handle_1: HMENU16;
    let i_var2: i16;
    let mut point_1: &mut POINT16;

    if (param_1.field_0xec) == 0x0 {
        menu_handle_1 = LoadMenu16(*win_handle_1, ctx.s_OPPOPMENU_1050_0150);
        (param_1.field_0xec) = menu_handle_1;
        if menu_handle_1 == 0x0 {
            return;
        }
        *win_handle_1 = ctx.s_tile2_bmp_1050_1538 as HWND16;
        menu_handle_1 = GetSubMenu16(ctx.s_tile2_bmp_1050_1538 as HMENU16, 0x0);
        (param_1.field_0xec) = menu_handle_1;
        if menu_handle_1 == 0x0 {
            return;
        }
    }
    local_6.x = x_coord;
    local_6.y = y_coord;
    ClientToScreen16(*win_handle_1, &local_6);
    TrackPopupMenu16(ctx.s_tile2_bmp_1050_1538 as HMENU16, 0x0, local_6.x, local_6.y,  0x0,ctx.PTR_LOOP_1050_0396, &local_6);
    return;
}


pub fn unk_win_msg_op_1008_0a3c(
    ctx: &mut AppContext,
    param_1: &mut Struct_1008_0a3c,
    param_2: u16,
    win_handle: HWND16) -> bool
{
    let bvar1: bool;

    if (param_2 & 0xfff0) == 0xf140 {
        return param_1.field_0xde;
    }
    if (param_2 & 0xfff0) == 0xf060 {
        bvar1 = IsIconic16(win_handle);
        if bvar1 == false {
            PostMessage16(ctx.s_tile2_bmp_1050_1538 as HWND16, 0x0, 0x0, 0x1110067);
        }
        return false;
    }
    return true;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn message_box_op_1008_12dc(
    ctx: &mut AppContext,
    param_1: u32,
    param_2: &mut String,
    param_3: HINSTANCE16,
    param_4: &mut WNDCLASS16,
    in_af: u8,
    in_dx: u16) {
    let bvar1: bool;
    let u_var2: u16;
    // let in_dx: u16;
    let u_var3: u16;
    // let in_af: u8;
    let mut pc_var4: String;
    let u_stack36: u32;
    let u_stack16: u32;
    let local_c: [u8; 6];
    let cursor_handle_1: HCURSOR16;
    let curosr_handle_2: HCURSOR16;

    curosr_handle_2 = LoadCursor16(param_3, &read_string_from_rsrc(0x7f02));
    cursor_handle_1 = SetCursor16(ctx.s_tile2_bmp_1050_1538 as HCURSOR16);
    str_1008_6d8a(CONCAT22(param_4, local_c), param_2, in_dx, param_4, in_af);
    bvar1 = file_fn_1008_6e02(CONCAT22(param_4, local_c),
                              ctx.s_tile2_bmp_1050_1538, param_4);
    if (bvar1 == 0x0) {
        SetCursor16(ctx.s_tile2_bmp_1050_1538);
        pc_var4 = load_string_1010_847e(ctx.PTR_LOOP_1050_14cc,
                                        (ctx.PTR_LOOP_1050_14cc >> 0x10), 0x1010);
        // u_var3 = (pc_var4 >> 0x10);
        u_var2 = str_op_1008_60e8(pc_var4, u_var3);
        u_stack16 = CONCAT22(u_var3, u_var2);
        pc_var4 = load_string_1010_847e(ctx.PTR_LOOP_1050_14cc,
                                        (ctx.PTR_LOOP_1050_14cc >> 0x10), 0x1010);
        MessageBeep16(0x1010);
        MessageBox16(ctx.s_tile2_bmp_1050_1538, &ctx.PTR_LOOP_1050_0010, pc_var4,
                     (pc_var4 >> 0x10));
    } else {
        (ctx.PTR_LOOP_1050_5748 + 0x8) = 0x0;
        SetCursor16(ctx.s_tile2_bmp_1050_1538);
        pc_var4 = load_string_1010_847e(ctx.PTR_LOOP_1050_14cc,
                                        (ctx.PTR_LOOP_1050_14cc >> 0x10), 0x1010);
        // u_var3 = (pc_var4 >> 0x10);
        u_var2 = str_op_1008_60e8(pc_var4, u_var3);
        u_stack36 = CONCAT22(u_var3, u_var2);
        pc_var4 = load_string_1010_847e(ctx.PTR_LOOP_1050_14cc,
                                        (ctx.PTR_LOOP_1050_14cc >> 0x10), 0x1010);
        MessageBeep16(0x1010);
        MessageBox16(ctx.s_tile2_bmp_1050_1538, 0x40, pc_var4,
                     (pc_var4 >> 0x10));
        u_stack16 = u_stack36;
    }
    fn_ptr_1000_17ce(ctx, (u_stack16 & 0xffff | u_var3 << 0x10), 0x1000);
    close_file_1008_6dd0(CONCAT22(param_4, local_c), 0x1000);
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn win_ui_op_1008_1414(astruct_72 * * param_1, param_2: u32, param_3: &String, param_4: u16,
                           param_5: u8, param_6: u16)

{
    let ppcVar1: u32;
    let BVar2: bool;
    let u_var3: u16;
    let i_var4: i16;
    let pu_var5: u32;
    let u_var5: u32;
    let puVar6: U32Ptr;
    let uVar7: u16;
    let type : * mut u8;
    let uVar8: u16;
    let extraout_dx: u16;
    let unaff_DI: i16;
    let uVar9: u16;
    let puVar10: u32;
    let mut pcVar11: String;
    let puVar12: U32Ptr;
    let uVar13: u8;
    let uVar14: u8;
    let iVar15: i16;
    let local_2a: u32;
    let uStack38: u16;
    let iStack36: i16;
    let puStack34: U32Ptr;
    let uStack32: u32;
    let uStack28: u32;
    let uStack24: u32;
    let uStack20: u32;
    let uStack16: u32;
    let puStack12: U32Ptr;
    let local_8: [u8; 6];
    let u_var10: u16;

    puVar10 = str_1008_6d8a(CONCAT22(param_4, local_8), param_2, param_6,
                            param_4, param_5 as u16);
    // puVar6 = (puVar10 >> 0x10);
    BVar2 = read_file_1008_6e78((uint32_t)local_8, param_4, param_3, param_4);
    iVar15 = param_1;
    // uVar9 = (param_1 >> 0x10);
    if (BVar2 == 0x0) {
        if (ctx.PTR_LOOP_1050_0310 == 0x0) {
            ctx.PTR_LOOP_1050_0310 = 0x6d4;
        }
        pcVar11 = load_string_1010_847e(ctx.PTR_LOOP_1050_14cc,
                                        (ctx.PTR_LOOP_1050_14cc >> 0x10), 0x1010);
        // uVar7 = (pcVar11 >> 0x10);
        u_var3 = str_op_1008_60e8(pcVar11, uVar7);
        pcVar11 = load_string_1010_847e(ctx.PTR_LOOP_1050_14cc,
                                        (ctx.PTR_LOOP_1050_14cc >> 0x10), 0x1010);
        // type = (pcVar11 >> 0x10);
        puVar6 = type ;
        MessageBeep16(0x1010);
        MessageBox16(ctx.s_tile2_bmp_1050_1538, &ctx.PTR_LOOP_1050_0010, pcVar11, type );
        fn_ptr_1000_17ce(ctx, CONCAT22(uVar7, u_var3), 0x1000);
        param_3 = &ctx.PTR_LOOP_1050_1000;
        fn_ptr_op_1000_24cd(0x1, &stack0xfffe);
    }
    cursor_op_1008_2dcc(iVar15, uVar9, 0x8, param_3);
    puStack12 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x2f, param_4, puVar6, unaff_DI);
    // uVar8 = (puStack12 >> 0x10);
    u_var5 = (puStack12 + 0x20);
    uStack16 = u_var5;
    pass1_1030_8344(
        ctx, ctx.PTR_LOOP_1050_5748, u_var5);
    uStack20 = u_var5 & 0xffff | uVar8 << 0x10;
    uStack24 = (u_var5 + 0x10);
    i_var4 = ((uStack24 + 0x2) + -0x1) as i16;
    ppcVar1 = ((iVar15 + 0xe8) + 0x4) as u32;
    (**ppcVar1)(0x1030, (iVar15 + 0xe8), uStack16, (uStack16 >> 0x10),
                i_var4, 0x2);
    puVar6 = extraout_dx;
    pass1_1030_8344(
        ctx, ctx.PTR_LOOP_1050_5748, 0x4000001);
    uStack28 = CONCAT22(puVar6, i_var4 as u16);
    u_var5 = (i_var4 + 0x10) as u32;
    uStack32 = u_var5;
    pass1_1030_8344(
        ctx, ctx.PTR_LOOP_1050_5748, u_var5);
    iStack36 = u_var5 as i16;
    local_2a = (iStack36 + 0xc) as u32;
    uStack38 = (iStack36 + 0x10) as u16;
    puStack34 = puVar6;
    pu_var5 = pass1_1030_5b00(uStack20);
    uVar13 = SUB21(&local_2a, 0x0);
    uVar14 = (&local_2a >> 0x8);
    u_var3 = param_4;
    puVar12 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, pu_var5, param_4, puVar6,
                              &iStack36);
    // puVar6 = (puVar12 >> 0x10);
    pass1_1018_179e(puVar12, CONCAT22(u_var3, CONCAT11(uVar14, uVar13)), 0x1018, param_4);
    uVar13 = 0x0;
    uVar14 = 0x4;
    iVar15 = 0x1b;
    u_var10 = 0x1;
    puVar12 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x2b, param_4, puVar6, &iStack36);
    pass1_1010_043a(puVar12, CONCAT13(uVar14 as u16, CONCAT12(uVar13, u_var10)), iVar15, param_4);
    close_file_1008_6dd0(CONCAT22(param_4, local_8), 0x1010);
    return;
}


pub fn post_msg_1008_2d22(param_1: u32) {
    let pi_var1: U32Ptr;
    let u_var2: u32;
    let ppc_var3: u32;
    let i_var4: i16;
    let u_var5: u16;
    let unaff_SS: u16;
    let u_var6: u32;
    let uVar7: u16;
    let uVar8: u32;

    // u_var5 = (param_1 >> 0x10);
    i_var4 = param_1 as i16;
    if (((i_var4 + 0xee) != 0x0) && (pi_var1 = (i_var4 + 0xf2), *pi_var1 = *pi_var1 + -0x1,
                                    (i_var4 + 0xf2) < 0x1)) {
        uVar8 = (i_var4 + 0xee) as u32;
        ppc_var3 = ((i_var4 + 0xee) + 0x90) as u32;
        (**ppc_var3)();
        (i_var4 + 0xee) = 0x0;
        (i_var4 + 0xf2) = 0x0;
        if ((i_var4 + 0xe8) != 0x0) {
            uVar7 = 0x3;
            u_var6 = (i_var4 + 0xe8) as u32;
            ppc_var3 = ((i_var4 + 0xe8) + 0xc) as u32;
            (**ppc_var3)();
            show_win_1038_b68a(ctx.PTR_LOOP_1050_5b7c, &ctx.PTR_LOOP_1050_1038);
            show_window_1010_7ace(((i_var4 + 0xf4) as u32), unaff_SS);
            u_var2 = (i_var4 + 0xe8) as u32;
            ppc_var3 = ((i_var4 + 0xe8) + 0x98) as u32;
            (**ppc_var3)(0x1010, u_var2, (u_var2 >> 0x10), 0x1, u_var6, uVar7, uVar8);
            PostMessage16(0x1010, 0x0, 0x0, 0x11100fc);
        }
    }
    return;
}


pub fn cursor_op_1008_2dcc(param_1: i16, param_2: u16, param_3: u16, in_hinstance: HINSTANCE16,
)

{
    let u_var1: u32;
    let ppcVar2: u32;
    let cursor_handle: HCURSOR16;
    let in_DX: u16;
    let extraout_dx: u16;
    let i_var3: i16;
    let unaff_SS: u16;
    let u_var4: u32;

    u_var4 = 0x0;
    cursor_handle = LoadCursor16(in_hinstance, 0x7f02);
    u_var4 = u_var4 & 0xffff0000 | cursor_handle;
    cursor_handle = SetCursor16(ctx.s_tile2_bmp_1050_1538);
    i_var3 = param_1;
    if ((param_1 + 0xe8) != 0x0) {
        u_var1 = (param_1 + 0xe8) as u32;
        i_var3 = (param_1 + 0xe8);
        ppcVar2 = (i_var3 + 0x90) as u32;
        (**ppcVar2)(ctx.s_tile2_bmp_1050_1538, u_var1, (u_var1 >> 0x10), u_var4);
        in_DX = extraout_dx;
    }
    big_switch_1008_15d4(i_var3, s_tile2_bmp_1050_1538, unaff_SS, CONCAT22(param_2, param_1 as u16),
                         param_3);
    (param_1 + 0xe8) = cursor_handle as i16;
    (param_1 + 0xea) = in_DX as i16;
    u_var1 = (param_1 + 0xe8) as u32;
    if ((u_var1 + 0xe0) == 0x0) {
        u_var1 = (param_1 + 0xe8) as u32;
        ppcVar2 = ((param_1 + 0xe8) + 0x8) as u32;
        (**ppcVar2)(ctx.s_tile2_bmp_1050_1538, u_var1, (u_var1 >> 0x10));
        ppcVar2 = ((param_1 + 0xe8) + 0xc) as u32;
        (**ppcVar2)(ctx.s_tile2_bmp_1050_1538, (param_1 + 0xe8), 0x3);
        (param_1 + 0xce) = (param_1 + 0xe8);
    } else {
        (param_1 + 0xe8) = 0x0;
        ui_op_1008_2c4e(param_1, param_2, param_3, s_tile2_bmp_1050_1538);
        (param_1 + 0xce) = 0x0;
    }
    SetCursor16(ctx.s_tile2_bmp_1050_1538);
    return;
}


// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn win_ui_cursor_op_1008_2e9a(astruct_72 * * param_1, param_2: u16) {
    let u_var1: u16;
    let i_var2: i16;
    let in_DX: U32Ptr;
    let u_var3: u16;
    let u_var4: u16;
    let unaff_DI: i16;
    let in_AF: u8;
    local_22e: u8[0xa];
    let local_224: [u8; 108];
    let uStack284: u16;
    let mut pcStack282: String;
    let HStack274: HCURSOR16;
    let HStack272: HCURSOR16;
    let uStack270: u32;
    let
    UStack266: i32;
    let uStack262: u32;
    local_102: u8[0x100];

    local_102[0] = '\0';
    uStack262 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x2, param_2, in_DX, unaff_DI);
    // u_var1 = (uStack262 >> 0x10);
    i_var2 = uStack262 as i16;
    UStack266 = (i_var2 + 0x16);
    u_var3 = (i_var2 + 0x18) as u16;
    UStack266._0_2_ = u_var3 | UStack266;
    if (UStack266 == 0x0) {
        save_file_1008_3178(param_1, 0x1, param_2);
        UStack266 = CONCAT22(u_var3, UStack266);
        u_var4 = u_var3 | UStack266;
        if (u_var4 == 0x0) {
            PostMessage16(0x1010, 0x0, 0x0, 0x111013d);
            return;
        }
        string_1000_3d3e(CONCAT22(param_2, local_102), CONCAT22(u_var3, UStack266),
        );
        str_1000_4d58(CONCAT22(param_2, local_102), 0x0, 0x0,
                      CONCAT22(param_2, local_224), CONCAT22(param_2, local_22e));
        u_var3 = u_var4;
        if (local_22e[0] != '\0') {
            string_1000_3cea(CONCAT22(param_2, local_224), CONCAT22(param_2, local_22e) as i32);
            u_var3 = u_var4;
        }
        struct_1010_5f1e(uStack262, CONCAT22(param_2, local_224), u_var3);
    } else {
        pcStack282 = (i_var2 + 0x1a);
        string_1000_3d3e(CONCAT22(param_2, local_102), pcStack282);
        uStack284 = str_op_1000_3da4(CONCAT22(param_2, local_102));
        if (local_102[uStack284 - 0x1] != '\\') {
            local_102[uStack284] = '\\';
            local_102[uStack284 + 0x1] = '\0';
        }
        string_1000_3cea(CONCAT22(param_2, local_102) as i32, UStack266);
    }
    if (local_102[0] != '\0') {
        uStack270 = (param_1 + 0xe8);
        send_msg_1020_097e(uStack270, 0x1020);
        UpdateWindow16(0x1020);
        HStack272 = LoadCursor16(s_tile2_bmp_1050_1538, 0x7f02);
        HStack274 = SetCursor16(ctx.s_tile2_bmp_1050_1538);
        win_ui_op_1008_1414(param_1, CONCAT22(param_2, local_102), s_tile2_bmp_1050_1538,
                            param_2, in_AF, u_var3);
        SetCursor16(ctx.s_tile2_bmp_1050_1538);
    }
    return;
}


pub fn save_file_1008_3178(param_1: u32, param_2: i16, param_3: &mut WNDCLASS16) {
    let u_var1: u32;
    let i_var2: i16;
    let pu_var3: U32Ptr;
    let u_var4: u16;
    let BVar5: bool;
    let in_DX: U32Ptr;
    let extraout_dx: u16;
    let u_var6: u16;
    let unaff_DI: i16;
    let uVar7: u16;
    let mut pcVar8: String;
    let mut pcVar9: String;
    let in_buf_len_2: i16;
    let u_var10: u16;
    local_782: u8[0x104];
    let local_67e: [u16; 0x4];
    let paStack1654: &mut Struct18;
    let mut pCStack1650: String;
    let UStack1648: u16;
    let paStack1646: &mut Struct18;
    let local_666: [u8; 100];
    let mut pcStack1382: String;
    let local_562: u32;
    let uStack1374: u16;
    let mut pcStack1370: String;
    let uStack1350: u32;
    let puStack1346: U32Ptr;
    let uStack1342: u32;
    let mut pcStack1338: String;
    let puStack1334: U32Ptr;
    let uStack1330: u32;
    let uStack1326: u16;
    let mut pcStack1322: String;
    let cStack1306: u8;
    acStack1305: u8[0x101];
    let uStack1048: u16;
    local_416: u8[0x8];
    let uStack1038: u16;
    let local_40c: [u8; 102];
    let uStack778: u32;
    let puStack774: U32Ptr;
    let local_302: [u8; 100];
    u8
    local_202[0xff];
    acStack259: u8[0x101];

    acStack259[1] = 0x0;
    local_302[0] = 0x0;
    local_202[0] = 0x0;
    puStack774 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x2, param_3, in_DX, unaff_DI);
    // uVar7 = (puStack774 >> 0x10);
    i_var2 = puStack774;
    uStack778 = (i_var2 + 0x1a) as u32;
    u_var10 = (i_var2 + 0x1c) as u16;
    if ((u_var10 | uStack778) == 0x0) {
        paStack1646 = (i_var2 + 0x64);
        u_var10 = (i_var2 + 0x66) as u16;
        if ((u_var10 | paStack1646) != 0x0) {
            pass1_1008_5784(CONCAT22(param_3, local_67e),
                            paStack1646 & 0xffff | u_var10 << 0x10);
            pu_var3 = local_67e;
            pass1_1008_5b12(pu_var3, param_3);
            paStack1654 = CONCAT22(extraout_dx, pu_var3);
            if ((extraout_dx | pu_var3) != 0x0) {
                u_var1 = (pu_var3 + 0x2);
                uStack778._0_2_ = u_var1;
                // u_var10 = (u_var1 >> 0x10);
//         TODO: goto LAB_1008_3206;
            }
        }
    } else {
//LAB_1008_3206:
        string_1000_3d3e(CONCAT22(param_3, acStack259 + 0x1),
                         CONCAT22(u_var10, uStack778 as u16));
    }
    pass1_1000_5008(local_40c, param_3, 0x100, &stack0xfffe);
    uStack1038 = str_op_1000_3da4(CONCAT22(param_3, local_40c));
    if (local_40c[uStack1038 - 0x1] == '\\') {
        local_40c[uStack1038 - 0x1] = 0x0;
    }
    uStack1038 = str_op_1000_3da4(CONCAT22(param_3, acStack259 + 0x1));
    if (acStack259[uStack1038] == '\\') {
        acStack259[uStack1038] = '\0';
    }
    pass1_1000_4f2e(&stack0xfffe);
    // uVar7 = (puStack774 >> 0x10);
    uStack778 = (puStack774 + 0x12);
    u_var10 = (puStack774 + 0x14);
    if ((u_var10 | uStack778) != 0x0) {
        string_1000_3d3e(CONCAT22(param_3, local_202),
                         (uStack778 & 0xffff | u_var10 << 0x10));
    }
    local_416[0] = '\0';
    pcVar8 = load_string_1010_847e(ctx.PTR_LOOP_1050_14cc, (ctx.PTR_LOOP_1050_14cc >> 0x10), 0x1010);
    string_1000_3d3e(CONCAT22(param_3, local_416), pcVar8);
    uStack1048 = str_op_1000_3da4(CONCAT22(param_3, local_416));
    uStack1038 = uStack1048;
    // for (; -0x1 < uStack1048; uStack1048 -= 0x1) {
    //   if (local_416[uStack1048] == '.') {
    //     unk_str_op_1000_3d3e
    //               (CONCAT22(param_3,local_67e),
    //                CONCAT22(param_3,local_416 + uStack1048 + 0x1));
    //     unk_str_op_1000_3d3e
    //               (CONCAT22(param_3,local_416),CONCAT22(param_3,local_67e));
    //   }
    // }
    acStack1305[1] = 0x0;
    pcVar8 = load_string_1010_847e(ctx.PTR_LOOP_1050_14cc, (ctx.PTR_LOOP_1050_14cc >> 0x10), 0x1010);
    // u_var4 = (pcVar8 >> 0x10);
    string_1000_3d3e(CONCAT22(param_3, acStack1305 + 0x1), pcVar8);
    uStack1038 = str_op_1000_3da4(CONCAT22(param_3, acStack1305 + 0x1));
    cStack1306 = acStack1305[uStack1038];
    uStack1048 = 0x0;
    while (acStack1305[uStack1048 + 0x1] != '\0') {
        if (acStack1305[uStack1048 + 0x1] == cStack1306) {
            acStack1305[uStack1048 + 0x1] = '\0';
        }
        uStack1048 += 0x1;
    }
    pass1_1000_4906(CONCAT22(param_3, &local_562), 0x0, 0x48);
    local_562 = 0x48;
    uStack1374 = (param_1 + 0x8) as u16;
    pcStack1370 = acStack1305 + 0x1;
    pcVar8 = CONCAT22(param_3, local_202);
    puStack1346 = local_302;
    uStack1350 = 0x100;
    uStack1342 = 0x100;
    pcStack1338 = acStack259 + 0x1;
    pcStack1322 = local_416;
    pcStack1382 = 0x0;
    local_666[0] = 0x0;
    in_buf_len_2 = (ctx.PTR_LOOP_1050_14cc >> 0x10);
    if (param_2 == 0x1) {
        uStack1330 = 0x1804;
        pcVar9 = load_string_1010_847e(ctx.PTR_LOOP_1050_14cc, in_buf_len_2, 0x1010);
        // u_var4 = (pcVar9 >> 0x10);
        string_1000_3d3e(CONCAT22(param_3, local_666), pcVar9);
        puStack1334 = local_666;
        BVar5 = GetOpenFileName16(0x1000);
    } else {
        if (param_2 != 0x2) {
            debug_print_1008_6048(s_Unsupported_FileStructType_in_Op_1050_01ca, 0x1000, param_3);
//       TODO: goto LAB_1008_3461;
        }
        uStack1330 = 0x6;
        pcVar9 = load_string_1010_847e(ctx.PTR_LOOP_1050_14cc, in_buf_len_2, 0x1010);
        // u_var4 = (pcVar9 >> 0x10);
        string_1000_3d3e(CONCAT22(param_3, local_666), pcVar9);
        puStack1334 = local_666;
        BVar5 = GetSaveFileName16(0x1000);
    }
    if (BVar5 != 0x0) {
        pcStack1382 = pcVar8;
    }
//LAB_1008_3461:
    if (pcStack1382 != 0x0) {
        local_67e[0] = uStack1326;
        if (uStack1326 < 0x0) {
            paStack1654 = load_string_1010_847e(ctx.PTR_LOOP_1050_14cc,
                                                (ctx.PTR_LOOP_1050_14cc >> 0x10), 0x1010);
            // u_var6 = (paStack1654 >> 0x10);
            u_var4 = str_op_1008_60e8(paStack1654, u_var6);
            paStack1654 = CONCAT22(u_var6, u_var4);
            pcVar8 = load_string_1010_847e(ctx.PTR_LOOP_1050_14cc,
                                           (ctx.PTR_LOOP_1050_14cc >> 0x10), 0x1010);
            // UStack1648 = (pcVar8 >> 0x10);
            pCStack1650 = pcVar8;
            MessageBox16(0x1010, &ctx.PTR_LOOP_1050_0010, pCStack1650, UStack1648);
            pcStack1382 = 0x0;
            paStack1646 = paStack1654;
            fn_ptr_1000_17ce(ctx, paStack1654, 0x1000);
        } else {
            str_op_1000_3dbe(CONCAT13((param_3 >> 0x8),
                                      CONCAT12(param_3 as u8, local_782)), pcVar8,
                             uStack1326);
            local_782[uStack1326] = '\0';
            if (local_782[0] != '\0') {
                pass1_1010_60cc(puStack774, CONCAT22(param_3, local_782), u_var4);
            }
        }
    }
    pass1_1000_4f2e(&stack0xfffe);
    return;
}


// WARNING: Could not reconcile some variable overlaps

pub fn set_sys_color_1008_357e(param_1: &mut StructB, param_2: i16, in_index_3: i16, param_4: u16) {
    let u_var1: u16;
    COLORREF
    colorref_var2;
    let i_var2: i16;
    let i_var3: &mut Struct53;
    let i_var4: i16;
    let u_var6: &mut Struct53;
    let u_var5: u16;
    let count: i16;
    let uVar7: u32;
    let iStack132: i16;
    let local_80: u32;
    let uStack124: u16;
    let uStack122: u16;
    let uStack120: u16;
    let uStack118: u16;
    let uStack116: u16;
    let uStack114: u16;
    let uStack112: u32;
    let uStack108: u32;
    let uStack104: u16;
    let uStack102: u16;
    let uStack100: u16;
    let uStack98: u16;
    let uStack96: u16;
    let uStack94: u16;
    let uStack92: u16;
    let uStack90: u16;
    let uStack88: u32;
    let uStack84: u32;
    let uStack80: u16;
    let uStack78: u16;
    let uStack76: u32;
    let uStack72: u32;
    let uStack68: u32;
    let uStack64: u32;
    let uStack60: u32;
    let uStack56: u32;
    let uStack52: u32;
    let uStack48: u32;
    let local_2c: u32;
    let uStack40: u32;
    let uStack36: u32;
    let uStack32: u32;
    let uStack28: u32;
    let uStack24: u32;
    let uStack20: u32;
    let uStack16: u32;
    let uStack12: u32;
    let uStack8: u32;
    let uStack4: u16;

    local_2c = 0x70004;
    uStack40 = 0xf0000;
    uStack36 = 0x100014;
    uStack32 = 0xd0012;
    uStack28 = 0x6000e;
    uStack24 = 0x80005;
    uStack20 = 0x10011;
    uStack16 = 0x30002;
    uStack12 = 0xa0009;
    uStack8 = 0xc000b;
    uStack4 = 0x13;
    local_80 = 0x0;
    uStack108 = 0x808080;
    i_var2 = 0x100;
    uStack116 = 0x0;
    uStack114 = 0x100;
    uStack100 = 0x0;
    uStack98 = 0x100;
    uStack96 = 0xffff;
    uStack94 = 0x0;
    uStack124 = 0x2;
    uStack122 = 0x100;
    uStack120 = 0x2;
    uStack118 = 0x100;
    uStack104 = 0x2;
    uStack102 = 0x100;
    uStack92 = 0x2;
    uStack90 = 0x100;
    uStack88 = 0x0;
    uStack80 = 0xc0c0;
    uStack78 = 0x0;
    uStack76 = 0x0;
    uStack72 = 0x0;
    uStack68 = 0x0;
    uStack52 = 0x0;
    u_var1 = 0x8000;
    uStack112 = 0x8000;
    uStack84 = 0x8000;
    uStack64 = 0x8000;
    uStack60 = 0x8000;
    uStack56 = 0x8000;
    uStack48 = 0x8000;
    // u_var6 = (param_1 >> 0x10);
    i_var3 = param_1;
    if (&i_var3.field_0xf8 == 0x0) {
        mem_op_1000_179c(0x54, (s_You_may_not_run_a_turn__The_game_1050_00df + 0x21),
                         0x1000);
        i_var3.field_0xf8 = u_var1;
        i_var3.field_0xfa = i_var2;
        in_index_3 = 0x1000;
        // for (iStack132 = 0x0; iStack132 < 0x15; iStack132 += 0x1) {
        //   colorref_var2 = GetSysColor16(in_index_3);
        //   uVar7 = &i_var3.field_0xf8;
        //   u_var5 = (uVar7 >> 0x10);
        //   i_var4 = uVar7;
        //   *(COLORREF *)(i_var4 + iStack132 * 0x4) = colorref_var2;
        //   (i_var4 + iStack132 * 0x4 + 0x2) = i_var2;
        //   in_index_3 = ctx.s_tile2_bmp_1050_1538;
        // }
    }
    count = in_index_3;
    if (param_2 != 0x0) {
        count = ctx.s_tile2_bmp_1050_1538;
        colorref_var2 = GetSysColor16(in_index_3);
        if ((colorref_var2 == local_80) && (i_var2 == local_80._2_2_)) {
            return;
        }
    }
    if (ctx.PTR_LOOP_1050_0010 == 0x0) {
        uStack112 = 0xc0c0c0;
    }
    if (param_2 == 0x0) {
        uVar7 = &i_var3.field_0xf8;
    } else {
        uVar7 = CONCAT22(param_4, &local_80);
    }
    SetSysColors16(count, uVar7, (COLORREF *)(uVar7 >> 0x10));
    return;
}


pub fn fill_rect_1008_39ac(in_win_handle_1: HWND16) {
    RECT16
    local_brush_handle[0x2];
    let local_brush_handle_2: *mut RECT16;
    let HStack36: HDC16;
    let local_paint_struct: PAINTSTRUCT16;

    HStack36 = BeginPaint16(in_win_handle_1, &local_paint_struct);
    local_brush_handle_2 = CreateSolidBrush16(s_tile2_bmp_1050_1538);
    GetClientRect16(ctx.s_tile2_bmp_1050_1538, local_brush_handle);
    FillRect16(ctx.s_tile2_bmp_1050_1538, local_brush_handle_2,
               local_brush_handle);
    EndPaint16(ctx.s_tile2_bmp_1050_1538, &local_paint_struct);
    DeleteObject16(ctx.s_tile2_bmp_1050_1538);
    return;
}


pub fn post_quit_msg_1008_3af4(exit_code: i16) {
    PostQuitMessage16(exit_code);
    return;
}


pub fn win_ui_op_1008_3c34(param_1: u32, param_2: u8, param_3: u16, param_4: HDC16,
                           param_5: u16)

{
    let u_var1: u16;
    let ppcVar2: u32;
    let i_var3: i16;
    let u_var4: u16;
    let u_var5: u16;
    let HStack12: HPALETTE16;
    let uStack10: u16;
    let puStack6: u32;

    // u_var4 = (param_1 >> 0x10);
    i_var3 = param_1 as i16;
    if (((i_var3 + 0xa) | (i_var3 + 0x8)) != 0x0) {
        puStack6 = (i_var3 + 0x8) as u32;
        if (((i_var3 + 0xc) != 0x0) && ((param_2 & 0x1) != 0x0)) {
            puStack6 = (i_var3 + 0xc) as u32;
        }
        if (((i_var3 + 0x10) != 0x0) && ((param_2 & 0x4) != 0x0)) {
            puStack6 = (i_var3 + 0x10) as u32;
        }
        u_var5 = (ctx.PTR_LOOP_1050_4230 >> 0x10);
        uStack10 = (ctx.PTR_LOOP_1050_4230 + 0xe);
        u_var1 = (ctx.PTR_LOOP_1050_4230 + 0x10);
        HStack12 = palette_op_1008_4e08(CONCAT22(u_var1, uStack10), &param_3, u_var1, param_4);
        ppcVar2 = (*puStack6 + 0x4);
        (**ppcVar2)(param_4, puStack6, (puStack6 >> 0x10),
                    (i_var3 + 0x28), (i_var3 + 0x26), &param_3,
                    param_5);
        SelectPalette16(param_4, 0x0, HStack12);
        DeleteObject16(ctx.s_tile2_bmp_1050_1538);
    }
    return;
}


pub fn post_msg_1008_3d20(param_1: u32, param_2: HWND16) {
    PostMessage16(param_2, 0x0, 0x0, CONCAT22(0x111, ((param_1 + 0xcc) as u16)));
    return;
}


pub fn set_di_bits_to_device_1008_45d6(param_1: u32, param_2: HDC16) {
    let info: i16;
    let u_var1: u32;
    let bVar2: bool;
    let i_var3: i16;
    let y_dest: i16;
    let u_var4: u16;
    let cx: i16;

    // u_var4 = (param_1 >> 0x10);
    i_var3 = param_1 as i16;
    if ((i_var3 + 0x6) == 0x0) {
        pass1_1008_47cc(param_1);
    }
    if (((i_var3 + 0x8) | (i_var3 + 0x6)) == 0x0) {
        bVar2 = false;
    } else {
        if (((i_var3 + 0xc) | (i_var3 + 0xa)) == 0x0) {
            pass1_1008_4834(param_1);
        }
        bVar2 = true;
    }
    if (!bVar2) {
        return;
    }
    u_var1 = (i_var3 + 0x10) as u32;
    // cx = (u_var1 >> 0x10);
    y_dest = u_var1 as i16;
    info = (y_dest + 0x8);
    u_var1 = (i_var3 + 0x14) as u32;
    SetDIBitsToDevice(param_2, 0x0, y_dest, cx, u_var1 as i16, ((u_var1 >> 0x10) as i16), info,
                      0x0, 0x0, 0x0, (BITMAPINFO *)info as u16, (y_dest + 0x4));
    return;
}


pub fn stretch_di_bits_1008_465a(
    ctx: &mut AppContext,
    param_1: &mut Struct76,
    param_2: HDC16
) {
    let mut bits: Vec<u8> = Vec::new();
    let height_src: i16;
    let u_var1: u32;
    let b_var2: bool;
    let i_var3: i16;
    let height_dst: i16;
    let u_var4: u16;
    let x_src: i16;

    // u_var4 = (param_1 >> 0x10);
    i_var3 = param_1 as i16;
    if (i_var3 + 0x6) == 0x0 {
        pass1_1008_47cc(param_1);
    }
    if (((i_var3 + 0x8) | (i_var3 + 0x6)) == 0x0) {
        b_var2 = false;
    } else {
        if (((i_var3 + 0xc) | (i_var3 + 0xa)) == 0x0) {
            pass1_1008_4834(ctx, param_1);
        }
        b_var2 = true;
    }
    if (!b_var2) {
        return;
    }
    u_var1 = (i_var3 + 0x10) as u32;
    // x_src = (u_var1 >> 0x10);
    height_dst = u_var1 as i16;
    bits = (height_dst + 0x4);
    height_src = (height_dst + 0x8);
    u_var1 = (i_var3 + 0x14) as u32;
    StretchDIBits16(param_2, 0x20, 0xcc, 0x0, height_dst, x_src, u_var1 as i16,
                    ((u_var1 >> 0x10) as i16), height_src, bits, (BITMAPINFO *)0x0, 0x0,
                    CONCAT22(bits, height_src as u16));
    return;
}


pub fn palette_op_1008_46e4(param_1: u32, param_2: u16, param_3: u16, param_4: HDC16) -> u16

{
    let bVar1: bool;
    let u_var2: u16;
    let HVar2: HPALETTE16;
    let i_var3: i16;
    let u_var4: u16;
    let u_var5: u32;
    let u_var6: u32;

    // u_var4 = (param_1 >> 0x10);
    i_var3 = param_1 as i16;
    if ((i_var3 + 0x6) == 0x0) {
        u_var5._0_2_ = param_2;
        u_var5._2_2_ = param_3;
        pass1_1008_47cc((param_1 & 0xffff | u_var4 << 0x10));
        param_2 = u_var5 as u16;
        param_3 = u_var5._2_2_;
    }
    u_var6 = CONCAT22(param_3, param_2);
    if ((i_var3 + 0x6) == 0x0) {
        bVar1 = false;
    } else {
        if ((i_var3 + 0xa) == 0x0) {
            u_var6 = pass1_1008_4834((param_1 & 0xffff | u_var4 << 0x10));
        }
        bVar1 = true;
    }
    u_var2 = u_var6 as u16;
    if (!bVar1) {
        return 0x0;
    }
    create_palette_1008_4e38((i_var3 + 0xa), param_4, (u_var6 >> 0x10));
    (i_var3 + 0xe) = u_var2 as i16;
    HVar2 = SelectPalette16(param_4, 0x0, (i_var3 + 0xe));
    (i_var3 + 0x4) = HVar2 as i16;
    RealizePalette16(ctx.s_tile2_bmp_1050_1538);
    return (i_var3 + 0x4) as u16;
}



HPALETTE16
palette_op_1008_4e08(param_1: & mut Struct13,param_2: bool,param_3: u16,param_4: HDC16)

{
let HVar1: HPALETTE16;

create_palette_1008_4e38(param_1, param_4, param_3); HVar1 = SelectPalette16(param_4,0x0, param_2); RealizePalette16(ctx.s_tile2_bmp_1050_1538); return HVar1;
}


// WARNING: Unable to use type for symbol u_var3

pub unsafe fn create_palette_1008_4e38(
    in_struct_1: &mut Struct13,
    in_log_palette_2: &mut LOGPALETTE,
    param_3: U32Ptr) {
    let pi_var1: U32Ptr;
    let pu_var2: U32Ptr;
    let u_var4: u16;
    let local_struct_1: &mut Struct13;
    let iVar5: i16;
    let iVar6: i16;
    let uVar8: u16;
    let uVar9: u16;
    let u_var10: u16;
    let iStack14: i16;
    let puStack12: U32Ptr;
    let puStack8: U32Ptr;
    let u_var3: U32Ptr;

    // uVar8 = (in_struct_1 >> 0x10);
    local_struct_1 = in_struct_1;
    u_var4 = (local_struct_1.field_0xc + 0x2) * 0x4;
    if (local_struct_1.field_0xe == 0x0) {
        in_log_palette_2 = &ctx.PTR_LOOP_1050_1000;
        mem_op_1000_179c(u_var4, param_3 as u16, 0x1000);
        &local_struct_1.field_0xe = u_var4;
        (&local_struct_1.field_0xe + 0x2) = param_3;
        *local_struct_1.field_0xe = 0x300;
        u_var3 = local_struct_1.field_0xe;
        (u_var3 + 0x2) = local_struct_1.field_0xc;
        pu_var2 = local_struct_1.field_0xe;
        puStack8 = (pu_var2 & 0xffff0000 | (pu_var2 + 0x4));
        puStack12 = local_struct_1.field_0x4;
        iStack14 = 0x0;
        loop {
            pi_var1 = &local_struct_1.field_0xc;
            if (*pi_var1 == iStack14 || *pi_var1 < iStack14) { break; }
            // uVar9 = (puStack12 >> 0x10);
            iVar5 = puStack12;
            *puStack8 = *(iVar5 + 0x2);
            // u_var10 = (puStack8 >> 0x10);
            iVar6 = puStack8;
            *(iVar6 + 0x1) = *(iVar5 + 0x1);
            *(iVar6 + 0x2) = *puStack12;
            *(iVar6 + 0x3) = 0x0;
            iStack14 += 0x1;
            puStack8 = (puStack8 & 0xffff0000 | (iVar6 + 0x4));
            puStack12 = (puStack12 & 0xffff0000 | (iVar5 + 0x4));
        }
    }
    CreatePalette16(in_log_palette_2);
    return;
}


pub fn file_and_draw_op_1008_4f20(
    ctx: &mut AppContext,
    param_1: &mut Struct76,
    param_2: u32,
    param_3: u16,
    param_4: u32,
    param_5: u16
)

{
    let u_var1: u32;
    let b_force_background: u16;
    let color: COLORREF;
    let color_00: COLORREF;
    let x: u16;
    let u_var2: u16;
    let mut output: String;
    let i_var3: &mut Struct76;
    let u_var4: u16;
    let pa_var5: &mut Struct43;
    let u_var6: u32;
    init_data: DEVMODEA;
    let local_2c: HDC16;
    let mut p_cstack42: String;
    let mut p_cstack40: String;
    let local_26: [u8; 24];

    pass1_1008_4016(ctx, param_1);
    // u_var4 = (param_1 >> 0x10);
    i_var3 = param_1;
    (i_var3.field_0x1e) = param_4 as i16;
    (i_var3.field_0x22) = param_3 as i16;
    (i_var3.field_0x24) = param_2 as i16;
    param_1.field_0x0 = (ctx.s_SCInternalPutBldg2_site_0x_08lx__1050_5099[9..].to_string());
    (i_var3.field_0x2) = 0x1008;
    pa_var5 = unk_io_op_1010_830a(ctx.PTR_LOOP_1050_14cc, 0x2, param_5);
    // u_var2 = (pa_var5 >> 0x10);
    struct_op_1008_48fe(CONCAT22(param_5, local_26), 0x1, pa_var5, u_var2);
    read_file_1008_49e8(CONCAT22(param_5, local_26), 0x1010, u_var2);
    pass1_1008_5068(param_1, CONCAT22(param_5, local_26));
    pass1_1008_47cc(param_1);
    pass1_1008_4834(param_1);
    init_data = 0x0;
    u_var6 = pass1_1008_4772(param_1);
    // output = (u_var6 >> 0x10);
    p_cstack42 = u_var6;
    p_cstack40 = output;
    local_2c = CreateDC16(0x1010, p_cstack42, output, init_data);
    b_force_background = palette_op_1008_46e4(param_1, &local_2c, output,
                                              ctx.s_tile2_bmp_1050_1538);
    color = SetBkColor16(ctx.s_tile2_bmp_1050_1538, 0xffff);
    color_00 = SetTextColor16(ctx.s_tile2_bmp_1050_1538, (i_var3 + 0x22));
    x = str_op_1000_3da4((i_var3 + 0x1e));
    u_var1 = (i_var3 + 0x1e) as u32;
    TextOut16(0x1000, x, u_var1, (u_var1 >> 0x10), 0x0);
    SetBkColor16(ctx.s_tile2_bmp_1050_1538, color);
    SetTextColor16(ctx.s_tile2_bmp_1050_1538, color_00);
    SelectPalette16(ctx.s_tile2_bmp_1050_1538, 0x0, b_force_background);
    DeleteObject16(ctx.s_tile2_bmp_1050_1538);
    DeleteDC16(ctx.s_tile2_bmp_1050_1538);
    close_file_1008_496c(local_26, param_5);
    return;
}


pub fn cleanup_palette_1008_56e2(param_1: i32, param_2: HDC16) -> bool

{
    let HVar1: HPALETTE16;
    let u_var2: u16;

    // u_var2 = (param_1 >> 0x10);
    HVar1 = SelectPalette16(param_2, 0x0, (param_1 + 0x4));
    (param_1 + 0x4) = HVar1 as i32;
    DeleteObject16(ctx.s_tile2_bmp_1050_1538);
    return 0x1;
}


pub fn win_1008_5c5c(param_1: &WNDCLASS16, param_2: u16, param_3: u16, param_4: u32,
                     param_5: u16)

{
    pass1_1010_84f8(ctx.PTR_LOOP_1050_14cc, param_5, param_1);
    win_ui_op_1008_5cfe(param_4, CONCAT22(param_3, param_2), param_1);
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn win_1008_5c7c(param_1: u32, param_2: u32, param_3: &WNDCLASS16, param_4: u16,
                     param_5: u16)

{
    pass1_1010_85be(ctx.PTR_LOOP_1050_14cc, param_2, (param_2 >> 0x10), param_3);
    win_ui_op_1008_5cfe(param_1, CONCAT22(param_5, param_4), param_3);
    return;
}


pub fn win_1008_5c9e(param_1: u32, param_2: U32Ptr, param_3: u16, param_4: u16,
                     param_5: &WNDCLASS16)

{
    win_1008_5c7c(param_1, *param_2, param_5, param_3, param_4);
    return;
}


pub fn win_ui_op_1008_5cfe(param_1: u32, param_2: &mut String, in_wnd_class: &WNDCLASS16) {
    let u_var1: u32;
    let i_var2: i16;
    let i_var3: i16;
    let u_var4: u16;
    let DVar5: u32;
    let message_1: HWND16;
    let uStack298: u16;
    let window_handle_1: HWND16;
    let local_11e: [u8; 100];
    let mut string_1: String;
    let iStack26: i16;
    let iStack24: i16;
    let local_16: [u8; 4];
    let offset_val: i16;
    let mut pcStack14: String;
    let mut pcStack10: String;

    pass1_1000_4906(CONCAT22(in_wnd_class, local_16), 0x0, 0x14);
    pcStack10 = param_2;
    // u_var4 = (param_1 >> 0x10);
    i_var3 = param_1 as i16;
    u_var1 = (i_var3 + 0xc) as u32;
    iStack24 = (u_var1 + 0x72) as i16;
    iStack26 = 0x1;
    string_1 = s_waveaudio_1050_02a4;
    str_1000_4d58(param_2, 0x0, 0x0, 0x0, CONCAT22(in_wnd_class, local_11e),
    );
    i_var2 = string_1000_475e(ctx, CONCAT22(in_wnd_class, local_11e), 0x105002ae);
    if (i_var2 == 0x0) {
        u_var1 = (i_var3 + 0xc) as u32;
        iStack24 = (u_var1 + 0x74) as i16;
        string_1 = s_sequencer_1050_02b3;
        iStack26 = 0x0;
    }
    if (iStack24 != 0x0) {
        if ((iStack26 != 0x0) && ((i_var3 + 0x10) != 0x0)) {
            return;
        }
        if ((iStack26 == 0x0) && ((i_var3 + 0x12) != 0x0)) {
            return;
        }
        pcStack14 = string_1;
        DVar5 = mciSendCommand16(0x1000, local_16, CONCAT13(0x22, ZEXT23(in_wnd_class)),
                                 0x8030000);
        if (((DVar5 >> 0x10) | DVar5) == 0x0) {
            if (iStack26 == 0x0) {
                (i_var3 + 0x12) = 0x1;
            } else {
                (i_var3 + 0xa) = offset_val;
                (i_var3 + 0x10) = 0x1;
            }
            window_handle_1 = create_window_1008_5e7e(ctx, ctx.s_tile2_bmp_1050_1538, in_wnd_class);
            if (window_handle_1 == 0x0) {
                mci_send_command_1008_5cb6(param_1, offset_val, s_tile2_bmp_1050_1538);
                return;
            }
            pass1_1000_4906(CONCAT22(in_wnd_class, &message_1), 0x0,
                            0xc);
            message_1 = window_handle_1;
            uStack298 = 0x0;
            mciSendCommand16(0x1000, &message_1, CONCAT12(0x1, in_wnd_class),
                             0x8060000);
            SetWindowWord16(ctx.s_tile2_bmp_1050_1538, offset_val, 0x0);
            return;
        }
    }
    if (iStack26 == 0x0) {
        i_var2 = 0x11;
    } else {
        i_var2 = 0x10;
    }
    pass1_1010_1f62(in_wnd_class, param_1, i_var2);
    return;
}


pub fn create_window_1008_5e7e(
    ctx: &mut AppContext,
    stock_obj_id: u16,
    in_wnd_class: &WNDCLASS16) -> HWND16 {
    let pu_var1: u32;
    let pu_var2: u32;
    let bvar3: bool;
    let avar4: ATOM;
    let window_handle_1: HWND16;
    let i_var5: i16;
    let mut string_1: String;
    let pu_var6: String;
    let name: String;
    let u_stack42: u16;
    let u_stack40: u16;
    let u_stack38: u16;
    let u_stack36: u16;
    let pu_stack34: U32Ptr;
    let u_stack32: u16;
    let u_stack30: u16;
    let hstack28: HGDIOBJ16;
    let u_stack26: u32;
    let pu_stack22: u32;
    let local_12: [u32; 0x4] = [0;4];

    pu_var6 = local_12;
    string_1 = ctx.s_MciSoundWindow_1050_02bd.clone();
    // for (i_var5 = 0x3; i_var5 != 0x0; i_var5 += -0x1) {
    //   pu_var2 = pu_var6;
    //   pu_var6 = pu_var6 + 0x1;
    //   pu_var1 = string_1;
    //   string_1 = (string_1 + 0x4);
    //   *pu_var2 = *pu_var1;
    // }
    pu_var6 = string_1.clone();
    // *(pu_var6 + 0x2) = *(string_1 + 0x2);
    pu_var6[2] = string_1[2];
    name = 0x2000;
    u_stack42 = SUB42(address_of(ctx.DAT_1050_5f44) as u16, 0x0);
    u_stack40 = 0x1008;
    u_stack36 = 0x2;
    pu_stack34 = ctx.PTR_LOOP_1050_038c;
    u_stack32 = 0x0;
    u_stack30 = 0x0;
    u_stack38 = 0x0;
    hstack28 = GetStockObject16(stock_obj_id as i16);
    u_stack26 = 0x0;
    pu_stack22 = local_12[0];
    bvar3 = GetClassInfo16(ctx.s_tile2_bmp_1050_1538, &name, in_wnd_class);
    if (bvar3 == 0x0) {
        avar4 = RegisterClass16(ctx.s_tile2_bmp_1050_1538);
        if (avar4 == 0x0) {
            OutputDebugString16(ctx.s_tile2_bmp_1050_1538);
            return 0x0;
        }
    }
    window_handle_1 = CreateWindow16(ctx.s_tile2_bmp_1050_1538, 0x0,
                                     ZEXT24(ctx.PTR_LOOP_1050_038c) << 0x10, 0x0, PTR_LOOP_1050_0396, 0x1, 0x1, 0x8000, 0x8000, 0x0, 0xcf);
    return window_handle_1;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

LRESULT
make_def_win_proc_1008_5f44
(param_1: u16,in_wparam_2: WPARAM16,param_3: LPARAM,in_hwnd_4: HWND16)

{
let WVar1: u16; let in_DX: * mut u8; let unaff_DI: i16; WNDCLASS16 * unaff_SS; let LVar2: LRESULT; let pu_var3: * mut u16;

if (param_3._2_2_ == 0x2) {
WVar1 = GetWindowWord16(in_hwnd_4, 0x0); mci_send_command_1008_5cb6(ctx.PTR_LOOP_1050_02a0, WVar1, s_tile2_bmp_1050_1538);
pu_var3 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x37,unaff_SS, in_DX, unaff_DI); pass1_1008_aa28(pu_var3, pu_var3, unaff_SS);
}
else {
if (param_3._2_2_ != 0x3b9) {
LVar2 = DefWindowProc16(in_hwnd_4, param_1, in_wparam_2,param_3); return LVar2;
}
DestroyWindow16(in_hwnd_4);
}
return 0x0;
}



u16
win_ui_op_1008_8214(param_1: u16,param_2: i16,param_3: u16,param_4: u32,param_5: u16,
param_6: * mut u8,param_7: HWND16)

{
let IVar1: i16; let pu_var2: u32;
let pu_var3: * mut u16; let u_var4: u16;

if (param_2 == 0x81) {
u_var4 = 0x6; mem_op_1000_179c(0x6, param_6,0x1000); if ((param_6 | param_5) == 0x0) {
pu_var2 = 0x0;
}
else {
pu_var2 = pass1_1008_80d2(CONCAT22(param_6, param_5));
}
param_7 = ctx.s_tile2_bmp_1050_1538; SetWindowLong16(0x1000,pu_var2, CONCAT22(u_var4, (pu_var2 > > 0x10)));
}
if (param_2 == 0x1) {
pu_var3 = GetWindowLong16(param_7, 0x0); *pu_var3 = (param_4 + 0x8); IVar1 = GetDlgCtrlID16(ctx.s_tile2_bmp_1050_1538); (pu_var3 + 0x2) = IVar1;
}
return 0x1;
}



pub fn send_msg_1008_84ba(param_1: u16, param_2: u32, param_3: HWND16) {
    let i_var1: i16;
    let u_var2: u16;
    let uStack4: u16;

    // u_var2 = (param_2 >> 0x10);
    i_var1 = param_2 as i16;
    if (((i_var1 + 0x4) & 0x4) == 0x0) {
        if (((i_var1 + 0x4) & 0x8) == 0x0) {
            return;
        }
        uStack4 = 0x1;
    } else {
        uStack4 = 0x0;
    }
    SendMessage16(param_3, ((i_var1 + 0x2) as u16), 0x0, CONCAT22(0x115, uStack4) as i32);
    return;
}


pub fn win_sys_op_1008_84f2(param_1: u16, param_2: u16, param_3: i16, param_4: u32, param_5: HWND16) {
    let pbVar1: U32Ptr;
    let i_var2: i16;
    let i_var3: i16;
    let Bvar4: bool;
    let u_var5: u16;
    let u_var6: u16;
    let paVar7: &mut Struct18;
    let cVar8: u8;
    let local_a: RECT16;
    let i_stack4: i16;

    paVar7 = GetWindowLong16(param_5, 0x0);
    // u_var6 = (paVar7 >> 0x10);
    i_var3 = paVar7;
    if (param_4 == 0x1f) {
        (i_var3 + 0x4) = 0x0;
        KillTimer16(ctx.s_tile2_bmp_1050_1538, 0xfa6);
        KillTimer16(ctx.s_tile2_bmp_1050_1538, 0xfa7);
        ReleaseCapture16();
    } else {
        cVar8 = param_4 as u8;
        if (param_4 < 0x20) {
            if (param_4 != 0x14) {
                if (0x14 < param_4) {
                    // goto
                    // LAB_1008_8771;
                }
                u_var5 = (param_4 & 0xff00 | (cVar8 - 0x1)) as u16;
                if ((cVar8 - 0x1) == 0x0) {
//LAB_1008_8560:
                    win_ui_op_1008_8214(param_4._2_2_, param_4, param_3,
                                        CONCAT22(param_2, param_1), u_var5, u_var6,
                                        ctx.s_tile2_bmp_1050_1538);
                    return;
                }
                if (cVar8 == '\x02') {
                    fn_ptr_1000_17ce(ctx, paVar7, 0x1000);
                } else {
                    if (cVar8 != '\f') {
                        if (cVar8 != '\x0f') {
                            // goto
                            // LAB_1008_8771;
                        }
                        draw_op_1008_8288(param_4._2_2_, paVar7, s_tile2_bmp_1050_1538);
                    }
                }
            }
        } else {
            if (param_4 == 0x200) {
                if (((i_var3 + 0x4) & 0x1) != 0x0) {
                    GetClientRect16(ctx.s_tile2_bmp_1050_1538, &local_a);
                    i_var2 = (i_var3 + 0x4);
                    pbVar1 = (i_var3 + 0x4);
                    *pbVar1 = *pbVar1 & 0xf3;
                    BVar4 = PtInRect16(s_tile2_bmp_1050_1538,
                                       CONCAT22(param_2, param_1));
                    if (BVar4 == 0x0) {
                        pbVar1 = (i_var3 + 0x4);
                        *pbVar1 = *pbVar1 | 0x2;
                    } else {
                        if (param_2 < (i_stack4 >> 0x1) as u16) {
                            pbVar1 = (i_var3 + 0x4);
                            *pbVar1 = *pbVar1 | 0x4;
                        } else {
                            pbVar1 = (i_var3 + 0x4);
                            *pbVar1 = *pbVar1 | 0x8;
                        }
                        pbVar1 = (i_var3 + 0x4);
                        *pbVar1 = *pbVar1 & 0xfd;
                    }
                    if ((i_var3 + 0x4) != i_var2) {
                        InvalidateRect16(ctx.s_tile2_bmp_1050_1538,
                                         (&ctx.PTR_LOOP_1050_0000 + 0x1), 0x0);
                        UpdateWindow16(ctx.s_tile2_bmp_1050_1538);
                    }
                }
            } else {
                if (param_4 < 0x201) {
                    u_var5 = (param_4 - 0x81) as u16;
                    if (u_var5 == 0x0) {
                        // goto
                        // LAB_1008_8560;
                    }
                    if (param_4 != 0x113) {
//LAB_1008_8771:
                        DefWindowProc16(ctx.s_tile2_bmp_1050_1538, param_1, param_2,
                                        CONCAT13(((param_4 >> 0x8) as u16), CONCAT12(cVar8, param_3 as u16)));
                        return;
                    }
                    if (param_3 == 0xfa6) {
                        KillTimer16(ctx.s_tile2_bmp_1050_1538, 0xfa6);
                        SetTimer16(ctx.s_tile2_bmp_1050_1538, 0x0, 0x0,
                                   (&ctx.PTR_LOOP_1050_0000 + 0x1));
                    }
                    if (((i_var3 + 0x4) & 0x2) == 0x0) {
                        send_msg_1008_84ba(param_4._2_2_, paVar7, s_tile2_bmp_1050_1538);
                    }
                } else {
                    if (param_4 != 0x201) {
                        if (param_4 == 0x202) {
                            KillTimer16(ctx.s_tile2_bmp_1050_1538, 0xfa6);
                            KillTimer16(ctx.s_tile2_bmp_1050_1538, 0xfa7);
                            ReleaseCapture16();
                            u_var5 = (i_var3 + 0x4) as u16;
                            if (((u_var5 & 0x1) != 0x0) && ((u_var5 & 0xfffd) != 0x0)) {
                                pbVar1 = (i_var3 + 0x4);
                                *pbVar1 = *pbVar1 & 0xf2;
                                InvalidateRect16(ctx.s_tile2_bmp_1050_1538,
                                                 (&ctx.PTR_LOOP_1050_0000 + 0x1), 0x0);
                                UpdateWindow16(ctx.s_tile2_bmp_1050_1538);
                            }
                            SendMessage16(ctx.s_tile2_bmp_1050_1538, ((i_var3 + 0x2) as u16), 0x0,
                                          0x11100f9);
                            return;
                        }
                        if (param_4 != 0x203) {
                            // goto
                            // LAB_1008_8771;
                        }
                    }
                    pbVar1 = (i_var3 + 0x4);
                    *pbVar1 = *pbVar1 | 0x1;
                    GetClientRect16(ctx.s_tile2_bmp_1050_1538, &local_a);
                    if (param_2 < (i_stack4 >> 0x1) as u16) {
                        pbVar1 = (i_var3 + 0x4);
                        *pbVar1 = *pbVar1 | 0x4;
                    } else {
                        pbVar1 = (i_var3 + 0x4);
                        *pbVar1 = *pbVar1 | 0x8;
                    }
                    send_msg_1008_84ba(param_4._2_2_, paVar7, s_tile2_bmp_1050_1538);
                    SetTimer16(ctx.s_tile2_bmp_1050_1538, 0x0, 0x0,
                               (s_New_failed_in_Op__Op__Simulator_1050_0110 + 0x1c));
                    InvalidateRect16(ctx.s_tile2_bmp_1050_1538,
                                     (&ctx.PTR_LOOP_1050_0000 + 0x1), 0x0);
                    UpdateWindow16(ctx.s_tile2_bmp_1050_1538);
                    SetCapture16(ctx.s_tile2_bmp_1050_1538);
                }
            }
        }
    }
    return;
}


pub fn send_msg_1008_9640(param_1: u32, param_2: u16, param_3: HWND16) {
    if ((param_1 + 0x8) != 0x0) {
        SendMessage16(param_3, 0x0, 0x0, CONCAT22(0x86, param_2) as i32);
    }
    return;
}


pub fn set_win_text_1008_9664(param_1: u32, param_2: u16, param_3: &mut String) {
    string_1000_3d3e((param_1 & 0xffff0000 | (param_1 + 0xa)), param_3);
    SetWindowText16(0x1000, param_1 + 0xa);
    return;
}


pub fn show_win_1008_96ae(param_1: u32, param_2: i16, param_3: HWND16) -> bool

{
    let b_var1: bool;

    if ((param_1 + 0x8) != 0x0) {
        b_var1 = ShowWindow16(param_3, param_2);
        return b_var1;
    }
    return 0x0;
}


pub fn win_ui_reg_class_1008_96d2(
    ctx: &mut AppContext,
    param_1: &mut Struct20,
    in_h_inst_2: HINSTANCE16,
    in_wnd_class_3: &mut WNDCLASS16,
    stack0xfffe: u16)

{
    let bvar1: bool;
    let avar2: ATOM;
    let mut name_1c: String = String::new();
    let u_stack26: u16;
    let u_stack24: u16;
    let u_stack22: u32;
    let pu_stack18: U32Ptr;
    let u_stack16: u16;
    let u_stack14: u16;
    let u_stack12: u16;
    let u_stack10: u32;
    let i_stack6: i16;
    let u_stack4: u16;

    i_stack6 = param_1 + 0x5b;
    bvar1 = GetClassInfo16(in_h_inst_2, &name_1c, in_wnd_class_3);
    if bvar1 == false {
        name_1c = (param_1 + 0xc8);
        u_stack26 = 0x5632;
        u_stack24 = 0x1008;
        u_stack22 = 0x40000;
        pu_stack18 = ctx.PTR_LOOP_1050_038c;
        u_stack16 = (param_1 + 0xc2);
        u_stack14 = (param_1 + 0xc4);
        u_stack12 = (param_1 + 0xc6);
        u_stack10 = 0x0;
        u_stack4 = param_1._2_2_;
        avar2 = RegisterClass16(get_struct_ref_from_addr::<WNDCLASS16>(ctx.s_tile2_bmp_1050_1538));
        if avar2 == 0x0 {
            fn_ptr_op_1000_24cd(0x0, stack0xfffe as i16);
        }
    }
    return;
}


pub fn create_window_ex_1008_9760(astruct *in_struct_1, param_2: u16) {
    let u_var1: u32;
    let window_handle: HWND16;
    astruct * struct_1;
    let mut class_name: String;

    // class_name = (in_struct_1 >> 0x10);
    struct_1 = (astruct *)
    in_struct_1;
    if (struct_1.field_0x8 == 0x0) {
        u_var1 = struct_1.field_0xac;
        window_handle = CreateWIndowEx16(CONCAT22(struct_1, param_2), class_name, PTR_LOOP_1050_038c,
                                         CONCAT22(struct_1.field_0xbc, struct_1.field_0xca),
                                         struct_1.field_0xba, struct_1.field_0xb8, struct_1.field_0xb6,
                                         struct_1.field_0xb4, u_var1 as u16,
                                         ((u_var1 >> 0x10) as u16), 0x39e, ctx.data_seg);
        struct_1.field_0x8 = window_handle;
    }
    if (struct_1.field_0x8 == 0x0) {
        fn_ptr_op_1000_24cd(0x0, &stack0xfffe);
    }
    return;
}


pub fn begin_end_paint_1008_97c8(param_1: HWND16) {
    let local_22: PAINTSTRUCT16;

    BeginPaint16(param_1, &local_22);
    EndPaint16(ctx.s_tile2_bmp_1050_1538, &local_22);
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

u32
unk_win_op_1008_97f2
(param_1: * mut u32,param_2: & mut i16,param_3: WPARAM16,param_4: * mut u8,param_5: u16,
param_6: HWND16)

{
let ppcVar1: u32; let BVar2: bool;
let u_var3: u16; let i_var4: i16;
let u_var5: u16; let msg: u16;
let wparam: u16; let unaff_SS: u16;
let u_var6: u32; let uVar7: u8;
let uVar8: u8; let cVar9: u8;

msg = param_1;
// wparam = (param_1 >> 0x10); if (param_5 == 0x2b) {
if ( * param_2 == 0x4) {
win_ui_get_prop_op_1040_9566
(CONCAT22(param_3, param_2), &ctx.PTR_LOOP_1050_1040);
}
else {
ppcVar1 = ( * param_1 + 0x70); (* * ppcVar1)();
}
u_var5 = 0x1;
//     TODO: goto LAB_1008_9a95;
}
// uVar8 = (param_1 >> 0x10);
uVar7 = SUB41(param_1, 0x0); if (0x2b < param_5) {
cVar9 = param_5; if (param_5 == 0x112) {
if ((ctx.PTR_LOOP_1050_039a == 0x0) && (ppcVar1 = ( * param_1 + 0x48), i_var4 = ( * * ppcVar1)(), i_var4 != 0x0))
{
make_def_wnd_proc_1008_9ce6
(msg, wparam, param_2, param_3,
CONCAT13(0x1, CONCAT12(cVar9, param_4)), param_6);
}
}
else {
if (param_5 < 0x113) {
if (param_5 == 0x86) {
ppcVar1 = ( * param_1 + 0x80); u_var6 = ( * * ppcVar1)(); return u_var6;
}
if (param_5 < 0x87) {
if (param_5 == 0x85) {
ppcVar1 = ( * param_1 + 0x7c); u_var6 = ( * * ppcVar1)(); return u_var6;
}
if (param_5 < 0x86) {
if (cVar9 == '7') {
return (msg + 0xc2);
}
if (cVar9 == 'A') {
ppcVar1 = ( * param_1 + 0x2c); (* * ppcVar1)(param_6, param_1);
//               TODO: goto switchD_1008_9b30_caseD_1;
}
}
//           TODO: goto switchD_1008_9b30_caseD_4;
}
if (param_5 == 0x100) {
if (ctx.PTR_LOOP_1050_039a == 0x0) {
ppcVar1 = ( * param_1 + 0x6c); (* * ppcVar1)();
}
}
else {
if (param_5 == 0x102) {
if (ctx.PTR_LOOP_1050_039a == 0x0) {
ppcVar1 = ( * param_1 + 0x68); (* * ppcVar1)();
}
}
else {
if (param_5 != 0x111) {
// goto switchD_1008_9b30_caseD_4;
}
if ((param_4 != ctx.PTR_LOOP_1050_039c) && (ctx.PTR_LOOP_1050_039a == 0x0)) {
if (param_2 == 0x0) {
ppcVar1 = ( * param_1 + 0x40); (* * ppcVar1)();
}
else {
ppcVar1 = ( * param_1 + 0x44); (* * ppcVar1)();
}
}
}
}
}
else {
if (param_5 == 0x204) {
if (ctx.PTR_LOOP_1050_039a == 0x0) {
ppcVar1 = ( * param_1 + 0x60); (* * ppcVar1)();
}
}
else {
if (param_5 < 0x205) {
if (param_5 == 0x113) {
if (ctx.PTR_LOOP_1050_0388 != 0x0) {
pass1_1008_932a(ctx.PTR_LOOP_1050_0388, unaff_SS);
}
}
else {
if (param_5 == 0x117) {
if (param_3 == 0x0) {
ppcVar1 = ( * param_1 + 0x4c); (* * ppcVar1)();
}
else {
ppcVar1 = ( * param_1 + 0x20); (* * ppcVar1)();
}
}
else {
if (param_5 != 0x201) {
// goto switchD_1008_9b30_caseD_4;
}
if (ctx.PTR_LOOP_1050_039a == 0x0) {
ppcVar1 = ( * param_1 + 0x5c); (* * ppcVar1)();
}
}
}
}
else {
if (param_5 == 0x210) {
ppcVar1 = ( * param_1 + 0x64); (* * ppcVar1)();
}
else {
if (param_5 == 0x30f) {
//LAB_1008_9af8:
ppcVar1 = ( * param_1 + 0x8c); i_var4 = ( * * ppcVar1)(param_6, param_1);
//LAB_1008_9ada: return i_var4;
}
if (param_5 == 0x311) {
ppcVar1 = ( * param_1 + 0x88); i_var4 = ( * * ppcVar1)(); if (i_var4 != 0x0) {
// goto LAB_1008_9af8;
}
}
else {
if (param_5 != 0x3b9) {
// goto switchD_1008_9b30_caseD_4;
}
ppcVar1 = ( * param_1 + 0x24); (* * ppcVar1)();
}
}
}
}
}
}
//     TODO: goto switchD_1008_9b30_caseD_1;
}
if (false) {
switchD_1008_9b30_caseD_4: if ((param_5 < 0x400) || (0x7ffe < param_5)) {
u_var6 = make_def_wnd_proc_1008_9ce6
(msg, wparam, param_2,param_3, CONCAT22(param_5, param_4),
param_6); return u_var6;
}
ppcVar1 = ( * param_1 + 0x28); (* * ppcVar1)(param_6, uVar7, uVar8, param_2, param_3, CONCAT22(param_5, param_4));
}
else {
param_6 = 0x1008; switch(param_5) {
0x1 => break; 0x2 => ppcVar1 = ( *param_1 + 0x3c); ( * * ppcVar1)(0x1008, param_1); SetWindowLong16(0x1008,0x0, 0x0); BVar2 = IsWindow16(ctx.s_tile2_bmp_1050_1538); if (BVar2 != 0x0) {
PostMessage16(ctx.s_tile2_bmp_1050_1538, msg, wparam,0x11100c7);
}
break; 0x3 => ppcVar1 = ( * param_1 + 0x54); ( * * ppcVar1)(0x8, uVar7, wparam, param_3, param_2);
break; default:
//       TODO: goto switchD_1008_9b30_caseD_4;
0x5 => ppcVar1 = ( *param_1 + 0x58); ( * * ppcVar1)(0x8, uVar7, uVar8, param_3, param_2, param_4); break; 0x7 => ppcVar1 = ( * param_1 + 0x50); ( * *ppcVar1)(0x8, param_1, param_4);
break; 0x8 => ppcVar1 = ( * param_1 + 0x74); ( * * ppcVar1)(0x8, param_1, param_4); break; 0xd =>
ppcVar1 = ( * param_1 + 0x84); i_var4 = ( * * ppcVar1)(0x8, uVar7, uVar8, param_2, CONCAT12(param_4._0_1_, param_3));
//       TODO: goto LAB_1008_9ada;
0xf => ppcVar1 = (* param_1 + 0x34); ( * * ppcVar1)(0x1008, param_1); break; 0x10 => ppcVar1 = ( * param_1 + 0x38);
u_var6 = ( * * ppcVar1)(0x1008,param_1); return u_var6; 0x19 => ppcVar1 = ( * param_1 + 0x78); u_var3 = (* * ppcVar1)(0x8, uVar7, uVar8, param_2, CONCAT12(param_4._0_1_, param_3)); return CONCAT22(0x1050, u_var3); 0x1c => ppcVar1 = ( * param_1 + 0x30); ( * * ppcVar1)(0x8,param_1, param_4);
}
}
switchD_1008_9b30_caseD_1: u_var5 = 0x0;
//LAB_1008_9a95: return u_var5;
}


pub fn get_stock_obj_1008_9c56(param_1: i16) {
    GetStockObject16(param_1);
    return;
}


LRESULT
make_def_wnd_proc_1008_9ce6
(param_1: u16,param_2: u16,in_msg_3: u16,param_4: WPARAM16,param_5: LPARAM,
in_hwnd_5: HWND16)

{
let LVar1: LRESULT;

LVar1 = DefWindowProc16(in_hwnd_5, in_msg_3, param_4, param_5); return LVar1;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn post_win_msg_1008_a0e4(param_1: &mut Struct67, param_2: i32, param_3: i16, param_4: u16, param_5: u32,
                              param_6: i16, param_7: HWND16, param_8: u16)

{
    let pu_var1: u32;
    let ppcVar2: u32;
    let u_var3: u16;
    let bVar4: bool;
    let puVar4: &mut Struct68;
    let u_var5: &mut Struct66;
    let extraout_dx: u16;
    let uVar7: u16;
    let iVar7: &mut Struct67;
    let u_var6: &mut Struct67;
    let paStack14: &mut Struct99;
    let local_a: [u8; 8];

    // u_var6 = (param_1 >> 0x10);
    iVar7 = param_1;
    pass1_1008_5784(CONCAT22(param_8, local_a), iVar7.field_0xa);
    bVar4 = false;
    loop {
        puVar4 = local_a;
        pass1_1008_5b12(puVar4, param_8);
        if ((extraout_dx | puVar4) == 0x0) {
            // goto
            // LAB_1008_a146;
        }
        if ((param_1 + 0xde) != param_5) == false { break; }
    }
    puVar4.field_0xc = puVar4.field_0xc + param_3;
    puVar4.field_0xe = puVar4.field_0xe + param_2;
    bVar4 = true;
//LAB_1008_a146:
    if (!bVar4) {
        param_7 = 0x1000;
        paStack14 = pass1_1000_07fc(0x1000, _PTR_LOOP_1050_03a0);
        // uVar7 = (paStack14 >> 0x10);
        u_var3 = paStack14;
        if ((uVar7 | u_var3) == 0x0) {
            paStack14 = 0x0;
        } else {
            paStack14.field_0x0 = 0x389a;
            (u_var3 + 0x2) = 0x1008;
            (u_var3 + 0x4) = param_6 as u16;
            (u_var3 + 0x6) = param_5 as u16;
            (u_var3 + 0xa) = param_4;
            (u_var3 + 0xc) = param_3 as u16;
            (u_var3 + 0xe) = param_2 as u16;
            paStack14.field_0x0 = 0xad8e;
            (u_var3 + 0x2) = 0x1008;
        }
        pu_var1 = iVar7.field_0xa;
        ppcVar2 = (*iVar7.field_0xa + 0x8);
        (**ppcVar2)(0x1000, pu_var1, (pu_var1 >> 0x10), paStack14,
                    (paStack14 >> 0x10));
    }
    if (param_6 == 0x14) {
        PostMessage16(param_7, 0x0, 0x0, 0x11100fc);
    }
    return;
}


pub fn pass1_1008_a1f0(param_1: u16, param_2: u16, param_3: u8, param_4: u32, param_5: U32Ptr,
                       param_6: U32Ptr, param_7: U32Ptr, param_8: U32Ptr)

{
    let u_var1: u32;
    let ppcVar2: u32;
    let u_var3: u32;
    let u_var4: u16;
    let u_var5: u16;
    let extraout_dx: u16;
    let u_var6: u16;
    let puVar7: U32Ptr;
    let uVar8: u16;
    let i_var9: i16;
    let in_buf_len_5: U32Ptr;
    let u_var10: u16;
    let puVar11: U32Ptr;
    let mut pcVar12: String;
    let uVar13: u16;
    let uVar14: u8;
    let uVar15: u8;
    local_106: u8[0x100];
    let puStack6: u32;

    u_var4 = 0x0;
    *param_8 = 0x0;
    *param_7 = 0x0;
    *param_6 = 0x0;
    *param_5 = 0x0;
    // in_buf_len_5 = (param_4 >> 0x10);
    uVar8 = param_4 as u16;
    *(uVar8 + 0xe) = 0x0;
    ppcVar2 = ((uVar8 + 0xa) + 0x10) as u32;
    (**ppcVar2)(param_1, (uVar8 + 0xa));
    puStack6 = CONCAT22(extraout_dx, u_var4);
    u_var6 = extraout_dx | u_var4;
    if (u_var6 == 0x0) {
        return;
    }
    *param_8 = (u_var4 + 0x4);
    *param_6 = (u_var4 + 0xa);
    u_var5 = pass1_1008_ab80(uVar8, in_buf_len_5, *param_8);
    *param_5 = u_var5;
    // u_var10 = (puStack6 >> 0x10);
    i_var9 = puStack6 as i16;
    if (false) {
        // goto
        // switchD_1008_a835_caseD_3;
    }
    param_1 = 0x1008;
    uVar13 = ctx._PTR_LOOP_1050_14cc;
    u_var5 = (ctx.PTR_LOOP_1050_14cc >> 0x10);
    switch((i_var9 + 0x4))
    {
        0x1 => load_string_1010_84e0(0x1010, uVar13, u_var5, 0x3ff, (uVar8 + 0xe), in_buf_len_5);
        *param_7 = 0xd1;
//     TODO: goto LAB_1008_a2b1;
        0x2 => u_var1 = (i_var9 + 0x6) as u32;
        pass1_1028_e1ec(ctx.PTR_LOOP_1050_65e2, u_var1, (u_var1 >> 0x10));
        load_string_1010_84e0(0x1010, _PTR_LOOP_1050_14cc,
                              (ctx.PTR_LOOP_1050_14cc >> 0x10), 0x100, local_106, param_2);
        u_var3 = puStack6 >> 0x10;
        pcVar12 = pass1_1038_4d28(CONCAT13((u_var6 >> 0x8), CONCAT12(u_var6 as u8, i_var9 as u16)));
        param_1 = 0x1000;
        sys_1000_3f9c((uVar8 + 0xe), in_buf_len_5, local_106, param_2,
                      pcVar12, &stack0xfffe, u_var3, 0x1000, param_2, param_3);
        break;
        0x5 =>
//     TODO: goto LAB_1008_a277;
        0x6 => load_string_1010_84e0(0x1010, uVar13, u_var5, 0x3ff, (uVar8 + 0xe), in_buf_len_5);
        *param_7 = 0xd4;
//LAB_1008_a2b1:
        param_1 = 0x1010;
        *param_6 = 0x1;
        break;
        0x7 =>
//LAB_1008_a277:
        param_1 = 0x1010;
        load_string_1010_84e0(0x1010, uVar13, u_var5, 0x3ff, (uVar8 + 0xe), in_buf_len_5);
        break;
        0x9 => if ((uVar8 + 0x416) == 0x0) {
        (uVar8 + 0x416) = 0x1;
        break;
    }
//     TODO: goto LAB_1008_a35a;
        0xb => if ((uVar8 + 0x41a) == 0x0) {
        (uVar8 + 0x41a) = 0x1;
        break;
    }
//     TODO: goto LAB_1008_a35a;
        0xe => if ((uVar8 + 0x41c) == 0x0) {
        (uVar8 + 0x41c) = 0x1;
        break;
    }
//     TODO: goto LAB_1008_a35a;
        0x14 => if ((uVar8 + 0x418) == 0x0) {
        (uVar8 + 0x418) = 0x1;
        load_string_1010_84e0(0x1010, _PTR_LOOP_1050_14cc,
                              (ctx.PTR_LOOP_1050_14cc >> 0x10), 0x3ff, (uVar8 + 0xe),
                              in_buf_len_5);
        pcVar12 = load_string_1010_847e(ctx.PTR_LOOP_1050_14cc,
                                        (ctx.PTR_LOOP_1050_14cc >> 0x10), 0x1010);
        // puVar7 = (pcVar12 >> 0x10);
        string_1000_3cea((param_4 & 0xffff0000 | ZEXT24((uVar8 + 0xe))) as i32, pcVar12);
        *param_7 = 0x4c;
        uVar14 = 0x1;
        uVar15 = 0x0;
        i_var9 = 0xa;
        puVar11 = mixed_1010_20ba(ctx.PTR_LOOP_1050_0ed0, 0x2b, param_2, puVar7, in_buf_len_5);
        param_1 = 0x1010;
        pass1_1010_089e(param_2, puVar11, CONCAT11(uVar15, uVar14), i_var9);
        break;
    }
//     TODO: goto LAB_1008_a35a;
        0x16 => param_1 = 0x1010;
        load_string_1010_84e0(0x1010, uVar13, u_var5, 0x3ff, (uVar8 + 0xe), in_buf_len_5);
        *param_7 = 0x28;
        break;
        0x17 => param_1 = 0x1010;
        load_string_1010_84e0(0x1010, uVar13, u_var5, 0x3ff, (uVar8 + 0xe), in_buf_len_5);
        *param_7 = 0x2c;
        break;
        0x18 => param_1 = 0x1010;
        load_string_1010_84e0(0x1010, uVar13, u_var5, 0x3ff, (uVar8 + 0xe), in_buf_len_5);
        *param_7 = 0x2e;
        break;
        0x1b => param_1 = 0x1010;
        load_string_1010_84e0(0x1010, uVar13, u_var5, 0x3ff, (uVar8 + 0xe), in_buf_len_5);
        *param_7 = 0x30;
        break;
        0x1c => param_1 = 0x1010;
        load_string_1010_84e0(0x1010, uVar13, u_var5, 0x3ff, (uVar8 + 0xe), in_buf_len_5);
        *param_7 = 0x32;
        break;
        0x1f => param_1 = 0x1010;
        load_string_1010_84e0(0x1010, uVar13, u_var5, 0x3ff, (uVar8 + 0xe), in_buf_len_5);
        *param_7 = 0x34;
        break;
        0x21 => if ((uVar8 + 0x41e) == 0x0) {
        (uVar8 + 0x41e) = 0x1;
        break;
    }
//LAB_1008_a35a:
        *param_5 = 0x0;
        break;
        0x24 => param_1 = 0x1010;
        load_string_1010_84e0(0x1010, uVar13, u_var5, 0x3ff, (uVar8 + 0xe), in_buf_len_5);
        *param_7 = 0x2a;
        break;
        0x31 => param_1 = 0x1010;
        load_string_1010_84e0(0x1010, uVar13, u_var5, 0x3ff, (uVar8 + 0xe), in_buf_len_5);
        *param_7 = 0x27;
        break;
        0x32 => param_1 = 0x1010;
        load_string_1010_84e0(0x1010, uVar13, u_var5, 0x3ff, (uVar8 + 0xe), in_buf_len_5);
        *param_7 = 0x29;
        break;
        0x33 => param_1 = 0x1010;
        load_string_1010_84e0(0x1010, uVar13, u_var5, 0x3ff, (uVar8 + 0xe), in_buf_len_5);
        *param_7 = 0x2b;
        break;
        0x34 => param_1 = 0x1010;
        load_string_1010_84e0(0x1010, uVar13, u_var5, 0x3ff, (uVar8 + 0xe), in_buf_len_5);
        *param_7 = 0x2d;
        break;
        0x35 => param_1 = 0x1010;
        load_string_1010_84e0(0x1010, uVar13, u_var5, 0x3ff, (uVar8 + 0xe), in_buf_len_5);
        *param_7 = 0x2f;
        break;
        0x36 => param_1 = 0x1010;
        load_string_1010_84e0(0x1010, uVar13, u_var5, 0x3ff, (uVar8 + 0xe), in_buf_len_5);
        *param_7 = 0x31;
        break;
        0x37 => load_string_1010_84e0(0x1010, uVar13, u_var5, 0x3ff, (uVar8 + 0xe), in_buf_len_5);
        pcVar12 = load_string_1010_847e(ctx.PTR_LOOP_1050_14cc,
                                        (ctx.PTR_LOOP_1050_14cc >> 0x10), 0x1010);
        param_1 = 0x1000;
        string_1000_3cea((param_4 & 0xffff0000 | ZEXT24((uVar8 + 0xe))) as i32, pcVar12);
        *param_7 = 0x33;
        break;
        0x38 => param_1 = 0x1010;
        load_string_1010_84e0(0x1010, uVar13, u_var5, 0x3ff, (uVar8 + 0xe), in_buf_len_5);
        *param_7 = 0x35;
        break;
        0x39 => param_1 = 0x1010;
        load_string_1010_84e0(0x1010, uVar13, u_var5, 0x3ff, (uVar8 + 0xe), in_buf_len_5);
        *param_7 = 0x36;
        break;
        0x3a => param_1 = 0x1010;
        load_string_1010_84e0(0x1010, uVar13, u_var5, 0x3ff, (uVar8 + 0xe), in_buf_len_5);
        *param_7 = 0x37;
        break;
        0x3b => param_1 = 0x1010;
        load_string_1010_84e0(0x1010, uVar13, u_var5, 0x3ff, (uVar8 + 0xe), in_buf_len_5);
        *param_7 = 0x38;
        break;
        0x3c => param_1 = 0x1010;
        load_string_1010_84e0(0x1010, uVar13, u_var5, 0x3ff, (uVar8 + 0xe), in_buf_len_5);
        *param_7 = 0x39;
        break;
        0x3d => param_1 = 0x1010;
        load_string_1010_84e0(0x1010, uVar13, u_var5, 0x3ff, (uVar8 + 0xe), in_buf_len_5);
        *param_7 = 0xce;
        break;
        0x3e => param_1 = 0x1010;
        load_string_1010_84e0(0x1010, uVar13, u_var5, 0x3ff, (uVar8 + 0xe), in_buf_len_5);
        *param_7 = 0xcf;
        break;
        0x3f => param_1 = 0x1010;
        load_string_1010_84e0(0x1010, uVar13, u_var5, 0x3ff, (uVar8 + 0xe), in_buf_len_5);
        *param_7 = 0xd0;
        break;
        0x40 => param_1 = 0x1010;
        load_string_1010_84e0(0x1010, uVar13, u_var5, 0x3ff, (uVar8 + 0xe), in_buf_len_5);
        *param_7 = 0xd1;
        break;
        0x41 => param_1 = 0x1010;
        load_string_1010_84e0(0x1010, uVar13, u_var5, 0x3ff, (uVar8 + 0xe), in_buf_len_5);
        *param_7 = 0xd2;
        break;
        0x42 => param_1 = 0x1010;
        load_string_1010_84e0(0x1010, uVar13, u_var5, 0x3ff, (uVar8 + 0xe), in_buf_len_5);
        *param_7 = 0xd3;
        break;
        0x43 => param_1 = 0x1010;
        load_string_1010_84e0(0x1010, uVar13, u_var5, 0x3ff, (uVar8 + 0xe), in_buf_len_5);
        *param_7 = 0xd5;
        break;
        0x44 => param_1 = 0x1010;
        load_string_1010_84e0(0x1010, uVar13, u_var5, 0x3ff, (uVar8 + 0xe), in_buf_len_5);
        *param_7 = 0xd6;
        break;
        0x45 => param_1 = 0x1010;
        load_string_1010_84e0(0x1010, uVar13, u_var5, 0x3ff, (uVar8 + 0xe), in_buf_len_5);
        *param_7 = 0xd7;
    }
    // switchD_1008_a835_caseD_3:
    if (puStack6 != 0x0) {
    ppcVar2 = *puStack6;
    (**ppcVar2)(param_1, puStack6, (puStack6 >> 0x10), 0x1);
}
    return;
}



u32
pass1_1008_a8f4(param_1: u32,param_2: * mut u16,param_3: * mut u16,param_4: * mut u16,
param_5: u16,param_6: u16,param_7: u16,param_8: u8)

{
let i_var1: i16; let local_6: u32;

i_var1 = & local_6 + 0x2; pass1_1008_a1f0(param_6, param_7, param_8, param_1, param_2,
CONCAT22(param_7, & local_6), CONCAT22(param_7, i_var1),
param_4); pass1_1008_944e(param_3, local_6, (local_6 > > 0x10)); return CONCAT22(param_5, i_var1);
}



pub fn pass1_1008_a930(param_1: u32, param_2: i16, param_3: u16) {
    let u_var1: u32;
    let ppcVar2: u32;
    let pu_var3: U32Ptr;
    let extraout_dx: u16;
    let u_var4: u16;
    let iVar5: i16;
    let u_var6: u16;
    let puStack24: U32Ptr;
    let puStack18: U32Ptr;
    let local_a: [u8; 8];

    if (param_2 == 0x0) {
        return;
    }
    // u_var6 = (param_1 >> 0x10);
    iVar5 = param_1 as i16;
    pass1_1008_5784(CONCAT22(param_3, local_a), ((iVar5 + 0x410) as u32));
    loop {
        pu_var3 = local_a;
        pass1_1008_5b12(pu_var3, param_3);
        u_var4 = extraout_dx | pu_var3;
        if (u_var4 == 0x0) {
            mem_op_1000_179c(0x6, 0x0, 0x1000);
            puStack24 = CONCAT22(u_var4, pu_var3);
            if ((u_var4 | pu_var3) == 0x0) {
                puStack18 = 0x0;
            } else {
                *puStack24 = 0x389a;
                (pu_var3 + 0x2) = 0x1008;
                (pu_var3 + 0x4) = param_2;
                *puStack24 = 0xad8a;
                (pu_var3 + 0x2) = 0x1008;
                puStack18 = puStack24;
            }
            u_var1 = (iVar5 + 0x410) as u32;
            ppcVar2 = ((iVar5 + 0x410) + 0x8) as u32;
            (**ppcVar2)(0x1000, u_var1, (u_var1 >> 0x10), puStack18,
                        (puStack18 >> 0x10));
            return;
        }
        if ((pu_var3 + 0x4) != param_2) == false { break; }
    }
    return;
}


pub fn pass1_1008_a9ec(param_1: u32) -> u16

{
    let u_var1: u32;
    let in_AX: u16;
    let i_var2: i16;
    let u_var3: u16;
    WNDCLASS16 * unaff_SS;
    let uStack4: u16;

    uStack4 = 0x0;
    // u_var3 = (param_1 >> 0x10);
    i_var2 = param_1 as i16;
    if (((i_var2 + 0x414) == 0x0) && (u_var1 = (i_var2 + 0x410) as u32, (u_var1 + 0x8) != 0x0)) {
        (i_var2 + 0x414) = 0x1;
        pass1_1008_aa28(param_1 & 0xffff | u_var3 << 0x10, in_AX, unaff_SS);
        uStack4 = in_AX;
    }
    return uStack4;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1008_aa28(param_1: u32, param_2: u16, param_3: &WNDCLASS16) {
    let ppcVar1: u32;
    let u_var2: u32;
    let extraout_dx: u16;
    let i_var3: i16;
    let u_var4: u16;
    let puStack6: u32;

    // u_var4 = (param_1 >> 0x10);
    i_var3 = param_1 as i16;
    if ((i_var3 + 0x414) != 0x0) {
        u_var2 = (i_var3 + 0x410) as u32;
        if ((u_var2 + 0x8) == 0x0) {
            (i_var3 + 0x414) = 0x0;
            return;
        }
        ppcVar1 = ((i_var3 + 0x410) + 0x10) as u32;
        (**ppcVar1)();
        puStack6 = CONCAT22(extraout_dx, param_2);
        if ((extraout_dx | param_2) != 0x0) {
            win_1008_5c5c(param_3, param_2, extraout_dx | param_2, _PTR_LOOP_1050_02a0,
                          ((param_2 + 0x4) as u32));
            if (puStack6 != 0x0) {
                ppcVar1 = *puStack6;
                (**ppcVar1)();
            }
            return;
        }
    }
    return;
}


pub fn pass1_1008_aaa8(param_1: u16, param_2: u16, param_3: u16) -> u16

{
    let uStack4: u16;

    uStack4 = 0x0;
    switch(param_3)
    {
        0x1 => uStack4 = 0x24;
        break;
        0x2 => uStack4 = 0x16;
        break;
        0x3 => uStack4 = 0x17;
        break;
        0x4 => uStack4 = 0x18;
        break;
        0x5 => uStack4 = 0x1b;
        break;
        0x6 => uStack4 = 0x1c;
        break;
        0x7 => uStack4 = 0x1f;
    }
    return uStack4;
}


pub fn pass1_1008_ab12(param_1: u16, param_2: u16, param_3: u16) -> u16

{
    if (param_3 == 0x37) {
        return 0x22;
    }
    if (param_3 < 0x38) {
        if (param_3 == '\r') {
            return 0xf;
        }
        if (param_3 == '*') {
            return 0x2b;
        }
    }
    return 0x0;
}


pub fn pass1_1008_ab54(param_1: u32) -> u16

{
    let u_var1: u32;
    let u_var2: u16;
    let uStack4: u16;

    uStack4 = 0x0;
    // u_var2 = (param_1 >> 0x10);
    if (((param_1 + 0xa) != 0x0) && (u_var1 = (param_1 + 0xa), (u_var1 + 0x8) != 0x0)) {
        uStack4 = 0x1;
    }
    return uStack4;
}


pub fn pass1_1008_ab80(param_1: u16, param_2: u16, param_3: u16) -> u16

{
    let uStack4: u16;

    uStack4 = 0x0;
    if (true) {
        switch(param_3)
        {
            0x8 => uStack4 = 0x82;
            break;
            0x9 => uStack4 = 0x7f;
            break;
            0xa => uStack4 = 0x80;
            break;
            0xb => uStack4 = 0x84;
            break;
            0xc => uStack4 = 0x89;
            break;
            0xd => uStack4 = 0x8a;
            break;
            0xe => uStack4 = 0x8c;
            break;
            0xf => uStack4 = 0x8e;
            break;
            0x10 => uStack4 = 0x8f;
            break;
            0x11 => uStack4 = 0x90;
            break;
            0x12 => uStack4 = 0x91;
            break;
            0x13 => uStack4 = 0x95;
            break;
            0x14 => uStack4 = 0x96;
            break;
            0x16 => uStack4 = 0x9b;
            break;
            0x17 => uStack4 = 0x9f;
            break;
            0x18 => uStack4 = 0xa2;
            break;
            0x19 => uStack4 = 0xa4;
            break;
            0x1b => 0x1c => uStack4 = 0xa7;
            break;
            0x1d => uStack4 = 0xaa;
            break;
            0x1e => uStack4 = 0xac;
            break;
            0x1f => uStack4 = 0xad;
            break;
            0x20 => uStack4 = 0xae;
            break;
            0x21 => uStack4 = 0xb1;
            break;
            0x22 => uStack4 = 0xb3;
            break;
            0x23 => uStack4 = 0xb4;
            break;
            0x24 => uStack4 = 0xb5;
            break;
            0x25 => uStack4 = 0xb6;
            break;
            0x26 => uStack4 = 0xb7;
            break;
            0x27 => uStack4 = 0xab;
            break;
            0x28 => uStack4 = 0xb9;
            break;
            0x29 => uStack4 = 0xba;
            break;
            0x2a => uStack4 = 0xbc;
            break;
            0x2b => uStack4 = 0xbe;
            break;
            0x2c => uStack4 = 0xdf;
            break;
            0x2d => uStack4 = 0xe0;
        }
    }
    return uStack4;
}


pub fn pass1_1008_ad0c(param_1: U32Ptr, param_2: u8) -> u16

{
    let u_var1: u16;

    // u_var1 = (param_1 >> 0x10);
    *param_1 = 0x389a;
    (param_1)[0x1] = 0x1008;
    if ((param_2 & 0x1) != 0x0) {
        pass1_1000_093a(param_1, u_var1, 0x1000);
    }
    return param_1;
}


pub fn pass1_1008_ad38(param_1: U32Ptr, param_2: u8) -> u16

{
    *param_1 = 0x389a;
    (param_1 + 0x2) = 0x1008;
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return param_1;
}


pub fn pass1_1008_ad64(param_1: u32, param_2: u8) -> u32

{
    let unaff_SS: u16;

    pass1_1008_a086(param_1, unaff_SS);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return param_1;
}


pub fn pass1_1008_ada2(param_1: U32Ptr, param_2: i16) -> u16

{
    let u_var1: u16;

    // u_var1 = (param_1 >> 0x10);
    *param_1 = 0x0;
    (param_1 + 0x2) = 0x0;
    (param_1 + 0x4) = param_2;
    *param_1 = (param_2 * 0x6 + 0x3a4) as u16;
    return param_1;
}


pub fn pass1_1008_add2(param_1: U32Ptr) {
    *param_1 = ((param_1 + 0x4) * 0x6 + 0x3a4);
    return;
}


pub fn pass1_1008_adf2(param_1: u32) -> u16

{
    return ((param_1 + 0x4) * 0x6 + 0x3a4) as u16;
}


pub fn pass1_1008_ae0c(param_1: u32) -> u16

{
    return ((param_1 + 0x4) * 0x6 + 0x3a6) as u16;
}


pub fn pass1_1008_ae26(param_1: &mut i16) {
    let pi_var1: U32Ptr;
    let i_var2: i16;
    let i_var3: i16;
    let u_var4: u16;

    // u_var4 = (param_1 >> 0x10);
    i_var3 = param_1;
    i_var2 = ((i_var3 + 0x4) * 0x6 + 0x3a8);
    if (i_var2 == 0x2) {
        if ((i_var3 + 0x2) == 0x1) {
            *param_1 = *param_1 + -0x1;
            i_var2 = (i_var3 + 0x4) * 0x6;
            pi_var1 = (i_var2 + 0x3a4);
            if (*pi_var1 != *param_1 && *param_1 <= *pi_var1) {
                *param_1 = (i_var2 + 0x3a4) + 0x1;
                (i_var3 + 0x2) = 0x0;
                return;
            }
        } else {
            *param_1 = *param_1 + 0x1;
            i_var2 = (i_var3 + 0x4) * 0x6;
            if ((i_var2 + 0x3a6) < *param_1) {
                *param_1 = (i_var2 + 0x3a6) + -0x1;
                (i_var3 + 0x2) = 0x1;
                return;
            }
        }
    } else {
        if ((i_var2 != 0x3) && (i_var2 != 0x4)) {
            *param_1 = *param_1 + 0x1;
            i_var2 = (i_var3 + 0x4) * 0x6;
            if ((i_var2 + 0x3a6) < *param_1) {
                *param_1 = (i_var2 + 0x3a4);
            }
        }
    }
    return;
}


pub fn pass1_1008_aed8(param_1: u32) -> bool

{
    if (((param_1 + 0x4) * 0x6 + 0x3a4) != 0x0) {
        return 0x1;
    }
    return 0x0;
}



u32
pass1_1008_aefe(param_1: * mut u8,param_2: * mut u8,param_3: u16,param_4: * mut u8,param_5: u16
)

{
struct_op_1018_4cda(param_1, param_2, param_3); CONCAT22(param_2, param_1) = 0xaf7c; (param_1 + 0x2) = 0x1008; ctx.PTR_LOOP_1050_4230 = param_1; ctx.PTR_LOOP_1050_4232 = param_2; pass1_1018_4dce(CONCAT22(param_2, param_1), 0x1b3, param_4, param_5); return CONCAT22(param_2, param_1);
}



pub fn pass1_1008_af38(param_1: &mut Struct11) {
    param_1 = 0xaf7c;
    (param_1 + 0x2) = 0x1008;
    clenaup_win_ui_1018_4d22(param_1, 0x1018);
    return;
}


pub fn pass1_1008_af56(param_1: u32, param_2: u8) -> u32

{
    pass1_1008_af38(param_1);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return param_1;
}


pub fn pass1_1008_af94(param_1: &mut Struct643, param_2: u16, param_3: u16) {
    struct_op_1010_1d48(CONCAT22(param_2, param_1), param_3);
    param_1.field_0xa = 0x0;
    param_1.field_0xe = 0x0;
    param_1.field_0x12 = 0x0;
    param_1.field_0x16 = 0x0;
    param_1.field_0x1a = 0x0;
    param_1.field_0x1e = 0x0;
    param_1.field_0x22 = 0x0;
    CONCAT22(param_2, param_1) = 0xbdcc;
    param_1.field_0x2 = 0x1008;
    return;
}


pub fn pass1_1008_afde(param_1: U32Ptr, param_2: u16) {
    let pu_var1: u32;
    let u_var2: u16;
    let ppc_var3: u32;
    let i_var4: &mut Struct468;
    let u_var4: u16;

    // u_var4 = (param_1 >> 0x10);
    i_var4 = param_1;
    *param_1 = 0xbdcc;
    i_var4.field_0x2 = 0x1008;
    pu_var1 = i_var4.field_0xa;
    u_var2 = i_var4.field_0xc;
    if ((u_var2 | pu_var1) != 0x0) {
        ppc_var3 = *pu_var1;
        (**ppc_var3)();
    }
    pu_var1 = i_var4.field_0xe;
    u_var2 = i_var4.field_0x10;
    if ((u_var2 | pu_var1) != 0x0) {
        ppc_var3 = *pu_var1;
        (**ppc_var3)();
    }
    pu_var1 = i_var4.field_0x12;
    u_var2 = i_var4.field_0x14;
    if ((u_var2 | pu_var1) != 0x0) {
        ppc_var3 = *pu_var1;
        (**ppc_var3)();
    }
    pass1_1010_1d80(param_1, param_2);
    return;
}


pub fn pass1_1008_b05a(param_1: U32Ptr) -> u16

{
    let i_var1: &mut Struct193;
    let u_var1: u16;

    // u_var1 = (param_1 >> 0x10);
    i_var1 = param_1;
    *param_1 = 0x389a;
    i_var1.field_0x2 = 0x1008;
    i_var1.field_0x4 = 0x0;
    *param_1 = 0xbdc8;
    i_var1.field_0x2 = 0x1008;
    return param_1;
}


pub fn pass1_1008_b08c(param_1: &mut Struct18) {
    let i_var1: i16;
    let u_var2: u16;

    // u_var2 = (param_1 >> 0x10);
    i_var1 = param_1;
    *param_1 = 0xbdc8;
    (i_var1 + 0x2) = 0x1008;
    fn_ptr_1000_17ce(ctx, (i_var1 + 0x4), 0x1000);
    *param_1 = 0x389a;
    (i_var1 + 0x2) = 0x1008;
    return;
}


pub fn set_stuct_1008_b0bc(param_1: &mut Struct26) {
    let i_var1: &mut Struct26;
    let u_var1: u16;

    pass1_1008_b05a(param_1);
    // u_var1 = (param_1 >> 0x10);
    i_var1 = param_1;
    i_var1.field_0x8 = 0x0;
    i_var1.field_0xa = 0x0;
    i_var1.field_0xe = 0x0;
    i_var1.field_0x10 = 0x0;
    param_1 = 0xbdc4;
    i_var1.field_0x2 = 0x1008;
    return;
}


pub fn pass1_1008_b0f2(param_1: U32Ptr) -> u16

{
    let u_var1: u16;

    pass1_1008_b05a(param_1);
    // u_var1 = (param_1 >> 0x10);
    (param_1 + 0x8) = 0x0;
    *param_1 = 0xbdc0;
    (param_1 + 0x2) = 0x1008;
    return param_1;
}


pub fn pass1_1008_b11e(param_1: U32Ptr) -> u16

{
    let u_var1: u16;

    pass1_1008_b05a(param_1);
    // u_var1 = (param_1 >> 0x10);
    (param_1 + 0x8) = 0x0;
    *param_1 = 0xbddc;
    (param_1 + 0x2) = 0x1008;
    return param_1;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1008_b146(param_1: u32, param_2: u16, param_3: u16) {
    let u_var1: u32;
    let i_var2: i16;
    let u_var3: u16;

    // u_var3 = (param_1 >> 0x10);
    i_var2 = param_1 as i16;
    if ((i_var2 + 0x16) != 0x0) {
        u_var1 = (i_var2 + 0x16) as u32;
        pass1_1030_8344(
            ctx, ctx.PTR_LOOP_1050_5748,
            (u_var1 + 0xa));
        pass1_1038_3608(CONCAT22(param_3, param_2));
        u_var1 = (i_var2 + 0x16) as u32;
        (u_var1 + 0x8) = 0x0;
        u_var1 = (i_var2 + 0x16) as u32;
        (u_var1 + 0xa) = 0x0;
        u_var1 = (i_var2 + 0x16) as u32;
        (u_var1 + 0xe) = 0x0;
        u_var1 = (i_var2 + 0x16) as u32;
        (u_var1 + 0x10) = 0x0;
    }
    return;
}


pub fn pass1_1008_b1a6(param_1: u32, param_2: &mut String) {
    let lVar1: i32;
    let u_var2: u16;
    let in_DX: u16;
    let i_var3: &mut Struct467;
    let i_var4: &mut Struct466;
    let u_var3: u16;
    let u_var4: u16;

    // u_var3 = (param_1 >> 0x10);
    i_var3 = param_1;
    if (i_var3.field_0x16 != 0x0) {
        lVar1 = i_var3.field_0x16;
        fn_ptr_1000_17ce(ctx, (lVar1 + 0x4), 0x1000);
        u_var2 = str_op_1008_60e8(param_2, in_DX);
        lVar1 = i_var3.field_0x16;
        // u_var4 = (lVar1 >> 0x10);
        i_var4 = lVar1;
        i_var4.field_0x4 = u_var2;
        i_var4.field_0x6 = in_DX;
        i_var3.field_0x16 = 0x0;
    }
    return;
}
