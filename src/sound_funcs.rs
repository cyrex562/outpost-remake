use std::intrinsics::offset;

use crate::{defines::{Struct199, Struct620}, util::CONCAT22};
use crate::app_context::AppContext;
use crate::cleanup::window_msg_func_1010_7300;
use crate::draw::rect::call_fill_rect_1020_041e;
use crate::err_ops::set_error_mode_1010_85be;
use crate::list_funcs::{modify_u16_list_1008_5bdc, modify_u16_list_1008_5c34};
use crate::other_funcs::mixed_fn_1010_830a;
use crate::pass::pass13_funcs::pass1_1008_941a;
use crate::pass::pass14_funcs::pass1_1008_6978;
use crate::pass::pass8_funcs::{pass1_1010_1f62, pass1_1010_84f8, process_struct_1010_20ba};
use crate::pass::pass_funcs::pass1_1000_4906;
use crate::string_ops::misc::{process_string_1000_475e, process_string_1000_4d58};
use crate::struct_ops::struct_ops_2::process_struct_1000_179c;
use crate::structs::prog_structs_16::Struct13;
use crate::structs::prog_structs_2::{Struct199, Struct318};
use crate::structs::prog_structs_25::Struct65;
use crate::structs::prog_structs_4::Struct626;
use crate::structs::prog_structs_6::Struct620;
use crate::structs::prog_structs_9::Struct75;
use crate::sys_ops::win::win_func_1018_6bb6;
use crate::sys_ops::win_msg::process_win_msg_1008_9510;
use crate::ui_ops::cursor::load_cursor_1008_61b2;
use crate::ui_ops::misc::win_gui_fn_1020_3644;
use crate::ui_ops::window::create_win_1008_5e7e;
use crate::util::ZEXT24;
use crate::winapi::{mciGetErrorString16, mciSendCommand16};

pub fn sound_fn_1008_53ae(ctx: &mut AppContext, param_1: u32, param_2: u16) {
    let mut unaff_ss: u16;
    let dVar1: u32;
    let mut local_432: [u8; 1024];
    let mut local_32: u16;
    let mut local_30: u16;
    let mut local_2e: u16;
    let mut local_2c: u16;
    let mut local_22: u32;
    let mut local_20: u16;
    let mut local_1e: u32;
    let mut local_1a: u16;
    let mut local_16: u16;
    let mut local_14: String;
    let mut local_12: u32;
    let mut local_e: u32;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;

    local_1e = 0;
    local_16 = 0x28c;
    local_14 = ctx.g_alloc_addr_1050_1050.clone();
    local_12 = param_1;
    local_e = 0;
    local_a = 0;
    local_8 = 0x4000;
    local_6 = param_2;
    // dVar1 = mciSendCommand16(CONCAT22(unaff_ss, &local_1e), 0x30200, 0x803, 0);
    dVar1 = mciSendCommand16(0, 0x803, 0x30200, &local_1e);
    // local_20 = (dVar1  >> 0x10);
    local_22 = dVar1;
    if local_1a != 0 {
        if (local_20 | local_22) != 0 {
            mciGetErrorString16(offset, 0x400, local_432);
        }
        pass1_1000_4906(CONCAT22(unaff_ss, &local_2e), 0, 0xc);
        local_2e = param_2;
        local_2c = 0;
        // dVar1 = mciSendCommand16(CONCAT22(unaff_ss, &local_2e), 2, 0x806, local_1a);
      // local_20 = (dVar1  >> 0x10);
        dVar1 = mciSendCommand16(local_1a, 0x806, 2, &local_2e)
        local_22 = dVar1;
        if (local_20 | local_22) != 0 {
            // mciGetErrorString16(offset, 0x400, local_432);
            mciGetErrorString16(local_432, 0x400, 0);
        }
        local_32 = param_2;
        local_30 = 0;
        // dVar1 = mciSendCommand16(CONCAT22(unaff_ss, &local_32), 1, 0x804, local_1a);
        dVar1 = mciSendCommand16(local_1a, 0x804, 1, &local_32);
        // local_20 = (dVar1  >> 0x10);
        local_22 = dVar1;
        if (local_20 | local_22) != 0 {
            mciGetErrorString16(offset, 0x400, local_432);
        }
    }
    return;
}

pub unsafe fn mci_send_cmd_1008_5c5c(param_1: u32, param_2: u16) {
    let mut u_var1: u32;
    let mut local_DXAX_12: u32;

    u_var1 = pass1_1010_84f8(ctx.g_struct_73_1050_14cc, param_2);
    mci_send_command_1008_5cfe(param_1, u_var1);
    return;
}

pub fn mci_send_command_1008_5c7c(ctx:&mut AppContext, in_char_buff: Vec<u8>, struct_param_2: &mut  Struct13) {
    let mut u_var1 = set_error_mode_1010_85be(ctx, ctx.g_struct_73_1050_14cc, struct_param_2);
    mci_send_command_1008_5cfe(in_char_buff, u_var1);
    return;
}

pub fn mci_send_command_1008_5c9e(in_char_buff: Vec<u8>, in_struct_1: &mut  Struct13) {
    mci_send_command_1008_5c7c(in_char_buff, in_struct_1);
    return;
}

pub fn mci_send_command_1008_5cb6(param_1: &mut  Struct318, dev_id: u16) {
    let local_bx_23: &mut  Struct75;
    let mut u_var1: u16;
    let mut u_var2: u16;

    mciSendCommand16(0, 0, 0x804, dev_id);
  // u_var1 = (param_1  >> 0x10);
    local_bx_23 = param_1;
    if ((local_bx_23.field_0xa == 0) || (local_bx_23.field_0xa != dev_id)) {
        local_bx_23.field_0x12 = 0;
        u_var2 = 0x11;
    } else {
        local_bx_23.field_0x10 = 0;
        u_var2 = 0x10;
    }
    pass1_1010_1f62(param_1, u_var2);
    return;
}

// WARNING: Variable defined which should be unmapped: local_132

// WARNING: Could not reconcile some variable overlaps

pub fn mci_send_command_1008_5cfe(param_1: u32, param_2: u32) {
    let mut u_var1: u32;
    let mut i_var2: i32;
    let mut i_var3: i32;
    let mut local_es_31: u16;

    let dVar4: u32;
    let mut local_132: u16;
    let mut local_12c: u16;
    let mut local_12a: u16;
    let mut local_120: u16;
    let local_11e: u8;
    let mut local_1e: u16;
    let mut local_1c: u16;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_16: [u8; 4];
    let mut local_12: u16;
    let mut local_e: u32;
    let mut local_a: u32;
    let mut temp_5f24728895: u32;

    pass1_1000_4906(CONCAT22(ctx.stack_seg_reg, local_16), 0, 0x14);
    local_a = param_2;
  // local_es_31 = (param_1  >> 0x10);
    i_var3 = param_1;
    temp_5f24728895 = (i_var3 + 0xc);
    local_18 = (temp_5f24728895 + 0x72);
    local_1a = 1;
    _local_1e = s_waveaudio_1050_02a4;
    process_string_1000_4d58(param_2, 0, 0, 0, 0, &local_11e);
    i_var2 = process_string_1000_475e(CONCAT22(ctx.stack_seg_reg, &local_11e), s__mid_1050_02ae);
    if (i_var2 == 0) {
        u_var1 = (i_var3 + 0xc);
        local_18 = (u_var1 + 0x74);
        _local_1e = s_sequencer_1050_02b3;
        local_1a = 0;
    }
    if (local_18 != 0) {
        if ((local_1a != 0) && ((i_var3 + 0x10) != 0)) {
            return;
        }
        if ((local_1a == 0) && ((i_var3 + 0x12) != 0)) {
            return;
        }
        local_e = _local_1e;
        dVar4 = mciSendCommand16(CONCAT22(ctx.stack_seg_reg, local_16), 0x2200, 0x803, 0);
        local_120 = dVar4;
        if (((dVar4 >> 0x10) | local_120) == 0) {
            if (local_1a == 0) {
                (i_var3 + 0x12) = 1;
            } else {
                (i_var3 + 10) = local_12;
                (i_var3 + 0x10) = 1;
                local_120 = local_12;
            }
            create_win_1008_5e7e(param_1);
            if (local_120 == 0) {
                mci_send_command_1008_5cb6(param_1, local_12);
                return;
            }
            pass1_1000_4906(CONCAT22(ctx.stack_seg_reg, &local_12c), 0, 0xc);
            local_12c = local_120;
            local_12a = 0;
            mciSendCommand16(CONCAT22(ctx.stack_seg_reg, &local_12c), 1, 0x806, local_12);
            SetWindowWord16(0x38, local_12, 0, local_120);
            return;
        }
    }
    if (local_1a == 0) {
        local_132 = 0x11;
    } else {
        local_132 = 0x10;
    }
    pass1_1010_1f62(param_1, local_132);
    return;
}

pub fn mci_fn_1018_e2cc(in_Struct620_ptr_1: &mut  Struct620) {
    let piVar1: &mut  i32;
    let mut u_var2: i32;
    let mut u_var3: u32;
    let struct_a: &mut  Struct199;
    let paVar4: &mut  Struct199;

    let local_Struct620_ptr_1: &mut  Struct620;
    let mut u_var5: u16;
    let mut unaff_ss: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: [u8; 4];
    let fn_ptr_1: fn();

  // u_var5 = (in_Struct620_ptr_1  >> 0x10);
    local_Struct620_ptr_1 = in_Struct620_ptr_1;
    if (local_Struct620_ptr_1.field_0xee != 0) {
        fn_ptr_1 = (local_Struct620_ptr_1.field_0xee + 8);
        fn_ptr_1();
    }
    if (local_Struct620_ptr_1.field_0xea == 0) {
        local_Struct620_ptr_1.field_0xea = 1;
        pass1_1008_941a(CONCAT22(unaff_ss, local_6), 1, 0x7a);
        u_var3 = ZEXT24(local_6);
        mci_send_command_1008_5c9e(ctx.g_struct_1050_02a0, CONCAT22(unaff_ss, local_6));
        local_Struct620_ptr_1.field_0xec = u_var3;
        paVar4 = struct_a;
        process_struct_1000_179c(0x112, struct_a);
        if ((paVar4 | u_var3) == 0) {
            u_var2 = 0;
            _local_a = 0x0;
        } else {
            piVar1 = &local_Struct620_ptr_1.field_0xcc;
            unsafe {
                *piVar1 = *piVar1 + 1;
            }
            win_gui_fn_1020_3644(
                u_var3,
                paVar4,
                local_Struct620_ptr_1.field_0xcc,
                in_Struct620_ptr_1,
            );
            u_var2 = u_var3;
            _local_a = (u_var3 & 0xffff | ctx.dx_reg << 0x10);
        }
        pass1_1008_6978(in_Struct620_ptr_1, 0, _local_a & 0xffff0000 | u_var2);
        fn_ptr_1 = (*_local_a + 0xc);
        (**fn_ptr_1)(8, _local_a, local_8, 5);
    }
    return;
}

pub fn win_and_mci_fn_1018_ea66(param_1: &mut  Struct626) {
    let mut local_AX_77: u16;
    let local_struct_1: &mut  Struct626;
    let mut local_struct_1_hi: u16;

    let mut char_buf_1: [u8; 4];
    // fn_ptr_1: &mut  Vec<u8>;

  // local_struct_1_hi = (param_1  >> 0x10);
    local_struct_1 = param_1;
    if (local_struct_1.field_0xee != 0) {
        fn_ptr_1 = (local_struct_1.field_0xee + 8);
        (**fn_ptr_1)();
    }
    if (local_struct_1.bool_xEA == 0) {
        local_struct_1.bool_xEA = 1;
        pass1_1008_941a(CONCAT22(ctx.stack_seg_reg, char_buf_1), 1, 0x95);
        local_AX_77 = mci_send_command_1008_5c9e(
            ctx.g_struct_1050_02a0,
            CONCAT22(ctx.stack_seg_reg, char_buf_1),
        );
        local_struct_1.field_0xec = local_AX_77;
        window_msg_func_1010_7300(local_struct_1.field_0xf6, 0, 0x80000, 0);
    }
    return;
}

pub fn mci_cmd_fn_1020_07aa(in_struct_1: &mut  Struct13) {
    let local_struct_1: &mut  Struct13;
    let local_struct_1_hi: &mut  Struct13;
    let mut unaff_ss: u16;
    let mut char_buff_1: [u8; 20];

    call_fill_rect_1020_041e(in_struct_1);
  // local_struct_1_hi = (in_struct_1  >> 0x10);
    local_struct_1 = in_struct_1;
    if (local_struct_1.field_0xf0 == 0) {
        local_struct_1.field_0xf0 = 1;
        modify_u16_list_1008_5bdc(CONCAT22(unaff_ss, char_buff_1));
        mci_send_command_1008_5c9e(
            CONCAT22(unaff_ss, char_buff_1),
            (in_struct_1 & 0xffff0000 | ZEXT24(local_struct_1 + 1)),
        );
        modify_u16_list_1008_5c34(CONCAT22(unaff_ss, char_buff_1));
    }
    return;
}

pub fn mci_fn_1020_08b6(param_1: &mut  Struct65, param_2: u16, param_3: &mut Vec<u8>) {
    let mut iVar1: i32;
    let mut local_es_21: u16;

    load_cursor_1008_61b2(param_1, 1, param_2, param_3);
  // local_es_21 = (param_1  >> 0x10);
    iVar1 = param_1;
    (iVar1 + 0xe6) = 0;
    (iVar1 + 0xe8) = 0;
    param_1.ptr_a_lo = 0xb0e;
    (iVar1 + 2) = 0x1020;
    mci_send_cmd_1008_5c5c(ctx.g_struct_1050_02a0, 0x1d4);
    return;
}

pub fn sound_fn_1040_8978(param_1: &mut  u32, param_2: u16) {
    let fn_ptr_1: fn();

    process_win_msg_1008_9510();
    mci_send_cmd_1008_5c5c(ctx.g_struct_1050_02a0, param_2);
    unsafe {
        fn_ptr_1 = (*param_1 + 0x74);
        (**fn_ptr_1)(&ctx.PTR_LOOP_1050_1008, param_1);
    }
    return;
}

pub unsafe fn win_fn_1018_6adc(ctx: &mut AppContext, param_1: u32, param_2: &mut String) {
    let mut i_var1: i32;
    let mut i_var2: i32;
    let u_var3: u8;
    let mut u_var4: u16;
    let extraout_DL: u8;
    let mut i_var5: i32;
    let mut u_var6: u16;
    let pp_var7: &mut  Struct2551;
    let mut in_stack_0000ffdc: u32;
    // let in_string_1: String;
    let mut local_6: u16;
    let mut local_4: u16;

  // in_string_1 = CONCAT22((in_stack_0000ffdc  >> 0x10), 0x48);
    pp_var7 = process_struct_1010_20ba(ctx.g_struct_var_1050_0ed0, in_string_1);
  // u_var6 = (pp_var7  >> 0x10);
    i_var1 = (pp_var7 + 10);
    i_var2 = (pp_var7 + 0xc);
  // u_var6 = (param_1  >> 0x10);
    i_var5 = param_1;
    if (1 < i_var1 - (i_var5 + 0xe6)) {
        (i_var5 + 0xe2) = i_var1 / 2 - ((i_var5 + 0xe6) + 1) / 2;
    }
    if (1 < i_var2 - (i_var5 + 0xe8)) {
        (i_var5 + 0xe4) = i_var2 / 2 - ((i_var5 + 0xe8) + 1) / 2;
    }
    ShowCursor16(0x1010, 0, (in_string_1 >> 0x10));
    if ((i_var5 + 0xee) != 0) {
        u_var4 = mci_send_cmd_1008_5c5c(ctx.g_struct_1050_02a0, (i_var5 + 0xee));
        (i_var5 + 0xf0) = u_var4;
    }
    u_var3 = mixed_fn_1010_830a(ctx.g_struct_73_1050_14cc, (i_var5 + 0xec));
    sound_fn_1008_53ae(ctx, u_var3, extraout_DL, (i_var5 + 8));
    ShowCursor16(8, 1);
    win_func_1018_6bb6(param_1, (param_1 >> 0x10));
    return;
}
