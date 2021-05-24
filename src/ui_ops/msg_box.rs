use crate::app_context::AppContext;
use crate::err_ops::error_check_1000_17ce;
use crate::exit::fatal_app_exit_1000_3e9e;
use crate::string_ops::misc::{copy_string_1000_3d3e, process_string_1000_3cea};
use crate::struct_ops::struct_ops_2::process_struct_1000_179c;
use crate::structs::prog_structs_2::Struct199;
use crate::util::{CONCAT22, SBORROW2};
use crate::winapi::{MessageBeep16, MessageBox16, PostMessage16};

pub unsafe fn msg_box_1000_1f24(ctx: &mut AppContext,
                                param_1: &String,
                                param_2: &String) -> bool {
    let pi_var1: &mut  i32;

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

pub fn msg_box_1040_d3d0(param_1: u32) {

    let in_dx: &mut  Struct199;
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
  // u_var1 = (param_1  >> 0x10);
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

pub fn display_msg_box_1040_cce4(param_1: &mut Vec<u8>) {
    let msg_box_text: String;
    let in_dx: &mut  Struct199;
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

pub fn msg_box_1040_ad66(param_1: u32) {

    let in_dx: &mut  Struct199;
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

pub fn msg_box_1040_a85a(param_1: u32) {

    let in_dx: &mut  Struct199;
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

pub fn msg_box_1040_64ca(param_1: u32) {

    let in_dx: &mut Struct199;
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
        0x7ff,
    );
    load_string_1010_84e0(
        ctx._g_struct_73_1050_14cc,
        (ctx._g_struct_73_1050_14cc >> 0x10),
        0x3ff,
        CONCAT22(unaff_ss, local_104),
        0x800,
    );
    process_string_1000_3cea(CONCAT22(in_dx, in_ax), CONCAT22(unaff_ss, local_104));
    load_string_1010_84e0(
        ctx._g_struct_73_1050_14cc,
        (ctx._g_struct_73_1050_14cc >> 0x10),
        0x3ff,
        CONCAT22(unaff_ss, local_104),
        0x801,
    );
    process_string_1000_3cea(CONCAT22(in_dx, in_ax), CONCAT22(unaff_ss, local_104));
    load_string_1010_84e0(
        ctx._g_struct_73_1050_14cc,
        (ctx._g_struct_73_1050_14cc >> 0x10),
        0x3ff,
        CONCAT22(unaff_ss, local_104),
        0x802,
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

pub fn msg_box_1038_8a3a(param_1: u32) {

    let in_dx: &mut Struct199;
    let mut unaff_ss: u16;
    let mut local_212: u32;
    let mut local_20e: u32;
    let mut local_20a: [u8; 258];
    let mut local_108: u16;
    let mut local_106: u16;
    let mut local_104: [u8; 258];

    process_struct_1000_179c(0x1000, in_dx);
    _local_108 = CONCAT22(in_dx, in_ax);
    load_string_1010_84e0(
        ctx._g_struct_73_1050_14cc,
        (ctx._g_struct_73_1050_14cc >> 0x10),
        0x3ff,
        CONCAT22(in_dx, in_ax),
        0x7e6,
    );
    load_string_1010_84e0(
        ctx._g_struct_73_1050_14cc,
        (ctx._g_struct_73_1050_14cc >> 0x10),
        0x3ff,
        CONCAT22(unaff_ss, local_104),
        0x7e7,
    );
    process_string_1000_3cea(_local_108, CONCAT22(unaff_ss, local_104));
    load_string_1010_84e0(
        ctx._g_struct_73_1050_14cc,
        (ctx._g_struct_73_1050_14cc >> 0x10),
        0x3ff,
        CONCAT22(unaff_ss, local_104),
        0x7e8,
    );
    process_string_1000_3cea(_local_108, CONCAT22(unaff_ss, local_104));
    load_string_1010_84e0(
        ctx._g_struct_73_1050_14cc,
        (ctx._g_struct_73_1050_14cc >> 0x10),
        0x101,
        CONCAT22(unaff_ss, local_20a),
        0x57b,
    );
    MessageBox16(0, CONCAT22(unaff_ss, local_20a), _local_108, (param_1 + 6));
    error_check_1000_17ce(_local_108);
    return;
}
