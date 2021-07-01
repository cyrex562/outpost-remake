use std::intrinsics::offset;

use proc::free_proc_inst_1040_911e;

use crate::{defines::{
    AppContext, ATOM, COLORREF, HANDLE16, HBRUSH16, HCURSOR16, HDC16, HFILE16, HGDIOBJ16,
    HGLOBAL16, HICON16, HINSTANCE16, HMENU16, HPALETTE16, HPEN16, HTASK16, HWND16, LOGPALETTE,
    LPARAM, LRESULT, PAINTSTRUCT16, POINT16, RECT16, SEGPTR, WPARAM16,
}, func_ptr_funcs::call_fn_ptr_1000_2594, sys2, util::SUB42, winapi};
use crate::app_context::AppContext;
use crate::bool_funcs::check_and_set_global_1000_1fea;
use crate::cleanup::{ret_1040_78de, win_cleanup_1010_305a, win_cleanup_func_1040_782c, window_msg_func_1010_7300};
use crate::constants::PF_ALPHA_BYTE_INSTRUCTIONS;
use crate::err_ops::error_check_1000_17ce;
use crate::exit::{exit_1000_25cc, exit_1000_2950, return_1000_39e1};
use crate::func_ptr_funcs::{call_fn_ptr_1000_24cd, call_fn_ptr_1000_256b};
use crate::mem_funcs::alloc_mem::alloc_mem_1000_07fc;
use crate::mem_funcs::free_mem::free_mem_1000_1b68;
use crate::mem_funcs::mem_ops_1::{get_fn_ptr_at_address, get_type_at_address, StructuredData};
use crate::other_funcs::{exported_stub_1000_29dc, get_private_profile_str_1010_6132, mixed_fn_1010_830a, switch_stmt_1008_aaa8, switch_stmt_1008_d818, write_private_profile_str_1010_622a};
use crate::pass::pass12_funcs::{pass1_1008_b38c, pass1_1008_b47a, pass1_1008_b4a0, pass1_1008_b61a, pass1_1008_b63a, pass1_1008_b820, pass1_1008_c6ae, pass1_1008_c6fa};
use crate::pass::pass13_funcs::{pass1_1008_932a, pass1_1008_941a, pass1_1008_b200, pass1_1008_b366};
use crate::pass::pass14_funcs::{pass1_1008_3e94, pass1_1008_5784, pass1_1008_57c4, pass1_1008_5b12, pass1_fn_1008_60e8, pass1_fn_1008_612e};
use crate::pass::pass15_funcs::{pass1_1020_ba7e, pass1_1020_bb16, pass1_1020_c42e, ret_one_1020_c3ae, switch_statement_1020_c3b4};
use crate::pass::pass16_funcs::{pass1_1028_62c8, pass1_1028_6408, pass1_1028_740c, pass1_1028_75bc, pass1_1028_78b8, pass1_1028_7c4e, pass1_1028_7dfc, pass1_1028_7fb6, pass1_1028_8d9e, pass1_1028_8dec, post_msg_1028_76da};
use crate::pass::pass17_funcs::{pass1_1030_1d58, pass1_1030_2242, pass1_1030_3694, pass1_1030_375a, pass1_1030_38b8, pass1_1030_38f2, pass1_1030_5b00, pass1_1030_6a2c, pass1_1030_6c1a, pass1_1030_6c4c, pass1_1030_73a8, pass1_1030_7c50, pass1_1030_7ddc, pass1_1030_835a};
use crate::pass::pass19_funcs::{pass1_1038_df5c, pass1_1040_a5d0};
use crate::pass::pass20_funcs::{pass1_1010_ac92, pass1_1010_ae12, pass1_1010_ae92, pass1_1018_0ad4};
use crate::pass::pass3_funcs::{pass1_1028_45e2, pass1_1028_5ca0};
use crate::pass::pass4_funcs::{pass1_1028_bc90, pass1_1028_d01a, pass1_1028_d2b0, pass1_1028_dc52, pass1_1028_e1ec, pass1_1028_e4ec};
use crate::pass::pass5_funcs::{pass1_1030_bcae, pass1_1030_bd74};
use crate::pass::pass6_funcs::{pass1_1038_349e, pass1_1038_387e, pass1_1038_3ba0, pass1_1038_4d6e, pass1_1038_4e78, pass1_1038_4f54};
use crate::pass::pass7_funcs::{pass1_1018_1c9a, pass1_1018_1e78, pass1_1018_34a6, pass1_1018_3a7a};
use crate::pass::pass8_funcs::{pass1_1008_eb74, pass1_1010_043a, pass1_1010_089e, pass1_1010_1d80, pass1_1010_1ea6, pass1_1010_375e, pass1_1010_3770, pass1_1010_4674, pass1_1010_4788, pass1_1010_4df0, pass1_1010_5120, pass1_1010_519a, pass1_1010_52fc, pass1_1010_531c, pass1_1010_7818, pass1_1010_7b8c};
use crate::pass::pass9_funcs::pass1_1008_df4a;
use crate::pass::pass_funcs::{pass1_1000_3d7a, pass1_1000_4906, pass1_fn_1000_29af, pass1_fn_1000_29b5, pass1_fn_1000_3bac, pass1_fn_1000_3e2c, pass1_fn_1000_462e, pass1_fn_1000_47a4, pass1_fn_1000_5008};
use crate::sound_funcs::{mci_send_cmd_1008_5c5c, mci_send_command_1008_5c7c, mci_send_command_1008_5c9e};
use crate::string_ops::{load_string_1010_847e, load_string_1010_84e0, pass1_1028_767e, str_fn_1010_5286, str_fn_1010_6034};
use crate::string_ops::load::load_string_1008_b1f0;
use crate::string_ops::misc::{copy_string_1000_3d3e, get_string_index_1000_3da4, process_string_1000_3cea, process_string_1000_475e, process_string_1000_55b1, string_fn_1000_3f9c};
use crate::struct_ops::{process_struct_1010_20ba, struct_ops_2};
use crate::struct_ops::struct_ops_2::{init_struct_1000_1902, pass1_1038_df86, process_struct_1000_179c, process_struct_1000_2ce8, process_struct_1008_4772, process_struct_1008_574a, process_struct_1008_e586, process_struct_1010_1d48, process_struct_1040_7728, set_struct_1008_0000};
use crate::structs::prog_structs_12::Struct806;
use crate::structs::prog_structs_15::{Struct1156, Struct921};
use crate::structs::prog_structs_16::Struct493;
use crate::structs::prog_structs_17::Struct1115;
use crate::structs::prog_structs_19::Struct1162;
use crate::structs::prog_structs_2::{Struct131, Struct199, Struct318, Struct7};
use crate::structs::prog_structs_21::Struct74;
use crate::structs::prog_structs_23::{Struct1090, Struct1157, Struct1160, Struct1163, Struct1164, Struct1165, Struct1166, WinStruct42};
use crate::structs::prog_structs_24::{Struct, Struct103, Struct2111, Struct369};
use crate::structs::prog_structs_25::{Struct152, Struct64, Struct65, Struct77};
use crate::structs::prog_structs_26::{Struct1158, Struct1159, Struct824};
use crate::structs::prog_structs_27::{Struct361, Struct816};
use crate::structs::prog_structs_28::Struct300;
use crate::structs::prog_structs_29::{Struct375, Struct807};
use crate::structs::prog_structs_30::Struct359;
use crate::structs::prog_structs_31::{Struct130, Struct348, Struct370};
use crate::structs::prog_structs_7::{Struct1161, Struct376, Struct44};
use crate::structs::prog_structs_8::Struct60;
use crate::structs::prog_structs_9::Struct637;
use crate::sys_structs::{LOGPALETTE, PAINTSTRUCT16, POINT16, RECT16, WNDCLASS16};
use crate::typedefs::{ATOM, COLORREF, HANDLE16, HBRUSH16, HCURSOR16, HDC16, HFILE16, HGDIOBJ16, HGLOBAL16, HICON16, HINSTANCE16, HMENU16, HPALETTE16, HPEN16, HTASK16, HWND16, LPARAM, LRESULT, SEGPTR, WPARAM16};
use crate::ui_ops::cursor::set_cursor_1038_bc30;
use crate::ui_ops::dialog::{send_dialog_item_msg_1038_844a, send_dlg_item_msg_1038_8164};
use crate::ui_ops::menu::destroy_menu_func_1020_795c;
use crate::ui_ops::misc::{mixed_1040_8520, pass1_1038_af40, pass1_1038_e03e, win_fn_1010_71d6, win_fn_1020_1294, win_fn_1038_8f74, win_fn_1040_8b92, win_gui_fn_1010_79aa};
use crate::ui_ops::msg_box::msg_box_1040_64ca;
use crate::ui_ops::window::destroy_win_1040_7b98;
use crate::ui_ops::window::win_fn_1020_7270;
use crate::util::{CARRY1, CARRY2, CONCAT11, CONCAT12, CONCAT13, CONCAT22, CONCAT31, POPCOUNT, SBORROW1, SBORROW2, SUB21, SUB41, ZEXT24};
use crate::winapi::{CreateWindowEx16, GetClassInfo16, GetDOSEnviornment16, RegisterClass16, WritePrivateProfileString16};

pub mod dos_ops;
pub mod metrics;
pub mod win_msg;
pub mod private_profile_str;
pub mod win;
pub mod proc;
pub mod rsrc;

pub unsafe fn get_module_file_name_1000_262c(ctx: &mut AppContext, param_1: &mut String) {
    let pc_var1: String;
    let mut c_var2: char;
    let pu_var3: Vec<u8>;
    let mut i_var4: i32;
    let mut u_var5: u16;
    let mut u_var6: i32;
    let mut u_var7: i32;
    let mut ppc_var8: Vec<String>;
    let pc_var9: String;
    let pc_var10: String;
    let mut pc_var11: String;
    let mut u_var12: u16;
    // let unaff_ss: String;
    let mut u_var13: u32;
    let mut u_var14: u32;
    let mut in_stack_00000000: u16 = 0;
    let mut u_stack12: u16;
    let mut i_stack8: i32;
    let mut pc_stack6: String;
    let mut apu_stack4: [HINSTANCE16; 2] = [0;2];

    ctx.PTR_LOOP_1050_5fd4 = param_1.clone();
    *param_1 = ctx.s_fem15_wav_1050_263a[3..];
    ctx.PTR_LOOP_1050_5fd2 = in_stack_00000000;
    let u_var13 = exit_1000_2950(ctx, None);
    apu_stack4[1] = ctx.g_h_instance;
    let i_stack8 = 0x104;
    let u_stack12 = ctx.s_fem17_wav_1050_264e[7..];
    ctx.PTR_LOOP_1050_5fc4 = u_var13.clone();
    let u_var14 = u_var13.clone();
    let mut i_var4 = winapi::GetModuleFileName16(&ctx.g_h_instance, u_stack12, u_stack12.len());
    // u_var14 = (u_var14 >> 0x10);
    ctx.PTR_LOOP_1050_5fc2 = u_var14.clone();
    //// aram_1 = (u_var13  >> 0x10);
    let mut pc_stack6 = u_var13;
    *(i_stack8 + i_var4) = 0;
    i_var4 = 1;
    ctx.PTR_LOOP_1050_5fb8 = (&ctx.PTR_LOOP_1050_0000 + 1);
    pc_var9 = (ctx.s_New_failed_in_Op__Op__DialogHand_1050_0073 + 0xe);
    // LAB_1000_266c:
    while {
        while {
            pc_var1 = pc_var9;
            pc_var9 = String::from(pc_var9[1..].clone());
            c_var2 = pc_var1[0];
            Var2 == ' '
        } {}
        c_var2 == '\t'
    } {}
    if (c_var2 != '\r') && (c_var2 != '\0') {
        ctx.PTR_LOOP_1050_5fb8 = ctx.PTR_LOOP_1050_5fb8 + 1;
        loop {
            pc_var9 = pc_var9  - 1;
            // LAB_1000_267f:
            pc_var1 = pc_var9;
            pc_var9 = pc_var9[1..].clone();
            c_var2 = pc_var1[0];
            if (c_var2 == ' ') || (c_var2 == '\t') {}
            // goto LAB_1000_266c;
            if (c_var2 == '\r') || (c_var2 == '\0') {
                break;
            }
            if c_var2 == '\"' {
                // LAB_1000_26b8:
                while {
                    loop {
                        loop {
                            pc_var1 = pc_var9;
                            pc_var9 = pc_var9[1..].clone();
                            c_var2 = pc_var1[0];
                            if (c_var2 == '\r') || (c_var2 == '\0') {}
                            /* goto LAB_1000_26e8; */
                            if c_var2 == '\"' {}
                            // goto LAB_1000_267f;
                            if c_var2 == '\\' {
                                break;
                            }
                            i_var4 = i_var4 + 1;
                        }
                        u_var6 = 0;
                        while {
                            pc_var11 = pc_var9;
                            u_var6 = u_var6 + 1;
                            pc_var9 = pc_var11[1..].clone();
                            c_var2 = pc_var11[0];
                            c_var2 == '\\'
                        } {}
                        if c_var2 == '\"' {
                            break;
                        }
                        i_var4 = i_var4 + u_var6;
                        pc_var9 = pc_var11;
                    }
                    i_var4 = i_var4 + (u_var6 >> 1) + ((u_var6 & 1) != 0);
                    (u_var6 & 1) != 0
                } {}
                // goto LAB_1000_267f;
            }
            if c_var2 != '\\' {
                i_var4 = i_var4 + 1;
                // goto LAB_1000_267f;
            }
            u_var6 = 0;
            while {
                u_var6 = u_var6 + 1;
                pc_var1 = pc_var9;
                pc_var9 = pc_var9[1..].clone();
                c_var2 = pc_var1[0];
                c_var2 == '\\'
            } {}
            if c_var2 == '\"' {
                i_var4 = i_var4 + (u_var6 >> 1) + ((u_var6 & 1) != 0);
                if (u_var6 & 1) == 0 {}
                // goto LAB_1000_26b8;
                // goto LAB_1000_267f;
            }
            i_var4 = i_var4 + u_var6;
        }
    }
    // LAB_1000_26e8:
    pc_stack6 = ctx.g_alloc_addr_1050_1050.clone();
    i_var4 = -((ctx.PTR_LOOP_1050_5fb8 + (ctx.PTR_LOOP_1050_5fb8 + 1) * 4 + i_var4 + 1) & 0xfffe);
    ctx.PTR_LOOP_1050_5fba = (&pc_stack6 + i_var4 + 2);
    pc_var11 = (&pc_stack6 + (ctx.PTR_LOOP_1050_5fb8 + 1) * 4 + i_var4 + 2);
    ctx.PTR_LOOP_1050_5fbc = unaff_ss;
    *(&pc_stack6 + i_var4) = unaff_ss;
    pu_var3 = ctx.PTR_LOOP_1050_5fc4;
    u_var12 = *(&pc_stack6 + i_var4);
    (&pc_stack6 + i_var4 + 2) = ctx.PTR_LOOP_1050_5fc2;
    (&pc_stack6 + i_var4 + 4) = pu_var3;
    ppc_var8 = (&stack0x0000 + i_var4);
    *(&pc_stack6 + i_var4) = (&pc_stack6 + i_var4 + 2);

    // TODO
    //(&i_stack8 + i_var4) = offset;
    (&stack0xfff6 + i_var4) = (ctx.s_fem37_wav_1050_2716 + 9);
    u_var5 = exported_stub_1000_29dc();
    u_var5 = ctx.PTR_LOOP_1050_5f7e;
    pc_var9 = (ctx.s_New_failed_in_Op__Op__DialogHand_1050_0073 + 0xe);
    // LAB_1000_272e:
    while {
        while {
            pc_var1 = pc_var9;
            pc_var9 = pc_var9[1..].clone();
            c_var2 = pc_var1[0];
            c_var2 == ' '
        } {}
        c_var2 == '\t'
    } {}
    if (c_var2 == '\r') || (c_var2 == '\0') {
        // LAB_1000_27c1:
        *(&pc_stack6 + i_var4) = offset;
        (&i_stack8 + i_var4) = (ctx.s_fem54_wav_1050_27c0 + 5);
        u_var5 = exported_stub_1000_29dc();
        ppc_var8[0].clear();
        ppc_var8[1].clear();
        // WARNING: Could not recover jumptable at 0x100027d2. Too many branches
        // WARNING: Treating indirect jump as call
        (*&ctx.PTR_LOOP_1050_5fd2)();
        ctx._PTR_LOOP_1050_5fc2 = CONCAT22(ctx.PTR_LOOP_1050_5fc4, ctx.PTR_LOOP_1050_5fc2);
        return;
    }
    ppc_var8[0] = pc_var11;
    // TODO: assign string from correct location on stack
    //ppc_var8[1] = ctx.stack_seg_reg;
    // TODO: update/use index into list of strings differently
    //ppc_var8 = ppc_var8 + 2;
    loop {
        // TODO:
        // pc_var9 = pc_var9 + -1;
        // LAB_1000_274f:
        pc_var1 = pc_var9.clone();
        pc_var9 = pc_var9[1..].clone();
        c_var2 = pc_var1[0];
        if (c_var2 == ' ') || (c_var2 == '\t') {
            pc_var1 = pc_var11.clone();
            pc_var11 = pc_var11[1..].clone();
            pc_var1[0] = '\0';
            // goto LAB_1000_272e;
        }
        if (c_var2 == '\r') || (c_var2 == '\0') {
            // LAB_1000_27be:
            // pc_var11[0] = '\0';
            // goto LAB_1000_27c1;
        }
        pc_var10 = pc_var9;
        if c_var2 == '\"' {
            // LAB_1000_278b:
            loop {
                pc_var9 = pc_var10[1..].clone();
                c_var2 = pc_var10[0];
                if (c_var2 == '\r') || (c_var2 == '\0') {}
                // goto LAB_1000_27be;
                if c_var2 == '\"' {
                    break;
                }
                if c_var2 == '\\' {
                    u_var6 = 0;
                    while {
                        pc_var10 = pc_var9;
                        u_var6 = u_var6 + 1;
                        pc_var9 = pc_var10[1..].clone();
                        c_var2 = pc_var10[0];
                        c_var2 == '\\'
                    } {}
                    if c_var2 == '\"' {
                        u_var7 = u_var6 >> 1;
                        while u_var7 != 0 {
                            u_var7 = u_var7 - 1;
                            pc_var1 = pc_var11.clone();
                            pc_var11 = pc_var11[1..].clone();
                            pc_var1[0] = '\\';
                        }
                        {}
                        if (u_var6 & 1) == 0 {
                            break;
                        }
                        pc_var1 = pc_var11.clone();
                        pc_var11 = pc_var11[1..].clone();
                        pc_var1[0] = '\"';
                        pc_var10 = pc_var9;
                    } else {
                        while u_var6 != 0 {
                            u_var6 = u_var6 - 1;
                            pc_var1 = pc_var11.clone();
                            pc_var11 = pc_var11[1..].clone();
                            pc_var1[0] = '\\';
                        }
                        {}
                    }
                } else {
                    pc_var1 = pc_var11.clone();
                    pc_var11 = pc_var11[1..].clone();
                    pc_var1[0] = c_var2;
                    pc_var10 = pc_var9;
                }
            }
            // goto LAB_1000_274f;
        }
        if c_var2 != '\\' {
            pc_var1 = pc_var11.clone();
            pc_var11 = pc_var11[1..].clone();
            pc_var1[0] = c_var2;
            // goto LAB_1000_274f;
        }
        u_var6 = 0;
        while {
            u_var6 = u_var6 + 1;
            pc_var1 = pc_var9.clone();
            pc_var9 = pc_var9[1..].clone();
            c_var2 = pc_var1[0];
            c_var2 == '\\'
        } {}
        if c_var2 == '\"' {
            u_var7 = u_var6 >> 1;
            while u_var7 != 0 {
                u_var7 = u_var7 - 1;
                pc_var1 = pc_var11.clone();
                pc_var11 = pc_var11[1..].clone();
                pc_var1[0] = '\\';
            }
            pc_var10 = pc_var9;
            if (u_var6 & 1) == 0 {}
            // goto LAB_1000_278b;
            pc_var1 = pc_var11.clone();
            pc_var11 = pc_var11[1..].clone();
            pc_var1[0] = '\"';
            // goto LAB_1000_274f;
        }
        while u_var6 != 0 {
            u_var6 = u_var6 - 1;
            pc_var1 = pc_var11.clone();
            pc_var11 = pc_var11[1..].clone();
            pc_var1[0] = '\\';
        }
    }
}


pub fn set_global_uint_1000_4d0c(ctx: &mut AppContext, param_1: Option<u32>) {
    if param_1.is_some() { ctx.UINT_1050_61e8 = param_1; }
    ctx.PTR_LOOP_1050_61ea = 0;
}

pub fn reg_class_1008_96d2(ctx: &mut AppContext, param_1: &mut Struct65, param_2: u16) {
    let b_var1: bool;
    let AVar2: ATOM;
    let mut unaff_ss: u16;
    let mut local_1c: u16;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_16: u32;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    local_6 = param_1 + 0x5b;
    // b_var1 = GetClassInfo16(&local_1c, CONCAT22(local_6, unaff_ss), param_1);
    b_var1 = GetClassInfo16(param_1, local_6, &local_1c);
    if b_var1 == 0 {
        local_1c = (param_1 + 200);
        local_1a = 0x5632;
        local_18 = ctx.PTR_LOOP_1050_1008;
        local_16 = 0x40000;
        local_12 = ctx.g_h_instance_1050_038c;
        local_10 = (param_1 + 0xc2);
        local_e = (param_1 + 0xc4);
        local_c = (param_1 + 0xc6);
        local_a = 0;
        local_4 = param_1;
        AVar2 = RegisterClass16(&local_1c);
        // TODO
        // if AVar2 == 0 {
        //     call_fn_ptr_1000_24cd(0);
        // }
    }
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn pass1_1008_9c16(param_1: u16, param_2: u32, param_3: u32) {
    proc::def_wnd_proc_func_1008_9ce6(
        CONCAT22(param_2, param_1),
        CONCAT22(param_3, (param_2 >> 0x10)),
        CONCAT22(0x85, (param_3 >> 0x10)),
    );
    return;
}

pub fn print_fn_1008_97c8(param_1: StructuredData) {
    let mut u_var1: u16;
    let mut paint_struct_1: PAINTSTRUCT16 = PAINTSTRUCT16::new();
    let wnd_handle = param_1.buffer[8..10];
    winapi::BeginPaint16(wnd_handle, paint_struct_1);
    paint_struct_1.hdc = (param_1 + 8);
    EndPaint(&paint_struct_1, unaff_ss);
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

// WARNING: Variable defined which should be unmapped: local_1e
pub fn reg_class_1008_818c(param_1: u32) {
    let b_var1: bool;
    let AVar2: ATOM;
    let mut unaff_ss: u16;
    let mut local_1c: u16;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_16: u32;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    local_6 = param_1 + 4;
    b_var1 = GetClassInfo16(&local_1c, CONCAT22(local_6, unaff_ss), param_1);
    if (b_var1 == 0) {
        local_1c = (param_1 + 0x54);
        local_1a = 0x84f2;
        local_18 = &ctx.PTR_LOOP_1050_1008;
        local_16 = 0x40000;
        local_12 = ctx.g_h_instance_1050_038c;
        local_10 = 0;
        local_e = (param_1 + 0x58);
        local_c = (param_1 + 0x56);
        local_a = 0;
        local_4 = param_1;
        AVar2 = RegisterClass16(&local_1c);
        // if AVar2 == 0 {
        //     call_fn_ptr_1000_24cd(0);
        // }
    }
    return;
}

pub fn reg_class_1040_98c0(ctx: &mut AppContext, param_1: StructuredData) {
    let b_var1: bool;
    let AVar2: ATOM;
    let mut unaff_ss: u16;
    let mut local_1c: WNDCLASS16;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_16: u32;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    local_6 = param_1 + 4;
    b_var1 = GetClassInfo16(&local_1c, local_6, param_1);
    if b_var1 == 0 {
        local_1c = (param_1 + 0x54);
        local_1a = 0x9cde;
        local_18 = &ctx.PTR_LOOP_1050_1040;
        local_16 = 0x40000;
        local_12 = ctx.g_h_instance_1050_038c;
        local_10 = 0;
        local_e = (param_1 + 0x58);
        local_c = (param_1 + 0x56);
        local_a = 0;
        local_4 = param_1;
        AVar2 = RegisterClass16(&local_1c);
        if AVar2 == 0 {
            call_fn_ptr_1000_24cd(ctx, None);
        }
    }
    return;
}

pub fn get_prop_1040_9566(param_1: &mut i32) {
    let mut u_var1: u16;
    let mut i_var2: i32;
    let ppc_var3: fn();
    let h_var4: HANDLE16;
    let HVar5: HANDLE16;
    let mut i32_var6: i32;
    let mut u_var7: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

  // u_var7 = (param_1  >> 0x10);
    i32_var6 = param_1;
    if (unsafe { *param_1 == 4 }) {
        u_var1 = (i32_var6 + 0xc);
        h_var4 = winapi::GetProp16(s_thisHi_1050_5e6f, (i32_var6 + 10));
        HVar5 = winapi::GetProp16(s_thisLo_1050_5e68, (i32_var6 + 10));
        if ((h_var4 | HVar5) != 0) {
            i_var2 = (i32_var6 + 6);
            if (i_var2 == 1) {
                _local_c = CONCAT22(h_var4, u_var1);
                ppc_var3 = (*_local_c + 0xc);
                (**ppc_var3)(offset, u_var1, h_var4, (i32_var6 + 8));
                return;
            }
            if (i_var2 == 2) {
                _local_c = CONCAT22(h_var4, u_var1);
                ppc_var3 = (*_local_c + 0x10);
                (**ppc_var3)(offset, u_var1, h_var4, (i32_var6 + 8));
                return;
            }
            if (i_var2 == 4) {
                _local_c = CONCAT22(h_var4, u_var1);
                ppc_var3 = (*_local_c + 0x18);
                (**ppc_var3)(offset, u_var1, h_var4, *(i32_var6 + 8) & 0x10);
                return;
            }
        }
    }
    return;
}

pub fn pass1_1040_8f98(param_1: &mut Struct7) {
    let pu8_var1: Vec<u8>;
    let mut b_var2: u8;
    let mut cVar3: u8;
    let mut u_var4: i32;
    let mut u_var5: u16;
    let mut in_cx: u16;
    let mut bVar6: u8;
    let mut u_var7: i32;
    let mut bVar10: u8;
    let mut i_var9: i32;
    let pu_var11: u16;
    let mut u_var12: u16;
    let mut bVar13: bool;
    let mut b_var14: bool;
    let mut uStack0008: u16;
    let mut uStack000a: u32;
    let mut uStack000e: u32;
    let mut uStack0012: u32;
    let mut uStack0016: u16;
    let mut uStack0024: u16;
    let mut uStack0026: u16;
    let mut uStack0028: u16;
    let mut uStack002a: u16;
    let mut uStack0034: u16;
    let mut uStack0036: u16;
    let mut uStack0038: u16;
    let mut uStack003a: u16;
    let mut uStack003c: u16;
    let pu_var15: u16;
    let lVar16: u32;
    let mut in_stack_0000bf2a: u16;
    let mut in_stack_0000bf2c: u16;
    let local_4e: u8;
    let puStack34: Vec<u8>;
    let mut bVar8: u8;

    puStack34 = &stack0xfffe;
    pu_var11 = &stack0xfffe;
    cVar3 = '\x0f';
    while {
        unaff_bp = unaff_bp + -1;
        pu_var11 = pu_var11 + -1;
        unsafe {
            *pu_var11 = *unaff_bp;
        }
        cVar3 = cVar3 + -1;
        '\0' < cVar3
    } {}
    bVar10 = (in_bx >> 8);
    unaff_si[in_bx] = bVar10;
    bVar8 = (ctx.dx_reg + 1 >> 8);
    b_var2 = bVar8 + bVar10;
    bVar13 = CARRY1(bVar8, bVar10) || CARRY1(b_var2, in_CF);
    u_var4 = ctx.dx_reg + 1 & 0xff;
    u_var7 = u_var4 | (b_var2 + in_CF) << 8;
    lVar16 = CONCAT22(in_cx, u_var7);
    pu_var15 = CONCAT22(&stack0xbf2a, &stack0xfffe);
    pu8_var1 = unaff_si + in_bx;
    bVar6 = u_var4;
    unsafe {
        b_var2 = *pu8_var1;
        bVar8 = *pu8_var1 - bVar6;
        b_var14 = *pu8_var1 < bVar6 || bVar8 < bVar13;
        *pu8_var1 = bVar8 - bVar13;
        let pb_var1_val = unsafe { *pu8_var1 };
        if (pb_var1_val != 0
            && (SBORROW1(b_var2, bVar6) != SBORROW1(bVar8, bVar13)) == (pb_var1_val < '\0'))
        {
            pu8_var1 = unaff_si;
            bVar13 = CARRY1(*pu8_var1, bVar10) || CARRY1(*pu8_var1 + bVar10, b_var14);
            *pu8_var1 = *pu8_var1 + bVar10 + b_var14;
            b_var2 = local_4e + in_bx;
            b_var14 = CARRY1(local_4e, in_bx) || CARRY1(b_var2, bVar13);
            local_4e = b_var2 + bVar13;
            pu8_var1 = unaff_si + -0x4f;
            b_var2 = *pu8_var1;
            bVar8 = *pu8_var1;
            *pu8_var1 = bVar8 + bVar10 + b_var14;
            pu8_var1 = unaff_si + -0x78;
            *pu8_var1 =
                *pu8_var1 + in_cx + (CARRY1(b_var2, bVar10) || CARRY1(bVar8 + bVar10, b_var14));
            *pu_var15 = ctx.s_1_1050_389a;
            uStack0008 = 0;
            uStack000a = 0;
            uStack000e = 0;
            uStack0012 = 0;
            uStack0016 = 0;
            uStack0034 = 5;
            u_var5 = 0;
            uStack0036 = 0;
            uStack0038 = 0;
            uStack003a = 2;
            uStack003c = 0;
            *pu_var15 = 0x9800;
            uStack0026 = 5;
            uStack0024 = 5;
            uStack002a = 0;
            uStack0028 = 0;
            puStack34 = &stack0xfffe;
            if ((in_stack_0000bf2c != 0) && (puStack34 = &stack0xfffe, in_stack_0000bf2a != 0)) {
                uStack0036 = 1;
                puStack34 = &stack0xfffe;
                mixed_fn_1010_830a(ctx.g_struct_73_1050_14cc, in_stack_0000bf2c);
              // u_var12 = (pu_var15  >> 0x10);
                (pu_var15 + 8) = u_var5;
                (pu_var15 + 10) = ctx.dx_reg;
                unaff_cs = 0x1010;
                mixed_fn_1010_830a(ctx.g_struct_73_1050_14cc, in_stack_0000bf2a);
              // u_var12 = (pu_var15  >> 0x10);
                i_var9 = pu_var15;
                (i_var9 + 0xc) = u_var5;
                (i_var9 + 0xe) = ctx.dx_reg;
                if (in_ax == 0) {
                    (i_var9 + 0x10) = 0;
                    u_var7 = ctx.dx_reg;
                } else {
                    unaff_cs = 0x1010;
                    mixed_fn_1010_830a(ctx.g_struct_73_1050_14cc, in_ax);
                  // u_var12 = (pu_var15  >> 0x10);
                    (pu_var15 + 0x10) = u_var5;
                    (pu_var15 + 0x12) = ctx.dx_reg;
                    u_var7 = ctx.dx_reg;
                }
            }
          // u_var12 = (pu_var15  >> 0x10);
            local_bx_272 = pu_var15;
            u_var5 = local_bx_272.field_0x36;
            local_bx_272.field_0x30 = u_var5;
            local_bx_272.field_0x2e = u_var5;
            local_bx_272.field_0x32 = 0;
            if (lVar16 != 0) {
                unaff_cs = &ctx.PTR_LOOP_1050_1008;
                pass1_fn_1008_60e8();
              // u_var12 = (pu_var15  >> 0x10);
                (pu_var15 + 4) = u_var5;
                (pu_var15 + 6) = u_var7;
            }
          // u_var5 = (pu_var15  >> 0x10);
            i_var9 = pu_var15;
            (i_var9 + 0x22) = 0;
            (i_var9 + 0x1e) = 0;
            (i_var9 + 0x20) = 0;
            if ctx._g_proc_inst_1050_5e18 == 0 {
                let hinst: HINSTANCE16 = CONCAT22(0x9684, ctx.g_h_instance_1050_038c) as HINSTANCE16;
                ctx._g_proc_inst_1050_5e18 =
                    winapi::MakeProcInstance16(ctx.code_seg_reg, hinst);
            }
            ctx.PTR_LOOP_1050_5e16 = ctx.PTR_LOOP_1050_5e16 + 1;
            return;
        }
        if *pu8_var1 != 0 {
            error_check_1000_17ce(ctx, param_1);
        }
    }
    return;
}

pub unsafe fn process_struct_1040_8478(
    param_1: &mut u32,
    param_2: u16,
    param_3: u32,
    param_4: u32,
    param_5: u16,
) -> &mut u16 {
    let extraout_var: u32;
    let mut u_var1: u32;
    let mut u_var2: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    process_struct_1040_7728(param_1, (&ctx.PTR_LOOP_1050_0000 + 1), 0, 0xfc3, param_5);
    //// _var2 = (param_1  >> 0x10);
    local_bx_27 = *param_1;
    local_bx_27.field_0x8e = 0;
    u_var1 = (extraout_var & 0xffff00) << 8;
    local_bx_27.field_0x98 = param_2;
    local_bx_27.field_0x9a = 0;
    local_bx_27.field_0xb2 = 0;
    *param_1 = 0x8ddc;
    local_bx_27.field_0x2 = ctx.PTR_LOOP_1050_1040;
    local_bx_27.field_0x9e = 0;
    local_bx_27.field_0xa2 = 300;
    pass1_fn_1008_60e8(param_4);
    local_bx_27.field_0x90 = u_var1;
    local_bx_27.field_0x92 = ctx.dx_reg;
    pass1_fn_1008_60e8(param_3);
    local_bx_27.field_0x94 = u_var1;
    local_bx_27.field_0x96 = ctx.dx_reg;
    win_fn_1040_8b92(*param_1);
  ctx.PTR_LOOP_1050_5df8 = 0x0;
    return u_var1 & 0xffff0000 | param_1 & 0xffff;
}

pub unsafe fn pass1_1038_e4bc(param_1: &mut u32, param_2: Vec<u8>, param_3: Vec<u8>) {
    let pp_var1: fn();
    let mut u_var2: u32;
    let paVar3: Struct493;
    let mut u_var4: i32;
    let pu_var5: Vec<u8>;
    let mut u_var6: i32;
    let mut u_var7: i32;
    let mut u_var8: u16;
    let pp_var9: Struct2111;
    let mut u_var10: u16;
    let u_var11: u8;
    let mut u_var12: u16;
    let mut u_var13: u16;
    let mut u_var14: u16;
    let mut local_22: u16;
    let mut local_20: u16;
    let mut local_1e: u16;
    let mut local_1c: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    if (param_3 == 0x1c4) {
        pp_var9 = struct_ops_2::process_struct_1010_20ba(ctx.g_struct_var_1050_0ed0, CONCAT22(unaff_si, 0x2f));
      // u_var12 = (pp_var9  >> 0x10);
        u_var7 = (pp_var9 + 0x24);
        u_var6 = (pp_var9 + 0x26);
        if ((u_var6 | u_var7) != 0) {
            paVar3 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, u_var7, u_var6);
            u_var7 = u_var6 | paVar3;
            if (u_var7 != 0) {
                pu_var5 = pass1_1008_c6fa(ctx._PTR_LOOP_1050_06e0, 0x20);
                u_var4 = pu_var5;
                pass1_1038_4e78(CONCAT22(u_var6, paVar3), pu_var5 & 0xffff | u_var7 << 0x10);
                _local_16 = CONCAT22(ctx.dx_reg, u_var4);
                u_var2 = *_local_16;
                pp_var1 = u_var2 + 8;
                u_var7 = u_var4;
                (**pp_var1)(&ctx.PTR_LOOP_1050_1008, u_var4, ctx.dx_reg);
                if ((ctx.dx_reg | u_var7) == 0) {
                    if (_local_16 != 0x0) {
                        pp_var1 = u_var2;
                        (**pp_var1)(&ctx.PTR_LOOP_1050_1008, u_var4, ctx.dx_reg, 1);
                    }
                } else {
                    pp_var1 = (*_local_16 + 4);
                    u_var6 = u_var4;
                    (**pp_var1)(8, u_var4, ctx.dx_reg, 0, 0);
                    u_var8 = ctx.dx_reg;
                    paVar3 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, u_var7, ctx.dx_reg);
                    pp_var9 = struct_ops_2::process_struct_1010_20ba(
                        ctx.g_struct_var_1050_0ed0,
                        CONCAT22(u_var6, 0x32),
                    );
                    win_fn_1010_71d6(
                        pp_var9,
                        1,
                        ((u_var8 & 0xff00) << 0x10 | CONCAT12(u_var8, &paVar3.field_0xc)),
                    );
                    if (_local_16 != 0x0) {
                        pp_var1 = *_local_16;
                        (**pp_var1)(0x1010, u_var4, ctx.dx_reg, 1);
                    }
                }
            }
        }
    } else {
        if (param_3 == 0x1c5) {
            u_var12 = 0xe;
        } else {
            if (param_3 != 0x1c6) {
                win_msg::post_win_msg_1040_7b3c(param_1, param_2, (param_2 >> 0x10), param_3);
                return;
            }
            u_var12 = 0xd;
        }
        u_var14 = 0;
        u_var13 = 0;
        u_var10 = 0;
        u_var11 = 0;
        pp_var9 = struct_ops_2::process_struct_1010_20ba(ctx.g_struct_var_1050_0ed0, 0x32);
        window_msg_func_1010_7300(
            pp_var9,
            (pp_var9 >> 0x10),
            u_var10,
            u_var11,
            u_var12,
            u_var13,
            u_var14,
        );
    }
    return;
}

pub fn pass1_1038_8d7e(param_1: Vec<u8>) {
    ret_1040_78de();
    win_fn_1038_8f74(param_1);
    return;
}

pub unsafe fn pass1_1038_8810(param_1: Vec<u8>) {
    let mut i_var1: i32;
    let mut local_102: [u8; 256];

    i_var1 = send_dlg_item_msg_1038_8164(
        param_1,
        CONCAT22(unaff_ss, local_102),
        (s_logo_bmp_1050_1850 + 6),
    );
    if (i_var1 != 0) {
        pass1_1008_b63a(*(param_1 + 0x94), CONCAT22(unaff_ss, local_102));
    }
    return;
}

pub fn pass1_1038_7dac(param_1: Vec<u8>) {
    ret_1040_78de();
    send_dialog_item_msg_1038_844a(param_1);
    return;
}

pub unsafe fn pass1_1038_7356(param_1: &mut Struct1159, param_2: &mut Struct921) {
    let mut pu_var1: u32;
    let mut u_var2: u32;
    let mut struct_var3: Struct7;
    let mut lVar3: u32;
    let mut b_var4: bool;
    let mut u_var5: i32;
    let mut paVar6: Struct493;
    let mut u_var8: i32;
    let mut u_var9: u16;
    let mut u_var10: u16;
    let mut bVar11: bool;
    let mut u_var12: u32;
    let mut u_var13: u32;
    let mut local_32: u32;
    let mut local_1e: u16;
    let mut local_1c: u16;
    let mut local_1a: u32;
    let mut local_12: u32;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;
    let mut pu_var7: Vec<u8>;

    u_var12 = pass1_1030_73a8(param_2);
  // u_var8 = (u_var12  >> 0x10);
    u_var5 = u_var8;
    b_var4 = pass1_1008_c6ae(ctx._PTR_LOOP_1050_06e0, (u_var12 + 0xc), 4);
    local_bx_40 = param_1;
  // u_var9 = (param_1  >> 0x10);
    if (b_var4 == 0) {
        b_var4 = pass1_1008_c6ae(ctx._PTR_LOOP_1050_06e0, (u_var12 + 0xc), 3);
        if (b_var4 == 0) {
            // code_r0x10387545:
            pass1_1038_6f5a(param_1, param_2);
            // goto LAB_1038_7549;
        }
        if ((local_bx_40.field_0xc != 0) || (&local_bx_40.field_0xe != 0)) {
            u_var13 = pass1_1028_45e2(u_var12);
          // u_var5 = (u_var13  >> 0x10);
            pu_var1 = &local_bx_40.field_0x18;
            unsafe {
                bVar11 = *pu_var1 < u_var5;
                if ((bVar11 || *pu_var1 == u_var5)
                    && (bVar11
                        || (
                            pu_var1 = &local_bx_40.field_0x16,
                            *pu_var1 < u_var13 || *pu_var1 == u_var13,
                        )))
                {}
            }
            // goto code_r0x10387545;
        }
    } else {
        u_var13 = pass1_1028_62c8(u_var12);
      // u_var5 = (u_var13  >> 0x10);
        pu_var1 = &local_bx_40.field_0x18;
        unsafe {
            bVar11 = *pu_var1 < u_var5;
        }
        if ((bVar11 || unsafe { *pu_var1 == u_var5 })
            && (bVar11
                || (
                    pu_var1 = &local_bx_40.field_0x16,
                    unsafe { *pu_var1 < u_var13 } || unsafe { *pu_var1 == u_var13 },
                )))
        {
            if (local_bx_40.field_0x12 == 0) {
                if (local_bx_40.field_0x14 == 0) {}
                // goto LAB_1038_74e0;
                pu_var7 = _PTR_LOOP_1050_68a2;
                alloc_mem_1000_07fc(ctx._PTR_LOOP_1050_68a2);
                u_var5 = pu_var7;
                local_32 = pu_var7 & 0xffff | ctx.dx_reg << 0x10;
                if ((ctx.dx_reg | u_var5) == 0) {
                    local_32 = 0;
                } else {
                    local_32 = ctx.s_1_1050_389a;
                    (u_var5 + 2) = &ctx.PTR_LOOP_1050_1008;
                    (u_var5 + 4) = 0;
                    (u_var5 + 6) = 0;
                    (u_var5 + 8) = 0;
                    (u_var5 + 10) = 0;
                    (u_var5 + 0xc) = 0;
                    local_32 = 0x56ce;
                    (u_var5 + 2) = 0x1018;
                }
              // u_var10 = (local_32  >> 0x10);
                local_bx_185 = local_32;
                local_bx_185.field_0x8 = local_bx_40.field_0x14;
                local_bx_185.field_0xa = local_bx_40.field_0x16;
                u_var5 = pass1_1020_c42e(local_bx_40.field_0x14);
            } else {
                pu_var7 = _PTR_LOOP_1050_68a2;
                alloc_mem_1000_07fc(ctx._PTR_LOOP_1050_68a2);
                u_var5 = pu_var7;
                local_1a = pu_var7 & 0xffff | ctx.dx_reg << 0x10;
                if ((ctx.dx_reg | u_var5) == 0) {
                    local_1a = 0;
                } else {
                    local_1a = ctx.s_1_1050_389a;
                    (u_var5 + 2) = &ctx.PTR_LOOP_1050_1008;
                    (u_var5 + 4) = 0;
                    (u_var5 + 6) = 0;
                    (u_var5 + 8) = 0;
                    (u_var5 + 10) = 0;
                    (u_var5 + 0xc) = 0;
                    local_1a = 0x56ce;
                    (u_var5 + 2) = 0x1018;
                }
              // u_var10 = (local_1a  >> 0x10);
                local_bx_185 = local_1a;
                local_bx_185.field_0x6 = local_bx_40.field_0x12;
                local_bx_185.field_0xa = local_bx_40.field_0x16;
                u_var5 = switch_statement_1020_c3b4(local_bx_40.field_0x12);
            }
            lVar3 = u_var5 * local_bx_185.field_0xa;
          // u_var5 = (lVar3  >> 0x10);
            local_bx_185.field_0xc = lVar3;
            pass1_1028_6408(u_var12, CONCAT22(u_var10, local_bx_185));
            // goto LAB_1038_7549;
        }
    }
    // LAB_1038_74e0:
    pass1_1038_709c(param_1, param_2);
    // LAB_1038_7549:
    u_var2 = local_bx_40.field_0x8;
  // paVar6 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, u_var2, (u_var2  >> 0x10));
    pass1_1030_6c4c(
        CONCAT22(u_var5, paVar6),
        &paVar6[1].field_0x16 + local_bx_40.field_0x26,
    );
    local_bx_40.field_0xc = 0;
    local_bx_40.field_0x12 = 0;
    local_bx_40.field_0x14 = 0;
    local_bx_40.field_0x16 = 0;
    struct_var3 = local_bx_40.field_0xe;
    u_var5 = local_bx_40.field_0x10;
    if (u_var5 | struct_var3) != 0 {
        pass1_1020_ba7e((struct_var3 & 0xffff | u_var5 << 0x10));
        error_check_1000_17ce(ctx, &mut struct_var3);
    }
    local_bx_40.field_0xe = 0;
    return;
}

pub unsafe fn pass1_1038_709c(param_1: &mut Struct1159, param_2: &mut Struct921) {
    let mut u_var1: u32;
    let mut u_var2: u16;
    let mut u_var3: i32;
    let mut i_var4: i32;
    let pu_var5: Vec<u8>;
    let mut u_var6: u16;
    let mut u_var7: u16;
    let mut local_28: u32;
    let mut local_20: u32;
    let mut local_18: u32;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u32;
    let mut local_c: u16;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

  // u_var6 = (param_1  >> 0x10);
    local_bx_4 = param_1;
    if ((local_bx_4.field_0x10 | &local_bx_4.field_0xe) == 0) {
        if (local_bx_4.field_0xc == 0) {
            if (local_bx_4.field_0x12 == 0) {
                if (local_bx_4.field_0x14 == 0) {
                    return;
                }
                pu_var5 = _PTR_LOOP_1050_68a2;
                alloc_mem_1000_07fc(ctx._PTR_LOOP_1050_68a2);
                u_var3 = pu_var5;
                local_28 = pu_var5 & 0xffff | ctx.dx_reg << 0x10;
                if ((ctx.dx_reg | u_var3) == 0) {
                    local_28 = 0;
                } else {
                    local_28 = ctx.s_1_1050_389a;
                    (u_var3 + 2) = &ctx.PTR_LOOP_1050_1008;
                    (u_var3 + 4) = 0;
                    (u_var3 + 6) = 0;
                    (u_var3 + 8) = 0;
                    (u_var3 + 10) = 0;
                    (u_var3 + 0xc) = 0;
                    local_28 = 0x56ce;
                    (u_var3 + 2) = 0x1018;
                }
              // u_var7 = (local_28  >> 0x10);
                (local_28 + 8) = local_bx_4.field_0x14;
                (local_28 + 10) = local_bx_4.field_0x16;
                u_var2 = pass1_1020_c42e(local_bx_4.field_0x14);
            } else {
                pass1_1030_7c50(param_2, &local_bx_4.field_0x16, local_bx_4.field_0x12);
                pu_var5 = _PTR_LOOP_1050_68a2;
                alloc_mem_1000_07fc(ctx._PTR_LOOP_1050_68a2);
                u_var3 = pu_var5;
                local_28 = pu_var5 & 0xffff | ctx.dx_reg << 0x10;
                if ((ctx.dx_reg | u_var3) == 0) {
                    local_28 = 0;
                } else {
                    local_28 = ctx.s_1_1050_389a;
                    (u_var3 + 2) = &ctx.PTR_LOOP_1050_1008;
                    (u_var3 + 4) = 0;
                    (u_var3 + 6) = 0;
                    (u_var3 + 8) = 0;
                    (u_var3 + 10) = 0;
                    (u_var3 + 0xc) = 0;
                    local_28 = 0x56ce;
                    (u_var3 + 2) = 0x1018;
                }
              // u_var7 = (local_28  >> 0x10);
                (local_28 + 6) = local_bx_4.field_0x12;
                (local_28 + 10) = local_bx_4.field_0x16;
                u_var2 = switch_statement_1020_c3b4(local_bx_4.field_0x12);
            }
          // u_var7 = (local_28  >> 0x10);
            local_bx_337 = local_28;
            i_var4 = u_var2 * local_bx_337.field_0xa;
        } else {
            pu_var5 = _PTR_LOOP_1050_68a2;
            alloc_mem_1000_07fc(ctx._PTR_LOOP_1050_68a2);
            u_var3 = pu_var5;
            local_28 = pu_var5 & 0xffff | ctx.dx_reg << 0x10;
            if ((ctx.dx_reg | u_var3) == 0) {
                local_28 = 0;
            } else {
                local_28 = ctx.s_1_1050_389a;
                (u_var3 + 2) = &ctx.PTR_LOOP_1050_1008;
                (u_var3 + 4) = 0;
                (u_var3 + 6) = 0;
                (u_var3 + 8) = 0;
                (u_var3 + 10) = 0;
                (u_var3 + 0xc) = 0;
                local_28 = 0x56ce;
                (u_var3 + 2) = 0x1018;
            }
          // u_var7 = (local_28  >> 0x10);
            local_bx_337 = local_28;
            local_bx_337.field_0x4 = local_bx_4.field_0xc;
            i_var4 = local_bx_4.field_0x16;
            local_bx_337.field_0xa = i_var4;
        }
        local_bx_337.field_0xc = i_var4;
        pass1_1030_6a2c(param_2, CONCAT22(u_var7, local_bx_337));
    } else {
        u_var1 = &local_bx_4.field_0xe;
        local_4 = (u_var1 + 4);
        local_c = 0;
        while (local_c < local_4) {
            pass1_1020_bb16(
                &local_bx_4.field_0xe,
                CONCAT22(unaff_ss, &local_a),
                CONCAT22(unaff_ss, &local_6),
                local_c,
            );
            if (local_a != 0) {
                pu_var5 = _PTR_LOOP_1050_68a2;
                alloc_mem_1000_07fc(ctx._PTR_LOOP_1050_68a2);
                u_var3 = pu_var5;
                local_10 = pu_var5 & 0xffff | ctx.dx_reg << 0x10;
                if ((ctx.dx_reg | u_var3) == 0) {
                    local_10 = 0;
                } else {
                    local_10 = ctx.s_1_1050_389a;
                    (u_var3 + 2) = &ctx.PTR_LOOP_1050_1008;
                    (u_var3 + 4) = 0;
                    (u_var3 + 6) = 0;
                    (u_var3 + 8) = 0;
                    (u_var3 + 10) = 0;
                    (u_var3 + 0xc) = 0;
                    local_10 = 0x56ce;
                    (u_var3 + 2) = 0x1018;
                }
              // u_var7 = (local_10  >> 0x10);
                local_bx_49 = local_10;
                local_bx_49.field_0x4 = local_6;
                local_bx_49.field_0xa = local_a;
                u_var2 = ret_one_1020_c3ae();
                local_bx_49.field_0xc = u_var2 * local_bx_49.field_0xa;
                pass1_1030_6a2c(param_2, local_10);
            }
            local_c = local_c + 1;
        }
    }
    return;
}

pub unsafe fn pass1_1038_6f5a(param_1: u32, param_2: &mut Struct921) {
    // ppu_var1: &mut Vec<u8>;
    let local_AX_168: Struct1157;
    let mut u_var2: i32;
    let mut u_var3: u16;

    let local_bx_4: Struct1156;
    let mut i_var5: i32;
    let mut u_var6: u16;
    let mut u_var7: u16;
    let mut unaff_ss: u16;
    let mut local_18: u32;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u32;
    let mut local_c: u16;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;
    let pu_var4: Vec<u8>;

  // u_var6 = (param_1  >> 0x10);
    local_bx_4 = param_1;
    if (local_bx_4.field_0xe == 0x0) {
        if (local_bx_4.field_0xc != 0) {
            pass1_1030_7ddc(param_2, local_bx_4.field_0x16, local_bx_4.field_0xc);
            return;
        }
        if (local_bx_4.field_0x12 != 0) {
            pass1_1030_7c50(param_2, local_bx_4.field_0x16, local_bx_4.field_0x12);
            return;
        }
        pu_var4 = _PTR_LOOP_1050_68a2;
        alloc_mem_1000_07fc(ctx._PTR_LOOP_1050_68a2);
        u_var2 = pu_var4;
        local_10 = pu_var4 & 0xffff | ctx.dx_reg << 0x10;
        if ((ctx.dx_reg | u_var2) == 0) {
            local_10 = 0;
        } else {
            local_10 = ctx.s_1_1050_389a;
            (u_var2 + 2) = &ctx.PTR_LOOP_1050_1008;
            (u_var2 + 4) = 0;
            (u_var2 + 6) = 0;
            (u_var2 + 8) = 0;
            (u_var2 + 10) = 0;
            (u_var2 + 0xc) = 0;
            local_10 = 0x56ce;
            (u_var2 + 2) = 0x1018;
        }
      // u_var7 = (local_10  >> 0x10);
        i_var5 = local_10;
        (i_var5 + 8) = local_bx_4.field_0x14;
        (i_var5 + 10) = &local_bx_4.field_0x16;
        u_var3 = pass1_1020_c42e(local_bx_4.field_0x14);
        (i_var5 + 0xc) = u_var3 * (i_var5 + 10);
        pass1_1030_6a2c(param_2, local_10);
    } else {
        ppu_var1 = local_bx_4.field_0xe;
        local_4 = (ppu_var1 + 4);
        local_c = 0;
        while (local_c < local_4) {
            pass1_1020_bb16(
                local_bx_4.field_0xe,
                CONCAT22(unaff_ss, &local_a),
                CONCAT22(unaff_ss, &local_6),
                local_c,
            );
            if (CONCAT22(local_a, local_a) != 0) {
                pass1_1030_7ddc(param_2, CONCAT22(local_a, local_a), local_6);
            }
            local_c = local_c + 1;
        }
    }
    return;
}

pub unsafe fn pass1_1038_095e(param_1: u16, param_2: u16, param_1_00: i32, param_2_00: &mut Struct1115) {
    let pp_var1: fn();
    let paVar2: Struct493;
    let mut u8_var3: bool;
    let pu_var4: Vec<u8>;
    let pu_var5: Vec<u8>;
    let mut u_var6: u32;
    let mut u_var7: i32;
    let mut u_var8: u32;
    let mut u_var9: u16;
    let mut unaff_ss: u16;
    let mut u_var10: i32;
    let mut u_var11: u16;
    let mut local_48: u32;
    let mut local_44: u32;
    let mut local_40: u16;
    let mut local_3e: u16;
    let mut local_3c: u16;
    let mut local_3a: u32;
    let mut local_36: u32;
    let mut local_32: u16;
    let mut local_30: u16;
    let mut local_2e: u16;
    let mut local_2c: u32;
    let mut local_28: [u8; 2];
    let mut local_26: u16;
    let mut local_24: u16;
    let mut local_22: u16;
    let mut local_20: u16;
    let mut local_1e: u16;
    let mut local_1c: u16;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u32;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    _local_6 = struct_ops_2::process_struct_1010_20ba(ctx.g_struct_var_1050_0ed0, CONCAT22(local_48, 0x37));
    local_a = *ctx._PTR_LOOP_1050_65e2;
  // u_var9 = (param_2_00  >> 0x10);
    if (local_a % 10 == 0) {
        if (param_1_00 < 0xc9) {
            u_var11 = 0x3f;
        } else {
            if (param_1_00 < 800) {}
            // goto LAB_1038_09c3;
            u_var11 = 0x3e;
        }
        win_msg::post_win_msg_1008_a0e4(_local_6, 0, 0, 1, (param_2_00 + 4), u_var11);
    }
    // LAB_1038_09c3:
    local_c = (param_2_00 + 0x22);
    local_e = 0;
    local_12 = *ctx._PTR_LOOP_1050_65e2;
    u_var7 = (ctx._PTR_LOOP_1050_65e2 + 2);
    if (local_c < 0x4b) {
        if (local_c < 0x3c) {
            if (local_c < 0x32) {}
            // goto LAB_1038_0a1c;
            u_var10 = 0x1e;
        } else {
            u_var10 = 0xf;
        }
    } else {
        u_var10 = 5;
    }
    u_var8 = (local_12 & 0xffff | u_var7 << 0x10) % u_var10;
    u_var7 = u_var8;
    if (u_var8 == 0) {
        local_e = 1;
    }
    // LAB_1038_0a1c:
    if (local_e != 0) {
        pu_var5 = pass1_1008_c6fa(ctx._PTR_LOOP_1050_06e0, 0xf);
        u_var9 = SUB42(pu_var5, 0);
        pass1_1038_4e78(param_2_00, pu_var5 & 0xffff | u_var7 << 0x10);
        _local_16 = CONCAT22(u_var7, u_var9);
        pu_var5 = pass1_1008_c6fa(ctx._PTR_LOOP_1050_06e0, 0x1a);
        u_var11 = pu_var5;
        local_1a = u_var11;
        local_18 = u_var7;
        pass1_1038_4d6e(param_2_00, pu_var5 & 0xffff | u_var7 << 0x10);
        _local_1e = CONCAT22(u_var7, u_var11);
        pp_var1 = (*_local_16 + 0x10);
        (**pp_var1)(&ctx.PTR_LOOP_1050_1008, _local_16, (_local_16 >> 0x10));
        _local_22 = CONCAT22(u_var7, u_var11);
        pp_var1 = (*_local_1e + 0x10);
        (**pp_var1)(&ctx.PTR_LOOP_1050_1008, _local_1e, (_local_1e >> 0x10));
        _local_26 = CONCAT22(u_var7, u_var11);
        u_var8 = pass1_1030_bcae(local_28, unaff_ss);
        local_36 = 0;
        while (true) {
          // u_var8 = u_var8  >> 0x10;
            u_var9 = 0x1030;
            if (_local_22 <= local_36) {
                break;
            }
            u_var6 = _local_22;
            pass1_1030_1d58(_local_16);
            paVar2 = (u_var6 & 0xffff | u_var8 << 0x10);
            u8_var3 = false;
            local_3a = 0;
            while (local_3a < _local_26) {
                u_var6 = _local_26;
                pass1_1030_1d58(_local_1e);
                pu_var4 = local_28;
                pass1_1030_bd74(
                    pu_var4,
                    unaff_ss,
                    paVar2,
                    (u_var6 & 0xffff | u_var8 << 0x10),
                );
                if (pu_var4 < 6) {
                    u8_var3 = true;
                    break;
                }
                local_3a = local_3a + 1;
            }
            u_var8 = pass1_1030_73a8(paVar2);
            if (!u8_var3) {
                u_var9 = SUB42(&ctx.PTR_LOOP_1050_1028, 0);
                pass1_1028_5ca0(u_var8);
                break;
            }
            local_36 = local_36 + 1;
        }
        if (_local_16 != 0x0) {
            pp_var1 = *_local_16;
            (**pp_var1)(u_var9, _local_16, (_local_16 >> 0x10), 1);
        }
        if (_local_1e != 0x0) {
            pp_var1 = *_local_1e;
            (**pp_var1)(u_var9, _local_1e, (_local_1e >> 0x10), 1);
        }
    }
    return;
}

pub unsafe fn pass1_1030_838e(param_1: &mut u32) {
    unsafe {
        pass1_1028_d2b0(param_1);
    }
    pass1_1028_d01a((param_1 + 4));
    win_msg::send_win_msg_1028_e242(ctx._PTR_LOOP_1050_65e2, (ctx._PTR_LOOP_1050_65e2 >> 0x10));
    return;
}

pub unsafe fn pass1_1030_83ba(ctx: &mut AppContext, param_1: &mut u32, param_2: &mut u32) {
    let u32_var1: u32;

    while (u32_var1 = param_2 -1, param_2 != 0) {
        unsafe {
            pass1_1028_d2b0(param_1);
        }
        pass1_1028_d01a((param_1 + 4));
        *param_2 = u32_var1;
        if u32_var1 != 0 {
            win_msg::send_win_msg_1028_e242(ctx._PTR_LOOP_1050_65e2, 0);
        }
    }
    win_msg::send_win_msg_1028_e242(
        ctx._PTR_LOOP_1050_65e2,
        (ctx._PTR_LOOP_1050_65e2 >> 0x10),
        1,
    );
    return;
}

pub unsafe fn pass1_1028_af08(param_1: u32) -> u16 {
    let mut u_var1: u32;
    let mut u_var2: u16;
    let paVar3: Struct493;
    let mut u_var4: u16;
    let ppVar5: Struct2111;
    let mut in_stack_0000ffde: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u32;

    struct_ops_2::process_struct_1010_20ba(ctx.g_struct_var_1050_0ed0, CONCAT22(in_stack_0000ffde, 2));
    if ((u16_1050_13ae < 1) || (SBORROW2(u16_1050_13ae, 1))) {
        // LAB_1028_af27:
        local_c = 1;
    } else {
        if (u16_1050_13ae == 2 || (u16_1050_13ae - 1) < 1) {
            local_c = 0x10001;
            // goto LAB_1028_af42;
        }
        if (u16_1050_13ae != 4) {}
        // goto LAB_1028_af27;
        local_c = 2;
    }
    local_c = CONCAT22(local_c, 3);
    // LAB_1028_af42:
    u_var2 = pass1_fn_1008_612e(local_c, local_c);
  // u_var4 = (param_1  >> 0x10);
    local_bx_72 = param_1;
    local_bx_72.field_0x114 = u_var2;
    ppVar5 = struct_ops_2::process_struct_1010_20ba(
        ctx.g_struct_var_1050_0ed0,
        CONCAT22(in_stack_0000ffde, 0x37),
    );
  // u_var2 = (ppVar5  >> 0x10);
    u_var1 = local_bx_72.field_0x108;
  // paVar3 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, u_var1, (u_var1  >> 0x10));
    win_msg::post_win_msg_1008_a0e4(
        &mut ppVar5,
        0,
        local_bx_72.field_0x114,
        &paVar3[0x11].field_0xa,
        local_bx_72.field_0x108,
        2,
    );
    ppVar5 = struct_ops_2::process_struct_1010_20ba(
        ctx.g_struct_var_1050_0ed0,
        CONCAT22(in_stack_0000ffde, 0x2b),
    );
    pass1_1010_043a(ppVar5, &paVar3.field_0x4, 0xd);
    return 1;
}

pub unsafe fn pass1_1028_a188(param_1: u16, param_2: u16, param_1_00: i32, param_2_00: i32, param_5: u32) {
    let mut u_var1: u32;
    let lVar2: u32;
    let mut u_var3: u32;
    let mut u_var4: u16;
    let mut u_var5: i32;
    let mut u_var6: i32;
    let mut u_var7: u32;
    let lVar8: u32;
    let lVar9: u32;
    let mut u_var10: i32;
    let local_bx_6: Struct816;
    let mut i_var11: i32;
    let mut unaff_si: u16;
    let mut u_var12: u16;
    let ppVar13: Struct2111;
    let mut local_26: u16;
    let mut local_24: u16;
    let mut local_12: u32;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u32;
    let mut local_6: u32;

  // u_var12 = (param_5  >> 0x10);
    local_bx_6 = param_5;
    u_var1 = &local_bx_6.field_0x1f6;
    u_var6 = &local_bx_6.field_0x1f8;
    u_var5 = u_var1 + 0x18c;
  // u_var4 = (u_var1  >> 0x10);
    u_var7 = u_var5;
    pass1_1030_38f2(u_var1, u_var6, param_2_00);
    u_var3 = 100 / param_1_00;
    u_var10 = u_var3 >> 0xf;
    i_var11 = param_2_00 * 4;
    lVar2 = (u_var7 & 0xffff | u_var6 << 0x10) + (i_var11 + u_var5);
    lVar8 = lVar2 / (u_var3 & 0xffff | u_var10 << 0x10);
    lVar9 = lVar8 * (u_var3 & 0xffff | u_var10 << 0x10);
    local_e = lVar2;
  // local_c = (lVar2  >> 0x10);
    u_var6 = lVar9;
    (u_var5 + i_var11) = local_e - u_var6;
    (u_var5 + i_var11 + 2) = (local_c - (lVar9 >> 0x10)) - (local_e < u_var6);
    local_12 = (lVar8 >> 0x10);
    local_12._0_2_ = lVar8;
    if ((local_12 | local_12) != 0) {
        pass1_1030_375a(u_var1, u_var4, param_2_00, local_12, local_12);
        if (local_bx_6.field_0x200 != 0x8000002) {
            ppVar13 =
                struct_ops_2::process_struct_1010_20ba(ctx.g_struct_var_1050_0ed0, CONCAT22(unaff_si, 0x37));
            win_msg::post_win_msg_1008_a0e4(
                &mut ppVar13,
                0,
                local_12,
                local_bx_6.field_0x208,
                local_bx_6.field_0x4,
                2,
            );
            ppVar13 =
                struct_ops_2::process_struct_1010_20ba(ctx.g_struct_var_1050_0ed0, CONCAT22(unaff_si, 0x2b));
            pass1_1010_043a(ppVar13, local_bx_6.field_0x4, 0xd);
        }
    }
    return;
}

pub unsafe fn pass1_1028_9a02(param_1: &mut Struct806) -> u8 {
    let mut u_var1: u32;
    let paVar2: Struct493;
    let mut u_var3: i32;
    let pu_var4: u16;
    let mut u_var5: u32;
    let mut u_var6: u16;
    let pp_var7: Struct2111;
    let u_var8: u8;
    let u_var9: u8;
    let u_var10: u8;
    let u_var11: u8;
    let mut u_var12: u16;
    let mut u_var13: u16;
    let mut u_var14: u16;
    let mut uVar15: u16;
    let mut local_38: u16;
    let mut local_36: u16;
    let mut local_30: u16;
    let mut local_2e: u16;
    let mut local_1e: u16;
    let mut local_1a: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u32;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

  // u_var6 = (param_1  >> 0x10);
    local_bx_4 = param_1;
    u_var1 = local_bx_4.field_0x108;
  // paVar2 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, u_var1, (u_var1  >> 0x10));
    _local_6 = CONCAT22(ctx.dx_reg, paVar2);
    u_var5 = paVar2[0x10].field_0x16;
    u_var1 = local_bx_4.field_0x110;
    local_a = u_var5;
    pass1_1030_3694(u_var5, (u_var5 >> 0x10), 0, u_var1, (u_var1 >> 0x10));
    u_var3 = u_var5;
    local_bx_4.field_0x114 = u_var3;
    local_bx_4.field_0x116 = ctx.dx_reg;
    pass1_1030_38b8(local_a, (local_a >> 0x10));
    if ((ctx.dx_reg | u_var3) == 0) {
        local_12 = (_local_6 + 0x200);
        pp_var7 = struct_ops_2::process_struct_1010_20ba(ctx.g_struct_var_1050_0ed0, CONCAT22(local_38, 0x2b));
      // local_14 = (pp_var7  >> 0x10);
        local_16 = pp_var7;
        if (local_12 == 0x8000002) {
            u_var14 = 0x1f;
        } else {
            u_var14 = 0xb;
        }
        pass1_1010_043a(pp_var7, (_local_6 + 4), u_var14);
        if (local_12 == 0x8000001) {
            u_var6 = 2;
        } else {
            u_var6 = 1;
        }
        local_12 = CONCAT22(0x800, u_var6);
        pass1_1038_349e(_local_6, CONCAT22(0x800, u_var6));
        local_1e = 0;
        local_1a = 0;
        pass1_1028_dc52(
            CONCAT13((unaff_ss >> 8), CONCAT12(unaff_ss, &local_30)),
            (&ctx.PTR_LOOP_1050_0000 + 1),
            0,
            0x400,
        );
        while (true) {
            pu_var4 = &local_30;
            pass1_1028_e4ec(CONCAT22(unaff_ss, pu_var4));
            _local_6 = CONCAT22(ctx.dx_reg, pu_var4);
            if ((ctx.dx_reg | pu_var4) == 0) {
                break;
            }
            if ((pu_var4 + 0x100) == 0x8000002) {
                local_1a = 1;
            } else {
                local_1e = 1;
            }
        }
        if (local_1e == 0) {
            u_var13 = 0;
            uVar15 = 0x3c;
            u_var10 = 1;
            u_var11 = 0;
            u_var12 = 0;
            u_var6 = 0;
            u_var14 = 0;
            u_var8 = 0;
            u_var9 = 0;
            pp_var7 = struct_ops_2::process_struct_1010_20ba(ctx.g_struct_var_1050_0ed0, 0x37);
            win_msg::post_win_msg_1008_a0e4(
                &mut pp_var7,
                CONCAT22(u_var6, CONCAT11(u_var9, u_var8)),
                u_var14,
                CONCAT11(u_var11, u_var10),
                CONCAT22(u_var13, u_var12),
                uVar15,
            );
        }
    }
    return 0x1;
}

pub fn pass1_1028_74e4(param_1: u32) -> u16 {
    pass1_1028_7fb6(param_1);
    pass1_1028_7c4e(param_1);
    pass1_1028_7dfc(param_1);
    post_msg_1028_76da(param_1);
    pass1_1028_767e(param_1);
    pass1_1028_75bc();
    pass1_1028_78b8(param_1);
    return 1;
}

pub unsafe fn pass1_1028_6ff6(param_1: u32) {
    let pu_var1: u16;
    let paVar2: Struct493;
    let mut u_var3: u16;
    let mut u_var4: u32;




    let mut u_var5: i32;

    let mut extraout_dx_04: i32;
    let mut unaff_ss: u16;
    let ppVar6: Struct2111;
    let mut u_var7: u16;
    let mut u_var8: u16;
    let u_var9: u8;
    let u_var10: u8;
    let u_var11: u8;
    let mut u_var12: u16;
    let mut u_var13: u16;
    let mut u_var14: u16;
    let mut local_48: u16;
    let mut local_46: u16;
    let mut local_44: u16;
    let mut local_34: u16;
    let mut local_32: u16;
    let mut local_30: u16;
    let mut local_2e: u16;
    let mut local_26: Struct2111;
    let mut local_22: u16;
    let mut local_20: u16;
    let mut local_1e: u16;
    let mut local_1c: u16;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    u_var10 = (unaff_ss >> 8);
    pass1_1028_dc52(
        CONCAT13(u_var10, CONCAT12(unaff_ss, &local_14)),
        (&ctx.PTR_LOOP_1050_0000 + 1),
        0,
        0x400,
    );
    local_1a = 1;
    local_1c = 0;
    while {
        while {
            pu_var1 = &local_14;
            pass1_1028_e4ec(CONCAT22(unaff_ss, pu_var1));
            _local_18 = CONCAT22(ctx.dx_reg, pu_var1);
            if ((ctx.dx_reg | pu_var1) == 0) {}
            // goto LAB_1028_7066;
            (pu_var1[0xff] == 0) || ((pu_var1 + 0x100) == 0x8000002)
        } {}
        local_1c = 1;
        u_var4 = (pu_var1 + 0xfb);
        local_26 = u_var4;
        pass1_1030_38b8(u_var4, (u_var4 >> 0x10));
        (ctx.dx_reg < 0) || (ctx.dx_reg < 1 && (u_var4 == 0))
    } {}
    local_1a = 0;
    // LAB_1028_7066:
    local_a = local_6;
    local_c = local_8;
    if (local_4 != 0) {
        local_a = 0;
        local_c = 1;
    }
    local_1e = 0;
    while (true) {
        pu_var1 = &local_14;
        pass1_1028_e4ec(CONCAT22(unaff_ss, pu_var1));
        _local_18 = CONCAT22(ctx.dx_reg, pu_var1);
        local_20 = ctx.dx_reg | pu_var1;
        if (local_20 == 0) {
            break;
        }
        if ((pu_var1 + 0x100) == 0x8000001) {
            local_1e = 1;
        }
    }
    if (local_1e == 0) {
        paVar2 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, 1, 0x400);
        _local_18 = CONCAT22(local_20, paVar2);
        local_20 = local_20 | paVar2;
        if (local_20 != 0) {
          ctx.PTR_LOOP_1050_4fe8 = (&ctx.PTR_LOOP_1050_0000 + 1);
            u_var3 = 0;
            u_var14 = 1;
            _local_34 = struct_ops_2::process_struct_1010_20ba(ctx.g_struct_var_1050_0ed0, 0x2b);
            pass1_1010_089e(_local_34, u_var3, u_var14);
            pass1_1010_089e(_local_34, 0, 2);
            pass1_1010_089e(_local_34, 0, 3);
            pass1_1010_089e(_local_34, 0, 4);
            pass1_1010_089e(_local_34, 0, 5);
            pass1_1010_089e(_local_34, 0, 7);
            pass1_1010_089e(_local_34, 0, 8);
            pass1_1010_089e(_local_34, 0, 10);
            local_20 = ctx.dx_reg;
        }
    }
    if ((local_1c != 0) && (local_1a != 0)) {
        u_var13 = 0;
        u_var14 = 6;
        u_var9 = 1;
        u_var11 = 0;
        u_var12 = 0;
        u_var8 = 0;
        u_var3 = 0;
        u_var7 = 0;
        _local_34 = struct_ops_2::process_struct_1010_20ba(ctx.g_struct_var_1050_0ed0, 0x37);
      // local_20 = (_local_34  >> 0x10);
        win_msg::post_win_msg_1008_a0e4(
            _local_34,
            CONCAT22(u_var8, u_var7),
            u_var3,
            CONCAT11(u_var11, u_var9),
            CONCAT22(u_var13, u_var12),
            u_var14,
        );
    }
    local_22 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, 1, 0x800);
    u_var5 = local_20 | local_22;
    if (((((u_var5 != 0)
        && (
            u_var3 = pass1_1030_2242(CONCAT22(local_20, local_22), 4),
            u_var3 == 0,
        ))
        && (
            u_var3 = pass1_1030_2242(CONCAT22(local_20, local_22), 0x2a),
            u_var3 == 0,
        ))
        && ((
            u_var3 = pass1_1030_2242(CONCAT22(local_20, local_22), 0x4b),
            u_var3 == 0
                && (
                    u_var3 = pass1_1030_2242(CONCAT22(local_20, local_22), 0x54),
                    u_var3 == 0,
                ),
        )))
        && ((
            u_var3 = pass1_1030_2242(CONCAT22(local_20, local_22), 0x2c),
            u_var3 == 0
                && ((
                    u_var3 = pass1_1030_2242(CONCAT22(local_20, local_22), 0x3c),
                    u_var3 == 0
                        && (
                            u_var3 = pass1_1030_2242(CONCAT22(local_20, local_22), 0x3d),
                            u_var3 == 0,
                        ),
                )),
        )))
    {
        if (local_4 != 0) {
            local_8 = 1;
            local_6 = 0;
        }
        _local_34 = (_local_34 & 0xffff0000);
        local_30 = 0;
        local_c = local_8;
        local_a = local_6;
        while {
            while {
                pu_var1 = &local_14;
                pass1_1028_e4ec(CONCAT22(unaff_ss, pu_var1));
                _local_18 = CONCAT22(ctx.dx_reg, pu_var1);
                u_var5 = ctx.dx_reg | pu_var1;
                if (u_var5 == 0) {}
                // goto LAB_1028_72d3;
                (pu_var1 + 0x100) == 0x8000002
            } {}
          // u_var3 = (param_1  >> 0x10);
            if ((local_34 == 0)
                && (
                    pass1_1028_740c(param_1, u_var3, 0x22, CONCAT22(ctx.dx_reg, pu_var1)),
                    pu_var1 != 0x0,
                ))
            {
                _local_34 = CONCAT22(local_32, 1);
            }
            if ((local_30 == 0)
                && (
                    pass1_1028_740c(param_1, u_var3, 0x24, _local_18),
                    pu_var1 != 0x0,
                ))
            {
                local_30 = 1;
            }
            (local_34 == 0) || (local_30 == 0)
        } {}
        u_var13 = 0;
        u_var14 = 0x14;
        u_var9 = 1;
        u_var11 = 0;
        u_var12 = 0;
        u_var8 = 0;
        u_var3 = 0;
        u_var7 = 0;
        local_26 = struct_ops_2::process_struct_1010_20ba(ctx.g_struct_var_1050_0ed0, 0x37);
      // u_var5 = (local_26  >> 0x10);
        win_msg::post_win_msg_1008_a0e4(
            &mut local_26,
            CONCAT22(u_var8, u_var7),
            u_var3,
            CONCAT11(u_var11, u_var9),
            CONCAT22(u_var13, u_var12),
            u_var14,
        );
    }
    // LAB_1028_72d3:
    paVar2 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, 1, 0x400);
    _local_18 = CONCAT22(u_var5, paVar2);
    if ((u_var5 | paVar2) != 0) {
        ppVar6 = struct_ops_2::process_struct_1010_20ba(ctx.g_struct_var_1050_0ed0, CONCAT22(local_48, 0x3b));
      // local_2e = (ppVar6  >> 0x10);
        local_30 = ppVar6;
        pass1_1008_df4a(ppVar6);
        ppVar6 = struct_ops_2::process_struct_1010_20ba(ctx.g_struct_var_1050_0ed0, CONCAT22(local_48, 0x3c));
      // local_2e = (ppVar6  >> 0x10);
        local_30 = ppVar6;
        pass1_1018_34a6(ppVar6);
        pass1_1028_dc52(
            CONCAT13(u_var10, CONCAT12(unaff_ss, &local_46)),
            (&ctx.PTR_LOOP_1050_0000 + 1),
            0,
            0x400,
        );
        while (true) {
            pu_var1 = &local_46;
            pass1_1028_e4ec(CONCAT22(unaff_ss, pu_var1));
            _local_34 = CONCAT22(extraout_dx_04, pu_var1);
            if ((extraout_dx_04 | pu_var1) == 0) {
                break;
            }
            if ((pu_var1 + 0x100) != 0x8000002) {
                pass1_1038_3ba0(CONCAT22(extraout_dx_04, pu_var1));
            }
        }
    }
    return;
}

pub fn get_sys_metrics_1020_7c1a(param_1: &mut u16, param_2: u32) {
    let mut u_var1: u16;
    let i_var2: u16;
    let mut i_var3: i32;
    let mut u_var4: u16;
    let mut u_var5: u16;
    let mut local_10: u16;

  // u_var4 = (param_2  >> 0x10);
    u_var1 = (param_2 + 8);
  // u_var5 = (param_1  >> 0x10);
    i_var3 = param_1;
    unsafe {
        *param_1 = ctx.s_1_1050_389a;
    }
    (i_var3 + 2) = &ctx.PTR_LOOP_1050_1008;
    unsafe {
        *param_1 = (ctx.s_18_2_1050_3aa5 + 3);
    }
    (i_var3 + 2) = &ctx.PTR_LOOP_1050_1008;
    (i_var3 + 4) = u_var1;
    unsafe {
        *param_1 = ctx.s_0_020_1050_3ab0;
    }
    (i_var3 + 2) = &ctx.PTR_LOOP_1050_1008;
    (i_var3 + 6) = param_2;
    (i_var3 + 10) = 0;
    (i_var3 + 0xe) = 0;
    (i_var3 + 0x10) = 0;
    (i_var3 + 0x12) = 0;
    unsafe {
        *param_1 = 0x7f72;
    }
    (i_var3 + 2) = 0x1020;
    (i_var3 + 10) = (param_2 + 0xe4);
    i_var2 = winapi::GetSystemMetrics16(4);
    (i_var3 + 0xe) = i_var2;
    i_var2 = winapi::GetSystemMetrics16(5);
    (i_var3 + 0x10) = i_var2;
    i_var2 = winapi::GetSystemMetrics16(6);
    (i_var3 + 0x12) = i_var2;
    return;
}

pub fn cleanup_fn_1020_28fc(ctx: &mut AppContext, param_1: &mut Struct376) {
    param_1.ptr_a_lo = (ctx.s__P__P__P__P__P__P__P__P__P__P__P_1050_2e35 + 0x15);
    (param_1 + 2) = 0x1020;
    destroy_menu_func_1020_795c(param_1);
    return;
}
