use std::intrinsics::offset;

use crate::{mixed_fn_1010_830a, sys1, winapi_funcs};
use crate::app_context::AppContext;
use crate::bool_funcs::check_and_set_global_1000_1fea;
use crate::cleanup::{ret_1040_78de, window_msg_func_1010_7300};
use crate::draw::draw2::win_fn_1020_7270;
use crate::err_funcs::error_check_1000_17ce;
use crate::func_ptr_funcs::call_fn_ptr_1000_24cd;
use crate::mem_funcs::{alloc_mem_1000_07fc, free_mem_1000_1b68, get_type_at_address, StructuredData};
use crate::other_funcs::{switch_stmt_1008_aaa8, switch_stmt_1008_d818};
use crate::pass::pass12_funcs::{pass1_1008_b38c, pass1_1008_b47a, pass1_1008_b4a0, pass1_1008_b61a, pass1_1008_b63a, pass1_1008_b820, pass1_1008_c6ae, pass1_1008_c6fa};
use crate::pass::pass13_funcs::{pass1_1008_941a, pass1_1008_b200, pass1_1008_b366};
use crate::pass::pass14_funcs::{pass1_1008_5784, pass1_1008_5b12, pass1_fn_1008_60e8, pass1_fn_1008_612e};
use crate::pass::pass15_funcs::{pass1_1020_ba7e, pass1_1020_bb16, pass1_1020_c42e, ret_one_1020_c3ae, switch_statement_1020_c3b4};
use crate::pass::pass16_funcs::{pass1_1028_62c8, pass1_1028_6408, pass1_1028_740c, pass1_1028_75bc, pass1_1028_78b8, pass1_1028_7c4e, pass1_1028_7dfc, pass1_1028_7fb6, pass1_1028_8d9e, pass1_1028_8dec, post_msg_1028_76da};
use crate::pass::pass17_funcs::{pass1_1030_1d58, pass1_1030_2242, pass1_1030_3694, pass1_1030_375a, pass1_1030_38b8, pass1_1030_38f2, pass1_1030_5b00, pass1_1030_6a2c, pass1_1030_6c1a, pass1_1030_6c4c, pass1_1030_73a8, pass1_1030_7c50, pass1_1030_7ddc, pass1_1030_835a};
use crate::pass::pass19_funcs::pass1_1038_df5c;
use crate::pass::pass20_funcs::{pass1_1010_ae12, pass1_1010_ae92, pass1_1018_0ad4};
use crate::pass::pass3_funcs::{pass1_1028_45e2, pass1_1028_5ca0};
use crate::pass::pass4_funcs::{pass1_1028_bc90, pass1_1028_d01a, pass1_1028_d2b0, pass1_1028_dc52, pass1_1028_e1ec, pass1_1028_e4ec};
use crate::pass::pass5_funcs::{pass1_1030_bcae, pass1_1030_bd74};
use crate::pass::pass6_funcs::{pass1_1038_349e, pass1_1038_387e, pass1_1038_3ba0, pass1_1038_4d6e, pass1_1038_4e78, pass1_1038_4f54};
use crate::pass::pass7_funcs::{pass1_1018_1c9a, pass1_1018_1e78, pass1_1018_34a6, pass1_1018_3a7a};
use crate::pass::pass8_funcs::{pass1_1008_eb74, pass1_1010_043a, pass1_1010_089e, pass1_1010_1ea6, pass1_1010_4df0, pass1_1010_5120, pass1_1010_519a, pass1_1010_52fc, pass1_1010_531c};
use crate::pass::pass9_funcs::pass1_1008_df4a;
use crate::pass::pass_funcs::{pass1_1000_3d7a, pass1_fn_1000_3e2c};
use crate::prog_structs::prog_structs_12::Struct806;
use crate::prog_structs::prog_structs_15::{Struct1156, Struct921};
use crate::prog_structs::prog_structs_16::Struct493;
use crate::prog_structs::prog_structs_17::Struct1115;
use crate::prog_structs::prog_structs_2::{Struct199, Struct7};
use crate::prog_structs::prog_structs_21::Struct74;
use crate::prog_structs::prog_structs_23::{Struct1090, Struct1157};
use crate::prog_structs::prog_structs_24::{Struct103, Struct2111};
use crate::prog_structs::prog_structs_26::Struct1159;
use crate::prog_structs::prog_structs_27::Struct816;
use crate::prog_structs::prog_structs_28::Struct300;
use crate::prog_structs::prog_structs_7::Struct376;
use crate::sound_funcs::{mci_send_cmd_1008_5c5c, mci_send_command_1008_5c7c, mci_send_command_1008_5c9e};
use crate::string_ops1::{copy_string_1000_3d3e, load_string_1008_b1f0};
use crate::struct_ops1::{init_struct_1000_1902, process_struct_1000_179c, process_struct_1008_e586, set_struct_1008_0000};
use crate::struct_ops2::{pass1_1038_df86, process_struct_1010_20ba, process_struct_1040_7728};
use crate::sys_structs::WNDCLASS16;
use crate::typedefs::{ATOM, HANDLE16, HINSTANCE16, HWND16, LPARAM, LRESULT, WPARAM16};
use crate::ui_funcs::ui1::{destroy_win_1040_7b98, free_proc_inst_1040_911e, mixed_1040_8520, win_fn_1010_71d6, win_fn_1040_8b92};
use crate::ui_funcs::ui2::{destroy_menu_func_1020_795c, msg_box_1040_64ca, pass1_1038_af40, pass1_1038_e03e, send_dialog_item_msg_1038_844a, send_dlg_item_msg_1038_8164, set_cursor_1038_bc30, win_fn_1038_8f74};
use crate::util::{CARRY1, CONCAT11, CONCAT12, CONCAT13, CONCAT22, CONCAT31, SBORROW1, SBORROW2, SUB21, SUB42};
use crate::winapi_funcs::{GetClassInfo16, RegisterClass16};

pub fn process_win_msg_1008_9498() -> WPARAM16 {
    let mut b_result: u16;
    let b_var1: bool;
    let i_var2: u16;
    let mut h_wnd_dlg: u16;
    let mut local_18: u16;
    let mut message: [u8; 18];
    let mut local_4: u16;

    // LAB_1008_949c:
    b_result = GetMessage16(0, 0, 0, message);
    if (b_result == 0) {
        return message._4_2_;
    }
    if ((ctx._PTR_LOOP_1050_1040 + 8) != 0) {}
    // goto code_r0x100894cd;
    // goto LAB_1008_94dc;
    // code_r0x100894cd:
    b_var1 = IsDialogMessage16(message, h_wnd_dlg);
    if (b_var1 == 0) {
        // LAB_1008_94dc:
        if (g_haccel_1050_0398 != 0x0) {
            i_var2 = TranslateAccelerator16(
                CONCAT22(h_wnd_dlg, message),
                g_haccel_1050_0398,
                ctx.g_h_window,
            );
            if (i_var2 != 0) {}
            // goto LAB_1008_949c;
        }
        TranslateMessage16(message);
        DispatchMessage16(message);
    }
    // goto LAB_1008_949c;
}

pub fn process_win_msg_1008_9510(param_1: &mut i32, param_2: &mut String) {
    let b_var1: bool;
    let i_var2: u16;
    let unaff_ss: HWND16;
    let mut local_16: u16;
    let mut msg: [u8; 18];

    // LAB_1008_9578:
    if (unsafe { *param_1 != 0 }) {
        b_var1 = GetMessage16(0, 0, 0, msg);
        if (b_var1 != 0) {
            if ((ctx._PTR_LOOP_1050_1040 + 8) != 0) {}
            // goto code_r0x10089538;
            // goto LAB_1008_9547;
        }
    }
    return;
    // code_r0x10089538:
    b_var1 = IsDialogMessage16(msg, unaff_ss);
    if (b_var1 == 0) {
        // LAB_1008_9547:
        if (g_haccel_1050_0398 != 0x0) {
            i_var2 =
                TranslateAccelerator16(CONCAT22(unaff_ss, msg), g_haccel_1050_0398, ctx.g_h_window);
            if (i_var2 != 0) {}
            // goto LAB_1008_9578;
        }
        TranslateMessage16(msg);
        DispatchMessage16(msg);
    }
    // goto LAB_1008_9578;
}

pub fn send_win_msg_1008_9640(param_1: u32, param_2: WPARAM16) {
    let mut u_var1: u16;

    u_var1 = (param_1 >> 0x10);
    if ((param_1 + 8) != 0) {
        winapi_funcs::SendMessage16(0, param_2, 0x86, (param_1 + 8));
    }
    return;
}

pub fn send_win_msg_1008_84ba(param_1: u16, param_2: &mut HWND16) {
    let mut i_var1: i32;
    let mut u_var2: u16;
    let mut local_4: u16;

    u_var2 = (param_1 >> 0x10);
    i_var1 = param_1;
    if ((*(i_var1 + 4) & 4) == 0) {
        if ((*(i_var1 + 4) & 8) == 0) {
            return;
        }
        local_4 = 1;
    } else {
        local_4 = 0;
    }
    winapi_funcs::SendMessage16(*(i_var1 + 2), local_4, 0x115, *param_1);
    return;
}

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
    b_var1 = GetClassInfo16(&local_1c, CONCAT22(local_6, unaff_ss), param_1._2_2_);
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
        local_4 = param_1._2_2_;
        AVar2 = RegisterClass16(&local_1c);
        // if AVar2 == 0 {
        //     call_fn_ptr_1000_24cd(0);
        // }
    }
    return;
}

pub unsafe fn free_proc_and_check_err_1008_3cd6(param_1: &mut Struct7, param_2: u8) -> &mut Struct7{
    free_proc_inst_1040_911e(param_1);
    if (param_2 & 1) != 0 {
        error_check_1000_17ce(ctx, param_1);
    }
    return param_1;
}

pub fn pos_msg_1008_3d20(param_1: u32) {
    let mut u_var1: u16;

    u_var1 = (param_1 >> 0x10);
    winapi_funcs::PostMessage16(0, (param_1 + 0xcc), 0x111, (param_1 + 0xbc));
    return;
}

pub unsafe fn process_win_msg_1008_54aa(
    ctx: &mut AppContext,
    struct_param_1: &mut Struct97,
    param_2: u16,
    param_3: u16,
    param_4: u16,
    in_h_instance: u16,
) {
    let pp_var1: fn();
    // let struct_a: Struct199;
    let mut base_var2: u16;
    let mut local_1e: u32;
    let mut local_10: u16;
    let mut local_e: u16;
    // let func_ptr_c: &mut Vec<u8>;
    let mut temp_5f75f7f23c: u32;

    if param_3_00 != 0 {
        return;
    }
    sys1::dos3_call_1000_435c(ctx, None);
    sys1::set_global_uint_1000_4d0c(ctx, None);
    check_and_set_global_1000_1fea(ctx);
    init_struct_1000_1902(None, 0x32, 0, 0x12);
    _PTR_LOOP_1050_03a0 = CONCAT22(local_DX_57, ctx.ax_reg);
    init_struct_1000_1902(None, 100, 0, 0xc);
    _PTR_LOOP_1050_029c = CONCAT22(local_DX_81, ctx.ax_reg);
    init_struct_1000_1902(None, 100, 0, 0x10);
    _PTR_LOOP_1050_4fb8 = CONCAT22(local_DX_105, ctx.ax_reg);
    init_struct_1000_1902(None, 100, 0, 0xe);
    _PTR_LOOP_1050_68a2 = CONCAT22(local_DX_129, ctx.ax_reg);
    init_struct_1000_1902(None, 500, 0, 0x42);
    _PTR_LOOP_1050_5744 = CONCAT22(local_DX_153, ctx.ax_reg);
    init_struct_1000_1902(None, 0x32, 0, 6);
    ctx.g_h_instance_1050_038c = in_h_instance;
    ctx.PTR_LOOP_1050_038e = param_3_00;
    ctx.PTR_LOOP_1050_0390 = struct_param_1;
    let struct_a: Struct103 = get_type_at_address::<Struct103>(ctx.dx_reg as u32);
    ctx.PTR_LOOP_1050_5768 = ctx.ax_reg;
    ctx.PTR_LOOP_1050_576a = ctx.dx_reg;
    pass1_fn_1008_60e8();
    ctx._PTR_LOOP_1050_0392 = CONCAT22(struct_a, struct_param_1);
    process_struct_1000_179c(0xc, &mut struct_a);
    if (struct_a | *struct_param_1) == 0 {
        *struct_param_1 = 0;
        base_var2 = 0;
    } else {
        set_struct_1008_0000(struct_param_1);
        base_var2 = local_DX_249;
    }
    let func_ptr_c = CONCAT22(base_var2, *struct_param_1);
    if (ctx._PTR_LOOP_1050_0392 != 0) {
        // WARNING: Load size is inaccurate
        pp_var1 = (*func_ptr_c + 4);
        pp_var1(0x1000, struct_param_1, base_var2, ctx._PTR_LOOP_1050_0392);
    }
    local_1e = CONCAT22(base_var2, struct_param_1);
    // WARNING: Load size is inaccurate
    temp_5f75f7f23c = *func_ptr_c;
    // TODO
    // pp_var1 = temp_5f75f7f23c + 4;
    // (pp_var1)();
    process_win_msg_1008_9498(local_1e);
    if (func_ptr_c != 0) {
        pp_var1 = temp_5f75f7f23c;
        (**pp_var1)(0x1000, struct_param_1, base_var2, 1);
    }
    free_mem_1000_1b68(ctx._PTR_LOOP_1050_03a0);
    free_mem_1000_1b68(ctx._PTR_LOOP_1050_029c);
    free_mem_1000_1b68(ctx._PTR_LOOP_1050_4fb8);
    free_mem_1000_1b68(ctx._PTR_LOOP_1050_68a2);
    free_mem_1000_1b68(ctx._PTR_LOOP_1050_5744);
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
        local_4 = param_1._2_2_;
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

    u_var7 = (param_1 >> 0x10);
    i32_var6 = param_1;
    if (unsafe { *param_1 == 4 }) {
        u_var1 = (i32_var6 + 0xc);
        h_var4 = winapi_funcs::GetProp16(s_thisHi_1050_5e6f, (i32_var6 + 10));
        HVar5 = winapi_funcs::GetProp16(s_thisLo_1050_5e68, (i32_var6 + 10));
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

pub fn send_win_msg_1040_93e6(param_1: u32) -> LRESULT {
    let mut u_var1: u16;
    let LVar2: LRESULT;

    u_var1 = (param_1 >> 0x10);
    LVar2 = winapi_funcs::SendMessage16(0, (param_1 + 0x1c), 0x111, (param_1 + 0x1a));
    return LVar2;
}

pub fn send_msg_1040_9404(param_1: u32) -> LRESULT {
    let mut u_var1: u16;
    let LVar2: LRESULT;
    u_var1 = (param_1 >> 0x10);
    LVar2 = winapi_funcs::SendMessage16(0, (param_1 + 0x1c), 0x111, (param_1 + 0x1a));
    return LVar2;
}

pub unsafe fn make_proc_inst_1040_8fb8(
    in_Struct1: &mut Struct74,
    param_2: u16,
    param_3: u32,
    param_4: u16,
    param_5: u16,
    param_6: u16,
    param_7: u16,
    param_8: u16,
) {
    let mut u_var1: u16;
    let mut u_var2: u16;
    let mut hinstance: u16;
    let mut in_stack_00000006: u16;

    *_in_Struct1 = ctx.s_1_1050_389a;
    in_Struct1.field_0x2 = &ctx.PTR_LOOP_1050_1008;
    &in_Struct1.field_0x4 = 0;
    &in_Struct1.field_0x8 = 0;
    &in_Struct1.field_0xc = 0;
    &in_Struct1.field_0x10 = 0;
    in_Struct1.field_0x14 = 0;
    in_Struct1.field_0x18 = 0;
    in_Struct1.field_0x1a = param_8;
    in_Struct1.field_0x1c = param_7;
    in_Struct1.field_0x36 = 5;
    u_var1 = 0;
    in_Struct1.field_0x38 = 0;
    in_Struct1.field_0x3a = 0;
    in_Struct1.field_0x3c = 2;
    in_Struct1.field_0x3e = 0;
    in_Struct1.field_0x40 = param_2;
    *_in_Struct1 = 0x9800;
    in_Struct1.field_0x2 = &ctx.PTR_LOOP_1050_1040;
    u_var2 = in_Struct1.field_0x36;
    in_Struct1.field_0x28 = u_var2;
    in_Struct1.field_0x26 = u_var2;
    in_Struct1.field_0x2c = 0;
    in_Struct1.field_0x2a = 0;
    if ((param_6 != 0) && (param_5 != 0)) {
        in_Struct1.field_0x38 = 1;
        mixed_fn_1010_830a(ctx._g_struct_73_1050_14cc, param_6);
        in_Struct1.field_0x8 = u_var1;
        in_Struct1.field_0xa = ctx.dx_reg;
        hinstance = 0x1010;
        mixed_fn_1010_830a(ctx._g_struct_73_1050_14cc, param_5);
        in_Struct1.field_0xc = u_var1;
        in_Struct1.field_0xe = ctx.dx_reg;
        if (param_4 == 0) {
            &in_Struct1.field_0x10 = 0;
            ctx.dx_reg = ctx.dx_reg;
        } else {
            hinstance = 0x1010;
            mixed_fn_1010_830a(ctx._g_struct_73_1050_14cc, param_4);
            in_Struct1.field_0x10 = u_var1;
            in_Struct1.field_0x12 = ctx.dx_reg;
            ctx.dx_reg = ctx.dx_reg;
        }
    }
    u_var2 = in_Struct1.field_0x36;
    in_Struct1.field_0x30 = u_var2;
    in_Struct1.field_0x2e = u_var2;
    in_Struct1.field_0x32 = 0;
    if (param_3 != 0) {
        hinstance = &ctx.PTR_LOOP_1050_1008;
        pass1_fn_1008_60e8(param_3);
        in_Struct1.field_0x4 = u_var2;
        in_Struct1.field_0x6 = ctx.dx_reg;
    }
    in_Struct1.field_0x22 = 0;
    in_Struct1.field_0x1e = 0;
    in_Struct1.field_0x20 = 0;
    if (_g_proc_inst_1050_5e18 == 0) {
        _g_proc_inst_1050_5e18 =
            winapi_funcs::MakeProcInstance16(hinstance, CONCAT22(0x9684, ctx.g_h_instance_1050_038c));
    }
    PTR_LOOP_1050_5e16 = PTR_LOOP_1050_5e16 + 1;
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
                mixed_fn_1010_830a(ctx._g_struct_73_1050_14cc, in_stack_0000bf2c);
                u_var12 = (pu_var15 >> 0x10);
                (pu_var15 + 8) = u_var5;
                (pu_var15 + 10) = ctx.dx_reg;
                unaff_cs = 0x1010;
                mixed_fn_1010_830a(ctx._g_struct_73_1050_14cc, in_stack_0000bf2a);
                u_var12 = (pu_var15 >> 0x10);
                i_var9 = pu_var15;
                (i_var9 + 0xc) = u_var5;
                (i_var9 + 0xe) = ctx.dx_reg;
                if (in_ax == 0) {
                    (i_var9 + 0x10) = 0;
                    u_var7 = ctx.dx_reg;
                } else {
                    unaff_cs = 0x1010;
                    mixed_fn_1010_830a(ctx._g_struct_73_1050_14cc, in_ax);
                    u_var12 = (pu_var15 >> 0x10);
                    (pu_var15 + 0x10) = u_var5;
                    (pu_var15 + 0x12) = ctx.dx_reg;
                    u_var7 = ctx.dx_reg;
                }
            }
            u_var12 = (pu_var15 >> 0x10);
            local_bx_272 = pu_var15;
            u_var5 = local_bx_272.field_0x36;
            local_bx_272.field_0x30 = u_var5;
            local_bx_272.field_0x2e = u_var5;
            local_bx_272.field_0x32 = 0;
            if (lVar16 != 0) {
                unaff_cs = &ctx.PTR_LOOP_1050_1008;
                pass1_fn_1008_60e8();
                u_var12 = (pu_var15 >> 0x10);
                (pu_var15 + 4) = u_var5;
                (pu_var15 + 6) = u_var7;
            }
            u_var5 = (pu_var15 >> 0x10);
            i_var9 = pu_var15;
            (i_var9 + 0x22) = 0;
            (i_var9 + 0x1e) = 0;
            (i_var9 + 0x20) = 0;
            if ctx._g_proc_inst_1050_5e18 == 0 {
                let hinst: HINSTANCE16 = CONCAT22(0x9684, ctx.g_h_instance_1050_038c) as HINSTANCE16;
                ctx._g_proc_inst_1050_5e18 =
                    winapi_funcs::MakeProcInstance16(ctx.code_seg_reg, hinst);
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
    // u_var2 = (param_1 >> 0x10);
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
    PTR_LOOP_1050_5df8 = 0x0;
    return u_var1 & 0xffff0000 | param_1 & 0xffff;
}

pub fn post_win_msg_1040_7f1c(param_1: u32, param_2: i32) -> u16 {
    let mut i_var1: i32;
    let mut u_var2: u16;

    u_var2 = (param_1 >> 0x10);
    i_var1 = param_1;
    if ((i_var1 + 0x78) != 0) {
        return 0;
    }
    if ((i_var1 + 0x60) != param_2) {
        (i_var1 + 0x60) = param_2;
        winapi_funcs::PostMessage16(0, 0, 0x85, (i_var1 + 6));
    }
    return 1;
}

pub unsafe fn post_win_msg_1040_7f56(param_1: &mut StructuredData, param_2: &String) {
    let mut str_var1: String = param_1::get_string(0x10, None);
    let mut hwnd: HWND16 = param_1::get_u16(6) as HWND16;
    copy_string_1000_3d3e(&mut str_var1, param_2);
    // winapi_funcs::PostMessage16(0, 0, 0x85, (param_1 + 6));
    winapi_funcs::PostMessage16(hwnd, 0x85, 0, 0);
    return;
}

pub fn post_win_msg_1040_7b3c(param_1: &mut u32, param_2: u16, param_3: &mut String, param_4: i32) -> u16 {
    let pp_var1: fn();

    if (param_2 == 1) || (param_2 == 2) {
        unsafe {
            pp_var1 = (*param_1 + 0x14);
        }
        (**pp_var1)();
    } else {
        if param_2 == 0x6f {
            unsafe {
                pp_var1 = (*param_1 + 0x2c);
            }
            (**pp_var1)();
        } else {
            if param_2 != 0x12e {
                return 0;
            }
            winapi_funcs::PostMessage16(0, 0xf060, 0x112, (param_1 + 6));
        }
    }
    return 1;
}

pub fn post_win_msg_1040_672e(ctx: &mut AppContext, param_1: &mut u32, param_2: u16, param_3: &mut String, param_4: u32) {
    let mut u_var1: u16;

    if param_3 == (ctx.s_vrpal_bmp_1050_183a + 7) {
        msg_box_1040_64ca(param_1);
    } else {
        if param_3 == (ctx.s_logo_bmp_1050_1850 + 1) {
            u_var1 = 0x2a;
        } else {
            if param_3 != (ctx.s_logo_bmp_1050_1850 + 2) {
                post_win_msg_1040_7b3c(param_1, param_2, param_3);
                return;
            }
            u_var1 = 0x29;
        }
        pass1_1038_af40(ctx._g_Struct112_a, *(param_1 + 8), u_var1);
        winapi_funcs::PostMessage16(0, 2, 0x111, (param_1 + 6));
    }
    return;
}

pub unsafe fn send_win_msg_1040_4cf4(param_1: u32) {
    let mut u_var1: u32;
    let mut u_var2: u32;
    let mut u_var3: u32;
    let h_wnd: HWND16;
    let mut i_var4: i32;
    let mut i_var5: i32;
    let mut u_var6: u16;
    let mut u_var7: u16;
    let mut unaff_ss: u16;
    let LVar8: LRESULT;
    let mut local_58: u16;
    let mut local_56: u16;
    let mut local_54: u16;
    let mut local_52: [u8; 80];

    u_var6 = (param_1 >> 0x10);
    i_var5 = param_1;
    h_wnd = winapi_funcs::GetDlgItem16(6000, (i_var5 + 6));
    LVar8 = winapi_funcs::SendMessage16(0, 0, 0x407, h_wnd);
    if (LVar8 != 0xffff) {
        winapi_funcs::SendMessage16(CONCAT22(unaff_ss, local_52), LVar8, 0x408, h_wnd);
    }
    u_var3 = (i_var5 + 0x90);
    u_var1 = (i_var5 + 0x94);
    i_var4 = pass1_1010_ae12(
        u_var1,
        (u_var1 >> 0x10),
        CONCAT22(unaff_ss, local_52),
        (u_var3 + 10),
    );
    if (i_var4 != -1) {
        u_var1 = (i_var5 + 0x90);
        u_var7 = (u_var1 >> 0x10);
        i_var4 = u_var1;
        u_var2 = (i_var4 + 6);
        local_58 = u_var2;
        pass1_1010_ae92((i_var5 + 0x94), local_58, (i_var4 + 10), u_var2);
    }
    return;
}

pub unsafe fn win_fn_1040_3374(param_1: u32, param_2: &mut u32, param_3: HWND16) {
    let pp_var1: fn();
    let mut u_var2: Struct7;
    let mut u_var3: u32;
    let mut u_var4: u16;
    let mut u_var5: u16;
    let LVar6: LRESULT;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    u_var5 = SUB42(offset, 0);
    LVar6 = winapi_funcs::SendMessage16(0, 0, 0x40b, param_3);
    u_var2 = LVar6;
    u_var4 = (param_2 >> 0x10);
    unsafe {
        pp_var1 = (*param_2 + 0x10);
    }
    (**pp_var1)(offset, param_2, u_var4);
    _local_6 = CONCAT22(ctx.dx_reg, u_var2);
    local_a = 0;
    loop {
        if (_local_6 <= local_a) {
            return;
        }
        unsafe {
            pp_var1 = (*param_2 + 4);
        }
        u_var3 = _local_6;
        (**pp_var1)(u_var5, param_2, u_var4, local_a, (local_a >> 0x10));
        u_var2 = u_var3;
        pass1_1018_3a7a(
            *(param_1 + 0x96),
            CONCAT13((ctx.dx_reg >> 8), CONCAT12(ctx.dx_reg, u_var2)),
        );
        LVar6 = winapi_funcs::SendMessage16(CONCAT22(ctx.dx_reg, u_var2), 0, 0x403, param_3);
        u_var5 = 0x1000;
        error_check_1000_17ce(ctx, &mut u_var2);
        if LVar6 == -1 {
            break;
        }
        if LVar6 == -2 {
            return;
        }
        local_a = local_a + 1;
    }
    return;
}

pub fn send_msg_1040_323c(param_1: u32) -> LRESULT {
    let mut i_var1: i32;
    let mut u_var2: u16;
    let LVar3: LRESULT;
    let mut local_a: u16;
    let mut local_6: u16;
    u_var2 = (param_1 >> 0x10);
    i_var1 = param_1;
    LVar3 = winapi_funcs::SendMessage16(0, 0, 0x407, (i_var1 + 0x92));
    winapi_funcs::SendMessage16(0, 0, 0x407, (i_var1 + 0x94));
    winapi_funcs::SendMessage16(
        param_1 & 0xffff0000 | (i_var1 + 0x9a),
        LVar3,
        0x408,
        (i_var1 + 0x92),
    );
    LVar3 = winapi_funcs::SendMessage16(
        param_1 & 0xffff0000 | (i_var1 + 0x19a),
        0x408,
        0x408,
        (i_var1 + 0x94),
    );
    return LVar3;
}

pub unsafe fn send_msg_1040_1696(param_1: u32, param_2: HWND16) {
    let mut u_var1: u32;
    let p_uvar2: u16;
    let mut u_var3: u16;
    let mut u_var4: u32;
    let mut u_var5: u16;
    let mut unaff_ss: u16;
    let w_var6: WPARAM16;
    let mut u_var7: u16;
    let mut local_22: u32;
    let mut local_1e: u32;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u32;
    let mut local_c: u32;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    winapi_funcs::SendMessage16(0, 0, 0x40b, param_2);
    winapi_funcs::SendMessage16(0, 0, 0xb, param_2);
    local_4 = 0;
    pu_var2 = &local_4;
    u_var5 = (param_1 >> 0x10);
    pass1_1010_519a((param_1 + 0x8e), CONCAT22(unaff_ss, pu_var2));
    local_12 = 0;
    loop {
        if (local_4 <= local_12) {
            break;
        }
        u_var4 = (pu_var2 + local_12 * 2);
        w_var6 = 0;
        u_var7 = 0x403;
        u_var1 = (param_1 + 0x8e);
        str_fn_1010_5286(u_var1, (u_var1 >> 0x10), u_var4);
        winapi_funcs::SendMessage16(
            u_var4 & 0xffff | ctx.dx_reg << 0x10,
            w_var6,
            u_var7,
            param_2,
        );
        error_check_1000_17ce((u_var4 & 0xffff | ctx.dx_reg << 0x10));
        local_12 = local_12 + 1;
    }
    w_var6 = 0;
    u_var7 = 0x40a;
    u_var3 = local_4;
    load_string_1010_847e(
        ctx._g_struct_73_1050_14cc,
        (ctx._g_struct_73_1050_14cc >> 0x10),
        0x531,
    );
    winapi_funcs::SendMessage16(CONCAT22(ctx.dx_reg, u_var3), w_var6, u_var7, param_2);
    winapi_funcs::SendMessage16(0, 1, 0xb, param_2);
    return;
}

pub unsafe fn destroy_win_1040_13b2(param_1: u32, param_2: i32) {
    let mut u_var1: u32;
    let ppc_var2: fn();
    let mut u_var3: u32;
    let paVar4: Struct199;

    let struct_a: Struct103;
    let mut u_var5: i32;
    let struct_a_00: Struct103;
    let mut i32_var6: i32;
    let mut i_var7: i32;
    let mut u_var8: u16;
    let mut u_var9: u16;
    let mut unaff_cs: u16;
    let mut unaff_ss: u16;
    let LVar10: LRESULT;
    let mut local_232: u16;
    let mut local_230: u16;
    let paStack556: Struct199;
    let mut local_116: u32;
    let mut local_112: u32;
    let mut local_10e: u16;
    let mut local_10c: u16;
    let mut local_10a: u16;
    let mut local_108: u16;
    let mut local_106: u16;
    let mut local_104: u16;
    let mut local_102: u16;
    let local_100: [Struct199; 6];
    let mut local_ae: u16;
    let mut local_ac: u16;
    let local_aa: [Struct199; 6];
    let mut local_58: u16;
    let mut local_56: u16;
    let mut local_54: [u8; 82];

    i32_var6 = param_1;
    u_var8 = (param_1 >> 0x10);
    if (param_2 != 0) {
        local_56 = winapi_funcs::GetDlgItem16(0xfd2, (i32_var6 + 6));
        winapi_funcs::SendMessage16(CONCAT22(unaff_ss, local_54), 0x51, 0xd, local_56);
        local_58 = local_54;
        local_232._0_1_ = unaff_ss;
        local_232._1_1_ = (unaff_ss >> 8);
        pass1_fn_1000_3e2c(CONCAT13(local_232._1_1_, CONCAT12(local_232, local_58)));
        local_ac = winapi_funcs::GetDlgItem16((s_vrpal_bmp_1050_183a + 4), (i32_var6 + 6));
        LVar10 = winapi_funcs::SendMessage16(0, 0, 0x407, local_ac);
        local_ae = LVar10;
        if (local_ae != 0xffff) {
            winapi_funcs::SendMessage16(CONCAT22(unaff_ss, local_aa), local_ae, 0x408, local_ac);
        }
        local_ac = winapi_funcs::GetDlgItem16((s_vrpal_bmp_1050_183a + 5), (i32_var6 + 6));
        LVar10 = winapi_funcs::SendMessage16(0, 0, 0x407, local_ac);
        paVar4 = LVar10;
        local_ae = paVar4;
        if (paVar4 != 0xffff) {
            paStack556 = local_ac;
            LVar10 = winapi_funcs::SendMessage16(
                CONCAT13(local_232._1_1_, CONCAT12(local_232, local_100)),
                paVar4,
                0x408,
                local_ac,
            );
            paVar4 = LVar10;
        }
        paStack556 = (ctx._g_struct_73_1050_14cc >> 0x10);
        load_string_1010_847e();
        _local_104 = CONCAT22(ctx.dx_reg, paVar4);
        paStack556 = local_100;
        struct_a = local_aa;
        local_230._0_1_ = SUB21(struct_a, 0);
        pass1_1000_3d7a(local_230);
        if (paVar4 != 0x0) {
            paStack556 = _local_104;
            paVar4 = local_aa;
            local_230._0_1_ = SUB21(paVar4, 0);
            pass1_1000_3d7a(local_230);
            if (paVar4 != 0x0) {
                paStack556 = _local_104;
                paVar4 = local_100;
                pass1_1000_3d7a(0);
                if (paVar4 != 0x0) {
                    paStack556 = local_aa;
                    pass1_1010_531c((i32_var6 + 0x8e), CONCAT22(unaff_ss, paStack556));
                    paVar4 = local_100;
                    paStack556 = paVar4;
                    pass1_1010_52fc((i32_var6 + 0x8e), CONCAT22(unaff_ss, paVar4));
                    u_var1 = (i32_var6 + 0x8e);
                    paStack556 = (u_var1 >> 0x10);
                    pass1_1010_5120(u_var1, local_58);
                    _local_10a = CONCAT22(local_108, paVar4);
                    if (paVar4 == 0x0) {
                        paStack556 = 0x1010;
                        local_10c = struct_a_00;
                        process_struct_1000_179c(0xb4, &mut struct_a_00);
                        u_var5 = local_10c | paVar4;
                        local_10e = paVar4;
                        if (u_var5 == 0) {
                            paVar4 = 0x0;
                            u_var5 = 0;
                        } else {
                            paStack556 = 0x57b;
                            mixed_1040_8520(
                                CONCAT13((local_10c >> 8), CONCAT12(local_10c, paVar4)),
                                ctx.g_h_window,
                                0x30,
                                2,
                                0x57b,
                                0x7d2,
                            );
                        }
                        local_230._0_1_ = u_var5;
                        local_230._1_1_ = (u_var5 >> 8);
                        ppc_var2 = (CONCAT13(local_230._1_1_, CONCAT12(local_230, paVar4)) + 0x74);
                        paStack556 = paVar4;
                        ppc_var2();
                        return;
                    }
                    u_var3 = (i32_var6 + 0x8e);
                    local_112 = (u_var3 + 0x12);
                    u_var3 = (i32_var6 + 0x8e);
                    u_var9 = (u_var3 >> 0x10);
                    i_var7 = u_var3;
                    local_116 = (i_var7 + 0x16);
                    u_var3 = (i32_var6 + 0x8e);
                    local_106 = (u_var3 + 10);
                    paStack556 = local_116;
                    pass1_1028_8d9e(
                        CONCAT22(unaff_ss, &stack0xfdd2),
                        local_106,
                        local_112,
                        local_116 & 0xffff | *(i_var7 + 0x18) << 0x10,
                    );
                    paStack556 = &stack0xfdd2;
                    pass1_1030_835a(ctx._g_bool_1050_5748, CONCAT22(unaff_ss, paStack556));
                    paStack556 = &stack0xfdd2;
                    unaff_cs = SUB42(&PTR_LOOP_1050_1028, 0);
                    pass1_1028_8dec(CONCAT22(unaff_ss, paStack556));
                    // goto LAB_1040_1619;
                }
            }
        }
        paStack556 = 0x1000;
        unaff_cs = 0x1000;
        process_struct_1000_179c(0xb4, &mut struct_a);
        u_var5 = struct_a | paVar4;
        local_10c = struct_a;
        local_10e = paVar4;
        if (u_var5 == 0) {
            paVar4 = 0x0;
            u_var5 = 0;
        } else {
            paStack556 = 0x57b;
            mixed_1040_8520(
                CONCAT13((struct_a >> 8), CONCAT12(struct_a, paVar4)),
                ctx.g_h_window,
                0x30,
                2,
                0x57b,
                0x755,
            );
        }
        _local_10a = CONCAT22(u_var5, paVar4);
        ppc_var2 = (*_local_10a + 0x74);
        paStack556 = paVar4;
        ppc_var2();
    }
    // LAB_1040_1619:
    paStack556 = unaff_cs;
    winapi_funcs::DestroyWindow16((i32_var6 + 6));
    return;
}

pub fn post_win_msg_1040_0d5e(param_1: u32, param_2: i32) {
    if param_2 != 0 {
        winapi_funcs::PostMessage16(0, 1, 0x111, (param_1 + 8));
    }
    return;
}

pub unsafe fn send_win_msg_1038_ed8a(param_1: &mut u32, param_2: &mut String, param_3: u32) {
    let mut u_var1: i32;
    let mut u_var2: u16;
    let paVar3: Struct493;
    let mut i_var4: i32;
    let mut u_var5: i32;
    let ppVar6: Struct2111;
    let lVar7: u32;
    let h_wnd: HWND16;
    let mut in_stack_0000ffe2: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_10: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    h_wnd = ctx.g_h_window;
    if (param_3._2_2_ != 0x1c8) {
        if (param_3._2_2_ == 0x1c9) {
            ppVar6 = process_struct_1010_20ba(
                ctx._g_Struct372_1050_0ed0,
                CONCAT22(in_stack_0000ffe2, 0x2f),
            );
            u_var2 = (ppVar6 >> 0x10);
            u_var1 = (ppVar6 + 0x20);
            u_var5 = (ppVar6 + 0x22);
            if ((u_var5 | u_var1) == 0) {
                return;
            }
            paVar3 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, u_var1, u_var5);
            if ((u_var5 | paVar3) == 0) {
                return;
            }
            i_var4 = pass1_1030_5b00(CONCAT22(u_var5, paVar3));
            ppVar6 = process_struct_1010_20ba(
                ctx._g_Struct372_1050_0ed0,
                CONCAT22(in_stack_0000ffe2, i_var4),
            );
            if (ppVar6 == 0x0) {
                return;
            }
            lVar7 = pass1_1018_0ad4(ppVar6);
            if (lVar7 == 0) {
                return;
            }
            param_3._2_2_ = 0x72;
            h_wnd = (lVar7 + 8);
        } else {
            if (param_3._2_2_ != 0x1ca) {
                post_win_msg_1040_7b3c(param_1, param_2, param_2, param_3, param_3._2_2_);
                return;
            }
        }
    }
    winapi_funcs::SendMessage16(0, param_3._2_2_, 0x111, h_wnd);
    return;
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

    if (param_3._2_2_ == 0x1c4) {
        pp_var9 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(unaff_si, 0x2f));
        u_var12 = (pp_var9 >> 0x10);
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
                    pp_var9 = process_struct_1010_20ba(
                        ctx._g_Struct372_1050_0ed0,
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
        if (param_3._2_2_ == 0x1c5) {
            u_var12 = 0xe;
        } else {
            if (param_3._2_2_ != 0x1c6) {
                post_win_msg_1040_7b3c(param_1, param_2, (param_2 >> 0x10), param_3);
                return;
            }
            u_var12 = 0xd;
        }
        u_var14 = 0;
        u_var13 = 0;
        u_var10 = 0;
        u_var11 = 0;
        pp_var9 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, 0x32);
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

pub unsafe fn post_win_msg_1038_dcb0(param_1: u32) {
    let pp_var1: fn();
    let mut in_ax: i32;
    let mut u_var2: u16;
    let mut u_var3: u16;
    let mut u_var4: i32;
    let struct_a: Struct103;
    let mut i_var5: i32;
    let mut u_var6: u16;
    let mut u_var7: u16;
    let mut u_var8: u16;
    let u_var9: u8;
    let u_var10: u8;
    let mut u_var11: u16;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: [u8; 4];
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: [u8; 4];
    let mut local_6: u16;
    let mut local_4: u16;

    process_struct_1000_179c(0xb4, ctx.dx_reg);
    u_var4 = ctx.dx_reg | in_ax;
    i_var5 = param_1;
    u_var3 = (param_1 >> 0x10);
    if (u_var4 == 0) {
        u_var2 = 0;
        u_var4 = 0;
    } else {
        u_var2 = mixed_1040_8520(CONCAT22(ctx.dx_reg, in_ax), (i_var5 + 6), 4, 3, 0x634, 0x726);
    }
    _local_6 = CONCAT22(u_var4, u_var2);
    pass1_1008_941a(CONCAT22(unaff_ss, local_a), 1, 0x49);
    pp_var1 = (*_local_6 + 0x6c);
    _local_10 = (**pp_var1)();
    struct_a = (_local_10 >> 0x10);
    local_c = _local_10;
    if (local_c == 6) {
        process_struct_1000_179c(0xb4, &mut struct_a);
        u_var4 = (_local_10 >> 0x10) | _local_10;
        if (_local_10 == 0x0) {
            u_var3 = 0;
            u_var4 = 0;
        } else {
            u_var3 = mixed_1040_8520(_local_10, (i_var5 + 6), 0, 2, 0x634, 0x728);
        }
        _local_6 = CONCAT22(u_var4, u_var3);
        pass1_1008_941a(CONCAT22(unaff_ss, local_14), 1, 0x4a);
        pp_var1 = (*_local_6 + 0x6c);
        (**pp_var1)(
            &ctx.PTR_LOOP_1050_1008,
            _local_6,
            (_local_6 >> 0x10),
            local_14,
            unaff_ss,
        );
        u_var9 = 0;
        u_var10 = 0;
        u_var11 = 0x15;
        u_var7 = 1;
        u_var8 = 0;
        u_var2 = 0;
        u_var6 = 0;
        u_var3 = 0;
        _local_18 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, 0x37);
        sys1::post_win_msg_1008_a0e4(
            _local_18,
            CONCAT22(u_var2, u_var3),
            u_var6,
            u_var7,
            CONCAT13(u_var10, CONCAT12(u_var9, u_var8)),
            u_var11,
        );
        winapi_funcs::PostMessage16(0, 0xfc, 0x111, ctx.g_h_window);
        return;
    }
    process_struct_1000_179c(0xb4, &mut struct_a);
    u_var4 = (_local_10 >> 0x10) | _local_10;
    if (_local_10 == 0x0) {
        u_var3 = 0;
        u_var4 = 0;
    } else {
        u_var3 = mixed_1040_8520(_local_10, (i_var5 + 6), 0, 2, 0x634, 0x729);
    }
    _local_6 = CONCAT22(u_var4, u_var3);
    pass1_1008_941a(CONCAT22(unaff_ss, &local_18), 1, 0x4b);
    pp_var1 = (*_local_6 + 0x6c);
    (**pp_var1)(
        &ctx.PTR_LOOP_1050_1008,
        _local_6,
        (_local_6 >> 0x10),
        &local_18,
        unaff_ss,
    );
    return;
}

pub unsafe fn win_fn_1038_da68(param_1: &mut u32, param_2: u16, param_3_00: &mut String, param_3: &mut Struct103) {
    let pp_var1: fn();
    let u_var2: u8;
    let pu_var3: Vec<u8>;
    let mut u_var4: i32;
    let mut u_var5: u16;
    let pu_var6: Vec<u8>;
    let mut u_var7: u16;
    let mut local_16: [u8; 4];
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;

    if param_3_00 == 0x204 {
        bad1_1038_de20(
            CONCAT13((param_2 >> 8), CONCAT12(param_2, param_1)),
            (s_SOLDefaultWindowClass_1050_01fe + 6),
            param_3,
            param_3._2_2_,
        );
        return;
    }
    local_6 = 0;
    local_8 = 0;
    if (param_3._2_2_ == (s_New_failed_in_Op__Op__Simulator_1050_0110 + 0x11)) {
        local_a = 1;
        local_6 = 0x6ec0000;
        local_8 = 0x15;
    } else {
        if ((s_New_failed_in_Op__Op__Simulator_1050_0110 + 0x11) < param_3._2_2_) {
            if (param_3._2_2_ == (s_New_failed_in_Op__Op__Simulator_1050_0110 + 0x12)) {
                u_var7 = 0x14;
            } else {
                if (param_3._2_2_ != (s_New_failed_in_Op__Op__Simulator_1050_0110 + 0x13)) {
                    if (param_3._2_2_ + -0x125 == 0x0) {
                        pp_var1 = (ctx._g_struct_ptr_1050_02a0 + 4);
                        param_3._2_2_ = param_3._2_2_ + -0x125;
                        (**pp_var1)();
                        (param_1 + 0x90) = 1;
                        mci_send_cmd_1008_5c5c(ctx._g_struct_ptr_1050_02a0, 0x1db);
                        (param_1 + 0x8e) = 0x100;
                        ctx.dx_reg = ctx.dx_reg;
                    } else {
                        pu_var3 = param_3._2_2_ + -0x126;
                        if (pu_var3 == 0x0) {
                            (param_1 + 0x8e) = 0;
                            mci_send_command_1008_5c7c(ctx._g_struct_ptr_1050_02a0, 0xcb0001);
                            mixed_fn_1010_830a(ctx._g_struct_73_1050_14cc, 0x1f8);
                            local_e = CONCAT22(ctx.dx_reg, pu_var3);
                            ctx.dx_reg = ctx.dx_reg;
                            param_3._2_2_ =
                                // winapi_funcs::WinHelp16(0, 3, CONCAT22(ctx.dx_reg, pu_var3), (param_1 + 6));
                            winapi_funcs::WinHelp(param_1 + 6, pu_var3, CONCAT22(ctx.dx_reg, pu_var3), 3, 0);
                        } else {
                            if (param_3._2_2_ + -0x127 != 0x0) {}
                            // goto LAB_1038_dc20;
                            param_3._2_2_ = param_3._2_2_ + -0x127;
                            post_win_msg_1038_dcb0(param_1, param_2);
                            ctx.dx_reg = ctx.dx_reg;
                        }
                    }
                    // goto LAB_1038_dac3;
                }
                u_var7 = 0x28;
            }
            // LAB_1038_db1c:
            pu_var6 = pass1_1038_af40(ctx._g_Struct112_a, *(param_1 + 8), u_var7);
            ctx.dx_reg = (pu_var6 >> 0x10);
            param_3._2_2_ = pu_var6;
        } else {
            if (param_3._2_2_ + -0x100 == 0x0) {
                param_3._2_2_ = param_3._2_2_ + -0x100;
                if ((param_1 + 0x8e) == 0) {
                    u_var2 =
                        pass1_1010_1ea6(ctx._g_struct_ptr_1050_02a0, CONCAT22(param_2, param_1));
                    param_3._2_2_ = CONCAT31(extraout_var, u_var2);
                    (param_1 + 0x90) = 0;
                }
                local_6 = CONCAT22(0x72c, local_6);
                local_8 = 0x48;
            } else {
                if (param_3._2_2_ + -0x11c == 0x0) {
                    param_3._2_2_ = param_3._2_2_ + -0x11c;
                    pass1_1038_df86(CONCAT22(param_2, param_1));
                    ctx.dx_reg = ctx.dx_reg;
                } else {
                    if (param_3._2_2_ + -0x11d != 0x0) {
                        if (param_3._2_2_ == (s_New_failed_in_Op__Op__Simulator_1050_0110 + 0xe)) {
                            u_var7 = 0x1d;
                        } else {
                            if (param_3._2_2_
                                != (s_New_failed_in_Op__Op__Simulator_1050_0110 + 0x10))
                            {
                                // LAB_1038_dc20:
                                post_win_msg_1040_7b3c(
                                    &mut param_1,
                                    param_2,
                                    param_3_00,
                                    param_3,
                                    param_3._2_2_,
                                );
                                return;
                            }
                            u_var7 = 0x1c;
                        }
                        // goto LAB_1038_db1c;
                    }
                    param_3._2_2_ = param_3._2_2_ + -0x11d;
                    pass1_1038_df5c(CONCAT22(param_2, param_1));
                    ctx.dx_reg = ctx.dx_reg;
                }
            }
        }
    }
    // LAB_1038_dac3:
    if (local_6._2_2_ == 0) {
        return;
    }
    if local_6 == 0 {
        process_struct_1000_179c(0xb4, ctx.dx_reg);
        u_var4 = ctx.dx_reg | param_3._2_2_;
        local_12 = param_3._2_2_;
        local_10 = ctx.dx_reg;
        if u_var4 != 0 {
            u_var5 = SUB42(&ctx.PTR_LOOP_1050_1040, 0);
            mixed_1040_8520(
                CONCAT13((ctx.dx_reg >> 8), CONCAT12(ctx.dx_reg, param_3)),
                (param_1 + 6),
                0,
                2,
                0x634,
                local_6,
            );
            // goto LAB_1038_dc37;
        }
    } else {
        process_struct_1000_179c(0xb4, ctx.dx_reg);
        u_var4 = ctx.dx_reg | param_3._2_2_;
        local_12 = param_3._2_2_;
        local_10 = ctx.dx_reg;
        if u_var4 != 0 {
            u_var5 = &ctx.PTR_LOOP_1050_1040;
            mixed_1040_8520(
                param_3,
                (param_1 + 6),
                0,
                3,
                0x634,
                local_6._2_2_,
            );
            // goto LAB_1038_dc37;
        }
    }
    u_var5 = 0x1000;
    param_3._2_2_ = 0x0;
    u_var4 = 0;
    // LAB_1038_dc37:
    local_e = CONCAT22(u_var4, param_3._2_2_);
    if (local_8 == 0) {
        pp_var1 = (*local_e + 0x74);
        (**pp_var1)(u_var5, param_3._2_2_, u_var4);
    } else {
        pass1_1008_941a(CONCAT22(unaff_ss, local_16), 1, local_8);
        pp_var1 = (*local_e + 0x6c);
        (**pp_var1)(
            &ctx.PTR_LOOP_1050_1008,
            local_e,
            (local_e >> 0x10),
            local_16,
        );
    }
    return;
}

pub fn post_win_msg_1038_d840(param_1: Vec<u8>, param_2: i32) {
    let mut i_var1: i32;
    let mut u_var2: u16;

    i_var1 = param_1;
    u_var2 = (param_1 >> 0x10);
    if (param_2 == 0x10) {
        if ((i_var1 + 0x8e) != 0) {
            winapi_funcs::PostMessage16(0, (i_var1 + 0x8e), 0x111, (i_var1 + 6));
            (i_var1 + 0x8e) = 0;
            return;
        }
    } else {
        if (param_2 < 0x11) {
            if (param_2 == 0x1) {
                (i_var1 + 0x90) = 0;
                (i_var1 + 0x92) = 0;
                return;
            }
            if (param_2 == '\x03') {
                pass1_1038_e03e(param_1);
                return;
            }
        }
    }
    return;
}

pub fn win_fn_1038_d118(param_1: u32, param_2: u32, param_3: HWND16) {
    let pp_var1: fn();
    let mut u_var2: u32;
    let mut cVar3: u8;

    let h_var4: HANDLE16;
    let HVar5: HANDLE16;
    let mut u_var6: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    h_var4 = winapi_funcs::GetProp16(CONCAT22(in_ax, 0x5bf3), param_3);
    HVar5 = winapi_funcs::GetProp16(CONCAT22(in_ax, 0x5bec), param_3);
    _local_6 = CONCAT22(h_var4, HVar5);
    if (param_2._2_2_ == 0x30) {
        if (param_2 == 0) {
            return;
        }
        SetProp16(param_2, CONCAT22(in_ax, 0x5c06), param_3);
        return;
    }
    u_var6 = (param_1 >> 0x10);
    if (param_2._2_2_ < 0x31) {
        cVar3 = (param_2 >> 0x10);
        if (cVar3 == 0x2) {
            if ((h_var4 | HVar5) != 0) {
                u_var2 = *_local_6;
                pp_var1 = u_var2 + 6;
                (**pp_var1)(
                    offset,
                    HVar5,
                    h_var4,
                    param_1,
                    u_var6,
                    param_2,
                    param_2._2_2_,
                );
                if (_local_6 != 0x0) {
                    pp_var1 = u_var2;
                    (**pp_var1)(offset, HVar5, h_var4, 1);
                }
            }
            h_var4 = winapi_funcs::GetProp16(CONCAT22(in_ax, 0x5bfa), param_3);
            if (h_var4 == 0) {
                return;
            }
            winapi_funcs::DeleteObject16(h_var4);
            winapi_funcs::RemoveProp16(CONCAT22(in_ax, 0x5c00), param_3);
            return;
        }
        if (cVar3 == '\x06') {
            if ((param_2 != 1) && (param_2 != 2)) {
                u_var2 = &PTR_LOOP_1050_5bc8;
                (u_var2 + 8) = 0;
                return;
            }
            u_var2 = &PTR_LOOP_1050_5bc8;
            (u_var2 + 8) = param_3;
            return;
        }
    }
    if ((h_var4 | HVar5) != 0) {
        pp_var1 = (*_local_6 + 0xc);
        (**pp_var1)(
            offset,
            HVar5,
            h_var4,
            param_1,
            u_var6,
            param_2,
            param_2._2_2_,
        );
    }
    return;
}

pub fn call_win_proc_1038_d020(param_1: u16, param_2: u32, param_3: u32) {
    let pp_var1: fn();

    let HVar2: HANDLE16;
    let win_proc: &mut Vec<u8>;
    let HVar3: HANDLE16;
    let mut u_var4: i32;
    let LVar5: LRESULT;
    let mut u_var6: u32;
    let mut local_e: u32;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    HVar2 = winapi_funcs::GetProp16(CONCAT22(in_ax, 0x5bd7), param_3._2_2_);
    win_proc = winapi_funcs::GetProp16(
        CONCAT13((in_ax >> 8), CONCAT12(in_ax, 0x5bd0)),
        param_3._2_2_,
    );
    _local_6 = CONCAT22(HVar2, win_proc);
    HVar2 = winapi_funcs::GetProp16(CONCAT22(in_ax, 0x5be5), param_3._2_2_);
    HVar3 = winapi_funcs::GetProp16(CONCAT22(in_ax, 0x5bde), param_3._2_2_);
    _local_a = CONCAT22(HVar2, HVar3);
    if ((HVar2 | HVar3) != 0) {
        local_e = 0;
        if (param_3 == 0x19) {
            pp_var1 = (*_local_a + 0x34);
            local_e = (**pp_var1)(offset, HVar3, HVar2, param_1, param_2);
        } else {
            if (param_3 == 0x86) {
                pp_var1 = (*_local_a + 0x20);
                u_var4 = (**pp_var1)(offset, HVar3, HVar2, param_2._2_2_);
                // goto LAB_1038_d10e;
            }
            if ((param_3 == 0x112) && ((param_2._2_2_ & 0xfff0) == 0xf140)) {
                LVar5 = winapi_funcs::SendMessage16(0, 0xf140, 0x112, &ctx.g_h_window);
                u_var4 = (LVar5 == 0);
                // goto LAB_1038_d10e;
            }
        }
        if (local_e != 0) {
            return local_e;
        }
    }
    if (_local_6 != 0) {
        u_var6 = CallWindowProc16(
            CONCAT22(param_2, param_1),
            param_2._2_2_,
            param_3,
            param_3._2_2_,
            win_proc,
        );
        return u_var6;
    }
    u_var4 = 0;
    // LAB_1038_d10e:
    return u_var4;
}

pub fn make_proc_inst_1038_cf6c(param_1: &mut u16) {
    let pu_var1: Vec<u8>;
    let pu_var2: Vec<u8>;
    let unaff_cs: HANDLE16;
    let pvVar3: &mut Vec<u8>;

    pu_var2 = (param_1 >> 0x10);
    pu_var1 = param_1;
    unsafe {
        *param_1 = ctx.s_1_1050_389a;
    }
    (pu_var1 + 2) = &ctx.PTR_LOOP_1050_1008;
    (pu_var1 + 4) = 0;
    (pu_var1 + 8) = 0;
    unsafe {
        *param_1 = 0xd23e;
    }
    (pu_var1 + 2) = &PTR_LOOP_1050_1038;
    PTR_LOOP_1050_5bc8 = pu_var1;
    PTR_LOOP_1050_5bca = pu_var2;
    pvVar3 = winapi_funcs::MakeProcInstance16(unaff_cs, CONCAT22(0xd116, ctx.g_h_instance_1050_038c));
    (pu_var1 + 4) = pvVar3;
    (pu_var1 + 6) = (pvVar3 >> 0x10);
    pvVar3 = winapi_funcs::MakeProcInstance16(offset, CONCAT22(0xd01e, ctx.g_h_instance_1050_038c));
    PTR_LOOP_1050_5bcc = pvVar3;
    PTR_LOOP_1050_5bce = (pvVar3 >> 0x10);
    return;
}

pub unsafe fn destroy_win_1038_cc00(param_1: &mut u32, param_2: u16, param_3: u16, param_4: u32) {
    let mut i_var1: i32;
    let mut u_var2: u16;

    if (param_3._2_2_ == 0x1cd) {
        u_var2 = 1;
    } else {
        if (param_3._2_2_ == 0x1ce) {
            u_var2 = 2;
        } else {
            if (param_3._2_2_ == 0x1cf) {
                u_var2 = 3;
            } else {
                if (param_3._2_2_ == 0x1d0) {
                    u_var2 = 4;
                } else {
                    if (param_3._2_2_ != 0x1d1) {
                        post_win_msg_1040_7b3c(
                            param_1,
                            param_2,
                            param_3_00,
                            param_3,
                            param_3._2_2_,
                        );
                        return;
                    }
                    u_var2 = 5;
                }
            }
        }
    }
    i_var1 = pass1_1008_eb74((param_1 + 0x8e), u_var2);
    if (i_var1 != 0) {
        mci_send_command_1008_5c7c(ctx._g_struct_ptr_1050_02a0, CONCAT22(i_var1, 1));
        winapi_funcs::DestroyWindow16((param_1 + 6));
    }
    return;
}

pub unsafe fn destroy_win_1038_c836(param_1: &mut u32, param_2: &mut String, param_3: u16, param_4: u32) {
    let mut u_var1: u32;
    let mut local_6: [u8; 4];

    if (param_2._2_2_ == 0xfce) {
        pass1_1008_941a(CONCAT22(unaff_ss, local_6), 1, 0xac);
        mci_send_command_1008_5c9e(ctx._g_struct_ptr_1050_02a0, CONCAT22(unaff_ss, local_6));
        u_var1 = (param_1 + 0x8e);
        (u_var1 + 10) = 6;
        winapi_funcs::DestroyWindow16((param_1 + 6));
        PTR_LOOP_1050_5b80 = 0x0;
        return;
    }
    post_win_msg_1040_7b3c(param_1, CONCAT22(param_3, param_2_00), param_2);
    return;
}

pub unsafe fn win_fn_1038_c374(ctx: &mut AppContext, param_1: u32, param_2: &mut u32, param_3: HWND16) {
    let mut u_var1: u32;
    let ppc_var2: fn();
    let mut u_var3: u16;
    let mut u_var4: u32;
    let mut u_var5: u16;
    let mut u_var6: u16;
    let LVar7: LRESULT;
    let struct_var8: &mut Struct7;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    u_var6 = SUB42(offset, 0);
    LVar7 = winapi_funcs::SendMessage16(0, 0, 0x40b, param_3);
    u_var3 = LVar7;
    u_var5 = (param_2 >> 0x10);
    unsafe {
        ppc_var2 = (*param_2 + 0x10);
    }
    ppc_var2(offset, param_2, u_var5);
    _local_6 = CONCAT22(ctx.dx_reg, u_var3);
    local_a = 0;
    loop {
        if (_local_6 <= local_a) {
            return;
        }
        unsafe {
            ppc_var2 = (*param_2 + 4);
        }
        u_var4 = _local_6;
        ppc_var2(u_var6, param_2, u_var5, local_a, (local_a >> 0x10));
        u_var1 = (param_1 + 0x8e);
        struct_var8 = process_struct_1008_e586(
            u_var1,
            (u_var1 >> 0x10),
            CONCAT13((ctx.dx_reg >> 8), CONCAT12(ctx.dx_reg, u_var4)),
        );
        LVar7 = winapi_funcs::SendMessage16(struct_var8, 0, 0x403, param_3);
        u_var6 = 0x1000;
        error_check_1000_17ce(ctx, struct_var8);
        if (LVar7 == -1) {
            break;
        }
        if (LVar7 == -2) {
            return;
        }
        local_a = local_a + 1;
    }
    return;
}

pub fn send_win_msg_1038_c228(param_1: u32) -> LRESULT {
    let mut i_var1: i32;
    let mut u_var2: u16;
    let LVar3: LRESULT;
    let mut local_a: u16;
    let mut local_6: u16;

    u_var2 = (param_1 >> 0x10);
    i_var1 = param_1;
    LVar3 = winapi_funcs::SendMessage16(0, 0, 0x407, (i_var1 + 0x92));
    winapi_funcs::SendMessage16(0, 0, 0x407, (i_var1 + 0x94));
    winapi_funcs::SendMessage16(
        param_1 & 0xffff0000 | (i_var1 + 0x9e),
        LVar3,
        0x408,
        (i_var1 + 0x92),
    );
    LVar3 = winapi_funcs::SendMessage16(
        param_1 & 0xffff0000 | (i_var1 + 0x19e),
        0x408,
        0x408,
        (i_var1 + 0x94),
    );
    return LVar3;
}

pub unsafe fn check_gui_dlg_1038_b922(ctx: &mut AppContext, param_1: &mut u32, param_2: &mut String, uparam_3: i32) {
    let pi_var1: i32;
    let mut i_var2: i32;
    let in_struct_1: Struct300;
    let ppc_var3: fn();
    let h_var4: HANDLE16;
    let mut u_var5: u16;
    let h_var6: HWND16;
    let mut u_var7: i32;
    let puVar8: Vec<u8>;
    let mut u_var9: u32;
    let pu_var10: u16;
    let mut struct_a: Struct103;
    let mut u_var11: u16;
    let mut i_var12: i32;
    let mut u_var13: u16;
    let mut u_var14: u16;
    let mut unaff_ss: u16;
    let LVar15: LRESULT;
    let uVar16: u8;
    let mut local_470: u16;
    let mut local_46e: u16;
    let mut local_46c: u16;
    let mut local_468: u16;
    let mut local_466: u16;
    let mut local_464: [u8; 80];
    let mut local_414: [u8; 1024];
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;

    local_6 = 0;
    i_var12 = param_1;
    u_var13 = (param_1 >> 0x10);
    if (param_3 < 0x1b5) {
        if (0x19f < param_3) {
            local_8 = winapi_funcs::GetDlgItem16(param_3, (i_var12 + 6));
            LVar15 = winapi_funcs::SendMessage16(0, 0, 0x400, local_8);
            local_a = LVar15;
            if (local_a == 2) {
                return;
            }
            winapi_funcs::SendMessage16(0, (local_a == 0), 0x401, local_8);
            u_var5 = IsDlgButtonChecked16(param_3, (i_var12 + 6));
            if (u_var5 != 0) {
                pi_var1 = (i_var12 + 0x96);
                unsafe {
                    *pi_var1 = *pi_var1 + -1;
                }
                h_var6 = winapi_funcs::GetDlgItem16(0xfb1, (i_var12 + 6));
                IsWindowEnabled16(offset, h_var6);
                u_var7 = ctx.dx_reg;
                if (h_var6 == 0) {
                    h_var6 = winapi_funcs::GetDlgItem16(0xfb1, (i_var12 + 6));
                    winapi_funcs::EnableWindow16(1, h_var6);
                }
                if ((i_var12 + 0x96) < 0) {
                    CheckDlgButton16(0, (i_var12 + 0x98), (i_var12 + 6));
                    (i_var12 + 0x96) = 0;
                }
                (i_var12 + 0x98) = param_3;
                pass1_1018_1c9a(*(i_var12 + 0x92), param_3);
                u_var9 = pass1_1018_1e78((i_var12 + 0x92), 0xffff);
                local_e = (u_var9 & 0xffff | u_var7 << 0x10);
                if ((u_var7 | u_var9) == 0) {
                    local_10 = 0;
                } else {
                    local_10 = (u_var9 + 0x1c);
                }
                mci_send_command_1008_5c7c(&ctx._g_struct_ptr_1050_02a0, CONCAT22(local_10, 1));
                return;
            }
            pi_var1 = (i_var12 + 0x96);
            unsafe {
                *pi_var1 = *pi_var1 + 1;
            }
            if ((i_var12 + 0x96) != 1) {
                return;
            }
            h_var6 = winapi_funcs::GetDlgItem16(0xfb1, (i_var12 + 6));
            winapi_funcs::EnableWindow16(0, h_var6);
            return;
        }
        if (param_3 == 2) {
            return;
        }
        // LAB_1038_bc20:
        uVar16 = param_3;
    } else {
        if (param_3 == 0xfb1) {
            local_46c._0_1_ = 0xa0;
            local_46c._1_1_ = 1;
            while (CONCAT11(local_46c._1_1_, local_46c) < 0x1b5) {
                u_var5 = IsDlgButtonChecked16(CONCAT11(local_46c._1_1_, local_46c), (i_var12 + 6));
                if (u_var5 == 1) {
                    in_struct_1 = (i_var12 + 0x8e);
                    uVar16 = (in_struct_1 >> 0x10);
                    switch_stmt_1008_d818(in_struct_1, CONCAT11(local_46c._1_1_, local_46c));
                    // goto LAB_1038_bba2;
                }
                i_var2 = CONCAT11(local_46c._1_1_, local_46c) + 1;
                local_46c._0_1_ = i_var2;
                local_46c._1_1_ = (i_var2 >> 8);
            }
        } else {
            if (param_3 != 0xfbe) {}
            // goto LAB_1038_bc20;
            local_e = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(local_470, 2));
            u_var7 = (local_e >> 0x10);
            local_10 = u16_1050_13ae;
            if (u16_1050_13ae == 1) {
                local_10 = 2;
            }
            local_a = (local_10 * 0xc + 0x5b84) - 1;
            pass1_fn_1008_612e(0, local_a);
            u_var9 = pass1_1018_1e78((i_var12 + 0x92), ((local_10 * 6 + local_a) * 2 + 0x5b86));
            _local_14 = (u_var9 & 0xffff | u_var7 << 0x10);
            load_string_1010_84e0(
                ctx._g_struct_73_1050_14cc,
                (ctx._g_struct_73_1050_14cc >> 0x10),
                0x50,
                CONCAT13((unaff_ss >> 8), CONCAT12(unaff_ss, local_464)),
                0x5f1,
            );
            load_string_1010_847e(
                ctx._g_struct_73_1050_14cc,
                (ctx._g_struct_73_1050_14cc >> 0x10),
                *_local_14,
            );
            struct_a = ctx.dx_reg;
            u_var7 = wsprintf16(
                local_414,
                CONCAT13(0x5b, CONCAT12(0xc0, unaff_ss)),
                CONCAT13((local_464 >> 8), CONCAT12(local_464, 0x1050)),
            );
            u_var14 = 0x1000;
            process_struct_1000_179c(0xb4, &mut struct_a);
            h_var4 = ctx.g_h_window;
            if ((struct_a | u_var7) == 0) {
                local_8 = 0;
                u_var11 = 0;
            } else {
                local_470 = ctx._g_struct_73_1050_14cc;
                local_46e = (ctx._g_struct_73_1050_14cc >> 0x10);
                puVar8 = local_414;
                load_string_1010_847e(ctx, local_470, local_46e, 0x7b);
                local_46c._0_1_ = ctx.dx_reg;
                local_46c._1_1_ = (ctx.dx_reg >> 8);
                u_var14 = SUB42(&ctx.PTR_LOOP_1050_1040, 0);
                u_var11 = ctx.dx_reg;
                pu_var10 = process_struct_1040_8478(
                    CONCAT13((puVar8 >> 8), CONCAT12(puVar8, 0x41)),
                    0x41,
                    CONCAT13(local_46c._1_1_, CONCAT12(local_46c, puVar8)),
                    CONCAT22(unaff_ss, local_414),
                    h_var4,
                );
                local_8 = pu_var10;
            }
            ppc_var3 = (CONCAT22(u_var11, local_8) + 0x74);
            (**ppc_var3)(u_var14, local_8);
            if (local_8 != 1) {
                return;
            }
            switch_stmt_1008_d818((i_var12 + 0x8e), (_local_14 + 0x1a));
            uVar16 = *(_local_14 + 0x1a);
            // LAB_1038_bba2:
            set_cursor_1038_bc30(param_1, u_var13, uVar16);
        }
        winapi_funcs::PostMessage16(0, 0xce, 0x111, ctx.g_h_window);
        uVar16 = 1;
    }
    post_win_msg_1040_7b3c(
        param_1,
        (param_1 >> 0x10),
        param_2,
        (param_2 >> 0x10),
        uVar16,
    );
    return;
}

pub fn send_msg_1038_a9fa(param_1: u32, param_2: i32) {
    let h_wnd: u16;
    let mut i_var1: i32;
    let mut u_var2: u16;
    let ppVar3: Struct2111;
    let LVar4: LRESULT;
    let mut in_stack_0000fff0: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    if (param_2 != 0) {
        ppVar3 =
            process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(in_stack_0000fff0, 2));
        u_var2 = (param_1 >> 0x10);
        i_var1 = param_1;
        LVar4 = SendDlgItemMessage16(0, 0, 0x400, 0x116, (i_var1 + 6));
        if (LVar4 == 0) {
            LVar4 = SendDlgItemMessage16(0, 0, 0x400, 0x117, (i_var1 + 6));
            if (LVar4 == 0) {
                LVar4 = SendDlgItemMessage16(0, 0, 0x400, 0x118, (i_var1 + 6));
                if (LVar4 == 0) {
                    LVar4 = SendDlgItemMessage16(0, 0, 0x400, 0x119, (i_var1 + 6));
                    if (LVar4 != 0) {
                        u16_1050_13ae = 4;
                    }
                } else {
                    u16_1050_13ae = 3;
                }
            } else {
                u16_1050_13ae = 2;
            }
        } else {
            u16_1050_13ae = 1;
        }
        LVar4 = SendDlgItemMessage16(0, 0, 0x400, 0x11a, (i_var1 + 6));
        (ppVar3 + 0x82) = LVar4;
        h_wnd = winapi_funcs::GetWindowWord16(-8, (i_var1 + 6));
        winapi_funcs::PostMessage16(0, 0x105, 0x111, h_wnd);
        destroy_win_1040_7b98(i_var1, u_var2, 1);
    }
    return;
}

pub fn pass1_1038_8d7e(param_1: Vec<u8>) {
    ret_1040_78de();
    win_fn_1038_8f74(param_1);
    return;
}

pub unsafe fn get_dlg_item_int_1038_8afe(param_1: u32) {
    let mut u_var1: u32;
    let mut u_var2: u16;
    let mut i_var3: i32;
    let mut u_var4: u16;
    let mut local_a: u32;
    let mut local_6: u16;
    let local_4: bool;

    u_var4 = (param_1 >> 0x10);
    i_var3 = param_1;
    u_var2 = GetDlgItemInt16(0, &local_4, unaff_ss, (s_dibtext_bmp_1050_1844 + 9));
    pass1_1030_6c1a((i_var3 + 0x94), u_var2);
    u_var1 = (i_var3 + 0x94);
    pass1_1038_387e((u_var1 + 0x2e), u_var2, (i_var3 + 0x9c), (i_var3 + 0x94));
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

pub unsafe fn send_dlg_item_msg_1038_87b2(param_1: Vec<u8>) -> i32 {
    let mut u_var1: u32;
    let l_param: LPARAM;
    let mut i_var2: i32;
    let mut u_var3: u16;
    let mut unaff_ss: u16;
    let LVar4: LRESULT;
    let mut local_106: u16;
    let mut local_104: u16;
    let mut local_102: [u8; 256];

    i_var2 = send_dlg_item_msg_1038_8164(
        param_1,
        CONCAT22(unaff_ss, local_102),
        (s_logo_bmp_1050_1850 + 5),
    );
    if (i_var2 != 0) {
        u_var3 = (param_1 >> 0x10);
        i_var2 = param_1;
        pass1_1008_b61a((i_var2 + 0x94), CONCAT22(unaff_ss, local_102));
        u_var1 = (i_var2 + 0x94);
        l_param = load_string_1008_b1f0(u_var1, (u_var1 >> 0x10));
        LVar4 = SendDlgItemMessage16(
            l_param,
            0xffff,
            0x40d,
            (s_logo_bmp_1050_1850 + 6),
            (i_var2 + 6),
        );
        i_var2 = LVar4;
    }
    return i_var2;
}

pub unsafe fn send_dlg_item_msg_1038_8618(param_1: Vec<u8>) -> i32 {
    let mut u_var1: u32;
    let mut i_var2: i32;
    let mut i_var3: i32;
    let mut u_var4: u16;
    let mut unaff_ss: u16;
    let lVar5: u32;
    let mut u_var6: u32;
    let LVar7: LRESULT;
    let mut local_112: u16;
    let mut local_110: u16;
    let mut local_10e: u16;
    let mut local_10c: u16;
    let mut local_10a: u16;
    let mut local_108: u16;
    let mut local_106: [u8; 256];
    let mut local_6: u16;
    let mut local_4: u16;

    u_var4 = (param_1 >> 0x10);
    i_var3 = param_1;
    _local_6 = pass1_1008_b820((i_var3 + 0x94));
    i_var2 = _local_6;
    if (_local_6 != 0) {
        i_var2 = send_dlg_item_msg_1038_8164(
            param_1,
            CONCAT22(unaff_ss, local_106),
            (s_logo_bmp_1050_1850 + 4),
        );
        if (i_var2 != 0) {
            SendDlgItemMessage16(0, 0, 0xb, (s_logo_bmp_1050_1850 + 5), (i_var3 + 6));
            SendDlgItemMessage16(0, 0, 0xb, (s_logo_bmp_1050_1850 + 6), (i_var3 + 6));
            SendDlgItemMessage16(0, 0, 0x405, (s_logo_bmp_1050_1850 + 5), (i_var3 + 6));
            SendDlgItemMessage16(0, 0, 0x405, (s_logo_bmp_1050_1850 + 6), (i_var3 + 6));
            u_var1 = (i_var3 + 0x94);
            lVar5 = pass1_1008_b4a0(u_var1, (u_var1 >> 0x10), local_106, unaff_ss);
            pass1_1008_b200((i_var3 + 0x94));
            if (lVar5 != 0) {
                send_dialog_item_msg_1038_8400(i_var3, u_var4, lVar5, (s_logo_bmp_1050_1850 + 5));
                u_var6 = pass1_1008_b366((i_var3 + 0x94));
                if (u_var6 != 0) {
                    SendDlgItemMessage16(
                        u_var6,
                        0xffff,
                        0x40d,
                        (s_logo_bmp_1050_1850 + 5),
                        (i_var3 + 6),
                    );
                }
            }
            u_var6 = pass1_1008_b38c((i_var3 + 0x94));
            if (u_var6 != 0) {
                send_dialog_item_msg_1038_8400(i_var3, u_var4, u_var6, (s_logo_bmp_1050_1850 + 6));
                u_var6 = pass1_1008_b47a((i_var3 + 0x94));
                if (u_var6 != 0) {
                    SendDlgItemMessage16(
                        u_var6,
                        0xffff,
                        0x40d,
                        (s_logo_bmp_1050_1850 + 6),
                        (i_var3 + 6),
                    );
                }
            }
            SendDlgItemMessage16(0, 1, 0xb, (s_logo_bmp_1050_1850 + 5), (i_var3 + 6));
            LVar7 = SendDlgItemMessage16(0, 1, 0xb, (s_logo_bmp_1050_1850 + 6), (i_var3 + 6));
            i_var2 = LVar7;
        }
    }
    return i_var2;
}

pub unsafe fn send_dialog_item_msg_1038_8400(param_1: u32, param_2: u32, param_3: u16) {
    let mut unaff_ss: u16;
    let lVar1: u32;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: [u8; 8];

    pass1_1008_5784(CONCAT22(unaff_ss, local_a), param_2);
    while (true) {
        lVar1 = pass1_1008_5b12(CONCAT22(unaff_ss, local_a));
        if (lVar1 == 0) {
            break;
        }
        SendDlgItemMessage16((lVar1 + 4), 0, 0x401, param_3, (param_1 + 6));
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
    u_var8 = (u_var12 >> 0x10);
    u_var5 = u_var8;
    b_var4 = pass1_1008_c6ae(ctx._PTR_LOOP_1050_06e0, (u_var12 + 0xc), 4);
    local_bx_40 = param_1;
    u_var9 = (param_1 >> 0x10);
    if (b_var4 == 0) {
        b_var4 = pass1_1008_c6ae(ctx._PTR_LOOP_1050_06e0, (u_var12 + 0xc), 3);
        if (b_var4 == 0) {
            // code_r0x10387545:
            pass1_1038_6f5a(param_1, param_2);
            // goto LAB_1038_7549;
        }
        if ((local_bx_40.field_0xc != 0) || (&local_bx_40.field_0xe != 0)) {
            u_var13 = pass1_1028_45e2(u_var12);
            u_var5 = (u_var13 >> 0x10);
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
        u_var5 = (u_var13 >> 0x10);
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
                alloc_mem_1000_07fc(_PTR_LOOP_1050_68a2);
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
                u_var10 = (local_32 >> 0x10);
                local_bx_185 = local_32;
                local_bx_185.field_0x8 = local_bx_40.field_0x14;
                local_bx_185.field_0xa = local_bx_40.field_0x16;
                u_var5 = pass1_1020_c42e(local_bx_40.field_0x14);
            } else {
                pu_var7 = _PTR_LOOP_1050_68a2;
                alloc_mem_1000_07fc(_PTR_LOOP_1050_68a2);
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
                u_var10 = (local_1a >> 0x10);
                local_bx_185 = local_1a;
                local_bx_185.field_0x6 = local_bx_40.field_0x12;
                local_bx_185.field_0xa = local_bx_40.field_0x16;
                u_var5 = switch_statement_1020_c3b4(local_bx_40.field_0x12);
            }
            lVar3 = u_var5 * local_bx_185.field_0xa;
            u_var5 = (lVar3 >> 0x10);
            local_bx_185.field_0xc = lVar3;
            pass1_1028_6408(u_var12, CONCAT22(u_var10, local_bx_185));
            // goto LAB_1038_7549;
        }
    }
    // LAB_1038_74e0:
    pass1_1038_709c(param_1, param_2);
    // LAB_1038_7549:
    u_var2 = local_bx_40.field_0x8;
    paVar6 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, u_var2, (u_var2 >> 0x10));
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

    u_var6 = (param_1 >> 0x10);
    local_bx_4 = param_1;
    if ((local_bx_4.field_0x10 | &local_bx_4.field_0xe) == 0) {
        if (local_bx_4.field_0xc == 0) {
            if (local_bx_4.field_0x12 == 0) {
                if (local_bx_4.field_0x14 == 0) {
                    return;
                }
                pu_var5 = _PTR_LOOP_1050_68a2;
                alloc_mem_1000_07fc(_PTR_LOOP_1050_68a2);
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
                u_var7 = (local_28 >> 0x10);
                (local_28 + 8) = local_bx_4.field_0x14;
                (local_28 + 10) = local_bx_4.field_0x16;
                u_var2 = pass1_1020_c42e(local_bx_4.field_0x14);
            } else {
                pass1_1030_7c50(param_2, &local_bx_4.field_0x16, local_bx_4.field_0x12);
                pu_var5 = _PTR_LOOP_1050_68a2;
                alloc_mem_1000_07fc(_PTR_LOOP_1050_68a2);
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
                u_var7 = (local_28 >> 0x10);
                (local_28 + 6) = local_bx_4.field_0x12;
                (local_28 + 10) = local_bx_4.field_0x16;
                u_var2 = switch_statement_1020_c3b4(local_bx_4.field_0x12);
            }
            u_var7 = (local_28 >> 0x10);
            local_bx_337 = local_28;
            i_var4 = u_var2 * local_bx_337.field_0xa;
        } else {
            pu_var5 = _PTR_LOOP_1050_68a2;
            alloc_mem_1000_07fc(_PTR_LOOP_1050_68a2);
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
            u_var7 = (local_28 >> 0x10);
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
                alloc_mem_1000_07fc(_PTR_LOOP_1050_68a2);
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
                u_var7 = (local_10 >> 0x10);
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

    u_var6 = (param_1 >> 0x10);
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
        alloc_mem_1000_07fc(_PTR_LOOP_1050_68a2);
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
        u_var7 = (local_10 >> 0x10);
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
            if (CONCAT22(local_a._2_2_, local_a) != 0) {
                pass1_1030_7ddc(param_2, CONCAT22(local_a._2_2_, local_a), local_6);
            }
            local_c = local_c + 1;
        }
    }
    return;
}

pub unsafe fn win_fn_1038_362e(param_1: &mut Struct1090) {
    let mut i_var1: i32;
    let mut u_var2: i32;
    let mut ppVar3: Struct2111;
    let mut in_stack_0000fff8: u32;
    let mut u_var4: u16;

    u_var2 = (param_1 >> 0x10);
    local_bx_4 = param_1;
    if local_bx_4.field_0x214 == 0 {
        u_var4 = (in_stack_0000fff8 >> 0x10);
        i_var1 = pass1_1038_4f54((param_1 & 0xffff | u_var2 << 0x10), 0x1f);
        if i_var1 == 0 {
            local_bx_4.field_0x214 = 0x14;
        } else {
            local_bx_4.field_0x214 = 0x28;
        }
       let mut struct_var3 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(u_var4, 0x37));
        sys1::post_win_msg_1008_a0e4(&mut struct_var3, 0, 0, 1, local_bx_4.field_0x4, 0x38);
        local_bx_4.field_0x216 = 0;
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

    _local_6 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(local_48, 0x37));
    local_a = *ctx._PTR_LOOP_1050_65e2;
    u_var9 = (param_2_00 >> 0x10);
    if (local_a % 10 == 0) {
        if (param_1_00 < 0xc9) {
            u_var11 = 0x3f;
        } else {
            if (param_1_00 < 800) {}
            // goto LAB_1038_09c3;
            u_var11 = 0x3e;
        }
        sys1::post_win_msg_1008_a0e4(_local_6, 0, 0, 1, (param_2_00 + 4), u_var11);
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
            u_var8 = u_var8 >> 0x10;
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
                u_var9 = SUB42(&PTR_LOOP_1050_1028, 0);
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

pub unsafe fn win_fn_1030_e67c(ctx: &mut AppContext, param_1: u32) -> u16 {
    let mut u_var1: u16;
    let ppVar2: Struct2111;
    let mut in_stack_0000fff6: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    ppVar2 = process_struct_1010_20ba(
        ctx._g_Struct372_1050_0ed0,
        CONCAT22((in_stack_0000fff6 >> 0x10), 0x37),
    );
    u_var1 = switch_stmt_1008_aaa8(ppVar2, (ppVar2 >> 0x10), (param_1 + 0x108));
    if (u_var1 != 0) {
        sys1::post_win_msg_1008_a0e4(&mut ppVar2, 0, 0, 1, 0, u_var1);
    }
    return 1;
}

pub unsafe fn pass1_1030_838e(param_1: &mut u32) {
    unsafe {
        pass1_1028_d2b0(param_1);
    }
    pass1_1028_d01a((param_1 + 4));
    send_win_msg_1028_e242(ctx._PTR_LOOP_1050_65e2, (ctx._PTR_LOOP_1050_65e2 >> 0x10));
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
            send_win_msg_1028_e242(ctx._PTR_LOOP_1050_65e2, 0);
        }
    }
    send_win_msg_1028_e242(
        ctx._PTR_LOOP_1050_65e2,
        (ctx._PTR_LOOP_1050_65e2 >> 0x10),
        1,
    );
    return;
}

pub unsafe fn send_win_msg_1028_e242(ctx: &mut AppContext, param_1: u32, param_2: u32) {
    if *param_1 % 100 == 0 {
        // SendMessage16(0, 0, 0x41, ctx.g_h_window);
        let result = winapi_funcs::SendMessage16(ctx.g_h_window, 0x41, 0, 0);
    }
    *param_1 = *param_1 + 1;
    if param_2 != 0 {
        bad_1028_e28a();
    }
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

    process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(in_stack_0000ffde, 2));
    if ((u16_1050_13ae < 1) || (SBORROW2(u16_1050_13ae, 1))) {
        // LAB_1028_af27:
        local_c._2_2_ = 1;
    } else {
        if (u16_1050_13ae == 2 || (u16_1050_13ae - 1) < 1) {
            local_c = 0x10001;
            // goto LAB_1028_af42;
        }
        if (u16_1050_13ae != 4) {}
        // goto LAB_1028_af27;
        local_c._2_2_ = 2;
    }
    local_c = CONCAT22(local_c._2_2_, 3);
    // LAB_1028_af42:
    u_var2 = pass1_fn_1008_612e(local_c._2_2_, local_c);
    u_var4 = (param_1 >> 0x10);
    local_bx_72 = param_1;
    local_bx_72.field_0x114 = u_var2;
    ppVar5 = process_struct_1010_20ba(
        ctx._g_Struct372_1050_0ed0,
        CONCAT22(in_stack_0000ffde, 0x37),
    );
    u_var2 = (ppVar5 >> 0x10);
    u_var1 = local_bx_72.field_0x108;
    paVar3 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, u_var1, (u_var1 >> 0x10));
    sys1::post_win_msg_1008_a0e4(
        &mut ppVar5,
        0,
        local_bx_72.field_0x114,
        &paVar3[0x11].field_0xa,
        local_bx_72.field_0x108,
        2,
    );
    ppVar5 = process_struct_1010_20ba(
        ctx._g_Struct372_1050_0ed0,
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

    u_var12 = (param_5 >> 0x10);
    local_bx_6 = param_5;
    u_var1 = &local_bx_6.field_0x1f6;
    u_var6 = &local_bx_6.field_0x1f8;
    u_var5 = u_var1 + 0x18c;
    u_var4 = (u_var1 >> 0x10);
    u_var7 = u_var5;
    pass1_1030_38f2(u_var1, u_var6, param_2_00);
    u_var3 = 100 / param_1_00;
    u_var10 = u_var3 >> 0xf;
    i_var11 = param_2_00 * 4;
    lVar2 = (u_var7 & 0xffff | u_var6 << 0x10) + (i_var11 + u_var5);
    lVar8 = lVar2 / (u_var3 & 0xffff | u_var10 << 0x10);
    lVar9 = lVar8 * (u_var3 & 0xffff | u_var10 << 0x10);
    local_e = lVar2;
    local_c = (lVar2 >> 0x10);
    u_var6 = lVar9;
    (u_var5 + i_var11) = local_e - u_var6;
    (u_var5 + i_var11 + 2) = (local_c - (lVar9 >> 0x10)) - (local_e < u_var6);
    local_12._2_2_ = (lVar8 >> 0x10);
    local_12._0_2_ = lVar8;
    if ((local_12._2_2_ | local_12) != 0) {
        pass1_1030_375a(u_var1, u_var4, param_2_00, local_12, local_12._2_2_);
        if (local_bx_6.field_0x200 != 0x8000002) {
            ppVar13 =
                process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(unaff_si, 0x37));
            sys1::post_win_msg_1008_a0e4(
                &mut ppVar13,
                0,
                local_12,
                local_bx_6.field_0x208,
                local_bx_6.field_0x4,
                2,
            );
            ppVar13 =
                process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(unaff_si, 0x2b));
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

    u_var6 = (param_1 >> 0x10);
    local_bx_4 = param_1;
    u_var1 = local_bx_4.field_0x108;
    paVar2 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, u_var1, (u_var1 >> 0x10));
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
        pp_var7 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(local_38, 0x2b));
        local_14 = (pp_var7 >> 0x10);
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
            pp_var7 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, 0x37);
            sys1::post_win_msg_1008_a0e4(
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
            PTR_LOOP_1050_4fe8 = (&ctx.PTR_LOOP_1050_0000 + 1);
            u_var3 = 0;
            u_var14 = 1;
            _local_34 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, 0x2b);
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
        _local_34 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, 0x37);
        local_20 = (_local_34 >> 0x10);
        sys1::post_win_msg_1008_a0e4(
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
            u_var3 = (param_1 >> 0x10);
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
        local_26 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, 0x37);
        u_var5 = (local_26 >> 0x10);
        sys1::post_win_msg_1008_a0e4(
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
        ppVar6 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(local_48, 0x3b));
        local_2e = (ppVar6 >> 0x10);
        local_30 = ppVar6;
        pass1_1008_df4a(ppVar6);
        ppVar6 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(local_48, 0x3c));
        local_2e = (ppVar6 >> 0x10);
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

pub fn win_msg_fn_1020_d460(ctx: &mut AppContext, param_1: &mut Vec<u8>, param_2: u32, param_3: u32, param_4: u32) -> u16 {
    let mut u_var1: u16;
    let ppVar2: Struct2111;
    let mut in_stack_0000ffe8: u16;
    let mut local_a: u16;
    let mut local_8: u16;

    u_var1 = pass1_1028_bc90(param_1, param_2, param_3, param_4);
    if (u_var1 != 0) {
        pass1_1038_af40(ctx._g_Struct112_a, *(_PTR_LOOP_1050_4230 + 0x16), 0x11);
        ctx.PTR_LOOP_1050_5b80 = (&ctx.PTR_LOOP_1050_0000 + 1);
        process_win_msg_1008_9510(&mut ctx.PTR_LOOP_1050_5b80, &mut ctx.g_alloc_addr_1050_1050);
        let struct_var2 = process_struct_1010_20ba(
            ctx._g_Struct372_1050_0ed0,
            CONCAT22(in_stack_0000ffe8, 0x3a),
        );
        (param_1 + 0x20) = (struct_var2 + 10);
        u_var1 = 1;
    }
    return u_var1;
}

pub fn get_sys_metrics_1020_7c1a(param_1: &mut u16, param_2: u32) {
    let mut u_var1: u16;
    let i_var2: u16;
    let mut i_var3: i32;
    let mut u_var4: u16;
    let mut u_var5: u16;
    let mut local_10: u16;

    u_var4 = (param_2 >> 0x10);
    u_var1 = (param_2 + 8);
    u_var5 = (param_1 >> 0x10);
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
    i_var2 = winapi_funcs::GetSystemMetrics16(4);
    (i_var3 + 0xe) = i_var2;
    i_var2 = winapi_funcs::GetSystemMetrics16(5);
    (i_var3 + 0x10) = i_var2;
    i_var2 = winapi_funcs::GetSystemMetrics16(6);
    (i_var3 + 0x12) = i_var2;
    return;
}

pub fn post_win_msg_1020_79fc(param_1: u32, param_2: u16, param_3: u16, param_4: i32) {
    let mut u_var1: u32;
    let ppc_var2: fn();
    let mut i_var3: i32;
    let mut i_var4: i32;
    let mut u_var5: u16;
    let mut u_var6: u16;

    u_var5 = (param_1 >> 0x10);
    i_var4 = param_1;
    u_var6 = (*(i_var4 + 0xe0) >> 0x10);
    ppc_var2 = ((i_var4 + 0xe0) + 0x24);
    i_var3 = ppc_var2();
    if (i_var3 != param_2) {
        winapi_funcs::PostMessage16(0, 0, 0x85, (i_var4 + 8));
        u_var1 = (i_var4 + 0xe0);
        ppc_var2 = ((i_var4 + 0xe0) + 0x28);
        ppc_var2(offset, u_var1, (u_var1 >> 0x10), param_2, u_var6);
    }
    return;
}

pub unsafe fn call_win_fn_1020_7526(param_1: &mut Struct7, param_2: u8) -> &mut Struct7 {
    win_fn_1020_7270(ctx, param_1);
    if (param_2 & 1) != 0 {
        error_check_1000_17ce(ctx, param_1);
    }
    return param_1;
}

pub fn post_msg_1020_4394(ctx: &mut AppContext, param_1: u32, param_2: i32) {
    let mut u_var1: u32;
    let mut i_var2: i32;
    let mut u_var3: u16;

    i_var2 = param_1;
    u_var3 = (param_1 >> 0x10);
    if (param_2 == 0x10) {
        if ((i_var2 + 0x34) != 0) {
            winapi_funcs::PostMessage16(0, 0xf6, 0x111, ctx.g_h_window);
            return;
        }
    } else {
        if (param_2 < 0x11) {
            if (param_2 == 0x1) {
                (i_var2 + 0x18) = 0;
                return;
            }
            if (param_2 == 0xb) {
                u_var1 = (i_var2 + 0x2c);
                (u_var1 + 0xe) = (i_var2 + -0xda);
                return;
            }
        }
    }
    return;
}

pub fn cleanup_fn_1020_28fc(ctx: &mut AppContext, param_1: &mut Struct376) {
    param_1.ptr_a_lo = (ctx.s__P__P__P__P__P__P__P__P__P__P__P_1050_2e35 + 0x15);
    (param_1 + 2) = 0x1020;
    destroy_menu_func_1020_795c(param_1);
    return;
}

pub fn post_win_msg_1020_291a(param_1: &mut StructuredData) {
    let hwnd = param_1.get_u16(8) as HWND16;
    // winapi_funcs::PostMessage16(0, 0, 0x10, (param_1 + 8));
    winapi_funcs::PostMessage16(hwnd, 0x10, 0, 0);
    return;
}

pub unsafe fn post_win_msg_1020_1ca4(param_1: u32) -> u16 {
    let mut pp_var1: fn();
    let mut u_var2: u16;
    let mut u_var3: i32;
    let mut u_var4: u16;
    let mut pu_var5: Struct103;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;

    // u_var4 = (param_1 >> 0x10);
    if (param_1 + 0x96) == 0 {
        pu_var5 = pass1_1010_4df0((param_1 + 0x8e));
        if pu_var5 == 0 {
            u_var4 = 0x1000;
            process_struct_1000_179c(0xb4, &mut pu_var5);
            u_var3 = pu_var5;
            if pu_var5 == 0x0 {
                u_var2 = 0;
                u_var3 = 0;
            } else {
                u_var4 = &ctx.PTR_LOOP_1050_1040;
                u_var2 = mixed_1040_8520(&mut pu_var5, ctx.g_h_window, 0x30, 2, 0x57b, 0x62a);
            }
            _local_a = CONCAT22(u_var3, u_var2);
            pp_var1 = (*_local_a + 0x74);
            (**pp_var1)(u_var4, u_var2, u_var3);
            return 0;
        }
        winapi_funcs::PostMessage16(0, 0xde, 0x111, ctx.g_h_window);
    }
    return 1;
}
