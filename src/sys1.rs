use std::intrinsics::offset;

use crate::{defines::{
    AppContext, ATOM, COLORREF, HANDLE16, HBRUSH16, HCURSOR16, HDC16, HFILE16, HGDIOBJ16,
    HGLOBAL16, HICON16, HINSTANCE16, HMENU16, HPALETTE16, HPEN16, HTASK16, HWND16, LOGPALETTE,
    LPARAM, LRESULT, PAINTSTRUCT16, POINT16, RECT16, SEGPTR, WPARAM16,
}, exported_stub_1000_29dc, func_ptr_funcs::call_fn_ptr_1000_2594, mem_funcs::free_mem_1000_1b68, mixed_fn_1010_830a, sys2, util::SUB42, winapi_funcs};
use crate::app_context::AppContext;
use crate::bool_funcs::check_and_set_global_1000_1fea;
use crate::cleanup::{ret_1040_78de, win_cleanup_1010_305a, win_cleanup_func_1040_782c, window_msg_func_1010_7300};
use crate::constants::PF_ALPHA_BYTE_INSTRUCTIONS;
use crate::draw::draw2::win_fn_1020_7270;
use crate::err_funcs::error_check_1000_17ce;
use crate::exit::{exit_1000_25cc, exit_1000_2950, return_1000_39e1};
use crate::func_ptr_funcs::{call_fn_ptr_1000_24cd, call_fn_ptr_1000_256b};
use crate::mem_funcs::{alloc_mem_1000_07fc, get_fn_ptr_at_address, get_type_at_address, StructuredData};
use crate::other_funcs::{get_private_profile_str_1010_6132, switch_stmt_1008_aaa8, switch_stmt_1008_d818, write_private_profile_str_1010_622a};
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
use crate::prog_structs::prog_structs_12::Struct806;
use crate::prog_structs::prog_structs_15::{Struct1156, Struct921};
use crate::prog_structs::prog_structs_16::Struct493;
use crate::prog_structs::prog_structs_17::Struct1115;
use crate::prog_structs::prog_structs_19::Struct1162;
use crate::prog_structs::prog_structs_2::{Struct131, Struct199, Struct318, Struct7};
use crate::prog_structs::prog_structs_21::Struct74;
use crate::prog_structs::prog_structs_23::{Struct1090, Struct1157, Struct1160, Struct1163, Struct1164, Struct1165, Struct1166, win_struct_42};
use crate::prog_structs::prog_structs_24::{Struct, Struct103, Struct2111, Struct369};
use crate::prog_structs::prog_structs_25::{Struct152, Struct64, Struct65, Struct77};
use crate::prog_structs::prog_structs_26::{Struct1158, Struct1159, Struct824};
use crate::prog_structs::prog_structs_27::{Struct361, Struct816};
use crate::prog_structs::prog_structs_28::Struct300;
use crate::prog_structs::prog_structs_29::{Struct375, Struct807};
use crate::prog_structs::prog_structs_30::Struct359;
use crate::prog_structs::prog_structs_31::{Struct130, Struct348, Struct370};
use crate::prog_structs::prog_structs_7::{Struct1161, Struct376, Struct44};
use crate::prog_structs::prog_structs_8::Struct60;
use crate::prog_structs::prog_structs_9::Struct637;
use crate::sound_funcs::{mci_send_cmd_1008_5c5c, mci_send_command_1008_5c7c, mci_send_command_1008_5c9e};
use crate::string_ops1::{copy_string_1000_3d3e, get_string_index_1000_3da4, load_string_1008_b1f0, load_string_1010_847e, load_string_1010_84e0, pass1_1028_767e, process_string_1000_3cea, process_string_1000_475e, process_string_1000_55b1, str_fn_1010_5286, str_fn_1010_6034, string_fn_1000_3f9c};
use crate::struct_ops1::{init_struct_1000_1902, process_struct_1000_179c, process_struct_1000_2ce8, process_struct_1008_4772, process_struct_1008_574a, process_struct_1008_e586, process_struct_1010_1d48, set_struct_1008_0000};
use crate::struct_ops2::{pass1_1038_df86, process_struct_1010_20ba, process_struct_1040_7728};
use crate::sys_structs::{LOGPALETTE, PAINTSTRUCT16, POINT16, RECT16, WNDCLASS16};
use crate::typedefs::{ATOM, COLORREF, HANDLE16, HBRUSH16, HCURSOR16, HDC16, HFILE16, HGDIOBJ16, HGLOBAL16, HICON16, HINSTANCE16, HMENU16, HPALETTE16, HPEN16, HTASK16, HWND16, LPARAM, LRESULT, SEGPTR, WPARAM16};
use crate::ui_funcs::ui1::{destroy_win_1040_7b98, free_proc_inst_1040_911e, mixed_1040_8520, win_fn_1010_71d6, win_fn_1020_1294, win_fn_1040_8b92, win_gui_fn_1010_79aa};
use crate::ui_funcs::ui2::{destroy_menu_func_1020_795c, msg_box_1040_64ca, pass1_1038_af40, pass1_1038_e03e, send_dialog_item_msg_1038_844a, send_dlg_item_msg_1038_8164, set_cursor_1038_bc30, win_fn_1038_8f74};
use crate::util::{CARRY1, CARRY2, CONCAT11, CONCAT12, CONCAT13, CONCAT22, CONCAT31, POPCOUNT, SBORROW1, SBORROW2, SUB21, SUB41, ZEXT24};
use crate::winapi_funcs::{CreateWindowEx16, GetClassInfo16, GetDOSEnviornment16, RegisterClass16, WritePrivateProfileString16};

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
    let mut i_var4 = winapi_funcs::GetModuleFileName16(&ctx.g_h_instance, u_stack12, u_stack12.len());
    // u_var14._2_2_ = (u_var14 >> 0x10);
    ctx.PTR_LOOP_1050_5fc2 = u_var14.clone();
    // param_1 = (u_var13 >> 0x10);
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

pub unsafe fn get_dos_env_1000_27d6(ctx: &mut AppContext) {
    let pi_var1: i32;
    let pc_var2: String;
    let pi_var3: i32;
    let mut c_var4: char;
    let mut ppcVar5: String;
    let mut i32_var6: i32;
    let mut i_var7: i32;
    let mut pi_var8: i32;
    let mut pi_var9: i32;
    let mut pc_var10: String;
    let mut pi_var11: i32;
    let mut b_var12: bool;
    let mut u_var13: String;
    let mut u_var14: String;
    let mut pc_var15: String;
    let mut u_var16 = &ctx.g_alloc_addr_1050_1050;
    let mut dos_env = GetDOSEnviornment16();
    // dos_env._2_2_ = (dos_env >> 0x10);
    // if dos_env != 0 {
    //     dos_env._2_2_ = 0;
    // }
    let mut i_var7 = 0;
    let mut pc_var10 = String::new();
    let mut i32_var6 = -1;
    if dos_env != 0 {
        c_var4 = '\0';
        while c_var4 != '\0' {
            while {
                if i32_var6 == 0 {
                    break;
                }
                i32_var6 = i32_var6 - 1;
                pc_var2 = pc_var10;
                pc_var10 = pc_var10[1..].clone();
                pc_var2[0] != '\0'
            } {}
            i_var7 = i_var7 + 1;
            pc_var2 = pc_var10;
            pc_var10 = pc_var10[1..].clone();
            c_var4 = pc_var2[0];
        }
    }
    u_var13 = exit_1000_2950(ctx, u_var16);
    u_var14 = exit_1000_2950(ctx, None);
    // pc_var15 = (u_var13 >> 0x10);
    pc_var10 = u_var13;
    // uVar16 = (u_var14 >> 0x10);
    ctx.PTR_LOOP_1050_5fbe = u_var14;
    pi_var8 = 0x0;
    loop {
        ctx.PTR_LOOP_1050_5fc0 = u_var14.clone();
        ppcVar5 = u_var14.clone();
        if i_var7 == 0 {
            ppcVar5[0] = 0x0;
            ppcVar5[1] = 0x0;
            return;
        }
        b_var12 = pi_var8 == ctx.s__C_FILE_INFO__1050_5f5c[0];
        if b_var12 {
            pi_var11 = ctx.s__C_FILE_INFO__1050_5f5c;
            i32_var6 = 6;
            pi_var9 = pi_var8;
            while {
                if i32_var6 == 0 {
                    break;
                }
                i32_var6 = i32_var6 - 1;
                pi_var3 = pi_var11;
                pi_var11 = pi_var11 + 1;
                pi_var1 = pi_var9;
                pi_var9 = pi_var9 + 1;
                b_var12 = *pi_var1 == *pi_var3;
                b_var12
            } {}
            if !b_var12 {}
            // goto LAB_1000_2867;
        } else {
            // LAB_1000_2867:
            ppcVar5[0] = pc_var10;
            ppcVar5[1] = pc_var15.clone();
            // u_var14 = CONCAT22(ctx.PTR_LOOP_1050_5fc0, ppcVar5 + 2);
        }
        while {
            ctx.PTR_LOOP_1050_5fc0 = u_var14.clone();
            pi_var1 = pi_var8;
            pi_var8 = (pi_var8 + 1);
            c_var4 = *pi_var1;
            pc_var2 = pc_var10.clone();
            pc_var10 = pc_var10[1..].clone();
            pc_var2[0] = c_var4;
            // u_var14 = (u_var14 & 0xffff) | ctx.PTR_LOOP_1050_5fc0;
            c_var4 != '\0'
        } {}
        i_var7 = i_var7 - 1;
    }
}

pub unsafe fn get_dos_env_1000_27da() {
    let pi_var1: i32;
    let pc_var2: String;
    let pi_var3: i32;
    let mut c_var4: char;
    let mut i_var5: i32;
    let mut i32_var6: i32;
    let mut u_var7: u16;
    char * *ppc_var8;
    let mut i_var9: i32;
    let pi_var10: i32;
    let pi_var11: i32;
    let pc_var12: String;
    let pi_var13: i32;
    let mut b_var14: bool;
    let s_var15: SEGPTR;
    let mut u_var16: String;
    let mut pc_var17: String = String::new();

    s_var15 = GetDOSEnviornment16();
    // i32_var6 = (s_var15 >> 0x10);
    // if s_var15 != 0 {
    //     i32_var6 = 0;
    // }
    i_var9 = 0;
    pc_var12 = String::new();
    i_var5 = -1;
    if i32_var6 != 0 {
        c_var4 = *0x0;
        while c_var4 != '\0' {
            while {
                if i_var5 == 0 {
                    break;
                }
                i_var5 = i_var5 + -1;
                pc_var2 = pc_var12;
                pc_var12 = pc_var12[1..].clone();
                pc_var2[0] != '\0'
            } {}
            i_var9 = i_var9 + 1;
            pc_var2 = pc_var12;
            pc_var12 = pc_var12[1..].clone();
            unsafe {
                c_var4 = pc_var2[0];
            }
        }
    }
    u_var16 = exit_1000_2950(ctx, None);
    // pc_var17 = (u_var16 >> 0x10);
    pc_var12 = u_var16;
    u_var16 = exit_1000_2950(ctx, None);
    // u_var7 = (u_var16 >> 0x10);
    ppc_var8 = u_var16;
    // 0x5fbe = ppc_var8;
    // ctx.PTR_LOOP_1050_5fc0 = u_var7;
    pi_var10 = 0x0;
    loop {
        if i_var9 == 0 {
            ppc_var8[0] = 0x0;
            ppc_var8[1] = 0x0;
            return;
        }
        unsafe {
            b_var14 = pi_var10[0] == ctx.s__C_FILE_INFO__1050_5f5c;
        }
        if b_var14 {
            pi_var13 = ctx.s__C_FILE_INFO__1050_5f5c;
            i_var5 = 6;
            pi_var11 = pi_var10;
            while {
                if i_var5 == 0 {
                    break;
                }
                i_var5 = i_var5 + -1;
                pi_var3 = pi_var13;
                pi_var13 = pi_var13 + 1;
                pi_var1 = pi_var11;
                pi_var11 = pi_var11 + 1;
                unsafe {
                    b_var14 = *pi_var1 == *pi_var3;
                }
                b_var14
            } {}
            if !b_var14 {}
            // goto LAB_1000_2867;
        } else {
            // LAB_1000_2867:
            ppc_var8 = pc_var12[0];
            ppc_var8[1] = u_var16;
            ppc_var8 = ppc_var8 + 2;
        }
        while {
            pi_var1 = pi_var10;
            pi_var10 = (pi_var10 + 1);

                c_var4 = *pi_var1;

            pc_var2 = pc_var12;
            pc_var12 = pc_var12[1..].clone();

                pc_var2[0] = c_var4;
            c_var4 != '\0'
        } {}
        i_var9 = i_var9 + -1;
    }
}

pub unsafe fn dos3_call_1000_2bb6(ctx: &mut AppContext, param_1: &String, param_2: &mut Struct152) -> u32 {
    let mut pu8_var1: &Vec<u8>;
    let mut struct_var2: Struct152;
    let mut u8_var3: u8;
    let mut i_var4: &String;
    let mut i_var5: &String;
    let mut str_var6: &String;
    let mut u16_var7: &Vec<u8>;
    let mut i32_var8: i32;

    // struct_var2 = param_2;
    i32_var8 = unaff_bp + 1;
    u16_var7 = &ctx.g_alloc_addr_1050_1050;
    u8_var3 = param_2.field_0xa;
    if ((u8_var3 & 0x82) != 0) && ((u8_var3 & 0x40) == 0) {
        param_2.field_0x4 = 0;
        if (u8_var3 & 1) != 0 {
            if (u8_var3 & 0x10) == 0 {}
            // goto LAB_1000_2c37;
            param_2.field_0x0 = param_2.field_0x6.clone();
            u8_var3 = u8_var3 & 0xfe;
        }
        param_2.field_0xa = u8_var3 & 0xef | 2;
        str_var6 = &param_2.field_0xb;
        if ((u8_var3 & 8) == 0)
            && ((u8_var3 & 4) != 0
                || ((&param_2.field_0xf0 & 1) == 0
                    && (PTR_LOOP_1050_61ec != 0x0
                        && ((param_2 == 0x621c || (param_2 == 0x6228))
                            && ((*(str_var6 + 0x5f90) & 0x40) != 0))
            //             || (
            // process_struct_1000_2ce8(param_2, ctx.dx_reg),
            // (struct_var2.field_0xa & 8) == 0,
            //             )
        )))
        {
            i_var4 = dos3_call_1000_39f2(
                &mut str_var6,
                &param_1,
                None
            );
            i_var5 = 1;
        } else {
            i_var4 = &param_2.field_0x6;
            // i_var5 = &param_2.field_0x0 - i_var4;
            param_2.field_0x0 = i_var4[1..].clone();
            param_2.field_0x4 = param_2.field_0xf2  - 1;
            if i_var5 == 0 {
                // i_var4 = 0;
                if (*(str_var6 + 0x5f90) & 0x20) != 0 {
                    dos3_call_1000_3636(&str_var6, 0, 0, 2);
                    // i_var4 = 0;
                    i_var5 = i_var4;
                }
            } else {
                i_var4 = dos3_call_1000_39f2(&mut str_var6, &struct_var2.field_0x6, Some(&mut struct_var2.field_0x8), i_var5);
            }
           struct_var2.field_0x6 = param_1.clone();
        }
        if i_var4 == i_var5 {
            return param_1 & 0xff;
        }
    }
    // LAB_1000_2c37:
    pu8_var1 = &struct_var2.field_0xa;
    pu8_var1[0] = pu8_var1[0] | 0x20;
    return 0xffff;
}

pub unsafe fn dos3_call_1000_35fe(ctx: &mut AppContext, param_1: i32) -> u16 {
    let pc_var1: code;
    let mut u_var2: u16;
    let mut u8_var3: bool;

    if param_1 < u16_1050_5f8a {
        u8_var3 = false;
        pc_var1 = winapi_funcs::swi(0x21);
        unsafe {
            u_var2 = (*pc_var1)(ctx.bp_reg + 1);
        }
        if !u8_var3 {
            *(param_1 + 0x5f90) = 0;
        }
    } else {
        u_var2 = 0x900;
        u8_var3 = true;
    }
    if u8_var3 {
        pass1_fn_1000_29b5(u_var2);
        return 0xffff;
    }
    return 0;
}

pub unsafe fn dos3_call_1000_3636(param_1: &String, param_2: i32, uparam_3: i32, param_4: i32) {
    let pu8_var1: Vec<u8>;
    let pc_var2: code;
    let mut u_var3: u16;
    let mut unaff_bp: i32;
    let mut u8_var4: bool;
    let mut u_var5: u32;
    let mut local_8: u16;
    let mut local_6: u16;

    if (((param_1 < u16_1050_5f8a) || (PTR_LOOP_1050_61ec == 0x0)) || (2 < param_1)) {
        if ((PTR_LOOP_1050_6064 == 0x0) || ((param_3 & 0x8000) == 0)) {}
        // goto LAB_1000_36e3;
        if (param_4 == 0) {}
        // goto LAB_1000_369b;
        u8_var4 = false;
        pc_var2 = winapi_funcs::swi(0x21);
        unsafe {
            u_var5 = (*pc_var2)();
        }
        u_var3 = u_var5;
        if (u8_var4) {}
        // goto LAB_1000_299d;
        if ((param_4 & 2) == 0) {
            if (-1 < ((u_var5 >> 0x10) + param_3 + CARRY2(u_var3, param_2))) {
                // LAB_1000_36e3:
                u8_var4 = false;
                pc_var2 = winapi_funcs::swi(0x21);
                unsafe {
                    u_var3 = (*pc_var2)();
                }
                if (!u8_var4) {
                    pu8_var1 = (param_1 + 0x5f90);
                    u8_var4 = false;
                    unsafe {
                        *pu8_var1 = *pu8_var1 & 0xfd;
                    }
                }
                // goto LAB_1000_299d;
            }
        } else {
            pc_var2 = winapi_funcs::swi(0x21);
            unsafe {
                u_var5 = (*pc_var2)(unaff_bp + 1);
            }
            if (-1 < ((u_var5 >> 0x10) + param_3 + CARRY2(u_var5, param_2))) {}
            // goto LAB_1000_36e3;
            pc_var2 = winapi_funcs::swi(0x21);
            unsafe {
                (*pc_var2)();
            }
        }
        // LAB_1000_369b:
        u_var3 = s_471_bmp_1050_1600;
    } else {
        u_var3 = 0x900;
    }
    u8_var4 = true;
    // LAB_1000_299d:
    if (u8_var4) {
        pass1_fn_1000_29b5(u_var3);
    }
    return;
}

pub unsafe fn dos3_call_1000_370a(
    param_1: u16,
    param_2: u16,
    param_3: i32,
    param_4: u8,
    param_5: i32,
) -> u16 {
    let pc_var1: code;
    let mut u_var2: i32;
    let mut u_var3: u16;
    let mut i_var4: i32;
    let mut b_var5: u8;
    let mut u_var6: i32;
    
    let mut u_var7: u16;
    let mut unaff_bp: i32;
    let mut u_var8: i32;
    let mut bVar9: bool;
    let mut in_stack_0000fff2: i32;
    let local_8: u8;
    let local_7: u8;
    let local_6: u8;
    let mut local_5: u8;

    _param_3 = param_4;
    u_var2 = _local_6 & 0xff00;
    _local_6 = u_var2 | param_3;
    b_var5 = 0;
    if (((param_2 & 0x8000) == 0) && ((param_2 & 0x4000) != 0 || ((u8_1050_6061 & 0x80) == 0))) {
        b_var5 = 0x80;
    }
    bVar9 = false;
    pc_var1 = winapi_funcs::swi(0x21);
    u_var6 = param_2;
    unsafe {
        u_var3 = (*pc_var1)();
    }
    if (bVar9) {
        if ((u_var3 == 2) && ((u_var6 & 0x100) != 0)) {
            _local_8 = b_var5;
            _local_6 = (s_____s__lu_1050_38cd + 3);
            return_1000_39e1();
            u_var6 = 0;
            _param_3 = param_4;
            // LAB_1000_38e3:
            bVar9 = false;
            pc_var1 = winapi_funcs::swi(0x21);
            unsafe {
                u_var3 = (*pc_var1)();
            }
            if (bVar9) {}
            // goto LAB_1000_299d;
            if ((local_6 != '\0')
                || (
                    u_var7 = u_var3,
                    u_var8 = in_stack_0000fff2,
                    (param_2 & 2) == 0,
                ))
            {
                pc_var1 = winapi_funcs::swi(0x21);
                unsafe {
                    (*pc_var1)();
                }
                bVar9 = false;
                pc_var1 = winapi_funcs::swi(0x21);
                unsafe {
                    u_var3 = (*pc_var1)();
                }
                if (bVar9) {}
                // goto LAB_1000_299d;
                u_var7 = u_var3;
                u_var8 = _local_6;
                if (((_local_8 & 0x100) == 0) && ((_param_3 & 1) != 0)) {
                    u_var6 = (u_var6 | 1);
                    bVar9 = false;
                    pc_var1 = winapi_funcs::swi(0x21);
                    unsafe {
                        u_var3 = (*pc_var1)();
                    }
                    u_var8 = unaff_bp + 1;
                    if (bVar9) {}
                    // goto LAB_1000_299d;
                }
            }
            // LAB_1000_3973:
            if ((_local_8 & 0x40) == 0) {
                pc_var1 = winapi_funcs::swi(0x21);
                unsafe {
                    (*pc_var1)();
                }
                b_var5 = 0;
                if ((u_var6 & 1) != 0) {
                    b_var5 = 0x10;
                }
                if ((param_2 & 8) != 0) {
                    b_var5 = b_var5 | 0x20;
                }
            } else {
                b_var5 = 0;
            }
            if (u_var7 < &u16_1050_5f8a) {
                *(u_var7 + 0x5f90) = b_var5 | local_8 | 1;
                return u_var7;
            }
            pc_var1 = winapi_funcs::swi(0x21);
            unsafe {
                (*pc_var1)();
            }
            u_var3 = 0x1800;
        }
    } else {
        if ((u_var6 & 0x500) != 0x500) {
            _local_8 = CONCAT11(1, b_var5);
            pc_var1 = winapi_funcs::swi(0x21);
            unsafe {
                (*pc_var1)();
            }
            if ((ctx.dx_reg & 0x80) != 0) {
                _local_8 = _local_8 | 0x40;
            }
            u_var7 = u_var3;
            u_var8 = _local_6;
            if ((_local_8 & 0x40) == 0) {
                if ((param_2 & 0x200) == 0) {
                    if (((_local_8 & 0x80) != 0) && ((param_2 & 2) != 0)) {
                        pc_var1 = winapi_funcs::swi(0x21);
                        unsafe {
                            (*pc_var1)();
                        }
                        pc_var1 = winapi_funcs::swi(0x21);
                        unsafe {
                            i_var4 = (*pc_var1)();
                        }
                        if ((i_var4 != 0) && (local_5 = (u_var2 >> 8), local_5 == '\x1a')) {
                            pc_var1 = winapi_funcs::swi(0x21);
                            unsafe {
                                (*pc_var1)(unaff_bp + 1);
                            }
                            pc_var1 = winapi_funcs::swi(0x21);
                            unsafe {
                                (*pc_var1)();
                            }
                        }
                        u_var6 = 0;
                        pc_var1 = winapi_funcs::swi(0x21);
                        unsafe {
                            (*pc_var1)();
                        }
                        u_var7 = u_var3;
                        u_var8 = in_stack_0000fff2;
                    }
                } else {
                    if ((param_2 & 3) == 0) {
                        unsafe {
                            pc_var1 = winapi_funcs::swi(0x21);
                            (*pc_var1)();
                            pc_var1 = winapi_funcs::swi(0x21);
                            (*pc_var1)();
                        }
                        // goto LAB_1000_38e3;
                    }
                    u_var6 = 0;
                    pc_var1 = winapi_funcs::swi(0x21);
                    unsafe {
                        (*pc_var1)();
                    }
                    u_var7 = u_var3;
                }
            }
            // goto LAB_1000_3973;
        }
        pc_var1 = winapi_funcs::swi(0x21);
        unsafe {
            (*pc_var1)();
        }
        u_var3 = 0x1100;
    }
    bVar9 = true;
    // LAB_1000_299d:
    if (bVar9) {
        pass1_fn_1000_29b5(u_var3);
        u_var3 = 0xffff;
    }
    return u_var3;
}

pub unsafe fn dos3_call_1000_39f2(param_1: &mut String, param_2: &String, param_3: Option<&mut u16>) -> u16 {
    let pc_var1: String;
    let mut u_var2: i32;
    let pcVar3: code;
    let mut u_var4: u16;
    let mut cVar5: u8;
    let mut u_var6: u16;
    let pcVar7: String;
    let mut u_var8: i32;
    let pc_var9: String;
    let mut i_var10: i32;
    let pu_var11: u16;
    let mut unaff_bp: i32;
    let mut unaff_si: u16;
    let pc_var12: String;
    let pc_var13: String;
    let mut u_var14: u16;
    let mut unaff_cs: u16;
    let mut unaff_ss: u16;
    let uVar15: u8;
    let mut bVar16: u8;
    let mut in_af: u8;
    let mut bVar17: bool;
    let mut cVar18: u8;
    let mut cVar19: u8;
    let mut u_var20: u32;
    let mut uStack18: u16;
    let mut uStack16: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut uStack4: u16;
    let mut iStack2: i32;

    iStack2 = unaff_bp + 1;
    uStack4 = SUB42(&ctx.g_alloc_addr_1050_1050, 0);
    local_e = u16_1050_5f8a;
    pcVar7 = u16_1050_5f8a;
    if ((PTR_LOOP_1050_61ec != 0x0)
        && (
            pcVar7 = PTR_s_ed_in_Op_Op_1050_0028_1050_5f8e,
            param_1 < (&dos_alloc_addr_1050_0002 + 1),
        ))
    {
        *param_1 = u16_1050_5f8a;
    }
    if pcVar7 <= param_1 {
        uVar15 = true;
        u_var6 = 0x900;
        // goto LAB_1000_299d;
    }
    pcVar7 = param_1;
    if (param_1[0x5f90] & 0x20) != 0 {
        uVar15 = false;
        pcVar3 = winapi_funcs::swi(0x21);
        unsafe {
            u_var6 = (*pcVar3)();
        }
        unaff_cs = 0x1000;
        if (uVar15) {}
        // goto LAB_1000_299d;
    }
    pc_var12 = param_2;
    if ((pcVar7[0x5f90] & 0x80) == 0) {
        // LAB_1000_3acf:
        uVar15 = false;
        u_var6 = param_3;
        if (param_3 != 0) {
            local_c = &ctx.g_alloc_addr_1050_1050;
            uVar15 = pcVar7 < local_e;
            if (uVar15) {
                uVar15 = 0;
                pcVar3 = winapi_funcs::swi(0x21);
                unsafe {
                    u_var20 = (*pcVar3)();
                }
            } else {
                local_e = s_2_3_1050_3b71;
                u_var20 = process_string_1000_55b1(ctx);
            }
            u_var6 = u_var20;
            local_a = param_2._2_2_;
            if (uVar15) {
                u_var6 = CONCAT11(9, u_var20);
            } else {
                uVar15 = false;
                if (u_var6 == 0) {
                    if (((pcVar7[0x5f90] & 0x40) == 0) || (*(u_var20 >> 0x10) != '\x1a')) {
                        uVar15 = true;
                        u_var6 = 0x1c00;
                    } else {
                        uVar15 = false;
                    }
                }
            }
        }
    } else {
        local_a = &ctx.g_alloc_addr_1050_1050;
        bVar17 = true;
        local_6 = 0;
        local_8 = 0;
        local_c = &local_e;
        u_var6 = param_3;
        pc_var13 = pc_var12;
        local_e = unaff_si;
        if (param_3 != 0) {
            while {
                if (u_var6 == 0) {
                    break;
                }
                u_var6 = u_var6 - 1;
                pc_var1 = pc_var13;
                pc_var13 = pc_var13 + 1;
                unsafe {
                    bVar17 = *pc_var1 == '\n';
                }
                !bVar17
            } {}
            if (!bVar17) {}
            // goto LAB_1000_3acf;
            pc_var9 = pc_var12;
            uStack18 = unaff_cs;
            uStack16 = param_2._2_2_;
            u_var8 = pass1_fn_1000_3bac();
            u_var4 = uStack16;
            if (u_var8 < 0xa9) {
                uStack18 = unaff_cs;
                uStack18 = bad_1000_25f2();
                if (pc_var13 == pc_var9) {
                    return local_e;
                }
                //TODO
                //bVar16 = param_1 < local_e;
                pc_var12 = param_1 + -local_e;
                cVar19 = pc_var12 < 0;
                cVar18 = pc_var12 == 0x0;
                cVar5 = (POPCOUNT(pc_var12 & 0xff) & 1) == 0;
                if (bVar16) {
                    bVar16 = 0;
                    cVar19 = '\0';
                    cVar18 = 0x1;
                    cVar5 = 0x1;
                    pcVar3 = winapi_funcs::swi(0x21);
                    unsafe {
                        u_var8 = (*pcVar3)(&ctx.g_alloc_addr_1050_1050, u_var6, pcVar7);
                    }
                } else {
                    u_var8 = process_string_1000_55b1(ctx);
                }
                if (!bVar16) {
                    local_6 = local_6 + u_var8;
                    bVar16 = u_var6 < u_var8;
                    u_var2 = u_var6 - u_var8;
                    cVar19 = u_var2 < 0;
                    cVar18 = u_var2 == 0;
                    cVar5 = (POPCOUNT(u_var2 & 0xff) & 1) == 0;
                    if (bVar16 || cVar18) {
                        return local_e;
                    }
                }
                u_var2 = (cVar19 << 7 | cVar18 << 6 | in_af << 4 | cVar5 << 2 | 2 | bVar16) << 8;
                u_var6 = u_var8 & 0xff | u_var2;
                if (local_6 == 0) {
                    uVar15 = (u_var2 & 0x100) != 0;
                    if (uVar15) {
                        u_var6 = CONCAT11(9, (u_var8 & 0xff));
                    } else {
                        if (((param_1[0x5f90] & 0x40) == 0) || (unsafe { *param_2 != '\x1a' })) {
                            uVar15 = true;
                            u_var6 = 0x1c00;
                        } else {
                            uVar15 = false;
                        }
                    }
                    // goto LAB_1000_299d;
                }
            } else {
                pu_var11 = &uStack18 + 1;
                i_var10 = 0x200;
                if (u_var8 < 0x228) {
                    i_var10 = 0x80;
                }
                i_var10 = -i_var10;
                pcVar7 = (&uStack18 + i_var10 + 2);
                (&uStack18 + i_var10) = unaff_ss;
                u_var14 = (&uStack18 + i_var10);
                while {
                    pc_var1 = pc_var12;
                    pc_var12 = pc_var12 + 1;
                    unsafe {
                        cVar5 = *pc_var1;
                    }
                    if (cVar5 == '\n') {
                        cVar5 = '\r';
                        if (pcVar7 == pu_var11) {
                            (&uStack18 + i_var10) = (s_5_24_1050_3ab9 + 4);
                            cVar5 = dos3_call_1000_3ad9(0);
                        }
                        pc_var1 = pcVar7;
                        pcVar7 = pcVar7 + 1;
                        unsafe {
                            *pc_var1 = cVar5;
                        }
                        cVar5 = '\n';
                        local_8 = local_8 + 1;
                    }
                    if (pcVar7 == pu_var11) {
                        (&uStack18 + i_var10) = s_94_72_1050_3ac8;
                        cVar5 = dos3_call_1000_3ad9(0);
                    }
                    pc_var1 = pcVar7;
                    pcVar7 = pcVar7 + 1;
                    unsafe {
                        *pc_var1 = cVar5;
                    }
                    *param_3 = *param_3 - 1;
                    *param_3 != 0
                } {}
                (&uStack18 + i_var10) = (ctx.s_0_020_1050_3ab0 + 1);
                dos3_call_1000_3ad9(0);
            }
        }
        uVar15 = local_6 < local_8;
        u_var6 = local_6 - local_8;
    }
    // LAB_1000_299d:
    if (uVar15) {
        local_c = s_fem102_wav_1050_29a2;
        pass1_fn_1000_29b5(u_var6);
        u_var6 = 0xffff;
    }
    return u_var6;
}

// WARNING: Unable to track spacebase fully for stack
// WARNING: Removing unreachable block (ram,0x10003afe)

pub unsafe fn dos3_call_1000_3ad9(param_1: u16) -> u16 {
    let pu_var1: u32;
    let pi_var2: i32;
    let mut i_var3: i32;
    let mut u_var4: i32;
    let pcVar5: code;
    let mut u_var6: u16;
    let mut u_var7: i32;
    let mut in_cx: i32;
    let mut u_var8: i32;
    let mut unaff_bp: i32;
    let mut unaff_DI: i32;
    let mut unaff_ss: u16;
    let mut bVar9: u8;
    let mut bVar10: bool;
    let mut cVar11: u8;
    let mut in_af: u8;
    let mut cVar12: u8;
    let mut cVar13: u8;

    if (unaff_DI == ctx.dx_reg) {
        return param_1;
    }
    u_var8 = (unaff_bp + 6);
    pu_var1 = (unaff_bp + -0xc);
    unsafe {
        bVar9 = u_var8 < *pu_var1;
        u_var7 = u_var8 - *pu_var1;
    }
    cVar13 = u_var7 < 0;
    cVar12 = u_var7 == 0;
    cVar11 = (POPCOUNT(u_var7 & 0xff) & 1) == 0;
    if (bVar9) {
        bVar9 = 0;
        cVar13 = '\0';
        cVar12 = 0x1;
        cVar11 = 0x1;
        pcVar5 = winapi_funcs::swi(0x21);
        unsafe {
            u_var7 = (*pcVar5)(&ctx.g_alloc_addr_1050_1050);
        }
    } else {
        u_var7 = process_string_1000_55b1(ctx);
    }
    if (!bVar9) {
        pi_var2 = (unaff_bp + -4);
        unsafe {
            *pi_var2 = *pi_var2 + u_var7;
        }
        bVar9 = in_cx < u_var7;
        u_var4 = in_cx - u_var7;
        cVar13 = u_var4 < 0;
        cVar12 = u_var4 == 0;
        cVar11 = (POPCOUNT(u_var4 & 0xff) & 1) == 0;
        if (bVar9 || cVar12) {
            return param_1;
        }
    }
    u_var4 = (cVar13 << 7 | cVar12 << 6 | in_af << 4 | cVar11 << 2 | 2 | bVar9) << 8;
    u_var6 = u_var7 & 0xff | u_var4;
    if ((unaff_bp + -4) == 0) {
        bVar10 = (u_var4 & 0x100) != 0;
        if (bVar10) {
            u_var6 = CONCAT11(9, (u_var7 & 0xff));
        } else {
            if (((*(u_var8 + 0x5f90) & 0x40) == 0) || (**(unaff_bp + 8) != '\x1a')) {
                bVar10 = true;
                u_var6 = 0x1c00;
            } else {
                bVar10 = false;
            }
        }
    } else {
        u_var8 = (unaff_bp + -4);
        pu_var1 = (unaff_bp + -6);
        unsafe {
            bVar10 = u_var8 < *pu_var1;
            u_var6 = u_var8 - *pu_var1;
        }
    }
    i_var3 = (unaff_bp + -10);
    if bVar10 {
        (i_var3 + 2) = s_fem102_wav_1050_29a2;
        // *(i_var3 + 2)
        pass1_fn_1000_29b5(u_var6, );
        u_var6 = 0xffff;
    }
    return u_var6;
}

pub unsafe fn dos3_call_1000_42de(param_1: u32, param_2: &mut u16, param_3: &mut u16) {
    let mut u_var1: u16;
    let mut u_var2: u16;
    let pcVar3: code;
    let mut u_var4: u16;
    let mut u_var5: u16;
    let mut unaff_bp: i32;
    let mut i32_var6: i32;
    let mut unaff_si: u16;
    let mut i_var7: i32;
    let mut u_var8: u16;
    let mut u_var9: u16;
    let mut u_var10: u16;
    let mut bVar11: bool;
    let mut u_var12: u32;
    let mut u_var13: u16;

    i32_var6 = unaff_bp + 1;
    u_var13 = SUB42(&ctx.g_alloc_addr_1050_1050, 0);
    u_var8 = (param_1 >> 0x10);
    i_var7 = param_1;
    u_var5 = (i_var7 + 2);
    u_var4 = (i_var7 + 4);
    u_var1 = (i_var7 + 8);
    u_var8 = (i_var7 + 10);
    u_var9 = (param_3 >> 0x10);
    unsafe {
        u_var10 = *param_3;
    }
    u_var2 = (param_3 + 6);
    bVar11 = false;
    pcVar3 = winapi_funcs::swi(0x21);
    unsafe {
        u_var12 = (*pcVar3)();
    }
    unsafe {
        *param_3 = u_var10;
    }
    (param_3 + 6) = u_var2;
    u_var10 = (param_2 >> 0x10);
    i_var7 = param_2;
    unsafe {
        *param_2 = u_var12;
    }
    (i_var7 + 2) = u_var5;
    (i_var7 + 4) = u_var4;
    (i_var7 + 6) = (u_var12 >> 0x10);
    (i_var7 + 8) = u_var1;
    (i_var7 + 10) = u_var8;
    if bVar11 {
        pass1_fn_1000_29af(ctx.si_reg, u_var13, i32_var6);
    }
    (i_var7 + 0xc) = bVar11;
    return;
}

pub unsafe fn dos3_call_1000_435c(ctx: &mut AppContext, param_1: Option<&mut u16>) {
    let pc_var1: code;
    let mut u_var2: u16;
    let mut in_cx: i32;
    let mut u_var3: i32;
    
    
    let mut u_var4: i32;
    let mut u_var5: i32;
    let mut cVar6: u8;

    pc_var1 = winapi_funcs::swi(0x21);
    unsafe {
        (*pc_var1)(&ctx.g_alloc_addr_1050_1050);
    }
    pc_var1 = winapi_funcs::swi(0x21);
    u_var3 = in_cx;
    u_var5 = ctx.dx_reg;
    unsafe {
        (*pc_var1)();
    }
    cVar6 = u_var3;
    pc_var1 = winapi_funcs::swi(0x21);
    unsafe {
        (*pc_var1)(u_var3 >> 8);
    }
    u_var4 = ctx.dx_reg;
    if ((u_var5 != ctx.dx_reg) && (u_var4 = ctx.dx_reg, cVar6 == '\x17')) {
        u_var3 = in_cx;
        u_var4 = u_var5;
    }
    u_var2 = pass1_fn_1000_462e(u_var3 - 0x7bc, u_var4 >> 8, u_var4 & 0xff);
    if (param_1._2_2_ != 0) {
        (param_1 + 2) = u_var4;
        unsafe {
            *param_1 = u_var2;
        }
    }
    return;
}

pub fn set_global_uint_1000_4d0c(ctx: &mut AppContext, param_1: Option<u32>) {
    if param_1.is_some() { ctx.UINT_1050_61e8 = param_1; }
    ctx.PTR_LOOP_1050_61ea = 0;
}

pub unsafe fn dos3_call_1000_4f2e() -> u16 {
    let pc_var1: code;
    let mut u_var2: u16;
    let mut unaff_bp: i32;
    let mut u8_var3: bool;

    u8_var3 = false;
    pc_var1 = winapi_funcs::swi(0x21);
    unsafe {
        u_var2 = (*pc_var1)(&ctx.g_alloc_addr_1050_1050, unaff_bp + 1);
    }
    if (u8_var3) {
        pass1_fn_1000_29b5(u_var2);
        return 0xffff;
    }
    return 0;
}

pub fn dos3_call_1000_4f94() -> i32 {
    let pc_var1: code;
    let mut b_var2: u8;
    let mut unaff_bp: i32;

    pc_var1 = winapi_funcs::swi(0x21);
    unsafe {
        b_var2 = (*pc_var1)(unaff_bp + 1);
    }
    return b_var2 + 1;
}

pub fn dos3_call_1000_4fbe(param_1: u8) -> u16 {
    let pc_var1: code;
    let mut c_var2: u8;
    let mut u_var3: u16;
    let mut unaff_bp: i32;

    pc_var1 = winapi_funcs::swi(0x21);
    unsafe {
        (*pc_var1)(unaff_bp + 1);
    }
    pc_var1 = winapi_funcs::swi(0x21);
    unsafe {
        c_var2 = (*pc_var1)();
    }
    u_var3 = 0xffff;
    if ((c_var2 + 0x1) == param_1) {
        u_var3 = 0;
    }
    return u_var3;
}

pub unsafe fn dos3_call_1000_514e() -> u16 {
    let pc_var1: code;
    let mut u_var2: u16;
    let mut unaff_bp: i32;
    let mut u8_var3: bool;

    u8_var3 = false;
    pc_var1 = winapi_funcs::swi(0x21);
    unsafe {
        u_var2 = (*pc_var1)(&ctx.g_alloc_addr_1050_1050, unaff_bp + 1);
    }
    if (u8_var3) {
        pass1_fn_1000_29b5(u_var2);
        return 0xffff;
    }
    return 0;
}

pub unsafe fn dos3_call_1000_5174() -> u32 {
    let pc_var1: code;
    let mut u_var2: u16;
    let mut unaff_bp: i32;
    let mut u8_var3: bool;

    u8_var3 = false;
    pc_var1 = winapi_funcs::swi(0x21);
    unsafe {
        u_var2 = (*pc_var1)(unaff_bp + 1);
    }
    if (!u8_var3) {
        return 0;
    }
    pass1_fn_1000_29b5(u_var2);
    return u_var2 & 0xff;
}

pub unsafe fn dos3_call_1000_51aa(param_1: &mut String, uparam_2_00: &String, param_2: u16) -> u32 {
    let pc_var1: code;

    pc_var1 = winapi_funcs::swi(0x21);
    unsafe {
        (*pc_var1)(&ctx.g_alloc_addr_1050_1050);
        pc_var1 = winapi_funcs::swi(0x21);
        (*pc_var1)();
        pc_var1 = winapi_funcs::swi(0x21);
        (*pc_var1)();
        pc_var1 = winapi_funcs::swi(0x21);
        (*pc_var1)();
    }
    if (param_2_00 & 0x100) == 0 {
        return 0;
    }
    pass1_fn_1000_29b5(param_2);
    return param_2 & 0xff;
}

pub unsafe fn dos3_call_1000_23ea(ctx: &mut AppContext, a: i16, b: u16) {
    let pu8_var1: Vec<u8>;
    let pu8_var2: Vec<u8>;
    let mut u8_var3: u8;
    let mut u8_var4: u8;
    let pu8_var5: Vec<u8>;
    let mut i32_var6: i32;
    let pfn_var7: fn();
    let pfn_var8: fn();
    let mut u_var9: u16;
    let mut i_var10: i32;
    let str_142: String;
    let pu8_var11: Vec<u8>;
    let pu8_var12: Vec<u8>;
    let mut u16_segment: u16;
    let mut b_var14: bool;
    let mut u16_var15: u16;
    let mut pfn_var16: u32;

    // DOS API
    pfn_var8 = winapi_funcs::swi(0x21);
    pfn_var8(ctx.bp_reg + 1);
    // DOS API
    pfn_var8 = winapi_funcs::swi(0x21);
    g_u16_ptr_1050_5f6a = b;
    ctx.PTR_LOOP_1050_5f6c = ctx.es_reg;
    pfn_var8();
    u16_segment = 0x1000;
    u16_var15 = u16_segment;
    u_var9 = exported_stub_1000_29dc();
    if &ctx.g_fn_ptr_1050_6202 != 0 {
        b_var14 = false;
        pfn_var7 = &ctx.g_fn_ptr_1050_6200;
        u16_var15 = u16_segment;
        pfn_var7();
        if b_var14 {
            exit_1000_25cc(ctx);
            return;
        }
        pfn_var16 = 0x6200;
        u16_var15 = u16_segment;
        pfn_var16();
    }
    i32_var6 = (ctx.s_New_failed_in_Op__Op_1050_0020 + 0xc);
    if i32_var6 != 0 {
        pu8_var11 = 0x0;
        while {
            b_var14 = *pu8_var11 == 0;
            if b_var14 {
                break;
            }
            i_var10 = 0xd;
            str_142 = ctx.s__C_FILE_INFO__1050_5f5c;
            while {
                if i_var10 == 0 {
                    break;
                }
                i_var10 = i_var10 + -1;
                pu8_var5 = pu8_var11;
                pu8_var11 = pu8_var11 + 1;
                pu8_var1 = str_142;
                str_142 = (str_142 + 1);
                b_var14 = *pu8_var1 == *pu8_var5;
                b_var14
            } {}
            if (b_var14) {
                pu8_var12 = 0x5f90;
                // goto LAB_1000_2495;
            }
            i_var10 = 0x7fff;
            b_var14 = true;
            while {
                if (i_var10 == 0) {
                    break;
                }
                i_var10 = i_var10 + -1;
                pu8_var1 = pu8_var11;
                pu8_var11 = pu8_var11 + 1;
                b_var14 = *pu8_var1 == 0;
                !b_var14
            } {}
            b_var14
        } {}
    }
    // LAB_1000_24a9:
    u16_var15 = ctx.s_266_bmp_1050_24ae + 4;
    call_fn_ptr_1000_2594();
    u16_var15 = ctx.s_264_bmp_1050_24b6 + 5;
    call_fn_ptr_1000_2594();
    u16_var15 = 0x24c4;
    call_fn_ptr_1000_2594();
    return;
    // LAB_1000_2495:
    pu8_var2 = pu8_var11 + 1;
    unsafe {
        u8_var3 = *pu8_var11;
    }
    if (u8_var3 < 0x41) {}
    // goto LAB_1000_24a9;
    pu8_var11 = pu8_var11 + 2;
    unsafe {
        u8_var4 = *pu8_var2;
    }
    if (u8_var4 < 0x41) {}
    // goto LAB_1000_24a9;
    pu8_var1 = pu8_var12;
    pu8_var12 = pu8_var12 + 1;
    unsafe {
        *pu8_var1 = u8_var4 + 0xbf | (u8_var3 + 0xbf) * '\x10';
    }
    // goto LAB_1000_2495;
}

pub fn dos_api_call_1000_24ff(ctx: &mut AppContext, dos_api_val: u16) {
    let pc_var1: code;
    let mut c_var2: u8;

    ctx.PTR_LOOP_1050_5fc9 = 1;
    c_var2 = 0x1;
    call_fn_ptr_1000_2594();
    call_fn_ptr_1000_2594();
    call_fn_ptr_1000_256b();
    if c_var2 == '\0' {
        pc_var1 = winapi_funcs::swi(0x21);
        unsafe {
            (*pc_var1)();
        }
    }
    return;
}

pub fn reg_class_1008_96d2(param_1: &mut Struct65, param_2: u16) {
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
    b_var1 = GetClassInfo16(&local_1c, CONCAT22(local_6, unaff_ss), param_1._2_2_);
    if (b_var1 == 0) {
        local_1c = (param_1 + 200);
        local_1a = 0x5632;
        local_18 = ctx.PTR_LOOP_1050_1008;
        local_16 = 0x40000;
        local_12 = ctx.g_h_instance_1050_038c;
        local_10 = (param_1 + 0xc2);
        local_e = (param_1 + 0xc4);
        local_c = (param_1 + 0xc6);
        local_a = 0;
        local_4 = param_1._2_2_;
        AVar2 = RegisterClass16(&local_1c);
        // TODO
        // if AVar2 == 0 {
        //     call_fn_ptr_1000_24cd(0);
        // }
    }
    return;
}

pub unsafe fn get_sys_metrics_1010_46f6(param_1: u32) {
    let mut u_var1: u16;
    let i_var2: u16;
    let i_var3: u16;
    let mut i_var4: i32;
    let mut u_var5: u16;
    let mut unaff_ss: u16;
    let ppVar6: Struct2111;
    let mut u_var7: u32;
    let puVar8: u16;
    let pu_var9: Vec<u8>;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    pu_var9 = CONCAT22(unaff_ss, &local_4);
    puVar8 = &local_6;
    ppVar6 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(puVar8, 0x48));
    pass1_1008_3e94((ppVar6 + 0xe), CONCAT22(unaff_ss, puVar8), pu_var9);
    u_var5 = (param_1 >> 0x10);
    i_var4 = param_1;
    u_var7 = process_struct_1008_4772((i_var4 + 0x66));
    u_var1 = (u_var7 >> 0x10);
    (i_var4 + 0x18) = local_4 + 8;
    (i_var4 + 0x1a) = local_6 + 9;
    i_var2 = winapi_funcs::GetSystemMetrics16(5);
    (i_var4 + 0x1c) = i_var2 * 2 + (u_var7 + 4);
    i_var2 = winapi_funcs::GetSystemMetrics16(4);
    i_var3 = winapi_funcs::GetSystemMetrics16(6);
    (i_var4 + 0x1e) = i_var3 + i_var2 + (u_var7 + 8);
    return;
}

pub unsafe fn get_sys_metrics_1018_09a8(param_1: u32) {
    let mut u_var1: u32;
    let i_var2: u16;
    let i_var3: u16;
    let mut i_var4: i32;
    let mut u_var5: u16;
    let mut unaff_ss: u16;
    let ppVar6: Struct2111;
    let pu_var7: u16;
    let puVar8: Vec<u8>;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    local_4 = winapi_funcs::GetSystemMetrics16(4);
    u_var5 = (param_1 >> 0x10);
    i_var4 = param_1;
    local_6 = (i_var4 + 0x12) - 2;
    puVar8 = CONCAT22(unaff_ss, &local_8);
    pu_var7 = &local_a;
    ppVar6 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(pu_var7, 0x48));
    pass1_1008_3e94((ppVar6 + 0xe), CONCAT22(unaff_ss, pu_var7), puVar8);
    (i_var4 + 0x18) = local_6 * local_4 + local_8 + 0x146;
    (i_var4 + 0x1a) = local_6 * local_4 + local_a + 9;
    i_var2 = winapi_funcs::GetSystemMetrics16(5);
    u_var1 = (i_var4 + 0x5a);
    (i_var4 + 0x1c) = i_var2 * 2 + (u_var1 + 4);
    i_var2 = winapi_funcs::GetSystemMetrics16(4);
    i_var3 = winapi_funcs::GetSystemMetrics16(6);
    u_var1 = (i_var4 + 0x5a);
    (i_var4 + 0x1e) = i_var3 + i_var2 + (u_var1 + 8);
    return;
}

pub fn get_sys_metrics_1018_1ea0(param_1: u32) {
    let i_var1: u16;
    let i_var2: u16;
    let mut i_var3: i32;
    let mut u_var4: u16;

    i_var1 = winapi_funcs::GetSystemMetrics16(5);
    u_var4 = (param_1 >> 0x10);
    i_var3 = param_1;
    (i_var3 + 0x2e) = i_var1 * 2 + (i_var3 + 0x36);
    i_var1 = winapi_funcs::GetSystemMetrics16(4);
    i_var2 = winapi_funcs::GetSystemMetrics16(6);
    (i_var3 + 0x30) = i_var1 + (i_var3 + 0x38) + i_var2;
    return;
}

pub unsafe fn get_sys_metrics_1018_2f56(param_1: u32) {
    let mut u_var1: u16;
    let i_var2: u16;
    let i_var3: u16;
    let mut i_var4: i32;
    let mut u_var5: u16;
    let mut unaff_ss: u16;
    let ppVar6: Struct2111;
    let mut u_var7: u32;
    let puVar8: u16;
    let pu_var9: Vec<u8>;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    pu_var9 = CONCAT22(unaff_ss, &local_4);
    puVar8 = &local_6;
    ppVar6 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(puVar8, 0x48));
    pass1_1008_3e94((ppVar6 + 0xe), CONCAT22(unaff_ss, puVar8), pu_var9);
    u_var5 = (param_1 >> 0x10);
    i_var4 = param_1;
    u_var7 = process_struct_1008_4772((i_var4 + 0x24));
    u_var1 = (u_var7 >> 0x10);
    (i_var4 + 0x18) = local_4 + 0xb5;
    (i_var4 + 0x1a) = local_6 + 9;
    i_var2 = winapi_funcs::GetSystemMetrics16(5);
    (i_var4 + 0x1c) = i_var2 * 2 + (u_var7 + 4);
    i_var2 = winapi_funcs::GetSystemMetrics16(4);
    i_var3 = winapi_funcs::GetSystemMetrics16(6);
    (i_var4 + 0x1e) = i_var3 + i_var2 + (u_var7 + 8);
    return;
}

pub fn get_sys_metrics_1018_4b1e(
    param_1: &mut Struct375,
    param_2: u16,
    param_3: u16,
) -> &mut Struct375 {
    let mut i_var1: i32;
    let mut u_var2: u16;

    process_struct_1010_1d48(param_1, param_3);
    u_var2 = (param_1 >> 0x10);
    i_var1 = param_1;
    (i_var1 + 0x12) = param_2;
    (i_var1 + 0x14) = 0;
    param_1.ptr_1_lo = &PTR_LOOP_1050_4c9e;
    (i_var1 + 2) = 0x1018;
    if (PTR_LOOP_1050_416c == 0x0) {
        PTR_LOOP_1050_416c = winapi_funcs::GetSystemMetrics16(4);
        PTR_LOOP_1050_416e = winapi_funcs::GetSystemMetrics16(5);
        PTR_LOOP_1050_4170 = winapi_funcs::GetSystemMetrics16(6);
    }
    return param_1;
}

pub unsafe fn call_win_proc_1040_a410(param_1: u16, param_2: u32, param_3: u32) {
    let win_proc: &mut Vec<u8>;
    let mut u_var1: i32;
    let ppc_var2: fn();
    let mut u_var3: u32;
    let mut u_var4: u16;
    let mut u_var5: u32;
    let mut local_a: u16;
    let mut local_6: u32;

    local_6 = 0;
    if (param_3 == 0x19) {
        ppc_var2 = (&PTR_LOOP_1050_5ee0 + 0x34);
        local_6 = ppc_var2();
        ctx.dx_reg = (local_6 >> 0x10);
    } else {
        if (param_3 == 0x86) {
            ppc_var2 = (&PTR_LOOP_1050_5ee0 + 0x20);
            u_var5 = ppc_var2();
            return u_var5;
        }
        if (param_3 == 0x110) {
            u_var3 = &PTR_LOOP_1050_5ee0;
            u_var5 = send_win_msg_1040_a308(u_var3, (u_var3 >> 0x10), param_1, param_2);
            return u_var5;
        }
    }
    if (local_6 != 0) {
        return local_6 & 0xffff | ctx.dx_reg << 0x10;
    }
    u_var3 = PTR_LOOP_1050_5bc8;
    // u_var4 = (u_var3 >> 0x10);
    local_bx_122 = u_var3;
    win_proc = local_bx_122.fn_ptr_0x4;
    u_var1 = local_bx_122.field_0x6;
    if ((u_var1 | win_proc) == 0) {
        return u_var1 << 0x10;
    }
    u_var5 = CallWindowProc16(
        CONCAT22(param_2, param_1),
        (param_2 >> 0x10),
        param_3,
        (param_3 >> 0x10),
        win_proc,
    );
    return u_var5;
}

pub unsafe fn send_win_msg_1040_a3d0(param_1: u32) {
    let mut u_var1: u32;
    let HVar2: HWND16;
    let mut i_var3: i32;
    let mut u_var4: u16;

    u_var4 = (param_1 >> 0x10);
    i_var3 = param_1;
    if ((i_var3 + 0x90) != 0) {
        u_var1 = (i_var3 + 0x90);
        (u_var1 + 0x14) = (i_var3 + 6);
        HVar2 = winapi_funcs::GetDlgItem16(0x1826, (i_var3 + 6));
        send_win_msg_1040_a308(param_1, 0, HVar2);
    }
    return;
}

pub unsafe fn send_win_msg_1040_a308(param_1: u32, param_2: u16, param_3: u16, param_4: HWND16) {
    let pu_var1: u16;
    let mut u_var2: u32;
    let LVar3: LPARAM;
    let mut i_var4: i32;
    let mut u_var5: u16;
    let ppVar6: Struct2111;
    let WVar7: WPARAM16;
    let mut u_var8: u16;
    let HVar9: HWND16;
    let mut in_stack_0000fff2: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;

    winapi_funcs::SendMessage16(0, 0, 0x405, param_2);
    winapi_funcs::SendMessage16(0, 0, 0xb, param_2);
    u_var5 = (param_1 >> 0x10);
    i_var4 = param_1;
    u_var2 = (i_var4 + 0x90);
    if ((u_var2 + 0x10) == 0) {
        WVar7 = 0;
        u_var8 = 0x401;
        HVar9 = param_2;
        LVar3 = load_string_1010_847e(
            ctx._g_struct_73_1050_14cc,
            (ctx._g_struct_73_1050_14cc >> 0x10),
            0x531,
        );
        winapi_funcs::SendMessage16(LVar3, WVar7, u_var8, HVar9);
    } else {
        ppVar6 =
            process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(in_stack_0000fff2, 3));
        local_c = 0;
        while (
            u_var2 = (i_var4 + 0x90),
            pu_var1 = (u_var2 + 0x10),
            unsafe { *pu_var1 != local_c && local_c <= *pu_var1 },
        ) {
            WVar7 = 0;
            u_var8 = 0x401;
            u_var2 = (i_var4 + 0x90);
            u_var2 = (u_var2 + 0xc);
            HVar9 = param_2;
            LVar3 = pass1_1010_ac92(ppVar6, (ppVar6 >> 0x10), (u_var2 + local_c * 2));
            winapi_funcs::SendMessage16(LVar3, WVar7, u_var8, HVar9);
            local_c = local_c + 1;
        }
    }
    winapi_funcs::SendMessage16(0, 1, 0xb, param_2);
    return 1;
}

pub unsafe fn win_fn_1020_0ec4(param_1: &mut u32, param_2: i32) {
    let mut u_var1: u32;
    let ppc_var2: fn();
    let mut cVar3: u8;
    let mut i_var4: i32;
    let mut i_var5: i32;
    let mut u_var6: i32;
    
    
    
    let mut u_var7: u16;
    let pp_var8: Struct2111;
    let w_param: WPARAM16;
    let h_wnd: HWND16;
    let mut u_var9: u16;
    let mut local_c: u16;
    let mut local_a: u16;

    u_var7 = (param_1 >> 0x10);
    i_var4 = param_1;
    if (param_2 == 0xfb) {
        pp_var8 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(local_c, 0x30));
        u_var9 = SUB42(pp_var8, 0);
        pass1_1010_375e(pp_var8);
        unsafe {
            ppc_var2 = (*param_1 + 0x14);
        }
        ppc_var2(0x1010, param_1, u_var7, 1, u_var9, ctx.dx_reg);
        pass1_1010_375e(pp_var8);
        pass1_1010_4788((i_var4 + 0xf2), CONCAT22(ctx.dx_reg, u_var9));
        return;
    }
    if (0xfb < param_2) {
        match (param_2) {
            _ => {
                return;
            }
            0x12a => {
                h_wnd = (i_var4 + 8);
                w_param = 0xf012;
            }
            300 => {
                h_wnd = (i_var4 + 8);
                w_param = 0xf020;
            }
        }
        winapi_funcs::PostMessage16(0, w_param, 0x112, h_wnd);
        return;
    }
    if (param_2 == 0xf3) {
        u_var9 = 3;
    } else {
        if (0xf3 < param_2) {
            return;
        }
        cVar3 = param_2;
        u_var6 = param_2 & 0xff00 | (cVar3 + 0x91);
        if ((cVar3 + 0x91) == 0) {
            mixed_fn_1010_830a(ctx._g_struct_73_1050_14cc, 0x1f8);
            // winapi_funcs::WinHelp16(0x28, 1, CONCAT22(ctx.dx_reg, u_var6), (i_var4 + 8));
            winapi_funcs::WinHelp16(i_var4 + 8, CONCAT22(ctx.dx_reg, u_var6), 1, 0x28);
            return;
        }
        if (cVar3 == 'r') {
            i_var5 = i_var4 + 10;
            u_var9 = u_var7;
            pp_var8 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(i_var5, 0x30));
            pass1_1010_3770(pp_var8, CONCAT22(u_var9, i_var5));
            pass1_1038_af40(ctx._g_Struct112_a, *(i_var4 + 8), 3);
            return;
        }
        if (cVar3 == -0xf) {
            u_var9 = 1;
        } else {
            if (cVar3 != -0xe) {
                return;
            }
            u_var9 = 2;
        }
    }
    u_var1 = (i_var4 + 0xf2);
    pass1_1010_4674(u_var1, (u_var1 >> 0x10), u_var9);
    return;
}

pub fn call_win_fn_1020_0e8e(in_struct_1: &mut Struct637, param_2: u32, param_3: u32) {
    let mut i_var1: i32;
    // fn_ptr_1: &mut Vec<u8>;

    i_var1 = win_fn_1020_1294(CONCAT22(param_2, in_struct_1), (param_2 >> 0x10), param_3);
    if (i_var1 == 0) {
        fn_ptr_1 = (in_struct_1.field_0x4 + 0x5c);
        (**fn_ptr_1)();
    }
    return;
}

pub unsafe fn win_msg_fn_1020_0ae8(ctx: &mut AppContext, struct_param_1: &mut Struct7, param_2: u8) -> &mut Struct7 {
    send_win_msg_1020_08fe(struct_param_1);
    if (param_2 & 1) != 0 {
        error_check_1000_17ce(ctx, struct_param_1);
    }
    return struct_param_1;
}

pub unsafe fn send_win_msg_1020_08fe(in_struct_1: &mut Struct7) {
    let hwnd: HWND16;
    let b_var1: bool;
    let local_struct_1: Struct7;
    let local_struct_1_hi: Struct7;
    let mut local_4: u16;
    let mut temp_5f73679df1: u32;
    local_struct_1_hi = (in_struct_1 >> 0x10);
    local_struct_1 = in_struct_1;
    in_struct_1.ptr_a_lo = 0xb0e;
    local_struct_1.ptr_a_hi = 0x1020;
    if (&local_struct_1.field_0xe8 != 0) {
        temp_5f73679df1 = local_struct_1.field_0xe8;
        hwnd = (temp_5f73679df1 + 6);
        b_var1 = winapi_funcs::IsWindow16(hwnd);
        if b_var1 != 0 {
            // winapi_funcs::SendMessage16(0, 1, 0x111, hwnd);
            winapi_funcs::SendMessage16(hwnd, 0x111, 1, 0);
        }
        local_struct_1.field_0xe8 = 0;
    }
    pass1_1008_57c4((in_struct_1 & 0xffff0000 | ZEXT24(&local_struct_1.struct_215_ptr_0xd2)));
    in_struct_1.ptr_a_lo = 0x380a;
    local_struct_1.ptr_a_hi = &ctx.PTR_LOOP_1050_1008;
    in_struct_1.ptr_a_lo = ctx.s_1_1050_389a;
    local_struct_1.ptr_a_hi = &ctx.PTR_LOOP_1050_1008;
    return;
}

pub fn send_win_msg_1020_097e(param_1: u32) {
    let mut u_var1: u32;
    let mut i_var2: i32;
    let mut u_var3: u16;

    u_var3 = (param_1 >> 0x10);
    i_var2 = param_1;
    if (((i_var2 + 0xea) | (i_var2 + 0xe8)) != 0) {
        u_var1 = (i_var2 + 0xe8);
        winapi_funcs::SendMessage16(0, 1, 0x111, (u_var1 + 6));
        (i_var2 + 0xe8) = 0;
    }
    return;
}

pub fn post_msg_1020_03b2(param_1: u32) {
    let mut u_var1: u32;

    u_var1 = (param_1 + 0xe2);
    winapi_funcs::PostMessage16(0, (u_var1 + 0x16), 0x111, ctx.g_h_window);
    return;
}

pub fn post_msg_1020_03d6(param_1: u32) {
    let mut u_var1: u32;

    u_var1 = (param_1 + 0xe2);
    winapi_funcs::PostMessage16(0, (u_var1 + 0x16), 0x111, ctx.g_h_window);
    return;
}

pub fn post_msg_1020_03fa(param_1: u32) {
    let mut u_var1: u32;

    u_var1 = (param_1 + 0xe2);
    winapi_funcs::PostMessage16(0, (u_var1 + 0x16), 0x111, ctx.g_h_window);
    return;
}

pub fn post_win_msg_1020_061c(param_1: u32, param_2: i32) {
    let mut u_var1: u32;
    let mut u_var2: u16;

    u_var2 = (param_1 >> 0x10);
    if (param_2 == 1) {
        (param_1 + 6) = 0;
        return;
    }
    if (param_2 != 2) {
        return;
    }
    u_var1 = (param_1 + 6);
    winapi_funcs::PostMessage16(0, (u_var1 + 0x16), 0x111, ctx.g_h_window);
    return;
}

pub fn win_func_1018_6bb6(param_1: u32) {
    let b_var1: bool;
    let mut i_var2: i32;
    let mut u_var3: u16;

    u_var3 = (param_1 >> 0x10);
    i_var2 = param_1;
    if ((i_var2 + 0xea) != 0) {
        winapi_funcs::PostMessage16(0, (i_var2 + 0xea), 0x111, ctx.g_h_window);
    }
    winapi_funcs::PostMessage16(0, 0x79, 0x111, ctx.g_h_window);
    if ((i_var2 + 0xf0) != 0) {
        b_var1 = winapi_funcs::IsWindow16((i_var2 + 0xf0));
        if (b_var1 != 0) {
            winapi_funcs::DestroyWindow16((i_var2 + 0xf0));
            (i_var2 + 0xf0) = 0;
        }
    }
    return;
}

pub unsafe fn send_win_msg_1010_7c9e(param_1: u32, param_2: u16) {
    let mut u_var1: u32;
    let BVar2: bool;
    let mut i_var3: i32;
    let mut u_var4: u16;
    let mut unaff_ss: u16;
    let lVar5: u32;
    let mut u_var6: u32;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u32;
    let mut local_a: [u8; 8];

    u_var4 = (param_1 >> 0x10);
    i_var3 = param_1;
    if ((((i_var3 + 0x1e) | (i_var3 + 0x1c)) != 0) && (param_2 != 0)) {
        pass1_1008_5784(CONCAT22(unaff_ss, local_a), (i_var3 + 0x1c));
        while (true) {
            lVar5 = pass1_1008_5b12(CONCAT22(unaff_ss, local_a));
            u_var4 = (lVar5 >> 0x10);
            i_var3 = lVar5;
            if (lVar5 == 0) {
                break;
            }
            if ((i_var3 + 4) != 0) {
                u_var6 = pass1_1030_73a8((i_var3 + 4));
                BVar2 = pass1_1008_c6ae(ctx._PTR_LOOP_1050_06e0, (u_var6 + 0xc), param_2);
                if (BVar2 != 0) {
                    u_var1 = (i_var3 + 8);
                    winapi_funcs::SendMessage16(0, 0xeb, 0x111, (u_var1 + 6));
                }
            }
        }
    }
    return;
}

pub unsafe fn send_msg_1010_7c42(param_1: u32) {
    let mut u_var1: u32;
    let mut i_var2: i32;
    let mut u_var3: u16;
    let mut unaff_ss: u16;
    let lVar4: u32;
    let mut local_e: u32;
    let mut local_a: [u8; 8];

    u_var3 = (param_1 >> 0x10);
    i_var2 = param_1;
    if (((i_var2 + 0x1e) | (i_var2 + 0x1c)) != 0) {
        pass1_1008_5784(CONCAT22(unaff_ss, local_a), (i_var2 + 0x1c));
        while (true) {
            lVar4 = pass1_1008_5b12(CONCAT22(unaff_ss, local_a));
            if (lVar4 == 0) {
                break;
            }
            u_var1 = (lVar4 + 8);
            winapi_funcs::SendMessage16(0, 0xeb, 0x111, (u_var1 + 6));
        }
    }
    return;
}

pub unsafe fn write_private_profile_str_1010_5b10(param_1: &mut Struct376) {
    let pu_var1: u32;
    let mut u_var2: i32;
    let mut u_var3: u32;
    let ppcVar4: fn();
    let mut u_var5: u16;
    let mut u_var6: u16;
    let pp_var7: Struct2111;
    let mut local_c: u16;
    let mut local_8: u16;

    u_var6 = (param_1 >> 0x10);
    local_bx_5 = param_1;
    param_1.ptr_a_lo = 0x6312;
    local_bx_5.field_0x2 = 0x1010;
    pp_var7 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(unaff_si, 0x48));
    u_var3 = local_bx_5.field_0xe;
    string_fn_1000_3f9c(ctx,
        u_var3,
        (u_var3 >> 0x10),
        s__d__d_1050_149c,
        &ctx.g_alloc_addr_1050_1050,
        *(pp_var7 + 10),
    );
    if (local_bx_5.field_0x80 == 0) {
        u_var5 = SUB42(s_off_1050_13c8, 0);
    } else {
        u_var5 = SUB42(s_on_1050_13c4, 0);
    }
    WritePrivateProfileString16(
        local_bx_5.field_0xa,
        CONCAT22(0x1050, u_var5),
        s_falseMap_1050_13de,
        s_general_1050_13b0,
    );
    WritePrivateProfileString16(
        local_bx_5.field_0xa,
        local_bx_5.field_0xe,
        s_rez_1050_13c0,
        s_general_1050_13b0,
    );
    if (local_bx_5.field_0x1e == 0) {
        u_var5 = SUB42(s_off_1050_13c8, 0);
    } else {
        u_var5 = SUB42(s_on_1050_13c4, 0);
    }
    WritePrivateProfileString16(
        local_bx_5.field_0xa,
        CONCAT22(0x1050, u_var5),
        s_anims_1050_13cc,
        s_general_1050_13b0,
    );
    if (local_bx_5.field_0x74 == 0) {
        u_var5 = SUB42(s_off_1050_13c8, 0);
    } else {
        u_var5 = SUB42(s_on_1050_13c4, 0);
    }
    WritePrivateProfileString16(
        local_bx_5.field_0xa,
        CONCAT22(0x1050, u_var5),
        s_music_1050_13d2,
        s_general_1050_13b0,
    );
    if (local_bx_5.field_0x72 == 0) {
        u_var5 = SUB42(s_off_1050_13c8, 0);
    } else {
        u_var5 = SUB42(s_on_1050_13c4, 0);
    }
    WritePrivateProfileString16(
        local_bx_5.field_0xa,
        CONCAT22(0x1050, u_var5),
        s_sound_1050_13d8,
        s_general_1050_13b0,
    );
    if (local_bx_5.field_0x20 == 0) {
        u_var5 = SUB42(s_off_1050_13c8, 0);
    } else {
        u_var5 = SUB42(s_on_1050_13c4, 0);
    }
    WritePrivateProfileString16(
        local_bx_5.field_0xa,
        CONCAT22(0x1050, u_var5),
        s_movies_1050_13e8,
        s_general_1050_13b0,
    );
    u_var3 = local_bx_5.field_0xe;
    string_fn_1000_3f9c(ctx,
        u_var3,
        &ctx.s__lu_1050_14a2,
        &ctx.g_alloc_addr_1050_1050,
        local_bx_5.field_0x76,
    );
    WritePrivateProfileString16(
        local_bx_5.field_0xa,
        local_bx_5.field_0xe,
        s_turns_1050_1466,
        s_general_1050_13b0,
    );
    if local_bx_5.field_0x7a == 0 {
        u_var5 = ctx.s_off_1050_13c8;
    } else {
        u_var5 = ctx.s_on_1050_13c4;
    }
    WritePrivateProfileString16(
        local_bx_5.field_0xa,
        CONCAT22(0x1050, u_var5),
        s_turnsDlgStatus_1050_146c,
        s_general_1050_13b0,
    );
    WritePrivateProfileString16(
        local_bx_5.field_0xa,
        local_bx_5.field_0x1a,
        s_savePath_1050_147c,
        s_general_1050_13b0,
    );
    WritePrivateProfileString16(
        local_bx_5.field_0xa,
        local_bx_5.field_0x68,
        s_aiName_1050_1486,
        s_general_1050_13b0,
    );
    WritePrivateProfileString16(
        local_bx_5.field_0xa,
        local_bx_5.field_0x6c,
        s_playerName_1050_148e,
        s_general_1050_13b0,
    );
    local_c = 1;
    while {
        // switchD_1010:2ab5:
        write_private_profile_str_1010_622a(local_bx_5, u_var6, local_c);
        local_c = local_c + 1;
        local_c < 8
    } {}
    error_check_1000_17ce(ctx, local_bx_5.field_0xa);
    error_check_1000_17ce(ctx, local_bx_5.field_0xe);
    error_check_1000_17ce(ctx, local_bx_5.field_0x12);
    error_check_1000_17ce(ctx, local_bx_5.field_0x16);
    error_check_1000_17ce(ctx, local_bx_5.field_0x1a);
    pu_var1 = local_bx_5.field_0x64;
    u_var2 = local_bx_5.field_0x66;
    if ((u_var2 | pu_var1) != 0) {
        unsafe {
            ppcVar4 = *pu_var1;
        }
        (**ppcVar4)(0x1000, pu_var1, u_var2, 1);
    }
    error_check_1000_17ce(local_bx_5.field_0x68);
    error_check_1000_17ce(local_bx_5.field_0x6c);
    pass1_1010_1d80(param_1);
    return;
}

pub unsafe fn get_private_profile_str_1010_5404(ctx: &mut AppContext, param_1: &mut Struct64, param_2: u32) {
    let pi_var1: i32;
    let pu_var2: u32;
    let mut u_var3: u32;
    let ppcVar4: fn();
    let mut u_var5: u32;
    let u_var6: u8;
    let mut u_var7: i32;
    let mut u_var8: u16;
    let mut i_var9: i32;
    let i_var10: u16;
    let mut pa_var11: Struct103;
    let mut u_var12: i32;
    let mut struct_a: Struct103 = Struct103::new();
    let mut pa_var14: Struct103;
    let pu_var15: Vec<u8>;
    let mut u_var16: u16;
    let mut unaff_ss: u16;
    let pp_var17: Struct2111;
    let u_var18: u8;
    let u_var19: u8;
    let mut u_var20: u16;
    let mut u_var21: u16;
    let mut u_var22: u16;
    let mut u_var23: u16;
    let local_135: u8;
    let local_134: u8;
    let local_133: u8;
    let mut local_2e: u16;
    let mut local_2c: u16;
    let mut local_2a: u16;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: u32;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;
    let pu_var13: u16;

    // u_var22 = param_2;
    process_struct_1010_1d48(CONCAT22(u_var22, param_1), (param_2 >> 0x10));
    u_var8 = 0;
    param_1.field_0xa = 0;
    param_1.str_field_0xe = 0;
    param_1.field_0x12 = 0;
    param_1.field_0x16 = 0;
    param_1.field_0x1a = 0;
    param_1.field_0x62 = 0;
    param_1.field_0x64 = 0;
    param_1.field_0x68 = 0;
    param_1.field_0x6c = 0;
    param_1.field_0x70 = 1;
    param_1.field_0x7a = 0;
    param_1.field_0x7c = 0;
    param_1.field_0x7e = 0;
    param_1.field_0x80 = 0;
    param_1.field_0x82 = 1;
    CONCAT22(u_var22, param_1) = 0x6312;
    param_1.field_0x2 = 0x1010;
    str_fn_1010_6034(CONCAT22(u_var22, param_1));
    pa_var14 = struct_a;
    process_struct_1000_179c(0x101, &mut struct_a);
    param_1.str_field_0xe = u_var8;
    &param_1.field_0x10 = pa_var14;
    pass1_fn_1000_5008(ctx, &param_1.str_field_0xe, &pa_var14, 0x100);
    u_var7 = get_string_index_1000_3da4(&mut param_1.str_field_0xe);
    u_var5 = &param_1.str_field_0xe;
    u_var16 = (u_var5 >> 0x10);
    pu_var15 = (u_var5 + u_var7);
    if pu_var15[-1] != '\\' {
        unsafe {
            *pu_var15 = 0x5c;
        }
        u_var5 = &param_1.str_field_0xe;
        *(u_var5 + u_var7 + 1) = 0;
    }
    local_4 = u_var7;
    load_string_1010_847e(
        ctx._g_struct_73_1050_14cc,
        (ctx._g_struct_73_1050_14cc >> 0x10),
        0x578,
    );
    pa_var14 = ctx.dx_reg;
    local_8 = u_var7;
    local_6 = ctx.dx_reg;
    process_string_1000_3cea(&param_1.str_field_0xe, CONCAT22(ctx.dx_reg, u_var7));
    u_var5 = &param_1.str_field_0xe;
    pass1_fn_1008_60e8(u_var5);
    param_1.field_0xa = u_var7;
    &param_1.field_0xc = pa_var14;
    u_var23 = SUB42(&ctx.g_alloc_addr_1050_1050, 0);
    u_var5 = &param_1.str_field_0xe;
    u_var16 = SUB42(offset, 0);
    u_var8 = GetPrivateProfileString16(
        CONCAT22(param_1.field_0xa, 0x1008),
        pa_var14,
        CONCAT22(u_var5, 0x100),
        CONCAT22(0x149a, (u_var5 >> 0x10)),
        0x13c01050,
        0x13b01050,
    );
    if (*&param_1.str_field_0xe != '\0') {
        u_var16 = 0x1000;
        pass1_fn_1000_3e2c(&param_1.str_field_0xe);
        local_16 = u_var8;
        pp_var17 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(u_var23, 0x48));
        pa_var14 = (pp_var17 >> 0x10);
        local_1a = pp_var17;
        local_a = (local_1a + 10);
        local_c = (local_1a + 0xc);
        param_1.field_0x62 = (local_16 != local_a);
        local_18 = pa_var14;
    }
    u_var5 = &param_1.str_field_0xe;
    u_var3 = &param_1.field_0xa;
    u_var23 = SUB42(offset, 0);
    GetPrivateProfileString16(
        CONCAT22(u_var3, u_var16),
        (u_var3 >> 0x10),
        CONCAT22(u_var5, 0x100),
        CONCAT22(0x149a, (u_var5 >> 0x10)),
        0x13de1050,
        0x13b01050,
    );
    if (*&param_1.str_field_0xe != '\0') {
        u_var23 = 0x1000;
        i_var9 = process_string_1000_475e(&param_1.str_field_0xe, s_on_1050_13c4);
        if (i_var9 == 0) {
            param_1.field_0x80 = 1;
        }
    }
    u_var5 = &param_1.str_field_0xe;
    u_var3 = &param_1.field_0xa;
    u_var16 = SUB42(offset, 0);
    GetPrivateProfileString16(
        CONCAT13((u_var3 >> 8), CONCAT12(u_var3, u_var23)),
        (u_var3 >> 0x10),
        CONCAT22(u_var5, 0x100),
        CONCAT22(0x149a, (u_var5 >> 0x10)),
        0x13d21050,
        0x13b01050,
    );
    if (*&param_1.str_field_0xe != '\0') {
        u_var16 = 0x1000;
        i_var9 = process_string_1000_475e(&param_1.str_field_0xe, s_off_1050_13c8);
        if (i_var9 == 0) {
            param_1.field_0x74 = 0;
        }
    }
    u_var5 = &param_1.str_field_0xe;
    u_var3 = &param_1.field_0xa;
    u_var23 = SUB42(offset, 0);
    GetPrivateProfileString16(
        CONCAT13((u_var3 >> 8), CONCAT12(u_var3, u_var16)),
        (u_var3 >> 0x10),
        CONCAT22(u_var5, 0x100),
        CONCAT22(0x149a, (u_var5 >> 0x10)),
        0x13d81050,
        0x13b01050,
    );
    if (*&param_1.str_field_0xe != '\0') {
        u_var23 = 0x1000;
        i_var9 = process_string_1000_475e(&param_1.str_field_0xe, s_off_1050_13c8);
        if (i_var9 == 0) {
            param_1.field_0x72 = 0;
        }
    }
    u_var5 = &param_1.str_field_0xe;
    u_var3 = &param_1.field_0xa;
    u_var16 = SUB42(offset, 0);
    GetPrivateProfileString16(
        CONCAT13((u_var3 >> 8), CONCAT12(u_var3, u_var23)),
        (u_var3 >> 0x10),
        CONCAT22(u_var5, 0x100),
        CONCAT22(0x149a, (u_var5 >> 0x10)),
        0x13cc1050,
        0x13b01050,
    );
    if (*&param_1.str_field_0xe != '\0') {
        u_var16 = 0x1000;
        i_var9 = process_string_1000_475e(&param_1.str_field_0xe, s_off_1050_13c8);
        if (i_var9 == 0) {
            param_1.field_0x1e = 0;
        }
    }
    u_var5 = &param_1.str_field_0xe;
    u_var3 = &param_1.field_0xa;
    u_var23 = SUB42(offset, 0);
    GetPrivateProfileString16(
        CONCAT13((u_var3 >> 8), CONCAT12(u_var3, u_var16)),
        (u_var3 >> 0x10),
        CONCAT13((u_var5 >> 8), CONCAT12(u_var5, 0x100)),
        CONCAT22(0x149a, (u_var5 >> 0x10)),
        0x13e81050,
        0x13b01050,
    );
    if (*&param_1.str_field_0xe != '\0') {
        u_var23 = 0x1000;
        i_var9 = process_string_1000_475e(&param_1.str_field_0xe, s_off_1050_13c8);
        if (i_var9 == 0) {
            param_1.field_0x20 = 0;
        }
    }
    u_var5 = &param_1.str_field_0xe;
    u_var3 = &param_1.field_0xa;
    u_var16 = SUB42(offset, 0);
    u_var7 = GetPrivateProfileString16(
        CONCAT22(u_var3, u_var23),
        (u_var3 >> 0x10),
        CONCAT13((u_var5 >> 8), CONCAT12(u_var5, 0x100)),
        CONCAT22(0x149a, (u_var5 >> 0x10)),
        0x14661050,
        0x13b01050,
    );
    pa_var11 = pa_var14;
    if (*&param_1.str_field_0xe != '\0') {
        u_var16 = 0x1000;
        pass1_fn_1000_3e2c(&param_1.str_field_0xe);
        pa_var11 = (pa_var14 | u_var7);
        local_2e = u_var7;
        local_2c = pa_var14;
        if ((pa_var14 | u_var7) != 0x0) {
            param_1.field_0x76 = u_var7;
            param_1.field_0x78 = pa_var14;
            pa_var11 = pa_var14;
        }
    }
    u_var21 = SUB42(&ctx.g_alloc_addr_1050_1050, 0);
    u_var5 = &param_1.str_field_0xe;
    u_var3 = &param_1.field_0xa;
    u_var23 = SUB42(offset, 0);
    GetPrivateProfileString16(
        CONCAT22(u_var3, u_var16),
        (u_var3 >> 0x10),
        CONCAT13((u_var5 >> 8), CONCAT12(u_var5, 0x100)),
        CONCAT13(0x14, CONCAT12(0x9a, (u_var5 >> 0x10))),
        0x146c1050,
        0x13b01050,
    );
    if (*&param_1.str_field_0xe != '\0') {
        u_var23 = 0x1000;
        i_var9 = process_string_1000_475e(&param_1.str_field_0xe, s_on_1050_13c4);
        if (i_var9 == 0) {
            param_1.field_0x7a = 1;
        }
    }
    u_var20 = SUB42(&ctx.g_alloc_addr_1050_1050, 0);
    u_var5 = &param_1.str_field_0xe;
    u_var3 = &param_1.field_0xa;
    u_var16 = SUB42(offset, 0);
    i_var10 = GetPrivateProfileString16(
        CONCAT13((u_var3 >> 8), CONCAT12(u_var3, u_var23)),
        (u_var3 >> 0x10),
        CONCAT13((u_var5 >> 8), CONCAT12(u_var5, 0x100)),
        CONCAT13(0x14, CONCAT12(0x9a, (u_var5 >> 0x10))),
        0x147c1050,
        0x13b01050,
    );
    if (*&param_1.str_field_0xe != '\0') {
        u_var5 = &param_1.str_field_0xe;
        u_var16 = SUB42(&ctx.PTR_LOOP_1050_1008, 0);
        pass1_fn_1008_60e8(u_var5, (u_var5 >> 0x10), u_var20, u_var21);
        param_1.field_0x1a = i_var10;
        param_1.field_0x1c = pa_var11;
    }
    u_var21 = SUB42(&ctx.g_alloc_addr_1050_1050, 0);
    u_var5 = &param_1.str_field_0xe;
    u_var3 = &param_1.field_0xa;
    u_var23 = SUB42(offset, 0);
    i_var10 = GetPrivateProfileString16(
        CONCAT22(u_var3, u_var16),
        (u_var3 >> 0x10),
        CONCAT22(u_var5, 0x100),
        CONCAT13(0x14, CONCAT12(0x9a, (u_var5 >> 0x10))),
        0x14861050,
        0x13b01050,
    );
    if (*&param_1.str_field_0xe != '\0') {
        u_var5 = &param_1.str_field_0xe;
        u_var23 = SUB42(&ctx.PTR_LOOP_1050_1008, 0);
        pass1_fn_1008_60e8(u_var5, (u_var5 >> 0x10), u_var21);
        param_1.field_0x68 = i_var10;
        param_1.field_0x6a = pa_var11;
    }
    u_var16 = SUB42(&ctx.g_alloc_addr_1050_1050, 0);
    u_var5 = &param_1.str_field_0xe;
    u_var3 = &param_1.field_0xa;
    u_var7 = GetPrivateProfileString16(
        CONCAT13((u_var3 >> 8), CONCAT12(u_var3, u_var23)),
        (u_var3 >> 0x10),
        CONCAT22(u_var5, 0x100),
        CONCAT13(0x14, CONCAT12(0x9a, (u_var5 >> 0x10))),
        0x148e1050,
        0x13b01050,
    );
    if (*&param_1.str_field_0xe != '\0') {
        u_var5 = &param_1.str_field_0xe;
        pass1_fn_1008_60e8(u_var5, (u_var5 >> 0x10), u_var16);
        param_1.field_0x6c = u_var7;
        param_1.field_0x6e = pa_var11;
    }
    if (param_1.field_0x62 == 0) {
        local_2e = winapi_funcs::GetSystemMetrics16(4);
        local_2a = 1;
        while {
            get_private_profile_str_1010_6132(param_1, u_var22, local_2a);
            i_var9 = &param_1.field_0x0 + local_2a * 8;
            pa_var11 = ctx.dx_reg;
            let pi_var1_val = unsafe { *pi_var1 };
            let pu_var2_val = unsafe { *pu_var2 };

            if (((((i_var9 + 0x22) < 0) || ((i_var9 + 0x24) < 0))
                || (
                    pi_var1 = (i_var9 + 0x22),
                    pi_var1_val != local_a - local_2e && (local_a - local_2e) <= pi_var1_val,
                ))
                || (
                    u_var7 = local_c - local_2e,
                    pu_var2 = (i_var9 + 0x24),
                    pu_var2_val != u_var7 && u_var7 <= pu_var2_val,
                ))
            {
                u_var6 = pass1_1000_4906(
                    CONCAT13(
                        (param_2 >> 8),
                        CONCAT12(param_2, &param_1.field_0x22 + local_2a * 8),
                    ),
                    0,
                    8,
                );
                u_var7 = CONCAT31(extraout_var, u_var6);
            }
            local_2a = local_2a + 1;
            local_2a < 8
        } {}
    }
    process_struct_1000_179c(0xc, &mut pa_var11);
    _local_32 = CONCAT22(pa_var11, u_var7);
    pa_var14 = (pa_var11 | u_var7);
    if (pa_var14 == 0x0) {
        pa_var11 = 0x0;
        pa_var14 = 0x0;
    } else {
        pa_var11 = process_struct_1008_574a(ctx, CONCAT22(pa_var11, u_var7));
    }
    param_1.field_0x64 = pa_var11;
    &param_1.field_0x66 = pa_var14;
    u_var5 = &param_1.str_field_0xe;
    pass1_fn_1000_5008(ctx, u_var5, (u_var5 >> 0x10), 0x100);
    u_var7 = get_string_index_1000_3da4(&mut param_1.str_field_0xe);
    u_var5 = &param_1.str_field_0xe;
    u_var16 = (u_var5 >> 0x10);
    pu_var15 = (u_var5 + u_var7);
    if (pu_var15[-1] != '\\') {
        unsafe {
            *pu_var15 = 0x5c;
        }
        u_var5 = &param_1.str_field_0xe;
        *(u_var5 + u_var7 + 1) = 0;
    }
    u_var5 = &param_1.str_field_0xe;
    local_4 = u_var7;
    pass1_fn_1008_60e8(u_var5, (u_var5 >> 0x10));
    _local_10 = CONCAT22(pa_var14, u_var7);
    process_struct_1000_179c(8, &mut pa_var14);
    _local_32 = CONCAT22(pa_var14, u_var7);
    if ((pa_var14 | u_var7) == 0) {
        local_14 = 0;
    } else {
        _local_32 = ctx.s_1_1050_389a.clone();
        (u_var7 + 2) = ctx.PTR_LOOP_1050_1008;
        (u_var7 + 4) = _local_10;
        _local_32 = 0x6322;
        (u_var7 + 2) = 0x1010;
        local_14 = _local_32;
    }
    u_var5 = &param_1.field_0x64;
    ppcVar4 = (&param_1.field_0x64 + 4);
    (**ppcVar4)(0, u_var5, (u_var5 >> 0x10), local_14, (local_14 >> 0x10));
    u_var5 = &param_1.str_field_0xe;
    u_var3 = &param_1.field_0xa;
    u_var7 = ctx.dx_reg;
    u_var12 = GetPrivateProfileString16(
        CONCAT22(u_var3, 0x1000),
        (u_var3 >> 0x10),
        CONCAT22(u_var5, 0x100),
        CONCAT22(0x149a, (u_var5 >> 0x10)),
        0x13f01050,
        0x13b01050,
    );
    pu_var13 = u_var12;
    if (*&param_1.str_field_0xe != '\0') {
        u_var5 = &param_1.str_field_0xe;
        u_var18 = u_var5;
        u_var19 = (u_var5 >> 8);
        u_var16 = (u_var5 >> 0x10);
        while (true) {
            u_var12 = pu_var13;
            pass1_fn_1000_47a4(CONCAT22(u_var16, CONCAT11(u_var19, u_var18)), s___1050_13f8);
            if ((u_var7 | u_var12) == 0) {
                break;
            }
            local_2e = u_var12;
            local_2c = u_var7;
            pa_var14 = copy_string_1000_3d3e(
                CONCAT22(unaff_ss, &local_134),
                CONCAT13((u_var7 >> 8), CONCAT12(u_var7, u_var12)),
            );
            local_4 = get_string_index_1000_3da4(CONCAT13(
                (unaff_ss >> 8),
                CONCAT12(unaff_ss, &local_134),
            ));
            if ((&local_135)[local_4] != '\\') {
                (&local_134)[local_4] = 0x5c;
                (&local_133)[local_4] = '\0';
            }
            pu_var13 = ZEXT24(&local_134);
            pass1_fn_1008_60e8(&local_134);
            _local_10 = pu_var13 & 0xffff | ZEXT24(pa_var14) << 0x10;
            process_struct_1000_179c(8, &mut pa_var14);
            u_var7 = pu_var13;
            _local_32 = (pu_var13 & 0xffff | ZEXT24(pa_var14) << 0x10);
            if ((pa_var14 | u_var7) == 0) {
                local_14 = 0;
            } else {
                _local_32 = ctx.s_1_1050_389a.clone();
                (u_var7 + 2) = ctx.PTR_LOOP_1050_1008;
                (u_var7 + 4) = _local_10;
                _local_32 = 0x6322;
                (u_var7 + 2) = 0x1010;
                pu_var13 = _local_32;
                local_14 = _local_32;
            }
            u_var5 = &param_1.field_0x64;
            ppcVar4 = (&param_1.field_0x64 + 8);
            (**ppcVar4)(
                0x1000,
                u_var5,
                (u_var5 >> 0x10),
                local_14,
                (local_14 >> 0x10),
            );
            u_var18 = 0;
            u_var19 = 0;
            u_var16 = 0;
            u_var7 = ctx.dx_reg;
        }
    }
    return;
}

pub unsafe fn win_cleanup_func_1040_b0f8(param_1: &mut Struct7) -> u8 {
    let mut u_var1: i32;
    let mut u_var2: i32;
    let mut u_var3: u8;
    let mut u_var4: u16;
    let mut ppVar5: Struct2111;
    let mut in_stack_0000ffe8: u32;
    let mut local_a: u16;
    let mut local_8: u16;

    u_var4 = (param_1 >> 0x10);
    local_bx_4 = param_1;
    param_1.ptr_a_lo = 0xb772;
    local_bx_4.field_0x2 = &ctx.PTR_LOOP_1050_1040;
    ppVar5 = process_struct_1010_20ba(
        ctx._g_Struct372_1050_0ed0,
        CONCAT22((in_stack_0000ffe8 >> 0x10), 0x32),
    );
    pass1_1010_7b8c(&mut ppVar5, local_bx_4.field_0x6);
    if local_bx_4[0x11].field_0x6 != 0 {
        winapi_funcs::DeleteObject16(local_bx_4[0x11].field_0x6);
        local_bx_4[0x11].field_0x6 = 0;
    }
    u_var1 = (local_bx_4 + 0x12).field_0x0;
    u_var2 = local_bx_4[0x12].field_0x2;
    _local_a = CONCAT22(u_var2, u_var1);
    if (u_var2 | u_var1) != 0 {
        pass1_1040_a5d0(CONCAT22(u_var2, u_var1));
        error_check_1000_17ce(ctx, _local_a);
    }
    u_var3 = win_cleanup_func_1040_782c(ctx, param_1);
    return u_var3;
}

pub fn win_fn_1010_0242(param_1: u16, param_2: u16, param_3: HWND16) -> u16 {
    let pp_var1: fn();
    
    let BVar2: bool;
    let wVar3: u16;
    let ppVar4: Struct2111;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    BVar2 = winapi_funcs::IsWindow16(param_1);
    if (BVar2 != 0) {
        wVar3 = winapi_funcs::GetWindowWord16(-6, param_1);
        if (wVar3 == &ctx.g_h_instance_1050_038c) {
            BVar2 = winapi_funcs::IsIconic16(param_1);
            if (BVar2 != 0) {
                _local_8 = CONCAT22(local_6, 0x45);
                ppVar4 = process_struct_1010_20ba(&g_Struct372_1050_0ed0, _local_8);
                _local_8 = (ppVar4 & 0xffff0000 | param_1);
                pp_var1 = (*_local_8 + 0x10);
                (**pp_var1)(offset, ppVar4, 1);
            }
        }
    }
    return 1;
}

pub unsafe fn win_fn_1010_1656(param_1: &mut Struct318, param_2: u16, param_3: &mut StructuredData, param_4: u16) {
    let mut u_var1: u32;
    let mut i_var2: i32;
    let mut i_var3: i32;
    let mut i_var4: i32;
    let mut u_var5: u16;
    let struct_var6: Struct2111;
    let mut u_var7: u32;
    let mut in_stack_0000000c: u16;
    let mut in_stack_0000ffe0: u16;
    let mut local_c: u16;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;
    let temp_7ff6e820591: Struct369;

    win_cleanup_1010_305a(param_1, param_2, param_3);
    if param_1.field_0x16 == 3 {
        struct_var6 = process_struct_1010_20ba(
            ctx._g_Struct372_1050_0ed0,
            CONCAT22(param_4, 0x32),
        );
        u_var1 = param_1.field_0x32;
        u_var1 = (u_var1 + 0x42);
        // u_var5 = (u_var1 >> 0x10);
        i_var4 = u_var1;
        u_var1 = (i_var4 + 0x16);
        u_var7 = pass1_1030_73a8((u_var1 + 4));
        i_var2 = u_var7;
        pass1_1010_7818(struct_var6, u_var7);
        u_var1 = (i_var4 + 0x16);
        i_var3 = i_var2;
        win_gui_fn_1010_79aa(&mut struct_var6, 0, (u_var1 + 4));
        if i_var3 == 0 {
            u_var1 = (i_var4 + 0x16);
            u_var1 = (u_var1 + 4);
            window_msg_func_1010_7300(struct_var6, 0, 0, i_var2, u_var1, (u_var1 >> 0x10));
        }
    }
    return;
}

pub unsafe fn free_rsrc_1010_4b3e(ctx: &mut AppContext, param_1: &mut Struct7) {
    let pu_var1: u16;
    let pu_var2: u32;
    let fn_ptr_var4: fn();
    let pu_var5: &Struct131;
    let b_var7: bool;
    let mut local_4: u16;

    param_1.u16_fld_0 = (ctx.s_SCForceMorale__s_for_colony__08l_1050_5024 + 6);
    param_1.u16_fld_1 = 0x1010; // (i_var8 + 2)
    if param_1.u16_field_0x2a != 0 { // i_var8 + 0x2a
        // unaff_cs = offset;
        b_var7 = GlobalUnlock16(param_1.u16_field_0x2a); // (i_var8 + 0x2a)
        if b_var7 == false {
            // unaff_cs = SUB42(offset, 0);
            FreeResource16(param_1.u16_field_0x2a); // (i_var8 + 0x2a)
        }
    }
    param_1.u16_field_0x2a = 0; //(i_var8 + 0x2a) = 0;
    if param_1.struct_field_0x12 != 0 { // (i_var8 + 0x12)
        local_4 = 0;
        loop {
            pu_var5 = &param_1.struct_field_0x12;
            pu_var1 = pu_var5.field_0x8;  //(pu_var5 + 8);
            unsafe {
                if pu_var1 == local_4 || pu_var1 < local_4 {
                    break;
                }
                // u_var11 = (*pu_var5 >> 0x10);
                // i_var9 = *pu_var5;

                pu_var2 = (pu_var5.field_0x0 + local_4 * 4) as u32;
                u_var3 = (pu_var5.field_0x0 + local_4 * 4 + 2);
                if (u_var3 | pu_var2) != 0 {
                    fn_ptr_var4 = get_fn_ptr_at_address(pu_var2);
                    (fn_ptr_var4)(ctx.code_seg_reg, pu_var2, u_var3, 1);
                }
            }
            local_4 = local_4 + 1;
        }
    }
    // u_var6 = (i_var8 + 0x12);
    let mut u_var6 = &mut param_1.struct_field_0x12;
    error_check_1000_17ce(ctx, &mut u_var6.field_0x4);
    error_check_1000_17ce(ctx, &mut param_1.struct_field_0x12);
    pu_var2 = param_1.u32_field_0x16;
    // u_var3 = (i_var8 + 0x18);
    if (pu_var2) != 0 {
        fn_ptr_var4 = get_fn_ptr_at_address(pu_var2);
        fn_ptr_var4(0x1000, pu_var2, 1);
    }
    error_check_1000_17ce(ctx, &mut param_1.pv_buffer_0x1a); // (i_var8 + 0x1a)
    pass1_1010_1d80(param_1);
    return;
}

pub fn load_rsrc_1010_4e9e(in_struct_1: &mut Struct60) -> SEGPTR {
    let mut u_var1: u32;
    let mut u_var2: u32;
    let mut unlock_result: u16;
    let mut h_resource: u16;
    let mut handle: HGLOBAL16;
    let mut u_var3: u16;
    let mut SVar4: SEGPTR;

    u_var3 = (in_struct_1 >> 0x10);
    local_bx_5 = in_struct_1;
    if local_bx_5.field_0x20 != 0 {
        if local_bx_5.resource_handle != 0 {
            unlock_result = GlobalUnlock16(local_bx_5.resource_handle);
            if unlock_result == 0 {
                FreeResource16(local_bx_5.resource_handle);
            }
        }
        u_var1 = local_bx_5.field_0x12;
        u_var2 = (u_var1 + 4);
        h_resource = FindResource16(
            0xa,
            *((u_var2 + local_bx_5.field_0x20 * 2) * 2 + 0x1384),
            ctx.g_h_instance_1050_038c,
        );
        handle = LoadResource16(h_resource, ctx.g_h_instance_1050_038c);
        local_bx_5.resource_handle = handle;
        if (handle != 0) {
            SVar4 = LockResource16(handle);
            return SVar4;
        }
    }
    return 0;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn send_msg_1040_c85a(param_1: u32) {
    _PTR_LOOP_1050_5efe = param_1;
    winapi_funcs::SendMessage16(0, 0xfa, 0x111, (param_1 + 0x1a));
    return;
}

pub unsafe fn window_msg_func_1008_97f2(
    param_1: &mut Vec<u8>,
    param_2: &mut i32,
    param_3: i32,
    param_4_00: Vec<u8>,
    param_4: i32,
) {
    let pp_var1: fn();
    let BVar2: bool;
    let mut u_var3: u16;
    let mut i_var4: i32;
    let mut u_var5: i32;
    let unaff_cs: u8;
    let mut u_var6: u32;
    let u_var7: u8;
    let u_var8: u8;
    let mut cVar9: u8;

    i_var4 = param_1;
    u_var3 = (param_1 >> 0x10);
    u_var7 = SUB41(param_1, 0);
    if (param_4 == 0x2b) {
        if (unsafe { *param_2 == 4 }) {
            sys2::get_prop_1040_9566(param_2, param_3);
        } else {
            unsafe {
                pp_var1 = (*param_1 + 0x70);
            }
            (**pp_var1)();
        }
        u_var5 = 1;
        // goto LAB_1008_9a95;
    }
    u_var8 = (param_1 >> 0x10);
    if (param_4 < 0x2c) {
        unaff_cs = 8;
        match (param_4) {
            1 => {}
            2 => {
                unsafe {
                    pp_var1 = (*param_1 + 0x3c);
                }
                (**pp_var1)(&ctx.PTR_LOOP_1050_1008, i_var4, u_var3);
                SetWindowLong16(0, 0);
                BVar2 = winapi_funcs::IsWindow16((i_var4 + 0xbc));
                if (BVar2 != 0) {
                    winapi_funcs::PostMessage16(param_1, 199, 0x111, (i_var4 + 0xbc));
                }
            }
            3 => {
                unsafe {
                    pp_var1 = (*param_1 + 0x54);
                }
                (**pp_var1)(8, u_var7, u_var3, param_3, param_2);
            }
            // default:
            // goto switchD_1008_9b30_caseD_4;
            5 => {
                unsafe {
                    pp_var1 = (*param_1 + 0x58);
                }
                (**pp_var1)(8, u_var7, u_var8, param_3, param_2, param_4_00);
            }
            7 => {
                unsafe {
                    pp_var1 = (*param_1 + 0x50);
                }
                (**pp_var1)(8, i_var4, u_var3, param_4_00);
            }
            8 => {
                unsafe {
                    pp_var1 = (*param_1 + 0x74);
                }
                (**pp_var1)(8, i_var4, u_var3, param_4_00);
            }
            0xd => {
                unsafe {
                    pp_var1 = (*param_1 + 0x84);
                }
                i_var4 = (**pp_var1)(
                    8,
                    u_var7,
                    u_var8,
                    param_2,
                    CONCAT12(param_4_00._0_1_, param_3),
                );
                // goto LAB_1008_9ada;
            }
            0xf => {
                unsafe {
                    pp_var1 = (*param_1 + 0x34);
                }
                (**pp_var1)(&ctx.PTR_LOOP_1050_1008, i_var4, u_var3);
            }
            0x10 => {
                unsafe {
                    pp_var1 = (*param_1 + 0x38);
                }
                u_var6 = (**pp_var1)(&ctx.PTR_LOOP_1050_1008, i_var4, u_var3);
                return u_var6;
            }
            0x19 => {
                unsafe {
                    pp_var1 = (*param_1 + 0x78);
                }
                u_var3 = (**pp_var1)(
                    8,
                    u_var7,
                    u_var8,
                    param_2,
                    CONCAT12(param_4_00._0_1_, param_3),
                );
                return CONCAT22(0x1050, u_var3);
            }
            0x1c => {
                unsafe {
                    pp_var1 = (*param_1 + 0x30);
                }
                (**pp_var1)(8, i_var4, u_var3, param_4_00);
            }
            _ => {}
        }
    } else {
        cVar9 = param_4;
        if (param_4 == 0x112) {
            if ((PTR_LOOP_1050_039a == 0x0)
                && (
                    unsafe { pp_var1 = (*param_1 + 0x48) },
                    unsafe { i_var4 = (**pp_var1)() },
                    i_var4 != 0,
                ))
            {
                def_wnd_proc_func_1008_9ce6(
                    param_1,
                    CONCAT13((param_3 >> 8), CONCAT12(param_3, param_2)),
                    CONCAT13(1, CONCAT12(cVar9, param_4_00)),
                );
            }
        } else {
            if (param_4 < 0x113) {
                if (param_4 == 0x86) {
                    unsafe {
                        pp_var1 = (*param_1 + 0x80);
                    }
                    u_var6 = (**pp_var1)();
                    return u_var6;
                }
                if (param_4 < 0x87) {
                    if (param_4 == 0x85) {
                        unsafe {
                            pp_var1 = (*param_1 + 0x7c);
                        }
                        u_var6 = (**pp_var1)();
                        return u_var6;
                    }
                    if (param_4 < 0x86) {
                        if (cVar9 == '7') {
                            return *(i_var4 + 0xc2);
                        }
                        if (cVar9 == 'A') {
                            unsafe {
                                pp_var1 = (*param_1 + 0x2c);
                            }
                            (**pp_var1)();
                            // goto switchD_1008_9b30_caseD_1;
                        }
                    }
                    // switchD_1008_9b30_caseD_4:
                    if ((param_4 < 0x400) || (0x7ffe < param_4)) {
                        u_var6 = def_wnd_proc_func_1008_9ce6(
                            param_1,
                            CONCAT22(param_3, param_2),
                            CONCAT22(param_4, param_4_00),
                        );
                        return u_var6;
                    }
                    unsafe {
                        pp_var1 = (*param_1 + 0x28);
                    }
                    (**pp_var1)(
                        unaff_cs,
                        u_var7,
                        u_var8,
                        param_2,
                        param_3,
                        CONCAT22(param_4, param_4_00),
                    );
                } else {
                    if (param_4 == 0x100) {
                        if (PTR_LOOP_1050_039a == 0x0) {
                            unsafe {
                                pp_var1 = (*param_1 + 0x6c);
                            }
                            (**pp_var1)();
                        }
                    } else {
                        if (param_4 == 0x102) {
                            if (PTR_LOOP_1050_039a == 0x0) {
                                unsafe {
                                    pp_var1 = (*param_1 + 0x68);
                                }
                                (**pp_var1)();
                            }
                        } else {
                            if (param_4 != 0x111) {}
                            // goto switchD_1008_9b30_caseD_4;
                            if ((param_4_00 != PTR_LOOP_1050_039c) && (PTR_LOOP_1050_039a == 0x0)) {
                                if (param_2 == 0x0) {
                                    unsafe {
                                        pp_var1 = (*param_1 + 0x40);
                                    }
                                    (**pp_var1)();
                                } else {
                                    unsafe {
                                        pp_var1 = (*param_1 + 0x44);
                                    }
                                    (**pp_var1)();
                                }
                            }
                        }
                    }
                }
            } else {
                if (param_4 == 0x204) {
                    if (PTR_LOOP_1050_039a == 0x0) {
                        unsafe {
                            pp_var1 = (*param_1 + 0x60);
                        }
                        (**pp_var1)();
                    }
                } else {
                    if (param_4 < 0x205) {
                        if (param_4 == 0x113) {
                            if (_PTR_LOOP_1050_0388 != 0) {
                                pass1_1008_932a(_PTR_LOOP_1050_0388);
                            }
                        } else {
                            if (param_4 == 0x117) {
                                if (param_3 == 0) {
                                    unsafe {
                                        pp_var1 = (*param_1 + 0x4c);
                                    }
                                    (**pp_var1)();
                                } else {
                                    unsafe {
                                        pp_var1 = (*param_1 + 0x20);
                                    }
                                    (**pp_var1)();
                                }
                            } else {
                                if (param_4 != 0x201) {}
                                // goto switchD_1008_9b30_caseD_4;
                                if (PTR_LOOP_1050_039a == 0x0) {
                                    unsafe {
                                        pp_var1 = (*param_1 + 0x5c);
                                    }
                                    (**pp_var1)();
                                }
                            }
                        }
                    } else {
                        if (param_4 == 0x210) {
                            unsafe {
                                pp_var1 = (*param_1 + 100);
                            }
                            (**pp_var1)();
                        } else {
                            if (param_4 == 0x30f) {
                                // LAB_1008_9af8:
                                unsafe {
                                    pp_var1 = (*param_1 + 0x8c);
                                }
                                i_var4 = (**pp_var1)();
                                // LAB_1008_9ada:
                                return i_var4;
                            }
                            if (param_4 == 0x311) {
                                unsafe {
                                    pp_var1 = (*param_1 + 0x88);
                                }
                                i_var4 = (**pp_var1)();
                                if (i_var4 != 0) {}
                                // goto LAB_1008_9af8;
                            } else {
                                if (param_4 != 0x3b9) {}
                                // goto switchD_1008_9b30_caseD_4;
                                unsafe {
                                    pp_var1 = (*param_1 + 0x24);
                                }
                                (**pp_var1)();
                            }
                        }
                    }
                }
            }
        }
    }
    // switchD_1008_9b30_caseD_1:
    u_var5 = 0;
    // LAB_1008_9a95:
    return u_var5;
}

pub fn pass1_1008_9c16(param_1: u16, param_2: u32, param_3: u32) {
    def_wnd_proc_func_1008_9ce6(
        CONCAT22(param_2, param_1),
        CONCAT22(param_3, (param_2 >> 0x10)),
        CONCAT22(0x85, (param_3 >> 0x10)),
    );
    return;
}

pub fn def_wn_proc_1008_9c30(param_1: u16, param_2: u32, param_3: u32) {
    def_wnd_proc_func_1008_9ce6(
        CONCAT22(param_2, param_1),
        CONCAT22(param_3, (param_2 >> 0x10)),
        CONCAT22(0x86, (param_3 >> 0x10)),
    );
    return;
}

pub fn def_wnd_proc_func_1008_9ce6(param_1: &mut Vec<u8>, param_2: u32, param_3: u32) -> LRESULT {
    let LVar1: LRESULT;

    LVar1 = DefWindowProc16(param_2, param_3, (param_3 >> 0x10), (param_1 + 8));
    return LVar1;
}

pub unsafe fn post_win_msg_1008_a0e4(
    param_1: &mut Struct2111,
    param_2: u32,
    param_3: u16,
    param_4: u16,
    param_5: u32,
    param_6: u16,
) -> u8 {
    let mut u_var1: u32;
    let ppc_var2: fn();
    let mut u8_var3: bool;
    let u_var4: u8;
    let mut u_var5: i32;
    let mut u_var7: u32;
    let puVar8: u16;
    
    
    let mut i_var9: i32;
    let mut u_var10: u16;
    let mut unaff_ss: u16;
    let mut local_20: u32;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u32;
    let mut local_a: [u8; 8];
    let b_var6: bool;

    u_var10 = (param_1 >> 0x10);
    i_var9 = param_1;
    pass1_1008_5784(CONCAT22(unaff_ss, local_a), (i_var9 + 10));
    u8_var3 = false;
    while {
        u_var7 = ZEXT24(local_a);
        pass1_1008_5b12(CONCAT22(unaff_ss, local_a));
        u_var5 = u_var7;
        if ((ctx.dx_reg | u_var5) == 0) {}
        // goto LAB_1008_a146;
        (u_var5 + 4) != param_6
    } {}
    (u_var5 + 0xc) = (u_var5 + 0xc) + param_3;
    u_var7 = (u_var5 + 0xe) + param_2;
    (u_var5 + 0xe) = u_var7;
    u8_var3 = true;
    // LAB_1008_a146:
    u_var4 = u_var7;
    if (!u8_var3) {
        puVar8 = _PTR_LOOP_1050_03a0;
        alloc_mem_1000_07fc(_PTR_LOOP_1050_03a0);
        u_var5 = puVar8;
        local_e = puVar8 & 0xffff | ctx.dx_reg << 0x10;
        if ((ctx.dx_reg | u_var5) == 0) {
            local_e = 0;
        } else {
            local_e = ctx.s_1_1050_389a;
            (u_var5 + 2) = &ctx.PTR_LOOP_1050_1008;
            (u_var5 + 4) = param_6;
            (u_var5 + 6) = param_5;
            (u_var5 + 10) = param_4;
            (u_var5 + 0xc) = param_3;
            (u_var5 + 0xe) = param_2;
            local_e = 0xad8e;
            (u_var5 + 2) = &ctx.PTR_LOOP_1050_1008;
            puVar8 = local_e;
        }
        u_var4 = puVar8;
        u_var1 = (i_var9 + 10);
        ppc_var2 = ((i_var9 + 10) + 8);
        ppc_var2(0x1000, u_var1, (u_var1 >> 0x10), local_e, (local_e >> 0x10));
    }
    if (param_6 == 0x14) {
        b_var6 = winapi_funcs::PostMessage16(0, 0xfc, 0x111, ctx.g_h_window);
        u_var4 = b_var6;
    }
    return u_var4;
}

pub fn create_win_1008_9760(in_win_struct: &mut win_struct_42) -> StructuredData {
    let mut win_handle_1: u16 = 0;
    let win_struct: win_struct_42;
    let mut u_var1: i32;

    // u_var1 = (in_win_struct >> 0x10);
    // win_struct = in_win_struct;
    if win_struct.win_handle_0x8 == 0 {
        let data: Vec<u8> = in_win_struct::to_le_bytes_vec();
        win_handle_1 = CreateWindowEx16(in_win_struct.style_0xb0, &in_win_struct.class_name_0x5b, &ctx.g_string_1050_039e, in_win_struct.style_0xac, in_win_struct.x_0xb4, in_win_struct.y_0xb6, in_win_struct.width_0xb8, in_win_struct.height_0xba, in_win_struct.parent_window_0xbc, in_win_struct.menu_handle_0xca,  ctx.g_h_instance_1050_038c, &data);
        in_win_struct.win_handle_0x8 = win_handle_1;
    }
    // if in_win_struct.win_handle_0x8 == 0 {
    //     win_handle_1 = call_fn_ptr_1000_24cd(0);
    // }
    let mut out: StructuredData = StructuredData::new();
    out.base = ctx.dx_reg;
    out.offset = win_handle_1;
    return out;
}

pub fn print_fn_1008_97c8(param_1: StructuredData) {
    let mut u_var1: u16;
    let mut paint_struct_1: PAINTSTRUCT16 = PAINTSTRUCT16::new();
    let wnd_handle = param_1.buffer[8..10];
    winapi_funcs::BeginPaint16(wnd_handle, paint_struct_1);
    paint_struct_1.hdc = (param_1 + 8);
    EndPaint(&paint_struct_1, unaff_ss);
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

// WARNING: Variable defined which should be unmapped: local_1e
