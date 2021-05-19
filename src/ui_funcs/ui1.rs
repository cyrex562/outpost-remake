use crate::app_context::AppContext;
use crate::exit::fatal_app_exit_1000_3e9e;
use crate::string_funcs::{copy_string_1000_3d3e, process_string_1000_3cea, get_string_index_1000_3da4, fn_1008_6048, process_string_1000_4d58, string_fn_1000_3f9c, process_string_1000_3dbe};
use crate::sys_funcs::{dos3_call_1000_51aa, process_win_msg_1008_9510, post_win_msg_1040_7b3c, reg_class_1040_98c0, win_cleanup_func_1040_b0f8, create_win_1008_9760, reg_class_1008_96d2, make_proc_inst_1038_cf6c, make_proc_inst_1040_8fb8, send_win_msg_1008_84ba, send_msg_1010_7c42, win_func_1018_6bb6, get_sys_metrics_1020_7c1a, load_rsrc_1010_4e9e, reg_class_1008_818c, dos3_call_1000_4f2e, send_win_msg_1020_097e};
use crate::{ui_funcs, mixed_fn_1010_830a};
use crate::ui_funcs::ui2;
use crate::util::{CONCAT22, SBORROW2, ZEXT24, SUB42, SUB41, CONCAT11, CONCAT12, CONCAT24, CONCAT26, CONCAT28, CONCAT210, CONCAT212, CONCAT214, CONCAT66, CONCAT13, SUB21, CONCAT31, CARRY1};
use crate::winapi_funcs::{LoadString16, MessageBeep16, MessageBox16, PostMessage16, WritePrivateProfileString16, DestroyWindow16, InvalidateRect16, GetClientRect16, GetWindowLong16, TrackPopupMenu16, ClientToScreen16, GetSubMenu16, LoadMenu16, WinHelp16, IsWindow16, MoveWindow16, GetSystemMetrics16, GetWindowRect16, EnableWindow16, GetDlgItem16, SetWindowPos16, SendMessage16, GetProp16, CreateWindow16, LoadIcon16, DeleteObject16, SelectPalette16, SetTextColor16, SetBkColor16, SelectObject16, DrawText16, DrawIcon16, BeginPaint16, FreeProcInstance16, RemoveProp16, ShowWindow16, UpdateWindow16, PtInRect16, GetStockObject16, LoadCursor16, ReleaseCapture16, SetCapture16, SetFocus16, GetDlgCtrlID16, MakeProcInstance16, SetCursor16, IsIconic16, DestroyIcon16, DestroyMenu16, GetVersion16, GetWindowWord16, DeleteDC16, SetDlgItemText16, GetDC16, EnableMenuItem16, SetWindowText16, BringWindowToTop16, swi, Rectangle16, CreateDC16, EnumChildWindows16, ReleaseDC16, FillRect16, CreateSolidBrush16, RegisterClass16, GetClassInfo16};
use crate::pass::pass8_funcs::{pass1_1010_2ee2, pass1_1010_32c0, pass1_1010_043a, pass1_1010_7e40, pass1_1010_2024, pass1_1010_038e, pass1_1010_7818, pass1_1010_1d80, pass1_1010_40cc, pass1_1010_459e, pass1_1010_451a, pass1_1010_41d6, pass1_1010_1dda, pass1_1010_3c52, pass1_1010_3cd0, pass1_1010_3c9e, pass1_1010_1ea6, pass1_1010_4f20, pass1_1010_878c, pass1_1010_1f62, pass1_1010_715c, pass1_1010_6ca2, pass1_1010_01f8, pass1_1010_7d38, pass1_1010_60cc, pass1_1010_5f4c, pass1_1010_5f1e, process_struct_1010_20ba};
use crate::cleanup::{destroy_win_1010_3202, window_msg_func_1010_7300};
use crate::typedefs::{HWND16, HANDLE16, DLGPROC16, LRESULT, COLORREF, HPALETTE16, WPARAM16, HGDIOBJ16, HCURSOR16, HINSTANCE16, HDC16, SEGPTR, HFILE16, ATOM};
use crate::prog_structs::prog_structs_30::{Struct124, Struct119, Struct347};
use std::intrinsics::offset;
use crate::prog_structs::prog_structs_31::{Struct32, Struct20, Struct27, Struct47, Struct17, Struct382, Struct16};
use crate::prog_structs::prog_structs_24::{Struct6, Struct103, Struct2111};
use crate::struct_funcs::{process_struct_1000_179c, process_struct_1040_7728, process_struct_1040_9824, process_struct_1008_3ab8, process_struct_1008_4c58, process_struct_1008_48fe, process_struct_1020_0baa, process_struct_1020_04f6, struct_fn_1000_160a, process_struct_1008_4772, process_struct_1040_bf3e, process_struct_1010_1d48, process_struct_1008_4834, process_struct_1008_47cc};
use crate::prog_structs::prog_structs_10::{Struct73, Struct62};
use crate::prog_structs::prog_structs_29::{Struct362, Struct41, Struct179, Struct48, Struct123};
use crate::prog_structs::prog_structs_2::{Struct199, Struct7, Struct318};
use crate::pass::pass13_funcs::{pass1_1008_ab54, pass1_1008_aa28, pass1_1008_9628};
use crate::draw::draw1::{get_sys_metrics_1040_8c66, realize_palette_1008_4e08, draw_1040_9948, process_struct_1040_9252, draw_1008_8288, draw_fn_1018_e4f2, get_gui_dc_1018_4db0, pt_in_rect_1010_40f8, select_and_delete_palette_1020_92c4, draw_fn_1018_ec74, load_cursor_1020_7f7a};
use crate::sound_funcs::{mci_send_command_1008_5c7c, mci_fn_1020_08b6, mci_send_command_1008_5cb6, sound_fn_1008_53ae, mci_send_cmd_1008_5c5c};
use crate::prog_structs::prog_structs_28::{Struct40, Struct351, Struct447};
use crate::pass::pass14_funcs::{pass1_1008_4d72, pass1_fn_1008_60e8, pass1_1008_6d8a, pass1_1008_6978, pass1_1008_57c4, pass1_1008_405c, pass1_1008_4d84, pass1_1008_3f92, pass1_1008_3e0e, pass1_1008_3e94, pass1_1008_6562, pass1_1008_5b12, pass1_1008_5784, pass1_1008_3e76, pass1_1008_687a, pass1_1008_6a04};
use crate::sys_structs::{PAINTSTRUCT16, RECT16, BITMAPINFO};
use crate::prog_structs::prog_structs_26::{Struct59, Struct340, Struct339};
use crate::err_funcs::{error_check_1000_17ce, set_error_mode_1010_8b14};
use crate::prog_structs::prog_structs_23::{Struct356, win_struct_42, Struct43, Struct387};
use crate::prog_structs::prog_structs_7::{Struct44, Struct376, Struct629, Struct628};
use crate::pass::pass10_funcs::{pass1_1040_b040, pass1_1040_bfde};
use crate::file_funcs::file1::{close_file_1008_6dd0, close_file_1008_6e78, write_to_file_1008_6e02, close_file_1008_496c, read_file_1008_7dee, write_to_file_1008_7e1c};
use crate::pass::pass7_funcs::{pass1_1018_179e, pass1_1018_50ea, pass1_1018_57d2, pass1_1018_6198, pass1_1018_4dce, pass1_1018_4cda, pass1_1018_5206, pass1_1018_526a};
use crate::pass::pass17_funcs::{pass1_1030_5b00, pass1_1030_8344, pass1_1030_73a8, pass1_1030_6e14, pass1_1030_6ddc, pass1_1030_8326};
use crate::func_ptr_funcs::call_fn_ptr_1000_24cd;
use crate::prog_structs::prog_structs_27::pass1_struct_2;
use crate::pass::pass_funcs::{pass1_1008_392e, pass1_fn_1000_484c, pass1_1000_4906, pass1_fn_1000_5008};
use crate::other_funcs::{set_timer_1008_91ba, zero_array_val_1008_5394, zero_list_1008_3e38, switch_statement_1008_73ea};
use crate::prog_structs::prog_structs_18::Struct180;
use crate::list_funcs::{modify_u16_list_1008_5bdc, clear_list_elements_1008_80d2};
use crate::prog_structs::prog_structs_25::{Struct219, Struct63, Struct71, Struct65, Struct67};
use crate::pass::pass6_funcs::pass1_1038_b6e0;
use crate::pass::pass20_funcs::{pass1_1010_91cc, pass1_1010_9172, pass1_1010_b028, pass1_1010_ad22, pass1_1010_dd5e, pass1_1010_c320, pass1_1010_c3c2, pass1_1018_04b8, pass1_1010_ad64, pass1_1010_a58a, pass1_1010_a5ca, pass1_1010_ac62, pass1_1010_a5ac, pass1_1010_a5ec, pass1_1010_a568, pass1_1010_9210, pass1_1010_9130, pass1_1010_9044};
use crate::pass::pass4_funcs::{pass1_1028_e4ec, pass1_1028_dc52};
use crate::prog_structs::prog_structs_6::Struct622;
use crate::pass::pass19_funcs::{pass1_1018_e834, pass1_1018_e100, pass1_1020_022c, pass1_1040_a5d0};
use crate::prog_structs::prog_structs_9::{Struct636, Struct637, Struct594, Struct604};
use crate::pass::pass15_funcs::pass1_1020_bae6;
use crate::prog_structs::prog_structs_8::Struct60;
use crate::mem_funcs::alloc_mem_1000_1708;
use crate::prog_structs::prog_structs_1::Struct104;
use crate::prog_structs::prog_structs_16::Struct588;
use crate::prog_structs::prog_structs_12::Struct94;
use crate::prog_structs::prog_structs_17::Struct61;
use crate::prog_structs::prog_structs_21::Struct12;
use crate::big_funcs::big_fn_1008_15d4;

// pub fn win_fn_1008_1414(param_1: u32,param_2: u32)

// {
//     let mut u_var1: u32;
//     let ppc_var2: fn();
//     let BVar3: bool;
//     let mut u_var4: u16;
//     let mut u_var5: u16;
//     let ppVar6: *mut pass1_struct_2;
//     let mut i_var7: i32;
//
//
//     let mut u_var8: u16;
//
//     let mut unaff_DI: u16;
//     let mut unaff_ss: u16;
//     let pp_var9: *mut pass1_struct_1;
//     let mut u_var10: u16;
//     let u_var11: u8;
//     let u_var12: u8;
//     let mut local_50: u32;
//     let mut local_4c: u16;
//     let mut local_4a: u16;
//     let mut local_3c: u16;
//     let mut local_3a: u16;
//     let mut local_38: u16;
//     let mut local_36: u16;
//     let mut local_34: u16;
//     let mut local_32: u16;
//     let mut local_30: u16;
//     let mut local_2e: u16;
//     let mut local_2c: u16;
//     let mut local_2a: u32;
//     let mut uStack38: u16;
//     let mut local_24: u16;
//     let mut local_22: u16;
//     let mut local_20: u32;
//     let mut local_1c: u16;
//     let mut local_1a: u16;
//     let mut local_18: u32;
//     let mut local_14: u16;
//     let mut local_12: u16;
//     let mut local_10: u32;
//     let mut local_c: u16;
//     let mut local_a: u16;
//     let mut local_8: [u8;6];

//     pass1_1008_6d8a(CONCAT22(unaff_ss, local_8), param_2);
//     BVar3 = close_file_1008_6e78(CONCAT22(unaff_ss, local_8));
//     i_var7 = param_1;
//     u_var10 = (param_1 >> 0x10);
//     if (BVar3 == 0)
//     {
//         if (ctx.g_u16_1050_0310 == 0)
//         {
//             ctx.g_u16_1050_0310 = 0x6d4;
//         }
//         u_var4 = ctx.g_u16_1050_0310;
//         load_string_1010_847e(ctx._g_struct_73_1050_14cc, (ctx._g_struct_73_1050_14cc >> 0x10),
//                               ctx.g_u16_1050_0310);
//         u_var8 = ctx.dx_reg;
//         pass1_fn_1008_60e8(u_var4, ctx.dx_reg);
//         u_var5 = u_var4;
//         load_string_1010_847e(ctx._g_struct_73_1050_14cc, (ctx._g_struct_73_1050_14cc >> 0x10), 0x57b);
//         MessageBeep16(0x10);
//         MessageBox16(0x10, CONCAT22(ctx.dx_reg, u_var5), CONCAT22(u_var8, u_var4),
//                      (i_var7 + 8));
//         error_check_1000_17ce(CONCAT22(u_var8, u_var4));
//         call_fn_ptr_1000_24cd(1);
//     }
//     set_cursor_1008_2dcc(i_var7, u_var10, 8);
//     _local_c = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(unaff_DI, 0x2f));
//     u_var8 = (_local_c >> 0x10);
//     local_10 = (_local_c + 0x20);
//     ppVar6 = pass1_1030_8344(ctx._g_bool_1050_5748, (ctx._g_bool_1050_5748 >> 0x10),
//                              local_10);
//     _local_14 = CONCAT22(u_var8, ppVar6);
//     local_18 = &ppVar6.field_0x10;
//     u_var1 = (i_var7 + 0xe8);
//     ppc_var2 = ((i_var7 + 0xe8) + 4);
//     ppc_var2(0x1030, u_var1, (u_var1 >> 0x10), local_10, (local_10 >> 0x10),
//                 (local_18 + 2) + -1, 2);
//     local_22 = ctx.dx_reg;
//     ppVar6 = pass1_1030_8344(ctx._g_bool_1050_5748, (ctx._g_bool_1050_5748 >> 0x10),
//                              0x4000001);
//     _local_1c = CONCAT22(local_22, ppVar6);
//     local_20 = &ppVar6.field_0x10;
//     local_24 = pass1_1030_8344(ctx._g_bool_1050_5748,
//                                     (ctx._g_bool_1050_5748 >> 0x10), local_20);
//     local_2a = (local_24 + 0xc);
//     uStack38 = (local_24 + 0x10);
//     i_var7 = pass1_1030_5b00(_local_14);
//     u_var11 = SUB21(&local_2a, 0);
//     u_var12 = (&local_2a >> 8);
//     u_var10 = unaff_ss;
//     pp_var9 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT13(u_var12, CONCAT12(u_var11, i_var7)));
//     pass1_1018_179e(pp_var9, CONCAT22(u_var10, CONCAT11(u_var12, u_var11)));
//     u_var11 = 0;
//     u_var12 = 4;
//     u_var5 = 0x1b;
//     u_var10 = 1;
//     pp_var9 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, 0x1002b);
//     pass1_1010_043a(pp_var9, CONCAT13(u_var12, CONCAT12(u_var11, u_var10)), u_var5);
//     close_file_1008_6dd0(CONCAT22(unaff_ss, local_8));
//     return;
// }

// WARNING: Variable defined which should be unmapped: local_8

// pub fn set_focus_1038_c558(param_1: *mut Struct20)

// {
//     let mut u_var1: u16;

//     win_gui_func_1040_78e2(param_1);
//     move_window_1040_826c(param_1, 0xffffffff);
//     u_var1 = (param_1 >> 0x10);
//     ShowWindow16(5, (param_1 + 6));
//     SetFocus16((param_1 + 6));
//     return;
// }


pub unsafe fn msg_box_1000_1f24(ctx: &mut AppContext,
                                param_1: &String,
                                param_2: &String) -> bool {
    let pi_var1: *mut i32;

    if ctx.ax_reg < (param_1 + 0xc) {
        msg_box_1000_214c(0, 0, 0xd940, &mut ctx.PTR_LOOP_1050_1040);
        return true;
    }
    pi_var1 = (param_1 + 0xc);
    *pi_var1 = *pi_var1 + 1;
    return false;
}

// WARNING: Removing unreachable block (ram,0x10001f92)
pub fn out_of_mem_msg_box_1000_1f7e(param_1: &String) -> u16 {
    // let mut u_var1: i32;
    let mut u_var1: char;
    // let mut c_var2: u8;
    let mut c_var2: char;
    // let mut u_var3: u16;
    let mut u_var3: u16;
    // let mut u_var4: u32;
    let mut u_var4: String;

    u_var1 = param_1[0];
    if u_var1 == 0xf as char {
        // LAB_1000_1fb6:
        u_var3 = 1;
    } else {
        if u_var1 < '\x10' {
            c_var2 = u_var1;
            if c_var2 == 0x2 as char {}
            // goto LAB_1000_1fb6;
            if (0 < (c_var2 + -2)) && ((c_var2 + -3) < 0xc) {
                u_var3 = 0;
                // goto LAB_1000_1fbe;
            }
        }
        u_var3 = 0;
    }
    // LAB_1000_1fbe:
    u_var4 = out_of_mem_msg_1000_1fd2();
    u_var3 = msg_box_1000_214c(0, u_var3, &u_var4, 0);
    return u_var3;
}

// : String out_of_mem_msg_1000_1fd2()
pub fn out_of_mem_msg_1000_1fd2() -> String {
    // let mut in_ax: i32;
    let mut in_ax: i32;

    if in_ax == 2 {
        return "Out of memory.  Please free some memory, then choose retry.".to_string();
    }
    return CONCAT22(0x1000, (s_242_flc_1050_1c76 + 4) + in_ax * 0x17);
}

pub fn msg_box_1000_214c(param_1: u16, param_4: u16, param_2: &String) -> u16 {
    let i_var1: u16;
    let mut i_var2: i32;
    let mut _type: u16 = 2 - (param_1 == 0) | 0x2110;
    MessageBeep16(0);
    loop {
        // i_var1 = MessageBox16(_type, &String::from("SmartHeap Library"), param_2, 0);
        i_var1 = MessageBox16(0, param_2, &String::from("SmartHeap Library"), _type);
        i_var2 = i_var1 - 1;
        if i_var2 == 0 {
            return 0;
        }
        if (0 < i_var2) && (!SBORROW2(i_var2, 1)) {
            if i_var1 == 3 || i_var1 + -2 < 1 {
                fatal_app_exit_1000_3e9e();
                return 0;
            }
            if (i_var1 == 4) {
                return 1;
            }
            if (i_var1 == 5) {
                return 0;
            }
        }
        if ((_type & 0x2000) == 0) {
            return 0;
        }
        _type = _type & 0xdfef | 0x1010;
    }
}

pub fn gui_fn_1008_2c4e(param_1: *mut Struct16, param_2: u16, param_3: u16) {
    let pi_var1: *mut i32;
    let mut u_var2: u32;
    let ppc_var3: fn();
    let mut cursor: u16;
    let h_cursor: HCURSOR16;
    let mut u_var4: u16;
    let mut u_var5: u32;
    let mut local_6: u16;

    cursor = LoadCursor16(0x7f02, 0);
    h_cursor = SetCursor16(cursor);
    pi_var1 = &param_1.field_0xf2;
    unsafe {
        *pi_var1 = *pi_var1 + 1;
    }
    if (&param_1.field_0xee != 0) {
        u_var5 = &param_1.field_0xee;
        ppc_var3 = (&param_1.field_0xee + 0x90);
        (**ppc_var3)(offset, u_var5, (u_var5 >> 0x10));
    }
    u_var5 = big_fn_1008_15d4(CONCAT22(param_2, param_1), param_3);
    u_var4 = (u_var5 >> 0x10);
    &param_1.field_0xee = u_var5;
    param_1.field_0xf0 = u_var4;
    ppc_var3 = (&param_1.field_0xee + 8);
    (**ppc_var3)(offset, &param_1.field_0xee, u_var4);
    if (param_1.field_0xe8 != 0) {
        u_var2 = param_1.field_0xe8;
        ppc_var3 = (param_1.field_0xe8 + 0xc);
        (**ppc_var3)(offset, u_var2, (u_var2 >> 0x10), 0);
    }
    ui2::show_window_1038_b634(ctx._g_Struct112_a, (ctx._g_Struct112_a >> 0x10));
    u_var2 = param_1.field_0xf4;
    show_window_1010_7a76(u_var2, (u_var2 >> 0x10));
    u_var5 = &param_1.field_0xee;
    ppc_var3 = (&param_1.field_0xee + 0xc);
    (**ppc_var3)(0x1010, u_var5, (u_var5 >> 0x10), 5);
    u_var5 = &param_1.field_0xee;
    BringWindowToTop16((u_var5 + 8));
    SetCursor16(h_cursor);
    return;
}

pub fn set_cursor_1008_2dcc(param_1: *mut Struct16, param_2: u16, param_3: u16) {
    let mut u_var1: u32;
    let ppc_var2: fn();
    let HVar3: HCURSOR16;
    let h_cursor: HCURSOR16;

    let mut local_6: u16;

    HVar3 = LoadCursor16(0x7f02, 0);
    h_cursor = SetCursor16(HVar3);
    HVar3 = h_cursor;
    if (param_1.field_0xe8 != 0) {
        u_var1 = param_1.field_0xe8;
        ppc_var2 = (param_1.field_0xe8 + 0x90);
        ppc_var2(offset, u_var1, (u_var1 >> 0x10));
    }
    big_fn_1008_15d4(CONCAT22(param_2_00, param_1), param_2);
    &param_1.field_0xe8 = HVar3;
    (&param_1.field_0xe8 + 2) = ctx.dx_reg;
    u_var1 = param_1.field_0xe8;
    if ((u_var1 + 0xe0) == 0) {
        u_var1 = param_1.field_0xe8;
        ppc_var2 = (param_1.field_0xe8 + 8);
        ppc_var2(offset, u_var1, (u_var1 >> 0x10));
        u_var1 = param_1.field_0xe8;
        ppc_var2 = (param_1.field_0xe8 + 0xc);
        ppc_var2(offset, u_var1, (u_var1 >> 0x10), 3);
        &param_1.field_0xce = param_1.field_0xe8;
    } else {
        param_1.field_0xe8 = 0;
        gui_fn_1008_2c4e(param_1, param_2_00, param_2);
        &param_1.field_0xce = 0;
    }
    SetCursor16(h_cursor);
    return;
}

pub fn set_cursor_1008_2e9a(param_1: u32) {
    let mut u_var1: u32;
    let mut u_var2: u16;
    let mut u_var3: i32;

    let mut i_var4: i32;
    let mut unaff_si: u16;
    let mut unaff_ss: u16;
    let u_var5: u8;
    let u_var6: u8;
    let mut u_var7: u16;
    let mut in_stack_0000fdd2: u8;
    let mut local_224: [u8; 264];
    let mut local_11c: u16;
    let mut local_11a: u32;
    let mut local_112: u16;
    let mut local_110: u16;
    let mut local_10e: u32;
    let mut local_10a: u32;
    let mut local_106: u16;
    let mut local_104: u16;
    let local_102: u8;
    let local_101: u8;

    local_102 = '\0';
    _local_106 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(unaff_si, 2));
    u_var2 = (_local_106 >> 0x10);
    i_var4 = _local_106;
    local_10a = (i_var4 + 0x16);
    u_var3 = (i_var4 + 0x18) | local_10a;
    u_var5 = unaff_ss;
    u_var6 = (unaff_ss >> 8);
    u_var7 = (param_1 >> 0x10);
    if (u_var3 == 0) {
        open_save_fn_1008_3178(param_1, u_var7, 1);
        local_10a = CONCAT22(ctx.dx_reg, u_var3);
        if ((ctx.dx_reg | u_var3) == 0) {
            PostMessage16(0, 0x13d, 0x111, ctx.g_h_window);
            return;
        }
        copy_string_1000_3d3e(
            CONCAT13(u_var6, CONCAT12(u_var5, &local_102)),
            CONCAT22(ctx.dx_reg, u_var3),
        );
        process_string_1000_4d58(&local_102);
        if (in_stack_0000fdd2 != '\0') {
            process_string_1000_3cea(
                CONCAT13(u_var6, CONCAT12(u_var5, local_224)),
                CONCAT22(unaff_ss, &stack0xfdd2),
            );
        }
        pass1_1010_5f1e(_local_106, CONCAT22(unaff_ss, local_224));
    } else {
        local_11a = *(i_var4 + 0x1a);
        copy_string_1000_3d3e(CONCAT13(u_var6, CONCAT12(u_var5, &local_102)), local_11a);
        local_11c = get_string_index_1000_3da4(CONCAT22(unaff_ss, &local_102));
        if (*(&local_104 + local_11c + 1) != '\\') {
            (&local_102)[local_11c] = '\\';
            (&local_101)[local_11c] = '\0';
        }
        process_string_1000_3cea(CONCAT13(u_var6, CONCAT12(u_var5, &local_102)), local_10a);
    }
    if (local_102 != '\0') {
        i_var4 = param_1;
        local_10e = (i_var4 + 0xe8);
        send_win_msg_1020_097e(local_10e, (local_10e >> 0x10));
        u_var1 = (i_var4 + 0xe8);
        UpdateWindow16((u_var1 + 8));
        local_110 = LoadCursor16(0x7f02, 0);
        local_112 = SetCursor16(local_110);
        win_fn_1008_1414(i_var4, u_var7);
        SetCursor16(local_112);
    }
    return;
}

pub fn open_save_1008_30cc(param_1: u32) {
    let mut ctx.stack_seg_reg: i32;
    let in_string_2: String;
    let mut local_DXAX_125: u32;
    let mut local_218: i32;
    let mut local_216: i32;
    let mut local_214: u16;
    let mut local_212: u16;
    let mut string_1: [u8; 10];
    let mut string_2: [u8; 256];
    let mut local_106: u16;
    let mut local_104: u16;
    let mut string_3: [u8; 256];

    string_3[0] = '\0';
    in_string_2 = open_save_fn_1008_3178(param_1, 2);
    if (in_string_2 != 0x0) {
        copy_string_1000_3d3e(CONCAT22(ctx.stack_seg_reg, string_3), in_string_2);
        process_string_1000_4d58(string_3);
        if (string_1[0] != '\0') {
            process_string_1000_3cea(
                CONCAT22(ctx.stack_seg_reg, string_2),
                CONCAT22(ctx.stack_seg_reg, string_1),
            );
        }
        local_DXAX_125 =
            process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(string_2, 2));
        pass1_1010_5f4c(local_DXAX_125, CONCAT22(ctx.stack_seg_reg, string_2));
        if (string_3[0] != '\0') {
            win_fn_1008_12dc(param_1, string_3, ctx.stack_seg_reg);
        }
    }
    return;
}

pub fn open_save_fn_1008_3178(param_1: u32, param_2: i32) {
    let mut u_var1: u32;
    let p_uvar2: *mut u16;
    let pu_var3: Vec<u8>;
    let mut i_var4: i32;
    let pu_var5: *mut u32;
    let mut u_var6: u32;



    let mut u_var7: u16;


    let mut u_var8: u16;
    let mut extraout_dx_04: u16;
    let mut extraout_dx_05: u16;
    let mut unaff_si: u16;
    let mut u_var9: u16;
    let mut unaff_ss: u16;
    let mut u_var10: i32;
    let mut local_786: u32;
    let mut local_782: [u8; 260];
    let mut local_67e: u16;
    let mut local_676: u16;
    let mut local_674: u16;
    let mut local_672: u16;
    let mut local_670: u16;
    let mut local_66e: u32;
    let local_666: u8;
    let mut local_566: u32;
    let mut local_562: u32;
    let mut local_55e: u16;
    let mut local_55a: u16;
    let mut local_558: u16;
    let mut local_54a: u16;
    let mut local_548: u16;
    let mut local_546: u32;
    let mut local_542: u16;
    let mut local_540: u16;
    let mut local_53e: u32;
    let mut local_53a: u16;
    let mut local_538: u16;
    let mut local_536: u16;
    let mut local_534: u16;
    let mut local_532: u32;
    let mut local_52e: u16;
    let mut local_52a: u16;
    let mut local_528: u16;
    let local_51a: u8;
    let local_519: u8;
    let local_518: u8;
    let mut local_418: u16;
    let local_416: u8;
    let mut local_415: [u8; 7];
    let mut local_40e: u16;
    let mut local_40c: [u8; 258];
    let mut local_30a: u32;
    let mut local_306: u16;
    let mut local_304: u16;
    let local_302: u8;
    let local_202: u8;
    let local_103: u8;
    let local_102: u8;

    local_102 = '\0';
    local_302 = '\0';
    local_202 = '\0';
    _local_306 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(unaff_si, 2));
    u_var9 = (_local_306 >> 0x10);
    i_var4 = _local_306;
    local_30a = (i_var4 + 0x1a);
    u_var10 = (i_var4 + 0x1c);
    if ((u_var10 | local_30a) == 0) {
        local_66e = (i_var4 + 100);
        u_var10 = (i_var4 + 0x66);
        if ((u_var10 | local_66e) != 0) {
            pass1_1008_5784(
                CONCAT22(unaff_ss, &local_67e),
                local_66e & 0xffff | u_var10 << 0x10,
            );
            pu_var2 = &local_67e;
            pass1_1008_5b12(CONCAT22(unaff_ss, pu_var2));
            _local_676 = CONCAT22(ctx.dx_reg, pu_var2);
            if ((ctx.dx_reg | pu_var2) != 0) {
                u_var1 = (pu_var2 + 2);
                local_30a._0_2_ = u_var1;
                u_var10 = (u_var1 >> 0x10);
                // goto LAB_1008_3206;
            }
        }
    } else {
        // LAB_1008_3206:
        copy_string_1000_3d3e(CONCAT22(unaff_ss, &local_102), CONCAT22(u_var10, local_30a));
    }
    pass1_fn_1000_5008(local_40c);
    local_40e = get_string_index_1000_3da4(CONCAT22(unaff_ss, local_40c));
    if (local_40c[local_40e - 1] == '\\') {
        local_40c[local_40e - 1] = 0;
    }
    local_40e = get_string_index_1000_3da4(CONCAT22(unaff_ss, &local_102));
    if ((&local_103)[local_40e] == '\\') {
        (&local_103)[local_40e] = '\0';
    }
    dos3_call_1000_4f2e(&local_102);
    u_var9 = (_local_306 >> 0x10);
    local_30a = (_local_306 + 0x12);
    u_var10 = (_local_306 + 0x14);
    pu_var3 = (u_var10 | local_30a);
    if (pu_var3 != 0x0) {
        pu_var3 = &local_202;
        copy_string_1000_3d3e(
            CONCAT22(unaff_ss, pu_var3),
            (local_30a & 0xffff | u_var10 << 0x10),
        );
    }
    local_416 = '\0';
    load_string_1010_847e(
        ctx._g_struct_73_1050_14cc,
        (ctx._g_struct_73_1050_14cc >> 0x10),
        0x579,
    );
    copy_string_1000_3d3e(
        CONCAT22(unaff_ss, &local_416),
        CONCAT22(ctx.dx_reg, pu_var3),
    );
    local_418 = get_string_index_1000_3da4(CONCAT22(unaff_ss, &local_416));
    u_var6 = local_418;
    local_40e = local_418;
    while (u_var9 = u_var6, -1 < local_418) {
        if ((&local_416)[local_418] == '.') {
            copy_string_1000_3d3e(
                CONCAT22(unaff_ss, &local_67e),
                CONCAT22(unaff_ss, local_415 + local_418),
            );
            u_var6 = ZEXT24(&local_416);
            copy_string_1000_3d3e(
                CONCAT22(unaff_ss, &local_416),
                CONCAT22(unaff_ss, &local_67e),
            );
        }
        local_418 = local_418 - 1;
    }
    local_518 = '\0';
    load_string_1010_847e(
        ctx._g_struct_73_1050_14cc,
        (ctx._g_struct_73_1050_14cc >> 0x10),
        0x74c,
    );
    u_var7 = copy_string_1000_3d3e(
        CONCAT22(unaff_ss, &local_518),
        CONCAT22(ctx.dx_reg, u_var9),
    );
    local_40e = get_string_index_1000_3da4(CONCAT22(unaff_ss, &local_518));
    local_51a = (&local_519)[local_40e];
    local_418 = 0;
    while ((&local_518)[local_418] != '\0') {
        if ((&local_518)[local_418] == local_51a) {
            (&local_518)[local_418] = '\0';
        }
        local_418 = local_418 + 1;
    }
    pass1_1000_4906(CONCAT22(unaff_ss, &local_562), 0, 0x48);
    local_562 = 0x48;
    u_var9 = (param_1 >> 0x10);
    local_55e = (param_1 + 8);
    local_55a = &local_518;
    _local_54a = CONCAT22(unaff_ss, &local_202);
    local_542 = &local_302;
    local_546 = 0x100;
    local_53e = 0x100;
    local_53a = &local_102;
    local_52a = &local_416;
    local_566 = 0;
    local_666 = '\0';
    i_var4 = param_2 + -1;
    u_var8 = (ctx._g_struct_73_1050_14cc >> 0x10);
    if (i_var4 == 0) {
        local_532 = 0x1804;
        load_string_1010_847e(ctx._g_struct_73_1050_14cc, u_var8, 0x74d);
        copy_string_1000_3d3e(
            CONCAT22(unaff_ss, &local_666),
            CONCAT22(ctx.dx_reg, i_var4),
        );
        local_536 = &local_666;
        pu_var5 = &local_562;
        GetOpenFileName16(0x1000, pu_var5);
    } else {
        param_2 = param_2 + -2;
        if (param_2 != 0) {
            fn_1008_6048(
                s_Unsupported_FileStructType_in_Op_1050_01ca,
                u_var7,
                SUB21(param_2, 0),
            );
            // goto LAB_1008_3461;
        }
        local_532 = 6;
        load_string_1010_847e(ctx._g_struct_73_1050_14cc, u_var8, 0x74e);
        copy_string_1000_3d3e(
            CONCAT22(unaff_ss, &local_666),
            CONCAT22(extraout_dx_05, param_2),
        );
        local_536 = &local_666;
        pu_var5 = &local_562;
        GetSaveFileName16(0x1000, pu_var5);
    }
    if (pu_var5 != 0x0) {
        local_566 = _local_54a;
    }
    // LAB_1008_3461:
    if (local_566 != 0) {
        local_67e = local_52e;
        if (local_52e < 0) {
            load_string_1010_847e(
                ctx._g_struct_73_1050_14cc,
                (ctx._g_struct_73_1050_14cc >> 0x10),
                0x3fd,
            );
            _local_676 = CONCAT22(ctx.dx_reg, local_52e);
            u_var8 = ctx.dx_reg;
            pass1_fn_1008_60e8(local_52e, ctx.dx_reg);
            _local_676 = CONCAT22(u_var8, local_52e);
            load_string_1010_847e(
                ctx._g_struct_73_1050_14cc,
                (ctx._g_struct_73_1050_14cc >> 0x10),
                0x57b,
            );
            local_670 = extraout_dx_04;
            MessageBox16(
                0x10,
                CONCAT13((extraout_dx_04 >> 8), CONCAT12(extraout_dx_04, local_52e)),
                _local_676,
                (param_1 + 8),
            );
            local_566 = 0;
            local_66e = _local_676;
            error_check_1000_17ce(_local_676);
        } else {
            process_string_1000_3dbe(
                CONCAT13((unaff_ss >> 8), CONCAT12(unaff_ss, local_782)),
                _local_54a,
                local_52e,
            );
            local_782[local_52e] = '\0';
            if (local_782[0] != '\0') {
                pass1_1010_60cc(_local_306, CONCAT22(unaff_ss, local_782));
            }
        }
    }
    dos3_call_1000_4f2e(local_40c);
    return;
}

pub fn sys_color_func_1008_357e(param_1: u32, param_2: i32) {
    let mut u_var1: u32;
    let mut u_var2: u16;
    let mut u_var3: u16;
    let local_bx_221: *mut Struct67;
    let mut i_var4: i32;
    let mut u_var5: u16;
    let mut unaff_ss: u16;
    let CVar6: COLORREF;
    let mut color_ref: u32;
    let puStack140: *mut u32;
    let mut local_84: u16;
    let mut local_82: u16;
    let mut local_80: u32;
    let mut local_7c: u16;
    let mut local_7a: u16;
    let mut local_78: u16;
    let mut local_76: u16;
    let mut local_74: u16;
    let mut local_72: u16;
    let mut local_70: u32;
    let mut local_6c: u32;
    let mut local_68: u16;
    let mut local_66: u16;
    let mut local_64: u16;
    let mut local_62: u16;
    let mut local_60: u16;
    let mut local_5e: u16;
    let mut local_5c: u16;
    let mut local_5a: u16;
    let mut local_58: u32;
    let mut local_54: u32;
    let mut local_50: u16;
    let mut local_4e: u16;
    let mut local_4c: u32;
    let mut local_48: u32;
    let mut local_44: u32;
    let mut local_40: u32;
    let mut local_3c: u32;
    let mut local_38: u32;
    let mut local_34: u32;
    let mut local_30: u32;
    let mut count: u32;
    let mut local_28: u32;
    let mut local_24: u32;
    let mut local_20: u32;
    let mut local_1c: u32;
    let mut local_18: u32;
    let mut local_14: u32;
    let mut local_10: u32;
    let mut local_c: u32;
    let mut local_8: u32;
    let mut local_4: u16;

    count = 0x70004;
    local_28 = 0xf0000;
    local_24 = 0x100014;
    local_20 = 0xd0012;
    local_1c = 0x6000e;
    local_18 = 0x80005;
    local_14 = 0x10011;
    local_10 = 0x30002;
    local_c = 0xa0009;
    local_8 = 0xc000b;
    local_4 = 0x13;
    local_80 = 0;
    local_6c = 0x808080;
    u_var3 = 0x100;
    local_74 = 0;
    local_72 = 0x100;
    local_64 = 0;
    local_62 = 0x100;
    local_60 = 0xffff;
    local_5e = 0;
    local_7c = 2;
    local_7a = 0x100;
    local_78 = 2;
    local_76 = 0x100;
    local_68 = 2;
    local_66 = 0x100;
    local_5c = 2;
    local_5a = 0x100;
    local_58 = 0;
    local_50 = 0xc0c0;
    local_4e = 0;
    local_4c = 0;
    local_48 = 0;
    local_44 = 0;
    local_34 = 0;
    u_var2 = 0x8000;
    local_70 = 0x8000;
    local_54 = 0x8000;
    local_40 = 0x8000;
    local_3c = 0x8000;
    local_38 = 0x8000;
    local_30 = 0x8000;
    u_var5 = (param_1 >> 0x10);
    local_bx_221 = param_1;
    if (&local_bx_221.field_0xf8 == 0) {
        process_struct_1000_179c(0x54, (s_You_may_not_run_a_turn__The_game_1050_00df + 0x21));
        local_bx_221.field_0xf8 = u_var2;
        &local_bx_221.field_0xfa = u_var3;
        local_84 = 0;
        while (local_84 < 0x15) {
            CVar6 = GetSysColor16(&count + local_84 * 2);
            u_var1 = &local_bx_221.field_0xf8;
            u_var3 = (u_var1 >> 0x10);
            i_var4 = u_var1;
            (i_var4 + local_84 * 4) = CVar6;
            (i_var4 + local_84 * 4 + 2) = (CVar6 >> 0x10);
            local_84 = local_84 + 1;
        }
    }
    if (param_2 != 0) {
        CVar6 = GetSysColor16(count);
        if ((CVar6 == local_80) && ((CVar6 >> 0x10) == local_80._2_2_)) {
            return;
        }
    }
    if (ctx.PTR_LOOP_1050_0010 == 0x0) {
        local_70 = 0xc0c0c0;
    }
    if (param_2 == 0) {
        color_ref = &local_bx_221.field_0xf8;
    } else {
        color_ref = CONCAT22(unaff_ss, &local_80);
    }
    puStack140 = &count;
    SetSysColors16(color_ref, color_ref._2_2_, puStack140);
    return;
}

pub fn fill_client_window_1008_39ac(in_struct_12: *mut Struct12) {
    let local_bx_4: *mut Struct12;
    let unaff_ss: HWND16;
    let mut in_stack_00000006: u16;
    let HStack44: HWND16;
    let mut brush: u16;
    let mut b_result: u16;
    let paint_struct: [u8; 32];

    b_result = BeginPaint(CONCAT22(unaff_ss, paint_struct), in_struct_12.h_window);
    brush = CreateSolidBrush16(0x210070b);
    GetClientRect16(CONCAT22(unaff_ss, &stack0xffd2), in_struct_12.h_window);
    HStack44 = b_result;
    FillRect16(brush, &stack0xffd2, unaff_ss);
    HStack44 = in_struct_12.h_window;
    EndPaint(paint_struct, unaff_ss);
    DeleteObject16(brush);
    return;
}

pub fn set_di_bits_to_dev_1008_45d6(param_1: *mut Struct103) {
    let mut bVar1: bool;
    let mut i_var2: i32;
    let mut u_var3: u16;
    let mut local_6: u16;

    u_var3 = (param_1 >> 0x10);
    i_var2 = param_1;
    if ((i_var2 + 6) == 0) {
        process_struct_1008_47cc(param_1);
    }
    if (((i_var2 + 8) | (i_var2 + 6)) == 0) {
        bVar1 = false;
    } else {
        if (((i_var2 + 0xc) | (i_var2 + 10)) == 0) {
            process_struct_1008_4834(param_1);
        }
        bVar1 = true;
    }
    if (!bVar1) {
        return;
    }
    SetDIBitsToDevice();
    return;
}

pub fn stretch_di_bits_1008_465a(param_1: *mut Struct103, param_2: HDC16) {
    let x_src: u16;
    let y_src: u16;
    let mut u_var1: u32;
    let mut b_var2: bool;
    let mut i_var3: i32;
    let info: *mut BITMAPINFO;
    let mut u_var4: u16;
    let bits: &mut Vec<u8>;
    let mut local_4: u16;

    u_var4 = (param_1 >> 0x10);
    i_var3 = param_1;
    if ((i_var3 + 6) == 0) {
        process_struct_1008_47cc(param_1);
    }
    if (((i_var3 + 8) | (i_var3 + 6)) == 0) {
        b_var2 = false;
    } else {
        if (((i_var3 + 0xc) | (i_var3 + 10)) == 0) {
            process_struct_1008_4834(param_1);
        }
        b_var2 = true;
    }
    if (!b_var2) {
        return;
    }
    u_var1 = (i_var3 + 0x10);
    bits = (u_var1 >> 0x10);
    info = u_var1;
    x_src = &(info.bmi_header).bi_width;
    y_src = &(info.bmi_header).bi_height;
    u_var1 = (i_var3 + 0x14);
    StretchDIBits16(
        0xcc0020,
        0,
        info,
        bits,
        u_var1,
        (u_var1 >> 0x10),
        y_src,
        x_src,
        0,
        0,
        y_src,
        x_src,
        param_2,
    );
    return;
}

pub fn set_win_1008_5634(
    param_1: *mut u32,
    param_2: u16,
    param_3_00: WPARAM16,
    param_3: u16,
    param_4: i32,
) {
    let pp_var1: fn();

    let lVar2: u32;
    let mut u_var3: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    u_var3 = SUB42(&ctx.g_alloc_addr_1050_1050, 0);
    lVar2 = param_4 << 0x10;
    _local_6 = GetWindowLong16(0, param_4);
    if (((_local_6 >> 0x10) | _local_6) == 0) {
        if (param_3 != 1) {
            DefWindowProc16(CONCAT22(param_2, param_1), param_3_00, param_3, param_4);
            return;
        }
        unsafe {
            _local_6 = *param_1;
        }
        lVar2 = param_4 << 0x10;
        SetWindowLong16(_local_6, (_local_6 >> 0x10));
        pass1_1008_9628(_local_6, param_4);
    }
    pp_var1 = (*_local_6 + 0x1c);
    (**pp_var1)(
        offset,
        _local_6,
        (_local_6 >> 0x10),
        param_1,
        param_2,
        param_3_00,
        param_3,
        lVar2,
        u_var3,
    );
    return;
}

pub fn cleanup_palette_1008_56e2(param_1: u32, param_2: *mut HDC16) -> u16 {
    let h_gdi_obj: HPALETTE16;
    let mut u_var1: u16;
    u_var1 = (param_1 >> 0x10);
    h_gdi_obj = SelectPalette16(0, (param_1 + 4), unsafe { *param_2 });
    (param_1 + 4) = h_gdi_obj;
    DeleteObject16(h_gdi_obj);
    return 1;
}

pub fn create_win_1008_5e7e() -> u16 {
    let pu_var1: *mut u32;
    let pu_var2: *mut u32;
    let BVar3: bool;
    let AVar4: ATOM;
    let mut window_handle: u16;
    let mut i_var5: i32;
    let mut local_ESI_9: string;
    let pu_var6: *mut u32;
    let unaff_ss: HINSTANCE16;
    let mut local_2e: u16;
    let mut local_2c: u16;
    let mut local_2a: u16;
    let mut local_28: u16;
    let mut local_26: u16;
    let mut local_24: u16;
    let mut local_22: u16;
    let mut local_20: u16;
    let mut local_1e: u16;
    let mut stock_obj: u16;
    let mut local_1a: u32;
    let mut local_16: u16;
    let mut local_14: u16;
    let local_12: [u32; 4];

    pu_var6 = local_12;
    local_ESI_9._0_2_ = s_MciSoundWindow_1050_02bd;
    i_var5 = 3;
    while (i_var5 != 0) {
        i_var5 = i_var5 + -1;
        pu_var2 = pu_var6;
        pu_var6 = pu_var6 + 1;
        pu_var1 = local_ESI_9;
        local_ESI_9._0_2_ = local_ESI_9 + 1;
        unsafe {
            *pu_var2 = *pu_var1;
        }
    }
    pu_var6 = local_ESI_9;
    *(pu_var6 + 2) = *(local_ESI_9 + 2);
    local_2c = 0x2000;
    local_2a = &u16_1050_5f44;
    local_28 = &ctx.PTR_LOOP_1050_1008;
    local_24 = 2;
    local_22 = ctx.g_h_instance_1050_038c;
    local_20 = 0;
    local_1e = 0;
    local_26 = 0;
    stock_obj = GetStockObject16(0);
    local_1a = 0;
    local_16 = local_12;
    BVar3 = GetClassInfo16(&local_2c, CONCAT22(local_16, unaff_ss), unaff_ss);
    if (BVar3 == 0) {
        AVar4 = RegisterClass16(&local_2c);
        if (AVar4 == 0) {
            OutputDebugString16(s_MciSound_registerClass_failed_1050_02cc);
            return 0;
        }
    }
    window_handle = CreateWindow16(
        0,
        ctx.g_h_instance_1050_038c,
        0,
        ctx.g_h_window,
        1,
        1,
        -0x8000,
        -0x8000,
        0xcf0000,
        s_MciSound_registerClass_failed_1050_02cc + 0x1e,
        CONCAT22(unaff_ss, local_12),
    );
    return window_handle;
}

pub fn load_cursor_1008_61b2(
    in_Struct65_ptr: *mut Struct65,
    in_u16_2: u16,
    param_3: u16,
    param_4: &mut Vec<u8>,
) -> *mut Struct65 {
    let mut u_var1: u16;
    let HVar2: HGDIOBJ16;
    let HVar3: HCURSOR16;
    let local_struct_65_ptr_1: *mut Struct65;
    let mut u_var4: u16;
    let ppVar5: *mut pass1_struct_1;
    let mut in_stack_0000ffe8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    pass1_1008_687a(in_Struct65_ptr, param_4);
    u_var4 = (in_Struct65_ptr >> 0x10);
    local_struct_65_ptr_1 = in_Struct65_ptr;
    local_struct_65_ptr_1.u16_xde = in_u16_2;
    local_struct_65_ptr_1.u16_xe0 = 0;
    in_Struct65_ptr.ptr_a_lo = 0x6378;
    local_struct_65_ptr_1.ptr_a_hi = &ctx.PTR_LOOP_1050_1008;
    copy_string_1000_3d3e(
        (in_Struct65_ptr & 0xffff0000 | ZEXT24(&local_struct_65_ptr_1.char_ptr_x5b)),
        s_DanBrotherton_1050_0302,
    );
    HVar2 = GetStockObject16(4);
    local_struct_65_ptr_1.h_gdi_obj_xc6 = HVar2;
    HVar3 = LoadCursor16(0x7f00, 0);
    local_struct_65_ptr_1.h_cursor_xc4 = HVar3;
    local_struct_65_ptr_1.u16_xc8 = (s_572_bmp_1050_2007 + 4);
    local_struct_65_ptr_1.u32_xac = 0x45000000;
    local_struct_65_ptr_1.Struct590_ptr_xbc = (param_4 + 8);
    ppVar5 = process_struct_1010_20ba(
        ctx._g_Struct372_1050_0ed0,
        CONCAT22(in_stack_0000ffe8, 0x48),
    );
    u_var1 = (ppVar5 >> 0x10);
    local_struct_65_ptr_1.u16_xb4 = 0;
    local_struct_65_ptr_1.u16_xb6 = 0;
    local_struct_65_ptr_1.u16_xb8 = (ppVar5 + 10);
    local_struct_65_ptr_1.u16_xba = (ppVar5 + 0xc);
    &local_struct_65_ptr_1.u32_xca = param_3;
    reg_class_1008_96d2(in_Struct65_ptr, in_stack_0000ffe8);
    return in_Struct65_ptr;
}

pub fn destroy_win_1008_628e(param_1: *mut Struct594, param_2: u16) {
    let pp_var1: fn();

    pp_var1 = ((param_1 + 0xd2) + 0x14);
    (**pp_var1)();
    DestroyWindow16((param_1 + 8));
    (param_1 + 8) = 0;
    return;
}

pub fn fill_rect_1008_62c0(param_1: u32) {
    let mut u_var1: u16;
    let unaff_ss: HWND16;
    let local_2e: RECT16;
    let mut local_26: u16;
    let mut local_24: u16;
    let local_22: PAINTSTRUCT16;

    u_var1 = (param_1 >> 0x10);
    local_24 = BeginPaint16(CONCAT22(unaff_ss, &local_22), (param_1 + 8));
    local_26 = CreateSolidBrush16(0x210070b);
    GetClientRect16(CONCAT22(unaff_ss, &local_2e), (param_1 + 8));
    FillRect16(local_26, &local_2e, unaff_ss);
    EndPaint(&local_22, unaff_ss);
    DeleteObject16(local_26);
    return;
}

pub fn show_window_1008_68c6(param_1: *mut Struct628, param_2: u16, param_3: u16) -> u16 {
    let mut local_AX_13: u16;
    let mut local_4: u16;

    local_AX_13 = show_window_1008_96ae(CONCAT22(param_2, param_1), param_3);
    pass1_1008_6a04(param_1, param_2);
    return local_AX_13;
}

pub fn load_cursor_1008_7f62(param_1: *mut Struct65, param_2: u16, param_3: u32) -> *mut Struct65 {
    let HVar1: HGDIOBJ16;
    let HVar2: HCURSOR16;
    let mut i_var3: i32;
    let mut u_var4: u16;
    let mut in_stack_0000fffc: u16;

    pass1_1008_687a(param_1, param_3);
    u_var4 = (param_1 >> 0x10);
    i_var3 = param_1;
    (i_var3 + 0xde) = param_2;
    param_1.ptr_a_lo = 0x8042;
    (i_var3 + 2) = &ctx.PTR_LOOP_1050_1008;
    copy_string_1000_3d3e(
        (param_1 & 0xffff0000 | (i_var3 + 0x5b)),
        s_SOLChildPar_1050_0358,
    );
    HVar1 = GetStockObject16(5);
    (i_var3 + 0xc6) = HVar1;
    HVar2 = LoadCursor16(0x7f00, 0);
    (i_var3 + 0xc4) = HVar2;
    (i_var3 + 200) = (s_572_bmp_1050_2007 + 1);
    (i_var3 + 0xac) = 0x44000000;
    (i_var3 + 0xbc) = (param_3 + 8);
    (i_var3 + 0xca) = (i_var3 + 0xde);
    reg_class_1008_96d2(param_1, in_stack_0000fffc);
    return param_1;
}

pub fn load_cursor_1008_80ee(param_1: *mut u16) -> *mut u16 {
    let HVar1: HCURSOR16;
    let HVar2: HGDIOBJ16;
    let mut i_var3: i32;
    let mut u_var4: u16;

    u_var4 = (param_1 >> 0x10);
    i_var3 = param_1;
    unsafe {
        *param_1 = ctx.s_1_1050_389a;
    }
    (i_var3 + 2) = &ctx.PTR_LOOP_1050_1008;
    (i_var3 + 0x54) = 0;
    (i_var3 + 0x56) = 0;
    (i_var3 + 0x58) = 0;
    unsafe {
        *param_1 = 0x87c8;
    }
    (i_var3 + 2) = &ctx.PTR_LOOP_1050_1008;
    copy_string_1000_3d3e(
        (param_1 & 0xffff0000 | (i_var3 + 4)),
        s_MicroSpinControl_1050_0370,
    );
    (i_var3 + 0x54) = 3;
    HVar1 = LoadCursor16(0x7f00, 0);
    (i_var3 + 0x58) = HVar1;
    HVar2 = GetStockObject16(4);
    (i_var3 + 0x56) = HVar2;
    reg_class_1008_818c(i_var3, u_var4);
    return param_1;
}

pub fn set_window_text_1008_9664(param_1: u32, param_2: u16, param_3: String) {
    copy_string_1000_3d3e((param_1 & 0xffff0000 | (param_1 + 10)), param_3);
    SetWindowText16(param_1 & 0xffff0000 | (param_1 + 10), (param_1 + 8));
    return;
}

pub fn destroy_wiin_1008_9698(param_1: u32) {
    DestroyWindow16((param_1 + 8));
    return;
}

pub fn show_window_1008_96ae(param_1: u32, param_2: u16) -> bool {
    let b_var1: bool;
    let mut local_es_3: u16;

    local_es_3 = (param_1 >> 0x10);
    if ((param_1 + 8) != 0) {
        b_var1 = ShowWindow16(param_2, (param_1 + 8));
        return b_var1;
    }
    return 0;
}

pub fn gui_get_info_func_1008_da12(param_1: *mut Struct61, param_2: u32) {
    let pu_var1: *mut u16;
    let mut b_var2: u8;
    let mut u_var3: u32;
    let mut u_var4: u32;
    let hdc: HDC16;
    let i_var5: u16;
    let mut u_var6: i32;
    let local_AX_193: *mut Struct62;
    let pa_var7: *mut Struct94;
    let pa_var8: *mut Struct94;
    let mut count: u16;
    let ctx.dx_reg: *mut u16;
    let pu_var9: *mut u16;
    let mut i_var10: i32;
    let mut u_var11: u16;
    let mut local_20: u32;
    let mut local_18: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    u_var11 = param_2;
    process_struct_1010_1d48(CONCAT22(u_var11, param_1), (param_2 >> 0x10));
    param_1.field_0xa = 0;
    param_1.field_0xc = 0;
    zero_list_1008_3e38(CONCAT13(
        (param_2 >> 8),
        CONCAT12(param_2, &param_1.field_0xe),
    ));
    param_1.field_0x14 = 0;
    param_1.field_0x16 = 0;
    &param_1.field_0x18 = 0;
    CONCAT22(u_var11, param_1) = 0xdc80;
    param_1.field_0x2 = &ctx.PTR_LOOP_1050_1008;
    hdc = GetDC16(0);
    i_var5 = GetDeviceCaps16(8, hdc);
    param_1.field_0xa = i_var5;
    i_var5 = GetDeviceCaps16(10, hdc);
    param_1.field_0xc = i_var5;
    pass1_1008_3e76(
        CONCAT22(u_var11, &param_1.field_0xe),
        0,
        (param_1.field_0xc + -0x1e0) / 2,
        (param_1.field_0xa + -0x280) / 2,
    );
    pu_var9 = ctx.dx_reg;
    u_var6 = GetDeviceCaps16(0x26, hdc);
    if ((u_var6 & 0x100) != 0) {
        i_var5 = GetDeviceCaps16(0x68, hdc);
        param_1.field_0x14 = i_var5;
        local_AX_193 = GetDeviceCaps16(0x6a, hdc);
        param_1.field_0x16 = local_AX_193;
        if (ctx.__g_Struct94_ptr_1 == 0) {
            pa_var7 = ((local_AX_193 + 1) * 4);
            struct_fn_1000_160a();
        } else {
            pu_var9 = ctx.g_u16_ptr_1050_5f2e;
            pa_var7 = _g_Struct94_ptr_1;
        }
        alloc_mem_1000_1708(((local_AX_193 + 1) * 4), 0, 1, pa_var7, pu_var9);
        _local_8 = CONCAT22(pu_var9, pa_var7);
        pa_var8 = ((param_1.field_0x16 + 1) * 4);
        if (ctx.__g_Struct94_ptr_1 == 0) {
            ctx.g_u16_ptr_1050_5f2e = pu_var9;
            _g_Struct94_ptr_1 = pa_var8;
            struct_fn_1000_160a();
        } else {
        }
        alloc_mem_1000_1708(pa_var8, 0, 1, _g_Struct94_ptr_1, ctx.g_u16_ptr_1050_5f2e);
        param_1.field_0x18 = _g_Struct94_ptr_1;
        param_1.field_0x1a = ctx.g_u16_ptr_1050_5f2e;
        if (_local_8 != 0x0) {
            if (&param_1.field_0x18 != 0) {
                count = param_1.field_0x16 / 2;
                GetSystemPalette16(CONCAT22(pu_var9, pa_var7), count, 0, hdc);
                GetSystemPalette16(
                    CONCAT22(pu_var9, &pa_var7.field_0x0 + count * 2),
                    count,
                    param_1.field_0x14 - count,
                    hdc,
                );
                local_20 = &param_1.field_0x18;
                local_10 = 0;
                while (
                    u_var4 = local_20,
                    pu_var1 = &param_1.field_0x16,
                    unsafe { *pu_var1 != local_10 } && unsafe { local_10 <= *pu_var1 },
                ) {
                    b_var2 = *(&pa_var7.field_0x0 + local_10 * 2);
                    u_var3 = local_20 >> 0x10;
                    i_var10 = local_20;
                    local_20 = local_20 & 0xffff0000 | (i_var10 + 4);
                    u_var4 = CONCAT11(
                        *((&pa_var7.field_0x0 + local_10 * 2) + 1),
                        *(&pa_var7.field_0x0 + local_10 * 2 + 1),
                    );
                    (i_var10 + 2) = b_var2;
                    local_10 = local_10 + 1;
                }
            }
        }
        error_check_1000_17ce(_local_8);
    }
    ReleaseDC16(hdc, 0);
    return;
}

pub fn send_dialog_item_msg_1040_d79c(param_1: u32) {
    let lVar1: u32;
    let mut u_var2: u16;

    let mut i_var3: i32;
    let mut u_var4: u16;
    let mut unaff_ss: u16;
    let LVar5: LRESULT;
    let mut local_10e: u32;
    let mut local_10a: u16;
    let mut local_108: u16;
    let mut local_106: [u8; 256];
    let mut local_6: u16;
    let mut local_4: u16;

    _local_6 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(local_10e, 3));
    u_var4 = (param_1 >> 0x10);
    i_var3 = param_1;
    pass1_1010_c3c2(
        _local_6,
        (_local_6 >> 0x10),
        CONCAT22(unaff_ss, local_106),
        (i_var3 + 0x98),
    );
    SendDlgItemMessage16(
        CONCAT22(unaff_ss, local_106),
        0,
        0xc,
        (s_dibtext_bmp_1050_1844 + 2),
        (i_var3 + 6),
    );
    SendDlgItemMessage16(0, 0, 0xb, (s_dibtext_bmp_1050_1844 + 3), (i_var3 + 6));
    LVar5 = SendDlgItemMessage16(0, 0, 0x405, (s_dibtext_bmp_1050_1844 + 3), (i_var3 + 6));
    u_var2 = LVar5;
    pass1_1010_9044((i_var3 + 0x9c));
    _local_10a = CONCAT22(ctx.dx_reg, u_var2);
    local_10e._0_2_ = 0;
    local_10e._2_2_ = 0;
    while (CONCAT22(local_10e._2_2_, local_10e) < _local_10a) {
        pass1_1010_9130((i_var3 + 0x9c), CONCAT22(unaff_ss, local_106), local_10e);
        if (local_106[0] != '\0') {
            SendDlgItemMessage16(
                CONCAT22(unaff_ss, local_106),
                0,
                0x401,
                (s_dibtext_bmp_1050_1844 + 3),
                (i_var3 + 6),
            );
        }
        lVar1 = CONCAT22(local_10e._2_2_, local_10e) + 1;
        local_10e._0_2_ = lVar1;
        local_10e._2_2_ = (lVar1 >> 0x10);
    }
    SendDlgItemMessage16(0, 1, 0xb, (s_dibtext_bmp_1050_1844 + 3), (i_var3 + 6));
    return;
}

pub fn enable_window_1040_d6be(param_1: u32) {
    let HVar1: HWND16;
    let mut i_var2: i32;
    let mut u_var3: u16;

    u_var3 = (param_1 >> 0x10);
    i_var2 = param_1;
    HVar1 = GetDlgItem16(1, (i_var2 + 6));
    EnableWindow16(0, HVar1);
    HVar1 = GetDlgItem16(2, (i_var2 + 6));
    EnableWindow16(0, HVar1);
    HVar1 = GetDlgItem16((s_vrpal_bmp_1050_183a + 7), (i_var2 + 6));
    EnableWindow16(0, HVar1);
    HVar1 = GetDlgItem16((s_dibtext_bmp_1050_1844 + 4), (i_var2 + 6));
    EnableWindow16(0, HVar1);
    HVar1 = GetDlgItem16((s_dibtext_bmp_1050_1844 + 5), (i_var2 + 6));
    EnableWindow16(0, HVar1);
    HVar1 = GetDlgItem16((s_dibtext_bmp_1050_1844 + 6), (i_var2 + 6));
    EnableWindow16(0, HVar1);
    HVar1 = GetDlgItem16((s_dibtext_bmp_1050_1844 + 7), (i_var2 + 6));
    EnableWindow16(0, HVar1);
    (i_var2 + 0xa0) = 1;
    return;
}

pub fn enable_window_1040_d60e(in_Struct588_ptr_1: *mut Struct588) -> u8 {
    let HVar1: HWND16;
    let BVar2: bool;
    let mut i_var3: i32;
    let mut u_var4: u16;

    u_var4 = (in_Struct588_ptr_1 >> 0x10);
    i_var3 = in_Struct588_ptr_1;
    HVar1 = GetDlgItem16(1, (i_var3 + 6));
    EnableWindow16(1, HVar1);
    HVar1 = GetDlgItem16(2, (i_var3 + 6));
    EnableWindow16(1, HVar1);
    HVar1 = GetDlgItem16((s_vrpal_bmp_1050_183a + 7), (i_var3 + 6));
    EnableWindow16(1, HVar1);
    HVar1 = GetDlgItem16((s_dibtext_bmp_1050_1844 + 4), (i_var3 + 6));
    EnableWindow16(1, HVar1);
    HVar1 = GetDlgItem16((s_dibtext_bmp_1050_1844 + 5), (i_var3 + 6));
    EnableWindow16(1, HVar1);
    HVar1 = GetDlgItem16((s_dibtext_bmp_1050_1844 + 6), (i_var3 + 6));
    EnableWindow16(1, HVar1);
    HVar1 = GetDlgItem16((s_dibtext_bmp_1050_1844 + 7), (i_var3 + 6));
    BVar2 = EnableWindow16(1, HVar1);
    (i_var3 + 0xa0) = 0;
    return BVar2;
}

pub fn msg_box_1040_d3d0(param_1: u32) {

    let in_dx: *mut Struct199;
    let mut u_var1: u16;
    let mut unaff_ss: u16;
    let mut local_212: u32;
    let mut local_20e: u32;
    let mut local_20a: u16;
    let mut local_208: u16;
    let mut local_206: [u8; 258];
    let mut local_104: [u8; 258];

    process_struct_1000_179c(0x1000, in_dx);
    load_string_1010_84e0(
        ctx._g_struct_73_1050_14cc,
        (ctx._g_struct_73_1050_14cc >> 0x10),
        0x100,
        CONCAT22(unaff_ss, local_206),
        0x57b,
    );
    load_string_1010_84e0(
        ctx._g_struct_73_1050_14cc,
        (ctx._g_struct_73_1050_14cc >> 0x10),
        0x3ff,
        CONCAT22(in_dx, in_ax),
        0x7da,
    );
    load_string_1010_84e0(
        ctx._g_struct_73_1050_14cc,
        (ctx._g_struct_73_1050_14cc >> 0x10),
        0x3ff,
        CONCAT22(unaff_ss, local_104),
        0x7db,
    );
    process_string_1000_3cea(CONCAT22(in_dx, in_ax), CONCAT22(unaff_ss, local_104));
    load_string_1010_84e0(
        ctx._g_struct_73_1050_14cc,
        (ctx._g_struct_73_1050_14cc >> 0x10),
        0x3ff,
        CONCAT22(unaff_ss, local_104),
        0x7dc,
    );
    process_string_1000_3cea(CONCAT22(in_dx, in_ax), CONCAT22(unaff_ss, local_104));
    load_string_1010_84e0(
        ctx._g_struct_73_1050_14cc,
        (ctx._g_struct_73_1050_14cc >> 0x10),
        0x3ff,
        CONCAT22(unaff_ss, local_104),
        0x7dd,
    );
    process_string_1000_3cea(CONCAT22(in_dx, in_ax), CONCAT22(unaff_ss, local_104));
    load_string_1010_84e0(
        ctx._g_struct_73_1050_14cc,
        (ctx._g_struct_73_1050_14cc >> 0x10),
        0x3ff,
        CONCAT22(unaff_ss, local_104),
        0x7de,
    );
    process_string_1000_3cea(CONCAT22(in_dx, in_ax), CONCAT22(unaff_ss, local_104));
    load_string_1010_84e0(
        ctx._g_struct_73_1050_14cc,
        (ctx._g_struct_73_1050_14cc >> 0x10),
        0x3ff,
        CONCAT22(unaff_ss, local_104),
        0x7df,
    );
    process_string_1000_3cea(CONCAT22(in_dx, in_ax), CONCAT22(unaff_ss, local_104));
    u_var1 = (param_1 >> 0x10);
    MessageBox16(
        0,
        CONCAT22(unaff_ss, local_206),
        CONCAT22(in_dx, in_ax),
        (param_1 + 6),
    );
    load_string_1010_84e0(
        ctx._g_struct_73_1050_14cc,
        (ctx._g_struct_73_1050_14cc >> 0x10),
        0x3ff,
        CONCAT22(in_dx, in_ax),
        0x7e0,
    );
    load_string_1010_84e0(
        ctx._g_struct_73_1050_14cc,
        (ctx._g_struct_73_1050_14cc >> 0x10),
        0x3ff,
        CONCAT22(unaff_ss, local_104),
        0x7e1,
    );
    process_string_1000_3cea(CONCAT22(in_dx, in_ax), CONCAT22(unaff_ss, local_104));
    load_string_1010_84e0(
        ctx._g_struct_73_1050_14cc,
        (ctx._g_struct_73_1050_14cc >> 0x10),
        0x3ff,
        CONCAT22(unaff_ss, local_104),
        0x7e2,
    );
    process_string_1000_3cea(CONCAT22(in_dx, in_ax), CONCAT22(unaff_ss, local_104));
    load_string_1010_84e0(
        ctx._g_struct_73_1050_14cc,
        (ctx._g_struct_73_1050_14cc >> 0x10),
        0x3ff,
        CONCAT22(unaff_ss, local_104),
        0x7e3,
    );
    process_string_1000_3cea(CONCAT22(in_dx, in_ax), CONCAT22(unaff_ss, local_104));
    load_string_1010_84e0(
        ctx._g_struct_73_1050_14cc,
        (ctx._g_struct_73_1050_14cc >> 0x10),
        0x3ff,
        CONCAT22(unaff_ss, local_104),
        0x7e4,
    );
    process_string_1000_3cea(CONCAT22(in_dx, in_ax), CONCAT22(unaff_ss, local_104));
    load_string_1010_84e0(
        ctx._g_struct_73_1050_14cc,
        (ctx._g_struct_73_1050_14cc >> 0x10),
        0x3ff,
        CONCAT22(unaff_ss, local_104),
        0x7e5,
    );
    process_string_1000_3cea(CONCAT22(in_dx, in_ax), CONCAT22(unaff_ss, local_104));
    MessageBox16(
        0,
        CONCAT22(unaff_ss, local_206),
        CONCAT22(in_dx, in_ax),
        (param_1 + 6),
    );
    error_check_1000_17ce(CONCAT22(in_dx, in_ax));
    return;
}

pub fn send_dialog_item_msg_1040_d20c(in_struct_588_ptr_1: *mut Struct588, param_2: u32) {
    let ppVar1: *mut pass1_struct_2;
    let mut in_dx: u16;
    let struct_588_ptr_2: *mut Struct588;
    let mut u_var2: u16;
    let mut unaff_ss: u16;
    let ppVar3: *mut pass1_struct_1;
    let pu_var4: Vec<u8>;
    let mut u_var5: u16;
    let struct_588_ptr_1: *mut Struct588;
    let mut local_10a: u16;
    let mut local_108: u16;
    let mut local_106: [u8; 256];
    let mut local_6: u16;
    let mut local_4: u16;

    if (param_2 == 0) {
        enable_window_1040_d60e(in_struct_588_ptr_1);
        return;
    }
    u_var2 = (in_struct_588_ptr_1 >> 0x10);
    struct_588_ptr_2 = in_struct_588_ptr_1;
    if (struct_588_ptr_2.field_0xa0 != 0) {
        pass1_1010_9210(struct_588_ptr_2.field_0x9c);
        enable_window_1040_d60e(in_struct_588_ptr_1);
        ppVar1 = pass1_1030_8344(
            ctx._g_bool_1050_5748,
            (ctx._g_bool_1050_5748 >> 0x10),
            param_2,
        );
        pu_var4 = local_106;
        u_var5 = unaff_ss;
        local_6 = ppVar1;
        ppVar3 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(pu_var4, 3));
        pass1_1010_c3c2(
            ppVar3,
            (ppVar3 >> 0x10),
            CONCAT22(u_var5, pu_var4),
            CONCAT22(in_dx, ppVar1),
        );
        SendDlgItemMessage16(
            CONCAT22(unaff_ss, local_106),
            0,
            0x401,
            (s_dibtext_bmp_1050_1844 + 3),
            struct_588_ptr_2.field_0x6,
        );
    }
    return;
}

pub fn send_dlg_item_msg_1040_d29c(param_1: u32) {
    send_dialog_item_msg_1040_d79c(param_1);
    return;
}

pub fn enable_window_1040_cf1c(in_Struct123: *mut Struct123) -> LRESULT {
    let h_wnd: HWND16;
    let local_Struct123: *mut Struct123;
    let mut u_var1: u16;
    let mut unaff_ss: u16;
    let ppVar2: *mut pass1_struct_1;
    let mut u_var3: u32;
    let LVar4: LRESULT;
    let enable: bool;
    let mut u_var5: u16;
    let mut buffer_50c: [u8; 1026];
    let mut local_10a: u16;
    let mut local_108: u16;
    let mut buffer_106: [u8; 256];
    let mut local_6: u16;
    let mut local_4: u16;

    ppVar2 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(buffer_50c._0_2_, 3));
    u_var1 = (in_Struct123 >> 0x10);
    local_Struct123 = in_Struct123;
    pass1_1010_c3c2(
        ppVar2,
        (ppVar2 >> 0x10),
        CONCAT22(unaff_ss, buffer_106),
        local_Struct123.field_0x94,
    );
    SendDlgItemMessage16(
        CONCAT22(unaff_ss, buffer_106),
        0,
        0xc,
        (s_dibtext_bmp_1050_1844 + 2),
        local_Struct123.h_wnd_0x6,
    );
    // msg = WM_SETREDRAW
    SendDlgItemMessage16(
        0,
        0,
        0xb,
        (s_vrpal_bmp_1050_183a + 8),
        local_Struct123.h_wnd_0x6,
    );
    // WM_PSD_ENVSTAMPRECT
    SendDlgItemMessage16(
        0,
        0,
        0x405,
        (s_vrpal_bmp_1050_183a + 8),
        local_Struct123.h_wnd_0x6,
    );
    u_var5 = SUB42(s_vrpal_bmp_1050_183a + 8, 0);
    u_var3 = pass1_1018_526a(local_Struct123.field_0x98, local_Struct123.field_0x94);
    send_dialog_item_msg_1040_ce12(in_Struct123, u_var3, u_var5);
    // 040c   1036   SB_GETTEXTLENGTHW
    // 040c   1036   TB_ISBUTTONHIDDEN
    LVar4 = SendDlgItemMessage16(
        0,
        0,
        0x40c,
        (s_vrpal_bmp_1050_183a + 8),
        local_Struct123.h_wnd_0x6,
    );
    if (((LVar4 >> 0x10) != 0 && -1 < LVar4) || (-1 < LVar4 && (LVar4 != 0))) {
        h_wnd = GetDlgItem16(1, local_Struct123.h_wnd_0x6);
        enable = 1;
    } else {
        load_string_1010_84e0(
            ctx._g_struct_73_1050_14cc,
            (ctx._g_struct_73_1050_14cc >> 0x10),
            0x3ff,
            CONCAT22(unaff_ss, buffer_50c),
            0x44a,
        );
        // 0401   1025   DDM_DRAW
        // 0401   1025   HKM_SETHOTKEY
        // 0401   1025   TB_ENABLEBUTTON
        // 0401   1025   WM_CHOOSEFONT_GETLOGFONT
        // 0401   1025   WM_PSD_FULLPAGERECT
        SendDlgItemMessage16(
            CONCAT22(unaff_ss, buffer_50c),
            0,
            0x401,
            (s_vrpal_bmp_1050_183a + 8),
            local_Struct123.h_wnd_0x6,
        );
        h_wnd = GetDlgItem16(1, local_Struct123.h_wnd_0x6);
        enable = 0;
    }
    EnableWindow16(enable, h_wnd);
    // WM_SETREDRAW
    LVar4 = SendDlgItemMessage16(
        0,
        1,
        0xb,
        (s_vrpal_bmp_1050_183a + 8),
        local_Struct123.h_wnd_0x6,
    );
    return LVar4;
}

pub fn enable_window_1040_cc66(param_1: *mut Struct123) {
    let pp_var1: fn();

    pp_var1 = ((param_1 + 0x98) + 0x10);
    (**pp_var1)();
    enable_window_1040_cf1c(param_1);
    return;
}

pub fn win_gui_fn_1040_cc8c(param_1: *mut Struct124, param_2: u16, param_3: u16, param4: u32) {
    if (param_3._2_2_ == 0xeb) {
        enable_window_1040_cf1c(CONCAT22(param_2, param_1));
    } else {
        if (param_3._2_2_ == (s_vrpal_bmp_1050_183a + 7)) {
            display_msg_box_1040_cce4(CONCAT22(param_2, param_1));
        } else {
            if (param_3._2_2_ != (s_vrpal_bmp_1050_183a + 8)) {
                win_gui_fn_1040_b54a(param_1, param_2, param_3_00, param_3);
                return;
            }
            if (param_3 == 1) {
                send_dlg_item_msg_1040_ce76(CONCAT22(param_2, param_1));
            }
        }
    }
    return;
}

pub fn display_msg_box_1040_cce4(param_1: &mut Vec<u8>) {
    let msg_box_text: String;
    let in_dx: *mut Struct199;
    let mut unaff_ss: u16;
    let mut local_212: u32;
    let mut local_20e: u32;
    let mut local_20a: u16;
    let mut local_208: u16;
    let mut msg_box_title: [u8; 258];
    let mut string_3: [u8; 258];

    process_struct_1000_179c(0x1000, in_dx);
    load_string_1010_84e0(
        ctx._g_struct_73_1050_14cc,
        (ctx._g_struct_73_1050_14cc >> 0x10),
        0x100,
        CONCAT22(unaff_ss, msg_box_title),
        0x57b,
    );
    load_string_1010_84e0(
        ctx._g_struct_73_1050_14cc,
        (ctx._g_struct_73_1050_14cc >> 0x10),
        0x3ff,
        CONCAT22(in_dx, msg_box_text),
        0x7e9,
    );
    load_string_1010_84e0(
        ctx._g_struct_73_1050_14cc,
        (ctx._g_struct_73_1050_14cc >> 0x10),
        0x3ff,
        CONCAT22(unaff_ss, string_3),
        0x7ea,
    );
    process_string_1000_3cea(CONCAT22(in_dx, msg_box_text), CONCAT22(unaff_ss, string_3));
    load_string_1010_84e0(
        ctx._g_struct_73_1050_14cc,
        (ctx._g_struct_73_1050_14cc >> 0x10),
        0x3ff,
        CONCAT22(unaff_ss, string_3),
        0x7eb,
    );
    process_string_1000_3cea(CONCAT22(in_dx, msg_box_text), CONCAT22(unaff_ss, string_3));
    // type =  MB_OK 0x00000000L The message box contains one push button: OK. This
    // is the default.
    MessageBox16(
        0,
        CONCAT22(unaff_ss, msg_box_title),
        CONCAT22(in_dx, msg_box_text),
        (param_1 + 6),
    );
    error_check_1000_17ce(CONCAT22(in_dx, msg_box_text));
    return;
}

pub fn send_dlg_item_int_1040_cdac(param_1: u32, param_2: u16, param_3: u16, param_4: i32) -> u16 {
    let pi_var1: *mut i32;
    let mut i_var2: i32;
    let mut u8_var3: bool;
    let mut i_var4: i32;
    let mut u_var5: u16;
    let mut local_4: u16;

    u8_var3 = false;
    i_var4 = param_1;
    u_var5 = (param_1 >> 0x10);
    if (param_2 == 0) {
        i_var2 = (i_var4 + 0x9e);
        pi_var1 = (i_var4 + 0x9c);
        unsafe {
            if (*pi_var1 == i_var2 || *pi_var1 < i_var2) {}
            // goto LAB_1040_cdef;
            pi_var1 = (i_var4 + 0x9e);
            *pi_var1 = *pi_var1 + 1;
        }
    } else {
        if (param_2 != 1) {}
        // goto LAB_1040_cdef;
        if ((i_var4 + 0x9e) < 1) {}
        // goto LAB_1040_cdef;
        pi_var1 = (i_var4 + 0x9e);
        unsafe {
            *pi_var1 = *pi_var1 + -1;
        }
    }
    u8_var3 = true;
    // LAB_1040_cdef:
    if (u8_var3) {
        (0, *(i_var4 + 0x9e), 0x18e, (i_var4 + 6));
    }
    return 0;
}

pub fn send_dialog_item_msg_1040_ce12(param_1: u32, param_2: u32, param_3: u16) {
    let mut unaff_ss: u16;
    let lVar1: u32;
    let mut local_10e: u16;
    let mut local_10c: u16;
    let mut local_10a: u32;
    let mut iStack262: i32;
    let HStack260: HWND16;
    let mut local_a: [u8; 8];

    pass1_1008_5784(CONCAT22(unaff_ss, local_a), param_2);
    while (true) {
        lVar1 = pass1_1008_5b12(CONCAT22(unaff_ss, local_a));
        if (lVar1 == 0) {
            break;
        }
        wsprintf16(
            &local_10a,
            CONCAT22(0x5f12, unaff_ss),
            CONCAT22((lVar1 + 4), 0x1050),
        );
        HStack260 = (param_1 + 6);
        iStack262 = param_3;
        local_10a = 0x4010000;
        _local_10e = CONCAT22(unaff_ss, &local_10a);
        SendDlgItemMessage16(_local_10e, 0, 0x401, param_3, HStack260);
    }
    return;
}

pub fn send_dlg_item_msg_1040_ce76(param_1: u32) {
    let mut i_var1: i32;
    let mut u_var2: u16;
    let mut unaff_ss: u16;
    let LVar3: LRESULT;
    let mut u_var4: u32;
    let mut local_10a: u16;
    let mut local_108: u16;
    let mut local_106: [u8; 256];
    let mut local_6: u16;
    let mut local_4: u16;

    u_var2 = (param_1 >> 0x10);
    i_var1 = param_1;
    LVar3 = SendDlgItemMessage16(0, 0, 0x409, (s_vrpal_bmp_1050_183a + 8), (i_var1 + 6));
    local_6 = LVar3;
    local_4 = local_6 >> 0xf;
    if (local_6 != 0xffff) {
        SendDlgItemMessage16(
            CONCAT22(unaff_ss, local_106),
            local_6,
            0x40a,
            (s_vrpal_bmp_1050_183a + 8),
            (i_var1 + 6),
        );
        u_var4 = pass1_1018_5206((i_var1 + 0x98));
        if (u_var4 != 0) {
            (i_var1 + 0x9c) = (u_var4 + 8);
            (i_var1 + 0x9e) = 0;
            SetDlgItemint16(0, 0, 0x18e, (i_var1 + 6));
            SetDlgItemint16(0, *(i_var1 + 0x9c), 0x191, (i_var1 + 6));
        }
    }
    return;
}

pub fn destroy_win_1040_bf92(param_1: *mut Struct339) {
    let local_bx_4: *mut Struct339;
    let mut u_var1: i32;

    u_var1 = (param_1 >> 0x10);
    local_bx_4 = param_1;
    param_1 = 0xc53e;
    local_bx_4.field_0x2 = &ctx.PTR_LOOP_1050_1040;
    pass1_1010_1ea6(local_bx_4.field_0x6, (param_1 & 0xffff | u_var1 << 0x10));
    destroy_win_1010_2fa0(local_bx_4.field_0x6);
    param_1 = ctx.s_0_020_1050_3ab0;
    local_bx_4.field_0x2 = &ctx.PTR_LOOP_1050_1008;
    param_1 = ctx.s_1_1050_389a;
    local_bx_4.field_0x2 = &ctx.PTR_LOOP_1050_1008;
    return;
}

pub fn win_fn_1040_bbe2(param_1: *mut Struct124, param_2: u16, param_3: u16, param_4: u32) {
    let mut u_var1: u16;
    let ppc_var2: fn();
    let mut u_var3: u32;
    let mut i_var4: i32;
    let b_var5: bool;
    let mut u_var6: u16;
    let mut u_var7: u16;
    let h_wnd: HWND16;
    let mut u_var8: u16;
    let unaff_ss: HWND16;
    let pp_var9: *mut pass1_struct_1;
    let pu_var10: Vec<u8>;
    let u_var11: u8;
    let u_var12: u8;
    let mut u_var13: u16;
    let mut u_var14: u16;
    let mut local_22: u16;
    let mut local_20: u16;
    let mut local_1e: u16;
    let mut local_1c: u16;
    let mut local_1a: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    if (param_2._2_2_ != 0x10c) {
        if (param_2._2_2_ < 0x10d) {
            if (param_2._2_2_ == 0xfa) {
                ppc_var2 = (&param_1[1].field_0x4 + 0x18);
                ppc_var2();
                return;
            }
            if (param_2._2_2_ == 0x10a) {
                GetClientRect16(CONCAT22(unaff_ss, &local_a), &param_1.field_0x6);
                u_var3 = &param_1[1].field_0x4;
                local_8 = local_8 + 3;
                local_a = (u_var3 + 0x1a) - 9;
                local_6 = local_6 - 3;
                local_4 = local_4 - 3;
                InvalidateRect16(1, &local_a, unaff_ss);
                destroy_win_1010_2fa0(&param_1[1].field_0x4);
                pass1_1010_32c0(&param_1[1].field_0x4, 0);
                u_var3 = &param_1[1].field_0x4;
                local_22 = (u_var3 >> 0x10);
                pass1_1010_2ee2(u_var3, local_22);
                return;
            }
            if (param_2._2_2_ != 0x10b) {
                // LAB_1040_be78:
                win_gui_fn_1040_b54a(param_1, param_2_00, param_3, param_2);
                return;
            }
            u_var3 = &param_1[1].field_0x4;
            u_var6 = (u_var3 + 0x12);
            u_var7 = u_var6;
            pp_var9 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(u_var6, 3));
            u_var8 = (pp_var9 >> 0x10);
            i_var4 = pass1_1010_a5ca(pp_var9, u_var8, u_var7);
            if ((u_var6 != 0x70) && (i_var4 == 0)) {
                return;
            }
            u_var3 = &param_1[1].field_0x1c;
            u_var13 = u_var3;
            u_var14 = (u_var3 >> 0x10);
            u_var3 = &param_1[1].field_0x4;
            u_var1 = (u_var3 + 0x12);
            u_var11 = u_var1;
            u_var12 = (u_var1 >> 8);
        } else {
            if (param_2._2_2_ != 0x10d) {
                if (param_2._2_2_ == 0x10e) {
                    pp_var9 = process_struct_1010_20ba(
                        ctx._g_Struct372_1050_0ed0,
                        CONCAT22(local_22, 0x32),
                    );
                    i_var4 = win_gui_fn_1010_79aa(pp_var9, 0xfc6, &param_1[1].field_0x1c);
                    if (i_var4 != 0) {
                        return;
                    }
                    u_var3 = &param_1[1].field_0x1c;
                    window_msg_func_1010_7300(pp_var9, 0, 0, 0x13, u_var3, (u_var3 >> 0x10));
                    return;
                }
                if (param_2._2_2_ != 0xbbb) {
                    if (param_2._2_2_ == 0xbbc) {
                        pp_var9 = process_struct_1010_20ba(
                            ctx._g_Struct372_1050_0ed0,
                            CONCAT22(local_22, 3),
                        );
                        u_var8 = (pp_var9 >> 0x10);
                        u_var6 = pp_var9;
                        u_var7 = pass1_1010_a5ac(u_var6, u_var8, &param_1[1].field_0x1c);
                        i_var4 = pass1_1010_a58a(u_var6, u_var8, u_var7);
                        if (i_var4 == 0) {
                            pass1_1010_a568(u_var6, u_var8, u_var7);
                        }
                        h_wnd = GetDlgItem16(0xbbc, &param_1.field_0x6);
                        EnableWindow16(0, h_wnd);
                        return;
                    }
                    // goto LAB_1040_be78;
                }
                if ((&param_1[1].field_0x22 == 0)
                    || (b_var5 = IsWindow16(&param_1[1].field_0x22), b_var5 == 0))
                {
                    pu_var10 = ui2::pass1_1038_af40(ctx._g_Struct112_a, *&param_1.field_0x6, 0x1b);
                    &param_1[1].field_0x22 = (pu_var10 + 6);
                    ShowWindow16(1, &param_1[1].field_0x22);
                    return;
                }
                local_22 = &param_1[1].field_0x22;
                // goto LAB_1040_bd39;
            }
            pp_var9 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(local_22, 3));
            u_var8 = (pp_var9 >> 0x10);
            u_var3 = &param_1[1].field_0x1c;
            u_var13 = u_var3;
            u_var14 = (u_var3 >> 0x10);
            u_var11 = 0x71;
            u_var12 = 0;
        }
        local_1e = pp_var9;
        pass1_1010_a5ec(
            local_1e,
            u_var8,
            CONCAT11(u_var12, u_var11),
            CONCAT22(u_var14, u_var13),
        );
        if ((&param_1[1].field_0x20 != 0)
            && (b_var5 = IsWindow16(&param_1[1].field_0x20), b_var5 != 0))
        {
            SendMessage16(0, 0xeb, 0x111, &param_1[1].field_0x20);
        }
    }
    local_22 = &param_1.field_0x6;
    // LAB_1040_bd39:
    DestroyWindow16(local_22);
    return;
}

pub fn destroy_window_1040_bb78(param_1: u32) {
    let pu_var1: *mut u32;
    let mut u_var2: i32;
    let ppc_var3: fn();
    let b_var4: bool;
    let mut i_var5: i32;
    let mut u_var6: u16;
    let mut unaff_cs: u16;

    u_var6 = (param_1 >> 0x10);
    i_var5 = param_1;
    if ((i_var5 + 0xb6) != 0) {
        unaff_cs = SUB42(offset, 0);
        b_var4 = IsWindow16((i_var5 + 0xb6));
        if (b_var4 != 0) {
            unaff_cs = SUB42(offset, 0);
            DestroyWindow16((i_var5 + 0xb6));
        }
    }
    (i_var5 + 0xb6) = 0;
    pu_var1 = (i_var5 + 0x94);
    u_var2 = (i_var5 + 0x96);
    if ((u_var2 | pu_var1) != 0) {
        unsafe { ppc_var3 = *pu_var1 };
        (**ppc_var3)(unaff_cs, pu_var1, u_var2, 1);
    }
    (i_var5 + 0x94) = 0;
    (i_var5 + 0x98) = 0;
    return;
}

pub fn set_window_pos_1040_b8d2(param_1: *mut Struct20) {
    let mut u_var1: u32;
    let mut u_var2: u16;
    let mut u_var3: i32;
    let mut u_var4: u16;
    let mut u_var5: u16;
    let mut i32_var6: i32;

    let struct_a: *mut Struct199;
    let pa_var7: *mut Struct199;
    let ctx.dx_reg: *mut Struct199;
    let pa_var8: *mut Struct199;
    let ctx.dx_reg: *mut Struct199;
    let ctx.dx_reg: *mut Struct199;
    let ctx.dx_reg: *mut Struct199;
    let extraout_dx_04: *mut Struct199;
    let extraout_dx_05: *mut Struct199;
    let mut extraout_dx_06: u16;
    let mut u_var9: u16;
    let mut i_var10: i32;
    let mut u_var11: u16;
    let mut u_var12: u16;
    let ppVar13: *mut pass1_struct_1;
    let mut in_stack_0000ffe4: u16;
    let mut local_12: u16;
    let mut local_10: u32;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    win_gui_func_1040_78e2(param_1);
    ppVar13 = process_struct_1010_20ba(
        ctx._g_Struct372_1050_0ed0,
        CONCAT22(in_stack_0000ffe4, 0x31),
    );
    pa_var8 = (ppVar13 >> 0x10);
    u_var3 = ppVar13;
    u_var11 = (param_1 >> 0x10);
    i_var10 = param_1;
    (i_var10 + 0x98) = u_var3;
    (i_var10 + 0x9a) = pa_var8;
    process_struct_1000_179c(10, pa_var8);
    if ((pa_var8 | u_var3) == 0) {
        (i_var10 + 0x94) = 0;
    } else {
        process_struct_1040_bf3e(CONCAT22(pa_var8, u_var3), (i_var10 + 6));
        (i_var10 + 0x94) = u_var3;
        (i_var10 + 0x96) = ctx.dx_reg;
    }
    pass1_1040_bfde((i_var10 + 0x94), (i_var10 + 0x98));
    pa_var8 = struct_a;
    process_struct_1000_179c(0x42, struct_a);
    pa_var7 = (pa_var8 | u_var3);
    if (pa_var7 != 0x0) {
        win_fn_1008_3bd6(
            u_var3,
            pa_var8,
            1,
            0xa000a,
            0x810000,
            0x10a0080,
            CONCAT22(in_stack_0000ffe4, (i_var10 + 6)),
        );
        pa_var7 = ctx.dx_reg;
    }
    process_struct_1000_179c(0x42, pa_var7);
    pa_var8 = (pa_var7 | u_var3);
    if (pa_var8 != 0x0) {
        win_fn_1008_3bd6(
            u_var3,
            pa_var7,
            1,
            0xa0028,
            0x850000,
            0x10b0084,
            CONCAT22(in_stack_0000ffe4, (i_var10 + 6)),
        );
        pa_var8 = ctx.dx_reg;
    }
    process_struct_1000_179c(0x42, pa_var8);
    pa_var7 = (pa_var8 | u_var3);
    if (pa_var7 != 0x0) {
        win_fn_1008_3bd6(
            u_var3,
            pa_var8,
            1,
            0xa0046,
            0x870000,
            0x10d0086,
            CONCAT22(in_stack_0000ffe4, (i_var10 + 6)),
        );
        pa_var7 = ctx.dx_reg;
    }
    process_struct_1000_179c(0x42, pa_var7);
    pa_var8 = (pa_var7 | u_var3);
    if (pa_var8 != 0x0) {
        win_fn_1008_3bd6(
            u_var3,
            pa_var7,
            1,
            0xa0064,
            0x890000,
            0x10e0088,
            CONCAT22(in_stack_0000ffe4, (i_var10 + 6)),
        );
        pa_var8 = ctx.dx_reg;
    }
    process_struct_1000_179c(0x42, pa_var8);
    pa_var7 = (pa_var8 | u_var3);
    if (pa_var7 != 0x0) {
        win_fn_1008_3bd6(
            u_var3,
            pa_var8,
            1,
            0xa0082,
            0x830000,
            0x10c0082,
            CONCAT22(in_stack_0000ffe4, (i_var10 + 6)),
        );
        pa_var7 = extraout_dx_04;
    }
    process_struct_1000_179c(0x42, pa_var7);
    pa_var8 = (pa_var7 | u_var3);
    if (pa_var8 != 0x0) {
        win_fn_1008_3bd6(
            u_var3,
            pa_var7,
            1,
            0xa00d2,
            0x8b0000,
            0xbbb008a,
            CONCAT22(in_stack_0000ffe4, (i_var10 + 6)),
        );
        pa_var8 = extraout_dx_05;
    }
    process_struct_1000_179c(0x42, pa_var8);
    if ((pa_var8 | u_var3) == 0) {
        u_var3 = 0;
        u_var12 = 0;
    } else {
        win_fn_1008_3bd6(
            u_var3,
            pa_var8,
            1,
            0xa00a0,
            0x8d008e,
            0xbbc008c,
            CONCAT22(in_stack_0000ffe4, (i_var10 + 6)),
        );
        u_var12 = extraout_dx_06;
    }
    ppVar13 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(in_stack_0000ffe4, 3));
    u_var9 = (ppVar13 >> 0x10);
    u_var2 = ppVar13;
    u_var4 = pass1_1010_a5ac(u_var2, u_var9, (i_var10 + 0xb0));
    u_var5 = pass1_1010_ac62(u_var2, u_var9, 0x1e);
    if (u_var5 != 0) {
        pass1_1010_a5ca(u_var2, u_var9, u_var4);
        if (0 < u_var5) {
            pass1_1010_a58a(u_var2, u_var9, u_var4);
            if (u_var5 == 0) {}
            // goto LAB_1040_bb26;
        }
    }
    enable_window_1040_9234(u_var3, u_var12, 0);
    // LAB_1040_bb26:
    u_var1 = (i_var10 + 0x98);
    i32_var6 = u_var1;
    u_var1 = u_var1 & 0xffff0000;
    u_var12 = (u_var1 >> 0x10);
    SetWindowPos16(
        0x40,
        (i32_var6 + 0x10),
        (i32_var6 + 0xe),
        (i32_var6 + 0xc),
        (u_var1 | i32_var6 + 10),
        0,
        (i_var10 + 6),
    );
    return;
}

pub fn destroy_window_1040_b726(param_1: *mut u32, param_2: i32) {
    let pp_var1: fn();

    if (param_2 != 0) {
        unsafe {
            pp_var1 = (*param_1 + 0x78);
        }
        (**pp_var1)();
    }
    DestroyWindow16((param_1 + 6));
    return;
}

pub fn win_gui_fn_1040_b54a(param_1: *mut Struct124, param_2: u16, param_3: u16, param_2_00: u32) {
    let in_struct_1: &mut Struct44;
    let pp_var1: fn();
    let mut u_var2: u32;
    let mut u_var3: u16;
    let mut u_var4: u16;
    let pa_var5: &mut Struct44;

    let mut i32_var6: i32;
    let mut u_var7: u16;
    let pp_var8: *mut pass1_struct_1;
    let u_var9: u8;
    let mut u_var10: u16;
    let mut u_var11: u16;
    let mut u_var12: u16;
    let mut in_stack_0000ffe6: u16;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_c: u32;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;
    let mut fn_ptr_2: u32;

    if (param_2_00._2_2_ == 0xea) {
        fn_ptr_2 = (CONCAT22(param_2, param_1) + 0x5c);
        (**fn_ptr_2)();
    } else {
        if (param_2_00._2_2_ == 0xeb) {
            pp_var8 = process_struct_1010_20ba(
                ctx._g_Struct372_1050_0ed0,
                CONCAT22(in_stack_0000ffe6, 3),
            );
            in_struct_1 = &param_1.field_0x90;
            if (in_struct_1 != 0x0) {
                u_var7 = (in_struct_1 >> 0x10);
                u_var10 = 0x1010;
                pa_var5 = in_struct_1;
                pass1_1010_ad64(
                    pp_var8,
                    CONCAT22((in_struct_1 + 10), (pp_var8 >> 0x10)),
                    (in_struct_1 + 6),
                );
                param_1.field_0x90 = pa_var5;
                &param_1.field_0x92 = ctx.dx_reg;
                if ((ctx.dx_reg | param_1.field_0x90) == 0) {
                    &param_1.field_0x90 = in_struct_1;
                } else {
                    if (in_struct_1 != 0x0) {
                        pass1_1040_a5d0(in_struct_1);
                        u_var10 = 0x1000;
                        error_check_1000_17ce(in_struct_1);
                    }
                    pp_var1 = (CONCAT22(param_2, param_1) + 0x70);
                    (**pp_var1)(u_var10, param_1, param_2);
                }
            }
        } else {
            if (param_2_00._2_2_ == 0x1790) {
                pp_var8 = process_struct_1010_20ba(
                    ctx._g_Struct372_1050_0ed0,
                    CONCAT22(in_stack_0000ffe6, 0x32),
                );
                u_var3 = pp_var8;
                u_var2 = &param_1.field_0x90;
                u_var2 = (u_var2 + 6);
                pass1_1010_7d38(u_var3, (pp_var8 >> 0x10), u_var2, (u_var2 >> 0x10));
                u_var4 = u_var3;
                win_gui_fn_1010_79aa(pp_var8, 0xfab, 0);
                if (u_var4 != 0) {
                    return;
                }
                if (u_var3 == 0) {
                    u_var2 = &param_1.field_0x90;
                    u_var7 = (u_var2 >> 0x10);
                    i32_var6 = u_var2;
                    u_var2 = (i32_var6 + 6);
                    u_var11 = u_var2;
                    u_var12 = (u_var2 >> 0x10);
                    u_var10 = 0x14;
                } else {
                    u_var2 = &param_1.field_0x90;
                    u_var7 = (u_var2 >> 0x10);
                    i32_var6 = u_var2;
                    u_var2 = (i32_var6 + 6);
                    u_var11 = u_var2;
                    u_var12 = (u_var2 >> 0x10);
                    u_var10 = 9;
                }
                u_var9 = u_var7;
            } else {
                if (param_2_00._2_2_ == 0x1824) {
                    pp_var8 = process_struct_1010_20ba(
                        ctx._g_Struct372_1050_0ed0,
                        CONCAT22(in_stack_0000ffe6, 0x32),
                    );
                    i32_var6 = pp_var8;
                    u_var2 = &param_1.field_0x90;
                    win_gui_fn_1010_79aa(pp_var8, 0xfc5, (u_var2 + 6));
                    if (i32_var6 != 0) {
                        return;
                    }
                    u_var2 = &param_1.field_0x90;
                    u_var2 = (u_var2 + 6);
                    u_var11 = u_var2;
                    u_var12 = (u_var2 >> 0x10);
                    u_var10 = 0x12;
                    u_var9 = 0;
                } else {
                    if (param_2_00._2_2_ != 0x1830) {
                        post_win_msg_1040_7b3c(param_1, param_2, param_3, param_2_00);
                        return;
                    }
                    pp_var8 = process_struct_1010_20ba(
                        ctx._g_Struct372_1050_0ed0,
                        CONCAT22(in_stack_0000ffe6, 0x32),
                    );
                    i32_var6 = pp_var8;
                    u_var2 = &param_1.field_0x90;
                    win_gui_fn_1010_79aa(pp_var8, 0xfb6, (u_var2 + 6));
                    if (i32_var6 != 0) {
                        return;
                    }
                    u_var2 = &param_1.field_0x90;
                    u_var2 = (u_var2 + 6);
                    u_var11 = u_var2;
                    u_var12 = (u_var2 >> 0x10);
                    u_var10 = 0xc;
                    u_var9 = 0;
                }
            }
            window_msg_func_1010_7300(pp_var8, i32_var6, u_var9, u_var10, u_var11, u_var12);
        }
    }
    return;
}

pub fn show_win_1040_b43c(param_1: *mut u32) {
    let pp_var1: fn();

    unsafe {
        pp_var1 = (*param_1 + 0x70);
    }
    (**pp_var1)();
    ShowWindow16(5, (param_1 + 6));
    return;
}

pub fn set_dlg_item_txt_1040_b45e(param_1: u32) {
    let mut u_var1: u32;
    let p_uvar2: *mut u16;
    let mut i_var3: i32;
    let mut u_var4: u16;
    let mut local_8: u16;
    let mut local_6: u32;

    u_var4 = (param_1 >> 0x10);
    i_var3 = param_1;
    if ((i_var3 + 0x90) != 0) {
        u_var1 = (i_var3 + 0x90);
        (u_var1 + 0x14) = (i_var3 + 6);
        u_var1 = (i_var3 + 0x90);
        local_6 = (u_var1 + 2);
        local_8 = 0;
        while (
            pu_var2 = (i_var3 + 0x90),
            *pu_var2 != local_8 && local_8 <= *pu_var2,
        ) {
            u_var1 = (local_6 + 2);
            SetDlgItemText16(CONCAT22(u_var1, local_6), (u_var1 >> 0x10), (i_var3 + 6));
            local_8 = local_8 + 1;
            local_6 = local_6 & 0xffff0000 | (local_6 + 10);
        }
    }
    return;
}

pub fn win_gui_fn_1040_b4c8(param_1: u32) {
    let mut u_var1: u32;
    let mut i_var2: i32;
    let mut u_var3: u16;
    let ppVar4: *mut pass1_struct_1;
    let mut local_6: u16;
    let mut local_4: u16;

    u_var3 = (param_1 >> 0x10);
    if ((param_1 + 0x90) != 0) {
        ppVar4 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(local_6, 0x32));
        u_var1 = (param_1 + 0x90);
        i_var2 = (u_var1 + 10);
        if (i_var2 == 4) {
            i_var2 = win_gui_fn_1010_79aa(ppVar4, 0xfd9, 0);
            if (i_var2 == 0) {
                u_var3 = 0xe;
                // LAB_1040_b50f:
                window_msg_func_1010_7300(ppVar4, i_var2, i_var2, u_var3, i_var2, i_var2);
                return;
            }
        } else {
            if (((0 < i_var2 + -5) && (!SBORROW2(i_var2 + -5, 1))) && (i_var2 + -6 < 2)) {
                i_var2 = win_gui_fn_1010_79aa(ppVar4, 0xfda, 0);
                if (i_var2 == 0) {
                    u_var3 = 0xd;
                    // goto LAB_1040_b50f;
                }
            }
        }
    }
    return;
}

pub fn set_window_pos_1040_b230(param_1: *mut Struct20) {
    let pp_var1: fn();
    let i_var2: u16;
    let mut i_var3: i32;
    let mut u_var4: u16;
    let unaff_ss: HWND16;
    let pu_var5: *mut u16;
    let u_var6: u8;
    let u_var7: u8;
    let pu_var8: *mut u16;
    let HVar9: HWND16;
    let mut local_1e: u16;
    let mut local_1c: u16;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    win_gui_func_1040_78e2(param_1);
    if (PTR_LOOP_1050_5ef8 == (&PTR_DAT_0005_0000_1050_0004 + 1)) {
        PTR_LOOP_1050_5ef8 = 0x0;
    }
    pu_var8 = &local_4;
    pu_var5 = &local_6;
    u_var6 = unaff_ss;
    u_var7 = (unaff_ss >> 8);
    HVar9 = unaff_ss;
    _local_a = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(pu_var5, 0x48));
    pass1_1008_3e94(
        (_local_a + 0xe),
        CONCAT13(u_var7, CONCAT12(u_var6, pu_var5)),
        CONCAT22(HVar9, pu_var8),
    );
    u_var4 = (_local_a >> 0x10);
    local_c = (_local_a + 10);
    local_e = (_local_a + 0xc);
    i_var2 = GetSystemMetrics16(4);
    i_var3 = i_var2 * PTR_LOOP_1050_5ef8 + 10;
    PTR_LOOP_1050_5ef8 = PTR_LOOP_1050_5ef8 + 1;
    _local_12 = CONCAT22(i_var3 + local_4, i_var3 + local_6);
    u_var4 = (param_1 >> 0x10);
    GetWindowRect16(CONCAT22(&local_1a, 0x1538), unaff_ss);
    if (local_e < ((local_14 - local_18) + local_12)) {
        _local_12 = _local_12 & 0xffff0000 | (0xfffe - ((local_14 - local_18) - local_e));
    }
    if (local_c < ((local_16 - local_1a) + local_10)) {
        _local_12 = _local_12 & 0xffff | (0xfffe - ((local_16 - local_1a) - local_c)) << 0x10;
    }
    SetWindowPos16(1, 0, 0, _local_12, (_local_12 >> 0x10), 0, (param_1 + 6));
    pp_var1 = (param_1 + 0x6c);
    (**pp_var1)(offset, param_1, u_var4);
    return;
}

pub fn set_dialog_item_txt_1040_ad14(in_struct_1: *mut Struct347) {
    set_dialog_item_text_1040_ae04(in_struct_1);
    return;
}

pub fn win_gui_fn_1040_ad24(param_1: *mut Struct124, param_2: u16, param_3: u16, param_4: u32) {
    if (param_3._2_2_ == 0xeb) {
        set_dialog_item_text_1040_ae04(CONCAT22(param_2, param_1));
    } else {
        if (param_3._2_2_ != 0x1f0) {
            win_gui_fn_1040_b54a(param_1, param_2, param_3_00, param_3);
            return;
        }
        msg_box_1040_ad66(CONCAT22(param_2, param_1));
    }
    return;
}

pub fn msg_box_1040_ad66(param_1: u32) {

    let in_dx: *mut Struct199;
    let mut unaff_ss: u16;
    let mut local_212: u32;
    let mut local_20e: u32;
    let mut local_20a: u16;
    let mut local_208: u16;
    let mut local_206: [u8; 258];
    let mut local_104: [u8; 258];

    process_struct_1000_179c(0x1000, in_dx);
    load_string_1010_84e0(
        ctx._g_struct_73_1050_14cc,
        (ctx._g_struct_73_1050_14cc >> 0x10),
        0x100,
        CONCAT22(unaff_ss, local_206),
        0x57b,
    );
    load_string_1010_84e0(
        ctx._g_struct_73_1050_14cc,
        (ctx._g_struct_73_1050_14cc >> 0x10),
        0x3ff,
        CONCAT22(in_dx, in_ax),
        0x7f3,
    );
    load_string_1010_84e0(
        ctx._g_struct_73_1050_14cc,
        (ctx._g_struct_73_1050_14cc >> 0x10),
        0x3ff,
        CONCAT22(unaff_ss, local_104),
        0x7f4,
    );
    process_string_1000_3cea(CONCAT22(in_dx, in_ax), CONCAT22(unaff_ss, local_104));
    MessageBox16(
        0,
        CONCAT22(unaff_ss, local_206),
        CONCAT22(in_dx, in_ax),
        (param_1 + 6),
    );
    error_check_1000_17ce(CONCAT22(in_dx, in_ax));
    return;
}

pub fn set_dialog_item_text_1040_ae04(param_1: *mut Struct347) {
    let mut u_var1: u32;
    let mut b_var2: bool;
    let mut u_var3: u16;
    let string_2: String;
    let string_3: String;
    let mut u_var4: i32;
    let mut u_var5: u16;
    let mut local_AX_349: u16;
    let lVar6: u32;
    let local_DL_13: u8;
    let mut local_DX_81: u16;
    let mut local_DX_110: u16;

    let mut local_DX_373: u16;
    let local_struct_1: *mut Struct347;
    let plVar7: *mut long;
    let mut local_SI__1: u16;
    let mut local_DI__1: u16;
    let mut ctx.es_reg: u16;

    let pp_var8: *mut pass1_struct_1;
    let mut u_var9: u32;
    let mut u_var10: u32;
    let mut local_124: u32;
    let mut local_120: u16;
    let mut local_11e: u16;
    let mut local_118: u16;
    let mut local_116: u32;
    let mut local_112: u16;
    let mut local_110: u16;
    let mut local_10e: u16;
    let mut local_10c: u16;
    let mut local_10a: u16;
    let mut local_108: u16;
    let mut local_106: u16;
    let mut local_104: u16;
    let mut string_1: [u8; 256];
    let mut temp_5f97b0c4d6: u32;

    pp_var8 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(local_SI__1, 3));
    ctx.es_reg = (param_1 >> 0x10);
    local_struct_1 = param_1;
    pass1_1010_c3c2(
        pp_var8,
        (pp_var8 >> 0x10),
        CONCAT22(ctx.stack_seg_reg, string_1),
        local_struct_1.field_0x94,
    );
    SetDlgItemText16(
        CONCAT22(ctx.stack_seg_reg, string_1),
        0x1778,
        local_struct_1.window_handle,
    );
    u_var9 = pass1_1030_73a8(local_struct_1.field_0x94);
    string_3 = (u_var9 + 0x20);
    u_var3 = (u_var9 >> 0x10);
    u_var10 = pass1_1030_8326();
    local_DX_110 = (u_var10 >> 0x10);
    u_var9 = 0;
    local_116 = 0;
    b_var2 = false;
    local_118 = 0;
    while (true) {
        local_AX_349 = u_var9;
        if (9 < local_118) {
            break;
        }
        u_var4 = (string_3 + local_118 * 0xc + 2) | (string_3 + local_118 * 0xc);
        u_var9 = u_var4;
        if (u_var4 != 0) {
            plVar7 = (string_3 + local_118 * 0xc);
            u_var5 = big_switch_statement_1020_c222((plVar7 + 1));
            SetDlgItemText16(
                CONCAT22(local_DX_110, u_var5),
                local_116 + 0x1d2,
                local_struct_1.window_handle,
            );
            unsafe {
                lVar6 = *plVar7 - (u_var10 & 0xffff);
            }
            wsprintf16(
                string_1,
                CONCAT13(0x5e, CONCAT12(0xf4, ctx.stack_seg_reg)),
                CONCAT13((lVar6 >> 8), CONCAT12(lVar6, 0x1050)),
            );
            _local_120 = CONCAT22(ctx.stack_seg_reg, string_1);
            SetDlgItemText16(_local_120, local_116 + 0x1dc, local_struct_1.window_handle);
            u_var1 = local_struct_1.field_0x98;
            local_124._0_2_ = u_var1;
            local_124._2_2_ = (u_var1 >> 0x10);
            wsprintf_1010_8c96(
                local_124,
                local_124._2_2_,
                string_1,
                ctx.stack_seg_reg,
                plVar7,
                u_var3,
            );
            u_var9 = ZEXT24(string_1);
            _local_120 = CONCAT22(ctx.stack_seg_reg, string_1);
            local_DX_110 = ctx.dx_reg;
            SetDlgItemText16(_local_120, local_116 + 0x1e6, local_struct_1.window_handle);
            local_116 = CONCAT22(1, local_116 + 1);
            b_var2 = true;
        }
        local_118 = local_118 + 1;
    }
    if (!b_var2) {
        load_string_1010_847e(
            ctx._g_struct_73_1050_14cc,
            (ctx._g_struct_73_1050_14cc >> 0x10),
            0x531,
        );
        SetDlgItemText16(
            CONCAT22(local_DX_373, local_AX_349),
            0x1d2,
            local_struct_1.window_handle,
        );
    }
    return;
}

pub fn set_win_placement_1010_0070(param_1: u32, param_2: i32, param_3: HWND16) {
    let pp_var1: fn();
    let mut u_var2: u16;
    let pu_var3: *mut u32;
    let mut unaff_ss: u16;
    let lVar4: u32;
    let pu_var5: *mut u16;
    let mut local_22: u16;
    let mut local_20: u32;
    let mut local_1c: u16;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    local_18 = 0x16;
    _local_16 = 0;
    local_12 = 0;
    local_10 = 0;
    local_e = 0;
    local_c = 0;
    local_a = 0;
    local_8 = 0;
    local_6 = 0;
    local_4 = 0;
    pu_var5 = &local_18;
    GetWindowPlacement16();
    if ((local_10 == 0xffff) || (param_2 != 0)) {
        _local_16 = 0x50001;
        lVar4 = GetWindowLong16(0, param_3);
        u_var2 = (lVar4 >> 0x10);
        pu_var3 = (lVar4 + 0xe0);
        unsafe {
            pp_var1 = (*pu_var3 + 0x38);
        }
        (**pp_var1)(offset, pu_var3, (lVar4 + 0xe2), pu_var5);
        pass1_1010_01f8(param_1, CONCAT22(unaff_ss, &local_18), pu_var3);
        SetWindowPlacement16();
    }
    return;
}

pub fn set_win_placement_1010_010e(param_1: u16, param_2: u16, param_3: HWND16) {
    let pp_var1: fn();
    let mut i_var2: i32;
    let pu_var3: *mut u16;
    let mut u_var4: u16;
    let pu_var5: *mut u32;

    let lVar6: u32;
    let pu_var7: *mut u16;
    let mut local_26: u16;
    let mut local_24: u16;
    let mut local_22: u16;
    let mut local_20: u32;
    let mut local_1c: u16;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    local_18 = 0x16;
    local_16 = 0;
    local_14 = 0;
    local_12 = 0;
    local_10 = 0;
    local_e = 0;
    local_c = 0;
    local_a = 0;
    local_8 = 0;
    local_6 = 0;
    local_4 = 0;
    pu_var7 = &local_18;
    GetWindowPlacement16();
    if (local_a == 0xffff) {
        lVar6 = GetWindowLong16(0, param_1);
        u_var4 = (lVar6 >> 0x10);
        pu_var5 = (lVar6 + 0xe0);
        unsafe {
            pp_var1 = (*pu_var5 + 0x1c);
        }
        (**pp_var1)(offset, pu_var5, (lVar6 + 0xe2), pu_var7);
        i_var2 = pu_var5;
        pu_var3 = (pu_var5 & 0xffff | ctx.dx_reg << 0x10);
        local_14 = 9;
        unsafe {
            local_a = *pu_var3;
        }
        local_8 = (i_var2 + 2);
        unsafe {
            local_6 = (i_var2 + 4) + *pu_var3;
        }
        local_4 = (i_var2 + 2) + (i_var2 + 6);
        SetWindowPlacement16(offset, &local_18);
    }
    return;
}

pub fn enum_child_windows_1010_01be() {
    let unaff_cs: HANDLE16;
    let pvVar1: &mut Vec<u8>;
    let mut local_6: u16;
    let mut local_4: u16;

    if (ctx.PTR_LOOP_1050_0010 == 0x0) {
        pvVar1 = MakeProcInstance16(unaff_cs, CONCAT22(0x240, ctx.g_h_instance_1050_038c));
        EnumChildWindows16(0, pvVar1, (pvVar1 >> 0x10));
        FreeProcInstance16(CONCAT22(pvVar1, 0x1538));
    }
    return;
}

pub fn win_gui_fn_1010_2a32(param_1: i32, uparam_2_00: i32, param_2: &HFILE16) {
    let pi_var1: *mut i32;
    let pc_var2: String;
    let pbVar3: Vec<u8>;
    let left: u16;
    let top: u16;
    let right: u16;
    let bottom: u16;
    let in_struct_104_ptr: *mut Struct104;
    let mut u_var4: u32;
    let mut entry: string;
    let mut string: string;
    let mut filename: string;
    let mut u_var5: u32;
    let mut bVar6: u8;
    let pu_var7: *mut u32;
    let pu_var8: *mut u32;
    let ppc_var9: fn();
    let pc_var10: *mut code;
    let h_gdi_obj: HGDIOBJ16;
    let pu_var11: *mut u16;
    let u_var12: u8;
    let u_var13: u8;
    let u_var14: u8;
    let u_var15: u8;
    let u_var16: u8;
    let u_var17: u8;
    let u_var18: u8;
    let u_var19: u8;
    let u_var20: u8;
    let u_var21: u8;
    let u_var22: u8;
    let u_var23: u8;
    let u_var24: u8;
    let u_var25: u8;
    let local_AX_123: *mut Struct382;
    let mut u_var26: i32;
    let HVar27: HDC16;
    let pu_var28: Vec<u8>;
    let h_gdi_obj_00: HPALETTE16;
    let pu_var29: *mut u32;
    let mut u_var30: u16;
    let BVar31: bool;
    let mut bVar32: u8;
    let mut in_dx: i32;


    let struct_a: *mut Struct199;
    let ctx.dx_reg: *mut Struct199;
    let in_bx: *mut u32;
    let mut i_var33: i32;
    let mut unaff_bp: u16;
    let mut unaff_si: i32;
    let mut i_var34: i32;
    let mut unaff_DI: u16;
    let mut unaff_es: u16;
    let mut unaff_cs: u16;
    let mut u_var35: u16;
    let mut unaff_ss: u16;
    let mut bVar36: bool;
    let mut bVar37: u8;
    let mut u_var38: u32;
    let mut in_stack_00000000: i32;
    let mut in_stack_00000002: u16;
    let mut local_36: u16;
    let mut local_2e: u32;
    let mut local_2a: u16;
    let mut local_26: u32;
    let mut local_22: u16;
    let mut iStack32: i32;
    let HStack30: HGDIOBJ16;
    let HStack28: HGDIOBJ16;
    let pcStack26: String;
    let pcStack24: String;
    let mut local_16: u32;
    let uStack18: u8;
    let uStack17: u8;
    let uStack16: u8;
    let uStack15: u8;
    let uStack14: u8;
    let uStack13: u8;
    let u_stack12: u8;
    let uStack11: u8;
    let uStack10: u8;
    let uStack9: u8;
    let mut bStack8: u8;
    let uStack7: u8;
    let uStack6: u8;
    let uStack5: u8;
    let mut local_4: u16;
    let uStack2: u8;
    let uStack1: u8;

    i_var33 = param_1;
    uStack9 = 0xfe;
    u_var13 = uStack9;
    uStack6 = 0xfe;
    u_var21 = uStack6;
    uStack2 = unaff_bp;
    uStack1 = (unaff_bp >> 8);
    local_4._0_1_ = 0;
    local_4._1_1_ = 0;
    uStack10 = param_1;
    u_var12 = uStack10;
    uStack9 = (param_1 >> 8);
    u_var15 = uStack9;
    bStack8 = param_2_00;
    u_var18 = bStack8;
    uStack7 = (param_2_00 >> 8);
    u_var17 = uStack7;
    if ((param_2._2_2_ + 0xec76 & 3) != 0) {}
    // goto LAB_1010_2ad8;
    local_AX_123 = (param_2._2_2_ + 0xec76 >> 1);
    bVar36 = local_AX_123 < (s_version__d__d_1050_0012 + 10);
    if ((s_version__d__d_1050_0012 + 10) < local_AX_123) {}
    // goto LAB_1010_2ad8;
    unaff_cs = 0x1010;
    uStack6 = SUB21(in_bx, 0);
    u_var22 = uStack6;
    uStack5 = (in_bx >> 8);
    u_var24 = uStack5;
    uStack6 = unaff_ss;
    u_var23 = uStack6;
    uStack5 = (unaff_ss >> 8);
    u_var25 = uStack5;
    uStack10 = SUB41(param_2, 0);
    u_var14 = uStack10;
    uStack9 = (param_2 >> 8);
    u_var16 = uStack9;
    bStack8 = (param_2 >> 0x10);
    u_var19 = bStack8;
    uStack7 = (param_2 >> 0x18);
    u_var20 = uStack7;
    uStack6 = u_var22;
    uStack5 = u_var24;
    match (local_AX_123) {
        // default:
        // goto switchD_1010_2ab5_caseD_0;
        0x1 | 0x3 | 0xb => {
            local_AX_123.field_0xa = in_bx;
        }
        0x9 | 0xf | 0x15 | 0x1b => {
            local_AX_123.field_0xa = in_bx;
            local_AX_123.field_0x10 = in_bx;
            local_AX_123.field_0xc = in_bx;
            return;
        }
        0x5 => {
            u_stack12 = 0x10;
            uStack11 = 0x10;
            uStack14 = 0x35;
            uStack13 = 0x40;
            BVar31 = write_to_file_1008_7e1c(
                param_2,
                ZEXT24(in_bx),
                CONCAT13(
                    (in_stack_00000000 >> 8),
                    CONCAT12(in_stack_00000000, unaff_bp),
                ),
            );
            if (BVar31 != 0) {
                return;
            }
            ctx.g_u16_1050_0310 = 0x6d0;
            return;
        }
        0x6 => {
            local_4._0_1_ = 0;
            // goto LAB_1010_2ad8;
        }
        0x7 => {
            uStack6 = 0x10;
            uStack5 = 0x10;
            bStack8 = 0x34;
            uStack7 = 0x48;
            unsafe {
                ppc_var9 = *in_bx;
            }
            (**ppc_var9)();
            i_var33 = param_2 + 0x105;
            uStack6 = i_var33;
            uStack5 = (i_var33 >> 8);
            uStack10 = ctx._g_struct_73_1050_14cc;
            uStack9 = (ctx._g_struct_73_1050_14cc >> 8);
            bStack8 = (ctx._g_struct_73_1050_14cc >> 0x10);
            uStack7 = (ctx._g_struct_73_1050_14cc >> 0x18);
            u_stack12 = 0x10;
            uStack11 = 0x10;
            uStack14 = 0x45;
            uStack13 = 0x48;
            win_gui_fn_1010_8170();
            i_var34 = param_2 * 4;
            (param_1 + i_var34 + 0x26) = i_var33;
            (param_1 + i_var34 + 0x28) = ctx.dx_reg;
            uStack6 = 0x50;
            uStack5 = 0x10;
            bStack8 = 0x80;
            uStack7 = 0x13;
            u_stack12 = 0;
            uStack11 = 0;
            uStack10 = 0;
            uStack9 = 0;
            uStack16 = 0;
            uStack15 = 0;
            uStack14 = 0;
            uStack13 = 0;
            in_struct_104_ptr = (param_1 + 0x26 + i_var34);
            local_16._2_1_ = SUB41(in_struct_104_ptr, 0);
            local_16._3_1_ = (in_struct_104_ptr >> 8);
            uStack18 = (in_struct_104_ptr >> 0x10);
            uStack17 = (in_struct_104_ptr >> 0x18);
            local_16._0_2_ = 0x1010;
            pcStack24 = &PTR_LOOP_1050_486c;
            u_var38 = process_struct_1008_4772(in_struct_104_ptr);
            uStack18 = u_var38;
            uStack17 = (u_var38 >> 8);
            uStack16 = (u_var38 >> 0x10);
            uStack15 = (u_var38 >> 0x18);
            local_16._0_2_ = &ctx.PTR_LOOP_1050_1008;
            pcStack24 = 0x4879;
            local_16._2_1_ = uStack18;
            local_16._3_1_ = uStack17;
            uStack18 = uStack16;
            uStack17 = uStack15;
            HVar27 = CreateDC16(
                u_var38,
                (u_var38 & 0xff000000 | CONCAT12(uStack16, (u_var38 >> 0x10))),
                CONCAT13(uStack11, CONCAT12(u_stack12, CONCAT11(uStack13, uStack14))),
                CONCAT13(uStack7, CONCAT12(bStack8, CONCAT11(uStack9, uStack10))),
            );
            local_16._2_1_ = HVar27;
            local_16._3_1_ = (HVar27 >> 8);
            u_var5 = (_PTR_LOOP_1050_4230 + 0xe);
            pcStack24 = u_var5;
            local_16._0_2_ = (u_var5 >> 0x10);
            pcStack26 = (&local_16 + 2);
            bStack8 = pcStack26;
            uStack7 = (pcStack26 >> 8);
            u_stack12 = u_var5;
            uStack11 = (u_var5 >> 8);
            uStack10 = (u_var5 >> 0x10);
            uStack9 = (u_var5 >> 0x18);
            uStack14 = 0x38;
            uStack13 = 0x15;
            uStack16 = 0x97;
            uStack15 = 0x48;
            uStack6 = u_var23;
            uStack5 = u_var25;
            realize_palette_1008_4e08();
            uStack6 = local_16._2_1_;
            uStack5 = local_16._3_1_;
            bStack8 = local_4;
            uStack7 = local_4._1_1_;
            uStack10 = 8;
            uStack9 = 0x10;
            u_stack12 = 0xa5;
            uStack11 = 0x48;
            HStack28 = SelectObject16(
                CONCAT11(local_4._1_1_, local_4),
                CONCAT11(local_16._3_1_, local_16._2_1_),
            );
            uStack6 = local_16._2_1_;
            uStack5 = local_16._3_1_;
            bStack8 = local_16._2_1_;
            uStack7 = local_16._3_1_;
            uStack10 = 0x38;
            uStack9 = 0x15;
            u_var35 = SUB42(offset, 0);
            u_stack12 = 0xb3;
            uStack11 = 0x48;
            HStack30 = SelectObject16(
                CONCAT11(local_16._3_1_, local_16._2_1_),
                CONCAT11(local_16._3_1_, local_16._2_1_),
            );
            iStack32 = 0;
            while (true) {
                u_var13 = uStack15;
                u_var12 = uStack16;
                pi_var1 = (param_1 + 0x74);
                uStack16 = u_var35;
                uStack15 = (u_var35 >> 8);
                if (unsafe { *pi_var1 == iStack32 } || unsafe { *pi_var1 < iStack32 }) {
                    break;
                }
                uStack6 = 8;
                uStack5 = 0;
                i_var33 = (iStack32 * 0x10 + param_2) * 8;
                i_var34 = i_var33 + (param_1 + 0x70);
                u_var35 = (param_1 + 0x72);
                bStack8 = u_var35;
                uStack7 = (u_var35 >> 8);
                uStack10 = i_var34;
                uStack9 = (i_var34 >> 8);
                pu_var28 = &uStack14;
                uStack14 = SUB21(pu_var28, 0);
                uStack13 = (pu_var28 >> 8);
                u_var35 = 0x1000;
                uStack18 = 0xe1;
                uStack17 = 0x48;
                u_stack12 = u_var23;
                uStack11 = u_var25;
                pass1_fn_1000_484c(
                    CONCAT13(u_var25, CONCAT12(u_var23, pu_var28)),
                    CONCAT13(uStack7, CONCAT12(bStack8, i_var34)),
                    8,
                );
                if (pu_var28 != 0x0) {
                    uStack6 = local_16._2_1_;
                    uStack5 = local_16._3_1_;
                    u_var5 = (param_1 + 0x70);
                    u_var35 = (u_var5 >> 0x10);
                    i_var34 = u_var5;
                    left = (i_var34 + i_var33);
                    bStack8 = left;
                    uStack7 = (left >> 8);
                    i_var33 = i_var33 + i_var34;
                    top = (i_var33 + 2);
                    uStack10 = top;
                    uStack9 = (top >> 8);
                    right = (i_var33 + 4);
                    u_stack12 = right;
                    uStack11 = (right >> 8);
                    bottom = (i_var33 + 6);
                    uStack14 = bottom;
                    uStack13 = (bottom >> 8);
                    uStack16 = 0;
                    uStack15 = 0x10;
                    u_var35 = SUB42(offset, 0);
                    uStack18 = 8;
                    uStack17 = 0x49;
                    Rectangle16(
                        bottom,
                        right,
                        top,
                        left,
                        CONCAT11(local_16._3_1_, local_16._2_1_),
                    );
                }
                iStack32 = iStack32 + 1;
            }
            uStack6 = local_16._2_1_;
            uStack5 = local_16._3_1_;
            bStack8 = pcStack26;
            uStack7 = (pcStack26 >> 8);
            uStack10 = 0;
            uStack9 = 0;
            uStack14 = 0x24;
            uStack13 = 0x49;
            u_stack12 = uStack16;
            uStack16 = u_var12;
            uStack11 = uStack15;
            uStack15 = u_var13;
            h_gdi_obj_00 = SelectPalette16(0, pcStack26, CONCAT11(local_16._3_1_, local_16._2_1_));
            local_4._0_1_ = h_gdi_obj_00;
            local_4._1_1_ = (h_gdi_obj_00 >> 8);
            uStack6 = 0x38;
            uStack5 = 0x15;
            bStack8 = 0x2a;
            uStack7 = 0x49;
            DeleteObject16(h_gdi_obj_00);
            uStack2 = local_16._2_1_;
            uStack1 = local_16._3_1_;
            local_4._0_1_ = HStack28;
            local_4._1_1_ = (HStack28 >> 8);
            uStack6 = 0x38;
            uStack5 = 0x15;
            bStack8 = 0x35;
            uStack7 = 0x49;
            SelectObject16(HStack28, CONCAT11(local_16._3_1_, local_16._2_1_));
            uStack2 = local_16._2_1_;
            uStack1 = local_16._3_1_;
            local_4._0_1_ = HStack30;
            local_4._1_1_ = (HStack30 >> 8);
            uStack6 = 0x38;
            uStack5 = 0x15;
            bStack8 = 0x40;
            uStack7 = 0x49;
            SelectObject16(HStack30, CONCAT11(local_16._3_1_, local_16._2_1_));
            uStack2 = local_16._2_1_;
            uStack1 = local_16._3_1_;
            local_4._0_1_ = 0x38;
            local_4._1_1_ = 0x15;
            uStack6 = 0x48;
            uStack5 = 0x49;
            DeleteDC16(CONCAT11(local_16._3_1_, local_16._2_1_));
            h_gdi_obj = CONCAT11(local_4._1_1_, local_4);
            uStack2 = 0x38;
            uStack1 = 0x15;
            local_4._0_1_ = 0x50;
            local_4._1_1_ = 0x49;
            DeleteObject16(h_gdi_obj);
            return;
        }
        0x8 => {
            local_4._0_1_ = 3;
            // goto LAB_1010_2ad8;
        }
        0xd => {
            pbVar3 = (&local_AX_123.field_0x0 + unaff_si);
            unsafe {
                bVar37 = *pbVar3;
                bVar6 = *pbVar3 + in_dx;
                *pbVar3 = bVar6 + bVar36;
            }
            pu_var7 = (CARRY1(bVar37, in_dx) || CARRY1(bVar6, bVar36));
            pu_var8 = in_bx + -0x404;
            bVar37 = in_bx < 0x1010 || pu_var8 < pu_var7;
            pu_var29 = (pu_var8 - pu_var7);
            pc_var10 = swi(4);
            if (SBORROW2(in_bx, 0x1010) != SBORROW2(pu_var8, pu_var7)) {
                unsafe {
                    (*pc_var10)();
                }
                in_dx = ctx.dx_reg;
            }
            bVar36 = pu_var29 < 0x1010 || pu_var29 + -0x404 < bVar37;
            pbVar3 = (&local_AX_123.field_0x0 + unaff_si);
            unsafe {
                bVar37 = *pbVar3;
                bVar32 = in_dx;
                bVar6 = *pbVar3;
                *pbVar3 = bVar6 + bVar32 + bVar36;
                pc_var2 = (&local_AX_123.field_0x0 + unaff_si);
                *pc_var2 =
                    *pc_var2 + bVar32 + (CARRY1(bVar37, bVar32) || CARRY1(bVar6 + bVar32, bVar36));
            }
            bStack8 = (&uStack2 >> 8);
            u_stack12 = in_stack_00000000;
            uStack11 = (in_stack_00000000 >> 8);
            uStack10 = in_stack_00000002;
            uStack13 = uStack1;
            uStack15 = local_4._1_1_;
            uStack14 = uStack2;
            uStack17 = 0x10;
            uStack16 = 0x10;
            local_16._3_1_ = 0x4d;
            uStack18 = 0x50;
            uStack9 = u_var13;
            pass1_1018_4cda(
                CONCAT11(uStack2, local_4._1_1_),
                CONCAT13(uStack10, CONCAT12(uStack11, CONCAT11(u_stack12, uStack1))),
            );
            i_var33 = CONCAT11(uStack2, local_4._1_1_);
            u_stack12 = in_stack_00000000;
            pu_var11 = CONCAT13(u_stack12, CONCAT12(uStack1, i_var33));
            unsafe {
                *pu_var11 = (s_SinternalPutBldg2_site_0x_08lx__1050_5099 + 1);
            }
            (i_var33 + 2) = 0x1010;
            uStack11 = 0xb3;
            uStack10 = 1;
            uStack13 = uStack1;
            uStack15 = local_4._1_1_;
            uStack14 = uStack2;
            uStack17 = 0x18;
            uStack16 = 0x10;
            local_16._3_1_ = 0x65;
            uStack18 = 0x50;
            pass1_1018_4dce(CONCAT13(u_stack12, CONCAT12(uStack1, i_var33)), 0x1b3);
            _PTR_LOOP_1050_4230 = CONCAT13(
                u_stack12,
                CONCAT12(uStack1, CONCAT11(uStack2, local_4._1_1_)),
            );
            return;
        }
        0xe => (param_1 + 0x20) = param_2,
        0x11 => {
            loop {
                // WARNING: Do nothing block with infinite loop
            }
        }
        0x12 => {
            PTR_LOOP_1050_10c6 = (0 < param_2);
            PTR_LOOP_1050_1142 = (2 < param_2)
        }
        0x13 => {
            uStack5 = (&uStack2 >> 8);
            i_var33 = param_1 * 8 + in_stack_00000000;
            if (((((i_var33 + 0x22) != 0) || ((i_var33 + 0x24) != 0)) || ((i_var33 + 0x26) != 0))
                || ((i_var33 + 0x28) != 0))
            {
                i_var33 = param_1 * 8 + in_stack_00000000;
                u_var5 = (i_var33 + 0x22);
                u_var4 = (i_var33 + 0x26);
                uStack14 = u_var4;
                uStack13 = (u_var4 >> 8);
                u_stack12 = (u_var4 >> 0x10);
                uStack11 = (u_var4 >> 0x18);
                uStack18 = u_var5;
                uStack17 = (u_var5 >> 8);
                uStack16 = (u_var5 >> 0x10);
                uStack15 = (u_var5 >> 0x18);
                local_16._2_1_ = 0x50;
                local_16._3_1_ = 0x10;
                local_16._0_2_ = s__d__d__d__d_1050_14ae;
                u_var4 = (in_stack_00000000 + 0xe);
                pcStack26 = u_var4;
                pcStack24 = (u_var4 >> 0x10);
                HStack28 = 0x1010;
                HStack30 = 0x627c;
                uStack6 = u_var21;
                string_fn_1000_3f9c(
                    pcStack26,
                    pcStack24,
                    s__d__d__d__d_1050_14ae,
                    &ctx.g_alloc_addr_1050_1050,
                    u_var5,
                );
                u_stack12 = 0x50;
                uStack11 = 0x10;
                uStack14 = 0xb8;
                uStack13 = 0x13;
                entry = (param_1 * 4 + 0x1446);
                uStack18 = SUB41(entry, 0);
                uStack17 = (entry >> 8);
                uStack16 = (entry >> 0x10);
                uStack15 = (entry >> 0x18);
                string = (in_stack_00000000 + 0xe);
                local_16._0_2_ = string;
                local_16._2_1_ = (string >> 0x10);
                local_16._3_1_ = (string >> 0x18);
                filename = (in_stack_00000000 + 10);
                pcStack26 = filename;
                pcStack24 = (filename >> 0x10);
                HStack28 = 0x1000;
                HStack30 = 0x62a0;
                WritePrivateProfileString16(filename, string, entry, s_windows_1050_13b8);
            }
            return;
        }
        0x14 => (param_1 + 0x24) = param_2,
        0x17 => {
            struct_a = (in_dx + -1);
            uStack6 = unaff_DI;
            uStack5 = (unaff_DI >> 8);
            pbVar3 = (&local_AX_123.field_0x0 + unaff_si);
            unsafe {
                *pbVar3 = *pbVar3 | struct_a;
            }
            (param_1 + 0x12) = in_bx;
            (param_1 + 0x14) = struct_a;
            local_2a = 0;
            loop {
                uStack10 = unaff_cs;
                u_var12 = uStack10;
                uStack9 = (unaff_cs >> 8);
                u_var13 = uStack9;
                if (local_36 <= local_2a) {
                    uStack10 = 2;
                    uStack9 = 0;
                    bStack8 = 0;
                    uStack7 = 0;
                    u_var26 = param_1 + 0x1a;
                    uStack14 = u_var26;
                    uStack13 = (u_var26 >> 8);
                    uStack11 = ((param_2_00 << 0x10) >> 0x18);
                    local_16._0_2_ = 0x9f2f;
                    local_16._2_1_ = u_var12;
                    local_16._3_1_ = u_var13;
                    uStack18 = u_var14;
                    uStack17 = u_var16;
                    uStack16 = u_var19;
                    uStack15 = u_var20;
                    u_stack12 = u_var18;
                    BVar31 = read_file_1008_7dee(
                        param_2,
                        ((param_2_00 & 0xff00) << 0x10 | CONCAT12(u_var18, u_var26)),
                        2,
                    );
                    if (BVar31 != 0) {
                        uStack10 = 2;
                        uStack9 = 0;
                        bStack8 = 0;
                        uStack7 = 0;
                        u_var26 = param_1 + 0x1c;
                        uStack14 = u_var26;
                        uStack13 = (u_var26 >> 8);
                        uStack11 = ((param_2_00 << 0x10) >> 0x18);
                        local_16._2_1_ = 8;
                        local_16._3_1_ = 0x10;
                        local_16._0_2_ = 0x9f4a;
                        uStack18 = u_var14;
                        uStack17 = u_var16;
                        uStack16 = u_var19;
                        uStack15 = u_var20;
                        u_stack12 = u_var18;
                        BVar31 = read_file_1008_7dee(
                            param_2,
                            ((param_2_00 & 0xff00) << 0x10 | CONCAT12(u_var18, u_var26)),
                            2,
                        );
                        if (BVar31 != 0) {
                            uStack10 = 2;
                            uStack9 = 0;
                            bStack8 = 0;
                            uStack7 = 0;
                            u_var26 = param_1 + 0x1e;
                            uStack14 = u_var26;
                            uStack13 = (u_var26 >> 8);
                            uStack11 = ((param_2_00 << 0x10) >> 0x18);
                            local_16._2_1_ = 8;
                            local_16._3_1_ = 0x10;
                            local_16._0_2_ = 0x9f65;
                            uStack18 = u_var14;
                            uStack17 = u_var16;
                            uStack16 = u_var19;
                            uStack15 = u_var20;
                            u_stack12 = u_var18;
                            BVar31 = read_file_1008_7dee(
                                param_2,
                                ((param_2_00 & 0xff00) << 0x10 | CONCAT12(u_var18, u_var26)),
                                2,
                            );
                            if (BVar31 != 0) {
                                return;
                            }
                        }
                    }
                    ctx.g_u16_1050_0310 = 0x6d2;
                    return;
                }
                bStack8 = 8;
                uStack7 = 0;
                u_stack12 = 0xe4;
                uStack11 = 0x9e;
                u_var30 = local_36;
                process_struct_1000_179c(8, struct_a);
                local_2e = CONCAT22(struct_a, u_var30);
                if ((struct_a | u_var30) == 0) {
                    local_26 = 0;
                } else {
                    local_2e = ctx.s_1_1050_389a;
                    (u_var30 + 2) = &ctx.PTR_LOOP_1050_1008;
                    local_2e = 0xa1c4;
                    (u_var30 + 2) = 0x1010;
                    local_26 = local_2e;
                }
                uStack10 = 2;
                uStack9 = 0;
                bStack8 = 0;
                uStack7 = 0;
                uStack14 = SUB21(&local_22, 0);
                uStack13 = (&local_22 >> 8);
                local_16._2_1_ = 0;
                local_16._3_1_ = 0x10;
                local_16._0_2_ = 0x9e69;
                uStack18 = u_var14;
                uStack17 = u_var16;
                uStack16 = u_var19;
                uStack15 = u_var20;
                u_stack12 = u_var23;
                uStack11 = u_var25;
                BVar31 = read_file_1008_7dee(
                    param_2,
                    CONCAT13(u_var25, CONCAT12(u_var23, &local_22)),
                    2,
                );
                i_var33 = local_26;
                uStack10 = (local_26 >> 0x10);
                u_var15 = uStack10;
                uStack9 = (local_26 >> 0x18);
                u_var17 = uStack9;
                u_stack12 = local_26;
                u_var12 = u_stack12;
                uStack11 = (local_26 >> 8);
                u_var13 = uStack11;
                if (BVar31 == 0) {
                    break;
                }
                uStack10 = 2;
                uStack9 = 0;
                bStack8 = 0;
                uStack7 = 0;
                u_var26 = i_var33 + 6;
                uStack14 = u_var26;
                uStack13 = (u_var26 >> 8);
                u_stack12 = ((local_26 & 0xffff0000) >> 0x10);
                uStack11 = ((local_26 & 0xffff0000) >> 0x18);
                local_16._2_1_ = 8;
                local_16._3_1_ = 0x10;
                local_16._0_2_ = 0x9e82;
                uStack18 = u_var14;
                uStack17 = u_var16;
                uStack16 = u_var19;
                uStack15 = u_var20;
                BVar31 = read_file_1008_7dee(
                    param_2,
                    (local_26 & 0xff000000 | CONCAT12(u_stack12, u_var26)),
                    2,
                );
                if (BVar31 == 0) {
                    break;
                }
                bStack8 = local_22;
                uStack7 = (local_22 >> 8);
                uStack14 = 8;
                uStack13 = 0x10;
                unaff_cs = SUB42(&ctx.PTR_LOOP_1050_1008, 0);
                uStack16 = 0xb6;
                uStack15 = 0x9e;
                u_stack12 = u_var14;
                uStack11 = u_var16;
                uStack10 = u_var19;
                uStack9 = u_var20;
                switch_statement_1008_73ea(param_2, param_2._2_2_, local_22);
                (i_var33 + 4) = BVar31;
                u_var5 = (param_1 + 0x12);
                uStack14 = u_var5;
                uStack13 = (u_var5 >> 8);
                u_stack12 = (u_var5 >> 0x10);
                uStack11 = (u_var5 >> 0x18);
                uStack16 = 8;
                uStack15 = 0x10;
                uStack18 = 0xd2;
                uStack17 = 0x9e;
                ppc_var9 = ((param_1 + 0x12) + 4);
                uStack10 = u_var12;
                uStack9 = u_var13;
                bStack8 = u_var15;
                uStack7 = u_var17;
                (**ppc_var9)();
                local_2a = local_2a + 1;
                struct_a = ctx.dx_reg;
            }
            if (local_26 == 0) {
                ctx.g_u16_1050_0310 = 0x6d2;
                return;
            }
            bStack8 = 1;
            uStack7 = 0;
            uStack14 = 8;
            uStack13 = 0x10;
            uStack16 = 0xa6;
            uStack15 = 0x9e;
            ppc_var9 = local_26;
            local_16._0_2_ = i_var33;
            local_16._2_1_ = u_var15;
            local_16._3_1_ = u_var17;
            u_stack12 = u_var12;
            uStack11 = u_var13;
            uStack10 = u_var15;
            uStack9 = u_var17;
            (**ppc_var9)();
            ctx.g_u16_1050_0310 = 0x6d2;
            return;
        }
        0x18 => {
            local_4._0_1_ = 2;
            // goto LAB_1010_2ad8;
        }
        0x19 => {
            uStack6 = 0x3c;
            uStack5 = 0;
            bStack8 = 0x2a;
            uStack7 = 0;
            uStack10 = 8;
            uStack9 = 0;
            uStack16 = 0x10;
            uStack15 = 0x10;
            uStack18 = 0x40;
            uStack17 = 0x6e;
            uStack14 = u_var12;
            uStack13 = u_var15;
            u_stack12 = u_var18;
            uStack11 = u_var17;
            u_var30 = pass1_1010_6ca2(CONCAT13(u_var17, CONCAT12(u_var18, param_1)), 8);
            if (u_var30 != 0) {
                param_1 = 0x1a;
                uStack2 = 0x52;
                uStack1 = 0x6e;
                pass1_1010_715c(CONCAT22(0x1a, i_var33), 0x1a);
            }
            if (param_2 == 0x2c) {
                uStack2 = 99;
                uStack1 = 0x6e;
                pass1_1010_715c(CONCAT22(0x1d, param_1), 0x1d);
            }
            uStack2 = 0x5a;
            uStack1 = 0;
            local_4._0_1_ = 0x10;
            local_4._1_1_ = 0x10;
            uStack6 = 0x74;
            uStack5 = 0x6e;
            u_var30 = pass1_1010_6ca2(0x5a, 2);
            if (u_var30 != 0) {
                uStack2 = 0x27;
                uStack1 = 0x6d;
                pass1_1010_715c(0x1c005a, 0x1c);
            }
            return;
        }
        0x1a => {
            (param_1 + 0x26) = param_2;
        }
    }
    local_4._0_1_ = 1;
    // LAB_1010_2ad8:
    local_4._1_1_ = 0;
    if ((local_4 == 1) || (local_4 == 2)) {
        if (local_4 == 1) {
            param_2 = ((param_1 + 0x20) + (param_1 + 0x22) + (param_1 + 0x24) + (param_1 + 0x26));
        }
        if (param_2 != 0) {
            u_var26 = param_2 / 2 + 1;
            if (5 < u_var26) {
                u_var26 = 5;
            }
            param_2 = u_var26;
            if ((local_4 == 1) && (PTR_LOOP_1050_10c6 != 0x0)) {
                if (4 < u_var26) {
                    u_var26 = 4;
                }
                param_2 = u_var26;
            }
        }
    }
    (local_4 * 0x7c + 0xed6) = param_2;
    uStack6 = 0xc;
    uStack5 = 0;
    u_stack12 = unaff_cs;
    uStack11 = (unaff_cs >> 8);
    uStack14 = 0x4b;
    uStack13 = 0x2b;
    uStack10 = u_var12;
    uStack9 = u_var15;
    bStack8 = u_var18;
    uStack7 = u_var17;
    pass1_1010_1f62(CONCAT13(u_var17, CONCAT12(u_var18, param_1)), 0xc);
    // switchD_1010_2ab5_caseD_0:
    return;
}

pub fn destroy_win_1010_2fa0(param_1: *mut Struct340) {
    let pu_var1: *mut u16;
    let mut u_var2: u32;
    let local_bx_7: *mut Struct340;
    let mut u_var3: u16;
    let mut local_4: u16;

    u_var3 = (param_1 >> 0x10);
    local_bx_7 = param_1;
    local_bx_7.field_0x28 = 0;
    local_4 = 0;
    loop {
        pu_var1 = &local_bx_7.field_0x16;
        if (unsafe { *pu_var1 == local_4 } || unsafe { *pu_var1 < local_4 }) {
            break;
        }
        u_var2 = (&local_bx_7.field_0x2a + local_4 * 4);
        DestroyWindow16((u_var2 + 0x18));
        local_4 = local_4 + 1;
    }
    local_bx_7.field_0x16 = 0;
    if ((&local_bx_7.field_0x54 | local_bx_7.field_0x52) != 0) {
        local_4 = 0;
        while {
            u_var2 = &local_bx_7.field_0x52;
            if ((u_var2 + local_4 * 4) != 0) {
                u_var2 = &local_bx_7.field_0x52;
                u_var2 = (u_var2 + local_4 * 4);
                DestroyWindow16((u_var2 + 0x18));
                u_var2 = &local_bx_7.field_0x52;
                (u_var2 + local_4 * 4) = 0;
            }
            local_4 = local_4 + 1;
            local_4 < 10
        } {}
        error_check_1000_17ce(&local_bx_7.field_0x52);
        &local_bx_7.field_0x52 = 0;
    }
    return;
}

pub fn win_gui_fn_1010_32f4(param_1: *mut Struct387, param_2: *mut u32) {
    let pu_var1: *mut u16;
    let pu_var2: *mut u32;
    let mut u_var3: u32;
    let mut u_var4: u32;
    let mut u_var5: u32;
    let mut u_var6: i32;
    let mut i_var7: i32;
    let mut u_var8: u16;
    let ctx.dx_reg: *mut u16;

    let local_bx_5: *mut Struct387;
    let mut i_var9: i32;
    let mut i_var10: i32;
    let mut u_var11: u16;
    let mut u_var12: u16;
    let mut unaff_cs: u16;
    let mut u_var13: u16;
    let mut local_30: u16;
    let mut local_2e: u16;
    let mut local_28: u16;
    let mut local_26: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u32;
    let temp_86224844142: *mut u32;
    let mut temp_5f9d4a48c3: u32;
    let mut temp_5fd8766d76: u32;
    let fn_ptr_1: fn();

    u_var11 = (param_1 >> 0x10);
    local_bx_5 = param_1;
    if (&local_bx_5.field_0x52 != 0) {
        unaff_cs = 0x1000;
        error_check_1000_17ce(&local_bx_5.field_0x52);
        &local_bx_5.field_0x52 = 0;
        local_bx_5.field_0x18 = 0;
    }
    u_var6 = param_2._2_2_ | param_2;
    if ((param_2 != 0x0)
        && (
            fn_ptr_1 = (param_1 + 0x24),
            (**fn_ptr_1)(unaff_cs, param_1, param_2),
            u_var6 != 0,
        ))
    {
        unsafe {
            fn_ptr_1 = (*param_2 + 4);
        }
        (**fn_ptr_1)(unaff_cs, param_2);
        local_bx_5.field_0x18 = u_var6;
        if (u_var6 != 0) {
            local_bx_5.field_0x24 = 0;
            local_bx_5.field_0x26 = 0;
            _g_Struct94_ptr_1 = local_bx_5.field_0x28;
            pu_var1 = &local_bx_5.field_0x18;
            unsafe {
                *pu_var1 = *pu_var1 - _g_Struct94_ptr_1;
            }
            if (10 < local_bx_5.field_0x18) {
                local_bx_5.field_0x26 = 1;
                local_bx_5.field_0x18 = 10;
            }
            if (1 < local_bx_5.field_0x28) {
                local_bx_5.field_0x24 = 1;
            }
            if (ctx.__g_Struct94_ptr_1 == 0) {
                ctx.g_u16_ptr_1050_5f2e = ctx.dx_reg;
                struct_fn_1000_160a();
            } else {
            }
            u_var13 = 0x1000;
            alloc_mem_1000_1708(0x28, 0, 1, _g_Struct94_ptr_1, ctx.g_u16_ptr_1050_5f2e);
            local_bx_5.field_0x52 = _g_Struct94_ptr_1;
            &local_bx_5.field_0x54 = ctx.g_u16_ptr_1050_5f2e;
            if ((&local_bx_5.field_0x54 | local_bx_5.field_0x52) != 0) {
                u_var3 = (param_2 + 8);
                i_var7 = local_bx_5.field_0x10 + -10;
                local_c = 0;
                local_10 = 0;
                while (
                    pu_var1 = &local_bx_5.field_0x18,
                    unsafe { *pu_var1 != local_10 } && unsafe { local_10 <= *pu_var1 },
                ) {
                    u_var4 = &local_bx_5.field_0x52;
                    u_var6 = u_var4 + local_10 * 4;
                    u_var4 = u_var4 & 0xffff0000;
                    _local_30 = (u_var4 | u_var6);
                    temp_5fd8766d76 = ((local_bx_5.field_0x28 + local_10) * 4 + u_var3);
                    fn_ptr_1 = (param_1 + 0x1c);
                    u_var8 = local_10;
                    (**fn_ptr_1)(
                        u_var13,
                        param_1,
                        temp_5fd8766d76,
                        (temp_5fd8766d76 >> 0x10),
                        local_bx_5.field_0x22,
                    );
                    *_local_30 = u_var8;
                    *(u_var6 + 2) = ctx.dx_reg;
                    u_var5 = &local_bx_5.field_0x52;
                    u_var5 = (u_var5 + local_10 * 4);
                    local_c = local_c + (u_var5 + 0x24) + 8;
                    if (i_var7 < local_c) {
                        u_var13 = SUB42(&ctx.PTR_LOOP_1050_1008, 0);
                        fn_1008_6048(
                            s_overflow_on_node__d_1050_11ca,
                            ctx.dx_reg,
                            SUB21(i_var7, 0),
                        );
                        local_bx_5.field_0x18 = local_10 - 1;
                        local_bx_5.field_0x26 = 1;
                        u_var5 = &local_bx_5.field_0x52;
                        u_var12 = (u_var5 >> 0x10);
                        i_var10 = u_var5;
                        pu_var2 = (i_var10 + local_10 * 4);
                        u_var6 = (i_var10 + local_10 * 4 + 2);
                        if ((u_var6 | pu_var2) != 0) {
                            unsafe {
                                fn_ptr_1 = *pu_var2;
                            }
                            (**fn_ptr_1)(&ctx.PTR_LOOP_1050_1008, pu_var2, u_var6, 1);
                        }
                        u_var5 = &local_bx_5.field_0x52;
                        i_var10 = local_10 * 4;
                        (u_var5 + i_var10) = 0;
                        if (0 < local_10) {
                            u_var5 = &local_bx_5.field_0x52;
                            u_var12 = (u_var5 >> 0x10);
                            i_var9 = u_var5;
                            pu_var2 = (i_var9 + i_var10 + -4);
                            u_var6 = (i_var9 + i_var10 + -2);
                            if ((u_var6 | pu_var2) != 0) {
                                unsafe {
                                    fn_ptr_1 = *pu_var2;
                                }
                                (**fn_ptr_1)(&ctx.PTR_LOOP_1050_1008, pu_var2, u_var6, 1);
                            }
                            u_var5 = &local_bx_5.field_0x52;
                            (local_10 * 4 + u_var5 + -4) = 0;
                        }
                    }
                    local_10 = local_10 + 1;
                }
                local_bx_5.field_0x20 = 10;
                u_var13 = local_bx_5.field_0x1e;
                u_var3 = &local_bx_5.field_0x52;
                update_window_1040_93aa(u_var3, (u_var3 >> 0x10), 10, u_var13);
                local_10 = 1;
                while (
                    pu_var1 = &local_bx_5.field_0x18,
                    unsafe { *pu_var1 != local_10 } && unsafe { local_10 <= *pu_var1 },
                ) {
                    u_var3 = &local_bx_5.field_0x52;
                    u_var3 = (local_10 * 4 + u_var3 + -4);
                    i_var7 = u_var3;
                    u_var12 = (u_var3 >> 0x10);
                    u_var3 = &local_bx_5.field_0x52;
                    u_var3 = (u_var3 + local_10 * 4);
                    update_window_1040_93aa(
                        u_var3,
                        (u_var3 >> 0x10),
                        (i_var7 + 0x20) + (i_var7 + 0x24) + 0x8,
                        u_var13,
                    );
                    local_10 = local_10 + 1;
                }
            }
        }
    }
    return;
}

pub fn win_gui_fn_1010_79aa(param_1: &mut Struct2111, param_2: u16, param_3: u32) {
    let hwnd: HWND16;
    let mut u_var1: u32;
    let mut u_var2: u32;
    let local_AX_66: *mut Struct17;

    let mut u_var3: u16;
    let mut unaff_ss: u16;
    let mut local_1c: u16;
    let mut local_12: u32;
    let mut local_e: u32;
    let mut local_a: [u8; 8];

    u_var3 = (param_1 >> 0x10);
    if (((param_1 + 0x1c) != 0) && (param_3 != 0 || (param_2 != 0))) {
        pass1_1008_5784(CONCAT22(unaff_ss, local_a), (param_1 + 0x1c));
        local_12 = 0;
        while {
            local_AX_66 = local_a;
            pass1_1008_5b12(CONCAT22(unaff_ss, local_AX_66));
            local_e = CONCAT22(ctx.dx_reg, local_AX_66);
            if ((ctx.dx_reg | local_AX_66) == 0) {}
            // goto LAB_1010_7a49;
            if (((param_2 == 0) && (local_AX_66.field_0x4 == param_3))
                || (param_3 == 0 && (u_var1 = local_AX_66.field_0x8, (u_var1 + 10) == param_2)))
            {
                break;
            }
            (local_AX_66.field_0x4 != param_3)
                || (u_var1 = local_AX_66.field_0x8, (u_var1 + 10) != param_2)
        } {}
        local_12 = local_e;
        // LAB_1010_7a49:
        if (local_12 != 0) {
            u_var2 = (local_12 + 8);
            hwnd = (u_var2 + 6);
            SetFocus16(hwnd);
            BringWindowToTop16(hwnd);
            return;
        }
    }
    return;
}

pub fn show_window_1010_7a76(param_1: u32) {
    let mut u_var1: u32;
    let mut i_var2: i32;
    let mut u_var3: u16;
    let mut unaff_ss: u16;
    let lVar4: u32;
    let mut local_e: u32;
    let mut local_a: [u8; 8];

    u_var3 = (param_1 >> 0x10);
    i_var2 = param_1;
    if ((i_var2 + 0x20) == 0) {
        (i_var2 + 0x20) = 1;
        pass1_1008_5784(CONCAT22(unaff_ss, local_a), (i_var2 + 0x1c));
        while (true) {
            lVar4 = pass1_1008_5b12(CONCAT22(unaff_ss, local_a));
            if (lVar4 == 0) {
                break;
            }
            u_var1 = (lVar4 + 8);
            ShowWindow16(0, (u_var1 + 6));
        }
    }
    return;
}

pub fn show_window_1010_7ace(param_1: u32) {
    let mut u_var1: u32;
    let mut i_var2: i32;
    let mut u_var3: u16;
    let mut unaff_ss: u16;
    let lVar4: u32;
    let mut local_e: u32;
    let mut local_a: [u8; 8];

    u_var3 = (param_1 >> 0x10);
    i_var2 = param_1;
    if ((i_var2 + 0x20) != 0) {
        (i_var2 + 0x20) = 0;
        pass1_1008_5784(CONCAT22(unaff_ss, local_a), (i_var2 + 0x1c));
        while (true) {
            lVar4 = pass1_1008_5b12(CONCAT22(unaff_ss, local_a));
            if (lVar4 == 0) {
                break;
            }
            u_var1 = (lVar4 + 8);
            ShowWindow16(1, (u_var1 + 6));
        }
    }
    return;
}

pub fn destroy_win_1010_7b26(param_1: u32, param_2: libc::c_long) {
    let mut u_var1: u32;
    let pu_var2: Vec<u8>;

    let mut i_var3: i32;
    let mut u_var4: u16;
    let mut unaff_ss: u16;
    let mut local_e: u32;
    let mut local_a: [u8; 8];

    u_var4 = (param_1 >> 0x10);
    i_var3 = param_1;
    if (((i_var3 + 0x1e) | (i_var3 + 0x1c)) != 0) {
        pass1_1008_5784(CONCAT22(unaff_ss, local_a), (i_var3 + 0x1c));
        while {
            pu_var2 = local_a;
            pass1_1008_5b12(CONCAT22(unaff_ss, pu_var2));
            if ((ctx.dx_reg | pu_var2) == 0) {
                break;
            }
            (pu_var2 + 4) != param_2
        } {}
        if ((ctx.dx_reg | pu_var2) != 0) {
            u_var1 = (pu_var2 + 8);
            DestroyWindow16((u_var1 + 6));
        }
    }
    return;
}

pub fn win_gui_fn_1010_8096(param_1: *mut u32, param_2: u16) {
    let in_struct_1: &mut Struct44;
    let u_var1: u8;
    let pc_var2: String;
    let extraout_var: u32;


    let mut i_var4: i32;
    let mut u_var5: u16;
    let mut unaff_ss: i32;
    let string_b: String;
    let mut local_312: u32;
    let mut local_30e: u32;
    let mut local_30a: u16;
    let mut local_308: u16;
    let mut string_1: [u8; 256];
    let mut string_2: [u8; 256];
    let mut string_3: [u8; 260];
    let mut u_var3: u16;

    u_var5 = (param_1 >> 0x10);
    process_string_1000_4d58(((param_1 + 0xe82) * 4 + 0x2526), 0, 0);
    copy_string_1000_3d3e(CONCAT22(unaff_ss, string_3), CONCAT22(unaff_ss, string_2));
    if (param_2 == 2) {
        string_b = "b";
    } else {
        string_b = "a";
    }
    process_string_1000_3cea(CONCAT22(unaff_ss, string_3), string_b);
    process_string_1000_3cea(CONCAT22(unaff_ss, string_3), CONCAT22(unaff_ss, string_1));
    pc_var2 = string_3;
    set_error_mode_1010_8b14(param_1, pc_var2, unaff_ss);
    _local_30a = CONCAT22(ctx.dx_reg, pc_var2);
    i_var4 = ctx.dx_reg;
    if ((pc_var2 == string_3) && (ctx.dx_reg == unaff_ss)) {
        msg_box_1010_8bb4(param_1, pc_var2, ctx.dx_reg);
        i_var4 = ctx.dx_reg;
    }
    unsafe {
        in_struct_1 = *param_1;
    }
    u_var1 = error_check_1000_17ce(in_struct_1);
    u_var3 = CONCAT31(extraout_var, u_var1);
    pass1_fn_1008_60e8(_local_30a, in_struct_1, in_struct_1);
    param_1 = u_var3;
    (param_1 + 2) = i_var4;
    return;
}

pub fn win_gui_fn_1010_8170(param_1: *mut u32, param_2: i32) {
    let mut u_var1: i32;
    let in_dx: *mut u16;
    let mut i_var2: i32;
    let local_bx_20: &mut Struct447;
    let mut u_var3: u16;
    let mut u_var4: u32;

    u_var3 = (param_1 >> 0x10);
    i_var2 = param_1;
    u_var1 = (i_var2 + 0x680);
    local_bx_20 = (param_2 * 0x10);
    if (local_bx_20.field_0x16 != u_var1) {
        win_gui_fn_1010_8096(param_1, local_bx_20.field_0x16);
        u_var4 = pass1_1010_878c(i_var2, u_var3, local_bx_20.field_0x16);
        in_dx = (u_var4 >> 0x10);
        u_var1 = u_var4;
        if ((i_var2 + 0x67c) == 0) {
            return;
        }
    }
    param_2 = param_2 * 0x10;
    pass1_1008_6562(
        (i_var2 + 0x67c),
        CONCAT22((param_2 + 0x1c), (param_2 + 0x1e)),
        (param_2 + 0x1a),
        u_var1,
        in_dx,
    );
    return;
}

pub fn msg_box_1010_8bb4(ctx: &mut AppContext,
                         param_1: u32,
                         param_3: &String) {
    let mut title: String;
    // let mut in_string_2: String;
    // let mut unaff_ss: u16;
    let mut w_param: u32;
    let mut local_402: String = String::new();

    let mut in_string_2 = load_string_1010_847e(
        ctx,
        ctx._g_struct_73_1050_14cc,
        0x3fa,
    );
    copy_string_1000_3d3e(local_402, in_string_2);
    process_string_1000_3cea(local_402, param_1);
    title = load_string_1010_847e(
        ctx,
        ctx._g_struct_73_1050_14cc,
        0x57b,
    );
    // MessageBox16(0x1010, title, CONCAT22(unaff_ss, local_402), ctx.g_h_window);
    MessageBox16(ctx.g_h_window,  local_402, &title, 0x1010);
    // PostMessage16(0, 0xee, 0x111, ctx.g_h_window);
    PostMessage16(ctx.g_h_window, 0x111, 0xee, 0);
    return;
}

pub fn load_cursor_1018_5840(param_1: *mut Struct65, param_2: u16, param_3: u32) {
    let mut u_var1: u16;
    let mut i_var2: i32;
    let mut unaff_bp: u16;
    let mut u_var3: u16;
    let ppVar4: *mut pass1_struct_1;

    load_cursor_1020_7f7a(param_1, CONCAT22(param_2, 6), param_3);
    u_var3 = (param_1 >> 0x10);
    i_var2 = param_1;
    (i_var2 + 0xee) = 0;
    (i_var2 + 0xf2) = 0;
    (i_var2 + 0xf6) = 0;
    param_1.ptr_a_lo = (s_Alloc__s_1050_5a5b + 7);
    (i_var2 + 2) = 0x1018;
    (i_var2 + 0xe2) = 0x5afe;
    (i_var2 + 0xe4) = 0x1018;
    ppVar4 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(unaff_bp, 0x27));
    u_var1 = (ppVar4 >> 0x10);
    (i_var2 + 0xf2) = ppVar4;
    (i_var2 + 0xf4) = u_var1;
    (i_var2 + 0xe6) = (i_var2 + 0xf2);
    (i_var2 + 0xe8) = u_var1;
    return;
}

pub fn set_window_text_1018_6066(param_1: u32, in_window_text: u32, in_dlg_item_id: u16) {
    let hwnd: HWND16;

    hwnd = GetDlgItem16(in_dlg_item_id, (param_1 + 6));
    SetWindowText16(in_window_text, hwnd);
    return;
}

pub fn set_window_text_1018_6630(in_struct_604_ptr: *mut Struct604) {
    let mut in_dlg_item_id: u16;
    let mut u_var1: i32;
    let struct_604_ptr_1: *mut Struct604;
    let struct_60_ptr_hi: *mut Struct604;
    let mut local_c: u16;
    let window_text: SEGPTR;
    let mut local_4: u16;
    let struct_60_ptr_1: *mut Struct60;

    struct_60_ptr_hi = (in_struct_604_ptr >> 0x10);
    struct_604_ptr_1 = in_struct_604_ptr;
    window_text = load_rsrc_1010_4e9e(struct_604_ptr_1.Struct60_ptr);
    if (window_text != 0) {
        local_c = 0;
        while (local_c < 9) {
            struct_60_ptr_1 = struct_604_ptr_1.Struct60_ptr;
            in_dlg_item_id = pass1_1010_4f20(struct_60_ptr_1, (struct_60_ptr_1 >> 0x10), local_c);
            set_window_text_1018_6066(struct_604_ptr_1.field_0xa, window_text, in_dlg_item_id);
            u_var1 = get_string_index_1000_3da4(window_text);
            window_text = window_text & 0xffff0000 | (window_text + u_var1 + 1);
            local_c = local_c + 1;
        }
    }
    return;
}

pub fn destroy_win_1018_c518(in_struct_376_1: *mut Struct376) {
    let b_rc: bool;
    let struct_a_1: *mut Struct376;
    let struct_a_2: *mut Struct376;
    let temp_5fa5d31fd0: *mut Struct376;

    struct_a_2 = (in_struct_376_1 >> 0x10);
    struct_a_1 = in_struct_376_1;
    in_struct_376_1.ptr_a_lo = 0xc8bc;
    struct_a_1.ptr_a_hi = 0x1018;
    error_check_1000_17ce(&struct_a_1.struct_44_a);
    if (struct_a_1.window_handle_a != 0) {
        b_rc = IsWindow16(struct_a_1.window_handle_a);
        if (b_rc != 0) {
            DestroyWindow16(struct_a_1.window_handle_a);
            struct_a_1.window_handle_a = 0;
        }
    }
    pass1_1020_022c(in_struct_376_1);
    return;
}

pub fn destroy_win_fn_1018_c896(param_1: *mut Struct376, param_2: u8) -> *mut Struct376 {
    destroy_win_1018_c518(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn create_win_fn_1018_df40(in_struct_42_ptr_1: *mut win_struct_42) {
    let local_struct_42_1: *mut win_struct_42;
    let mut local_struct_42_hi_1: u16;
    let create_win_result: *mut Struct199;
    let mut local_u32_43: u32;
    let mut local_4: u16;

    create_win_result = create_win_1008_9760(in_struct_42_ptr_1);
    process_struct_1000_179c(10, (create_win_result >> 0x10));
    local_struct_42_1 = in_struct_42_ptr_1;
    local_struct_42_hi_1 = (in_struct_42_ptr_1 >> 0x10);
    if (create_win_result != 0x0) {
        local_u32_43 = pass1_1018_e100(create_win_result, local_struct_42_1.win_handle_0x8);
        local_struct_42_1.u16_xe2 = local_u32_43;
        local_struct_42_1.u16_xe4 = (local_u32_43 >> 0x10);
        return;
    }
    &local_struct_42_1.u16_xe2 = 0;
    return;
}

pub fn destroy_win_fn_1018_df92(param_1: *mut Struct594) {
    let pu_var1: *mut u32;
    let pvVar2: &mut Vec<u8>;
    let local_struct_594_ptr_1: *mut Struct594;
    let mut unaff_si: u16;
    let mut u_var3: u16;
    let temp_862fc4681f0: *mut u32;
    let local_fn_ptr_1: fn();

    destroy_win_1008_628e(param_1, unaff_si);
    u_var3 = (param_1 >> 0x10);
    local_struct_594_ptr_1 = param_1;
    pu_var1 = local_struct_594_ptr_1.u32_xE2;
    pvVar2 = local_struct_594_ptr_1.vptr_xE4;
    if ((pvVar2 | pu_var1) != 0) {
        unsafe {
            local_fn_ptr_1 = *pu_var1;
        }
        (**local_fn_ptr_1)(&ctx.PTR_LOOP_1050_1008, pu_var1, pvVar2, 1);
    }
    &local_struct_594_ptr_1.u32_xE2 = 0;
    return;
}

pub fn destroy_win_fn_1018_e72a(param_1: *mut Struct594) {
    let pu_var1: *mut u32;
    let mut u_var2: i32;
    let mut u_var3: u16;
    let mut in_stack_0000fff6: u16;
    Struct594 * *temp_86277dbc59c;
    let fn_ptr_1: fn();

    u_var3 = (param_1 >> 0x10);
    pu_var1 = (param_1 + 0xee);
    u_var2 = (param_1 + 0xf0);
    if ((u_var2 | pu_var1) != 0) {
        unsafe {
            fn_ptr_1 = *pu_var1;
        }
        (**fn_ptr_1)();
    }
    destroy_win_1008_628e(param_1, in_stack_0000fff6);
    return;
}

pub fn win_gui_fn_1018_e8bc(param_1: &mut Struct44) {
    let local_bx_3: *mut Struct376;
    let mut u_var1: u16;

    u_var1 = (param_1 >> 0x10);
    local_bx_3 = param_1;
    param_1.ptr_a_lo = 0xe912;
    local_bx_3.ptr_a_hi = 0x1018;
    if (&local_bx_3.u8_ptr_x14 != 0) {
        // WARNING: Load size is inaccurate
        pass1_1010_1dda(local_bx_3.u8_ptr_x14);
    }
    select_and_delete_palette_1020_92c4(param_1);
    return;
}

pub fn win_gui_fn_1018_e8ec(param_1: *mut Struct376, param_2: u8) -> *mut Struct376 {
    win_gui_fn_1018_e8bc(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn win_gui_fn_1018_eada(param_1: *mut win_struct_42) {
    let local_struct_1: *mut win_struct_42;
    let local_struct_1_hi: *mut win_struct_42;
    let paVar1: *mut Struct199;
    // local_DXAX_28: i32;
    let mut local_DXAX_61: u32;
    let mut local_4: u16;

    paVar1 = create_win_1008_9760(param_1);
    local_DXAX_28._2_2_ = (paVar1 >> 0x10);
    local_struct_1_hi = (param_1 >> 0x10);
    local_struct_1 = param_1;
    local_DXAX_28._0_2_ =
        get_gui_dc_1018_4db0(*&local_struct_1.u32_xf2, local_struct_1.win_handle_0x8);
    process_struct_1000_179c(0x28, local_DXAX_28._2_2_);
    if ((local_DXAX_28._2_2_ | local_DXAX_28) != 0) {
        local_DXAX_61 = draw_fn_1018_ec74(
            local_DXAX_28,
            local_DXAX_28._2_2_,
            local_struct_1.win_handle_0x8,
        );
        local_struct_1.char_ptr_16_0xee = local_DXAX_61;
        local_struct_1.field_0xf0 = (local_DXAX_61 >> 0x10);
        return;
    }
    &local_struct_1.char_ptr_16_0xee = 0;
    return;
}

pub fn win_gui_fn_1018_eb3e(in_struct_1: *mut Struct594) {
    let pu_var1: *mut u32;
    let mut u_var2: i32;
    let ppc_var3: fn();
    // ppu_var4: *mut Vec<u8>;
    let pa_var5: *mut Struct594;
    let local_struct_1: *mut Struct594;
    let local_struct_1_hi: *mut Struct594;
    let mut in_stack_0000fff2: u16;
    let temp_862c2f7bda0: Vec<u8>;

    local_struct_1_hi = (in_struct_1 >> 0x10);
    local_struct_1 = in_struct_1;
    pu_var1 = local_struct_1.u8_ptr_16_xee;
    u_var2 = local_struct_1.field_0xf0;
    if ((u_var2 | pu_var1) != 0) {
        unsafe {
            ppc_var3 = *pu_var1;
        }
        (**ppc_var3)();
    }
    if (&local_struct_1.field_0xf6 != 0) {
        if ((local_struct_1_hi | local_struct_1) == 0) {
            ppu_var4 = 0x0;
            pa_var5 = 0x0;
        } else {
            ppu_var4 = &local_struct_1.u32_xE2;
            pa_var5 = local_struct_1_hi;
        }
        pass1_1010_1ea6(*&local_struct_1.field_0xf6, CONCAT22(pa_var5, ppu_var4));
    }
    destroy_win_1008_628e(in_struct_1, in_stack_0000fff2);
    return;
}

pub fn win_gui_fn_1018_ed98(in_struct_1: &mut Struct44) {
    let local_bx_3: *mut Struct376;
    let mut u_var1: i32;

    u_var1 = (in_struct_1 >> 0x10);
    local_bx_3 = in_struct_1;
    in_struct_1.ptr_a_lo = 0x1cc;
    local_bx_3.ptr_a_hi = 0x1020;
    if (&local_bx_3.u8_ptr_x14 != 0) {
        // WARNING: Load size is inaccurate
        pass1_1010_1ea6(
            local_bx_3.u8_ptr_x14,
            (in_struct_1 & 0xffff | u_var1 << 0x10),
        );
        // WARNING: Load size is inaccurate
        pass1_1010_1dda(local_bx_3.u8_ptr_x14);
    }
    select_and_delete_palette_1020_92c4(in_struct_1);
    return;
}

pub fn win_gui_fn_1020_01a6(in_struct_1: *mut Struct376, in_byte_1: u8) -> *mut Struct376 {
    win_gui_fn_1018_ed98(in_struct_1);
    if ((in_byte_1 & 1) != 0) {
        error_check_1000_17ce(in_struct_1);
    }
    return in_struct_1;
}

pub fn load_cursor_fn_1020_01d8(
    in_struct_1: *mut Struct65,
    in_struct_1_hi: *mut Struct65,
    param_3: u16,
    param_4: u16,
    param_5: u16,
    param_6: u16,
    param_7: u16,
    param_8: &mut Vec<u8>,
) {
    load_cursor_1008_61b2(
        CONCAT22(in_struct_1_hi, in_struct_1),
        param_3,
        param_7,
        param_8,
    );
    &in_struct_1.ptr_b_lo = 0;
    &in_struct_1.u16_xe6 = 0;
    in_struct_1.u16_xea = param_6;
    in_struct_1.u16_xec = param_5;
    in_struct_1.u16_xee = param_3_00;
    CONCAT22(in_struct_1_hi, in_struct_1) = 0x45a;
    in_struct_1.ptr_a_hi = 0x1020;
    return;
}

pub fn win_gui_fn_1020_028c(param_1: *mut Struct628, param_2: u16, param_3: u16) {
    pass1_1010_3c9e(param_1.field_0xe2);
    show_window_1008_68c6(param_1, param_2, param_3);
    return;
}

pub fn pass1_1020_02ae(in_struct_1: *mut Struct594) {
    let local_struct_1: *mut Struct594;
    let mut unaff_si: u16;
    let local_struct_1_hi: *mut Struct594;
    let temp_5f77ded944: Vec<u8>;
    let temp_5f51233cf1: Vec<u8>;
    // fn_ptr_1: *mut Vec<u8>;

    local_struct_1_hi = (in_struct_1 >> 0x10);
    local_struct_1 = in_struct_1;
    pass1_1010_3cd0(&local_struct_1.u32_xE2);
    destroy_win_1008_628e(in_struct_1, unaff_si);
    // WARNING: Load size is inaccurate
    temp_5f77ded944 = local_struct_1.u8_ptr_32_xE6;
    temp_5f51233cf1 = *(&local_struct_1.u8_ptr_32_xE6 + 2);
    if ((temp_5f51233cf1 | temp_5f77ded944) != 0) {
        fn_ptr_1 = temp_5f77ded944;
        (**fn_ptr_1)(&ctx.PTR_LOOP_1050_1008, temp_5f77ded944, temp_5f51233cf1, 1);
    }
    local_struct_1.u8_ptr_32_xE6 = 0x0;
    // WARNING: Load size is inaccurate
    pass1_1010_1dda(local_struct_1.u32_xE2);
    &local_struct_1.u32_xE2 = 0;
    return;
}

pub fn create_win_fn_1020_0316(in_struct_1: *mut win_struct_42) {
    let local_struct_1: *mut win_struct_42;
    let local_struct_1_hi: *mut win_struct_42;
    let ppVar1: *mut pass1_struct_1;
    let in_struct_1_00: *mut Struct629;
    let mut u_var2: u32;
    let mut in_stack_0000fff2: u16;
    let mut local_c: u16;

    create_win_1008_9760(in_struct_1);
    ppVar1 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(in_stack_0000fff2, 1));
    local_struct_1_hi = (in_struct_1 >> 0x10);
    local_struct_1 = in_struct_1;
    local_struct_1.u16_xe2 = ppVar1;
    local_struct_1.u16_xe4 = (ppVar1 >> 0x10);
    u_var2 = &local_struct_1.u16_xe2;
    (u_var2 + 0x16) = &local_struct_1.field_0xea;
    u_var2 = &local_struct_1.u16_xe2;
    *(u_var2 + 0x12) = local_struct_1.char_ptr_16_0xee;
    in_struct_1_00 = pass1_1010_3c52(&local_struct_1.u16_xe2, &local_struct_1.field_0xec);
    process_struct_1000_179c(0x12, (in_struct_1_00 >> 0x10));
    if (in_struct_1_00 != 0x0) {
        u_var2 = process_struct_1020_04f6(in_struct_1_00, local_struct_1.win_handle_0x8);
        &local_struct_1.field_0xe6 = u_var2;
        &local_struct_1.field_0xe8 = (u_var2 >> 0x10);
        return;
    }
    &local_struct_1.field_0xe6 = 0;
    return;
}

pub fn win_gui_fn_1020_0762(
    param_1: *mut Struct65,
    param_2: u32,
    param_3: u32,
    param_4: u16,
    param_5: u32,
    param_6: u32,
) {
    let in_struct_1: *mut Struct65;
    let in_struct_1_hi: *mut Struct65;

    in_struct_1 = param_1;
    in_struct_1_hi = (param_1 >> 0x10);
    load_cursor_fn_1020_01d8(
        in_struct_1,
        in_struct_1_hi,
        param_2,
        (param_2 >> 0x10),
        param_4,
        param_5,
        (param_5 >> 0x10),
        param_6,
    );
    in_struct_1.u16_xf0 = 0;
    &in_struct_1.u16_xf2 = param_3;
    param_1.ptr_a_lo = 0x81a;
    in_struct_1.ptr_a_hi = 0x1020;
    return;
}

pub fn enable_menu_item_1020_1000() {
    let mut in_stack_0000000a: i32;
    let mut in_stack_0000000c: u16;

    if (in_stack_0000000a != 0) {
        return;
    }
    EnableMenuItem16(0x400, 3, in_stack_0000000c);
    return;
}

pub fn destroy_icon_func_1020_1038(in_struct_1: *mut Struct48) {
    let pu_var1: *mut u32;
    let mut u_var2: i32;
    let local_struct_1: *mut Struct48;
    let local_struct_1_hi: *mut Struct48;
    let fn_ptr_1: fn();

    local_struct_1_hi = (in_struct_1 >> 0x10);
    local_struct_1 = in_struct_1;
    DestroyIcon16(local_struct_1.handle_0xc2);
    local_struct_1.handle_0xc2 = 0;
    local_struct_1.field_0x8 = 0;
    pu_var1 = local_struct_1.fn_ptr_0xf6;
    u_var2 = &local_struct_1.field_0xf8;
    if ((u_var2 | pu_var1) != 0) {
        unsafe {
            fn_ptr_1 = *pu_var1;
        }
        (**fn_ptr_1)(offset, pu_var1, u_var2, 1);
    }
    &local_struct_1.fn_ptr_0xf6 = 0;
    pass1_1010_1dda(local_struct_1.field_0xf2);
    local_struct_1.field_0xf2 = 0;
    return;
}

pub fn update_window_1020_10a0(param_1: *mut win_struct_42) {
    let mut u_var1: u32;
    let ppc_var2: fn();
    let mut u_var3: i32;
    let pi_var4: *mut u16;
    let pa_var5: *mut Struct199;

    let paVar6: *mut Struct199;
    let ctx.dx_reg: *mut Struct199;
    let struct_a: *mut Struct199;


    let mut i_var7: i32;
    let mut unaff_si: u16;
    let mut u_var8: u16;
    let paVar9: *mut Struct199;
    let ppVar10: *mut pass1_struct_1;
    let mut u_var11: u32;
    let mut u_var12: u16;
    let local_3a: *mut Struct71;
    let mut local_8: u16;

    paVar9 = create_win_1008_9760(param_1);
    paVar6 = (paVar9 >> 0x10);
    u_var3 = paVar9;
    process_struct_1000_179c(0x42, paVar6);
    pa_var5 = (paVar6 | u_var3);
    i_var7 = param_1;
    u_var8 = (param_1 >> 0x10);
    if (pa_var5 != 0x0) {
        win_fn_1008_3bd6(
            u_var3,
            paVar6,
            0,
            0x1f009b,
            0x750000,
            0xf10074,
            CONCAT22(unaff_si, (i_var7 + 8)),
        );
        pa_var5 = ctx.dx_reg;
    }
    process_struct_1000_179c(0x42, pa_var5);
    paVar6 = (pa_var5 | u_var3);
    if (paVar6 != 0x0) {
        win_fn_1008_3bd6(
            u_var3,
            pa_var5,
            0,
            0x31009b,
            0x770000,
            0xf20076,
            CONCAT22(unaff_si, (i_var7 + 8)),
        );
        paVar6 = ctx.dx_reg;
    }
    process_struct_1000_179c(0x42, paVar6);
    if ((paVar6 | u_var3) != 0) {
        win_fn_1008_3bd6(
            u_var3,
            paVar6,
            0,
            0x77009b,
            0x790000,
            0xf30078,
            CONCAT22(unaff_si, (i_var7 + 8)),
        );
    }
    ppVar10 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(unaff_si, 0x2d));
    u_var12 = (ppVar10 >> 0x10);
    (i_var7 + 0xf2) = ppVar10;
    (i_var7 + 0xf4) = u_var12;
    u_var3 = (i_var7 + 0xf2);
    (i_var7 + 0xe0) = u_var3;
    (i_var7 + 0xe2) = u_var12;
    LoadIcon16(
        0x1010,
        s_PLNTICON_1050_4267,
        &ctx.g_alloc_addr_1050_1050,
        ctx.g_h_instance_1050_038c,
    );
    (i_var7 + 0xc2) = u_var3;
    u_var1 = (i_var7 + 0xf2);
    u_var12 = u_var1;
    ppc_var2 = ((i_var7 + 0xf2) + 0x30);
    ppc_var2(offset, u_var12, (u_var1 >> 0x10), u_var3);
    pa_var5 = struct_a;
    process_struct_1000_179c(0x24, struct_a);
    if ((pa_var5 | u_var3) == 0) {
        (i_var7 + 0xf6) = 0;
    } else {
        get_dc_1020_1418(u_var3, pa_var5, param_1, u_var8);
        (i_var7 + 0xf6) = u_var3;
        (i_var7 + 0xf8) = ctx.dx_reg;
    }
    (i_var7 + 0xe8) = (i_var7 + 0xf6);
    ppVar10 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(u_var12, 0x2f));
    u_var11 = pass1_1018_04b8(ppVar10);
    pass1_1010_41d6((i_var7 + 0xf2), u_var11);
    u_var11 = pass1_1010_451a((i_var7 + 0xf2));
    pi_var4 = u_var11;
    u_var1 = param_1;
    ppc_var2 = (u_var1 + 0x14);
    ppc_var2(0x1010, i_var7, u_var8, 0, u_var11, (u_var11 >> 0x10));
    u_var12 = 1;
    ppc_var2 = (u_var1 + 0x10);
    ppc_var2();
    pass1_1010_459e((i_var7 + 0xf2));
    ppc_var2 = ((i_var7 + 0xf2) + 0x10);
    ppc_var2(0x1010, (i_var7 + 0xf2), param_1, u_var12);
    MoveWindow16(
        1,
        pi_var4[3],
        pi_var4[2],
        pi_var4[1],
        unsafe { *pi_var4 },
        (i_var7 + 8),
    );
    UpdateWindow16((i_var7 + 8));
    return;
}

pub fn win_fn_1020_1294(param_1: *mut Struct637, param_2: u16, param_3: u16) {
    let mut u_var1: u32;
    let mut u_var2: u32;
    let ppc_var3: fn();
    let ppVar4: *mut pass1_struct_2;
    let pa_var5: *mut Struct199;
    let h_cursor: *mut u16;
    let h_cursor_00: HCURSOR16;
    let mut in_dx: i32;

    let mut i32_var6: i32;
    let mut u_var7: u16;
    let mut unaff_ss: u16;
    let mut local_1e: u16;
    let mut local_1c: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u32;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    ppVar4 = pass1_1030_8344(
        ctx._g_bool_1050_5748,
        (ctx._g_bool_1050_5748 >> 0x10),
        0x4000001,
    );
    if ((in_dx | ppVar4) == 0) {
        local_6 = param_3;
        local_4 = param_2;
        u_var7 = (param_1 >> 0x10);
        i32_var6 = param_1;
        pass1_1010_40cc((i32_var6 + 0xf2));
        _local_a = CONCAT22(ctx.dx_reg, param_2);
        u_var1 = (i32_var6 + 0xf2);
        pa_var5 = (u_var1 + 0x76);
        local_e = u_var1 & 0xffff0000 | ZEXT24(pa_var5);
        pass1_1008_3e94(
            pa_var5,
            CONCAT22(unaff_ss, &local_12),
            CONCAT22(unaff_ss, &local_10),
        );
        local_6 = local_6 - local_10;
        local_4 = local_4 - local_12;
        h_cursor = &local_6;
        u_var2 = (i32_var6 + 0xf2);
        pt_in_rect_1010_40f8(u_var2, (u_var2 >> 0x10), h_cursor, unaff_ss);
        if (h_cursor != 0xffff) {
            h_cursor_00 = LoadCursor16(0x7f02, 0);
            SetCursor16(h_cursor_00);
            ppc_var3 = (*_local_a + 4);
            (**ppc_var3)();
            pass1_1008_3e0e(param_1);
            SetCursor16(h_cursor);
        }
    }
    return;
}

pub fn call_destroy_menu_fn_1020_135e(param_1: &mut Struct44, param_2: u8) -> &mut Struct44 {
    ui2::destroy_menu_func_1020_795c(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn get_dc_1020_1418(param_1: *mut u16, param_2: u32) {
    let mut u_var1: u32;
    let ppc_var2: fn();
    let pu_var3: *mut u16;
    let mut u_var4: u32;

    let local_bx_17: *mut Struct63;
    let mut unaff_ss: u16;
    let ppVar5: *mut pass1_struct_1;
    let mut i32_var6: i32;
    let mut u_var7: u16;
    let mut in_stack_0000ffdc: u16;
    let mut local_10: u32;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    i32_var6 = param_1;
    u_var7 = (param_1 >> 0x10);
    get_sys_metrics_1020_7c1a(i32_var6, u_var7, param_2, (param_2 >> 0x10));
    (i32_var6 + 0x14) = 0;
    (i32_var6 + 0x18) = 0;
    zero_list_1008_3e38((param_1 & 0xffff0000 | (i32_var6 + 0x1e)));
    unsafe {
        *param_1 = 0x1730;
    }
    (i32_var6 + 2) = 0x1020;
    ppVar5 = process_struct_1010_20ba(
        ctx._g_Struct372_1050_0ed0,
        CONCAT22(in_stack_0000ffdc, 0x2d),
    );
    (i32_var6 + 0x14) = ppVar5;
    (i32_var6 + 0x16) = (ppVar5 >> 0x10);
    _local_6 = process_struct_1010_20ba(
        ctx._g_Struct372_1050_0ed0,
        CONCAT22(in_stack_0000ffdc, 0x29),
    );
    u_var1 = (i32_var6 + 0x14);
    ppc_var2 = ((i32_var6 + 0x14) + 4);
    ppc_var2(0x10, u_var1, (u_var1 >> 0x10), 0, i32_var6, u_var7);
    local_8 = GetDC16((i32_var6 + 4));
    u_var1 = (i32_var6 + 0x14);
    (u_var1 + 0x7c) = local_8;
    u_var1 = (i32_var6 + 0x14);
    u_var4 = (u_var1 + 0x66);
    (i32_var6 + 0x18) = u_var4;
    ppc_var2 = ((i32_var6 + 0x18) + 0x14);
    ppc_var2();
    u_var1 = (_local_6 + 0xe);
    pass1_1008_4d84((u_var4 & 0xffff | ctx.dx_reg << 0x10), u_var1);
    pu_var3 = &local_8;
    realize_palette_1008_4e08(u_var1, pu_var3, unaff_ss);
    (i32_var6 + 0x1c) = pu_var3;
    return;
}

pub fn set_dialog_item_1040_a94a(in_struct_1: *mut Struct351) {
    let pi_var1: *mut i32;
    let mut u_var2: u32;
    let lVar3: u32;
    let mut u_var4: u16;
    let mut u_var5: u16;
    let value: Vec<u8>;
    let mut u_var6: u16;
    let mut u_var7: i32;
    let HVar8: HWND16;
    let mut value_00: u16;

    let mut u_var10: i32;
    let local_struct_1: *mut Struct351;
    let mut i_var11: i32;
    let mut unaff_si: u16;
    let mut u_var12: u16;
    let mut u_var13: u16;
    let mut unaff_ss: u16;
    let mut b_var14: bool;
    let ppVar15: *mut pass1_struct_1;
    let mut local_128: u16;
    let mut local_126: u16;
    let mut local_120: u16;
    let mut local_11e: u16;
    let mut local_11c: u16;
    let mut local_11a: u32;
    let mut local_116: u16;
    let mut local_114: u16;
    let mut local_112: u32;
    let mut local_10e: u16;
    let mut local_10c: u16;
    let mut local_10a: u16;
    let mut local_108: u16;
    let mut local_106: u16;
    let mut local_104: u16;
    let mut local_102: [u8; 256];
    let mut u_var9: u32;

    ppVar15 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(unaff_si, 3));
    u_var5 = (ppVar15 >> 0x10);
    u_var4 = ppVar15;
    u_var12 = (in_struct_1 >> 0x10);
    local_struct_1 = in_struct_1;
    pass1_1010_c3c2(
        u_var4,
        u_var5,
        CONCAT22(unaff_ss, local_102),
        local_struct_1.field_0x94,
    );
    SetDlgItemText16(
        CONCAT22(unaff_ss, local_102),
        0x1f2,
        local_struct_1.field_0x6,
    );
    pass1_1010_c320(
        u_var4,
        u_var5,
        CONCAT22(unaff_ss, local_102),
        local_struct_1.field_0x94,
    );
    SetDlgItemText16(
        CONCAT22(unaff_ss, local_102),
        0x1774,
        local_struct_1.field_0x6,
    );
    str_fn_1010_c446(
        ppVar15,
        CONCAT22(unaff_ss, local_102),
        local_struct_1.field_0x94,
    );
    value = local_102;
    SetDlgItemText16(CONCAT22(unaff_ss, value), 0x1773, local_struct_1.field_0x6);
    u_var2 = local_struct_1.field_0x94;
    pass1_1030_6ddc(u_var2, (u_var2 >> 0x10));
    (0, value, 0x1f5, local_struct_1.field_0x6);
    pass1_1030_6e14(local_struct_1.field_0x94);
    (0, value, 0x1f6, local_struct_1.field_0x6);
    if (local_struct_1.field_0x98 != 0) {
        pass1_1010_dd5e(u_var4, u_var5, local_struct_1.field_0x94);
        if ((ctx.dx_reg | value) != 0) {
            u_var2 = local_struct_1.field_0x94;
            u_var13 = (u_var2 >> 0x10);
            i_var11 = u_var2;
            lVar3 = (i_var11 + 0x26);
            u_var10 = (i_var11 + 0x28);
            local_114 = 0x1798;
            local_116 = 0x17c3;
            local_struct_1.field_0xea = 0;
            local_120 = 1;
            while (local_120 < 0x25) {
                if (lVar3 == 0) {
                    value_00 = 0;
                    u_var7 = 0;
                } else {
                    u_var9 = pass1_1020_bae6(lVar3, CONCAT22(local_120, (lVar3 >> 0x10)));
                    value_00 = u_var9;
                    u_var7 = u_var10;
                }
                b_var14 = (value + local_120 * 4) != 0;
                u_var10 = u_var7;
                if (b_var14) {
                    u_var6 = value_00;
                    big_switch_statement_1020_c0d8(local_120);
                    SetDlgItemText16(
                        CONCAT22(u_var10, u_var6),
                        local_114,
                        local_struct_1.field_0x6,
                    );
                    SetDlgItemInt16(
                        0,
                        *(value + local_120 * 4),
                        local_116,
                        local_struct_1.field_0x6,
                    );
                }
                u_var7 = u_var7 | value_00;
                if (u_var7 != 0) {
                    if (!b_var14) {
                        big_switch_statement_1020_c0d8(local_120);
                        SetDlgItemText16(
                            CONCAT22(u_var10, u_var7),
                            local_114,
                            local_struct_1.field_0x6,
                        );
                    }
                    SetDlgItemInt16(0, value_00, local_116, local_struct_1.field_0x6);
                    i_var11 = local_struct_1.field_0xea;
                    HVar8 = GetDlgItem16(local_114, local_struct_1.field_0x6);
                    (&local_struct_1.field_0x9a + i_var11 * 2) = HVar8;
                    pi_var1 = &local_struct_1.field_0xea;
                    unsafe {
                        *pi_var1 = *pi_var1 + 1;
                    }
                    i_var11 = local_struct_1.field_0xea;
                    HVar8 = GetDlgItem16(local_116, local_struct_1.field_0x6);
                    (&local_struct_1.field_0x9a + i_var11 * 2) = HVar8;
                    pi_var1 = &local_struct_1.field_0xea;
                    unsafe {
                        *pi_var1 = *pi_var1 + 1;
                    }
                    b_var14 = true;
                }
                if (b_var14) {
                    local_114 = local_114 + 1;
                    local_116 = local_116 + 1;
                }
                local_120 = local_120 + 1;
            }
        }
    }
    return;
}

pub fn msg_box_1040_a85a(param_1: u32) {

    let in_dx: *mut Struct199;
    let mut unaff_ss: u16;
    let mut local_212: u32;
    let mut local_20e: u32;
    let mut local_20a: u16;
    let mut local_208: u16;
    let mut local_206: [u8; 258];
    let mut local_104: [u8; 258];

    process_struct_1000_179c(0x1000, in_dx);
    load_string_1010_84e0(
        ctx._g_struct_73_1050_14cc,
        (ctx._g_struct_73_1050_14cc >> 0x10),
        0x100,
        CONCAT22(unaff_ss, local_206),
        0x57b,
    );
    load_string_1010_84e0(
        ctx._g_struct_73_1050_14cc,
        (ctx._g_struct_73_1050_14cc >> 0x10),
        0x3ff,
        CONCAT22(in_dx, in_ax),
        0x7ef,
    );
    load_string_1010_84e0(
        ctx._g_struct_73_1050_14cc,
        (ctx._g_struct_73_1050_14cc >> 0x10),
        0x3ff,
        CONCAT22(unaff_ss, local_104),
        0x7f0,
    );
    process_string_1000_3cea(CONCAT22(in_dx, in_ax), CONCAT22(unaff_ss, local_104));
    load_string_1010_84e0(
        ctx._g_struct_73_1050_14cc,
        (ctx._g_struct_73_1050_14cc >> 0x10),
        0x3ff,
        CONCAT22(unaff_ss, local_104),
        0x7f1,
    );
    process_string_1000_3cea(CONCAT22(in_dx, in_ax), CONCAT22(unaff_ss, local_104));
    load_string_1010_84e0(
        ctx._g_struct_73_1050_14cc,
        (ctx._g_struct_73_1050_14cc >> 0x10),
        0x3ff,
        CONCAT22(unaff_ss, local_104),
        0x7f2,
    );
    process_string_1000_3cea(CONCAT22(in_dx, in_ax), CONCAT22(unaff_ss, local_104));
    MessageBox16(
        0,
        CONCAT22(unaff_ss, local_206),
        CONCAT22(in_dx, in_ax),
        (param_1 + 6),
    );
    error_check_1000_17ce(CONCAT22(in_dx, in_ax));
    return;
}

pub fn set_dialog_item_1040_a84a(param_1: *mut Struct351) {
    set_dialog_item_1040_a94a(param_1);
    return;
}

pub fn win_fn_1040_a784(
    param_1: *mut Struct124,
    param_2: *mut Struct124,
    param_3: u16,
    param_4: u32,
) {
    let h_wnd: HWND16;
    let paVar1: *mut Struct124;

    paVar1 = param_1;
    if (param_3._2_2_ != 0xeb) {
        if (param_3._2_2_ == 500) {
            msg_box_1040_a85a(param_1, param_2);
            return;
        }
        if (param_3._2_2_ == 0x1f7) {
            _PTR_LOOP_1050_5ef0 = (param_1 + 1);
            ui2::pass1_1038_af40(ctx._g_Struct112_a, *&param_1.field_0x8, 0x23);
            PostMessage16(0, 2, 0x111, &param_1.field_0x6);
            return;
        }
        if (param_3._2_2_ != 0x17d8) {
            win_gui_fn_1040_b54a(param_1, param_2, param_3_00, param_3);
            return;
        }
        SetWindowPos16(6, 0xed, 0x237, 0, 0, 0, &param_1.field_0x6);
        h_wnd = GetDlgItem16(0x17d8, &param_1.field_0x6);
        paVar1 = offset;
        EnableWindow16(0, h_wnd);
        &param_1[1].field_0x4 = 1;
        param_2 = param_1;
    }
    set_dialog_item_1040_a94a(CONCAT22(param_2, paVar1));
    return;
}

pub fn win_fn_1020_0dc4(in_struct_1: *mut win_struct_42, param_2: u16, param_3: u32) {
    let mut i_var1: i32;
    let local_struct_1_hi: *mut win_struct_42;

    ui2::call_load_cursor_1020_790e(in_struct_1, s_PCPOPMENU_1050_4256, param_2, param_3);
    local_struct_1_hi = (in_struct_1 >> 0x10);
    i_var1 = in_struct_1;
    (i_var1 + 0xf2) = 0;
    (i_var1 + 0xf6) = 0;
    (i_var1 + 0xfa) = 0;
    in_struct_1.u16_x0 = 0x1384;
    (i_var1 + 2) = 0x1020;
    copy_string_1000_3d3e(
        (in_struct_1 & 0xffff0000 | (i_var1 + 0x5b)),
        s_VrMode_1050_4260,
    );
    (i_var1 + 0xac) = 0x44c00000;
    update_window_1020_10a0(in_struct_1);
    return;
}

pub fn win_fn_1020_09ba(in_struct_1: *mut win_struct_42) {
    let local_struct_1: *mut win_struct_42;
    let local_struct_1_hi: *mut win_struct_42;
    let in_struct_1_00: *mut Struct636;
    let mut u_var1: u32;
    let mut local_4: u16;

    in_struct_1_00 = create_win_1008_9760(in_struct_1);
    process_struct_1000_179c(0xe, (in_struct_1_00 >> 0x10));
    local_struct_1 = in_struct_1;
    local_struct_1_hi = (in_struct_1 >> 0x10);
    if (in_struct_1_00 != 0x0) {
        u_var1 = process_struct_1020_0baa(in_struct_1_00, local_struct_1.win_handle_0x8);
        local_struct_1.u16_xe2 = u_var1;
        local_struct_1.u16_xe4 = (u_var1 >> 0x10);
        return;
    }
    &local_struct_1.u16_xe2 = 0;
    return;
}

pub fn win_fn_1018_e6c6(in_struct_a: *mut win_struct_42) {
    let struct_d: *mut Struct199;
    let string_base_a: String;
    let struct_a: *mut Struct199;
    let local_DX_44: Vec<u8>;
    let struct_c_lo: *mut win_struct_42;
    let struct_c_hi: *mut win_struct_42;
    let struct_b: *mut Struct199;
    let local_4: Vec<u8>;

    struct_b = create_win_1008_9760(in_struct_a);
    struct_a = (struct_b >> 0x10);
    struct_c_hi = (in_struct_a >> 0x10);
    struct_c_lo = in_struct_a;
    struct_d = get_gui_dc_1018_4db0(*&struct_c_lo.u32_xf2, struct_c_lo.win_handle_0x8);
    process_struct_1000_179c(0x18, struct_a);
    local_DX_44 = (struct_a | struct_d);
    if (local_DX_44 != 0x0) {
        string_base_a._0_1_ =
            pass1_1018_e834(struct_d, CONCAT22(struct_c_lo.win_handle_0x8, struct_a));
        string_base_a._0_2_ = CONCAT11(string_base_a._1_1_, string_base_a);
        struct_c_lo.char_ptr_16_0xee = string_base_a;
        struct_c_lo.field_0xf0 = local_DX_44;
        return;
    }
    &struct_c_lo.char_ptr_16_0xee = 0;
    return;
}

pub fn win_fn_1018_e384(in_win_struct_42_ptr: *mut win_struct_42) {
    let in_struct_622_ptr: *mut Struct622;
    let struct_a: *mut Struct199;
    let local_win_struct_42_ptr_1: *mut win_struct_42;
    let mut local_win_struct_42_ptr_1_hi: u16;
    let paVar1: *mut Struct199;
    let mut local_DXAX_61: u32;
    let mut local_4: u16;

    paVar1 = create_win_1008_9760(in_win_struct_42_ptr);
    struct_a = (paVar1 >> 0x10);
    local_win_struct_42_ptr_1_hi = (in_win_struct_42_ptr >> 0x10);
    local_win_struct_42_ptr_1 = in_win_struct_42_ptr;
    in_struct_622_ptr = get_gui_dc_1018_4db0(
        *&local_win_struct_42_ptr_1.u32_xf2,
        local_win_struct_42_ptr_1.win_handle_0x8,
    );
    process_struct_1000_179c(0x18, struct_a);
    if ((struct_a | in_struct_622_ptr) != 0) {
        local_DXAX_61 = draw_fn_1018_e4f2(
            in_struct_622_ptr,
            CONCAT22(local_win_struct_42_ptr_1.win_handle_0x8, struct_a),
        );
        local_win_struct_42_ptr_1.char_ptr_16_0xee = local_DXAX_61;
        local_win_struct_42_ptr_1.field_0xf0 = (local_DXAX_61 >> 0x10);
        return;
    }
    &local_win_struct_42_ptr_1.char_ptr_16_0xee = 0;
    return;
}

pub fn win_fn_1018_6adc(param_1: u32) {
    let mut i_var1: i32;
    let mut i_var2: i32;
    let u_var3: u8;
    let mut u_var4: u16;
    let extraout_DL: u8;
    let mut i_var5: i32;
    let mut u_var6: u16;
    let pp_var7: *mut pass1_struct_1;
    let mut in_stack_0000ffdc: u32;
    let in_string_1: String;
    let mut local_6: u16;
    let mut local_4: u16;

    in_string_1 = CONCAT22((in_stack_0000ffdc >> 0x10), 0x48);
    pp_var7 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, in_string_1);
    u_var6 = (pp_var7 >> 0x10);
    i_var1 = (pp_var7 + 10);
    i_var2 = (pp_var7 + 0xc);
    u_var6 = (param_1 >> 0x10);
    i_var5 = param_1;
    if (1 < i_var1 - (i_var5 + 0xe6)) {
        (i_var5 + 0xe2) = i_var1 / 2 - ((i_var5 + 0xe6) + 1) / 2;
    }
    if (1 < i_var2 - (i_var5 + 0xe8)) {
        (i_var5 + 0xe4) = i_var2 / 2 - ((i_var5 + 0xe8) + 1) / 2;
    }
    ShowCursor16(0x1010, 0, (in_string_1 >> 0x10));
    if ((i_var5 + 0xee) != 0) {
        u_var4 = mci_send_cmd_1008_5c5c(ctx._g_struct_ptr_1050_02a0, (i_var5 + 0xee));
        (i_var5 + 0xf0) = u_var4;
    }
    u_var3 = mixed_fn_1010_830a(ctx._g_struct_73_1050_14cc, (i_var5 + 0xec));
    sound_fn_1008_53ae(u_var3, extraout_DL, (i_var5 + 8));
    ShowCursor16(8, 1);
    win_func_1018_6bb6(param_1, (param_1 >> 0x10));
    return;
}

pub fn win_fn_1018_5e9a(param_1: *mut Struct20) {
    let mut u_var1: u32;
    char * *ppc_var2;
    let mut u_var3: i32;
    let i_var4: u16;
    let pu_var5: *mut u16;
    let struct_a: *mut Struct199;


    let mut i32_var6: i32;
    let mut i_var7: i32;
    let mut u_var8: u16;
    let mut u_var9: u16;
    let mut unaff_ss: u16;
    let ppVar10: *mut pass1_struct_1;
    let mut in_stack_0000ffb2: u16;
    let mut in_stack_0000ffb6: u16;
    let mut local_36: u16;
    let mut local_34: u16;
    let mut local_32: u16;
    let mut local_30: u32;
    let mut local_2c: u16;
    let mut local_2a: u16;
    let mut local_28: u16;
    let mut local_26: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: [u8; 6];
    let mut local_8: u16;
    let mut local_6: u32;

    win_gui_func_1040_78e2(param_1);
    ppVar10 = process_struct_1010_20ba(
        ctx._g_Struct372_1050_0ed0,
        CONCAT22(in_stack_0000ffb2, 0x39),
    );
    struct_a = (ppVar10 >> 0x10);
    u_var3 = ppVar10;
    u_var8 = (param_1 >> 0x10);
    i32_var6 = param_1;
    (i32_var6 + 0x8e) = u_var3;
    (i32_var6 + 0x90) = struct_a;
    process_struct_1000_179c(0x12, struct_a);
    if ((struct_a | u_var3) == 0) {
        (i32_var6 + 0x92) = 0;
    } else {
        pass1_1018_6198(CONCAT22(struct_a, u_var3), param_1, (i32_var6 + 6));
        (i32_var6 + 0x92) = u_var3;
        (i32_var6 + 0x94) = ctx.dx_reg;
    }
    u_var1 = (i32_var6 + 0x8e);
    local_6 = u_var1 & 0xffff0000 | (u_var1 + 10);
    GetClientRect16(CONCAT22(unaff_ss, local_e), (i32_var6 + 6));
    i_var4 = GetSystemMetrics16(4);
    (local_6 + 6) = i_var4 + local_8;
    ppVar10 = process_struct_1010_20ba(
        ctx._g_Struct372_1050_0ed0,
        CONCAT22(in_stack_0000ffb6, 0x48),
    );
    local_14 = (ppVar10 >> 0x10);
    local_16 = ppVar10;
    local_10 = (local_16 + 10);
    local_12 = (local_16 + 0xc);
    u_var9 = (local_6 >> 0x10);
    (local_6 + 2) = (local_12 - (local_6 + 6)) / 2;
    local_6 = local_10 / 2 + 3;
    pass1_1028_dc52(
        CONCAT22(unaff_ss, &local_28),
        (&ctx.PTR_LOOP_1050_0000 + 1),
        0,
        0x100,
    );
    while (true) {
        pu_var5 = &local_28;
        pass1_1028_e4ec(CONCAT22(unaff_ss, pu_var5));
        if ((ctx.dx_reg | pu_var5) == 0) {
            break;
        }
        ppc_var2 = (pu_var5 + 8);
        if (ppc_var2 != 0x0) {
            process_string_1000_3cea((param_1 & 0xffff0000 | (i32_var6 + 0x10)), *ppc_var2);
        }
    }
    u_var9 = (local_6 >> 0x10);
    i_var7 = local_6;
    SetWindowPos16(
        0x44,
        (i_var7 + 6),
        (i_var7 + 4),
        (i_var7 + 2),
        local_6,
        0,
        (i32_var6 + 6),
    );
    return;
}

pub fn win_cleanup_1018_4d22(param_1: *mut Struct376) {
    let pu_var1: *mut u32;
    let mut u_var2: i32;
    let ppc_var3: fn();
    let h_gdi_obj: HPALETTE16;
    let local_bx_4: *mut Struct43;
    let mut u_var4: u16;
    let mut unaff_cs: u16;

    u_var4 = (param_1 >> 0x10);
    local_bx_4 = param_1;
    param_1.ptr_a_lo = (s_SinternalPutBldg_site_0x_08lx__b_1050_5046 + 0x12);
    local_bx_4.field_0x2 = 0x1018;
    if (local_bx_4.h_dc != 0) {
        h_gdi_obj = SelectPalette16(0, local_bx_4.palette, local_bx_4.h_dc);
        DeleteObject16(h_gdi_obj);
        unaff_cs = SUB42(offset, 0);
        DeleteDC16(local_bx_4.h_dc);
    }
    pu_var1 = local_bx_4.field_0xa;
    u_var2 = local_bx_4.field_0xc;
    if ((u_var2 | pu_var1) != 0) {
        unsafe {
            ppc_var3 = *pu_var1;
        }
        (**ppc_var3)(unaff_cs, pu_var1, u_var2, 1);
    }
    pu_var1 = local_bx_4.field_0xe;
    u_var2 = local_bx_4.field_0x10;
    if ((u_var2 | pu_var1) != 0) {
        unsafe {
            ppc_var3 = *pu_var1;
        }
        (**ppc_var3)(unaff_cs, pu_var1, u_var2, 1);
    }
    _PTR_LOOP_1050_4230 = 0;
    pass1_1010_1d80(param_1);
    return;
}

pub fn win_fn_1018_2978(param_1: u32) {
    let mut u_var1: u32;
    let ppc_var2: fn();

    let pu_var3: Vec<u8>;
    let pu_var4: Vec<u8>;
    let pu_var5: *mut u16;
    let mut i32_var6: i32;

    let struct_a: *mut Struct199;
    let pa_var7: *mut Struct199;

    let mut u_var8: u16;
    let struct_a_00: *mut Struct199;

    let ctx.dx_reg: *mut Struct199;

    let mut i_var9: i32;
    let mut u_var10: u16;
    let mut u_var11: u16;
    let mut unaff_ss: u16;
    let u_var12: u8;
    let mut local_42: u32;
    let mut local_3e: u16;
    let mut local_3c: u16;
    let mut local_3a: u16;
    let mut local_38: u16;
    let mut local_36: u16;
    let mut local_34: u16;
    let mut local_32: u16;
    let mut local_30: u16;
    let mut local_2e: u16;
    let mut local_2c: u16;
    let mut local_2a: [u8; 36];
    let mut local_6: u16;
    let mut local_4: u16;

    win_gui_fn_1010_8096(ctx._g_struct_73_1050_14cc, 1);
    pu_var3 = local_2a;
    u_var12 = (unaff_ss >> 8);
    local_4 = ctx.dx_reg;
    process_struct_1008_48fe(
        CONCAT13(u_var12, CONCAT12(unaff_ss, pu_var3)),
        1,
        CONCAT22(ctx.dx_reg, in_ax),
    );
    u_var11 = 0x1000;
    pa_var7 = struct_a;
    process_struct_1000_179c(0x1e, struct_a);
    if ((pa_var7 | pu_var3) == 0) {
        pu_var4 = 0x0;
        u_var8 = 0;
    } else {
        pu_var4 = local_2a;
        u_var11 = SUB42(&ctx.PTR_LOOP_1050_1008, 0);
        pass1_1008_3f92(CONCAT22(pa_var7, pu_var3), CONCAT22(unaff_ss, pu_var4));
        u_var8 = ctx.dx_reg;
    }
    _local_2e = CONCAT22(u_var8, pu_var4);
    ppc_var2 = (*_local_2e + 0x14);
    ppc_var2(u_var11, pu_var4, u_var8);
    _local_32 = CONCAT22(struct_a_00, pu_var4);
    pa_var7 = struct_a_00;
    process_struct_1000_179c(0x14, struct_a_00);
    if ((pa_var7 | pu_var4) == 0) {
        pu_var4 = 0x0;
        u_var11 = 0;
    } else {
        process_struct_1008_4c58(CONCAT22(pa_var7, pu_var4));
        u_var11 = ctx.dx_reg;
    }
    u_var10 = (param_1 >> 0x10);
    i_var9 = param_1;
    (i_var9 + 0xe) = pu_var4;
    (i_var9 + 0x10) = u_var11;
    pass1_1008_4d84((i_var9 + 0xe), _local_32);
    pu_var5 = &local_3a;
    pa_var7 = ctx.dx_reg;
    GetClientRect16(
        CONCAT13(u_var12, CONCAT12(unaff_ss, pu_var5)),
        ctx.g_h_window,
    );
    u_var11 = 0x1000;
    process_struct_1000_179c(0x1e, pa_var7);
    if ((pa_var7 | pu_var5) == 0) {
        (i_var9 + 10) = 0;
    } else {
        i32_var6 = (local_34 - local_38) + 1;
        u_var1 = (i_var9 + 0xe);
        u_var11 = SUB42(&ctx.PTR_LOOP_1050_1008, 0);
        pass1_1008_405c(
            pu_var5,
            pa_var7,
            u_var1,
            (u_var1 >> 0x10),
            i32_var6,
            (local_36 - local_3a) + 1,
        );
        (i_var9 + 10) = i32_var6;
        (i_var9 + 0xc) = ctx.dx_reg;
    }
    if (_local_2e != 0x0) {
        ppc_var2 = *_local_2e;
        ppc_var2(
            u_var11,
            _local_2e,
            (_local_2e >> 0x10),
            1,
            u_var8,
            _local_2e,
            _local_2e,
        );
    }
    close_file_1008_496c(local_2a);
    return;
}

pub fn win_fn_1010_71d6(param_1: u32, param_2: u16, param_3: *mut u16) {
    let mut u_var1: u32;
    let mut u_var2: i32;
    let mut u_var3: u16;
    let mut in_ax: i32;
    let mut u_var4: u16;
    let mut u_var5: u16;

    let mut u_var6: i32;
    let mut u_var7: u16;
    let mut u_var8: u32;
    let mut local_14: u16;
    let mut local_10: u16;
    let mut local_e: u32;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    u_var7 = (param_1 >> 0x10);
    pass1_1010_ad22((param_1 + 0x14));
    _local_6 = CONCAT22(ctx.dx_reg, in_ax);
    if ((ctx.dx_reg | in_ax) == 0) {
        return;
    }
    u_var8 = pass1_1030_73a8(CONCAT22(ctx.dx_reg, in_ax));
    u_var6 = (u_var8 >> 0x10);
    u_var2 = u_var8;
    if (((u_var6 | u_var2) != 0) && ((u_var2 + 0x1c) == 0x8000002)) {
        return;
    }
    u_var1 = (in_ax + 0x2e);
    local_e._0_2_ = u_var1;
    if ((((in_ax + 0x30) | local_e) != 0) && ((local_e + 0x200) == 0x8000002)) {
        return;
    }
    u_var1 = (param_1 + 0x14);
    u_var4 = pass1_1010_b028(u_var1, (u_var1 >> 0x10), u_var8);
    u_var5 = (u_var2 + 0x12);
    u_var3 = u_var5;
    if ((u_var5 != 4) && (u_var3 = param_2, u_var5 == 7)) {
        param_2 = 5;
        u_var3 = param_2;
    }
    param_2 = u_var3;
    u_var5 = param_2 - 2;
    if (u_var5 != 0) {
        if (param_2 == 3) {
            u_var5 = u_var4 - 0xb;
            if ((u_var5 == 0) || (u_var5 = u_var4 - 0x37, u_var5 == 0)) {
                local_14 = 0xb;
            } else {
                local_14 = 10;
            }
            // goto LAB_1010_72a7;
        }
        u_var5 = param_2 - 4;
        if (u_var5 == 0) {
            local_14 = 0x17;
            // goto LAB_1010_72a7;
        }
        u_var5 = param_2 - 5;
        if (u_var5 != 0) {
            pass1_1010_7818(param_1, u_var8);
            local_14 = u_var5;
            // goto LAB_1010_72a7;
        }
    }
    local_14 = 0xc;
    // LAB_1010_72a7:
    if (local_14 == 0) {
        return;
    }
    win_gui_fn_1010_79aa(param_1, 0, _local_6);
    if (u_var5 == 0) {
        window_msg_func_1010_7300(param_1, 0, 0, local_14, _local_6);
    }
    return;
}

pub fn win_fn_1010_7174(param_1: u32, param_2: i32) {
    let mut u_var1: u32;
    let mut i_var2: i32;
    let mut u_var3: u16;

    i_var2 = param_1;
    u_var3 = (param_1 >> 0x10);
    if (param_2 == 0x13) {
        u_var1 = (i_var2 + 0x18);
        destroy_win_1010_7b26(param_1 & 0xffff0000 | (i_var2 - 10), (u_var1 + 0x28));
        return;
    }
    if (param_2 < 0x14) {
        if (param_2 == 0x1) {
            (i_var2 + 10) = 0;
            (i_var2 + 0x18) = 0;
            return;
        }
        if (param_2 == '\x05') {
            send_msg_1010_7c42(param_1 & 0xffff0000 | (i_var2 - 10));
            return;
        }
    }
    return;
}

pub fn win_fn_1040_c886(param_1: u32, param_2: u8, param_3: HDC16) {
    let pp_var1: fn();
    let p_uvar2: *mut u16;
    let h_gdi_obj: HPALETTE16;
    let mut i_var3: i32;
    let mut u_var4: u16;
    let mut u_var5: u16;
    let mut unaff_cs: u16;
    let mut unaff_ss: u16;
    let mut local_c: u16;
    let mut local_8: u32;
    let mut local_4: u16;

    u_var4 = (param_1 >> 0x10);
    i_var3 = param_1;
    if (((i_var3 + 10) | (i_var3 + 8)) != 0) {
        local_4 = 0;
        if ((i_var3 + 0x46) == 0) {
            u_var5 = (_PTR_LOOP_1050_4230 >> 0x10);
            local_c = (_PTR_LOOP_1050_4230 + 0xe);
            pu_var2 = &param_3;
            unaff_cs = SUB42(&ctx.PTR_LOOP_1050_1008, 0);
            realize_palette_1008_4e08(local_c, (_PTR_LOOP_1050_4230 + 0x10), pu_var2, unaff_ss);
            local_4 = pu_var2;
        }
        local_8 = (i_var3 + 8);
        u_var5 = (i_var3 + 10);
        if ((((i_var3 + 0xe) | (i_var3 + 0xc)) != 0) && ((param_2 & 1) != 0)) {
            local_8 = (i_var3 + 0xc);
            u_var5 = (i_var3 + 0xe);
        }
        if (((i_var3 + 0x10) != 0) && ((param_2 & 4) != 0)) {
            local_8 = (i_var3 + 0x10);
            u_var5 = (i_var3 + 0x12);
        }
        pp_var1 = (local_8 + 4);
        (**pp_var1)(
            unaff_cs,
            local_8,
            u_var5,
            (i_var3 + 0x28),
            (i_var3 + 0x26),
            &param_3,
        );
        if ((i_var3 + 0x46) == 0) {
            h_gdi_obj = SelectPalette16(0, local_4, param_3);
            DeleteObject16(h_gdi_obj);
        }
    }
    return;
}

pub fn win_fn_1040_c028(param_1: u32, param_2: i32) {
    let mut u_var1: u32;
    let mut u_var2: u32;
    let mut u_var3: u32;
    let mut i_var4: i32;
    let mut i_var5: i32;
    let mut u_var6: i32;
    let mut i_var7: i32;
    let mut u_var9: u16;
    let unaff_cs: bool;
    let unaff_ss: *mut u16;
    let rect: *mut RECT16;
    let mut local_34: u32;
    let mut local_30: u32;
    let mut local_2c: u32;
    let mut local_28: u32;
    let mut local_24: u32;
    let mut local_20: u16;
    let mut local_14: u16;
    let mut local_12: u32;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;
    let pu_var8: *mut u16;

    i_var7 = param_1;
    u_var9 = (param_1 >> 0x10);
    if (param_2 == 8) {
        GetClientRect16(CONCAT22(unaff_ss, &local_a), (i_var7 + 4));
        u_var1 = (i_var7 + 6);
        u_var3 = (i_var7 + 6);
        i_var5 = (u_var3 + 0x16);
        u_var3 = (i_var7 + 6);
        local_a = (u_var3 + 0x1a);
        u_var3 = (i_var7 + 6);
        local_8 = (u_var3 + 0x1c);
        if (i_var5 != 0) {
            if (i_var5 < 2) {
                i_var4 = 1;
            } else {
                i_var4 = 2;
            }
            u_var2 = ((i_var5 - i_var4) * 4 + u_var1 + 0x2a);
            i_var5 = u_var2;
            u_var2 = u_var2 & 0xffff0000;
            local_a = (i_var5 + 0x22) + (u_var2 | i_var5 + 0x1e);
        }
        u_var1 = (i_var7 + 6);
        local_6 = (u_var1 + 0x1e);
        local_4 = local_4 - 5;
    } else {
        if (param_2 != 9) {
            if (param_2 != 10) {
                return;
            }
            u_var1 = (i_var7 + 6);
            u_var6 = u_var1 + 0x2a;
            if (((i_var7 + 8) | u_var6) == 0) {
                return;
            }
            u_var3 = (i_var7 + 6);
            u_var2 = (((u_var3 + 0x16) + -1) * 4 + u_var6);
            i_var7 = u_var2;
            u_var2 = u_var2 & 0xffff0000;
            pu_var8 = (u_var2 | i_var7 + 0x1e);
            u_var9 = (u_var2 >> 0x10);
            local_8 = (i_var7 + 0x20) - 8;
            unsafe {
                local_a = *pu_var8;
            }
            unsafe {
                local_6 = (i_var7 + 0x22) + *pu_var8;
            }
            local_4 = (i_var7 + 0x20);
            rect = &local_a;
            unaff_cs = 0;
            // goto LAB_1040_c19d;
        }
        local_a = 0;
        local_8 = 0;
        local_6 = 0;
        local_4 = 0;
        GetClientRect16(CONCAT22(unaff_ss, &local_a), (i_var7 + 4));
        u_var1 = (i_var7 + 6);
        local_a = (u_var1 + 0x1a);
        u_var1 = (i_var7 + 6);
        local_6 = (u_var1 + 0x1e);
        local_4 = local_4 - 5;
        u_var1 = (i_var7 + 6);
        u_var3 = (i_var7 + 6);
        i_var7 = (u_var3 + 0x16);
        if (0 < i_var7) {
            u_var1 = (u_var1 + i_var7 * 4 + 0x26);
            i_var7 = u_var1;
            u_var9 = (u_var1 >> 0x10);
            local_8 = (i_var7 + 0x20) + (i_var7 + 0x24);
        }
    }
    unaff_ss = &local_a;
    rect = (&ctx.PTR_LOOP_1050_0000 + 1);
    // LAB_1040_c19d:
    InvalidateRect16(unaff_cs, rect, unaff_ss);
    return;
}

pub fn post_win_msg_1040_d2ac(param_1: *mut Struct124, param_2: u16, param_3: u16, param_4: u32) {
    let LVar1: LRESULT;

    if (param_2._2_2_ == (s_dibtext_bmp_1050_1844 + 4)) {
        SendDlgItemMessage16(
            0,
            0,
            0x405,
            (s_dibtext_bmp_1050_1844 + 3),
            &param_1.field_0x6,
        );
        pass1_1010_9172(&param_1[1].field_0x8);
    } else {
        if ((s_dibtext_bmp_1050_1844 + 4) < param_2._2_2_) {
            if (param_2._2_2_ == (s_dibtext_bmp_1050_1844 + 5)) {
                LVar1 = SendDlgItemMessage16(
                    0,
                    0,
                    0x40c,
                    (s_dibtext_bmp_1050_1844 + 3),
                    &param_1.field_0x6,
                );
                if ((LVar1 != -1) || ((LVar1 >> 0x10) != -1)) {
                    SendDlgItemMessage16(
                        0,
                        LVar1 - 1,
                        0x403,
                        (s_dibtext_bmp_1050_1844 + 3),
                        &param_1.field_0x6,
                    );
                    pass1_1010_91cc(&param_1[1].field_0x8);
                }
            } else {
                if (param_2._2_2_ == (s_dibtext_bmp_1050_1844 + 6)) {
                    enable_window_1040_d6be(param_1, param_2_00);
                    pass1_1018_57d2((param_1 + 1), CONCAT22(param_2_00, param_1));
                    PostMessage16(0, 0x203, 0x111, ctx.g_h_window);
                } else {
                    if (param_2._2_2_ != (s_dibtext_bmp_1050_1844 + 7)) {}
                    // goto LAB_1040_d3b3;
                    _ctx.PTR_LOOP_1050_5a68 = &param_1[1].field_0x4;
                    ui2::pass1_1038_af40(ctx._g_Struct112_a, *&param_1.field_0x6, 0x27);
                }
            }
        } else {
            if (param_2._2_2_ == 0xeb) {
                send_dialog_item_msg_1040_d79c(CONCAT22(param_2_00, param_1));
            } else {
                if (param_2._2_2_ != (s_vrpal_bmp_1050_183a + 7)) {
                    // LAB_1040_d3b3:
                    win_gui_fn_1040_b54a(param_1, param_2_00, param_3, param_2);
                    return;
                }
                msg_box_1040_d3d0(param_1, param_2_00);
            }
        }
    }
    return;
}

pub fn win_cleanup_1040_d1bc(param_1: &mut Struct44) {
    let pu_var1: *mut u32;
    let mut u_var2: i32;
    let ppc_var3: fn();
    let mut i_var4: i32;
    let mut u_var5: u16;

    u_var5 = (param_1 >> 0x10);
    i_var4 = param_1;
    param_1.ptr_a_lo = 0xd8c4;
    (i_var4 + 2) = &ctx.PTR_LOOP_1050_1040;
    pass1_1038_b6e0(ctx._g_Struct112_a, *(i_var4 + 6));
    pu_var1 = (i_var4 + 0x9c);
    u_var2 = (i_var4 + 0x9e);
    if ((u_var2 | pu_var1) != 0) {
        unsafe {
            ppc_var3 = *pu_var1;
        }
        (**ppc_var3)(&PTR_LOOP_1050_1038, pu_var1, u_var2, 1);
    }
    win_cleanup_func_1040_b0f8(param_1);
    return;
}

pub fn win_fn_1040_cace(param_1: u32) {
    let mut u_var1: u32;
    let mut b_var2: bool;
    let mut i_var3: i32;
    let i_var4: u16;
    let mut i_var5: i32;
    let mut u_var6: u16;
    let unaff_ss: u16;
    let mut bVar7: bool;
    let mut b: u16;
    let mut u_var8: u16;
    let mut local_218: u16;
    let mut local_214: u16;
    let mut local_20c: u32;
    let mut local_208: [u8; 256];
    let mut local_108: [u8; 256];
    let mut local_8: u16;
    let mut local_6: u16;
    let local_4: bool;

    u_var6 = (param_1 >> 0x10);
    i_var5 = param_1;
    local_6 = GetDlgItemInt16(0, &local_4, unaff_ss, 0x18e);
    local_8 = GetDlgItemInt16(0, &local_4, unaff_ss, 0x191);
    if (local_6 == 0) {
        return;
    }
    pass1_1018_50ea((i_var5 + 0x98), local_6, (i_var5 + 0x94));
    load_string_1010_84e0(
        ctx._g_struct_73_1050_14cc,
        (ctx._g_struct_73_1050_14cc >> 0x10),
        0x3ff,
        CONCAT22(unaff_ss, local_208),
        0x57b,
    );
    u_var1 = (i_var5 + 0x94);
    b = (ctx._g_struct_73_1050_14cc >> 0x10);
    if ((u_var1 + 0x36) == 0) {
        load_string_1010_84e0(
            ctx._g_struct_73_1050_14cc,
            b,
            0x3ff,
            CONCAT22(unaff_ss, local_108),
            0x7ed,
        );
        i_var3 = MessageBox16(
            0x34,
            CONCAT22(unaff_ss, local_208),
            CONCAT22(unaff_ss, local_108),
            (i_var5 + 8),
        );
    } else {
        load_string_1010_84e0(
            ctx._g_struct_73_1050_14cc,
            b,
            0x3ff,
            CONCAT22(unaff_ss, local_108),
            0x7ec,
        );
        i_var3 = MessageBox16(
            0x34,
            CONCAT22(unaff_ss, local_208),
            CONCAT22(unaff_ss, local_108),
            (i_var5 + 8),
        );
    }
    b_var2 = i_var3 == 6;
    bVar7 = false;
    if ((!b_var2) && (u_var1 = (i_var5 + 0x94), (u_var1 + 0x34) < 1)) {
        load_string_1010_84e0(
            ctx._g_struct_73_1050_14cc,
            (ctx._g_struct_73_1050_14cc >> 0x10),
            0x3ff,
            CONCAT22(unaff_ss, local_108),
            0x7ee,
        );
        i_var4 = MessageBox16(
            0x34,
            CONCAT22(unaff_ss, local_208),
            CONCAT22(unaff_ss, local_108),
            (i_var5 + 8),
        );
        bVar7 = i_var4 == 6;
        b_var2 = false;
    }
    if (b_var2) {
        _ctx.PTR_LOOP_1050_5f16 = (i_var5 + 0x94);
        u_var8 = 0x26;
    } else {
        if (!bVar7) {
            return;
        }
        _ctx.PTR_LOOP_1050_5a68 = (i_var5 + 0x94);
        u_var8 = 0x27;
    }
    ui2::pass1_1038_af40(ctx._g_Struct112_a, *(i_var5 + 8), u_var8);
    return;
}

pub fn destroy_win_1040_caa6(param_1: u16, param_2: u32) {
    let paVar1: *mut Struct318;
    let BVar2: bool;

    BVar2 = 0;
    paVar1 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, 0x2b);
    pass1_1010_038e(paVar1, BVar2);
    destroy_window_1040_b726(param_1, param_2);
    return;
}

pub fn win_fn_1008_84f4(param_1: u16, uparam_2_00: i32, param_2: WPARAM16, param_3: u32) {
    let pu8_var1: Vec<u8>;
    let mut u_var2: i32;
    let mut i_var3: i32;
    let mut i_var4: i32;
    let mut cVar5: u8;
    let b_var6: bool;
    let mut u_var7: u16;
    let mut unaff_ss: u16;
    let in_struct_1: &mut Struct44;
    let u_var8: u8;
    let HVar9: HWND16;
    let hwnd: HWND16;
    let mut local_1e: u16;
    let mut local_1c: u16;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u32;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_4: u16;

    hwnd = &ctx.g_alloc_addr_1050_1050;
    HVar9 = param_3._2_2_;
    in_struct_1 = GetWindowLong16(0, param_3._2_2_);
    u_var7 = (in_struct_1 >> 0x10);
    i_var4 = in_struct_1;
    if (param_3 == 0x1f) {
        (i_var4 + 4) = 0;
        KillTimer16(0xfa6, param_3._2_2_);
        KillTimer16(0xfa7, param_3._2_2_);
        ReleaseCapture16(hwnd);
    } else {
        cVar5 = param_3;
        u_var8 = (param_3 >> 0x10);
        if (param_3 < 0x20) {
            if (param_3 != 0x14) {
                if (0x14 < param_3) {}
                // goto LAB_1008_8771;
                if (cVar5 == 0x1) {
                    // LAB_1008_8560:
                    win_fn_1008_8214(u_var8, cVar5, param_2, param_1, param_2_00);
                    return;
                }
                if (cVar5 == 0x2) {
                    error_check_1000_17ce(in_struct_1);
                } else {
                    if (cVar5 != 0xc) {
                        if (cVar5 != 0xf) {}
                        // goto LAB_1008_8771;
                        draw_1008_8288(param_3._2_2_, i_var4, u_var7);
                    }
                }
            }
        } else {
            if (param_3 == 0x200) {
                if ((*(i_var4 + 4) & 1) != 0) {
                    GetClientRect16(CONCAT22(unaff_ss, &stack0xfff6), param_3._2_2_);
                    i_var3 = (i_var4 + 4);
                    pu8_var1 = (i_var4 + 4);
                    unsafe {
                        *pu8_var1 = *pu8_var1 & 0xf3;
                    }
                    b_var6 = PtInRect16(CONCAT22(param_2_00, param_1), &stack0xfff6);
                    if (b_var6 == 0) {
                        pu8_var1 = (i_var4 + 4);
                        unsafe {
                            *pu8_var1 = *pu8_var1 | 2;
                        }
                    } else {
                        if (param_2_00 < local_4 >> 1) {
                            pu8_var1 = (i_var4 + 4);
                            unsafe {
                                *pu8_var1 = *pu8_var1 | 4;
                            }
                        } else {
                            pu8_var1 = (i_var4 + 4);
                            unsafe {
                                *pu8_var1 = *pu8_var1 | 8;
                            }
                        }
                        pu8_var1 = (i_var4 + 4);
                        unsafe {
                            *pu8_var1 = *pu8_var1 & 0xfd;
                        }
                    }
                    if ((i_var4 + 4) != i_var3) {
                        InvalidateRect16(1, 0x0, 0);
                        UpdateWindow16(param_3._2_2_);
                    }
                }
            } else {
                if (param_3 < 0x201) {
                    if (param_3 == 0x81) {}
                    // goto LAB_1008_8560;
                    if (param_3 != 0x113) {
                        // LAB_1008_8771:
                        DefWindowProc16(
                            CONCAT13((param_2_00 >> 8), CONCAT12(param_2_00, param_1)),
                            param_2,
                            param_3,
                            param_3._2_2_,
                        );
                        return;
                    }
                    if (param_2 == 0xfa6) {
                        KillTimer16(0xfa6, param_3._2_2_);
                        SetTimer16(0, 0, 1, 0xfa7);
                        HVar9 = param_3._2_2_;
                    }
                    if ((*(i_var4 + 4) & 2) == 0) {
                        send_win_msg_1008_84ba(u_var8, i_var4, u_var7, HVar9);
                    }
                } else {
                    if (param_3 != 0x201) {
                        if (param_3 == 0x202) {
                            KillTimer16(0xfa6, param_3._2_2_);
                            KillTimer16(0xfa7, param_3._2_2_);
                            ReleaseCapture16(hwnd);
                            u_var2 = (i_var4 + 4);
                            if (((u_var2 & 1) != 0) && ((u_var2 & 0xfffd) != 0)) {
                                pu8_var1 = (i_var4 + 4);
                                unsafe {
                                    *pu8_var1 = *pu8_var1 & 0xf2;
                                }
                                InvalidateRect16(1, 0x0, 0);
                                UpdateWindow16(param_3._2_2_);
                            }
                            SendMessage16(*(i_var4 + 2), 0xf9, 0x111, in_struct_1.ptr_a_lo);
                            return;
                        }
                        if (param_3 != 0x203) {}
                        // goto LAB_1008_8771;
                    }
                    pu8_var1 = (i_var4 + 4);
                    unsafe {
                        *pu8_var1 = *pu8_var1 | 1;
                    }
                    GetClientRect16(CONCAT22(unaff_ss, &stack0xfff6), param_3._2_2_);
                    if (param_2_00 < (local_4 >> 1)) {
                        pu8_var1 = (i_var4 + 4);
                        unsafe {
                            *pu8_var1 = *pu8_var1 | 4;
                        }
                    } else {
                        pu8_var1 = (i_var4 + 4);
                        unsafe {
                            *pu8_var1 = *pu8_var1 | 8;
                        }
                    }
                    send_win_msg_1008_84ba(param_3._2_2_, i_var4, u_var7);
                    SetTimer16(0, 0, 300, 0xfa6);
                    InvalidateRect16(1, 0x0, 0);
                    UpdateWindow16(param_3._2_2_);
                    SetCapture16(param_3._2_2_);
                }
            }
        }
    }
    return;
}

pub fn win_fn_1008_8214(param_1: HWND16, param_2: i32, param_3: u16, param_4: u32) -> u16 {
    let mut in_ax: i32;
    let i_var1: u16;
    let in_dx: *mut Struct199;
    let mut u_var2: u32;
    let pu_var3: *mut u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    if (param_2 == 0x81) {
        process_struct_1000_179c(6, in_dx);
        if ((in_dx | in_ax) == 0) {
            u_var2 = 0;
        } else {
            u_var2 = clear_list_elements_1008_80d2(CONCAT22(in_dx, in_ax));
        }
        SetWindowLong16(u_var2, (u_var2 >> 0x10));
    }
    if (param_2 == 1) {
        pu_var3 = GetWindowLong16(0, param_1);
        unsafe {
            *pu_var3 = (param_3 + 8);
        }
        i_var1 = GetDlgCtrlID16(param_1);
        (pu_var3 + 2) = i_var1;
    }
    return 1;
}

pub fn win_fn_1008_5f44(param_1: u16, param_2: u32, param_3: u32) -> LRESULT {
    let wVar1: u16;
    let LVar2: LRESULT;
    let paVar3: *mut Struct219;
    let mut in_stack_0000fff8: u16;

    if (param_3 == 2) {
        wVar1 = GetWindowWord16(0, param_3._2_2_);
        mci_send_command_1008_5cb6(
            ctx._g_struct_ptr_1050_02a0,
            (ctx._g_struct_ptr_1050_02a0 >> 0x10),
            wVar1,
        );
        paVar3 = process_struct_1010_20ba(
            ctx._g_Struct372_1050_0ed0,
            CONCAT22(in_stack_0000fff8, 0x37),
        );
        pass1_1008_aa28(paVar3, paVar3);
    } else {
        if (param_3 != 0x3b9) {
            LVar2 = DefWindowProc16(
                CONCAT22(param_2, param_1),
                (param_2 >> 0x10),
                param_3,
                param_3._2_2_,
            );
            return LVar2;
        }
        DestroyWindow16(param_3._2_2_);
    }
    return 0;
}

pub fn win_fn_1008_3bd6(
    param_1: u16,
    param_2: u16,
    param_3: u16,
    param_3_00: u32,
    param_5: u32,
    param_6: u32,
    param_7: u32,
) {
    make_proc_inst_1040_8fb8(
        param_1,
        param_3,
        0,
        param_5,
        param_5._2_2_,
        param_6,
        param_6._2_2_,
        param_7,
    );
    CONCAT22(param_2, param_1) = 0x3cfc;
    (param_1 + 2) = &ctx.PTR_LOOP_1050_1008;
    (param_1 + 0x36) = 0;
    (param_1 + 0x26) = 0;
    process_struct_1040_9252(CONCAT22(param_2, param_1));
    create_win_1040_92dc(CONCAT22(param_2, param_1));
    update_window_1040_93aa(CONCAT22(param_2, param_1), param_3_00);
    return;
}

pub fn win_func_1008_3c34(param_1: u32, param_2: u8, param_3: HDC16) {
    let pp_var1: fn();
    let h_gdi_obj: HPALETTE16;
    let mut i_var2: i32;
    let mut u_var3: u16;
    let mut unaff_ss: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_6: u32;

    u_var3 = (param_1 >> 0x10);
    i_var2 = param_1;
    if (((i_var2 + 10) | (i_var2 + 8)) != 0) {
        local_6 = (i_var2 + 8);
        if (((i_var2 + 0xc) != 0) && ((param_2 & 1) != 0)) {
            local_6 = (i_var2 + 0xc);
        }
        if (((i_var2 + 0x10) != 0) && ((param_2 & 4) != 0)) {
            local_6 = (i_var2 + 0x10);
        }
        u_var3 = (_PTR_LOOP_1050_4230 >> 0x10);
        local_a = (_PTR_LOOP_1050_4230 + 0xe);
        local_c = &param_3;
        realize_palette_1008_4e08(local_a, (_PTR_LOOP_1050_4230 + 0x10), local_c, unaff_ss);
        pp_var1 = (local_6 + 4);
        (**pp_var1)();
        h_gdi_obj = SelectPalette16(0, local_c, param_3);
        DeleteObject16(h_gdi_obj);
    }
    return;
}

pub fn win_fn_1008_016e(ctx: &mut AppContext, param_1: u32) {
    let pp_var1: fn();
    let p_uvar2: *mut u16;
    let mut cVar3: u8;
    let mut in_ax: i32;
    let pu_var4: Vec<u8>;
    let pu_var5: Vec<u8>;
    let mut u_var6: u32;
    let mut u_var7: u32;
    let extraout_var: u32;
    let extraout_var_00: u32;
    let extraout_var_01: u32;
    let extraout_var_02: u32;
    let extraout_var_03: u32;
    let struct_a: *mut Struct199;
    let mut u_var8: i32;

    let ctx.dx_reg: *mut Struct199;
    let paVar9: *mut Struct199;
    let ctx.dx_reg: *mut Struct199;
    let pa_var10: *mut Struct199;
    let ctx.dx_reg: *mut Struct199;
    let ctx.dx_reg: *mut Struct199;
    let extraout_dx_04: *mut Struct199;
    let extraout_dx_05: *mut Struct199;
    let mut extraout_dx_06: u16;
    let mut u_var11: u16;
    let mut i_var12: i32;
    let mut u_var13: u16;
    let mut unaff_ss: u16;
    let u_var14: u8;
    let u_var15: u8;
    let mut in_stack_0000fe46: u16;
    let mut local_13e: [u8; 172];
    let mut local_92: [u8; 128];
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    GetVersion16();
    local_8 = in_ax & 0xff;
    u_var6 = local_8;
    local_a = in_ax >> 8;
    local_4 = struct_a;
    if ((local_8 < 3) || ((paVar9 = struct_a, local_8 == 3 && (local_a < 10)))) {
        u_var13 = 0x1000;
        local_10 = struct_a;
        process_struct_1000_179c(0xb4, struct_a);
        local_12 = u_var6;
        u_var8 = local_10 | local_12;
        if (u_var8 == 0) {
            u_var6 = 0;
            u_var8 = 0;
        } else {
            u_var13 = SUB42(&ctx.PTR_LOOP_1050_1040, 0);
            mixed_1040_8520(
                (u_var6 & 0xffff | local_10 << 0x10),
                0,
                0x10,
                2,
                0x5de,
                0x5dd,
            );
        }
        _local_e = (u_var6 & 0xffff | u_var8 << 0x10);
        pp_var1 = (*_local_e + 0x74);
        (**pp_var1)(u_var13, u_var6, u_var8);
        paVar9 = ctx.dx_reg;
        call_fn_ptr_1000_24cd(1);
    }
    fn_1008_6048(s_version__d__d_1050_0012, paVar9, SUB41(u_var6, 0));
    if ((local_8 == 3) && (0xb < local_a)) {
        ctx.PTR_LOOP_1050_0010 = (&ctx.PTR_LOOP_1050_0000 + 1);
    }
    LoadString16(
        0x80,
        CONCAT22(unaff_ss, local_92),
        0x578,
        ctx.g_h_instance_1050_038c,
    );
    pu_var4 = local_92;
    dos3_call_1000_51aa(pu_var4);
    if (pu_var4 != 0x0) {
        u_var14 = unaff_ss;
        u_var15 = (unaff_ss >> 8);
        LoadString16(
            0x80,
            CONCAT13(u_var15, CONCAT12(u_var14, local_13e)),
            0x57b,
            ctx.g_h_instance_1050_038c,
        );
        LoadString16(
            0x80,
            CONCAT13(u_var15, CONCAT12(u_var14, &stack0xfe42)),
            0x62e,
            ctx.g_h_instance_1050_038c,
        );
        pu_var4 = MessageBox16(
            0x10,
            CONCAT13(u_var15, CONCAT12(u_var14, local_13e)),
            CONCAT22(unaff_ss, &stack0xfe42),
            0,
        );
        call_fn_ptr_1000_24cd(1);
    }
    process_struct_1000_179c(4, paVar9);
    if ((paVar9 | pu_var4) == 0) {
        pu_var5 = 0x0;
        pa_var10 = 0x0;
        local_12 = pu_var4;
        local_10 = paVar9;
    } else {
        pu_var5 = pu_var4;
        local_12 = pu_var4;
        local_10 = paVar9;
        zero_array_val_1008_5394(CONCAT22(paVar9, pu_var4));
        pa_var10 = ctx.dx_reg;
    }
    u_var13 = (param_1 >> 0x10);
    i_var12 = param_1;
    (i_var12 + 8) = pu_var5;
    (i_var12 + 10) = pa_var10;
    u_var7 = (i_var12 + 8);
    pu_var2 = (i_var12 + 8);
    ctx._PTR_LOOP_1050_0298 = u_var7;
    *pu_var2 = 0x70;
    (pu_var2 + 2) = offset;
    process_struct_1000_179c(0x126, pa_var10);
    u_var8 = u_var7;
    paVar9 = (pa_var10 | u_var8);
    local_12 = u_var8;
    local_10 = pa_var10;
    if (paVar9 != 0x0) {
        pass1_1010_2024();
        paVar9 = ctx.dx_reg;
    }
    if (ctx._g_Struct372_1050_0ed0 == 0) {
        cVar3 = fn_1008_6048(s_New_failed_in_Op__Op_1050_0020, paVar9, SUB21(u_var8, 0));
        u_var8 = CONCAT31(extraout_var, cVar3);
        call_fn_ptr_1000_24cd(1);
    }
    process_struct_1000_179c(0xe8c, paVar9);
    pa_var10 = (paVar9 | u_var8);
    local_12 = u_var8;
    local_10 = paVar9;
    if (pa_var10 != 0x0) {
        pass1_1010_7e40(CONCAT22(paVar9, u_var8));
        pa_var10 = ctx.dx_reg;
    }
    if (ctx._g_struct_73_1050_14cc == 0) {
        cVar3 = fn_1008_6048(
            s_New_failed_in_Op__Op__ResLibr_1050_0035,
            pa_var10,
            SUB21(u_var8, 0),
        );
        u_var8 = CONCAT31(extraout_var_00, cVar3);
        call_fn_ptr_1000_24cd(1);
    }
    process_struct_1000_179c(0xb0, pa_var10);
    paVar9 = (pa_var10 | u_var8);
    local_12 = u_var8;
    local_10 = pa_var10;
    if (paVar9 != 0x0) {
        ui2::pass1_1038_aeca();
        paVar9 = ctx.dx_reg;
    }
    if (ctx._g_Struct112_a == 0) {
        cVar3 = fn_1008_6048(
            s_New_failed_in_Op__Op__DialogCtr_1050_0053,
            paVar9,
            SUB21(u_var8, 0),
        );
        u_var8 = CONCAT31(extraout_var_01, cVar3);
        call_fn_ptr_1000_24cd(1);
    }
    process_struct_1000_179c(10, paVar9);
    pa_var10 = (paVar9 | u_var8);
    local_12 = u_var8;
    local_10 = paVar9;
    if (pa_var10 != 0x0) {
        make_proc_inst_1038_cf6c();
        pa_var10 = extraout_dx_04;
    }
    if (ctx._PTR_LOOP_1050_1040 == 0) {
        cVar3 = fn_1008_6048(
            s_New_failed_in_Op__Op__DialogHand_1050_0073,
            pa_var10,
            SUB21(u_var8, 0),
        );
        u_var8 = CONCAT31(extraout_var_02, cVar3);
        call_fn_ptr_1000_24cd(1);
    }
    process_struct_1000_179c(0x14, pa_var10);
    paVar9 = (pa_var10 | u_var8);
    local_12 = u_var8;
    local_10 = pa_var10;
    if (paVar9 != 0x0) {
        modify_u16_list_1008_5bdc(CONCAT22(pa_var10, u_var8));
        paVar9 = extraout_dx_05;
    }
    if ctx._g_struct_ptr_1050_02a0 == 0 {
        cVar3 = fn_1008_6048(
            s_New_failed_in_Op__Op__Simulator_1050_0097,
            paVar9,
            SUB21(u_var8, 0),
        );
        u_var8 = CONCAT31(extraout_var_03, cVar3);
        call_fn_ptr_1000_24cd(1);
    }
    process_struct_1000_179c(0xfc, paVar9);
    local_10 = paVar9;
    local_12 = u_var8;
    if ((paVar9 | u_var8) == 0) {
        u_var8 = 0;
        u_var11 = 0;
    } else {
        win_fn_1008_0536();
        u_var11 = extraout_dx_06;
    }
    (i_var12 + 4) = u_var8;
    *(i_var12 + 6) = u_var11;
    if ((i_var12 + 4) == 0) {
        fn_1008_6048(s_New_failed_in_Op__Op_1050_00b7, u_var11, SUB21(u_var8, 0));
        call_fn_ptr_1000_24cd(1);
    }
    reg_class_1008_96d2((i_var12 + 4), in_stack_0000fe46);
    pp_var1 = ((i_var12 + 4) + 8);
    (**pp_var1)(0x1000);
    u_var7 = (i_var12 + 4);
    ctx.g_h_window = (u_var7 + 8);
    u_var7 = (i_var12 + 4);
    pp_var1 = ((i_var12 + 4) + 0xc);
    (**pp_var1)(0x1000, u_var7, (u_var7 >> 0x10), 3);
    u_var7 = (i_var12 + 4);
    UpdateWindow16((u_var7 + 8));
    return;
}

pub fn win_fn_1008_0536(param_1: *mut Struct180) {
    let mut u_var1: u16;
    let HVar2: HCURSOR16;
    let HVar3: HGDIOBJ16;
    let mut i_var4: i32;
    let mut u_var5: u16;
    let ppVar6: *mut pass1_struct_1;
    let HVar7: HINSTANCE16;

    process_struct_1008_3ab8(param_1);
    u_var5 = (param_1 >> 0x10);
    i_var4 = param_1;
    u_var1 = 0;
    (i_var4 + 0xe0) = 0;
    (i_var4 + 0xe4) = 0;
    (i_var4 + 0xe8) = 0;
    (i_var4 + 0xec) = 0;
    (i_var4 + 0xee) = 0;
    (i_var4 + 0xf2) = 0;
    (i_var4 + 0xf4) = 0;
    (i_var4 + 0xf8) = 0;
    param_1 = s_0_1050_389e;
    (i_var4 + 2) = &ctx.PTR_LOOP_1050_1008;
    (i_var4 + 200) = (s_572_bmp_1050_2007 + 1);
    (i_var4 + 0xac) = 0;
    (i_var4 + 0xae) = 0x8700;
    HVar7 = ctx.g_h_instance_1050_038c;
    LoadIcon16();
    (i_var4 + 0xc2) = u_var1;
    HVar2 = LoadCursor16(0x7f00, 0);
    (i_var4 + 0xc4) = HVar2;
    HVar3 = GetStockObject16(4);
    (i_var4 + 0xc6) = HVar3;
    process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(HVar7, 0x48));
    copy_string_1000_3d3e((param_1 & 0xffff0000 | (i_var4 + 10)), s_Outpost_1050_00d7);
    ppVar6 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(HVar7, 0x32));
    (i_var4 + 0xf4) = ppVar6;
    (i_var4 + 0xf6) = (ppVar6 >> 0x10);
    sys_color_func_1008_357e(i_var4, u_var5, 1);
    return;
}

pub fn win_cleanup_1008_0618(param_1: u32) {
    let pu_var1: *mut u32;
    let mut u_var2: i32;
    let ppc_var3: fn();
    let u_var4: u8;
    let extraout_var: u32;
    let mut u_var5: i32;

    let local_bx_5: *mut Struct47;
    let mut u_var6: u16;

    u_var6 = (param_1 >> 0x10);
    local_bx_5 = param_1;
    param_1 = s_0_1050_389e;
    local_bx_5.field_0x2 = &ctx.PTR_LOOP_1050_1008;
    sys_color_func_1008_357e(param_1, 0);
    error_check_1000_17ce(local_bx_5.field_0xf8);
    if (local_bx_5.field_0xec != 0) {
        DestroyMenu16(0x1000);
    }
    DestroyIcon16(local_bx_5.field_0xc2);
    local_bx_5.field_0xc2 = 0;
    pu_var1 = local_bx_5.field_0xe0;
    u_var2 = local_bx_5.field_0xe2;
    u_var5 = u_var2 | pu_var1;
    if (u_var5 != 0) {
        unsafe {
            ppc_var3 = *pu_var1;
        }
        (**ppc_var3)(offset, pu_var1, u_var2, 1);
        u_var5 = ctx.dx_reg;
    }
    u_var4 = pass1_1008_57c4((param_1 & 0xffff0000 | &local_bx_5.field_0xd2));
    param_1 = 0x380a;
    local_bx_5.field_0x2 = &ctx.PTR_LOOP_1050_1008;
    param_1 = ctx.s_1_1050_389a;
    local_bx_5.field_0x2 = &ctx.PTR_LOOP_1050_1008;
    return CONCAT31(extraout_var, u_var4) & 0xffff | u_var5 << 0x10;
}

pub fn track_popup_menu_1008_09ba(param_1: u32, param_2: u16) {
    let mut HVar1: u16;
    let mut i_var2: i32;
    let mut u_var3: u16;
    let mut unaff_cs: u16;
    let unaff_ss: HWND16;
    let mut local_6: u16;
    let mut local_4: u16;

    u_var3 = (param_1 >> 0x10);
    i_var2 = param_1;
    if ((i_var2 + 0xec) == 0) {
        HVar1 = LoadMenu16(s_OPPOPMENU_1050_0150, ctx.g_h_instance_1050_038c);
        (i_var2 + 0xec) = HVar1;
        if (HVar1 == 0) {
            return;
        }
        local_6 = (i_var2 + 0xec);
        unaff_cs = SUB42(offset, 0);
        HVar1 = GetSubMenu16(0, local_6);
        (i_var2 + 0xec) = HVar1;
        if (HVar1 == 0) {
            return;
        }
    }
    local_4 = param_2;
    local_6 = (i_var2 + 8);
    ClientToScreen16(CONCAT22(&local_6, unaff_cs), unaff_ss);
    TrackPopupMenu16(0x0, 0, ctx.g_h_window, 0, local_4, local_6, 0);
    return;
}

pub fn send_window_msg_1008_0a3c(param_1: u32, param_2: i32) -> u16 {
    let b_var1: bool;
    let mut i_var2: i32;
    let mut u_var3: u16;

    i_var2 = param_1;
    u_var3 = (param_1 >> 0x10);
    if ((param_2 & 0xfff0) == 0xf140) {
        return (i_var2 + 0xde);
    }
    if ((param_2 & 0xfff0) == 0xf060) {
        b_var1 = IsIconic16((i_var2 + 8));
        if (b_var1 == 0) {
            PostMessage16(0, 0x67, 0x111, (i_var2 + 8));
        }
        return 0;
    }
    return 1;
}

pub fn create_window_1008_0af8(in_win_struct: *mut win_struct_42) -> u16 {
    let pu_var1: *mut u16;
    let ppc_var2: fn();
    let local_win_handle: HANDLE16;
    let struct_a: *mut Struct199;
    let paVar3: *mut Struct199;

    let mut local_DX_85: i32;
    let struct_a_00: *mut Struct199;
    let mut local_DX_175: i32;

    let local_win_struct_42: *mut Struct119;
    let mut local_es_13: u16;
    let mut local_CS_138: u16;
    let paVar4: *mut Struct199;
    let local_16: u32;
    let mut local_6: u16;
    let mut temp_5fdbebe016: i32;
    char * *fn_ptr_1;
    let mut temp_5fc4833446: u32;

    paVar4 = create_win_1008_9760(in_win_struct);
    struct_a = (paVar4 >> 0x10);
    local_es_13 = (in_win_struct >> 0x10);
    local_win_struct_42 = in_win_struct;
    local_win_handle = local_win_struct_42.win_handle_0x8;
    ctx.g_h_window = local_win_handle;
    process_struct_1000_179c(0x12, struct_a);
    paVar3 = (struct_a | local_win_handle);
    if (paVar3 != 0x0) {
        set_timer_1008_91ba(local_win_handle, struct_a);
        paVar3 = ctx.dx_reg;
    }
    process_struct_1000_179c(6, paVar3);
    if ((paVar3 | local_win_handle) == 0) {
        &local_win_struct_42.field_0xe0 = 0;
    } else {
        pass1_1008_392e(
            CONCAT22(paVar3, local_win_handle),
            local_win_struct_42.win_handle_0x8,
        );
        local_win_struct_42.field_0xe0 = local_win_handle;
        local_win_struct_42.field_0xe2 = local_DX_85;
    }
    fn_ptr_1 = (in_win_struct + 0x14);
    (**fn_ptr_1)(0x1000, in_win_struct, 0, 0x15a, &ctx.g_alloc_addr_1050_1050);
    local_CS_138 = 0x1000;
    paVar3 = struct_a_00;
    process_struct_1000_179c(0xec, struct_a_00);
    _local_6 = CONCAT22(paVar3, local_win_handle);
    if ((paVar3 | local_win_handle) == 0) {
        &local_win_struct_42.field_0xe4 = 0;
    } else {
        pu_var1 = &local_win_struct_42.field_0xcc;
        unsafe {
            *pu_var1 = *pu_var1 + 1;
        }
        local_CS_138 = 0x1020;
        mci_fn_1020_08b6(_local_6, local_win_struct_42.field_0xcc, in_win_struct);
        local_win_struct_42.field_0xe4 = local_win_handle;
        local_win_struct_42.field_0xe6 = local_DX_175;
    }
    if (local_win_struct_42.field_0xce != 0) {
        ppc_var2 = (local_win_struct_42.field_0xce + 0x10);
        ppc_var2();
    }
    local_win_struct_42.field_0xce = &local_win_struct_42.field_0xe4;
    temp_5fc4833446 = &local_win_struct_42.field_0xe4;
    ppc_var2 = (&local_win_struct_42.field_0xe4 + 0x10);
    ppc_var2();
    temp_5fdbebe016 = local_win_struct_42.field_0xe6;
    local_win_struct_42.field_0xe8 = &local_win_struct_42.field_0xe4;
    ppc_var2 = (local_win_struct_42.field_0xe8 + 8);
    ppc_var2(
        local_CS_138,
        &local_win_struct_42.field_0xe8,
        temp_5fdbebe016,
        temp_5fc4833446,
        1,
    );
    ppc_var2 = (local_win_struct_42.field_0xe8 + 0xc);
    ppc_var2();
    pass1_1008_6978(
        (in_win_struct & 0xffff | local_es_13 << 0x10),
        0,
        local_win_struct_42.field_0xe8,
    );
    return ctx.dx_reg;
}

pub fn call_fill_client_window_1008_1230(param_1: &mut Vec<u8>) {
    fill_client_window_1008_39ac((param_1 + 0xe0));
    return;
}

pub fn win_fn_1008_12dc(param_1: u32, param_2: u32) {
    let pu_var1: Vec<u8>;
    let mut u_var2: u16;
    let mut u_var3: u16;
    let h_var4: HCURSOR16;

    let mut hwnd: i32;



    let mut u_var5: u16;
    let mut unaff_ss: u16;
    let mut local_28: u32;
    let mut local_24: u16;
    let mut local_22: u16;
    let mut local_20: u16;
    let mut local_1e: u16;
    let mut local_1c: u32;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: [u8; 6];
    let mut local_6: u16;
    let mut local_4: u16;

    local_4 = LoadCursor16(0x7f02, 0);
    local_6 = SetCursor16(local_4);
    pass1_1008_6d8a(
        CONCAT13((unaff_ss >> 8), CONCAT12(unaff_ss, local_c)),
        param_2,
    );
    pu_var1 = local_c;
    write_to_file_1008_6e02(pu_var1, unaff_ss);
    u_var5 = (param_1 >> 0x10);
    if (pu_var1 == 0x0) {
        SetCursor16(local_6);
        local_28._0_2_ = ctx._g_struct_73_1050_14cc;
        local_28._2_2_ = (ctx._g_struct_73_1050_14cc >> 0x10);
        u_var2 = ctx.g_u16_1050_0310;
        load_string_1010_847e(local_28, local_28._2_2_, ctx.g_u16_1050_0310);
        hwnd = ctx.dx_reg;
        pass1_fn_1008_60e8(u_var2, ctx.dx_reg, 0);
        _local_10 = CONCAT22(hwnd, u_var2);
        local_28._0_2_ = ctx._g_struct_73_1050_14cc;
        local_28._2_2_ = (ctx._g_struct_73_1050_14cc >> 0x10);
        u_var3 = u_var2;
        load_string_1010_847e(local_28, local_28._2_2_, 0x57b);
        MessageBeep16(0x10);
        MessageBox16(
            0x10,
            CONCAT22(ctx.dx_reg, u_var3),
            CONCAT22(hwnd, u_var2),
            (param_1 + 8),
        );
    } else {
        (ctx._g_bool_1050_5748 + 8) = 0;
        h_var4 = SetCursor16(local_6);
        local_28._0_2_ = ctx._g_struct_73_1050_14cc;
        local_28._2_2_ = (ctx._g_struct_73_1050_14cc >> 0x10);
        load_string_1010_847e(local_28, local_28._2_2_, 0x6d3);
        pass1_fn_1008_60e8(h_var4, ctx.dx_reg, ctx.dx_reg);
        local_28._0_2_ = ctx._g_struct_73_1050_14cc;
        local_28._2_2_ = (ctx._g_struct_73_1050_14cc >> 0x10);
        load_string_1010_847e(local_28, local_28._2_2_, 0x57b);
        MessageBeep16(0);
        hwnd = (param_1 + 8);
        MessageBox16(0x40, CONCAT22(ctx.dx_reg, h_var4), (hwnd << 0x10), hwnd);
        _local_10 = CONCAT22(hwnd, hwnd);
    }
    error_check_1000_17ce((_local_10 & 0xffff | hwnd << 0x10));
    close_file_1008_6dd0(CONCAT22(unaff_ss, local_c));
    return;
}

pub fn win_fn_1008_1414(param_1: u32, param_2: u32) {
    let mut u_var1: u32;
    let ppc_var2: fn();
    let BVar3: bool;
    let mut u_var4: u16;
    let mut u_var5: u16;
    let ppVar6: *mut pass1_struct_2;
    let mut i_var7: i32;


    let mut u_var8: u16;

    let mut unaff_DI: u16;
    let mut unaff_ss: u16;
    let pp_var9: *mut pass1_struct_1;
    let mut u_var10: u16;
    let u_var11: u8;
    let u_var12: u8;
    let mut local_50: u32;
    let mut local_4c: u16;
    let mut local_4a: u16;
    let mut local_3c: u16;
    let mut local_3a: u16;
    let mut local_38: u16;
    let mut local_36: u16;
    let mut local_34: u16;
    let mut local_32: u16;
    let mut local_30: u16;
    let mut local_2e: u16;
    let mut local_2c: u16;
    let mut local_2a: u32;
    let mut uStack38: u16;
    let mut local_24: u16;
    let mut local_22: u16;
    let mut local_20: u32;
    let mut local_1c: u16;
    let mut local_1a: u16;
    let mut local_18: u32;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u32;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: [u8; 6];

    pass1_1008_6d8a(CONCAT22(unaff_ss, local_8), param_2);
    BVar3 = close_file_1008_6e78(CONCAT22(unaff_ss, local_8));
    i_var7 = param_1;
    u_var10 = (param_1 >> 0x10);
    if (BVar3 == 0) {
        if (ctx.g_u16_1050_0310 == 0) {
            ctx.g_u16_1050_0310 = 0x6d4;
        }
        u_var4 = ctx.g_u16_1050_0310;
        load_string_1010_847e(
            ctx._g_struct_73_1050_14cc,
            (ctx._g_struct_73_1050_14cc >> 0x10),
            ctx.g_u16_1050_0310,
        );
        u_var8 = ctx.dx_reg;
        pass1_fn_1008_60e8(u_var4, ctx.dx_reg);
        u_var5 = u_var4;
        load_string_1010_847e(
            ctx._g_struct_73_1050_14cc,
            (ctx._g_struct_73_1050_14cc >> 0x10),
            0x57b,
        );
        MessageBeep16(0x10);
        MessageBox16(
            0x10,
            CONCAT22(ctx.dx_reg, u_var5),
            CONCAT22(u_var8, u_var4),
            (i_var7 + 8),
        );
        error_check_1000_17ce(CONCAT22(u_var8, u_var4));
        call_fn_ptr_1000_24cd(1);
    }
    set_cursor_1008_2dcc(i_var7, u_var10, 8);
    _local_c = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(unaff_DI, 0x2f));
    u_var8 = (_local_c >> 0x10);
    local_10 = (_local_c + 0x20);
    ppVar6 = pass1_1030_8344(
        ctx._g_bool_1050_5748,
        (ctx._g_bool_1050_5748 >> 0x10),
        local_10,
    );
    _local_14 = CONCAT22(u_var8, ppVar6);
    local_18 = &ppVar6.field_0x10;
    u_var1 = (i_var7 + 0xe8);
    ppc_var2 = ((i_var7 + 0xe8) + 4);
    ppc_var2(
        0x1030,
        u_var1,
        (u_var1 >> 0x10),
        local_10,
        (local_10 >> 0x10),
        (local_18 + 2) + -1,
        2,
    );
    local_22 = ctx.dx_reg;
    ppVar6 = pass1_1030_8344(
        ctx._g_bool_1050_5748,
        (ctx._g_bool_1050_5748 >> 0x10),
        0x4000001,
    );
    _local_1c = CONCAT22(local_22, ppVar6);
    local_20 = &ppVar6.field_0x10;
    local_24 = pass1_1030_8344(
        ctx._g_bool_1050_5748,
        (ctx._g_bool_1050_5748 >> 0x10),
        local_20,
    );
    local_2a = (local_24 + 0xc);
    uStack38 = (local_24 + 0x10);
    i_var7 = pass1_1030_5b00(_local_14);
    u_var11 = SUB21(&local_2a, 0);
    u_var12 = (&local_2a >> 8);
    u_var10 = unaff_ss;
    pp_var9 = process_struct_1010_20ba(
        ctx._g_Struct372_1050_0ed0,
        CONCAT13(u_var12, CONCAT12(u_var11, i_var7)),
    );
    pass1_1018_179e(pp_var9, CONCAT22(u_var10, CONCAT11(u_var12, u_var11)));
    u_var11 = 0;
    u_var12 = 4;
    u_var5 = 0x1b;
    u_var10 = 1;
    pp_var9 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, 0x1002b);
    pass1_1010_043a(
        pp_var9,
        CONCAT13(u_var12, CONCAT12(u_var11, u_var10)),
        u_var5,
    );
    close_file_1008_6dd0(CONCAT22(unaff_ss, local_8));
    return;
}

pub fn win_fn_1008_2b54(param_1: u32) -> u16 {
    let pp_var1: fn();
    let pc_var2: String;
    let mut in_ax: i32;
    let mut u_var3: u16;
    let in_dx: *mut Struct199;
    let mut u_var4: i32;
    let mut u_var5: u16;
    let mut unaff_ss: u16;
    let mut local_aa: u16;
    let mut local_a8: u16;
    let mut local_a6: u16;
    let mut local_a4: u16;
    let mut local_56: [u8; 80];
    let mut local_6: u16;
    let mut local_4: u16;

    local_4 = 0;
    if (_PTR_LOOP_1050_4230 == 0) {
        pc_var2 = load_string_1010_847e(
            ctx._g_struct_73_1050_14cc,
            (ctx._g_struct_73_1050_14cc >> 0x10),
            0x5f2,
        );
        copy_string_1000_3d3e(CONCAT22(unaff_ss, local_56), pc_var2);
        pc_var2 = load_string_1010_847e(
            ctx._g_struct_73_1050_14cc,
            (ctx._g_struct_73_1050_14cc >> 0x10),
            0x57b,
        );
        copy_string_1000_3d3e(CONCAT22(unaff_ss, &local_a6), pc_var2);
        local_4 = MessageBox16(
            0x21,
            CONCAT13((unaff_ss >> 8), CONCAT12(unaff_ss, &local_a6)),
            CONCAT22(unaff_ss, local_56),
            ctx.g_h_window,
        );
    } else {
        u_var5 = 0x1000;
        process_struct_1000_179c(0xb4, in_dx);
        u_var4 = in_dx | in_ax;
        if (u_var4 == 0) {
            u_var3 = 0;
            u_var4 = 0;
        } else {
            u_var5 = SUB42(&ctx.PTR_LOOP_1050_1040, 0);
            u_var3 = mixed_1040_8520(
                CONCAT22(in_dx, in_ax),
                ctx.g_h_window,
                0x21,
                2,
                0x57b,
                0x5f2,
            );
        }
        _local_a6 = CONCAT22(u_var4, u_var3);
        pp_var1 = (*_local_a6 + 0x74);
        local_4 = (**pp_var1)(u_var5, u_var3, u_var4, in_ax);
    }
    local_6 = local_4;
    if (local_4 != 1) {
        local_6 = 0;
    }
    if (((local_6 != 0) && (ctx._g_bool_1050_5748 != 0))
        && (
            u_var4 = (ctx._g_bool_1050_5748 + 8),
            _local_a6 = (_local_a6 & 0xffff0000 | u_var4),
            u_var4 != 0,
        ))
    {
        PostMessage16(0, 0xb4, 0x111, (param_1 + 8));
        local_6 = 0;
    }
    return local_6;
}

pub fn win_fn_1008_2d22(param_1: u32) {
    let pi_var1: *mut i32;
    let mut u_var2: u32;
    let ppc_var3: fn();
    let mut i_var4: i32;
    let mut u_var5: u16;
    let mut u_var6: u32;
    let mut u_var7: u16;
    let mut u_var8: u32;

    u_var5 = (param_1 >> 0x10);
    i_var4 = param_1;
    if (((i_var4 + 0xee) != 0)
        && (
            pi_var1 = (i_var4 + 0xf2),
            unsafe { *pi_var1 = *pi_var1 + -1 },
            (i_var4 + 0xf2) < 1,
        ))
    {
        u_var8 = (i_var4 + 0xee);
        ppc_var3 = ((i_var4 + 0xee) + 0x90);
        (**ppc_var3)();
        (i_var4 + 0xee) = 0;
        (i_var4 + 0xf2) = 0;
        if ((i_var4 + 0xe8) != 0) {
            u_var7 = 3;
            u_var6 = (i_var4 + 0xe8);
            ppc_var3 = ((i_var4 + 0xe8) + 0xc);
            (**ppc_var3)();
            ui2::show_win_1038_b68a(ctx._g_Struct112_a, (ctx._g_Struct112_a >> 0x10));
            u_var2 = (i_var4 + 0xf4);
            show_window_1010_7ace(u_var2, (u_var2 >> 0x10));
            u_var2 = (i_var4 + 0xe8);
            ppc_var3 = ((i_var4 + 0xe8) + 0x98);
            (**ppc_var3)(0x1010, u_var2, (u_var2 >> 0x10), 1, u_var6, u_var7, u_var8);
            PostMessage16(0, 0xfc, 0x111, ctx.g_h_window);
        }
    }
    return;
}

pub fn win_fn_1008_3018(param_1: u32) {
    let string_b: String;
    let mut u_var1: u16;
    let mut i_var2: i32;
    let ctx.ax_reg: *mut Struct179;
    let mut str_index: u16;
    let mut local_DX_17: i32;
    let mut unaff_si: u16;
    let mut ctx.stack_seg_reg: i32;
    let mut local_114: u16;
    let mut local_112: u32;
    let mut local_10a: u32;
    let mut local_106: u32;
    let mut local_str: [u8; 256];

    local_str[0] = '\0';
    local_106 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(unaff_si, 2));
    u_var1 = (local_106 >> 0x10);
    i_var2 = local_106;
    string_b = *(i_var2 + 0x12);
    local_10a._0_2_ = string_b;
    if (((i_var2 + 0x14) | local_10a) == 0) {
        open_save_1008_30cc(param_1);
    } else {
        copy_string_1000_3d3e(CONCAT22(ctx.stack_seg_reg, local_str), *(i_var2 + 0x1a));
        str_index = get_string_index_1000_3da4(CONCAT22(ctx.stack_seg_reg, local_str));
        if (local_str[str_index - 1] != '\\') {
            local_str[str_index] = '\\';
            local_str[str_index + 1] = '\0';
        }
        process_string_1000_3cea(CONCAT22(ctx.stack_seg_reg, local_str), string_b);
        if (local_str[0] != '\0') {
            win_fn_1008_12dc(param_1, local_str, ctx.stack_seg_reg);
            return;
        }
    }
    return;
}

pub fn post_quit_msg_1008_3af4() {
    PostQuitMessage16();
    return;
}

pub fn win_cleanup_fn_1040_a294(param_1: &mut Struct44) {
    let mut local_CS__1: u16;

    param_1.ptr_a_lo = 0xa4e8;
    (param_1 + 2) = &ctx.PTR_LOOP_1050_1040;
    PTR_LOOP_1050_5eda = PTR_LOOP_1050_5eda + -1;
    if (PTR_LOOP_1050_5eda == 0x0) {
        FreeProcInstance16(CONCAT22(_PTR_LOOP_1050_5edc, local_CS__1));
        _PTR_LOOP_1050_5edc = 0;
    }
    win_cleanup_func_1040_b0f8(param_1);
    return;
}

pub fn win_gui_fn_1040_a2cc(param_1: *mut Struct124, param_2: u32, param_3: u32) {
    let mut u_var1: i32;
    let mut u_var2: u32;

    if (param_3._2_2_ == 0x1826) {
        if ((param_3 == 1) || (1 < param_3 - 1 && (param_3 - 3 < 2))) {
            u_var1 = 1;
        } else {
            u_var1 = 0;
        }
        return u_var1;
    }
    u_var2 = win_gui_fn_1040_b54a(param_1, param_2, (param_2 >> 0x10), param_3);
    return u_var2;
}

pub fn make_proc_inst_1040_a234(param_1: Vec<u8>, param_2: Vec<u8>, param_3: u16, param_4: u32) {
    let unaff_cs: HANDLE16;

    pass1_1040_b040(
        CONCAT22(param_2, param_1),
        CONCAT22(param_3, param_3_00),
        (param_3 >> 0x10),
    );
    CONCAT22(param_2, param_1) = 0xa4e8;
    (param_1 + 2) = &ctx.PTR_LOOP_1050_1040;
    if (_PTR_LOOP_1050_5edc == 0) {
        _PTR_LOOP_1050_5edc =
            MakeProcInstance16(unaff_cs, CONCAT22(0xa40e, ctx.g_h_instance_1050_038c));
    }
    (param_1 + 0xc) = _PTR_LOOP_1050_5edc;
    PTR_LOOP_1050_5eda = PTR_LOOP_1050_5eda + 1;
    PTR_LOOP_1050_5ee0 = param_1;
    PTR_LOOP_1050_5ee2 = param_2;
    return;
}

pub fn win_fn_1040_9ce0(param_1: i32, param_2: u16, param_4: HWND16, param_3: u32) {
    let pu8_var1: Vec<u8>;
    let mut i_var2: i32;
    let mut id: u16;
    let mut i_var3: i32;
    let mut u8_var4: u8;

    let WVar5: WPARAM16;
    let b_var6: bool;
    let mut offset: i32;
    let struct_a: *mut Struct199;
    let ctx.dx_reg: HWND16;
    let mut u_var7: u16;
    let mut unaff_ss: u16;
    let in_struct_1: &mut Struct44;
    let pWVar8: *mut WPARAM16;
    let LVar9: LRESULT;
    let mut u_var10: u32;
    let mut u_var11: u16;
    let h_var12: HWND16;
    let mut local_1e: u16;
    let mut local_1c: u16;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u32;
    let local_a: RECT16;

    h_var12 = &ctx.g_alloc_addr_1050_1050;
    in_struct_1 = GetWindowLong16(0, param_3._2_2_);
    struct_a = (in_struct_1 >> 0x10);
    i_var3 = in_struct_1;
    u_var7 = ((in_struct_1 & 0xffff0000) >> 0x10);
    if (param_3 == 0x30) {
        (i_var3 + 0x5a) = param_2;
    } else {
        if (param_3 < 0x31) {
            if (param_3 == 0x1f) {
                (i_var3 + 4) = 0;
                ReleaseCapture16(h_var12);
                return;
            }
            if (0x1f < param_3) {}
            // goto LAB_1040_a1ae;
            u8_var4 = param_3;
            if (u8_var4 == 8) {
                pu8_var1 = (i_var3 + 4);
                unsafe {
                    *pu8_var1 = *pu8_var1 & 0xf7;
                }
                local_1e._0_1_ = false;
                b_var6 = IsWindow16(param_2);
                if (b_var6 != 0) {
                    u_var10 = SendMessage16(0, 0, 0x87, param_2);
                    local_1e._0_1_ = (u_var10 & 0x20) == 0;
                }
                (i_var3 + 0x56) = 0;
                SendMessage16(0, (i_var3 + 0x5c), 0x401, (i_var3 + 2));
                if (((i_var3 + 0x5c) != 0) && ((i_var3 + 0x5c) != in_struct_1.ptr_a_lo)) {
                    SendDlgItemMessage16(local_1e, 1, 0x404, (i_var3 + 0x5c), (i_var3 + 2));
                }
                (i_var3 + 0x5c) = 0;
            } else {
                if (u8_var4 < 9) {
                    if (u8_var4 == 1) {
                        pWVar8 = GetWindowLong16(0, param_3._2_2_);
                        i_var3 = pWVar8;
                        u_var7 = ((pWVar8 & 0xffff0000) >> 0x10);
                        (i_var3 + 2) = (param_1 + 8);
                        WVar5 = GetDlgCtrlID16(param_3._2_2_);
                        unsafe {
                            *pWVar8 = WVar5;
                        }
                        (i_var3 + 0x56) = (param_1 + 0x12);
                        copy_string_1000_3d3e(
                            (pWVar8 & 0xffff0000 | (i_var3 + 6)),
                            *(param_1 + 0x16),
                        );
                        if ((*(param_1 + 0x12) & 1) != 0) {
                            SendMessage16(0, unsafe { *pWVar8 }, 0x401, (param_1 + 8));
                        }
                        if (((param_1 + 0x14) & 0x800) == 0) {
                            return;
                        }
                        pu8_var1 = (i_var3 + 4);
                        unsafe {
                            *pu8_var1 = *pu8_var1 | 4;
                        }
                        return;
                    }
                    if (u8_var4 == 2) {
                        error_check_1000_17ce(in_struct_1);
                        SetWindowLong16(0, 0);
                        return;
                    }
                    if (u8_var4 != 7) {}
                    // goto LAB_1040_a1ae;
                    pu8_var1 = (i_var3 + 4);
                    unsafe {
                        *pu8_var1 = *pu8_var1 | 8;
                    }
                    LVar9 = SendMessage16(0, 0, 0x400, (i_var3 + 2));
                    id = LVar9;
                    if (((LVar9 >> 0x10) == 0x534b)
                        && ((i_var3 + 0x5c) = id, id != in_struct_1.ptr_a_lo))
                    {
                        SendDlgItemMessage16(1, 0, 0x404, id, (i_var3 + 2));
                    }
                    SendMessage16(0, in_struct_1.ptr_a_lo, 0x401, (i_var3 + 2));
                    (i_var3 + 0x56) = 1;
                } else {
                    if (u8_var4 == 10) {
                        pu8_var1 = (i_var3 + 4);
                        unsafe {
                            *pu8_var1 = *pu8_var1 & 0xfb;
                        }
                        if (param_2 == 0) {
                            pu8_var1 = (i_var3 + 4);
                            unsafe {
                                *pu8_var1 = *pu8_var1 | 4;
                            }
                        }
                    } else {
                        if (u8_var4 != 0xc) {
                            if (u8_var4 == 0xf) {
                                draw_1040_9948(param_3._2_2_, i_var3, u_var7);
                                return;
                            }
                            // goto LAB_1040_a1ae;
                        }
                        if (CONCAT22(param_2_00, param_1) != 0) {
                            copy_string_1000_3d3e(
                                (in_struct_1 & 0xffff0000 | (i_var3 + 6)),
                                CONCAT22(param_2_00, param_1),
                            );
                        }
                    }
                }
            }
            // goto LAB_1040_9e20;
        }
        if (param_3 == 0x200) {
            if ((*(i_var3 + 4) & 1) == 0) {
                return;
            }
            GetClientRect16(CONCAT22(unaff_ss, &local_a), param_3._2_2_);
            i_var2 = (i_var3 + 4);
            b_var6 = PtInRect16(CONCAT22(param_2_00, param_1), &local_a);
            if (b_var6 == 0) {
                pu8_var1 = (i_var3 + 4);
                unsafe {
                    *pu8_var1 = *pu8_var1 & 0xfd;
                }
            } else {
                pu8_var1 = (i_var3 + 4);
                unsafe {
                    *pu8_var1 = *pu8_var1 | 2;
                }
            }
            param_1 = (i_var3 + 4) - i_var2;
        } else {
            if (param_3 < 0x201) {
                offset = param_3 - 0x81;
                if (offset == 0) {
                    process_struct_1000_179c(0x5e, struct_a);
                    if ((struct_a | offset) == 0) {
                        offset = 0;
                        h_var12 = 0;
                    } else {
                        process_struct_1040_9824(CONCAT22(struct_a, offset));
                        h_var12 = ctx.dx_reg;
                    }
                    SetWindowLong16(offset, h_var12);
                    return;
                }
                if (param_3 == 0x87) {
                    return;
                }
                h_var12 = param_3 - 0x100;
                if (h_var12 == 0) {
                    if ((param_2 == 0x26) || (param_2 == 0x25)) {
                        u_var7 = (i_var3 + 2);
                        u_var11 = 1;
                    } else {
                        if ((param_2 != 0x28) && (param_2 != 0x27)) {
                            if (((param_2 == 0x20) || (param_2 == 0xd))
                                && (&PTR_LOOP_1050_5ed8 == 0))
                            {
                                &PTR_LOOP_1050_5ed8 = 1;
                                pu8_var1 = (i_var3 + 4);
                                unsafe {
                                    *pu8_var1 = *pu8_var1 | 2;
                                }
                                // goto LAB_1040_9e20;
                            }
                            // goto LAB_1040_a1ae;
                        }
                        u_var7 = (i_var3 + 2);
                        u_var11 = 0;
                    }
                    GetNextDlgTabItem16(::offset, u_var11, param_3._2_2_, u_var7);
                    SetFocus16(h_var12);
                    return;
                }
                if ((param_3 == 0x101) && (&PTR_LOOP_1050_5ed8 != 0)) {
                    &PTR_LOOP_1050_5ed8 = 0;
                    pu8_var1 = (i_var3 + 4);
                    unsafe {
                        *pu8_var1 = *pu8_var1 & 0xfd;
                    }
                    InvalidateRect16(1, 0x0, 0);
                    UpdateWindow16(param_3._2_2_);
                    SendMessage16(0, in_struct_1.ptr_a_lo, 0x111, (i_var3 + 2));
                    return;
                }
                // LAB_1040_a1ae:
                DefWindowProc16(
                    CONCAT22(param_2_00, param_1),
                    param_2,
                    param_3,
                    param_3._2_2_,
                );
                return;
            }
            if (param_3 == 0x201) {
                // LAB_1040_9e74:
                SetFocus16(param_3._2_2_);
                pu8_var1 = (i_var3 + 4);
                unsafe {
                    *pu8_var1 = *pu8_var1 | 3;
                }
                InvalidateRect16(1, 0x0, 0);
                UpdateWindow16(param_3._2_2_);
                SetCapture16(param_3._2_2_);
                return;
            }
            if (param_3 == 0x202) {
                ReleaseCapture16(h_var12);
                GetClientRect16(CONCAT22(unaff_ss, &local_a), param_3._2_2_);
                if ((*(i_var3 + 4) & 1) == 0) {
                    return;
                }
                pu8_var1 = (i_var3 + 4);
                unsafe {
                    *pu8_var1 = *pu8_var1 & 0xfc;
                }
                InvalidateRect16(1, 0x0, 0);
                UpdateWindow16(param_3._2_2_);
                local_1e._0_1_ = param_2_00;
                local_1e._1_1_ = (param_2_00 >> 8);
                b_var6 = PtInRect16(
                    CONCAT13(local_1e._1_1_, CONCAT12(local_1e, param_1)),
                    &local_a,
                );
                if (b_var6 == 0) {
                    return;
                }
                PostMessage16(0, in_struct_1.ptr_a_lo, 0x111, (i_var3 + 2));
                return;
            }
            if (param_3 == 0x203) {}
            // goto LAB_1040_9e74;
            if (param_3 != 0x404) {}
            // goto LAB_1040_a1ae;
            if (param_2 == 1) {
                (i_var3 + 0x56) = 1;
            } else {
                (i_var3 + 0x56) = 0;
            }
        }
    }
    if (param_1 == 0) {
        return;
    }
    // LAB_1040_9e20:
    InvalidateRect16(1, 0x0, 0);
    UpdateWindow16(param_3._2_2_);
    return;
}

pub fn load_cursor_1040_9854(param_1: *mut u16) -> *mut u16 {
    let HVar1: HCURSOR16;
    let HVar2: HGDIOBJ16;
    let mut i_var3: i32;
    let mut u_var4: u16;

    u_var4 = (param_1 >> 0x10);
    i_var3 = param_1;
    unsafe {
        *param_1 = ctx.s_1_1050_389a;
    }
    (i_var3 + 2) = &ctx.PTR_LOOP_1050_1008;
    unsafe {
        *param_1 = 0xa230;
    }
    (i_var3 + 2) = &ctx.PTR_LOOP_1050_1040;
    copy_string_1000_3d3e((param_1 & 0xffff0000 | (i_var3 + 4)), s_OPButton_1050_5ece);
    (i_var3 + 0x54) = 3;
    HVar1 = LoadCursor16(0x7f00, 0);
    (i_var3 + 0x58) = HVar1;
    HVar2 = GetStockObject16(4);
    (i_var3 + 0x56) = HVar2;
    reg_class_1040_98c0(i_var3, u_var4);
    return param_1;
}

pub fn call_win_proc_1040_9686(param_1: u16, param_2: u16, param_3_00: WPARAM16, param_3: u32) {
    let pp_var1: fn();

    let HVar2: HANDLE16;
    let HVar3: HANDLE16;
    let b_var4: bool;
    let mut in_dx: u16;
    let mut unaff_ss: u16;
    let u_var5: u8;
    let local_1a: RECT16;
    let mut local_12: u32;
    let mut local_e: u32;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    u_var5 = (in_ax >> 8);
    HVar2 = GetProp16(CONCAT13(u_var5, CONCAT12(in_ax, 0x5e7d)), param_3._2_2_);
    HVar3 = GetProp16(CONCAT13(u_var5, CONCAT12(in_ax, 0x5e76)), param_3._2_2_);
    _local_6 = CONCAT22(HVar2, HVar3);
    HVar2 = GetProp16(CONCAT22(in_ax, 0x5e8b), param_3._2_2_);
    HVar3 = GetProp16(CONCAT22(in_ax, 0x5e84), param_3._2_2_);
    _local_a = CONCAT22(HVar2, HVar3);
    if ((HVar2 | HVar3) != 0) {
        if (param_3 == 2) {
            local_12 = _local_a;
            local_e = _local_a;
            if (_local_a != 0x0) {
                pp_var1 = *_local_a;
                (**pp_var1)(offset, HVar3, HVar2, 1);
            }
        } else {
            if (param_3 == 0x201) {
                HVar2 = GetProp16(CONCAT22(in_ax, 0x5e92), param_3._2_2_);
                if (HVar2 == 0) {
                    GetClientRect16(CONCAT22(unaff_ss, &local_1a), (_local_a + 0x18));
                    b_var4 = PtInRect16(CONCAT22(param_2, param_1), &local_1a);
                    if (b_var4 == 0) {
                        return;
                    }
                    fn_1008_6048(CONCAT22(in_ax, 0x5e98), in_dx, SUB21(b_var4, 0));
                    pp_var1 = (*_local_a + 0x1c);
                    (**pp_var1)(
                        &ctx.PTR_LOOP_1050_1008,
                        _local_a,
                        (_local_a >> 0x10),
                        param_2,
                        param_1,
                        param_3_00,
                    );
                    return;
                }
            } else {
                if (param_3 == 0x204) {
                    GetClientRect16(CONCAT22(unaff_ss, &local_1a), (HVar3 + 0x18));
                    b_var4 = PtInRect16(CONCAT22(param_2, param_1), &local_1a);
                    if (b_var4 == 0) {
                        return;
                    }
                    fn_1008_6048(CONCAT22(in_ax, 0x5eab), in_dx, SUB21(b_var4, 0));
                    pp_var1 = (*_local_a + 0x20);
                    (**pp_var1)(
                        8,
                        _local_a,
                        (_local_a >> 0x10),
                        param_2,
                        param_1,
                        param_3_00,
                    );
                    return;
                }
            }
        }
    }
    if (_local_6 != 0) {
        CallWindowProc16(
            CONCAT13((param_2 >> 8), CONCAT12(param_2, param_1)),
            param_3_00,
            param_3,
            param_3._2_2_,
            _local_6,
        );
    }
    return;
}

pub fn update_window_1040_93aa(param_1: u32, param_2: u16, param_3: u16) {
    let mut i_var1: i32;
    let mut u_var2: u16;

    u_var2 = (param_1 >> 0x10);
    i_var1 = param_1;
    (i_var1 + 0x1e) = param_3;
    (i_var1 + 0x20) = param_2;
    MoveWindow16(
        1,
        (i_var1 + 0x24),
        (i_var1 + 0x22),
        param_2,
        (i_var1 + 0x1e),
        (i_var1 + 0x18),
    );
    UpdateWindow16((i_var1 + 0x18));
    return;
}

pub fn create_win_1040_92dc(param_1: *mut Struct41) {
    let mut window: u16;
    let handle: HANDLE16;
    let local_bx_4: *mut Struct41;
    let handle_00: HANDLE16;
    let lVar1: u32;

    handle_00 = (param_1 >> 0x10);
    local_bx_4 = param_1;
    if (local_bx_4.field_0x18 == 0) {
        window = CreateWindow16(
            0,
            ctx.g_h_instance_1050_038c,
            local_bx_4.field_0x1c,
            local_bx_4.field_0x1a,
            0,
            0,
            local_bx_4.field_0x20,
            local_bx_4.field_0x1e,
            0x4000000b,
            s__1050_5e3e,
            s_button_1050_5e3f,
        );
        local_bx_4.field_0x18 = window;
        lVar1 = SetWindowLong16(_g_proc_inst_1050_5e18, (_g_proc_inst_1050_5e18 >> 0x10));
        handle = (lVar1 >> 0x10);
        local_bx_4.field_0x14 = lVar1;
        local_bx_4.field_0x16 = handle;
        SetProp16(handle, s_procHi_1050_5e46, local_bx_4.field_0x18);
        SetProp16(
            local_bx_4.field_0x14,
            s_procLo_1050_5e4d,
            local_bx_4.field_0x18,
        );
        SetProp16(handle_00, s_thisHi_1050_5e54, local_bx_4.field_0x18);
        SetProp16(local_bx_4, s_thisLo_1050_5e5b, local_bx_4.field_0x18);
        if (local_bx_4.field_0x40 != 0) {
            SetProp16(1, s_IsDlg_1050_5e62, local_bx_4.field_0x18);
        }
        ShowWindow16(5, local_bx_4.field_0x18);
    }
    return;
}

pub fn enable_window_1040_9234(param_1: u32, param_2: bool) {
    let mut u_var1: u16;

    u_var1 = (param_1 >> 0x10);
    if ((param_1 + 0x18) != 0) {
        EnableWindow16(param_2, (param_1 + 0x18));
    }
    return;
}

pub fn free_proc_inst_1040_911e(in_struct_1: &mut Struct7) {
    let pu_var1: *mut u32;
    let mut u_var2: i32;
    let mut u_var3: u32;
    let ppcVar4: fn();
    let local_struct_1: *mut Struct356;
    let mut u_var5: u16;

    u_var5 = (in_struct_1 >> 0x10);
    local_struct_1 = in_struct_1;
    in_struct_1 = 0x9800;
    local_struct_1.field_0x2 = &ctx.PTR_LOOP_1050_1040;
    if (local_struct_1.field_0x38 != 0) {
        pu_var1 = local_struct_1.field_0x8;
        u_var2 = local_struct_1.field_0xa;
        if ((u_var2 | pu_var1) != 0) {
            unsafe {
                ppcVar4 = *pu_var1;
            }
            (**ppcVar4)();
        }
        pu_var1 = local_struct_1.field_0xc;
        u_var2 = local_struct_1.field_0xe;
        if ((u_var2 | pu_var1) != 0) {
            unsafe {
                ppcVar4 = *pu_var1;
            }
            (**ppcVar4)();
        }
        pu_var1 = local_struct_1.field_0x10;
        u_var2 = local_struct_1.field_0x12;
        if ((u_var2 | pu_var1) != 0) {
            unsafe {
                ppcVar4 = *pu_var1;
            }
            (**ppcVar4)();
        }
    }
    error_check_1000_17ce(local_struct_1.field_0x4);
    u_var3 = local_struct_1.field_0x14;
    SetWindowLong16(u_var3, (u_var3 >> 0x10));
    RemoveProp16(s_thisLo_1050_5e1c, local_struct_1.field_0x18);
    RemoveProp16(s_thisHi_1050_5e23, local_struct_1.field_0x18);
    RemoveProp16(s_procLo_1050_5e2a, local_struct_1.field_0x18);
    RemoveProp16(s_procHi_1050_5e31, local_struct_1.field_0x18);
    RemoveProp16(s_IsDlg_1050_5e38, local_struct_1.field_0x18);
    PTR_LOOP_1050_5e16 = PTR_LOOP_1050_5e16 + -1;
    if (PTR_LOOP_1050_5e16 == 0x0) {
        FreeProcInstance16(CONCAT22(_g_proc_inst_1050_5e18, 0x1538));
        _g_proc_inst_1050_5e18 = 0;
    }
    in_struct_1 = ctx.s_1_1050_389a;
    local_struct_1.field_0x2 = &ctx.PTR_LOOP_1050_1008;
    return;
}

pub fn enable_window_1040_8ea0(param_1: *mut Struct59, param_2: u32, param_3: u32) {
    let enable: bool;
    let h_wnd: HWND16;

    if (param_3._2_2_ == 0xf8) {
        h_wnd = GetDlgItem16(0x17d8, param_1.field_0x6);
        enable = 1;
    } else {
        if (param_3._2_2_ != 0x17d8) {
            win_gui_fn_1040_b54a(param_1, param_2, (param_2 >> 0x10), param_3);
            return;
        }
        SetWindowPos16(6, 0xf6, 0x269, 0, 0, 0, param_1.field_0x6);
        enable = offset;
        GetDlgItem16(0x17d8, param_1.field_0x6);
        h_wnd = 0;
    }
    EnableWindow16(enable, h_wnd);
    return;
}

pub fn draw_1040_8a06(param_1: u32) {
    let mut u_var1: u32;
    let mut i_var2: i32;
    let mut h_dc: u16;
    let dev_ctx_handle: *mut u16;
    let obj_handle: HANDLE16;
    let h_gdi_obj: HPALETTE16;
    let mut u_var3: u16;
    let mut i_var4: i32;
    let count: u16;
    let unaff_ss: HWND16;
    let mut u_var5: u32;
    let CVar6: COLORREF;
    let color: COLORREF;
    let u_stack68: u8;
    let mut local_3e: u16;
    let mut local_3c: u16;
    let mut local_3a: u16;
    let mut local_38: u16;
    let mut local_36: u16;
    let mut local_34: u16;
    let mut local_32: u16;
    let mut local_30: u16;
    let mut local_2e: u16;
    let mut local_2c: u16;
    let mut local_2a: u16;
    let mut local_28: u32;
    let mut local_24: u16;
    let local_22: *mut PAINTSTRUCT16;
    let iStack30: u16;

    count = (param_1 >> 0x10);
    i_var4 = param_1;
    local_24 = BeginPaint16(CONCAT22(unaff_ss, &local_22), (i_var4 + 6));
    u_var5 = (_PTR_LOOP_1050_4230 + 0xe);
    local_28._0_2_ = u_var5;
    local_28._2_2_ = (u_var5 >> 0x10);
    dev_ctx_handle = &local_24;
    realize_palette_1008_4e08(local_28, local_28._2_2_, dev_ctx_handle, unaff_ss);
    u_var5 = pass1_1008_4d72(u_var5);
    u_var3 = (u_var5 >> 0x10);
    i_var2 = u_var5;
    DrawIcon16(
        (i_var4 + 0x8e),
        CONCAT214(
            local_3c,
            CONCAT212(local_3e, CONCAT66(u_stack68, CONCAT24(local_24, 0x14000a))),
        ),
        CONCAT214(
            u_var3,
            CONCAT212(
                i_var2,
                CONCAT210(
                    CONCAT11(2, *(i_var2 + 0x94)),
                    CONCAT28(
                        CONCAT11(*(i_var2 + 0x95), *(i_var2 + 0x96)),
                        CONCAT26(local_34, CONCAT24(local_36, CONCAT22(local_38, local_3a))),
                    ),
                ),
            ),
        ),
        dev_ctx_handle,
    );
    CVar6 = SetBkColor16(0, local_24);
    color = SetTextColor16(0x15388a8c, local_24);
    local_3e = 0;
    obj_handle = GetProp16(s_hfont_1050_5dfa, (i_var4 + 6));
    if (obj_handle != 0) {
        local_3e = SelectObject16(obj_handle, local_24);
    }
    u_var1 = (i_var4 + 0x90);
    local_28._0_2_ = u_var1;
    local_28._2_2_ = (u_var1 >> 0x10);
    DrawText16(
        0x10,
        (i_var4 + 0x9e),
        count,
        CONCAT22(local_28, 0xffff),
        local_28._2_2_,
    );
    if (obj_handle != 0) {
        SelectObject16(local_3e, local_24);
    }
    SetBkColor16(CONCAT22(0x8ae7, CVar6), local_24);
    SetTextColor16(color, local_24);
    h_dc = local_24;
    local_22 = local_24;
    local_24 = offset;
    h_gdi_obj = SelectPalette16(0, offset, h_dc);
    local_22 = offset;
    local_24 = 0x8b23;
    DeleteObject16(h_gdi_obj);
    iStack30 = (i_var4 + 6);
    local_22 = &local_22;
    local_24 = offset;
    EndPaint(local_22, unaff_ss);
    return;
}

pub fn win_fn_1040_8b3c(param_1: u16, param_2: u32, param_3: u32) {
    if ((param_3._2_2_ != 0x0)
        && (param_3._2_2_ == (&ctx.PTR_LOOP_1050_0000 + 1)
            || param_3._2_2_ == &dos_alloc_addr_1050_0002
            || ((&dos_alloc_addr_1050_0002 + 1) < param_3._2_2_ + -2
                && (param_3._2_2_ + -6 < &dos_alloc_addr_1050_0002))))
    {
        PTR_LOOP_1050_5df4 = 0x0;
        PTR_LOOP_1050_5df8 = param_3._2_2_;
        return;
    }
    post_win_msg_1040_7b3c(param_1, param_2, param_3);
    return;
}

pub fn destroy_window_1040_8b7e(param_1: u32) {
    DestroyWindow16((param_1 + 6));
    return;
}

pub fn win_fn_1040_8b92(param_1: u32) {
    let mut bVar1: u8;
    let mut u_var2: u16;
    let mut u_var3: u16;

    u_var3 = (param_1 >> 0x10);
    bVar1 = *(param_1 + 0x98) & 0xf0;
    if ((((bVar1 == 0x30) || (bVar1 == 0x10)) || (bVar1 == 0x10))
        || ((bVar1 == 0x40 || (bVar1 == 0x40)) || (bVar1 == 0x20)))
    {
        u_var2 = LoadIcon16();
        (param_1 + 0x8e) = u_var2;
    }
    return;
}

pub fn create_win_1040_8bea(
    param_1: *mut Struct40,
    param_2: u16,
    menu: u16,
    x_coord: u16,
    y_coord: u16,
    window_name: u32,
) -> HANDLE16 {
    let mut window_1: u16;
    let w_param: HANDLE16;
    let ctx.bx_reg: *mut Struct40;
    let mut u_var1: u16;
    let LVar2: LRESULT;
    let mut local_8: u16;
    let mut style: u32;

    style = 0x50010000;
    if (param_2 != 0) {
        style = 0x50010001;
    }
    u_var1 = (param_1 >> 0x10);
    ctx.bx_reg = param_1;
    if (ctx.bx_reg.field_0x74 != 0) {
        style = style | 0x8000000;
    }
    window_1 = CreateWindow16(
        0,
        ctx.g_h_instance_1050_038c,
        menu,
        ctx.bx_reg.parent,
        0x17,
        0x58,
        y_coord,
        x_coord,
        style,
        window_name,
        s_OPButton_1050_5e00,
    );
    w_param = GetProp16(s_hfont_1050_5e09, ctx.bx_reg.parent);
    if (w_param != 0) {
        LVar2 = SendMessage16(1, w_param, 0x30, window_1);
        w_param = LVar2;
    }
    return w_param;
}

pub fn win_fn_1040_89a4(param_1: *mut u32, param_2: *mut u16) {
    let mut u_var1: u16;
    let ppc_var2: fn();
    let mut u_var3: u16;
    let mut u_var4: u16;
    let ppVar5: *mut pass1_struct_1;
    let mut in_stack_0000fff0: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    process_win_msg_1008_9510(&PTR_LOOP_1050_5df4, &ctx.g_alloc_addr_1050_1050);
    u_var3 = (param_2 + 2);
    unsafe {
        u_var1 = *param_2;
    }
    u_var4 = 0x1010;
    ppVar5 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(in_stack_0000fff0, 2));
    if ((ppVar5 + 0x72) != 0) {
        u_var4 = SUB42(&ctx.PTR_LOOP_1050_1008, 0);
        u_var3 = mci_send_command_1008_5c7c(ctx._g_struct_ptr_1050_02a0, CONCAT22(u_var3, u_var1));
        (param_1 + 0x8c) = u_var3;
    }
    unsafe {
        ppc_var2 = (*param_1 + 0x74);
    }
    ppc_var2(u_var4, param_1);
    return;
}

pub fn win_fn_1040_8718(param_1: *mut Struct20) -> Vec<u8> {
    let pi_var1: *mut i32;
    let mut u_var2: u32;
    let mut i_var3: i32;
    let mut u_var4: u16;
    let mut unaff_ss: u16;
    let ppVar5: *mut pass1_struct_1;
    let mut u_var6: u16;
    let mut menu: u16;
    let u_var7: u8;
    let u_var8: u8;
    let u_var9: u8;
    let u_var10: u8;
    let mut u_var11: i32;
    let mut local_120: u16;
    let mut local_11e: u16;
    let mut local_10c: u16;
    let mut local_10a: u16;
    let mut local_108: u16;
    let mut local_106: u16;
    let mut local_104: u16;
    let mut local_4: u16;

    process_win_msg_1008_9510(&PTR_LOOP_1050_5df4, &ctx.g_alloc_addr_1050_1050);
    i_var3 = param_1;
    u_var11 = (param_1 >> 0x10);
    win_gui_func_1040_78e2(param_1);
    PTR_LOOP_1050_5df6 = (i_var3 + 6);
    if ((i_var3 + 0x94) != 0) {
        copy_string_1000_3d3e(
            (param_1 & 0xff000000 | CONCAT12((param_1 >> 0x10), i_var3 + 0x10)),
            *(i_var3 + 0x94),
        );
    }
    get_sys_metrics_1040_8c66(i_var3, u_var11);
    local_4 = *(i_var3 + 0x98) & 0xf;
    if (local_4 == 1) {
        (i_var3 + 0xae) = ((i_var3 + 0xaa) + -0xc4) / 2;
        load_string_1010_84e0(
            ctx._g_struct_73_1050_14cc,
            (ctx._g_struct_73_1050_14cc >> 0x10),
            0xff,
            CONCAT22(unaff_ss, &local_104),
            0x592,
        );
        u_var2 = (i_var3 + 0xae);
        create_win_1040_8bea(
            (param_1 & 0xffff | u_var11 << 0x10),
            1,
            1,
            u_var2,
            (u_var2 >> 0x10),
            CONCAT22(unaff_ss, &local_104),
        );
        pi_var1 = (i_var3 + 0xae);
        unsafe {
            *pi_var1 = *pi_var1 + 0x6c;
        }
        load_string_1010_84e0(
            ctx._g_struct_73_1050_14cc,
            (ctx._g_struct_73_1050_14cc >> 0x10),
            0xff,
            CONCAT22(unaff_ss, &local_104),
            0x7d8,
        );
        u_var2 = (i_var3 + 0xae);
        u_var7 = u_var2;
        u_var8 = (u_var2 >> 8);
        u_var9 = (u_var2 >> 0x10);
        u_var10 = (u_var2 >> 0x18);
        menu = 2;
    } else {
        if (local_4 != 4) {
            (i_var3 + 0xae) = ((i_var3 + 0xaa) + -0x58) / 2;
            load_string_1010_84e0(
                ctx._g_struct_73_1050_14cc,
                (ctx._g_struct_73_1050_14cc >> 0x10),
                0xff,
                CONCAT22(unaff_ss, &local_104),
                0x592,
            );
            u_var2 = (i_var3 + 0xae);
            u_var7 = u_var2;
            u_var8 = (u_var2 >> 8);
            u_var9 = (u_var2 >> 0x10);
            u_var10 = (u_var2 >> 0x18);
            u_var6 = 1;
            menu = 1;
            // goto LAB_1040_88a5;
        }
        (i_var3 + 0xae) = ((i_var3 + 0xaa) + -0xc4) / 2;
        load_string_1010_84e0(
            ctx._g_struct_73_1050_14cc,
            (ctx._g_struct_73_1050_14cc >> 0x10),
            0xff,
            CONCAT22(unaff_ss, &local_104),
            0x650,
        );
        u_var2 = (i_var3 + 0xae);
        create_win_1040_8bea(
            (param_1 & 0xffff | u_var11 << 0x10),
            1,
            6,
            u_var2,
            (u_var2 >> 0x10),
            CONCAT22(unaff_ss, &local_104),
        );
        pi_var1 = (i_var3 + 0xae);
        unsafe {
            *pi_var1 = *pi_var1 + 0x6c;
        }
        load_string_1010_84e0(
            ctx._g_struct_73_1050_14cc,
            (ctx._g_struct_73_1050_14cc >> 0x10),
            0xff,
            CONCAT22(unaff_ss, &local_104),
            0x651,
        );
        u_var2 = (i_var3 + 0xae);
        u_var7 = u_var2;
        u_var8 = (u_var2 >> 8);
        u_var9 = (u_var2 >> 0x10);
        u_var10 = (u_var2 >> 0x18);
        menu = 7;
    }
    u_var6 = 0;
    // LAB_1040_88a5:
    create_win_1040_8bea(
        (param_1 & 0xffff | u_var11 << 0x10),
        u_var6,
        menu,
        CONCAT11(u_var8, u_var7),
        CONCAT11(u_var10, u_var9),
        CONCAT22(unaff_ss, &local_104),
    );
    ppVar5 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(local_120, 0x48));
    u_var4 = (ppVar5 >> 0x10);
    local_104 = (ppVar5 + 10);
    local_4 = (ppVar5 + 0xc);
    SetWindowPos16(
        0x40,
        (i_var3 + 0xac),
        (i_var3 + 0xaa),
        (local_4 - (i_var3 + 0xac)) / 2,
        (local_104 - (i_var3 + 0xaa)) / 2,
        0,
        (i_var3 + 6),
    );
    PTR_LOOP_1050_5df4 = (&ctx.PTR_LOOP_1050_0000 + 1);
    process_win_msg_1008_9510(&PTR_LOOP_1050_5df4, &ctx.g_alloc_addr_1050_1050);
    destroy_window_1040_8b7e(i_var3, u_var11);
    PTR_LOOP_1050_5df6 = 0x0;
    if ((i_var3 + 0xb2) != 0) {
        ppVar5 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(local_11e, 0x37));
        i_var3 = pass1_1008_ab54(ppVar5);
        if (i_var3 != 0) {
            PostMessage16(0, 0xfc, 0x111, ctx.g_h_window);
        }
    }
    return PTR_LOOP_1050_5df8;
}

pub fn enable_window_1040_86dc(param_1: u32) {
    let HVar1: HWND16;
    let mut u_var2: u16;

    u_var2 = (param_1 >> 0x10);
    HVar1 = GetDlgItem16(1, (param_1 + 6));
    if (HVar1 != 0) {
        EnableWindow16(1, HVar1);
        HVar1 = GetDlgItem16(2, (param_1 + 6));
        if (HVar1 != 0) {
            EnableWindow16(1, HVar1);
        }
    }
    return;
}

pub fn mixed_1040_8520(
    param_1: &mut Struct103,
    param_2: u16,
    param_3: u16,
    param_4: u16,
    param_5: u16,
    param_6: u16,
) -> *mut Struct362 {
    let mut out_buffer: string;
    let paVar1: *mut Struct362;
    let mut u_var2: u16;
    let pu_var3: *mut u16;
    let mut u_var4: i32;
    let in_dx: *mut Struct199;
    let mut unaff_cs: u16;
    let mut u_var5: u16;
    let mut unaff_ss: u16;
    let mut u_var6: u32;
    let in_string_1: String;
    let pcVar7: String;
    let mut uStack54: u16;
    let local_32: *mut Struct362;
    let mut uStack48: u16;
    let uStack46: u8;
    let uStack45: u8;
    let uStack44: u8;
    let uStack43: u8;
    let paStack42: *mut Struct73;
    let mut uStack40: u32;
    let mut local_20: u16;
    let mut local_1e: u16;
    let mut local_1c: u16;
    let mut local_1a: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    uStack40._2_2_ = param_2;
    paStack42 = 0x0;
    uStack40._0_2_ = 0xfc3;
    uStack46 = 1;
    uStack45 = 0;
    uStack44 = 0;
    uStack43 = 0;
    local_32 = param_1;
    paVar1 = local_32;
    uStack48 = (param_1 >> 0x10);
    u_var2 = uStack48;
    uStack54 = 0x853b;
    _local_32 = param_1;
    process_struct_1040_7728(param_1, (&ctx.PTR_LOOP_1050_0000 + 1), 0, 0xfc3, param_2);
    paVar1.field_0x8e = 0;
    paVar1.field_0x98 = param_3;
    paVar1.field_0x9a = 0;
    paVar1.field_0xb2 = 0;
    unsafe {
        *param_1 = 0x8ddc;
    }
    paVar1.field_0x2 = &ctx.PTR_LOOP_1050_1040;
    _local_a = 0;
    _local_6 = 300;
    paVar1.field_0x9e = 0;
    paVar1.field_0xa2 = 300;
    _local_e = CONCAT22(unaff_ss, &param_5);
    local_10 = param_4;
    if (param_4 != 0) {
        _local_e = CONCAT22(unaff_ss, &param_6);
        local_12 = param_5;
        uStack40._2_2_ = param_5;
        paStack42 = ctx._g_struct_73_1050_14cc;
        uStack40._0_2_ = (ctx._g_struct_73_1050_14cc >> 0x10);
        uStack44 = unaff_cs;
        uStack43 = (unaff_cs >> 8);
        unaff_cs = 0x1010;
        uStack46 = 0xa7;
        uStack45 = 0x85;
        u_var6 = load_str_1010_84ac(paStack42, uStack40, param_5);
        in_dx = (u_var6 >> 0x10);
        paVar1.field_0x94 = u_var6;
        paVar1.field_0x96 = in_dx;
        local_10 = local_10 - 1;
    }
    local_16 = 0;
    while (pu_var3 = _local_e, local_10 != 0) {
        _local_e = (_local_e & 0xffff0000 | (local_e + 2));
        unsafe {
            uStack40._2_2_ = *pu_var3;
        }
        paStack42 = ctx._g_struct_73_1050_14cc;
        uStack40._0_2_ = (ctx._g_struct_73_1050_14cc >> 0x10);
        uStack44 = unaff_cs;
        uStack43 = (unaff_cs >> 8);
        uStack46 = 0xd6;
        uStack45 = 0x85;
        local_14 = uStack40._2_2_;
        local_10 = local_10 - 1;
        in_string_1 = load_string_1010_847e(paStack42, uStack40, uStack40._2_2_);
        local_1a = (in_string_1 >> 0x10);
        paStack42 = 0x1010;
        unaff_cs = 0x1000;
        uStack44 = 0xe3;
        uStack43 = 0x85;
        pcVar7 = in_string_1;
        u_var4 = get_string_index_1000_3da4(in_string_1);
        uStack40._2_2_ = (pcVar7 >> 0x10);
        local_1c = pcVar7;
        in_dx = (in_string_1 >> 0x10);
        uStack40._0_2_ = in_string_1;
        local_16 = local_16 + u_var4;
    }
    uStack40._2_2_ = (local_16 + 1);
    u_var5 = 0x1000;
    paStack42 = 0x85fd;
    uStack40._0_2_ = unaff_cs;
    local_10 = local_10 - 1;
    process_struct_1000_179c(uStack40._2_2_, in_dx);
    paVar1.field_0x90 = uStack40._2_2_;
    &paVar1.field_0x92 = in_dx;
    _local_e = CONCAT22(unaff_ss, &param_6);
    local_10 = param_4 - 1;
    if (local_10 != 0) {
        _local_e = CONCAT22(unaff_ss, &stack0x0012);
        local_14 = param_6;
        uStack40._2_2_ = param_6;
        out_buffer = &paVar1.field_0x90;
        paStack42 = out_buffer;
        uStack40._0_2_ = (out_buffer >> 0x10);
        uStack44 = 0xff;
        uStack43 = 3;
        uStack48 = ctx._g_struct_73_1050_14cc;
        uStack46 = (ctx._g_struct_73_1050_14cc >> 0x10);
        uStack45 = (ctx._g_struct_73_1050_14cc >> 0x18);
        local_32 = &ctx.PTR_LOOP_1050_1000;
        u_var5 = 0x1010;
        load_string_1010_84e0(
            uStack48,
            (ctx._g_struct_73_1050_14cc >> 0x10),
            0x3ff,
            out_buffer,
            param_6,
        );
        local_10 = local_10 - 1;
    }
    while (pu_var3 = _local_e, local_10 != 0) {
        _local_e = (_local_e & 0xffff0000 | (local_e + 2));
        unsafe {
            uStack40._2_2_ = *pu_var3;
        }
        paStack42 = ctx._g_struct_73_1050_14cc;
        uStack40._0_2_ = (ctx._g_struct_73_1050_14cc >> 0x10);
        uStack44 = u_var5;
        uStack43 = (u_var5 >> 8);
        uStack46 = 0x5e;
        uStack45 = 0x86;
        local_14 = uStack40._2_2_;
        local_10 = local_10 - 1;
        uStack40 = load_string_1010_847e(paStack42, uStack40, uStack40._2_2_);
        pcVar7 = *&paVar1.field_0x90;
        uStack44 = SUB41(pcVar7, 0);
        uStack43 = (pcVar7 >> 8);
        paStack42 = (pcVar7 >> 0x10);
        uStack46 = 0x10;
        uStack45 = 0x10;
        u_var5 = 0x1000;
        uStack48 = 0x8674;
        _local_20 = uStack40;
        process_string_1000_3cea(pcVar7, uStack40);
    }
    uStack44 = 0x8a;
    uStack43 = 0x86;
    paStack42 = u_var5;
    local_10 = local_10 - 1;
    uStack40 = param_1;
    win_fn_1040_8b92(param_1);
    PTR_LOOP_1050_5df8 = 0x0;
    return paVar1;
}

pub fn move_window_1040_826c(param_1: u32, param_2: u16, param_3: u16) {
    let i_var1: u16;
    let mut unaff_cs: u16;
    let unaff_ss: HWND16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    GetWindowRect16(CONCAT22(&local_e, unaff_cs), unaff_ss);
    if ((param_3 == 0xffff) || (param_2 == 0xffff)) {
        i_var1 = GetSystemMetrics16(0);
        local_4 = (i_var1 - (local_a - local_e)) / 2;
        i_var1 = GetSystemMetrics16(1);
        param_2 = (i_var1 - (local_8 - local_c)) / 2;
    } else {
        local_4 = param_3;
    }
    local_6 = param_2;
    MoveWindow16(
        0,
        local_8 - local_c,
        local_a - local_e,
        param_2,
        local_4,
        (param_1 + 6),
    );
    return;
}

pub fn set_sys_modal_window_1040_81fe(param_1: u32) {
    SetSysModalWindow16((param_1 + 6));
    return;
}

pub fn get_message_1040_81b6(param_1: u32) {
    let b_var1: bool;
    let mut u_var2: u16;
    let unaff_ss: HWND16;

    u_var2 = (param_1 >> 0x10);
    (param_1 + 0x78) = 1;
    while (true) {
        b_var1 = IsWindow16((param_1 + 6));
        if (b_var1 == 0) {
            return;
        }
        b_var1 = GetMessage16(0, 0, 0, &stack0xffec);
        if (b_var1 == 0) {
            break;
        }
        IsDialogMessage16(&stack0xffec, unaff_ss);
    }
    return;
}

pub fn win_fn_1040_800c(param_1: u32) {


    let local_bx_17: *mut Struct6;
    let mut u_var1: u16;
    let mut iStack18: i32;
    let mut local_f: u8;
    let mut iStack16: i32;
    let mut w_command: u16;
    let lp_help_file: String;
    let mut h_wnd: u16;

    mixed_fn_1010_830a(ctx._g_struct_73_1050_14cc, 0x1f8);
    u_var1 = (param_1 >> 0x10);
    local_bx_17 = param_1;
    if (local_bx_17.field_0x8a == 0) {
        h_wnd = local_bx_17.field_0x6;
        iStack16 = 0;
        w_command = 3;
        iStack18 = 0;
    } else {
        h_wnd = local_bx_17.field_0x6;
        w_command = 1;
        iStack18 = local_bx_17.field_0x8a;
        iStack16 = iStack18 >> 0xf;
    }
    _lp_help_file = CONCAT22(ctx.dx_reg, in_ax);
    WinHelp16(
        CONCAT22(iStack16, iStack18),
        w_command,
        _lp_help_file,
        h_wnd,
    );
    return;
}

pub fn track_popup_menu_1040_7f86(param_1: u32, param_2: u16) {
    let mut menu_handle: u16;
    let mut HVar1: u16;
    let local_bx_4: *mut Struct27;
    let mut u_var2: u16;
    let mut unaff_cs: u16;
    let mut window_handle: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    u_var2 = (param_1 >> 0x10);
    local_bx_4 = param_1;
    if ((local_bx_4.menu_name != 0x0) && (local_bx_4.menu_handle_2 == 0)) {
        menu_handle = LoadMenu16(local_bx_4.menu_name, ctx.g_h_instance_1050_038c);
        local_bx_4.menu_handle_2 = menu_handle;
        if (menu_handle == 0) {
            return;
        }
        local_6 = local_bx_4.menu_handle_2;
        unaff_cs = SUB42(offset, 0);
        HVar1 = GetSubMenu16(0, local_6);
        local_bx_4.menu_handle_2 = HVar1;
        if (HVar1 == 0) {
            return;
        }
    }
    local_4 = param_2;
    local_6 = local_bx_4.field_0x6;
    ClientToScreen16(CONCAT22(&local_6, unaff_cs), window_handle);
    TrackPopupMenu16(0x0, 0, local_bx_4.field_0x6, 0, local_4, local_6, 0);
    return;
}

pub fn destroy_win_1040_7b98(param_1: u32) {
    let mut u_var1: u16;

    u_var1 = (param_1 >> 0x10);
    if ((param_1 + 0x74) == 0) {
        DestroyWindow16((param_1 + 6));
    }
    return;
}

pub fn win_gui_func_1040_78e2(param_1: *mut Struct20) {
    let pp_var1: fn();
    let dlg_proc: DLGPROC16;
    let HVar2: HWND16;
    let local_bx_5: *mut Struct32;
    let handle: HANDLE16;
    let mut u_var3: u16;
    let lVar4: u32;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    handle = (param_1 >> 0x10);
    local_bx_5 = param_1;
    if (&local_bx_5.field_0xc == 0) {
        u_var3 = (ctx._PTR_LOOP_1050_1040 >> 0x10);
        dlg_proc = (ctx._PTR_LOOP_1050_1040 + 4);
        HVar2 = (ctx._PTR_LOOP_1050_1040 + 6);
    } else {
        dlg_proc = local_bx_5.field_0xc;
        HVar2 = local_bx_5.field_0xe;
    }
    HVar2 = CreateDialog16(
        dlg_proc,
        HVar2,
        CONCAT22(local_bx_5.field_0xa, local_bx_5.h_instance),
        0,
    );
    local_bx_5.h_window = HVar2;
    GetWindowText16(0x50, param_1 & 0xffff0000 | ZEXT24(local_bx_5 + 1), HVar2);
    lVar4 = GetWindowLong16(-4, local_bx_5.h_window);
    SetWindowLong16(_PTR_LOOP_1050_5bcc, (_PTR_LOOP_1050_5bcc >> 0x10));
    SetProp16(local_bx_5, s_thisLo_1050_5dcd, local_bx_5.h_window);
    SetProp16(handle, s_thisHi_1050_5dd4, local_bx_5.h_window);
    local_a = lVar4;
    SetProp16(local_a, s_procLo_1050_5ddb, local_bx_5.h_window);
    local_8 = (lVar4 >> 0x10);
    SetProp16(local_8, s_procHi_1050_5de2, local_bx_5.h_window);
    pp_var1 = (param_1 + 0x50);
    (**pp_var1)(offset, param_1);
    return;
}

pub fn win_fn_1040_748c(param_1: *mut Struct124, param_2: u16, param_3: u16, param_4: u32) {
    let pp_var1: fn();
    let mut u_var2: u32;
    let unaff_ss: HWND16;
    let mut u_var3: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    match (param_2._2_2_) {
        0xfa => {
            pp_var1 = ((param_1 + 1) + 0x18);
            (**pp_var1)()
        }
        _ => {
            win_gui_fn_1040_b54a(param_1, param_2_00, param_3, param_2);
            return;
        }
        0xfd => {
            if (u16_1050_0ecc == 0) {
                return;
            }
            u16_1050_0ecc = 0;
            // goto LAB_1040_755d;
        }
        0xfe => {
            if (u16_1050_0ecc == 1) {
                return;
            }
            u16_1050_0ecc = 1;
            // goto LAB_1040_755d;
        }
        0xff => {
            if (u16_1050_0ecc == 2) {
                return;
            }
            u16_1050_0ecc = 2;
            // LAB_1040_755d:
            u_var2 = (param_1 + 1);
            pp_var1 = ((param_1 + 1) + 0x10);
            (**pp_var1)(0x40, u_var2, (u_var2 >> 0x10));
            u_var2 = (param_1 + 1);
            pass1_1010_2ee2(u_var2, (u_var2 >> 0x10));
            PostMessage16(0, 0x10a, 0x111, &param_1.field_0x6)
        }
        0x107 => {
            u_var3 = 0;
            // goto LAB_1040_75ba;
        }
        0x108 => {
            u_var3 = 1;
            // LAB_1040_75ba:
            u_var2 = (param_1 + 1);
            destroy_win_1010_3202(u_var2, (u_var2 >> 0x10), u_var3)
        }
        0x10a => {
            GetClientRect16(CONCAT22(unaff_ss, &local_a), &param_1.field_0x6);
            u_var2 = (param_1 + 1);
            local_8 = local_8 + 3;
            local_a = (u_var2 + 0x1a) - 9;
            local_6 = local_6 - 3;
            local_4 = local_4 - 3;
            InvalidateRect16(1, &local_a, unaff_ss);
            destroy_win_1010_2fa0((param_1 + 1));
            pass1_1010_32c0((param_1 + 1), 0);
            u_var2 = (param_1 + 1);
            pass1_1010_2ee2(u_var2, (u_var2 >> 0x10))
        }
        0x10c => {
            DestroyWindow16(&param_1.field_0x6);
        }
    }
    return;
}
